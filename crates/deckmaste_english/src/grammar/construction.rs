use std::sync::OnceLock;

use strum::IntoEnumIterator;

use super::RuleTag;
use crate::construction::ConstructionBackend;
use crate::construction::ConstructionEvidence;
use crate::construction::ConstructionFamily;
use crate::construction::ConstructionId;
use crate::construction::ConstructionOwner;
use crate::construction::ConstructionRegistry;
use crate::construction::ConstructionRegistryError;
use crate::construction::DominanceEdge;

pub(super) fn construction_id(tag: RuleTag) -> ConstructionId {
    let name: &'static str = tag.into();
    ConstructionId::new(name)
}

fn evidence(tag: RuleTag) -> ConstructionEvidence {
    let _ = tag;
    ConstructionEvidence::structural("production shape")
}

fn family(tag: RuleTag) -> ConstructionFamily {
    ConstructionFamily::new(
        construction_id(tag),
        ConstructionOwner::Handwritten,
        ConstructionBackend::Chart,
        evidence(tag),
    )
}

fn dominance_edges() -> [DominanceEdge; 4] {
    let edge = |dominant, subordinate| {
        DominanceEdge::new(construction_id(dominant), construction_id(subordinate))
    };
    [
        // These relationships make the grammar's former insertion-order
        // preferences explicit. They were censused against the full English
        // suite when construction identity replaced numeric rule order.
        edge(RuleTag::NounPhraseNominal, RuleTag::NounPhraseMinus),
        edge(
            RuleTag::NounPhraseSubjectPronoun,
            RuleTag::NounPhraseObjectPronoun,
        ),
        edge(RuleTag::ClauseSimple, RuleTag::ClauseCopular),
        edge(RuleTag::RelativeSubject, RuleTag::RelativeObject),
    ]
}

pub(super) fn registry() -> &'static ConstructionRegistry {
    static REGISTRY: OnceLock<ConstructionRegistry> = OnceLock::new();
    REGISTRY.get_or_init(|| {
        merged_registry(crate::constructions::GROUPS)
            .expect("production construction registry must be valid")
    })
}

pub(super) fn handwritten_registry() -> &'static ConstructionRegistry {
    static REGISTRY: OnceLock<ConstructionRegistry> = OnceLock::new();
    REGISTRY.get_or_init(|| {
        ConstructionRegistry::new(RuleTag::iter().map(family), dominance_edges())
            .expect("static construction registry must be valid")
    })
}

pub(crate) fn families() -> &'static [ConstructionFamily] {
    registry().families()
}

pub(crate) fn family_by_id(id: ConstructionId) -> Option<ConstructionFamily> {
    registry().family(id)
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
    replace_handwritten: bool,
) -> Result<ConstructionRegistry, ConstructionRegistryError> {
    let generated_ids = groups
        .iter()
        .flat_map(|group| {
            group
                .constructions
                .iter()
                .map(|construction| construction.id)
        })
        .collect::<std::collections::BTreeSet<_>>();
    let generated_families = groups
        .iter()
        .flat_map(|group| group.constructions.iter().map(generated_family));
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
    ConstructionRegistry::new(
        RuleTag::iter()
            .filter(|tag| {
                let id: &'static str = (*tag).into();
                !replace_handwritten || !generated_ids.contains(id)
            })
            .map(family)
            .chain(generated_families),
        dominance_edges().into_iter().chain(generated_edges),
    )
}

fn generated_family(
    construction: &deckmaste_construction_compiler::runtime::ConstructionData,
) -> ConstructionFamily {
    ConstructionFamily::new(
        ConstructionId::new(construction.id),
        ConstructionOwner::Generated,
        ConstructionBackend::Chart,
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
    use strum::IntoEnumIterator;

    use super::super::RuleTag;
    use super::construction_id;
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
        for tag in RuleTag::iter() {
            let id = construction_id(tag);
            let family = registry.family(id).expect("tag missing from registry");
            assert_eq!(family.owner(), ConstructionOwner::Handwritten);
            assert_eq!(family.backend(), ConstructionBackend::Chart);
        }
        for id in ["noun_phrase_coordination", "shared_determiner_nominal"] {
            let family = registry
                .family(ConstructionId::new(id))
                .expect("generated coordination family is registered");
            assert_eq!(family.owner(), ConstructionOwner::Generated);
            assert_eq!(family.backend(), ConstructionBackend::Chart);
        }
    }

    #[test]
    fn production_v01_has_exactly_34_generated_owners_and_declaration_edges() {
        // Mutations caught: leave any V01 ID as a RuleTag-backed handwritten
        // family, activate only part of the declaration group, duplicate an
        // owner, or retain/drop a handwritten dominance mirror.
        let declarations = crate::constructions::predicate::PREDICATE_DECLARATION.constructions;
        assert_eq!(declarations.len(), 34, "required V01 declarations");
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
                "missing declaration-owned V01 edge {winner}>{loser}"
            );
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
        // Mutation caught: add/drop a RuleTag or generated declaration without
        // updating the migration inventory, or accidentally count the two
        // chart-backed fragment categories as handwritten ability entries.
        let families = registry().families();
        let handwritten = families
            .iter()
            .filter(|family| family.owner() == ConstructionOwner::Handwritten)
            .count();
        let generated = families
            .iter()
            .filter(|family| family.owner() == ConstructionOwner::Generated)
            .count();
        assert_eq!(handwritten, 106, "handwritten chart families");
        assert_eq!(generated, 86, "generated chart families");
        assert_eq!(families.len(), 192, "all chart families");

        let chart_fragment_entries = [FragmentKind::Nominal, FragmentKind::Sentence];
        let ability_fragment_entries = [
            FragmentKind::Cost,
            FragmentKind::KeywordLine,
            FragmentKind::Ability,
        ];
        assert_eq!(chart_fragment_entries.len(), 2);
        assert_eq!(ability_fragment_entries.len(), 3);
        assert_eq!(handwritten + ability_fragment_entries.len(), 109);
        assert_eq!(families.len() + ability_fragment_entries.len(), 195);
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
            construction_id(RuleTag::NounPhraseNominal),
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
    fn production_m01_nominals_have_exactly_the_generated_owners_and_dominance() {
        // Mutations caught: leave any M01 family registered through RuleTag,
        // omit one declaration from the atomic activation, or drop/add an
        // edge from the tracked M01 dominance graph.
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
            "the M01 declaration must own exactly the tracked ten edges"
        );
        for &(winner, loser) in &expected {
            assert!(
                registry().dominates(ConstructionId::new(winner), ConstructionId::new(loser)),
                "missing M01 edge {winner} > {loser}"
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
            }];
        const GROUP: deckmaste_construction_compiler::runtime::GroupData =
            synthetic_group("g", CONSTRUCTIONS);
        let merged = super::merged_registry(&[&GROUP]).expect("clean synthetic group must merge");
        let generated_family = merged
            .family(ConstructionId::new("gen_c"))
            .expect("generated row must be present");
        assert_eq!(generated_family.owner(), ConstructionOwner::Generated);
        assert_eq!(generated_family.backend(), ConstructionBackend::Chart);
        for tag in RuleTag::iter() {
            let family = merged
                .family(construction_id(tag))
                .expect("handwritten row must survive the merge");
            assert_eq!(family.owner(), ConstructionOwner::Handwritten);
        }
    }

    #[test]
    fn inactive_registry_is_the_handwritten_control() {
        // `merged_registry(&[])` (no active generated groups) must carry
        // exactly the same rows as the real `registry()` static — the
        // guarantee that activation changes nothing for production parses.
        let merged = super::merged_registry(&[]).expect("no groups must merge cleanly");
        let tags = RuleTag::iter().collect::<Vec<_>>();
        assert_eq!(merged.families().len(), tags.len());
        assert_eq!(
            merged.families().len(),
            handwritten_registry().families().len()
        );
        for tag in tags {
            let id = construction_id(tag);
            let production_family = handwritten_registry()
                .family(id)
                .expect("tag missing from handwritten_registry()");
            let merged_family = merged
                .family(id)
                .expect("tag missing from merged_registry(&[])");
            assert_eq!(production_family, merged_family);
        }
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
