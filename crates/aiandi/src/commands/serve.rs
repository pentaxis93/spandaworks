// `aiandi serve` command implementation.
//!
//! MCP server for OpenCode integration.
//! Exposes aiandi capabilities as MCP tools.

use anyhow::Result;
use rmcp::{
    handler::server::{router::tool::ToolRouter, wrapper::Parameters},
    model::*,
    schemars, tool, tool_handler, tool_router, ServerHandler, ServiceExt,
};
use tracing_subscriber::{fmt, prelude::*, EnvFilter};

use crate::commands::{doctor, inbox, init};
use inbox::InboxOptions;
use init::InitOptions;

// ============================================================================
// Request types for each tool (with JSON Schema generation)
// ============================================================================

#[derive(serde::Deserialize, schemars::JsonSchema)]
pub struct InitRequest {
    /// Skills to install (comma-separated). Defaults to all available skills.
    skills: Option<String>,
    /// Force overwrite existing files
    force: Option<bool>,
}

#[derive(serde::Deserialize, schemars::JsonSchema)]
pub struct InboxRequest {
    /// Text to capture to inbox
    text: String,
    /// Additional tags (comma-separated)
    tags: Option<String>,
    /// Project to assign
    project: Option<String>,
}

#[derive(serde::Deserialize, schemars::JsonSchema)]
pub struct DoctorRequest {
    // Doctor takes no parameters currently
}

// ============================================================================
// The aiandi MCP Server
// ============================================================================

#[derive(Clone)]
pub struct AiandiServer {
    tool_router: ToolRouter<Self>,
}

#[tool_router]
impl AiandiServer {
    fn new() -> Self {
        Self {
            tool_router: Self::tool_router(),
        }
    }

    // =========================================================================
    // aiandi Tools
    // =========================================================================

    #[tool(
        description = "Initialize aiandi by extracting bundled skills to .opencode/skill/ directory. Creates directory structure and copies skill files."
    )]
    async fn aiandi_init(&self, Parameters(req): Parameters<InitRequest>) -> String {
        // Parse skills parameter
        let skills_opt = req.skills.as_ref().map(|s| {
            s.split(',')
                .map(|s| s.trim().to_string())
                .collect::<Vec<_>>()
        });

        // Build options
        let options = InitOptions {
            skills: skills_opt.clone(),
            no_skills: false,
            agents: None, // Install all agents
            no_agents: false,
            force: req.force.unwrap_or(false),
            dry_run: false,
        };

        // Run init
        match init::run_with_options(&options) {
            Ok(_result) => {
                "✓ aiandi initialized successfully. Skills extracted to .opencode/skill/"
                    .to_string()
            }
            Err(e) => format!("✗ Initialization failed: {}", e),
        }
    }

    #[tool(
        description = "Capture text to GTD inbox via TaskWarrior. Adds task with +in tag for later processing."
    )]
    async fn aiandi_inbox(&self, Parameters(req): Parameters<InboxRequest>) -> String {
        // Parse tags
        let tags_vec = req
            .tags
            .as_ref()
            .map(|t| {
                t.split(',')
                    .map(|s| s.trim().to_string())
                    .collect::<Vec<_>>()
            })
            .unwrap_or_default();

        // Build options
        let options = InboxOptions {
            text: req.text.clone(),
            tags: tags_vec,
            project: req.project.clone(),
            dry_run: false,
        };

        // Run inbox capture
        match inbox::run_with_options(&options) {
            Ok(result) => {
                if let Some(task_id) = result.task_id {
                    format!("✓ Captured to inbox: task {}", task_id)
                } else {
                    "✓ Captured to inbox (dry-run mode)".to_string()
                }
            }
            Err(e) => format!("✗ Capture failed: {}", e),
        }
    }

    #[tool(
        description = "Run system health checks. Verifies OpenCode installation, TaskWarrior, config directories, and skills."
    )]
    async fn aiandi_doctor(&self, _params: Parameters<DoctorRequest>) -> String {
        // Run doctor checks - it returns (), so we just run it
        doctor::run();
        "✓ Health checks complete. See output for details.".to_string()
    }
}

#[tool_handler]
impl ServerHandler for AiandiServer {
    fn get_info(&self) -> ServerInfo {
        ServerInfo {
            server_info: Implementation {
                name: "aiandi".to_string(),
                version: env!("CARGO_PKG_VERSION").to_string(),
                icons: None,
                title: None,
                website_url: None,
            },
            capabilities: ServerCapabilities::builder().enable_tools().build(),
            ..Default::default()
        }
    }
}

// ============================================================================
// Main server startup
// ============================================================================

/// Start the MCP server on stdin/stdout
pub async fn start_server() -> Result<()> {
    // Initialize logging to stderr (NEVER stdout - would corrupt JSON-RPC)
    tracing_subscriber::registry()
        .with(fmt::layer().with_writer(std::io::stderr))
        .with(EnvFilter::from_default_env().add_directive("aiandi=info".parse()?))
        .init();

    tracing::info!("Starting aiandi MCP server");

    // Create server and run with stdio transport
    let transport = (tokio::io::stdin(), tokio::io::stdout());
    let service = AiandiServer::new().serve(transport).await?;

    tracing::info!("Server initialized, waiting for requests");
    service.waiting().await?;

    Ok(())
}

// ============================================================================
// CLI compatibility layer
// ============================================================================

/// Legacy Args structure (kept for backwards compatibility with existing tests)
#[derive(Debug, Clone, clap::Args)]
pub struct Args {
    // MCP servers use stdio, so port/host are not used
    // Kept for backwards compatibility
    #[arg(long, short = 'p', default_value_t = 3000, hide = true)]
    pub port: u16,

    #[arg(long, short = 'H', default_value = "127.0.0.1", hide = true)]
    pub host: String,
}

impl Default for Args {
    fn default() -> Self {
        Self {
            port: 3000,
            host: "127.0.0.1".to_string(),
        }
    }
}

/// Run the serve command from CLI arguments
pub fn run_from_args(_args: &Args) -> Result<()> {
    // MCP servers must run in async context
    tokio::runtime::Runtime::new()?.block_on(start_server())
}

#[cfg(test)]
mod tests {
    use super::*;

    // ===================
    // TDD: Tests for serve command
    // ===================

    #[test]
    fn test_default_port() {
        let args = Args::default();
        assert_eq!(args.port, 3000);
    }

    #[test]
    fn test_default_host() {
        let args = Args::default();
        assert_eq!(args.host, "127.0.0.1");
    }

    #[test]
    fn test_server_creation() {
        let server = AiandiServer::new();
        // Should not panic
        drop(server);
    }

    #[test]
    fn test_server_info() {
        let server = AiandiServer::new();
        let info = server.get_info();

        assert_eq!(info.server_info.name, "aiandi");
        assert_eq!(info.server_info.version, env!("CARGO_PKG_VERSION"));
        assert!(info.capabilities.tools.is_some());
    }
}
