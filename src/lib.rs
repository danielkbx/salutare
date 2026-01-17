/*
 * Copyright © 2026 Daniel Wetzel
 * Licensed under the Apache License, Version 2.0
 * https://github.com/danielkbx/salutare
 */

//! Salutare library crate.
//!
//! This crate exposes the reusable core modules so they can be:
//! - unit tested and integration tested
//! - reused by the binary (`src/main.rs`)
//!
//! The executable entrypoint remains in `src/main.rs`.

pub mod config;
pub mod greetings;
pub mod http;
pub mod state;
