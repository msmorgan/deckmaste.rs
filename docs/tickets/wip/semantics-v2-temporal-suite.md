---
needs: []
---
**Three temporal sorts: Timing (when), Duration (how long), UsageLimit (how
often).** Ruling 2026-10-08 ("Time is three sorts", `docs/decisions/semantics-v2.md`
§7). Antigravity reviewed the design. `Window` is pronoun scope and is not
touched. Standard constraints apply.

## Shape

- `TurnRef = current | previous | next (player) | each (player) | later |
  that`. `that` is today's `Reach.thatTurn`. There is no `this`.
- `Timing`: `inPart (part : TurnPart) (whose : Option NounPhrase) (which :
  TurnRef)`, `relativeToEvent (side : before | after) (event : GameEvent)`,
  `whileDoing (event)`, `onlyIf (condition)`, `and`, `or`, `not`. Every
  timing field takes one: activated, triggered, spell, `StaticSpec.replacement`,
  `DeonticRider.play` and the special action of the face-down ticket.
  - Macros: `asSorcery = and([inPart(mainPhase, you, current),
    onlyIf(not(exists(inZone(stack))))])` [CR#307.5], and `asInstant =
    onlyIf(hasPriority(you))`.
  - New: `Condition.hasPriority (player)` [CR#117.1d]. If no timing is
    written, a play is allowed any time you have priority.
- `Duration`: `between (start : Option GameEvent) (end_ : Option GameEvent)`
  and `forAsLongAs (condition)` [CR#611.2b].
  - `GameEvent` gains `endOf (part) (whose) (which : TurnRef)` and `now`.
    `beginningOf` gains `which : TurnRef`. Fold `PartQuant` into it where it
    says the same, and name the result in the record.
  - Readings: "until end of turn" is `between(none, endOf(turn, current))`.
    "Until your next turn" ends at `beginningOf(turn, you, next)`. "On a later
    turn" is `between(endOf(turn, current), none)` [CR#702.143a,702.170d].
    History windows end at `now`.
- `UsageLimit (times : CountBound) (per : Period)`, `Period = turn |
  yourTurn | game | part (TurnPart)` [CR#602.5b]. `RankPeriod` stays an event
  ordinal and shares only `Period`.
- `LookbackClause (event) (within : Duration) (timing : Option Timing)`.
  `Lookback.thisWay` and `Lookback.triggering` name no span. Find them a
  home that is not a `Duration`, or STOP.
- Checker: references and kinds only. A `whose` noun must be player-kinded.
  Nothing else is refused (the 2026-10-06 charter).
- Rust mirror, RON helpers (`untilEndOfTurn`, `thisTurn`, `asSorcery`,
  `onceEachTurn`, …) re-pointed, and bench macros aligned.

## Retires

The old `Timing` (`asSorcery | asInstant | duringPart | beforePart`),
`PlayTiming`, `PlayLimit`, the old `UsageLimit`, `Concurrent`, the old
`Duration`, `DurationEnd`, `Lookback`, and `Reach.thatTurn` (into
`TurnRef.that`). `JoinedHeader.while_` becomes a `Timing`.

## Re-spells (same asserted outcome)

Every pin and canon card that writes a retired timing, duration, limit or
lookback. That includes `lean/Semantics/Proofs/Lookback.lean`, the timing
and duration cases in `Deontic.lean`, `EventPatterns.lean`, `Faces.lean` and
`GetsBothDeltas.lean`, Angus Mackenzie
(`relativeToEvent(before, beginningOf(combatDamage, current))` [CR#510]),
and Unleash Fury.

## Bodies (decided here)

flash [CR#702.8a], sneak [CR#702.190a], miracle [CR#702.94a], forecast [CR#702.57a],
boast [CR#702.142a] and warp [CR#702.185a] get their bodies in this ticket,
because each needs only the temporal sorts. Foretell, plot and suspend need
the special action, so they go to `semantics-v2-face-down-and-special-actions`.

## Witnesses

- Lion's Eye Diamond (`hasPriority`, added to canon); Mystic Veil [CR#307.5a]:
  `happened(cast(spell), within(thisGame), not(asSorcery))`.
- One "until your next turn" card and one "during your next turn" card.
- Pins: a `whose` that is not a player is refused, and a turn reference
  `that` with no antecedent turn is refused.

## Deltas

Glossary entries Timing, Duration, Usage Limit and Turn Reference are already
in `docs/contexts/game-model/CONTEXT.md`. Amend them if the build shows a
flaw. Runs in parallel with cost-substitutes and piles. Refresh before
integrate, and expect conflicts in `Words`/`Phrase`/`Abilities`.
