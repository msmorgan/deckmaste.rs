---
needs: [lean-v3-authoring-impl-6-cards-dewarting]
---
# Retire keyword-regime checks, residual game-rule refusals, and dead checker fields

**Retire the `KeywordFacts` body/regime and `ClassGuard` apparatus, `CardFace.choices`
(`Refusal.jointChoices`), the remaining phrase-level non-anaphora game-rule refusals,
and dead `Check/` fields, while keeping `keywordParamFits` (`argumentSchemas`) and
the basic `Card.lean` frame laws (`cardLine`, `cardBox`, `cardCost`).** Standard
constraints apply.

## Scope

### 1. Retire `KeywordFacts` body/regime checking, `ClassGuard`, and `jointChoices`

- In `lean/Semantics/Check/FactTypes.lean`, `lean/Semantics/Check/Facts.lean`,
  `lean/Semantics/Check/CardFacts.lean`, `lean/Semantics/Check/Card.lean`,
  `lean/Semantics/Check/Phrase.lean`, and `lean/Semantics/Check/Refusal.lean`:
  - Delete `Refusal.keywordBodyFits`, `StackRegime`, `ComponentFacts`,
    `KeywordFacts.definition`, `KeywordFacts.regime`,
    `KeywordFacts.functionsOnStack`, `KeywordFacts.counterEligible`,
    `StaticSpec.regimeFits`, `bodyEventRegime`, `Ability.regimeFits`, and
    `KeywordFacts.bodyPartFits`.
  - Delete the `ClassGuard` apparatus (`Predicate.impliedClass`,
    `Condition.impliedClass`, `Action.classGuard`, `StaticSpec.classGuard`,
    `Ability.classGuard`).
  - Delete `CardFace.choices : List QualitySort := []` (`lean/Semantics/Card.lean:16`),
    `jointDRS`, `jointChoicesOk`, and `Refusal.jointChoices` (`Check/Card.lean:314,398`).
  - Keep `Refusal.keywordParamFits` (`argumentSchemas`) and the basic `Card.lean`
    frame laws (`cardLine`, `cardBox`, `cardCost`).

### 2. Retire remaining phrase-level non-anaphora game-rule refusals

- Delete `lean/Semantics/Check/DeonticPatients.lean` (`Refusal.deonticPatientOk`,
  `[CR#509.1a]` self-blocking check) and remove its import from `lean/Semantics.lean`
  and `Check/Phrase.lean`.
- Delete `lean/Semantics/Check/Deck.lean` (`Refusal.deckReadable` and
  `Refusal.deckComparable`) and remove its import.
- In `lean/Semantics/Check/Phrase.lean`, `lean/Semantics/Check/Events.lean`,
  `lean/Semantics/Check/LocalShape.lean`, `lean/Semantics/Check/Words.lean`, and
  `lean/Semantics/Check/Refusal.lean`:
  - Delete `Refusal.qualifiedLimit` (`[CR#602.5b]` check that activation limits
    wrap activated abilities).
  - Delete `Refusal.counterBatchOk` (`[CR#310.12b]` emptying counter batch check).
  - Delete `Refusal.selfDefinedOk` (`[CR#604.3]` CDA `.ptDefinition` subject check).
  - Delete `Refusal.manaSymbolOk` (`[CR#107.4e,107.4f]` hybrid/Phyrexian mana
    symbol halves check).
  - Delete `Refusal.colorBoundOk` and `Refusal.colorCountOk` (`[CR#105.2]` color
    count range check).
  - Delete `Refusal.rolesOk` (distinct party roles check).
  - Delete `Refusal.subtypeType` (`typeOtherThan` creature subtype check).
  - Delete `Refusal.twoParties` (2-player check on `Exchanged.lifeTotals`).
  - Delete `Refusal.kindAmountOk` (`.sameAs` counter source unit-amount check).
  - Delete `Refusal.verbBecomesOk` and `ActFacts.intransitive` (`Check/Words.lean:131`,
    `Check/Phrase.lean:2254`).

### 3. Remove dead fields & functions and fix `changeLife (.set _)` outcome publication

- In `lean/Semantics/Check/Words.lean` and `lean/Semantics/Check/FactTypes.lean`:
  - Delete the unread fields `ActorShape.ability`, `ActorShape.selfDefined`,
    `ActorShape.twoParties`, `Referent.targets`, `DeedFeature.sacrificing`,
    `DeedFeature.tapping`, and `DesignationFacts.effectful`.
  - Delete the unreferenced functions `featureLabel`, `deedLabel`, and
    `Stamp.feature` (`Check/Words.lean:232,234,1017`).
- **Fix `Action.changeLife (.set _)` DRS outcome publication (`[CR#119.5,701.10d]`):**
  - Under `[CR#119.5]`, setting a player's life total causes that player to gain or
    lose the necessary amount of life. Update `stepDeltaWith` / `Delta.outcome` in
    `Check/Events.lean` and `Check/Phrase.lean` so `Action.changeLife (.set _)`
    (`setLifeTotal` and `doubleLifeTotal`) publishes `[outcomeB .lifeGained,
    outcomeB .lifeLost]` to the DRS (matching `Exchanged.lifeTotals` at
    `Check/Phrase.lean:1188`).

### 4. Update `lean/Semantics/Proofs/` and `lean/CONTRACTS.md`

- Retire `lean/Semantics/Proofs/KeywordDefinitionRegimes.lean`, the artificial
  `KeywordFacts` row pins in `Proofs/KeywordByCardClass.lean`,
  `Proofs/GiftVariants.lean`, and `Proofs/LinkedChoiceReadback.lean`, and the
  negative refusal pins for the retired refusals across `Proofs/*.lean`.
- Update `lean/CONTRACTS.md` to reflect the retired checks and remove the stale
  passages on `ActFacts.permanentActor` for `adapt`/`monstrosity` (lines 301–305),
  `jointChoices` (line 538), and `KeywordFacts.definition` / `StackRegime` /
  `ComponentFacts` (lines 747–820).

## Verification

- `./lean/scripts/build` (`lake build --wfail`) and `cargo xtask lean check` pass
  with zero warnings and zero `sorry`s.
- `cargo xtask cite check --list-noncompliant` empty and `cargo xtask cite check`
  0 stale.
