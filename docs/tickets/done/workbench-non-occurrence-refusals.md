---
needs: []
---
**Re-ground every grammar exclusion that was justified by corpus
non-occurrence.** The 2026-09-02 Zen comment sweep of the workbench grammar
modules found 25 deleted comment blocks that justified a closed arm, missing
slot, narrowed gate, or absent row by "no supported line writes this" /
"measured zero" rather than by a rule. A count is never a refusal
(`docs/memory/rulings/measurements-live-in-pins.md`): each site must
either cite the CR rule that makes the shape meaningless, or open the shape.
18 of the 25 named no rule at all.

Sites (module, construct, verdict of the sweep's audit — `no` = no rule
named, `unclear` = a rule was gestured at but not cited):

Effect.idr
- direction slot, "less" arm — no
- floor rider on mana-run reductions ("can't reduce … to less than") — no
- nested static conditionals (gate withholds width) — no
- two turn-part windows on one statement — unclear
- nested cost joins — no
- power "in addition to" another power — no
- repeat-memory exclusion beyond the process's own choice — no
- specified starting player other than the caster — no
- one-direction die-result shift slot — no
- object-scoped "chaos ensues" participant slot — unclear
- ability-on-the-stack counter recipient — no
- removing direction for own-kind counter arithmetic — no
- named anchor slot for "you get" additional parts — no
- traversal of effect bodies for door deixis — no
- joint cross-ability choice-typing container — no

Events.idr
- joined dealer-side damage complement — unclear

Phrase.idr
- stronger uniqueness gate for chosen-player reads — unclear
- general mana-symbol matcher — no
- player-specific `It` row — no
- distributive-agent group binding after the pass — no
- second-chooser devotion read — unclear
- non-battlefield (emblem) grantor widening — unclear

Words.idr
- `PaidCost` readback rows for the twenty alternative-cost keywords — no
- `PaidCost` readbacks for entwine/escalate; per-mode multiplier — unclear
- face-down trigger-header cell — no

Done when each site carries either a `[CR#…]` citation on the excluding arm
or an opened shape with a bench witness, and no comment in the seven grammar
modules argues from a corpus count.

## As landed

- Effect.idr — direction slot, “less” arm — opened (+ witness lessAsThoughCondition)
- Effect.idr — floor rider on mana-run reductions — opened (+ witness manaRunReductionFloor)
- Effect.idr — nested static conditionals — opened (+ witness nestedStaticConditionals)
- Effect.idr — two turn-part windows on one statement — opened (+ witness nestedTurnPartWindows)
- Effect.idr — nested cost joins — opened (+ witness nestedCompoundCost)
- Effect.idr — power “in addition to” another power — cited [CR#707.9d]
- Effect.idr — repeat-memory exclusion beyond the process’s own choice — opened (+ witness repeatWithIndependentException)
- Effect.idr — specified starting player other than the caster — opened (+ witness voteStartingWithSpecifiedPlayer)
- Effect.idr — one-direction die-result shift slot — opened (+ witness oneWayResultShift)
- Effect.idr — object-scoped “chaos ensues” participant slot — opened (+ witness objectScopedChaos)
- Effect.idr — ability-on-the-stack counter recipient — opened (+ witness abilityCounterRecipient)
- Effect.idr — removing direction for own-kind counter arithmetic — opened (+ witness removeOwnCounterKinds)
- Effect.idr — named anchor slot for “you get” additional parts — opened (+ witness namedAdditionalPartAnchor)
- Effect.idr — traversal of effect bodies for door deixis — opened (+ witness delayedDoorTraversal)
- Effect.idr — joint cross-ability choice-typing container — opened (+ witness jointCrossAbilityChoice)
- Events.idr — joined dealer-side damage complement — opened (+ witness joinedDealerDamageComplement)
- Phrase.idr — stronger uniqueness gate for chosen-player reads — opened (+ witness lastChosenPlayerRead)
- Phrase.idr — general mana-symbol matcher — opened (+ witness generalManaSymbolMatcher)
- Phrase.idr — player-specific `It` row — opened (+ witness playerItRead)
- Phrase.idr — distributive-agent group binding after the pass — opened (+ witness distributiveGroupSurvives)
- Phrase.idr — second-chooser devotion read — opened (+ witness secondChooserDevotionRead)
- Phrase.idr — non-battlefield emblem grantor widening — opened (+ witness emblemGrantorRead)
- Words.idr — `PaidCost` readbacks for twenty alternative-cost keywords — opened (+ witness alternativeCostReadbacks)
- Words.idr — `PaidCost` readbacks for entwine/escalate and per-mode multiplier — opened (+ witness modalCostReadbacks)
- Words.idr — face-down trigger-header cell — opened (+ witness turnedFaceDownHeader)
