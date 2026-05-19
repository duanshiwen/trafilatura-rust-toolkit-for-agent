# trafilatura-rust-for-mcp

Rust library for extracting main text and metadata from web pages, inspired by [Trafilatura](https://github.com/adbar/trafilatura) and designed for MCP workflows.

> Status: **initial Rust port / public-library scaffold**. The crate implements practical core extraction, metadata extraction, rendering, fingerprinting, and optional downloads. It is not yet a full feature-for-feature replacement for Python Trafilatura.

## Features

- Extract main article text from HTML.
- Remove common boilerplate such as navigation, footer, sidebars, sharing blocks, and related links.
- Extract metadata from:
  - OpenGraph tags
  - Twitter/meta tags
  - canonical links
  - JSON-LD
  - title/headings
- Output formats:
  - plain text
  - Markdown
  - JSON
  - simple XML
  - simple HTML
- MCP-friendly JSON output profile via `ExtractorOptions::for_mcp()`.
- Optional async downloader using `reqwest` + rustls.
- Content fingerprinting with simhash.
- Idiomatic Rust errors via `thiserror`.

## Install

```toml
[dependencies]
trafilatura-rust-for-mcp = "0.1"
```

During local development:

```toml
[dependencies]
trafilatura-rust-for-mcp = { path = "../trafilatura-rust-for-mcp" }
```

## Quick start

```rust
use trafilatura_rust_for_mcp::{extract, ExtractorOptions};

fn main() -> trafilatura_rust_for_mcp::Result<()> {
    let html = r#"
        <html>
          <body>
            <nav>Home About</nav>
            <article>
              <h1>Hello</h1>
              <p>This is the main text.</p>
            </article>
          </body>
        </html>
    "#;

    let text = extract(html, &ExtractorOptions::default())?;
    println!("{text}");
    Ok(())
}
```

## Extract with metadata

```rust
use trafilatura_rust_for_mcp::{extract_with_metadata, ExtractorOptions};

fn main() -> trafilatura_rust_for_mcp::Result<()> {
    let html = r#"
      <html>
        <head>
          <meta property="og:title" content="Example Article">
          <meta name="author" content="Ada Lovelace">
        </head>
        <body><article><p>Main article text.</p></article></body>
      </html>
    "#;

    let doc = extract_with_metadata(html, &ExtractorOptions::default())?;
    assert_eq!(doc.title.as_deref(), Some("Example Article"));
    assert_eq!(doc.author.as_deref(), Some("Ada Lovelace"));
    Ok(())
}
```

## MCP-oriented JSON

```rust
use trafilatura_rust_for_mcp::{extract, ExtractorOptions};

fn main() -> trafilatura_rust_for_mcp::Result<()> {
    let options = ExtractorOptions::for_mcp();
    let json = extract("<article><p>Hello MCP.</p></article>", &options)?;
    println!("{json}");
    Ok(())
}
```

## Download and extract

The `download` feature is enabled by default.

```rust,no_run
use trafilatura_rust_for_mcp::{extract, fetch_url, ExtractorOptions, FetchOptions};

#[tokio::main]
async fn main() -> trafilatura_rust_for_mcp::Result<()> {
    let html = fetch_url("https://example.com", &FetchOptions::default()).await?;
    let text = extract(&html, &ExtractorOptions::default())?;
    println!("{text}");
    Ok(())
}
```

Disable network dependencies:

```toml
trafilatura-rust-for-mcp = { version = "0.1", default-features = false }
```

## Architecture

See [docs/TECHNICAL.md](docs/TECHNICAL.md).

## Usage guide

See [docs/USAGE.md](docs/USAGE.md).

## Roadmap

- Better DOM mutation and cleanup parity with Python Trafilatura.
- More robust date extraction.
- Feed and sitemap discovery modules.
- TEI XML output.
- Benchmark suite against Python Trafilatura fixtures.
- Optional WASM target.
- CLI binary feature.

## License

Apache-2.0. See [LICENSE](LICENSE).

## Acknowledgements

This project is inspired by Adrien Barbaresi’s excellent Python [Trafilatura](https://github.com/adbar/trafilatura) project. The Rust implementation is newly authored and intentionally scoped for idiomatic Rust library usage and MCP integrations.
