---
needs: [core-anaphor-surface, cards-corpus-dry-run]
---
**Rebuild `idris/src/Core.idr` around the antecedent stack (Idris v2).** Idris
stops serializing and becomes exactly three things: the design laboratory, the
proof suite, and the table generator for the Rust elaborator. This ticket is
the model rebuild; table emission + fixture export + `Cards.idr` re-encode is
[[idris-tables-fixtures-v2]].

Related: [[idris-naming-phase-two]] (naming policy applies to v2 names —
mirror concepts, never preserve a weaker concept behind a matching name).

## The rebuild

- The binding context becomes the antecedent STACK: `Ctx = List Ante` with
  `Ante = {sort, cardinality, site, expectedZone}`; references are sorted
  anaphors resolved by an auto-implicit `resolve` (proof search finds the
  nearest compatible antecedent); sequences are the telescope (`SeqList`:
  clause i+1's context is `intro(clause i, ctx)`); `intro` is a TOTAL
  function over producing clauses.
- The resolution algorithm must match the Rust elaborator as calibrated by the
  corpus dry-run ([[cards-corpus-dry-run]]) — including exact-sort precedence
  if that pre-approved loosening was adopted. Resolve/telescope is deliberately
  implemented twice (Idris + Rust) against SHARED fixtures; divergence is the
  meta-level failure mode, and the fixtures are the counter.
- `%default total` throughout; proofs carried at erased (0) quantity —
  runtime-relevant proof data is a modeling bug.
- One stack-weakening lemma (the stack is a list; `Elem`-mechanical) instead
  of per-rule ad-hoc weakening.
- **Message-pinned failing blocks:** every checker rule gets a positive
  example AND a `failing "<expected error>"` negative whose pinned message
  names the rule — no vacuous `failing` blocks that pass because of an
  unrelated error. Every failing block twins a RON reject fixture on the Rust
  side (exported in [[idris-tables-fixtures-v2]]).
- Carry over the v1 dependent guarantees on the new footing rather than
  regressing them: kind-indexed predicates (the object/player lattice),
  registry-parameterized vocabularies (subtype category, counter scope,
  designation scope, keyword shape), event caps gating the event-role
  references, cost caps gating payment anaphora — now derived from the same
  total functions the emitted tables come from, with no catch-all fallbacks
  (a caps function must be total-by-enumeration, not total-by-default-arm).

## Done

- `Core.idr` v2 typechecks with `%default total`; the v1 stack-free binding
  model is gone.
- Positive + message-pinned negative for every rule the Rust elaborator
  enforces; a rule with no Idris twin is a CI failure once
  [[idris-tables-fixtures-v2]] wires the export.
- Shared resolve fixtures (same inputs, same expected bindings) pass in both
  implementations.

## Verification

- `idris2 --build mtg.ipkg` (in `idris/`) — clean, including all `failing`
  blocks.
- `cargo test -p deckmaste_plugin` — shared resolution fixtures still green on
  the Rust side.
- `cargo xtask cite check` — 0 stale, `--list-noncompliant` empty (comments
  in Core.idr citing rules count).
