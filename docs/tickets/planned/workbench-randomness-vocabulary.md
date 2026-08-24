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
