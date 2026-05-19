# 贡献指南

感谢你有兴趣为 `trafilatura-rust-for-mcp` 做贡献。

## 开发环境设置

```bash
git clone https://github.com/your-org/trafilatura-rust-for-mcp.git
cd trafilatura-rust-for-mcp
cargo test
```

## 质量检查

提交 Pull Request 之前，请运行：

```bash
cargo fmt
cargo test
cargo clippy --all-targets -- -D warnings
cargo doc --no-deps
```

## 编码原则

- 库代码中不要使用 `unwrap()`。
- 使用 `thiserror` 定义类型化错误。
- 为公共 API 保持 rustdoc 文档。
- 参数优先使用借用输入（如 `&str`），避免不必要的自有 `String`。
- feature flags 保持最小且显式。
- 修改提取逻辑时添加对应测试。

## Pull Request 检查清单

- [ ] 已添加或更新测试。
- [ ] 如果公共 API 发生变化，已更新文档。
- [ ] 已运行 `cargo fmt`。
- [ ] `cargo test` 通过。
- [ ] `cargo clippy --all-targets -- -D warnings` 通过。

## 适合路线图的贡献方向

适合作为首次贡献的方向：

- 更多元数据字段。
- 更好的日期提取。
- 更多样板内容过滤 fixtures。
- Feed/sitemap 发现。
- TEI 输出。
- 与 Python Trafilatura 对比的基准测试 fixtures。
