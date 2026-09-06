# English grammar design

The standalone `english/` Lake project models the NLP grammar of Oracle
English under the [accepted decision](decisions/english-lean-design-workbench.md).
It has no dependency on or interaction with `lean/` (Semantics). The
[Oracle English glossary](contexts/oracle-english/CONTEXT.md) owns its terms.
The [review](english-grammar-review.md) records checked claims and limits.

## Reviewed architecture

One recursive `Syntax` carries phrases, clauses, textual groups, missing
positions and explicit relative forms. Syntax values are analyses; proofs of
judgments are evidence, not additional readings. Features are judgments over
that syntax, rather than a Cartesian product of category variants.

The complete candidate entry point is `Analysis.Reading`, whose evidence is
`Dependencies.Admitted`. It requires all three layers:

1. `Admissible`: closed category/frame derivation and an independent surface
   realization. `JudgesIn` carries a recoverability context and gap resources;
   `Judges` starts with empty context. This is the composition layer, not the
   final candidate contract.
2. `Features.Conforms`: declared noun/determiner countability, modifier order,
   and temporal NP distribution, checked recursively at their hosts.
3. `Dependencies.Safe`: extraction and sharing boundaries and the relative
   form's discharge conditions, checked recursively.

`Analysis.Selected` compares only those checked candidates. Preference edges
require a local declared marked-role competition or an Identity Claim and a
lexical claim over the same surface, and lift through unchanged supported
contexts. The numeric measure proves termination; it never creates an edge.
`Analysis.Related` closes explicit scope moves over checked candidates, with
every intermediate checked too. `Analysis.package` retains exactly the
selected members of that grammatical equivalence class. There is no fallback
that puts every equal-text tree in one class or arbitrarily removes ties.

The earlier `Selection`, `Scope`, `GrammaticalScope`, `RolePreference` and
`FrameScope` modules retain their named fragment laws. Their raw candidate
interfaces are proof components. In particular, the older host-structure
invariant describes its attachment fragment; frame-boundary variation is a
separate extension, not a claim that the old invariant holds for reanchoring.

## Composition decisions

| Concern | Reviewed representation |
|---|---|
| Lexical frames | Ordered grammatical relations, fixed markers, and marked Complements; repeated nonempty frame portions use `frameCoordination`. Voice/form and finite agreement are separately declared. |
| Nominal features | `NominalUse` propagates declared count/mass uses through modifiers, relatives and coordination. `DeterminerUse` constrains determinatives, cardinals and genitives. Attributive noun/type uses and Targeting Markers have distinct declared constructions. Targeting precedes descriptive modifiers; no total adjective-order hierarchy is guessed. |
| Agreement | Additive NP coordination joins person and makes plural agreement. Alternative coordination consults the nearer conjunct at the clause boundary. The implemented finite clause is subject-before-verb; the after-verb projection is proved separately. |
| Temporal phrases | A bare NP adjunct needs `Temporal` evidence. That license adds no argument to a passive Verb Frame. Existing retained-object passive frames remain independent lexical declarations. |
| Extraction | Ordered `GapUse` values retain category and grammatical relation. Adjunct and complex-subject boundaries constrain extraction; ordinary coordination cannot impersonate explicit shared-gap coordination. |
| Relatives | That, zero, fronted and supplementary forms are distinct on the same relative carrier. Zero cannot discharge a Subject; fronting needs a declared relative pronoun/possessive phrase of the gap's category. Pied-piping fronts a PP. Possessive relatives agree with the fronted phrase, independently of the modified noun. |
| Ellipsis | An omitted VP records form/voice, without recursively embedding an antecedent tree. A paragraph makes earlier overt VP projections available to subsequent items. Quotation resets the context; parentheticals inherit preceding context without exporting their internal introductions. |
| Shared dependents | Explicit shared gaps plus `rightNodeRaising` distinguish a following shared filler from preceding-context ellipsis. Checked scope moves cover shared nominal heads and plural determiners as well as modifiers and auxiliary attachment. |
| Arithmetic and comparison | Category composition supplies cardinal determinatives, Measure Phrases, arithmetic and comparative complements. Arithmetic truth and variable denotation are outside the grammar. |

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

## Textual and lexical boundaries

Document, paragraph, cost, keyword, modal and type groups are flat ordered
collections with explicit item categories and cardinalities. No generic
coordination rule licenses a Document. An initial adverbial belongs to an
ordinary clause, so a trigger-shaped sentence can head a multi-sentence body.
Ordinary and activated paragraphs share the same modal/body structures.
Empty documents and textual sections are admitted where appropriate.

`Surface` atoms own spaces, bound suffixes, quotes, symbols and line breaks.
`Realizes` relates grammar to annotated surfaces; `Spells` relates a surface to
an exact string. Sentence completion handles nested quotes and the closing
comma of a sentence-final supplement. Keyword collections admit comma or
semicolon separators, with one canonical singleton shape. Parentheticals can
occupy paragraph positions. Face tags group independent text boxes and make
no assertion about card legality or game meaning.

Lexical forms are supplied relations. They must preserve case, identity,
Onset where required, and boundary ownership. `BoundaryInteractions.wordPayload`
checks the proposed word-atom whitespace/quote boundary contract with positive
and negative examples; it is not yet a validator over an entire supplied
Lexicon. Structured ranges, chapter lists, symbols and lexical allomorphs may
be supplied as declared notation. English does not validate their game values.
Production migration must retain source bytes and distinguish any explicitly
recorded normalization; this model supplies no normalization or Rust roundtrip
proof. Italics and physical card-layout metadata are outside the NLP surface.

## What the proofs establish

The finite challenges named in the review cover inhabited combinations,
exclusions, retained distinctions, finite selection, enumeration independence
and exact packing. The final grammar admits checked frame-pair alternatives,
shared-head alternatives, count/mass phrases and relative dependencies.
The original nested-mobile challenge has four distinct admitted readings;
quotation preserves all four. Equal lexical surfaces do not equate lexical
identities. Frame-boundary moves preserve lexical leaves and anchor count,
including the flat-versus-nested cardinality fence.

These are laws about stated fragments and declarations. They do not establish
Oracle completeness, global absence of unintended ambiguity, completeness of
candidate enumeration, or correctness of a future Rust implementation.

## Migration obligations and bounded limits

The architectural choices above are resolved. The formalization is a checked
prototype of those choices, not an exhaustive set of productions. The
[migration-design ticket](tickets/critical/english-v2-grammar-migration-design.md)
now owns the correspondence and sequencing. Before implementing a family, it
must pin the following elaborations and preserve its inherited witnesses:

- Generalize local marked-role competition beyond the checked Object plus one
  marked Complement, using the declared sequence, first eligible occurrence,
  and supported context lifting. `projectHost` distinguishes fixed markers
  from opaque marked-role edges. Do not infer a comparison from counts.
- Extend the explicit scope-move inventory where additional shared-frame or
  shared-preposition forms need it. Preserve complete alternative readings,
  coordinator identity and group topology. Anchor count is one necessary
  invariant, not a complete classification algorithm.
- Extend the same feature/dependency judgments for qualification/distributive
  phrase distributions, free/wh content clauses, nominal ellipsis, omitted
  destinations outside the checked relative pattern, and intra-sentence
  recoverability. These are grammatical elaborations, not lexical population
  and not deferred silently to the card tail loop. Existing fog obligations
  remain assigned in the wayfinder.
- Populate and challenge lexical distributions, inflections, genitive endings,
  richer quantity/comparison forms, literal-word quotations, inline fragment
  reminders, notation and prescribed textual templates. The supplied-form
  contract must be enforced at the production lexical/source boundary.
- Establish Rust correspondence by structural tests. Keep game-reference
  resolution and card validation outside English. No executable Lean parser,
  renderer, generator or Semantics consumer is required.

The review is complete for its named design decisions and finite challenges.
Migration planning may proceed; a production slice is ready only when its
applicable elaborations and correspondence checks have been discharged.
