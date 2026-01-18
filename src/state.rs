/*
 * Copyright © 2026 Daniel Wetzel
 * Licensed under the Apache License, Version 2.0
 * https://github.com/danielkbx/salutare
 */

use crate::greetings::GreetingRow;
use std::sync::Arc;
use std::collections::HashMap;
use std::time::Instant;
use tokio::sync::RwLock;

/// Application state shared across request handlers.
///
/// We keep greetings in memory and share them using `Arc` so that:
/// - We avoid copying the entire vector per request
/// - Cloning state is cheap
///
/// The data is immutable after startup, so we do not need locks.
#[derive(Debug, Clone)]
pub struct AppState {
    pub greetings: Arc<Vec<GreetingRow>>,
    /// Deterministic shuffled index order over `greetings`.
    pub permutations: Arc<Vec<usize>>,
    /// Optional Slack signing key
    pub slack_signing_secret: std::sync::Arc<Option<String>>,
    /// Optional Slack OAuth token
    pub slack_bot_token: std::sync::Arc<Option<String>>,

    /// Small in-memory cache for Slack display name lookups.
    /// Key: Slack user_id (e.g. U12345)
    /// Value: (display_name, inserted_at)
    pub slack_name_cache: std::sync::Arc<RwLock<HashMap<String, (String, Instant)>>>,
}