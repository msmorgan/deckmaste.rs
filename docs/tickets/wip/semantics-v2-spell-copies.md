---
needs: []
---
**Copy keywords offer new targets unconditionally.** Ruling 2026-10-09 ("A
spell copy's target guard is vacuous", `docs/decisions/semantics-v2.md` §7).
Bodies casualty, replicate, epic and conspire's copy half, the census's
"condition that a spell has any targets" group. Standard constraints apply.

## Shape

- No optional subject on `Predicate.targets`; no "has any targets"
  condition. A copy keyword writes `may(chooseNewTargets(…))` with no guard,
  as storm does: its player may leave every target unchanged, so a copy
  with no targets has none to change [CR#707.10c].
- `TargetExtent.only`: "targets only …" (24 cards), beside `someTarget` and
  `soleTarget`.
- Copying "for each … it could target" [CR#707.10d] is not this ticket:
  it stays with `engine-copy-target-variants`.
- Rust mirror and RON helper for `only`.

## Re-spells (same asserted outcome)

- gravestorm (`plugins_v2/builtin/macros/keyword_abilities/gravestorm.ron`):
  its `doIf(matches(thisSpell, targets(…, someTarget)), …)` guard goes;
  `may(chooseNewTargets(…))` follows the copy directly. Regenerate
  `lean/Semantics/Macros/Declared.lean` so `cargo xtask lean macros check`
  passes; pins over gravestorm keep their statements.
- Any other copy keyword or canon card with the same guard, re-spelled the
  same way.

## Bodies

- casualty: `addedCost(perform(sacrifice(a(and([creature,
  powerAtLeast(N)])))), true)` plus a cast trigger copying the spell under
  `keywordCostPaid(casualty, this)` [CR#702.153a].
- replicate: `addedCost(Param(0), true, anyNumberOfTimes)` plus a cast
  trigger copying it `timesPaid` times [CR#702.56a].
- epic: the `restOfGame` can't-cast-spells deontic and `delay(beginningOf(
  upkeep, you, each), copy(this, except: ability(epic)), restOfGame)`
  [CR#702.50a].
- conspire's copy half: the cast trigger copying under `keywordCostPaid`
  [CR#702.78a]. Its cost (two untapped creatures you control that share a
  color with it) is `semantics-v2-sharing`'s `shares`; if sharing has not
  landed, conspire stays bodyless and its STOP names that ticket.

## Witnesses (canon)

A canon card each for casualty, replicate and epic if canon lacks one; one
"targets only …" card (`only`). Pins: each body checks; `only` over a
player-kinded subject where an object is required is refused.

## Siblings

Cross-references `semantics-v2-sharing` (conspire's cost) and, through the
`timesPaid` spelling, `semantics-v2-linked-readback`'s rename; neither
blocks. Refresh before integrate; expect conflicts in
`Words`/`Phrase`/`Abilities`.
