# Technical Specification: Klip MCP Clipboard Server

## 1. System Overview

### 1.1 Purpose
Klip is a cross-platform Model Context Protocol (MCP) server that enables AI agents and applications to interact with the system clipboard programmatically. It provides a secure, standardized interface for clipboard operations through the MCP protocol.

### 1.2 Design Goals
- **Cross-platform**: Support Linux (X11/Wayland), macOS, and Windows
- **Reliability**: Robust error handling and graceful degradation
- **Security**: Input validation, size limits, and safe clipboard operations
- **Performance**: Async operations, minimal latency
- **Simplicity**: Easy to install, configure, and use
- **Standards-compliant**: Full MCP protocol compliance

## 2. MCP Protocol Implementation

### 2.1 Server Capabilities
```json
{
  "capabilities": {
    "tools": {
      "listChanged": false
    }
  }
}
```

### 2.2 Server Information
```json
{
  "name": "klip",
  "version": "0.1.0",
  "protocolVersion": "2024-11-05",
  "instructions": "Clipboard operations for copying text to the system clipboard"
}
```

## 3. Tool Specifications

### 3.1 clipboard_set

#### Description
Copies a text string to the system clipboard, making it available for pasting in other applications.

#### Input Schema
```json
{
  "type": "object",
  "properties": {
    "text": {
      "type": "string",
      "description": "The text to copy to the clipboard",
      "maxLength": 10485760
    }
  },
  "required": ["text"]
}
```

#### Output Schema (Success)
```json
{
  "content": [
    {
      "type": "text",
      "text": "Successfully copied {length} characters to clipboard"
    }
  ]
}
```

#### Output Schema (Error)
```json
{
  "isError": true,
  "content": [
    {
      "type": "text",
      "text": "Error: {error_message}"
    }
  ]
}
```

#### Behavior
1. Validate input text (non-null, valid UTF-8, size check)
2. Open clipboard connection
3. Clear existing clipboard contents
4. Set new clipboard text
5. Close clipboard connection
6. Return success/failure status

#### Error Codes and Messages
- `CLIPBOARD_UNAVAILABLE`: "Clipboard is not available or accessible"
- `TEXT_TOO_LARGE`: "Text exceeds maximum size limit of 10MB"
- `INVALID_UTF8`: "Text contains invalid UTF-8 sequences"
- `PERMISSION_DENIED`: "Permission to access clipboard was denied"
- `OPERATION_TIMEOUT`: "Clipboard operation timed out after 5 seconds"
- `UNKNOWN_ERROR`: "An unexpected error occurred: {details}"

### 3.2 clipboard_get (Future Enhancement)

#### Description
Retrieves the current text content from the system clipboard.

#### Input Schema
```json
{
  "type": "object",
  "properties": {}
}
```

#### Output Schema (Success)
```json
{
  "content": [
    {
      "type": "text",
      "text": "{clipboard_contents}"
    }
  ]
}
```

#### Error Codes
- Same as clipboard_set, plus:
- `NO_TEXT_CONTENT`: "Clipboard does not contain text content"
- `CLIPBOARD_EMPTY`: "Clipboard is empty"

## 4. Architecture

### 4.1 Component Diagram
```
┌─────────────────────────────────────────┐
│         MCP Client (e.g., Claude)       │
└─────────────┬───────────────────────────┘
              │ JSON-RPC over stdio
              │
┌─────────────▼───────────────────────────┐
│         Klip MCP Server (main.rs)       │
│  ┌──────────────────────────────────┐   │
│  │   rmcp Server Framework          │   │
│  │  - Protocol handling             │   │
│  │  - Tool routing                  │   │
│  │  - Async runtime                 │   │
│  └──────────┬───────────────────────┘   │
│             │                            │
│  ┌──────────▼───────────────────────┐   │
│  │   Tool Handlers (tools.rs)       │   │
│  │  - clipboard_set                 │   │
│  │  - Input validation              │   │
│  │  - Error handling                │   │
│  └──────────┬───────────────────────┘   │
│             │                            │
│  ┌──────────▼───────────────────────┐   │
│  │   Clipboard Module (clipboard.rs)│   │
│  │  - Platform abstraction          │   │
│  │  - Size limits                   │   │
│  │  - Timeout handling              │   │
│  └──────────┬───────────────────────┘   │
└─────────────┼───────────────────────────┘
              │
┌─────────────▼───────────────────────────┐
│      arboard Clipboard Library          │
│  ┌────────┬──────────┬──────────────┐   │
│  │ Linux  │  macOS   │   Windows    │   │
│  │ X11/   │ NSPaste- │  Win32 API   │   │
│  │Wayland │ board    │              │   │
│  └────────┴──────────┴──────────────┘   │
└─────────────┬───────────────────────────┘
              │
┌─────────────▼───────────────────────────┐
│        System Clipboard                 │
└─────────────────────────────────────────┘
```

### 4.2 Data Flow

#### clipboard_set Operation
```
1. Client sends JSON-RPC request:
   {
     "jsonrpc": "2.0",
     "id": 1,
     "method": "tools/call",
     "params": {
       "name": "clipboard_set",
       "arguments": {
         "text": "Hello, World!"
       }
     }
   }

2. Server receives request via stdio

3. rmcp framework deserializes and routes to clipboard_set handler

4. Handler validates input:
   - Check text is not null
   - Verify UTF-8 encoding
   - Check size <= 10MB

5. Handler calls clipboard module:
   clipboard::set_text("Hello, World!")

6. Clipboard module calls arboard:
   let mut clipboard = Clipboard::new()?;
   clipboard.set_text("Hello, World!")?;

7. arboard interacts with OS clipboard

8. Success/error bubbles back up the chain

9. Server sends JSON-RPC response:
   {
     "jsonrpc": "2.0",
     "id": 1,
     "result": {
       "content": [{
         "type": "text",
         "text": "Successfully copied 13 characters to clipboard"
       }]
     }
   }
```

## 5. Error Handling Strategy

### 5.1 Error Types Hierarchy
```rust
// Error hierarchy
pub enum KlipError {
    Clipboard(ClipboardError),
    Validation(ValidationError),
    Mcp(McpError),
}

pub enum ClipboardError {
    Unavailable,
    PermissionDenied,
    OperationTimeout,
    PlatformSpecific(String),
}

pub enum ValidationError {
    TextTooLarge { size: usize, max: usize },
    InvalidUtf8,
    EmptyText,
}
```

### 5.2 Error Handling Principles
1. **Fail fast**: Validate early, before expensive operations
2. **Descriptive messages**: Include context and actionable information
3. **No panics**: All errors must be Result<T, E> or gracefully handled
4. **Logging**: Log errors at appropriate levels (warn, error)
5. **Recovery**: Attempt graceful degradation where possible

## 6. Security Measures

### 6.1 Input Validation
```rust
fn validate_text(text: &str) -> Result<(), ValidationError> {
    // Check empty
    if text.is_empty() {
        return Err(ValidationError::EmptyText);
    }
    
    // Check UTF-8 validity (Rust strings are UTF-8, but verify)
    if !text.is_char_boundary(text.len()) {
        return Err(ValidationError::InvalidUtf8);
    }
    
    // Check size limit (10MB)
    let size = text.len();
    const MAX_SIZE: usize = 10 * 1024 * 1024;
    if size > MAX_SIZE {
        return Err(ValidationError::TextTooLarge { 
            size, 
            max: MAX_SIZE 
        });
    }
    
    Ok(())
}
```

### 6.2 Resource Limits
- **Maximum text size**: 10MB (10,485,760 bytes)
- **Operation timeout**: 5 seconds
- **Memory**: Bounded by text size limit

### 6.3 Permissions
- **Linux**: Requires X11/Wayland display access
- **macOS**: May require accessibility permissions for some apps
- **Windows**: Standard user clipboard access

## 7. Performance Specifications

### 7.1 Latency Targets
- **Small text (< 1KB)**: < 10ms
- **Medium text (1KB - 100KB)**: < 50ms
- **Large text (100KB - 1MB)**: < 200ms
- **Maximum text (10MB)**: < 1000ms

### 7.2 Throughput
- Sequential operations: ~100 ops/sec for small text
- Async operations: Limited by clipboard locking, typically 1 at a time

### 7.3 Resource Usage
- **Memory**: Base: ~5MB, Peak: Base + text size
- **CPU**: Minimal, mostly I/O bound
- **Startup time**: < 100ms

## 8. Platform-Specific Implementation

### 8.1 Linux (X11)
```rust
// Features required
arboard = { version = "3.4", default-features = true }

// Runtime dependencies
// - libxcb
// - libxcb-render
// - libxcb-shape
// - libxcb-xfixes

// Behavior
// - Uses XCB (X11) protocol
// - Clipboard may clear when app exits
// - Supports CLIPBOARD and PRIMARY selections
```

### 8.2 Linux (Wayland)
```rust
// Features required
arboard = { version = "3.4", features = ["wayland-data-control"] }

// Runtime dependencies
// - Wayland compositor with wlr-data-control protocol

// Behavior
// - Uses wlr-data-control protocol
// - Clipboard persistence depends on compositor
```

### 8.3 macOS
```rust
// No special features required
arboard = { version = "3.4" }

// System frameworks (linked automatically)
// - AppKit.framework

// Behavior
// - Uses NSPasteboard
// - Clipboard persists after app exit
// - Full Unicode support
```

### 8.4 Windows
```rust
// No special features required
arboard = { version = "3.4" }

// System libraries (linked automatically)
// - user32.dll
// - kernel32.dll

// Behavior
// - Uses Win32 Clipboard API
// - CF_UNICODETEXT format
// - Clipboard persists after app exit
```

## 9. Configuration

### 9.1 Environment Variables
```bash
# Logging level
RUST_LOG=klip=debug,rmcp=info

# Maximum clipboard text size (bytes)
KLIP_MAX_SIZE=10485760

# Clipboard operation timeout (seconds)
KLIP_TIMEOUT=5

# Platform-specific: Force X11 on Linux
KLIP_FORCE_X11=1
```

### 9.2 MCP Client Configuration
```json
{
  "mcpServers": {
    "klip": {
      "command": "/path/to/klip",
      "args": [],
      "env": {
        "RUST_LOG": "info",
        "KLIP_MAX_SIZE": "10485760",
        "KLIP_TIMEOUT": "5"
      }
    }
  }
}
```

## 10. Testing Requirements

### 10.1 Unit Tests
- Clipboard wrapper functions (mocked)
- Input validation functions
- Error conversion and formatting
- Configuration parsing

### 10.2 Integration Tests
- End-to-end tool invocation
- Actual clipboard operations (platform-dependent)
- Error scenarios
- Timeout handling

### 10.3 Platform Tests
- Linux X11 environment
- Linux Wayland environment (if available)
- macOS
- Windows

### 10.4 Test Coverage Target
- Line coverage: > 80%
- Branch coverage: > 70%
- Critical paths: 100%

## 11. Logging Strategy

### 11.1 Log Levels
```rust
// ERROR: Critical failures
tracing::error!("Failed to access clipboard: {}", err);

// WARN: Recoverable issues
tracing::warn!("Clipboard text truncated to {} bytes", MAX_SIZE);

// INFO: Normal operations
tracing::info!("Clipboard set: {} characters", text.len());

// DEBUG: Detailed flow
tracing::debug!("Initializing clipboard connection");

// TRACE: Very detailed
tracing::trace!("Validated UTF-8: {} bytes", text.len());
```

### 11.2 Structured Logging
```rust
use tracing::{info, error};

#[tracing::instrument]
async fn set_clipboard(text: &str) -> Result<(), ClipboardError> {
    info!(text_len = text.len(), "Setting clipboard");
    // ...
}
```

## 12. Deployment

### 12.1 Binary Distribution
- **Linux**: Statically linked binary (or with shared libs)
- **macOS**: Universal binary (x86_64 + arm64)
- **Windows**: .exe with Visual C++ runtime

### 12.2 Package Managers
- **Cargo**: `cargo install klip`
- **Homebrew** (macOS): `brew install klip`
- **Chocolatey** (Windows): `choco install klip`
- **AUR** (Arch Linux): `yay -S klip`

### 12.3 Container Support (Future)
- Docker image with X11/Wayland support
- Not recommended (clipboard access issues)

## 13. Monitoring and Observability

### 13.1 Metrics (Future Enhancement)
- Operations per second
- Success/failure rates
- Latency percentiles (p50, p95, p99)
- Error types distribution

### 13.2 Health Checks
- Clipboard availability check
- MCP protocol version compatibility

## 14. Backwards Compatibility

### 14.1 Version Policy
- Semantic versioning (SemVer)
- Major version: Breaking changes
- Minor version: New features (backwards compatible)
- Patch version: Bug fixes

### 14.2 MCP Protocol Versions
- Support current MCP protocol version
- Gracefully handle version negotiation

## 15. Dependencies

### 15.1 Direct Dependencies
```toml
rmcp = "0.2"           # MIT/Apache-2.0
arboard = "3.4"        # MIT/Apache-2.0
tokio = "1"            # MIT
serde = "1"            # MIT/Apache-2.0
serde_json = "1"       # MIT/Apache-2.0
thiserror = "2"        # MIT/Apache-2.0
tracing = "0.1"        # MIT
tracing-subscriber = "0.3"  # MIT
```

### 15.2 Build Dependencies
```toml
[build-dependencies]
# None required for basic build
```

### 15.3 Platform-Specific System Dependencies

**Linux (Debian/Ubuntu)**:
```bash
apt-get install libxcb1-dev libxcb-render0-dev libxcb-shape0-dev libxcb-xfixes0-dev
```

**Linux (Fedora)**:
```bash
dnf install libxcb-devel
```

**macOS**: None (uses system frameworks)

**Windows**: None (uses system libraries)

## 16. Future Considerations

### 16.1 Image Support
- PNG format support
- JPEG format support
- Base64 encoding for transport

### 16.2 Rich Text Support
- HTML clipboard format
- RTF (Rich Text Format)
- Markdown with formatting

### 16.3 Clipboard History
- Store recent clipboard items
- Retrieve from history
- Persistence across restarts

### 16.4 Clipboard Monitoring
- Watch for clipboard changes
- Notify clients of changes
- Event-based API

### 16.5 Multi-Clipboard Support
- Named clipboards
- Multiple clipboard buffers
- Clipboard synchronization

## 17. References

- MCP Specification: https://modelcontextprotocol.io/specification/
- rmcp Documentation: https://docs.rs/rmcp
- arboard Documentation: https://docs.rs/arboard
- JSON-RPC 2.0: https://www.jsonrpc.org/specification
