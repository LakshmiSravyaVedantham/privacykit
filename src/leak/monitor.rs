use anyhow::Result;
use rusqlite::Connection;

use crate::common::actions_for_data_types;
use crate::phantom::breach_db::{find_breaches_for_email, BreachInfo};

use super::actions::insert_action;
use super::watchlist::{get_watchlist, is_known_breach, record_breach_seen};

/// Result of scanning a single identifier
pub struct ScanResult {
    pub identifier: String,
    pub new_breaches: Vec<BreachInfo>,
    pub known_breaches: usize,
    pub actions_generated: usize,
}

/// Scan all watched identifiers for new breaches
pub fn scan_all(conn: &Connection) -> Result<Vec<ScanResult>> {
    let watchlist = get_watchlist(conn)?;
    let mut results = Vec::new();

    for (_id, identifier, _added) in &watchlist {
        let result = scan_identifier(conn, identifier)?;
        results.push(result);
    }

    Ok(results)
}

/// Scan a single identifier for new breaches
pub fn scan_identifier(conn: &Connection, identifier: &str) -> Result<ScanResult> {
    let breaches = find_breaches_for_email(identifier);
    let mut new_breaches = Vec::new();
    let mut known_count = 0;
    let mut action_count = 0;

    for breach in &breaches {
        if is_known_breach(conn, identifier, breach.name)? {
            known_count += 1;
        } else {
            record_breach_seen(conn, identifier, breach.name)?;
            new_breaches.push(breach.clone());

            // Generate actions for new breaches
            let data_types: Vec<String> =
                breach.data_types.iter().map(|s| s.to_string()).collect();
            let actions = actions_for_data_types(breach.name, &data_types);
            for (action_text, severity) in &actions {
                insert_action(conn, breach.name, identifier, &action_text, severity)?;
                action_count += 1;
            }
        }
    }

    Ok(ScanResult {
        identifier: identifier.to_string(),
        new_breaches,
        known_breaches: known_count,
        actions_generated: action_count,
    })
}
