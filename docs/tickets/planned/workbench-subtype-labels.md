---
needs: []
---
# Open the creature-type vocabulary — labels, not constructors

Ruling (user, 2026-08-25): creature subtypes must not be enum constructors,
for the same reason keyword actions are not (`Action::Composite`-style label +
meaning-in-data; see the workbench-mirrors-v2 ruling). The vocabulary is open
— new names every set — and the rules treat the members as names referenced
only by label [CR#205.3m]. Today `data Subtype` (Words.idr) is a closed enum
mixing that open name class with structural subtypes, and every new creature
type costs a constructor plus a `subtypeType` total-table row. The recent
`Phyrexian` vs `ManaSymbol.Phyrexian` constructor collision (resolved by
renaming the symbol to `PhyrexianMana`) is the shape's own symptom.

## What it needs

1. An open label form for creature types: the label as data (string or
   interned name word), the hosting card type as its sort — so `HasSubtype`,
   `TypeLine.subs`, `ChoiceDomain.TypeOtherThan`, `subsFitLine` and the
   [CR#205.3m] creature/Kindred sharing keep working over it. Study how
   `VerbLabel` landed as the open keyword-action vocabulary and mirror that
   shape.
2. A decision, recorded in the ticket close, on the boundary: which subtype
   classes stay structural constructors because the CR attaches per-member
   meaning (Saga, Aura, Equipment, Curse, basic land types, Adventure, …) and
   which follow creature types into the open form (planeswalker types are
   names too; land types split — basics structural, nonbasic names open).
   The boundary test is rules-meaning, never corpus counts.
3. Migration of the existing creature-type constructors and their
   `subtypeType`/`Eq` rows, with Cards.idr/Proofs*.idr use sites updated.
   No witness lost, no pin silently passing.
4. Re-examine the `PhyrexianMana` rename once the collision's cause is gone:
   keep or restore, one line in the close either way.

## Consumption boundary

`idris/src/Experimental/Words.idr` (the `Subtype` split), `Phrase.idr` and
`Card.idr` consumers (`HasSubtype`, `subsFitLine`, `seedTypeAlts`,
`chapterLineOk`), `Macros.idr`, evidence bench `Cards.idr`, pins
`Proofs*.idr`. No Rust crate.

## Acceptance

- A new creature type is introducible without touching any core enum or
  total table; the structural/named boundary is recorded with its rule basis
  per class.
- `idris/scripts/build` PASS, no witness lost, no pin silently passing.

Standard constraints apply.
