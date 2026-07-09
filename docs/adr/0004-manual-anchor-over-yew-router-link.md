<!--
SPDX-FileCopyrightText: 2024-2026 RAprogramm <andrey.rozanov.vl@gmail.com>
SPDX-License-Identifier: MIT
-->

# 0004 — Render a manual `<a>` instead of wrapping `yew_router::Link`

- **Status:** accepted
- **Date:** 2026-05-10
- **Deciders:** RAprogramm

## Context

Through 0.9.x `NavLink` was a thin wrapper around `yew_router::Link`. The
wrapper computed the active state, then handed off rendering to `Link`,
which produced the `<a>` element and intercepted clicks.

Two needs piled up against this layering:

1. **Attribute control.** Active links should emit `aria-current="page"`
   (ADR 0005), and the rendered set of attributes should be fully under
   our control for future additions (e.g. `data-active`, custom event
   handlers, `download` for export links). `yew_router::Link` does not
   expose hooks for the attribute set it renders.
2. **Basename handling for sub-path deployments.** The demo deploys at
   `https://raprogramm.github.io/yew-nav-link/`, i.e. under a non-empty
   basename. `Link` renders `href` from `to.to_path()` directly, which
   produces visually wrong hrefs on hover (`/about` instead of
   `/yew-nav-link/about`). The demo worked around this with a runtime
   `<base>` hack — fragile and only addresses the symptom on that one
   site.

## Decision

From 0.10.0, render the `<a>` manually:

- `href` is built from `to.to_path()` prepended with the navigator's
  basename when present.
- An `onclick` handler intercepts left-clicks (and only left-clicks —
  modifier-clicks fall through to the browser for "open in new tab")
  and pushes the route via the captured `Navigator`.
- The full attribute set is under the component's control.

Modifier-click behaviour matches `Link` exactly: `meta`, `ctrl`, `shift`,
`alt`, and middle-click skip `prevent_default` so the browser takes the
default route (new tab, save target, etc.).

## Consequences

**Positive**

- Correct `href` under any basename, with no runtime workaround.
- `aria-current="page"` lands on active links (ADR 0005).
- Future attribute additions are a one-line change inside `NavLink`'s
  `html!` instead of a router-layer feature request.
- One fewer indirection in the render tree.

**Negative**

- We now own a small navigation-affordance contract that `yew_router`
  used to maintain. If `yew_router` changes how `Navigator::push`
  interacts with the browser's `popstate`, we need to track that.
  Mitigated by exhaustive integration tests (issue #113, #114).
- The wrapper grows from ~10 lines to ~50. Acceptable: the additional
  lines are intent-revealing.

**Cost to reverse**

Low. Reverting to a `Link` wrapper is a localised change in
`src/active_link/nav_link.rs`. Consumer-visible behaviour for the
non-basename case is identical, so a revert ships as a patch release.

## References

- `src/active_link/nav_link.rs` — the manual implementation.
- CHANGELOG `[0.10.0]` — "Changed: NavLink is rendered as a manual `<a>`".
