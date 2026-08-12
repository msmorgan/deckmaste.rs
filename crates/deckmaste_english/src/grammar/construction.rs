use std::sync::OnceLock;

use crate::construction::ConstructionBackend;
use crate::construction::ConstructionEvidence;
use crate::construction::ConstructionFamily;
use crate::construction::ConstructionId;
use crate::construction::ConstructionRegistry;
use crate::construction::ConstructionRegistryError;
use crate::construction::DominanceEdge;

pub(super) fn registry() -> &'static ConstructionRegistry {
    static REGISTRY: OnceLock<ConstructionRegistry> = OnceLock::new();
    REGISTRY.get_or_init(|| {
        registry_from_groups(crate::constructions::ALL_GROUPS)
            .expect("production construction registry must be valid")
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

pub(super) fn registry_from_groups(
    groups: &[&'static deckmaste_construction_compiler::runtime::GroupData],
) -> Result<ConstructionRegistry, ConstructionRegistryError> {
    let families = groups.iter().flat_map(|group| {
        group
            .constructions
            .iter()
            .map(|construction| generated_family(group.backend, construction))
    });
    let edges = groups.iter().flat_map(|group| {
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
    ConstructionRegistry::new(families, edges)
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
    use super::registry;
    use super::registry_from_groups;
    use crate::construction::ConstructionBackend;
    use crate::construction::ConstructionId;
    use crate::construction::ConstructionRegistryError;

    #[test]
    fn production_registry_is_exactly_the_generated_declaration_set() {
        let registry = registry();
        let declared = crate::constructions::ALL_GROUPS
            .iter()
            .flat_map(|group| group.constructions)
            .map(|construction| construction.id)
            .collect::<Vec<_>>();
        let registered = registry
            .families()
            .iter()
            .map(|family| family.id().as_str())
            .collect::<Vec<_>>();

        assert_eq!(registered, declared);
        assert_eq!(registered.len(), 229);
        assert_eq!(registered.iter().filter(|id| **id == "ability").count(), 1);
    }

    #[test]
    fn production_registry_preserves_each_generated_backend() {
        for group in crate::constructions::ALL_GROUPS {
            let expected = match group.backend {
                deckmaste_construction_compiler::runtime::ConstructionBackendData::Chart => {
                    ConstructionBackend::Chart
                }
                deckmaste_construction_compiler::runtime::ConstructionBackendData::Ability => {
                    ConstructionBackend::Ability
                }
            };
            for declaration in group.constructions {
                let family = registry()
                    .family(ConstructionId::new(declaration.id))
                    .unwrap_or_else(|| panic!("missing generated family {}", declaration.id));
                assert_eq!(family.backend(), expected, "{}", declaration.id);
            }
        }
    }

    #[test]
    fn generated_registry_assembly_is_group_permutation_neutral() {
        let normal = registry_from_groups(crate::constructions::ALL_GROUPS).unwrap();
        let mut groups = crate::constructions::ALL_GROUPS.to_vec();
        groups.reverse();
        let reversed = registry_from_groups(&groups).unwrap();

        for family in normal.families() {
            let id = family.id();
            let other = reversed
                .family(id)
                .unwrap_or_else(|| panic!("reversed assembly omitted {id}"));
            assert_eq!(family.backend(), other.backend(), "{id}");
            for possible_subordinate in normal.families() {
                assert_eq!(
                    normal.dominates(id, possible_subordinate.id()),
                    reversed.dominates(id, possible_subordinate.id()),
                    "dominance changed for {id} -> {}",
                    possible_subordinate.id(),
                );
            }
        }
    }

    #[test]
    fn isolated_generated_assembly_rejects_missing_cross_group_target() {
        let group = crate::constructions::coordination::GROUPS[0];
        assert!(matches!(
            registry_from_groups(&[group]),
            Err(ConstructionRegistryError::UnknownDominanceTarget(_))
        ));
    }

    #[test]
    fn empty_generated_assembly_is_valid_and_empty() {
        let registry = registry_from_groups(&[]).unwrap();
        assert!(registry.families().is_empty());
    }
}
