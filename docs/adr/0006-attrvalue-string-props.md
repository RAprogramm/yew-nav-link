<!--
SPDX-FileCopyrightText: RAprogramm <andrey.rozanov.vl@gmail.com>
SPDX-License-Identifier: MIT
-->

# 0006 — String props use `AttrValue`

- **Status:** accepted
- **Date:** 2026-07-11
- **Supersedes:** [0001](0001-static-str-classes.md)

## Context

ADR 0001 fixed `NavLink`'s `class` / `active_class` as `&'static str`,
trading runtime composition away for zero-allocation literals. The same
type then spread by imitation to every string-ish prop in the crate:
`id`, `text`, `toggle_text`, `href`, `variant`, `role`, `aria_label`.

Two things changed the balance:

1. **The zero-cost argument no longer discriminates.** Yew's `AttrValue`
   is a two-variant handle: `AttrValue::Static(&'static str)` — the
   exact same pointer ADR 0001 optimized for — or `Rc<str>` for runtime
   strings. `html!` converts string literals into `AttrValue::Static`
   with no allocation, so the 99% literal case costs the same as before.
   Yew's own documentation recommends `AttrValue` over `String`/`&str`
   for props.
2. **The restriction turned out to bite in practice.** `PageLink`'s
   `href: Option<&'static str>` makes a pagination link's URL impossible
   to build at runtime — `format!("/page/{n}")` cannot become
   `&'static str` without leaking. What was a stylistic trade for class
   slots is a functional hole for URL slots.

## Decision

All string-valued props take `AttrValue` (`Option<AttrValue>` when
optional). Defaults use `AttrValue::Static`, keeping the literal path
allocation-free. Class-list props that benefit from composition and
deduplication stay `Classes`, as before.

## Consequences

**Positive**

- Runtime values work everywhere they should — most importantly
  `PageLink::href`.
- Props match the idiom the Yew book teaches, so consumers coming from
  other Yew code find no surprises.
- Cloning props stays cheap: `AttrValue` clones are a pointer copy
  (`Static`) or an `Rc` refcount bump.

**Negative**

- The type-level guarantee that a class name is a compile-time constant
  is gone; a consumer can now funnel a runtime string into a class slot.
  The escaping behaviour of the DOM layer still prevents HTML injection.
- One-time semver break: struct-literal constructions of props must wrap
  literals in `AttrValue::Static` (in `html!` nothing changes).

**Cost to reverse**

Moderate and symmetric to ADR 0001's estimate: signature changes across
public props, one-line fixes per call site.
