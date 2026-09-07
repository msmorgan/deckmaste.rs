---
needs: []
---
**`Stat` has no `defense` axis, so a battle's printed defense cannot be read.**
Found by `lean-rules-tables` (2026-09-07): `CharacteristicStat` (what an effect
*writes*) is `power|toughness|loyalty|defense`, but `Stat` (what an `Amount`
*reads*, through `ProjAxis.stat`) is `power|toughness|manaValue|loyalty`. A
battle's defense on the battlefield is its defense-counter count [CR#310.4c], so
the counter read covers the state-based action [CR#704.5w] and the damage result
[CR#120.3h] — both land in `Proofs/Rules.lean`. What has no spelling is the
*printed* defense, and so the battle's intrinsic conferral [CR#310.4b] ("enters
with a number of defense counters equal to its printed defense number"), the
exact analogue of the planeswalker row [CR#306.5b] that `rules/grant/` writes.

Add `defense` to `Stat` with its citation [CR#310.4,310.4a], thread it through
`Stat.comparedType` (→ `battle`) and every exhaustive read in `Check/`, mirror it
in `crates/deckmaste_semantics_v2` (drift test), and add the battle conferral row
to `Proofs/Rules.lean` beside `planeswalkerLoyalty`. Verify the axis against real
card text that compares a battle's defense before settling the spelling.
Standard constraints apply.
