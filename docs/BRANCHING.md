<!--
SPDX-FileCopyrightText: 2024-2026 RAprogramm <andrey.rozanov.vl@gmail.com>
SPDX-License-Identifier: MIT
-->

# Branching and merge policy

This document is the source of truth for how changes reach `main`. It
codifies what `.github/settings.yml` enforces at the GitHub layer and how
`CONTRIBUTING.md` instructs contributors to interact with it.

## 1. The default branch

`main` is the only long-lived branch. Every release ships from it.

Protection rules (set by `.github/settings.yml`):

| Rule | Value |
|---|---|
| Required status check | `CI Success` (aggregate of every job in `.github/workflows/ci.yml`) |
| Strict status checks | yes — PR must be up to date with `main` |
| Linear history | yes — no merge commits |
| Force pushes | blocked |
| Branch deletion | blocked |
| Approving reviews required | 1 (CODEOWNERS) |
| Stale review dismissal | yes |

There is no `develop`, no `release/*`, no `hotfix/*`. Earlier versions of
`settings.yml` mention a `develop` branch; that is legacy configuration and
is not used in practice.

## 2. Feature branches

One branch per GitHub issue, named after the issue number:

```bash
git checkout -b 123
```

No prefixes (`feat/`, `feature/`, `bugfix/`), no descriptive slugs. The
issue is the source of truth for what the branch is about; the title and
body of the PR carry the human-readable framing.

A branch may carry multiple commits — the squash-merge collapses them into
a single line on `main`. Keep commits small and focused; rebase locally to
clean up before pushing if needed.

## 3. Commit format

Every commit message starts with the issue reference and a type prefix:

```
#<issue> <type> <description>
```

No colon after the type, no `(scope)`. `cliff.toml` rewrites this form
into a conventional-commit subject (`<type>: <description>`) before
git-cliff classifies it, so the prefixes still decide which CHANGELOG
section the commit lands in:

| Prefix | CHANGELOG section | Triggers minor/patch bump under release-plz |
|---|---|---|
| `feat` | Features | minor |
| `fix` | Bug Fixes | patch |
| `docs` | Documentation | patch |
| `refactor` | Refactoring | patch |
| `ci` | CI | patch |
| `deps`, `chore(deps)` | Dependencies | patch |
| `test` | (skipped) | none |
| `chore` | (skipped) | none |

Breaking changes carry a `!` after the type (`feat!`, `refactor!`) and
trigger a major bump.

## 4. Merge style

**Squash merge, delete branch.** Configured in `.github/settings.yml`:

```yaml
allow_squash_merge: true
allow_merge_commit: false
allow_rebase_merge: true
delete_branch_on_merge: true
squash_merge_commit_title: PR_TITLE
squash_merge_commit_message: PR_BODY
```

Rebase merge is allowed for the rare case where individual intermediate
commits carry value on their own (e.g. a sequence of independent refactors
that should be bisectable). Default is squash: one PR = one commit on
`main`, with the PR title becoming the conventional-commit subject and the
PR body becoming the commit message.

Force pushes to `main` are blocked at the GitHub layer; this is not a
convention, it is a hard wall.

## 5. Pull request

| Field | Requirement |
|---|---|
| Title | Issue number only (e.g. `123`). The squash commit subject inherits this. |
| Body | Must include `Closes #123` so the issue auto-closes on merge. |
| Status checks | `CI Success` must be green. |
| Reviews | 1 approving review from a `CODEOWNERS` entry (currently `@RAprogramm`). |
| Up-to-date | Strict mode is on — rebase onto `main` if it has advanced since CI ran. |

The PR body is what readers of the commit history will see on `main`, so
write it for that audience, not just the reviewer.

## 6. Reverting

A bad change is rolled back with a revert PR, never with a force-push or
history rewrite:

```bash
git revert <sha>
git checkout -b <new-issue>
git push -u origin <new-issue>
gh pr create --title "<new-issue>" --body "Closes #<new-issue>"
```

The revert commit follows the same `#<issue> revert <description>` form,
and the new issue documents *why* the original change was rolled back.

## 7. Releases

Releases happen on `main`. There is no release branch.

The bump → publish flow is documented in [`RELEASE.md`](../RELEASE.md). In
short: a release PR (today hand-edited, eventually opened by `release-plz`)
bumps `Cargo.toml` `version` and prepends a section to `CHANGELOG.md`;
merging it triggers the publish job. Every release tag is annotated and
signed.

## 8. References

- [`CONTRIBUTING.md`](../CONTRIBUTING.md) — full contributor workflow.
- [`RELEASE.md`](../RELEASE.md) — release procedure.
- [`.github/settings.yml`](../.github/settings.yml) — enforced settings.
- [`cliff.toml`](../cliff.toml) — conventional commit → CHANGELOG mapping.
