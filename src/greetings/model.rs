/// Represents one greeting row loaded from the CSV file.
///
/// The CSV is expected to provide:
/// 1) A numeric, 1-based running index (id)
/// 2) The greeting text in the language of that row
/// 3) The language name in German
/// 4) The language name in English
///
/// We keep this as a plain struct (no serde) because:
/// - We only deserialize from CSV (via the `csv` crate records API)
/// - We want full control over validation and error reporting
#[derive(Debug, Clone)]
pub struct GreetingRow {
    /// Running index (starts at 1 in the CSV).
    ///
    /// We store it as u32 because:
    /// - It is naturally non-negative
    /// - It’s sufficient for realistic CSV sizes
    pub id: u32,

    /// The greeting itself in the language of the row
    /// (e.g. "Bonjour.", "Buenos días.", "おはようございます。").
    pub greeting: String,

    /// Name of the language in German (e.g. "Französisch").
    pub language_de: String,

    /// Name of the language in English (e.g. "French").
    pub language_en: String,
}