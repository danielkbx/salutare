use serde::Serialize;

/// JSON response returned by GET /api/v1/greeting.
///
/// This API is versioned by the URL prefix `/api/v1`.
/// Any breaking changes should be introduced in `/api/v2` etc.
#[derive(Debug, Serialize)]
pub struct GreetingResponse {
    /// UTC date used to determine "today" (YYYY-MM-DD).
    ///
    /// We define the day boundary strictly at 00:00 UTC to avoid timezone ambiguity
    /// and ensure consistent results worldwide.
    pub date_utc: String,

    /// Running row id from the CSV.
    ///
    /// This makes results reproducible and debuggable:
    /// - If a user reports an issue, they can send `id`
    /// - You can locate the row in the CSV easily
    pub id: u32,

    /// The greeting text itself in the language of the selected row.
    pub greeting: String,

    /// Language name metadata.
    pub language: LanguageInfo,

    /// The offset applied by the caller (defaults to 0).
    pub offset: i64,

    /// The effective day number used after applying the offset.
    /// This is mainly for debugging/reproducibility.
    pub day_number_utc: i64,
}

/// Language name metadata for the selected greeting.
#[derive(Debug, Serialize)]
pub struct LanguageInfo {
    /// Language name in German.
    pub de: String,

    /// Language name in English.
    pub en: String,
}
