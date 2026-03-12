mod common;
mod leak;
mod phantom;
mod track;

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(
    name = "privacykit",
    version,
    about = "Privacy toolkit — track your trackers, find ghost accounts, monitor for breaches",
    long_about = "privacykit combines three privacy tools into one binary:\n\n\
        track   — Scan websites for tracking scripts and third-party domains\n\
        phantom — Find which data breaches expose your accounts\n\
        leak    — Monitor identifiers for new breach appearances"
)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Tracker network mapper — scan websites for tracking scripts
    Track {
        #[command(subcommand)]
        action: TrackAction,
    },
    /// Ghost account finder — scan for breach exposure and manage actions
    Phantom {
        #[command(subcommand)]
        action: PhantomAction,
    },
    /// Breach monitoring — watch identifiers and get alerts on new breaches
    Leak {
        #[command(subcommand)]
        action: LeakAction,
    },
}

#[derive(Subcommand)]
enum TrackAction {
    /// Scan URLs from a text file for trackers
    Scan {
        /// Path to file with URLs (one per line)
        file: String,
    },
    /// Check if a domain is a known tracker
    Check {
        /// Domain to check (e.g., doubleclick.net)
        domain: String,
    },
    /// Show tracker database statistics
    Stats,
}

#[derive(Subcommand)]
enum PhantomAction {
    /// Scan an email for known breaches
    Scan {
        /// Email address to scan
        email: String,
    },
    /// Show pending actions from breach scans
    Actions,
    /// Mark an action as done
    Done {
        /// Action ID
        id: i64,
    },
    /// Skip an action
    Skip {
        /// Action ID
        id: i64,
    },
    /// Check a password against HIBP (k-anonymity)
    CheckPassword,
    /// Show phantom status and stats
    Status,
}

#[derive(Subcommand)]
enum LeakAction {
    /// Add an identifier to the watchlist
    Watch {
        /// Email, username, or other identifier
        identifier: String,
    },
    /// Remove an identifier from the watchlist
    Unwatch {
        /// Identifier to remove
        identifier: String,
    },
    /// Scan watchlist for new breaches
    Scan,
    /// Show monitoring dashboard
    Dashboard,
    /// Show pending leak actions
    Actions,
    /// Mark a leak action as done
    Done {
        /// Action ID
        id: i64,
    },
    /// Skip a leak action
    Skip {
        /// Action ID
        id: i64,
    },
    /// Show quick status summary
    Status,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Track { action } => match action {
            TrackAction::Scan { file } => track::cmd_scan(&file).await?,
            TrackAction::Check { domain } => track::cmd_check(&domain)?,
            TrackAction::Stats => track::cmd_stats()?,
        },
        Commands::Phantom { action } => match action {
            PhantomAction::Scan { email } => phantom::cmd_scan(&email)?,
            PhantomAction::Actions => phantom::cmd_actions()?,
            PhantomAction::Done { id } => phantom::cmd_done(id)?,
            PhantomAction::Skip { id } => phantom::cmd_skip(id)?,
            PhantomAction::CheckPassword => phantom::cmd_check_password().await?,
            PhantomAction::Status => phantom::cmd_status()?,
        },
        Commands::Leak { action } => match action {
            LeakAction::Watch { identifier } => leak::cmd_watch(&identifier)?,
            LeakAction::Unwatch { identifier } => leak::cmd_unwatch(&identifier)?,
            LeakAction::Scan => leak::cmd_scan()?,
            LeakAction::Dashboard => leak::cmd_dashboard()?,
            LeakAction::Actions => leak::cmd_actions()?,
            LeakAction::Done { id } => leak::cmd_done(id)?,
            LeakAction::Skip { id } => leak::cmd_skip(id)?,
            LeakAction::Status => leak::cmd_status()?,
        },
    }

    Ok(())
}
