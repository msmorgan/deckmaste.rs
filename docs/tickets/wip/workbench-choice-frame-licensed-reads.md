---
needs: []
---
# The choice frame that licenses a later read

Two ledger items from
`docs/tickets/done/workbench-amount-comparison-and-quantity.md`, both NOT
landed, both blocked on the same shape: a choice clause that mints nothing a
later clause can read back off.

## 1. Duneblast — "Choose up to one creature. Destroy the rest."

Quoted from that round's ledger (oracle text verified there):

> `TheRest`'s presupposition is a partitioned group, and a choice mints none:
> neither `countGroups` nor `countParts` sees a choice clause. Landing it needs
> either a new `TheRest` licensor over the world's creatures or a different row,
> which is a ruling this round did not have.

The pair's other half **is** landed — Berserker's Frenzy's counted choice
benches as `berserkersFrenzyLowRoll` — so the delta is exactly the complement
read, not the choice.

## 2. Boreas Charger / Sandstone Oracle / Slithermuse — deferred whole

> "Choose an opponent who controls more lands than you … the difference" needs a
> predicate-level member-relative comparison AND a gap-licensing choice frame.
> Neither exists; building half of it lands none of the three.

Two pieces, and that round's own warning is that half of them is worth nothing:

- **the member-relative comparison at predicate level** — a description picking
  a player by comparing their count against the reader's, rather than against a
  literal; and
- **the gap-licensing choice frame** — the choice that makes "the difference"
  readable afterwards.

Take them together, and note the second is the same phenomenon as §1's: a choice
that leaves a readable residue behind it.

## Neighbours

`AggregateOver`'s own binder is landed and benched; the amount round's other
relativized-count blockers (per-member event mentions, the `{X} less` cost
rider, the Suspect counter kind) are routed elsewhere and are not this round's.
The chooser POSITIONS — who may choose and what a choice may bind — belong to
[workbench-choice-chosen-and-ascription](workbench-choice-chosen-and-ascription.md);
this ticket owns only what a choice leaves behind for the next clause to read.

## Consumption boundary

`idris/src/Experimental.idr` (`Effect.Choose` and its intro, `TheRest` and its
licensor, `countGroups`/`countParts`, `CompareAmt`'s operand slots, the
predicate-level comparison), `idris/src/Experimental/Words.idr` if the
comparison needs a word of its own, the pin modules
`idris/src/Experimental/Proofs*.idr`, and the evidence bench
`idris/src/Experimental/Cards.idr`. No Rust crate.

## Acceptance

- Duneblast benches whole, or the licensor question ends in a written ruling.
- All three of Boreas Charger, Sandstone Oracle and Slithermuse bench — not one
  or two; the round that deferred them said building half lands none.
- `idris/scripts/build` PASS, no witness lost, no pin silently passing.

Standard constraints apply.

## As landed

### The ruling: a choice IS the partition

"The rest" presupposes a set to be the rest OF and something already taken
out of it. Before this round the set had to be a group the text assembled
(`SomeOf`'s "one of them"), and a choice clause assembled none — which is why
Duneblast did not write. The ruling is that a choice makes the partition
itself:

> [CR#608.2d] has the player announce a choice offered by an effect *while
> applying the effect*, and forbids an option that is illegal or impossible.
> The members a choice picks therefore come out of the described set as the
> effect resolves, and the ones it did not pick stay behind. That is a
> partition of the description's extension, exactly the partition "one of
> them" writes over a mentioned group — the only difference being that a
> choice's set is never mentioned, so no binding stands for it. "The rest"
> names the unchosen members of that partition.
>
> The same rule draws the row's boundary. [CR#608.2d] is explicitly about
> choices *other than* those already made as part of casting the spell, and a
> target is announced during casting [CR#601.2c]. A choice clause naming a
> target therefore announces no resolution-time choice and partitions
> nothing; "Choose any number of target creatures. Destroy the rest." stays
> unwritable (`badRestAfterTargetChoice`).

Mechanically: `chosenDelta` marks the mention a choice announces with the
partitive determiner (object side only — only the object side has a
complement to read), and `theRestOk` no longer demands that an assembled
group stand beside the part. `restSource` is what "the rest" reads its
description off: the group when the text wrote one, and otherwise the
choice's own part, which carries the description the choice was made under.
`groupSpent` now spends the partition in both shapes — the group goes and the
parts stop being parts — so one disposition per remainder still holds
(`badRestDisposedTwice` unchanged; `badChoiceRestDisposedTwice` is its choice
twin).

### Rows and licensors

- `Phrase.chosenDet` / `chosenDelta` / `chosenIntro` — the choice clause's
  own intro. `Effect.Choose`'s `effIntro`/`preIntro`/`annIntro` route through
  it. Grounding: [CR#608.2d], with [CR#601.2c] for the target exclusion.
- `Words.theRestOk` widened to `countGroups bs <= 1 && not (countParts bs == Z)`;
  `Words.restSource` added; `zoneOfGroup`/`tyOfGroup` route through it;
  `groupSpent` re-marks spent parts. Grounding: [CR#608.2d].
- `Phrase.Predicate.CompareOver : {k : Kind} -> (dom : Predicate bs k) ->`
  `{auto ph : Phrasal k} -> (measure : Amount (bindFor TheD OneOf ph dom :: (predDelta dom ++ bs))) ->`
  `(r : Comparator) -> (bound : Amount bs) -> Predicate bs k`
  — the member-relative comparison. `Compare` reads one of the referent's own
  characteristics against a bound; this reads a whole amount taken ON the
  referent, over `AggregateOver`'s element binder, so "controls more lands
  than you" compares the member's count with the reader's. The domain rides
  the row for `Superlative`'s reason: nothing outside the row can bind the
  member the measurement is taken on. Grounding: [CR#608.2h] settles both
  counts once when the effect applies, and [CR#608.2c] is what lets the text
  after it name the margin.
- The gap-licensing frame is `predDelta (CompareOver …) = gapB :: …`. The
  margin is the comparison's own residue, as `CompareAmt`'s is; the choice
  frame carries it forward because a choice clause exports its description's
  deltas. No separate choice-side gap mint was needed.

### Bench

- `duneblast`, `duneblastCard` — whole.
- `opponentWithMoreLands`, `boreasChargerChoice`, `boreasChargerDifference` —
  Boreas Charger's first clause and the margin it leaves readable.
- `sandstoneOracle` — whole card.
- `slithermuseTrigger` — the trigger whole.

### Pins

`badRestWithoutAPartition`, `badRestAfterTargetChoice`,
`badChoiceRestDisposedTwice`, `badMemberInComparisonBound` (ProofsG);
`theRestResolvesInPrefix` restated to witness the part alone, with
`theRestGroupResolvesInPrefix` carrying the group witness where the text
assembled one.

### Named, not built

- **Boreas Charger's second clause** — "search your library for a number of
  Plains cards equal to the difference": `Effect.Search` takes no quantity.
  Already routed to
  [workbench-anaphora-mentions-and-creation](workbench-anaphora-mentions-and-creation.md),
  where Boreas Charger is now named as a payoff.
- **Slithermuse's evoke** — the keyword alternative cost, routed to
  [workbench-cost-and-payment-residues](workbench-cost-and-payment-residues.md).
- **`Compare`'s own margin is not minted.** "creature with power greater than
  3 … the difference" stays unwritable. Not a rules refusal: the reason is
  structural, since `TheDifference` resolves by uniqueness and a second gap
  standing in every text that also writes a `CompareAmt` would break that
  read. Recorded here, not gated with a rule.
- **The choice partition licenses "the rest" after ANY resolution-time object
  choice**, including ones no printed card follows with the phrase. Tolerated
  overgeneration: the rules make each of them meaningful, and refusing them
  would need a count.
