use anyhow::{Context, Result};
use sha1::{Digest, Sha1};

/// Check a password against the HIBP Pwned Passwords API using k-anonymity.
///
/// 1. SHA1 hash the password
/// 2. Send the first 5 characters to api.pwnedpasswords.com/range/
/// 3. Match the remaining hash suffix locally
/// 4. Return the number of times seen (0 = not found)
pub async fn check_password_hibp(password: &str) -> Result<u64> {
    let hash = sha1_hex(password);
    let prefix = &hash[..5];
    let suffix = &hash[5..];

    let client = crate::common::http_client()?;
    let url = format!("https://api.pwnedpasswords.com/range/{}", prefix);

    let response = client
        .get(&url)
        .send()
        .await
        .context("Failed to query HIBP Pwned Passwords API")?;

    let body = response.text().await.context("Failed to read HIBP response")?;

    for line in body.lines() {
        let parts: Vec<&str> = line.split(':').collect();
        if parts.len() == 2 {
            let line_suffix = parts[0].trim().to_uppercase();
            if line_suffix == suffix.to_uppercase() {
                let count: u64 = parts[1].trim().parse().unwrap_or(0);
                return Ok(count);
            }
        }
    }

    Ok(0)
}

/// Compute SHA1 hex digest of a string
pub fn sha1_hex(input: &str) -> String {
    let mut hasher = Sha1::new();
    hasher.update(input.as_bytes());
    let result = hasher.finalize();
    hex_encode(&result)
}

/// Encode bytes as uppercase hex string
fn hex_encode(bytes: &[u8]) -> String {
    bytes.iter().map(|b| format!("{:02X}", b)).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sha1_hex_known_value() {
        // SHA1("password") = 5BAA61E4C9B93F3F0682250B6CF8331B7EE68FD8
        assert_eq!(sha1_hex("password"), "5BAA61E4C9B93F3F0682250B6CF8331B7EE68FD8");
    }

    #[test]
    fn test_sha1_hex_empty() {
        // SHA1("") = DA39A3EE5E6B4B0D3255BFEF95601890AFD80709
        assert_eq!(sha1_hex(""), "DA39A3EE5E6B4B0D3255BFEF95601890AFD80709");
    }
}
