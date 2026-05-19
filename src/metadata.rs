//! Metadata extraction from HTML documents.

use once_cell::sync::Lazy;
use regex::Regex;
use scraper::{Html, Selector};
use serde_json::Value;
use url::Url;

use crate::{document::Document, text};

static TITLE_SPLIT: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"\s+[–•·—|⁄*⋆~‹«<›»>:-]\s+").expect("valid regex"));
static DATE_RE: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"\b\d{4}-\d{2}-\d{2}\b").expect("valid regex"));

/// Extract document metadata from parsed HTML.
pub fn extract_metadata(html: &Html, default_url: Option<&str>) -> Document {
    let mut doc = Document {
        url: default_url.map(ToOwned::to_owned),
        ..Document::default()
    };

    extract_meta_tags(html, &mut doc);
    extract_json_ld(html, &mut doc);

    if doc.title.is_none() {
        doc.title = extract_title(html);
    }
    if doc.url.is_none() {
        doc.url = extract_canonical_url(html, default_url);
    }
    if let Some(url) = &doc.url {
        doc.hostname = Url::parse(url)
            .ok()
            .and_then(|parsed| parsed.host_str().map(ToOwned::to_owned));
    }
    if doc.sitename.is_none() {
        doc.sitename = extract_sitename(html, doc.url.as_deref());
    }
    if doc.date.is_none() {
        doc.date = extract_date_hint(html);
    }
    doc.clean_and_trim();
    doc
}

fn extract_meta_tags(html: &Html, doc: &mut Document) {
    let Ok(selector) = Selector::parse("head meta[content]") else {
        return;
    };
    for element in html.select(&selector) {
        let value = element.value();
        let content = value
            .attr("content")
            .map(text::normalize_spaces)
            .filter(|s| !s.is_empty());
        let Some(content) = content else { continue };
        let key = value
            .attr("property")
            .or_else(|| value.attr("name"))
            .or_else(|| value.attr("itemprop"))
            .unwrap_or("")
            .to_ascii_lowercase();

        match key.as_str() {
            "og:title" | "twitter:title" | "title" | "headline" | "dc.title" | "dcterms.title" => {
                doc.title.get_or_insert(content);
            }
            "og:description"
            | "twitter:description"
            | "description"
            | "dc.description"
            | "dcterms.description" => {
                doc.description.get_or_insert(content);
            }
            "og:site_name" | "application-name" | "twitter:site" | "publisher" | "dc.publisher" => {
                doc.sitename
                    .get_or_insert(content.trim_start_matches('@').to_owned());
            }
            "og:url" | "twitter:url" => {
                doc.url.get_or_insert(content);
            }
            "og:image"
            | "og:image:url"
            | "og:image:secure_url"
            | "twitter:image"
            | "twitter:image:src"
            | "image" => {
                doc.image.get_or_insert(content);
            }
            "og:type" => {
                doc.pagetype.get_or_insert(content);
            }
            "author" | "article:author" | "citation_author" | "dc.creator" | "dc:creator"
            | "creator" => {
                doc.author = merge_authors(doc.author.take(), &content);
            }
            "keywords" | "citation_keywords" | "parsely-tags" | "tags" => {
                doc.tags.extend(text::split_meta_values(&content));
            }
            "article:section" | "category" => {
                doc.categories.extend(text::split_meta_values(&content));
            }
            "article:published_time" | "date" | "dc.date" | "pubdate" | "publishdate" => {
                doc.date.get_or_insert(content.chars().take(10).collect());
            }
            _ => {}
        }
    }
}

fn extract_json_ld(html: &Html, doc: &mut Document) {
    let Ok(selector) = Selector::parse(r#"script[type="application/ld+json"]"#) else {
        return;
    };
    for element in html.select(&selector) {
        let json_text = element.text().collect::<String>();
        let Ok(value) = serde_json::from_str::<Value>(&json_text) else {
            continue;
        };
        visit_json_ld(&value, doc);
    }
}

fn visit_json_ld(value: &Value, doc: &mut Document) {
    match value {
        Value::Array(values) => values.iter().for_each(|v| visit_json_ld(v, doc)),
        Value::Object(map) => {
            if let Some(graph) = map.get("@graph") {
                visit_json_ld(graph, doc);
            }
            if doc.title.is_none() {
                doc.title =
                    json_string(map.get("headline")).or_else(|| json_string(map.get("name")));
            }
            if doc.description.is_none() {
                doc.description = json_string(map.get("description"));
            }
            if doc.date.is_none() {
                doc.date = json_string(map.get("datePublished"))
                    .or_else(|| json_string(map.get("dateCreated")))
                    .map(|d| d.chars().take(10).collect());
            }
            if doc.pagetype.is_none() {
                doc.pagetype = json_string(map.get("@type")).map(|s| s.to_ascii_lowercase());
            }
            if let Some(author) = map.get("author") {
                extract_json_author(author, doc);
            }
            if let Some(publisher) = map.get("publisher") {
                extract_json_publisher(publisher, doc);
            }
            if doc.categories.is_empty() {
                if let Some(section) = map.get("articleSection") {
                    match section {
                        Value::String(s) => doc.categories.push(s.clone()),
                        Value::Array(values) => {
                            doc.categories
                                .extend(values.iter().filter_map(|v| json_string(Some(v))));
                        }
                        _ => {}
                    }
                }
            }
            if doc.image.is_none() {
                doc.image = json_string(map.get("image"));
            }
        }
        _ => {}
    }
}

fn extract_json_author(value: &Value, doc: &mut Document) {
    match value {
        Value::String(name) => doc.author = merge_authors(doc.author.take(), name),
        Value::Array(values) => values.iter().for_each(|v| extract_json_author(v, doc)),
        Value::Object(map) => {
            if let Some(name) = json_string(map.get("name")) {
                doc.author = merge_authors(doc.author.take(), &name);
            }
        }
        _ => {}
    }
}

fn extract_json_publisher(value: &Value, doc: &mut Document) {
    if doc.sitename.is_some() {
        return;
    }
    match value {
        Value::String(name) => doc.sitename = Some(name.clone()),
        Value::Object(map) => doc.sitename = json_string(map.get("name")),
        _ => {}
    }
}

fn json_string(value: Option<&Value>) -> Option<String> {
    match value? {
        Value::String(s) => Some(text::normalize_spaces(s)),
        Value::Array(values) => values.iter().find_map(|v| json_string(Some(v))),
        Value::Object(map) => json_string(map.get("url")).or_else(|| json_string(map.get("name"))),
        _ => None,
    }
    .filter(|s| !s.is_empty())
}

fn extract_title(html: &Html) -> Option<String> {
    for selector_str in ["h1", "[class*=headline]", "[class*=title]", "title"] {
        let Ok(selector) = Selector::parse(selector_str) else {
            continue;
        };
        if let Some(element) = html.select(&selector).next() {
            let title = text::normalize_spaces(&element.text().collect::<Vec<_>>().join(" "));
            if !title.is_empty() {
                if selector_str == "title" {
                    return TITLE_SPLIT.split(&title).next().map(text::normalize_spaces);
                }
                return Some(title);
            }
        }
    }
    None
}

fn extract_canonical_url(html: &Html, default_url: Option<&str>) -> Option<String> {
    for selector in [
        r#"link[rel="canonical"]"#,
        r#"link[rel="alternate"][hreflang="x-default"]"#,
        "base[href]",
    ] {
        let Ok(selector) = Selector::parse(selector) else {
            continue;
        };
        if let Some(element) = html.select(&selector).next() {
            if let Some(href) = element.value().attr("href") {
                return absolutize_url(href, default_url).or_else(|| Some(href.to_owned()));
            }
        }
    }
    default_url.map(ToOwned::to_owned)
}

fn extract_sitename(html: &Html, default_url: Option<&str>) -> Option<String> {
    if let Some(title) = extract_title_from_tag(html) {
        let parts: Vec<_> = TITLE_SPLIT
            .split(&title)
            .map(text::normalize_spaces)
            .collect();
        if let Some(site) = parts
            .into_iter()
            .find(|part| part.contains('.') || part.len() <= 40)
        {
            return Some(site);
        }
    }
    default_url
        .and_then(|u| Url::parse(u).ok())
        .and_then(|u| u.host_str().map(ToOwned::to_owned))
}

fn extract_title_from_tag(html: &Html) -> Option<String> {
    let Ok(selector) = Selector::parse("title") else {
        return None;
    };
    html.select(&selector)
        .next()
        .map(|e| text::normalize_spaces(&e.text().collect::<Vec<_>>().join(" ")))
        .filter(|s| !s.is_empty())
}

fn extract_date_hint(html: &Html) -> Option<String> {
    let html_text = html.root_element().html();
    DATE_RE.find(&html_text).map(|m| m.as_str().to_owned())
}

fn absolutize_url(href: &str, base: Option<&str>) -> Option<String> {
    if Url::parse(href).is_ok() {
        return Some(href.to_owned());
    }
    base.and_then(|b| Url::parse(b).ok())
        .and_then(|base_url| base_url.join(href).ok())
        .map(|u| u.to_string())
}

fn merge_authors(current: Option<String>, new_author: &str) -> Option<String> {
    let cleaned = normalize_author(new_author)?;
    let mut authors = current
        .map(|s| s.split("; ").map(ToOwned::to_owned).collect::<Vec<_>>())
        .unwrap_or_default();
    if !authors.contains(&cleaned) {
        authors.push(cleaned);
    }
    if authors.is_empty() {
        None
    } else {
        Some(authors.join("; "))
    }
}

fn normalize_author(author: &str) -> Option<String> {
    let author = text::normalize_spaces(author)
        .trim_matches(|c: char| matches!(c, ':' | '-' | '—' | '–' | '/' | '\\'))
        .trim()
        .to_owned();
    if author.is_empty()
        || author.starts_with("http")
        || author.contains('@') && !author.contains(' ')
    {
        None
    } else {
        Some(author)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn extracts_og_metadata() {
        let html = Html::parse_document(
            r#"<html><head><meta property="og:title" content="Title"><meta name="author" content="Ada Lovelace"></head></html>"#,
        );
        let doc = extract_metadata(&html, Some("https://example.com/post"));
        assert_eq!(doc.title.as_deref(), Some("Title"));
        assert_eq!(doc.author.as_deref(), Some("Ada Lovelace"));
        assert_eq!(doc.hostname.as_deref(), Some("example.com"));
    }
}
