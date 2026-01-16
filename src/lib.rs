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
