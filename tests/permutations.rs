/*
 * Copyright © 2026 Daniel Wetzel
 * Licensed under the Apache License, Version 2.0
 * https://github.com/danielkbx/salutare
 */

use salutare::greetings::build_deterministic_permutation;
use std::collections::HashSet;
#[test]
fn permutation_is_a_true_permutation() {
    let n = 128;
    let salt = "integration-test-salt";

    let perm = build_deterministic_permutation(n, salt).unwrap();
    assert_eq!(perm.len(), n);

    let set: HashSet<usize> = perm.iter().copied().collect();
    assert_eq!(set.len(), n);

    for i in 0..n {
        assert!(set.contains(&i), "missing index {}", i);
    }
}

#[test]
fn permutation_is_deterministic() {
    let n = 256;
    let salt = "stable-seed";

    let p1 = build_deterministic_permutation(n, salt).unwrap();
    let p2 = build_deterministic_permutation(n, salt).unwrap();

    assert_eq!(p1, p2);
}

#[test]
fn permutation_changes_with_salt() {
    let n = 256;

    let p1 = build_deterministic_permutation(n, "salt-a").unwrap();
    let p2 = build_deterministic_permutation(n, "salt-b").unwrap();

    assert_ne!(p1, p2);
}
