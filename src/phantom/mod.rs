pub mod actions;
pub mod breach_db;
pub mod checker;
pub mod report;

use anyhow::Result;
use colored::Colorize;
use std::io::{self, Write};

use crate::common::{self, actions_for_data_types};
use actions::{init_db, insert_action, mark_done, mark_skipped, print_pending, print_status};
use breach_db::find_breaches_for_email;
use checker::check_password_hibp;
use report::{print_breach_report, print_password_result};

fn open_phantom_db() -> Result<rusqlite::Connection> {
    let data_dir = common::data_dir()?;
    let db_path = data_dir.join("phantom.db");
    let conn = common::open_db(&db_path)?;
    init_db(&conn)?;
    Ok(conn)
}

/// Scan an email for known breaches and generate actions
pub fn cmd_scan(email: &str) -> Result<()> {
    let breaches = find_breaches_for_email(email);
    print_breach_report(email, &breaches);

    if breaches.is_empty() {
        return Ok(());
    }

    // Generate actions and store in SQLite
    let conn = open_phantom_db()?;
    let mut action_count = 0;

    for breach in &breaches {
        let data_types: Vec<String> = breach.data_types.iter().map(|s| s.to_string()).collect();
        let actions = actions_for_data_types(breach.name, &data_types);
        for (action_text, severity) in &actions {
            insert_action(&conn, breach.name, action_text, severity)?;
            action_count += 1;
        }
    }

    println!(
        "\n  Generated {} action items. Use `privacykit phantom actions` to view them.",
        action_count.to_string().cyan()
    );
    Ok(())
}

/// Show pending actions
pub fn cmd_actions() -> Result<()> {
    let conn = open_phantom_db()?;
    print_pending(&conn)
}

/// Mark action as done
pub fn cmd_done(id: i64) -> Result<()> {
    let conn = open_phantom_db()?;
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

/// Skip an action
pub fn cmd_skip(id: i64) -> Result<()> {
    let conn = open_phantom_db()?;
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

/// Interactive password check via HIBP
pub async fn cmd_check_password() -> Result<()> {
    print!("  Enter password to check (input hidden): ");
    io::stdout().flush()?;

    // Read password (not truly hidden in all terminals, but functional)
    let mut password = String::new();
    io::stdin().read_line(&mut password)?;
    let password = password.trim();

    if password.is_empty() {
        anyhow::bail!("No password provided");
    }

    println!("  Checking against HIBP database...");
    let count = check_password_hibp(password).await?;
    print_password_result(count);
    Ok(())
}

/// Show phantom stats
pub fn cmd_status() -> Result<()> {
    let conn = open_phantom_db()?;
    print_status(&conn)
}
