<!--
SPDX-FileCopyrightText: 2024-2026 RAprogramm <andrey.rozanov-vl@gmail.com>
SPDX-License-Identifier: MIT
-->

# Release process

`yew-nav-link` releases are driven by [release-plz][rplz]. Conventional
commits land on `main`; release-plz reads them, opens a "release PR"
that bumps `Cargo.toml` and prepends a section to `CHANGELOG.md`, and
when that PR is squash-merged it tags the commit, publishes to
crates.io, and creates the GitHub release.

There is no manual version-bump path. The maintainer's only release
action is to review and merge the release PR.

[rplz]: https://release-plz.dev

## Versioning policy

The crate follows
[Semantic Versioning 2.0.0](https://semver.org/spec/v2.0.0.html) with
Cargo's `0.x` interpretation:

- While the major is `0`, **breaking changes** bump the minor
  (`0.x → 0.(x+1)`).
- **Additive, non-breaking** changes (new public exports, bug fixes)
  bump the patch (`0.x.y → 0.x.(y+1)`).

After 1.0 the standard major/minor/patch rules apply.

`Semver Checks` (`cargo semver-checks`) runs on every PR and blocks a
release whose API change exceeds the version bump. release-plz reads
the same conventional commits to decide the bump, so by the time a
release PR exists the version and the diff already agree.

## How a release happens

```
conventional commit on main
        │
        ▼
release-plz-pr job (workflow: Release-plz)
        │
        ▼
release PR — open or updated, with Cargo.toml + CHANGELOG diff
        │
        ▼  (maintainer reviews, CI runs, semver gate passes)
        │
        ▼
squash-merge to main
        │
        ▼
release-plz-release job
        │
        ├─► git tag vX.Y.Z (annotated)
        ├─► cargo publish
        └─► gh release create with body from the CHANGELOG section
```

The trigger is `push` to `main`; both jobs live in
`.github/workflows/release-plz.yml`. The `release-pr` job keeps the
release PR fresh after each commit; the `release` job runs after the
release PR's merge commit lands.

## Configuration

- `release-plz.toml` — per-package settings: `git_tag_name = "v{{ version }}"`, `git_release_name = "v{{ version }}"`, changelog mapping that mirrors `cliff.toml` (`feat` → Features, `fix` → Bug Fixes, …).
- `.github/workflows/release-plz.yml` — the two jobs. Concurrency is keyed on `release-plz-${{ github.ref }}` so two pushes never race a release PR update.

The `CRATES_IO_TOKEN` repository secret is reused from the previous
manual flow. Pull-request and release operations authenticate as a
dedicated GitHub App: the workflow mints a short-lived installation
token via `actions/create-github-app-token` from
`RELEASE_PLZ_APP_ID` + `RELEASE_PLZ_APP_PRIVATE_KEY` and hands it to
release-plz. Setup is documented in
[`docs/RELEASE_PLZ_APP_SETUP.md`](docs/RELEASE_PLZ_APP_SETUP.md) — the App
permissions are exactly `Contents: read & write` and `Pull requests:
read & write`, scoped to this single repository. The App identity is
required, not optional: PRs and releases created by the default
`GITHUB_TOKEN` do not trigger downstream workflows, breaking the
autonomous release loop.

## What the maintainer still does

1. **Land conventional commits on `main`.** Prefix decides changelog
   group and version bump:

   | Prefix | Section | Bump |
   |---|---|---|
   | `feat` | Features | minor (or major with `feat!`) |
   | `fix` | Bug Fixes | patch |
   | `docs` | Documentation | patch |
   | `refactor` | Refactoring | patch |
   | `ci` | CI | patch |
   | `deps`, `chore(deps)` | Dependencies | patch |
   | `test`, `chore` | (skipped) | none |

2. **Wait for the release PR.** It appears within a minute of the
   triggering push (release-plz reuses the same PR across pushes).

3. **Review and merge it.** Squash-merge, deletion of the release-plz
   branch is automatic.

4. **Verify.** Within a minute of the merge:
   - <https://crates.io/crates/yew-nav-link> shows the new version.
   - <https://docs.rs/yew-nav-link> rebuilds (a few extra minutes).
   - <https://github.com/RAprogramm/yew-nav-link/releases> has a
     populated entry whose body is the CHANGELOG section for the
     version.

   If any of these is missing, inspect the most recent run of the
   `Release-plz` workflow on `main`.

## Yanking

If a release ships a regression severe enough that consumers should
**not** pin it, yank it on crates.io:

```bash
cargo yank --version X.Y.Z
```

Document the reason in `CHANGELOG.md` under `### Security` or `### Fixed`
of the next release. release-plz will pick the entry up on the next
push.

## Pre-release / RC versions

The 0.9.x and 0.10.x series do not currently use pre-release tags.
When 1.0 approaches, candidate cuts will use the standard
`1.0.0-rc.N` form, which Cargo resolves correctly under
`cargo install --version "<1.0"` semantics. release-plz supports
pre-releases via `version_groups` in `release-plz.toml`; the config
will gain that block at the 1.0 cut.

## Backfilling release notes

Releases produced before PR #51 had empty bodies because the workflow
used the deprecated `actions/create-release@v1`. release-plz only
manages future releases — to repopulate an older one:

```bash
notes=$(mktemp)
awk -v v="0.9.2" '
  BEGIN { in_section = 0 }
  /^## \[/ {
    if (in_section) { exit }
    if ($0 ~ "^## \\[" v "\\]") { in_section = 1; next }
  }
  in_section { print }
' CHANGELOG.md > "$notes"
gh release edit "v0.9.2" --notes-file "$notes"
rm "$notes"
```
