use std::collections::BTreeSet;
use std::collections::HashMap;
use std::collections::HashSet;

use quote::ToTokens;
use syn::spanned::Spanned;

use crate::feature;
use crate::feature::Feature;
use crate::identifier::ADMISSIBLE_SITES_FIELD;
use crate::identifier::BUILD_FUNCTION;
use crate::identifier::CHECKED_BUILD_FUNCTION;
use crate::identifier::FIXED_RUNTIME_TYPE_NAMES;
use crate::identifier::PRIVATE_ROOT_RENDERER_PREFIX;
use crate::identifier::RIGHT_PERIPHERY_PREPOSITION_TRAIT;
use crate::identifier::RIGHTMOST_LEAF_CATEGORY_TRAIT;
use crate::identifier::RIGHTMOST_LEAF_IS_FUNCTION;
use crate::identifier::RIGHTMOST_LEAF_TRAIT;
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
use crate::model::FormGuardSource;
use crate::model::RequireExprSource;
use crate::model::RequireSubjectSource;
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
    pub(crate) fn carries_concord_class(self) -> bool {
        self.carries_output
    }

    pub(crate) fn requires_external_concord_class(self) -> bool {
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
    LexFixed {
        terminal: String,
        variant: String,
    },
    Marked {
        role: String,
        category: String,
        terminal: String,
        variant: String,
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
    VerbProjected {
        role: String,
        terminal: String,
    },
    OpenDeclaration {
        kind: crate::macro_def::DeclarationKind,
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
            | Self::LexFixed { terminal, .. }
            | Self::Marked { terminal, .. }
            | Self::Identity { terminal, .. }
            | Self::Noun { terminal, .. }
            | Self::VerbFixed { terminal, .. }
            | Self::VerbProjected { terminal, .. } => Some(terminal),
            Self::Literal | Self::Category { .. } | Self::OpenDeclaration { .. } => None,
        }
    }

    fn is_complete(&self) -> bool {
        match self {
            Self::Literal => true,
            Self::Category { role, category } => !role.is_empty() && !category.is_empty(),
            Self::Lex { role, terminal }
            | Self::Identity { role, terminal }
            | Self::Noun { role, terminal }
            | Self::VerbProjected { role, terminal } => !role.is_empty() && !terminal.is_empty(),
            Self::LexFixed { terminal, variant } | Self::VerbFixed { terminal, variant } => {
                !terminal.is_empty() && !variant.is_empty()
            }
            Self::Marked {
                role,
                category,
                terminal,
                variant,
            } => {
                !role.is_empty()
                    && !category.is_empty()
                    && !terminal.is_empty()
                    && !variant.is_empty()
            }
            Self::OpenDeclaration { name, .. } => !name.is_empty(),
        }
    }

    fn is_supported_by(&self, terminals: &HashMap<&str, &TerminalCapabilities>) -> bool {
        match self {
            Self::Literal | Self::Category { .. } | Self::OpenDeclaration { .. } => true,
            Self::Lex { terminal, .. } | Self::LexFixed { terminal, .. } => {
                terminals.get(terminal.as_str()).is_some_and(|info| {
                    info.supports_lex_atom() && info.has_direct_render_build_traversal()
                })
            }
            Self::Marked { terminal, .. } => terminals.get(terminal.as_str()).is_some_and(|info| {
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
            Self::VerbProjected { terminal, .. } => terminals
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
            Self::LexFixed { terminal, variant } => format!("lex({terminal}::{variant})"),
            Self::Marked {
                role,
                terminal,
                variant,
                ..
            } => format!("marked({terminal}::{variant}, {role})"),
            Self::Identity { role, .. } => format!("identity({role})"),
            Self::Noun { role, .. } => format!("noun({role})"),
            Self::VerbFixed { terminal, variant } => format!("verb({terminal}::{variant})"),
            Self::VerbProjected { role, .. } => format!("verb({role})"),
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
    declaration_verb: bool,
    concord_class_verb: bool,
    participle_verb: bool,
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
    let sequence_features = validate_sequence_feature_roles(&raw, &structural)?;
    validate_generated_owned_paths(&raw)?;
    let resolved = validate_resolution(&raw, &symbols)?;
    validate_owned_english_verb_lexemes(&raw, &resolved)?;
    validate_stored_fields(&raw)?;
    validate_mobile_roles(&raw)?;
    validate_bindings(&raw)?;
    let mut invariants = validate_invariants(&raw, &symbols)?;
    let (feature_equations, dynamic_numbers) = validate_features(&raw, &symbols)?;
    let feature_resolutions = seal_feature_resolutions(&raw, &feature_equations, &invariants);
    fold_unit_invariants(&raw, &mut invariants, &feature_resolutions)?;
    let category_render = seal_category_render_capabilities(&raw, &feature_resolutions);
    let concord_class_carry_sums = concord_class_carry_sums(&raw);
    validate_abstract_product_external_concord_class_sums(
        &structural,
        &category_render,
        &concord_class_carry_sums,
    )?;
    let category_reads = seal_category_feature_reads(&raw, &category_render);
    validate_contextual_concord_class_uses(&raw, &category_render)?;
    let mut boxed_fields = validate_category_graph(&raw);
    boxed_fields.extend(structural.boxed_fields.iter().cloned());
    validate_roots(&raw, &symbols, &category_render)?;
    validate_backend_completeness(&raw, &resolved)?;
    let ResolvedGrammar {
        atoms_by_construction,
        verb_lexeme_provider,
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
            sequence_features,
            concord_class_carry_sums,
            atoms_by_construction,
            invariants,
            verb_lexeme_provider.as_deref(),
        )?,
    })
}

pub(crate) fn validate_declaration_verb_consumers(semantic: &SemanticPlan) -> syn::Result<()> {
    let consumers = semantic
        .constructions()
        .iter()
        .flat_map(crate::semantic::ConstructionPlan::forms)
        .flat_map(crate::semantic::FormPlan::atoms)
        .filter_map(|atom| match atom.value_atom() {
            crate::semantic::AtomPlan::Lex { terminal, .. }
            | crate::semantic::AtomPlan::Marked { terminal, .. } => Some(terminal.as_str()),
            crate::semantic::AtomPlan::Literal(_)
            | crate::semantic::AtomPlan::SentenceInitialLiteral(_)
            | crate::semantic::AtomPlan::StructuralLiteral(_)
            | crate::semantic::AtomPlan::Category { .. }
            | crate::semantic::AtomPlan::LexFixed { .. }
            | crate::semantic::AtomPlan::Identity { .. }
            | crate::semantic::AtomPlan::Noun { .. }
            | crate::semantic::AtomPlan::VerbFixed { .. }
            | crate::semantic::AtomPlan::OpenDeclaration(_)
            | crate::semantic::AtomPlan::Bound { .. }
            | crate::semantic::AtomPlan::Circumfix { .. } => None,
        })
        .collect::<HashSet<_>>();
    let mut errors = None;

    for (_, declaration) in semantic.runtime_declaration_verbs() {
        let name = declaration.codec_name();
        if !consumers.contains(name) {
            combine(
                &mut errors,
                syn::Error::new(
                    proc_macro2::Span::call_site(),
                    format!(
                        "declaration_verb codec `{name}` has no construction consumer; every declared frame_set must build through the grammar"
                    ),
                ),
            );
        }
    }

    finish(errors)
}

fn validate_owned_english_verb_lexemes(
    raw: &Declarations,
    resolved: &ResolvedGrammar,
) -> syn::Result<()> {
    let mut owned = resolved
        .verb_lexeme_provider
        .iter()
        .cloned()
        .collect::<HashSet<_>>();
    for declaration in &raw.declarations {
        let Declaration::Codec(binding) = declaration else {
            continue;
        };
        let Some(crate::model::GeneratedCodecRecipe::DeclarationVerb(recipe)) = &binding.generated
        else {
            continue;
        };
        owned.extend(
            recipe
                .closed_slots
                .iter()
                .map(|slot| identifier_key(&slot.value)),
        );
    }

    let mut errors = None;
    for declaration in &raw.declarations {
        let Declaration::Lexeme(lexeme) = declaration else {
            continue;
        };
        if lexeme_recipe(raw, lexeme) != Some(crate::morphology::MorphologyRecipe::EnglishVerb) {
            continue;
        }
        let name = identifier_key(&lexeme.name);
        if !owned.contains(&name) {
            combine(
                &mut errors,
                syn::Error::new(
                    lexeme.name.span(),
                    format!(
                        "EnglishVerb lexeme `{name}` is unowned; it must provide a fixed verb atom or a declaration_verb closed branch"
                    ),
                ),
            );
        }
    }
    finish(errors)
}

fn concord_class_carry_sums(raw: &Declarations) -> HashSet<String> {
    let providers = feature_providers(raw);
    raw.declarations
        .iter()
        .filter_map(|declaration| {
            let Declaration::AbstractSum(sum) = declaration else {
                return None;
            };
            let name = identifier_key(&sum.name);
            providers
                .contains(&(name.clone(), ParsedFeature::ConcordClass))
                .then_some(name)
        })
        .collect()
}

fn validate_abstract_product_external_concord_class_sums(
    structural: &StructuralSemantics,
    category_render: &HashMap<String, CategoryRenderCapability>,
    concord_class_carry_sums: &HashSet<String>,
) -> syn::Result<()> {
    let mut external_sums = HashSet::new();
    loop {
        let before = external_sums.len();
        for sum in &structural.sums {
            if !concord_class_carry_sums.contains(sum.name()) {
                continue;
            }
            let requires_external = sum
                .alternatives()
                .iter()
                .any(|alternative| match alternative.value() {
                    ValueKindPlan::Category(category) => category_render
                        .get(category)
                        .is_some_and(|capability| capability.requires_external_concord_class()),
                    ValueKindPlan::Sum(nested) => external_sums.contains(nested),
                    ValueKindPlan::Product(_)
                    | ValueKindPlan::Lex(_)
                    | ValueKindPlan::Identity(_) => false,
                });
            if requires_external {
                external_sums.insert(sum.name().to_owned());
            }
        }
        if external_sums.len() == before {
            break;
        }
    }

    let mut errors = None;
    for product in &structural.products {
        for field in product.fields() {
            let ValueKindPlan::Sum(sum) = field.kind().value() else {
                continue;
            };
            if !external_sums.contains(sum) {
                continue;
            }
            combine(
                &mut errors,
                syn::Error::new(
                    field.span(),
                    format!(
                        "{}.{role}: ConcordClass-bearing sum `{sum}` requires external concord_class, but abstract products cannot declare feature writers",
                        product.name(),
                        role = field.name(),
                    ),
                ),
            );
        }
    }
    finish(errors)
}

fn validate_sequence_feature_roles(
    raw: &Declarations,
    structural: &StructuralSemantics,
) -> syn::Result<HashMap<(String, String), Vec<Feature>>> {
    let providers = feature_providers(raw);
    let mut errors = None;
    let mut result = HashMap::new();
    for declaration in &raw.declarations {
        let Declaration::Construction(construction) = declaration else { continue };
        let construction_id = identifier_key(&construction.name);
        let element = identifier_key(&construction.element.name);
        let fields = construction
            .element
            .fields
            .iter()
            .map(|field| (identifier_key(&field.name), field))
            .collect::<HashMap<_, _>>();
        let uses = sequence_feature_uses(construction, &fields, &element, &mut errors);
        for (role, uses) in uses {
            let mut distinct = uses
                .iter()
                .map(|(feature, _, _, _)| *feature)
                .collect::<HashSet<_>>()
                .into_iter()
                .collect::<Vec<_>>();
            distinct.sort_unstable_by_key(|feature| sequence_feature_order(*feature));
            let duplicate = distinct
                .iter()
                .find(|feature| sequence_feature_has_competing_equations(&uses, **feature));
            if let Some(feature) = duplicate {
                combine(
                    &mut errors,
                    syn::Error::new(
                        uses.iter()
                            .find(|(used, _, _, _)| used == feature)
                            .expect("duplicate feature has a use")
                            .1,
                        format!(
                            "{element}.{role}: sequence feature {} has more than one equation",
                            feature_name(*feature),
                        ),
                    ),
                );
                continue;
            }
            let mut resolved_features = Vec::new();
            for feature in distinct {
                let span = uses
                    .iter()
                    .find(|(used, _, _, _)| *used == feature)
                    .expect("distinct feature has a use")
                    .1;
                let Some(resolved_feature) = supported_sequence_feature(feature) else {
                    combine(
                        &mut errors,
                        syn::Error::new(
                            span,
                            format!(
                                "{element}.{role}: sequence feature propagation supports homogeneous concord_class or number, first-member onset, or last-member possessive ending, found {}",
                                feature_name(feature)
                            ),
                        ),
                    );
                    continue;
                };
                let Some(field) = fields.get(&role) else { continue };
                let FieldKind::Sequence { .. } = &field.kind else { continue };
                let Some(structural_field) = structural
                    .construction_fields
                    .get(&(construction_id.clone(), role.clone()))
                else {
                    continue;
                };
                let StructuralFieldKindPlan::Sequence { item, bounds, .. } =
                    structural_field.kind()
                else {
                    continue;
                };
                if bounds.min() == 0 {
                    combine(
                        &mut errors,
                        syn::Error::new(
                            span,
                            format!(
                                "{element}.{role}: sequence feature {} requires a statically nonempty sequence",
                                feature_name(feature),
                            ),
                        ),
                    );
                }
                let category = match (feature, item) {
                    (
                        ParsedFeature::ConcordClass,
                        ValueKindPlan::Category(category) | ValueKindPlan::Sum(category),
                    )
                    | (
                        ParsedFeature::Number
                        | ParsedFeature::Onset
                        | ParsedFeature::PossessiveEnding,
                        ValueKindPlan::Category(category),
                    ) => category,
                    (
                        ParsedFeature::ConcordClass,
                        ValueKindPlan::Lex(_)
                        | ValueKindPlan::Identity(_)
                        | ValueKindPlan::Product(_),
                    )
                    | (
                        ParsedFeature::Number
                        | ParsedFeature::Onset
                        | ParsedFeature::PossessiveEnding,
                        ValueKindPlan::Sum(_)
                        | ValueKindPlan::Lex(_)
                        | ValueKindPlan::Identity(_)
                        | ValueKindPlan::Product(_),
                    ) => {
                        combine(
                            &mut errors,
                            syn::Error::new(
                                span,
                                format!(
                                    "{element}.{role}: sequence feature {} requires feature-bearing category items",
                                    feature_name(feature),
                                ),
                            ),
                        );
                        continue;
                    }
                    _ => unreachable!("unsupported sequence feature was rejected"),
                };
                if !providers.contains(&(category.clone(), feature)) {
                    combine(
                        &mut errors,
                        syn::Error::new(
                            span,
                            format!(
                                "{element}.{role}: category `{category}` does not provide {}",
                                feature_name(feature),
                            ),
                        ),
                    );
                    continue;
                }
                resolved_features.push(resolved_feature);
            }
            if !resolved_features.is_empty() {
                result.insert((element.clone(), role), resolved_features);
            }
        }
    }
    finish(errors)?;
    Ok(result)
}

fn sequence_feature_has_competing_equations(
    uses: &[(ParsedFeature, proc_macro2::Span, usize, bool)],
    feature: ParsedFeature,
) -> bool {
    [false, true].into_iter().any(|is_writer| {
        uses.iter()
            .filter(|(used, _, _, writer)| *used == feature && *writer == is_writer)
            .map(|(_, _, equation, _)| *equation)
            .collect::<HashSet<_>>()
            .len()
            > 1
    })
}

fn sequence_feature_order(feature: ParsedFeature) -> usize {
    match feature {
        ParsedFeature::ConcordClass => 0,
        ParsedFeature::Number => 1,
        ParsedFeature::Onset => 2,
        ParsedFeature::PossessiveEnding => 3,
        _ => 4,
    }
}

fn supported_sequence_feature(feature: ParsedFeature) -> Option<Feature> {
    match feature {
        ParsedFeature::ConcordClass => Some(Feature::ConcordClass),
        ParsedFeature::Number => Some(Feature::Number),
        ParsedFeature::Onset => Some(Feature::Onset),
        ParsedFeature::PossessiveEnding => Some(Feature::PossessiveEnding),
        _ => None,
    }
}

fn sequence_feature_uses(
    construction: &crate::Construction,
    fields: &HashMap<String, &crate::model::Field>,
    element: &str,
    errors: &mut Option<syn::Error>,
) -> HashMap<String, Vec<(ParsedFeature, proc_macro2::Span, usize, bool)>> {
    let mut uses: HashMap<String, Vec<(ParsedFeature, proc_macro2::Span, usize, bool)>> =
        HashMap::new();
    for (equation_index, equation) in construction.equations.iter().enumerate() {
        if let ParsedFeaturePlace::Role { field, feature } = &equation.target
            && matches!(
                fields.get(&identifier_key(field)).map(|field| &field.kind),
                Some(FieldKind::Sequence { .. })
            )
        {
            if *feature == ParsedFeature::Onset {
                combine(
                    errors,
                    syn::Error::new(
                        field.span(),
                        format!(
                            "{element}.{}: sequence onset is a first-member relay; derive construction onset from `{}.onset` instead",
                            identifier_key(field),
                            identifier_key(field),
                        ),
                    ),
                );
            } else {
                uses.entry(identifier_key(field)).or_default().push((
                    *feature,
                    field.span(),
                    equation_index,
                    true,
                ));
            }
        }
        if let ParsedFeatureValue::FromRole(source) = &equation.value
            && matches!(
                fields
                    .get(&identifier_key(&source.role))
                    .map(|field| &field.kind),
                Some(FieldKind::Sequence { .. })
            )
        {
            uses.entry(identifier_key(&source.role)).or_default().push((
                source.feature,
                source.role.span(),
                equation_index,
                false,
            ));
        }
    }
    uses
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
    let explicit_sum_owned = explicit_sum_owned_category_names(raw);
    let mut products = raw
        .declarations
        .iter()
        .filter_map(|declaration| match declaration {
            Declaration::AbstractProduct(product) => Some(identifier_key(&product.name)),
            _ => None,
        })
        .collect::<HashSet<_>>();
    products.extend(
        raw.declarations
            .iter()
            .filter_map(|declaration| match declaration {
                Declaration::Construction(construction)
                    if explicit_sum_owned.contains(&path_name(&construction.category)) =>
                {
                    Some(identifier_key(&construction.element.name))
                }
                _ => None,
            }),
    );
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
                let explicit_sum_owned =
                    explicit_sum_owned.contains(&path_name(&construction.category));
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
                    node: if explicit_sum_owned {
                        structural_product_node(&owner)
                    } else {
                        structural_construction_node(&identifier_key(&construction.name))
                    },
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
        if let Declaration::Construction(construction) = declaration
            && !explicit_sum_owned.contains(&path_name(&construction.category))
        {
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
    construction.forms.iter().all(|form| {
        form.atoms
            .iter()
            .any(|atom| form_atom_has_fixed_width(atom, &fields))
    })
}

fn form_atom_has_fixed_width(atom: &FormAtom, fields: &HashMap<String, &FieldKind>) -> bool {
    match atom {
        FormAtom::Literal(value)
        | FormAtom::LicensedLiteral(value)
        | FormAtom::SentenceInitial(value)
        | FormAtom::StructuralLiteral(value) => !value.value().is_empty(),
        FormAtom::Lex(role) | FormAtom::Identity(role) | FormAtom::Noun(role) => {
            fields.get(&identifier_key(role)).is_some_and(|kind| {
                !matches!(kind, FieldKind::Optional(_) | FieldKind::Sequence { .. })
            })
        }
        FormAtom::FixedLex(_) | FormAtom::Verb(_) | FormAtom::OpenVerb(_) => true,
        FormAtom::Marked(marked) => fields
            .get(&identifier_key(&marked.role))
            .is_some_and(|kind| {
                !matches!(kind, FieldKind::Optional(_) | FieldKind::Sequence { .. })
            }),
        FormAtom::Role(_) => false,
        FormAtom::Bound(bound) => form_atom_has_fixed_width(&bound.value, fields),
        FormAtom::Circumfix(circumfix) => {
            !circumfix.prefix.value().is_empty() || !circumfix.suffix.value().is_empty()
        }
    }
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
        RequireExprSource::In { .. } | RequireExprSource::OptionalPresence { .. } => {}
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
                FieldKind::Zeroable { item, .. } => (
                    resolve_structural_field_value(item, &label, products, sums, symbols, errors)
                        .map(StructuralFieldKindPlan::Zeroable),
                    None,
                    true,
                ),
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
        FieldKind::Zeroable { .. } | FieldKind::Optional(_) | FieldKind::Sequence { .. } => {
            return None;
        }
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
        FieldKind::Zeroable { .. } | FieldKind::Optional(_) | FieldKind::Sequence { .. } => {
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
    use crate::model::SurfaceCaseTransition;

    if source.transition != SurfaceCaseTransition::Preserve && role == "terminator" {
        let transition = match source.transition {
            SurfaceCaseTransition::Preserve => unreachable!(),
            SurfaceCaseTransition::SentenceInitial => "sentence_initial",
            SurfaceCaseTransition::Continuation => "continuation",
        };
        let allowed_roles = if source.transition == SurfaceCaseTransition::Continuation {
            "sequence separators"
        } else {
            "form surfaces and sequence separators"
        };
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
            syn::Error::new(
                span,
                format!("{label}: {transition} is supported only on {allowed_roles}"),
            ),
        );
    }
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
    let surface = FixedSurfacePlan::new(atoms, source.transition);
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
            syn::Error::new(
                span,
                if source.transition == SurfaceCaseTransition::SentenceInitial {
                    format!("{label}: sentence_initial target must realize at least one byte")
                } else if source.transition == SurfaceCaseTransition::Continuation {
                    format!("{label}: continuation target must realize at least one byte")
                } else {
                    format!("{label}: empty {role} surface")
                },
            ),
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
        StructuralFieldKindPlan::Zeroable(_) | StructuralFieldKindPlan::Optional(_) => true,
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
        StructuralFieldKindPlan::Required(_)
        | StructuralFieldKindPlan::Zeroable(_)
        | StructuralFieldKindPlan::Optional(_) => true,
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
            Declaration::Vocab(vocab) => {
                validate_vocab_declaration_shape(vocab, &mut independent_errors);
            }
            Declaration::Construction(_)
            | Declaration::AbstractProduct(_)
            | Declaration::AbstractSum(_)
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
            MorphologyRecipe::EnglishVerb | MorphologyRecipe::EnglishParticiple => {}
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
    if let Some((_, first)) = noun_providers.first() {
        for (name, provider) in noun_providers.iter().skip(1) {
            combine(
                &mut errors,
                syn::Error::new(
                    name.span(),
                    format!("multiple noun lexeme providers `{first}` and `{provider}`"),
                ),
            );
        }
    }
    finish(errors)
}

fn validate_vocab_declaration_shape(vocab: &crate::model::Vocab, errors: &mut Option<syn::Error>) {
    let mut feature_defaults = HashSet::new();
    for default in &vocab.feature_defaults {
        if !matches!(
            default.feature,
            crate::model::Feature::BareDurationLicense
                | crate::model::Feature::BareLocativeComplement
                | crate::model::Feature::DeterminerNumber
                | crate::model::Feature::ModifierLicense
                | crate::model::Feature::NominalLicense
                | crate::model::Feature::HomographLicense
                | crate::model::Feature::PrepositionComplementKind
                | crate::model::Feature::PrepositionAttachment
        ) {
            combine(
                errors,
                syn::Error::new(
                    default.value.span(),
                    "closed vocab metadata supports only BareDurationLicense, BareLocativeComplement, DeterminerNumber, HomographLicense, ModifierLicense, NominalLicense, PrepositionAttachment, and PrepositionComplementKind",
                ),
            );
        }
        if !feature_defaults.insert(default.feature) {
            combine(
                errors,
                syn::Error::new(default.value.span(), "duplicate vocab feature default"),
            );
        }
        if crate::feature::Feature::from(default.feature)
            .member(&default.value)
            .is_err()
        {
            combine(
                errors,
                syn::Error::new(default.value.span(), "invalid vocab feature default"),
            );
        }
    }
    for variant in &vocab.variants {
        let mut feature_overrides = HashSet::new();
        for override_ in &variant.feature_overrides {
            if !matches!(
                override_.feature,
                crate::model::Feature::BareDurationLicense
                    | crate::model::Feature::BareLocativeComplement
                    | crate::model::Feature::DeterminerNumber
                    | crate::model::Feature::ModifierLicense
                    | crate::model::Feature::NominalLicense
                    | crate::model::Feature::HomographLicense
                    | crate::model::Feature::PrepositionComplementKind
                    | crate::model::Feature::PrepositionAttachment
            ) {
                combine(
                    errors,
                    syn::Error::new(
                        override_.value.span(),
                        "closed vocab metadata supports only BareDurationLicense, BareLocativeComplement, DeterminerNumber, HomographLicense, ModifierLicense, NominalLicense, PrepositionAttachment, and PrepositionComplementKind",
                    ),
                );
            }
            if !feature_overrides.insert(override_.feature) {
                combine(
                    errors,
                    syn::Error::new(override_.value.span(), "duplicate vocab feature override"),
                );
            }
            if crate::feature::Feature::from(override_.feature)
                .member(&override_.value)
                .is_err()
            {
                combine(
                    errors,
                    syn::Error::new(override_.value.span(), "invalid vocab feature override"),
                );
            }
        }
    }
    let explicit_features = vocab
        .variants
        .iter()
        .flat_map(|variant| variant.feature_overrides.iter().map(|row| row.feature))
        .collect::<HashSet<_>>();
    for feature in explicit_features.difference(&feature_defaults) {
        for variant in &vocab.variants {
            if !variant
                .feature_overrides
                .iter()
                .any(|row| row.feature == *feature)
            {
                combine(
                    errors,
                    syn::Error::new(
                        variant.name.span(),
                        format!(
                            "vocab feature `{}` without a default must be declared on every member",
                            crate::feature::Feature::from(*feature).key()
                        ),
                    ),
                );
            }
        }
    }
}

fn validate_lexeme_declaration_shape(
    lexeme: &crate::model::Lexeme,
    errors: &mut Option<syn::Error>,
) {
    let mut feature_defaults = HashSet::new();
    for default in &lexeme.feature_defaults {
        if !matches!(
            default.feature,
            crate::model::Feature::BareLocativeLicense
                | crate::model::Feature::Compoundability
                | crate::model::Feature::Countability
                | crate::model::Feature::LocativeTemporalLicense
                | crate::model::Feature::MannerAnaphorClass
                | crate::model::Feature::ModifierLicense
                | crate::model::Feature::Properness
                | crate::model::Feature::Relationality
        ) {
            combine(
                errors,
                syn::Error::new(
                    default.value.span(),
                    "closed lexeme metadata supports only BareLocativeLicense, Compoundability, Countability, LocativeTemporalLicense, MannerAnaphorClass, ModifierLicense, Properness, and Relationality",
                ),
            );
        }
        if !feature_defaults.insert(default.feature) {
            combine(
                errors,
                syn::Error::new(default.value.span(), "duplicate lexeme feature default"),
            );
        }
        if crate::feature::Feature::from(default.feature)
            .member(&default.value)
            .is_err()
        {
            combine(
                errors,
                syn::Error::new(default.value.span(), "invalid lexeme feature default"),
            );
        }
    }
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
        let mut feature_overrides = HashSet::new();
        for override_ in &member.feature_overrides {
            if !matches!(
                override_.feature,
                crate::model::Feature::BareLocativeLicense
                    | crate::model::Feature::Compoundability
                    | crate::model::Feature::Countability
                    | crate::model::Feature::LocativeTemporalLicense
                    | crate::model::Feature::MannerAnaphorClass
                    | crate::model::Feature::ModifierLicense
                    | crate::model::Feature::Properness
                    | crate::model::Feature::Relationality
            ) {
                combine(
                    errors,
                    syn::Error::new(
                        override_.value.span(),
                        "closed lexeme metadata supports only BareLocativeLicense, Compoundability, Countability, LocativeTemporalLicense, MannerAnaphorClass, ModifierLicense, Properness, and Relationality",
                    ),
                );
            }
            if !feature_overrides.insert(override_.feature) {
                combine(
                    errors,
                    syn::Error::new(override_.value.span(), "duplicate lexeme feature override"),
                );
            }
            if crate::feature::Feature::from(override_.feature)
                .member(&override_.value)
                .is_err()
            {
                combine(
                    errors,
                    syn::Error::new(override_.value.span(), "invalid lexeme feature override"),
                );
            }
        }
    }
    let explicit_features = lexeme
        .members
        .iter()
        .flat_map(|member| member.feature_overrides.iter().map(|row| row.feature))
        .collect::<HashSet<_>>();
    for feature in explicit_features.difference(&feature_defaults) {
        for member in &lexeme.members {
            if !member
                .feature_overrides
                .iter()
                .any(|row| row.feature == *feature)
            {
                combine(
                    errors,
                    syn::Error::new(
                        member.name.span(),
                        format!(
                            "lexeme feature `{}` without a default must be declared on every member",
                            crate::feature::Feature::from(*feature).key()
                        ),
                    ),
                );
            }
        }
    }
}

fn validate_catalog_identity(
    source: &crate::model::CatalogIdentitySource,
    errors: &mut Option<syn::Error>,
) {
    match source.provider_slots.as_slice() {
        [] => combine(
            errors,
            syn::Error::new(
                source.recipe.span(),
                "catalog_identity requires one `provider` field",
            ),
        ),
        [_] => {}
        [_, rest @ ..] => {
            for duplicate in rest {
                combine(
                    errors,
                    syn::Error::new(
                        duplicate.slot.span(),
                        "duplicate catalog_identity field `provider`",
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
            crate::model::GeneratedIdentityRecipe::Catalog(source) => {
                validate_catalog_identity(source, &mut errors);
            }
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

#[expect(
    clippy::too_many_lines,
    reason = "closed generated-codec recipes share one exhaustive validation dispatch"
)]
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
                        if !matches!(slot, crate::model::UnsignedPrimitiveSource::U32 { .. }) {
                            combine(
                                &mut errors,
                                syn::Error::new(
                                    slot.primitive().span(),
                                    "signed_decimal magnitude must be `u32`",
                                ),
                            );
                        }
                        for duplicate in rest {
                            combine(
                                &mut errors,
                                syn::Error::new(
                                    duplicate.slot().span(),
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
            crate::model::GeneratedCodecRecipe::DeclarationDeterminative(source) => {
                validate_declaration_determinative_source(source, &mut errors);
            }
            crate::model::GeneratedCodecRecipe::DeclarationTerm(source) => {
                validate_declaration_term_source(source, &mut errors);
            }
            crate::model::GeneratedCodecRecipe::DeclarationVerb(source) => {
                validate_declaration_verb_source(raw, source, &mut errors);
            }
            crate::model::GeneratedCodecRecipe::EnglishCardinal(source)
            | crate::model::GeneratedCodecRecipe::UnsignedDecimal(source) => {
                validate_unsigned_number_source(source, &mut errors);
            }
        }
    }
    finish_generated_codec_validation(raw, errors)
}

fn validate_declaration_determinative_source(
    source: &crate::model::DeclarationDeterminativeSource,
    errors: &mut Option<syn::Error>,
) {
    if source.closed_slots.len() > 1 {
        for duplicate in &source.closed_slots[1..] {
            combine(
                errors,
                syn::Error::new(
                    duplicate.slot.span(),
                    "duplicate declaration_determinative field `closed`",
                ),
            );
        }
    }
    let has_closed = source
        .closed_slots
        .first()
        .is_some_and(|slot| !slot.members.is_empty());
    if !has_closed {
        combine(
            errors,
            syn::Error::new(
                source.recipe.span(),
                "declaration_determinative requires a nonempty `closed` provider",
            ),
        );
    }
    if let Some(closed) = source.closed_slots.first() {
        if closed.members.is_empty() {
            combine(
                errors,
                syn::Error::new(
                    closed.slot.span(),
                    "declaration_determinative closed provider cannot be empty",
                ),
            );
        }
        let mut lemmas = HashSet::new();
        for member in &closed.members {
            let lemma = identifier_key(&member.lemma);
            if !lemmas.insert(lemma.clone()) {
                combine(
                    errors,
                    syn::Error::new(
                        member.lemma.span(),
                        format!("duplicate declaration_determinative lemma `{lemma}`"),
                    ),
                );
            }
            validate_determinative_member(member, errors);
        }
    }
}

fn validate_determinative_member(
    member: &crate::model::DeclarationDeterminativeMemberSource,
    errors: &mut Option<syn::Error>,
) {
    validate_determinative_member_slot(
        &member.number_license_slots,
        &member.lemma,
        "number_license",
        &["SingularOnly", "PluralOnly", "Both"],
        errors,
    );
    validate_determinative_member_slot(
        &member.fused_head_license_slots,
        &member.lemma,
        "fused_head_license",
        &[
            "NominalOnly",
            "PartitiveOnly",
            "FusedHead",
            "PluralPredeterminer",
        ],
        errors,
    );
    validate_determinative_member_slot(
        &member.nominal_license_slots,
        &member.lemma,
        "nominal_license",
        &[
            "AnyNominal",
            "CountNominal",
            "BareSingularNoun",
            "MassOrPluralCount",
        ],
        errors,
    );
    validate_determinative_member_slot(
        &member.bare_duration_license_slots,
        &member.lemma,
        "bare_duration_license",
        &["BareDurationLicensed", "MarkerRequired"],
        errors,
    );
    match member.realization_slots.as_slice() {
        [] => combine(
            errors,
            syn::Error::new(
                member.lemma.span(),
                "declaration_determinative member requires one `realizations` field",
            ),
        ),
        [slot, rest @ ..] => {
            for duplicate in rest {
                combine(
                    errors,
                    syn::Error::new(
                        duplicate.slot.span(),
                        "duplicate declaration_determinative member field `realizations`",
                    ),
                );
            }
            if slot.realizations.is_empty() {
                combine(
                    errors,
                    syn::Error::new(
                        slot.slot.span(),
                        "declaration_determinative realizations cannot be empty",
                    ),
                );
            }
            let mut conditions = Vec::<(Option<String>, Option<String>, proc_macro2::Span)>::new();
            for row in &slot.realizations {
                if row.surface_slots.len() != 1 {
                    combine(
                        errors,
                        syn::Error::new(
                            member.lemma.span(),
                            "declaration_determinative realization requires one `surface` field",
                        ),
                    );
                }
                validate_optional_determinative_condition(
                    &row.phrase_number_slots,
                    "phrase_number",
                    &["Singular", "Plural"],
                    errors,
                );
                validate_optional_determinative_condition(
                    &row.following_onset_slots,
                    "following_onset",
                    &["Consonant", "Vowel"],
                    errors,
                );
                if row.surface_slots.len() == 1
                    && row.phrase_number_slots.len() <= 1
                    && row.following_onset_slots.len() <= 1
                {
                    if crate::macro_def::normalize_surface_onset(
                        &row.surface_slots[0].value(),
                        None,
                    )
                    .is_none()
                    {
                        combine(
                            errors,
                            syn::Error::new(
                                row.surface_slots[0].span(),
                                "declaration_determinative realization has no discernible onset",
                            ),
                        );
                    }
                    let phrase = row
                        .phrase_number_slots
                        .first()
                        .map(|slot| identifier_key(&slot.value));
                    let onset = row
                        .following_onset_slots
                        .first()
                        .map(|slot| identifier_key(&slot.value));
                    let number = member
                        .number_license_slots
                        .first()
                        .map(|slot| identifier_key(&slot.value));
                    if matches!(number.as_deref(), Some("SingularOnly"))
                        && matches!(phrase.as_deref(), Some("Plural"))
                        || matches!(number.as_deref(), Some("PluralOnly"))
                            && matches!(phrase.as_deref(), Some("Singular"))
                    {
                        combine(
                            errors,
                            syn::Error::new(
                                row.surface_slots[0].span(),
                                "declaration_determinative realization conflicts with its number_license",
                            ),
                        );
                    }
                    if conditions.iter().any(|(other_phrase, other_onset, _)| {
                        (phrase.is_none() || other_phrase.is_none() || phrase == *other_phrase)
                            && (onset.is_none() || other_onset.is_none() || onset == *other_onset)
                    }) {
                        combine(
                            errors,
                            syn::Error::new(
                                row.surface_slots[0].span(),
                                "overlapping declaration_determinative realizations",
                            ),
                        );
                    }
                    conditions.push((phrase, onset, row.surface_slots[0].span()));
                }
            }
        }
    }
}

fn validate_determinative_member_slot(
    slots: &[crate::model::GeneratedIdentSlot],
    lemma: &syn::Ident,
    name: &str,
    allowed: &[&str],
    errors: &mut Option<syn::Error>,
) {
    match slots {
        [] => combine(
            errors,
            syn::Error::new(
                lemma.span(),
                format!("declaration_determinative member requires one `{name}` field"),
            ),
        ),
        [slot, rest @ ..] => {
            for duplicate in rest {
                combine(
                    errors,
                    syn::Error::new(
                        duplicate.slot.span(),
                        format!("duplicate declaration_determinative member field `{name}`"),
                    ),
                );
            }
            let value = identifier_key(&slot.value);
            if !allowed.contains(&value.as_str()) {
                combine(
                    errors,
                    syn::Error::new(
                        slot.value.span(),
                        format!("invalid declaration_determinative {name} `{value}`"),
                    ),
                );
            }
        }
    }
}

fn validate_optional_determinative_condition(
    slots: &[crate::model::GeneratedIdentSlot],
    name: &str,
    allowed: &[&str],
    errors: &mut Option<syn::Error>,
) {
    if let Some((slot, rest)) = slots.split_first() {
        for duplicate in rest {
            combine(
                errors,
                syn::Error::new(
                    duplicate.slot.span(),
                    format!("duplicate declaration_determinative realization field `{name}`"),
                ),
            );
        }
        let value = identifier_key(&slot.value);
        if !allowed.contains(&value.as_str()) {
            combine(
                errors,
                syn::Error::new(
                    slot.value.span(),
                    format!("invalid declaration_determinative {name} `{value}`"),
                ),
            );
        }
    }
}

fn validate_unsigned_number_source(
    source: &crate::model::UnsignedNumberSource,
    errors: &mut Option<syn::Error>,
) {
    let recipe = &source.recipe;
    match source.magnitude_slots.as_slice() {
        [] => combine(
            errors,
            syn::Error::new(
                recipe.span(),
                format!("{recipe} requires one `magnitude` field"),
            ),
        ),
        [slot, rest @ ..] => {
            let supported = match recipe.to_string().as_str() {
                "english_cardinal" => {
                    matches!(slot, crate::model::UnsignedPrimitiveSource::U32 { .. })
                }
                "unsigned_decimal" => matches!(
                    slot,
                    crate::model::UnsignedPrimitiveSource::U32 { .. }
                        | crate::model::UnsignedPrimitiveSource::NonZeroU32 { .. }
                ),
                _ => unreachable!("validated unsigned-number source has a closed recipe"),
            };
            if !supported {
                let expected =
                    if recipe == "unsigned_decimal" { "`u32` or `NonZeroU32`" } else { "`u32`" };
                combine(
                    errors,
                    syn::Error::new(
                        slot.primitive().span(),
                        format!("{recipe} magnitude must be {expected}"),
                    ),
                );
            }
            for duplicate in rest {
                combine(
                    errors,
                    syn::Error::new(
                        duplicate.slot().span(),
                        format!("duplicate {recipe} field `magnitude`"),
                    ),
                );
            }
        }
    }
}

fn finish_generated_codec_validation(
    raw: &Declarations,
    mut errors: Option<syn::Error>,
) -> syn::Result<()> {
    validate_declaration_noun_domains_are_pairwise_intentional(raw, &mut errors);
    validate_declaration_term_domains_are_pairwise_intentional(raw, &mut errors);
    validate_declaration_verb_domains_are_pairwise_intentional(raw, &mut errors);
    finish(errors)
}

fn validate_declaration_term_domains_are_pairwise_intentional(
    raw: &Declarations,
    errors: &mut Option<syn::Error>,
) {
    let terms = raw
        .declarations
        .iter()
        .filter_map(|declaration| {
            let Declaration::Codec(binding) = declaration else {
                return None;
            };
            let Some(crate::model::GeneratedCodecRecipe::DeclarationTerm(source)) =
                &binding.generated
            else {
                return None;
            };
            let position = source.position_slots.first()?;
            let kinds = source.kind_slots.first()?;
            let params = source.param_policy_slots.is_empty().then(|| {
                source
                    .param_slots
                    .first()
                    .map(|slot| slot.kinds.iter().map(identifier_key).collect::<Vec<_>>())
                    .unwrap_or_default()
            });
            let feature = source
                .feature_slots
                .first()
                .map_or_else(|| "Fixed".to_owned(), |slot| identifier_key(&slot.value));
            Some((
                binding,
                identifier_key(&position.value),
                kinds
                    .kinds
                    .iter()
                    .map(identifier_key)
                    .collect::<BTreeSet<_>>(),
                params,
                feature,
            ))
        })
        .collect::<Vec<_>>();
    for (index, (left, left_position, left_kinds, left_params, left_feature)) in
        terms.iter().enumerate()
    {
        for (right, right_position, right_kinds, right_params, right_feature) in &terms[index + 1..]
        {
            if left_position != right_position || left_feature != right_feature {
                continue;
            }
            if left_params.is_none() != right_params.is_none() {
                continue;
            }
            let params_overlap =
                left_params.is_none() || right_params.is_none() || left_params == right_params;
            if !params_overlap {
                continue;
            }
            if let Some(kind) = left_kinds.intersection(right_kinds).next() {
                combine(
                    errors,
                    syn::Error::new(
                        right.name.span(),
                        format!(
                            "declaration_term domains `{}` and `{}` overlap at `{kind}/{left_position}/{left_feature}/{left_params:?}`",
                            left.name, right.name
                        ),
                    ),
                );
            }
        }
    }
}

#[expect(
    clippy::too_many_lines,
    reason = "one validator keeps the sealed declaration-term recipe contract together"
)]
fn validate_declaration_term_source(
    source: &crate::model::DeclarationTermSource,
    errors: &mut Option<syn::Error>,
) {
    let position = validate_single_ident_slot(
        &source.position_slots,
        &source.recipe,
        "position",
        "declaration_term",
        errors,
    );
    let kinds = match source.kind_slots.as_slice() {
        [] => {
            combine(
                errors,
                syn::Error::new(
                    source.recipe.span(),
                    "declaration_term requires one `kinds` field",
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
                        "duplicate declaration_term field `kinds`",
                    ),
                );
            }
            Some(slot)
        }
    };
    validate_declaration_term_params(source, errors);
    if let [_, rest @ ..] = source.feature_slots.as_slice() {
        for duplicate in rest {
            combine(
                errors,
                syn::Error::new(
                    duplicate.slot.span(),
                    "duplicate declaration_term field `feature`",
                ),
            );
        }
    }

    if let Some(position) = position
        && !matches!(
            identifier_key(position).as_str(),
            "FixedTerm" | "FixedKeyword"
        )
    {
        combine(
            errors,
            syn::Error::new(
                position.span(),
                "declaration_term position must be `FixedTerm` or `FixedKeyword`",
            ),
        );
    }

    if let Some(feature) = source.feature_slots.first()
        && !matches!(
            identifier_key(&feature.value).as_str(),
            "Fixed" | "BoundSuffix" | "Participle" | "BlockLabel"
        )
    {
        combine(
            errors,
            syn::Error::new(
                feature.value.span(),
                "declaration_term feature must be `Fixed`, `BoundSuffix`, `Participle`, or `BlockLabel`",
            ),
        );
    }
    if position.is_some_and(|position| identifier_key(position) == "FixedTerm")
        && source
            .feature_slots
            .first()
            .is_some_and(|feature| identifier_key(&feature.value) != "Fixed")
    {
        combine(
            errors,
            syn::Error::new(
                source.feature_slots[0].value.span(),
                "FixedTerm declaration_term codecs support only the `Fixed` feature",
            ),
        );
    }
    if source
        .feature_slots
        .first()
        .is_some_and(|feature| identifier_key(&feature.value) == "BlockLabel")
        && position.is_none_or(|position| identifier_key(position) != "FixedKeyword")
    {
        combine(
            errors,
            syn::Error::new(
                source.feature_slots[0].value.span(),
                "BlockLabel declaration_term codecs require the `FixedKeyword` position",
            ),
        );
    }

    let Some(kinds) = kinds else { return };
    if kinds.kinds.is_empty() {
        combine(
            errors,
            syn::Error::new(
                kinds.slot.span(),
                "declaration_term kind set cannot be empty",
            ),
        );
    }
    let mut seen = HashSet::new();
    for kind in &kinds.kinds {
        let name = identifier_key(kind);
        if !matches!(
            name.as_str(),
            "KeywordAbility" | "AbilityWord" | "FlavorWord" | "CounterKind" | "Designation"
        ) {
            combine(
                errors,
                syn::Error::new(
                    kind.span(),
                    "declaration_term kinds must be `KeywordAbility`, `AbilityWord`, `FlavorWord`, `CounterKind`, or `Designation`",
                ),
            );
            continue;
        }
        if !seen.insert(name.clone()) {
            combine(
                errors,
                syn::Error::new(
                    kind.span(),
                    format!("duplicate declaration_term kind `{name}`"),
                ),
            );
        }
        if let Some(position) = position {
            let position = identifier_key(position);
            let compatible = matches!(
                (position.as_str(), name.as_str()),
                ("FixedKeyword", "KeywordAbility")
                    | (
                        "FixedTerm",
                        "AbilityWord" | "FlavorWord" | "CounterKind" | "Designation"
                    )
            );
            if !compatible {
                combine(
                    errors,
                    syn::Error::new(
                        kind.span(),
                        format!(
                            "declaration kind `{name}` is not compatible with declaration_term position `{position}`"
                        ),
                    ),
                );
            }
        }
    }
}

fn validate_declaration_term_params(
    source: &crate::model::DeclarationTermSource,
    errors: &mut Option<syn::Error>,
) {
    for slot in &source.param_slots {
        for parameter in &slot.kinds {
            let name = identifier_key(parameter);
            if let Err(reason) = crate::macro_def::ParameterType::new(name) {
                combine(errors, syn::Error::new(parameter.span(), reason));
            }
        }
    }
    let parameter_fields = source
        .param_slots
        .iter()
        .map(|slot| slot.slot.span())
        .chain(
            source
                .param_policy_slots
                .iter()
                .map(|slot| slot.slot.span()),
        )
        .collect::<Vec<_>>();
    if let [_, rest @ ..] = parameter_fields.as_slice() {
        for duplicate in rest {
            combine(
                errors,
                syn::Error::new(*duplicate, "duplicate declaration_term field `params`"),
            );
        }
    }
    if let Some(policy) = source.param_policy_slots.first()
        && identifier_key(&policy.value) != "Any"
    {
        combine(
            errors,
            syn::Error::new(
                policy.value.span(),
                "declaration_term params policy must be `Any`",
            ),
        );
    }
}

fn validate_declaration_verb_domains_are_pairwise_intentional(
    raw: &Declarations,
    errors: &mut Option<syn::Error>,
) {
    let verbs = raw
        .declarations
        .iter()
        .filter_map(|declaration| {
            let Declaration::Codec(binding) = declaration else {
                return None;
            };
            let Some(crate::model::GeneratedCodecRecipe::DeclarationVerb(source)) =
                &binding.generated
            else {
                return None;
            };
            declaration_verb_domain(source).map(|domain| (binding, domain))
        })
        .collect::<Vec<_>>();
    for (index, (left, left_domain)) in verbs.iter().enumerate() {
        for (right, right_domain) in &verbs[index + 1..] {
            if left_domain.position != right_domain.position
                || left_domain.feature != right_domain.feature
                || left_domain.class != right_domain.class
                || left_domain.tail != right_domain.tail
            {
                continue;
            }
            let closed_overlap = left_domain
                .closed
                .as_ref()
                .is_some_and(|left_closed| right_domain.closed.as_ref() == Some(left_closed));
            if left_domain.closed.is_none() && right_domain.closed.is_none() {
                combine(
                    errors,
                    syn::Error::new(
                        right.name.span(),
                        format!(
                            "declaration_verb domains `{}` and `{}` overlap at `{}/{}/{}`",
                            left.name,
                            right.name,
                            left_domain.position,
                            left_domain.class,
                            left_domain.tail
                        ),
                    ),
                );
            } else if closed_overlap {
                let closed = left_domain
                    .closed
                    .as_deref()
                    .expect("closed overlap has a lexeme");
                combine(
                    errors,
                    syn::Error::new(
                        right.name.span(),
                        format!(
                            "declaration_verb domains `{}` and `{}` overlap at closed `{closed}`/{}/{}/{}",
                            left.name,
                            right.name,
                            left_domain.position,
                            left_domain.class,
                            left_domain.tail
                        ),
                    ),
                );
            }
        }
    }
}

struct DeclarationVerbDomain {
    closed: Option<String>,
    position: String,
    class: String,
    feature: String,
    tail: String,
}

fn declaration_verb_domain(
    source: &crate::model::DeclarationVerbSource,
) -> Option<DeclarationVerbDomain> {
    let position = source.position_slots.first()?;
    let feature = source.feature_slots.first()?;
    let tail = source.tail_slots.first()?;
    let tail = tail
        .atoms
        .iter()
        .map(|atom| {
            let key = match &atom.kind {
                crate::model::DeclarationVerbTailAtomKindSource::Literal(literal) => {
                    format!("Literal({:?})", literal.value())
                }
                crate::model::DeclarationVerbTailAtomKindSource::Lex(path) => {
                    format!("Lex({})", quote::quote!(#path))
                }
                crate::model::DeclarationVerbTailAtomKindSource::Marked { marker, role } => {
                    format!(
                        "Marked({}, {})",
                        quote::quote!(#marker),
                        identifier_key(role)
                    )
                }
                crate::model::DeclarationVerbTailAtomKindSource::Amount(_) => "Amount".to_owned(),
                crate::model::DeclarationVerbTailAtomKindSource::ObjectNounPhrase(_) => {
                    "ObjectNounPhrase".to_owned()
                }
                crate::model::DeclarationVerbTailAtomKindSource::PredicativeComplement(_) => {
                    "PredicativeComplement".to_owned()
                }
                crate::model::DeclarationVerbTailAtomKindSource::FrameComplementPair(_) => {
                    "FrameComplementPair".to_owned()
                }
                crate::model::DeclarationVerbTailAtomKindSource::Role(role) => {
                    format!("Role({})", identifier_key(role))
                }
            };
            if atom.optional { format!("{key}?") } else { key }
        })
        .collect::<Vec<_>>()
        .join(", ");
    Some(DeclarationVerbDomain {
        closed: source
            .closed_slots
            .first()
            .map(|slot| identifier_key(&slot.value)),
        position: identifier_key(&position.value),
        class: source.class_slots.first().map_or_else(
            || "Predicate".to_owned(),
            |slot| identifier_key(&slot.value),
        ),
        feature: identifier_key(&feature.value),
        tail: format!("[{tail}]"),
    })
}

fn validate_declaration_noun_domains_are_pairwise_intentional(
    raw: &Declarations,
    errors: &mut Option<syn::Error>,
) {
    let nouns = raw
        .declarations
        .iter()
        .filter_map(|declaration| {
            let Declaration::Codec(binding) = declaration else { return None };
            let Some(crate::model::GeneratedCodecRecipe::DeclarationNoun(_)) = &binding.generated
            else {
                return None;
            };
            Some(binding)
        })
        .collect::<Vec<_>>();
    let Some(first) = nouns.first() else { return };
    for duplicate in nouns.iter().skip(1) {
        combine(
            errors,
            syn::Error::new(
                duplicate.name.span(),
                format!(
                    "multiple declaration_noun inventories `{}` and `{}`; combine declaration kinds in one codec",
                    first.name, duplicate.name
                ),
            ),
        );
    }
}

fn declaration_noun_domain_members(
    source: &crate::model::DeclarationNounSource,
) -> BTreeSet<String> {
    let Some(kinds) = source.kind_slots.first() else {
        return BTreeSet::new();
    };
    let mut members = BTreeSet::new();
    for kind in &kinds.kinds {
        match (
            identifier_key(&kind.kind).as_str(),
            kind.subtype_family.as_ref().map(identifier_key),
        ) {
            ("Type", None) => {
                members.insert("Type".to_owned());
            }
            ("TurnPart", None) => {
                members.insert("TurnPart".to_owned());
            }
            ("Subtype", None) => {
                for family in declaration_subtype_families() {
                    members.insert(format!("Subtype({family})"));
                }
            }
            ("Subtype", Some(family))
                if declaration_subtype_families().contains(&family.as_str()) =>
            {
                members.insert(format!("Subtype({family})"));
            }
            _ => {}
        }
    }
    members
}

fn declaration_subtype_families() -> &'static [&'static str] {
    &[
        "Artifact",
        "Battle",
        "Creature",
        "Enchantment",
        "Land",
        "Planeswalker",
        "Spell",
    ]
}

fn validate_declaration_verb_source(
    raw: &Declarations,
    source: &crate::model::DeclarationVerbSource,
    errors: &mut Option<syn::Error>,
) {
    let closed = match source.closed_slots.as_slice() {
        [] => None,
        [slot, rest @ ..] => {
            for duplicate in rest {
                combine(
                    errors,
                    syn::Error::new(
                        duplicate.slot.span(),
                        "duplicate declaration_verb field `closed`",
                    ),
                );
            }
            Some(&slot.value)
        }
    };
    let class = match source.class_slots.as_slice() {
        [] => None,
        [slot, rest @ ..] => {
            for duplicate in rest {
                combine(
                    errors,
                    syn::Error::new(
                        duplicate.slot.span(),
                        "duplicate declaration_verb field `class`",
                    ),
                );
            }
            Some(&slot.value)
        }
    };
    let position = validate_single_ident_slot(
        &source.position_slots,
        &source.recipe,
        "position",
        "declaration_verb",
        errors,
    );
    let feature = validate_single_ident_slot(
        &source.feature_slots,
        &source.recipe,
        "feature",
        "declaration_verb",
        errors,
    );
    validate_declaration_verb_tail(raw, source, errors);
    validate_frame_complement_pair_recipe(source, closed, class, feature, errors);

    if let Some(position) = position
        && position != "Verb"
    {
        combine(
            errors,
            syn::Error::new(position.span(), "declaration_verb position must be `Verb`"),
        );
    }
    if let Some(feature) = feature
        && !matches!(
            identifier_key(feature).as_str(),
            "ConcordClass" | "Participle"
        )
    {
        combine(
            errors,
            syn::Error::new(
                feature.span(),
                "declaration_verb feature must be `ConcordClass` or `Participle`",
            ),
        );
    }
    if let Some(class) = class
        && !matches!(
            identifier_key(class).as_str(),
            "Predicate" | "Auxiliary" | "ProVerb"
        )
    {
        combine(
            errors,
            syn::Error::new(
                class.span(),
                "declaration_verb class must be `Predicate`, `Auxiliary`, or `ProVerb`",
            ),
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
            Some(lexeme) if lexeme.members.is_empty() => combine(
                errors,
                syn::Error::new(
                    closed.span(),
                    "declaration_verb closed branch must have at least one lexeme member",
                ),
            ),
            Some(lexeme)
                if feature.is_some_and(|feature| {
                    let expected = match identifier_key(feature).as_str() {
                        "ConcordClass" => Some(crate::morphology::MorphologyRecipe::EnglishVerb),
                        "Participle" => {
                            Some(crate::morphology::MorphologyRecipe::EnglishParticiple)
                        }
                        _ => None,
                    };
                    lexeme_recipe(raw, lexeme) != expected
                }) =>
            {
                combine(
                    errors,
                    syn::Error::new(
                        closed.span(),
                        "declaration_verb closed branch morphology must match its feature axis",
                    ),
                );
            }
            Some(_) => {}
            None => combine(
                errors,
                syn::Error::new(
                    closed.span(),
                    "declaration_verb closed branch must name a lexeme declaration",
                ),
            ),
        }
    }
}

fn validate_frame_complement_pair_recipe(
    source: &crate::model::DeclarationVerbSource,
    closed: Option<&syn::Ident>,
    class: Option<&syn::Ident>,
    feature: Option<&syn::Ident>,
    errors: &mut Option<syn::Error>,
) {
    let is_pair = source.tail_slots.first().is_some_and(|tail| {
        matches!(
            tail.atoms.as_slice(),
            [crate::model::DeclarationVerbTailAtomSource {
                kind: crate::model::DeclarationVerbTailAtomKindSource::FrameComplementPair(_),
                ..
            }]
        )
    });
    if !is_pair {
        return;
    }
    for (actual, expected, message) in [
        (
            class,
            "Predicate",
            "FrameComplementPair declaration_verb patterns must have Predicate class",
        ),
        (
            feature,
            "ConcordClass",
            "FrameComplementPair declaration_verb patterns must have ConcordClass feature",
        ),
    ] {
        if let Some(actual) = actual
            && identifier_key(actual) != expected
        {
            combine(errors, syn::Error::new(actual.span(), message));
        }
    }
    if let Some(closed) = closed {
        combine(
            errors,
            syn::Error::new(
                closed.span(),
                "FrameComplementPair declaration_verb patterns cannot have a closed branch",
            ),
        );
    }
}

#[expect(
    clippy::too_many_lines,
    reason = "Verb Frame tail validation keeps the closed atom inventory together"
)]
fn validate_declaration_verb_tail(
    raw: &Declarations,
    source: &crate::model::DeclarationVerbSource,
    errors: &mut Option<syn::Error>,
) {
    let tail = match source.tail_slots.as_slice() {
        [] => {
            combine(
                errors,
                syn::Error::new(
                    source.recipe.span(),
                    "declaration_verb requires one `tail` field",
                ),
            );
            return;
        }
        [slot, rest @ ..] => {
            for duplicate in rest {
                combine(
                    errors,
                    syn::Error::new(
                        duplicate.slot.span(),
                        "duplicate declaration_verb field `tail`",
                    ),
                );
            }
            slot
        }
    };
    let mut seen_labels = HashSet::new();
    let mut nonliteral_occurrences =
        HashMap::<String, Vec<&crate::model::DeclarationVerbTailAtomSource>>::new();
    let mut seen_literals = HashSet::new();
    for atom in &tail.atoms {
        if let Some(label) = &atom.label {
            let key = identifier_key(label);
            if !seen_labels.insert(key.clone()) {
                combine(
                    errors,
                    syn::Error::new(
                        label.span(),
                        format!("duplicate declaration_verb tail label `{key}`"),
                    ),
                );
            }
        }
        match &atom.kind {
            crate::model::DeclarationVerbTailAtomKindSource::Literal(literal) => {
                if literal.value().is_empty() {
                    combine(
                        errors,
                        syn::Error::new(
                            literal.span(),
                            "declaration_verb tail literals cannot be empty",
                        ),
                    );
                }
                if atom.label.is_some() {
                    combine(
                        errors,
                        syn::Error::new(
                            literal.span(),
                            "declaration_verb tail labels cannot prefix literals",
                        ),
                    );
                }
                let key = format!("Literal({:?})", literal.value());
                if !seen_literals.insert(key.clone()) {
                    combine(
                        errors,
                        syn::Error::new(
                            literal.span(),
                            format!("duplicate declaration_verb tail atom `{key}`"),
                        ),
                    );
                }
            }
            crate::model::DeclarationVerbTailAtomKindSource::Lex(path) => {
                let segments = path.segments.iter().collect::<Vec<_>>();
                if segments.len() != 2 {
                    combine(
                        errors,
                        syn::Error::new_spanned(
                            path,
                            "declaration_verb tail vocabulary atom must be `Type::Variant`",
                        ),
                    );
                    continue;
                }
                let terminal = identifier_key(&segments[0].ident);
                let variant = identifier_key(&segments[1].ident);
                let found = raw.declarations.iter().any(|declaration| {
                    matches!(
                        declaration,
                        Declaration::Vocab(vocab)
                            if identifier_key(&vocab.name) == terminal
                                && vocab.variants.iter().any(|row| identifier_key(&row.name) == variant)
                    )
                });
                if !found {
                    combine(
                        errors,
                        syn::Error::new_spanned(
                            path,
                            format!("unknown vocabulary atom `{terminal}::{variant}`"),
                        ),
                    );
                }
                if atom.label.is_some() {
                    combine(
                        errors,
                        syn::Error::new_spanned(
                            path,
                            "declaration_verb tail labels cannot prefix vocabulary atoms",
                        ),
                    );
                }
                let key = format!("Lex({terminal}::{variant})");
                if !seen_literals.insert(key.clone()) {
                    combine(
                        errors,
                        syn::Error::new_spanned(
                            path,
                            format!("duplicate declaration_verb tail atom `{key}`"),
                        ),
                    );
                }
            }
            crate::model::DeclarationVerbTailAtomKindSource::Marked { marker, role } => {
                let segments = marker.segments.iter().collect::<Vec<_>>();
                if segments.len() != 2 {
                    combine(
                        errors,
                        syn::Error::new_spanned(
                            marker,
                            "declaration_verb marked-role atom must use a `Type::Variant` marker",
                        ),
                    );
                    continue;
                }
                let terminal = identifier_key(&segments[0].ident);
                let variant = identifier_key(&segments[1].ident);
                let marker_found = raw.declarations.iter().any(|declaration| {
                    matches!(
                        declaration,
                        Declaration::Vocab(vocab)
                            if identifier_key(&vocab.name) == terminal
                                && vocab.variants.iter().any(|row| identifier_key(&row.name) == variant)
                    )
                });
                if !marker_found {
                    combine(
                        errors,
                        syn::Error::new_spanned(
                            marker,
                            format!("unknown vocabulary atom `{terminal}::{variant}`"),
                        ),
                    );
                }
                let role_key = identifier_key(role);
                nonliteral_occurrences
                    .entry(format!("Marked({terminal}::{variant}, {role_key})"))
                    .or_default()
                    .push(atom);
            }
            crate::model::DeclarationVerbTailAtomKindSource::Amount(_) => {
                nonliteral_occurrences
                    .entry("Amount".to_owned())
                    .or_default()
                    .push(atom);
            }
            crate::model::DeclarationVerbTailAtomKindSource::ObjectNounPhrase(_) => {
                nonliteral_occurrences
                    .entry("ObjectNounPhrase".to_owned())
                    .or_default()
                    .push(atom);
            }
            crate::model::DeclarationVerbTailAtomKindSource::PredicativeComplement(_) => {
                nonliteral_occurrences
                    .entry("PredicativeComplement".to_owned())
                    .or_default()
                    .push(atom);
            }
            crate::model::DeclarationVerbTailAtomKindSource::FrameComplementPair(ident) => {
                nonliteral_occurrences
                    .entry("FrameComplementPair".to_owned())
                    .or_default()
                    .push(atom);
                if tail.atoms.len() != 1 || atom.optional || atom.label.is_some() {
                    combine(
                        errors,
                        syn::Error::new(
                            ident.span(),
                            "FrameComplementPair must be the sole, required, unlabeled declaration_verb tail pattern",
                        ),
                    );
                }
            }
            crate::model::DeclarationVerbTailAtomKindSource::Role(role) => {
                nonliteral_occurrences
                    .entry(format!("Role({})", identifier_key(role)))
                    .or_default()
                    .push(atom);
            }
        }
    }
    for (key, occurrences) in nonliteral_occurrences {
        if occurrences.len() <= 1 || occurrences.iter().all(|atom| atom.label.is_some()) {
            continue;
        }
        let span = match &occurrences[1].kind {
            crate::model::DeclarationVerbTailAtomKindSource::Amount(atom)
            | crate::model::DeclarationVerbTailAtomKindSource::ObjectNounPhrase(atom)
            | crate::model::DeclarationVerbTailAtomKindSource::PredicativeComplement(atom)
            | crate::model::DeclarationVerbTailAtomKindSource::FrameComplementPair(atom)
            | crate::model::DeclarationVerbTailAtomKindSource::Role(atom) => atom.span(),
            crate::model::DeclarationVerbTailAtomKindSource::Marked { marker, .. } => marker.span(),
            crate::model::DeclarationVerbTailAtomKindSource::Literal(_)
            | crate::model::DeclarationVerbTailAtomKindSource::Lex(_) => {
                unreachable!("only nonliteral occurrences are grouped")
            }
        };
        let message = if occurrences.iter().all(|atom| atom.label.is_none()) {
            format!("duplicate declaration_verb tail atom `{key}`")
        } else {
            format!("repeated declaration_verb tail atom `{key}` must label every occurrence")
        };
        combine(errors, syn::Error::new(span, message));
    }
}

fn validate_declaration_noun_source(
    raw: &Declarations,
    source: &crate::model::DeclarationNounSource,
    errors: &mut Option<syn::Error>,
) {
    let closed = match source.closed_slots.as_slice() {
        [] => None,
        [slot, rest @ ..] => {
            for duplicate in rest {
                combine(
                    errors,
                    syn::Error::new(
                        duplicate.slot.span(),
                        "duplicate declaration_noun field `closed`",
                    ),
                );
            }
            Some(&slot.value)
        }
    };
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
        validate_declaration_noun_kinds(source, kinds, errors);
    }
}

fn validate_declaration_noun_kinds(
    source: &crate::model::DeclarationNounSource,
    kinds: &crate::model::DeclarationNounKindsSource,
    errors: &mut Option<syn::Error>,
) {
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
    for kind in &kinds.kinds {
        let name = identifier_key(&kind.kind);
        if !matches!(name.as_str(), "Type" | "TurnPart" | "Subtype") {
            combine(
                errors,
                syn::Error::new(
                    kind.kind.span(),
                    "declaration_noun kinds must be `Type`, `TurnPart`, or `Subtype`",
                ),
            );
            continue;
        }
        let family = kind.subtype_family.as_ref().map(identifier_key);
        if matches!(name.as_str(), "Type" | "TurnPart") && family.is_some() {
            combine(
                errors,
                syn::Error::new(
                    kind.kind.span(),
                    format!("declaration_noun `{name}` filter does not accept a subtype family"),
                ),
            );
            continue;
        }
        if let Some(family) = &family
            && !declaration_subtype_families().contains(&family.as_str())
        {
            combine(
                errors,
                syn::Error::new(
                    kind.subtype_family.as_ref().expect("family exists").span(),
                    format!("unknown declaration_noun subtype family `{family}`"),
                ),
            );
            continue;
        }
        let key = family.map_or_else(|| name.clone(), |family| format!("{name}({family})"));
        if !seen.insert(key.clone()) {
            combine(
                errors,
                syn::Error::new(
                    kind.kind.span(),
                    format!("duplicate declaration_noun kind `{key}`"),
                ),
            );
        }
    }
    let members = declaration_noun_domain_members(source);
    if !kinds.kinds.is_empty() && members.is_empty() {
        combine(
            errors,
            syn::Error::new(
                kinds.slot.span(),
                "declaration_noun domain is not resolvable",
            ),
        );
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

fn seal_category_feature_reads(
    raw: &Declarations,
    category_render: &HashMap<String, CategoryRenderCapability>,
) -> HashMap<String, HashSet<Feature>> {
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
    let mut reads = categories
        .iter()
        .filter_map(|category| {
            let reads = [
                Feature::ConcordClass,
                Feature::Cardinality,
                Feature::ModifierLicense,
                Feature::DeterminerNumber,
                Feature::FusedHeadLicense,
                Feature::PrepositionComplementKind,
                Feature::LocativeTemporalLicense,
                Feature::NominalForm,
                Feature::NominalLicense,
                Feature::Number,
                Feature::Onset,
                Feature::PossessiveEnding,
            ]
            .into_iter()
            .filter(|feature| {
                (raw_category_reads_feature(raw, category, *feature)
                    && !(*feature == Feature::ConcordClass
                        && category_render.get(category).is_some_and(|capability| {
                            capability.requires_external_concord_class()
                        })))
                    || raw_sequence_reads_inherent_category_feature(
                        raw,
                        category,
                        *feature,
                        category_render,
                    )
            })
            .collect::<HashSet<_>>();
            (!reads.is_empty()).then_some((category.clone(), reads))
        })
        .collect::<HashMap<_, _>>();
    let providers = feature_providers(raw);
    for declaration in &raw.declarations {
        let Declaration::AbstractSum(sum) = declaration else {
            continue;
        };
        if !providers.contains(&(identifier_key(&sum.name), ParsedFeature::ConcordClass)) {
            continue;
        }
        for alternative in &sum.alternatives {
            let category = path_name(&alternative.value_type);
            if categories.contains(&category)
                && !category_render
                    .get(&category)
                    .is_some_and(|capability| capability.requires_external_concord_class())
            {
                reads
                    .entry(category)
                    .or_default()
                    .insert(Feature::ConcordClass);
            }
        }
    }
    reads
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
                for form in &construction.forms {
                    for atom in &form.atoms {
                        if let FormAtom::Verb(VerbOperand::Fixed(path)) = atom {
                            validate_generated_owned_path(path, &mut errors);
                        }
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
        FieldKind::Zeroable {
            value_type, item, ..
        } => {
            validate_generated_owned_path(value_type, errors);
            validate_generated_owned_field_kind(item, errors);
        }
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
    let noun_morphologies = raw
        .declarations
        .iter()
        .filter_map(|declaration| match declaration {
            Declaration::Morphology(morphology)
                if identifier_key(&morphology.recipe) == "english_noun" =>
            {
                Some(identifier_key(&morphology.name))
            }
            _ => None,
        })
        .collect::<HashSet<_>>();
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
                for form in &construction.forms {
                    reject_raw_keyword_identifier(
                        &form.name,
                        "generated form/rule fragment",
                        &mut errors,
                    );
                }
                categories.insert(category.clone());
                duplicate_name(&mut source_names, &name, &construction.name, &mut errors);
                let category_variant = pascal_case(&name);
                validate_generated_rust_ident(
                    &category_variant,
                    &format!("category variant for construction `{name}`"),
                    construction.name.span(),
                    &mut errors,
                );
                for form in &construction.forms {
                    validate_generated_rust_ident(
                        &pascal_case(&identifier_key(&form.name)),
                        &format!("form/rule fragment for construction `{name}`"),
                        form.name.span(),
                        &mut errors,
                    );
                }
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
                    let spelling = &variant.word;
                    let word = spelling.value();
                    if word.is_empty() {
                        combine(
                            &mut errors,
                            syn::Error::new(spelling.span(), "vocab spelling must not be empty"),
                        );
                    }
                    if !words.insert(word.clone()) {
                        combine(
                            &mut errors,
                            syn::Error::new(spelling.span(), format!("duplicate word `{word}`")),
                        );
                    }
                }
                terminals.entry(name).or_insert(TerminalInfo {
                    kind: TerminalKind::Vocab,
                    codec_atom: None,
                    declaration_verb: false,
                    concord_class_verb: false,
                    participle_verb: false,
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
                    codec_atom: noun_morphologies
                        .contains(&identifier_key(&lexeme.morphology))
                        .then_some(CodecAtomClass::Noun),
                    declaration_verb: false,
                    concord_class_verb: false,
                    participle_verb: false,
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
                    declaration_verb: matches!(
                        &binding.generated,
                        Some(crate::model::GeneratedCodecRecipe::DeclarationVerb(_))
                    ),
                    concord_class_verb: matches!(
                        &binding.generated,
                        Some(crate::model::GeneratedCodecRecipe::DeclarationVerb(recipe))
                            if recipe.feature_slots.first().is_some_and(|slot| slot.value == "ConcordClass")
                    ),
                    participle_verb: matches!(
                        &binding.generated,
                        Some(crate::model::GeneratedCodecRecipe::DeclarationVerb(recipe))
                            if recipe.feature_slots.first().is_some_and(|slot| slot.value == "Participle")
                    ),
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

    validate_explicit_sum_owned_categories(raw, &mut errors);
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
    let semantic_name = identifier_key(authored);
    if semantic_name.starts_with(PRIVATE_ROOT_RENDERER_PREFIX) {
        combine(
            errors,
            syn::Error::new(
                authored.span(),
                format!(
                    "identifier `{authored}` enters reserved compiler-internal namespace `{PRIVATE_ROOT_RENDERER_PREFIX}` for {generated_role}"
                ),
            ),
        );
    }
    if is_raw_keyword(authored) {
        combine(
            errors,
            syn::Error::new(
                authored.span(),
                format!(
                    "raw keyword `{authored}` has semantic identity `{semantic_name}` and is unsupported for {generated_role}"
                ),
            ),
        );
    }
}

fn explicit_sum_owned_category_names(raw: &Declarations) -> HashSet<String> {
    let construction_categories = raw
        .declarations
        .iter()
        .filter_map(|declaration| match declaration {
            Declaration::Construction(construction) => Some(path_name(&construction.category)),
            _ => None,
        })
        .collect::<HashSet<_>>();
    raw.declarations
        .iter()
        .filter_map(|declaration| match declaration {
            Declaration::AbstractSum(sum) => {
                let name = identifier_key(&sum.name);
                construction_categories.contains(&name).then_some(name)
            }
            _ => None,
        })
        .collect()
}

fn validate_explicit_sum_owned_categories(raw: &Declarations, errors: &mut Option<syn::Error>) {
    let owned = explicit_sum_owned_category_names(raw);
    for category in owned {
        let members = raw
            .declarations
            .iter()
            .filter_map(|declaration| match declaration {
                Declaration::Construction(construction)
                    if path_name(&construction.category) == category =>
                {
                    Some((
                        identifier_key(&construction.element.name),
                        construction.element.name.span(),
                    ))
                }
                _ => None,
            })
            .collect::<Vec<_>>();
        let sum = raw
            .declarations
            .iter()
            .find_map(|declaration| match declaration {
                Declaration::AbstractSum(sum) if identifier_key(&sum.name) == category => Some(sum),
                _ => None,
            })
            .expect("owned category name comes from an abstract sum");
        let member_names = members
            .iter()
            .map(|(name, _)| name.as_str())
            .collect::<HashSet<_>>();
        let mut mapped = HashSet::new();
        for alternative in &sum.alternatives {
            let value = path_name(&alternative.value_type);
            if !member_names.contains(value.as_str()) {
                combine(
                    errors,
                    syn::Error::new_spanned(
                        &alternative.value_type,
                        format!(
                            "explicit sum `{category}` maps alternative `{}` to foreign construction element `{value}`",
                            alternative.name
                        ),
                    ),
                );
            } else if !mapped.insert(value.clone()) {
                combine(
                    errors,
                    syn::Error::new_spanned(
                        &alternative.value_type,
                        format!(
                            "explicit sum `{category}` maps construction element `{value}` more than once"
                        ),
                    ),
                );
            }
        }
        for (member, span) in members {
            if !mapped.contains(&member) {
                combine(
                    errors,
                    syn::Error::new(
                        span,
                        format!(
                            "explicit sum `{category}` is missing construction element `{member}`"
                        ),
                    ),
                );
            }
        }
    }
}

#[derive(Default)]
struct GeneratedNameInventory {
    type_names: HashMap<String, String>,
    value_names: HashMap<String, String>,
    visitor_items: HashMap<String, String>,
    terminal_variants: HashMap<String, String>,
    catalog_provider_variants: HashMap<String, String>,
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

    fn register_catalog_provider_variant(
        &mut self,
        provider: &syn::Ident,
        errors: &mut Option<syn::Error>,
    ) {
        let semantic_identity = identifier_key(provider);
        let authored_spelling = provider.to_string();
        let role = format!(
            "generated catalog provider variant `{semantic_identity}` authored as `{authored_spelling}`"
        );
        validate_generated_rust_ident(&semantic_identity, &role, provider.span(), errors);
        if self
            .catalog_provider_variants
            .get(&semantic_identity)
            .is_some_and(|previous| previous == &role)
        {
            return;
        }
        register_associated_name(
            &mut self.catalog_provider_variants,
            "CatalogProvider variant",
            &semantic_identity,
            &role,
            provider.span(),
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
        self.register_construction_variant(generated, role, span, errors);
        self.register_rule_id_variant(generated, role, span, errors);
    }

    fn register_construction_variant(
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
    }

    fn register_rule_id_variant(
        &mut self,
        generated: &str,
        role: &str,
        span: proc_macro2::Span,
        errors: &mut Option<syn::Error>,
    ) {
        validate_generated_rust_ident(generated, role, span, errors);
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
    let explicit_sum_owned = explicit_sum_owned_category_names(raw);
    let feature_providers = feature_providers(raw);
    let fixed_span = proc_macro2::Span::call_site();
    for (name, role) in [
        (VISITOR_TRAIT, "fixed generated visitor trait"),
        (
            RIGHTMOST_LEAF_TRAIT,
            "fixed generated rightmost-leaf traversal trait",
        ),
        (
            RIGHTMOST_LEAF_CATEGORY_TRAIT,
            "fixed generated rightmost-leaf category trait",
        ),
        (
            RIGHT_PERIPHERY_PREPOSITION_TRAIT,
            "fixed generated right-periphery role-preposition trait",
        ),
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
            RIGHTMOST_LEAF_IS_FUNCTION,
            "fixed generated rightmost-leaf predicate",
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
            | FieldKind::Zeroable { .. }
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
                if !explicit_sum_owned.contains(&category)
                    && seen_categories.insert(category.clone())
                {
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
                    for (feature, parsed_feature, spelling, display, reserve_provider) in [
                        (
                            Feature::ConcordClass,
                            ParsedFeature::ConcordClass,
                            "concord_class",
                            "ConcordClass",
                            false,
                        ),
                        (
                            Feature::Number,
                            ParsedFeature::Number,
                            "number",
                            "Number",
                            false,
                        ),
                        (
                            Feature::ModifierLicense,
                            ParsedFeature::ModifierLicense,
                            "modifier_license",
                            "ModifierLicense",
                            true,
                        ),
                        (
                            Feature::DeterminerNumber,
                            ParsedFeature::DeterminerNumber,
                            "determiner_number",
                            "DeterminerNumber",
                            true,
                        ),
                        (
                            Feature::FusedHeadLicense,
                            ParsedFeature::FusedHeadLicense,
                            "fused_head_license",
                            "FusedHeadLicense",
                            true,
                        ),
                        (
                            Feature::PrepositionComplementKind,
                            ParsedFeature::PrepositionComplementKind,
                            "preposition_complement_kind",
                            "PrepositionComplementKind",
                            true,
                        ),
                        (
                            Feature::LocativeTemporalLicense,
                            ParsedFeature::LocativeTemporalLicense,
                            "locative_temporal_license",
                            "LocativeTemporalLicense",
                            true,
                        ),
                        (
                            Feature::NominalForm,
                            ParsedFeature::NominalForm,
                            "nominal_form",
                            "NominalForm",
                            true,
                        ),
                        (
                            Feature::NominalLicense,
                            ParsedFeature::NominalLicense,
                            "nominal_license",
                            "NominalLicense",
                            true,
                        ),
                        (Feature::Onset, ParsedFeature::Onset, "onset", "Onset", true),
                        (
                            Feature::PrepositionAttachment,
                            ParsedFeature::PrepositionAttachment,
                            "preposition_attachment",
                            "PrepositionAttachment",
                            true,
                        ),
                        (
                            Feature::Relationality,
                            ParsedFeature::Relationality,
                            "relationality",
                            "Relationality",
                            true,
                        ),
                    ] {
                        if raw_category_reads_feature(raw, &category, feature)
                            || (reserve_provider
                                && feature_providers.contains(&(category.clone(), parsed_feature)))
                        {
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
                if explicit_sum_owned.contains(&category) {
                    names.register_construction_variant(
                        &format!("{}{category_variant}", pascal_case(&category)),
                        &format!("Construction variant for `{construction_name}`"),
                        construction.name.span(),
                        errors,
                    );
                } else {
                    names.register_category_variant(
                        &category,
                        &category_variant,
                        &format!(
                            "generated category variants for construction `{construction_name}`"
                        ),
                        construction.name.span(),
                        errors,
                    );
                }
                for form in &construction.forms {
                    let rule_span = if construction.forms.len() == 1 {
                        construction.name.span()
                    } else {
                        form.name.span()
                    };
                    let form_rule = if explicit_sum_owned.contains(&category) {
                        let base = format!("{}Construction", pascal_case(&element));
                        if construction.forms.len() == 1 {
                            base
                        } else {
                            format!("{base}{}", pascal_case(&identifier_key(&form.name)))
                        }
                    } else if construction.forms.len() == 1 {
                        format!("{}{category_variant}", pascal_case(&category))
                    } else {
                        format!(
                            "{}{category_variant}{}",
                            pascal_case(&category),
                            pascal_case(&identifier_key(&form.name))
                        )
                    };
                    let rule_role = if construction.forms.len() == 1 {
                        format!("RuleId for construction {construction_name}")
                    } else {
                        format!(
                            "RuleId for construction form {construction_name}.{}",
                            form.name
                        )
                    };
                    register_structural_owner_rule_names(
                        &mut names,
                        &form_rule,
                        &element,
                        &construction.element.fields,
                        &construction.requirements,
                        &rule_role,
                        rule_span,
                        explicit_sum_owned.contains(&category),
                        errors,
                    );
                }
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
                                | FieldKind::Zeroable { .. }
                                | FieldKind::Optional(_)
                                | FieldKind::Sequence { .. } => None,
                            })
                    }) else {
                        continue;
                    };
                    let (spelling, display) = match feature {
                        ParsedFeature::ConcordClass => ("concord_class", "ConcordClass"),
                        ParsedFeature::BareDurationLicense => {
                            ("bare_duration_license", "BareDurationLicense")
                        }
                        ParsedFeature::BareLocativeLicense => {
                            ("bare_locative_license", "BareLocativeLicense")
                        }
                        ParsedFeature::Cardinality => ("cardinality", "Cardinality"),
                        ParsedFeature::Compoundability => ("compoundability", "Compoundability"),
                        ParsedFeature::Countability => ("countability", "Countability"),
                        ParsedFeature::HomographLicense => {
                            ("homograph_license", "HomographLicense")
                        }
                        ParsedFeature::MannerAnaphorClass => {
                            ("manner_anaphor_class", "MannerAnaphorClass")
                        }
                        ParsedFeature::ModifierLicense => ("modifier_license", "ModifierLicense"),
                        ParsedFeature::DeterminerNumber => {
                            ("determiner_number", "DeterminerNumber")
                        }
                        ParsedFeature::FusedHeadLicense => {
                            ("fused_head_license", "FusedHeadLicense")
                        }
                        ParsedFeature::Focus => ("focus", "Focus"),
                        ParsedFeature::PrepositionComplementKind => {
                            ("preposition_complement_kind", "PrepositionComplementKind")
                        }
                        ParsedFeature::LocativeTemporalLicense => {
                            ("locative_temporal_license", "LocativeTemporalLicense")
                        }
                        ParsedFeature::NominalForm => ("nominal_form", "NominalForm"),
                        ParsedFeature::NominalLicense => ("nominal_license", "NominalLicense"),
                        ParsedFeature::Number => ("number", "Number"),
                        ParsedFeature::Onset => ("onset", "Onset"),
                        ParsedFeature::Participle => ("participle", "Participle"),
                        ParsedFeature::PossessiveEnding => {
                            ("possessive_ending", "PossessiveEnding")
                        }
                        ParsedFeature::Properness => ("properness", "Properness"),
                        ParsedFeature::Relationality => ("relationality", "Relationality"),
                        ParsedFeature::BareLocativeComplement => {
                            ("bare_locative_complement", "BareLocativeComplement")
                        }
                        ParsedFeature::PrepositionAttachment => {
                            ("preposition_attachment", "PrepositionAttachment")
                        }
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
                    Some(
                        crate::model::GeneratedCodecRecipe::EnglishCardinal(_)
                            | crate::model::GeneratedCodecRecipe::UnsignedDecimal(_)
                    )
                ) {
                    for (prefix, role) in [
                        ("format_", "formatter"),
                        ("parse_", "parser"),
                        ("render_", "renderer"),
                    ] {
                        names.register_value(
                            &prefixed(prefix, &name),
                            &format!("generated unsigned numeral {role} for `{name}`"),
                            binding.name.span(),
                            errors,
                        );
                    }
                    if matches!(
                        binding.generated,
                        Some(crate::model::GeneratedCodecRecipe::EnglishCardinal(_))
                    ) {
                        for (feature, label) in [
                            ("concord_class", "ConcordClass"),
                            ("determiner_number", "DeterminerNumber"),
                            ("number", "Number"),
                            ("cardinality", "Cardinality"),
                        ] {
                            names.register_value(
                                &feature_helper(feature, &name),
                                &format!("generated {label} provider for `{name}`"),
                                binding.name.span(),
                                errors,
                            );
                        }
                    }
                }
                if let Some(family) = match binding.generated {
                    Some(crate::model::GeneratedCodecRecipe::DeclarationNoun(_)) => {
                        Some("declaration_noun")
                    }
                    Some(crate::model::GeneratedCodecRecipe::DeclarationDeterminative(_)) => {
                        Some("declaration_determinative")
                    }
                    Some(crate::model::GeneratedCodecRecipe::DeclarationVerb(_)) => {
                        Some("declaration_verb")
                    }
                    Some(
                        crate::model::GeneratedCodecRecipe::DeclarationTerm(_)
                        | crate::model::GeneratedCodecRecipe::SignedDecimal(_)
                        | crate::model::GeneratedCodecRecipe::EnglishCardinal(_)
                        | crate::model::GeneratedCodecRecipe::UnsignedDecimal(_)
                        | crate::model::GeneratedCodecRecipe::Unsupported { .. },
                    )
                    | None => None,
                } {
                    let open_value = format!("Declaration{}", identifier_key(&binding.name));
                    register_terminal_names(
                        &mut names,
                        &open_value,
                        &format!("{family} open value"),
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
                    (
                        Some(
                            crate::model::GeneratedCodecRecipe::EnglishCardinal(_)
                            | crate::model::GeneratedCodecRecipe::UnsignedDecimal(_),
                        ),
                        None,
                    ) => {
                        names.register_terminal_variant(
                            &name,
                            &format!("generated unsigned numeral terminal variant for `{name}`"),
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
                    (None, Some(crate::model::GeneratedIdentityRecipe::Catalog(source))) => {
                        names.register_terminal_variant(
                            &name,
                            &format!("generated catalog identity terminal variant for `{name}`"),
                            binding.name.span(),
                            errors,
                        );
                        for provider in &source.provider_slots {
                            names.register_catalog_provider_variant(&provider.value, errors);
                        }
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
                    false,
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
                    let variant = identifier_key(&alternative.name);
                    if explicit_sum_owned.contains(&owner) {
                        names.register_category_variant(
                            &owner,
                            &variant,
                            &format!("generated explicit sum variant for `{owner}.{variant}`"),
                            alternative.name.span(),
                            errors,
                        );
                        names.register_rule_id_variant(
                            &format!("{}{}", pascal_case(&owner), pascal_case(&variant)),
                            &format!("generated structural sum RuleId for `{owner}.{variant}`"),
                            alternative.name.span(),
                            errors,
                        );
                    } else {
                        names.register_rule_variant(
                            &format!("{}{}", pascal_case(&owner), pascal_case(&variant)),
                            &format!("generated structural sum RuleId for `{owner}.{variant}`"),
                            alternative.name.span(),
                            errors,
                        );
                    }
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
            FieldKind::Zeroable { .. } | FieldKind::Optional(_) => {
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
    rule_id_only: bool,
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
        if rule_id_only {
            names.register_rule_id_variant(&variant, &role, span, errors);
        } else {
            names.register_rule_variant(&variant, &role, span, errors);
        }
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
        Feature::ConcordClass => ParsedFeature::ConcordClass,
        Feature::BareDurationLicense => ParsedFeature::BareDurationLicense,
        Feature::BareLocativeLicense => ParsedFeature::BareLocativeLicense,
        Feature::Cardinality => ParsedFeature::Cardinality,
        Feature::Compoundability => ParsedFeature::Compoundability,
        Feature::Countability => ParsedFeature::Countability,
        Feature::HomographLicense => ParsedFeature::HomographLicense,
        Feature::MannerAnaphorClass => ParsedFeature::MannerAnaphorClass,
        Feature::ModifierLicense => ParsedFeature::ModifierLicense,
        Feature::DeterminerNumber => ParsedFeature::DeterminerNumber,
        Feature::FusedHeadLicense => ParsedFeature::FusedHeadLicense,
        Feature::Focus => ParsedFeature::Focus,
        Feature::PrepositionComplementKind => ParsedFeature::PrepositionComplementKind,
        Feature::LocativeTemporalLicense => ParsedFeature::LocativeTemporalLicense,
        Feature::NominalForm => ParsedFeature::NominalForm,
        Feature::NominalLicense => ParsedFeature::NominalLicense,
        Feature::Number => ParsedFeature::Number,
        Feature::Onset => ParsedFeature::Onset,
        Feature::Participle => ParsedFeature::Participle,
        Feature::PossessiveEnding => ParsedFeature::PossessiveEnding,
        Feature::Properness => ParsedFeature::Properness,
        Feature::Relationality => ParsedFeature::Relationality,
        Feature::BareLocativeComplement => ParsedFeature::BareLocativeComplement,
        Feature::PrepositionAttachment => ParsedFeature::PrepositionAttachment,
    };
    raw.declarations.iter().any(|declaration| {
        let Declaration::Construction(construction) = declaration else { return false };
        construction.equations.iter().any(|equation| {
            matches!(&equation.value, ParsedFeatureValue::FromRole(slot) if slot.feature == parsed_feature
                && construction.element.fields.iter().any(|field| identifier_key(&field.name) == identifier_key(&slot.role) && matches!(&field.kind, FieldKind::Category(path) if path_name(path) == category))
                && !construction.equations.iter().any(|writer| matches!(&writer.target, ParsedFeaturePlace::Role { field, feature: writer_feature } if identifier_key(field) == identifier_key(&slot.role) && *writer_feature == parsed_feature)))
        }) || construction.equations.iter().any(|equation| {
            matches!(&equation.target, ParsedFeaturePlace::Role { field, feature: target_feature }
                if *target_feature == parsed_feature
                    && construction.element.fields.iter().any(|candidate| {
                        same_identifier(&candidate.name, field)
                            && matches!(&candidate.kind, FieldKind::Zeroable { item, .. }
                                if matches!(item.as_ref(), FieldKind::Category(path) if path_name(path) == category))
                    }))
        }) || (feature == Feature::Number
            && construction
                .forms
                .iter()
                .flat_map(|form| &form.atoms)
                .any(|atom| matches!(atom, FormAtom::Noun(_)))
            && path_name(&construction.category) == category)
    })
}

fn raw_sequence_reads_inherent_category_feature(
    raw: &Declarations,
    category: &str,
    feature: Feature,
    category_render: &HashMap<String, CategoryRenderCapability>,
) -> bool {
    if feature == Feature::ConcordClass
        && category_render
            .get(category)
            .is_some_and(|capability| capability.requires_external_concord_class())
    {
        return false;
    }
    let parsed_feature = match feature {
        Feature::ConcordClass => ParsedFeature::ConcordClass,
        Feature::BareDurationLicense => ParsedFeature::BareDurationLicense,
        Feature::BareLocativeLicense => ParsedFeature::BareLocativeLicense,
        Feature::Cardinality => ParsedFeature::Cardinality,
        Feature::Compoundability => ParsedFeature::Compoundability,
        Feature::Countability => ParsedFeature::Countability,
        Feature::HomographLicense => ParsedFeature::HomographLicense,
        Feature::MannerAnaphorClass => ParsedFeature::MannerAnaphorClass,
        Feature::ModifierLicense => ParsedFeature::ModifierLicense,
        Feature::DeterminerNumber => ParsedFeature::DeterminerNumber,
        Feature::FusedHeadLicense => ParsedFeature::FusedHeadLicense,
        Feature::Focus => ParsedFeature::Focus,
        Feature::PrepositionComplementKind => ParsedFeature::PrepositionComplementKind,
        Feature::LocativeTemporalLicense => ParsedFeature::LocativeTemporalLicense,
        Feature::NominalForm => ParsedFeature::NominalForm,
        Feature::NominalLicense => ParsedFeature::NominalLicense,
        Feature::Number => ParsedFeature::Number,
        Feature::Onset => ParsedFeature::Onset,
        Feature::Participle => ParsedFeature::Participle,
        Feature::PossessiveEnding => ParsedFeature::PossessiveEnding,
        Feature::Properness => ParsedFeature::Properness,
        Feature::Relationality => ParsedFeature::Relationality,
        Feature::BareLocativeComplement => ParsedFeature::BareLocativeComplement,
        Feature::PrepositionAttachment => ParsedFeature::PrepositionAttachment,
    };
    raw.declarations.iter().any(|declaration| {
        let Declaration::Construction(construction) = declaration else {
            return false;
        };
        construction.equations.iter().any(|equation| {
            matches!(
                &equation.value,
                ParsedFeatureValue::FromRole(source)
                    if source.feature == parsed_feature
                        && construction.element.fields.iter().any(|field| {
                            same_identifier(&field.name, &source.role)
                                && matches!(
                                    &field.kind,
                                    FieldKind::Sequence { item, .. }
                                        if matches!(item.as_ref(), FieldKind::Category(path) if path_name(path) == category)
                                )
                        })
                        && (feature == Feature::Number
                            || !construction.equations.iter().any(|writer| {
                                matches!(
                                    &writer.target,
                                    ParsedFeaturePlace::Role { field, feature }
                                        if same_identifier(field, &source.role)
                                            && *feature == parsed_feature
                                )
                            }))
            )
        })
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

fn validate_bound_form_atom<'a>(
    atom: &'a FormAtom,
    is_form_final: bool,
    fields: &HashMap<String, &FieldKind>,
    errors: &mut Option<syn::Error>,
) -> &'a FormAtom {
    let FormAtom::Bound(bound) = atom else {
        return atom;
    };
    if let Some(authored_affix) = &bound.affix {
        let affix = authored_affix.value();
        if affix.is_empty() {
            combine(
                errors,
                syn::Error::new(
                    authored_affix.span(),
                    "a bound affix must have a nonempty fixed byte surface",
                ),
            );
        }
        if affix.chars().any(char::is_whitespace) {
            combine(
                errors,
                syn::Error::new(
                    authored_affix.span(),
                    "a bound affix must not contain whitespace",
                ),
            );
        }
    }
    if let FormAtom::Literal(literal) | FormAtom::LicensedLiteral(literal) = bound.value.as_ref() {
        combine(
            errors,
            syn::Error::new(
                literal.span(),
                "a bound atom accepts exactly one ordinary value atom",
            ),
        );
    }
    let value_role = match bound.value.as_ref() {
        FormAtom::Role(role)
        | FormAtom::Lex(role)
        | FormAtom::Identity(role)
        | FormAtom::Noun(role)
        | FormAtom::Verb(VerbOperand::Projected(role)) => Some(role),
        FormAtom::Literal(_)
        | FormAtom::LicensedLiteral(_)
        | FormAtom::SentenceInitial(_)
        | FormAtom::StructuralLiteral(_)
        | FormAtom::FixedLex(_)
        | FormAtom::Verb(VerbOperand::Fixed(_))
        | FormAtom::OpenVerb(_)
        | FormAtom::Marked(_)
        | FormAtom::Bound(_)
        | FormAtom::Circumfix(_) => None,
    };
    if let Some(role) = value_role
        && matches!(
            fields.get(&identifier_key(role)),
            Some(FieldKind::Optional(_) | FieldKind::Sequence { .. })
        )
    {
        combine(
            errors,
            syn::Error::new(
                role.span(),
                "bound atom values must be required, singular fields",
            ),
        );
    }
    if bound.affix.is_none()
        && is_form_final
        && let Some(role) = value_role
    {
        combine(
            errors,
            syn::Error::new(
                role.span(),
                "a boundary-only atom must be followed by the material it binds",
            ),
        );
    }
    &bound.value
}

fn validate_circumfix_form_atom(
    circumfix: &crate::model::CircumfixAtom,
    fields: &HashMap<String, &FieldKind>,
    errors: &mut Option<syn::Error>,
) {
    for (side, affix) in [("prefix", &circumfix.prefix), ("suffix", &circumfix.suffix)] {
        let surface = affix.value();
        if surface.is_empty() {
            combine(
                errors,
                syn::Error::new(
                    affix.span(),
                    format!("a circumfix {side} must have a nonempty fixed byte surface"),
                ),
            );
        }
        if surface.chars().any(char::is_whitespace) {
            combine(
                errors,
                syn::Error::new(
                    affix.span(),
                    format!("a circumfix {side} must not contain whitespace"),
                ),
            );
        }
    }
    if matches!(
        fields.get(&identifier_key(&circumfix.role)),
        Some(FieldKind::Optional(_))
    ) {
        combine(
            errors,
            syn::Error::new(
                circumfix.role.span(),
                "circumfix atom values must be required singular or sequence fields",
            ),
        );
    }
    check_role_kind(&circumfix.role, fields, true, errors);
}

#[expect(
    clippy::too_many_lines,
    reason = "resolution validates every form, feature flow, and terminal family together"
)]
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
        let form_verb_rows = construction
            .forms
            .iter()
            .map(form_verbs)
            .collect::<Vec<_>>();
        let verb_operands = form_verb_rows
            .iter()
            .flat_map(|(operands, _)| operands.iter().copied())
            .collect::<Vec<_>>();
        let open_verb_count = form_verb_rows
            .iter()
            .map(|(_, open_verb_count)| open_verb_count)
            .sum::<usize>();
        let has_fixed_verb = verb_operands
            .iter()
            .any(|operand| matches!(operand, VerbOperand::Fixed(_)))
            || open_verb_count != 0;
        let mut local_vocab_providers =
            local_vocab_feature_providers(construction, &fields, symbols);
        local_vocab_providers.extend(fields.keys().filter_map(|field| {
            let field = syn::Ident::new(field, construction.name.span());
            role_provides_number(raw, &fields, &field)
                .then(|| (identifier_key(&field), ParsedFeature::Number))
        }));
        local_vocab_providers.extend(fields.keys().filter_map(|field| {
            let field = syn::Ident::new(field, construction.name.span());
            role_provides_cardinality(raw, &fields, &field)
                .then(|| (identifier_key(&field), ParsedFeature::Cardinality))
        }));
        local_vocab_providers.extend(construction.equations.iter().filter_map(|equation| {
            let ParsedFeaturePlace::Role { field, feature } = &equation.target else {
                return None;
            };
            match feature {
                ParsedFeature::Cardinality => role_provides_cardinality(raw, &fields, field)
                    .then(|| (identifier_key(field), ParsedFeature::Cardinality)),
                ParsedFeature::Number => role_provides_number(raw, &fields, field)
                    .then(|| (identifier_key(field), ParsedFeature::Number)),
                ParsedFeature::ConcordClass
                | ParsedFeature::BareDurationLicense
                | ParsedFeature::BareLocativeLicense
                | ParsedFeature::Compoundability
                | ParsedFeature::Countability
                | ParsedFeature::HomographLicense
                | ParsedFeature::MannerAnaphorClass
                | ParsedFeature::ModifierLicense
                | ParsedFeature::DeterminerNumber
                | ParsedFeature::FusedHeadLicense
                | ParsedFeature::Focus
                | ParsedFeature::PrepositionComplementKind
                | ParsedFeature::LocativeTemporalLicense
                | ParsedFeature::NominalForm
                | ParsedFeature::NominalLicense
                | ParsedFeature::Onset
                | ParsedFeature::Participle
                | ParsedFeature::PossessiveEnding
                | ParsedFeature::BareLocativeComplement
                | ParsedFeature::PrepositionAttachment
                | ParsedFeature::Properness
                | ParsedFeature::Relationality => None,
            }
        }));
        for (form, (form_verbs, form_open_verb_count)) in
            construction.forms.iter().zip(&form_verb_rows)
        {
            if form_verbs.len() + form_open_verb_count > 1 {
                combine(
                    &mut errors,
                    syn::Error::new(
                        form.name.span(),
                        "an MVP form may contain only one concord_class-bearing verb slot",
                    ),
                );
            }
        }
        if has_fixed_verb && fields.contains_key("verb") {
            let span = construction
                .element
                .fields
                .iter()
                .find(|field| identifier_key(&field.name) == "verb")
                .map_or(construction.forms[0].name.span(), |field| field.name.span());
            combine(
                &mut errors,
                syn::Error::new(
                    span,
                    "field `verb` collides with the fixed verb atom's implicit concord_class slot",
                ),
            );
        }
        validate_form_guard_subject_composition(construction, &fields, symbols, &mut errors);
        validate_feature_guarded_traversal_programs(construction, &mut errors);
        for field in &construction.element.fields {
            validate_resolved_field_kind(&field.kind, symbols, &mut errors);
            if let Some(check) = &field.check {
                if !matches!(
                    &field.kind,
                    FieldKind::Category(_)
                        | FieldKind::Zeroable { .. }
                        | FieldKind::Optional(_)
                        | FieldKind::Sequence { .. }
                        | FieldKind::Lex(_)
                ) {
                    combine(
                        &mut errors,
                        syn::Error::new(
                            field.name.span(),
                            "checked fields require a category, lexical, zeroable category, optional category, or sequence value",
                        ),
                    );
                }
                for argument in &check.arguments {
                    match argument {
                        crate::model::FieldCheckArgument::Feature(argument) => {
                            check_feature_role(
                                &argument.role,
                                argument.feature,
                                &fields,
                                symbols,
                                &feature_providers,
                                &local_vocab_providers,
                                has_fixed_verb,
                                &verb_operands,
                                &mut errors,
                            );
                        }
                        crate::model::FieldCheckArgument::Value { role } => {
                            if !fields.contains_key(&identifier_key(role)) {
                                combine(
                                    &mut errors,
                                    syn::Error::new(
                                        role.span(),
                                        "field-check value projection names an unknown role",
                                    ),
                                );
                            }
                        }
                        crate::model::FieldCheckArgument::VerbFrameRolePrepositions { role } => {
                            check_verb_frame_role_prepositions(role, &fields, symbols, &mut errors);
                        }
                    }
                }
            }
        }
        for form in &construction.forms {
            validate_form_guard(construction, form, &fields, symbols, &mut errors);
            let final_atom_index = form.atoms.len().saturating_sub(1);
            for (atom_index, atom) in form.atoms.iter().enumerate() {
                if let FormAtom::Circumfix(circumfix) = atom {
                    validate_circumfix_form_atom(circumfix, &fields, &mut errors);
                    continue;
                }
                let atom = validate_bound_form_atom(
                    atom,
                    atom_index == final_atom_index,
                    &fields,
                    &mut errors,
                );
                match atom {
                    FormAtom::Role(role) => check_role_kind(role, &fields, true, &mut errors),
                    FormAtom::Lex(role) => check_lex_role(role, &fields, symbols, &mut errors),
                    FormAtom::FixedLex(path) => {
                        check_terminal_variant(path, TerminalKind::Vocab, symbols, &mut errors);
                    }
                    FormAtom::Marked(marked) => {
                        check_terminal_variant(
                            &marked.marker,
                            TerminalKind::Vocab,
                            symbols,
                            &mut errors,
                        );
                        check_role_kind(&marked.role, &fields, true, &mut errors);
                    }
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
                    }
                    FormAtom::Verb(VerbOperand::Fixed(path)) => {
                        check_terminal_variant(path, TerminalKind::Lexeme, symbols, &mut errors);
                    }
                    FormAtom::OpenVerb(open) => validate_open_declaration(open, &mut errors),
                    FormAtom::Literal(_)
                    | FormAtom::LicensedLiteral(_)
                    | FormAtom::SentenceInitial(_)
                    | FormAtom::StructuralLiteral(_) => {}
                    FormAtom::Bound(_) => unreachable!("bound atom values cannot nest"),
                    FormAtom::Circumfix(_) => {
                        unreachable!("circumfix atoms were validated before ordinary atoms")
                    }
                }
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum FiniteGuardSubjectKind {
    Membership,
    OptionalPresence,
    Feature(ParsedFeature),
}

fn validate_form_guard_subject_composition(
    construction: &crate::model::Construction,
    fields: &HashMap<String, &FieldKind>,
    symbols: &Symbols,
    errors: &mut Option<syn::Error>,
) {
    fn collect<'a>(
        guard: &'a RequireExprSource,
        subjects: &mut Vec<(&'a syn::Ident, FiniteGuardSubjectKind)>,
    ) {
        match guard {
            RequireExprSource::OptionalPresence { role, .. } => {
                subjects.push((role, FiniteGuardSubjectKind::OptionalPresence));
            }
            RequireExprSource::In {
                subject: crate::model::RequireSubjectSource::Role(role),
                ..
            } => subjects.push((role, FiniteGuardSubjectKind::Membership)),
            RequireExprSource::In {
                subject: crate::model::RequireSubjectSource::RoleFeature { role, feature },
                ..
            } => subjects.push((role, FiniteGuardSubjectKind::Feature(*feature))),
            RequireExprSource::All(operands) | RequireExprSource::Any(operands) => {
                for operand in operands {
                    collect(operand, subjects);
                }
            }
            RequireExprSource::In { .. } | RequireExprSource::Length { .. } => {}
        }
    }

    let mut first_by_role = HashMap::<String, FiniteGuardSubjectKind>::new();
    for form in &construction.forms {
        let crate::model::FormGuardSource::When(guard) = &form.guard else {
            continue;
        };
        let mut subjects = Vec::new();
        collect(guard, &mut subjects);
        for (role, kind) in subjects {
            let role_key = identifier_key(role);
            if let Some(existing) = first_by_role.get(&role_key)
                && existing != &kind
            {
                let membership_and_presence = matches!(
                    (existing, kind),
                    (
                        FiniteGuardSubjectKind::Membership,
                        FiniteGuardSubjectKind::OptionalPresence
                    ) | (
                        FiniteGuardSubjectKind::OptionalPresence,
                        FiniteGuardSubjectKind::Membership
                    )
                );
                let optional_vocab = fields
                    .get(&role_key)
                    .and_then(|field| finite_vocab_role(field))
                    .is_some_and(|(path, optional)| {
                        optional
                            && symbols
                                .terminals
                                .get(&path_name(path))
                                .is_some_and(|terminal| terminal.kind == TerminalKind::Vocab)
                    });
                if !(membership_and_presence && optional_vocab) {
                    combine(
                        errors,
                        syn::Error::new(
                            role.span(),
                            format!(
                                "form guards cannot mix finite subjects for role `{role}`; correlation is not modeled"
                            ),
                        ),
                    );
                }
            } else {
                first_by_role.insert(role_key, kind);
            }
        }
    }
}

fn validate_feature_guarded_traversal_programs(
    construction: &crate::model::Construction,
    errors: &mut Option<syn::Error>,
) {
    fn contains_feature_subject(guard: &RequireExprSource) -> bool {
        match guard {
            RequireExprSource::In {
                subject: crate::model::RequireSubjectSource::RoleFeature { .. },
                ..
            } => true,
            RequireExprSource::All(operands) | RequireExprSource::Any(operands) => {
                operands.iter().any(contains_feature_subject)
            }
            RequireExprSource::OptionalPresence { .. }
            | RequireExprSource::In { .. }
            | RequireExprSource::Length { .. } => false,
        }
    }

    let has_feature_guard = construction.forms.iter().any(|form| {
        matches!(&form.guard, crate::model::FormGuardSource::When(guard) if contains_feature_subject(guard))
    });
    if !has_feature_guard {
        return;
    }

    let signature = |form: &crate::model::Form| {
        form.atoms
            .iter()
            .filter_map(|atom| {
                let atom = match atom {
                    FormAtom::Bound(bound) => bound.value.as_ref(),
                    atom => atom,
                };
                match atom {
                    FormAtom::Literal(_)
                    | FormAtom::LicensedLiteral(_)
                    | FormAtom::SentenceInitial(_)
                    | FormAtom::StructuralLiteral(_) => None,
                    FormAtom::FixedLex(path) => Some(format!("lex-fixed:{}", path_name(path))),
                    FormAtom::Marked(marked) => Some(format!(
                        "marked:{}:{}",
                        path_name(&marked.marker),
                        identifier_key(&marked.role)
                    )),
                    FormAtom::Role(role) => Some(format!("category:{}", identifier_key(role))),
                    FormAtom::Lex(role) => Some(format!("lex:{}", identifier_key(role))),
                    FormAtom::Identity(role) => Some(format!("identity:{}", identifier_key(role))),
                    FormAtom::Noun(role) => Some(format!("noun:{}", identifier_key(role))),
                    FormAtom::Verb(VerbOperand::Projected(role)) => {
                        Some(format!("verb-role:{}", identifier_key(role)))
                    }
                    FormAtom::Verb(VerbOperand::Fixed(path)) => {
                        Some(format!("verb-fixed:{}", path_name(path)))
                    }
                    FormAtom::OpenVerb(open) => Some(format!(
                        "open-verb:{}:{}",
                        identifier_key(&open.kind),
                        open.name.value()
                    )),
                    FormAtom::Bound(_) => unreachable!("bound atom values cannot nest"),
                    FormAtom::Circumfix(circumfix) => {
                        Some(format!("category:{}", identifier_key(&circumfix.role)))
                    }
                }
            })
            .collect::<Vec<_>>()
    };
    let Some(canonical) = construction.forms.first() else {
        return;
    };
    let canonical_signature = signature(canonical);
    for form in construction.forms.iter().skip(1) {
        if signature(form) != canonical_signature {
            combine(
                errors,
                syn::Error::new(
                    form.name.span(),
                    format!(
                        "feature-guarded forms must have identical traversal programs; form `{}` differs from `{}`",
                        form.name, canonical.name
                    ),
                ),
            );
        }
    }
}

fn validate_resolved_field_kind(
    kind: &FieldKind,
    symbols: &Symbols,
    errors: &mut Option<syn::Error>,
) {
    match kind {
        FieldKind::Zeroable { item, .. } | FieldKind::Sequence { item, .. } => {
            validate_resolved_field_kind(item, symbols, errors);
        }
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

fn form_verbs(form: &crate::model::Form) -> (Vec<&VerbOperand>, usize) {
    let verb_operands = form
        .atoms
        .iter()
        .filter_map(|atom| match atom {
            FormAtom::Verb(operand) => Some(operand),
            _ => None,
        })
        .collect();
    let open_verb_count = form
        .atoms
        .iter()
        .filter(|atom| matches!(atom, FormAtom::OpenVerb(_)))
        .count();
    (verb_operands, open_verb_count)
}

fn validate_form_guard(
    construction: &crate::model::Construction,
    form: &crate::model::Form,
    fields: &HashMap<String, &FieldKind>,
    symbols: &Symbols,
    errors: &mut Option<syn::Error>,
) {
    let FormGuardSource::When(guard) = &form.guard else {
        return;
    };
    validate_form_guard_expr(construction, guard, fields, symbols, errors);
}

fn validate_form_guard_expr(
    construction: &crate::model::Construction,
    guard: &RequireExprSource,
    fields: &HashMap<String, &FieldKind>,
    symbols: &Symbols,
    errors: &mut Option<syn::Error>,
) {
    match guard {
        RequireExprSource::OptionalPresence { role, .. } => match fields.get(&identifier_key(role))
        {
            Some(FieldKind::Zeroable { .. } | FieldKind::Optional(_)) => {}
            Some(_) => combine(
                errors,
                syn::Error::new(
                    role.span(),
                    "optional-presence guard requires an optional role",
                ),
            ),
            None => combine(
                errors,
                syn::Error::new(role.span(), format!("unknown form-guard role `{role}`")),
            ),
        },
        RequireExprSource::In {
            subject: crate::model::RequireSubjectSource::Role(role),
            ..
        } => match fields.get(&identifier_key(role)) {
            Some(kind) if finite_vocab_role(kind).is_some() => {
                let (path, _) = finite_vocab_role(kind).expect("guarded above");
                match symbols.terminals.get(&path_name(path)) {
                    Some(info) if info.kind == TerminalKind::Vocab => {}
                    Some(_) => combine(
                        errors,
                        syn::Error::new(role.span(), "form guard membership requires a vocab role"),
                    ),
                    None => combine(
                        errors,
                        syn::Error::new(role.span(), format!("unknown form-guard role `{role}`")),
                    ),
                }
            }
            Some(_) => combine(
                errors,
                syn::Error::new(role.span(), "form guard membership requires a vocab role"),
            ),
            None => combine(
                errors,
                syn::Error::new(role.span(), format!("unknown form-guard role `{role}`")),
            ),
        },
        RequireExprSource::In {
            subject: crate::model::RequireSubjectSource::RoleFeature { role, feature: _ },
            ..
        } => {
            if !fields.contains_key(&identifier_key(role)) {
                combine(
                    errors,
                    syn::Error::new(role.span(), format!("unknown form-guard role `{role}`")),
                );
            }
        }
        RequireExprSource::In {
            subject: crate::model::RequireSubjectSource::ConstructionFeature(_),
            ..
        } => {}
        RequireExprSource::All(operands) | RequireExprSource::Any(operands) => {
            for operand in operands {
                validate_form_guard_expr(construction, operand, fields, symbols, errors);
            }
        }
        RequireExprSource::Length { .. } => combine(
            errors,
            syn::Error::new(
                construction.name.span(),
                "form guard contains an unsupported predicate subject",
            ),
        ),
    }
}

fn open_declaration_kind(kind: &syn::Ident) -> Option<crate::macro_def::DeclarationKind> {
    use crate::macro_def::DeclarationKind;

    match kind.to_string().as_str() {
        "KeywordAction" => Some(DeclarationKind::KeywordAction),
        "KeywordAbility" => Some(DeclarationKind::KeywordAbility),
        "FlavorWord" => Some(DeclarationKind::FlavorWord),
        "Type" => Some(DeclarationKind::Type),
        "TurnPart" => Some(DeclarationKind::TurnPart),
        "CounterKind" => Some(DeclarationKind::CounterKind),
        "Designation" => Some(DeclarationKind::Designation),
        _ => None,
    }
}

#[expect(
    clippy::too_many_lines,
    reason = "grammar-use resolution exhaustively seals every form atom shape"
)]
fn resolve_grammar_uses(raw: &Declarations) -> syn::Result<ResolvedGrammar> {
    let mut errors = None;
    let mut atoms_by_construction = HashMap::new();

    for declaration in &raw.declarations {
        let Declaration::Construction(construction) = declaration else { continue };
        let fields: HashMap<_, _> = construction
            .element
            .fields
            .iter()
            .map(|field| (identifier_key(&field.name), &field.kind))
            .collect();
        let mut atoms = Vec::new();
        for atom in construction.forms.iter().flat_map(|form| &form.atoms) {
            if let FormAtom::Circumfix(circumfix) = atom {
                if let Some(FieldKind::Category(path)) = fields
                    .get(&identifier_key(&circumfix.role))
                    .map(|kind| field_kind_leaf(kind))
                {
                    atoms.push(AtomContribution::Category {
                        role: identifier_key(&circumfix.role),
                        category: path_name(path),
                    });
                }
                continue;
            }
            let atom = match atom {
                FormAtom::Bound(bound) => bound.value.as_ref(),
                atom => atom,
            };
            let resolved = match atom {
                FormAtom::Literal(_)
                | FormAtom::LicensedLiteral(_)
                | FormAtom::SentenceInitial(_)
                | FormAtom::StructuralLiteral(_) => Some(AtomContribution::Literal),
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
                FormAtom::FixedLex(path) => {
                    let segments: Vec<_> = path.segments.iter().collect();
                    let Some(terminal) = segments.iter().rev().nth(1) else { continue };
                    let Some(variant) = segments.last() else { continue };
                    Some(AtomContribution::LexFixed {
                        terminal: identifier_key(&terminal.ident),
                        variant: identifier_key(&variant.ident),
                    })
                }
                FormAtom::Marked(marked) => {
                    let segments: Vec<_> = marked.marker.segments.iter().collect();
                    let Some(terminal) = segments.iter().rev().nth(1) else { continue };
                    let Some(variant) = segments.last() else { continue };
                    match fields
                        .get(&identifier_key(&marked.role))
                        .map(|kind| field_kind_leaf(kind))
                    {
                        Some(FieldKind::Category(path)) => Some(AtomContribution::Marked {
                            role: identifier_key(&marked.role),
                            category: path_name(path),
                            terminal: identifier_key(&terminal.ident),
                            variant: identifier_key(&variant.ident),
                        }),
                        _ => None,
                    }
                }
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
                FormAtom::Verb(VerbOperand::Projected(role)) => match fields
                    .get(&identifier_key(role))
                    .map(|kind| field_kind_leaf(kind))
                {
                    Some(FieldKind::Lex(path)) => Some(AtomContribution::VerbProjected {
                        role: identifier_key(role),
                        terminal: path_name(path),
                    }),
                    _ => None,
                },
                FormAtom::Bound(_) => unreachable!("bound atom values cannot nest"),
                FormAtom::Circumfix(_) => {
                    unreachable!("circumfix atoms resolve before ordinary atoms")
                }
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

    let fixed_verb_terminals = atoms_by_construction
        .values()
        .flat_map(|(_, atoms)| atoms)
        .filter_map(|atom| match atom {
            AtomContribution::VerbFixed { terminal, .. } => Some(terminal.as_str()),
            _ => None,
        })
        .collect::<HashSet<_>>();
    let verb_providers = raw
        .declarations
        .iter()
        .filter_map(|declaration| {
            let Declaration::Lexeme(lexeme) = declaration else {
                return None;
            };
            let name = identifier_key(&lexeme.name);
            (fixed_verb_terminals.contains(name.as_str())
                && lexeme_recipe(raw, lexeme)
                    == Some(crate::morphology::MorphologyRecipe::EnglishVerb))
            .then(|| (name, lexeme.name.span()))
        })
        .collect::<Vec<_>>();

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
        Some(FieldKind::Lex(path)) => symbols
            .terminals
            .get(&path_name(path))
            .is_some_and(|info| info.codec_atom == Some(CodecAtomClass::Noun)),
        Some(FieldKind::Identity(_)) => false,
        Some(
            FieldKind::Category(_)
            | FieldKind::Zeroable { .. }
            | FieldKind::Optional(_)
            | FieldKind::Sequence { .. },
        )
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

fn check_verb_frame_role_prepositions(
    role: &syn::Ident,
    fields: &HashMap<String, &FieldKind>,
    symbols: &Symbols,
    errors: &mut Option<syn::Error>,
) {
    let Some(FieldKind::Lex(path)) = fields.get(&identifier_key(role)) else {
        combine(
            errors,
            syn::Error::new(
                role.span(),
                format!(
                    "verb-frame role-preposition projection requires a declaration-verb lexical role; `{role}` is not one"
                ),
            ),
        );
        return;
    };
    let terminal = path_name(path);
    if !symbols
        .terminals
        .get(&terminal)
        .is_some_and(|info| info.declaration_verb)
    {
        combine(
            errors,
            syn::Error::new(
                role.span(),
                format!(
                    "verb-frame role-preposition projection requires a declaration-verb codec; `{terminal}` is not one"
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
        if !matches!(feature, ParsedFeature::ConcordClass | ParsedFeature::Onset) {
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
        Some(FieldKind::Zeroable { item, .. }) => {
            let FieldKind::Category(path) = item.as_ref() else {
                unreachable!("zeroable fields contain categories")
            };
            let category = path_name(path);
            if !providers.contains(&(category.clone(), feature)) {
                combine(
                    errors,
                    syn::Error::new(
                        role.span(),
                        format!("category `{category}` does not provide {}", feature_name(feature)),
                    ),
                );
            }
        }
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
        Some(FieldKind::Sequence { item, .. })
            if matches!(
                feature,
                ParsedFeature::ConcordClass
                    | ParsedFeature::Number
                    | ParsedFeature::Onset
                    | ParsedFeature::PossessiveEnding
            )
                && matches!(item.as_ref(), FieldKind::Category(path) if providers.contains(&(path_name(path), feature))) =>
        {
        }
        Some(FieldKind::Lex(_) | FieldKind::Identity(_))
            if matches!(
                feature,
                ParsedFeature::Onset | ParsedFeature::PossessiveEnding
            ) => {}
        Some(FieldKind::Lex(path))
            if providers.contains(&(path_name(path), feature)) =>
        {
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
                .is_some_and(|terminal| terminal.concord_class_verb) =>
        {
            if !matches!(feature, ParsedFeature::ConcordClass | ParsedFeature::Onset) {
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
        FieldKind::Zeroable { item, .. } | FieldKind::Sequence { item, .. } => {
            field_kind_leaf(item)
        }
        FieldKind::Optional(value) => field_kind_leaf(value),
        FieldKind::Category(_) | FieldKind::Lex(_) | FieldKind::Identity(_) => kind,
    }
}

fn finite_vocab_role(kind: &FieldKind) -> Option<(&syn::Path, bool)> {
    match kind {
        FieldKind::Lex(path) => Some((path, false)),
        FieldKind::Optional(inner) => match inner.as_ref() {
            FieldKind::Lex(path) => Some((path, true)),
            FieldKind::Category(_) | FieldKind::Identity(_) | FieldKind::Zeroable { .. } => None,
            FieldKind::Optional(_) | FieldKind::Sequence { .. } => {
                unreachable!("nested cardinalities are rejected while parsing")
            }
        },
        FieldKind::Category(_)
        | FieldKind::Identity(_)
        | FieldKind::Zeroable { .. }
        | FieldKind::Sequence { .. } => None,
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
            .is_some_and(|info| info.concord_class_verb || info.participle_verb),
        Some(FieldKind::Identity(_)) => false,
        Some(
            FieldKind::Category(_)
            | FieldKind::Zeroable { .. }
            | FieldKind::Optional(_)
            | FieldKind::Sequence { .. },
        )
        | None => return,
    };
    if !supported {
        combine(
            errors,
            syn::Error::new(
                role.span(),
                format!(
                    "verb role `{role}` must use a ConcordClass- or Participle-aware declaration_verb terminal"
                ),
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
        for form in &construction.forms {
            let mut counts: HashMap<String, usize> = construction
                .element
                .fields
                .iter()
                .map(|field| (identifier_key(&field.name), 0))
                .collect();
            for atom in &form.atoms {
                let atom = match atom {
                    FormAtom::Bound(bound) => bound.value.as_ref(),
                    atom => atom,
                };
                let role = match atom {
                    FormAtom::Role(role)
                    | FormAtom::Lex(role)
                    | FormAtom::Identity(role)
                    | FormAtom::Noun(role)
                    | FormAtom::Verb(VerbOperand::Projected(role)) => Some(role),
                    FormAtom::Marked(marked) => Some(&marked.role),
                    FormAtom::Verb(VerbOperand::Fixed(_))
                    | FormAtom::FixedLex(_)
                    | FormAtom::OpenVerb(_)
                    | FormAtom::Literal(_)
                    | FormAtom::LicensedLiteral(_)
                    | FormAtom::SentenceInitial(_)
                    | FormAtom::StructuralLiteral(_) => None,
                    FormAtom::Bound(_) => unreachable!("bound atom values cannot nest"),
                    FormAtom::Circumfix(circumfix) => Some(&circumfix.role),
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
                            format!(
                                "field `{}` is not present in form `{}`",
                                field.name, form.name
                            ),
                        ),
                    ),
                    1 => {}
                    count => combine(
                        &mut errors,
                        syn::Error::new(
                            field.name.span(),
                            format!(
                                "field `{}` is used {count} times in form `{}`",
                                field.name, form.name
                            ),
                        ),
                    ),
                }
            }
        }
    }
    finish(errors)
}

fn validate_mobile_roles(raw: &Declarations) -> syn::Result<()> {
    fn role(atom: &FormAtom) -> Option<&syn::Ident> {
        let atom = match atom {
            FormAtom::Bound(bound) => bound.value.as_ref(),
            atom => atom,
        };
        match atom {
            FormAtom::Role(role)
            | FormAtom::Lex(role)
            | FormAtom::Identity(role)
            | FormAtom::Noun(role)
            | FormAtom::Verb(VerbOperand::Projected(role)) => Some(role),
            FormAtom::Marked(marked) => Some(&marked.role),
            FormAtom::Circumfix(circumfix) => Some(&circumfix.role),
            FormAtom::Verb(VerbOperand::Fixed(_))
            | FormAtom::FixedLex(_)
            | FormAtom::OpenVerb(_)
            | FormAtom::Literal(_)
            | FormAtom::LicensedLiteral(_)
            | FormAtom::SentenceInitial(_)
            | FormAtom::StructuralLiteral(_) => None,
            FormAtom::Bound(_) => unreachable!("bound atom values cannot nest"),
        }
    }

    let mut errors = None;
    for declaration in &raw.declarations {
        if let Declaration::AbstractProduct(product) = declaration {
            for field in product.fields.iter().filter(|field| field.mobile) {
                combine(
                    &mut errors,
                    syn::Error::new(
                        field.name.span(),
                        format!(
                            "mobile role `{}` must belong to a construction element",
                            field.name,
                        ),
                    ),
                );
            }
        }
        let Declaration::Construction(construction) = declaration else {
            continue;
        };
        let fields = construction
            .element
            .fields
            .iter()
            .map(|field| (identifier_key(&field.name), field))
            .collect::<HashMap<_, _>>();
        if fields.contains_key(ADMISSIBLE_SITES_FIELD)
            && construction.element.fields.iter().any(|field| field.mobile)
        {
            combine(
                &mut errors,
                syn::Error::new(
                    construction.element.name.span(),
                    format!(
                        "a construction with a mobile role reserves the field name `{ADMISSIBLE_SITES_FIELD}`",
                    ),
                ),
            );
        }
        for field in construction
            .element
            .fields
            .iter()
            .filter(|field| field.mobile)
        {
            let contains_one_category = match &field.kind {
                FieldKind::Category(_) | FieldKind::Lex(_) => true,
                FieldKind::Optional(value) => {
                    matches!(value.as_ref(), FieldKind::Category(_) | FieldKind::Lex(_))
                }
                FieldKind::Zeroable { item, .. } => {
                    matches!(item.as_ref(), FieldKind::Category(_) | FieldKind::Lex(_))
                }
                FieldKind::Identity(_) | FieldKind::Sequence { .. } => false,
            };
            if !contains_one_category {
                combine(
                    &mut errors,
                    syn::Error::new(
                        field.name.span(),
                        format!(
                            "mobile role `{}` must contain one category Constituent",
                            field.name,
                        ),
                    ),
                );
            }
            let scope_sibling = field.mobile_scope_sibling.as_ref().map(identifier_key);
            if let Some(scope_sibling_name) = field.mobile_scope_sibling.as_ref() {
                let scope_sibling_key = identifier_key(scope_sibling_name);
                if !fields.contains_key(&scope_sibling_key) {
                    combine(
                        &mut errors,
                        syn::Error::new(
                            scope_sibling_name.span(),
                            format!(
                                "mobile role `{}` names unknown scope sibling `{scope_sibling_key}`",
                                field.name,
                            ),
                        ),
                    );
                }
            }
            let name = identifier_key(&field.name);
            for form in &construction.forms {
                let Some(index) = form
                    .atoms
                    .iter()
                    .position(|atom| role(atom).is_some_and(|role| identifier_key(role) == name))
                else {
                    continue;
                };
                let edge = index == 0 || index + 1 == form.atoms.len();
                let is_sequence_role = |atom: &FormAtom| {
                    role(atom)
                        .and_then(|role| fields.get(&identifier_key(role)))
                        .is_some_and(|field| matches!(field.kind, FieldKind::Sequence { .. }))
                };
                let sequence_adjacent = index
                    .checked_sub(1)
                    .is_some_and(|adjacent| is_sequence_role(&form.atoms[adjacent]))
                    || form.atoms.get(index + 1).is_some_and(is_sequence_role);
                let immediately_left_of_scope_sibling =
                    scope_sibling.as_ref().is_some_and(|scope_sibling| {
                        form.atoms
                            .get(index + 1)
                            .and_then(role)
                            .is_some_and(|role| identifier_key(role) == *scope_sibling)
                    });
                if !edge && !immediately_left_of_scope_sibling && !sequence_adjacent {
                    combine(
                        &mut errors,
                        syn::Error::new(
                            field.name.span(),
                            format!(
                                "mobile role `{}` must be at an edge, immediately left of its scope sibling, or adjacent to a `seq` role in form `{}`",
                                field.name, form.name,
                            ),
                        ),
                    );
                }
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
                } else if matches!(
                    binding.generated,
                    Some(
                        crate::model::GeneratedCodecRecipe::EnglishCardinal(_)
                            | crate::model::GeneratedCodecRecipe::UnsignedDecimal(_)
                    )
                ) {
                    register_terminal_callbacks(
                        &mut callbacks,
                        &binding.name,
                        VisitMode::Borrowed,
                        format!("unsigned numeral codec `{}`", binding.name),
                        errors,
                    );
                } else if let Some(identity) = &binding.generated_identity {
                    let (mode, family) = match identity {
                        crate::model::GeneratedIdentityRecipe::Context(_) => {
                            (VisitMode::Copy, "context")
                        }
                        crate::model::GeneratedIdentityRecipe::Catalog(_) => {
                            (VisitMode::Borrowed, "catalog")
                        }
                        crate::model::GeneratedIdentityRecipe::Unsupported { .. } => continue,
                    };
                    register_terminal_callbacks(
                        &mut callbacks,
                        &binding.name,
                        mode,
                        format!("{family} identity `{}`", binding.name),
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
        crate::RequireExprSource::OptionalPresence { role, present } => {
            let role_name = identifier_key(role);
            match fields.get(&role_name) {
                Some(FieldKind::Zeroable { .. }) => Ok(vec![PredicateConjunctionPlan::new(vec![
                    PredicateAtomPlan::new(
                        PredicateSubjectPlan::OptionalPresenceRole { role: role.clone() },
                        vec![PredicateMemberPlan::Presence(*present)],
                    ),
                ])]),
                Some(kind @ FieldKind::Optional(_)) => {
                    let subject = finite_vocab_role(kind)
                        .filter(|(_, optional)| *optional)
                        .and_then(|(path, _)| {
                            let terminal = path_name(path);
                            symbols
                                .terminals
                                .get(&terminal)
                                .filter(|info| info.kind == TerminalKind::Vocab)
                                .map(|_| PredicateSubjectPlan::VocabRole {
                                    role: role.clone(),
                                    terminal,
                                    optional: true,
                                })
                        })
                        .unwrap_or_else(|| PredicateSubjectPlan::OptionalPresenceRole {
                            role: role.clone(),
                        });
                    Ok(vec![PredicateConjunctionPlan::new(vec![
                        PredicateAtomPlan::new(
                            subject,
                            vec![PredicateMemberPlan::Presence(*present)],
                        ),
                    ])])
                }
                Some(_) => Err(syn::Error::new(
                    role.span(),
                    format!("optional-presence predicate subject `{role_name}` is not optional"),
                )),
                None => Err(syn::Error::new(
                    role.span(),
                    format!("unknown predicate subject `{role_name}`"),
                )),
            }
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
                Some(kind) if finite_vocab_role(kind).is_some() => {
                    let (path, optional) = finite_vocab_role(kind).expect("guarded above");
                    let terminal = path_name(path);
                    match symbols.terminals.get(&terminal) {
                        Some(info) if info.kind == TerminalKind::Vocab => (
                            PredicateSubjectPlan::VocabRole {
                                role: role.clone(),
                                terminal,
                                optional,
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
                    FieldKind::Identity(_)
                    | FieldKind::Zeroable { .. }
                    | FieldKind::Optional(_)
                    | FieldKind::Sequence { .. }
                    | FieldKind::Lex(_),
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
                        .forms
                        .iter()
                        .flat_map(|form| &form.atoms)
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
            let optional = matches!(
                fields.get(&role_name),
                Some(FieldKind::Optional(inner))
                    if matches!(inner.as_ref(), FieldKind::Category(_))
            );
            (
                PredicateSubjectPlan::RoleFeature {
                    role: role.clone(),
                    feature: internal,
                    optional,
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
    let optional_vocab = matches!(
        subject,
        PredicateSubjectPlan::VocabRole { optional: true, .. }
    );
    if !optional_vocab && allowed.len() == domain.len() {
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
            let allowed = intersect_predicate_members(left_atom, right_atom);
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

fn intersect_predicate_members(
    left: &PredicateAtomPlan,
    right: &PredicateAtomPlan,
) -> Vec<PredicateMemberPlan> {
    if !matches!(
        left.subject(),
        PredicateSubjectPlan::VocabRole { optional: true, .. }
    ) {
        let right_members = right
            .allowed()
            .iter()
            .map(PredicateMemberPlan::semantic_key)
            .collect::<HashSet<_>>();
        return left
            .allowed()
            .iter()
            .filter(|member| right_members.contains(&member.semantic_key()))
            .cloned()
            .collect();
    }

    let mut allowed = Vec::new();
    for left_member in left.allowed() {
        for right_member in right.allowed() {
            let member = match (left_member, right_member) {
                (PredicateMemberPlan::Variant(left), PredicateMemberPlan::Variant(right))
                    if same_identifier(left, right) =>
                {
                    Some(left_member.clone())
                }
                (PredicateMemberPlan::Presence(true), PredicateMemberPlan::Variant(_)) => {
                    Some(right_member.clone())
                }
                (PredicateMemberPlan::Variant(_), PredicateMemberPlan::Presence(true)) => {
                    Some(left_member.clone())
                }
                (PredicateMemberPlan::Presence(left), PredicateMemberPlan::Presence(right))
                    if left == right =>
                {
                    Some(left_member.clone())
                }
                (
                    PredicateMemberPlan::Variant(_)
                    | PredicateMemberPlan::Presence(_)
                    | PredicateMemberPlan::Feature(_),
                    PredicateMemberPlan::Variant(_)
                    | PredicateMemberPlan::Presence(_)
                    | PredicateMemberPlan::Feature(_),
                ) => None,
            };
            if let Some(member) = member
                && !allowed.iter().any(|existing: &PredicateMemberPlan| {
                    existing.semantic_key() == member.semantic_key()
                })
            {
                allowed.push(member);
            }
        }
    }
    allowed
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
                        (matches!(
                            *feature,
                            ParsedFeature::Onset | ParsedFeature::PossessiveEnding
                        )
                            && matches!(kind, FieldKind::Lex(_) | FieldKind::Identity(_)))
                            || matches!(kind, FieldKind::Category(path) | FieldKind::Lex(path) if providers.contains(&(path_name(path), *feature)))
                            || matches!(kind, FieldKind::Optional(item) if matches!(item.as_ref(), FieldKind::Category(path) if providers.contains(&(path_name(path), *feature))))
                            || matches!(kind, FieldKind::Zeroable { item, .. } if matches!(item.as_ref(), FieldKind::Category(path) if providers.contains(&(path_name(path), *feature))))
                            || matches!(kind, FieldKind::Sequence { item, .. } if matches!(item.as_ref(), FieldKind::Category(path) if providers.contains(&(path_name(path), *feature))))
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
                    if let Some(path) =
                        fields
                            .get(&identifier_key(field))
                            .and_then(|kind| match kind {
                                FieldKind::Category(path) => Some(path),
                                FieldKind::Sequence { item, .. } => match item.as_ref() {
                                    FieldKind::Category(path) => Some(path),
                                    _ => None,
                                },
                                _ => None,
                            })
                    {
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
                    if let Some(path) =
                        fields
                            .get(&identifier_key(&slot.role))
                            .and_then(|kind| match kind {
                                FieldKind::Category(path) => Some(path),
                                FieldKind::Sequence { item, .. } => match item.as_ref() {
                                    FieldKind::Category(path) => Some(path),
                                    _ => None,
                                },
                                _ => None,
                            })
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
        for atom in construction.forms.iter().flat_map(|form| &form.atoms) {
            match atom {
                FormAtom::Noun(_) if !has_number => combine(
                    &mut errors,
                    syn::Error::new(
                        construction.forms[0].name.span(),
                        "noun atom requires number in scope",
                    ),
                ),
                _ => {}
            }
        }
        for atom in construction.forms.iter().flat_map(|form| &form.atoms) {
            let slot = match atom {
                FormAtom::Verb(VerbOperand::Fixed(_)) | FormAtom::OpenVerb(_) => {
                    Some("verb".to_owned())
                }
                FormAtom::Verb(VerbOperand::Projected(role)) => Some(identifier_key(role)),
                _ => None,
            };
            let Some(slot) = slot else { continue };
            let participle_slot = fields
                .get(&slot)
                .and_then(|kind| match kind {
                    FieldKind::Lex(path) => symbols.terminals.get(&path_name(path)),
                    FieldKind::Category(_)
                    | FieldKind::Identity(_)
                    | FieldKind::Zeroable { .. }
                    | FieldKind::Optional(_)
                    | FieldKind::Sequence { .. } => None,
                })
                .is_some_and(|terminal| terminal.participle_verb);
            if participle_slot {
                continue;
            }
            let direct_writer = writers.contains(&format!("{slot}.concord_class"));
            let equality_writer = construction.equations.iter().any(|equation| {
                matches!(
                    (&equation.target, &equation.value),
                    (
                        ParsedFeaturePlace::Construction(ParsedFeature::ConcordClass),
                        ParsedFeatureValue::FromRole(source)
                    ) if identifier_key(&source.role) == slot && source.feature == ParsedFeature::ConcordClass
                )
            });
            if !direct_writer && !equality_writer {
                combine(
                    &mut errors,
                    syn::Error::new(
                        construction.forms[0].name.span(),
                        format!(
                            "verb atom requires concord_class binding for `{slot}.concord_class`"
                        ),
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
        places.extend(
            construction
                .forms
                .iter()
                .flat_map(|form| &form.atoms)
                .filter_map(|atom| {
                    let atom = match atom {
                        FormAtom::Bound(bound) => bound.value.as_ref(),
                        atom => atom,
                    };
                    let field = match atom {
                        FormAtom::Verb(VerbOperand::Fixed(_)) | FormAtom::OpenVerb(_) => {
                            syn::Ident::new("verb", construction.forms[0].name.span())
                        }
                        FormAtom::Verb(VerbOperand::Projected(role)) => role.clone(),
                        FormAtom::Literal(_)
                        | FormAtom::LicensedLiteral(_)
                        | FormAtom::SentenceInitial(_)
                        | FormAtom::StructuralLiteral(_)
                        | FormAtom::FixedLex(_)
                        | FormAtom::Marked(_)
                        | FormAtom::Role(_)
                        | FormAtom::Lex(_)
                        | FormAtom::Identity(_)
                        | FormAtom::Noun(_)
                        | FormAtom::Circumfix(_) => return None,
                        FormAtom::Bound(_) => unreachable!("bound atom values cannot nest"),
                    };
                    Some(feature::FeaturePlace::Role {
                        field,
                        feature: feature::Feature::ConcordClass,
                    })
                }),
        );
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

#[allow(
    clippy::too_many_lines,
    reason = "the category capability fixed point and its closed checks are kept together"
)]
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
    let mut concord_class_contextual = HashSet::new();

    let contextual_sums = |contextual_categories: &HashSet<String>| {
        let mut contextual = HashSet::new();
        loop {
            let before = contextual.len();
            for declaration in &raw.declarations {
                let Declaration::AbstractSum(sum) = declaration else { continue };
                if sum.alternatives.iter().any(|alternative| {
                    let value = path_name(&alternative.value_type);
                    contextual_categories.contains(&value) || contextual.contains(&value)
                }) {
                    contextual.insert(identifier_key(&sum.name));
                }
            }
            if contextual.len() == before {
                break;
            }
        }
        contextual
    };

    for construction in &constructions {
        let has_external_verb =
            construction
                .forms
                .iter()
                .flat_map(|form| &form.atoms)
                .any(|atom| {
                    let atom = match atom {
                        FormAtom::Bound(bound) => bound.value.as_ref(),
                        atom => atom,
                    };
                    let role = match atom {
                        FormAtom::Verb(VerbOperand::Fixed(_)) | FormAtom::OpenVerb(_) => {
                            "verb".to_owned()
                        }
                        FormAtom::Verb(VerbOperand::Projected(role)) => identifier_key(role),
                        FormAtom::Literal(_)
                        | FormAtom::LicensedLiteral(_)
                        | FormAtom::SentenceInitial(_)
                        | FormAtom::StructuralLiteral(_)
                        | FormAtom::FixedLex(_)
                        | FormAtom::Marked(_)
                        | FormAtom::Role(_)
                        | FormAtom::Lex(_)
                        | FormAtom::Identity(_)
                        | FormAtom::Noun(_)
                        | FormAtom::Circumfix(_) => return false,
                        FormAtom::Bound(_) => unreachable!("bound atom values cannot nest"),
                    };
                    if construction.element.fields.iter().any(|field| {
                        identifier_key(&field.name) == role
                            && matches!(&field.kind, FieldKind::Lex(path) if terminal_is_participle_declaration(raw, &path_name(path)))
                    }) {
                        return false;
                    }
                    let place = feature::FeaturePlace::Role {
                        field: syn::Ident::new(&role, construction.forms[0].name.span()),
                        feature: feature::Feature::ConcordClass,
                    };
                    resolutions
                        .get(&identifier_key(&construction.name))
                        .and_then(|values| values.get(&place))
                        == Some(&feature::FeatureResolution::External)
                });
        if has_external_verb {
            concord_class_contextual.insert(path_name(&construction.category));
        }
    }

    loop {
        let before = concord_class_contextual.len();
        let contextual_sums = contextual_sums(&concord_class_contextual);
        for construction in &constructions {
            let passes_external_to_child = construction
                .forms
                .iter()
                .flat_map(|form| &form.atoms)
                .any(|atom| {
                    let FormAtom::Role(role) = atom else { return false };
                    let Some(category) = construction.element.fields.iter().find_map(|field| {
                        same_identifier(&field.name, role)
                            .then_some(&field.kind)
                            .and_then(|kind| match kind {
                                FieldKind::Category(category) => Some(path_name(category)),
                                FieldKind::Lex(_)
                                | FieldKind::Identity(_)
                                | FieldKind::Zeroable { .. }
                                | FieldKind::Optional(_)
                                | FieldKind::Sequence { .. } => None,
                            })
                    }) else {
                        return false;
                    };
                    let place = feature::FeaturePlace::Role {
                        field: role.clone(),
                        feature: feature::Feature::ConcordClass,
                    };
                    concord_class_contextual.contains(&category)
                        && construction.equations.iter().any(|equation| {
                            matches!(
                                &equation.target,
                                ParsedFeaturePlace::Role {
                                    field,
                                    feature: ParsedFeature::ConcordClass,
                                } if same_identifier(field, role)
                            )
                        })
                        && resolutions
                            .get(&identifier_key(&construction.name))
                            .and_then(|values| values.get(&place))
                            == Some(&feature::FeatureResolution::External)
                });
            let relays_external_sequence = construction.equations.iter().any(|equation| {
                let (
                    ParsedFeaturePlace::Construction(ParsedFeature::ConcordClass),
                    ParsedFeatureValue::FromRole(source),
                ) = (&equation.target, &equation.value)
                else {
                    return false;
                };
                source.feature == ParsedFeature::ConcordClass
                    && construction.element.fields.iter().any(|field| {
                        same_identifier(&field.name, &source.role)
                            && matches!(
                                &field.kind,
                                FieldKind::Sequence { item, .. }
                                    if matches!(item.as_ref(), FieldKind::Category(path) if {
                                        let item = path_name(path);
                                        concord_class_contextual.contains(&item)
                                            || contextual_sums.contains(&item)
                                    })
                            )
                    })
            });
            let relays_external_role = construction.equations.iter().any(|equation| {
                let (
                    ParsedFeaturePlace::Construction(ParsedFeature::ConcordClass),
                    ParsedFeatureValue::FromRole(source),
                ) = (&equation.target, &equation.value)
                else {
                    return false;
                };
                source.feature == ParsedFeature::ConcordClass
                    && construction.element.fields.iter().any(|field| {
                        same_identifier(&field.name, &source.role)
                            && matches!(&field.kind, FieldKind::Category(path) if {
                                let source = path_name(path);
                                concord_class_contextual.contains(&source)
                                    || contextual_sums.contains(&source)
                            })
                    })
            });
            if passes_external_to_child || relays_external_role || relays_external_sequence {
                concord_class_contextual.insert(path_name(&construction.category));
            }
        }
        if concord_class_contextual.len() == before {
            break;
        }
    }

    let mut context_required = HashSet::new();
    let structural_names = raw
        .declarations
        .iter()
        .filter_map(|declaration| match declaration {
            Declaration::AbstractProduct(product) => Some(identifier_key(&product.name)),
            Declaration::AbstractSum(sum) => Some(identifier_key(&sum.name)),
            Declaration::Construction(_)
            | Declaration::Vocab(_)
            | Declaration::Morphology(_)
            | Declaration::Lexeme(_)
            | Declaration::Codec(_)
            | Declaration::Identity(_)
            | Declaration::Root(_) => None,
        })
        .collect::<HashSet<_>>();
    loop {
        let before = context_required.len();
        for construction in &constructions {
            let reads_context = construction.forms.iter().any(|form| {
                form.atoms.iter().any(|atom| {
                    let atom = match atom {
                        FormAtom::Bound(bound) => bound.value.as_ref(),
                        atom => atom,
                    };
                    match atom {
                        FormAtom::Identity(_) => true,
                        FormAtom::Role(role) => construction
                            .element
                            .fields
                            .iter()
                            .find(|field| same_identifier(&field.name, role))
                            .is_some_and(|field| match field_kind_leaf(&field.kind) {
                                FieldKind::Category(path) => {
                                    let category = path_name(path);
                                    context_required.contains(&category)
                                        || structural_names.contains(&category)
                                }
                                FieldKind::Lex(_)
                                | FieldKind::Identity(_)
                                | FieldKind::Zeroable { .. }
                                | FieldKind::Optional(_)
                                | FieldKind::Sequence { .. } => false,
                            }),
                        FormAtom::Circumfix(circumfix) => construction
                            .element
                            .fields
                            .iter()
                            .find(|field| same_identifier(&field.name, &circumfix.role))
                            .is_some_and(|field| match field_kind_leaf(&field.kind) {
                                FieldKind::Category(path) => {
                                    context_required.contains(&path_name(path))
                                }
                                FieldKind::Lex(_)
                                | FieldKind::Identity(_)
                                | FieldKind::Zeroable { .. }
                                | FieldKind::Optional(_)
                                | FieldKind::Sequence { .. } => false,
                            }),
                        FormAtom::Marked(marked) => construction
                            .element
                            .fields
                            .iter()
                            .find(|field| same_identifier(&field.name, &marked.role))
                            .is_some_and(|field| match field_kind_leaf(&field.kind) {
                                FieldKind::Category(path) => {
                                    let category = path_name(path);
                                    context_required.contains(&category)
                                        || structural_names.contains(&category)
                                }
                                FieldKind::Lex(_)
                                | FieldKind::Identity(_)
                                | FieldKind::Zeroable { .. }
                                | FieldKind::Optional(_)
                                | FieldKind::Sequence { .. } => false,
                            }),
                        FormAtom::Literal(_)
                        | FormAtom::LicensedLiteral(_)
                        | FormAtom::SentenceInitial(_)
                        | FormAtom::StructuralLiteral(_)
                        | FormAtom::FixedLex(_)
                        | FormAtom::Lex(_)
                        | FormAtom::Verb(_)
                        | FormAtom::OpenVerb(_)
                        | FormAtom::Noun(_) => false,
                        FormAtom::Bound(_) => unreachable!("bound atom values cannot nest"),
                    }
                }) || matches!(&form.guard, FormGuardSource::When(requirement) if guard_reads_realized_category_feature(construction, requirement))
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
            .requires_external_input = concord_class_contextual.contains(&category);
    }
    for (category, capability) in &mut capabilities {
        capability.carries_output =
            providers.contains(&(category.clone(), ParsedFeature::ConcordClass));
        capability.requires_context = context_required.contains(category);
    }
    capabilities
}

fn terminal_is_participle_declaration(raw: &Declarations, terminal: &str) -> bool {
    raw.declarations.iter().any(|declaration| {
        matches!(
            declaration,
            Declaration::Codec(binding)
                if identifier_key(&binding.name) == terminal
                    && matches!(
                        &binding.generated,
                        Some(crate::model::GeneratedCodecRecipe::DeclarationVerb(recipe))
                            if recipe.feature_slots.first().is_some_and(|slot| slot.value == "Participle")
                    )
        )
    })
}

fn guard_reads_realized_category_feature(
    construction: &crate::Construction,
    requirement: &RequireExprSource,
) -> bool {
    match requirement {
        RequireExprSource::In {
            subject:
                RequireSubjectSource::RoleFeature {
                    role,
                    feature: ParsedFeature::Onset | ParsedFeature::PossessiveEnding,
                },
            ..
        } => construction
            .element
            .fields
            .iter()
            .find(|field| same_identifier(&field.name, role))
            .is_some_and(|field| matches!(field.kind, FieldKind::Category(_))),
        RequireExprSource::All(requirements) | RequireExprSource::Any(requirements) => requirements
            .iter()
            .any(|requirement| guard_reads_realized_category_feature(construction, requirement)),
        RequireExprSource::OptionalPresence { .. }
        | RequireExprSource::Length { .. }
        | RequireExprSource::In { .. } => false,
    }
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
                feature: feature::Feature::ConcordClass,
            } if (identifier_key(field) == "verb"
                && !construction
                    .element
                    .fields
                    .iter()
                    .any(|candidate| same_identifier(&candidate.name, field)))
                || construction.forms.iter().flat_map(|form| &form.atoms).any(|atom| {
                    let atom = match atom {
                        FormAtom::Bound(bound) => bound.value.as_ref(),
                        atom => atom,
                    };
                    matches!(atom, FormAtom::Verb(VerbOperand::Projected(role)) if same_identifier(role, field))
                }) =>
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

fn validate_contextual_concord_class_uses(
    raw: &Declarations,
    capabilities: &HashMap<String, CategoryRenderCapability>,
) -> syn::Result<()> {
    let mut errors = None;
    for declaration in &raw.declarations {
        let Declaration::Construction(construction) = declaration else { continue };
        for atom in construction.forms.iter().flat_map(|form| &form.atoms) {
            let FormAtom::Role(role) = atom else { continue };
            let Some(category) = construction.element.fields.iter().find_map(|field| {
                same_identifier(&field.name, role)
                    .then_some(&field.kind)
                    .and_then(|kind| match kind {
                        FieldKind::Category(category) => Some(path_name(category)),
                        FieldKind::Lex(_)
                        | FieldKind::Identity(_)
                        | FieldKind::Zeroable { .. }
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
                        feature: ParsedFeature::ConcordClass,
                    } if same_identifier(field, role)
                )
            });
            let relays_output_from_role = construction.equations.iter().any(|equation| {
                matches!(
                    (&equation.target, &equation.value),
                    (
                        ParsedFeaturePlace::Construction(ParsedFeature::ConcordClass),
                        ParsedFeatureValue::FromRole(source),
                    ) if source.feature == ParsedFeature::ConcordClass
                        && same_identifier(&source.role, role)
                )
            });
            if !has_writer && !relays_output_from_role {
                combine(
                    &mut errors,
                    syn::Error::new(
                        role.span(),
                        format!(
                            "contextual category `{category}` requires a `{role}.concord_class` writer at every bare role use"
                        ),
                    ),
                );
            }
        }
    }
    finish(errors)
}

fn declared_terminal_feature_providers(
    raw: &Declarations,
    providers: &mut HashSet<(String, ParsedFeature)>,
) {
    for declaration in &raw.declarations {
        if let Declaration::Lexeme(lexeme) = declaration {
            for default in &lexeme.feature_defaults {
                providers.insert((identifier_key(&lexeme.name), default.feature));
            }
            let explicit_features = lexeme
                .members
                .iter()
                .flat_map(|member| member.feature_overrides.iter().map(|row| row.feature))
                .collect::<HashSet<_>>();
            for feature in explicit_features {
                if lexeme.members.iter().all(|member| {
                    member
                        .feature_overrides
                        .iter()
                        .any(|row| row.feature == feature)
                }) {
                    providers.insert((identifier_key(&lexeme.name), feature));
                }
            }
        }
        if let Declaration::Vocab(vocab) = declaration {
            for default in &vocab.feature_defaults {
                providers.insert((identifier_key(&vocab.name), default.feature));
            }
            let explicit_features = vocab
                .variants
                .iter()
                .flat_map(|variant| variant.feature_overrides.iter().map(|row| row.feature))
                .collect::<HashSet<_>>();
            for feature in explicit_features {
                if vocab.variants.iter().all(|variant| {
                    variant
                        .feature_overrides
                        .iter()
                        .any(|row| row.feature == feature)
                }) {
                    providers.insert((identifier_key(&vocab.name), feature));
                }
            }
        }
    }
    for declaration in &raw.declarations {
        let Declaration::Codec(codec) = declaration else {
            continue;
        };
        let Some(crate::model::GeneratedCodecRecipe::DeclarationNoun(recipe)) = &codec.generated
        else {
            continue;
        };
        let Some(closed) = recipe.closed_slots.first() else {
            continue;
        };
        let closed_name = identifier_key(&closed.value);
        let Some(Declaration::Lexeme(lexeme)) = raw.declarations.iter().find(|declaration| {
            matches!(declaration, Declaration::Lexeme(lexeme) if identifier_key(&lexeme.name) == closed_name)
        }) else {
            continue;
        };
        for default in &lexeme.feature_defaults {
            providers.insert((identifier_key(&codec.name), default.feature));
        }
        let explicit_features = lexeme
            .members
            .iter()
            .flat_map(|member| member.feature_overrides.iter().map(|row| row.feature))
            .collect::<HashSet<_>>();
        for feature in explicit_features {
            if lexeme.members.iter().all(|member| {
                member
                    .feature_overrides
                    .iter()
                    .any(|row| row.feature == feature)
            }) {
                providers.insert((identifier_key(&codec.name), feature));
            }
        }
    }
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
    declared_terminal_feature_providers(raw, &mut providers);
    for (category, constructions) in categories {
        for feature in [
            ParsedFeature::ConcordClass,
            ParsedFeature::BareDurationLicense,
            ParsedFeature::BareLocativeComplement,
            ParsedFeature::Cardinality,
            ParsedFeature::ModifierLicense,
            ParsedFeature::DeterminerNumber,
            ParsedFeature::FusedHeadLicense,
            ParsedFeature::Focus,
            ParsedFeature::PrepositionComplementKind,
            ParsedFeature::LocativeTemporalLicense,
            ParsedFeature::NominalForm,
            ParsedFeature::NominalLicense,
            ParsedFeature::Number,
            ParsedFeature::Onset,
            ParsedFeature::PossessiveEnding,
            ParsedFeature::PrepositionAttachment,
            ParsedFeature::Relationality,
        ] {
            if constructions.iter().all(|construction| construction.equations.iter().any(|equation| matches!(equation.target, ParsedFeaturePlace::Construction(found) if found == feature))) {
                providers.insert((category.clone(), feature));
            }
        }
    }
    for declaration in &raw.declarations {
        let Declaration::Construction(construction) = declaration else { continue };
        let product = identifier_key(&construction.element.name);
        for equation in &construction.equations {
            if let ParsedFeaturePlace::Construction(feature) = equation.target {
                providers.insert((product.clone(), feature));
            }
        }
    }
    for declaration in &raw.declarations {
        let Declaration::Codec(binding) = declaration else {
            continue;
        };
        if matches!(
            binding.generated,
            Some(crate::model::GeneratedCodecRecipe::EnglishCardinal(_))
        ) {
            let terminal = identifier_key(&binding.name);
            providers.insert((terminal.clone(), ParsedFeature::ConcordClass));
            providers.insert((terminal.clone(), ParsedFeature::DeterminerNumber));
            providers.insert((terminal.clone(), ParsedFeature::Number));
            providers.insert((terminal, ParsedFeature::Cardinality));
        }
        if matches!(
            binding.generated,
            Some(crate::model::GeneratedCodecRecipe::DeclarationDeterminative(_))
        ) {
            let terminal = identifier_key(&binding.name);
            providers.insert((terminal.clone(), ParsedFeature::BareDurationLicense));
            providers.insert((terminal.clone(), ParsedFeature::DeterminerNumber));
            providers.insert((terminal.clone(), ParsedFeature::FusedHeadLicense));
            providers.insert((terminal, ParsedFeature::NominalLicense));
        }
        if let Some(crate::model::GeneratedCodecRecipe::DeclarationNoun(recipe)) =
            &binding.generated
            && let Some(closed) = recipe.closed_slots.first()
            && let Some(lexeme) =
                raw.declarations
                    .iter()
                    .find_map(|declaration| match declaration {
                        Declaration::Lexeme(lexeme)
                            if identifier_key(&lexeme.name) == identifier_key(&closed.value) =>
                        {
                            Some(lexeme)
                        }
                        _ => None,
                    })
        {
            let terminal = identifier_key(&binding.name);
            for default in &lexeme.feature_defaults {
                if matches!(
                    default.feature,
                    ParsedFeature::BareLocativeLicense
                        | ParsedFeature::Compoundability
                        | ParsedFeature::Countability
                        | ParsedFeature::LocativeTemporalLicense
                        | ParsedFeature::Properness
                        | ParsedFeature::Relationality
                ) {
                    providers.insert((terminal.clone(), default.feature));
                }
            }
        }
        if let Some(crate::model::GeneratedCodecRecipe::DeclarationVerb(recipe)) =
            &binding.generated
            && let Some(feature) = recipe.feature_slots.first()
        {
            let feature = match identifier_key(&feature.value).as_str() {
                "ConcordClass" => ParsedFeature::ConcordClass,
                "Participle" => ParsedFeature::Participle,
                _ => continue,
            };
            providers.insert((identifier_key(&binding.name), feature));
        }
    }
    loop {
        let before = providers.len();
        for declaration in &raw.declarations {
            let Declaration::AbstractSum(sum) = declaration else { continue };
            let sum_name = identifier_key(&sum.name);
            for feature in [
                ParsedFeature::ConcordClass,
                ParsedFeature::BareDurationLicense,
                ParsedFeature::ModifierLicense,
                ParsedFeature::DeterminerNumber,
                ParsedFeature::FusedHeadLicense,
                ParsedFeature::Focus,
                ParsedFeature::LocativeTemporalLicense,
                ParsedFeature::NominalForm,
                ParsedFeature::NominalLicense,
                ParsedFeature::Relationality,
            ] {
                if !sum.alternatives.is_empty()
                    && sum.alternatives.iter().all(|alternative| {
                        providers.contains(&(path_name(&alternative.value_type), feature))
                    })
                {
                    providers.insert((sum_name.clone(), feature));
                }
            }
        }
        if providers.len() == before {
            break;
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
        ParsedFeature::ConcordClass => "concord_class",
        ParsedFeature::BareDurationLicense => "bare_duration_license",
        ParsedFeature::BareLocativeLicense => "bare_locative_license",
        ParsedFeature::Cardinality => "cardinality",
        ParsedFeature::Compoundability => "compoundability",
        ParsedFeature::Countability => "countability",
        ParsedFeature::HomographLicense => "homograph_license",
        ParsedFeature::MannerAnaphorClass => "manner_anaphor_class",
        ParsedFeature::ModifierLicense => "modifier_license",
        ParsedFeature::DeterminerNumber => "determiner_number",
        ParsedFeature::FusedHeadLicense => "fused_head_license",
        ParsedFeature::Focus => "focus",
        ParsedFeature::PrepositionComplementKind => "preposition_complement_kind",
        ParsedFeature::LocativeTemporalLicense => "locative_temporal_license",
        ParsedFeature::NominalForm => "nominal_form",
        ParsedFeature::NominalLicense => "nominal_license",
        ParsedFeature::Number => "number",
        ParsedFeature::Onset => "onset",
        ParsedFeature::Participle => "participle",
        ParsedFeature::PossessiveEnding => "possessive_ending",
        ParsedFeature::Properness => "properness",
        ParsedFeature::Relationality => "relationality",
        ParsedFeature::BareLocativeComplement => "bare_locative_complement",
        ParsedFeature::PrepositionAttachment => "preposition_attachment",
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
                    format!("standalone render root `{category}` requires external concord_class"),
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
    let noun_morphologies = raw
        .declarations
        .iter()
        .filter_map(|declaration| match declaration {
            Declaration::Morphology(morphology)
                if identifier_key(&morphology.recipe) == "english_noun" =>
            {
                Some(identifier_key(&morphology.name))
            }
            _ => None,
        })
        .collect::<HashSet<_>>();

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
                noun_atom: noun_morphologies.contains(&identifier_key(&lexeme.morphology)),
                verb_atom: resolved
                    .verb_lexeme_provider
                    .as_ref()
                    .is_some_and(|provider| provider == &identifier_key(&lexeme.name)),
                direct_render: noun_morphologies.contains(&identifier_key(&lexeme.morphology)),
                direct_build: noun_morphologies.contains(&identifier_key(&lexeme.morphology)),
                traversal: true,
            }),
            Declaration::Codec(binding) => terminals.push(TerminalCapabilities {
                name: identifier_key(&binding.name),
                lex_atom: binding.codec_atom == Some(CodecAtomClass::Lex),
                identity_atom: false,
                noun_atom: binding.codec_atom == Some(CodecAtomClass::Noun),
                verb_atom: matches!(
                    binding.generated,
                    Some(crate::model::GeneratedCodecRecipe::DeclarationVerb(_))
                ),
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
        if atoms.len()
            != construction
                .forms
                .iter()
                .map(|form| form.atoms.len())
                .sum::<usize>()
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
                validate_lowerable_feature_compositions(raw, construction, &mut errors);
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

#[expect(
    clippy::match_same_arms,
    clippy::too_many_lines,
    reason = "the finite feature matrix keeps each unsupported feature/value axis explicit"
)]
fn validate_lowerable_feature_compositions(
    raw: &Declarations,
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
        .forms
        .iter()
        .flat_map(|form| &form.atoms)
        .any(|atom| matches!(atom, FormAtom::Noun(_)));
    validate_matched_feature_roles(construction, errors);

    for equation in &construction.equations {
        let lowerable = match (&equation.target, &equation.value) {
            (
                ParsedFeaturePlace::Construction(_),
                ParsedFeatureValue::Constant(_) | ParsedFeatureValue::Match { .. },
            )
            | (
                ParsedFeaturePlace::Construction(
                    ParsedFeature::ConcordClass
                    | ParsedFeature::Onset
                    | ParsedFeature::PossessiveEnding,
                ),
                ParsedFeatureValue::FromRole(_),
            ) => true,
            (
                ParsedFeaturePlace::Construction(
                    ParsedFeature::Cardinality | ParsedFeature::Number,
                ),
                ParsedFeatureValue::FromRole(source),
            ) => match source.feature {
                ParsedFeature::Number => {
                    !has_noun || role_provides_number(raw, &fields, &source.role)
                }
                ParsedFeature::Cardinality => role_provides_cardinality(raw, &fields, &source.role),
                _ => false,
            },
            (
                ParsedFeaturePlace::Construction(
                    ParsedFeature::BareLocativeLicense
                    | ParsedFeature::Compoundability
                    | ParsedFeature::Countability
                    | ParsedFeature::HomographLicense
                    | ParsedFeature::MannerAnaphorClass
                    | ParsedFeature::Properness,
                ),
                _,
            ) => false,
            (
                ParsedFeaturePlace::Construction(
                    ParsedFeature::BareDurationLicense
                    | ParsedFeature::DeterminerNumber
                    | ParsedFeature::FusedHeadLicense
                    | ParsedFeature::Focus
                    | ParsedFeature::PrepositionComplementKind
                    | ParsedFeature::LocativeTemporalLicense
                    | ParsedFeature::NominalForm
                    | ParsedFeature::NominalLicense
                    | ParsedFeature::ModifierLicense
                    | ParsedFeature::BareLocativeComplement
                    | ParsedFeature::PrepositionAttachment
                    | ParsedFeature::Relationality,
                ),
                ParsedFeatureValue::FromRole(source),
            ) => role_feature_is_constructible(
                raw,
                construction,
                &fields,
                &source.role,
                source.feature,
            ),
            (
                ParsedFeaturePlace::Role {
                    field,
                    feature: ParsedFeature::ConcordClass,
                },
                ParsedFeatureValue::Constant(_) | ParsedFeatureValue::FromRole(_),
            ) => {
                identifier_key(field) == "verb" || role_provides_concord_class(raw, &fields, field)
            }
            (ParsedFeaturePlace::Role { field, .. }, ParsedFeatureValue::Match { role, .. }) => {
                same_identifier(field, role)
                    && matches!(fields.get(&identifier_key(field)), Some(FieldKind::Lex(_)))
            }
            (
                ParsedFeaturePlace::Role {
                    feature: ParsedFeature::Cardinality | ParsedFeature::Number,
                    field,
                },
                ParsedFeatureValue::Constant(_) | ParsedFeatureValue::FromRole(_),
            ) => match &equation.target {
                ParsedFeaturePlace::Role {
                    feature: ParsedFeature::Number,
                    ..
                } => role_provides_number(raw, &fields, field),
                ParsedFeaturePlace::Role {
                    feature: ParsedFeature::Cardinality,
                    ..
                } => role_provides_cardinality(raw, &fields, field),
                _ => false,
            },
            (
                ParsedFeaturePlace::Role {
                    feature:
                        ParsedFeature::BareLocativeLicense
                        | ParsedFeature::Compoundability
                        | ParsedFeature::Countability
                        | ParsedFeature::HomographLicense
                        | ParsedFeature::MannerAnaphorClass
                        | ParsedFeature::Properness
                        | ParsedFeature::Relationality,
                    ..
                },
                _,
            ) => false,
            (
                ParsedFeaturePlace::Role {
                    feature: ParsedFeature::Onset | ParsedFeature::PossessiveEnding,
                    ..
                },
                ParsedFeatureValue::Constant(_) | ParsedFeatureValue::FromRole(_),
            ) => false,
            (
                ParsedFeaturePlace::Role {
                    feature:
                        ParsedFeature::BareDurationLicense
                        | ParsedFeature::DeterminerNumber
                        | ParsedFeature::FusedHeadLicense
                        | ParsedFeature::Focus
                        | ParsedFeature::PrepositionComplementKind
                        | ParsedFeature::LocativeTemporalLicense
                        | ParsedFeature::NominalForm
                        | ParsedFeature::NominalLicense
                        | ParsedFeature::ModifierLicense
                        | ParsedFeature::BareLocativeComplement
                        | ParsedFeature::PrepositionAttachment,
                    ..
                },
                ParsedFeatureValue::Constant(_) | ParsedFeatureValue::FromRole(_),
            ) => false,
            (
                ParsedFeaturePlace::Role {
                    field,
                    feature: ParsedFeature::Participle,
                },
                ParsedFeatureValue::Constant(_),
            ) => matches!(fields.get(&identifier_key(field)), Some(FieldKind::Lex(_))),
            (
                ParsedFeaturePlace::Role {
                    feature: ParsedFeature::Participle,
                    ..
                }
                | ParsedFeaturePlace::Construction(ParsedFeature::Participle),
                ParsedFeatureValue::FromRole(_),
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

fn validate_matched_feature_roles(
    construction: &crate::Construction,
    errors: &mut Option<syn::Error>,
) {
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
    let concord_class = matched_role(ParsedFeature::ConcordClass);
    let number = matched_role(ParsedFeature::Number);
    if concord_class
        .zip(number)
        .is_some_and(|(concord_class, number)| !same_identifier(concord_class, number))
    {
        combine(
            errors,
            syn::Error::new(
                construction.name.span(),
                "unimplemented in MVP: `feature equation composition`; construction concord_class and number matches must use the same vocabulary role",
            ),
        );
    }
}

fn role_provides_concord_class(
    raw: &Declarations,
    fields: &HashMap<String, &FieldKind>,
    role: &syn::Ident,
) -> bool {
    match fields.get(&identifier_key(role)) {
        Some(FieldKind::Category(_) | FieldKind::Sequence { .. }) => true,
        Some(FieldKind::Lex(path)) => {
            raw.declarations
                .iter()
                .any(|declaration| match declaration {
                    Declaration::Codec(binding)
                        if identifier_key(&binding.name) == path_name(path)
                            && matches!(
                                binding.generated,
                                Some(crate::model::GeneratedCodecRecipe::DeclarationVerb(_))
                            ) =>
                    {
                        true
                    }
                    Declaration::Lexeme(lexeme)
                        if identifier_key(&lexeme.name) == path_name(path) =>
                    {
                        lexeme_recipe(raw, lexeme)
                            == Some(crate::morphology::MorphologyRecipe::EnglishVerb)
                    }
                    _ => false,
                })
        }
        Some(FieldKind::Identity(_) | FieldKind::Zeroable { .. } | FieldKind::Optional(_))
        | None => false,
    }
}

fn role_provides_number(
    raw: &Declarations,
    fields: &HashMap<String, &FieldKind>,
    role: &syn::Ident,
) -> bool {
    match fields.get(&identifier_key(role)) {
        Some(FieldKind::Zeroable { .. } | FieldKind::Category(_) | FieldKind::Sequence { .. }) => {
            true
        }
        Some(FieldKind::Lex(path)) => {
            raw.declarations
                .iter()
                .any(|declaration| match declaration {
                    Declaration::Codec(binding)
                        if identifier_key(&binding.name) == path_name(path)
                            && matches!(
                                binding.generated,
                                Some(
                                    crate::model::GeneratedCodecRecipe::DeclarationNoun(_)
                                        | crate::model::GeneratedCodecRecipe::EnglishCardinal(_)
                                )
                            ) =>
                    {
                        true
                    }
                    Declaration::Lexeme(lexeme)
                        if identifier_key(&lexeme.name) == path_name(path) =>
                    {
                        lexeme_recipe(raw, lexeme)
                            == Some(crate::morphology::MorphologyRecipe::EnglishNoun)
                    }
                    _ => false,
                })
        }
        Some(FieldKind::Identity(_) | FieldKind::Optional(_)) | None => false,
    }
}

fn role_provides_cardinality(
    raw: &Declarations,
    fields: &HashMap<String, &FieldKind>,
    role: &syn::Ident,
) -> bool {
    match fields.get(&identifier_key(role)) {
        Some(FieldKind::Category(_)) => true,
        Some(FieldKind::Lex(path)) => raw.declarations.iter().any(|declaration| {
            matches!(
                declaration,
                Declaration::Codec(binding)
                    if identifier_key(&binding.name) == path_name(path)
                        && matches!(
                            binding.generated,
                            Some(crate::model::GeneratedCodecRecipe::EnglishCardinal(_))
                        )
            )
        }),
        Some(
            FieldKind::Identity(_)
            | FieldKind::Zeroable { .. }
            | FieldKind::Optional(_)
            | FieldKind::Sequence { .. },
        )
        | None => false,
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
        for feature in [
            ParsedFeature::ConcordClass,
            ParsedFeature::Cardinality,
            ParsedFeature::Focus,
            ParsedFeature::ModifierLicense,
            ParsedFeature::Number,
            ParsedFeature::Onset,
            ParsedFeature::PossessiveEnding,
        ] {
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
        ParsedFeature::ConcordClass => "concord_class",
        ParsedFeature::BareDurationLicense => "bare_duration_license",
        ParsedFeature::BareLocativeLicense => "bare_locative_license",
        ParsedFeature::Cardinality => "cardinality",
        ParsedFeature::Compoundability => "compoundability",
        ParsedFeature::Countability => "countability",
        ParsedFeature::HomographLicense => "homograph_license",
        ParsedFeature::MannerAnaphorClass => "manner_anaphor_class",
        ParsedFeature::ModifierLicense => "modifier_license",
        ParsedFeature::DeterminerNumber => "determiner_number",
        ParsedFeature::FusedHeadLicense => "fused_head_license",
        ParsedFeature::Focus => "focus",
        ParsedFeature::PrepositionComplementKind => "preposition_complement_kind",
        ParsedFeature::LocativeTemporalLicense => "locative_temporal_license",
        ParsedFeature::NominalForm => "nominal_form",
        ParsedFeature::NominalLicense => "nominal_license",
        ParsedFeature::Number => "number",
        ParsedFeature::Onset => "onset",
        ParsedFeature::Participle => "participle",
        ParsedFeature::PossessiveEnding => "possessive_ending",
        ParsedFeature::Properness => "properness",
        ParsedFeature::Relationality => "relationality",
        ParsedFeature::BareLocativeComplement => "bare_locative_complement",
        ParsedFeature::PrepositionAttachment => "preposition_attachment",
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

    #[test]
    fn mobile_roles_are_edge_scope_sibling_or_sequence_adjacent_and_diagnostics_name_the_role() {
        let validated = validate(quote! {
            construction child: Child {
                element ChildNode {}
                form child = "child";
            }
            construction right_edge: Root {
                element RightEdge { prefix: Child, tail: mobile Child, }
                form right_edge = prefix tail;
            }
            construction left_edge: Root {
                element LeftEdge { shared: mobile Child, suffix: Child, }
                form left_edge = shared suffix;
            }
            construction scope_sibling: Root {
                element ScopeSibling {
                    prefix: Child,
                    shared: mobile(body) Child,
                    body: Child,
                }
                form scope_sibling = prefix shared body;
            }
            construction sequence_left: SharedRoot {
                element SequenceLeft {
                    shared: mobile Child,
                    conjuncts: seq Child separated by " ",
                    suffix: Child,
                }
                form sequence_left = shared conjuncts suffix;
            }
            construction sequence_right: OtherRoot {
                element SequenceRight {
                    prefix: Child,
                    conjuncts: seq Child separated by " ",
                    shared: mobile Child,
                    suffix: Child,
                }
                form sequence_right = prefix conjuncts shared suffix;
            }
            root Root { punctuation = "."; eoi = true; standalone_render = true; }
            root SharedRoot { punctuation = "."; eoi = true; standalone_render = true; }
            root OtherRoot { punctuation = "."; eoi = true; standalone_render = true; }
        })
        .expect("edge, scope-sibling, and either-side sequence-adjacent mobile roles validate");
        let scope_sibling = validated
            .semantic
            .constructions()
            .iter()
            .find(|construction| construction.construction_id() == "scope_sibling")
            .expect("scope-sibling construction survives semantic lowering")
            .fields()
            .iter()
            .find(|field| field.name_key() == "shared")
            .expect("mobile field survives semantic lowering");
        assert_eq!(scope_sibling.mobile_scope_sibling(), Some("body"));

        let diagnostic = error(quote! {
            construction child: Child {
                element ChildNode {}
                form child = "child";
            }
            construction invalid: Root {
                element Invalid {
                    prefix: Child,
                    mobile_left: mobile(scope) Child,
                    separator: Child,
                    scope: Child,
                }
                form invalid = prefix mobile_left separator scope;
            }
            root Root { punctuation = "."; eoi = true; standalone_render = true; }
        });
        assert!(
            diagnostic.contains("mobile role `mobile_left` must be at an edge, immediately left of its scope sibling, or adjacent to a `seq` role in form `invalid`"),
            "{diagnostic}",
        );
    }

    #[test]
    fn a_mobile_role_naming_a_scope_sibling_its_element_lacks_is_a_declaration_error() {
        let diagnostic = error(quote! {
            construction child: Child {
                element ChildNode {}
                form child = "child";
            }
            construction absent_sibling: Root {
                element AbsentSibling { shared: mobile(body) Child, suffix: Child, }
                form absent_sibling = shared suffix;
            }
            root Root { punctuation = "."; eoi = true; standalone_render = true; }
        });
        assert!(
            diagnostic.contains("mobile role `shared` names unknown scope sibling `body`"),
            "{diagnostic}",
        );
    }

    #[test]
    fn checked_fields_accept_feature_bearing_lexical_values() {
        validate(quote! {
            codec DeterminativeHead {
                generate declaration_determinative {
                    closed = [
                        Each {
                            bare_duration_license = BareDurationLicensed;
                            number_license = SingularOnly;
                            fused_head_license = FusedHead;
                            nominal_license = CountNominal;
                            realizations = [{ surface = "each"; }];
                        },
                    ];
                }
            }
            construction checked: Root {
                element Checked {
                    head: lex DeterminativeHead
                        checked by determinative_is_fused(head.fused_head_license),
                }
                form checked = lex(head);
            }
            root Root { punctuation = "."; eoi = true; standalone_render = true; }
        })
        .expect("feature-bearing lexical field accepts a build-only check");
    }

    #[test]
    fn declaration_determinatives_require_bare_duration_metadata() {
        let diagnostic = error(quote! {
            codec DeterminativeHead {
                generate declaration_determinative {
                    closed = [
                        Each {
                            number_license = SingularOnly;
                            fused_head_license = FusedHead;
                            nominal_license = CountNominal;
                            realizations = [{ surface = "each"; }];
                        },
                    ];
                }
            }
        });
        assert!(
            diagnostic.contains(
                "declaration_determinative member requires one `bare_duration_license` field"
            ),
            "{diagnostic}",
        );
    }

    #[test]
    fn verb_frame_role_preposition_projection_requires_a_declared_verb_frame() {
        let diagnostic = error(quote! {
            vocab Head { Value = "value", }
            construction object: Object {
                element ObjectValue {}
                form object = "object";
            }
            construction checked: Root {
                element Checked {
                    head: lex Head,
                    object: Object checked by accepts_role_prepositions(
                        head.verb_frame_role_prepositions
                    ),
                }
                form checked = lex(head) object;
            }
            root Root { punctuation = "."; eoi = true; standalone_render = true; }
        });
        assert!(
            diagnostic.contains(
                "verb-frame role-preposition projection requires a declaration-verb codec"
            ),
            "{diagnostic}",
        );
    }

    #[test]
    fn defaultless_lexeme_features_are_exhaustive_declared_data() {
        validate(quote! {
            morphology EnglishNoun {
                feature = Number;
                recipe = english_noun;
            }
            lexeme Head using EnglishNoun {
                First = "first" { feature Relationality = NonRelational; },
                Second = "second" { feature Relationality = QualifiedRelational; },
            }
            construction root: Root { element RootNode {} form root = "root"; }
            root Root { punctuation = "."; eoi = true; standalone_render = true; }
        })
        .expect("an explicit value on every member is a sealed lexical feature");

        let diagnostic = error(quote! {
            morphology EnglishNoun {
                feature = Number;
                recipe = english_noun;
            }
            lexeme Head using EnglishNoun {
                First = "first" { feature Relationality = NonRelational; },
                Second = "second",
            }
            construction root: Root { element RootNode {} form root = "root"; }
            root Root { punctuation = "."; eoi = true; standalone_render = true; }
        });
        assert!(
            diagnostic.contains(
                "lexeme feature `relationality` without a default must be declared on every member"
            ),
            "{diagnostic}"
        );
    }

    #[test]
    fn checked_callback_arguments_seed_build_carriers_without_render_reads() {
        let validated = validate(quote! {
            construction determiner: Determinative {
                element Determiner {}
                derive fused_head_license = Values::FusedHead;
                form determiner = "each";
            }
            construction object: Object {
                element ObjectNode {}
                derive number = Values::Plural;
                form object = "creatures";
            }
            construction partitive: Root {
                element Partitive {
                    head: Determinative checked by accepts_partitive(
                        head.fused_head_license,
                        whole.number,
                    ),
                    whole: Object,
                }
                form partitive = head whole;
            }
            root Root { punctuation = "."; eoi = true; standalone_render = true; }
        })
        .expect("callback companion-feature fixture validates");
        assert!(
            !validated
                .semantic()
                .category_reads_feature("Object", crate::feature::Feature::Number,)
        );
        assert!(validated.semantic().category_carries_number("Object"));
    }

    #[test]
    fn explicit_sum_owned_construction_categories_require_one_exact_element_mapping() {
        validate(quote! {
            abstract sum Choice { Left: LeftNode, Renamed: RightNode, }
            construction left: Choice {
                element LeftNode {}
                form left = "left";
            }
            construction right: Choice {
                element RightNode {}
                form right = "right";
            }
            root Choice { punctuation = "."; eoi = true; standalone_render = true; }
        })
        .expect("the explicit sum is the sole category authority");

        let missing = error(quote! {
            abstract sum Choice { Left: LeftNode, }
            construction left: Choice { element LeftNode {} form left = "left"; }
            construction right: Choice { element RightNode {} form right = "right"; }
            root Choice { punctuation = "."; eoi = true; standalone_render = true; }
        });
        assert!(
            missing.contains("explicit sum `Choice` is missing construction element `RightNode`"),
            "{missing}"
        );

        let duplicate = error(quote! {
            abstract sum Choice { Left: LeftNode, Again: LeftNode, }
            construction left: Choice { element LeftNode {} form left = "left"; }
            root Choice { punctuation = "."; eoi = true; standalone_render = true; }
        });
        assert!(
            duplicate.contains(
                "explicit sum `Choice` maps construction element `LeftNode` more than once"
            ),
            "{duplicate}"
        );

        let foreign = error(quote! {
            abstract sum Choice { Left: LeftNode, Other: ForeignNode, }
            construction left: Choice { element LeftNode {} form left = "left"; }
            root Choice { punctuation = "."; eoi = true; standalone_render = true; }
        });
        assert!(
            foreign.contains(
                "explicit sum `Choice` maps alternative `Other` to foreign construction element `ForeignNode`"
            ),
            "{foreign}"
        );
    }

    #[test]
    fn guarded_forms_reject_category_and_open_domain_subjects() {
        let category = error(quote! {
            vocab Word { One = "one", }
            construction guarded: Cat {
                element Guarded { child: Other, }
                form selected when child is One = child;
                form fallback otherwise = child;
            }
            root Cat { punctuation = "."; eoi = true; standalone_render = true; }
        });
        assert!(
            category.contains("form guard membership requires a vocab role"),
            "{category}"
        );

        let open_domain = error(quote! {
            morphology EnglishVerb { feature = ConcordClass; recipe = english_verb; }
            lexeme Word using EnglishVerb { One = "one", }
            construction guarded: Cat {
                element Guarded { word: lex Word, }
                form selected when word is One = lex(word);
                form fallback otherwise = lex(word);
            }
            root Cat { punctuation = "."; eoi = true; standalone_render = true; }
        });
        assert!(
            open_domain.contains("form guard membership requires a vocab role"),
            "{open_domain}"
        );
    }

    #[test]
    fn optional_vocab_and_presence_requirements_seal_checked_cross_products() {
        validate(quote! {
            vocab Word { That = "that", Those = "those", }
            vocab Mode { One = "one", Two = "two", }
            construction optional: Cat {
                element OptionalValue { word: opt lex Word, mode: lex Mode, }
                require any(
                    all(word.is_none(), mode is One),
                    all(word is That, mode is Two)
                );
                form optional = lex(word) lex(mode);
            }
            root Cat { punctuation = "."; eoi = true; standalone_render = true; }
        })
        .expect("optional absence and optional vocab membership are decidable invariants");
    }

    #[test]
    fn optional_presence_and_vocab_requirements_reject_domain_misuse() {
        let required = error(quote! {
            vocab Word { That = "that", }
            construction required: Cat {
                element RequiredValue { word: lex Word, }
                require word.is_none();
                form required = lex(word);
            }
            root Cat { punctuation = "."; eoi = true; standalone_render = true; }
        });
        assert!(
            required.contains("optional-presence predicate subject `word` is not optional"),
            "{required}",
        );

        let unknown_member = error(quote! {
            vocab Word { That = "that", }
            construction optional: Cat {
                element OptionalValue { word: opt lex Word, }
                require word is Those;
                form optional = lex(word);
            }
            root Cat { punctuation = "."; eoi = true; standalone_render = true; }
        });
        assert!(
            unknown_member.contains("unknown predicate member `Those`"),
            "{unknown_member}",
        );

        let guard_required = error(quote! {
            vocab Word { That = "that", }
            construction required: Cat {
                element RequiredValue { word: lex Word, }
                form absent when word.is_none() = lex(word);
                form present otherwise = lex(word);
            }
            root Cat { punctuation = "."; eoi = true; standalone_render = true; }
        });
        assert!(
            guard_required.contains("optional-presence guard requires an optional role"),
            "{guard_required}",
        );

        let sequence_requirement = error(quote! {
            vocab Word { That = "that", }
            construction sequence: Cat {
                element SequenceValue { words: seq lex Word separated by " ", }
                require words is That;
                form sequence = lex(words);
            }
            root Cat { punctuation = "."; eoi = true; standalone_render = true; }
        });
        assert!(
            sequence_requirement
                .contains("predicate subject `words` is not a category or vocab predicate domain"),
            "{sequence_requirement}",
        );

        let sequence_guard = error(quote! {
            vocab Word { That = "that", }
            construction sequence: Cat {
                element SequenceValue { words: seq lex Word separated by " ", }
                form selected when words is That = lex(words);
                form fallback otherwise = lex(words);
            }
            root Cat { punctuation = "."; eoi = true; standalone_render = true; }
        });
        assert!(
            sequence_guard.contains("form guard membership requires a vocab role"),
            "{sequence_guard}",
        );
    }

    #[test]
    fn optional_vocab_absence_and_membership_share_one_sealed_predicate_domain() {
        let nested = error(quote! {
            vocab Word { That = "that", Those = "those", }
            construction optional: Cat {
                element OptionalValue { word: opt lex Word, }
                require all(word.is_none(), word is That);
                form optional = lex(word);
            }
            root Cat { punctuation = "."; eoi = true; standalone_render = true; }
        });
        assert!(
            nested.contains("empty predicate intersection for `word`"),
            "{nested}",
        );

        let accumulated = error(quote! {
            vocab Word { That = "that", Those = "those", }
            construction optional: Cat {
                element OptionalValue { word: opt lex Word, }
                require word.is_none();
                require word is That;
                form optional = lex(word);
            }
            root Cat { punctuation = "."; eoi = true; standalone_render = true; }
        });
        assert!(
            accumulated.contains("empty predicate intersection for `word`"),
            "{accumulated}",
        );
        assert!(
            accumulated.contains("invariant predicate has no satisfiable alternative"),
            "{accumulated}",
        );
    }

    #[test]
    fn optional_vocab_presence_intersects_absence_and_present_members() {
        validate(quote! {
            vocab Word { That = "that", Those = "those", }
            construction optional: Cat {
                element OptionalValue { word: opt lex Word, }
                require all(word.is_some(), word is That);
                form optional = lex(word);
            }
            root Cat { punctuation = "."; eoi = true; standalone_render = true; }
        })
        .expect("presence intersected with a member retains that present member");

        let contradiction = error(quote! {
            vocab Word { That = "that", Those = "those", }
            construction optional: Cat {
                element OptionalValue { word: opt lex Word, }
                require all(word.is_some(), word.is_none());
                form optional = lex(word);
            }
            root Cat { punctuation = "."; eoi = true; standalone_render = true; }
        });
        assert!(
            contradiction.contains("empty predicate intersection for `word`"),
            "{contradiction}",
        );
    }

    #[test]
    fn bound_atoms_validate_the_fixed_affix_and_right_adjacent_shapes() {
        validate(quote! {
            vocab Modifier { Black = "black", Elf = "Elf", }
            construction owner: Owner {
                element OwnerValue { modifier: lex Modifier, }
                form owner = lex(modifier);
            }
            construction plain_prefix: Root {
                element PlainPrefix { modifier: lex Modifier, }
                form plain_prefix = prefix("non", lex(modifier));
            }
            construction hyphen_prefix: Root {
                element HyphenPrefix { modifier: lex Modifier, }
                form hyphen_prefix = prefix("non-", lex(modifier));
            }
            construction singular_suffix: Root {
                element SingularSuffix { owner: Owner, }
                form singular_suffix = suffix(owner, "'s");
            }
            construction plural_suffix: Root {
                element PluralSuffix { owner: Owner, }
                form plural_suffix = suffix(owner, "'");
            }
            construction fused: Root {
                element Fused { owner: Owner, modifier: lex Modifier, }
                form fused = right_adjacent(owner) lex(modifier);
            }
            root Root { punctuation = "."; eoi = true; standalone_render = true; }
        })
        .expect("validated boundary atoms retain one ordinary value atom");
    }

    #[test]
    fn bound_atoms_reject_empty_whitespace_and_literal_affixes_or_values() {
        let rejected = [
            (
                quote! { prefix("", lex(modifier)) },
                "nonempty fixed byte surface",
            ),
            (
                quote! { prefix("non ", lex(modifier)) },
                "must not contain whitespace",
            ),
            (
                quote! { suffix(lex(modifier), "\t") },
                "must not contain whitespace",
            ),
            (
                quote! { prefix("non", "black") },
                "exactly one ordinary value atom",
            ),
            (
                quote! { right_adjacent("black") },
                "exactly one ordinary value atom",
            ),
        ];

        for (atom, diagnostic) in rejected {
            let actual = error(quote! {
                vocab Modifier { Black = "black", }
                construction invalid: Root {
                    element Invalid { modifier: lex Modifier, }
                    form invalid = #atom;
                }
                root Root { punctuation = "."; eoi = true; standalone_render = true; }
            });
            assert!(actual.contains(diagnostic), "{actual}");
        }
    }

    #[test]
    fn boundary_only_atoms_reject_a_form_final_position() {
        let actual = error(quote! {
            vocab Modifier { Black = "black", }
            construction invalid: Root {
                element Invalid { modifier: lex Modifier, }
                form invalid = right_adjacent(lex(modifier));
            }
            root Root { punctuation = "."; eoi = true; standalone_render = true; }
        });
        assert!(
            actual.contains("must be followed by the material it binds"),
            "{actual}"
        );
    }

    #[test]
    fn bound_atoms_reject_optional_and_sequence_role_values() {
        let rejected = [
            (quote! { maybe: opt Item, }, quote! { prefix("non", maybe) }),
            (quote! { maybe: opt Item, }, quote! { suffix(maybe, "'s") }),
            (quote! { items: seq Item, }, quote! { prefix("non", items) }),
            (quote! { items: seq Item, }, quote! { suffix(items, "'s") }),
            (
                quote! { maybe: opt Item, },
                quote! { right_adjacent(maybe) },
            ),
            (
                quote! { items: seq Item, },
                quote! { right_adjacent(items) },
            ),
        ];

        for (field, atom) in rejected {
            let actual = error(quote! {
                construction item: Item {
                    element ItemValue {}
                    form item = "item";
                }
                construction invalid: Root {
                    element Invalid { #field }
                    form invalid = #atom;
                }
                root Root { punctuation = "."; eoi = true; standalone_render = true; }
            });
            assert!(
                actual.contains("bound atom values must be required, singular fields"),
                "{actual}"
            );
        }
    }

    #[test]
    fn circumfix_atoms_validate_required_and_sequence_category_roles() {
        validate(quote! {
            construction item: Item {
                element ItemValue {}
                form item = "item";
            }
            construction singular: Root {
                element Singular { value: Item, }
                form singular = circumfix("[", value, "]");
            }
            construction sequence: Root {
                element Sequence { values: seq Item separated by "}{", }
                require len(values) >= 1;
                form sequence = circumfix("{", values, "}");
            }
            root Root { punctuation = "."; eoi = true; standalone_render = true; }
        })
        .expect("circumfix roles retain their singular or sequence structural authority");
    }

    #[test]
    fn circumfix_atoms_reject_empty_whitespace_optional_and_lexical_inputs() {
        let affixes = [
            (
                quote! { circumfix("", value, "]") },
                "circumfix prefix must have a nonempty fixed byte surface",
            ),
            (
                quote! { circumfix("[", value, "") },
                "circumfix suffix must have a nonempty fixed byte surface",
            ),
            (
                quote! { circumfix("[ ", value, "]") },
                "circumfix prefix must not contain whitespace",
            ),
            (
                quote! { circumfix("[", value, " ]") },
                "circumfix suffix must not contain whitespace",
            ),
        ];
        for (atom, diagnostic) in affixes {
            let actual = error(quote! {
                construction item: Item {
                    element ItemValue {}
                    form item = "item";
                }
                construction invalid: Root {
                    element Invalid { value: Item, }
                    form invalid = #atom;
                }
                root Root { punctuation = "."; eoi = true; standalone_render = true; }
            });
            assert!(actual.contains(diagnostic), "{actual}");
        }

        let optional = error(quote! {
            construction item: Item {
                element ItemValue {}
                form item = "item";
            }
            construction invalid: Root {
                element Invalid { value: opt Item, }
                form invalid = circumfix("[", value, "]");
            }
            root Root { punctuation = "."; eoi = true; standalone_render = true; }
        });
        assert!(
            optional.contains("circumfix atom values must be required singular or sequence fields"),
            "{optional}",
        );

        let lexical = error(quote! {
            vocab Word { Value = "value", }
            construction invalid: Root {
                element Invalid { value: lex Word, }
                form invalid = circumfix("[", value, "]");
            }
            root Root { punctuation = "."; eoi = true; standalone_render = true; }
        });
        assert!(
            lexical.contains("lexical field `value` used as a category role"),
            "{lexical}",
        );
    }

    #[test]
    fn possessive_ending_is_a_sealed_role_feature_for_guarded_suffix_forms() {
        validate(quote! {
            vocab OwnerWord { Daxos = "Daxos", Players = "players", }
            construction owner: Owner {
                element OwnerValue { value: lex OwnerWord, }
                derive number = match value {
                    Daxos => Values::Singular,
                    Players => Values::Plural,
                };
                derive possessive_ending = value.possessive_ending;
                form owner = lex(value);
            }
            construction possessive: Root {
                element Possessive { owner: Owner, }
                derive number = owner.number;
                form singular when number is Singular = suffix(owner, "'s");
                form plural_s when all(
                    number is Plural,
                    owner.possessive_ending is EndsInS
                ) = suffix(owner, "'");
                form plural_other otherwise = suffix(owner, "'s");
            }
            root Root { punctuation = "."; eoi = true; standalone_render = true; }
        })
        .expect("possessive ending is derived from the owner's realized lexical surface");
    }

    #[test]
    fn guarded_forms_validate_each_form_and_reserve_each_rule_name() {
        validate(quote! {
            vocab Word { One = "one", Two = "two", }
            construction guarded: Cat {
                element Guarded { word: lex Word, }
                form selected when word is One = lex(word);
                form fallback otherwise = lex(word);
            }
            root Cat { punctuation = "."; eoi = true; standalone_render = true; }
        })
        .expect("every guarded form has a validated backend atom inventory");

        let missing_role = error(quote! {
            vocab Word { One = "one", }
            construction guarded: Cat {
                element Guarded { word: lex Word, }
                form selected when word is One = lex(word);
                form fallback otherwise = "fallback";
            }
            root Cat { punctuation = "."; eoi = true; standalone_render = true; }
        });
        assert!(
            missing_role.contains("field `word` is not present in form `fallback`"),
            "{missing_role}"
        );

        let collision = error(quote! {
            vocab Word { One = "one", }
            construction guarded: Cat {
                element Guarded { word: lex Word, }
                form foo_bar when word is One = lex(word);
                form FooBar otherwise = lex(word);
            }
            root Cat { punctuation = "."; eoi = true; standalone_render = true; }
        });
        assert!(
            collision.contains("Construction/RuleId variant"),
            "{collision}"
        );
    }

    #[test]
    fn guarded_forms_keep_a_nullable_fallback_in_zero_width_cycle_checks() {
        let actual = error(quote! {
            construction loop: Cat {
                element Loop { next: opt Cat, }
                form present when next.is_some() = "present" next;
                form absent otherwise = next;
            }
            root Cat { punctuation = "."; eoi = true; standalone_render = true; }
        });
        assert!(
            actual.contains("Loop.next: zero-width recursive cycle"),
            "{actual}"
        );
    }

    #[test]
    fn onset_roles_reject_terminals_without_a_sealed_provider_capability() {
        let custom_identity = error(quote! {
            identity Handle {
                value_type = Handle;
                lexical = Lexical::Handle;
                render = render_handle;
                build { pattern = BuildValue::Handle(handle); construct = handle; }
                traversal {
                    callback = copy;
                    argument = handle;
                    variant Primary;
                }
            }
            construction only: Root {
                element Only { handle: identity Handle, }
                derive onset = handle.onset;
                form only = identity(handle);
            }
            root Root { punctuation = "."; eoi = true; standalone_render = true; }
        });
        assert!(
            custom_identity.contains("terminal `Handle` does not provide onset"),
            "{custom_identity}"
        );
        assert!(!custom_identity.contains("internal"), "{custom_identity}");

        let signed_decimal = error(quote! {
            codec SignedNumber {
                generate signed_decimal {
                    magnitude = u32;
                    sign_type = Sign { Positive = none, Negative = "-", };
                }
            }
            construction only: Root {
                element Only { count: lex SignedNumber, }
                derive onset = count.onset;
                form only = lex(count);
            }
            root Root { punctuation = "."; eoi = true; standalone_render = true; }
        });
        assert!(
            signed_decimal.contains("terminal `SignedNumber` does not provide onset"),
            "{signed_decimal}"
        );
        assert!(!signed_decimal.contains("internal"), "{signed_decimal}");
    }

    #[test]
    fn sentence_initial_rejects_sequence_terminators_with_an_exact_diagnostic() {
        let actual = error(quote! {
            construction item: Item {
                element ItemValue {}
                form item = "item";
            }
            abstract product Items {
                values: seq Item terminated by sentence_initial("; "),
            }
            root Item { punctuation = "."; eoi = true; standalone_render = true; }
        });

        assert_eq!(
            actual,
            ":: core :: compile_error ! { \"Items.values: sentence_initial is supported only on form surfaces and sequence separators\" }",
        );

        let empty = error(quote! {
            construction item: Item {
                element ItemValue {}
                form item = "item";
            }
            abstract product Items {
                values: seq Item separated by sentence_initial(""),
            }
            root Item { punctuation = "."; eoi = true; standalone_render = true; }
        });
        assert_eq!(
            empty,
            ":: core :: compile_error ! { \"Items.values: sentence_initial target must realize at least one byte\" }",
        );

        let continuation_terminator = error(quote! {
            construction item: Item {
                element ItemValue {}
                form item = "item";
            }
            abstract product Items {
                values: seq Item terminated by continuation(" Then "),
            }
            root Item { punctuation = "."; eoi = true; standalone_render = true; }
        });
        assert_eq!(
            continuation_terminator,
            ":: core :: compile_error ! { \"Items.values: continuation is supported only on sequence separators\" }",
        );

        let empty_continuation = error(quote! {
            construction item: Item {
                element ItemValue {}
                form item = "item";
            }
            abstract product Items {
                values: seq Item separated by continuation(""),
            }
            root Item { punctuation = "."; eoi = true; standalone_render = true; }
        });
        assert_eq!(
            empty_continuation,
            ":: core :: compile_error ! { \"Items.values: continuation target must realize at least one byte\" }",
        );

        let neutral_continuation = crate::validate_declarations(
            crate::parse_declarations(quote! {
            construction item: Item {
                element ItemValue {}
                form item = "item";
            }
            abstract product Items {
                values: seq Item separated by continuation(" ↦ "),
            }
            root Item { punctuation = "."; eoi = true; standalone_render = true; }
            })
            .expect("neutral continuation fixture parses"),
        );
        assert!(
            neutral_continuation.is_ok(),
            "continuation is a generic structural case transition: {neutral_continuation:?}",
        );
    }

    #[test]
    fn implicit_verb_onset_requires_the_same_provider_in_every_form() {
        let missing_source = quote! {
            morphology EnglishVerb { feature = ConcordClass; recipe = english_verb; }
            lexeme Verbs using EnglishVerb { Act = "act", }
            vocab Mode { WithVerb = "with", WithoutVerb = "without", }
            construction guarded: Root {
                element Guarded { mode: lex Mode, }
                derive verb.concord_class = Values::Other;
                derive onset = verb.onset;
                form with_verb when mode is WithVerb = lex(mode) verb(Verbs::Act);
                form without_verb otherwise = lex(mode);
            }
            root Root { punctuation = "."; eoi = true; standalone_render = true; }
        };
        let parsed = crate::parse_declarations(missing_source)
            .expect("partial multi-form verb onset fixture parses");
        let Declaration::Construction(construction) = &parsed.declarations[3] else {
            panic!("fourth declaration is the guarded construction")
        };
        let expected_span = construction.forms[1].name.span();
        let missing = crate::validate_declarations(parsed)
            .expect_err("every form must contain the sealed verb onset provider");
        assert_same_span(missing.span(), expected_span);
        let missing = missing.to_string();
        assert!(
            missing
                .contains("verb onset requires exactly one sealed terminal provider in every form"),
            "{missing}"
        );
        assert!(!missing.contains("internal"), "{missing}");

        crate::validate_declarations(
            crate::parse_declarations(quote! {
                morphology EnglishVerb { feature = ConcordClass; recipe = english_verb; }
                lexeme Verbs using EnglishVerb { Act = "act", }
                vocab Mode { First = "first", Second = "second", }
                construction guarded: Root {
                    element Guarded { mode: lex Mode, }
                    derive verb.concord_class = Values::Other;
                    derive onset = verb.onset;
                    form first when mode is First = lex(mode) verb(Verbs::Act);
                    form second otherwise = lex(mode) verb(Verbs::Act);
                }
                root Root { punctuation = "."; eoi = true; standalone_render = true; }
            })
            .expect("uniform multi-form verb onset fixture parses"),
        )
        .expect("the same verb onset provider is present in every form");
    }

    #[test]
    fn feature_guarded_forms_require_identical_traversal_programs() {
        let actual = error(quote! {
            vocab Word { Artifact = "artifact", Player = "player", }
            construction guarded: Root {
                element Guarded { left: lex Word, right: lex Word, }
                derive onset = left.onset;
                form vowel when left.onset is Vowel = lex(left) lex(right);
                form consonant otherwise = lex(right) lex(left);
            }
            root Root { punctuation = "."; eoi = true; standalone_render = true; }
        });
        assert!(
            actual.contains("feature-guarded forms must have identical traversal programs"),
            "{actual}"
        );
        assert!(!actual.contains("internal"), "{actual}");
    }

    #[test]
    fn guarded_forms_reject_mixed_finite_subjects_for_one_role() {
        let actual = error(quote! {
            vocab Word { Artifact = "artifact", Player = "player", }
            construction guarded: Root {
                element Guarded { word: lex Word, }
                derive onset = word.onset;
                form artifact when word is Artifact = lex(word);
                form vowel when word.onset is Vowel = lex(word);
                form fallback otherwise = lex(word);
            }
            root Root { punctuation = "."; eoi = true; standalone_render = true; }
        });
        assert!(
            actual.contains("form guards cannot mix finite subjects for role `word`"),
            "{actual}"
        );
        assert!(!actual.contains("internal"), "{actual}");
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
                derive concord_class = verb.concord_class;
                derive verb.concord_class = Values::Other;
                form action = verb(VerbLexeme::Deal);
            }
            root Ability { punctuation = "."; eoi = true; standalone_render = true; }
        });

        assert!(actual.contains("unknown morphology `Missing`"), "{actual}");
    }

    #[test]
    fn generated_morphology_rejects_unknown_recipe_and_axis_mismatch() {
        let unknown = error(quote! {
            morphology EnglishVerb { feature = ConcordClass; recipe = conjectural_verb; }
            lexeme VerbLexeme using EnglishVerb { Deal = "deal", }
            construction action: Ability {
                element Action {}
                derive concord_class = verb.concord_class;
                derive verb.concord_class = Values::Other;
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
                derive concord_class = Values::Other;
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
            morphology EnglishVerb { feature = ConcordClass; recipe = english_verb; }
            morphology EnglishVerb { feature = ConcordClass; recipe = english_verb; }
        });
        assert!(
            declarations.contains("duplicate") && declarations.contains("EnglishVerb"),
            "{declarations}"
        );

        let members = error(quote! {
            morphology EnglishVerb { feature = ConcordClass; recipe = english_verb; }
            lexeme VerbLexeme using EnglishVerb { Deal = "deal", Deal = "deal", }
        });
        assert!(
            members.contains("duplicate lexeme member `Deal`"),
            "{members}"
        );

        let overrides = error(quote! {
            morphology EnglishVerb { feature = ConcordClass; recipe = english_verb; }
            lexeme VerbLexeme using EnglishVerb {
                Deal = "deal" { Other = "deal", Other = "deal", },
            }
        });
        assert!(
            overrides.contains("duplicate override `Other`"),
            "{overrides}"
        );
    }

    #[test]
    fn generated_morphology_rejects_unknown_override_feature() {
        let actual = error(quote! {
            morphology EnglishVerb { feature = ConcordClass; recipe = english_verb; }
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
                    morphology EnglishVerb { feature = ConcordClass; recipe = english_verb; }
                    lexeme VerbLexeme using EnglishVerb { Deal = "", }
                },
                "lexeme lemma must not be empty",
            ),
            (
                quote! {
                    morphology EnglishVerb { feature = ConcordClass; recipe = english_verb; }
                    lexeme VerbLexeme using EnglishVerb {
                        Deal = "deal" { Other = "", },
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
            morphology EnglishVerb { feature = ConcordClass; recipe = english_verb; }
            lexeme VerbLexeme using EnglishVerb {
                Deal = "deal",
                Be = "be" {
                    Other = "are",
                    ThirdPersonSingular = "is",
                },
            }
            construction action: Ability {
                element Action {}
                derive concord_class = verb.concord_class;
                derive verb.concord_class = Values::Other;
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
                ("Deal", crate::macro_def::SurfaceFeature::PLAIN, "deal"),
                (
                    "Deal",
                    crate::macro_def::SurfaceFeature::THIRD_PERSON_SINGULAR_PRESENT,
                    "deals",
                ),
                ("Be", crate::macro_def::SurfaceFeature::PLAIN, "are"),
                (
                    "Be",
                    crate::macro_def::SurfaceFeature::THIRD_PERSON_SINGULAR_PRESENT,
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
            morphology EnglishVerb { feature = ConcordClass; recipe = english_verb; }
            lexeme VerbLexeme using EnglishVerb { TwoWords = "unrelated", }
            construction action: Ability {
                element Action {}
                derive concord_class = verb.concord_class;
                derive verb.concord_class = Values::Other;
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
            morphology EnglishVerb { feature = ConcordClass; recipe = english_verb; }
            lexeme VerbLexeme using EnglishVerb {
                Deal = "deal",
                Be = "be" {
                    ThirdPersonSingular = "is",
                    Other = "are",
                },
            }
            construction action: Ability {
                element Action {}
                derive concord_class = verb.concord_class;
                derive verb.concord_class = Values::Other;
                form action = verb(VerbLexeme::Deal);
            }
            root Ability { punctuation = "."; eoi = true; standalone_render = true; }
        })
        .expect("resolved morphology validates")
        .into_semantic();

        let morphology = &semantic.morphologies()[0];
        assert_eq!(morphology.name(), "EnglishVerb");
        assert_eq!(morphology.feature(), crate::Feature::ConcordClass);
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
                (
                    crate::macro_def::SurfaceFeature::THIRD_PERSON_SINGULAR_PRESENT,
                    "is",
                ),
                (crate::macro_def::SurfaceFeature::PLAIN, "are"),
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

    fn unsigned_number_error(recipe: &str, body: &proc_macro2::TokenStream) -> String {
        let recipe = syn::Ident::new(recipe, proc_macro2::Span::call_site());
        error(quote! {
            codec Number {
                generate #recipe { #body }
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

    fn catalog_identity_error(body: &proc_macro2::TokenStream) -> String {
        error(quote! {
            identity CardName {
                generate catalog_identity { #body }
            }
            construction named: Cat {
                element Named { name: identity CardName, }
                form named = identity(name);
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

    fn declaration_term_error(body: &proc_macro2::TokenStream) -> String {
        let raw = crate::parse_declarations(quote! {
            codec Term {
                generate declaration_term { #body }
            }
        });
        match raw.and_then(|raw| super::validate_generated_codecs(&raw)) {
            Ok(()) => panic!("fixture must be invalid"),
            Err(error) => error.into_compile_error().to_string(),
        }
    }

    #[test]
    fn declaration_term_recipe_is_feature_selective_and_category_safe() {
        let raw = crate::parse_declarations(quote! {
            codec KeywordAbility {
                generate declaration_term {
                    position = FixedKeyword;
                    kinds = [KeywordAbility];
                    params = [Cost];
                }
            }
            codec AnyKeywordAbility {
                generate declaration_term {
                    position = FixedKeyword;
                    kinds = [KeywordAbility];
                    params = Any;
                }
            }
            codec QualityKeywordAbility {
                generate declaration_term {
                    position = FixedKeyword;
                    kinds = [KeywordAbility];
                    params = [Quality];
                }
            }
            codec KeywordParticipialAdjective {
                generate declaration_term {
                    position = FixedKeyword;
                    kinds = [KeywordAbility];
                    params = Any;
                    feature = Participle;
                }
            }
            codec KeywordBlockLabel {
                generate declaration_term {
                    position = FixedKeyword;
                    kinds = [KeywordAbility];
                    params = Any;
                    feature = BlockLabel;
                }
            }
            codec FixedTerm {
                generate declaration_term {
                    position = FixedTerm;
                    kinds = [CounterKind, Designation];
                }
            }
        })
        .expect("the exact declaration_term syntax parses");
        super::validate_generated_codecs(&raw)
            .expect("the compatible declaration kinds and positions validate");

        assert!(
            declaration_term_error(&quote! {
                position = FixedTerm;
                kinds = [KeywordAbility];
            })
            .contains("declaration kind `KeywordAbility` is not compatible")
        );
        assert!(
            declaration_term_error(&quote! {
                position = FixedKeyword;
                kinds = [CounterKind];
            })
            .contains("declaration kind `CounterKind` is not compatible")
        );
        assert!(
            declaration_term_error(&quote! {
                position = Noun;
                kinds = [Designation];
            })
            .contains("declaration_term position must be `FixedTerm` or `FixedKeyword`")
        );
        assert!(
            declaration_term_error(&quote! {
                position = FixedKeyword;
                kinds = [KeywordAbility];
                params = [Cost];
                params = [Quality];
            })
            .contains("duplicate declaration_term field `params`")
        );
        assert!(
            declaration_term_error(&quote! {
                position = FixedTerm;
                kinds = [Designation];
                feature = Participle;
            })
            .contains("FixedTerm declaration_term codecs support only the `Fixed` feature")
        );
        assert!(
            declaration_term_error(&quote! {
                position = FixedKeyword;
                kinds = [KeywordAbility];
                params = Every;
                feature = Participle;
            })
            .contains("declaration_term params policy must be `Any`")
        );
        assert!(
            declaration_term_error(&quote! {
                position = FixedKeyword;
                kinds = [KeywordAbility];
                params = [Mystery];
            })
            .contains("unknown parameter type `Mystery`")
        );
        assert!(
            declaration_term_error(&quote! {
                position = FixedKeyword;
                kinds = [KeywordAbility];
                feature = Plural;
            })
            .contains(
                "declaration_term feature must be `Fixed`, `BoundSuffix`, `Participle`, or `BlockLabel`",
            )
        );
    }

    fn declaration_verb_source_error(body: &proc_macro2::TokenStream) -> String {
        let raw = crate::parse_declarations(quote! {
            morphology EnglishVerb { feature = ConcordClass; recipe = english_verb; }
            morphology EnglishNoun { feature = Number; recipe = english_noun; }
            lexeme CoreVerb using EnglishVerb { Destroy = "destroy", }
            lexeme EmptyVerb using EnglishVerb {}
            lexeme Nouns using EnglishNoun { Object = "object", }
            codec Verb {
                generate declaration_verb { #body }
            }
        });
        match raw.and_then(|raw| super::validate_generated_codecs(&raw)) {
            Ok(()) => panic!("fixture must be invalid"),
            Err(error) => error.into_compile_error().to_string(),
        }
    }

    #[test]
    fn declaration_verb_recipe_validation_is_closed_and_structural() {
        let raw = crate::parse_declarations(quote! {
            morphology EnglishVerb { feature = ConcordClass; recipe = english_verb; }
            lexeme CoreVerb using EnglishVerb { Destroy = "destroy", }
            codec Verb {
                generate declaration_verb {
                    closed = CoreVerb;
                    position = Verb;
                    tail = ["with", Amount, ObjectNounPhrase];
                    feature = ConcordClass;
                }
            }
        })
        .expect("the exact declaration_verb syntax parses");
        super::validate_generated_codecs(&raw)
            .expect("the exact declaration_verb source validates");

        let labeled_repetition = crate::parse_declarations(quote! {
            codec SearchForVerb {
                generate declaration_verb {
                    position = Verb;
                    tail = [
                        location: ObjectNounPhrase,
                        "for",
                        sought: ObjectNounPhrase,
                    ];
                    feature = ConcordClass;
                }
            }
        })
        .expect("the labeled repeated-tail syntax parses");
        super::validate_generated_codecs(&labeled_repetition)
            .expect("distinct labels authorize the repeated nonliteral kind");

        let frame_complement_pair = crate::parse_declarations(quote! {
            codec PairVerb {
                generate declaration_verb {
                    position = Verb;
                    tail = [FrameComplementPair];
                    feature = ConcordClass;
                }
            }
        })
        .expect("the structural pair pattern parses");
        super::validate_generated_codecs(&frame_complement_pair)
            .expect("the sole Predicate/ConcordClass pair pattern validates");

        for (body, expected) in [
            (
                quote! {
                    tail = [];
                    feature = ConcordClass;
                },
                "declaration_verb requires one `position` field",
            ),
            (
                quote! {
                    position = Verb;
                    position = Verb;
                    tail = [];
                    feature = ConcordClass;
                },
                "duplicate declaration_verb field `position`",
            ),
            (
                quote! {
                    position = Verb;
                    feature = ConcordClass;
                },
                "declaration_verb requires one `tail` field",
            ),
            (
                quote! {
                    position = Verb;
                    tail = [];
                },
                "declaration_verb requires one `feature` field",
            ),
            (
                quote! {
                    position = Verb;
                    tail = [];
                    feature = ConcordClass;
                    feature = ConcordClass;
                },
                "duplicate declaration_verb field `feature`",
            ),
            (
                quote! {
                    closed = CoreVerb;
                    closed = CoreVerb;
                    position = Verb;
                    tail = [];
                    feature = ConcordClass;
                },
                "duplicate declaration_verb field `closed`",
            ),
            (
                quote! {
                    class = Auxiliary;
                    class = ProVerb;
                    position = Verb;
                    tail = [];
                    feature = ConcordClass;
                },
                "duplicate declaration_verb field `class`",
            ),
            (
                quote! {
                    class = Infinitive;
                    position = Verb;
                    tail = [];
                    feature = ConcordClass;
                },
                "class must be `Predicate`, `Auxiliary`, or `ProVerb`",
            ),
            (
                quote! {
                    closed = Missing;
                    position = Verb;
                    tail = [];
                    feature = ConcordClass;
                },
                "closed branch must name a lexeme declaration",
            ),
            (
                quote! {
                    closed = EmptyVerb;
                    position = Verb;
                    tail = [];
                    feature = ConcordClass;
                },
                "closed branch must have at least one lexeme member",
            ),
            (
                quote! {
                    closed = Nouns;
                    position = Verb;
                    tail = [];
                    feature = ConcordClass;
                },
                "closed branch morphology must match its feature axis",
            ),
            (
                quote! {
                    position = Noun;
                    tail = [];
                    feature = ConcordClass;
                },
                "position must be `Verb`",
            ),
            (
                quote! {
                    position = Verb;
                    kinds = [KeywordAbility];
                    tail = [];
                    feature = ConcordClass;
                },
                "declaration_verb recipe accepts only `closed`, `class`, `position`, `tail`, and `feature` fields",
            ),
            (
                quote! {
                    position = Verb;
                    tail = [""];
                    feature = ConcordClass;
                },
                "tail literals cannot be empty",
            ),
            (
                quote! {
                    position = Verb;
                    tail = [Amount, Amount];
                    feature = ConcordClass;
                },
                "duplicate declaration_verb tail atom `Amount`",
            ),
            (
                quote! {
                    position = Verb;
                    tail = ["with", "with"];
                    feature = ConcordClass;
                },
                "duplicate declaration_verb tail atom",
            ),
            (
                quote! {
                    position = Verb;
                    tail = [location: ObjectNounPhrase, ObjectNounPhrase];
                    feature = ConcordClass;
                },
                "repeated declaration_verb tail atom `ObjectNounPhrase` must label every occurrence",
            ),
            (
                quote! {
                    position = Verb;
                    tail = [location: ObjectNounPhrase, location: ObjectNounPhrase];
                    feature = ConcordClass;
                },
                "duplicate declaration_verb tail label `location`",
            ),
            (
                quote! {
                    position = Verb;
                    tail = [preposition: "for"];
                    feature = ConcordClass;
                },
                "declaration_verb tail labels cannot prefix literals",
            ),
            (
                quote! {
                    position = Verb;
                    tail = [];
                    tail = [Amount];
                    feature = ConcordClass;
                },
                "duplicate declaration_verb field `tail`",
            ),
            (
                quote! {
                    position = Verb;
                    tail = [];
                    feature = Number;
                },
                "feature must be `ConcordClass` or `Participle`",
            ),
            (
                quote! {
                    position = Verb;
                    tail = [];
                    feature = ConcordClass;
                    frame_set = Transitive;
                },
                "recipe accepts only `closed`, `class`, `position`, `tail`, and `feature` fields",
            ),
            (
                quote! {
                    position = Verb;
                    tail = [FrameComplementPair, Amount];
                    feature = ConcordClass;
                },
                "FrameComplementPair must be the sole, required, unlabeled declaration_verb tail pattern",
            ),
            (
                quote! {
                    closed = CoreVerb;
                    position = Verb;
                    tail = [FrameComplementPair];
                    feature = ConcordClass;
                },
                "FrameComplementPair declaration_verb patterns cannot have a closed branch",
            ),
            (
                quote! {
                    class = Auxiliary;
                    position = Verb;
                    tail = [FrameComplementPair];
                    feature = ConcordClass;
                },
                "FrameComplementPair declaration_verb patterns must have Predicate class",
            ),
            (
                quote! {
                    position = Verb;
                    tail = [FrameComplementPair];
                    feature = Participle;
                },
                "FrameComplementPair declaration_verb patterns must have ConcordClass feature",
            ),
        ] {
            let message = declaration_verb_source_error(&body);
            assert!(
                message.contains(expected),
                "expected {expected:?} in {message}"
            );
        }
    }

    #[test]
    fn declaration_verb_class_validation_is_sealed() {
        for (body, expected) in [
            (
                quote! {
                    class = Auxiliary;
                    class = ProVerb;
                    position = Verb;
                    tail = [];
                    feature = ConcordClass;
                },
                "duplicate declaration_verb field `class`",
            ),
            (
                quote! {
                    class = Infinitive;
                    position = Verb;
                    tail = [];
                    feature = ConcordClass;
                },
                "class must be `Predicate`, `Auxiliary`, or `ProVerb`",
            ),
        ] {
            let message = declaration_verb_source_error(&body);
            assert!(
                message.contains(expected),
                "expected {expected:?} in {message}"
            );
        }
    }

    #[test]
    fn declaration_verb_domains_reject_exact_overlap_without_source_order_priority() {
        let overlapping = crate::parse_declarations(quote! {
            codec FirstVerb {
                generate declaration_verb {
                    position = Verb;
                    tail = [Amount];
                    feature = ConcordClass;
                }
            }
            codec LaterVerb {
                generate declaration_verb {
                    position = Verb;
                    tail = [Amount];
                    feature = ConcordClass;
                }
            }
        })
        .expect("overlapping declaration_verb recipes parse");
        let error = super::validate_generated_codecs(&overlapping)
            .expect_err("source order must not resolve an overlapping verb frame")
            .to_string();
        assert!(
            error.contains(
                "declaration_verb domains `FirstVerb` and `LaterVerb` overlap at `Verb/Predicate/[Amount]`"
            ),
            "{error}"
        );

        let label_only_distinct = crate::parse_declarations(quote! {
            codec FirstSearchVerb {
                generate declaration_verb {
                    position = Verb;
                    tail = [location: ObjectNounPhrase, "for", sought: ObjectNounPhrase];
                    feature = ConcordClass;
                }
            }
            codec LaterSearchVerb {
                generate declaration_verb {
                    position = Verb;
                    tail = [source: ObjectNounPhrase, "for", object: ObjectNounPhrase];
                    feature = ConcordClass;
                }
            }
        })
        .expect("label-only-distinct declaration_verb recipes parse");
        let error = super::validate_generated_codecs(&label_only_distinct)
            .expect_err("labels must not distinguish normalized verb-frame domains")
            .to_string();
        assert!(
            error.contains(
                "declaration_verb domains `FirstSearchVerb` and `LaterSearchVerb` overlap at `Verb/Predicate/[ObjectNounPhrase, Literal(\"for\"), ObjectNounPhrase]`"
            ),
            "{error}"
        );

        let disjoint = crate::parse_declarations(quote! {
            codec IntransitiveVerb {
                generate declaration_verb {
                    position = Verb;
                    tail = [];
                    feature = ConcordClass;
                }
            }
            codec MeasureComplementVerb {
                generate declaration_verb {
                    position = Verb;
                    tail = [Amount];
                    feature = ConcordClass;
                }
            }
        })
        .expect("disjoint declaration_verb recipes parse");
        super::validate_generated_codecs(&disjoint)
            .expect("different exact tails are structurally disjoint");

        let class_disjoint = crate::parse_declarations(quote! {
            codec PredicateVerb {
                generate declaration_verb {
                    position = Verb;
                    tail = [];
                    feature = ConcordClass;
                }
            }
            codec AuxiliaryVerb {
                generate declaration_verb {
                    class = Auxiliary;
                    position = Verb;
                    tail = [];
                    feature = ConcordClass;
                }
            }
            codec ProVerb {
                generate declaration_verb {
                    class = ProVerb;
                    position = Verb;
                    tail = [];
                    feature = ConcordClass;
                }
            }
        })
        .expect("sealed declaration_verb classes parse");
        super::validate_generated_codecs(&class_disjoint)
            .expect("identical tails in distinct verb-frame classes do not overlap");
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

        validate(quote! {
            morphology EnglishNoun { feature = Number; recipe = english_noun; }
            codec TypeNoun {
                generate declaration_noun {
                    position = Noun;
                    kinds = [Type];
                    feature = Number;
                }
            }
            construction only: Cat { element Only {} form only = "only"; }
            root Cat { punctuation = "."; eoi = true; standalone_render = true; }
        })
        .expect("a declaration-only noun codec validates without a closed branch");

        for (body, expected) in [
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
                "kinds must be `Type`, `TurnPart`, or `Subtype`",
            ),
            (
                quote! {
                    closed = NounLexeme;
                    position = Noun;
                    kinds = [Subtype(Contraption)];
                    feature = Number;
                },
                "unknown declaration_noun subtype family `Contraption`",
            ),
            (
                quote! {
                    closed = NounLexeme;
                    position = Noun;
                    kinds = [Type, Subtype];
                    feature = ConcordClass;
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
    fn declaration_noun_inventory_combines_contributors_and_role_number_flow() {
        validate(quote! {
            morphology EnglishNoun { feature = Number; recipe = english_noun; }
            lexeme NounLexeme using EnglishNoun { Player = "player", }
            codec Noun {
                generate declaration_noun {
                    closed = NounLexeme;
                    position = Noun;
                    kinds = [Type, Subtype(Creature), TurnPart];
                    feature = Number;
                }
            }
            construction modified: Phrase {
                element Modified {
                    modifier: lex Noun,
                    head: lex Noun,
                }
                derive modifier.number = Values::Singular;
                derive head.number = modifier.number;
                derive number = head.number;
                form modified = noun(modifier) noun(head);
            }
            root Phrase { punctuation = "."; eoi = true; standalone_render = true; }
        })
        .expect("one aggregate noun inventory and role-derived Number validate");
    }

    #[test]
    fn parallel_declaration_noun_inventories_are_rejected() {
        let error = validate(quote! {
            morphology EnglishNoun { feature = Number; recipe = english_noun; }
            lexeme NounLexeme using EnglishNoun { Player = "player", }
            codec TypeNoun {
                generate declaration_noun {
                    closed = NounLexeme;
                    position = Noun;
                    kinds = [Type];
                    feature = Number;
                }
            }
            codec CreatureNoun {
                generate declaration_noun {
                    position = Noun;
                    kinds = [Subtype(Creature)];
                    feature = Number;
                }
            }
            construction only: Cat { element Only {} form only = "only"; }
            root Cat { punctuation = "."; eoi = true; standalone_render = true; }
        })
        .expect_err("parallel topical noun codecs must not recreate multiple inventories");
        assert!(
            error
                .to_string()
                .contains("multiple declaration_noun inventories `TypeNoun` and `CreatureNoun`; combine declaration kinds in one codec"),
            "{error}"
        );
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
    fn catalog_identity_recipe_requires_exactly_one_named_provider() {
        validate(quote! {
            identity CardName {
                generate catalog_identity { provider = CardNames; }
            }
            construction named: Cat {
                element Named { name: identity CardName, }
                form named = identity(name);
            }
            root Cat { punctuation = "."; eoi = true; standalone_render = true; }
        })
        .expect("one named provider validates");

        let missing = catalog_identity_error(&quote! {});
        assert!(
            missing.contains("catalog_identity requires one `provider` field"),
            "{missing}"
        );
        let duplicate = catalog_identity_error(&quote! {
            provider = CardNames;
            provider = AlternateCardNames;
        });
        assert!(
            duplicate.contains("duplicate catalog_identity field `provider`"),
            "{duplicate}"
        );
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
    fn unsigned_numeral_recipe_validation_pins_each_magnitude_boundary() {
        validate(quote! {
            codec CardinalNumber {
                generate english_cardinal { magnitude = u32; }
            }
            codec ScalarNumber {
                generate unsigned_decimal { magnitude = u32; }
            }
            codec NonZeroScalarNumber {
                generate unsigned_decimal { magnitude = NonZeroU32; }
            }
            construction only: Cat { element Only {} form only = "only"; }
            root Cat { punctuation = "."; eoi = true; standalone_render = true; }
        })
        .expect("the sealed unsigned numeral primitives validate");

        for (recipe, unsupported) in [
            (
                "english_cardinal",
                "english_cardinal magnitude must be `u32`",
            ),
            (
                "unsigned_decimal",
                "unsigned_decimal magnitude must be `u32` or `NonZeroU32`",
            ),
        ] {
            let missing = unsigned_number_error(recipe, &quote! {});
            assert!(
                missing.contains(&format!("{recipe} requires one `magnitude` field")),
                "{missing}"
            );
            let unsupported_message = unsigned_number_error(recipe, &quote! { magnitude = u64; });
            assert!(
                unsupported_message.contains(unsupported),
                "{unsupported_message}"
            );
            let duplicate =
                unsigned_number_error(recipe, &quote! { magnitude = u32; magnitude = NonZeroU32; });
            assert!(
                duplicate.contains(&format!("duplicate {recipe} field `magnitude`")),
                "{duplicate}"
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
        let FormAtom::OpenVerb(open) = &construction.forms[0].atoms[0] else {
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
        let FormAtom::OpenVerb(open) = &construction.forms[0].atoms[0] else {
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
    fn multi_form_rule_id_collisions_point_at_each_authored_form() {
        for (first_form, second_form) in [("foo_bar", "FooBar"), ("FooBar", "foo_bar")] {
            let source: proc_macro2::TokenStream = format!(
                r#"
                    vocab Word {{ One = "one", }}
                    construction guarded: Cat {{
                        element Guarded {{ word: lex Word, }}
                        form {first_form} when word is One = lex(word);
                        form {second_form} otherwise = lex(word);
                    }}
                    root Cat {{ punctuation = "."; eoi = true; standalone_render = true; }}
                "#,
            )
            .parse()
            .expect("multi-form RuleId collision declaration syntax");
            let parsed = crate::parse_declarations(source.clone())
                .expect("multi-form collision syntax parses");
            let Declaration::Construction(construction) = &parsed.declarations[1] else {
                panic!("second declaration is the colliding construction")
            };
            let expected_span = construction.forms[1].name.span();
            let error = crate::generate(source)
                .expect_err("case-converted multi-form RuleId variants cannot collide");
            assert_same_span(error.span(), expected_span);
            let message = error.to_string();
            assert!(
                message.contains("RuleId for construction form guarded.foo_bar")
                    && message.contains("RuleId for construction form guarded.FooBar"),
                "{message}"
            );
        }
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
            (
                "RightmostLeaf",
                "fixed generated rightmost-leaf traversal trait",
            ),
            (
                "RightmostLeafCategory",
                "fixed generated rightmost-leaf category trait",
            ),
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
            "ConcordClass",
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
                "rightmost_leaf_is",
                "fixed generated rightmost-leaf predicate",
            ),
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
        crate::generate(nested_source)
            .expect("a body category does not collide with its standalone root renderer");

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

        for (feature, value, display) in [
            ("modifier_license", "Unrestricted", "ModifierLicense"),
            ("determiner_number", "SingularOnly", "DeterminerNumber"),
            ("fused_head_license", "NominalOnly", "FusedHeadLicense"),
            (
                "preposition_complement_kind",
                "UnrestrictedComplement",
                "PrepositionComplementKind",
            ),
            (
                "locative_temporal_license",
                "Unlicensed",
                "LocativeTemporalLicense",
            ),
            ("nominal_form", "BareSingularNoun", "NominalForm"),
            ("nominal_license", "AnyNominal", "NominalLicense"),
            ("onset", "Consonant", "Onset"),
            (
                "preposition_attachment",
                "AdjunctCapable",
                "PrepositionAttachment",
            ),
            ("relationality", "NonRelational", "Relationality"),
        ] {
            let feature = syn::Ident::new(feature, proc_macro2::Span::call_site());
            let value = syn::Ident::new(value, proc_macro2::Span::call_site());
            let helper = syn::Ident::new(
                &format!("{feature}_for_source"),
                proc_macro2::Span::call_site(),
            );
            let provider_helper = crate::generate(quote! {
                construction source: Source {
                    element SourceNode {}
                    derive #feature = Values::#value;
                    form source = "source";
                }
                construction collision: Collision {
                    element #helper {}
                    form collision = "collision";
                }
                root Source { punctuation = "."; eoi = true; standalone_render = true; }
            })
            .expect_err("a latent provider helper participates in the value namespace")
            .to_string();
            assert!(
                provider_helper.contains(&format!("semantic identity `{helper}`"))
                    && provider_helper
                        .contains(&format!("generated {display} helper for category `Source`"))
                    && provider_helper.contains("generated unit element constructor"),
                "{provider_helper}"
            );
            assert!(!provider_helper.contains("internal"), "{provider_helper}");
        }
    }

    #[test]
    fn standalone_root_renderer_does_not_collide_with_a_body_category() {
        let generated = crate::generate(quote! {
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
        })
        .expect("a body category has a distinct renderer from its standalone root");
        let source = generated.tokens().to_string();
        assert!(
            source.contains("fn __deckmaste_construction_internal_render_root_foo"),
            "{source}"
        );
        assert!(source.contains("fn render_foo_body"), "{source}");
    }

    #[test]
    fn authored_identifiers_cannot_enter_the_private_root_renderer_namespace() {
        let error = crate::generate(quote! {
            construction root: Root {
                element __deckmaste_construction_internal_render_root_root {}
                form root = "root";
            }
            root Root { punctuation = "."; eoi = true; standalone_render = true; }
        })
        .expect_err("authored elements cannot collide with compiler-private root renderers")
        .to_string();
        assert!(
            error.contains("reserved compiler-internal namespace")
                && error.contains("__deckmaste_construction_internal_render_root_root"),
            "{error}"
        );
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
            morphology EnglishVerb { feature = ConcordClass; recipe = english_verb; }
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
            morphology EnglishVerb { feature = ConcordClass; recipe = english_verb; }
            lexeme Verbs using EnglishVerb { Be = "be", }
            construction only: Cat {
                element Only { word: lex Word, }
                derive concord_class = Anything::Bare;
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
            morphology EnglishVerb { feature = ConcordClass; recipe = english_verb; }
            lexeme Verbs using EnglishVerb { Be = "be", }
            construction leaf_node: Branch { element LeafNode {} derive concord_class = Anything::Bare; form leaf = verb(Verbs::Be); }
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
                require concord_class is Other;
                form only = "only";
            }
            root Root { punctuation = "."; eoi = true; standalone_render = true; }
        });
        assert!(
            unconstructible.contains(
                "construction predicate `concord_class` has no constructible feature expression"
            ),
            "{unconstructible}"
        );

        let parse_only = error(quote! {
            morphology EnglishVerb { feature = ConcordClass; recipe = english_verb; }
            lexeme Verbs using EnglishVerb { Act = "act", }
            construction only: Root {
                element Only {}
                require verb.concord_class is Other;
                derive concord_class = verb.concord_class;
                form only = verb(Verbs::Act);
            }
            root Root { punctuation = "."; eoi = true; standalone_render = true; }
        });
        assert!(
            parse_only.contains("parse-only morphology feature state `verb.concord_class`"),
            "{parse_only}"
        );
    }

    #[test]
    fn modifier_license_vocab_metadata_flows_through_derivation_and_role_guard() {
        let expansion = crate::generate(quote! {
            vocab Modifiers {
                feature ModifierLicense = Unrestricted;
                Creature = "creature",
                Target = "target" { feature ModifierLicense = LocalDeterminer; },
            }
            construction modifier: Modifier {
                element ModifierNode { modifier: lex Modifiers, }
                require modifier.modifier_license is LocalDeterminer;
                derive modifier_license = modifier.modifier_license;
                form modifier = lex(modifier);
            }
            root Modifier { punctuation = "."; eoi = true; standalone_render = true; }
        })
        .expect("modifier-license fixture generates");
        let emitted = expansion.tokens().to_string();
        assert!(
            emitted.contains("enum ModifierLicense { Unrestricted , LocalDeterminer }"),
            "{emitted}"
        );
        assert!(
            emitted.contains("fn modifier_license_for_modifiers"),
            "{emitted}"
        );
        assert!(
            emitted.contains("Modifiers :: Creature => ModifierLicense :: Unrestricted"),
            "{emitted}"
        );
        assert!(
            emitted.contains("Modifiers :: Target => ModifierLicense :: LocalDeterminer"),
            "{emitted}"
        );
        assert!(
            emitted.contains("matches ! (modifier_license_for_modifiers (modifier) , ModifierLicense :: LocalDeterminer)"),
            "{emitted}"
        );
        assert!(emitted.contains("Lexical :: Modifiers"), "{emitted}");
        assert!(
            emitted.contains("ModifierNode :: try_new (* modifiers)"),
            "{emitted}"
        );
    }

    #[test]
    fn determiner_vocab_metadata_emits_number_and_nominal_license_helpers() {
        let expansion = crate::generate(quote! {
            vocab DeterminerWord {
                feature DeterminerNumber = Both;
                feature NominalLicense = AnyNominal;
                Owner = "owner",
            }
            construction phrase: Phrase {
                element PhraseNode { determiner: lex DeterminerWord, }
                require determiner.determiner_number is Both;
                require determiner.nominal_license is AnyNominal;
                form phrase = lex(determiner);
            }
            root Phrase { punctuation = "."; eoi = true; standalone_render = true; }
        })
        .expect("determiner-vocabulary metadata fixture generates");
        let emitted = expansion.tokens().to_string();
        assert!(
            emitted.contains("fn determiner_number_for_determiner_word"),
            "{emitted}"
        );
        assert!(
            emitted.contains("DeterminerWord :: Owner => DeterminerNumber :: Both"),
            "{emitted}"
        );
        assert!(
            emitted.contains("fn nominal_license_for_determiner_word"),
            "{emitted}"
        );
        assert!(
            emitted.contains("DeterminerWord :: Owner => NominalLicense :: AnyNominal"),
            "{emitted}"
        );
    }

    #[test]
    fn require_rejects_generated_new_name_collisions() {
        let concord_class_role_new_collision = error(quote! {
            vocab Mode { One = "one", Two = "two", }
            construction bare: Child {
                element BareChild {}
                derive concord_class = Values::Other;
                form bare = "bare";
            }
            construction third: Child {
                element ThirdChild {}
                derive concord_class = Values::ThirdPersonSingular;
                form third = "third";
            }
            construction only: Root {
                element Only { mode: lex Mode, r#new: Child, }
                derive r#new.concord_class = mode.concord_class;
                derive mode.concord_class = match mode {
                    One => Values::Other,
                    Two => Values::ThirdPersonSingular,
                };
                form only = lex(mode) r#new;
            }
            root Root { punctuation = "."; eoi = true; standalone_render = true; }
        });
        assert!(
            concord_class_role_new_collision
                .contains("generated construction associated item `new`"),
            "{concord_class_role_new_collision}"
        );

        let transitive_new_collision = error(quote! {
            vocab Mode { One = "one", Two = "two", }
            construction only: Root {
                element Only { r#new: lex Mode, }
                require concord_class is Other;
                derive concord_class = match r#new {
                    One => Values::Other,
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
            morphology EnglishVerb { feature = ConcordClass; recipe = english_verb; }
            lexeme Actions using EnglishVerb { Go = "go", }
            construction only: Cat {
                element Only {}
                derive concord_class = verb.concord_class;
                derive verb.concord_class = Values::Other;
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
            morphology EnglishVerb { feature = ConcordClass; recipe = english_verb; }
            lexeme VerbLexeme using EnglishVerb { Destroy = "destroy", }
            lexeme NounLexeme using EnglishVerb { Player = "player", }
            construction destroy: Cat {
                element Destroy {}
                derive concord_class = verb.concord_class;
                derive verb.concord_class = Values::Other;
                form destroy = verb(VerbLexeme::Destroy);
            }
            construction player: Cat {
                element Player {}
                derive concord_class = verb.concord_class;
                derive verb.concord_class = Values::Other;
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
    fn declaration_verb_frames_can_use_distinct_closed_english_verb_lexicons() {
        let validated = validate(quote! {
            morphology EnglishVerb { feature = ConcordClass; recipe = english_verb; }
            lexeme VerbLexeme using EnglishVerb { Be = "be", }
            lexeme CoreIntransitiveVerb using EnglishVerb { Enter = "enter", }
            lexeme CoreTransitiveVerb using EnglishVerb { Control = "control", }
            lexeme CoreMeasureComplementVerb using EnglishVerb { Draw = "draw", }
            codec IntransitiveVerb {
                generate declaration_verb {
                    closed = CoreIntransitiveVerb;
                    position = Verb;
                    tail = [];
                    feature = ConcordClass;
                }
            }
            codec TransitiveVerb {
                generate declaration_verb {
                    closed = CoreTransitiveVerb;
                    position = Verb;
                    tail = [ObjectNounPhrase];
                    feature = ConcordClass;
                }
            }
            codec MeasureComplementVerb {
                generate declaration_verb {
                    closed = CoreMeasureComplementVerb;
                    position = Verb;
                    tail = [Amount];
                    feature = ConcordClass;
                }
            }
            construction only: Cat {
                element Only {}
                derive concord_class = verb.concord_class;
                derive verb.concord_class = Values::Other;
                form only = verb(VerbLexeme::Be);
            }
            root Cat { punctuation = "."; eoi = true; standalone_render = true; }
        })
        .expect("generated frames may each name a closed English-verb lexicon");
        let semantic = validated.semantic();
        assert_eq!(
            semantic
                .runtime_verb_lexeme()
                .expect("the fixed verb atom retains one global provider")
                .name(),
            "VerbLexeme"
        );
        for (codec, closed) in [
            ("IntransitiveVerb", "CoreIntransitiveVerb"),
            ("TransitiveVerb", "CoreTransitiveVerb"),
            ("MeasureComplementVerb", "CoreMeasureComplementVerb"),
        ] {
            let (_, plan) = semantic
                .runtime_declaration_verb_for(codec)
                .unwrap_or_else(|| panic!("{codec} declaration-verb plan is retained"));
            assert_eq!(
                plan.closed_lexeme()
                    .expect("the frame has a closed lexical source")
                    .to_string(),
                closed
            );
            assert!(
                !semantic
                    .lexeme(closed)
                    .expect("the frame lexicon is planned")
                    .is_verb_provider(),
                "a generated frame lexicon must not replace the fixed verb provider"
            );
        }

        let direct_second_provider = error(quote! {
            morphology EnglishVerb { feature = ConcordClass; recipe = english_verb; }
            lexeme VerbLexeme using EnglishVerb { Be = "be", }
            lexeme CoreIntransitiveVerb using EnglishVerb { Enter = "enter", }
            codec IntransitiveVerb {
                generate declaration_verb {
                    closed = CoreIntransitiveVerb;
                    position = Verb;
                    tail = [];
                    feature = ConcordClass;
                }
            }
            construction be: Cat {
                element Be {}
                derive concord_class = verb.concord_class;
                derive verb.concord_class = Values::Other;
                form be = verb(VerbLexeme::Be);
            }
            construction enter: Cat {
                element Enter {}
                derive concord_class = verb.concord_class;
                derive verb.concord_class = Values::Other;
                form enter = verb(CoreIntransitiveVerb::Enter);
            }
            root Cat { punctuation = "."; eoi = true; standalone_render = true; }
        });
        assert!(
            direct_second_provider.contains("multiple verb lexeme providers")
                && direct_second_provider.contains("VerbLexeme")
                && direct_second_provider.contains("CoreIntransitiveVerb"),
            "{direct_second_provider}"
        );
    }

    #[test]
    fn english_verb_lexicons_must_be_owned_by_a_fixed_atom_or_declaration_frame() {
        let unowned = error(quote! {
            morphology EnglishVerb { feature = ConcordClass; recipe = english_verb; }
            lexeme VerbLexeme using EnglishVerb { Be = "be", }
            lexeme UnownedVerb using EnglishVerb { Drift = "drift", }
            construction be: Cat {
                element Be {}
                derive concord_class = verb.concord_class;
                derive verb.concord_class = Values::Other;
                form be = verb(VerbLexeme::Be);
            }
            root Cat { punctuation = "."; eoi = true; standalone_render = true; }
        });
        assert!(
            unowned.contains("EnglishVerb lexeme `UnownedVerb` is unowned")
                && unowned.contains("fixed verb atom")
                && unowned.contains("declaration_verb closed branch"),
            "{unowned}",
        );
    }

    #[test]
    fn declaration_verb_frame_sets_require_a_construction_consumer() {
        let orphaned = crate::generate(quote! {
            codec WithObjectVerb {
                generate declaration_verb {
                    position = Verb;
                    tail = ["with", object: ObjectNounPhrase];
                    feature = ConcordClass;
                }
            }
            construction only: Cat { element Only {} form only = "only"; }
            root Cat { punctuation = "."; eoi = true; standalone_render = true; }
        })
        .expect_err("consumerless declaration verbs must fail code generation")
        .into_compile_error()
        .to_string();
        assert!(
            orphaned
                .contains("declaration_verb codec `WithObjectVerb` has no construction consumer")
                && orphaned.contains("every declared frame_set must build through the grammar"),
            "{orphaned}",
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
            morphology EnglishVerb { feature = ConcordClass; recipe = english_verb; }
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
                derive concord_class = Anything::Plural;
                derive concord_class = Anything::Bare;
                derive number = match word {
                    One => Anything::Singular,
                    One => Anything::Plural,
                };
                form broken = lex(word) noun(noun) verb(Verbs::Be);
            }
            root Cat { punctuation = "."; eoi = true; standalone_render = true; }
        });
        assert!(message.contains("Concord Class value"), "{message}");
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
                derive person.concord_class = match person {
                    One => Values::Other,
                    Many => Values::ThirdPersonSingular,
                };
                derive concord_class = person.concord_class;
                form valid = lex(person);
            }
            root Root { punctuation = "."; eoi = true; standalone_render = true; }
        })
        .expect("the explicit exhaustive role writer provides concord_class");

        let missing = error(quote! {
            vocab Person { One = "one", Many = "many", }
            construction missing: Root {
                element Missing { person: lex Person, }
                derive concord_class = person.concord_class;
                form missing = lex(person);
            }
            root Root { punctuation = "."; eoi = true; standalone_render = true; }
        });
        assert!(
            missing.contains("does not have an exhaustive local concord_class writer"),
            "{missing}"
        );

        let mismatched = error(quote! {
            vocab Person { One = "one", Many = "many", }
            construction mismatched: Root {
                element Mismatched { person: lex Person, other: lex Person, }
                derive person.concord_class = match other {
                    One => Values::Other,
                    Many => Values::ThirdPersonSingular,
                };
                derive concord_class = person.concord_class;
                form mismatched = lex(person) lex(other);
            }
            root Root { punctuation = "."; eoi = true; standalone_render = true; }
        });
        assert!(
            mismatched.contains("does not have an exhaustive local concord_class writer"),
            "{mismatched}"
        );

        let mixed = error(quote! {
            vocab Person { One = "one", Many = "many", }
            construction mixed: Root {
                element Mixed { person: lex Person, }
                derive person.concord_class = match person {
                    One => Values::Singular,
                    Many => Values::Plural,
                };
                derive concord_class = person.concord_class;
                form mixed = lex(person);
            }
            root Root { punctuation = "."; eoi = true; standalone_render = true; }
        });
        assert!(mixed.contains("is not a Concord Class value"), "{mixed}");
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
                derive concord_class = leaf.concord_class;
                form parent = leaf;
            }
            root Parent { punctuation = "."; eoi = true; standalone_render = true; }
        });
        assert!(
            unbound.contains("does not provide concord_class"),
            "{unbound}"
        );

        let cycle = error(quote! {
            construction leaf_node: Branch { element LeafNode {} derive concord_class = Anything::Bare; form leaf = "leaf"; }
            construction parent_node: Parent {
                element ParentNode { left: Branch, right: Branch, }
                derive left.concord_class = right.concord_class;
                derive right.concord_class = left.concord_class;
                form parent = left right;
            }
            root Parent { punctuation = "."; eoi = true; standalone_render = true; }
        });
        assert!(cycle.contains("feature equation cycle"), "{cycle}");

        let atoms = error(quote! {
            morphology EnglishVerb { feature = ConcordClass; recipe = english_verb; }
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
        assert!(
            atoms.contains("verb atom requires concord_class"),
            "{atoms}"
        );
        assert!(atoms.contains("noun atom requires number"), "{atoms}");
    }

    #[test]
    fn homogeneous_nonempty_category_sequences_support_homogeneous_feature_flow() {
        validate(quote! {
            construction bare: Item {
                element BareItem {}
                derive concord_class = Values::Other;
                form bare = "bare";
            }
            construction third: Item {
                element ThirdItem {}
                derive concord_class = Values::ThirdPersonSingular;
                form third = "third";
            }
            construction inbound: Root {
                element InboundSequence { source: Item, members: seq Item separated by " ", }
                require len(members) >= 2;
                derive members.concord_class = source.concord_class;
                form inbound = source members;
            }
            construction outward: Coordinated {
                element OutwardSequence { members: seq Item separated by " ", }
                require len(members) >= 2;
                derive concord_class = members.concord_class;
                form outward = members;
            }
            construction verified: Root {
                element CheckedSequence { source: Item, coordinated: Coordinated, }
                derive coordinated.concord_class = source.concord_class;
                form verified = source coordinated;
            }
            root Root { punctuation = "."; eoi = true; standalone_render = true; }
        })
        .expect("uniform inbound and homogeneous outward sequence concord_class must validate");

        let number_expansion = crate::generate(quote! {
            construction singular: Item {
                element SingularItem {}
                derive number = Values::Singular;
                derive onset = Values::Consonant;
                form singular = "singular";
            }
            construction plural: Item {
                element PluralItem {}
                derive number = Values::Plural;
                derive onset = Values::Vowel;
                form plural = "plural";
            }
            construction downward: Root {
                element DownwardNumberSequence { source: Item, members: seq Item separated by " ", }
                require len(members) >= 2;
                derive members.number = source.number;
                form downward = source members;
            }
            construction outward: Coordinated {
                element OutwardNumberSequence { members: seq Item separated by " ", }
                require len(members) >= 2;
                derive number = members.number;
                derive onset = members.onset;
                form outward = members;
            }
            construction verified: Root {
                element CheckedNumberSequence { source: Item, coordinated: Coordinated, }
                derive coordinated.number = source.number;
                form verified = source coordinated;
            }
            root Root { punctuation = "."; eoi = true; standalone_render = true; }
        })
        .expect("homogeneous downward and outward sequence Number must validate");
        let number_source = number_expansion
            .items()
            .iter()
            .map(|item| item.tokens.to_string())
            .collect::<Vec<_>>()
            .join("\n");
        for required in [
            "fn number_for_item",
            "OutwardNumberSequenceMembersSequence (Vec < Item > , Number , Onset)",
            "number_for_item (value) ==",
            "item_0_number == item_2_number",
            "item_0_onset",
        ] {
            assert!(
                number_source.contains(required),
                "generated homogeneous Number support is missing {required:?}: {number_source}",
            );
        }

        let empty = error(quote! {
            construction item: Item {
                element ItemValue {}
                derive concord_class = Values::Other;
                form item = "item";
            }
            construction empty: Root {
                element EmptySequence { members: seq Item separated by " ", }
                derive concord_class = members.concord_class;
                form empty = members;
            }
            root Root { punctuation = "."; eoi = true; standalone_render = true; }
        });
        assert!(
            empty.contains(
                "EmptySequence.members: sequence feature concord_class requires a statically nonempty sequence"
            ),
            "{empty}"
        );

        let lexical = error(quote! {
            vocab Word { One = "one", }
            construction lexical: Root {
                element LexicalSequence { members: seq lex Word separated by " ", }
                require len(members) >= 2;
                derive concord_class = members.concord_class;
                form lexical = lex(members);
            }
            root Root { punctuation = "."; eoi = true; standalone_render = true; }
        });
        assert!(
            lexical.contains(
                "LexicalSequence.members: sequence feature concord_class requires feature-bearing category items"
            ),
            "{lexical}"
        );

        let identity = error(quote! {
            identity HandleSpelling {
                generate context {
                    Full => card_name,
                    Abbreviated => abbreviated_card_name,
                    canonical_on_collision = Full;
                }
            }
            construction identity: Root {
                element IdentitySequence {
                    members: seq identity HandleSpelling separated by " ",
                }
                require len(members) >= 2;
                derive concord_class = members.concord_class;
                form identity = identity(members);
            }
            root Root { punctuation = "."; eoi = true; standalone_render = true; }
        });
        assert!(
            identity.contains(
                "IdentitySequence.members: sequence feature concord_class requires feature-bearing category items"
            ),
            "{identity}"
        );

        let non_provider = error(quote! {
            construction item: Item { element ItemValue {} form item = "item"; }
            construction missing: Root {
                element MissingProvider { members: seq Item separated by " ", }
                require len(members) >= 2;
                derive concord_class = members.concord_class;
                form missing = members;
            }
            root Root { punctuation = "."; eoi = true; standalone_render = true; }
        });
        assert!(
            non_provider.contains(
                "MissingProvider.members: category `Item` does not provide concord_class"
            ),
            "{non_provider}"
        );

        let unsupported = error(quote! {
            construction cardinal: Item {
                element CardinalItem {}
                derive cardinality = Values::One;
                form numbered = "item";
            }
            construction unsupported: Root {
                element UnsupportedSequence { members: seq Item separated by " ", }
                require len(members) >= 2;
                derive cardinality = members.cardinality;
                form unsupported = members;
            }
            root Root { punctuation = "."; eoi = true; standalone_render = true; }
        });
        assert!(
            unsupported.contains(
                "UnsupportedSequence.members: sequence feature propagation supports homogeneous concord_class or number, first-member onset, or last-member possessive ending, found cardinality"
            ),
            "{unsupported}"
        );

        validate(quote! {
            construction item: Item {
                element ItemValue {}
                derive concord_class = Values::Other;
                derive number = Values::Singular;
                form item = "item";
            }
            construction mixed: Root {
                element MixedSequence { members: seq Item separated by " ", }
                require len(members) >= 2;
                derive concord_class = members.concord_class;
                derive number = members.number;
                form mixed = members;
            }
            root Root { punctuation = "."; eoi = true; standalone_render = true; }
        })
        .expect("distinct features may each have one equation on one sequence role");

        let duplicate = error(quote! {
            construction item: Item {
                element ItemValue {}
                derive number = Values::Singular;
                form item = "item";
            }
            construction duplicate: Root {
                element DuplicateNumberSequence {
                    source: Item,
                    members: seq Item separated by " ",
                }
                require len(members) >= 2;
                derive number = members.number;
                derive source.number = members.number;
                form duplicate = source members;
            }
            root Root { punctuation = "."; eoi = true; standalone_render = true; }
        });
        assert!(
            duplicate.contains(
                "DuplicateNumberSequence.members: sequence feature number has more than one equation"
            ),
            "{duplicate}"
        );

        let contending_writers = error(quote! {
            construction item: Item {
                element ItemValue {}
                derive number = Values::Singular;
                form item = "item";
            }
            construction contending: Root {
                element ContendingNumberSequence {
                    first: Item,
                    second: Item,
                    members: seq Item separated by " ",
                }
                require len(members) >= 2;
                derive members.number = first.number;
                derive members.number = second.number;
                form contending = first second members;
            }
            root Root { punctuation = "."; eoi = true; standalone_render = true; }
        });
        assert!(
            contending_writers.contains(
                "ContendingNumberSequence.members: sequence feature number has more than one equation"
            ),
            "{contending_writers}"
        );
    }

    #[test]
    fn concord_class_sequences_accept_feature_bearing_sums_without_advertising_other_sum_features()
    {
        validate(quote! {
            construction bare: Item {
                element BareItem {}
                derive concord_class = Values::Other;
                derive number = Values::Singular;
                form bare = "bare";
            }
            abstract sum ConcordClassChoice { Item, }
            construction coordinated: Root {
                element SumSequence { members: seq ConcordClassChoice separated by " ", }
                require len(members) >= 2;
                derive members.concord_class = Values::Other;
                derive concord_class = members.concord_class;
                form coordinated = members;
            }
            root Root { punctuation = "."; eoi = true; standalone_render = true; }
        })
        .expect("concord_class-bearing sums have a real transient carrier and helper");

        let unsupported = error(quote! {
            construction numbered: Item {
                element NumberedItem {}
                derive number = Values::Singular;
                form numbered = "numbered";
            }
            abstract sum NumberedChoice { Item, }
            construction relayed: Root {
                element NumberRelay { choice: NumberedChoice, }
                derive number = choice.number;
                form relayed = choice;
            }
            root Root { punctuation = "."; eoi = true; standalone_render = true; }
        });
        assert!(
            unsupported.contains("category `NumberedChoice` does not provide number"),
            "{unsupported}"
        );
    }

    #[test]
    fn mixed_sum_concord_class_preserves_intrinsic_constraints_and_contextual_relay() {
        let plan = validate(quote! {
            morphology EnglishVerb { feature = ConcordClass; recipe = english_verb; }
            lexeme Verbs using EnglishVerb { Act = "act", }
            vocab Marker { One = "one", Many = "many", }
            construction bare: Item {
                element BareItem {}
                derive concord_class = Values::Other;
                form bare = "bare";
            }
            construction third: Item {
                element ThirdItem {}
                derive concord_class = Values::ThirdPersonSingular;
                form third = "third";
            }
            construction contextual: Contextual {
                element ContextualItem {}
                derive concord_class = verb.concord_class;
                form contextual = verb(Verbs::Act);
            }
            abstract sum MixedChoice { Item, Contextual, }
            construction inbound: InboundRoot {
                element InboundMixed { members: seq MixedChoice separated by " ", }
                require len(members) >= 2;
                derive members.concord_class = Values::Other;
                form inbound = members;
            }
            construction relay: MixedRelay {
                element RelayedMixed { members: seq MixedChoice separated by " ", }
                require len(members) >= 2;
                derive concord_class = members.concord_class;
                form relay = members;
            }
            construction fixed_marker: MixedRelay {
                element CheckedConstant { marker: lex Marker, }
                require marker is One;
                derive concord_class = Values::Other;
                form fixed_marker = lex(marker);
            }
            construction envelope: EnvelopeRoot {
                element Envelope { relay: MixedRelay, }
                derive relay.concord_class = Values::Other;
                form envelope = relay;
            }
            root InboundRoot { punctuation = "."; eoi = true; standalone_render = true; }
            root EnvelopeRoot { punctuation = "."; eoi = true; standalone_render = true; }
        })
        .expect("mixed intrinsic/contextual concord_class has generated constraint authority")
        .into_semantic();

        assert!(plan.sum_requires_external_concord_class("MixedChoice"));
        assert!(plan.sum_carries_concord_class("MixedChoice"));
        assert!(plan.category_requires_external_concord_class("MixedRelay"));
        assert!(plan.category_carries_concord_class("MixedRelay"));
        let envelope = plan
            .constructions()
            .iter()
            .find(|construction| construction.construction_id() == "envelope")
            .expect("envelope construction is sealed");
        assert!(plan.construction_requires_checked_ast(envelope));
        let ast = crate::emit::ast::emit(&plan)
            .expect("required sum concord_class constructor checks emit through the AST emitter");
        assert!(
            ast.iter().any(|item| item
                .tokens
                .to_string()
                .contains("concord_class_matches_for_mixed_choice")),
            "the required sum concord_class helper remains reachable from emitted constructors",
        );
        crate::emit::build::emit(&plan)
            .expect("each checked category variant emits from its own ConcordClass source");
    }

    #[test]
    fn contextual_category_relay_uses_the_enclosing_concord_class_without_an_exact_helper() {
        let plan = validate(quote! {
            morphology EnglishVerb { feature = ConcordClass; recipe = english_verb; }
            lexeme Actions using EnglishVerb { Act = "act", }
            construction contextual: Predicate {
                element ContextualPredicate {}
                derive concord_class = verb.concord_class;
                form contextual = verb(Actions::Act);
            }
            construction adjunct: Predicate {
                element AdjunctPredicate { predicate: Predicate, }
                derive concord_class = predicate.concord_class;
                form adjunct = predicate "again";
            }
            construction envelope: Root {
                element Envelope { predicate: Predicate, }
                derive predicate.concord_class = Values::Other;
                form envelope = predicate;
            }
            root Root { punctuation = "."; eoi = true; standalone_render = true; }
        })
        .expect("a contextual category can relay ConcordClass through a recursive envelope")
        .into_semantic();

        assert!(plan.category_requires_external_concord_class("Predicate"));
        assert!(!plan.category_reads_feature("Predicate", crate::feature::Feature::ConcordClass,));
        crate::emit::render::emit(&plan)
            .expect("contextual ConcordClass does not require an impossible exact feature helper");
    }

    #[test]
    fn validates_metadata_without_type_name_switches_and_preserves_order() {
        let validated = validate(quote! {
            vocab Pointing { Near = "this", Far = "those", }
            morphology EnglishVerb { feature = ConcordClass; recipe = english_verb; }
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
                derive concord_class = Whatever::ThirdPersonSingular;
                derive verb.concord_class = Whatever::ThirdPersonSingular;
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
    fn validates_fixed_verb_slots_and_parent_concord_class_flow() {
        validate(quote! {
            codec Heads {
                atom = noun;
                value_type = Heads;
                lexical = Lexical::Heads;
                render = render_heads;
                build { pattern = BuildValue::Heads(value); construct = value; }
                traversal { callback = borrowed; argument = value; call visitor::visit_heads(borrowed(value)); }
            }
            morphology EnglishVerb { feature = ConcordClass; recipe = english_verb; }
            lexeme Actions using EnglishVerb {
                Destroy = "destroy",
                Be = "be",
                Control = "control",
            }

            construction noun_node: NounPhrase {
                element NounNode { head: lex Heads, }
                derive concord_class = Values::ThirdPersonSingular;
                derive number = Values::Singular;
                form noun_node = noun(head);
            }
            construction count_node: NounPhrase {
                element CountNode { head: lex Heads, }
                derive concord_class = Values::Other;
                derive number = Values::Plural;
                derive verb.concord_class = Values::Other;
                form count_node = noun(head) verb(Actions::Control);
            }
            construction destroy: VerbPhrase {
                element Destroy { object: NounPhrase, }
                derive concord_class = verb.concord_class;
                form destroy = verb(Actions::Destroy) object;
            }
            construction where_clause: Clause {
                element WhereClause { value: NounPhrase, }
                derive verb.concord_class = Values::ThirdPersonSingular;
                form where_clause = "where" verb(Actions::Be) value;
            }
            construction imperative: Sentence {
                element Imperative { predicate: VerbPhrase, }
                derive predicate.concord_class = Values::Other;
                form imperative = predicate;
            }
            construction declarative: Sentence {
                element Declarative { subject: NounPhrase, predicate: VerbPhrase, }
                derive predicate.concord_class = subject.concord_class;
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
    fn rejects_number_reads_from_fixed_verbs_and_unproven_projected_verbs() {
        let fixed = error(quote! {
            morphology EnglishVerb { feature = ConcordClass; recipe = english_verb; }
            lexeme Verbs using EnglishVerb { Be = "be", }
            construction broken: Cat {
                element Broken {}
                derive verb.concord_class = Values::Other;
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
            morphology EnglishVerb { feature = ConcordClass; recipe = english_verb; }
            lexeme Verbs using EnglishVerb { Be = "be", }
            construction broken: Cat {
                element Broken { word: lex Verbs, }
                derive word.concord_class = Values::Other;
                derive number = word.number;
                form broken = verb(word);
            }
            root Cat { punctuation = "."; eoi = true; standalone_render = true; }
        });
        assert!(
            projected
                .contains("verb role `word` must use a ConcordClass- or Participle-aware declaration_verb terminal")
                && projected.contains(
                    "lexical role `word` does not have an exhaustive local number writer"
                ),
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
    fn projected_english_verb_lexeme_remains_rejected() {
        let message = error(quote! {
            morphology EnglishVerb { feature = ConcordClass; recipe = english_verb; }
            lexeme Verbs using EnglishVerb { Be = "be", }
            construction projected: Cat {
                element Projected { word: lex Verbs, }
                derive word.concord_class = Values::Other;
                form projected = verb(word);
            }
            root Cat { punctuation = "."; eoi = true; standalone_render = true; }
        });
        assert!(
            message
                .contains("verb role `word` must use a ConcordClass- or Participle-aware declaration_verb terminal"),
            "{message}"
        );
    }

    #[test]
    fn projected_verb_accepts_only_declaration_backed_concord_class_terminals() {
        let validated = validate(quote! {
            codec TransitiveVerb {
                generate declaration_verb {
                    position = Verb;
                    tail = [ObjectNounPhrase];
                    feature = ConcordClass;
                }
            }
            construction transitive: VerbPhrase {
                element Transitive {
                    head: lex TransitiveVerb,
                    object: Object,
                }
                derive concord_class = head.concord_class;
                form active = verb(head) object;
            }
            construction object: Object { element ObjectValue {} form object = "object"; }
            construction imperative: Sentence {
                element Imperative { predicate: VerbPhrase, }
                derive predicate.concord_class = Values::Other;
                form imperative = predicate;
            }
            root Sentence { punctuation = "."; eoi = true; standalone_render = true; }
        })
        .expect("ConcordClass-aware declaration verbs permit parent-bound projected verb roles");
        let terminal = validated
            .semantic()
            .terminals()
            .iter()
            .find(|terminal| terminal.name() == "TransitiveVerb")
            .expect("the declaration verb terminal is sealed");
        assert!(terminal.supports_verb_atom());

        let non_verb = error(quote! {
            vocab Words { Act = "act", }
            construction projected: Cat {
                element Projected { word: lex Words, }
                form projected = verb(word);
            }
            root Cat { punctuation = "."; eoi = true; standalone_render = true; }
        });
        assert!(
            non_verb
                .contains("verb role `word` must use a ConcordClass- or Participle-aware declaration_verb terminal"),
            "{non_verb}"
        );
    }

    #[test]
    fn declaration_participle_axis_is_sealed_and_requires_matching_closed_morphology() {
        crate::generate(quote! {
            morphology EnglishParticiple {
                feature = Participle;
                recipe = english_participle;
            }
            lexeme ParticipleVerb using EnglishParticiple {
                Deal = "deal" { Participle = "dealt", },
                Turn = "turn",
            }
            codec PassiveHead {
                generate declaration_verb {
                    closed = ParticipleVerb;
                    position = Verb;
                    tail = [ObjectNounPhrase];
                    feature = Participle;
                }
            }
            construction passive: Predicate {
                element Passive { head: lex PassiveHead, }
                derive onset = head.onset;
                form passive = verb(head);
            }
            root Predicate { punctuation = "."; eoi = true; standalone_render = true; }
        })
        .expect("the finite participle terminal projects onset without a stored form tag");

        let mismatch = error(quote! {
            morphology EnglishVerb { feature = ConcordClass; recipe = english_verb; }
            lexeme FiniteVerb using EnglishVerb { Deal = "deal", }
            codec PassiveHead {
                generate declaration_verb {
                    closed = FiniteVerb;
                    position = Verb;
                    tail = [ObjectNounPhrase];
                    feature = Participle;
                }
            }
        });
        assert!(
            mismatch.contains("closed branch morphology must match its feature axis"),
            "{mismatch}"
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
    fn abstract_products_reject_external_concord_class_sums_for_every_field_shape() {
        for (field, role) in [
            (quote! { required: Choice, }, "required"),
            (quote! { optional: opt Choice, }, "optional"),
            (quote! { members: seq Choice separated by " ", }, "members"),
        ] {
            let actual = error(quote! {
                morphology EnglishVerb { feature = ConcordClass; recipe = english_verb; }
                lexeme Verbs using EnglishVerb { Act = "act", }
                construction contextual: Child {
                    element ContextualChild {}
                    derive concord_class = verb.concord_class;
                    form contextual = verb(Verbs::Act);
                }
                abstract sum Choice { Child, }
                abstract product Holder { #field }
                construction anchor: Root {
                    element Anchor {}
                    form anchor = "anchor";
                }
                root Root { punctuation = "."; eoi = true; standalone_render = true; }
            });
            let expected = format!(
                "Holder.{role}: ConcordClass-bearing sum `Choice` requires external concord_class, but abstract products cannot declare feature writers"
            );
            assert!(
                actual.contains(&expected),
                "expected `{expected}` in {actual}"
            );
            assert!(!actual.contains("internal"), "{actual}");
        }
    }

    #[test]
    fn abstract_products_reject_transitively_external_concord_class_sums() {
        let actual = error(quote! {
            morphology EnglishVerb { feature = ConcordClass; recipe = english_verb; }
            lexeme Verbs using EnglishVerb { Act = "act", }
            construction contextual: Child {
                element ContextualChild {}
                derive concord_class = verb.concord_class;
                form contextual = verb(Verbs::Act);
            }
            abstract sum Choice { Inner, }
            abstract sum Inner { Child, }
            abstract product Holder { required: Choice, }
            construction anchor: Root {
                element Anchor {}
                form anchor = "anchor";
            }
            root Root { punctuation = "."; eoi = true; standalone_render = true; }
        });
        let expected = "Holder.required: ConcordClass-bearing sum `Choice` requires external concord_class, but abstract products cannot declare feature writers";
        assert!(
            actual.contains(expected),
            "expected `{expected}` in {actual}"
        );
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
                    morphology EnglishVerb { feature = ConcordClass; recipe = english_verb; }
                    lexeme Verbs using EnglishVerb { Act = "act", }
                    construction only: Root {
                        element Only {}
                        derive concord_class = verb.concord_class;
                        derive verb.concord_class = Values::Other;
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
                        derive concord_class = Values<Item = u8>::Bare;
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
                derive concord_class = r#mode.concord_class;
                derive mode.concord_class = match r#mode {
                    One => Values::Other,
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
            (
                "concord_class",
                quote! { derive concord_class = Values::Other; },
            ),
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
                        derive concord_class = Values::Other;
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
                "concord_class match with constant noun number",
                quote! {
                    vocab Count { One = "one", Many = "many", }
                    #head_binding
                    construction only: Root {
                        element Only { count: lex Count, head: lex Head, }
                        derive concord_class = match count {
                            One => Values::ThirdPersonSingular,
                            Many => Values::Other,
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
                        derive concord_class = Values::Other;
                        derive number = Values::Singular;
                        form first = "first";
                    }
                    construction second: Root {
                        element Second {}
                        derive concord_class = Values::ThirdPersonSingular;
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
                        derive concord_class = match count {
                            One => Values::ThirdPersonSingular,
                            Many => Values::Other,
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
                "feature equation composition",
            ),
            (
                "partial concord_class provider",
                quote! {
                    construction provider: Root {
                        element Provider {}
                        derive concord_class = Values::Other;
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
                        derive concord_class = child.concord_class;
                        derive child.concord_class = Values::Other;
                        form parent = child;
                    }
                    construction child: Child {
                        element ChildNode {}
                        derive concord_class = Values::Other;
                        form child = "child";
                    }
                    construction entry: Entry { element EntryNode {} form entry = "entry"; }
                    root Entry { punctuation = "."; eoi = true; standalone_render = true; }
                },
            ),
            (
                "implicit verb constant to construction",
                quote! {
                    morphology EnglishVerb { feature = ConcordClass; recipe = english_verb; }
                    lexeme Verbs using EnglishVerb { Act = "act", }
                    construction action: Action {
                        element ActionNode {}
                        derive concord_class = verb.concord_class;
                        derive verb.concord_class = Values::Other;
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
                        derive concord_class = child.concord_class;
                        derive child.concord_class = mode.concord_class;
                        derive mode.concord_class = match mode {
                            One => Values::ThirdPersonSingular,
                            Many => Values::Other,
                        };
                        form parent = child lex(mode);
                    }
                    construction bare: Child {
                        element BareChild {}
                        derive concord_class = Values::Other;
                        form bare = "bare";
                    }
                    construction third: Child {
                        element ThirdChild {}
                        derive concord_class = Values::ThirdPersonSingular;
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
                        derive concord_class = right.concord_class;
                        derive right.concord_class = left.concord_class;
                        form pair = right left;
                    }
                    construction bare: Child {
                        element BareChild {}
                        derive concord_class = Values::Other;
                        form bare = "bare";
                    }
                    construction third: Child {
                        element ThirdChild {}
                        derive concord_class = Values::ThirdPersonSingular;
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
            morphology EnglishVerb { feature = ConcordClass; recipe = english_verb; }
            lexeme Verbs using EnglishVerb { Act = "act", }
            construction action: Child {
                element ActionNode {}
                derive concord_class = verb.concord_class;
                form action = verb(Verbs::Act);
            }
            construction parent: Parent {
                element ParentNode { child: Child, }
                form parent = child;
            }
            root Parent { punctuation = "."; eoi = true; standalone_render = true; }
        })
        .expect_err("a contextual category role requires an concord_class writer")
        .to_string();
        assert!(nested.contains("child.concord_class"), "{nested}");
        assert!(nested.contains("contextual category `Child`"), "{nested}");
        assert!(!nested.contains("internal"), "{nested}");

        let root = crate::generate(quote! {
            morphology EnglishVerb { feature = ConcordClass; recipe = english_verb; }
            lexeme Verbs using EnglishVerb { Act = "act", }
            construction action: Child {
                element ActionNode {}
                derive concord_class = verb.concord_class;
                form action = verb(Verbs::Act);
            }
            root Child { punctuation = "."; eoi = true; standalone_render = true; }
        })
        .expect_err("a contextual category cannot be a standalone render root")
        .to_string();
        assert!(root.contains("standalone render root `Child`"), "{root}");
        assert!(root.contains("external concord_class"), "{root}");
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
        assert_eq!(expansion.plan().items().len(), 150);
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
                (
                    "document".to_owned(),
                    "predicate".to_owned(),
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
                (crate::SourceDeclarationKind::Vocab, "Words".to_owned()),
                (crate::SourceDeclarationKind::Lexeme, "Nouns".to_owned()),
                (crate::SourceDeclarationKind::Lexeme, "Verbs".to_owned()),
                (
                    crate::SourceDeclarationKind::Codec,
                    "SignedNumber".to_owned(),
                ),
                (
                    crate::SourceDeclarationKind::Morphology,
                    "EnglishNoun".to_owned(),
                ),
                (
                    crate::SourceDeclarationKind::Morphology,
                    "EnglishVerb".to_owned(),
                ),
                (
                    crate::SourceDeclarationKind::Construction,
                    "leaf".to_owned(),
                ),
                (
                    crate::SourceDeclarationKind::Construction,
                    "chain".to_owned(),
                ),
                (
                    crate::SourceDeclarationKind::Construction,
                    "action".to_owned(),
                ),
                (crate::SourceDeclarationKind::Root, "Action".to_owned()),
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
                "concord_class = verb.concord_class; verb.concord_class = Other".to_owned(),
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
                    "concord_class = match mode { Solo => ThirdPersonSingular, Group => Other }; number = match mode { Solo => Singular, Group => Plural }".to_owned()
                ),
                (
                    "nested".to_owned(),
                    "concord_class = next.concord_class; number = next.number".to_owned()
                ),
                ("action".to_owned(), "concord_class = verb.concord_class".to_owned()),
                ("idle".to_owned(), "concord_class = Other".to_owned()),
                (
                    "document".to_owned(),
                    "predicate.concord_class = subject.concord_class".to_owned()
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
        assert_eq!(expansion.plan().items().len(), 150);
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

        assert!(semantic.category_reads_feature("Expr", crate::feature::Feature::ConcordClass));
        assert!(semantic.category_reads_feature("Expr", crate::feature::Feature::Number));
        assert!(
            !semantic.category_reads_feature("Predicate", crate::feature::Feature::ConcordClass)
        );
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
                    crate::semantic::TerminalPlan::CatalogIdentity(value) =>
                        (value.source_index(), "identity", value.name().to_owned(),),
                    crate::semantic::TerminalPlan::SignedDecimal(value) =>
                        (value.source_index(), "codec", value.codec_name().to_owned(),),
                    crate::semantic::TerminalPlan::UnsignedNumber(value) =>
                        (value.source_index(), "codec", value.codec_name().to_owned(),),
                    crate::semantic::TerminalPlan::DeclarationDeterminative(value) =>
                        (value.source_index(), "codec", value.codec_name().to_owned(),),
                    crate::semantic::TerminalPlan::DeclarationNoun(value) =>
                        (value.source_index(), "codec", value.codec_name().to_owned(),),
                    crate::semantic::TerminalPlan::DeclarationTerm(value) =>
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
                        ("concord_class".to_owned(), "Runtime".to_owned()),
                        ("number".to_owned(), "Runtime".to_owned())
                    ]
                ),
                (
                    "nested".to_owned(),
                    vec![
                        ("concord_class".to_owned(), "Runtime".to_owned()),
                        ("number".to_owned(), "Runtime".to_owned())
                    ]
                ),
                (
                    "action".to_owned(),
                    vec![
                        ("concord_class".to_owned(), "External".to_owned()),
                        ("verb.concord_class".to_owned(), "External".to_owned())
                    ]
                ),
                (
                    "idle".to_owned(),
                    vec![("concord_class".to_owned(), "Known(Other)".to_owned())]
                ),
                ("solo".to_owned(), vec![]),
                (
                    "document".to_owned(),
                    vec![("predicate.concord_class".to_owned(), "Runtime".to_owned())]
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
        assert_eq!(emission.items().len(), 150);
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
