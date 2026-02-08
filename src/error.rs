use thiserror::Error;

/// Errors that can occur when working with the clipboard
#[derive(Debug, Error)]
pub enum ClipboardError {
    #[error("Failed to initialize clipboard: {0}")]
    InitializationFailed(String),

    #[error("Failed to copy to clipboard: {0}")]
    CopyFailed(String),

    #[allow(dead_code)]
    #[error("Failed to read from clipboard: {0}")]
    ReadFailed(String),

    #[allow(dead_code)]
    #[error("Clipboard contains non-text data")]
    NonTextData,

    #[allow(dead_code)]
    #[error("Clipboard is empty")]
    Empty,
}
