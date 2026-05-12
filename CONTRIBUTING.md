<!--
SPDX-FileCopyrightText: 2024-2026 RAprogramm <andrey.rozanov-vl@gmail.com>
SPDX-License-Identifier: MIT
-->

# Contributing

Thank you for your interest in contributing to `yew-nav-link`. This document
walks you from a fresh clone all the way to a merged pull request. Anything
that touches the public API also touches `docs/REQUIREMENTS.md` and
`docs/ARCHITECTURE.md` — keep all three in sync.

## Code of conduct

Participation is governed by [`CODE_OF_CONDUCT.md`](CODE_OF_CONDUCT.md).
Report unacceptable behaviour via the contact in that file.

## Dev environment

You need:

- Rust **stable 1.95+** plus a `nightly` toolchain (used only for `rustfmt`):
  ```bash
  rustup default stable
  rustup toolchain install nightly --component rustfmt
  rustup target add wasm32-unknown-unknown
  ```
- [`trunk`](https://trunkrs.dev/) for the demo crate:
  ```bash
  cargo install --locked trunk
  ```
- The CI tooling, mirrored locally so you can reproduce the pipeline:
  ```bash
  cargo install --locked cargo-deny cargo-audit cargo-llvm-cov cargo-nextest \
                          git-cliff
  pip install --user reuse
  ```

The pre-commit hook in `.hooks/pre-commit` invokes most of these, so a single
`git commit` runs `fmt --check`, `clippy --pedantic --nursery`,
`cargo deny check`, `cargo audit`, and `actionlint`.

To wire it in once per clone:

```bash
git config core.hooksPath .hooks
```

## Common tasks

| Task | Command |
|---|---|
| Build | `cargo build --workspace --all-features` |
| Format check | `cargo +nightly fmt --all -- --check` |
| Lint | `cargo clippy --workspace --all-targets --all-features -- -D warnings -W clippy::pedantic -W clippy::nursery` |
| Tests | `cargo nextest run --all-features` |
| Doc tests | `cargo test --doc --all-features` |
| Coverage report | `cargo llvm-cov --all-features --html` |
| Build the demo | `cd example && trunk build --release` |
| Serve the demo | `cd example && trunk serve` then open <http://127.0.0.1:3000> |
| Generate CHANGELOG preview | `git-cliff --unreleased` |
| Verify SPDX headers | `reuse lint` |

## Workflow

### 1. Open or pick an issue

- Bugs: use the [bug report template][bug].
- Features: use the [feature request template][feature].
- Security: **do not** open a public issue — see [`SECURITY.md`](SECURITY.md).
- Questions: GitHub Discussions or Issues with the `question` label.
- Browse [`good first issue`][gfi] for bounded entry points.

[bug]: https://github.com/RAprogramm/yew-nav-link/issues/new?template=bug.yml
[feature]: https://github.com/RAprogramm/yew-nav-link/issues/new?template=feature.yml
[gfi]: https://github.com/RAprogramm/yew-nav-link/issues?q=is%3Aopen+label%3A%22good+first+issue%22

### 2. Create a branch

Branch names match the issue number:

```bash
git checkout -b 123
```

### 3. Commit format

```bash
git commit -m "#123 feat: add custom class support"
```

| Type | Use for | Triggers in CHANGELOG |
|---|---|---|
| `feat` | New public API | yes — `Features` |
| `fix` | Bug fix | yes — `Bug Fixes` |
| `docs` | Markdown / rustdoc only | yes — `Documentation` |
| `refactor` | Internal restructuring, no behaviour change | yes — `Refactoring` |
| `ci` | CI / release pipeline | yes — `CI` |
| `test` | Test additions or modifications | no |
| `chore` | Maintenance, dependency bumps, tooling | no |

`git-cliff` reads these prefixes (`cliff.toml`).

### 4. Open a pull request

- **Title:** issue number only (e.g. `123`).
- **Body:** must include `Closes #123` so the issue auto-closes on merge.
- **Reviews:** there is no required-reviewer rule on `main`, but every PR
  must pass the `CI Success` aggregate status check before merge — that's
  enforced by branch protection.
- **Merge style:** squash, with branch auto-deleted on merge. The PR title
  becomes the conventional-commit subject on `main`; the PR body becomes the
  commit message body. We keep `main` linear; force-pushes to `main` and
  branch deletion are blocked. Rebase merge stays allowed for the rare case
  where intermediate commits carry independent value. See
  [`docs/BRANCHING.md`](docs/BRANCHING.md) for the full policy.

## Code standards

| Rule | Requirement |
|---|---|
| `unsafe` | Forbidden in `src/` and `tests/`. |
| `unwrap()` / `expect()` | Forbidden outside `#[cfg(test)]`. Use `?`, `Option::map_or_else`, `unwrap_or`. |
| Unnecessary `clone()` | Avoid. Pass references. |
| Public items | Every `pub` item carries a `///` doc comment, plus a doctest where it makes sense. |
| Line width | 99 characters (`max_width` in `.rustfmt.toml`). |
| Trailing commas | Never (`trailing_comma = "Never"` in `.rustfmt.toml`). |
| Edition | Rust 2024. |
| MSRV | 1.95. |

## CI overview

`.github/workflows/ci.yml` runs the following jobs on every PR. The
`ci-success` aggregate must come back green for branch protection to allow
merge.

| Job | Required | Notes |
|---|---|---|
| `Extract MSRV` | yes | reads `rust-version` from `Cargo.toml` |
| `Check` | yes | matrix: 3 toolchains × 3 OSes |
| `Format` | yes | nightly `rustfmt --check` |
| `Lint (clippy)` | yes | pedantic + nursery |
| `Documentation` | yes | `cargo doc --all-features` with `RUSTDOCFLAGS=-D warnings` |
| `no-std` | yes | informational stub (Yew is std-only) |
| `Security` | yes | `cargo deny check` + `cargo audit` |
| `REUSE Compliance` | yes | `reuse lint` |
| `Test` | yes (skip allowed) | `cargo nextest` + doctests |
| `WASM Tests` | yes | `wasm-pack test --headless --chrome --firefox --test wasm` |
| `Coverage` | yes (skip allowed) | `cargo llvm-cov` → Codecov upload |
| `Benchmarks` | yes (skip allowed) | `cargo bench --no-run` |
| `Example WASM build` | yes | `trunk build --release` against `example/` |
| `E2E` | yes | Playwright suite under `example/e2e/` on chromium + firefox |
| `Lighthouse` | yes | thresholds: perf 0.85, a11y 0.9, best-practices 0.9, SEO 0.9 |
| `Actionlint` | yes | lints all workflow YAML |
| `Changelog` | yes (skip allowed) | runs `git-cliff` |

Two more workflows handle deployment side-effects on push to `main`:

- `.github/workflows/pages.yml` deploys the demo to
  <https://raprogramm.github.io/yew-nav-link/>.
- `.github/workflows/release-plz.yml` opens / updates the release PR
  and, when that PR is merged, tags + publishes to crates.io + creates
  the GitHub release. See [`RELEASE.md`](RELEASE.md) for the full flow.

## Releasing

The bump → publish dance is documented in [`RELEASE.md`](RELEASE.md). In
short: edit `Cargo.toml` `version`, prepend a `[X.Y.Z] - YYYY-MM-DD`
section to `CHANGELOG.md`, merge to `main`, and the release job pushes to
crates.io and creates the GitHub release.

## Recognition

All contributors are listed in [`AUTHORS.md`](AUTHORS.md). When your first
PR merges, add yourself to that list in a follow-up patch (or ask the
maintainer to do it).

## References

- [`README.md`](README.md) — overview and quick start.
- [`CODE_OF_CONDUCT.md`](CODE_OF_CONDUCT.md)
- [`SECURITY.md`](SECURITY.md)
- [`docs/REQUIREMENTS.md`](docs/REQUIREMENTS.md) — formal functional and non-functional requirements.
- [`docs/ARCHITECTURE.md`](docs/ARCHITECTURE.md) — design rationale.
- [`docs/ROADMAP.md`](docs/ROADMAP.md) — trajectory toward 0.10 and 1.0.

## Questions?

Open a [discussion][discussions] or an issue with the `question` label.

[discussions]: https://github.com/RAprogramm/yew-nav-link/discussions
