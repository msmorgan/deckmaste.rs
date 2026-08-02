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
- Correctness story from day one: a debug-only raise map over the mirrored
  subset with a CI round-trip property (`raise(lower(t)) ≡α t`); the
  convention that every future non-identity arm carries its justification
  in place (the crate IS the divergence ledger) plus a per-variant test.
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

Standard constraints apply. Round-trip property green over canon and
wizards (lower every loaded authored term, raise, compare); exhaustiveness
enforced by the compiler (no wildcard arms in the mapping).
