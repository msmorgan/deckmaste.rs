---
needs: []
---
**A handoff to a group publishes no outcome for "that much" to read, so extort
cannot be spelled over the handoff.** Found at
`plugins-v2-implicit-actor-spelling` (2026-10-05), STOP 2 of its landing
record. Standard constraints apply.

## The case

Extort [CR#702.101a]: "“Extort” means “Whenever you cast a spell, you may pay
{W/B}. If you do, each opponent loses 1 life and you gain life equal to the
total life lost this way.”" Syndic of Tithes (`data/derived/cards.jsonl`):
"Extort (Whenever you cast a spell, you may pay {W/B}. If you do, each
opponent loses 1 life and you gain that much life.)"

The handoff spelling, which the 2026-10-05 ruling makes the only place a
performer is written:

```ron
sequentially([act(each(opponent), loseLife(1)), gainLife(thatMuch)])
```

is refused `[Semantics.Refusal.quantOutcomeInScope 0]` on Syndic of Tithes.
The explicit-agent spelling checks, and is what
`plugins_v2/builtin/macros/keyword_abilities/extort.ron` writes today:

```ron
sequentially([changeLife(down(1), each(opponent)), gainLife(thatMuch)])
```

## Why, in the Lean checker

- `thatMuch` is checked at `lean/Semantics/Check/PhraseRules.lean:415`:
  `refuse (n == 1) (.quantOutcomeInScope n)` with `n :=
  countAmountOutcomes bs`. `countAmountOutcomes`
  (`lean/Semantics/Check/Words.lean:677`) counts only singular outcome
  bindings (`⟨_, .one, .outcome s⟩` with `s.isAmount`).
- The explicit agent: `Instruction.profile`'s `.changeLife (.down a) who` arm
  (`lean/Semantics/Check/Abilities.lean:1165`) publishes `[outcomeB
  .lifeLost]` as its deed whatever the plurality of `who`, and `outcomeB`
  (`Words.lean:624`) is singular. So "each opponent loses 1 life" leaves one
  `lifeLost` outcome, the total, and `thatMuch` finds exactly one.
- The handoff: the `.act who body` arm (`Abilities.lean:1207`), for a plural
  performer (`.many`), returns `⟨bs, pluralizeIntroduced (what the body
  introduced) ++ pluralizeIntroduced (who's mentions) ++ bs, none, []⟩`. The
  body's `lifeLost` outcome survives only inside `pluralizeIntroduced`
  (`Words.lean:917`; `pluralizeBinding`, `:912`, sets `.many` on every
  binding but a `.self` one), and the deed list is empty. The outcome is
  therefore plural, `countAmountOutcomes` skips it, and `n = 0`.
- The arm follows `doForEach` (`Abilities.lean:1231`), which publishes the
  same way. The plural treatment is what pairs each opponent with their own
  mentions; it was not written with outcomes in mind.

## What it blocks

`changeLife` cannot lose its agent parameter while extort needs it: it is
the only `changeLife` call in `plugins_v2/`. So
`semantics-v2-drop-agent-fields` cannot delete `changeLife`'s agent field
until extort can be written `act(each(opponent), loseLife(1))`.

## What a fix must preserve

- The per-player pairing the handoff was given for: what each member's body
  introduced stays published as a group, plural. Pins
  `handedGroupPublishesPlurals` (`lean/Semantics/Proofs/Actor.lean:436`),
  `okDistributedDiscardReadsAsGroup` (`:44`) and
  `badDistributedDiscardReadSingular` (`:52`) keep their outcomes.
- The existing `thatMuch` pins keep theirs, among them
  `Proofs/Trigger.lean` `okLifePaymentThatMuch` and
  `badKeywordCostPaymentThatMuch`.

## Options visible in the code (not a design)

- The `.act` `.many` arm could carry the body's amount outcomes through as
  singular totals, as `doesProfile`'s `.many, _` arm
  (`Abilities.lean:1077`) keeps the body's deed for an `enact` with a plural
  agent.
- `pluralizeBinding` could leave outcome bindings singular, which would
  change `doForEach` as well.
- `countAmountOutcomes` could count a plural amount outcome as a total.

Which is right depends on whether "that much" after "each opponent loses 1
life" is one total [CR#702.101a] in every such sentence, and on what
`doForEach` should publish; neither is settled here.

## Proof

- Extort written `act(each(opponent), loseLife(1))`, then
  `gainLife(thatMuch)`; Syndic of Tithes proves `Card.check = []`.
- A Lean pin for the handed-off form beside the explicit-agent one, both
  checking clean.
- No caller in `plugins_v2/` passes `changeLife` an agent, so the alias can
  drop the parameter here or in `semantics-v2-drop-agent-fields`.
