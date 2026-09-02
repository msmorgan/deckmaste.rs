# workbench-small-residues

The bucket round's surviving small items (2026-09-02; corrected counts —
full method and blockers in done/workbench-event-disjunction-tail.md and
Cards.idr's residue trailers):

- Ordinal-per-turn activation restrictor — 11 lines / 11 cards, two shapes.
- Entry-rider defender slot — 15/15 of 121 (needs the `TokenRider` module
  move above `Noun`/`AttackDefender`).
- Attachment-host word with no card type — 105 lines / 80 cards total; the
  prohibition subset is 31/31.
- Cross-kind you-or-description join — 42/41 (the "you or a player" class
  was a false positive, joins two players).
- Per-player partition — 6/6 of 28 (element scoping on `ForEachOf`).
- Marked player-sort read ("the last chosen player" — Beckoning
  Will-o'-Wisp / Triarch Stalker).
- Ordinal-player cluster (`NthPlayer`, `OpponentOf` sibling — probed
  blockers in done/workbench-each-player-binder.md).
- Face-down spell — 7 descriptions + 1 cast-instruction.
- Subtype retention — 2/2 (Cavernous Maw, Duplicant).
- `Choose` binder gap — 2/2 (Call to the Void, Malik).
- Singletons waiting on second carriers: MoveCounters partitive (1),
  Master Biomancer's rider (1), Temporal Anchor's step-event (1).
- The general event disjunction stays DECLINED with its live successor
  (the tail-aligned union) recorded in the bucket's close.

Re-measure at claim; several may fall out of each other.
