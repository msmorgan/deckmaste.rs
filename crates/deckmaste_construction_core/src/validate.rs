use std::collections::HashMap;
use std::collections::HashSet;

use quote::ToTokens;
use syn::spanned::Spanned;

use crate::feature;
use crate::feature::Feature;
use crate::identifier::BUILD_FUNCTION;
use crate::identifier::CHECKED_BUILD_FUNCTION;
use crate::identifier::FIXED_RUNTIME_TYPE_NAMES;
use crate::identifier::RULE_CATEGORY_TYPE;
use crate::identifier::RULE_CONSTRUCTION_TYPE;
use crate::identifier::RULE_ID_COUNT;
use crate::identifier::RULE_ID_INDEX;
use crate::identifier::RULE_ID_PUBLIC_CONSTRUCTION;
use crate::identifier::RULE_ID_TYPE;
use crate::identifier::RULES_CONSTANT;
use crate::identifier::SEQUENCE_SEPARATOR_FUNCTION;
use crate::identifier::SEQUENCE_TERMINATOR_FUNCTION;
use crate::identifier::StructuralSequenceStyle;
use crate::identifier::VISITOR_TRAIT;
use crate::identifier::category_renderer;
use crate::identifier::feature_helper;
use crate::identifier::is_raw_keyword;
use crate::identifier::key as identifier_key;
use crate::identifier::lexeme_surface_helper;
use crate::identifier::pascal_case;
use crate::identifier::path_key;
use crate::identifier::prefixed;
use crate::identifier::same as same_identifier;
use crate::identifier::snake_case;
use crate::identifier::spelling_key;
use crate::identifier::structural_sequence_aggregate;
use crate::identifier::structural_sequence_builder;
use crate::identifier::structural_sequence_category;
use crate::identifier::structural_sequence_helper_categories;
use crate::identifier::structural_sequence_renderer;
use crate::identifier::structural_sequence_rule;
use crate::identifier::structural_sequence_walker;
use crate::model::CodecAtomClass;
use crate::model::Declaration;
use crate::model::Declarations;
use crate::model::Feature as ParsedFeature;
use crate::model::FeaturePlace as ParsedFeaturePlace;
use crate::model::FeatureValue as ParsedFeatureValue;
use crate::model::FieldKind;
use crate::model::FormAtom;
use crate::model::RequireExprSource;
use crate::model::TerminalBinding;
use crate::model::TraversalKind;
use crate::model::VerbOperand;
use crate::model::VisitMode;
use crate::semantic::EdgeClass;
use crate::semantic::FixedSurfaceAtomPlan;
use crate::semantic::FixedSurfacePlan;
use crate::semantic::InvariantPlan;
use crate::semantic::LengthBounds;
use crate::semantic::PositionalSeparatorPlan;
use crate::semantic::PredicateAtomPlan;
use crate::semantic::PredicateConjunctionPlan;
use crate::semantic::PredicateMemberPlan;
use crate::semantic::PredicateSubjectPlan;
use crate::semantic::ProductPlan;
use crate::semantic::SemanticPlan;
use crate::semantic::SeparatorPlan;
use crate::semantic::SequenceSurfacePlan;
use crate::semantic::StructuralFieldKindPlan;
use crate::semantic::StructuralFieldPlan;
use crate::semantic::StructuralHelperNames;
use crate::semantic::StructuralSemantics;
use crate::semantic::SumAlternativePlan;
use crate::semantic::SumPlan;
use crate::semantic::ValueKindPlan;

#[derive(Debug)]
pub struct ValidatedDeclarations {
    semantic: SemanticPlan,
}

#[derive(Debug, Clone, Copy, Default)]
pub(crate) struct CategoryRenderCapability {
    carries_output: bool,
    requires_external_input: bool,
    requires_context: bool,
}

impl CategoryRenderCapability {
    pub(crate) fn carries_agreement(self) -> bool {
        self.carries_output
    }

    pub(crate) fn requires_external_agreement(self) -> bool {
        self.requires_external_input
    }

    pub(crate) fn requires_context(self) -> bool {
        self.requires_context
    }
}

#[derive(Debug, Clone)]
pub(crate) enum AtomContribution {
    Literal,
    Category {
        role: String,
        category: String,
    },
    Lex {
        role: String,
        terminal: String,
    },
    Identity {
        role: String,
        terminal: String,
    },
    Noun {
        role: String,
        terminal: String,
    },
    VerbFixed {
        terminal: String,
        variant: String,
    },
    OpenDeclaration {
        kind: macro_ron::v2::DeclarationKind,
        name: String,
    },
}

#[derive(Debug, Clone)]
#[allow(
    clippy::struct_excessive_bools,
    reason = "the sealed inventory records independent backend capabilities"
)]
struct TerminalCapabilities {
    name: String,
    lex_atom: bool,
    identity_atom: bool,
    noun_atom: bool,
    verb_atom: bool,
    direct_render: bool,
    direct_build: bool,
    traversal: bool,
}

#[allow(
    dead_code,
    reason = "sealed resolved atom contributions are consumed by Task 4 code generation"
)]
impl AtomContribution {
    pub(crate) fn terminal(&self) -> Option<&str> {
        match self {
            Self::Lex { terminal, .. }
            | Self::Identity { terminal, .. }
            | Self::Noun { terminal, .. }
            | Self::VerbFixed { terminal, .. } => Some(terminal),
            Self::Literal | Self::Category { .. } | Self::OpenDeclaration { .. } => None,
        }
    }

    fn is_complete(&self) -> bool {
        match self {
            Self::Literal => true,
            Self::Category { role, category } => !role.is_empty() && !category.is_empty(),
            Self::Lex { role, terminal }
            | Self::Identity { role, terminal }
            | Self::Noun { role, terminal } => !role.is_empty() && !terminal.is_empty(),
            Self::VerbFixed { terminal, variant } => !terminal.is_empty() && !variant.is_empty(),
            Self::OpenDeclaration { name, .. } => !name.is_empty(),
        }
    }

    fn is_supported_by(&self, terminals: &HashMap<&str, &TerminalCapabilities>) -> bool {
        match self {
            Self::Literal | Self::Category { .. } | Self::OpenDeclaration { .. } => true,
            Self::Lex { terminal, .. } => terminals.get(terminal.as_str()).is_some_and(|info| {
                info.supports_lex_atom() && info.has_direct_render_build_traversal()
            }),
            Self::Identity { terminal, .. } => {
                terminals.get(terminal.as_str()).is_some_and(|info| {
                    info.supports_identity_atom() && info.has_direct_render_build_traversal()
                })
            }
            Self::Noun { terminal, .. } => terminals.get(terminal.as_str()).is_some_and(|info| {
                info.supports_noun_atom() && info.has_direct_render_build_traversal()
            }),
            Self::VerbFixed { terminal, .. } => terminals
                .get(terminal.as_str())
                .is_some_and(|info| info.supports_verb_atom() && info.has_traversal()),
        }
    }

    #[cfg(test)]
    pub(crate) fn snapshot(&self) -> String {
        match self {
            Self::Literal => "literal".to_owned(),
            Self::Category { role, category } => format!("category({role}: {category})"),
            Self::Lex { role, .. } => format!("lex({role})"),
            Self::Identity { role, .. } => format!("identity({role})"),
            Self::Noun { role, .. } => format!("noun({role})"),
            Self::VerbFixed { terminal, variant } => format!("verb({terminal}::{variant})"),
            Self::OpenDeclaration { kind, name } => {
                format!("open_verb({kind:?}, {name}, Verb)")
            }
        }
    }
}

#[allow(
    dead_code,
    reason = "sealed terminal contributions are consumed by Task 4 code generation"
)]
impl TerminalCapabilities {
    pub(crate) fn name(&self) -> &str {
        &self.name
    }

    pub(crate) fn has_direct_render(&self) -> bool {
        self.direct_render
    }

    pub(crate) fn has_direct_build(&self) -> bool {
        self.direct_build
    }

    pub(crate) fn has_traversal(&self) -> bool {
        self.traversal
    }

    pub(crate) fn has_direct_render_build_traversal(&self) -> bool {
        self.has_direct_render() && self.has_direct_build() && self.has_traversal()
    }

    pub(crate) fn supports_lex_atom(&self) -> bool {
        self.lex_atom
    }

    pub(crate) fn supports_identity_atom(&self) -> bool {
        self.identity_atom
    }

    pub(crate) fn supports_noun_atom(&self) -> bool {
        self.noun_atom
    }

    pub(crate) fn supports_verb_atom(&self) -> bool {
        self.verb_atom
    }
}

impl ValidatedDeclarations {
    #[must_use]
    pub fn declaration_count(&self) -> usize {
        self.semantic.declaration_count()
    }

    #[allow(
        dead_code,
        reason = "the sealed semantic authority is introduced ahead of emitter migration"
    )]
    pub(crate) fn semantic(&self) -> &SemanticPlan {
        &self.semantic
    }

    #[cfg(test)]
    pub(crate) fn into_semantic(self) -> SemanticPlan {
        self.semantic
    }

    #[allow(
        dead_code,
        reason = "sealed feature IR is consumed by Task 4 code generation"
    )]
    pub(crate) fn feature_equations(&self, construction: &str) -> &[feature::FeatureEquation] {
        self.semantic.feature_equations(construction)
    }

    #[allow(
        dead_code,
        reason = "sealed boxing metadata is consumed by Task 4 code generation"
    )]
    pub(crate) fn boxed_fields(&self) -> &HashSet<(String, String)> {
        self.semantic.boxed_fields()
    }

    #[allow(
        dead_code,
        reason = "sealed number metadata is consumed by Task 4 code generation"
    )]
    pub(crate) fn dynamic_number_constructions(&self) -> &HashSet<String> {
        self.semantic.dynamic_number_constructions()
    }

    #[cfg(test)]
    fn declaration_names(&self) -> Vec<String> {
        self.semantic
            .declaration_keys()
            .iter()
            .map(|declaration| declaration.name().to_owned())
            .collect()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum TerminalKind {
    Vocab,
    Lexeme,
    Codec,
    Identity,
}

#[derive(Debug)]
struct TerminalInfo {
    kind: TerminalKind,
    codec_atom: Option<CodecAtomClass>,
    variants: HashSet<String>,
    variant_order: Vec<String>,
}

#[derive(Debug)]
struct Symbols {
    categories: HashSet<String>,
    structural_types: HashSet<String>,
    category_variant_order: HashMap<String, Vec<String>>,
    terminals: HashMap<String, TerminalInfo>,
}

#[derive(Debug)]
struct ResolvedGrammar {
    atoms_by_construction: HashMap<String, (proc_macro2::Span, Vec<AtomContribution>)>,
    verb_lexeme_provider: Option<String>,
}

#[allow(
    clippy::needless_pass_by_value,
    reason = "the validation boundary consumes the unsealed declaration graph"
)]
pub(crate) fn validate_declarations(raw: Declarations) -> syn::Result<ValidatedDeclarations> {
    validate_generated_codecs(&raw)?;
    validate_generated_identities(&raw)?;
    validate_morphology(&raw)?;
    let (symbols, _) = validate_namespaces(&raw)?;
    let structural = validate_structural_semantics(&raw, &symbols)?;
    validate_generated_owned_paths(&raw)?;
    let resolved = validate_resolution(&raw, &symbols)?;
    validate_stored_fields(&raw)?;
    validate_bindings(&raw)?;
    let mut invariants = validate_invariants(&raw, &symbols)?;
    let (feature_equations, dynamic_numbers) = validate_features(&raw, &symbols)?;
    let feature_resolutions = seal_feature_resolutions(&raw, &feature_equations, &invariants);
    fold_unit_invariants(&raw, &mut invariants, &feature_resolutions)?;
    let category_render = seal_category_render_capabilities(&raw, &feature_resolutions);
    let category_reads = seal_category_feature_reads(&raw);
    validate_contextual_agreement_uses(&raw, &category_render)?;
    let mut boxed_fields = validate_category_graph(&raw);
    boxed_fields.extend(structural.boxed_fields.iter().cloned());
    validate_roots(&raw, &symbols, &category_render)?;
    validate_backend_completeness(&raw, &resolved)?;
    let ResolvedGrammar {
        atoms_by_construction,
        ..
    } = resolved;
    Ok(ValidatedDeclarations {
        semantic: SemanticPlan::new(
            &raw,
            structural,
            boxed_fields,
            dynamic_numbers,
            category_reads,
            feature_equations,
            feature_resolutions,
            category_render,
            atoms_by_construction,
            invariants,
        )?,
    })
}

#[derive(Debug)]
struct StructuralOwnerDraft {
    source_index: usize,
    name: String,
    node: String,
    fields: Vec<StructuralFieldDraft>,
    is_product: bool,
    has_fixed_width: bool,
    construction_id: Option<String>,
}

#[derive(Debug)]
struct StructuralFieldDraft {
    name: String,
    span: proc_macro2::Span,
    kind: StructuralFieldKindPlan,
    helper_names: Option<StructuralHelperNames>,
    authored_structural: bool,
}

#[derive(Debug)]
struct StructuralSumDraft {
    source_index: usize,
    name: String,
    node: String,
    alternatives: Vec<SumAlternativePlan>,
    alternative_spans: Vec<proc_macro2::Span>,
}

#[allow(
    clippy::too_many_lines,
    reason = "the structural sealing pass accumulates and then seals one mutually recursive declaration graph"
)]
fn validate_structural_semantics(
    raw: &Declarations,
    symbols: &Symbols,
) -> syn::Result<StructuralSemantics> {
    let products = raw
        .declarations
        .iter()
        .filter_map(|declaration| match declaration {
            Declaration::AbstractProduct(product) => Some(identifier_key(&product.name)),
            _ => None,
        })
        .collect::<HashSet<_>>();
    let sums = raw
        .declarations
        .iter()
        .filter_map(|declaration| match declaration {
            Declaration::AbstractSum(sum) => Some(identifier_key(&sum.name)),
            _ => None,
        })
        .collect::<HashSet<_>>();
    let mut errors = None;
    let mut owners = Vec::new();
    let mut sum_drafts = Vec::new();
    let mut bounds_by_owner = HashMap::new();

    for (source_index, declaration) in raw.declarations.iter().enumerate() {
        match declaration {
            Declaration::AbstractProduct(product) => {
                let owner = identifier_key(&product.name);
                let bounds = normalize_length_requirements(
                    &owner,
                    &product.fields,
                    &product.requirements,
                    &mut errors,
                );
                let fields = seal_structural_fields(
                    &owner,
                    &product.fields,
                    &bounds,
                    &products,
                    &sums,
                    symbols,
                    true,
                    &mut errors,
                );
                bounds_by_owner.insert(owner.clone(), bounds);
                owners.push(StructuralOwnerDraft {
                    source_index,
                    node: structural_product_node(&owner),
                    name: owner,
                    fields,
                    is_product: true,
                    has_fixed_width: false,
                    construction_id: None,
                });
            }
            Declaration::Construction(construction) => {
                let owner = identifier_key(&construction.element.name);
                let bounds = normalize_length_requirements(
                    &owner,
                    &construction.element.fields,
                    &construction.requirements,
                    &mut errors,
                );
                let fields = seal_structural_fields(
                    &owner,
                    &construction.element.fields,
                    &bounds,
                    &products,
                    &sums,
                    symbols,
                    false,
                    &mut errors,
                );
                bounds_by_owner.insert(owner.clone(), bounds);
                owners.push(StructuralOwnerDraft {
                    source_index,
                    node: structural_construction_node(&identifier_key(&construction.name)),
                    name: owner,
                    fields,
                    is_product: false,
                    has_fixed_width: construction_has_fixed_width(construction),
                    construction_id: Some(identifier_key(&construction.name)),
                });
            }
            Declaration::AbstractSum(sum) => {
                let owner = identifier_key(&sum.name);
                let mut alternatives = Vec::new();
                let mut alternative_spans = Vec::new();
                for alternative in &sum.alternatives {
                    let role = identifier_key(&alternative.name);
                    let target = path_name(&alternative.value_type);
                    let value = resolve_structural_bare_value(&target, &products, &sums, symbols);
                    if let Some(value) = value {
                        alternatives.push(SumAlternativePlan::new(role, value));
                        alternative_spans.push(alternative.name.span());
                    } else {
                        combine(
                            &mut errors,
                            syn::Error::new_spanned(
                                &alternative.value_type,
                                format!(
                                    "{owner}.{role}: unresolved structural alternative {target}"
                                ),
                            ),
                        );
                    }
                }
                sum_drafts.push(StructuralSumDraft {
                    source_index,
                    node: structural_sum_node(&owner),
                    name: owner,
                    alternatives,
                    alternative_spans,
                });
            }
            Declaration::Vocab(_)
            | Declaration::Morphology(_)
            | Declaration::Lexeme(_)
            | Declaration::Codec(_)
            | Declaration::Identity(_)
            | Declaration::Root(_) => {}
        }
    }

    let mut category_members: HashMap<String, Vec<String>> = HashMap::new();
    for declaration in &raw.declarations {
        if let Declaration::Construction(construction) = declaration {
            category_members
                .entry(path_name(&construction.category))
                .or_default()
                .push(structural_construction_node(&identifier_key(
                    &construction.name,
                )));
        }
    }
    let nullable_nodes = compute_structural_nullability(&owners, &sum_drafts, &category_members);
    validate_nullable_repeated_items(&owners, &nullable_nodes, &mut errors);
    validate_zero_width_cycles(
        &owners,
        &sum_drafts,
        &category_members,
        &nullable_nodes,
        &mut errors,
    );
    finish(errors)?;

    let all_edges = structural_dependency_edges(&owners, &sum_drafts, &category_members);
    let construction_fields = owners
        .iter()
        .filter_map(|owner| owner.construction_id.as_ref().map(|id| (id, owner)))
        .flat_map(|(construction_id, owner)| {
            let all_edges = &all_edges;
            owner
                .fields
                .iter()
                .filter(|field| field.authored_structural)
                .map(move |field| {
                    let target = structural_value_node(field.kind.value());
                    (
                        (construction_id.clone(), field.name.clone()),
                        StructuralFieldPlan::new(
                            field.name.clone(),
                            field.span,
                            field.kind.clone(),
                            graph_reaches(all_edges, &target, &owner.node),
                            field.helper_names.clone(),
                        ),
                    )
                })
        })
        .collect();
    let construction_nullability = owners
        .iter()
        .filter_map(|owner| {
            owner.construction_id.as_ref().map(|construction_id| {
                (
                    construction_id.clone(),
                    nullable_nodes.contains(&owner.node),
                )
            })
        })
        .collect();
    let boxed_fields = owners
        .iter()
        .filter_map(|owner| owner.construction_id.as_ref().map(|id| (id, owner)))
        .flat_map(|(construction_id, owner)| {
            let all_edges = &all_edges;
            owner.fields.iter().filter_map(move |field| {
                let target = structural_value_node(field.kind.value());
                (field.authored_structural && graph_reaches(all_edges, &target, &owner.node))
                    .then(|| (construction_id.clone(), field.name.clone()))
            })
        })
        .collect();
    let product_plans = owners
        .into_iter()
        .filter(|owner| owner.is_product)
        .map(|owner| {
            let fields = owner
                .fields
                .into_iter()
                .map(|field| {
                    let target = structural_value_node(field.kind.value());
                    let recursive = graph_reaches(&all_edges, &target, &owner.node);
                    StructuralFieldPlan::new(
                        field.name,
                        field.span,
                        field.kind,
                        recursive,
                        field.helper_names,
                    )
                })
                .collect();
            ProductPlan::new(
                owner.source_index,
                owner.name.clone(),
                fields,
                bounds_by_owner.remove(&owner.name).unwrap_or_default(),
                nullable_nodes.contains(&owner.node),
            )
        })
        .collect();
    let sum_plans = sum_drafts
        .into_iter()
        .map(|sum| {
            let alternatives = sum
                .alternatives
                .into_iter()
                .map(|alternative| {
                    let target = structural_value_node(alternative.value());
                    let recursive = graph_reaches(&all_edges, &target, &sum.node);
                    alternative.with_recursive(recursive)
                })
                .collect();
            SumPlan::new(
                sum.source_index,
                sum.name,
                alternatives,
                nullable_nodes.contains(&sum.node),
            )
        })
        .collect();
    let nullable_types = nullable_nodes
        .iter()
        .filter_map(|node| {
            node.strip_prefix("product:")
                .or_else(|| node.strip_prefix("sum:"))
                .or_else(|| node.strip_prefix("category:"))
                .map(str::to_owned)
        })
        .collect();
    Ok(StructuralSemantics {
        products: product_plans,
        sums: sum_plans,
        nullable_types,
        construction_fields,
        construction_nullability,
        boxed_fields,
    })
}

fn construction_has_fixed_width(construction: &crate::Construction) -> bool {
    let fields = construction
        .element
        .fields
        .iter()
        .map(|field| (identifier_key(&field.name), &field.kind))
        .collect::<HashMap<_, _>>();
    construction.form.atoms.iter().any(|atom| match atom {
        FormAtom::Literal(value) => !value.value().is_empty(),
        FormAtom::Lex(role) | FormAtom::Identity(role) | FormAtom::Noun(role) => {
            fields.get(&identifier_key(role)).is_some_and(|kind| {
                !matches!(kind, FieldKind::Optional(_) | FieldKind::Sequence { .. })
            })
        }
        FormAtom::Verb(_) | FormAtom::OpenVerb(_) => true,
        FormAtom::Role(_) => false,
    })
}

fn normalize_length_requirements(
    owner: &str,
    fields: &[crate::model::Field],
    requirements: &[RequireExprSource],
    errors: &mut Option<syn::Error>,
) -> HashMap<String, LengthBounds> {
    let sequences = fields
        .iter()
        .filter(|field| matches!(field.kind, FieldKind::Sequence { .. }))
        .map(|field| (identifier_key(&field.name), field.name.span()))
        .collect::<HashMap<_, _>>();
    let mut bounds = sequences
        .keys()
        .map(|role| (role.clone(), LengthBounds::new(0, None)))
        .collect::<HashMap<_, _>>();
    for requirement in requirements {
        if matches!(
            requirement,
            RequireExprSource::All(_) | RequireExprSource::Any(_)
        ) {
            reject_grouped_length_requirements(owner, requirement, errors);
            continue;
        }
        let RequireExprSource::Length {
            owner: authored_owner,
            role,
            comparison,
            value,
        } = requirement
        else {
            continue;
        };
        let role_name = identifier_key(role);
        let label = format!("{owner}.{role_name}");
        if authored_owner
            .as_ref()
            .is_some_and(|authored| path_name(authored) != owner)
        {
            combine(
                errors,
                syn::Error::new_spanned(
                    authored_owner.as_ref().expect("checked owner"),
                    format!("{label}: length requirement names a different owner"),
                ),
            );
            continue;
        }
        if !sequences.contains_key(&role_name) {
            combine(
                errors,
                syn::Error::new(
                    role.span(),
                    format!("{label}: length requirement requires a sequence role"),
                ),
            );
            continue;
        }
        let Ok(number) = value.base10_parse::<usize>() else {
            combine(
                errors,
                syn::Error::new(value.span(), format!("{label}: length bound overflow")),
            );
            continue;
        };
        let requirement_bounds = match comparison {
            crate::model::LengthComparison::Equal => LengthBounds::new(number, Some(number)),
            crate::model::LengthComparison::GreaterThanOrEqual => LengthBounds::new(number, None),
            crate::model::LengthComparison::GreaterThan => {
                let Some(min) = number.checked_add(1) else {
                    combine(
                        errors,
                        syn::Error::new(value.span(), format!("{label}: length bound overflow")),
                    );
                    continue;
                };
                LengthBounds::new(min, None)
            }
            crate::model::LengthComparison::LessThanOrEqual => LengthBounds::new(0, Some(number)),
            crate::model::LengthComparison::LessThan => {
                let Some(max) = number.checked_sub(1) else {
                    combine(
                        errors,
                        syn::Error::new(value.span(), format!("{label}: len < 0 is impossible")),
                    );
                    continue;
                };
                LengthBounds::new(0, Some(max))
            }
        };
        let current = bounds
            .get(&role_name)
            .copied()
            .expect("sequence bounds initialized");
        let min = current.min().max(requirement_bounds.min());
        let max = match (current.max(), requirement_bounds.max()) {
            (Some(left), Some(right)) => Some(left.min(right)),
            (Some(value), None) | (None, Some(value)) => Some(value),
            (None, None) => None,
        };
        if max.is_some_and(|max| min > max) {
            combine(
                errors,
                syn::Error::new(
                    value.span(),
                    format!("{label}: contradictory length requirements"),
                ),
            );
        } else {
            bounds.insert(role_name, LengthBounds::new(min, max));
        }
    }
    bounds
}

fn reject_grouped_length_requirements(
    owner: &str,
    requirement: &RequireExprSource,
    errors: &mut Option<syn::Error>,
) {
    match requirement {
        RequireExprSource::Length { role, .. } => {
            let role_name = identifier_key(role);
            combine(
                errors,
                syn::Error::new(
                    role.span(),
                    format!("{owner}.{role_name}: grouped length requirements are unsupported"),
                ),
            );
        }
        RequireExprSource::All(operands) | RequireExprSource::Any(operands) => {
            for operand in operands {
                reject_grouped_length_requirements(owner, operand, errors);
            }
        }
        RequireExprSource::In { .. } => {}
    }
}

#[allow(
    clippy::too_many_arguments,
    reason = "structural field sealing needs the complete resolved declaration inventory"
)]
fn seal_structural_fields(
    owner: &str,
    fields: &[crate::model::Field],
    bounds: &HashMap<String, LengthBounds>,
    products: &HashSet<String>,
    sums: &HashSet<String>,
    symbols: &Symbols,
    abstract_product: bool,
    errors: &mut Option<syn::Error>,
) -> Vec<StructuralFieldDraft> {
    fields
        .iter()
        .filter_map(|field| {
            let role = identifier_key(&field.name);
            let label = format!("{owner}.{role}");
            let (kind, helper_names, authored_structural) = match &field.kind {
                FieldKind::Optional(value) => (
                    resolve_structural_field_value(value, &label, products, sums, symbols, errors)
                        .map(StructuralFieldKindPlan::Optional),
                    None,
                    true,
                ),
                FieldKind::Sequence { item, surface } => {
                    let value = resolve_structural_field_value(
                        item, &label, products, sums, symbols, errors,
                    );
                    let role_bounds = bounds
                        .get(&role)
                        .copied()
                        .unwrap_or_else(|| LengthBounds::new(0, None));
                    let surface = seal_sequence_surface(
                        &label,
                        field.name.span(),
                        surface,
                        role_bounds,
                        symbols,
                        errors,
                    );
                    (
                        value.map(|item| StructuralFieldKindPlan::Sequence {
                            item,
                            bounds: role_bounds,
                            surface,
                        }),
                        Some(structural_helper_names(owner, &role)),
                        true,
                    )
                }
                FieldKind::Category(_) | FieldKind::Lex(_) | FieldKind::Identity(_) => (
                    if abstract_product {
                        resolve_structural_field_value(
                            &field.kind,
                            &label,
                            products,
                            sums,
                            symbols,
                            errors,
                        )
                    } else {
                        resolve_structural_field_value_silent(&field.kind, products, sums, symbols)
                    }
                    .map(StructuralFieldKindPlan::Required),
                    None,
                    abstract_product
                        || matches!(
                            resolve_structural_field_value_silent(
                                &field.kind,
                                products,
                                sums,
                                symbols,
                            ),
                            Some(ValueKindPlan::Product(_) | ValueKindPlan::Sum(_))
                        ),
                ),
            };
            kind.map(|kind| StructuralFieldDraft {
                name: role,
                span: field.name.span(),
                kind,
                helper_names,
                authored_structural,
            })
        })
        .collect()
}

fn resolve_structural_field_value_silent(
    kind: &FieldKind,
    products: &HashSet<String>,
    sums: &HashSet<String>,
    symbols: &Symbols,
) -> Option<ValueKindPlan> {
    let (path, explicit) = match kind {
        FieldKind::Category(path) => (path, None),
        FieldKind::Lex(path) => (path, Some(false)),
        FieldKind::Identity(path) => (path, Some(true)),
        FieldKind::Optional(_) | FieldKind::Sequence { .. } => return None,
    };
    let name = path_name(path);
    match explicit {
        None => resolve_structural_bare_value(&name, products, sums, symbols),
        Some(true) => symbols
            .terminals
            .get(&name)
            .filter(|terminal| terminal.kind == TerminalKind::Identity)
            .map(|_| ValueKindPlan::Identity(name)),
        Some(false) => symbols
            .terminals
            .get(&name)
            .filter(|terminal| terminal.kind != TerminalKind::Identity)
            .map(|_| ValueKindPlan::Lex(name)),
    }
}

fn resolve_structural_field_value(
    kind: &FieldKind,
    label: &str,
    products: &HashSet<String>,
    sums: &HashSet<String>,
    symbols: &Symbols,
    errors: &mut Option<syn::Error>,
) -> Option<ValueKindPlan> {
    let (path, explicit) = match kind {
        FieldKind::Category(path) => (path, None),
        FieldKind::Lex(path) => (path, Some(TerminalKind::Codec)),
        FieldKind::Identity(path) => (path, Some(TerminalKind::Identity)),
        FieldKind::Optional(_) | FieldKind::Sequence { .. } => {
            unreachable!("the parser rejects nested structural cardinality")
        }
    };
    let name = path_name(path);
    let resolved = match explicit {
        None => resolve_structural_bare_value(&name, products, sums, symbols),
        Some(TerminalKind::Identity) => symbols
            .terminals
            .get(&name)
            .filter(|terminal| terminal.kind == TerminalKind::Identity)
            .map(|_| ValueKindPlan::Identity(name.clone())),
        Some(_) => symbols
            .terminals
            .get(&name)
            .filter(|terminal| terminal.kind != TerminalKind::Identity)
            .map(|_| ValueKindPlan::Lex(name.clone())),
    };
    if resolved.is_none() {
        combine(
            errors,
            syn::Error::new_spanned(path, format!("{label}: unresolved structural value {name}")),
        );
    }
    resolved
}

fn resolve_structural_bare_value(
    name: &str,
    products: &HashSet<String>,
    sums: &HashSet<String>,
    symbols: &Symbols,
) -> Option<ValueKindPlan> {
    if products.contains(name) {
        Some(ValueKindPlan::Product(name.to_owned()))
    } else if sums.contains(name) {
        Some(ValueKindPlan::Sum(name.to_owned()))
    } else if symbols.categories.contains(name) {
        Some(ValueKindPlan::Category(name.to_owned()))
    } else {
        None
    }
}

fn seal_sequence_surface(
    label: &str,
    role_span: proc_macro2::Span,
    source: &crate::model::SequenceSurfaceSource,
    bounds: LengthBounds,
    symbols: &Symbols,
    errors: &mut Option<syn::Error>,
) -> SequenceSurfacePlan {
    let separator = source.separator.as_ref().map(|separator| match separator {
        crate::model::SeparatorSource::Uniform(surface) => {
            let surface = seal_fixed_surface(label, "separator", surface, symbols, errors);
            SeparatorPlan::Uniform(surface)
        }
        crate::model::SeparatorSource::Positional(rows) => {
            let mut seen = HashSet::new();
            let mut sealed = Vec::new();
            for row in rows {
                let spelling = identifier_key(&row.class);
                let class = match spelling.as_str() {
                    "pair" => Some(EdgeClass::Pair),
                    "first" => Some(EdgeClass::First),
                    "middle" => Some(EdgeClass::Middle),
                    "last" => Some(EdgeClass::Last),
                    _ => None,
                };
                let Some(class) = class else {
                    combine(
                        errors,
                        syn::Error::new(
                            row.class.span(),
                            format!("{label}: unknown positional separator class {spelling}"),
                        ),
                    );
                    continue;
                };
                if !seen.insert(class) {
                    combine(
                        errors,
                        syn::Error::new(row.class.span(), format!("{label}: duplicate {spelling}")),
                    );
                    continue;
                }
                let surface = seal_fixed_surface(label, "separator", &row.surface, symbols, errors);
                sealed.push(PositionalSeparatorPlan::new(class, surface));
            }
            for (class, spelling, reachable) in [
                (EdgeClass::Pair, "pair", bounds.allows(2)),
                (EdgeClass::First, "first", bounds.allows_at_least(3)),
                (EdgeClass::Middle, "middle", bounds.allows_at_least(4)),
                (EdgeClass::Last, "last", bounds.allows_at_least(3)),
            ] {
                if reachable && !seen.contains(&class) {
                    combine(
                        errors,
                        syn::Error::new(role_span, format!("{label}: missing {spelling}")),
                    );
                } else if !reachable && seen.contains(&class) {
                    let span = rows
                        .iter()
                        .find(|row| identifier_key(&row.class) == spelling)
                        .map_or_else(proc_macro2::Span::call_site, |row| row.class.span());
                    combine(
                        errors,
                        syn::Error::new(span, format!("{label}: unreachable {spelling}")),
                    );
                }
            }
            SeparatorPlan::Positional(sealed)
        }
    });
    let terminator = source
        .terminator
        .as_ref()
        .map(|surface| seal_fixed_surface(label, "terminator", surface, symbols, errors));
    SequenceSurfacePlan::new(separator, terminator)
}

fn seal_fixed_surface(
    label: &str,
    role: &str,
    source: &crate::model::FixedSurfaceSource,
    symbols: &Symbols,
    errors: &mut Option<syn::Error>,
) -> FixedSurfacePlan {
    let atoms = source
        .atoms
        .iter()
        .filter_map(|atom| match atom {
            crate::model::FixedSurfaceAtomSource::Literal(value) => {
                Some(FixedSurfaceAtomPlan::Literal(value.value()))
            }
            crate::model::FixedSurfaceAtomSource::Lex(path) => {
                let segments = &path.segments;
                let variant = segments.last()?;
                let Some(terminal) = segments.iter().rev().nth(1) else {
                    combine(
                        errors,
                        syn::Error::new_spanned(
                            path,
                            format!("{label}: fixed surface lexeme requires Type::Variant"),
                        ),
                    );
                    return None;
                };
                let terminal_name = identifier_key(&terminal.ident);
                let variant_name = identifier_key(&variant.ident);
                if !symbols
                    .terminals
                    .get(&terminal_name)
                    .is_some_and(|info| info.variants.contains(&variant_name))
                {
                    combine(
                        errors,
                        syn::Error::new_spanned(
                            path,
                            format!(
                                "{label}: unresolved fixed surface lexeme {terminal_name}::{variant_name}"
                            ),
                        ),
                    );
                    return None;
                }
                Some(FixedSurfaceAtomPlan::Lex {
                    terminal: terminal_name,
                    variant: variant_name,
                })
            }
        })
        .collect();
    let surface = FixedSurfacePlan::new(atoms);
    if surface.is_empty() {
        let span =
            source
                .atoms
                .first()
                .map_or_else(proc_macro2::Span::call_site, |atom| match atom {
                    crate::model::FixedSurfaceAtomSource::Literal(value) => value.span(),
                    crate::model::FixedSurfaceAtomSource::Lex(path) => path.span(),
                });
        combine(
            errors,
            syn::Error::new(span, format!("{label}: empty {role} surface")),
        );
    }
    surface
}

fn structural_helper_names(owner: &str, role: &str) -> StructuralHelperNames {
    StructuralHelperNames::new(
        structural_sequence_aggregate(owner, role),
        structural_sequence_category(owner, role),
        structural_sequence_rule(owner, role),
        structural_sequence_builder(owner, role),
        structural_sequence_renderer(owner, role),
        structural_sequence_walker(owner, role),
    )
}

fn structural_product_node(name: &str) -> String {
    format!("product:{name}")
}

fn structural_sum_node(name: &str) -> String {
    format!("sum:{name}")
}

fn structural_category_node(name: &str) -> String {
    format!("category:{name}")
}

fn structural_construction_node(name: &str) -> String {
    format!("construction:{name}")
}

fn structural_value_node(value: &ValueKindPlan) -> String {
    match value {
        ValueKindPlan::Category(name) => structural_category_node(name),
        ValueKindPlan::Product(name) => structural_product_node(name),
        ValueKindPlan::Sum(name) => structural_sum_node(name),
        ValueKindPlan::Lex(name) => format!("lex:{name}"),
        ValueKindPlan::Identity(name) => format!("identity:{name}"),
    }
}

fn structural_dependency_edges(
    owners: &[StructuralOwnerDraft],
    sums: &[StructuralSumDraft],
    category_members: &HashMap<String, Vec<String>>,
) -> HashMap<String, Vec<String>> {
    let mut edges = HashMap::new();
    for owner in owners {
        edges.insert(
            owner.node.clone(),
            owner
                .fields
                .iter()
                .map(|field| structural_value_node(field.kind.value()))
                .collect(),
        );
    }
    for sum in sums {
        edges.insert(
            sum.node.clone(),
            sum.alternatives
                .iter()
                .map(|alternative| structural_value_node(alternative.value()))
                .collect(),
        );
    }
    for (category, members) in category_members {
        edges.insert(structural_category_node(category), members.clone());
    }
    edges
}

fn graph_reaches(edges: &HashMap<String, Vec<String>>, from: &str, target: &str) -> bool {
    let mut todo = vec![from.to_owned()];
    let mut seen = HashSet::new();
    while let Some(node) = todo.pop() {
        if node == target {
            return true;
        }
        if seen.insert(node.clone()) {
            todo.extend(edges.get(&node).into_iter().flatten().cloned());
        }
    }
    false
}

fn compute_structural_nullability(
    owners: &[StructuralOwnerDraft],
    sums: &[StructuralSumDraft],
    category_members: &HashMap<String, Vec<String>>,
) -> HashSet<String> {
    let mut nullable = HashSet::new();
    loop {
        let before = nullable.len();
        for owner in owners {
            if !owner.has_fixed_width
                && owner
                    .fields
                    .iter()
                    .all(|field| structural_field_is_nullable(&field.kind, &nullable))
            {
                nullable.insert(owner.node.clone());
            }
        }
        for sum in sums {
            if sum
                .alternatives
                .iter()
                .any(|alternative| nullable.contains(&structural_value_node(alternative.value())))
            {
                nullable.insert(sum.node.clone());
            }
        }
        for (category, members) in category_members {
            if members.iter().any(|member| nullable.contains(member)) {
                nullable.insert(structural_category_node(category));
            }
        }
        if nullable.len() == before {
            return nullable;
        }
    }
}

fn structural_field_is_nullable(
    kind: &StructuralFieldKindPlan,
    nullable: &HashSet<String>,
) -> bool {
    match kind {
        StructuralFieldKindPlan::Required(value) => {
            nullable.contains(&structural_value_node(value))
        }
        StructuralFieldKindPlan::Optional(_) => true,
        StructuralFieldKindPlan::Sequence {
            item,
            bounds,
            surface,
        } => {
            bounds.allows(0)
                || (nullable.contains(&structural_value_node(item))
                    && sequence_surface_can_be_empty(surface, *bounds))
        }
    }
}

fn sequence_surface_can_be_empty(surface: &SequenceSurfacePlan, bounds: LengthBounds) -> bool {
    if bounds.allows(0) {
        return true;
    }
    if surface.terminator().is_some() {
        return false;
    }
    bounds.allows(1) || surface.separator().is_none()
}

fn validate_nullable_repeated_items(
    owners: &[StructuralOwnerDraft],
    nullable: &HashSet<String>,
    errors: &mut Option<syn::Error>,
) {
    for owner in owners {
        for field in &owner.fields {
            let StructuralFieldKindPlan::Sequence { item, bounds, .. } = &field.kind else {
                continue;
            };
            if bounds.max().is_none() && nullable.contains(&structural_value_node(item)) {
                combine(
                    errors,
                    syn::Error::new(
                        field.span,
                        format!("{}.{}: nullable repeated item", owner.name, field.name),
                    ),
                );
            }
        }
    }
}

fn validate_zero_width_cycles(
    owners: &[StructuralOwnerDraft],
    sums: &[StructuralSumDraft],
    category_members: &HashMap<String, Vec<String>>,
    nullable: &HashSet<String>,
    errors: &mut Option<syn::Error>,
) {
    let mut zero_edges: HashMap<String, Vec<String>> = category_members
        .iter()
        .map(|(category, members)| (structural_category_node(category), members.clone()))
        .collect();
    for sum in sums {
        zero_edges.insert(
            sum.node.clone(),
            sum.alternatives
                .iter()
                .map(|alternative| structural_value_node(alternative.value()))
                .collect(),
        );
    }
    for owner in owners {
        let mut targets = Vec::new();
        if owner.has_fixed_width {
            zero_edges.insert(owner.node.clone(), targets);
            continue;
        }
        for (index, field) in owner.fields.iter().enumerate() {
            let siblings_nullable = owner.fields.iter().enumerate().all(|(other, sibling)| {
                index == other || structural_field_is_nullable(&sibling.kind, nullable)
            });
            if siblings_nullable && field_can_expose_zero_width_edge(&field.kind) {
                targets.push(structural_value_node(field.kind.value()));
            }
        }
        zero_edges.insert(owner.node.clone(), targets);
    }
    for owner in owners {
        if owner.has_fixed_width {
            continue;
        }
        for (index, field) in owner.fields.iter().enumerate() {
            let siblings_nullable = owner.fields.iter().enumerate().all(|(other, sibling)| {
                index == other || structural_field_is_nullable(&sibling.kind, nullable)
            });
            if !field.authored_structural
                || !siblings_nullable
                || !field_can_expose_zero_width_edge(&field.kind)
            {
                continue;
            }
            let target = structural_value_node(field.kind.value());
            if graph_reaches(&zero_edges, &target, &owner.node) {
                combine(
                    errors,
                    syn::Error::new(
                        field.span,
                        format!("{}.{}: zero-width recursive cycle", owner.name, field.name),
                    ),
                );
            }
        }
    }
    for sum in sums {
        for (alternative, span) in sum.alternatives.iter().zip(&sum.alternative_spans) {
            let target = structural_value_node(alternative.value());
            if graph_reaches(&zero_edges, &target, &sum.node) {
                combine(
                    errors,
                    syn::Error::new(
                        *span,
                        format!(
                            "{}.{}: zero-width recursive cycle",
                            sum.name,
                            alternative.name()
                        ),
                    ),
                );
            }
        }
    }
}

fn field_can_expose_zero_width_edge(kind: &StructuralFieldKindPlan) -> bool {
    match kind {
        StructuralFieldKindPlan::Required(_) | StructuralFieldKindPlan::Optional(_) => true,
        StructuralFieldKindPlan::Sequence {
            bounds, surface, ..
        } => {
            let has_positive = bounds.max().is_none_or(|max| max >= 1);
            has_positive && sequence_surface_can_be_empty(surface, *bounds)
        }
    }
}

fn validate_morphology(raw: &Declarations) -> syn::Result<()> {
    use crate::morphology::MorphologyRecipe;

    let mut independent_errors = None;
    let mut morphology_names = HashSet::new();
    for declaration in &raw.declarations {
        match declaration {
            Declaration::Morphology(morphology) => {
                let name = identifier_key(&morphology.name);
                if !morphology_names.insert(name.clone()) {
                    combine(
                        &mut independent_errors,
                        syn::Error::new(
                            morphology.name.span(),
                            format!("duplicate morphology declaration `{name}`"),
                        ),
                    );
                }
                match MorphologyRecipe::from_ident(&morphology.recipe) {
                    None => combine(
                        &mut independent_errors,
                        syn::Error::new(
                            morphology.recipe.span(),
                            format!("unknown morphology recipe `{}`", morphology.recipe),
                        ),
                    ),
                    Some(recipe) if recipe.feature() != morphology.feature => combine(
                        &mut independent_errors,
                        syn::Error::new(
                            morphology.name.span(),
                            format!("morphology `{name}` feature axis does not match recipe"),
                        ),
                    ),
                    Some(_) => {}
                }
            }
            Declaration::Lexeme(lexeme) => {
                validate_lexeme_declaration_shape(lexeme, &mut independent_errors);
            }
            Declaration::Construction(_)
            | Declaration::AbstractProduct(_)
            | Declaration::AbstractSum(_)
            | Declaration::Vocab(_)
            | Declaration::Codec(_)
            | Declaration::Identity(_)
            | Declaration::Root(_) => {}
        }
    }
    finish(independent_errors)?;

    let morphologies = raw
        .declarations
        .iter()
        .filter_map(|declaration| {
            let Declaration::Morphology(morphology) = declaration else {
                return None;
            };
            Some((
                identifier_key(&morphology.name),
                MorphologyRecipe::from_ident(&morphology.recipe)
                    .expect("independently validated morphology recipe"),
            ))
        })
        .collect::<HashMap<_, _>>();
    let mut errors = None;
    let mut verb_providers = Vec::new();
    let mut noun_providers = Vec::new();
    for declaration in &raw.declarations {
        let Declaration::Lexeme(lexeme) = declaration else {
            continue;
        };
        let morphology_name = identifier_key(&lexeme.morphology);
        let Some(&recipe) = morphologies.get(&morphology_name) else {
            combine(
                &mut errors,
                syn::Error::new(
                    lexeme.morphology.span(),
                    format!("unknown morphology `{morphology_name}`"),
                ),
            );
            continue;
        };
        match recipe {
            MorphologyRecipe::EnglishVerb => {
                verb_providers.push((&lexeme.name, identifier_key(&lexeme.name)));
            }
            MorphologyRecipe::EnglishNoun => {
                noun_providers.push((&lexeme.name, identifier_key(&lexeme.name)));
            }
        }
        for member in &lexeme.members {
            for row in &member.overrides {
                if recipe.feature_from_ident(&row.feature).is_none() {
                    combine(
                        &mut errors,
                        syn::Error::new(
                            row.feature.span(),
                            format!("unknown override feature `{}`", row.feature),
                        ),
                    );
                }
            }
        }
    }
    for (kind, providers) in [("verb", verb_providers), ("noun", noun_providers)] {
        if let Some((_, first)) = providers.first() {
            for (name, provider) in providers.iter().skip(1) {
                combine(
                    &mut errors,
                    syn::Error::new(
                        name.span(),
                        format!("multiple {kind} lexeme providers `{first}` and `{provider}`"),
                    ),
                );
            }
        }
    }
    finish(errors)
}

fn validate_lexeme_declaration_shape(
    lexeme: &crate::model::Lexeme,
    errors: &mut Option<syn::Error>,
) {
    let mut members = HashSet::new();
    for member in &lexeme.members {
        let member_name = identifier_key(&member.name);
        if !members.insert(member_name.clone()) {
            combine(
                errors,
                syn::Error::new(
                    member.name.span(),
                    format!("duplicate lexeme member `{member_name}`"),
                ),
            );
        }
        if member.lemma.value().is_empty() {
            combine(
                errors,
                syn::Error::new(member.lemma.span(), "lexeme lemma must not be empty"),
            );
        }
        let mut overrides = HashSet::new();
        for row in &member.overrides {
            let feature = identifier_key(&row.feature);
            if !overrides.insert(feature.clone()) {
                combine(
                    errors,
                    syn::Error::new(
                        row.feature.span(),
                        format!("duplicate override `{feature}`"),
                    ),
                );
            }
            if row.surface.value().is_empty() {
                combine(
                    errors,
                    syn::Error::new(
                        row.surface.span(),
                        "lexeme override surface must not be empty",
                    ),
                );
            }
        }
    }
}

fn validate_generated_identities(raw: &Declarations) -> syn::Result<()> {
    let mut errors = None;
    for declaration in &raw.declarations {
        let Declaration::Identity(binding) = declaration else {
            continue;
        };
        let Some(recipe) = &binding.generated_identity else {
            continue;
        };
        match recipe {
            crate::model::GeneratedIdentityRecipe::Unsupported { name } => combine(
                &mut errors,
                syn::Error::new(
                    name.span(),
                    format!("unknown generated identity recipe `{name}`"),
                ),
            ),
            crate::model::GeneratedIdentityRecipe::Context(source) => {
                if source.arms.len() != 2 {
                    combine(
                        &mut errors,
                        syn::Error::new(
                            source.recipe.span(),
                            "context identity requires exactly two arms",
                        ),
                    );
                }
                let mut variants = HashSet::new();
                let mut accessors = HashSet::new();
                for arm in &source.arms {
                    let variant = identifier_key(&arm.variant);
                    let accessor = identifier_key(&arm.accessor);
                    if !variants.insert(variant.clone()) {
                        combine(
                            &mut errors,
                            syn::Error::new(
                                arm.variant.span(),
                                format!("duplicate context identity arm `{}`", arm.variant),
                            ),
                        );
                    }
                    if !accessors.insert(accessor.clone()) {
                        combine(
                            &mut errors,
                            syn::Error::new(
                                arm.accessor.span(),
                                format!("duplicate context identity accessor `{}`", arm.accessor),
                            ),
                        );
                    }
                    let expected_accessor = match variant.as_str() {
                        "Full" => Some("card_name"),
                        "Abbreviated" => Some("abbreviated_card_name"),
                        _ => {
                            combine(
                                &mut errors,
                                syn::Error::new(
                                    arm.variant.span(),
                                    "context identity arms must be `Full` and `Abbreviated`",
                                ),
                            );
                            None
                        }
                    };
                    if !matches!(accessor.as_str(), "card_name" | "abbreviated_card_name") {
                        combine(
                            &mut errors,
                            syn::Error::new(
                                arm.accessor.span(),
                                format!("unknown ParseContext accessor `{}`", arm.accessor),
                            ),
                        );
                    } else if let Some(expected) = expected_accessor
                        && accessor != expected
                    {
                        combine(
                            &mut errors,
                            syn::Error::new(
                                arm.accessor.span(),
                                format!(
                                    "context identity arm `{variant}` requires ParseContext accessor `{expected}`"
                                ),
                            ),
                        );
                    }
                }
                for required in ["Full", "Abbreviated"] {
                    if !variants.contains(required) {
                        combine(
                            &mut errors,
                            syn::Error::new(
                                source.recipe.span(),
                                format!("context identity requires `{required}` arm"),
                            ),
                        );
                    }
                }
                match source.canonical_slots.as_slice() {
                    [] => combine(
                        &mut errors,
                        syn::Error::new(
                            source.recipe.span(),
                            "context identity requires `canonical_on_collision`",
                        ),
                    ),
                    [canonical, rest @ ..] => {
                        for duplicate in rest {
                            combine(
                                &mut errors,
                                syn::Error::new(
                                    duplicate.slot.span(),
                                    "duplicate context identity `canonical_on_collision`",
                                ),
                            );
                        }
                        let canonical_name = identifier_key(&canonical.arm);
                        if !variants.contains(&canonical_name) {
                            combine(
                                &mut errors,
                                syn::Error::new(
                                    canonical.arm.span(),
                                    format!(
                                        "unknown canonical context identity arm `{}`",
                                        canonical.arm
                                    ),
                                ),
                            );
                        } else if canonical_name != "Full" {
                            combine(
                                &mut errors,
                                syn::Error::new(
                                    canonical.arm.span(),
                                    "context identity canonical arm must be `Full`",
                                ),
                            );
                        }
                    }
                }
            }
        }
    }
    finish(errors)
}

fn validate_generated_codecs(raw: &Declarations) -> syn::Result<()> {
    let mut errors = None;
    for declaration in &raw.declarations {
        let Declaration::Codec(binding) = declaration else {
            continue;
        };
        let Some(recipe) = &binding.generated else {
            continue;
        };
        match recipe {
            crate::model::GeneratedCodecRecipe::Unsupported { name } => combine(
                &mut errors,
                syn::Error::new(
                    name.span(),
                    format!("unknown generated codec recipe `{name}`"),
                ),
            ),
            crate::model::GeneratedCodecRecipe::SignedDecimal(source) => {
                match source.magnitude_slots.as_slice() {
                    [] => combine(
                        &mut errors,
                        syn::Error::new(
                            source.recipe.span(),
                            "signed_decimal requires one `magnitude` field",
                        ),
                    ),
                    [slot, rest @ ..] => {
                        if slot.primitive != "u32" {
                            combine(
                                &mut errors,
                                syn::Error::new(
                                    slot.primitive.span(),
                                    "signed_decimal magnitude must be `u32`",
                                ),
                            );
                        }
                        for duplicate in rest {
                            combine(
                                &mut errors,
                                syn::Error::new(
                                    duplicate.slot.span(),
                                    "duplicate signed_decimal field `magnitude`",
                                ),
                            );
                        }
                    }
                }

                match source.sign_type_slots.as_slice() {
                    [] => combine(
                        &mut errors,
                        syn::Error::new(
                            source.recipe.span(),
                            "signed_decimal requires one `sign_type` field",
                        ),
                    ),
                    [sign, rest @ ..] => {
                        for duplicate in rest {
                            combine(
                                &mut errors,
                                syn::Error::new(
                                    duplicate.slot.span(),
                                    "duplicate signed_decimal field `sign_type`",
                                ),
                            );
                        }
                        if identifier_key(&sign.name) == identifier_key(&binding.name) {
                            combine(
                                &mut errors,
                                syn::Error::new(
                                    sign.name.span(),
                                    format!(
                                        "generated signed_decimal type `{}` collides with codec type `{}`",
                                        sign.name, binding.name
                                    ),
                                ),
                            );
                        }
                        let mut seen = HashSet::new();
                        let mut positive = None;
                        let mut negative = None;
                        for role in &sign.roles {
                            let variant = identifier_key(&role.variant);
                            if !seen.insert(variant.clone()) {
                                combine(
                                    &mut errors,
                                    syn::Error::new(
                                        role.variant.span(),
                                        format!(
                                            "duplicate signed_decimal sign role `{}`",
                                            role.variant
                                        ),
                                    ),
                                );
                                continue;
                            }
                            match variant.as_str() {
                                "Positive" => positive = Some(role),
                                "Negative" => negative = Some(role),
                                _ => combine(
                                    &mut errors,
                                    syn::Error::new(
                                        role.variant.span(),
                                        "signed_decimal sign roles must be `Positive` and `Negative`",
                                    ),
                                ),
                            }
                        }
                        if positive.is_none() || negative.is_none() {
                            combine(
                                &mut errors,
                                syn::Error::new(
                                    sign.name.span(),
                                    "signed_decimal sign_type requires `Positive` and `Negative` roles",
                                ),
                            );
                        }
                        if let Some(positive) = positive
                            && !matches!(
                                positive.spelling,
                                crate::model::SignedDecimalSignSpelling::None(_)
                            )
                        {
                            combine(
                                &mut errors,
                                syn::Error::new(
                                    positive.variant.span(),
                                    "signed_decimal positive sign must be `none`",
                                ),
                            );
                        }
                        if let Some(negative) = negative {
                            let valid = valid_negative_sign_spelling(&negative.spelling);
                            if !valid {
                                combine(
                                    &mut errors,
                                    syn::Error::new(
                                        negative.variant.span(),
                                        "signed_decimal negative sign must be one ASCII byte `-`",
                                    ),
                                );
                            }
                        }
                    }
                }
            }
            crate::model::GeneratedCodecRecipe::DeclarationNoun(source) => {
                validate_declaration_noun_source(raw, source, &mut errors);
            }
        }
    }
    finish(errors)
}

fn validate_declaration_noun_source(
    raw: &Declarations,
    source: &crate::model::DeclarationNounSource,
    errors: &mut Option<syn::Error>,
) {
    let closed = validate_single_ident_slot(
        &source.closed_slots,
        &source.recipe,
        "closed",
        "declaration_noun",
        errors,
    );
    let position = validate_single_ident_slot(
        &source.position_slots,
        &source.recipe,
        "position",
        "declaration_noun",
        errors,
    );
    let feature = validate_single_ident_slot(
        &source.feature_slots,
        &source.recipe,
        "feature",
        "declaration_noun",
        errors,
    );
    let kinds = match source.kind_slots.as_slice() {
        [] => {
            combine(
                errors,
                syn::Error::new(
                    source.recipe.span(),
                    "declaration_noun requires one `kinds` field",
                ),
            );
            None
        }
        [slot, rest @ ..] => {
            for duplicate in rest {
                combine(
                    errors,
                    syn::Error::new(
                        duplicate.slot.span(),
                        "duplicate declaration_noun field `kinds`",
                    ),
                );
            }
            Some(slot)
        }
    };

    if let Some(position) = position
        && position != "Noun"
    {
        combine(
            errors,
            syn::Error::new(position.span(), "declaration_noun position must be `Noun`"),
        );
    }
    if let Some(feature) = feature
        && feature != "Number"
    {
        combine(
            errors,
            syn::Error::new(feature.span(), "declaration_noun feature must be `Number`"),
        );
    }
    if let Some(closed) = closed {
        let closed_name = identifier_key(closed);
        let matching = raw
            .declarations
            .iter()
            .find_map(|declaration| match declaration {
                Declaration::Lexeme(lexeme) if identifier_key(&lexeme.name) == closed_name => {
                    Some(lexeme)
                }
                Declaration::Construction(_)
                | Declaration::AbstractProduct(_)
                | Declaration::AbstractSum(_)
                | Declaration::Vocab(_)
                | Declaration::Morphology(_)
                | Declaration::Lexeme(_)
                | Declaration::Codec(_)
                | Declaration::Identity(_)
                | Declaration::Root(_) => None,
            });
        match matching {
            Some(lexeme) if !lexeme.members.is_empty() => {}
            Some(_) => combine(
                errors,
                syn::Error::new(
                    closed.span(),
                    "declaration_noun closed branch must have at least one lexeme member",
                ),
            ),
            None => combine(
                errors,
                syn::Error::new(
                    closed.span(),
                    "declaration_noun closed branch must name a lexeme declaration",
                ),
            ),
        }
    }
    if let Some(kinds) = kinds {
        if kinds.kinds.is_empty() {
            combine(
                errors,
                syn::Error::new(
                    kinds.slot.span(),
                    "declaration_noun kind set cannot be empty",
                ),
            );
        }
        let mut seen = HashSet::new();
        let mut valid_kinds = !kinds.kinds.is_empty();
        for kind in &kinds.kinds {
            let name = identifier_key(kind);
            if !matches!(name.as_str(), "Type" | "Subtype") {
                valid_kinds = false;
                combine(
                    errors,
                    syn::Error::new(
                        kind.span(),
                        "declaration_noun kinds must be `Type` or `Subtype`",
                    ),
                );
            } else if !seen.insert(name.clone()) {
                combine(
                    errors,
                    syn::Error::new(
                        kind.span(),
                        format!("duplicate declaration_noun kind `{name}`"),
                    ),
                );
            }
        }
        if valid_kinds {
            for required in ["Type", "Subtype"] {
                if !seen.contains(required) {
                    combine(
                        errors,
                        syn::Error::new(
                            kinds.slot.span(),
                            format!("declaration_noun requires `{required}` kind"),
                        ),
                    );
                }
            }
        }
    }
}

fn validate_single_ident_slot<'a>(
    slots: &'a [crate::model::GeneratedIdentSlot],
    recipe: &syn::Ident,
    field: &str,
    recipe_name: &str,
    errors: &mut Option<syn::Error>,
) -> Option<&'a syn::Ident> {
    match slots {
        [] => {
            combine(
                errors,
                syn::Error::new(
                    recipe.span(),
                    format!("{recipe_name} requires one `{field}` field"),
                ),
            );
            None
        }
        [slot, rest @ ..] => {
            for duplicate in rest {
                combine(
                    errors,
                    syn::Error::new(
                        duplicate.slot.span(),
                        format!("duplicate {recipe_name} field `{field}`"),
                    ),
                );
            }
            Some(&slot.value)
        }
    }
}

fn valid_negative_sign_spelling(spelling: &crate::model::SignedDecimalSignSpelling) -> bool {
    matches!(
        spelling,
        crate::model::SignedDecimalSignSpelling::Literal(literal)
            if literal.value() == "-" && literal.value().len() == 1
    )
}

fn seal_category_feature_reads(raw: &Declarations) -> HashMap<String, HashSet<Feature>> {
    let categories = raw
        .declarations
        .iter()
        .filter_map(|declaration| match declaration {
            Declaration::Construction(construction) => Some(path_name(&construction.category)),
            Declaration::AbstractProduct(_)
            | Declaration::AbstractSum(_)
            | Declaration::Vocab(_)
            | Declaration::Morphology(_)
            | Declaration::Lexeme(_)
            | Declaration::Codec(_)
            | Declaration::Identity(_)
            | Declaration::Root(_) => None,
        })
        .collect::<HashSet<_>>();
    categories
        .into_iter()
        .filter_map(|category| {
            let reads = [Feature::Agreement, Feature::Number]
                .into_iter()
                .filter(|feature| raw_category_reads_feature(raw, &category, *feature))
                .collect::<HashSet<_>>();
            (!reads.is_empty()).then_some((category, reads))
        })
        .collect()
}

fn validate_generated_owned_paths(raw: &Declarations) -> syn::Result<()> {
    let mut errors = None;
    for declaration in &raw.declarations {
        match declaration {
            Declaration::Construction(construction) => {
                validate_generated_owned_path(&construction.category, &mut errors);
                for field in &construction.element.fields {
                    validate_generated_owned_field_kind(&field.kind, &mut errors);
                }
                for equation in &construction.equations {
                    match &equation.value {
                        ParsedFeatureValue::Constant(path) => {
                            validate_generated_owned_path(path, &mut errors);
                        }
                        ParsedFeatureValue::Match { arms, .. } => {
                            for arm in arms {
                                validate_generated_owned_path(&arm.value, &mut errors);
                            }
                        }
                        ParsedFeatureValue::FromRole(_) => {}
                    }
                }
                for atom in &construction.form.atoms {
                    if let FormAtom::Verb(VerbOperand::Fixed(path)) = atom {
                        validate_generated_owned_path(path, &mut errors);
                    }
                }
            }
            Declaration::Root(root) => {
                validate_generated_owned_path(&root.category, &mut errors);
            }
            Declaration::Codec(binding) | Declaration::Identity(binding) => {
                for call in &binding.traversal.calls {
                    validate_generated_owned_path(&call.callback, &mut errors);
                }
                for branch in &binding.traversal.branches {
                    for arm in &branch.arms {
                        validate_generated_owned_path(&arm.call.callback, &mut errors);
                    }
                }
            }
            Declaration::AbstractProduct(_)
            | Declaration::AbstractSum(_)
            | Declaration::Vocab(_)
            | Declaration::Morphology(_)
            | Declaration::Lexeme(_) => {}
        }
    }
    finish(errors)
}

fn validate_generated_owned_field_kind(kind: &FieldKind, errors: &mut Option<syn::Error>) {
    match kind {
        FieldKind::Category(path) | FieldKind::Lex(path) | FieldKind::Identity(path) => {
            validate_generated_owned_path(path, errors);
        }
        FieldKind::Optional(value) => validate_generated_owned_field_kind(value, errors),
        FieldKind::Sequence { item, surface } => {
            validate_generated_owned_field_kind(item, errors);
            for fixed in surface
                .separator
                .iter()
                .flat_map(|separator| match separator {
                    crate::model::SeparatorSource::Uniform(surface) => vec![surface],
                    crate::model::SeparatorSource::Positional(rows) => {
                        rows.iter().map(|row| &row.surface).collect()
                    }
                })
                .chain(surface.terminator.iter())
            {
                for atom in &fixed.atoms {
                    if let crate::model::FixedSurfaceAtomSource::Lex(path) = atom {
                        validate_generated_owned_path(path, errors);
                    }
                }
            }
        }
    }
}

fn validate_generated_owned_path(path: &syn::Path, errors: &mut Option<syn::Error>) {
    if path
        .segments
        .iter()
        .any(|segment| !matches!(segment.arguments, syn::PathArguments::None))
    {
        combine(
            errors,
            syn::Error::new_spanned(
                path,
                "compiler-generated identity requires a qself-free, non-generic identifier path",
            ),
        );
    }
}

#[allow(
    clippy::too_many_lines,
    reason = "namespace validation accumulates every independent declaration error in source order"
)]
fn validate_namespaces(raw: &Declarations) -> syn::Result<(Symbols, Vec<String>)> {
    let mut errors = None;
    let mut declaration_names = Vec::new();
    let mut source_names: HashMap<String, proc_macro2::Span> = HashMap::new();
    let mut categories = HashSet::new();
    let mut category_variant_order: HashMap<String, Vec<String>> = HashMap::new();
    let mut terminals = HashMap::new();
    let structural_types = raw
        .declarations
        .iter()
        .filter_map(|declaration| match declaration {
            Declaration::AbstractProduct(product) => Some(identifier_key(&product.name)),
            Declaration::AbstractSum(sum) => Some(identifier_key(&sum.name)),
            _ => None,
        })
        .collect();

    for declaration in &raw.declarations {
        match declaration {
            Declaration::Construction(construction) => {
                let name = identifier_key(&construction.name);
                declaration_names.push(name.clone());
                let category = path_name(&construction.category);
                reject_raw_keyword_identifier(
                    &construction.name,
                    "generated category variant and rule identity",
                    &mut errors,
                );
                if let Some(category_ident) = construction.category.segments.last() {
                    reject_raw_keyword_identifier(
                        &category_ident.ident,
                        "generated category type",
                        &mut errors,
                    );
                }
                reject_raw_keyword_identifier(
                    &construction.element.name,
                    "generated element type",
                    &mut errors,
                );
                reject_raw_keyword_identifier(
                    &construction.form.name,
                    "generated form/rule fragment",
                    &mut errors,
                );
                categories.insert(category.clone());
                duplicate_name(&mut source_names, &name, &construction.name, &mut errors);
                let category_variant = pascal_case(&name);
                validate_generated_rust_ident(
                    &category_variant,
                    &format!("category variant for construction `{name}`"),
                    construction.name.span(),
                    &mut errors,
                );
                validate_generated_rust_ident(
                    &pascal_case(&identifier_key(&construction.form.name)),
                    &format!("form/rule fragment for construction `{name}`"),
                    construction.form.name.span(),
                    &mut errors,
                );
                let variants = category_variant_order.entry(category.clone()).or_default();
                if !variants.contains(&category_variant) {
                    variants.push(category_variant);
                }

                let mut fields = HashSet::new();
                for field in &construction.element.fields {
                    let field_key = identifier_key(&field.name);
                    if !fields.insert(field_key) {
                        combine(
                            &mut errors,
                            syn::Error::new(
                                field.name.span(),
                                format!("duplicate field `{}`", field.name),
                            ),
                        );
                    }
                }
            }
            Declaration::Vocab(vocab) => {
                let name = identifier_key(&vocab.name);
                declaration_names.push(name.clone());
                reject_raw_keyword_identifier(&vocab.name, "generated vocab type", &mut errors);
                duplicate_name(&mut source_names, &name, &vocab.name, &mut errors);
                let mut variants = HashSet::new();
                let mut variant_order = Vec::new();
                let mut words = HashSet::new();
                for variant in &vocab.variants {
                    let variant_key = identifier_key(&variant.name);
                    reject_raw_keyword_identifier(
                        &variant.name,
                        "generated vocab variant",
                        &mut errors,
                    );
                    validate_generated_rust_ident(
                        &variant_key,
                        &format!("generated vocab variant for `{name}`"),
                        variant.name.span(),
                        &mut errors,
                    );
                    if variants.insert(variant_key.clone()) {
                        variant_order.push(variant_key);
                    } else {
                        combine(
                            &mut errors,
                            syn::Error::new(
                                variant.name.span(),
                                format!("duplicate variant `{}`", variant.name),
                            ),
                        );
                    }
                    let word = variant.word.value();
                    if word.is_empty() {
                        combine(
                            &mut errors,
                            syn::Error::new(
                                variant.word.span(),
                                "vocab spelling must not be empty",
                            ),
                        );
                    }
                    if !words.insert(word.clone()) {
                        combine(
                            &mut errors,
                            syn::Error::new(
                                variant.word.span(),
                                format!("duplicate word `{word}`"),
                            ),
                        );
                    }
                }
                terminals.entry(name).or_insert(TerminalInfo {
                    kind: TerminalKind::Vocab,
                    codec_atom: None,
                    variants,
                    variant_order,
                });
            }
            Declaration::Morphology(morphology) => {
                let name = identifier_key(&morphology.name);
                declaration_names.push(name.clone());
                duplicate_name(&mut source_names, &name, &morphology.name, &mut errors);
            }
            Declaration::Lexeme(lexeme) => {
                let name = identifier_key(&lexeme.name);
                declaration_names.push(name.clone());
                reject_raw_keyword_identifier(&lexeme.name, "generated lexeme type", &mut errors);
                duplicate_name(&mut source_names, &name, &lexeme.name, &mut errors);
                let mut variants = HashSet::new();
                let mut variant_order = Vec::new();
                for member in &lexeme.members {
                    let variant = &member.name;
                    let variant_key = identifier_key(variant);
                    reject_raw_keyword_identifier(variant, "generated lexeme variant", &mut errors);
                    validate_generated_rust_ident(
                        &variant_key,
                        &format!("generated lexeme variant for `{name}`"),
                        variant.span(),
                        &mut errors,
                    );
                    if variants.insert(variant_key.clone()) {
                        variant_order.push(variant_key);
                    } else {
                        combine(
                            &mut errors,
                            syn::Error::new(
                                variant.span(),
                                format!("duplicate variant `{variant}`"),
                            ),
                        );
                    }
                }
                terminals.entry(name).or_insert(TerminalInfo {
                    kind: TerminalKind::Lexeme,
                    codec_atom: None,
                    variants,
                    variant_order,
                });
            }
            Declaration::Codec(binding) | Declaration::Identity(binding) => {
                let name = identifier_key(&binding.name);
                declaration_names.push(name.clone());
                let binding_kind =
                    if matches!(declaration, Declaration::Codec(_)) { "codec" } else { "identity" };
                reject_raw_keyword_identifier(
                    &binding.name,
                    &format!("generated {binding_kind} binding type"),
                    &mut errors,
                );
                duplicate_name(&mut source_names, &name, &binding.name, &mut errors);
                let kind = if matches!(declaration, Declaration::Codec(_)) {
                    TerminalKind::Codec
                } else {
                    TerminalKind::Identity
                };
                match declaration {
                    Declaration::Identity(_) if binding.codec_atom.is_some() => combine(
                        &mut errors,
                        syn::Error::new(
                            binding.name.span(),
                            "identity binding cannot claim a codec atom class",
                        ),
                    ),
                    _ => {}
                }
                terminals.entry(name).or_insert(TerminalInfo {
                    kind,
                    codec_atom: binding.codec_atom,
                    variants: HashSet::new(),
                    variant_order: Vec::new(),
                });
            }
            Declaration::AbstractProduct(product) => {
                let name = identifier_key(&product.name);
                declaration_names.push(name.clone());
                reject_raw_keyword_identifier(
                    &product.name,
                    "generated abstract product type",
                    &mut errors,
                );
                duplicate_name(&mut source_names, &name, &product.name, &mut errors);
                let mut fields = HashSet::new();
                for field in &product.fields {
                    let role = identifier_key(&field.name);
                    if !fields.insert(role) {
                        combine(
                            &mut errors,
                            syn::Error::new(
                                field.name.span(),
                                format!("duplicate field `{}`", field.name),
                            ),
                        );
                    }
                }
            }
            Declaration::AbstractSum(sum) => {
                let name = identifier_key(&sum.name);
                declaration_names.push(name.clone());
                reject_raw_keyword_identifier(
                    &sum.name,
                    "generated abstract sum type",
                    &mut errors,
                );
                duplicate_name(&mut source_names, &name, &sum.name, &mut errors);
                let mut alternatives = HashSet::new();
                for alternative in &sum.alternatives {
                    let role = identifier_key(&alternative.name);
                    if !alternatives.insert(role) {
                        combine(
                            &mut errors,
                            syn::Error::new(
                                alternative.name.span(),
                                format!("duplicate alternative `{}`", alternative.name),
                            ),
                        );
                    }
                }
            }
            Declaration::Root(root) => declaration_names.push(path_name(&root.category)),
        }
    }

    validate_generated_name_inventory(raw, &mut errors);
    finish(errors)?;
    Ok((
        Symbols {
            categories,
            structural_types,
            category_variant_order,
            terminals,
        },
        declaration_names,
    ))
}

fn duplicate_name(
    names: &mut HashMap<String, proc_macro2::Span>,
    semantic_name: &str,
    authored_name: &syn::Ident,
    errors: &mut Option<syn::Error>,
) {
    if names
        .insert(semantic_name.to_owned(), authored_name.span())
        .is_some()
    {
        combine(
            errors,
            syn::Error::new(
                authored_name.span(),
                format!("duplicate declaration `{authored_name}`"),
            ),
        );
    }
}

fn reject_raw_keyword_identifier(
    authored: &syn::Ident,
    generated_role: &str,
    errors: &mut Option<syn::Error>,
) {
    if is_raw_keyword(authored) {
        combine(
            errors,
            syn::Error::new(
                authored.span(),
                format!(
                    "raw keyword `{authored}` has semantic identity `{}` and is unsupported for {generated_role}",
                    identifier_key(authored)
                ),
            ),
        );
    }
}

#[derive(Default)]
struct GeneratedNameInventory {
    type_names: HashMap<String, String>,
    value_names: HashMap<String, String>,
    visitor_items: HashMap<String, String>,
    terminal_variants: HashMap<String, String>,
    category_variants: HashMap<String, String>,
    rule_variants: HashMap<String, String>,
    rule_id_items: HashMap<String, String>,
}

impl GeneratedNameInventory {
    fn register_type(
        &mut self,
        generated: &str,
        role: &str,
        span: proc_macro2::Span,
        errors: &mut Option<syn::Error>,
    ) {
        register_module_name(&mut self.type_names, "type", generated, role, span, errors);
    }

    fn register_value(
        &mut self,
        generated: &str,
        role: &str,
        span: proc_macro2::Span,
        errors: &mut Option<syn::Error>,
    ) {
        register_module_name(
            &mut self.value_names,
            "value",
            generated,
            role,
            span,
            errors,
        );
    }

    fn register_visitor_item(
        &mut self,
        generated: &str,
        role: &str,
        span: proc_macro2::Span,
        errors: &mut Option<syn::Error>,
    ) {
        let semantic_generated = spelling_key(generated);
        if role.starts_with("declared traversal callback")
            && self
                .visitor_items
                .get(&semantic_generated)
                .is_some_and(|previous| previous.starts_with("declared traversal callback"))
        {
            return;
        }
        register_module_name(
            &mut self.visitor_items,
            "Visitor item",
            generated,
            role,
            span,
            errors,
        );
    }

    fn register_category_variant(
        &mut self,
        category: &str,
        variant: &str,
        role: &str,
        span: proc_macro2::Span,
        errors: &mut Option<syn::Error>,
    ) {
        let semantic_identity = format!("{category}::{variant}");
        register_associated_name(
            &mut self.category_variants,
            "category variant",
            &semantic_identity,
            role,
            span,
            errors,
        );
    }

    fn register_terminal_variant(
        &mut self,
        generated: &str,
        role: &str,
        span: proc_macro2::Span,
        errors: &mut Option<syn::Error>,
    ) {
        validate_generated_rust_ident(generated, role, span, errors);
        register_associated_name(
            &mut self.terminal_variants,
            "Lexical/Leaf/TerminalClass variant",
            &spelling_key(generated),
            role,
            span,
            errors,
        );
    }

    fn register_rule_variant(
        &mut self,
        generated: &str,
        role: &str,
        span: proc_macro2::Span,
        errors: &mut Option<syn::Error>,
    ) {
        validate_generated_rust_ident(generated, role, span, errors);
        register_associated_name(
            &mut self.rule_variants,
            "Construction/RuleId variant",
            generated,
            role,
            span,
            errors,
        );
        register_associated_name(
            &mut self.rule_id_items,
            "RuleId associated item",
            generated,
            role,
            span,
            errors,
        );
    }
}

fn register_module_name(
    names: &mut HashMap<String, String>,
    namespace: &str,
    generated: &str,
    role: &str,
    span: proc_macro2::Span,
    errors: &mut Option<syn::Error>,
) {
    validate_generated_rust_ident(generated, role, span, errors);
    let semantic_generated = spelling_key(generated);
    if let Some(previous) = names.get(&semantic_generated) {
        if previous != role {
            combine(
                errors,
                syn::Error::new(
                    span,
                    format!(
                        "generated Rust {namespace} name `{generated}` collides: semantic identity `{semantic_generated}` is shared by {previous} and {role}"
                    ),
                ),
            );
        }
    } else {
        names.insert(semantic_generated, role.to_owned());
    }
}

fn register_associated_name(
    names: &mut HashMap<String, String>,
    namespace: &str,
    semantic_identity: &str,
    role: &str,
    span: proc_macro2::Span,
    errors: &mut Option<syn::Error>,
) {
    if let Some(previous) = names.get(semantic_identity) {
        if previous != role {
            combine(
                errors,
                syn::Error::new(
                    span,
                    format!(
                        "generated Rust {namespace} `{semantic_identity}` collides: semantic identity `{semantic_identity}` is shared by {previous} and {role}"
                    ),
                ),
            );
        }
    } else {
        names.insert(semantic_identity.to_owned(), role.to_owned());
    }
}

fn validate_generated_name_inventory(raw: &Declarations, errors: &mut Option<syn::Error>) {
    let _ = generated_name_inventory(raw, errors);
}

#[allow(
    clippy::too_many_lines,
    reason = "the sealed source-ordered inventory enumerates every emitted Rust namespace"
)]
fn generated_name_inventory(
    raw: &Declarations,
    errors: &mut Option<syn::Error>,
) -> GeneratedNameInventory {
    let mut names = GeneratedNameInventory::default();
    let fixed_span = proc_macro2::Span::call_site();
    for (name, role) in [
        (VISITOR_TRAIT, "fixed generated visitor trait"),
        (RULE_CATEGORY_TYPE, "fixed generated rules category type"),
        (
            RULE_CONSTRUCTION_TYPE,
            "fixed generated rules construction type",
        ),
        (RULE_ID_TYPE, "fixed generated rules rule-id type"),
    ] {
        names.register_type(name, role, fixed_span, errors);
    }
    for name in FIXED_RUNTIME_TYPE_NAMES {
        names.register_type(name, "fixed generated runtime type", fixed_span, errors);
    }
    for (name, role) in [
        (RULES_CONSTANT, "fixed generated rules table constant"),
        (BUILD_FUNCTION, "fixed generated build function"),
        (
            CHECKED_BUILD_FUNCTION,
            "fixed generated checked-build function",
        ),
        (
            SEQUENCE_SEPARATOR_FUNCTION,
            "fixed generated structural separator lookup",
        ),
        (
            SEQUENCE_TERMINATOR_FUNCTION,
            "fixed generated structural terminator lookup",
        ),
    ] {
        names.register_value(name, role, fixed_span, errors);
    }
    for (name, role) in [
        (RULE_ID_COUNT, "fixed generated RuleId test count"),
        (
            RULE_ID_PUBLIC_CONSTRUCTION,
            "fixed generated RuleId public-construction accessor",
        ),
        (RULE_ID_INDEX, "fixed generated RuleId index accessor"),
    ] {
        register_associated_name(
            &mut names.rule_id_items,
            "RuleId associated item",
            name,
            role,
            fixed_span,
            errors,
        );
    }
    for (name, role) in [
        ("Literal", "fixed literal terminal variant"),
        ("EndOfInput", "fixed end-of-input terminal variant"),
        ("Declaration", "fixed open-declaration terminal variant"),
    ] {
        names.register_terminal_variant(name, role, fixed_span, errors);
    }

    let verb_lexeme_provider = raw.declarations.iter().find_map(|declaration| {
        let Declaration::Lexeme(lexeme) = declaration else {
            return None;
        };
        (lexeme_recipe(raw, lexeme) == Some(crate::morphology::MorphologyRecipe::EnglishVerb))
            .then(|| identifier_key(&lexeme.name))
    });
    if let Some(provider) = &verb_lexeme_provider {
        let span = raw
            .declarations
            .iter()
            .find_map(|declaration| match declaration {
                Declaration::Lexeme(lexeme) if identifier_key(&lexeme.name) == *provider => {
                    Some(lexeme.name.span())
                }
                _ => None,
            })
            .unwrap_or(fixed_span);
        names.register_terminal_variant(
            "Verb",
            "generated verb lexical/leaf variant",
            span,
            errors,
        );
        names.register_terminal_variant(
            "VerbLexeme",
            "generated verb terminal-class variant",
            span,
            errors,
        );
    }
    let has_noun_terminal = raw
        .declarations
        .iter()
        .any(|declaration| match declaration {
            Declaration::Lexeme(lexeme) => {
                lexeme_recipe(raw, lexeme) == Some(crate::morphology::MorphologyRecipe::EnglishNoun)
            }
            Declaration::Codec(binding) => {
                binding.codec_atom == Some(CodecAtomClass::Noun)
                    && binding.lexical_variant.is_some()
            }
            _ => false,
        });
    if has_noun_terminal {
        names.register_terminal_variant(
            "Noun",
            "generated noun terminal variant",
            fixed_span,
            errors,
        );
    }

    let nested_categories = raw
        .declarations
        .iter()
        .filter_map(|declaration| match declaration {
            Declaration::Construction(construction) => Some(&construction.element.fields),
            _ => None,
        })
        .flatten()
        .filter_map(|field| match &field.kind {
            FieldKind::Category(path) => Some(path_name(path)),
            FieldKind::Lex(_)
            | FieldKind::Identity(_)
            | FieldKind::Optional(_)
            | FieldKind::Sequence { .. } => None,
        })
        .collect::<HashSet<_>>();
    let standalone_roots = raw
        .declarations
        .iter()
        .filter_map(|declaration| match declaration {
            Declaration::Root(root) if root.standalone_render => Some(path_name(&root.category)),
            _ => None,
        })
        .collect::<HashSet<_>>();
    let mut seen_categories = HashSet::new();

    for declaration in &raw.declarations {
        match declaration {
            Declaration::Construction(construction) => {
                let construction_name = identifier_key(&construction.name);
                let category = path_name(&construction.category);
                if seen_categories.insert(category.clone()) {
                    let category_span = construction.category.span();
                    names.register_type(
                        &category,
                        &format!("generated category type for `{category}`"),
                        category_span,
                        errors,
                    );
                    let standalone_root = standalone_roots.contains(&category);
                    if !standalone_root || nested_categories.contains(&category) {
                        let renderer = category_renderer(&category, standalone_root);
                        let role = if standalone_root {
                            format!("generated nested-root category renderer for `{category}`")
                        } else {
                            format!("generated category renderer for `{category}`")
                        };
                        names.register_value(&renderer, &role, category_span, errors);
                    }
                    names.register_value(
                        &prefixed("walk_", &category),
                        &format!("generated category walker for `{category}`"),
                        category_span,
                        errors,
                    );
                    names.register_visitor_item(
                        &prefixed("visit_", &category),
                        &format!("generated category callback for `{category}`"),
                        category_span,
                        errors,
                    );
                    for (feature, spelling, display) in [
                        (Feature::Agreement, "agreement", "Agreement"),
                        (Feature::Number, "number", "Number"),
                    ] {
                        if raw_category_reads_feature(raw, &category, feature) {
                            names.register_value(
                                &feature_helper(spelling, &category),
                                &format!("generated {display} helper for category `{category}`"),
                                category_span,
                                errors,
                            );
                        }
                    }
                }

                let element = identifier_key(&construction.element.name);
                let element_span = construction.element.name.span();
                names.register_type(
                    &element,
                    &format!("generated element type for `{construction_name}`"),
                    element_span,
                    errors,
                );
                if construction.element.fields.is_empty() {
                    names.register_value(
                        &element,
                        &format!("generated unit element constructor for `{construction_name}`"),
                        element_span,
                        errors,
                    );
                }
                names.register_value(
                    &prefixed("walk_", &element),
                    &format!("generated construction walker for `{construction_name}`"),
                    element_span,
                    errors,
                );
                names.register_visitor_item(
                    &prefixed("visit_", &element),
                    &format!("generated construction callback for `{construction_name}`"),
                    element_span,
                    errors,
                );
                register_structural_field_names(
                    &mut names,
                    &element,
                    &construction.element.fields,
                    &construction.requirements,
                    errors,
                );

                let category_variant = pascal_case(&construction_name);
                names.register_category_variant(
                    &category,
                    &category_variant,
                    &format!("generated category variants for construction `{construction_name}`"),
                    construction.name.span(),
                    errors,
                );
                register_structural_owner_rule_names(
                    &mut names,
                    &format!("{}{category_variant}", pascal_case(&category)),
                    &element,
                    &construction.element.fields,
                    &construction.requirements,
                    &format!("RuleId for construction {construction_name}"),
                    construction.name.span(),
                    errors,
                );
                for equation in &construction.equations {
                    let ParsedFeatureValue::Match { role, .. } = &equation.value else {
                        continue;
                    };
                    let feature = match &equation.target {
                        ParsedFeaturePlace::Construction(feature)
                        | ParsedFeaturePlace::Role { feature, .. } => feature,
                    };
                    let Some(vocab) = construction.element.fields.iter().find_map(|field| {
                        (identifier_key(&field.name) == identifier_key(role))
                            .then_some(&field.kind)
                            .and_then(|kind| match kind {
                                FieldKind::Lex(path) => Some(path_name(path)),
                                FieldKind::Category(_)
                                | FieldKind::Identity(_)
                                | FieldKind::Optional(_)
                                | FieldKind::Sequence { .. } => None,
                            })
                    }) else {
                        continue;
                    };
                    let (spelling, display) = match feature {
                        ParsedFeature::Agreement => ("agreement", "Agreement"),
                        ParsedFeature::Number => ("number", "Number"),
                    };
                    names.register_value(
                        &feature_helper(spelling, &vocab),
                        &format!("generated {display} helper for vocab `{vocab}`"),
                        role.span(),
                        errors,
                    );
                }
            }
            Declaration::Vocab(vocab) => {
                let name = identifier_key(&vocab.name);
                register_terminal_names(
                    &mut names,
                    &name,
                    "vocab",
                    vocab.name.span(),
                    true,
                    errors,
                );
                names.register_terminal_variant(
                    &name,
                    &format!("generated vocab terminal variant for `{name}`"),
                    vocab.name.span(),
                    errors,
                );
            }
            Declaration::Lexeme(lexeme) => {
                let name = identifier_key(&lexeme.name);
                register_terminal_names(
                    &mut names,
                    &name,
                    "lexeme",
                    lexeme.name.span(),
                    false,
                    errors,
                );
                names.register_value(
                    &lexeme_surface_helper(&name),
                    &format!("generated lexeme surface helper for `{name}`"),
                    lexeme.name.span(),
                    errors,
                );
            }
            Declaration::Codec(binding) | Declaration::Identity(binding) => {
                let kind =
                    if matches!(declaration, Declaration::Codec(_)) { "codec" } else { "identity" };
                let name = identifier_key(&binding.name);
                register_terminal_names(
                    &mut names,
                    &name,
                    kind,
                    binding.name.span(),
                    false,
                    errors,
                );
                if let Some(crate::model::GeneratedCodecRecipe::SignedDecimal(source)) =
                    &binding.generated
                    && let Some(sign) = source.sign_type_slots.first()
                {
                    register_terminal_names(
                        &mut names,
                        &identifier_key(&sign.name),
                        "signed_decimal sign",
                        sign.name.span(),
                        false,
                        errors,
                    );
                }
                if matches!(
                    binding.generated,
                    Some(crate::model::GeneratedCodecRecipe::DeclarationNoun(_))
                ) {
                    let open_value = format!("Declaration{}", identifier_key(&binding.name));
                    register_terminal_names(
                        &mut names,
                        &open_value,
                        "declaration_noun open value",
                        binding.name.span(),
                        false,
                        errors,
                    );
                }
                match (&binding.generated, &binding.generated_identity) {
                    (Some(crate::model::GeneratedCodecRecipe::SignedDecimal(_)), None) => {
                        names.register_terminal_variant(
                            &name,
                            &format!("generated signed_decimal terminal variant for `{name}`"),
                            binding.name.span(),
                            errors,
                        );
                    }
                    (None, Some(crate::model::GeneratedIdentityRecipe::Context(_))) => {
                        let aggregate = name.strip_suffix("Spelling").unwrap_or(&name);
                        names.register_terminal_variant(
                            aggregate,
                            &format!("generated context identity terminal variant for `{name}`"),
                            binding.name.span(),
                            errors,
                        );
                    }
                    (None, None) if binding.codec_atom != Some(CodecAtomClass::Noun) => {
                        if let Some(variant) = binding.lexical_variant.as_ref().and_then(|path| {
                            path.segments
                                .last()
                                .map(|segment| identifier_key(&segment.ident))
                        }) {
                            names.register_terminal_variant(
                                &variant,
                                &format!("generated {kind} terminal variant for `{name}`"),
                                binding.name.span(),
                                errors,
                            );
                        }
                    }
                    _ => {}
                }
                for leaf in &binding.traversal.leaf_callbacks {
                    names.register_visitor_item(
                        &identifier_key(&leaf.name),
                        &format!("declared traversal callback for `{name}`"),
                        leaf.name.span(),
                        errors,
                    );
                }
            }
            Declaration::AbstractProduct(product) => {
                let owner = identifier_key(&product.name);
                names.register_value(
                    &prefixed("render_", &owner),
                    &format!("generated abstract product renderer for `{owner}`"),
                    product.name.span(),
                    errors,
                );
                names.register_value(
                    &prefixed("walk_", &owner),
                    &format!("generated abstract product walker for `{owner}`"),
                    product.name.span(),
                    errors,
                );
                names.register_visitor_item(
                    &prefixed("visit_", &owner),
                    &format!("generated abstract product visitor callback for `{owner}`"),
                    product.name.span(),
                    errors,
                );
                names.register_type(
                    &owner,
                    &format!("generated abstract product type for `{owner}`"),
                    product.name.span(),
                    errors,
                );
                register_structural_field_names(
                    &mut names,
                    &owner,
                    &product.fields,
                    &product.requirements,
                    errors,
                );
                register_structural_owner_rule_names(
                    &mut names,
                    &format!("{}Product", pascal_case(&owner)),
                    &owner,
                    &product.fields,
                    &product.requirements,
                    &format!("generated structural product RuleId for `{owner}`"),
                    product.name.span(),
                    errors,
                );
            }
            Declaration::AbstractSum(sum) => {
                let owner = identifier_key(&sum.name);
                names.register_value(
                    &prefixed("render_", &owner),
                    &format!("generated abstract sum renderer for `{owner}`"),
                    sum.name.span(),
                    errors,
                );
                names.register_value(
                    &prefixed("walk_", &owner),
                    &format!("generated abstract sum walker for `{owner}`"),
                    sum.name.span(),
                    errors,
                );
                names.register_visitor_item(
                    &prefixed("visit_", &owner),
                    &format!("generated abstract sum visitor callback for `{owner}`"),
                    sum.name.span(),
                    errors,
                );
                names.register_type(
                    &owner,
                    &format!("generated abstract sum type for `{owner}`"),
                    sum.name.span(),
                    errors,
                );
                for alternative in &sum.alternatives {
                    names.register_rule_variant(
                        &format!(
                            "{}{}",
                            pascal_case(&owner),
                            pascal_case(&identifier_key(&alternative.name))
                        ),
                        &format!(
                            "generated structural sum RuleId for `{}.{}`",
                            owner,
                            identifier_key(&alternative.name)
                        ),
                        alternative.name.span(),
                        errors,
                    );
                }
            }
            Declaration::Root(_) | Declaration::Morphology(_) => {}
        }
    }
    names
}

fn register_structural_field_names(
    names: &mut GeneratedNameInventory,
    owner: &str,
    fields: &[crate::model::Field],
    requirements: &[RequireExprSource],
    errors: &mut Option<syn::Error>,
) {
    let bounds = inventory_length_bounds(owner, fields, requirements);
    for field in fields {
        let role = identifier_key(&field.name);
        let collision_role = format!("{owner}.{role}: generated helper name collision");
        match &field.kind {
            FieldKind::Optional(_) => {
                let aggregate = format!("{}{}Optional", pascal_case(owner), pascal_case(&role));
                names.register_rule_variant(
                    &format!("{aggregate}Absent"),
                    &format!("generated structural optional RuleId for `{owner}.{role}`"),
                    field.name.span(),
                    errors,
                );
                names.register_rule_variant(
                    &format!("{aggregate}Present"),
                    &format!("generated structural optional RuleId for `{owner}.{role}`"),
                    field.name.span(),
                    errors,
                );
            }
            FieldKind::Sequence { surface, .. } => {
                let aggregate = structural_sequence_aggregate(owner, &role);
                for generated in [aggregate.clone(), structural_sequence_rule(owner, &role)] {
                    names.register_type(&generated, &collision_role, field.name.span(), errors);
                }
                for generated in [
                    structural_sequence_builder(owner, &role),
                    structural_sequence_renderer(owner, &role),
                    structural_sequence_walker(owner, &role),
                ] {
                    names.register_value(&generated, &collision_role, field.name.span(), errors);
                }
                names.register_visitor_item(
                    &prefixed("visit_", &format!("{owner}_{role}_sequence")),
                    &collision_role,
                    field.name.span(),
                    errors,
                );
                let field_bounds = bounds
                    .get(&role)
                    .copied()
                    .unwrap_or_else(|| LengthBounds::new(0, None));
                let style = if matches!(
                    &surface.separator,
                    Some(crate::model::SeparatorSource::Positional(_))
                ) {
                    StructuralSequenceStyle::Positional
                } else {
                    StructuralSequenceStyle::Uniform
                };
                for (_, generated) in
                    structural_sequence_helper_categories(owner, &role, field_bounds.max(), style)
                {
                    names.register_type(&generated, &collision_role, field.name.span(), errors);
                }
                for suffix in sequence_helper_rule_suffixes(surface, field_bounds) {
                    names.register_rule_variant(
                        &format!("{aggregate}{suffix}"),
                        &format!("generated structural sequence RuleId for `{owner}.{role}`"),
                        field.name.span(),
                        errors,
                    );
                }
            }
            FieldKind::Category(_) | FieldKind::Lex(_) | FieldKind::Identity(_) => {}
        }
    }
}

fn inventory_length_bounds(
    owner: &str,
    fields: &[crate::model::Field],
    requirements: &[RequireExprSource],
) -> HashMap<String, LengthBounds> {
    let mut ignored_errors = None;
    normalize_length_requirements(owner, fields, requirements, &mut ignored_errors)
}

fn sequence_helper_rule_suffixes(
    surface: &crate::model::SequenceSurfaceSource,
    bounds: LengthBounds,
) -> Vec<String> {
    match &surface.separator {
        Some(crate::model::SeparatorSource::Positional(_)) => {
            if let Some(maximum) = bounds.max() {
                let mut suffixes = Vec::new();
                for position in 2..maximum {
                    let total = position + 1;
                    if bounds.allows(total) {
                        suffixes.push(format!("Count{position}Last"));
                    }
                    if total < maximum {
                        suffixes.push(format!("Count{position}Middle"));
                    }
                }
                return suffixes;
            }
            let tail_length = bounds.min().max(3) - 1;
            vec![
                if tail_length == 2 {
                    "Last".to_owned()
                } else {
                    format!("TailLength{tail_length}")
                },
                "Middle".to_owned(),
            ]
        }
        Some(crate::model::SeparatorSource::Uniform(_)) | None => {
            if let Some(maximum) = bounds.max() {
                let mut suffixes = Vec::new();
                for count in 1..=maximum {
                    if bounds.allows(count) {
                        suffixes.push(format!("Count{count}Final"));
                    }
                    if count < maximum {
                        suffixes.push(format!("Count{count}Continue"));
                    }
                }
                return suffixes;
            }
            vec![
                if bounds.min() <= 1 {
                    "Singleton".to_owned()
                } else {
                    format!("Length{}", bounds.min())
                },
                "Recursive".to_owned(),
            ]
        }
    }
}

#[expect(
    clippy::too_many_arguments,
    reason = "owner rule inventory registration carries one complete diagnostic identity"
)]
fn register_structural_owner_rule_names(
    names: &mut GeneratedNameInventory,
    base: &str,
    owner: &str,
    fields: &[crate::model::Field],
    requirements: &[RequireExprSource],
    base_role: &str,
    span: proc_macro2::Span,
    errors: &mut Option<syn::Error>,
) {
    let bounds = inventory_length_bounds(owner, fields, requirements);
    let state_fields = fields
        .iter()
        .filter_map(|field| {
            let FieldKind::Sequence { surface, .. } = &field.kind else {
                return None;
            };
            let role = identifier_key(&field.name);
            let field_bounds = bounds
                .get(&role)
                .copied()
                .unwrap_or_else(|| LengthBounds::new(0, None));
            let suffixes = sequence_owner_rule_suffixes(surface, field_bounds);
            (!suffixes.is_empty()).then(|| {
                (
                    role.clone(),
                    structural_sequence_aggregate(owner, &role),
                    suffixes,
                )
            })
        })
        .collect::<Vec<_>>();

    let variants = owner_rule_variant_names(base, &state_fields);
    let role = if state_fields.is_empty() {
        base_role.to_owned()
    } else {
        format!("generated structural owner-state RuleId for `{owner}`")
    };
    for variant in variants {
        names.register_rule_variant(&variant, &role, span, errors);
    }
}

fn sequence_owner_rule_suffixes(
    surface: &crate::model::SequenceSurfaceSource,
    bounds: LengthBounds,
) -> Vec<String> {
    match &surface.separator {
        Some(crate::model::SeparatorSource::Positional(_)) => {
            if let Some(maximum) = bounds.max() {
                let mut suffixes = Vec::new();
                for length in 0..3 {
                    if bounds.allows(length) {
                        suffixes.push(sequence_exact_owner_suffix(length));
                    }
                }
                let minimum = bounds.min().max(3);
                if maximum >= minimum {
                    suffixes.push(if minimum == 3 {
                        "ThreePlus".to_owned()
                    } else {
                        format!("Minimum{minimum}Plus")
                    });
                }
                return suffixes;
            }
            let mut suffixes = Vec::new();
            for length in 0..3 {
                if bounds.allows(length) {
                    suffixes.push(sequence_exact_owner_suffix(length));
                }
            }
            let minimum = bounds.min().max(3);
            suffixes.push(if minimum == 3 {
                "ThreePlus".to_owned()
            } else {
                format!("Minimum{minimum}Plus")
            });
            suffixes
        }
        Some(crate::model::SeparatorSource::Uniform(_)) | None => match bounds.max() {
            None if bounds.min() == 0 => vec!["Empty".to_owned(), "NonEmpty".to_owned()],
            None => Vec::new(),
            Some(maximum) => {
                let mut suffixes = Vec::new();
                if bounds.allows(0) {
                    suffixes.push("Empty".to_owned());
                }
                if maximum >= bounds.min().max(1) {
                    suffixes.push("NonEmpty".to_owned());
                }
                suffixes
            }
        },
    }
}

fn sequence_exact_owner_suffix(length: usize) -> String {
    match length {
        0 => "Empty".to_owned(),
        1 => "Singleton".to_owned(),
        2 => "Pair".to_owned(),
        length => format!("Length{length}"),
    }
}

fn owner_rule_variant_names(
    base: &str,
    state_fields: &[(String, String, Vec<String>)],
) -> Vec<String> {
    if state_fields.is_empty() {
        return vec![base.to_owned()];
    }
    if let [(.., aggregate, suffixes)] = state_fields {
        return suffixes
            .iter()
            .map(|suffix| format!("{aggregate}{suffix}"))
            .collect();
    }
    state_fields
        .iter()
        .fold(vec![base.to_owned()], |variants, (role, _, suffixes)| {
            variants
                .into_iter()
                .flat_map(|variant| {
                    suffixes
                        .iter()
                        .map(move |suffix| format!("{variant}{}{suffix}", pascal_case(role)))
                })
                .collect()
        })
}

fn raw_category_reads_feature(raw: &Declarations, category: &str, feature: Feature) -> bool {
    let parsed_feature = match feature {
        Feature::Agreement => ParsedFeature::Agreement,
        Feature::Number => ParsedFeature::Number,
    };
    raw.declarations.iter().any(|declaration| {
        let Declaration::Construction(construction) = declaration else { return false };
        construction.equations.iter().any(|equation| {
            matches!(&equation.value, ParsedFeatureValue::FromRole(slot) if slot.feature == parsed_feature
                && construction.element.fields.iter().any(|field| identifier_key(&field.name) == identifier_key(&slot.role) && matches!(&field.kind, FieldKind::Category(path) if path_name(path) == category))
                && !construction.equations.iter().any(|writer| matches!(&writer.target, ParsedFeaturePlace::Role { field, feature: writer_feature } if identifier_key(field) == identifier_key(&slot.role) && *writer_feature == parsed_feature)))
        }) || (feature == Feature::Number
            && construction
                .form
                .atoms
                .iter()
                .any(|atom| matches!(atom, FormAtom::Noun(_)))
            && path_name(&construction.category) == category)
    })
}

fn register_terminal_names(
    names: &mut GeneratedNameInventory,
    name: &str,
    kind: &str,
    span: proc_macro2::Span,
    has_renderer: bool,
    errors: &mut Option<syn::Error>,
) {
    names.register_type(
        name,
        &format!("generated {kind} type for `{name}`"),
        span,
        errors,
    );
    if has_renderer {
        names.register_value(
            &prefixed("render_", name),
            &format!("generated {kind} renderer for `{name}`"),
            span,
            errors,
        );
    }
    names.register_value(
        &prefixed("walk_", name),
        &format!("generated {kind} walker for `{name}`"),
        span,
        errors,
    );
    names.register_visitor_item(
        &prefixed("visit_", name),
        &format!("generated {kind} callback for `{name}`"),
        span,
        errors,
    );
}

fn validate_generated_rust_ident(
    generated: &str,
    owner: &str,
    span: proc_macro2::Span,
    errors: &mut Option<syn::Error>,
) {
    if syn::parse_str::<syn::Ident>(generated).is_err() {
        combine(
            errors,
            syn::Error::new(
                span,
                format!("generated Rust identifier `{generated}` for {owner} is invalid"),
            ),
        );
    }
}

fn validate_resolution(raw: &Declarations, symbols: &Symbols) -> syn::Result<ResolvedGrammar> {
    let mut errors = None;
    let feature_providers = feature_providers(raw);
    for declaration in &raw.declarations {
        let Declaration::Construction(construction) = declaration else { continue };
        let fields: HashMap<_, _> = construction
            .element
            .fields
            .iter()
            .map(|field| (identifier_key(&field.name), &field.kind))
            .collect();
        let (verb_operands, open_verb_count) = form_verbs(construction);
        let has_fixed_verb = verb_operands
            .iter()
            .any(|operand| matches!(operand, VerbOperand::Fixed(_)))
            || open_verb_count != 0;
        let local_vocab_providers = local_vocab_feature_providers(construction, &fields, symbols);
        if verb_operands.len() + open_verb_count > 1 {
            combine(
                &mut errors,
                syn::Error::new(
                    construction.form.name.span(),
                    "an MVP form may contain only one agreement-bearing verb slot",
                ),
            );
        }
        if has_fixed_verb && fields.contains_key("verb") {
            let span = construction
                .element
                .fields
                .iter()
                .find(|field| identifier_key(&field.name) == "verb")
                .map_or(construction.form.name.span(), |field| field.name.span());
            combine(
                &mut errors,
                syn::Error::new(
                    span,
                    "field `verb` collides with the fixed verb atom's implicit agreement slot",
                ),
            );
        }
        for field in &construction.element.fields {
            validate_resolved_field_kind(&field.kind, symbols, &mut errors);
        }
        for atom in &construction.form.atoms {
            match atom {
                FormAtom::Role(role) => check_role_kind(role, &fields, true, &mut errors),
                FormAtom::Lex(role) => check_lex_role(role, &fields, symbols, &mut errors),
                FormAtom::Identity(role) => match fields
                    .get(&identifier_key(role))
                    .map(|kind| field_kind_leaf(kind))
                {
                    None => combine(
                        &mut errors,
                        syn::Error::new(role.span(), format!("unknown role `{role}`")),
                    ),
                    Some(FieldKind::Identity(_)) => {}
                    Some(_) => combine(
                        &mut errors,
                        syn::Error::new(
                            role.span(),
                            format!("field `{role}` is not an identity role"),
                        ),
                    ),
                },
                FormAtom::Noun(role) => check_noun_role(role, &fields, symbols, &mut errors),
                FormAtom::Verb(VerbOperand::Projected(role)) => {
                    check_verb_role(role, &fields, symbols, &mut errors);
                    reject_projected_verb_role(role, &mut errors);
                }
                FormAtom::Verb(VerbOperand::Fixed(path)) => {
                    check_terminal_variant(path, TerminalKind::Lexeme, symbols, &mut errors);
                }
                FormAtom::OpenVerb(open) => validate_open_declaration(open, &mut errors),
                FormAtom::Literal(_) => {}
            }
        }
        for equation in &construction.equations {
            if let ParsedFeaturePlace::Role { field, feature } = &equation.target {
                check_feature_role(
                    field,
                    *feature,
                    &fields,
                    symbols,
                    &feature_providers,
                    &local_vocab_providers,
                    has_fixed_verb,
                    &verb_operands,
                    &mut errors,
                );
            }
            match &equation.value {
                ParsedFeatureValue::FromRole(slot) => check_feature_role(
                    &slot.role,
                    slot.feature,
                    &fields,
                    symbols,
                    &feature_providers,
                    &local_vocab_providers,
                    has_fixed_verb,
                    &verb_operands,
                    &mut errors,
                ),
                ParsedFeatureValue::Match { role, .. } => {
                    check_vocab_role(role, &fields, symbols, &mut errors);
                }
                ParsedFeatureValue::Constant(_) => {}
            }
        }
    }
    finish(errors)?;
    resolve_grammar_uses(raw)
}

fn validate_resolved_field_kind(
    kind: &FieldKind,
    symbols: &Symbols,
    errors: &mut Option<syn::Error>,
) {
    match kind {
        FieldKind::Category(path) => {
            let name = path_name(path);
            if !symbols.categories.contains(&name) && !symbols.structural_types.contains(&name) {
                combine(
                    errors,
                    syn::Error::new_spanned(path, format!("unknown category `{name}`")),
                );
            }
        }
        FieldKind::Lex(path) | FieldKind::Identity(path) => {
            let name = path_name(path);
            match symbols.terminals.get(&name) {
                None => combine(
                    errors,
                    syn::Error::new_spanned(path, format!("unknown terminal type `{name}`")),
                ),
                Some(info)
                    if matches!(kind, FieldKind::Identity(_))
                        && info.kind != TerminalKind::Identity =>
                {
                    combine(
                        errors,
                        syn::Error::new_spanned(
                            path,
                            format!("terminal `{name}` is not an identity binding"),
                        ),
                    );
                }
                _ => {}
            }
        }
        FieldKind::Optional(value) => validate_resolved_field_kind(value, symbols, errors),
        FieldKind::Sequence { item, .. } => validate_resolved_field_kind(item, symbols, errors),
    }
}

fn validate_open_declaration(
    open: &crate::model::OpenDeclarationAtom,
    errors: &mut Option<syn::Error>,
) {
    if open_declaration_kind(&open.kind).is_none() {
        combine(
            errors,
            syn::Error::new(
                open.kind.span(),
                format!("unsupported open declaration kind `{}`", open.kind),
            ),
        );
    }
    if open.name.value().is_empty() {
        combine(
            errors,
            syn::Error::new(open.name.span(), "open declaration name is empty"),
        );
    }
}

fn form_verbs(construction: &crate::model::Construction) -> (Vec<&VerbOperand>, usize) {
    let verb_operands = construction
        .form
        .atoms
        .iter()
        .filter_map(|atom| match atom {
            FormAtom::Verb(operand) => Some(operand),
            _ => None,
        })
        .collect();
    let open_verb_count = construction
        .form
        .atoms
        .iter()
        .filter(|atom| matches!(atom, FormAtom::OpenVerb(_)))
        .count();
    (verb_operands, open_verb_count)
}

fn reject_projected_verb_role(role: &syn::Ident, errors: &mut Option<syn::Error>) {
    combine(
        errors,
        syn::Error::new(
            role.span(),
            "unimplemented in MVP: `projected verb role`; use a fixed verb path",
        ),
    );
}

fn open_declaration_kind(kind: &syn::Ident) -> Option<macro_ron::v2::DeclarationKind> {
    use macro_ron::v2::DeclarationKind;

    match kind.to_string().as_str() {
        "KeywordAction" => Some(DeclarationKind::KeywordAction),
        "KeywordAbility" => Some(DeclarationKind::KeywordAbility),
        "Type" => Some(DeclarationKind::Type),
        "CounterKind" => Some(DeclarationKind::CounterKind),
        "Designation" => Some(DeclarationKind::Designation),
        _ => None,
    }
}

fn resolve_grammar_uses(raw: &Declarations) -> syn::Result<ResolvedGrammar> {
    let mut errors = None;
    let mut atoms_by_construction = HashMap::new();
    let verb_providers = raw
        .declarations
        .iter()
        .filter_map(|declaration| {
            let Declaration::Lexeme(lexeme) = declaration else {
                return None;
            };
            (lexeme_recipe(raw, lexeme) == Some(crate::morphology::MorphologyRecipe::EnglishVerb))
                .then(|| (identifier_key(&lexeme.name), lexeme.name.span()))
        })
        .collect::<Vec<_>>();

    for declaration in &raw.declarations {
        let Declaration::Construction(construction) = declaration else { continue };
        let fields: HashMap<_, _> = construction
            .element
            .fields
            .iter()
            .map(|field| (identifier_key(&field.name), &field.kind))
            .collect();
        let mut atoms = Vec::new();
        for atom in &construction.form.atoms {
            let resolved = match atom {
                FormAtom::Literal(_) => Some(AtomContribution::Literal),
                FormAtom::Role(role) => fields
                    .get(&identifier_key(role))
                    .map(|kind| field_kind_leaf(kind))
                    .and_then(|kind| match kind {
                        FieldKind::Category(path) => Some(AtomContribution::Category {
                            role: identifier_key(role),
                            category: path_name(path),
                        }),
                        _ => None,
                    }),
                FormAtom::Lex(role) => match fields
                    .get(&identifier_key(role))
                    .map(|kind| field_kind_leaf(kind))
                {
                    Some(FieldKind::Lex(path)) => Some(AtomContribution::Lex {
                        role: identifier_key(role),
                        terminal: path_name(path),
                    }),
                    _ => None,
                },
                FormAtom::Identity(role) => match fields
                    .get(&identifier_key(role))
                    .map(|kind| field_kind_leaf(kind))
                {
                    Some(FieldKind::Identity(path)) => Some(AtomContribution::Identity {
                        role: identifier_key(role),
                        terminal: path_name(path),
                    }),
                    _ => None,
                },
                FormAtom::Noun(role) => match fields.get(&identifier_key(role)) {
                    Some(FieldKind::Lex(path)) => Some(AtomContribution::Noun {
                        role: identifier_key(role),
                        terminal: path_name(path),
                    }),
                    _ => None,
                },
                FormAtom::Verb(VerbOperand::Fixed(path)) => {
                    let segments: Vec<_> = path.segments.iter().collect();
                    let Some(terminal) = segments.iter().rev().nth(1) else { continue };
                    let Some(variant) = segments.last() else { continue };
                    let terminal = identifier_key(&terminal.ident);
                    let variant = identifier_key(&variant.ident);
                    Some(AtomContribution::VerbFixed { terminal, variant })
                }
                FormAtom::OpenVerb(open) => open_declaration_kind(&open.kind).map(|kind| {
                    AtomContribution::OpenDeclaration {
                        kind,
                        name: open.name.value(),
                    }
                }),
                FormAtom::Verb(VerbOperand::Projected(_)) => None,
            };
            if let Some(resolved) = resolved {
                atoms.push(resolved);
            }
        }
        atoms_by_construction.insert(
            identifier_key(&construction.name),
            (construction.name.span(), atoms),
        );
    }

    if let Some((first, _)) = verb_providers.first() {
        for (provider, span) in verb_providers.iter().skip(1) {
            combine(
                &mut errors,
                syn::Error::new(
                    *span,
                    format!(
                        "multiple verb lexeme providers `{first}` and `{provider}`; all verb atoms must use one declared lexeme"
                    ),
                ),
            );
        }
    }
    finish(errors)?;
    Ok(ResolvedGrammar {
        atoms_by_construction,
        verb_lexeme_provider: verb_providers.first().map(|(name, _)| name.clone()),
    })
}

fn lexeme_recipe(
    raw: &Declarations,
    lexeme: &crate::Lexeme,
) -> Option<crate::morphology::MorphologyRecipe> {
    let morphology_name = identifier_key(&lexeme.morphology);
    raw.declarations.iter().find_map(|declaration| {
        let Declaration::Morphology(morphology) = declaration else {
            return None;
        };
        (identifier_key(&morphology.name) == morphology_name)
            .then(|| crate::morphology::MorphologyRecipe::from_ident(&morphology.recipe))
            .flatten()
    })
}

fn check_lex_role(
    role: &syn::Ident,
    fields: &HashMap<String, &FieldKind>,
    symbols: &Symbols,
    errors: &mut Option<syn::Error>,
) {
    check_role_kind(role, fields, false, errors);
    if let Some(FieldKind::Lex(path)) = fields
        .get(&identifier_key(role))
        .map(|kind| field_kind_leaf(kind))
    {
        let terminal = path_name(path);
        match symbols.terminals.get(&terminal).map(|info| info.kind) {
            Some(TerminalKind::Vocab) | None => {}
            Some(TerminalKind::Codec) => {
                if symbols
                    .terminals
                    .get(&terminal)
                    .is_some_and(|info| info.codec_atom != Some(CodecAtomClass::Lex))
                {
                    combine(
                        errors,
                        syn::Error::new(
                            role.span(),
                            format!(
                                "codec `{terminal}` declares `noun` and cannot serve lex atom role `{role}`"
                            ),
                        ),
                    );
                }
            }
            Some(TerminalKind::Identity) => combine(
                errors,
                syn::Error::new(
                    role.span(),
                    format!("lex atom role `{role}` resolves to identity binding `{terminal}`"),
                ),
            ),
            Some(TerminalKind::Lexeme) => combine(
                errors,
                syn::Error::new(
                    role.span(),
                    format!("lex atom role `{role}` resolves to name-only lexeme `{terminal}`"),
                ),
            ),
        }
    }
}

fn check_noun_role(
    role: &syn::Ident,
    fields: &HashMap<String, &FieldKind>,
    symbols: &Symbols,
    errors: &mut Option<syn::Error>,
) {
    check_role_kind(role, fields, false, errors);
    let supported = match fields.get(&identifier_key(role)) {
        Some(FieldKind::Lex(path)) => symbols.terminals.get(&path_name(path)).is_some_and(|info| {
            info.kind == TerminalKind::Codec && info.codec_atom == Some(CodecAtomClass::Noun)
        }),
        Some(FieldKind::Identity(_)) => false,
        Some(FieldKind::Category(_) | FieldKind::Optional(_) | FieldKind::Sequence { .. })
        | None => return,
    };
    if !supported {
        let declared = match fields.get(&identifier_key(role)) {
            Some(FieldKind::Lex(path)) => symbols
                .terminals
                .get(&path_name(path))
                .and_then(|info| info.codec_atom),
            _ => None,
        };
        let detail = match declared {
            Some(CodecAtomClass::Lex) => "; codec declares `lex`",
            Some(CodecAtomClass::Noun) | None => "",
        };
        combine(
            errors,
            syn::Error::new(
                role.span(),
                format!(
                    "noun atom role `{role}` requires a codec binding declaring `noun`{detail}"
                ),
            ),
        );
    }
}

#[allow(
    clippy::too_many_arguments,
    reason = "the validation check receives the closed symbol and feature environments explicitly"
)]
fn check_feature_role(
    role: &syn::Ident,
    feature: ParsedFeature,
    fields: &HashMap<String, &FieldKind>,
    symbols: &Symbols,
    providers: &HashSet<(String, ParsedFeature)>,
    local_vocab_providers: &HashSet<(String, ParsedFeature)>,
    has_fixed_verb: bool,
    verb_operands: &[&VerbOperand],
    errors: &mut Option<syn::Error>,
) {
    if identifier_key(role) == "verb" && has_fixed_verb {
        if feature != ParsedFeature::Agreement {
            combine(
                errors,
                syn::Error::new(
                    role.span(),
                    format!(
                        "verb slot `{role}` does not provide {}",
                        feature_name(feature)
                    ),
                ),
            );
        }
        return;
    }
    match fields.get(&identifier_key(role)) {
        Some(FieldKind::Category(path)) => {
            let category = path_name(path);
            if !providers.contains(&(category.clone(), feature)) {
                combine(
                    errors,
                    syn::Error::new(
                        role.span(),
                        format!(
                            "category `{category}` does not provide {}",
                            feature_name(feature)
                        ),
                    ),
                );
            }
        }
        Some(FieldKind::Lex(_))
            if local_vocab_providers.contains(&(identifier_key(role), feature)) =>
        {
        }
        Some(FieldKind::Lex(path))
            if verb_operands.iter().any(
                |operand| matches!(operand, VerbOperand::Projected(field) if same_identifier(field, role)),
            ) && symbols
                .terminals
                .get(&path_name(path))
                .is_some_and(|terminal| terminal.kind == TerminalKind::Lexeme) =>
        {
            if feature != ParsedFeature::Agreement {
                combine(
                    errors,
                    syn::Error::new(
                        role.span(),
                        format!(
                            "verb slot `{role}` does not provide {}",
                            feature_name(feature)
                        ),
                    ),
                );
            }
        }
        Some(FieldKind::Lex(_)) => combine(
            errors,
            syn::Error::new(
                role.span(),
                format!(
                    "lexical role `{role}` does not have an exhaustive local {} writer",
                    feature_name(feature)
                ),
            ),
        ),
        Some(_) => combine(
            errors,
            syn::Error::new(
                role.span(),
                format!("role `{role}` does not provide {}", feature_name(feature)),
            ),
        ),
        None => combine(
            errors,
            syn::Error::new(role.span(), format!("unknown feature role `{role}`")),
        ),
    }
}

fn local_vocab_feature_providers(
    construction: &crate::Construction,
    fields: &HashMap<String, &FieldKind>,
    symbols: &Symbols,
) -> HashSet<(String, ParsedFeature)> {
    construction
        .equations
        .iter()
        .filter_map(|equation| {
            let ParsedFeaturePlace::Role { field, feature } = &equation.target else {
                return None;
            };
            let ParsedFeatureValue::Match { role, arms } = &equation.value else {
                return None;
            };
            if !same_identifier(field, role) {
                return None;
            }
            let Some(FieldKind::Lex(path)) = fields.get(&identifier_key(role)) else {
                return None;
            };
            let terminal = symbols.terminals.get(&path_name(path))?;
            if terminal.kind != TerminalKind::Vocab {
                return None;
            }
            let variants = arms
                .iter()
                .map(|arm| identifier_key(&arm.variant))
                .collect::<HashSet<_>>();
            (variants.len() == arms.len() && variants == terminal.variants)
                .then(|| (identifier_key(role), *feature))
        })
        .collect()
}

fn check_role_kind(
    role: &syn::Ident,
    fields: &HashMap<String, &FieldKind>,
    category: bool,
    errors: &mut Option<syn::Error>,
) {
    match fields
        .get(&identifier_key(role))
        .map(|kind| field_kind_leaf(kind))
    {
        None => combine(
            errors,
            syn::Error::new(role.span(), format!("unknown role `{role}`")),
        ),
        Some(FieldKind::Category(_)) if !category => combine(
            errors,
            syn::Error::new(
                role.span(),
                format!("category field `{role}` used as a lexical role"),
            ),
        ),
        Some(FieldKind::Lex(_) | FieldKind::Identity(_)) if category => combine(
            errors,
            syn::Error::new(
                role.span(),
                format!("lexical field `{role}` used as a category role"),
            ),
        ),
        _ => {}
    }
}

fn field_kind_leaf(kind: &FieldKind) -> &FieldKind {
    match kind {
        FieldKind::Optional(value) => field_kind_leaf(value),
        FieldKind::Sequence { item, .. } => field_kind_leaf(item),
        FieldKind::Category(_) | FieldKind::Lex(_) | FieldKind::Identity(_) => kind,
    }
}

fn check_verb_role(
    role: &syn::Ident,
    fields: &HashMap<String, &FieldKind>,
    symbols: &Symbols,
    errors: &mut Option<syn::Error>,
) {
    check_role_kind(role, fields, false, errors);
    let supported = match fields.get(&identifier_key(role)) {
        Some(FieldKind::Lex(path)) => symbols
            .terminals
            .get(&path_name(path))
            .is_some_and(|info| info.kind == TerminalKind::Lexeme),
        Some(FieldKind::Identity(_)) => false,
        Some(FieldKind::Category(_) | FieldKind::Optional(_) | FieldKind::Sequence { .. })
        | None => return,
    };
    if !supported {
        combine(
            errors,
            syn::Error::new(
                role.span(),
                format!("verb role `{role}` must use a declared lexeme"),
            ),
        );
    }
}

fn check_vocab_role(
    role: &syn::Ident,
    fields: &HashMap<String, &FieldKind>,
    symbols: &Symbols,
    errors: &mut Option<syn::Error>,
) {
    check_role_kind(role, fields, false, errors);
    if let Some(FieldKind::Lex(path)) = fields.get(&identifier_key(role)) {
        let name = path_name(path);
        if symbols
            .terminals
            .get(&name)
            .is_some_and(|info| info.kind != TerminalKind::Vocab)
        {
            combine(
                errors,
                syn::Error::new(
                    role.span(),
                    format!("match role `{role}` must use a declared vocab"),
                ),
            );
        }
    }
}

fn check_terminal_variant(
    path: &syn::Path,
    expected: TerminalKind,
    symbols: &Symbols,
    errors: &mut Option<syn::Error>,
) {
    let segments: Vec<_> = path.segments.iter().collect();
    if segments.len() < 2 {
        combine(
            errors,
            syn::Error::new_spanned(path, "fixed terminal value must be `Type::Variant`"),
        );
        return;
    }
    let terminal = identifier_key(&segments[segments.len() - 2].ident);
    let variant = identifier_key(&segments.last().expect("at least two").ident);
    match symbols.terminals.get(&terminal) {
        None => combine(
            errors,
            syn::Error::new_spanned(path, format!("unknown terminal `{terminal}`")),
        ),
        Some(info) if info.kind != expected => combine(
            errors,
            syn::Error::new_spanned(path, format!("terminal `{terminal}` has the wrong kind")),
        ),
        Some(info) if !info.variants.contains(&variant) => combine(
            errors,
            syn::Error::new_spanned(
                path,
                format!("unknown variant `{variant}` for lexeme `{terminal}`"),
            ),
        ),
        Some(_) => {}
    }
}

fn validate_stored_fields(raw: &Declarations) -> syn::Result<()> {
    let mut errors = None;
    for declaration in &raw.declarations {
        let Declaration::Construction(construction) = declaration else { continue };
        let mut counts: HashMap<String, usize> = construction
            .element
            .fields
            .iter()
            .map(|field| (identifier_key(&field.name), 0))
            .collect();
        for atom in &construction.form.atoms {
            let role = match atom {
                FormAtom::Role(role)
                | FormAtom::Lex(role)
                | FormAtom::Identity(role)
                | FormAtom::Noun(role)
                | FormAtom::Verb(VerbOperand::Projected(role)) => Some(role),
                FormAtom::Verb(VerbOperand::Fixed(_))
                | FormAtom::OpenVerb(_)
                | FormAtom::Literal(_) => None,
            };
            if let Some(role) = role
                && let Some(count) = counts.get_mut(&identifier_key(role))
            {
                *count += 1;
            }
        }
        for field in &construction.element.fields {
            match counts[&identifier_key(&field.name)] {
                0 => combine(
                    &mut errors,
                    syn::Error::new(
                        field.name.span(),
                        format!("field `{}` is not present in the form", field.name),
                    ),
                ),
                1 => {}
                count => combine(
                    &mut errors,
                    syn::Error::new(
                        field.name.span(),
                        format!("field `{}` is used {count} times in the form", field.name),
                    ),
                ),
            }
        }
    }
    finish(errors)
}

fn validate_bindings(raw: &Declarations) -> syn::Result<()> {
    let mut errors = None;
    let callbacks = traversal_callbacks(raw, &mut errors);
    for declaration in &raw.declarations {
        match declaration {
            Declaration::Codec(binding) | Declaration::Identity(binding)
                if binding.generated.is_none() && binding.generated_identity.is_none() =>
            {
                validate_binding(binding, &callbacks, &mut errors);
            }
            Declaration::Construction(_)
            | Declaration::AbstractProduct(_)
            | Declaration::AbstractSum(_)
            | Declaration::Vocab(_)
            | Declaration::Morphology(_)
            | Declaration::Lexeme(_)
            | Declaration::Codec(_)
            | Declaration::Identity(_)
            | Declaration::Root(_) => {}
        }
    }
    finish(errors)
}

#[derive(Clone)]
struct TraversalSignature {
    mode: VisitMode,
    value_type: String,
    owner: String,
}

#[derive(Default)]
struct TraversalCallbacks {
    walkers: HashMap<String, TraversalSignature>,
    visitors: HashMap<String, TraversalSignature>,
}

fn traversal_callbacks(raw: &Declarations, errors: &mut Option<syn::Error>) -> TraversalCallbacks {
    let mut callbacks = TraversalCallbacks::default();
    let mut categories = HashSet::new();
    for declaration in &raw.declarations {
        match declaration {
            Declaration::Construction(construction) => {
                let category = path_name(&construction.category);
                if categories.insert(category.clone()) {
                    register_traversal_callback(
                        &mut callbacks.walkers,
                        format!("walk_{}", snake_case(&category)),
                        VisitMode::Borrowed,
                        category.clone(),
                        format!("category `{category}`"),
                        construction.category.span(),
                        errors,
                    );
                    register_traversal_callback(
                        &mut callbacks.visitors,
                        format!("visit_{}", snake_case(&category)),
                        VisitMode::Borrowed,
                        category.clone(),
                        format!("category `{category}`"),
                        construction.category.span(),
                        errors,
                    );
                }
                let element = identifier_key(&construction.element.name);
                for (prefix, registry) in [
                    ("walk", &mut callbacks.walkers),
                    ("visit", &mut callbacks.visitors),
                ] {
                    register_traversal_callback(
                        registry,
                        format!("{prefix}_{}", snake_case(&element)),
                        VisitMode::Borrowed,
                        element.clone(),
                        format!("construction `{}`", construction.name),
                        construction.element.name.span(),
                        errors,
                    );
                }
            }
            Declaration::Vocab(vocab) => register_terminal_callbacks(
                &mut callbacks,
                &vocab.name,
                VisitMode::Copy,
                format!("vocab `{}`", vocab.name),
                errors,
            ),
            Declaration::Lexeme(lexeme) => register_terminal_callbacks(
                &mut callbacks,
                &lexeme.name,
                VisitMode::Copy,
                format!("lexeme `{}`", lexeme.name),
                errors,
            ),
            Declaration::Codec(binding) | Declaration::Identity(binding) => {
                if let Some(crate::model::GeneratedCodecRecipe::SignedDecimal(source)) =
                    &binding.generated
                {
                    if let Some(sign) = source.sign_type_slots.first() {
                        register_terminal_callbacks(
                            &mut callbacks,
                            &sign.name,
                            VisitMode::Copy,
                            format!("signed_decimal sign `{}`", sign.name),
                            errors,
                        );
                    }
                    register_terminal_callbacks(
                        &mut callbacks,
                        &binding.name,
                        VisitMode::Borrowed,
                        format!("signed_decimal codec `{}`", binding.name),
                        errors,
                    );
                } else if binding.generated_identity.is_some() {
                    register_terminal_callbacks(
                        &mut callbacks,
                        &binding.name,
                        VisitMode::Copy,
                        format!("context identity `{}`", binding.name),
                        errors,
                    );
                } else if let Some(mode) = binding.traversal.callback_mode {
                    register_terminal_callbacks_as(
                        &mut callbacks,
                        &identifier_key(&binding.name),
                        &simple_type_name(&binding.value_type),
                        mode,
                        format!("binding `{}`", binding.name),
                        binding.name.span(),
                        errors,
                    );
                }
            }
            Declaration::AbstractProduct(_)
            | Declaration::AbstractSum(_)
            | Declaration::Root(_)
            | Declaration::Morphology(_) => {}
        }
    }
    for declaration in &raw.declarations {
        let (Declaration::Codec(binding) | Declaration::Identity(binding)) = declaration else {
            continue;
        };
        for leaf in &binding.traversal.leaf_callbacks {
            register_traversal_callback(
                &mut callbacks.visitors,
                identifier_key(&leaf.name),
                leaf.mode,
                type_name(&leaf.value_type),
                format!("binding `{}` leaf callback", binding.name),
                leaf.name.span(),
                errors,
            );
        }
    }
    callbacks
}

fn register_terminal_callbacks(
    callbacks: &mut TraversalCallbacks,
    name: &syn::Ident,
    mode: VisitMode,
    owner: String,
    errors: &mut Option<syn::Error>,
) {
    let ty = identifier_key(name);
    register_terminal_callbacks_as(callbacks, &ty, &ty, mode, owner, name.span(), errors);
}

fn register_terminal_callbacks_as(
    callbacks: &mut TraversalCallbacks,
    generated_name: &str,
    value_type: &str,
    mode: VisitMode,
    owner: String,
    span: proc_macro2::Span,
    errors: &mut Option<syn::Error>,
) {
    register_traversal_callback(
        &mut callbacks.walkers,
        format!("walk_{}", snake_case(generated_name)),
        mode,
        value_type.to_owned(),
        owner.clone(),
        span,
        errors,
    );
    register_traversal_callback(
        &mut callbacks.visitors,
        format!("visit_{}", snake_case(generated_name)),
        mode,
        value_type.to_owned(),
        owner,
        span,
        errors,
    );
}

fn register_traversal_callback(
    callbacks: &mut HashMap<String, TraversalSignature>,
    name: String,
    mode: VisitMode,
    value_type: String,
    owner: String,
    span: proc_macro2::Span,
    errors: &mut Option<syn::Error>,
) {
    if let Some(previous) = callbacks.get(&name) {
        combine(
            errors,
            syn::Error::new(
                span,
                format!(
                    "traversal callback `{name}` collides between {} and {owner}",
                    previous.owner
                ),
            ),
        );
    } else {
        callbacks.insert(
            name,
            TraversalSignature {
                mode,
                value_type,
                owner,
            },
        );
    }
}

fn validate_binding(
    binding: &TerminalBinding,
    callbacks: &TraversalCallbacks,
    errors: &mut Option<syn::Error>,
) {
    validate_binding_value_type(binding, errors);
    validate_context_identity(binding, errors);
    let mut bound = HashSet::new();
    let Some(build) = &binding.build else {
        bound.insert(binding.traversal.argument.as_ref().map_or_else(
            || snake_case(&identifier_key(&binding.name)),
            identifier_key,
        ));
        validate_traversal(binding, &bound, callbacks, errors);
        return;
    };
    let Some(lexical_variant) = &binding.lexical_variant else {
        combine(
            errors,
            syn::Error::new(
                binding.name.span(),
                "binding build requires lexical metadata",
            ),
        );
        return;
    };
    let pattern_ok = match &build.pattern {
        syn::Pat::TupleStruct(tuple) => {
            let variant_matches = tuple
                .path
                .segments
                .last()
                .zip(lexical_variant.segments.last())
                .is_some_and(|(left, right)| same_identifier(&left.ident, &right.ident));
            let mut shapes_ok = true;
            for pat in &tuple.elems {
                let syn::Pat::Ident(ident) = pat else {
                    shapes_ok = false;
                    continue;
                };
                if ident.by_ref.is_some() || ident.mutability.is_some() || ident.subpat.is_some() {
                    shapes_ok = false;
                    continue;
                }
                if !bound.insert(identifier_key(&ident.ident)) {
                    combine(
                        errors,
                        syn::Error::new(
                            ident.ident.span(),
                            format!("duplicate binding pattern slot `{}`", ident.ident),
                        ),
                    );
                }
            }
            variant_matches && shapes_ok
        }
        _ => false,
    };
    if !pattern_ok {
        combine(
            errors,
            syn::Error::new_spanned(
                &build.pattern,
                "binding pattern must be one declared BuildValue tuple variant with unique identifier slots",
            ),
        );
    }
    let noun_number_slot = (binding.codec_atom == Some(CodecAtomClass::Noun))
        .then(|| match &build.pattern {
            syn::Pat::TupleStruct(tuple) => tuple.elems.iter().find_map(|pattern| {
                let syn::Pat::Ident(slot) = pattern else {
                    return None;
                };
                (identifier_key(&slot.ident) == "number").then_some(&slot.ident)
            }),
            _ => None,
        })
        .flatten();
    if let Some(number) = noun_number_slot {
        combine(
            errors,
            syn::Error::new(
                number.span(),
                "noun binding pattern slot `number` collides with the generated scanner-number field",
            ),
        );
    }
    if !closed_expr(&build.construct, &bound, true) {
        combine(
            errors,
            syn::Error::new_spanned(
                &build.construct,
                "binding construct expression is outside the closed path/call/field syntax or references an undeclared pattern name",
            ),
        );
    }
    validate_traversal(binding, &bound, callbacks, errors);
}

fn validate_binding_value_type(binding: &TerminalBinding, errors: &mut Option<syn::Error>) {
    let syn::Type::Path(value_type) = &binding.value_type else {
        combine(
            errors,
            syn::Error::new_spanned(
                &binding.value_type,
                "binding value_type must be a qself-free, non-generic identifier path",
            ),
        );
        return;
    };
    let supported_shape = value_type.qself.is_none()
        && !value_type.path.segments.is_empty()
        && value_type
            .path
            .segments
            .iter()
            .all(|segment| matches!(segment.arguments, syn::PathArguments::None));
    if !supported_shape {
        combine(
            errors,
            syn::Error::new_spanned(
                &binding.value_type,
                "binding value_type must be a qself-free, non-generic identifier path",
            ),
        );
        return;
    }
    if value_type
        .path
        .segments
        .last()
        .map(|segment| &segment.ident)
        .is_none_or(|name| !same_identifier(name, &binding.name))
    {
        combine(
            errors,
            syn::Error::new_spanned(
                &binding.value_type,
                format!(
                    "binding value_type final identifier must match binding declaration `{}`",
                    binding.name
                ),
            ),
        );
    }
}

fn validate_context_identity(binding: &TerminalBinding, errors: &mut Option<syn::Error>) {
    let Some(crate::RenderBinding::ContextIdentity(arms)) = &binding.render else {
        return;
    };
    if arms.is_empty() {
        combine(
            errors,
            syn::Error::new(
                binding.name.span(),
                "context identity render requires at least one arm",
            ),
        );
    }
    let mut seen = HashSet::new();
    for arm in arms {
        if !seen.insert(identifier_key(&arm.variant)) {
            combine(
                errors,
                syn::Error::new(
                    arm.variant.span(),
                    format!("duplicate context identity variant `{}`", arm.variant),
                ),
            );
        }
    }
    let traversal = binding
        .traversal
        .variants
        .iter()
        .map(identifier_key)
        .collect::<HashSet<_>>();
    for variant in seen.difference(&traversal) {
        combine(
            errors,
            syn::Error::new(
                binding.name.span(),
                format!("unknown context identity variant `{variant}`"),
            ),
        );
    }
    for variant in traversal.difference(&seen) {
        combine(
            errors,
            syn::Error::new(
                binding.name.span(),
                format!("missing context identity variant `{variant}`"),
            ),
        );
    }
}

#[allow(
    clippy::too_many_lines,
    reason = "the closed traversal schema is validated as one ordered diagnostic pass"
)]
fn validate_traversal(
    binding: &TerminalBinding,
    bound: &HashSet<String>,
    callbacks: &TraversalCallbacks,
    errors: &mut Option<syn::Error>,
) {
    let traversal = &binding.traversal;
    if let Some(part) = traversal.parts.first() {
        combine(
            errors,
            syn::Error::new(
                part.name.span(),
                "unimplemented in MVP: `legacy traversal part/visit`; use the typed callback/argument/body schema",
            ),
        );
    } else if let Some(visit) = traversal.visit_order.first() {
        combine(
            errors,
            syn::Error::new(
                visit.span(),
                "unimplemented in MVP: `legacy traversal part/visit`; use the typed callback/argument/body schema",
            ),
        );
    }
    let uses_closed_recipe = traversal.callback_mode.is_some()
        || traversal.argument.is_some()
        || !traversal.variants.is_empty()
        || !traversal.fields.is_empty()
        || !traversal.calls.is_empty()
        || !traversal.branches.is_empty()
        || !traversal.leaf_callbacks.is_empty();
    if uses_closed_recipe {
        if traversal.callback_mode.is_none() {
            combine(
                errors,
                syn::Error::new(
                    binding.name.span(),
                    "closed traversal recipe requires callback pass mode",
                ),
            );
        }
        if traversal.argument.is_none() {
            combine(
                errors,
                syn::Error::new(
                    binding.name.span(),
                    "closed traversal recipe requires an argument name",
                ),
            );
        }
        if !traversal.parts.is_empty() || !traversal.visit_order.is_empty() {
            combine(
                errors,
                syn::Error::new(
                    binding.name.span(),
                    "closed traversal recipe cannot mix legacy part/visit metadata",
                ),
            );
        }
        let enum_body = !traversal.variants.is_empty() && traversal.branches.is_empty();
        let calls_body = !traversal.calls.is_empty();
        let match_body = !traversal.branches.is_empty();
        let body_kinds = [enum_body, calls_body, match_body]
            .into_iter()
            .filter(|present| *present)
            .count();
        if body_kinds != 1 {
            combine(
                errors,
                syn::Error::new(
                    binding.name.span(),
                    "closed traversal recipe requires exactly one enum, calls, or match body",
                ),
            );
        }
        if let Some(mode) = traversal.callback_mode {
            let mismatch = match (enum_body, calls_body, match_body, mode) {
                (true, false, false, VisitMode::Borrowed) => {
                    Some("enum traversal body requires copy callback mode")
                }
                (false, true, false, VisitMode::Copy) => {
                    Some("calls traversal body requires borrowed callback mode")
                }
                (false, false, true, VisitMode::Copy) => {
                    Some("match traversal body requires borrowed callback mode")
                }
                _ => None,
            };
            if let Some(message) = mismatch {
                combine(errors, syn::Error::new(binding.name.span(), message));
            }
        }
        let mut names = HashSet::new();
        for variant in &traversal.variants {
            if !names.insert(identifier_key(variant)) {
                combine(
                    errors,
                    syn::Error::new(
                        variant.span(),
                        format!("duplicate traversal variant `{variant}`"),
                    ),
                );
            }
        }
        names.clear();
        let mut call_bound = bound.clone();
        let argument = traversal.argument.as_ref().map_or_else(
            || snake_case(&identifier_key(&binding.name)),
            identifier_key,
        );
        call_bound.insert(argument.clone());
        names.insert(argument.clone());
        let mut call_types = HashMap::from([(argument, simple_type_name(&binding.value_type))]);
        for field in &traversal.fields {
            let name = identifier_key(&field.name);
            if !names.insert(name.clone()) {
                combine(
                    errors,
                    syn::Error::new(
                        field.name.span(),
                        format!("duplicate traversal field `{}`", field.name),
                    ),
                );
            }
            call_bound.insert(name.clone());
            call_types.insert(name, type_name(&field.value_type));
        }
        names.clear();
        for leaf in &traversal.leaf_callbacks {
            if !identifier_key(&leaf.name).starts_with("visit_") {
                combine(
                    errors,
                    syn::Error::new(
                        leaf.name.span(),
                        "leaf callback name must start with `visit_`",
                    ),
                );
            }
            if !names.insert(identifier_key(&leaf.name)) {
                combine(
                    errors,
                    syn::Error::new(
                        leaf.name.span(),
                        format!("duplicate traversal leaf callback `{}`", leaf.name),
                    ),
                );
            }
        }
        for call in &traversal.calls {
            validate_traversal_call(call, &call_bound, &call_types, callbacks, errors);
        }
        if traversal.branches.len() > 1 {
            combine(
                errors,
                syn::Error::new(
                    binding.name.span(),
                    "closed traversal recipe accepts exactly one match body",
                ),
            );
        }
        for branch in &traversal.branches {
            if !closed_expr(&branch.value, &call_bound, false) {
                combine(
                    errors,
                    syn::Error::new_spanned(
                        &branch.value,
                        "traversal match value must be rooted in the bound runtime value",
                    ),
                );
            }
            if branch.arms.is_empty() {
                combine(
                    errors,
                    syn::Error::new_spanned(
                        &branch.value,
                        "traversal match requires at least one branch",
                    ),
                );
            }
            if traversal.variants.is_empty() {
                combine(
                    errors,
                    syn::Error::new(
                        binding.name.span(),
                        "traversal match requires declared exhaustive variants",
                    ),
                );
            }
            let matched_type = expr_type(&branch.value, &call_types);
            let mut covered = HashSet::new();
            for arm in &branch.arms {
                let mut arm_bound = call_bound.clone();
                let mut arm_types = call_types.clone();
                let variant = arm
                    .variant
                    .segments
                    .last()
                    .map(|segment| identifier_key(&segment.ident))
                    .unwrap_or_default();
                if !covered.insert(variant.clone()) {
                    combine(
                        errors,
                        syn::Error::new_spanned(
                            &arm.variant,
                            format!("duplicate traversal match variant `{variant}`"),
                        ),
                    );
                }
                let owner = arm
                    .variant
                    .segments
                    .iter()
                    .rev()
                    .nth(1)
                    .map(|segment| identifier_key(&segment.ident));
                if owner.as_ref() != matched_type.as_ref() {
                    combine(
                        errors,
                        syn::Error::new_spanned(
                            &arm.variant,
                            "traversal match variant type does not match the matched runtime value",
                        ),
                    );
                }
                let binding_name = identifier_key(&arm.binding);
                arm_bound.insert(binding_name.clone());
                arm_types.insert(binding_name, type_name(&arm.value_type));
                validate_traversal_call(&arm.call, &arm_bound, &arm_types, callbacks, errors);
            }
            let declared = traversal
                .variants
                .iter()
                .map(identifier_key)
                .collect::<HashSet<_>>();
            for variant in covered.difference(&declared) {
                combine(
                    errors,
                    syn::Error::new(
                        binding.name.span(),
                        format!("unknown traversal match variant `{variant}`"),
                    ),
                );
            }
            for variant in declared.difference(&covered) {
                combine(
                    errors,
                    syn::Error::new(
                        binding.name.span(),
                        format!("missing traversal match variant `{variant}`"),
                    ),
                );
            }
        }
    }
    let mut parts = HashSet::new();
    for part in &binding.traversal.parts {
        let name = identifier_key(&part.name);
        if !parts.insert(name.clone()) {
            combine(
                errors,
                syn::Error::new(
                    part.name.span(),
                    format!("duplicate traversal part `{name}`"),
                ),
            );
        }
        if !closed_expr(&part.value, bound, false) {
            combine(
                errors,
                syn::Error::new_spanned(
                    &part.value,
                    "traversal expression must be a field path rooted in a declared pattern name",
                ),
            );
        }
        let _ = part.kind == TraversalKind::Scalar;
    }
    let mut visited = HashSet::new();
    for visit in &binding.traversal.visit_order {
        let name = identifier_key(visit);
        if !parts.contains(&name) {
            combine(
                errors,
                syn::Error::new(visit.span(), format!("unknown traversal part `{name}`")),
            );
        } else if !visited.insert(name.clone()) {
            combine(
                errors,
                syn::Error::new(
                    visit.span(),
                    format!("traversal part `{name}` is visited more than once"),
                ),
            );
        }
    }
    for part in &binding.traversal.parts {
        if !visited.contains(&identifier_key(&part.name)) {
            combine(
                errors,
                syn::Error::new(
                    part.name.span(),
                    format!("traversal part `{}` is never visited", part.name),
                ),
            );
        }
    }
}

fn validate_traversal_call(
    call: &crate::TraversalCall,
    bound: &HashSet<String>,
    types: &HashMap<String, String>,
    callbacks: &TraversalCallbacks,
    errors: &mut Option<syn::Error>,
) {
    let (valid_callback, callback_name, registry) = match call.callback.segments.len() {
        1 if call.callback.leading_colon.is_none() => (
            true,
            identifier_key(&call.callback.segments[0].ident),
            &callbacks.walkers,
        ),
        2 if call.callback.leading_colon.is_none()
            && identifier_key(&call.callback.segments[0].ident) == "visitor" =>
        {
            (
                true,
                identifier_key(&call.callback.segments[1].ident),
                &callbacks.visitors,
            )
        }
        _ => (
            false,
            call.callback.to_token_stream().to_string(),
            &callbacks.walkers,
        ),
    };
    if !valid_callback {
        combine(
            errors,
            syn::Error::new_spanned(
                &call.callback,
                "traversal callback must be a local walker or `visitor::method`",
            ),
        );
    }
    let signature = valid_callback
        .then(|| registry.get(&callback_name))
        .flatten();
    if valid_callback && signature.is_none() {
        combine(
            errors,
            syn::Error::new_spanned(
                &call.callback,
                format!("unknown traversal callback `{callback_name}`"),
            ),
        );
    }
    if let Some(signature) = signature {
        if signature.mode != call.mode {
            combine(
                errors,
                syn::Error::new_spanned(
                    &call.callback,
                    format!(
                        "traversal callback `{callback_name}` requires {} traversal mode",
                        visit_mode_name(signature.mode)
                    ),
                ),
            );
        }
        if let Some(value_type) = expr_type(&call.value, types) {
            if value_type != signature.value_type {
                combine(
                    errors,
                    syn::Error::new_spanned(
                        &call.value,
                        format!(
                            "traversal callback `{callback_name}` expects `{}` but argument is `{value_type}`",
                            signature.value_type
                        ),
                    ),
                );
            }
        } else {
            combine(
                errors,
                syn::Error::new_spanned(
                    &call.value,
                    "traversal callback argument lacks a declared value type",
                ),
            );
        }
    }
    if !closed_expr(&call.value, bound, false) {
        combine(
            errors,
            syn::Error::new_spanned(
                &call.value,
                "traversal callback argument must be a field path rooted in a declared runtime value",
            ),
        );
    }
}

fn expr_type(expr: &syn::Expr, types: &HashMap<String, String>) -> Option<String> {
    match expr {
        syn::Expr::Path(path) if path.qself.is_none() && path.path.segments.len() == 1 => types
            .get(&identifier_key(&path.path.segments[0].ident))
            .cloned(),
        syn::Expr::Paren(paren) => expr_type(&paren.expr, types),
        _ => None,
    }
}

fn simple_type_name(value_type: &syn::Type) -> String {
    match value_type {
        syn::Type::Path(path) => path.path.segments.last().map_or_else(
            || type_name(value_type),
            |segment| identifier_key(&segment.ident),
        ),
        _ => type_name(value_type),
    }
}

fn type_name(value_type: &syn::Type) -> String {
    semantic_token_name(value_type.to_token_stream())
}

fn semantic_token_name(tokens: proc_macro2::TokenStream) -> String {
    tokens
        .into_iter()
        .map(|token| match token {
            proc_macro2::TokenTree::Group(group) => {
                let inner = semantic_token_name(group.stream());
                match group.delimiter() {
                    proc_macro2::Delimiter::Parenthesis => format!("({inner})"),
                    proc_macro2::Delimiter::Brace => format!("{{{inner}}}"),
                    proc_macro2::Delimiter::Bracket => format!("[{inner}]"),
                    proc_macro2::Delimiter::None => inner,
                }
            }
            proc_macro2::TokenTree::Ident(ident) => identifier_key(&ident),
            proc_macro2::TokenTree::Punct(punct) => punct.as_char().to_string(),
            proc_macro2::TokenTree::Literal(literal) => literal.to_string(),
        })
        .collect()
}

fn visit_mode_name(mode: VisitMode) -> &'static str {
    match mode {
        VisitMode::Copy => "copy",
        VisitMode::Borrowed => "borrowed",
    }
}

fn closed_expr(expr: &syn::Expr, bound: &HashSet<String>, allow_calls: bool) -> bool {
    match expr {
        syn::Expr::Path(path) => {
            path.qself.is_none()
                && path.path.segments.len() == 1
                && path
                    .path
                    .segments
                    .first()
                    .is_some_and(|segment| bound.contains(&identifier_key(&segment.ident)))
        }
        syn::Expr::Field(field) => closed_expr(&field.base, bound, false),
        syn::Expr::Call(call) if allow_calls => {
            matches!(&*call.func, syn::Expr::Path(path) if path.qself.is_none() && path.path.segments.len() > 1)
                && call.args.iter().all(|arg| closed_expr(arg, bound, true))
        }
        syn::Expr::Paren(paren) => closed_expr(&paren.expr, bound, allow_calls),
        _ => false,
    }
}

fn validate_invariants(
    raw: &Declarations,
    symbols: &Symbols,
) -> syn::Result<HashMap<String, (proc_macro2::Span, InvariantPlan)>> {
    let mut errors = None;
    let mut invariants = HashMap::new();
    for declaration in &raw.declarations {
        let Declaration::Construction(construction) = declaration else { continue };
        let fields: HashMap<_, _> = construction
            .element
            .fields
            .iter()
            .map(|field| (identifier_key(&field.name), &field.kind))
            .collect();
        let mut alternatives = vec![PredicateConjunctionPlan::new(Vec::new())];
        for requirement in &construction.requirements {
            if matches!(requirement, RequireExprSource::Length { .. }) {
                continue;
            }
            let normalized =
                match normalize_predicate(raw, construction, requirement, &fields, symbols) {
                    Ok(normalized) => normalized,
                    Err(error) => {
                        combine(&mut errors, error);
                        continue;
                    }
                };
            match predicate_all(alternatives, &normalized) {
                Ok(combined) => alternatives = combined,
                Err(error) => {
                    combine(&mut errors, error);
                    alternatives = Vec::new();
                }
            }
        }
        deduplicate_alternatives(&mut alternatives);
        if alternatives.is_empty() {
            combine(
                &mut errors,
                syn::Error::new(
                    construction.name.span(),
                    "invariant predicate has no satisfiable alternative",
                ),
            );
        }
        let invariant = InvariantPlan::from_alternatives(alternatives);
        invariants.insert(
            identifier_key(&construction.name),
            (construction.name.span(), invariant),
        );
    }
    finish(errors)?;
    Ok(invariants)
}

fn normalize_predicate(
    raw: &Declarations,
    construction: &crate::Construction,
    expression: &crate::RequireExprSource,
    fields: &HashMap<String, &FieldKind>,
    symbols: &Symbols,
) -> syn::Result<Vec<PredicateConjunctionPlan>> {
    match expression {
        crate::RequireExprSource::In { subject, members } => {
            let atom =
                resolve_predicate_atom(raw, construction, subject, members, fields, symbols)?;
            Ok(vec![PredicateConjunctionPlan::new(
                atom.into_iter().collect(),
            )])
        }
        crate::RequireExprSource::All(operands) => {
            let mut errors = None;
            let mut alternatives = vec![PredicateConjunctionPlan::new(Vec::new())];
            for operand in operands {
                match normalize_predicate(raw, construction, operand, fields, symbols) {
                    Ok(operand) => match predicate_all(alternatives.clone(), &operand) {
                        Ok(combined) => alternatives = combined,
                        Err(error) => combine(&mut errors, error),
                    },
                    Err(error) => combine(&mut errors, error),
                }
            }
            errors.map_or(Ok(alternatives), Err)
        }
        crate::RequireExprSource::Any(operands) => {
            let mut errors = None;
            let mut alternatives = Vec::new();
            for operand in operands {
                match normalize_predicate(raw, construction, operand, fields, symbols) {
                    Ok(mut operand) => alternatives.append(&mut operand),
                    Err(error) => combine(&mut errors, error),
                }
            }
            deduplicate_alternatives(&mut alternatives);
            errors.map_or(Ok(alternatives), Err)
        }
        crate::RequireExprSource::Length { .. } => Ok(Vec::new()),
    }
}

fn resolve_predicate_atom(
    raw: &Declarations,
    construction: &crate::Construction,
    subject: &crate::RequireSubjectSource,
    members: &[syn::Ident],
    fields: &HashMap<String, &FieldKind>,
    symbols: &Symbols,
) -> syn::Result<Option<PredicateAtomPlan>> {
    let (subject, domain) = match subject {
        crate::RequireSubjectSource::Role(role) => {
            let role_name = identifier_key(role);
            match fields.get(&role_name) {
                Some(FieldKind::Category(path)) => {
                    let category = path_name(path);
                    let domain = symbols
                        .category_variant_order
                        .get(&category)
                        .cloned()
                        .unwrap_or_default();
                    (
                        PredicateSubjectPlan::CategoryRole {
                            role: role.clone(),
                            category,
                        },
                        PredicateDomain::Variants(domain),
                    )
                }
                Some(FieldKind::Lex(path)) => {
                    let terminal = path_name(path);
                    match symbols.terminals.get(&terminal) {
                        Some(info) if info.kind == TerminalKind::Vocab => (
                            PredicateSubjectPlan::VocabRole {
                                role: role.clone(),
                                terminal,
                            },
                            PredicateDomain::Variants(info.variant_order.clone()),
                        ),
                        _ => {
                            return Err(syn::Error::new(
                                role.span(),
                                format!(
                                    "predicate subject `{role_name}` is not a category or vocab predicate domain"
                                ),
                            ));
                        }
                    }
                }
                Some(
                    FieldKind::Identity(_) | FieldKind::Optional(_) | FieldKind::Sequence { .. },
                ) => {
                    return Err(syn::Error::new(
                        role.span(),
                        format!(
                            "predicate subject `{role_name}` is not a category or vocab predicate domain"
                        ),
                    ));
                }
                None => {
                    return Err(syn::Error::new(
                        role.span(),
                        format!("unknown predicate subject `{role_name}`"),
                    ));
                }
            }
        }
        crate::RequireSubjectSource::RoleFeature { role, feature } => {
            let internal = feature::Feature::from(*feature);
            let role_name = identifier_key(role);
            if !fields.contains_key(&role_name) {
                let parse_only = role_name == "verb"
                    && construction
                        .form
                        .atoms
                        .iter()
                        .any(|atom| matches!(atom, FormAtom::Verb(_) | FormAtom::OpenVerb(_)));
                let message = if parse_only {
                    format!(
                        "predicate cannot constrain parse-only morphology feature state `{role_name}.{}`",
                        internal.key()
                    )
                } else {
                    format!("unknown predicate subject `{role_name}.{}`", internal.key())
                };
                return Err(syn::Error::new(role.span(), message));
            }
            if !role_feature_is_constructible(raw, construction, fields, role, *feature) {
                return Err(syn::Error::new(
                    role.span(),
                    format!(
                        "role predicate `{role_name}.{}` has no constructible feature expression",
                        internal.key()
                    ),
                ));
            }
            (
                PredicateSubjectPlan::RoleFeature {
                    role: role.clone(),
                    feature: internal,
                },
                PredicateDomain::Feature(internal),
            )
        }
        crate::RequireSubjectSource::ConstructionFeature(feature) => {
            let internal = feature::Feature::from(*feature);
            if !construction_feature_is_constructible(raw, construction, fields, *feature) {
                return Err(syn::Error::new(
                    members
                        .first()
                        .map_or_else(proc_macro2::Span::call_site, syn::Ident::span),
                    format!(
                        "construction predicate `{}` has no constructible feature expression",
                        internal.key()
                    ),
                ));
            }
            (
                PredicateSubjectPlan::ConstructionFeature(internal),
                PredicateDomain::Feature(internal),
            )
        }
    };
    let allowed = resolve_predicate_members(&domain, members)?;
    if allowed.len() == domain.len() {
        return Ok(None);
    }
    Ok(Some(PredicateAtomPlan::new(subject, allowed)))
}

enum PredicateDomain {
    Variants(Vec<String>),
    Feature(feature::Feature),
}

impl PredicateDomain {
    fn len(&self) -> usize {
        match self {
            Self::Variants(variants) => variants.len(),
            Self::Feature(feature) => feature.domain().len(),
        }
    }
}

fn resolve_predicate_members(
    domain: &PredicateDomain,
    members: &[syn::Ident],
) -> syn::Result<Vec<PredicateMemberPlan>> {
    let mut errors = None;
    let mut authored = HashMap::new();
    for member in members {
        let key = identifier_key(member);
        if authored.insert(key.clone(), member).is_some() {
            combine(
                &mut errors,
                syn::Error::new(member.span(), format!("duplicate predicate member `{key}`")),
            );
        }
    }
    let mut allowed = Vec::new();
    match domain {
        PredicateDomain::Variants(domain) => {
            for member in members {
                let key = identifier_key(member);
                if !domain.contains(&key) {
                    combine(
                        &mut errors,
                        syn::Error::new(member.span(), format!("unknown predicate member `{key}`")),
                    );
                }
            }
            for domain_member in domain {
                if let Some(member) = authored.get(domain_member) {
                    allowed.push(PredicateMemberPlan::Variant((*member).clone()));
                }
            }
        }
        PredicateDomain::Feature(feature) => {
            let mut resolved = HashMap::new();
            for member in members {
                match feature.member(member) {
                    Ok(value) => {
                        resolved.insert(value.key(), (value, member.span()));
                    }
                    Err(error) => combine(&mut errors, error),
                }
            }
            for value in feature.domain() {
                if let Some((value, span)) = resolved.get(value.key()) {
                    allowed.push(PredicateMemberPlan::Feature(feature::Spanned::new(
                        *value, *span,
                    )));
                }
            }
        }
    }
    errors.map_or(Ok(allowed), Err)
}

fn predicate_all(
    left: Vec<PredicateConjunctionPlan>,
    right: &[PredicateConjunctionPlan],
) -> syn::Result<Vec<PredicateConjunctionPlan>> {
    let mut errors = None;
    let mut combined = Vec::new();
    for left in left {
        for right in right {
            match merge_predicate_conjunctions(&left, right) {
                Ok(conjunction) => combined.push(conjunction),
                Err(error) => combine(&mut errors, error),
            }
        }
    }
    deduplicate_alternatives(&mut combined);
    if combined.is_empty() {
        errors.map_or(Ok(combined), Err)
    } else {
        Ok(combined)
    }
}

fn merge_predicate_conjunctions(
    left: &PredicateConjunctionPlan,
    right: &PredicateConjunctionPlan,
) -> syn::Result<PredicateConjunctionPlan> {
    let mut atoms = left.atoms().to_vec();
    for right_atom in right.atoms() {
        let subject_key = right_atom.subject().semantic_key();
        if let Some(left_atom) = atoms
            .iter_mut()
            .find(|atom| atom.subject().semantic_key() == subject_key)
        {
            let right_members = right_atom
                .allowed()
                .iter()
                .map(PredicateMemberPlan::semantic_key)
                .collect::<HashSet<_>>();
            let allowed = left_atom
                .allowed()
                .iter()
                .filter(|member| right_members.contains(&member.semantic_key()))
                .cloned()
                .collect::<Vec<_>>();
            if allowed.is_empty() {
                let subject = right_atom
                    .subject()
                    .role()
                    .map_or_else(|| right_atom.subject().semantic_key(), ToString::to_string);
                return Err(syn::Error::new(
                    right_atom
                        .subject()
                        .role()
                        .map_or_else(proc_macro2::Span::call_site, syn::Ident::span),
                    format!("empty predicate intersection for `{subject}`"),
                ));
            }
            *left_atom = PredicateAtomPlan::new(left_atom.subject().clone(), allowed);
        } else {
            atoms.push(right_atom.clone());
        }
    }
    Ok(PredicateConjunctionPlan::new(atoms))
}

fn deduplicate_alternatives(alternatives: &mut Vec<PredicateConjunctionPlan>) {
    let mut seen = HashSet::new();
    alternatives.retain(|alternative| {
        let mut atoms = alternative
            .atoms()
            .iter()
            .map(PredicateAtomPlan::semantic_key)
            .collect::<Vec<_>>();
        atoms.sort();
        seen.insert(atoms)
    });
}

fn construction_feature_is_constructible(
    raw: &Declarations,
    construction: &crate::Construction,
    fields: &HashMap<String, &FieldKind>,
    feature: ParsedFeature,
) -> bool {
    feature_place_is_constructible(
        construction,
        fields,
        &feature_providers(raw),
        &ParsedFeaturePlace::Construction(feature),
        &mut HashSet::new(),
    )
}

fn role_feature_is_constructible(
    raw: &Declarations,
    construction: &crate::Construction,
    fields: &HashMap<String, &FieldKind>,
    role: &syn::Ident,
    feature: ParsedFeature,
) -> bool {
    feature_place_is_constructible(
        construction,
        fields,
        &feature_providers(raw),
        &ParsedFeaturePlace::Role {
            field: role.clone(),
            feature,
        },
        &mut HashSet::new(),
    )
}

fn feature_place_is_constructible(
    construction: &crate::Construction,
    fields: &HashMap<String, &FieldKind>,
    providers: &HashSet<(String, ParsedFeature)>,
    place: &ParsedFeaturePlace,
    visiting: &mut HashSet<String>,
) -> bool {
    let key = place_key(place);
    if !visiting.insert(key.clone()) {
        return false;
    }
    let constructible = construction
        .equations
        .iter()
        .find(|equation| place_key(&equation.target) == key)
        .map_or_else(
            || match place {
                ParsedFeaturePlace::Role { field, feature } => fields
                    .get(&identifier_key(field))
                    .is_some_and(|kind| {
                        matches!(kind, FieldKind::Category(path) if providers.contains(&(path_name(path), *feature)))
                    }),
                ParsedFeaturePlace::Construction(_) => false,
            },
            |equation| {
                feature_expression_is_constructible(
                    construction,
                    fields,
                    providers,
                    &equation.value,
                    visiting,
                )
            },
        );
    visiting.remove(&key);
    constructible
}

fn feature_expression_is_constructible(
    construction: &crate::Construction,
    fields: &HashMap<String, &FieldKind>,
    providers: &HashSet<(String, ParsedFeature)>,
    value: &ParsedFeatureValue,
    visiting: &mut HashSet<String>,
) -> bool {
    match value {
        ParsedFeatureValue::Constant(_) => true,
        ParsedFeatureValue::Match { role, .. } => fields.contains_key(&identifier_key(role)),
        ParsedFeatureValue::FromRole(slot) => feature_place_is_constructible(
            construction,
            fields,
            providers,
            &ParsedFeaturePlace::Role {
                field: slot.role.clone(),
                feature: slot.feature,
            },
            visiting,
        ),
    }
}

type FeatureValidation = (
    HashMap<String, Vec<feature::FeatureEquation>>,
    HashSet<String>,
);

#[allow(
    clippy::too_many_lines,
    reason = "feature validation accumulates the complete directional graph before sealing it"
)]
fn validate_features(raw: &Declarations, symbols: &Symbols) -> syn::Result<FeatureValidation> {
    let mut errors = None;
    let providers = feature_providers(raw);
    let mut lowered = HashMap::new();
    let mut dynamic_numbers = HashSet::new();
    for declaration in &raw.declarations {
        let Declaration::Construction(construction) = declaration else { continue };
        let fields: HashMap<_, _> = construction
            .element
            .fields
            .iter()
            .map(|field| (identifier_key(&field.name), &field.kind))
            .collect();
        let mut writers = HashSet::new();
        let mut edges: HashMap<String, Vec<String>> = HashMap::new();
        let mut equations = Vec::new();
        for equation in &construction.equations {
            let target_feature = match &equation.target {
                ParsedFeaturePlace::Construction(feature)
                | ParsedFeaturePlace::Role { feature, .. } => *feature,
            };
            let target_key = place_key(&equation.target);
            if !writers.insert(target_key.clone()) {
                let span = match &equation.target {
                    ParsedFeaturePlace::Construction(_) => construction.name.span(),
                    ParsedFeaturePlace::Role { field, .. } => field.span(),
                };
                combine(
                    &mut errors,
                    syn::Error::new(span, format!("duplicate writer for `{target_key}`")),
                );
            }
            let target = match &equation.target {
                ParsedFeaturePlace::Construction(feature) => {
                    feature::FeaturePlace::Construction((*feature).into())
                }
                ParsedFeaturePlace::Role { field, feature } => {
                    if let Some(FieldKind::Category(path)) = fields.get(&identifier_key(field)) {
                        let category = path_name(path);
                        if !providers.contains(&(category.clone(), *feature)) {
                            combine(
                                &mut errors,
                                syn::Error::new(
                                    field.span(),
                                    format!(
                                        "category `{category}` does not provide {}",
                                        feature_name(*feature)
                                    ),
                                ),
                            );
                        }
                    }
                    feature::FeaturePlace::Role {
                        field: field.clone(),
                        feature: (*feature).into(),
                    }
                }
            };
            let value = match &equation.value {
                ParsedFeatureValue::Constant(path) => {
                    match feature::lower_constant(target_feature, path) {
                        Ok(value) => feature::FeatureExpr::Constant(feature::Spanned::new(
                            value,
                            path.span(),
                        )),
                        Err(error) => {
                            combine(&mut errors, error);
                            continue;
                        }
                    }
                }
                ParsedFeatureValue::FromRole(slot) => {
                    if slot.feature != target_feature {
                        combine(
                            &mut errors,
                            syn::Error::new(slot.role.span(), "feature domains cannot mix"),
                        );
                        continue;
                    }
                    if let Some(FieldKind::Category(path)) = fields.get(&identifier_key(&slot.role))
                    {
                        let category = path_name(path);
                        if !providers.contains(&(category.clone(), slot.feature)) {
                            combine(
                                &mut errors,
                                syn::Error::new(
                                    slot.role.span(),
                                    format!(
                                        "category `{category}` does not provide {}",
                                        feature_name(slot.feature)
                                    ),
                                ),
                            );
                        }
                    }
                    let source_key = format!(
                        "{}.{}",
                        identifier_key(&slot.role),
                        feature_name(slot.feature)
                    );
                    edges.entry(target_key).or_default().push(source_key);
                    feature::FeatureExpr::FromRole {
                        role: slot.role.clone(),
                        feature: slot.feature.into(),
                    }
                }
                ParsedFeatureValue::Match { role, arms } => {
                    let mut lowered_arms = Vec::new();
                    let mut seen = HashSet::new();
                    let terminal = match fields.get(&identifier_key(role)) {
                        Some(FieldKind::Lex(path)) => symbols.terminals.get(&path_name(path)),
                        _ => None,
                    };
                    for arm in arms {
                        let variant = identifier_key(&arm.variant);
                        if !seen.insert(variant.clone()) {
                            combine(
                                &mut errors,
                                syn::Error::new(
                                    arm.variant.span(),
                                    format!("duplicate match arm `{variant}`"),
                                ),
                            );
                        }
                        if terminal.is_some_and(|info| !info.variants.contains(&variant)) {
                            combine(
                                &mut errors,
                                syn::Error::new(
                                    arm.variant.span(),
                                    format!("unknown match variant `{variant}`"),
                                ),
                            );
                        }
                        match feature::lower_constant(target_feature, &arm.value) {
                            Ok(value) => lowered_arms.push((
                                feature::Spanned::new(arm.variant.clone(), arm.variant.span()),
                                value,
                            )),
                            Err(error) => combine(&mut errors, error),
                        }
                    }
                    if let Some(terminal) = terminal {
                        for missing in &terminal.variant_order {
                            if !seen.contains(missing) {
                                combine(
                                    &mut errors,
                                    syn::Error::new(
                                        role.span(),
                                        format!("missing match arm `{missing}`"),
                                    ),
                                );
                            }
                        }
                    }
                    if target_feature == ParsedFeature::Number {
                        dynamic_numbers.insert(identifier_key(&construction.name));
                    }
                    feature::FeatureExpr::MatchVocab {
                        role: role.clone(),
                        arms: lowered_arms,
                    }
                }
            };
            equations.push(feature::FeatureEquation::new(target, value));
        }
        if contains_cycle(&edges) {
            combine(
                &mut errors,
                syn::Error::new(construction.name.span(), "feature equation cycle"),
            );
        }
        let has_number = writers.contains("construction.number");
        for atom in &construction.form.atoms {
            match atom {
                FormAtom::Noun(_) if !has_number => combine(
                    &mut errors,
                    syn::Error::new(
                        construction.form.name.span(),
                        "noun atom requires number in scope",
                    ),
                ),
                _ => {}
            }
        }
        for atom in &construction.form.atoms {
            let slot = match atom {
                FormAtom::Verb(VerbOperand::Fixed(_)) | FormAtom::OpenVerb(_) => {
                    Some("verb".to_owned())
                }
                FormAtom::Verb(VerbOperand::Projected(role)) => Some(identifier_key(role)),
                _ => None,
            };
            let Some(slot) = slot else { continue };
            let direct_writer = writers.contains(&format!("{slot}.agreement"));
            let equality_writer = construction.equations.iter().any(|equation| {
                matches!(
                    (&equation.target, &equation.value),
                    (
                        ParsedFeaturePlace::Construction(ParsedFeature::Agreement),
                        ParsedFeatureValue::FromRole(source)
                    ) if identifier_key(&source.role) == slot && source.feature == ParsedFeature::Agreement
                )
            });
            if !direct_writer && !equality_writer {
                combine(
                    &mut errors,
                    syn::Error::new(
                        construction.form.name.span(),
                        format!("verb atom requires agreement binding for `{slot}.agreement`"),
                    ),
                );
            }
        }
        lowered.insert(identifier_key(&construction.name), equations);
    }
    finish(errors)?;
    Ok((lowered, dynamic_numbers))
}

fn seal_feature_resolutions(
    raw: &Declarations,
    equations: &HashMap<String, Vec<feature::FeatureEquation>>,
    invariants: &HashMap<String, (proc_macro2::Span, InvariantPlan)>,
) -> HashMap<String, HashMap<feature::FeaturePlace, feature::FeatureResolution>> {
    let mut sealed = HashMap::new();
    for declaration in &raw.declarations {
        let Declaration::Construction(construction) = declaration else { continue };
        let local_equations = equations
            .get(&identifier_key(&construction.name))
            .map_or(&[] as &[feature::FeatureEquation], Vec::as_slice);
        let invariant = &invariants
            .get(&identifier_key(&construction.name))
            .expect("validated invariant is present")
            .1;
        let mut places = local_equations
            .iter()
            .map(|equation| equation.target().clone())
            .collect::<Vec<_>>();
        if construction.form.atoms.iter().any(|atom| {
            matches!(
                atom,
                FormAtom::Verb(VerbOperand::Fixed(_)) | FormAtom::OpenVerb(_)
            )
        }) {
            places.push(feature::FeaturePlace::Role {
                field: syn::Ident::new("verb", construction.form.name.span()),
                feature: feature::Feature::Agreement,
            });
        }
        let mut resolutions = HashMap::new();
        for place in places {
            let resolution = resolve_local_feature(
                construction,
                invariant,
                local_equations,
                &place,
                &mut HashSet::new(),
            );
            resolutions.insert(place, resolution);
        }
        sealed.insert(identifier_key(&construction.name), resolutions);
    }
    sealed
}

fn fold_unit_invariants(
    raw: &Declarations,
    invariants: &mut HashMap<String, (proc_macro2::Span, InvariantPlan)>,
    resolutions: &HashMap<String, HashMap<feature::FeaturePlace, feature::FeatureResolution>>,
) -> syn::Result<()> {
    for declaration in &raw.declarations {
        let Declaration::Construction(construction) = declaration else { continue };
        if !construction.element.fields.is_empty() {
            continue;
        }
        let construction_id = identifier_key(&construction.name);
        let invariant = &mut invariants
            .get_mut(&construction_id)
            .expect("validated unit invariant is present")
            .1;
        invariant.constant_fold_unit(&construction.name, resolutions.get(&construction_id))?;
    }
    Ok(())
}

fn seal_category_render_capabilities(
    raw: &Declarations,
    resolutions: &HashMap<String, HashMap<feature::FeaturePlace, feature::FeatureResolution>>,
) -> HashMap<String, CategoryRenderCapability> {
    let constructions = raw
        .declarations
        .iter()
        .filter_map(|declaration| match declaration {
            Declaration::Construction(construction) => Some(construction),
            _ => None,
        })
        .collect::<Vec<_>>();
    let providers = feature_providers(raw);
    let mut agreement_contextual = HashSet::new();

    for construction in &constructions {
        let has_fixed_verb = construction.form.atoms.iter().any(|atom| {
            matches!(
                atom,
                FormAtom::Verb(VerbOperand::Fixed(_)) | FormAtom::OpenVerb(_)
            )
        });
        let verb_place = feature::FeaturePlace::Role {
            field: syn::Ident::new("verb", construction.form.name.span()),
            feature: feature::Feature::Agreement,
        };
        if has_fixed_verb
            && resolutions
                .get(&identifier_key(&construction.name))
                .and_then(|values| values.get(&verb_place))
                == Some(&feature::FeatureResolution::External)
        {
            agreement_contextual.insert(path_name(&construction.category));
        }
    }

    loop {
        let before = agreement_contextual.len();
        for construction in &constructions {
            let passes_external_to_child = construction.form.atoms.iter().any(|atom| {
                let FormAtom::Role(role) = atom else { return false };
                let Some(category) = construction.element.fields.iter().find_map(|field| {
                    same_identifier(&field.name, role)
                        .then_some(&field.kind)
                        .and_then(|kind| match kind {
                            FieldKind::Category(category) => Some(path_name(category)),
                            FieldKind::Lex(_)
                            | FieldKind::Identity(_)
                            | FieldKind::Optional(_)
                            | FieldKind::Sequence { .. } => None,
                        })
                }) else {
                    return false;
                };
                let place = feature::FeaturePlace::Role {
                    field: role.clone(),
                    feature: feature::Feature::Agreement,
                };
                agreement_contextual.contains(&category)
                    && construction.equations.iter().any(|equation| {
                        matches!(
                            &equation.target,
                            ParsedFeaturePlace::Role {
                                field,
                                feature: ParsedFeature::Agreement,
                            } if same_identifier(field, role)
                        )
                    })
                    && resolutions
                        .get(&identifier_key(&construction.name))
                        .and_then(|values| values.get(&place))
                        == Some(&feature::FeatureResolution::External)
            });
            if passes_external_to_child {
                agreement_contextual.insert(path_name(&construction.category));
            }
        }
        if agreement_contextual.len() == before {
            break;
        }
    }

    let mut context_required = HashSet::new();
    loop {
        let before = context_required.len();
        for construction in &constructions {
            let reads_context = construction.form.atoms.iter().any(|atom| match atom {
                FormAtom::Identity(_) => true,
                FormAtom::Role(role) => construction
                    .element
                    .fields
                    .iter()
                    .find(|field| same_identifier(&field.name, role))
                    .is_some_and(|field| {
                        matches!(&field.kind, FieldKind::Category(path) if context_required.contains(&path_name(path)))
                    }),
                FormAtom::Literal(_)
                | FormAtom::Lex(_)
                | FormAtom::Verb(_)
                | FormAtom::OpenVerb(_)
                | FormAtom::Noun(_) => false,
            });
            if reads_context {
                context_required.insert(path_name(&construction.category));
            }
        }
        if context_required.len() == before {
            break;
        }
    }

    let mut capabilities = HashMap::new();
    for construction in constructions {
        let category = path_name(&construction.category);
        capabilities
            .entry(category.clone())
            .or_insert_with(CategoryRenderCapability::default)
            .requires_external_input = agreement_contextual.contains(&category);
    }
    for (category, capability) in &mut capabilities {
        capability.carries_output =
            providers.contains(&(category.clone(), ParsedFeature::Agreement));
        capability.requires_context = context_required.contains(category);
    }
    capabilities
}

fn resolve_local_feature(
    construction: &crate::Construction,
    invariant: &InvariantPlan,
    equations: &[feature::FeatureEquation],
    place: &feature::FeaturePlace,
    visiting: &mut HashSet<feature::FeaturePlace>,
) -> feature::FeatureResolution {
    if !visiting.insert(place.clone()) {
        return feature::FeatureResolution::Runtime;
    }
    let resolved = equation_for_place(equations, place).map_or_else(
        || match place {
            feature::FeaturePlace::Role {
                field,
                feature: feature::Feature::Agreement,
            } if identifier_key(field) == "verb"
                && !construction
                    .element
                    .fields
                    .iter()
                    .any(|candidate| same_identifier(&candidate.name, field)) =>
            {
                feature::FeatureResolution::External
            }
            feature::FeaturePlace::Construction(_) | feature::FeaturePlace::Role { .. } => {
                feature::FeatureResolution::Runtime
            }
        },
        |equation| match equation.value() {
            feature::FeatureExpr::Constant(value) => {
                feature::FeatureResolution::Known(*value.value())
            }
            feature::FeatureExpr::FromRole { role, feature } => resolve_local_feature(
                construction,
                invariant,
                equations,
                &feature::FeaturePlace::Role {
                    field: role.clone(),
                    feature: *feature,
                },
                visiting,
            ),
            feature::FeatureExpr::MatchVocab { role, arms } => {
                let refined = known_invariant_variant(invariant, &identifier_key(role)).and_then(
                    |refinement_variant| {
                        arms.iter()
                            .find(|(variant, _)| {
                                same_identifier(variant.value(), refinement_variant)
                            })
                            .map(|(_, value)| *value)
                    },
                );
                let uniform = arms
                    .first()
                    .map(|(_, first)| *first)
                    .filter(|first| arms.iter().all(|(_, value)| value == first));
                refined.or(uniform).map_or(
                    feature::FeatureResolution::Runtime,
                    feature::FeatureResolution::Known,
                )
            }
        },
    );
    visiting.remove(place);
    resolved
}

fn known_invariant_variant<'a>(invariant: &'a InvariantPlan, role: &str) -> Option<&'a syn::Ident> {
    let [alternative] = invariant.alternatives() else {
        return None;
    };
    alternative.atoms().iter().find_map(|atom| {
        let [PredicateMemberPlan::Variant(member)] = atom.allowed() else {
            return None;
        };
        atom.subject()
            .role()
            .is_some_and(|candidate| identifier_key(candidate) == role)
            .then_some(member)
    })
}

fn equation_for_place<'a>(
    equations: &'a [feature::FeatureEquation],
    place: &feature::FeaturePlace,
) -> Option<&'a feature::FeatureEquation> {
    equations.iter().find(|equation| equation.target() == place)
}

fn validate_contextual_agreement_uses(
    raw: &Declarations,
    capabilities: &HashMap<String, CategoryRenderCapability>,
) -> syn::Result<()> {
    let mut errors = None;
    for declaration in &raw.declarations {
        let Declaration::Construction(construction) = declaration else { continue };
        for atom in &construction.form.atoms {
            let FormAtom::Role(role) = atom else { continue };
            let Some(category) = construction.element.fields.iter().find_map(|field| {
                same_identifier(&field.name, role)
                    .then_some(&field.kind)
                    .and_then(|kind| match kind {
                        FieldKind::Category(category) => Some(path_name(category)),
                        FieldKind::Lex(_)
                        | FieldKind::Identity(_)
                        | FieldKind::Optional(_)
                        | FieldKind::Sequence { .. } => None,
                    })
            }) else {
                continue;
            };
            if !capabilities
                .get(&category)
                .is_some_and(|capability| capability.requires_external_input)
            {
                continue;
            }
            let has_writer = construction.equations.iter().any(|equation| {
                matches!(
                    &equation.target,
                    ParsedFeaturePlace::Role {
                        field,
                        feature: ParsedFeature::Agreement,
                    } if same_identifier(field, role)
                )
            });
            if !has_writer {
                combine(
                    &mut errors,
                    syn::Error::new(
                        role.span(),
                        format!(
                            "contextual category `{category}` requires a `{role}.agreement` writer at every bare role use"
                        ),
                    ),
                );
            }
        }
    }
    finish(errors)
}

fn feature_providers(raw: &Declarations) -> HashSet<(String, ParsedFeature)> {
    let mut categories: HashMap<String, Vec<&crate::model::Construction>> = HashMap::new();
    for declaration in &raw.declarations {
        if let Declaration::Construction(construction) = declaration {
            categories
                .entry(path_name(&construction.category))
                .or_default()
                .push(construction);
        }
    }
    let mut providers = HashSet::new();
    for (category, constructions) in categories {
        for feature in [ParsedFeature::Agreement, ParsedFeature::Number] {
            if constructions.iter().all(|construction| construction.equations.iter().any(|equation| matches!(equation.target, ParsedFeaturePlace::Construction(found) if found == feature))) {
                providers.insert((category.clone(), feature));
            }
        }
    }
    providers
}

fn place_key(place: &ParsedFeaturePlace) -> String {
    match place {
        ParsedFeaturePlace::Construction(feature) => {
            format!("construction.{}", feature_name(*feature))
        }
        ParsedFeaturePlace::Role { field, feature } => {
            format!("{}.{}", identifier_key(field), feature_name(*feature))
        }
    }
}

fn feature_name(feature: ParsedFeature) -> &'static str {
    match feature {
        ParsedFeature::Agreement => "agreement",
        ParsedFeature::Number => "number",
    }
}

fn contains_cycle(edges: &HashMap<String, Vec<String>>) -> bool {
    fn visit(
        node: &str,
        edges: &HashMap<String, Vec<String>>,
        visiting: &mut HashSet<String>,
        done: &mut HashSet<String>,
    ) -> bool {
        if done.contains(node) {
            return false;
        }
        if !visiting.insert(node.to_owned()) {
            return true;
        }
        if edges.get(node).is_some_and(|next| {
            next.iter()
                .any(|child| edges.contains_key(child) && visit(child, edges, visiting, done))
        }) {
            return true;
        }
        visiting.remove(node);
        done.insert(node.to_owned());
        false
    }
    let mut visiting = HashSet::new();
    let mut done = HashSet::new();
    edges
        .keys()
        .any(|node| visit(node, edges, &mut visiting, &mut done))
}

fn validate_category_graph(raw: &Declarations) -> HashSet<(String, String)> {
    let mut graph: HashMap<String, HashSet<String>> = HashMap::new();
    for declaration in &raw.declarations {
        if let Declaration::Construction(construction) = declaration {
            let category = path_name(&construction.category);
            let entry = graph.entry(category).or_default();
            for field in &construction.element.fields {
                if let FieldKind::Category(path) = &field.kind {
                    entry.insert(path_name(path));
                }
            }
        }
    }
    let reaches = |from: &str, target: &str| {
        let mut todo = vec![from.to_owned()];
        let mut seen = HashSet::new();
        while let Some(node) = todo.pop() {
            if node == target {
                return true;
            }
            if seen.insert(node.clone()) {
                todo.extend(graph.get(&node).into_iter().flatten().cloned());
            }
        }
        false
    };
    let mut boxed = HashSet::new();
    for declaration in &raw.declarations {
        if let Declaration::Construction(construction) = declaration {
            let owner = path_name(&construction.category);
            for field in &construction.element.fields {
                if let FieldKind::Category(path) = &field.kind {
                    let target = path_name(path);
                    if reaches(&target, &owner) {
                        boxed.insert((
                            identifier_key(&construction.name),
                            identifier_key(&field.name),
                        ));
                    }
                }
            }
        }
    }
    boxed
}

fn validate_roots(
    raw: &Declarations,
    symbols: &Symbols,
    capabilities: &HashMap<String, CategoryRenderCapability>,
) -> syn::Result<()> {
    let mut errors = None;
    let roots: Vec<_> = raw
        .declarations
        .iter()
        .filter_map(|declaration| match declaration {
            Declaration::Root(root) => Some(root),
            _ => None,
        })
        .collect();
    if roots.is_empty() {
        combine(
            &mut errors,
            syn::Error::new(
                proc_macro2::Span::call_site(),
                "declarations require at least one root",
            ),
        );
        return finish(errors);
    }
    let mut seen = HashSet::new();
    for root in &roots {
        let category = path_name(&root.category);
        if !symbols.categories.contains(&category) && !symbols.structural_types.contains(&category)
        {
            combine(
                &mut errors,
                syn::Error::new_spanned(
                    &root.category,
                    format!("unknown root category `{category}`"),
                ),
            );
        }
        if !seen.insert(category.clone()) {
            combine(
                &mut errors,
                syn::Error::new_spanned(
                    &root.category,
                    format!("duplicate root for category `{category}`"),
                ),
            );
        }
        let Some(punctuation) = root.punctuation.as_ref() else {
            continue;
        };
        if punctuation.value().chars().count() != 1 {
            combine(
                &mut errors,
                syn::Error::new(
                    punctuation.span(),
                    "root punctuation must be exactly one Unicode scalar",
                ),
            );
        }
        if root.standalone_render
            && capabilities
                .get(&category)
                .is_some_and(|capability| capability.requires_external_input)
        {
            combine(
                &mut errors,
                syn::Error::new_spanned(
                    &root.category,
                    format!("standalone render root `{category}` requires external agreement"),
                ),
            );
        }
    }
    if !roots.iter().any(|root| root.eoi) {
        combine(
            &mut errors,
            syn::Error::new_spanned(
                &roots[0].category,
                "roots require at least one end-of-input parse entry",
            ),
        );
    }
    if !roots.iter().any(|root| root.standalone_render) {
        combine(
            &mut errors,
            syn::Error::new_spanned(
                &roots[0].category,
                "roots require at least one standalone render entry",
            ),
        );
    }
    finish(errors)
}

fn validate_backend_completeness(
    raw: &Declarations,
    resolved: &ResolvedGrammar,
) -> syn::Result<()> {
    let mut errors = None;
    let mut terminals = Vec::new();

    validate_lowerable_backend_shapes(raw)?;

    for declaration in &raw.declarations {
        match declaration {
            Declaration::Construction(_)
            | Declaration::AbstractProduct(_)
            | Declaration::AbstractSum(_)
            | Declaration::Morphology(_)
            | Declaration::Root(_) => {}
            Declaration::Vocab(vocab) => terminals.push(TerminalCapabilities {
                name: identifier_key(&vocab.name),
                lex_atom: true,
                identity_atom: false,
                noun_atom: false,
                verb_atom: false,
                direct_render: true,
                direct_build: true,
                traversal: true,
            }),
            Declaration::Lexeme(lexeme) => terminals.push(TerminalCapabilities {
                name: identifier_key(&lexeme.name),
                lex_atom: false,
                identity_atom: false,
                noun_atom: false,
                verb_atom: resolved
                    .verb_lexeme_provider
                    .as_ref()
                    .is_some_and(|provider| provider == &identifier_key(&lexeme.name)),
                direct_render: false,
                direct_build: false,
                traversal: true,
            }),
            Declaration::Codec(binding) => terminals.push(TerminalCapabilities {
                name: identifier_key(&binding.name),
                lex_atom: binding.codec_atom == Some(CodecAtomClass::Lex),
                identity_atom: false,
                noun_atom: binding.codec_atom == Some(CodecAtomClass::Noun),
                verb_atom: false,
                direct_render: binding.generated.is_some() || binding.render.is_some(),
                direct_build: binding.generated.is_some() || binding.build.is_some(),
                traversal: binding.generated.is_some() || closed_traversal_is_lowerable(binding),
            }),
            Declaration::Identity(binding) => {
                let generated = binding.generated_identity.is_some();
                terminals.push(TerminalCapabilities {
                    name: identifier_key(&binding.name),
                    lex_atom: false,
                    identity_atom: true,
                    noun_atom: false,
                    verb_atom: false,
                    direct_render: generated || binding.render.is_some(),
                    direct_build: generated || binding.build.is_some(),
                    traversal: generated || closed_traversal_is_lowerable(binding),
                });
            }
        }
    }

    if !raw
        .declarations
        .iter()
        .any(|declaration| matches!(declaration, Declaration::Construction(_)))
    {
        combine(
            &mut errors,
            syn::Error::new(
                proc_macro2::Span::call_site(),
                "backend requires construction contributions",
            ),
        );
    }
    let terminal_capabilities: HashMap<_, _> = terminals
        .iter()
        .map(|terminal| (terminal.name.as_str(), terminal))
        .collect();
    for declaration in &raw.declarations {
        let Declaration::Construction(construction) = declaration else {
            continue;
        };
        let construction_id = identifier_key(&construction.name);
        let Some((_, atoms)) = resolved.atoms_by_construction.get(&construction_id) else {
            combine(
                &mut errors,
                syn::Error::new(
                    construction.name.span(),
                    format!("construction `{construction_id}` is missing resolved backend atoms"),
                ),
            );
            continue;
        };
        if atoms.len() != construction.form.atoms.len()
            || atoms
                .iter()
                .any(|atom| !atom.is_complete() || !atom.is_supported_by(&terminal_capabilities))
        {
            combine(
                &mut errors,
                syn::Error::new(
                    construction.name.span(),
                    format!("construction `{construction_id}` is missing a resolved backend fact"),
                ),
            );
        }
    }
    finish(errors)
}

fn validate_lowerable_backend_shapes(raw: &Declarations) -> syn::Result<()> {
    let mut errors = None;

    for declaration in &raw.declarations {
        match declaration {
            Declaration::Construction(construction) => {
                for atom in &construction.form.atoms {
                    if let FormAtom::Verb(VerbOperand::Projected(role)) = atom {
                        combine(
                            &mut errors,
                            syn::Error::new(
                                role.span(),
                                "unimplemented in MVP: `projected verb role`; backend closure requires a fixed verb path",
                            ),
                        );
                    }
                }
                validate_lowerable_feature_compositions(construction, &mut errors);
            }
            Declaration::Codec(binding) | Declaration::Identity(binding) => {
                if binding.generated.is_none()
                    && binding.generated_identity.is_none()
                    && (!binding_value_type_is_lowerable(binding)
                        || !closed_traversal_is_lowerable(binding))
                {
                    combine(
                        &mut errors,
                        syn::Error::new(
                            binding.name.span(),
                            format!(
                                "binding `{}` is outside the lowerable MVP value/traversal shape",
                                binding.name
                            ),
                        ),
                    );
                }
            }
            Declaration::Root(root) => {
                let Some(punctuation) = root.punctuation.as_ref() else {
                    continue;
                };
                let punctuation = punctuation.value();
                let mut characters = punctuation.chars();
                match (characters.next(), characters.next()) {
                    (Some(character), None) if character.is_alphanumeric() => combine(
                        &mut errors,
                        syn::Error::new(
                            root.punctuation
                                .as_ref()
                                .expect("root punctuation checked")
                                .span(),
                            "root punctuation must not be alphanumeric",
                        ),
                    ),
                    (Some(_), None) => {}
                    _ => combine(
                        &mut errors,
                        syn::Error::new(
                            root.punctuation
                                .as_ref()
                                .expect("root punctuation checked")
                                .span(),
                            "root punctuation must be exactly one Unicode scalar",
                        ),
                    ),
                }
            }
            Declaration::AbstractProduct(_)
            | Declaration::AbstractSum(_)
            | Declaration::Vocab(_)
            | Declaration::Morphology(_)
            | Declaration::Lexeme(_) => {}
        }
    }
    validate_category_feature_uniformity(raw, &mut errors);
    if let Some(errors) = errors.take() {
        return Err(errors);
    }
    Ok(())
}

fn validate_lowerable_feature_compositions(
    construction: &crate::Construction,
    errors: &mut Option<syn::Error>,
) {
    let fields = construction
        .element
        .fields
        .iter()
        .map(|field| (identifier_key(&field.name), &field.kind))
        .collect::<HashMap<_, _>>();
    let has_noun = construction
        .form
        .atoms
        .iter()
        .any(|atom| matches!(atom, FormAtom::Noun(_)));
    let matched_role = |feature| {
        construction.equations.iter().find_map(|equation| {
            matches!(&equation.target, ParsedFeaturePlace::Construction(found) if *found == feature)
                .then_some(&equation.value)
                .and_then(|value| match value {
                    ParsedFeatureValue::Match { role, .. } => Some(role),
                    ParsedFeatureValue::Constant(_) | ParsedFeatureValue::FromRole(_) => None,
                })
        })
    };
    let agreement_match_role = matched_role(ParsedFeature::Agreement);
    let number_match_role = matched_role(ParsedFeature::Number);
    if agreement_match_role
        .zip(number_match_role)
        .is_some_and(|(agreement, number)| !same_identifier(agreement, number))
    {
        combine(
            errors,
            syn::Error::new(
                construction.name.span(),
                "unimplemented in MVP: `feature equation composition`; construction agreement and number matches must use the same vocabulary role",
            ),
        );
    }

    for equation in &construction.equations {
        let lowerable = match (&equation.target, &equation.value) {
            (
                ParsedFeaturePlace::Construction(_),
                ParsedFeatureValue::Constant(_) | ParsedFeatureValue::Match { .. },
            )
            | (
                ParsedFeaturePlace::Construction(ParsedFeature::Agreement),
                ParsedFeatureValue::FromRole(_),
            ) => true,
            (
                ParsedFeaturePlace::Construction(ParsedFeature::Number),
                ParsedFeatureValue::FromRole(source),
            ) => {
                !has_noun
                    || matches!(
                        fields.get(&identifier_key(&source.role)),
                        Some(FieldKind::Category(_))
                    )
            }
            (
                ParsedFeaturePlace::Role {
                    field,
                    feature: ParsedFeature::Agreement,
                },
                ParsedFeatureValue::Constant(_) | ParsedFeatureValue::FromRole(_),
            ) => {
                identifier_key(field) == "verb"
                    || matches!(
                        fields.get(&identifier_key(field)),
                        Some(FieldKind::Category(_))
                    )
            }
            (ParsedFeaturePlace::Role { field, .. }, ParsedFeatureValue::Match { role, .. }) => {
                same_identifier(field, role)
                    && matches!(fields.get(&identifier_key(field)), Some(FieldKind::Lex(_)))
            }
            (
                ParsedFeaturePlace::Role {
                    feature: ParsedFeature::Number,
                    ..
                },
                ParsedFeatureValue::Constant(_) | ParsedFeatureValue::FromRole(_),
            ) => false,
        };
        if !lowerable {
            let span = match &equation.target {
                ParsedFeaturePlace::Construction(_) => construction.name.span(),
                ParsedFeaturePlace::Role { field, .. } => field.span(),
            };
            combine(
                errors,
                syn::Error::new(
                    span,
                    "unimplemented in MVP: `feature equation composition` is not lowerable by every backend",
                ),
            );
        }
    }
}

fn validate_category_feature_uniformity(raw: &Declarations, errors: &mut Option<syn::Error>) {
    let mut categories: Vec<(String, Vec<&crate::Construction>)> = Vec::new();
    for construction in raw
        .declarations
        .iter()
        .filter_map(|declaration| match declaration {
            Declaration::Construction(construction) => Some(construction),
            _ => None,
        })
    {
        let category = path_name(&construction.category);
        if let Some((_, members)) = categories.iter_mut().find(|(name, _)| name == &category) {
            members.push(construction);
        } else {
            categories.push((category, vec![construction]));
        }
    }

    for (category, members) in categories {
        for feature in [ParsedFeature::Agreement, ParsedFeature::Number] {
            let provides = |construction: &crate::Construction| {
                construction.equations.iter().any(|equation| {
                    matches!(
                        equation.target,
                        ParsedFeaturePlace::Construction(found) if found == feature
                    )
                })
            };
            if !members.iter().any(|construction| provides(construction)) {
                continue;
            }
            for construction in members
                .iter()
                .copied()
                .filter(|construction| !provides(construction))
            {
                combine(
                    errors,
                    syn::Error::new(
                        construction.name.span(),
                        format!(
                            "unimplemented in MVP: `category feature provider uniformity`; category `{category}` construction `{}` must provide `{}`",
                            construction.name,
                            parsed_feature_name(feature),
                        ),
                    ),
                );
            }
        }
    }
}

fn parsed_feature_name(feature: ParsedFeature) -> &'static str {
    match feature {
        ParsedFeature::Agreement => "agreement",
        ParsedFeature::Number => "number",
    }
}

fn binding_value_type_is_lowerable(binding: &TerminalBinding) -> bool {
    let syn::Type::Path(value_type) = &binding.value_type else { return false };
    value_type.qself.is_none()
        && !value_type.path.segments.is_empty()
        && value_type
            .path
            .segments
            .iter()
            .all(|segment| matches!(segment.arguments, syn::PathArguments::None))
        && value_type
            .path
            .segments
            .last()
            .is_some_and(|segment| same_identifier(&segment.ident, &binding.name))
}

fn closed_traversal_is_lowerable(binding: &TerminalBinding) -> bool {
    let traversal = &binding.traversal;
    let enum_body = !traversal.variants.is_empty() && traversal.branches.is_empty();
    let calls_body = !traversal.calls.is_empty();
    let match_body = !traversal.branches.is_empty();
    traversal.parts.is_empty()
        && traversal.visit_order.is_empty()
        && traversal.callback_mode.is_some()
        && traversal.argument.is_some()
        && [enum_body, calls_body, match_body]
            .into_iter()
            .filter(|present| *present)
            .count()
            == 1
}

fn path_name(path: &syn::Path) -> String {
    path_key(path)
}

fn combine(errors: &mut Option<syn::Error>, error: syn::Error) {
    if let Some(errors) = errors {
        errors.combine(error);
    } else {
        *errors = Some(error);
    }
}

fn finish(errors: Option<syn::Error>) -> syn::Result<()> {
    errors.map_or(Ok(()), Err)
}

#[cfg(test)]
pub(crate) mod tests {
    #![allow(
        clippy::too_many_lines,
        reason = "validation fixtures pin complete closed-schema diagnostics and golden input"
    )]
    use std::collections::BTreeSet;
    use std::collections::HashSet;

    use quote::quote;
    use syn::spanned::Spanned;

    use crate::Declaration;
    use crate::FormAtom;

    fn validate(tokens: proc_macro2::TokenStream) -> syn::Result<super::ValidatedDeclarations> {
        super::validate_declarations(crate::parse_declarations(tokens)?)
    }

    fn error(tokens: proc_macro2::TokenStream) -> String {
        validate(tokens)
            .expect_err("fixture must be invalid")
            .into_compile_error()
            .to_string()
    }

    fn assert_same_span(actual: proc_macro2::Span, expected: proc_macro2::Span) {
        assert_eq!(format!("{actual:?}"), format!("{expected:?}"));
    }

    #[test]
    fn generated_morphology_rejects_unknown_morphology_name() {
        let actual = error(quote! {
            lexeme VerbLexeme using Missing { Deal = "deal", }
            construction action: Ability {
                element Action {}
                derive agreement = verb.agreement;
                derive verb.agreement = Values::Bare;
                form action = verb(VerbLexeme::Deal);
            }
            root Ability { punctuation = "."; eoi = true; standalone_render = true; }
        });

        assert!(actual.contains("unknown morphology `Missing`"), "{actual}");
    }

    #[test]
    fn generated_morphology_rejects_unknown_recipe_and_axis_mismatch() {
        let unknown = error(quote! {
            morphology EnglishVerb { feature = Agreement; recipe = conjectural_verb; }
            lexeme VerbLexeme using EnglishVerb { Deal = "deal", }
            construction action: Ability {
                element Action {}
                derive agreement = verb.agreement;
                derive verb.agreement = Values::Bare;
                form action = verb(VerbLexeme::Deal);
            }
            root Ability { punctuation = "."; eoi = true; standalone_render = true; }
        });
        assert!(
            unknown.contains("unknown morphology recipe `conjectural_verb`"),
            "{unknown}"
        );

        let mismatch = error(quote! {
            morphology EnglishVerb { feature = Number; recipe = english_verb; }
            lexeme VerbLexeme using EnglishVerb { Deal = "deal", }
            construction action: Ability {
                element Action {}
                derive agreement = Values::Bare;
                form action = verb(VerbLexeme::Deal);
            }
            root Ability { punctuation = "."; eoi = true; standalone_render = true; }
        });
        assert!(
            mismatch.contains("morphology `EnglishVerb` feature axis does not match recipe"),
            "{mismatch}"
        );
    }

    #[test]
    fn generated_morphology_rejects_duplicate_declarations_members_and_overrides() {
        let declarations = error(quote! {
            morphology EnglishVerb { feature = Agreement; recipe = english_verb; }
            morphology EnglishVerb { feature = Agreement; recipe = english_verb; }
        });
        assert!(
            declarations.contains("duplicate") && declarations.contains("EnglishVerb"),
            "{declarations}"
        );

        let members = error(quote! {
            morphology EnglishVerb { feature = Agreement; recipe = english_verb; }
            lexeme VerbLexeme using EnglishVerb { Deal = "deal", Deal = "deal", }
        });
        assert!(
            members.contains("duplicate lexeme member `Deal`"),
            "{members}"
        );

        let overrides = error(quote! {
            morphology EnglishVerb { feature = Agreement; recipe = english_verb; }
            lexeme VerbLexeme using EnglishVerb {
                Deal = "deal" { Bare = "deal", Bare = "deal", },
            }
        });
        assert!(
            overrides.contains("duplicate override `Bare`"),
            "{overrides}"
        );
    }

    #[test]
    fn generated_morphology_rejects_unknown_override_feature() {
        let actual = error(quote! {
            morphology EnglishVerb { feature = Agreement; recipe = english_verb; }
            lexeme VerbLexeme using EnglishVerb {
                Deal = "deal" { Singular = "deal", },
            }
        });

        assert!(
            actual.contains("unknown override feature `Singular`"),
            "{actual}"
        );
    }

    #[test]
    fn generated_morphology_rejects_empty_lemma_and_override_surface() {
        for (source, expected) in [
            (
                quote! {
                    morphology EnglishVerb { feature = Agreement; recipe = english_verb; }
                    lexeme VerbLexeme using EnglishVerb { Deal = "", }
                },
                "lexeme lemma must not be empty",
            ),
            (
                quote! {
                    morphology EnglishVerb { feature = Agreement; recipe = english_verb; }
                    lexeme VerbLexeme using EnglishVerb {
                        Deal = "deal" { Bare = "", },
                    }
                },
                "lexeme override surface must not be empty",
            ),
        ] {
            let actual = error(source);
            assert!(actual.contains(expected), "{actual}");
        }
    }

    #[test]
    fn generated_morphology_seals_complete_source_ordered_replacement_rows() {
        let semantic = validate(quote! {
            morphology EnglishVerb { feature = Agreement; recipe = english_verb; }
            lexeme VerbLexeme using EnglishVerb {
                Deal = "deal",
                Be = "be" {
                    Bare = "are",
                    ThirdPersonSingular = "is",
                },
            }
            construction action: Ability {
                element Action {}
                derive agreement = verb.agreement;
                derive verb.agreement = Values::Bare;
                form action = verb(VerbLexeme::Deal);
            }
            root Ability { punctuation = "."; eoi = true; standalone_render = true; }
        })
        .expect("the sealed morphology validates")
        .into_semantic();
        let lexeme = semantic
            .terminals()
            .iter()
            .find_map(|terminal| match terminal {
                crate::semantic::TerminalPlan::Lexeme(lexeme) => Some(lexeme),
                _ => None,
            })
            .expect("the lexeme has a semantic row");

        assert_eq!(
            lexeme
                .surfaces()
                .iter()
                .map(|row| (row.member(), row.feature(), row.surface()))
                .collect::<Vec<_>>(),
            [
                ("Deal", macro_ron::v2::SurfaceFeature::Bare, "deal"),
                (
                    "Deal",
                    macro_ron::v2::SurfaceFeature::ThirdPersonSingular,
                    "deals",
                ),
                ("Be", macro_ron::v2::SurfaceFeature::Bare, "are"),
                (
                    "Be",
                    macro_ron::v2::SurfaceFeature::ThirdPersonSingular,
                    "is",
                ),
            ]
        );
        assert_eq!(
            lexeme
                .surfaces()
                .iter()
                .map(|row| (row.member(), row.feature(), row.surface()))
                .collect::<HashSet<_>>()
                .len(),
            lexeme.surfaces().len(),
            "the sealed plan cannot contain a duplicate exact row"
        );
        assert!(
            !lexeme
                .surfaces()
                .iter()
                .any(|row| { row.member() == "Be" && matches!(row.surface(), "be" | "bes") })
        );
    }

    #[test]
    fn generated_morphology_uses_lemmas_not_variant_name_heuristics() {
        let semantic = validate(quote! {
            morphology EnglishVerb { feature = Agreement; recipe = english_verb; }
            lexeme VerbLexeme using EnglishVerb { TwoWords = "unrelated", }
            construction action: Ability {
                element Action {}
                derive agreement = verb.agreement;
                derive verb.agreement = Values::Bare;
                form action = verb(VerbLexeme::TwoWords);
            }
            root Ability { punctuation = "."; eoi = true; standalone_render = true; }
        })
        .expect("an explicit lemma seals independently of its variant")
        .into_semantic();
        let crate::semantic::TerminalPlan::Lexeme(lexeme) = &semantic.terminals()[0] else {
            panic!("the only terminal is the lexeme")
        };

        assert_eq!(
            lexeme
                .surfaces()
                .iter()
                .map(crate::semantic::LexemeSurfacePlan::surface)
                .collect::<Vec<_>>(),
            ["unrelated", "unrelateds"]
        );
    }

    #[test]
    fn generated_morphology_seals_resolved_morphology_and_ordered_irregulars() {
        let semantic = validate(quote! {
            morphology EnglishVerb { feature = Agreement; recipe = english_verb; }
            lexeme VerbLexeme using EnglishVerb {
                Deal = "deal",
                Be = "be" {
                    ThirdPersonSingular = "is",
                    Bare = "are",
                },
            }
            construction action: Ability {
                element Action {}
                derive agreement = verb.agreement;
                derive verb.agreement = Values::Bare;
                form action = verb(VerbLexeme::Deal);
            }
            root Ability { punctuation = "."; eoi = true; standalone_render = true; }
        })
        .expect("resolved morphology validates")
        .into_semantic();

        let morphology = &semantic.morphologies()[0];
        assert_eq!(morphology.name(), "EnglishVerb");
        assert_eq!(morphology.feature(), crate::Feature::Agreement);
        assert_eq!(
            morphology.recipe(),
            crate::morphology::MorphologyRecipe::EnglishVerb
        );
        let crate::semantic::TerminalPlan::Lexeme(lexeme) = &semantic.terminals()[0] else {
            panic!("the only terminal is the lexeme")
        };
        assert_eq!(lexeme.morphology().name(), "EnglishVerb");
        assert_eq!(lexeme.irregulars().len(), 1);
        assert_eq!(lexeme.irregulars()[0].member(), "Be");
        assert_eq!(
            lexeme.irregulars()[0]
                .overrides()
                .iter()
                .map(|row| (row.feature(), row.surface()))
                .collect::<Vec<_>>(),
            [
                (macro_ron::v2::SurfaceFeature::ThirdPersonSingular, "is",),
                (macro_ron::v2::SurfaceFeature::Bare, "are"),
            ]
        );
    }

    fn signed_decimal_error(body: &proc_macro2::TokenStream) -> String {
        error(quote! {
            codec SignedNumber {
                generate signed_decimal { #body }
            }
            construction only: Cat { element Only {} form only = "only"; }
            root Cat { punctuation = "."; eoi = true; standalone_render = true; }
        })
    }

    fn context_identity_error(body: &proc_macro2::TokenStream) -> String {
        error(quote! {
            identity SelfReferenceSpelling {
                generate context { #body }
            }
            construction only: Cat {
                element Only { spelling: identity SelfReferenceSpelling, }
                form only = identity(spelling);
            }
            root Cat { punctuation = "."; eoi = true; standalone_render = true; }
        })
    }

    fn declaration_noun_error(body: &proc_macro2::TokenStream) -> String {
        error(quote! {
            morphology EnglishNoun { feature = Number; recipe = english_noun; }
            lexeme NounLexeme using EnglishNoun { Player = "player", }
            codec Noun {
                generate declaration_noun { #body }
            }
            construction only: Cat { element Only {} form only = "only"; }
            root Cat { punctuation = "."; eoi = true; standalone_render = true; }
        })
    }

    #[test]
    fn declaration_noun_recipe_validation_is_closed_and_structural() {
        validate(quote! {
            morphology EnglishNoun { feature = Number; recipe = english_noun; }
            lexeme NounLexeme using EnglishNoun { Player = "player", }
            codec Noun {
                generate declaration_noun {
                    closed = NounLexeme;
                    position = Noun;
                    kinds = [Type, Subtype];
                    feature = Number;
                }
            }
            construction only: Cat { element Only {} form only = "only"; }
            root Cat { punctuation = "."; eoi = true; standalone_render = true; }
        })
        .expect("the exact declaration noun recipe validates");

        for (body, expected) in [
            (
                quote! {
                    position = Noun;
                    kinds = [Type, Subtype];
                    feature = Number;
                },
                "declaration_noun requires one `closed` field",
            ),
            (
                quote! {
                    closed = NounLexeme;
                    closed = NounLexeme;
                    position = Noun;
                    kinds = [Type, Subtype];
                    feature = Number;
                },
                "duplicate declaration_noun field `closed`",
            ),
            (
                quote! {
                    closed = MissingLexeme;
                    position = Noun;
                    kinds = [Type, Subtype];
                    feature = Number;
                },
                "closed branch must name a lexeme declaration",
            ),
            (
                quote! {
                    closed = NounLexeme;
                    position = Verb;
                    kinds = [Type, Subtype];
                    feature = Number;
                },
                "position must be `Noun`",
            ),
            (
                quote! {
                    closed = NounLexeme;
                    position = Noun;
                    kinds = [];
                    feature = Number;
                },
                "kind set cannot be empty",
            ),
            (
                quote! {
                    closed = NounLexeme;
                    position = Noun;
                    kinds = [Type, Subtype, Type];
                    feature = Number;
                },
                "duplicate declaration_noun kind `Type`",
            ),
            (
                quote! {
                    closed = NounLexeme;
                    position = Noun;
                    kinds = [Type, KeywordAction];
                    feature = Number;
                },
                "kinds must be `Type` or `Subtype`",
            ),
            (
                quote! {
                    closed = NounLexeme;
                    position = Noun;
                    kinds = [Type];
                    feature = Number;
                },
                "requires `Subtype` kind",
            ),
            (
                quote! {
                    closed = NounLexeme;
                    position = Noun;
                    kinds = [Type, Subtype];
                    feature = Agreement;
                },
                "feature must be `Number`",
            ),
        ] {
            let message = declaration_noun_error(&body);
            assert!(
                message.contains(expected),
                "expected {expected:?} in {message}"
            );
        }
    }

    #[test]
    fn context_identity_recipe_validation_is_closed_and_structural() {
        validate(quote! {
            identity SelfReferenceSpelling {
                generate context {
                    Full => card_name,
                    Abbreviated => abbreviated_card_name,
                    canonical_on_collision = Full;
                }
            }
            construction only: Cat {
                element Only { spelling: identity SelfReferenceSpelling, }
                form only = identity(spelling);
            }
            root Cat { punctuation = "."; eoi = true; standalone_render = true; }
        })
        .expect("the exact context identity recipe validates");

        for (body, expected) in [
            (
                quote! { canonical_on_collision = Full; },
                "context identity requires exactly two arms",
            ),
            (
                quote! {
                    Full => card_name,
                    Full => abbreviated_card_name,
                    canonical_on_collision = Full;
                },
                "duplicate context identity arm `Full`",
            ),
            (
                quote! {
                    Full => card_name,
                    Abbreviated => card_name,
                    canonical_on_collision = Full;
                },
                "duplicate context identity accessor `card_name`",
            ),
            (
                quote! {
                    Full => card_name,
                    Abbreviated => display_name,
                    canonical_on_collision = Full;
                },
                "unknown ParseContext accessor `display_name`",
            ),
            (
                quote! {
                    Full => card_name,
                    Abbreviated => abbreviated_card_name,
                },
                "context identity requires `canonical_on_collision`",
            ),
            (
                quote! {
                    Full => card_name,
                    Abbreviated => abbreviated_card_name,
                    canonical_on_collision = Short;
                },
                "unknown canonical context identity arm `Short`",
            ),
            (
                quote! {
                    Full => card_name,
                    canonical_on_collision = Full;
                },
                "context identity requires exactly two arms",
            ),
        ] {
            let message = context_identity_error(&body);
            assert!(message.contains(expected), "{expected}: {message}");
        }
    }

    #[test]
    fn signed_decimal_recipe_validation_is_closed_and_structural() {
        let valid = validate(quote! {
            codec SignedNumber {
                generate signed_decimal {
                    magnitude = u32;
                    sign_type = Sign {
                        Positive = none,
                        Negative = "-",
                    };
                }
            }
            construction only: Cat { element Only {} form only = "only"; }
            root Cat { punctuation = "."; eoi = true; standalone_render = true; }
        });
        valid.expect("the exact signed-decimal recipe validates");

        let unsupported = signed_decimal_error(&quote! {
            magnitude = u64;
            sign_type = Sign { Positive = none, Negative = "-", };
        });
        assert!(
            unsupported.contains("signed_decimal magnitude must be `u32`"),
            "{unsupported}"
        );

        let missing = signed_decimal_error(&quote! {
            magnitude = u32;
            sign_type = Sign { Negative = "-", };
        });
        assert!(
            missing.contains("signed_decimal sign_type requires `Positive` and `Negative` roles"),
            "{missing}"
        );

        let duplicate = signed_decimal_error(&quote! {
            magnitude = u32;
            sign_type = Sign { Positive = none, Positive = none, Negative = "-", };
        });
        assert!(
            duplicate.contains("duplicate signed_decimal sign role `Positive`"),
            "{duplicate}"
        );

        let positive = signed_decimal_error(&quote! {
            magnitude = u32;
            sign_type = Sign { Positive = "+", Negative = "-", };
        });
        assert!(
            positive.contains("signed_decimal positive sign must be `none`"),
            "{positive}"
        );

        for negative in [quote! { "" }, quote! { "--" }, quote! { "−" }] {
            let message = signed_decimal_error(&quote! {
                magnitude = u32;
                sign_type = Sign { Positive = none, Negative = #negative, };
            });
            assert!(
                message.contains("signed_decimal negative sign must be one ASCII byte"),
                "{message}"
            );
        }
    }

    #[test]
    fn signed_decimal_rejects_unknown_recipes_and_mixed_binding_metadata() {
        let unknown = error(quote! {
            codec SignedNumber { generate hexadecimal {} }
            construction only: Cat { element Only {} form only = "only"; }
            root Cat { punctuation = "."; eoi = true; standalone_render = true; }
        });
        assert!(
            unknown.contains("unknown generated codec recipe `hexadecimal`"),
            "{unknown}"
        );

        let mixed = error(quote! {
            codec SignedNumber {
                value_type = SignedNumber;
                generate signed_decimal {
                    magnitude = u32;
                    sign_type = Sign { Positive = none, Negative = "-", };
                }
            }
            construction only: Cat { element Only {} form only = "only"; }
            root Cat { punctuation = "."; eoi = true; standalone_render = true; }
        });
        assert!(
            mixed.contains("generated codec cannot mix `generate` with legacy binding fields"),
            "{mixed}"
        );
    }

    #[test]
    fn open_verb_validation_errors_point_at_the_authored_kind_and_name() {
        let unsupported_source: proc_macro2::TokenStream = r#"
            construction unsupported: VerbPhrase {
                element Unsupported {}
                form unsupported = open_verb(Subtype, "Elf");
            }
            root VerbPhrase { punctuation = "."; eoi = true; standalone_render = true; }
        "#
        .parse()
        .expect("unsupported-kind fixture tokenizes");
        let unsupported = crate::parse_declarations(unsupported_source)
            .expect("the open_verb syntax parses before kind validation");
        let Declaration::Construction(construction) = &unsupported.declarations[0] else {
            panic!("first declaration is a construction")
        };
        let FormAtom::OpenVerb(open) = &construction.form.atoms[0] else {
            panic!("form contains an open declaration atom")
        };
        let expected_kind_span = open.kind.span();
        let error = super::validate_declarations(unsupported)
            .expect_err("subtype requires an authored supertype and is not a simple kind");
        assert_eq!(
            error.to_string(),
            "unsupported open declaration kind `Subtype`"
        );
        assert_same_span(error.span(), expected_kind_span);

        let empty_name_source: proc_macro2::TokenStream = r#"
            construction unnamed: VerbPhrase {
                element Unnamed {}
                form unnamed = open_verb(KeywordAction, "");
            }
            root VerbPhrase { punctuation = "."; eoi = true; standalone_render = true; }
        "#
        .parse()
        .expect("empty-name fixture tokenizes");
        let unnamed = crate::parse_declarations(empty_name_source)
            .expect("the open_verb syntax parses before name validation");
        let Declaration::Construction(construction) = &unnamed.declarations[0] else {
            panic!("first declaration is a construction")
        };
        let FormAtom::OpenVerb(open) = &construction.form.atoms[0] else {
            panic!("form contains an open declaration atom")
        };
        let expected_name_span = open.name.span();
        let error = super::validate_declarations(unnamed)
            .expect_err("an open declaration name cannot be empty");
        assert_eq!(error.to_string(), "open declaration name is empty");
        assert_same_span(error.span(), expected_name_span);
    }

    #[test]
    fn rejects_namespace_role_and_stored_field_errors() {
        let message = error(quote! {
            vocab Word { One = "same", One = "same", Two = "same", }
            vocab Word { Three = "three", }
            construction same_name: Cat {
                element SameName { child: Missing, child: lex Word, }
                form same_name = lex(child) lex(child);
            }
            construction same__name: Cat { element SameNameTwo {} form same_name = missing; }
            root Cat { punctuation = "."; eoi = true; standalone_render = true; }
        });
        assert!(
            message.contains("duplicate declaration `Word`"),
            "{message}"
        );
        assert!(message.contains("duplicate variant `One`"), "{message}");
        assert!(message.contains("duplicate word `same`"), "{message}");
        assert!(message.contains("duplicate field `child`"), "{message}");
        assert!(
            message.contains("semantic identity `Cat::SameName`")
                && message.contains("generated category variants"),
            "{message}"
        );

        let snake_collision = error(quote! {
            vocab FOOBar { One = "one", }
            vocab Foo_Bar { Two = "two", }
            construction only: Cat { element Only {} form only = "only"; }
            root Cat { punctuation = "."; eoi = true; standalone_render = true; }
        });
        assert!(
            snake_collision.contains("render_foo_bar"),
            "{snake_collision}"
        );

        let unknown = error(quote! {
            construction only: Cat { element Only { child: Missing, } form only = child; }
            root Cat { punctuation = "."; eoi = true; standalone_render = true; }
        });
        assert!(unknown.contains("unknown category `Missing`"), "{unknown}");

        let unknown_terminal = error(quote! {
            construction only: Cat { element Only { child: lex Missing, } form only = lex(child); }
            root Cat { punctuation = "."; eoi = true; standalone_render = true; }
        });
        assert!(
            unknown_terminal.contains("unknown terminal type `Missing`"),
            "{unknown_terminal}"
        );

        let coverage = error(quote! {
            vocab Word { One = "one", }
            construction only: Cat {
                element Only { first: lex Word, second: lex Word, }
                form only = lex(first) lex(first);
            }
            root Cat { punctuation = "."; eoi = true; standalone_render = true; }
        });
        assert!(coverage.contains("used 2 times"), "{coverage}");
        assert!(
            coverage.contains("field `second` is not present"),
            "{coverage}"
        );
    }

    #[test]
    fn rejects_derived_emitted_names_that_are_not_rust_identifiers() {
        let keyword = error(quote! {
            construction self: Cat { element Only {} form self = "only"; }
            root Cat { punctuation = "."; eoi = true; standalone_render = true; }
        });
        assert!(
            keyword.contains("generated Rust identifier `Self`")
                && keyword.contains("category variant"),
            "{keyword}"
        );

        let form_keyword = error(quote! {
            construction valid: Cat { element Valid {} form self = "only"; }
            root Cat { punctuation = "."; eoi = true; standalone_render = true; }
        });
        assert!(
            form_keyword.contains("generated Rust identifier `Self`")
                && form_keyword.contains("form/rule fragment"),
            "{form_keyword}"
        );

        let raw_element = error(quote! {
            construction valid: Cat { element r#type {} form valid = "only"; }
            root Cat { punctuation = "."; eoi = true; standalone_render = true; }
        });
        assert!(
            raw_element.contains("raw keyword `r#type`")
                && raw_element.contains("semantic identity `type`")
                && raw_element.contains("generated element type"),
            "{raw_element}"
        );

        validate(quote! {
            construction where: Cat { element WhereNode {} form where = "where"; }
            root Cat { punctuation = "."; eoi = true; standalone_render = true; }
        })
        .expect("a DSL keyword is valid when every derived Rust identifier is valid");
    }

    fn assert_raw_keyword_declaration_rejected(
        tokens: proc_macro2::TokenStream,
        generated_role: &str,
    ) {
        let message = crate::generate(tokens)
            .expect_err("a raw-keyword declaration name must fail before backend emission")
            .to_string();
        assert!(
            message.contains("raw keyword `r#type`")
                && message.contains("semantic identity `type`")
                && message.contains(generated_role),
            "{message}"
        );
        assert!(!message.contains("internal"), "{message}");
    }

    #[test]
    fn rejects_raw_keyword_category_names_at_the_authored_declaration() {
        assert_raw_keyword_declaration_rejected(
            quote! {
                construction only: r#type { element Only {} form only = "only"; }
                root r#type { punctuation = "."; eoi = true; standalone_render = true; }
            },
            "generated category type",
        );
    }

    #[test]
    fn rejects_raw_keyword_construction_names_at_the_authored_declaration() {
        assert_raw_keyword_declaration_rejected(
            quote! {
                construction r#type: Root { element Only {} form only = "only"; }
                root Root { punctuation = "."; eoi = true; standalone_render = true; }
            },
            "generated category variant",
        );
    }

    #[test]
    fn rejects_raw_keyword_form_and_terminal_variant_names_at_the_authored_declaration() {
        assert_raw_keyword_declaration_rejected(
            quote! {
                construction only: Root { element Only {} form r#type = "only"; }
                root Root { punctuation = "."; eoi = true; standalone_render = true; }
            },
            "generated form/rule fragment",
        );
        assert_raw_keyword_declaration_rejected(
            quote! {
                vocab Word { r#type = "type", }
                construction only: Root { element Only {} form only = "only"; }
                root Root { punctuation = "."; eoi = true; standalone_render = true; }
            },
            "generated vocab variant",
        );
        assert_raw_keyword_declaration_rejected(
            quote! {
                morphology EnglishNoun { feature = Number; recipe = english_noun; }
                lexeme Word using EnglishNoun { r#type = "type", }
                construction only: Root { element Only {} form only = "only"; }
                root Root { punctuation = "."; eoi = true; standalone_render = true; }
            },
            "generated lexeme variant",
        );
    }

    #[test]
    fn rejects_raw_keyword_element_names_at_the_authored_declaration() {
        assert_raw_keyword_declaration_rejected(
            quote! {
                construction only: Root { element r#type {} form only = "only"; }
                root Root { punctuation = "."; eoi = true; standalone_render = true; }
            },
            "generated element type",
        );
    }

    #[test]
    fn rejects_raw_keyword_vocab_names_at_the_authored_declaration() {
        assert_raw_keyword_declaration_rejected(
            quote! {
                vocab r#type { One = "one", }
                construction only: Root {
                    element Only { word: lex r#type, }
                    form only = lex(word);
                }
                root Root { punctuation = "."; eoi = true; standalone_render = true; }
            },
            "generated vocab type",
        );
    }

    #[test]
    fn rejects_raw_keyword_lexeme_names_at_the_authored_declaration() {
        assert_raw_keyword_declaration_rejected(
            quote! {
                morphology EnglishNoun { feature = Number; recipe = english_noun; }
                lexeme r#type using EnglishNoun { One = "one", }
                construction only: Root { element Only {} form only = "only"; }
                root Root { punctuation = "."; eoi = true; standalone_render = true; }
            },
            "generated lexeme type",
        );
    }

    #[test]
    fn rejects_raw_keyword_codec_names_at_the_authored_declaration() {
        assert_raw_keyword_declaration_rejected(
            quote! {
                codec r#type {
                    value_type = r#type;
                    traversal { callback = copy; argument = value; variant One; }
                }
                construction only: Root { element Only {} form only = "only"; }
                root Root { punctuation = "."; eoi = true; standalone_render = true; }
            },
            "generated codec binding type",
        );
    }

    #[test]
    fn rejects_raw_keyword_identity_names_at_the_authored_declaration() {
        assert_raw_keyword_declaration_rejected(
            quote! {
                identity r#type {
                    value_type = r#type;
                    lexical = Lexical::r#type;
                    render context_identity { One => card_name, }
                    build { pattern = BuildValue::r#type(value); construct = value; }
                    traversal { callback = copy; argument = value; variant One; }
                }
                construction only: Root { element Only {} form only = "only"; }
                root Root { punctuation = "."; eoi = true; standalone_render = true; }
            },
            "generated identity binding type",
        );
    }

    #[test]
    fn canonicalizes_raw_nonkeyword_element_names_across_public_items_and_helpers() {
        let expansion = crate::generate(quote! {
            construction r#raw_node: r#RawCategory {
                element r#RawNode {}
                form raw_node = "raw";
            }
            root r#RawCategory { punctuation = "."; eoi = true; standalone_render = true; }
        })
        .expect("a raw nonkeyword element has one legal generated identity");
        let formatted = crate::format_expansion(&expansion).expect("generated items format");
        assert!(formatted.contains("pub struct RawNode;"), "{formatted}");
        assert!(
            formatted.contains("fn visit_raw_node") && formatted.contains("pub fn walk_raw_node"),
            "{formatted}"
        );
        assert!(!formatted.contains("r#"), "{formatted}");
    }

    #[test]
    fn rejects_case_folded_walker_and_callback_collisions_before_emission() {
        let case_folded = crate::generate(quote! {
            morphology EnglishNoun { feature = Number; recipe = english_noun; }
            lexeme HttpServer using EnglishNoun { One = "one", }
            construction only: HTTPServer { element Only {} form only = "only"; }
            root HTTPServer { punctuation = "."; eoi = true; standalone_render = true; }
        })
        .expect_err("case-folded helper identities must be unique")
        .to_string();
        assert!(
            case_folded.contains("semantic identity `walk_http_server`")
                && case_folded.contains("generated category walker")
                && case_folded.contains("generated lexeme walker"),
            "{case_folded}"
        );

        let prefixed = crate::generate(quote! {
            codec Runtime {
                value_type = Runtime;
                traversal {
                    callback = borrowed;
                    argument = runtime;
                    leaf visit_http_server: i32 = copy;
                    field value: i32;
                    call visitor::visit_http_server(copy(value));
                }
            }
            construction only: HTTPServer { element Only {} form only = "only"; }
            root HTTPServer { punctuation = "."; eoi = true; standalone_render = true; }
        })
        .expect_err("authored callback names must not collide with generated callbacks")
        .to_string();
        assert!(
            prefixed.contains("semantic identity `visit_http_server`")
                && prefixed.contains("generated category callback")
                && prefixed.contains("declared traversal callback"),
            "{prefixed}"
        );
        assert!(!prefixed.contains("internal"), "{prefixed}");
    }

    #[test]
    fn rejects_case_converted_category_variant_and_rule_identity_collisions() {
        let category_variant = error(quote! {
            construction foo_bar: Root { element First {} form first = "first"; }
            construction FooBar: Root { element Second {} form second = "second"; }
            root Root { punctuation = "."; eoi = true; standalone_render = true; }
        });
        assert!(
            category_variant.contains("semantic identity `Root::FooBar`")
                && category_variant.contains("generated category variants"),
            "{category_variant}"
        );

        let rule = error(quote! {
            construction bar_baz: Foo { element First {} form first = "first"; }
            construction baz: FooBar { element Second {} form second = "second"; }
            root Foo { punctuation = "."; eoi = true; standalone_render = true; }
            root FooBar { punctuation = "."; eoi = true; standalone_render = true; }
        });
        assert!(
            rule.contains("semantic identity `FooBarBaz`")
                && rule.contains("RuleId for construction bar_baz")
                && rule.contains("RuleId for construction baz"),
            "{rule}"
        );
    }

    #[test]
    fn rejects_rule_id_variants_colliding_with_fixed_associated_items() {
        let source: proc_macro2::TokenStream = r#"
            construction o_u_n_t: C {
                element CountNode {}
                form count = "count";
            }
            root C { punctuation = "."; eoi = true; standalone_render = true; }
        "#
        .parse()
        .expect("RuleId associated-item collision declaration syntax");
        let parsed = crate::parse_declarations(source.clone()).expect("collision syntax parses");
        let Declaration::Construction(construction) = &parsed.declarations[0] else {
            panic!("first declaration is the colliding construction")
        };
        let expected_span = construction.name.span();
        let error = crate::generate(source)
            .expect_err("a RuleId variant cannot shadow a fixed associated item");
        assert_same_span(error.span(), expected_span);
        let message = error.to_string();
        assert!(
            message.contains("semantic identity `COUNT`")
                && message.contains("fixed generated RuleId test count")
                && message.contains("RuleId for construction o_u_n_t"),
            "{message}"
        );
        assert!(!message.contains("internal"), "{message}");
    }

    #[test]
    fn rejects_authored_rule_ids_colliding_with_every_structural_helper_family() {
        let cases = [
            quote! {
                construction item: Item { element ItemValue {} form item = "item"; }
                abstract product Holder { maybe: opt Item, }
                construction optional_absent: HolderMaybe {
                    element OptionalCollision {}
                    form absent = "absent";
                }
                root HolderMaybe { punctuation = "."; eoi = true; standalone_render = true; }
            },
            quote! {
                construction item: Item { element ItemValue {} form item = "item"; }
                abstract product Holder { items: seq Item separated by ", ", }
                require len(Holder.items) = 2;
                construction sequence_count2_final: HolderItems {
                    element SequenceCollision {}
                    form count2 = "count2";
                }
                root HolderItems { punctuation = "."; eoi = true; standalone_render = true; }
            },
            quote! {
                construction item: Item { element ItemValue {} form item = "item"; }
                abstract product Holder { items: seq Item separated by ", ", }
                require len(Holder.items) >= 2;
                require len(Holder.items) <= 4;
                construction sequence_count1_continue: HolderItems {
                    element SequenceContinueCollision {}
                    form count1 = "count1";
                }
                root HolderItems { punctuation = "."; eoi = true; standalone_render = true; }
            },
            quote! {
                construction item: Item { element ItemValue {} form item = "item"; }
                abstract product Holder {
                    items: seq Item separated by position {
                        pair = "<P>";
                        first = "<F>";
                        middle = "<M>";
                        last = "<L>";
                    },
                }
                require len(Holder.items) >= 2;
                require len(Holder.items) <= 4;
                construction sequence_count2_last: HolderItems {
                    element PositionalLastCollision {}
                    form last = "last";
                }
                root HolderItems { punctuation = "."; eoi = true; standalone_render = true; }
            },
            quote! {
                construction item: Item { element ItemValue {} form item = "item"; }
                abstract product Holder {
                    items: seq Item separated by position {
                        pair = "<P>";
                        first = "<F>";
                        middle = "<M>";
                        last = "<L>";
                    },
                }
                require len(Holder.items) >= 2;
                require len(Holder.items) <= 4;
                construction sequence_count2_middle: HolderItems {
                    element PositionalMiddleCollision {}
                    form middle = "middle";
                }
                root HolderItems { punctuation = "."; eoi = true; standalone_render = true; }
            },
            quote! {
                construction item: Item { element ItemValue {} form item = "item"; }
                abstract sum FooBar { baz: Item, }
                construction bar_baz: Foo {
                    element SumCollision {}
                    form alpha = "alpha";
                }
                root Foo { punctuation = "."; eoi = true; standalone_render = true; }
            },
            quote! {
                construction item: Item { element ItemValue {} form item = "item"; }
                construction exact_pair: ExactPair {
                    element ExactPairValue {
                        items: seq Item separated by position { pair = " and "; },
                    }
                    require len(items) = 2;
                    form exact_pair = items;
                }
                construction pair_value_items_sequence_pair: Exact {
                    element OwnerStateCollision {}
                    form pair = "pair";
                }
                root Exact {
                    punctuation = "."; eoi = true; standalone_render = true;
                }
            },
        ];

        for source in cases {
            let parsed = crate::parse_declarations(source.clone()).expect("collision case parses");
            let expected_span = parsed
                .declarations
                .iter()
                .rev()
                .find_map(|declaration| match declaration {
                    Declaration::Construction(construction) => Some(construction.name.span()),
                    _ => None,
                })
                .expect("colliding authored construction");
            let error = crate::validate_declarations(parsed)
                .expect_err("a structural RuleId helper collision must fail validation");
            assert_same_span(error.span(), expected_span);
            let message = error.to_string();
            assert!(
                message.contains("Construction/RuleId variant") && message.contains("structural"),
                "the collision must identify the structural RuleId family: {message}",
            );
            assert!(!message.contains("internal"), "{message}");
        }
    }

    #[test]
    fn rejects_authored_type_colliding_with_a_counted_sequence_category() {
        let parsed = crate::parse_declarations(quote! {
            construction item: Item { element ItemValue {} form item = "item"; }
            abstract product Holder { items: seq Item separated by ", ", }
            require len(Holder.items) >= 2;
            require len(Holder.items) <= 4;
            abstract product HolderItemsSequenceCount3Category {}
            root Holder { punctuation = "."; eoi = true; standalone_render = true; }
        })
        .expect("counted-category collision fixture parses");
        let expected_span = parsed
            .declarations
            .iter()
            .find_map(|declaration| match declaration {
                Declaration::AbstractProduct(product)
                    if product.name == "HolderItemsSequenceCount3Category" =>
                {
                    Some(product.name.span())
                }
                _ => None,
            })
            .expect("colliding authored product span");

        let error = crate::validate_declarations(parsed)
            .expect_err("an authored type cannot collide with a counted helper category");
        assert_same_span(error.span(), expected_span);
        let message = error.to_string();
        assert!(
            message.contains("HolderItemsSequenceCount3Category")
                && message.contains("generated helper name collision"),
            "the collision must identify the exact counted category family: {message}",
        );
    }

    #[test]
    fn accepts_authored_base_category_when_a_sequence_has_no_reachable_helper_state() {
        for source in [
            quote! {
                construction item: Item { element ItemValue {} form item = "item"; }
                abstract product Holder { items: seq Item separated by ", ", }
                require len(Holder.items) = 0;
                abstract product HolderItemsSequenceCategory {}
                root Item { punctuation = "."; eoi = true; standalone_render = true; }
            },
            quote! {
                construction item: Item { element ItemValue {} form item = "item"; }
                abstract product Holder {
                    items: seq Item separated by position { pair = " and "; },
                }
                require len(Holder.items) = 2;
                abstract product HolderItemsSequenceCategory {}
                root Item { punctuation = "."; eoi = true; standalone_render = true; }
            },
        ] {
            validate(source)
                .expect("an authored type may use a sequence category name that is not emitted");
        }
    }

    #[test]
    fn raw_validation_and_sealed_emission_have_exact_helper_category_set_parity() {
        let source = quote! {
            construction item: Item { element ItemValue {} form item = "item"; }
            abstract product UniformZero { items: seq Item separated by ", ", }
            require len(UniformZero.items) = 0;
            abstract product PositionalTwo {
                items: seq Item separated by position { pair = " and "; },
            }
            require len(PositionalTwo.items) = 2;
            abstract product UniformFinite { items: seq Item separated by ", ", }
            require len(UniformFinite.items) >= 2;
            require len(UniformFinite.items) <= 4;
            abstract product PositionalFinite {
                items: seq Item separated by position {
                    pair = " and ";
                    first = ", ";
                    middle = ", ";
                    last = ", and ";
                },
            }
            require len(PositionalFinite.items) >= 2;
            require len(PositionalFinite.items) <= 4;
            abstract product Unbounded { items: seq Item separated by ", ", }
            root Item { punctuation = "."; eoi = true; standalone_render = true; }
        };
        let raw = crate::parse_declarations(source.clone()).expect("parity fixture parses");
        let mut errors = None;
        let inventory = super::generated_name_inventory(&raw, &mut errors);
        assert!(errors.is_none(), "raw parity inventory is collision-free");
        let semantic = validate(source)
            .expect("parity fixture validates")
            .into_semantic();

        for (owner, expected) in [
            ("UniformZero", Vec::<&str>::new()),
            ("PositionalTwo", Vec::new()),
            (
                "UniformFinite",
                vec![
                    "UniformFiniteItemsSequenceCategory",
                    "UniformFiniteItemsSequenceCount2Category",
                    "UniformFiniteItemsSequenceCount3Category",
                    "UniformFiniteItemsSequenceCount4Category",
                ],
            ),
            (
                "PositionalFinite",
                vec![
                    "PositionalFiniteItemsSequenceCategory",
                    "PositionalFiniteItemsSequenceCount3Category",
                ],
            ),
            ("Unbounded", vec!["UnboundedItemsSequenceCategory"]),
        ] {
            let collision_role = format!("{owner}.items: generated helper name collision");
            let raw_names = inventory
                .type_names
                .iter()
                .filter(|&(name, role)| role == &collision_role && name.ends_with("Category"))
                .map(|(name, _)| name.clone())
                .collect::<BTreeSet<_>>();
            let emitted_names = crate::emit::structural_helper_categories(&semantic)
                .into_iter()
                .filter(|category| category.owner == owner && category.field.name() == "items")
                .map(|category| category.name)
                .collect::<BTreeSet<_>>();
            let expected = expected
                .into_iter()
                .map(str::to_owned)
                .collect::<BTreeSet<_>>();

            assert_eq!(raw_names, expected, "raw helper categories for {owner}");
            assert_eq!(
                emitted_names, expected,
                "sealed emitted helper categories for {owner}"
            );
        }
    }

    fn counted_inventory_size(maximum: usize) -> (usize, usize) {
        let maximum = syn::LitInt::new(&maximum.to_string(), proc_macro2::Span::call_site());
        let parsed = crate::parse_declarations(quote! {
            construction item: Item { element ItemValue {} form item = "item"; }
            abstract product FiniteUniformValue {
                items: seq Item separated by "<S>" terminated by "<T>",
            }
            require len(FiniteUniformValue.items) <= #maximum;
            abstract product FinitePositionalValue {
                items: seq Item separated by position {
                    pair = "<P>";
                    first = "<F>";
                    middle = "<M>";
                    last = "<L>";
                } terminated by "<T>",
            }
            require len(FinitePositionalValue.items) <= #maximum;
            root Item { punctuation = "."; eoi = true; standalone_render = true; }
        })
        .expect("counted inventory fixture parses");
        let mut errors = None;
        let inventory = super::generated_name_inventory(&parsed, &mut errors);
        assert!(
            errors.is_none(),
            "counted inventory fixture is collision-free"
        );
        let belongs_to_counted_family = |role: &&String| {
            role.contains("FiniteUniformValue") || role.contains("FinitePositionalValue")
        };
        (
            inventory
                .type_names
                .values()
                .filter(belongs_to_counted_family)
                .count(),
            inventory
                .rule_variants
                .values()
                .filter(belongs_to_counted_family)
                .count(),
        )
    }

    #[test]
    fn counted_sequence_collision_inventory_has_exact_linear_growth() {
        assert_eq!(counted_inventory_size(32), (68, 128));
        assert_eq!(counted_inventory_size(64), (132, 256));
    }

    #[test]
    fn rejects_authored_types_colliding_with_fixed_generated_aggregates() {
        for (fixed, fixed_role) in [
            ("Visitor", "fixed generated visitor trait"),
            ("Category", "fixed generated rules category type"),
            ("Construction", "fixed generated rules construction type"),
            ("RuleId", "fixed generated rules rule-id type"),
        ] {
            let source: proc_macro2::TokenStream = format!(
                r#"
                construction only: {fixed} {{
                    element FixedCollisionNode {{}}
                    form only = "only";
                }}
                root {fixed} {{ punctuation = "."; eoi = true; standalone_render = true; }}
                "#
            )
            .parse()
            .expect("fixed-collision declaration syntax");
            let parsed =
                crate::parse_declarations(source.clone()).expect("collision syntax parses");
            let Declaration::Construction(construction) = &parsed.declarations[0] else {
                panic!("first declaration is the colliding construction")
            };
            let expected_span = construction.category.span();
            let error = crate::generate(source)
                .expect_err("an authored type cannot escape into a fixed generated aggregate");
            assert_same_span(error.span(), expected_span);
            let message = error.to_string();
            assert!(
                message.contains(&format!("semantic identity `{fixed}`"))
                    && message.contains(fixed_role)
                    && message.contains("generated category type"),
                "{message}"
            );
            assert!(!message.contains("internal"), "{message}");
        }
    }

    #[test]
    fn rejects_authored_identities_colliding_with_every_fixed_runtime_type() {
        for fixed in [
            "Agreement",
            "Number",
            "FeatureConstraint",
            "CasePosition",
            "PrefixPosition",
            "StructuralTransition",
            "ScanPosition",
            "SequenceOwner",
            "FixedSurfaceAtom",
            "DeclarationClass",
            "DeclarationMatcher",
            "DeclarationLeaf",
            "Lexical",
            "Leaf",
            "TerminalClass",
            "LexicalTerminal",
            "LexicalProvenanceKind",
            "LexicalOwnerTemplate",
            "LexicalOwner",
            "BuildViolation",
            "BuildRejection",
        ] {
            let source: proc_macro2::TokenStream = format!(
                r#"
                identity {fixed} {{
                    generate context {{
                        Full => card_name,
                        Abbreviated => abbreviated_card_name,
                        canonical_on_collision = Full;
                    }}
                }}
                construction only: Cat {{
                    element FixedRuntimeCollisionNode {{ spelling: identity {fixed}, }}
                    form only = identity(spelling);
                }}
                root Cat {{ punctuation = "."; eoi = true; standalone_render = true; }}
                "#
            )
            .parse()
            .expect("fixed-runtime collision declaration syntax");
            let parsed =
                crate::parse_declarations(source.clone()).expect("collision syntax parses");
            let Declaration::Identity(identity) = &parsed.declarations[0] else {
                panic!("first declaration is the colliding identity")
            };
            let expected_span = identity.name.span();
            let error = crate::generate(source)
                .expect_err("an identity cannot escape into a fixed generated runtime type");
            assert_same_span(error.span(), expected_span);
            let message = error.to_string();
            assert!(
                message.contains(&format!("semantic identity `{fixed}`"))
                    && message.contains("fixed generated runtime type")
                    && message.contains("generated identity type"),
                "{fixed}: {message}"
            );
            assert!(!message.contains("internal"), "{message}");
        }
    }

    #[test]
    fn rejects_authored_unit_constructors_colliding_with_fixed_generated_values() {
        for (element, fixed_role) in [
            ("RULES", "fixed generated rules table constant"),
            ("build", "fixed generated build function"),
            ("build_checked", "fixed generated checked-build function"),
            (
                "sequence_separator",
                "fixed generated structural separator lookup",
            ),
            (
                "sequence_terminator",
                "fixed generated structural terminator lookup",
            ),
        ] {
            let source: proc_macro2::TokenStream = format!(
                r#"
                construction only: Root {{
                    element {element} {{}}
                    form only = "only";
                }}
                root Root {{ punctuation = "."; eoi = true; standalone_render = true; }}
                "#
            )
            .parse()
            .expect("fixed-value collision declaration syntax");
            let parsed =
                crate::parse_declarations(source.clone()).expect("collision syntax parses");
            let Declaration::Construction(construction) = &parsed.declarations[0] else {
                panic!("first declaration is the colliding construction")
            };
            let expected_span = construction.element.name.span();
            let error = crate::generate(source)
                .expect_err("a unit constructor cannot escape into a fixed generated value");
            assert_same_span(error.span(), expected_span);
            let message = error.to_string();
            assert!(
                message.contains(&format!("semantic identity `{element}`"))
                    && message.contains(fixed_role)
                    && message.contains("generated unit element constructor"),
                "{message}"
            );
            assert!(!message.contains("internal"), "{message}");
        }

        crate::generate(quote! {
            vocab RULES { One = "one", }
            construction only: Root {
                element build { word: lex RULES, }
                form only = lex(word);
            }
            root Root { punctuation = "."; eoi = true; standalone_render = true; }
        })
        .expect("types without value constructors may share fixed value spellings");
    }

    #[test]
    fn rejects_authored_values_colliding_with_structural_product_and_sum_render_walk_names() {
        for (element, abstract_declaration, generated) in [
            (
                "render_holder",
                "abstract product Holder {}",
                "generated abstract product renderer",
            ),
            (
                "walk_holder",
                "abstract product Holder {}",
                "generated abstract product walker",
            ),
            (
                "render_choice",
                "abstract sum Choice { Root, }",
                "generated abstract sum renderer",
            ),
            (
                "walk_choice",
                "abstract sum Choice { Root, }",
                "generated abstract sum walker",
            ),
        ] {
            let source: proc_macro2::TokenStream = format!(
                r#"
                construction only: Root {{
                    element {element} {{}}
                    form only = "only";
                }}
                {abstract_declaration}
                root Root {{ punctuation = "."; eoi = true; standalone_render = true; }}
                "#,
            )
            .parse()
            .expect("structural renderer/walker collision syntax");
            let error = crate::generate(source).expect_err("generated structural helper collides");
            let message = error.to_string();
            assert!(
                message.contains(element) && message.contains(generated),
                "{element}: {message}",
            );
        }

        let error = crate::generate(quote! {
            codec Runtime {
                value_type = Runtime;
                traversal {
                    callback = borrowed;
                    argument = runtime;
                    leaf visit_holder: Runtime = borrowed;
                    call visitor::visit_holder(borrowed(runtime));
                }
            }
            construction only: Root { element RootNode {} form only = "only"; }
            abstract product Holder {}
            root Root { punctuation = "."; eoi = true; standalone_render = true; }
        })
        .expect_err("the generated product visitor callback collides");
        let message = error.to_string();
        assert!(
            message.contains("visit_holder")
                && message.contains("generated abstract product visitor callback"),
            "{message}",
        );
    }

    #[test]
    fn rejects_conditional_render_names_in_the_complete_value_namespace() {
        let nested_source: proc_macro2::TokenStream = r#"
            construction foo: Foo { element FooNode {} form foo = "foo"; }
            construction wrapper: Wrapper {
                element WrapperNode { foo: Foo, }
                form wrapper = foo;
            }
            construction foo_body: FooBody {
                element FooBodyNode {}
                form foo_body = "body";
            }
            root Foo { punctuation = "."; eoi = true; standalone_render = true; }
        "#
        .parse()
        .expect("nested-root collision declaration syntax");
        let parsed =
            crate::parse_declarations(nested_source.clone()).expect("collision syntax parses");
        let Declaration::Construction(foo_body) = &parsed.declarations[2] else {
            panic!("third declaration is the colliding FooBody construction")
        };
        let expected_span = foo_body.category.span();
        let nested_error = crate::generate(nested_source)
            .expect_err("a nested standalone root reserves its exact body renderer");
        assert_same_span(nested_error.span(), expected_span);
        let nested_root = nested_error.to_string();
        assert!(
            nested_root.contains("semantic identity `render_foo_body`")
                && nested_root.contains("generated nested-root category renderer for `Foo`")
                && nested_root.contains("generated category renderer for `FooBody`"),
            "{nested_root}"
        );
        assert!(!nested_root.contains("internal"), "{nested_root}");

        let feature_helper = crate::generate(quote! {
            codec Head {
                atom = noun;
                value_type = Head;
                lexical = Lexical::Head;
                render = render_head;
                build { pattern = BuildValue::Head(head); construct = head; }
                traversal {
                    callback = borrowed;
                    argument = head;
                    call visitor::visit_head(borrowed(head));
                }
            }
            construction source: Source {
                element SourceNode {}
                derive number = Values::Singular;
                form source = "source";
            }
            construction phrase: Phrase {
                element PhraseNode { source: Source, head: lex Head, }
                derive number = source.number;
                form phrase = source noun(head);
            }
            construction collision: Collision {
                element number_for_source {}
                form collision = "collision";
            }
            root Phrase { punctuation = "."; eoi = true; standalone_render = true; }
        })
        .expect_err("a conditional feature helper participates in the value namespace")
        .to_string();
        assert!(
            feature_helper.contains("semantic identity `number_for_source`")
                && feature_helper.contains("generated Number helper for category `Source`")
                && feature_helper.contains("generated unit element constructor"),
            "{feature_helper}"
        );
        assert!(!feature_helper.contains("internal"), "{feature_helper}");
    }

    #[test]
    fn rejects_role_kind_unknown_variant_and_refinement_domain_errors() {
        let lexical_as_role = error(quote! {
            vocab Word { One = "one", }
            construction only: Cat { element Only { word: lex Word, } form only = word; }
            root Cat { punctuation = "."; eoi = true; standalone_render = true; }
        });
        assert!(
            lexical_as_role.contains("lexical field `word` used as a category role"),
            "{lexical_as_role}"
        );

        let category_as_lex = error(quote! {
            construction leaf_node: Branch { element LeafNode {} form leaf = "leaf"; }
            construction only: Cat { element Only { leaf: Branch, } form only = lex(leaf); }
            root Cat { punctuation = "."; eoi = true; standalone_render = true; }
        });
        assert!(
            category_as_lex.contains("category field `leaf` used as a lexical role"),
            "{category_as_lex}"
        );

        let lexeme_as_plain_lex = error(quote! {
            morphology EnglishVerb { feature = Agreement; recipe = english_verb; }
            lexeme Verbs using EnglishVerb { Be = "be", }
            construction only: Cat { element Only { word: lex Verbs, } form only = lex(word); }
            root Cat { punctuation = "."; eoi = true; standalone_render = true; }
        });
        assert!(
            lexeme_as_plain_lex.contains("lex atom")
                && lexeme_as_plain_lex.contains("name-only lexeme `Verbs`"),
            "{lexeme_as_plain_lex}"
        );

        let variants = error(quote! {
            vocab Word { One = "one", }
            morphology EnglishVerb { feature = Agreement; recipe = english_verb; }
            lexeme Verbs using EnglishVerb { Be = "be", }
            construction only: Cat {
                element Only { word: lex Word, }
                derive agreement = Anything::Bare;
                form only = lex(word) verb(Verbs::Missing);
            }
            root Cat { punctuation = "."; eoi = true; standalone_render = true; }
        });
        assert!(
            variants.contains("unknown variant `Missing` for lexeme `Verbs`"),
            "{variants}"
        );

        let vocab_variant = error(quote! {
            vocab Word { One = "one", }
            construction only: Cat {
                element Only { word: lex Word, }
                require word is Missing;
                form only = lex(word);
            }
            root Cat { punctuation = "."; eoi = true; standalone_render = true; }
        });
        assert!(
            vocab_variant.contains("unknown predicate member `Missing`"),
            "{vocab_variant}"
        );

        let wrong_domain = error(quote! {
            morphology EnglishVerb { feature = Agreement; recipe = english_verb; }
            lexeme Verbs using EnglishVerb { Be = "be", }
            construction leaf_node: Branch { element LeafNode {} derive agreement = Anything::Bare; form leaf = verb(Verbs::Be); }
            construction only: Cat { element Only { leaf: Branch, } require leaf is Be; form only = leaf; }
            root Cat { punctuation = "."; eoi = true; standalone_render = true; }
        });
        assert!(
            wrong_domain.contains("unknown predicate member `Be`"),
            "{wrong_domain}"
        );
    }

    #[test]
    fn require_rejects_unknown_subjects_kind_mismatches_and_unknown_members() {
        let unknown_subject = error(quote! {
            vocab Mode { One = "one", }
            construction only: Root {
                element Only { mode: lex Mode, }
                require missing is One;
                form only = lex(mode);
            }
            root Root { punctuation = "."; eoi = true; standalone_render = true; }
        });
        assert!(
            unknown_subject.contains("unknown predicate subject `missing`"),
            "{unknown_subject}"
        );

        let kind_mismatch = error(quote! {
            identity Handle {
                value_type = Handle;
                lexical = Lexical::Handle;
                render = render_handle;
                build { pattern = BuildValue::Handle(value); construct = value; }
                traversal { callback = borrowed; argument = value; call visitor::visit_handle(borrowed(value)); }
            }
            construction only: Root {
                element Only { handle: identity Handle, }
                require handle is One;
                form only = identity(handle);
            }
            root Root { punctuation = "."; eoi = true; standalone_render = true; }
        });
        assert!(
            kind_mismatch.contains("category or vocab predicate domain"),
            "{kind_mismatch}"
        );

        let unknown_member = error(quote! {
            vocab Mode { One = "one", }
            construction only: Root {
                element Only { mode: lex Mode, }
                require mode is Missing;
                form only = lex(mode);
            }
            root Root { punctuation = "."; eoi = true; standalone_render = true; }
        });
        assert!(
            unknown_member.contains("unknown predicate member `Missing`"),
            "{unknown_member}"
        );
    }

    #[test]
    fn require_rejects_semantic_duplicates_and_empty_intersections() {
        let duplicates = error(quote! {
            vocab Mode { One = "one", Two = "two", }
            construction only: Root {
                element Only { mode: lex Mode, }
                require mode in [One, r#One];
                form only = lex(mode);
            }
            root Root { punctuation = "."; eoi = true; standalone_render = true; }
        });
        assert!(
            duplicates.contains("duplicate predicate member `One`"),
            "{duplicates}"
        );

        let empty = error(quote! {
            vocab Mode { One = "one", Two = "two", }
            construction only: Root {
                element Only { mode: lex Mode, }
                require mode is One;
                require mode is Two;
                form only = lex(mode);
            }
            root Root { punctuation = "."; eoi = true; standalone_render = true; }
        });
        assert!(
            empty.contains("empty predicate intersection for `mode`"),
            "{empty}"
        );
    }

    #[test]
    fn require_rejects_unconstructible_and_parse_only_feature_state() {
        let unconstructible = error(quote! {
            construction only: Root {
                element Only {}
                require agreement is Bare;
                form only = "only";
            }
            root Root { punctuation = "."; eoi = true; standalone_render = true; }
        });
        assert!(
            unconstructible.contains(
                "construction predicate `agreement` has no constructible feature expression"
            ),
            "{unconstructible}"
        );

        let parse_only = error(quote! {
            morphology EnglishVerb { feature = Agreement; recipe = english_verb; }
            lexeme Verbs using EnglishVerb { Act = "act", }
            construction only: Root {
                element Only {}
                require verb.agreement is Bare;
                derive agreement = verb.agreement;
                form only = verb(Verbs::Act);
            }
            root Root { punctuation = "."; eoi = true; standalone_render = true; }
        });
        assert!(
            parse_only.contains("parse-only morphology feature state `verb.agreement`"),
            "{parse_only}"
        );
    }

    #[test]
    fn require_rejects_generated_new_name_collisions() {
        let transitive_new_collision = error(quote! {
            vocab Mode { One = "one", Two = "two", }
            construction only: Root {
                element Only { r#new: lex Mode, }
                require agreement is Bare;
                derive agreement = match r#new {
                    One => Values::Bare,
                    Two => Values::ThirdPersonSingular,
                };
                form only = lex(r#new);
            }
            root Root { punctuation = "."; eoi = true; standalone_render = true; }
        });
        assert!(
            transitive_new_collision.contains("generated construction associated item `new`"),
            "{transitive_new_collision}"
        );

        let new_collision = error(quote! {
            vocab Mode { One = "one", Two = "two", }
            construction only: Root {
                element Only { r#new: lex Mode, }
                require r#new is One;
                form only = lex(r#new);
            }
            root Root { punctuation = "."; eoi = true; standalone_render = true; }
        });
        assert!(
            new_collision.contains("generated construction associated item `new`"),
            "{new_collision}"
        );

        let same_item_collision = error(quote! {
            vocab Mode { One = "one", Two = "two", }
            construction only: Root {
                element Only { r#new: lex Mode, }
                require r#new is One;
                form only = lex(r#new);
            }
            root Root { punctuation = "."; eoi = true; standalone_render = true; }
        });
        assert_eq!(
            same_item_collision.matches("collides").count(),
            1,
            "the same generated accessor is not diagnosed twice: {same_item_collision}",
        );
    }

    #[test]
    fn constrained_structural_products_reserve_both_generated_constructor_names() {
        for (field, expected) in [
            (
                quote! { try_new },
                "generated structural associated item `try_new` for `Holder` collides with generated structural accessor `Holder.try_new`",
            ),
            (
                quote! { new },
                "generated structural associated item `new` for `Holder` collides with generated structural accessor `Holder.new`",
            ),
        ] {
            let actual = error(quote! {
                construction item: Item {
                    element ItemValue {}
                    form item = "item";
                }
                abstract product Holder { #field: seq Item, }
                require len(Holder.#field) >= 1;
                root Holder { eoi = true; standalone_render = true; }
            });
            assert!(actual.contains(expected), "{actual}");
        }
    }

    #[test]
    fn structural_only_construction_products_reserve_both_generated_constructor_names() {
        for (field, expected) in [
            (
                quote! { try_new },
                "generated construction associated item `try_new` for `ContainerValue` collides with generated construction accessor `ContainerValue.try_new`",
            ),
            (
                quote! { new },
                "generated construction associated item `new` for `ContainerValue` collides with generated construction accessor `ContainerValue.new`",
            ),
        ] {
            let actual = error(quote! {
                construction item: Item {
                    element ItemValue {}
                    form item = "item";
                }
                construction container: Container {
                    element ContainerValue { #field: seq Item, }
                    require len(ContainerValue.#field) >= 1;
                    form container = #field;
                }
                root Container { eoi = true; standalone_render = true; }
            });
            assert!(actual.contains(expected), "{actual}");
        }
    }

    #[test]
    fn unconstrained_structural_constructor_names_and_nonreserved_accessors_remain_legal() {
        validate(quote! {
            construction item: Item {
                element ItemValue {}
                form item = "item";
            }
            abstract product PublicNames { new: Item, try_new: seq Item, }
            abstract product ConstrainedNeighbor { items: seq Item, }
            require len(ConstrainedNeighbor.items) >= 1;
            root PublicNames { eoi = true; standalone_render = true; }
        })
        .expect("unconstrained reserved names and a nonreserved constrained accessor are legal");
    }

    #[test]
    fn rejects_identity_bindings_used_through_the_lex_atom() {
        let identity_as_lex = error(quote! {
            identity Existing {
                value_type = Existing;
                lexical = Lexical::Existing;
                render = render_existing;
                build { pattern = BuildValue::Existing(value); construct = value; }
                traversal { callback = borrowed; argument = value; call visitor::visit_existing(borrowed(value)); }
            }
            construction only: Cat {
                element Only { value: lex Existing, }
                form only = lex(value);
            }
            root Cat { punctuation = "."; eoi = true; standalone_render = true; }
        });
        assert!(
            identity_as_lex.contains("lex atom") && identity_as_lex.contains("identity binding"),
            "{identity_as_lex}"
        );

        let identity_as_noun = error(quote! {
            identity Existing {
                value_type = Existing;
                lexical = Lexical::Existing;
                render = render_existing;
                build { pattern = BuildValue::Existing(value); construct = value; }
                traversal { callback = borrowed; argument = value; call visitor::visit_existing(borrowed(value)); }
            }
            construction only: Cat {
                element Only { value: identity Existing, }
                derive number = Values::Singular;
                form only = noun(value);
            }
            root Cat { punctuation = "."; eoi = true; standalone_render = true; }
        });
        assert!(
            identity_as_noun.contains("noun atom") && identity_as_noun.contains("codec binding"),
            "{identity_as_noun}"
        );

        validate(quote! {
            identity Existing {
                value_type = Existing;
                lexical = Lexical::Existing;
                render = render_existing;
                build { pattern = BuildValue::Existing(value); construct = value; }
                traversal { callback = borrowed; argument = value; call visitor::visit_existing(borrowed(value)); }
            }
            construction only: Cat {
                element Only { value: identity Existing, }
                form only = identity(value);
            }
            root Cat { punctuation = "."; eoi = true; standalone_render = true; }
        })
        .expect("identity bindings are consumed only through identity fields and atoms");
    }

    #[test]
    fn resolves_recipe_selected_verb_lexeme_provider_without_a_name_switch() {
        let validated = validate(quote! {
            morphology EnglishVerb { feature = Agreement; recipe = english_verb; }
            lexeme Actions using EnglishVerb { Go = "go", }
            construction only: Cat {
                element Only {}
                derive agreement = verb.agreement;
                derive verb.agreement = Values::Bare;
                form only = verb(Actions::Go);
            }
            root Cat { punctuation = "."; eoi = true; standalone_render = true; }
        })
        .expect("the verb recipe selects capability regardless of the lexeme name");
        let actions = validated
            .semantic()
            .terminals()
            .iter()
            .find(|terminal| terminal.name() == "Actions")
            .expect("Actions terminal is planned");
        assert!(actions.supports_verb_atom());

        let inconsistent = error(quote! {
            morphology EnglishVerb { feature = Agreement; recipe = english_verb; }
            lexeme VerbLexeme using EnglishVerb { Destroy = "destroy", }
            lexeme NounLexeme using EnglishVerb { Player = "player", }
            construction destroy: Cat {
                element Destroy {}
                derive agreement = verb.agreement;
                derive verb.agreement = Values::Bare;
                form destroy = verb(VerbLexeme::Destroy);
            }
            construction player: Cat {
                element Player {}
                derive agreement = verb.agreement;
                derive verb.agreement = Values::Bare;
                form player = verb(NounLexeme::Player);
            }
            root Cat { punctuation = "."; eoi = true; standalone_render = true; }
        });
        assert!(
            inconsistent.contains("multiple verb lexeme providers")
                && inconsistent.contains("VerbLexeme")
                && inconsistent.contains("NounLexeme"),
            "{inconsistent}"
        );
    }

    #[test]
    fn unused_noun_lexemes_retain_traversal_without_direct_atom_capability() {
        let validated = validate(quote! {
            morphology EnglishNoun { feature = Number; recipe = english_noun; }
            lexeme Objects using EnglishNoun { Thing = "thing", }
            construction only: Cat { element Only {} form only = "only"; }
            root Cat { punctuation = "."; eoi = true; standalone_render = true; }
        })
        .expect("an unused noun lexeme still contributes its visitor traversal");
        let terminal = validated
            .semantic()
            .terminals()
            .first()
            .expect("lexeme terminal is planned");
        assert!(terminal.has_traversal());
        assert!(!terminal.has_direct_render());
        assert!(!terminal.has_direct_build());
        assert!(!terminal.supports_lex_atom());
        assert!(!terminal.supports_noun_atom());
        assert!(!terminal.supports_verb_atom());
    }

    #[test]
    fn rejects_standalone_codec_atom_class_mismatches() {
        let signed_number_as_noun = error(quote! {
            codec SignedNumber {
                atom = lex;
                value_type = SignedNumber;
                lexical = Lexical::SignedNumber;
                render = render_signed_number;
                build { pattern = BuildValue::SignedNumber(value); construct = value; }
                traversal { callback = borrowed; argument = value; call visitor::visit_signed_number(borrowed(value)); }
            }
            construction only: Cat {
                element Only { value: lex SignedNumber, }
                derive number = Values::Singular;
                form only = noun(value);
            }
            root Cat { punctuation = "."; eoi = true; standalone_render = true; }
        });
        assert!(
            signed_number_as_noun.contains("declares `lex`")
                && signed_number_as_noun.contains("noun atom"),
            "{signed_number_as_noun}"
        );

        let noun_as_lex = error(quote! {
            codec Noun {
                atom = noun;
                value_type = Noun;
                lexical = Lexical::Noun;
                render = render_noun;
                build { pattern = BuildValue::Noun(value); construct = value; }
                traversal { callback = borrowed; argument = value; call visitor::visit_noun(borrowed(value)); }
            }
            construction only: Cat {
                element Only { value: lex Noun, }
                form only = lex(value);
            }
            root Cat { punctuation = "."; eoi = true; standalone_render = true; }
        });
        assert!(
            noun_as_lex.contains("declares `noun`") && noun_as_lex.contains("lex atom"),
            "{noun_as_lex}"
        );
    }

    #[test]
    fn rejects_incomplete_or_open_terminal_binding_metadata() {
        let binding = error(quote! {
            codec ScalarNumber {
                atom = lex;
                value_type = ScalarNumber;
                lexical = Lexical::ScalarNumber;
                render = render_number;
                build {
                    pattern = BuildValue::ScalarNumber(value, _);
                    construct = arbitrary + rust;
                }
                traversal {
                    part whole = scalar(other.whole);
                    visit missing;
                }
            }
            construction only: Cat { element Only { number: lex ScalarNumber, } form only = lex(number); }
            root Cat { punctuation = "."; eoi = true; standalone_render = true; }
        });
        assert!(binding.contains("binding pattern"), "{binding}");
        assert!(
            binding.contains("binding construct expression"),
            "{binding}"
        );
        assert!(
            binding.contains("unknown traversal part `missing`"),
            "{binding}"
        );
    }

    #[test]
    fn rejects_feature_flow_errors_together_within_the_pass() {
        let message = error(quote! {
            vocab NumberWord { One = "one", Two = "two", }
            morphology EnglishVerb { feature = Agreement; recipe = english_verb; }
            lexeme Verbs using EnglishVerb { Be = "be", }
            codec Noun {
                atom = noun;
                value_type = Noun;
                lexical = Lexical::Noun;
                render = render_noun;
                build { pattern = BuildValue::Noun(value); construct = value; }
                traversal { callback = borrowed; argument = value; call visitor::visit_noun(borrowed(value)); }
            }
            construction broken: Cat {
                element Broken { word: lex NumberWord, noun: lex Noun, }
                derive agreement = Anything::Plural;
                derive agreement = Anything::Bare;
                derive number = match word {
                    One => Anything::Singular,
                    One => Anything::Plural,
                };
                form broken = lex(word) noun(noun) verb(Verbs::Be);
            }
            root Cat { punctuation = "."; eoi = true; standalone_render = true; }
        });
        assert!(message.contains("agreement value"), "{message}");
        assert!(message.contains("duplicate writer"), "{message}");
        assert!(message.contains("duplicate match arm `One`"), "{message}");
        assert!(message.contains("missing match arm `Two`"), "{message}");
    }

    #[test]
    fn lexical_feature_reads_require_an_exhaustive_same_construction_writer() {
        validate(quote! {
            vocab Person { One = "one", Many = "many", }
            construction valid: Root {
                element Valid { person: lex Person, }
                derive person.agreement = match person {
                    One => Values::Bare,
                    Many => Values::ThirdPersonSingular,
                };
                derive agreement = person.agreement;
                form valid = lex(person);
            }
            root Root { punctuation = "."; eoi = true; standalone_render = true; }
        })
        .expect("the explicit exhaustive role writer provides agreement");

        let missing = error(quote! {
            vocab Person { One = "one", Many = "many", }
            construction missing: Root {
                element Missing { person: lex Person, }
                derive agreement = person.agreement;
                form missing = lex(person);
            }
            root Root { punctuation = "."; eoi = true; standalone_render = true; }
        });
        assert!(
            missing.contains("does not have an exhaustive local agreement writer"),
            "{missing}"
        );

        let mismatched = error(quote! {
            vocab Person { One = "one", Many = "many", }
            construction mismatched: Root {
                element Mismatched { person: lex Person, other: lex Person, }
                derive person.agreement = match other {
                    One => Values::Bare,
                    Many => Values::ThirdPersonSingular,
                };
                derive agreement = person.agreement;
                form mismatched = lex(person) lex(other);
            }
            root Root { punctuation = "."; eoi = true; standalone_render = true; }
        });
        assert!(
            mismatched.contains("does not have an exhaustive local agreement writer"),
            "{mismatched}"
        );

        let mixed = error(quote! {
            vocab Person { One = "one", Many = "many", }
            construction mixed: Root {
                element Mixed { person: lex Person, }
                derive person.agreement = match person {
                    One => Values::Singular,
                    Many => Values::Plural,
                };
                derive agreement = person.agreement;
                form mixed = lex(person);
            }
            root Root { punctuation = "."; eoi = true; standalone_render = true; }
        });
        assert!(mixed.contains("is not an agreement value"), "{mixed}");
    }

    #[test]
    fn rejects_open_or_incompatible_closed_traversal_schemas() {
        let empty_context = error(quote! {
            identity Flag {
                value_type = Flag;
                lexical = Lexical::Flag;
                render context_identity {}
                build { pattern = BuildValue::Flag(flag); construct = flag; }
                traversal { callback = copy; argument = flag; variant Full; variant Short; }
            }
            construction only: Root { element Only { flag: identity Flag, } form only = identity(flag); }
            root Root { punctuation = "."; eoi = true; standalone_render = true; }
        });
        assert!(
            empty_context.contains("context identity render requires at least one arm"),
            "{empty_context}"
        );

        let duplicate_context = error(quote! {
            identity Flag {
                value_type = Flag;
                lexical = Lexical::Flag;
                render context_identity { Full => card_name, Full => abbreviated_card_name, }
                build { pattern = BuildValue::Flag(flag); construct = flag; }
                traversal { callback = copy; argument = flag; variant Full; variant Short; }
            }
            construction only: Root { element Only { flag: identity Flag, } form only = identity(flag); }
            root Root { punctuation = "."; eoi = true; standalone_render = true; }
        });
        assert!(
            duplicate_context.contains("duplicate context identity variant `Full`"),
            "{duplicate_context}"
        );
        assert!(
            duplicate_context.contains("missing context identity variant `Short`"),
            "{duplicate_context}"
        );

        let open_pattern = error(quote! {
            codec Runtime {
                value_type = Runtime;
                traversal {
                    callback = borrowed;
                    argument = runtime;
                    match runtime {
                        _ => visitor::visit_runtime(borrowed(runtime)),
                    }
                }
            }
            construction only: Root { element Only {} form only = "only"; }
            root Root { punctuation = "."; eoi = true; standalone_render = true; }
        });
        assert!(
            open_pattern.contains("closed variant-binding pattern"),
            "{open_pattern}"
        );

        let coverage = error(quote! {
            codec Runtime {
                value_type = Runtime;
                traversal {
                    callback = borrowed;
                    argument = runtime;
                    variant First;
                    variant Second;
                    match runtime {
                        Runtime::First(value: Runtime) => visitor::visit_runtime(borrowed(value)),
                        Runtime::First(other: Runtime) => visitor::visit_runtime(borrowed(other)),
                    }
                }
            }
            construction only: Root { element Only {} form only = "only"; }
            root Root { punctuation = "."; eoi = true; standalone_render = true; }
        });
        assert!(
            coverage.contains("duplicate traversal match variant `First`"),
            "{coverage}"
        );
        assert!(
            coverage.contains("missing traversal match variant `Second`"),
            "{coverage}"
        );

        let callback = error(quote! {
            codec Runtime {
                value_type = Runtime;
                traversal {
                    callback = borrowed;
                    argument = runtime;
                    call walk_missing(borrowed(runtime));
                }
            }
            construction only: Root { element Only {} form only = "only"; }
            root Root { punctuation = "."; eoi = true; standalone_render = true; }
        });
        assert!(
            callback.contains("unknown traversal callback `walk_missing`"),
            "{callback}"
        );

        let pass = error(quote! {
            codec Runtime {
                value_type = Runtime;
                traversal {
                    callback = borrowed;
                    argument = runtime;
                    call visitor::visit_runtime(copy(runtime));
                }
            }
            construction only: Root { element Only {} form only = "only"; }
            root Root { punctuation = "."; eoi = true; standalone_render = true; }
        });
        assert!(pass.contains("requires borrowed traversal mode"), "{pass}");

        let value_type = error(quote! {
            codec Sign {
                value_type = Sign;
                traversal { callback = copy; argument = sign; variant Positive; }
            }
            codec Runtime {
                value_type = Runtime;
                traversal {
                    callback = borrowed;
                    argument = runtime;
                    field sign: Runtime;
                    call walk_sign(copy(sign));
                }
            }
            construction only: Root { element Only {} form only = "only"; }
            root Root { punctuation = "."; eoi = true; standalone_render = true; }
        });
        assert!(
            value_type.contains("expects `Sign` but argument is `Runtime`"),
            "{value_type}"
        );

        let collision = error(quote! {
            codec First {
                value_type = First;
                traversal {
                    callback = borrowed;
                    argument = first;
                    leaf visit_shared: str = borrowed;
                    call visitor::visit_shared(borrowed(first));
                }
            }
            identity Second {
                value_type = Second;
                traversal {
                    callback = borrowed;
                    argument = second;
                    leaf visit_shared: str = borrowed;
                    call visitor::visit_shared(borrowed(second));
                }
            }
            construction only: Root { element Only {} form only = "only"; }
            root Root { punctuation = "."; eoi = true; standalone_render = true; }
        });
        assert!(
            collision.contains("traversal callback `visit_shared` collides"),
            "{collision}"
        );
    }

    #[test]
    fn rejects_traversal_body_modes_codegen_does_not_support() {
        let borrowed_variants = error(quote! {
            codec BorrowedEnum {
                value_type = BorrowedEnum;
                traversal {
                    callback = borrowed;
                    argument = borrowed_enum;
                    variant One;
                }
            }
            construction only: Root { element Only {} form only = "only"; }
            root Root { punctuation = "."; eoi = true; standalone_render = true; }
        });
        assert!(
            borrowed_variants.contains("enum traversal body requires copy callback mode"),
            "{borrowed_variants}"
        );

        let copy_calls = error(quote! {
            codec CopyCalls {
                value_type = CopyCalls;
                traversal {
                    callback = copy;
                    argument = copy_calls;
                    call visitor::visit_copy_calls(copy(copy_calls));
                }
            }
            construction only: Root { element Only {} form only = "only"; }
            root Root { punctuation = "."; eoi = true; standalone_render = true; }
        });
        assert!(
            copy_calls.contains("calls traversal body requires borrowed callback mode"),
            "{copy_calls}"
        );

        let copy_match = error(quote! {
            codec Part {
                value_type = Part;
                traversal { callback = copy; argument = part; variant Value; }
            }
            codec CopyMatch {
                value_type = CopyMatch;
                traversal {
                    callback = copy;
                    argument = copy_match;
                    variant Part;
                    match copy_match {
                        CopyMatch::Part(part: Part) => walk_part(copy(part)),
                    }
                }
            }
            construction only: Root { element Only {} form only = "only"; }
            root Root { punctuation = "."; eoi = true; standalone_render = true; }
        });
        assert!(
            copy_match.contains("match traversal body requires borrowed callback mode"),
            "{copy_match}"
        );
    }

    #[test]
    fn rejects_unbound_reads_cycles_and_missing_atom_features() {
        let unbound = error(quote! {
            construction leaf_node: Branch { element LeafNode {} form leaf = "leaf"; }
            construction parent_node: Parent {
                element ParentNode { leaf: Branch, }
                derive agreement = leaf.agreement;
                form parent = leaf;
            }
            root Parent { punctuation = "."; eoi = true; standalone_render = true; }
        });
        assert!(unbound.contains("does not provide agreement"), "{unbound}");

        let cycle = error(quote! {
            construction leaf_node: Branch { element LeafNode {} derive agreement = Anything::Bare; form leaf = "leaf"; }
            construction parent_node: Parent {
                element ParentNode { left: Branch, right: Branch, }
                derive left.agreement = right.agreement;
                derive right.agreement = left.agreement;
                form parent = left right;
            }
            root Parent { punctuation = "."; eoi = true; standalone_render = true; }
        });
        assert!(cycle.contains("feature equation cycle"), "{cycle}");

        let atoms = error(quote! {
            morphology EnglishVerb { feature = Agreement; recipe = english_verb; }
            lexeme Verbs using EnglishVerb { Be = "be", }
            codec Nouns {
                atom = noun;
                value_type = Nouns;
                lexical = Lexical::Nouns;
                render = render_nouns;
                build { pattern = BuildValue::Nouns(value); construct = value; }
                traversal { callback = borrowed; argument = value; call visitor::visit_nouns(borrowed(value)); }
            }
            construction broken: Cat {
                element Broken { noun: lex Nouns, }
                form broken = noun(noun) verb(Verbs::Be);
            }
            root Cat { punctuation = "."; eoi = true; standalone_render = true; }
        });
        assert!(atoms.contains("verb atom requires agreement"), "{atoms}");
        assert!(atoms.contains("noun atom requires number"), "{atoms}");
    }

    #[test]
    fn validates_metadata_without_type_name_switches_and_preserves_order() {
        let validated = validate(quote! {
            vocab Pointing { Near = "this", Far = "those", }
            morphology EnglishVerb { feature = Agreement; recipe = english_verb; }
            lexeme Actions using EnglishVerb { Exist = "exist", }
            codec ObjectWord {
                atom = noun;
                value_type = crate::runtime::ObjectWord;
                lexical = RuntimeLeaf::ObjectWord;
                render = crate::runtime::render_object;
                build { pattern = RuntimeValue::ObjectWord(value); construct = value; }
                traversal { callback = borrowed; argument = value; call visitor::visit_object_word(borrowed(value)); }
            }
            construction nested: Loop {
                element Nested { body: Loop, }
                form nested = body;
            }
            construction pointing_rule: Thing {
                element PointingRule { word: lex Pointing, head: lex ObjectWord, }
                derive agreement = Whatever::ThirdPersonSingular;
                derive verb.agreement = Whatever::ThirdPersonSingular;
                derive number = match word {
                    Near => Whatever::Singular,
                    Far => Whatever::Plural,
                };
                form pointing = lex(word) noun(head) verb(Actions::Exist);
            }
            root Loop { punctuation = "!"; eoi = true; standalone_render = false; }
            root Thing { punctuation = "."; eoi = false; standalone_render = true; }
        })
        .expect("renamed metadata drives validation");
        assert_eq!(validated.declaration_count(), 8);
        assert_eq!(
            validated.declaration_names(),
            [
                "Pointing",
                "EnglishVerb",
                "Actions",
                "ObjectWord",
                "nested",
                "pointing_rule",
                "Loop",
                "Thing"
            ]
        );
        assert!(
            validated
                .boxed_fields()
                .contains(&("nested".to_owned(), "body".to_owned()))
        );
        assert!(
            !validated
                .boxed_fields()
                .contains(&("pointing_rule".to_owned(), "head".to_owned()))
        );
        assert!(
            validated
                .dynamic_number_constructions()
                .contains("pointing_rule")
        );
    }

    #[test]
    fn validates_fixed_verb_slots_and_parent_agreement_flow() {
        validate(quote! {
            codec Heads {
                atom = noun;
                value_type = Heads;
                lexical = Lexical::Heads;
                render = render_heads;
                build { pattern = BuildValue::Heads(value); construct = value; }
                traversal { callback = borrowed; argument = value; call visitor::visit_heads(borrowed(value)); }
            }
            morphology EnglishVerb { feature = Agreement; recipe = english_verb; }
            lexeme Actions using EnglishVerb {
                Destroy = "destroy",
                Be = "be",
                Control = "control",
            }

            construction noun_node: NounPhrase {
                element NounNode { head: lex Heads, }
                derive agreement = Values::ThirdPersonSingular;
                derive number = Values::Singular;
                form noun_node = noun(head);
            }
            construction count_node: NounPhrase {
                element CountNode { head: lex Heads, }
                derive agreement = Values::Bare;
                derive number = Values::Plural;
                derive verb.agreement = Values::Bare;
                form count_node = noun(head) verb(Actions::Control);
            }
            construction destroy: VerbPhrase {
                element Destroy { object: NounPhrase, }
                derive agreement = verb.agreement;
                form destroy = verb(Actions::Destroy) object;
            }
            construction where_clause: Clause {
                element WhereClause { value: NounPhrase, }
                derive verb.agreement = Values::ThirdPersonSingular;
                form where_clause = "where" verb(Actions::Be) value;
            }
            construction imperative: Sentence {
                element Imperative { predicate: VerbPhrase, }
                derive predicate.agreement = Values::Bare;
                form imperative = predicate;
            }
            construction declarative: Sentence {
                element Declarative { subject: NounPhrase, predicate: VerbPhrase, }
                derive predicate.agreement = subject.agreement;
                form declarative = subject predicate;
            }
            root Sentence { punctuation = "."; eoi = true; standalone_render = true; }
        })
        .expect("fixed verb slots and role inheritance are fully bound");
    }

    #[test]
    fn demonstrative_lowers_as_one_rule_with_either_number_and_exhaustive_guard() {
        let validated = validate(quote! {
            vocab PointingWords { That = "that", Those = "those", }
            codec HeadWord {
                atom = noun;
                value_type = crate::runtime::HeadWord;
                lexical = RuntimeLeaf::HeadWord;
                render = crate::runtime::render_head;
                build { pattern = RuntimeValue::HeadWord(value); construct = value; }
                traversal { callback = borrowed; argument = value; call visitor::visit_head_word(borrowed(value)); }
            }
            construction demonstrative: NounPhrase {
                element DemonstrativeNp { word: lex PointingWords, head: lex HeadWord, }
                derive number = match word {
                    That => Values::Singular,
                    Those => Values::Plural,
                };
                form demonstrative = lex(word) noun(head);
            }
            root NounPhrase { punctuation = "."; eoi = true; standalone_render = true; }
        })
        .expect("exhaustive demonstrative word/number map validates");

        assert!(
            validated
                .dynamic_number_constructions()
                .contains("demonstrative")
        );
        let equations = validated.feature_equations("demonstrative");
        assert_eq!(
            equations
                .iter()
                .filter(|equation| matches!(
                    equation.value(),
                    crate::feature::FeatureExpr::MatchVocab { .. }
                ))
                .count(),
            1,
            "one dynamic number guard feeds the one declared form/rule"
        );
        let match_arms = equations
            .iter()
            .find_map(|equation| match equation.value() {
                crate::feature::FeatureExpr::MatchVocab { arms, .. } => Some(arms),
                _ => None,
            })
            .expect("dynamic number equation retains its match arms");
        assert_eq!(
            match_arms
                .iter()
                .map(|(variant, value)| (variant.value().to_string(), format!("{value:?}")))
                .collect::<Vec<_>>(),
            [
                ("That".to_owned(), "Singular".to_owned()),
                ("Those".to_owned(), "Plural".to_owned()),
            ]
        );
        let construction = validated
            .semantic()
            .constructions()
            .iter()
            .find(|construction| construction.construction_id() == "demonstrative")
            .expect("demonstrative construction remains in the sealed IR");
        assert_eq!(construction.form(), "demonstrative");
    }

    #[test]
    fn rejects_number_reads_from_fixed_and_projected_verb_slots() {
        let fixed = error(quote! {
            morphology EnglishVerb { feature = Agreement; recipe = english_verb; }
            lexeme Verbs using EnglishVerb { Be = "be", }
            construction broken: Cat {
                element Broken {}
                derive verb.agreement = Values::Bare;
                derive number = verb.number;
                form broken = verb(Verbs::Be);
            }
            root Cat { punctuation = "."; eoi = true; standalone_render = true; }
        });
        assert!(
            fixed.contains("verb slot `verb` does not provide number"),
            "{fixed}"
        );

        let projected = error(quote! {
            morphology EnglishVerb { feature = Agreement; recipe = english_verb; }
            lexeme Verbs using EnglishVerb { Be = "be", }
            construction broken: Cat {
                element Broken { word: lex Verbs, }
                derive word.agreement = Values::Bare;
                derive number = word.number;
                form broken = verb(word);
            }
            root Cat { punctuation = "."; eoi = true; standalone_render = true; }
        });
        assert!(
            projected.contains("verb slot `word` does not provide number"),
            "{projected}"
        );
    }

    #[test]
    fn boxes_only_recursive_category_edges() {
        let validated = validate(quote! {
            construction event: Clause { element EventClause {} form event = "event"; }
            construction sentence_leaf: Sentence { element SentenceLeaf {} form sentence_leaf = "sentence"; }
            construction with_where: Sentence {
                element WithWhere { body: Sentence, clause: Clause, }
                form with_where = body clause;
            }
            root Sentence { punctuation = "."; eoi = true; standalone_render = true; }
        })
        .expect("recursive category graph validates");
        assert!(
            validated
                .boxed_fields()
                .contains(&("with_where".to_owned(), "body".to_owned()))
        );
        assert!(
            !validated
                .boxed_fields()
                .contains(&("with_where".to_owned(), "clause".to_owned()))
        );
    }

    #[test]
    fn parser_fence_rejects_incomplete_bindings_and_deferred_structural_require() {
        let incomplete = error(quote! {
            codec Number {
                atom = lex;
                value_type = Number;
                lexical = Lexical::Number;
                render = render_number;
                build { pattern = BuildValue::Number(value); construct = value; }
            }
        });
        assert!(
            incomplete.contains("binding requires explicit traversal slot"),
            "{incomplete}"
        );

        let structural = error(quote! {
            construction only: Cat {
                element Only { value: Cat, }
                require value != nothing;
                form only = value;
            }
        });
        assert!(
            structural.contains("Plan 05 structural declarations"),
            "{structural}"
        );
        assert!(structural.contains("unimplemented in MVP"), "{structural}");
    }

    #[test]
    fn projected_verb_is_rejected_before_backend_planning() {
        let generated = crate::generate(quote! {
            morphology EnglishVerb { feature = Agreement; recipe = english_verb; }
            lexeme Verbs using EnglishVerb { Be = "be", }
            construction projected: Cat {
                element Projected { word: lex Verbs, }
                derive word.agreement = Values::Bare;
                form projected = verb(word);
            }
            root Cat { punctuation = "."; eoi = true; standalone_render = true; }
        })
        .expect_err("projected verb roles are outside the fixed-path MVP");
        let message = generated.to_string();
        assert!(
            message.contains("unimplemented in MVP: `projected verb role`"),
            "{message}"
        );
        assert!(
            !message.contains("internal"),
            "validation must own this diagnostic: {message}"
        );
    }

    #[test]
    fn structural_validation_reports_owner_and_role_for_every_failure_class() {
        let cases = [
            (
                quote! {
                    abstract product Holder {
                        items: seq Node separated by position {
                            first = ", "; middle = ", "; last = ", and ";
                        },
                    }
                    require len(Holder.items) = 2;
                },
                "Holder.items: missing pair",
            ),
            (
                quote! {
                    abstract product Holder {
                        items: seq Node separated by position {
                            pair = " and "; first = ", "; middle = ", ";
                            last = ", and "; last = " or ";
                        },
                    }
                    require len(Holder.items) >= 2;
                },
                "Holder.items: duplicate last",
            ),
            (
                quote! {
                    abstract product Holder {
                        items: seq Node separated by position {
                            pair = " and "; first = ", "; middle = ", "; last = ", and ";
                        },
                    }
                    require len(Holder.items) >= 2;
                    require len(Holder.items) <= 3;
                },
                "Holder.items: unreachable middle",
            ),
            (
                quote! { abstract product Holder { items: seq Node separated by "", } },
                "Holder.items: empty separator surface",
            ),
            (
                quote! { abstract product Holder { items: seq Node terminated by "", } },
                "Holder.items: empty terminator surface",
            ),
            (
                quote! {
                    abstract product Empty {}
                    abstract product Holder { items: seq Empty, }
                },
                "Holder.items: nullable repeated item",
            ),
            (
                quote! {
                    abstract product Holder { items: seq Node, }
                    require len(Holder.items) >= 3;
                    require len(Holder.items) <= 2;
                },
                "Holder.items: contradictory length requirements",
            ),
            (
                quote! { abstract sum Holder { items: Missing, } },
                "Holder.items: unresolved structural alternative Missing",
            ),
            (
                quote! {
                    abstract product Holder { items: seq Node, }
                    abstract product HolderItemsSequence {}
                },
                "Holder.items: generated helper name collision",
            ),
            (
                quote! {
                    abstract product Holder { items: seq Holder, }
                    require len(Holder.items) = 1;
                },
                "Holder.items: zero-width recursive cycle",
            ),
        ];

        for (source, expected) in cases {
            let actual = error(source);
            assert!(
                actual.contains(expected),
                "expected `{expected}` in {actual}"
            );
        }
    }

    #[test]
    fn structural_validation_accumulates_independent_declaration_errors() {
        let actual = error(quote! {
            abstract sum Choice { absent: Missing, }
            abstract product Holder { items: seq Node separated by "", }
        });

        assert!(
            actual.contains("Choice.absent: unresolved structural alternative Missing"),
            "{actual}"
        );
        assert!(
            actual.contains("Holder.items: empty separator surface"),
            "{actual}"
        );
    }

    #[test]
    fn structural_sum_only_cycle_is_rejected_with_owner_evidence() {
        let actual = error(quote! {
            abstract sum Left { right: Right, }
            abstract sum Right { left: Left, }
            construction entry: Root {
                element Entry { value: Left, }
                form entry = value;
            }
            root Root { punctuation = "."; eoi = true; standalone_render = true; }
        });

        assert!(
            actual.contains("Left.right: zero-width recursive cycle"),
            "{actual}"
        );
        assert!(
            actual.contains("Right.left: zero-width recursive cycle"),
            "{actual}"
        );
    }

    #[test]
    fn structural_grouped_length_requirements_are_rejected_with_owner_and_role() {
        let actual = error(quote! {
            abstract product AllHolder {
                items: seq AllHolder terminated by ".",
            }
            require all(len(AllHolder.items) >= 1, len(AllHolder.items) <= 2);
            abstract product AnyHolder {
                items: seq AnyHolder terminated by ".",
            }
            require any(len(AnyHolder.items) = 1, len(AnyHolder.items) = 2);
        });

        assert!(
            actual.contains("AllHolder.items: grouped length requirements are unsupported"),
            "{actual}"
        );
        assert!(
            actual.contains("AnyHolder.items: grouped length requirements are unsupported"),
            "{actual}"
        );
    }

    #[test]
    fn structural_length_normalization_rejects_underflow_and_overflow() {
        let underflow = error(quote! {
            abstract product Holder { items: seq Holder terminated by ".", }
            require len(Holder.items) < 0;
        });
        assert!(
            underflow.contains("Holder.items: len < 0 is impossible"),
            "{underflow}"
        );

        let overflow =
            syn::LitInt::new(&format!("{}0", usize::MAX), proc_macro2::Span::call_site());
        let overflow = error(quote! {
            abstract product Holder { items: seq Holder terminated by ".", }
            require len(Holder.items) > #overflow;
        });
        assert!(
            overflow.contains("Holder.items: length bound overflow"),
            "{overflow}"
        );
    }

    #[test]
    fn legacy_traversal_is_rejected_even_when_the_binding_is_unused() {
        for tokens in [
            quote! {
                codec Legacy {
                    atom = lex;
                    value_type = Legacy;
                    lexical = Lexical::Legacy;
                    render = render_legacy;
                    build { pattern = BuildValue::Legacy(value); construct = value; }
                    traversal { part value = scalar(value); visit value; }
                }
                construction used: Root { element Used { value: lex Legacy, } form used = lex(value); }
                root Root { punctuation = "."; eoi = true; standalone_render = true; }
            },
            quote! {
                codec Legacy {
                    atom = lex;
                    value_type = Legacy;
                    lexical = Lexical::Legacy;
                    render = render_legacy;
                    build { pattern = BuildValue::Legacy(value); construct = value; }
                    traversal { part value = scalar(value); visit value; }
                }
                construction unused: Root { element Unused {} form unused = "unused"; }
                root Root { punctuation = "."; eoi = true; standalone_render = true; }
            },
        ] {
            let failure = crate::generate(tokens).expect_err("legacy traversal is deferred");
            let message = failure.to_string();
            assert!(
                message.contains("unimplemented in MVP: `legacy traversal part/visit`"),
                "{message}"
            );
            assert!(
                !message.contains("internal"),
                "validation must reject unused legacy metadata too: {message}"
            );
        }
    }

    #[test]
    fn roots_require_exactly_one_unicode_scalar() {
        for punctuation in ["", "..", "e\u{301}"] {
            let punctuation = syn::LitStr::new(punctuation, proc_macro2::Span::call_site());
            let message = crate::generate(quote! {
                construction only: Root { element Only {} form only = "only"; }
                root Root { punctuation = #punctuation; eoi = true; standalone_render = true; }
            })
            .expect_err("root punctuation must be one scalar")
            .to_string();
            assert!(
                message.contains("root punctuation must be exactly one Unicode scalar"),
                "{message}"
            );
            assert!(!message.contains("internal"), "{message}");
        }

        crate::generate(quote! {
            construction only: Root { element Only {} form only = "only"; }
            root Root { punctuation = "❤"; eoi = true; standalone_render = true; }
        })
        .expect("one non-ASCII Unicode scalar is valid punctuation metadata");

        let message = crate::generate(quote! {
            construction only: Root { element Only {} form only = "only"; }
            root Root { punctuation = "A"; eoi = true; standalone_render = true; }
        })
        .expect_err("alphanumeric root punctuation cannot delimit a preceding lexical surface")
        .to_string();
        assert!(
            message.contains("root punctuation must not be alphanumeric"),
            "{message}"
        );
    }

    #[test]
    fn binding_value_type_is_an_identifier_path_ending_in_the_binding_name() {
        let fixture = |value_type: syn::Type| {
            quote! {
                codec Thing {
                    atom = lex;
                    value_type = #value_type;
                    lexical = Lexical::Thing;
                    render = render_thing;
                    build { pattern = BuildValue::Thing(value); construct = value; }
                    traversal { callback = borrowed; argument = thing; call visitor::visit_thing(borrowed(thing)); }
                }
                construction only: Root { element Only { thing: lex Thing, } form only = lex(thing); }
                root Root { punctuation = "."; eoi = true; standalone_render = true; }
            }
        };

        crate::generate(fixture(syn::parse_quote!(crate::runtime::Thing)))
            .expect("an identifier-only namespace path with a matching final ident is supported");
        for invalid in [
            syn::parse_quote!((Thing, Thing)),
            syn::parse_quote!(&Thing),
            syn::parse_quote!(crate::runtime::Thing<u8>),
            syn::parse_quote!(<Thing as Trait>::Associated),
            syn::parse_quote!(Thing!()),
            syn::parse_quote!(crate::runtime::Other),
        ] {
            let message = crate::generate(fixture(invalid))
                .expect_err("unsupported binding type must fail validation")
                .to_string();
            assert!(message.contains("binding value_type"), "{message}");
            assert!(!message.contains("internal"), "{message}");
        }
    }

    #[test]
    fn generated_owned_paths_reject_unsupported_shapes_before_emission() {
        let cases = [
            (
                "generic construction category",
                quote! {
                    construction only: Root<u8> {
                        element Only {}
                        form only = "only";
                    }
                    root Root { punctuation = "."; eoi = true; standalone_render = true; }
                },
            ),
            (
                "generic category role",
                quote! {
                    construction child: Child { element ChildNode {} form child = "child"; }
                    construction parent: Parent {
                        element ParentNode { child: Child<u8>, }
                        form parent = child;
                    }
                    root Parent { punctuation = "."; eoi = true; standalone_render = true; }
                },
            ),
            (
                "qself category role",
                quote! {
                    construction child: Child { element ChildNode {} form child = "child"; }
                    construction parent: Parent {
                        element ParentNode { child: <Projection as Trait>::Child, }
                        form parent = child;
                    }
                    root Parent { punctuation = "."; eoi = true; standalone_render = true; }
                },
            ),
            (
                "parenthesized category role",
                quote! {
                    construction child: Child { element ChildNode {} form child = "child"; }
                    construction parent: Parent {
                        element ParentNode { child: Child(u8), }
                        form parent = child;
                    }
                    root Parent { punctuation = "."; eoi = true; standalone_render = true; }
                },
            ),
            (
                "associated generic category role",
                quote! {
                    construction child: Child { element ChildNode {} form child = "child"; }
                    construction parent: Parent {
                        element ParentNode { child: Child<Item = u8>, }
                        form parent = child;
                    }
                    root Parent { punctuation = "."; eoi = true; standalone_render = true; }
                },
            ),
            (
                "generic generated terminal role",
                quote! {
                    vocab Word { One = "one", }
                    construction only: Root {
                        element Only { word: lex Word<u8>, }
                        form only = lex(word);
                    }
                    root Root { punctuation = "."; eoi = true; standalone_render = true; }
                },
            ),
            (
                "generic generated verb variant",
                quote! {
                    morphology EnglishVerb { feature = Agreement; recipe = english_verb; }
                    lexeme Verbs using EnglishVerb { Act = "act", }
                    construction only: Root {
                        element Only {}
                        derive agreement = verb.agreement;
                        derive verb.agreement = Values::Bare;
                        form only = verb(Verbs<u8>::Act);
                    }
                    root Root { punctuation = "."; eoi = true; standalone_render = true; }
                },
            ),
            (
                "associated arguments on a lowered feature value",
                quote! {
                    construction only: Root {
                        element Only {}
                        derive agreement = Values<Item = u8>::Bare;
                        form only = "only";
                    }
                    root Root { punctuation = "."; eoi = true; standalone_render = true; }
                },
            ),
            (
                "generic generated root category",
                quote! {
                    construction only: Root { element Only {} form only = "only"; }
                    root Root<u8> { punctuation = "."; eoi = true; standalone_render = true; }
                },
            ),
        ];

        for (name, tokens) in cases {
            let message = crate::generate(tokens)
                .expect_err("unsupported generated-owned path must fail public generation")
                .to_string();
            assert!(
                message.contains(
                    "compiler-generated identity requires a qself-free, non-generic identifier path"
                ),
                "{name}: {message}",
            );
            assert!(!message.contains("internal"), "{name}: {message}");
        }
    }

    #[test]
    fn generic_external_runtime_callbacks_remain_supported() {
        let expansion = crate::generate(quote! {
            codec Thing {
                atom = lex;
                value_type = Thing;
                lexical = Lexical::Thing;
                render = runtime::render::<u8>;
                build { pattern = BuildValue::Thing(value); construct = value; }
                traversal {
                    callback = borrowed;
                    argument = thing;
                    call visitor::visit_thing(borrowed(thing));
                }
            }
            construction only: Root {
                element Only { thing: lex Thing, }
                form only = lex(thing);
            }
            root Root { punctuation = "."; eoi = true; standalone_render = true; }
        })
        .expect("external runtime callback paths may be generic");
        let output = expansion.tokens().to_string();
        assert!(output.contains("runtime :: render :: < u8 >"), "{output}");
    }

    fn assert_generated_traversal_callback_rejected(tokens: proc_macro2::TokenStream) {
        let message = crate::generate(tokens)
            .expect_err("generated traversal callback arguments must fail public generation")
            .to_string();
        assert!(
            message.contains(
                "compiler-generated identity requires a qself-free, non-generic identifier path"
            ),
            "{message}",
        );
        assert!(!message.contains("internal"), "{message}");
    }

    #[test]
    fn generated_traversal_callback_rejects_codec_local_walker_arguments() {
        assert_generated_traversal_callback_rejected(quote! {
            codec Thing {
                value_type = Thing;
                traversal {
                    callback = borrowed;
                    argument = thing;
                    call walk_thing::<u8>(borrowed(thing));
                }
            }
            construction only: Root { element Only {} form only = "only"; }
            root Root { punctuation = "."; eoi = true; standalone_render = true; }
        });
    }

    #[test]
    fn generated_traversal_callback_rejects_identity_visitor_arguments() {
        assert_generated_traversal_callback_rejected(quote! {
            identity Thing {
                value_type = Thing;
                traversal {
                    callback = borrowed;
                    argument = thing;
                    call visitor::visit_thing::<u8>(borrowed(thing));
                }
            }
            construction only: Root { element Only {} form only = "only"; }
            root Root { punctuation = "."; eoi = true; standalone_render = true; }
        });
    }

    #[test]
    fn generated_traversal_callback_rejects_branch_target_arguments() {
        assert_generated_traversal_callback_rejected(quote! {
            vocab Marker { One = "one", }
            codec Thing {
                value_type = Thing;
                traversal {
                    callback = borrowed;
                    argument = thing;
                    variant Marker;
                    match thing {
                        Thing::Marker(marker: Marker) =>
                            visitor::visit_marker::<Item = u8>(copy(marker)),
                    }
                }
            }
            construction only: Root { element Only {} form only = "only"; }
            root Root { punctuation = "."; eoi = true; standalone_render = true; }
        });
    }

    #[test]
    fn generated_traversal_callback_rejects_qself_target() {
        assert_generated_traversal_callback_rejected(quote! {
            codec Thing {
                value_type = Thing;
                traversal {
                    callback = borrowed;
                    argument = thing;
                    call <Callbacks as VisitorCallbacks>::walk_thing(borrowed(thing));
                }
            }
            construction only: Root { element Only {} form only = "only"; }
            root Root { punctuation = "."; eoi = true; standalone_render = true; }
        });
    }

    #[test]
    fn generated_traversal_callback_rejects_parenthesized_target() {
        assert_generated_traversal_callback_rejected(quote! {
            codec Thing {
                value_type = Thing;
                traversal {
                    callback = borrowed;
                    argument = thing;
                    call walk_thing(u8)(borrowed(thing));
                }
            }
            construction only: Root { element Only {} form only = "only"; }
            root Root { punctuation = "."; eoi = true; standalone_render = true; }
        });
    }

    #[test]
    fn semantic_identifier_domain_is_validated_before_sealing() {
        let message = crate::generate(quote! {
            codec Head {
                atom = noun;
                value_type = Head;
                lexical = Lexical::Head;
                render = render_head;
                build { pattern = BuildValue::Head(number); construct = number; }
                traversal {
                    callback = borrowed;
                    argument = number;
                    call visitor::visit_head(borrowed(number));
                }
            }
            construction only: Root {
                element Only { head: lex Head, }
                form only = noun(head);
            }
            root Root { punctuation = "."; eoi = true; standalone_render = true; }
        })
        .expect_err("an unrenamable noun scanner field collision must fail validation")
        .to_string();
        assert!(
            message.contains(
                "noun binding pattern slot `number` collides with the generated scanner-number field"
            ),
            "{message}"
        );
        assert!(!message.contains("internal"), "{message}");

        let declaration = error(quote! {
            vocab Marker { One = "one", }
            vocab r#Marker { Two = "two", }
            construction only: Root { element Only {} form only = "only"; }
            root Root { punctuation = "."; eoi = true; standalone_render = true; }
        });
        assert!(
            declaration.contains("duplicate declaration `r#Marker`"),
            "{declaration}"
        );

        let product = error(quote! {
            vocab Marker { One = "one", }
            construction only: Root {
                element Only { payload: lex Marker, r#payload: lex Marker, }
                form only = lex(payload) lex(r#payload);
            }
            root Root { punctuation = "."; eoi = true; standalone_render = true; }
        });
        assert!(product.contains("duplicate field `r#payload`"), "{product}");

        let build_slots = error(quote! {
            codec Pair {
                atom = lex;
                value_type = Pair;
                lexical = Lexical::Pair;
                render = render_pair;
                build {
                    pattern = BuildValue::Pair(payload, r#payload);
                    construct = Pair::new(payload, r#payload);
                }
                traversal {
                    callback = borrowed;
                    argument = pair;
                    call visitor::visit_pair(borrowed(pair));
                }
            }
            construction only: Root {
                element Only { pair: lex Pair, }
                form only = lex(pair);
            }
            root Root { punctuation = "."; eoi = true; standalone_render = true; }
        });
        assert!(
            build_slots.contains("duplicate binding pattern slot `r#payload`"),
            "{build_slots}"
        );
        assert!(!build_slots.contains("internal"), "{build_slots}");

        let traversal_fields = error(quote! {
            codec Runtime {
                value_type = Runtime;
                traversal {
                    callback = borrowed;
                    argument = runtime;
                    leaf visit_piece: i32 = copy;
                    field payload: i32;
                    field r#payload: i32;
                    call visitor::visit_piece(copy(payload));
                    call visitor::r#visit_piece(copy(r#payload));
                }
            }
            construction only: Root { element Only {} form only = "only"; }
            root Root { punctuation = "."; eoi = true; standalone_render = true; }
        });
        assert!(
            traversal_fields.contains("duplicate traversal field `r#payload`"),
            "{traversal_fields}"
        );
        assert!(
            !traversal_fields.contains("unknown traversal callback"),
            "{traversal_fields}"
        );

        let traversal_argument = error(quote! {
            codec Runtime {
                value_type = Runtime;
                traversal {
                    callback = borrowed;
                    argument = payload;
                    leaf visit_piece: Runtime = borrowed;
                    field r#payload: Runtime;
                    call visitor::visit_piece(borrowed(payload));
                }
            }
            construction only: Root { element Only {} form only = "only"; }
            root Root { punctuation = "."; eoi = true; standalone_render = true; }
        });
        assert!(
            traversal_argument.contains("duplicate traversal field `r#payload`"),
            "{traversal_argument}"
        );

        let raw_number = error(quote! {
            codec Head {
                atom = noun;
                value_type = Head;
                lexical = Lexical::Head;
                render = render_head;
                build { pattern = BuildValue::Head(r#number); construct = r#number; }
                traversal {
                    callback = borrowed;
                    argument = head;
                    call visitor::visit_head(borrowed(head));
                }
            }
            construction only: Root {
                element Only { head: lex Head, }
                form only = noun(head);
            }
            root Root { punctuation = "."; eoi = true; standalone_render = true; }
        });
        assert!(
            raw_number.contains(
                "noun binding pattern slot `number` collides with the generated scanner-number field"
            ),
            "{raw_number}"
        );
        assert!(!raw_number.contains("internal"), "{raw_number}");

        crate::generate(quote! {
            vocab r#Marker { One = "one", }
            construction only: Root {
                element Only { r#payload: lex Marker, }
                form only = lex(payload);
            }
            root Root { punctuation = "."; eoi = true; standalone_render = true; }
        })
        .expect("raw declarations and fields resolve through ordinary references");

        crate::generate(quote! {
            codec Part {
                value_type = Part;
                traversal { callback = copy; argument = part; variant Value; }
            }
            codec Runtime {
                value_type = Runtime;
                traversal {
                    callback = borrowed;
                    argument = r#runtime;
                    variant Part;
                    match runtime {
                        Runtime::Part(r#payload: Part) => r#walk_part(copy(payload)),
                    }
                }
            }
            construction only: Root { element Only {} form only = "only"; }
            root Root { punctuation = "."; eoi = true; standalone_render = true; }
        })
        .expect("raw traversal arguments, bindings, and callbacks resolve canonically");

        crate::generate(quote! {
            vocab r#Mode { r#One = "one", }
            construction only: Root {
                element Only { r#mode: lex Mode, }
                derive agreement = r#mode.agreement;
                derive mode.agreement = match r#mode {
                    One => Values::Bare,
                };
                form only = lex(mode);
            }
            root Root { punctuation = "."; eoi = true; standalone_render = true; }
        })
        .expect("feature places, match variants, and role references share semantic keys");
    }

    #[test]
    fn partial_category_feature_providers_fail_validation() {
        for (feature, equation) in [
            ("agreement", quote! { derive agreement = Values::Bare; }),
            ("number", quote! { derive number = Values::Singular; }),
        ] {
            let message = crate::generate(quote! {
                construction provider: Root {
                    element Provider {}
                    #equation
                    form provider = "provider";
                }
                construction missing: Root {
                    element Missing {}
                    form missing = "missing";
                }
                root Root { punctuation = "."; eoi = true; standalone_render = true; }
            })
            .expect_err("category feature providers must be total before sealing")
            .to_string();
            assert!(
                message.contains("category feature provider uniformity")
                    && message.contains(feature)
                    && message.contains("missing"),
                "{message}"
            );
            assert!(!message.contains("internal"), "{message}");
        }
    }

    #[test]
    fn feature_composition_matrix_matches_backend_lowerability() {
        let head_binding = quote! {
            codec Head {
                atom = noun;
                value_type = Head;
                lexical = Lexical::Head;
                render = render_head;
                build { pattern = BuildValue::Head(head); construct = head; }
                traversal {
                    callback = borrowed;
                    argument = head;
                    call visitor::visit_head(borrowed(head));
                }
            }
        };
        let accepted = [
            (
                "constant construction features",
                quote! {
                    #head_binding
                    construction only: Root {
                        element Only { head: lex Head, }
                        derive agreement = Values::Bare;
                        derive number = Values::Singular;
                        form only = noun(head);
                    }
                    root Root { punctuation = "."; eoi = true; standalone_render = true; }
                },
            ),
            (
                "category FromRole with a noun",
                crate::test_support::role_derived_noun_tokens(),
            ),
            (
                "same-writer paired matches with zero nouns",
                crate::test_support::vocab_matched_number_without_noun_tokens(),
            ),
            (
                "number match with one noun",
                quote! {
                    vocab Count { One = "one", Many = "many", }
                    #head_binding
                    construction only: Root {
                        element Only { count: lex Count, head: lex Head, }
                        derive number = match count {
                            One => Values::Singular,
                            Many => Values::Plural,
                        };
                        form only = lex(count) noun(head);
                    }
                    root Root { punctuation = "."; eoi = true; standalone_render = true; }
                },
            ),
            (
                "number match with two nouns",
                crate::test_support::vocab_matched_number_with_two_nouns_tokens(),
            ),
            (
                "refined dynamic vocabulary role",
                quote! {
                    vocab Count { One = "one", Many = "many", }
                    construction only: Root {
                        element Only { count: lex Count, }
                        require count is One;
                        derive number = match count {
                            One => Values::Singular,
                            Many => Values::Plural,
                        };
                        form only = lex(count);
                    }
                    root Root { punctuation = "."; eoi = true; standalone_render = true; }
                },
            ),
            (
                "agreement match with constant noun number",
                quote! {
                    vocab Count { One = "one", Many = "many", }
                    #head_binding
                    construction only: Root {
                        element Only { count: lex Count, head: lex Head, }
                        derive agreement = match count {
                            One => Values::ThirdPersonSingular,
                            Many => Values::Bare,
                        };
                        derive number = Values::Singular;
                        form only = lex(count) noun(head);
                    }
                    root Root { punctuation = "."; eoi = true; standalone_render = true; }
                },
            ),
            (
                "full category providers",
                quote! {
                    construction first: Root {
                        element First {}
                        derive agreement = Values::Bare;
                        derive number = Values::Singular;
                        form first = "first";
                    }
                    construction second: Root {
                        element Second {}
                        derive agreement = Values::ThirdPersonSingular;
                        derive number = Values::Plural;
                        form second = "second";
                    }
                    root Root { punctuation = "."; eoi = true; standalone_render = true; }
                },
            ),
        ];
        for (name, fixture) in accepted {
            if let Err(error) = crate::generate(fixture) {
                let message = error.to_string();
                assert!(!message.contains("internal"), "{name}: {message}");
                panic!("accepted feature composition `{name}` failed: {message}");
            }
        }

        let rejected = [
            (
                "different match writers",
                quote! {
                    vocab Count { One = "one", Many = "many", }
                    vocab Tone { Plain = "plain", Marked = "marked", }
                    construction only: Root {
                        element Only { count: lex Count, tone: lex Tone, }
                        derive agreement = match count {
                            One => Values::ThirdPersonSingular,
                            Many => Values::Bare,
                        };
                        derive number = match tone {
                            Plain => Values::Singular,
                            Marked => Values::Plural,
                        };
                        form only = lex(count) lex(tone);
                    }
                    root Root { punctuation = "."; eoi = true; standalone_render = true; }
                },
                "feature equation composition",
            ),
            (
                "lexical FromRole noun source",
                quote! {
                    vocab Count { One = "one", Many = "many", }
                    #head_binding
                    construction only: Root {
                        element Only { count: lex Count, head: lex Head, }
                        derive count.number = match count {
                            One => Values::Singular,
                            Many => Values::Plural,
                        };
                        derive number = count.number;
                        form only = lex(count) noun(head);
                    }
                    root Root { punctuation = "."; eoi = true; standalone_render = true; }
                },
                "derive target unimplemented in MVP",
            ),
            (
                "partial agreement provider",
                quote! {
                    construction provider: Root {
                        element Provider {}
                        derive agreement = Values::Bare;
                        form provider = "provider";
                    }
                    construction missing: Root {
                        element Missing {}
                        form missing = "missing";
                    }
                    root Root { punctuation = "."; eoi = true; standalone_render = true; }
                },
                "category feature provider uniformity",
            ),
            (
                "partial number provider",
                quote! {
                    construction provider: Root {
                        element Provider {}
                        derive number = Values::Singular;
                        form provider = "provider";
                    }
                    construction missing: Root {
                        element Missing {}
                        form missing = "missing";
                    }
                    root Root { punctuation = "."; eoi = true; standalone_render = true; }
                },
                "category feature provider uniformity",
            ),
        ];
        for (name, fixture, diagnostic) in rejected {
            let message = crate::generate(fixture)
                .expect_err("a deferred composition must fail validation")
                .to_string();
            assert!(message.contains(diagnostic), "{name}: {message}");
            assert!(!message.contains("internal"), "{name}: {message}");
        }
    }

    #[test]
    fn task_11_feature_chains_generate_independent_of_source_and_equation_order() {
        let fixtures = [
            (
                "category constant to construction, provider declared later",
                quote! {
                    construction parent: Parent {
                        element ParentNode { child: Child, }
                        derive agreement = child.agreement;
                        derive child.agreement = Values::Bare;
                        form parent = child;
                    }
                    construction child: Child {
                        element ChildNode {}
                        derive agreement = Values::Bare;
                        form child = "child";
                    }
                    construction entry: Entry { element EntryNode {} form entry = "entry"; }
                    root Entry { punctuation = "."; eoi = true; standalone_render = true; }
                },
            ),
            (
                "implicit verb constant to construction",
                quote! {
                    morphology EnglishVerb { feature = Agreement; recipe = english_verb; }
                    lexeme Verbs using EnglishVerb { Act = "act", }
                    construction action: Action {
                        element ActionNode {}
                        derive agreement = verb.agreement;
                        derive verb.agreement = Values::Bare;
                        form action = verb(Verbs::Act);
                    }
                    construction entry: Entry { element EntryNode {} form entry = "entry"; }
                    root Entry { punctuation = "."; eoi = true; standalone_render = true; }
                },
            ),
            (
                "refined writer through category to construction",
                quote! {
                    vocab Mode { One = "one", Many = "many", }
                    construction parent: Parent {
                        element ParentNode { child: Child, mode: lex Mode, }
                        require mode is One;
                        derive agreement = child.agreement;
                        derive child.agreement = mode.agreement;
                        derive mode.agreement = match mode {
                            One => Values::ThirdPersonSingular,
                            Many => Values::Bare,
                        };
                        form parent = child lex(mode);
                    }
                    construction bare: Child {
                        element BareChild {}
                        derive agreement = Values::Bare;
                        form bare = "bare";
                    }
                    construction third: Child {
                        element ThirdChild {}
                        derive agreement = Values::ThirdPersonSingular;
                        form third = "third";
                    }
                    construction entry: Entry { element EntryNode {} form entry = "entry"; }
                    root Entry { punctuation = "."; eoi = true; standalone_render = true; }
                },
            ),
            (
                "bound category chain with source after consumer",
                quote! {
                    construction pair: Pair {
                        element PairNode { right: Child, left: Child, }
                        derive agreement = right.agreement;
                        derive right.agreement = left.agreement;
                        form pair = right left;
                    }
                    construction bare: Child {
                        element BareChild {}
                        derive agreement = Values::Bare;
                        form bare = "bare";
                    }
                    construction third: Child {
                        element ThirdChild {}
                        derive agreement = Values::ThirdPersonSingular;
                        form third = "third";
                    }
                    construction entry: Entry { element EntryNode {} form entry = "entry"; }
                    root Entry { punctuation = "."; eoi = true; standalone_render = true; }
                },
            ),
        ];

        for (name, fixture) in fixtures {
            if let Err(error) = crate::generate(fixture) {
                let message = error.to_string();
                assert!(!message.contains("internal"), "{name}: {message}");
                panic!("accepted transitive feature chain `{name}` failed: {message}");
            }
        }
    }

    #[test]
    fn task_11_contextual_category_requirements_fail_at_roles_and_roots() {
        let nested = crate::generate(quote! {
            morphology EnglishVerb { feature = Agreement; recipe = english_verb; }
            lexeme Verbs using EnglishVerb { Act = "act", }
            construction action: Child {
                element ActionNode {}
                derive agreement = verb.agreement;
                form action = verb(Verbs::Act);
            }
            construction parent: Parent {
                element ParentNode { child: Child, }
                form parent = child;
            }
            root Parent { punctuation = "."; eoi = true; standalone_render = true; }
        })
        .expect_err("a contextual category role requires an agreement writer")
        .to_string();
        assert!(nested.contains("child.agreement"), "{nested}");
        assert!(nested.contains("contextual category `Child`"), "{nested}");
        assert!(!nested.contains("internal"), "{nested}");

        let root = crate::generate(quote! {
            morphology EnglishVerb { feature = Agreement; recipe = english_verb; }
            lexeme Verbs using EnglishVerb { Act = "act", }
            construction action: Child {
                element ActionNode {}
                derive agreement = verb.agreement;
                form action = verb(Verbs::Act);
            }
            root Child { punctuation = "."; eoi = true; standalone_render = true; }
        })
        .expect_err("a contextual category cannot be a standalone render root")
        .to_string();
        assert!(root.contains("standalone render root `Child`"), "{root}");
        assert!(root.contains("external agreement"), "{root}");
        assert!(!root.contains("internal"), "{root}");
    }

    #[test]
    fn rejects_missing_duplicate_and_incapable_roots() {
        let missing =
            error(quote! { construction only: Cat { element Only {} form only = "only"; } });
        assert!(missing.contains("at least one root"), "{missing}");

        let duplicate = error(quote! {
            construction only: Cat { element Only {} form only = "only"; }
            root Cat { punctuation = "."; eoi = true; standalone_render = false; }
            root Cat { punctuation = "!"; eoi = false; standalone_render = true; }
        });
        assert!(
            duplicate.contains("duplicate root for category `Cat`"),
            "{duplicate}"
        );

        let coverage = error(quote! {
            construction a_rule: A { element ARule {} form a = "a"; }
            construction b_rule: B { element BRule {} form b = "b"; }
            root A { punctuation = "."; eoi = true; standalone_render = false; }
            root B { punctuation = "."; eoi = false; standalone_render = false; }
        });
        assert!(coverage.contains("standalone render entry"), "{coverage}");
    }

    #[test]
    fn roots_accept_declared_structural_types_but_reject_unknown_names() {
        validate(quote! {
            construction only: Cat { element Only {} form only = "only"; }
            abstract product Document { child: Cat, }
            root Document { eoi = true; standalone_render = true; }
        })
        .expect("a declared abstract product is a valid root type");

        let unknown = error(quote! {
            construction only: Cat { element Only {} form only = "only"; }
            root Missing { eoi = true; standalone_render = true; }
        });
        assert!(
            unknown.contains("unknown root category `Missing`"),
            "{unknown}",
        );
    }

    #[test]
    fn exposes_sealed_codegen_contribution_and_lowering_records() {
        let validated = validate(quote! {
            vocab Words { One = "one", }
            construction recursive: Node {
                element Recursive { child: Node, word: lex Words, }
                form recursive = child lex(word);
            }
            root Node { punctuation = "."; eoi = true; standalone_render = true; }
        })
        .expect("complete declaration yields a codegen inventory");

        assert!(
            validated
                .boxed_fields()
                .contains(&("recursive".to_owned(), "child".to_owned()))
        );
        assert!(validated.dynamic_number_constructions().is_empty());
        let semantic = validated.semantic();
        assert_eq!(semantic.constructions().len(), 1);
        let construction = &semantic.constructions()[0];
        assert_eq!(construction.construction_id(), "recursive");
        assert_eq!(construction.element_type(), "Recursive");
        assert_eq!(construction.category_variant(), "Recursive");
        assert_eq!(semantic.terminals().len(), 1);
        let terminal = &semantic.terminals()[0];
        assert!(terminal.has_direct_render());
        assert!(terminal.has_direct_build());
        assert!(terminal.has_traversal());
        assert_eq!(semantic.roots().len(), 1);
    }

    #[test]
    fn synthetic_projection_fixture_validates_mechanisms_without_product_names() {
        let tokens = crate::test_support::synthetic_projection_tokens();
        let raw = crate::parse_declarations(tokens.clone())
            .expect("synthetic projection declarations parse through the public API");
        let validated = crate::validate_declarations(raw)
            .expect("synthetic projection declarations validate together");
        let expansion = crate::generate(tokens).expect("synthetic projection declarations emit");

        assert_eq!(validated.semantic().constructions().len(), 6);
        assert_eq!(validated.semantic().terminals().len(), 8);
        assert_eq!(validated.semantic().roots().len(), 1);
        assert_eq!(expansion.plan().items().len(), 104);
        assert!(expansion.items().iter().any(|item| {
            matches!(
                &item.key,
                crate::ItemKey::Named {
                    kind: crate::NamedKind::Function,
                    name,
                } if name == "scan_lexical"
            )
        }));
        assert!(
            validated
                .boxed_fields()
                .contains(&("nested".to_owned(), "next".to_owned()))
        );
        assert!(validated.dynamic_number_constructions().contains("leaf"));

        let invariant_access = validated
            .semantic()
            .constructions()
            .iter()
            .flat_map(|construction| {
                construction.fields().iter().filter_map(|field| {
                    field.accessor_mode().map(|mode| {
                        (
                            construction.construction_id().to_owned(),
                            field.name_key(),
                            mode,
                        )
                    })
                })
            })
            .collect::<Vec<_>>();
        assert_eq!(
            invariant_access,
            [
                (
                    "solo".to_owned(),
                    "mode".to_owned(),
                    crate::semantic::AccessorMode::Copy,
                ),
                (
                    "document".to_owned(),
                    "subject".to_owned(),
                    crate::semantic::AccessorMode::Borrow,
                ),
            ],
        );

        let terminal = |name| {
            validated
                .semantic()
                .terminals()
                .iter()
                .find(|terminal| terminal.name() == name)
                .unwrap_or_else(|| panic!("synthetic terminal `{name}` is planned"))
        };
        assert!(terminal("ActionStem").supports_verb_atom());
        assert!(!terminal("ObjectStem").supports_verb_atom());
        assert!(terminal("Resource").supports_noun_atom());
        assert!(!terminal("Resource").supports_lex_atom());
        assert!(terminal("Pair").supports_lex_atom());
        assert!(!terminal("Pair").supports_noun_atom());
        assert!(terminal("Handle").supports_identity_atom());
        assert!(!terminal("Handle").supports_lex_atom());
        assert!(terminal("Record").has_traversal());

        assert_eq!(
            validated
                .semantic()
                .constructions()
                .iter()
                .map(|contribution| (
                    contribution.construction_id(),
                    contribution.category_variant(),
                    contribution.element_type(),
                ))
                .collect::<Vec<_>>(),
            [
                ("leaf", "Leaf", "LeafNode"),
                ("nested", "Nested", "NestedNode"),
                ("action", "Action", "ActionNode"),
                ("idle", "Idle", "IdleNode"),
                ("solo", "Solo", "SoloTag"),
                ("document", "Document", "DocumentNode"),
            ]
        );
    }

    #[test]
    fn semantic_plan_is_source_ordered_complete_and_repeatable() {
        let tokens = crate::test_support::representative_tokens();
        let first = crate::validate_declarations(
            crate::parse_declarations(tokens.clone()).expect("representative fixture parses"),
        )
        .expect("representative fixture validates");
        let second = crate::validate_declarations(
            crate::parse_declarations(tokens).expect("representative fixture parses again"),
        )
        .expect("representative fixture validates again");

        let expected = crate::semantic::SemanticSnapshot {
            declaration_keys: vec![
                (crate::DeclarationKind::Vocab, "Words".to_owned()),
                (crate::DeclarationKind::Lexeme, "Nouns".to_owned()),
                (crate::DeclarationKind::Lexeme, "Verbs".to_owned()),
                (crate::DeclarationKind::Codec, "SignedNumber".to_owned()),
                (crate::DeclarationKind::Morphology, "EnglishNoun".to_owned()),
                (crate::DeclarationKind::Morphology, "EnglishVerb".to_owned()),
                (crate::DeclarationKind::Construction, "leaf".to_owned()),
                (crate::DeclarationKind::Construction, "chain".to_owned()),
                (crate::DeclarationKind::Construction, "action".to_owned()),
                (crate::DeclarationKind::Root, "Action".to_owned()),
            ],
            constructions: vec![
                (
                    "leaf".to_owned(),
                    "WordLeaf".to_owned(),
                    "leaf".to_owned(),
                    vec!["lex(word)".to_owned()],
                ),
                (
                    "chain".to_owned(),
                    "Chain".to_owned(),
                    "chain".to_owned(),
                    vec!["category(next: Node)".to_owned(), "lex(word)".to_owned()],
                ),
                (
                    "action".to_owned(),
                    "ActionElement".to_owned(),
                    "action".to_owned(),
                    vec![
                        "verb(Verbs::Act)".to_owned(),
                        "category(node: Node)".to_owned(),
                    ],
                ),
            ],
            terminals: vec![
                (
                    "Words".to_owned(),
                    "vocab".to_owned(),
                    vec![
                        "lex".to_owned(),
                        "render".to_owned(),
                        "build".to_owned(),
                        "traversal".to_owned(),
                    ],
                ),
                (
                    "Nouns".to_owned(),
                    "lexeme".to_owned(),
                    vec!["traversal".to_owned()],
                ),
                (
                    "Verbs".to_owned(),
                    "lexeme".to_owned(),
                    vec!["verb".to_owned(), "traversal".to_owned()],
                ),
                (
                    "SignedNumber".to_owned(),
                    "codec".to_owned(),
                    vec![
                        "lex".to_owned(),
                        "render".to_owned(),
                        "build".to_owned(),
                        "traversal".to_owned(),
                    ],
                ),
            ],
            roots: vec![("Action".to_owned(), true, true)],
            feature_equations: vec![(
                "action".to_owned(),
                "agreement = verb.agreement; verb.agreement = Bare".to_owned(),
            )],
            boxed_fields: vec![("chain".to_owned(), "next".to_owned())],
            dynamic_number_constructions: vec![],
        };

        assert_eq!(first.semantic().snapshot(), expected);
        assert_eq!(first.semantic().snapshot(), second.semantic().snapshot());
    }

    #[test]
    fn semantic_plan_owns_every_existing_resolved_fact() {
        let tokens = crate::test_support::synthetic_projection_tokens();
        let raw = crate::parse_declarations(tokens.clone()).expect("synthetic fixture parses");
        let validated = crate::validate_declarations(raw).expect("synthetic fixture validates");
        let snapshot = validated.semantic().snapshot();
        let expansion = crate::generate(tokens).expect("synthetic fixture still emits");

        assert_eq!(
            snapshot.declaration_keys.len(),
            validated.declaration_count()
        );
        assert_eq!(
            snapshot.constructions,
            vec![
                (
                    "leaf".to_owned(),
                    "LeafNode".to_owned(),
                    "leaf".to_owned(),
                    vec!["lex(mode)".to_owned(), "noun(resource)".to_owned()]
                ),
                (
                    "nested".to_owned(),
                    "NestedNode".to_owned(),
                    "nested".to_owned(),
                    vec![
                        "literal".to_owned(),
                        "category(next: Expr)".to_owned(),
                        "lex(marker)".to_owned()
                    ]
                ),
                (
                    "action".to_owned(),
                    "ActionNode".to_owned(),
                    "action".to_owned(),
                    vec!["verb(ActionStem::Activate)".to_owned()]
                ),
                (
                    "idle".to_owned(),
                    "IdleNode".to_owned(),
                    "idle".to_owned(),
                    vec!["literal".to_owned()]
                ),
                (
                    "solo".to_owned(),
                    "SoloTag".to_owned(),
                    "solo".to_owned(),
                    vec!["lex(mode)".to_owned()]
                ),
                (
                    "document".to_owned(),
                    "DocumentNode".to_owned(),
                    "document".to_owned(),
                    vec![
                        "category(subject: Expr)".to_owned(),
                        "category(predicate: Predicate)".to_owned(),
                        "identity(handle)".to_owned(),
                        "lex(pair)".to_owned()
                    ]
                ),
            ]
        );
        assert_eq!(
            snapshot.terminals,
            vec![
                (
                    "Mode".to_owned(),
                    "vocab".to_owned(),
                    vec![
                        "lex".to_owned(),
                        "render".to_owned(),
                        "build".to_owned(),
                        "traversal".to_owned()
                    ]
                ),
                (
                    "ObjectStem".to_owned(),
                    "lexeme".to_owned(),
                    vec!["traversal".to_owned()]
                ),
                (
                    "ActionStem".to_owned(),
                    "lexeme".to_owned(),
                    vec!["verb".to_owned(), "traversal".to_owned()]
                ),
                (
                    "Resource".to_owned(),
                    "codec".to_owned(),
                    vec![
                        "noun".to_owned(),
                        "render".to_owned(),
                        "build".to_owned(),
                        "traversal".to_owned()
                    ]
                ),
                (
                    "Marker".to_owned(),
                    "codec".to_owned(),
                    vec![
                        "lex".to_owned(),
                        "render".to_owned(),
                        "build".to_owned(),
                        "traversal".to_owned()
                    ]
                ),
                (
                    "Handle".to_owned(),
                    "identity".to_owned(),
                    vec![
                        "identity".to_owned(),
                        "render".to_owned(),
                        "build".to_owned(),
                        "traversal".to_owned()
                    ]
                ),
                (
                    "Pair".to_owned(),
                    "codec".to_owned(),
                    vec![
                        "lex".to_owned(),
                        "render".to_owned(),
                        "build".to_owned(),
                        "traversal".to_owned()
                    ]
                ),
                (
                    "Record".to_owned(),
                    "identity".to_owned(),
                    vec!["identity".to_owned(), "traversal".to_owned()]
                ),
            ]
        );
        assert_eq!(snapshot.roots, vec![("Document".to_owned(), true, true)]);
        assert_eq!(
            snapshot.feature_equations,
            vec![
                (
                    "leaf".to_owned(),
                    "agreement = match mode { Solo => ThirdPersonSingular, Group => Bare }; number = match mode { Solo => Singular, Group => Plural }".to_owned()
                ),
                (
                    "nested".to_owned(),
                    "agreement = next.agreement; number = next.number".to_owned()
                ),
                ("action".to_owned(), "agreement = verb.agreement".to_owned()),
                ("idle".to_owned(), "agreement = Bare".to_owned()),
                (
                    "document".to_owned(),
                    "predicate.agreement = subject.agreement".to_owned()
                ),
            ]
        );
        assert_eq!(
            snapshot.boxed_fields,
            vec![("nested".to_owned(), "next".to_owned())]
        );
        assert_eq!(
            snapshot.dynamic_number_constructions,
            vec!["leaf".to_owned()]
        );
        assert_eq!(expansion.plan().items().len(), 104);
        assert!(expansion.items().iter().any(|item| {
            matches!(
                &item.key,
                crate::ItemKey::Named {
                    kind: crate::NamedKind::Function,
                    name,
                } if name == "scan_lexical"
            )
        }));
    }

    #[test]
    fn semantic_plan_seals_source_indexes_resolutions_and_capabilities() {
        let tokens = crate::test_support::synthetic_projection_tokens();
        let validated = {
            let raw = crate::parse_declarations(tokens).expect("synthetic fixture parses");
            crate::validate_declarations(raw).expect("synthetic fixture validates")
        };
        let semantic = validated.semantic();

        assert!(semantic.category_reads_feature("Expr", crate::feature::Feature::Agreement));
        assert!(semantic.category_reads_feature("Expr", crate::feature::Feature::Number));
        assert!(!semantic.category_reads_feature("Predicate", crate::feature::Feature::Agreement));
        assert!(!semantic.category_reads_feature("Tag", crate::feature::Feature::Number));

        assert_eq!(
            semantic
                .constructions()
                .iter()
                .map(|row| (row.source_index(), row.construction_id().to_owned()))
                .collect::<Vec<_>>(),
            [
                (10, "leaf".to_owned()),
                (11, "nested".to_owned()),
                (12, "action".to_owned()),
                (13, "idle".to_owned()),
                (14, "solo".to_owned()),
                (15, "document".to_owned()),
            ]
        );
        assert_eq!(
            semantic
                .terminals()
                .iter()
                .map(|row| match row {
                    crate::semantic::TerminalPlan::Vocab(value) =>
                        (value.source_index(), "vocab", value.name().to_owned(),),
                    crate::semantic::TerminalPlan::Lexeme(value) =>
                        (value.source_index(), "lexeme", value.name().to_owned(),),
                    crate::semantic::TerminalPlan::Binding(value) => (
                        value.source_index(),
                        match value.kind() {
                            crate::TerminalBindingKind::Codec => "codec",
                            crate::TerminalBindingKind::Identity => "identity",
                        },
                        value.name().to_owned(),
                    ),
                    crate::semantic::TerminalPlan::ContextIdentity(value) =>
                        (value.source_index(), "identity", value.name().to_owned(),),
                    crate::semantic::TerminalPlan::SignedDecimal(value) =>
                        (value.source_index(), "codec", value.codec_name().to_owned(),),
                    crate::semantic::TerminalPlan::DeclarationNoun(value) =>
                        (value.source_index(), "codec", value.codec_name().to_owned(),),
                })
                .collect::<Vec<_>>(),
            [
                (0, "vocab", "Mode".to_owned()),
                (1, "lexeme", "ObjectStem".to_owned()),
                (2, "lexeme", "ActionStem".to_owned()),
                (3, "codec", "Resource".to_owned()),
                (4, "codec", "Marker".to_owned()),
                (5, "identity", "Handle".to_owned()),
                (6, "codec", "Pair".to_owned()),
                (7, "identity", "Record".to_owned()),
            ]
        );
        assert_eq!(
            semantic
                .roots()
                .iter()
                .map(|row| (row.source_index(), row.category().to_owned()))
                .collect::<Vec<_>>(),
            [(16, "Document".to_owned())]
        );
        assert_eq!(
            semantic.feature_resolutions_snapshot(),
            vec![
                (
                    "leaf".to_owned(),
                    vec![
                        ("agreement".to_owned(), "Runtime".to_owned()),
                        ("number".to_owned(), "Runtime".to_owned())
                    ]
                ),
                (
                    "nested".to_owned(),
                    vec![
                        ("agreement".to_owned(), "Runtime".to_owned()),
                        ("number".to_owned(), "Runtime".to_owned())
                    ]
                ),
                (
                    "action".to_owned(),
                    vec![
                        ("agreement".to_owned(), "External".to_owned()),
                        ("verb.agreement".to_owned(), "External".to_owned())
                    ]
                ),
                (
                    "idle".to_owned(),
                    vec![("agreement".to_owned(), "Known(Bare)".to_owned())]
                ),
                ("solo".to_owned(), vec![]),
                (
                    "document".to_owned(),
                    vec![("predicate.agreement".to_owned(), "Runtime".to_owned())]
                ),
            ]
        );
        assert_eq!(
            semantic.category_render_capabilities_snapshot(),
            vec![
                ("Document".to_owned(), false, false, true),
                ("Expr".to_owned(), true, false, false),
                ("Predicate".to_owned(), true, true, false),
                ("Tag".to_owned(), false, false, false),
            ]
        );

        let emission = crate::plan::plan_emission(validated.semantic())
            .expect("the already validated semantic plan emits");
        assert_eq!(emission.items().len(), 104);
        assert!(emission.items().iter().any(|item| {
            matches!(
                &item.key,
                crate::ItemKey::Named {
                    kind: crate::NamedKind::Function,
                    name,
                } if name == "scan_lexical"
            )
        }));
    }
}
