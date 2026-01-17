/*
 * Copyright © 2026 Daniel Wetzel
 * Licensed under the Apache License, Version 2.0
 * https://github.com/danielkbx/salutare
 */

/// Pure selection logic used by the HTTP handler.
///
/// This function is intentionally deterministic and side-effect free so it can be unit-tested.
///
/// Inputs:
/// - `day_number_utc`: days since Unix epoch (1970-01-01), in UTC
/// - `offset`: caller-provided offset (may be negative)
/// - `n`: number of available greetings
/// - `permutations`: deterministic permutation of indices `0..n-1`
///
/// Output:
/// - a valid index into the greetings vector (`0..n-1`)
///
/// Guarantees:
/// - If `permutations` is a true permutation of `0..n-1`, then for any fixed `offset`,
///   the indices produced over any consecutive `n` day numbers contain no repeats.
pub fn pick_index(day_number_utc: i64, offset: i64, n: usize, permutations: &[usize]) -> usize {
    // Defensive sanity checks. These are programmer errors, not runtime errors.
    debug_assert!(n > 0, "n must be > 0");
    debug_assert!(permutations.len() == n, "permutations length must match n");

    // Apply offset before mapping into the permutation cycle.
    //
    // We must use Euclidean modulo so negative values wrap correctly into [0, n).
    let pos = (day_number_utc + offset).rem_euclid(n as i64) as usize;

    // Map the position into an actual greeting index via the permutation.
    permutations[pos]
}
