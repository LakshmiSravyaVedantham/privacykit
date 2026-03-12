use std::collections::HashMap;

/// Category of tracking behavior
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum TrackerCategory {
    Advertising,
    Analytics,
    Social,
    Fingerprinting,
    Content,
}

impl std::fmt::Display for TrackerCategory {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TrackerCategory::Advertising => write!(f, "Advertising"),
            TrackerCategory::Analytics => write!(f, "Analytics"),
            TrackerCategory::Social => write!(f, "Social"),
            TrackerCategory::Fingerprinting => write!(f, "Fingerprinting"),
            TrackerCategory::Content => write!(f, "Content"),
        }
    }
}

/// Information about a known tracker
#[derive(Debug, Clone)]
pub struct TrackerInfo {
    pub company: &'static str,
    pub category: TrackerCategory,
    pub description: &'static str,
}

/// Build the tracker database with 50+ known tracker domains
pub fn build_tracker_db() -> HashMap<&'static str, TrackerInfo> {
    let mut db = HashMap::new();

    // --- Google (12 domains) ---
    let google_ads = TrackerInfo {
        company: "Google",
        category: TrackerCategory::Advertising,
        description: "Google advertising network",
    };
    for domain in &[
        "doubleclick.net",
        "googlesyndication.com",
        "googleadservices.com",
        "googleads.g.doubleclick.net",
        "adservice.google.com",
        "pagead2.googlesyndication.com",
    ] {
        db.insert(*domain, google_ads.clone());
    }

    let google_analytics = TrackerInfo {
        company: "Google",
        category: TrackerCategory::Analytics,
        description: "Google Analytics tracking",
    };
    for domain in &[
        "google-analytics.com",
        "googletagmanager.com",
        "googletagservices.com",
        "analytics.google.com",
        "tagmanager.google.com",
        "www.googletagmanager.com",
    ] {
        db.insert(*domain, google_analytics.clone());
    }

    // --- Meta / Facebook (6 domains) ---
    let meta_social = TrackerInfo {
        company: "Meta",
        category: TrackerCategory::Social,
        description: "Meta social tracking pixel",
    };
    for domain in &[
        "facebook.net",
        "connect.facebook.net",
        "facebook.com",
        "fbcdn.net",
        "fb.com",
        "instagram.com",
    ] {
        db.insert(*domain, meta_social.clone());
    }

    // --- Amazon (4 domains) ---
    let amazon_ads = TrackerInfo {
        company: "Amazon",
        category: TrackerCategory::Advertising,
        description: "Amazon advertising and tracking",
    };
    for domain in &[
        "amazon-adsystem.com",
        "assoc-amazon.com",
        "advertising.amazon.com",
        "aax.amazon-adsystem.com",
    ] {
        db.insert(*domain, amazon_ads.clone());
    }

    // --- Microsoft (4 domains) ---
    let ms_analytics = TrackerInfo {
        company: "Microsoft",
        category: TrackerCategory::Analytics,
        description: "Microsoft analytics and advertising",
    };
    for domain in &[
        "clarity.ms",
        "bat.bing.com",
        "bing.com",
        "c.bing.com",
    ] {
        db.insert(*domain, ms_analytics.clone());
    }

    // --- Adobe (3 domains) ---
    let adobe = TrackerInfo {
        company: "Adobe",
        category: TrackerCategory::Analytics,
        description: "Adobe Experience Cloud tracking",
    };
    for domain in &[
        "demdex.net",
        "omtrdc.net",
        "everesttech.net",
    ] {
        db.insert(*domain, adobe.clone());
    }

    // --- Oracle (3 domains) ---
    let oracle = TrackerInfo {
        company: "Oracle",
        category: TrackerCategory::Advertising,
        description: "Oracle Data Cloud / BlueKai tracking",
    };
    for domain in &[
        "bluekai.com",
        "addthis.com",
        "moatads.com",
    ] {
        db.insert(*domain, oracle.clone());
    }

    // --- Criteo (3 domains) ---
    let criteo = TrackerInfo {
        company: "Criteo",
        category: TrackerCategory::Advertising,
        description: "Criteo retargeting and advertising",
    };
    for domain in &[
        "criteo.com",
        "criteo.net",
        "emailretargeting.com",
    ] {
        db.insert(*domain, criteo.clone());
    }

    // --- Twitter/X (2 domains) ---
    let twitter = TrackerInfo {
        company: "Twitter/X",
        category: TrackerCategory::Social,
        description: "Twitter/X social tracking",
    };
    for domain in &["platform.twitter.com", "t.co"] {
        db.insert(*domain, twitter.clone());
    }

    // --- LinkedIn (2 domains) ---
    let linkedin = TrackerInfo {
        company: "LinkedIn",
        category: TrackerCategory::Social,
        description: "LinkedIn Insight tracking",
    };
    for domain in &["snap.licdn.com", "linkedin.com"] {
        db.insert(*domain, linkedin.clone());
    }

    // --- TikTok (2 domains) ---
    let tiktok = TrackerInfo {
        company: "TikTok",
        category: TrackerCategory::Social,
        description: "TikTok pixel tracking",
    };
    for domain in &["analytics.tiktok.com", "tiktok.com"] {
        db.insert(*domain, tiktok.clone());
    }

    // --- Pinterest (1 domain) ---
    db.insert(
        "ct.pinterest.com",
        TrackerInfo {
            company: "Pinterest",
            category: TrackerCategory::Social,
            description: "Pinterest conversion tracking",
        },
    );

    // --- Hotjar (2 domains) ---
    let hotjar = TrackerInfo {
        company: "Hotjar",
        category: TrackerCategory::Analytics,
        description: "Hotjar session recording and heatmaps",
    };
    for domain in &["hotjar.com", "static.hotjar.com"] {
        db.insert(*domain, hotjar.clone());
    }

    // --- New Relic (1 domain) ---
    db.insert(
        "nr-data.net",
        TrackerInfo {
            company: "New Relic",
            category: TrackerCategory::Analytics,
            description: "New Relic performance monitoring",
        },
    );

    // --- Segment (1 domain) ---
    db.insert(
        "segment.io",
        TrackerInfo {
            company: "Segment",
            category: TrackerCategory::Analytics,
            description: "Segment customer data platform",
        },
    );

    // --- Mixpanel (1 domain) ---
    db.insert(
        "mixpanel.com",
        TrackerInfo {
            company: "Mixpanel",
            category: TrackerCategory::Analytics,
            description: "Mixpanel product analytics",
        },
    );

    // --- Quantcast (1 domain) ---
    db.insert(
        "quantserve.com",
        TrackerInfo {
            company: "Quantcast",
            category: TrackerCategory::Advertising,
            description: "Quantcast audience measurement",
        },
    );

    // --- The Trade Desk (1 domain) ---
    db.insert(
        "adsrvr.org",
        TrackerInfo {
            company: "The Trade Desk",
            category: TrackerCategory::Advertising,
            description: "The Trade Desk programmatic advertising",
        },
    );

    // --- Taboola (1 domain) ---
    db.insert(
        "taboola.com",
        TrackerInfo {
            company: "Taboola",
            category: TrackerCategory::Content,
            description: "Taboola content recommendation",
        },
    );

    // --- Outbrain (1 domain) ---
    db.insert(
        "outbrain.com",
        TrackerInfo {
            company: "Outbrain",
            category: TrackerCategory::Content,
            description: "Outbrain content recommendation",
        },
    );

    // --- MediaMath (1 domain) ---
    db.insert(
        "mathtag.com",
        TrackerInfo {
            company: "MediaMath",
            category: TrackerCategory::Advertising,
            description: "MediaMath demand-side platform",
        },
    );

    // --- AppNexus / Xandr (1 domain) ---
    db.insert(
        "adnxs.com",
        TrackerInfo {
            company: "Xandr (Microsoft)",
            category: TrackerCategory::Advertising,
            description: "AppNexus/Xandr programmatic advertising",
        },
    );

    // --- Rubicon Project (1 domain) ---
    db.insert(
        "rubiconproject.com",
        TrackerInfo {
            company: "Magnite",
            category: TrackerCategory::Advertising,
            description: "Magnite (Rubicon Project) ad exchange",
        },
    );

    // --- Scorecard Research (1 domain) ---
    db.insert(
        "scorecardresearch.com",
        TrackerInfo {
            company: "comScore",
            category: TrackerCategory::Analytics,
            description: "comScore audience measurement",
        },
    );

    // --- Cloudflare Insights (1 domain) ---
    db.insert(
        "static.cloudflareinsights.com",
        TrackerInfo {
            company: "Cloudflare",
            category: TrackerCategory::Analytics,
            description: "Cloudflare Web Analytics",
        },
    );

    // --- Snap / Snapchat (1 domain) ---
    db.insert(
        "sc-static.net",
        TrackerInfo {
            company: "Snap Inc.",
            category: TrackerCategory::Social,
            description: "Snapchat pixel tracking",
        },
    );

    // --- Heap Analytics (1 domain) ---
    db.insert(
        "heapanalytics.com",
        TrackerInfo {
            company: "Heap",
            category: TrackerCategory::Analytics,
            description: "Heap product analytics",
        },
    );

    // --- FullStory (1 domain) ---
    db.insert(
        "fullstory.com",
        TrackerInfo {
            company: "FullStory",
            category: TrackerCategory::Fingerprinting,
            description: "FullStory session replay and analytics",
        },
    );

    // --- Optimizely (1 domain) ---
    db.insert(
        "optimizely.com",
        TrackerInfo {
            company: "Optimizely",
            category: TrackerCategory::Analytics,
            description: "Optimizely experimentation platform",
        },
    );

    // --- LiveRamp (1 domain) ---
    db.insert(
        "rlcdn.com",
        TrackerInfo {
            company: "LiveRamp",
            category: TrackerCategory::Fingerprinting,
            description: "LiveRamp identity resolution",
        },
    );

    // --- ID5 (1 domain) ---
    db.insert(
        "id5-sync.com",
        TrackerInfo {
            company: "ID5",
            category: TrackerCategory::Fingerprinting,
            description: "ID5 universal ID for advertising",
        },
    );

    db
}

/// Check if a domain or any of its parent domains is a known tracker
pub fn lookup_domain<'a>(
    db: &'a HashMap<&'static str, TrackerInfo>,
    domain: &str,
) -> Option<&'a TrackerInfo> {
    let domain = domain.to_lowercase();
    // Try exact match first
    if let Some(info) = db.get(domain.as_str()) {
        return Some(info);
    }
    // Try parent domains (e.g., "www.google-analytics.com" -> "google-analytics.com")
    let parts: Vec<&str> = domain.split('.').collect();
    for i in 1..parts.len().saturating_sub(1) {
        let parent = parts[i..].join(".");
        if let Some(info) = db.get(parent.as_str()) {
            return Some(info);
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tracker_db_has_50_plus_entries() {
        let db = build_tracker_db();
        assert!(
            db.len() >= 50,
            "Tracker DB should have 50+ entries, got {}",
            db.len()
        );
    }

    #[test]
    fn test_lookup_exact_domain() {
        let db = build_tracker_db();
        let info = lookup_domain(&db, "doubleclick.net");
        assert!(info.is_some());
        assert_eq!(info.unwrap().company, "Google");
    }

    #[test]
    fn test_lookup_subdomain() {
        let db = build_tracker_db();
        let info = lookup_domain(&db, "www.google-analytics.com");
        assert!(info.is_some());
        assert_eq!(info.unwrap().company, "Google");
    }

    #[test]
    fn test_lookup_unknown_domain() {
        let db = build_tracker_db();
        let info = lookup_domain(&db, "example.com");
        assert!(info.is_none());
    }

    #[test]
    fn test_all_categories_present() {
        let db = build_tracker_db();
        let categories: std::collections::HashSet<&TrackerCategory> =
            db.values().map(|v| &v.category).collect();
        assert!(categories.contains(&TrackerCategory::Advertising));
        assert!(categories.contains(&TrackerCategory::Analytics));
        assert!(categories.contains(&TrackerCategory::Social));
        assert!(categories.contains(&TrackerCategory::Fingerprinting));
        assert!(categories.contains(&TrackerCategory::Content));
    }
}
