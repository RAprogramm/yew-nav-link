# Contributing

Thanks for your interest in contributing to `yew-nav-link`!

## Quick Start

1. Fork and clone the repo
2. Create a branch from an issue: `git checkout -b 123-feature-name`
3. Make changes following our standards
4. Submit a PR

## Standards

This project follows [RustManifest](https://github.com/RAprogramm/RustManifest) guidelines:

### Code Style

- Format with `cargo +nightly fmt`
- Max 99 characters per line
- No `mod.rs` files
- Use `::` only in imports

### Quality

- No `unwrap()` or `expect()` in library code
- Avoid unnecessary `clone()`
- Prefer `&str` over `String`, `&[T]` over `Vec<T>`

### Documentation

- Doc comments (`///`) on all public items
- Include examples in doc comments

### Commits

Format: `#issue type: description`

```
#42 feat: add custom class support
#15 fix: active state detection
#8 docs: update usage examples
```

Types: `feat`, `fix`, `docs`, `refactor`, `test`, `chore`

## Before PR

```bash
cargo +nightly fmt
cargo clippy -- -D warnings
cargo test
```

## Questions?

Open an issue or check [RustManifest](https://github.com/RAprogramm/RustManifest) for detailed guidelines.
