use anyhow::{Context, Result};
use std::{net::SocketAddr, path::PathBuf};

/// Central configuration for the Salutare service.
///
/// The configuration is loaded once at startup from environment variables.
/// Defaults are chosen to be developer-friendly while remaining production-safe.
///
/// Environment variables:
/// - CSV_PATH: path to greetings CSV file (default: "greetings.csv")
/// - SALUTARE_SALT: secret salt used to build deterministic permutations (default: "dev-salt-change-me")
/// - BIND_ADDR: socket address the Rust service binds to (default: "127.0.0.1:8080")
///
/// Notes:
/// - In production, bind to localhost and use Nginx for TLS and rate limiting.
/// - SALUTARE_SALT should be set to a strong, private value in production.
#[derive(Debug, Clone)]
pub struct Config {
    /// CSV file path used at startup.
    pub csv_path: PathBuf,

    /// Salt used to create deterministic permutations.
    pub salt: String,

    /// Bind address for the HTTP server.
    pub bind_addr: SocketAddr,
}

impl Config {
    /// Load configuration from environment variables, apply defaults,
    /// and validate basic invariants.
    pub fn from_env() -> Result<Self> {
        // CSV path: default to repo-local file for dev.
        let csv_path = std::env::var("CSV_PATH").unwrap_or_else(|_| "greetings.csv".to_string());
        let csv_path = PathBuf::from(csv_path);

        // Salt: default is intentionally not secure, to encourage overriding in prod.
        let salt =
            std::env::var("SALUTARE_SALT").unwrap_or_else(|_| "dev-salt-change-me".to_string());

        // Bind address: localhost by default (safe for production behind a reverse proxy).
        let bind_addr = std::env::var("BIND_ADDR").unwrap_or_else(|_| "127.0.0.1:8080".to_string());
        let bind_addr: SocketAddr = bind_addr
            .parse()
            .with_context(|| format!("Invalid BIND_ADDR '{}'", bind_addr))?;

        let cfg = Self {
            csv_path,
            salt,
            bind_addr,
        };

        cfg.validate()?;
        Ok(cfg)
    }

    /// Validate configuration invariants early.
    ///
    /// We fail fast at startup to avoid running a partially-configured service.
    fn validate(&self) -> Result<()> {
        // Ensure the CSV path exists and is a regular file.
        // This avoids confusing runtime errors later.
        let meta = std::fs::metadata(&self.csv_path).with_context(|| {
            format!(
                "CSV_PATH does not exist or cannot be accessed: {:?}",
                self.csv_path
            )
        })?;

        if !meta.is_file() {
            anyhow::bail!("CSV_PATH is not a file: {:?}", self.csv_path);
        }

        // Very lightweight salt sanity check.
        // We do not enforce strong secrets here, but we provide guardrails.
        if self.salt.trim().is_empty() {
            anyhow::bail!("SALUTARE_SALT must not be empty");
        }

        Ok(())
    }
}
