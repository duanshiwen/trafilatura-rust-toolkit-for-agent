# 更新日志

本文件记录此项目的所有重要变更。

格式基于 [Keep a Changelog](https://keepachangelog.com/en/1.1.0/)。项目正式发布后将遵循语义化版本控制。

## [0.1.0] - 2026-05-19

### 新增

- 初始 Rust 库 crate。
- 公共 API：
  - `extract`
  - `extract_with_metadata`
  - `Extractor`
  - `ExtractorOptions`
- 基于 CSS 选择器启发式规则的正文提取。
- 从 meta 标签、OpenGraph、canonical 链接和 JSON-LD 中提取元数据。
- 输出格式：TXT、Markdown、JSON、XML、HTML。
- Simhash 内容指纹。
- 位于 `download` feature 后的可选异步下载器。
- README、技术文档、使用指南和示例。
- 单元测试和 rustdoc 示例。
