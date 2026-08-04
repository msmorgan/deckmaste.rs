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
    match tag {
        RuleTag::PredicatedArgumentFromExtend | RuleTag::PredicatedArgumentBareExtend => {
            ConstructionEvidence::guard("keyword-grant conjunction gate")
        }
        RuleTag::RulesObjectFollowupNominalRelative
        | RuleTag::RulesObjectFollowupNominalPrepositional => {
            ConstructionEvidence::role("rules-object attachment role")
        }
        RuleTag::NominalPrepositional => ConstructionEvidence::feature("nominal attachment phase"),
        _ => ConstructionEvidence::structural("production shape"),
    }
}

fn family(tag: RuleTag) -> ConstructionFamily {
    ConstructionFamily::new(
        construction_id(tag),
        ConstructionOwner::Handwritten,
        ConstructionBackend::Chart,
        evidence(tag),
    )
}

fn dominance_edges() -> [DominanceEdge; 19] {
    let edge = |dominant, subordinate| {
        DominanceEdge::new(construction_id(dominant), construction_id(subordinate))
    };
    [
        // These relationships make the grammar's former insertion-order
        // preferences explicit. They were censused against the full English
        // suite when construction identity replaced numeric rule order.
        edge(
            RuleTag::NominalQuantityModifier,
            RuleTag::NominalPrepositional,
        ),
        edge(
            RuleTag::NominalQuantityModifier,
            RuleTag::NominalPostpositiveAdjective,
        ),
        edge(RuleTag::NominalQuantityModifier, RuleTag::NominalComparison),
        edge(RuleTag::NominalDeterminer, RuleTag::NominalComparison),
        edge(RuleTag::NominalPrepositional, RuleTag::NominalInfinitive),
        edge(
            RuleTag::NominalPrepositional,
            RuleTag::NominalCoordinatedModifier,
        ),
        edge(
            RuleTag::NominalPrepositional,
            RuleTag::NominalKeywordPredicatedArgument,
        ),
        edge(RuleTag::NominalPrepositional, RuleTag::NominalNoun),
        edge(
            RuleTag::NominalReducedRecipientPassive,
            RuleTag::NominalNoun,
        ),
        edge(RuleTag::NominalRelative, RuleTag::NominalPrepositional),
        edge(RuleTag::NounPhraseNominal, RuleTag::NounPhraseCoordination),
        edge(RuleTag::NounPhraseNominal, RuleTag::NounPhraseMinus),
        edge(
            RuleTag::NounPhraseSubjectPronoun,
            RuleTag::NounPhraseObjectPronoun,
        ),
        edge(RuleTag::VerbPhraseAuxiliary, RuleTag::VerbPhraseAdjective),
        edge(RuleTag::VerbPhraseAuxiliary, RuleTag::VerbPhraseAdverb),
        edge(RuleTag::VerbPhraseAuxiliary, RuleTag::VerbPhraseAbility),
        edge(RuleTag::VerbPhraseBase, RuleTag::VerbPhraseAuxiliaryProform),
        edge(RuleTag::ClauseSimple, RuleTag::ClauseCopular),
        edge(RuleTag::RelativeSubject, RuleTag::RelativeObject),
    ]
}

pub(super) fn registry() -> &'static ConstructionRegistry {
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

#[cfg_attr(
    not(test),
    allow(
        dead_code,
        reason = "consumed by test-assembly activation; production parses keep the handwritten-only registry() static"
    )
)]
pub(super) fn merged_registry(
    groups: &[&'static deckmaste_construction_compiler::runtime::GroupData],
) -> Result<ConstructionRegistry, ConstructionRegistryError> {
    let generated_families = groups
        .iter()
        .flat_map(|group| group.constructions.iter().map(generated_family));
    let generated_edges = groups.iter().flat_map(|group| {
        group.constructions.iter().flat_map(|construction| {
            construction.dominates.iter().map(|loser| {
                DominanceEdge::new(
                    ConstructionId::new(construction.id),
                    ConstructionId::new(loser),
                )
            })
        })
    });
    ConstructionRegistry::new(
        RuleTag::iter().map(family).chain(generated_families),
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
        ConstructionEvidence::structural("generated production"),
    )
}

#[cfg(test)]
mod tests {
    use strum::IntoEnumIterator;

    use super::super::RuleTag;
    use super::construction_id;
    use super::registry;
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
    fn every_handwritten_tag_has_exactly_one_registry_row() {
        let registry = registry();
        let tags = RuleTag::iter().collect::<Vec<_>>();
        assert_eq!(registry.families().len(), tags.len());
        for tag in tags {
            let id = construction_id(tag);
            let family = registry.family(id).expect("tag missing from registry");
            assert_eq!(family.owner(), ConstructionOwner::Handwritten);
            assert_eq!(family.backend(), ConstructionBackend::Chart);
        }
    }

    #[test]
    fn historic_registration_preferences_are_declared_as_dominance() {
        let preferences = [
            (
                RuleTag::NominalQuantityModifier,
                RuleTag::NominalPrepositional,
            ),
            (
                RuleTag::NominalQuantityModifier,
                RuleTag::NominalPostpositiveAdjective,
            ),
            (RuleTag::NominalQuantityModifier, RuleTag::NominalComparison),
            (RuleTag::NominalDeterminer, RuleTag::NominalComparison),
            (RuleTag::NominalPrepositional, RuleTag::NominalInfinitive),
            (
                RuleTag::NominalPrepositional,
                RuleTag::NominalCoordinatedModifier,
            ),
            (
                RuleTag::NominalPrepositional,
                RuleTag::NominalKeywordPredicatedArgument,
            ),
            (RuleTag::NominalPrepositional, RuleTag::NominalNoun),
            (
                RuleTag::NominalReducedRecipientPassive,
                RuleTag::NominalNoun,
            ),
            (RuleTag::NominalRelative, RuleTag::NominalPrepositional),
            (RuleTag::NounPhraseNominal, RuleTag::NounPhraseCoordination),
            (RuleTag::NounPhraseNominal, RuleTag::NounPhraseMinus),
            (
                RuleTag::NounPhraseSubjectPronoun,
                RuleTag::NounPhraseObjectPronoun,
            ),
            (RuleTag::VerbPhraseAuxiliary, RuleTag::VerbPhraseAdjective),
            (RuleTag::VerbPhraseAuxiliary, RuleTag::VerbPhraseAdverb),
            (RuleTag::VerbPhraseAuxiliary, RuleTag::VerbPhraseAbility),
            (RuleTag::VerbPhraseBase, RuleTag::VerbPhraseAuxiliaryProform),
            (RuleTag::ClauseSimple, RuleTag::ClauseCopular),
            (RuleTag::RelativeSubject, RuleTag::RelativeObject),
        ];
        for (dominant, subordinate) in preferences {
            assert!(
                registry().dominates(construction_id(dominant), construction_id(subordinate)),
                "{dominant:?} must dominate {subordinate:?}",
            );
        }
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
            constructions,
        }
    }

    #[test]
    fn generated_id_colliding_with_a_handwritten_family_is_refused() {
        const CONSTRUCTIONS: &[deckmaste_construction_compiler::runtime::ConstructionData] =
            &[deckmaste_construction_compiler::runtime::ConstructionData {
                id: "noun_phrase_coordination",
                category: "Internal",
                internal: true,
                own_type: Some("Synthetic"),
                bind_path: None,
                deserialize: false,
                selection_unique: false,
                dominates: &[],
                fields: &[],
                witnesses: &[],
                forms: &[],
            }];
        const GROUP: deckmaste_construction_compiler::runtime::GroupData =
            synthetic_group("g", CONSTRUCTIONS);
        let error = super::merged_registry(&[&GROUP]).unwrap_err();
        assert_eq!(
            error,
            ConstructionRegistryError::DuplicateId(construction_id(
                RuleTag::NounPhraseCoordination
            ))
        );
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
                deserialize: false,
                selection_unique: false,
                dominates: &["missing"],
                fields: &[],
                witnesses: &[],
                forms: &[],
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
                deserialize: false,
                selection_unique: false,
                dominates: &["gen_b"],
                fields: &[],
                witnesses: &[],
                forms: &[],
            }];
        const B_CONSTRUCTIONS: &[deckmaste_construction_compiler::runtime::ConstructionData] =
            &[deckmaste_construction_compiler::runtime::ConstructionData {
                id: "gen_b",
                category: "Internal",
                internal: true,
                own_type: Some("Synthetic"),
                bind_path: None,
                deserialize: false,
                selection_unique: false,
                dominates: &["gen_a"],
                fields: &[],
                witnesses: &[],
                forms: &[],
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
                deserialize: false,
                selection_unique: false,
                dominates: &[],
                fields: &[],
                witnesses: &[],
                forms: &[],
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
    fn production_registry_is_untouched() {
        // `merged_registry(&[])` (no active generated groups) must carry
        // exactly the same rows as the real `registry()` static — the
        // guarantee that activation changes nothing for production parses.
        let merged = super::merged_registry(&[]).expect("no groups must merge cleanly");
        let tags = RuleTag::iter().collect::<Vec<_>>();
        assert_eq!(merged.families().len(), tags.len());
        assert_eq!(merged.families().len(), registry().families().len());
        for tag in tags {
            let id = construction_id(tag);
            let production_family = registry().family(id).expect("tag missing from registry()");
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
                assert!(std::ptr::eq(selected, registry()));
            }
            super::super::parse_support::SelectedRegistry::Owned(_) => {
                panic!("Inactive must select the registry() static, not a reconstruction");
            }
        }
    }
}
