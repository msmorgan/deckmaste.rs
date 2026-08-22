---
needs: []
---
# Type both conditional orientations

The coordination half of this ticket moved to `workbench-coordination-family`
on 2026-08-22; this ticket now covers the conditional containers alone.

## Conditional orientations and containers

One design across both conditional containers, plus the negation boundary found
inside the settled family. The orientation question has grown a second payer
since chapter 58 and is the whole blocker on three cards.

- **The STATIC orientation.** About 70 lines pronominalise the STATEMENT's
  subject inside the condition ("X has hexproof as long as IT's untapped"),
  where chapter 50's fronted flow pronominalises the CONDITION's subject in the
  body.
- **The ONE-SHOT orientation.** `Effect.If` types the trailing orientation only
  — its condition sits at `preIntro e`, which is what `badTrailingPostStateZone`
  refuses a post-state zone at — while SPELLING both. So a fronted condition
  cannot contribute anything to its own consequent, which costs Balance of
  Power, Vraska's [−9] and Iymrith their gap read. Balance of Power's condition
  half compiles today, so the orientation is that card's whole blocker.
- **One constructor cannot type both arguments in each other's context**
  (finding 362). Both halves are therefore SECOND ORIENTATIONS — carried by the
  marking or by the order, whichever the corpus supports — and not a second
  `condIntro`. The independent-condition trailing forms are writable today and
  are NOT part of this.
- **The counted-condition "unless"** (finding 354), small: the conditional
  static's `Unless` marking demands a NEGATABLE condition and `condNegatable`
  answers False for `CompareAmt`, so "unless you control an artifact" composes
  and "unless you control four or more artifacts" (Gadrak) does not. It belongs
  to the comparison's negation gap, not the deontic's. Measure before
  scheduling.

## Reading-order inversions this ticket owns (added 2026-08-21)

- **`Effect.If` is oriented to the postposed sentence.** `If (e) (c : Condition
  (preIntro e)) otherwise` lets the condition read the effect's mentions —
  "Counter target spell if it's red" — and that is reading order for that shape.
  The bench uses it for leading-if sentences authored backwards (Tezzeret,
  `Experimental/Cards.idr` ~592, ~601, ~2140), and a leading condition's own
  mentions are unreadable because `condDelta` is `[]` for every `Condition`.
  Ruling: **both orientations are core**, because they introduce mentions in
  different places and under forward authoring the introduction site is the
  binding structure. "Counter target spell if it's red" introduces the target
  in the effect and the condition reads it back — today's shape, renamed
  `OnlyIf : (e : Effect bs) -> (c : Condition (preIntro e)) -> …`. "If you
  control three artifacts, draw two instead" is a new leading
  `If : (c : Condition bs) -> (e : Effect (condIntro c)) -> Maybe (Effect bs)
  -> Effect bs`, with `condDelta` carrying what a condition introduces. The
  postposed form is NOT a macro over the leading one: the expansion would have
  to relocate the target noun into the condition and pronominalize the hole,
  and the only single-constructor alternative introduces the target above both
  clauses — the prenex lift the workbench exists to remove — or a binder with
  cataphora (full endophora), rejected: oracle text is strictly anaphoric, so
  a forward-only binder is true to the CNL and cataphora would be a miserable
  refactor in service of nothing the corpus writes. Macros name the
  English over the two; the bench's leading-if cards move to `If`.
- **`WhereLetter` / `WhereLetterStatic`** take the definition first where
  English postposes it ("…, where X is the number of …"); eight bench sites
  author backwards and `DefinedLetter` is the largest family. Same treatment:
  reading-order shape in core, or a macro that restores it — record which.

## Consumption boundary

`idris/src/Experimental.idr` (`Effect.If`, `Conditionally`, `condIntro`,
`condNegatable`, `badTrailingPostStateZone`), `idris/src/Experimental/Words.idr`
(the doubled `if` spelling), `idris/src/Experimental/Events.idr` for the
one-shot's context threading, evidence bench
`idris/src/Experimental/Cards.idr`. No Rust crate.

## Acceptance

- Both orientations type in both containers; Balance of Power benches, and
  Vraska's [−9] and Iymrith read their gaps correctly.
- `badTrailingPostStateZone` still refuses what it refuses today; finding 362's
  no-single-constructor result is respected, not worked around.
- Gadrak's counted "unless" composes without opening negation wider than the
  comparison it fixes.
- `idris/scripts/build` PASS, no witness lost, no pin silently passing.

Standard constraints apply.

## As-landed

Round 1 is the conditional-orientations half only. Every coordination section
above is untouched and still open.

**Core.** `Effect.OnlyIf e c oth` is today's postposed shape renamed, condition
at `preIntro e`. `Effect.If c e oth` is the new leading orientation, consequent
at `condIntro c`. Neither is a macro over the other and neither reads forward —
finding 362 respected, each constructor typing one argument in the other's
context. Both arms drop the condition's mentions on the way out (`effIntro`,
`annIntro`, `preIntro`, `replacedCtx`): a conditioned clause exports nothing.

`otherwiseCtx e = outcomesOnly (deedDelta e) ++ annIntro e` types the
`otherwise` arm of both orientations — the phrases the then-branch announced
and the quantity it wrote, never a referent the deed would have made.
`Words.outcomesOnly` is the new filter.

`condDelta` absorbed `condMint`/`condMintAll`; `condIntro c = condDelta c ++ bs`.
`CompareAmt` introduces the margin and the phrases its two amounts name; the
`Matches` self re-mention rows carry over; `Exists`, `Happened`, `GameIs`,
`NoHolder` and `NotCond` introduce nothing; `AndCond` sums via `condDeltaAll`.

`StaticEffect.OnlyWhile se c` is the postposed static twin of `Conditionally`,
condition at `staticIntro se`, same `marking`/`NotConditional`/`MarkingOk`
discipline. `Events.PlayWindow = WhileSearchingLibrary` lands as a new defaulted
`window` slot on `MayPlay` (a static functioning from the library, not a
`Timing` arm).

`WhereLetter`/`WhereLetterStatic` stay **binder-first** in core — the letter is a
name the ability defines once [CR#107.3] and the definition reads nothing the
body announces, so a reading-order core would be the cataphoric binder the
workbench rejects. The English order is restored by `Macros.whereLetter` /
`Macros.whereLetterStatic`; Krenko is rewritten through the macro.

**Macros added:** `ifThen`, `ifThenElse`, `onlyIf`, `onlyIfNot`, `onlyWhile`,
`onlyUnless`, `whereLetter`, `whereLetterStatic`,
`mayCastFromWhileSearching`. `monstrosity` re-authored leading — [CR#701.37a]
writes the condition first.

**Bench:** seven new witnesses — `balanceOfPower`,
`vraskaBetrayalsStingUltimate`, `iymrithGapDraw`, `dragonlordOjutaiHexproof`,
`causticBroncoLoss`, `gadrakCantAttack`, `panglacialWurmCast`. Two new pins in
`ProofsE` (`badOtherwiseReadsLeadingArm`, `badLeadingConditionAntecedent`); one
retired (below). `badTrailingPostStateZone` still refuses, unchanged in shape.

### Orientation, decided from printed oracle text

Every migrated site was read from the printed line (the `card` script), not from
a description. Printed leading "If …, …" → `If`; printed trailing "… if …" →
`OnlyIf`.

- **Moved to leading `If`:** builtToSmash, amassZombiesTwo (both clauses),
  unholyAnnex, tezzeretDrawTwo, zimoneDrawTwo, timelyReinforcements (both),
  survivalCache, celestialConvergence (both), secretsOfTheGoldenCity,
  jushiApprentice, cursedScroll, magusOfTheScroll, candlesOfLeng,
  approachOfTheSecondSun, answeredPrayers, primalSurge, zimoneAndDina.
- **Stay postposed (`OnlyIf`):** overload, flamesOfTheRazeBoar, disruptingShoal,
  savageSwipeLine, dreamThief, galvanicBlastLine.
- **Moved to `OnlyWhile`:** nimbleMongoose.

Two sites the round's design listed as moving stay postposed, on the text:
flamesOfTheRazeBoar ("… deals 2 damage to each other creature that player
controls **if** you control a creature with power 4 or greater") and dreamThief
("draw a card **if** you've cast another blue spell this turn").

### Ledger

- **Vraska.** The ticket's "Vraska's [−9]" is **Vraska, Betrayal's Sting**, not
  Vraska, Golgari Queen — the latter's [−9] is an emblem grant with no gap read.
  Benched as `vraskaBetrayalsStingUltimate`.
- **Iymrith** pays twice. Its draw line ("Then if you have fewer than three
  cards in hand, draw cards equal to the difference") benches as `iymrithGapDraw`.
  Its static, "Iymrith has ward {4} as long as it's untapped", is the
  subject-pronominalising `OnlyWhile` shape; it is benched at Dragonlord
  Ojutai's simpler spelling of the same sentence ("has hexproof as long as it's
  untapped") because ward-with-a-cost is a separate unbuilt row.
- **The counted "unless" needed no negation change.** The source has no
  `condNegatable`; `NotCond` is ungated and `markingOk Unless c = condNegated c`
  is satisfied by any `NotCond`. Gadrak composes today as
  `Macros.onlyUnless (Deontic … Forbid Attack …) (CompareAmt …)`. The finding-354
  premise recorded above is report drift, not a gap.
- **`badDoubleConditional` retired.** It refused a canonical-form preference —
  nesting re-spells a conjunction — and named no rule, so under
  `docs/memory/rulings/measurements-live-in-pins.md` it is not a pin; a nested
  conditional is rules-meaningful ("if A, then if B, …"). The postposed twin was
  not minted. The type-level gate (`nn : NotConditional se`, now on both
  `Conditionally` and `OnlyWhile`) is unchanged and still refuses nesting; only
  the assertion about it is gone, and no printed card is left unrepresentable
  (`AndCond` spells the conjunction). Whether that gate itself should go is a
  separate call, not made here.
- **The search window cites only what the CR says.** All of [CR#701.23a..701.23j]
  was read: no subrule states that a player may cast a spell while searching, and
  no rule elsewhere does either. `PlayWindow` therefore cites [CR#701.23a] (the
  search action) and [CR#113.6b] (an ability that states its zones functions only
  from them), and its docstring records the absence.
- **Tolerated over-generation, unpinned.** Under leading `If`, `annIntro e`
  carries `condIntro c`, so the `otherwise` arm can also read the condition's own
  mints. A target written inside the condition is legitimately in scope; a failed
  comparison's margin is meaningful-but-odd. Stripping it needs an
  announcement-delta the grammar lacks.
- **No chapter or finding numbers** were assigned this round; the decision record
  is the constructors' docstrings plus this section.
- **Static re-orientation is a later round.** desperateCastaways, bombur,
  brightspearZealot, deepwayNavigator, urborgScavengers, thunderstaff,
  martyrsOfKorlis are untouched. thunderstaff and martyrsOfKorlis are visibly
  fronted authorings of postposed print ("… as long as it's untapped").
- **Citation defect fixed in passing.** `badTrailingPostStateZone` cited
  [CR#109.2a] — the "card"-plus-zone-name description — for the battlefield
  default. Every other site in the tree cites [CR#109.2], which is the rule that
  makes the claim. Corrected.
- Bench size: witnesses 787 → 794; pin declarations 444 → 445.
