# klip

🤖📎 Cross-platform MCP server allowing agents to copy values to the system clipboard

### Features

- ✅ Cross-platform support (Windows, macOS, Linux)
- ✅ Simple `copy_to_clipboard` tool
- ✅ Fast, lightweight Rust implementation
- ✅ Full Unicode and emoji support

## Installation

* **macOS or Linux via [Homebrew](https://brew.sh/)**: `brew tap timrogers/tap && brew install klip`
**macOS, Linux or Windows via [Cargo](https://doc.rust-lang.org/cargo/), Rust's package manager**: `cargo install klip`
**macOS, Linux or Windows via direct binary download**: Download the [latest release](https://github.com/timrogers/litra-rs/releases/latest) for your platform, then copy the binary to `$PATH`

## Usage

### Visual Studio Code

Use the buttons below to install the server.

[![Install in VS Code](https://img.shields.io/badge/VS_Code-Install_Server-0098FF?style=flat-square&logo=visualstudiocode&logoColor=white)](https://insiders.vscode.dev/redirect/mcp/install?name=klip&config=%7B%22type%22%3A%20%22stdio%22%2C%20%22command%22%3A%20%22klip%22%7D) [![Install in VS Code Insiders](https://img.shields.io/badge/VS_Code_Insiders-Install_Server-24bfa5?style=flat-square&logo=visualstudiocode&logoColor=white)](https://insiders.vscode.dev/redirect/mcp/install?name=klip&config=%7B%22type%22%3A%20%22stdio%22%2C%20%22command%22%3A%20%22klip%22%7D&quality=insiders)

### Copilot CLI

1. Start the Copilot CLI by running `copilot`.
2. Run the `/mcp add` command to start adding an MCP server
3. Enter `klip` in the "Name" field.
4. Enter `klip` in the "Command" field.
5. Enter `"*"*` in the "Tools" field.
6. Save by pressing <kbd>Ctrl</kbd>+<kbd>S</kbd>.
7. Check that the tool is working by using the prompt `Copy the current ISO8601 date to the clipboard`.

### Claude Code 

1. Install the MCP server with the `claude mcp add-json` command

```bash
claude mcp add-json klip '{"type": "stdio", "command": "klip"}'
```

2. Start Claude Code by running `claude`.
3. Check that the tool is working by using the prompt `Copy the current ISO8601 date to the clipboard`.

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