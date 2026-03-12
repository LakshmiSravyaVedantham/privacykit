pub mod actions;
pub mod dashboard;
pub mod monitor;
pub mod watchlist;

use anyhow::Result;
use colored::Colorize;

use crate::common::{self, print_header, severity_for_data_types};
use actions::{mark_done, mark_skipped, print_pending, print_status};
use dashboard::print_dashboard;
use monitor::scan_all;
use watchlist::{add_watch, init_db, remove_watch};

fn open_leak_db() -> Result<rusqlite::Connection> {
    let data_dir = common::data_dir()?;
    let db_path = data_dir.join("leakwatch.db");
    let conn = common::open_db(&db_path)?;
    init_db(&conn)?;
    Ok(conn)
}

/// Add an identifier to the watchlist
pub fn cmd_watch(identifier: &str) -> Result<()> {
    let conn = open_leak_db()?;
    if add_watch(&conn, identifier)? {
        println!(
            "  {} Added {} to watchlist.",
            "✓".green().bold(),
            identifier.cyan()
        );
        println!("  Run `privacykit leak scan` to check for breaches.");
    } else {
        println!(
            "  {} {} is already on the watchlist.",
            "→".yellow(),
            identifier.cyan()
        );
    }
    Ok(())
}

/// Remove an identifier from the watchlist
pub fn cmd_unwatch(identifier: &str) -> Result<()> {
    let conn = open_leak_db()?;
    if remove_watch(&conn, identifier)? {
        println!(
            "  {} Removed {} from watchlist.",
            "✓".green().bold(),
            identifier.cyan()
        );
    } else {
        println!(
            "  {} {} was not on the watchlist.",
            "✗".red(),
            identifier.cyan()
        );
    }
    Ok(())
}

/// Scan all watched identifiers for new breaches
pub fn cmd_scan() -> Result<()> {
    let conn = open_leak_db()?;
    let results = scan_all(&conn)?;

    if results.is_empty() {
        println!(
            "\n  {} Watchlist is empty. Add identifiers with `privacykit leak watch <email>`.",
            "→".yellow()
        );
        return Ok(());
    }

    print_header("BREACH SCAN RESULTS");

    let mut total_new = 0;
    for result in &results {
        if result.new_breaches.is_empty() {
            println!(
                "\n  {} {} — no new breaches (previously seen: {})",
                "✓".green(),
                result.identifier.white().bold(),
                result.known_breaches
            );
        } else {
            total_new += result.new_breaches.len();
            println!(
                "\n  {} {} — {} NEW breach{} found!",
                "⚠".red().bold(),
                result.identifier.white().bold(),
                result.new_breaches.len().to_string().red().bold(),
                if result.new_breaches.len() == 1 { "" } else { "es" }
            );
            for breach in &result.new_breaches {
                let data_types: Vec<String> =
                    breach.data_types.iter().map(|s| s.to_string()).collect();
                let severity = severity_for_data_types(&data_types);
                println!(
                    "    {} {} ({}) — {}",
                    "●".red(),
                    breach.name.white().bold(),
                    breach.date,
                    severity.colored_str()
                );
            }
            println!(
                "    Generated {} action items.",
                result.actions_generated.to_string().cyan()
            );
        }
    }

    if total_new == 0 {
        println!(
            "\n  {} No new breaches detected across all monitored identifiers.",
            "✓".green().bold()
        );
    } else {
        println!(
            "\n  {} {} new breach{} found. Use `privacykit leak actions` to view required actions.",
            "⚠".yellow().bold(),
            total_new.to_string().red().bold(),
            if total_new == 1 { "" } else { "es" }
        );
    }

    Ok(())
}

/// Show the monitoring dashboard
pub fn cmd_dashboard() -> Result<()> {
    let conn = open_leak_db()?;
    print_dashboard(&conn)
}

/// Show pending actions
pub fn cmd_actions() -> Result<()> {
    let conn = open_leak_db()?;
    print_pending(&conn)
}

/// Mark a leak action as done
pub fn cmd_done(id: i64) -> Result<()> {
    let conn = open_leak_db()?;
    if mark_done(&conn, id)? {
        println!("  {} Action {} marked as done.", "✓".green().bold(), id);
    } else {
        println!(
            "  {} Action {} not found or already completed.",
            "✗".red(),
            id
        );
    }
    Ok(())
}

/// Skip a leak action
pub fn cmd_skip(id: i64) -> Result<()> {
    let conn = open_leak_db()?;
    if mark_skipped(&conn, id)? {
        println!("  {} Action {} skipped.", "→".yellow(), id);
    } else {
        println!(
            "  {} Action {} not found or already completed.",
            "✗".red(),
            id
        );
    }
    Ok(())
}

/// Show quick status summary
pub fn cmd_status() -> Result<()> {
    let conn = open_leak_db()?;
    print_status(&conn)
}
