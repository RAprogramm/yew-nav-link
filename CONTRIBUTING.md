# Contributing

Thank you for your interest in contributing to `yew-nav-link`. This document outlines the workflow and standards for all contributions.

## Workflow

### 1. Create a Branch

Branch names must match the issue number:

```bash
git checkout -b 123
```

### 2. Commit Format

```bash
git commit -m "#123 feat: add custom class support"
```

Format: `#<issue> <type>: <description>`

| Type | Description |
|------|-------------|
| `feat` | New feature |
| `fix` | Bug fix |
| `docs` | Documentation only |
| `refactor` | Code restructuring, no behavior change |
| `test` | Test additions or modifications |
| `chore` | Maintenance, dependencies, tooling |

### 3. Create a Pull Request

- **Title**: Issue number only (e.g. `123`)
- **Description**: Must include `Closes #123`

## Pre-commit Checklist

Run these commands before pushing:

```bash
cargo +nightly fmt
cargo clippy -- -D warnings
cargo test
```

## CI Pipeline

| Job | Command | Requirement |
|-----|---------|-------------|
| Format | `cargo +nightly fmt --check` | Must pass |
| Lint | `cargo clippy -- -D warnings` | Must pass |
| Test | `cargo nextest run --workspace --all-features` | Must pass |
| Coverage | `cargo llvm-cov` | 95%+ target |
| Audit | `cargo audit` | Informational |
| Package | `cargo package --locked` | Must pass |

## Code Standards

| Rule | Requirement |
|------|-------------|
| No `unwrap()` / `expect()` | Use `?`, `.ok_or()`, or explicit error handling |
| No unnecessary `clone()` | Pass references where possible |
| `::` only in imports | `use foo::bar` is valid; `foo::bar()` in paths is not |
| Doc comments | All public items must have `///` documentation |
| Max line width | 99 characters |
| Edition | Rust 2024 |

## Project Structure

```
src/
├── lib.rs              # Public API and re-exports
├── nav_link.rs         # Core NavLink component
├── nav/                # Primitives: NavList, NavItem, NavDivider
├── components/         # UI components
├── hooks/              # Reactive hooks
├── utils/              # Path, URL, keyboard utilities
├── attrs.rs            # Attribute builders
├── errors.rs           # Error types
└── macros/             # Declarative macros (feature-gated)
```

## Feature Flags

| Feature | Description |
|---------|-------------|
| `macros` | Enables declarative macros (`nav_link!`) |

## Questions?

Open an issue or start a discussion in the [repository](https://github.com/RAprogramm/yew-nav-link).
