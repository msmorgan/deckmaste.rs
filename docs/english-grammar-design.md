# English grammar design

This is the evolving design of `english/English`, under the
[accepted decision](decisions/english-lean-design-workbench.md). Scope is
Vintage-playable Magic as specified in `CLAUDE.md`. The
[Oracle English glossary](contexts/oracle-english/CONTEXT.md) owns linguistic
terms; [CONTEXT-MAP.md](../CONTEXT-MAP.md) separates them from Game Model.
The [style guide](oracle-style-guide.md) supplies structural direction, with
supported Oracle text supplying conformance evidence. Existing Rust
constructions are migration evidence, not the category inventory.

## Representation and dependencies

Use ordinary recursive syntax values and inductive grammatical judgments in
`Prop`. Analyses are syntax values, not proof objects: two proofs of one
judgment do not represent two readings. Features constrain derivations rather
than making every feature combination a different Lean type. This makes
ill-formed candidates and exclusion claims expressible, and makes changes to
agreement or gap constraints local to judgments. The tradeoff is that every
admissibility claim must carry a grammatical derivation; constructing a syntax
value alone proves nothing about its grammaticality.

The implemented dependency order is `English.Grammar` (shared syntax,
features, lexical assumptions, grammatical and realization judgments), then
`English.Witnesses`, `English.Composition`, `English.Documents`, and the
selection/scope modules, imported by `English.lean`.
One `Syntax` recursively contains phrases and clauses. The mutual
`Judges` / `JudgeChildren` / `JudgeFrame` group checks their composition.
Documents and selection consume this model; neither licensing nor realization
depends on selection or `Semantics`.

`Production` gives ordered categories for local constructions. A Verb Frame is
an ordered list of `FrameItem.argument` values (grammatical relation plus
category) and `FrameItem.fixed` lexical markers. `JudgeFrame` instantiates the
schema against actual children; no construction or spelling guard decides a
head's frame. The same NP category supplies Subject, Object and a preposition's
Complement. Determiner is the function assigned by `determine`; its child
category is Determinative Phrase.

`Judges lexicon tree category gaps` records ordered unfilled positions.
`Derives` is its closed, empty-gap case. Ordinary composition concatenates
resources, a relative construction discharges one matching NP gap, and explicit
shared coordination requires the same gap in both conjuncts. An ellipsis node
retains its supplied antecedent tree, whose structure must derive; identifying
that antecedent in discourse is an assumption, not a theorem of this judgment.

`FiniteLicense` propagates the overt head's declared agreement through
auxiliaries, adjuncts, coordination and ellipsis. Regular inflection is the
`FiniteForm` helper; it does not constrain invariant or irregular heads.
Voice is independent of Inflectional Form: auxiliary declarations select both
form and voice, so a passive frame cannot enter a perfect construction merely
because both use a past participle. Negation is polarity on an auxiliary
construction with a licensed complement. There is no generic production that
puts *not* before any finite verb.

`Realizes` and `Linearizes` connect these trees and their children to surfaces.
`Admissible` requires a closed derivation and realization, independently of
selection. Lexical licensing and word forms are distinct relations: neither
lexical completeness nor disjoint spellings are assumed.

`English.Surface` now defines annotated atoms: words, opening and closing
boundaries, symbols, and line breaks. Ordinary string literals coerce to word
atoms, preserving the earlier witnesses. `Spells` relates an annotated surface
to an exact Lean string. Structural binding governs spaces: closing punctuation
and bound suffixes attach left; opening punctuation attaches right; adjacent
symbols bind; line breaks remain explicit. `Written` combines grammatical
admissibility, realization and exact spelling independently of selection.
No input normalization or Rust byte-roundtrip theorem is assumed.

Document productions own sentence/keyword-line capitalization and punctuation.
Sentence completion recognizes an already terminated quoted sentence, and
quotation changes inner quote delimiters structurally. These small spelling
helpers answer the boundary-ownership questions; they do not constitute an
executable parser or complete renderer. Lexical forms retain internal case,
apostrophes and declared symbols. Faces retain independent text boxes.

## Whole intended grammar and source map

All sixteen style-guide sections are assigned below. A row describes intended
scope, not implemented coverage. The introductory checklist and final review
are review criteria over these structures rather than additional syntax.

| Guide section | Grammatical responsibility | Next formal owner |
|---|---|---|
| 1. Instructions | Clause voice, finiteness and tense; sentence scope; paragraphs and ability boundaries | Composition for clauses; document for sentences and paragraphs |
| 2. Capitalization | Lexical case, proper names/type names, sentence and keyword-line initial case | Document realization |
| 3. Punctuation and glyphs | Clause/coordination punctuation; cost colon; labels and dashes; parentheses, quotes and apostrophes; hyphens, signs and slashes; bullets, bars, brackets, mana/other symbols, infinity | Document, using composition's constituent boundaries |
| 4. Numbers, quantities, comparisons | Count determiners versus scalar notation; variables; number/amount/much/many; arithmetic, comparison and rounding phrases | Composition; notation realization in document |
| 5. Names and anaphora | Names and self-reference, pronouns, demonstratives, chosen/former/latter references, this-way/event anaphora | Composition and document capitalization; game-reference resolution is outside this project |
| 6. Object/player/target descriptions | Nominal and NP, determinatives versus Determiner function, quantification, ordered modifiers, Targeting Marker, owner/controller and zone/event postmodifiers | Composition |
| 7. Type grammar | Noun and attributive uses, subtype/supertype features, non- polarity, plural morphology/agreement, choosing names/types, retained characteristics | Composition; type-line and case in document |
| 8. Ability architecture | Spell, activated, triggered and static text; cost/effect/instruction boundaries; loyalty, ability words, intervening conditions, delayed/reflexive triggers, shared keyword lines | Document, with clauses/conditions supplied by composition |
| 9. Costs and casting | Additional/alternative costs, increases/reductions, permissions and restrictions, zone dependencies | Composition for constituent grammar; document for cost lists and notation |
| 10. Logic and choice | Modal auxiliaries, negation, sequence, conditions, ordinary and modal choices, and/or/and-or, distribution and shared dependents | Composition; modal lists/pawprints in document; scope alternatives in selection |
| 11. Timing and duration | Turn-name NPs, bare temporal adjuncts, endpoint PPs/clauses, this/next/each, duration and bounded intervals | Composition |
| 12. Effect families | Frames for card/zone verbs; token/copy/counter and characteristic descriptions; combat, damage, life, mana; casting/control/attachment and other actions; random choices, dice, coins and voting | Composition via lexical frames and shared phrase/clause grammar |
| 13. Continuous/replacement/prevention templates | Conditional would/instead, enters and skip templates; prevention, characteristic predication, restrictions and exceptions | Composition; multi-sentence grouping in document |
| 14. Keywords, labels, reminder | Keyword abilities/actions, parameter and quality selection, bound surfaces, ability/flavor words, parenthetical reminder prose, named/batch forms | Document; shared lexical frames and host categories in composition |
| 15. Frame-dependent text | Sagas, Classes, levelers, Cases, Rooms, die tables, Stations; face/half boundaries including split, Adventure, aftermath, meld and prepare text; siege reminder text | Document; referential expressions in composition |
| 16. Editorial review | Architecture and surface passes over the above, including interactions and source conformance | Design review; no new grammatical category |

### Categories, features, and relations to develop

Lexical categories include nouns, verbs, adjectives, adverbs, prepositions,
determinatives, pronouns, and coordinators. Distinct uses of a lexeme are
licensed by declared features; shared spelling is not shared distribution.
Target Noun, Target Verb and Targeting Marker must remain distinct. Status and
Designation are Game Model concepts, not grammar categories.

Phrase structure includes Nominal, NP, Adjective Phrase, Adverb Phrase, PP,
Lexical Verb Phrase, and auxiliary/finite/nonfinite Verb Phrase structure.
Subject, Object, Predicate, Determiner, Complement, and Adjunct are functions
or relations borne by constituents, not parallel copies of their categories.
A Verb Frame declares ordered complement requirements and fixed markers; its
instantiation supplies the Lexical Verb Phrase. Predicative complements may
be NP, Adjective Phrase or PP. Preposition selection and attachment must be
feature-driven. Costs use these grammatical categories in document positions;
semantic cost families do not each imply a new grammatical category.

Features to separate include Person and Number, derived Concord Class,
Finiteness and verbal Inflectional Form, voice and polarity, lexical
countability and complement licensing, modifier distribution/order, and
pronunciation Onset at realization. Not all of these belong on every node.
The composition ticket must choose feature carriers and propagation rules,
including coordinated agreement, instead of creating their Cartesian product.

Clause structure must cover imperative and declarative instructions,
interrogative/complement content, finite and nonfinite subordination,
relative clauses, auxiliaries, negation, copular predication, passives,
conditions, and adjuncts. Gap dependencies must support relative which/whose,
pied-piping, nonrestrictive relatives, finite that/wh complements, gerunds,
ellipsis and gapped destinations. Recipient passives with retained objects
must be distinguishable from temporal adjuncts. Anaphoric syntax records
reference shape; resolution to game referents is outside grammatical proof.

Coordination applies at licensed constituent categories, with ordered
conjuncts, explicit coordinator and shared dependents/gaps. It must account for
correlatives, gapping and right-node raising as well as ordinary lists.
Attachment and modifier scope remain visible in trees. Linearization may
identify surfaces of distinct trees; selection must not erase the difference
merely because one wrapper or traversal order wins.

Documents supply sentence/paragraph and ability group structure, cost lists,
keyword lines, modal lists and frame layouts. Notation has its own composition
and realization rules while embedding ordinary grammatical constituents.
Reminder prose and quoted/granted abilities require explicit embedding rules;
they are not opaque strings introduced to make unsupported cases pass.

## Checked initial fragment

`English/Witnesses.lean` defines an illustrative three-lexeme inventory and the
synthetic nominal surface `white creatures and artifacts`. Both analyses are
admissible: modifying just the first conjunct, or modifying the coordination.
The trees are proved unequal. These are scope candidates in the fragment,
not a claim about a named card, semantic equivalence, or a preferred Oracle
reading. The original coordination now uses the general production through an
abbreviation; it is not a second admissible wrapper.

The same file proves that an unlicensed noun use of the adjective lexeme has
no nominal derivation. Together the positive and negative witnesses show that
the licensing relation is inhabited and restrictive. Selection, packing and
precedence are deliberately absent from these definitions.

## Composition decisions and evidence

`English.Composition` adds explicit structural and surface witnesses for:

- A relative body whose subject gap survives an auxiliary and PP adjunct;
  the head discharges it, yielding the synthetic phrase *creatures that can
  attack during turns*.
- A declared Object frame filled by coordinated NPs, with the surface
  *destroy creatures and artifacts*; the same `creatures` tree also occupies
  Subject and preposition-Complement relations.
- Cardinal determinatives, arithmetic Measure Phrases, comparative Adjective
  Phrases, nonfinite clauses, negative auxiliaries, and passive selection.
- Agreement and frame-arity exclusions with positive twins, fixed-marker
  identity, a gap that cannot derive as closed, and sharing one gap across
  coordinated clauses.

The source directions are style guide §4, “Numbers, quantities, and
comparisons”; §§5–7, names and nominal/type grammar; §10, “Logic, choice, and
coordination”; §11, “Timing and duration”; and §§12–13, effect templates.
The witnesses above are synthetic combinations, not corpus transcriptions.

The following scope decisions bound this formal composition model. They are
requirements for the design review's challenges, not instructions to restore
card-driven scheduling:

| Capability | Decision and exact limit |
|---|---|
| Lexical distributions and modifier order | Model licenses nominal adjectives, declared noun numbers and bare plural NPs; countability, attributive noun/type features, Targeting Marker ordering and genitives need a feature-carrier decision before broader claims. No Game Model Status category is introduced. |
| Frames and grammatical relations | Ordered arguments and fixed lexical markers are modeled; copular NP/Adjective Phrase/PP complements and retained-object passives use declared frame data. Automatic active-to-passive frame conversion is outside this model's claims. |
| Agreement | Finite heads declare form/agreement relations, including invariant modals. Binary same-category coordination resolves `and` NPs to plural; mixed-person/mixed-number and `or` proximity agreement remain a review decision, not silently licensed by a guessed rule. |
| Subordination and gaps | Finite/nonfinite clause embedding, one NP relative gap and explicit across-conjunct sharing are modeled. Extraction islands, whose/pied-piping, zero versus overt/nonrestrictive relatives and right-node raising need distinct discharge/placement decisions before their acceptance is asserted. |
| Ellipsis and anaphora | VP ellipsis carries an explicit grammatical antecedent; this models recoverability as an input. Discourse accessibility, omitted destinations, nominal ellipsis and door/half/former/latter reference resolution are not proved. Review must model the grammatical distribution of these forms; game-reference resolution is outside English. |
| Adjuncts | Host/dependent licensing and before/after placement are explicit. Marked subordinate clauses are distinguished from bare clauses. Bare temporal NPs still need a distribution feature; the eleven passive-temporal obligations remain migration/review challenges. |
| Quantities and comparisons | Cardinal-to-determinative projection, Measure Phrase arithmetic and selected comparative complements are modeled. Variable binding, fractions/rounding spellings and full countability constraints are documentary/lexical extensions; arithmetic truth is not grammatical acceptance. |

**Less certain:** the single-gap resource representation is useful evidence
for composition, but does not settle extraction constraints or antecedent
accessibility. The review must challenge these before declaring a migration
slice ready. No theorem here establishes global grammatical soundness.

## Document decisions and evidence

Document constructions use the same recursive `Syntax`, `Judges` and
`Realizes` as clauses. A quoted Document can fill an ordinary lexical frame,
so nesting does not require a separate quoted grammar. `English.Documents`
contains typed witnesses carrying their concrete trees, derivations and
surfaces, with independent exact-text assertions. `DocumentShape` contains
only the structural vocabulary; the shared Grammar owns recursive licensing.

| Source-map portion | Representation and evidence | Remaining review question |
|---|---|---|
| §§1–3, sentences and editorial boundaries | Sentence/body/ability/document composition; paragraph breaks; punctuation atoms and exact `Spells`; empty sentences have no surface derivation | Which source-level whitespace variants must be retained rather than canonicalized? No normalization is modeled here. |
| §2, case | Sentence and keyword-line initial case; type-line lexical case remains independent of common-noun word forms | Pronunciation/onset and the full lexical case inventory remain lexical obligations. |
| §3, quotation | A Document fills a quoted-text complement of an ordinary verb frame; nested quotes alternate straight double/single marks; terminal period is not duplicated | Literal-word quotations and punctuation other than terminal periods need their own boundary rules. |
| §3, symbols and notation | Symbol atoms bind adjacent symbols; signs, slashes and numeric/range headers retain exact lexical notation surfaces | Decide whether ranges, scalar pairs and chapter lists become structured notation terms or lexically supplied notation. Current range witnesses assert exact strings, not a general numeral conversion theorem. |
| §8, ability architecture | Ordinary bodies, typed cost lists and colon boundaries, triggered subordinate clause plus instruction; subsequent sentences group in bodies | Grammar does not infer semantic spell/static/triggered classifications from typography. Intervening-if and delayed/reflexive body attachment still need interaction challenges. |
| §14, keywords | Bare, parameterized and bound-suffix declarations; keyword lists; the same parameterized/bound tree in a line and a verb frame | Keyword-specific separators and allowed quality categories remain declarations. Semi-colon lists and inline reminder fragments need review extensions. |
| §14, labels/reminders | Ability-word versus flavor-word label kinds are retained; labels use spaced dashes; reminders contain grammatical bodies; nested reminders are rejected structurally | Full prescribed reminder templates and inline NP/condition reminders are not populated. Print italics are label metadata, not characters in source spelling. |
| §10, modes | Introducer clause, separate mode bodies, bullet/newline boundaries and notation-weighted modes | The exact pawprint-symbol inventory and mode cardinality restrictions are declaration/layout questions; the synthetic weighted witness does not invent a card or symbol meaning. |
| §15, Saga/Class/leveler | Chapter dash rows, cost-and-Level headers, level range/stat/body bands, with ordinary grammar inside each section | Populate validated numeral/range forms and challenge multi-chapter headers; grouping text by section is modeled, chapter/level game meaning is not. |
| §15, Cases | Separate To solve and Solved sections, including an activated ability in the latter | The optional prescribed solve reminder uses the general reminder mechanism; exact template population remains lexical/document work. |
| §15, dice/Stations | Distinct vertical-bar and em-dash die rows, en-dash range witness, and bound-plus Station thresholds | Check row coverage/order constraints and multiline sections against representative layouts before migration. |
| §15, Rooms and faces/halves | `Faces` holds independent typed Documents for Room doors, transforming/modal/split/Adventure/aftermath/meld/prepare portions; no combined name is inserted into a sentence | Separate supplied text portions and their visible boundaries are inputs. Face counts, card legality and frame metadata validation are outside the NLP grammar. |
| §15, standalone frame reminders | A reminder can form an independent document paragraph, using the same grammar and nesting check | Exact source-specific templates, including siege reminders, need population; no semantic effect is inferred from parenthesization. |
| §7, type lines | Supertypes precede a nonempty type group, then optional dash/subtypes; wrong group order is excluded | Lexical catalog completeness and capitalization conformance remain external evidence. |

All example instructions and layouts are synthetic. The style-guide section
references identify design evidence, not quotations of particular Oracle
cards. These witnesses establish compositional shape and exact spelling under
their lexical assumptions, not complete coverage of each template family.

## Inherited obligations and remaining decisions

The [wayfinder](english-grammar-wayfinder.md) retains ticket identities;
[fog](tickets/fog.md) retains the detailed counterexamples. They challenge the
source-derived design; they do not define its inventory.

| Obligation group | Decision or evidence still owed |
|---|---|
| Grammatical relations and person/number agreement | Composition: feature carriers, relation judgments, agreement and negative witnesses |
| Lexeme-owned frames, subordinate/relative clauses, Target Verb | Composition: one recursive phrase/clause model, frame satisfaction, extraction and gap discharge; preserve the named relative-clause obligations in the wayfinder |
| Adjectives and keyword subject modifiers | Composition: lexical distributions, nominal hosts, adjective/participle predication and modifier order; document: keyword realization |
| Attachment classes, locative licensing, of-complements, remaining prepositions | Composition: selected complements versus adjuncts and declared licensing; no spelling guards |
| Locative coordination, cross-host scope gates, complement-pair nesting | Composition and selection: multi-mobile/shared adjuncts, conjunct boundaries, wrapper duplicates and retained alternatives |
| Cost families and type-line declarations/constructions | Document: grammatical cost composition, punctuation and type-line realization; migration: retire semantic category duplication |
| Gerunds, gapped destinations, ellipsis, recipient passives | Composition: explicit missing-position dependencies and retained complements; preserve the three named destination obligations in the wayfinder |
| Copular complements, type modifiers and polarity | Composition: NP/Adjective Phrase/PP licensing rather than status-specific forms |
| Arithmetic, fractions, comparisons, degree Adverb Phrases | Composition: value/measure syntax and attachment; document: surface notation |
| Eleven passive-temporal misselections | Composition: complement/adjunct distinction; selection: reject unsupported object readings while retaining legitimate scope alternatives |
| Door/half references, former/latter and event references | Composition: syntactic referential forms; document: textual boundaries; no game-reference resolution |
| Shared gaps, relatives, complement clauses and coordination | Composition: extraction constraints, pied-piping, nonrestrictive attachment, correlative coordination, gapping and right-node raising |
| Reminder/editorial text and unowned/unbucketed historical tails | Document and design review: representative challenges and explicitly routed residuals |

**Less certain:** ordinary syntax plus judgments is the initial representation,
not a claim that indices will never help. Gap accounting and mutual recursion
are the first stress test. The composition ticket must record any revision
with a cross-capability witness, rather than preserve the initial encoding at
the cost of a second phrase grammar.

**Less certain:** exact spelling is now connected through annotated atoms,
but the full numeral/notation inventory and tolerated source whitespace are
still review questions. Selection can state textual claims through `Written`
without assuming this is a complete transcription model.

The selection model below distinguishes acyclicity, surviving analyses and
canonical packages with retained readings. Its general laws are conditional
on the supplied feature and scope classifications; no global uniqueness or
classification-completeness theorem is claimed.

The design review owes interaction witnesses and resolution of blocking open
decisions. Migration design then maps this model to Rust, decides which
machinery survives, and rewrites implementation slices with preserved
regression obligations. Neither a scope table nor these Lean proofs establishes
corpus completeness or correctness of the production implementation.

## Selection and packing decisions and evidence

The following finite inventory was fixed before implementation and is now
checked in `English.Selection`, `English.SelectionWitnesses` and `English.Scope`:

| Named obligation | Assumptions and claim |
|---|---|
| `selected_sound`, `selected_enumeration`, `selected_permutation` | An independently supplied admissibility judgment; selection ignores list order and multiplicity and retains only admitted members. Instantiation uses `Admissible` on the shared grammar. |
| `preference_decreases`, `preference_asymmetric`, `preference_transitive`, `no_preference_cycle` | One coherent feature assignment per analysis and one comparison region. Frame-role claims precede identity claims; identity breaks equal frame-role claims. No lexical or construction names participate. |
| `selected_exists` | A finite list containing an admitted analysis has a survivor under those preference rules. This does not assert uniqueness. |
| `packing_exact`, `packing_enumeration`, `packing_unique`, `different_classes_separate` | A supplied scope-class key; each occupied class contains exactly its complete surviving trees. Keys require separate grammatical justification. Equality is extensional and disregards enumeration/proof identity. |
| `modifier_scope_preserves`, `modifier_scope_unique` | Binary plural nominal coordination, licensed Adjective Phrase and closed heads; whole-coordination and first-conjunct modifier scope have the same surface and one exact package. The package retains two unequal trees. |
| `acyclic_tie`, `same_surface_separate` | Concrete inhabited grammar counterexamples: acyclicity does not select one tree, and equal text alone does not justify packing. |
| `cross_host_scope`, `nonfinal_boundary`, `anchor_shape_separate`, `joint_alternatives_exact` | Structural challenges for a mobile spanning hosts, right-peripheral attachment, nested versus flat anchors, and correlated mobile choices. Any abstraction from production frame-pair syntax is stated explicitly. |

Feature assignment is an adapter obligation, not a theorem that the current
lexicon already classifies every candidate. Qualification and distributive
measure licensing belong in grammatical admissibility before preferences;
this ticket does not invent verb-adjunct sites for those categories. The
first-eligible-site restriction is represented independently of the preference
ordering. Packing stores whole trees, not independently recombined mobile-site
sets. No scope key is inferred merely from equal text or construction names.

`Selected` retains admitted maximal analyses in a finite candidate list. The
four-valued `Claims.rank` is a compact encoding of the two ordered principles:
frame-role claims precede identity claims; identity resolves equal frame-role
claims. It is not a tunable score. Comparisons require the same region.
`frame_role_preempts` and `identity_preempts` prove those priorities;
`ordered_principles_winner` checks all four feature combinations, and
`inadmissible_cannot_suppress` checks that an inadmissible higher-ranked input
cannot defeat an admitted one. The assignment of claims and comparison
regions must reflect declared grammatical evidence. These laws do not prove
that an arbitrary assignment is linguistically justified.

`Packs` is a specification for a nonempty package: its readings must be
exactly the survivors with its scope key. `pack` supplies such a relational
normal form. `packing_unique` proves extensional uniqueness at a fixed key;
`unpack_exact` proves both preservation and absence of invented analyses across
all occupied classes. `packed_admissible` connects packages of selected shared
`Syntax` trees back to their independent grammatical derivations and surfaces.
No executable compactor, preferred tree, or canonical ordering of readings is
required. A production representation must separately satisfy this contract.

The generic modifier theorem covers any licensed Adjective Phrase and two
closed plural Nominals with supplied realizations, not just the initial three
lexemes. Whole-coordination and first-conjunct scope realize the same surface.
`modifier_package_retains_both` exhibits two unequal admitted trees in one
package; `acyclic_tie` certifies that neither is eliminated by the neutral
policy. `homographs_admitted` and `same_surface_separate` give the opposite
boundary: declared lexical identity can differ at identical text, and a
scope key preserving that identity keeps the readings in different packages.
Equal text alone is therefore insufficient grounds for packing.

The inherited challenges have the following precise disposition:

| Challenge | Checked result | Limit / next obligation |
|---|---|---|
| Cross-host scope | `CrossHost.cross_host_admitted` and `cross_host_scope` retain attachment outside an auxiliary, inside it over coordinated predicates, and on the final predicate. Each tree realizes the synthetic *can attack and attack during turns*, with the auxiliary expressed once. | This is a shared-Syntax witness for the structural Class C challenge, not a transcription or production test of the inherited named card. |
| Conjunct boundary and frame opacity | `nonfinal_boundary` excludes entry into the first conjunct for every Coordinator; `frame_transparent` permits descent through a frame host, while `role_edge_opaque` stops at the declared-role edge. | `Host` is an explicit boundary abstraction, not a grammatical derivation or an implemented projection from `Syntax`. Review must supply/check that projection for actual frame-pair syntax. |
| First eligible frame site | `first_eligible_unique`, `first_eligible_member`, `first_eligible_exists` and `first_eligible_skips` characterize the first eligible site in root-to-deeper right-periphery order. | Eligibility is a supplied grammatical predicate. A true frame-role claim must be backed by this judgment; deriving that evidence from every lexical frame is still an adapter obligation. |
| Flat versus nested anchors | `anchor_shape_separate` prevents packing different ordered group arities/topologies, irrespective of site assignments. | `Anchors` abstracts the inherited frame-complement-pair challenge. It does not claim that the current grammar has modeled all such pairs or their production anchor projection. |
| A mobile containing another mobile's scope region | `joint_alternatives_exact` retains a correlated pair of assignments and excludes both hybrids, even though separate projections would admit a hybrid. The generic `unpack_exact` law applies to complete `Syntax` trees as well. | The two-Boolean witness is a counterexample to independent recombination, not a full grammatical witness for Class D. Review must challenge joint assignments with an actual nested-mobile tree. |
| Qualification/distributive licensing | `qualification_sites_excluded` and `measure_postmodifier_excluded` show that the modeled dependent categories cannot acquire the disallowed adjunct sites. | These are exclusions in the current category distribution. Full degree, scalar and distributive-measure constructions and the passive-temporal inventory are still composition/review obligations. |
| Duplicate wrappers | Packages retain whole analyses; a scope key must preserve grammatical structure outside the justified alternation. | This ticket adds no duplicate grammar wrapper. A proof that all production wrapper duplicates have been consolidated belongs to migration; packing must not hide them. |

The additional supporting public laws are `neutral_selected`,
`preference_asymmetric`, `preference_transitive`, `packing_exists`,
`unpack_exact`, and `packed_admissible`. Together with the named laws above,
there are 39 public theorems and four private proof helpers. LSP axiom checks
for all 39 public theorems use only `propext`, `Quot.sound`, and
`Classical.choice` (each theorem uses a subset); the helpers are covered
transitively by those declarations. The source scanner's `opaque` matches in
`Scope.lean` are prose about role boundaries, not Lean opaque declarations.
All earlier 74 theorem assertions are preserved unchanged, and the complete
Lean target build passes with warnings treated as errors.

The two rejected generalizations are resolved, rather than weakened silently:
the modeled preference has a survivor on finite inhabited input, without
asserting a unique reading; packing retains the exact joint relation, not the
Cartesian product
of projected choices. Scope-package uniqueness is proved for the stated
modifier class, while the counterexamples remain checked declarations.

**Less certain:** the right-periphery abstraction and the completeness of the
scope keys are still the weakest correspondence boundary. The design review
must connect these abstractions to actual frame-pair/nested-mobile grammar and
challenge document embeddings before migration decisions. Neither package
uniqueness nor the feature-order proofs justify assigning all equal-text trees
to one class, nor do they establish Oracle adequacy or Rust correctness.


## Whole-model review status

The [2026-09-05 review](english-grammar-review.md) finds the formal foundation
useful but does not yet approve defining the production migration. It adds a
checked four-reading nested-mobile class and its quotation embedding, and
repairs terminal punctuation at deeper quote nesting. It also identifies
concrete document-closure and canonical-representation gaps, in addition to
the feature/context and scope-classification assumptions above. The review's
residual recommendations await discussion; they are not accepted rulings.
The review ticket remains open until those design blockers are resolved.


## Independent project boundary

English is a standalone NLP grammar for parsing and bracketing Oracle English,
as clarified by the user on 2026-09-05. The project root is `english/`, with its
own Lake configuration and warning-free `scripts/build` gate. It has no
interaction or dependency with the separate `lean/` Semantics project.
Grammatical dependencies and textual boundaries are in scope; game-reference
resolution, card validation and frame legality are not. Surface relations are
used to compare grammatical analyses, not to create a card-validation service.

## Review consolidation implemented

The [review's implemented-closure section](english-grammar-review.md#implemented-closure-textual-collections-and-grammatical-scope)
supersedes the earlier binary document-group and arbitrary scope-key sketches.
Textual collections are flat and consume distinct item categories. Initial
adverbials compose into ordinary clauses and multi-sentence paragraphs; modal
groups share ordinary and colon-prefixed paragraph hosts. Document categories
are excluded from linguistic coordination. Empty text portions are admitted
without making game-layout claims.

Scope classes are now quotients of licensed contextual moves among admitted
same-surface Syntax trees. They preserve lexical identities and host topology,
retain all four checked nested-mobile readings, and lift through quotation.
This closes the class-key design for the implemented scope fragment; extending
the move inventory and implementing declaration-backed preference and
partial-frame coordination remain review obligations. Ordinary feature and
dependency closure also remains open.
