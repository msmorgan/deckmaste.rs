# English grammar design

This is the evolving design of `lean/English`, under the
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
`English.Witnesses` and `English.Composition`, imported by `English.lean`.
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

`Surface` currently means a list of surface atoms represented by strings.
The witness uses words as atoms. It is not a byte string, scanner contract, or
normalized spelling; no byte-exact roundtrip claim follows. Linearization
orders atoms. The document ticket must settle the relationship between atoms,
word boundaries, bound forms, case, whitespace, punctuation, and exact text
before making any claim about byte surfaces.

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
| 5. Names and anaphora | Names and self-reference, pronouns, demonstratives, chosen/former/latter references, this-way/event anaphora | Composition; capitalization in document; denotation belongs to Semantics |
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
| Ellipsis and anaphora | VP ellipsis carries an explicit grammatical antecedent; this models recoverability as an input. Discourse accessibility, omitted destinations, nominal ellipsis and door/half/former/latter reference resolution are not proved. Review must choose which are grammatical context judgments versus semantic resolution. |
| Adjuncts | Host/dependent licensing and before/after placement are explicit. Marked subordinate clauses are distinguished from bare clauses. Bare temporal NPs still need a distribution feature; the eleven passive-temporal obligations remain migration/review challenges. |
| Quantities and comparisons | Cardinal-to-determinative projection, Measure Phrase arithmetic and selected comparative complements are modeled. Variable binding, fractions/rounding spellings and full countability constraints are documentary/lexical extensions; arithmetic truth is not grammatical acceptance. |

**Less certain:** the single-gap resource representation is useful evidence
for composition, but does not settle extraction constraints or antecedent
accessibility. The review must challenge these before declaring a migration
slice ready. No theorem here establishes global grammatical soundness.

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
| Door/half references, former/latter and event references | Composition: syntactic referential forms; document: embedding/layout; Semantics owns their meaning |
| Shared gaps, relatives, complement clauses and coordination | Composition: extraction constraints, pied-piping, nonrestrictive attachment, correlative coordination, gapping and right-node raising |
| Reminder/editorial text and unowned/unbucketed historical tails | Document and design review: representative challenges and explicitly routed residuals |

**Less certain:** ordinary syntax plus judgments is the initial representation,
not a claim that indices will never help. Gap accounting and mutual recursion
are the first stress test. The composition ticket must record any revision
with a cross-capability witness, rather than preserve the initial encoding at
the cost of a second phrase grammar.

**Less certain:** atom-level realization will be sufficient for structural
scope proofs but its best extension to exact text is unresolved. The document
ticket must choose bound-word composition and surface ownership before the
selection ticket states textual rather than atom-level claims.

Selection owes an independent model of candidate sets, admissibility,
preference, ties, and canonical packing with retained readings. Its named
proofs must distinguish acyclicity from unique choice and unique choice from
unique interpretation. It must explicitly state the fragment and lexical
assumptions of each theorem. No global uniqueness theorem is promised.

The design review owes interaction witnesses and resolution of blocking open
decisions. Migration design then maps this model to Rust, decides which
machinery survives, and rewrites implementation slices with preserved
regression obligations. Neither a scope table nor these Lean proofs establishes
corpus completeness or correctness of the production implementation.
