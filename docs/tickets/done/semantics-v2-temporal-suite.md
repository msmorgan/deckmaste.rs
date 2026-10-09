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

## Landing record

Series on the claim `wnwssvyn`, oldest first: `voqrtksm` (Lean nodes, checker,
bench re-spelled), `lxvzssou` (Rust mirror, expansion samples), `vtslmwwk` (RON
helpers re-pointed, bench macros aligned), `xxskkxzl` (flash and sneak bodies,
pins, facts, STOP notes), `nokxtkup` (canon witnesses), `soprrnnt` (glossary,
citation lock), `mvnxpurq` (this record), `ywpqtwvw` (`Instruction.limited`,
per the orchestrator's ruling on STOP 2). Numbers are measured on `ywpqtwvw`, after refresh onto
`semantics-v2-cost-substitutes` and `semantics-v2-piles` (Galvanic Relay exiles
`slice(Top, 1, library(you))`). `lean/` changed.

1. **Census delta**: not run. No english_v3 input changed (grammar, lexicon and
   keyword spellings untouched). Lost faces: none.
2. **Witnesses**: `cargo xtask lean check`: canon 159/159 (4 new: Lion's Eye
   Diamond `onlyIf(hasPriority(you))`; Hag of Inner Weakness, "until your next
   turn"; Galvanic Relay, "during your next turn"; Donatello's Technique, sneak),
   testing 8/8. `lean definitions` 553/553. Mystic Veil is a pin
   (`Proofs/Turn.okCastWhenASorceryCouldNotHaveBeen`: `happened(cast, thisGame,
   not(asSorcery))`). Pins `badTimingWhoseNotAPlayer` (a non-player `whose` is
   `kindMismatch`) and `badTimingThatTurnWithoutAntecedent` (`that` with no turn
   is `turnAntecedent 0`).
3. **Spot checks**: Lion's Eye Diamond activates under `asInstant`; Hag's -2/-1
   lasts `between(none, beginningOf(turn, you, next))`; Galvanic Relay's play
   permission lasts `between(beginningOf(turn, you, next), endOf(turn, you,
   next))`; Donatello's Technique's sneak permits casting under
   `and([asInstant, inPart(declareBlockers, you, current)])` paying {U} plus
   returning an unblocked creature you control; Ravener's flash is now a play
   permission at `asInstant`; Angus Mackenzie's window is
   `relativeToEvent(before, beginningOf(combatDamage, none, current))`; Unleash
   Fury lasts `between(none, endOf(turn, none, current))`; Storm Fleet Spy reads
   `soFarThisTurn` (`between(beginningOf(turn, none, current), now)`); Graf Rats
   and Luminarch Aspirant trigger at `beginningOf(…, you, each(none))`; Final
   Fortune's delayed trigger is `beginningOf(endStep, none, that)`.
4. **Deviations and STOPs**:
   - `whose` vs `TurnRef.next/each (player)`: both kept as ruled; `player` is
     `Option`, and every producer writes the owner on `whose` and `none` on the
     reference (matching the ticket's `beginningOf(turn, you, next)`).
   - `PartQuant` folded: `the`/`each` in a trigger header → `each(none)`, `the`
     in a delayed trigger → `next(none)`, `byTurn(thatTurn)` → `that`.
     `HeaderPossessor` retired into `whose : Option NounPhrase` (beyond the
     ticket's list; it only carried `byPlayer`/`byTurn`).
   - `Lookback.thisWay`/`.triggering`: homed as sibling constructors
     `LookbackClause.thisWay (event)` and `.triggering (event)`, not Durations.
   - `RankPeriod` retired into `Period` (`within thisGame` → `game`, `each turn`
     → `turn`). `PaidFacet` moved into the phrase block (its readback window is
     an `Option Duration`, field renamed `within`). `CountBound` moved to
     `Triggers.lean`. `UsageLimit` is a one-constructor inductive `mk (times)
     (per)` (a `SupportsMacros` mirror needs an enum).
   - `UsageLimit.actionOncePerTurn` ("Do this only once each turn" [CR#603.2h])
     moved to a second carrier, orchestrator's ruling (not the owner's):
     `Instruction.limited (instruction) (limit : UsageLimit)`, RON/Lean helpers
     `limited` and `doOnlyOnceEachTurn`. Iron Man and Donal are re-spelled to an
     unlimited trigger whose instruction is limited; the old outcomes are pinned
     (`Turn.okDoThisOnlyOnceEachTurn`, `Turn.limitedChecksItsInstruction`). RON
     `actionOncePerTurn`, `until`, `byTurn`, `thatTurn` retired.
   - `lastTurn` ends at `endOf(turn, none, previous)`, not `now`; an
     effect's "this turn" is `between(none, endOf(turn, current))`, the lookback's
     is the new `soFarThisTurn`. `asSorcery` follows the ticket (no priority
     conjunct, though [CR#307.5] names one); orchestrator confirmed both readings:
     an unrestricted timing already means priority, and `hasPriority` is for
     mana abilities that could act without it.
   - `durationPossessor` now applies to a Duration's boundary events and
     admits a target. Pin outcomes changed in name only:
     `Static.badPluralNextTurnDuration` `.singular` → `.durationPossessor`;
     `Static.badSkipDuringUnboundNextTurn` refuses the unbound "their" at both
     boundaries (2 anaphors, bench macro writes it twice); the `thatTurn` pins
     (`Turn.badDeicticTurnWithoutIntroducer`, `Turn.badThatTurnAfterTwoTurns`,
     `Anaphora.badThatTurnWithoutTurn`) now read `turnAntecedent n`, a new
     refusal replacing `anaphor thatTurn` (reference check).
   - STOPs, left bodyless with updated notes: forecast and boast
     (`KeywordParam` has no `Ability`; qualifiers now writable), miracle (a
     timing rides only the play rider, not a reveal permission), warp (static
     that creates a delayed trigger, as dash/blitz). Flash and sneak have bodies.
   - Glossary: Duration amended ("last turn" ends at its turn's end; "this way"
     and the triggering event are not Durations).
5. **Tests**: `cargo test -p deckmaste_semantics_v2` pass (45+7+1+4+43);
   `cargo clippy -p deckmaste_semantics_v2 --tests -- -D warnings` clean;
   `cargo test -p xtask` pass (382+4+14, 1 ignored pre-existing); `cargo clippy
   -p xtask --tests -- -D warnings` clean; `lake build --wfail` 89/89;
   `cargo xtask facts check`, `cite check` (0 stale) clean. Lean: 362 bench
   declarations re-spelled, 14 pins added, 0 removed, 0 ignored. Rust: 3 test
   lines re-spelled (`corpus.rs` ×2, `plugins_v2_declarations.rs` ×1), 2 alias
   reads added (`corpus.rs`), 0 removed.
6. **Timing**: `cargo xtask lean check` 140.8 s wall (one run, default
   workers); census not run.
