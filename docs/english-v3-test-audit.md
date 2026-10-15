# English v3 test obligation audit

The record of `english-v4-corpus-stratification-and-judgment-harness`'s audit of
`crates/deckmaste_english_v3/tests/suite/`, for `english-v4-cutover-v3`
([`docs/decisions/english-v4.md`](decisions/english-v4.md) §7). Every v3 test is
listed with what became of it: its genuine grammatical obligations migrated to
the v4 judgment set (`crates/deckmaste_english_v4/judgments/<file>.txt`, same
stem as the v3 file), its remainder retired with a reason.

1163 tests: 372 migrated, 638 partly migrated, 120 retired, and 33 parser-engine tests.

## Retirement reasons

| Code | Retired because the test |
|---|---|
| `shape` | pins a v3 Reading constructor, form index or tree shape |
| `policing` | asserts a policing feature, or rejects a grammatical sentence to suppress a benign Reading |
| `count` | pins a Reading count that is not the claim |
| `lexical-v3` | tests a fact of the v3 lexicon or plugin sources; see "Lexical facts" below |
| `wrong` | asserts an analysis the v4 rulings reject |
| `helper` | tests a v3 test helper |
| `engine` | tests the v3 parsing engine, not English; see "Parser-engine tests" below |

A `partly-migrated` test lists the number of judgment lines it yielded and the
codes of its retired remainder. Migrated lines were adjudicated by
independent judges (Claude Sonnet, then GPT-6-Astra as the second judge; an
earlier Claude Haiku pass was advisory only); a line they rejected on the
authorities' grounds was corrected or dropped, so a file can hold fewer lines
than its tests yielded. Disagreements the authorities do not settle went to the
owner.

The grammar parses Oracle text; it does not validate templating
(`crates/deckmaste_english_v4/judgments/README.md`, "What a negative entry may
claim"). A migrated negative was kept only where its rejection falls out of
distinctions the grammar needs anyway (agreement, selected frames,
phrase-internal word order, lexical scope); 219 that would have needed a rule
of their own (notation, capitalization, punctuation, hyphenation, word choice,
correlative pairing) were dropped after migration, and 269 remain.

## Sentence tests

### `additional_slash_adjustment.rs` → `judgments/additional_slash_adjustment.txt` (12 lines)

| Test | Disposition | Detail |
|---|---|---|
| `witnesses_keep_the_modifier_inside_the_selected_adjustment_complement` | migrated | 5 |
| `independent_additive_class_members_obey_both_roundtrip_laws` | partly-migrated | 2; shape |
| `articles_and_modifiers_are_paired_and_agree_in_onset` | partly-migrated | 2; shape |
| `noun_controls_keep_their_nominal_modifier_and_bare_pair` | partly-migrated | 3; shape |

### `all_but_exception.rs` → `judgments/all_but_exception.txt` (23 lines)

| Test | Disposition | Detail |
|---|---|---|
| `independent_exception_phrases_keep_the_head_pp_and_both_roundtrip_laws` | partly-migrated | 4; shape (round-trip/Reading value pin) |
| `prenominal_exceptions_are_whole_determiners` | migrated | 3 |
| `partitives_keep_the_whole_exception_as_their_fused_head` | migrated | 2 |
| `witness_sentences_preserve_their_controls_independent_ambiguity` | partly-migrated | 5; count (ambiguity equals control count) |
| `declared_licences_exclude_coordinator_and_floated_quantifier_twins` | partly-migrated | 4; policing (all but the bottom card, all but one life, but 1 of ..), shape (admit of wrong-head Reading) |
| `ordinary_determination_partitives_and_prevention_controls_keep_reading_values` | migrated | 5 |

### `amount_complements.rs` → `judgments/amount_complements.txt` (13 lines)

| Test | Disposition | Detail |
|---|---|---|
| `independently_constructed_amount_and_imperative_roundtrip` | retired | shape: builds Reading constructors, visits nodes, round-trip |
| `oracle_numeric_complements_and_agreement` | partly-migrated | 13; shape (Amount fragments 0/1,000/X/three), policing (Scry 1000. size cap) |

### `analytic_not_after_be.rs` → `judgments/analytic_not_after_be.txt` (17 lines)

| Test | Disposition | Detail |
|---|---|---|
| `witnesses_keep_negation_on_the_auxiliary_and_the_ordinary_complement` | partly-migrated | 5; shape |
| `analytic_and_synthetic_pairs_keep_equal_readings` | partly-migrated | 11; count |
| `licences_and_positive_auxiliary_status_control_admission` | partly-migrated | 1 (It is not your turn.); policing,lexical-v3 |
| `independent_negated_predicate_exports_synthetic_polarity_and_roundtrips` | retired | engine: independent Reading construction, round-trip and admission summary |

### `anywhere_else_goal.rs` → `judgments/anywhere_else_goal.txt` (11 lines)

| Test | Disposition | Detail |
|---|---|---|
| `independent_posthead_values_obey_both_roundtrip_laws` | partly-migrated | 2; rest (constructed Reading, visit_words) shape |
| `selected_put_has_object_then_locative_np` | partly-migrated | 2; FrameValue complement vectors shape |
| `ticket_witnesses_select_the_posthead_np_as_locative_complement` | migrated | 4 |
| `optional_dependent_and_locative_use_require_declared_licenses` | partly-migrated | 3; constructed-Reading admit errors shape |

### `arithmetic_nominal_operand.rs` → `judgments/arithmetic_nominal_operand.txt` (13 lines)

| Test | Disposition | Detail |
|---|---|---|
| `kindle_keeps_the_card_count_as_a_right_operand` | migrated | 1 |
| `ivory_tower_keeps_the_card_count_as_a_left_operand` | migrated | 1 |
| `allosaurus_rider_keeps_the_land_count` | migrated | 1 |
| `haunting_apparition_keeps_the_modified_card_count` | migrated | 1 |
| `food_chain_keeps_the_exiled_creatures_mana_value` | migrated | 1 |
| `war_report_keeps_two_scalar_nominal_operands` | partly-migrated | 1; reading count 4 dropped (count) |
| `torbrans_quantified_damage_is_a_scalar_nominal_operand` | partly-migrated | 1; readings 1 and MeasurePhrase/ArithmeticOperand categories dropped (count) |
| `unlicensed_operands_and_preposition_functions_stay_excluded` | retired | policing: operand-licence negatives (a creature plus 1 etc.) |
| `independently_composed_nominal_arithmetic_roundtrips_and_checks_the_licence` | retired | shape: hand-built Readings, admit/realize |
| `independently_composed_chains_retain_both_bracketings` | partly-migrated | 1 (readings 2 ambiguity witness); rest shape |
| `r10e_only_gains_keep_a_scalar_copular_complement` | partly-migrated | 5; count 1 dropped (count) |

### `articles.rs` → `judgments/articles.txt` (35 lines)

| Test | Disposition | Detail |
|---|---|---|
| `independent_article_values_preserve_onset_variants_and_modifier_attachment` | partly-migrated | 2; lexical-v3 shape |
| `articles_reject_wrong_onset_plural_and_mass_uses` | partly-migrated | 8; lexical-v3 |
| `homographs_keep_pronunciation_correlated_with_their_article_variant` | retired | lexical-v3: fixture homograph xenic/an |
| `articles_and_case_reach_real_grammatical_hosts` | partly-migrated | 25; shape |
| `independently_supplied_names_preserve_exact_identity_and_case` | retired | lexical-v3: fixture catalog names with exact case |

### `as_clause_np_licensing.rs` → `judgments/as_clause_np_licensing.txt` (8 lines)

| Test | Disposition | Detail |
|---|---|---|
| `comparison_and_duration_survive_without_the_finite_as_alternatives` | migrated | 4 |
| `the_negative_probe_has_no_finite_as_clause` | migrated | 1 |
| `finite_as_complements_keep_their_attested_readings` | partly-migrated | 3; count (9/1/1 reading counts, except readings 1 on the fragment) |
| `an_independent_until_modifier_is_rejected_by_admission` | retired | shape: Reading construction/admit; the !Modifier claim is carried by the Deputy entries |

### `as_long_as.rs` → `judgments/as_long_as.txt` (14 lines)

| Test | Disposition | Detail |
|---|---|---|
| `conditional_witnesses_retain_the_if_twins_readings_and_structure` | partly-migrated | 4 (a 5th, Aeronaut, is merged into the aeronaut_tinkerer line); shape (if-twin structure equality), count |
| `aeronaut_tinkerer_has_exactly_the_vp_conditional_reading` | migrated | 1 |
| `master_thief_has_exactly_the_vp_duration_reading` | migrated | 2 |
| `independently_constructed_conditional_pp_obeys_both_roundtrip_laws` | partly-migrated | 2; shape |
| `conditional_head_excludes_that_and_nonfinite_complements` | migrated | 3 |
| `duration_comparison_requires_its_expanded_complement_marker` | migrated | 2 |

### `as_though_content_clauses.rs` → `judgments/as_though_content_clauses.txt` (9 lines)

| Test | Disposition | Detail |
|---|---|---|
| `vedalken_orrery` | migrated | 1 |
| `high_fae_trickster` | migrated | merged into vedalken_orrery line; keyword lines (newline text) not expressible, sentence only |
| `blur_of_heroism` | migrated | 1 |
| `mirror_wall` | migrated | 1 |
| `ruxa_patient_professor` | migrated | 1 |
| `quicken_modal_passive` | migrated | 1 (first sentence only; newline text not expressible) |
| `independently_constructed_irrealis_preserves_value_and_leaf_order` | partly-migrated | 1; engine,shape |
| `named_irrealis_word_controls_admission_and_parsing` | retired | lexical-v3: as though is a declared preposition lexeme (identity-gated); unnamed copy rejected |
| `adjunct_licence_excludes_nominal_predicative_and_preposed_functions` | partly-migrated | 1; engine |
| `irrealis_is_non_tensed_and_does_not_relax_ordinary_agreement` | partly-migrated | 2; lexical-v3 |

### `asyndetic_premodifier_coordination.rs` → `judgments/asyndetic_premodifier_coordination.txt` (9 lines)

| Test | Disposition | Detail |
|---|---|---|
| `terror` | migrated | 1 |
| `duress` | migrated | 1 |
| `druidic_satchel` | migrated | 1 |
| `victim_of_night` | migrated | 1 |
| `filter_out` | migrated | 1 |
| `independently_constructed_mixed_members_share_one_head` | migrated | 1 |
| `independently_constructed_three_members_have_one_grouping` | migrated | 1 |
| `existing_syndetic_nominal_reading_is_preserved` | partly-migrated | 1; exact single-reading/tree shape retired (shape) |
| `royal_decree_keeps_its_coordinator_ending_nominal_list` | migrated | 1 |

### `at_least_cardinal.rs` → `judgments/at_least_cardinal.txt` (11 lines)

| Test | Disposition | Detail |
|---|---|---|
| `sokka_keeps_the_bounded_cardinal_inside_its_coordinated_subject` | migrated | 1 |
| `svyelun_keeps_the_bounded_cardinal_in_determiner_function` | migrated | 1 |
| `renegade_reaper_keeps_singular_agreement_under_the_bound` | partly-migrated | 1; reading count 2 dropped (count) |
| `synchronized_eviction_keeps_the_relative_clause_on_the_nominal` | partly-migrated | 1; reading count 2 dropped (count) |
| `glimmer_lens_keeps_the_bounded_cardinal_inside_its_coordinated_subject` | migrated | 1 |
| `bound_agreement_follows_the_cardinal` | migrated | 4 |
| `bounded_cardinals_compose_with_both_measure_consumers` | migrated | 2 |
| `independently_constructed_bounds_preserve_both_roundtrip_laws_and_identity` | retired | shape: hand-built Readings, round-trip identity |
| `the_bound_is_neither_a_free_adjunct_nor_an_np_complement` | partly-migrated | 2 (!PrepositionPhrase); at most / bare at least negatives retired (policing) |

### `at_random_before_source.rs` → `judgments/at_random_before_source.txt` (12 lines)

| Test | Disposition | Detail |
|---|---|---|
| `surreal_memoir` | migrated | 1 |
| `make_a_wish` | migrated | 1 |
| `ghoulraiser` | migrated | 1 |
| `haunted_fengraf` | migrated | 1 |
| `moldgraf_monstrosity` | migrated | 1 |
| `existing_adjuncts_keep_their_routes_and_manner_is_not_an_np_modifier` | partly-migrated | 6; policing |
| `independently_assembled_frame_preserves_both_roundtrip_laws_and_the_licence` | partly-migrated | 1; shape policing |

### `attributive_scalar_comparative.rs` → `judgments/attributive_scalar_comparative.txt` (19 lines)

| Test | Disposition | Detail |
|---|---|---|
| `implicit_standards_are_adjective_modifiers_of_property_nouns` | partly-migrated | 5; shape (Reading constructors) |
| `overt_standards_belong_to_one_discontinuous_governor` | partly-migrated | 3; shape (discontinuous governor categories/forms) |
| `authentic_witnesses_roundtrip_and_controls_keep_their_complement_roles` | partly-migrated | 11; shape (MeasuredAttribute/ScalarMeasureValue constructors) |
| `independently_constructed_attributive_property_roundtrips` | retired | shape: builds a v3 Reading by hand; nominal modifier claim covered by the NounPhrase entries |
| `complement_free_comparatives_do_not_enter_predicative_function` | retired | policing: "Its power is greater." is grammatical English; v3 suppressed predicative use of complement-free comparatives |

### `bare_zone_noun_complement.rs` → `judgments/bare_zone_noun_complement.txt` (23 lines)

| Test | Disposition | Detail |
|---|---|---|
| `sea_gate_wreckage_has_a_bare_np_without_an_elided_possessor` | migrated | 1 |
| `norin_plays_the_card_from_a_bare_location` | migrated | 1 |
| `wild_magic_sorcerer_retains_both_source_attachments` | partly-migrated | 2; count (8 readings) |
| `umbris_uses_the_noun_exile_in_the_scoped_card_text` | migrated | 1 |
| `ennis_puts_cards_into_a_bare_location` | migrated | 1 |
| `independently_constructed_bare_location_pps_obey_both_roundtrip_laws` | partly-migrated | 5; shape |
| `bare_location_permission_does_not_escape_to_other_prepositions_or_cases` | partly-migrated | 7; policing (on/of/to exile, put on exile, bare NP exile/hand, admit-error check) |
| `determined_and_spatial_edge_complements_keep_their_existing_routes` | partly-migrated | 5; count, shape |

### `base_phrases.rs` → `judgments/base_phrases.txt` (16 lines)

| Test | Disposition | Detail |
|---|---|---|
| `independent_uncoordinated_phrase_primitives_roundtrip` | partly-migrated | 7; shape,engine (Cardinal/Amount/MeasurePhrase 'three'/'3' have no CGEL category, dropped) |
| `selected_infinitive_is_a_whole_bare_predicate_and_not_a_sentence` | partly-migrated | 5; engine |
| `oracle_adverb_constituents_keep_declared_positions` | migrated | 4 |

### `become_adjectival_passive.rs` → `judgments/become_adjectival_passive.txt` (11 lines)

| Test | Disposition | Detail |
|---|---|---|
| `flint_golem_becomes_blocked_has_an_adjectival_predicative_complement` | migrated | 1 |
| `talruum_champion_keeps_the_selected_by_complement_inside_the_adjective` | partly-migrated | 1; count 3 retired (count) |
| `vedalken_ghoul_trained_cheetah_and_somberwald_alpha_preserve_the_adjective` | migrated | 3 |
| `independent_simple_adjectives_preserve_identity_and_have_one_route` | partly-migrated | 3; constructed Readings/word identity shape |
| `independent_complemented_adjective_selects_its_marker_and_object` | partly-migrated | 2; wrong_frame/marker/head admit errors shape |
| `adjectival_complements_cannot_take_objects_or_verbal_inflections` | partly-migrated | 1; becomes blocks/blocking retired (policing) |

### `bounded_quantity_mass_noun.rs` → `judgments/bounded_quantity_mass_noun.txt` (16 lines)

| Test | Disposition | Detail |
|---|---|---|
| `abandoned_campground_measures_life_with_an_upper_comparative_bound` | migrated | 1 |
| `divinity_of_pride_measures_life_in_a_conditional_complement` | migrated | 1 |
| `innocent_bystander_measures_the_passive_damage_complement` | migrated | 1 |
| `callous_giant_measures_the_active_damage_object` | migrated | 1 |
| `dragonborn_champion_measures_damage_before_a_recipient_complement` | migrated | 1 |
| `independent_bounded_measurements_keep_one_structure_and_both_roundtrip_laws` | partly-migrated | 3; round-trip/hand-built Reading parts retired (shape) |
| `zubera_damage_subject_keeps_singular_agreement` | migrated | 2 |
| `count_heads_plain_measurements_and_postpositive_controls_keep_their_roles` | partly-migrated | 6; ComparativeQuantity category dropped (shape) |

### `by_complement_functions.rs` → `judgments/by_complement_functions.txt` (5 lines)

| Test | Disposition | Detail |
|---|---|---|
| `avacyn_internalised_complement_belongs_to_the_bare_passive` | migrated | 1 |
| `spikeshell_active_extent_is_selected_by_its_lexical_frame` | migrated | 2 |
| `trusted_advisor_passive_extent_remains_a_selected_complement` | migrated | 1 |
| `noctis_means_preserves_its_complete_gerund_participial_complement` | migrated | 1 |

### `card_noun_premodifier.rs` → `judgments/card_noun_premodifier.txt` (12 lines)

| Test | Disposition | Detail |
|---|---|---|
| `crop_sigil` | migrated | 1 |
| `peer_past_the_veil` | migrated | 1 |
| `deluge_of_doom` | migrated | 1 |
| `memoricide` | partly-migrated | 1; count (11/1 reading counts) |
| `curse_of_silence` | migrated | 1 |
| `independently_constructed_card_premodifiers_roundtrip_without_duplicate_routes` | partly-migrated | 4; shape (hand-built Reading) |
| `open_the_gates_preserves_coordination_without_card_as_a_coordinated_premodifier` | migrated | 1 |
| `attested_card_attributes_are_available_in_choose_instructions` | migrated | 2 |

### `causative_have.rs` → `judgments/causative_have.txt` (9 lines)

| Test | Disposition | Detail |
|---|---|---|
| `quill_slinger_boggart_selects_object_and_bare_infinitival_under_may` | migrated | 1 |
| `rage_forger_keeps_the_damage_predicate_inside_the_infinitival` | migrated | 1 |
| `joraga_bard_selects_plural_object_and_infinitival` | migrated | 1 |
| `extractor_demon_selects_the_player_and_mill_infinitival` | migrated | 1 |
| `ebon_dragon_selects_the_opponent_as_object` | migrated | 1 |
| `wandering_troubadour_finite_had_selects_object_and_infinitival` | migrated | 1 |
| `lava_blister_finite_has_selects_the_named_damage_source_as_object` | migrated | 1 |
| `independent_ebon_dragon_constituent_preserves_both_roundtrip_laws` | partly-migrated | 1; shape |
| `causative_complement_requires_plain_form_without_to` | partly-migrated | 1; policing (to-infinitive negative, no-new-reading baseline diff) |

### `cause_catenative.rs` → `judgments/cause_catenative.txt` (13 lines)

| Test | Disposition | Detail |
|---|---|---|
| `witnesses_select_the_object_and_to_infinitival_in_every_reading` | partly-migrated | 5; shape |
| `independent_nominal_ambiguities_are_preserved_under_w9_a` | migrated | merged into witnesses line (relative clause ambiguity groups on 3 of the texts) |
| `the_frame_licence_preserves_controls_and_requires_the_infinitival` | partly-migrated | 7; policing (cause abilities rejected), lexical-v3 (frame removal) |
| `independently_constructed_torpor_orb_constituent_preserves_both_laws` | partly-migrated | 1; engine,shape |

### `central_adverb_position.rs` → `judgments/central_adverb_position.txt` (8 lines)

| Test | Disposition | Detail |
|---|---|---|
| `vastwood_animist` | migrated | 1 |
| `thelonite_druid` | migrated | 1 |
| `verdant_touch` | migrated | 1 |
| `aven_warcraft` | migrated | 1 |
| `might_of_the_meek` | migrated | 1 |
| `independently_assembled_central_adjunct_roundtrips` | migrated | 1 |
| `central_licence_does_not_license_other_positions_or_adverbs` | retired | policing: CentralAdjunctLicence negatives reject grammatical positions |
| `parnesse_auxiliary_keeps_the_adjunct_outside_the_object` | migrated | 1 |
| `dark_dabbling_imperative_keeps_the_central_adjunct_outside_its_object` | migrated | 1 |

### `champion_keyword_line.rs` → `judgments/champion_keyword_line.txt` (13 lines)

| Test | Disposition | Detail |
|---|---|---|
| `champion_article_keeps_the_subject_role_and_both_roundtrip_laws` | partly-migrated | 7; SubjectKeyword Reading, keyword/subtype lexeme ids, hand-built round-trip retired (shape/lexical-v3) |
| `champion_coordinated_descriptions_remain_the_subject_payload` | partly-migrated | 2; SubjectKeyword Reading equality retired (shape) |
| `enchant_retains_its_bare_subject_and_champion_requires_the_licensed_article` | partly-migrated | 4; agreement negatives kept; "Champion Elemental", "Champion the creature", "Enchant a land" negatives retired (policing) |

### `clause_final_predicative_as.rs` → `judgments/clause_final_predicative_as.txt` (9 lines)

| Test | Disposition | Detail |
|---|---|---|
| `experimental_synthesizer` | partly-migrated | 1; shape |
| `lions_eye_diamond` | migrated | 1 |
| `sun_blessed_guardian` | migrated | 1 |
| `intrepid_tenderfoot` | migrated | 1 |
| `cat_burglar` | migrated | 1 |
| `independently_constructed_focused_predicative_roundtrips` | partly-migrated | 1; shape |
| `selected_and_comparative_controls_gain_no_as_adjunct` | migrated | 3 |

### `clitic_contractions.rs` → `judgments/clitic_contractions.txt` (23 lines)

| Test | Disposition | Detail |
|---|---|---|
| `risen_reef_clitic_keeps_the_nominal_predicative_complement` | migrated | 2 |
| `shackle_slinger_clitic_keeps_the_adjectival_predicative_complement` | migrated | 2 |
| `mirrorpool_clitic_keeps_the_relative_clause_subject_gap` | migrated | 2 |
| `bounty_agent_clitic_keeps_the_relative_clause_complement_coordination` | migrated | 2 |
| `goblin_cohort_clitic_keeps_the_perfect_complement` | migrated | 2 |
| `panglacial_wurm_clitic_keeps_the_progressive_complement` | migrated | 2 |
| `myth_unbound_clitic_has_keeps_the_perfect_passive_complement` | migrated | 2 |
| `independently_constructed_clitic_copula_and_passive_roundtrip` | partly-migrated | 2; shape (hand-built Readings, obsolete form index) |
| `clitic_auxiliaries_require_overt_complements_and_licensed_hosts` | partly-migrated | 7; policing (you and they're tapped: grammatical spoken English) |
| `clitic_be_retains_all_full_be_frames_and_have_retains_the_perfect_use` | retired | lexical-v3: contracted be/have lexemes carry the full be frames / perfect auxiliary frames; have-clitic has no main-verb use |

### `clitic_have_object_relative.rs` → `judgments/clitic_have_object_relative.txt` (13 lines)

| Test | Disposition | Detail |
|---|---|---|
| `ticket_witnesses_have_a_perfect_predicate_with_an_object_gap` | migrated | 5 |
| `strong_have_retains_lexical_and_perfect_relative_analyses` | migrated | 1 |
| `hollow_one_shares_one_auxiliary_over_coordinated_participle_gaps` | partly-migrated | 1; policing (finite perfect+preterite coordination reading is tolerated not endorsed) |
| `independently_built_perfect_gap_obeys_both_laws_and_declared_licences` | partly-migrated | 2; shape (constructed Readings, admit errors) |
| `gap_free_perfects_and_temporal_relatives_keep_their_roles` | migrated | 4 |

### `combat_intervals.rs` → `judgments/combat_intervals.txt` (8 lines)

| Test | Disposition | Detail |
|---|---|---|
| `dire_fleet_warmonger_keeps_the_trigger_attachment` | partly-migrated | 1; shape (NounPremodifier != combat check) |
| `might_of_the_ancestors_keeps_the_trigger_attachment` | partly-migrated | 1; shape |
| `glyph_of_destruction_keeps_the_bare_boundary` | partly-migrated | 1; shape |
| `phantom_whelp_keeps_the_bare_boundary` | partly-migrated | 1; shape |
| `echo_circlet_keeps_the_temporal_adjunct` | partly-migrated | 1; shape |
| `combat_damage_keeps_its_premodifier_analysis` | partly-migrated | 2; policing (NounPhrase combat rejected) |
| `bare_during_combat_is_an_adjunct_with_an_interval_complement` | migrated | 1 |

### `comparative_complements.rs` → `judgments/comparative_complements.txt` (18 lines)

| Test | Disposition | Detail |
|---|---|---|
| `equality_complements_belong_to_the_adjective_inside_the_object` | migrated | 3 |
| `equality_is_predicative_and_survives_a_postposed_recipient_order` | migrated | 2 |
| `an_independent_nominal_comparison_preserves_structure_and_words` | migrated | 0 new (covered by equality line); constructed Reading/words are shape |
| `an_independent_shared_governor_comparison_preserves_structure_and_words` | migrated | 1; constructed Reading/words are shape |
| `scalar_inequality_and_shared_governors_keep_their_complements` | migrated | 6 |
| `non_scalar_inequality_is_postpositive_without_losing_bare_other` | partly-migrated | 6; `than its power` as PP retired (policing: than-PP is grammatical) |

### `comparative_quantities.rs` → `judgments/comparative_quantities.txt` (30 lines)

| Test | Disposition | Detail |
|---|---|---|
| `comparative_quantities_compose_with_the_existing_counted_nominal` | migrated | 16 |
| `scalar_hosts_can_reuse_the_single_numeral_coordination` | partly-migrated | 1; reading count 1 dropped (count) |
| `independent_comparative_quantities_preserve_structure_and_lexical_identity` | retired | shape: hand-built Readings, laws, node/word identity |
| `comparative_determiners_preserve_finite_agreement` | partly-migrated | 3; hand-built Reading equality retired (shape) |
| `comparative_quantity_selection_rejects_invalid_independent_values` | retired | shape: admission of hand-built invalid Readings |
| `comparative_quantities_do_not_overlap_other_quantity_or_adjunct_routes` | partly-migrated | 11; category-exclusivity (Cardinal/than-one PP) and "three and more" retired (policing/count) |

### `comparative_quantity_than.rs` → `judgments/comparative_quantity_than.txt` (16 lines)

| Test | Disposition | Detail |
|---|---|---|
| `witnesses_keep_the_determiner_and_selected_comparison_with_the_nominal_head` | partly-migrated | 6; shape |
| `head_selection_case_and_outer_determiner_follow_the_comparative_licence` | partly-migrated | 6; policing |
| `an_independently_constructed_comparative_np_roundtrips` | partly-migrated | 1; shape |
| `existing_comparisons_keep_their_roles` | partly-migrated | 3; shape |

### `coordinated_focused_adjuncts.rs` → `judgments/coordinated_focused_adjuncts.txt` (11 lines)

| Test | Disposition | Detail |
|---|---|---|
| `thunderhead_gunner_coordinates_pp_and_frequency` | migrated | 1 |
| `instill_energy_coordinates_temporal_pp_and_frequency` | migrated | 1 |
| `temple_of_cyclical_time_coordinates_conditional_and_predicative_pps` | migrated | 1 |
| `security_detail_coordinates_conditional_pp_and_frequency` | migrated | 1 |
| `hakim_keeps_the_member_focus_route` | migrated | 1 |
| `standalone_controls_keep_their_readings` | migrated | 6 |
| `clause_plus_focused_phrase_requires_its_own_route` | retired | policing: "Any player may activate this ability but only as a sorcery." is grammatical Oracle English |
| `independently_assembled_thunderhead_adjunct_roundtrips` | retired | shape: hand-built FocusedAdjunctCoordination Reading |
| `focus_cannot_enclose_a_coordination_with_a_focused_member` | retired | shape: admission of a hand-built FocusedPhrase Reading |

### `coordinated_participial_premodifier.rs` → `judgments/coordinated_participial_premodifier.txt` (8 lines)

| Test | Disposition | Detail |
|---|---|---|
| `independently_constructed_pair_preserves_categories_and_one_modifier` | migrated | 2 |
| `independently_constructed_serial_list_has_one_shared_head` | migrated | 1 |
| `furnace_blessed_conqueror` | partly-migrated | 1; policing (tapped is not a verbal premodifier) |
| `pugnacious_pugilist` | partly-migrated | 1; policing (tapped is not a verbal premodifier) |
| `krothuss` | partly-migrated | 1; policing (tapped is not a verbal premodifier) |
| `tetsuo_umezawa` | partly-migrated | 1; policing (tapped is not a verbal premodifier) |
| `sonar_strike` | partly-migrated | 1; policing (tapped is not a verbal premodifier) |

### `coordinated_property_values.rs` → `judgments/coordinated_property_values.txt` (18 lines)

| Test | Disposition | Detail |
|---|---|---|
| `reduce_in_stature_and_illusory_wrappings_select_the_pair` | partly-migrated | 1; shape (SlashPair 0/2 constituent has no CGEL category) |
| `godhead_of_awe_selects_the_pair_under_plural_have` | partly-migrated | 1; shape |
| `admiral_brass_selects_the_pair_under_pronominal_have` | partly-migrated | 1; shape |
| `glamer_gifter_keeps_the_property_complement_inside_the_first_predicate` | partly-migrated | 1; shape |
| `property_value_shape_matches_the_ordered_coordinates` | partly-migrated | 3; policing (arity/order of property coordinates vs slash pair) |
| `with_property_values_postmodifies_the_nominal_in_become` | partly-migrated | 1; shape |
| `independent_ordered_pair_and_premodification_preserve_both_laws` | retired | engine: independent Reading construction; Its power is 0/2. rejection is policing |
| `a_property_complement_does_not_supply_a_required_outer_determiner` | migrated | 2 |
| `only_ordinary_adjectival_premodification_preserves_property_values` | partly-migrated | 8; policing (tapped/untapped/blocked/creature power 2 rejected lexically) |

### `coordinated_subject_agreement.rs` → `judgments/coordinated_subject_agreement.txt` (14 lines)

| Test | Disposition | Detail |
|---|---|---|
| `shared_determiner_nominal_subject_requires_plural_agreement` | partly-migrated | 5; reading counts 1/3 retired (count) |
| `singular_determiner_selection_survives_nominal_coordination` | migrated | 1 |
| `singular_selecting_determiners_fix_agreement_over_coordinated_nominals` | partly-migrated | 5; whole-face counts and 2nd paragraph retired (count) |
| `alternative_coordination_preserves_singular_agreement` | partly-migrated | 2; count 1 retired (count) |
| `independently_constructed_coordinated_subject_roundtrips_with_singular_coordinates` | migrated | 1; node/word identity is shape |

### `copular_scalar_location.rs` → `judgments/copular_scalar_location.txt` (12 lines)

| Test | Disposition | Detail |
|---|---|---|
| `savage_swipe_locates_a_scalar_property` | partly-migrated | 1; "that creature is 2" negative retired (policing) |
| `copular_witnesses_retain_the_value_as_one_complement` | partly-migrated | 7; count 1 (count) and "that creature is 0 or less" negatives (policing) retired |
| `number_of_scalar_location_keeps_singular_concord` | migrated | 2 |
| `independent_scalar_location_preserves_both_laws_and_traversal_identity` | retired | shape: hand-built Readings, traversal identity |
| `independent_non_scalar_subject_and_wrong_frame_are_rejected` | retired | shape/policing: admission of hand-built invalid Readings |
| `scalar_values_and_other_copular_routes_do_not_overlap` | partly-migrated | 3; ScalarValue/route exclusivity, count 1 and grammatical-negative probes retired (shape/policing/count) |

### `correlative_adjectives.rs` → `judgments/correlative_adjectives.txt` (16 lines)

| Test | Disposition | Detail |
|---|---|---|
| `independently_constructed_correlative_adjectives_have_explicit_hosts` | partly-migrated | 7; shape policing |
| `correlative_adjectival_serials_preserve_marker_and_oxford_comma` | partly-migrated | 9; shape |

### `correlative_finite_complements.rs` → `judgments/correlative_finite_complements.txt` (9 lines)

| Test | Disposition | Detail |
|---|---|---|
| `independent_either_finite_clauses_keep_each_selected_finite_constituent` | partly-migrated | 5; shape (hand-built Readings, EitherCoordination constructors) |
| `selected_either_complements_require_finite_conjuncts_and_oxford_pairing` | partly-migrated | 4; policing (if both..and / if neither..nor are grammatical; missing Oxford comma is style) |

### `correlative_gaps.rs` → `judgments/correlative_gaps.txt` (19 lines)

| Test | Disposition | Detail |
|---|---|---|
| `independently_constructed_correlative_gaps_preserve_selected_transitive_frames` | retired | shape: constructed Readings; content covered by relative-clause lines in correlative_gaps |
| `correlative_gaps_enforce_pairing_oxford_lists_and_concord` | partly-migrated | 17; policing (missing-Oxford-comma list negatives, 4 lines) |
| `authentic_relative_excerpts_retain_the_unpronounced_shared_object` | migrated | 2 (merged into lines 1-2 of correlative_gaps.txt) |

### `correlatives.rs` → `judgments/correlatives.txt` (49 lines)

| Test | Disposition | Detail |
|---|---|---|
| `independent_correlative_pairs_preserve_every_supported_phrase_family` | partly-migrated | 25; shape,engine (Cardinal/Amount/MeasurePhrase/QuotedText/selected-head fixtures have no CGEL category, dropped) |
| `independent_either_and_neither_serials_require_final_comma_and_pair_discharge` | partly-migrated | 17; policing (neither without serial comma rejected: style rule, benign), shape |
| `malformed_pairs_bare_nor_and_disallowed_main_clause_prefixes_are_rejected` | partly-migrated | 6; engine (malformed Reading admit), policing (both you attack and they attack) |
| `correlative_selected_heads_reuse_shared_object_hosts_without_duplicate_atomic_host` | partly-migrated | 1; shape |

### `cost_scalar_complement.rs` → `judgments/cost_scalar_complement.txt` (12 lines)

| Test | Disposition | Detail |
|---|---|---|
| `cost_witnesses_read_with_their_existing_adjuncts` | migrated | 6 |
| `independent_complements_preserve_both_laws_and_traversal_identity` | partly-migrated | 2; constructed Readings, node/word identity retired (shape); `{1} less`/`to cast` fragments have no CGEL category |
| `the_infinitival_is_hollow_and_the_measure_is_one_selected_complement` | partly-migrated | 0 new (texts covered by the witnesses); hollow-gap/Reading checks retired (shape) |
| `ordinary_infinitivals_and_unlicensed_measure_heads_are_rejected` | partly-migrated | 2; to cast this spell / {1} greater / fewer retired (policing); constructed errors shape |
| `optional_infinitival_and_mixed_symbols_have_unique_constituents` | partly-migrated | 2 new; counts 2/1/2/1 retired (count) |

### `counter_compounds.rs` → `judgments/counter_compounds.txt` (11 lines)

| Test | Disposition | Detail |
|---|---|---|
| `authentic_counter_compounds_preserve_macro_owner_and_noun_inflection` | partly-migrated | 2; lexeme owner ids retired (lexical-v3) |
| `authentic_counter_np_coordination_keeps_whole_compound_conjuncts` | partly-migrated | 1; "charge and loyalty counters" negative retired (policing) |
| `authentic_numeric_counter_compounds_preserve_article_pronunciation` | migrated | 4 |
| `authentic_additional_numeric_counter_kinds_keep_signed_zero_notation` | partly-migrated | 4; lexeme ids retired (lexical-v3) |

### `counter_kind_nouns.rs` → `judgments/counter_kind_nouns.txt` (29 lines)

| Test | Disposition | Detail |
|---|---|---|
| `xathrid_gorgon` | migrated | 1 |
| `flamewar_streetwise_operative` | migrated | 1 |
| `out_of_the_tombs` | migrated | 1 |
| `the_magic_mirror` | migrated | 1 |
| `icatian_javelineers` | migrated | 1 |
| `independently_constructed_compounds_preserve_identity_and_number` | migrated | 24 |

### `data_batch_2.rs` → `judgments/data_batch_2.txt` (17 lines)

| Test | Disposition | Detail |
|---|---|---|
| `declarations_fill_attested_roles_and_preserve_both_roundtrip_laws` | partly-migrated | 15; shape (frame indices, hand-built Readings, round-trip), policing (bare "combat" has no NP reading), lexical-v3 (end/transform/surveil verbs; flip, life total, mana ability nouns declared) |
| `combat_source_deferrals_follow_the_selected_complement_route` | migrated | 2 |

### `declaration_batch_b.rs` → `judgments/declaration_batch_b.txt` (8 lines)

| Test | Disposition | Detail |
|---|---|---|
| `controlled_creatures_keep_the_verbal_postmodifier` | migrated | 2 |
| `outside_is_the_preposition_phrase_complement_of_from` | migrated | 2 |
| `modal_instead_retains_its_preposition_identity_in_central_position` | migrated | 2 |
| `independently_assembled_central_preposition_roundtrips` | retired | shape: constructed Reading round-trip with leading-space realization |
| `central_preposition_requires_its_declared_licence` | retired | policing: admit-error on constructed Outside central modifier (declared licence) |
| `independently_assembled_passive_complement_retains_control_participle` | partly-migrated | 1; shape |
| `independently_assembled_nested_source_roundtrips` | partly-migrated | 1; shape |

### `degree_quantifiers.rs` → `judgments/degree_quantifiers.txt` (16 lines)

| Test | Disposition | Detail |
|---|---|---|
| `tamanoa_degree_quantifier` | migrated | 1 |
| `mourning_thrull_degree_quantifier` | migrated | 1 |
| `crosstown_courier_degree_quantifier` | migrated | 1 |
| `guilty_conscience_degree_quantifier` | migrated | 1 |
| `firedrinker_satyr_degree_quantifier` | migrated | 1 |
| `degree_quantifiers_keep_head_countability_and_one_constituency` | partly-migrated | 11; policing (NounPhrase that many / that much rejected: benign elliptical NP) |
| `independent_degree_phrases_preserve_both_roundtrip_laws_and_traversal_identity` | retired | engine: round trip/traversal identity; single-reading claim already in degree_quantifiers_keep_head_countability line set |
| `independent_degree_values_reject_unlicensed_roles_and_wrong_noun_heads` | retired | engine: admission of hand-built invalid Readings |

### `degree_quantity_arithmetic.rs` → `judgments/degree_quantity_arithmetic.txt` (13 lines)

| Test | Disposition | Detail |
|---|---|---|
| `replacement_clauses_keep_the_pp_inside_the_degree_determiner` | partly-migrated | 2; equal-count-to-control and degree Reading identity retired (shape/count) |
| `the_quantified_noun_retains_plural_agreement_and_its_head` | partly-migrated | 3; MeasurePhrase negative and Reading family retired (shape) |
| `independently_built_determiner_and_modifier_preserve_both_roundtrip_laws` | partly-migrated | 2 (plus one is shared with the next test); During-as-modifier admit failure retired (shape) |
| `cardinal_complements_retain_spelled_digit_and_subtraction_structure` | partly-migrated | 4; ` plus X` negative retired (policing) |
| `scalar_arithmetic_and_multiplier_controls_keep_their_existing_routes` | partly-migrated | 2; ArithmeticMeasure vs modifier route retired (shape) |

### `delayed_right_constituent_passive.rs` → `judgments/delayed_right_constituent_passive.txt` (19 lines)

| Test | Disposition | Detail |
|---|---|---|
| `witnesses_keep_two_incomplete_members_and_one_shared_np` | partly-migrated | 5; Reading constructors/frames/coordination internals retired (shape) |
| `independently_constructed_shared_passive_roundtrips` | retired | shape: hand-built Reading round-trip |
| `complete_controls_keep_their_existing_routes_and_stranding_stays_unread` | partly-migrated | 14; no-PassiveSelectedHead and stranded-fragment negatives retired (shape/policing) |
| `declared_frames_and_function_licences_exclude_wrong_members` | retired | shape: admission of hand-built invalid Readings |

### `derived_punctuation.rs` → `judgments/derived_punctuation.txt` (8 lines)

| Test | Disposition | Detail |
|---|---|---|
| `selected_literals_scan_realize_and_admit_with_one_form_index` | retired | engine: local test-only construction grammar |
| `coordination_rejects_extra_forms_and_complete_consumers_reject_unterminated_content` | partly-migrated | 4; engine shape |
| `all_keyword_separator_surfaces_derive_from_the_head` | migrated | 4 |

### `designation_categories.rs` → `judgments/designation_categories.txt` (6 lines)

| Test | Disposition | Detail |
|---|---|---|
| `commodore_guff_has_an_ordinary_commander_noun` | migrated | 1 |
| `flamekin_herald_uses_the_existing_noun_as_a_premodifier` | migrated | 1 |
| `blazing_firesinger_enters_with_an_adjective` | migrated | 1 |
| `colossus_of_akros_has_an_adjectival_predicate` | migrated | 1 |
| `court_of_ire_keeps_the_determiner_separate_from_the_noun` | migrated | 1 |
| `goaded_premodification_keeps_the_verb_owner` | migrated | 1 |
| `independently_constructed_nouns_and_adjectives_roundtrip_with_their_owners` | retired | lexical-v3: designation lexemes commander/monarch/initiative/city's blessing are nouns; prepared/saddled/monstrous/renowned/solved adjectives; goaded/suspected/harnessed are verb forms not designation adjectives |

### `devotion_to_colour.rs` → `judgments/devotion_to_colour.txt` (14 lines)

| Test | Disposition | Detail |
|---|---|---|
| `independently_constructed_colour_complements_roundtrip` | partly-migrated | 2; shape |
| `witnesses_select_the_colour_complement_on_devotion` | migrated | 4 |
| `devotion_retains_scalar_denotation_in_subject_and_value_roles` | migrated | 2 |
| `selected_noun_phrase_complement_retires_devotion_modifier_attachment` | migrated | 2 |
| `colour_complement_use_does_not_create_nominal_or_elided_noun_readings` | partly-migrated | 4; policing (bare colour word as NounPhrase is a benign reading) |
| `protection_keeps_its_existing_adjectival_quality` | migrated | 1 |

### `difference_noun.rs` → `judgments/difference_noun.txt` (5 lines)

| Test | Disposition | Detail |
|---|---|---|
| `independently_constructed_anaphoric_noun_roundtrips` | partly-migrated | 1; engine |
| `difference_owns_its_regular_count_noun_paradigm` | retired | lexical-v3: difference is a count noun with regular plural differences (ScalarDenotation feature) |
| `oracle_witnesses_select_difference_in_the_complement_role` | partly-migrated | 4; shape (v3 complement-kind constructors) |

### `divided_as_you_choose.rs` → `judgments/divided_as_you_choose.txt` (8 lines)

| Test | Disposition | Detail |
|---|---|---|
| `forked_bolt` | partly-migrated | 1; exact reading count 1 and Reading shape retired (count/shape) |
| `magma_opus` | partly-migrated | 1; exact reading count 1 and Reading shape retired (count/shape) |
| `samut_the_tested` | partly-migrated | 1; exact reading count 1 and Reading shape retired (count/shape) |
| `death_to_our_enemies` | partly-migrated | 1; exact reading count 1 and Reading shape retired (count/shape) |
| `boulderfall` | partly-migrated | 1; exact reading count 1 and Reading shape retired (count/shape) |
| `among_is_not_a_dependent_of_choose_and_manner_cannot_precede_the_object` | partly-migrated | 1; two negatives retired (policing: grammatical PP; suppression of benign reading) |
| `independently_assembled_participle_preserves_both_roundtrip_laws` | migrated | 2; constructed Reading is shape |

### `do_so_and_able_to.rs` → `judgments/do_so_and_able_to.txt` (21 lines)

| Test | Disposition | Detail |
|---|---|---|
| `card_witnesses_keep_both_complement_roles_and_nominal_modification` | migrated | 5 |
| `attested_pro_form_predicates_select_the_complement` | partly-migrated | 2; Reading constructor check retired (shape) |
| `independently_constructed_pro_form_obeys_both_roundtrip_laws` | retired | shape: hand-built Reading round-trip |
| `constructed_adjective_selects_the_infinitival_complement` | partly-migrated | 1; hand-built Reading admission retired (shape) |
| `every_declared_pro_form_licence_is_required` | retired | policing: invented *Licence/*Use features toggled off |
| `bare_adjectives_and_nonagreeing_or_unlicensed_heads_gain_no_route` | partly-migrated | 9; "If you did so", "doing so", tapped-reading-shape retired (policing/shape) |
| `existing_ellipsis_and_verbless_conditional_roles_are_preserved` | partly-migrated | 4; BareEllipsis/VerblessClause constructors retired (shape) |

### `each_of_partitive.rs` → `judgments/each_of_partitive.txt` (21 lines)

| Test | Disposition | Detail |
|---|---|---|
| `succumb_to_the_cold` | migrated | 1 |
| `involuntary_cooldown` | migrated | 1 |
| `reap_what_is_sown` | migrated | 1 |
| `felidar_savior` | migrated | 1 |
| `exploration` | migrated | 1 |
| `absolute_virtue` | migrated | 1 |
| `independent_fused_head_has_one_reading_and_exact_lexical_owners` | partly-migrated | 1; shape lexical-v3 |
| `fused_head_controls_singular_agreement_independently_of_plural_oblique` | partly-migrated | 2; shape |
| `unlicensed_fused_heads_markers_and_singular_obliques_are_rejected` | partly-migrated | 7; policing count |
| `attested_object_and_preposition_positions_share_the_fused_head_structure` | migrated | 5 |

### `enter_as_copy_complement.rs` → `judgments/enter_as_copy_complement.txt` (10 lines)

| Test | Disposition | Detail |
|---|---|---|
| `essence_of_the_wild` | migrated | 1 |
| `mirror_image` | migrated | 1 |
| `vesuva` | migrated | 1 |
| `body_double` | migrated | 1 |
| `masterwork_of_ingenuity` | migrated | 1 |
| `independently_constructed_enter_complement_roundtrips` | retired | shape: hand-built Reading with frame index and round-trip |
| `existing_enter_frames_and_adjunct_exclusion_survive` | partly-migrated | 3; count (readings 1), policing (die as a copy of this creature) |
| `geths_summons_keeps_finite_as_as_an_adjunct` | migrated | 1 |
| `master_biomancer_keeps_licensed_unlike_complement_coordination` | migrated | 1 |
| `adjunct_only_coordinate_cannot_fill_the_predicative_complement` | retired | policing: unlike-function coordination of PPs rejected to suppress a benign reading |

### `enter_as_copy_except_clause.rs` → `judgments/enter_as_copy_except_clause.txt` (16 lines)

| Test | Disposition | Detail |
|---|---|---|
| `evil_twin` | migrated | 1 |
| `copycrook` | migrated | 1 |
| `phantasmal_image` | migrated | 1 |
| `machine_gods_effigy` | migrated | 1 |
| `finite_and_depictive_hosts_keep_the_exception_on_enter` | migrated | 4 |
| `controls_preserve_their_complete_reading_sets` | partly-migrated | 7; lexical-v3 (reading sets compared to a lexicon with the licence removed), count |
| `exception_requires_the_declared_enter_licence` | retired | lexical-v3: unlicensed core-verb:Enter lexicon rejects; v4 lexical fact: enter licenses except-adjunct |
| `independently_composed_exception_roundtrips` | partly-migrated | 1; shape |

### `exactly_cardinal_modifier.rs` → `judgments/exactly_cardinal_modifier.txt` (12 lines)

| Test | Disposition | Detail |
|---|---|---|
| `library_of_alexandria_keeps_precision_inside_the_card_count` | partly-migrated | 1; shape |
| `homicidal_seclusion_keeps_precision_inside_the_determiner` | partly-migrated | 1; shape |
| `near_death_experience_keeps_precision_inside_the_measured_amount` | partly-migrated | 1; shape |
| `alluring_suitor_keeps_precision_inside_the_prepositions_complement` | partly-migrated | 1; shape |
| `amalia_keeps_precision_inside_the_scalar_value` | partly-migrated | 1; shape |
| `attested_nominal_and_partitive_constituents_keep_the_numeral_modifier` | partly-migrated | 4; shape |
| `independently_constructed_precise_numerals_obey_both_roundtrip_laws` | partly-migrated | 2; engine |
| `precision_has_no_np_modifier_or_clause_adjunct_licence` | partly-migrated | 1 (Its power is exactly.); policing (exactly those colors etc. are grammatical, deferred), lexical-v3 |

### `except_matrix_complements.rs` → `judgments/except_matrix_complements.txt` (7 lines)

| Test | Disposition | Detail |
|---|---|---|
| `evil_eye_of_orms_by_gore` | partly-migrated | 1; matrix-host/PostmodifiedNominal Reading checks retired (shape) |
| `shifting_sliver` | partly-migrated | 1; matrix-host/PostmodifiedNominal Reading checks retired (shape) |
| `seeker` | partly-migrated | 1; matrix-host/PostmodifiedNominal Reading checks retired (shape) |
| `quantum_misalignment` | partly-migrated | 1; host-permission Reading checks retired (shape) |
| `keldon_firebombers` | partly-migrated | 1; host/no-nominal-modifier Reading checks retired (shape) |
| `shifting_sliver_control_retains_its_complete_reading` | partly-migrated | 1; count 7 retired (count) |
| `independently_constructed_passive_exception_roundtrips` | migrated | 1 |
| `copy_description_requires_its_declared_host_permission` | retired | policing: mutates MatrixComplementLicence feature to reject a grammatical sentence |

### `existential_there.rs` → `judgments/existential_there.txt` (19 lines)

| Test | Disposition | Detail |
|---|---|---|
| `sandstone_needle_condition` | partly-migrated | 1; Reading constructors, dummy-pronoun feature checks retired (shape) |
| `glacial_crasher_unless` | partly-migrated | 1; Reading constructors, dummy-pronoun feature checks retired (shape) |
| `kamahls_desire_as_long_as` | partly-migrated | 1; Reading constructors, dummy-pronoun feature checks retired (shape) |
| `case_of_the_shifting_visage_main_clause` | partly-migrated | 1; Reading constructors, dummy-pronoun feature checks retired (shape) |
| `drop_of_honey_temporal_clause` | partly-migrated | 1; Reading constructors, dummy-pronoun feature checks retired (shape) |
| `agreement_mismatches_do_not_read` | migrated | 8 |
| `bare_and_extended_existentials_preserve_distinct_constituencies` | partly-migrated | 2; v3 count 3 and Adjunct-of-be reading rejected by same-head priority (count) |
| `dummy_pronoun_has_no_ordinary_nominal_or_adverb_route` | partly-migrated | 2; there-as-NP/AdverbPhrase negatives and "there remains a creature" retired (policing); no Adverb/There lexeme (lexical-v3) |
| `contracted_ambiguity_and_preterite_lexical_form` | partly-migrated | 2; counts 2/3/0, "card cards" negative, preterite realization retired (count, policing, lexical-v3) |
| `independently_constructed_existential_retains_both_traversals` | retired | shape: hand-built Reading traversal identity |
| `independently_constructed_bare_existential_roundtrips` | retired | shape: hand-built Reading round-trip |
| `independent_construction_rejects_wrong_agreement_and_lexical_permissions` | retired | shape: admission of hand-built invalid Readings |

### `fixed_cost_phrases.rs` → `judgments/fixed_cost_phrases.txt` (22 lines)

| Test | Disposition | Detail |
|---|---|---|
| `glimmervoid_keeps_the_color_phrase_inside_the_mana_nominal` | migrated | 1 |
| `additional_and_chosen_mana_keep_their_internal_modifiers` | partly-migrated | 3; shape |
| `independent_mana_values_preserve_number_attachment_and_adds_existing_frame` | partly-migrated | 2; shape lexical-v3 |
| `independent_cost_opener_preserves_predication_and_infinitival_selection` | partly-migrated | 1; shape |
| `mutated_witnesses_reject_wrong_number_finiteness_and_of_adjuncts` | partly-migrated | 5; policing shape |
| `predicative_preposition_adjuncts_require_a_preposed_comma` | partly-migrated | 5; policing |
| `additional_costs_allow_discard_and_alternative_payment_actions` | migrated | 2 |
| `altars_reap_keeps_the_cast_infinitive_inside_the_predicative_cost_phrase` | migrated | 1 |
| `terrarion_keeps_the_combination_phrase_inside_the_mana_nominal` | migrated | 1 |
| `implements_of_sacrifice_keeps_any_one_color_as_a_noun_phrase` | migrated | 1 |

### `floated_each_quantifier.rs` → `judgments/floated_each_quantifier.txt` (19 lines)

| Test | Disposition | Detail |
|---|---|---|
| `ticket_witnesses_quantify_the_plural_subject_and_roundtrip` | partly-migrated | 5; shape (QuantifiedClause/PostAuxiliaryQuantification construction counts) |
| `preverbal_each_scopes_over_the_coordinated_predicate` | migrated | 0; merged into ticket_witnesses lines (Press the Advantage, Fated Clash) |
| `singular_subjects_and_post_lexical_each_are_rejected` | migrated | 6 |
| `post_auxiliary_each_preserves_modal_and_scalar_complements` | migrated | 3 |
| `independently_constructed_post_auxiliary_head_retains_its_words_and_frame` | retired | shape: hand-built QuantifiedAuxiliaryHead Reading and admission |
| `each_determiner_and_partitive_readings_are_preserved` | partly-migrated | 2; shape (no QuantifiedClause construction) |
| `relative_subject_each_retains_plural_agreement` | migrated | 3 |

### `focus_only_object.rs` → `judgments/focus_only_object.txt` (13 lines)

| Test | Disposition | Detail |
|---|---|---|
| `attested_objects_retain_their_np_heads` | partly-migrated | 5; shape (control-comparison of NP heads) |
| `subject_focus_uses_the_same_np_instance` | partly-migrated | 1; shape |
| `existing_adjunct_focus_does_not_become_np_focus` | partly-migrated | 6; shape (no FocusedPhrase NounPhrase node) |
| `independently_constructed_focused_np_roundtrips` | partly-migrated | 1; engine |

### `focusing_only.rs` → `judgments/focusing_only.txt` (18 lines)

| Test | Disposition | Detail |
|---|---|---|
| `mind_slash_predicative_as_now_reads` | partly-migrated | 1; count (single reading of the fragment) |
| `celestial_enforcer_focuses_the_conditional_adjunct` | migrated | 1 |
| `hammer_of_bogardan_focuses_the_temporal_adjunct` | migrated | 1 |
| `flywheel_racer_focuses_the_conditional_adjunct` | migrated | 1 |
| `bare_as_keeps_its_preposed_licence_and_gains_final_position` | partly-migrated | 2; count (single reading) |
| `selected_and_comparative_as_does_not_become_a_comparative_adjunct` | migrated | 1 |
| `selected_and_comparative_as_does_not_become_an_enter_adjunct` | migrated | 1 |
| `quakebringer_focuses_the_final_adjunct_in_a_finite_clause` | migrated | 1 |
| `defiant_stand_coordinates_focused_adjuncts` | migrated | 1 |
| `independent_nominal_and_interval_gaps_remain_visible` | retired | wrong: asserts grammatical Oracle sentences have no Reading (v3 vocabulary gap) |
| `focus_does_not_supply_missing_constituents_or_other_functions` | partly-migrated | 7; policing (bare "Activate only.", "as tapped", fronted "Only during your turn, activate.") |
| `independently_constructed_focus_preserves_structure_and_lexical_identity` | partly-migrated | 1; shape |

### `frequency.rs` → `judgments/frequency.txt` (9 lines)

| Test | Disposition | Detail |
|---|---|---|
| `independent_frequency_values_preserve_quantity_number_and_count_sense` | partly-migrated | 5; `X cards` negative retired (shape: FrequencyPhrase category; grammatical NP) |
| `independent_frequency_attachment_preserves_the_intransitive_frame` | partly-migrated | 1; transitive co-reading existence retired (shape) |
| `authentic_explore_frequency_and_transitive_frame_are_preserved` | partly-migrated | 3; frame index/Reading checks retired (shape) |

### `from_anywhere.rs` → `judgments/from_anywhere.txt` (9 lines)

| Test | Disposition | Detail |
|---|---|---|
| `brinebound_gift` | partly-migrated | 1; reading count 4 dropped (count) |
| `yawgmoths_agenda` | partly-migrated | 1; reading count 4 dropped (count) |
| `planar_void` | partly-migrated | 1; reading count 3 dropped (count) |
| `rest_in_peace` | partly-migrated | 1; reading count 4 dropped (count) |
| `generous_soul` | partly-migrated | 1; reading count 4 dropped (count) |
| `independently_constructed_fused_head_and_source_obey_both_laws` | partly-migrated | 2; hand-built Reading identity retired (shape) |
| `compound_determinative_replaces_the_adverb_owner` | retired | lexical-v3: lemma anywhere is a single Determinative lexeme (no locative proform) |
| `comparative_modifier_attaches_to_anywhere_and_retains_nominal_scope` | partly-migrated | 2 (ambiguity witness); hand-built Reading/admission retired (shape) |

### `from_preposition_phrase_complement.rs` → `judgments/from_preposition_phrase_complement.txt` (9 lines)

| Test | Disposition | Detail |
|---|---|---|
| `ostrich_horse_and_satyr_wayfinder_keep_the_nested_source` | migrated | 1 |
| `animal_magnetism_keeps_the_nested_source` | migrated | 1 |
| `seek_the_wilds_keeps_the_nested_source` | migrated | 1 |
| `tomakul_scrapsmith_keeps_the_nested_source` | migrated | 1 |
| `midnight_tilling_keeps_the_nested_source` | migrated | 2 |
| `bag_of_devouring_keeps_every_source_in_the_declared_verb_slot` | migrated | 1 |
| `independently_constructed_source_pp_preserves_the_complete_value` | retired | shape policing: from among them duplicated in midnight_tilling; licence feature is policing |
| `avatar_destiny_keeps_its_coordinated_tails_and_checks_their_object_features` | partly-migrated | 1; shape |
| `independently_constructed_return_checks_its_projected_object_feature` | partly-migrated | 1; shape |

### `functions_coordination.rs` → `judgments/functions_coordination.txt` (8 lines)

| Test | Disposition | Detail |
|---|---|---|
| `independent_object_gaps_keep_frames_and_concord_before_coordination` | retired | shape: v3 BareObjectGap/FiniteObjectGap fragment categories; negatives are agreement/Oxford-comma policing of those fragments |
| `selected_finite_clause_complements_coordinate_without_imperative_contamination` | partly-migrated | 4; policing (missing Oxford comma; imperative+declarative and VP-coordination rejected as CoordinatedFiniteClause) |
| `unlike_predicative_categories_share_a_function_without_erasing_their_categories` | partly-migrated | 4; shape (hand-built Readings), policing (missing Oxford comma) |

### `gapped_coordination_shared_adjunct.rs` → `judgments/gapped_coordination_shared_adjunct.txt` (12 lines)

| Test | Disposition | Detail |
|---|---|---|
| `witnesses_select_manner_inside_the_final_tail` | partly-migrated | 5; shape (Manner Complement selected inside final tail; exactly one cluster) |
| `mixed_goals_preserve_every_np_coordination_reading` | partly-migrated | 1; count (NP-coordination readings retained, exactly one added), lexical-v3 (licence-removed baseline) |
| `independently_constructed_mixed_tails_roundtrip` | partly-migrated | 1; shape |
| `mixed_tails_require_the_declared_licence` | partly-migrated | 2; lexical-v3 (SelectedTailLicence removal) |
| `nonfinal_locative_tail_selects_its_manner_complement` | partly-migrated | 1; shape (Manner selected by non-final locative tail) |
| `marked_np_tail_does_not_select_manner` | retired | policing: rejects grammatical "Put one into your hand in any order and the rest ..." |
| `attested_nonfinal_manner_tails_select_complete_frames` | partly-migrated | 2; shape |

### `generic_predicates.rs` → `judgments/generic_predicates.txt` (7 lines)

| Test | Disposition | Detail |
|---|---|---|
| `authentic_amass_consumes_declared_frame_without_shape_aliases` | partly-migrated | 2; engine,shape (Amount constituent has no CGEL category) |
| `selected_with_retains_marker_ownership_and_complement_order` | partly-migrated | 2; engine,shape |
| `legacy_role_labels_remain_unsupported_instead_of_becoming_phrase_categories` | partly-migrated | 1; lexical-v3 (unsupported frame bookkeeping) |
| `urzas_ruinous_blast_keeps_the_adjective_inside_the_relative_clause` | partly-migrated | 1; count (2 readings = commuting premodifier/relative order), shape |
| `sun_droplet_remove_retains_object_and_marked_source_roundtrip` | partly-migrated | 1; lexical-v3 (remove frames), engine |

### `gerund_preposition_complements.rs` → `judgments/gerund_preposition_complements.txt` (12 lines)

| Test | Disposition | Detail |
|---|---|---|
| `storm_of_memories` | partly-migrated | 1; per-Reading nominal-attachment scan retired (shape) |
| `spellweaver_volute` | partly-migrated | 2; Reading scan retired (shape) |
| `knowledge_exploitation` | partly-migrated | 1; Reading scan retired (shape) |
| `syncopate` | partly-migrated | 1; Reading scan retired (shape) |
| `the_five_doctors` | partly-migrated | 1; Reading scan retired (shape) |
| `independently_constructed_complement_preserves_both_roundtrip_laws` | migrated | 1; constructed Reading/words are shape |
| `gerund_complement_requires_declared_licence` | partly-migrated | 2; GerundClausePreposition class removal retired (policing/lexical-v3) |
| `complement_shape_controls_nominal_modifier_permission` | partly-migrated | 2; `the copy without paying its mana cost` NP negative retired (policing: grammatical NP, suppresses a reading) |
| `coordination_does_not_restore_a_forbidden_nominal_attachment` | retired | policing: rejects four grammatical NPs to forbid the nominal attachment |
| `jazal_goldmane_keeps_its_nominal_complement` | partly-migrated | 1; count 2, Reading scans and `of paying its mana cost` negative retired (count/policing) |

### `get_object_frames.rs` → `judgments/get_object_frames.txt` (7 lines)

| Test | Disposition | Detail |
|---|---|---|
| `attune_with_aether` | partly-migrated | 1; frame index and reading count 1 retired (shape/count) |
| `phyresis_outbreak` | partly-migrated | 1; frame index and reading count retired (shape/count) |
| `crypt_cobra` | partly-migrated | 1; frame index and reading count 2 retired (shape/count) |
| `scrapper_champion` | partly-migrated | 1; frame index and reading count retired (shape/count) |
| `vexing_radgull` | partly-migrated | 1; frame index and reading count 3 retired (shape/count) |
| `independently_constructed_energy_object_preserves_both_laws_and_traversal` | retired | shape: hand-built Reading, node/word traversal |
| `power_toughness_adjustment_keeps_its_single_frame` | partly-migrated | 2; Reading/frame-index equality and counts retired (shape/count) |

### `grammar.rs` → `judgments/grammar.txt` (212 lines)

| Test | Disposition | Detail |
|---|---|---|
| `authored_vocabulary_composes_across_the_grammar` | migrated | 18 |
| `agreement_case_frames_and_local_ellipsis_are_admission_constraints` | migrated | 15 |
| `independent_named_nominal_retains_selected_frame_and_catalog_identity` | partly-migrated | 2; shape lexical-v3 policing |
| `independent_participial_premodifier_preserves_verb_form_and_distribution` | partly-migrated | 6; shape policing |
| `lexical_gap_words_reach_existing_compositional_hosts` | migrated | 9 |
| `independent_amount_complement_consumes_its_frame_without_losing_countability` | partly-migrated | 12; shape policing |
| `independently_constructed_auxiliary_ellipsis_roundtrips_without_discourse_context` | partly-migrated | 1; shape |
| `counts_and_measures_interact_with_nominals_frames_and_prepositions` | partly-migrated | 25; shape policing |
| `independent_measure_values_preserve_operator_structure_and_notation` | partly-migrated | 2; shape |
| `small_digits_do_not_multiply_a_predicate_reading` | migrated | 1 |
| `slash_pairs_preserve_ordered_components_and_explicit_signs` | partly-migrated | 19; shape policing |
| `slash_pairs_compose_with_nominal_modifiers_and_selected_predicates` | partly-migrated | 8; shape policing |
| `independent_slash_consumers_preserve_count_components_and_leaf_order` | partly-migrated | 1; shape |
| `a_selected_adjective_frame_requires_its_complement_in_both_directions` | partly-migrated | 4; shape |
| `document_collections_connect_keywords_costs_modes_quotes_and_clauses` | partly-migrated | 26; shape multi-line texts not expressible |
| `independent_keyword_collection_preserves_parameters_separators_and_leaf_identity` | partly-migrated | 2; shape |
| `target_verb_and_targeting_marker_retain_distinct_roles_in_relatives` | partly-migrated | 3; shape policing |
| `nested_document_admission_uses_a_normal_worker_stack` | retired | engine: deep Paragraph/Parenthetical admission stack depth |
| `type_lines_require_ordered_nonempty_groups_from_declared_vocabulary` | partly-migrated | 17; shape lexical-v3 empty text not expressible |
| `independent_type_line_retains_flat_groups_and_lexical_identity` | retired | shape: SubtypedLine tree; text duplicates type_lines test |
| `complemented_adjectives_are_postpositive_in_both_directions` | partly-migrated | 5; shape |
| `integrated_pp_modifiers_stay_inside_the_nominal` | partly-migrated | 2; count shape |
| `copular_location_has_a_selected_complement_reading` | partly-migrated | 4; shape policing |
| `clause_taking_prepositions_keep_their_category_and_complement_selection` | partly-migrated | 12; wrong lexical-v3 shape |
| `serial_coordination_composes_with_agreement_case_and_predicate_forms` | migrated | 16 |
| `plain_form_leaves_finiteness_to_the_clause_construction` | partly-migrated | 3; shape policing |

### `granted_ability_quotes.rs` → `judgments/granted_ability_quotes.txt` (10 lines)

| Test | Disposition | Detail |
|---|---|---|
| `compulsory_rest_selects_an_activated_ability` | migrated | 1 |
| `carrier_thrall_selects_an_activated_ability` | migrated | 1 |
| `rain_of_filth_selects_an_ability_under_a_duration_adjunct` | migrated | 1 |
| `heroes_of_the_revel_postmodifies_the_token_with_an_ability` | partly-migrated | 1; shape (QuotedText/Ability/Document constituents; surfaces with embedded quotes not expressible) |
| `web_shooters_coordinates_a_keyword_and_a_triggered_ability` | migrated | 1 |
| `energy_flux_selects_a_triggered_ability_with_plural_agreement` | migrated | 1 |
| `independent_grant_preserves_the_nested_ability_frame_and_traversal` | retired | shape: hand-built nested Reading and traversal |
| `grant_rejects_a_second_terminal_unpaired_quotes_and_unread_inner_text` | partly-migrated | 3; policing (second terminal after quote; trailing duration after quote) |
| `ogre_marauder_preserves_its_fragment_quotation_and_following_duration` | migrated | 1 |

### `half_rounded.rs` → `judgments/half_rounded.txt` (15 lines)

| Test | Disposition | Detail |
|---|---|---|
| `grievous_wound` | migrated | 1 |
| `cut_your_losses` | migrated | 1 |
| `endless_ranks_of_the_dead` | migrated | 1 |
| `the_endstone` | migrated | 1 |
| `heartless_hidetsugu` | migrated | 1 |
| `malignus` | migrated | 1 |
| `fraction_requires_a_declared_definite_host_and_has_no_bare_dependent_route` | partly-migrated | 2; policing (half X damage / half of their library / NP+supplement negatives are benign), shape |
| `lexical_licence_controls_supplement_admission_and_keeps_the_multiplier_controls` | partly-migrated | 4; lexical-v3 (ParticipialSupplementUse removal), count |
| `independently_constructed_fraction_preserves_both_roundtrip_laws` | partly-migrated | 1; shape |
| `independently_composed_supplement_preserves_anchor_and_particle` | partly-migrated | 1; shape |
| `lurking_evil_rounding_supplement_anchors_on_the_action_cost` | migrated | 1 |

### `head_primitives.rs` → nothing migrated

| Test | Disposition | Detail |
|---|---|---|
| `independent_atomic_heads_preserve_every_exact_selected_signature` | retired | engine: fixture lexicon, atomic selected-head Readings (no CGEL phrase) |
| `independent_atomic_heads_reject_retired_ability_object_signatures` | retired | engine: fixture lexicon frame signatures |
| `independent_atomic_heads_reject_wrong_signature_index_and_morphology` | retired | engine: admission of hand-built invalid heads |

### `independent_genitive_pronoun.rs` → `judgments/independent_genitive_pronoun.txt` (24 lines)

| Test | Disposition | Detail |
|---|---|---|
| `existing_personal_pronouns_have_one_independent_genitive_head` | partly-migrated | 6; `We` lexeme absence retired (lexical-v3: no independent genitive ours declared) |
| `independent_genitive_is_the_complement_of_than` | migrated | 1 |
| `witnesses_preserve_their_controls_independent_ambiguity` | partly-migrated | 8; equal-count-to-control retired (count) |
| `dependent_forms_keep_their_nominal_and_do_not_become_independent_heads` | partly-migrated | 9; Reading-form scans and `yours` AdjectivePhrase negative retired (shape) |
| `independent_head_reuses_existing_number_neutral_agreement` | retired | engine: admission of constructed clauses vs verb person features |

### `instead_replacement.rs` → `judgments/instead_replacement.txt` (20 lines)

| Test | Disposition | Detail |
|---|---|---|
| `replacement_witnesses_retain_their_hosts_and_one_preposition_owner` | partly-migrated | 10; one-owner/no-OmittedPlain Reading checks retired (shape/lexical-v3) |
| `bare_instead_is_a_preposition_phrase_with_no_nominal_function` | partly-migrated | 3; AdverbPhrase/NounPhrase negatives retired (policing); hand-built Reading identity (shape) |
| `final_connective_retains_clause_and_coordinated_predicate_attachment` | partly-migrated | 2; SecondaryVerbPhrase re-expressed as coordinated Clauses per ruling |
| `reality_twist_retains_the_nominal_complement_of_the_connective` | migrated | 1 |
| `passive_relative_witness_keeps_the_overt_auxiliary_complement` | partly-migrated | 1; OmittedPlain absence retired (shape) |
| `attested_auxiliary_ellipsis_controls_remain_licensed` | partly-migrated | 2; OmittedPlain presence retired (shape) |
| `passive_relative_twin_retains_its_two_overt_readings` | partly-migrated | 1; count 2 and PP-after-relative reading dropped (count); stranding negatives retired (policing) |
| `unpunctuated_initial_connective_rejects_an_overt_complement` | partly-migrated | 0 new (Divine Resilience positive already migrated); "Instead of that card exile it." negative retired (policing) |

### `irrealis_scalar_difference.rs` → `judgments/irrealis_scalar_difference.txt` (8 lines)

| Test | Disposition | Detail |
|---|---|---|
| `pilot_witnesses_use_the_measure_as_comparative_modifier` | migrated | 3 |
| `independent_values_preserve_both_roundtrip_laws` | partly-migrated | 2; shape |
| `measured_comparative_requires_the_declared_licence_and_empty_frame` | retired | policing: MeasureModifierUse licence feature and frame admission |
| `irrealis_scalar_retains_subject_and_form_restrictions` | retired | policing: v3 IrrealisClause category rejects benign finite clauses |
| `explicit_comparison_and_bounded_postpositive_keep_their_roles` | partly-migrated | 3; shape |

### `italic_heads.rs` → `judgments/italic_heads.txt` (6 lines)

| Test | Disposition | Detail |
|---|---|---|
| `authentic_open_labels_preserve_independent_keyword_bodies` | partly-migrated | 2; shape (FlavorWordHead cost/word traversal), count (exactly one Reading) |
| `a_declared_ability_word_is_never_also_a_flavor_word` | partly-migrated | 1; shape (AbilityWordHead vs FlavorWordHead admission) |
| `label_requires_its_complete_written_separator_and_a_parsed_body` | partly-migrated | 3; lexical-v3 (unparsed-prose body is a vocabulary gap), engine (newline-before-dash separator not expressible) |
| `independently_constructed_nested_labels_and_invalid_lexical_shapes_are_rejected` | retired | shape: admission of hand-built nested FlavorWordHead Readings |

### `keyword_dash_cost_line.rs` → `judgments/keyword_dash_cost_line.txt` (13 lines)

| Test | Disposition | Detail |
|---|---|---|
| `independently_constructed_cost_keywords_keep_both_roundtrip_laws` | partly-migrated | 3; shape |
| `dash_parameters_retain_every_existing_cost_reading` | partly-migrated | 5; shape (Reading-set equality with Cost sub-readings) |
| `activated_costs_and_ability_words_keep_their_structure` | partly-migrated | 2; shape |
| `dash_costs_require_a_cost_parameter_and_selected_terminal` | partly-migrated | 3; policing (missing terminal period), shape (constructed haste cost Reading) |
| `final_component_projection_preserves_empty_and_nonempty_repeat_roundtrips` | retired | engine: synthetic construction-compiler fixture, not English |

### `keyword_mention_without_parameter.rs` → `judgments/keyword_mention_without_parameter.txt` (20 lines)

| Test | Disposition | Detail |
|---|---|---|
| `bare_keywords_fill_the_granted_ability_complement` | partly-migrated | 4 (merged with premodifier test on shared card texts); shape |
| `bare_keyword_is_the_complement_of_with` | partly-migrated | 2; shape |
| `keyword_premodifiers_keep_keyword_identity_in_costs_and_abilities` | partly-migrated | 5 (2 merged into bare_keywords lines); shape |
| `independently_constructed_bare_mentions_roundtrip` | partly-migrated | 4; shape,engine (GrantedAbility duplicates dropped) |
| `parameterized_keywords_keep_their_complete_payload_reading` | partly-migrated | 5; shape |
| `keyword_premodification_requires_the_head_permission` | retired | policing: flashback creature / bare Boast, Exhaust, Enchant, Prototype rejected lexically; engine admit checks |
| `cycling_keeps_the_gerund_and_keyword_premodifier_readings` | migrated | 1 (ambiguity witness) |

### `keyword_payloads.rs` → `judgments/keyword_payloads.txt` (17 lines)

| Test | Disposition | Detail |
|---|---|---|
| `authentic_quality_and_subject_keywords_keep_declared_payloads` | partly-migrated | 3; constructed Reading identity and 2-Reading set retired (shape/count) |
| `authentic_amount_cost_quality_cost_and_size_payloads_keep_separators` | partly-migrated | 3; constructed Reading identity retired (shape) |
| `quality_amount_keywords_read_a_bare_quality_then_a_number` | partly-migrated | 5; bare `Devour Food` negative retired (policing: bare keyword mention now valid); Reading shape |
| `keyword_payload_contracts_reject_wrong_markers_numbers_and_separators` | partly-migrated | 5; Enchant/Prototype/Splice onto Arcane missing-parameter negatives retired (policing) |
| `authentic_boast_payload_preserves_activated_ability_structure` | partly-migrated | 1; constructed paragraph/ability tree retired (shape) |

### `keyword_pp.rs` → `judgments/keyword_pp.txt` (7 lines)

| Test | Disposition | Detail |
|---|---|---|
| `selected_keyword_pp_bases_roundtrip_before_coordination` | partly-migrated | 3; hand-built Reading and "from flying" negative retired (shape/policing) |
| `keyword_pp_coordinates_only_after_its_primitive_is_licensed` | partly-migrated | 4; hand-built Reading retired (shape) |

### `keyword_quality_phrases.rs` → `judgments/keyword_quality_phrases.txt` (2 lines)

| Test | Disposition | Detail |
|---|---|---|
| `authentic_coordinated_qualities_preserve_determined_noun_phrase_structure` | partly-migrated | 2; shape count |
| `keyword_quality_rejects_targeting_including_mixed_coordinands` | retired | policing: protection quality excludes targeting; grammatical NP |

### `keyword_state_adjectives.rs` → `judgments/keyword_state_adjectives.txt` (15 lines)

| Test | Disposition | Detail |
|---|---|---|
| `independent_state_complements_preserve_both_roundtrip_laws` | partly-migrated | 2; shape (hand-built Readings) |
| `remain_and_become_witnesses_select_the_declared_adjective` | partly-migrated | 3; policing (it remains destroyed rejected as lexical gap) |
| `be_preserves_the_adjectival_and_verbal_passive_analyses` | migrated | 3 |
| `nominal_participles_keep_their_route_and_by_adjunct_matches_tapped` | partly-migrated | 5; shape (no adjective lexeme in premodifier route) |
| `saddle_active_witnesses_use_the_exported_verb` | migrated | 2 |

### `keyword_verbs.rs` → `judgments/keyword_verbs.txt` (17 lines)

| Test | Disposition | Detail |
|---|---|---|
| `attested_keyword_verbs_have_their_declared_roles_in_every_reading` | migrated | 12 |
| `independently_built_mutate_predicates_preserve_both_roundtrip_laws` | partly-migrated | 2; shape |
| `henry_wus_exploited_creature_is_a_verbal_premodifier` | migrated | 1 |
| `saddle_keeps_the_attested_adjectival_state_routes` | migrated | 2 |

### `kind_of_counter.rs` → `judgments/kind_of_counter.txt` (15 lines)

| Test | Disposition | Detail |
|---|---|---|
| `independently_constructed_taxonomic_complement_obeys_both_roundtrip_laws` | partly-migrated | 1; engine |
| `postmodifiers_keep_the_ordinary_nominal_attachment_shapes` | partly-migrated | 4; shape |
| `ticket_witnesses_use_the_licensed_nominal_complement` | partly-migrated | 5; shape |
| `bare_count_complement_requires_the_declared_head_licence` | retired | policing: type of counter / counter / of counter rejected via licence feature; lexical-v3 lexicon manipulation |
| `existing_controls_retain_their_ordinary_np_complements` | partly-migrated | 5; shape |

### `landwalk_bound_suffix.rs` → `judgments/landwalk_bound_suffix.txt` (9 lines)

| Test | Disposition | Detail |
|---|---|---|
| `keyword_labels_preserve_bound_structure_and_both_roundtrip_laws` | partly-migrated | 6; Document-level count 1 and word identity retired (shape) |
| `running_text_witnesses_retain_lowercase_stems` | partly-migrated | 2; stem/suffix word identity retired (lexical-v3) |
| `bound_stems_do_not_admit_free_nouns_or_separated_keywords` | retired | policing: bare-stem/capitalisation/fragment negatives reject benign readings |
| `generic_landwalk_has_only_the_composed_lexical_route` | partly-migrated | 1; landwalk is composed (land stem + bound suffix), no standalone landwalk lexeme (lexical-v3) |

### `last_turn_temporal_adjunct.rs` → `judgments/last_turn_temporal_adjunct.txt` (10 lines)

| Test | Disposition | Detail |
|---|---|---|
| `first_response` | partly-migrated | 1; Reading constructors/word-owner check retired (shape/lexical-v3) |
| `lambholt_elder` | partly-migrated | 1; Reading constructors/word-owner check retired (shape/lexical-v3) |
| `silverpelt_werewolf` | partly-migrated | 1; Reading constructors/word-owner check retired (shape/lexical-v3) |
| `call_of_the_full_moon` | partly-migrated | 1; Reading constructors/word-owner check retired (shape/lexical-v3) |
| `feast_on_the_fallen` | partly-migrated | 1; Reading constructors/word-owner check retired (shape/lexical-v3) |
| `independently_constructed_deictic_temporal_adjunct_obeys_both_laws` | partly-migrated | 1; admission of hand-built Readings and "turn"/"next turn" negatives retired (shape/policing) |
| `last_has_one_word_owner_and_keeps_ordinary_premodification` | partly-migrated | 4; lexical note: "last" is one AttributiveAdjective lexeme (lexical-v3) |

### `lexical_compositions.rs` → `judgments/lexical_compositions.txt` (29 lines)

| Test | Disposition | Detail |
|---|---|---|
| `combat_damage_keeps_its_noun_premodifier_and_mass_head` | migrated | 3 |
| `defending_player_is_a_bare_np_with_a_verbal_premodifier` | partly-migrated | 3; policing shape |
| `plural_bare_genitives_preserve_possessor_number_and_the_apostrophe` | migrated | 8 |
| `attested_hybrid_and_phyrexian_symbols_retain_cost_and_mana_identity` | partly-migrated | 2; lexical-v3 |
| `at_random_retains_a_preposition_with_an_adjective_complement` | partly-migrated | 2; policing shape |
| `color_negative_formation_keeps_the_adjective_lexical_hosts` | migrated | 11 |

### `lexical_gaps.rs` → `judgments/lexical_gaps.txt` (9 lines)

| Test | Disposition | Detail |
|---|---|---|
| `oracle_constituents_use_declared_verb_frames` | partly-migrated | 9; policing ("this creature remains" alone is grammatical), shape (frame index); lexical-v3 (remain locative frame) |
| `independent_intransitive_values_retain_frame_and_lexical_identity` | retired | lexical-v3: resolve and trigger are intransitive-only verbs (frame 0); also shape (hand-built Reading) |

### `library_positions.rs` → `judgments/library_positions.txt` (13 lines)

| Test | Disposition | Detail |
|---|---|---|
| `look_selects_library_cards_with_internal_position_and_of_modifier` | migrated | 1 |
| `reveal_and_exile_select_library_position_objects` | migrated | 2 |
| `put_consumes_its_order_tail_in_every_retained_reading` | partly-migrated | 2; policing (rejects grammatical "look at ... in any order") |
| `independent_position_pp_retains_noun_complement_and_both_roundtrip_laws` | partly-migrated | 3; shape |
| `put_selects_a_position_destination_instead_of_a_free_adjunct` | migrated | 2 |
| `selected_destination_preserves_passive_and_object_gap_readings` | migrated | 2 |
| `stand_together_retains_coordinated_object_and_locative_complements` | partly-migrated | 1; count (5 readings), shape |

### `licence_hygiene.rs` → `judgments/licence_hygiene.txt` (7 lines)

| Test | Disposition | Detail |
|---|---|---|
| `unattested_have_nominal_exception_is_rejected` | retired | policing: grammatical but unattested exception rejected |
| `scheming_fence_retains_its_loyalty_ability_residual` | partly-migrated | 1 (polarity reversed: real Oracle text, v3 asserted 0 readings); wrong |
| `passive_host_with_an_internalised_marker_rejects_another_internalised_complement` | retired | policing: grammatical stacked by-phrase/except-by rejected |
| `shifting_sliver_keeps_its_unfilled_passive_exception` | partly-migrated | 1; count (1 reading contradicts be+participle keeping both readings) |
| `restricted_pp_cannot_be_laundered_through_an_embedded_complement` | migrated | 2 |
| `satyr_wayfinder_keeps_its_nested_source_complement` | migrated | 1 |
| `season_of_the_witch_nominal_exception_allows_a_comma` | partly-migrated | 1; shape |
| `content_exception_allows_a_space` | partly-migrated | 1; shape |
| `separator_does_not_supply_a_missing_host_licence` | retired | policing: Draw a card except for three rejected |

### `mana.rs` → `judgments/mana.txt` (14 lines)

| Test | Disposition | Detail |
|---|---|---|
| `independent_mana_values_preserve_symbol_sequence` | partly-migrated | 2; negatives ({T},{Q},{2},... as ManaPhrase) retired (policing); admit failures shape |
| `colored_payment_objects_have_one_reading` | partly-migrated | 3; count 1 retired (count) |
| `independent_finite_and_secondary_complements_use_declared_frame` | partly-migrated | 4; `draws {G}`/`add {T}` negatives retired (policing); frame indices shape |
| `authentic_basic_mana_abilities_have_structured_complements` | partly-migrated | 5; count 1 and SelectedPredicate/ManaPhrase scan retired (count/shape) |

### `mana_symbol_nominal.rs` → `judgments/mana_symbol_nominal.txt` (9 lines)

| Test | Disposition | Detail |
|---|---|---|
| `independently_constructed_symbol_sequences_preserve_both_roundtrip_laws` | retired | shape: hand-built Readings; Nominal-category symbol sequences |
| `additional_mana_is_the_head_of_the_selected_object_nominal` | partly-migrated | 4; symbol-nominal head Reading shape and frame item checks retired (shape) |
| `cost_notation_is_the_subject_of_the_licensed_locative_small_clause` | partly-migrated | 2; IGNORED upstream (v3 route missing); small-clause subject claim retired (shape) |
| `bare_symbols_keep_their_notation_routes_and_cannot_be_noun_phrases` | partly-migrated | 3; symbol-not-NounPhrase negatives and no-MeasuredSymbolNominal retired (shape) |

### `mana_symbol_passive_subject.rs` → `judgments/mana_symbol_passive_subject.txt` (13 lines)

| Test | Disposition | Detail |
|---|---|---|
| `witnesses_keep_mana_subject_and_existing_purpose_attachments` | partly-migrated | 5; count shape |
| `the_simple_conditional_keeps_both_passive_purpose_attachment_heights` | migrated | 1 |
| `passive_licence_belongs_to_the_declared_frame_class` | partly-migrated | 3; lexical-v3 policing |
| `spend_selects_the_mana_complement_without_an_object_projection` | partly-migrated | 2; shape |
| `agreement_and_symbol_boundaries_remain_governed` | partly-migrated | 1; policing shape |
| `independently_constructed_passive_preserves_both_roundtrip_laws` | partly-migrated | 1; shape |

### `measured_damage_nominal.rs` → `judgments/measured_damage_nominal.txt` (14 lines)

| Test | Disposition | Detail |
|---|---|---|
| `hold_at_bay_keeps_its_measured_nominal_and_relative_postmodifier` | migrated | 1 |
| `esper_battlemage_keeps_its_measured_nominal_and_relative_postmodifier` | migrated | 1 |
| `anoint_keeps_its_measured_nominal_and_relative_postmodifier` | migrated | 1 |
| `barbed_wire_keeps_its_measured_nominal_and_relative_postmodifier` | migrated | 1 |
| `kitsune_healer_keeps_its_measured_nominal_and_relative_postmodifier` | migrated | 1 |
| `independent_bare_measurements_have_one_constituency_and_both_roundtrip_laws` | partly-migrated | 5; shape (hand-built MeasuredNounPhrase and node/word traversal) |
| `numeral_measurements_allow_determiners_and_postmodifiers_without_count_plural` | partly-migrated | 4; policing (3 damage that would..., 3 damage to any target, 3 mana of any color, two damage are benign) |

### `modal_header_quantities.rs` → `judgments/modal_header_quantities.txt` (15 lines)

| Test | Disposition | Detail |
|---|---|---|
| `modal_headers_keep_the_count_complement_and_each_mode_body` | partly-migrated | 3; shape (bulleted Mode bodies are multi-line, not expressible) |
| `count_shapes_are_unique_and_do_not_compete_with_nominal_objects` | partly-migrated | 12; count (exactly-one counts), wrong ("Choose both." / "Choose one that hasn't been chosen." / "You may choose both instead." / "Put one into your hand." are grammatical) |
| `independently_constructed_coordinated_count_roundtrips_in_its_selected_frame` | retired | shape: constructed Readings; content covered by Choose one or both line |

### `multiplier_predeterminer.rs` → `judgments/multiplier_predeterminer.txt` (12 lines)

| Test | Disposition | Detail |
|---|---|---|
| `boon_reflection` | partly-migrated | 1; shape (host-reading-set equality) |
| `fire_servant` | partly-migrated | 1; shape |
| `polliwallop` | partly-migrated | 1; shape |
| `price_of_progress` | partly-migrated | 1; shape |
| `jovial_evil` | partly-migrated | 1; shape |
| `city_on_fire_keeps_the_multiplier_noun` | partly-migrated | 1; shape |
| `bruvac_keeps_the_demonstrative_quantity_inside_the_host` | partly-migrated | 1; shape |
| `independently_constructed_multiplier_preserves_both_roundtrip_laws` | partly-migrated | 1; engine |
| `declared_licence_preserves_the_controls_complete_reading_sets` | partly-migrated | 4; lexical-v3 (PredeterminerUse removal), count |
| `nondefinite_and_unclassified_hosts_do_not_gain_a_multiplier_route` | retired | policing: attested scalar-X forms (twice X life) rejected as NP; deferred, not ungrammatical |

### `multiword_lexemes.rs` → `judgments/multiword_lexemes.txt` (17 lines)

| Test | Disposition | Detail |
|---|---|---|
| `independent_multiword_nominals_own_one_lexical_leaf_in_both_roundtrip_directions` | partly-migrated | 8; single-lexical-leaf identity (mana value, mana cost one lexeme) is lexical-v3 |
| `grammar_consumes_noun_number_and_countability_without_decomposing_the_lexeme` | partly-migrated | 2; wrong-mass/crossed-number admit errors retired (shape) |
| `independent_orientation_pps_and_coordination_preserve_whole_lexical_words` | partly-migrated | 5; whole-lexical-word leaves (face up/face down single lexemes) lexical-v3 |
| `orientation_predication_uses_the_existing_selected_locative_frame` | partly-migrated | 2; frame index shape |
| `reusable_realization_context_checks_independent_values_and_changed_text` | retired | engine: RealizationContext vs Grammar::realize equivalence |

### `name_lexeme_scope.rs` → `judgments/name_lexeme_scope.txt` (23 lines)

| Test | Disposition | Detail |
|---|---|---|
| `shortened_names_need_their_cards_scope` | migrated | 5 |
| `shortened_names_keep_one_card_identity_and_roundtrip_in_its_scope` | partly-migrated | 6; lexical-v3 (Jace the Mind Sculptor / Ajani Goldmane have no shortened name in catalog, variant indices, Name realization), count (3 readings), shape |
| `type_lines_use_subtype_nouns_but_never_plural_or_negative_forms` | migrated | 10 |
| `plural_full_name_subject_and_token_name_keep_their_intended_identity` | partly-migrated | 2; lexical-v3 (plural agreement of name lexeme Aang and Katara, variant 0), count |

### `named_modes.rs` → `judgments/named_modes.txt` (20 lines)

| Test | Disposition | Detail |
|---|---|---|
| `whole_faces_have_flavor_word_labels_on_modes_and_parsed_paragraph_bodies` | partly-migrated | 16; shape multi-line modal text not expressible |
| `plain_modal_controls_keep_unlabelled_paragraph_bodies` | retired | shape: multi-line modal text not expressible in a judgment line |
| `a_flavor_word_head_outside_a_mode_remains_an_ordinary_ability` | migrated | 1 |
| `bullets_and_a_modal_header_are_not_standalone_flavor_abilities` | migrated | 3 |
| `an_independently_constructed_named_mode_roundtrips_with_its_exact_label` | retired | shape: Mode form index and flavor-word label admission |

### `named_modes_regression.rs` → `judgments/named_modes_regression.txt` (6 lines)

| Test | Disposition | Detail |
|---|---|---|
| `siege_sentence_line_hosts_named_modes_with_paragraph_bodies` | partly-migrated | 2; multi-line modal document structure not expressible as one line, mixed-label negatives retired (shape/policing) |
| `typhoid_mary_sentence_paragraph_hosts_all_three_named_modes` | partly-migrated | 4; multi-line modal document structure not expressible as one line (shape) |

### `named_token_apposition.rs` → `judgments/named_token_apposition.txt` (11 lines)

| Test | Disposition | Detail |
|---|---|---|
| `token_descriptions_supplement_the_name_inside_the_object` | partly-migrated | 5; count (witness count equals control count), shape (AscriptiveNounPhraseSupplement wrapping) |
| `controls_keep_their_unsupplemented_structures` | partly-migrated | 5; shape (no supplement node) |
| `supplement_requires_a_name_anchor_and_indefinite_article` | retired | policing: definite or generic-anchor appositives (Angelo, the ...; a token, a ...) are benign English |
| `independently_composed_supplement_preserves_both_roundtrip_laws` | partly-migrated | 1; shape (hand-built Reading composition) |

### `negated_to_infinitival.rs` → `judgments/negated_to_infinitival.txt` (11 lines)

| Test | Disposition | Detail |
|---|---|---|
| `witnesses_keep_secondary_negation_in_the_selected_infinitival` | partly-migrated | 4; shape |
| `controls_preserve_their_roles_and_roundtrip` | partly-migrated | 3; shape |
| `secondary_licence_does_not_supply_matrix_negation_or_the_to_not_alternant` | partly-migrated | 2; policing (split infinitive to not untap, double not rejected) |
| `negation_requires_the_named_word` | retired | lexical-v3: not is a named negator lexeme (identity-gated) |
| `independently_constructed_negated_complement_preserves_both_laws` | partly-migrated | 2 (merged into witnesses/secondary lines); engine |

### `negative_common_noun_premodifier.rs` → `judgments/negative_common_noun_premodifier.txt` (7 lines)

| Test | Disposition | Detail |
|---|---|---|
| `hour_of_reckoning` | partly-migrated | 1; lexeme-owner checks retired (lexical-v3) |
| `swift_spiral` | partly-migrated | 1; lexeme-owner checks retired (lexical-v3) |
| `rhythm_of_the_wild` | partly-migrated | 1; lexeme-owner checks retired (lexical-v3) |
| `isilu_carrier_of_twilight` | partly-migrated | 1; lexeme-owner checks retired (lexical-v3) |
| `thraben_watcher` | partly-migrated | 1; lexeme-owner checks retired (lexical-v3) |
| `independently_constructed_negative_noun_keeps_its_identity_and_plural_head` | partly-migrated | 2; `token`/`nontokens` NounPremodifier negatives retired (policing); identity lexical-v3 |

### `next_time_relative_adjunct.rs` → `judgments/next_time_relative_adjunct.txt` (13 lines)

| Test | Disposition | Detail |
|---|---|---|
| `dazzling_reflection` | partly-migrated | 1; InitialPreposition/AdverbialGapRelativeClause constructors retired (shape) |
| `words_of_waste` | partly-migrated | 1; constructors retired (shape) |
| `new_way_forward` | partly-migrated | 1; constructors retired (shape) |
| `circle_of_protection_red` | partly-migrated | 1; constructors retired (shape) |
| `words_of_wilding` | partly-migrated | 1; constructors retired (shape) |
| `independently_constructed_definite_temporal_adjunct_requires_a_dependent` | partly-migrated | 2; hand-built Readings and "the time" negative retired (shape/policing) |
| `independently_preposed_this_turn_uses_the_same_construction_and_checks_form_category` | partly-migrated | 1; hand-built Reading admission retired (shape) |
| `each_time_requires_a_count_nominal` | partly-migrated | 1; count 1 dropped (count) |
| `preposed_licence_does_not_open_postposed_definite_or_manner_routes` | partly-migrated | 5; three postposed/manner negatives retired (policing) |

### `no_more_than.rs` → `judgments/no_more_than.txt` (28 lines)

| Test | Disposition | Detail |
|---|---|---|
| `comparative_modifier_is_internal_to_the_determinative_phrase` | partly-migrated | 3; shape (hand-built comparison Readings and round-trip) |
| `existing_quantificational_modifier_class_is_licensed` | partly-migrated | 8; shape (hand-built Readings) |
| `frequency_comparison_preserves_its_complement_and_roundtrip` | partly-migrated | 7; shape (FrequencyPredicate constructor, hand-built Readings) |
| `authentic_witnesses_retain_the_comparative_modifier_role` | partly-migrated | 5; shape (ComparativeDeterminativePhrase modifier/head lexeme ids) |
| `comparative_modifier_preserves_agreement_and_polarity_boundaries` | partly-migrated | 5; count (modified readings equal control count; 2 readings for "No creatures can attack..."), shape (hand-built Reading) |

### `nominal_adjuncts.rs` → `judgments/nominal_adjuncts.txt` (9 lines)

| Test | Disposition | Detail |
|---|---|---|
| `authentic_boundary_pp_keeps_its_bare_interval_structure` | partly-migrated | 1; shape |
| `independently_constructed_combat_intervals_roundtrip` | partly-migrated | 4; shape |
| `authentic_temporal_and_manner_adjuncts_keep_their_noun_identity` | partly-migrated | 3; policing |
| `authentic_preterite_host_preserves_temporal_adjunct_attachment` | partly-migrated | 1; shape count |

### `noncombat_negative_noun.rs` → `judgments/noncombat_negative_noun.txt` (6 lines)

| Test | Disposition | Detail |
|---|---|---|
| `chandras_spitfire` | partly-migrated | 1; count (reading count equals combat twin) |
| `niv_mizzet_visionary` | partly-migrated | 1; count (reading count equals combat twin) |
| `mark_of_asylum` | partly-migrated | 1; count (reading count equals combat twin) |
| `wildfire_elemental` | partly-migrated | 1; count (reading count equals combat twin) |
| `chandras_pyreling` | partly-migrated | 1; count (reading count equals combat twin) |
| `negative_noun_retains_plugin_provenance_and_declared_properties` | retired | lexical-v3: noncombat is plugin noun combat with non- prefix, same properties, lemma noncombat |
| `independently_constructed_negative_premodifier_roundtrips_with_mass_head` | partly-migrated | 1; policing (non-combat / non combat / bare noncombat AdjectivePhrase negatives), shape |

### `notation_coordination.rs` → `judgments/notation_coordination.txt` (20 lines)

| Test | Disposition | Detail |
|---|---|---|
| `independent_noncoordinated_units_survive` | partly-migrated | 2; shape,engine (Cardinal/Amount/MeasurePhrase/QuotedText units have no CGEL category, dropped) |
| `independently_constructed_binary_and_oxford_series_retain_every_child` | partly-migrated | 3; shape,engine (cardinal coordination dropped, no CGEL category) |
| `all_notation_families_compose_and_reject_non_oxford_serials` | partly-migrated | 8; policing (serial comma omission rejected: style rule), shape (Cardinal/Amount/MeasurePhrase/QuotedText dropped) |
| `cardinal_coordination_agrees_with_the_whole_range` | migrated | 9 |
| `correlative_markers_keep_determinative_identity_and_nor_is_distinct` | retired | lexical-v3: both/either/neither determinative lexemes; nor is Alternative correlative coordinator; and/or noncorrelative |
| `bare_nor_needs_negative_or_correlative_licensing` | partly-migrated | 3 (2 merged); policing/shape (Cardinal/Amount/MeasurePhrase/QuotedText dropped) |

### `noun_premodifiers.rs` → `judgments/noun_premodifiers.txt` (24 lines)

| Test | Disposition | Detail |
|---|---|---|
| `independently_declared_noun_modifier_bases_keep_noun_identity` | partly-migrated | 3; NounPremodifier fragments and lexeme category asserts retired (lexical-v3) |
| `authentic_noun_modification_and_stacking_have_complete_independent_values` | partly-migrated | 4; node/word identity retired (shape) |
| `authentic_plural_heads_do_not_require_plural_noun_modifiers` | partly-migrated | 3; plural-modifier negatives retired (policing); wrong premodifier admit error shape |
| `authentic_negative_type_nouns_use_the_same_modifier_primitive` | partly-migrated | 4; barrier Reading shape retired |
| `authentic_coordinated_noun_modifiers_share_the_head_with_oxford_serials` | partly-migrated | 3; comma-less/serial-comma negatives retired (policing); fragment categories shape |
| `permanent_premodifiers_keep_independently_constructed_noun_identity` | partly-migrated | 5; barrier/noun-identity Reading shape retired |
| `permanent_head_use_is_unchanged_and_plural_modifiers_are_rejected` | partly-migrated | 2; plural premodifier negatives retired (policing) |

### `number_transparency.rs` → `judgments/number_transparency.txt` (10 lines)

| Test | Disposition | Detail |
|---|---|---|
| `sway_subject_preserves_exact_readings_and_nominal_head` | partly-migrated | 3; exact-Reading equality, word-owner list, laws retired (shape/lexical-v3) |
| `independently_constructed_sway_clause_requires_plural_third_concord` | partly-migrated | 4; hand-built Reading admission, NominalCore category retired (shape) |
| `eerie_interlude_keeps_both_relative_clause_scopes_and_plural_concord` | partly-migrated | 1 (two-group ambiguity witness); exact 3-Reading set and admission summaries retired (shape/count) |
| `attested_ordinary_number_constituents_keep_singular_concord` | partly-migrated | 3; hand-built Reading/Summary equality retired (shape); "the number ... become" negative derived from constructed inadmissible clause |
| `transparent_quantification_rejects_singular_obliques_and_preserves_other_heads` | partly-migrated | 1; "any number of mana" negative retired (policing); rest shape |
| `a_later_of_modifier_keeps_the_first_oblique_and_both_attachment_scopes` | partly-migrated | 1 (ambiguity witness); Summary equality retired (shape) |

### `once_frequency.rs` → `judgments/once_frequency.txt` (16 lines)

| Test | Disposition | Detail |
|---|---|---|
| `kozileks_translator_frequency_restriction` | migrated | 1 |
| `jeskai_devotee_frequency_restriction` | migrated | 0 duplicate of kozileks_translator_frequency_restriction |
| `basking_rootwalla_frequency_restriction` | migrated | 0 duplicate of kozileks_translator_frequency_restriction |
| `twinblade_slasher_frequency_restriction` | migrated | 0 duplicate of kozileks_translator_frequency_restriction |
| `ondu_spiritdancer_frequency_restriction` | migrated | 1 |
| `bounding_frequency_retains_adverb_category` | partly-migrated | 4; policing count |
| `independent_frequency_focus_preserves_both_roundtrip_laws_and_traversal` | partly-migrated | 2; shape |
| `frequency_and_distributive_temporal_np_remain_separate_adjuncts` | partly-migrated | 1; policing shape |
| `remorseless_punishment_unfocused_frequency_retains_the_object` | migrated | 1 |
| `independent_fused_determiner_head_and_transitive_do_preserve_lexical_identity` | partly-migrated | 5; shape |
| `each_player_who_does_has_one_subject_relative_with_auxiliary_ellipsis` | partly-migrated | 1; shape |

### `overgeneration_prune.rs` → `judgments/overgeneration_prune.txt` (11 lines)

| Test | Disposition | Detail |
|---|---|---|
| `loyalty_costs_use_oracle_signs_and_cannot_join_cost_lists` | partly-migrated | 3; policing (ASCII hyphen sign and mixed cost lists rejected as game-rule policing), count (Cost readings 1), shape (SlashPair) |
| `a_filled_passive_rejects_a_second_internalised_complement_adjunct` | partly-migrated | 2; count (readings 2/1/7) |
| `singular_determiners_resolve_subject_relative_agreement` | migrated | 3 |
| `spore_cloud_distributive_np_coordination_agrees_singular` | migrated | 3 |
| `unattested_quantified_auxiliary_and_central_positions_are_pruned` | retired | policing: "They are each dealt 2 damage.", "You may still cast it." etc. are grammatical English |

### `participial_selection.rs` → `judgments/participial_selection.txt` (4 lines)

| Test | Disposition | Detail |
|---|---|---|
| `independent_authentic_perfect_passives_preserve_both_auxiliary_layers` | partly-migrated | 2; shape |
| `authentic_mixed_coordination_retains_ordinary_and_passive_predicates` | partly-migrated | 1; shape |
| `auxiliary_selection_rejects_bare_passives_and_repeated_passive_auxiliaries` | partly-migrated | 1; policing ("have destroyed" fragment), shape (admit-error constructions) |

### `participial_uses.rs` → `judgments/participial_uses.txt` (15 lines)

| Test | Disposition | Detail |
|---|---|---|
| `authentic_attacking_and_blocking_modifiers_retain_verbal_identity` | partly-migrated | 2; shape |
| `authentic_tapped_and_untapped_modifiers_retain_adjective_identity` | partly-migrated | 2; shape (premodified-nominal vs barrier-premodified form) |
| `idyllic_beachfront_enters_with_an_adjectival_depictive` | partly-migrated | 1; shape |
| `righteous_blow_keeps_coordinated_verbal_modifiers_under_the_nominal` | partly-migrated | 1; shape |
| `yore_tiller_nephilim_coordinates_adjectival_and_participial_depictives` | partly-migrated | 1; shape |
| `copular_become_does_not_select_a_gerund_participial_predicate` | retired | policing: becomes attacking rejected by frame selection |
| `hero_of_bladehold_auxiliary_frame_is_not_an_intransitive_depictive_host` | partly-migrated | 0 new (! assertion folded into hero_of_bladehold_mixed line); engine |
| `grafdiggers_cage_enter_the_battlefield_keeps_an_object_complement` | partly-migrated | 1; shape |
| `authentic_complemented_participial_postmodifiers_preserve_their_objects_and_auxiliary` | partly-migrated | 3; shape, count (player being attacked: be+participle keeps both readings) |
| `hero_of_bladehold_mixed_be_complement_preserves_adjective_and_progressive` | partly-migrated | 1; shape, count |
| `actual_movement_constituents_keep_selected_destination_before_depictive` | partly-migrated | 3; shape, count (plain/past-participle form duplicates) |
| `participial_postmodifiers_and_mixed_complements_reject_wrong_form_or_function` | retired | engine: admission of hand-built invalid Readings, empty-text parses |

### `partitive_determinative_heads.rs` → `judgments/partitive_determinative_heads.txt` (58 lines)

| Test | Disposition | Detail |
|---|---|---|
| `named_witnesses_select_the_of_complement_under_the_determinative` | partly-migrated | 5; partitive Reading identity retired (shape) |
| `independent_plural_partitives_have_exactly_the_fused_head_route` | migrated | 13 |
| `mass_partitives_use_the_same_selected_complement_construction` | migrated | 8 |
| `unrestricted_plural_agreement_is_selected_by_the_verb_without_a_twin_np` | partly-migrated | 3; exact reading-set equality retired (shape) |
| `number_selection_follows_the_declared_head_use` | migrated | 16 |
| `licensed_head_marker_and_oblique_restrictions_are_retained` | partly-migrated | 10; `any among them`,`all among them`,`any of the exiled card`,`some/all/much of that creature` retired (policing: grammatical partitives of singular nouns) |
| `all_as_object_keeps_the_selected_partitive_complement` | migrated | 1 |
| `noun_oblique_relatives_remain_inside_the_selected_complement` | migrated | 2 |
| `fused_head_relatives_remain_a_shared_residual` | retired | policing: asserts known coverage gap (Destroy any of them that are Walls) must fail |

### `partitive_quantity_phrases.rs` → `judgments/partitive_quantity_phrases.txt` (13 lines)

| Test | Disposition | Detail |
|---|---|---|
| `independent_quantity_heads_keep_one_route_and_both_roundtrip_laws` | partly-migrated | 5; hand-built Reading round-trip retired (shape) |
| `multiplier_remains_external_to_the_degree_headed_partitive` | partly-migrated | 1; "twice external to partitive" structure not migrated (shape) |
| `bare_cardinal_partitives_use_the_same_quantitative_head_route` | partly-migrated | 2; inadmissible hand-built Readings retired (shape) |
| `urzas_armor_has_a_numeric_head_and_noncount_partitive_oblique` | partly-migrated | 1; PartitiveNounPhrase Reading retired (shape) |
| `scalpelexis_keeps_the_quantity_coordination_inside_the_fused_head` | partly-migrated | 1; PartitiveNounPhrase Reading retired (shape) |
| `mangara_has_a_comparative_quantity_subject` | partly-migrated | 1; PartitiveNounPhrase Reading retired (shape) |
| `mana_rig_has_a_quantitative_preposition_phrase_as_fused_head` | partly-migrated | 1; PartitiveNounPhrase Reading retired (shape) |
| `parallel_lives_has_one_degree_head_under_the_existing_predeterminer` | partly-migrated | 1; PartitiveNounPhrase Reading retired (shape) |

### `partner_with_name_keyword.rs` → `judgments/partner_with_name_keyword.txt` (11 lines)

| Test | Disposition | Detail |
|---|---|---|
| `printed_partner_lines_keep_the_name_parameter_and_both_roundtrip_laws` | partly-migrated | 5; shape |
| `name_payload_uses_the_catalog_class_including_names_outside_partner_pairs` | partly-migrated | 3; shape |
| `name_keyword_rejects_quality_payloads_bare_heads_and_unagreed_markers` | partly-migrated | 1; policing shape |
| `bare_partner_and_the_dashed_variant_keep_their_existing_lexical_owners` | partly-migrated | 2; lexical-v3 |

### `past_participial_premodifiers.rs` → `judgments/past_participial_premodifiers.txt` (9 lines)

| Test | Disposition | Detail |
|---|---|---|
| `ugins_labyrinth_returns_the_exiled_card` | migrated | 1 |
| `florian_plays_the_exiled_card` | migrated | 1 |
| `isochron_scepter_copies_the_exiled_card` | migrated | 1 |
| `erratic_explosion_puts_the_revealed_cards_on_the_bottom` | migrated | 1 |
| `the_princess_takes_flight_returns_the_exiled_card` | migrated | 1 |
| `independently_built_attested_modifiers_preserve_passive_form_and_both_laws` | partly-migrated | 3; shape (hand-built Readings, Trigger frame admission) |
| `lithoform_engine_coordinates_activated_and_triggered` | migrated | 1 |

### `perfect_relative_adverbial_gap.rs` → `judgments/perfect_relative_adverbial_gap.txt` (11 lines)

| Test | Disposition | Detail |
|---|---|---|
| `resolved_witnesses_use_temporal_gaps_and_perfect_auxiliaries` | partly-migrated | 5; count (24/72/3 readings), shape |
| `frequency_and_lexical_have_controls_keep_their_analyses` | migrated | 3 |
| `temporal_relative_constituents_and_captain_america_are_covered` | partly-migrated | 2; count (16 and 24 readings), shape |
| `independently_constructed_temporal_relative_roundtrips_and_requires_licensed_head` | partly-migrated | 1; shape |

### `permanent_noun_premodifier.rs` → `judgments/permanent_noun_premodifier.txt` (5 lines)

| Test | Disposition | Detail |
|---|---|---|
| `cosmic_rebirth` | migrated | 1 |
| `summon_esper_maduin` | migrated | 1 |
| `primevals_glorious_rebirth` | migrated | 1 |
| `divine_gambit` | migrated | 1 |
| `skyshroud_poacher` | migrated | 1 |

### `phase_in_out.rs` → `judgments/phase_in_out.txt` (34 lines)

| Test | Disposition | Detail |
|---|---|---|
| `ticket_witnesses_retain_phase_and_its_obligatory_complement` | partly-migrated | 7; Marker FrameValue/particle lexeme identity retired (shape/lexical-v3) |
| `ten_newly_covered_faces_have_the_selected_particle_analysis` | partly-migrated | 11; particle counts and multi-paragraph split (count/shape) |
| `independently_constructed_inflections_and_particles_roundtrip` | partly-migrated | 6; constructed Readings, frame-position lookup retired (shape) |
| `phase_has_no_bare_frame_or_free_particle_adjunct_analysis` | partly-migrated | 2; `phases`/`phases up`/`rounds out` negatives retired (policing); no Adverb/Out lexeme and Phase frames only In/Out (lexical-v3) |
| `noun_phase_and_existing_verb_controls_keep_their_readings` | partly-migrated | 8; equality with phase-less lexicon retired (engine) |

### `phrasal_keyword_surfaces.rs` → `judgments/phrasal_keyword_surfaces.txt` (11 lines)

| Test | Disposition | Detail |
|---|---|---|
| `phrasal_keyword_lines_have_one_declared_route_and_roundtrip` | partly-migrated | 6; exact Document/KeywordLine Reading and keyword lexeme ids retired (shape/lexical-v3); keyword names declared as atomic terms |
| `doctors_companion_mentions_keep_the_whole_keyword_as_complement` | partly-migrated | 3; hand-built Reading identity retired (shape) |
| `partner_and_the_period_terminated_choose_sentence_keep_their_roles` | partly-migrated | 2; Sentence-category/BareKeyword Reading checks retired (shape) |

### `phrase_coordination.rs` → `judgments/phrase_coordination.txt` (24 lines)

| Test | Disposition | Detail |
|---|---|---|
| `independent_phrase_coordinations_preserve_structure_and_oxford_commas` | migrated | 18 |
| `authentic_shared_modifiers_and_phrase_positions_compose` | partly-migrated | 6; policing |

### `play_with_absolute.rs` → `judgments/play_with_absolute.txt` (16 lines)

| Test | Disposition | Detail |
|---|---|---|
| `telepathy_selects_a_plural_subject_predicate_complement` | migrated | 1 |
| `enduring_renewal_selects_the_clause_in_an_imperative` | migrated | 1 |
| `field_of_dreams_retains_the_subjects_nominal_structure` | migrated | 1 |
| `future_sight_selects_the_clause_in_an_imperative` | migrated | 1 |
| `sen_triplets_selects_the_clause_in_a_coordinated_finite_predicate` | migrated | 1 |
| `unrelated_predicates_transitive_frames_and_the_card_name_keep_their_values` | partly-migrated | 8; engine (reading sets equal under a lexicon with the new frames removed) |
| `independently_selected_complement_preserves_both_roundtrip_laws` | retired | shape: hand-built SelectedPredicate with frame index and round-trip |
| `admission_requires_the_declared_verb_frame_and_marker_licence` | retired | lexical-v3: play declares a with + verbless-clause frame; with-licence feature; engine admission |
| `the_clause_is_neither_a_free_adjunct_nor_an_object_postmodifier` | partly-migrated | 2; shape (no VerblessClause node in standalone PP) |
| `stromgald_spy_selects_the_clause_under_causative_have` | migrated | 1 |

### `plugin_term_definitions.rs` → `judgments/plugin_term_definitions.txt` (11 lines)

| Test | Disposition | Detail |
|---|---|---|
| `plugin_terms_retain_their_definition_owners_and_categories` | retired | lexical-v3: modified/poisoned adjectives; expend/descend event verbs; multikicker keyword; convoke/devour/evolve verbs |
| `attested_plugin_terms_fill_their_declared_roles` | migrated | 9 |
| `independently_constructed_predicate_adjectives_preserve_both_roundtrip_laws` | partly-migrated | 2; shape |

### `polyhedral_die_nouns.rs` → `judgments/polyhedral_die_nouns.txt` (20 lines)

| Test | Disposition | Detail |
|---|---|---|
| `die_nouns_head_the_object_of_roll` | migrated | 11 |
| `independently_constructed_nouns_preserve_singular_and_invariant_plural` | partly-migrated | 6; engine (singular/plural number readings are lexical-v3 inflection facts: d-nouns have invariant plural) |
| `existing_die_controls_keep_their_object_heads` | migrated | 3 (one merged with die_nouns line) |

### `possessive_phrases.rs` → `judgments/possessive_phrases.txt` (13 lines)

| Test | Disposition | Detail |
|---|---|---|
| `possessed_bases_preserve_singular_second_person_before_coordination` | partly-migrated | 7; constructed Readings and plural-possessor admit errors retired (shape) |
| `authentic_possessed_preposition_bases_then_their_coordination` | migrated | 6 |

### `postposed_heavy_object.rs` → `judgments/postposed_heavy_object.txt` (17 lines)

| Test | Disposition | Detail |
|---|---|---|
| `return_witnesses_keep_the_object_in_its_declared_slot` | partly-migrated | 6; PostposedObject FrameValue/slot checks retired (shape) |
| `the_alternation_applies_to_other_declared_frame_shapes` | partly-migrated | 2; FrameValue checks retired (shape) |
| `default_order_retains_ordinary_frame_values` | partly-migrated | 4; absence of PostposedObject retired (shape) |
| `independently_constructed_postposed_object_obeys_admission_and_roundtrip` | retired | shape: hand-built Reading admission/round-trip, SourceFreeObject requirement |
| `a_light_active_object_cannot_postpose` | migrated | 1 |
| `agitator_ant_and_melira_passive_fragments_keep_their_readings_by_value` | partly-migrated | 2; Reading-set equality retired (shape) |
| `tapped_retains_the_existing_depictive_role_before_the_postposed_object` | partly-migrated | 1; DepictivePhrase/FrameValue shape retired |
| `intervening_source_adjunct_keeps_its_role_when_the_object_postposes` | partly-migrated | 1; InterveningPrepositionAdjunct category retired (shape) |

### `postpositive_comparative_property.rs` → `judgments/postpositive_comparative_property.txt` (11 lines)

| Test | Disposition | Detail |
|---|---|---|
| `howlgeist` | migrated | 1 |
| `unliving_psychopath` | migrated | 1 |
| `ensnaring_bridge` | migrated | 1 |
| `immobilizer_eldrazi` | migrated | 1 |
| `kiora_sovereign_of_the_deep` | migrated | 1 |
| `shared_comparative_complement_stays_inside_the_value_slot` | partly-migrated | 2; shape |
| `existing_routes_keep_their_predicate_readings` | migrated | 2 |
| `independent_comparative_property_value_preserves_both_laws` | partly-migrated | 2; shape |

### `pp_admission.rs` → `judgments/pp_admission.txt` (10 lines)

| Test | Disposition | Detail |
|---|---|---|
| `nominal_complement_pp_cannot_become_a_free_clause_or_verb_adjunct` | partly-migrated | 2; shape (admission of hand-built Readings) |
| `pp_adjuncts_and_selected_locatives_keep_their_readings` | partly-migrated | 6; policing (Creatures attack by 2 plus 2.), shape (SelectedExtentPassive) |
| `distribution_preserves_nominal_ambiguity_without_detached_of_adjuncts` | migrated | 2 |

### `predeterminer_all.rs` → `judgments/predeterminer_all.txt` (32 lines)

| Test | Disposition | Detail |
|---|---|---|
| `independently_constructed_universal_quantifiers_keep_both_roundtrip_laws` | partly-migrated | 5; shape |
| `tolarian_winds` | migrated | 1 |
| `awaken_the_erstwhile` | migrated | 1 |
| `robber_fly` | migrated | 1 |
| `endurance` | migrated | 1 |
| `forgotten_creation` | migrated | 1 |
| `existing_routes_keep_their_complete_reading_sets` | partly-migrated | 10; lexical-v3 (reading sets compared to a lexicon without PredeterminerUse), count |
| `number_definiteness_and_stacking_constraints_are_bidirectional` | partly-migrated | 7; policing (twice all / half all / all half stacking negatives), shape (admit-error constructions) |
| `the_host_exports_plural_subject_agreement` | partly-migrated | 5; shape (constructed admit-error clauses) |

### `predicate_terms.rs` → `judgments/predicate_terms.txt` (9 lines)

| Test | Disposition | Detail |
|---|---|---|
| `predicate_and_event_terms_fill_the_attested_grammatical_roles` | migrated | 8 |
| `independently_constructed_outlaw_premodifier_keeps_noun_identity_and_roundtrips` | partly-migrated | 1; engine |

### `preposed_duration_for.rs` → `judgments/preposed_duration_for.txt` (14 lines)

| Test | Disposition | Detail |
|---|---|---|
| `witnesses_have_one_duration_pp_with_the_licensed_preposed_clause_scopes` | partly-migrated | 7; exact scope-pair Reading sets retired (shape/count); Rod clause-scope ambiguity not expressible |
| `conditional_controls_have_only_the_compound_preposition_analysis` | partly-migrated | 2; exact Reading set retired (shape/count) |
| `clause_final_controls_keep_the_duration_complement_analysis` | partly-migrated | 2; Reading scan retired (shape) |
| `duration_complement_does_not_license_nominal_or_conditional_twins` | partly-migrated | 3; `target artifact for as long as ...` NP negative and Reading-type checks retired (policing/shape) |

### `preposed_frequency.rs` → `judgments/preposed_frequency.txt` (10 lines)

| Test | Disposition | Detail |
|---|---|---|
| `witnesses_have_two_ordered_preposed_adjuncts` | partly-migrated | 5; InitialAdverb/InitialPreposition constructors and one-structure count retired (shape/count) |
| `independent_adjunct_composition_preserves_all_readings_and_roundtrips` | partly-migrated | 2; hand-built Reading-set equality and AdverbPhrase negatives retired (shape/policing) |
| `final_frequency_retains_the_interval_over_the_adverb_predicate` | partly-migrated | 3; NominalAdjunctPredicate/AdverbPredicate nesting retired (shape) |

### `preposition_function_licences.rs` → `judgments/preposition_function_licences.txt` (8 lines)

| Test | Disposition | Detail |
|---|---|---|
| `animal_magnetism_has_only_the_two_selected_segment_readings` | partly-migrated | 1; shape |
| `library_of_leng_fixed_preposition_keeps_its_pp_complement` | partly-migrated | 1; shape |
| `hurricane_keyword_pp_remains_a_nominal_modifier` | partly-migrated | 1; shape |
| `raise_dead_keeps_its_selected_source_without_object_attachment` | migrated | 1 |
| `of_shapes_keep_their_nominal_and_predicative_functions` | partly-migrated | 4; shape count |

### `preposition_head_primitives.rs` → nothing migrated

| Test | Disposition | Detail |
|---|---|---|
| `selected_np_preposition_heads_are_independent_roundtrips` | retired | shape: v3-internal SelectedPrepositionHead category and hand-built Readings |

### `preverbal_next.rs` → `judgments/preverbal_next.txt` (11 lines)

| Test | Disposition | Detail |
|---|---|---|
| `galvanic_iteration` | migrated | 1 |
| `the_clone_saga` | migrated | 1 |
| `all_out_assault` | migrated | 1 |
| `dual_strike` | migrated | 1 |
| `attributive_next_and_existing_central_adjuncts_keep_their_readings` | partly-migrated | 6; lexical-v3 (reading sets compared with Adverb/Next removed), count |
| `next_has_only_the_preverbal_licence` | retired | policing: rejects grammatical sentences (Next, you cast a spell / You cast a spell next / You may next cast ...) |
| `independently_assembled_preverbal_adjunct_roundtrips` | partly-migrated | 1; shape |

### `produce_verb.rs` → `judgments/produce_verb.txt` (7 lines)

| Test | Disposition | Detail |
|---|---|---|
| `incubation_druid` | partly-migrated | 1; count (2 readings) |
| `break_the_ice` | partly-migrated | 1; shape (ManaPhrase constituent has no CGEL category) |
| `dictate_of_karametra` | partly-migrated | 1; count (4 readings) |
| `pale_moon` | partly-migrated | 1; count (3 readings) |
| `miraris_wake` | partly-migrated | 1; count (4 readings) |
| `newly_covered_faces_keep_object_gap_relatives` | partly-migrated | 2 new (other 8 cards share texts with lines above or identical, card scope merged to first); count |

### `progressive_object_gap_relative.rs` → `judgments/progressive_object_gap_relative.txt` (24 lines)

| Test | Disposition | Detail |
|---|---|---|
| `ticket_witnesses_link_the_progressive_object_gap_to_the_antecedent` | partly-migrated | 5; Reading scans (ObjectRelativeNominal, no adverbial gap relative) retired (shape) |
| `independent_strong_and_clitic_relatives_obey_both_roundtrip_laws` | partly-migrated | 4; past-participle gap and 're coordination admit errors retired (shape) |
| `strong_phrase_subjects_and_that_relatives_share_the_object_gap_route` | migrated | 4 |
| `subject_gaps_passives_and_ungapped_progressives_keep_their_roles` | partly-migrated | 11; `attacking with` negative retired (policing: grammatical stranded preposition); frame/Reading checks shape |

### `pronominal_other.rs` → `judgments/pronominal_other.txt` (11 lines)

| Test | Disposition | Detail |
|---|---|---|
| `ticket_witnesses_have_other_as_the_noun_head_of_a_definite_np` | partly-migrated | 5; exact DeterminedNounPhrase Reading retired (shape); lexeme CommonNoun/Other retired (lexical-v3) |
| `independent_definite_np_roundtrips_and_preserves_the_lexical_values` | partly-migrated | 1; hand-built Reading and word values retired (shape/lexical-v3) |
| `other_requires_the_selected_determiner_class` | partly-migrated | 1 (*an other); each/any/no/some other and "each other" negatives retired (policing) |
| `other_before_an_overt_noun_remains_an_adjective` | partly-migrated | 4; exact Reading and "the others" negative retired (shape/policing) |
| `selection_survives_nominal_coordination_and_postmodification` | retired | policing: determiner-class restriction negatives on fused-head "other" |

### `proper_names.rs` → `judgments/proper_names.txt` (2 lines)

| Test | Disposition | Detail |
|---|---|---|
| `authentic_card_references_have_independent_proper_name_np_values` | migrated | 2 |

### `property_equal_to.rs` → `judgments/property_equal_to.txt` (10 lines)

| Test | Disposition | Detail |
|---|---|---|
| `master_symmetrist` | migrated | 1 |
| `chalice_of_the_void` | migrated | 1 |
| `sanctum_prelate` | migrated | 1 |
| `ratchet_bomb` | migrated | 1 |
| `aether_vial` | migrated | 1 |
| `predicative_equality_keeps_its_reading` | migrated | 1 |
| `nonproperty_damage_keeps_its_postmodifier` | migrated | 1 |
| `ironclaw_curse_keeps_its_coordinated_governor_readings` | partly-migrated | 1; count (6 readings), shape (ComparativeGovernorHead category) |
| `soul_separator_coordinates_two_property_values` | migrated | 1 |
| `landed_arithmetic_composes_with_equality` | partly-migrated | 1; shape (MeasurePhrase/ArithmeticOperand categories) |
| `independently_composed_equality_property_preserves_both_laws` | retired | shape: hand-built Readings of the equality property |

### `property_pair_each_equal_to.rs` → `judgments/property_pair_each_equal_to.txt` (16 lines)

| Test | Disposition | Detail |
|---|---|---|
| `ticket_witnesses_keep_the_coordinated_predicand_and_internal_adjunct` | partly-migrated | 4; shape (MeasuredAttribute / quantifier-inside-AdjP structure) |
| `equality_without_each_also_predicates_the_coordination` | partly-migrated | 2; shape |
| `quantification_requires_plural_property_agreement_and_one_adjunct` | partly-migrated | 4; policing ("each greater than" negative; standalone AdjP "each equal to ..." negative) |
| `clausal_single_property_and_slash_pair_controls_keep_their_roles` | partly-migrated | 4; policing (power and toughness 3 / ... greater than negatives), shape (PropertyValue) |
| `independently_composed_property_predication_preserves_both_roundtrip_laws` | partly-migrated | 2; shape |

### `purpose_infinitival.rs` → `judgments/purpose_infinitival.txt` (20 lines)

| Test | Disposition | Detail |
|---|---|---|
| `molten_note_admits_the_purpose_of_the_bare_passive` | migrated | 1 |
| `skyrider_elf_keeps_the_purpose_inside_the_mana_postmodifier` | migrated | 1 |
| `infuse_with_the_elements_keeps_the_purpose_inside_the_where_supplement` | migrated | 1 |
| `scaled_nurturer_admits_a_finite_predicate_purpose` | migrated | 1 |
| `angel_of_jubilation_preserves_coordinated_hosts_and_predicates` | migrated | 1 |
| `piracy_focuses_the_purpose_constituent` | migrated | 1 |
| `the_np_postmodifier_contains_the_passive_and_its_purpose` | partly-migrated | 2; shape (host count) |
| `purpose_does_not_supply_a_negative_clause_or_a_noun_relative` | partly-migrated | 3; policing (to not cast, to cast, mana to cast this spell rejected), lexical-v3 (MeasureModifierUse removal) |
| `independently_constructed_purpose_passive_and_focus_preserve_identity` | partly-migrated | 3; engine,shape |
| `purpose_attachment_reads_the_declared_licence` | retired | lexical-v3: PurposeAdjunctLicence feature on spend; removing it blocks purpose attachment |
| `stative_predicates_keep_their_frames_without_purpose_attachment` | partly-migrated | 3; policing (You have a card to cast this spell etc. rejected) |
| `auxiliary_predicates_inherit_the_selected_predicates_purpose_licence` | partly-migrated | 3; shape (ManaSubject/ManaPhrase/ManaPassivePredicate constituents have no CGEL category) |

### `quality_cost_keyword_lines.rs` → `judgments/quality_cost_keyword_lines.txt` (12 lines)

| Test | Disposition | Detail |
|---|---|---|
| `witnesses_select_quality_and_cost_without_a_second_keyword_route` | partly-migrated | 3; exact parameter-set equality retired (shape/count) |
| `independently_constructed_parameters_roundtrip_with_their_lexical_owners` | migrated | 1 (Equip Human; Equip Knight merged above); constructed Reading shape |
| `cost_only_equip_keeps_its_owner_and_quality_cannot_be_empty` | partly-migrated | 3; Craft with.../Equip with Knight negatives retired (wrong: valid Oracle Craft lines asserted unread; policing) |
| `marked_quality_controls_keep_their_selected_parameters` | partly-migrated | 3; KeywordQualityPreposition Reading shape retired |
| `bare_keyword_premodifiers_keep_the_cost_only_owner` | partly-migrated | 2; lexeme-owner identity retired (lexical-v3) |

### `quantified_energy_symbol.rs` → `judgments/quantified_energy_symbol.txt` (10 lines)

| Test | Disposition | Detail |
|---|---|---|
| `independently_constructed_energy_measurement_preserves_both_roundtrip_laws` | partly-migrated | 1; hand-built Reading identity retired (shape) |
| `aethertide_whale_gets_a_cardinal_measurement_with_a_symbol_head` | partly-migrated | 1; MeasuredSymbolNominal shape retired |
| `aethersquall_ancient_pays_a_cardinal_measurement_as_a_cost` | partly-migrated | 1; MeasuredSymbolNominal shape retired |
| `roil_cartographer_pays_a_measurement_beside_a_symbol_cost` | partly-migrated | 1; MeasuredSymbolNominal shape retired |
| `overclocked_electromancer_gets_a_variable_measurement_with_a_symbol_head` | partly-migrated | 1; MeasuredSymbolNominal shape retired |
| `bare_energy_symbols_keep_notation_without_a_measured_nominal_route` | partly-migrated | 5; absence of MeasuredSymbolNominal retired (shape) |
| `an_energy_nominal_requires_a_dependent_to_form_a_noun_phrase` | retired | shape: bare {E} Nominal/NounPhrase category partition |

### `quantified_mana_symbol.rs` → `judgments/quantified_mana_symbol.txt` (13 lines)

| Test | Disposition | Detail |
|---|---|---|
| `independent_quantifiers_have_one_symbol_head_and_preserve_both_laws` | partly-migrated | 6; shape |
| `witnesses_quantify_the_symbol_inside_the_selected_object` | migrated | 5 |
| `declared_mana_class_supplies_the_quantified_paradigm` | retired | lexical-v3: all ManaSymbolUse/SymbolNominalUse=Count symbols (incl hybrid/Phyrexian) accept "six {sym}" |
| `generic_and_bare_notation_keep_their_protected_routes` | partly-migrated | 2; policing |

### `quoted_ability_coordination.rs` → `judgments/quoted_ability_coordination.txt` (11 lines)

| Test | Disposition | Detail |
|---|---|---|
| `card_witnesses_quote_static_triggered_and_activated_abilities` | partly-migrated | 5; shape (QuotedClause/Ability constituents, surfaces with embedded quotes not expressible) |
| `both_coordinates_keep_the_granted_ability_function` | partly-migrated | 3; shape (GrantedAbility coordination categories) |
| `following_adjunct_and_predicate_coordinate_leave_the_ability_unterminated` | partly-migrated | 2; shape (QuotedClause Ability constituent) |
| `complete_and_lowercase_quotations_have_no_ability_twin` | retired | count/shape: exactly-one-quotation-analysis pin on v3 QuotedText category |
| `omitted_sentence_terminal_is_confined_to_the_quotation` | partly-migrated | 1; policing (bare fragment without period, doubled period, mixed terminals), shape (QuotedClause form admission) |
| `independently_composed_unterminated_quotation_roundtrips` | retired | shape: hand-built QuotedClause Reading and round-trip |

### `rather_than_complements.rs` → `judgments/rather_than_complements.txt` (10 lines)

| Test | Disposition | Detail |
|---|---|---|
| `bare_infinitival_witnesses` | migrated | 3 |
| `nominal_witness` | migrated | 1 |
| `gerund_witness` | migrated | 1 |
| `complement_forms_and_markers_must_match` | partly-migrated | 2; policing (mixed gerund/bare, "rather than its mana cost", "rather to pay", standalone rather) |
| `independent_comparisons_roundtrip_with_traversal_identity` | partly-migrated | 3; shape |
| `governor_selection_reads_the_declared_feature` | retired | lexical-v3: rather has MatrixComparisonUse feature licensing rather-than comparison |

### `reading_support.rs` → `judgments/reading_support.txt` (4 lines)

| Test | Disposition | Detail |
|---|---|---|
| `several_constituents_witness_low_of_attachment` | migrated | 1 |
| `seedborn_muse_retains_exactly_two_adjunct_attachments` | migrated | 1 (ambiguity witness with ! on nominal attachment) |
| `complete_constituent_surfaces_keep_source_capitalization` | migrated | 1 |
| `a_substring_crossing_constituent_boundaries_is_rejected` | retired | helper: assert_constituents should_panic on a boundary-crossing surface |
| `constituents_from_different_readings_do_not_jointly_witness_an_analysis` | retired | helper: assert_constituents requires one Reading for all claims |
| `a_matching_surface_at_the_wrong_category_is_rejected` | retired | helper: assert_constituents category check |
| `repeated_surfaces_choose_each_constituents_actual_occurrence` | migrated | 1 |
| `an_unqualified_repeated_surface_is_rejected` | retired | helper: assert_constituents ambiguity of repeated surface |
| `a_chosen_occurrence_cannot_match_a_constituent_elsewhere` | retired | helper: assert_constituent_occurrences occurrence check |

### `relative_clause_adjuncts.rs` → `judgments/relative_clause_adjuncts.txt` (2 lines)

| Test | Disposition | Detail |
|---|---|---|
| `independently_constructed_seedborn_muse_relative_retains_internal_pp` | partly-migrated | 2; constructed Readings, agreement-mismatch admit error retired (shape) |
| `object_gap_pp_adjuncts_preserve_declared_adverbial_use` | retired | shape: gap-host fragments and PrepositionPredicate admission; Seedborn attachment already covered |

### `remove_complements.rs` → `judgments/remove_complements.txt` (8 lines)

| Test | Disposition | Detail |
|---|---|---|
| `spitting_hydra_remove_cost_selects_source` | partly-migrated | 1; FrameValue/SourceComplement shape retired |
| `timberline_ridge_trigger_selects_source` | partly-migrated | 1; FrameValue/SourceComplement shape retired |
| `phyrexian_prowler_remove_cost_selects_source` | partly-migrated | 1; FrameValue/SourceComplement shape retired |
| `voracious_hatchling_triggers_select_source` | partly-migrated | 2; FrameValue/SourceComplement shape retired |
| `perfect_intimidation_imperative_selects_source` | partly-migrated | 1; FrameValue/SourceComplement shape retired |
| `junk_golem_finite_clause_selects_source` | partly-migrated | 1; FrameValue/SourceComplement shape retired |
| `sun_droplet_modal_selects_source` | partly-migrated | 1; FrameValue/SourceComplement shape retired |

### `remove_source_attachment.rs` → `judgments/remove_source_attachment.txt` (17 lines)

| Test | Disposition | Detail |
|---|---|---|
| `storage_counter_removed_this_way_is_a_bare_passive_postmodifier` | migrated | 1 |
| `catti_brie_retains_the_bare_passive_in_the_variable_supplement` | migrated | 1 |
| `white_ward_selects_an_object_without_a_source` | migrated | 1 |
| `spires_and_gustcloak_select_combat_as_source` | migrated | 3 |
| `an_explicit_source_has_one_complement_attachment` | partly-migrated | 2; count |
| `combat_keeps_its_existing_nominal_distributions` | partly-migrated | 1; policing duplicate-of-lexical_compositions |
| `source_controls_keep_their_selected_complements` | partly-migrated | 3; shape |
| `independently_constructed_bare_passive_roundtrips` | partly-migrated | 1; shape |
| `source_inside_a_transparent_object_oblique_is_selected_by_the_verb` | migrated | 1 |
| `a_genitive_possessor_cannot_hide_the_verbs_source` | migrated | 1 |
| `gerund_source_retains_selected_and_matrix_attachments_under_rs12` | migrated | 1 |
| `existing_return_frames_keep_their_head_only_object_licence` | migrated | 1 |

### `restricted_nominals.rs` → `judgments/restricted_nominals.txt` (9 lines)

| Test | Disposition | Detail |
|---|---|---|
| `authentic_bare_status_nominals_keep_modifier_and_head_identity` | partly-migrated | 3; shape (BareStatusNounPhrase constructors, lexeme ids), count (exactly one Reading) |
| `a_status_modifier_keeps_its_existing_determiner_bearing_composition` | partly-migrated | 1; shape (TargetNounPhrase/PremodifiedNominal) |
| `unlicensed_adjectives_do_not_admit_bare_singular_count_nominals` | partly-migrated | 1; policing (kicked creature is licence-list policing); lexical-v3 (kicker adjective declared); shape (admit of hand-built Reading) |
| `authentic_full_genitive_possessor_preserves_internal_structure` | partly-migrated | 4; count (two readings: lexical untap step vs compositional), shape (GenitiveNounPhrase constructors) |

### `ring_tempts_you.rs` → `judgments/ring_tempts_you.txt` (10 lines)

| Test | Disposition | Detail |
|---|---|---|
| `main_coordinated_and_saga_clauses_keep_subject_verb_and_object` | migrated | 3 |
| `trigger_condition_is_an_ordinary_finite_clause` | migrated | 1 |
| `designation_is_a_count_noun_under_a_genitive_determiner_and_in_the_plural` | partly-migrated | 2; lexical-v3 (countability, plural form of Ring-bearer) |
| `independently_constructed_transitive_predicate_roundtrips` | partly-migrated | 1; shape |
| `plugin_provenance_and_regular_inflections_replace_the_keyword_leaf` | retired | lexical-v3: noun Ring + verb tempt (regular forms) from The Ring Tempts You; Ring-bearer count noun; no keyword leaf |
| `neighboring_keyword_actions_and_designation_controls_still_read` | migrated | 3 |

### `same_as_comparative.rs` → `judgments/same_as_comparative.txt` (7 lines)

| Test | Disposition | Detail |
|---|---|---|
| `witnesses_keep_the_complement_inside_the_premodified_nominal` | partly-migrated | 4 + 1 card-scoped; shape, wrong (v3 asserts as that land is NOT a PP; CGEL: as + NP is a PP complement of same) |
| `an_independent_comparative_nominal_roundtrips_with_its_selected_complement` | partly-migrated | 1; engine,shape |
| `the_anaphoric_same_without_as_keeps_its_readings` | partly-migrated | 1; shape (frame index of same) |

### `scalar_cardinals.rs` → `judgments/scalar_cardinals.txt` (19 lines)

| Test | Disposition | Detail |
|---|---|---|
| `pinned_maximum_quantities_keep_the_counted_nominal_structure` | partly-migrated | 4; QuantifiedNounPhrase Reading shape and counts 1/2 retired (shape/count); `up to two` Cardinal negative retired (shape) |
| `independently_constructed_maximum_determiners_preserve_number_and_lexical_owners` | partly-migrated | 2; cost ranking (total_cost) and leaf identity retired (engine/shape) |
| `maximum_quantity_preserves_finite_agreement_and_targeted_genitives` | partly-migrated | 4; constructed genitive shape retired |
| `maximum_determiners_reject_unlicensed_complements_and_number_mismatches` | partly-migrated | 8; constructed admit errors retired (shape) |
| `pinned_clauses_contain_the_maximum_targets` | migrated | 2 |

### `scalar_location_plain_be_and_become.rs` → `judgments/scalar_location_plain_be_and_become.txt` (51 lines)

| Test | Disposition | Detail |
|---|---|---|
| `witnesses_keep_scalar_location_complement_roles` | partly-migrated | 6; ScalarLocationPredicate/BareScalarLocationPredicate categories re-expressed as VerbPhrase/Predicative |
| `auxiliary_class_selects_the_plain_head_and_finite_value_paradigm` | partly-migrated | 32; Reading constructors/frame kinds retired (shape) |
| `independent_values_preserve_both_laws_and_lexical_identity` | partly-migrated | 2; hand-built Readings retired (shape) |
| `non_scalar_subjects_and_wrong_head_forms_stay_excluded` | retired | policing: scalar-subject restriction ("It is 0." etc.); shape: hand-built invalid Readings |
| `existing_copular_predicative_and_passive_routes_keep_their_values` | migrated | 11 |

### `scalar_property_values.rs` → `judgments/scalar_property_values.txt` (24 lines)

| Test | Disposition | Detail |
|---|---|---|
| `property_values_are_dependents_of_the_property_noun` | partly-migrated | 10; count shape |
| `with_property_values_postmodifies_the_target_nominal` | migrated | 6 |
| `scalar_values_reuse_one_coordination_without_determiner_or_clause_overlap` | partly-migrated | 3; shape count |
| `independently_constructed_property_values_preserve_both_roundtrip_laws` | partly-migrated | 5; shape |

### `selected_complement_clusters.rs` → `judgments/selected_complement_clusters.txt` (12 lines)

| Test | Disposition | Detail |
|---|---|---|
| `authentic_recipient_clusters_keep_independent_complete_sentence_structures` | partly-migrated | 3; shape (hand-built full sentence/ability Readings, node and word traversal) |
| `selected_cluster_hosts_require_coordination_and_parallel_selected_markers` | partly-migrated | 1; policing (into-marker variants are benign adjuncts), shape (hand-built single-cluster predicate admission) |
| `avacyn_recipient_stays_with_the_lexical_passive_below_both_auxiliaries` | partly-migrated | 1; shape (PrepositionPredicate admission of hand-built Readings) |
| `all_nine_inherited_recipient_witnesses_have_exact_independent_document_sets` | migrated | 7; chart-item budget is engine (not migrated) |
| `fireslinger_reading_is_preserved_with_fewer_predictions` | migrated | 0; text merged into authentic_recipient_clusters Fireslinger line; engine (chart-item budget) |
| `goblin_artillery_reading_is_preserved_with_fewer_predictions` | migrated | 0; text merged into all_nine Goblin Artillery line; engine (chart-item budget) |
| `forge_devil_reading_is_preserved_with_fewer_predictions` | migrated | 0; text merged into all_nine Forge Devil line; engine (chart-item budget) |

### `selected_frame_passives.rs` → `judgments/selected_frame_passives.txt` (4 lines)

| Test | Disposition | Detail |
|---|---|---|
| `authentic_selected_object_and_passive_gap_preserve_frame_and_marker` | partly-migrated | 2; count (plain and participial syncretic readings), shape (admit-error with To marker) |
| `authentic_retained_object_and_reduced_passives_keep_manner_attachment` | partly-migrated | 2; shape (constructed admit-error passives) |

### `shared_heads.rs` → `judgments/shared_heads.txt` (14 lines)

| Test | Disposition | Detail |
|---|---|---|
| `independent_selected_head_coordination_preserves_flat_arity_and_layered_scope` | retired | shape: atomic selected-head fragments (draw, discard, and exile) have no CGEL phrase; 2 groupings of same-coordinator chain is a canonicalization (count); no-Oxford rejections are policing |
| `independent_shared_objects_and_predicatives_keep_selected_functions` | partly-migrated | 2; engine, policing (become and draw cards: mixed-function shared complement, undecided) |
| `independent_auxiliary_sharing_preserves_complement_form_and_voice` | partly-migrated | 4; shape |
| `selected_heads_reject_crossed_agreement_and_secondary_forms` | partly-migrated | 2; engine (admission of hand-built invalid Readings) |
| `independently_constructed_ordinary_serial_values_require_oxford_comma` | partly-migrated | 4; policing (non-Oxford serial rejected: style rule), shape |
| `independent_shared_quantitative_and_infinitive_complements_preserve_selected_categories` | partly-migrated | 2; shape,engine |

### `shared_preposition_heads.rs` → `judgments/shared_preposition_heads.txt` (4 lines)

| Test | Disposition | Detail |
|---|---|---|
| `authentic_mutate_preposition_heads_share_np_complement` | migrated | 2; constructed Reading roundtrip is shape |
| `shared_preposition_heads_require_oxford_and_intersect_permissions` | partly-migrated | 2; Oxford-comma/nor/mixed-permission negatives retired (policing); realize errors shape |

### `signed_numeral_cost.rs` → `judgments/signed_numeral_cost.txt` (5 lines)

| Test | Disposition | Detail |
|---|---|---|
| `ob_nixilis_the_hate_twisted` | partly-migrated | 1; ActivatedAbility/Cost split retired (shape) |
| `kiora_behemoth_beckoner` | partly-migrated | 1; ActivatedAbility/Cost split retired (shape) |
| `arlinn_voice_of_the_pack` | partly-migrated | 1; ActivatedAbility/Cost split retired (shape) |
| `jiang_yanggu_wildcrafter` | partly-migrated | 1; ActivatedAbility/Cost split retired (shape) |
| `teyo_the_shieldmage` | partly-migrated | 1; ActivatedAbility/Cost split retired (shape) |
| `independently_constructed_signed_costs_preserve_signs_and_traversal` | retired | shape: hand-built Readings, signs and traversal; Cost category has no CGEL counterpart |
| `zero_cost_is_independently_constructible` | retired | shape: hand-built ZeroScalarCost Reading |
| `unsigned_nonzero_and_malformed_notation_are_not_costs` | retired | policing: Cost-category licence negatives on bare numerals; Cost has no CGEL counterpart |

### `singular_you.rs` → `judgments/singular_you.txt` (5 lines)

| Test | Disposition | Detail |
|---|---|---|
| `authentic_you_and_draw_clause_have_only_singular_second_person_values` | partly-migrated | 3; shape count |
| `authentic_coordinated_subject_retains_singular_you_leaf` | migrated | 1 |
| `authentic_discard_clause_has_no_plural_second_person_leaves` | partly-migrated | 1; shape |

### `spelled_cardinal_scalar_measure.rs` → `judgments/spelled_cardinal_scalar_measure.txt` (15 lines)

| Test | Disposition | Detail |
|---|---|---|
| `journey_on_keeps_the_cardinal_as_the_left_operand` | partly-migrated | 1; shape (ArithmeticMeasure left/right operand constructors) |
| `welcome_to_sweettooth_keeps_the_cardinal_as_the_left_operand` | partly-migrated | 1; shape (ArithmeticMeasure operands) |
| `bosss_chauffeur_keeps_the_cardinal_as_the_left_operand` | partly-migrated | 1; shape (ArithmeticMeasure operands) |
| `mind_burst_keeps_the_cardinal_as_the_left_operand` | partly-migrated | 1; shape (ArithmeticMeasure operands) |
| `sea_gate_restoration_and_profts_eidetic_memory_keep_right_operands` | partly-migrated | 2; shape (ArithmeticMeasure operands) |
| `comparisons_keep_the_cardinal_as_the_governors_complement` | partly-migrated | 3; shape (ComparativeGovernorHead/CardinalComparativeComplement) |
| `bare_scalar_values_keep_the_existing_cardinal_and_coordination_routes` | partly-migrated | 2; shape (ScalarValue categories, exact Reading sets) |
| `comparison_quantities_keep_only_the_determiner_route` | partly-migrated | 2; policing ("less than one mana" is grammatical English), shape (QuantifiedNounPhrase constructors) |
| `independently_constructed_cardinal_operands_and_standards_roundtrip` | retired | shape: hand-built Readings, digit-vs-cardinal admission |
| `burn_at_the_stakes_new_comparison_readings_equal_the_digit_control` | partly-migrated | 2; shape (normalising v3 Readings and comparing sets) |

### `starting_with_adjunct.rs` → `judgments/starting_with_adjunct.txt` (13 lines)

| Test | Disposition | Detail |
|---|---|---|
| `horus_heresy_preposes_the_order_adjunct` | migrated | 1 |
| `eureka_preserves_independent_attachments` | migrated | 1 |
| `whims_of_the_fates_preposes_the_order_adjunct` | migrated | 1 |
| `druid_of_purification_places_the_order_after_the_trigger` | migrated | 1 |
| `plague_of_vermin_preposes_the_order_adjunct` | migrated | 1 |
| `independently_constructed_preposition_keeps_its_selected_complement` | partly-migrated | 1; shape |
| `starting_selects_a_with_phrase_and_cannot_skip_its_complement` | partly-migrated | 2; policing (starting from you / starting during your turn are grammatical English) |
| `starting_nominals_keep_the_attributive_adjective` | migrated | 2 |
| `existing_preposition_controls_preserve_their_analyses` | migrated | 3 |

### `superlative_quantifiers.rs` → `judgments/superlative_quantifiers.txt` (19 lines)

| Test | Disposition | Detail |
|---|---|---|
| `the_article_determines_one_shared_superlative_modifier_route` | migrated | 5 |
| `most_keeps_its_bare_determiner_reading_and_declared_number_licence` | partly-migrated | 5; wrong (the most common color rejected: grammatical analytic superlative, deferred) |
| `card_witnesses_use_the_superlative_inside_objects_and_with_complements` | partly-migrated | 7; shape (one superlative route per reading) |
| `independent_pp_attachment_ambiguity_matches_the_fewest_control` | partly-migrated | 0 new (ambiguity witnesses merged into card_witnesses lines); count |
| `witch_king_retains_least_modifying_its_count_scalar_property` | partly-migrated | 2 (1 merged with the_article line); shape |

### `supertype_premodified_bound_keyword.rs` → `judgments/supertype_premodified_bound_keyword.txt` (13 lines)

| Test | Disposition | Detail |
|---|---|---|
| `attested_premodifiers_preserve_lexical_roles_and_both_roundtrip_laws` | partly-migrated | 4; BoundQualityKeyword identity retired (shape/lexical-v3) |
| `gleam_of_resistance_cost_is_the_keyword_parameter` | migrated | 1 |
| `running_mentions_keep_one_keyword_complement_and_lowercase_premodification` | partly-migrated | 3; Reading/word identity retired (shape) |
| `nonsupertype_and_phrasal_modifiers_do_not_enter_the_bound_keyword_route` | partly-migrated | 5; Green/Tapped/Snow and legendary/spacing/missing-cost negatives retired (policing); modifier None checks shape |

### `supplementary_temporal_adjunct.rs` → `judgments/supplementary_temporal_adjunct.txt` (7 lines)

| Test | Disposition | Detail |
|---|---|---|
| `witnesses_supplement_the_clause_outside_the_focused_adjunct` | partly-migrated | 1; exact Supplementation Reading and 5-card duplicate scopes retired (shape) |
| `independently_constructed_supplement_preserves_both_roundtrip_laws` | retired | shape: hand-built Reading round-trip |
| `controls_keep_integrated_adjuncts_and_their_focus` | partly-migrated | 6; FocusedPhrase/Supplementation absence retired (shape) |
| `supplement_licence_does_not_admit_medial_or_focused_supplements` | retired | policing: medial/focused supplement licence negatives; shape: hand-built invalid Reading |

### `targeting_projection.rs` → `judgments/targeting_projection.txt` (25 lines)

| Test | Disposition | Detail |
|---|---|---|
| `bare_target_projection_uses_the_declared_number_boundary` | partly-migrated | 5; shape count |
| `numerals_and_noun_premodifiers_keep_the_targeted_nominal` | migrated | 4 |
| `postmodifier_scopes_above_and_below_targeting_remain_distinct` | partly-migrated | 3; shape count |
| `targeted_genitives_preserve_the_possessor_and_coordinated_head` | migrated | 2 |
| `coordination_keeps_shared_and_local_targeting_scopes` | migrated | 2 |
| `quantifiers_retain_targeting_below_the_determiner` | partly-migrated | 1; shape count |
| `adjective_and_participial_premodifiers_stay_below_targeting` | migrated | 4 |
| `simple_adjective_above_targeting_preserves_the_single_nominal_projection` | migrated | 3 |
| `plural_coordination_retains_wide_and_first_conjunct_targeting` | migrated | 1 |

### `then_sequencing.rs` → `judgments/then_sequencing.txt` (13 lines)

| Test | Disposition | Detail |
|---|---|---|
| `independently_constructed_stitcher_link_preserves_both_roundtrip_laws_and_traversals` | retired | shape: hand-built Coordination Reading, obsolete form index |
| `bare_instruction_pairs_are_independently_constructible_coordinated_clauses` | partly-migrated | 2; shape (hand-built ClauseCoordination and rejected imperative-over-predicate coordination) |
| `connective_marker_requires_punctuation_and_a_clausal_or_verbal_role` | partly-migrated | 2; policing (unpunctuated "then" and ", then" between nominals/adjectives) |
| `blur_coordinates_complete_imperative_predicates` | migrated | 1 |
| `obsessive_stitcher_coordinates_under_the_activated_ability` | migrated | 1 |
| `risky_research_coordinates_keyword_and_lexical_predicates` | migrated | 1 |
| `beneath_the_sands_retains_the_final_link_in_a_flat_serial_list` | partly-migrated | 1; shape (ClauseSeries grouping category) |
| `erode_shares_one_subject_and_modal_across_three_predicates` | partly-migrated | 1; shape (BarePredicate/SecondaryPredicateSeries categories) |
| `vengeful_villagers_keeps_the_second_clauses_own_subject_and_modal` | migrated | 1 |
| `cryptic_annelid_retains_marker_readings_without_an_unmarked_adverb_conjunct` | partly-migrated | 1; shape (Coordinator/Then vs Adverb/Then lexeme counts) |
| `independently_composed_unmarked_adverb_conjunct_fails_admission_and_realization` | retired | shape: admission of a hand-built SerialCoordination |
| `overt_coordinator_preserves_the_second_conjuncts_connective_adverb` | partly-migrated | 1; shape (lexeme-id ownership of And and Adverb/Then) |
| `erode_retains_clause_series_and_shared_subject_readings_for_both_markers` | partly-migrated | 1; count (14 readings, 8/6 split of clause-series vs shared-subject) |

### `trailing_adjunct_height.rs` → `judgments/trailing_adjunct_height.txt` (33 lines)

| Test | Disposition | Detail |
|---|---|---|
| `trailing_adjuncts_keep_their_predicate_scope` | migrated | 8 |
| `preposed_and_relative_controls_keep_their_readings` | migrated | 2 |
| `skyrider_patrol_keeps_every_clause_coordination_attachment` | partly-migrated | 1; shape (exact Reading-set equality with constructed attachments) |
| `nissa_keeps_every_adjunct_above_the_modal` | partly-migrated | 1; shape (exact Reading-set equality) |
| `skyrider_patrol_keeps_stacked_clause_coordination_adjuncts` | partly-migrated | 1; shape (exact Reading-set equality) |
| `rulings_keep_every_previously_lost_face_reading` | partly-migrated | 19; shape (multi-line faces split into single-line ability lines; keyword-only lines dropped; check_height tree invariant) |
| `existential_predicate_carries_the_trailing_preposition` | partly-migrated | 1; shape (exact Reading-set equality) |

### `transitive_turn_face.rs` → `judgments/transitive_turn_face.txt` (10 lines)

| Test | Disposition | Detail |
|---|---|---|
| `liege_of_the_axe` | partly-migrated | 1; count (1 reading contradicts be+participle keeping both readings), shape |
| `shaleskin_plower` | partly-migrated | 1; count, shape |
| `daru_mender` | partly-migrated | 1; count, shape |
| `aven_farseer` | partly-migrated | 1; count, shape |
| `zimone_mystery_unraveler` | partly-migrated | 1; count, shape |
| `active_directions_keep_selected_objects` | partly-migrated | 2; count, shape |
| `vesuvan_shapeshifter_passive_face_down` | partly-migrated | 1; count, shape |
| `unattested_intransitive_frame_is_retired` | retired | policing: turns face up is grammatical English, rejected for lack of attestation |
| `independently_constructed_active_and_passive_frames_roundtrip` | partly-migrated | 2; engine,shape |
| `invariant_frame_markers_reject_plural_faces` | retired | policing: face up/down frozen marker; turned faces up is grammatical |

### `typecycling_bound_suffix.rs` → `judgments/typecycling_bound_suffix.txt` (20 lines)

| Test | Disposition | Detail |
|---|---|---|
| `canyon_crawler_swampcycling` | partly-migrated | 1; Document count 1 and word identity retired (count/shape) |
| `shepherding_spirits_plainscycling` | partly-migrated | 1; Document count 1 and word identity retired (count/shape) |
| `step_through_wizardcycling` | partly-migrated | 1; Document count 1 and word identity retired (count/shape) |
| `generous_ent_forestcycling` | partly-migrated | 1; Document count 1 and word identity retired (count/shape) |
| `other_attested_classes_keep_their_declared_owners` | partly-migrated | 6; stem lexeme owners retired (lexical-v3) |
| `stem_class_binding_and_mandatory_cost_constrain_admission` | partly-migrated | 7; `Swamp cycling {2}` and bare `Swampcycling` negatives retired (policing); constructed admit error shape |
| `gleam_of_resistance_reads_the_ruled_supertype_premodifier` | migrated | 0 new (covered by supertype_premodified_bound_keyword) |
| `plain_cycling_keeps_its_free_lexical_owner` | partly-migrated | 3; Cycling owner is free lexeme, no standalone typecycling lexeme (lexical-v3); count 1 retired |

### `up_to_that_many.rs` → `judgments/up_to_that_many.txt` (19 lines)

| Test | Disposition | Detail |
|---|---|---|
| `witnesses_use_a_bounded_degree_determiner_in_each_nominal_position` | partly-migrated | 5; QuantitativeDegreePrepositionPhrase/DegreePrepositionPhrase categories and bounded() Reading retired (shape) |
| `independent_bounded_degree_values_preserve_both_roundtrip_laws_and_traversal` | partly-migrated | 2 (in the NounPhrase lines); hand-built Readings retired (shape) |
| `bounded_degree_has_no_goal_pp_or_demonstrative_determiner_route` | partly-migrated | 1 (!PrepositionPhrase); "many target creatures" Nominal negative retired (policing) |
| `bounded_degree_preserves_head_selection_and_subject_agreement` | partly-migrated | 7; hand-built invalid Reading admission retired (shape) |
| `unbounded_degree_and_cardinal_controls_keep_their_constituent_roles` | migrated | 5 |

### `verbless_if_complement.rs` → `judgments/verbless_if_complement.txt` (7 lines)

| Test | Disposition | Detail |
|---|---|---|
| `ares_active_finite_predicate` | migrated | 1 |
| `jurin_passive_modal_predicate` | migrated | 1 |
| `tangle_angler_activated_ability` | migrated | 1 |
| `alluring_siren_selected_object` | migrated | 1 |
| `hustle_coordinated_predicates` | migrated | 1 |
| `independently_constructed_complement_keeps_identity_and_leaf_order` | partly-migrated | 1; shape |
| `both_lexical_licences_and_the_adjunct_function_are_required` | retired | policing: VerblessClauseComplement/VerblessPredicateUse/PrepositionFunctionLicence features |
| `verbless_complement_does_not_supply_nominal_or_generic_adjective_use` | partly-migrated | 1; policing |

### `verbless_state_predicate_class.rs` → `judgments/verbless_state_predicate_class.txt` (40 lines)

| Test | Disposition | Detail |
|---|---|---|
| `state_derivations_supply_one_verbless_route_and_preserve_control_ambiguity` | partly-migrated | 17; wrong (while other / while same: attributive-only adjectives are not predicative), shape (hand-built PrepositionPredicate adjuncts, exact Reading sets) |
| `independently_composed_state_complements_keep_existing_owners_and_both_laws` | partly-migrated | 22; wrong (while other / while same), shape (hand-built Readings, form indices) |
| `haunted_adds_the_declared_state_complement_beside_the_preserved_passive` | migrated | 1 |

### `verbless_while_adjunct.rs` → `judgments/verbless_while_adjunct.txt` (11 lines)

| Test | Disposition | Detail |
|---|---|---|
| `witnesses_attach_one_verbless_route_to_the_attack_predicate` | partly-migrated | 4; count (exactly one route, equality with if-control readings) |
| `independent_complement_keeps_owner_roundtrips_and_lexical_licences` | partly-migrated | 2; lexical-v3 (VerblessClauseComplement / VerblessPredicateUse / PrepositionFunctionLicence), policing (creature while saddled NP; LocativeComplement), shape |
| `finite_while_copular_saddled_and_if_able_readings_are_preserved` | partly-migrated | 5; lexical-v3 (reading sets compared with licence features removed), count |

### `vocabulary_batch_1.rs` → `judgments/vocabulary_batch_1.txt` (72 lines)

| Test | Disposition | Detail |
|---|---|---|
| `vocabulary_only_witnesses_preserve_constituents` | partly-migrated | 72; lexical-v3 (the vocabulary additions themselves), policing (This creature moves. rejected: intransitive move), shape (Name 'Cadet' constituent has no CGEL category) |

### `vocabulary_batch_2.rs` → `judgments/vocabulary_batch_2.txt` (172 lines)

| Test | Disposition | Detail |
|---|---|---|
| `vocabulary_only_witnesses_preserve_roles` | migrated | 172 (199 witnesses; same-text witnesses on two cards merged); lexical-v3 coverage: vocabulary words checked |

### `waterbend_cost_verb.rs` → `judgments/waterbend_cost_verb.txt` (8 lines)

| Test | Disposition | Detail |
|---|---|---|
| `independently_constructed_action_cost_has_a_cost_symbol_complement` | retired | shape: hand-built Cost/ActionCost Reading (Cost has no CGEL counterpart) |
| `oracle_cost_and_predicate_uses_keep_the_selected_complement` | partly-migrated | 7; ActionCost-vs-predicate role and CostSymbols category retired (shape) |
| `ordinary_paradigm_is_declared_and_roundtrips` | retired | lexical-v3: waterbend keyword action has full regular paradigm (waterbend/waterbends/waterbending/waterbended preterite+participle) |
| `bare_frame_is_retained_and_the_fixed_term_route_is_retired` | partly-migrated | 1; NounPhrase and "Waterbend a card" negatives retired (policing); hand-built Reading (shape) |

### `where_variable_clause.rs` → `judgments/where_variable_clause.txt` (16 lines)

| Test | Disposition | Detail |
|---|---|---|
| `chameleon_colossus_defines_its_adjustment` | migrated | 1 |
| `wild_beastmaster_retains_the_trigger_and_effect_clause` | migrated | 1 |
| `hemosymbic_mite_defines_a_targeted_adjustment` | migrated | 1 |
| `elenda_retains_the_variable_token_quantity` | migrated | 1 |
| `tip_the_scales_defines_a_negative_adjustment` | migrated | 1 |
| `firebending_student_supplements_the_keyword_label_once` | migrated | 1 |
| `supplementary_licence_excludes_integrated_and_locative_routes` | partly-migrated | 6; policing count |
| `independently_constructed_keyword_supplement_roundtrips_structurally` | retired | shape: Supplementation tree; text duplicates firebending_student line |
| `independent_supplement_rejects_an_unlicensed_preposition` | retired | shape: admission of Supplementation with a wrong preposition head |
| `kraul_harpooner_keeps_sequencing_outside_the_medial_definition` | partly-migrated | 1; shape multi-line text reduced to one line |
| `independently_composed_medial_supplement_retains_its_anchor_and_continuation` | retired | shape: MedialSupplementation tree; text covered by kraul line |
| `sardian_cliffstomper_keeps_coordination_in_the_anchor_condition` | partly-migrated | 1; count shape |
| `coordinated_definitions_retain_general_coordination` | migrated | 1 |
| `neverending_torment_keeps_general_coordination_outside_the_medial_definition` | partly-migrated | 1; shape multi-line text reduced to one line |

### `your_choice_of_keywords.rs` → `judgments/your_choice_of_keywords.txt` (17 lines)

| Test | Disposition | Detail |
|---|---|---|
| `independently_constructed_choice_retains_the_whole_keyword_complement` | partly-migrated | 2; shape (hand-built possessive NP Reading) |
| `gain_and_lose_take_choice_as_object_with_keywords_inside_of` | partly-migrated | 5; shape (SelectedPredicate/KeywordComplementPreposition constructors) |
| `declared_nominal_frame_is_required_and_parameterized_keywords_are_compositional` | partly-migrated | 2; lexical-v3 (choice declares a nominal frame taking of + keywords), policing (a spell of flying / a creature of deathtouch / of flying are not clearly ungrammatical) |
| `controls_preserve_their_complete_reading_sets` | partly-migrated | 8; engine (reading sets equal under a lexicon without the keyword frame) |

## Parser-engine tests

`packed.rs` and `slice.rs` test v3's engine over synthetic grammars. Each
property is (a) already asserted by a `deckmaste_syntax` test, (b) part of
the engine contract and owed by `english-v4-syntax-engine-obligations`, or (c)
specific to v3's design.

### `crates/deckmaste_english_v3/tests/suite/packed.rs`

| Test | Class | Property |
|---|---|---|
| `homographs_are_readings_and_duplicate_productions_are_not` | b | Lexical homographs yield distinct Readings, while two identical productions deriving the same tree collapse to one Reading (derivation_count still counts both). |
| `multiword_and_bound_overlaps_keep_distinct_leaf_structure` | b | A multiword term declared as one lexeme is atomic and needs exact separators, distinct from the separate-word parse of the same text (bound-morpheme half is lexical analysis). |
| `nullable_completion_handles_both_discovery_orders_and_late_growth` | b | Nullable nonterminals are completed in any discovery order, so every empty-span derivation is found even when nullability grows late. |
| `ambiguous_attachment_has_catalan_many_readings_in_a_small_forest` | a | n-way attachment ambiguity yields Catalan-many Readings packed in a forest of polynomial size; asserted by engine.rs pp_chain_packs_catalan_many_trees_in_a_small_forest and best_readings_do_not_enumerate_every_attachment. |
| `partial_histories_do_not_enumerate_homograph_products` | b | Incomplete items pack lexical homograph choices, so the item count grows linearly with input length rather than with the product of homograph choices. |
| `incomplete_prefixes_pack_products_of_distinct_completed_summaries` | b | Constituents with equal summaries are packed while distinct summaries stay distinct, and incomplete prefixes share histories referencing different child nodes. |
| `agreement_keeps_correlated_alternatives_and_rejects_crossed_features` | b | Sibling constituents with feature alternatives stay correlated under an agreement equation: each Reading takes one alternative per child and crossed feature combinations are rejected (engine.rs has no Agree equation test). |
| `open_dependencies_survive_packing_until_the_governing_parent` | b | A dependency left open by a descendant survives packing and is discharged only at the governing parent, and an unclosed constituent is not a root (v4 analogue is Gap only; frame, sharing and recoverability are v3 summary kinds). |
| `lexical_frame_is_carried_to_its_parent` | c | A lexeme's frame is carried up as an open-frame summary a parent may require; v3-specific because v4 derives frames at schema level and records selected_frame (its counterpart is asserted by frames.rs and amount_frames.rs, not by this row). |
| `cycles_are_reported_without_losing_finite_siblings` | b | A unit cycle does not hide finite sibling derivations of the same constituent (the cycle-reporting API is v3-specific; v4 silently drops cyclic derivations). |
| `independent_values_preserve_features_variants_and_unicode_surface` | b | Every declared lexical value, including spelling variants and non-ASCII surfaces, is a Reading of its own realization (law 2) and parses back to exactly that value. |
| `local_features_survive_equal_future_admissibility_summaries` | c | Production-local features that never reach the parent's summary still distinguish otherwise-equal Readings; v3-specific because it relies on the Materializer state argument, while v4 constants bind the mother's own features and there is no hidden per-production state. |
| `lexical_use_choices_survive_when_the_parent_forgets_them` | b | A lexical use choice (count vs mass) that the parent does not inspect still yields distinct Readings, so lexical identity is kept in the tree. |
| `duplicate_empty_derivations_are_one_reading_and_empty_literals_are_leaves` | b | Duplicate productions deriving an empty span collapse to one Reading, and an empty literal is kept as a Reading leaf rather than dropped. |
| `real_lexical_homographs_are_not_selected_by_the_chart` | b | The chart does not pick among lexical homographs (cast as noun or verb, one as numeral, determinative or noun): each analysis reaches every production that accepts its category. |
| `all_attachment_readings_have_exact_leaf_ownership_and_traversal_order` | a | Every Reading of an attachment ambiguity realizes to exactly the input text, so its leaves cover the input once in order (law 1); span bookkeeping is v3 API; asserted by engine.rs readings() helper (realize == text). |
| `malformed_occurrences_report_input_errors` | c | Inconsistent lexical occurrences (bad span, undeclared variant, altered tokens) are rejected as input errors; v3-specific because v4 parse takes text only and runs analysis internally, so there is no external occurrence input to validate. |
| `materializer_errors_are_visible_as_internal_failures` | c | A failing build callback surfaces as a materializer error rather than being swallowed; v3-specific because v4 has no user-supplied Materializer and builds Readings internally. |
| `packed_segments_union_summaries_without_expanding_the_document_product` | b | Per-unit alternatives of a segmented document are unioned per span, and per-unit readings are counted without expanding the product of units (the census per-unit counting contract; segments() itself is v3 API). |
| `packed_segments_stop_before_a_quoted_nested_document` | b | Unit boundaries are top-level: a newline inside a quoted nested document does not split a unit. |
| `packed_segments_report_conflicting_root_segmentations` | b | Two derivations that segment the same input into units differently are reported as ambiguous rather than merged into one unit count. |
| `packed_segments_report_cycles_outside_the_segment_category` | c | A cycle in the document category is reported when segmenting another category; v3-specific because v4 has no cycle reporting and the ADR is silent on cycles. |
| `prediction_skips_only_terminals_that_the_scanner_cannot_match` | c | Prediction creates items only for terminals the input can match (predicted_items metric); v3-specific chart metric, while the v4 contract requires only correct recognition, which engine.rs parse tests cover. |
| `prediction_retains_empty_literals_and_nullable_first_nonterminals` | b | A nullable nonterminal or an empty literal at the start of a production is still predicted and its derivations retained. |

### `crates/deckmaste_english_v3/tests/suite/slice.rs`

| Test | Class | Property |
|---|---|---|
| `interacting_tense_agreement_complement_and_attachment_readings_are_exact` | a | Agreement filtering (tense, number) and both PP attachment sites compose into exactly the endorsed Reading set; each half is asserted by engine.rs pp_chain_packs_catalan_many_trees_in_a_small_forest and feature_requirements_reject_disagreement, and the joint exact-set claim is grammar-level. |
| `homographs_multiword_overlap_and_duplicate_derivations_keep_their_identities` | b | Homograph Readings (cast, one) and a multiword term keep their own identities, and a duplicate derivation is counted in derivations but collapses to one Reading (derivations 2, duplicates 1). |
| `wrong_agreement_frames_and_jointly_invalid_bundles_are_rejected` | b | Agreement is checked on a lexeme's joint alternative, never on the marginal union of its alternatives: a jointly invalid Reading is rejected by parse and by admission. |
| `every_declared_finite_bundle_spelling_and_capitalization_roundtrips_independently` | b | Law 2 holds for every declared finite lexical value, including spelling variants and capitalization: each is admitted and parses back to itself (family of 112). |
| `all_nominal_values_and_surface_orders_preserve_lexical_and_structural_traversal` | b | Laws 1 and 2 hold for every declared nominal value (72), and node and word traversal follows surface order (traversal API is v3). |
| `finite_document_family_checks_both_laws_and_boundary_rejection` | b | Both round-trip laws hold over a whole document family (0 to 4 sentences, with and without header), and malformed sentence boundaries are not Readings. |
| `packed_inspection_and_bounded_requests_do_not_enumerate_the_document` | a | Packed size stays bounded rather than product-sized and the first Readings are obtained without building the product; asserted by engine.rs best_readings_do_not_enumerate_every_attachment and pp_chain_packs_catalan_many_trees_in_a_small_forest (write_packed and document scaling are v3 API). |
| `recognition_consumes_supplied_alternatives_without_rescanning` | b | The chart recognizes over the lexical analysis it is given: removing an alternative removes exactly the Readings that need it, with no production-directed rescanning (ADR section 1; the explicit injection seam is v3 API). |
| `finite_complement_families_cover_every_nominal_and_both_optional_attachment_sites` | b | Laws 1 and 2 hold for every complement family: basic NPs (180), PPs over every preposition (360), and imperatives with both optional attachment sites (720). |

## Lexical facts

Facts asserted by `lexical-v3` retirements, checked against
`deckmaste_lexical_source_v4`: `held-and-tested`, `held-untested`,
`superseded` (v4 deliberately differs) or `gap`.

| v3 test | Fact | Status | Evidence |
|---|---|---|---|
| `analytic_not_after_be.rs::licences_and_positive_auxiliary_status_control_admission` | not is the named predicate negator (identity-gated word) | held-untested | crates/deckmaste_lexical_source_v4/lexicon/english/adverbs.ron:31 Central+Phrasal; closed-class so a grammar may name it; crates/deckmaste_english_v4/judgments/analytic_not_after_be.txt (pending: COMPLETED_STRATUM=None) |
| `analytic_not_after_be.rs::licences_and_positive_auxiliary_status_control_admission` | contracted 's/'re is a positive auxiliary be (not negative): 'It's not your turn' but 'isn't not' rejected | held-untested | clitic be crates/deckmaste_lexical_source_v4/lexicon/english/closed_class.ron:151 (Copular), negative be lexeme crates/deckmaste_lexical_source_v4/lexicon/english/closed_class.ron:122 negative: true; lexical analysis "it's" -> verb:'re Present; crates/deckmaste_english_v4/judgments/analytic_not_after_be.txt (pending: COMPLETED_STRATUM=None) |
| `articles.rs::articles_reject_wrong_onset_plural_and_mass_uses` | mana is a mass noun (so 'a mana' fails) and plural 'creatures'/'artifacts' reject a/an | held-untested | crates/deckmaste_lexical_source_v4/lexicon/english/nouns.ron:83 countability [Mass]; plural/singular agreement of `a` is grammar (a has no number feature) |
| `articles.rs::articles_reject_wrong_onset_plural_and_mass_uses` | wrong-onset article is rejected (an creature, a artifact, a unusual creature, ...) | held-untested | article_onset per spelling is declared on the determinative (closed_class.ron a/an) and exposed by surface_features; mismatch rejection is grammar-side (pending judgment articles.txt) |
| `articles.rs::homographs_keep_pronunciation_correlated_with_their_article_variant` | two homographs with different pronunciations keep their own onset (xenic: an vs a) | held-and-tested | crates/deckmaste_lexical/tests/pronunciation.rs::normalized_pronunciation_respects_overrides_spelling_variants_and_numerals (override vs default xenic); the v3 test was a fixture homograph, not a real word |
| `articles.rs::independent_article_values_preserve_onset_variants_and_modifier_attachment` | the indefinite article has spellings a/an with onsets Consonant/Vowel | held-untested | crates/deckmaste_lexical_source_v4/lexicon/english/closed_class.ron:7; crates/deckmaste_lexical/tests/pronunciation.rs::invalid_pronunciation_declarations_fail_loading (article onsets must cover every spelling) and normalized_pronunciation_respects_overrides_spelling_variants_and_numerals |
| `articles.rs::independent_article_values_preserve_onset_variants_and_modifier_attachment` | onset of the following pronounced word chooses a/an: creature/artifact, unusual (an), uniform (a), hourlong (an), one-eyed (a), capitalized A/An | held-untested | orthographic default onset rules (crates/deckmaste_lexical/src/pronunciation.rs:7-70: hour*, uni*, one, ...) handle all of these; pronunciation.rs tests cover xenic/heir/numerals/island/forest but not unusual/uniform/hourlong/one-eyed specifically; capitalization variants covered by analysis.rs::initial_variants_do_not_case_fold_exact_catalog_entries |
| `articles.rs::independently_supplied_names_preserve_exact_identity_and_case` | a catalog Name is case-exact: 'Prisoner Zero' does not match 'prisoner zero' | held-and-tested | crates/deckmaste_lexical/tests/analysis.rs::initial_variants_do_not_case_fold_exact_catalog_entries |
| `as_though_content_clauses.rs::irrealis_is_non_tensed_and_does_not_relax_ordinary_agreement` | be has an Irrealis form 'were'/'weren't' distinct from Preterite; irrealis carries no tense | held-untested | crates/deckmaste_lexical_source_v4/lexicon/english/closed_class.ron:113, crates/deckmaste_lexical_source_v4/lexicon/english/closed_class.ron:122; lexical analysis 'weren't' -> verb:aren't Preterite and Irrealis; the no-tense constraint (loader rejects Irrealis with tense) is not asserted in v4 tests; crates/deckmaste_english_v4/judgments/as_though_content_clauses.txt (pending: COMPLETED_STRATUM=None) |
| `as_though_content_clauses.rs::named_irrealis_word_controls_admission_and_parsing` | as though is a declared (named) preposition lexeme taking a clause | held-untested | crates/deckmaste_lexical_source_v4/lexicon/english/prepositions.ron:32 FiniteClause, hosts Predicate; closed-class, so nameable by the grammar (identity-gating = Closed Word rule); crates/deckmaste_english_v4/judgments/as_though_content_clauses.txt (pending: COMPLETED_STRATUM=None) |
| `cause_catenative.rs::the_frame_licence_preserves_controls_and_requires_the_infinitival` | cause takes object + to-infinitive (cause abilities to trigger) | held-untested | crates/deckmaste_lexical_source_v4/lexicon/english/verbs.ron:21 ComplexCatenative(ToInf); `to` is a subordinator (crates/deckmaste_lexical_source_v4/lexicon/english/closed_class.ron:107); crates/deckmaste_english_v4/judgments/cause_catenative.txt (pending: COMPLETED_STRATUM=None) |
| `cause_catenative.rs::the_frame_licence_preserves_controls_and_requires_the_infinitival` | controls: have (bare-inf complex catenative), choose (to-inf catenative), trigger intransitive | held-untested | crates/deckmaste_lexical_source_v4/lexicon/english/closed_class.ron:137 ComplexCatenative(BareInf); crates/deckmaste_lexical_source_v4/lexicon/english/verbs.ron:23 Catenative(ToInf); crates/deckmaste_lexical_source_v4/lexicon/english/verbs.ron:132 |
| `champion_keyword_line.rs::champion_article_keeps_the_subject_role_and_both_roundtrip_laws` | the creature types Champion can name (Elemental, Elf, Kithkin, Merfolk, Faerie, Goblin) and `creature` are declared subtypes/types | held-and-tested | crates/deckmaste_lexical_source_v4/tests/cr_coverage.rs::subtypes_by_family, crates/deckmaste_lexical_source_v4/tests/cr_coverage.rs::card_types_and_supertypes |
| `champion_keyword_line.rs::champion_article_keeps_the_subject_role_and_both_roundtrip_laws` | champion also a verb ('champion a Faerie') | held-untested | crates/deckmaste_lexical_source_v4/catalogs/keyword_abilities.ron:148 |
| `champion_keyword_line.rs::champion_article_keeps_the_subject_role_and_both_roundtrip_laws` | Champion takes a quality parameter (Champion an Elf / a creature) | held-untested | crates/deckmaste_lexical_source_v4/catalogs/keyword_abilities.ron:146 Quality(None); cr_coverage::keyword_abilities checks headings only |
| `clitic_contractions.rs::clitic_be_retains_all_full_be_frames_and_have_retains_the_perfect_use` | contracted be ('s, 're, 'm) is a bound auxiliary with the Copular frames of full be | held-untested | crates/deckmaste_lexical_source_v4/lexicon/english/closed_class.ron:151 binding: Suffix, frames: [Copular] (same as be crates/deckmaste_lexical_source_v4/lexicon/english/closed_class.ron:112); lexical analysis "you're", "I'm", "it's"; crates/deckmaste_english_v4/judgments/clitic_contractions.txt (pending: COMPLETED_STRATUM=None) |
| `clitic_contractions.rs::clitic_be_retains_all_full_be_frames_and_have_retains_the_perfect_use` | contracted have ('ve, 's) carries only the perfect-auxiliary use, no main-verb frames | held-untested | crates/deckmaste_lexical_source_v4/lexicon/english/closed_class.ron:145 declares no frames (README: only the perfect auxiliary has no frame); lexical analysis "you've"; crates/deckmaste_english_v4/judgments/clitic_contractions.txt (pending: COMPLETED_STRATUM=None) |
| `counter_compounds.rs::authentic_additional_numeric_counter_kinds_keep_signed_zero_notation` | +1/+0, -0/-1, -0/-2, -1/-0 counter kinds keep their written sign notation | held-untested | crates/deckmaste_lexical_source_v4/catalogs/counters.ron:8, crates/deckmaste_lexical_source_v4/catalogs/counters.ron:12, crates/deckmaste_lexical_source_v4/catalogs/counters.ron:13, crates/deckmaste_lexical_source_v4/catalogs/counters.ron:14 ([CR#122.1a]); the generic mechanism is tested (crates/deckmaste_lexical/tests/measured_compounds.rs::a_declared_signed_zero_preserves_its_written_sign_in_both_noun_forms); named_counter_kinds covers only +1/+1 and -1/-1 from [CR#122.1a] |
| `counter_compounds.rs::authentic_counter_compounds_preserve_macro_owner_and_noun_inflection` | charge counter is a compound counter noun with plural 'charge counters' | held-untested | crates/deckmaste_lexical_source_v4/catalogs/counters.ron:21 ([CR#702.44a]); lexical analysis 'charge counters' -> counter:charge Plural; cr_coverage::named_counter_kinds covers only the [CR#122.1] kinds, not charge |
| `data_batch_2.rs::declarations_fill_attested_roles_and_preserve_both_roundtrip_laws` | end is a verb with Intransitive (steps end) and Monotransitive (End the turn) frames | held-untested | crates/deckmaste_lexical_source_v4/lexicon/english/verbs.ron:51; crates/deckmaste_english_v4/judgments/data_batch_2.txt (pending: COMPLETED_STRATUM=None) |
| `data_batch_2.rs::declarations_fill_attested_roles_and_preserve_both_roundtrip_laws` | flip is a count noun ('lose the flip'), life total a multiword noun | held-untested | crates/deckmaste_lexical_source_v4/lexicon/english/nouns.ron:66, crates/deckmaste_lexical_source_v4/lexicon/english/nouns.ron:156 |
| `data_batch_2.rs::declarations_fill_attested_roles_and_preserve_both_roundtrip_laws` | mana ability is a compound-noun lexeme (compound-noun form of mana + ability) | superseded | v4 declares mana (crates/deckmaste_lexical_source_v4/lexicon/english/nouns.ron:83) and ability (crates/deckmaste_lexical_source_v4/lexicon/english/nouns.ron:8) separately; the compound is a general noun-premodifier construction; crates/deckmaste_english_v4/judgments/data_batch_2.txt (pending: COMPLETED_STRATUM=None) 'Nominal "mana ability"' |
| `data_batch_2.rs::declarations_fill_attested_roles_and_preserve_both_roundtrip_laws` | remove takes object + from-source complement; transform takes Intransitive and into complement; surveil is a verb | held-untested | crates/deckmaste_lexical_source_v4/lexicon/english/verbs.ron:103; crates/deckmaste_lexical_source_v4/catalogs/keyword_actions.ron:91 (Intransitive, Prepositional([into])); crates/deckmaste_lexical_source_v4/catalogs/keyword_actions.ron:83 |
| `data_batch_2.rs::declarations_fill_attested_roles_and_preserve_both_roundtrip_laws` | put takes the intransitive preposition 'back' as complement ('put them back', 'put the rest back on top of your library') and 'there' ('It was put there') | gap | crates/deckmaste_lexical_source_v4/lexicon/english/verbs.ron:98 is TransitiveWithPP([into,on,onto]) only, so neither back nor there is a selected complement of put (a PP complement must be named by the frame, see deckmaste_syntax/tests/intransitive_prepositions.rs); back and there exist only as ComplementOnly intransitive prepositions (crates/deckmaste_lexical_source_v4/lexicon/english/prepositions.ron:40, crates/deckmaste_lexical_source_v4/lexicon/english/prepositions.ron:158); crates/deckmaste_english_v4/judgments/data_batch_2.txt (pending: COMPLETED_STRATUM=None) expects 'PrepositionPhrase "back" as Complement of put' |
| `data_batch_2.rs::declarations_fill_attested_roles_and_preserve_both_roundtrip_laws` | 'this game' / 'any time' nominal adjuncts: game noun, any determinative, time countable/mass noun | held-untested | crates/deckmaste_lexical_source_v4/lexicon/english/nouns.ron:69, crates/deckmaste_lexical_source_v4/lexicon/english/nouns.ron:128 [Count,Mass], crates/deckmaste_lexical_source_v4/lexicon/english/closed_class.ron:10 |
| `data_batch_2.rs::declarations_fill_attested_roles_and_preserve_both_roundtrip_laws` | bare 'combat' has no NP reading (combat in 'removal from combat' is a mass noun object of from) | superseded | v4 declares combat as [Count, Mass] (nouns.ron:31): 'Remove it from combat' is expected to parse with combat as NP complement of from (crates/deckmaste_english_v4/judgments/data_batch_2.txt (pending: COMPLETED_STRATUM=None)); the v3 negative was policing |
| `designation_categories.rs::independently_constructed_nouns_and_adjectives_roundtrip_with_their_owners` | commander (plural commanders), monarch, initiative, city's blessing are designation nouns | held-untested | commander crates/deckmaste_lexical_source_v4/catalogs/designations.ron:48, monarch crates/deckmaste_lexical_source_v4/catalogs/designations.ron:37, initiative crates/deckmaste_lexical_source_v4/catalogs/designations.ron:38, city's blessing crates/deckmaste_lexical_source_v4/catalogs/designations.ron:15 (Mass); lexical analysis commanders -> designation:commander Plural; designations are not in any cr_coverage test |
| `designation_categories.rs::independently_constructed_nouns_and_adjectives_roundtrip_with_their_owners` | prepared, saddled, monstrous, renowned, solved are designation adjectives | held-untested | crates/deckmaste_lexical_source_v4/catalogs/designations.ron:36, crates/deckmaste_lexical_source_v4/catalogs/designations.ron:28, crates/deckmaste_lexical_source_v4/catalogs/designations.ron:7, crates/deckmaste_lexical_source_v4/catalogs/designations.ron:11, crates/deckmaste_lexical_source_v4/catalogs/designations.ron:34 |
| `designation_categories.rs::independently_constructed_nouns_and_adjectives_roundtrip_with_their_owners` | goaded, suspected, harnessed are verb forms, NOT designation adjectives | superseded | v4 declares them as designation adjectives (crates/deckmaste_lexical_source_v4/catalogs/designations.ron:6, crates/deckmaste_lexical_source_v4/catalogs/designations.ron:9, crates/deckmaste_lexical_source_v4/catalogs/designations.ron:10) in addition to the keyword-action participles; lexical analysis 'goaded' -> action:goad PastParticiple/Preterite + designation:goaded Invariant |
| `difference_noun.rs::difference_owns_its_regular_count_noun_paradigm` | difference is a count noun with regular plural 'differences' | held-untested | crates/deckmaste_lexical_source_v4/lexicon/english/nouns.ron:50; lexical analysis -> noun:difference Singular / noun:difference Plural; generic shape asserted by paradigms.rs::every_noun_has_its_full_paradigm (not this lexeme specifically) |
| `difference_noun.rs::difference_owns_its_regular_count_noun_paradigm` | difference is a scalar denotation (ScalarDenotation) with between-complement | superseded | no ScalarDenotation feature; v4 declares Complement(PP([between])) on difference (nouns.ron:50) |
| `difference_noun.rs::difference_owns_its_regular_count_noun_paradigm` | difference carries no NumberTransparency feature (not a number-transparent noun) | held-untested | v4 has no NumberTransparency feature at all; nothing marks any noun transparent |
| `each_of_partitive.rs::independent_fused_head_has_one_reading_and_exact_lexical_owners` | each is a determinative taking an of-partitive complement; 'them' is the accusative of they | held-untested | crates/deckmaste_lexical_source_v4/lexicon/english/closed_class.ron:14 Complement(PP([of])); they/them crates/deckmaste_lexical_source_v4/lexicon/english/closed_class.ron:74; of crates/deckmaste_lexical_source_v4/lexicon/english/prepositions.ron:123; crates/deckmaste_english_v4/judgments/each_of_partitive.txt (pending: COMPLETED_STRATUM=None) |
| `enter_as_copy_except_clause.rs::controls_preserve_their_complete_reading_sets` | enter takes the as-a-copy-of matrix complement (core-verb:Enter MatrixComplementLicence) | superseded | v4 has no verb licence feature (english-v4.md bans policing flags): enter is Intransitive+Monotransitive (crates/deckmaste_lexical_source_v4/lexicon/english/verbs.ron:53), `as` hosts Predicate (crates/deckmaste_lexical_source_v4/lexicon/english/prepositions.ron:24); copy noun has of-frame (crates/deckmaste_lexical_source_v4/lexicon/english/nouns.ron:35); crates/deckmaste_english_v4/judgments/enter_as_copy_except_clause.txt (pending: COMPLETED_STRATUM=None) |
| `enter_as_copy_except_clause.rs::controls_preserve_their_complete_reading_sets` | copy is a relational noun taking of (copy of any creature) | held-untested | crates/deckmaste_lexical_source_v4/lexicon/english/nouns.ron:35; crates/deckmaste_english_v4/judgments/enter_as_copy_except_clause.txt (pending: COMPLETED_STRATUM=None) |
| `enter_as_copy_except_clause.rs::exception_requires_the_declared_enter_licence` | except introduces an exception clause/adjunct only where licensed by the matrix verb enter | superseded | except declares FiniteClause/NP/PP complements with host categories (crates/deckmaste_lexical_source_v4/lexicon/english/prepositions.ron:75); no per-verb licence in v4; crates/deckmaste_english_v4/judgments/enter_as_copy_except_clause.txt (pending: COMPLETED_STRATUM=None) |
| `exactly_cardinal_modifier.rs::precision_has_no_np_modifier_or_clause_adjunct_licence` | exactly is a degree adverb only in phrasal (cardinal-modifier) position: not clause-initial, VP-final or central | held-untested | crates/deckmaste_lexical_source_v4/lexicon/english/adverbs.ron:17 positions: [Phrasal] (so 'Exactly, draw', 'draw a card exactly', 'You exactly draw' have no position); crates/deckmaste_english_v4/judgments/exactly_cardinal_modifier.txt (pending: COMPLETED_STRATUM=None) |
| `existential_there.rs::contracted_ambiguity_and_preterite_lexical_form` | be has Preterite plural 'were' ('there were'); 's contracts is/has | held-untested | crates/deckmaste_lexical_source_v4/lexicon/english/closed_class.ron:112; "there's" -> verb:'re Present + verb:'ve Present (lexical analysis) |
| `existential_there.rs::contracted_ambiguity_and_preterite_lexical_form` | 'card cards' is not a premodified nominal (card is no attribute-premodifier head) | superseded | policing (AttributePremodifierHead feature abolished); reading counts 2/3/0 are retired count assertions |
| `existential_there.rs::dummy_pronoun_has_no_ordinary_nominal_or_adverb_route` | existential there is a dummy pronoun agreeing with the displaced subject (singular and plural nominative), not an ordinary NP | held-untested | crates/deckmaste_lexical_source_v4/lexicon/english/closed_class.ron:65 (3rd Singular and Plural Nominative); crates/deckmaste_english_v4/judgments/existential_there.txt (pending: COMPLETED_STRATUM=None) |
| `existential_there.rs::dummy_pronoun_has_no_ordinary_nominal_or_adverb_route` | there has no Adverb lexeme (vocab:Adverb/There absent) | superseded | locative there exists as an intransitive preposition per CGEL (crates/deckmaste_lexical_source_v4/lexicon/english/prepositions.ron:158, ComplementOnly), not an adverb; adverbs.ron has no there |
| `fixed_cost_phrases.rs::independent_mana_values_preserve_number_attachment_and_adds_existing_frame` | mana is a mass noun with of-phrase measure ('one mana of any color', 'two mana of any one color') | held-untested | crates/deckmaste_lexical_source_v4/lexicon/english/nouns.ron:83 Mass; any crates/deckmaste_lexical_source_v4/lexicon/english/closed_class.ron:10 of-frame; color noun; crates/deckmaste_english_v4/judgments/fixed_cost_phrases.txt (pending: COMPLETED_STRATUM=None) |
| `fixed_cost_phrases.rs::independent_mana_values_preserve_number_attachment_and_adds_existing_frame` | add takes a direct object ('add one mana of any color') | held-untested | crates/deckmaste_lexical_source_v4/lexicon/english/verbs.ron:7 Monotransitive |
| `from_anywhere.rs::compound_determinative_replaces_the_adverb_owner` | anywhere is a single compound Determinative lexeme; no locative proform/adverb anywhere | held-untested | crates/deckmaste_lexical_source_v4/lexicon/english/closed_class.ron:12; adverbs.ron has no anywhere; lexical analysis 'anywhere' -> det:anywhere only |
| `gapped_coordination_shared_adjunct.rs::mixed_goals_preserve_every_np_coordination_reading` | put selects goal PP complements (into/on/onto) so 'one of them into your hand and the rest on the bottom of your library' coordinates | held-untested | crates/deckmaste_lexical_source_v4/lexicon/english/verbs.ron:98 TransitiveWithPP([into,on,onto]); rest noun crates/deckmaste_lexical_source_v4/lexicon/english/nouns.ron:108; bottom noun crates/deckmaste_lexical_source_v4/lexicon/english/nouns.ron:23; crates/deckmaste_english_v4/judgments/gapped_coordination_shared_adjunct.txt (pending: COMPLETED_STRATUM=None) |
| `gapped_coordination_shared_adjunct.rs::mixed_tails_require_the_declared_licence` | SelectedTailLicence lexical feature gates mixed selected-complement tail coordination | superseded | policing feature abolished (english-v4.md Ban on Policing Flags); coordination of selected tails is generic Right-Node/gapping in the engine; crates/deckmaste_english_v4/judgments/gapped_coordination_shared_adjunct.txt (pending: COMPLETED_STRATUM=None) |
| `generic_predicates.rs::legacy_role_labels_remain_unsupported_instead_of_becoming_phrase_categories` | enter has a legacy frame with unsupported slot category Object ('unsupported frames' bookkeeping) | superseded | v3-specific frame-bookkeeping artifact; v4 loader rejects invalid frames outright (tests/loader.rs::a_frame_belongs_to_its_head_category) |
| `generic_predicates.rs::legacy_role_labels_remain_unsupported_instead_of_becoming_phrase_categories` | enter takes the battlefield as direct object ('enter the battlefield', one reading) | held-untested | crates/deckmaste_lexical_source_v4/lexicon/english/verbs.ron:53 Monotransitive |
| `generic_predicates.rs::sun_droplet_remove_retains_object_and_marked_source_roundtrip` | remove takes an object and optionally a from-source complement (remove a charge counter from this artifact; passive 'removed from combat') | held-untested | crates/deckmaste_lexical_source_v4/lexicon/english/verbs.ron:103 Monotransitive+TransitiveWithPP([from]); crates/deckmaste_lexical_source_v4/lexicon/english/prepositions.ron:90; crates/deckmaste_english_v4/judgments/data_batch_2.txt (pending: COMPLETED_STRATUM=None) |
| `gerund_preposition_complements.rs::gerund_complement_requires_declared_licence` | without and of (instead of) take a gerund complement; GerundClausePreposition class | held-untested | crates/deckmaste_lexical_source_v4/lexicon/english/prepositions.ron:197 Gerund; crates/deckmaste_lexical_source_v4/lexicon/english/prepositions.ron:123 Gerund ComplementOnly; crates/deckmaste_lexical_source_v4/lexicon/english/prepositions.ron:112 PP(of); by/for/while/to/rather than also Gerund; crates/deckmaste_english_v4/judgments/gerund_preposition_complements.txt (pending: COMPLETED_STRATUM=None); class replaced by Gerund frames |
| `grammar.rs::clause_taking_prepositions_keep_their_category_and_complement_selection` | until, before, after, whenever, if, when are prepositions taking a finite clause complement | held-untested | crates/deckmaste_lexical_source_v4/lexicon/english/prepositions.ron:169, crates/deckmaste_lexical_source_v4/lexicon/english/prepositions.ron:46, crates/deckmaste_lexical_source_v4/lexicon/english/prepositions.ron:17, crates/deckmaste_lexical_source_v4/lexicon/english/prepositions.ron:181, crates/deckmaste_lexical_source_v4/lexicon/english/prepositions.ron:95, crates/deckmaste_lexical_source_v4/lexicon/english/prepositions.ron:180 each with FiniteClause; crates/deckmaste_english_v4/judgments/grammar.txt (pending: COMPLETED_STRATUM=None) |
| `grammar.rs::clause_taking_prepositions_keep_their_category_and_complement_selection` | if is a Subordinator (v3 asserted category Subordinator) | superseded | CGEL reclassification: if is a preposition (crates/deckmaste_lexical_source_v4/lexicon/english/prepositions.ron:95); closed_class.ron subordinators are only that/to/whether (crates/deckmaste_lexical_source_v4/lexicon/english/closed_class.ron:107) |
| `grammar.rs::clause_taking_prepositions_keep_their_category_and_complement_selection` | with, that, whether take no finite-clause preposition frame ('with you draw cards' rejected); 'that' is a subordinator | held-untested | crates/deckmaste_lexical_source_v4/lexicon/english/prepositions.ron:190 has KeywordPhrase/NP/QuotedAbility only; that/whether in subordinators crates/deckmaste_lexical_source_v4/lexicon/english/closed_class.ron:107 |
| `grammar.rs::independent_named_nominal_retains_selected_frame_and_catalog_identity` | name is a verb taking a complex-transitive frame ('named Powerstone Shard') and card names are Catalog Names | held-untested | crates/deckmaste_lexical_source_v4/lexicon/english/verbs.ron:84 ComplexTransitive; card names are lexemes (name:Powerstone Shard via names.rs:76-92; lexical analysis); tests/names.rs covers derivation only |
| `grammar.rs::independent_named_nominal_retains_selected_frame_and_catalog_identity` | artifact is a card type noun (a named artifact) | held-and-tested | crates/deckmaste_lexical_source_v4/tests/cr_coverage.rs::card_types_and_supertypes |
| `grammar.rs::type_lines_require_ordered_nonempty_groups_from_declared_vocabulary` | card types, supertypes and subtypes in type lines are declared vocabulary (Instant, Artifact Creature, Basic Snow Land, Legendary, Kindred Instant, Battle; Golem, Island, Human, Wizard, Elf, Soldier, Siege) | held-and-tested | crates/deckmaste_lexical_source_v4/tests/cr_coverage.rs::card_types_and_supertypes and crates/deckmaste_lexical_source_v4/tests/cr_coverage.rs::subtypes_by_family (exact CR spelling, every family) |
| `grammar.rs::type_lines_require_ordered_nonempty_groups_from_declared_vocabulary` | an unknown word is not a subtype ('NoSuchSubtype' unknown) | held-and-tested | undeclared words are not guessed: crates/deckmaste_lexical/tests/analysis.rs::undeclared_words_and_parts_of_speech_are_not_guessed (unknown_words reported) |
| `half_rounded.rs::lexical_licence_controls_supplement_admission_and_keeps_the_multiplier_controls` | round declares down/up as its selected particle ('rounded down/up' supplement; ParticipialSupplementUse) | held-untested | crates/deckmaste_lexical_source_v4/lexicon/english/verbs.ron:115 Prepositional+TransitiveWithPP([down,up]); down/up Intransitive prepositions crates/deckmaste_lexical_source_v4/lexicon/english/prepositions.ron:69, crates/deckmaste_lexical_source_v4/lexicon/english/prepositions.ron:176; crates/deckmaste_english_v4/judgments/half_rounded.txt (pending: COMPLETED_STRATUM=None) |
| `half_rounded.rs::lexical_licence_controls_supplement_admission_and_keeps_the_multiplier_controls` | half is a predeterminer of a definite/possessive NP ('half their library', 'half the cards') | gap | half is declared only as a count noun with plural halves and an of-complement (crates/deckmaste_lexical_source_v4/lexicon/english/nouns.ron:71); no determinative/predeterminer declaration in closed_class.ron, yet crates/deckmaste_english_v4/judgments/half_rounded.txt (pending: COMPLETED_STRATUM=None) and predeterminer_all.txt expect 'half their library' / 'half the cards' to parse as an NP |
| `half_rounded.rs::lexical_licence_controls_supplement_admission_and_keeps_the_multiplier_controls` | multiplier controls: twice (adverb) and double (attributive adjective) in 'double that damage', 'copy that spell twice' | held-untested | crates/deckmaste_lexical_source_v4/lexicon/english/adverbs.ron:49 End+Phrasal; crates/deckmaste_lexical_source_v4/lexicon/english/adjectives.ron:108; double also keyword action crates/deckmaste_lexical_source_v4/catalogs/keyword_actions.ron:34; crates/deckmaste_english_v4/judgments/half_rounded.txt (pending: COMPLETED_STRATUM=None) |
| `independent_genitive_pronoun.rs::existing_personal_pronouns_have_one_independent_genitive_head` | you->yours, he->his, she->hers, they->theirs each have exactly one independent-genitive cell | held-untested | crates/deckmaste_lexical_source_v4/lexicon/english/closed_class.ron:100, he crates/deckmaste_lexical_source_v4/lexicon/english/closed_class.ron:45, she crates/deckmaste_lexical_source_v4/lexicon/english/closed_class.ron:62, they crates/deckmaste_lexical_source_v4/lexicon/english/closed_class.ron:78; lexical analysis yours/hers/his/theirs; v4 test asserting this: none (tests/analysis.rs::independent_genitive_is_a_declared_pronoun_paradigm_cell uses its own fixture) |
| `independent_genitive_pronoun.rs::existing_personal_pronouns_have_one_independent_genitive_head` | it has no independent genitive ('its' is not an NP) | held-untested | crates/deckmaste_lexical_source_v4/lexicon/english/closed_class.ron:50 declares Nominative/Accusative/Genitive only; crates/deckmaste_english_v4/judgments/independent_genitive_pronoun.txt (pending: COMPLETED_STRATUM=None) '*NounPhrase its' |
| `independent_genitive_pronoun.rs::existing_personal_pronouns_have_one_independent_genitive_head` | first person: I -> mine is an independent genitive | gap | closed_class.ron declares no first-person pronoun (no I/me/my/mine/we): lexical analysis 'mine' -> stem only, 'me','my','we' unknown; yet crates/deckmaste_english_v4/judgments/independent_genitive_pronoun.txt (pending: COMPLETED_STRATUM=None) expects '+NounPhrase mine => readings 1' (the 'm contraction exists only as an auxiliary) |
| `independent_genitive_pronoun.rs::existing_personal_pronouns_have_one_independent_genitive_head` | no `We` lexeme / no 'ours' | held-untested | trivially held: no first-person plural pronoun declared (lexical analysis 'we', 'ours' unknown) |
| `instead_replacement.rs::replacement_witnesses_retain_their_hosts_and_one_preposition_owner` | instead is a single preposition lexeme (intransitive 'exile it instead'; 'instead of' + gerund) | held-untested | crates/deckmaste_lexical_source_v4/lexicon/english/prepositions.ron:112 frames Intransitive/PP([of]); no adverb instead in adverbs.ron; crates/deckmaste_english_v4/judgments/instead_replacement.txt (pending: COMPLETED_STRATUM=None) |
| `italic_heads.rs::label_requires_its_complete_written_separator_and_a_parsed_body` | an italic head 'Diana —' needs the full ' — ' separator and a parsed body | superseded | labels are Flavor Words/Ability Words in the lexical layer (open class): crates/deckmaste_lexical/tests/analysis.rs::flavor_heads_are_lossless_open_labels_at_line_starts, an_italic_run_no_declaration_spells_is_a_flavor_word and crates/deckmaste_lexical_source_v4/tests/labels.rs; the separator/body check is grammar-side |
| `italic_heads.rs::label_requires_its_complete_written_separator_and_a_parsed_body` | ability words are registered labels, never flavor words | held-and-tested | crates/deckmaste_lexical_source_v4/tests/labels.rs::a_registered_label_is_never_a_flavor_word; crates/deckmaste_lexical_source_v4/tests/cr_coverage.rs::ability_words |
| `kind_of_counter.rs::bare_count_complement_requires_the_declared_head_licence` | kind (but not type) takes a bare count-nominal of-complement ('kind of counter') | held-untested | crates/deckmaste_lexical_source_v4/lexicon/english/nouns.ron:76 has Complement(PP([of])), `type` (crates/deckmaste_lexical_source_v4/lexicon/english/nouns.ron:133) has none; `of` takes a bare Nominal complement (crates/deckmaste_lexical_source_v4/lexicon/english/prepositions.ron:125); crates/deckmaste_english_v4/judgments/kind_of_counter.txt (pending: COMPLETED_STRATUM=None) |
| `landwalk_bound_suffix.rs::generic_landwalk_has_only_the_composed_lexical_route` | generic 'landwalk' has only the composed route (stem land + bound walk) and NO standalone landwalk lexeme | gap | v4 declares landwalk as a keyword lexeme itself (crates/deckmaste_lexical_source_v4/catalogs/keyword_abilities.ron:30), so lexical analysis 'landwalk' yields keyword:landwalk Invariant AND stem:land + keyword:landwalk:bound: two analyses where v3 asserted exactly one |
| `landwalk_bound_suffix.rs::running_text_witnesses_retain_lowercase_stems` | mountainwalk and forestwalk analyse as a land-type stem plus the bound suffix 'walk' | held-untested | crates/deckmaste_lexical_source_v4/catalogs/keyword_abilities.ron:30 bound walk with stems [LandType, CardType]; lexical analysis mountainwalk -> stem:mountain + keyword:landwalk:bound; forestwalk likewise; no test beyond keyword heading coverage |
| `last_turn_temporal_adjunct.rs::call_of_the_full_moon` | Call of the Full Moon: 'last turn' -- last is the adjective and turn the noun (word-owner check); lose life/cast/transform/sacrifice are declared verbs | held-untested | crates/deckmaste_lexical_source_v4/lexicon/english/adjectives.ron:31; crates/deckmaste_lexical_source_v4/lexicon/english/nouns.ron:132; crates/deckmaste_lexical_source_v4/lexicon/english/verbs.ron:74; crates/deckmaste_english_v4/judgments/last_turn_temporal_adjunct.txt (pending: COMPLETED_STRATUM=None) |
| `last_turn_temporal_adjunct.rs::feast_on_the_fallen` | Feast on the Fallen: 'last turn' -- last is the adjective and turn the noun (word-owner check); lose life/cast/transform/sacrifice are declared verbs | held-untested | crates/deckmaste_lexical_source_v4/lexicon/english/adjectives.ron:31; crates/deckmaste_lexical_source_v4/lexicon/english/nouns.ron:132; crates/deckmaste_lexical_source_v4/lexicon/english/verbs.ron:74; crates/deckmaste_english_v4/judgments/last_turn_temporal_adjunct.txt (pending: COMPLETED_STRATUM=None) |
| `last_turn_temporal_adjunct.rs::first_response` | First Response: 'last turn' -- last is the adjective and turn the noun (word-owner check); lose life/cast/transform/sacrifice are declared verbs | held-untested | crates/deckmaste_lexical_source_v4/lexicon/english/adjectives.ron:31; crates/deckmaste_lexical_source_v4/lexicon/english/nouns.ron:132; crates/deckmaste_lexical_source_v4/lexicon/english/verbs.ron:74; crates/deckmaste_english_v4/judgments/last_turn_temporal_adjunct.txt (pending: COMPLETED_STRATUM=None) |
| `last_turn_temporal_adjunct.rs::lambholt_elder` | Lambholt Elder: 'last turn' -- last is the adjective and turn the noun (word-owner check); lose life/cast/transform/sacrifice are declared verbs | held-untested | crates/deckmaste_lexical_source_v4/lexicon/english/adjectives.ron:31; crates/deckmaste_lexical_source_v4/lexicon/english/nouns.ron:132; crates/deckmaste_lexical_source_v4/lexicon/english/verbs.ron:74; crates/deckmaste_english_v4/judgments/last_turn_temporal_adjunct.txt (pending: COMPLETED_STRATUM=None) |
| `last_turn_temporal_adjunct.rs::last_has_one_word_owner_and_keeps_ordinary_premodification` | last has exactly one lexical analysis (a single attributive adjective) | held-untested | crates/deckmaste_lexical_source_v4/lexicon/english/adjectives.ron:31; lexical analysis 'last' -> adj:last Invariant only |
| `last_turn_temporal_adjunct.rs::silverpelt_werewolf` | Silverpelt Werewolf: 'last turn' -- last is the adjective and turn the noun (word-owner check); lose life/cast/transform/sacrifice are declared verbs | held-untested | crates/deckmaste_lexical_source_v4/lexicon/english/adjectives.ron:31; crates/deckmaste_lexical_source_v4/lexicon/english/nouns.ron:132; crates/deckmaste_lexical_source_v4/lexicon/english/verbs.ron:74; crates/deckmaste_english_v4/judgments/last_turn_temporal_adjunct.txt (pending: COMPLETED_STRATUM=None) |
| `lexical_compositions.rs::attested_hybrid_and_phyrexian_symbols_retain_cost_and_mana_identity` | all ten two-color hybrid symbols, five Phyrexian symbols and the {2/x} monocolored hybrids are declared symbols | held-untested | crates/deckmaste_lexical_source_v4/lexicon/english/closed_class.ron:173 (hybrid), crates/deckmaste_lexical_source_v4/lexicon/english/closed_class.ron:174 (monocolored hybrid), crates/deckmaste_lexical_source_v4/lexicon/english/closed_class.ron:176 (Phyrexian); symbols are Core Symbol lexemes; no test enumerates them |
| `lexical_compositions.rs::attested_hybrid_and_phyrexian_symbols_retain_cost_and_mana_identity` | FixedCostSymbol vs mana-symbol identity (cost symbol vs mana symbol use features) | superseded | v4 symbols are one Symbol category with no ManaSymbolUse/FixedCostSymbol feature; use is grammar-side |
| `lexical_gaps.rs::independent_intransitive_values_retain_frame_and_lexical_identity` | resolve and trigger are verbs with an intransitive frame | held-untested | v4 declares trigger as Intransitive+Monotransitive (crates/deckmaste_lexical_source_v4/lexicon/english/verbs.ron:132), a superset of the v3 frame, and resolve as Intransitive only (crates/deckmaste_lexical_source_v4/lexicon/english/verbs.ron:108) |
| `lexical_gaps.rs::oracle_constituents_use_declared_verb_frames` | resolve and trigger are intransitive verbs; resolves/triggers agree in number | held-untested | crates/deckmaste_lexical_source_v4/lexicon/english/verbs.ron:108 Intransitive; crates/deckmaste_lexical_source_v4/lexicon/english/verbs.ron:132 Intransitive+Monotransitive; paradigm generic: paradigms.rs::every_verb_has_its_full_paradigm |
| `lexical_gaps.rs::oracle_constituents_use_declared_verb_frames` | remain takes a locative/predicative complement ('remains on the battlefield') | held-untested | crates/deckmaste_lexical_source_v4/lexicon/english/verbs.ron:102 Copular |
| `lexical_gaps.rs::oracle_constituents_use_declared_verb_frames` | distribute takes an among-PP ('Distribute two +1/+1 counters') | held-untested | crates/deckmaste_lexical_source_v4/lexicon/english/verbs.ron:46 TransitiveWithPP([among]) |
| `mana_symbol_passive_subject.rs::passive_licence_belongs_to_the_declared_frame_class` | spend, add, produce take a mana direct object (passives 'spent', 'added', 'produced' with a mana subject); pay does not | held-untested | crates/deckmaste_lexical_source_v4/lexicon/english/verbs.ron:120, crates/deckmaste_lexical_source_v4/lexicon/english/verbs.ron:7, crates/deckmaste_lexical_source_v4/lexicon/english/verbs.ron:95 Monotransitive; crates/deckmaste_lexical_source_v4/lexicon/english/verbs.ron:90 also Monotransitive |
| `mana_symbol_passive_subject.rs::passive_licence_belongs_to_the_declared_frame_class` | 'paid' is rejected as a mana passive (v3 mana-frame class excludes pay) | superseded | policing frame class abolished (ManaPassive... flags banned by english-v4.md); passives are engine-derived from active frames, so pay passivizes like any transitive |
| `multiplier_predeterminer.rs::declared_licence_preserves_the_controls_complete_reading_sets` | twice is a multiplier modifying an NP ('twice its power') and an adverb ('copy that spell twice', 'Activate only twice') | held-untested | crates/deckmaste_lexical_source_v4/lexicon/english/adverbs.ron:49 positions [End, Phrasal]; crates/deckmaste_english_v4/judgments/half_rounded.txt (pending: COMPLETED_STRATUM=None) / multiplier_predeterminer.txt |
| `multiplier_predeterminer.rs::declared_licence_preserves_the_controls_complete_reading_sets` | double is both attributive multiplier and the keyword action ('double target player's life total', 'double strike') | held-untested | crates/deckmaste_lexical_source_v4/lexicon/english/adjectives.ron:108; crates/deckmaste_lexical_source_v4/catalogs/keyword_actions.ron:34; crates/deckmaste_lexical_source_v4/catalogs/keyword_abilities.ron:9 |
| `multiword_lexemes.rs::independent_multiword_nominals_own_one_lexical_leaf_in_both_roundtrip_directions` | mana value and mana cost are single multiword noun lexemes with plurals 'mana values'/'mana costs' | held-untested | crates/deckmaste_lexical_source_v4/lexicon/english/nouns.ron:158, crates/deckmaste_lexical_source_v4/lexicon/english/nouns.ron:157; lexical analysis 'mana values' -> noun:mana value Plural (overlapping with mana + values, as multiword_and_bound_forms_keep_overlapping_analyses allows); crates/deckmaste_english_v4/judgments/multiword_lexemes.txt (pending: COMPLETED_STRATUM=None) |
| `multiword_lexemes.rs::independent_orientation_pps_and_coordination_preserve_whole_lexical_words` | face up / face down are single multiword preposition lexemes | superseded | v4 has no multiword 'face up'/'face down': it composes face (noun/keyword verb crates/deckmaste_lexical_source_v4/catalogs/keyword_actions.ron:173) with intransitive prepositions up/down (crates/deckmaste_lexical_source_v4/lexicon/english/prepositions.ron:176, crates/deckmaste_lexical_source_v4/lexicon/english/prepositions.ron:69); face-up/face-down are also adjectives (crates/deckmaste_lexical_source_v4/lexicon/english/adjectives.ron:19, crates/deckmaste_lexical_source_v4/lexicon/english/adjectives.ron:18); verb frames: draft ComplexTransitive, turn ComplexTransitive |
| `multiword_lexemes.rs::independent_orientation_pps_and_coordination_preserve_whole_lexical_words` | or is a coordinator joining the orientation phrases | held-untested | crates/deckmaste_lexical_source_v4/lexicon/english/closed_class.ron:106 |
| `name_lexeme_scope.rs::plural_full_name_subject_and_token_name_keep_their_intended_identity` | a card name that is a plural noun phrase ('Aang and Katara') carries plural number (NameNumber=Plural): 'Aang and Katara enter.' but not 'enters.' | gap | name lexemes are built by names.rs:78-92 with no number feature (single Invariant form); lexical analysis 'Aang and Katara' -> name:Aang and Katara Invariant only; no plural-name declaration anywhere in v4 |
| `name_lexeme_scope.rs::plural_full_name_subject_and_token_name_keep_their_intended_identity` | 'named Wasp': Wasp is a Name (token name introduced by 'named') | gap | lexical analysis 'Wasp' -> unknown; tokens.ron declares only the [CR#111.10] predefined token names; no catalog for names a card text introduces (see vocabulary_batch_2 token-name rows) |
| `name_lexeme_scope.rs::shortened_names_keep_one_card_identity_and_roundtrip_in_its_scope` | a Shortened Name (Chandra, Jace, Ajani, Gandalf, Niv-Mizzet) is one card's own name, in scope only on that card | held-and-tested | crates/deckmaste_lexical_source_v4/tests/names.rs::shortened_names_are_derived_from_the_full_name and crates/deckmaste_lexical_source_v4/tests/names.rs::a_shortened_name_is_in_scope_only_on_its_own_card (rule in crates/deckmaste_lexical_source_v4/src/names.rs:19) |
| `name_lexeme_scope.rs::shortened_names_keep_one_card_identity_and_roundtrip_in_its_scope` | shortened name realizes and re-analyzes to the same Name value (variant index 1, scoped roundtrip) | held-and-tested | crates/deckmaste_lexical_source_v4/tests/paradigms.rs::every_value_realizes_and_analyzes_back (surfaces [full, short] with card_scoped_surfaces; names.rs:86-89); variant numbering is an internal detail |
| `name_lexeme_scope.rs::shortened_names_keep_one_card_identity_and_roundtrip_in_its_scope` | 'Jace, the Mind Sculptor' and 'Ajani Goldmane' have NO shortened name (Jace/Ajani unreadable on their card) | superseded | v4 deliberately derives 'Jace' for Jace, the Mind Sculptor and 'Ajani' (a planeswalker type is the character's own name): crates/deckmaste_lexical_source_v4/src/names.rs:13-18, asserted opposite in tests/names.rs::shortened_names_are_derived_from_the_full_name |
| `name_lexeme_scope.rs::shortened_names_keep_one_card_identity_and_roundtrip_in_its_scope` | Chandra is also the planeswalker subtype Chandra ('a Chandra planeswalker card') | held-and-tested | crates/deckmaste_lexical_source_v4/tests/cr_coverage.rs::subtypes_by_family (planeswalker types [CR#205.3j]) |
| `negated_to_infinitival.rs::negation_requires_the_named_word` | not is a named (identity-gated) negator word | held-untested | crates/deckmaste_lexical_source_v4/lexicon/english/adverbs.ron:31; closed class (Core); crates/deckmaste_english_v4/judgments/negated_to_infinitival.txt (pending: COMPLETED_STRATUM=None) |
| `negative_common_noun_premodifier.rs::hour_of_reckoning` | 'nontoken' (Destroy all nontoken creatures.) is the prefix non- on the noun token: owner lexeme check | superseded | v4 composes non (affix crates/deckmaste_lexical_source_v4/lexicon/english/closed_class.ron:163) + noun token (crates/deckmaste_lexical_source_v4/lexicon/english/nouns.ron:129); lexical analysis 'nontoken(s)' -> affix:non + noun:token; no `lexeme:CommonNoun/Token/non`; crates/deckmaste_english_v4/judgments/negative_common_noun_premodifier.txt (pending: COMPLETED_STRATUM=None) |
| `negative_common_noun_premodifier.rs::independently_constructed_negative_noun_keeps_its_identity_and_plural_head` | negative noun token/non keeps singular identity while its head noun is plural (nontoken creatures) | superseded | same affix+noun composition; plural head creatures = type:creature Plural; 'token'/'nontokens' NounPremodifier negatives were policing |
| `negative_common_noun_premodifier.rs::isilu_carrier_of_twilight` | 'nontoken' (Each other nontoken creature you control has persist.) is the prefix non- on the noun token: owner lexeme check | superseded | v4 composes non (affix crates/deckmaste_lexical_source_v4/lexicon/english/closed_class.ron:163) + noun token (crates/deckmaste_lexical_source_v4/lexicon/english/nouns.ron:129); lexical analysis 'nontoken(s)' -> affix:non + noun:token; no `lexeme:CommonNoun/Token/non`; crates/deckmaste_english_v4/judgments/negative_common_noun_premodifier.txt (pending: COMPLETED_STRATUM=None) |
| `negative_common_noun_premodifier.rs::rhythm_of_the_wild` | 'nontoken' (Nontoken creatures you control have riot.) is the prefix non- on the noun token: owner lexeme check | superseded | v4 composes non (affix crates/deckmaste_lexical_source_v4/lexicon/english/closed_class.ron:163) + noun token (crates/deckmaste_lexical_source_v4/lexicon/english/nouns.ron:129); lexical analysis 'nontoken(s)' -> affix:non + noun:token; no `lexeme:CommonNoun/Token/non`; crates/deckmaste_english_v4/judgments/negative_common_noun_premodifier.txt (pending: COMPLETED_STRATUM=None) |
| `negative_common_noun_premodifier.rs::swift_spiral` | 'nontoken' (Exile target nontoken creature.) is the prefix non- on the noun token: owner lexeme check | superseded | v4 composes non (affix crates/deckmaste_lexical_source_v4/lexicon/english/closed_class.ron:163) + noun token (crates/deckmaste_lexical_source_v4/lexicon/english/nouns.ron:129); lexical analysis 'nontoken(s)' -> affix:non + noun:token; no `lexeme:CommonNoun/Token/non`; crates/deckmaste_english_v4/judgments/negative_common_noun_premodifier.txt (pending: COMPLETED_STRATUM=None) |
| `negative_common_noun_premodifier.rs::thraben_watcher` | 'nontoken' (Other nontoken creatures you control get +1/+1 and have vigilance.) is the prefix non- on the noun token: owner lexeme check | superseded | v4 composes non (affix crates/deckmaste_lexical_source_v4/lexicon/english/closed_class.ron:163) + noun token (crates/deckmaste_lexical_source_v4/lexicon/english/nouns.ron:129); lexical analysis 'nontoken(s)' -> affix:non + noun:token; no `lexeme:CommonNoun/Token/non`; crates/deckmaste_english_v4/judgments/negative_common_noun_premodifier.txt (pending: COMPLETED_STRATUM=None) |
| `noncombat_negative_noun.rs::negative_noun_retains_plugin_provenance_and_declared_properties` | 'noncombat' is the noun combat with a non- prefix: same category/properties, lemma noncombat | superseded | v4 composes it: affix non (crates/deckmaste_lexical_source_v4/lexicon/english/closed_class.ron:163) + noun combat (count/mass, crates/deckmaste_lexical_source_v4/lexicon/english/nouns.ron:31); lexical analysis 'noncombat' -> affix:non + noun:combat; crates/deckmaste_english_v4/judgments/noncombat_negative_noun.txt (pending: COMPLETED_STRATUM=None) |
| `notation_coordination.rs::correlative_markers_keep_determinative_identity_and_nor_is_distinct` | both, either, neither are determinatives (floated/correlative markers) | held-untested | crates/deckmaste_lexical_source_v4/lexicon/english/closed_class.ron:13, crates/deckmaste_lexical_source_v4/lexicon/english/closed_class.ron:15, crates/deckmaste_lexical_source_v4/lexicon/english/closed_class.ron:26 in determinatives; crates/deckmaste_english_v4/judgments/notation_coordination.txt (pending: COMPLETED_STRATUM=None) |
| `notation_coordination.rs::correlative_markers_keep_determinative_identity_and_nor_is_distinct` | nor is a coordinator distinct from and/or; and/or noncorrelative; coordination kinds Alternative/Correlative | superseded | coordinators are plain lemmas (crates/deckmaste_lexical_source_v4/lexicon/english/closed_class.ron:106): nor and and/or are separate lexemes; CoordinationKind/CorrelativeCoordinator features are not lexical in v4 (correlation is grammar-level) |
| `noun_premodifiers.rs::independently_declared_noun_modifier_bases_keep_noun_identity` | creature, Vampire, Assembly-Worker, Time Lord are nouns usable as noun premodifiers | held-and-tested | crates/deckmaste_lexical_source_v4/tests/cr_coverage.rs::card_types_and_supertypes (creature) and crates/deckmaste_lexical_source_v4/tests/cr_coverage.rs::subtypes_by_family (Vampire, Assembly-Worker, Time Lord exact spelling); premodifier use is grammar |
| `number_transparency.rs::sway_subject_preserves_exact_readings_and_nominal_head` | any, number (with of), of, target (Targeting Marker adjective), creature as the lexical owners of 'any number of target creatures' | held-untested | crates/deckmaste_lexical_source_v4/lexicon/english/closed_class.ron:10, crates/deckmaste_lexical_source_v4/lexicon/english/nouns.ron:86 PP(of), crates/deckmaste_lexical_source_v4/lexicon/english/adjectives.ron:68 (attributive-only marker), type:creature |
| `number_transparency.rs::sway_subject_preserves_exact_readings_and_nominal_head` | number is number-transparent: 'any number of target creatures' agrees plural with the of-complement | superseded | no NumberTransparency feature; agreement is derived from the of-complement in the grammar (crates/deckmaste_english_v4/judgments/number_transparency.txt (pending: COMPLETED_STRATUM=None): 'readings follow the of-complement for any number, the head for the number') |
| `partner_with_name_keyword.rs::bare_partner_and_the_dashed_variant_keep_their_existing_lexical_owners` | Partner and the dashed variant Partner—Survivors are separate keyword lexemes | held-untested | crates/deckmaste_lexical_source_v4/catalogs/keyword_abilities.ron:222 with variants partner—Survivors ... (crates/deckmaste_lexical_source_v4/catalogs/keyword_abilities.ron:228); lexical analysis 'Partner—Survivors' -> keyword:partner—Survivors; cr_coverage::keyword_abilities checks headings only |
| `phase_in_out.rs::phase_has_no_bare_frame_or_free_particle_adjunct_analysis` | phase has no bare/intransitive frame, no phases up, and out is not a free adverb (no Adverb/Out) | held-untested | phase frames are only Prepositional([in,out]); out is a ComplementOnly preposition (never an adjunct: crates/deckmaste_lexical_source_v4/lexicon/english/prepositions.ron:135); round takes down/up not out (crates/deckmaste_lexical_source_v4/lexicon/english/verbs.ron:115); crates/deckmaste_english_v4/judgments/phase_in_out.txt (pending: COMPLETED_STRATUM=None) |
| `phase_in_out.rs::ticket_witnesses_retain_phase_and_its_obligatory_complement` | phase takes the obligatory particle in/out | held-untested | crates/deckmaste_lexical_source_v4/lexicon/english/verbs.ron:91 Prepositional([in,out]); in crates/deckmaste_lexical_source_v4/lexicon/english/prepositions.ron:102 and out crates/deckmaste_lexical_source_v4/lexicon/english/prepositions.ron:135 are Intransitive preps; crates/deckmaste_english_v4/judgments/phase_in_out.txt (pending: COMPLETED_STRATUM=None) |
| `phrasal_keyword_surfaces.rs::phrasal_keyword_lines_have_one_declared_route_and_roundtrip` | Choose a Background, Doctor's companion, Partner—Friends forever / Character select / Father & son / Survivors are whole keyword surfaces | held-untested | crates/deckmaste_lexical_source_v4/catalogs/keyword_abilities.ron:230, crates/deckmaste_lexical_source_v4/catalogs/keyword_abilities.ron:231, crates/deckmaste_lexical_source_v4/catalogs/keyword_abilities.ron:227, crates/deckmaste_lexical_source_v4/catalogs/keyword_abilities.ron:225, crates/deckmaste_lexical_source_v4/catalogs/keyword_abilities.ron:226, crates/deckmaste_lexical_source_v4/catalogs/keyword_abilities.ron:228 as keyword variants; lexical analysis yields each as a single keyword: lexeme; cr_coverage checks headings only |
| `play_with_absolute.rs::admission_requires_the_declared_verb_frame_and_marker_licence` | play takes a with-absolute clause as selected complement ('play with their hands revealed') | gap | crates/deckmaste_lexical_source_v4/catalogs/keyword_actions.ron:60 frames are Monotransitive+Intransitive only (no Prepositional([with])), and `with` declares no verbless/participial-clause complement (crates/deckmaste_lexical_source_v4/lexicon/english/prepositions.ron:190: KeywordPhrase, NP, QuotedAbility); crates/deckmaste_english_v4/judgments/play_with_absolute.txt (pending: COMPLETED_STRATUM=None) expects 'PrepositionPhrase "with their hands revealed" as Complement of play' vs Adjunct of attack (attack has Prepositional([with]), verbs.ron:13) |
| `play_with_absolute.rs::admission_requires_the_declared_verb_frame_and_marker_licence` | play is a keyword action with Monotransitive (play lands) and Intransitive frames | held-untested | crates/deckmaste_lexical_source_v4/catalogs/keyword_actions.ron:60 |
| `play_with_absolute.rs::admission_requires_the_declared_verb_frame_and_marker_licence` | PrepositionFunctionLicence on with (Modifier\|Adjunct\|PreposedAdjunct) gates the with-absolute | superseded | function licences replaced by per-frame host categories (prepositions.ron header) |
| `plugin_term_definitions.rs::plugin_terms_retain_their_definition_owners_and_categories` | modified is an adjective | superseded | v4 takes modified as the past participle of verb modify (crates/deckmaste_lexical_source_v4/lexicon/english/verbs.ron:79; adjectives.ron header: participles come from the verb); lexical analysis 'modified' -> verb:modify PastParticiple |
| `plugin_term_definitions.rs::plugin_terms_retain_their_definition_owners_and_categories` | poisoned is an adjective | held-untested | crates/deckmaste_lexical_source_v4/lexicon/english/adjectives.ron:54 |
| `plugin_term_definitions.rs::plugin_terms_retain_their_definition_owners_and_categories` | expend and descend are verbs (event verbs) | held-untested | crates/deckmaste_lexical_source_v4/lexicon/english/verbs.ron:54 (Amount), crates/deckmaste_lexical_source_v4/lexicon/english/verbs.ron:43 (Intransitive) |
| `plugin_term_definitions.rs::plugin_terms_retain_their_definition_owners_and_categories` | multikicker is a keyword variant of kicker | held-untested | crates/deckmaste_lexical_source_v4/catalogs/keyword_abilities.ron:67; cr_coverage::keyword_abilities checks only rule headings, not variants |
| `plugin_term_definitions.rs::plugin_terms_retain_their_definition_owners_and_categories` | convoke, devour, evolve are also verbs derived from the keyword definition | held-untested | crates/deckmaste_lexical_source_v4/catalogs/keyword_abilities.ron:108 (convoke verb), crates/deckmaste_lexical_source_v4/catalogs/keyword_abilities.ron:164, crates/deckmaste_lexical_source_v4/catalogs/keyword_abilities.ron:186; lexical analysis -> keyword_verb:convoke/devour/evolve Plain |
| `plugin_term_definitions.rs::plugin_terms_retain_their_definition_owners_and_categories` | plugin Source provenance (kind Plugin, owner path plugins_v2/builtin/macros/...) on these lexemes | superseded | v4 provenance is Catalog/Core by data file (README table); plugins_v2/builtin is deleted at english-v4-cutover-v3 (CLAUDE.md) |
| `polyhedral_die_nouns.rs::independently_constructed_nouns_preserve_singular_and_invariant_plural` | d4,d6,d8,d10,d12,d20 are nouns (singular) | held-untested | crates/deckmaste_lexical_source_v4/lexicon/english/nouns.ron:40..crates/deckmaste_lexical_source_v4/lexicon/english/nouns.ron:45 |
| `polyhedral_die_nouns.rs::independently_constructed_nouns_preserve_singular_and_invariant_plural` | the plural of d4..d20 is invariant ('two d6'): 'd6' reads as both singular and plural | gap | nouns.ron:40-45 declare plain nouns with no plural override, so the plural is the regular 'd6s' (lexical analysis 'd6s' -> noun:d6 Plural; 'd6' -> Singular only); Oracle prints 'Roll two d6'; crates/deckmaste_english_v4/judgments/polyhedral_die_nouns.txt (pending: COMPLETED_STRATUM=None) expects '+ Roll two d6 ... => NounPhrase "two d6" as Object of roll' |
| `predeterminer_all.rs::existing_routes_keep_their_complete_reading_sets` | all and both are determinatives that take an of-complement ('all of the cards', 'both of the cards') and precede the/possessives ('all the cards') | held-untested | crates/deckmaste_lexical_source_v4/lexicon/english/closed_class.ron:8, crates/deckmaste_lexical_source_v4/lexicon/english/closed_class.ron:13 (Complement(PP([of]))); crates/deckmaste_english_v4/judgments/predeterminer_all.txt (pending: COMPLETED_STRATUM=None) |
| `predeterminer_all.rs::existing_routes_keep_their_complete_reading_sets` | half and twice as predeterminers in 'half the cards', 'twice that many cards' | gap | twice is an adverb (crates/deckmaste_lexical_source_v4/lexicon/english/adverbs.ron:49) but half is only a noun (see half_rounded gap); crates/deckmaste_english_v4/judgments/predeterminer_all.txt (pending: COMPLETED_STRATUM=None) lists '+NounPhrase half the cards' |
| `predeterminer_all.rs::existing_routes_keep_their_complete_reading_sets` | many/that many, other, three as the controls ('all other creatures', 'all three cards', 'that many cards') | held-untested | crates/deckmaste_lexical_source_v4/lexicon/english/closed_class.ron:22, crates/deckmaste_lexical_source_v4/lexicon/english/adjectives.ron:49; numerals are lexical-crate numerals |
| `preverbal_next.rs::attributive_next_and_existing_central_adjuncts_keep_their_readings` | next is an Adverb in preverbal position ('When you next cast an instant or sorcery spell') | gap | adverbs.ron declares no `next` (only adjective crates/deckmaste_lexical_source_v4/lexicon/english/adjectives.ron:44); lexical analysis 'next' -> adj:next only; crates/deckmaste_english_v4/judgments/preverbal_next.txt (pending: COMPLETED_STRATUM=None) expects 'AdverbPhrase "next" as Adjunct of cast' (Galvanic Iteration, The Clone Saga, All-Out Assault, Dual Strike) |
| `preverbal_next.rs::attributive_next_and_existing_central_adjuncts_keep_their_readings` | attributive next ('your next upkeep', 'the next end step', 'the next time') | held-untested | crates/deckmaste_lexical_source_v4/lexicon/english/adjectives.ron:44; crates/deckmaste_english_v4/judgments/preverbal_next.txt (pending: COMPLETED_STRATUM=None) |
| `preverbal_next.rs::attributive_next_and_existing_central_adjuncts_keep_their_readings` | central adverbs also and still ('You also draw a card', 'It still has flying') | held-untested | crates/deckmaste_lexical_source_v4/lexicon/english/adverbs.ron:6 Front+Central; crates/deckmaste_lexical_source_v4/lexicon/english/adverbs.ron:46 Central |
| `pronominal_other.rs::independent_definite_np_roundtrips_and_preserves_the_lexical_values` | 'the other' realizes from determinative the + word other | held-untested | crates/deckmaste_lexical_source_v4/lexicon/english/closed_class.ron:32, crates/deckmaste_lexical_source_v4/lexicon/english/adjectives.ron:49 |
| `pronominal_other.rs::ticket_witnesses_have_other_as_the_noun_head_of_a_definite_np` | other is a noun head in 'the other' (one of them ... and the other) | superseded | v4 declares only the adjective other (crates/deckmaste_lexical_source_v4/lexicon/english/adjectives.ron:49); lexical analysis 'the other' -> det:the + adj:other; a CGEL fused modifier-head analysis would be grammar-side (crates/deckmaste_english_v4/judgments/pronominal_other.txt (pending: COMPLETED_STRATUM=None) '+NounPhrase the other => ... head other') |
| `purpose_infinitival.rs::purpose_attachment_reads_the_declared_licence` | spend licenses a purpose to-infinitive adjunct (PurposeAdjunctLicence) | superseded | purpose adjuncts are general, not verb-licensed; spend is Monotransitive+TransitiveWithPP([on]) (crates/deckmaste_lexical_source_v4/lexicon/english/verbs.ron:120); `spent` participle; crates/deckmaste_english_v4/judgments/purpose_infinitival.txt (pending: COMPLETED_STRATUM=None) |
| `purpose_infinitival.rs::purpose_does_not_supply_a_negative_clause_or_a_noun_relative` | purpose to-infinitive takes no negation or modal; 'mana to cast this spell' is not a noun relative; more has MeasureModifierUse | superseded | policing (english-v4.md Ban on Policing Flags); more is a determinative with of/than frames (crates/deckmaste_lexical_source_v4/lexicon/english/closed_class.ron:23); crates/deckmaste_english_v4/judgments/purpose_infinitival.txt (pending: COMPLETED_STRATUM=None) |
| `quality_cost_keyword_lines.rs::bare_keyword_premodifiers_keep_the_cost_only_owner` | equip costs / equip abilities: equip is the keyword lexeme as noun premodifier (single owner) | held-untested | crates/deckmaste_lexical_source_v4/catalogs/keyword_abilities.ron:16; lexical analysis 'equip costs' also yields keyword_verb:equip Plain/Present, so the premodifier owner can be keyword or verb lexeme (grammar decides); crates/deckmaste_english_v4/judgments/quality_cost_keyword_lines.txt (pending: COMPLETED_STRATUM=None) |
| `quantified_mana_symbol.rs::declared_mana_class_supplies_the_quantified_paradigm` | every colored/hybrid/Phyrexian mana symbol can be quantified by a numeral ('six {R/G}') | superseded | v4 symbols are plain Symbol lexemes (crates/deckmaste_lexical_source_v4/lexicon/english/closed_class.ron:168); numeral+symbol NP is a grammar production (crates/deckmaste_english_v4/judgments/quantified_mana_symbol.txt (pending: COMPLETED_STRATUM=None) '+NounPhrase six {G}'), no per-symbol class |
| `rather_than_complements.rs::governor_selection_reads_the_declared_feature` | rather licenses a comparison complement (rather than) via MatrixComparisonUse | superseded | v4 declares the multiword preposition 'rather than' with NP/BareInf/Gerund complements (crates/deckmaste_lexical_source_v4/lexicon/english/prepositions.ron:145); bare `rather` is not a lexeme (lexical analysis 'rather' -> stem rat + her); crates/deckmaste_english_v4/judgments/rather_than_complements.txt (pending: COMPLETED_STRATUM=None) |
| `restricted_nominals.rs::unlicensed_adjectives_do_not_admit_bare_singular_count_nominals` | kicked is declared vocabulary (kicker adjective): 'kicked creature' is analysable | superseded | v4 has no kicker/adjective lexeme: kicked = past participle of verb kick (crates/deckmaste_lexical_source_v4/catalogs/keyword_abilities.ron:70); lexical analysis 'kicked' -> keyword_verb:kick PastParticiple/Preterite |
| `restricted_nominals.rs::unlicensed_adjectives_do_not_admit_bare_singular_count_nominals` | red is an adjective (colour word) and creature a noun, so 'red creature' has no unknown words | held-untested | crates/deckmaste_lexical_source_v4/lexicon/english/adjectives.ron:90, type:creature (cr_coverage::card_types_and_supertypes) |
| `ring_tempts_you.rs::designation_is_a_count_noun_under_a_genitive_determiner_and_in_the_plural` | Ring-bearer is a count noun with plural Ring-bearers (takes genitive determiner 'your Ring-bearer') | held-untested | crates/deckmaste_lexical_source_v4/catalogs/designations.ron:8 (default Count); lexical analysis 'Ring-bearers' -> designation:Ring-bearer Plural; paradigms.rs::every_noun_has_its_full_paradigm checks Singular+Plural forms generally |
| `ring_tempts_you.rs::plugin_provenance_and_regular_inflections_replace_the_keyword_leaf` | The Ring Tempts You contributes noun Ring and verb tempt with regular forms (tempt/tempts/tempted/tempting) | held-untested | crates/deckmaste_lexical_source_v4/catalogs/keyword_actions.ron:166 and noun Ring (crates/deckmaste_lexical_source_v4/catalogs/keyword_actions.ron:169); lexical analysis: tempts Present, tempted Past/Preterite, tempting Gerund; paradigms.rs::every_verb_has_its_full_paradigm |
| `ring_tempts_you.rs::plugin_provenance_and_regular_inflections_replace_the_keyword_leaf` | no whole-phrase keyword leaf for 'the Ring tempts you' | held-untested | keyword_actions.ron declares the verb and noun, not a phrase lexeme ('the Ring tempts you' analyses as det+noun+verb+pronoun) |
| `ring_tempts_you.rs::plugin_provenance_and_regular_inflections_replace_the_keyword_leaf` | Ring-bearer is a separate designation noun lexeme | held-untested | crates/deckmaste_lexical_source_v4/catalogs/designations.ron:8 |
| `supertype_premodified_bound_keyword.rs::attested_premodifiers_preserve_lexical_roles_and_both_roundtrip_laws` | Snow/Legendary/Basic are supertype premodifiers of a bound keyword; Nonbasic is non+basic | held-untested | crates/deckmaste_lexical_source_v4/catalogs/types.ron:1 (cr_coverage::card_types_and_supertypes covers membership); affix non crates/deckmaste_lexical_source_v4/lexicon/english/closed_class.ron:163; lexical analysis 'nonbasic landwalk' -> affix:non + supertype:basic + keyword:landwalk(+bound); v3's PredicativeAdjective/Legendary is now supertype:legendary |
| `supertype_premodified_bound_keyword.rs::attested_premodifiers_preserve_lexical_roles_and_both_roundtrip_laws` | landcycling {1} with Basic: bound suffix cycling after stem land | held-untested | crates/deckmaste_lexical_source_v4/catalogs/keyword_abilities.ron:56 bound cycling stems [Subtype,CardType,Supertype]; lexical analysis 'basic landcycling' -> supertype:basic + stem:land + keyword:cycling:bound |
| `typecycling_bound_suffix.rs::other_attested_classes_keep_their_declared_owners` | Mountaincycling, Islandcycling, Wizardcycling, Slivercycling, Halflingcycling, landcycling: subtype/type stem + bound cycling | held-untested | crates/deckmaste_lexical_source_v4/catalogs/keyword_abilities.ron:56 stems [Subtype, CardType, Supertype]; lexical analysis Mountaincycling/Wizardcycling/Slivercycling/Halflingcycling -> subtype:* + keyword:cycling:bound; the stems themselves are tested in cr_coverage::subtypes_by_family |
| `typecycling_bound_suffix.rs::plain_cycling_keeps_its_free_lexical_owner` | plain Cycling is the free keyword lexeme keyword:cycling | held-untested | crates/deckmaste_lexical_source_v4/catalogs/keyword_abilities.ron:56; lexical analysis 'Cycling' -> keyword:cycling (+ verb gerund) |
| `typecycling_bound_suffix.rs::plain_cycling_keeps_its_free_lexical_owner` | there is no standalone 'typecycling' lexeme | held-untested | lexical analysis 'typecycling' -> noun:type + keyword:cycling:bound only; no lexeme with lemma typecycling |
| `typecycling_bound_suffix.rs::plain_cycling_keeps_its_free_lexical_owner` | Cycling {2}/{1}/{1}{W} has exactly one lexical-owner reading | superseded | owner-count (reading-count) assertion; the lexicon also lists keyword_verb:cycle gerund and the bound form for 'cycling' (retired as count/shape per audit) |
| `verbless_while_adjunct.rs::finite_while_copular_saddled_and_if_able_readings_are_preserved` | while takes a finite clause; if takes a verbless clause ('if able') | held-untested | crates/deckmaste_lexical_source_v4/lexicon/english/prepositions.ron:185, crates/deckmaste_lexical_source_v4/lexicon/english/prepositions.ron:95 VerblessClause; able adjective with ToInf (crates/deckmaste_lexical_source_v4/lexicon/english/adjectives.ron:5); crates/deckmaste_english_v4/judgments/verbless_while_adjunct.txt (pending: COMPLETED_STRATUM=None) |
| `verbless_while_adjunct.rs::independent_complement_keeps_owner_roundtrips_and_lexical_licences` | while takes a verbless-clause complement (VerblessClauseComplement) and is a predicate-hosted adjunct only | held-untested | crates/deckmaste_lexical_source_v4/lexicon/english/prepositions.ron:185 frames FiniteClause/VerblessClause/Gerund, all hosts Predicate (so 'creature while saddled' as NP modifier and a LocativeComplement reading are not licensed); crates/deckmaste_english_v4/judgments/verbless_while_adjunct.txt (pending: COMPLETED_STRATUM=None) |
| `verbless_while_adjunct.rs::independent_complement_keeps_owner_roundtrips_and_lexical_licences` | saddled is a designation adjective usable in a verbless clause (VerblessPredicateUse) | held-untested | crates/deckmaste_lexical_source_v4/catalogs/designations.ron:28 (adjective); also keyword_verb:saddle participle; crates/deckmaste_english_v4/judgments/verbless_while_adjunct.txt (pending: COMPLETED_STRATUM=None) |
| `verbless_while_adjunct.rs::independent_complement_keeps_owner_roundtrips_and_lexical_licences` | PrepositionFunctionLicence feature on While (Modifier vs Adjunct function) | superseded | function licences replaced by per-frame host categories (Nominal/Predicate/ComplementOnly) in prepositions.ron header |
| `verbless_while_adjunct.rs::independent_complement_keeps_owner_roundtrips_and_lexical_licences` | tapped is a verbless predicate ('while tapped') | held-untested | crates/deckmaste_lexical_source_v4/catalogs/keyword_actions.ron:86 (tapped participle); crates/deckmaste_english_v4/judgments/verbless_while_adjunct.txt (pending: COMPLETED_STRATUM=None) |
| `vocabulary_batch_1.rs::vocabulary_only_witnesses_preserve_constituents` | move has no intransitive use ('This creature moves.' has no reading) | held-untested | crates/deckmaste_lexical_source_v4/lexicon/english/verbs.ron:81 frames Monotransitive, Prepositional([to]), TransitiveWithPP([onto]) -- no Intransitive; crates/deckmaste_english_v4/judgments/vocabulary_batch_1.txt (pending: COMPLETED_STRATUM=None) (negative entry) |
| `vocabulary_batch_1.rs::vocabulary_only_witnesses_preserve_constituents` | declared is declared vocabulary usable as SecondaryVerbPhrase (e.g. 'declared' on Rapid Fire) | held-untested | verb:declare at crates/deckmaste_lexical_source_v4/lexicon/english/verbs.ron:42; judgment vocabulary_batch_1.txt (pending) |
| `vocabulary_batch_1.rs::vocabulary_only_witnesses_preserve_constituents` | take is declared vocabulary usable as SecondaryVerbPhrase (e.g. 'Take an extra turn' on Time Walk) | held-untested | verb:take at crates/deckmaste_lexical_source_v4/lexicon/english/verbs.ron:127; judgment vocabulary_batch_1.txt (pending) |
| `vocabulary_batch_1.rs::vocabulary_only_witnesses_preserve_constituents` | pile is declared vocabulary usable as Nominal (e.g. 'pile' on Fact or Fiction) | held-untested | noun:pile at crates/deckmaste_lexical_source_v4/lexicon/english/nouns.ron:98; judgment vocabulary_batch_1.txt (pending) |
| `vocabulary_batch_1.rs::vocabulary_only_witnesses_preserve_constituents` | door is declared vocabulary usable as NounPhrase (e.g. 'this door' on Underwater Tunnel) | held-untested | noun:door at crates/deckmaste_lexical_source_v4/lexicon/english/nouns.ron:52; judgment vocabulary_batch_1.txt (pending) |
| `vocabulary_batch_1.rs::vocabulary_only_witnesses_preserve_constituents` | move is declared vocabulary usable as SecondaryVerbPhrase (e.g. 'Move all counters from target creature onto another target creature' on Fate Transfer) | held-untested | verb:move at crates/deckmaste_lexical_source_v4/lexicon/english/verbs.ron:81; judgment vocabulary_batch_1.txt (pending) |
| `vocabulary_batch_1.rs::vocabulary_only_witnesses_preserve_constituents` | excess is declared vocabulary usable as AdjectivePhrase (e.g. 'Excess' on Flame Spill) | held-untested | adj:excess at crates/deckmaste_lexical_source_v4/lexicon/english/adjectives.ron:16; judgment vocabulary_batch_1.txt (pending) |
| `vocabulary_batch_1.rs::vocabulary_only_witnesses_preserve_constituents` | deck is declared vocabulary usable as NounPhrase (e.g. 'A deck' on Rat Colony) | held-untested | noun:deck at crates/deckmaste_lexical_source_v4/lexicon/english/nouns.ron:47; judgment vocabulary_batch_1.txt (pending) |
| `vocabulary_batch_1.rs::vocabulary_only_witnesses_preserve_constituents` | opening is declared vocabulary usable as AdjectivePhrase (e.g. 'opening' on Leyline of Lightning) | superseded | no standalone `opening`: v4 declares the multiword noun 'opening hand' (crates/deckmaste_lexical_source_v4/lexicon/english/nouns.ron:160; comment: 'opening occurs in no other sense'); lexical analysis 'opening' unknown, 'your opening hand' analyses |
| `vocabulary_batch_1.rs::vocabulary_only_witnesses_preserve_constituents` | result is declared vocabulary usable as Nominal (e.g. 'result' on Clay Golem) | held-untested | noun:result at crates/deckmaste_lexical_source_v4/lexicon/english/nouns.ron:109; judgment vocabulary_batch_1.txt (pending) |
| `vocabulary_batch_1.rs::vocabulary_only_witnesses_preserve_constituents` | begin is declared vocabulary usable as SecondaryVerbPhrase (e.g. 'begin an extra turn' on Ugin's Nexus) | held-untested | verb:begin at crates/deckmaste_lexical_source_v4/lexicon/english/verbs.ron:15; judgment vocabulary_batch_1.txt (pending) |
| `vocabulary_batch_1.rs::vocabulary_only_witnesses_preserve_constituents` | secretly is declared vocabulary usable as Clause (e.g. 'secretly choose an opponent' on Guardian Archon) | held-untested | adv:secretly at crates/deckmaste_lexical_source_v4/lexicon/english/adverbs.ron:41; judgment vocabulary_batch_1.txt (pending) |
| `vocabulary_batch_1.rs::vocabulary_only_witnesses_preserve_constituents` | completed is declared vocabulary usable as PastParticiplePredicate (e.g. 'completed a dungeon' on Gloom Stalker) | held-untested | verb:complete at crates/deckmaste_lexical_source_v4/lexicon/english/verbs.ron:30; judgment vocabulary_batch_1.txt (pending) |
| `vocabulary_batch_1.rs::vocabulary_only_witnesses_preserve_constituents` | unblocked is declared vocabulary usable as AdjectivePhrase (e.g. 'unblocked' on Curtain of Light) | held-untested | adj:unblocked at crates/deckmaste_lexical_source_v4/lexicon/english/adjectives.ron:73; judgment vocabulary_batch_1.txt (pending) |
| `vocabulary_batch_1.rs::vocabulary_only_witnesses_preserve_constituents` | fully is declared vocabulary usable as FiniteClause (e.g. 'you fully unlock a Room' on Entity Tracker) | held-untested | adv:fully at crates/deckmaste_lexical_source_v4/lexicon/english/adverbs.ron:20; judgment vocabulary_batch_1.txt (pending) |
| `vocabulary_batch_1.rs::vocabulary_only_witnesses_preserve_constituents` | text is declared vocabulary usable as NominalCore (e.g. 'text' on Alter Reality) | held-untested | noun:text at crates/deckmaste_lexical_source_v4/lexicon/english/nouns.ron:126; judgment vocabulary_batch_1.txt (pending) |
| `vocabulary_batch_1.rs::vocabulary_only_witnesses_preserve_constituents` | unspent is declared vocabulary usable as AdjectivePhrase (e.g. 'unspent' on Omnath, Locus of Mana) | held-untested | adj:unspent at crates/deckmaste_lexical_source_v4/lexicon/english/adjectives.ron:76; judgment vocabulary_batch_1.txt (pending) |
| `vocabulary_batch_1.rs::vocabulary_only_witnesses_preserve_constituents` | note is declared vocabulary usable as SecondaryVerbPhrase (e.g. 'noted for it this turn' on Bell Borca, Spectral Sergeant) | held-untested | verb:note at crates/deckmaste_lexical_source_v4/lexicon/english/verbs.ron:85; judgment vocabulary_batch_1.txt (pending) |
| `vocabulary_batch_1.rs::vocabulary_only_witnesses_preserve_constituents` | instances is declared vocabulary usable as Nominal (e.g. 'instances' on Alter Reality) | held-untested | noun:instance at crates/deckmaste_lexical_source_v4/lexicon/english/nouns.ron:74; judgment vocabulary_batch_1.txt (pending) |
| `vocabulary_batch_1.rs::vocabulary_only_witnesses_preserve_constituents` | life total is declared vocabulary usable as NounPhrase (e.g. 'life totals' on Soul Conduit) | held-untested | noun:life total at crates/deckmaste_lexical_source_v4/lexicon/english/nouns.ron:156; judgment vocabulary_batch_1.txt (pending) |
| `vocabulary_batch_1.rs::vocabulary_only_witnesses_preserve_constituents` | team is declared vocabulary usable as NounPhrase (e.g. 'your team' on Rushblade Commander) | held-untested | noun:team at crates/deckmaste_lexical_source_v4/lexicon/english/nouns.ron:124; judgment vocabulary_batch_1.txt (pending) |
| `vocabulary_batch_1.rs::vocabulary_only_witnesses_preserve_constituents` | activation is declared vocabulary usable as NounPremodifier (e.g. 'activation' on Quicksilver Elemental) | held-untested | noun:activation at crates/deckmaste_lexical_source_v4/lexicon/english/nouns.ron:9; judgment vocabulary_batch_1.txt (pending) |
| `vocabulary_batch_1.rs::vocabulary_only_witnesses_preserve_constituents` | followed is declared vocabulary usable as SecondaryVerbPhrase (e.g. 'followed by an additional main phase' on Seize the Day) | held-untested | verb:follow at crates/deckmaste_lexical_source_v4/lexicon/english/verbs.ron:58; judgment vocabulary_batch_1.txt (pending) |
| `vocabulary_batch_1.rs::vocabulary_only_witnesses_preserve_constituents` | odd is declared vocabulary usable as AdjectivePhrase (e.g. 'odd' on Obosh, the Preypiercer) | held-untested | adj:odd at crates/deckmaste_lexical_source_v4/lexicon/english/adjectives.ron:45; judgment vocabulary_batch_1.txt (pending) |
| `vocabulary_batch_1.rs::vocabulary_only_witnesses_preserve_constituents` | even is declared vocabulary usable as AdjectivePhrase (e.g. 'even' on Gyruda, Doom of Depths) | held-untested | adj:even at crates/deckmaste_lexical_source_v4/lexicon/english/adjectives.ron:15; judgment vocabulary_batch_1.txt (pending) |
| `vocabulary_batch_1.rs::vocabulary_only_witnesses_preserve_constituents` | lowest is declared vocabulary usable as AdjectivePhrase (e.g. 'lowest' on Culling Scales) | held-untested | adj:lowest at crates/deckmaste_lexical_source_v4/lexicon/english/adjectives.ron:37; judgment vocabulary_batch_1.txt (pending) |
| `vocabulary_batch_1.rs::vocabulary_only_witnesses_preserve_constituents` | suspended is declared vocabulary usable as AdjectivePhrase (e.g. 'suspended' on Dust of Moments) | held-untested | keyword_verb:suspend at crates/deckmaste_lexical_source_v4/catalogs/keyword_abilities.ron:127; judgment vocabulary_batch_1.txt (pending) |
| `vocabulary_batch_1.rs::vocabulary_only_witnesses_preserve_constituents` | bargained is declared vocabulary usable as AdjectivePhrase (e.g. 'bargained' on Ice Out) | held-untested | keyword_verb:bargain at crates/deckmaste_lexical_source_v4/catalogs/keyword_abilities.ron:308; judgment vocabulary_batch_1.txt (pending) |
| `vocabulary_batch_1.rs::vocabulary_only_witnesses_preserve_constituents` | warped is declared vocabulary usable as AdjectivePhrase (e.g. 'warped' on Close Encounter) | held-untested | keyword_verb:warp at crates/deckmaste_lexical_source_v4/catalogs/keyword_abilities.ron:350; judgment vocabulary_batch_1.txt (pending) |
| `vocabulary_batch_1.rs::vocabulary_only_witnesses_preserve_constituents` | colored is declared vocabulary usable as AdjectivePhrase (e.g. 'colored' on Void Mirror) | held-untested | adj:colored at crates/deckmaste_lexical_source_v4/lexicon/english/adjectives.ron:10; judgment vocabulary_batch_1.txt (pending) |
| `vocabulary_batch_1.rs::vocabulary_only_witnesses_preserve_constituents` | hasn't/haven't is declared vocabulary usable as FinitePredicate (e.g. 'haven't cast a spell this turn' on Stoic Sphinx) | held-untested | verb:haven't at crates/deckmaste_lexical_source_v4/lexicon/english/closed_class.ron:141; judgment vocabulary_batch_1.txt (pending) |
| `vocabulary_batch_1.rs::vocabulary_only_witnesses_preserve_constituents` | identity is declared vocabulary usable as Nominal (e.g. 'color identity' on Command Tower) | held-untested | noun:color identity at crates/deckmaste_lexical_source_v4/lexicon/english/nouns.ron:150; judgment vocabulary_batch_1.txt (pending) |
| `vocabulary_batch_1.rs::vocabulary_only_witnesses_preserve_constituents` | symbols is declared vocabulary usable as Nominal (e.g. 'mana symbols' on Primalcrux) | held-untested | noun:symbol at crates/deckmaste_lexical_source_v4/lexicon/english/nouns.ron:120; judgment vocabulary_batch_1.txt (pending) |
| `vocabulary_batch_1.rs::vocabulary_only_witnesses_preserve_constituents` | kind is declared vocabulary usable as Nominal (e.g. 'kinds' on Hundred-Battle Veteran) | held-untested | noun:kind at crates/deckmaste_lexical_source_v4/lexicon/english/nouns.ron:76; judgment vocabulary_batch_1.txt (pending) |
| `vocabulary_batch_1.rs::vocabulary_only_witnesses_preserve_constituents` | highest is declared vocabulary usable as AdjectivePhrase (e.g. 'highest' on Arbiter of Knollridge) | held-untested | adj:highest at crates/deckmaste_lexical_source_v4/lexicon/english/adjectives.ron:27; judgment vocabulary_batch_1.txt (pending) |
| `vocabulary_batch_1.rs::vocabulary_only_witnesses_preserve_constituents` | separate is declared vocabulary usable as SecondaryVerbPhrase (e.g. 'Separate all permanents target player controls into two piles' on Liliana of the Veil) | held-untested | verb:separate at crates/deckmaste_lexical_source_v4/lexicon/english/verbs.ron:116; judgment vocabulary_batch_1.txt (pending) |
| `vocabulary_batch_1.rs::vocabulary_only_witnesses_preserve_constituents` | cadet is a Name introduced by card text (token name / option name) usable as Name (e.g. 'Cadet' on Craftwork Crusher) | gap | lexical analysis leaves the name unknown; no v4 catalog declares text-introduced names (tokens.ron has only the [CR#111.10] predefined tokens; card-names.txt only card names). v3 held these as lexeme:Name/* entries in crates/deckmaste_lexical_source/lexicon/core.ron |
| `vocabulary_batch_1.rs::vocabulary_only_witnesses_preserve_constituents` | starting is declared vocabulary usable as AdjectivePhrase (e.g. 'starting' on Angel of Destiny) | held-untested | verb:start at crates/deckmaste_lexical_source_v4/lexicon/english/verbs.ron:123; judgment vocabulary_batch_1.txt (pending) |
| `vocabulary_batch_1.rs::vocabulary_only_witnesses_preserve_constituents` | word is declared vocabulary usable as Nominal (e.g. 'color word' on Alter Reality) | held-untested | noun:word at crates/deckmaste_lexical_source_v4/lexicon/english/nouns.ron:139; judgment vocabulary_batch_1.txt (pending) |
| `vocabulary_batch_1.rs::vocabulary_only_witnesses_preserve_constituents` | right is declared vocabulary usable as Nominal (e.g. 'right' on Hithlain Rope) | held-untested | noun:right at crates/deckmaste_lexical_source_v4/lexicon/english/nouns.ron:110; judgment vocabulary_batch_1.txt (pending) |
| `vocabulary_batch_1.rs::vocabulary_only_witnesses_preserve_constituents` | switch is declared vocabulary usable as SecondaryVerbPhrase (e.g. 'Switch target creature's power and toughness' on Dwarven Thaumaturgist) | held-untested | verb:switch at crates/deckmaste_lexical_source_v4/lexicon/english/verbs.ron:126; judgment vocabulary_batch_1.txt (pending) |
| `vocabulary_batch_1.rs::vocabulary_only_witnesses_preserve_constituents` | friends is declared vocabulary usable as Nominal (e.g. 'friends' on Master of Ceremonies) | held-untested | noun:friend at crates/deckmaste_lexical_source_v4/lexicon/english/nouns.ron:68; judgment vocabulary_batch_1.txt (pending) |
| `vocabulary_batch_1.rs::vocabulary_only_witnesses_preserve_constituents` | beginning-phase V5 guard is declared vocabulary usable as Nominal (e.g. 'beginning phase' on Cyclonus, Cybertronian Fighter) | held-and-tested | turn_part:beginning phase at crates/deckmaste_lexical_source_v4/catalogs/turn_parts.ron:5; tested by crates/deckmaste_lexical_source_v4/tests/cr_coverage::phases_and_steps; judgment vocabulary_batch_1.txt (pending) |
| `vocabulary_batch_2.rs::vocabulary_only_witnesses_preserve_roles` | plotted is declared vocabulary usable as AdjectivePhrase (e.g. 'plotted' on Lilah, Undefeated Slickshot) | held-untested | keyword_verb:plot at crates/deckmaste_lexical_source_v4/catalogs/keyword_abilities.ron:321; judgment vocabulary_batch_2.txt (pending) |
| `vocabulary_batch_2.rs::vocabulary_only_witnesses_preserve_roles` | reselect is declared vocabulary usable as SecondaryVerbPhrase (e.g. 'reselect the targets' on Chef's Kiss) | held-untested | verb:reselect at crates/deckmaste_lexical_source_v4/lexicon/english/verbs.ron:107; judgment vocabulary_batch_2.txt (pending) |
| `vocabulary_batch_2.rs::vocabulary_only_witnesses_preserve_roles` | count is declared vocabulary usable as SecondaryVerbPhrase (e.g. 'count the number of permanents you control' on Touch of the Eternal) | held-untested | verb:count at crates/deckmaste_lexical_source_v4/lexicon/english/verbs.ron:36; judgment vocabulary_batch_2.txt (pending) |
| `vocabulary_batch_2.rs::vocabulary_only_witnesses_preserve_roles` | khans is a Name introduced by card text (token name / option name) usable as NounPhrase (e.g. 'Khans' on Monastery Siege) | gap | lexical analysis leaves the name unknown; no v4 catalog declares text-introduced names (tokens.ron has only the [CR#111.10] predefined tokens; card-names.txt only card names). v3 held these as lexeme:Name/* entries in crates/deckmaste_lexical_source/lexicon/core.ron |
| `vocabulary_batch_2.rs::vocabulary_only_witnesses_preserve_roles` | heartwood is declared vocabulary usable as NounPremodifier (e.g. 'Heartwood' on Aerid Konstrari) | held-and-tested | subtype:Heartwood at crates/deckmaste_lexical_source_v4/catalogs/subtypes.ron:17; tested by crates/deckmaste_lexical_source_v4/tests/cr_coverage::subtypes_by_family; judgment vocabulary_batch_2.txt (pending) |
| `vocabulary_batch_2.rs::vocabulary_only_witnesses_preserve_roles` | double-faced is declared vocabulary usable as AdjectivePhrase (e.g. 'double-faced' on Invasion of Pyrulea) | held-untested | adj:double-faced at crates/deckmaste_lexical_source_v4/lexicon/english/adjectives.ron:13; judgment vocabulary_batch_2.txt (pending) |
| `vocabulary_batch_2.rs::vocabulary_only_witnesses_preserve_roles` | 'Twin' / 'Stangg Twin' are Names (tokens introduced by card text: Gemini Engine 'token named Twin', Stangg 'create Stangg Twin') | gap | lexical analysis leaves the name unknown; no v4 catalog declares text-introduced names (tokens.ron has only the [CR#111.10] predefined tokens; card-names.txt only card names). v3 held these as lexeme:Name/* entries in crates/deckmaste_lexical_source/lexicon/core.ron |
| `vocabulary_batch_2.rs::vocabulary_only_witnesses_preserve_roles` | walker is declared vocabulary usable as NounPremodifier (e.g. 'Walker' on Enkira, Hostile Scavenger) | held-and-tested | token:Walker at crates/deckmaste_lexical_source_v4/catalogs/tokens.ron:7; tested by crates/deckmaste_lexical_source_v4/tests/cr_coverage::predefined_tokens; judgment vocabulary_batch_2.txt (pending) |
| `vocabulary_batch_2.rs::vocabulary_only_witnesses_preserve_roles` | active is declared vocabulary usable as AdjectivePhrase (e.g. 'active' on Siren's Call) | held-untested | adj:active at crates/deckmaste_lexical_source_v4/lexicon/english/adjectives.ron:6; judgment vocabulary_batch_2.txt (pending) |
| `vocabulary_batch_2.rs::vocabulary_only_witnesses_preserve_roles` | give is declared vocabulary usable as SecondaryVerbPhrase (e.g. 'give that permanent or player another counter of that kind' on Powerful Broker) | held-untested | verb:give at crates/deckmaste_lexical_source_v4/lexicon/english/verbs.ron:63; judgment vocabulary_batch_2.txt (pending) |
| `vocabulary_batch_2.rs::vocabulary_only_witnesses_preserve_roles` | protect is declared vocabulary usable as FinitePredicate (e.g. 'protects it' on Etched Host Doombringer) | held-untested | verb:protect at crates/deckmaste_lexical_source_v4/lexicon/english/verbs.ron:97; judgment vocabulary_batch_2.txt (pending) |
| `vocabulary_batch_2.rs::vocabulary_only_witnesses_preserve_roles` | axe is a Name introduced by card text (token name / option name) usable as Name (e.g. 'Axe' on Dáin Ironfoot) | gap | lexical analysis leaves the name unknown; no v4 catalog declares text-introduced names (tokens.ron has only the [CR#111.10] predefined tokens; card-names.txt only card names). v3 held these as lexeme:Name/* entries in crates/deckmaste_lexical_source/lexicon/core.ron |
| `vocabulary_batch_2.rs::vocabulary_only_witnesses_preserve_roles` | boo is a Name introduced by card text (token name / option name) usable as NounPhrase (e.g. 'Boo' on Minsc, Beloved Ranger) | gap | lexical analysis leaves the name unknown; no v4 catalog declares text-introduced names (tokens.ron has only the [CR#111.10] predefined tokens; card-names.txt only card names). v3 held these as lexeme:Name/* entries in crates/deckmaste_lexical_source/lexicon/core.ron |
| `vocabulary_batch_2.rs::vocabulary_only_witnesses_preserve_roles` | doombot is a Name introduced by card text (token name / option name) usable as Name (e.g. 'Doombot' on Castle Doom) | gap | lexical analysis leaves the name unknown; no v4 catalog declares text-introduced names (tokens.ron has only the [CR#111.10] predefined tokens; card-names.txt only card names). v3 held these as lexeme:Name/* entries in crates/deckmaste_lexical_source/lexicon/core.ron |
| `vocabulary_batch_2.rs::vocabulary_only_witnesses_preserve_roles` | moloid is a Name introduced by card text (token name / option name) usable as Name (e.g. 'Moloid' on Mole Man, Moloid Master) | gap | lexical analysis leaves the name unknown; no v4 catalog declares text-introduced names (tokens.ron has only the [CR#111.10] predefined tokens; card-names.txt only card names). v3 held these as lexeme:Name/* entries in crates/deckmaste_lexical_source/lexicon/core.ron |
| `vocabulary_batch_2.rs::vocabulary_only_witnesses_preserve_roles` | reorder is declared vocabulary usable as SecondaryVerbPhrase (e.g. 'Reorder your graveyard' on Search for Survivors) | held-untested | verb:reorder at crates/deckmaste_lexical_source_v4/lexicon/english/verbs.ron:104; judgment vocabulary_batch_2.txt (pending) |
| `vocabulary_batch_2.rs::vocabulary_only_witnesses_preserve_roles` | sturdy is a Name introduced by card text (token name / option name) usable as Name (e.g. 'Sturdy Shield' on U.S.Agent, John Walker) | gap | lexical analysis leaves the name unknown; no v4 catalog declares text-introduced names (tokens.ron has only the [CR#111.10] predefined tokens; card-names.txt only card names). v3 held these as lexeme:Name/* entries in crates/deckmaste_lexical_source/lexicon/core.ron |
| `vocabulary_batch_2.rs::vocabulary_only_witnesses_preserve_roles` | simultaneously is declared vocabulary usable as AdverbPhrase (e.g. 'simultaneously' on Goblin Game) | held-untested | adv:simultaneously at crates/deckmaste_lexical_source_v4/lexicon/english/adverbs.ron:43; judgment vocabulary_batch_2.txt (pending) |
| `vocabulary_batch_2.rs::vocabulary_only_witnesses_preserve_roles` | quality is declared vocabulary usable as NounPhrase (e.g. 'the chosen quality' on Thanos, the Mad Titan) | held-untested | noun:quality at crates/deckmaste_lexical_source_v4/lexicon/english/nouns.ron:105; judgment vocabulary_batch_2.txt (pending) |
| `vocabulary_batch_2.rs::vocabulary_only_witnesses_preserve_roles` | direction is declared vocabulary usable as NounPhrase (e.g. 'the last chosen direction' on Mystic Barrier) | held-untested | noun:direction at crates/deckmaste_lexical_source_v4/lexicon/english/nouns.ron:51; judgment vocabulary_batch_2.txt (pending) |
| `vocabulary_batch_2.rs::vocabulary_only_witnesses_preserve_roles` | foe is declared vocabulary usable as NounPhrase (e.g. 'Each foe' on Pir's Whim) | held-untested | noun:foe at crates/deckmaste_lexical_source_v4/lexicon/english/nouns.ron:67; judgment vocabulary_batch_2.txt (pending) |
| `vocabulary_batch_2.rs::vocabulary_only_witnesses_preserve_roles` | bid is declared vocabulary usable as SecondaryVerbPhrase (e.g. 'bid life' on Pain's Reward) | held-untested | verb:bid at crates/deckmaste_lexical_source_v4/lexicon/english/verbs.ron:17; judgment vocabulary_batch_2.txt (pending) |
| `vocabulary_batch_2.rs::vocabulary_only_witnesses_preserve_roles` | bidding is declared vocabulary usable as Nominal (e.g. 'bidding' on Pain's Reward) | held-untested | verb:bid at crates/deckmaste_lexical_source_v4/lexicon/english/verbs.ron:17; judgment vocabulary_batch_2.txt (pending) |
| `vocabulary_batch_2.rs::vocabulary_only_witnesses_preserve_roles` | chapter is declared vocabulary usable as NounPremodifier (e.g. 'chapter' on Narci, Fable Singer) | held-untested | noun:chapter at crates/deckmaste_lexical_source_v4/lexicon/english/nouns.ron:25; judgment vocabulary_batch_2.txt (pending) |
| `vocabulary_batch_2.rs::vocabulary_only_witnesses_preserve_roles` | nearest is declared vocabulary usable as AdjectivePhrase (e.g. 'nearest' on Mystic Barrier) | held-untested | adj:nearest at crates/deckmaste_lexical_source_v4/lexicon/english/adjectives.ron:41; judgment vocabulary_batch_2.txt (pending) |
| `vocabulary_batch_2.rs::vocabulary_only_witnesses_preserve_roles` | money is declared vocabulary usable as NounPhrase (e.g. 'money' on Master of Ceremonies) | held-untested | noun:money at crates/deckmaste_lexical_source_v4/lexicon/english/nouns.ron:183; judgment vocabulary_batch_2.txt (pending) |
| `vocabulary_batch_2.rs::vocabulary_only_witnesses_preserve_roles` | voter is declared vocabulary usable as NounPhrase (e.g. 'the voter' on Elrond of the White Council) | held-untested | noun:voter at crates/deckmaste_lexical_source_v4/lexicon/english/nouns.ron:137; judgment vocabulary_batch_2.txt (pending) |
| `vocabulary_batch_2.rs::vocabulary_only_witnesses_preserve_roles` | final is declared vocabulary usable as AdjectivePhrase (e.g. 'final' on Narci, Fable Singer) | held-untested | adj:final at crates/deckmaste_lexical_source_v4/lexicon/english/adjectives.ron:20; judgment vocabulary_batch_2.txt (pending) |
| `vocabulary_batch_2.rs::vocabulary_only_witnesses_preserve_roles` | high is declared vocabulary usable as AdjectivePhrase (e.g. 'high' on Pain's Reward) | held-untested | adj:high at crates/deckmaste_lexical_source_v4/lexicon/english/adjectives.ron:27; judgment vocabulary_batch_2.txt (pending) |
| `vocabulary_batch_2.rs::vocabulary_only_witnesses_preserve_roles` | bidder is declared vocabulary usable as NounPhrase (e.g. 'The high bidder' on Pain's Reward) | held-untested | noun:bidder at crates/deckmaste_lexical_source_v4/lexicon/english/nouns.ron:19; judgment vocabulary_batch_2.txt (pending) |
| `vocabulary_batch_2.rs::vocabulary_only_witnesses_preserve_roles` | affect is declared vocabulary usable as SecondaryVerbPhrase (e.g. 'affect its color identity' on Fallaji Wayfarer) | held-untested | verb:affect at crates/deckmaste_lexical_source_v4/lexicon/english/verbs.ron:8; judgment vocabulary_batch_2.txt (pending) |
| `vocabulary_batch_2.rs::vocabulary_only_witnesses_preserve_roles` | above is declared vocabulary usable as PrepositionPhrase (e.g. 'above this card' on Ashen Ghoul) | held-untested | prep:above at crates/deckmaste_lexical_source_v4/lexicon/english/prepositions.ron:15; judgment vocabulary_batch_2.txt (pending) |
| `vocabulary_batch_2.rs::vocabulary_only_witnesses_preserve_roles` | natural is declared vocabulary usable as AdjectivePhrase (e.g. 'natural' on Netherese Puzzle-Ward) | held-untested | adj:natural at crates/deckmaste_lexical_source_v4/lexicon/english/adjectives.ron:40; judgment vocabulary_batch_2.txt (pending) |
| `vocabulary_batch_2.rs::vocabulary_only_witnesses_preserve_roles` | lethal is declared vocabulary usable as AdjectivePhrase (e.g. 'lethal' on Ogre Enforcer) | held-untested | adj:lethal at crates/deckmaste_lexical_source_v4/lexicon/english/adjectives.ron:35; judgment vocabulary_batch_2.txt (pending) |
| `vocabulary_batch_2.rs::vocabulary_only_witnesses_preserve_roles` | receive is declared vocabulary usable as FiniteObjectGap (e.g. 'received' on Círdan the Shipwright) | held-untested | verb:receive at crates/deckmaste_lexical_source_v4/lexicon/english/verbs.ron:99; judgment vocabulary_batch_2.txt (pending) |
| `vocabulary_batch_2.rs::vocabulary_only_witnesses_preserve_roles` | yet is declared vocabulary usable as AdverbPhrase (e.g. 'yet' on Karakyk Guardian) | held-untested | adv:yet at crates/deckmaste_lexical_source_v4/lexicon/english/adverbs.ron:50; judgment vocabulary_batch_2.txt (pending) |
| `vocabulary_batch_2.rs::vocabulary_only_witnesses_preserve_roles` | guess is declared vocabulary usable as FinitePredicate (e.g. 'guessed' on Gollum, Scheming Guide) | held-untested | verb:guess at crates/deckmaste_lexical_source_v4/lexicon/english/verbs.ron:65; judgment vocabulary_batch_2.txt (pending) |
| `vocabulary_batch_2.rs::vocabulary_only_witnesses_preserve_roles` | ballistic boulder is a Name introduced by card text (token name / option name) usable as Name (e.g. 'Ballistic Boulder' on Fire Navy Trebuchet) | gap | lexical analysis leaves the name unknown; no v4 catalog declares text-introduced names (tokens.ron has only the [CR#111.10] predefined tokens; card-names.txt only card names). v3 held these as lexeme:Name/* entries in crates/deckmaste_lexical_source/lexicon/core.ron |
| `vocabulary_batch_2.rs::vocabulary_only_witnesses_preserve_roles` | cordyceps infected is a Name introduced by card text (token name / option name) usable as Name (e.g. 'Cordyceps Infected' on Ellie, Brick Master) | gap | lexical analysis leaves the name unknown; no v4 catalog declares text-introduced names (tokens.ron has only the [CR#111.10] predefined tokens; card-names.txt only card names). v3 held these as lexeme:Name/* entries in crates/deckmaste_lexical_source/lexicon/core.ron |
| `vocabulary_batch_2.rs::vocabulary_only_witnesses_preserve_roles` | koma's coil is a Name introduced by card text (token name / option name) usable as Name (e.g. 'Koma's Coil' on Koma, Cosmos Serpent) | gap | lexical analysis leaves the name unknown; no v4 catalog declares text-introduced names (tokens.ron has only the [CR#111.10] predefined tokens; card-names.txt only card names). v3 held these as lexeme:Name/* entries in crates/deckmaste_lexical_source/lexicon/core.ron |
| `vocabulary_batch_2.rs::vocabulary_only_witnesses_preserve_roles` | plaguebearer of nurgle is a Name introduced by card text (token name / option name) usable as Name (e.g. 'Plaguebearer of Nurgle' on Nurgle's Rot) | gap | lexical analysis leaves the name unknown; no v4 catalog declares text-introduced names (tokens.ron has only the [CR#111.10] predefined tokens; card-names.txt only card names). v3 held these as lexeme:Name/* entries in crates/deckmaste_lexical_source/lexicon/core.ron |
| `vocabulary_batch_2.rs::vocabulary_only_witnesses_preserve_roles` | marit lage is a Name introduced by card text (token name / option name) usable as NounPhrase (e.g. 'Marit Lage' on Marit Lage's Slumber) | gap | lexical analysis leaves the name unknown; no v4 catalog declares text-introduced names (tokens.ron has only the [CR#111.10] predefined tokens; card-names.txt only card names). v3 held these as lexeme:Name/* entries in crates/deckmaste_lexical_source/lexicon/core.ron |
| `vocabulary_batch_2.rs::vocabulary_only_witnesses_preserve_roles` | vecna is a Name introduced by card text (token name / option name) usable as NounPhrase (e.g. 'Vecna' on The Book of Vile Darkness) | gap | lexical analysis leaves the name unknown; no v4 catalog declares text-introduced names (tokens.ron has only the [CR#111.10] predefined tokens; card-names.txt only card names). v3 held these as lexeme:Name/* entries in crates/deckmaste_lexical_source/lexicon/core.ron |
| `vocabulary_batch_2.rs::vocabulary_only_witnesses_preserve_roles` | fewest is declared vocabulary usable as Nominal (e.g. 'fewest creatures' on Gor Muldrak, Amphinologist) | held-untested | det:fewest at crates/deckmaste_lexical_source_v4/lexicon/english/closed_class.ron:19; judgment vocabulary_batch_2.txt (pending) |
| `vocabulary_batch_2.rs::vocabulary_only_witnesses_preserve_roles` | wrong is declared vocabulary usable as AdverbPhrase (e.g. 'wrong' on The Seventh Doctor) | held-untested | adj:wrong at crates/deckmaste_lexical_source_v4/lexicon/english/adjectives.ron:81; judgment vocabulary_batch_2.txt (pending) |
| `vocabulary_batch_2.rs::vocabulary_only_witnesses_preserve_roles` | following is declared vocabulary usable as AdjectivePhrase (e.g. 'following' on Torment of Hailfire) | held-untested | verb:follow at crates/deckmaste_lexical_source_v4/lexicon/english/verbs.ron:58; judgment vocabulary_batch_2.txt (pending) |
| `vocabulary_batch_2.rs::vocabulary_only_witnesses_preserve_roles` | characteristic is declared vocabulary usable as NounPhrase (e.g. 'those characteristics' on Outlaws' Merriment) | held-untested | noun:characteristic at crates/deckmaste_lexical_source_v4/lexicon/english/nouns.ron:26; judgment vocabulary_batch_2.txt (pending) |
| `vocabulary_batch_2.rs::vocabulary_only_witnesses_preserve_roles` | head is declared vocabulary usable as Nominal (e.g. 'heads' on Molten Sentry) | held-untested | noun:head at crates/deckmaste_lexical_source_v4/lexicon/english/nouns.ron:73; judgment vocabulary_batch_2.txt (pending) |
| `vocabulary_batch_2.rs::vocabulary_only_witnesses_preserve_roles` | tail is declared vocabulary usable as Nominal (e.g. 'tails' on Molten Sentry) | held-untested | noun:tail at crates/deckmaste_lexical_source_v4/lexicon/english/nouns.ron:121; judgment vocabulary_batch_2.txt (pending) |
| `vocabulary_batch_2.rs::vocabulary_only_witnesses_preserve_roles` | beneath is declared vocabulary usable as PrepositionPhrase (e.g. 'beneath the top X cards of that library' on Quarry Colossus) | held-untested | prep:beneath at crates/deckmaste_lexical_source_v4/lexicon/english/prepositions.ron:56; judgment vocabulary_batch_2.txt (pending) |
| `vocabulary_batch_2.rs::vocabulary_only_witnesses_preserve_roles` | lock is declared vocabulary usable as SecondaryVerbPhrase (e.g. 'Lock or unlock a door of target Room you control' on Marina Vendrell) | held-untested | verb:lock at crates/deckmaste_lexical_source_v4/lexicon/english/verbs.ron:72; judgment vocabulary_batch_2.txt (pending) |
| `vocabulary_batch_2.rs::vocabulary_only_witnesses_preserve_roles` | mark is declared vocabulary usable as SecondaryVerbPhrase (e.g. 'marked' on Ogre Enforcer) | held-untested | verb:mark at crates/deckmaste_lexical_source_v4/lexicon/english/verbs.ron:77; judgment vocabulary_batch_2.txt (pending) |
| `vocabulary_batch_2.rs::vocabulary_only_witnesses_preserve_roles` | abzan is a Name introduced by card text (token name / option name) usable as Name (e.g. 'Abzan' on Barrensteppe Siege) | gap | lexical analysis leaves the name unknown; no v4 catalog declares text-introduced names (tokens.ron has only the [CR#111.10] predefined tokens; card-names.txt only card names). v3 held these as lexeme:Name/* entries in crates/deckmaste_lexical_source/lexicon/core.ron |
| `vocabulary_batch_2.rs::vocabulary_only_witnesses_preserve_roles` | mardu is a Name introduced by card text (token name / option name) usable as Name (e.g. 'Mardu' on Barrensteppe Siege) | gap | lexical analysis leaves the name unknown; no v4 catalog declares text-introduced names (tokens.ron has only the [CR#111.10] predefined tokens; card-names.txt only card names). v3 held these as lexeme:Name/* entries in crates/deckmaste_lexical_source/lexicon/core.ron |
| `vocabulary_batch_2.rs::vocabulary_only_witnesses_preserve_roles` | jeskai is a Name introduced by card text (token name / option name) usable as Name (e.g. 'Jeskai' on Windcrag Siege) | gap | lexical analysis leaves the name unknown; no v4 catalog declares text-introduced names (tokens.ron has only the [CR#111.10] predefined tokens; card-names.txt only card names). v3 held these as lexeme:Name/* entries in crates/deckmaste_lexical_source/lexicon/core.ron |
| `vocabulary_batch_2.rs::vocabulary_only_witnesses_preserve_roles` | sultai is a Name introduced by card text (token name / option name) usable as Name (e.g. 'Sultai' on Hollowmurk Siege) | gap | lexical analysis leaves the name unknown; no v4 catalog declares text-introduced names (tokens.ron has only the [CR#111.10] predefined tokens; card-names.txt only card names). v3 held these as lexeme:Name/* entries in crates/deckmaste_lexical_source/lexicon/core.ron |
| `vocabulary_batch_2.rs::vocabulary_only_witnesses_preserve_roles` | temur is a Name introduced by card text (token name / option name) usable as Name (e.g. 'Temur' on Glacierwood Siege) | gap | lexical analysis leaves the name unknown; no v4 catalog declares text-introduced names (tokens.ron has only the [CR#111.10] predefined tokens; card-names.txt only card names). v3 held these as lexeme:Name/* entries in crates/deckmaste_lexical_source/lexicon/core.ron |
| `vocabulary_batch_2.rs::vocabulary_only_witnesses_preserve_roles` | redhorn pass is a Name introduced by card text (token name / option name) usable as Name (e.g. 'Redhorn Pass' on Travel Through Caradhras) | gap | lexical analysis leaves the name unknown; no v4 catalog declares text-introduced names (tokens.ron has only the [CR#111.10] predefined tokens; card-names.txt only card names). v3 held these as lexeme:Name/* entries in crates/deckmaste_lexical_source/lexicon/core.ron |
| `vocabulary_batch_2.rs::vocabulary_only_witnesses_preserve_roles` | jaces is declared vocabulary usable as Nominal (e.g. 'Jaces you control' on Jace, Multiverse Architect) | held-and-tested | subtype:Jace at crates/deckmaste_lexical_source_v4/catalogs/subtypes.ron:433; tested by crates/deckmaste_lexical_source_v4/tests/cr_coverage::subtypes_by_family; judgment vocabulary_batch_2.txt (pending) |
| `vocabulary_batch_2.rs::vocabulary_only_witnesses_preserve_roles` | angelo is a Name introduced by card text (token name / option name) usable as NounPhrase (e.g. 'Angelo' on Rinoa Heartilly) | gap | lexical analysis leaves the name unknown; no v4 catalog declares text-introduced names (tokens.ron has only the [CR#111.10] predefined tokens; card-names.txt only card names). v3 held these as lexeme:Name/* entries in crates/deckmaste_lexical_source/lexicon/core.ron |
| `vocabulary_batch_2.rs::vocabulary_only_witnesses_preserve_roles` | banana is a Name introduced by card text (token name / option name) usable as Name (e.g. 'Banana' on Kibo, Uktabi Prince) | gap | lexical analysis leaves the name unknown; no v4 catalog declares text-introduced names (tokens.ron has only the [CR#111.10] predefined tokens; card-names.txt only card names). v3 held these as lexeme:Name/* entries in crates/deckmaste_lexical_source/lexicon/core.ron |
| `vocabulary_batch_2.rs::vocabulary_only_witnesses_preserve_roles` | butterfly is a Name introduced by card text (token name / option name) usable as Name (e.g. 'Butterfly' on Giant Caterpillar) | gap | lexical analysis leaves the name unknown; no v4 catalog declares text-introduced names (tokens.ron has only the [CR#111.10] predefined tokens; card-names.txt only card names). v3 held these as lexeme:Name/* entries in crates/deckmaste_lexical_source/lexicon/core.ron |
| `vocabulary_batch_2.rs::vocabulary_only_witnesses_preserve_roles` | carnivore is a Name introduced by card text (token name / option name) usable as Name (e.g. 'Carnivore' on Tooth and Claw) | gap | lexical analysis leaves the name unknown; no v4 catalog declares text-introduced names (tokens.ron has only the [CR#111.10] predefined tokens; card-names.txt only card names). v3 held these as lexeme:Name/* entries in crates/deckmaste_lexical_source/lexicon/core.ron |
| `vocabulary_batch_2.rs::vocabulary_only_witnesses_preserve_roles` | contract is a Name introduced by card text (token name / option name) usable as Name (e.g. 'Contract' on Scriv, the Obligator) | gap | lexical analysis leaves the name unknown; no v4 catalog declares text-introduced names (tokens.ron has only the [CR#111.10] predefined tokens; card-names.txt only card names). v3 held these as lexeme:Name/* entries in crates/deckmaste_lexical_source/lexicon/core.ron |
| `vocabulary_batch_2.rs::vocabulary_only_witnesses_preserve_roles` | cragflame is a Name introduced by card text (token name / option name) usable as NounPhrase (e.g. 'Cragflame' on Mabel, Heir to Cragflame) | gap | lexical analysis leaves the name unknown; no v4 catalog declares text-introduced names (tokens.ron has only the [CR#111.10] predefined tokens; card-names.txt only card names). v3 held these as lexeme:Name/* entries in crates/deckmaste_lexical_source/lexicon/core.ron |
| `vocabulary_batch_2.rs::vocabulary_only_witnesses_preserve_roles` | darkstar is a Name introduced by card text (token name / option name) usable as NounPhrase (e.g. 'Darkstar' on Rufus Shinra) | gap | lexical analysis leaves the name unknown; no v4 catalog declares text-introduced names (tokens.ron has only the [CR#111.10] predefined tokens; card-names.txt only card names). v3 held these as lexeme:Name/* entries in crates/deckmaste_lexical_source/lexicon/core.ron |
| `vocabulary_batch_2.rs::vocabulary_only_witnesses_preserve_roles` | humble merchant is a Name introduced by card text (token name / option name) usable as Name (e.g. 'Humble Merchant' on Honest Work) | gap | lexical analysis leaves the name unknown; no v4 catalog declares text-introduced names (tokens.ron has only the [CR#111.10] predefined tokens; card-names.txt only card names). v3 held these as lexeme:Name/* entries in crates/deckmaste_lexical_source/lexicon/core.ron |
| `vocabulary_batch_2.rs::vocabulary_only_witnesses_preserve_roles` | kaldra is a Name introduced by card text (token name / option name) usable as NounPhrase (e.g. 'Kaldra' on Helm of Kaldra) | gap | lexical analysis leaves the name unknown; no v4 catalog declares text-introduced names (tokens.ron has only the [CR#111.10] predefined tokens; card-names.txt only card names). v3 held these as lexeme:Name/* entries in crates/deckmaste_lexical_source/lexicon/core.ron |
| `vocabulary_batch_2.rs::vocabulary_only_witnesses_preserve_roles` | keimi is a Name introduced by card text (token name / option name) usable as NounPhrase (e.g. 'Keimi' on Tatsunari, Toad Rider) | gap | lexical analysis leaves the name unknown; no v4 catalog declares text-introduced names (tokens.ron has only the [CR#111.10] predefined tokens; card-names.txt only card names). v3 held these as lexeme:Name/* entries in crates/deckmaste_lexical_source/lexicon/core.ron |
| `vocabulary_batch_2.rs::vocabulary_only_witnesses_preserve_roles` | kelp is a Name introduced by card text (token name / option name) usable as Name (e.g. 'Kelp' on Wall of Kelp) | gap | lexical analysis leaves the name unknown; no v4 catalog declares text-introduced names (tokens.ron has only the [CR#111.10] predefined tokens; card-names.txt only card names). v3 held these as lexeme:Name/* entries in crates/deckmaste_lexical_source/lexicon/core.ron |
| `vocabulary_batch_2.rs::vocabulary_only_witnesses_preserve_roles` | lotus is a Name introduced by card text (token name / option name) usable as Name (e.g. 'Lotus' on Kwia Vigorbloom) | gap | lexical analysis leaves the name unknown; no v4 catalog declares text-introduced names (tokens.ron has only the [CR#111.10] predefined tokens; card-names.txt only card names). v3 held these as lexeme:Name/* entries in crates/deckmaste_lexical_source/lexicon/core.ron |
| `vocabulary_batch_2.rs::vocabulary_only_witnesses_preserve_roles` | munitions is a Name introduced by card text (token name / option name) usable as Name (e.g. 'Munitions' on Weapons Manufacturing) | gap | lexical analysis leaves the name unknown; no v4 catalog declares text-introduced names (tokens.ron has only the [CR#111.10] predefined tokens; card-names.txt only card names). v3 held these as lexeme:Name/* entries in crates/deckmaste_lexical_source/lexicon/core.ron |
| `vocabulary_batch_2.rs::vocabulary_only_witnesses_preserve_roles` | tamiyo's notebook is a Name introduced by card text (token name / option name) usable as NounPhrase (e.g. 'Tamiyo's Notebook' on Tamiyo, Compleated Sage) | gap | lexical analysis leaves the name unknown; no v4 catalog declares text-introduced names (tokens.ron has only the [CR#111.10] predefined tokens; card-names.txt only card names). v3 held these as lexeme:Name/* entries in crates/deckmaste_lexical_source/lexicon/core.ron |
| `vocabulary_batch_2.rs::vocabulary_only_witnesses_preserve_roles` | jumblebones is a Name introduced by card text (token name / option name) usable as NounPhrase (e.g. 'Jumblebones' on Ozox, the Clattering King) | gap | lexical analysis leaves the name unknown; no v4 catalog declares text-introduced names (tokens.ron has only the [CR#111.10] predefined tokens; card-names.txt only card names). v3 held these as lexeme:Name/* entries in crates/deckmaste_lexical_source/lexicon/core.ron |
| `vocabulary_batch_2.rs::vocabulary_only_witnesses_preserve_roles` | icy manalith is a Name introduced by card text (token name / option name) usable as Name (e.g. 'Icy Manalith' on Svella, Ice Shaper) | gap | lexical analysis leaves the name unknown; no v4 catalog declares text-introduced names (tokens.ron has only the [CR#111.10] predefined tokens; card-names.txt only card names). v3 held these as lexeme:Name/* entries in crates/deckmaste_lexical_source/lexicon/core.ron |
| `vocabulary_batch_2.rs::vocabulary_only_witnesses_preserve_roles` | stoneforged blade is a Name introduced by card text (token name / option name) usable as Name (e.g. 'Stoneforged Blade' on Nahiri, the Lithomancer) | gap | lexical analysis leaves the name unknown; no v4 catalog declares text-introduced names (tokens.ron has only the [CR#111.10] predefined tokens; card-names.txt only card names). v3 held these as lexeme:Name/* entries in crates/deckmaste_lexical_source/lexicon/core.ron |
| `vocabulary_batch_2.rs::vocabulary_only_witnesses_preserve_roles` | lightning rager is a Name introduced by card text (token name / option name) usable as Name (e.g. 'Lightning Rager' on Rite of the Raging Storm) | gap | lexical analysis leaves the name unknown; no v4 catalog declares text-introduced names (tokens.ron has only the [CR#111.10] predefined tokens; card-names.txt only card names). v3 held these as lexeme:Name/* entries in crates/deckmaste_lexical_source/lexicon/core.ron |
| `vocabulary_batch_2.rs::vocabulary_only_witnesses_preserve_roles` | vitu-ghazi is a Name introduced by card text (token name / option name) usable as Name (e.g. 'Vitu-Ghazi' on Awakening of Vitu-Ghazi) | gap | lexical analysis leaves the name unknown; no v4 catalog declares text-introduced names (tokens.ron has only the [CR#111.10] predefined tokens; card-names.txt only card names). v3 held these as lexeme:Name/* entries in crates/deckmaste_lexical_source/lexicon/core.ron |
| `vocabulary_batch_2.rs::vocabulary_only_witnesses_preserve_roles` | mishra's warform is a Name introduced by card text (token name / option name) usable as NounPhrase (e.g. 'Mishra's Warform' on Mishra, Eminent One) | gap | lexical analysis leaves the name unknown; no v4 catalog declares text-introduced names (tokens.ron has only the [CR#111.10] predefined tokens; card-names.txt only card names). v3 held these as lexeme:Name/* entries in crates/deckmaste_lexical_source/lexicon/core.ron |
| `vocabulary_batch_2.rs::vocabulary_only_witnesses_preserve_roles` | mechtitan is a Name introduced by card text (token name / option name) usable as NounPhrase (e.g. 'Mechtitan' on Mechtitan Core) | gap | lexical analysis leaves the name unknown; no v4 catalog declares text-introduced names (tokens.ron has only the [CR#111.10] predefined tokens; card-names.txt only card names). v3 held these as lexeme:Name/* entries in crates/deckmaste_lexical_source/lexicon/core.ron |
| `vocabulary_batch_2.rs::vocabulary_only_witnesses_preserve_roles` | zabu is a Name introduced by card text (token name / option name) usable as NounPhrase (e.g. 'Zabu' on Ka-Zar of the Savage Land) | gap | lexical analysis leaves the name unknown; no v4 catalog declares text-introduced names (tokens.ron has only the [CR#111.10] predefined tokens; card-names.txt only card names). v3 held these as lexeme:Name/* entries in crates/deckmaste_lexical_source/lexicon/core.ron |
| `vocabulary_batch_2.rs::vocabulary_only_witnesses_preserve_roles` | hornet is a Name introduced by card text (token name / option name) usable as Name (e.g. 'Hornet' on Hornet Cannon) | gap | lexical analysis leaves the name unknown; no v4 catalog declares text-introduced names (tokens.ron has only the [CR#111.10] predefined tokens; card-names.txt only card names). v3 held these as lexeme:Name/* entries in crates/deckmaste_lexical_source/lexicon/core.ron |
| `vocabulary_batch_2.rs::vocabulary_only_witnesses_preserve_roles` | wood is a Name introduced by card text (token name / option name) usable as Name (e.g. 'Wood' on Jungle Patrol) | gap | lexical analysis leaves the name unknown; no v4 catalog declares text-introduced names (tokens.ron has only the [CR#111.10] predefined tokens; card-names.txt only card names). v3 held these as lexeme:Name/* entries in crates/deckmaste_lexical_source/lexicon/core.ron |
| `vocabulary_batch_2.rs::vocabulary_only_witnesses_preserve_roles` | recent is declared vocabulary usable as AdjectivePhrase (e.g. 'recent' on Rocket Launcher) | held-untested | adj:recent at crates/deckmaste_lexical_source_v4/lexicon/english/adjectives.ron:60; judgment vocabulary_batch_2.txt (pending) |
| `vocabulary_batch_2.rs::vocabulary_only_witnesses_preserve_roles` | redistribute is declared vocabulary usable as SecondaryVerbPhrase (e.g. 'Redistribute any number of players' life totals' on Reverse the Sands) | held-untested | verb:redistribute at crates/deckmaste_lexical_source_v4/lexicon/english/verbs.ron:100; judgment vocabulary_batch_2.txt (pending) |
| `vocabulary_batch_2.rs::vocabulary_only_witnesses_preserve_roles` | subtract is declared vocabulary usable as SecondaryVerbPhrase (e.g. 'subtract the number of cards in your hand' on The Deck of Many Things) | held-untested | verb:subtract at crates/deckmaste_lexical_source_v4/lexicon/english/verbs.ron:125; judgment vocabulary_batch_2.txt (pending) |
| `vocabulary_batch_2.rs::vocabulary_only_witnesses_preserve_roles` | find is declared vocabulary usable as FiniteObjectGap (e.g. 'find' on Opposition Agent) | held-untested | verb:find at crates/deckmaste_lexical_source_v4/lexicon/english/verbs.ron:55; judgment vocabulary_batch_2.txt (pending) |
| `vocabulary_batch_2.rs::vocabulary_only_witnesses_preserve_roles` | redwing is a Name introduced by card text (token name / option name) usable as NounPhrase (e.g. 'Redwing' on Falcon, Winged Wonder) | gap | lexical analysis leaves the name unknown; no v4 catalog declares text-introduced names (tokens.ron has only the [CR#111.10] predefined tokens; card-names.txt only card names). v3 held these as lexeme:Name/* entries in crates/deckmaste_lexical_source/lexicon/core.ron |
| `vocabulary_batch_2.rs::vocabulary_only_witnesses_preserve_roles` | rock is a Name introduced by card text (token name / option name) usable as Name (e.g. 'Rock' on Toggo, Goblin Weaponsmith) | gap | lexical analysis leaves the name unknown; no v4 catalog declares text-introduced names (tokens.ron has only the [CR#111.10] predefined tokens; card-names.txt only card names). v3 held these as lexeme:Name/* entries in crates/deckmaste_lexical_source/lexicon/core.ron |
| `vocabulary_batch_2.rs::vocabulary_only_witnesses_preserve_roles` | settlement is a Name introduced by card text (token name / option name) usable as Name (e.g. 'Settlement' on Preston Garvey, Minuteman) | gap | lexical analysis leaves the name unknown; no v4 catalog declares text-introduced names (tokens.ron has only the [CR#111.10] predefined tokens; card-names.txt only card names). v3 held these as lexeme:Name/* entries in crates/deckmaste_lexical_source/lexicon/core.ron |
| `vocabulary_batch_2.rs::vocabulary_only_witnesses_preserve_roles` | reliquary dragon is a Name introduced by card text (token name / option name) usable as Name (e.g. 'Reliquary Dragon' on Dragonbroods' Relic) | gap | lexical analysis leaves the name unknown; no v4 catalog declares text-introduced names (tokens.ron has only the [CR#111.10] predefined tokens; card-names.txt only card names). v3 held these as lexeme:Name/* entries in crates/deckmaste_lexical_source/lexicon/core.ron |
| `vocabulary_batch_2.rs::vocabulary_only_witnesses_preserve_roles` | wirefly is a Name introduced by card text (token name / option name) usable as Name (e.g. 'Wirefly' on Wirefly Hive) | gap | lexical analysis leaves the name unknown; no v4 catalog declares text-introduced names (tokens.ron has only the [CR#111.10] predefined tokens; card-names.txt only card names). v3 held these as lexeme:Name/* entries in crates/deckmaste_lexical_source/lexicon/core.ron |
| `vocabulary_batch_2.rs::vocabulary_only_witnesses_preserve_roles` | zeppelin is a Name introduced by card text (token name / option name) usable as Name (e.g. 'Zeppelin' on Lita, Mechanical Engineer) | gap | lexical analysis leaves the name unknown; no v4 catalog declares text-introduced names (tokens.ron has only the [CR#111.10] predefined tokens; card-names.txt only card names). v3 held these as lexeme:Name/* entries in crates/deckmaste_lexical_source/lexicon/core.ron |
| `vocabulary_batch_2.rs::vocabulary_only_witnesses_preserve_roles` | nalaar aetherjet is a Name introduced by card text (token name / option name) usable as Name (e.g. 'Nalaar Aetherjet' on Pia Nalaar, Chief Mechanic) | gap | lexical analysis leaves the name unknown; no v4 catalog declares text-introduced names (tokens.ron has only the [CR#111.10] predefined tokens; card-names.txt only card names). v3 held these as lexeme:Name/* entries in crates/deckmaste_lexical_source/lexicon/core.ron |
| `vocabulary_batch_2.rs::vocabulary_only_witnesses_preserve_roles` | urami is a Name introduced by card text (token name / option name) usable as NounPhrase (e.g. 'Urami' on Tomb of Urami) | gap | lexical analysis leaves the name unknown; no v4 catalog declares text-introduced names (tokens.ron has only the [CR#111.10] predefined tokens; card-names.txt only card names). v3 held these as lexeme:Name/* entries in crates/deckmaste_lexical_source/lexicon/core.ron |
| `vocabulary_batch_2.rs::vocabulary_only_witnesses_preserve_roles` | tuktuk is a Name introduced by card text (token name / option name) usable as Name (e.g. 'Tuktuk the Returned' on Tuktuk the Explorer) | gap | lexical analysis leaves the name unknown; no v4 catalog declares text-introduced names (tokens.ron has only the [CR#111.10] predefined tokens; card-names.txt only card names). v3 held these as lexeme:Name/* entries in crates/deckmaste_lexical_source/lexicon/core.ron |
| `waterbend_cost_verb.rs::ordinary_paradigm_is_declared_and_roundtrips` | waterbend has the full regular paradigm waterbend/waterbends/waterbending | held-untested | crates/deckmaste_lexical_source_v4/catalogs/keyword_actions.ron:219; lexical analysis -> Plain/Present, waterbends Present, waterbending Gerund; paradigms.rs::every_verb_has_its_full_paradigm (generic) |
| `waterbend_cost_verb.rs::ordinary_paradigm_is_declared_and_roundtrips` | waterbend has the full paradigm waterbend/waterbends/waterbent/waterbending | held-untested | crates/deckmaste_lexical_source_v4/catalogs/keyword_actions.ron:219 (past: "waterbent", following the head verb bend, alongside airbent/earthbent/firebent); paradigms.rs::every_verb_has_its_full_paradigm (generic) |
| `your_choice_of_keywords.rs::declared_nominal_frame_is_required_and_parameterized_keywords_are_compositional` | choice takes an of-complement of keywords ('your choice of flying', 'of deathtouch or lifelink') | held-untested | crates/deckmaste_lexical_source_v4/lexicon/english/nouns.ron:27 has Complement(PP([of])) and prepositions.ron gives `of` a KeywordPhrase complement (hosts: ComplementOnly); crates/deckmaste_english_v4/judgments/your_choice_of_keywords.txt (pending: COMPLETED_STRATUM=None) |
| `your_choice_of_keywords.rs::declared_nominal_frame_is_required_and_parameterized_keywords_are_compositional` | parameterized keywords (double strike, protection from red) are lexical keywords usable in the list | held-untested | crates/deckmaste_lexical_source_v4/catalogs/keyword_abilities.ron:9, crates/deckmaste_lexical_source_v4/catalogs/keyword_abilities.ron:32 Quality(from) |
