<!--
SPDX-FileCopyrightText: RAprogramm <andrey.rozanov.vl@gmail.com>
SPDX-License-Identifier: MIT
-->

# 0001 — `class` and `active_class` are `&'static str`

- **Status:** accepted
- **Date:** 2026-05-12
- **Deciders:** RAprogramm

## Context

`NavLinkProps` exposes two CSS-class slots:

```rust
#[prop_or("nav-link")]
pub class: &'static str,

#[prop_or("active")]
pub active_class: &'static str,
```

The natural alternatives in the Yew ecosystem are `AttrValue` (an
internally `Rc<str>`-backed handle used by attribute values) and
`Classes` (an internal `Vec<AttrValue>` with deduplication and lazy
join). Both accept runtime-built strings; `&'static str` does not.

The question is whether the cost of `&'static str` — consumers cannot
pass an interpolated class list — outweighs the cost of the alternatives:
an allocation on every render, a deeper trait stack, and a less obvious
"what is this argument" answer for newcomers.

## Decision

Keep `&'static str` for both class slots.

The crate's contract is that class slots configure the wrapper. A
consumer who needs runtime classes composes them on the parent element
or via a tailwind/CSS-in-JS layer, not by funnelling a `format!("{}{}",
…)` through `NavLink`. The 99% case is a literal class name; making
literals zero-cost is the right trade.

For the breadcrumb chain and other components where class composition is
unavoidable, those props use `Classes` directly. The asymmetry is
deliberate: `NavLink` is the hot path, breadcrumbs are not.

## Consequences

**Positive**

- Zero allocation per render for the common case.
- Compile-time enforcement that class names are constants — a `String`
  built from user input cannot land in a CSS class slot, eliminating a
  class of HTML-injection footguns at the type level.
- `NavLinkProps: Copy`-ish: every field is trivially clonable, making
  `#[derive(Clone, PartialEq)]` cheap.

**Negative**

- A consumer who needs runtime class composition on `NavLink`
  specifically cannot do it through the prop. They must either lift the
  class to the parent or wrap `NavLink` in their own component.
- The breadcrumb-vs-NavLink asymmetry is a small surprise for new
  readers and is documented in `docs/ARCHITECTURE.md`.

**Cost to reverse**

Moderate. Switching to `AttrValue` is a semver-breaking signature change
across the public props of `NavLink`. The internal `build_class` helper
would also need to grow an allocation path. No data migration; consumer
upgrade is one-line per call site.
