//! Output renderers for extracted documents.

use crate::{config::OutputFormat, document::Document, error::Result};

/// Render a document to a string in the selected format.
pub fn render_document(
    document: &Document,
    format: OutputFormat,
    with_metadata: bool,
) -> Result<String> {
    match format {
        OutputFormat::Txt | OutputFormat::Markdown => Ok(render_text(document, with_metadata)),
        OutputFormat::Json => Ok(serde_json::to_string(document)?),
        OutputFormat::Xml => Ok(render_xml(document)),
        OutputFormat::Html => Ok(render_html(document, with_metadata)),
    }
}

fn render_text(document: &Document, with_metadata: bool) -> String {
    let mut out = String::new();
    if with_metadata {
        out.push_str("---\n");
        push_meta(&mut out, "title", document.title.as_deref());
        push_meta(&mut out, "author", document.author.as_deref());
        push_meta(&mut out, "url", document.url.as_deref());
        push_meta(&mut out, "hostname", document.hostname.as_deref());
        push_meta(&mut out, "description", document.description.as_deref());
        push_meta(&mut out, "sitename", document.sitename.as_deref());
        push_meta(&mut out, "date", document.date.as_deref());
        if !document.categories.is_empty() {
            push_meta(
                &mut out,
                "categories",
                Some(&document.categories.join(", ")),
            );
        }
        if !document.tags.is_empty() {
            push_meta(&mut out, "tags", Some(&document.tags.join(", ")));
        }
        push_meta(&mut out, "fingerprint", document.fingerprint.as_deref());
        push_meta(&mut out, "license", document.license.as_deref());
        out.push_str("---\n");
    }
    out.push_str(document.text.trim());
    if let Some(comments) = &document.comments {
        if !comments.trim().is_empty() {
            out.push_str("\n\n");
            out.push_str(comments.trim());
        }
    }
    out
}

fn push_meta(out: &mut String, key: &str, value: Option<&str>) {
    if let Some(value) = value.filter(|v| !v.trim().is_empty()) {
        out.push_str(key);
        out.push_str(": ");
        out.push_str(value);
        out.push('\n');
    }
}

fn render_xml(document: &Document) -> String {
    let mut out = String::from("<doc");
    push_attr(&mut out, "title", document.title.as_deref());
    push_attr(&mut out, "author", document.author.as_deref());
    push_attr(&mut out, "url", document.url.as_deref());
    push_attr(&mut out, "hostname", document.hostname.as_deref());
    push_attr(&mut out, "date", document.date.as_deref());
    push_attr(&mut out, "fingerprint", document.fingerprint.as_deref());
    out.push_str(">\n<main>");
    out.push_str(&escape_xml(&document.text));
    out.push_str("</main>");
    if let Some(comments) = &document.comments {
        out.push_str("\n<comments>");
        out.push_str(&escape_xml(comments));
        out.push_str("</comments>");
    }
    out.push_str("\n</doc>");
    out
}

fn render_html(document: &Document, with_metadata: bool) -> String {
    let mut out = String::from("<!doctype html>\n<html>");
    if with_metadata {
        out.push_str("<head>");
        push_meta_tag(&mut out, "title", document.title.as_deref());
        push_meta_tag(&mut out, "author", document.author.as_deref());
        push_meta_tag(&mut out, "url", document.url.as_deref());
        push_meta_tag(&mut out, "description", document.description.as_deref());
        out.push_str("</head>");
    }
    out.push_str("<body>");
    for para in document.text.split('\n').filter(|p| !p.trim().is_empty()) {
        out.push_str("<p>");
        out.push_str(&escape_xml(para));
        out.push_str("</p>");
    }
    out.push_str("</body></html>");
    out
}

fn push_attr(out: &mut String, key: &str, value: Option<&str>) {
    if let Some(value) = value.filter(|v| !v.trim().is_empty()) {
        out.push(' ');
        out.push_str(key);
        out.push_str("=\"");
        out.push_str(&escape_xml(value));
        out.push('"');
    }
}

fn push_meta_tag(out: &mut String, key: &str, value: Option<&str>) {
    if let Some(value) = value.filter(|v| !v.trim().is_empty()) {
        out.push_str("<meta name=\"");
        out.push_str(key);
        out.push_str("\" content=\"");
        out.push_str(&escape_xml(value));
        out.push_str("\">");
    }
}

fn escape_xml(input: &str) -> String {
    input
        .replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&apos;")
}
