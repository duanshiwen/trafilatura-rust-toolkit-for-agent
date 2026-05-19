//! Main extraction pipeline.

use scraper::{ElementRef, Html, Selector};

use crate::{
    config::{ExtractorOptions, Focus},
    dedup,
    document::Document,
    error::{Result, TrafilaturaError},
    metadata, output, text,
};

/// Stateless extractor object.
#[derive(Debug, Clone)]
pub struct Extractor {
    options: ExtractorOptions,
}

impl Extractor {
    /// Create a new extractor with the provided options.
    pub fn new(options: ExtractorOptions) -> Self {
        Self { options }
    }

    /// Extract rendered text or structured output from an HTML string.
    pub fn extract(&self, html: &str) -> Result<String> {
        let doc = self.extract_with_metadata(html)?;
        output::render_document(&doc, self.options.output_format, self.options.with_metadata)
    }

    /// Extract document content and metadata from an HTML string.
    pub fn extract_with_metadata(&self, html: &str) -> Result<Document> {
        if html.trim().is_empty() {
            return Err(TrafilaturaError::Extraction("empty input".to_owned()));
        }

        let parsed = Html::parse_document(html);
        let mut doc = metadata::extract_metadata(&parsed, self.options.url.as_deref());

        let main_text = extract_main_text(&parsed, &self.options);
        let fallback_text = if main_text.len() < self.options.min_extracted_size {
            baseline_extract(&parsed, &self.options)
        } else {
            String::new()
        };

        let chosen = if main_text.len() >= fallback_text.len() {
            main_text
        } else {
            fallback_text
        };
        let chosen = text::normalize_multiline(&chosen);
        if chosen.len() < self.options.min_output_size {
            return Err(TrafilaturaError::Extraction(
                "no sufficient main content found".to_owned(),
            ));
        }

        doc.raw_text = Some(chosen.clone());
        doc.text = chosen;
        if self.options.include_comments {
            doc.comments = extract_comments(&parsed);
        }
        if self.options.deduplicate {
            doc.fingerprint = Some(dedup::content_fingerprint(&format!(
                "{} {}",
                doc.title.as_deref().unwrap_or_default(),
                doc.text
            )));
        }
        doc.clean_and_trim();
        Ok(doc)
    }
}

/// Extract content with default one-shot API.
pub fn extract(html: &str, options: &ExtractorOptions) -> Result<String> {
    Extractor::new(options.clone()).extract(html)
}

/// Extract document and metadata with default one-shot API.
pub fn extract_with_metadata(html: &str, options: &ExtractorOptions) -> Result<Document> {
    Extractor::new(options.clone()).extract_with_metadata(html)
}

fn extract_main_text(parsed: &Html, options: &ExtractorOptions) -> String {
    if let Some(best) = select_best_candidate(parsed, options) {
        let text = collect_candidate_text(best, options);
        if !text.trim().is_empty() {
            return text;
        }
    }
    String::new()
}

fn select_best_candidate<'a>(
    parsed: &'a Html,
    options: &ExtractorOptions,
) -> Option<ElementRef<'a>> {
    let selectors = [
        "article",
        "main",
        "[itemprop*=articleBody]",
        "[class*=article-body]",
        "[id*=article-body]",
        "[class*=article__body]",
        "[id*=article__body]",
        "[class*=article-content]",
        "[id*=article-content]",
        "[class*=entry-content]",
        "[id*=entry-content]",
        "[class*=post-content]",
        "[class*=post_body]",
        "[class*=post-body]",
        "[class*=story-body]",
        "[id*=story-body]",
        "[class*=main-content]",
        "[id*=main-content]",
        "[role=main]",
        "body",
    ];

    let mut best: Option<(usize, ElementRef<'a>)> = None;
    for selector in selectors {
        let Ok(selector) = Selector::parse(selector) else {
            continue;
        };
        for candidate in parsed.select(&selector) {
            if is_discardable(candidate) {
                continue;
            }
            let score = candidate_score(candidate, options);
            if score > best.as_ref().map_or(0, |(score, _)| *score) {
                best = Some((score, candidate));
            }
        }
        if best
            .as_ref()
            .is_some_and(|(score, _)| *score > options.min_extracted_size)
        {
            break;
        }
    }
    best.map(|(_, candidate)| candidate)
}

fn candidate_score(candidate: ElementRef<'_>, options: &ExtractorOptions) -> usize {
    let content_text = text::normalize_spaces(&candidate.text().collect::<Vec<_>>().join(" "));
    if content_text.is_empty() {
        return 0;
    }
    let text_len = content_text.len();
    let link_density = link_density(candidate);
    let paragraph_bonus = count_selector(candidate, "p") * 30;
    let heading_bonus = count_selector(candidate, "h1,h2,h3") * 20;
    let table_penalty = if options.include_tables {
        0
    } else {
        count_selector(candidate, "table") * 50
    };
    let density_penalty = (link_density * text_len as f64) as usize;
    let focus_bonus = match options.focus {
        Focus::Precision if link_density < 0.15 => 100,
        Focus::Recall => 50,
        _ => 0,
    };
    text_len
        .saturating_add(paragraph_bonus)
        .saturating_add(heading_bonus)
        .saturating_add(focus_bonus)
        .saturating_sub(table_penalty)
        .saturating_sub(density_penalty)
}

fn collect_candidate_text(candidate: ElementRef<'_>, options: &ExtractorOptions) -> String {
    let mut blocks = Vec::new();
    let mut block_selectors = vec!["h1", "h2", "h3", "p", "blockquote", "pre", "li"];
    if options.include_tables {
        block_selectors.extend(["th", "td"]);
    }
    if let Ok(selector) = Selector::parse(&block_selectors.join(",")) {
        for element in candidate.select(&selector) {
            if is_discardable(element) || link_density(element) > max_link_density(options) {
                continue;
            }
            let tag = element.value().name();
            let block = render_block(element, tag, options);
            if !text::is_boilerplate_text(&block) && !blocks.contains(&block) {
                blocks.push(block);
            }
        }
    }

    if options.include_images {
        if let Ok(selector) = Selector::parse("img[src]") {
            for image in candidate.select(&selector) {
                if let Some(markdown) = render_image(image) {
                    blocks.push(markdown);
                }
            }
        }
    }

    if blocks.is_empty() {
        let all_text = text::normalize_spaces(&candidate.text().collect::<Vec<_>>().join(" "));
        if !all_text.is_empty() {
            blocks.push(all_text);
        }
    }
    blocks.join("\n\n")
}

fn baseline_extract(parsed: &Html, options: &ExtractorOptions) -> String {
    for selector in ["article", "main", "body"] {
        let Ok(selector) = Selector::parse(selector) else {
            continue;
        };
        if let Some(element) = parsed.select(&selector).next() {
            let text = collect_candidate_text(element, options);
            if text.len() >= options.min_output_size {
                return text;
            }
        }
    }
    String::new()
}

fn extract_comments(parsed: &Html) -> Option<String> {
    let selectors = [
        "[id*=comment] p",
        "[class*=comment] p",
        "[id*=comments] li",
        "[class*=comments] li",
    ];
    let mut blocks = Vec::new();
    for selector in selectors {
        let Ok(selector) = Selector::parse(selector) else {
            continue;
        };
        for element in parsed.select(&selector) {
            let block = text::normalize_spaces(&element.text().collect::<Vec<_>>().join(" "));
            if !text::is_boilerplate_text(&block) && block.len() > 10 && !blocks.contains(&block) {
                blocks.push(block);
            }
        }
    }
    if blocks.is_empty() {
        None
    } else {
        Some(blocks.join("\n\n"))
    }
}

fn render_block(element: ElementRef<'_>, tag: &str, options: &ExtractorOptions) -> String {
    let all_text = text::normalize_spaces(&element.text().collect::<Vec<_>>().join(" "));
    let own_text = element_own_text(element).unwrap_or_else(|| all_text.clone());
    let text_value = if matches!(tag, "h1" | "h2" | "h3") {
        own_text
    } else {
        all_text
    };
    if options.output_format == crate::config::OutputFormat::Markdown || options.include_formatting
    {
        match tag {
            "h1" => format!("# {text_value}"),
            "h2" => format!("## {text_value}"),
            "h3" => format!("### {text_value}"),
            "li" => format!("- {text_value}"),
            "pre" => format!("```\n{text_value}\n```"),
            "blockquote" => format!("> {text_value}"),
            _ => {
                if options.include_links {
                    render_links_as_markdown(element).unwrap_or(text_value)
                } else {
                    text_value
                }
            }
        }
    } else {
        text_value
    }
}

fn element_own_text(element: ElementRef<'_>) -> Option<String> {
    let raw = element
        .children()
        .filter_map(|child| child.value().as_text())
        .map(|text| text.text.to_string())
        .collect::<Vec<_>>()
        .join(" ");
    let normalized = text::normalize_spaces(&raw);
    if normalized.is_empty() {
        None
    } else {
        Some(normalized)
    }
}

fn render_links_as_markdown(element: ElementRef<'_>) -> Option<String> {
    let Ok(link_selector) = Selector::parse("a[href]") else {
        return None;
    };
    let mut output = text::normalize_spaces(&element.text().collect::<Vec<_>>().join(" "));
    for link in element.select(&link_selector) {
        let link_text = text::normalize_spaces(&link.text().collect::<Vec<_>>().join(" "));
        let Some(href) = link.value().attr("href") else {
            continue;
        };
        if !link_text.is_empty() && output.contains(&link_text) {
            output = output.replacen(&link_text, &format!("[{link_text}]({href})"), 1);
        }
    }
    Some(output)
}

fn render_image(image: ElementRef<'_>) -> Option<String> {
    let src = image
        .value()
        .attr("src")
        .or_else(|| image.value().attr("data-src"))?;
    let alt = image
        .value()
        .attr("alt")
        .or_else(|| image.value().attr("title"))
        .unwrap_or("");
    Some(format!("![{}]({})", text::normalize_spaces(alt), src))
}

fn count_selector(element: ElementRef<'_>, selector: &str) -> usize {
    Selector::parse(selector).map_or(0, |selector| element.select(&selector).count())
}

fn link_density(element: ElementRef<'_>) -> f64 {
    let all_text_len = text::normalize_spaces(&element.text().collect::<Vec<_>>().join(" "))
        .len()
        .max(1);
    let Ok(selector) = Selector::parse("a") else {
        return 0.0;
    };
    let link_text_len = element
        .select(&selector)
        .map(|link| text::normalize_spaces(&link.text().collect::<Vec<_>>().join(" ")).len())
        .sum::<usize>();
    link_text_len as f64 / all_text_len as f64
}

fn max_link_density(options: &ExtractorOptions) -> f64 {
    match options.focus {
        Focus::Precision => 0.25,
        Focus::Balanced => 0.5,
        Focus::Recall => 0.8,
    }
}

fn is_discardable(element: ElementRef<'_>) -> bool {
    let tag = element.value().name();
    if matches!(
        tag,
        "script" | "style" | "nav" | "footer" | "aside" | "form" | "iframe" | "noscript"
    ) {
        return true;
    }
    let attrs = format!(
        "{} {} {}",
        element.value().attr("id").unwrap_or_default(),
        element.value().attr("class").unwrap_or_default(),
        element.value().attr("role").unwrap_or_default()
    )
    .to_ascii_lowercase();
    let bad_markers = [
        "footer",
        "sidebar",
        "nav",
        "menu",
        "breadcrumb",
        "related",
        "share",
        "social",
        "cookie",
        "banner",
        "advert",
        "newsletter",
        "promo",
        "widget",
        "modal",
        "overlay",
        "pagination",
    ];
    bad_markers.iter().any(|marker| attrs.contains(marker))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn extracts_article_text() {
        let html = r"<html><body><nav>Home About</nav><article><h1>Title</h1><p>This is the first paragraph.</p><p>This is the second paragraph.</p></article></body></html>";
        let doc = extract_with_metadata(html, &ExtractorOptions::default()).expect("extracts");
        assert!(doc.text.contains("first paragraph"));
        assert!(!doc.text.contains("Home About"));
    }

    #[test]
    fn renders_markdown_headings() {
        let options = ExtractorOptions {
            output_format: crate::config::OutputFormat::Markdown,
            include_formatting: true,
            ..ExtractorOptions::default()
        };
        let html = r"<html><body><article><h1>Hello</h1><p>World.</p></article></body></html>";
        let out = extract(html, &options).expect("extracts");
        assert!(out.starts_with("# Hello"));
    }
}
