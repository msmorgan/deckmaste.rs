---
needs: []
---
**Delete the casual-supplement card types and the command-zone card class;
keep the planar-die effects.** Fresh workbench review 2026-09-03, R1,
resolved by ruling.

**Ruling (settled 2026-09-03): the workbench aims at objects that can be in a
Vintage deck.** `Words.CardType` loses `Conspiracy`, `Phenomenon`, `Plane`,
`Scheme`, `Vanguard` **and `Dungeon`**, and with them go
`Card.CardClass.CommandZoneCard`, `Card.commandZoneType`,
`Card.commandZoneTypeAbilityOk` (`Card.idr:95–102`),
`Card.anyCommandZoneType`, the `CommandZoneCard` clauses of
`Card.keywordCardOk` and `Card.classAbilityOk` (`Card.idr:52, 85–92`), the
`Words.KeywordFacts.onCommandZoneCard` column and its per-row value, and
anything whose only remaining dependency is one of those. Emblems are the
future home of dungeon semantics; venturing is not modelled by a `Dungeon`
card type.

The planar-die **effects** stay: `RollPlanarDie`, `ChaosEnsues` and
`PlanarDie` are printed on Vintage-legal cards (Fractured Powerstone, Missy,
Centaur of Attention), so the die machinery is card-backed even though the
plane card type is not.

This reverses parts of `done/workbench-card-class-and-command-zone` and
`done/workbench-command-zone-per-type-abilities`; both landed before the
scope ruling and are superseded by it, not contradicted on their own terms.
Note that the bench's one `Conspiracy` mention is the *enchantment* named
Conspiracy, which is unaffected.

Any pin whose subject is a supplement type goes with it — a pin refusing an
ability on a `Scheme` has no subject once `Scheme` is gone. A pin that
refuses something for a reason surviving the deletion is re-spelled, not
deleted; the landing record reports the split.

Size: S–M (~70 lines of deletion plus the column).

Done when: `CardType` holds the fifteen types minus the six named above and
`CardClass` has two constructors; `onCommandZoneCard` is gone from
`KeywordFacts` and from every row (including `Storied`'s); `grep -rn
'CommandZoneCard\|commandZoneType' idris/src` is empty; Fractured Powerstone
still typechecks as a bench witness; the deleted pins are accounted for by
name in the landing record with the reason each had no surviving subject; the
build is 44/44 with 0 errors and 0 warnings. Standard constraints apply, plus
the RON-shaped constraint: a core constructor is admissible only if the RON
re-emitter can produce it from a RON node, and a macro only if it names a RON
macro (`docs/decisions/workbench-ron-shaped-and-label-rulings.md`).

## As landed

- `Words.CardType` now holds nine constructors (`Creature`, `Artifact`,
  `Land`, `Enchantment`, `Instant`, `Sorcery`, `Planeswalker`, `Battle`,
  `Kindred`); `Conspiracy`, `Dungeon`, `Phenomenon`, `Plane`, `Scheme`,
  `Vanguard` and their `cardTypeIx`/`cardTypeAt`/`cardTypeAtIx` clauses are
  gone.
- `Card.CardClass` now holds two constructors (`PermanentCard`,
  `SpellCard`); `CommandZoneCard` is gone, and `cardClassOf` no longer
  branches on it.
- `Card.anyCommandZoneType`, `Card.commandZoneTypeAbilityOk`, and
  `Card.commandZoneTypesAbilityOk` (the plural driver — not itself named by
  the ticket, but its sole dependency was `commandZoneTypeAbilityOk`, which
  the ticket named) are deleted; `Card.cardAbilityOk` no longer conjoins
  a `commandZoneTypesAbilityOk` check.
- The `CommandZoneCard` clauses of `Card.keywordCardOk` and
  `Card.classAbilityOk` (7 clauses: `KeywordAbility`, `Activated`,
  `Triggered`, `Static`, `AlsoForKeywords`, `Spell`, `ItalicHead`,
  `MayBeginOnBattlefield` — 8 counting `keywordCardOk`'s own clause) are
  gone.
- `Words.commandZoneType` (the whole function, 15 clauses) is deleted;
  `Words.permanentType`, `Words.spellCardType`, `Words.ascribesAsType`, and
  `Words.retainable` lose their six clauses for the deleted constructors
  each (these weren't named by the ticket text but pattern-match
  exhaustively over `CardType`, so the deleted constructors' clauses no
  longer parse — "anything whose only remaining dependency is one of
  those").
- `Words.KeywordFacts.onCommandZoneCard` is gone from the record, from
  `MkKeywordFacts`/`MkModesKeywordFacts`, and from all 87 rows of
  `keywordFacts` (including `Storied`'s, which was the one row with
  `onCommandZoneCard = True`).
- Two pins deleted, both in `ProofsFaces.idr`: `badConspiracyActivated`
  ("an activated ability printed on a conspiracy card") and
  `badDungeonStatic` ("a static ability printed on a dungeon card") —
  reason for both: subject deleted by ruling (their `MkTypeLine`s named
  `Conspiracy`/`Dungeon`, constructors `CardType` no longer has).
- The bench's `Conspiracy` mention (`Cards/Choice.idr:404`, the enchantment
  card) and the `Cards/Anaphora.idr:729` docstring "Grenzo, Dungeon Warden"
  are untouched — neither is the `CardType`/`CommandZoneCard` vocabulary.
  Planar-die effects (`RollPlanarDie`, `ChaosEnsues`, `PlanarDie`) and their
  bench witnesses are untouched.

## Landing record

- Numbers before/after: `CardType` constructors 15 → 9. `CardClass`
  constructors 3 → 2. `KeywordFacts` record fields 10 → 9 (drops
  `onCommandZoneCard`); all 87 `keywordFacts` rows lose that one
  boolean. Functions deleted: `Words.commandZoneType` (15 clauses),
  `Card.anyCommandZoneType`, `Card.commandZoneTypeAbilityOk` (6 clauses),
  `Card.commandZoneTypesAbilityOk`. `cr-citations.lock` unchanged (diff
  carries no citations).
- Gate lines. `cd idris && rm -rf build && ./scripts/build`: last module
  line `44/44: Building Cards (src/Cards.idr)`, exit 0, 44 module build
  lines, 0 Warning and 0 Error lines. `cargo xtask cite check
  --list-noncompliant`: 2 non-compliant citation-looking strings, both in
  `docs/tickets/done/ability-kind-taxonomy.md` (pre-existing, outside this
  diff, which touches only `idris/`) — 0 in the diff. `cargo xtask cite
  check`: `checked 18218 citations against cr.txt (eff. 2026-08-07); 0
  stale`. `cargo xtask cite audit --diff` (`jj diff --git | ...`):
  `audited 0 citation site(s) — nothing selected` (the diff is pure code
  deletion, no `[CR#…]` lines touched). Grep gate
  (`grep -rnwE "Conspiracy|Phenomenon|Plane|Scheme|Vanguard|Dungeon|
  CommandZoneCard|commandZoneType|anyCommandZoneType|onCommandZoneCard"
  idris/src/Experimental/`): 2 hits, both card names outside the deleted
  vocabulary — `Cards/Anaphora.idr:729` ("Grenzo, Dungeon Warden",
  docstring) and `Cards/Choice.idr:405` (`Macros.card "Conspiracy"`, the
  enchantment).
- Assurance: restored 0; re-spelled 0 (no pin's refusal reason survived the
  deletion under a different subject); ignored 0; added 0; removed 2
  (`badConspiracyActivated`, `badDungeonStatic` — both named above, reason
  "subject deleted by ruling").
- Deviations and additions: none beyond the ticket's letter — the extra
  clause deletions in `permanentType`, `spellCardType`, `ascribesAsType`,
  and `retainable`, and the deletion of `commandZoneTypesAbilityOk`, fall
  under the ticket's "anything whose only remaining dependency is one of
  those" clause (the six `CardType` constructors and their sole users) and
  are not separate scope.
- No STOP taken.
