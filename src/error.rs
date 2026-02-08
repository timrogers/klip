use thiserror::Error;

/// Errors that can occur when working with the clipboard
#[derive(Debug, Error)]
pub enum ClipboardError {
    #[error("Failed to initialize clipboard: {0}")]
    InitializationFailed(String),

    #[error("Failed to copy to clipboard: {0}")]
    CopyFailed(String),

    #[error("Failed to read from clipboard: {0}")]
    ReadFailed(String),

    #[error("Clipboard contains non-text data")]
    NonTextData,

    #[error("Clipboard is empty")]
    Empty,
}
