# Checker contracts

> **Architecture:** [Semantics v3](../docs/decisions/semantics-v3.md) specifies the
> checker this document describes. Every term is stepped through a Discourse
> Representation Structure (`DRS`, `stepOps` in [Phrase.lean](Semantics/Check/Phrase.lean)):
> each mention introduces or resolves a discourse referent (`Referent`, with a stable
> `ReferentId`); boxes nest; anaphors resolve by `J1` accessibility, `J2` clause-mate
> exclusion, `D′` for bare pronouns and `AC>R1` for demonstratives (`resolveCandidates`);
> a Definition body runs in its own Reference Scope (`Action.ownScope`, `enterOwnScope`,
> `inCaller`, `leaveOwnScope`). Authoring has the Definition/Template split and the
> Unified Action-Clause Spine (`instruct`, `does`, `would`, `cant`/`must`/`can`/`gatedBy`,
> `did`, `by`, `any`, `which`). The positional binding stack, its read windows and the
> capture annotations that preceded the DRT checker are deleted.

The Lean model is a prototype. Its authority is rules correctness and internal
consistency, not compatibility with previous checker behavior. Tests that encode
obsolete shapes or incorrect laws must be corrected with an explanation and
meaningful replacement coverage (user clarification, 2026-09-06).

The checker's charter (`docs/decisions/semantics-v2.md` §7, ruling 2026-10-06)
is that a written term is a faithful reading of the card's English: its
references resolve (anaphora, outcomes, choices, stamps, reference scopes,
plurality, the handoff performer) and its terms are well-kinded. It is not a
rules engine. An action the game cannot carry out is still a faithful
reading, since the impossible part is ignored and the rest is done
[CR#101.3,609.3]. The game-rule refusals below (zones, timing windows, costs
and deontics, tokens, grants, the card frame) are frozen: no new one lands
without a dated ruling, and the existing ones are retired bucket by bucket by
the `lean-checker-sweep-*` tickets, which update the sections they touch. The
zone and object-state bucket's 33 refusals are retired (`lean-checker-sweep-zone`,
2026-10-08): a status, damage, an attack, a move, an attachment, an exchange,
a stack action or a stat change no longer demands where its object is or which
objects it can reach. Where such a refusal also fixed a kind, the kind is
checked by `kindMismatch` or `kindLte`. The timing and window bucket's 10 refusals are retired
(`lean-checker-sweep-timing`, 2026-10-08): an activation, cast or trigger window,
the possessor of a turn part, an added turn part and its anchor, a "while" event,
the event that ends a duration, a usage limit, the event a replacement or a
trigger multiplier watches, a held clause, a nested "instead" and a chapter
ability's header no longer demand when the game can do what they say. The cost
and deontic bucket is retired (`lean-checker-sweep-costs`, 2026-10-08): 14 of
its 15 refusals go, so the checker no longer asks who pays a cost, whether a
symbol or an action can be a cost, whether modes are costed alike, or which
deeds a permission, a bound, an "as though" or a play rider goes with; an empty
mana run is refused as an empty list (`nonEmpty`), a term shape. The fifteenth,
`deonticPatientOk`, which stayed narrowed to references and kinds, is retired too
(`lean-v3-authoring-impl-7-checker-and-regime-retirement`).
The other game-rule bucket is retired (`lean-checker-sweep-game-rules`, 2026-10-08): 21 of its 43 refusals go,
so the checker no longer asks whether a deed rides, whether fateseal opens an
opponent's library, whether a deed's row admits the handed performer (the
handoff's kind is still checked, `kindMismatch`), whether a static ability, a
trigger header, an event or a rules row targets, whether a door is unlocked by
name, whether an exchange names one value twice, whether an ability can be
granted, granted to that subject, carved out of a retention or established as a
clause, whether a characteristic edit fits its subject, op and zone, whether a
shield's use fits, whether a designation is one an action confers, or what
a token's or an emblem's types, power and toughness, subtypes, abilities and
qualities must be. Their term shapes stay under existing constructors: an empty
edit list, emblem or color list is `nonEmpty`, an edit that adds or removes an
empty type line is `lineNonEmpty`, a list naming a value twice is `distinct`, and
an edit's chosen quality is `qualityRead`. Five stay as kind, plurality or term
shape: `designationScope` (a designation's holder sort), `facesFit`, and the registry
and table shape checks `tokenNamed`, `definitionHolder` and `definitionNamed`;
`definitionScoped` went with the designation record's zone, card type and Room half columns
(`lean-v3-catalog-items-self-contained`), and `manaSymbolOk` and `twoParties`, kept then, are
retired with the phrase-level game rules below. The
card-frame rules (`cardLine` through `prototypeFrame`) are kept: the owner
decided so on 2026-10-08 (§7 ruling 2026-10-06, consequence (d)), because they check
that a term is a well-formed card, not what an action does. Twelve remain of the fourteen:
`chapterFrame` (a chapter ability on a card without the Saga subtype) and `adventureInset` (an
Adventure inset without the Adventure subtype) are retired with the subtype frame table, because
a type line that disagrees with its card's text is a card-authoring mistake, not a text-meaning
error (ruling #2 of 2026-10-22, `lean-v3-card-class-gate-removal`). Zone state on referents stays, because references resolve
on it. The game-rule checks that remained outside these buckets are retired
(`lean-v3-authoring-impl-4-checker-dewarting`), so outside the card frame the checker enforces
reference resolution and kind agreement only. The phrase-level game rules that survived it
are retired as well (`lean-v3-authoring-impl-7-checker-and-regime-retirement`):
`deonticPatientOk` (a self-block), `deckReadable` and `deckComparable`, `qualifiedLimit` (a usage limit wraps an activated ability),
`counterBatchOk` (the last counter of a kind), `selfDefinedOk` (a characteristic-defining
ability's subject), `manaSymbolOk` (a hybrid or Phyrexian symbol's halves), `colorBoundOk`
and `colorCountOk` (how many colors an object has), `rolesOk` (distinct party roles; an
empty role list is `nonEmpty`), `subtypeType` (a creature-type exclusion names a creature
type), `twoParties` (an exchange of life totals names two parties), `kindAmountOk` (the
amount beside "the same kinds of counters") and `verbBecomesOk` (with the deed table's
`intransitive` column). So are the keyword-definition checks (`keywordBodyFits`: a
definition part's category, stack regime and card-class guard against the keyword's row)
and the card face's joint choices (`jointChoices`): a later ability on a face reads a choice
an earlier one makes through the threaded text context, as any reference does. The deed
table's role columns are gone, and
with them `deedFits` and `deedNounOk`: no deed asks whether its performer or its patient is a
player or an object, which card types or object classes it is, or which zone it is in. A
keyword action a permanent performs is refused on a player by its own Definition body, whose
reads of its performer (a controller, a counter, a designation, a power) do not resolve on a
player. Also gone are an object's possessor and `patientPossessor`, a referent's counter
memory and `counterMemory`, the counter holder tables (`counterSourceScope`,
`counterKindNamed`; `counterHolderKind` still requires an object or a player), the stat
type presupposition (`statHeadTysOk` [CR#208.3]; the projection's `.object` or `.player` scope
stays as `projScope`), the description contradiction tables (`contradictionFree`), a settable
exchanged value (`settableValue`), a permanent spell's card types (`permanentSpellType`),
the ascribed card type and subtype (`ascriptionOk`), a move rider's destination
(`riderKind`), and the zones `handoffPerformer` and `designationHolder` read. A type or
"copied" refinement of an attachment host word that is not an object noun is refused
`kindLte`. On the question of
scope this supersedes "rules correctness" in the paragraph above; the rest of
that paragraph stands.

The checker receives the information that indexed syntax formerly carried in
its types. This document records where those duties live. A passing check is
an admission of the written semantic term, not a proof of lowering or execution.
The remaining representation boundaries are named below rather than hidden by
the existence of a checking function.

## Card authoring and trusted expansion

`spelled <| card` is the checked authoring boundary. It retains an
`Authoring.Form` tree before unfolding erases named template and definition
calls, and constructs `Spelled` with both `onlyAuthored` and the existing
`ok : card.check = []` proof. The constructor is private. Record construction
and record update cannot replace these checks. Unregistered helper definitions
and nested template and definition arguments are inspected; opaque or variable
semantic content without authoring evidence is refused, including expressions
nested in printed-data records.

The authoring coercions expand before `inspect` reads the term, so `spelled` sees the Template
they stand for and the retained Form is the one the written-out call gives: core's `α → Option α`
(`untilEndOfTurn` for `some untilEndOfTurn`), a written mana cost as a `Cost` (`[generic 1]` for
`mana [generic 1]`), an `Action` as a `Cost` (`sacrifice thisArtifact` for `perform (…)`), and a
catalog `Subtype` as a `Predicate` (`goblin` for `hasSubtype goblin`). A leading-dot name resolves
against the expected type before any coercion applies, so `some .combat`, `mana [.snow]` and
`hasColor .white` keep their heads. A numeral is an `Amount` (`3` for `.lit 3`): it stays an
`OfNat.ofNat` term that `inspect` reads through to its `Amount.lit` leaf, naming the same Templates
and literals. A single-faced card is `singleFacedCard { name := …, … }`. The pins are
`Proofs/Authoring`.

Semantic expressions must use authored declarations: a `semantic_definition`
(a Definition, the rules-defined vocabulary of a keyword action, keyword
ability, counter, token or designation) or a `semantic_template` (a Template,
a transparent phrasing the rules do not define). Printed-data records,
enumerated leaf values, and numeric literals remain ordinary data; their
semantic fields are inspected recursively. The modules are layered one way,
`Semantics.Primitives` ← `Semantics.Templates` ← Definitions ← cards, the Definitions all
re-exported by `Semantics.Definitions`. Templates name no catalog item and no card type. The leaf
catalogs (`Semantics.Definitions.Counters`, `.Designations`, `.Relations`, `.Subtypes`) are
spelled over primitives only, except that `.Subtypes` is spelled over the raw `Subtype`
constructors, which have no primitive; `Semantics.Definitions.Phrasings` holds the Templates that
name a card type or a leaf catalog entry ("this creature", "your party"), spelled over Templates
and the leaf catalogs; the composite Definitions (`Semantics.Definitions.Actions`, `.Keywords`)
are spelled over all of them. The checker sits beside this stack, not under it: Primitives, the
leaf catalogs, Templates and `Phrasings` import none of `Semantics.Check` (the structural readers
they need, such as `Location.sort`, live with the Core AST), and only the composite Definitions
read a checker reader (`NounPhrase.plur`, for "It/They can't be regenerated"). A word a Template
sort and a Definition share (`crew`, `saddle`) is dispatched on its expected type where the
Definition is declared, so Templates name nothing above them. A declaration whose bare name another
declaration or a syntax word already takes is named in its sort's namespace (`Subtype.army`,
`Subtype.infinity`, `DesignationLabel.solved`, `Ability.splice`).
`Semantics.Primitives` provides the generated constructor wrappers
(`declare_semantic_primitives`, `Semantics.Primitives.<Type>.<name>`), which
have definitionally identical constructor signatures and are registered as
templates, together with a few worded primitive templates and the
characteristic inputs. `NounPhrase.pro` and `NounPhrase.which` have no
primitive wrapper: they are expansion carriers (`internal_expansion`) that only
templates and definitions write. `Subtype` is a semantic expression with no primitive wrapper
(`internal_expansion`) either. A subtype in authored content, including one on a printed type
line, is a leaf catalog entry (`Semantics.Definitions.Subtypes`), so *Mishra's Factory* names
`assemblyWorker` and cannot misspell Assembly-Worker [CR#205.3m]. `spelled` refuses a raw
`Subtype.of` or `Subtype.spell` in a card, including one built by an ordinary helper definition
that the card inspection follows. `CounterKind`, `DesignationLabel` and `RelationLabel` are catalog
items the same way (`internal_expansion` semantic expressions, entries in
`Semantics.Definitions.Counters`, `.Designations` and `.Relations`), and each value carries its
own columns, a counter's holder and a designation's scope, so no checker table names them: an
undeclared counter, designation or relation is refused at the authoring boundary, as a raw
constructor, and not by the checker. Card fragments and `Proofs/` terms that call the checker
directly are fixtures and are not checked against any declaration list (owner ruling,
2026-10-23). Authored declarations (`semantic_definition`,
`semantic_template`) are trusted, as for every semantic type. Values that are not `spelled`
cards are not checked: the checker's tables (`Check/Facts.lean`), and the pins that build a raw
subtype on purpose (`badNamelessSubtype` in `Proofs/Rules.lean`, and the refusal pins in
`Proofs/Authoring.lean`). `Semantics.Templates` holds every
`semantic_template` that names no card type and no catalog entry; the `Semantics.Definitions`
submodules hold every `semantic_definition` and the templates that name a card type or a catalog
entry (`Phrasings`, which names leaf catalog entries rather than spelling their strings) or whose
bodies need a composite definition: the templates over its actions follow the actions, and a
keyword's licensed phrasings follow the keyword.
Defaults retain their calls.

A declaration generates the hidden construction context, splice closures, and
registration from one ordinary Lean definition body. It returns its semantic
type directly; `expand` is a transparent identity, not a trust boundary. An
`Action` Definition's body must establish its own Reference Scope
(`Action.ownScope`); the declaration is refused otherwise. Only authored
declarations are trusted. Neither helpers, discarded arguments, nor
projections may conceal raw semantic syntax, even if their result is ordinary
data. Inspection skips ordinary helper bodies only after scanning the instantiated
expansion's reachable definitions for semantic types, constructors, and calls;
its arguments are still inspected. This includes semantic functions hidden in
containers and supplied higher-order template or definition calls. Generated
Binding-identity and caller-scope constructors have no public primitive.

A supplied parameterized body is inspected under evidence for its explicit noun
parameters. Other semantic free variables remain refused. Callback type aliases
are resolved during type elaboration; callbacks receive the receiving
declaration's construction context. A caller cannot substitute a different
hidden lexical context. The authoring tree records formal body binders and
their uses as well as the enclosing named call.

The kernel proves that the retained authoring tree contains no raw semantic
constructor and that the resulting card passes the checker. Correspondence
between the input expression and that tree is the elaborator's responsibility;
this is not a kernel proof about Lean source text. Registration establishes a
trusted authored declaration. The checker validates its expanded body and
contextual use, but does not prove that a Definition implements its label's
rules meaning. Extending the Definitions and Templates therefore requires
reviewing the expansion itself. Raw expanded terms remain available to checker
diagnostics and proof suites.

The tree retains template and definition names, data constructors and literals
as a future attachment point for spellings. The separate English project has no
dependency on this mechanism. No rendering or execution equivalence is claimed.

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
Equipment's or Fortification's host is an object [CR#301.5,301.6]), and so is a host word
that refines a player by a card type. A host's card types, written in one description or
in a counterpart or host word, are not checked against the attachment word
(`lean-checker-sweep-zone`, `lean-v3-authoring-impl-4-checker-dewarting`). Quantified
attachment counts and counterpart target publication remain checked. The `isAttached`, `attachedBy`, and `attachedTo`
templates expand to the shared predicate.

`currentFace` describes a single double-faced permanent as a whole. Its front
and back alternatives are distinct from face-up and face-down Status. The
`isTransformed` template expands to the back alternative: prior transformation,
copying back-face characteristics, or a back-face-up component in a merged or
melded object is insufficient [CR#701.27g]. The checker validates the predicate's
object domain; runtime evaluation of current game state
remains Rust's responsibility. The fold does not claim an execution proof.

## Historical events

A lookback is a `DidClause`: a `Subject`, an `Action`, an optional event ordinal (`Ordinal`
within an optional `Period`, the shape `nthOccurrence` carries) and an optional `LookbackSpan`,
either a `duration` (with an optional `Timing` the action happened under) or an
occurrence (`thisWay`, every event of the named kind the current spell or ability caused, paying
its costs included, read through the referents it stamped (Participial Descriptions, below), so
*Rally the Horde*'s "each nonland card exiled this way" reads the cards all three of its exiles
exiled, and *Sinister Concierge*'s "each card exiled this way" reads both cards its optional exile
and its "if you do" exile exiled; `triggering`, the event that triggered the ability), which names no span,
or `thisObject`, the described object's own casting or activation in its current existence
[CR#400.7] ("artifact spells you cast", "if you cast it from your hand"; a history window, "a
spell you cast this turn", keeps its `duration`). There is no separate cast-by, cast-from or
activated-by predicate: each is a `Predicate.did` over `cast` or `activate` with that span.
`Condition.did`, `Predicate.did` and
`Amount.tally` carry one; the `did` and `was` (agentless) templates write it, and only
`Predicate.did`/`Predicate.was` may leave the span unwritten. An unwritten span is refused
`unanchoredLookback` everywhere but a Participial Description (below). With no agent
written, or one written with "by", the clause hides the enclosing handoff's performer
while it steps: the deed looked back at is not the performer's. `stepDidClauseWith`
steps the action as an event pattern (`actionPattern`) after the clause's subject, so it publishes its participants without
moving current referents or manufacturing an outcome. `Predicate.did` is a relative
clause: it opens a new clause with one `which` referent (`relativeBinder`) and leaves
through `leaveRelative`, which closes everything the clause introduced except a definite
attachment host. A relative clause conjoined in an object `Predicate.and` binds `which` as
that whole predicate binds its head (`relativeSeed`: the sibling conjuncts' types and zone,
so "creature that blocked" reads a creature on the battlefield); elsewhere `which` is a bare
referent of the enclosing predicate's kind. The `which` template writes `which`
(`NounPhrase.which`). A `which` with no enclosing relative clause is refused
(`anaphor`); a nested relative clause binds its own `which`, and `Condition.did`
clears the binder, so a nested lookback cannot read its parent's participant; and a relative clause that never reads its own `which` is refused
`lookbackSubject`. Repeated reads within one clause name the same participant.

A `thisWay` lookback is grounded in its own text: it reads back an event the earlier text of the
same spell or ability caused, its costs included ("Counter target spell. If that spell is countered
this way, …" [CR#608.2c]; `thisWayReadsItsCostsEvent`), and is refused `thisWayUngrounded`
otherwise, so "Draw a card. If you search your library this way, shuffle" is refused
(`thisWayNamesNoEarlierEvent`). Each resolving or paid action a lookback can name
(`Action.eventHeadOk`) records its event (`DRS.withCaused`, `DRS.causedEvents`): the Deed that
names it, the Zone Change it writes, its classes (`EventVerb`, for an event that names neither:
damage dealt, life gained apart from life lost [CR#119.3]), its Actor's kind and the kind of what
it moved or acted on. Paying a cost records a payment however the cost is written [CR#118.1], so
"if you paid life this way" reads a cost's "Pay 2 life" as it reads an effect's
(`thisWayClassesTheEvent`). The lookback matches one by its own Deed (casting reads a card played
[CR#701.18b]), Zone Change (into the same zone, from the same one when both name it) or class
(`CausedEvent.readBy`), done by an Actor its subject can name (a player's sacrifice is not "this
creature's", `thisWayReadsTheActorItsSubjectNames`) or, where the subject is what the event moved
or acted on ("that creature dies" [CR#700.4], "a land enters"), done to something it can name
(`CausedEvent.subjectFits`, `thisWayReadsWhatTheEventMoved`). Destroying or sacrificing a
creature also records its move from the battlefield to a graveyard, which "if it dies this way"
reads. A permission or requirement writes the act it allows or demands, so it grounds "this way"
too, though no one need take it ("You may play those cards this turn. If you cast a spell this
way, …"); a Participial Description that nothing stamped reads what is acted on that way as the
game goes on, introduced as it is read ("each spell cast this way"); a prohibition writes no act
(`thisWayReadsAPermittedAct`). A Participial Description's anchoring clause is grounded by the
referents its deed stamped instead (`DRS.participialAnchorRead`). The text, not the game, grounds the lookback: whether the
event happened is the engine's. So an optional body and both branches of a `doIf` keep their
events, a declined "you may search" included, for a later "if you search your library this way"
(`agencyOutfitterSearch`, `optionalBodyEventGroundsThisWay`); "if you do" is not a lookback, and
reads the choice (`withContinuation`). Nothing a delayed or reflexive triggered ability does
grounds a "this way" in the ability that created it [CR#603.7,603.12]
(`reflexiveAbilityEventsStayItsOwn`), though its own text reads that ability's events, as it reads
its stamps; another ability's events, a linked one's included, are never read [CR#607.1,607.2a]
(`thisWayReadsNoLinkedEvent`). A `triggering` lookback reads the events its triggered ability's
headers name, a delayed ability's event or a reflexive one's "when you do" [CR#603.2,603.12]
(`DRS.triggerEvents`), each layer of a keyword action's expansion included ("whenever you
sacrifice a creature, if it died …"), and is refused `triggeringUngrounded` otherwise
(`triggeringReadsItsTriggerEvent`).

`GameEvent.facts` supplies the magnitude property directly from the event
constructor; which events a replacement, a trigger multiplier, a duration or a
"while" clause may watch is not checked (`lean-checker-sweep-timing`). Source sets and
exclusions use `Location.not`; an enacted deed's optional locus is checked against the
deed's declared loci (`lookbackLocus`). A cast may leave its spell unspecified, which
introduces no spell reference.

`Proofs/Lookback.lean` checks participant isolation, retained references, source and
locus validation, and event-fact preservation through modifiers. These are
checker guarantees; no engine history-query implementation is claimed.

## Reference resolution

Every anaphoric `NounPhrase.pro` resolves against the visible referents of the
current `DRS` (`resolveCandidates`, [Words.lean](Semantics/Check/Words.lean)). The
accessibility filter (`J1`) admits a referent that no open Reference Scope masks
(`DRS.visibleReferents`) and that the reach accepts (`reaches`: number, sort, carrier
word and stamp; `reachesCharacteristic` while a characteristic is read, so a departed
permanent's last known characteristics stay readable). A bare pronoun (`.bare`,
`.atSlot _`) then follows `D′`: in a direct argument slot it excludes the current
clause's mentions (`J2`, `DRS.isClauseMate` over `pendingRoles`), keeps the nearest
clause and the highest grammatical role (`GramRole.rank`: subject, object, other), and
resolves only to a unique winner; none or a tie is refused `anaphor`. A referent
re-mentioned in a direct argument slot of the current clause ranks there, by the best of its
buffered slot roles, before `nextClause` commits it (`DRS.rankingClause`, `DRS.rankingRole`; owner ruling
2026-10-21). So "that creature deals damage equal to its power" reads "its" as that
creature. An argument-slot pronoun has already excluded such a referent under `J2`. A demonstrative
(`.word _`, `.unionHalf _`) follows `AC>R1`: it also excludes clause-mates
in an argument slot and any referent a bare pronoun already resolved to in this clause
(`pronounResolved`), then takes the most recently introduced candidate. Neither
reaches a referent a linked ability published (`Referent.linked`); only a Participial
Description (below) does. A reflexive (`NounPhrase.refl`) resolves to the current clause's subject (`resolveReflexive`).
Possessors and other nested mentions record no direct-argument role, so "target
creature's controller sacrifices it" reads the creature. `nextClause` commits the
pending roles, advances `currentClause` and clears `pronounResolved`.

A measured domain is no antecedent: `Amount.countOf` ("the number of …") and
`Amount.aggregate` ("the greatest … among …") close the domain they measure as a relative
clause is closed, keeping only the targets announced inside it (`leaveMeasure`,
`measureKeepsItsAnnouncedTargets`), so "those creatures" after "where X is the number of
attacking creatures" reads the creatures that got +X/+0 (`measuredDomainIsNotAnAntecedent`,
`tearsOfRageThoseCreaturesReadTheSubject`). An existential (`Condition.exists_`: "if there is a
…", "if two or more creatures are tied …") publishes its subject, a plain, counted or
coordinated indefinite (`NounPhrase.existentialMention`), for the consequent
(`Referent.existentialDomain`); a `doIf` with no other branch, and a replacement's condition
(`Replacement.onlyIf`), close it after the consequent (`semantics-v3.md` §1.3, box 2;
`replacementConditionClosesItsExistentialDomain`). The domain a replacement's condition
publishes (`Referent.replacementDomain`, `markReplacementDomain`) ranks below ordinary mentions
for a plural pronoun (`D′` and `AC>R1`) and for the groups "the rest" and "the other" are taken
from (`preferOrdinary` in `resolveCandidates`, `countGroups`, `countedGroupSize`): it is read only
where no ordinary candidate is in scope (`flowStateThemAndTheRestReadTheCardsLookedAt`). Any
other existential's domain ranks as an ordinary mention, the nearest
(`okJuxtaposeThemReadsTheTiedPermanents`, `purgingScytheThemReadsTheTiedCreatures`). A
self-replacement's replacement (`Action.replace`) starts from what the replaced action read,
without the referents it introduced but its targets (`replacementStart`;
`selfReplacementReadsWhatTheReplacedActionRead`, `okFblthpSelfReplacement`); after it, later text reads the replacement's referents first, then
the replaced action's. The subject of a property test (`Condition.matches`)
is definite, except that a coordination may hold an indefinite conjunct, read existentially
(`onlyACoordinatedTestSubjectHoldsAnIndefinite`, `titaniaThemReadsTitaniaAndArgoth`).

A Participial Description is a `NounPhrase.described` with determiner `the` (one, or
`many`: "the exiled cards"), `all`, `each` or the bare plural, one of whose conjuncts is a
`Predicate.did` with no span written or `thisWay`, whose `which` is what the deed acted on
(`Predicate.participialAnchor?`). It introduces no referent: `participialCandidates`
selects the referents its anchor stamped (`Referent.anchoredBy`): the deed the clause names
(`Referent.stampedBy`, which also reads the deed a keyword ability's verb was done by:
devouring is sacrificing [CR#702.82b]) or, where no deed names a move, the move's Zone Change,
by the destination it writes and the origin, if one is written, so "the creature returned this
way" and "the creature put onto the battlefield this way" read one creature
(`returnedAndPutReadTheReturnedCard`, `returnedCreatureReadsThePermanentItIs`,
`zoneChangeReadsItsWrittenOrigin`), of its number, named by the other conjuncts' Head Noun
(`card`, a battlefield location as "permanent", Card Types: `stampedWordOk`) and in the zones
they name now. "Card" reads the zone the object is in now; a Card Type or "permanent" reads the
permanent it is, if it is on the battlefield now (one a move put there), or else the permanent it
was when acted on, a departed one by its last known information [CR#608.2h], so "each permanent
put into a graveyard this way" reads a destroyed permanent and no discarded card
(`permanentPutIntoAGraveyardReadsItsOrigin`). `thisWay` reads every referent the current ability
stamped, inside an optional body or not (`okEachCardOfSeveralExiles`,
`thisWayReadsWhatItsCostDid`, `eachCardExiledThisWayReadsEveryExile`), and none a linked ability
published,
since "this way" is no linking phrase [CR#607.1,607.2a]; no span written also reads those a linked
ability published, and a singular
`the` reads a published group [CR#607.3]. `participialRead` resolves: a group determiner
reads every candidate (a Card Type picks members, asking nothing of the group); `the`
reads one site, one object of the current ability or what one linked ability published,
whose exclusive arms count as one; a group reads a part taken from a group it also reads
(cascade's hit within its run) once, as one of the group's members. Otherwise it is refused `participialDescription`. Its
agent must be none, "you", the performer, the source, or for a keyword ability's own verb
the object that has it (`participialAgent`): a stamp records no agent. `Proofs/Lookback.lean`, `Proofs/LinkedReadback.lean`
and `Proofs/NounWords.lean` pin it.

`NounPhrase.pro` and `NounPhrase.which` are expansion carriers (`internal_expansion`):
cards write the templates (`it`, `that`, …) that expand to them. No template reads a
position. The scope templates that once carried a numeric or patterned read are
ordinary phrases now: `attachToIt` writes `it`; `dealDamageOwnPower` and
`controllerSacrifices` bind their subject with `firstMention` and read it back with
`laterMention`; `comparesOwnStat` measures `ownMember`, a bare pronoun that the
comparison's own clause box resolves to its candidate (`Predicate.compareOver`);
`aTheirChoice` is a determiner mode (`ChoiceMode.theirChoice`) checked by
`choiceModeChecks`; "gets +P/+T" is one `StaticSpec.characteristicChange` with a
power and a toughness `CharacteristicEdit.stat`, which steps its subject once and its
toughness amount after its power amount. A printed "+0" is written: "gets +X/+0" is
`getsPt subject (up X) (up (.lit 0))`, never a lone power `modification`.

`Proofs/DrtResolution.lean` pins each rule (nearest clause, role ranking, true ties,
clause-mate exclusion, possessor exemption, masking, number, carrier, demonstrative
recency and its exclusions, reflexives). `Proofs/ReferenceScopes.lean` proves the
former scope-template readings under ordinary resolution, including two general
theorems that a subject introduced in the newest clause wins over every older
referent, for any outer context; `Proofs/DrtCards.lean` and `Proofs/DrtWitnesses.lean`
pin corpus cards. These are reference-checking laws, not lowering or runtime
object-identity proofs.

## Controller as the subject of a sacrifice

`controllerSacrifices subject` hands a `sacrifice` to the subject's controller:
`instruct (possessorOf .controller (firstMention id subject)) (sacrifice (laterMention
id subject))`. The possessor phrase introduces the subject once, under its Binding
Identity, and then its controller, who performs the body. The deed's patient reads
that referent back by identity, so it declares no second target, and an object a
nested description mentions cannot become the patient. An already-referential subject
introduces nothing new and is read again. The ordinary sacrifice expansion moves the
patient to the graveyard and records sacrifice provenance [CR#701.21a].

The Arcum Dagsson fragment leaves one controller and the same artifact creature, now
in the graveyard with its sacrifice stamp; subsequent player, card and stamped
permanent reads use those referents. `Proofs/ControllerSacrifice.lean` proves the
expansion for every subject and for arbitrary outer contexts (the move preserves every
unrelated outer referent), target multiplicity, the existing-reference case, and later
reads. Authoring through templates and definitions establishes trusted expansion
origin; these are checker/reference guarantees, not an engine execution proof.

## The performer and the handoff

`NounPhrase.actor` is whoever performs the action it is part of: the controller
[CR#109.5] unless an enclosing `Action.instruct` names another player or a
permanent. `instruct subj body` steps `subj` in the subject slot, then pushes a
checker-private handoff frame (`Payload.actor`, an `ActorShape` recording the
performer's kind, whether it is "you", and the referents it named) and steps the body
under it. The innermost frame decides (`actorShape?`), so inside a handoff to a
permanent `actor` is the permanent and `controllerOf(actor)` a player; outside every
handoff, and inside `instruct you`, `actor` reads as `you`. The performer must be a
player or an object; anything else is refused `handoffPerformer`. Where the object is
(a card in a graveyard handed an action) is not checked. Leaving the handoff
drops the frame (`dropActorFrame`): a handoff publishes the referents its subject and
body introduced, so a handed-to target stays available, and a handoff to `this`
publishes only the body's.

"<this> deals …" on any card, a spell's as a permanent's, is a handoff to the source,
`instruct this_ (dealDamage …)`: an object that deals damage is the source of that damage
[CR#120.1], and a bare `dealDamage` would have the controller, a player, as its performer.
Where one sentence names the source once for several recipients, one handoff wraps them all.
"<X> deals damage equal to its power to <Y>" is the `dealDamageOwnPower` Template, whose tree
is the pronoun form `instruct X (dealDamage (powerOf it) Y)` (owner ruling 2026-10-21).

No action writes its performer: the agent fields are gone
(`semantics-v2-drop-agent-fields`), and with them `choose`'s chooser and an added
turn's taker (`semantics-v2-chooser-and-taker-fields`). A choice is judged as the
performer's: its chosen phrase must be `agentChoosable` (`choiceClause`). An added
turn is taken by the performer, which must be a player [CR#500.7]; an added phase or
step is added to the current turn [CR#500.8,500.9]. A deed reads the performer from
the frame. `enact` does not check it against the deed: crewing, saddling, fully unlocking
and triggering check the frame's kind (`eventAgent`), and a deed's own body reads its
performer like any other reference. A deed whose row says a permanent performs it
(`ActFacts.permanentActor`: fight, explore, adapt, connive, endure, harness and
monstrosity [CR#701.14a,701.37a,701.44a,701.46a,701.50a,701.63a,701.64a]) written with no
handoff to a permanent
(printed "Adapt N" [CR#701.46a]) is performed by the source permanent: its body is
stepped under a frame for "this permanent". Inside a handoff to another player there is
no such default, and the body's reads of its performer are refused on the player:
explore, connive and endure hand their steps to the performer's controller
(`possessorKind`), adapt reads its counters, monstrosity its designation and fight its
power (`kindMismatch`). `Proofs/Actor.lean` pins each. An event's performer is written in
the same kind: "whenever a creature you control explores". Who pays a cost (the
retired `payAgrees`, `costPaidByYou` and `keywordCostPaidByYou`) is not checked
(`lean-checker-sweep-costs`); a cost may hand its action to another player.

What a handoff publishes depends on its performer's number. A singular performer
publishes the body's mentions as they are. A handoff to `each …`, or to a plural group
over a body with a performer of its own (`Action.performed`: a deed, a life change, a
draw, a choice, a vote, a payment, …), is memberwise: inside the body the group's
referents read singular, as each member performing it, and they read plural again
afterwards. A distributive handoff pluralizes what its body introduced
(`pluralizeBinding`), except outcomes, which stay one total: "each opponent loses 1
life and you gain life equal to the total life lost this way" [CR#702.101a] reads it
through `thatMuch`. `doForEach` publishes no such total, so "that much" after it is
refused (`badForEachLossThatMuch`).

A one-card slice of a library or graveyard reads its owner's number (`NounPhrase.plur`):
"the top card of your library" is one card, "the top card of your opponents' libraries"
several. An owner written with `each` is the exception: "the top card of each player's
library" is one card per player, collected, so the slice is plural and reads back as
"those cards" (`playersTopCardIsPlural`, `eachPlayerTopCardSliceIsPlural`). Any other
one-card slice is one; a longer slice, a run and `oneEachOf` are plural.

A nested ability's performer is its own controller: granted ability text, token
characteristic text and emblem abilities are stepped in `linkedTextContext` (only chosen
qualities, chosen players and the objects the enclosing ability has acted on carry in, never
a handoff frame), and a keyword ability body in its own Reference Scope (`enterOwnScope`).
The acted-on objects carry in linked, read back by a Participial Description only ("return the
exiled card"), never by a pronoun: abilities on a token, an emblem or a permanent and on the
source of the ability that created it or put it onto the battlefield can be linked
[CR#607.1d,607.2a], and an ability granted to the object the granting ability is printed on
counts as printed on it [CR#607.1a]. A grant onto another object (*Massimo, the Magician*'s "that
creature gains '… copy the exiled card'") is read as Oracle writes it (*Hofri Ghostforge*,
*Timothar, Baron of Bats*; `Proofs/LinkedReadback.lean`).

An object referent records no possessor. Whether a sacrificed permanent is one its
performer controls, or a discarded card one in its performer's hand, is a game rule
[CR#701.21a,701.9a], not a reference, and is not checked
(`lean-v3-authoring-impl-4-checker-dewarting`).

`Proofs/Actor.lean` pins the handoff cards, what each kind of performer publishes,
nested handoffs, nested abilities, the parameter `actor`, every cost the bench hands
to another player, the group handoff's totals, and the handoff to a permanent with the
default to the source permanent. These are checker guarantees, not an execution proof.

## Binding identity and a keyword body's own scope

A Definition's or Template's referent parameter is bound at the body's first mention and
read back by its Binding Identity at every later one (`docs/decisions/semantics-v3.md`
§§1.2,1.4). `NounPhrase.firstMention id phrase` is stepped as `phrase` is and records
`id` on every referent `phrase` introduced or resolved (`Referent.idents`,
`recordIdentityById`), each member of a coordination included. `NounPhrase.laterMention
id phrase` reads the visible referents recorded under `id`: it selects nothing and
resolves no anaphor, so a target argument is one targeting however often the body
mentions it, and a referent introduced between the mentions does not move the read. The
identity is a name, never a position. Its `phrase` answers only the structural readers
(`isYou`, `selfDefinedOk`, `kind?`, `plur`, …). A phrase that introduces nothing ("you",
"this") is read again at a later mention; a later mention with no visible referent under
`id` is refused `bindingIdentity`, never read as a second selection, unless the first mention
itself bound nothing: that phrase's own refusal is the one reported, and the identity is
recorded unbound (`unboundIdentities`) so its later mentions add none. A zone change
gives the moved object a fresh referent without the identities [CR#400.7], so a later
mention of the pre-move parameter is refused; updates in place keep them.

A keyword action body is read in its own Reference Scope: `Action.ownScope deed body`,
which every `Action` Definition opens with (`semantic_definition` refuses one that does
not). `enterOwnScope` pushes an `OwnScopeFrame` that masks every referent of the calling
text except the performer the innermost handoff names, and opens a new clause. The
body's parameters are written `inCaller`: `enterCaller` pops only the top frame, so the
argument is read in the immediate caller's view, with any enclosing Definition's masks
still in force, and `leaveCaller` unmasks what the argument introduced or resolved and
records it as caller-owned (`callerReferentIds`). `leaveOwnScope` pops the frame, restores the caller's pending roles as they were when the body
was entered, with each argument the body read in its caller pending as the call's object
(a mention in the body never changes a role the caller can see), drops
the body's partitive and player scratch referents, keeps every update to caller-owned
referents and their successors, and exports the body's other new referents (its
outcomes, piles, qualities, letters, turn references and object results) at the
caller's clause as objects of the call. `Action.scopeBody` sees through these wrappers
for structural reads. A keyword ability's body is stepped in its own Reference Scope
too (`enterOwnScope`), so it cannot read its card's other mentions.
`Proofs/BindingIdentity.lean` pins the identity reads, fight, regenerate, the
looks of scry, surveil and fateseal with the slice's number, connive and airbend;
`Proofs/Actor.lean` pins the scope (the literal amass after a destroy, refused bare and
checked in its scope, Azog's amount read in the caller's view, a body that cannot read
its caller, a keyword ability that cannot read its card). These are checker guarantees,
not an execution proof.

## Scheduling fields

`skipUntap` steps its subject in the patient slot and then its amount. `skipPart`
steps only its count. `addTurnPart` steps its count and checks that the performer in
context is a player; an added turn then pushes a turn reference (`turnRefB`) that
"that turn" reads back, after the amount, so the amount cannot read it. Part, anchor
and following-part fields carry no references and are not checked
(`lean-checker-sweep-timing`). No scheduling field opens a box.

An amount's fields are stepped in textual order (`stepAmount`); arithmetic reads its
operands left to right, so a mention inside the amount is available to a later
operand. `Proofs/Turn.lean` pins the turn reference an added turn publishes
(`okThatTurnAfterASingleTurn`, `badThatTurnAfterTwoTurns`), forward-reference refusal,
and reads through unrelated outer referents.

## Definitions and composition

`StaticSpec.letterDefinition` is the one numeric definition form. The `define`
template establishes it without a duration. In static and action positions alike it
steps the amount in the incoming scope and closes a letter an earlier phrase left
open (`defineLetter`); a definition with no open letter to close is refused
`openLetter`. An earlier target or affected-object mention remains available to a
trailing definition; nothing is hoisted. A durationless numeric definition is
admissible inside a cost (`okNumericDefinitionAsCost`).

`withContinuation` carries an optional policy, decided by its performer (the actor in
context), or a required policy. Its positive branch is conditioned on the decision or start of
payment [CR#118.12], not a test of the resulting events. A required continuation
keeps the reflexive-enclosure and armed-continuation checks (`reflexEnclosure`,
`ifDoneArmed`). `repeat_ (.fixed …)` carries its count and body and publishes the
count; the other repetition policies remain unchanged.

Noun conjunctions and alternatives are lists, as are cost alternatives. They
require at least two members; the binary `both`, `eitherOf`, and `either` templates
supply two-member lists. Noun conjunctions thread introductions in textual order;
alternatives step their members independently. Predicate and condition lists
retain their existing cardinality and scope laws. `sequentially` steps each action
in a new clause after the previous one's output, while `simultaneously` withholds
each sibling's new outcomes from the next sibling and restores them afterwards. Both
require a nonempty list (`nonEmpty`). `Proofs/Composition.lean` pins these laws.
These are syntax and checker guarantees; they do not prove Rust execution
equivalence.

## Enclosed traversals

These are explicit boundaries in the stepper (`stepOps`), not exceptions to be
inferred from a failed pronoun. Unless a row says otherwise, a construction steps its
members in order through one `DRS`, and `nextClause` separates clauses:

| Construction | Context rule |
|---|---|
| `sequentially`, `Cost.compound`, `StaticSpec.conjunction` | Step each member after the previous member's output, each in a new clause; a nested conjunction is refused `notCoord`. |
| `simultaneously` | Step each member after the previous one, withholding the new outcomes of earlier siblings from later ones. |
| Predicate sibling modifiers, condition siblings | Step in the same containing context under the head's kind; conjunction does not open clauses. |
| Alternatives, modes and result-table rows | Noun alternatives step each member from the enclosing context and merge what they introduce; cost alternatives publish nothing; trigger alternatives and joined headers keep what every header introduces (`commonHeaderContext`). Each mode branches from the enclosing context, its cost before its body, and publishes nothing (`stepModeListWith`); result-table rows check a nonzero literal range before each body (`stepRollRowListWith`). |
| `withContinuation` (`may` / `doIfDone` templates) | The body, the success arm after the body, and the failure arm from before the body each open a clause; only the optional body's shortfall and repeat count, the choice evidence, the events the body or an arm caused (read by a later "this way") and the objects the body or an arm stamped escape. A stamped object escapes into `DRS.participialOnly` (an earlier object, moved or not, under its earlier id, read once however many paths stamped it, `objectStampedOnTwoPathsIsReadOnce`; one an arm introduced under a fresh id), read by a Participial Description alone ("each card exiled this way"), never by a pronoun, a group, a count or a Contrast Anchor (owner ruling 2026-10-22: "this way" ranges over the events that happened; `eachCardExiledThisWayReadsEveryExile`, `optionalBodyObjectEscapesOnlyToParticipialReads`). |
| `doIf` (`doIf` / `doOnlyIf` templates) | `doIf` first announces the then-branch's targets, chosen as the spell or ability is put on the stack [CR#601.2c]; the condition reads them, wherever it is printed (`doOnlyIf`, "counter target spell if …"), but nothing else the branch would choose or create. It then steps the then-branch in a new clause after the condition, under the ids it announced its targets with, so a later line reads the target the condition read; the condition's referents take ids past those reserved for the branch, and should the branch, in the condition's context, need more ids than reserved, it takes fresh ones instead, so no two referents share an id (`conditionalBranchIdsNeverCollideWithTheCondition`); its output continues when there is no else-branch. The else-branch is stepped from the context before the `doIf`, plus the announced targets and the amounts the then-branch names ("otherwise, each opponent loses that much life"); an indefinite participant the then-branch would choose as it resolves is not in it (`badOtherwiseReadsIndefiniteThenArm`). With an else-branch, only the then-branch's targets, and the events either branch caused, escape. |
| `doForEach`, `doForEachKind` | Push one singular loop referent (or one quality) at the header clause and step the body in the next clause; on exit keep only outer referents, with their updates, and publish the loop's choice history, explicit target results and created results as plural aggregates (`publishLoopTargetResults`, `publishLoopCreatedResults`); `doForEachKind` also publishes each outcome sort its body published, once and as a group (`publishLoopOutcomes`), so doubling each kind of counter puts counters. |
| `enact` | The performer in context (`actorShape?`) is checked at the deed's agent kind; a deed performed by a permanent with no handoff to one gets a frame for the source permanent. Post-event, the body's patients receive the deed's stamp (`applyInPlaceStamp`). |
| `instruct` (handoff) | See "The performer and the handoff" above. |
| `ownScope`, `inCaller` | See "Binding identity and a keyword body's own scope" above. |
| `delay`, `triggerReflexively` | Step the inner body in a new clause after `settleTargets`, its `triggering` lookbacks reading the delayed event or what the reflexive ability's "when you do" names; nothing the inner body introduces or causes escapes (only `nextId` advances). A resolving `delay` records the delayed triggered ability it creates, an ability in no zone, in `DRS.delayedTriggers` under the Binding Identity `delay.trigger`: no text mentions it, so no pronoun, word, group or count reads it, and only a later mention of that identity does, as the special action that establishes that it can't trigger (`cant (.actor …) trigger`, `delayUnlessPaid`, [CR#116.2c]; `cantTriggerReadsTheDelayedTriggerByIdentity`, `pronounAfterDelayReadsTheCreature`, `badPronounAfterDelayedDraw`). |
| `replace`, `holdUntil` | `replace` steps the replaced action as a pre-event pattern and the replacement after it in a new clause; `holdUntil` steps the held action, then its event. The static `replace` steps its `would` clause, then the replacement in a new clause, and publishes only the letters the clause and the replacement introduce, so a later "where X is" defines them [CR#107.3i]. A replacement steps what it makes of the event: `instead` and `also` step their action; `withRiders` steps its riders, refused (`riddenEntering`) unless the replaced event is an entering, a move to the battlefield or a token's creation, and (`entryRidersOnly`) for a Pile Placement among them [CR#614.1c,614.12]; `onlyIf` steps its condition, then its body in a new clause after it, as `doIf` steps its then-branch. A caller's replacement steps in one caller frame, its condition and action together. The static `replace` also publishes the choices its `also` action makes to later lines, as "As this enters, choose a color" [CR#614.1c]. |
| `revealedAsCast` | Steps the spell the card is revealed as cast onto, then what it grants in a new clause, reading that spell; nothing either introduces escapes (only `nextId` advances). |
| Lookbacks (`did`) and `would` clauses | Step the action as an event pattern (`actionPattern`); see "Historical events" above. |
| Activated abilities and special actions | Step the cost after dropping its local X (`dropLetter`); the body starts in a new clause after it and reads every mention the cost made, wherever the cost moved it: whether the engine can find the object at runtime is the engine's concern [CR#608.2h]. Timing and guard clauses read the body's output. A special action's `player`, when written (one an effect allows another player, [CR#116.2c]), is stepped first, as its subject. |
| Granted, token, emblem and keyword abilities | Embedded ability text is stepped in `linkedTextContext`: only chosen qualities, chosen players and, for Participial Descriptions only, the objects the enclosing ability acted on carry in: text on a token, emblem or permanent the source created or put onto the battlefield [CR#607.1d], an ability granted to the object the granting ability is printed on [CR#607.1a], and a grant onto another object, read as Oracle writes it (*Massimo, the Magician*). Its performer is its own controller. A keyword ability's body is stepped in its own Reference Scope (`enterOwnScope`). |
| Card text versus nested lists | `Ability.checkText` threads line-level choices, additional- and alternative-cost results, added targets and the objects a printed triggered or activated ability acted on (`Ability.textContext`); a keyword's `body` is threaded the same way (`checkTextWith`); `Ability.checkAll` checks sibling abilities in one supplied context. |
| Printed boxes | Literal-only; they do not read discourse. Effect-written stat slots instead step their Amount expressions. |
| Named-card choice domain | The card-description predicate is stepped in its own Reference Scope (`enterOwnScope`) at object kind and publishes nothing. Other choice-domain payloads use the consumer's context. |

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
| `source` | No Head Noun and no carrier: an object referent introduced by a description that names it a source (`Referent.introducedAsSource`: "a source of your choice", "whenever a source deals damage"), in any zone [CR#609.7a]. The `thatSource` Template ("that source") is `that source`; it never reads the Source of the text, nor an object introduced under any other description (owner ruling 2026-10-22). |
| `ability`, `player`, `pile` | The corresponding payload category. |
| `stack`, `join` | The existing shared-stack and joined-reference disciplines, including their kind constraints. |

`wordReaches` applies refinements to the same referent. `halfReaches` applies
the whole refined word within each join half; it cannot take a type from one
half and copy origin from another. `stampedWordOk`, a Participial Description's
Head Noun read, retains the deed stamp and event-carrier checks, and receives
origin so a copied refinement can compose with that read. Ordinary self
exclusion, plurality, uniqueness counting, and re-stamping policy are unchanged.

[Proofs/NounWords](Semantics/Proofs/NounWords.lean) pins matching identities,
incompatible types/carriers, original versus copied abilities, ambiguity,
join-half isolation, and provenance. Its three general commutativity theorems
cover type/copy refinement order in direct, half, and stamped matchers.
These refinements have no referent effects; this does not assert commutativity
of arbitrary Predicate modifiers.

### Remembered type evidence

Referents preserve card-type facts as canonical, duplicate-free lists. A conjunction
unions its written card types; their order does not select a grammatical head.
An artifact creature supports both type words [CR#205.2b]. `[]` means no known
card type, not a claim that the referent has no types. Explicit card-type evidence
is retained before the existing subtype-derived fallback; a creature subtype does
not add a creature fact to an explicitly Kindred description. The separate
`Predicate.seedType` presupposition calculation is not stored type evidence.

A same-kind disjunction retains only facts common to every alternative. Joined
kinds keep separate evidence in each `Payload` half, with intersection
within each half's alternatives. A read must satisfy all refinements within one
half. The legacy flattened `Payload.ty` projection skips nonobject halves; it
must not be used to borrow evidence across halves. Choice-arm payloads and
same-kind group summaries also intersect their facts; narrowing a group or
remarking a tested referent adds facts to that referent.

Type alternatives retain conjunction/disjunction structure: conjunction combines
facts, and disjunction requires each alternative to support a type word's reach. An
entirely unknown description supplies no evidence. Attachment profiles collect every type
refinement, including when seeding or moving their referent.

Movement retains the remembered facts and changes the carrier separately.
A battlefield type word then stops matching, while a correspondingly refined
card word can match the card in exile [CR#109.2,109.2a,110.1]. This is evidence
for reference resolution, not a simulation of continuous characteristic changes.

`Proofs/TypeEvidence.lean` proves `pureTypePermutation` for arbitrary lists of
pure card-type modifiers, including duplicates and the empty list. Its concrete
pins cover both type words, missing types, common versus alternative evidence,
joined halves, attachment profiles, and movement. This does not assert
that arbitrary modifiers with referent effects commute.


The source inventory is the indexed `data` declarations and parameterized
records in `idris/src/Experimental/*.idr`, including proof datatypes. An
indexed proof datatype names a rule below; a context-indexed syntax datatype
names the stepping entry point that must receive its field's context. Names
in the first column identify the reference evidence, not Lean APIs to extend.
Plain unindexed data/records impose no erased-index duty. `KeywordShapes` and
`FactsGen` have no indexed syntax. `Unspellable` is a proof utility, replaced
by exact-refusal theorems; it is not an authorable term. Unless a row names
another file, the Lean entry points are in [Phrase.lean](Semantics/Check/Phrase.lean);
"steps" means the entry point steps the field's references and checks its
expected child kinds in the incoming `DRS`.

### Card

| Reference datatype | Lean duty |
|---|---|
| `CardLine` | `Characteristics.cardLineOk`/`lineLaws` ([Card.lean](Semantics/Check/Card.lean)) check the card-type and subtype relationships. |
| `CardBox` | `cardBoxOk` takes face side, types and text together; it checks printed literals, type fit, and characteristic-defined absent slots. |
| `CharacteristicsLaws` | `CardFace.check` supplies the side and checks line, text, box and cost. |
| `SharedLineHalfLaws` | `SharedLineHalf.check` receives the shared characteristics and checks each half against them. |
| `LevelBandLaws` | `LevelBand.check` receives the inner characteristics and checks the range, text, and box. |
| `LevelBandsLaws` | `Card.check`'s leveler case checks every band, frame eligibility (`levelerFrameOk`) and `bandsDisjoint`. |
| `PrototypeAltLaws` | `prototypeAltCheck` receives the inner characteristics and checks the alternate cost and box. |

### Effect

| Reference datatype | Lean duty |
|---|---|
| `SpendPurpose` | `stepSpendPurposeWith` steps it. |
| `ManaHeld` | The `manaRetention` static case steps its player; `ManaHeld.localChecks` ([LocalShape.lean](Semantics/Check/LocalShape.lean)) checks its shape. |
| `AsThough` | `stepAsThoughWith` steps it. |
| `Exchanged` | `stepExchangedWith` steps it. Whether an exchanged value can be set is not checked (`lean-v3-authoring-impl-4-checker-dewarting`). |
| `WrittenQuality` | Shared by a bundle's qualities and `CharacteristicEdit.quality`: `stepWrittenQualityWith` steps it (`stepCharacteristicBundleWith` each quality of the bundle in order) and `WrittenQuality.shape` checks a chosen quality is read as a chosen one. |
| `CounterKindSource` | `stepCounterKindSourceWith` steps it; `CounterKindSource.localChecks` checks the counter is known. |
| `CostShift` | `stepCostShiftWith` steps it; `CostShift.localChecks` checks its shape. |
| `CountBound` | No Lean type: a usage limit's count is an `Amount` (`UsageLimit.mk times per`), stepped by `stepUsageLimitWith`. |
| `DeedComplement` | Retired with `deonticPatientOk` (`lean-v3-authoring-impl-7-checker-and-regime-retirement`): a counterpart's deed membership, kind and identity with the subject are not checked beyond ordinary reference resolution. |
| `DeonticPatient` | The old deed/role indices were parameters shared by all constructors. A counterpart is stepped as any reference; that it is not the subject read back (`deonticPatientOk`) is no longer checked (`lean-v3-authoring-impl-7-checker-and-regime-retirement`), and which deeds a patient form, a bound, an "as though" or a play rider goes with, and a counterpart's class, card types and zone, are not checked (`lean-checker-sweep-costs`). |
| `PreventCut` | `stepPreventCutWith` steps it, the cut of a `preventDamage` or a `redirectDamage`. |
| `DividedVerb` | `stepDividedVerbWith` steps it. |
| `DividedTakes` | `Action.distribute` checks the recipient kind (an object or a player, `kindLte`) or object kind according to the closed `DividedVerb` constructor; which objects damage reaches is not checked (`lean-checker-sweep-zone`). |
| `CtrlOverrideOk` | `MoveRider.under` checks player kind. Retired by `lean-checker-sweep-zone` (§7 charter ruling 2026-10-06): the single-controller demand is not checked. |
| `ProducedMana` | `stepProducedManaWith` steps it (`ProducedMana.check` in [AbilityRules.lean](Semantics/Check/AbilityRules.lean)); `ProducedMana.localChecks` checks its runs. |
| `Repetition` | `stepRepetitionWith` steps it. |
| `EachStackOk` | A distributive handoff or loop keeps every outer referent with its updates and pluralizes or aggregates only what its body introduced (`instruct`, `doForEach`). |
| `DeckReadable` | Retired (`lean-v3-authoring-impl-7-checker-and-regime-retirement`): `checkDeckTraitWith` and `checkDeckConditionWith` step a deck condition's predicates from an empty context at object kind; whether a predicate reads a characteristic of a card in a deck is not checked. |
| `DeckComparable` | Retired (`lean-v3-authoring-impl-7-checker-and-regime-retirement`): which quality a deck's cards agree on is not checked. |
| `QualityPayload` | An authoring input in `Semantics.Primitives` that expands to edits (below); it is not checked syntax. |
| `TokenSpec` | `stepTokenSpecMetadataWith` steps it. |
| `StaticSpec` | `stepStaticSpec` (`StaticSpec.check`) steps it. |
| `Compulsion` | No Lean type: requirements are `StaticSpec.must` clauses, stepped as a deontic. |
| `PlayPayment` | `stepPlayPaymentWith` steps it (`PlayPayment.check` in [AbilityRules.lean](Semantics/Check/AbilityRules.lean)). `payingInstead` is the alternative cost of a cast a permission grants from another zone or at another time, or that an instruction directs; a pure alternative cost, granting no cast, is `StaticSpec.altCost` [CR#118.9,601.3]. |
| `DeonticRider` | No Lean type: the rider is retired (`lean-v3-authoring-surface` §2.3). |
| `MoveRider` | `stepMoveRiderWith` steps it (`MoveRider.check` and `MoveRider.checkAll` in [AbilityRules.lean](Semantics/Check/AbilityRules.lean)). Whether its destination takes an Entry Rider or a Pile Placement is not checked (`lean-v3-authoring-impl-4-checker-dewarting`). An `attachedTo` host is an object or a player (`kindLte`); a negated rider is named only in an event pattern (`riderNegationPattern`). |
| `Cost` | `stepCost` (`Cost.check` in [Abilities.lean](Semantics/Check/Abilities.lean)) steps it; `Cost.localChecks` checks its shape. |
| `ManaRider` | `stepManaRiderWith` steps it; `ManaRider.localChecks` checks its shape. |
| `CopyExcept` | `stepCopyExceptWith` steps it (`CopyExcept.check` in [AbilityRules.lean](Semantics/Check/AbilityRules.lean)). |
| `RollRow` | `stepRollRowListWith` checks each row's nonzero literal range and steps its body. |
| `Action` | `stepAction` (`Action.check`) steps it. `search scope for_` steps `scope`, then `for_` in the patient role; after the event, what `for_` introduced is in the searched zone (if the scope names one) with the `search` stamp (`applySearchFound`). An unqualified search's `for_` is `any`, which introduces nothing. |
| `Instructions` | The length is the actual list length; `sequentially` refuses an empty list (`nonEmpty`) and steps each member in a new clause after the previous output. No independent length value remains. |
| `SimInstructions` | The length is the actual list length; `simultaneously` refuses an empty list and withholds earlier siblings' outcomes from later siblings. |
| `KeywordParam` | `checkKeywordParamsWith` steps each argument against the keyword's schema (`keywordSchemaFor`). |
| `AbilityLost` | `stepAbilitySelectionWith` steps each carried keyword term (`knownKeywordTerm`) or written ability (in the granted context). |
| `AbilityAt` | `Ability.check` dispatches each ability constructor, its expected phrase kinds, and its explicit local contexts. |
| `StaticParts` | The actual list supplies length; `StaticSpec.conjunction` refuses an empty list, refuses a nested conjunction (`notCoord`) and steps each part in a new clause after the previous one. |
| `CostSeq` | The actual list supplies length; `Cost.compound` refuses an empty list and steps each cost after the previous one. |
| `AddedPayment` | Retired by `lean-checker-sweep-costs` (§7 charter ruling 2026-10-06): `StaticSpec.addedCost` and an alternative cost (`StaticSpec.altCost` or `PlayPayment.payingInstead`) no longer require `Cost.offBattlefield`; the card-class rule that still read it, and `Cost.offBattlefield` with it, is deleted (`lean-v3-by-card-class`). |
| `AbilitySeq` | `Ability.checkText` threads the choices and additional-cost results introduced by earlier lines; nested ability lists use `Ability.checkAll` at their enclosing context. |

### Events

| Reference datatype | Lean duty |
|---|---|
| `LookbackSubject` | `Predicate.did` binds the described participant as `which`; the event stepper validates each use (see "Historical events"). |
| `LookbackComplement` | Ordinary event participant checks; there is no separate complement-kind table. |
| `ChoiceMode` | `choiceModeChecks` in `stepDetPhraseWith`. |
| `Possessable` | Retired by `lean-checker-sweep-zone` (§7 charter ruling 2026-10-06): a possessed shared zone is not refused. A referent records no possessor (`lean-v3-authoring-impl-4-checker-dewarting`). |

### Phrase

| Reference datatype | Lean duty |
|---|---|
| `ChoiceInScope` | `Predicate.notChosen` reads the partitive choices and the choice history (`countParts`, `DRS.choiceWasPossible`). |
| `ColorTerm` | `stepColorTermWith` steps it. |
| `LibPlace` | Merged into `Location` (`Location.fromEnd`, stepped by `stepLocationWith`); the place's arrangement and ordinal fit are retired by `lean-checker-sweep-zone`. |
| `Location` | `stepLocationWith` steps it (`Location.check` in [AbilityRules.lean](Semantics/Check/AbilityRules.lean)): a zone's owner is a player, a place from a pile's end has a nonzero ordinal, a made pile is a written pile mention, the cards exiled with a source name a link source, and a coordination (`Location.coord`, with "and", "or" or "and/or") has at least two members (a search's members name distinct zones). |
| `NameSource` | The `named` predicate case requires one standing card-name choice for `chosen` (`choiceRef`). |
| `ChoiceDomain` | `stepChoiceDomainWith` steps its contents, and `sortedDomainCheck` ([AbilityRules.lean](Semantics/Check/AbilityRules.lean)) checks them against the choice's sort. |
| `EventComplement` | Retired in favor of event participant, source, destination, and locus fields. |
| `ComplementWritten` | Required event fields cannot be omitted. An action that declares a locus requires that locus in the event stepper. |
| `LookbackClause` | `stepDidClauseWith` steps the carried action under the clause's subject as an event pattern. |
| `Predicate` | `stepPredicate` (`Predicate.check`, `Predicate.checkAt`) steps each child under the head's kind (`relativeKind`) and uses `kindCheck`; the children of an "and" coordination share a scope, and the alternatives of an "or" or "and/or" coordination branch with their own inferred kinds. |
| `DetPhrase` | `stepDetPhraseWith` steps it. |
| `RolesOk` | The `oneEachOf` noun case steps each role's predicate and checks object kind; an empty role list is `nonEmpty`. Whether the roles are distinct (`rolesOk`) is not checked (`lean-v3-authoring-impl-7-checker-and-regime-retirement`). |
| `Noun` | `stepNounPhrase` (`NounPhrase.check`, `NounPhrase.checkAt`) receives the expected kind and steps references in the incoming `DRS`. An "and" or "and/or" coordination (`NounPhrase.coord`) steps its conjuncts in order and then publishes a plural group referent over them (`pushCoordinatedGroup`), which a later "them" reads; each member the coordination introduced is a part of the group (`Referent.partOf`), so a group reading counts it once (`wholeGroups`: bare pronoun winners, Participial Description reads, group counts and "the rest"/"the other" groups). A coordination whose introduced members are all parts of one group is itself a part of that group. |
| `Amount` | `stepAmount` (`Amount.check`) steps it. |
| `Quantity` | `stepQuantityWith` steps it. |
| `NonZeroQ` | `Quantity.nonZero` (`nonZeroQ`) in the consumers that require nonzero cardinality; zero remains allowed in other quantity slots. |
| `SliceCount` | `stepSliceCountWith` steps it. |
| `ComplementAnchor` | `NounPhrase.anchorPhrase` and singularity in the `otherThan` predicate case (`complementAnchor`), for a printed "other than <NP>" only. A bare "another"/"other" (`Predicate.other`) writes no anchor; the `other` predicate case resolves its Contrast Anchor (`DRS.contrastAnchor?`, `semantics-v3.md` §0.1). A mention of the Source coordinated before the phrase is its anchor (`coordinatedSelf`). Otherwise: (1) the nearest earlier mention in scope compatible with the head in kind, Card Type, zone and controller restriction (`controlFits`), never the subject of the `is` it is directly predicated of (`contrastExcluded`, cleared for nested noun phrases), a referent a linked ability published, a referent a Condition introduced (`Referent.fromCondition`), or a mention of the Source or "you" (`Referent.defaultAnchor`); (2) failing that, the first compatible particular (a target, a definite description or an attachment host) the clause introduces after its agent, when "another <X>" is the agent of a `would`, event, deontic, handoff, copy, characteristic-change or text-change clause (`clauseParticipants`); (3) otherwise the Source, for an Object head in a zone the Source may be in and not restricted to a controller other than "you" (an instant's or sorcery's is the stack; any other Source's is unknown), whatever its Card Types, or "you", for a Player head [CR#109.5]. A head that may be an Object or a Player never falls back to "you". It refuses `contrastAnchor` when none fits. A written granted ability is read with unknown Source types and the granting text's objects excluded (`grantedContext`). Two "other"s in one conjunction refuse `otherAnchored`. |
| `LinkSource` | `NounPhrase.linkSource` in the `exiledWith` location case (`linkSource`). |
| `PileMention` | `NounPhrase.pileMention` plus noun stepping in the `pile` location case (`pileMention`). |
| `PaidSubject` | Object-kind checking in the `paid` amount case. Retired by `lean-checker-sweep-zone` (§7 charter ruling 2026-10-06): the stack-zone demand is not checked. |
| `DestOk` | Retired by `lean-checker-sweep-zone` (§7 charter ruling 2026-10-06): a move's destination shape is not checked. |
| `FlipScope` | `stepFlipScopeWith` steps it. |
| `IgnoredOutcomes` | `stepIgnoredOutcomesWith` steps it. |
| `Ballot` | `stepBallotWith` steps it (`Ballot.check` in [AbilityRules.lean](Semantics/Check/AbilityRules.lean)). |
| `CostSubject` | Retired by `lean-checker-sweep-zone` (§7 charter ruling 2026-10-06): a cost change's subject is not required to be on the stack. |
| `StackActOn` | Retired by `lean-checker-sweep-zone` (§7 charter ruling 2026-10-06): retargeting and copying do not demand a stack object, and counter's enacted move checks no source zone. |
| `Condition` | `stepCondition` (`Condition.check`) steps it. |
| `MarkingOk` | Retired with `CondMarking.unless_` (`lean-v3-authoring-impl-14-ast-and-authoring-cleanup`): "unless C" is `onlyUnless`, the condition `not C` under `ifSo`, so there is no marking to agree with its condition. |
| `TokenPhrase` | `NounPhrase.tokenPhrase` plus object-kind checking in the token-creation event (`tokenPhrase`). |
| `Exposed` | The `expose` action case steps it. |
| `VisibleThing` | `stepVisibleThingWith` steps it. |
| `DamageRecipient` | The `dealDamage` action case checks the recipient's kind alone (an object or a player, `kindLte`). Which objects can be dealt damage [CR#120.1a] and where they are are not checked (`lean-checker-sweep-zone`). |
| `StatusHolder` | The `setStatus` action case requires the object kind; the battlefield zone and the settable status value are not checked (`lean-checker-sweep-zone`). |
| `NotAnAbility` | Retired at movement: the action-fold ticket explicitly corrects this false blanket prohibition; ending a turn or combat can exile stack abilities [CR#724.1b,724.2b]. |
| `DiscardOk` | Discard is an enacted move. Retired by `lean-checker-sweep-zone` (§7 charter ruling 2026-10-06): its source zone is not checked. The authoring boundary above excludes arbitrary authored tag/body combinations. |
| `Movable` | `Action.move` checks the moved kind against `object ∨ pile` (`kindLte`), so objects (including abilities) and piles move and players and quality values are refused. The destination's riders, arrangement and card types are not checked (`lean-checker-sweep-zone`). |

### Triggers

| Reference datatype | Lean duty |
|---|---|
| `ManaTypeTerm` | The `tapForMana` action case steps a colored mana type's `ColorTerm`. |
| `Duration` | `stepDurationWith` (`Duration.check` in [AbilityRules.lean](Semantics/Check/AbilityRules.lean)) steps its boundary events and condition; a boundary that names whose turn names one definite player (`durationPossessor`). |
| `TurnRef` | `stepTurnRefWith` (`TurnRef.check`): a named player is a player, and `that` reads back exactly one turn an earlier phrase introduced (`turnAntecedent`). |
| `UsageLimit` | `stepUsageLimitWith` (`UsageLimit.check`) steps the count's amount; its period names no referent. |
| `AttackDefender` | No Lean type: an attack's defender is an optional noun phrase of the `attack` actions, which must be singular. |
| `DamagePatient` | No Lean type: a damage event's recipient is the `dealDamage` action's, stepped in its patient slot. |
| `Door` | `stepDoorWith` steps it. |
| `RollWatch` | `RollWatch.localChecks` ([LocalShape.lean](Semantics/Check/LocalShape.lean)) checks its shape. |
| `CrimeSubject` | `Action.commitCrime` as an event pattern (`does subj commitCrime`) requires a singular performer (`singular`). |
| `GameEvent` | `stepGameEvent` (`GameEvent.check`) steps it. |
| `Causing` | `stepCausingWith` steps it. "By an effect" and "if an effect would" are `GameEvent.causes .anEffect` and `WouldClause.causedBy .anEffect` (Template `byAnEffect`) [CR#609.1,614.16]; no Action carries an effect flag, since whether an Action is an effect follows from the node that encloses it (a turn-based action is none, [CR#703.1]). |
| `TriggerWindow` | Merged into `Timing.inPart`; `stepTimingWith` steps the possessor's references. Which turn parts and possessors a window admits is not checked (`lean-checker-sweep-timing`). |
| `Timing` | `stepTimingWith` (`Timing.check`) checks references and kinds only: whose turn is a player's, and the events, conditions and turn references it names. |
| `AltEvent` | `stepTriggerHeaderWith` steps each alternative event, and `commonHeaderContext` keeps what every alternative introduces. The old header-word index is gone; the event determines its own event class. Header nontargeting is retired by `lean-checker-sweep-game-rules` (§7 charter ruling 2026-10-06). |
| `JoinedHeader` | `stepTriggerHeaderWith` steps each joined header with its own alternatives, "while" and timing. |

### Words

| Reference datatype | Lean duty |
|---|---|
| `OptOk` | Optional consumers step a present value and skip an absent one; absence introduces no payload to check. |
| `Joins` | `joinKinds`, `Predicate.kind?` and `NounPhrase.kind?` derive the resulting kind; there is no independently supplied result index. |
| `KnownAct` | `knownAct` at deed consumers reads the deed facts; the Definition is trusted to supply the corresponding expansion. |
| `Payload` | `Payload.kind` derives the kind from the constructor; `Referent` has no separate kind field that can disagree. The checker-private handoff frame (`Payload.actor`) is not a discourse referent: reflexives, resolution and the published referents pass over it. |
| `HeadTy` | No Lean type: `NounPhrase.ty` and `Payload.ty` derive lists of known card-type facts; joined kinds retain evidence per half. |
| `DieSides` | `DieSides.localChecks` ([LocalShape.lean](Semantics/Check/LocalShape.lean)) checks its shape. |
| `Targetable` | `Kind.targetable` at determiner, copying and target consumers (`targetable`). |
| `Targeter` | `Kind.targeter` in the `targets` predicate case and the deontic patients (`targeter`). |
| `Phrasal` | `Kind.phrasal` at copying, iteration and noun consumers. |
| `KnownKeyword` | `knownKeyword`, `KeywordTerm.known`, and `keywordParamFits` at keyword consumers. |
| `BasicLandType` | The Idris reference's `basicLandLine` helper has no Lean counterpart. No caller can supply a proof-erased list to that removed API; `SubtypeSpace.basicLand` is a closed constructor. |
| `BasicLandTypes` | Same removed `basicLandLine` API as `BasicLandType`; a printed face's subtype validation remains `subsFitLine` (the card frame). A token's or an edit's subtypes are not checked against its types (`lean-checker-sweep-game-rules`). |
| `Delta` | The type parameter survives as `Delta α`; it was not erased. `stepDeltaWith` steps a `Delta Amount`'s carried amount. |
| `GivingWarrant` | Retired with the conferrer tag (2026-09-07): which expansion confers a designation is not tracked. `Action.gainDesignation` checks the designation's own row: its scope and holder (`designationScope`, `designationHolder`). Whether an action may confer it is not checked (`lean-checker-sweep-game-rules`), and the facts table no longer carries the `effectful` column (`lean-v3-authoring-impl-7-checker-and-regime-retirement`). |
| `DesignationHolder` | `designationHolderOk` (the designation is held by a player or an object) and the declared designation scope checks at designation consumers; where the holder is is not checked. |
| `AxesAt` | `axesAt` in the `compare` predicate case requires a nonempty list whose axes all have the expected scope. |
| `StatusVal` | The status is the value; no separate category argument can disagree. Consumers retain the status-holder checks; a description holding two contradictory statuses is not refused. |

### Records and dependent pairs

- `InstrProfile bs`: the old `bs` parameter did not constrain any record field; a
  step returns the output `DRS` and its refusals, with no separate profile record.
- `TokenChars bs`: `stepCharacteristicBundleWith` and `stepCharacteristicsWith`
  step the carried expressions, power, toughness, loyalty and defense in slot order.
  Nested characteristic text is stepped in `linkedTextContext`.
- `Binding`'s kind/payload pair: the kind is now derived from the payload
  (`Referent.kind`), so there is no separate index to validate. Alternative arms keep what
  they share (`commonHeaderContext`).
- Other erased implicit binders on functions, templates and definitions refer to these same
  datatype duties; erasing the binder does not remove the obligation at the
  resulting term's consumer. In particular, templates and definitions are not an independent
  certificate for arbitrary enacted bodies; the authoring boundary above establishes their trusted origin.


### Retention

`StaticSpec.retention spec subject` ("This effect doesn't remove …") exempts `subject` from one
state-based consequence of `spec` [CR#702.16n,702.16p,303.4c,704.5m]. Beside the
Characteristic-Defining Ability, it is the other game rule the checker enforces outside the card
frame, by the owner's ruling (`lean-v3-effect-keeps-existing-attachments`, 3a):
`retentionClause` refuses a `spec` that is not one clause removing Attachments from its host (a
protection grant, or a prohibition on attaching; a conjunction is refused, the exemption being one
clause's), and `retentionSubject` a `subject` that is not "this Aura" (or Equipment, or
Fortification) on that host, nor a description of Attachments whose "attached to …" reads the
host back by an anaphor. Applying the exemption, and reading "already attached to" as the effect
starts to apply [CR#702.16p], are the engine's.

### Characteristic edits and keyword argument schemas

`StaticSpec.characteristicChange` and `CopyExcept.edits` carry the same typed
`CharacteristicEdit` list. A static change of stats ("gets +P/+T", "has base power and
toughness P/T", "its power is equal to …", "switch its power and toughness") and of abilities
("has flying", "has all activated abilities of …", backup's selection) is a
`characteristicChange` too. "Gets +P/+T" is one `getsPt` carrying both printed deltas, a "+0"
included; the single-stat `modification` template is only for a change whose text names one
stat. A Characteristic-Defining Ability is its own form,
`StaticSpec.characteristicDefining subject edits` [CR#604.3a] (templates `ptDefinition`,
`ptDefinitions`, `qualityDefinition`; changeling and devoid are written with it): it functions in
all zones [CR#604.3], applies first in layer 4 or 5 [CR#613.3] and in layer 7a for power and
toughness [CR#613.4a], and its power and toughness sets fill a printed `*`
(`StaticSpec.definedStats`). It is the one game rule the checker enforces outside the card frame,
by the owner's decision (`lean-v3-authoring-impl-14-ast-and-authoring-cleanup`), because the form
is the rules classification: `cdaEdit` refuses an edit other than a color, subtype, power or
toughness definition, `cdaSubject` a subject other than the object itself, `cdaGranted` the form
inside a grant or an established effect, and `cdaConditional` a condition on it
[CR#604.3a]. Its stat edits each name their number, which a later edit reads back ("its toughness
is equal to that number plus 1"). A stat edit's `sets` stands below zero and its
`adds` or `loses` is clamped (`CharacteristicEdit.numberSlots`); an "each" subject distributes
over the whole change; a granted keyword whose number is a letter introduces it. `TypeLineChanges` uses optional axes: `none` leaves
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

`QualityPayload` is an authoring input in `Semantics.Primitives`, not a
checked syntax field. It expands to edits, its op written once over them.
Ordinary additions add their type line but set every other written value; a
removal removes each name, color list, ability and mana cost, and writes a number (power,
toughness, loyalty, defense) as lowered by it, `stat .loses`, there being no removal of a
number. A retained type ("It's still
a land") adds the type line whatever the op [CR#205.1b]; no edit stores the
retained type. `copyCharacteristics` expands the old
packed input into one edit block and separate ability exceptions. The former
minimum-two-fields packing rule has no semantic counterpart in an edit list.

`Ability.keyword` takes an ordered `List KeywordParam`. The hand-maintained
`KeywordFacts.argumentSchemas` (`Check/Facts.lean`) enumerate admissible orders
and each quality argument's domain; the checker does not branch on keyword
names. In particular, qualified cost forms retain their object-quality
constraint and cost-only alternative. Quality arguments can read the incoming
context; subject arguments, numbers, costs, and deck conditions retain their
closed contexts. Argument lists do not publish referents between elements.

### A keyword defined by card class: `byCardClass`

A keyword that means one ability on a permanent and another on an instant or
sorcery spell [CR#702.55a,702.131a,702.131b,702.174b] writes its two halves as
`Ability.byCardClass onPermanent onInstantOrSorcery` (ruling 2026-10-22/23). Gift
keeps its shared cost before the pair, `byCardClass (when (does thisPermanent enter)
…) (spell (doIf …))` [CR#702.174a,702.174b]; haunt's pair is its dies trigger and its
trigger on the spell's being put into a graveyard during its resolution
[CR#702.55a]; ascend's is a static ability and a spell ability
[CR#702.131a,702.131b]. No half carries a card-class guard.

- **Engine.** The card has one half, the one its Card Class selects:
  `onPermanent` on a permanent card, `onInstantOrSorcery` on an instant or sorcery
  card (`cardClassOf`, `Words.lean`) [CR#110.4,110.4a]. Once
  `lean-v3-card-type-composite-definition` lands, the class is computed from the
  card type's fields.
- **Checker.** Both halves are checked, each in the same incoming context
  (`Check/Phrase.lean`, the `byCardClass` arm), so a choice an earlier ability
  publishes (gift's chosen player [CR#607.2d]) resolves in both, and a half wrong on
  its own is refused whatever the other is. Neither half publishes to the text after
  the pair, since that text is checked for either class.

"During its resolution" and "as it resolves" (haunt's spell half, buyback, rebound
[CR#702.27a,702.55a,702.88a]) write the move as caused by the spell's resolution, over
the `resolve` Procedure and its Definition: `GameEvent.causes (.event (does .any
(resolve thisSpell))) <move>` on a trigger event, `WouldClause.causedBy (.event …)
<clause>` on a replacement's would-clause. The resolution causes the move, which is
its final part [CR#608.2n], and the one form serves a trigger event and a would-clause
alike; `Timing.whileDoing` states only a co-occurrence and has no slot on a
would-clause. A countered spell's move (`counter`) carries no such cause [CR#701.6a].
The move itself is `putInto thisSpell graveyard (from_ := stack)`, so its observation
point is computed: arrival, as a spell put into a graveyard from the stack is none of
the look-back zone changes [CR#603.10a].

- No ability is checked against the card's class. The card-class lint
  (`classAbilityOk`, `cardTextOk`, the `cardText` refusal, `typesCombinable`) is
  deleted (ruling 2026-10-22/23, extending the batch-4 ruling that removed the
  keyword gate and superseded the 2026-08-22 ruling that kept one): a keyword or
  ability on the wrong card class is a card-authoring mistake, not a text-meaning
  error. A kindred card still has another card type [CR#308.1]
  (`kindredHasAnotherType`, part of `cardLineOk`).
  The parts of a keyword's definition are not checked against the class:
  the definition-fit, stack-regime and class-guard checks (`keywordBodyFits`,
  `KeywordFacts.definition`, `StackRegime`, `ComponentFacts`, `ClassGuard`) are
  retired (`lean-v3-authoring-impl-7-checker-and-regime-retirement`),
  and the facts table no longer carries a definition, a regime, `functionsOnStack`,
  `counterEligible` or a card-class column.
- Pins: `Semantics/Proofs/KeywordByCardClass.lean` (gift's, haunt's and ascend's
  halves on both card classes, both halves checked, ascend's tree),
  `Semantics/Proofs/GiftVariants.lean` (gift's spelled variants on both card classes)
  and `Semantics/Proofs/DuringResolution.lean` (the resolution cause and computed
  observation point on haunt, buyback and rebound).

### The player chosen as a cost was paid

Gift's first ability is a cost, "As an additional cost to cast this spell, you
may choose an opponent" [CR#702.174a], and its second hands the effect to the
chosen player [CR#702.174b,702.174d..702.174i]. The two are linked
[CR#607.2d,607.2i], and the second reads the choice as "the chosen player"
(`the isChosenPlayer`, `.chosenPlayer .theChoice`), the words a choice in its own
ability is read by: `Ability.textContext` publishes the cost's choice to the
abilities after it, across gift's body as across a card's text. It requires exactly
one player choice in scope (`countChoice .player`), refused
`choiceRef theChoice player n` otherwise; in gift's body the cost is the only one.

- Pins: `Semantics/Proofs/LinkedChoiceReadback.lean` (one, none and two choices,
  both class-guarded halves, the keyword body) and `Semantics/Proofs/GiftVariants.lean`.

### Linked Facets of a Link Source, linked objects, and declared Relations

`PaidFacet` and `PaidCostName` are `LinkedFacet` and `LinkSource`
(`semantics-v2.md` §7, ruling 2026-10-09, "A linked ability reads back a
Linked Facet of a Link Source"). A Link Source names a cost; the facets read its
payment, among them `LinkedFacet.taken which`, whether the choice `which` offered
was taken ("if tribute wasn't paid" [CR#702.104b]), an amount facet named when its
source is (`linkedFacetNamed`).

The objects a linked ability acted on are read by a Participial Description, written
as the same ability would write it of its own deed ("the exiled card", "the
sacrificed creature" [CR#607.2a]): linked and self-linked abilities share one AST.
`Ability.textContext` publishes them `linked`: an additional or alternative cost's
results (emerge's `StaticSpec.altCost` cost, harmonize's cast permission's
`PlayPayment.payingInstead`, offering, and a cast permission's added cost,
`PlayPayment.plusAdditional`), the target `addsTarget` adds, stamped
by targeting so mutate's "the target" is its own spell's [CR#702.140a] (a conditional static
publishes what its spec does, so mutate's target, required only once its cost is chosen
[CR#601.2c], is still read back), what a
replacement does `also` (devour [CR#702.82b]), and the objects a printed triggered or
activated ability itself introduces, in whatever zone it put them, never what its trigger
event names [CR#607.1]. Each step is published once; at a branch point each arm is stepped from the
state before it and its objects carry the arm (`Referent.linkArms`), so arms taken instead of
each other read as one place (champion [CR#702.72a]); a Definition's own scope is read whole,
along each path through it (`Action.arms`). A keyword ability's body publishes into the
card's text like any other line: hideaway's "the exiled card", read by a printed ability,
depends on it, and cumulative upkeep's sacrifice of its own permanent is likewise visible to
the lines after it, to a Participial Description only. Resolution refuses a description
whose deed or Head Noun no published object fits ("the exiled creature" when a card
left a graveyard) and one two abilities could supply [CR#607.2a]. Where the object is
when the linked ability resolves (a card that left exile) is the engine's, not the
checker's.

A Relation (`RelationLabel`) is declared in
`Semantics/Definitions/Relations.lean`, as a designation is in
`Designations.lean`, and `Definition.relation label` (`Rules.lean`) is its record. A card names a
declared relation or none: the constructor is an `internal_expansion`, so the authoring boundary
refuses an undeclared one. `Action.relate` checks both operands as objects, so a player operand is
refused `kindMismatch`; every relation links two objects, and neither the label nor its record
gives a side another kind.
`Predicate.related` steps its counterpart and `NounPhrase.counterpart` its
subject, and neither looks the label up. These are kind and reference checks;
when a relation ends [CR#702.95e,702.99c] is the engine's.
Pins: `Semantics/Proofs/LinkedReadback.lean`.

### The ability printed with a keyword, and its qualifiers

The ability a keyword is printed with ("Boast — [Cost]: [Effect]") is the
keyword's parameter, `KeywordParam.ability`, and the restriction the keyword
adds is a qualifier on it, `Ability.qualified ability timing limit guard
from_` (`semantics-v2.md` §7, ruling 2026-10-09) [CR#113.10a,113.6b]. A
qualified ability keeps the wrapped ability's category (`Ability.category`).

- The guard reads "as long as", "this ability triggers only if" or "activate
  only if" by the wrapped ability's category [CR#702.169b..702.169d]; the
  checker checks it, the timing and the limit for kinds and references only.
- Which ability a limit is written on is not checked (`qualifiedLimit` is
  retired, `lean-v3-authoring-impl-7-checker-and-regime-retirement`).
  A triggered ability's limit is written on its header.
- "Activate only once" is a limit over `Period.thisObject`, reset when the
  object changes zones [CR#602.5b,702.177a].
- `AbilitySelection.printedAfter keyword` selects the abilities printed below
  a keyword's line [CR#702.165a,702.165c], granted by
  `CharacteristicEdit.addedSelectionOf source selection`. A selection
  naming a keyword the card does not print is refused as a reference of the
  card's text (`printedAfterKeyword`, `printedAfterLaws`); the walk follows
  the forms a grant is written in, as `namesThisDoor` does.
- `PlayerStat.speed` is a player's speed [CR#702.179a]; its game state is
  `engine-speed`'s.

Pins: `Semantics/Proofs/AbilityQualifiers.lean`.

### Movement destinations

`Action.move` requires a destination. Every move checks its destination's
references and the moved kind (an object or a pile), and publishes the stated
destination. The destination's shape, the arrangement's fit to the moved
plurality, the subject's card types and the riders' fit to the destination are
game rules the checker does not assert (`lean-checker-sweep-zone`). There is
no destinationless movement action.

"Put" and "return" name no deed: each is ordinary English for a Zone Change
[CR#400.7]. The put and return Templates (`putInto`, `put`, `putWith`,
`putOntoBattlefield*`, `returnTo`, `returnToBattlefield*`, `returnToHand`) keep
their wordings and expand through one helper to a bare `.move`, so a put and a
return with the same subject, origin, destination and riders are one tree
(`putAndReturnExpandToOneTree`). The written origin, not the verb, decides the
Observation Point, as described below for `putInto`
(`graveyardToHandObservedOnDeparture`). Every move stamps what it moved with
its Zone Change (`Stamp.zoneChange`: the zone it left, if known, and the zone it
entered), whether or not a deed names the move; of the bare moves, only a move
to exile is also stamped by a deed, exiling [CR#701.13a].

User ruling (2026-09-06): Counter is one enactment of the Counter label around
stack-to-graveyard movement, uniformly for its spell or ability subject. This
supersedes the prior optional-destination design and its ability-specific branch.
The enacted movement no longer checks its stack source (`lean-checker-sweep-zone`). The rule defining
Counter supplies stack removal and a spell's graveyard destination [CR#701.6a];
this uniform expansion is the workbench's chosen representation, not an added
runtime lifetime model.

Owner ruling (2026-10-22): the 2026-09-06 ruling stands, and Counter is not split
by subject. An ability on the stack behaves as a token does: it stops existing
once it leaves its home zone, however that zone change happens [CR#608.2n,704.5d].
That an ability countered to the graveyard then ceases is the engine's lifetime
rule, so one stack-to-graveyard enactment serves a spell and an ability alike, and
engine consistency outweighs a subject-dependent special case for [CR#701.6a]'s
"A countered spell is put into its owner's graveyard". This answers the
question `lean-v3-countered-ability-ceases` put: its split is not made.

An ability referent stores its zone, initially the stack. Movement updates that
zone while retaining its ability identity and copy origin. A stack reference
resolves on actual stack evidence; counter and copy check no zone. Element referents and
unions retain location evidence rather than restoring a hardcoded stack location.

`Action.clearDamage` removes all marked damage from a permanent. Its
subject must be an object; where it is is not checked, and it need not
currently be a creature [CR#120.6]. It publishes the ordinary subject reference and no damage
outcome. Regeneration composes this primitive with tapping and conditional
removal from combat inside a destruction replacement. `Action.markDamage` [CR#120.3e]
publishes the damage marked (`damageMarked`), which a replacement of the marking reads back.

Damage rules are compositions (owner ruling 2026-10-22). A prevention, redirection or scaling
effect is a replacement of a damage event, `replace (would <source> (dealDamage …))
(instead …)` [CR#614.1a,615.1]: `instead (preventDamage cut (also := …))` prevents it, its
additional effect stepped after the prevention publishes what it prevented [CR#615.5];
`instead (redirectDamage cut to)` deals it to one other permanent or player, `to` singular
(`singular`) and of that kind (`kindLte`) [CR#614.9]; `instead (instruct it (dealDamage …))`
has the source deal a modified amount instead [CR#614.1a,614.6], twice that much when
it doubles [CR#701.10g]. A cut is stepped before the prevention
publishes its outcome. With no damage named (`damage := none`), either action acts on "that
damage", the damage in scope, and refuses `damageDealtInScope` without one; the prevention
turns that damage into the damage prevented (`DRS.withDamagePrevented`), so "that much" in its
additional effect reads one amount. "Can't be prevented" and "can't be dealt instead" are
prohibitions over these actions, `cant any (preventDamage .all (damage := some clause))`,
both of them deontic and event heads [CR#615.12]. A prevention records the event its additional
effect's "this way" reads (`did any (preventDamage …) thisWay`). An `instead` over a damage event
is that event modified [CR#614.6]: its `dealDamage` keeps the event's kind, and a recipient left
unwritten is the event's. A damage pattern's recipient is matched member by member, so a group
recipient refuses `perMember` only in an instruction.


### Template and definition parameters and expanded actions

A `semantic_template` or `semantic_definition` takes ordinary Lean parameters, plus
`splice T` parameters for supplied bodies. A splice parameter is closed in its
caller's scope (`CallerSplice.close`, from `CallerScope`): the body the caller wrote
reads the caller's referents, and a parameterized body (`α → β`) receives the
receiving declaration's subjects through its noun parameters. Every other parameter
is substituted as written. A Definition wraps each argument it reads in `inCaller`
(`CallerScope.caller`: `NounPhrase.inCaller`, `Amount.inCaller`, `Action.inCaller`, …
for every phrase type that can be an argument) and binds a referent argument at its
first mention with `firstMention`, reading it back with `laterMention`
("Binding identity and a keyword body's own scope" above). The Binding Identity is named
`<declaration>.<parameter>` (`destroy.subject`, `fight.other`), and the identity is outermost:
`firstMention id (inCaller arg)`, `laterMention id (inCaller arg)`, since the identity is the
declaration's and only the argument is read in the caller. Where a mention stands in the body
never changes a role the caller can see (`leaveOwnScope`, below), so "Target creature fights
target creature. Tap it." reads the fight's source (`fightSourceIsSelectedBySubjectPriority`). A Template that names one
argument twice does the same (`dealDamageOwnPower`, `controllerSacrifices`). An argument
is therefore selected once, at its textual first use, however often the body names it,
and two identical argument expressions remain two selections. An Amount argument is
an expression read wherever the body writes it, so it observes outcomes introduced
before each read. No construction-time context survives in the result: the generated
`AuthoringContext` only tracks lexical depth.

`actor` passed as an argument is read as `actor` itself: it introduces nothing, so
inside a handoff it is the handed performer (`capturedActorIsTheHandedPlayer`).
An argument the body never writes is never stepped and introduces nothing
(`unusedSubjectDoesNotIntroduceAReferent`); one it writes is stepped where it stands,
and a malformed reference does not gain validity by being an argument. Any required availability
guard belongs in the ordinary template or definition body. `Proofs/CallerScope.lean`
pins these laws: textual first use, one selection per parameter, separate selections
for identical arguments, identity through nested templates, loss of identity on a
zone change, supplied bodies that keep their caller's pronouns and update the
original referent, and amount reads that observe new outcomes.

The `fight` Definition binds the other creature and its performer at their first
mentions inside an ordinary creature guard (`doIf` over `matches … isCreature` for
both), then has each deal damage equal to its power to the other simultaneously
[CR#701.14a..701.14c]. Group subjects are rejected by the singular power read
and per-member damage checks. The creature guard is also its battlefield guard:
"creature" with no zone or "card" names a creature on the battlefield [CR#109.2].

Fight's guard reads its participants as they are when the fight happens: if either
is no longer on the battlefield or no longer a creature, neither fights or deals
damage [CR#701.14b] (`fightGuardTestsBothCreaturesNow`). That is fight's rule; the
general rule for an effect's information about an object is [CR#608.2h]. A
present-tense `doIf` condition (and a `doOnlyIf`'s postposed one) over a
description that names no zone and no "card" ("if it's a creature") reads the
permanent as it is, since Oracle text writes a departed object's information in
the past tense ("if it was a creature"). A description that names a zone or "card"
tests the object where it is: *Bioplasm*'s `is itCard isCreature` holds of the
exiled creature card (`okBioplasmCardTest`). The checker does not evaluate a guard;
this is what the lowering reads it as, and evaluating it is the engine's.

Regeneration installation creates a next-time-this-turn destruction replacement.
The Regenerate label belongs to its application: clear marked damage, have the
controller tap the permanent, and remove it from combat if applicable. A static
regeneration replacement uses that application repeatedly; it does not install
a further shield [CR#701.19a..701.19c].

"It can't be regenerated", printed with no duration after a destroy, is later text
that modifies the meaning of the destroy (`doUnderProhibition`,
`Duration.modifiesPrecedingAction`): a prohibition in force only for the destroy's
own event, so regeneration shields do not apply to that destruction
[CR#608.2c,701.19c] (`destroyCantRegenerateRidesTheDestroy`; owner ruling
2026-10-22). It is no separate continuous effect, and it stays its own clause, so
its subject ("It", "That creature", "A creature destroyed this way") is a mention
read back from the destroy. A printed "this turn" keeps its duration
(`doAndForbid`, `printedThisTurnProhibitionLastsTheTurn`).

Duration convention for `establish` (owner ruling 2026-10-22, pinned by
`durationConventionWitnesses`): `none` is no printed Duration, so the effect lasts until
the game ends [CR#611.2a] (*Vexing Shusher*, *Boseiju*). A [CR#608.2c] rider that modifies the
preceding action is `some .modifiesPrecedingAction` (*Terror*; *Pinpoint Avalanche*'s "The damage
can't be prevented"). A printed "for the rest of the game" is `some restOfGame` (epic, *Screaming
Nemesis*). Because the rider is neither `none` nor `restOfGame`, it binds the
destroy's event and not the creature: a regeneration shield created after a destroy that did not
destroy the creature (it is indestructible) still works [CR#701.19c]
(`indestructibleVictimKeepsLaterRegenerationShield`). Which shields exist
when the destroy happens is the engine's.

The `counter` Definition expands to one labeled stack-to-graveyard move in its
own scope, `ownScope (enact counter (move subject stack graveyard …))`.
It reads the subject once and needs no ability/spell branch.
The movement pre-state remains available to postposed conditions such as
Ertai's Trickery.
An alternative may also re-read the primary clause's stated amount, as in
Caustic Bronco's "otherwise ... that much"; this is a numeric reference, not
evidence that the primary branch actually happened.

Converting is transforming written with another verb [CR#701.28a]: a "transforms into" trigger
sees either and "can't transform" forbids either [CR#701.27e,701.28f], so `convert` is a Template
whose tree is `transform`'s (`convertIsTransform`), with no Deed of its own.

Losing counters (`loseCounters`) is the shared counter removal (`Action.removeCounters`)
performed from a player, with its removed-counter outcome.

Moving counters (`moveCounters`) is a removal from the source followed by a placement on the
destination, each publishing its outcome [CR#122.5]. Two parts of that rule are not modelled:
the move is all or nothing (if either half is impossible, no counter is removed or put), and,
with no kind written, what is put is a counter of each kind on the source (`sameAs`) rather
than exactly the counter removed.

## Rules tables

A rules table's row supplies its own subject: the scope predicate binds `this`,
so the row's condition, effect and conferred ability are checked in the empty
context, exactly as a printed face's text is. That a state-based row
targets nothing [CR#704.1,601.2c] is a game rule the checker no longer asserts:
the `nontarget` refusal is retired here as on a static ability
(`lean-checker-sweep-game-rules`).

Two representation boundaries are named rather than hidden. A conferral confers
an `Ability` or an ability-free property (`Conferral`, as a registry definition's does);
whether an ability is one an effect could grant (`grantable`) is retired with the grant rules.
The results of damage are property conferrals, each an `asEvent` over the damage dealt to what
the row scopes [CR#120.3a,120.3c,120.3e,120.3g,120.3h], so removing abilities leaves them; a
result's counter is a declared one when the row is authored (the authoring boundary), and which
holder the counter's declaration gives it is not checked. Toxic is one row reading the total toxic value
[CR#702.164b,702.164c].

A counter's kind is a name: `CounterKind.named` carries the counter
declaration's name verbatim (`p1p1Counter`, `flyingCounter`, `chargeCounter`),
and "counters with the same name or description are interchangeable"
[CR#122.1], with what the counter is placed on (`holder`, an object unless the declaration
says a player). Each kind is a declaration in `Semantics/Definitions/Counters.lean`, a +X/+Y
counter [CR#122.1a] and a keyword counter [CR#122.1b] included, and the constructor is an
`internal_expansion`, so a card names a declared kind or none; `CounterKind.scope` reads the
value's own holder. A counter definition
(`Definition.counter`, `Rules.lean`) states what ONE counter confers, and the
model applies it once per counter held: the +1/+1 counter's conferral is
"+1/+1" (`boost`, one `characteristicChange`), a keyword counter's is its keyword
(`grants`). No conferral reads the count of its own kind. That reading is the
model's: `Definition.check` checks each conferral once in the empty
context and computes no multiplicity, so nothing in the checker depends on it
(`Proofs/Rules.lean`, `p1p1CounterCountMultipliedTwin`, which checks clean).

A definition's conferral is checked as the same syntax is inside a card
(`Conferral.check`, `Check/Rules.lean`). There is no bearer exemption: since
`lean-checker-sweep-zone` no stat change, a card's or a definition's, demands
that its subject be on the battlefield, so a +X/+Y counter's conferral needs no
narrowing to count on a creature card in any zone [CR#122.1a]. The +1/+1 and
-1/-1 definitions check clean (`p1p1CounterDefinitionZone`,
`m1m1CounterDefinitionZone`).

The registry-definition emitter (`cargo xtask lean definitions`) is retired
(`lean/README.md`); `Proofs/Rules.lean` proves `Definition.check = []` by
`decide` for the Registry Definitions it writes. `Definition.check` has a rule
for every constructor, an exhaustive match, so there is no definition the
checker ignores.

A designation is a name too: `DesignationLabel.named` carries the designation
declaration's name verbatim (`goaded`, `monarch`, `leftHalfUnlocked`), or, for
a declaration that lists members, a member's (`day`, `night` [CR#731.1]); the
phrase a card prints ("the monarch") is the declaration's spelling and never
reaches the checker. The label also carries its `scope` (who holds it), which gives `holder`,
`possessorOk`, `heldByItsCard` and `gameWide`; each designation is a declaration in
`Semantics/Definitions/Designations.lean`, and the constructor is an `internal_expansion`, so a
card names a declared designation or none. A designation
declaration's bare name means its label, so it reads at a designation
position; its definition (`Definition.designation`) is a separate record the
registry holds under the name (ruling 2026-10-10).

The two lethal-damage state-based actions [CR#704.5g,704.5h] have no row: the
grammar projects no marked damage and has no "since state-based actions were
last checked" lookback. `Stat.loyalty` and `Stat.defense` read by zone:
counters on the battlefield [CR#306.5c,310.4c], the printed number elsewhere
[CR#306.5a,310.4a]. The state-based and damage rows read the counters
directly; the entry conferrals [CR#306.5b,310.4b] read the stat,
`statOf (.stat .loyalty|.defense) .this`, which off the battlefield already is
the printed number. There is no printed-number projection: when an entering
object is read is the engine's concern (owner ruling 2026-10-22; ADR §0.1 item 5).

## Shared event patterns and static applicability

A zone-change event is `GameEvent.does subj` over an
`Action.move subject from_ to at_ cause riders`, whose subject is the clause's
own (`NounPhrase.actor`) in the verb templates: it names the subject, origin,
optional destination, and observation point (`ObservationPoint`: `arrival` or
`departure`). Its checking and historical mention order is subject,
origin, destination. An endpoint can refer to a player introduced by an earlier
field. A missing endpoint is unconstrained; empty origin sets, exclusion of every
origin, and movement from a single shared zone back into itself are refused.
Player-qualified exclusions are conservative when only a zone sort is known.
A pattern's riders describe how the object would enter, whether or not an
effect wrote the rider. A listed rider matches when it describes the entering,
and a negated rider (`MoveRider.not r`) matches an entering that `r` does not
describe. Before [CR#303.4f]'s choice, an entering is attached to a host H only
when its effect names H. That is why enchant's choice watches only an entering
that names no host (`enchantChoiceExcludesANamedHost`). A pattern's
`.attachedTo .any` matches any host the effect names, whether it is still
defined or not, so an undefined host takes the [CR#303.4i] path. A negated
rider outside a pattern is refused (`riderNegationPattern`): an action's riders
are what its text writes, so outside a pattern there is nothing for a negation
to deny.

The verbs `die`, `leave` and `enter`, which move the clause's own subject
(`NounPhrase.actor`), and `putInto` are templates. Battlefield and graveyard
departure patterns observe on departure; ordinary arrivals observe on arrival.
In particular, graveyard arrival from anywhere is not a battlefield-departure
pattern [CR#603.6c,603.10a]. The raw observation point retains the distinction.
The put and return Templates take it from `Location.observedOnDeparture`, which
classes a written origin list as departure-observed when any of its origins is
(an ability that watches a permanent leave the battlefield is a
leaves-the-battlefield ability whatever else it watches [CR#603.6c]), and the
checker reads a move's own observation point: only a departure-observed move
from the battlefield reads its patient as the permanent it was, so a mixed
origin list cannot be read historically while it is classed as an arrival
(`mixedOriginWithTheBattlefieldIsObservedOnDeparture`,
`mixedOriginReadsThePermanentOnlyOnDeparture`).
Interception publishes departure location; a triggered continuation publishes
arrival location. Historical observation publishes mentions without moving current
references or manufacturing an event outcome. Source and destination possessors
are available in ordinary field order.

Damage event patterns share optional source and patient fields and require at
least one participant. Named active/passive authoring forms remain templates; no
semantic voice field is present. Entry and attachment choice specs likewise share
an occasion enum while retaining choice sort, domain, and disclosure.

A counter event's amount or quantity, and a roll's count, is what its pattern watches; no batch
field is carried (owner ruling 2026-10-21). "A counter is put" is `.lit 1`, "a counter is removed"
`exactly 1` and "you roll a die" `.lit 1`, each matched by every counter or die, since one event
can contain several occurrences [CR#603.2c]; "one or more" is `anyAmount` or `orMore 1`, matched
by the event [CR#714.2b]; "the last counter is removed" is `Quantity.theLast`
[CR#702.62a,310.12b], and "the Nth counter is put" is `Amount.theNth`, the put that takes its
holder from fewer than N counters of the kind to N or more [CR#122.7], not an `nthOccurrence`
count of puts. Only a plural amount or quantity publishes the count (`countersPut`,
`countersRemoved`, `diceRolled`) that "that many" or "the result" reads, and nothing reads a
pattern's amount as an exact count. No Oracle text prints "two or more counters are put"; writing
it would need a `Quantity` on `putCounters`, which takes an `Amount`.

A replacement effect's "the first time … each turn" is the ordinal of the event it watches for
[CR#614.1], `WouldClause.nthOccurrence`, as a trigger's is `GameEvent.nthOccurrence`; it is not a
Usage Limit, though it counts within the same `Period`. An event ordinal counts from one, never
within `Period.thisObject`, and spells "each turn" `Period.turn`, never `part .turn`
(`Refusal.ordinalNonZero`, `Refusal.eventOrdinalPeriod`). Which event is the first is a fact of the turn's history, the engine's to count.

A Roll Modifier (`StaticSpec.rollModifier roll modify limit`, the `after` Template: "After you
roll a die, you may remove a +1/+1 counter from this creature. If you do, increase or decrease the
result by 1.") is a static ability, not a trigger: it changes a roll's natural result before the
result is final [CR#706.2,706.2a], and a trigger would resolve only after the result was used
[CR#113.3c]. It steps its roll event, which announces the result `shiftResult` reads, then its
modification in a new clause, then its Usage Limit (`okAfterRollShift`, `badShiftWithoutRoll`).

A control change named as an event, a lookback or a forbidden act is `Action.changeControl
subject from_` (`gainControlOf`), with the player control is taken from when the text says "from"
(a trigger on control taken from a player looks back in time [CR#603.10d]). In a resolving position it is refused `controlChangePattern`: the instruction "gain
control of" is `establish (controlGrant …)` [CR#611.2a,613.1b]. The pattern stamps its patient
with `.core .gainControl`, as the grant does, so one Participial Description reads either.
`Proofs/Counters.lean`, `Proofs/Anaphora.lean` and `Proofs/EventPatterns.lean` pin these.

`partScope` expands to conditional applicability with `Condition.duringPart`.
The condition reads the ordinary conditional context, including the spec's
subject and numeric definitions. A whole-turn condition is meaningful even with
no possessor. Any stated possessor must denote a player in an admissible number.

Off-battlefield applicability is written as explicit noun selections. Controlled
permanents, controlled spells, and owned cards outside the battlefield are
separate domains. Celestial Dawn includes every controlled spell; Encroaching
Mycosynth includes permanent spells. A placeless card description and its
published referent stay placeless until positive zone evidence constrains them.
Subtype-family edits require a hosting card type, not battlefield membership.

The prototype priority applies: the discarded placement-spelling prohibitions,
wrapper-specific `notExtended` refusal, and accidental battlefield evidence on
placeless card/source referents are not compatibility contracts. Their affected
witnesses are re-spelled or corrected in the landing record.
