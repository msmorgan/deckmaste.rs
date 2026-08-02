---
needs: [core-demacro]
design: true
---
**Design-gated: a shared characteristics-atoms crate below both grammars,
so authoring and core stop mirroring the value vocabulary.** Context:
`docs/decisions/authoring-spelling-lowering.md` (§1, the card bullet).
Deliberately NOT part of the fork program's Stage 1 — this is a purely
additive deduplication for after both grammars exist and the dust settles.

## The prize

`Color`, `Supertype`, the closed `Type` enum, stat-value atoms (exact
inventory is design question 1) defined ONCE in a crate below
`deckmaste_authoring` and `deckmaste_core`: no fork copies, no identity
mapping arms for atoms, no drift surface. Natural secondary home for the
engine's `BaseCharacteristics` trait if pulling it below the engine ever
pays.

## The tension (the design gate)

1. **Inventory**: which types are genuinely shared atoms? `Subtype` blurs
   the edge (registry-defined data, not a closed enum) — likely stays
   grammar/registry-side.
2. **The derive problem**: authoring needs `SupportsMacros` on the atoms
   (straggler registration: `Green` as a def with frames), the derive must
   sit with the type (orphan rule), so the shared crate needs an optional
   `macros` feature — and cargo feature unification switches it on for
   every consumer in any build that includes authoring, core included.
   The `core-demacro` purity gate must then be re-phrased ("core's
   sources and API are macro-free") rather than "no macro_ron anywhere in
   core's tree". Decide whether that weakening is acceptable, or find a
   sharper mechanism (newtype wrappers authoring-side were considered and
   disliked: registration/serde noise).
3. **Idris**: the authoring mirror models atoms wherever they live;
   confirm the emitter is indifferent to the crate boundary.

## Gates

Standard constraints apply. Zero behavior change; atom mapping arms
deleted from `deckmaste_lowering` with the round-trip property still
green; the re-phrased (or preserved) core purity gate stated explicitly in
the plan.
