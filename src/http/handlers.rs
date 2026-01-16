use crate::http::error::ApiError;
use crate::http::response::{GreetingResponse, LanguageInfo};
use crate::state::AppState;
use axum::{Json, extract::Query, extract::State};
use chrono::{NaiveDate, Utc};
use serde::Deserialize;

/// Liveness/readiness style endpoint.
/// Returns "OK – <n> greetings loaded" so monitoring can confirm the CSV was loaded.
pub async fn healthz(State(state): State<AppState>) -> String {
    format!("OK – {} greetings loaded", state.greetings.len())
}

/// Query parameters for GET /api/v1/greeting.
///
/// `offset` allows callers to shift the deterministic daily selection.
/// This is useful to avoid everyone receiving the exact same greeting each day.
#[derive(Debug, Deserialize)]
pub struct GreetingQuery {
    /// Optional day offset applied to the UTC day number before indexing.
    ///
    /// Example:
    /// - offset=0  => default behavior
    /// - offset=1  => "tomorrow" in the permutation
    /// - offset=-1 => "yesterday" in the permutation
    pub offset: Option<i64>,
}

/// Returns the greeting of the day as JSON (`GET /api/v1/greeting`).
///
/// ## Purpose
/// This endpoint serves a single “daily greeting” selected deterministically from the
/// greetings loaded from `greetings.csv` at startup. The selection is designed to:
///
/// - Change only at **00:00 UTC** (global consistency, no server-local timezone effects).
/// - Be **deterministic** (same UTC day + same offset => same response).
/// - Provide **high variability** even if the CSV order is not random.
/// - Guarantee **no row repeats until all rows have been used once** (within a cycle),
///   by selecting via a deterministic permutation of indices.
///
/// ## Day boundary (UTC)
/// The “current day” is computed from the system clock using UTC and converted to a
/// date-only representation (`YYYY-MM-DD`). The greeting changes exactly at UTC midnight.
///
/// ## Deterministic “no-repeat” selection
/// At startup, the service builds a deterministic permutation of all row indices
/// (`0..N-1`) using a seeded shuffle. This permutation is stored in `AppState.permutations`.
///
/// For a given request:
///
/// 1. Compute the integer day number `D` = days since Unix epoch (1970-01-01) in UTC.
/// 2. Apply an optional caller-provided offset `O` (defaults to 0).
/// 3. Compute the permutation position `P = (D + O) mod N` using Euclidean modulo
///    (so negative offsets work as expected).
/// 4. Select the actual row index `I = permutations[P]`.
///
/// Because `permutations` is a true permutation of length `N`, iterating `P` across
/// `0..N-1` yields each row exactly once. Therefore:
///
/// - For any fixed offset, there are **no repeats for N consecutive UTC days**.
/// - After `N` days, the cycle repeats (same permutation order again).
///
/// ## Query parameters
/// - `offset` (optional, integer):
///   Shifts the day number before indexing into the permutation. This allows callers to
///   obtain a different deterministic “stream” of greetings without affecting global
///   behavior.
///
/// Examples:
/// - `/api/v1/greeting` → default stream (offset = 0)
/// - `/api/v1/greeting?offset=17` → shifted stream
/// - `/api/v1/greeting?offset=-3` → shifted stream backwards
///
/// ## Response
/// Returns a JSON payload containing:
/// - The UTC date used for selection (`YYYY-MM-DD`)
/// - The selected CSV row id (useful for debugging / reproducibility)
/// - The greeting text
/// - The language name in German and English
///
/// The response is safe for high concurrency:
/// - `AppState` holds immutable data (`Arc<Vec<_>>`)
/// - No locks are required per request
///
/// ## Error behavior
/// - Invalid query parameters are rejected by Axum’s query extractor (typically `400 Bad Request`)
///   if they cannot be deserialized into `GreetingQuery`.
pub async fn greeting(
    State(state): State<AppState>,
    Query(query): Query<GreetingQuery>,
) -> Result<Json<GreetingResponse>, ApiError> {
    let today_utc = Utc::now().date_naive();
    let epoch = NaiveDate::from_ymd_opt(1970, 1, 1).expect("valid epoch date");

    let days_since_epoch = today_utc.signed_duration_since(epoch).num_days();

    let offset = query.offset.unwrap_or(0);
    // Hard limits to prevent abuse or accidental misuse.
    const OFFSET_MIN: i64 = -100;
    const OFFSET_MAX: i64 = 100;

    if !(OFFSET_MIN..=OFFSET_MAX).contains(&offset) {
        return Err(ApiError::bad_request(format!(
            "offset out of range (allowed: {}..{})",
            OFFSET_MIN, OFFSET_MAX
        )));
    }
    let n = state.greetings.len();
    let idx = crate::greetings::pick_index(days_since_epoch, offset, n, &state.permutations);
    let day_with_offset = days_since_epoch + offset;
    let row = &state.greetings[idx];

    Ok(Json(GreetingResponse {
        date_utc: today_utc.to_string(),
        day_number_utc: day_with_offset,
        offset,
        id: row.id,
        greeting: row.greeting.clone(),
        language: LanguageInfo {
            de: row.language_de.clone(),
            en: row.language_en.clone(),
        },
    }))
}
