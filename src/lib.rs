//! Rust library for extracting main text and metadata from web pages.
//!
//! This crate is inspired by the Python project Trafilatura, but it is an idiomatic
//! Rust implementation intended for public library use and MCP-style workflows.
//!
//! # Quick start
//!
//! ```rust
//! use trafilatura_rust_for_mcp::{extract, ExtractorOptions};
//!
//! let html = r#"<html><body><article><h1>Hello</h1><p>Main text.</p></article></body></html>"#;
//! let text = extract(html, &ExtractorOptions::default()).unwrap();
//! assert!(text.contains("Main text."));
//! ```

pub mod config;
pub mod dedup;
pub mod document;
pub mod error;
pub mod extractor;
pub mod metadata;
pub mod output;
pub mod text;

#[cfg(feature = "download")]
pub mod download;

pub use config::{ExtractorOptions, Focus, OutputFormat};
pub use document::Document;
pub use error::{Result, TrafilaturaError};
pub use extractor::{extract, extract_with_metadata, Extractor};

#[cfg(feature = "download")]
pub use download::{fetch_url, FetchOptions};
