//! Optional HTTP download support.
//!
//! This module is enabled by the `download` feature, which is part of default features.

use std::time::Duration;

use crate::{
    config::ExtractorOptions,
    error::{Result, TrafilaturaError},
};

/// Download configuration.
#[derive(Debug, Clone)]
pub struct FetchOptions {
    /// Request timeout.
    pub timeout: Duration,
    /// User-Agent header.
    pub user_agent: String,
    /// Maximum accepted response size in bytes.
    pub max_size: usize,
}

impl Default for FetchOptions {
    fn default() -> Self {
        Self {
            timeout: Duration::from_secs(30),
            user_agent: format!("trafilatura-rust-for-mcp/{} (+https://github.com/your-org/trafilatura-rust-for-mcp)", env!("CARGO_PKG_VERSION")),
            max_size: ExtractorOptions::default().max_file_size,
        }
    }
}

/// Fetch a URL and return decoded HTML/text.
///
/// # Errors
/// Returns an error when the request fails, status is not successful, or the
/// response exceeds [`FetchOptions::max_size`].
pub async fn fetch_url(url: &str, options: &FetchOptions) -> Result<String> {
    let client = reqwest::Client::builder()
        .timeout(options.timeout)
        .user_agent(&options.user_agent)
        .build()
        .map_err(|err| TrafilaturaError::Download(err.to_string()))?;

    let response = client
        .get(url)
        .send()
        .await
        .map_err(|err| TrafilaturaError::Download(err.to_string()))?;

    if !response.status().is_success() {
        return Err(TrafilaturaError::Download(format!(
            "HTTP status {}",
            response.status()
        )));
    }

    if let Some(length) = response.content_length() {
        if length as usize > options.max_size {
            return Err(TrafilaturaError::Download(format!(
                "response too large: {length} bytes"
            )));
        }
    }

    let bytes = response
        .bytes()
        .await
        .map_err(|err| TrafilaturaError::Download(err.to_string()))?;
    if bytes.len() > options.max_size {
        return Err(TrafilaturaError::Download(format!(
            "response too large: {} bytes",
            bytes.len()
        )));
    }

    String::from_utf8(bytes.to_vec()).map_err(|err| TrafilaturaError::Download(err.to_string()))
}
