# Testing klip

This document describes how to test the klip MCP server.

## Manual Testing

### Build and Run

```bash
# Build the server
cargo build --release

# Run the server (it will wait for MCP protocol messages on stdin)
./target/release/klip
```

The server communicates via stdin/stdout using the MCP protocol. It will:
1. Log startup message to stderr
2. Wait for MCP protocol messages on stdin
3. Respond with MCP protocol messages on stdout

### Testing with Claude Desktop

1. Build the release binary:
   ```bash
   cargo build --release
   ```

2. Add to your Claude Desktop config (`~/Library/Application Support/Claude/claude_desktop_config.json` on macOS):
   ```json
   {
     "mcpServers": {
       "klip": {
         "command": "/absolute/path/to/klip/target/release/klip"
       }
     }
   }
   ```

3. Restart Claude Desktop

4. Ask Claude to copy something to your clipboard:
   ```
   Please copy "Hello from klip!" to my clipboard
   ```

5. Paste (Cmd+V / Ctrl+V) to verify it worked!

## Automated Testing

### Unit Tests

```bash
# Run all tests
cargo test

# Run with output
cargo test -- --nocapture

# Run specific test
cargo test test_clipboard_initialization
```

Note: Tests gracefully handle headless environments where clipboard is unavailable.

### Integration Testing

The CI/CD pipeline automatically:
- Builds on all platforms (Linux, macOS, Windows)
- Runs tests on all platforms
- Creates release binaries
- Tests cross-compilation

## Troubleshooting

### Linux: Clipboard not working

Make sure you have X11 or Wayland running. In headless environments (SSH, Docker), clipboard access won't work.

For X11, ensure `DISPLAY` is set:
```bash
echo $DISPLAY
```

### macOS: Permission denied

Make sure the binary has execute permissions:
```bash
chmod +x target/release/klip
```

### Windows: Binary not found

Use the full path to the binary including `.exe`:
```
C:\path\to\klip\target\release\klip.exe
```

## Manual MCP Protocol Testing

You can test the server manually by sending MCP protocol messages. Here's an example:

```bash
# Start the server
./target/release/klip
```

Then send (via stdin):
```json
{
  "jsonrpc": "2.0",
  "id": 1,
  "method": "initialize",
  "params": {
    "protocolVersion": "2024-11-05",
    "capabilities": {},
    "clientInfo": {
      "name": "test-client",
      "version": "1.0.0"
    }
  }
}
```

The server will respond with its capabilities including the `copy_to_clipboard` tool.

To test the tool:
```json
{
  "jsonrpc": "2.0",
  "id": 2,
  "method": "tools/call",
  "params": {
    "name": "copy_to_clipboard",
    "arguments": {
      "text": "Test message"
    }
  }
}
```

The server will copy the text to your clipboard and respond with success.
