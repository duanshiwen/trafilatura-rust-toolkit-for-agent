# trafilatura-rust-for-mcp

一个用于从网页中提取正文与元数据的 Rust 库，灵感来自 [Trafilatura](https://github.com/adbar/trafilatura)，并面向 MCP 工作流设计。

> 状态：**初始 Rust 移植版 / 公共库脚手架**。当前 crate 已实现实用的核心正文提取、元数据提取、内容渲染、指纹计算以及可选下载能力。它目前还不是 Python Trafilatura 的完整功能等价替代品。

## 功能特性

- 从 HTML 中提取文章主体文本。
- 移除常见样板内容，例如导航栏、页脚、侧边栏、分享模块和相关阅读链接。
- 提取元数据，来源包括：
  - OpenGraph 标签
  - Twitter/meta 标签
  - canonical 链接
  - JSON-LD
  - 页面标题/标题元素
- 支持输出格式：
  - 纯文本
  - Markdown
  - JSON
  - 简单 XML
  - 简单 HTML
- 通过 `ExtractorOptions::for_mcp()` 提供 MCP 友好的 JSON 输出配置。
- 基于 `reqwest` + rustls 的可选异步下载器。
- 使用 simhash 生成内容指纹。
- 通过 `thiserror` 提供惯用的 Rust 错误类型。

## 安装

```toml
[dependencies]
trafilatura-rust-for-mcp = "0.1"
```

本地开发时可以使用 path 依赖：

```toml
[dependencies]
trafilatura-rust-for-mcp = { path = "../trafilatura-rust-for-mcp" }
```

## 快速开始

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

## 提取正文与元数据

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

## 面向 MCP 的 JSON 输出

```rust
use trafilatura_rust_for_mcp::{extract, ExtractorOptions};

fn main() -> trafilatura_rust_for_mcp::Result<()> {
    let options = ExtractorOptions::for_mcp();
    let json = extract("<article><p>Hello MCP.</p></article>", &options)?;
    println!("{json}");
    Ok(())
}
```

## 下载并提取

`download` feature 默认启用。

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

如需禁用网络相关依赖：

```toml
trafilatura-rust-for-mcp = { version = "0.1", default-features = false }
```

## 架构说明

参见 [docs/TECHNICAL.md](docs/TECHNICAL.md)。

## 使用指南

参见 [docs/USAGE.md](docs/USAGE.md)。

## Swift / macOS 原生客户端

本项目提供 Swift Package 集成路径，可供 SwiftUI/AppKit macOS 客户端调用 Rust 提取能力。

先生成 Swift Package 所需的 `.xcframework`：

```bash
scripts/build-swift-package.sh
```

然后在 Xcode 中添加本地 Swift Package：

```text
swift/TrafilaturaSwift
```

Swift 代码中使用：

```swift
import Trafilatura

let text = try Trafilatura.extractText(fromHTML: html)
let json = try Trafilatura.extractJSONForMCP(fromHTML: html)
```

详细说明见 [docs/SWIFT_INTEGRATION.md](docs/SWIFT_INTEGRATION.md)。

## 路线图

- 改进 DOM 修改与清理逻辑，使其更接近 Python Trafilatura。
- 更健壮的日期提取。
- Feed 与 sitemap 发现模块。
- TEI XML 输出。
- 基于 Python Trafilatura fixtures 的基准测试套件。
- 可选 WASM 目标。
- CLI 二进制功能。

## 许可证

Apache-2.0。参见 [LICENSE](LICENSE)。

## 致谢

本项目灵感来自 Adrien Barbaresi 出色的 Python [Trafilatura](https://github.com/adbar/trafilatura) 项目。当前 Rust 实现为全新编写，并刻意聚焦于惯用 Rust 库用法和 MCP 集成场景。
