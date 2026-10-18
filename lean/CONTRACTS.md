# Checker contracts

> **Target architecture (2026-10-14):** [Semantics v3](../docs/decisions/semantics-v3.md)
> specifies the clean-sheet DRT checker (`DRS`, `ReferentId`, nested DRS boxes,
> `J1` accessibility, `J2` clause-mate exclusion, `D′` bare-pronoun resolution,
> and `AC>R1` demonstrative resolution), the explicit `Definition` vs.
> `Template` split, and the Unified Action-Clause Spine (`instruct`, `does`,
> `would`, `cant`/`must`/`can`/`gatedBy`, `did`, `by`, `any`, `which`) that
> replace the `Bindings` stack, `Window`, and strict `countReach == 1` rules
> described below as the `lean-v3-*` tickets land.

The Lean model is a prototype. Its authority is rules correctness and internal
consistency, not compatibility with previous checker behavior. Tests that encode
obsolete shapes or incorrect laws must be corrected with an explanation and
meaningful replacement coverage (user clarification, 2026-09-06).

The checker's charter (`docs/decisions/semantics-v2.md` §7, ruling 2026-10-06)
is that a written term is a faithful reading of the card's English: its
references resolve (anaphora, outcomes, choices, stamps, scope windows,
plurality, the handoff performer) and its terms are well-kinded. It is not a
rules engine. An instruction the game cannot carry out is still a faithful
reading, since the impossible part is ignored and the rest is done
[CR#101.3,609.3]. The game-rule refusals below (zones, timing windows, costs
and deontics, tokens, grants, the card frame) are frozen: no new one lands
without a dated ruling, and the existing ones are retired bucket by bucket by
the `lean-checker-sweep-*` tickets, which update the sections they touch. The
zone and object-state bucket's 33 refusals are retired (`lean-checker-sweep-zone`,
2026-10-08): a status, damage, an attack, a move, an attachment, an exchange,
a stack action or a stat change no longer demands where its object is or which
objects it can reach. Where such a refusal also fixed a kind, the kind is
checked by `kindMismatch` or `kindLte`. The deed table's role checks
(`deedFits`, `deedNounOk`) are not in this bucket and still read a deed's zone
and card types. The timing and window bucket's 10 refusals are retired
(`lean-checker-sweep-timing`, 2026-10-08): an activation, cast or trigger window,
the possessor of a turn part, an added turn part and its anchor, a "while" event,
the event that ends a duration, a usage limit, the event a replacement or a
trigger multiplier watches, a held clause, a nested "instead" and a chapter
ability's header no longer demand when the game can do what they say. The cost
and deontic bucket is retired (`lean-checker-sweep-costs`, 2026-10-08): 14 of
its 15 refusals go, so the checker no longer asks who pays a cost, whether a
symbol or an instruction can be a cost, whether modes are costed alike, or which
deeds a permission, a bound, an "as though" or a play rider goes with; an empty
mana run is refused as an empty list (`nonEmpty`), a term shape. The fifteenth,
`deonticPatientOk`, stays narrowed to references and kinds: a counterpart is not
the subject read back, is of a kind the deed's other role takes, and a
complement names one of the rule's deeds once. The other game-rule bucket is
retired (`lean-checker-sweep-game-rules`, 2026-10-08): 21 of its 43 refusals go,
so the checker no longer asks whether a deed rides, whether fateseal opens an
opponent's library, whether a deed's row admits the handed performer (the
handoff's kind is still checked, `kindMismatch`), whether a static ability, a
trigger header, an event or a rules row targets, whether a door is unlocked by
name, whether an exchange names one value twice, whether an ability can be
granted, granted to that subject, carved out of a retention or established as a
clause, whether a characteristic edit fits its subject, op and zone, whether a
shield's use fits, whether a designation is one an instruction confers, or what
a token's or an emblem's types, power and toughness, subtypes, abilities and
qualities must be. Their term shapes stay under existing constructors: an empty
edit list, emblem or color list is `nonEmpty`, an edit that adds or removes an
empty type line is `lineNonEmpty`, a list naming a value twice is `distinct`, and
an edit's chosen quality is `qualityRead`. Eight stay as kind, plurality or term
shape: `designationScope` (a designation's holder sort), `facesFit`,
`manaSymbolOk`, `twoParties`, and the registry and table shape checks
`tokenNamed`, `definitionHolder`, `definitionNamed` and `definitionScoped`. The
fourteen card-frame rules (`cardLine` through `prototypeFrame`) are kept: the owner
decided so on 2026-10-08 (§7 ruling 2026-10-06, consequence (d)), because they check
that a term is a well-formed card, not what an instruction does. Zone state on bindings stays, because references resolve
on it. On the question of
scope this supersedes "rules correctness" in the paragraph above; the rest of
that paragraph stands.

The checker receives the information that indexed syntax formerly carried in
its types. This document records where those duties live. A passing check is
an admission of the written semantic term, not a proof of lowering or execution.
The remaining representation boundaries are named below rather than hidden by
the existence of a checking function.

## Card authoring and trusted expansion

`spelled <| card` is the checked authoring boundary. It retains an
`Authoring.Form` tree before unfolding erases named macro calls, and constructs
`Spelled` with both `onlyAuthored` and the existing `ok : card.check = []` proof.
The constructor is private. Record construction and record update cannot replace
these checks. Unregistered helper definitions and nested macro arguments are
inspected; opaque or variable semantic content without authoring evidence is
refused, including expressions nested in printed-data records.

Semantic expressions must use registered macro declarations. Printed-data
records, enumerated leaf values, and numeric literals remain ordinary data;
their semantic fields are inspected recursively. `Semantics.Macros.Primitives`
provides named wrappers with definitionally identical constructor signatures.
`Action.enact`, `NounPhrase.pro`, and `Window` have no primitive wrapper:
they are expansion carriers owned by the trusted macro layer. Existing idiomatic
templates and definitions use `semantic_template` and `semantic_definition` declarations. Defaults retain their calls.

A declaration generates the hidden construction context, parameter bindings, and
registration from one ordinary Lean definition body. It returns its semantic
type directly; `expand` is a transparent identity, not a trust boundary. Only
registered definitions are trusted. Neither helpers, discarded arguments, nor
projections may conceal raw semantic syntax, even if their result is ordinary
data. Inspection skips ordinary helper bodies only after scanning the instantiated
expansion's reachable definitions for semantic types, constructors, and calls;
its arguments are still inspected. This includes semantic functions hidden in
containers and supplied higher-order macro calls. Generated binding and
caller-scope constructors have no public primitive.

A supplied parameterized body is inspected under evidence for its explicit noun
parameters. Other semantic free variables remain refused. Callback type aliases
are resolved during type elaboration; callbacks receive the receiving macro's
construction context. A caller cannot substitute a different hidden lexical
context. The authoring tree records formal body binders and their uses as well
as the enclosing named call.

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
[CR#303.4b]. Equipped or fortified player hosts are refused (`kindLte`: an
Equipment's or Fortification's host is an object [CR#301.5,301.6]). Host
card-type restrictions written in one description remain a contradiction
(`contradictionFree`); a counterpart's or a host word's card types are no longer
checked against the attachment word (`lean-checker-sweep-zone`). Quantified
attachment counts and counterpart target publication remain checked. The old `isAttached`, `attachedBy`, and `attachedTo`
names are trusted macros over the shared predicate.

`currentFace` describes a single double-faced permanent as a whole. Its front
and back alternatives are distinct from face-up and face-down Status. The
`isTransformed` macro expands to the back alternative: prior transformation,
copying back-face characteristics, or a back-face-up component in a merged or
melded object is insufficient [CR#701.27g]. The checker validates the predicate's
object domain; runtime evaluation of current game state
remains Rust's responsibility. The fold does not claim an execution proof.

## Historical events

`LookbackClause` contains a `GameEvent` and either a span (`mk`: a `Duration`, with an
optional `Timing` the event happened under) or an occurrence (`thisWay`, the event the
enclosing instruction caused; `triggering`, the event that triggered the ability), which name no
span (`semantics-v2-temporal-suite`). The `relative` macro
marks the participant described by the enclosing predicate or condition.
That participant is checked against the host kind by the same recursive checker
that validates ordinary event participants. A gap outside a lookback is refused;
a nested lookback binds its own participant and cannot supply its parent's gap.
Repeated occurrences may refer to the same participant, but each lookback must
use its own bound participant at least once.

`GameEvent.facts` supplies the magnitude property directly from the event
constructor; which events a replacement, a trigger multiplier, a duration or a
"while" clause may watch is not checked (`lean-checker-sweep-timing`). Source sets and
exclusions use `Location.not`; named actions carry an optional locus checked
against their declared features. A cast may leave its spell unspecified, which
introduces no spell reference. Historical-event profiles retain named
participants without simulating a new event or introducing its outcome.

`Proofs/Lookback.lean` checks binding isolation, retained references, source and
locus validation, and event-fact preservation through modifiers. These are
checker guarantees; no engine history-query implementation is claimed.

## Grammatical references and internal windows

Ordinary `it`, `them`, and worded pronouns resolve in the whole containing
context and require exactly one compatible antecedent. This is the checker's
current behaviour, not the target: `docs/decisions/semantics-v2.md` §7 (ruling
2026-10-07) replaces it with measured resolution rules, which apply once
`lean-drt-anaphora-refactor` lands. The authoring boundary
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
| `itPrior` | The preceding instruction's output, through `Action.intro`; existing outer mentions are not part of the empty-input pattern. |
| `itCondSubject` | The condition's published mentions. The author no longer supplies a `Bindings` argument. |
| `dealDamageOwnPower` | The source's self-subject introduction before the amount is read. The author no longer supplies a `Bindings` argument. |
| `agentRef` | The agent's own introduction, including singularization of distributive agents. An agent introducing nothing is re-read directly. |
| `itsOther` / `sameWindow` | The first stat modification's subject and amount introductions, in the same order as `StaticSpec.intro`. An originally empty introduction uses the whole context. `getsPt` no longer uses it: "gets +P/+T" is the one node `StaticSpec.ptModification`, which checks its subject once and its toughness delta after its power delta's introductions; the bench's hand-written two-modification clauses still read their toughness half through it. |
| `ownSubject` | The shared subject's introduction; an empty owned introduction remains empty and cannot capture an outer antecedent. |
| `lookedCards` | The library slice's introduction, including its possessor's mentions. |
| `attachToIt` / `requireBlockIt` | The containing context outside the current noun's introduction. Ambiguity within that outer context remains an error. |
| `aTheirChoice` | The immediately preceding chooser mention. The macro requires that grammatical caller position; the fixed one-binding window is independent of the outer context. |
| `comparesOwnStat` | The one subject binding that `Predicate.check` places immediately before the measure, read through its `member` argument, `ownMember` unless written. Its fixed internal depth is independent of outer context and letters. |

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

## The performer and the handoff

`NounPhrase.actor` is whoever performs the instruction it is part of: the
controller [CR#109.5] unless an enclosing `Action.act` names another
player or a permanent. `act performer body` takes a player, or an object whose
zone is the battlefield, a permanent [CR#110.1]; any other object is refused
`handoffPerformer`. Its body is checked in `actorCtx`. A handoff frame records
the performer's structural facts (the captured-subject `NounShape`, its kind
among them); the innermost frame decides, and `actor` is checked as that
kind, so inside a handoff to a permanent `actor` is the permanent, and
`controllerOf(actor)` is a player. Outside every handoff, and inside
`act you`, `actor` reads exactly as `you` in every rule.

No instruction writes its performer: the agent fields are gone
(`semantics-v2-drop-agent-fields`), and with them `choose`'s chooser and an
added turn's taker (`semantics-v2-chooser-and-taker-fields`). A choice is
judged as the performer's: its chosen phrase must be `agentChoosable`, and
"starting with you" needs a distributive performer. An added turn is taken by
the performer [CR#500.7]; an added phase or step is added to the current turn
[CR#500.8,500.9]. Each rule that read a field reads the performer in
context, `performerOf perf` (`Check/Abilities.lean`): the actor, or the group a
handoff hands this very instruction. A handoff to a group (`each opponent`)
whose body is one instruction that has a performer of its own
(`Action.performed`: a deed, a life change, a draw, a choice, a vote, an
offer, …) hands it to each member as that instruction's performer:
`Action.checkWith`/`profileWith` check and profile the body with the group
as `perf`, under the handoff frame alone (`performerFrame`), exactly as the
deleted explicit plural agent was judged (`enactKeepsOuter`, `doesProfile`,
`chosenIntroBy`, one `voteHeld`). A group handoff over any other body, and every
handoff to one performer, keeps the frame-and-pluralize reading below.

A handoff to `this` ("this creature explores") puts the permanent in view for
its body (`performerInView`), since `this` introduces no binding: "that
permanent" in an inner `act(controllerOf(actor), …)` reads it. A handed-to
target is already in view through its own introduction. Leaving the handoff
drops that binding with the frame (`leaveHandoff`), so a handoff to `this`
publishes what a handoff to `you` does: only the body's mentions.

An enacted deed's performer (`enactPerformer`: the group a handoff hands it,
else the actor where its row names a performer) is checked at its row's agent
kind (`deedAgentKind`, refused `kindMismatch`). A deed whose row names no
performer ("fight" [CR#701.14a]) records none written with no handoff; that a
handoff may not hand it one is a game rule the checker no longer asserts
(`enactAgentOk`, retired by `lean-checker-sweep-game-rules`). Explore, endure, adapt, harness and
monstrosity are performed by a permanent [CR#701.44a,701.63a], the other
wrapped keyword actions by a player. A permanent-performed deed whose
performer is the controller, written with no handoff to a permanent (printed
"Adapt N" [CR#701.46a]), is performed by the source permanent
(`enactDefaultsToThis`): its body is checked in a handoff to it, so `actor`
there is "this permanent". Inside a handoff to another player there is no such
default, and the player is refused at the deed's agent kind. An event's performer is written in the same
kind (`deedAgentKind`): "whenever a creature you control explores".

Rules that ask whether a phrase is the controller read `actor` in its handoff
(`actorView`, `isYouIn`). The self-exchange and opponent's-library checks that
read them are retired (`lean-checker-sweep-game-rules`).
A cost may hand its instruction to another player, and its handoff twin gave the
same refusals as the explicit-agent spelling while that existed. Who pays a cost
(the retired `payAgrees`, `costPaidByYou` and `keywordCostPaidByYou`) is not
checked (`lean-checker-sweep-costs`); the `Proofs/Actor.lean` pins that asserted
those refusals now read the payer's handoff through `isYouIn` in the context
the checker builds (`actorCtx`, `ownPerformerCtx`, `captureBindings`). The actor is
singular, so an `enact`, an `offer` or a choice with no group handed to it takes
its one-player path; distribution belongs to the handoff. `sameKnownZone` does not equate two
`actor` zones, since nested handoffs can name different players.

What a handoff publishes depends on its performer's number. A singular
performer publishes the body's own profile without the handoff frame, so a
targeted player stays bound below the body's mentions. A group performer
(`each opponent`) publishes what each member's body introduced as a group,
pluralized as `doForEach` publishes it, with one exception: a singular
amount outcome the body introduced stays one total and is the handoff's
deed, as the explicit plural agent leaves it, so "each opponent loses 1 life
and you gain life equal to the total life lost this way" [CR#702.101a] reads
it through `thatMuch`. `doForEach` keeps every outcome plural.

An object binding records its Binding Possessor (`Payload.object`'s last field,
`Possessor`): "you" or another player, relative to the controller [CR#109.5],
or unknown. A description seeds it (`Predicate.seedPossessor`) from a
possessed hand, graveyard or library (`.in_ (handOf p)` [CR#400.1,400.3]) and from
a controller predicate (`.hasPossessor .controller p`, and its negation on
`you` [CR#108.4]), with `actor` read in its handoff
(`NounPhrase.possessorSide`), so a binding made inside a handoff keeps its
meaning outside it. A move between one player's own zones keeps it; a move to
or from a shared zone and a gain of control leave it unknown
(`keptPossessor` [CR#110.2]). A deed whose facts row marks its patient as the
performer's own (`ActFacts.ownPatient`: sacrifice [CR#701.21a], discard
[CR#701.9a]) reads the patient's possessor (`NounPhrase.possessor`), the
conjunct of an indefinite patient and the binding of a definite referent
alike, and is refused `patientPossessor` only when that possessor is known to
be another player than the performer (`enactPatientPossessorOk`); an unknown
possessor is not refused. This is a reference agreement, not a game rule about
zones. `Proofs/Possessor.lean` pins both deeds in both forms with their
passing twins. Every pattern over `Payload.object` names the possessor: the
field defaults to unknown, and a pattern that leaves it out matches only the
unknown possessor. `Proofs/Actor.lean` pins the handoff
cards, the bindings each kind of performer leaves, nested handoffs, nested
abilities, captured `actor`, every cost the bench hands to another player, the
group handoff's totals, and the handoff to a permanent with the default to the
source permanent. The bench's group handoffs over one instruction (votes,
distributed choices and sacrifices, drains) pin the group-performer reading.
These are checker guarantees, not an execution proof.

## Binding identity and a keyword body's own scope

A registry macro's referent parameter is bound at the body's first mention and
read back by its Binding Identity at every later one (`docs/decisions/semantics-v2.md`
§7, ruling 2026-10-06; §12.2). `NounPhrase.firstMention id phrase` is checked
and introduced as `phrase` is, and records the binding `phrase` introduced or
resolved under `id` (`Binding.idents`, `NounResult.recordIdentity`).
`NounPhrase.laterMention id phrase` reads the most recent binding recorded
under `id` (`identityAddress`): it selects nothing and searches no window, so a
target argument is one targeting however often the body mentions it, and a
binding pushed between the mentions does not move the read. The identity is a
name, never a stack position. Its `phrase` answers only the structural readers
(`isYou`, `selfDefinedOk`, `kind?`, `plur`, …), as a captured parameter's
`NounShape` does. A phrase that records nothing ("you", "this") is read again
at a later mention; a later mention whose phrase would introduce, with no
record, is refused `bindingIdentity`, never read as a second selection. Updates
in place (`moveIntro`, `refine`, `setBindingAt`) keep a binding's identities.

A keyword action body is read in its own Reference Scope:
`Action.ownScope scope body`, which the loader writes inside the action's
`enact` wrapper, masks every mention of the calling text but the performer the
innermost handoff names: its frame and the bindings it put in view
(`Payload.actor`'s width, `enterOwnScope`). The body's parameters are written
`inCaller scope`; a caller view of an own scope lifts that mask for the
parameter, masks the body's own mentions and the scope's frame, so a parameter
forwarded into a nested body reads the next scope out (`enterCaller`). Leaving
the scope keeps what the body introduced and every update (`leaveOwnScope`). A
caller view with no frame open is the phrase's own view: the structural reads of
an `enact` wrapper (`enactPatient`, `Action.unscoped` for the provenance a
deed stamps) see a parameter where the call writes it. A keyword ability body is
checked from an empty context, its own scope. `Proofs/BindingIdentity.lean` pins
the identity reads, fight, regenerate, the looks of scry, surveil and fateseal
with the slice's number, and connive; `Proofs/Actor.lean` pins the scope (the
literal amass after a destroy, refused bare and checked in its scope, Azog's
amount read in the caller's view, a body that cannot read its caller, a keyword
ability that cannot read its card). These are checker guarantees, not an
execution proof.

## Scheduling fields

For `skipUntap`, `skipPart`, and `addTurnPart`, let `bs`
be the input context. The subject (for `skipPart` and `addTurnPart`, the
performer in context) is checked in `bs`. Its amount is checked in
`nomIntro bs subject`. Part, anchor, and following-part fields carry no references
and are not checked (`lean-checker-sweep-timing`).
No scheduling field introduces an enclosed scope. The output is
`Amount.intro afterSubject amount`; an added turn additionally puts `turnRefB`
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
| `enact` | The performer in context (`enactPerformer`) establishes `enactCtx`; a group a handoff hands the deed checks `enactKeepsOuter`. Tag/body trust is established at the macro authoring boundary above. |
| `act` (handoff) | The body is checked in `actorCtx`: the player's `agentIntro` under a handoff frame that `actor` reads. A singular performer exports the body's profile outside the frame, so a targeted performer stays published; a distributive performer follows `doForEach` (preserve outer bindings, pluralize exported local mentions), unless the body is one instruction with a performer of its own, which takes the group as that performer (`performerFrame`). |
| Delayed, reflexive and `triggerThisWay` clauses | Use `delayedCtx`, `reflexCtx` or `thisWayCtx`. The enclosed body's private mentions do not escape as ordinary sequential mentions. |
| Replacement and held clauses | Use `replacedCtx` or the held body's announcements. Do not export the event as already completed. |
| Event alternatives and joined headers | Check arms from the common input; `sharedCtx`/`joinedCtx` compute what the combined header can expose. |
| Activated abilities | Check the cost after dropping its local X; the body sees only public cost mentions. Timing, guard and activator clauses use the ability's enclosing context. |
| Granted/keyword/token/emblem abilities | A nested ability has its own scope. Keyword bodies, token characteristic text and emblem abilities use the empty context; grant checks use the grant's enclosing context. A nested ability's performer is its own controller: inside a handoff, `ownPerformerCtx` adds a frame back to "you" before its cost, header or body is checked. |
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
for stat and deed checks. Contradiction checks over attachment words can use
any supporting type within one conjunctive alternative. Attachment profiles collect every type
refinement, including when seeding or moving their referent.

Movement retains the remembered facts and changes the carrier separately.
A battlefield type word then stops matching, while a correspondingly refined
card word can match the card in exile [CR#109.2,109.2a,110.1]. This is evidence
for reference resolution, not a simulation of continuous characteristic changes.

`Proofs/TypeEvidence.lean` proves `pureTypePermutation` for arbitrary lists of
pure card-type modifiers, including duplicates and the empty list. Its concrete
pins cover both type words, missing types, common versus alternative evidence,
joined halves, attachment profiles, and movement. This does not assert
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
| `DeedComplement` | `DeonticPatient.check` checks each counterpart; `deonticPatientOk` checks its deed membership, its kind against the deed's other role, and that it is not the subject read back. |
| `DeonticPatient` | The old deed/role indices were parameters shared by all constructors. `deonticPatientOk` at `StaticSpec.deonticRule` checks a counterpart's kind and that it is not the subject read back; which deeds a patient form, a bound, an "as though" or a play rider goes with, and a counterpart's class, card types and zone, are not checked (`lean-checker-sweep-costs`). Child phrases use `DeonticPatient.check`. |
| `DamageScope` | `DamageScope.check` in [AbilityRules.lean](Semantics/Check/AbilityRules.lean) checks references and expected child kinds in the supplied context. |
| `DamageAgent` | `DamageAgent.check` in [AbilityRules.lean](Semantics/Check/AbilityRules.lean) checks references and expected child kinds in the supplied context. |
| `Unpreventable` | `Unpreventable.check` in [AbilityRules.lean](Semantics/Check/AbilityRules.lean) checks references and expected child kinds in the supplied context. |
| `PreventCut` | `PreventCut.check` in [AbilityRules.lean](Semantics/Check/AbilityRules.lean) checks references and expected child kinds in the supplied context. |
| `DamageScale` | `DamageScale.check` in [AbilityRules.lean](Semantics/Check/AbilityRules.lean) checks references and expected child kinds in the supplied context. |
| `DividedVerb` | `DividedVerb.check` in [AbilityRules.lean](Semantics/Check/AbilityRules.lean) checks references and expected child kinds in the supplied context. |
| `DividedTakes` | `Action.distribute` checks the recipient kind (`objectOrPlayerCheck`) or object kind according to the closed `DividedVerb` constructor; which objects damage reaches is not checked (`lean-checker-sweep-zone`). |
| `CtrlOverrideOk` | `MoveRider.under` checks player kind. Retired by `lean-checker-sweep-zone` (§7 charter ruling 2026-10-06): the single-controller demand is not checked. |
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
| `MoveRider` | `MoveRider.check` and `MoveRider.checkAll` in [AbilityRules.lean](Semantics/Check/AbilityRules.lean) check references and expected child kinds in the supplied context; an Entry Rider or a Pile Placement whose kind its destination does not allow is refused `riderKind` (owner ruling 2026-10-08, Q28–Q29). |
| `Cost` | `Cost.check` in [AbilityRules.lean](Semantics/Check/AbilityRules.lean) checks references and expected child kinds in the supplied context. |
| `ManaRider` | `ManaRider.check` in [AbilityRules.lean](Semantics/Check/AbilityRules.lean) checks references and expected child kinds in the supplied context. |
| `CopyExcept` | `CopyExcept.check` in [AbilityRules.lean](Semantics/Check/AbilityRules.lean) checks references and expected child kinds in the supplied context. |
| `RollRow` | `Action.checkRows` checks the range and each row body; row bodies start at the same outer bindings. |
| `Action` | `Action.check` in [AbilityRules.lean](Semantics/Check/AbilityRules.lean) checks references and expected child kinds in the supplied context. |
| `Instructions` | The length is the actual list length; `Action.check` checks nonemptiness and `checkSeq` threads `Action.intro`. No independent length value remains. |
| `SimInstructions` | The length is the actual list length; nonemptiness is checked and `checkSim` threads `Action.annIntro`, keeping simultaneous outcomes out of sibling inputs. |
| `KeywordParam` | `KeywordParam.check` in [AbilityRules.lean](Semantics/Check/AbilityRules.lean) checks references and expected child kinds in the supplied context. |
| `AbilityLost` | `AbilityLost.checkAll` checks each carried keyword term or chosen-ability reference. |
| `AbilityAt` | `Ability.check` dispatches each ability constructor, its expected phrase kinds, and its explicit local contexts. |
| `StaticParts` | The actual list supplies length; `conjunction` checks nonemptiness and `checkParts` checks noncoordination and threads `StaticSpec.intro`. |
| `CostSeq` | The actual list supplies length; `Cost.compound` checks nonemptiness and `Cost.checkSeq` threads `Cost.intro`. |
| `AddedPayment` | Retired by `lean-checker-sweep-costs` (§7 charter ruling 2026-10-06): `StaticSpec.addedCost` and an alternative cost no longer require `Cost.offBattlefield`; the card-class rule (`classAbilityOk`) still reads it. |
| `AbilitySeq` | `Ability.checkText` threads the choices and letters introduced by earlier lines; nested ability lists use `Ability.checkAll` at their enclosing context. |

### Events

| Reference datatype | Lean duty |
|---|---|
| `LookbackSubject` | `LookbackClause.check` binds the described participant, and the shared event checker validates each use. |
| `LookbackComplement` | Ordinary `GameEvent` participant checks; there is no separate complement-kind table. |
| `ChoiceMode` | `ChoiceMode.check` in [Phrase.lean](Semantics/Check/Phrase.lean) checks references and expected child kinds in the supplied context. |
| `Possessable` | Retired by `lean-checker-sweep-zone` (§7 charter ruling 2026-10-06): a possessed shared zone is not refused. `Zone.possessable` still decides which moves keep a binding's possessor. |

### Phrase

| Reference datatype | Lean duty |
|---|---|
| `ChoiceInScope` | `countParts` in `Predicate.notChosen` and `Condition.choseThisWay` requires a compatible standing choice. |
| `ColorTerm` | `ColorTerm.check` in [Phrase.lean](Semantics/Check/Phrase.lean) checks references and expected child kinds in the supplied context. |
| `LibPlace` | `LibraryPlace.check`; the place's arrangement and ordinal fit are retired by `lean-checker-sweep-zone`. |
| `Location` | `Location.check` in [AbilityRules.lean](Semantics/Check/AbilityRules.lean) checks references and expected child kinds in the supplied context: a zone's owner is a player, a place from a pile's end has a nonzero ordinal, a made pile is a written pile mention, the cards exiled with a source name a link source, and a disjunction has at least two disjuncts (a search's disjuncts name distinct zones). |
| `NameSource` | `NameSource.check` in [Phrase.lean](Semantics/Check/Phrase.lean) checks references and expected child kinds in the supplied context. |
| `ChoiceDomain` | `sortedDomainCheck` checks both domain contents and `ChoiceDomain.sort` at `qualityNoun`, `ofYourChoice`, `entersChoice`, and `attachChoice`. |
| `EventComplement` | Retired in favor of event participant, source, destination, and locus fields. |
| `ComplementWritten` | Required event fields cannot be omitted. An action that declares a locus requires that locus in the shared event checker. |
| `LookbackClause` | The shared checker validates the carried event under a locally bound participant of the host kind. |
| `Predicate` | `Predicate.check` receives an expected kind, checks every child, and uses `kindCheck`; conjunction children share a scope, joined alternatives receive their own inferred kinds. |
| `DetPhrase` | `DetPhrase.check` in [Phrase.lean](Semantics/Check/Phrase.lean) checks references and expected child kinds in the supplied context. |
| `RolesOk` | `rolesOk` in `NounPhrase.oneEachOf`, alongside per-role `Predicate.checkAll` and object-kind checks. |
| `Noun` | `NounPhrase.check` receives the expected kind, applies `ctxCheck`, and checks references in the supplied bindings. Authorable offset references remain owned by `lean-grammatical-reference-scopes`. |
| `Amount` | `Amount.check` in [Phrase.lean](Semantics/Check/Phrase.lean) checks references and expected child kinds in the supplied context. |
| `Quantity` | `Quantity.check` in [Phrase.lean](Semantics/Check/Phrase.lean) checks references and expected child kinds in the supplied context. |
| `NonZeroQ` | `Quantity.nonZero` in the consumers that require nonzero cardinality; zero remains allowed in other quantity slots. |
| `SliceCount` | `SliceCount.check` in [Phrase.lean](Semantics/Check/Phrase.lean) checks references and expected child kinds in the supplied context. |
| `ComplementAnchor` | `NounPhrase.anchorPhrase` and singularity in `Predicate.otherThan`. |
| `LinkSource` | `NounPhrase.linkSource` in `Predicate.exiledWith`. |
| `PileMention` | `NounPhrase.pileMention` plus noun checking in `Predicate.inPile`. |
| `PaidSubject` | Object-kind checking in `Amount.paid`. Retired by `lean-checker-sweep-zone` (§7 charter ruling 2026-10-06): the stack-zone demand is not checked. |
| `DestOk` | Retired by `lean-checker-sweep-zone` (§7 charter ruling 2026-10-06): a move's destination shape is not checked. |
| `FlipScope` | `FlipScope.check` in [Phrase.lean](Semantics/Check/Phrase.lean) checks references and expected child kinds in the supplied context. |
| `IgnoredOutcomes` | `IgnoredOutcomes.check` in [Phrase.lean](Semantics/Check/Phrase.lean) checks references and expected child kinds in the supplied context. |
| `Ballot` | `Ballot.check` in [Phrase.lean](Semantics/Check/Phrase.lean) checks references and expected child kinds in the supplied context. |
| `CostSubject` | Retired by `lean-checker-sweep-zone` (§7 charter ruling 2026-10-06): a cost change's subject is not required to be on the stack. |
| `StackActOn` | Retired by `lean-checker-sweep-zone` (§7 charter ruling 2026-10-06): retargeting and copying do not demand a stack object, and counter's enacted move checks no source zone. |
| `Condition` | `Condition.check` in [Phrase.lean](Semantics/Check/Phrase.lean) checks references and expected child kinds in the supplied context. |
| `MarkingOk` | `markingOk` in `StaticSpec.conditional`. |
| `TokenPhrase` | `NounPhrase.tokenPhrase` plus object-kind checking in the token-creation event. |
| `Exposed` | `Exposed.check` in [Phrase.lean](Semantics/Check/Phrase.lean) checks references and expected child kinds in the supplied context. |
| `VisibleThing` | `VisibleThing.check` in [Phrase.lean](Semantics/Check/Phrase.lean) checks references and expected child kinds in the supplied context. |
| `DamageRecipient` | `objectOrPlayerCheck` in damage instructions, `DamagePatient.check` and the damage-result rule: the recipient's kind alone. Which objects can be dealt damage [CR#120.1a] and where they are are not checked (`lean-checker-sweep-zone`). |
| `StatusHolder` | `Action.setStatus` requires the object kind; the battlefield zone and the settable status value are not checked (`lean-checker-sweep-zone`). |
| `NotAnAbility` | Retired at movement: the action-fold ticket explicitly corrects this false blanket prohibition; ending a turn or combat can exile stack abilities [CR#724.1b,724.2b]. |
| `DiscardOk` | Discard is an enacted move. Retired by `lean-checker-sweep-zone` (§7 charter ruling 2026-10-06): its source zone is not checked. The macro authoring boundary above excludes arbitrary authored tag/body combinations. |
| `Movable` | `Action.move` checks the moved kind against `object ∨ pile` (`kindLte`), so objects (including abilities) and piles move and players and quality values are refused. The destination's riders, arrangement and card types are not checked (`lean-checker-sweep-zone`). |

### Triggers

| Reference datatype | Lean duty |
|---|---|
| `ManaTypeTerm` | `ManaTypeTerm.check` in [Phrase.lean](Semantics/Check/Phrase.lean) checks references and expected child kinds in the supplied context. |
| `Duration` | `Duration.check` in [Phrase.lean](Semantics/Check/Phrase.lean) checks its boundary events and condition; a boundary that names whose turn names one definite player (`durationPossessor`). |
| `TurnRef` | `TurnRef.check` in [Phrase.lean](Semantics/Check/Phrase.lean): a named player is a player, and `that` reads back exactly one turn an earlier phrase introduced (`turnAntecedent`). |
| `UsageLimit` | `UsageLimit.check` in [Phrase.lean](Semantics/Check/Phrase.lean) checks the count's amount; its period names no referent. |
| `AttackDefender` | `AttackDefender.check` in [Phrase.lean](Semantics/Check/Phrase.lean) checks references and expected child kinds in the supplied context. |
| `DamagePatient` | `DamagePatient.check` in [Phrase.lean](Semantics/Check/Phrase.lean) checks references and expected child kinds in the supplied context. |
| `Door` | `Door.check` in [Phrase.lean](Semantics/Check/Phrase.lean) checks references and expected child kinds in the supplied context. |
| `RollWatch` | `RollWatch.check` in [Phrase.lean](Semantics/Check/Phrase.lean) checks references and expected child kinds in the supplied context. |
| `CrimeSubject` | `GameEvent.commitsCrime` checks a player subject and singularity. |
| `GameEvent` | `GameEvent.check` in [Phrase.lean](Semantics/Check/Phrase.lean) checks references and expected child kinds in the supplied context. |
| `Causing` | `Causing.check` in [Phrase.lean](Semantics/Check/Phrase.lean) checks references and expected child kinds in the supplied context. |
| `TriggerWindow` | Merged into `Timing.inPart`; `Timing.check` checks the possessor's references. Which turn parts and possessors a window admits is not checked (`lean-checker-sweep-timing`). |
| `Timing` | `Timing.check` in [Phrase.lean](Semantics/Check/Phrase.lean) checks references and kinds only: whose turn is a player's, and the events, conditions and turn references it names. |
| `AltEvent` | `GameEvent.check` checks each alternative event. The old header-word index is gone; the event determines its own event class. Header nontargeting is retired by `lean-checker-sweep-game-rules` (§7 charter ruling 2026-10-06). |
| `JoinedHeader` | `JoinedHeader.check` in [Phrase.lean](Semantics/Check/Phrase.lean) checks references and expected child kinds in the supplied context. |

### Words

| Reference datatype | Lean duty |
|---|---|
| `OptOk` | Optional consumers use their `Opt*.check` or `map`/`getD` branches; absence introduces no payload to check. |
| `Joins` | `joinKinds`, `Predicate.kindOfAll`, and `NounPhrase.kind?` derive the resulting kind; there is no independently supplied result index. |
| `KnownAct` | `knownAct`/`knownActs` at deed consumers read the deed facts; the macro definition is trusted to supply the corresponding expansion. |
| `Payload` | `Payload.kind` derives the kind from the constructor; `Binding` has no separate kind field that can disagree. The checker-private `parameterFrame` and `actor` frames are not referents; windows, caller masks, forgetting and `countOnes` pass over handoff frames. |
| `HeadTy` | `NounPhrase.ty` and `Payload.ty` derive lists of known card-type facts; joined kinds retain evidence per half. |
| `DieSides` | `DieSides.check` in [AbilityRules.lean](Semantics/Check/AbilityRules.lean) checks references and expected child kinds in the supplied context. |
| `Targetable` | `Kind.targetable` at `DetPhrase.check`, copying and target consumers. |
| `Targeter` | `Kind.targeter` in `Predicate.check` and `DeonticPatient.check`. |
| `Phrasal` | `Kind.phrasal` at copying, iteration and noun consumers. |
| `KnownKeyword` | `knownKeyword`, `KeywordTerm.known`, and `keywordParamFits` at keyword consumers. |
| `BasicLandType` | The old `basicLandLine` macro has no Lean counterpart. No caller can supply a proof-erased list to that removed API; `SubtypeSpace.basicLand` is a closed constructor. |
| `BasicLandTypes` | Same removed `basicLandLine` API as `BasicLandType`; a printed face's subtype validation remains `subsFitLine` (the card frame). A token's or an edit's subtypes are not checked against its types (`lean-checker-sweep-game-rules`). |
| `Delta` | The type parameter survives as `Delta α`; it was not erased. `Delta Amount` consumers check the carried amount. |
| `GivingWarrant` | Retired with the conferrer tag (2026-09-07): which expansion confers a designation is not tracked. `Action.gainDesignation` checks the designation's own row: its scope and holder. Whether an instruction may confer it (the row's `effectful`) is not checked (`lean-checker-sweep-game-rules`). |
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
remove operations require values. Both lists are checked for term shape only
(`CharacteristicEdit.shape`): an empty list is `nonEmpty`, a type line that
writes nothing or adds or removes no value is `lineNonEmpty`, an added or
removed empty color list is `nonEmpty`, a list naming a value twice is
`distinct`, and a chosen quality is read as a chosen one (`qualityRead`). Which
edits fit their subject, op and zone (subtype hosting, a type already held,
"still a" retention, a name or base stat added or removed, a granted spell
ability) are game rules the checker does not assert
(`lean-checker-sweep-game-rules`); the aggregate `becomesOk` refusal is retired.
No edit requires its subject or source to be on the battlefield
(`lean-checker-sweep-zone`).

Numeric edits thread a scope local to their block. Other edit payloads read
the incoming scope; nested written abilities remain closed. The subject of
an ordinary change is not published before its edits. Ability-removal
selectors check no zone for their subject: specified written abilities
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

### A keyword definition split by card class

A keyword that means one ability on a permanent and another on an instant or
sorcery spell [CR#702.55a,702.131a,702.131b,702.174b] writes one definition
holding both halves, each guarded by the card's class with the ordinary
conditional forms: a `doIf` with no otherwise-branch, a trigger's intervening
condition, or a conditional (not `unless`) static ability, whose condition is
`.matches .this p` with `p` implying a Card Class (`Predicate.impliedClass`:
an instant or sorcery type implies the instant-or-sorcery class, a permanent
type the permanent class [CR#110.4a]; `permanentCard` and `instantOrSorcery`
are the helpers). `semantics-v2.md` §7, ruling 2026-10-06 ("Gift and the
additional-cost keywords share one mechanism"); there is no selector device.

- A spell ability [CR#113.3a] is a fourth `AbilityCategory` (`.spell`), which a
  registry row's `definition` may declare; the facts generator reads it off a
  `spell(…)` body part.
- `KeywordFacts.bodyPartFits` checks each part against the row: its category
  is declared; a class guard names a class the row admits
  (`onPermanentCard` / `onInstantOrSorceryCard`), so a guard on a class the
  keyword's card can never have is refused; a spell part on a keyword that
  also appears on permanents is under the instant-or-sorcery guard; and the
  part's form admits its own declared regime (below).
- Each half is checked as a term on every card, with no class context
  (`Ability.checkAll [] body`). Against the card's class, `classAbilityOk`
  checks the parts guarded by that class and skips the parts guarded by the
  other one, which never apply to the card. Unguarded parts are not checked
  against the class, as before.
- Pins: `Semantics/Proofs/KeywordByCardClass.lean`, whose definition-fit pins
  hold a gift-shaped row of their own (written before gift was bodied), and
  `Semantics/Proofs/GiftVariants.lean`, which checks gift's spelled variants
  against the registry's own gift row on both card classes.

### Each component of a keyword definition carries its own regime

A keyword definition is a list of components, each carrying the stack regime
the rules give it (`ComponentFacts`: a category and an `Option StackRegime`,
`none` where it functions off the stack); `semantics-v2.md` §7, ruling
2026-10-06 ("Gift and the additional-cost keywords share one mechanism").
Kicker is the one-component case [CR#702.33a]; offspring and squad pair a
cost at casting with an enters trigger [CR#702.175a,702.157a]; casualty's
second ability is a cast trigger [CR#702.153a]; gift's is an enters trigger on
a permanent and a spell ability on an instant or sorcery
[CR#702.174a,702.174b]. Nothing is exempt: every part of a written definition
must be a declared component of its category whose regime its form admits
(`Ability.regimeFits`).

- **Form-determined.** A triggered ability keys on its event's regime
  (`bodyEventRegime`: a cast event at casting, any other off the stack); a
  spell ability is at resolution [CR#113.3a]; an additional cost or a cost
  change is at casting [CR#601.2f]; an activated ability and any other static
  ability are off the stack.
- **Open, declared.** A permission to cast the card (`altCost`, or a
  `deonticRule` permitting with a `play` rider) admits either at casting or off
  the stack, never at resolution: such an ability functions on the stack
  unless it states its zones, and the keyword's own rule says which
  [CR#113.6b,113.6d,702.34a,702.117a]. The facts generator takes such a
  component's regime from the overlay's `open_regimes` (surge, spectacle,
  web-slinging and impending declare at casting
  [CR#702.117a,702.137a,702.188a,702.176a]; the rest default off the stack)
  and refuses an entry for a component whose form determines its regime.
- **The row's `regime`** stays the keyword's own regime. Where it is granted
  to a spell [CR#113.6] it was read by `grantSubjectFits`, retired with
  `grantSubject` by `lean-checker-sweep-game-rules`. For a keyword whose
  definition is declared it is the summary of the components (the first
  stack regime among them, `ComponentFacts.summary`); the generator refuses a
  disagreeing overlay value and `Proofs/Tables`
  (`everyDefinedKeywordRegimeIsItsComponentsSummary`) pins it.
- A part whose declared regime does not fit its form is a term-shape matter
  (charter, §7 2026-10-06), not a game-rule refusal: it is refused as
  `keywordBodyFits`, as before.
- Pins: `Semantics/Proofs/KeywordDefinitionRegimes.lean`. Impending's
  end-step trigger now fits its own component; its twin under the old single
  casting regime is refused.

### The player chosen as a cost was paid

Gift's first ability is a cost, "As an additional cost to cast this spell, you
may choose an opponent" [CR#702.174a], and its second hands the effect to the
chosen player [CR#702.174b,702.174d..702.174i]. The two are linked
[CR#607.2d,607.2i], as kicker's cost and its "was kicked" readback are
[CR#702.33e], and the second
reads the first through the same paid-cost facets: `LinkedFacet.chosenPlayer
which`, "the player chosen as [which] was paid", keyed by `LinkSource` and
written `.byKeyword (.named "gift")` (`semantics-v2.md` §7, ruling 2026-10-06,
"Gift and the additional-cost keywords share one mechanism"). There is no
general linked-ability binding.

- The facet is a player, so it is read by `NounPhrase.paid facet subject`
  (`chosenAsPaid`); `Amount.paid` reads the amount facets. Each refuses the
  other's facet (`linkedFacetKind`): a term-shape check.
- It resolves as kicker's readback does: the cost is named in the registry
  (`linkedFacetNamed`); where the subject is (the `paidSubject` stack demand) is
  no longer checked (`lean-checker-sweep-zone`). It adds one reference check: paying the named cost chooses
  a player, the row's `costChoosesPlayer` column (gift's alone), else
  `linkedChoiceInScope which`, since the linked ability has no choice to read
  [CR#607.2d]. "The additional cost" and "the alternative cost" declare no
  choice. Neither check asserts a game rule (§7, "The checker's charter").
- A bare "the chosen player" (`chosenPlayer`, `.chosenPlayer .theChoice`) still
  reads only a choice in its own ability, and is refused with
  `choiceRef theChoice player 0` where there is none.
- Pins: `Semantics/Proofs/LinkedChoiceReadback.lean`, including both
  class-guarded halves of a gift-shaped definition.


### Linked Facets of a Link Source, and declared Relations

`PaidFacet` and `PaidCostName` are `LinkedFacet` and `LinkSource`
(`semantics-v2.md` §7, ruling 2026-10-09, "A linked ability reads back a
Linked Facet of a Link Source"). A Link Source is also a printed ability named
by its deed, `LinkSource.byDeed deed` ("the exiled card" [CR#607.2a]), and two
facets join the five:

- `LinkedFacet.objects which deed plurality`, the objects `deed` acted on
  ("the sacrificed creature", "creatures it devoured" [CR#702.82b]), read by
  `NounPhrase.paid` at object kind and the written number. It is named when
  its source is (`linkedFacetNamed`), the deed is declared and acts on
  objects, and the source performs that deed (`LinkSource.performs`): a
  `byDeed` source performs only its own deed, and a keyword ability's own deed
  is performed only by that keyword's source. Anything else is refused
  `linkedFacetNamed`: there is nothing to read back.
- `LinkedFacet.taken which`, whether the choice `which` offered was taken
  ("if tribute wasn't paid" [CR#702.104b]), an amount facet.

A Relation (`RelationLabel`) is declared as a designation is, under
`plugins_v2/builtin/macros/relations/`, as `Definition.relation label left
right`; `cargo xtask facts` writes `relationTable`. `Action.relate`,
`Predicate.related` and `NounPhrase.counterpart` check each side at the kind
the declaration gives it, so a side of the wrong kind is refused
`kindMismatch`, and an undeclared label `knownRelation`. These are kind and
reference checks; when a relation ends [CR#702.95e,702.99c] is the engine's.
Pins: `Semantics/Proofs/LinkedReadback.lean`.

### The ability printed with a keyword, and its qualifiers

The ability a keyword is printed with ("Boast — [Cost]: [Effect]") is the
keyword's parameter, `KeywordParam.ability`, and the restriction the keyword
adds is a qualifier on it, `Ability.qualified ability timing limit guard
from_` (`semantics-v2.md` §7, ruling 2026-10-09) [CR#113.10a,113.6b]. A
qualified ability keeps the wrapped ability's category, stack regime and class
guard, so a keyword definition written as a qualifier over its argument is
checked against the keyword's row by the printed ability's category; the
row's `definition` names the categories the CR entry admits (solved: static,
triggered and activated [CR#702.169a]).

- The guard reads "as long as", "this ability triggers only if" or "activate
  only if" by the wrapped ability's category [CR#702.169b..702.169d]; the
  checker checks it, the timing and the limit for kinds and references only.
- A limit is admitted on an activated ability alone (`qualifiedLimit`): a
  usage limit restricts activation [CR#602.5b]. A triggered ability's limit
  is written on its header.
- "Activate only once" is a limit over `Period.thisObject`, reset when the
  object changes zones [CR#602.5b,702.177a].
- `AbilitySelection.printedAfter keyword` selects the abilities printed below
  a keyword's line [CR#702.165a,702.165c], granted by
  `StaticSpec.abilityGrantSelected subject source selection`. A selection
  naming a keyword the card does not print is refused as a reference of the
  card's text (`printedAfterKeyword`, `printedAfterLaws`); the walk follows
  the forms a grant is written in, as `namesThisDoor` does.
- A cost shift on an ability ("this ability's cost is reduced", power-up
  [CR#702.193a]) is determined as the ability is activated, so it takes no
  stack regime [CR#602.2b,601.2f].
- `PlayerStat.speed` is a player's speed [CR#702.179a]; its game state is
  `engine-speed`'s.

Pins: `Semantics/Proofs/AbilityQualifiers.lean`.

### Movement destinations

`Action.move` requires a destination. Every move checks its destination's
references and the moved kind (an object or a pile), and publishes the stated
destination. The destination's shape, the arrangement's fit to the moved
plurality, the subject's card types and the riders' fit to the destination are
game rules the checker does not assert (`lean-checker-sweep-zone`). There is
no destinationless movement instruction.

User ruling (2026-09-06): Counter is one enactment of the Counter label around
stack-to-graveyard movement, uniformly for its spell or ability subject. This
supersedes the prior optional-destination design and its ability-specific branch.
The enacted movement no longer checks its stack source (`lean-checker-sweep-zone`). The rule defining
Counter supplies stack removal and a spell's graveyard destination [CR#701.6a];
this uniform expansion is the workbench's chosen representation, not an added
runtime lifetime model.

An ability binding stores its zone, initially the stack. Movement updates that
zone while retaining its ability identity and copy origin. A stack reference
resolves on actual stack evidence; counter and copy check no zone. Element bindings and
unions retain location evidence rather than restoring a hardcoded stack location.

`Action.clearDamage` removes all marked damage from a permanent. Its
subject must be an object; where it is is not checked, and it need not
currently be a creature [CR#120.6]. It publishes the ordinary subject reference and no damage
outcome. Regeneration composes this primitive with tapping and conditional
removal from combat inside a destruction replacement.


### Template and definition parameters and expanded actions

`semantic_template` and `semantic_definition` accept `capture NounPhrase`, `capture Amount`, and `splice T`
annotations alongside ordinary configuration arguments. A captured subject is
selected or resolved once; its repeated uses retain that identity while reading
current properties. A captured amount retains one value, while ordinary Amount
arguments substitute an expression that can be read repeatedly. The overloaded
capture operation derives its input kind from the annotated Lean type.

`withBindings` and `inCaller` are private expansion forms for NounPhrase,
Predicate, Amount, Quantity, Location, Condition, GameEvent, Cost, Action,
and StaticSpec. Bindings contain an ordered list of typed inputs. Captures occur
left to right at that node, including inside conditions and repetition. Two
identical argument expressions remain two selections. A nested macro forwards
an existing parameter reference; a fresh argument creates a fresh capture.
No macro registry or construction-time context object survives in the result.

`Window.parameter` and `Amount.parameter` carry generated scope and slot keys.
Authors use parameter names. The checker frame points to actual slots, including
outer inline slots, rather than payload-equal copies. It adds no ordinary
pronoun candidate. An inline subject remains a returned value when its private
frame closes. A noun result carries its full context, additions, returned
address, and value, so a macro can return any captured subject. Movement updates
that returned subject while the scope is live. Explicit resolved/type/marker
views remain attached to the returned value.

A captured `actor` is read as `actor` itself: it introduces nothing, and a frozen
parameter shape would record the controller even inside a handoff.

Captured numeric and noun reads retain the structural facts their consumers
need. Numeric facts include literal cardinality; noun facts include source and
payer roles, opponent restrictions, and two-party coordination. Checks that
inspect enacted moves, library slices, replacement nesting, self-reference, or
definition costs follow scopes and their capture obligations. Reusing a value introduces no second copy of its original
syntax. Inputs are still checked, including unused captures; a malformed
reference does not gain validity by being captured. Capture itself adds no
whole-body success requirement. Any required availability guard belongs in the
ordinary macro body.

A `splice` preserves caller references by temporarily masking mentions from the
receiving macro. Its body can explicitly accept macro-local subjects through
noun parameters. Temporary masks preserve live addresses and are distinct from
permanently forgotten mentions. Leaving caller scope restores visibility without
undoing updates or resurrecting forgotten mentions. Captures keep forgotten
slots private until their final scope closes; aliases share their identity
through filtering and branch joins.

Captures use actual introductions in the current context. An existing X in a
later target therefore introduces no second X and cannot shift an earlier
reference accidentally. The older kind-pattern windows above remain appropriate
only at their documented immediate boundaries. Collection-member binding,
aggregation, and the numeric-anaphora redesign remain separate work.

The bench's captured `fight` macro uses an ordinary creature-and-battlefield guard for both captured subjects,
then simultaneous damage at each subject's power [CR#701.14a..701.14c]. Group
subjects are rejected by the singular power read and per-member damage checks;
the checker no longer relies on a failed singular private reference to reject
them or borrows attacking-creature admissibility for fight. The `plugins_v2` declaration
binds each creature at its first mention instead and writes no guard: a
condition may not introduce its subject (`testSubject`), and the battlefield
demand is the rules' [CR#701.14b].

Regeneration installation creates a next-time-this-turn destruction replacement.
The Regenerate label belongs to its application: clear marked damage, have the
controller tap the permanent, and remove it from combat if applicable. A static
regeneration replacement uses that application repeatedly; it does not install
a further shield [CR#701.19a..701.19c].

Counter expands directly to `enact Counter (move subject graveyard [])`.
It reads the subject once and needs no operand capture or ability/spell branch.
The movement pre-state remains available to postposed conditions such as
Ertai's Trickery.
An alternative may also re-read the primary clause's stated amount, as in
Caustic Bronco's "otherwise ... that much"; this is a numeric reference, not
evidence that the primary branch actually happened.

Losing counters captures its player before checking the amount, then uses the
shared removal instruction and removed-counter outcome. Cost symbols, draw,
Room operations, and exchange remain unchanged in this pass.

## Rules tables

A rules table's row supplies its own subject: the scope predicate binds `this`,
so the row's condition, effect and conferred ability are checked in the empty
binding context, exactly as a printed face's text is. That a state-based row
targets nothing [CR#704.1,601.2c] is a game rule the checker no longer asserts:
the `nontarget` refusal is retired here as on a static ability
(`lean-checker-sweep-game-rules`).

Two representation boundaries are named rather than hidden. A conferral confers
an `Ability`, not a separate property type; whether it is one an effect could
grant (`grantable`) is retired with the grant rules. A damage result names a `CounterKind`,
and the counter registry's declared holder is what fixes the recipient's kind;
there is no second registry lookup.

A counter's kind is a name: `CounterKind.named` carries the counter
declaration's name verbatim (`p1p1Counter`, `flyingCounter`, `chargeCounter`),
and "counters with the same name or description are interchangeable"
[CR#122.1]. `CounterKind.check` is one registry lookup (`knownCounter`: does
the generated `counterFacts` table have a row of that name), and
`CounterKind.scope` reads the holder from the same row for every kind, a +X/+Y
counter [CR#122.1a] and a keyword counter [CR#122.1b] included; the generator
writes one row per counter declaration. A counter definition
(`Definition.counter`, `Rules.lean`) states what ONE counter confers, and the
model applies it once per counter held: the +1/+1 counter's conferral is
"+1/+1" (`boost`, one `ptModification`), a keyword counter's is its keyword
(`grants`). No conferral reads the count of its own kind. That reading is the
model's: `Definition.check` checks each conferral once in the empty binding
context and computes no multiplicity, so nothing in the checker depends on it
(`Proofs/Rules.lean`, `p1p1CounterCountMultipliedTwin`, which checks clean).

A definition's conferral is checked as the same syntax is inside a card
(`Conferral.check`, `Check/Rules.lean`). There is no bearer exemption: since
`lean-checker-sweep-zone` no stat change, a card's or a definition's, demands
that its subject be on the battlefield, so a +X/+Y counter's conferral needs no
narrowing to count on a creature card in any zone [CR#122.1a]. The +1/+1 and
-1/-1 definitions check clean (`p1p1CounterDefinitionZone`,
`m1m1CounterDefinitionZone`).

Every Registry Definition is gated: `cargo xtask lean definitions` re-emits
each counter, subtype and designation definition `plugins_v2/builtin` declares
and proves `Definition.check = []` by `decide`, as `lean check` does for cards
(`lean/README.md`). `Definition.check` has a rule for every constructor, an
exhaustive match, so there is no definition the gate reads and the checker
ignores; a declaration of a registry kind whose body does not read as its
family's `Definition` fails the read rather than being skipped.

A designation is a name too: `DesignationLabel.named` carries the designation
declaration's name verbatim (`goaded`, `monarch`, `leftHalfUnlocked`), or, for
a declaration that lists members, a member's (`day`, `night` [CR#731.1]); the
phrase a card prints ("the monarch") is the declaration's spelling and never
reaches the checker. `DesignationLabel.facts` is one registry lookup by name
(`findDesignation`), and every column (`checked`, `scope`, `holder`, the
zone, type and Room half) is read from that row; the generator writes one row
per designation, labelled with its name. `RoomHalf.designation` returns the
designation the table marks with that half, by name. A designation
declaration's bare name means its label, so it reads at a designation
position; its definition (`Definition.designation`) is a separate record the
registry holds under the name (ruling 2026-10-10).

The two lethal-damage state-based actions [CR#704.5g,704.5h] have no row: the
grammar projects no marked damage and has no "since state-based actions were
last checked" lookback. `Stat` likewise has no `defense` axis, so a battle's
printed defense [CR#310.4b] cannot be read; the battle rows here read defense
counters instead [CR#310.4c], which is what the rules define the battlefield
value as.

## Shared event patterns and static applicability

`GameEvent.zoneChange` names the subject, optional origin, optional destination,
and observation point. Its checking and historical mention order is subject,
origin, destination. An endpoint can refer to a player introduced by an earlier
field. A missing endpoint is unconstrained; empty origin sets, exclusion of every
origin, and movement from a single shared zone back into itself are refused.
Player-qualified exclusions are conservative when only a zone sort is known.

`dies`, `leaves`, `enters`, and `putInto` are authoring macros. Battlefield
and graveyard departure patterns observe before; ordinary arrivals observe after.
In particular, graveyard arrival from anywhere is not a battlefield-departure
pattern [CR#603.6c,603.10a]. The raw observation point retains the distinction.
Interception publishes departure location; a triggered continuation publishes
arrival location. Historical observation publishes mentions without moving current
references or manufacturing an event outcome. Source and destination possessors
are available in ordinary field order.

Damage event patterns share optional source and patient fields and require at
least one participant. Named active/passive authoring forms remain macros; no
semantic voice field is present. Entry and attachment choice specs likewise share
an occasion enum while retaining choice sort, domain, and disclosure.

`partScope` expands to conditional applicability with `Condition.duringPart`.
The condition reads the ordinary conditional context, including the spec's
subject and numeric definitions. A whole-turn condition is meaningful even with
no possessor. Any stated possessor must denote a player in an admissible number.

Off-battlefield applicability is written as explicit noun selections. Controlled
permanents, controlled spells, and owned cards outside the battlefield are
separate domains. Celestial Dawn includes every controlled spell; Encroaching
Mycosynth includes permanent spells. A placeless card description and its
published binding stay placeless until positive zone evidence constrains them.
Subtype-family edits require a hosting card type, not battlefield membership.

The prototype priority applies: the discarded placement-spelling prohibitions,
wrapper-specific `notExtended` refusal, and accidental battlefield evidence on
placeless card/source bindings are not compatibility contracts. Their affected
witnesses are re-spelled or corrected in the landing record.
