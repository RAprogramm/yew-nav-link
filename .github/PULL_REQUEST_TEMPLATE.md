## Summary

<!-- What does this PR do, and why? Write for the reader of the commit
history on `main`, not just the reviewer. -->

Closes #

## Changes

-

## Author checklist

- [ ] `cargo +nightly fmt --all`
- [ ] `cargo clippy --all-targets --all-features -- -D warnings`
- [ ] `cargo nextest run --all-features` (plus `cargo test --doc` if docs changed)
- [ ] Behaviour / API changes propagated to `README.md`, `docs/` and `docs/REQUIREMENTS.md`
- [ ] Public-API changes reflected in `docs/public-api.txt` (`cargo public-api -sss`) and covered by tests
- [ ] Commits follow `#<issue> <type> <description>` (see [`docs/BRANCHING.md`](../docs/BRANCHING.md))

## Reviewer checklist

<!-- Confirmed by the CODEOWNERS reviewer before approving. -->

- [ ] Change is scoped to the linked issue — no unrelated edits
- [ ] Any public-API change is intentional and semver-appropriate
- [ ] Tests exercise the new behaviour, not merely compile it
- [ ] Docs / ADRs reflect any design decision introduced here
- [ ] `CI Success` is green and the branch is up to date with `main`

The full merge policy lives in [`docs/BRANCHING.md`](../docs/BRANCHING.md).
