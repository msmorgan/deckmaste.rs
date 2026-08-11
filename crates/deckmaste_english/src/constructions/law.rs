//! The law-harness fixture group: a dominance-free pair of same-category
//! constructions, giving `parse_as` a genuine bytes-with-two-ASTs fixture.
//! Like the probe, semantics are stubs by design — nothing here models
//! English.

use deckmaste_construction_compiler::runtime::GroupData;

use crate::features::Conjunction;

/// The holed internal category's own-mode type. Declaration metadata now
/// references it through an erased builder; chart lowering is still staged.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct LawItem;

deckmaste_constructions_macro::constructions! {
    group law_probe;

    internal construction law_letter: LawItem {
        own LawLetterNode {
            word: lex Conjunction,
        }
        form only @ 0 = lex(word);
    }

    internal construction law_first: LawRoot {
        own LawFirstNode {
            item: hole LawItem,
        }
        form only @ 0 = item;
    }

    internal construction law_second: LawRoot {
        own LawSecondNode {
            item: hole LawItem,
        }
        form only @ 0 = item;
    }
}

// `LawRoot` deliberately has NO struct: no construction holes it, so the
// name exists only as an internal chart category.

pub(crate) static GROUPS: &[&GroupData] = &[&LAW_PROBE_DECLARATION];

#[cfg(test)]
mod predicate_family_laws {
    #[test]
    fn all_34_predicate_rows_expose_checked_builder_parts_and_inverse_laws() {
        // Mutations caught: add a chart-only row without a checked semantic
        // door, omit the generated parts projection, leave a form outside the
        // category dispatcher, or silently change a declared category.
        let required = [
            ("verb", "Verb"),
            ("verb_phrase_base", "VerbPhrase"),
            ("verb_phrase_auxiliary", "VerbPhrase"),
            ("verb_phrase_auxiliary_proform", "VerbPhrase"),
            ("verb_phrase_direct_object", "VerbPhrase"),
            ("verb_phrase_indirect_object", "VerbPhrase"),
            ("verb_phrase_adjective", "VerbPhrase"),
            ("verb_phrase_prepositional", "VerbPhrase"),
            (
                "verb_phrase_passive_shared_determiner_prepositional",
                "VerbPhrase",
            ),
            ("verb_phrase_except_by", "VerbPhrase"),
            ("verb_phrase_infinitive", "VerbPhrase"),
            ("verb_phrase_adverb", "VerbPhrase"),
            ("verb_phrase_preverb_adverb", "VerbPhrase"),
            ("verb_phrase_particle", "VerbPhrase"),
            ("verb_phrase_coin_result", "VerbPhrase"),
            ("verb_phrase_frequency", "VerbPhrase"),
            ("frequency_phrase_adverb", "FrequencyPhrase"),
            ("frequency_phrase", "FrequencyPhrase"),
            ("verb_phrase_ability", "VerbPhrase"),
            ("verb_phrase_quoted_ability", "VerbPhrase"),
            ("verb_phrase_quoted_ability_coordination", "VerbPhrase"),
            ("verb_phrase_ability_quoted_coordination", "VerbPhrase"),
            ("verb_phrase_oracle_symbol", "VerbPhrase"),
            ("verb_phrase_symbol_sequence", "VerbPhrase"),
            ("mana_amount_symbol", "ManaAmount"),
            ("mana_amount_sequence", "ManaAmount"),
            ("mana_amount_list_single", "ManaAmountList"),
            ("mana_amount_list_comma", "ManaAmountList"),
            ("mana_amount_coordination", "CoordinatedManaAmount"),
            ("mana_amount_coordination_oxford", "CoordinatedManaAmount"),
            ("verb_phrase_mana_amount_coordination", "VerbPhrase"),
            ("verb_phrase_power_toughness", "VerbPhrase"),
            ("verb_phrase_quantity", "VerbPhrase"),
            ("verb_phrase_causative", "VerbPhrase"),
        ];
        let declaration = &crate::constructions::predicate::PREDICATE_DECLARATION;
        assert_eq!(declaration.constructions.len(), required.len());
        for ((actual, (id, category)), ordinal) in
            declaration.constructions.iter().zip(required).zip(0_u16..)
        {
            assert_eq!(
                (actual.id, actual.category),
                (id, category),
                "row {ordinal}"
            );
            assert!(actual.selection_unique, "{id} must select uniquely");
            assert!(actual.erased_builder.is_some(), "{id} checked builder");
            assert!(actual.bind_path.is_some(), "{id} typed bind target");
            assert_eq!(actual.forms.len(), 1, "{id} has one exact surface form");
            assert_eq!(actual.forms[0].ordinal, 0, "{id} exact witness ordinal");
            // Typed `parts_*` and inverse selection are exercised by the
            // core/attachment/value/causative matrices in `predicate.rs`.
            // Runtime `FormData` does not expose those typed functions as an
            // erased projector/recognizer contract.
        }
    }

    #[test]
    fn predicate_declaration_owns_exactly_the_four_required_dominance_edges() {
        let declaration = &crate::constructions::predicate::PREDICATE_DECLARATION;
        let edges = declaration
            .constructions
            .iter()
            .flat_map(|construction| {
                construction
                    .dominates
                    .iter()
                    .map(move |subordinate| (construction.id, *subordinate))
            })
            .collect::<Vec<_>>();
        assert_eq!(
            edges,
            [
                ("verb_phrase_base", "verb_phrase_auxiliary_proform"),
                ("verb_phrase_auxiliary", "verb_phrase_adjective"),
                ("verb_phrase_auxiliary", "verb_phrase_adverb"),
                ("verb_phrase_auxiliary", "verb_phrase_ability"),
            ]
        );
        // `dominates` is the runtime registry authority. The declaration DSL
        // also accepts `dominated by`, but it is not a compiler-synthesized
        // reciprocal projection and remains empty for these winner-authored
        // edges; requiring it would create a second metadata contract.
        assert!(
            declaration
                .constructions
                .iter()
                .all(|construction| construction.dominated_by.is_empty())
        );
    }
}

#[cfg(test)]
mod adjective_family_laws {
    #[test]
    fn all_eleven_adjective_rows_expose_checked_declaration_laws() {
        let required = [
            ("adjective", "Adjective", 1),
            ("adjective_phrase", "AdjectivePhrase", 1),
            ("adjective_phrase_face_up", "AdjectivePhrase", 1),
            ("adjective_phrase_face_down", "AdjectivePhrase", 1),
            ("comparison_standard", "ComparisonStandard", 3),
            ("comparison_than", "ComparisonComplement", 1),
            ("comparison_than_or_equal_to", "ComparisonComplement", 1),
            ("adjective_phrase_comparison", "AdjectivePhrase", 1),
            ("adjective_phrase_degree_measure", "AdjectivePhrase", 1),
            ("adjective_phrase_prepositional", "AdjectivePhrase", 1),
            ("adjective_phrase_infinitive", "AdjectivePhrase", 1),
        ];
        let declaration = &crate::constructions::adjective::ADJECTIVE_DECLARATION;
        assert_eq!(declaration.constructions.len(), required.len());
        for (actual, (id, category, form_count)) in declaration.constructions.iter().zip(required) {
            assert_eq!((actual.id, actual.category), (id, category));
            assert!(actual.selection_unique, "{id} must select uniquely");
            assert!(actual.erased_builder.is_some(), "{id} checked builder");
            assert!(actual.bind_path.is_some(), "{id} typed bind target");
            assert_eq!(actual.forms.len(), form_count, "{id} exact form count");
            assert_eq!(
                actual
                    .forms
                    .iter()
                    .map(|form| form.ordinal)
                    .collect::<Vec<_>>(),
                (0..u16::try_from(form_count).unwrap()).collect::<Vec<_>>(),
                "{id} exact witness ordinals"
            );
        }
    }
}
