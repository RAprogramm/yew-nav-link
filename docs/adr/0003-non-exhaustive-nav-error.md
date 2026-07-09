<!--
SPDX-FileCopyrightText: 2024-2026 RAprogramm <andrey.rozanov.vl@gmail.com>
SPDX-License-Identifier: MIT
-->

# 0003 — `NavError` is `#[non_exhaustive]` from 0.10.0

- **Status:** accepted
- **Date:** 2026-05-10
- **Deciders:** RAprogramm

## Context

`NavError` carries the failure cases the navigation layer surfaces to
consumers:

```rust
pub enum NavError {
    RouteNotFound,
    InvalidRoute(String),
    NavigationCancelled,
}
```

Through 0.9.x the enum was plain — consumers could write exhaustive
`match` expressions without a wildcard arm. Pleasant in the short term;
a semver bear-trap in the long term, because *adding* a variant is then
a breaking change.

The ROADMAP slates several future variants: redirect cancellation
distinct from generic cancellation, permission-denied (when the consumer
wires up a route guard), timeout. Each addition under plain enum
semantics would force a major bump.

## Decision

Mark `NavError` `#[non_exhaustive]` as part of the 0.10.0 breaking-change
pass.

```rust
#[non_exhaustive]
pub enum NavError {
    RouteNotFound,
    InvalidRoute(String),
    NavigationCancelled,
}
```

Foreign-crate exhaustive matches must add a wildcard `_ =>`. Internal
matches inside `yew-nav-link` are unaffected — `#[non_exhaustive]` does
not apply within the defining crate.

## Consequences

**Positive**

- New variants ship under semver-minor bumps. No further major bumps for
  this reason alone.
- Forces consumers to confront the "what if a new error appears"
  question at compile time, which is the right place for it.

**Negative**

- One-time mechanical migration for every consumer that exhaustively
  matched `NavError`. The compiler points exactly at the missing arm.
- Wildcard arms can hide forgotten handling. Mitigated by recommending
  `_ => unreachable!("unknown NavError variant: {err:?}")` for
  developer-built handlers that want loud failures.

**Cost to reverse**

High. Dropping `#[non_exhaustive]` is itself a non-trivial change:
consumers may rely on it to keep their wildcard arms exhaustive-by-fiat,
and removing it would be a semver-minor that *enables* exhaustive
matching but does not require it. We do not expect to reverse.

## References

- `src/errors.rs:65` — the `#[non_exhaustive]` attribute.
- CHANGELOG `[0.10.0]` — "Breaking changes".
