# English derived-family inventory

This is the executable migration inventory for
[`english-grammar-is-derived`](decisions/english-grammar-is-derived.md). It is a
snapshot of the active construction registry and ability parser, not a second
grammar authority. A migration updates this inventory when it flips a listed
row; production declarations and Rust types remain authoritative.

## Census derivation and accounting

The chart census comes from `RuleTag` in
`crates/deckmaste_english/src/grammar/mod.rs`. `RuleTag` derives `EnumIter` and
`IntoStaticStr` with `snake_case`, and
`grammar/construction.rs::handwritten_registry` maps every iterated tag to one
handwritten, fan-out-one chart row. The merged production registry adds two
already-generated rows, `noun_phrase_coordination` and
`shared_determiner_nominal`; they are not remaining work. The ability census
comes from the three non-chart entry points documented and dispatched by
`FragmentKind`: `Cost`, `KeywordLine`, and `Ability`.

| source | handwritten / ungenerated | already generated | accounted total |
|---|---:|---:|---:|
| chart construction registry | 190 | 2 | 192 |
| handwritten ability layer | 3 | 0 | 3 |
| migration inventory | 193 | 2 | 195 |

Every one of the 190 chart IDs occurs once in exactly one unit below. The unit
counts sum to 190. The three ability IDs occur once in A01. Thus the remaining
work is 193 families, with no `later` row. No raw corpus query was needed for
this accounting; the census is grounded in the registry and the current
`FragmentKind` dispatch. Future corpus evidence must use supported faces, and
normalized-template questions must use the existing English instruments.

## How to read the unit records

Each `ConstructionId` in a unit inherits every datum in that unit's dossier;
the lists are a compact per-family record, not an informal bucket. A unit is
the smallest vertical migration that can flip its rows without leaving a
second constructor or renderer authority. Rows are grouped only where they
share a public AST ingress, mutually recursive helper categories, or one
discontinuous coordination representation.

Owner abbreviations name current code, all under `deckmaste_english` unless
otherwise stated:

- `NR` — parse registration in `grammar/rules.rs`, recognition in
  `grammar/scan.rs`, reduction in `grammar/reduction.rs`, and lowering in
  `grammar/lowering.rs`.
- `CR` — parse registration in `grammar/clause/rules.rs`, reduction in
  `grammar/clause/reduction.rs`, and lowering in
  `grammar/clause/lowering.rs`.
- `OR` — `grammar/opacity.rs` for registration, scanning, reduction, and
  lowering.
- `AR` — `grammar/ability.rs`, including its chart callbacks, rather than the
  chart registry.
- `REN` — structural matching in `renderer.rs`.
- `SYN-P`, `SYN-C`, and `SYN-A` — public raw constructors in
  `syntax/phrase.rs`, `syntax/clause.rs`, and `syntax/ability.rs`.

All chart units currently use one-span `ConstructionBackend::Chart`. `C1` is
already expressible as fan-out-one whole-subtree declarations; `C2` first
needs a local scalar, identity, or field-slice/lens hole; `C3` is scheduled
after the hole primitives it uses and first needs selection, valency, gap, or
attachment constraints; `C4` first needs tuple-valued/discontinuous support,
or a measured CFG approximation plus filtering; `C5` needs the generated
ability-layer backend. Dependencies below are capability dependencies, not a
claim that a generated production cannot consume a still-handwritten subtree.

The active serialized consumer is `serde::Serialize` through
`deckmaste_spelling::view::of`; active frame consumers are
`deckmaste_spelling::{compile,unify,render,witness}` through
`parse_fragment`/`render_fragment`. `xtask` consumes parse provenance through
`english::{inspect,recovery,unknown_phrases}`. Every unit must migrate those
consumers when its shape changes. None of the syntax nodes currently exposes
a general `Deserialize` ingress; a migration that adds one must route it
through the generated validator.

Surface facts are classified against the Oracle style guide, which is
evidence rather than grammar authority. Meaning-bearing choices stay typed;
house-style facts are derived only when exactness proves them. In particular,
serial-comma and punctuation conventions use style guide §3, “Punctuation and
glyphs”; quantity spelling uses §4, “Numbers, quantities, and comparisons”;
self-reference and pronouns use §5, “Names, self-reference, pronouns, and
anaphora”; nominal selection uses §§6–7; ability frames and keyword lines use
§§8 and 14; and coordination uses §10, “Logic, choice, and coordination.”

## Migration-unit ledger

| unit | rows | class | migration ticket | actual capability needs |
|---|---:|---|---|---|
| Q01 quantity | 10 | C2 scalar | `english-derived-quantity-family` | inventory |
| D01 determiner and possession | 9 | C3 | `english-derived-determiner-possession-family` | scalar, identity, lens |
| J01 adjective and comparison | 9 | C3 | `english-derived-adjective-comparison-family` | scalar, identity, lens |
| N01 noun identity and opacity | 2 | C2 identity | `english-derived-noun-lexeme-family` | inventory |
| M01 nominal spine | 37 | C2 lens | `english-derived-nominal-family` | scalar, identity |
| P01 noun phrase | 19 | C3 | `english-derived-noun-phrase-family` | scalar and identity |
| P02 prepositional phrase | 2 | C3 | `english-derived-prepositional-family` | identity |
| V01 predicate spine | 34 | C3 | `english-derived-predicate-family` | scalar, identity, lens |
| F01 nonfinite clause | 4 | C3 | `english-derived-nonfinite-clause-family` | valency and form |
| F02 finite and copular clause | 18 | C3 | `english-derived-finite-clause-family` | valency and agreement |
| F03 clause attachment | 16 | C3 | `english-derived-clause-attachment-family` | valency, form, and attachment constraints |
| F04 clause coordination | 6 | C4 | `english-derived-clause-coordination-family` | finite agreement, structural design, tuple yields |
| R01 relative clause | 8 | C3 | `english-derived-relative-clause-family` | valency and gaps |
| S01 sentence | 1 | C1 | `english-derived-sentence-family` | inventory |
| C01 phrase coordination | 15 | C4 | `english-derived-phrase-coordination-family` | lens, valency, structural design, tuple yields |
| A01 ability layer | 3 | C5 | `english-ability-construction-backend` | inventory |

## Q01 — quantity

**Stable IDs (10):** `quantity_exact`, `quantity_at_least`, `quantity_or`,
`quantity_x`, `quantity_both`, `quantity_up_to`, `quantity_that_many`,
`quantity_that_much`, `quantity_more_than`, `quantity_fewer_than`.

- **Owners and AST:** NR → `Quantity`, `QuantityValue`, `NumberLiteral`, and
  `ComparativeWord`; REN renders the quantity inside determiners, modifiers,
  complements, and arithmetic values; SYN-P exposes the constructors.
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

**Stable IDs (9):** `determiner_closed`, `determiner_target`,
`determiner_quantified_target`, `determiner_quantity`,
`determiner_possessive_this_card`, `possessive_noun_base`,
`possessive_noun_determined`, `determiner_possessive_noun`,
`possessive_noun_adjective`.

- **Owners and AST:** NR → `Determiner`, `Possessor`,
  `PossessiveNounPhrase`, `Demonstrative`, and embedded `Quantity`; REN owns
  determiner/possessor spelling; SYN-P owns raw ingress.
- **Holes and constraints:** Quantity and possessive-NP subtrees, scalar and
  identity values, and a lens over the possessor phrase. Article/onset,
  cardinality, already-determined, and demonstrative agreement are required
  constraints.
- **Ambiguity/backend:** fan-out one on Chart. There is no direct dominance
  edge inside D01; `nominal_determiner` later dominates `nominal_comparison`.
- **Witnesses:** demonstrative and possessor identity are meaning-bearing;
  abbreviated/full self-reference remains stored where present. `a`/`an` is
  derived from onset and is house style; pronoun/self-reference spelling uses
  style guide §5.
- **Consumers and gates:** Nominal fragments and spelling views. Direct-AST
  fixtures cover each determiner and nested possessors; `inspect` pins article
  and cardinality rejections; exactness covers `a`/`an`, target quantities,
  and possessives; negatives reject double determination and wrong number or
  onset.

## J01 — adjective and comparison

**Stable IDs (9):** `adjective`, `adjective_phrase`,
`adjective_phrase_face_up`, `adjective_phrase_face_down`,
`adjective_phrase_comparison`, `adjective_phrase_degree_measure`,
`comparison_standard`, `comparison_than`,
`comparison_than_or_equal_to`.

- **Owners and AST:** NR → `AdjectivePhrase`, `AdjectiveComplement`,
  `ComparisonComplement`, and `ComparisonMarker`; REN owns adjective and
  comparison linearization; SYN-P owns constructors.
- **Holes and constraints:** adjective identity, numeric scalar, complement
  subtree/sum, and complement-vector lens. Comparison class/state, card
  orientation, onset, and predicative-only degree measure are selection
  constraints.
- **Ambiguity/backend:** fan-out one on Chart, no direct registry dominance
  edge. Competing noun/adjective/clause standards remain packed when viable.
- **Witnesses:** face orientation, comparison marker, and standard are
  meaning-bearing. Capitalization and compound spelling are vocabulary/style
  realizations; comparison number spelling follows style guide §4.
- **Consumers and gates:** Nominal/Sentence fragments and spelling frames.
  Direct-AST fixtures cover every complement sum and degree measure; inspect
  pins selected standard and comparison-state rejection; exactness covers
  face-up/down and both comparison markers; negatives reject a completed or
  wrong-class comparison and attributive degree measures.

## N01 — noun identity and lexical opacity

**Stable IDs (2):** `noun`, `noun_opaque`.

- **Owners and AST:** `noun` uses NR; `noun_opaque` uses OR. Both produce
  `NounInstance`/`Noun` (including `OpaqueLexeme`); REN owns noun spelling and
  SYN-P owns construction.
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

- **Owners and AST:** NR, including the late specialized rule builders, →
  `NominalPhrase`, `NominalModifier`, `NominalComplement`,
  `PredicatedArgument`, and the rules-object/reduced-passive helpers; REN owns
  nominal/modifier/complement output; SYN-P owns raw ingress.
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

**Stable IDs (19):** `noun_phrase_set_exception_bare`,
`noun_phrase_set_exception_for`, `noun_phrase_nominal`,
`rules_object_noun_phrase`, `noun_phrase_subject_pronoun`,
`noun_phrase_object_pronoun`, `noun_phrase_reciprocal`,
`noun_phrase_quantity`, `noun_phrase_this_card`,
`noun_phrase_full_this_card`, `noun_phrase_possessive_this_card`,
`noun_phrase_demonstrative`, `noun_phrase_partitive`,
`noun_phrase_each_partitive`, `noun_phrase_any_number_of`,
`noun_phrase_minus`, `noun_phrase_half`, `noun_phrase_half_rounded_up`,
`noun_phrase_half_rounded_down`.

- **Owners and AST:** NR → `NounPhrase` and its pronoun, self-reference,
  partitive, set-exception, and arithmetic variants; REN owns noun-phrase and
  arithmetic rendering; SYN-P owns constructors.
- **Holes and constraints:** nominal/quantity/NP subtree holes; identity holes
  for pronoun, reciprocal, demonstrative, and self-reference form; scalar and
  field holes for arithmetic and rounding. Agreement, pronoun case,
  coordination domain, set-exception host, notional plurality, and
  rules-object followup are constraints.
- **Ambiguity/backend:** fan-out one on Chart. `noun_phrase_nominal` dominates
  `noun_phrase_minus` and the already-generated
  `noun_phrase_coordination`; `noun_phrase_subject_pronoun` dominates
  `noun_phrase_object_pronoun`.
- **Witnesses:** pronoun, demonstrative, reciprocal, arithmetic operator, and
  rounding are meaning-bearing. Full versus abbreviated self-reference is a
  stored exact witness; realized name comes from identity. Pronoun and
  self-reference surface conventions use style guide §5.
- **Consumers and gates:** Nominal fragments, every clause/cost frame, spelling
  views, and inspect. Direct ASTs cover all variants and notional agreement;
  inspect pins the three dominance relations; exactness covers self-reference
  and rounding punctuation; negatives reject case/number mismatches,
  ineligible set exceptions, and malformed arithmetic.

## P02 — prepositional phrase

**Stable IDs (2):** `prepositional_phrase`, `prepositional_object`.

- **Owners and AST:** NR → `PrepositionalPhrase` and its noun-phrase,
  prepositional, gerund, and adverb object variants; REN owns linearization;
  SYN-P owns constructors.
- **Holes and constraints:** preposition identity plus one whole typed object
  subtree. Object variant, attachment role, and selected-complement versus
  adjunct use are constraints.
- **Ambiguity/backend:** fan-out one on Chart, no direct dominance edge; PP
  attachment ambiguities remain packed and resolved only by the owning
  nominal/predicate constraints and declared costs.
- **Witnesses:** preposition and object identity are meaning-bearing; spacing
  is derived house style. No free punctuation witness belongs to this unit.
- **Consumers and gates:** Nominal, predicate, cost, and sentence trees plus
  spelling views. Direct ASTs cover every object variant; inspect names the
  selected attachment owner; exactness covers nested PPs; negatives reject
  object-category and role violations.

## V01 — predicate spine

**Stable IDs (34):** `verb`, `verb_phrase_base`, `verb_phrase_auxiliary`,
`verb_phrase_auxiliary_proform`, `verb_phrase_direct_object`,
`verb_phrase_indirect_object`, `verb_phrase_adjective`,
`verb_phrase_prepositional`,
`verb_phrase_passive_shared_determiner_prepositional`,
`verb_phrase_except_by`, `verb_phrase_infinitive`, `verb_phrase_adverb`,
`verb_phrase_preverb_adverb`, `verb_phrase_particle`,
`verb_phrase_coin_result`, `verb_phrase_frequency`,
`frequency_phrase_adverb`, `verb_phrase_ability`,
`verb_phrase_quoted_ability`, `verb_phrase_quoted_ability_coordination`,
`verb_phrase_ability_quoted_coordination`, `verb_phrase_oracle_symbol`,
`verb_phrase_symbol_sequence`, `mana_amount_symbol`,
`mana_amount_sequence`, `mana_amount_list_single`,
`mana_amount_list_comma`, `mana_amount_coordination`,
`mana_amount_coordination_oxford`, `verb_phrase_mana_amount_coordination`,
`verb_phrase_power_toughness`, `verb_phrase_quantity`,
`verb_phrase_causative`, `frequency_phrase`.

- **Owners and AST:** CR for predicates, mana helpers, frequency, and the late
  `verb_phrase_causative`; these lower
  to `Predicate`, `PredicateHead`, `VerbPhrase`, `PredicateObject`,
  `PredicateComplement`, `PredicateAdjunct`, `FrequencyPhrase`, and typed mana
  sequences. REN owns predicate/element rendering; SYN-C owns ingress.
- **Holes and constraints:** identity holes for verbs, auxiliaries, particles,
  coin results, adverbs, and symbols; scalar holes for quantities and
  power/toughness; subtree holes for NP, PP, infinitive, adjective, and ability;
  lenses into pre-object and post-object element slices. Lexical valency,
  predicate form, voice, auxiliary agreement, direct/indirect object slots,
  causative frame, selected PP, and attachment phase are required constraints.
- **Ambiguity/backend:** fan-out one on Chart. `verb_phrase_auxiliary` dominates
  `verb_phrase_adjective`, `verb_phrase_adverb`, and `verb_phrase_ability`;
  `verb_phrase_base` dominates `verb_phrase_auxiliary_proform`. Existing
  attachment/precedence costs remain named tie-breakers.
- **Witnesses:** voice, form, object role, particle, conjunction, and event
  order are semantic. Pre/post-object position and keyword/quoted ordering are
  stored because they affect exact output. Mana-list Oxford commas and ordinary
  spacing are style evidence under §§3, 4, and 10.
- **Consumers and gates:** Sentence, Cost, KeywordLine, and Ability entry
  points, spelling frames, views, inspect, and recovery. Direct ASTs cover
  every valency and element position; inspect pins dominance, role, and cost;
  exactness covers pre/post-object order, mixed ability objects, mana lists,
  symbols, and particles; negatives reject surplus/missing arguments, wrong
  form/voice, illegal shared determiners, bad causatives, and malformed lists.

## F01 — nonfinite clause

**Stable IDs (4):** `infinitive_to`, `infinitive_not_to`,
`gerund_clause_base`, `gerund_clause_subordinate_after`.

- **Owners and AST:** CR → `InfinitiveClause` and `GerundClause`; REN and SYN-C
  own output and ingress.
- **Holes and constraints:** whole predicate/clause subtree holes plus the
  negation identity. Bare/infinitive/gerund form and subordinate attachment are
  required; there is no discontinuity.
- **Ambiguity/backend:** fan-out one on Chart, no direct dominance edge.
  Predicate valency rather than registration order licenses the complement.
- **Witnesses:** infinitive negation and subordinate position are
  meaning-bearing; `to` and spacing are derived fixed style.
- **Consumers and gates:** nominal and predicate complements, PP objects,
  complex clauses, spelling views. Direct AST and inspect fixtures cover both
  infinitives and gerund attachment; exactness covers `to`/`not to`; negatives
  reject wrong predicate forms and unlicensed dependent roots.

## F02 — finite and copular clause

**Stable IDs (18):** `simple_clause_subject`,
`simple_clause_subject_distributive_each`,
`simple_clause_contracted_subject`, `simple_clause_subjectless`,
`clause_simple`, `clause_elliptical`, `clause_existential`,
`copular_remainder_noun`, `copular_remainder_adjective`,
`copular_remainder_prepositional`, `copular_remainder_power_toughness`,
`copular_remainder_prepositional_adjunct`, `copular_remainder_adverb`,
`copular_remainder_negated`, `copular_remainder_distributive_each`,
`clause_copular`, `clause_contracted_copular`,
`clause_variable_value_constraint`.

- **Owners and AST:** CR → `IndependentClause`, `Predicate`, copular payloads,
  existential form, and the variable-value constraint; REN and SYN-C own
  output and ingress.
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

**Stable IDs (16):** `clause_adverb_before`,
`clause_sentence_adverbial_before`,
`clause_prepositional_before`, `clause_subordinate_before`,
`clause_subordinate_gerund_before`, `clause_subordinate_after_elliptical`,
`clause_subordinate_after`, `clause_subordinate_after_comma`,
`clause_subordinate_after_infinitive`, `exception_rider_single`,
`exception_rider_conjoined`, `exception_rider_comma`,
`exception_rider_oxford`, `clause_excepted`, `clause_restriction_run`,
`clause_restriction_member`.

- **Owners and AST:** CR, including late exception/restriction rules and NR's
  late fronted-gerund registration, → `ComplexClause`, positioned
  `ClauseAttachment`, exception riders, and restriction runs; REN and SYN-C
  own output and ingress.
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

**Stable IDs (6):** `clause_coordination`, `clause_coordination_comma`,
`clause_coordination_asyndetic`,
`clause_coordination_copular_noun_prepositional`,
`clause_coordination_copular_noun_prepositional_comma`,
`clause_coordination_copular_noun_prepositional_asyndetic`.

- **Owners and AST:** CR registers the three general rows; NR registers the
  three late shared-copular rows. CR owns substantive reduction/lowering for
  all six. They construct `IndependentClause::Coordinated`,
  `ClauseCoordination`, shared predicate continuations, and shared-copular
  continuations; REN and SYN-C own output and ingress.
- **Holes and constraints:** clause/simple-clause, noun-phrase, and PP
  subtrees; conjunction/comma identity; member sequences; and lenses into the
  shared subject, predicate, and copula. Finite agreement, subject presence,
  imperative adoption, conjunction class, copular agreement, continuation
  kind, and member scope are required.
- **Ambiguity/backend:** recognition uses one-span Chart productions, but
  total destruction of the flattened coordinated AST cannot assign every
  continuation one local subtree yield: subjectless predicates and copular
  continuations linearize with subject/agreement context stored outside the
  member. The six rows share that recursive ingress and renderer, so they
  migrate atomically after C4 tuple-valued/discontinuous declarations or a
  measured CFG approximation plus filtering. Packed complete-member versus
  shared-predicate readings stay visible.
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

The member-scoped predicate lists from
`english-coordination-structural-design` land here: per-member trailing
conditions and shared-subject `A, B, then C` are part of this migration, not a
permanent exception.

## R01 — relative clause

**Stable IDs (8):** `relative_object`,
`relative_object_contracted_subject`,
`relative_subject_contracted_auxiliary`, `relative_subject`,
`relative_subject_distributive_each`, `relative_contracted_copular_noun`,
`relative_contracted_copular_adjective`,
`relative_contracted_copular_prepositional`.

- **Owners and AST:** CR → `RelativeClause`, relative marker, gap, subject,
  and predicate/copular forms; REN and SYN-C own output and ingress.
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

- **Owners and AST:** CR → `Sentence`/`SentenceBody`; REN and SYN-A own output
  and ingress.
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

**Stable IDs (15):** `modifier_conjunct_adjective`,
`modifier_conjunct_noun`, `modifier_conjunct_negated`,
`modifier_list_single`, `modifier_list_comma`,
`coordinated_modifier_conjoined`, `coordinated_modifier_oxford`,
`nominal_coordinated_modifier`, `prepositional_phrase_list_pair`,
`prepositional_phrase_list_comma`,
`prepositional_phrase_sibling_coordinated`,
`verb_phrase_coordinated_adjective`,
`copular_remainder_coordinated_adjective`,
`relative_contracted_copular_coordinated_adjective`,
`nominal_power_toughness_complement`.

- **Owners and AST:** NR registers all fifteen rows and owns substantive
  reduction/lowering for the modifier, PP, and nominal rows. The top-level NR
  dispatch sends `verb_phrase_coordinated_adjective`,
  `copular_remainder_coordinated_adjective`, and
  `relative_contracted_copular_coordinated_adjective` to CR, whose reduction
  and lowering modules own their substantive handwritten logic. The rows
  construct modifier and PP sequences, `AdjectivePhraseCoordination`,
  `PrepositionalPhraseCoordination`, nominal complements, and the consuming
  nominal/predicate/copular/relative variants; REN plus SYN-P/SYN-C own output
  and ingress.
- **Holes and constraints:** typed sum members, field lenses, scalar
  power/toughness, conjunction/comma identity, recursive sequences, and
  member-specific attachments. Adjective-only predicative position,
  nominal attachment phase, repeated versus shared preposition, minimum
  arity, heterogeneous member type, and common-head versus head-list selection
  are required.
- **Ambiguity/backend:** current recognition is one-span Chart. Heterogeneous
  and member-scoped yields first require C4 tuple yields or a measured CFG
  approximation plus filtering. `nominal_prepositional` dominates
  `nominal_coordinated_modifier`; other competing common-head/head-list
  analyses remain packed until a declared structural discriminator selects.
- **Witnesses:** member grouping, conjunction, repeated preposition, and
  per-member attachment are semantic. Oxford/asyndetic punctuation is house
  style and remains stored until exactness proves it derivable (style guide
  §§3 and 10).
- **Consumers and gates:** Nominal/Sentence/Ability fragments, spelling views,
  inspect. Direct ASTs cover every member type, two/three-plus arity, shared and
  repeated prepositions, predicative coordination, and local P/T grouping;
  inspect pins common-head/head-list and attachment choices; exactness covers
  all comma/conjunction spellings; negatives reject binary Oxford commas,
  bare comma runs, non-adjective predicatives, and illegal mixed members.

All remaining outputs of `english-coordination-structural-design` enter this
unit at their first compatible site: mixed keyword/quoted `with` lists,
or-coordinated appositive bodies with quantity postmodifiers, and principled
common-head versus heterogeneous head-list selection. They are acceptance
scope, not deferred residue. The compiler work also closes the pilot's named
sequence-member quantification, typed sum/variant mapping, presence-valued
comma, and emitted total own-mode linearizer requirements.

## A01 — ability layer

**Stable IDs (3):** `cost`, `keyword_line`, `ability`.

- **Owners and AST:** AR's `parse_cost_fragment`/`parse_cost`,
  `parse_keyword_line_fragment`/`parse_keyword_list`, and
  `parse_ability_fragment`/`parse_ability`; REN's `cost`,
  `keyword_ability_list`, and `ability`; SYN-A's `Cost`,
  `KeywordAbilityList`, `Ability`, and all frame payloads.
- **Holes and constraints:** heterogeneous clause/NP/symbol/recovery cost
  components and alternatives; keyword item sequences with typed arguments;
  full ability-frame sums containing paragraphs, costs, triggers, modal modes,
  chapters, roll rows, level bands, station thresholds, loyalty, and nested
  abilities. Top-level punctuation, nesting, catalog selection, frame shape,
  and attachment to chart subtrees are required.
- **Ambiguity/backend:** these rows have no chart productions or registry rows
  today. C5 supplies one generated ability backend and registers the stable
  IDs. Frame priority and attempted alternatives become declared dominance or
  explicit unique guards; parser function order cannot remain semantic.
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

## Completion invariants

Each migration ticket must land its declaration, parse/render/build
projections, active consumer and serialized-view changes, direct-AST and
`inspect` fixtures, exactness and negative fixtures, registry flip, and
handwritten parse/reduction/lowering/renderer/constructor deletion audit in
one change. It may not defer any of those to completion.

The completion node depends on all fifteen chart tickets and the existing
ability-backend ticket. Its final audit therefore has 193 newly migrated plus
two already-generated families—195 generated families in all—to prove, with no
handwritten or unregistered family left to discover.
