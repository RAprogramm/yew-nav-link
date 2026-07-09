<!--
SPDX-FileCopyrightText: RAprogramm <andrey.rozanov.vl@gmail.com>
SPDX-License-Identifier: MIT
-->

# Introduction

This book is the architectural reference for [`yew-nav-link`][crate]. It
is not a tutorial — the crate's [`README`][readme] is the place to learn
*what* the library does and *how* to drop it into your Yew app. The book
collects the documents that explain *why* the crate is shaped the way
it is and the contracts it commits to.

[crate]: https://crates.io/crates/yew-nav-link
[readme]: https://github.com/RAprogramm/yew-nav-link#readme

## What is here

- **[Requirements](REQUIREMENTS.md)** — the functional and non-functional
  contracts the crate commits to. Every claim in this document is
  exercised by the test suite, the demo, or the CI pipeline.
- **[Architecture](ARCHITECTURE.md)** — the module layout, the
  active-state algorithm, hook contracts, and the breadcrumb context flow.
- **[Roadmap](ROADMAP.md)** — what is in flight between the current 0.10
  line and the 1.0 freeze.
- **[Branching and merge policy](BRANCHING.md)** — how changes reach
  `main` and what the squash-merge contract is.
- **[Architecture Decision Records](adr/README.md)** — one MADR file per
  non-trivial decision. The reasoning behind 0.9 → 0.10 lives here.

## What is *not* here

- API docs. Those live on [docs.rs][docs] and are generated from the
  crate's source. The book intentionally does not duplicate them.
- The contributor workflow. That is in
  [`CONTRIBUTING.md`][contributing] in the repository root.
- The release procedure. That is in [`RELEASE.md`][release].
- The changelog. That is in [`CHANGELOG.md`][changelog].

[docs]: https://docs.rs/yew-nav-link
[contributing]: https://github.com/RAprogramm/yew-nav-link/blob/main/CONTRIBUTING.md
[release]: https://github.com/RAprogramm/yew-nav-link/blob/main/RELEASE.md
[changelog]: https://github.com/RAprogramm/yew-nav-link/blob/main/CHANGELOG.md

## How to read this book

ADRs are append-only and small. Read the index first
([`adr/README.md`](adr/README.md)), pick the decision you want context on,
and the body answers in roughly one screen. The Reference and Process
sections are longer prose; read top-to-bottom on first visit, jump back
to a section by its heading on subsequent visits.

When a record is superseded, it stays in place and the new record marks
it `superseded by NNNN`. The history is intact — the file index always
reflects the latest accepted state.
