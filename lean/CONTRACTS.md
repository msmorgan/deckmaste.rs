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
an edit's chosen quality is `qualityRead`. Six stay as kind, plurality or term
shape: `designationScope` (a designation's holder sort), `facesFit`, and the registry
and table shape checks `tokenNamed`, `definitionHolder`, `definitionNamed` and
`definitionScoped`; `manaSymbolOk` and `twoParties`, kept then, are retired with the
phrase-level game rules below. The
fourteen card-frame rules (`cardLine` through `prototypeFrame`) are kept: the owner
decided so on 2026-10-08 (§7 ruling 2026-10-06, consequence (d)), because they check
that a term is a well-formed card, not what an action does. Zone state on referents stays, because references resolve
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

Semantic expressions must use authored declarations: a `semantic_definition`
(a Definition, the rules-defined vocabulary of a keyword action, keyword
ability, counter, token or designation) or a `semantic_template` (a Template,
a transparent phrasing the rules do not define). Printed-data records,
enumerated leaf values, and numeric literals remain ordinary data; their
semantic fields are inspected recursively. The modules are layered
`Semantics.Primitives` ← leaf catalogs ← `Semantics.Templates` ← composite Definitions, all
re-exported by `Semantics.Definitions`. The leaf catalogs
(`Semantics.Definitions.Counters`, `.Designations`, `.Relations`, `.Subtypes`) are spelled over
primitives only; the composite Definitions (`Semantics.Definitions.Actions`, `.Keywords`) are
spelled over Templates. The checker sits beside this stack, not under it: Primitives, the leaf
catalogs and Templates import none of `Semantics.Check` (the structural readers they need, such
as `Location.sort`, live with the Core AST), and only the composite Definitions read a checker
reader (`NounPhrase.plur`, for "It/They can't be regenerated"). A word a Template sort and a
Definition share (`crew`, `saddle`) is dispatched on its expected type where the Definition is
declared, so Templates name nothing above them. A declaration whose bare name another
declaration or a syntax word already takes is named in its sort's namespace (`Subtype.army`,
`Subtype.infinity`, `DesignationLabel.solved`, `Ability.splice`).
`Semantics.Primitives` provides the generated constructor wrappers
(`declare_semantic_primitives`, `Semantics.Primitives.<Type>.<name>`), which
have definitionally identical constructor signatures and are registered as
templates, together with a few worded primitive templates and the
characteristic inputs. `NounPhrase.pro` and `NounPhrase.which` have no
primitive wrapper: they are expansion carriers (`internal_expansion`) that only
templates and definitions write. `Semantics.Templates` holds every
`semantic_template` and names leaf catalog entries rather than spelling their strings;
the `Semantics.Definitions` submodules hold every `semantic_definition` and the few
templates whose bodies need a composite definition: the templates over its actions
follow the actions, and a keyword's licensed phrasings follow the keyword.
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

A lookback is a `DidClause`: a `Subject`, an `Action` and a `LookbackSpan`, either a
`duration` (with an optional `Timing` the action happened under) or an occurrence
(`thisWay`, the event the enclosing action caused; `triggering`, the event that
triggered the ability), which names no span. `Condition.did`, `Predicate.did` and
`Amount.tally` carry one. `stepDidClauseWith` steps the action as an event pattern
(`actionPattern`) after the clause's subject, so it publishes its participants without
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
resolves only to a unique winner; none or a tie is refused `anaphor`. A demonstrative
(`.word _`, `.tokenBorn`, `.unionHalf _`) follows `AC>R1`: it also excludes clause-mates
in an argument slot and any referent a bare pronoun already resolved to in this clause
(`pronounResolved`), then takes the most recently introduced candidate. A stamped
participle (`.verbed …`) must match exactly one referent. A reflexive
(`NounPhrase.refl`) resolves to the current clause's subject (`resolveReflexive`).
Possessors and other nested mentions record no direct-argument role, so "target
creature's controller sacrifices it" reads the creature. `nextClause` commits the
pending roles, advances `currentClause` and clears `pronounResolved`.

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
toughness amount after its power amount.

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
characteristic text and emblem abilities are stepped in `choiceContext` (only chosen
qualities and chosen players carry in, never a handoff frame), and a keyword ability
body in its own Reference Scope (`enterOwnScope`).

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
`id` is refused `bindingIdentity`, never read as a second selection. A zone change
gives the moved object a fresh referent without the identities [CR#400.7], so a later
mention of the pre-move parameter is refused; updates in place keep them.

A keyword action body is read in its own Reference Scope: `Action.ownScope deed body`,
which every `Action` Definition opens with (`semantic_definition` refuses one that does
not). `enterOwnScope` pushes an `OwnScopeFrame` that masks every referent of the calling
text except the performer the innermost handoff names, and opens a new clause. The
body's parameters are written `inCaller`: `enterCaller` pops only the top frame, so the
argument is read in the immediate caller's view, with any enclosing Definition's masks
still in force, and `leaveCaller` unmasks what the argument introduced or resolved and
records it as caller-owned (`callerReferentIds`). `leaveOwnScope` pops the frame, drops
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
| `withContinuation` (`may` / `doIfDone` templates) | The body, the success arm after the body, and the failure arm from before the body each open a clause; only the optional body's shortfall and repeat count, and the choice evidence, escape. |
| `doIf` (`doIf` / `doOnlyIf` templates) | `doIf` first announces the then-branch's targets, chosen as the spell or ability is put on the stack [CR#601.2c]; the condition reads them, wherever it is printed (`doOnlyIf`, "counter target spell if …"), but nothing else the branch would choose or create. It then steps the then-branch in a new clause after the condition, whose output continues when there is no else-branch. The else-branch reads the then-branch as announced (its targets and the amounts it names, "otherwise, each opponent loses that much life"); with it, only the then-branch's targets escape. |
| `doForEach`, `doForEachKind` | Push one singular loop referent (or one quality) at the header clause and step the body in the next clause; on exit keep only outer referents, with their updates, and publish the loop's choice history, explicit target results and created results as plural aggregates (`publishLoopTargetResults`, `publishLoopCreatedResults`); `doForEachKind` also publishes each outcome sort its body published, once and as a group (`publishLoopOutcomes`), so doubling each kind of counter puts counters. |
| `enact` | The performer in context (`actorShape?`) is checked at the deed's agent kind; a deed performed by a permanent with no handoff to one gets a frame for the source permanent. Post-event, the body's patients receive the deed's stamp (`applyInPlaceStamp`). |
| `instruct` (handoff) | See "The performer and the handoff" above. |
| `ownScope`, `inCaller` | See "Binding identity and a keyword body's own scope" above. |
| `delay`, `triggerReflexively` | Step the inner body in a new clause after `settleTargets`; nothing the inner body introduces escapes (only `nextId` advances). |
| `replace`, `holdUntil` | `replace` steps the replaced action as a pre-event pattern and the replacement after it in a new clause; `holdUntil` steps the held action, then its event. The static `replace` steps its `would` clause, then the replacement in a new clause, and publishes only the letters the clause and the replacement introduce, so a later "where X is" defines them [CR#107.3i]. A replacement steps what it makes of the event: `instead` and `also` step their action; `withRiders` steps its riders, refused (`riddenEntering`) unless the replaced event is an entering, a move to the battlefield or a token's creation, and (`entryRidersOnly`) for a Pile Placement among them [CR#614.1c,614.12]; `onlyIf` steps its condition, then its body in a new clause after it, as `doIf` steps its then-branch. A caller's replacement steps in one caller frame, its condition and action together. The static `replace` also publishes the choices its `also` action makes to later lines, as "As this enters, choose a color" [CR#614.1c]. |
| Lookbacks (`did`) and `would` clauses | Step the action as an event pattern (`actionPattern`); see "Historical events" above. |
| Activated abilities and special actions | Step the cost after dropping its local X (`dropLetter`); the body starts in a new clause that keeps only the cost's public mentions (`publicOnly`). Timing and guard clauses read the body's output. |
| Granted, token, emblem and keyword abilities | Embedded ability text is stepped in `choiceContext`: only chosen qualities and chosen players carry in, so its performer is its own controller. A keyword ability's body is stepped in its own Reference Scope (`enterOwnScope`). |
| Card text versus nested lists | `Ability.checkText` threads line-level choices and public additional-cost results (`Ability.textContext`); `Ability.checkAll` checks sibling abilities in one supplied context. |
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
| `ability`, `player`, `pile` | The corresponding payload category. |
| `stack`, `join` | The existing shared-stack and joined-reference disciplines, including their kind constraints. |

`wordReaches` applies refinements to the same referent. `halfReaches` applies
the whole refined word within each join half; it cannot take a type from one
half and copy origin from another. `verbedWordOk` retains the deed stamp and
event-carrier checks, and now receives origin so a copied refinement can
compose with that read. Ordinary self exclusion, plurality,
uniqueness counting, `Reach.tracksObject`, and re-stamping policy are unchanged.

[Proofs/NounWords](Semantics/Proofs/NounWords.lean) pins matching identities,
incompatible types/carriers, original versus copied abilities, ambiguity,
join-half isolation, and provenance. Its three general commutativity theorems
cover type/copy refinement order in direct, half, and verbed matchers.
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
kinds keep separate evidence in each `HeadTy`/`Payload` half, with intersection
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
| `TokenQuality` | Now `WrittenQuality`, shared by a bundle's qualities and `CharacteristicEdit.quality`: `stepWrittenQualityWith` steps it (`stepCharacteristicBundleWith` each quality of the bundle in order) and `WrittenQuality.shape` checks a chosen quality is read as a chosen one. |
| `CounterKindSource` | `stepCounterKindSourceWith` steps it; `CounterKindSource.localChecks` checks the counter is known. |
| `CostShift` | `stepCostShiftWith` steps it; `CostShift.localChecks` checks its shape. |
| `CountBound` | No Lean type: a usage limit's count is an `Amount` (`UsageLimit.mk times per`), stepped by `stepUsageLimitWith`. |
| `DeedComplement` | Retired with `deonticPatientOk` (`lean-v3-authoring-impl-7-checker-and-regime-retirement`): a counterpart's deed membership, kind and identity with the subject are not checked beyond ordinary reference resolution. |
| `DeonticPatient` | The old deed/role indices were parameters shared by all constructors. A counterpart is stepped as any reference; that it is not the subject read back (`deonticPatientOk`) is no longer checked (`lean-v3-authoring-impl-7-checker-and-regime-retirement`), and which deeds a patient form, a bound, an "as though" or a play rider goes with, and a counterpart's class, card types and zone, are not checked (`lean-checker-sweep-costs`). |
| `DamageScope` | `stepDamageScopeWith` steps it. |
| `DamageAgent` | `stepDamageAgentWith` steps it. |
| `Unpreventable` | The `preventionBan` static case steps its described damage. |
| `PreventCut` | `stepPreventCutWith` steps it. |
| `DamageScale` | `stepDamageOpWith` steps a shifted scale's amount. |
| `DividedVerb` | `stepDividedVerbWith` steps it. |
| `DividedTakes` | `Action.distribute` checks the recipient kind (an object or a player, `kindLte`) or object kind according to the closed `DividedVerb` constructor; which objects damage reaches is not checked (`lean-checker-sweep-zone`). |
| `CtrlOverrideOk` | `MoveRider.under` checks player kind. Retired by `lean-checker-sweep-zone` (§7 charter ruling 2026-10-06): the single-controller demand is not checked. |
| `ProducedMana` | `stepProducedManaWith` steps it (`ProducedMana.check` in [AbilityRules.lean](Semantics/Check/AbilityRules.lean)); `ProducedMana.localChecks` checks its runs. |
| `Repetition` | `stepRepetitionWith` steps it. |
| `EachStackOk` | A distributive handoff or loop keeps every outer referent with its updates and pluralizes or aggregates only what its body introduced (`instruct`, `doForEach`). |
| `DeckReadable` | Retired (`lean-v3-authoring-impl-7-checker-and-regime-retirement`): `checkDeckTraitWith` and `checkDeckConditionWith` step a deck condition's predicates from an empty context at object kind; whether a predicate reads a characteristic of a card in a deck is not checked. |
| `DeckComparable` | Retired (`lean-v3-authoring-impl-7-checker-and-regime-retirement`): which quality a deck's cards agree on is not checked. |
| `QualityPayload` | An authoring input in `Semantics.Primitives` that expands to edits (below); it is not checked syntax. |
| `TokenSpec` | `stepTokenSpecWith` steps it. |
| `StaticSpec` | `stepStaticSpec` (`StaticSpec.check`) steps it. |
| `Compulsion` | No Lean type: requirements are `StaticSpec.must` clauses, stepped as a deontic. |
| `PlayPayment` | `stepPlayPaymentWith` steps it (`PlayPayment.check` in [AbilityRules.lean](Semantics/Check/AbilityRules.lean)). |
| `DeonticRider` | No Lean type: the rider is retired (`lean-v3-authoring-surface` §2.3). |
| `DamageOp` | `stepDamageOpWith` steps it. |
| `MoveRider` | `stepMoveRiderWith` steps it (`MoveRider.check` and `MoveRider.checkAll` in [AbilityRules.lean](Semantics/Check/AbilityRules.lean)). Whether its destination takes an Entry Rider or a Pile Placement is not checked (`lean-v3-authoring-impl-4-checker-dewarting`). |
| `Cost` | `stepCost` (`Cost.check` in [Abilities.lean](Semantics/Check/Abilities.lean)) steps it; `Cost.localChecks` checks its shape. |
| `ManaRider` | `stepManaRiderWith` steps it; `ManaRider.localChecks` checks its shape. |
| `CopyExcept` | `stepCopyExceptWith` steps it (`CopyExcept.check` in [AbilityRules.lean](Semantics/Check/AbilityRules.lean)). |
| `RollRow` | `stepRollRowListWith` checks each row's nonzero literal range and steps its body. |
| `Action` | `stepAction` (`Action.check`) steps it. |
| `Instructions` | The length is the actual list length; `sequentially` refuses an empty list (`nonEmpty`) and steps each member in a new clause after the previous output. No independent length value remains. |
| `SimInstructions` | The length is the actual list length; `simultaneously` refuses an empty list and withholds earlier siblings' outcomes from later siblings. |
| `KeywordParam` | `checkKeywordParamsWith` steps each argument against the keyword's schema (`keywordSchemaFor`). |
| `AbilityLost` | `stepCharacteristicEditWith` steps each carried keyword term or chosen-ability reference. |
| `AbilityAt` | `Ability.check` dispatches each ability constructor, its expected phrase kinds, and its explicit local contexts. |
| `StaticParts` | The actual list supplies length; `StaticSpec.conjunction` refuses an empty list, refuses a nested conjunction (`notCoord`) and steps each part in a new clause after the previous one. |
| `CostSeq` | The actual list supplies length; `Cost.compound` refuses an empty list and steps each cost after the previous one. |
| `AddedPayment` | Retired by `lean-checker-sweep-costs` (§7 charter ruling 2026-10-06): `StaticSpec.addedCost` and an alternative cost no longer require `Cost.offBattlefield`; the card-class rule (`classAbilityOk`) still reads it. |
| `AbilitySeq` | `Ability.checkText` threads the choices and public additional-cost results introduced by earlier lines; nested ability lists use `Ability.checkAll` at their enclosing context. |

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
| `ChoiceInScope` | `Predicate.notChosen` reads the partitive choices and the choice history (`countParts`, `DRS.choiceWasPossible`); `Condition.choseThisWay` requires a compatible standing choice. |
| `ColorTerm` | `stepColorTermWith` steps it. |
| `LibPlace` | Merged into `Location` (`Location.fromEnd`, stepped by `stepLocationWith`); the place's arrangement and ordinal fit are retired by `lean-checker-sweep-zone`. |
| `Location` | `stepLocationWith` steps it (`Location.check` in [AbilityRules.lean](Semantics/Check/AbilityRules.lean)): a zone's owner is a player, a place from a pile's end has a nonzero ordinal, a made pile is a written pile mention, the cards exiled with a source name a link source, and a disjunction has at least two disjuncts (a search's disjuncts name distinct zones). |
| `NameSource` | The `named` predicate case requires one standing card-name choice for `chosen` (`choiceRef`). |
| `ChoiceDomain` | `stepChoiceDomainWith` steps its contents, and `sortedDomainCheck` ([AbilityRules.lean](Semantics/Check/AbilityRules.lean)) checks them against the choice's sort. |
| `EventComplement` | Retired in favor of event participant, source, destination, and locus fields. |
| `ComplementWritten` | Required event fields cannot be omitted. An action that declares a locus requires that locus in the event stepper. |
| `LookbackClause` | `stepDidClauseWith` steps the carried action under the clause's subject as an event pattern. |
| `Predicate` | `stepPredicate` (`Predicate.check`, `Predicate.checkAt`) steps each child under the head's kind (`relativeKind`) and uses `kindCheck`; conjunction children share a scope, alternatives branch with their own inferred kinds. |
| `DetPhrase` | `stepDetPhraseWith` steps it. |
| `RolesOk` | The `oneEachOf` noun case steps each role's predicate and checks object kind; an empty role list is `nonEmpty`. Whether the roles are distinct (`rolesOk`) is not checked (`lean-v3-authoring-impl-7-checker-and-regime-retirement`). |
| `Noun` | `stepNounPhrase` (`NounPhrase.check`, `NounPhrase.checkAt`) receives the expected kind and steps references in the incoming `DRS`. |
| `Amount` | `stepAmount` (`Amount.check`) steps it. |
| `Quantity` | `stepQuantityWith` steps it. |
| `NonZeroQ` | `Quantity.nonZero` (`nonZeroQ`) in the consumers that require nonzero cardinality; zero remains allowed in other quantity slots. |
| `SliceCount` | `stepSliceCountWith` steps it. |
| `ComplementAnchor` | `NounPhrase.anchorPhrase` and singularity in the `otherThan` predicate case (`complementAnchor`). |
| `LinkSource` | `NounPhrase.linkSource` in the `exiledWith` location case (`linkSource`). |
| `PileMention` | `NounPhrase.pileMention` plus noun stepping in the `pile` location case (`pileMention`). |
| `PaidSubject` | Object-kind checking in the `paid` amount and noun cases. Retired by `lean-checker-sweep-zone` (§7 charter ruling 2026-10-06): the stack-zone demand is not checked. |
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
| `Causing` | `stepCausingWith` steps it. |
| `TriggerWindow` | Merged into `Timing.inPart`; `stepTimingWith` steps the possessor's references. Which turn parts and possessors a window admits is not checked (`lean-checker-sweep-timing`). |
| `Timing` | `stepTimingWith` (`Timing.check`) checks references and kinds only: whose turn is a player's, and the events, conditions and turn references it names. |
| `AltEvent` | `stepTriggerHeaderWith` steps each alternative event, and `commonHeaderContext` keeps what every alternative introduces. The old header-word index is gone; the event determines its own event class. Header nontargeting is retired by `lean-checker-sweep-game-rules` (§7 charter ruling 2026-10-06). |
| `JoinedHeader` | `stepTriggerHeaderWith` steps each joined header with its own alternatives, "while" and timing. |

### Words

| Reference datatype | Lean duty |
|---|---|
| `OptOk` | Optional consumers step a present value and skip an absent one; absence introduces no payload to check. |
| `Joins` | `joinKinds`, `Predicate.kind?` and `NounPhrase.kind?` derive the resulting kind; there is no independently supplied result index. |
| `KnownAct` | `knownAct`/`knownActs` at deed consumers read the deed facts; the Definition is trusted to supply the corresponding expansion. |
| `Payload` | `Payload.kind` derives the kind from the constructor; `Referent` has no separate kind field that can disagree. The checker-private handoff frame (`Payload.actor`) is not a discourse referent: reflexives, resolution and the published referents pass over it. |
| `HeadTy` | `NounPhrase.ty` and `Payload.ty` derive lists of known card-type facts; joined kinds retain evidence per half. |
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
  Nested characteristic text is stepped in `choiceContext`.
- `Binding`'s kind/payload pair and `unionPayload`'s returned dependent pair:
  the kind is now derived from the payload (`Referent.kind`), so there is no separate
  index to validate. Union payloads retain only card-type facts shared by both arms.
- Other erased implicit binders on functions, templates and definitions refer to these same
  datatype duties; erasing the binder does not remove the obligation at the
  resulting term's consumer. In particular, templates and definitions are not an independent
  certificate for arbitrary enacted bodies; the authoring boundary above establishes their trusted origin.


### Characteristic edits and keyword argument schemas

`StaticSpec.characteristicChange` and `CopyExcept.edits` carry the same typed
`CharacteristicEdit` list. A static change of stats ("gets +P/+T", "has base power and
toughness P/T", "its power is equal to …", "switch its power and toughness") and of abilities
("has flying", "has all activated abilities of …", backup's selection) is a
`characteristicChange` too. A Characteristic-Defining Ability is its own form,
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
removal removes each one, its mana cost included. A retained type ("It's still
a land") adds the type line whatever the op [CR#205.1b]; no edit stores the
retained type. `copyCharacteristics` expands the old
packed input into one edit block and separate ability exceptions. The former
minimum-two-fields packing rule has no semantic counterpart in an edit list.

`Ability.keyword` takes an ordered `List KeywordParam`. Generated
`KeywordFacts.argumentSchemas` enumerate admissible orders and each quality
argument's domain. The generator reads declaration columns and existing
variant metadata; the checker does not branch on keyword names. In particular,
qualified cost forms retain their object-quality constraint and declared
cost-only alternative. Quality arguments can read the incoming context;
subject arguments, numbers, costs, and deck conditions retain their closed
contexts. Argument lists do not publish referents between elements. The registry
codec still limits declaration combinations; changing that codec is outside
this fold.

### A keyword definition split by card class

A keyword that means one ability on a permanent and another on an instant or
sorcery spell [CR#702.55a,702.131a,702.131b,702.174b] writes one definition
holding both halves, each guarded by the card's class with the ordinary
conditional forms (a `doIf` with no otherwise-branch, a trigger's intervening
condition, or a conditional static ability over `.matches .this p`;
`isPermanentCard` and `isInstantOrSorcery` are the templates).
`semantics-v2.md` §7, ruling 2026-10-06 ("Gift and the additional-cost keywords
share one mechanism"); there is no selector device.

- Each half is checked as a term on every card, with no class context
  (`Ability.checkAll [] body`).
- Against the card's class, `classAbilityOk` checks the keyword itself
  (`keywordCardOk`: the row's `onPermanentCard` / `onInstantOrSorceryCard`).
  The parts of its definition are not checked against the row or the class:
  the definition-fit, stack-regime and class-guard checks (`keywordBodyFits`,
  `KeywordFacts.definition`, `StackRegime`, `ComponentFacts`, `ClassGuard`) are
  retired (`lean-v3-authoring-impl-7-checker-and-regime-retirement`),
  and the facts table no longer carries a definition, a regime, `functionsOnStack`
  or `counterEligible`.
- Pins: `Semantics/Proofs/KeywordByCardClass.lean` and
  `Semantics/Proofs/GiftVariants.lean`, which check gift's halves and spelled
  variants on both card classes.

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
general linked-ability reference.

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

User ruling (2026-09-06): Counter is one enactment of the Counter label around
stack-to-graveyard movement, uniformly for its spell or ability subject. This
supersedes the prior optional-destination design and its ability-specific branch.
The enacted movement no longer checks its stack source (`lean-checker-sweep-zone`). The rule defining
Counter supplies stack removal and a spell's graveyard destination [CR#701.6a];
this uniform expansion is the workbench's chosen representation, not an added
runtime lifetime model.

An ability referent stores its zone, initially the stack. Movement updates that
zone while retaining its ability identity and copy origin. A stack reference
resolves on actual stack evidence; counter and copy check no zone. Element referents and
unions retain location evidence rather than restoring a hardcoded stack location.

`Action.clearDamage` removes all marked damage from a permanent. Its
subject must be an object; where it is is not checked, and it need not
currently be a creature [CR#120.6]. It publishes the ordinary subject reference and no damage
outcome. Regeneration composes this primitive with tapping and conditional
removal from combat inside a destruction replacement.


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
`<declaration>.<parameter>` (`destroy.subject`, `fight.other`). Where `inCaller` and the
mention nest is not yet one rule: the order moves the caller-view role a body mention leaves
(`fight` reads its other creature back inside `inCaller` as damage's recipient, outside it as
the handoff's performer, and `Proofs/Damage.lean` pins "Tap it" reading the fight's source). A Template that names one
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
and per-member damage checks. It writes no battlefield guard: that demand is the
rules' [CR#701.14b].

Regeneration installation creates a next-time-this-turn destruction replacement.
The Regenerate label belongs to its application: clear marked damage, have the
controller tap the permanent, and remove it from combat if applicable. A static
regeneration replacement uses that application repeatedly; it does not install
a further shield [CR#701.19a..701.19c].

The `counter` Definition expands to one labeled stack-to-graveyard move in its
own scope, `ownScope (enact counter (move subject stack graveyard …))`.
It reads the subject once and needs no ability/spell branch.
The movement pre-state remains available to postposed conditions such as
Ertai's Trickery.
An alternative may also re-read the primary clause's stated amount, as in
Caustic Bronco's "otherwise ... that much"; this is a numeric reference, not
evidence that the primary branch actually happened.

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
an `Ability`, not a separate property type; whether it is one an effect could
grant (`grantable`) is retired with the grant rules. A damage result names a `CounterKind`;
its recipient is an object or a player (`counterHolderKind`), and which of the two the
counter registry declares is not checked.

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

A zone-change event is `GameEvent.does subj` over an
`Action.move subject from_ to at_ cause riders`, whose subject is the clause's
own (`NounPhrase.actor`) in the verb templates: it names the subject, origin,
optional destination, and observation point (`ObservationPoint`: `arrival` or
`departure`). Its checking and historical mention order is subject,
origin, destination. An endpoint can refer to a player introduced by an earlier
field. A missing endpoint is unconstrained; empty origin sets, exclusion of every
origin, and movement from a single shared zone back into itself are refused.
Player-qualified exclusions are conservative when only a zone sort is known.

The verbs `die`, `leave` and `enter`, which move the clause's own subject
(`NounPhrase.actor`), and `putInto` are templates. Battlefield and graveyard
departure patterns observe on departure; ordinary arrivals observe on arrival.
In particular, graveyard arrival from anywhere is not a battlefield-departure
pattern [CR#603.6c,603.10a]. The raw observation point retains the distinction.
Interception publishes departure location; a triggered continuation publishes
arrival location. Historical observation publishes mentions without moving current
references or manufacturing an event outcome. Source and destination possessors
are available in ordinary field order.

Damage event patterns share optional source and patient fields and require at
least one participant. Named active/passive authoring forms remain templates; no
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
published referent stay placeless until positive zone evidence constrains them.
Subtype-family edits require a hosting card type, not battlefield membership.

The prototype priority applies: the discarded placement-spelling prohibitions,
wrapper-specific `notExtended` refusal, and accidental battlefield evidence on
placeless card/source referents are not compatibility contracts. Their affected
witnesses are re-spelled or corrected in the landing record.
