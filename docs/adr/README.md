<!--
SPDX-FileCopyrightText: RAprogramm <andrey.rozanov.vl@gmail.com>
SPDX-License-Identifier: MIT
-->

# Architecture Decision Records

Each non-trivial design decision in `yew-nav-link` is captured as a
short [MADR][madr]-style document. Decisions are append-only — a record
is never edited after its status moves to `accepted`. Reversals are
expressed as new ADRs that mark the prior record `superseded by NNNN`.

[madr]: https://adr.github.io/madr/

## Index

| ID | Title | Status |
|---|---|---|
| [0000](0000-record-architecture-decisions.md) | Record architecture decisions | accepted |
| [0001](0001-static-str-classes.md) | `class` and `active_class` are `&'static str` | accepted |
| [0002](0002-drop-macros-feature.md) | Drop the `macros` feature in 0.9.0 | accepted |
| [0003](0003-non-exhaustive-nav-error.md) | `NavError` is `#[non_exhaustive]` from 0.10.0 | accepted |
| [0004](0004-manual-anchor-over-yew-router-link.md) | Render a manual `<a>` instead of wrapping `yew_router::Link` | accepted |
| [0005](0005-active-state-via-aria-current.md) | Active `NavLink` emits `aria-current="page"` | accepted |

## When to write an ADR

Write one when the answer to "why is it this way" is non-obvious and
would not be derivable from reading the code alone. Renaming a field is
not an ADR. Choosing `&'static str` over `AttrValue` is an ADR.

## Template

Copy `0000-record-architecture-decisions.md` as a starting point. Keep
each record to roughly one screen — context, decision, consequences. The
goal is to read fast and rot slowly.
