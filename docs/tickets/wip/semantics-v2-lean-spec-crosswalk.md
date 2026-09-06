---
needs: [lean-grammatical-reference-scopes, lean-conjunction-type-evidence, lean-enact-expansion-boundary, lean-erased-index-obligations, lean-field-context-threading, lean-printed-stat-validation, lean-controller-sacrifices-composition, lean-noun-word-refinements, lean-zero-life-exemption-is-a-deontic, docs-lean-is-the-workbench]
---

**Retired: the user rejected this ticket's premise on 2026-09-06.**

The requested exhaustive mapping from English AST constructions to Lean and
its specification-readiness requirement were premature architectural
requirements, not established defects. They are withdrawn.

The user's intended translation mechanism is meaningful English spellings
owned by semantic macros, through `semantics_v2` RON. This ticket does not
establish a separate direct mapping, a Lean bridge in Rust lowering, or new
worked-corpus obligations. The previously approved macro-only authoring
boundary remains valid.

## Landing record

PROVE: Retirement only. All speculative implementation and audit artifacts
were removed before integration: `crates/deckmaste_lowering/tests/lean_bridge.rs`,
`lean/CROSSWALK.md`, `lean/crosswalk/constructions.md`, and
`lean/crosswalk/worked-cards.md`. No baseline code, card, or assertion changed.
No coverage identity was lost. Structural and licensing behavior is unchanged.

DISCLOSE: The discarded Rust file contained three newly introduced,
unintegrated tests; no baseline tests were removed, re-spelled, restored, or
ignored. No new tests remain. The cancellation withdraws the proposed mapping
and transcription obligations, rather than reporting them as implemented.
No English corpus, selection, or licensing metrics were remeasured.

REPORT: Documentation-only retirement on change
`xnwzqkytqpxsuyornmsxpvpypzlqvmwq`; unchanged coverage lock `covered` 20,254.
Construction counts, overlap inventories, and performance were not remeasured.
No push was performed.
