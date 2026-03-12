use anyhow::{Context, Result};
use reqwest::Client;
use scraper::{Html, Selector};
use std::collections::HashSet;

/// Extract the domain from a URL string
pub fn extract_domain(url: &str) -> Option<String> {
    // Handle protocol-relative URLs
    let url = if url.starts_with("//") {
        format!("https:{}", url)
    } else {
        url.to_string()
    };

    reqwest::Url::parse(&url)
        .ok()
        .and_then(|u| u.host_str().map(|h| h.to_lowercase()))
}

/// Extract the base domain from a full URL (for comparing with page domain)
pub fn base_domain(domain: &str) -> String {
    let parts: Vec<&str> = domain.split('.').collect();
    if parts.len() >= 2 {
        parts[parts.len() - 2..].join(".")
    } else {
        domain.to_string()
    }
}

/// Fetch a page and extract all third-party domains from resource references
pub async fn scan_page(client: &Client, url: &str) -> Result<HashSet<String>> {
    let response = client
        .get(url)
        .send()
        .await
        .with_context(|| format!("Failed to fetch {}", url))?;

    let body = response
        .text()
        .await
        .with_context(|| format!("Failed to read body of {}", url))?;

    let page_domain = extract_domain(url).unwrap_or_default();
    let page_base = base_domain(&page_domain);

    Ok(extract_third_party_domains(&body, &page_base))
}

/// Parse HTML and extract all third-party domains from resource attributes
pub fn extract_third_party_domains(html: &str, page_base_domain: &str) -> HashSet<String> {
    let document = Html::parse_document(html);
    let mut domains = HashSet::new();

    // Selectors for elements that load external resources
    let selectors = vec![
        ("script[src]", "src"),
        ("img[src]", "src"),
        ("iframe[src]", "src"),
        ("link[href]", "href"),
        ("source[src]", "src"),
        ("video[src]", "src"),
        ("audio[src]", "src"),
    ];

    for (selector_str, attr) in selectors {
        if let Ok(selector) = Selector::parse(selector_str) {
            for element in document.select(&selector) {
                if let Some(value) = element.value().attr(attr) {
                    if let Some(domain) = extract_domain(value) {
                        let domain_base = base_domain(&domain);
                        if domain_base != page_base_domain && !domain.is_empty() {
                            domains.insert(domain);
                        }
                    }
                }
            }
        }
    }

    domains
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_domain_https() {
        assert_eq!(
            extract_domain("https://www.google-analytics.com/analytics.js"),
            Some("www.google-analytics.com".to_string())
        );
    }

    #[test]
    fn test_extract_domain_protocol_relative() {
        assert_eq!(
            extract_domain("//cdn.example.com/script.js"),
            Some("cdn.example.com".to_string())
        );
    }

    #[test]
    fn test_extract_domain_invalid() {
        assert_eq!(extract_domain("/local/path.js"), None);
    }

    #[test]
    fn test_base_domain() {
        assert_eq!(base_domain("www.google-analytics.com"), "google-analytics.com");
        assert_eq!(base_domain("example.com"), "example.com");
    }

    #[test]
    fn test_extract_third_party_domains() {
        let html = r#"
        <html>
        <head>
            <script src="https://www.google-analytics.com/analytics.js"></script>
            <script src="https://connect.facebook.net/en_US/fbevents.js"></script>
            <script src="/local/script.js"></script>
            <link href="https://example.com/style.css" rel="stylesheet">
        </head>
        <body>
            <img src="https://doubleclick.net/pixel.gif">
            <iframe src="https://platform.twitter.com/widgets.js"></iframe>
        </body>
        </html>
        "#;

        let domains = extract_third_party_domains(html, "example.com");
        assert!(domains.contains("www.google-analytics.com"));
        assert!(domains.contains("connect.facebook.net"));
        assert!(domains.contains("doubleclick.net"));
        assert!(domains.contains("platform.twitter.com"));
        // Local paths should not appear
        assert!(!domains.iter().any(|d| d.contains("local")));
        // Same-domain should not appear
        assert!(!domains.contains("example.com"));
    }
}
