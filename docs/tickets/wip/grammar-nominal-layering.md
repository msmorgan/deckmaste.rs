---
needs: []
---
# Nominal layering

Replace the flat recursion on `Nominal` with category layers so that the order in which modifiers are applied to one head stops producing extra trees. Today `Nominal` wraps itself for premodifiers, postmodifiers and relatives alike, so n premodifiers and one postmodifier give n+1 bracketings: "red creatures you control" has 2 trees, "red artifact creatures you control" 3, "red nonlegendary artifact creatures you control" 4. An estimate from a stratified sample puts the removable trees at about 78,000 of 482,041 (plausibly 57,000 to 106,000; the heavy tail was under-sampled).

## The gate, built first

A restricted normal-form check in the census tool, with no grammar annotations. The normal form of a Reading is the tree itself except that, on one connected nominal-head chain, the interleaving of commuting premodifier and postmodifier applications is ignored: `(base tree, ordered premodifier records, ordered postmodifier records)`. Order within each side is kept. Each modifier record keeps its constructor, form, optional Category field and every non-head field, recursively normalised. Everything else is compared exactly: no global sorting, no transparent wrappers.

- The wrappers it knows today: `PremodifiedNominal`, `NounPremodifiedNominal`, `ObjectRelativeNominal`, `SubjectRelativeNominal`, `PostmodifiedNominal`, `ParticipialPostmodifiedNominal`, `NamedNominal`. After layering it needs an adapter from the new layers to the same modifier records.
- Commuting modifiers, to start: the colour words, *nonlegendary*, adjectival *tapped*, card-type noun premodifiers. Everything else is a barrier: *other*, *another*, focus words, determiners, cardinals, superlatives, comparatives, targeting and slash-modified premodifiers, postpositive, participial and coordinated premodifiers. The list is stated in one place, by lexical class where a class exists, and is extended only with a reason; "nonred", "enchanted" and creature-subtype premodifiers ("Goblin creatures you control") are the first candidates and are not yet in it.
- The census records each unit's set of normal forms; a new baseline is taken with them. The check fails on a lost face, an incomplete enumeration, incompatible unit boundaries, or any unit whose set of normal forms differs. A tree lost whose normal form has no survivor is a failure, never a report line.
- It must keep distinct: a modifier attached to a different head; a different lexeme, form or variant; relative-gap shape; scope of a barrier modifier; coordination hosts; selected frames; quotation boundaries. Required cases, with expected counts of trees and normal forms, are the 30 probed in `docs/evidence/grammar-review-2026-10-11/analysis-key-measurement.md` (for example "other red creatures you control" 3 trees, 2 normal forms; "Destroy target creature with flying." 2 and 2; "Artifact spells you cast have convoke." 4 and 2).
- Landing the check alone changes no Reading: identical digests.

## The change

- Layers as plain categories: a nominal core, then postmodifiers and relatives, then premodifiers, then the noun phrase. That order (premodifiers outside postmodifiers) is a convention for commuting modifiers, where the two bracketings are one analysis; CGEL treats such a nominal as structurally ambiguous with usually no practical difference (CGEL, Ch. 5, §14.2, pp. 446–447). A barrier modifier keeps every scope it has today.
- The "has this already happened" features that police the flat recursion go where layering makes them unnecessary; each removal is listed.
- Design first and stop: the layer categories and which existing constructions land in which layer, how barrier modifiers keep both scopes, how coordination of nominals and fused heads fit, what happens to each policing feature, and the inventory of existing tests that pin nominal tree shapes with their re-spelling for the same witnesses. Tests are re-spelled, never deleted.

## Gate

No face lost; per unit, the set of normal forms unchanged while tree count falls; full tests. Note: census delta (faces, Readings before and after, lost faces none); the judgment-set line does not apply yet.

## Phase 1: restricted check (2026-10-13)

Implemented in `xtask`, without grammar or lexicon changes. Census schema 10
adds `nominal_normal_form_version: 1` and a sorted set of SHA-256 fingerprints
on every completely enumerated, valid unit. Reading digest encoding remains 2.
`english-v3 diff BEFORE AFTER --check nominal-normal-forms` requires identical
face populations, analyzed sources, unit boundaries/categories, surrounding
structure and per-unit normal-form sets. Missing or incomplete evidence is an
error. Schema 9 remains usable for the separate `identical-digests` check.

The implementation parses the existing Reading Debug identity into labelled
records; quoted strings are atomic, field order and all payloads are retained.
Only the seven reviewed wrappers and the reviewed lexical inventory commute.
An unknown node stays exact; an altered recognized wrapper shape is an error,
not a silently ignored field. New layer constructors will require the explicit
adapter described below. The recursive-construction inventory test covers the
15 directly declared recursive Nominal constructions; the two schema uses and
coordination are reviewed separately in the placement table below.

Full corpus: 32,828 faces; 30,794 read; 2,034 unread; 75,412 units;
482,041 Readings; zero incomplete enumerations or validation issues. Against
`baseline-v2-closed-class.json`, `--check identical-digests` passed on all
32,828 faces: no source, count or digest changes. The resulting version-1 `target/english-v3/nominal-baseline.json` was later
found to predate the final derived-id guard; its normal-form sets are stale.
Its raw Reading digests remain valid evidence. The corrected baseline and the
source-identity guard are recorded below.

An uncommitted scratch census build suppressed actual enumerated Readings
before both digests were computed, without modifying saved fingerprints:

| Scratch deletion | Trees before → after | Normal forms before → after | Check |
|---|---:|---:|---|
| One order twin in “Destroy red creatures you control.” | 2 → 1 | 1 → 1 | Pass |
| The relative-outside-`other` scope in “Destroy other red creatures you control.” | 3 → 2 | 2 → 1 | Failure: one lost key |
| One PP attachment in “Destroy target creature with flying.” | 2 → 1 | 2 → 1 | Failure: one lost key |

These are fault injections into the census enumerator, not a claim that a
layered grammar has been implemented. The scratch mutation was restored;
the rebuilt clean command reproduced the original seven Readings with
identical digests on all three fixture faces. Scratch census files were deleted.

Tests include all 30 measured judgments, the three unapproved modifiers,
lexical form/variant/frame/quotation retention, ordered multiplicity,
recognized-shape failures, recursive-construction inventory, and negative
metadata/evidence cases. A derived card-type id is explicitly excluded from
the base card-type inventory; adding a `/non` derivative cannot silently expand
the whitelist.

## Phase 2: proposed layering — awaiting owner review

No grammar conversion is committed in this phase. Standard constraints apply.
The check is deliberately narrower than a general dependency analysis.

### Categories and scope

Use two additional plain Categories, `NominalCore` and `PostmodifiedNominal`,
with the existing `Nominal` as the complete nominal category. The category and
construction named `PostmodifiedNominal` may coexist, as many existing pairs
already do; alternatively `NominalPostmodification` is a less overloaded name.
Recommendation: `NominalPostmodification` for the Category in the sketches below.

1. `NominalCore` contains bare and selected heads, and **barrier** constructions.
2. `NominalPostmodification` lifts a core and recursively adds the five approved
   postmodifier/relative wrappers.
3. `Nominal` lifts that layer and recursively adds approved commuting adjective
   and noun premodifiers.

A barrier's head is a **full `Nominal`**, while its result is `NominalCore`.
This resets the layers without moving anything through the barrier. It retains
both `other(red(creatures + relative))` and
`other(red(creatures)) + relative`. All inner subtrees remain independently
layered. There is no empty cycle: each reset consumes its modifier; the two
lifts are unidirectional. Within-side order is never changed.

The two new interfaces expose the same 28 slots as Nominal, with propagation
modes but without coordination annotations unless that category coordinates.
Each lift must forward **all** slots, including today's local
`NominalDependentPresent`, `ScalarMeasurementHeadUse` and `PropertyValuePosition`;
head inheritance alone does not forward local slots. Retain absence where the
old summary has absence. No generic compiler feature is proposed.

| Existing construction or binding | Proposed result / head | What remains exact |
|---|---|---|
| `Noun`, `BareFramedNoun`, `MeasuredSymbolNominal`, `SelectedComplementNominal`, `SymbolComplementNominal`, `PrepositionComplementNominal` | `NominalCore`; existing lexical or selected head | Lexeme, form, frame, marker and complement. The bare taxonomic complement remains a full Nominal constrained by its existing requirements. |
| `PremodifiedNominal` | `Nominal`; head `Nominal`; commuting simple adjective only | Existing modifier subtree and all current admission/summary expressions. |
| `NounPremodifiedNominal` | `Nominal`; head `Nominal`; commuting card-type noun only | Existing noun modifier subtree and head restrictions. |
| Proposed `BarrierPremodifiedNominal`, `BarrierNounPremodifiedNominal` | `NominalCore`; head `Nominal`; complementary barrier use | Same old forms/fields, tests and lexical choices; adapter maps to the old corresponding modifier record. |
| `PostmodifiedNominal`, `ParticipialPostmodifiedNominal`, `NamedNominal`, `SubjectRelativeNominal`, `ObjectRelativeNominal` | `NominalPostmodification`; head `NominalPostmodification` | PP/participle/name/relative subtree, gap kind, agreement and all admission tables. |
| `CoordinatedPremodifiedNominal`, `SuperlativePremodifiedNominal`, `CardinalPremodifiedNominal`, `CardinalMeasuredNominal`, `ParticipialPremodifier`, `TargetedNominal`, `SlashModifiedNominal`, `CorrelativePostpositiveNominal` | `NominalCore`; head `Nominal` | Entire barrier, including every inside/outside scope that it has today. |
| `DiscontinuousComparativeNominal`'s Nominal instance | `NominalCore`; head `Nominal` | Governor, selected comparison, ordering and the old `category: Nominal` meaning in the adapter. Its NounPhrase instance stays unchanged. |
| `PostpositiveNominal`'s Nominal instance | `NominalCore`; head explicitly `Nominal` (not `Self`) | Complemented adjective is a barrier. Its fused-head NounPhrase instance stays unchanged. |
| Nominal `Coordination`, `SerialCoordination` and helpers | completed result `NominalCore`, members full `Nominal` | Conjunct boundaries, coordinator, all summary combiners, serial requirements; explicitly retain old helper names and old category identity through the adapter. |

The existing generator supports distinct result and member Categories. The
intended declaration is `coordinable NominalCore of Nominal using Standard`,
with today's `NominalSeries` helper name stated explicitly. The old `Nominal`
combine annotations move to the coordinating result interface; they must not
remain on a non-coordinable category. A modifier outside this core still
modifies the entire coordination, not each conjunct. No coordination grouping
is flattened by this ticket.

Determiners, `another`, focusing words and fused heads remain at their current
NounPhrase boundaries; their full Nominal children receive layers, but the
check never crosses those boundaries. A focused or coordinated adjective is
not a simple commuting adjective. Named nominals use the approved post layer;
the selected naming predicate and catalog identity remain intact. Subject,
object and adverbial-gap relatives remain different records with different
relative subtrees; no gap is inferred or erased.

### Commuting versus barrier lexical uses: decision needed

Today's lexical properties do not distinguish the reviewed inventory from all
other simple adjective and noun modifiers. Positive `in Class` requirements
cannot express the complementary barrier route, and making two overlapping
routes would recreate ambiguity. Do not introduce negated requirements or
another compiler capability for this.

Recommendation: a validated lexical property
`NominalModifierOrder { Barrier, Commuting } default Barrier`, set in the data
or its existing lexical producer for exactly the reviewed inventory. Expose it
locally on AdjectivePhrase and NounPremodifier; only the simple `Adjective` and
`NounPremodifier` constructions export the lexical value. Other adjective
constructions retain Barrier, even when a child word is red. The noun-category
premodifier form exports it; the keyword-category form remains Barrier. The two
nominal modifier routes require complementary values. This is a grammatical
use property, not a named reference to an open-class word. It changes lexical
metadata, not ids, forms, variants, frames or lexical values.

The tool's reviewed whitelist stays independent. A grammar property cannot
quietly widen what the gate permits. Pin agreement of the simple-use property
with that list in tests, including negative modifiers. Before Phase 3, confirm
that this small lexical-data change is acceptable; Phase 1 changes no lexicon.

### Core declarations, before and proposed after

The following sketches show changed routing; each body keeps today's complete
requirements and exports. Ellipses mean unchanged declarations, not proposed
syntax or omitted admission checks.

```rust
// Before
construction Noun: Nominal { ... }
construction PremodifiedNominal: Nominal {
    form [modifier: AdjectivePhrase, " ", head: Nominal];
    ...
}
construction ObjectRelativeNominal: Nominal {
    form [head: Nominal, " ", relative: ObjectRelativeClause];
    ...
}
```

```rust
// Proposed after
feature NominalModifierOrder { Barrier, Commuting } default Barrier;
construction Noun: NominalCore { ... }
construction NominalCoreProjection: NominalPostmodification {
    form [head: NominalCore];
    export NominalDependentPresent = head.NominalDependentPresent;
    export ScalarMeasurementHeadUse = head.ScalarMeasurementHeadUse;
    export PropertyValuePosition = head.PropertyValuePosition;
}
construction NominalProjection: Nominal {
    form [head: NominalPostmodification];
    export NominalDependentPresent = head.NominalDependentPresent;
    export ScalarMeasurementHeadUse = head.ScalarMeasurementHeadUse;
    export PropertyValuePosition = head.PropertyValuePosition;
}
construction PremodifiedNominal: Nominal {
    form [modifier: AdjectivePhrase, " ", head: Nominal];
    require modifier.NominalModifierOrder = Commuting;
    ...
}
construction BarrierPremodifiedNominal: NominalCore {
    form [modifier: AdjectivePhrase, " ", head: Nominal];
    require modifier.NominalModifierOrder = Barrier;
    ...
}
construction ObjectRelativeNominal: NominalPostmodification {
    form [head: NominalPostmodification, " ", relative: ObjectRelativeClause];
    ...
}
```

`red(artifact(creatures + relative))` becomes the sole order for the core
three-tree example. `other(red(creatures + relative))` and
`other(red(creatures)) + relative` still have different barrier placement.
Neither attachment of “Destroy target creature with flying.” is retired.

### Reading adapter and assurance

Keep existing constructor names and payload field order for unchanged uses.
Routing a constructor to a different fixed Category need not add a stored
Category field. The two lifts add explicit `form: 0, head` nodes. Strip **only
those exact shapes**, with checked categories where applicable, in the adapter.
The two new barrier constructors map to their old modifier constructors,
retaining form and all fields. Schema/coordination result categories that
change from Nominal to NominalCore need individually listed inverse mappings;
never globally rename Category values, since a category can distinguish a
coordination host elsewhere. Preserve every other category/form value.

Both the legacy Reading and the adapted layered Reading must produce the same
normal-form encoding. The adapter extends recognized input shapes without
changing the equivalence. Under the source-identity guard, any normalizer edit
requires a new version and an unchanged-grammar baseline before conversion. Reject unknown
forms/fields of the new recognized nodes. Assert separately that the grammar
admits and realizes the independently built **new** trees: adapting old trees
is a comparison operation, not an admission bypass.

The reference tree with all approved premodifiers outside the post layer must
still pass every existing requirement. In particular, property-value,
selected-preposition, source and agreement summaries cannot be assumed to
commute merely because the surface modifiers do. Phase 3 must compare these
routes on the corpus and independent witnesses. A lost normal form is a STOP,
not a reason to change the adapter or whitelist.

### Features retained, rather than prematurely deleted

| Feature | Remaining admission use / reason to keep |
|---|---|
| `NominalDependentPresent` | `HeavyObject` frame category; bare taxonomic noun complement requires No; nominal adjunct and fused-head licensing. Layer membership cannot replace these external contracts. |
| `ComparativeModifierPresent` | `bare_noun_phrase` licensing distinguishes implicit comparative modifiers. Comparative barriers remain. |
| `SourceModifierPresent` | `SourceFreeObject` frame category and selected-object/source-marker licensing distinguish attachments. |
| `WholeObjectSourcePresent` | `WholeSourceFreeObject` and genitive/PP propagation distinguish a source of the whole object from a nested source. |
| `SubjectRelativeNumber`, `NominalAgreementNumber`, `number`, `NumberTransparency` | Relative agreement and transparent quantity heads; layering does not decide these. |
| `Targeting`, `DeterminerRequirement`, `NominalComplementMarker` | Target projection, determination and selected versus free noun complements remain distinct. |
| `PropertyValuePosition`, `ScalarMeasurementHeadUse`, `MeasurePosition`, `CardinalMeasurementUse` | Scalar and measured uses still depend on them; forwarding through lifts must be exact. |
| `SelectedPrepositionUse`, `UnmarkedConjunctLicence`, `BareGenitiveHost`, remaining interface slots | Selected PP ownership, coordination segmentation, genitive distribution and downstream consumers remain. No removal justified by routing alone. |

No policing-feature deletion is currently proposed. A later removal requires a
specific replacement of all its readers and an admission argument, not a name
that sounds structural.

### Proposed whitelist extensions — not implemented

| Candidate | Linguistic reason to consider it | Recommendation / boundary |
|---|---|---|
| Negative colour adjectives, first `nonred` | Restrictive colour exclusion can intersect a relative or PP restriction on the same nominal head, like the positive colour class. | Approve the declared negative colour class, not every `non-` word; preserve each lexical identity and keep adjective-internal focus/coordination opaque. |
| Adjectival `enchanted` | A simple state modifier can intersect the same head's other restrictions, like adjectival tapped. | Approve only the adjectival lexical use after checking its admission summaries; verbal participial structure remains a barrier. |
| Creature-subtype noun premodifiers | A subtype classifier restricts the same head, parallel to the approved card-type classifier. | Approve only the simple noun-premodifier use of the catalog's creature-subtype inventory; retain selected, quoted, coordinated and compound internal structure. Check head-licensing tables unchanged. |

None is needed to obtain the initial colour/card-type improvement. Until
reviewed, each two-order example for `nonred`, `enchanted`, and `Goblin` remains
two normal forms and must remain two Readings after layering. These additions
would change the equivalence, so approval would require a separately versioned
normal-form baseline before a grammar conversion, not a mid-gate relaxation.

### Existing shape-test inventory and re-spelling plan

A source inventory of explicit existing nominal constructor uses in the active
suite found **123 test bodies and 103 helper bodies in 115 files** (226 bodies).
This includes base-noun builders, which also need the new lifts; it is not a
claim that all 226 bodies are tests. The separate `slice` grammar is excluded
and remains unchanged. Parsing-only card tests and shared helper callers must
also run in full, even when they have no literal constructor to inventory.

Apply the following re-spelling to each listed body, keeping its exact witness,
lexical values, selected frames, field comparisons, negative admission cases,
realization and independently-built-value assertions:

- Base builders add the two checked lifts at a complete Nominal boundary.
  Within postmodifier chains, add only the core lift at their base; add the
  outer Nominal lift once, after the last postmodifier.
- Approved adjective/card-type noun chains build premodifiers outside the post
  layer, with unchanged order on each side. Replace the retired order twin's
  expected tree with the canonical tree for the **same text**, and retain its
  normal-form expectation. Do not replace structural equality with a tag test.
- Unapproved adjective/noun uses build the corresponding barrier constructor
  around a complete Nominal. Dedicated barrier constructors and their selected
  complement/relative children retain their old payload, wrapped at the new
  category boundaries. Both scope/attachment witnesses remain independently
  asserted; a single canonical tree is not a substitute for a distinct scope.
- Coordination uses the explicit new result-category value only where its
  nominal binding changes. Preserve ordered conjuncts, helper constructors,
  coordinator and every NounPhrase-hosted alternative.
- Count assertions change only by the demonstrated commuting duplicates.
  The 30 new normal-form judgments keep their old key counts; their raw-tree
  counts are re-spelled individually after layering. The three unapproved
  modifier tests remain 2 trees / 2 normal forms unless separately approved.

The table identifies every inventoried body; the common rules above specify
its mechanical re-spelling. In Phase 3, review each changed assertion against
its old independent value rather than deriving expectations from the parser.
Paths are under `crates/deckmaste_english_v3/tests/`.

| File | Bodies to review / re-spell |
|---|---|
| `suite/additional_slash_adjustment.rs` | `noun_controls_keep_their_nominal_modifier_and_bare_pair` |
| `suite/articles.rs` | `independent_article_values_preserve_onset_variants_and_modifier_attachment`, `homographs_keep_pronunciation_correlated_with_their_article_variant` |
| `suite/as_clause_np_licensing.rs` | `comparison_witness` (helper), `an_independent_until_modifier_is_rejected_by_admission` |
| `suite/as_long_as.rs` | `independently_constructed_conditional_pp_obeys_both_roundtrip_laws` |
| `suite/as_though_content_clauses.rs` | `adjunct_licence_excludes_nominal_predicative_and_preposed_functions` |
| `suite/asyndetic_premodifier_coordination.rs` | `modified` (helper), `witness` (helper), `existing_syndetic_nominal_reading_is_preserved`, `royal_decree_keeps_its_coordinator_ending_nominal_list` |
| `suite/attributive_scalar_comparative.rs` | `implicit_standards_are_adjective_modifiers_of_property_nouns`, `overt_standards_belong_to_one_discontinuous_governor`, `independently_constructed_attributive_property_roundtrips` |
| `suite/bare_zone_noun_complement.rs` | `bare_location` (helper), `wild_magic_sorcerer_retains_both_source_attachments` |
| `suite/base_phrases.rs` | `independent_uncoordinated_phrase_primitives_roundtrip` |
| `suite/become_adjectival_passive.rs` | `independent_complemented_adjective_selects_its_marker_and_object` |
| `suite/bounded_quantity_mass_noun.rs` | `amount` (helper) |
| `suite/by_complement_functions.rs` | `noun` (helper), `avacyn_passive` (helper), `avacyn_by` (helper), `noctis_means_preserves_its_complete_gerund_participial_complement` |
| `suite/card_noun_premodifier.rs` | `witness` (helper), `independently_constructed_card_premodifiers_roundtrip_without_duplicate_routes` |
| `suite/causative_have.rs` | `noun` (helper) |
| `suite/cause_catenative.rs` | `independently_constructed_torpor_orb_constituent_preserves_both_laws` |
| `suite/champion_keyword_line.rs` | `noun` (helper) |
| `suite/clause_final_predicative_as.rs` | `witness` (helper), `independently_constructed_focused_predicative_roundtrips` |
| `suite/comparative_complements.rs` | `an_independent_nominal_comparison_preserves_structure_and_words`, `an_independent_shared_governor_comparison_preserves_structure_and_words` |
| `suite/comparative_quantities.rs` | `counted` (helper) |
| `suite/comparative_quantity_than.rs` | `witnesses_keep_the_determiner_and_selected_comparison_with_the_nominal_head`, `an_independently_constructed_comparative_np_roundtrips` |
| `suite/coordinated_participial_premodifier.rs` | `nominal` (helper), `witness` (helper) |
| `suite/coordinated_property_values.rs` | `independent_ordered_pair_and_premodification_preserve_both_laws` |
| `suite/coordinated_subject_agreement.rs` | `independently_constructed_coordinated_subject_roundtrips_with_singular_coordinates` |
| `suite/copular_scalar_location.rs` | `scalar_location` (helper), `independent_non_scalar_subject_and_wrong_frame_are_rejected` |
| `suite/correlative_adjectives.rs` | `independently_constructed_correlative_adjectives_have_explicit_hosts` |
| `suite/correlative_finite_complements.rs` | `clause` (helper) |
| `suite/correlatives.rs` | `np` (helper) |
| `suite/counter_compounds.rs` | `noun` (helper), `authentic_counter_compounds_preserve_macro_owner_and_noun_inflection` |
| `suite/counter_kind_nouns.rs` | `independently_constructed_compounds_preserve_identity_and_number` |
| `suite/data_batch_2.rs` | `declarations_fill_attested_roles_and_preserve_both_roundtrip_laws` |
| `suite/declaration_batch_b.rs` | `controlled_postmodifier` (helper) |
| `suite/degree_quantifiers.rs` | `quantified` (helper) |
| `suite/delayed_right_constituent_passive.rs` | `independently_constructed_shared_passive_roundtrips` |
| `suite/designation_categories.rs` | `independently_constructed_nouns_and_adjectives_roundtrip_with_their_owners` |
| `suite/devotion_to_colour.rs` | `selected` (helper), `witnesses_select_the_colour_complement_on_devotion`, `selected_noun_phrase_complement_retires_devotion_modifier_attachment` |
| `suite/difference_noun.rs` | `difference` (helper) |
| `suite/divided_as_you_choose.rs` | `witness` (helper), `independently_assembled_participle_preserves_both_roundtrip_laws` |
| `suite/do_so_and_able_to.rs` | `witness` (helper), `bare_adjectives_and_nonagreeing_or_unlicensed_heads_gain_no_route` |
| `suite/enter_as_copy_complement.rs` | `independently_constructed_enter_complement_roundtrips` |
| `suite/enter_as_copy_except_clause.rs` | `exception_scopes` (helper) |
| `suite/except_matrix_complements.rs` | `witness` (helper) |
| `suite/existential_there.rs` | `independent_condition` (helper) |
| `suite/fixed_cost_phrases.rs` | `noun` (helper), `mana_nominal` (helper), `independent_cost_opener_preserves_predication_and_infinitival_selection`, `mutated_witnesses_reject_wrong_number_finiteness_and_of_adjuncts` |
| `suite/focus_only_object.rs` | `independently_constructed_focused_np_roundtrips` |
| `suite/focusing_only.rs` | `selected_and_comparative_as_does_not_become_a_comparative_adjunct`, `independently_constructed_focus_preserves_structure_and_lexical_identity` |
| `suite/from_anywhere.rs` | `comparative_modifier_attaches_to_anywhere_and_retains_nominal_scope` |
| `suite/functions_coordination.rs` | `unlike_predicative_categories_share_a_function_without_erasing_their_categories` |
| `suite/generic_predicates.rs` | `noun` (helper), `selected_with_retains_marker_ownership_and_complement_order`, `urzas_blast` (helper) |
| `suite/gerund_preposition_complements.rs` | `witness` (helper), `without_paying` (helper) |
| `suite/grammar.rs` | `independent_named_nominal_retains_selected_frame_and_catalog_identity`, `independent_participial_premodifier_preserves_verb_form_and_distribution`, `independent_amount_complement_consumes_its_frame_without_losing_countability`, `slash_pairs_compose_with_nominal_modifiers_and_selected_predicates`, `independent_slash_consumers_preserve_count_components_and_leaf_order`, `complemented_adjectives_are_postpositive_in_both_directions`, `integrated_pp_modifiers_stay_inside_the_nominal`, `serial_coordination_composes_with_agreement_case_and_predicate_forms` |
| `suite/half_rounded.rs` | `independently_constructed_fraction_preserves_both_roundtrip_laws` |
| `suite/head_primitives.rs` | `cards` (helper) |
| `suite/instead_replacement.rs` | `replacement_witnesses_retain_their_hosts_and_one_preposition_owner` |
| `suite/irrealis_scalar_difference.rs` | `irrealis_scalar` (helper) |
| `suite/keyword_mention_without_parameter.rs` | `noun` (helper), `keyword_premodifiers_keep_keyword_identity_in_costs_and_abilities`, `keyword_premodification_requires_the_head_permission` |
| `suite/keyword_payloads.rs` | `noun` (helper), `authentic_quality_and_subject_keywords_keep_declared_payloads`, `authentic_amount_cost_quality_cost_and_size_payloads_keep_separators` |
| `suite/keyword_quality_phrases.rs` | `noun` (helper), `bare_choice` (helper), `color_of_choice` (helper), `exact` (helper) |
| `suite/kind_of_counter.rs` | `bare_complement` (helper), `check_taxonomic_roles` (helper), `existing_controls_retain_their_ordinary_np_complements` |
| `suite/last_turn_temporal_adjunct.rs` | `last_turn` (helper) |
| `suite/lexical_compositions.rs` | `combat_damage_keeps_its_noun_premodifier_and_mass_head`, `defending_player_is_a_bare_np_with_a_verbal_premodifier`, `plural_bare_genitives_preserve_possessor_number_and_the_apostrophe` |
| `suite/library_positions.rs` | `independent_position_pp_retains_noun_complement_and_both_roundtrip_laws` |
| `suite/mana_symbol_nominal.rs` | `nominal` (helper), `symbol_nominal_head` (helper), `additional_mana_is_the_head_of_the_selected_object_nominal`, `bare_symbols_keep_their_notation_routes_and_cannot_be_noun_phrases` |
| `suite/measured_damage_nominal.rs` | `amount` (helper) |
| `suite/multiplier_predeterminer.rs` | `independently_constructed_multiplier_preserves_both_roundtrip_laws` |
| `suite/multiword_lexemes.rs` | `independent_multiword_nominals_own_one_lexical_leaf_in_both_roundtrip_directions`, `grammar_consumes_noun_number_and_countability_without_decomposing_the_lexeme`, `reusable_realization_context_checks_independent_values_and_changed_text` |
| `suite/negated_to_infinitival.rs` | `independently_constructed_negated_complement_preserves_both_laws` |
| `suite/negative_common_noun_premodifier.rs` | `independently_constructed_negative_noun_keeps_its_identity_and_plural_head` |
| `suite/next_time_relative_adjunct.rs` | `noun` (helper), `independently_constructed_definite_temporal_adjunct_requires_a_dependent` |
| `suite/nominal_adjuncts.rs` | `noun` (helper), `authentic_boundary_pp_keeps_its_bare_interval_structure`, `independently_constructed_combat_intervals_roundtrip` |
| `suite/noncombat_negative_noun.rs` | `witness` (helper), `independently_constructed_negative_premodifier_roundtrips_with_mass_head` |
| `suite/noun_premodifiers.rs` | `noun` (helper), `modified` (helper), `authentic_coordinated_noun_modifiers_share_the_head_with_oxford_serials`, `permanent_head_use_is_unchanged_and_plural_modifiers_are_rejected` |
| `suite/number_transparency.rs` | `noun` (helper), `of` (helper), `targets` (helper), `predicate` (helper), `eerie_interlude_keeps_both_relative_clause_scopes_and_plural_concord` |
| `suite/participial_uses.rs` | `noun` (helper), `authentic_attacking_and_blocking_modifiers_retain_verbal_identity`, `authentic_tapped_and_untapped_modifiers_retain_adjective_identity`, `righteous_blow_keeps_coordinated_verbal_modifiers_under_the_nominal`, `authentic_complemented_participial_postmodifiers_preserve_their_objects_and_auxiliary`, `hero_of_bladehold_mixed_be_complement_preserves_adjective_and_progressive`, `actual_movement_constituents_keep_selected_destination_before_depictive` (helper), `participial_postmodifiers_and_mixed_complements_reject_wrong_form_or_function` |
| `suite/partitive_quantity_phrases.rs` | `determined` (helper), `independent_quantity_heads_keep_one_route_and_both_roundtrip_laws` |
| `suite/past_participial_premodifiers.rs` | `independently_built_attested_modifiers_preserve_passive_form_and_both_laws` |
| `suite/perfect_relative_adverbial_gap.rs` | `noun` (helper), `independently_constructed_temporal_relative_roundtrips_and_requires_licensed_head` |
| `suite/permanent_noun_premodifier.rs` | `witness` (helper) |
| `suite/phrase_coordination.rs` | `independent_phrase_coordinations_preserve_structure_and_oxford_commas` |
| `suite/polyhedral_die_nouns.rs` | `noun` (helper), `assert_roll_object` (helper) |
| `suite/possessive_phrases.rs` | `possessed` (helper) |
| `suite/postpositive_comparative_property.rs` | `independent_comparative_property_value_preserves_both_laws` |
| `suite/pp_admission.rs` | `nominal_complement_pp_cannot_become_a_free_clause_or_verb_adjunct` |
| `suite/predeterminer_all.rs` | `determined` (helper), `independently_constructed_universal_quantifiers_keep_both_roundtrip_laws`, `number_definiteness_and_stacking_constraints_are_bidirectional` |
| `suite/preposition_function_licences.rs` | `noun` (helper), `hurricane_keyword_pp_remains_a_nominal_modifier`, `chosen_type` (helper) |
| `suite/progressive_object_gap_relative.rs` | `has_relative_modifier` (helper), `ticket_witnesses_link_the_progressive_object_gap_to_the_antecedent`, `subject_gaps_passives_and_ungapped_progressives_keep_their_roles` |
| `suite/pronominal_other.rs` | `other_noun` (helper), `independent_definite_np_roundtrips_and_preserves_the_lexical_values`, `other_before_an_overt_noun_remains_an_adjective` |
| `suite/property_equal_to.rs` | `independently_composed_equality_property_preserves_both_laws` |
| `suite/property_pair_each_equal_to.rs` | `independently_composed_property_predication_preserves_both_roundtrip_laws` |
| `suite/purpose_infinitival.rs` | `spell` (helper) |
| `suite/quality_cost_keyword_lines.rs` | `nominal` (helper) |
| `suite/quantified_energy_symbol.rs` | `energy_head` (helper), `cardinal_witness` (helper), `bare_energy_symbols_keep_notation_without_a_measured_nominal_route` |
| `suite/quantified_mana_symbol.rs` | `symbol` (helper), `witnesses_quantify_the_symbol_inside_the_selected_object`, `generic_and_bare_notation_keep_their_protected_routes` |
| `suite/reading_support.rs` | `seedborn_muse_retains_exactly_two_adjunct_attachments` |
| `suite/relative_clause_adjuncts.rs` | `noun` (helper), `during_untap_step` (helper), `seedborn_muse` (helper) |
| `suite/remove_source_attachment.rs` | `selected_source` (helper), `bare_passive_postmodifier` (helper) |
| `suite/restricted_nominals.rs` | `noun` (helper), `a_status_modifier_keeps_its_existing_determiner_bearing_composition`, `authentic_full_genitive_possessor_preserves_internal_structure` |
| `suite/ring_tempts_you.rs` | `assert_ring_clause` (helper), `designation_is_a_count_noun_under_a_genitive_determiner_and_in_the_plural` |
| `suite/same_as_comparative.rs` | `witnesses_keep_the_complement_inside_the_premodified_nominal`, `an_independent_comparative_nominal_roundtrips_with_its_selected_complement` |
| `suite/scalar_cardinals.rs` | `pinned_maximum_quantities_keep_the_counted_nominal_structure`, `noun` (helper), `targeted` (helper), `independently_constructed_maximum_determiners_preserve_number_and_lexical_owners` |
| `suite/scalar_location_plain_be_and_become.rs` | `become_clause` (helper) |
| `suite/scalar_property_values.rs` | `independently_constructed_property_values_preserve_both_roundtrip_laws` |
| `suite/selected_complement_clusters.rs` | `noun` (helper), `mass_noun` (helper), `any_target` (helper), `another_creature` (helper) |
| `suite/selected_frame_passives.rs` | `noun` (helper), `authentic_retained_object_and_reduced_passives_keep_manner_attachment` |
| `suite/shared_heads.rs` | `cards` (helper) |
| `suite/shared_preposition_heads.rs` | `cards` (helper) |
| `suite/singular_you.rs` | `noun` (helper) |
| `suite/spelled_cardinal_scalar_measure.rs` | `normalize_burn_standard` (helper) |
| `suite/superlative_quantifiers.rs` | `nominal` (helper), `superlative` (helper), `card_witnesses_use_the_superlative_inside_objects_and_with_complements`, `independent_pp_attachment_ambiguity_matches_the_fewest_control`, `witch_king_retains_least_modifying_its_count_scalar_property` |
| `suite/supplementary_temporal_adjunct.rs` | `expected` (helper) |
| `suite/targeting_projection.rs` | `noun` (helper), `targeted` (helper), `flying` (helper), `bare_target_projection_uses_the_declared_number_boundary`, `numerals_and_noun_premodifiers_keep_the_targeted_nominal`, `adjective_and_participial_premodifiers_stay_below_targeting`, `simple_adjective_above_targeting_preserves_the_single_nominal_projection` |
| `suite/then_sequencing.rs` | `card_object` (helper) |
| `suite/transitive_turn_face.rs` | `independently_constructed_active_and_passive_frames_roundtrip` |
| `suite/up_to_that_many.rs` | `independent_bounded_degree_values_preserve_both_roundtrip_laws_and_traversal`, `bounded_degree_preserves_head_selection_and_subject_agreement` |
| `suite/where_variable_clause.rs` | `keyword_supplement` (helper) |
| `suite/your_choice_of_keywords.rs` | `choice` (helper), `declared_nominal_frame_is_required_and_parameterized_keywords_are_compositional` |


### Verification of the gate implementation

- `cargo test -p xtask`: 481 passed, one pre-existing ignored test.
- `cargo test -p deckmaste_english_v3`: 1,165 passed, one pre-existing ignored test.
- `cargo clippy -p xtask -p deckmaste_english_v3 --tests -- -D warnings`: passed.
- Assurance delta: 41 tests added; one existing schema-compatibility test
  re-spelled/extended to cover both old and new additive layouts; zero tests
  removed, weakened, newly ignored or restored.
- Existing ignores unchanged: `macro_schema_census_count_matches_21` (live
  corpus census cross-check, on demand); and
  `cost_notation_is_the_subject_of_the_licensed_locative_small_clause`
  (`english-v3-mana-symbol-verbless-subject`, MS4 governed subject route).
- Citation checks: zero noncompliant strings, zero stale citations; diff audit
  found no changed citation sites.
- Final subset recheck: Chief Engineer's raw digest, surrounding structure,
  unit counts and normal-form sets equal the retained full baseline for that
  subset. This did not cover negative card-type nouns: the earlier claim that
  they did not exist was wrong. The plugin type producer emits them through
  `native::negative_lexeme`, independently of core.ron's two negative noun
  formations. The retained census preceded the final `/non` guard.

### Expected effect and remaining review decisions

The corrected conservative check measures **392,155 normal forms for 482,041
Readings**. There are **3,480 collapsing units on 3,286 faces**. Those units
contain 200,185 Readings and 110,299 normal forms. Thus **89,886 Readings
(18.65%)** are removable if the layered route admits one survivor for every
normal form. Target counts are 75,412 unchanged units, 392,155 unchanged normal
forms and 392,155 Readings; face coverage stays 30,794 read / 2,034 unread.

The five largest units contain 32,256, 17,640, 15,456, 11,520 and 7,216 trees
respectively: Slimefoot's Survey, Nael, Avizoa Aeronaut, Bring to Light,
Whiskervale Forerunner and Osseous Sticktwister. Their conservative normal-form
counts are 10,368, 6,804, 10,200, 6,912 and 7,216. These witnesses belong in the
first subset gate alongside the 30 judgments and independent scope witnesses.

A scratch structural model enumerated all 21,845 application chains of length
0–7 using approved pre/post modifiers and two barrier kinds. Its 14,840 normal
forms each had exactly one survivor under the proposed layer/reset routing.
This establishes the routing property, not commutation of English feature
exports. No scratch grammar change was needed for this structural check, and
no grammar or lexical-data change is part of the Phase 1 commit.

Decisions requested before Phase 3:

1. Approve the local `NominalModifierOrder` property and its default-Barrier
   classification in lexical data/producers. Recommendation: yes; avoid a new
   compiler capability merely to complement a lexical class.
2. Approve explicit lift/barrier constructors and the narrowly enumerated
   old-tree adapter. Recommendation: yes; preserve the compiler's ordinary
   Reading contract rather than inventing invisible productions.
3. Decide the three proposed whitelist extensions individually. Recommendation:
   approve their simple restrictive uses as bounded above, with a new versioned
   baseline **before** any conversion using them. Keeping all three deferred
   still delivers the corrected 89,886-tree opportunity.
4. Keep the policing features listed above for the initial conversion.
   Recommendation: yes; they still enforce admission outside nominal routing.

Main failure risks: a non-commuting summary update despite an intersective
modifier; an omitted local slot at a lift; accidentally treating a compound or
focused adjective as simple; erasing a stored category in an overly broad
adapter; changing nominal coordination into NP coordination; weakening a shape
test to accept fewer scopes. Any changed normal-form set stops the conversion.
The mutually recursive Nominal routes are one conversion family: moving only
object relatives to the restricted post layer before moving barriers and
coordination would lose attachments to those hosts. A preparatory, isomorphic
commit may introduce core-head lifts and split lexical barrier routes while
leaving all modifier recursion on full Nominal; its adapted normal forms and
tree counts must be unchanged. The routing commit then moves the five post
wrappers, all reset barriers and nominal coordination together. Review and
iterate on small subsets inside that commit, but land only the complete,
gate-passing component. Do not mistake a small textual edit for a closed family.


## Corrected conservative baseline and source identity

The owner approved correction of the stale Phase 1 baseline before layering.
The whitelist stays exactly as committed: positive card-type noun modifiers
commute; negative card-type nouns do not. The old census used a prefix-only
matcher that also accepted `/non` derivatives. Its source hash reproduces that
earlier implementation exactly. This was an evidence/versioning error, not a
licensed equivalence change.

Normal-form version 2 records the compiled classification/normalization source
SHA-256 separately from the checkout snapshot. The normal-form gate requires
both reports' version and hash to equal the running check's. A version-to-source
lock test requires a new version for source edits. Incremental reuse rejects
mismatched identities too, so copied sets cannot be relabelled as current.
The Perilous Predicament regression writes a census baseline, reads its one
unit back, and compares its two recorded normal forms with those computed
independently from its Readings. Missing hashes, stale versions and equal but
stale hashes on both reports are rejection tests.

The difference between the stale prefix matcher and the conservative matcher
identifies simple negative card-type noun premodifier uses in 1,024 units on
981 faces, containing 22,212 Readings. This measures uses relevant to the
modifier-order check, not every lexical occurrence of a negative card type. Of these, 390 units on 388 faces (16,261 Readings)
have fewer normal forms under the erroneous prefix matcher: 8,402 versus
15,721 conservatively, a difference of 7,319. Across all affected units the
counts are 14,107 versus 21,426. They remain barriers. They are not included in
the approved follow-up (negative colours, bare adjectival enchanted/equipped,
and simple uncoordinated, unquoted creature-subtype noun modifiers).

The approved layers are NominalCore, NominalPostmodification and Nominal with
two unary lifts. NominalModifierOrder defaults to Barrier; the tool whitelist
remains independent and agreement will be tested. Keep all policing features.
The layer conversion and approved follow-up have not yet been applied.

Correction verification (`putoonqn`): full unchanged-grammar census has 32,828
faces, 30,794 read, 482,041 Readings, 75,412 units, 392,155 normal forms,
247,973,895 chart items, zero incomplete enumerations and zero issues. Wall
time is 560 s 579 ms with four workers (old baseline 657 s 651 ms; host load differs).
`identical-digests` against the previous census passes on every face; the
version-2 normal-form self-check passes. The old baseline is rejected by the
normal-form gate for its version, then deleted. The retained baseline is
`target/english-v3/nominal-baseline-v2.json`. Its Perilous Predicament unit has
two normal forms, equal to the independent subset census.

Full xtask tests: 484 passed, one existing ignore; clippy with tests and denied
warnings passed. The extended incremental-loader test also passed after its
additional stale/missing-hash cases were added. Assurance: three new tests,
two existing tests extended; none deleted, weakened, newly ignored or restored.
Citation checks: zero noncompliant strings, zero stale citations, no changed
citation sites.

## Conservative lexical classification preparation

`NominalModifierOrder { Barrier, Commuting } default Barrier` is now declared
as a lexical property. Positive colours and nonlegendary are assigned after
negative adjective formation; adjectival tapped is split from the shared
participial-adjective class. The existing card-type producer assigns Commuting
to the positive noun after emitting its negative derivative. No lexeme id,
form, variant or frame changes. No construction consumes the property yet;
layer routing and Reading shapes are unchanged.

The whole loaded-lexicon agreement test calls the independent normalizer's
existing simple-use matcher; the matcher source and version stay unchanged.
The colour provenance test retains its five existing colour witnesses, source,
form and paradigm comparisons. Before: `negative.properties == base.properties`.
After: compare the entire property records except NominalModifierOrder, and
separately assert positive Commuting and negative default Barrier. Classes,
countability, every other feature and all frames still compare exactly.
This is one re-spelled test, not a weakened or deleted assertion.

Remaining: add the reviewed layer adapter under a new version and take its
unchanged-grammar baseline; convert the mutually recursive nominal routes and
re-spell their shape tests; gate and assess chart/time; only then extend the
commuting list with the separately approved additions and its own baseline.
No policing feature has been removed. The ticket remains in progress.


Classification census (`lynmlnrq`): 32,828 faces, 30,794 read, 482,041 Readings,
75,412 units and 392,155 normal forms. The raw identical-digests gate passes
against version 2. Chart items are unchanged at 247,973,895. Wall time is
603 s 443 ms versus 560 s 579 ms (+7.6%), both four workers; the classification run
also overlapped full tests. This is not a measurement of the still-unimplemented
lifts. The approved follow-up has not begun.

| Largest unit | Baseline trees | Preparation trees | Normal forms (both) |
|---|---:|---:|---:|
| Slimefoot's Survey, unit 0 | 32,256 | 32,256 | 10,368 |
| Nael, Avizoa Aeronaut, unit 1 | 17,640 | 17,640 | 6,804 |
| Bring to Light, unit 0 | 15,456 | 15,456 | 10,200 |
| Whiskervale Forerunner, unit 0 | 11,520 | 11,520 | 6,912 |
| Osseous Sticktwister, unit 1 | 7,216 | 7,216 | 7,216 |

Both full-corpus gates pass on all 32,828 faces: identical raw digests and
identical per-unit normal-form sets, with unchanged source, units and
surrounding structure. The step census was deleted after its gates; only the
corrected baseline is retained. Full tests: English 1,165 passed / one existing
ignore; lexical source 100 passed; xtask 485 passed / one existing ignore.
Clippy with tests and denied warnings passes for all three crates, including a
final xtask check after moving the regression test's temporary census under
`target/english-v3/`. The test also passed again at that location.

Assurance for this preparation: one added inventory-agreement test; one
re-spelled colour-provenance test; none removed, weakened, restored or newly
ignored. Across the correction and preparation: four added tests, two existing
incremental tests extended, one re-spelled. README and glossary describe the
new property and commuting/barrier distinction. Citation checks remain clean.
This is a complete preparatory prefix; no layer routing, lift or shape-test
conversion has been applied, and no follow-up modifier has been added.

Census delta (`lynmlnrq`, preparatory prefix): faces read 30,794 → 30,794; Readings 482,041 → 482,041; lost faces none.
Judgment set: does not apply yet.

## Layer adapter and version-3 baseline

Version 3 adds only the reviewed inverse representation mapping: strip the two
exact unary lifts; restore the two barrier constructor names; restore the old
Nominal category on Coordination, SerialCoordination,
DiscontinuousComparativeNominal and PostpositiveNominal only. Changed forms or
fields of these new shapes fail closed. The independent conservative lexical
inventory is unchanged. The source lock records
`2b48ff9165ddecd677360be67aefe948248701b03b6db81a1bf7dfb7c8ba6273`.

The unchanged-grammar baseline has 32,828 faces, 30,794 read, 482,041 trees,
75,412 units, 392,155 normal forms and 247,973,895 chart items. The isolated
four-worker census took 571 s 477 ms (host load 4.219 / 6.345 / 6.287), with
no concurrent test or build command. The full identical-digests check against
version 2 passes. A separate streaming comparison checks exact equality of
all 75,412 per-unit normal-form sets across the version change; the normal-form
gate deliberately refuses cross-version comparisons. Version 2 is then deleted;
`target/english-v3/nominal-baseline-v3.json` is the conversion baseline.

Assurance: two tests added (strict inverse mappings, nested lifts in a commuting
chain), the version-lock test extended; no tests removed, weakened or newly
ignored. Full xtask: 487 passed, one existing ignore. Clippy with tests and
warnings denied passed; all 43 normal-form tests passed after the style fixes.
