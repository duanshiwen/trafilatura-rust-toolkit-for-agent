//! Basic extraction example.

use trafilatura_rust_for_mcp::{extract_with_metadata, ExtractorOptions};

fn main() -> trafilatura_rust_for_mcp::Result<()> {
    let html = r#"
        <html>
          <head>
            <meta property="og:title" content="Example Article">
            <meta name="author" content="Ada Lovelace">
          </head>
          <body>
            <nav>Home About Contact</nav>
            <article>
              <h1>Example Article</h1>
              <p>This is the main article text.</p>
              <p>This is another paragraph worth keeping.</p>
            </article>
            <footer>Copyright and links</footer>
          </body>
        </html>
    "#;

    let options = ExtractorOptions {
        with_metadata: true,
        deduplicate: true,
        ..ExtractorOptions::default()
    };
    let doc = extract_with_metadata(html, &options)?;

    println!("Title: {:?}", doc.title);
    println!("Author: {:?}", doc.author);
    println!("Fingerprint: {:?}", doc.fingerprint);
    println!("\n{}", doc.text);
    Ok(())
}
