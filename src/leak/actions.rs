use anyhow::{Context, Result};
use colored::Colorize;
use rusqlite::Connection;

use crate::common::{self, print_header, Severity};

/// Insert an action for leakwatch. Deduplicates by breach_name + identifier + action.
pub fn insert_action(
    conn: &Connection,
    breach_name: &str,
    identifier: &str,
    action: &str,
    severity: &Severity,
) -> Result<i64> {
    let existing: Option<i64> = conn
        .query_row(
            "SELECT id FROM leak_actions WHERE breach_name = ?1 AND identifier = ?2 AND action = ?3",
            rusqlite::params![breach_name, identifier, action],
            |row| row.get(0),
        )
        .ok();

    if let Some(id) = existing {
        return Ok(id);
    }

    conn.execute(
        "INSERT INTO leak_actions (breach_name, identifier, action, severity) VALUES (?1, ?2, ?3, ?4)",
        rusqlite::params![breach_name, identifier, action, severity.as_str()],
    )
    .context("Failed to insert leak action")?;

    Ok(conn.last_insert_rowid())
}

/// Mark a leak action as done
pub fn mark_done(conn: &Connection, id: i64) -> Result<bool> {
    let rows = conn.execute(
        "UPDATE leak_actions SET status = 'done', completed_at = datetime('now') WHERE id = ?1 AND status = 'pending'",
        rusqlite::params![id],
    )?;
    Ok(rows > 0)
}

/// Skip a leak action
pub fn mark_skipped(conn: &Connection, id: i64) -> Result<bool> {
    let rows = conn.execute(
        "UPDATE leak_actions SET status = 'skipped', completed_at = datetime('now') WHERE id = ?1 AND status = 'pending'",
        rusqlite::params![id],
    )?;
    Ok(rows > 0)
}

/// Get all pending leak actions
pub fn get_pending_actions(conn: &Connection) -> Result<Vec<(i64, String, String, String, String)>> {
    let mut stmt = conn.prepare(
        "SELECT id, breach_name, identifier, action, severity FROM leak_actions WHERE status = 'pending' ORDER BY
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
                row.get::<_, String>(4)?,
            ))
        })?
        .collect::<Result<Vec<_>, _>>()
        .context("Failed to fetch pending leak actions")?;

    Ok(rows)
}

/// Get leak action stats
pub fn get_stats(conn: &Connection) -> Result<(usize, usize, usize, usize)> {
    let total: usize = conn.query_row(
        "SELECT COUNT(*) FROM leak_actions",
        [],
        |row| row.get(0),
    )?;
    let pending: usize = conn.query_row(
        "SELECT COUNT(*) FROM leak_actions WHERE status = 'pending'",
        [],
        |row| row.get(0),
    )?;
    let done: usize = conn.query_row(
        "SELECT COUNT(*) FROM leak_actions WHERE status = 'done'",
        [],
        |row| row.get(0),
    )?;
    let skipped: usize = conn.query_row(
        "SELECT COUNT(*) FROM leak_actions WHERE status = 'skipped'",
        [],
        |row| row.get(0),
    )?;
    Ok((total, pending, done, skipped))
}

/// Print pending leak actions
pub fn print_pending(conn: &Connection) -> Result<()> {
    let actions = get_pending_actions(conn)?;

    if actions.is_empty() {
        println!("\n  {} No pending leak actions.", "✓".green().bold());
        return Ok(());
    }

    print_header("PENDING LEAK ACTIONS");
    for (id, breach, identifier, action, severity) in &actions {
        let sev = Severity::from_str_loose(severity);
        println!(
            "\n  [{}] {} — {} ({})",
            id.to_string().cyan(),
            sev.colored_str(),
            breach.white().bold(),
            identifier.dimmed()
        );
        println!("      {}", action.white());
    }
    println!(
        "\n  Use `privacykit leak done <id>` or `privacykit leak skip <id>` to manage."
    );
    Ok(())
}

/// Print leak status summary
pub fn print_status(conn: &Connection) -> Result<()> {
    let (total, pending, done, skipped) = get_stats(conn)?;

    print_header("LEAKWATCH STATUS");
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
