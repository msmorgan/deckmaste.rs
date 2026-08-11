//! Spelling's mutable pattern tree over the English construction projection.
//!
//! The English crate owns selection of constructions and forms. This module
//! adds only the two things a frame compiler needs: stable paths through
//! named grammatical roles, and hole nodes. No Rust type, enum, field, or
//! serde identity is represented here.

use std::collections::BTreeMap;
use std::fmt;

use deckmaste_english::ConstructionProjection;
use deckmaste_english::ProjectedAtom;
use deckmaste_english::ProjectedMember;
use deckmaste_english::ProjectedValue;
use deckmaste_english::ProjectedWitness;

use crate::HoleClass;

/// One stable hop through a construction projection.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd)]
pub enum ProjectionStep {
    /// A named construction or element role.
    Role(&'static str),
    /// The payload of a present optional role.
    Present,
    /// A member of a declaration-owned sequence, in semantic surface order.
    Member(usize),
}

impl fmt::Display for ProjectionStep {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Role(role) => write!(formatter, ".{role}"),
            Self::Present => formatter.write_str("?"),
            Self::Member(index) => write!(formatter, "[{index}]"),
        }
    }
}

/// A stable address inside a [`ProjectionTree`].
#[derive(Debug, Clone, Default, PartialEq, Eq, Hash, Ord, PartialOrd)]
pub struct ProjectionPath(pub Vec<ProjectionStep>);

impl ProjectionPath {
    #[must_use]
    pub fn root() -> Self {
        Self::default()
    }

    #[must_use]
    pub fn then(&self, step: ProjectionStep) -> Self {
        let mut steps = self.0.clone();
        steps.push(step);
        Self(steps)
    }

    #[must_use]
    pub fn parent(&self) -> Option<Self> {
        (!self.0.is_empty()).then(|| Self(self.0[..self.0.len() - 1].to_vec()))
    }

    #[must_use]
    pub fn is_prefix_of(&self, other: &Self) -> bool {
        other.0.starts_with(&self.0)
    }

    #[must_use]
    pub fn resolve<'tree>(&self, root: &'tree ProjectionTree) -> Option<&'tree ProjectionTree> {
        self.0.iter().try_fold(root, |node, step| node.child(*step))
    }

    #[must_use]
    pub fn resolve_mut<'tree>(
        &self,
        root: &'tree mut ProjectionTree,
    ) -> Option<&'tree mut ProjectionTree> {
        let mut node = root;
        for step in &self.0 {
            node = node.child_mut(*step)?;
        }
        Some(node)
    }

    /// Rebases this path below `prefix`, if `prefix` addresses an ancestor.
    #[must_use]
    pub fn strip_prefix(&self, prefix: &Self) -> Option<Self> {
        self.0
            .strip_prefix(prefix.0.as_slice())
            .map(|steps| Self(steps.to_vec()))
    }
}

impl fmt::Display for ProjectionPath {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.0.is_empty() {
            return formatter.write_str("<root>");
        }
        for step in &self.0 {
            write!(formatter, "{step}")?;
        }
        Ok(())
    }
}

/// One selected construction, with declaration-owned identity and roles.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ConstructionNode {
    pub category: &'static str,
    pub construction: &'static str,
    pub form: &'static str,
    pub ordinal: u16,
    pub roles: BTreeMap<&'static str, ProjectionTree>,
    pub witnesses: BTreeMap<&'static str, ProjectedWitness>,
    pub literals: Vec<&'static str>,
}

/// One member of a declaration-owned sequence.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ElementNode {
    pub element: &'static str,
    pub variant: Option<&'static str>,
    pub roles: BTreeMap<&'static str, ProjectionTree>,
}

/// One selected alternative of a declaration-owned sum role.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VariantNode {
    pub element: &'static str,
    pub variant: &'static str,
    pub roles: BTreeMap<&'static str, ProjectionTree>,
}

/// One declaration-owned record product.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProductNode {
    pub element: &'static str,
    pub roles: BTreeMap<&'static str, ProjectionTree>,
}

/// The compiler-owned construction tree extended with spelling holes.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ProjectionTree {
    Construction(ConstructionNode),
    Element(ElementNode),
    Variant(VariantNode),
    Product(ProductNode),
    Atom(ProjectedAtom),
    Optional(Option<Box<ProjectionTree>>),
    Sequence {
        role: &'static str,
        members: Vec<ProjectionTree>,
    },
    Hole {
        index: usize,
        class: HoleClass,
    },
}

impl ProjectionTree {
    #[must_use]
    pub fn from_projection(projection: ConstructionProjection) -> Self {
        Self::Construction(ConstructionNode {
            category: projection.category,
            construction: projection.construction,
            form: projection.form,
            ordinal: projection.ordinal,
            roles: projection
                .roles
                .into_iter()
                .map(|(role, value)| (role, Self::from_value(value)))
                .collect(),
            witnesses: projection.witnesses,
            literals: projection.literals,
        })
    }

    fn from_value(value: ProjectedValue) -> Self {
        match value {
            ProjectedValue::Construction(projection) => Self::from_projection(*projection),
            ProjectedValue::Atom(atom) => Self::Atom(atom),
            ProjectedValue::Optional(value) => {
                Self::Optional(value.map(|value| Box::new(Self::from_value(*value))))
            }
            ProjectedValue::Sequence(sequence) => Self::Sequence {
                role: sequence.role,
                members: sequence
                    .members
                    .into_iter()
                    .map(Self::from_member)
                    .collect(),
            },
            ProjectedValue::Variant(variant) => Self::Variant(VariantNode {
                element: variant.element,
                variant: variant.variant,
                roles: variant
                    .roles
                    .into_iter()
                    .map(|(role, value)| (role, Self::from_value(value)))
                    .collect(),
            }),
            ProjectedValue::Product(product) => Self::Product(ProductNode {
                element: product.element,
                roles: product
                    .roles
                    .into_iter()
                    .map(|(role, value)| (role, Self::from_value(value)))
                    .collect(),
            }),
        }
    }

    fn from_member(member: ProjectedMember) -> Self {
        Self::Element(ElementNode {
            element: member.element,
            variant: member.variant,
            roles: member
                .roles
                .into_iter()
                .map(|(role, value)| (role, Self::from_value(value)))
                .collect(),
        })
    }

    #[must_use]
    pub fn construction(&self) -> Option<&ConstructionNode> {
        match self {
            Self::Construction(node) => Some(node),
            _ => None,
        }
    }

    #[must_use]
    pub fn construction_mut(&mut self) -> Option<&mut ConstructionNode> {
        match self {
            Self::Construction(node) => Some(node),
            _ => None,
        }
    }

    #[must_use]
    pub fn atom(&self) -> Option<&ProjectedAtom> {
        match self {
            Self::Atom(atom) => Some(atom),
            _ => None,
        }
    }

    #[must_use]
    pub fn role(&self, role: &str) -> Option<&Self> {
        match self {
            Self::Construction(node) => node.roles.get(role),
            Self::Element(node) => node.roles.get(role),
            Self::Variant(node) => node.roles.get(role),
            Self::Product(node) => node.roles.get(role),
            _ => None,
        }
    }

    #[must_use]
    pub fn role_mut(&mut self, role: &str) -> Option<&mut Self> {
        match self {
            Self::Construction(node) => node.roles.get_mut(role),
            Self::Element(node) => node.roles.get_mut(role),
            Self::Variant(node) => node.roles.get_mut(role),
            Self::Product(node) => node.roles.get_mut(role),
            _ => None,
        }
    }

    #[must_use]
    pub fn is_vacuous(&self) -> bool {
        match self {
            Self::Optional(None) => true,
            Self::Sequence { members, .. } => members.is_empty(),
            _ => false,
        }
    }

    #[must_use]
    pub fn children(&self) -> Vec<(ProjectionStep, &Self)> {
        match self {
            Self::Construction(node) => node
                .roles
                .iter()
                .map(|(role, value)| (ProjectionStep::Role(role), value))
                .collect(),
            Self::Element(node) => node
                .roles
                .iter()
                .map(|(role, value)| (ProjectionStep::Role(role), value))
                .collect(),
            Self::Variant(node) => node
                .roles
                .iter()
                .map(|(role, value)| (ProjectionStep::Role(role), value))
                .collect(),
            Self::Product(node) => node
                .roles
                .iter()
                .map(|(role, value)| (ProjectionStep::Role(role), value))
                .collect(),
            Self::Optional(Some(value)) => vec![(ProjectionStep::Present, value.as_ref())],
            Self::Sequence { members, .. } => members
                .iter()
                .enumerate()
                .map(|(index, value)| (ProjectionStep::Member(index), value))
                .collect(),
            Self::Atom(_) | Self::Optional(None) | Self::Hole { .. } => Vec::new(),
        }
    }

    #[must_use]
    pub fn walk(&self) -> Vec<(ProjectionPath, &Self)> {
        let mut out = Vec::new();
        self.walk_into(&ProjectionPath::root(), &mut out);
        out
    }

    fn walk_into<'tree>(
        &'tree self,
        at: &ProjectionPath,
        out: &mut Vec<(ProjectionPath, &'tree Self)>,
    ) {
        out.push((at.clone(), self));
        for (step, child) in self.children() {
            child.walk_into(&at.then(step), out);
        }
    }

    fn child(&self, step: ProjectionStep) -> Option<&Self> {
        match (self, step) {
            (Self::Construction(node), ProjectionStep::Role(role)) => node.roles.get(role),
            (Self::Element(node), ProjectionStep::Role(role)) => node.roles.get(role),
            (Self::Variant(node), ProjectionStep::Role(role)) => node.roles.get(role),
            (Self::Product(node), ProjectionStep::Role(role)) => node.roles.get(role),
            (Self::Optional(Some(value)), ProjectionStep::Present) => Some(value),
            (Self::Sequence { members, .. }, ProjectionStep::Member(index)) => members.get(index),
            _ => None,
        }
    }

    fn child_mut(&mut self, step: ProjectionStep) -> Option<&mut Self> {
        match (self, step) {
            (Self::Construction(node), ProjectionStep::Role(role)) => node.roles.get_mut(role),
            (Self::Element(node), ProjectionStep::Role(role)) => node.roles.get_mut(role),
            (Self::Variant(node), ProjectionStep::Role(role)) => node.roles.get_mut(role),
            (Self::Product(node), ProjectionStep::Role(role)) => node.roles.get_mut(role),
            (Self::Optional(Some(value)), ProjectionStep::Present) => Some(value),
            (Self::Sequence { members, .. }, ProjectionStep::Member(index)) => {
                members.get_mut(index)
            }
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use deckmaste_english::ProjectedProduct;
    use deckmaste_english::ProjectedVariant;

    use super::*;

    #[test]
    fn direct_sum_variants_keep_identity_and_role_paths() {
        let projection = ConstructionProjection {
            category: "Ability",
            construction: "ability",
            form: "keyword",
            ordinal: 9,
            roles: BTreeMap::from([
                (
                    "kind",
                    ProjectedValue::Variant(ProjectedVariant {
                        element: "ability_kind",
                        variant: "Keyword",
                        roles: BTreeMap::from([(
                            "payload",
                            ProjectedValue::Atom(ProjectedAtom::DerivedSequenceScalar {
                                codec: "Fixture",
                                index: 0,
                                len: 1,
                            }),
                        )]),
                    }),
                ),
                (
                    "record",
                    ProjectedValue::Product(ProjectedProduct {
                        element: "fixture_record",
                        roles: BTreeMap::from([(
                            "field",
                            ProjectedValue::Atom(ProjectedAtom::DerivedSequenceScalar {
                                codec: "ProductFixture",
                                index: 1,
                                len: 2,
                            }),
                        )]),
                    }),
                ),
            ]),
            witnesses: BTreeMap::new(),
            literals: Vec::new(),
        };

        let tree = ProjectionTree::from_projection(projection);
        let kind = tree
            .role("kind")
            .expect("the kind role survives conversion");
        let ProjectionTree::Variant(kind) = kind else {
            panic!("the direct sum must remain a variant node: {kind:#?}");
        };
        assert_eq!((kind.element, kind.variant), ("ability_kind", "Keyword"));
        let payload = ProjectionPath(vec![
            ProjectionStep::Role("kind"),
            ProjectionStep::Role("payload"),
        ]);
        assert!(matches!(
            payload.resolve(&tree),
            Some(ProjectionTree::Atom(ProjectedAtom::DerivedSequenceScalar {
                codec: "Fixture",
                index: 0,
                len: 1,
            }))
        ));
        assert!(tree.walk().iter().any(|(path, _)| path == &payload));

        let record = tree
            .role("record")
            .expect("the product role survives conversion");
        let ProjectionTree::Product(record) = record else {
            panic!("the record must remain a product node: {record:#?}");
        };
        assert_eq!(record.element, "fixture_record");
        assert!(matches!(
            tree.role("record").and_then(|record| record.role("field")),
            Some(ProjectionTree::Atom(ProjectedAtom::DerivedSequenceScalar {
                codec: "ProductFixture",
                index: 1,
                len: 2,
            }))
        ));
    }
}
