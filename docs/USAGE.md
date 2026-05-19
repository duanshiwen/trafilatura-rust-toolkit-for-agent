# Usage Guide

## Basic extraction

```rust
use trafilatura_rust_for_mcp::{extract, ExtractorOptions};

let html = "<article><p>Hello world.</p></article>";
let text = extract(html, &ExtractorOptions::default())?;
```

## Configure output format

```rust
use trafilatura_rust_for_mcp::{extract, ExtractorOptions, OutputFormat};

let options = ExtractorOptions {
    output_format: OutputFormat::Markdown,
    include_formatting: true,
    ..ExtractorOptions::default()
};

let markdown = extract(html, &options)?;
```

## JSON output for MCP

```rust
use trafilatura_rust_for_mcp::{extract, ExtractorOptions};

let options = ExtractorOptions::for_mcp();
let json = extract(html, &options)?;
```

`ExtractorOptions::for_mcp()` sets:

- JSON output
- metadata enabled
- comments disabled
- links enabled
- formatting enabled

This is useful when a tool needs to pass structured article content into an MCP pipeline or LLM context.

## Extract metadata object

```rust
use trafilatura_rust_for_mcp::{extract_with_metadata, ExtractorOptions};

let doc = extract_with_metadata(html, &ExtractorOptions::default())?;
println!("Title: {:?}", doc.title);
println!("Text: {}", doc.text);
```

## Source URL

Pass a source URL to improve hostname/canonical metadata:

```rust
use trafilatura_rust_for_mcp::ExtractorOptions;

let options = ExtractorOptions {
    url: Some("https://example.com/article".to_string()),
    with_metadata: true,
    ..ExtractorOptions::default()
};
```

## Precision vs recall

```rust
use trafilatura_rust_for_mcp::{ExtractorOptions, Focus};

let precision = ExtractorOptions {
    focus: Focus::Precision,
    ..ExtractorOptions::default()
};

let recall = ExtractorOptions {
    focus: Focus::Recall,
    ..ExtractorOptions::default()
};
```

- `Precision`: more aggressive boilerplate filtering.
- `Balanced`: default.
- `Recall`: tolerate more candidate content.

## Downloads

With default features:

```rust,no_run
use trafilatura_rust_for_mcp::{fetch_url, FetchOptions};

#[tokio::main]
async fn main() -> trafilatura_rust_for_mcp::Result<()> {
    let html = fetch_url("https://example.com", &FetchOptions::default()).await?;
    println!("{}", html);
    Ok(())
}
```

Without network support:

```toml
trafilatura-rust-for-mcp = { version = "0.1", default-features = false }
```

## Fingerprints

Enable `deduplicate` to calculate a simhash fingerprint:

```rust
let options = ExtractorOptions {
    deduplicate: true,
    with_metadata: true,
    ..ExtractorOptions::default()
};
let doc = extract_with_metadata(html, &options)?;
println!("{:?}", doc.fingerprint);
```

## Error handling

All fallible APIs return `Result<T, TrafilaturaError>`:

```rust
match extract(html, &ExtractorOptions::default()) {
    Ok(text) => println!("{text}"),
    Err(err) => eprintln!("extraction failed: {err}"),
}
```

## Recommended MCP integration pattern

1. Fetch or receive HTML.
2. Call `extract_with_metadata` or `extract` with `ExtractorOptions::for_mcp()`.
3. Store the resulting JSON in your MCP tool response.
4. Keep original URL in `ExtractorOptions::url`.
5. Use `deduplicate` when building corpora or crawl pipelines.
