# English derived-family inventory

This is the executable construction-family inventory for
[`english-grammar-is-derived`](decisions/english-grammar-is-derived.md). It is a
snapshot of the active construction registry and ability parser, not a second
grammar authority. Production declarations and Rust types remain authoritative;
this document records their stable IDs, contracts, and executable census.

## Census derivation and accounting

The census comes from the generated production registry in
`grammar/construction.rs`. Its Chart backend contributes the ten Q01 quantity
rows, the ten D01 determiner/possession rows, the 11 J01 adjective rows, the
37 M01 nominal rows, S01 `sentence`, N01 `noun`/`noun_opaque`, the two earlier
generated coordination rows plus all 14 stable C01 declarations and nine
internal C01 support declarations, all 36 V01
predicate rows, the four F01 nonfinite-clause
rows, all 20 F02 finite-clause rows, all 16 F03 clause-attachment rows, all six
F04 clause-coordination rows and the internal `simple_clause_subjectless_attached`,
`coordinated_predicate_attachment`, `coordinated_predicate_attachment_comma`,
`coordinated_predicate_attachment_elliptical`,
`shared_copular_predicate`, `shared_grant_ability_complement`,
`shared_grant_quoted_complement`, `shared_grant_base`,
`shared_grant_prefix_start`, `shared_grant_prefix_continue`,
`clause_coordination_shared_grant`,
`clause_coordination_shared_grant_comma`, and
`clause_coordination_shared_grant_asyndetic` adapters, and all
20 P01 noun-phrase rows, both P02 prepositional-phrase rows, and all nine R01
relative-clause rows. The ability
census comes from the three non-chart entry points documented and dispatched by
`FragmentKind`: `Cost`, `KeywordLine`, and `Ability`.

| backend | registered generated families |
|---|---:|
| Chart | 227 |
| Ability | 3 |
| all production families | 230 |

The family ledger below, its structural owners, and the registered internal
support declarations bring the active chart registry to 227. Every stable
chart row is generated.
The three ability IDs occur once in the generated ability backend. Thus the
inventory has no unregistered row. No raw corpus query was needed for this
accounting; the census is grounded
in the registry and the current `FragmentKind` dispatch. Future corpus
evidence must use supported faces, and normalized-template questions must use
the existing English instruments.

## How to read the unit records

Each `ConstructionId` in a unit inherits every datum in that unit's dossier;
the lists are a compact per-family record, not an informal bucket. A unit is
the smallest vertical family that shares one construction authority. Rows are grouped only where they
share a public AST ingress, mutually recursive helper categories, or one
recursive coordination representation.

Each record names its generated declaration, semantic AST, checked ingress,
linearization, and active consumers directly. These are collaborating parts of
one derived family, not independent ownership routes.

All chart units currently use one-span `ConstructionBackend::Chart`. `C1` is
already expressible as fan-out-one whole-subtree declarations; `C2` first
needs a local scalar, identity, or field-slice/lens hole; `C3` is scheduled
after the hole primitives it uses and first needs selection, valency, gap, or
attachment constraints; `C4` needs recursive coordination owners, typed member
sums, sequence-wide feature predicates, and local structural selection; `C5`
needs the generated ability-layer backend. Every current chart unit remains
fan-out-one on Chart. Dependencies below are capability dependencies, not a
claim that a generated production cannot consume another generated subtree.

The active structural consumer is `ConstructionProjection` through
`deckmaste_spelling::{compile,unify,render,witness}` and its `ProjectionTree`;
serialized Rust field or variant layout is not an English construction
contract. `xtask` consumes typed syntax and construction provenance through
`english::{inspect,recovery,unknown_phrases}` and aggregate parser work through
`english::performance`. Every unit must update those consumers when its shape
changes. None of the syntax nodes currently exposes a general `Deserialize`
ingress; a declaration that adds one must route it through the generated
validator.

Surface facts are classified against the Oracle style guide, which is
evidence rather than grammar authority. Meaning-bearing choices stay typed;
house-style facts are derived only when exactness proves them. In particular,
serial-comma and punctuation conventions use style guide §3, “Punctuation and
glyphs”; quantity spelling uses §4, “Numbers, quantities, and comparisons”;
self-reference and pronouns use §5, “Names, self-reference, pronouns, and
anaphora”; nominal selection uses §§6–7; ability frames and keyword lines use
§§8 and 14; and coordination uses §10, “Logic, choice, and coordination.”

## Construction-unit ledger

| unit | rows | class | source ticket | actual capability needs |
|---|---:|---|---|---|
| Q01 quantity | 10 | C2 scalar | `english-derived-quantity-family` | generated |
| D01 determiner and possession | 10 | C3 | `english-derived-determiner-possession-family` | generated |
| J01 adjective and comparison | 11 | C3 | `english-derived-adjective-comparison-family` | scalar, identity, lens |
| N01 noun identity and opacity | 2 | C2 identity | `english-derived-noun-lexeme-family` | generated |
| M01 nominal spine | 37 | C2 lens | `english-derived-nominal-family` | generated |
| P01 noun phrase | 20 | C3 | `english-derived-noun-phrase-family` | generated |
| P02 prepositional phrase | 2 | C3 | `english-derived-prepositional-family` | generated |
| V01 predicate spine | 36 | C3 | `english-derived-predicate-family` | scalar, identity, lens |
| F01 nonfinite clause | 4 | C3 | `english-derived-nonfinite-clause-family` | generated |
| F02 finite and copular clause | 20 | C3 | `english-derived-finite-clause-family` | generated |
| F03 clause attachment | 16 | C3 | `english-derived-clause-attachment-family` | generated |
| F04 clause coordination | 6 | C4 | `english-derived-clause-coordination-family` | generated |
| R01 relative clause | 9 | C3 | `english-derived-relative-clause-family` | generated |
| S01 sentence | 1 | C1 | `english-derived-sentence-family` | generated |
| C01 phrase coordination | 14 | C4 | `english-derived-phrase-coordination-family` | typed member sums, sequence predicates, head/member selection |
| A01 ability layer | 3 | C5 | `english-ability-construction-backend` | inventory |

## Q01 — quantity

**Stable IDs (10):** `quantity_exact`, `quantity_at_least`, `quantity_or`,
`quantity_x`, `quantity_both`, `quantity_up_to`, `quantity_that_many`,
`quantity_that_much`, `quantity_more_than`, `quantity_fewer_than`.

- **Construction and AST:** generated declarations → `Quantity`, `QuantityValue`,
  `NumberLiteral`, and `ComparativeWord`; the generated inverse renders the
  quantity inside determiners, modifiers, complements, and arithmetic values;
  checked quantity builders expose the public AST ingress.
- **Holes and constraints:** scalar holes for values and numeral notation,
  and no discontinuity. `ComparativeWord` is a closed four-variant enum handled
  by Q01's finite bound-value mapping and existing typed lexeme spelling form,
  not an open identity hole; generic identity-hole support first lands in N01.
  Cardinality, `X`, mass/count behavior, and literal-one tests are semantic
  feature flow.
- **Ambiguity/backend:** fan-out one on Chart; no declared dominance edge.
  Existing per-form costs and any equal-cost alternatives remain explicit.
- **Witnesses:** numeral notation and `ComparativeWord` are stored exactness
  witnesses; bound direction and literal value are semantic. Spelled-out
  versus digit conventions are house style (style guide §4) and may be
  derived only after exact replay proves them.
- **Consumers and gates:** Nominal and Sentence fragments, the spelling view,
  and inspect. Direct-AST fixtures cover every `Quantity` variant; `inspect`
  covers a competing quantity/nominal attachment; exactness covers every
  stored notation/word; negatives reject missing bounds, singular violations,
  and invalid `X` combinations.

## D01 — determiner and possession

**Status:** generated.

**Stable IDs (10):** `determiner_closed`, `determiner_all_the`, `determiner_target`,
`determiner_quantified_target`, `determiner_quantity`,
`determiner_possessive_this_card`, `possessive_noun_base`,
`possessive_noun_determined`, `determiner_possessive_noun`,
`possessive_noun_adjective`.

- **Construction and AST:** the D01 declaration and its generated chart, feature,
  lowering, and inverse adapters are the single construction authority for
  sealed `Determiner` and `Possessor`, the checked possessive projection over
  `NominalPhrase`, `Demonstrative`, and embedded `Quantity`. Public callers use
  checked builders and read-only semantic projections; serde keeps the legacy
  determiner/possessor view shape without reopening raw ingress.
- **Holes and constraints:** Quantity and possessive-NP subtrees, scalar and
  identity values, and a lens over the possessor phrase. Article/onset,
  cardinality, already-determined, and demonstrative agreement are required
  constraints. The composed `all the` value requires a plural or mass noun.
- **Ambiguity/backend:** fan-out one on Chart. There is no direct dominance
  edge inside D01; `nominal_determiner` later dominates `nominal_comparison`.
- **Witnesses:** demonstrative and possessor identity are meaning-bearing;
  abbreviated/full self-reference remains stored where present. `a`/`an` is
  derived from onset and is house style; pronoun/self-reference spelling uses
  style guide §5.
- **Consumers and gates:** Nominal fragments and spelling views. Direct-AST
  fixtures cover each determiner and nested possessors; `inspect` pins article
  and cardinality rejections; exactness covers `a`/`an`, target quantities,
  possessives, and both words of `all the`; negatives reject double
  determination, wrong number, and wrong onset.

## J01 — adjective and comparison

**Status:** generated.

**Stable IDs (11):** `adjective`, `adjective_phrase`,
`adjective_phrase_face_up`, `adjective_phrase_face_down`,
`adjective_phrase_comparison`, `adjective_phrase_degree_measure`,
`comparison_standard`, `comparison_than`,
`comparison_than_or_equal_to`, `adjective_phrase_prepositional`,
`adjective_phrase_infinitive`.

- **Construction and AST:** the J01 declaration and generated adapters →
  `AdjectivePhrase`, `AdjectiveComplement`, `ComparisonComplement`, and
  `ComparisonMarker`; generated inverse dispatch owns linearization and checked
  builders own ingress.
- **Holes and constraints:** adjective identity, numeric scalar, complement
  subtree/sum, and complement-vector lens. The declared recursive owner roles
  cover the whole lexical prepositional/infinitival posthead chain. Comparison
  class/state, card orientation, onset, and predicative-only degree measure are
  selection constraints.
- **Ambiguity/backend:** fan-out one on Chart, no direct registry dominance
  edge. Competing noun/adjective/clause standards remain packed when viable.
- **Witnesses:** face orientation, comparison marker, and standard are
  meaning-bearing. Capitalization and compound spelling are vocabulary/style
  realizations; comparison number spelling follows style guide §4.
- **Consumers and gates:** Nominal/Sentence fragments and spelling frames.
  Direct-AST fixtures cover every complement sum, the recursive declared
  posthead chain, and degree measure; inspect pins selected standard and
  comparison-state rejection; exactness covers face-up/down and both
  comparison markers; negatives reject a completed or wrong-class comparison
  and attributive degree measures.

## N01 — noun identity and lexical opacity

**Stable IDs (2):** `noun`, `noun_opaque`.

**Status:** generated.

- **Construction and AST:** the `noun` and `noun_opaque` declarations plus the
  generated chart adapter produce `NounInstance`/`Noun` (including
  `OpaqueLexeme`). Generated inverse dispatch consumes their total identity
  linearizers; the syntax module retains the public value types.
- **Holes and constraints:** identity holes for vocabulary/catalog nouns and
  exact opaque spelling. Noun form, onset, coordination domain, adjunct class,
  and recipient-passive eligibility flow as inherent features.
- **Ambiguity/backend:** fan-out one on Chart, no dominance edge. Opacity cost
  remains an explicit recovery dispreference, never a silent lexical choice.
- **Witnesses:** catalog identity is semantic vocabulary; opaque spelling is a
  stored exactness escape. Case and subtype/type spelling follow style guide
  §§2 and 7.
- **Consumers and gates:** every higher phrase family, spelling views, recovery
  and unknown-phrase instruments. Direct-AST and inspect fixtures distinguish
  known from opaque nouns; exactness replays opaque bytes; negatives prevent a
  known noun from being demoted behind an opaque head or opacity from entering
  exact mode.

## M01 — nominal spine

**Status:** generated.

**Stable IDs (37):** `nominal_noun`, `nominal_adjective`,
`nominal_noun_modifier`, `nominal_combat_step_name`,
`nominal_negated_modifier`, `nominal_quantity_modifier`,
`nominal_power_toughness_modifier`, `nominal_determiner`,
`nominal_prepositional`, `nominal_infinitive`,
`nominal_quantity_complement`, `nominal_keyword_symbol_argument`,
`predicated_quality_from`, `predicated_argument_from_single`,
`predicated_argument_from_extend`, `nominal_keyword_predicated_argument`,
`predicated_quality_bare`, `predicated_argument_bare_single`,
`predicated_argument_bare_extend`,
`nominal_keyword_atom_carried_predicated_argument`, `nominal_relative`,
`rules_object_nominal_base`, `rules_object_followup_nominal_relative`,
`rules_object_followup_nominal_prepositional`,
`nominal_reduced_recipient_passive`, `reduced_recipient_passive_theme`,
`reduced_recipient_passive_nominal_adjunct`,
`nominal_postpositive_adjective`,
`nominal_postpositive_adjective_conjoined_prepositional`,
`nominal_postpositive_adjective_conjoined`,
`nominal_postpositive_adjective_asyndetic`,
`nominal_postpositive_adjective_oxford`, `nominal_comparison`,
`nominal_devotion`, `devotion_color_single`, `devotion_color_pair`,
`nominal_times_clause`.

- **Owners and AST:** generated declarations and their typed adapters →
  `NominalPhrase`, `NominalModifier`, `NominalComplement`,
  `PredicatedArgument`, and the rules-object/reduced-passive helpers; the
  generated nominal inverse owns M01 output. `NominalPhrase` exposes read-only
  accessors and a checked bare-noun constructor rather than public writable
  fields, while its serialized view remains output-only.
- **Holes and constraints:** subtree holes for noun, adjective, determiner,
  quantity, PP, relative, infinitive, and clause; scalar/identity holes for
  power/toughness, symbols, keyword atoms, colors, and conjunction; field
  lenses into ordered modifiers/complements and the head. Attachment phase,
  comparison completion, shared determiner, rules-object role, keyword-grant
  conjunction, reduced-passive frame, onset, and cardinality are required.
- **Ambiguity/backend:** fan-out one on Chart. Declared dominance is:
  `nominal_quantity_modifier` over `nominal_prepositional`,
  `nominal_postpositive_adjective`, and `nominal_comparison`;
  `nominal_determiner` over `nominal_comparison`; `nominal_prepositional` over
  `nominal_infinitive`, `nominal_coordinated_modifier`,
  `nominal_keyword_predicated_argument`, and `nominal_noun`;
  `nominal_reduced_recipient_passive` over `nominal_noun`; and
  `nominal_relative` over `nominal_prepositional`.
- **Witnesses:** head/modifier/complement order, comparison, keyword argument,
  and attachment owner are semantic. Conjunction identity is semantic;
  asyndetic/Oxford comma fields remain stored unless measured derivable.
  Hyphenation, number spelling, and serial commas are house style under style
  guide §§3–4 and 10.
- **Consumers and gates:** Nominal fragments are a direct spelling frame
  category; serialized views expose every field. Direct-AST fixtures cover
  each modifier/complement and the rules-object/reduced-passive shapes;
  inspect pins every listed dominance/role decision; exactness covers stored
  commas, keyword spellings, and ordering; negatives reject illegal attachment
  phases, conjunctions, article agreement, and unlicensed passive or keyword
  heads.

## P01 — noun phrase

**Stable IDs (20):** `noun_phrase_set_exception_bare`,
`noun_phrase_set_exception_for`, `noun_phrase_nominal`,
`rules_object_noun_phrase`, `noun_phrase_subject_pronoun`,
`noun_phrase_object_pronoun`, `noun_phrase_reciprocal`,
`noun_phrase_quantity`, `noun_phrase_this_card`,
`noun_phrase_full_this_card`, `noun_phrase_possessive_this_card`,
`noun_phrase_demonstrative`, `noun_phrase_partitive`,
`noun_phrase_each_partitive`, `noun_phrase_any_number_of`,
`noun_phrase_targets_beyond_first`,
`noun_phrase_minus`, `noun_phrase_half`, `noun_phrase_half_rounded_up`,
`noun_phrase_half_rounded_down`.

- **Owners and AST:** the P01 declaration owns generated chart parsing, feature
  lowering, checked builders, and inverse linearization for `NounPhrase` and
  its pronoun, self-reference, partitive, set-exception, arithmetic, and
  rules-object forms. `NounPhrase` seals an owning, public
  `NounPhraseKind`; named self-reference genitives and `any number of` are
  direct alternatives rather than compatibility nominal spines.
- **Holes and constraints:** nominal/quantity/NP subtree holes; identity holes
  for pronoun, reciprocal, demonstrative, and self-reference form; scalar and
  field holes for arithmetic and rounding. The closed distributive
  `each target beyond the first` alternative keeps that grammatical boundary
  typed without admitting `beyond` as an open preposition. Agreement, pronoun case,
  coordination domain, set-exception host, notional plurality, and
  rules-object followup are constraints.
- **Ambiguity/backend:** fan-out one on Chart. `noun_phrase_nominal` dominates
  `noun_phrase_minus` and the already-generated
  `noun_phrase_coordination`; `noun_phrase_subject_pronoun` dominates
  `noun_phrase_object_pronoun`; and `noun_phrase_any_number_of` dominates the
  formal-singular `noun_phrase_nominal` reading when both complete.
- **Witnesses:** pronoun, demonstrative, reciprocal, arithmetic operator, and
  rounding are meaning-bearing. Full versus abbreviated self-reference is a
  stored exact witness; realized name comes from identity. `AnyNumberOf`
  stores notional plural agreement and its selected complement. Pronoun and
  self-reference surface conventions use style guide §5.
- **Consumers and gates:** Nominal fragments, every clause/cost frame, spelling
  views, and inspect. Direct ASTs cover all variants and notional agreement;
  inspect pins the four direct dominance relations; exactness covers self-reference
  and rounding punctuation; negatives reject case/number mismatches,
  ineligible set exceptions, and malformed arithmetic.

## P02 — prepositional phrase

**Status:** generated.

**Stable IDs (2):** `prepositional_phrase`, `prepositional_object`.

- **Owners and AST:** generated declarations own parsing, lowering, inverse
  rendering, checked building, and direct typed storage for
  `PrepositionalPhrase` and its noun-phrase, prepositional, gerund, and adverb
  object variants. A sealed `PrepositionalObject` owns that sum; no generic
  `Phrase` compatibility value sits behind it.
- **Holes and constraints:** preposition identity plus one whole typed object
  subtree. Object variant, adverb lexical class, attachment role, and
  selected-complement versus adjunct use are constraints.
- **Ambiguity/backend:** fan-out one on Chart, no direct dominance edge; PP
  attachment ambiguities remain packed and resolved only by the owning
  nominal/predicate constraints and declared costs.
- **Witnesses:** preposition and object identity are meaning-bearing. A shared
  preposition over a coordinated noun phrase remains distinct from sibling
  phrases that repeat their prepositions; spacing is derived house style. No
  free punctuation witness belongs to this unit.
- **Consumers and gates:** Nominal, predicate, cost, and sentence trees plus
  spelling views. Direct ASTs cover every object variant; inspect names the
  selected attachment owner; exactness covers nested PPs. The ability-owned
  `with "…"` postmodifier is an explicit predicate adjunct, not a hidden P02
  value. Negatives reject object-category, lexical-class, role, and private
  wrapper bypasses.

## V01 — predicate spine

**Status:** generated.

**Stable IDs (36):** `verb`, `verb_phrase_base`, `verb_phrase_auxiliary`,
`verb_phrase_auxiliary_proform`, `verb_phrase_direct_object`,
`verb_phrase_indirect_object`, `verb_phrase_adjective`,
`verb_phrase_pronominal_resultative_prepositional`,
`verb_phrase_prepositional`,
`verb_phrase_passive_shared_determiner_prepositional`,
`verb_phrase_except_by`, `verb_phrase_infinitive`, `verb_phrase_adverb`,
`verb_phrase_preverb_adverb`, `verb_phrase_particle`,
`verb_phrase_coin_result`, `verb_phrase_frequency`,
`frequency_phrase_adverb`, `frequency_phrase`, `verb_phrase_ability`,
`verb_phrase_quoted_ability`, `verb_phrase_quoted_ability_coordination`,
`verb_phrase_ability_quoted_coordination`, `verb_phrase_oracle_symbol`,
`verb_phrase_symbol_sequence`, `mana_amount_symbol`,
`mana_amount_sequence`, `mana_amount_list_single`,
`mana_amount_list_comma`, `mana_amount_coordination`,
`mana_amount_coordination_oxford`, `verb_phrase_mana_amount_coordination`,
`verb_phrase_power_toughness`, `verb_phrase_quantity`,
`verb_phrase_counted_energy`,
`verb_phrase_causative`.

- **Construction and AST:** the predicate declaration and its generated chart,
  lexical, feature, contextual object-gap/reduced-passive, lowering, exact,
  and inverse adapters are the single V01 authority. These lower to
  `Predicate`, sealed `PredicateHead`/`HeadedPredicate`, `VerbPhrase`,
  `PredicateObject`, `PredicateComplement`, `PredicateAdjunct`,
  `FrequencyPhrase`, and typed mana sequences. The production renderer
  reconstructs the exact retained lexical frame through the sealed ingress
  and invokes the generated inverse; no parallel V01 element renderer or
  writable public construction path remains. Public construction-in-progress
  is opaque and can only be advanced through checked builders; finished
  payloads expose read-only semantic accessors. The three A01 Ability-backend
  entry points consume predicate declarations without owning them.
- **Holes and constraints:** identity holes for verbs, auxiliaries, particles,
  coin results, adverbs, and symbols; scalar holes for quantities and
  power/toughness; subtree holes for NP, PP, infinitive, adjective, and ability;
  lenses into pre-object and post-object element slices. Lexical valency,
  predicate form, voice, auxiliary agreement, direct/indirect object slots,
  causative frame, selected PP, and attachment phase are required constraints.
  The counted-energy dependent is a sealed quantity-plus-`{E}` carrier; its
  internal generated owner admits positive cardinal words and `X` while the
  existing cost parser continues to consume the resulting imperative clause.
  The pronominal-resultative row retains a pronoun direct-object distinction
  long enough to compose a destination PP, past-participle resultative, and
  trailing PP without matching any verb, adjective, preposition, or card name.
- **Ambiguity/backend:** fan-out one on Chart. Declaration-owned
  `verb_phrase_auxiliary` dominates
  `verb_phrase_adjective`, `verb_phrase_adverb`, and `verb_phrase_ability`;
  `verb_phrase_base` dominates `verb_phrase_auxiliary_proform`. Existing
  attachment/precedence costs remain named declaration outputs, including the
  structural attachment count that keeps reduced-passive frequency from being
  reclassified as a nominal adjunct.
- **Witnesses:** voice, form, object role, particle, conjunction, and event
  order are semantic. Pre/post-object position and keyword/quoted ordering are
  stored because they affect exact output. Mana-list Oxford commas and ordinary
  spacing are style evidence under §§3, 4, and 10.
- **Consumers and gates:** Sentence, Cost, KeywordLine, and Ability entry
  points all report generated V01 provenance; spelling frames, serialized
  views, verbose inspect, and recovery traverse the same production owner.
  Spelling agreement addresses the declared `verb` construction's `head`
  identity and its sealed `VerbAnalysis` citation operation; no flat
  predicate/verb-slot projection or serialized field path is part of that
  contract.
  Direct ASTs cover
  every valency and element position; inspect pins dominance, role, and cost;
  exactness covers pre/post-object order, mixed ability objects, mana lists,
  symbols, and particles; negatives reject surplus/missing arguments, wrong
  form/voice, illegal shared determiners, bad causatives, and malformed lists.

## F01 — nonfinite clause

**Status:** generated.

**Stable IDs (4):** `infinitive_to`, `infinitive_not_to`,
`gerund_clause_base`, `gerund_clause_subordinate_after`.

- **Owners and AST:** the F01 declaration and its generated chart, typed-feature,
  lowering, inverse-render, and checked-build projections are the single
  authority for sealed `InfinitiveClause` and `GerundClause` values. Public
  callers use checked builders and read-only semantic projections. A gerund is
  exactly `GerundClauseKind::Base` or the binary recursive
  `GerundClauseKind::RatherThan { matrix, alternative }`; it has no generic
  attachment vector.
- **Holes and constraints:** whole predicate/clause subtree holes. Infinitive
  negation is preserved by the selected typed construction; infinitive/gerund
  form, lexical valency, and subordinate attachment are required constraints.
  There is no discontinuity.
- **Ambiguity/backend:** fan-out one on Chart, no direct dominance edge.
  Predicate valency rather than registration order licenses the complement.
- **Witnesses:** infinitive negation and recursive left/right association are
  meaning-bearing, while
  the fixed after-matrix position, absent comma, `rather than` subordinator,
  `to`, and spacing are derived fixed style.
- **Consumers and gates:** nominal and predicate complements, PP objects,
  complex clauses, spelling views. Direct AST and inspect fixtures cover both
  infinitives and gerund attachment; exactness covers `to`/`not to` and nested
  `rather than`; negatives reject wrong predicate forms, raw construction, and
  unlicensed dependent roots.

## F02 — finite and copular clause

**Status:** generated.

**Stable IDs (20):** `simple_clause_subject`,
`simple_clause_subject_distributive_each`,
`simple_clause_contracted_subject`, `simple_clause_subjectless`,
`clause_simple`, `clause_elliptical`, `clause_existential`,
`copular_remainder_noun`, `copular_remainder_adjective`,
`copular_remainder_coordinated_adjective`,
`copular_remainder_prepositional`, `copular_remainder_power_toughness`,
`copular_remainder_catalog_atom`,
`copular_remainder_prepositional_adjunct`, `copular_remainder_adverb`,
`copular_remainder_negated`, `copular_remainder_distributive_each`,
`clause_copular`, `clause_contracted_copular`,
`clause_variable_value_constraint`.

- **Owners and AST:** the F02 declaration and generated chart, typed-feature,
  lowering, inverse-render, and checked-build projections own
  `IndependentClause`, staged `SimpleClause`/copular payloads, existential
  form, and the variable-value constraint. Generated attachment and
  coordination consumers keep
  using the staged categories without recreating F02 productions.
- **Holes and constraints:** subject, predicate, and copular-complement
  subtrees; auxiliary/modal/negation identities; scalar power/toughness and
  variable values; lenses into predicate heads and copular adjuncts. Subject–
  verb agreement, distributive `each`, contraction, predicate valency,
  existential number, and copular complement class are required.
- **Ambiguity/backend:** fan-out one on Chart. `clause_simple` dominates
  `clause_copular`; other viable copular attachment alternatives remain
  inspectable with their costs.
- **Witnesses:** modality, negation, contraction, existential form, subject,
  and complement are meaning-bearing or exact distinctions. Capitalization
  and contractions follow style guide §§1 and 5; no source guess is allowed.
- **Consumers and gates:** Sentence/Ability/Cost fragments, spelling frames,
  views, inspect. Direct ASTs cover each independent/copy form; inspect pins
  agreement and the dominance edge; exactness covers contractions and
  existential spellings; negatives reject agreement, wrong complement class,
  contradictory distributive/contraction state, and non-numeric variable
  constraints.

## F03 — clause attachment

**Status:** generated.

**Stable IDs (16):** `clause_adverb_before`,
`clause_sentence_adverbial_before`,
`clause_prepositional_before`, `clause_subordinate_before`,
`clause_subordinate_gerund_before`, `clause_subordinate_after_elliptical`,
`clause_subordinate_after`, `clause_subordinate_after_comma`,
`clause_subordinate_after_infinitive`, `exception_rider_single`,
`exception_rider_conjoined`, `exception_rider_comma`,
`exception_rider_oxford`, `clause_excepted`, `clause_restriction_run`,
`clause_restriction_member`.

- **Owners and AST:** the F03 declaration and generated chart, typed-feature,
  lowering, inverse-render, and checked-build projections own `ComplexClause`,
  positioned `ClauseAttachment`, exception riders, and restriction runs.
- **Holes and constraints:** clause/gerund/infinitive/PP/adverb subtrees,
  conjunction identities, member sequences, and lenses into attachment lists.
  Attachment position/owner, subordinator class, finite/nonfinite form,
  exception host, restriction role, and member scope are required.
- **Ambiguity/backend:** fan-out one on Chart. Every right-hand side is a
  contiguous CFG production; C3 attachment/selection constraints are the
  first missing capability. Attachment costs and packed scope alternatives
  stay visible; no registration-order choice may survive.
- **Witnesses:** conjunction, sequence, condition scope, and attachment
  position are semantic. `ClauseAttachment::comma` remains stored where the
  attachment topology cannot derive it. Exception/restriction serial commas
  are derived from member count and conjunction under the existing measured
  exactness law (style guide §§3 and 10).
- **Consumers and gates:** Sentence/Ability fragments, spelling frames and
  views, inspect. Direct ASTs cover every position, exception/restriction fold,
  and attachment scope; inspect exposes scope and viable alternatives;
  exactness covers every comma form; negatives reject binary Oxford commas,
  orphan dependents, wrong subordinators, and attachment to an ineligible
  host.

## F04 — clause coordination

**Status:** generated.

**Stable IDs (6):** `clause_coordination`, `clause_coordination_comma`,
`clause_coordination_asyndetic`,
`clause_coordination_copular_noun_prepositional`,
`clause_coordination_copular_noun_prepositional_comma`,
`clause_coordination_copular_noun_prepositional_asyndetic`.

**Internal support IDs (13):** `simple_clause_subjectless_attached`,
`coordinated_predicate_attachment`, `coordinated_predicate_attachment_comma`,
`coordinated_predicate_attachment_elliptical`,
`shared_copular_predicate`, `shared_grant_ability_complement`,
`shared_grant_quoted_complement`, `shared_grant_base`,
`shared_grant_prefix_start`, `shared_grant_prefix_continue`,
`clause_coordination_shared_grant`,
`clause_coordination_shared_grant_comma`, and
`clause_coordination_shared_grant_asyndetic`.

- **Owners and AST:** the clause declaration and generated chart, typed-feature,
  lowering, inverse-render, and checked-build projections own all six rows.
  They construct `IndependentClause::Coordinated`, `ClauseCoordination`, shared
  predicate continuations, and shared-copular continuations. The public clause
  facade is the checked ingress; syntax carriers expose immutable accessors.
  Internal shared-grant owners preserve an elided repeated `has` as a typed
  realization witness while keeping each condition on its own predicate.
- **Holes and constraints:** clause/simple-clause, noun-phrase, and PP
  subtrees; conjunction/comma identity; member sequences; and lenses into the
  shared subject, predicate, and copula. Finite agreement, subject presence,
  imperative adoption, conjunction class, copular agreement, continuation
  kind, and member scope are required.
- **Ambiguity/backend:** fan-out one on Chart. Canonical `FiniteClause` owns a
  shared subject once and `PredicateExpression::Coordinated` owns its
  contiguous predicate members; `IndependentClause::Coordinated` owns
  complete members. Subjectless and shared-copular continuations are therefore
  invertible structural views, not tuple-valued yields. Packed complete-member
  versus shared-predicate readings stay visible.
- **Witnesses:** conjunction, member grouping, shared-subject/copula scope,
  and continuation kind are semantic. `ClauseCoordination::comma` is stored
  because the AST and measured corpus do not determine it; exact replay must
  preserve it (style guide §§3 and 10).
- **Consumers and gates:** Sentence/Ability fragments, spelling frames and
  views, inspect. Direct ASTs cover complete members, subjectless predicate
  continuations, shared copulas, agreement, and member-scoped conditions;
  inspect exposes scope and viable alternatives; exactness covers every comma
  and conjunction form; negatives reject invalid agreement, conjunction,
  shared-subject, and copular continuations.

Per-member trailing conditions and shared-subject `A, B, then C` are owned by
the same generated coordination family rather than a separate exception.

## R01 — relative clause

**Status:** generated.

**Stable IDs (9):** `relative_object`,
`relative_object_contracted_subject`,
`relative_subject_contracted_auxiliary`, `relative_subject`,
`relative_subject_distributive_each`, `relative_contracted_copular_noun`,
`relative_contracted_copular_adjective`,
`relative_contracted_copular_coordinated_adjective`,
`relative_contracted_copular_prepositional`.

- **Owners and AST:** the R01 declaration and its generated chart, feature,
  lowering, inverse-render, and checked-build projections are the single
  authority for sealed `RelativeClause`, relative marker, gap, subject, and
  predicate/copular forms. Public callers use checked builders and immutable
  semantic accessors; R01 also owns the coordinated-adjective relative while
  C01 owns only the coordinated adjective phrase that fills its complement.
- **Holes and constraints:** NP, VP, adjective, and PP subtree holes plus
  relative-marker/auxiliary identity. Subject/object gap, relativizer,
  predicate valency, contraction, agreement, and distributive `each` are
  required.
- **Ambiguity/backend:** fan-out one on Chart. `relative_subject` dominates
  `relative_object`; every remaining gap ambiguity stays packed and visible.
- **Witnesses:** relativizer (including zero), gap, contraction, and subject
  are meaning-bearing/exact. Demonstrative `that` inside the body must not be
  guessed as a relativizer; style guide §5 supplies surface evidence.
- **Consumers and gates:** nominal complements and clause trees, spelling
  views, inspect. Direct ASTs distinguish subject/object/zero gaps and each
  contracted copular form; inspect pins the dominance edge and decisive gap;
  exactness covers zero/explicit markers and contractions; negatives reject
  impossible valency, agreement, and gap combinations.

## S01 — sentence

**Stable ID (1):** `sentence`.

**Status:** generated.

- **Construction and AST:** the `sentence` declaration and generated chart
  adapter own the independent-clause form of `Sentence`; generated inverse
  dispatch linearizes the declaration and the syntax facade routes that ingress
  through its checked builder. Choice, power/toughness, non-initial trigger,
  recovery, and dash-appositive sentence bodies are internal carriers of the
  A01 Ability backend, not additional S01 chart forms.
- **Holes and constraints:** one whole `Clause` subtree; no local hole or
  selection upgrade is needed.
- **Ambiguity/backend:** fan-out one on Chart, no dominance edge. The two
  period/no-period productions share one construction and one AST.
- **Witnesses:** the terminal period is derivable from sentence/terminal-quote
  structure and is house style (style guide §3); it is intentionally not
  stored.
- **Consumers and gates:** Sentence fragments, paragraphs, costs, spelling
  frames/views, inspect. Direct AST and inspect cover ordinary and
  quote-terminated sentences; exactness covers with/without consumed period;
  negatives reject dependent-only roots and doubled punctuation.

## C01 — phrase coordination

**Stable IDs (14):** `modifier_conjunct_adjective`,
`modifier_conjunct_noun`, `modifier_conjunct_negated`,
`modifier_list_single`, `modifier_list_comma`,
`coordinated_modifier_conjoined`, `coordinated_modifier_oxford`,
`nominal_coordinated_modifier`, `prepositional_phrase_list_pair`,
`prepositional_phrase_list_comma`,
`prepositional_phrase_sibling_coordinated`,
`verb_phrase_coordinated_adjective`,
`nominal_power_toughness_complement`, `nominal_with_attributes`.

- **Construction and AST:** generated C01 declarations own all fourteen rows and
  their Chart reduction/lowering for the modifier, PP, nominal, and predicate
  carriers. F02 and R01 consume the coordinated adjective category through
  their own generated declarations. The C01 rows
  construct modifier and PP sequences, `AdjectivePhraseCoordination`,
  `PrepositionalPhraseCoordination`, nominal complements, and the consuming
  nominal and predicate variants; the dedicated generated
  `WithAttributeMember`/`WithAttributeList` sum and `nominal_with_attributes`
  construction own mixed keyword/quoted-ability `with` complements. Generated
  inverse dispatch plus checked phrase/clause facades own output and ingress.
- **Holes and constraints:** typed sum members, field lenses, scalar
  power/toughness, conjunction/comma identity, recursive sequences, and
  member-specific attachments. Adjective-only predicative position,
  nominal attachment phase, repeated versus shared preposition, minimum
  arity, heterogeneous member type, and common-head versus head-list selection
  are required. Common-head selection is constructional: each modifier must
  independently fill the trailing head's attributive slot. It does not require
  a shared MTG semantic class. A head list is selected when the members each
  carry their own complete head; the determiner and outer complements stay on
  the resulting owner.
- **Ambiguity/backend:** fan-out one on Chart. Shared context belongs to an
  explicit nominal, prepositional phrase, finite clause, or predicate owner;
  members and their local attachments are contiguous. Heterogeneous slots use
  a construction-local closed sum. `nominal_prepositional` dominates
  `nominal_coordinated_modifier`. Common-head eligibility is based on
  recursive constructional substitutability, not a uniform semantic or
  catalog class. Other equally viable readings remain packed.
- **Witnesses:** member grouping, conjunction, repeated preposition, and
  per-member attachment are semantic. The checked modifier, adjective, and
  mixed-`with` carriers derive Oxford/asyndetic punctuation from arity,
  position, and conjunction; exact replay tests prove that topology is
  sufficient (style guide §§3 and 10).
- **Consumers and gates:** Nominal/Sentence/Ability fragments, spelling views,
  inspect. Direct ASTs cover every member type, two/three-plus arity, shared and
  repeated prepositions, predicative coordination, local P/T grouping, and the
  dedicated mixed `with` member sum; inspect pins common-head/head-list,
  mixed-`with`, and attachment choices; exactness covers all comma/conjunction
  spellings; negatives reject binary Oxford commas, bare comma runs,
  non-adjective predicatives, and mixed members outside attributive `with`.
  Corpus acceptance fixtures include Alien Invasion, Basilica Shepherd, and
  Blink for mixed `with`; Abzan Monument or Deceptive Landscape, Grassland
  Crusader, Open the Gates or District Guide, Monument to Perfection,
  Banishing Slash or Summon: Yojimbo, and Cowabunga! or Kirri, Talented Sprout
  for common-head and complete-head-list coverage.

The phrase-side outputs of `english-coordination-structural-design` enter this
unit at their first compatible site: `with_attribute_member_keyword`,
`with_attribute_member_quoted`, the recursive `with_attribute_list_*`
declarations, and `nominal_with_attributes` form a dedicated mixed
keyword/quoted `with` member sum. Common-head versus head-list selection is
principled and constructional: `an Elf, Orc, or enchantment creature you
control`, `basic, Sphere, or Locus land card`, and `Mutant, Ninja, Turtle, or
land card` are common-head modifier coordinations, while `a basic land card or
Gate card` is a complete-head list. Sycorax Commander's outer `then`/`or`
grouping belongs to F04; its
`that many cards minus one` subtree is a supporting P01 admission/selection
repair, not a coordination variant. These are acceptance scope, not deferred
residue. The compiler work is limited to any still-missing generic feature
flow, named sequence-member predicates, presence-valued comma, lenses, and
total own-mode linearization; no tuple-yield backend lands here.

## A01 — ability layer

**Stable IDs (3):** `cost`, `keyword_line`, `ability`.

**Status:** generated.

- **Construction and AST:** the A01 declaration and its generated Ability backend own
  the three root constructions, their checked build/destructure projections,
  stable parse provenance, and inverse dispatch for `Cost`,
  `KeywordAbilityList`, and `Ability`. Declaration-owned typed component and
  frame callbacks consume the existing clause, noun-phrase, symbol, recovery,
  keyword-argument, paragraph, and frame payloads. The syntax facade seals all
  three roots behind private storage and read-only semantic projections; their
  derived serialization is output-only, and public callers use the `cost`,
  `keyword_line`, and `ability` checked facades.
- **Holes and constraints:** heterogeneous clause/NP/symbol/recovery cost
  components and alternatives; keyword item sequences with typed arguments;
  full ability-frame sums containing paragraphs, costs, triggers, modal modes,
  chapters, roll rows, level bands, station thresholds, loyalty, and nested
  abilities. Its declaration module also owns the ability-internal sentence
  carriers that do not have standalone S01 chart forms, including the checked
  coordinated-choice dash appositive. Top-level punctuation, nesting, catalog
  selection, frame shape,
  and attachment to chart subtrees are required.
- **Ambiguity/backend:** one generated Ability backend registers the three
  stable IDs and is filtered out of chart assembly. Cost and keyword-line roots
  select uniquely; ability-frame alternatives use explicit guard ranks, and
  registration and candidate permutation are semantic-neutral.
- **Witnesses:** cost components, target/choice structure, keyword arguments,
  headers, ranges, modes, and frame kinds are meaning-bearing. Flavor/ability
  words, separator forms, modal suffix, contractions, punctuation, and
  capitalization are stored only where exact replay cannot derive them.
  Surface evidence is style guide §§3, 8, 9, 14, and 15.
- **Consumers and gates:** the three direct `FragmentKind` entry points,
  whole `OracleText`, quoted/nested abilities, all spelling frame/view paths,
  inspect, recovery, and unknown-phrase reporting. Direct ASTs cover every
  `CostComponent`, keyword argument, and `AbilityKind`; inspect exposes the
  chosen stable ID/guard and alternatives; exactness covers every frame and
  stored separator; negatives reject malformed costs, keyword tails, orphan
  modes, empty effects, invalid ranges, and frame collisions.

## Steady-state invariants

Every family change lands its declaration, parse/render/build projections,
active consumer and serialized-view changes, direct-AST and `inspect` fixtures,
exactness and negative fixtures, and the parallel-authority deletion audit in
one change. It also runs the build-excluding parent/current
`english performance --check` audit from `docs/english-parser-performance.md`
so grammar work cannot hide growth in failed or abandoned paths.

The executable census proves 227 Chart families plus three Ability families:
230 generated production families in all, with no owner branch or unregistered
supported family left to discover.
