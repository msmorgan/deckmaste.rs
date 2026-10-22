---
needs: [lean-v3-action-clause-spine]
---
**[port-required: semantics-v3] `Stat` has no `defense` axis, so a battle's
printed defense cannot be read.** Found by `lean-rules-tables` (2026-09-07):
`CharacteristicStat` (what an effect *writes*) is
`power|toughness|loyalty|defense`, but `Stat` (what an `Amount` *reads*, through
`ProjAxis.stat`) is `power|toughness|manaValue|loyalty`. A battle's defense on
the battlefield is its defense-counter count [CR#310.4c], so the counter read
covers the state-based action [CR#704.5w] and the damage result [CR#120.3h].
What has no spelling is the *printed* defense, and so the battle's intrinsic
conferral [CR#310.4b], the exact analogue of the planeswalker row [CR#306.5b]
that `rules/grant/` writes.

Decided (2026-10-13, owner; `semantics-v3.md` §3.7; archived `semantics-v2.md`
§7, Q81, Q82):

- `Stat` gains `defense` [CR#310.4,310.4a]; `Stat.comparedType` maps it to the
  battle card type beside loyalty's planeswalker. `Stat` and `CharacteristicStat`
  stay two enums: mana value is not a characteristic [CR#109.3], so the split is
  real.
- [CR#310.4b] is written as a conferral on the battle card-type definition: it
  enters with defense counters equal to `statOf(stat(defense), this)`, the same
  home and shape as the planeswalker loyalty entry. It is the witness the
  `defense` axis exists for.

`lean-v3-stat-reads-and-number-regimes` (2026-10-22) landed the battle row beside
`planeswalkerLoyalty`. Its interim `ProjAxis.printed` read is retired (owner ruling 2026-10-22,
`semantics-v3.md` §0.1 item 5): the [CR#310.4b] conferral reads `statOf(stat(defense), this)`, as
decided above, because off the battlefield a battle's defense already is its printed number
[CR#310.4a].

Do: add `defense` to `Stat` with its citation, thread it through
`Stat.comparedType` and every exhaustive read in `Check/`, mirror it in
`crates/deckmaste_semantics_v3` (when created in `semantics-v3-dsl-and-plugins`),
and add the battle conferral row beside `planeswalkerLoyalty` in
`Proofs/Rules.lean`. Standard constraints apply.
