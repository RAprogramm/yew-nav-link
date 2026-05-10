<!--
SPDX-FileCopyrightText: 2024-2026 RAprogramm <andrey.rozanov-vl@gmail.com>
SPDX-License-Identifier: MIT
-->

# Roadmap

Public plan toward stabilising `yew-nav-link` at 1.0. Anything here is a
target, not a promise; the [GitHub milestones][milestones] track the
authoritative cutline of each release.

[milestones]: https://github.com/RAprogramm/yew-nav-link/milestones

## 0.9.x — current line (patch series)

**Released.** Status: maintained.

- 0.9.0 dropped the macros feature.
- 0.9.1 replaced the multi-page demo with a single-file SPA, fixed the
  `use_breadcrumbs` segment-loop shadow binding, and SPDX-tagged every
  source file.
- 0.9.2 surfaced `BreadcrumbLabelProvider` at the crate root.
- 0.9.3 made `BreadcrumbLabelProviderContext` part of the public API so
  consumers can actually inject a provider (the trait alone was reachable
  but inert).

The 0.9.x line continues to receive **patch-level fixes only**: bug fixes,
documentation, demo, CI, and dependency bumps. No public API changes land
on 0.9.x.

## 0.10.0 — breaking-change pass

**Target window:** before 1.0 freeze. **Status:** planned.

This is where every accumulated breaking change ships at once, so
consumers upgrade in a single hop:

- **Drop the unused `route_params.rs` module.** It exists privately,
  isn't re-exported, and is documented as removed in CHANGELOG. Cleaning
  it out is a tree-shake, not a feature loss.
- **Library-level `aria-current="page"` on active `NavLink`.** Today the
  demo wraps active items in `aria-current` manually. Embedding it in
  `NavLink` itself fixes accessibility once for every consumer; it
  changes the rendered HTML, hence the breaking bump.
- **Tighter MSRV review.** 1.95 was the latest stable when the line was
  cut; 0.10 will pin to whichever stable Rust ships within the
  development window.
- **Audit `prop_or_default` defaults** across components for consistency
  before they freeze at 1.0.

## 1.0.0 — API freeze

**Target window:** after 0.10 has spent at least one quarter in the
ecosystem with no API changes. **Status:** dependent on 0.10 feedback.

The 1.0 commitment is small and deliberately boring:

- **Public API freeze.** Every name re-exported from `lib.rs` becomes a
  semver-stable surface; subsequent breaking changes require a 2.0.
- **Documented backwards-compatibility window** in `SECURITY.md` and
  `docs/REQUIREMENTS.md`: how long 1.x receives security patches.
- **Migration guide** from 0.x in `CHANGELOG.md` for the major bump.

Nothing about 1.0 is meant to be flashy. It is the version we keep
shipping for years.

## Beyond 1.0 — speculative

Tracked in [GitHub Discussions][discuss], not committed to:

- SSR support: yew-router supports it, but the active-state hooks have
  not been tested under SSR.
- Optional `axum-router`-style integration helpers for projects that
  generate their `Routable` enum from a backend.
- A standalone `aria-current` consumer hook for projects that build their
  own link components but want yew-nav-link's matching algorithm.

[discuss]: https://github.com/RAprogramm/yew-nav-link/discussions

## How to influence the roadmap

Open an issue with the `enhancement` label and a clear use case. Concrete,
narrow proposals beat large redesigns; we will steer toward what existing
consumers actually need.
