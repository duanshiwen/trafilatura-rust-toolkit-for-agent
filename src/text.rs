//! Text normalization and filtering helpers.

use once_cell::sync::Lazy;
use regex::Regex;

static SOCIAL_FILTER: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"(?i)^\W*(druck|e-?mail|facebook|flipboard|google|instagram|linkedin|mail|pdf|pinterest|pocket|print|qq|reddit|twitter|wechat|weibo|whatsapp|xing|more on this|mehr zum thema)\W*$")
        .expect("valid social filter regex")
});

/// Collapse whitespace and trim a string.
pub fn normalize_spaces(input: &str) -> String {
    input.split_whitespace().collect::<Vec<_>>().join(" ")
}

/// Normalize text while preserving paragraph-like line breaks.
pub fn normalize_multiline(input: &str) -> String {
    input
        .lines()
        .map(normalize_spaces)
        .filter(|line| !line.is_empty())
        .collect::<Vec<_>>()
        .join("\n")
}

/// Returns `true` when text is likely boilerplate social/share UI.
pub fn is_boilerplate_text(input: &str) -> bool {
    let text = normalize_spaces(input);
    text.is_empty() || SOCIAL_FILTER.is_match(&text)
}

/// Split a comma/semicolon separated metadata field into unique values.
pub fn split_meta_values(input: &str) -> Vec<String> {
    let mut out = Vec::new();
    for part in input.split([',', ';', '|']) {
        let value = normalize_spaces(part);
        if !value.is_empty() && !out.contains(&value) {
            out.push(value);
        }
    }
    out
}
