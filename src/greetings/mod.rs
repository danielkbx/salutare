/// The `greetings` module is responsible for:
/// - Defining the in-memory data model for greetings
/// - Loading greetings from CSV at application startup
///
/// Keeping this in a dedicated module helps:
/// - Avoid bloating `main.rs`
/// - Keep responsibilities clear
/// - Prepare for future extensions (e.g. different data sources)
pub mod loader;
// Re-export the primary public API of this module.
// This keeps call sites clean: `greetings::load_greetings_csv()` etc.
pub use loader::load_greetings_csv;

pub mod model;
pub use model::GreetingRow;
pub mod permutation;
pub use permutation::build_deterministic_permutation;

pub mod selector;
pub use selector::pick_index;
