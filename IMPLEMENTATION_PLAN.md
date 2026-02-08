# Implementation Plan: Cross-Platform MCP Clipboard Server in Rust

## Overview
This document outlines the detailed plan for implementing a cross-platform Model Context Protocol (MCP) server in Rust that allows AI agents to copy strings to the system clipboard.

## 1. Project Architecture

### 1.1 Technology Stack
- **Language**: Rust (stable)
- **MCP SDK**: `rmcp` (official Rust MCP SDK from modelcontextprotocol/rust-sdk)
- **Clipboard Library**: `arboard` (cross-platform clipboard support)
- **Async Runtime**: Tokio
- **Error Handling**: `thiserror` for custom error types
- **Serialization**: `serde` and `serde_json` (included with rmcp)

### 1.2 Supported Platforms
- **Linux**: X11 and Wayland support
- **macOS**: Native clipboard API
- **Windows**: Native clipboard API

### 1.3 Transport
- **Stdio**: Primary transport for MCP communication (standard for desktop integration)
- Future consideration: HTTP/SSE for web-based clients

## 2. Core Components

### 2.1 Project Structure
```
klip/
├── Cargo.toml              # Project manifest
├── README.md               # User documentation
├── LICENSE                 # License file (already exists)
├── IMPLEMENTATION_PLAN.md  # This file
├── src/
│   ├── main.rs            # Entry point, server setup
│   ├── lib.rs             # Library root (optional, for testability)
│   ├── clipboard.rs       # Clipboard operations wrapper
│   ├── tools.rs           # MCP tool implementations
│   ├── error.rs           # Error types
│   └── config.rs          # Configuration (optional)
├── tests/
│   ├── integration_tests.rs  # Integration tests
│   └── clipboard_tests.rs    # Clipboard-specific tests
└── examples/
    └── basic_usage.md        # Usage examples
```

### 2.2 MCP Tools to Implement

#### Tool 1: `clipboard_set`
- **Description**: Copy a string to the system clipboard
- **Input**:
  - `text` (string, required): The text to copy to clipboard
- **Output**: Success/failure message
- **Error Cases**:
  - Clipboard unavailable
  - Text too large (implement size limit)
  - Permission denied

#### Tool 2: `clipboard_get` (Optional, for future enhancement)
- **Description**: Retrieve current clipboard contents
- **Input**: None
- **Output**: Current clipboard text content
- **Error Cases**:
  - Clipboard unavailable
  - Non-text content
  - Permission denied

### 2.3 Key Dependencies (Cargo.toml)
```toml
[package]
name = "klip"
version = "0.1.0"
edition = "2021"
authors = ["timrogers"]
description = "Cross-platform MCP server for clipboard operations"
license = "MIT"
repository = "https://github.com/timrogers/klip"

[dependencies]
rmcp = "0.2"                    # Official Rust MCP SDK
arboard = "3.4"                 # Cross-platform clipboard
tokio = { version = "1", features = ["full"] }
serde = { version = "1", features = ["derive"] }
serde_json = "1"
thiserror = "2"
tracing = "0.1"                 # Logging
tracing-subscriber = "0.3"      # Logging setup

[dev-dependencies]
tokio-test = "0.4"

[features]
default = ["x11"]
x11 = ["arboard/default"]
wayland = ["arboard/wayland-data-control"]
```

## 3. Implementation Steps

### Phase 1: Project Setup
1. Initialize Cargo project with appropriate metadata
2. Add all required dependencies to Cargo.toml
3. Set up basic project structure (src/, tests/, examples/)
4. Configure logging with tracing
5. Create .gitignore for Rust projects

### Phase 2: Clipboard Module
1. Create `src/clipboard.rs` with wrapper around arboard
2. Implement cross-platform clipboard operations:
   - `set_text(text: &str) -> Result<(), ClipboardError>`
   - `get_text() -> Result<String, ClipboardError>` (optional)
3. Add size limits and validation
4. Handle platform-specific edge cases:
   - Linux: X11 vs Wayland detection
   - macOS: Pasteboard access permissions
   - Windows: Clipboard format handling
5. Write unit tests for clipboard operations

### Phase 3: Error Handling
1. Create `src/error.rs` with custom error types:
   - `ClipboardError`: Clipboard operation failures
   - `McpError`: MCP-specific errors
2. Implement proper error conversion and propagation
3. Add descriptive error messages for users

### Phase 4: MCP Server Implementation
1. Create `src/tools.rs` with tool implementations:
   - Define `ClipboardServer` struct
   - Implement tool handlers using `#[tool]` macro
   - Implement `clipboard_set` tool
   - (Optional) Implement `clipboard_get` tool
2. Use `#[tool_router]` macro for routing
3. Implement `ServerHandler` trait for server capabilities

### Phase 5: Main Entry Point
1. Create `src/main.rs`:
   - Initialize tracing/logging
   - Create ClipboardServer instance
   - Set up stdio transport
   - Start server and handle lifecycle
2. Add graceful shutdown handling
3. Add command-line argument parsing (optional, for config)

### Phase 6: Documentation
1. Update README.md with:
   - Project description
   - Installation instructions
   - Configuration guide for MCP clients (Claude Desktop, etc.)
   - Usage examples
   - Platform-specific requirements
   - Troubleshooting section
2. Add inline documentation (rustdoc comments)
3. Create examples in examples/ directory
4. Add CONTRIBUTING.md (optional)

### Phase 7: Testing
1. Write unit tests for clipboard module
2. Write integration tests for MCP tools
3. Test on all three platforms (Linux, macOS, Windows)
4. Test with actual MCP clients:
   - Claude Desktop
   - Custom test harness
5. Test edge cases:
   - Empty strings
   - Very large strings
   - Unicode and emoji
   - Special characters

### Phase 8: Build and Distribution
1. Set up GitHub Actions CI/CD:
   - Build on Linux, macOS, Windows
   - Run tests
   - Run clippy (linting)
   - Run cargo fmt (formatting check)
2. Create release binaries:
   - Compile optimized builds for each platform
   - Package with documentation
3. Add cargo publish configuration
4. Create installation scripts (optional)

## 4. Configuration

### 4.1 MCP Client Configuration (Claude Desktop Example)
```json
{
  "mcpServers": {
    "klip": {
      "command": "/path/to/klip",
      "args": [],
      "env": {
        "RUST_LOG": "info"
      }
    }
  }
}
```

### 4.2 Environment Variables (Optional)
- `RUST_LOG`: Log level (trace, debug, info, warn, error)
- `KLIP_MAX_SIZE`: Maximum clipboard text size in bytes (default: 10MB)
- `KLIP_TIMEOUT`: Operation timeout in seconds (default: 5s)

## 5. Security Considerations

### 5.1 Size Limits
- Implement maximum clipboard text size (default 10MB)
- Prevent memory exhaustion attacks
- Validate input before clipboard operations

### 5.2 Validation
- Sanitize input strings (check for null bytes, etc.)
- Validate UTF-8 encoding
- Handle invalid Unicode gracefully

### 5.3 Permissions
- Document platform-specific permission requirements
- Handle permission denied errors gracefully
- Provide clear error messages for permission issues

### 5.4 Dependencies
- Use only well-maintained, reputable crates
- Regular dependency updates via Dependabot
- Security audit with cargo-audit

## 6. Performance Considerations

### 6.1 Async Operations
- Use async/await throughout for non-blocking operations
- Clipboard operations should not block the event loop
- Consider using `tokio::task::spawn_blocking` for clipboard ops if needed

### 6.2 Resource Management
- Properly clean up clipboard resources
- Handle clipboard locks/ownership correctly (especially on Linux)
- Implement timeouts for clipboard operations

## 7. Platform-Specific Details

### 7.1 Linux
- **Default**: X11 clipboard support
- **Wayland**: Enable with feature flag `wayland`
- **Dependencies**: X11 libraries or Wayland compositor
- **Notes**: Clipboard persistence after app exit may vary

### 7.2 macOS
- **API**: Uses NSPasteboard
- **Permissions**: May require accessibility permissions for some apps
- **Notes**: Generally works out of the box

### 7.3 Windows
- **API**: Uses Windows Clipboard API
- **Notes**: Clipboard must be opened/closed properly
- **Formats**: Handle CF_UNICODETEXT format

## 8. Testing Strategy

### 8.1 Unit Tests
- Test clipboard wrapper functions in isolation
- Mock clipboard operations where possible
- Test error handling paths

### 8.2 Integration Tests
- Test MCP tool invocations end-to-end
- Test with actual clipboard operations
- Verify tool responses conform to MCP spec

### 8.3 Manual Testing
- Test with Claude Desktop on each platform
- Verify clipboard contents after operations
- Test with various text inputs (Unicode, emojis, special chars)

### 8.4 CI/CD Tests
- Automated builds on all platforms
- Run unit tests in CI (may need headless setup for clipboard)
- Lint and format checks

## 9. Documentation Deliverables

### 9.1 README.md
- Project overview and features
- Quick start guide
- Installation instructions
- MCP client configuration examples
- Platform-specific setup
- Troubleshooting

### 9.2 Code Documentation
- Rustdoc comments on all public APIs
- Examples in documentation
- Module-level documentation

### 9.3 Usage Examples
- Basic clipboard_set example
- Integration with Claude Desktop
- Error handling examples
- Advanced configuration

## 10. Future Enhancements (Post-MVP)

### 10.1 Additional Features
- Image clipboard support (PNG, JPEG)
- Rich text/HTML clipboard support
- Clipboard history
- Clipboard monitoring/notifications
- Multiple clipboard format support

### 10.2 Additional Transports
- HTTP/SSE transport for web clients
- WebSocket transport for real-time applications
- Unix socket transport for local IPC

### 10.3 Advanced Security
- API token authentication
- Rate limiting
- Audit logging

### 10.4 UI/UX Improvements
- Configuration GUI (optional)
- System tray integration
- Clipboard preview

## 11. Success Criteria

The implementation will be considered successful when:
1. ✓ Server compiles on Linux, macOS, and Windows
2. ✓ `clipboard_set` tool works reliably on all platforms
3. ✓ Server integrates with Claude Desktop or similar MCP client
4. ✓ Comprehensive documentation is available
5. ✓ Unit and integration tests pass
6. ✓ CI/CD pipeline is functional
7. ✓ Error handling is robust and user-friendly
8. ✓ Code follows Rust best practices (clippy, fmt)

## 12. Timeline Estimate

- **Phase 1-2**: Project setup and clipboard module (2-4 hours)
- **Phase 3-5**: Error handling and MCP implementation (3-5 hours)
- **Phase 6-7**: Documentation and testing (2-4 hours)
- **Phase 8**: CI/CD and distribution (2-3 hours)

**Total Estimated Time**: 9-16 hours for a complete, production-ready implementation

## 13. Risk Mitigation

### 13.1 Identified Risks
1. **Platform-specific clipboard issues**: Mitigate with thorough testing on each platform
2. **MCP SDK changes**: Pin SDK version, monitor for updates
3. **Clipboard permission issues**: Document requirements clearly, handle errors gracefully
4. **CI/CD headless clipboard testing**: May need special setup or skip actual clipboard tests in CI

### 13.2 Contingency Plans
- If arboard has issues, consider clipboard-rs as alternative
- If rmcp SDK is problematic, could use mcpr or implement MCP protocol directly
- If stdio transport has issues, can implement HTTP/SSE as alternative

## 14. References

### 14.1 MCP Protocol
- Official Specification: https://modelcontextprotocol.io/specification/
- Rust SDK Documentation: https://docs.rs/rmcp
- GitHub Repository: https://github.com/modelcontextprotocol/rust-sdk

### 14.2 Clipboard Libraries
- arboard: https://github.com/1Password/arboard
- arboard docs: https://docs.rs/arboard
- clipboard-rs: https://github.com/ChurchTao/clipboard-rs

### 14.3 Example Implementations
- MCP Clipboard Servers (various languages): https://mcpmarket.com/server/clipboard-2
- Go implementation: https://github.com/smorand/clipboard-mcp

## 15. Conclusion

This plan provides a comprehensive roadmap for implementing a robust, cross-platform MCP clipboard server in Rust. The implementation will leverage the official rmcp SDK for MCP protocol handling and arboard for clipboard operations, ensuring reliability and maintainability. The phased approach allows for incremental development and testing, with clear success criteria and risk mitigation strategies.
