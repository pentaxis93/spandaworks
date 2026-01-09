//! Modal state management
//!
//! Holds the in-memory state for the current mode session.
//! State is ephemeral - it exists for the lifetime of the MCP server process.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::RwLock;

/// The three operational modes
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "lowercase")]
pub enum Mode {
    /// Default coding mode - technical collaboration
    #[default]
    Default,
    /// Ops mode - trusted steward for life logistics
    Ops,
    /// Ceremonial mode - full ritual container (/open)
    Ceremonial,
}

impl std::fmt::Display for Mode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Mode::Default => write!(f, "default"),
            Mode::Ops => write!(f, "ops"),
            Mode::Ceremonial => write!(f, "ceremonial"),
        }
    }
}

/// Status of an attention item
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum AttentionStatus {
    /// Currently being worked on
    Hot,
    /// Surfaced but not yet addressed
    Waiting,
    /// Handled this session
    Handled,
}

/// An item in the attention stack
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AttentionItem {
    /// Unique identifier
    pub id: String,
    /// Brief description
    pub description: String,
    /// Current status
    pub status: AttentionStatus,
    /// When this item was surfaced
    pub surfaced_at: DateTime<Utc>,
    /// When status last changed
    pub updated_at: DateTime<Utc>,
    /// Optional context/notes
    pub notes: Option<String>,
}

/// A mode transition record
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModeTransition {
    /// The mode that was entered
    pub mode: Mode,
    /// When the transition occurred
    pub entered_at: DateTime<Utc>,
    /// When the mode was exited (None if current)
    pub exited_at: Option<DateTime<Utc>>,
}

/// The complete modal state
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModalState {
    /// Current operating mode
    pub current_mode: Mode,
    /// When current mode was entered
    pub mode_entered_at: DateTime<Utc>,
    /// Active context description (e.g., "Working on: health insurance")
    pub active_context: Option<String>,
    /// Items requiring attention
    pub attention_stack: Vec<AttentionItem>,
    /// History of mode transitions this session
    pub mode_history: Vec<ModeTransition>,
}

impl Default for ModalState {
    fn default() -> Self {
        let now = Utc::now();
        Self {
            current_mode: Mode::Default,
            mode_entered_at: now,
            active_context: None,
            attention_stack: Vec::new(),
            mode_history: vec![ModeTransition {
                mode: Mode::Default,
                entered_at: now,
                exited_at: None,
            }],
        }
    }
}

impl ModalState {
    /// Create new state starting in a specific mode
    pub fn new(mode: Mode) -> Self {
        let now = Utc::now();
        Self {
            current_mode: mode,
            mode_entered_at: now,
            active_context: None,
            attention_stack: Vec::new(),
            mode_history: vec![ModeTransition {
                mode,
                entered_at: now,
                exited_at: None,
            }],
        }
    }

    /// Transition to a new mode
    pub fn enter_mode(&mut self, mode: Mode) {
        let now = Utc::now();

        // Close out previous mode in history
        if let Some(last) = self.mode_history.last_mut() {
            if last.exited_at.is_none() {
                last.exited_at = Some(now);
            }
        }

        // Update current mode
        self.current_mode = mode;
        self.mode_entered_at = now;

        // Add to history
        self.mode_history.push(ModeTransition {
            mode,
            entered_at: now,
            exited_at: None,
        });
    }

    /// Set the active context
    pub fn set_context(&mut self, context: Option<String>) {
        self.active_context = context;
    }

    /// Add an item to the attention stack
    pub fn add_attention(&mut self, id: String, description: String) -> &AttentionItem {
        let now = Utc::now();
        let item = AttentionItem {
            id: id.clone(),
            description,
            status: AttentionStatus::Waiting,
            surfaced_at: now,
            updated_at: now,
            notes: None,
        };
        self.attention_stack.push(item);
        self.attention_stack.last().unwrap()
    }

    /// Mark an attention item as hot (being worked on)
    pub fn mark_hot(&mut self, id: &str) -> Option<&AttentionItem> {
        if let Some(item) = self.attention_stack.iter_mut().find(|i| i.id == id) {
            item.status = AttentionStatus::Hot;
            item.updated_at = Utc::now();
            Some(item)
        } else {
            None
        }
    }

    /// Mark an attention item as handled
    pub fn mark_handled(&mut self, id: &str) -> Option<&AttentionItem> {
        if let Some(item) = self.attention_stack.iter_mut().find(|i| i.id == id) {
            item.status = AttentionStatus::Handled;
            item.updated_at = Utc::now();
            Some(item)
        } else {
            None
        }
    }

    /// Get items by status
    pub fn get_by_status(&self, status: AttentionStatus) -> Vec<&AttentionItem> {
        self.attention_stack
            .iter()
            .filter(|i| i.status == status)
            .collect()
    }

    /// Get all hot items
    pub fn hot_items(&self) -> Vec<&AttentionItem> {
        self.get_by_status(AttentionStatus::Hot)
    }

    /// Get all waiting items
    pub fn waiting_items(&self) -> Vec<&AttentionItem> {
        self.get_by_status(AttentionStatus::Waiting)
    }

    /// Get all handled items
    pub fn handled_items(&self) -> Vec<&AttentionItem> {
        self.get_by_status(AttentionStatus::Handled)
    }

    /// Get duration in current mode
    pub fn mode_duration(&self) -> chrono::Duration {
        Utc::now() - self.mode_entered_at
    }
}

/// Thread-safe state wrapper
pub type SharedState = Arc<RwLock<ModalState>>;

/// Create a new shared state instance
pub fn new_shared_state() -> SharedState {
    Arc::new(RwLock::new(ModalState::default()))
}

/// Create a new shared state starting in a specific mode
pub fn new_shared_state_with_mode(mode: Mode) -> SharedState {
    Arc::new(RwLock::new(ModalState::new(mode)))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mode_transitions() {
        let mut state = ModalState::default();
        assert_eq!(state.current_mode, Mode::Default);

        state.enter_mode(Mode::Ops);
        assert_eq!(state.current_mode, Mode::Ops);
        assert_eq!(state.mode_history.len(), 2);
        assert!(state.mode_history[0].exited_at.is_some());
        assert!(state.mode_history[1].exited_at.is_none());
    }

    #[test]
    fn test_attention_stack() {
        let mut state = ModalState::default();

        state.add_attention("ins-1".to_string(), "Follow up on insurance".to_string());
        state.add_attention("cal-1".to_string(), "Schedule doctor appointment".to_string());

        assert_eq!(state.attention_stack.len(), 2);
        assert_eq!(state.waiting_items().len(), 2);
        assert_eq!(state.hot_items().len(), 0);

        state.mark_hot("ins-1");
        assert_eq!(state.hot_items().len(), 1);
        assert_eq!(state.waiting_items().len(), 1);

        state.mark_handled("ins-1");
        assert_eq!(state.handled_items().len(), 1);
        assert_eq!(state.hot_items().len(), 0);
    }

    #[test]
    fn test_context() {
        let mut state = ModalState::default();
        assert!(state.active_context.is_none());

        state.set_context(Some("Working on: health insurance".to_string()));
        assert_eq!(
            state.active_context,
            Some("Working on: health insurance".to_string())
        );

        state.set_context(None);
        assert!(state.active_context.is_none());
    }
}
