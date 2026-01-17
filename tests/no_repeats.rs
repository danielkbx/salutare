/*
 * Copyright © 2026 Daniel Wetzel
 * Licensed under the Apache License, Version 2.0
 * https://github.com/danielkbx/salutare
 */

use salutare::greetings::build_deterministic_permutation;
use std::collections::HashSet;
#[test]
fn no_repeats_within_one_full_cycle() {
    let n = 365;
    let salt = "cycle-invariant";

    let perm = build_deterministic_permutation(n, salt).unwrap();

    let start_day: i64 = 42_000;
    let mut seen = HashSet::with_capacity(n);

    for offset in 0..n as i64 {
        let day = start_day + offset;
        let pos = (day.rem_euclid(n as i64)) as usize;
        let idx = perm[pos];

        seen.insert(idx);
    }

    assert_eq!(
        seen.len(),
        n,
        "expected no repeats before all rows were used"
    );
}
