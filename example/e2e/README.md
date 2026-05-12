<!--
SPDX-FileCopyrightText: 2024-2026 RAprogramm <andrey.rozanov-vl@gmail.com>
SPDX-License-Identifier: MIT
-->

# End-to-end tests

Playwright suite that drives a real browser against the `yew-nav-link`
demo. The browser tests under `tests/wasm/` exercise component-level
behaviour in isolation; this suite exercises the assembled demo end to
end and is what catches regressions a unit test cannot see — wiring
mistakes, missing CSS, breakages in the router/breadcrumb integration.

## Running locally

The suite expects a built demo in `../dist/`. Two commands:

```bash
# build the demo (release profile picked to match what gh-pages ships)
cd example
trunk build --release

# then run the suite — playwright spins up a static server itself
cd e2e
npm install
npx playwright install --with-deps chromium firefox
npm test
```

The `playwright.config.ts` `webServer` block invokes
`npx serve ../dist --single` on port 4173 and waits for it to come up
before tests start. `--single` is important — without it the static
server would 404 on every client-routed path (`/components`,
`/hooks/team/platform`, …) instead of falling back to `index.html` so
the SPA can take over.

## Layout

```
example/e2e/
  playwright.config.ts   single project for chromium + firefox
  tests/
    navigation.spec.ts   top-nav click, back/forward, partial-match
    dropdown.spec.ts     NavDropdown open/close on toggle click
    breadcrumbs.spec.ts  use_breadcrumbs + BreadcrumbLabelProvider
    pagination.spec.ts   Pagination active-page marker
```

## What is covered, what is not

Covered:

- `NavLink` activates on click, advertises `aria-current="page"`, and
  retains the active class across back/forward.
- `partial=true` keeps the parent active under a nested route.
- `NavDropdown` toggles `aria-expanded` and the `.open` class.
- The breadcrumb trail renders one `[aria-current="page"]` entry whose
  text comes from `BreadcrumbLabelProvider`, not the raw slug.
- `Pagination` renders exactly one `.pagination-item.active`.

Not yet covered, tracked in follow-up issues:

- `NavDropdown` close on outside-click and on Escape — the component
  does not handle those events today; tests will land alongside the
  feature.
- `<NavList>` keyboard navigation (Arrow / Home / End) — the demo does
  not surface a keyboard-driven list yet, so there is no stable
  selector to anchor a test on.

## CI

`.github/workflows/ci.yml` runs the `E2E` job on every PR after the
`Example WASM build` job: it reuses the built `example/dist/` artefact,
installs Playwright with its browser dependencies, and runs the suite
on chromium and firefox in headed-headless mode.
