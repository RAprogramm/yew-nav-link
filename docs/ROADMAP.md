<!--
SPDX-FileCopyrightText: 2024-2026 RAprogramm <andrey.rozanov.vl@gmail.com>
SPDX-License-Identifier: MIT
-->

# Roadmap

Public plan toward stabilising `yew-nav-link` at 1.0. Anything here is a
target, not a promise; the [GitHub milestones][milestones] track the
authoritative cutline of each release.

[milestones]: https://github.com/RAprogramm/yew-nav-link/milestones

## 0.9.x — previous line (closed)

**Released.** Status: closed; no further patches. The active line is 0.10.x.

- 0.9.0 dropped the macros feature.
- 0.9.1 replaced the multi-page demo with a single-file SPA, fixed the
  `use_breadcrumbs` segment-loop shadow binding, and SPDX-tagged every
  source file.
- 0.9.2 surfaced `BreadcrumbLabelProvider` at the crate root.
- 0.9.3 made `BreadcrumbLabelProviderContext` part of the public API so
  consumers can actually inject a provider (the trait alone was reachable
  but inert).
- 0.9.4 fixed `normalize_path` `.`/`..` resolution, made
  `urlencoding_decode` UTF-8 aware, and stopped `pagination_page` panics
  on adversarial inputs.

## 0.10.x — current line

**Released 2026-05-10.** Status: maintained.

The breaking-change pass before 1.0 — four targeted breakages consumers
upgrade through in one hop:

- `NavError` is `#[non_exhaustive]`, leaving room for new variants under
  semver-minor.
- `BreadcrumbLabelProviderContext`'s tuple field is private; construct
  via `::new` and read via `.provider()`.
- The orphan `route_params.rs` module is removed; migrate to
  `use_route::<R>()` with a `Routable` enum.
- Active `NavLink` emits `aria-current="page"` on the rendered `<a>` for
  screen-reader-correct active state.

MSRV review for the line completed with no bump: stable Rust 1.95 was
the latest stable when 0.10 was cut and remained so through the window
(see closed issue #71).

Subsequent 0.10.x patches address CI, dependency bumps, and
documentation only — no public API changes.

## 1.0.0 — API freeze

**Target window:** earliest 2026-08, after 0.10.x has spent at least
one quarter in the ecosystem with no public API changes. **Status:**
dependent on 0.10 feedback.

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
