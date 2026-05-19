# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/), and this project adheres to semantic versioning once released.

## [0.1.0] - 2026-05-19

### Added

- Initial Rust library crate.
- Public APIs:
  - `extract`
  - `extract_with_metadata`
  - `Extractor`
  - `ExtractorOptions`
- Main content extraction with CSS selector heuristics.
- Metadata extraction from meta tags, OpenGraph, canonical links, and JSON-LD.
- Output formats: TXT, Markdown, JSON, XML, HTML.
- Simhash content fingerprinting.
- Optional async downloader behind `download` feature.
- README, technical documentation, usage guide, and examples.
- Unit tests and rustdoc example.
