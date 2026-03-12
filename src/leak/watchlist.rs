use anyhow::{Context, Result};
use rusqlite::Connection;

/// Initialize the leakwatch SQLite tables
pub fn init_db(conn: &Connection) -> Result<()> {
    conn.execute_batch(
        "CREATE TABLE IF NOT EXISTS watchlist (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            identifier TEXT NOT NULL UNIQUE,
            added_at TEXT NOT NULL DEFAULT (datetime('now'))
        );

        CREATE TABLE IF NOT EXISTS seen_breaches (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            identifier TEXT NOT NULL,
            breach_name TEXT NOT NULL,
            first_seen TEXT NOT NULL DEFAULT (datetime('now')),
            UNIQUE(identifier, breach_name)
        );

        CREATE TABLE IF NOT EXISTS leak_actions (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            breach_name TEXT NOT NULL,
            identifier TEXT NOT NULL,
            action TEXT NOT NULL,
            severity TEXT NOT NULL,
            status TEXT NOT NULL DEFAULT 'pending',
            created_at TEXT NOT NULL DEFAULT (datetime('now')),
            completed_at TEXT
        );",
    )
    .context("Failed to create leakwatch tables")?;
    Ok(())
}

/// Add an identifier to the watchlist
pub fn add_watch(conn: &Connection, identifier: &str) -> Result<bool> {
    let result = conn.execute(
        "INSERT OR IGNORE INTO watchlist (identifier) VALUES (?1)",
        rusqlite::params![identifier],
    )?;
    Ok(result > 0)
}

/// Remove an identifier from the watchlist
pub fn remove_watch(conn: &Connection, identifier: &str) -> Result<bool> {
    let result = conn.execute(
        "DELETE FROM watchlist WHERE identifier = ?1",
        rusqlite::params![identifier],
    )?;
    Ok(result > 0)
}

/// Get all watched identifiers
pub fn get_watchlist(conn: &Connection) -> Result<Vec<(i64, String, String)>> {
    let mut stmt =
        conn.prepare("SELECT id, identifier, added_at FROM watchlist ORDER BY added_at")?;
    let rows = stmt
        .query_map([], |row| {
            Ok((
                row.get::<_, i64>(0)?,
                row.get::<_, String>(1)?,
                row.get::<_, String>(2)?,
            ))
        })?
        .collect::<Result<Vec<_>, _>>()
        .context("Failed to fetch watchlist")?;
    Ok(rows)
}

/// Check if a breach has been seen before for a given identifier
pub fn is_known_breach(conn: &Connection, identifier: &str, breach_name: &str) -> Result<bool> {
    let count: i64 = conn.query_row(
        "SELECT COUNT(*) FROM seen_breaches WHERE identifier = ?1 AND breach_name = ?2",
        rusqlite::params![identifier, breach_name],
        |row| row.get(0),
    )?;
    Ok(count > 0)
}

/// Record a breach as seen for a given identifier
pub fn record_breach_seen(
    conn: &Connection,
    identifier: &str,
    breach_name: &str,
) -> Result<()> {
    conn.execute(
        "INSERT OR IGNORE INTO seen_breaches (identifier, breach_name) VALUES (?1, ?2)",
        rusqlite::params![identifier, breach_name],
    )?;
    Ok(())
}

/// Get count of seen breaches for an identifier
pub fn breach_count(conn: &Connection, identifier: &str) -> Result<usize> {
    let count: usize = conn.query_row(
        "SELECT COUNT(*) FROM seen_breaches WHERE identifier = ?1",
        rusqlite::params![identifier],
        |row| row.get(0),
    )?;
    Ok(count)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn setup_test_db() -> Connection {
        let conn = Connection::open_in_memory().unwrap();
        conn.execute_batch("PRAGMA journal_mode=WAL; PRAGMA foreign_keys=ON;")
            .unwrap();
        init_db(&conn).unwrap();
        conn
    }

    #[test]
    fn test_add_and_get_watchlist() {
        let conn = setup_test_db();
        assert!(add_watch(&conn, "test@example.com").unwrap());
        let list = get_watchlist(&conn).unwrap();
        assert_eq!(list.len(), 1);
        assert_eq!(list[0].1, "test@example.com");
    }

    #[test]
    fn test_add_duplicate_watch() {
        let conn = setup_test_db();
        assert!(add_watch(&conn, "test@example.com").unwrap());
        assert!(!add_watch(&conn, "test@example.com").unwrap());
    }

    #[test]
    fn test_remove_watch() {
        let conn = setup_test_db();
        add_watch(&conn, "test@example.com").unwrap();
        assert!(remove_watch(&conn, "test@example.com").unwrap());
        let list = get_watchlist(&conn).unwrap();
        assert!(list.is_empty());
    }

    #[test]
    fn test_breach_tracking() {
        let conn = setup_test_db();
        add_watch(&conn, "test@example.com").unwrap();

        assert!(!is_known_breach(&conn, "test@example.com", "Adobe").unwrap());
        record_breach_seen(&conn, "test@example.com", "Adobe").unwrap();
        assert!(is_known_breach(&conn, "test@example.com", "Adobe").unwrap());
        assert_eq!(breach_count(&conn, "test@example.com").unwrap(), 1);
    }
}
