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
use deckmaste_construction_compiler::runtime::ProjectedSequence;
use deckmaste_construction_compiler::runtime::ProjectedValue;
use deckmaste_construction_compiler::runtime::ProjectedWitness;
use deckmaste_construction_compiler::runtime::ProjectionInput;
use deckmaste_construction_compiler::runtime::ProjectionSink;
use deckmaste_construction_compiler::runtime::ProjectionValue;

use crate::Fragment;
use crate::syntax::Clause;
use crate::syntax::CostComponent;
use crate::syntax::KeywordArgument;
use crate::syntax::NumberLiteral;
use crate::syntax::Phrase;
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
) -> Result<Option<ConstructionProjection>, ProjectionError> {
    if category == "PowerToughness" {
        return Ok(value
            .as_any()
            .downcast_ref::<PowerToughness>()
            .copied()
            .map(project_power_toughness));
    }
    if category == "NumberLiteral" {
        return Ok(value
            .as_any()
            .downcast_ref::<NumberLiteral>()
            .copied()
            .map(project_number_literal));
    }
    if category == "SignedScalar" {
        return Ok(value
            .as_any()
            .downcast_ref::<SignedScalar>()
            .copied()
            .map(project_signed_scalar));
    }
    if category == "KeywordArgument" {
        return value
            .as_any()
            .downcast_ref::<KeywordArgument>()
            .map(project_keyword_argument)
            .transpose();
    }
    Ok(None)
}

fn project_subtree(
    category: &'static str,
    value: &dyn ProjectionValue,
) -> Result<ConstructionProjection, ProjectionError> {
    if category == "NounInstance" && value.as_any().is::<NounInstance>() {
        return project_value("Noun", value);
    }
    project_value(category, value)
}

fn nested_or_flat(
    category: &'static str,
    value: &dyn ProjectionValue,
) -> Result<ProjectedValue, ProjectionError> {
    match project_value(category, value) {
        Ok(projection) => Ok(ProjectedValue::Construction(Box::new(projection))),
        Err(ProjectionError::NoConstruction { .. }) => {
            Ok(ProjectedValue::Atom(ProjectedAtom::FlatSubtree {
                category,
                value: OwnedProjectionValue::new(value),
            }))
        }
        Err(error) => Err(error),
    }
}

fn optional_scalar<T>(codec: &'static str, value: Option<T>) -> ProjectedValue
where
    T: ProjectionValue,
{
    ProjectedValue::Optional(value.map(|value| Box::new(atom_scalar(codec, &value))))
}

fn project_phrase(value: &Phrase) -> Result<ConstructionProjection, ProjectionError> {
    let (form, ordinal, role, projected) = match value {
        Phrase::Clause(value) => (
            "clause",
            0,
            "clause",
            nested_or_flat("Clause", value.as_ref())?,
        ),
        Phrase::NounPhrase(value) => (
            "noun_phrase",
            1,
            "noun_phrase",
            nested_or_flat("NounPhrase", value.as_ref())?,
        ),
        Phrase::AdjectivePhrase(value) => (
            "adjective_phrase",
            2,
            "adjective_phrase",
            nested_or_flat("AdjectivePhrase", value.as_ref())?,
        ),
        Phrase::PrepositionalPhrase(value) => (
            "prepositional_phrase",
            3,
            "prepositional_phrase",
            nested_or_flat("PrepositionalPhrase", value.as_ref())?,
        ),
        Phrase::Quantity(value) => (
            "quantity",
            4,
            "quantity",
            nested_or_flat("Quantity", value)?,
        ),
        Phrase::Adverb(value) => (
            "adverb",
            5,
            "adverb",
            atom_identity("Adverb", "Vocab", value),
        ),
        Phrase::CatalogAtom(value) => (
            "catalog_atom",
            6,
            "catalog_atom",
            ProjectedValue::Construction(Box::new(project_catalog_atom(value))),
        ),
        Phrase::ColorWord(value) => (
            "color_word",
            7,
            "color_word",
            atom_identity("ColorWord", "ColorWord", value),
        ),
        Phrase::Cost(value) => (
            "cost",
            8,
            "cost",
            ProjectedValue::Atom(ProjectedAtom::FlatSubtree {
                category: "Cost",
                value: OwnedProjectionValue::new(value),
            }),
        ),
        Phrase::ThisCard(value) => (
            "this_card",
            9,
            "form",
            atom_identity("ThisCard", "ThisCardForm", value),
        ),
        Phrase::OracleSymbol(value) => (
            "oracle_symbol",
            10,
            "symbol",
            atom_identity("OracleSymbol", "OracleSymbol", value),
        ),
        Phrase::SymbolSequence(value) => (
            "symbol_sequence",
            11,
            "symbols",
            ProjectedValue::Atom(ProjectedAtom::FlatSubtree {
                category: "OracleSymbols",
                value: OwnedProjectionValue::new(value),
            }),
        ),
        Phrase::NumberLiteral(value) => (
            "number_literal",
            12,
            "number",
            ProjectedValue::Construction(Box::new(project_number_literal(*value))),
        ),
        Phrase::SignedScalar(value) => (
            "signed_scalar",
            13,
            "scalar",
            ProjectedValue::Construction(Box::new(project_signed_scalar(*value))),
        ),
        Phrase::PowerToughness(value) => (
            "power_toughness",
            14,
            "stats",
            ProjectedValue::Construction(Box::new(project_power_toughness(*value))),
        ),
        Phrase::EmbeddedAbility(value) => (
            "embedded_ability",
            15,
            "ability",
            nested_or_flat("Ability", value.as_ref())?,
        ),
        Phrase::QuotedAbility(value) => (
            "quoted_ability",
            16,
            "ability",
            ProjectedValue::Atom(ProjectedAtom::FlatSubtree {
                category: "QuotedAbility",
                value: OwnedProjectionValue::new(value.as_ref()),
            }),
        ),
        Phrase::Recovered(value) => (
            "recovered",
            17,
            "text",
            atom_identity("RecoveredText", "RecoveredText", value),
        ),
    };
    Ok(flat_projection(
        "Phrase",
        "flat_phrase",
        form,
        ordinal,
        BTreeMap::from([(role, projected)]),
    ))
}

/// Temporary A01 adapter for the one deliberately opaque bound element in
/// the current ability declarations. The ability output retrofit will replace
/// this with declaration-owned sum and nonempty-sequence structure.
fn project_cost_component(
    value: &CostComponent,
) -> Result<ConstructionProjection, ProjectionError> {
    let (form, ordinal, roles) = match value {
        CostComponent::Symbols(symbols) => (
            "symbols",
            0,
            BTreeMap::from([(
                "symbols",
                ProjectedValue::Atom(ProjectedAtom::FlatSubtree {
                    category: "OracleSymbols",
                    value: OwnedProjectionValue::new(symbols),
                }),
            )]),
        ),
        CostComponent::Clause(clause) => {
            let clause = Clause::Independent((**clause).clone());
            (
                "clause",
                1,
                BTreeMap::from([("clause", nested_or_flat("Clause", &clause)?)]),
            )
        }
        CostComponent::Noun(noun_phrase) => (
            "noun",
            2,
            BTreeMap::from([(
                "noun_phrase",
                nested_or_flat("NounPhrase", noun_phrase.as_ref())?,
            )]),
        ),
        CostComponent::Alternative(left, right) => (
            "alternative",
            3,
            BTreeMap::from([
                (
                    "left",
                    ProjectedValue::Construction(Box::new(project_cost_component(left)?)),
                ),
                (
                    "right",
                    ProjectedValue::Construction(Box::new(project_cost_component(right)?)),
                ),
            ]),
        ),
        CostComponent::Recovered(text) => (
            "recovered",
            4,
            BTreeMap::from([(
                "text",
                atom_identity("RecoveredText", "RecoveredText", text),
            )]),
        ),
    };
    Ok(flat_projection(
        "CostComponent",
        "flat_cost_component",
        form,
        ordinal,
        roles,
    ))
}

fn project_keyword_argument(
    value: &KeywordArgument,
) -> Result<ConstructionProjection, ProjectionError> {
    let (form, ordinal, roles) = match value {
        KeywordArgument::Absent => ("absent", 0, BTreeMap::new()),
        KeywordArgument::Counted(value) => (
            "counted",
            1,
            BTreeMap::from([("count", nested_or_flat("Quantity", value)?)]),
        ),
        KeywordArgument::Costed(value) => (
            "costed",
            2,
            BTreeMap::from([(
                "cost",
                ProjectedValue::Atom(ProjectedAtom::FlatSubtree {
                    category: "KeywordCost",
                    value: OwnedProjectionValue::new(value),
                }),
            )]),
        ),
        KeywordArgument::CountedCost { count, symbols } => (
            "counted_cost",
            3,
            BTreeMap::from([
                (
                    "count",
                    ProjectedValue::Construction(Box::new(project_number_literal(*count))),
                ),
                (
                    "symbols",
                    ProjectedValue::Atom(ProjectedAtom::FlatSubtree {
                        category: "OracleSymbols",
                        value: OwnedProjectionValue::new(symbols),
                    }),
                ),
            ]),
        ),
        KeywordArgument::Predicated(value) => {
            let members = value
                .qualities
                .iter()
                .map(|quality| {
                    Ok(ProjectedMember {
                        element: "flat_predicated_quality",
                        variant: None,
                        roles: BTreeMap::from([
                            (
                                "preposition",
                                optional_scalar("Preposition", quality.preposition),
                            ),
                            (
                                "quality",
                                ProjectedValue::Construction(Box::new(project_phrase(
                                    &quality.quality,
                                )?)),
                            ),
                        ]),
                    })
                })
                .collect::<Result<Vec<_>, ProjectionError>>()?;
            (
                "predicated",
                4,
                BTreeMap::from([(
                    "qualities",
                    ProjectedValue::Sequence(ProjectedSequence {
                        role: "qualities",
                        members,
                    }),
                )]),
            )
        }
        KeywordArgument::Qualified(value) => (
            "qualified",
            5,
            BTreeMap::from([(
                "quality",
                ProjectedValue::Construction(Box::new(project_phrase(value)?)),
            )]),
        ),
        KeywordArgument::Statted { symbols, stats } => (
            "statted",
            6,
            BTreeMap::from([
                (
                    "symbols",
                    ProjectedValue::Atom(ProjectedAtom::FlatSubtree {
                        category: "OracleSymbols",
                        value: OwnedProjectionValue::new(symbols),
                    }),
                ),
                (
                    "stats",
                    ProjectedValue::Construction(Box::new(project_power_toughness(*stats))),
                ),
            ]),
        ),
        KeywordArgument::Named { separator, label } => (
            "named",
            7,
            BTreeMap::from([
                (
                    "separator",
                    atom_scalar("KeywordArgumentSeparator", separator),
                ),
                ("label", atom_identity("KeywordLabel", "String", label)),
            ]),
        ),
        KeywordArgument::Recovered { text } => (
            "recovered",
            8,
            BTreeMap::from([(
                "text",
                atom_identity("RecoveredText", "RecoveredText", text),
            )]),
        ),
        KeywordArgument::RestrictedCost {
            preposition,
            restriction,
            cost,
        } => (
            "restricted_cost",
            9,
            BTreeMap::from([
                ("preposition", optional_scalar("Preposition", *preposition)),
                (
                    "restriction",
                    nested_or_flat("NounPhrase", restriction.as_ref())?,
                ),
                (
                    "cost",
                    ProjectedValue::Atom(ProjectedAtom::FlatSubtree {
                        category: "KeywordCost",
                        value: OwnedProjectionValue::new(cost),
                    }),
                ),
            ]),
        ),
    };
    Ok(flat_projection(
        "KeywordArgument",
        "flat_keyword_argument",
        form,
        ordinal,
        roles,
    ))
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

    fn current_roles(&mut self) -> Result<&mut BTreeMap<&'static str, ProjectedValue>, String> {
        if let Some(sequence) = self.sequences.last_mut() {
            return sequence
                .members
                .last_mut()
                .map(|member| &mut member.roles)
                .ok_or_else(|| {
                    format!(
                        "sequence `{}` emitted a value before a member",
                        sequence.role
                    )
                });
        }
        Ok(&mut self.roles)
    }

    fn insert(&mut self, role: &'static str, value: ProjectedValue) -> Result<(), String> {
        let roles = self.current_roles()?;
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
        match project_subtree(category, value) {
            Ok(projection) => self.insert(role, ProjectedValue::Construction(Box::new(projection))),
            Err(ProjectionError::NoConstruction { .. }) => {
                let projected = project_flat_subtree(category, value)
                    .map_err(|error| error.to_string())?
                    .map_or_else(
                        || {
                            ProjectedValue::Atom(ProjectedAtom::FlatSubtree {
                                category,
                                value: OwnedProjectionValue::new(value),
                            })
                        },
                        |projection| ProjectedValue::Construction(Box::new(projection)),
                    );
                self.insert(role, projected)
            }
            Err(error) => Err(error.to_string()),
        }
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
        self.sequences.push(OpenSequence {
            role,
            expected_len: len,
            members: Vec::with_capacity(len),
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
        value: &dyn ProjectionInput,
    ) -> Result<(), String> {
        let opaque = if element == "cost_component" {
            let component = value
                .as_any()
                .downcast_ref::<CostComponent>()
                .ok_or_else(|| "`cost_component` projected a non-CostComponent value".to_owned())?;
            Some(
                project_cost_component(component)
                    .map(|projection| ProjectedValue::Construction(Box::new(projection)))
                    .map_err(|error| error.to_string())?,
            )
        } else {
            None
        };
        let sequence = self
            .sequences
            .last_mut()
            .ok_or_else(|| format!("element `{element}` has no open sequence"))?;
        let member = sequence
            .members
            .last_mut()
            .ok_or_else(|| format!("element `{element}` has no current member"))?;
        member.element = element;
        if let Some(value) = opaque {
            member.roles.insert("component", value);
        }
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
    use deckmaste_construction_compiler::runtime::ProjectedValue;

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
    use crate::syntax::CoordinatedAdjectivePhrase;
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
            ProjectedValue::Atom(_) | ProjectedValue::Optional(None) => {}
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
            .expect("known noun uses N01");
        assert!(noun.roles.contains_key("identity"));
        assert!(constructions.iter().all(|construction| {
            !matches!(
                construction.construction,
                "flat_noun_instance" | "flat_noun"
            )
        }));
    }

    #[test]
    fn direct_p01_special_forms_project_their_semantic_roles() {
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
    fn nested_p02_projection_preserves_each_typed_object_boundary() {
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
    fn v01_projection_exposes_declared_roles_and_typed_lexical_identity() {
        let sentence = parse_fragment(
            "Draw a card.",
            &Catalogs::default(),
            FragmentKind::Sentence,
            "",
            false,
        )
        .into_fragment()
        .expect("predicate fixture parses");
        let projection =
            super::project_fragment(&sentence).expect("predicate fixture projects through V01");
        let mut constructions = Vec::new();
        collect_constructions(&projection, &mut constructions);
        let direct_object = constructions
            .iter()
            .copied()
            .find(|candidate| candidate.construction == "verb_phrase_direct_object")
            .expect("the selected V01 direct-object construction remains visible");
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
    fn f01_projection_exposes_infinitive_and_recursive_gerund_roles() {
        let predicate = |verb, slot| {
            crate::predicate::build_predicate_verb(
                crate::word::VerbInstance {
                    verb: crate::word::Verb::Word(verb),
                    slot,
                },
                crate::predicate::PredicateFrameChoice::Intransitive,
            )
            .and_then(crate::predicate::finish_predicate)
            .expect("the F01 fixture predicate builds")
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
    fn f02_projection_uses_canonical_finite_subject_and_predicate_roles() {
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
    fn f02_projection_keeps_existential_and_copular_construction_owners() {
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
    fn f03_projection_preserves_recursive_one_edge_attachment_scope() {
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
    fn f03_projection_distinguishes_trailing_subordinate_comma_forms() {
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
    fn f03_projection_preserves_restriction_sequence_roles_and_member_forms() {
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
    fn f03_projection_preserves_recursive_exception_list_forms() {
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
    fn r01_projection_derives_gap_from_body_without_a_duplicate_role() {
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
    fn r01_projection_covers_every_remaining_specialized_relative_form() {
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
    fn r01_projection_keeps_the_coordinated_adjective_owner() {
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
        let Some(ProjectedValue::Atom(ProjectedAtom::FlatSubtree { category, value })) =
            projection.roles.get("complement")
        else {
            panic!("the specialized owner must expose its coordinated complement");
        };
        assert_eq!(*category, "CoordinatedAdjectivePhrase");
        assert!(value.downcast_ref::<CoordinatedAdjectivePhrase>().is_some());
        assert!(!projection.roles.contains_key("gap"));
    }

    #[test]
    fn s01_projection_keeps_the_clause_role_in_both_sentence_forms() {
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
}
