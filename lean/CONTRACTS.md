# Checker contracts

The checker receives the information that indexed syntax formerly carried in
its types. This document records where those duties live. A passing check is
an admission of the written semantic term, not a proof of lowering or execution.
The remaining representation boundaries are named below rather than hidden by
the existence of a checking function.

## Card authoring and trusted expansion

`spelled <| card` is the checked authoring boundary. It retains an
`Authoring.Form` tree before unfolding erases named macro calls, and constructs
`Spelled` with both `onlyMacros` and the existing `ok : card.check = []` proof.
The constructor is private. Record construction and record update cannot replace
these checks. Unregistered helper definitions and nested macro arguments are
inspected; opaque or variable semantic content without authoring evidence is
refused, including expressions nested in printed-data records.

Semantic expressions must use registered macro declarations. Printed-data
records, enumerated leaf values, and numeric literals remain ordinary data;
their semantic fields are inspected recursively. `Semantics.Macros.Primitives`
provides named wrappers with definitionally identical constructor signatures.
`Instruction.enact`, `NounPhrase.pro`, and `Window` have no primitive wrapper:
they are expansion carriers owned by the trusted macro layer. Existing idiomatic
macros are registered there too. Defaults retain their macro calls.

The kernel proves that the retained authoring tree contains no raw semantic
constructor and that the resulting card passes the checker. Correspondence
between the input expression and that tree is the elaborator's responsibility;
this is not a kernel proof about Lean source text. Registration establishes a
trusted macro definition. The checker validates its expanded body and contextual
use, but does not prove that the macro implements its label's rules meaning.
Extending the macro library therefore requires reviewing the expansion itself.
Raw expanded terms remain available to checker diagnostics and proof suites.

The tree retains macro names, data constructors and literals as a future
attachment point for spellings. The separate English project has no dependency
on this mechanism. No rendering or execution equivalence is claimed.

The anaphora bench's `manifestPlacement` is explicitly a placement-only fragment
of manifest [CR#701.40a]. It retains that bench's prior semantic value; it does
not model face-down characteristics or the turn-up special action.

## Scheduling fields

For `skipUntap`, `skipPart`, `addTurn`, and `addPart`, let `bs`
be the input context. The subject is checked in `bs`. Its amount is checked in
`nomIntro bs subject`; an absent optional subject leaves `bs` unchanged through
`optAgentIntro`. Part, anchor, and following-part fields carry no references.
No scheduling field introduces an enclosed scope. The output is
`Amount.intro afterSubject amount`; `addTurn` additionally puts `turnRefB`
on the announced/output context, after checking the amount. Its pre-context
has no turn reference. These inputs apply equally to checks and profiles.

Use `Amount.intro` to compose an amount's internal fields. Concatenating its
`introduced` list with an outer stack is not an interchangeable operation:
arithmetic traverses its operands in order and puts the latest mention first.
`Proofs/Turn` pins exact output bindings, forward-reference refusal, reads
through unrelated outer bindings, and mentions created inside the amount.

## Enclosed traversals

These are explicit boundaries in the surrounding checker, not exceptions to
be inferred from a failed pronoun:

| Construction | Context rule |
|---|---|
| Sequential instructions, compound costs, coordinated statics | Thread each member's output into the next member. |
| Simultaneous instructions | Thread announcements; sibling events do not read each other's outcomes. |
| Predicate sibling modifiers, condition siblings | Check in the same containing context; do not make conjunction sequential. |
| Alternatives, modes and result-table rows | Branch from their enclosing context; successful branch-local mentions are not unconditionally exported. A mode's cost precedes its own body. |
| `offer`, `doIfDone` | The success arm sees the body output; the failure arm starts before the body. |
| `doIf`, `doOnlyIf` | The condition/body order and `otherwiseCtx` are explicit; conditional execution exports no unconditional event outcome. |
| `doForEach`, `doForEachKind`, `repeatTimes` | Check the body in the element/value/count context; verify preservation of outer bindings and pluralize exported local mentions. |
| `enact` | The subject establishes `agentCtx`; distributive execution checks `enactKeepsOuter`. Tag/body trust is established at the macro authoring boundary above. |
| Delayed, reflexive and `triggerThisWay` clauses | Use `delayedCtx`, `reflexCtx` or `thisWayCtx`. The enclosed body's private mentions do not escape as ordinary sequential mentions. |
| Replacement and held clauses | Use `replacedCtx` or the held body's announcements. Do not export the event as already completed. |
| Event alternatives and joined headers | Check arms from the common input; `sharedCtx`/`joinedCtx` compute what the combined header can expose. |
| Activated abilities | Check the cost after dropping its local X; the body sees only public cost mentions. Timing, guard and activator clauses use the ability's enclosing context. |
| Granted/keyword/token/emblem abilities | A nested ability has its own scope. Keyword bodies, token characteristic text and emblem abilities use the empty context; grant checks use the grant's enclosing context. |
| Card text versus nested lists | `Ability.checkText` threads line-level choices; `Ability.checkAll` checks sibling abilities in one supplied context. |
| Printed boxes | Literal-only; they do not read discourse. Effect-written stat slots instead thread their Amount expressions. |
| Named-card choice domain | The card-description predicate is closed (`Predicate.check .object []`). Other choice-domain payloads use the consumer's context. |

## Index duties

### Noun words and refinements

`NounWord.ofType word ty` narrows a word by written type evidence;
`NounWord.copied word` narrows it by copy origin. They replace `typedCard ty`
with `ofType card ty`, and `abilityCopy` with `copied ability`. Each retains
the base word's carrier and kind. This follows the distinction between a type
word's implicit permanent carrier and an explicit card or spell word
[CR#109.2..109.2b], and between an original ability and a copy that is itself
an ability [CR#707.10]. No arbitrary predicate replaces the read discipline.

The remaining heads each supply an independent read requirement:

| Head | Requirement retained by refinements |
| --- | --- |
| `type ty` | Current battlefield carrier and remembered type; `ofType permanent ty` is not equivalent because `permanent` can also read a former battlefield carrier through its stamp. |
| `card`, `spell` | Their current card-zone or stack carrier. |
| `permanent`, `token` | Battlefield/stamp discipline for the former; current battlefield plus token origin for the latter. |
| `copy` | An object-payload copy, without imposing a card, spell, or permanent carrier; `copied ability` separately retains the ability head. There is no generic object word to expand this head through. |
| `ability`, `player`, `pile` | The corresponding payload category. |
| `stack`, `join` | The existing shared-stack and joined-reference disciplines, including their kind constraints. |

`wordReaches` applies refinements to the same binding. `halfReaches` applies
the whole refined word within each join half; it cannot take a type from one
half and copy origin from another. `verbedWordOk` retains the deed stamp and
event-carrier checks, and now receives origin so a copied refinement can
compose with that read. Ordinary self exclusion, plurality, windows,
uniqueness counting, `Reach.tracksObject`, and re-stamping policy are unchanged.

[Proofs/NounWords](Semantics/Proofs/NounWords.lean) pins matching identities,
incompatible types/carriers, original versus copied abilities, ambiguity,
join-half isolation, and provenance. Its three general commutativity theorems
cover type/copy refinement order in direct, half, and verbed matchers.
These refinements have no binding effects; this does not assert commutativity
of arbitrary Predicate modifiers.

### Remembered type evidence

Bindings preserve card-type facts as canonical, duplicate-free lists. A conjunction
unions its written card types; their order does not select a grammatical head.
An artifact creature supports both type words [CR#205.2b]. `[]` means no known
card type, not a claim that the referent has no types. Explicit card-type evidence
is retained before the existing subtype-derived fallback; a creature subtype does
not add a creature fact to an explicitly Kindred description. The separate
`Predicate.seedType` presupposition calculation is not stored type evidence.

A same-kind disjunction retains only facts common to every alternative. Joined
kinds keep separate evidence in each `HeadTy`/`Payload` half, with intersection
within each half's alternatives. A read must satisfy all refinements within one
half. The legacy flattened `Payload.ty` projection skips nonobject halves; it
must not be used to borrow evidence across halves. Choice-arm payloads and
same-kind group summaries also intersect their facts; narrowing a group or
remarking a tested referent adds facts to that referent.

Type alternatives retain conjunction/disjunction structure for role checks:
conjunction combines facts, disjunction requires each alternative to support the
role. An entirely unknown description retains the previous no-evidence behavior
for stat and deed checks. Damage and attachment checks can use any supporting
type within one conjunctive alternative. Attachment profiles collect every type
refinement, including when seeding or moving their referent.

Movement retains the remembered facts and changes the carrier separately.
A battlefield type word then stops matching, while a correspondingly refined
card word can match the card in exile [CR#109.2,109.2a,110.1]. This is evidence
for reference resolution, not a simulation of continuous characteristic changes.

`Proofs/TypeEvidence.lean` proves `pureTypePermutation` for arbitrary lists of
pure card-type modifiers, including duplicates and the empty list. Its concrete
pins cover both type words, missing types, common versus alternative evidence,
joined halves, damage, attachment profiles, and movement. This does not assert
that arbitrary modifiers with binding effects commute.


The source inventory is the indexed `data` declarations and parameterized
records in `idris/src/Experimental/*.idr`, including proof datatypes. An
indexed proof datatype names a rule below; a context-indexed syntax datatype
names the checking entry point that must receive its field's context. Names
in the first column identify the reference evidence, not APIs to extend.
Plain unindexed data/records impose no erased-index duty. `KeywordShapes` and
`FactsGen` have no indexed syntax. `Unspellable` is a proof utility, replaced
by exact-refusal theorems; it is not an authorable term.

### Card

| Reference datatype | Lean duty |
|---|---|
| `CardLine` | `Characteristics.cardLineOk`/`lineLaws` check the card-type and subtype relationships. |
| `CardBox` | `cardBoxOk` takes face side, types and text together; it checks printed literals, type fit, and characteristic-defined absent slots. |
| `CharacteristicsLaws` | `CardFace.check` supplies the side and checks line, text, box, cost and joint choices. |
| `SharedLineHalfLaws` | `SharedLineHalf.check` receives the shared characteristics and checks each half against them. |
| `LevelBandLaws` | `LevelBand.check` receives the inner characteristics and checks the range, text, and box. |
| `LevelBandsLaws` | `Card.leveler` checks every band, frame eligibility and `bandsDisjoint`. |
| `PrototypeAltLaws` | `prototypeAltCheck` receives the inner characteristics and checks the alternate cost and box. |

### Effect

| Reference datatype | Lean duty |
|---|---|
| `SpendPurpose` | `SpendPurpose.check` in [AbilityRules.lean](Semantics/Check/AbilityRules.lean) checks references and expected child kinds in the supplied context. |
| `ManaHeld` | `ManaHeld.check` in [AbilityRules.lean](Semantics/Check/AbilityRules.lean) checks references and expected child kinds in the supplied context. |
| `AsThough` | `AsThough.check` in [AbilityRules.lean](Semantics/Check/AbilityRules.lean) checks references and expected child kinds in the supplied context. |
| `Exchanged` | `Exchanged.check` in [AbilityRules.lean](Semantics/Check/AbilityRules.lean) checks references and expected child kinds in the supplied context. |
| `TokenQuality` | `TokenQuality.checkAll` checks each quality at the bundle context. |
| `CounterKindSource` | `CounterKindSource.check` in [AbilityRules.lean](Semantics/Check/AbilityRules.lean) checks references and expected child kinds in the supplied context. |
| `CostShift` | `CostShift.check` in [AbilityRules.lean](Semantics/Check/AbilityRules.lean) checks references and expected child kinds in the supplied context. |
| `CountBound` | `CountBound.check` in [AbilityRules.lean](Semantics/Check/AbilityRules.lean) checks references and expected child kinds in the supplied context. |
| `DeedComplement` | `DeonticPatient.check` checks each counterpart; `deonticPatientOk` checks its deed membership, role and relation to the subject. |
| `DeonticPatient` | The old deed/role indices were parameters shared by all constructors. `deonticPatientOk` and `deonticRiderOk` at `StaticSpec.deonticRule` enforce their actual relationships; child phrases use `DeonticPatient.check`. |
| `DamageScope` | `DamageScope.check` in [AbilityRules.lean](Semantics/Check/AbilityRules.lean) checks references and expected child kinds in the supplied context. |
| `DamageAgent` | `DamageAgent.check` in [AbilityRules.lean](Semantics/Check/AbilityRules.lean) checks references and expected child kinds in the supplied context. |
| `Unpreventable` | `Unpreventable.check` in [AbilityRules.lean](Semantics/Check/AbilityRules.lean) checks references and expected child kinds in the supplied context. |
| `PreventCut` | `PreventCut.check` in [AbilityRules.lean](Semantics/Check/AbilityRules.lean) checks references and expected child kinds in the supplied context. |
| `DamageScale` | `DamageScale.check` in [AbilityRules.lean](Semantics/Check/AbilityRules.lean) checks references and expected child kinds in the supplied context. |
| `DividedVerb` | `DividedVerb.check` in [AbilityRules.lean](Semantics/Check/AbilityRules.lean) checks references and expected child kinds in the supplied context. |
| `DividedTakes` | `Instruction.distribute` checks damage-recipient suitability or object kind according to the closed `DividedVerb` constructor. |
| `CtrlOverrideOk` | `TokenRider.under` checks player kind and `NounPhrase.ctrlOverrideOk`. |
| `ProducedMana` | `ProducedMana.check` in [AbilityRules.lean](Semantics/Check/AbilityRules.lean) checks references and expected child kinds in the supplied context. |
| `Repetition` | `Repetition.check` in [AbilityRules.lean](Semantics/Check/AbilityRules.lean) checks references and expected child kinds in the supplied context. |
| `EachStackOk` | `eachStackOk`, used by `enactKeepsOuter`, accepts preserved outer bindings or closure of the distributive agent’s own parts. |
| `DeckReadable` | `Predicate.deckReadable` in `DeckTrait.check` and `DeckCondition.check`, with an empty context and object kind. |
| `DeckComparable` | `QualitySort.deckComparable` in `DeckCondition.check`. |
| `QualityPayload` | `QualityPayload.check` in [AbilityRules.lean](Semantics/Check/AbilityRules.lean) checks references and expected child kinds in the supplied context. |
| `TokenSpec` | `TokenSpec.check` in [AbilityRules.lean](Semantics/Check/AbilityRules.lean) checks references and expected child kinds in the supplied context. |
| `StaticSpec` | `StaticSpec.check` in [AbilityRules.lean](Semantics/Check/AbilityRules.lean) checks references and expected child kinds in the supplied context. |
| `Compulsion` | `Compulsion.check` in [AbilityRules.lean](Semantics/Check/AbilityRules.lean) checks references and expected child kinds in the supplied context. |
| `PlayPayment` | `PlayPayment.check` in [AbilityRules.lean](Semantics/Check/AbilityRules.lean) checks references and expected child kinds in the supplied context. |
| `DeonticRider` | `DeonticRider.check` in [AbilityRules.lean](Semantics/Check/AbilityRules.lean) checks references and expected child kinds in the supplied context. |
| `DamageOp` | `DamageOp.check` in [AbilityRules.lean](Semantics/Check/AbilityRules.lean) checks references and expected child kinds in the supplied context. |
| `TokenRider` | `TokenRider.check` in [AbilityRules.lean](Semantics/Check/AbilityRules.lean) checks references and expected child kinds in the supplied context. |
| `Cost` | `Cost.check` in [AbilityRules.lean](Semantics/Check/AbilityRules.lean) checks references and expected child kinds in the supplied context. |
| `ManaRider` | `ManaRider.check` in [AbilityRules.lean](Semantics/Check/AbilityRules.lean) checks references and expected child kinds in the supplied context. |
| `CopyExcept` | `CopyExcept.check` in [AbilityRules.lean](Semantics/Check/AbilityRules.lean) checks references and expected child kinds in the supplied context. |
| `RollRow` | `Instruction.checkRows` checks the range and each row body; row bodies start at the same outer bindings. |
| `Instruction` | `Instruction.check` in [AbilityRules.lean](Semantics/Check/AbilityRules.lean) checks references and expected child kinds in the supplied context. |
| `Instructions` | The length is the actual list length; `Instruction.check` checks nonemptiness and `checkSeq` threads `Instruction.intro`. No independent length value remains. |
| `SimInstructions` | The length is the actual list length; nonemptiness is checked and `checkSim` threads `Instruction.annIntro`, keeping simultaneous outcomes out of sibling inputs. |
| `KeywordParam` | `KeywordParam.check` in [AbilityRules.lean](Semantics/Check/AbilityRules.lean) checks references and expected child kinds in the supplied context. |
| `AbilityLost` | `AbilityLost.checkAll` checks each carried keyword term or chosen-ability reference. |
| `AbilityAt` | `Ability.check` dispatches each ability constructor, its expected phrase kinds, and its explicit local contexts. |
| `StaticParts` | The actual list supplies length; `conjunction` checks nonemptiness and `checkParts` checks noncoordination and threads `StaticSpec.intro`. |
| `CostSeq` | The actual list supplies length; `Cost.compound` checks nonemptiness and `Cost.checkSeq` threads `Cost.intro`. |
| `AddedPayment` | `StaticSpec.addedCost` requires `Cost.offBattlefield`. |
| `AbilitySeq` | `Ability.checkText` threads the choices and letters introduced by earlier lines; nested ability lists use `Ability.checkAll` at their enclosing context. |

### Events

| Reference datatype | Lean duty |
|---|---|
| `LookbackSubject` | `lookbackSubjectOk` in `LookbackClause.check`. |
| `LookbackComplement` | `lookbackComplementOk` in `OptComplement.check`. |
| `ChoiceMode` | `ChoiceMode.check` in [PhraseRules.lean](Semantics/Check/PhraseRules.lean) checks references and expected child kinds in the supplied context. |
| `Possessable` | `Zone.possessable` in `ZoneScope.check`. |

### Phrase

| Reference datatype | Lean duty |
|---|---|
| `ChoiceInScope` | `countParts` in `Predicate.notChosen` and `Condition.choseThisWay` requires a compatible standing choice. |
| `ColorTerm` | `ColorTerm.check` in [PhraseRules.lean](Semantics/Check/PhraseRules.lean) checks references and expected child kinds in the supplied context. |
| `ZoneScope` | `ZoneScope.check` receives the containing zone and checks `Zone.possessable` plus the player kind. |
| `LibPlace` | `LibraryPlace.check`, plus `placeArrangementOk` and `placeOrdinalOk` in `ZoneExpr.check`. |
| `ZoneExpr` | `ZoneExpr.check` in [PhraseRules.lean](Semantics/Check/PhraseRules.lean) checks references and expected child kinds in the supplied context. |
| `NameSource` | `NameSource.check` in [PhraseRules.lean](Semantics/Check/PhraseRules.lean) checks references and expected child kinds in the supplied context. |
| `ChoiceDomain` | `sortedDomainCheck` checks both domain contents and `ChoiceDomain.sort` at `qualityNoun`, `ofYourChoice`, `entersChoice`, and `attachChoice`. |
| `EventSource` | `EventSource.check` in [PhraseRules.lean](Semantics/Check/PhraseRules.lean) checks references and expected child kinds in the supplied context. |
| `EventComplement` | `OptComplement.check` receives the event and subject kind, then checks complement kind and source/destination/locus. Replacement by ordinary event fields is owned by `lean-lookback-carries-a-game-event`. |
| `ComplementWritten` | `LookbackClause.check` requires a complement unless `bareLookbackOk` admits its absence. |
| `LookbackClause` | `LookbackClause.check` and `lookbackSubjectOk` receive the subject kind; `OptComplement.check` receives that same kind and event. |
| `Predicate` | `Predicate.check` receives an expected kind, checks every child, and uses `kindCheck`; conjunction children share a scope, joined alternatives receive their own inferred kinds. |
| `DetPhrase` | `DetPhrase.check` in [PhraseRules.lean](Semantics/Check/PhraseRules.lean) checks references and expected child kinds in the supplied context. |
| `RolesOk` | `rolesOk` in `NounPhrase.oneEachOf`, alongside per-role `Predicate.checkAll` and object-kind checks. |
| `Noun` | `NounPhrase.check` receives the expected kind, applies `ctxCheck`, and checks references in the supplied bindings. Authorable offset references remain owned by `lean-grammatical-reference-scopes`. |
| `Amount` | `Amount.check` in [PhraseRules.lean](Semantics/Check/PhraseRules.lean) checks references and expected child kinds in the supplied context. |
| `Quantity` | `Quantity.check` in [PhraseRules.lean](Semantics/Check/PhraseRules.lean) checks references and expected child kinds in the supplied context. |
| `NonZeroQ` | `Quantity.nonZero` in the consumers that require nonzero cardinality; zero remains allowed in other quantity slots. |
| `SliceCount` | `SliceCount.check` in [PhraseRules.lean](Semantics/Check/PhraseRules.lean) checks references and expected child kinds in the supplied context. |
| `ComplementAnchor` | `NounPhrase.anchorPhrase` and singularity in `Predicate.otherThan`. |
| `LinkSource` | `NounPhrase.linkSource` in `Predicate.exiledWith`. |
| `PileMention` | `NounPhrase.pileMention` plus noun checking in `Predicate.inPile`. |
| `PaidSubject` | `NounPhrase.paidSubjectOk` and object-kind checking in `Amount.paid`. |
| `DestOk` | `ZoneExpr.destOk` in `Instruction.move`. |
| `SearchScope` | `SearchScope.check` in [PhraseRules.lean](Semantics/Check/PhraseRules.lean) checks references and expected child kinds in the supplied context. |
| `FlipScope` | `FlipScope.check` in [PhraseRules.lean](Semantics/Check/PhraseRules.lean) checks references and expected child kinds in the supplied context. |
| `IgnoredOutcomes` | `IgnoredOutcomes.check` in [PhraseRules.lean](Semantics/Check/PhraseRules.lean) checks references and expected child kinds in the supplied context. |
| `Ballot` | `Ballot.check` in [PhraseRules.lean](Semantics/Check/PhraseRules.lean) checks references and expected child kinds in the supplied context. |
| `CostSubject` | `NounPhrase.costSubjectOk` in `StaticSpec.costShift` and `altCost`. |
| `StackActOn` | `NounPhrase.counterable`/`copiable` at `counterSpell`, `chooseNewTargets`, and copying consumers. |
| `Condition` | `Condition.check` in [PhraseRules.lean](Semantics/Check/PhraseRules.lean) checks references and expected child kinds in the supplied context. |
| `MarkingOk` | `markingOk` in `StaticSpec.conditional`. |
| `TokenPhrase` | `NounPhrase.tokenPhrase` plus object-kind checking in the token-creation event. |
| `Exposed` | `Exposed.check` in [PhraseRules.lean](Semantics/Check/PhraseRules.lean) checks references and expected child kinds in the supplied context. |
| `VisibleThing` | `VisibleThing.check` in [PhraseRules.lean](Semantics/Check/PhraseRules.lean) checks references and expected child kinds in the supplied context. |
| `DamageRecipient` | `NounPhrase.damageRecipient` in damage instructions and `DamagePatient.check`. |
| `StatusHolder` | `Instruction.setStatus` requires the object kind and battlefield zone. |
| `NotAnAbility` | `NounPhrase.movable` excludes ability phrases at `Instruction.move`. |
| `DiscardOk` | Discard is an enacted move; the deed facts and `enactPatientZoneOk` check its source. The macro authoring boundary above excludes arbitrary authored tag/body combinations. |
| `Movable` | `NounPhrase.movable` at `Instruction.move` excludes nonmovable terms, including ability phrases. |
| `DurationEnd` | `DurationEnd.check` in [Triggers.lean](Semantics/Check/Triggers.lean) checks references and expected child kinds in the supplied context. |

### Triggers

| Reference datatype | Lean duty |
|---|---|
| `ManaTypeTerm` | `ManaTypeTerm.check` in [Triggers.lean](Semantics/Check/Triggers.lean) checks references and expected child kinds in the supplied context. |
| `Duration` | `Duration.check` in [Triggers.lean](Semantics/Check/Triggers.lean) checks references and expected child kinds in the supplied context. |
| `AttackDefender` | `AttackDefender.check` in [Triggers.lean](Semantics/Check/Triggers.lean) checks references and expected child kinds in the supplied context. |
| `DamagePatient` | `DamagePatient.check` in [Triggers.lean](Semantics/Check/Triggers.lean) checks references and expected child kinds in the supplied context. |
| `Door` | `Door.check` in [Triggers.lean](Semantics/Check/Triggers.lean) checks references and expected child kinds in the supplied context. |
| `RollWatch` | `RollWatch.check` in [Triggers.lean](Semantics/Check/Triggers.lean) checks references and expected child kinds in the supplied context. |
| `CrimeSubject` | `GameEvent.commitsCrime` checks a player subject and singularity. |
| `GameEvent` | `GameEvent.check` in [Triggers.lean](Semantics/Check/Triggers.lean) checks references and expected child kinds in the supplied context. |
| `Causing` | `Causing.check` in [Triggers.lean](Semantics/Check/Triggers.lean) checks references and expected child kinds in the supplied context. |
| `HeaderPossessor` | `HeaderPossessor.check` in [Triggers.lean](Semantics/Check/Triggers.lean) checks references and expected child kinds in the supplied context. |
| `TriggerWindow` | Merged into `Timing.duringPart`; `Timing.check` checks the possessor and `windowOk`. |
| `Timing` | `Timing.check` in [Triggers.lean](Semantics/Check/Triggers.lean) checks references and expected child kinds in the supplied context. |
| `AltEvent` | `headerEventCheck` checks each alternative event for header nontargeting and valid status. The old header-word index is gone; the event determines its own event class. |
| `Concurrent` | `Concurrent.check` in [Triggers.lean](Semantics/Check/Triggers.lean) checks references and expected child kinds in the supplied context. |
| `JoinedHeader` | `JoinedHeader.check` in [Triggers.lean](Semantics/Check/Triggers.lean) checks references and expected child kinds in the supplied context. |

### Words

| Reference datatype | Lean duty |
|---|---|
| `OptOk` | Optional consumers use their `Opt*.check` or `map`/`getD` branches; absence introduces no payload to check. |
| `Joins` | `joinKinds`, `Predicate.kindOfAll`, and `NounPhrase.kind?` derive the resulting kind; there is no independently supplied result index. |
| `KnownAct` | `knownAct`/`knownActs` at deed consumers read the deed facts; the macro definition is trusted to supply the corresponding expansion. |
| `Payload` | `Payload.kind` derives the kind from the constructor; `Binding` has no separate kind field that can disagree. |
| `HeadTy` | `NounPhrase.ty` and `Payload.ty` derive lists of known card-type facts; joined kinds retain evidence per half. |
| `DieSides` | `DieSides.check` in [AbilityRules.lean](Semantics/Check/AbilityRules.lean) checks references and expected child kinds in the supplied context. |
| `Targetable` | `Kind.targetable` at `DetPhrase.check`, copying and target consumers. |
| `Targeter` | `Kind.targeter` in `Predicate.check` and `DeonticPatient.check`. |
| `Phrasal` | `Kind.phrasal` at copying, iteration and noun consumers. |
| `KnownKeyword` | `knownKeyword`, `KeywordTerm.known`, and `keywordParamFits` at keyword consumers. |
| `BasicLandType` | The old `basicLandLine` macro has no Lean counterpart. No caller can supply a proof-erased list to that removed API; `SubtypeSpace.basicLand` is a closed constructor. |
| `BasicLandTypes` | Same removed `basicLandLine` API as `BasicLandType`; ordinary characteristic subtype validation remains `subsFitLine`. |
| `Delta` | The type parameter survives as `Delta α`; it was not erased. `Delta Amount` consumers check the carried amount. |
| `GivingWarrant` | `conferralOk` at `Instruction.gainDesignation` checks the designation and its declared conferrer together. |
| `DesignationHolder` | `designationHolderOk` and the declared designation scope/holder checks at designation consumers. |
| `AxesAt` | `axesAt` in `Predicate.check` requires a nonempty list whose axes all have the expected scope. |
| `StatusVal` | `Status.category` derives the category from the status. No separate category argument can disagree; consumers retain `Status.markable` and status-holder checks. |

### Records and dependent pairs

- `InstrProfile bs`: the old `bs` parameter did not constrain any record field;
  the four binding fields remain, with explicit checking/traversal consumers.
- `TokenChars bs`: `Characteristics.checkWritten` and
  `CharacteristicBundle.check` validate the carried expressions; `statsCheck`
  threads the former power/toughness dependent pair in slot order. Nested
  characteristic text remains closed.
- `Binding`'s kind/payload pair and `unionPayload`'s returned dependent pair:
  the kind is now derived from the payload, so there is no separate index to
  validate. Union payloads retain only card-type facts shared by both arms.
- Other erased implicit binders on functions and macros refer to these same
  datatype duties; erasing the binder does not remove the obligation at the
  resulting term's consumer. In particular, macros are not an independent
  certificate for arbitrary enacted bodies; the authoring boundary above establishes their trusted origin.
