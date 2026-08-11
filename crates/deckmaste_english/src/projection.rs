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
use crate::features::VerbSlot;
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
    provider: &'static str,
    value: &dyn ProjectionValue,
) -> Option<ConstructionProjection> {
    if let Some(value) = value.as_any().downcast_ref::<crate::catalog::CatalogAtom>() {
        return Some(project_catalog_atom(value));
    }
    if provider == "LexicalVerb" {
        return value
            .as_any()
            .downcast_ref::<crate::grammar::VerbAnalysis>()
            .map(project_verb_analysis);
    }
    None
}

fn project_verb_analysis(value: &crate::grammar::VerbAnalysis) -> ConstructionProjection {
    flat_projection(
        "VerbAnalysis",
        "flat_verb_analysis",
        "only",
        0,
        BTreeMap::from([
            (
                "verb",
                atom_identity("Verb", "Verb", &value.instance().verb),
            ),
            (
                "slot",
                ProjectedValue::Construction(Box::new(project_verb_slot(value.instance().slot))),
            ),
            (
                "frame",
                ProjectedValue::Atom(ProjectedAtom::FlatSubtree {
                    category: "PredicateFrame",
                    value: OwnedProjectionValue::new(value.frame()),
                }),
            ),
        ]),
    )
}

fn project_verb_slot(value: VerbSlot) -> ConstructionProjection {
    let (form, ordinal, roles) = match value {
        VerbSlot::Infinitive => ("infinitive", 0, BTreeMap::new()),
        VerbSlot::Imperative => ("imperative", 1, BTreeMap::new()),
        VerbSlot::Present { person, number } => (
            "present",
            2,
            BTreeMap::from([
                ("person", atom_scalar("Person", &person)),
                ("number", atom_scalar("Number", &number)),
            ]),
        ),
        VerbSlot::Past { person, number } => (
            "past",
            3,
            BTreeMap::from([
                ("person", atom_scalar("Person", &person)),
                ("number", atom_scalar("Number", &number)),
            ]),
        ),
        VerbSlot::PresentParticiple => ("present_participle", 4, BTreeMap::new()),
        VerbSlot::PastParticiple => ("past_participle", 5, BTreeMap::new()),
    };
    flat_projection("VerbSlot", "flat_verb_slot", form, ordinal, roles)
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
    use deckmaste_construction_compiler::runtime::ProjectedValue;

    use crate::CatalogKind;
    use crate::Catalogs;
    use crate::FragmentKind;
    use crate::parse_fragment;

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
}
