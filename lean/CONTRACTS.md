# Checker contracts

The Lean model is a prototype. Its authority is rules correctness and internal
consistency, not compatibility with previous checker behavior. Tests that encode
obsolete shapes or incorrect laws must be corrected with an explanation and
meaningful replacement coverage (user clarification, 2026-09-06).

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

## Attachment predicates and current face

`Predicate.attachment` explicitly identifies whether its candidate is the host
or the attachment. The optional attachment word restricts the relation's kind;
it never supplies direction. An absent counterpart tests existence. A present
counterpart retains its own quantity and introduces exactly its original
mentions, including nested targets, into the surrounding phrase.

Host predicates admit object and player domains. Enchanted-player predicates
are accepted, consistent with the existing enchanted-player noun and
[CR#303.4b]. Equipped or fortified player hosts are refused. Existing host
card-type restrictions, quantified attachment counts, and counterpart target
publication remain checked. The old `isAttached`, `attachedBy`, and `attachedTo`
names are trusted macros over the shared predicate.

`currentFace` describes a single double-faced permanent as a whole. Its front
and back alternatives are distinct from face-up and face-down Status. The
`isTransformed` macro expands to the back alternative: prior transformation,
copying back-face characteristics, or a back-face-up component in a merged or
melded object is insufficient [CR#701.27g]. The checker validates the predicate's
object domain and battlefield context; runtime evaluation of current game state
remains Rust's responsibility. The fold does not claim an execution proof.

## Historical events

`LookbackClause` contains a `GameEvent` and a time window. The `relative` macro
marks the participant described by the enclosing predicate or condition.
That participant is checked against the host kind by the same recursive checker
that validates ordinary event participants. A gap outside a lookback is refused;
a nested lookback binds its own participant and cannot supply its parent's gap.
Repeated occurrences may refer to the same participant, but each lookback must
use its own bound participant at least once.

`GameEvent.facts` supplies magnitude, replacement, counting, duration, and
concurrency properties directly from the event constructor. Source sets and
exclusions use `EventSource`; named actions carry an optional locus checked
against their declared features. A cast may leave its spell unspecified, which
introduces no spell reference. Historical-event profiles retain named
participants without simulating a new event or introducing its outcome.

`Proofs/Lookback.lean` checks binding isolation, retained references, source and
locus validation, and event-fact preservation through modifiers. These are
checker guarantees; no engine history-query implementation is claimed.

## Grammatical references and internal windows

Ordinary `it`, `them`, and worded pronouns resolve in the whole containing
context and require exactly one compatible antecedent. The authoring boundary
excludes raw `NounPhrase.pro` and `Window` construction, including both numeric
windows and introduction patterns. Named scope macros carry the phrase,
instruction or condition whose mentions they re-read; their expanded windows
are internal checker data, not positional reads in the authorable grammar.

`Window.introduced` and `outsideIntroduced` validate an ordered introduction
pattern against the actual context with `introductionWidth`. Every non-letter
binding must match its expected kind at the current position. Matching does
not search past an unrelated binding. Letter entries are conditional because
`introducedLetters` creates a binding only when that letter was absent from the
input context. A missing letter therefore contributes zero width instead of
letting an empty-context count consume an outer object or player. A mismatch
produces an empty read window and cannot mutate outer bindings.

The patterns describe binding kinds, not referent identities or rules meanings.
Their ownership comes from the trusted scope macro's expansion. Numeric
`top`/`below` windows remain available for internal diagnostics and the fixed
single-subject comparison and chooser slots. They have no card-authoring constructor.

The empty-context calculations now produce conditional introduction patterns,
never an assumed fixed depth. The audit covers every affected helper:

| Scope macro | Owning phrase and boundary |
| --- | --- |
| `itPrior` | The preceding instruction's output, through `Instruction.intro`; existing outer mentions are not part of the empty-input pattern. |
| `itCondSubject` | The condition's published mentions. The author no longer supplies a `Bindings` argument. |
| `dealDamageOwnPower` | The source's self-subject introduction before the amount is read. The author no longer supplies a `Bindings` argument. |
| `agentRef` | The agent's own introduction, including singularization of distributive agents. An agent introducing nothing is re-read directly. |
| `itsOther` / `sameWindow` | The first stat modification's subject and amount introductions, in the same order as `StaticSpec.intro`. An originally empty introduction uses the whole context. |
| `ownSubject` | The shared subject's introduction; an empty owned introduction remains empty and cannot capture an outer antecedent. |
| `lookedCards` | The library slice's introduction, including its possessor's mentions. |
| `attachToIt` / `requireBlockIt` | The containing context outside the current noun's introduction. Ambiguity within that outer context remains an error. |
| `aTheirChoice` | The immediately preceding chooser mention. The macro requires that grammatical caller position; the fixed one-binding window is independent of the outer context. |
| `comparesOwnStat` | The one subject binding that `Predicate.check` places immediately before the measure. Its fixed internal depth is independent of outer context and letters. |

`Proofs/ReferenceScopes.lean` pins the former X-reuse counterexample, scope twins
with and without outer bindings, preservation of selected type/carrier facts,
outer ambiguity, and authoring rejection of raw numeric and patterned reads.
Two general theorems establish the own-stat read and its creature type for every
outer context, with either a fresh or an already-bound X. Another proves exact
prefix isolation for arbitrary non-letter introductions; the comparison and
chooser scopes are also proved for arbitrary outer contexts. These are reference-checking laws,
not positional lowering or runtime object-identity proofs.

## Controller as the subject of a sacrifice

`controllerSacrifices subject` composes `controllerOf`, an owned subject read,
and `sacrifice`. The relational noun introduces the target object once and
then its controller; the deed's body reads that object without declaring a
second target. `NounPhrase.introducesOwnReferent` distinguishes a phrase's own
introduced referent from mentions nested in its arguments. For such phrases,
the read selects the controller and that referent; nested object descriptions
cannot make the patient ambiguous. Coordinated references have no single
introduced head and retain ordinary ambiguity checks. An already-referential subject is read directly again because
it introduces no new object. The ordinary sacrifice expansion moves the
patient to the graveyard and records sacrifice provenance [CR#701.21a].

The Arcum Dagsson fragment retains the former constructor's input and
post-action bindings: one controller and the same artifact creature, now in
the graveyard with its sacrifice stamp. Subsequent player, card and stamped
permanent reads use those bindings. The generic deed machinery additionally
provides an explicit stamped rider context; the removed special constructor
had no rider context and fell back to its unstamped input. This representation
change is intentional and separately pinned.

`Proofs/ControllerSacrifice.lean` proves the full profile for arbitrary outer
contexts, target multiplicity, the existing-reference case, and later reads.
No controller-sacrifice constructor or dedicated validation, profile, cost,
reflexive-enclosure or number-slot arm remains. Macro authoring still establishes
trusted expansion origin; these are checker/reference guarantees, not an engine
execution proof.

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

## Definitions and composition

`StaticSpec.letterDefinition` is the one numeric definition form. The `define`
macro establishes it without a duration. Both static and instruction positions
check the amount in the incoming scope, close an already-open letter, and reject
an unused or duplicate definition. An earlier target or affected-object mention
remains available to a trailing definition; nothing is hoisted. A durationless
numeric definition is admissible inside a cost, retaining the former instruction
form's behavior. Other established specs and duration-bearing definitions retain
their cost refusal. The formerly separate `establish (letterDefinition …) none`
spelling now has this same cost admissibility and clamped number slot.

`withContinuation` carries an optional policy with its deciding player, or a
required policy. Its positive branch is conditioned on the decision or start of
payment [CR#118.12], not a test of the resulting events. The policy retains the
existing mandatory-enclosure and armed-continuation checks. `repeat_ (.fixed …)`
carries its count and body; the other repetition policies remain unchanged.

Noun conjunctions and alternatives are lists, as are cost alternatives. They
require at least two members; the binary `both`, `eitherOf`, and `either` macros
supply two-member lists. Noun conjunctions thread introductions in textual order;
alternatives check their members independently. Predicate and condition lists
retain their existing cardinality and scope laws. `sequentially` threads instruction
outputs, while `simultaneously` threads announcements without publishing sibling
outcomes. Both require a nonempty list. These are syntax and checker guarantees;
they do not prove Rust execution equivalence.

## Enclosed traversals

These are explicit boundaries in the surrounding checker, not exceptions to
be inferred from a failed pronoun:

| Construction | Context rule |
|---|---|
| Sequential instructions, compound costs, coordinated statics | Thread each member's output into the next member. |
| Simultaneous instructions | Thread announcements; sibling events do not read each other's outcomes. |
| Predicate sibling modifiers, condition siblings | Check in the same containing context; do not make conjunction sequential. |
| Alternatives, modes and result-table rows | Branch from their enclosing context; successful branch-local mentions are not unconditionally exported. A mode's cost precedes its own body. |
| `withContinuation` (`offer` / `doIfDone` macros) | The success arm sees the body output; the failure arm starts before the body. |
| `doIf`, `doOnlyIf` | Each branch checks before either branch's effects. Postposed conditions see the primary instruction's pre-state. Branch-local mentions and outcomes do not escape; facts about prior mentions are joined across branches. |
| `doForEach`, `doForEachKind`, `repeat_ (.fixed …)` | Check the body in the element/value/count context; verify preservation of outer bindings and pluralize exported local mentions. |
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
| `LookbackSubject` | `LookbackClause.check` binds the described participant, and the shared event checker validates each use. |
| `LookbackComplement` | Ordinary `GameEvent` participant checks; there is no separate complement-kind table. |
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
| `EventComplement` | Retired in favor of event participant, source, destination, and locus fields. |
| `ComplementWritten` | Required event fields cannot be omitted. An action that declares a locus requires that locus in the shared event checker. |
| `LookbackClause` | The shared checker validates the carried event under a locally bound participant of the host kind. |
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
| `StackActOn` | `NounPhrase.counterable`/`copiable` at `chooseNewTargets` and copying consumers. Counter expands to enacted moves, whose source-zone checks report `ZoneFits`. |
| `Condition` | `Condition.check` in [PhraseRules.lean](Semantics/Check/PhraseRules.lean) checks references and expected child kinds in the supplied context. |
| `MarkingOk` | `markingOk` in `StaticSpec.conditional`. |
| `TokenPhrase` | `NounPhrase.tokenPhrase` plus object-kind checking in the token-creation event. |
| `Exposed` | `Exposed.check` in [PhraseRules.lean](Semantics/Check/PhraseRules.lean) checks references and expected child kinds in the supplied context. |
| `VisibleThing` | `VisibleThing.check` in [PhraseRules.lean](Semantics/Check/PhraseRules.lean) checks references and expected child kinds in the supplied context. |
| `DamageRecipient` | `NounPhrase.damageRecipient` in damage instructions and `DamagePatient.check`. |
| `StatusHolder` | `Instruction.setStatus` requires the object kind and battlefield zone. |
| `NotAnAbility` | Retired at movement: the action-fold ticket explicitly corrects this false blanket prohibition; ending a turn or combat can exile stack abilities [CR#724.1b,724.2b]. |
| `DiscardOk` | Discard is an enacted move; the deed facts and `enactPatientZoneOk` check its source. The macro authoring boundary above excludes arbitrary authored tag/body combinations. |
| `Movable` | `NounPhrase.movable` at `Instruction.move` admits objects (including abilities) and piles, and excludes players and quality values. |
| `DurationEnd` | `DurationEnd.check` in [Triggers.lean](Semantics/Check/Triggers.lean) checks references and expected child kinds in the supplied context. |

### Triggers

| Reference datatype | Lean duty |
|---|---|
| `ManaTypeTerm` | `ManaTypeTerm.check` in [PhraseRules.lean](Semantics/Check/PhraseRules.lean) checks references and expected child kinds in the supplied context. |
| `Duration` | `Duration.check` in [Triggers.lean](Semantics/Check/Triggers.lean) checks references and expected child kinds in the supplied context. |
| `AttackDefender` | `AttackDefender.check` in [PhraseRules.lean](Semantics/Check/PhraseRules.lean) checks references and expected child kinds in the supplied context. |
| `DamagePatient` | `DamagePatient.check` in [PhraseRules.lean](Semantics/Check/PhraseRules.lean) checks references and expected child kinds in the supplied context. |
| `Door` | `Door.check` in [PhraseRules.lean](Semantics/Check/PhraseRules.lean) checks references and expected child kinds in the supplied context. |
| `RollWatch` | `RollWatch.check` in [PhraseRules.lean](Semantics/Check/PhraseRules.lean) checks references and expected child kinds in the supplied context. |
| `CrimeSubject` | `GameEvent.commitsCrime` checks a player subject and singularity. |
| `GameEvent` | `GameEvent.check` in [PhraseRules.lean](Semantics/Check/PhraseRules.lean) checks references and expected child kinds in the supplied context. |
| `Causing` | `Causing.check` in [PhraseRules.lean](Semantics/Check/PhraseRules.lean) checks references and expected child kinds in the supplied context. |
| `HeaderPossessor` | `HeaderPossessor.check` in [PhraseRules.lean](Semantics/Check/PhraseRules.lean) checks references and expected child kinds in the supplied context. |
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


### Characteristic edits and keyword argument schemas

`StaticSpec.characteristicChange` and `CopyExcept.edits` carry the same typed
`CharacteristicEdit` list. `TypeLineChanges` uses optional axes: `none` leaves
an axis unchanged, while `some []` explicitly clears it under `sets`. Add and
remove operations require values. Type and subtype writes in one ordinary
edit block share host evidence, independent of their order. Ordinary edit
validity still reports the aggregate `becomesOk` refusal; copy edits retain
their separate nonempty-line and canonicality checks and validate operations
without requiring the source to be on the battlefield.

Numeric edits thread a scope local to their block. Other edit payloads read
the incoming scope; nested written abilities remain closed. The subject of
an ordinary change is not published before its edits. Ability-removal
selectors retain the subject's battlefield check: specified written abilities
read the incoming scope, while an all-except predicate reads the subject's
self context. Separate copy exceptions each read the incoming scope.
`CopyExcept.ability`, `thisAbility`, and `entersWithCounters` retain the copy
context; sharing edits does not turn copy exceptions into later static effects.

`QualityPayload` is an authoring input in `Macros.CharacteristicInputs`, not a
checked syntax field. It expands to edits. Ordinary additions add their type
line but set written colors and numeric characteristics. Named additions and
power/toughness removal remain refused. `copyCharacteristics` expands the old
packed input into one edit block and separate ability exceptions. The former
minimum-two-fields packing rule has no semantic counterpart in an edit list.

`Ability.keyword` takes an ordered `List KeywordParam`. Generated
`KeywordFacts.argumentSchemas` enumerate admissible orders and each quality
argument's domain. The generator reads declaration columns and existing
variant metadata; the checker does not branch on keyword names. In particular,
qualified cost forms retain their object-quality constraint and declared
cost-only alternative. Quality arguments can read the incoming context;
subject arguments, numbers, costs, and deck conditions retain their closed
contexts. Argument lists do not publish bindings between elements. The registry
codec still limits declaration combinations; changing that codec is outside
this fold.


### Movement without an arrival zone

`Instruction.move` carries an optional destination. `some destination` keeps
the existing destination, arrangement, type, and rider checks; `none` records
an exit without asserting an arrival zone and requires no riders. Both forms
check the subject and preserve its reference publication. Destinationless
movement does not itself mean countering: that label and a spell's graveyard
destination belong to the Counter expansion [CR#701.6a].

An ability binding stores its zone, initially the stack. Movement updates that
zone while retaining its ability identity and copy origin. An exited ability
can still be referenced as an ability, but a stack reference and counter/copy
checks require actual stack evidence. Element bindings and unions retain
location evidence rather than restoring a hardcoded stack location. This is
reference-state checking, not a runtime lifetime or state-based-action model.

`Instruction.clearDamage` removes all marked damage from a permanent. Its
subject must be an object on the battlefield, but need not currently be a
creature [CR#120.6]. It publishes the ordinary subject reference and no damage
outcome. Regeneration composes this primitive with tapping and conditional
removal from combat inside a destruction replacement.


### Captured operands and expanded actions

`Instruction.withOperands` resolves noun operands once, left to right, in their
actual context. An unavailable required operand prevents the entire body. Each
`Window.operand` reads the nearest lexical frame, including inside an effect
established by the body. The frame is checker-private; it adds no ordinary
pronoun candidate. Its addresses point to the original binding slots rather
than payload-equal copies. Literal subjects without public mentions are inline
values. Nested scopes resolve their arguments against the outer frame before
opening their own frame; updates reach the same captured object. Forgetting an
ordinary discourse mention keeps its slot private until the last capture closes;
all aliases continue to share its identity across filtering and conditional joins.

This is necessary for multi-operand macros. For example, a second target that
uses an existing X introduces no new X. Computing its introduction pattern in
an empty context can mistake the existing X for a new binding and shift a prior
pronoun past its object. Captures use the actual introductions, while the older
kind-pattern windows above remain appropriate only at their documented immediate
boundaries. This operand scope is independent of the deferred collection-member
binder and numeric aggregation design.

Fight uses a single creature-and-battlefield guard for both captured subjects,
then simultaneous damage at each subject's power. The whole-body operand scope
also retains target resolution requirements [CR#701.14a..701.14c]. The checker
no longer borrows attacking-creature admissibility for fight.

Regeneration installation creates a next-time-this-turn destruction replacement.
The Regenerate label belongs to its application: clear marked damage, have the
controller tap the permanent, and remove it from combat if applicable. A static
regeneration replacement uses that application repeatedly; it does not install
a further shield [CR#701.19a..701.19c].

Counter selects between an enacted exit for an ability and an enacted move to
the graveyard for a spell. Both retain the stack-source obligation [CR#701.6a].
Conditional publication conservatively joins branch facts, so it does not invent
an arrival zone for an ability or leave a countered object on the stack. The
pre-state remains available to postposed conditions such as Ertai's Trickery.
An alternative may also re-read the primary clause's stated amount, as in
Caustic Bronco's "otherwise ... that much"; this is a numeric reference, not
evidence that the primary branch actually happened.

Losing counters captures its player before checking the amount, then uses the
shared removal instruction and removed-counter outcome. Cost symbols, draw,
Room operations, and exchange remain unchanged in this pass.
