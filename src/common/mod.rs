use anyhow::{Context, Result};
use colored::Colorize;
use reqwest::Client;
use rusqlite::Connection;
use std::path::PathBuf;
use std::time::Duration;

/// Get the privacykit data directory (~/.privacykit/)
pub fn data_dir() -> Result<PathBuf> {
    let home = dirs::home_dir().context("Could not determine home directory")?;
    let dir = home.join(".privacykit");
    std::fs::create_dir_all(&dir)
        .with_context(|| format!("Failed to create data directory: {}", dir.display()))?;
    Ok(dir)
}

/// Open a SQLite connection at the given path, with WAL mode enabled
pub fn open_db(path: &PathBuf) -> Result<Connection> {
    let conn = Connection::open(path)
        .with_context(|| format!("Failed to open database: {}", path.display()))?;
    conn.execute_batch("PRAGMA journal_mode=WAL; PRAGMA foreign_keys=ON;")?;
    Ok(conn)
}

/// Build a shared reqwest HTTP client with sensible defaults
pub fn http_client() -> Result<Client> {
    Client::builder()
        .timeout(Duration::from_secs(15))
        .user_agent("privacykit/0.1.0")
        .build()
        .context("Failed to build HTTP client")
}

/// Severity levels for actions
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Severity {
    Low,
    Medium,
    High,
    Critical,
}

impl std::fmt::Display for Severity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Severity::Low => write!(f, "LOW"),
            Severity::Medium => write!(f, "MEDIUM"),
            Severity::High => write!(f, "HIGH"),
            Severity::Critical => write!(f, "CRITICAL"),
        }
    }
}

impl Severity {
    pub fn colored_str(&self) -> String {
        match self {
            Severity::Low => "LOW".blue().to_string(),
            Severity::Medium => "MEDIUM".yellow().to_string(),
            Severity::High => "HIGH".red().to_string(),
            Severity::Critical => "CRITICAL".red().bold().to_string(),
        }
    }

    pub fn from_str_loose(s: &str) -> Self {
        match s.to_uppercase().as_str() {
            "CRITICAL" => Severity::Critical,
            "HIGH" => Severity::High,
            "MEDIUM" => Severity::Medium,
            _ => Severity::Low,
        }
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            Severity::Low => "LOW",
            Severity::Medium => "MEDIUM",
            Severity::High => "HIGH",
            Severity::Critical => "CRITICAL",
        }
    }
}

/// Print a colored bar chart line
pub fn print_bar(label: &str, value: usize, max: usize, width: usize) {
    let ratio = if max == 0 {
        0.0
    } else {
        value as f64 / max as f64
    };
    let filled = (ratio * width as f64).round() as usize;
    let bar: String = "█".repeat(filled);
    let empty: String = "░".repeat(width.saturating_sub(filled));
    println!(
        "  {:<30} {} {} ({})",
        label.white().bold(),
        bar.green(),
        empty,
        value.to_string().cyan()
    );
}

/// Print a section header
pub fn print_header(title: &str) {
    let line = "─".repeat(60);
    println!("\n{}", line.dimmed());
    println!("  {}", title.white().bold());
    println!("{}", line.dimmed());
}

/// Determine severity from breach data types
pub fn severity_for_data_types(data_types: &[String]) -> Severity {
    let lower: Vec<String> = data_types.iter().map(|s| s.to_lowercase()).collect();
    if lower.iter().any(|d| {
        d.contains("password")
            || d.contains("credit card")
            || d.contains("ssn")
            || d.contains("social security")
            || d.contains("financial")
    }) {
        Severity::Critical
    } else if lower.iter().any(|d| {
        d.contains("phone")
            || (d.contains("address") && !d.contains("email") && !d.contains("ip address"))
            || (d.contains("name") && !d.contains("username"))
            || d.contains("dob")
            || d.contains("date of birth")
            || d.contains("passport")
            || d.contains("genetic")
    }) {
        Severity::High
    } else if lower
        .iter()
        .any(|d| d.contains("email") || d.contains("username"))
    {
        Severity::Medium
    } else {
        Severity::Low
    }
}

/// Generate action items based on breach data types
pub fn actions_for_data_types(breach_name: &str, data_types: &[String]) -> Vec<(String, Severity)> {
    let mut actions = Vec::new();
    let lower: Vec<String> = data_types.iter().map(|s| s.to_lowercase()).collect();

    if lower.iter().any(|d| d.contains("password")) {
        actions.push((
            format!(
                "Change password for {} and any sites sharing that password",
                breach_name
            ),
            Severity::Critical,
        ));
        actions.push((
            "Enable two-factor authentication where possible".to_string(),
            Severity::Critical,
        ));
    }
    if lower.iter().any(|d| d.contains("credit card") || d.contains("financial")) {
        actions.push((
            format!(
                "Check bank/credit card statements for unauthorized charges ({})",
                breach_name
            ),
            Severity::Critical,
        ));
        actions.push((
            "Consider placing a fraud alert on your credit report".to_string(),
            Severity::High,
        ));
    }
    if lower.iter().any(|d| d.contains("ssn") || d.contains("social security")) {
        actions.push((
            "Freeze your credit at all three bureaus (Equifax, Experian, TransUnion)".to_string(),
            Severity::Critical,
        ));
    }
    if lower.iter().any(|d| d.contains("phone")) {
        actions.push((
            format!(
                "Watch for phishing calls/texts referencing {} data",
                breach_name
            ),
            Severity::High,
        ));
    }
    if lower.iter().any(|d| d.contains("address") || d.contains("name")) {
        actions.push((
            format!(
                "Monitor for identity theft attempts related to {}",
                breach_name
            ),
            Severity::High,
        ));
    }
    if lower.iter().any(|d| d.contains("email")) {
        actions.push((
            format!(
                "Watch for targeted phishing emails referencing {}",
                breach_name
            ),
            Severity::Medium,
        ));
    }
    if actions.is_empty() {
        actions.push((
            format!("Review your account settings at {}", breach_name),
            Severity::Low,
        ));
    }
    actions
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_severity_display() {
        assert_eq!(Severity::Critical.to_string(), "CRITICAL");
        assert_eq!(Severity::High.to_string(), "HIGH");
        assert_eq!(Severity::Medium.to_string(), "MEDIUM");
        assert_eq!(Severity::Low.to_string(), "LOW");
    }

    #[test]
    fn test_severity_from_str_loose() {
        assert_eq!(Severity::from_str_loose("CRITICAL"), Severity::Critical);
        assert_eq!(Severity::from_str_loose("high"), Severity::High);
        assert_eq!(Severity::from_str_loose("medium"), Severity::Medium);
        assert_eq!(Severity::from_str_loose("whatever"), Severity::Low);
    }

    #[test]
    fn test_severity_ordering() {
        assert!(Severity::Critical > Severity::High);
        assert!(Severity::High > Severity::Medium);
        assert!(Severity::Medium > Severity::Low);
    }

    #[test]
    fn test_severity_for_data_types_critical() {
        let types = vec!["Passwords".to_string(), "Email addresses".to_string()];
        assert_eq!(severity_for_data_types(&types), Severity::Critical);
    }

    #[test]
    fn test_severity_for_data_types_high() {
        let types = vec!["Phone numbers".to_string(), "Names".to_string()];
        assert_eq!(severity_for_data_types(&types), Severity::High);
    }

    #[test]
    fn test_severity_for_data_types_medium() {
        let types = vec!["Email addresses".to_string()];
        assert_eq!(severity_for_data_types(&types), Severity::Medium);
    }

    #[test]
    fn test_severity_for_data_types_low() {
        let types = vec!["IP addresses".to_string()];
        assert_eq!(severity_for_data_types(&types), Severity::Low);
    }

    #[test]
    fn test_actions_for_password_breach() {
        let actions = actions_for_data_types("TestBreach", &["Passwords".to_string()]);
        assert!(actions.len() >= 2);
        assert!(actions.iter().any(|(_, s)| *s == Severity::Critical));
    }

    #[test]
    fn test_actions_for_email_only_breach() {
        let actions = actions_for_data_types("TestBreach", &["Email addresses".to_string()]);
        assert!(!actions.is_empty());
        assert!(actions.iter().any(|(_, s)| *s == Severity::Medium));
    }

    #[test]
    fn test_actions_for_unknown_data() {
        let actions = actions_for_data_types("TestBreach", &["Avatars".to_string()]);
        assert_eq!(actions.len(), 1);
        assert_eq!(actions[0].1, Severity::Low);
    }
}
