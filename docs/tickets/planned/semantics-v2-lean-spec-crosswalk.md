---
needs: [lean-grammatical-reference-scopes, lean-conjunction-type-evidence, lean-enact-expansion-boundary, lean-erased-index-obligations, lean-field-context-threading, lean-printed-stat-validation, lean-controller-sacrifices-composition, lean-noun-word-refinements, lean-zero-life-exemption-is-a-deontic, docs-lean-is-the-workbench]
---
**Finish the construction and lowering crosswalk that makes Lean usable as
the semantics_v2 specification.** The user designated `lean/` as the new target
on 2026-09-05. The readiness review found checker omissions and unresolved
representation boundaries; the prerequisites close those before this final
specification audit. `docs/decisions/semantics-v2.md` already requires both a
construction audit and transcription of the worked corpus. The historical
`idris/src/Bridge.idr` T-rule inventory describes an earlier shape.

Deliver a current Lean-owned crosswalk from every in-scope English AST
construction to its Lean spelling and lowering rule, or an explicit lowering
obligation routed to a named ticket. Inventory from the actual current English
declarations, not only from the accepted card bench. Record the resolved macro
trust boundary, grammatical reference scopes, type evidence, and field-context
contracts with links to their Lean checks and pins. Identify the structural
laws supporting those contracts, including pure type-conjunction invariance
where applicable, reference scope, and macro profile/provenance preservation.

Reconcile the worked-corpus audit with current Lean spellings and name every
uncovered in-scope item. A passing `Spelled` theorem establishes checker
acceptance; do not present it as a proof of correct lowering or execution.
Port the relevant Bridge examples into checked Lean/target pairs or another
explicitly checked translation artifact; do not extend the retired Idris model.

`docs-lean-is-the-workbench` owns the succession documentation and dated notes
on historical ADRs. `semantics-v2-adr-touchups` separately owns its two proposed
wording decisions; preserve that ticket's scope and approval boundary. This
ticket owns the current crosswalk and readiness assessment, not a rewrite of
historical decisions. Report remaining blockers honestly rather than declaring
the specification complete while an in-scope construction has no settled
meaning. Normal Vintage scope applies.
