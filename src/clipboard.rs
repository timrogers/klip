use crate::error::ClipboardError;
use arboard::Clipboard;

/// Manages clipboard operations across platforms
pub struct ClipboardManager {
    clipboard: Clipboard,
}

impl ClipboardManager {
    /// Creates a new clipboard manager
    pub fn new() -> Result<Self, ClipboardError> {
        let clipboard = Clipboard::new()
            .map_err(|e| ClipboardError::InitializationFailed(e.to_string()))?;

        Ok(Self { clipboard })
    }

    /// Copies text to the system clipboard
    pub fn copy(&mut self, text: &str) -> Result<(), ClipboardError> {
        self.clipboard
            .set_text(text)
            .map_err(|e| ClipboardError::CopyFailed(e.to_string()))
    }

    /// Gets current clipboard content
    pub fn get(&mut self) -> Result<String, ClipboardError> {
        self.clipboard
            .get_text()
            .map_err(|e| ClipboardError::ReadFailed(e.to_string()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Note: These tests require a clipboard (X11/Wayland on Linux, native on Windows/macOS)
    // They will be skipped in headless CI environments

    #[test]
    fn test_clipboard_initialization() {
        let result = ClipboardManager::new();
        // If clipboard is unavailable (headless CI), that's acceptable
        if result.is_err() {
            eprintln!("Clipboard not available in test environment");
        }
    }

    #[test]
    fn test_copy_and_read_text() {
        let mut manager = match ClipboardManager::new() {
            Ok(m) => m,
            Err(e) => {
                eprintln!("Skipping test - clipboard not available: {}", e);
                return;
            }
        };
        
        let test_text = "Hello, klip!";
        
        // Copy text
        let copy_result = manager.copy(test_text);
        assert!(copy_result.is_ok());
        
        // Read back
        let read_result = manager.get();
        assert!(read_result.is_ok());
        assert_eq!(read_result.unwrap(), test_text);
    }

    #[test]
    fn test_copy_unicode() {
        let mut manager = match ClipboardManager::new() {
            Ok(m) => m,
            Err(e) => {
                eprintln!("Skipping test - clipboard not available: {}", e);
                return;
            }
        };
        
        let test_text = "Hello 世界 🌍";
        
        let result = manager.copy(test_text);
        assert!(result.is_ok());
        
        let read_result = manager.get();
        assert!(read_result.is_ok());
        assert_eq!(read_result.unwrap(), test_text);
    }

    #[test]
    fn test_copy_empty_string() {
        let mut manager = match ClipboardManager::new() {
            Ok(m) => m,
            Err(e) => {
                eprintln!("Skipping test - clipboard not available: {}", e);
                return;
            }
        };
        
        let result = manager.copy("");
        assert!(result.is_ok());
    }
}
