#!/bin/bash
# SPDX-License-Identifier: MIT
# SPDX-FileCopyrightText: 2026 RAprogramm <andrey.rozanov.vl@gmail.com>
#
# Syncs version and MSRV references in docs from Cargo.toml, the single
# source of truth. Run from the repository root:
#
#   ./scripts/sync-versions.sh          # rewrite references in place
#   ./scripts/sync-versions.sh --check  # exit non-zero if anything is stale
#
# Synced references:
#   - `yew-nav-link = "X.Y"` install snippets (README.md, src/lib.rs,
#     example/src/lib.rs)
#   - MSRV rows (README.md, docs/REQUIREMENTS.md)

set -euo pipefail

version="$(grep -m1 '^version = ' Cargo.toml | cut -d'"' -f2)"
msrv="$(grep -m1 '^rust-version = ' Cargo.toml | cut -d'"' -f2)"
minor="${version%.*}"

if [ -z "$version" ] || [ -z "$msrv" ]; then
  echo "error: could not read version/rust-version from Cargo.toml" >&2
  exit 1
fi

sync_file() {
  local file="$1" expr="$2"
  local synced
  synced="$(sed -E "$expr" "$file")"
  if [ "$synced" != "$(cat "$file")" ]; then
    if [ "${CHECK:-0}" = "1" ]; then
      echo "stale: $file (run ./scripts/sync-versions.sh)" >&2
      STALE=1
    else
      printf '%s\n' "$synced" >"$file.tmp"
      mv "$file.tmp" "$file"
      echo "synced: $file"
    fi
  fi
}

CHECK=0
STALE=0
[ "${1:-}" = "--check" ] && CHECK=1

snippet="s/yew-nav-link = \"[0-9]+\\.[0-9]+\"/yew-nav-link = \"${minor}\"/"
sync_file README.md "$snippet"
sync_file src/lib.rs "$snippet"
sync_file example/src/lib.rs "$snippet"

sync_file README.md "s/\\| Rust \\| [0-9]+\\.[0-9]+\\+ \\|/| Rust | ${msrv}+ |/"
sync_file docs/REQUIREMENTS.md "s/Rust \\*\\*[0-9]+\\.[0-9]+\\+\\*\\*/Rust **${msrv}+**/"

if [ "$STALE" = "1" ]; then
  exit 1
fi
