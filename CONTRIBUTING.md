# Contributing

## Workflow

### 1. Create branch from issue number

```bash
git checkout -b 123
```

Branch name = issue number only.

### 2. Commit format

```bash
git commit -m "#123 feat: add custom class support"
```

Format: `#<issue> <type>: <description>`

Types: `feat`, `fix`, `docs`, `refactor`, `test`, `chore`

### 3. Create PR

- Title: `123`
- Description must include: `Closes #123`

## Before commit

Run locally:

```bash
cargo +nightly fmt
cargo clippy -- -D warnings
cargo test
```

## CI checks

| Check | Command |
|-------|---------|
| Format | `cargo +nightly fmt --check` |
| Lint | `cargo clippy -- -D warnings` |
| Test | `cargo test` |
| Coverage | `cargo llvm-cov` (95%+ required) |

## Code standards

| Rule | Example |
|------|---------|
| No `unwrap()` / `expect()` | Use `?` or `.ok_or()` |
| No unnecessary `clone()` | Pass references |
| `::` only in imports | `use foo::bar` ok, `foo::bar()` bad |
| Doc comments on public items | `/// Description` |
| Max line width | 99 chars |

## Full guidelines

See [RustManifest](https://github.com/RAprogramm/RustManifest)
