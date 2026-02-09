mod clipboard;
mod error;

use clap::Parser;
use clipboard::ClipboardManager;
use rmcp::handler::server::tool::ToolRouter;
use rmcp::handler::server::wrapper::Parameters;
use rmcp::model::*;
use rmcp::tool_handler;
use rmcp::tool_router;
use rmcp::transport::stdio;
use rmcp::ErrorData as McpError;
use rmcp::ServerHandler;
use rmcp::ServiceExt;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use tracing_subscriber::EnvFilter;

/// Cross-platform MCP server for clipboard operations
#[derive(Parser, Debug)]
#[command(name = "klip")]
#[command(author, version, about, long_about = None)]
struct Cli {}

/// Input parameters for the copy_to_clipboard tool
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct CopyToClipboardInput {
    /// The text content to copy to the clipboard
    pub text: String,
}

#[derive(Clone)]
pub struct KlipServer {
    tool_router: ToolRouter<Self>,
}

#[tool_router]
impl KlipServer {
    fn new() -> Self {
        Self {
            tool_router: Self::tool_router(),
        }
    }

    /// Copy text to the system clipboard
    #[rmcp::tool(description = "Copy text to the system clipboard")]
    async fn copy_to_clipboard(
        &self,
        Parameters(params): Parameters<CopyToClipboardInput>,
    ) -> Result<CallToolResult, McpError> {
        let char_count = params.text.chars().count();

        // Create clipboard manager and copy text
        let mut manager = ClipboardManager::new()
            .map_err(|e| McpError::new(ErrorCode(-32000), e.to_string(), None))?;

        manager
            .copy(&params.text)
            .map_err(|e| McpError::new(ErrorCode(-32000), e.to_string(), None))?;

        let message = format!("Successfully copied {} characters to clipboard", char_count);

        Ok(CallToolResult::success(vec![Content::text(message)]))
    }
}

#[tool_handler]
impl ServerHandler for KlipServer {
    fn get_info(&self) -> ServerInfo {
        ServerInfo {
            instructions: Some(
                "A clipboard management server that allows copying text to the system clipboard"
                    .to_string(),
            ),
            capabilities: ServerCapabilities::builder().enable_tools().build(),
            ..Default::default()
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Parse CLI arguments (this handles --help and --version automatically)
    let _cli = Cli::parse();

    // Initialize tracing/logging
    tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info")),
        )
        .init();

    tracing::info!("Starting klip MCP server");

    // Create and run the server
    let service = KlipServer::new()
        .serve(stdio())
        .await
        .inspect_err(|e| eprintln!("Error starting server: {}", e))?;

    tracing::info!("klip MCP server initialized and ready");

    service.waiting().await?;

    Ok(())
}
