/*
 * Copyright © 2026 Daniel Wetzel
 * Licensed under the Apache License, Version 2.0
 * https://github.com/danielkbx/salutare
 */

use crate::greetings::pick_index;
use crate::state::AppState;
use axum::{
    body::Bytes,
    extract::State,
    http::{HeaderMap, StatusCode},
    response::IntoResponse,
    Json,
};
use hmac::{Hmac, Mac};
use serde::Deserialize;
use sha2::Sha256;
use subtle::ConstantTimeEq;

type HmacSha256 = Hmac<Sha256>;

#[derive(Debug, Deserialize)]
struct SlackCommandForm {
    user_id: String,
    user_name: String,
    text: Option<String>,

    // Slack usually provides locale like "de-DE" or "en-US".
    // If missing, we assume non-German.
    locale: Option<String>,
}

fn is_german_locale(locale: Option<&str>) -> bool {
    locale
        .unwrap_or("")
        .to_ascii_lowercase()
        .starts_with("de")
}

#[derive(Debug, serde::Serialize)]
struct SlackMessage {
    response_type: &'static str, // "in_channel"
    text: String,
}

pub async fn command(
    State(state): State<AppState>,
    headers: HeaderMap,
    body: Bytes,
) -> impl IntoResponse {
    let Some(secret) = state.slack_signing_secret.as_ref() else {
        return (
            StatusCode::NOT_IMPLEMENTED,
            Json(serde_json::json!({
            "error": "Slack integration is not configured (missing SLACK_SIGNING_SECRET)"
        })),
        )
            .into_response();
    };

    // Verify Slack signature and timestamp (replay protection).
    if let Err(status) = verify_slack_request(&secret, &headers, &body) {
        return status.into_response();
    }

    // Parse the x-www-form-urlencoded payload.
    let form: SlackCommandForm = match serde_urlencoded::from_bytes(&body) {
        Ok(v) => v,
        Err(_) => return StatusCode::BAD_REQUEST.into_response(),
    };

    // Deterministic per-user offset based on Slack user_id.
    // This ensures different users get different sequences by default.
    let user_offset = stable_user_offset(&secret, &form.user_id);

    // Select today's greeting (UTC day boundary, as in the API).
    let today_utc = chrono::Utc::now().date_naive();
    let epoch = chrono::NaiveDate::from_ymd_opt(1970, 1, 1).unwrap();
    let day_number = today_utc.signed_duration_since(epoch).num_days();

    let n = state.greetings.len();
    let idx = pick_index(day_number, user_offset, n, &state.permutations);
    let row = &state.greetings[idx];

    // Make authorship explicit in the message text.
    let is_de = is_german_locale(form.locale.as_deref());
    let verb = if is_de { "sagt" } else { "says" };
    // Optional: choose language label depending on locale
    let language_name = if is_de { &row.language_de } else { &row.language_en };
    let text = format!(
        "{} {} {} (_{}_)",
        form.user_name,
        verb,
        row.greeting,
        language_name
    );

    let msg = SlackMessage {
        response_type: "in_channel",
        text,
    };

    (StatusCode::OK, Json(msg)).into_response()
}

fn verify_slack_request(
    secret: &str,
    headers: &HeaderMap,
    body: &[u8],
) -> Result<(), StatusCode> {
    let ts = headers
        .get("X-Slack-Request-Timestamp")
        .and_then(|v| v.to_str().ok())
        .ok_or(StatusCode::UNAUTHORIZED)?;

    let sig = headers
        .get("X-Slack-Signature")
        .and_then(|v| v.to_str().ok())
        .ok_or(StatusCode::UNAUTHORIZED)?;

    // Reject replays older than 5 minutes.
    let ts_i64: i64 = ts.parse().map_err(|_| StatusCode::UNAUTHORIZED)?;
    let now = chrono::Utc::now().timestamp();
    if (now - ts_i64).abs() > 60 * 5 {
        return Err(StatusCode::UNAUTHORIZED);
    }

    // Slack signature base string: "v0:{timestamp}:{raw_body}"
    let base = format!("v0:{}:", ts);

    let mut mac = HmacSha256::new_from_slice(secret.as_bytes())
        .map_err(|_| StatusCode::UNAUTHORIZED)?;
    mac.update(base.as_bytes());
    mac.update(body);

    let digest = mac.finalize().into_bytes();
    let expected = format!("v0={}", hex::encode(digest));

    if expected.as_bytes().ct_eq(sig.as_bytes()).unwrap_u8() == 1 {
        Ok(())
    } else {
        Err(StatusCode::UNAUTHORIZED)
    }
}

fn stable_user_offset(secret: &str, user_id: &str) -> i64 {
    // HMAC(secret, user_id) -> take first 8 bytes -> stable u64
    let mut mac = HmacSha256::new_from_slice(secret.as_bytes()).expect("HMAC init");
    mac.update(user_id.as_bytes());
    let digest = mac.finalize().into_bytes();

    let mut b = [0u8; 8];
    b.copy_from_slice(&digest[..8]);
    let v = u64::from_be_bytes(b);

    // Keep it in a reasonable range; selection uses rem_euclid anyway.
    (v % 1_000_000) as i64
}