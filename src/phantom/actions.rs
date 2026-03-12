use anyhow::{Context, Result};
use colored::Colorize;
use rusqlite::Connection;

use crate::common::{self, print_header, Severity};

/// Initialize the phantom actions SQLite table
pub fn init_db(conn: &Connection) -> Result<()> {
    conn.execute_batch(
        "CREATE TABLE IF NOT EXISTS phantom_actions (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            breach_name TEXT NOT NULL,
            action TEXT NOT NULL,
            severity TEXT NOT NULL,
            status TEXT NOT NULL DEFAULT 'pending',
            created_at TEXT NOT NULL DEFAULT (datetime('now')),
            completed_at TEXT
        );",
    )
    .context("Failed to create phantom_actions table")?;
    Ok(())
}

/// Insert an action, returning its ID. Deduplicates by breach_name + action.
pub fn insert_action(
    conn: &Connection,
    breach_name: &str,
    action: &str,
    severity: &Severity,
) -> Result<i64> {
    // Check if this exact action already exists
    let existing: Option<i64> = conn
        .query_row(
            "SELECT id FROM phantom_actions WHERE breach_name = ?1 AND action = ?2",
            rusqlite::params![breach_name, action],
            |row| row.get(0),
        )
        .ok();

    if let Some(id) = existing {
        return Ok(id);
    }

    conn.execute(
        "INSERT INTO phantom_actions (breach_name, action, severity) VALUES (?1, ?2, ?3)",
        rusqlite::params![breach_name, action, severity.as_str()],
    )
    .context("Failed to insert action")?;

    Ok(conn.last_insert_rowid())
}

/// Mark an action as done
pub fn mark_done(conn: &Connection, id: i64) -> Result<bool> {
    let rows = conn.execute(
        "UPDATE phantom_actions SET status = 'done', completed_at = datetime('now') WHERE id = ?1 AND status = 'pending'",
        rusqlite::params![id],
    )?;
    Ok(rows > 0)
}

/// Skip an action
pub fn mark_skipped(conn: &Connection, id: i64) -> Result<bool> {
    let rows = conn.execute(
        "UPDATE phantom_actions SET status = 'skipped', completed_at = datetime('now') WHERE id = ?1 AND status = 'pending'",
        rusqlite::params![id],
    )?;
    Ok(rows > 0)
}

/// Get all pending actions
pub fn get_pending_actions(conn: &Connection) -> Result<Vec<(i64, String, String, String)>> {
    let mut stmt = conn.prepare(
        "SELECT id, breach_name, action, severity FROM phantom_actions WHERE status = 'pending' ORDER BY
            CASE severity
                WHEN 'CRITICAL' THEN 1
                WHEN 'HIGH' THEN 2
                WHEN 'MEDIUM' THEN 3
                ELSE 4
            END, id",
    )?;

    let rows = stmt
        .query_map([], |row| {
            Ok((
                row.get::<_, i64>(0)?,
                row.get::<_, String>(1)?,
                row.get::<_, String>(2)?,
                row.get::<_, String>(3)?,
            ))
        })?
        .collect::<Result<Vec<_>, _>>()
        .context("Failed to fetch pending actions")?;

    Ok(rows)
}

/// Get action stats (total, pending, done, skipped)
pub fn get_stats(conn: &Connection) -> Result<(usize, usize, usize, usize)> {
    let total: usize = conn.query_row(
        "SELECT COUNT(*) FROM phantom_actions",
        [],
        |row| row.get(0),
    )?;
    let pending: usize = conn.query_row(
        "SELECT COUNT(*) FROM phantom_actions WHERE status = 'pending'",
        [],
        |row| row.get(0),
    )?;
    let done: usize = conn.query_row(
        "SELECT COUNT(*) FROM phantom_actions WHERE status = 'done'",
        [],
        |row| row.get(0),
    )?;
    let skipped: usize = conn.query_row(
        "SELECT COUNT(*) FROM phantom_actions WHERE status = 'skipped'",
        [],
        |row| row.get(0),
    )?;
    Ok((total, pending, done, skipped))
}

/// Print pending actions
pub fn print_pending(conn: &Connection) -> Result<()> {
    let actions = get_pending_actions(conn)?;

    if actions.is_empty() {
        println!("\n  {} No pending actions.", "✓".green().bold());
        return Ok(());
    }

    print_header("PENDING ACTIONS");
    for (id, breach, action, severity) in &actions {
        let sev = Severity::from_str_loose(severity);
        println!(
            "\n  [{}] {} (from {})",
            id.to_string().cyan(),
            sev.colored_str(),
            breach.dimmed()
        );
        println!("      {}", action.white());
    }
    println!(
        "\n  Use `privacykit phantom done <id>` or `privacykit phantom skip <id>` to manage."
    );
    Ok(())
}

/// Print stats summary
pub fn print_status(conn: &Connection) -> Result<()> {
    let (total, pending, done, skipped) = get_stats(conn)?;

    print_header("PHANTOM STATUS");
    println!("\n  Total actions:   {}", total.to_string().cyan());
    println!("  Pending:         {}", pending.to_string().yellow());
    println!("  Completed:       {}", done.to_string().green());
    println!("  Skipped:         {}", skipped.to_string().dimmed());

    if total > 0 {
        let completion = ((done + skipped) as f64 / total as f64 * 100.0) as usize;
        println!("  Progress:        {}%", completion.to_string().cyan());
        common::print_bar("Completed", done + skipped, total, 30);
    }
    Ok(())
}
