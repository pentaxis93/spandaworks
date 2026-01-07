//! talos-pim: MCP server for Personal Information Management
//!
//! Provides tools for email, calendar, and contacts access via CLI wrappers:
//! - Email: notmuch (search/read), himalaya (send)
//! - Calendar: khal (list/create/update/delete)
//! - Contacts: khard (search/get/create/update/delete)

use anyhow::Result;
use rmcp::{
    ServerHandler, ServiceExt,
    handler::server::{router::tool::ToolRouter, wrapper::Parameters},
    model::*,
    schemars, tool, tool_handler, tool_router,
};
use tracing_subscriber::{EnvFilter, fmt, prelude::*};

mod calendar;
mod cli;
mod contacts;
mod email;

use calendar::CalendarTools;
use contacts::ContactTools;
use email::EmailTools;

// ============================================================================
// Request types for each tool (with JSON Schema generation)
// ============================================================================

#[derive(serde::Deserialize, schemars::JsonSchema)]
pub struct ListEventsRequest {
    /// Start date in YYYY-MM-DD format (default: today)
    start_date: Option<String>,
    /// Number of days to show (default: 7)
    days: Option<u32>,
}

#[derive(serde::Deserialize, schemars::JsonSchema)]
pub struct CreateEventRequest {
    /// Event title
    title: String,
    /// Date in YYYY-MM-DD format
    date: String,
    /// Start time in HH:MM format (24-hour). Omit for all-day event.
    start_time: Option<String>,
    /// End time in HH:MM format (24-hour). Omit for 1-hour default.
    end_time: Option<String>,
    /// Event location
    location: Option<String>,
    /// Event description
    description: Option<String>,
}

#[derive(serde::Deserialize, schemars::JsonSchema)]
pub struct SearchEmailsRequest {
    /// Search query using notmuch syntax (e.g., 'from:john subject:meeting date:thisweek')
    query: String,
    /// Maximum number of results (default: 20)
    limit: Option<u32>,
}

#[derive(serde::Deserialize, schemars::JsonSchema)]
pub struct ReadEmailRequest {
    /// Thread ID from search results (e.g., 'thread:00000000000012ab')
    thread_id: String,
}

#[derive(serde::Deserialize, schemars::JsonSchema)]
pub struct SendEmailRequest {
    /// Recipient email address
    to: String,
    /// Email subject
    subject: String,
    /// Email body text
    body: String,
    /// Must be true to actually send. Safety gate.
    confirm: bool,
}

#[derive(serde::Deserialize, schemars::JsonSchema)]
pub struct FindContactRequest {
    /// Search query (name, email, or phone fragment)
    query: String,
}

#[derive(serde::Deserialize, schemars::JsonSchema)]
pub struct GetContactRequest {
    /// Contact name (as shown in search results)
    name: String,
}

#[derive(serde::Deserialize, schemars::JsonSchema)]
pub struct CreateContactRequest {
    /// Full name
    name: String,
    /// Email address
    email: Option<String>,
    /// Phone number
    phone: Option<String>,
    /// Organization/company
    organization: Option<String>,
}

// ============================================================================
// The PIM MCP Server
// ============================================================================

#[derive(Clone)]
pub struct PimServer {
    tool_router: ToolRouter<Self>,
    calendar: CalendarTools,
    email: EmailTools,
    contacts: ContactTools,
}

#[tool_router]
impl PimServer {
    fn new() -> Self {
        Self {
            tool_router: Self::tool_router(),
            calendar: CalendarTools::new(),
            email: EmailTools::new(),
            contacts: ContactTools::new(),
        }
    }

    // =========================================================================
    // Calendar Tools
    // =========================================================================

    #[tool(
        description = "List calendar events for a date range. Returns events from all calendars (personal, Meli, Dennis, holidays)."
    )]
    async fn list_events(&self, Parameters(req): Parameters<ListEventsRequest>) -> String {
        self.calendar.list_events(req.start_date, req.days).await
    }

    #[tool(description = "Create a new calendar event on your personal calendar.")]
    async fn create_event(&self, Parameters(req): Parameters<CreateEventRequest>) -> String {
        self.calendar
            .create_event(
                req.title,
                req.date,
                req.start_time,
                req.end_time,
                req.location,
                req.description,
            )
            .await
    }

    // =========================================================================
    // Email Tools
    // =========================================================================

    #[tool(
        description = "Search emails using notmuch query syntax. Supports: from:, to:, subject:, date:, tag:, and free text."
    )]
    async fn search_emails(&self, Parameters(req): Parameters<SearchEmailsRequest>) -> String {
        self.email.search(req.query, req.limit).await
    }

    #[tool(description = "Read the full content of an email thread.")]
    async fn read_email(&self, Parameters(req): Parameters<ReadEmailRequest>) -> String {
        self.email.read_thread(req.thread_id).await
    }

    #[tool(description = "Send an email. Requires explicit confirmation for safety.")]
    async fn send_email(&self, Parameters(req): Parameters<SendEmailRequest>) -> String {
        self.email
            .send(req.to, req.subject, req.body, req.confirm)
            .await
    }

    // =========================================================================
    // Contact Tools
    // =========================================================================

    #[tool(description = "Search contacts by name, email, or phone number.")]
    async fn find_contact(&self, Parameters(req): Parameters<FindContactRequest>) -> String {
        self.contacts.search(req.query).await
    }

    #[tool(description = "Get full details of a specific contact.")]
    async fn get_contact(&self, Parameters(req): Parameters<GetContactRequest>) -> String {
        self.contacts.get(req.name).await
    }

    #[tool(description = "Create a new contact.")]
    async fn create_contact(&self, Parameters(req): Parameters<CreateContactRequest>) -> String {
        self.contacts
            .create(req.name, req.email, req.phone, req.organization)
            .await
    }
}

#[tool_handler]
impl ServerHandler for PimServer {
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
        .with(EnvFilter::from_default_env().add_directive("talos_pim=info".parse()?))
        .init();

    tracing::info!("Starting talos-pim MCP server");

    // Create server and run with stdio transport
    let transport = (tokio::io::stdin(), tokio::io::stdout());
    let service = PimServer::new().serve(transport).await?;

    tracing::info!("Server initialized, waiting for requests");
    service.waiting().await?;

    Ok(())
}
