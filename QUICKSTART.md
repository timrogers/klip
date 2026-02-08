# Quick Start Guide: Implementing Klip

This guide provides step-by-step instructions for implementing the Klip MCP clipboard server in Rust.

## Prerequisites

### System Requirements
- Rust 1.70+ (stable)
- Cargo
- Git

### Platform-Specific Requirements

**Linux**:
```bash
# Debian/Ubuntu
sudo apt-get install libxcb1-dev libxcb-render0-dev libxcb-shape0-dev libxcb-xfixes0-dev

# Fedora
sudo dnf install libxcb-devel

# Arch
sudo pacman -S libxcb
```

**macOS**:
```bash
# Xcode Command Line Tools (if not already installed)
xcode-select --install
```

**Windows**:
- Visual Studio 2019 or later with C++ build tools
- Or: Windows SDK with MSVC compiler

## Step-by-Step Implementation

### Step 1: Initialize Project

```bash
cd /path/to/klip
cargo init --name klip
```

### Step 2: Configure Cargo.toml

Create `Cargo.toml` with the following content:

```toml
[package]
name = "klip"
version = "0.1.0"
edition = "2021"
authors = ["Tim Rogers <me@timrogers.co.uk>"]
description = "Cross-platform MCP server for clipboard operations"
license = "MIT"
repository = "https://github.com/timrogers/klip"
readme = "README.md"
keywords = ["mcp", "clipboard", "cross-platform"]
categories = ["command-line-utilities"]

[dependencies]
rmcp = "0.2"
arboard = "3.4"
tokio = { version = "1", features = ["full"] }
serde = { version = "1", features = ["derive"] }
serde_json = "1"
thiserror = "2"
tracing = "0.1"
tracing-subscriber = { version = "0.3", features = ["env-filter"] }

[dev-dependencies]
tokio-test = "0.4"

[features]
default = ["x11"]
x11 = []
wayland = ["arboard/wayland-data-control"]

[profile.release]
opt-level = 3
lto = true
codegen-units = 1
strip = true
```

### Step 3: Create Error Types

Create `src/error.rs`:

```rust
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ClipboardError {
    #[error("Clipboard is not available or accessible")]
    Unavailable,
    
    #[error("Permission to access clipboard was denied")]
    PermissionDenied,
    
    #[error("Clipboard operation timed out")]
    OperationTimeout,
    
    #[error("Platform-specific error: {0}")]
    PlatformError(String),
}

#[derive(Error, Debug)]
pub enum ValidationError {
    #[error("Text exceeds maximum size of {max} bytes (got {size} bytes)")]
    TextTooLarge { size: usize, max: usize },
    
    #[error("Text contains invalid UTF-8")]
    InvalidUtf8,
    
    #[error("Text cannot be empty")]
    EmptyText,
}

#[derive(Error, Debug)]
pub enum KlipError {
    #[error("Clipboard error: {0}")]
    Clipboard(#[from] ClipboardError),
    
    #[error("Validation error: {0}")]
    Validation(#[from] ValidationError),
    
    #[error("Arboard error: {0}")]
    Arboard(#[from] arboard::Error),
}

// Convert KlipError to MCP ErrorData
impl From<KlipError> for rmcp::ErrorData {
    fn from(err: KlipError) -> Self {
        rmcp::ErrorData {
            code: -32000, // Server error
            message: err.to_string(),
            data: None,
        }
    }
}
```

### Step 4: Create Clipboard Module

Create `src/clipboard.rs`:

```rust
use crate::error::{ClipboardError, ValidationError};
use arboard::Clipboard;
use std::time::Duration;
use tracing::{debug, info, warn};

const MAX_SIZE: usize = 10 * 1024 * 1024; // 10MB
const OPERATION_TIMEOUT: Duration = Duration::from_secs(5);

pub struct ClipboardManager {
    clipboard: Clipboard,
}

impl ClipboardManager {
    pub fn new() -> Result<Self, ClipboardError> {
        let clipboard = Clipboard::new()
            .map_err(|e| {
                warn!("Failed to initialize clipboard: {}", e);
                ClipboardError::Unavailable
            })?;
        
        info!("Clipboard manager initialized");
        Ok(Self { clipboard })
    }
    
    pub fn set_text(&mut self, text: &str) -> Result<(), crate::error::KlipError> {
        // Validate input
        validate_text(text)?;
        
        debug!("Setting clipboard text: {} bytes", text.len());
        
        // Set clipboard
        self.clipboard.set_text(text)
            .map_err(|e| {
                warn!("Failed to set clipboard: {}", e);
                crate::error::KlipError::Arboard(e)
            })?;
        
        info!("Successfully set clipboard: {} characters", text.chars().count());
        Ok(())
    }
    
    pub fn get_text(&mut self) -> Result<String, crate::error::KlipError> {
        debug!("Getting clipboard text");
        
        let text = self.clipboard.get_text()
            .map_err(|e| {
                warn!("Failed to get clipboard: {}", e);
                crate::error::KlipError::Arboard(e)
            })?;
        
        info!("Successfully retrieved clipboard: {} characters", text.chars().count());
        Ok(text)
    }
}

fn validate_text(text: &str) -> Result<(), ValidationError> {
    // Check size
    let size = text.len();
    if size > MAX_SIZE {
        return Err(ValidationError::TextTooLarge { size, max: MAX_SIZE });
    }
    
    // Rust strings are always valid UTF-8, but double-check
    if !text.is_char_boundary(text.len()) {
        return Err(ValidationError::InvalidUtf8);
    }
    
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_validate_text_normal() {
        assert!(validate_text("Hello, world!").is_ok());
    }
    
    #[test]
    fn test_validate_text_empty() {
        // Empty is allowed
        assert!(validate_text("").is_ok());
    }
    
    #[test]
    fn test_validate_text_too_large() {
        let large_text = "a".repeat(MAX_SIZE + 1);
        assert!(matches!(
            validate_text(&large_text),
            Err(ValidationError::TextTooLarge { .. })
        ));
    }
    
    #[test]
    fn test_validate_text_unicode() {
        assert!(validate_text("Hello 世界 🌍").is_ok());
    }
}
```

### Step 5: Create Tool Handlers

Create `src/tools.rs`:

```rust
use crate::clipboard::ClipboardManager;
use rmcp::model::*;
use rmcp::{tool, tool_router, ServerHandler};
use std::sync::Arc;
use tokio::sync::Mutex;
use tracing::info;

#[derive(Clone)]
pub struct ClipboardServer {
    clipboard: Arc<Mutex<ClipboardManager>>,
    tool_router: rmcp::handler::server::tool::ToolRouter<Self>,
}

#[tool_router]
impl ClipboardServer {
    pub fn new() -> Result<Self, crate::error::ClipboardError> {
        let clipboard = ClipboardManager::new()?;
        Ok(Self {
            clipboard: Arc::new(Mutex::new(clipboard)),
            tool_router: Self::tool_router(),
        })
    }
    
    #[tool(description = "Copy text to the system clipboard")]
    async fn clipboard_set(
        &self,
        #[arg(description = "The text to copy to the clipboard")] text: String,
    ) -> Result<CallToolResult, rmcp::ErrorData> {
        info!("clipboard_set called with {} characters", text.chars().count());
        
        let mut clipboard = self.clipboard.lock().await;
        clipboard.set_text(&text)
            .map_err(|e| rmcp::ErrorData::from(e))?;
        
        let char_count = text.chars().count();
        Ok(CallToolResult::success(vec![
            Content::text(format!(
                "Successfully copied {} characters to clipboard",
                char_count
            ))
        ]))
    }
    
    // Optional: clipboard_get for future enhancement
    #[tool(description = "Get text from the system clipboard")]
    async fn clipboard_get(&self) -> Result<CallToolResult, rmcp::ErrorData> {
        info!("clipboard_get called");
        
        let mut clipboard = self.clipboard.lock().await;
        let text = clipboard.get_text()
            .map_err(|e| rmcp::ErrorData::from(e))?;
        
        Ok(CallToolResult::success(vec![
            Content::text(text)
        ]))
    }
}

impl ServerHandler for ClipboardServer {
    fn get_info(&self) -> ServerInfo {
        ServerInfo {
            name: "klip".to_string(),
            version: env!("CARGO_PKG_VERSION").to_string(),
            protocol_version: "2024-11-05".to_string(),
            instructions: Some(
                "Cross-platform clipboard operations for MCP. Use clipboard_set to copy text.".to_string()
            ),
            capabilities: ServerCapabilities::builder()
                .enable_tools()
                .build(),
        }
    }
}
```

### Step 6: Create Main Entry Point

Create `src/main.rs`:

```rust
mod clipboard;
mod error;
mod tools;

use rmcp::{ServiceExt, transport::stdio};
use tools::ClipboardServer;
use tracing::info;
use tracing_subscriber;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize logging
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new("info"))
        )
        .init();
    
    info!("Starting Klip MCP clipboard server v{}", env!("CARGO_PKG_VERSION"));
    
    // Create clipboard server
    let server = ClipboardServer::new()
        .map_err(|e| {
            eprintln!("Failed to initialize clipboard: {}", e);
            std::process::exit(1);
        })?;
    
    info!("Clipboard server initialized");
    
    // Start MCP server with stdio transport
    let service = server.serve(stdio()).await?;
    info!("MCP server started, listening on stdio");
    
    // Wait for shutdown
    service.waiting().await?;
    info!("MCP server shutting down");
    
    Ok(())
}
```

### Step 7: Create .gitignore

Create `.gitignore`:

```
# Rust
/target/
Cargo.lock
**/*.rs.bk
*.pdb

# IDE
.vscode/
.idea/
*.swp
*.swo
*~

# OS
.DS_Store
Thumbs.db

# Testing
/tmp/
```

### Step 8: Build the Project

```bash
# Debug build
cargo build

# Release build (optimized)
cargo build --release

# Check for errors
cargo check

# Run tests
cargo test

# Run clippy (linting)
cargo clippy -- -D warnings

# Format code
cargo fmt
```

### Step 9: Test Locally

```bash
# Run the server
cargo run

# Or run the release binary
./target/release/klip
```

### Step 10: Configure MCP Client

For Claude Desktop, add to `claude_desktop_config.json`:

**macOS**: `~/Library/Application Support/Claude/claude_desktop_config.json`
**Windows**: `%APPDATA%\Claude\claude_desktop_config.json`
**Linux**: `~/.config/Claude/claude_desktop_config.json`

```json
{
  "mcpServers": {
    "klip": {
      "command": "/absolute/path/to/klip/target/release/klip",
      "args": [],
      "env": {
        "RUST_LOG": "info"
      }
    }
  }
}
```

### Step 11: Test with Claude Desktop

1. Restart Claude Desktop
2. Check that klip appears in available MCP servers
3. Try a prompt: "Copy the text 'Hello from Klip!' to my clipboard"
4. Verify the text is in your clipboard (try pasting)

## Development Workflow

### Running Tests

```bash
# All tests
cargo test

# Specific test
cargo test test_validate_text

# With output
cargo test -- --nocapture

# Integration tests only
cargo test --test '*'
```

### Debugging

Enable debug logging:
```bash
RUST_LOG=klip=debug cargo run
```

### Code Quality

```bash
# Format check
cargo fmt --check

# Linting
cargo clippy -- -D warnings

# Security audit
cargo audit

# Dependency updates
cargo update
```

## Common Issues and Solutions

### Issue 1: Clipboard Not Available

**Error**: "Clipboard is not available or accessible"

**Solutions**:
- **Linux**: Ensure X11 is running (`echo $DISPLAY`)
- **Linux Wayland**: Enable wayland feature: `cargo build --features wayland`
- **macOS**: Check System Preferences > Security & Privacy > Accessibility
- **Windows**: Ensure no other app has exclusive clipboard access

### Issue 2: Build Fails on Linux

**Error**: "cannot find -lxcb"

**Solution**: Install required libraries:
```bash
sudo apt-get install libxcb1-dev libxcb-render0-dev libxcb-shape0-dev libxcb-xfixes0-dev
```

### Issue 3: Permission Denied

**Error**: "Permission to access clipboard was denied"

**Solution**: Grant clipboard access permissions in system settings

### Issue 4: Text Too Large

**Error**: "Text exceeds maximum size"

**Solution**: Split text into chunks or increase MAX_SIZE in `clipboard.rs`

## Next Steps

1. **Add Tests**: Write integration tests for clipboard operations
2. **Documentation**: Add rustdoc comments to all public APIs
3. **CI/CD**: Set up GitHub Actions for automated builds and tests
4. **Features**: Implement clipboard_get, image support, etc.
5. **Distribution**: Create releases, publish to crates.io

## Additional Resources

- [MCP Specification](https://modelcontextprotocol.io/specification/)
- [rmcp Documentation](https://docs.rs/rmcp)
- [arboard Documentation](https://docs.rs/arboard)
- [Rust Book](https://doc.rust-lang.org/book/)
- [Tokio Tutorial](https://tokio.rs/tokio/tutorial)

## Getting Help

- Check the TECHNICAL_SPEC.md for detailed specifications
- Review IMPLEMENTATION_PLAN.md for the full roadmap
- Open an issue on GitHub for bugs or questions
- Consult MCP community resources

## Summary Checklist

- [ ] Install Rust and system dependencies
- [ ] Create Cargo.toml with dependencies
- [ ] Implement error types (error.rs)
- [ ] Implement clipboard module (clipboard.rs)
- [ ] Implement tool handlers (tools.rs)
- [ ] Create main entry point (main.rs)
- [ ] Build and test locally
- [ ] Configure MCP client (Claude Desktop)
- [ ] Test end-to-end with Claude
- [ ] Add documentation and tests
- [ ] Set up CI/CD

Congratulations! You now have a working MCP clipboard server. 🎉
