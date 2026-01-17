/*
 * Copyright © 2026 Daniel Wetzel
 * Licensed under the Apache License, Version 2.0
 * https://github.com/danielkbx/salutare
 */

use anyhow::Result;
use axum::http::{Method, header};
use axum::{Router, routing::get};
use std::net::SocketAddr;
use std::time::Duration;
use tower_http::cors::CorsLayer;
use tracing::{Level, info};
use tracing_subscriber::EnvFilter;

use salutare::config::Config;
use salutare::greetings::{self, load_greetings_csv};
use salutare::http::handlers;
use salutare::state::AppState;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info")),
        )
        .with_max_level(Level::INFO)
        .with_target(false)
        .init();

    let csv_path = std::env::var("CSV_PATH").unwrap_or_else(|_| "greetings.csv".to_string());
    let salt = std::env::var("SALUTARE_SALT").unwrap_or_else(|_| "dev-salt-change-me".to_string());
    let cfg = Config::from_env()?;
    info!(
        "Config: csv_path={:?}, bind_addr={}",
        cfg.csv_path, cfg.bind_addr
    );

    let greetings = load_greetings_csv(&cfg.csv_path)?;
    info!(
        "Loaded {} greetings from {:?}",
        greetings.len(),
        cfg.csv_path
    );

    let permutations = greetings::build_deterministic_permutation(greetings.len(), &cfg.salt)?;
    info!(
        "Built deterministic permutation with {} entries",
        permutations.len()
    );

    if permutations.len() != greetings.len() {
        anyhow::bail!(
            "Invariant violated: permutations.len()={} but greetings.len()={}",
            permutations.len(),
            greetings.len()
        );
    }

    let state = AppState {
        greetings: std::sync::Arc::new(greetings),
        permutations: std::sync::Arc::new(permutations),
        slack_signing_secret: std::sync::Arc::new(cfg.slack_signing_secret),
    };

    let cors = CorsLayer::new()
        // Public API: allow browser calls from any origin.
        .allow_origin(tower_http::cors::Any)
        // Only allow safe, read-only methods.
        .allow_methods([Method::GET, Method::OPTIONS])
        // Keep allowed headers minimal. (OPTIONS preflight will validate this.)
        .allow_headers([header::ACCEPT, header::CONTENT_TYPE])
        // Cache preflight results in browsers to reduce OPTIONS traffic.
        .max_age(Duration::from_secs(60 * 60));

    let app = Router::new()
        .route("/api/v1/healthz", get(salutare::http::handlers::healthz))
        .route("/api/v1/greeting", get(handlers::greeting))
        .route("/slack/command", axum::routing::post(salutare::http::slack::command))
        .with_state(state)
        .layer(cors);

    let addr = cfg.bind_addr;
    info!("Salutare listening on http://{}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}
