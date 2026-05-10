<!--
SPDX-FileCopyrightText: 2024-2026 RAprogramm <andrey.rozanov-vl@gmail.com>
SPDX-License-Identifier: MIT
-->

# Architecture

This is a design rationale, not a tutorial. For the *what*, read the
[crate docs](https://docs.rs/yew-nav-link). For the *why*, read on.

## 1. Crate layout

```text
src/
├── lib.rs            Public API and re-exports. The crate root re-exports
│                     every type a typical consumer will reach for.
├── active_link/      The NavLink<R> component, the Match enum, and the
│   ├── mod.rs        per-render utility that decides the active state.
│   ├── nav_link.rs
│   ├── props.rs
│   ├── mode.rs
│   └── utils.rs
├── nav/              Structural primitives that don't care about routing:
│                     NavList, NavItem, NavDivider. These are render-only.
├── components/       Higher-level UI components built on top of nav/:
│                     badges, dropdowns, headers, icons, tabs, pagination,
│                     etc.
├── hooks/            Reactive hooks. Split into route_info/ (read-only
│                     state) and navigation/ (effects + query params).
├── utils/            Pure functions (paths, URL codec, keyboard helpers).
│                     No yew dependency.
├── attrs.rs          Type-safe attribute builders for consumers who roll
│                     their own elements.
└── errors.rs         NavError + NavResult<T>.
```

The split keeps three rules invariant:

- **`utils/` is leaf** — no dependency on yew. It is unit-testable on its
  own and could be carved into a sibling crate later without churn.
- **`active_link/` is the only place that knows about active-state
  matching.** Hooks delegate to the same primitive (`is_path_prefix`).
- **Components never own routing state.** `NavTabs` and `Pagination` accept
  the active index / page through props; the consumer holds the
  `use_state`. The library is render-only above the routing layer.

## 2. The active-state algorithm

`NavLink<R>` and the active-state hooks all answer one question: given a
target route `to: R` and the currently matched route, is `to` *active*?

```rust
fn is_active<R: Routable + PartialEq>(
    current: Option<R>,
    target: &R,
    partial: bool,
) -> bool {
    let Some(current) = current else { return false };
    if partial {
        is_path_prefix(&target.to_path(), &current.to_path())
    } else {
        &current == target
    }
}
```

`is_path_prefix(prefix, full)` is **segment-wise**: `/docs` is a prefix of
`/docs/api` but not of `/documentation`. We compute it by splitting both on
`/`, dropping empty fragments, and checking that the prefix's segments
match the head of `full`'s segments.

Why segment-wise? Because string-prefix matching has a long history of
false positives (e.g. `/admin` matching `/administrator`) and yew-router's
`Routable` enum gives us proper `to_path()` strings to work with.

## 3. The hook contract

Every hook reads from yew-router's reactive state via `use_route::<R>()`
and returns a *value*, not a callback registration. They re-render their
caller when the URL changes; that is the entire integration point.

```text
URL changes ──► yew-router state updates ──► use_route()
                                                │
                                                ├── use_is_active(...)
                                                ├── use_is_exact_active(...)
                                                ├── use_is_partial_active(...)
                                                ├── use_breadcrumbs()
                                                └── use_route_info()
```

`use_navigation()` is the dual: pure outputs that *write* to the URL via
yew-router's `Navigator`. It hands back ready-made `Callback<()>` so
consumers don't have to repeat the `Callback::from(move |_| ...)` boiler
plate.

## 4. Custom breadcrumb labels

`use_breadcrumbs` walks the current path and synthesises a list of
`BreadcrumbItem<R>` entries — but the **label** for each entry is delegated
to a `BreadcrumbLabelProvider` trait. The provider is injected through
Yew's context system using a public newtype:

```text
                                ┌─────────────────────────────────┐
                                │ BreadcrumbLabelProviderContext  │  newtype around
                                │   ├── Rc<dyn ...Provider>       │  Rc<dyn Provider>
                                └────────────┬────────────────────┘
                                             │
        <ContextProvider<BreadcrumbLabelProviderContext> context={…}>
                          <App/>            │
                            ...             │
                            use_breadcrumbs() ──► reads ctx ──► label_for_path()
```

Why a newtype rather than putting `Rc<dyn Provider>` directly into context?
Because yew-router's reactive context needs `PartialEq`, and we implement
that as `Rc::ptr_eq` so re-renders only happen when the *concrete provider
value* changes, not on every render where the provider is re-created via
`Rc::clone`.

This is the only stateful pattern in the crate; everything else is pure
props.

## 5. What we don't use, and why

- **No `unsafe`.** There is no FFI surface and no performance hot-path that
  would justify it.
- **No `no_std`.** Yew requires `std` (allocations, `Rc`, threading
  primitives behind WASM); the CI `no_std` job documents this and exits
  successfully so a no-std intention is impossible to merge by accident.
- **No async.** The hooks are synchronous; `use_navigation` returns
  callbacks and yew-router does the rest.
- **No internal `RefCell` / `Rc` mutability** beyond the breadcrumb
  provider context, which is logically immutable per app lifecycle.

## 6. Test layout

```text
tests/                                 # integration tests, Yew-aware
benches/                               # criterion benchmarks (path utils)
src/**/mod.rs (#[cfg(test)] modules)   # unit tests, focused per module
```

Integration tests live in `tests/` so they run against the public API
exactly as a consumer would write code; this catches accidental breakage
in re-exports that unit tests would miss.

## 7. Demo crate (`example/`)

The demo is a `cdylib` SPA served by trunk. It is intentionally **not** a
workspace member of the library crate — its only purpose is to demonstrate
the public API end-to-end and to be deployed to GitHub Pages.

The demo is structured around a `DemoCard` component: every public
component, hook, and utility appears in at least one card whose live
preview is rendered side-by-side with the exact Rust snippet that
produces it. The breadcrumb label provider is mounted at the top of the
tree so `use_breadcrumbs` has a real provider to read.
