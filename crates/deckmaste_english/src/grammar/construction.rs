use std::sync::OnceLock;

use crate::construction::ConstructionBackend;
use crate::construction::ConstructionEvidence;
use crate::construction::ConstructionFamily;
use crate::construction::ConstructionId;
use crate::construction::ConstructionOwner;
use crate::construction::ConstructionRegistry;
use crate::construction::ConstructionRegistryError;
use crate::construction::DominanceEdge;

pub(super) fn registry() -> &'static ConstructionRegistry {
    static REGISTRY: OnceLock<ConstructionRegistry> = OnceLock::new();
    REGISTRY.get_or_init(|| {
        merged_registry(crate::constructions::ALL_GROUPS)
            .expect("production construction registry must be valid")
    })
}

pub(super) fn handwritten_registry() -> &'static ConstructionRegistry {
    static REGISTRY: OnceLock<ConstructionRegistry> = OnceLock::new();
    REGISTRY.get_or_init(|| {
        ConstructionRegistry::new([], []).expect("static construction registry must be valid")
    })
}

pub(crate) fn families() -> &'static [ConstructionFamily] {
    registry().families()
}

pub(crate) fn family_by_id(id: ConstructionId) -> Option<ConstructionFamily> {
    registry().family(id)
}

pub(crate) fn dominates(winner: &'static str, loser: &'static str) -> bool {
    registry().dominates(ConstructionId::new(winner), ConstructionId::new(loser))
}

pub(super) fn merged_registry(
    groups: &[&'static deckmaste_construction_compiler::runtime::GroupData],
) -> Result<ConstructionRegistry, ConstructionRegistryError> {
    merged_registry_with_replacements(groups, false)
}

pub(super) fn merged_registry_for_activation(
    groups: &[&'static deckmaste_construction_compiler::runtime::GroupData],
) -> Result<ConstructionRegistry, ConstructionRegistryError> {
    merged_registry_with_replacements(groups, true)
}

fn merged_registry_with_replacements(
    groups: &[&'static deckmaste_construction_compiler::runtime::GroupData],
    _replace_handwritten: bool,
) -> Result<ConstructionRegistry, ConstructionRegistryError> {
    let generated_families = groups.iter().flat_map(|group| {
        group
            .constructions
            .iter()
            .map(|construction| generated_family(group.backend, construction))
    });
    let generated_edges = groups.iter().flat_map(|group| {
        group.constructions.iter().flat_map(|construction| {
            construction
                .dominates
                .iter()
                .map(|loser| {
                    DominanceEdge::new(
                        ConstructionId::new(construction.id),
                        ConstructionId::new(loser),
                    )
                })
                .chain(construction.dominated_by.iter().map(|winner| {
                    DominanceEdge::new(
                        ConstructionId::new(winner),
                        ConstructionId::new(construction.id),
                    )
                }))
        })
    });
    ConstructionRegistry::new(generated_families, generated_edges)
}

fn generated_family(
    backend: deckmaste_construction_compiler::runtime::ConstructionBackendData,
    construction: &deckmaste_construction_compiler::runtime::ConstructionData,
) -> ConstructionFamily {
    let backend = match backend {
        deckmaste_construction_compiler::runtime::ConstructionBackendData::Chart => {
            ConstructionBackend::Chart
        }
        deckmaste_construction_compiler::runtime::ConstructionBackendData::Ability => {
            ConstructionBackend::Ability
        }
    };
    ConstructionFamily::new(
        ConstructionId::new(construction.id),
        ConstructionOwner::Generated,
        backend,
        generated_evidence(construction),
    )
}

fn generated_evidence(
    construction: &deckmaste_construction_compiler::runtime::ConstructionData,
) -> ConstructionEvidence {
    use deckmaste_construction_compiler::runtime::EvidenceKindData;

    let Some(evidence) = construction.evidence else {
        return ConstructionEvidence::structural("generated production");
    };
    match evidence.kind {
        EvidenceKindData::Guard => ConstructionEvidence::guard(evidence.label),
        EvidenceKindData::Feature => ConstructionEvidence::feature(evidence.label),
        EvidenceKindData::Role => ConstructionEvidence::role(evidence.label),
    }
}

#[cfg(test)]
mod tests {
    use super::handwritten_registry;
    use super::registry;
    use crate::FragmentKind;
    use crate::construction::ConstructionBackend;
    use crate::construction::ConstructionEvidence;
    use crate::construction::ConstructionFamily;
    use crate::construction::ConstructionId;
    use crate::construction::ConstructionOwner;
    use crate::construction::ConstructionRegistry;
    use crate::construction::ConstructionRegistryError;
    use crate::construction::DominanceEdge;

    fn test_family(id: ConstructionId) -> ConstructionFamily {
        ConstructionFamily::new(
            id,
            ConstructionOwner::Handwritten,
            ConstructionBackend::Chart,
            ConstructionEvidence::structural("test production"),
        )
    }

    #[test]
    fn production_registry_has_one_owner_per_active_family() {
        let registry = registry();
        for id in ["noun_phrase_coordination", "shared_determiner_nominal"] {
            let family = registry
                .family(ConstructionId::new(id))
                .expect("generated coordination family is registered");
            assert_eq!(family.owner(), ConstructionOwner::Generated);
            assert_eq!(family.backend(), ConstructionBackend::Chart);
        }
    }

    #[test]
    fn production_noun_phrase_families_are_generated_only() {
        let ids = [
            "noun_phrase_set_exception_bare",
            "noun_phrase_set_exception_for",
            "noun_phrase_nominal",
            "rules_object_noun_phrase",
            "noun_phrase_subject_pronoun",
            "noun_phrase_object_pronoun",
            "noun_phrase_reciprocal",
            "noun_phrase_quantity",
            "noun_phrase_this_card",
            "noun_phrase_full_this_card",
            "noun_phrase_possessive_this_card",
            "noun_phrase_demonstrative",
            "noun_phrase_partitive",
            "noun_phrase_each_partitive",
            "noun_phrase_any_number_of",
            "noun_phrase_minus",
            "noun_phrase_half",
            "noun_phrase_half_rounded_up",
            "noun_phrase_half_rounded_down",
        ];

        for name in ids {
            let id = ConstructionId::new(name);
            assert!(
                handwritten_registry().family(id).is_none(),
                "{name} still has a handwritten owner"
            );
            let family = registry()
                .family(id)
                .unwrap_or_else(|| panic!("{name} is absent from production"));
            assert_eq!(family.owner(), ConstructionOwner::Generated, "{name}");
            assert_eq!(family.backend(), ConstructionBackend::Chart, "{name}");
        }

        let expected = [
            ("noun_phrase_nominal", "noun_phrase_minus"),
            ("noun_phrase_subject_pronoun", "noun_phrase_object_pronoun"),
            ("noun_phrase_nominal", "noun_phrase_coordination"),
            ("noun_phrase_any_number_of", "noun_phrase_nominal"),
        ];
        for (winner, loser) in expected {
            assert!(
                registry().dominates(ConstructionId::new(winner), ConstructionId::new(loser)),
                "missing noun-phrase dominance edge {winner}>{loser}"
            );
        }
        for &winner in &ids {
            for &loser in &ids {
                let expected = matches!(
                    (winner, loser),
                    (
                        "noun_phrase_nominal" | "noun_phrase_any_number_of",
                        "noun_phrase_minus"
                    ) | ("noun_phrase_subject_pronoun", "noun_phrase_object_pronoun")
                        | ("noun_phrase_any_number_of", "noun_phrase_nominal")
                );
                assert_eq!(
                    registry().dominates(ConstructionId::new(winner), ConstructionId::new(loser)),
                    expected,
                    "unexpected intra-family noun-phrase dominance relation {winner}>{loser}"
                );
            }
        }
    }

    #[test]
    fn prepositional_phrase_families_are_generated_only() {
        let ids = ["prepositional_phrase", "prepositional_object"];
        let declarations =
            crate::constructions::prepositional::PREPOSITIONAL_DECLARATION.constructions;
        assert_eq!(
            declarations
                .iter()
                .map(|construction| construction.id)
                .collect::<Vec<_>>(),
            ids
        );
        assert!(declarations.iter().all(|construction| {
            construction.dominates.is_empty() && construction.dominated_by.is_empty()
        }));

        for name in ids {
            let id = ConstructionId::new(name);
            assert!(
                handwritten_registry().family(id).is_none(),
                "{name} still has a handwritten owner"
            );
            let family = registry()
                .family(id)
                .unwrap_or_else(|| panic!("{name} is absent from production"));
            assert_eq!(family.owner(), ConstructionOwner::Generated, "{name}");
            assert_eq!(family.backend(), ConstructionBackend::Chart, "{name}");

            for other in registry().families() {
                assert!(
                    !registry().dominates(id, other.id()),
                    "prepositional-phrase rows must declare no outgoing dominance edge: {id} > {}",
                    other.id()
                );
            }
        }

        for incoming in ["noun_phrase_coordination", "shared_determiner_nominal"] {
            assert!(
                registry().dominates(
                    ConstructionId::new(incoming),
                    ConstructionId::new("prepositional_phrase")
                ),
                "missing phrase-coordination incoming edge {incoming} > prepositional_phrase"
            );
        }
    }

    #[test]
    fn predicate_family_has_exactly_34_generated_owners_and_declaration_edges() {
        // Mutations caught: leave any predicate construction handwritten,
        // activate only part of the declaration group, duplicate an owner, or
        // retain/drop a handwritten dominance mirror.
        let declarations = crate::constructions::predicate::PREDICATE_DECLARATION.constructions;
        assert_eq!(declarations.len(), 34, "required predicate declarations");
        for construction in declarations {
            let id = ConstructionId::new(construction.id);
            assert!(
                handwritten_registry().family(id).is_none(),
                "{} still has a handwritten owner",
                construction.id
            );
            let family = registry()
                .family(id)
                .unwrap_or_else(|| panic!("{} is absent from production", construction.id));
            assert_eq!(family.owner(), ConstructionOwner::Generated, "{id}");
            assert_eq!(family.backend(), ConstructionBackend::Chart, "{id}");
        }

        let expected = [
            ("verb_phrase_base", "verb_phrase_auxiliary_proform"),
            ("verb_phrase_auxiliary", "verb_phrase_adjective"),
            ("verb_phrase_auxiliary", "verb_phrase_adverb"),
            ("verb_phrase_auxiliary", "verb_phrase_ability"),
        ];
        let actual = declarations
            .iter()
            .flat_map(|construction| {
                construction
                    .dominates
                    .iter()
                    .map(move |subordinate| (construction.id, *subordinate))
            })
            .collect::<std::collections::BTreeSet<_>>();
        assert_eq!(actual, expected.into_iter().collect());
        for (winner, loser) in expected {
            assert!(
                registry().dominates(ConstructionId::new(winner), ConstructionId::new(loser)),
                "missing declaration-owned predicate edge {winner}>{loser}"
            );
        }
    }

    #[test]
    fn adjective_family_has_exactly_eleven_generated_owners_without_dominance() {
        let declarations = crate::constructions::adjective::ADJECTIVE_DECLARATION.constructions;
        assert_eq!(declarations.len(), 11, "required adjective declarations");
        let ids = declarations
            .iter()
            .map(|construction| ConstructionId::new(construction.id))
            .collect::<std::collections::BTreeSet<_>>();
        assert_eq!(ids.len(), 11, "adjective IDs are unique");

        for construction in declarations {
            let id = ConstructionId::new(construction.id);
            assert!(construction.dominates.is_empty(), "{id} declares dominance");
            assert!(
                construction.dominated_by.is_empty(),
                "{id} declares subordinate status"
            );
            assert!(
                handwritten_registry().family(id).is_none(),
                "{id} still has a handwritten owner"
            );
            let family = registry()
                .family(id)
                .unwrap_or_else(|| panic!("{id} is absent from production"));
            assert_eq!(family.owner(), ConstructionOwner::Generated, "{id}");
            assert_eq!(family.backend(), ConstructionBackend::Chart, "{id}");
            assert_eq!(
                registry()
                    .families()
                    .iter()
                    .filter(|family| family.id() == id)
                    .count(),
                1,
                "{id} must fan out to one production owner"
            );
            for other in registry().families() {
                assert!(
                    !registry().dominates(id, other.id()) && !registry().dominates(other.id(), id),
                    "adjective rows must have no dominance relation: {id} / {}",
                    other.id()
                );
            }
        }
    }

    #[test]
    fn determiner_family_has_exactly_ten_generated_owners_without_dominance() {
        let declarations = crate::constructions::determiner::DETERMINER_DECLARATION.constructions;
        assert_eq!(declarations.len(), 10, "required determiner declarations");
        let ids = declarations
            .iter()
            .map(|construction| ConstructionId::new(construction.id))
            .collect::<std::collections::BTreeSet<_>>();
        assert_eq!(ids.len(), 10, "determiner IDs are unique");

        let normal = crate::constructions::GROUPS.to_vec();
        let mut reversed_groups = normal.clone();
        reversed_groups.reverse();
        let normal = super::merged_registry_for_activation(&normal).unwrap();
        let reversed = super::merged_registry_for_activation(&reversed_groups).unwrap();
        for construction in declarations {
            let id = ConstructionId::new(construction.id);
            assert!(construction.dominates.is_empty(), "{id} declares dominance");
            assert!(
                construction.dominated_by.is_empty(),
                "{id} declares subordinate status"
            );
            assert!(
                handwritten_registry().family(id).is_none(),
                "{id} still has a handwritten owner"
            );
            let family = registry()
                .family(id)
                .unwrap_or_else(|| panic!("{id} is absent from production"));
            assert_eq!(family.owner(), ConstructionOwner::Generated, "{id}");
            assert_eq!(family.backend(), ConstructionBackend::Chart, "{id}");
            assert_eq!(normal.family(id), reversed.family(id), "{id}");
            assert_eq!(
                registry()
                    .families()
                    .iter()
                    .filter(|family| family.id() == id)
                    .count(),
                1,
                "{id} must fan out to one production owner"
            );
            for other in registry().families() {
                assert!(
                    !registry().dominates(id, other.id()) && !registry().dominates(other.id(), id),
                    "determiner rows must have no dominance relation: {id} / {}",
                    other.id()
                );
            }
        }
    }

    #[test]
    fn production_predicate_registry_is_group_permutation_neutral() {
        // Mutation caught: let generated replacement ownership or dominance
        // depend on the predicate group's insertion position.
        let normal = crate::constructions::GROUPS.to_vec();
        let mut reversed = normal.clone();
        reversed.reverse();
        let normal = super::merged_registry_for_activation(&normal)
            .expect("normal production group assembly is valid");
        let reversed = super::merged_registry_for_activation(&reversed)
            .expect("reversed production group assembly is valid");
        let required = crate::constructions::predicate::PREDICATE_DECLARATION
            .constructions
            .iter()
            .map(|construction| construction.id)
            .collect::<Vec<_>>();
        for id in required {
            let id = ConstructionId::new(id);
            assert_eq!(normal.family(id), reversed.family(id), "{id}");
            assert_eq!(
                normal
                    .family(id)
                    .expect("predicate family is present")
                    .owner(),
                ConstructionOwner::Generated,
                "{id}"
            );
        }
        for (winner, loser) in [
            ("verb_phrase_base", "verb_phrase_auxiliary_proform"),
            ("verb_phrase_auxiliary", "verb_phrase_adjective"),
            ("verb_phrase_auxiliary", "verb_phrase_adverb"),
            ("verb_phrase_auxiliary", "verb_phrase_ability"),
        ] {
            let winner = ConstructionId::new(winner);
            let loser = ConstructionId::new(loser);
            assert!(normal.dominates(winner, loser));
            assert!(reversed.dominates(winner, loser));
        }
    }

    #[test]
    fn production_registry_and_ability_entry_census_matches_the_inventory() {
        // Mutation caught: add/drop a handwritten or generated declaration
        // without updating the migration inventory, or accidentally count the two
        // chart-backed fragment categories as handwritten ability entries.
        let families = registry().families();
        let handwritten = families
            .iter()
            .filter(|family| {
                family.backend() == ConstructionBackend::Chart
                    && family.owner() == ConstructionOwner::Handwritten
            })
            .count();
        let generated = families
            .iter()
            .filter(|family| {
                family.backend() == ConstructionBackend::Chart
                    && family.owner() == ConstructionOwner::Generated
            })
            .count();
        let generated_ability = families
            .iter()
            .filter(|family| {
                family.backend() == ConstructionBackend::Ability
                    && family.owner() == ConstructionOwner::Generated
            })
            .count();
        assert_eq!(handwritten, 0, "handwritten chart families");
        assert_eq!(generated, 219, "generated chart families");
        assert_eq!(handwritten + generated, 219, "all chart families");
        assert_eq!(generated_ability, 3, "generated ability families");
        assert_eq!(families.len(), 222, "all active construction families");

        let chart_fragment_entries = [FragmentKind::Nominal, FragmentKind::Sentence];
        let ability_fragment_entries = [
            FragmentKind::Cost,
            FragmentKind::KeywordLine,
            FragmentKind::Ability,
        ];
        assert_eq!(chart_fragment_entries.len(), 2);
        assert_eq!(ability_fragment_entries.len(), 3);
        assert_eq!(
            handwritten + ability_fragment_entries.len() - generated_ability,
            0
        );
        assert_eq!(
            families.len() + ability_fragment_entries.len() - generated_ability,
            222
        );

        for id in [
            "simple_clause_subject",
            "simple_clause_subject_distributive_each",
            "simple_clause_contracted_subject",
            "simple_clause_subjectless",
            "clause_simple",
            "clause_elliptical",
            "clause_existential",
            "copular_remainder_noun",
            "copular_remainder_adjective",
            "copular_remainder_prepositional",
            "copular_remainder_power_toughness",
            "copular_remainder_prepositional_adjunct",
            "copular_remainder_adverb",
            "copular_remainder_negated",
            "copular_remainder_distributive_each",
            "clause_copular",
            "clause_contracted_copular",
            "clause_variable_value_constraint",
        ] {
            assert_eq!(
                registry()
                    .family(ConstructionId::new(id))
                    .expect("every finite-clause family is registered")
                    .owner(),
                ConstructionOwner::Generated,
                "{id} must have generated production ownership",
            );
        }
    }

    #[test]
    fn production_phrase_coordination_has_exactly_the_generated_owners() {
        let ids = [
            "modifier_conjunct_adjective",
            "modifier_conjunct_noun",
            "modifier_conjunct_negated",
            "modifier_list_single",
            "modifier_list_comma",
            "coordinated_modifier_conjoined",
            "coordinated_modifier_oxford",
            "nominal_coordinated_modifier",
            "prepositional_phrase_list_pair",
            "prepositional_phrase_list_comma",
            "prepositional_phrase_sibling_coordinated",
            "verb_phrase_coordinated_adjective",
            "nominal_power_toughness_complement",
            "nominal_with_attributes",
        ];
        for name in ids {
            let id = ConstructionId::new(name);
            assert!(
                handwritten_registry().family(id).is_none(),
                "{name} still has a handwritten owner"
            );
            let family = registry()
                .family(id)
                .unwrap_or_else(|| panic!("{name} generated declaration is registered"));
            assert_eq!(family.owner(), ConstructionOwner::Generated, "{name}");
            assert_eq!(family.backend(), ConstructionBackend::Chart, "{name}");
        }
    }

    #[test]
    fn nonfinite_family_has_exactly_four_generated_owners_without_dominance() {
        let ids = [
            "infinitive_to",
            "infinitive_not_to",
            "gerund_clause_base",
            "gerund_clause_subordinate_after",
        ];
        for name in ids {
            let id = ConstructionId::new(name);
            assert!(
                handwritten_registry().family(id).is_none(),
                "{name} still has a handwritten owner"
            );
            let family = registry()
                .family(id)
                .unwrap_or_else(|| panic!("{name} generated declaration is registered"));
            assert_eq!(family.owner(), ConstructionOwner::Generated, "{name}");
            assert_eq!(family.backend(), ConstructionBackend::Chart, "{name}");
            for other in registry().families() {
                assert!(
                    !registry().dominates(id, other.id()) && !registry().dominates(other.id(), id),
                    "nonfinite-clause rows must have no dominance relation: {id} / {}",
                    other.id()
                );
            }
        }
    }

    #[test]
    fn production_clause_attachment_families_are_generated_only() {
        // The sixteen attachment declarations own these families with no
        // remaining handwritten owner.
        let ids = [
            "clause_adverb_before",
            "clause_sentence_adverbial_before",
            "clause_prepositional_before",
            "clause_subordinate_before",
            "clause_subordinate_gerund_before",
            "clause_subordinate_after_elliptical",
            "clause_subordinate_after",
            "clause_subordinate_after_comma",
            "clause_subordinate_after_infinitive",
            "exception_rider_single",
            "exception_rider_conjoined",
            "exception_rider_comma",
            "exception_rider_oxford",
            "clause_excepted",
            "clause_restriction_run",
            "clause_restriction_member",
        ];
        for name in ids {
            let id = ConstructionId::new(name);
            assert!(
                handwritten_registry().family(id).is_none(),
                "{name} still has a handwritten owner"
            );
            let family = registry()
                .family(id)
                .unwrap_or_else(|| panic!("{name} generated declaration is registered"));
            assert_eq!(family.owner(), ConstructionOwner::Generated, "{name}");
            assert_eq!(family.backend(), ConstructionBackend::Chart, "{name}");
            for other in registry().families() {
                assert!(
                    !registry().dominates(id, other.id()) && !registry().dominates(other.id(), id),
                    "clause-attachment rows must have no dominance relation: {id} / {}",
                    other.id()
                );
            }
        }
    }

    #[test]
    fn production_clause_coordination_families_are_generated_only() {
        let ids = [
            "clause_coordination",
            "clause_coordination_comma",
            "clause_coordination_asyndetic",
            "clause_coordination_copular_noun_prepositional",
            "clause_coordination_copular_noun_prepositional_comma",
            "clause_coordination_copular_noun_prepositional_asyndetic",
        ];
        let normal_groups = crate::constructions::GROUPS.to_vec();
        let mut reversed_groups = normal_groups.clone();
        reversed_groups.reverse();
        let normal = super::merged_registry_for_activation(&normal_groups)
            .expect("normal generated group assembly is valid");
        let reversed = super::merged_registry_for_activation(&reversed_groups)
            .expect("reversed generated group assembly is valid");

        for name in ids {
            let id = ConstructionId::new(name);
            assert!(
                handwritten_registry().family(id).is_none(),
                "{name} still has a handwritten owner"
            );
            let family = registry()
                .family(id)
                .unwrap_or_else(|| panic!("{name} generated declaration is registered"));
            assert_eq!(family.owner(), ConstructionOwner::Generated, "{name}");
            assert_eq!(family.backend(), ConstructionBackend::Chart, "{name}");
            assert_eq!(normal.family(id), reversed.family(id), "{name}");
            for other in registry().families() {
                assert!(
                    !registry().dominates(id, other.id()) && !registry().dominates(other.id(), id),
                    "clause coordination must have no dominance relation: {id} / {}",
                    other.id()
                );
            }
        }
    }

    #[test]
    fn historic_registration_preferences_are_declared_as_dominance() {
        let preferences = [
            ("nominal_quantity_modifier", "nominal_prepositional"),
            (
                "nominal_quantity_modifier",
                "nominal_postpositive_adjective",
            ),
            ("nominal_quantity_modifier", "nominal_comparison"),
            ("nominal_determiner", "nominal_comparison"),
            ("nominal_prepositional", "nominal_infinitive"),
            ("nominal_prepositional", "nominal_coordinated_modifier"),
            (
                "nominal_prepositional",
                "nominal_keyword_predicated_argument",
            ),
            ("nominal_prepositional", "nominal_noun"),
            ("nominal_reduced_recipient_passive", "nominal_noun"),
            ("nominal_relative", "nominal_prepositional"),
            ("noun_phrase_nominal", "noun_phrase_minus"),
            ("noun_phrase_subject_pronoun", "noun_phrase_object_pronoun"),
            ("verb_phrase_auxiliary", "verb_phrase_adjective"),
            ("verb_phrase_auxiliary", "verb_phrase_adverb"),
            ("verb_phrase_auxiliary", "verb_phrase_ability"),
            ("verb_phrase_base", "verb_phrase_auxiliary_proform"),
            ("clause_simple", "clause_copular"),
            ("relative_subject", "relative_object"),
        ];
        for (dominant, subordinate) in preferences {
            assert!(
                registry().dominates(
                    ConstructionId::new(dominant),
                    ConstructionId::new(subordinate)
                ),
                "{dominant} must dominate {subordinate}",
            );
        }
        assert!(registry().dominates(
            ConstructionId::new("noun_phrase_nominal"),
            ConstructionId::new("noun_phrase_coordination"),
        ));
    }

    #[test]
    fn validation_rejects_an_unknown_dominance_target() {
        let a = ConstructionId::new("a");
        let missing = ConstructionId::new("missing");
        let error = ConstructionRegistry::new([test_family(a)], [DominanceEdge::new(a, missing)])
            .unwrap_err();
        assert_eq!(
            error,
            ConstructionRegistryError::UnknownDominanceTarget(missing)
        );
    }

    #[test]
    fn validation_rejects_a_dominance_cycle() {
        let a = ConstructionId::new("a");
        let b = ConstructionId::new("b");
        let error = ConstructionRegistry::new(
            [test_family(a), test_family(b)],
            [DominanceEdge::new(a, b), DominanceEdge::new(b, a)],
        )
        .unwrap_err();
        assert_eq!(error, ConstructionRegistryError::DominanceCycle);
    }

    const fn synthetic_group(
        name: &'static str,
        constructions: &'static [deckmaste_construction_compiler::runtime::ConstructionData],
    ) -> deckmaste_construction_compiler::runtime::GroupData {
        deckmaste_construction_compiler::runtime::GroupData {
            name,
            backend: deckmaste_construction_compiler::runtime::ConstructionBackendData::Chart,
            elements: &[],
            element_data: &[],
            lenses: &[],
            constructions,
        }
    }

    #[test]
    fn generated_id_registers_without_a_handwritten_mirror() {
        const CONSTRUCTIONS: &[deckmaste_construction_compiler::runtime::ConstructionData] =
            &[deckmaste_construction_compiler::runtime::ConstructionData {
                id: "noun_phrase_coordination",
                category: "Internal",
                internal: true,
                own_type: Some("Synthetic"),
                bind_path: None,
                lens: None,
                projection_variant: None,
                deserialize: false,
                selection_unique: false,
                dominates: &[],
                dominated_by: &[],
                fields: &[],
                witnesses: &[],
                forms: &[],
                requirements: &[],
                recognition_requirements: &[],
                feature_combinators: &[],
                evidence: None,
                erased_partial_builder: None,
                erased_builder: None,
                erased_projector: None,
                erased_linearizer:
                    deckmaste_construction_compiler::runtime::unavailable_erased_linearizer,
            }];
        const GROUP: deckmaste_construction_compiler::runtime::GroupData =
            synthetic_group("g", CONSTRUCTIONS);
        let merged = super::merged_registry(&[&GROUP]).expect("generated family registers");
        let family = merged
            .family(ConstructionId::new("noun_phrase_coordination"))
            .expect("the generated family is registered");
        assert_eq!(family.owner(), ConstructionOwner::Generated);
    }

    #[test]
    fn production_sentence_has_only_the_generated_owner() {
        let id = ConstructionId::new("sentence");
        assert!(handwritten_registry().family(id).is_none());
        let family = registry()
            .family(id)
            .expect("production sentence declaration is registered");
        assert_eq!(family.owner(), ConstructionOwner::Generated);
        assert_eq!(family.backend(), ConstructionBackend::Chart);
    }

    #[test]
    fn production_nouns_have_only_generated_owners() {
        for name in ["noun", "noun_opaque"] {
            let id = ConstructionId::new(name);
            assert!(
                handwritten_registry().family(id).is_none(),
                "{name} still has a handwritten owner"
            );
            let family = registry()
                .family(id)
                .unwrap_or_else(|| panic!("{name} generated declaration is registered"));
            assert_eq!(family.owner(), ConstructionOwner::Generated, "{name}");
            assert_eq!(family.backend(), ConstructionBackend::Chart, "{name}");
        }
    }

    #[test]
    fn production_quantities_have_only_generated_owners() {
        for name in [
            "quantity_exact",
            "quantity_at_least",
            "quantity_or",
            "quantity_x",
            "quantity_both",
            "quantity_up_to",
            "quantity_that_many",
            "quantity_that_much",
            "quantity_more_than",
            "quantity_fewer_than",
        ] {
            let id = ConstructionId::new(name);
            assert!(
                handwritten_registry().family(id).is_none(),
                "{name} still has a handwritten owner"
            );
            let family = registry()
                .family(id)
                .unwrap_or_else(|| panic!("{name} generated declaration is registered"));
            assert_eq!(family.owner(), ConstructionOwner::Generated, "{name}");
            assert_eq!(family.backend(), ConstructionBackend::Chart, "{name}");
        }
    }

    #[test]
    fn nominal_family_has_exactly_the_generated_owners_and_dominance() {
        // Mutations caught: leave any nominal family handwritten, omit one
        // declaration from the atomic activation, or drop/add a declaration-
        // owned dominance edge.
        let declarations = crate::constructions::nominal::NOMINAL_DECLARATION.constructions;
        assert_eq!(declarations.len(), 37);
        for construction in declarations {
            let name = construction.id;
            let id = ConstructionId::new(name);
            assert!(
                handwritten_registry().family(id).is_none(),
                "{name} still has a handwritten owner"
            );
            let family = registry()
                .family(id)
                .unwrap_or_else(|| panic!("{name} generated declaration is registered"));
            assert_eq!(family.owner(), ConstructionOwner::Generated, "{name}");
            assert_eq!(family.backend(), ConstructionBackend::Chart, "{name}");
        }

        let expected = [
            ("nominal_quantity_modifier", "nominal_prepositional"),
            (
                "nominal_quantity_modifier",
                "nominal_postpositive_adjective",
            ),
            ("nominal_quantity_modifier", "nominal_comparison"),
            ("nominal_determiner", "nominal_comparison"),
            ("nominal_prepositional", "nominal_infinitive"),
            ("nominal_prepositional", "nominal_coordinated_modifier"),
            (
                "nominal_prepositional",
                "nominal_keyword_predicated_argument",
            ),
            ("nominal_prepositional", "nominal_noun"),
            ("nominal_reduced_recipient_passive", "nominal_noun"),
            ("nominal_relative", "nominal_prepositional"),
        ];
        assert_eq!(expected.len(), 10);
        let actual = crate::constructions::nominal::NOMINAL_DECLARATION
            .constructions
            .iter()
            .flat_map(|construction| {
                construction
                    .dominates
                    .iter()
                    .map(move |subordinate| (construction.id, *subordinate))
            })
            .collect::<std::collections::BTreeSet<_>>();
        assert_eq!(
            actual,
            expected.iter().copied().collect(),
            "the nominal declaration must own exactly the tracked ten edges"
        );
        for &(winner, loser) in &expected {
            assert!(
                registry().dominates(ConstructionId::new(winner), ConstructionId::new(loser)),
                "missing nominal edge {winner} > {loser}"
            );
        }
    }

    #[test]
    fn cross_group_dominance_target_must_exist() {
        const CONSTRUCTIONS: &[deckmaste_construction_compiler::runtime::ConstructionData] =
            &[deckmaste_construction_compiler::runtime::ConstructionData {
                id: "gen_a",
                category: "Internal",
                internal: true,
                own_type: Some("Synthetic"),
                bind_path: None,
                lens: None,
                projection_variant: None,
                deserialize: false,
                selection_unique: false,
                dominates: &["missing"],
                dominated_by: &[],
                fields: &[],
                witnesses: &[],
                forms: &[],
                requirements: &[],
                recognition_requirements: &[],
                feature_combinators: &[],
                evidence: None,
                erased_partial_builder: None,
                erased_builder: None,
                erased_projector: None,
                erased_linearizer:
                    deckmaste_construction_compiler::runtime::unavailable_erased_linearizer,
            }];
        const GROUP: deckmaste_construction_compiler::runtime::GroupData =
            synthetic_group("g", CONSTRUCTIONS);
        let error = super::merged_registry(&[&GROUP]).unwrap_err();
        assert_eq!(
            error,
            ConstructionRegistryError::UnknownDominanceTarget(ConstructionId::new("missing"))
        );
    }

    #[test]
    fn cross_group_cycles_are_refused() {
        const A_CONSTRUCTIONS: &[deckmaste_construction_compiler::runtime::ConstructionData] =
            &[deckmaste_construction_compiler::runtime::ConstructionData {
                id: "gen_a",
                category: "Internal",
                internal: true,
                own_type: Some("Synthetic"),
                bind_path: None,
                lens: None,
                projection_variant: None,
                deserialize: false,
                selection_unique: false,
                dominates: &["gen_b"],
                dominated_by: &[],
                fields: &[],
                witnesses: &[],
                forms: &[],
                requirements: &[],
                recognition_requirements: &[],
                feature_combinators: &[],
                evidence: None,
                erased_partial_builder: None,
                erased_builder: None,
                erased_projector: None,
                erased_linearizer:
                    deckmaste_construction_compiler::runtime::unavailable_erased_linearizer,
            }];
        const B_CONSTRUCTIONS: &[deckmaste_construction_compiler::runtime::ConstructionData] =
            &[deckmaste_construction_compiler::runtime::ConstructionData {
                id: "gen_b",
                category: "Internal",
                internal: true,
                own_type: Some("Synthetic"),
                bind_path: None,
                lens: None,
                projection_variant: None,
                deserialize: false,
                selection_unique: false,
                dominates: &["gen_a"],
                dominated_by: &[],
                fields: &[],
                witnesses: &[],
                forms: &[],
                requirements: &[],
                recognition_requirements: &[],
                feature_combinators: &[],
                evidence: None,
                erased_partial_builder: None,
                erased_builder: None,
                erased_projector: None,
                erased_linearizer:
                    deckmaste_construction_compiler::runtime::unavailable_erased_linearizer,
            }];
        const GROUP_A: deckmaste_construction_compiler::runtime::GroupData =
            synthetic_group("a", A_CONSTRUCTIONS);
        const GROUP_B: deckmaste_construction_compiler::runtime::GroupData =
            synthetic_group("b", B_CONSTRUCTIONS);
        // Each group alone dangles a dominance target the compiler's own
        // in-group validation never sees (it only checks targets declared in
        // the same `constructions!` invocation) — `merged_registry` catches
        // it as an `UnknownDominanceTarget` until the other half is present.
        assert_eq!(
            super::merged_registry(&[&GROUP_A]).unwrap_err(),
            ConstructionRegistryError::UnknownDominanceTarget(ConstructionId::new("gen_b"))
        );
        assert_eq!(
            super::merged_registry(&[&GROUP_B]).unwrap_err(),
            ConstructionRegistryError::UnknownDominanceTarget(ConstructionId::new("gen_a"))
        );
        let error = super::merged_registry(&[&GROUP_A, &GROUP_B]).unwrap_err();
        assert_eq!(error, ConstructionRegistryError::DominanceCycle);
    }

    #[test]
    fn generated_rows_carry_generated_owner() {
        const CONSTRUCTIONS: &[deckmaste_construction_compiler::runtime::ConstructionData] =
            &[deckmaste_construction_compiler::runtime::ConstructionData {
                id: "gen_c",
                category: "Internal",
                internal: true,
                own_type: Some("Synthetic"),
                bind_path: None,
                lens: None,
                projection_variant: None,
                deserialize: false,
                selection_unique: false,
                dominates: &[],
                dominated_by: &[],
                fields: &[],
                witnesses: &[],
                forms: &[],
                requirements: &[],
                recognition_requirements: &[],
                feature_combinators: &[],
                evidence: None,
                erased_partial_builder: None,
                erased_builder: None,
                erased_projector: None,
                erased_linearizer:
                    deckmaste_construction_compiler::runtime::unavailable_erased_linearizer,
            }];
        const GROUP: deckmaste_construction_compiler::runtime::GroupData =
            synthetic_group("g", CONSTRUCTIONS);
        let merged = super::merged_registry(&[&GROUP]).expect("clean synthetic group must merge");
        let generated_family = merged
            .family(ConstructionId::new("gen_c"))
            .expect("generated row must be present");
        assert_eq!(generated_family.owner(), ConstructionOwner::Generated);
        assert_eq!(generated_family.backend(), ConstructionBackend::Chart);
    }

    #[test]
    fn inactive_registry_is_the_handwritten_control() {
        // `merged_registry(&[])` (no active generated groups) must carry
        // exactly the same rows as the real `registry()` static — the
        // guarantee that activation changes nothing for production parses.
        let merged = super::merged_registry(&[]).expect("no groups must merge cleanly");
        assert!(merged.families().is_empty());
        assert_eq!(
            merged.families().len(),
            handwritten_registry().families().len()
        );
        // The actual `Inactive` selection code (`parse_nonterminal::select_registry`,
        // not a re-derivation of its logic) must pick the `registry()` static
        // itself, not a freshly built `merged_registry`. This observes the
        // real function's chosen variant, so it reddens if the `Inactive` arm
        // is ever rewired to build its own registry.
        match super::super::parse_support::select_registry(
            super::super::generated::GeneratedActivation::Inactive,
        ) {
            super::super::parse_support::SelectedRegistry::Static(selected) => {
                assert!(std::ptr::eq(selected, handwritten_registry()));
            }
            super::super::parse_support::SelectedRegistry::Owned(_) => {
                panic!("Inactive must select the registry() static, not a reconstruction");
            }
        }
    }
}
