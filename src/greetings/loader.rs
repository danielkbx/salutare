/*
 * Copyright © 2026 Daniel Wetzel
 * Licensed under the Apache License, Version 2.0
 * https://github.com/danielkbx/salutare
 */

use crate::greetings::model::GreetingRow;
use anyhow::{Context, Result};
use std::path::Path;

/// Loads greeting rows from a CSV file into memory.
///
/// Expectations about the CSV format:
/// - The CSV has a header row (has_headers = true)
/// - Exactly (at least) 4 columns in the following order:
///   0: id (u32, running index starting at 1)
///   1: greeting (string, greeting in the row’s language)
///   2: language_de (string, language name in German)
///   3: language_en (string, language name in English)
///
/// This function performs:
/// - Parsing (id must be a number)
/// - Basic validation (no empty required fields)
/// - Post-validation (no duplicate IDs)
/// - Stabilization (sort by id)
///
/// Returns:
/// - Vec<GreetingRow> sorted by `id`
///
/// Fails fast with actionable error messages:
/// - If the file cannot be opened
/// - If a required column is missing
/// - If any required field is empty
/// - If parsing fails for the id
/// - If no rows are present
/// - If duplicate IDs exist
pub fn load_greetings_csv(path: impl AsRef<Path>) -> Result<Vec<GreetingRow>> {
    // Convert the provided path-like input into a concrete Path reference.
    // This keeps the API ergonomic (accepts &str, String, PathBuf, etc.).
    let path = path.as_ref();

    // Create a CSV reader.
    //
    // has_headers(true) means:
    // - The first row is treated as a header and is not returned as a record
    // - Line number calculations must account for the header line
    let mut rdr = csv::ReaderBuilder::new()
        .delimiter(b',')
        .has_headers(true)
        .from_path(path)
        .with_context(|| format!("Unable to open CSV file: {:?}", path))?;

    let mut out: Vec<GreetingRow> = Vec::new();

    // Iterate over all CSV records.
    //
    // `records()` yields Result<StringRecord>.
    // We also track an index to generate accurate error messages with line numbers.
    //
    // Important: because we have a header row, the first data row is line 2.
    for (record_idx, record) in rdr.records().enumerate() {
        let record = record?; // Convert CSV parse errors into early returns.

        // Extract expected columns by index. If a column is missing, fail with context.
        //
        // We call trim() to:
        // - remove accidental spaces
        // - keep validation strict (no "   " values)
        let id_str = record.get(0).context("Missing column 0: id")?.trim();
        let greeting = record
            .get(1)
            .context("Missing column 1: greeting")?
            .trim();
        let language_de = record
            .get(2)
            .context("Missing column 2: language_de")?
            .trim();
        let language_en = record
            .get(3)
            .context("Missing column 3: language_en")?
            .trim();

        // Compute a human-friendly CSV line number for error messages.
        // record_idx is 0-based and points to the first *data* row.
        // +2 accounts for: (1) 1-based line numbers and (2) the header row.
        let csv_line = record_idx + 2;

        // Parse the id (running index).
        // We add detailed context including the original string and line number.
        let id: u32 = id_str
            .parse()
            .with_context(|| format!("Invalid id '{}' at CSV line {}", id_str, csv_line))?;

        // Validate required fields.
        //
        // We deliberately treat empty strings as invalid because downstream logic
        // (API responses) should not have missing/empty names or greetings.
        if greeting.is_empty() || language_de.is_empty() || language_en.is_empty() {
            anyhow::bail!(
                "Empty required field at CSV line {} (id={})",
                csv_line,
                id
            );
        }

        // Store the parsed row as owned Strings (we do not keep references to the CSV record).
        out.push(GreetingRow {
            id,
            greeting: greeting.to_string(),
            language_de: language_de.to_string(),
            language_en: language_en.to_string(),
        });
    }

    // Validate that the file produced at least one data row.
    if out.is_empty() {
        anyhow::bail!("CSV contains no data rows: {:?}", path);
    }

    // Sort the rows by ID to ensure deterministic ordering in memory.
    //
    // This is useful because:
    // - It makes testing easier
    // - It makes future "pick by index" logic stable
    // - It allows quick duplicate detection via windows()
    out.sort_by_key(|r| r.id);

    // Detect duplicate IDs after sorting.
    //
    // windows(2) iterates over overlapping pairs:
    // [row0,row1], [row1,row2], ...
    // After sorting, duplicates will be adjacent.
    for pair in out.windows(2) {
        if pair[0].id == pair[1].id {
            anyhow::bail!("Duplicate id found in CSV: {}", pair[0].id);
        }
    }

    Ok(out)
}