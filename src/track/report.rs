use crate::common::{print_bar, print_header};
use crate::track::tracker_db::{TrackerCategory, TrackerInfo};
use colored::Colorize;
use std::collections::HashMap;

/// Results of scanning one or more pages
pub struct ScanResults {
    pub pages_scanned: usize,
    pub total_domains: usize,
    pub tracker_domains: usize,
    /// company -> list of (domain, info)
    pub by_company: HashMap<String, Vec<(String, TrackerInfo)>>,
    /// category -> count
    pub by_category: HashMap<TrackerCategory, usize>,
}

impl ScanResults {
    pub fn new() -> Self {
        ScanResults {
            pages_scanned: 0,
            total_domains: 0,
            tracker_domains: 0,
            by_company: HashMap::new(),
            by_category: HashMap::new(),
        }
    }

    pub fn add_tracker(&mut self, domain: String, info: TrackerInfo) {
        let company = info.company.to_string();
        let category = info.category.clone();
        self.by_company
            .entry(company)
            .or_default()
            .push((domain, info));
        *self.by_category.entry(category).or_insert(0) += 1;
        self.tracker_domains += 1;
    }
}

/// Print a full scan report to the terminal
pub fn print_scan_report(results: &ScanResults) {
    print_header("TRACKER SCAN REPORT");

    println!(
        "\n  Pages scanned:     {}",
        results.pages_scanned.to_string().cyan()
    );
    println!(
        "  Third-party domains: {}",
        results.total_domains.to_string().cyan()
    );
    println!(
        "  Known trackers:    {}",
        results.tracker_domains.to_string().red().bold()
    );

    if results.tracker_domains == 0 {
        println!("\n  {} No known trackers detected.", "✓".green().bold());
        return;
    }

    let tracking_pct = if results.total_domains > 0 {
        (results.tracker_domains as f64 / results.total_domains as f64) * 100.0
    } else {
        0.0
    };
    println!(
        "  Tracking ratio:    {:.1}%",
        tracking_pct.to_string().yellow()
    );

    // Company breakdown (sorted by count descending)
    print_header("TRACKERS BY COMPANY");
    let mut companies: Vec<(&String, &Vec<(String, TrackerInfo)>)> =
        results.by_company.iter().collect();
    companies.sort_by(|a, b| b.1.len().cmp(&a.1.len()));

    let max_count = companies.first().map(|(_, v)| v.len()).unwrap_or(1);

    for (company, trackers) in &companies {
        print_bar(company, trackers.len(), max_count, 30);
        for (domain, info) in *trackers {
            println!(
                "    {} {} ({})",
                "→".dimmed(),
                domain.dimmed(),
                info.category.to_string().dimmed()
            );
        }
    }

    // Category breakdown
    print_header("TRACKERS BY CATEGORY");
    let mut categories: Vec<(&TrackerCategory, &usize)> = results.by_category.iter().collect();
    categories.sort_by(|a, b| b.1.cmp(a.1));
    let max_cat = *categories.first().map(|(_, v)| *v).unwrap_or(&1);

    for (category, count) in &categories {
        let label = format!("{}", category);
        print_bar(&label, **count, max_cat, 30);
    }
}

/// Print a single domain check result
pub fn print_domain_check(domain: &str, info: Option<&TrackerInfo>) {
    match info {
        Some(info) => {
            println!(
                "\n  {} {} is a known tracker",
                "⚠".yellow().bold(),
                domain.red().bold()
            );
            println!("  Company:     {}", info.company.white().bold());
            println!("  Category:    {}", info.category.to_string().yellow());
            println!("  Description: {}", info.description);
        }
        None => {
            println!(
                "\n  {} {} is not in the tracker database",
                "✓".green().bold(),
                domain.green().bold()
            );
        }
    }
}

/// Print tracker database statistics
pub fn print_db_stats(db: &HashMap<&str, TrackerInfo>) {
    print_header("TRACKER DATABASE STATS");

    println!(
        "\n  Total tracker domains: {}",
        db.len().to_string().cyan()
    );

    let mut by_company: HashMap<&str, usize> = HashMap::new();
    let mut by_category: HashMap<&TrackerCategory, usize> = HashMap::new();

    for info in db.values() {
        *by_company.entry(info.company).or_insert(0) += 1;
        *by_category.entry(&info.category).or_insert(0) += 1;
    }

    println!(
        "  Companies tracked:    {}",
        by_company.len().to_string().cyan()
    );

    print_header("DOMAINS PER COMPANY");
    let mut companies: Vec<(&&str, &usize)> = by_company.iter().collect();
    companies.sort_by(|a, b| b.1.cmp(a.1));
    let max = *companies.first().map(|(_, v)| *v).unwrap_or(&1);
    for (company, count) in &companies {
        print_bar(company, **count, max, 30);
    }

    print_header("DOMAINS PER CATEGORY");
    let mut cats: Vec<(&&TrackerCategory, &usize)> = by_category.iter().collect();
    cats.sort_by(|a, b| b.1.cmp(a.1));
    let max_cat = *cats.first().map(|(_, v)| *v).unwrap_or(&1);
    for (cat, count) in &cats {
        let label = format!("{}", cat);
        print_bar(&label, **count, max_cat, 30);
    }
}
