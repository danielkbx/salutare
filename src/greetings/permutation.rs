/*
 * Copyright © 2026 Daniel Wetzel
 * Licensed under the Apache License, Version 2.0
 * https://github.com/danielkbx/salutare
 */

use anyhow::Result;
use rand::seq::SliceRandom;
use rand_chacha::ChaCha20Rng;
use rand_chacha::rand_core::SeedableRng;
use sha2::{Digest, Sha256};

/// Builds a deterministic permutation of indices `0..n-1`.
///
/// Why this exists:
/// - We must guarantee: "no repeats until all rows have been used once".
/// - Mapping `hash(date) % n` does NOT guarantee that (collisions happen).
/// - A permutation does guarantee it by construction.
///
/// How determinism is achieved:
/// - We derive a 32-byte seed from SHA-256(salt + n).
/// - We use ChaCha20Rng seeded with those 32 bytes.
/// - We shuffle using Fisher-Yates (via `SliceRandom::shuffle`).
///
/// Stability note:
/// - Given the same `salt` and the same `n`, the permutation is identical across runs.
/// - If `n` changes (CSV rows added/removed), the permutation changes (expected).
pub fn build_deterministic_permutation(n: usize, salt: &str) -> Result<Vec<usize>> {
    if n == 0 {
        anyhow::bail!("Cannot build permutation for n=0");
    }

    // Construct seed material that ties the permutation to both:
    // - your salt (so the order isn't trivially predictable from outside)
    // - the dataset size n (so the permutation is valid for current greetings length)
    //
    // Including `n` avoids edge cases where a seed created for a different size
    // could accidentally be reused in the future.
    let seed_material = format!("salutare:{}:{}", salt, n);

    // Derive a stable 32-byte seed from SHA-256.
    let digest = Sha256::digest(seed_material.as_bytes());
    let mut seed = [0u8; 32];
    seed.copy_from_slice(&digest[..32]);

    // Create a deterministic RNG.
    // ChaCha20 is a high-quality PRNG; seeded deterministically, it becomes reproducible.
    let mut rng = ChaCha20Rng::from_seed(seed);

    // Start with identity [0..n-1].
    let mut perm: Vec<usize> = (0..n).collect();

    // Shuffle in-place (Fisher-Yates under the hood).
    perm.shuffle(&mut rng);

    Ok(perm)
}