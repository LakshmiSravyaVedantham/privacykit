# privacykit

**Three privacy tools. One Rust binary. Zero dependencies.**

Track your trackers. Find your ghost accounts. Monitor for breaches. All locally, all offline-capable.

```
cargo install privacykit
```

## What's inside

| Command | What it does | Python equivalent |
|---------|-------------|-------------------|
| `privacykit track` | Scans websites for tracking scripts, pixels, and iframes from 72 companies across 163 domains | [trackmap](https://github.com/LakshmiSravyaVedantham/trackmap) |
| `privacykit phantom` | Checks your email against 20+ major breach databases covering 3B+ accounts | [phantom](https://github.com/LakshmiSravyaVedantham/phantom) |
| `privacykit leak` | Continuous breach monitoring with watchlist, alerts, and action tracking | [leakwatch](https://github.com/LakshmiSravyaVedantham/leakwatch) |

## Usage

### Track — map your tracking network

```bash
# Scan URLs from a file for trackers
privacykit track scan urls.txt

# Check if a domain is a known tracker
privacykit track check doubleclick.net

# Show tracker database stats
privacykit track stats
```

```
TRACKING REPORT
━━━━━━━━━━━━━━
Sites scanned:    50
Trackers found:   18 companies across 247 connections

TOP TRACKERS BY REACH
  Google                    ██████████████████░░  78.2%
  Meta                      █████████░░░░░░░░░░░  41.3%
  Amazon                    ██████░░░░░░░░░░░░░░  29.1%
```

### Phantom — find ghost accounts

```bash
# Scan an email for breaches
privacykit phantom scan you@gmail.com

# See your action plan (prioritized)
privacykit phantom actions

# Mark actions done/skipped
privacykit phantom done 3
privacykit phantom skip 5

# Check a password against HIBP (k-anonymity — password never leaves your machine)
privacykit phantom check-password

# Overall progress
privacykit phantom status
```

```
GHOST ACCOUNT REPORT
━━━━━━━━━━━━━━━━━━━━
  you@gmail.com — 12 breaches

    ■ Adobe          2013-10-04  Exposed: Passwords, Email addresses
    ■ LinkedIn       2021-06-22  Exposed: Email addresses, Names, Phone numbers
    ■ Dropbox        2012-07-01  Exposed: Email addresses, Passwords

  ACTIONS
    ID  Priority    Service     Action
    1   CRITICAL    Adobe       Change password
    2   CRITICAL    Dropbox     Change password
    3   HIGH        Adobe       Enable 2FA
```

### Leak — monitor for new breaches

```bash
# Add identifiers to your watchlist
privacykit leak watch personal@gmail.com
privacykit leak watch work@company.com

# Scan for new breaches
privacykit leak scan

# View dashboard
privacykit leak dashboard

# Manage actions
privacykit leak actions
privacykit leak done 1
privacykit leak status
```

```
LEAKWATCH DASHBOARD
━━━━━━━━━━━━━━━━━━━
  WATCHLIST (2 identifiers)
    ● personal@gmail.com     last scan: 2h ago
    ● work@company.com       last scan: 2h ago

  NEW ALERTS: 2
    ⚠ [8/10] Adobe — personal@gmail.com
    ⚠ [6/10] LinkedIn — work@company.com
```

## How it works

- **163 tracker domains** from 72 companies across 5 categories (Advertising, Analytics, Social, Fingerprinting, CDN)
- **20+ major breaches** in the built-in database covering 3 billion+ accounts
- **k-anonymity** for password checking — only 5 chars of SHA1 hash sent to HIBP
- **SQLite** for local state — action tracking, watchlists, scan history
- **Async scanning** with tokio + reqwest for fast page fetches

## Privacy

- Everything runs locally
- No accounts, no cloud, no telemetry
- Built-in databases work completely offline
- HIBP API is optional (only for real-time breach data and password checks)
- Your browsing history, emails, and passwords never leave your machine

## Build from source

```bash
git clone https://github.com/LakshmiSravyaVedantham/privacykit.git
cd privacykit
cargo build --release
./target/release/privacykit --help
```

## Why Rust?

The Python versions (trackmap, phantom, leakwatch) work great but need Python + pip + venv. This is one static binary. Download it, run it, done.

## License

MIT
