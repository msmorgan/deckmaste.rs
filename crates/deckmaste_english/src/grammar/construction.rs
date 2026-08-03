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

pub(super) fn registry() -> &'static ConstructionRegistry {
    static REGISTRY: OnceLock<ConstructionRegistry> = OnceLock::new();
    REGISTRY.get_or_init(|| {
        ConstructionRegistry::new(
            RuleTag::iter().map(family),
            [DominanceEdge::new(
                construction_id(RuleTag::NominalPrepositional),
                construction_id(RuleTag::NominalKeywordPredicatedArgument),
            )],
        )
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
    fn known_registration_accident_is_declared_as_dominance() {
        assert!(registry().dominates(
            construction_id(RuleTag::NominalPrepositional),
            construction_id(RuleTag::NominalKeywordPredicatedArgument),
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
}
