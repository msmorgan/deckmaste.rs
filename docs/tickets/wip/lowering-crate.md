---
needs: [authoring-crate-fork]
---
**Create `deckmaste_lowering`: the one-way total compile
`deckmaste_authoring` → `deckmaste_core`.** Design:
`docs/decisions/authoring-spelling-lowering.md`
(§9). Depends on both grammar crates; neither grammar crate may depend on
the other — the build graph is the architecture.

## Scope

- `lower(authored) -> core` as a total mapping over the mirrored grammar:
  generated identity-shaped arms (generation of ARMS only — no shared
  schema layer defining both enums; see spec §13.1).
- Correctness story from day one: **one mapping test per variant, each
  naming the engine shape it expects**; the convention that every future
  non-identity arm carries its justification in place (the crate IS the
  divergence ledger). The raise map the spec used to require here is
  WITHDRAWN (owner-settled 2026-08-02) — its coverage shrinks as arms
  diverge and `plugin-repoint` takes the first bite, so a second full
  150-impl mapping is not worth building; see spec §9.
- If `card-crate-split` has landed, this crate also depends on
  `deckmaste_card` (lowering targets both engine-side crates).
- Normalization (authored → authored normal form) lives in
  `deckmaste_authoring` (spec §9); this crate invokes it and owns the
  cross-grammar mapping. Sugar/elaboration rules land later
  (`target-sugar-elaboration`).
- The error-taxonomy pass (authoring parse errors / lowering errors / core
  validation — spec §9) happens here, where the layers meet; the
  expansion-equality law is this crate's to state and test.

## Gates

Standard constraints apply. One mapping test per variant, all green, each
asserting the expected engine shape to the depth stable Rust can pattern
against; corpus gate green over canon and builtin (every loaded authored
term lowers); exhaustiveness enforced by the compiler (no wildcard arms in
the mapping).
