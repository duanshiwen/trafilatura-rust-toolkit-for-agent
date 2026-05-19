# Contributing

Thanks for your interest in contributing to `trafilatura-rust-for-mcp`.

## Development setup

```bash
git clone https://github.com/your-org/trafilatura-rust-for-mcp.git
cd trafilatura-rust-for-mcp
cargo test
```

## Quality checks

Before submitting a pull request, run:

```bash
cargo fmt
cargo test
cargo clippy --all-targets -- -D warnings
cargo doc --no-deps
```

## Coding principles

- No `unwrap()` in library code.
- Use `thiserror` for typed errors.
- Keep public APIs documented with rustdoc.
- Prefer borrowed inputs (`&str`) over owned `String` where possible.
- Keep feature flags minimal and explicit.
- Add tests for extraction changes.

## Pull request checklist

- [ ] Tests added or updated.
- [ ] Documentation updated if public API changed.
- [ ] `cargo fmt` run.
- [ ] `cargo test` passes.
- [ ] `cargo clippy --all-targets -- -D warnings` passes.

## Roadmap-friendly contributions

Good first areas:

- More metadata fields.
- Better date extraction.
- More boilerplate fixtures.
- Feed/sitemap discovery.
- TEI output.
- Benchmark fixtures comparing Python Trafilatura.
