/// Information about a known data breach
#[derive(Debug, Clone)]
pub struct BreachInfo {
    pub name: &'static str,
    pub domain: &'static str,
    pub date: &'static str,
    pub data_types: Vec<&'static str>,
    pub accounts_affected: u64,
}

/// Build the breach database with 20+ major breaches
pub fn build_breach_db() -> Vec<BreachInfo> {
    vec![
        BreachInfo {
            name: "Adobe",
            domain: "adobe.com",
            date: "2013-10-04",
            data_types: vec!["Email addresses", "Passwords", "Usernames", "Password hints"],
            accounts_affected: 153_000_000,
        },
        BreachInfo {
            name: "LinkedIn",
            domain: "linkedin.com",
            date: "2012-05-05",
            data_types: vec!["Email addresses", "Passwords"],
            accounts_affected: 164_000_000,
        },
        BreachInfo {
            name: "Yahoo",
            domain: "yahoo.com",
            date: "2013-08-01",
            data_types: vec!["Email addresses", "Passwords", "Names", "Phone numbers", "Dates of birth", "Security questions"],
            accounts_affected: 3_000_000_000,
        },
        BreachInfo {
            name: "Facebook",
            domain: "facebook.com",
            date: "2019-04-01",
            data_types: vec!["Email addresses", "Phone numbers", "Names", "Dates of birth", "Locations"],
            accounts_affected: 533_000_000,
        },
        BreachInfo {
            name: "Equifax",
            domain: "equifax.com",
            date: "2017-09-07",
            data_types: vec!["Names", "SSN", "Dates of birth", "Addresses", "Credit card numbers"],
            accounts_affected: 147_000_000,
        },
        BreachInfo {
            name: "Marriott/Starwood",
            domain: "marriott.com",
            date: "2018-11-30",
            data_types: vec!["Email addresses", "Names", "Phone numbers", "Passport numbers", "Credit card numbers"],
            accounts_affected: 500_000_000,
        },
        BreachInfo {
            name: "MySpace",
            domain: "myspace.com",
            date: "2008-07-01",
            data_types: vec!["Email addresses", "Passwords", "Usernames"],
            accounts_affected: 360_000_000,
        },
        BreachInfo {
            name: "Dropbox",
            domain: "dropbox.com",
            date: "2012-07-01",
            data_types: vec!["Email addresses", "Passwords"],
            accounts_affected: 68_000_000,
        },
        BreachInfo {
            name: "Twitter",
            domain: "twitter.com",
            date: "2022-01-01",
            data_types: vec!["Email addresses", "Phone numbers", "Names", "Usernames"],
            accounts_affected: 200_000_000,
        },
        BreachInfo {
            name: "Canva",
            domain: "canva.com",
            date: "2019-05-24",
            data_types: vec!["Email addresses", "Passwords", "Names", "Usernames"],
            accounts_affected: 137_000_000,
        },
        BreachInfo {
            name: "Zynga",
            domain: "zynga.com",
            date: "2019-09-01",
            data_types: vec!["Email addresses", "Passwords", "Usernames", "Phone numbers"],
            accounts_affected: 173_000_000,
        },
        BreachInfo {
            name: "Capital One",
            domain: "capitalone.com",
            date: "2019-07-29",
            data_types: vec!["Names", "Addresses", "Phone numbers", "SSN", "Credit card numbers", "Financial data"],
            accounts_affected: 106_000_000,
        },
        BreachInfo {
            name: "Dubsmash",
            domain: "dubsmash.com",
            date: "2018-12-01",
            data_types: vec!["Email addresses", "Passwords", "Usernames"],
            accounts_affected: 162_000_000,
        },
        BreachInfo {
            name: "Under Armour / MyFitnessPal",
            domain: "myfitnesspal.com",
            date: "2018-02-01",
            data_types: vec!["Email addresses", "Passwords", "Usernames", "IP addresses"],
            accounts_affected: 150_000_000,
        },
        BreachInfo {
            name: "Exactis",
            domain: "exactis.com",
            date: "2018-06-01",
            data_types: vec!["Email addresses", "Names", "Addresses", "Phone numbers"],
            accounts_affected: 340_000_000,
        },
        BreachInfo {
            name: "T-Mobile",
            domain: "t-mobile.com",
            date: "2021-08-17",
            data_types: vec!["Names", "SSN", "Dates of birth", "Phone numbers", "Addresses"],
            accounts_affected: 77_000_000,
        },
        BreachInfo {
            name: "Twitch",
            domain: "twitch.tv",
            date: "2021-10-06",
            data_types: vec!["Email addresses", "Passwords", "Financial data"],
            accounts_affected: 5_000_000,
        },
        BreachInfo {
            name: "Uber",
            domain: "uber.com",
            date: "2016-10-01",
            data_types: vec!["Email addresses", "Names", "Phone numbers"],
            accounts_affected: 57_000_000,
        },
        BreachInfo {
            name: "LastPass",
            domain: "lastpass.com",
            date: "2022-12-01",
            data_types: vec!["Email addresses", "Passwords", "Names", "Billing addresses"],
            accounts_affected: 33_000_000,
        },
        BreachInfo {
            name: "Anthem",
            domain: "anthem.com",
            date: "2015-02-04",
            data_types: vec!["Names", "SSN", "Dates of birth", "Addresses", "Email addresses"],
            accounts_affected: 78_800_000,
        },
        BreachInfo {
            name: "eBay",
            domain: "ebay.com",
            date: "2014-05-01",
            data_types: vec!["Email addresses", "Passwords", "Names", "Addresses", "Phone numbers", "Dates of birth"],
            accounts_affected: 145_000_000,
        },
        BreachInfo {
            name: "MOVEit",
            domain: "moveit.com",
            date: "2023-06-01",
            data_types: vec!["Names", "SSN", "Financial data", "Addresses"],
            accounts_affected: 77_000_000,
        },
        BreachInfo {
            name: "23andMe",
            domain: "23andme.com",
            date: "2023-10-01",
            data_types: vec!["Names", "Dates of birth", "Genetic data", "Locations"],
            accounts_affected: 6_900_000,
        },
        BreachInfo {
            name: "Ticketmaster",
            domain: "ticketmaster.com",
            date: "2024-05-01",
            data_types: vec!["Names", "Email addresses", "Credit card numbers", "Addresses"],
            accounts_affected: 560_000_000,
        },
    ]
}

/// Search the breach database for breaches matching an email domain
pub fn find_breaches_for_email(email: &str) -> Vec<BreachInfo> {
    let db = build_breach_db();
    let email_domain = email
        .split('@')
        .nth(1)
        .unwrap_or("")
        .to_lowercase();

    // In a real tool you'd check HIBP API. Here we simulate by checking
    // if the email domain matches any breach domain, or return common
    // breaches that affect everyone (major platforms).
    //
    // For demo purposes: return breaches for well-known domains that most
    // people have accounts on, plus any exact domain match.
    let common_breach_domains = vec![
        "linkedin.com",
        "facebook.com",
        "twitter.com",
        "adobe.com",
        "dropbox.com",
        "canva.com",
    ];

    db.into_iter()
        .filter(|b| {
            b.domain == email_domain || common_breach_domains.contains(&b.domain)
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_breach_db_has_20_plus() {
        let db = build_breach_db();
        assert!(
            db.len() >= 20,
            "Breach DB should have 20+ entries, got {}",
            db.len()
        );
    }

    #[test]
    fn test_find_breaches_for_common_email() {
        let breaches = find_breaches_for_email("user@gmail.com");
        assert!(!breaches.is_empty(), "Should find breaches for gmail user");
    }

    #[test]
    fn test_find_breaches_for_matching_domain() {
        let breaches = find_breaches_for_email("user@yahoo.com");
        assert!(
            breaches.iter().any(|b| b.name == "Yahoo"),
            "Should find Yahoo breach for yahoo.com email"
        );
    }
}
