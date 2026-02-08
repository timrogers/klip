# klip

Cross-platform MCP server allowing agents to copy values to the system's clipboard

## Status

🚧 **Under Development** - See [IMPLEMENTATION_PLAN.md](IMPLEMENTATION_PLAN.md) for the complete implementation roadmap.

## Overview

**klip** is a Model Context Protocol (MCP) server written in Rust that enables AI agents like Claude to interact with your system clipboard. It provides a secure, cross-platform way for agents to copy text to your clipboard through the MCP protocol.

### Features (Planned)

- ✅ Cross-platform support (Windows, macOS, Linux)
- ✅ Simple `copy_to_clipboard` tool
- ✅ Fast, lightweight Rust implementation
- ✅ Secure clipboard operations with proper error handling
- ✅ Full Unicode and emoji support

## Implementation Plan

See [IMPLEMENTATION_PLAN.md](IMPLEMENTATION_PLAN.md) for:
- Detailed technical architecture
- Phase-by-phase implementation guide
- Platform-specific considerations
- Testing and security strategy
- Timeline and success criteria

## License

MIT License - see [LICENSE](LICENSE) for details.
