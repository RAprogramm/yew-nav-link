<!--
SPDX-FileCopyrightText: 2024-2026 RAprogramm <andrey.rozanov-vl@gmail.com>
SPDX-License-Identifier: MIT
-->

# 0005 — Active `NavLink` emits `aria-current="page"`

- **Status:** accepted
- **Date:** 2026-05-10
- **Deciders:** RAprogramm

## Context

Through 0.9.x, active `NavLink`s were distinguished by a CSS class only
(`active` by default, overridable via `active_class`). This is enough
for sighted users — the visual state is conveyed by stylesheet — and
zero for assistive technology, which has no semantic signal that one of
the links in a navigation list is the current page.

The web platform has a dedicated answer:
[`aria-current`][aria-current]. The value `"page"` declares that the
link points at the current document; screen readers (NVDA, VoiceOver,
JAWS) announce it as "current page" or equivalent. This is the WAI-ARIA
authoring guideline for navigation menus.

[aria-current]: https://www.w3.org/TR/wai-aria-1.2/#aria-current

## Decision

Active `NavLink`s emit `aria-current="page"` in addition to the active
class:

```rust
let aria_current = if is_active { Some("page") } else { None };
html! { <a class={class} href={href} aria-current={aria_current} ...> }
```

The attribute is omitted entirely (not set to a falsy value) when the
link is inactive, so the rendered DOM is minimal on non-current links.

## Consequences

**Positive**

- Screen readers correctly identify the current page in navigation menus.
- CSS authors get a second hook for active-state styling that does not
  depend on the class name configuration — `[aria-current="page"] { … }`
  works regardless of whether the consumer overrode `active_class`.
- Aligns with WAI-ARIA Authoring Practices for navigation patterns;
  Lighthouse accessibility audits credit the attribute.

**Negative**

- Behaviour change in the rendered DOM. Consumers whose existing CSS
  *only* targets `.active` continue to work. Consumers whose CSS
  matched `[aria-current]` for unrelated reasons may see new matches.
  Documented in CHANGELOG `[0.10.0]` under "Breaking changes" so the
  surprise is visible.
- A small amount of attribute churn during re-renders. Negligible —
  Yew's DOM diffing handles the toggle cleanly.

**Cost to reverse**

Low. Dropping the attribute is a one-line change. We do not expect to
reverse because the standard is clear and the accessibility benefit is
unconditional.

## References

- ADR 0004 — Manual `<a>` rendering enabled this attribute addition.
- `src/active_link/nav_link.rs` — the `aria_current` binding.
- CHANGELOG `[0.10.0]` — under "Breaking changes" and "Added".
