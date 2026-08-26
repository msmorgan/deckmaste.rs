---
needs: []
---
# Open the subtype vocabulary — labels, not constructors

Ruling (user, 2026-08-25; scope confirmed ALL subtypes 2026-08-26): no
subtype of any class — creature, enchantment, artifact, land, planeswalker,
spell type — is an enum constructor, for the same reason keyword actions are
not (`Action::Composite`-style label + meaning-in-data; see the
workbench-mirrors-v2 ruling). The vocabularies are open, the rules reference
members by name (creature types [CR#205.3m] are just the largest class), and
where a subtype has rules meaning it lives in conferral data. Today
`data Subtype` (Words.idr) is one closed enum over all classes, and every
new subtype costs a constructor plus a `subtypeType` total-table row. The recent
`Phyrexian` vs `ManaSymbol.Phyrexian` constructor collision (resolved by
renaming the symbol to `PhyrexianMana`) is the shape's own symptom.

## What it needs

1. `data Subtype` SURVIVES as a type — it is the sort `HasSubtype`,
   `TypeLine.subs`, and `subsFitLine` range over — but stops being a unit
   enum (user, 2026-08-26). The shape is a label-carrying constructor in
   `Ordinal`'s mold (`Nth : Nat -> Ordinal`): the label as data (string or
   interned name word), the hosting card type as its sort — so `HasSubtype`,
   `TypeLine.subs`, `ChoiceDomain.TypeOtherThan`, `subsFitLine` and the
   [CR#205.3m] creature/Kindred sharing keep working over it. Study how
   `VerbLabel` landed as the open keyword-action vocabulary and mirror that
   shape.
2. The boundary follows the plugin layer's settled model, not a fresh
   decision: in `plugins/builtin/macros/` every subtype is already an open
   vocabulary entry (one macro per subtype), and a rules-meaningful one
   carries its meaning as a `confers:` list (Aura, Equipment, Fortification,
   Saga, the basic land types via `macro/subtype/BasicLandType.ron`) — the
   macros' own comments mark the subtype special *because* it confers. The
   v1 crate already did this well (user, 2026-08-25):
   `crates/deckmaste_semantics` — `conferral_rule.rs` and the subtype
   handling around it — is the code-side model to study alongside the RON
   data. Per the mirrors-v2 ruling the workbench takes that shape: subtype
   = label,
   rules meaning = what the label confers, no structural constructor class
   at all. Where the workbench gates currently index on a structural subtype
   (e.g. `chapterLineOk`'s Saga check, `subsFitLine`), rework them to read
   the label or its conferral data; record per gate how it was re-grounded.
3. Migration of ALL existing `Subtype` constructors — every class, Aura and
   Saga alongside the creature types — and their
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

- A new subtype of any class is introducible without touching any core enum
  or total table; `data Subtype` survives as the label-carrying type, with
  no unit-enum arm left; each gate that indexed on a structural subtype
  records how it was re-grounded on the label or its conferral data.
- `idris/scripts/build` PASS, no witness lost, no pin silently passing.

Standard constraints apply.
