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

## Decided (owner, 2026-10-05): option A

The group handoff's `.many` arm carries the body's amount outcomes through as
singular totals, the way `doesProfile`'s plural-agent arm (`.many, _`,
`Abilities.lean:1077`) keeps the body's deed for an `enact` with a plural
agent. Owner: "yeah that's a match for how it's written so i guess so".
Extort's "you gain life equal to the total life lost this way"
[CR#702.101a] is the reading.

Not chosen: leaving every outcome binding singular in `pluralizeBinding`
(which would change `doForEach` as well), and counting a plural amount
outcome as a total in `countAmountOutcomes`.

## The work

An ordinary landing, under one rule: the smallest change to the `.act` arm
that keeps every existing pin at its outcome and the three group pins
(`handedGroupPublishesPlurals`, `okDistributedDiscardReadsAsGroup`,
`badDistributedDiscardReadSingular`) at theirs.

The goal spelling, proving on Syndic of Tithes:

```ron
sequentially([act(each(opponent), loseLife(1)), gainLife(thatMuch)])
```

Re-spell `extort.ron` to it.

## Proof

- Extort written `act(each(opponent), loseLife(1))`, then
  `gainLife(thatMuch)`; Syndic of Tithes proves `Card.check = []`.
- A Lean pin for the handed-off form beside the explicit-agent one, both
  checking clean.

## Routed on

After this lands no caller in `plugins_v2/` passes `changeLife` an agent;
dropping the alias's agent parameter is `semantics-v2-drop-agent-fields`'
step, not this ticket's (that ticket already needs this one).

## Landing record

The series, oldest first: `xwruuvomlkqw` (claim), `qqkkrzwlwwks` (Lean
checker and pins), `vkkvnouuvlpw` (extort re-spelled, `changeLife`
comment), `orqpnpuumtwr` (`lean/CONTRACTS.md`, ADR §12.1 sentence, the
`semantics-v2-drop-agent-fields` note), and this record. Each stage was
gated on its own tree; the code tree is the same at `vkkvnouuvlpw`,
`orqpnpuumtwr` and the record, which change only `docs/` and
`lean/CONTRACTS.md`.

**The change.** `Instruction.profile`, `.act who body`, `.many` arm
(`lean/Semantics/Check/Abilities.lean:1222`): of what the body introduced
(`fresh`), every singular amount outcome (`Binding.isAmountTotal`, `:1100`,
the bindings `countAmountOutcomes` counts) is the handoff's deed and stays
singular; the rest is pluralized as before.

```lean
let fresh := bodyP.intro.take (bodyP.intro.length - inner.length)
⟨bs, pluralizeIntroduced (fresh.filter (!·.isAmountTotal)) ++
  pluralizeIntroduced (NounPhrase.introduced bs who) ++ bs, none,
 fresh.filter Binding.isAmountTotal⟩
```

Why the explicit agent already checked: its `.changeLife (.down a) who` arm
publishes `outcomeB .lifeLost`, singular whatever `who`'s number, as the
deed; the handoff took the same binding into `pluralizeIntroduced`, so
`countAmountOutcomes` skipped it. The total is taken from the body's whole
introduction, not only its deed, because a sequence body's `.last` moves
its deed into what it announced. `doForEach` is unchanged.

**Proof.**
- `cd lean && ./scripts/build` (lake build, `--wfail`): Build completed
  successfully (82 jobs), on `qqkkrzwlwwks`. The first full build after the
  checker change rebuilt every proof module and the whole bench before any
  pin was added, so every existing pin keeps its asserted outcome, the
  three group pins (`handedGroupPublishesPlurals`,
  `okDistributedDiscardReadsAsGroup`, `badDistributedDiscardReadSingular`),
  `Proofs/Trigger.lean` `okLifePaymentThatMuch` and
  `badKeywordCostPaymentThatMuch`, and the ten cost twins among them.
  `Proofs/Actor.lean`: 44 → 51 theorems.
- `cargo xtask lean-check`: on `qqkkrzwlwwks` (old extort spelling) canon
  127/127, testing 5/5 (94.6s); on `vkkvnouuvlpw` canon 127/127, testing
  5/5 (94.2s). Syndic of Tithes proves `Card.check = []` with the emitted
  term `act (each opponent) (changeLife (down 1) actor)`, the same term as
  the Lean pin `okExtortHandedToEachOpponent`.
- Gate: on `qqkkrzwlwwks`, `cargo xtask gate --changed` reported "No
  workspace crates are affected by the changed paths" (Lean only). On
  `vkkvnouuvlpw` and after, it derived `cargo test -p
  deckmaste_construction_core -p deckmaste_lexical_source -p
  deckmaste_semantics_v2 -p deckmaste_construction_v3 -p
  deckmaste_english_v3 -p xtask`; run with `--no-fail-fast`: 88 test
  binaries, 1180 passed, 0 failed, 2 ignored (both pre-existing, each
  naming its blocker: `seedborn_muse_retains_relative_clause_attachment`,
  `macro_schema_census_count_matches_21`).
- `cargo xtask facts check`: up to date, every stage. `cargo xtask cite
  check --list-noncompliant`: 0; `cargo xtask cite check`: 0 stale (16012
  citations on the record's tree); `cite audit --diff` read for every stage's new
  `[CR#702.101a]` sites.
- `cargo xtask expansions` before and after (session scratch): 1511
  declarations, 0 failed. Changed declaration: `extort` only.

**Term change, classified.** Handoff class: extort's loss
`ChangeLife(delta: Down(1), agent: Described(each, opponent))` →
`Act(player: Described(each, opponent), body: ChangeLife(delta: Down(1),
agent: Actor))`. Changed card terms: Syndic of Tithes (through `extort`, its
source unchanged). No other declaration or card term moved.

**Pins added** (`lean/Semantics/Proofs/Actor.lean`, "A group handoff
publishes its amount outcomes as totals"), each closed by `decide`:
- `okExtortHandedToEachOpponent`: Syndic of Tithes with extort over
  `act (each .opponent) (Actor.loseLife 1)` then `Actor.gainLife .thatMuch`:
  `Card.check = []`.
- `extortHandoffTwin`: the explicit-agent spelling `changeLife (down 1)
  (each opponent)` checks `[]`, and the handoff spelling's refusals equal
  it.
- `handedGroupPublishesTotal`: `act (each .opponent) (Actor.loseLife 1)`
  publishes `[(outcome, one, the), (player, many, each)]`, and its
  bindings are equal (`==`) to the explicit plural agent's.
- `badGroupHandoffNoOutcomeThatMuch`: "Each opponent discards a card. You
  gain that much life." = `[.quantOutcomeInScope 0]`.
- `okGroupHandoffSequenceReadsLifeTotal`: "Each opponent loses 1 life and
  discards a card." then "that much" checks `[]` (it reads the life total),
  and then "exile them" checks `[]` (the cards stay a group).
- `badGroupHandoffTwoTotalsThatMuch`: a body that loses life twice leaves
  two totals, `[.quantOutcomeInScope 2]`.
- `badForEachLossThatMuch`: `doForEach (each opponent) (act they (loseLife
  1))` then "that much" = `[.quantOutcomeInScope 0]`: the for-each is not
  changed.

**Pins changed.** None. No existing pin's statement or expected value
moved.

**Tests.** Restored: 0. Re-spelled: 0 (no Rust test names the extort term;
the only `extort` hits under `crates/` are the facts table's `Extort` label
and regime, unaffected). Ignored: 0 new. Added: 7 Lean pins. Removed: 0.

**Deviations and additions.**
1. `Binding.isAmountTotal`, a new one-line predicate beside
   `statedNumbers`, names the bindings `countAmountOutcomes` counts.
2. The ticket said the body's amount outcomes; they are taken from all the
   body introduced, not only its deed, so a sequence body (`loseLife` then
   `discard`) still publishes its total (`okGroupHandoffSequenceReadsLifeTotal`).
   With the deed alone that case would stay refused.
3. The totals leave the pluralized group rather than appearing twice
   (singular in the deed, plural below); the published bindings then equal
   the explicit plural agent's exactly (`handedGroupPublishesTotal`).
4. Beyond the ticket's letter: `badGroupHandoffTwoTotalsThatMuch` and
   `badForEachLossThatMuch`, which pin what the change makes observable.
5. `changeLife.ron`'s comment no longer says extort needs the agent; the
   parameter stays (routed).

**STOPs.** None. Option A held all four conditions at once.

**Glossary.** No gap found.

**Not applicable.** Coverage lock, selection census, licensing-checker
totals, homograph and form-literal inventories and the performance
advisory: no English grammar, lexicon or corpus input changed, so
`coverage` was not run.

**Routed.**
- `semantics-v2-drop-agent-fields`: `grep -rn "changeLife(" plugins_v2`
  finds no caller outside the alias's own declaration, so nothing passes
  `changeLife` a performer; its `agent` parameter can go there (dated line
  added to that ticket).
