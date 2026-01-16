//! CLI command execution utilities
//!
//! Re-exports shared cli utilities.
//! 
//! This module now delegates to the shared `cli` crate,
//! which provides consistent CLI wrapping across all aiandi Rust packages.

// Re-export utilities used by pim package
pub use aiandi_cli::{run_command_stdout, run_command_with_stdin};
