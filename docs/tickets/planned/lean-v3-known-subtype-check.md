---
needs: [lean-v3-authoring-impl-14-ast-and-authoring-cleanup]
---
# Enforce catalog-only `Subtype` construction at the authoring boundary

`lean-v3-authoring-impl-14-ast-and-authoring-cleanup` retired the string
subtype templates (`creatureType "…"` and kin). Cards now name catalog
subtypes (`Definitions/Subtypes.lean`), which fixed *Mishra's Factory*
writing "AssemblyWorker" for Assembly-Worker `[CR#205.3m]`. However,
`Subtype` is not yet marked `@[internal_expansion]`, and 10 sites in
`Check.lean` and `Proofs/*.lean` still construct raw `Subtype.of host "label"`
values directly instead of using the catalog definitions.

## Design

- Enforce subtype validity at the authoring boundary only — do **not** add a
  `subtypeLabels` table to `Check/Facts.lean` or a `Refusal.knownSubtype` to
  `Check/`.
- Mark `Subtype` `@[internal_expansion]` in `Semantics/Words.lean` so
  `declare_semantic_primitives` does not expose `Semantics.Primitives.Subtype.of`
  or `Semantics.Primitives.Subtype.spell`; only `Definitions/Subtypes.lean`
  constructs `Subtype` values for authored declarations.
- Rewrite the raw `Subtype.of` sites in `Check.lean` and `Proofs/*.lean`
  (`Check.lean:205,224`, `Proofs/Rules.lean:45`, `Proofs/CorpusRecency.lean:24`,
  `Proofs/RefusalReachability.lean:119`, `Proofs/Numbers.lean:32,42`,
  `Proofs/Faces.lean:271,306,608`) to use the catalog definitions from
  `Definitions/Subtypes.lean` (`equipment`, `dragon`, `siege`, `aura`,
  `insect`, `elemental`, `jace`), leaving only `Check/Facts.lean` (generated
  below `Definitions/Subtypes.lean`) and `Proofs/Rules.lean:182` (the
  `Definition.check` negative pin for `Refusal.definitionNamed`).
