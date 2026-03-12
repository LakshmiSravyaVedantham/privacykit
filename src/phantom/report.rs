use crate::common::{print_header, severity_for_data_types};
use crate::phantom::breach_db::BreachInfo;
use colored::Colorize;

/// Print a breach scan report for an email
pub fn print_breach_report(email: &str, breaches: &[BreachInfo]) {
    print_header(&format!("BREACH SCAN: {}", email));

    if breaches.is_empty() {
        println!(
            "\n  {} No known breaches found for this email.",
            "✓".green().bold()
        );
        return;
    }

    println!(
        "\n  {} Found in {} known breach{}",
        "⚠".yellow().bold(),
        breaches.len().to_string().red().bold(),
        if breaches.len() == 1 { "" } else { "es" }
    );

    for breach in breaches {
        let data_types: Vec<String> = breach.data_types.iter().map(|s| s.to_string()).collect();
        let severity = severity_for_data_types(&data_types);

        println!("\n  {} {}", "●".red(), breach.name.white().bold());
        println!("    Date:     {}", breach.date);
        println!(
            "    Affected: {} accounts",
            format_number(breach.accounts_affected)
        );
        println!("    Severity: {}", severity.colored_str());
        println!("    Data exposed:");
        for dt in &breach.data_types {
            println!("      {} {}", "→".dimmed(), dt);
        }
    }
}

/// Print password check result
pub fn print_password_result(count: u64) {
    print_header("PASSWORD CHECK (HIBP k-anonymity)");

    if count == 0 {
        println!(
            "\n  {} This password was NOT found in any known breach.",
            "✓".green().bold()
        );
        println!("  This doesn't guarantee it's safe — use a unique password per site.");
    } else {
        println!(
            "\n  {} This password has been seen {} time{} in data breaches!",
            "⚠".red().bold(),
            count.to_string().red().bold(),
            if count == 1 { "" } else { "s" }
        );
        println!("  {} Change this password immediately on all sites.", "→".yellow());
        println!("  {} Use a unique, randomly generated password per site.", "→".yellow());
        println!("  {} Consider using a password manager.", "→".yellow());
    }

    println!("\n  {}", "Your password was NOT sent over the network.".dimmed());
    println!(
        "  {}",
        "Only the first 5 characters of the SHA1 hash were transmitted (k-anonymity).".dimmed()
    );
}

fn format_number(n: u64) -> String {
    if n >= 1_000_000_000 {
        format!("{:.1}B", n as f64 / 1_000_000_000.0)
    } else if n >= 1_000_000 {
        format!("{:.0}M", n as f64 / 1_000_000.0)
    } else if n >= 1_000 {
        format!("{:.0}K", n as f64 / 1_000.0)
    } else {
        n.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_number_billions() {
        assert_eq!(format_number(3_000_000_000), "3.0B");
    }

    #[test]
    fn test_format_number_millions() {
        assert_eq!(format_number(153_000_000), "153M");
    }

    #[test]
    fn test_format_number_thousands() {
        assert_eq!(format_number(5_000), "5K");
    }

    #[test]
    fn test_format_number_small() {
        assert_eq!(format_number(42), "42");
    }
}
