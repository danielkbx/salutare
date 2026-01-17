/*
 * Copyright © 2026 Daniel Wetzel
 * Licensed under the Apache License, Version 2.0
 * https://github.com/danielkbx/salutare
 */

use salutare::greetings::{build_deterministic_permutation, pick_index};
#[test]
fn offset_zero_matches_base_behavior() {
    let n = 50;
    let salt = "offset-test-salt";
    let permutations = build_deterministic_permutation(n, salt).unwrap();

    // Arbitrary but fixed day number.
    let day = 10_000i64;

    let base = pick_index(day, 0, n, &permutations);
    let offset_zero = pick_index(day, 0, n, &permutations);

    assert_eq!(base, offset_zero);
}

#[test]
fn offset_changes_selection_in_expected_way() {
    let n = 50;
    let salt = "offset-test-salt";
    let permutations = build_deterministic_permutation(n, salt).unwrap();

    let day = 10_000i64;

    // With our definition, offset=+1 should shift one position forward in the permutation.
    let idx0 = pick_index(day, 0, n, &permutations);
    let idx1 = pick_index(day, 1, n, &permutations);

    // This should usually differ; for a true permutation and n>1 it MUST differ,
    // because pos and pos+1 are different positions and map to different indices.
    assert_ne!(idx0, idx1);

    // Check exact expected mapping.
    let pos0 = (day + 0).rem_euclid(n as i64) as usize;
    let pos1 = (day + 1).rem_euclid(n as i64) as usize;

    assert_eq!(idx0, permutations[pos0]);
    assert_eq!(idx1, permutations[pos1]);
}

#[test]
fn negative_offset_wraps_correctly() {
    let n = 50;
    let salt = "offset-test-salt";
    let permutations = build_deterministic_permutation(n, salt).unwrap();

    let day = 10_000i64;

    let idx_minus_1 = pick_index(day, -1, n, &permutations);
    let pos_minus_1 = (day - 1).rem_euclid(n as i64) as usize;

    assert_eq!(idx_minus_1, permutations[pos_minus_1]);
}
