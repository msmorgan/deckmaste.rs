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

## As landed

The shape, in `Words.idr`:

```idris
public export
data Subtype : Type where
  MkSubtype : (host : CardType) -> (label : String) -> Subtype
```

`subtypeType` is now the `host` projection, `subtypeLabel` the new `label`
projection, and `Eq` reads both fields. Authoring goes through class helpers
mirroring `Semantics.idr`'s settled v2 `MkSubtype Category String […]`:
`creatureType` / `landType` / `artifactType` / `enchantmentType` /
`planeswalkerType` / `battleType` / `spellType`. The unit enum, its 99-row
`Eq` and its 99-row `subtypeType` are gone; nothing replaced them with a
table. A new subtype of any class is one call at the site that wants it.

**Label representation: `String`, not an interned name word.** The interned
form would have reinstated the closed vocabulary this round deletes, and the
string buys what the gates need: Idris reduces primitive string equality
during elaboration, so `Eq Subtype` still discharges every `So` gate that
consumes it (`subsFitLine`, `chapterFrameOk`, `predEq`) and `BasicLandType`'s
index still unifies against a literal label.

### Gates re-grounded

| gate | old index | new grounding |
| --- | --- | --- |
| `chapterLineOk` (Card.idr) | `elem Saga subs`, the `Saga` constructor | `elem (enchantmentType "Saga") subs` — label identity. [CR#714.1] gives the chapter symbols to Saga cards and Saga is a type-line word [CR#205.3b], so the label is the whole test; no conferral datum is consulted |
| `adventureInsetOk` (Card.idr) | `elem Adventure l.subs` | `elem (spellType "Adventure") l.subs` — label identity, [CR#205.3k] |
| `subsFitLine` / `spellSubtype` (Words.idr) | `subtypeType`'s total table | the `host` projection; the [CR#308.2] creature/Kindred sharing and the [CR#205.3k] instant/sorcery sharing read it exactly as before |
| `seedType` / `seedTypeAlts` (Phrase.idr) | `subtypeType` | the projection; [CR#205.3m]'s creature/Kindred alternation unchanged at source |
| `ChoiceDomain.TypeOtherThan` (Phrase.idr) | `subtypeType s = Creature` over a constructor | the same equation over the projected host — `artifactType "Equipment"` still refutes it as `Artifact = Creature` |
| `ascriptionOk` (Words.idr) | `subtypeType s == t` | the projection |
| `BasicLandType` / `BasicLandTypes` (Words.idr) | five constructors indexed by five `Subtype` constructors | the same five indexed by `landType "Plains"` … `landType "Forest"`. [CR#305.6] closes this set by enumeration, so it stays a closed family while the rest open; the auto-implicit search unifies on the literal label and `basicLandLine` is untouched |
| `Macros.thisAura` / `thisEquipment` / `thisSiege` | `AsType … (Just Aura)` etc. | `AsType … (Just (enchantmentType "Aura"))` etc. |
| attachment (`AttachHost Equipped` / `Fortified` / `Enchanted`) | never indexed on `Subtype` — the marking word is its own vocabulary | unchanged; `badEquippedLand`, `badFortifiedCreature`, `badEquippedPlayer` untouched |

**No conferral seam was built.** Every gate above re-grounds on label identity
alone, so a `confers` field would have been a field nothing reads. Proposal,
not landed: the workbench will need one the moment it has to state what a
subtype DOES rather than which word it is — the Aura graveyard SBA
[CR#704.5m], the Saga lore-counter machinery [CR#714.3a,714.3c], the
Equipment/Fortification attach grants [CR#301.5,301.6], the basic-land mana
ability [CR#305.6]. The shape is already settled twice over (v1's
`Subtype { name, types, confers }` and v2's third `List (Ability Base)`
field), so the seam is an added field plus a projection, not a design.

### Pins

All survived as-was — no discharge shape changed. Each pin that names a
subtype passes it as a value, never matches on it, so the constructor→label
move left every refusal mechanism in place: `badNonCreatureTypeExclusion`
refutes `subtypeType s = Creature` on the projected host,
`badChapterOnNonSaga` on an empty subtype list, `badAscribeForeignSubtype`
on `ascriptionOk`, `badSiegeWithoutBattle` on `subsFitLine`,
`badForestNonland` on the seed-type contradiction. Nothing was
weakened, nothing deleted, nothing blocked.

### Two names

- **`PhyrexianMana` → `Phyrexian` (restored).** The collision's cause was the
  subtype constructor, and there is no subtype constructor now — the creature
  type is the string `"Phyrexian"`. The mana symbol [CR#107.4f] takes its own
  name back.
- **`spellType : CardType -> Bool` → `spellCardType`.** The new subtype helper
  wanted the name, and it is the better claim to it: [CR#205.3k]'s spell types
  are subtypes, so the CardType predicate was the mis-named one. Three call
  sites.
