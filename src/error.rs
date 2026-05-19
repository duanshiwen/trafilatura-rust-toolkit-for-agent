//! Error types used by the extraction library.

/// Convenience result type used throughout this crate.
pub type Result<T> = std::result::Result<T, TrafilaturaError>;

/// Errors returned by this crate.
#[derive(Debug, thiserror::Error)]
pub enum TrafilaturaError {
    /// The provided HTML document could not be parsed or contained no useful content.
    #[error("HTML parsing or extraction failed: {0}")]
    Extraction(String),

    /// The requested output format is unsupported in the current context.
    #[error("unsupported output format: {0}")]
    UnsupportedFormat(String),

    /// URL parsing failed.
    #[error("invalid URL: {0}")]
    InvalidUrl(String),

    /// Network download failed.
    #[cfg(feature = "download")]
    #[error("download failed: {0}")]
    Download(String),

    /// JSON serialization failed.
    #[error("serialization failed: {0}")]
    Serialization(#[from] serde_json::Error),
}
