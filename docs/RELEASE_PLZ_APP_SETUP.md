<!--
SPDX-FileCopyrightText: 2024-2026 RAprogramm <andrey.rozanov-vl@gmail.com>
SPDX-License-Identifier: MIT
-->

# release-plz GitHub App setup

`.github/workflows/release-plz.yml` authenticates as a dedicated
GitHub App rather than as the default `GITHUB_TOKEN`. The App has
exactly two permissions — `Contents: read & write` and `Pull
requests: read & write` — and is installed on this single repository.
At job start the workflow exchanges the App's ID + private key for a
short-lived installation token via
`actions/create-github-app-token@v2`. That token is then passed to
`release-plz` as `GITHUB_TOKEN`.

This is the one-time setup the maintainer runs before the workflow
turns green.

## Why an App is required (not optional)

The default `secrets.GITHUB_TOKEN` has two problems for release-plz:

1. It cannot open pull requests unless the repo-wide
   `Settings → Actions → General → Workflow permissions → Allow GitHub
   Actions to create and approve pull requests` flag is enabled. That
   flag is broad — it applies to *every* workflow in the repo.
2. Even with the flag on, PRs and releases created with the default
   token **do not trigger downstream workflows**. The `CI` workflow
   does not run on the release PR; `Release attestations` does not
   fire on the published release. Every release would require manual
   close+reopen of the PR and manual `workflow_dispatch` for the
   attestation workflow. That is not "automatic".

The App identity is treated as an external actor: PRs and releases
it creates do trigger downstream workflows. Setup is one-time; after
that every release is end-to-end autonomous from a single
conventional commit on `main`.

A PAT would also work, but PATs expire and are tied to a person; the
App identity is project-scoped and short-lived tokens never need to
be rotated.

## Steps

### 1. Create the App

Go to <https://github.com/settings/apps/new> (or
`https://github.com/organizations/<org>/settings/apps/new` if the
repository is org-owned).

| Field | Value |
|---|---|
| GitHub App name | `release-plz-yew-nav-link` (or any unique name) |
| Homepage URL | `https://github.com/RAprogramm/yew-nav-link` |
| Webhook → Active | unchecked |
| Permissions → Repository → Contents | Read and write |
| Permissions → Repository → Pull requests | Read and write |
| Subscribe to events | none |
| Where can this GitHub App be installed? | Only on this account |

Save. GitHub redirects to the App's settings page.

### 2. Capture the App ID and a private key

On the App settings page:

1. Note the numeric **App ID** at the top.
2. Scroll to **Private keys** → **Generate a private key**. A `.pem`
   file downloads. Keep it; the next step uploads its contents to a
   secret.

### 3. Install the App on the repo

On the App settings page → **Install App** → choose your account →
**Only select repositories** → tick `yew-nav-link`. Confirm.

### 4. Upload the two secrets

Repository → **Settings → Secrets and variables → Actions → New
repository secret**:

| Secret name | Value |
|---|---|
| `RELEASE_PLZ_APP_ID` | the numeric App ID from step 2 |
| `RELEASE_PLZ_APP_PRIVATE_KEY` | the full contents of the `.pem` from step 2, including the BEGIN/END lines |

### 5. Disable the now-redundant repo-wide flag

After the App is in place, the repo-wide flag enabled earlier
becomes redundant. Tighten it back:

```bash
gh api repos/RAprogramm/yew-nav-link/actions/permissions/workflow \
  -X PUT \
  -F default_workflow_permissions=read \
  -F can_approve_pull_request_reviews=false
```

This step is optional but recommended; the App alone is sufficient.

### 6. Trigger the workflow

The next push to `main` triggers `Release-plz`. The workflow's `Mint
GitHub App token` step exchanges the App ID + private key for an
installation token, hands it to `release-plz`, and the release PR
opens as `release-plz-yew-nav-link[bot]`. From here on every release
is fully autonomous: open release PR → CI runs on it → squash-merge
→ publish → `Release attestations` fires.

## Rotation

GitHub App private keys do not expire. Rotate only on suspected
compromise: regenerate the private key on the App settings page,
update the `RELEASE_PLZ_APP_PRIVATE_KEY` secret, revoke the old key.
Existing release-plz PRs continue to work; only future workflow runs
mint tokens from the new key.

## Removal

To switch back to `GITHUB_TOKEN`: enable the repo-wide flag (Settings
→ Actions → General → Workflow permissions), then revert the `Mint
GitHub App token` steps in `release-plz.yml` and delete the two
secrets. The App can be uninstalled from the repo or deleted entirely.
Note that the manual close+reopen-and-dispatch dance returns.

## References

- [`release-plz.yml`](../.github/workflows/release-plz.yml) — the
  workflow that consumes these secrets.
- [`RELEASE.md`](../RELEASE.md) — the human-facing release flow.
- [GitHub Apps docs — installation access tokens][gh-installation].

[gh-installation]: https://docs.github.com/en/apps/creating-github-apps/authenticating-with-a-github-app/about-authentication-with-a-github-app#authentication-as-a-github-app-installation
