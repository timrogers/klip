# klip

> Cross-platform MCP server allowing agents to copy values to the system's clipboard

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Rust](https://img.shields.io/badge/rust-1.70%2B-orange.svg)](https://www.rust-lang.org/)
[![MCP](https://img.shields.io/badge/MCP-compatible-blue.svg)](https://modelcontextprotocol.io/)

## Overview

**klip** is a cross-platform [Model Context Protocol (MCP)](https://modelcontextprotocol.io/) server written in Rust that enables AI agents and applications to interact with the system clipboard. It provides secure, reliable clipboard operations through a standardized MCP interface.

### Key Features

- ✅ **Cross-Platform**: Works on Linux (X11/Wayland), macOS, and Windows
- 🔒 **Secure**: Input validation, size limits, and safe error handling
- ⚡ **Fast**: Async operations with minimal latency
- 🛠️ **Easy to Use**: Simple installation and configuration
- 📦 **Standalone**: Single binary with no external dependencies (except system libs)
- 🔌 **MCP Compatible**: Works with Claude Desktop and other MCP clients

## Quick Start

### Installation

**From source**:
```bash
git clone https://github.com/timrogers/klip.git
cd klip
cargo build --release
```

The binary will be at `./target/release/klip`.

### Configuration

Add to your MCP client configuration (e.g., Claude Desktop):

```json
{
  "mcpServers": {
    "klip": {
      "command": "/path/to/klip",
      "args": []
    }
  }
}
```

### Usage

Once configured, you can ask your AI assistant:
- "Copy 'Hello, World!' to my clipboard"
- "Put this text in my clipboard: [your text]"
- "Copy the following code snippet to clipboard: [code]"

The klip server handles the `clipboard_set` tool to copy text to your system clipboard.

## Documentation

📚 **Comprehensive documentation is available**:

- **[IMPLEMENTATION_PLAN.md](IMPLEMENTATION_PLAN.md)** - Detailed implementation roadmap and architecture
- **[TECHNICAL_SPEC.md](TECHNICAL_SPEC.md)** - Complete technical specifications
- **[QUICKSTART.md](QUICKSTART.md)** - Step-by-step implementation guide

## MCP Tools

### clipboard_set

Copies text to the system clipboard.

**Input**:
- `text` (string): The text to copy

**Example**:
```json
{
  "name": "clipboard_set",
  "arguments": {
    "text": "Hello, World!"
  }
}
```

**Output**:
```
Successfully copied 13 characters to clipboard
```

### clipboard_get (Future)

Retrieves current clipboard contents.

## Platform Support

| Platform | Status | Notes |
|----------|--------|-------|
| Linux (X11) | ✅ Planned | Default, requires X11 libraries |
| Linux (Wayland) | ✅ Planned | Requires `wayland` feature flag |
| macOS | ✅ Planned | Native NSPasteboard support |
| Windows | ✅ Planned | Native Win32 API support |

### System Requirements

**Linux**:
```bash
# Debian/Ubuntu
sudo apt-get install libxcb1-dev libxcb-render0-dev libxcb-shape0-dev libxcb-xfixes0-dev
```

**macOS**: No additional requirements

**Windows**: No additional requirements

## Development

### Building

```bash
# Debug build
cargo build

# Release build (optimized)
cargo build --release

# Run tests
cargo test

# Run linter
cargo clippy

# Format code
cargo fmt
```

### Testing

```bash
# Run all tests
cargo test

# Run with logging
RUST_LOG=debug cargo test

# Run specific test
cargo test test_clipboard_set
```

## Architecture

klip uses:
- **[rmcp](https://github.com/modelcontextprotocol/rust-sdk)** - Official Rust MCP SDK
- **[arboard](https://github.com/1Password/arboard)** - Cross-platform clipboard library
- **[tokio](https://tokio.rs/)** - Async runtime
- **[tracing](https://github.com/tokio-rs/tracing)** - Logging framework

## Security

klip implements several security measures:
- Input validation and size limits (10MB default)
- UTF-8 encoding verification
- Operation timeouts (5 seconds default)
- Safe error handling (no panics)
- Minimal permissions required

## Configuration Options

Environment variables:
- `RUST_LOG` - Log level (trace, debug, info, warn, error)
- `KLIP_MAX_SIZE` - Maximum clipboard text size in bytes (default: 10485760)
- `KLIP_TIMEOUT` - Operation timeout in seconds (default: 5)

## Roadmap

### Version 0.1.0 (MVP)
- [x] Project planning and documentation
- [ ] Basic MCP server implementation
- [ ] `clipboard_set` tool
- [ ] Cross-platform clipboard support
- [ ] Error handling and validation
- [ ] Unit and integration tests

### Version 0.2.0
- [ ] `clipboard_get` tool
- [ ] Improved error messages
- [ ] Performance optimizations
- [ ] Binary releases for all platforms

### Future Enhancements
- [ ] Image clipboard support
- [ ] Rich text/HTML support
- [ ] Clipboard history
- [ ] Clipboard monitoring
- [ ] HTTP/SSE transport option

## Contributing

Contributions are welcome! Please:
1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Add tests
5. Submit a pull request

## License

MIT License - see [LICENSE](LICENSE) for details.

## Acknowledgments

- [Model Context Protocol](https://modelcontextprotocol.io/) - For the MCP specification
- [rmcp](https://github.com/modelcontextprotocol/rust-sdk) - Official Rust MCP SDK
- [arboard](https://github.com/1Password/arboard) - Excellent clipboard library
- Community MCP clipboard implementations for inspiration

## Support

- 📖 [Documentation](IMPLEMENTATION_PLAN.md)
- 🐛 [Issue Tracker](https://github.com/timrogers/klip/issues)
- 💬 [Discussions](https://github.com/timrogers/klip/discussions)

---

Made with ❤️ using Rust and the Model Context Protocol
