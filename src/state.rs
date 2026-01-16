use crate::greetings::GreetingRow;
use std::sync::Arc;

/// Application state shared across request handlers.
///
/// We keep greetings in memory and share them using `Arc` so that:
/// - We avoid copying the entire vector per request
/// - Cloning state is cheap
///
/// The data is immutable after startup, so we do not need locks.
#[derive(Debug, Clone)]
pub struct AppState {
    pub greetings: Arc<Vec<GreetingRow>>,
    /// Deterministic shuffled index order over `greetings`.
    pub permutations: Arc<Vec<usize>>,
}