---
needs: []
---
# Write the random acts: the coin flip, the die roll and the at-random selection mode

"Flip a coin", "roll a d20" and "at random" are printed oracle text. The
grammar describes what a card says; it does not resolve it, exactly as `Choose`
describes a choice without making one. This round writes the describing
vocabulary and nothing else.

## Context

Source: [the v1/v2 comparison](../../memory/scratch/experimental-vs-semantics-comparison.md),
axis 21 (2026-08-24). **Conductor ruling, and the reason this is a ticket:** the
report's *MISSING (deliberate)* grade conflates description with execution. Only
execution is out of scope. The report itself says the absence "reads as ordinary
chapter-by-chapter scope; it is simply not written down" — this writes it down
and schedules the describing half.

## From the v1 comparison (2026-08-24)

> Crate: `Action::FlipCoins(Reference, Count, bool)`, `RollDice(Reference,
> Count, Uint)`, `RollPlanarDie(Reference)` (`action.rs:452,456,460`);
> `DeciderSpec { Controller, ActivePlayer, DefendingPlayer, Named,
> EachInTurnOrder, PriorityHolder, Rng }` (`decision.rs:17`);
> `Visibility { Open, CommittedHidden }` (`decision.rs:39`); `ChosenValueKind`
> (`decision.rs:60`). `Semantics.idr` even carries a `Vote` constructor the Rust
> side lacks.
>
> Workbench: `ChoiceMode : Bindings -> Type` (`Events.idr:464`) carries chooser
> and at-random per `semantics-v2.md:§7`'s "choice method is surface data" rule
> — which covers "a creature of their choice" and "at random" and nothing else.
> No coin, die, vote, or secret-choice construction.

Mirror how that decision subsystem carves the space — decider, visibility, value
kind — but **import none of its execution machinery**. `DeciderSpec::Rng` is an
engine decision; this grammar writes the sentence that says a die is rolled.

## The measured surface

Distinct oracle-text lines in the grammar corpus:

- **65** write "flip a coin"; **35** write "win the flip" and **41** "lose the
  flip" — the two arms are a near-universal pair, so the arms are part of the
  construction and not a coincidence of phrasing.
- **7** write "coin flip" as an *event* ("Whenever you win a coin flip").
- **55** write "roll a d⟨N⟩"; **29** read "the result"; **141** lines are
  outcome-table rows of the `1—9 |` form.
- **179** write "at random".

### Witnesses to name

- **Goblin Archaeologist** — "{R}, {T}: Flip a coin. If you win the flip,
  destroy target artifact and untap this creature. If you lose the flip,
  sacrifice this creature." Both arms are effects the grammar already writes, so
  the flip and its two arms are the whole blocker.
- **Contraband Livestock** — "Exile target creature, then roll a d20. / 1—9 |
  … / 10—19 | … / 20 | …" — the die's numeric result ranged over by an outcome
  table.
- **Hypnotic Specter** — "Whenever this creature deals damage to an opponent,
  that player discards a card at random." A two-line card whose other half is
  built.
- **Chance Encounter** — "Whenever you win a coin flip, …", the event-side
  reader, if the round takes it.

Re-measure before building; these counts are lines, not cards.

## What the round writes

- **The coin flip** [CR#705.1], with its win arm and lose arm. Decide whether
  the arms are slots on the flip construction or a conditional over an outcome
  the flip introduces — the 35/41 pairing is the evidence either way.
- **The die roll** [CR#706.1] — the rule says an effect "will specify what kind
  of die to roll and how many of those dice to roll", so the kind and the count
  are constructor arguments, not defaults. Its **numeric result** is forward
  readable like any outcome; "the result" (29 lines) is that read.
- **The outcome table**, if the 141 rows are one construction: ranges over the
  roll's result, each with its own effect.
- **"At random"** as a selection mode on an existing mention or selection shape
  — `ChoiceMode` already carries an at-random cell per `semantics-v2.md:§7`, so
  measure what the 179 lines want that it does not give before adding anything.

Each construction states what it introduces into `bs`: the flip's outcome and
the roll's number are ordinary forward-readable mints and must thread like one.

## Boundaries, so nothing is built twice

- **Voting and secret choices are not this round's.** They are a protocol bundle
  owned by
  [workbench-turn-structure-and-procedures](workbench-turn-structure-and-procedures.md)
  ("the secret-choice and voting pair"), which forbids taking a slice of a
  bundle. Menacing Ogre's secret number belongs there.
- **Die results as an amount** are already ledgered: die results are 12 lines on
  the X-rider's blocked right sides in
  [workbench-amount-comparison-and-quantity](workbench-amount-comparison-and-quantity.md),
  and 5 of that ticket's "difference between" lines are dice Contraptions. The
  read and the roll must land compatibly; whoever goes second reuses the first's
  shape.
- **Karplusan Minotaur** is recorded in
  [workbench-cost-and-payment-residues](workbench-cost-and-payment-residues.md)
  as blocked because "no coin-flip verb in the vocabulary at all". This round
  removes that blocker; it does not take that card's cumulative-upkeep half.
- **No execution semantics.** No randomness source, no decider resolution, no
  planar die beyond whatever a corpus line actually writes.

## Consumption boundary

The Idris workbench only: `idris/src/Experimental.idr` (`Effect` — the flip and
the roll as clauses; `Amount` for the result read; the conditional carriers the
arms use), `idris/src/Experimental/Events.idr` (`ChoiceMode`; a flip/roll event
row only if the event-side reader is taken), `idris/src/Experimental/Words.idr`
(the die kind, the outcome-range vocabulary, the at-random word),
`idris/src/Experimental/Macros.idr` (spelling), the pin modules
`idris/src/Experimental/Proofs*.idr`, evidence bench
`idris/src/Experimental/Cards.idr`. No Rust crate.

## Acceptance

- Goblin Archaeologist, Contraband Livestock and Hypnotic Specter bench, or each
  shortfall is named exactly.
- No execution semantics land: nothing resolves a flip, chooses a number, or
  models a randomness source.
- The flip's outcome and the roll's number thread into `bs` like any other
  forward-readable mint, and the read sites are the existing ones.
- The die's kind and count are written arguments, per the rule, not defaults.
- The at-random mode is measured against the existing `ChoiceMode` cell before
  anything new is minted; if the cell already serves the 179 lines, the round
  says so and adds nothing.
- The voting/secret-choice bundle is untouched and Menacing Ogre is not claimed
  here.
- The result-as-amount read is compatible with the amount ticket's ledgered 12
  die-result lines.
- `idris/scripts/build` PASS, no witness lost, no pin silently passing.

Standard constraints apply.

## As-landed

Standard constraints applied. `idris/scripts/build` 19/19 PASS on a clean
rebuild (`rm -rf build` first). **Four pins added**, none retired, none
silently passing — each has a live positive counterpart on the bench.
`Experimental/Cards.idr` binds no implicits; no `{default` anywhere under
`Experimental*`. Every count below was re-measured this round with the corpus
scripts, and every witness's oracle text re-fetched with `card`. The ticket's
numbers were all low; corrections are inline.

### Re-measured surface

| Ticket said | Measured | What it counts |
|---|---|---|
| 65 | **78** | "flip a coin" |
| 35 | **40** | "win the flip" — and all 40 share a line with "flip a coin" |
| 41 | **46** | "lose the flip" |
| 7 | **8** | "coin flip" as an event ("Whenever you win a coin flip") |
| 55 | **62** | "roll a d⟨N⟩" |
| 29 | **62** | "the result" |
| 141 | **141** | outcome-table rows — 74 two-ended `N1—N2`, 41 `N+`, 26 bare `N` |
| 179 | **273** | "at random" |
| — | **6** | "comes up heads"/"comes up tails" — a count the ticket did not take |

### Decisions the ticket left open

**1. The arms are conditions over an outcome the flip introduces, not slots
on the flip.** Three pieces of evidence, and they agree. (a) The pairing is
not universal: 78 flip lines against 40 win-arm lines, and every win-arm line
shares a line with "flip a coin", so 38 flips carry no arm at all —
Karplusan Minotaur's "Cumulative upkeep—Flip a coin" is a bare one. (b)
[CR#705.2] gives a flip **two** readings, not one: effects that "care only
about whether the coin comes up heads or tails", for which "no player wins or
loses a coin flip", and all other effects, where the flipper calls it and
wins or loses. A winner is therefore a reading OF the flip, and slots would
have to carry both vocabularies and stand empty for the bare flips. (c)
[CR#705.1] describes a complete flip without mentioning a winner. Landed:
`Effect.FlipCoins (who) (count)` minting `outcomeB CoinFlipped`, and two
conditions over it — `Condition.FlipCalled (who) (call : FlipCall)` for the
called reading, `Condition.FlipFace (face : CoinFace)` for the uncalled one,
which takes no subject because [CR#705.2] says no player wins that kind. Both
gate on `So (coinFlipInScope bs)`, an existence test built exactly like
`damageDealtInScope`, and both introduce nothing, like `Happened` and
`DealtThisWay`.

**2. The outcome table is one construction, it reuses the existing range
vocabulary, and it is a separate clause.** [CR#706.3a]'s three left-column
forms — a single number, "N1–N2", "N+" — are `Range (Just n) (Just n)`,
`Range (Just lo) (Just hi)` and `Range (Just lo) Nothing`, so **no new bound
or range vocabulary was minted**; `RollRow` carries `Quantity` with the
vocabulary's own `NonZeroQ` and `WellFormedQ` gates ([CR#706.1a] numbers a
die from 1, so a zero ceiling covers no result). `Effect.ResultsTable (rows :
List (RollRow bs))` is a clause gated on the roll's mint rather than a slot
on `RollDice`, because [CR#706.4] has rolls that carry no table and
[CR#706.3b] binds the roll, its modifiers and its table into one **ability**
without binding them into one sentence — a modifier clause may stand between.
It introduces nothing: only one striation happens [CR#706.3a].

**3. "At random" needs nothing beyond the existing `ChoiceMode` cell, and
nothing was minted.** Of the 273 lines, ~128 are the singular "a/an … at
random" that `Indefinite AtRandom` already writes (`Macros.aAtRandom`, live
at three `Cards.idr` sites before this round) — that is Hypnotic Specter's
half. The measured shortfall is **not in the mode vocabulary**: it is that
`CountedGroup` carries no `ChoiceMode` at all, which blocks ~30 counted-plural
lines ("discards two cards at random", "discard X cards at random"), and the
same gap blocks the same shape for the chooser cell — 35 lines write a plural
"… of their choice" ("sacrifices two permanents of their choice"). The
partitives ("one of them at random", 7; "one of the following at random", 5)
and "choose one or more modes at random" are mention-shape gaps too. So the
at-random cell serves, the round adds nothing, and widening which mention
shapes carry a `ChoiceMode` is a different axis — ledgered below.

### What else landed

- **The die roll.** `Effect.RollDice (who) (count : Amount) (sides : Nat)`
  with an `IsSucc sides` gate. Kind and count are both written arguments and
  neither has a default, per [CR#706.1]; `sides` is a `Nat` rather than a
  closed enum because [CR#706.1a] admits any positive N, and "d6" and
  "six-sided die" are two spellings of one term. Mints `outcomeB RollResult`.
- **The result read.** `Amount.TheResult`, gated `countOutcomes RollResult bs
  = 1` — sorted to the roll exactly as `PreventedThisWay` is sorted to
  prevention, rather than left to `ThatMuch`. `readAmount TheResult = True`,
  because printed text puts it on a comparison's left ("If the result is 1,
  …", "If the result is 0 or less, …"); `TheDifference` is the precedent.
  **This is the read the amount ticket's 12 die-result lines reuse**: the
  roll's number is an ordinary forward mint and `TheResult` is its read.
- **`ThatMuch` narrowed to outcomes that carry a number.** The round put a
  non-quantity into the `Outcome` kind, whose only general reader presumed a
  quantity, so the gate moved from `countOnes Outcome bs = 1` to
  `countQuantOutcomes bs = 1` (`outcomeIsQuantity`, False for `CoinFlipped`
  alone). `quantOutcome` and `countQuantOutcomesIsFold` were added in
  `ProofsAnaphora` and the two `thatMuch…Prefix` grounding lemmas re-proved
  against the new gate; `theResult…Prefix` joins them. No existing term was
  lost — the clean rebuild is green.
- **Karplusan Minotaur's cost-side blocker is removed**:
  `costActionOk (FlipCoins who _) = costNounOk who`, so "Cumulative
  upkeep—Flip a coin" composes as a cost. That card's cumulative-upkeep half
  is still not this round's.
- **No execution semantics landed.** Nothing resolves a flip, chooses a
  number, or models a randomness source; no decider, no visibility, no value
  kind was imported. The voting/secret-choice bundle is untouched and
  Menacing Ogre is not claimed here. No planar-die row, but not for want of
  a surface: 6 lines write "roll the planar die", one of them the imperative
  "{T}: Roll the planar die." (Fractured Powerstone). It is ledgered below,
  not measured away.

### Pins

Four, all in `ProofsG`, each naming the rule that makes its term meaningless;
none is justified by a count.

| Pin | Refuses | Rule |
|---|---|---|
| `badThatMuchAfterFlip` | "Flip a coin. Draw that many cards." | [CR#705.2] gives a flip a face and, when called, a winner — no number; [CR#706.2] is where a randomiser leaves one |
| `badFlipArmWithoutFlip` | "If you win the flip, …" with no flip | [CR#705.2] — the arm reads a flip the text made |
| `badTableWithoutRoll` | "1—9 \| Draw a card." with no roll | [CR#706.3a,706.2] — a striation ranges over the result, and no roll leaves none |
| `badNoughtSidedDie` | "Roll a d0." | [CR#706.1a,706.1] — N is positive and the die's outcomes are numbered 1 to N |

Positive counterparts on the bench, so none passes vacuously: the flip arms
and `ThatMuch`-after-damage in `goblinArchaeologist` and the existing damage
cards, the table and the d20 in `contrabandLivestock`.

### Ledger

| Item | State after this round | Exact missing piece |
|---|---|---|
| Goblin Archaeologist | **benches whole**, as `goblinArchaeologist` | — |
| Contraband Livestock | **benches whole**, as `contrabandLivestock` | — (needed two creature-subtype rows, `Ox` and `Boar`; `Goat` existed) |
| Hypnotic Specter | body benched (`hypnoticSpecterDiscard`); at-random half proven | the header. "This creature deals damage to an opponent" is a source-side, non-combat damage event, and `GameEvent` carries only recipient-side `IsDealtDamage` and combat-only `DealsCombatDamage` — the ticket's claim that its other half is built is wrong on the header |
| Chance Encounter | not taken | the event-side reader: an `EventName` and `GameEvent` row for winning/losing a coin flip (8 lines), which the ticket made optional |
| "Whenever you roll one or more dice" | not taken | the same event-side axis, roll side |
| Karplusan Minotaur | cost-side blocker removed | the coin-flip trigger event (above) and cumulative upkeep |
| "discards two cards at random", "discard X cards at random" (~30 lines) | blocked | a `ChoiceMode` slot on `CountedGroup` — 51 call sites, and the same slot unblocks the 35 plural "… of their choice" lines. A mention-shape round, not a randomness one |
| "choose one of them at random" (7), "one of the following at random" (5) | blocked | the same slot on `SomeOf` / on the modal list |
| "choose one or more modes at random", "select one or more targets at random" | blocked | a mode-list and a target-list selection mode; same axis |
| "Flip five coins. … for each coin that comes up heads" | blocked | a count over flips — the `EventCount` axis, not this round's |
| "roll two six-sided dice … If you rolled 7" | writable as a roll; the total is not | a read of several dice summed; `RollResult` mints one singular result, per [CR#706.2]'s per-roll definition |
| "roll that many dice plus one", "ignore the lowest roll" | blocked | roll modifiers and ignored rolls [CR#706.2b,706.6] — deliberately out |
| "Roll the planar die" (6 lines, 1 imperative + 4 triggers + 1 reminder) | blocked | a planar-die instruction and its event row; the die is not an N-sided die [CR#706.1a] and has its own faces — a Planechase axis, not this round's |
| Menacing Ogre, voting, secret choices | untouched, by boundary | owned by `workbench-turn-structure-and-procedures` |
