# klip

Cross-platform MCP server allowing agents to copy values to the system's clipboard

## Status

✅ **v0.1.0 - Ready for use!**

## Overview

**klip** is a Model Context Protocol (MCP) server written in Rust that enables AI agents like Claude to interact with your system clipboard. It provides a secure, cross-platform way for agents to copy text to your clipboard through the MCP protocol.

### Features

- ✅ Cross-platform support (Windows, macOS, Linux)
- ✅ Simple `copy_to_clipboard` tool
- ✅ Fast, lightweight Rust implementation
- ✅ Secure clipboard operations with proper error handling
- ✅ Full Unicode and emoji support

## Installation

### Pre-built Binaries

Download the latest release for your platform from the [GitHub Releases page](https://github.com/timrogers/klip/releases).

### From Source

```bash
cargo install klip
```

Or build from source:

```bash
git clone https://github.com/timrogers/klip.git
cd klip
cargo build --release
```

The binary will be available at `target/release/klip` (or `target/release/klip.exe` on Windows).

## Usage

### With Claude Desktop

Add the server to your Claude Desktop configuration file:

**macOS**: `~/Library/Application Support/Claude/claude_desktop_config.json`  
**Windows**: `%APPDATA%\Claude\claude_desktop_config.json`  
**Linux**: `~/.config/Claude/claude_desktop_config.json`

```json
{
  "mcpServers": {
    "klip": {
      "command": "/path/to/klip"
    }
  }
}
```

Replace `/path/to/klip` with the actual path to the klip binary.

### With Other MCP Clients

klip communicates via stdin/stdout using the MCP protocol, so it can be used with any MCP-compatible client. Simply configure your client to run the `klip` binary.

## Available Tools

### `copy_to_clipboard`

Copies text to the system clipboard.

**Parameters:**
- `text` (string, required): The text content to copy to the clipboard

**Example:**
```json
{
  "name": "copy_to_clipboard",
  "arguments": {
    "text": "Hello, World!"
  }
}
```

**Response:**
```
Successfully copied 13 characters to clipboard
```

## Development

### Building

```bash
cargo build
```

### Testing

```bash
cargo test
```

### Running Locally

```bash
cargo run
```

### CLI Options

```bash
# Show help
klip --help

# Show version
klip --version
```

## Platform Support

- **Linux**: X11 and Wayland
- **macOS**: 10.13 High Sierra and later
- **Windows**: Windows 10 and later

## License

MIT License - see [LICENSE](LICENSE) for details.

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.
