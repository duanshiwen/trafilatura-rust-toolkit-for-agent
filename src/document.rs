//! Extracted document data model.

use serde::{Deserialize, Serialize};

/// Metadata and content extracted from an HTML document.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct Document {
    /// Main title.
    pub title: Option<String>,
    /// Author or authors, separated by `; `.
    pub author: Option<String>,
    /// Canonical URL or source URL.
    pub url: Option<String>,
    /// Source hostname.
    pub hostname: Option<String>,
    /// Description/excerpt.
    pub description: Option<String>,
    /// Site name or publisher.
    pub sitename: Option<String>,
    /// Publication date, when found.
    pub date: Option<String>,
    /// Categories.
    pub categories: Vec<String>,
    /// Tags.
    pub tags: Vec<String>,
    /// Content fingerprint.
    pub fingerprint: Option<String>,
    /// External record ID.
    pub id: Option<String>,
    /// License string.
    pub license: Option<String>,
    /// Main extracted text.
    pub text: String,
    /// Extracted comments.
    pub comments: Option<String>,
    /// Raw text before final output formatting.
    pub raw_text: Option<String>,
    /// Detected/declared language.
    pub language: Option<String>,
    /// Representative image URL.
    pub image: Option<String>,
    /// Page type, commonly from OpenGraph or JSON-LD.
    pub pagetype: Option<String>,
}

impl Document {
    /// Returns `true` when the document contains no main text.
    pub fn is_empty(&self) -> bool {
        self.text.trim().is_empty()
    }

    /// Normalize string metadata by trimming whitespace and removing empty values.
    pub fn clean_and_trim(&mut self) {
        self.title = clean_option(self.title.take());
        self.author = clean_option(self.author.take());
        self.url = clean_option(self.url.take());
        self.hostname = clean_option(self.hostname.take());
        self.description = clean_option(self.description.take());
        self.sitename = clean_option(self.sitename.take());
        self.date = clean_option(self.date.take());
        self.fingerprint = clean_option(self.fingerprint.take());
        self.id = clean_option(self.id.take());
        self.license = clean_option(self.license.take());
        self.comments = clean_option(self.comments.take());
        self.raw_text = clean_option(self.raw_text.take());
        self.language = clean_option(self.language.take());
        self.image = clean_option(self.image.take());
        self.pagetype = clean_option(self.pagetype.take());
        self.text = crate::text::normalize_spaces(&self.text);
        self.categories = clean_vec(std::mem::take(&mut self.categories));
        self.tags = clean_vec(std::mem::take(&mut self.tags));
    }
}

fn clean_option(value: Option<String>) -> Option<String> {
    value
        .map(|s| crate::text::normalize_spaces(&s))
        .filter(|s| !s.is_empty())
}

fn clean_vec(values: Vec<String>) -> Vec<String> {
    let mut out = Vec::new();
    for value in values {
        let cleaned = crate::text::normalize_spaces(&value);
        if !cleaned.is_empty() && !out.contains(&cleaned) {
            out.push(cleaned);
        }
    }
    out
}
