//! CLI command execution utilities
//!
//! Re-exports shared spanda-cli utilities.
//! 
//! This module now delegates to the shared `spanda-cli` crate,
//! which provides consistent CLI wrapping across all Spandaworks Rust packages.

// Re-export utilities used by pim package
pub use spanda_cli::{run_command_stdout, run_command_with_stdin};
