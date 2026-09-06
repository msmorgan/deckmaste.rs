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
production correspondence below pins the implementation sequence and its
[obligation register](english-grammar-migration-obligations.md). Before implementing a family, it
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


## Production correspondence

Migration design, 2026-09-05. This is the production replacement plan authorized
by the accepted Lean decision. It implements the reviewed architecture inside
`english_v2`; it does not translate Lean proofs or require a Lean executable.
The ticket graph in the [wayfinder](english-grammar-wayfinder.md) schedules the
work. Existing passing behavior supplies regression evidence, not an extra
compatibility contract for superseded construction types.

### Machinery decisions

Inspected at migration claim `luwqpxyt`. Source names below are relative to
the named crate under `crates/`; these are reuse decisions based on source
inspection, not performance results.

| Machinery and inspected source | Decision and required change |
|---|---|
| `parser/engine.rs` in `deckmaste_english_v2` | Keep the Earley chart, packed forest, stateful scan, checked-completion callback and structured failure reporting. Its generic rule/category parameters already separate it from English constructions. Replace the static RHS lifetime restriction with a borrow from a prepared grammar so environment-owned frame rules can participate. No second parser. |
| `deckmaste_construction_core/src/{model,semantic,plan,validate}.rs` and `emit/` | Keep the parse/validate/plan/emit compiler and per-construction generated types, checked constructors, rendering and total visitors. Extend it for schema-checked heterogeneous frame sequences, explicit grammatical features and dependency summaries. No handwritten parallel AST. |
| `deckmaste_english_v2/src/environment.rs`, `core_verbs.ron`, `deckmaste_construction_core/src/macro_def.rs` | Keep immutable normalized provider rows, provenance, identity and collision validation. Unify core/plugin frame atoms; delete matching against the hand-enumerated codec tails. The environment prepares finite frame rules once, before parsing. |
| `parser/scan.rs`, generated `emit/scanner.rs` and `emit/terminal.rs` | Keep lexical matching, source spans, boundary state, Onset and identity provenance. Add frame/schema dispatch and grammatical feature constraints; preserve all lexical alternatives. Mid-word failure offsets are not evidence of a scanner split. |
| `parser/materialize.rs`, `emit/build.rs` | Keep memoized forest traversal and generated checked building. Separate category/feature/dependency admission from preference and packing. Context-dependent checks must not reject an open constituent before its enclosing context is available. |
| `parser/selection.rs` | Replace lexicographic structural specificity as grammatical arbitration. Keep typed ambiguity outcomes and diagnostic comparisons, with reasons changed to evidenced grammatical preference or scope-class membership. The empty exception registry gains no entries. |
| Scope projection/collapse in `parser/materialize.rs`, generated scope visitors | Adapt the existing derived-site representation; preserve the host/role/coordinator topology checks. Add complete correlated alternatives and frame anchorings as described below. Do not replace exact verification with hashes, equal text or anchor counts. |
| `render.rs`, `orthography.rs`, `parser/ownership.rs`, generated render/visit | Keep Writer, claim collection, contextual rendering and traversal checks. Extend their structural boundary transitions for nested quotes, supplements and document collections. Scope metadata owns no bytes and introduces no leaves. |
| `parser/diagnostic.rs`, xtask inspection/coverage/gates | Keep the real parser entry points, structured errors, direct AST inspection, per-unit accounting and structural gates. Adapt construction paths and selection reasons to the new generated types. No bracket-dump golden contract. |

### Formal-to-Rust mapping

| Formal contract | Production representation and evidence |
|---|---|
| `Category`, recursive `Syntax`, grammatical relations | Generated category enums and construction structs; typed role descriptors carry Subject/Object/Complement independently of the child category. NP recursion through clauses and relatives remains ordinary boxed generated structure. |
| `Lexicon`, `FrameItem`, `JudgeFrameIn` | A normalized schema registry, one lexical verb family and checked typed frame members. Core and plugin verbs consume the same registry. Compiled-consumer examples exercise new schemas supplied as data. |
| Form, Voice, Person, Number, finite agreement | Separate declared features and derived Concord Class. Clauses constrain transient lexical form choices; syntax retains grammatical distinctions needed to derive realization, not copied scanner spellings. Test mixed coordination and preterite/participle homographs. |
| `Features.Conforms` | Generated feature equations and checked constructors for countability, determiners, modifier order, temporal distribution and qualification/distributive placement. Check constructed values as well as parses. |
| `GapUse`, `Dependencies.Safe` | Ordered exposed-gap summaries carrying category and relation, composed through generated constructors; explicit discharge at relative/sharing constructions and boundary checks at adjuncts, subjects, quotes and coordination. No raw-text gap nodes. |
| `JudgesIn` recoverability context | A grammatical context folded through paragraph items, with form/voice summaries and explicit boundary behavior. It is separate from external `ParseContext` identity data. Neither resolves a Game Model Referent. |
| `Analysis.Reading` | A candidate enters final selection only after category/frame building, recursive feature checks, dependency/context checks and exact realization succeed. Early pruning is allowed only for context-independent failures. |
| `Analysis.Prefers`, `Selected` | Pairwise local declared-role and identity evidence, lifted through unchanged grammatical contexts. Qualification and distributive exclusions happen during admission. Finite survivor enumeration is order-independent; unrelated survivors remain a typed ambiguity. |
| `Analysis.Related`, `package_exact` | Verified grammatical move classes over complete checked candidates. The canonical AST plus derived correlated sites/anchorings describes exactly the retained alternatives on the migration witness sets. No preference for a conjunct boundary. |
| `Realizes`, `Spells`, `Surface` | Generated forms plus Writer/scanner structural transitions, with exact source claims. Rust roundtrip and ownership tests establish correspondence; the Lean relation is not a proof about Rust bytes. |
| Flat document/body/cost/keyword/type collections | Generated typed sequences with explicit cardinality and separators. Clause grammar supplies trigger-shaped initial adverbials; document position supplies colon, label, paragraph and text-box boundaries. |

### Runtime frame representation

The compiler generates one `LexicalVerbPhrase` construction family with a
checked sequence of typed frame members. Its finite member sum references the
ordinary grammatical categories: NP, AdjP, PP, measure, clause and VP, plus
explicit declared notation/keyword/quotation complements where required. Each
argument records its grammatical relation. A fixed lexical marker and a marked
Complement are distinct schema items; optionality belongs to the schema.
Member values are generated typed children, never strings, `Any`, or a second
universal syntax tree. Word-bearing schema items reference the owning lexical
inventory. Structural punctuation remains a form/notation concern.

Both core `Predicate([...])` and plugin `Custom(frames: [...])` normalize into
one atom language and interned `VerbFrame` registry. A key identifies a schema;
it no longer asks whether one of the grammar's named tail codecs happens to
match. Voice and lexical restrictions remain declarations. A passive retained
Object must be declared, never inferred from whatever follows a participle.

Prepare a finite rule set from the registry alongside the static generated
rules. Internal nonterminals identify `(schema, position)` and internal rule
IDs distinguish static productions, schema productions and root adapters.
Optional items generate skip/present transitions. Frame-part coordination uses
nonempty schema intervals; every conjunct is checked against the same interval.
Schema dispatch associates the scanned verb with its own registry entries.
Materialization folds those rules into the one generated member sequence and
checks length, category, relation and feature compatibility. Dynamic IDs do not
become public Construction identities or specificity weights. No leaked static
allocation, corpus-trained rule list, or per-verb runtime branch is needed.

The compiler ticket proves this engineering choice through the actual emitted
consumer before production activation: two lexemes sharing one schema, a new
ordered schema with no codec edit, an optional marked role, repeated frame
parts and a mismatched child rejection. The first production slice then replaces
all named tail codecs and their consumers together. This is the main uncertain
engineering cost; preparation size/time and parse work are measured, not assumed
cheap. A demonstrated limitation is reported against this design rather than
worked around with another hand-enumerated tail.

The compiler capability is declared with `frame_family` inside `constructions!`:

```rust
frame_family LexicalVerbPhrase {
    categories: [NounPhrase, PrepositionPhrase, MeasurePhrase],
    roles: [ObjectNounPhrase = Object(NounPhrase), Amount = Complement(MeasurePhrase)],
    coordinators: [Coordinator::And, Coordinator::Or, Coordinator::AndOr],
}
```

The generated module composes its prepared root with the invocation's ordinary
rules and typed child categories. `PreparedGrammar::new` resolves lexical rows
against that declared inventory; its `rules`, `scan` and `build` methods supply
the existing chart and materializer interfaces. `LexicalVerbPhrase::try_new`
checks head ownership and the complete member sequence. The finite child sum
retains the generated categories' feature carriers, and delegates their rendering
and traversal to the ordinary generated functions. Standalone child-root
punctuation does not enter an embedded frame.

`FrameItem` in `construction_core::macro_def` is the shared serialized language;
`CustomTailAtom` remains a compatibility alias for the immediate migration.
`Argument`, `Fixed`, `Marked` and `Optional` distinguish the grammatical cases.
The `roles` table resolves old core/plugin role spellings; new arguments state
their relation and category directly. Literal strings have no lexical owner and
are rejected by the prepared registry. The consumer ticket registers this
capability in the production category graph and removes the named tail codecs.

### Admission, selection and retained alternatives

The final pipeline is checked composition → feature/dependency/context
admission → evidenced preference → scope equivalence → canonical packaging.
An optimization can fuse passes only if its witness tests establish the same
survivors. Global structural-specificity ordering and exception-based arbitration
are retired when the selection migration lands. Do not remove them early while
unmigrated families still rely on them; until that landing they remain explicitly
transitional, and each earlier slice reports their selected-analysis effects.
The transition grants no permission to accept a newly wrong selection.

Retain the hoisted representative and generated derived metadata. Add
`AdmissibleAnchorings` for alternate frame-pair splits. Retain complete
correlations between mobile sites and anchorings: the enclosing host's derived
metadata records rows of jointly admissible assignments, obtained from complete
checked candidates. Existing per-mobile `AdmissibleSites` are projections of
those rows, not independent choices whose Cartesian product creates readings.
An assignment names structural paths and coordinator-leaf tuples, not another
stored subtree. Unambiguous hosts have no alternative rows. Metadata is derived
by the checked construction/canonicalization path, not author supplied.

A diagnostic expansion applies the declared moves/anchorings and rechecks each
result against the grammar. On each finite witness set, expansion must equal
the selected complete alternatives in both directions. This is a concrete test
of information preservation; neither an anchor-count invariant nor a smaller
packed count substitutes for it. Keep flat n-ary and nested binary coordination
distinct, preserve coordinator identities, lexical identities, role opacity and
unchanged wrappers. Choose a representative from the verified class: prefer
higher mobile sites outermost first, then break equal-height frame splits by earliest anchor tuple.
If the individual highest sites cannot coexist, retain an actual class member
and its correlated rows; never synthesize an inadmissible combination. That
ordering chooses presentation, never a reading to discard.

This retains the current AST strategy, extending its metadata for the actual
nested-mobile and frame-boundary witnesses. A first-class node containing
alternative subtrees and a general compressed alternative graph remain
unscheduled. The efficiency of the finite assignment table is less certain
than its straightforward correctness test; measure candidate/row counts and
report growth before adding compression.

### Replacement boundaries and iteration

Start with compiler capability in an emitted fixture, then activate the frame,
clause and agreement replacement together. Their current codec tails, relation
categories and form-specific predicate wrappers depend on each other; splitting
those into rename-only landings would leave adapters across the same recursion.
Nominal features and subordination form the next replacements, followed by
prepositions, extraction, scope/selection, measures and documents according to
the dependency graph. Document structure precedes discourse ellipsis so the
latter consumes a real paragraph context. Type-line ordering is independently
implemented at the existing open Type inventory.

Temporary coexistence is limited to the compiler fixture capability before its
first production consumer and to untouched grammatical families awaiting their
own replacement. Do not register both old and new derivations for the same
bracketing, add old-category aliases, or build a second parser for fallback.
When a slice changes a shared category, update its existing consumers in that
slice; a later ticket owns additional language capability, not adapters needed
to keep today's consumers compiling. New compiler syntax is tested through the
emitted consumer before use. Existing WIP results are assessed after integration;
no migration task edits someone else's WIP ticket or relies on its current diff.

Each implementation ticket includes its applicable formal elaborations and
positive/negative Rust witnesses. Extend Lean only for a concrete new grammar
claim the slice depends on, using LSP inspection; do not reopen the completed
review or demand global completeness. Develop on a focused supported subset
and finish with the standard whole-corpus accounting. Structural outcomes and
superseded-code deletion define completion; coverage and construction counts
remain observations. The closure ticket samples remaining failures and makes
the revisable decision to return to the tail loop.

### Reconciled inherited prescriptions

These decisions apply under the user's top-down migration authorization and
NLP-only clarification; the original tickets' examples survive in the
[obligation register](english-grammar-migration-obligations.md).

| Inherited prescription | Migration disposition |
|---|---|
| Group R capstone order; per-card frontier scheduling; add-only ratchet; whole-workspace tests | Replace with the dependency graph and current landing/gate contract. |
| Design from v1 or ignored postmortem documents | Use the reviewed model, glossary and independent style guide. Tracked done-ticket records remain provenance for named regressions. |
| Subject/Object as categories; per-tail codecs; tense/concord conflation | Replace together through the frame/clause slice. |
| Target Verb selects game-defined Spell/Ability Subjects | Remove that proposed guard. Preserve the marker/verb ambiguity probes and Infectious Curse obligation. The NLP grammar supplies grammatical constraints only; an unresolved lexical-category ambiguity needs an explicit ruling, not a semantic Subject whitelist. No claim that the reviewed scope relation already solves it. |
| Which/whose/pied-piping not required by the relative ticket | Include the reviewed forms and boundary checks in extraction migration. |
| One identical construction for every subordinator or polarity | Share categories/features and lexical inventory, but retain distinct constructions for different dependency or surface structures. Constructor-count minimization is not a law. |
| Cost constituents are ordinary phrase coordination; no category may contain “Cost” | Cost lists are flat document sequences of clauses/notation. Retire game-semantic predicate partitions; retain grammar for printed costs and symbols. |
| Type-order consumer first, declaration later; compiler changes forbidden | Migrate open Type schema, declarations, consumer and tests in one ticket. Compiler changes needed by grammar are authorized. |
| Every changed attachment pair is a hard tie | Verified scope alternatives pack; unrelated alternatives remain hard ties. Preserve the thirteen attachment cases and Class C/D witnesses without forcing one semantic attachment. |
| Independent site sets suffice; boundary variation needs no extended move | Keep derived metadata, but check complete alternatives and their correlations. Use the reviewed frame-boundary extension; preserve the production topology fences. |
| Semantics consumes English metadata; triggered envelope encodes rule meaning | No English/Semantics interaction. Initial adverbials and ordered conditions are clause grammar; textual envelopes retain only source structure. |

The Target Verb ambiguity is a known production risk, not a resolved theorem.
Its ticket remains a separate integration gate after the general replacements;
if its counterexample survives them, report the competing checked trees and seek
a grammatical/ambiguity ruling. Do not turn that one unresolved lexical rivalry
into another whole-grammar review.
