---
needs: [lean-v3-authoring-impl-14-ast-and-authoring-cleanup]
---
# Refuse a subtype no declaration has

`lean-v3-authoring-impl-14-ast-and-authoring-cleanup` retired the string
subtype templates (`creatureType "…"` and kin). Cards now name catalog
subtypes (`Definitions/Subtypes.lean`), which fixed *Mishra's Factory*
writing "AssemblyWorker" for Assembly-Worker `[CR#205.3m]`. The raw
constructor `Subtype.of host "label"` still accepts any string, and nothing
refuses an unknown one: 13 sites outside the catalog write it (the checker's
own examples in `Check.lean`, the generated `Facts.lean`, and some proofs).

The open question is design: where the list of subtypes the checker reads
comes from. The checker sits below the catalog (`Semantics.Definitions`
imports it), so it cannot read the declarations themselves.

Proposal: have `cargo xtask facts generate` emit a `subtypeLabels` table
beside `keywordActionLabels`, from the subtype declarations, and pin it
against `Definitions/Subtypes.lean`. Add `Refusal.knownSubtype` at every
place a `Subtype` is stepped (type lines, `hasSubtype`, subtype heads,
creation specs), checked as `knownKeyword` is. Then rewrite the remaining
raw sites in proofs to the catalog names, keeping a raw one only in the pin
that shows an unknown subtype is refused.
