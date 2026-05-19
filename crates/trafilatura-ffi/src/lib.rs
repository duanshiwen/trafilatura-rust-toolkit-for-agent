//! C ABI bridge for Swift/macOS clients.
//!
//! This crate intentionally keeps all raw pointer handling at the FFI boundary
//! and delegates extraction work to `trafilatura-rust-for-mcp`.

use libc::c_char;
use serde::Deserialize;
use std::ffi::{CStr, CString};
use std::panic::{catch_unwind, AssertUnwindSafe};
use std::ptr;
use trafilatura_rust_for_mcp::{extract, ExtractorOptions, OutputFormat};

/// FFI-safe result object returned to Swift/C callers.
///
/// On success, `data` is non-null and `error` is null.
/// On failure, `data` is null and `error` is non-null.
/// Both pointers must be released with [`trafilatura_free_result`].
#[repr(C)]
pub struct TrafilaturaResult {
    /// UTF-8 output string allocated by Rust.
    pub data: *mut c_char,
    /// UTF-8 error string allocated by Rust.
    pub error: *mut c_char,
}

#[derive(Debug, Default, Deserialize)]
struct OptionsJson {
    format: Option<String>,
    url: Option<String>,
    include_links: Option<bool>,
    include_comments: Option<bool>,
    include_formatting: Option<bool>,
    include_tables: Option<bool>,
    include_images: Option<bool>,
    deduplicate: Option<bool>,
    with_metadata: Option<bool>,
}

/// Extract plain text from an HTML string.
///
/// # Safety
///
/// `html` must be a valid, non-null, NUL-terminated UTF-8 C string.
/// The returned result must be released with [`trafilatura_free_result`].
#[no_mangle]
pub unsafe extern "C" fn trafilatura_extract_text(html: *const c_char) -> TrafilaturaResult {
    run_extraction(html, ExtractorOptions::default())
}

/// Extract MCP-friendly JSON from an HTML string.
///
/// # Safety
///
/// `html` must be a valid, non-null, NUL-terminated UTF-8 C string.
/// The returned result must be released with [`trafilatura_free_result`].
#[no_mangle]
pub unsafe extern "C" fn trafilatura_extract_json_for_mcp(
    html: *const c_char,
) -> TrafilaturaResult {
    run_extraction(html, ExtractorOptions::for_mcp())
}

/// Extract content from HTML using options encoded as JSON.
///
/// Supported JSON keys include `format`, `url`, `include_links`,
/// `include_comments`, `include_formatting`, `include_tables`,
/// `include_images`, `deduplicate`, and `with_metadata`.
///
/// # Safety
///
/// `html` and `options_json` must be valid, non-null, NUL-terminated UTF-8 C strings.
/// The returned result must be released with [`trafilatura_free_result`].
#[no_mangle]
pub unsafe extern "C" fn trafilatura_extract_with_options_json(
    html: *const c_char,
    options_json: *const c_char,
) -> TrafilaturaResult {
    let options = match parse_options(options_json) {
        Ok(options) => options,
        Err(error) => return TrafilaturaResult::err(error),
    };
    run_extraction(html, options)
}

/// Free a string allocated by this library.
///
/// # Safety
///
/// `ptr` must be null or a pointer previously returned by this library.
#[no_mangle]
pub unsafe extern "C" fn trafilatura_free_string(ptr: *mut c_char) {
    if !ptr.is_null() {
        drop(unsafe { CString::from_raw(ptr) });
    }
}

/// Free all strings in a [`TrafilaturaResult`].
///
/// # Safety
///
/// `result` must be a value returned by this library.
#[no_mangle]
pub unsafe extern "C" fn trafilatura_free_result(result: TrafilaturaResult) {
    unsafe {
        trafilatura_free_string(result.data);
        trafilatura_free_string(result.error);
    }
}

fn run_extraction(html: *const c_char, options: ExtractorOptions) -> TrafilaturaResult {
    let result = catch_unwind(AssertUnwindSafe(|| {
        let html = unsafe { c_str_to_str(html) }?;
        extract(html, &options).map_err(|error| error.to_string())
    }));

    match result {
        Ok(Ok(data)) => TrafilaturaResult::ok(data),
        Ok(Err(error)) => TrafilaturaResult::err(error),
        Err(_) => TrafilaturaResult::err("Rust extraction panicked"),
    }
}

fn parse_options(options_json: *const c_char) -> Result<ExtractorOptions, String> {
    let raw = unsafe { c_str_to_str(options_json) }?;
    let parsed: OptionsJson = serde_json::from_str(raw).map_err(|error| error.to_string())?;
    let mut options = ExtractorOptions::default();

    if let Some(format) = parsed.format {
        options.output_format = parse_output_format(&format)?;
        options.with_metadata = matches!(options.output_format, OutputFormat::Json)
            || parsed.with_metadata.unwrap_or(options.with_metadata);
    }
    if let Some(url) = parsed.url {
        options.url = Some(url);
    }
    if let Some(value) = parsed.include_links {
        options.include_links = value;
    }
    if let Some(value) = parsed.include_comments {
        options.include_comments = value;
    }
    if let Some(value) = parsed.include_formatting {
        options.include_formatting = value;
    }
    if let Some(value) = parsed.include_tables {
        options.include_tables = value;
    }
    if let Some(value) = parsed.include_images {
        options.include_images = value;
    }
    if let Some(value) = parsed.deduplicate {
        options.deduplicate = value;
    }
    if let Some(value) = parsed.with_metadata {
        options.with_metadata = value;
    }

    Ok(options)
}

fn parse_output_format(value: &str) -> Result<OutputFormat, String> {
    match value.to_ascii_lowercase().as_str() {
        "text" | "txt" => Ok(OutputFormat::Txt),
        "markdown" | "md" => Ok(OutputFormat::Markdown),
        "json" => Ok(OutputFormat::Json),
        "xml" => Ok(OutputFormat::Xml),
        "html" => Ok(OutputFormat::Html),
        other => Err(format!("unsupported output format: {other}")),
    }
}

unsafe fn c_str_to_str<'a>(value: *const c_char) -> Result<&'a str, String> {
    if value.is_null() {
        return Err("received null C string".to_string());
    }
    unsafe { CStr::from_ptr(value) }
        .to_str()
        .map_err(|error| error.to_string())
}

impl TrafilaturaResult {
    fn ok(data: String) -> Self {
        Self {
            data: string_to_ptr(data),
            error: ptr::null_mut(),
        }
    }

    fn err(error: impl Into<String>) -> Self {
        Self {
            data: ptr::null_mut(),
            error: string_to_ptr(error.into()),
        }
    }
}

fn string_to_ptr(value: String) -> *mut c_char {
    match CString::new(value) {
        Ok(value) => value.into_raw(),
        Err(_) => ptr::null_mut(),
    }
}
