# 使用指南

## 基础提取

```rust
use trafilatura_rust_for_mcp::{extract, ExtractorOptions};

let html = "<article><p>Hello world.</p></article>";
let text = extract(html, &ExtractorOptions::default())?;
```

## 配置输出格式

```rust
use trafilatura_rust_for_mcp::{extract, ExtractorOptions, OutputFormat};

let options = ExtractorOptions {
    output_format: OutputFormat::Markdown,
    include_formatting: true,
    ..ExtractorOptions::default()
};

let markdown = extract(html, &options)?;
```

## 面向 MCP 的 JSON 输出

```rust
use trafilatura_rust_for_mcp::{extract, ExtractorOptions};

let options = ExtractorOptions::for_mcp();
let json = extract(html, &options)?;
```

`ExtractorOptions::for_mcp()` 会设置：

- JSON 输出
- 启用元数据
- 禁用评论
- 启用链接
- 启用格式保留

当工具需要把结构化文章内容传入 MCP 流水线或 LLM 上下文时，这个配置很有用。

## 提取元数据对象

```rust
use trafilatura_rust_for_mcp::{extract_with_metadata, ExtractorOptions};

let doc = extract_with_metadata(html, &ExtractorOptions::default())?;
println!("Title: {:?}", doc.title);
println!("Text: {}", doc.text);
```

## 来源 URL

传入来源 URL 可以改善主机名与 canonical 元数据：

```rust
use trafilatura_rust_for_mcp::ExtractorOptions;

let options = ExtractorOptions {
    url: Some("https://example.com/article".to_string()),
    with_metadata: true,
    ..ExtractorOptions::default()
};
```

## 精确率与召回率

```rust
use trafilatura_rust_for_mcp::{ExtractorOptions, Focus};

let precision = ExtractorOptions {
    focus: Focus::Precision,
    ..ExtractorOptions::default()
};

let recall = ExtractorOptions {
    focus: Focus::Recall,
    ..ExtractorOptions::default()
};
```

- `Precision`：更激进地过滤样板内容。
- `Balanced`：默认模式。
- `Recall`：容忍更多候选内容，尽量多保留正文。

## 下载

使用默认 features 时：

```rust,no_run
use trafilatura_rust_for_mcp::{fetch_url, FetchOptions};

#[tokio::main]
async fn main() -> trafilatura_rust_for_mcp::Result<()> {
    let html = fetch_url("https://example.com", &FetchOptions::default()).await?;
    println!("{}", html);
    Ok(())
}
```

如需禁用网络支持：

```toml
trafilatura-rust-for-mcp = { version = "0.1", default-features = false }
```

## 内容指纹

启用 `deduplicate` 可以计算 simhash 指纹：

```rust
let options = ExtractorOptions {
    deduplicate: true,
    with_metadata: true,
    ..ExtractorOptions::default()
};
let doc = extract_with_metadata(html, &options)?;
println!("{:?}", doc.fingerprint);
```

## 错误处理

所有可能失败的 API 都返回 `Result<T, TrafilaturaError>`：

```rust
match extract(html, &ExtractorOptions::default()) {
    Ok(text) => println!("{text}"),
    Err(err) => eprintln!("extraction failed: {err}"),
}
```

## 推荐 MCP 集成模式

1. 获取或接收 HTML。
2. 使用 `ExtractorOptions::for_mcp()` 调用 `extract_with_metadata` 或 `extract`。
3. 将得到的 JSON 放入 MCP 工具响应中。
4. 在 `ExtractorOptions::url` 中保留原始 URL。
5. 构建语料库或爬取流水线时启用 `deduplicate`。
