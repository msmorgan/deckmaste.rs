//! Declaration-generated construction projections for spelling consumers.
//!
//! This is a transient compiler surface: stable construction/form identities,
//! named grammatical roles, typed scalar/lexical identities, sequences, and
//! retained witnesses. Rust storage layout and serde output are deliberately
//! absent.

use std::collections::BTreeMap;

use deckmaste_construction_compiler::runtime::ConstructionProjection;
use deckmaste_construction_compiler::runtime::ErasedLinearizationError;
use deckmaste_construction_compiler::runtime::LinearizationError;
use deckmaste_construction_compiler::runtime::OwnedProjectionValue;
use deckmaste_construction_compiler::runtime::ProjectedAtom;
use deckmaste_construction_compiler::runtime::ProjectedMember;
use deckmaste_construction_compiler::runtime::ProjectedProduct;
use deckmaste_construction_compiler::runtime::ProjectedSequence;
use deckmaste_construction_compiler::runtime::ProjectedValue;
use deckmaste_construction_compiler::runtime::ProjectedVariant;
use deckmaste_construction_compiler::runtime::ProjectedWitness;
use deckmaste_construction_compiler::runtime::ProjectionInput;
use deckmaste_construction_compiler::runtime::ProjectionSink;
use deckmaste_construction_compiler::runtime::ProjectionValue;

use crate::Fragment;
use crate::syntax::NumberLiteral;
use crate::syntax::PowerToughness;
use crate::syntax::SignedScalar;
use crate::word::NounInstance;

#[derive(Debug, thiserror::Error)]
pub enum ProjectionError {
    #[error("no declared construction projects category `{category}`")]
    NoConstruction { category: &'static str },
    #[error("category `{category}` has incomparable construction projections: {constructions:?}")]
    Incomparable {
        category: &'static str,
        constructions: Vec<&'static str>,
    },
    #[error("construction projection for `{construction}` failed: {message}")]
    Invalid {
        construction: &'static str,
        message: String,
    },
}

/// Projects one parsed fragment through its selected construction
/// declarations.
///
/// # Errors
///
/// Returns an error when no declaration recognizes the value, when two
/// incomparable declarations claim it, or when a generated projection event
/// stream is structurally invalid.
pub fn project_fragment(fragment: &Fragment) -> Result<ConstructionProjection, ProjectionError> {
    match fragment {
        Fragment::Nominal(value) => project_value("NounPhrase", value),
        Fragment::Sentence(value) => project_value("Sentence", value),
        Fragment::Cost(value) => project_value("Cost", value),
        Fragment::KeywordLine(value) => project_value("KeywordAbilityList", value),
        Fragment::Ability(value) => project_value("Ability", value),
    }
}

fn project_value(
    category: &'static str,
    value: &dyn ProjectionInput,
) -> Result<ConstructionProjection, ProjectionError> {
    let mut candidates = Vec::new();
    for construction in crate::constructions::ALL_GROUPS
        .iter()
        .flat_map(|group| group.constructions)
        .filter(|construction| construction.category == category)
    {
        let mut collector = ProjectionCollector::new(category, construction.id);
        match (construction.erased_linearizer)(value, &mut collector) {
            Ok(()) => candidates.push(collector.finish()?),
            Err(
                ErasedLinearizationError::WrongValueType { .. }
                | ErasedLinearizationError::Linearization(
                    LinearizationError::NoMatchingConstruction { .. }
                    | LinearizationError::NoMatchingForm { .. },
                ),
            ) => {}
            Err(ErasedLinearizationError::Linearization(error)) => {
                return Err(ProjectionError::Invalid {
                    construction: construction.id,
                    message: format!("{error:?}"),
                });
            }
        }
    }

    candidates.sort_by_key(|projection| projection.construction);
    candidates.dedup_by_key(|projection| projection.construction);
    match candidates.len() {
        0 => Err(ProjectionError::NoConstruction { category }),
        1 => Ok(candidates.pop().expect("one projection candidate")),
        _ => {
            let maximal = candidates
                .iter()
                .filter(|candidate| {
                    !candidates.iter().any(|other| {
                        other.construction != candidate.construction
                            && crate::grammar::construction::dominates(
                                other.construction,
                                candidate.construction,
                            )
                    })
                })
                .collect::<Vec<_>>();
            if let [winner] = maximal.as_slice() {
                return Ok((*winner).clone());
            }
            Err(ProjectionError::Incomparable {
                category,
                constructions: maximal
                    .into_iter()
                    .map(|projection| projection.construction)
                    .collect(),
            })
        }
    }
}

fn atom_scalar(codec: &'static str, value: &dyn ProjectionValue) -> ProjectedValue {
    ProjectedValue::Atom(ProjectedAtom::Scalar {
        codec,
        value: OwnedProjectionValue::new(value),
    })
}

fn atom_identity(
    provider: &'static str,
    value_type: &'static str,
    value: &dyn ProjectionValue,
) -> ProjectedValue {
    ProjectedValue::Atom(ProjectedAtom::Identity {
        provider,
        value_type,
        value: OwnedProjectionValue::new(value),
    })
}

fn flat_projection(
    category: &'static str,
    construction: &'static str,
    form: &'static str,
    ordinal: u16,
    roles: BTreeMap<&'static str, ProjectedValue>,
) -> ConstructionProjection {
    ConstructionProjection {
        category,
        construction,
        form,
        ordinal,
        roles,
        witnesses: BTreeMap::new(),
        literals: Vec::new(),
    }
}

fn project_flat_subtree(
    category: &'static str,
    value: &dyn ProjectionValue,
) -> Option<ConstructionProjection> {
    if category == "PowerToughness" {
        return value
            .as_any()
            .downcast_ref::<PowerToughness>()
            .copied()
            .map(project_power_toughness);
    }
    if category == "NumberLiteral" {
        return value
            .as_any()
            .downcast_ref::<NumberLiteral>()
            .copied()
            .map(project_number_literal);
    }
    if category == "SignedScalar" {
        return value
            .as_any()
            .downcast_ref::<SignedScalar>()
            .copied()
            .map(project_signed_scalar);
    }
    None
}

fn project_subtree(
    category: &'static str,
    value: &dyn ProjectionValue,
) -> Result<ConstructionProjection, ProjectionError> {
    if category == "NounInstance" && value.as_any().is::<NounInstance>() {
        return project_value("Noun", value);
    }
    if category == "Clause"
        && let Some(value) = value
            .as_any()
            .downcast_ref::<crate::syntax::IndependentClause>()
    {
        let clause = crate::syntax::Clause::Independent(value.clone());
        return project_value(category, &clause);
    }
    project_value(category, value)
}

fn projected_subtree(
    category: &'static str,
    value: &dyn ProjectionValue,
) -> Result<ProjectedValue, String> {
    match project_subtree(category, value) {
        Ok(projection) => Ok(ProjectedValue::Construction(Box::new(projection))),
        Err(ProjectionError::NoConstruction { .. }) => Ok(project_flat_subtree(category, value)
            .map_or_else(
                || {
                    ProjectedValue::Atom(ProjectedAtom::FlatSubtree {
                        category,
                        value: OwnedProjectionValue::new(value),
                    })
                },
                |projection| ProjectedValue::Construction(Box::new(projection)),
            )),
        Err(error) => Err(error.to_string()),
    }
}

fn project_catalog_atom(value: &crate::catalog::CatalogAtom) -> ConstructionProjection {
    let mut projection =
        flat_projection(
            "CatalogAtom",
            "flat_catalog_atom",
            "only",
            0,
            BTreeMap::from([
                (
                    "kind",
                    atom_identity("CatalogKind", "CatalogKind", &value.kind),
                ),
                (
                    "canonical",
                    atom_identity("CatalogCanonical", "String", &value.canonical().to_owned()),
                ),
                (
                    "vocabulary",
                    ProjectedValue::Optional(value.vocab.map(|vocab| {
                        Box::new(atom_identity("CatalogVocabulary", "Vocab", &vocab))
                    })),
                ),
            ]),
        );
    projection.witnesses.insert(
        "spelling",
        ProjectedWitness {
            path: "spelling",
            value: OwnedProjectionValue::new(&value.spelling().to_owned()),
        },
    );
    projection.witnesses.insert(
        "noun_spelling",
        ProjectedWitness {
            path: "render_noun",
            value: OwnedProjectionValue::new(&value.render_noun(false)),
        },
    );
    projection
}

fn project_flat_scalar(
    codec: &'static str,
    value: &dyn ProjectionValue,
) -> Option<ConstructionProjection> {
    if codec == "Numeral" {
        return value
            .as_any()
            .downcast_ref::<NumberLiteral>()
            .copied()
            .map(project_number_literal);
    }
    if codec == "PowerToughness" {
        return value
            .as_any()
            .downcast_ref::<PowerToughness>()
            .copied()
            .map(project_power_toughness);
    }
    None
}

fn project_number_literal(value: NumberLiteral) -> ConstructionProjection {
    flat_projection(
        "NumberLiteral",
        "flat_number_literal",
        "only",
        0,
        BTreeMap::from([
            ("value", atom_scalar("Integer", &value.value)),
            ("notation", atom_scalar("Numeral", &value.numeral)),
        ]),
    )
}

fn project_power_toughness(value: PowerToughness) -> ConstructionProjection {
    flat_projection(
        "PowerToughness",
        "flat_power_toughness",
        "only",
        0,
        BTreeMap::from([
            (
                "power",
                ProjectedValue::Construction(Box::new(project_signed_scalar(value.power))),
            ),
            (
                "toughness",
                ProjectedValue::Construction(Box::new(project_signed_scalar(value.toughness))),
            ),
        ]),
    )
}

fn project_signed_scalar(value: SignedScalar) -> ConstructionProjection {
    flat_projection(
        "SignedScalar",
        "flat_signed_scalar",
        "only",
        0,
        BTreeMap::from([
            ("sign", atom_scalar("ScalarSign", &value.sign)),
            ("value", atom_scalar("ScalarValue", &value.value)),
        ]),
    )
}

fn project_flat_identity(
    _provider: &'static str,
    value: &dyn ProjectionValue,
) -> Option<ConstructionProjection> {
    if let Some(value) = value.as_any().downcast_ref::<crate::catalog::CatalogAtom>() {
        return Some(project_catalog_atom(value));
    }
    None
}

struct OpenSequence {
    role: &'static str,
    expected_len: usize,
    members: Vec<ProjectedMember>,
    opened_at: usize,
}

struct OpenVariant {
    role: &'static str,
    element: &'static str,
    variant: &'static str,
    roles: BTreeMap<&'static str, ProjectedValue>,
    opened_at: usize,
}

struct OpenProduct {
    role: &'static str,
    element: &'static str,
    roles: BTreeMap<&'static str, ProjectedValue>,
    opened_at: usize,
}

struct ProjectionCollector {
    category: &'static str,
    expected_construction: &'static str,
    construction: Option<&'static str>,
    form: Option<&'static str>,
    ordinal: Option<u16>,
    roles: BTreeMap<&'static str, ProjectedValue>,
    witnesses: BTreeMap<&'static str, ProjectedWitness>,
    literals: Vec<&'static str>,
    sequences: Vec<OpenSequence>,
    variants: Vec<OpenVariant>,
    products: Vec<OpenProduct>,
    next_context: usize,
    ended: bool,
}

impl ProjectionCollector {
    fn new(category: &'static str, expected_construction: &'static str) -> Self {
        Self {
            category,
            expected_construction,
            construction: None,
            form: None,
            ordinal: None,
            roles: BTreeMap::new(),
            witnesses: BTreeMap::new(),
            literals: Vec::new(),
            sequences: Vec::new(),
            variants: Vec::new(),
            products: Vec::new(),
            next_context: 0,
            ended: false,
        }
    }

    fn finish(self) -> Result<ConstructionProjection, ProjectionError> {
        let construction = self
            .construction
            .ok_or_else(|| self.invalid("missing begin_form"))?;
        let form = self
            .form
            .ok_or_else(|| self.invalid("missing form identity"))?;
        let ordinal = self
            .ordinal
            .ok_or_else(|| self.invalid("missing form ordinal"))?;
        if !self.ended {
            return Err(self.invalid("missing end_form"));
        }
        if !self.sequences.is_empty() {
            return Err(self.invalid("an emitted sequence was not closed"));
        }
        if !self.variants.is_empty() {
            return Err(self.invalid("an emitted sum variant was not closed"));
        }
        if !self.products.is_empty() {
            return Err(self.invalid("an emitted product was not closed"));
        }
        Ok(ConstructionProjection {
            category: self.category,
            construction,
            form,
            ordinal,
            roles: self.roles,
            witnesses: self.witnesses,
            literals: self.literals,
        })
    }

    fn invalid(&self, message: impl Into<String>) -> ProjectionError {
        ProjectionError::Invalid {
            construction: self.expected_construction,
            message: message.into(),
        }
    }

    fn current_roles(
        &mut self,
        role: &'static str,
    ) -> Result<(&mut BTreeMap<&'static str, ProjectedValue>, &'static str), String> {
        let sequence_opened_at = self.sequences.last().map(|sequence| sequence.opened_at);
        let variant_opened_at = self.variants.last().map(|variant| variant.opened_at);
        let product_opened_at = self.products.last().map(|product| product.opened_at);
        if product_opened_at > sequence_opened_at && product_opened_at > variant_opened_at {
            let product = self
                .products
                .last_mut()
                .expect("a product context was observed");
            return Ok((&mut product.roles, role));
        }
        if variant_opened_at > sequence_opened_at && variant_opened_at > product_opened_at {
            let variant = self
                .variants
                .last_mut()
                .expect("a variant context was observed");
            return Ok((&mut variant.roles, role));
        }
        if sequence_opened_at.is_some() {
            let sequence = self
                .sequences
                .last_mut()
                .expect("a sequence context was observed");
            return sequence
                .members
                .last_mut()
                .map(|member| (&mut member.roles, role))
                .ok_or_else(|| {
                    format!(
                        "sequence `{}` emitted a value before a member",
                        sequence.role
                    )
                });
        }
        Ok((&mut self.roles, role))
    }

    fn insert(&mut self, role: &'static str, value: ProjectedValue) -> Result<(), String> {
        let (roles, role) = self.current_roles(role)?;
        match roles.get_mut(role) {
            Some(ProjectedValue::Optional(slot @ None)) => {
                *slot = Some(Box::new(value));
                Ok(())
            }
            Some(existing) if *existing == value => Ok(()),
            Some(_) => Err(format!(
                "role `{role}` was emitted more than once with different values"
            )),
            None => {
                roles.insert(role, value);
                Ok(())
            }
        }
    }
}

impl ProjectionSink for ProjectionCollector {
    fn begin_form(
        &mut self,
        construction: &'static str,
        form: &'static str,
        ordinal: u16,
    ) -> Result<(), String> {
        if construction != self.expected_construction {
            return Err(format!(
                "entry point `{}` began construction `{construction}`",
                self.expected_construction
            ));
        }
        if self.construction.replace(construction).is_some() {
            return Err("begin_form was emitted more than once".to_owned());
        }
        self.form = Some(form);
        self.ordinal = Some(ordinal);
        Ok(())
    }

    fn end_form(&mut self, construction: &'static str) -> Result<(), String> {
        if self.construction != Some(construction) {
            return Err(format!(
                "end_form named unexpected construction `{construction}`"
            ));
        }
        self.ended = true;
        Ok(())
    }

    fn literal(&mut self, literal: &'static str) -> Result<(), String> {
        self.literals.push(literal);
        Ok(())
    }

    fn subtree(
        &mut self,
        role: &'static str,
        category: &'static str,
        value: &dyn deckmaste_construction_compiler::runtime::ProjectionValue,
    ) -> Result<(), String> {
        self.insert(role, projected_subtree(category, value)?)
    }

    fn begin_sum_variant(
        &mut self,
        role: &'static str,
        element: &'static str,
        variant: &'static str,
    ) -> Result<(), String> {
        let opened_at = self.next_context;
        self.next_context += 1;
        self.variants.push(OpenVariant {
            role,
            element,
            variant,
            roles: BTreeMap::new(),
            opened_at,
        });
        Ok(())
    }

    fn end_sum_variant(&mut self, role: &'static str) -> Result<(), String> {
        let variant = self
            .variants
            .pop()
            .ok_or_else(|| format!("end_sum_variant `{role}` has no open variant"))?;
        if variant.role != role {
            return Err(format!(
                "end_sum_variant `{role}` closed `{}`",
                variant.role
            ));
        }
        self.insert(
            role,
            ProjectedValue::Variant(ProjectedVariant {
                element: variant.element,
                variant: variant.variant,
                roles: variant.roles,
            }),
        )
    }

    fn begin_product(&mut self, role: &'static str, element: &'static str) -> Result<(), String> {
        let opened_at = self.next_context;
        self.next_context += 1;
        self.products.push(OpenProduct {
            role,
            element,
            roles: BTreeMap::new(),
            opened_at,
        });
        Ok(())
    }

    fn end_product(&mut self, role: &'static str) -> Result<(), String> {
        let product = self
            .products
            .pop()
            .ok_or_else(|| format!("end_product `{role}` has no open product"))?;
        if product.role != role {
            return Err(format!("end_product `{role}` closed `{}`", product.role));
        }
        self.insert(
            role,
            ProjectedValue::Product(ProjectedProduct {
                element: product.element,
                roles: product.roles,
            }),
        )
    }

    fn scalar(
        &mut self,
        role: &'static str,
        codec: &'static str,
        value: &dyn deckmaste_construction_compiler::runtime::ProjectionValue,
    ) -> Result<(), String> {
        if let Some(projection) = project_flat_scalar(codec, value) {
            return self.insert(role, ProjectedValue::Construction(Box::new(projection)));
        }
        self.insert(
            role,
            ProjectedValue::Atom(ProjectedAtom::Scalar {
                codec,
                value: OwnedProjectionValue::new(value),
            }),
        )
    }

    fn identity(
        &mut self,
        role: &'static str,
        provider: &'static str,
        value_type: &'static str,
        value: &dyn deckmaste_construction_compiler::runtime::ProjectionValue,
    ) -> Result<(), String> {
        if let Some(projection) = project_flat_identity(provider, value) {
            return self.insert(role, ProjectedValue::Construction(Box::new(projection)));
        }
        self.insert(
            role,
            ProjectedValue::Atom(ProjectedAtom::Identity {
                provider,
                value_type,
                value: OwnedProjectionValue::new(value),
            }),
        )
    }

    fn begin_sequence(&mut self, role: &'static str, len: usize) -> Result<(), String> {
        let opened_at = self.next_context;
        self.next_context += 1;
        self.sequences.push(OpenSequence {
            role,
            expected_len: len,
            members: Vec::with_capacity(len),
            opened_at,
        });
        Ok(())
    }

    fn sequence_member(&mut self, role: &'static str, index: usize) -> Result<(), String> {
        let sequence = self
            .sequences
            .last_mut()
            .ok_or_else(|| format!("sequence member `{role}` has no open sequence"))?;
        if sequence.role != role || sequence.members.len() != index {
            return Err(format!(
                "sequence `{role}` emitted member {index} after {} member(s)",
                sequence.members.len()
            ));
        }
        sequence.members.push(ProjectedMember {
            element: "",
            variant: None,
            roles: BTreeMap::new(),
        });
        Ok(())
    }

    fn bound_value(
        &mut self,
        element: &'static str,
        _value: &dyn ProjectionInput,
    ) -> Result<(), String> {
        let sequence = self
            .sequences
            .last_mut()
            .ok_or_else(|| format!("element `{element}` has no open sequence"))?;
        let member = sequence
            .members
            .last_mut()
            .ok_or_else(|| format!("element `{element}` has no current member"))?;
        member.element = element;
        Ok(())
    }

    fn element_variant(
        &mut self,
        element: &'static str,
        variant: &'static str,
    ) -> Result<(), String> {
        let sequence = self
            .sequences
            .last_mut()
            .ok_or_else(|| format!("variant `{element}::{variant}` has no open sequence"))?;
        let member = sequence
            .members
            .last_mut()
            .ok_or_else(|| format!("variant `{element}::{variant}` has no current member"))?;
        if member.element != element {
            return Err(format!(
                "variant `{element}::{variant}` followed element `{}`",
                member.element
            ));
        }
        member.variant = Some(variant);
        Ok(())
    }

    fn derived_sequence_scalar(
        &mut self,
        role: &'static str,
        codec: &'static str,
        index: usize,
        len: usize,
    ) -> Result<(), String> {
        self.insert(
            role,
            ProjectedValue::Atom(ProjectedAtom::DerivedSequenceScalar { codec, index, len }),
        )
    }

    fn end_sequence(&mut self, role: &'static str) -> Result<(), String> {
        let sequence = self
            .sequences
            .pop()
            .ok_or_else(|| format!("end_sequence `{role}` has no open sequence"))?;
        if sequence.role != role || sequence.members.len() != sequence.expected_len {
            return Err(format!(
                "sequence `{role}` expected {} member(s), got {}",
                sequence.expected_len,
                sequence.members.len()
            ));
        }
        if let Some(member) = sequence
            .members
            .iter()
            .find(|member| member.element.is_empty())
        {
            return Err(format!(
                "sequence `{role}` has an element without declaration identity: {member:?}"
            ));
        }
        self.insert(
            role,
            ProjectedValue::Sequence(ProjectedSequence {
                role,
                members: sequence.members,
            }),
        )
    }

    fn optional(&mut self, role: &'static str, _present: bool) -> Result<(), String> {
        self.insert(role, ProjectedValue::Optional(None))
    }

    fn stored_witness(
        &mut self,
        name: &'static str,
        path: &'static str,
        value: &dyn deckmaste_construction_compiler::runtime::ProjectionValue,
    ) -> Result<(), String> {
        let witness = ProjectedWitness {
            path,
            value: OwnedProjectionValue::new(value),
        };
        match self.witnesses.insert(name, witness.clone()) {
            None => Ok(()),
            Some(previous) if previous == witness => Ok(()),
            Some(_) => Err(format!(
                "surface witness `{name}` was emitted with two values"
            )),
        }
    }
}

#[cfg(test)]
mod tests {
    use deckmaste_construction_compiler::runtime::ConstructionProjection;
    use deckmaste_construction_compiler::runtime::ProjectedAtom;
    use deckmaste_construction_compiler::runtime::ProjectedProduct;
    use deckmaste_construction_compiler::runtime::ProjectedSequence;
    use deckmaste_construction_compiler::runtime::ProjectedValue;
    use deckmaste_construction_compiler::runtime::ProjectedVariant;

    use crate::CatalogKind;
    use crate::Catalogs;
    use crate::Fragment;
    use crate::FragmentKind;
    use crate::features::Contraction;
    use crate::features::GapState;
    use crate::features::Number;
    use crate::parse_fragment;
    use crate::syntax::AttachmentPosition;
    use crate::syntax::ClauseAttachmentKind;
    use crate::syntax::CopularComplement;
    use crate::syntax::DependentClause;
    use crate::syntax::IndependentClause;
    use crate::syntax::NominalComplement;
    use crate::syntax::NounPhraseKind;
    use crate::syntax::Predicate;
    use crate::syntax::PredicateAdjunct;
    use crate::syntax::PredicateExpression;
    use crate::syntax::RelativeBody;
    use crate::syntax::RelativeClause;
    use crate::syntax::SentenceBody;
    use crate::syntax::SubordinateBody;
    use crate::syntax::Subordinator;
    use crate::syntax::ThisCardForm;

    fn collect_constructions<'a>(
        projection: &'a ConstructionProjection,
        constructions: &mut Vec<&'a ConstructionProjection>,
    ) {
        constructions.push(projection);
        for role in projection.roles.values() {
            collect_value_constructions(role, constructions);
        }
    }

    fn collect_value_constructions<'a>(
        value: &'a ProjectedValue,
        constructions: &mut Vec<&'a ConstructionProjection>,
    ) {
        match value {
            ProjectedValue::Construction(projection) => {
                collect_constructions(projection, constructions);
            }
            ProjectedValue::Optional(Some(value)) => {
                collect_value_constructions(value, constructions);
            }
            ProjectedValue::Sequence(sequence) => {
                for member in &sequence.members {
                    for role in member.roles.values() {
                        collect_value_constructions(role, constructions);
                    }
                }
            }
            ProjectedValue::Variant(variant) => {
                for role in variant.roles.values() {
                    collect_value_constructions(role, constructions);
                }
            }
            ProjectedValue::Product(product) => {
                for role in product.roles.values() {
                    collect_value_constructions(role, constructions);
                }
            }
            ProjectedValue::Atom(_) | ProjectedValue::Optional(None) => {}
        }
    }

    fn required_named_construction<'a>(
        projection: &'a ConstructionProjection,
        construction: &str,
    ) -> &'a ConstructionProjection {
        let mut constructions = Vec::new();
        collect_constructions(projection, &mut constructions);
        constructions
            .into_iter()
            .find(|candidate| candidate.construction == construction)
            .unwrap_or_else(|| panic!("missing construction {construction:?}: {projection:#?}"))
    }

    fn assert_projection_identity(
        projection: &ConstructionProjection,
        category: &'static str,
        construction: &'static str,
        form: &'static str,
        ordinal: u16,
        roles: &[&'static str],
    ) {
        assert_eq!(
            (
                projection.category,
                projection.construction,
                projection.form,
                projection.ordinal,
            ),
            (category, construction, form, ordinal),
            "{projection:#?}"
        );
        assert_eq!(
            projection.roles.keys().copied().collect::<Vec<_>>(),
            roles,
            "{projection:#?}"
        );
    }

    fn assert_no_flat_subtrees(projection: &ConstructionProjection) {
        fn visit(value: &ProjectedValue) {
            match value {
                ProjectedValue::Atom(ProjectedAtom::FlatSubtree { category, .. }) => {
                    panic!("projection retained a layout-erased {category} subtree")
                }
                ProjectedValue::Construction(projection) => {
                    assert_no_flat_subtrees(projection);
                }
                ProjectedValue::Optional(Some(value)) => visit(value),
                ProjectedValue::Sequence(sequence) => {
                    for member in &sequence.members {
                        for value in member.roles.values() {
                            visit(value);
                        }
                    }
                }
                ProjectedValue::Variant(variant) => {
                    for value in variant.roles.values() {
                        visit(value);
                    }
                }
                ProjectedValue::Product(product) => {
                    for value in product.roles.values() {
                        visit(value);
                    }
                }
                ProjectedValue::Atom(_) | ProjectedValue::Optional(None) => {}
            }
        }

        for value in projection.roles.values() {
            visit(value);
        }
    }

    fn required_construction_role<'a>(
        projection: &'a ConstructionProjection,
        role: &str,
    ) -> &'a ConstructionProjection {
        let Some(ProjectedValue::Construction(value)) = projection.roles.get(role) else {
            panic!("expected required construction role {role:?}: {projection:#?}");
        };
        value
    }

    fn optional_construction_role<'a>(
        projection: &'a ConstructionProjection,
        role: &str,
    ) -> &'a ConstructionProjection {
        let Some(ProjectedValue::Optional(Some(value))) = projection.roles.get(role) else {
            panic!("expected present optional role {role:?}: {projection:#?}");
        };
        let ProjectedValue::Construction(value) = value.as_ref() else {
            panic!("expected optional construction role {role:?}: {projection:#?}");
        };
        value
    }

    fn required_sequence_role<'a>(
        projection: &'a ConstructionProjection,
        role: &str,
    ) -> &'a ProjectedSequence {
        let Some(ProjectedValue::Sequence(value)) = projection.roles.get(role) else {
            panic!("expected required sequence role {role:?}: {projection:#?}");
        };
        value
    }

    fn required_variant_role<'a>(
        projection: &'a ConstructionProjection,
        role: &str,
    ) -> &'a ProjectedVariant {
        let Some(ProjectedValue::Variant(value)) = projection.roles.get(role) else {
            panic!("expected required variant role {role:?}: {projection:#?}");
        };
        value
    }

    fn required_nested_variant<'a>(
        roles: &'a std::collections::BTreeMap<&'static str, ProjectedValue>,
        role: &str,
    ) -> &'a ProjectedVariant {
        let Some(ProjectedValue::Variant(value)) = roles.get(role) else {
            panic!("expected nested variant role {role:?}: {roles:#?}");
        };
        value
    }

    fn required_nested_product<'a>(
        roles: &'a std::collections::BTreeMap<&'static str, ProjectedValue>,
        role: &str,
    ) -> &'a ProjectedProduct {
        let Some(ProjectedValue::Product(value)) = roles.get(role) else {
            panic!("expected nested product role {role:?}: {roles:#?}");
        };
        value
    }

    fn clean_fragment(source: &str, catalogs: &Catalogs, kind: FragmentKind) -> Fragment {
        let report = parse_fragment(source, catalogs, kind, "", false);
        assert!(
            report.clean(),
            "projection fixture must parse cleanly: {source}"
        );
        report
            .into_fragment()
            .expect("a clean projection fixture has a semantic fragment")
    }

    fn keyword_ability_catalogs() -> Catalogs {
        Catalogs::default()
            .with_catalog(CatalogKind::CardType, ["Artifact", "Creature"])
            .with_catalog(
                CatalogKind::KeywordAbility,
                [
                    "Flying",
                    "First strike",
                    "Ward",
                    "Fabricate",
                    "Suspend",
                    "Protection",
                    "Enchant",
                    "Prototype",
                    "Partner",
                    "Craft",
                    "Exhaust",
                ],
            )
    }

    fn keyword_ability_projection(source: &str, catalogs: &Catalogs) -> ConstructionProjection {
        let report = parse_fragment(
            source,
            catalogs,
            FragmentKind::KeywordLine,
            "Test Card",
            false,
        );
        let fragment = report
            .fragment()
            .unwrap_or_else(|| panic!("keyword fixture did not produce a fragment: {source}"));
        super::project_fragment(fragment)
            .unwrap_or_else(|error| panic!("keyword fixture did not project: {source}: {error}"))
    }

    fn independent_sentence(fragment: &Fragment) -> &IndependentClause {
        let Fragment::Sentence(sentence) = fragment else {
            panic!("expected a sentence fragment");
        };
        let SentenceBody::Independent(clause) = sentence.body() else {
            panic!("expected an independent sentence body");
        };
        clause
    }

    fn nominal_relative(fragment: &Fragment) -> &RelativeClause {
        let Fragment::Nominal(noun_phrase) = fragment else {
            panic!("expected a nominal fragment");
        };
        let NounPhraseKind::Nominal(nominal) = noun_phrase.kind() else {
            panic!("expected a nominal noun phrase");
        };
        let [NominalComplement::Relative(relative)] = nominal.complements() else {
            panic!("expected exactly one relative complement");
        };
        relative
    }

    #[test]
    fn cost_projection_is_a_nonempty_sequence_of_typed_variants() {
        let catalogs =
            Catalogs::default().with_catalog(CatalogKind::CardType, ["Artifact", "Creature"]);
        for (source, form, expected_variants) in [
            ("{T}", "plain", &["Symbols"][..]),
            ("Sacrifice a creature", "plain", &["Clause"][..]),
            ("A creature", "plain", &["Noun"][..]),
            ("{T} or {W}", "plain", &["Alternative"][..]),
            ("Frobnicate a creature", "plain", &["Recovered"][..]),
            (
                "Boast — {2}{R}, Sacrifice an artifact",
                "header",
                &["Symbols", "Clause"][..],
            ),
        ] {
            let report = parse_fragment(source, &catalogs, FragmentKind::Cost, "Test Card", false);
            let fragment = report
                .fragment()
                .unwrap_or_else(|| panic!("cost fixture did not produce a fragment: {source}"));
            let projection = super::project_fragment(fragment)
                .unwrap_or_else(|error| panic!("cost fixture did not project: {source}: {error}"));
            assert_eq!(projection.construction, "cost", "{source}");
            assert_eq!(projection.form, form, "{source}");

            let components = required_sequence_role(&projection, "components");
            assert!(!components.members.is_empty(), "{source}");
            assert_eq!(components.role, "components", "{source}");
            assert_eq!(
                components
                    .members
                    .iter()
                    .map(|member| {
                        assert_eq!(member.element, "cost_component", "{source}");
                        member.variant.expect("cost components select a variant")
                    })
                    .collect::<Vec<_>>(),
                expected_variants,
                "{source}",
            );

            if form == "header" {
                assert!(
                    matches!(
                        projection.roles.get("flavor_header"),
                        Some(ProjectedValue::Optional(Some(_)))
                    ),
                    "{source}: {projection:#?}"
                );
            } else {
                assert!(
                    !projection.roles.contains_key("flavor_header"),
                    "the plain cost form has no header role: {source}: {projection:#?}",
                );
            }

            if source == "{T} or {W}" {
                let alternative = &components.members[0];
                for role in ["cost_component.left", "cost_component.right"] {
                    let branch = required_nested_variant(&alternative.roles, role);
                    assert_eq!(branch.element, "cost_component");
                    assert_eq!(branch.variant, "Symbols");
                    assert_eq!(
                        branch.roles.keys().copied().collect::<Vec<_>>(),
                        ["payload"],
                        "recursive sum payload roles are local to their variant",
                    );
                }
            }
            if source == "Sacrifice a creature" {
                let Some(ProjectedValue::Construction(clause)) =
                    components.members[0].roles.get("cost_component.payload")
                else {
                    panic!(
                        "a typed clause cost must expose its Clause projection: {projection:#?}"
                    );
                };
                assert_eq!(clause.category, "Clause");
            }
        }
    }

    #[test]
    fn keyword_projection_preserves_argument_variants_and_nested_typed_payloads() {
        let catalogs = keyword_ability_catalogs();
        for (source, expected_argument, expected_cost) in [
            ("Flying", "Absent", None),
            ("Fabricate 2", "Counted", None),
            ("Ward {2}", "Costed", Some("Symbols")),
            ("Suspend 4—{1}{U}", "CountedCost", None),
            ("Protection from red", "Predicated", None),
            ("Enchant creature", "Qualified", None),
            ("Prototype {2}{G}{G} — 3/3", "Statted", None),
            ("Partner—Friends forever", "Named", None),
            (
                "Craft with artifact {1}{U}",
                "RestrictedCost",
                Some("Symbols"),
            ),
            ("Exhaust — {2}{G}: Draw a card.", "Costed", Some("Sentence")),
            ("Ward—Sacrifice a creature.", "Costed", Some("Components")),
            ("Ward {3}. This ability costs {1} less.", "Recovered", None),
        ] {
            let projection = keyword_ability_projection(source, &catalogs);
            assert_eq!(projection.construction, "keyword_line", "{source}");
            let abilities = required_sequence_role(&projection, "abilities");
            assert!(!abilities.members.is_empty(), "{source}");
            let first = &abilities.members[0];
            assert_eq!(first.element, "keyword_ability", "{source}");
            assert!(
                first.roles.contains_key("keyword_ability.ability"),
                "{source}"
            );
            let argument = required_nested_variant(&first.roles, "keyword_ability.argument");
            assert_eq!(argument.element, "keyword_argument", "{source}");
            assert_eq!(argument.variant, expected_argument, "{source}");
            assert!(
                argument.roles.keys().all(|role| !role.contains('.')),
                "variant payload roles must be local: {source}: {argument:#?}",
            );
            if expected_argument == "Predicated" {
                let predicated = required_nested_product(&argument.roles, "payload");
                assert_eq!(predicated.element, "predicated_argument", "{source}");
                let Some(ProjectedValue::Sequence(qualities)) = predicated.roles.get("qualities")
                else {
                    panic!("predicated argument lost its typed quality sequence: {predicated:#?}");
                };
                assert!(!qualities.members.is_empty(), "{source}");
                assert_eq!(
                    qualities.members[0].element, "predicated_quality",
                    "{source}"
                );
                let quality = required_nested_variant(
                    &qualities.members[0].roles,
                    "predicated_quality.quality",
                );
                assert_eq!(quality.element, "phrase_argument", "{source}");
            }
            if expected_argument == "Qualified" {
                let phrase = required_nested_variant(&argument.roles, "payload");
                assert_eq!(phrase.element, "phrase_argument", "{source}");
                assert_eq!(phrase.variant, "NounPhrase", "{source}");
            }
            if let Some(expected_cost) = expected_cost {
                let role = if expected_argument == "RestrictedCost" { "cost" } else { "payload" };
                let cost = required_nested_variant(&argument.roles, role);
                assert_eq!(cost.element, "keyword_cost", "{source}");
                assert_eq!(cost.variant, expected_cost, "{source}");
                assert!(
                    cost.roles.keys().all(|role| !role.contains('.')),
                    "nested variant payload roles must be local: {source}: {cost:#?}",
                );
                if expected_cost == "Components" {
                    let Some(ProjectedValue::Construction(cost)) = cost.roles.get("cost") else {
                        panic!("structured keyword cost lost its nested Cost root: {cost:#?}");
                    };
                    assert_eq!(cost.construction, "cost", "{source}");
                    assert!(
                        !required_sequence_role(cost, "components")
                            .members
                            .is_empty()
                    );
                }
            }
        }
    }

    #[test]
    fn keyword_projection_preserves_separator_edges_and_trailing_form() {
        let catalogs = keyword_ability_catalogs();
        let separated = keyword_ability_projection("Flying; first strike", &catalogs);
        let abilities = required_sequence_role(&separated, "abilities");
        assert_eq!(abilities.members.len(), 2);
        assert!(
            !abilities.members[0]
                .roles
                .contains_key("abilities.separator")
        );
        assert!(matches!(
            abilities.members[1].roles.get("abilities.separator"),
            Some(ProjectedValue::Atom(ProjectedAtom::Scalar {
                codec: "KeywordListSeparator",
                ..
            }))
        ));

        let trailing =
            keyword_ability_projection("Ward—Sacrifice a creature. Draw a card.", &catalogs);
        assert_eq!(trailing.form, "trailing");
        assert!(matches!(
            trailing.roles.get("trailing"),
            Some(ProjectedValue::Optional(Some(_)))
        ));
    }

    #[test]
    fn ability_projection_uses_the_direct_kind_sum_for_every_frame() {
        let catalogs = Catalogs::default()
            .with_catalog(CatalogKind::CardType, ["Artifact", "Creature", "Land"])
            .with_catalog(
                CatalogKind::KeywordAbility,
                ["Flying", "First strike", "Station"],
            );
        for (source, index, form, variant) in [
            ("{T}: Draw a card.", 0, "activated", "Activated"),
            ("{1}{R}: Level 2", 0, "class_level", "ClassLevel"),
            ("I — Draw a card.", 0, "chapter", "Chapter"),
            (
                "20 | Search your library for a card.",
                0,
                "roll_row",
                "RollRow",
            ),
            ("LEVEL 1-3\n4/4", 0, "level_band", "LevelBand"),
            (
                "Station\n8+ | Flying",
                1,
                "station_threshold",
                "StationThreshold",
            ),
            (
                "Whenever you attack, draw a card.",
                0,
                "triggered",
                "Triggered",
            ),
            (
                "[−X]: Exile each nonland permanent.",
                0,
                "loyalty",
                "Loyalty",
            ),
            (
                "Choose one —\n• Draw a card.\n• Gain 1 life.",
                0,
                "modal",
                "Modal",
            ),
            ("Flying", 0, "keyword", "Keyword"),
            ("Draw a card.", 0, "paragraph", "Paragraph"),
        ] {
            let report = crate::parse_with_catalogs(source, &catalogs);
            let ability = report
                .ast()
                .abilities
                .get(index)
                .unwrap_or_else(|| panic!("missing ability {index} for {source:?}"));
            let projection = super::project_value("Ability", ability)
                .unwrap_or_else(|error| panic!("ability did not project: {source}: {error}"));
            assert_eq!(projection.construction, "ability", "{source}");
            assert_eq!(projection.form, form, "{source}");
            let kind = required_variant_role(&projection, "kind");
            assert_eq!(kind.element, "ability_kind", "{source}");
            assert_eq!(kind.variant, variant, "{source}");
            assert_eq!(
                kind.roles.keys().copied().collect::<Vec<_>>(),
                ["payload"],
                "the selected kind owns one local payload role: {source}",
            );
            if variant == "Keyword" {
                let Some(ProjectedValue::Construction(keyword_line)) = kind.roles.get("payload")
                else {
                    panic!("keyword kind lost its nested keyword-line root: {kind:#?}");
                };
                assert_eq!(keyword_line.construction, "keyword_line");
            }
        }
    }

    #[test]
    fn ability_projection_keeps_the_single_optional_header_role() {
        let catalogs = Catalogs::default().with_catalog(CatalogKind::KeywordAbility, ["Flying"]);
        let report = parse_fragment(
            "Flying",
            &catalogs,
            FragmentKind::Ability,
            "Test Card",
            false,
        );
        let Fragment::Ability(plain) = report
            .fragment()
            .expect("the keyword ability produces a fragment")
        else {
            panic!("expected an ability fragment");
        };
        let plain_projection = super::project_value("Ability", plain)
            .expect("the headerless ability projects through its construction");
        assert!(matches!(
            plain_projection.roles.get("header"),
            Some(ProjectedValue::Optional(None))
        ));

        let labeled = crate::ability::build_ability(
            Some(crate::syntax::AbilityHeader::Flavor(
                crate::syntax::FlavorHeader::new("Showcase", 1),
            )),
            plain.kind().clone(),
        )
        .expect("a flavor header occupies the semantic header slot");
        let labeled_projection = super::project_value("Ability", &labeled)
            .expect("the labeled ability projects through its construction");
        assert!(matches!(
            labeled_projection.roles.get("header"),
            Some(ProjectedValue::Optional(Some(value)))
                if matches!(value.as_ref(), ProjectedValue::Atom(ProjectedAtom::FlatSubtree {
                    category: "AbilityHeader",
                    ..
                }))
        ));
    }

    #[test]
    fn projection_uses_stable_construction_form_and_roles() {
        let catalogs = Catalogs::default().with_catalog(CatalogKind::CardType, ["Creature"]);
        let report = parse_fragment(
            "target creature",
            &catalogs,
            FragmentKind::Nominal,
            "",
            false,
        );
        assert!(report.clean(), "{:?}", report.diagnostics());
        let projection = super::project_fragment(report.fragment().expect("clean fragment"))
            .expect("generated noun phrase projects");
        assert_eq!(projection.category, "NounPhrase");
        assert_eq!(projection.construction, "noun_phrase_nominal");
        assert_eq!(projection.form, "only");
        assert!(projection.roles.contains_key("nominal"));
    }

    #[test]
    fn all_the_projects_as_one_composed_determiner_owner() {
        let determiner = crate::determiner::all_the();
        let projection = super::project_value("Determiner", &determiner)
            .expect("the composed determiner projects");
        assert_projection_identity(
            &projection,
            "Determiner",
            "determiner_all_the",
            "only",
            0,
            &[],
        );
        assert_eq!(projection.literals, ["all", "the"]);
        assert_no_flat_subtrees(&projection);
    }

    #[test]
    fn projection_accepts_an_erased_category_value_for_a_projected_owner() {
        let catalogs =
            Catalogs::default().with_catalog(CatalogKind::CardType, ["Creature", "Artifact"]);
        let report = parse_fragment(
            "a creature and artifacts",
            &catalogs,
            FragmentKind::Nominal,
            "",
            false,
        );
        assert!(report.clean(), "{:?}", report.diagnostics());
        let projection = super::project_fragment(report.fragment().expect("clean fragment"))
            .expect("the projected coordination owner accepts its NounPhrase category value");
        assert_eq!(projection.category, "NounPhrase");
        assert_eq!(projection.construction, "noun_phrase_coordination");
        assert_eq!(projection.form, "flat");
        assert!(projection.roles.contains_key("first"));
        assert!(projection.roles.contains_key("rest"));
    }

    #[test]
    fn atomic_outputs_project_through_generated_construction_roles() {
        let catalogs = Catalogs::default().with_catalog(CatalogKind::CardType, ["Creature"]);
        let report = parse_fragment(
            "up to two target creatures",
            &catalogs,
            FragmentKind::Nominal,
            "",
            false,
        );
        assert!(report.clean(), "{:?}", report.diagnostics());
        let projection = super::project_fragment(report.fragment().expect("clean fragment"))
            .expect("atomic families project");
        let mut constructions = Vec::new();
        collect_constructions(&projection, &mut constructions);

        for expected in ["quantity_up_to", "determiner_quantified_target", "noun"] {
            assert!(
                constructions
                    .iter()
                    .any(|construction| construction.construction == expected),
                "missing generated atomic construction {expected}: {projection:#?}",
            );
        }
        let noun = constructions
            .iter()
            .find(|construction| construction.construction == "noun")
            .expect("known noun uses its generated construction");
        assert!(noun.roles.contains_key("identity"));
        assert!(constructions.iter().all(|construction| {
            !matches!(
                construction.construction,
                "flat_noun_instance" | "flat_noun"
            )
        }));
    }

    #[test]
    fn direct_noun_phrase_special_forms_project_their_semantic_roles() {
        let catalogs = Catalogs::default();
        let possessive_report = parse_fragment(
            "Nissa's",
            &catalogs,
            FragmentKind::Nominal,
            "Nissa Revane",
            true,
        );
        assert!(
            possessive_report.clean(),
            "{:?}",
            possessive_report.diagnostics()
        );
        let possessive_fragment = possessive_report.fragment().expect("clean fragment");
        let Fragment::Nominal(possessive) = possessive_fragment else {
            panic!("nominal fragment changed category: {possessive_fragment:#?}");
        };
        assert!(matches!(
            possessive.kind(),
            NounPhraseKind::PossessiveThisCard(ThisCardForm::AbbreviatedName)
        ));
        let possessive_projection = super::project_fragment(possessive_fragment)
            .expect("direct possessive self-reference projects");
        assert_eq!(
            (
                possessive_projection.category,
                possessive_projection.construction,
                possessive_projection.form,
                possessive_projection.ordinal,
            ),
            ("NounPhrase", "noun_phrase_possessive_this_card", "only", 0,)
        );
        let Some(ProjectedValue::Atom(ProjectedAtom::Identity {
            provider,
            value_type,
            value,
        })) = possessive_projection.roles.get("form")
        else {
            panic!("possessive form lost its typed identity: {possessive_projection:#?}");
        };
        assert_eq!(
            (*provider, *value_type),
            ("PossessiveThisCard", "ThisCardForm")
        );
        assert_eq!(
            value.downcast_ref::<ThisCardForm>(),
            Some(&ThisCardForm::AbbreviatedName)
        );

        let any_number_report = parse_fragment(
            "any number of target players",
            &catalogs,
            FragmentKind::Nominal,
            "",
            false,
        );
        assert!(
            any_number_report.clean(),
            "{:?}",
            any_number_report.diagnostics()
        );
        let any_number_fragment = any_number_report.fragment().expect("clean fragment");
        let Fragment::Nominal(any_number) = any_number_fragment else {
            panic!("nominal fragment changed category: {any_number_fragment:#?}");
        };
        let NounPhraseKind::AnyNumberOf(value) = any_number.kind() else {
            panic!("any-number phrase reconstructed a nominal spine: {any_number:#?}");
        };
        assert_eq!(value.plurality(), Number::Plural);
        let any_number_projection = super::project_fragment(any_number_fragment)
            .expect("direct any-number phrase projects");
        assert_eq!(
            (
                any_number_projection.category,
                any_number_projection.construction,
                any_number_projection.form,
                any_number_projection.ordinal,
            ),
            ("NounPhrase", "noun_phrase_any_number_of", "only", 0,)
        );
        assert_eq!(any_number_projection.literals, ["any", "number", "of"]);
        assert_eq!(
            any_number_projection
                .roles
                .keys()
                .copied()
                .collect::<Vec<_>>(),
            ["whole"]
        );
        let whole = required_construction_role(&any_number_projection, "whole");
        assert_eq!(whole.category, "NounPhrase");
    }

    #[test]
    fn nested_prepositional_projection_preserves_each_typed_object_boundary() {
        let report = parse_fragment(
            "cards from among them",
            &Catalogs::default(),
            FragmentKind::Nominal,
            "",
            false,
        );
        assert!(report.clean(), "{:?}", report.diagnostics());
        let projection = super::project_fragment(report.fragment().expect("clean fragment"))
            .expect("nested typed prepositional objects project");
        let mut constructions = Vec::new();
        collect_constructions(&projection, &mut constructions);
        let objects = constructions
            .into_iter()
            .filter(|projection| projection.construction == "prepositional_object")
            .collect::<Vec<_>>();
        assert_eq!(objects.len(), 2, "{projection:#?}");

        let outer = objects
            .iter()
            .copied()
            .find(|projection| projection.form == "prepositional_phrase")
            .expect("outer object retains its nested-PP alternative");
        assert_eq!(outer.ordinal, 1);
        assert_eq!(
            outer.roles.keys().copied().collect::<Vec<_>>(),
            ["prepositional_phrase"]
        );
        let nested = optional_construction_role(outer, "prepositional_phrase");
        assert_eq!(
            (nested.category, nested.construction),
            ("PrepositionalPhrase", "prepositional_phrase")
        );

        let inner = objects
            .iter()
            .copied()
            .find(|projection| projection.form == "noun_phrase")
            .expect("inner object retains its noun-phrase alternative");
        assert_eq!(inner.ordinal, 0);
        assert_eq!(
            inner.roles.keys().copied().collect::<Vec<_>>(),
            ["noun_phrase"]
        );
        let noun_phrase = optional_construction_role(inner, "noun_phrase");
        assert_eq!(noun_phrase.category, "NounPhrase");
    }

    fn projected_verb_analysis(
        projection: &ConstructionProjection,
    ) -> &crate::predicate::VerbAnalysis {
        let predicate = required_construction_role(projection, "predicate");
        assert_eq!(predicate.construction, "verb_phrase_base");
        let verb = required_construction_role(predicate, "head");
        assert_eq!((verb.category, verb.construction), ("Verb", "verb"));
        let Some(ProjectedValue::Atom(ProjectedAtom::Identity {
            provider,
            value_type,
            value,
        })) = verb.roles.get("head")
        else {
            panic!("declared verb lost its typed lexical identity: {verb:#?}");
        };
        assert_eq!((*provider, *value_type), ("LexicalVerb", "VerbAnalysis"));
        value
            .downcast_ref::<crate::predicate::VerbAnalysis>()
            .expect("LexicalVerb projects its public sealed analysis")
    }

    #[test]
    fn predicate_projection_exposes_declared_roles_and_typed_lexical_identity() {
        let sentence = parse_fragment(
            "Draw a card.",
            &Catalogs::default(),
            FragmentKind::Sentence,
            "",
            false,
        )
        .into_fragment()
        .expect("predicate fixture parses");
        let projection = super::project_fragment(&sentence)
            .expect("predicate fixture projects through its construction");
        let mut constructions = Vec::new();
        collect_constructions(&projection, &mut constructions);
        let direct_object = constructions
            .iter()
            .copied()
            .find(|candidate| candidate.construction == "verb_phrase_direct_object")
            .expect("the selected direct-object construction remains visible");
        assert_eq!(
            direct_object.roles.keys().copied().collect::<Vec<_>>(),
            ["object", "predicate"]
        );
        assert_eq!(
            required_construction_role(direct_object, "object").category,
            "NounPhrase"
        );
        let analysis = projected_verb_analysis(direct_object);
        assert_eq!(
            analysis.instance(),
            &crate::word::VerbInstance {
                verb: crate::word::Verb::Word(crate::word::Vocab::Draw),
                slot: crate::features::VerbSlot::Imperative,
            }
        );
        assert_eq!(analysis.citation_form().instance(), analysis.instance());
        assert!(constructions.iter().all(|candidate| !matches!(
            candidate.construction,
            "flat_verb_analysis" | "flat_verb_slot"
        )));
    }

    #[test]
    fn counted_energy_projection_keeps_typed_nested_product_roles() {
        let fragment = parse_fragment(
            "Pay six {E}.",
            &Catalogs::default(),
            FragmentKind::Sentence,
            "",
            false,
        )
        .into_fragment()
        .expect("counted energy fixture parses");
        let projection = super::project_fragment(&fragment).expect("counted energy projects");
        assert_no_flat_subtrees(&projection);
        let mut constructions = Vec::new();
        collect_constructions(&projection, &mut constructions);
        let predicate = constructions
            .into_iter()
            .find(|value| value.construction == "verb_phrase_counted_energy")
            .expect("counted energy has its generated predicate owner");
        let energy = required_construction_role(predicate, "energy");
        assert_eq!(
            (energy.category, energy.construction),
            ("CountedEnergy", "counted_energy")
        );
        assert!(matches!(
            energy.roles.get("quantity"),
            Some(ProjectedValue::Construction(_))
        ));
        assert!(matches!(
            energy.roles.get("symbol"),
            Some(ProjectedValue::Atom(_))
        ));
    }

    #[test]
    fn pronominal_resultative_projection_keeps_each_declared_continuation_role() {
        let fragment = parse_fragment(
            "Return it to the battlefield transformed under your control.",
            &Catalogs::default(),
            FragmentKind::Sentence,
            "",
            false,
        )
        .into_fragment()
        .expect("pronominal resultative fixture parses");
        let projection = super::project_fragment(&fragment).expect("resultative fixture projects");
        assert_no_flat_subtrees(&projection);
        let mut constructions = Vec::new();
        collect_constructions(&projection, &mut constructions);
        let resultative = constructions
            .into_iter()
            .find(|value| value.construction == "verb_phrase_pronominal_resultative_prepositional")
            .expect("the generated resultative owner remains visible");
        assert_eq!(
            resultative.roles.keys().copied().collect::<Vec<_>>(),
            ["adjective", "predicate", "preposition"]
        );
        assert_eq!(
            required_construction_role(resultative, "predicate").category,
            "VerbPhrase"
        );
        assert_eq!(
            required_construction_role(resultative, "adjective").category,
            "AdjectivePhrase"
        );
        assert_eq!(
            required_construction_role(resultative, "preposition").category,
            "PrepositionalPhrase"
        );
    }

    #[test]
    fn nonfinite_projection_exposes_infinitive_and_recursive_gerund_roles() {
        let predicate = |verb, slot| {
            crate::predicate::build_predicate_verb(
                crate::word::VerbInstance {
                    verb: crate::word::Verb::Word(verb),
                    slot,
                },
                crate::predicate::PredicateFrameChoice::Intransitive,
            )
            .and_then(crate::predicate::finish_predicate)
            .expect("the nonfinite-clause fixture predicate builds")
        };
        let infinitive_predicate = predicate(
            crate::word::Vocab::Attack,
            crate::features::VerbSlot::Infinitive,
        );
        let infinitive = crate::clause::build_infinitive_to(&infinitive_predicate)
            .expect("positive infinitive fixture builds");
        let negated = crate::clause::build_infinitive_not_to(&infinitive_predicate)
            .expect("negated infinitive fixture builds");
        for (value, construction, literals) in [
            (&infinitive, "infinitive_to", &["to"][..]),
            (&negated, "infinitive_not_to", &["not", "to"][..]),
        ] {
            let projection = super::project_value("InfinitiveClause", value)
                .expect("the declared infinitive projects");
            assert_eq!(
                (
                    projection.category,
                    projection.construction,
                    projection.form,
                    projection.ordinal,
                ),
                ("InfinitiveClause", construction, "only", 0)
            );
            assert_eq!(projection.literals, literals);
            assert_eq!(
                projected_verb_analysis(&projection).instance().verb,
                crate::word::Verb::Word(crate::word::Vocab::Attack)
            );
        }

        let predicate = crate::predicate::build_predicate_verb(
            crate::word::VerbInstance {
                verb: crate::word::Verb::Word(crate::word::Vocab::Attack),
                slot: crate::features::VerbSlot::PresentParticiple,
            },
            crate::predicate::PredicateFrameChoice::Intransitive,
        )
        .and_then(crate::predicate::finish_predicate)
        .expect("gerund fixture predicate builds");
        let alternative = crate::predicate::build_predicate_verb(
            crate::word::VerbInstance {
                verb: crate::word::Verb::Word(crate::word::Vocab::Block),
                slot: crate::features::VerbSlot::PresentParticiple,
            },
            crate::predicate::PredicateFrameChoice::Intransitive,
        )
        .and_then(crate::predicate::finish_predicate)
        .expect("alternative gerund fixture predicate builds");
        let matrix = crate::clause::build_gerund_clause_base(&predicate)
            .expect("matrix gerund fixture builds");
        let alternative = crate::clause::build_gerund_clause_base(&alternative)
            .expect("alternative gerund fixture builds");
        let relation = crate::clause::build_gerund_clause_subordinate_after(matrix, alternative)
            .expect("rather-than fixture builds");
        let relation =
            super::project_value("GerundClause", &relation).expect("rather-than fixture projects");
        assert_eq!(
            (
                relation.category,
                relation.construction,
                relation.form,
                relation.ordinal,
            ),
            ("GerundClause", "gerund_clause_subordinate_after", "only", 0,)
        );
        assert_eq!(relation.literals, ["rather", "than"]);
        assert_eq!(
            relation.roles.keys().copied().collect::<Vec<_>>(),
            ["alternative", "matrix"]
        );
        let matrix = required_construction_role(&relation, "matrix");
        let alternative = required_construction_role(&relation, "alternative");
        assert_eq!(matrix.construction, "gerund_clause_base");
        assert_eq!(alternative.construction, "gerund_clause_base");
        assert_eq!(
            projected_verb_analysis(matrix).instance().verb,
            crate::word::Verb::Word(crate::word::Vocab::Attack)
        );
        assert_eq!(
            projected_verb_analysis(alternative).instance().verb,
            crate::word::Verb::Word(crate::word::Vocab::Block)
        );
    }

    #[test]
    fn finite_projection_uses_canonical_subject_and_predicate_roles() {
        let explicit = clean_fragment(
            "You may draw a card.",
            &Catalogs::default(),
            FragmentKind::Sentence,
        );
        let IndependentClause::Finite(finite) = independent_sentence(&explicit) else {
            panic!("ordinary finite clauses must use the canonical finite owner");
        };
        assert!(finite.subject().is_some());
        let PredicateExpression::Simple(Predicate::Deontic(deontic)) = finite.predicate() else {
            panic!("the modal must live in the predicate expression");
        };
        assert!(matches!(
            deontic.inner(),
            Some(PredicateExpression::Simple(Predicate::Transitive(_)))
        ));

        let projection = super::project_fragment(&explicit).expect("the finite sentence projects");
        let clause = required_construction_role(&projection, "clause");
        assert_eq!(
            (clause.category, clause.construction),
            ("Clause", "clause_simple")
        );
        let simple = required_construction_role(clause, "simple");
        assert_eq!(simple.construction, "simple_clause_subject");
        assert_eq!(
            simple.roles.keys().copied().collect::<Vec<_>>(),
            ["predicate", "subject"]
        );
        assert_eq!(
            required_construction_role(simple, "subject").category,
            "NounPhrase"
        );
        assert_eq!(
            required_construction_role(simple, "predicate").category,
            "VerbPhrase"
        );

        let imperative =
            clean_fragment("Draw a card.", &Catalogs::default(), FragmentKind::Sentence);
        let IndependentClause::Finite(finite) = independent_sentence(&imperative) else {
            panic!("imperatives must use the canonical finite owner");
        };
        assert!(finite.subject().is_none());
        assert!(matches!(
            finite.predicate(),
            PredicateExpression::Simple(Predicate::Transitive(_))
        ));

        let projection =
            super::project_fragment(&imperative).expect("the imperative sentence projects");
        let clause = required_construction_role(&projection, "clause");
        let simple = required_construction_role(clause, "simple");
        assert_eq!(simple.construction, "simple_clause_subjectless");
        assert_eq!(
            simple.roles.keys().copied().collect::<Vec<_>>(),
            ["predicate"]
        );
    }

    #[test]
    fn finite_projection_keeps_existential_and_copular_construction_owners() {
        let catalogs = Catalogs::default().with_catalog(CatalogKind::CardType, ["Creature"]);

        let existential = clean_fragment("There are creatures.", &catalogs, FragmentKind::Sentence);
        let IndependentClause::Existential(existential_clause) = independent_sentence(&existential)
        else {
            panic!("existential syntax must keep its specialized semantic owner");
        };
        assert!(matches!(
            existential_clause.pivot.kind(),
            NounPhraseKind::Nominal(_)
        ));

        let projection =
            super::project_fragment(&existential).expect("the existential sentence projects");
        let clause = required_construction_role(&projection, "clause");
        assert_eq!(
            (
                clause.category,
                clause.construction,
                clause.form,
                clause.ordinal,
            ),
            ("Clause", "clause_existential", "only", 0)
        );
        assert!(clause.literals.is_empty());
        assert_eq!(
            clause.roles.keys().copied().collect::<Vec<_>>(),
            ["existential", "pivot"]
        );
        let Some(ProjectedValue::Atom(ProjectedAtom::Identity {
            provider,
            value_type,
            ..
        })) = clause.roles.get("existential")
        else {
            panic!("existential form must retain its typed identity");
        };
        assert_eq!((*provider, *value_type), ("Existential", "ExistentialForm"));
        assert_eq!(
            required_construction_role(clause, "pivot").category,
            "NounPhrase"
        );

        for (source, construction, contraction, roles) in [
            (
                "This creature is red.",
                "clause_copular",
                Contraction::Full,
                &["copula", "remainder", "subject"][..],
            ),
            (
                "It's red.",
                "clause_contracted_copular",
                Contraction::Contracted,
                &["remainder", "subject_auxiliary"][..],
            ),
        ] {
            let fragment = clean_fragment(source, &catalogs, FragmentKind::Sentence);
            let IndependentClause::Finite(finite) = independent_sentence(&fragment) else {
                panic!("copular syntax must use the canonical finite owner: {source}");
            };
            assert!(finite.subject().is_some());
            let PredicateExpression::Simple(Predicate::Copular(copular)) = finite.predicate()
            else {
                panic!("copular syntax must live in the finite predicate: {source}");
            };
            assert_eq!(copular.copula().contracted_with_subject(), contraction);
            assert!(matches!(
                copular.complement(),
                CopularComplement::Adjective(_)
            ));

            let projection =
                super::project_fragment(&fragment).expect("the copular sentence projects");
            let clause = required_construction_role(&projection, "clause");
            assert_eq!(
                (
                    clause.category,
                    clause.construction,
                    clause.form,
                    clause.ordinal,
                ),
                ("Clause", construction, "only", 0),
                "{source}"
            );
            assert!(clause.literals.is_empty(), "{source}");
            assert_eq!(
                clause.roles.keys().copied().collect::<Vec<_>>(),
                roles,
                "{source}"
            );
            let remainder = required_construction_role(clause, "remainder");
            assert_eq!(
                (remainder.category, remainder.construction),
                ("CopularRemainder", "copular_remainder_adjective"),
                "{source}"
            );
            assert_eq!(
                remainder.roles.keys().copied().collect::<Vec<_>>(),
                ["complement"],
                "{source}"
            );
        }
    }

    #[test]
    fn attachment_projection_preserves_recursive_one_edge_scope() {
        let catalogs =
            Catalogs::default().with_catalog(CatalogKind::CardType, ["Creature", "Spell"]);
        let fragment = clean_fragment(
            "At the beginning of each upkeep, if no spells were cast last turn, transform this creature.",
            &catalogs,
            FragmentKind::Sentence,
        );
        let IndependentClause::Complex(outer) = independent_sentence(&fragment) else {
            panic!("the first fronted constituent must own the outer scope edge");
        };
        assert_eq!(
            outer.attachment().position(),
            AttachmentPosition::BeforeMatrix
        );
        assert!(outer.attachment().comma().is_present());
        assert!(matches!(
            outer.attachment().payload(),
            ClauseAttachmentKind::Adjunct(PredicateAdjunct::Prepositional(_))
        ));

        let IndependentClause::Complex(inner) = outer.host() else {
            panic!("the outer edge must recursively wrap the next scoped edge");
        };
        assert_eq!(
            inner.attachment().position(),
            AttachmentPosition::BeforeMatrix
        );
        assert!(inner.attachment().comma().is_present());
        assert!(matches!(
            inner.attachment().payload(),
            ClauseAttachmentKind::Dependent(DependentClause::Subordinate(
                Subordinator::If,
                SubordinateBody::Finite(condition),
            )) if matches!(condition.as_ref(), IndependentClause::Finite(_))
        ));
        assert!(matches!(inner.host(), IndependentClause::Finite(_)));

        let projection =
            super::project_fragment(&fragment).expect("the recursive attachment sentence projects");
        let outer = required_construction_role(&projection, "clause");
        assert_eq!(outer.construction, "clause_prepositional_before");
        assert_eq!(
            outer.roles.keys().copied().collect::<Vec<_>>(),
            ["host", "preposition"]
        );
        let inner = required_construction_role(outer, "host");
        assert_eq!(inner.construction, "clause_subordinate_before");
        assert_eq!(
            inner.roles.keys().copied().collect::<Vec<_>>(),
            ["condition", "host", "subordinator"]
        );
        assert_eq!(
            required_construction_role(inner, "host").construction,
            "clause_simple"
        );
    }

    #[test]
    fn attachment_projection_distinguishes_trailing_subordinate_comma_forms() {
        let catalogs =
            Catalogs::default().with_catalog(CatalogKind::CardType, ["Creature", "Spell"]);
        for (source, construction, comma, subordinator, literals) in [
            (
                "You may cast this spell as though it had flash.",
                "clause_subordinate_after",
                crate::features::Comma::Absent,
                Subordinator::AsThough,
                &[][..],
            ),
            (
                "Target creature gets +X/+0 until end of turn, where X is the number of creatures you control.",
                "clause_subordinate_after_comma",
                crate::features::Comma::Present,
                Subordinator::Where,
                &[","][..],
            ),
        ] {
            let fragment = clean_fragment(source, &catalogs, FragmentKind::Sentence);
            let IndependentClause::Complex(complex) = independent_sentence(&fragment) else {
                panic!("the trailing subordinate must be one recursive scope edge: {source}");
            };
            assert_eq!(
                complex.attachment().position(),
                AttachmentPosition::AfterMatrix
            );
            assert_eq!(complex.attachment().comma(), comma);
            assert!(matches!(complex.host(), IndependentClause::Finite(_)));
            assert!(matches!(
                complex.attachment().payload(),
                ClauseAttachmentKind::Dependent(DependentClause::Subordinate(
                    actual,
                    SubordinateBody::Finite(condition),
                )) if *actual == subordinator
                    && matches!(condition.as_ref(), IndependentClause::Finite(_))
            ));

            let projection =
                super::project_fragment(&fragment).expect("the trailing subordinate projects");
            let clause = required_construction_role(&projection, "clause");
            assert_eq!(
                (
                    clause.category,
                    clause.construction,
                    clause.form,
                    clause.ordinal,
                ),
                ("Clause", construction, "only", 0),
                "{source}"
            );
            assert_eq!(clause.literals, literals, "{source}");
            assert_eq!(
                clause.roles.keys().copied().collect::<Vec<_>>(),
                ["condition", "host", "subordinator"],
                "{source}"
            );
            assert_eq!(
                required_construction_role(clause, "host").category,
                "Clause",
                "{source}"
            );
            assert_eq!(
                required_construction_role(clause, "condition").category,
                "Clause",
                "{source}"
            );
            let Some(ProjectedValue::Atom(ProjectedAtom::Identity {
                provider,
                value_type,
                value,
            })) = clause.roles.get("subordinator")
            else {
                panic!("the subordinator must retain its typed identity: {source}");
            };
            assert_eq!((*provider, *value_type), ("Subordinator", "Subordinator"));
            assert_eq!(value.downcast_ref::<Subordinator>(), Some(&subordinator));
        }
    }

    #[test]
    fn attachment_projection_preserves_restriction_sequence_roles_and_member_forms() {
        let fragment = clean_fragment(
            "Activate only as a sorcery and only once each turn.",
            &Catalogs::default(),
            FragmentKind::Sentence,
        );
        let IndependentClause::Complex(complex) = independent_sentence(&fragment) else {
            panic!("the restriction run must be one trailing recursive scope edge");
        };
        assert_eq!(
            complex.attachment().position(),
            AttachmentPosition::AfterMatrix
        );
        assert!(!complex.attachment().comma().is_present());
        let ClauseAttachmentKind::Restriction(run) = complex.attachment().payload() else {
            panic!("the attachment must retain its typed restriction payload");
        };
        assert!(matches!(
            run.first().adjuncts(),
            [PredicateAdjunct::Prepositional(_)]
        ));
        let [continuation] = run.rest() else {
            panic!("the two-member restriction run must have one continuation");
        };
        assert_eq!(
            continuation.conjunction(),
            Some(crate::features::Conjunction::And)
        );
        assert!(matches!(
            continuation.member().adjuncts(),
            [PredicateAdjunct::Adverb(_), PredicateAdjunct::Temporal(_)]
        ));

        let projection =
            super::project_fragment(&fragment).expect("the restriction sentence projects");
        let clause = required_construction_role(&projection, "clause");
        assert_eq!(
            (
                clause.category,
                clause.construction,
                clause.form,
                clause.ordinal,
            ),
            ("Clause", "clause_restriction_run", "only", 0)
        );
        assert!(clause.literals.is_empty());
        assert_eq!(
            clause.roles.keys().copied().collect::<Vec<_>>(),
            ["first", "host", "rest"]
        );

        let first = required_construction_role(clause, "first");
        assert_eq!(
            (
                first.category,
                first.construction,
                first.form,
                first.ordinal,
            ),
            (
                "RestrictionMember",
                "clause_restriction_member",
                "preposition",
                0,
            )
        );
        assert_eq!(first.literals, ["only"]);
        assert_eq!(
            first.roles.keys().copied().collect::<Vec<_>>(),
            ["preposition"]
        );
        assert!(matches!(
            first.roles.get("preposition"),
            Some(ProjectedValue::Optional(Some(_)))
        ));

        let Some(ProjectedValue::Sequence(rest)) = clause.roles.get("rest") else {
            panic!("restriction continuations must project as a declared sequence");
        };
        assert_eq!(rest.role, "rest");
        let [member] = rest.members.as_slice() else {
            panic!("the two-member restriction run must project one continuation");
        };
        assert_eq!(
            (member.element, member.variant),
            ("restriction_run_member", None)
        );
        assert_eq!(
            member.roles.keys().copied().collect::<Vec<_>>(),
            [
                "restriction_run_member.comma",
                "restriction_run_member.conjunction",
                "restriction_run_member.member",
            ]
        );
        assert!(matches!(
            member.roles.get("restriction_run_member.comma"),
            Some(ProjectedValue::Atom(ProjectedAtom::DerivedSequenceScalar {
                codec: "Comma",
                index: 0,
                len: 1,
            }))
        ));
        let Some(ProjectedValue::Optional(Some(conjunction))) =
            member.roles.get("restriction_run_member.conjunction")
        else {
            panic!("the final restriction member must retain its conjunction");
        };
        assert!(matches!(
            conjunction.as_ref(),
            ProjectedValue::Atom(ProjectedAtom::Scalar { codec: "Conjunction", value })
                if value.downcast_ref::<crate::features::Conjunction>()
                    == Some(&crate::features::Conjunction::And)
        ));
        let Some(ProjectedValue::Construction(member)) =
            member.roles.get("restriction_run_member.member")
        else {
            panic!("the continuation payload must project its restriction member");
        };
        assert_eq!(
            (
                member.category,
                member.construction,
                member.form,
                member.ordinal,
            ),
            (
                "RestrictionMember",
                "clause_restriction_member",
                "once_temporal",
                3,
            )
        );
        assert_eq!(member.literals, ["only", "once"]);
        assert!(matches!(
            member.roles.get("temporal"),
            Some(ProjectedValue::Optional(Some(_)))
        ));
    }

    #[test]
    fn attachment_projection_preserves_recursive_exception_list_forms() {
        let catalogs = Catalogs::default().with_catalog(CatalogKind::CardType, ["Creature"]);
        let fragment = clean_fragment(
            "You may have this creature enter as a copy of any creature on the battlefield, except it's red, it's green, it's blue, and it's white.",
            &catalogs,
            FragmentKind::Sentence,
        );
        let IndependentClause::Complex(complex) = independent_sentence(&fragment) else {
            panic!("the exception rider must be one trailing recursive scope edge");
        };
        assert_eq!(
            complex.attachment().position(),
            AttachmentPosition::AfterMatrix
        );
        assert!(complex.attachment().comma().is_present());
        let ClauseAttachmentKind::Exception(rider) = complex.attachment().payload() else {
            panic!("the attachment must retain its typed exception payload");
        };
        assert!(matches!(rider.first(), IndependentClause::Finite(_)));
        let [second, third, fourth] = rider.rest() else {
            panic!("the four-member exception rider must have three continuations");
        };
        assert_eq!(second.conjunction(), None);
        assert_eq!(third.conjunction(), None);
        assert_eq!(
            fourth.conjunction(),
            Some(crate::features::Conjunction::And)
        );
        assert!(
            rider
                .rest()
                .iter()
                .all(|member| matches!(member.clause(), IndependentClause::Finite(_)))
        );

        let projection =
            super::project_fragment(&fragment).expect("the exception sentence projects");
        let clause = required_construction_role(&projection, "clause");
        assert_eq!(
            (
                clause.category,
                clause.construction,
                clause.form,
                clause.ordinal,
            ),
            ("Clause", "clause_excepted", "complete", 0)
        );
        assert_eq!(clause.literals, [","]);
        assert_eq!(
            clause.roles.keys().copied().collect::<Vec<_>>(),
            ["host", "rider"]
        );

        let oxford = optional_construction_role(clause, "rider");
        assert_eq!(
            (
                oxford.category,
                oxford.construction,
                oxford.form,
                oxford.ordinal,
            ),
            ("ExceptionRider", "exception_rider_oxford", "only", 0)
        );
        assert_eq!(oxford.literals, [","]);
        assert_eq!(
            oxford.roles.keys().copied().collect::<Vec<_>>(),
            ["clause", "conjunction", "rider"]
        );
        assert_eq!(
            required_construction_role(oxford, "clause").category,
            "Clause"
        );

        let extend = required_construction_role(oxford, "rider");
        assert_eq!(
            (
                extend.category,
                extend.construction,
                extend.form,
                extend.ordinal,
            ),
            ("ExceptionRiderList", "exception_rider_comma", "extend", 1)
        );
        assert_eq!(extend.literals, [","]);
        assert_eq!(
            extend.roles.keys().copied().collect::<Vec<_>>(),
            ["clause", "list"]
        );

        let seed = optional_construction_role(extend, "list");
        assert_eq!(
            (seed.category, seed.construction, seed.form, seed.ordinal,),
            ("ExceptionRiderList", "exception_rider_comma", "seed", 0)
        );
        assert_eq!(seed.literals, [","]);
        assert_eq!(
            seed.roles.keys().copied().collect::<Vec<_>>(),
            ["clause", "rider"]
        );

        let single = optional_construction_role(seed, "rider");
        assert_eq!(
            (
                single.category,
                single.construction,
                single.form,
                single.ordinal,
            ),
            ("ExceptionRider", "exception_rider_single", "only", 0)
        );
        assert_eq!(single.literals, ["except"]);
        assert_eq!(single.roles.keys().copied().collect::<Vec<_>>(), ["clause"]);
    }

    #[test]
    fn phrase_coordination_projection_covers_modifier_and_power_toughness_owners() {
        use crate::coordination as coordination_api;
        use crate::features::Conjunction;
        use crate::syntax::NominalModifier;
        use crate::syntax::Polarity;
        use crate::syntax::ScalarSign;
        use crate::syntax::ScalarValue;
        use crate::word::Adjective;
        use crate::word::ColorWord;
        use crate::word::Noun;
        use crate::word::NounInstance;
        use crate::word::Vocab;

        let red = crate::adjective::build_adjective_phrase(Adjective::Color(ColorWord::Red))
            .expect("red is a declared adjective phrase");
        let green = crate::adjective::build_adjective_phrase(Adjective::Color(ColorWord::Green))
            .expect("green is a declared adjective phrase");
        let adjective = coordination_api::build_modifier_adjective(red)
            .expect("an adjective fills the modifier member sum");
        let negated = coordination_api::build_modifier_negated(NominalModifier::Adjective {
            polarity: Polarity::Negative,
            phrase: green,
        })
        .expect("a negative adjective fills the negated member alternative");
        let noun = coordination_api::build_modifier_noun(
            NounInstance::try_singular(Noun::Word(Vocab::Ability))
                .expect("ability is a singular count noun"),
        )
        .expect("a noun fills the modifier member sum");
        let binary = coordination_api::build_coordinated_modifier(
            adjective.clone(),
            Vec::new(),
            Conjunction::Or,
            noun.clone(),
        )
        .expect("the binary coordination is checked");
        let binary = super::project_value("CoordinatedModifier", &binary)
            .expect("the binary coordinated modifier projects");
        assert_projection_identity(
            &binary,
            "CoordinatedModifier",
            "coordinated_modifier_conjoined",
            "only",
            0,
            &["conjunction", "list", "modifier"],
        );
        assert_no_flat_subtrees(&binary);

        let coordinated = coordination_api::build_coordinated_modifier(
            adjective,
            vec![negated],
            Conjunction::And,
            noun,
        )
        .expect("the three-member Oxford coordination is checked");
        let projection = super::project_value("CoordinatedModifier", &coordinated)
            .expect("the coordinated modifier projects");

        for (category, construction, roles) in [
            (
                "CoordinatedModifier",
                "coordinated_modifier_oxford",
                &["conjunction", "list", "modifier"][..],
            ),
            (
                "ModifierList",
                "modifier_list_comma",
                &["list", "modifier"][..],
            ),
            ("ModifierList", "modifier_list_single", &["first"][..]),
            (
                "ModifierConjunct",
                "modifier_conjunct_adjective",
                &["adjective"][..],
            ),
            (
                "ModifierConjunct",
                "modifier_conjunct_negated",
                &["modifier"][..],
            ),
            ("ModifierConjunct", "modifier_conjunct_noun", &["noun"][..]),
        ] {
            assert_projection_identity(
                required_named_construction(&projection, construction),
                category,
                construction,
                "only",
                0,
                roles,
            );
        }
        assert_no_flat_subtrees(&projection);

        let card = crate::nominal::build_nominal_noun(
            NounInstance::try_singular(Noun::Word(Vocab::Card))
                .expect("card is a singular count noun"),
        )
        .expect("card is a declared nominal");
        let owned = coordination_api::build_nominal_coordinated_modifier(coordinated, card)
            .expect("the modifier can share the nominal head");
        let projection =
            super::project_value("NominalPhrase", &owned).expect("the nominal owner projects");
        assert_projection_identity(
            &projection,
            "NominalPhrase",
            "nominal_coordinated_modifier",
            "only",
            0,
            &["coordinated", "nominal"],
        );
        assert_no_flat_subtrees(&projection);

        let stats = crate::syntax::PowerToughness {
            power: crate::syntax::SignedScalar {
                sign: ScalarSign::None,
                value: ScalarValue::X,
            },
            toughness: crate::syntax::SignedScalar {
                sign: ScalarSign::None,
                value: ScalarValue::X,
            },
        };
        let toughness = crate::nominal::build_nominal_noun(
            NounInstance::try_mass(Noun::Word(Vocab::Toughness)).expect("toughness is a mass noun"),
        )
        .expect("toughness is a declared nominal");
        let toughness =
            coordination_api::build_nominal_power_toughness_complement(toughness, stats)
                .expect("a power/toughness value fills the characteristic complement");
        let projection = super::project_value("NominalPhrase", &toughness)
            .expect("the power/toughness owner projects");
        assert_projection_identity(
            &projection,
            "NominalPhrase",
            "nominal_power_toughness_complement",
            "only",
            0,
            &["nominal", "stats"],
        );
        assert_eq!(
            required_construction_role(&projection, "stats").construction,
            "flat_power_toughness"
        );
        assert_no_flat_subtrees(&projection);
    }

    #[test]
    fn phrase_coordination_projection_preserves_prepositional_topology() {
        use crate::features::Conjunction;
        use crate::syntax::Preposition;
        use crate::word::Noun;
        use crate::word::NounInstance;
        use crate::word::Vocab;

        fn simple(preposition: Preposition) -> crate::syntax::PrepositionalPhrase {
            let nominal = crate::nominal::build_nominal_noun(
                NounInstance::try_singular(Noun::Word(Vocab::Card))
                    .expect("card is a singular count noun"),
            )
            .expect("card is a declared nominal");
            let noun = crate::noun_phrase::build_noun_phrase_nominal(nominal)
                .expect("the nominal fills a noun phrase");
            let object = crate::prepositional_phrase::build_prepositional_object_noun_phrase(noun)
                .expect("the noun phrase fills a prepositional object");
            crate::prepositional_phrase::build_prepositional_phrase(preposition, object)
                .expect("the simple prepositional phrase is checked")
        }

        let first = simple(Preposition::From);
        let second = simple(Preposition::In);
        let pair = crate::prepositional_phrase::build_prepositional_phrase_coordination(
            first.clone(),
            vec![(Some(Conjunction::And), second.clone())],
        )
        .expect("a binary sibling coordination is checked");
        let pair = super::project_value("PrepositionalPhrase", &pair)
            .expect("the binary sibling coordination projects");
        assert_projection_identity(
            &pair,
            "PrepositionalPhrase",
            "prepositional_phrase_sibling_coordinated",
            "pair",
            0,
            &["conjunction", "first", "next"],
        );
        assert_no_flat_subtrees(&pair);

        let oxford = crate::prepositional_phrase::build_prepositional_phrase_coordination(
            first,
            vec![
                (None, second),
                (None, simple(Preposition::On)),
                (Some(Conjunction::Or), simple(Preposition::Under)),
            ],
        )
        .expect("a four-member sibling coordination is checked");
        let oxford = super::project_value("PrepositionalPhrase", &oxford)
            .expect("the Oxford sibling coordination projects");
        assert_projection_identity(
            &oxford,
            "PrepositionalPhrase",
            "prepositional_phrase_sibling_coordinated",
            "oxford",
            1,
            &["comma", "conjunction", "list", "next"],
        );
        assert_projection_identity(
            required_named_construction(&oxford, "prepositional_phrase_list_comma"),
            "PrepositionalPhraseList",
            "prepositional_phrase_list_comma",
            "only",
            0,
            &["list", "next"],
        );
        assert_projection_identity(
            required_named_construction(&oxford, "prepositional_phrase_list_pair"),
            "PrepositionalPhraseList",
            "prepositional_phrase_list_pair",
            "only",
            0,
            &["first", "second"],
        );
        assert_no_flat_subtrees(&oxford);
    }

    #[test]
    fn phrase_coordination_projection_exposes_predicative_adjective_members() {
        let green = crate::adjective::build_adjective_phrase(crate::word::Adjective::Color(
            crate::word::ColorWord::Green,
        ))
        .expect("green is a declared adjective phrase");
        let white = crate::adjective::build_adjective_phrase(crate::word::Adjective::Color(
            crate::word::ColorWord::White,
        ))
        .expect("white is a declared adjective phrase");
        let complement = crate::coordination::build_coordinated_adjective_phrase(
            green,
            Vec::new(),
            crate::features::Conjunction::AndOr,
            white,
        )
        .expect("the coordinated adjective carrier is checked");
        let head = crate::grammar::VerbAnalysis::new(
            crate::word::VerbInstance {
                verb: crate::word::Verb::Word(crate::word::Vocab::Be),
                slot: crate::features::VerbSlot::Infinitive,
            },
            crate::word::Vocab::Be.predicate_frames()[0],
        );
        let predicate = crate::constructions::predicate::build_verb_phrase_base(head)
            .expect("the declared be frame builds");
        let predicate =
            crate::constructions::coordination::build_verb_phrase_coordinated_adjective(
                predicate, complement,
            )
            .expect("the be frame admits the coordinated adjective complement");
        let predicate = super::project_value("VerbPhrase", &predicate)
            .expect("the adjective coordination projects");
        assert_projection_identity(
            &predicate,
            "VerbPhrase",
            "verb_phrase_coordinated_adjective",
            "only",
            0,
            &["complement", "predicate"],
        );
        let complement = required_construction_role(&predicate, "complement");
        assert_projection_identity(
            complement,
            "CoordinatedAdjectivePhrase",
            "coordinated_adjective_phrase",
            "only",
            0,
            &["first", "rest"],
        );
        let rest = required_sequence_role(complement, "rest");
        let [member] = rest.members.as_slice() else {
            panic!("a binary adjective coordination has one continuation")
        };
        assert_eq!(
            (member.element, member.variant),
            ("adjective_phrase_member", None)
        );
        assert_eq!(
            member.roles.keys().copied().collect::<Vec<_>>(),
            [
                "adjective_phrase_member.comma",
                "adjective_phrase_member.conjunction",
                "adjective_phrase_member.phrase",
            ]
        );
        assert_no_flat_subtrees(&predicate);
    }

    #[test]
    fn phrase_coordination_projection_preserves_mixed_with_member_and_list_forms() {
        let catalogs = Catalogs::default()
            .with_catalog(CatalogKind::CardType, ["Creature"])
            .with_catalog(CatalogKind::CreatureType, ["Goblin"])
            .with_catalog(
                CatalogKind::KeywordAbility,
                ["First strike", "Vigilance", "Toxic"],
            );
        let oxford = clean_fragment(
            "a 1/1 red Goblin creature token with first strike, vigilance, and \"Draw a card.\"",
            &catalogs,
            FragmentKind::Nominal,
        );
        let oxford = super::project_fragment(&oxford).expect("the mixed Oxford list projects");
        for (category, construction, roles) in [
            (
                "NominalPhrase",
                "nominal_with_attributes",
                &["attributes", "nominal"][..],
            ),
            (
                "WithAttributeList",
                "with_attribute_list_oxford",
                &["conjunction", "list", "member"][..],
            ),
            (
                "WithAttributeList",
                "with_attribute_list_comma",
                &["list", "member"][..],
            ),
            (
                "WithAttributeList",
                "with_attribute_list_single",
                &["first"][..],
            ),
            (
                "WithAttributeMember",
                "with_attribute_member_keyword",
                &["keyword"][..],
            ),
            (
                "WithAttributeMember",
                "with_attribute_member_quoted",
                &["quoted"][..],
            ),
            (
                "WithAttributeKeyword",
                "with_attribute_keyword_bare",
                &["ability"][..],
            ),
        ] {
            assert_projection_identity(
                required_named_construction(&oxford, construction),
                category,
                construction,
                "only",
                0,
                roles,
            );
        }
        assert_no_flat_subtrees(&oxford);

        let counted = clean_fragment(
            "a 1/1 red Goblin creature token with toxic 1 and \"Draw a card.\"",
            &catalogs,
            FragmentKind::Nominal,
        );
        let counted =
            super::project_fragment(&counted).expect("the mixed counted-keyword list projects");
        assert_projection_identity(
            required_named_construction(&counted, "with_attribute_list_conjoined"),
            "WithAttributeList",
            "with_attribute_list_conjoined",
            "only",
            0,
            &["conjunction", "list", "member"],
        );
        assert_projection_identity(
            required_named_construction(&counted, "with_attribute_keyword_counted"),
            "WithAttributeKeyword",
            "with_attribute_keyword_counted",
            "only",
            0,
            &["ability", "count"],
        );
        assert_no_flat_subtrees(&counted);
    }

    #[test]
    fn relative_projection_derives_gap_from_body_without_a_duplicate_role() {
        let catalogs = Catalogs::default().with_catalog(CatalogKind::CardType, ["Creature"]);
        let fragment = clean_fragment(
            "target creature you control",
            &catalogs,
            FragmentKind::Nominal,
        );
        let relative = nominal_relative(&fragment);
        assert_eq!(relative.gap(), GapState::Object);
        assert!(matches!(relative.body(), RelativeBody::ObjectGap { .. }));

        let projection = super::project_value("RelativeClause", relative)
            .expect("the object-gap relative projects");
        assert_eq!(
            (
                projection.category,
                projection.construction,
                projection.form,
                projection.ordinal,
            ),
            ("RelativeClause", "relative_object", "only", 0)
        );
        assert_eq!(
            projection.roles.keys().copied().collect::<Vec<_>>(),
            ["predicate", "subject"]
        );
        assert!(!projection.roles.contains_key("gap"));
    }

    #[test]
    fn relative_projection_covers_every_remaining_specialized_form() {
        let catalogs = Catalogs::default().with_catalog(CatalogKind::CardType, ["Creature"]);
        for (source, construction, gap, roles, literals) in [
            (
                "target creature you've cast",
                "relative_object_contracted_subject",
                GapState::Object,
                &["predicate", "subject_auxiliary"][..],
                &[][..],
            ),
            (
                "target creature who attacks",
                "relative_subject",
                GapState::Subject,
                &["marker", "predicate"][..],
                &[][..],
            ),
            (
                "creatures that each have a different mana value",
                "relative_subject_distributive_each",
                GapState::Subject,
                &["marker", "predicate"][..],
                &["each"][..],
            ),
            (
                "target creature that's a creature",
                "relative_contracted_copular_noun",
                GapState::Subject,
                &["complement", "subject_auxiliary"][..],
                &[][..],
            ),
            (
                "target creature that's red",
                "relative_contracted_copular_adjective",
                GapState::Subject,
                &["complement", "subject_auxiliary"][..],
                &[][..],
            ),
            (
                "target creature that's in exile",
                "relative_contracted_copular_prepositional",
                GapState::Subject,
                &["complement", "subject_auxiliary"][..],
                &[][..],
            ),
        ] {
            let fragment = clean_fragment(source, &catalogs, FragmentKind::Nominal);
            let relative = nominal_relative(&fragment);
            assert_eq!(relative.gap(), gap, "{source}");
            assert!(
                matches!(
                    (gap, relative.body()),
                    (GapState::Object, RelativeBody::ObjectGap { .. })
                        | (GapState::Subject, RelativeBody::SubjectGap(_))
                ),
                "{source}"
            );

            let projection = super::project_value("RelativeClause", relative)
                .expect("the specialized relative projects");
            assert_eq!(
                (
                    projection.category,
                    projection.construction,
                    projection.form,
                    projection.ordinal,
                ),
                ("RelativeClause", construction, "only", 0),
                "{source}"
            );
            assert_eq!(projection.literals, literals, "{source}");
            assert_eq!(
                projection.roles.keys().copied().collect::<Vec<_>>(),
                roles,
                "{source}"
            );
            assert!(!projection.roles.contains_key("gap"), "{source}");
        }

        let progressive = crate::predicate::build_predicate_verb(
            crate::word::VerbInstance {
                verb: crate::word::Verb::Word(crate::word::Vocab::Attack),
                slot: crate::features::VerbSlot::PresentParticiple,
            },
            crate::predicate::PredicateFrameChoice::Intransitive,
        )
        .and_then(crate::predicate::finish_predicate)
        .expect("the progressive relative predicate builds");
        let relative = crate::clause::build_relative_subject_contracted_auxiliary(
            crate::word::AuxiliaryInstance {
                auxiliary: crate::word::Auxiliary::Be,
                inflection: crate::word::AuxiliaryInflection::Present {
                    person: crate::features::Person::Third,
                    number: Number::Singular,
                },
                contracted_negation: Contraction::Full,
            },
            progressive,
        )
        .expect("the contracted progressive relative builds");
        assert_eq!(relative.gap(), GapState::Subject);
        assert!(matches!(
            relative.body(),
            RelativeBody::SubjectGap(Predicate::Intransitive(_))
        ));
        let projection = super::project_value("RelativeClause", &relative)
            .expect("the contracted progressive relative projects");
        assert_eq!(
            (
                projection.category,
                projection.construction,
                projection.form,
                projection.ordinal,
            ),
            (
                "RelativeClause",
                "relative_subject_contracted_auxiliary",
                "only",
                0,
            )
        );
        assert_eq!(
            projection.roles.keys().copied().collect::<Vec<_>>(),
            ["predicate", "subject_auxiliary"]
        );
        assert!(projection.literals.is_empty());
        assert!(!projection.roles.contains_key("gap"));
    }

    #[test]
    fn relative_projection_keeps_the_coordinated_adjective_owner() {
        let catalogs = Catalogs::default().with_catalog(CatalogKind::CardType, ["Creature"]);
        let fragment = clean_fragment(
            "target creature that's red and green",
            &catalogs,
            FragmentKind::Nominal,
        );
        let relative = nominal_relative(&fragment);
        assert_eq!(relative.gap(), GapState::Subject);
        assert!(matches!(
            relative.body(),
            RelativeBody::SubjectGap(Predicate::Copular(predicate))
                if matches!(
                    predicate.complement(),
                    CopularComplement::CoordinatedAdjective(_)
                )
        ));

        let projection = super::project_value("RelativeClause", relative)
            .expect("the coordinated-adjective relative projects");
        assert_eq!(
            projection.construction,
            "relative_contracted_copular_coordinated_adjective"
        );
        assert_eq!(
            projection.roles.keys().copied().collect::<Vec<_>>(),
            ["complement", "subject_auxiliary"]
        );
        assert_projection_identity(
            required_construction_role(&projection, "complement"),
            "CoordinatedAdjectivePhrase",
            "coordinated_adjective_phrase",
            "only",
            0,
            &["first", "rest"],
        );
        assert_no_flat_subtrees(&projection);
        assert!(!projection.roles.contains_key("gap"));
    }

    #[test]
    fn sentence_projection_keeps_the_clause_role_in_both_forms() {
        let catalogs = Catalogs::default().with_catalog(CatalogKind::CardType, ["Creature"]);
        for (source, form, ordinal, literals) in [
            ("Draw a card.", "period", 0, &["."][..]),
            (
                "Enchanted creature has \"{T}: Draw a card.\"",
                "terminal",
                1,
                &[][..],
            ),
        ] {
            let fragment = clean_fragment(source, &catalogs, FragmentKind::Sentence);
            let projection =
                super::project_fragment(&fragment).expect("the sentence form projects");
            assert_eq!(
                (
                    projection.category,
                    projection.construction,
                    projection.form,
                    projection.ordinal,
                ),
                ("Sentence", "sentence", form, ordinal)
            );
            assert_eq!(projection.literals, literals);
            assert_eq!(
                projection.roles.keys().copied().collect::<Vec<_>>(),
                ["clause"]
            );
            assert_eq!(
                required_construction_role(&projection, "clause").category,
                "Clause"
            );
        }
    }

    #[test]
    fn clause_coordination_projection_covers_every_owner_and_punctuation_form() {
        let catalogs = Catalogs::default()
            .with_catalog(CatalogKind::CreatureType, ["Goblin"])
            .with_catalog(
                CatalogKind::CardType,
                ["Creature", "Instant", "Land", "Sorcery"],
            );
        let fixtures = [
            (
                "You draw a card and you discard a card.",
                "clause_coordination",
                &["conjunction", "first", "next"][..],
                &[][..],
            ),
            (
                "You draw a card, and you discard a card.",
                "clause_coordination_comma",
                &["conjunction", "first", "next"][..],
                &[","][..],
            ),
            (
                "Draw a card, discard a card.",
                "clause_coordination_asyndetic",
                &["first", "next"][..],
                &[","][..],
            ),
            (
                "Enchanted creature gets +1/+1 and is a Goblin in addition to its other types.",
                "clause_coordination_copular_noun_prepositional",
                &["conjunction", "first", "predicate"][..],
                &[][..],
            ),
            (
                "Enchanted creature gets +1/+1, and is a Goblin in addition to its other types.",
                "clause_coordination_copular_noun_prepositional_comma",
                &["conjunction", "first", "predicate"][..],
                &[","][..],
            ),
            (
                "Enchanted creature gets +1/+1, is a Goblin in addition to its other types.",
                "clause_coordination_copular_noun_prepositional_asyndetic",
                &["first", "predicate"][..],
                &[","][..],
            ),
        ];

        for (source, construction, roles, literals) in fixtures {
            let fragment = clean_fragment(source, &catalogs, FragmentKind::Sentence);
            let projection = super::project_fragment(&fragment).unwrap_or_else(|error| {
                panic!("clause fixture did not project: {source}: {error}")
            });
            let owner = required_named_construction(&projection, construction);
            assert_projection_identity(owner, "Clause", construction, "only", 0, roles);
            assert_eq!(owner.literals, literals, "{source:?}: {owner:#?}");
            if construction.contains("copular_noun_prepositional") {
                assert_projection_identity(
                    required_construction_role(owner, "predicate"),
                    "SharedCopularPredicate",
                    "shared_copular_predicate",
                    "only",
                    0,
                    &["complement", "copula", "preposition"],
                );
            }
            assert_no_flat_subtrees(&projection);
        }
    }

    #[test]
    fn clause_coordination_projection_preserves_nested_grouping() {
        let fragment = clean_fragment(
            "You draw a card, then you discard a card, or you lose 1 life.",
            &Catalogs::default(),
            FragmentKind::Sentence,
        );
        let projection = super::project_fragment(&fragment)
            .expect("the nested complete-clause coordination projects");
        let outer = required_named_construction(&projection, "clause_coordination_comma");
        let inner = required_construction_role(outer, "first");
        assert_eq!(
            inner.construction, "clause_coordination_comma",
            "the inner then group remains the outer coordination's first role: {outer:#?}"
        );
        assert_eq!(outer.literals, [","]);
        assert_eq!(inner.literals, [","]);
        assert_no_flat_subtrees(&projection);
    }

    #[test]
    fn elided_grant_projection_keeps_the_complement_and_condition_roles_typed() {
        let catalogs = Catalogs::default()
            .with_catalog(CatalogKind::KeywordAbility, ["Trample", "Haste"])
            .with_catalog(CatalogKind::CreatureType, ["Beast", "Goblin"]);
        for (source, complement_owner) in [
            (
                "This creature has trample as long as you control a Beast and haste as long as you control a Goblin.",
                "shared_grant_ability_complement",
            ),
            (
                "This creature has trample as long as you control a Beast and \"{B}: Regenerate this creature\" as long as you control a Goblin.",
                "shared_grant_quoted_complement",
            ),
        ] {
            let fragment = clean_fragment(source, &catalogs, FragmentKind::Sentence);
            let projection = super::project_fragment(&fragment)
                .unwrap_or_else(|error| panic!("elided grant did not project: {error}"));
            let owner =
                required_named_construction(&projection, "clause_coordination_shared_grant");
            assert_projection_identity(
                owner,
                "Clause",
                "clause_coordination_shared_grant",
                "only",
                0,
                &["attachment", "complement", "conjunction", "first"],
            );
            let complement = required_construction_role(owner, "complement");
            assert_eq!(complement.category, "SharedGrantComplement");
            assert_eq!(complement.construction, complement_owner);
            assert!(owner.literals.is_empty());
            assert_no_flat_subtrees(&projection);
        }
    }
}
