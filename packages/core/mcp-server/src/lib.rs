//! spandaworks-mode: MCP server for modal state management
//!
//! Tracks:
//! - Current mode (ops, ceremonial, default)
//! - Mode entry timestamp
//! - Active context (what's being worked on)
//! - Attention stack (hot, waiting, handled items)
//!
//! This server provides working memory for modal operation,
//! enabling context persistence across conversation turns.

pub mod state;
