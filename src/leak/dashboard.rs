use anyhow::Result;
use colored::Colorize;
use rusqlite::Connection;

use crate::common::print_header;
use super::actions::get_stats;
use super::watchlist::{breach_count, get_watchlist};

/// Print the leakwatch monitoring dashboard
pub fn print_dashboard(conn: &Connection) -> Result<()> {
    print_header("LEAKWATCH DASHBOARD");

    let watchlist = get_watchlist(conn)?;
    let (total_actions, pending, done, skipped) = get_stats(conn)?;

    // Watchlist summary
    println!("\n  {}", "MONITORED IDENTIFIERS".white().bold());
    if watchlist.is_empty() {
        println!(
            "  {} No identifiers being monitored.",
            "→".dimmed()
        );
        println!("  Add one with: privacykit leak watch <email>");
    } else {
        for (_id, identifier, added) in &watchlist {
            let breaches = breach_count(conn, identifier)?;
            let breach_str = if breaches == 0 {
                "no breaches detected".green().to_string()
            } else {
                format!("{} breaches found", breaches).red().to_string()
            };
            println!(
                "  {} {} — {} (watching since {})",
                "●".cyan(),
                identifier.white().bold(),
                breach_str,
                added.dimmed()
            );
        }
    }

    // New alerts
    let new_alerts = count_new_alerts(conn)?;
    println!("\n  {}", "ALERTS".white().bold());
    if new_alerts > 0 {
        println!(
            "  {} {} new alert{} requiring attention",
            "⚠".yellow().bold(),
            new_alerts.to_string().red().bold(),
            if new_alerts == 1 { "" } else { "s" }
        );
    } else {
        println!(
            "  {} All clear — no new alerts",
            "✓".green().bold()
        );
    }

    // Action stats
    println!("\n  {}", "ACTION PROGRESS".white().bold());
    if total_actions == 0 {
        println!("  {} No actions yet.", "→".dimmed());
    } else {
        let completion_pct = if total_actions > 0 {
            ((done + skipped) as f64 / total_actions as f64 * 100.0) as usize
        } else {
            0
        };

        // Visual progress bar
        let bar_width = 30;
        let filled = (completion_pct as f64 / 100.0 * bar_width as f64).round() as usize;
        let bar = format!(
            "[{}{}] {}%",
            "█".repeat(filled).green(),
            "░".repeat(bar_width - filled),
            completion_pct
        );
        println!("  {}", bar);

        println!(
            "  {} pending  |  {} done  |  {} skipped  |  {} total",
            pending.to_string().yellow(),
            done.to_string().green(),
            skipped.to_string().dimmed(),
            total_actions.to_string().cyan()
        );
    }

    // Quick tips
    println!("\n  {}", "QUICK COMMANDS".white().bold());
    println!("  {} privacykit leak scan     — scan for new breaches", "→".dimmed());
    println!("  {} privacykit leak actions   — view pending actions", "→".dimmed());
    println!(
        "  {} privacykit leak done <id> — mark action complete",
        "→".dimmed()
    );

    println!();
    Ok(())
}

/// Count pending actions as "new alerts"
fn count_new_alerts(conn: &Connection) -> Result<usize> {
    let count: usize = conn.query_row(
        "SELECT COUNT(*) FROM leak_actions WHERE status = 'pending'",
        [],
        |row| row.get(0),
    )?;
    Ok(count)
}
