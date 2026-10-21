---
needs: [lean-v3-authoring-impl-14-ast-and-authoring-cleanup]
---
# Enforce catalog-only `Subtype` construction at the authoring boundary

`lean-v3-authoring-impl-14-ast-and-authoring-cleanup` retired the string
subtype templates (`creatureType "…"` and kin). Cards now name catalog
subtypes (`Definitions/Subtypes.lean`), which fixed *Mishra's Factory*
writing "AssemblyWorker" for Assembly-Worker `[CR#205.3m]`. Nothing enforces
it, though: `Subtype` was never a `semantic_expression`, so the authoring
inspection treated it as ordinary printed data. No primitive wrapper was ever
generated for it (`Semantics.Primitives.Subtype.of` and `.spell` did not
exist), and a `spelled` card writing a raw `.of .creature "AssemblyWorker"` was
accepted. 10 sites in `Check.lean` and `Proofs/*.lean` also still construct raw
`Subtype.of host "label"` values instead of using the catalog definitions.

## Design

- Enforce subtype validity at the authoring boundary only — do **not** add a
  `subtypeLabels` table to `Check/Facts.lean` or a `Refusal.knownSubtype` to
  `Check/`.
- Tag `Subtype` `semantic_expression` and `internal_expansion` in
  `Semantics/Words.lean`. As a semantic expression, a raw `Subtype.of` or
  `Subtype.spell` in a card is reported raw and refused by `spelled`, including
  one built by an ordinary helper that the card inspection follows. A catalog
  entry is an authored call. As an internal expansion,
  `declare_semantic_primitives` exposes no `Semantics.Primitives.Subtype.of`
  or `.spell` wrapper.
  - Owner ruling (2026-10-21): enforcement through `semantic_expression` is in
    scope for this ticket. That means card-level enforcement. The
    `internal_expansion` tag alone would have hidden nothing and refused
    nothing.
- Owner ruling (2026-10-21): the *Mishra's Factory* defect itself was fixed in
  `lean-v3-authoring-impl-14-ast-and-authoring-cleanup`, and this ticket guards
  cards only. Authored declarations (`semantic_definition`,
  `semantic_template`) are trusted, as for every semantic type. There is no
  declaration-time subtype check.
- Values that are not `spelled` cards are not checked. That covers the
  checker's tables (`Check/Facts.lean`) and the pins that build a raw subtype
  on purpose (`badNamelessSubtype` in `Proofs/Rules.lean`, and the refusal pins
  in `Proofs/Authoring.lean`). The basic land cards in `Cards/Choice.lean`
  (`basicLandCards`, plain `Card` values) become five `spelled` cards so that
  the boundary applies to them.
- Pin in `Proofs/Authoring.lean` that `spelled` refuses a raw `Subtype.of` and
  a raw `Subtype.spell`, and that no `Primitives.Subtype` wrapper exists. The
  accepted direction is the corpus: *Mishra's Factory* and the Adventure card in
  `Cards/Faces.lean` are `spelled` with catalog subtypes.
- Rewrite the raw `Subtype.of` sites in `Check.lean` and `Proofs/*.lean`
  (`Check.lean:205,224`, `Proofs/Rules.lean:45`, `Proofs/CorpusRecency.lean:24`,
  `Proofs/RefusalReachability.lean:119`, `Proofs/Numbers.lean:32,42`,
  `Proofs/Faces.lean:271,306,608`) to use the catalog definitions from
  `Definitions/Subtypes.lean` (`equipment`, `dragon`, `siege`, `aura`,
  `insect`, `elemental`, `jace`), leaving only `Check/Facts.lean` (generated
  below `Definitions/Subtypes.lean`) and `Proofs/Rules.lean:182` (the
  `Definition.check` negative pin for `Refusal.definitionNamed`).
