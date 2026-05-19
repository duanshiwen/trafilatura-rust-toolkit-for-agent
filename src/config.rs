//! Configuration types for extraction.

use serde::{Deserialize, Serialize};

/// Desired output format.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum OutputFormat {
    /// Plain text output.
    Txt,
    /// Markdown output.
    Markdown,
    /// JSON output.
    Json,
    /// Simplified XML output.
    Xml,
    /// Simplified HTML output.
    Html,
}

impl Default for OutputFormat {
    fn default() -> Self {
        Self::Txt
    }
}

/// Extraction focus mode.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Focus {
    /// Balanced precision and recall.
    Balanced,
    /// Prefer shorter but cleaner content.
    Precision,
    /// Prefer including more possible content.
    Recall,
}

impl Default for Focus {
    fn default() -> Self {
        Self::Balanced
    }
}

/// Options controlling extraction behavior.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExtractorOptions {
    /// Output format for [`crate::extract`].
    pub output_format: OutputFormat,
    /// Precision/recall mode.
    pub focus: Focus,
    /// Source URL, if known.
    pub url: Option<String>,
    /// Include metadata in rendered output where the format supports it.
    pub with_metadata: bool,
    /// Extract comments sections when possible.
    pub include_comments: bool,
    /// Preserve inline formatting for markdown output.
    pub include_formatting: bool,
    /// Preserve link targets in markdown output.
    pub include_links: bool,
    /// Include table text in output.
    pub include_tables: bool,
    /// Include image alt/title placeholders in markdown output.
    pub include_images: bool,
    /// Enable document fingerprint generation.
    pub deduplicate: bool,
    /// Minimum main text size before fallback extraction is attempted.
    pub min_extracted_size: usize,
    /// Minimum output text length.
    pub min_output_size: usize,
    /// Maximum input file size in bytes for download paths.
    pub max_file_size: usize,
}

impl Default for ExtractorOptions {
    fn default() -> Self {
        Self {
            output_format: OutputFormat::Txt,
            focus: Focus::Balanced,
            url: None,
            with_metadata: false,
            include_comments: true,
            include_formatting: false,
            include_links: false,
            include_tables: true,
            include_images: false,
            deduplicate: false,
            min_extracted_size: 250,
            min_output_size: 1,
            max_file_size: 20_000_000,
        }
    }
}

impl ExtractorOptions {
    /// Create options for JSON output with metadata enabled.
    pub fn json_with_metadata() -> Self {
        Self {
            output_format: OutputFormat::Json,
            with_metadata: true,
            ..Self::default()
        }
    }

    /// Create options optimized for MCP text ingestion.
    pub fn for_mcp() -> Self {
        Self {
            output_format: OutputFormat::Json,
            focus: Focus::Balanced,
            with_metadata: true,
            include_comments: false,
            include_links: true,
            include_formatting: true,
            ..Self::default()
        }
    }
}
