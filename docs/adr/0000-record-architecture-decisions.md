<!--
SPDX-FileCopyrightText: 2024-2026 RAprogramm <andrey.rozanov.vl@gmail.com>
SPDX-License-Identifier: MIT
-->

# 0000 — Record architecture decisions

- **Status:** accepted
- **Date:** 2026-05-12
- **Deciders:** RAprogramm

## Context

`docs/ARCHITECTURE.md` describes the *current* shape of the crate. It is
silent on *why* that shape was chosen, on what alternatives were rejected,
and on what would have to change for the decision to be revisited.
Reviewers (KaiCode jurors among them) reading the source can see the
result, not the reasoning. The reasoning lives in PR descriptions, git
history, and the maintainer's head — three places that decay differently
and none of which the casual reader will dig through.

## Decision

Maintain an Architecture Decision Record log in `docs/adr/`, one
decision per file, using the [MADR][madr] template.

Each ADR carries:

- A numeric, monotonically increasing ID (`0001-`, `0002-`, …) baked into
  the filename so cross-references survive renames.
- A status (`proposed`, `accepted`, `superseded by NNNN`, `deprecated`).
- The context that made the decision necessary.
- The decision itself, in declarative form.
- The consequences — positive, negative, and what it costs to reverse.

ADRs are written when the decision is made, not retrofitted later. The
opening batch (0001–0005) is the exception: it documents decisions that
shaped the 0.9/0.10 cycle and would otherwise stay implicit.

[madr]: https://adr.github.io/madr/

## Consequences

**Positive**

- `docs/ARCHITECTURE.md` becomes a living overview that links to the ADR
  log instead of trying to encode rationale inline.
- New contributors can see why a constraint exists before proposing to
  break it.
- Breaking changes carry an ADR — the ROADMAP entry becomes one line
  pointing at the ADR for the *why*.

**Negative**

- One more place to write to when shipping a non-trivial change.
  Mitigated by keeping ADRs short — a single screen is the target.

**Cost to reverse**

Low. ADRs are markdown files; removing them does not change any code.
