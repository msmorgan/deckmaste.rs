use std::sync::OnceLock;

use strum::IntoEnumIterator;

use super::RuleTag;
use crate::construction::ConstructionBackend;
use crate::construction::ConstructionEvidence;
use crate::construction::ConstructionFamily;
use crate::construction::ConstructionId;
use crate::construction::ConstructionOwner;
use crate::construction::ConstructionRegistry;
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
}
