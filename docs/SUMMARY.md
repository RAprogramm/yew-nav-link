<!--
SPDX-FileCopyrightText: 2024-2026 RAprogramm <andrey.rozanov-vl@gmail.com>
SPDX-License-Identifier: MIT
-->

# Summary

[Introduction](introduction.md)

# Reference

- [Requirements](REQUIREMENTS.md)
- [Architecture](ARCHITECTURE.md)
- [Roadmap](ROADMAP.md)

# Process

- [Branching and merge policy](BRANCHING.md)

# Architecture decisions

- [About ADRs](adr/README.md)
- [0000 — Record architecture decisions](adr/0000-record-architecture-decisions.md)
- [0001 — `class` and `active_class` are `&'static str`](adr/0001-static-str-classes.md)
- [0002 — Drop the `macros` feature in 0.9.0](adr/0002-drop-macros-feature.md)
- [0003 — `NavError` is `#[non_exhaustive]` from 0.10.0](adr/0003-non-exhaustive-nav-error.md)
- [0004 — Render a manual `<a>` instead of wrapping `yew_router::Link`](adr/0004-manual-anchor-over-yew-router-link.md)
- [0005 — Active `NavLink` emits `aria-current="page"`](adr/0005-active-state-via-aria-current.md)
