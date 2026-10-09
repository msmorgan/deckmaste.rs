---
needs: []
---
**Linked Facets, declared relations, merge and a mana-cost term.** Ruling
2026-10-09 ("A linked ability reads back a Linked Facet of a Link Source",
`docs/decisions/semantics-v2.md` §7). Bodies seven census residues: emerge,
offering, harmonize, haunt, soulbond, cipher, mutate. Standard constraints
apply.

## Shape

- Renames: `PaidFacet` → `LinkedFacet`, `PaidCostName` → `LinkSource`
  (Lean `Phrase.lean`/`Words.lean`, the Rust mirror in
  `crates/deckmaste_semantics_v2/src/{phrase,words,ron}.rs`, xtask
  `lean_macros.rs`, `lean/CONTRACTS.md`, `Check/Refusal.lean`). The five
  facets, the macros `chosenAsPaid`, `timesPaid`, `keywordCostPaid`,
  `costWasPaid`, `paidCostRead` and facts gating stay.
- `LinkSource.byDeed (verb)`: the printed ability identified by what it does
  ("the exiled card" of an exiling ability) [CR#607.2a].
- New facets: `objects (verb)` (what the linked ability's named deed acted
  on: "the sacrificed creature", "the exiled cards", "creatures it devoured"
  [CR#702.82b]) and `taken : Bool` ("tribute wasn't paid" [CR#702.104b]). The
  facts column that gates facets extends to them; offering's row gains
  `paidCost := true`.
- Relations: `Definition.relation (label) (left : Kind) (right : Kind)`,
  declared under `plugins_v2/builtin/macros/relations/` as designations are;
  `Instruction.relate (subject) (label) (counterpart)`, `Predicate.related
  (label) (counterpart : Option NounPhrase)`, `NounPhrase.counterpart (label)
  (subject)`. The checker reads the declared kinds.
- `Instruction.merge (subject) (onto) (position : top | bottom)` [CR#730.2];
  `StaticSpec.addsTarget (subject) (description)`, a target the spell gains
  when its alternative cost is chosen [CR#702.140a].
- `CostShift.run` → `CostShift.bySymbols (cost : ManaCostTerm) (rises)
  (coloredOnly)`, `ManaCostTerm = lit (cost) | manaCostOf (object)`. Read
  `coloredOnly`'s meaning from its pins before touching it.

## Re-spells (same asserted outcome)

Every `PaidFacet`/`PaidCostName`/`CostShift.run` site: Lean cards and pins
(`Proofs/GiftVariants.lean`, `Proofs/LinkedChoiceReadback.lean`), RON
helpers under `plugins_v2/builtin/macros/instructions/`, the bench, canon,
and Rust tests. `emerge.ron` and `harmonize.ron` comments name the new facet.

## Bodies

- offering: `bySymbols(manaCostOf(linked(theAdditional,
  objects(sacrifice))))` [CR#702.48a,702.48c].
- emerge: `less(statOf(manaValue, linked(theAlternative,
  objects(sacrifice))))` [CR#702.119a]; "emerge from [quality]"
  [CR#702.119b].
- harmonize: the cost taps a creature, reduction by its power, the exile
  replacement [CR#702.180a].
- haunt: `exile(this)` then `relate(it, haunting, target(creature))`, and
  "the creature it haunts" by `counterpart` [CR#702.55a,702.55b].
- soulbond: the two pairing triggers and `related(paired, …)`
  [CR#702.95a,702.95b].
- cipher: exile encoded, `relate(it, encodedOn, …)`, the granted trigger
  [CR#702.99a,702.99b].
- mutate: alternative cost plus `addsTarget` ("a non-Human creature with
  the same owner") and `merge` [CR#702.140a,730.2]; the "mutates" trigger
  (37 non-keyword cards).

## Witnesses (canon, non-keyword)

Ghoulcaller Gisa ("the sacrificed creature's power"), Spell Queller ("the
exiled card" in a second ability, `byDeed`), a "the chosen creature" card,
a "whenever this creature mutates" card. Pins: `objects` of a deed the
source never performs is refused; `relate` against a declared kind mismatch
is refused; the renamed facets keep their existing pins' statements.

## Siblings

`semantics-v2-off-stack-statics` reads `objects` and `taken` for devour and
tribute, and `semantics-v2-modal-choice` reads `timesPaid` for escalate;
neither blocks. Refresh before integrate; expect conflicts in
`Words`/`Phrase`/`Abilities`.
