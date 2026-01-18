/*
 * Copyright © 2026 Daniel Wetzel
 * Licensed under the Apache License, Version 2.0
 * https://github.com/danielkbx/salutare
 */

use axum::{
    http::StatusCode,
    response::{Html, IntoResponse},
    extract::Query,
};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct OAuthCallbackQuery {
    pub code: Option<String>,
    pub state: Option<String>,
    pub error: Option<String>,
}

/// Minimal OAuth callback endpoint.
///
/// Purpose:
/// - Slack requires a Redirect URL for app distribution / marketplace.
/// - We intentionally do NOT exchange `code` for a token (no persistence, no scopes required).
/// - This endpoint simply confirms installation to the user.
///
/// Notes:
/// - Slack will call this with either `code`+`state` OR `error`.
pub async fn callback(Query(q): Query<OAuthCallbackQuery>) -> impl IntoResponse {
    // If Slack returns an error, show a simple human-readable message.
    if let Some(err) = q.error.as_deref() {
        let html = format!(
            r#"<!doctype html>
<html lang="en">
<head>
  <meta charset="utf-8">
  <meta name="viewport" content="width=device-width, initial-scale=1">
  <title>Salutare – Slack Install</title>
</head>
<body style="font-family: system-ui, -apple-system, Segoe UI, Roboto, Arial, sans-serif; padding: 2rem;">
  <h1>Slack installation failed</h1>
  <p>Error: <code>{}</code></p>
  <p>You can close this window.</p>
</body>
</html>"#,
            err
        );
        return (StatusCode::OK, Html(html));
    }

    // Success path: Slack usually includes `code` and maybe `state`.
    // We do not use them here; we just confirm to the user.
    let html = r#"<!doctype html>
<html lang="en">
<head>
  <meta charset="utf-8">
  <meta name="viewport" content="width=device-width, initial-scale=1">
  <title>Salutare – Slack Install</title>
</head>
<body style="font-family: system-ui, -apple-system, Segoe UI, Roboto, Arial, sans-serif; padding: 2rem;">
  <h1>Salutare installed</h1>
  <p>You can now use <code>/salutare</code> in Slack.</p>
  <p>You can close this window.</p>
</body>
</html>"#;

    (StatusCode::OK, Html(html.to_string()))
}