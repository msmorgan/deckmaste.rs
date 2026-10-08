---
needs: []
---
**Retire the checker's cost and deontic refusals (15 constructors).**
Per the checker's charter (`docs/decisions/semantics-v2.md` §7, ruling
2026-10-06; the `lean/CONTRACTS.md` header). The checker admits a written term as a faithful reading of the
card's English: references resolve and terms are well-kinded. It is not a
rules engine, and an instruction the game cannot carry out is still a
faithful reading [CR#101.3,609.3]. Standard constraints apply.

## The bucket

| Constructor | `Refusal.lean` line | Pin lines | Candidate emission sites (`lean/Semantics/Check/`) |
|---|---|---|---|
| `modesCosted` [CR#702.172a] | 144 | 1 (1 file) | AbilityRules.lean:385 |
| `keywordCost` | 159 | 1 (1 file) | PhraseRules.lean:715 |
| `keywordCostPaidByYou` | 161 | 2 (2 files) | AbilityRules.lean:769 |
| `deonticBoundOk` | 186 | 1 (1 file) | AbilityRules.lean:676 |
| `deonticPatientOk` | 187 | 8 (3 files) | AbilityRules.lean:677 |
| `asThoughOk` | 188 | 2 (1 file) | AbilityRules.lean:678 |
| `deonticRiderOk` | 189 | 5 (1 file) | AbilityRules.lean:680 |
| `altPayment` | 197 | 3 (2 files) | AbilityRules.lean:491,650 |
| `addedPayment` | 198 | 2 (1 file) | AbilityRules.lean:651 |
| `manaRun` | 207 | 2 (2 files) | AbilityRules.lean:75,460; Card.lean:266 |
| `costAction` | 211 | 13 (3 files) | AbilityRules.lean:463 |
| `payable` | 213 | 5 (5 files) | Abilities.lean:1003,1004,1005,1013; AbilityRules.lean:340 |
| `payAgrees` | 214 | 7 (2 files) | AbilityRules.lean:340 |
| `costTapOnce` | 215 | 2 (1 file) | AbilityRules.lean:774 |
| `costPaidByYou` | 216 | 6 (2 files) | AbilityRules.lean:774 |

Pin lines in the bucket: 60.

The emission-site column is a grep for `.<name>` under `lean/Semantics/Check/`
(Refusal.lean excluded), 2026-10-06. Some matches are a same-named predicate
or another type's constructor (`NounPhrase.discardOk` is a predicate, not an
emission); confirm each before removing it. Pin lines count matching lines
under `lean/Semantics/Proofs/`.

## The work

- Remove each constructor's emission sites and the constructor itself, or
  keep a constructor with a one-line reason where this ticket allows it.
- Re-spell or retire each pin that asserted a retired refusal. A pin whose
  subject still exists is re-spelled against what the checker now says (the
  same term, its new outcome); a pin is removed only when the refusal was its
  whole subject, and the landing record names it.
- Update the facts table and `lean/CONTRACTS.md`: each section this sweep
  touches says what the checker no longer checks.

## Proof

- No reference-resolution or term-shape pin changes outcome.
- `cargo xtask lean check` over `plugins_v2/canon` and `plugins_v2/testing`
  stays all-pass, and `cargo xtask lean definitions` stays all-pass (or
  their regrouped names, `cargo xtask lean check` and `cargo xtask lean
  definitions`, once `semantics-v2-chooser-and-taker-fields` lands).
- `lean/scripts/build` passes.

## Did any retired pin encode a real card defect?

Mandatory. The landing fills this in, one line per retired or re-spelled pin:
**yes**, naming the card and the defect the refusal caught, or **no**. A yes
is reported to the owner with the landing record; it may argue for keeping
the refusal, which needs a dated ruling.

Every one of the 58 pins below is a hand-written sentence or a predicate fixture; none is a
printed card, and no canon or testing card failed a retired refusal before this landing (the
gate's baseline was canon 138/138, testing 8/8). The bucket table's 60 pin lines are these pins'
retired-refusal lines plus the 5 `deonticPatientOk` lines of the self-reference pins, which keep
their outcome (`Deontic.badBlocksItself` and the four `MacroParameters` concealment and identity
pins); the predicate fixtures (`Cost.payable`, `paidByYou`, `tapOnce`, `selfTapUses`,
`costActionOk`) are outside the grep and listed here too. No pin encoded a real card defect:
**0 yes**.

| Pin | File | Retired refusal(s) asserted | Fate | Real card defect? |
|---|---|---|---|---|
| `okCostAlternativeKeepsSymbolProperties` | Composition.lean | `Cost.payable`, `Cost.selfTapUses` (predicates) | re-spelled to its `Cost.offBattlefield` conjunct, which `classAbilityOk` still reads | no: a hand-written sentence, a twin of the rule |
| `badContinuousSpecAsCost` | Composition.lean | `costAction` | deleted | no: a hand-written sentence, a twin of the rule |
| `badDurationBearingDefinitionAsCost` | Composition.lean | `costAction` | deleted | no: a hand-written sentence, a twin of the rule |
| `badStateBasedCauseForLifeGain` | Deontic.lean | `deonticRiderOk` | deleted | no: a hand-written sentence, a twin of the rule |
| `badStateBasedCauseForMixedDeeds` | Deontic.lean | `deonticRiderOk` | deleted | no: a hand-written sentence, a twin of the rule |
| `badPayTapSymbol` | Deontic.lean | `payable` | deleted | no: a hand-written sentence, a twin of the rule |
| `badContinuousAsCost` | Deontic.lean | `costAction` | deleted | no: a hand-written sentence, a twin of the rule |
| `badInsteadAsCost` | Deontic.lean | `costAction` | deleted | no: a hand-written sentence, a twin of the rule |
| `badDelayedAsCost` | Deontic.lean | `costAction` | deleted | no: a hand-written sentence, a twin of the rule |
| `badHeldUntilAsCost` | Deontic.lean | `costAction` | deleted | no: a hand-written sentence, a twin of the rule |
| `badReflexiveAsCost` | Deontic.lean | `costAction` beside `reflexEnclosure` | re-spelled to `[.reflexEnclosure]` | no: a hand-written sentence, a twin of the rule |
| `badSkipAsCost` | Deontic.lean | `costAction` | deleted | no: a hand-written sentence, a twin of the rule |
| `badPayAsCost` | Deontic.lean | `costAction` | deleted | no: a hand-written sentence, a twin of the rule |
| `badSequentialCost` | Deontic.lean | `costAction` | deleted (sequenced costs, decided below) | no: a hand-written sentence, a twin of the rule |
| `badSimultaneousCost` | Deontic.lean | `costAction` | deleted | no: a hand-written sentence, a twin of the rule |
| `badRepeatAsCost` | Deontic.lean | `costAction` | deleted | no: a hand-written sentence, a twin of the rule |
| `badPlayFromBattlefield` | Deontic.lean | `deonticRiderOk` | deleted | no: a hand-written sentence, a twin of the rule |
| `badPlayFromStack` | Deontic.lean | `deonticRiderOk` | deleted | no: a hand-written sentence, a twin of the rule |
| `badCastALand` | Deontic.lean | `deonticPatientOk` (the counterpart's card types) | deleted | no: a hand-written sentence, a twin of the rule |
| `badPlayFromWrongZone` | Deontic.lean | `deonticRiderOk` | deleted | no: a hand-written sentence, a twin of the rule |
| `badForbidAttackWithPatient` | Deontic.lean | `deonticPatientOk` (the counterpart's card types) | deleted | no: a hand-written sentence, a twin of the rule |
| `badManaPremiseAtAttack` | Deontic.lean | `asThoughOk` | deleted | no: a hand-written sentence, a twin of the rule |
| `badCounterBoundTwice` | Deontic.lean | `deonticBoundOk` | deleted | no: a hand-written sentence, a twin of the rule |
| `badObjectPremiseAtSpend` | Deontic.lean | `asThoughOk` | deleted | no: a hand-written sentence, a twin of the rule |
| `badUntapCapGraveyardSet` | Zone.lean | `deonticPatientOk` (the counterpart's zone) | deleted | no: a hand-written sentence, a twin of the rule |
| `capturedCostsPreservePaymentClassifiers` | MacroParameters.lean | `Cost.paidByYou`, `tapOnce`, `payable` (predicates) | deleted | no: a hand-written sentence, a twin of the rule |
| `capturedYouRemainsThePayer` | MacroParameters.lean | `Cost.paidByYou` (predicate) | re-spelled: the captured "you" read back `isYou` | no: a hand-written sentence, a twin of the rule |
| `instructionScopesDoNotHideAnIncorrectPayer` | MacroParameters.lean | `Cost.paidByYou` (predicate) | re-spelled: the captured opponent read back is not `isYou` | no: a hand-written sentence, a twin of the rule |
| `scopedDefinitionRemainsAdmissibleAsCost` | MacroParameters.lean | `costActionOk` (predicate) | deleted | no: a hand-written sentence, a twin of the rule |
| `scopedDefinitionCannotConcealInvalidCostCapture` | MacroParameters.lean | `costActionOk` (predicate) | deleted | no: a hand-written sentence, a twin of the rule |
| `nestedCompoundCost` | Mana.lean | `costTapOnce` | deleted | no: a hand-written sentence, a twin of the rule |
| `badDoubleTapCost` | Mana.lean | `costTapOnce` | deleted | no: a hand-written sentence, a twin of the rule |
| `badEmptyManaCost` | Mana.lean | `manaRun` | re-spelled to `nonEmpty` (term shape kept) | no: a hand-written sentence, a twin of the rule |
| `badUnlessTapSymbol` | Mana.lean | `payable` | deleted | no: a hand-written sentence, a twin of the rule |
| `badPlayPaymentTapSymbol` | Mana.lean | `altPayment` | deleted | no: a hand-written sentence, a twin of the rule |
| `badAddedCostTapSymbol` | Mana.lean | `addedPayment` | deleted | no: a hand-written sentence, a twin of the rule |
| `badAddedCostLoyaltySymbol` | Mana.lean | `addedPayment` | deleted | no: a hand-written sentence, a twin of the rule |
| `badOpponentPaysYourCost` | Mana.lean | `keywordCostPaidByYou` | deleted | no: a hand-written sentence, a twin of the rule |
| `badSpreeMissingCost` | Choice.lean | `modesCosted` | deleted | no: a hand-written sentence, a twin of the rule |
| `badMismatchedPayer` | Choice.lean | `payAgrees` | deleted | no: a hand-written sentence, a twin of the rule |
| `badCostReadsSiblingDeed` | Keyword.lean | `costAction` (a stamped read inside a cost) | deleted (see DISCLOSE) | no: a hand-written sentence, a twin of the rule |
| `badForeignPayerCost` | Keyword.lean | `costPaidByYou` | deleted | no: a hand-written sentence, a twin of the rule |
| `badForeignSacrificeCost` | Keyword.lean | `costPaidByYou` | deleted | no: a hand-written sentence, a twin of the rule |
| `badPayCostlessKeyword` | Anaphora.lean | `keywordCost` | deleted | no: a hand-written sentence, a twin of the rule |
| `badPayLoyalty` | Faces.lean | `payable` | deleted | no: a hand-written sentence, a twin of the rule |
| `badAltCostTapSymbol` | Faces.lean | `altPayment` | deleted | no: a hand-written sentence, a twin of the rule |
| `badAltCostLoyaltySymbol` | Faces.lean | `altPayment` | deleted | no: a hand-written sentence, a twin of the rule |
| `badPrototypeWithoutAltCost` | Faces.lean | `manaRun` | re-spelled to `nonEmpty` (term shape kept) | no: a hand-written sentence, a twin of the rule |
| `badActorMismatchedPayer` | Actor.lean | `payAgrees` | deleted | no: a hand-written sentence, a twin of the rule |
| `badHandedOffLifeCost` | Actor.lean | `costPaidByYou` | re-spelled to `[]` plus the handed player's `isYouIn` (false) | no: a hand-written sentence, a twin of the rule |
| `foreignSacrificeCostHandoffTwin` | Actor.lean | `costPaidByYou` | re-spelled to `[]` | no: a hand-written sentence, a twin of the rule |
| `foreignPayerCostHandoffTwin` | Actor.lean | `costPaidByYou` | re-spelled to `[]` | no: a hand-written sentence, a twin of the rule |
| `opponentPaysYourCostHandoffTwin` | Actor.lean | `keywordCostPaidByYou` | re-spelled to `[]` | no: a hand-written sentence, a twin of the rule |
| `mismatchedPayerHandoffTwin` | Actor.lean | `payAgrees` | re-spelled to `[]` | no: a hand-written sentence, a twin of the rule |
| `badCapturedActorIsTheHandedPlayer` | Actor.lean | `payAgrees` (twice) | re-spelled to `[]` twice plus the inner handoff's `isYouIn` past the capture (true) | no: a hand-written sentence, a twin of the rule |
| `capturedActorIsTheHandedPlayer` | Actor.lean | `costPaidByYou` | re-spelled to `[]` plus the captured actor's `isYouIn` (false) | no: a hand-written sentence, a twin of the rule |
| `badNestedHandoffInnermost` | Actor.lean | `payAgrees` | re-spelled to `[]` plus `isYouIn` of the inner (true) and outer (false) handoffs | no: a hand-written sentence, a twin of the rule |
| `badGrantedAbilityKeepsItsOwnPerformer` | Actor.lean | `payAgrees` | re-spelled to `[]` plus `isYouIn` under `ownPerformerCtx` (true) | no: a hand-written sentence, a twin of the rule |

## Note

The refusal of sequenced costs (`costAction`, pin `badSequentialCost` in
`Proofs/Deontic.lean`) was recorded as staying on purpose in
`plugins-v2-keyword-body-defects` ("Sequenced costs stay refused", costs paid
in any order [CR#601.2h]). That decision predates the charter; this sweep
decides it again with a one-line reason.

## Landing record

Measured on the tip `kvumltlu` (the costs docs commit; its `lean/` tree is the pins commit
`pytrupos`'s) after `kata refresh` onto trunk `zkmuyqxw`, 2026-10-08. Both sweeps landed in one
claim (`checker-sweeps-timing-costs`) and were proved together. No coverage lock, grammar
construction or Rust crate is in play: the change touches `lean/` and `docs/` only.

Commits: `nnunsuqx` the emission sites, the dead predicates and the 14 `Refusal` constructors;
`pytrupos` the pins; `kvumltlu` CONTRACTS and this record.

### PROVE

- **No silent loss.** Every pin that asserted a retired refusal or read a retired predicate is
  named in the table above with its fate: 43 deleted (the refusal was its whole claim, retired
  by the §7 charter ruling of 2026-10-06, consequence (b)), 15 re-spelled. `cargo xtask lean check`:
  canon 138/138, testing 8/8; no card newly fails and none newly passes, since every card passed
  before. `cargo xtask lean definitions`: 553/553 (73 counters, 461 subtypes, 19 designations).
  `cargo xtask facts check`: up to date.
- **Structural laws.** `lean/scripts/build` (`lake build --wfail`, 88 jobs): clean at
  the tip; `lake build Semantics SemanticsCards --wfail` passed at the constructor commit, where
  the pin suites cannot yet pass, and the full build from the pins commit on. No reference-resolution or term-shape pin changed outcome:
  the five self-reference pins (`Deontic.badBlocksItself`,
  `MacroParameters.scopedCounterpartCannotConcealSelfReference`,
  `capturedCounterpartCannotConcealSelfReference`, `callerCounterpartCannotConcealSelfReference`,
  `capturedSubjectAndCounterpartStillShareIdentity`) still give `[.deonticPatientOk]`
  unchanged; `badReflexiveAsCost` keeps its `reflexEnclosure`; the empty-run pins keep a
  refusal, now the term-shape `nonEmpty`. The handoff pins whose subject was who the actor is
  (`Actor.lean`, the table above) keep that subject as a reference observable: `isYouIn` in
  the context the checker builds for the payment.
- **No word-naming.** No guard names a lexeme, construction or card.
- **Cites.** `cite check --list-noncompliant` empty; `cite check` 0 stale (16,352 citations);
  `cite audit --diff` from the claim read 5 sites (the five this landing adds in the two
  records), each against its claim. Citations that left with deleted docstrings and pins are not
  stale; the lock is unchanged.
- **Board.** `kata kanban check`: OK.
- **Expansions and lexicon export.** No file under `plugins_v2/`, `crates/` or the lexicon
  changed between the claim and the tip, so both are byte-identical by construction; neither
  was re-run.
- **Rust.** No crate's code or consumed data changed (there is no Rust mirror of `Refusal`; the
  facts generator names none of the 15 and emits none of the deleted `ActFacts` fields;
  `KeywordFacts.paidCost`, which it does emit, is still read by the paid-cost names in
  `Check/Keywords.lean`), so no `cargo test -p` or clippy run is owed.

### DISCLOSE

- **Constructors removed (14):** `modesCosted`, `keywordCost`, `keywordCostPaidByYou`,
  `deonticBoundOk`, `asThoughOk`, `deonticRiderOk`, `altPayment`, `addedPayment`, `manaRun`,
  `costAction`, `payable`, `payAgrees`, `costTapOnce`, `costPaidByYou`.
- **Constructor kept, narrowed (1): `deonticPatientOk`.** It carried a reference duty: a
  counterpart named after the subject is not the subject read back (`counterpartNotSelf`, by
  binding address), which five identity and concealment pins assert. It now checks only that,
  the counterpart's kind against the deed's other role (`deedKindOk`, the kind half of the old
  `deedFits` call), and that a complement names one of the rule's own deeds, no deed twice.
  Retired from it: which deeds `defendingPlayer` and `targetedBy` go with, and the
  counterpart's object class, card types and zone.
- **Emission sites removed (18 `refuse` calls, 3 of them re-routed):** `manaRun` at
  `CostShift.run`, `Cost.mana` and the prototype frame's alternative cost becomes
  `refuse (!c.isEmpty) .nonEmpty`; `Cost.perform` (`costAction`), `Instruction.pay`
  (`payable`, `payAgrees`), `chooseModes` (`modesCosted`), `PlayPayment.payingInstead` and
  `StaticSpec.altCost` (`altPayment`), `StaticSpec.addedCost` (`addedPayment`),
  `StaticSpec.deonticRule` (`deonticBoundOk`, `asThoughOk`, `deonticRiderOk`), `Ability.keyword`
  (`keywordCostPaidByYou`), `Ability.activated` (`costTapOnce`, `costPaidByYou`), and
  `GameEvent.paysCost` (`keywordCost`). The ticket's grep column is confirmed; the four
  `Abilities.lean` matches for `payable` were the `Cost.payable` definitions, not emissions.
- **Predicates deleted** (each fed only a retired refusal): in `Check/Abilities.lean`
  `Cost.selfTapUses`/`selfTapMax`/`selfTapCount`, `Cost.tapOnce`/`allTapOnce`,
  `NounPhrase.isActor`, `Instruction.paidByYouAs`, `Cost.paidByYouAs`/`allPaidByYouAs`,
  `Cost.paidByYou`, `keywordCostPaidByYou`, `payAgreesOk`, `payAgreesOkIn`,
  `Cost.payable`/`allPayable`, `StaticSpec.definitionCostOk`,
  `Instruction.costActionOk`/`costActionOkOpt`/`costModesOk`/`costStepsOk`,
  `Exchanged.costOk`, `CountBound.introduced`, `deonticBoundOk`, `Role.isAgent`,
  `counterpartFits` (replaced by `counterpartKindOk`), `DeonticRider.playRidden`,
  `asThoughOk`, `Compulsion.permits`, `StateBasedCause.deed`, `deonticRiderOk`; in
  `Check/Phrase.lean` `NounPhrase.costNounOk`/`allCostNounOk`, `CaptureInput.costNounOk`,
  `playSourceOk`; in `Check/Events.lean` `deedDefendsOk`, `deedTargetedOk`,
  `deedPremiseSort`, `deedPremiseOk`, `deedPlaysOk`, `deedBoundedOk`, `playWindowOk`,
  `playableFrom`, `Zone.placementDestOk`; in `Check/Words.lean` `manaRun`,
  `modesCostedUniformly`, and the hand-written `ActFacts` fields `counterfactual`, `plays` and
  `bounded` with their rows. Kept because another rule reads them: `Cost.offBattlefield`
  (`classAbilityOk`, the card-class rule), `keywordCosts` (the paid-cost names), `deedKindOk`,
  `distinctDeeds`, `counterpartNotSelf`.
- **Kind and term-shape checks kept.** The counterpart's kind, inside `deonticPatientOk`
  (above). An empty mana run, `nonEmpty`: no printed cost writes an empty run (`{0}` is one
  generic symbol), so an empty one is an ill-formed term, refused as `Cost.compound []` is.
- **Decided again: sequenced costs.** `costAction` goes whole, and with it the refusal of
  sequenced costs (`badSequentialCost`, recorded as staying in
  `plugins-v2-keyword-body-defects`): that costs are paid in any order [CR#601.2h] is a rule
  about payment, and "Discard a card, then sacrifice a creature" is a faithful reading of the
  printed words. No part of `costActionOk` enforced a reference or a kind: its instruction
  whitelist and its noun demands (no `and` noun, no stamped read) are rules about what can be a
  cost, and every noun and stamped read in a cost is still resolved by `Instruction.checkWith`.
  So `Keyword.badCostReadsSiblingDeed` ("Sacrifice a creature, Exile the sacrificed card")
  now passes: the stamped read resolves, threaded through `Cost.checkSeq` as before; only the
  cost rule refused it.
- **`keywordCostPaidByYou` and `costPaidByYou` (the actor-handoff landing's additions).** Both
  asserted a rule, that a cost is paid by its controller [CR#602.1a]; the reference they
  leaned on, who `actor` is in a handoff, lives in `actorView`/`isYouIn` and the context
  builders, which stay and which the re-spelled `Actor.lean` pins now read directly. No part
  was kept.
- **Deviations and additions.** The `Actor.lean` re-spellings add `isYouIn` conjuncts (listed
  in the table); `MacroParameters.capturedYouRemainsThePayer` and
  `instructionScopesDoNotHideAnIncorrectPayer` now read `MacroCapture.read … |>.isYou`, and the
  two private macros they used (`capturedPayment`, `capturedLifePayment`) are deleted as
  unused. Docstrings that cited a deleted pin (`Actor.lean` twins, `okHandedPayerIsTheTarget`,
  `GetsBothDeltas.badContinuousAsCostTwin`) say it was retired. No construction was added.
- **STOPs.** None.
- **Glossary.** No new term needed.

### REPORT

- Counts (Assurance): restored 0, re-spelled 15, ignored 0, added 0, removed 43 (each named
  above with the constructor it asserted; all retired by the §7 charter ruling, 2026-10-06,
  consequence (b)).
- Builds: the first build of the claim was warm (`lake build --wfail`, 88 jobs, under a
  second); `lake build Semantics SemanticsCards --wfail` took 154 to 169 seconds at the
  constructor commits, the full build 30 to 37 seconds at the pins commits. `cargo xtask lean
  check` took 99.1 seconds (its own timer; 377 seconds with a cold `cargo-xtask` compile), host
  load average 5.3 on 24 threads.
