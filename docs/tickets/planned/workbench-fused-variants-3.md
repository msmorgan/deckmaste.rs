---
needs: [workbench-clause-order-collapse]
---
**Collapse the separable fused variants, delete the dead proofs and unused
constructors, gate `StatOf`, and rename the overloaded constructors.** Fresh
workbench review 2026-09-03: F5, F6, F8, F10, F11, F12, F14, F15, F17, F18,
plus the `LookReq` collapse of ruling 8. One round, ten independent edits.

- **F10 — chosen/last-chosen quadruplet → one axis (S–M).**
  `Predicate.ChosenPlayer`/`TheLastChosenPlayer`, `OfChosen`/
  `OfTheLastChosen`, `ColorTerm.ThatColor`/`TheLastChosenColor` and
  `Amount.ChosenNumber`/`TheLastChosenNumber` differ only in the gate
  (`countChoice … = 1` versus the choice-stands variant). Add
  `data ChoiceRef = TheChoice | TheLatestChoice`, one constructor per family,
  `choiceRefOk ref n`. **"The last chosen" stays the printed lemma** — the
  macro spelling keeps that wording; only the constructor pair collapses.
  `TheLastChosen*` has 6 bench uses.
- **F11 — lookback triplet → one record (M).** `Predicate.HappenedTo`,
  `Condition.Happened` and `Amount.EventTally` each carry the same five
  `(ev, w, what, {cw : ComplementWritten what}, {sb : LookbackSubject ev k})`.
  `record Lookback bs k` holds them and the three constructors take it
  (`EventTally` adds `op`/`tallyOk`); `Macros.happened*`/`eventCount*` shrink.
- **F12 — `Phrase.Noun.PossessorOf`/`PossessorsOf` (S).** Fold into
  `PossessorOf ax n`; plurality comes from `n` and the `one` gate goes
  (`nounDelta` already branches on plurality).
- **F14 — small duplicates (S each).** `Cost.ScaledMana unit amt` ⊂
  `Cost.ScaledCost (Mana run) amt`: keep the readable `ScaledMana` spelling as
  a macro over `ScaledCost` (0 bench uses against 14). `Effect.SkipsAllOf` →
  `Continuously (Skips …) (Just RestOfGame)`. `RemoveCountersAmong` →
  `RemoveCounters` over a `SomeOf … grp` noun.
- **F15 — unused constructors (S).** `Effect.AsThoughLess`,
  `Effect.CostShiftRunWithFloor`, `Effect.Repetition.AgainExcept`,
  `Triggers.GameEvent.Triggers` and `Words.AttachWord.Fortified` (no Fortify
  row either) have zero bench uses. Delete each, or land the card that needs
  it with its pin — the landing record says which, by name.
- **F5 — `Phrase.Amount.StatOf` is not type-gated (S).** `Compare`/
  `Superlative` seed `comparedType` but `StatOf c n` takes any object noun:
  P2 `StatOf Power (target land)` and P7 `StatOf Loyalty (target (HasType
  Battle))` are both admitted, and a noncreature permanent has no power or
  toughness [CR#208.3]. Add `{auto 0 ty : So (fit of comparedType c against
  nounTy n)}`, `Nothing` permissive (animated lands are real). `Defense` on
  `Characteristic` [CR#109.3] is optional — no supported card compares it.
- **F6 — dead proofs in `Words` (S, ~90 lines).** `kindLteComm`,
  `kindLteAssocR`/`L`, `sameKindRefl`, `sameCardTypeEq` and `sameQEq` have no
  references, and `cardTypeAt`, `cardTypeAtIx`, `qualityAt`, `qualityAtIx` and
  `natEqSo` exist only to prove them. Keep `kindLteRefl`, `kindLteJoinL`/`R`
  and `kindLteInL`/`R`.
- **F8 — overloaded constructor names (S).** `Until` (`Triggers.Duration` /
  `Effect.Repetition`), `Named` (`Words.CounterKind` / `Phrase.Predicate`),
  `Define` (`Effect.StaticEffect` / `Effect.Effect`), `Up`/`Down`
  (`Words.Counter.Delta` / `Phrase.LifeOp`), `Bare` (`Words.Reach` /
  `Phrase.ZoneScope`) → `UntilCond`, `NamedCounter`, `DefinesLetter`,
  `LifeUp`/`LifeDown`, `BareScope`. The sixth pair (`StaticFirstDone`) is gone
  already — hence the dependency on `workbench-clause-order-collapse`.
- **F17 (S).** `Macros.Characteristic` → `Stat`, `CharAxis` → `StatAxis`: the
  CR's characteristics include name, colour and types [CR#109.3], which the
  grammar models as `Quality`; this sort is only the numeric quarter.
- **F18 — `scripts/check-eq-indexes` (S).** Keep the index-based-`Eq` check,
  drop the hard-coded count of 47.
- **Ruling 8 — `Macros.LookReq` (S–M).** Scry keeps its expanded body; the
  leaf-with-facts-row option is declined. `LookReq`'s four proof-carrying
  cases (`YourOneLookReq`, `YourManyLookReq`, `TheirOneLookReq`,
  `TheirManyLookReq`, `Macros.idr:1385–1446`) exist only because the agent
  noun cannot be re-read after `agentIntro`. Collapse them with an `Own`-style
  re-read of the agent's own delta, so one macro serves all four.

Also (copies-are-spells residue): after a one-shot `Copy` there is no spelling that reads the ORIGINAL spell (the copy binding carries `prov = Nothing`, and `TheVerbed "Copy"` is pinned unwritten); no bench card needs it yet — when one does, give the copy a provenance stamp or an `Other`-style exclusion rather than a new `Reach`.

Also (turn-parts residue): `Timing.BeforeAttackersDeclared` is a special case of a generic before-point timing; "only before the combat damage step" (Angus Mackenzie, Berserk, Blood Frenzy) needs `Timing.BeforePart (p : TurnPart)`; fold the special case into it and bench Berserk.

Also (levelers residue): `Words.keywordFacts` has no `LevelUp` row and there is no `Macros.levelUp`, so the three leveler witnesses spell "level up {cost}" as the ability it represents; add the row (the facts-from-ron generator's stub if one exists) and the macro, rewrite the three witnesses through it.

Done when: every constructor and proof named above is gone from the tree and
`grep` finds no residue; the collapsed families each have a bench witness for
both former spellings ("the last chosen" and "the chosen", a `Lookback` in
each of its three sorts, a singular and a plural possessor);
`StatOf Power (target land)` is refused and pinned, probed non-vacuous, while
an animated land still reads its power as a witness; `check-eq-indexes` passes
with no number in it; every renamed constructor's pins are re-spelled and
still refute; a scry card reads through the single `LookReq`; the build is
44/44 with 0 errors and 0 warnings. Standard constraints apply, plus the
RON-shaped constraint: a core constructor is admissible only if the RON
re-emitter can produce it from a RON node, and a macro only if it names a RON
macro (`docs/decisions/workbench-ron-shaped-and-label-rulings.md`).
