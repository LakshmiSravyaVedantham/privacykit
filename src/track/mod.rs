pub mod report;
pub mod scanner;
pub mod tracker_db;

use anyhow::{Context, Result};
use std::collections::HashSet;

use crate::common;
use report::{print_db_stats, print_domain_check, print_scan_report, ScanResults};
use scanner::scan_page;
use tracker_db::{build_tracker_db, lookup_domain};

/// Scan URLs from a file for trackers
pub async fn cmd_scan(file_path: &str) -> Result<()> {
    let content = std::fs::read_to_string(file_path)
        .with_context(|| format!("Failed to read file: {}", file_path))?;

    let urls: Vec<&str> = content
        .lines()
        .map(|l| l.trim())
        .filter(|l| !l.is_empty() && (l.starts_with("http://") || l.starts_with("https://")))
        .collect();

    if urls.is_empty() {
        anyhow::bail!("No valid URLs found in {}. Each line should be a full URL (http:// or https://)", file_path);
    }

    println!("Scanning {} URLs for trackers...\n", urls.len());

    let client = common::http_client()?;
    let db = build_tracker_db();
    let mut results = ScanResults::new();

    for url in &urls {
        print!("  Scanning {}... ", url);
        match scan_page(&client, url).await {
            Ok(domains) => {
                results.pages_scanned += 1;
                results.total_domains += domains.len();

                let mut page_trackers = 0;
                for domain in &domains {
                    if let Some(info) = lookup_domain(&db, domain) {
                        results.add_tracker(domain.clone(), info.clone());
                        page_trackers += 1;
                    }
                }
                println!(
                    "found {} third-party domains, {} trackers",
                    domains.len(),
                    page_trackers
                );
            }
            Err(e) => {
                println!("ERROR: {}", e);
            }
        }
    }

    print_scan_report(&results);
    Ok(())
}

/// Check if a single domain is a known tracker
pub fn cmd_check(domain: &str) -> Result<()> {
    let db = build_tracker_db();
    let info = lookup_domain(&db, domain);
    print_domain_check(domain, info);
    Ok(())
}

/// Show tracker database statistics
pub fn cmd_stats() -> Result<()> {
    let db = build_tracker_db();
    print_db_stats(&db);
    Ok(())
}

/// Scan a single URL and return the set of detected tracker domains (for testing/library use)
#[allow(dead_code)]
pub async fn scan_url_for_trackers(url: &str) -> Result<HashSet<String>> {
    let client = common::http_client()?;
    let db = build_tracker_db();
    let domains = scan_page(&client, url).await?;
    let trackers: HashSet<String> = domains
        .into_iter()
        .filter(|d| lookup_domain(&db, d).is_some())
        .collect();
    Ok(trackers)
}
