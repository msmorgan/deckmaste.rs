use std::collections::HashMap;
use std::collections::HashSet;
use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd)]
pub struct ConstructionId(&'static str);

impl ConstructionId {
    pub(crate) const fn new(name: &'static str) -> Self {
        Self(name)
    }

    #[must_use]
    pub const fn as_str(self) -> &'static str {
        self.0
    }
}

impl fmt::Display for ConstructionId {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.0)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConstructionOwner {
    Handwritten,
    Generated,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConstructionBackend {
    Chart,
    Ability,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConstructionEvidenceKind {
    Structural,
    Guard,
    Feature,
    Role,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ConstructionEvidence {
    kind: ConstructionEvidenceKind,
    label: &'static str,
}

impl ConstructionEvidence {
    pub(crate) const fn structural(label: &'static str) -> Self {
        Self {
            kind: ConstructionEvidenceKind::Structural,
            label,
        }
    }

    pub(crate) const fn guard(label: &'static str) -> Self {
        Self {
            kind: ConstructionEvidenceKind::Guard,
            label,
        }
    }

    pub(crate) const fn feature(label: &'static str) -> Self {
        Self {
            kind: ConstructionEvidenceKind::Feature,
            label,
        }
    }

    pub(crate) const fn role(label: &'static str) -> Self {
        Self {
            kind: ConstructionEvidenceKind::Role,
            label,
        }
    }

    #[must_use]
    pub const fn kind(self) -> ConstructionEvidenceKind {
        self.kind
    }

    #[must_use]
    pub const fn label(self) -> &'static str {
        self.label
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ConstructionFamily {
    id: ConstructionId,
    owner: ConstructionOwner,
    backend: ConstructionBackend,
    evidence: ConstructionEvidence,
}

impl ConstructionFamily {
    pub(crate) const fn new(
        id: ConstructionId,
        owner: ConstructionOwner,
        backend: ConstructionBackend,
        evidence: ConstructionEvidence,
    ) -> Self {
        Self {
            id,
            owner,
            backend,
            evidence,
        }
    }

    #[must_use]
    pub const fn id(self) -> ConstructionId {
        self.id
    }

    #[must_use]
    pub const fn owner(self) -> ConstructionOwner {
        self.owner
    }

    #[must_use]
    pub const fn backend(self) -> ConstructionBackend {
        self.backend
    }

    #[must_use]
    pub const fn evidence(self) -> ConstructionEvidence {
        self.evidence
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DominanceEdge {
    dominant: ConstructionId,
    subordinate: ConstructionId,
}

impl DominanceEdge {
    pub(crate) const fn new(dominant: ConstructionId, subordinate: ConstructionId) -> Self {
        Self {
            dominant,
            subordinate,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum ConstructionRegistryError {
    DuplicateId(ConstructionId),
    UnknownDominanceTarget(ConstructionId),
    SelfDominance(ConstructionId),
    DominanceCycle,
}

#[derive(Debug)]
pub(crate) struct ConstructionRegistry {
    families: Vec<ConstructionFamily>,
    family_indices: HashMap<ConstructionId, usize>,
    dominance: HashMap<ConstructionId, Vec<ConstructionId>>,
}

impl ConstructionRegistry {
    pub(crate) fn new(
        families: impl IntoIterator<Item = ConstructionFamily>,
        edges: impl IntoIterator<Item = DominanceEdge>,
    ) -> Result<Self, ConstructionRegistryError> {
        let families = families.into_iter().collect::<Vec<_>>();
        let mut family_indices = HashMap::with_capacity(families.len());
        for (index, family) in families.iter().copied().enumerate() {
            if family_indices.insert(family.id(), index).is_some() {
                return Err(ConstructionRegistryError::DuplicateId(family.id()));
            }
        }

        let mut dominance = HashMap::<ConstructionId, Vec<ConstructionId>>::new();
        for edge in edges {
            if !family_indices.contains_key(&edge.dominant) {
                return Err(ConstructionRegistryError::UnknownDominanceTarget(
                    edge.dominant,
                ));
            }
            if !family_indices.contains_key(&edge.subordinate) {
                return Err(ConstructionRegistryError::UnknownDominanceTarget(
                    edge.subordinate,
                ));
            }
            if edge.dominant == edge.subordinate {
                return Err(ConstructionRegistryError::SelfDominance(edge.dominant));
            }
            dominance
                .entry(edge.dominant)
                .or_default()
                .push(edge.subordinate);
        }

        let registry = Self {
            families,
            family_indices,
            dominance,
        };
        if registry.has_dominance_cycle() {
            return Err(ConstructionRegistryError::DominanceCycle);
        }
        Ok(registry)
    }

    pub(crate) fn families(&self) -> &[ConstructionFamily] {
        &self.families
    }

    pub(crate) fn family(&self, id: ConstructionId) -> Option<ConstructionFamily> {
        self.family_indices
            .get(&id)
            .map(|&index| self.families[index])
    }

    pub(crate) fn dominates(&self, dominant: ConstructionId, subordinate: ConstructionId) -> bool {
        let mut pending = self
            .dominance
            .get(&dominant)
            .into_iter()
            .flatten()
            .copied()
            .collect::<Vec<_>>();
        let mut visited = HashSet::new();
        while let Some(candidate) = pending.pop() {
            if candidate == subordinate {
                return true;
            }
            if visited.insert(candidate)
                && let Some(children) = self.dominance.get(&candidate)
            {
                pending.extend(children.iter().copied());
            }
        }
        false
    }

    fn has_dominance_cycle(&self) -> bool {
        self.families.iter().any(|family| {
            let id = family.id();
            self.dominates(id, id)
        })
    }
}
