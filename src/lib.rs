// Library entrypoint for the nextaudit crate.
// Re-export modules so integration tests and external consumers can use
// `nextaudit::config`, `nextaudit::rules`, etc.

pub mod cli;
pub mod config;
pub mod engine;
pub mod reporter;
pub mod docs;
pub mod rules;

// Re-export commonly used types at crate root if useful.
pub use engine::Issue;
