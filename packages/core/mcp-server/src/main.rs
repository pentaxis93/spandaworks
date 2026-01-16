//! aiandi-mode: MCP server for modal state management
//!
//! Provides tools for:
//! - Mode management (enter ops, ceremonial, or default mode)
//! - Context tracking (what's being worked on)
//! - Attention stack (hot, waiting, handled items)
//! - Mode history and duration queries

use anyhow::Result;
use chrono::Utc;
use rmcp::{
    handler::server::{router::tool::ToolRouter, wrapper::Parameters},
    model::*,
    schemars, tool, tool_handler, tool_router, ServerHandler, ServiceExt,
};
use serde::Serialize;
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing_subscriber::{fmt, prelude::*, EnvFilter};

mod state;

use state::{AttentionStatus, Mode, ModalState};

// ============================================================================
// Request types for each tool
// ============================================================================

#[derive(serde::Deserialize, schemars::JsonSchema)]
pub struct EnterModeRequest {
    /// The mode to enter: "ops", "ceremonial", or "default"
    mode: String,
}

#[derive(serde::Deserialize, schemars::JsonSchema)]
pub struct SetContextRequest {
    /// The context description (e.g., "Working on: health insurance"). Pass null/empty to clear.
    context: Option<String>,
}

#[derive(serde::Deserialize, schemars::JsonSchema)]
pub struct AddAttentionRequest {
    /// Unique identifier for this attention item
    id: String,
    /// Brief description of what needs attention
    description: String,
}

#[derive(serde::Deserialize, schemars::JsonSchema)]
pub struct UpdateAttentionRequest {
    /// The attention item ID to update
    id: String,
    /// New status: "hot", "waiting", or "handled"
    status: String,
}

#[derive(serde::Deserialize, schemars::JsonSchema)]
pub struct GetAttentionRequest {
    /// Optional status filter: "hot", "waiting", "handled", or "all"
    status: Option<String>,
}

// ============================================================================
// Response types
// ============================================================================

#[derive(Serialize)]
struct ModeStatus {
    current_mode: String,
    mode_entered_at: String,
    duration_seconds: i64,
    active_context: Option<String>,
    attention_counts: AttentionCounts,
}

#[derive(Serialize)]
struct AttentionCounts {
    hot: usize,
    waiting: usize,
    handled: usize,
}

#[derive(Serialize)]
struct AttentionItemResponse {
    id: String,
    description: String,
    status: String,
    surfaced_at: String,
    updated_at: String,
    notes: Option<String>,
}

impl From<&state::AttentionItem> for AttentionItemResponse {
    fn from(item: &state::AttentionItem) -> Self {
        Self {
            id: item.id.clone(),
            description: item.description.clone(),
            status: match item.status {
                AttentionStatus::Hot => "hot".to_string(),
                AttentionStatus::Waiting => "waiting".to_string(),
                AttentionStatus::Handled => "handled".to_string(),
            },
            surfaced_at: item.surfaced_at.to_rfc3339(),
            updated_at: item.updated_at.to_rfc3339(),
            notes: item.notes.clone(),
        }
    }
}

// ============================================================================
// The Mode MCP Server
// ============================================================================

#[derive(Clone)]
pub struct ModeServer {
    tool_router: ToolRouter<Self>,
    state: Arc<RwLock<ModalState>>,
}

#[tool_router]
impl ModeServer {
    fn new() -> Self {
        Self {
            tool_router: Self::tool_router(),
            state: Arc::new(RwLock::new(ModalState::default())),
        }
    }

    // =========================================================================
    // Mode Management
    // =========================================================================

    #[tool(description = "Get current mode status including mode, duration, context, and attention counts.")]
    async fn mode_status(&self) -> String {
        let state = self.state.read().await;
        let duration = state.mode_duration();

        let status = ModeStatus {
            current_mode: state.current_mode.to_string(),
            mode_entered_at: state.mode_entered_at.to_rfc3339(),
            duration_seconds: duration.num_seconds(),
            active_context: state.active_context.clone(),
            attention_counts: AttentionCounts {
                hot: state.hot_items().len(),
                waiting: state.waiting_items().len(),
                handled: state.handled_items().len(),
            },
        };

        serde_json::to_string_pretty(&status).unwrap_or_else(|e| format!("Error: {}", e))
    }

    #[tool(description = "Enter a mode: 'ops' (trusted steward), 'ceremonial' (/open ritual), or 'default' (coding).")]
    async fn enter_mode(&self, Parameters(req): Parameters<EnterModeRequest>) -> String {
        let mode = match req.mode.to_lowercase().as_str() {
            "ops" => Mode::Ops,
            "ceremonial" => Mode::Ceremonial,
            "default" => Mode::Default,
            other => return format!("Unknown mode '{}'. Use: ops, ceremonial, or default", other),
        };

        let mut state = self.state.write().await;
        let previous = state.current_mode;
        state.enter_mode(mode);

        format!(
            "Mode changed: {} -> {}",
            previous,
            state.current_mode
        )
    }

    #[tool(description = "Exit current mode, returning to default. Alias for entering 'default' mode.")]
    async fn exit_mode(&self) -> String {
        let mut state = self.state.write().await;
        let previous = state.current_mode;

        if previous == Mode::Default {
            return "Already in default mode".to_string();
        }

        state.enter_mode(Mode::Default);
        format!("Exited {} mode, returned to default", previous)
    }

    // =========================================================================
    // Context Management
    // =========================================================================

    #[tool(description = "Set the active context (what's currently being worked on). Pass empty/null to clear.")]
    async fn set_context(&self, Parameters(req): Parameters<SetContextRequest>) -> String {
        let mut state = self.state.write().await;
        let context = req.context.filter(|s| !s.trim().is_empty());

        state.set_context(context.clone());

        match context {
            Some(ctx) => format!("Context set: {}", ctx),
            None => "Context cleared".to_string(),
        }
    }

    #[tool(description = "Get the current active context, if any.")]
    async fn get_context(&self) -> String {
        let state = self.state.read().await;
        match &state.active_context {
            Some(ctx) => ctx.clone(),
            None => "No active context".to_string(),
        }
    }

    // =========================================================================
    // Attention Stack Management
    // =========================================================================

    #[tool(description = "Add an item to the attention stack. Items start in 'waiting' status.")]
    async fn add_attention(&self, Parameters(req): Parameters<AddAttentionRequest>) -> String {
        let mut state = self.state.write().await;

        // Check for duplicate
        if state.attention_stack.iter().any(|i| i.id == req.id) {
            return format!("Attention item '{}' already exists", req.id);
        }

        state.add_attention(req.id.clone(), req.description.clone());
        format!("Added attention item: {} - {}", req.id, req.description)
    }

    #[tool(description = "Update an attention item's status: 'hot' (being worked), 'waiting', or 'handled'.")]
    async fn update_attention(&self, Parameters(req): Parameters<UpdateAttentionRequest>) -> String {
        let status = match req.status.to_lowercase().as_str() {
            "hot" => AttentionStatus::Hot,
            "waiting" => AttentionStatus::Waiting,
            "handled" => AttentionStatus::Handled,
            other => return format!("Unknown status '{}'. Use: hot, waiting, or handled", other),
        };

        let mut state = self.state.write().await;

        let result = match status {
            AttentionStatus::Hot => state.mark_hot(&req.id),
            AttentionStatus::Waiting => {
                if let Some(item) = state.attention_stack.iter_mut().find(|i| i.id == req.id) {
                    item.status = AttentionStatus::Waiting;
                    item.updated_at = Utc::now();
                    Some(&*item)
                } else {
                    None
                }
            }
            AttentionStatus::Handled => state.mark_handled(&req.id),
        };

        match result {
            Some(item) => format!("Updated '{}' to {}", item.id, req.status),
            None => format!("Attention item '{}' not found", req.id),
        }
    }

    #[tool(description = "List attention items. Optionally filter by status: 'hot', 'waiting', 'handled', or 'all'.")]
    async fn list_attention(&self, Parameters(req): Parameters<GetAttentionRequest>) -> String {
        let state = self.state.read().await;

        let items: Vec<AttentionItemResponse> = match req.status.as_deref() {
            Some("hot") => state.hot_items().into_iter().map(Into::into).collect(),
            Some("waiting") => state.waiting_items().into_iter().map(Into::into).collect(),
            Some("handled") => state.handled_items().into_iter().map(Into::into).collect(),
            Some("all") | None => state.attention_stack.iter().map(Into::into).collect(),
            Some(other) => {
                return format!(
                    "Unknown status '{}'. Use: hot, waiting, handled, or all",
                    other
                )
            }
        };

        if items.is_empty() {
            return "No attention items".to_string();
        }

        serde_json::to_string_pretty(&items).unwrap_or_else(|e| format!("Error: {}", e))
    }

    // =========================================================================
    // Mode History
    // =========================================================================

    #[tool(description = "Get the history of mode transitions this session.")]
    async fn mode_history(&self) -> String {
        let state = self.state.read().await;

        let history: Vec<serde_json::Value> = state
            .mode_history
            .iter()
            .map(|t| {
                serde_json::json!({
                    "mode": t.mode.to_string(),
                    "entered_at": t.entered_at.to_rfc3339(),
                    "exited_at": t.exited_at.map(|dt| dt.to_rfc3339()),
                })
            })
            .collect();

        serde_json::to_string_pretty(&history).unwrap_or_else(|e| format!("Error: {}", e))
    }
}

#[tool_handler]
impl ServerHandler for ModeServer {
    fn get_info(&self) -> ServerInfo {
        ServerInfo {
            capabilities: ServerCapabilities::builder().enable_tools().build(),
            ..Default::default()
        }
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize logging to stderr (NEVER stdout - would corrupt JSON-RPC)
    tracing_subscriber::registry()
        .with(fmt::layer().with_writer(std::io::stderr))
        .with(EnvFilter::from_default_env().add_directive("aiandi_mode=info".parse()?))
        .init();

    tracing::info!("Starting aiandi-mode MCP server");

    // Create server and run with stdio transport
    let transport = (tokio::io::stdin(), tokio::io::stdout());
    let service = ModeServer::new().serve(transport).await?;

    tracing::info!("Server initialized, waiting for requests");
    service.waiting().await?;

    Ok(())
}
