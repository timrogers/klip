# Getting Started with klip Development

This guide will help you get started implementing the klip MCP server. Before diving into code, please read the [IMPLEMENTATION_PLAN.md](IMPLEMENTATION_PLAN.md) for comprehensive technical details.

## Prerequisites

### Required Tools
- **Rust** (1.70 or later): Install from [rustup.rs](https://rustup.rs/)
- **Git**: For version control
- **Text Editor/IDE**: VS Code with rust-analyzer recommended

### Platform-Specific Requirements

**Linux:**
```bash
# Ubuntu/Debian
sudo apt-get install libxcb-shape0-dev libxcb-xfixes0-dev

# Fedora
sudo dnf install libxcb-devel
```

**macOS:**
```bash
# Install Xcode Command Line Tools
xcode-select --install
```

**Windows:**
- Visual Studio 2019 or later with C++ tools
- Or: Windows Build Tools

## Quick Start

### 1. Initialize the Project

```bash
# Navigate to the repository
cd klip

# Initialize a new Rust project (this will create src/ and Cargo.toml)
cargo init --name klip

# Verify it builds
cargo build
cargo run
```

### 2. Add Core Dependencies

Edit `Cargo.toml` to add the essential dependencies:

```toml
[package]
name = "klip"
version = "0.1.0"
edition = "2021"

[dependencies]
# Async runtime
tokio = { version = "1.35", features = ["full"] }

# Serialization
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"

# Clipboard
arboard = "3.3"

# Error handling
thiserror = "1.0"
anyhow = "1.0"

# Logging
tracing = "0.1"
tracing-subscriber = { version = "0.3", features = ["env-filter"] }

[dev-dependencies]
tokio-test = "0.4"
```

### 3. Create the Project Structure

```bash
# Create source files
mkdir -p src
touch src/main.rs
touch src/server.rs
touch src/clipboard.rs
touch src/tools.rs
touch src/error.rs
touch src/config.rs

# Create test directory
mkdir -p tests
touch tests/clipboard_tests.rs
touch tests/integration_tests.rs
```

### 4. Implement Basic Clipboard Module

Start with `src/clipboard.rs`:

```rust
use arboard::Clipboard;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ClipboardError {
    #[error("Failed to initialize clipboard: {0}")]
    InitializationFailed(String),
    
    #[error("Failed to copy to clipboard: {0}")]
    CopyFailed(String),
}

pub struct ClipboardManager {
    clipboard: Clipboard,
}

impl ClipboardManager {
    pub fn new() -> Result<Self, ClipboardError> {
        let clipboard = Clipboard::new()
            .map_err(|e| ClipboardError::InitializationFailed(e.to_string()))?;
        
        Ok(Self { clipboard })
    }
    
    pub fn copy(&mut self, text: &str) -> Result<(), ClipboardError> {
        self.clipboard
            .set_text(text)
            .map_err(|e| ClipboardError::CopyFailed(e.to_string()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_clipboard_initialization() {
        let result = ClipboardManager::new();
        assert!(result.is_ok());
    }

    #[test]
    fn test_copy_text() {
        let mut manager = ClipboardManager::new().unwrap();
        let result = manager.copy("Hello, klip!");
        assert!(result.is_ok());
    }
}
```

### 5. Verify the Setup

```bash
# Run tests
cargo test

# Check for warnings
cargo clippy

# Format code
cargo fmt

# Build in release mode
cargo build --release
```

## Development Workflow

### Daily Development

1. **Write code** following the implementation plan phases
2. **Write tests** for new functionality
3. **Run tests** frequently: `cargo test`
4. **Check code quality**: `cargo clippy`
5. **Format code**: `cargo fmt`
6. **Commit changes** with clear messages

### Testing Across Platforms

Use GitHub Actions for automated cross-platform testing. Create `.github/workflows/ci.yml`:

```yaml
name: CI

on: [push, pull_request]

jobs:
  test:
    name: Test on ${{ matrix.os }}
    runs-on: ${{ matrix.os }}
    strategy:
      matrix:
        os: [ubuntu-latest, macos-latest, windows-latest]
    steps:
      - uses: actions/checkout@v4
      - uses: dtolnay/rust-toolchain@stable
      - name: Run tests
        run: cargo test --verbose
      - name: Run clippy
        run: cargo clippy -- -D warnings
```

## Common Issues and Solutions

### Issue: Clipboard initialization fails on Linux
**Solution:** Install required X11 development libraries:
```bash
sudo apt-get install libxcb-shape0-dev libxcb-xfixes0-dev
```

### Issue: Async/await errors
**Solution:** Ensure you're using the tokio runtime properly:
```rust
#[tokio::main]
async fn main() {
    // Your code here
}
```

### Issue: "cannot find macro `println` in this scope" in tests
**Solution:** Import the standard library:
```rust
#[cfg(test)]
mod tests {
    use super::*;
}
```

## Next Steps

After completing the basic setup:

1. **Read the Implementation Plan**: Review [IMPLEMENTATION_PLAN.md](IMPLEMENTATION_PLAN.md) Phase 1 in detail
2. **Implement Error Types**: Complete `src/error.rs` with comprehensive error handling
3. **Build the MCP Server**: Move to Phase 2 and implement `src/server.rs`
4. **Add Tool Definitions**: Implement the `copy_to_clipboard` tool in `src/tools.rs`
5. **Write Tests**: Create comprehensive test coverage
6. **Document**: Add rustdoc comments to all public APIs

## Resources

- **Implementation Plan**: [IMPLEMENTATION_PLAN.md](IMPLEMENTATION_PLAN.md)
- **Rust Book**: https://doc.rust-lang.org/book/
- **Tokio Tutorial**: https://tokio.rs/tokio/tutorial
- **arboard Docs**: https://docs.rs/arboard/
- **MCP Specification**: https://modelcontextprotocol.io/

## Getting Help

- Open an issue on GitHub for bugs or questions
- Check existing issues for common problems
- Refer to the implementation plan for architectural guidance

---

**Ready to start?** Begin with Phase 1 in the [IMPLEMENTATION_PLAN.md](IMPLEMENTATION_PLAN.md)!
