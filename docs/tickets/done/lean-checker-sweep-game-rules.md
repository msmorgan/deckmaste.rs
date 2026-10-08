---
needs: []
---
**Retire the checker's other game-rule refusals (43 constructors).**
Per the checker's charter (`docs/decisions/semantics-v2.md` §7, ruling
2026-10-06; the `lean/CONTRACTS.md` header). The checker admits a written term as a faithful reading of the
card's English: references resolve and terms are well-kinded. It is not a
rules engine, and an instruction the game cannot carry out is still a
faithful reading [CR#101.3,609.3]. Standard constraints apply.

## The bucket

| Constructor | `Refusal.lean` line | Pin lines | Candidate emission sites (`lean/Semantics/Check/`) |
|---|---|---|---|
| `designationScope` | 33 | 4 (3 files) | AbilityRules.lean:203,208; PhraseRules.lean:319,522,524,688 |
| `designationChecked` | 34 | 1 (1 file) | AbilityRules.lean:205,208; PhraseRules.lean:522,525 |
| `facesFit` | 145 | 0 (0 files) | AbilityRules.lean:214 |
| `opponentsLibrary` [CR#701.29a] | 164 | 2 (2 files) | AbilityRules.lean:336; Words.lean:416 |
| `deedRides` | 165 | 0 (0 files) | AbilityRules.lean:197 |
| `enactAgentOk` | 169 | 3 (2 files) | AbilityRules.lean:334 |
| `nontarget` | 174 | 5 (4 files) | AbilityRules.lean:788; PhraseRules.lean:677,705,743; Rules.lean:33,91,94 |
| `headerNontarget` | 179 | 1 (1 file) | EventContext.lean:307; Triggers.lean:44 |
| `doorNamesHost` | 180 | 1 (1 file) | AbilityRules.lean:206 |
| `selfExchanged` | 183 | 4 (2 files) | AbilityRules.lean:64 |
| `grantSubject` | 195 | 5 (3 files) | AbilityRules.lean:655 |
| `grantable` | 196 | 3 (2 files) | Abilities.lean:617,619,623,670,690; AbilityRules.lean:604,655,737; Rules.lean:41,87 |
| `notCarvedOut` | 199 | 0 (0 files) | Abilities.lean:721,722; AbilityRules.lean:689 |
| `clauseStatic` | 201 | 2 (2 files) | AbilityRules.lean:286 |
| `becomesOk` | 202 | 21 (6 files) | AbilityRules.lean:602,686 |
| `damageOpUse` | 205 | 1 (1 file) | AbilityRules.lean:706 |
| `manaSymbolOk` [CR#107.4e,107.4f] | 209 | 3 (2 files) | PhraseRules.lean:39 |
| `twoParties` | 225 | 2 (1 file) | Abilities.lean:33; AbilityRules.lean:46 |
| `tokenTyped` | 229 | 2 (2 files) | AbilityRules.lean:583 |
| `tokenPtOk` | 230 | 1 (1 file) | AbilityRules.lean:583 |
| `subsFitLine` | 231 | 2 (2 files) | AbilityRules.lean:584 |
| `tokenAbilities` | 232 | 1 (1 file) | AbilityRules.lean:585 |
| `tokenCanonical` | 233 | 3 (2 files) | AbilityRules.lean:585,601 |
| `tokenQualsFit` | 234 | 0 (0 files) | AbilityRules.lean:586 |
| `emblemAbilities` | 245 | 3 (2 files) | AbilityRules.lean:293 |
| `tokenNamed` [CR#111.10] | 249 | 1 (1 file) | Rules.lean:55 |
| `definitionHolder` [CR#122.1] | 252 | 1 (1 file) | Rules.lean:111 |
| `definitionNamed` | 254 | 3 (1 file) | Rules.lean:112,114,116 |
| `definitionScoped` [CR#701.15b] | 257 | 1 (1 file) | Rules.lean:118 |
| `cardLine` | 259 | 5 (1 file) | Card.lean:189,279 |
| `cardText` | 260 | 11 (1 file) | Card.lean:180 |
| `modalFrame` | 261 | 2 (1 file) | Card.lean:180 |
| `chapterFrame` | 262 | 1 (1 file) | Card.lean:181 |
| `doorFrame` | 263 | 2 (2 files) | Card.lean:185 |
| `cardName` | 264 | 6 (2 files) | Card.lean:195; PhraseRules.lean:61,488,489; Words.lean:53 |
| `cardBox` | 265 | 20 (2 files) | Card.lean:196,210,254,269 |
| `cardCost` | 266 | 2 (1 file) | Card.lean:197,211,265 |
| `adventureInset` | 268 | 1 (1 file) | Card.lean:214,285; Facts.lean:367 |
| `flipHalf` | 269 | 1 (1 file) | Card.lean:288,289 |
| `levelRange` | 270 | 1 (1 file) | Card.lean:252 |
| `levelerFrame` | 271 | 1 (1 file) | Card.lean:253,292 |
| `bandsDisjoint` | 272 | 1 (1 file) | Card.lean:293 |
| `prototypeFrame` | 273 | 2 (1 file) | Card.lean:268,296 |

Pin lines in the bucket: 132.

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

Every one of the 64 pins below is a hand-written sentence or a predicate fixture; none is a
printed card, and no canon or testing card failed a retired refusal before this landing (the
gate's baseline was canon 138/138, testing 8/8). The pins that assert a kept constructor
(`designationScope`: `Anaphora`, `Static.badGoadedPlayer`, `Faces.badPossessedMonarch`;
`manaSymbolOk`: `Mana`, `CharacteristicEdits`; `twoParties`: `Trigger`, the two
`MacroParameters` coordination fixtures; the registry and table shape pins in `Rules`) keep their
outcome and are not listed. No pin encoded a real card defect: **0 yes**.

| Pin | File | Retired refusal(s) asserted | Fate | Real card defect? |
|---|---|---|---|---|
| `badEmblemWithSpellAbility` | ActionFamilies.lean | `emblemAbilities` | deleted | no: a hand-written sentence, a twin of the rule |
| `copyCannotRemoveName` | CharacteristicEdits.lean | `becomesOk` | deleted | no: a hand-written sentence, a twin of the rule |
| `copyCannotAddBasePower` | CharacteristicEdits.lean | `becomesOk` | deleted | no: a hand-written sentence, a twin of the rule |
| `emptyRemovalIsRefused` | CharacteristicEdits.lean | `becomesOk` | re-spelled to `[.lineNonEmpty]` (term shape kept) | no: a hand-written sentence, a twin of the rule |
| `absentEditIsRefused` | CharacteristicEdits.lean | `becomesOk` | re-spelled to `[.nonEmpty]` (term shape kept) | no: a hand-written sentence, a twin of the rule |
| `copyCannotAddEmptyColor` | CharacteristicEdits.lean | `becomesOk` | re-spelled to `[.nonEmpty]` (term shape kept) | no: a hand-written sentence, a twin of the rule |
| `badStaticPlayerCantTargets` | Choice.lean | `nontarget` | deleted | no: a hand-written sentence, a twin of the rule |
| `badChosenBasicTypeOnCreature` | Choice.lean | `becomesOk` | deleted | no: a hand-written sentence, a twin of the rule |
| `badCreaturesAreMountains` | Choice.lean | `becomesOk` | deleted | no: a hand-written sentence, a twin of the rule |
| `badAddsNoColor` | Choice.lean | `becomesOk` | re-spelled to `[.nonEmpty]` (term shape kept) | no: a hand-written sentence, a twin of the rule |
| `badLosesNoColor` | Choice.lean | `becomesOk` | re-spelled to `[.nonEmpty]` (term shape kept) | no: a hand-written sentence, a twin of the rule |
| `badLosesPt` | Choice.lean | `becomesOk` | deleted | no: a hand-written sentence, a twin of the rule |
| `badStillOnAddition` | Choice.lean | `becomesOk` | deleted | no: a hand-written sentence, a twin of the rule |
| `badTokenDuplicateType` | Counters.lean | `tokenCanonical` | re-spelled to `[.distinct]` (term shape kept) | no: a hand-written sentence, a twin of the rule |
| `badTokenDuplicateColor` | Counters.lean | `tokenCanonical` | re-spelled to `[.distinct]` (term shape kept) | no: a hand-written sentence, a twin of the rule |
| `badBecomesZombieLand` | Counters.lean | `becomesOk` | deleted | no: a hand-written sentence, a twin of the rule |
| `badBecomesNothing` | Counters.lean | `becomesOk` | re-spelled to `[.lineNonEmpty]` (term shape kept) | no: a hand-written sentence, a twin of the rule |
| `badBecomesOwnType` | Counters.lean | `becomesOk` | deleted | no: a hand-written sentence, a twin of the rule |
| `badShieldNextTimeOnly` | Damage.lean | `damageOpUse` | deleted | no: a hand-written sentence, a twin of the rule |
| `badZombieArtifactToken` | Damage.lean | `subsFitLine` | deleted | no: a hand-written sentence, a twin of the rule |
| `badCreatureTokenNoPt` | Damage.lean | `tokenPtOk` | deleted | no: a hand-written sentence, a twin of the rule |
| `badTypelessToken` | Damage.lean | `tokenTyped` | deleted | no: a hand-written sentence, a twin of the rule |
| `badTargetedStateBasedLossGate` | Deontic.lean | `nontarget` | deleted | no: a hand-written sentence, a twin of the rule |
| `badStaticTargets` | Deontic.lean | `nontarget` | deleted | no: a hand-written sentence, a twin of the rule |
| `badBattlefieldFlash` | Faces.lean | `grantSubject` | deleted | no: a hand-written sentence, a twin of the rule |
| `badSpellIndestructible` | Faces.lean | `grantSubject` | deleted | no: a hand-written sentence, a twin of the rule |
| `badUnlockThisDoor` | Faces.lean | `doorNamesHost` | deleted | no: a hand-written sentence, a twin of the rule |
| `badGainsSpellAbility` | Keyword.lean | `grantable` | deleted | no: a hand-written sentence, a twin of the rule |
| `badBattlefieldConvoke` | Keyword.lean | `grantSubject` | deleted | no: a hand-written sentence, a twin of the rule |
| `badTokenSpellAbility` | Keyword.lean | `tokenAbilities` | deleted | no: a hand-written sentence, a twin of the rule |
| `badQuotedGrantOnSpell` | Keyword.lean | `grantSubject` | deleted | no: a hand-written sentence, a twin of the rule |
| `badFatesealYourOwnLibrary` | Keyword.lean | `opponentsLibrary` | deleted | no: a hand-written sentence, a twin of the rule |
| `badAltCostClause` | Mana.lean | `clauseStatic` | deleted | no: a hand-written sentence, a twin of the rule |
| `badWarpGrantInGraveyard` | Mana.lean | `grantSubject` | deleted | no: a hand-written sentence, a twin of the rule |
| `badNamedAddition` | Mana.lean | `becomesOk` | deleted | no: a hand-written sentence, a twin of the rule |
| `badRepeatedAdditionColor` | Mana.lean | `becomesOk` | re-spelled to `[.distinct]` (term shape kept) | no: a hand-written sentence, a twin of the rule |
| `badRepeatedAdditionType` | Mana.lean | `becomesOk` | re-spelled to `[.distinct]` (term shape kept) | no: a hand-written sentence, a twin of the rule |
| `badSbaThatTargets` | Rules.lean | `nontarget` | deleted | no: a hand-written sentence, a twin of the rule |
| `badConferralOfSpellAbility` | Rules.lean | `grantable` | deleted | no: a hand-written sentence, a twin of the rule |
| `badSubtypeConfersASpellAbility` | Rules.lean | `grantable` | deleted | no: a hand-written sentence, a twin of the rule |
| `badUntypedPredefinedToken` | Rules.lean | `tokenTyped`, `subsFitLine` | deleted | no: a hand-written sentence, a twin of the rule |
| `badTargetedOutcomeGate` | Static.lean | `nontarget` | deleted | no: a hand-written sentence, a twin of the rule |
| `badPtDefinitionClause` | Static.lean | `clauseStatic` | deleted | no: a hand-written sentence, a twin of the rule |
| `badGainsCommander` | Static.lean | `designationChecked` beside `designationScope`, `designationHolder` | re-spelled: the scope and holder refusals kept | no: a hand-written sentence, a twin of the rule |
| `badStillOnSubtypeSet` | Static.lean | `becomesOk` | deleted | no: a hand-written sentence, a twin of the rule |
| `badStillAnInstant` | Static.lean | `becomesOk` | deleted | no: a hand-written sentence, a twin of the rule |
| `everyDayNightLabelIsAnEffectfulDesignation` | Tables.lean | `DesignationLabel.checked` (predicate) | re-spelled: reads the facts row's `effectful` | no: a hand-written sentence, a twin of the rule |
| `aRuleOnlyDesignationIsNotEffectful` | Tables.lean | `DesignationLabel.checked` (predicate) | re-spelled: reads the facts row's `effectful` | no: a hand-written sentence, a twin of the rule |
| `anUndeclaredDayNightLabelIsNotADesignation` | Tables.lean | `DesignationLabel.checked` (predicate) | re-spelled: `DesignationLabel.known` is false | no: a hand-written sentence, a twin of the rule |
| `badTargetedDeathHeader` | Trigger.lean | `headerNontarget` | deleted | no: a hand-written sentence, a twin of the rule |
| `badKeywordEmblem` | Turn.lean | `emblemAbilities` | deleted | no: a hand-written sentence, a twin of the rule |
| `badEmptyEmblem` | Turn.lean | `emblemAbilities` | re-spelled to `[.nonEmpty]` (term shape kept) | no: a hand-written sentence, a twin of the rule |
| `badTokenDuplicateSupertype` | Turn.lean | `tokenCanonical` | re-spelled to `[.distinct]` (term shape kept) | no: a hand-written sentence, a twin of the rule |
| `badAgentedAgentlessAct` | Zone.lean | `enactAgentOk` beside three `kindMismatch` | re-spelled: the three kind refusals kept | no: a hand-written sentence, a twin of the rule |
| `badEveryCreatureTypeOnLand` | Zone.lean | `becomesOk` | deleted | no: a hand-written sentence, a twin of the rule |
| `badEveryBasicLandTypeOnCreature` | Zone.lean | `becomesOk` | deleted | no: a hand-written sentence, a twin of the rule |
| `badExchangeValueWithItself` | Zone.lean | `selfExchanged` | deleted | no: a hand-written sentence, a twin of the rule |
| `badExploreHandedToOpponent` | Actor.lean | `enactAgentOk` beside kind, possessor and anaphor refusals | re-spelled: those refusals kept | no: a hand-written sentence, a twin of the rule |
| `badAdaptHandedToOpponent` | Actor.lean | `enactAgentOk` beside `kindMismatch` | re-spelled: the kind refusal kept | no: a hand-written sentence, a twin of the rule |
| `scopedLookRetainsOpponentRequirement` | MacroParameters.lean | `opponentsLibrary` | deleted | no: a hand-written sentence, a twin of the rule |
| `scopedAmountCannotConcealSelfExchange` | MacroParameters.lean | `selfExchanged` | deleted | no: a hand-written sentence, a twin of the rule |
| `capturedAmountSubjectCannotConcealSelfExchange` | MacroParameters.lean | `selfExchanged` | deleted | no: a hand-written sentence, a twin of the rule |
| `sameSelectedSubjectCannotConcealSelfExchange` | MacroParameters.lean | `selfExchanged` | re-spelled to `[]` as `sameSelectedSubjectIsReadBackThroughTheCapture`: "it" resolves to the captured target | no: a hand-written sentence, a twin of the rule |
| `scopedLibrarySliceRetainsItsOwner` | MacroParameters.lean | `enactLibraryOwnerOk` (predicate) | deleted | no: a hand-written sentence, a twin of the rule |

## Decided at sweep time

- **The card-frame rules** (`cardLine` through `prototypeFrame`) are retired
  only at the owner's call, put to the owner when this sweep is worked: they
  are not instructions and do not move with the model (§7 ruling 2026-10-06,
  clause (d)).
- **The registry-definition refusals** (`definitionHolder`,
  `definitionNamed`, `definitionScoped`, `tokenNamed`) are shape checks on
  declarations rather than game rules, and probably stay. The sweeping agent
  decides each with a one-line reason.
- **Borderline constructors**, decided by the sweeping agent with a one-line
  reason each: `designationScope` and `designationChecked` (this bucket or
  zone), `notInstead` and `heldClause` (timing or this bucket, decided by
  `lean-checker-sweep-timing`), `manaSymbolOk`, `twoParties` and `nontarget`
  (this bucket or term shape). `selfDefinedOk`, `counterMemory`,
  `jointChoices`, `linkSource` and `readAmount` were borderline between
  reference and term shape; they are reference-side and stay.

## Landing record

Measured on the tip `tquwunwx` (this docs commit; its `lean/` tree is the pins commit
`qurwntxz`'s) after `kata refresh` onto trunk `sololsqo`, 2026-10-08. No coverage lock, grammar
construction or Rust crate is in play: the change touches `lean/` and `docs/` only.

Commits: `mzlrtupu` the emission sites, the dead predicates and the 21 `Refusal` constructors;
`qurwntxz` the pins; `tquwunwx` CONTRACTS, the charter's dated note and this record.

### PROVE

- **No silent loss.** Every pin that asserted a retired refusal or read a retired predicate is
  named in the table above with its fate: 44 deleted (the refusal was its whole claim, retired
  by the §7 charter ruling of 2026-10-06, consequence (b)), 20 re-spelled. `cargo xtask lean
  check`: canon 138/138, testing 8/8; no card newly fails and none newly passes, since every
  card passed before. `cargo xtask lean definitions`: 553/553 (73 counters, 461 subtypes,
  19 designations); the four registry and table shape checks still run. `cargo xtask facts
  check`: up to date.
- **Structural laws.** `lean/scripts/build` (`lake build --wfail`, 88 jobs): clean at the tip;
  `lake build Semantics SemanticsCards --wfail` passed at the constructor commit, where the pin
  suites cannot yet pass, and the full build from the pins commit on. No reference-resolution,
  kind or term-shape pin changed outcome: the pins of the eight kept constructors pass
  unchanged; the four pins that asserted a retired refusal beside a kind, possessor, anaphor,
  scope or holder refusal (`badGainsCommander`, `badAgentedAgentlessAct`,
  `badExploreHandedToOpponent`, `badAdaptHandedToOpponent`) keep exactly those; the term shape
  inside a retired refusal survives under an existing constructor (`nonEmpty`,
  `lineNonEmpty`, `distinct`, `qualityRead`), and the twelve pins that asserted it are
  re-spelled to it. The `enact` performer is still checked at its row's agent kind
  (`OptNoun.check` at `enactAgentKind`), and its reference half, the source permanent standing
  in for the controller (`enactDefaultsToThis`, `enactCtx`), is untouched.
- **No word-naming.** No guard names a lexeme, construction or card.
- **Cites.** `cite check --list-noncompliant` empty; `cite check` 0 stale (16,328 citations);
  `cite audit --diff` from the claim read 15 sites (the record's, CONTRACTS' and the reworded
  `Check/Rules.lean` docstrings'), each against its claim. Citations that left with deleted
  docstrings and pins are not stale; the lock is unchanged.
- **Board.** `kata kanban check`: OK.
- **Expansions and lexicon export.** No file under `plugins_v2/`, `crates/` or the lexicon
  changed between the claim and the tip in this landing's commits, so both are byte-identical
  by construction; neither was re-run.
- **Rust.** No crate's code or consumed data changed (there is no Rust mirror of `Refusal`; the
  facts generator names none of the 21 and emits none of the deleted hand-written `ActFacts`
  fields `rides` and `opponentsLibrary`; it still emits `DesignationFacts.effectful` and
  `KeywordFacts.functionsOnStack`, which no checker rule reads any more, and `Proofs/Tables`
  reads `effectful`), so no `cargo test -p` or clippy run is owed.

### DISCLOSE

- **Constructors removed (21):** `designationChecked`, `opponentsLibrary`, `deedRides`,
  `enactAgentOk`, `nontarget`, `headerNontarget`, `doorNamesHost`, `selfExchanged`,
  `grantSubject`, `grantable`, `notCarvedOut`, `clauseStatic`, `becomesOk`, `damageOpUse`,
  `tokenTyped`, `tokenPtOk`, `subsFitLine`, `tokenAbilities`, `tokenCanonical`,
  `tokenQualsFit`, `emblemAbilities`.
- **Constructors kept (8), one line each.**
  - `designationScope`: a kind check. Each site demands that the designation's declared
    holder sort (a player, an object, its card, the game) is the one the construction gives it
    to or reads it from; "your monarch" read as an object (`Faces.badPossessedMonarch`) is
    ill-kinded, not a broken rule. It carries no reference duty beyond that, and no rule about
    what the game does.
  - `facesFit`: term shape. A pile split's face list is empty or has one entry per pile, an
    arity agreement inside one term.
  - `manaSymbolOk` [CR#107.4e,107.4f]: term shape. A hybrid or Phyrexian symbol whose halves are
    the same is not a symbol, as a list naming a value twice is refused `distinct`.
  - `twoParties` [CR#701.12a]: plurality, which the charter names among the references. An
    exchange of life totals is between exactly two parties; a party phrase denoting one or many
    is a plurality error.
  - `tokenNamed` [CR#111.10], `definitionHolder` [CR#122.1], `definitionNamed`,
    `definitionScoped` [CR#701.15b]: shape checks on registry and table declarations (a nameless
    entry is unreachable, a holder outside object or player has no kind, a scope-narrowing field
    on a non-object designation describes nothing). They stay as the ticket expected.
- **Decided per the brief.**
  - `designationChecked`: retired. That a rule, not an instruction, confers the commander
    designation is a game rule; no read of the designation fails to resolve.
  - `enactAgentOk`: retired whole. Every pin that asserted it also asserted `kindMismatch` at
    the same performer, so its kind content was already checked by `OptNoun.check` at
    `enactAgentKind`; what it added is that a deed whose row names no performer may not be
    handed one, a game rule. The actor reference it read through (`actorView`) and the default
    to the source permanent (`enactDefaultsToThis`) stay and are used by `enactCtx` and
    `enactAgentKind`.
  - `nontarget` (a static ability, an event's subject, a rules row) and `headerNontarget`:
    retired. When targets are chosen [CR#601.2c] is a game rule; a targeted phrase in those
    places is well-kinded and its binding resolves.
- **Term shapes kept under existing constructors.** `becomesOk` and `tokenCanonical` held term
  shape beside their game rules; it is now `CharacteristicEdit.shape`: an empty edit list and an
  added or removed empty color list are `nonEmpty`, a type-line write that writes nothing or adds
  or removes no value is `lineNonEmpty`, a type, supertype or color list naming a value twice is
  `distinct` (also for a written token), and an edit's chosen quality must be a chosen read
  (`qualityRead`, as a token quality already is). `emblemAbilities`' empty emblem is `nonEmpty`.
  Retired with them: subtype hosting [CR#205.3d], a type already held, "still a" retention, a
  name or base stat added or removed, granted spell abilities [CR#113.3a], where a grant's
  subject is, a creature token's power and toughness, a token's card type, and an emblem's
  ability kinds.
- **Emission sites removed:** `Exchanged.values` (`selfExchanged`), `doAndForbid`
  (`deedRides`), `gainDesignation` and `setGameDesignation` and the `gameIs`/`noHolder`
  conditions (`designationChecked`; their `designationScope` stays), `unlock`
  (`doorNamesHost`), `establish` (`clauseStatic`), `createObject` (`emblemAbilities`),
  `enact` (`enactAgentOk`, `opponentsLibrary`), `TokenSpec.written` (five token refusals),
  `CopyExcept.edits`/`.ability`, `abilityGrant`, `characteristicChange`, `retention`,
  `damageRule`, `AbilityLost.written`, `Ability.static`, `SbaRule`, `ConferralRule`,
  `Conferral` (four), the `casts`/`activates`/`triggers` events (`nontarget`) and the trigger
  header (`headerNontarget`; `headerEventCheck` was left as plain `GameEvent.check` and is
  inlined). The ticket's grep column is confirmed; `twoParties` at `Abilities.lean` and
  `subsFitLine`, `facesFit` at `Words.lean` and `Card.lean` were the predicates, not emissions.
- **Predicates deleted** (each fed only a retired refusal): in `Check/Abilities.lean`
  `Amount.scopeBody`, `Amount.exchangedSubject`, `selfExchanged`, `TokenQuality.hosted`,
  `PreventCut.shieldUseOk`, `Instruction.lookedLibraryOwner`, `enactLibraryOwnerOk`,
  `enactLibraryOwnerOkIn`, `enactAgentOk`, `ptWritten`, `Predicate.regime`/`regimeAll`,
  `NounPhrase.regime`, `regimeMatches`, `Characteristics.typed`/`ptOk`/`abilitiesOk`,
  `Ability.grantable`, `CharacteristicBundle.qualsFit`, `TypeLineChanges.fits`,
  `CharacteristicEdit.fits`/`fitsAll`/`copyFits`/`canonical`/`writtenTypes`,
  `StaticSpec.notCarvedOut`, `DamageOp.useOk`, `Ability.emblemOk`, `emblemAbilitiesOk`,
  `Ability.regime`, `Ability.functionsOnStack`, `grantSubjectFits`, `grantSubjectOk`,
  `StaticSpec.clauseOk`/`partsClauseOk`; in `Check/Words.lean` `actOpponentsLibrary`,
  `anyTargetedAt`, `DesignationLabel.checked`, `spaceHosted`, `addedFits`, `anyNewType`,
  `addsSomething`, `CardType.retainable`, `retentionOk`, and the hand-written `ActFacts` fields
  `rides` (four rows) and `opponentsLibrary` (one row); `deedRidesOk` (`Check/Events.lean`);
  `Door.namesHost`, `GameEvent.headerNontarget` (`Check/EventContext.lean`);
  `keywordFunctionsOnStack` (`Check/Keywords.lean`). Kept because another rule reads them:
  `twoPartiesOk`, `facesFit`, `subsFitLine` (the card frame), `Characteristics.canonical`,
  `TypeLineChanges.canonical`, `colorOpOk`, `NounPhrase.opponentOnly`, `NounPhrase.targeted`,
  `keywordStackRegime`, `enactDefaultsToThis`.
- **Deviations and additions.** `CharacteristicEdit.writesLine`, `.shape` and `.shapeAll` are
  new (they carry the kept term shapes; no new `Refusal` constructor). `sameSelectedSubject`'s
  pin is renamed `sameSelectedSubjectIsReadBackThroughTheCapture` with its new claim. The three
  `Proofs/Tables` fixtures read the facts row directly. `Rules.lean`'s docstrings that stated
  the retired rules now say they are not asserted. No construction was added.
- **STOPs.** None.
- **Glossary.** No new term needed.

### Card-frame rules: kept pending the owner's call (Q42)

The fourteen card-frame refusals (`cardLine`, `cardText`, `modalFrame`, `chapterFrame`,
`doorFrame`, `cardName`, `cardBox`, `cardCost`, `adventureInset`, `flipHalf`, `levelRange`,
`levelerFrame`, `bandsDisjoint`, `prototypeFrame`) are untouched. The owner was asked (Q42)
whether to keep or retire them; no answer had arrived when the other 29 were done and proved, so
they stay under the ruling's default, "may survive the sweep at the owner's call" (§7 ruling
2026-10-06, consequence (d)), recorded there with a dated note. Their pins are unchanged.

### REPORT

- Counts (Assurance): restored 0, re-spelled 20, ignored 0, added 0, removed 44 (each named
  above with the constructor it asserted; all retired by the §7 charter ruling, 2026-10-06,
  consequence (b)).
- Builds: the first build of the claim was warm (`lake build --wfail`, 88 jobs, 0.16 seconds);
  `lake build Semantics SemanticsCards --wfail` took 166 seconds at the constructor commit, the
  full build 188 seconds on the first pins pass and 35 seconds once the pins were fixed.
  `cargo xtask lean check` took 101 seconds (its own timer, rounded; 397 seconds with a cold
  `cargo-xtask` compile after the refresh), host load average 4.85 on 24 threads.
