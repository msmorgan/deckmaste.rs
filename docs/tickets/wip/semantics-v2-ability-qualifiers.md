---
needs: []
---
**The ability printed with a keyword, and the restriction the keyword adds
to it.** Ruling 2026-10-09 ("The ability printed with a keyword is a keyword
parameter", `docs/decisions/semantics-v2.md` §7). Bodies eight census
residues: forecast, boast, exhaust, solved, ∞ (infinity), max speed,
power-up, backup. Standard constraints apply.

## Shape

- `KeywordParam.ability (ability : Ability)`, beside `cost`, `quality`,
  `subject`, `number`, `deckCondition`. The checker's keyword schemas
  already expect it for seven keywords; wire them to it.
- `Ability.qualified (ability : Ability) (timing : Option Timing) (limit :
  Option UsageLimit) (guard : Option Condition) (from : Option Zone)`
  [CR#113.10a,113.6b]. The guard reads "as long as" (static), "this ability
  triggers only if" (triggered) or "activate only if" (activated) by the
  wrapped ability's category [CR#702.169b..702.169d]; `from` is the zone
  the ability functions from [CR#113.6b].
- `Period.thisObject` on `UsageLimit`: "activate only once", reset when the
  object changes zones [CR#602.5b,702.177a].
- `AbilitySelection.printedAfter (keyword : KeywordLabel)`: the abilities
  printed below a keyword, for backup [CR#702.165a,702.165c].
- `PlayerStat` has no speed today: add `PlayerStat.speed` [CR#702.179a]
  and note in the landing record that its game state is `engine-speed`'s.
- Checker: kinds and references only; `qualified` refuses a `limit` on a
  static or triggered wrapped ability as a shape.
- Ability words are unchanged and stay `italicHead` [CR#207.2c].
- Rust mirror, RON helpers, bench macros spelled like the RON target.

## Bodies

- forecast: `qualified(Param(0), timing: inPart(upkeep, owner, current),
  limit: once per turn, from: hand)`, the activation revealing the card
  [CR#702.57a,702.57b].
- boast: activate only if this creature attacked this turn and only once
  each turn [CR#702.142a].
- exhaust: `limit: once per thisObject` [CR#702.177a].
- solved: the guard `hasDesignation(solved, this)` over each category
  [CR#702.169a..702.169d].
- ∞: `conditional(abilityGrant(this, Param(0)), hasDesignation(harnessed))`
  [CR#702.186b].
- max speed: the same with your speed 4, `PlayerStat.speed` [CR#702.178a].
- power-up: once per object, and cost reduced by `bySymbols(manaCostOf(
  this))` if this entered this turn [CR#702.193a]. `bySymbols` is
  `semantics-v2-linked-readback`'s rename of `CostShift.run`; if that has not
  landed, spell it `run` and name the sibling in the landing record.
- backup: the counters trigger, and if that's another creature it gains
  `printedAfter(backup)` until end of turn [CR#702.165a,702.165c].

## Re-spells

Any existing "activate only once each turn" pin re-spells only if its
`UsageLimit` constructor changes; the existing periods keep their meaning.

## Witnesses (canon)

A canon card per keyword bodied (forecast, boast, exhaust, a Case for
solved, a harnessed ∞ card, a max speed card, a power-up card, a backup
card). Pins: a `qualified` activated ability with a guard checks; a
`limit` on a static is refused; `printedAfter` of a keyword the card lacks is
refused as a reference.

## Siblings

Cross-references `semantics-v2-linked-readback` (`bySymbols`) and
`engine-speed` (speed state); neither blocks. Refresh before integrate;
expect conflicts in `Words`/`Phrase`/`Abilities`.
