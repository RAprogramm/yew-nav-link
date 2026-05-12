<!--
SPDX-FileCopyrightText: 2024-2026 RAprogramm <andrey.rozanov-vl@gmail.com>
SPDX-License-Identifier: MIT
-->

# Release process

`yew-nav-link` releases are driven by the version field in `Cargo.toml`.
Push a commit to `main` whose `Cargo.toml` carries a higher version than
the latest git tag, and CI does the rest: it publishes to crates.io,
tags the commit, and creates a GitHub release whose body comes from the
matching `CHANGELOG.md` section.

## Versioning policy

The crate follows
[Semantic Versioning 2.0.0](https://semver.org/spec/v2.0.0.html) with
Cargo's `0.x` interpretation:

- While the major is `0`, **breaking changes** bump the minor
  (`0.x → 0.(x+1)`).
- **Additive, non-breaking** changes (new public exports, bug fixes) bump
  the patch (`0.x.y → 0.x.(y+1)`).

After 1.0 the standard major/minor/patch rules apply.

## Cutting a release

1. **Decide the new version.** Look at the merged work since the last
   tag. Anything in `[Unreleased]` of `CHANGELOG.md` that changes the
   public API forces a minor bump in the 0.x line.

2. **Open an issue** named `Release vX.Y.Z` (per the standard
   `CONTRIBUTING.md` workflow). Branch named after the issue.

3. **Edit `Cargo.toml`** — set `version = "X.Y.Z"`.

4. **Update `CHANGELOG.md`.** Replace the `## [Unreleased]` heading with
   `## [Unreleased]` followed by `## [X.Y.Z] - YYYY-MM-DD` and move the
   relevant entries underneath. Group entries under the standard
   `Added` / `Changed` / `Deprecated` / `Removed` / `Fixed` / `Security`
   headings (see [Keep a Changelog](https://keepachangelog.com/en/1.1.0/)).

5. **Refresh `Cargo.lock`** by running any cargo command (`cargo check`).
   Commit the lock-file update separately to keep the diff readable:

   ```
   #N feat: <feature>
   #N chore: refresh Cargo.lock for X.Y.Z version bump
   ```

6. **Open a PR**, wait for `CI Success` to pass.

   The `Semver Checks` job runs `cargo semver-checks check-release`
   against the previously-published crates.io version. If your changes
   touch the public API in a way that does not match the version bump
   in step 3, this job fails — re-decide the version (a wider bump or
   smaller change) and make the PR consistent. Update both directions
   here, never bypass the gate with `--allow-...` flags.

7. **Squash-merge to `main`.** That push triggers the `release` job in
   `.github/workflows/ci.yml`, which:

   1. Reads the `version` value from `Cargo.toml`.
   2. Compares against the latest tag (`git describe --tags --abbrev=0`).
   3. Runs `cargo build --release --all-features` as a final smoke test.
   4. Publishes via `cargo publish` using `CRATES_IO_TOKEN`.
   5. Extracts the matching `[X.Y.Z]` section from `CHANGELOG.md` via
      `awk` into a temp file (see the **Extract release notes** step).
   6. Calls `softprops/action-gh-release@v2` with the temp file as
      `body_path` and `generate_release_notes: true` so GitHub appends
      the auto-generated PR list under our handwritten changelog.
   7. Tags the commit `vX.Y.Z`.

8. **Verify.** Within a minute:

   - <https://crates.io/crates/yew-nav-link> shows the new version.
   - <https://docs.rs/yew-nav-link> rebuilds (may take a few extra
     minutes).
   - <https://github.com/RAprogramm/yew-nav-link/releases/tag/vX.Y.Z>
     exists with a populated body.

   If any of these is missing, check the `release` job in the
   most-recent `CI` workflow run on `main`.

## Backfilling release notes

Releases produced before PR #51 had empty bodies because the workflow
used the deprecated `actions/create-release@v1`. To repopulate one:

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

## Yanking

If a release ships a regression severe enough that consumers should
**not** pin it, yank the version on crates.io:

```bash
cargo yank --version X.Y.Z
```

Document the reason in `CHANGELOG.md` under `### Security` or `### Fixed`
of the next release.

## Pre-release / RC versions

The 0.9.x and 0.10.x series do not currently use pre-release tags.
When 1.0 approaches, candidate cuts will use the standard
`1.0.0-rc.N` form, which Cargo resolves correctly under
`cargo install --version "<1.0"` semantics.
