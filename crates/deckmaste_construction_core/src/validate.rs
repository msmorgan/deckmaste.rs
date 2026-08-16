use std::collections::HashMap;
use std::collections::HashSet;

use quote::ToTokens;
use syn::spanned::Spanned;

use crate::feature;
use crate::identifier::key as identifier_key;
use crate::identifier::path_key;
use crate::identifier::same as same_identifier;
use crate::identifier::spelling_key;
use crate::model::CodecAtomClass;
use crate::model::ConstructorArgument;
use crate::model::Declaration;
use crate::model::Declarations;
use crate::model::Feature as ParsedFeature;
use crate::model::FeaturePlace as ParsedFeaturePlace;
use crate::model::FeatureValue as ParsedFeatureValue;
use crate::model::FieldKind;
use crate::model::FormAtom;
use crate::model::TerminalBinding;
use crate::model::TraversalKind;
use crate::model::VerbOperand;
use crate::model::VisitMode;

#[derive(Debug)]
pub struct ValidatedDeclarations {
    raw: Declarations,
    #[allow(
        dead_code,
        reason = "source-order metadata is consumed by Task 4 code generation"
    )]
    declaration_names: Vec<String>,
    #[allow(
        dead_code,
        reason = "boxing metadata is consumed by Task 4 code generation"
    )]
    boxed_fields: HashSet<(String, String)>,
    #[allow(
        dead_code,
        reason = "dynamic-number metadata is consumed by Task 4 code generation"
    )]
    dynamic_numbers: HashSet<String>,
    #[allow(
        dead_code,
        reason = "feature equations are consumed by Task 4 code generation"
    )]
    feature_equations: HashMap<String, Vec<feature::FeatureEquation>>,
    feature_resolutions:
        HashMap<String, HashMap<feature::FeaturePlace, feature::FeatureResolution>>,
    category_agreement: HashMap<String, CategoryAgreementCapability>,
    #[allow(
        dead_code,
        reason = "sealed contribution inventory is consumed by Task 4 code generation"
    )]
    contributions: ContributionInventory,
}

#[derive(Debug, Clone, Copy, Default)]
pub(crate) struct CategoryAgreementCapability {
    carries_output: bool,
    requires_external_input: bool,
}

#[derive(Debug)]
pub(crate) struct ContributionInventory {
    constructions: Vec<ConstructionContribution>,
    terminals: Vec<TerminalContribution>,
    roots: Vec<RootContribution>,
}

#[derive(Debug)]
pub(crate) struct ConstructionContribution {
    construction_id: String,
    element_type: String,
    category: String,
    category_variant: String,
    form: String,
    rule_id: String,
    build_arm: String,
    render_arm: String,
    visitor_method: String,
    walker: String,
    atoms: Vec<AtomContribution>,
    atom_count_complete: bool,
    terminal_capabilities_complete: bool,
}

#[derive(Debug, Clone)]
pub(crate) enum AtomContribution {
    Literal,
    Category { role: String, category: String },
    Lex { role: String, terminal: String },
    Identity { role: String, terminal: String },
    Noun { role: String, terminal: String },
    VerbFixed { terminal: String, variant: String },
}

#[derive(Debug)]
#[allow(
    clippy::struct_excessive_bools,
    reason = "the sealed inventory records independent backend capabilities"
)]
pub(crate) struct TerminalContribution {
    name: String,
    lex_atom: bool,
    identity_atom: bool,
    noun_atom: bool,
    verb_atom: bool,
    direct_render: bool,
    direct_build: bool,
    traversal: bool,
}

#[derive(Debug)]
#[allow(
    dead_code,
    reason = "sealed root contributions are consumed by Task 4 code generation"
)]
pub(crate) struct RootContribution {
    category: String,
    parse_entry: bool,
    render_entry: bool,
}

#[allow(
    dead_code,
    reason = "sealed contribution inventory is consumed by Task 4 code generation"
)]
impl ContributionInventory {
    pub(crate) fn constructions(&self) -> &[ConstructionContribution] {
        &self.constructions
    }

    pub(crate) fn terminals(&self) -> &[TerminalContribution] {
        &self.terminals
    }

    pub(crate) fn roots(&self) -> &[RootContribution] {
        &self.roots
    }
}

#[allow(
    dead_code,
    reason = "sealed construction contributions are consumed by Task 4 code generation"
)]
impl ConstructionContribution {
    pub(crate) fn construction_id(&self) -> &str {
        &self.construction_id
    }

    pub(crate) fn element_type(&self) -> &str {
        &self.element_type
    }

    pub(crate) fn category_variant(&self) -> &str {
        &self.category_variant
    }

    pub(crate) fn category(&self) -> &str {
        &self.category
    }

    pub(crate) fn form(&self) -> &str {
        &self.form
    }

    pub(crate) fn rule_id(&self) -> &str {
        &self.rule_id
    }

    pub(crate) fn build_arm(&self) -> &str {
        &self.build_arm
    }

    pub(crate) fn render_arm(&self) -> &str {
        &self.render_arm
    }

    pub(crate) fn visitor_method(&self) -> &str {
        &self.visitor_method
    }

    pub(crate) fn walker(&self) -> &str {
        &self.walker
    }

    pub(crate) fn atoms(&self) -> &[AtomContribution] {
        &self.atoms
    }

    pub(crate) fn is_complete(&self) -> bool {
        [
            &self.construction_id,
            &self.element_type,
            &self.category,
            &self.category_variant,
            &self.form,
            &self.rule_id,
            &self.build_arm,
            &self.render_arm,
            &self.visitor_method,
            &self.walker,
        ]
        .into_iter()
        .all(|slot| !slot.is_empty())
            && self.atom_count_complete
            && self.terminal_capabilities_complete
            && self.atoms.iter().all(AtomContribution::is_complete)
    }
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
            Self::Literal | Self::Category { .. } => None,
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
        }
    }

    fn is_supported_by(&self, terminals: &HashMap<&str, &TerminalContribution>) -> bool {
        match self {
            Self::Literal | Self::Category { .. } => true,
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
}

#[allow(
    dead_code,
    reason = "sealed terminal contributions are consumed by Task 4 code generation"
)]
impl TerminalContribution {
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

#[allow(
    dead_code,
    reason = "sealed root contributions are consumed by Task 4 code generation"
)]
impl RootContribution {
    pub(crate) fn category(&self) -> &str {
        &self.category
    }

    pub(crate) fn is_parse_entry(&self) -> bool {
        self.parse_entry
    }

    pub(crate) fn is_render_entry(&self) -> bool {
        self.render_entry
    }
}

impl ValidatedDeclarations {
    #[must_use]
    pub fn declaration_count(&self) -> usize {
        self.raw.declarations.len()
    }

    #[allow(
        dead_code,
        reason = "sealed raw declarations are consumed by Task 4 code generation"
    )]
    pub(crate) fn raw(&self) -> &Declarations {
        &self.raw
    }

    #[allow(
        dead_code,
        reason = "sealed feature IR is consumed by Task 4 code generation"
    )]
    pub(crate) fn feature_equations(&self, construction: &str) -> &[feature::FeatureEquation] {
        self.feature_equations
            .get(construction)
            .map_or(&[], Vec::as_slice)
    }

    pub(crate) fn category_carries_agreement(&self, category: &str) -> bool {
        self.category_agreement
            .get(category)
            .is_some_and(|capability| capability.carries_output)
    }

    pub(crate) fn category_requires_external_agreement(&self, category: &str) -> bool {
        self.category_agreement
            .get(category)
            .is_some_and(|capability| capability.requires_external_input)
    }

    pub(crate) fn feature_resolution(
        &self,
        construction: &str,
        place: &feature::FeaturePlace,
    ) -> Option<feature::FeatureResolution> {
        self.feature_resolutions
            .get(construction)
            .and_then(|resolutions| resolutions.get(place))
            .copied()
    }

    #[allow(
        dead_code,
        reason = "sealed boxing metadata is consumed by Task 4 code generation"
    )]
    pub(crate) fn boxed_fields(&self) -> &HashSet<(String, String)> {
        &self.boxed_fields
    }

    #[allow(
        dead_code,
        reason = "sealed number metadata is consumed by Task 4 code generation"
    )]
    pub(crate) fn dynamic_number_constructions(&self) -> &HashSet<String> {
        &self.dynamic_numbers
    }

    #[allow(
        dead_code,
        reason = "sealed contributions are consumed by Task 4 code generation"
    )]
    pub(crate) fn contributions(&self) -> &ContributionInventory {
        &self.contributions
    }

    #[cfg(test)]
    fn declaration_names(&self) -> Vec<&str> {
        self.declaration_names.iter().map(String::as_str).collect()
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
    category_variants: HashMap<String, HashSet<String>>,
    terminals: HashMap<String, TerminalInfo>,
}

#[derive(Debug)]
struct ResolvedGrammar {
    atoms_by_construction: HashMap<String, Vec<AtomContribution>>,
    verb_lexeme_provider: Option<String>,
}

pub(crate) fn validate_declarations(raw: Declarations) -> syn::Result<ValidatedDeclarations> {
    let (symbols, declaration_names) = validate_namespaces(&raw)?;
    let resolved = validate_resolution(&raw, &symbols)?;
    validate_stored_fields(&raw)?;
    validate_bindings_and_checked_metadata(&raw)?;
    validate_refinements(&raw, &symbols)?;
    let (feature_equations, dynamic_numbers) = validate_features(&raw, &symbols)?;
    let feature_resolutions = seal_feature_resolutions(&raw, &feature_equations);
    let category_agreement = seal_category_agreement_capabilities(&raw, &feature_resolutions);
    validate_contextual_agreement_uses(&raw, &category_agreement)?;
    let boxed_fields = validate_category_graph(&raw);
    validate_roots(&raw, &symbols, &category_agreement)?;
    let contributions = validate_backend_completeness(&raw, &resolved)?;
    Ok(ValidatedDeclarations {
        raw,
        declaration_names,
        boxed_fields,
        dynamic_numbers,
        feature_equations,
        feature_resolutions,
        category_agreement,
        contributions,
    })
}

#[allow(
    clippy::too_many_lines,
    reason = "namespace validation accumulates every independent declaration error in source order"
)]
fn validate_namespaces(raw: &Declarations) -> syn::Result<(Symbols, Vec<String>)> {
    let mut errors = None;
    let mut declaration_names = Vec::new();
    let mut source_names: HashMap<String, proc_macro2::Span> = HashMap::new();
    let mut rust_names: HashMap<String, (String, proc_macro2::Span)> = HashMap::new();
    let mut rule_ids: HashMap<String, (String, proc_macro2::Span)> = HashMap::new();
    let mut visitor_names: HashMap<String, (String, proc_macro2::Span)> = HashMap::new();
    let mut categories = HashSet::new();
    let mut category_variants: HashMap<String, HashSet<String>> = HashMap::new();
    let mut terminals = HashMap::new();

    for declaration in &raw.declarations {
        match declaration {
            Declaration::Construction(construction) => {
                let name = identifier_key(&construction.name);
                declaration_names.push(name.clone());
                let category = path_name(&construction.category);
                if categories.insert(category.clone()) {
                    register_rust_name(
                        &mut rust_names,
                        &pascal_case(&category),
                        &format!("category {category}"),
                        construction.category.span(),
                        &mut errors,
                    );
                    register_rust_name(
                        &mut rust_names,
                        &format!("render_{}", snake_case(&category)),
                        &format!("category renderer {category}"),
                        construction.category.span(),
                        &mut errors,
                    );
                }
                duplicate_name(&mut source_names, &name, &construction.name, &mut errors);
                register_rust_name(
                    &mut rust_names,
                    &construction.element.name.to_string(),
                    &format!("element {}", construction.element.name),
                    construction.element.name.span(),
                    &mut errors,
                );
                register_rust_name(
                    &mut rust_names,
                    &format!(
                        "walk_{}",
                        snake_case(&construction.element.name.to_string())
                    ),
                    &format!("construction walker {name}"),
                    construction.element.name.span(),
                    &mut errors,
                );
                register_rust_name(
                    &mut visitor_names,
                    &format!(
                        "visit_{}",
                        snake_case(&construction.element.name.to_string())
                    ),
                    &format!("construction visitor {name}"),
                    construction.element.name.span(),
                    &mut errors,
                );
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
                register_rust_name(
                    &mut rule_ids,
                    &format!("{}{category_variant}", pascal_case(&category)),
                    &format!("RuleId for construction {name}"),
                    construction.name.span(),
                    &mut errors,
                );
                if !category_variants
                    .entry(category.clone())
                    .or_default()
                    .insert(category_variant.clone())
                {
                    combine(
                        &mut errors,
                        syn::Error::new(
                            construction.name.span(),
                            format!(
                                "generated Rust name `{category}::{category_variant}` collides"
                            ),
                        ),
                    );
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
                duplicate_name(&mut source_names, &name, &vocab.name, &mut errors);
                register_rust_name(
                    &mut rust_names,
                    &pascal_case(&name),
                    &format!("vocab {name}"),
                    vocab.name.span(),
                    &mut errors,
                );
                register_rust_name(
                    &mut rust_names,
                    &format!("render_{}", snake_case(&name)),
                    &format!("vocab renderer {name}"),
                    vocab.name.span(),
                    &mut errors,
                );
                let mut variants = HashSet::new();
                let mut variant_order = Vec::new();
                let mut words = HashSet::new();
                for variant in &vocab.variants {
                    let variant_key = identifier_key(&variant.name);
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
            Declaration::Lexeme(lexeme) => {
                let name = identifier_key(&lexeme.name);
                declaration_names.push(name.clone());
                duplicate_name(&mut source_names, &name, &lexeme.name, &mut errors);
                register_rust_name(
                    &mut rust_names,
                    &pascal_case(&name),
                    &format!("lexeme {name}"),
                    lexeme.name.span(),
                    &mut errors,
                );
                let mut variants = HashSet::new();
                let mut variant_order = Vec::new();
                for variant in &lexeme.variants {
                    let variant_key = identifier_key(variant);
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
                duplicate_name(&mut source_names, &name, &binding.name, &mut errors);
                register_rust_name(
                    &mut rust_names,
                    &pascal_case(&name),
                    &format!("binding {name}"),
                    binding.name.span(),
                    &mut errors,
                );
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
            Declaration::Root(root) => declaration_names.push(path_name(&root.category)),
        }
    }

    finish(errors)?;
    Ok((
        Symbols {
            categories,
            category_variants,
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

fn register_rust_name(
    names: &mut HashMap<String, (String, proc_macro2::Span)>,
    generated: &str,
    owner: &str,
    span: proc_macro2::Span,
    errors: &mut Option<syn::Error>,
) {
    validate_generated_rust_ident(generated, owner, span, errors);
    let semantic_generated = spelling_key(generated);
    if let Some((previous, _)) = names.get(&semantic_generated) {
        if previous != owner {
            combine(
                errors,
                syn::Error::new(
                    span,
                    format!(
                        "generated Rust name `{generated}` collides for `{previous}` and `{owner}`"
                    ),
                ),
            );
        }
    } else {
        names.insert(semantic_generated, (owner.to_owned(), span));
    }
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
        let verb_operands: Vec<_> = construction
            .form
            .atoms
            .iter()
            .filter_map(|atom| match atom {
                FormAtom::Verb(operand) => Some(operand),
                _ => None,
            })
            .collect();
        let has_fixed_verb = verb_operands
            .iter()
            .any(|operand| matches!(operand, VerbOperand::Fixed(_)));
        let local_vocab_providers = local_vocab_feature_providers(construction, &fields, symbols);
        if verb_operands.len() > 1 {
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
            let ty = match &field.kind {
                FieldKind::Category(path) => {
                    let name = path_name(path);
                    if !symbols.categories.contains(&name) {
                        combine(
                            &mut errors,
                            syn::Error::new_spanned(path, format!("unknown category `{name}`")),
                        );
                    }
                    continue;
                }
                FieldKind::Lex(path) | FieldKind::Identity(path) => path,
            };
            let name = path_name(ty);
            let expected = if matches!(field.kind, FieldKind::Identity(_)) {
                TerminalKind::Identity
            } else {
                TerminalKind::Codec
            };
            match symbols.terminals.get(&name) {
                None => combine(
                    &mut errors,
                    syn::Error::new_spanned(ty, format!("unknown terminal type `{name}`")),
                ),
                Some(info)
                    if matches!(field.kind, FieldKind::Identity(_)) && info.kind != expected =>
                {
                    combine(
                        &mut errors,
                        syn::Error::new_spanned(
                            ty,
                            format!("terminal `{name}` is not an identity binding"),
                        ),
                    );
                }
                _ => {}
            }
        }
        for atom in &construction.form.atoms {
            match atom {
                FormAtom::Role(role) => check_role_kind(role, &fields, true, &mut errors),
                FormAtom::Lex(role) => check_lex_role(role, &fields, symbols, &mut errors),
                FormAtom::Identity(role) => match fields.get(&identifier_key(role)) {
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

fn reject_projected_verb_role(role: &syn::Ident, errors: &mut Option<syn::Error>) {
    combine(
        errors,
        syn::Error::new(
            role.span(),
            "unimplemented in MVP: `projected verb role`; use a fixed verb path",
        ),
    );
}

fn resolve_grammar_uses(raw: &Declarations) -> syn::Result<ResolvedGrammar> {
    let mut errors = None;
    let mut atoms_by_construction = HashMap::new();
    let mut verb_providers: Vec<(String, proc_macro2::Span)> = Vec::new();
    let mut seen_verb_providers = HashSet::new();

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
                FormAtom::Role(role) => match fields.get(&identifier_key(role)) {
                    Some(FieldKind::Category(path)) => Some(AtomContribution::Category {
                        role: identifier_key(role),
                        category: path_name(path),
                    }),
                    _ => None,
                },
                FormAtom::Lex(role) => match fields.get(&identifier_key(role)) {
                    Some(FieldKind::Lex(path)) => Some(AtomContribution::Lex {
                        role: identifier_key(role),
                        terminal: path_name(path),
                    }),
                    _ => None,
                },
                FormAtom::Identity(role) => match fields.get(&identifier_key(role)) {
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
                    record_verb_provider(
                        &terminal,
                        path.span(),
                        &mut seen_verb_providers,
                        &mut verb_providers,
                    );
                    Some(AtomContribution::VerbFixed { terminal, variant })
                }
                FormAtom::Verb(VerbOperand::Projected(_)) => None,
            };
            if let Some(resolved) = resolved {
                atoms.push(resolved);
            }
        }
        atoms_by_construction.insert(identifier_key(&construction.name), atoms);
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

fn record_verb_provider(
    terminal: &str,
    span: proc_macro2::Span,
    seen: &mut HashSet<String>,
    providers: &mut Vec<(String, proc_macro2::Span)>,
) {
    if seen.insert(terminal.to_owned()) {
        providers.push((terminal.to_owned(), span));
    }
}

fn check_lex_role(
    role: &syn::Ident,
    fields: &HashMap<String, &FieldKind>,
    symbols: &Symbols,
    errors: &mut Option<syn::Error>,
) {
    check_role_kind(role, fields, false, errors);
    if let Some(FieldKind::Lex(path)) = fields.get(&identifier_key(role)) {
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
        Some(FieldKind::Category(_)) | None => return,
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
    match fields.get(&identifier_key(role)) {
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
        Some(FieldKind::Category(_)) | None => return,
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
                FormAtom::Verb(VerbOperand::Fixed(_)) | FormAtom::Literal(_) => None,
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

fn validate_bindings_and_checked_metadata(raw: &Declarations) -> syn::Result<()> {
    let mut errors = None;
    let callbacks = traversal_callbacks(raw, &mut errors);
    for declaration in &raw.declarations {
        match declaration {
            Declaration::Construction(construction) => {
                let Some(checked) = &construction.checked else { continue };
                let fields: HashSet<_> = construction
                    .element
                    .fields
                    .iter()
                    .map(|field| identifier_key(&field.name))
                    .collect();
                let mut visible = HashSet::new();
                for visibility in &checked.visibilities {
                    let name = identifier_key(&visibility.role);
                    if !fields.contains(&name) {
                        combine(
                            &mut errors,
                            syn::Error::new(
                                visibility.role.span(),
                                format!("unknown checked visibility field `{name}`"),
                            ),
                        );
                    } else if !visible.insert(name.clone()) {
                        combine(
                            &mut errors,
                            syn::Error::new(
                                visibility.role.span(),
                                format!("duplicate checked visibility for `{name}`"),
                            ),
                        );
                    }
                }
                for field in &construction.element.fields {
                    if !visible.contains(&identifier_key(&field.name)) {
                        combine(
                            &mut errors,
                            syn::Error::new(
                                field.name.span(),
                                format!(
                                    "checked construction requires explicit visibility for field `{}`",
                                    field.name
                                ),
                            ),
                        );
                    }
                }
                let mut accessor_roles = HashSet::new();
                for accessor in &checked.accessors {
                    let name = identifier_key(&accessor.role);
                    if !fields.contains(&name) {
                        combine(
                            &mut errors,
                            syn::Error::new(
                                accessor.role.span(),
                                format!("unknown checked accessor field `{name}`"),
                            ),
                        );
                    } else if !accessor_roles.insert(name.clone()) {
                        combine(
                            &mut errors,
                            syn::Error::new(
                                accessor.role.span(),
                                format!("duplicate checked accessor for `{name}`"),
                            ),
                        );
                    }
                }
                for visibility in &checked.visibilities {
                    let name = identifier_key(&visibility.role);
                    let is_private = matches!(
                        visibility.visibility,
                        crate::NonPublicVisibility::Private(_)
                    );
                    if is_private && !accessor_roles.contains(&name) {
                        combine(
                            &mut errors,
                            syn::Error::new(
                                visibility.role.span(),
                                format!("private checked field `{name}` requires an accessor"),
                            ),
                        );
                    } else if !is_private && accessor_roles.contains(&name) {
                        combine(
                            &mut errors,
                            syn::Error::new(
                                visibility.role.span(),
                                format!(
                                    "non-private checked field `{name}` cannot declare an accessor"
                                ),
                            ),
                        );
                    }
                }
                let mut arguments = HashSet::new();
                for argument in &checked.constructor.arguments {
                    let role = match argument {
                        ConstructorArgument::Role(role)
                        | ConstructorArgument::VecRole { role, .. } => Some(role),
                        ConstructorArgument::Context(_) => None,
                    };
                    if let Some(role) = role {
                        let name = identifier_key(role);
                        if !fields.contains(&name) {
                            combine(
                                &mut errors,
                                syn::Error::new(
                                    role.span(),
                                    format!("unknown constructor argument field `{name}`"),
                                ),
                            );
                        } else if !arguments.insert(name.clone()) {
                            combine(
                                &mut errors,
                                syn::Error::new(
                                    role.span(),
                                    format!("constructor argument `{name}` is repeated"),
                                ),
                            );
                        }
                    }
                }
                for field in &construction.element.fields {
                    if !arguments.contains(&identifier_key(&field.name)) {
                        combine(
                            &mut errors,
                            syn::Error::new(
                                field.name.span(),
                                format!(
                                    "checked constructor does not receive field `{}`",
                                    field.name
                                ),
                            ),
                        );
                    }
                }
            }
            Declaration::Codec(binding) | Declaration::Identity(binding) => {
                validate_binding(binding, &callbacks, &mut errors);
            }
            _ => {}
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
                if let Some(mode) = binding.traversal.callback_mode {
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
            Declaration::Root(_) => {}
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

fn validate_refinements(raw: &Declarations, symbols: &Symbols) -> syn::Result<()> {
    let mut errors = None;
    for declaration in &raw.declarations {
        let Declaration::Construction(construction) = declaration else { continue };
        let fields: HashMap<_, _> = construction
            .element
            .fields
            .iter()
            .map(|field| (identifier_key(&field.name), &field.kind))
            .collect();
        let mut refined = HashSet::new();
        for requirement in &construction.requirements {
            let role = identifier_key(&requirement.role);
            if !refined.insert(role.clone()) {
                combine(
                    &mut errors,
                    syn::Error::new(
                        requirement.role.span(),
                        format!("duplicate refinement for role `{role}`"),
                    ),
                );
            }
            let variant = identifier_key(&requirement.variant);
            match fields.get(&role) {
                None => combine(
                    &mut errors,
                    syn::Error::new(
                        requirement.role.span(),
                        format!("unknown refinement role `{role}`"),
                    ),
                ),
                Some(FieldKind::Category(path)) => {
                    let category = path_name(path);
                    if !symbols
                        .category_variants
                        .get(&category)
                        .is_some_and(|variants| variants.contains(&variant))
                    {
                        combine(
                            &mut errors,
                            syn::Error::new(
                                requirement.variant.span(),
                                format!("`{variant}` is not a variant of category `{category}`"),
                            ),
                        );
                    }
                }
                Some(FieldKind::Lex(path)) => {
                    let terminal = path_name(path);
                    match symbols.terminals.get(&terminal) {
                        Some(info)
                            if info.kind == TerminalKind::Vocab
                                && info.variants.contains(&variant) => {}
                        Some(info) if info.kind == TerminalKind::Vocab => combine(
                            &mut errors,
                            syn::Error::new(
                                requirement.variant.span(),
                                format!("unknown variant `{variant}` for vocab `{terminal}`"),
                            ),
                        ),
                        _ => combine(
                            &mut errors,
                            syn::Error::new(
                                requirement.variant.span(),
                                format!(
                                    "role `{role}` is not a category or vocab refinement domain"
                                ),
                            ),
                        ),
                    }
                }
                Some(FieldKind::Identity(_)) => combine(
                    &mut errors,
                    syn::Error::new(
                        requirement.variant.span(),
                        format!("role `{role}` is not a category or vocab refinement domain"),
                    ),
                ),
            }
        }
    }
    finish(errors)
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
                FormAtom::Verb(VerbOperand::Fixed(_)) => Some("verb".to_owned()),
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
) -> HashMap<String, HashMap<feature::FeaturePlace, feature::FeatureResolution>> {
    let mut sealed = HashMap::new();
    for declaration in &raw.declarations {
        let Declaration::Construction(construction) = declaration else { continue };
        let local_equations = equations
            .get(&identifier_key(&construction.name))
            .map_or(&[] as &[feature::FeatureEquation], Vec::as_slice);
        let mut places = local_equations
            .iter()
            .map(|equation| equation.target().clone())
            .collect::<Vec<_>>();
        if construction
            .form
            .atoms
            .iter()
            .any(|atom| matches!(atom, FormAtom::Verb(VerbOperand::Fixed(_))))
        {
            places.push(feature::FeaturePlace::Role {
                field: syn::Ident::new("verb", construction.form.name.span()),
                feature: feature::Feature::Agreement,
            });
        }
        let mut resolutions = HashMap::new();
        for place in places {
            let resolution =
                resolve_local_feature(construction, local_equations, &place, &mut HashSet::new());
            resolutions.insert(place, resolution);
        }
        sealed.insert(identifier_key(&construction.name), resolutions);
    }
    sealed
}

fn seal_category_agreement_capabilities(
    raw: &Declarations,
    resolutions: &HashMap<String, HashMap<feature::FeaturePlace, feature::FeatureResolution>>,
) -> HashMap<String, CategoryAgreementCapability> {
    let constructions = raw
        .declarations
        .iter()
        .filter_map(|declaration| match declaration {
            Declaration::Construction(construction) => Some(construction),
            _ => None,
        })
        .collect::<Vec<_>>();
    let providers = feature_providers(raw);
    let mut contextual = HashSet::new();

    for construction in &constructions {
        let has_fixed_verb = construction
            .form
            .atoms
            .iter()
            .any(|atom| matches!(atom, FormAtom::Verb(VerbOperand::Fixed(_))));
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
            contextual.insert(path_name(&construction.category));
        }
    }

    loop {
        let before = contextual.len();
        for construction in &constructions {
            let passes_external_to_child = construction.form.atoms.iter().any(|atom| {
                let FormAtom::Role(role) = atom else { return false };
                let Some(category) = construction.element.fields.iter().find_map(|field| {
                    same_identifier(&field.name, role)
                        .then_some(&field.kind)
                        .and_then(|kind| match kind {
                            FieldKind::Category(category) => Some(path_name(category)),
                            FieldKind::Lex(_) | FieldKind::Identity(_) => None,
                        })
                }) else {
                    return false;
                };
                let place = feature::FeaturePlace::Role {
                    field: role.clone(),
                    feature: feature::Feature::Agreement,
                };
                contextual.contains(&category)
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
                contextual.insert(path_name(&construction.category));
            }
        }
        if contextual.len() == before {
            break;
        }
    }

    let mut capabilities = HashMap::new();
    for construction in constructions {
        let category = path_name(&construction.category);
        capabilities
            .entry(category.clone())
            .or_insert_with(CategoryAgreementCapability::default)
            .requires_external_input = contextual.contains(&category);
    }
    for (category, capability) in &mut capabilities {
        capability.carries_output =
            providers.contains(&(category.clone(), ParsedFeature::Agreement));
    }
    capabilities
}

fn resolve_local_feature(
    construction: &crate::Construction,
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
                equations,
                &feature::FeaturePlace::Role {
                    field: role.clone(),
                    feature: *feature,
                },
                visiting,
            ),
            feature::FeatureExpr::MatchVocab { role, arms } => {
                let refined = construction
                    .requirements
                    .iter()
                    .find(|requirement| same_identifier(&requirement.role, role))
                    .and_then(|requirement| {
                        arms.iter()
                            .find(|(variant, _)| {
                                same_identifier(variant.value(), &requirement.variant)
                            })
                            .map(|(_, value)| *value)
                    });
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

fn equation_for_place<'a>(
    equations: &'a [feature::FeatureEquation],
    place: &feature::FeaturePlace,
) -> Option<&'a feature::FeatureEquation> {
    equations.iter().find(|equation| equation.target() == place)
}

fn validate_contextual_agreement_uses(
    raw: &Declarations,
    capabilities: &HashMap<String, CategoryAgreementCapability>,
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
                        FieldKind::Lex(_) | FieldKind::Identity(_) => None,
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
    capabilities: &HashMap<String, CategoryAgreementCapability>,
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
        if !symbols.categories.contains(&category) {
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
        if root.punctuation.value().chars().count() != 1 {
            combine(
                &mut errors,
                syn::Error::new(
                    root.punctuation.span(),
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
) -> syn::Result<ContributionInventory> {
    let mut errors = None;
    let mut constructions = Vec::new();
    let mut terminals = Vec::new();
    let mut roots = Vec::new();

    validate_lowerable_backend_shapes(raw)?;

    for declaration in &raw.declarations {
        match declaration {
            Declaration::Construction(construction) => {
                let construction_id = identifier_key(&construction.name);
                let element_type = identifier_key(&construction.element.name);
                let category = path_name(&construction.category);
                let category_variant = pascal_case(&construction_id);
                let form = identifier_key(&construction.form.name);
                let rule_id = format!("{}{category_variant}", pascal_case(&category));
                let atoms = resolved
                    .atoms_by_construction
                    .get(&construction_id)
                    .cloned()
                    .unwrap_or_default();
                let record = ConstructionContribution {
                    construction_id,
                    element_type: element_type.clone(),
                    category,
                    category_variant,
                    form,
                    build_arm: rule_id.clone(),
                    render_arm: element_type.clone(),
                    visitor_method: format!("visit_{}", snake_case(&element_type)),
                    walker: format!("walk_{}", snake_case(&element_type)),
                    atom_count_complete: atoms.len() == construction.form.atoms.len(),
                    terminal_capabilities_complete: false,
                    atoms,
                    rule_id,
                };
                constructions.push(record);
            }
            Declaration::Vocab(vocab) => terminals.push(TerminalContribution {
                name: identifier_key(&vocab.name),
                lex_atom: true,
                identity_atom: false,
                noun_atom: false,
                verb_atom: false,
                direct_render: true,
                direct_build: true,
                traversal: true,
            }),
            Declaration::Lexeme(lexeme) => terminals.push(TerminalContribution {
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
            Declaration::Codec(binding) => terminals.push(TerminalContribution {
                name: identifier_key(&binding.name),
                lex_atom: binding.codec_atom == Some(CodecAtomClass::Lex),
                identity_atom: false,
                noun_atom: binding.codec_atom == Some(CodecAtomClass::Noun),
                verb_atom: false,
                direct_render: binding.render.is_some(),
                direct_build: binding.build.is_some(),
                traversal: closed_traversal_is_lowerable(binding),
            }),
            Declaration::Identity(binding) => {
                terminals.push(TerminalContribution {
                    name: identifier_key(&binding.name),
                    lex_atom: false,
                    identity_atom: true,
                    noun_atom: false,
                    verb_atom: false,
                    direct_render: binding.render.is_some(),
                    direct_build: binding.build.is_some(),
                    traversal: closed_traversal_is_lowerable(binding),
                });
            }
            Declaration::Root(root) => roots.push(RootContribution {
                category: path_name(&root.category),
                parse_entry: root.eoi,
                render_entry: root.standalone_render,
            }),
        }
    }

    if constructions.is_empty() {
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
    for construction in &mut constructions {
        construction.terminal_capabilities_complete = construction
            .atoms
            .iter()
            .all(|atom| atom.is_supported_by(&terminal_capabilities));
        if !construction.is_complete() {
            combine(
                &mut errors,
                syn::Error::new(
                    proc_macro2::Span::call_site(),
                    format!(
                        "construction `{}` is missing a resolved backend contribution",
                        construction.construction_id
                    ),
                ),
            );
        }
    }
    finish(errors)?;
    Ok(ContributionInventory {
        constructions,
        terminals,
        roots,
    })
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
                if !binding_value_type_is_lowerable(binding)
                    || !closed_traversal_is_lowerable(binding)
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
                if root.punctuation.value().chars().count() != 1 {
                    combine(
                        &mut errors,
                        syn::Error::new(
                            root.punctuation.span(),
                            "root punctuation must be exactly one Unicode scalar",
                        ),
                    );
                }
            }
            Declaration::Vocab(_) | Declaration::Lexeme(_) => {}
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

fn pascal_case(name: &str) -> String {
    name.split('_')
        .filter(|part| !part.is_empty())
        .map(|part| {
            let mut chars = part.chars();
            chars.next().map_or_else(String::new, |first| {
                first.to_uppercase().chain(chars).collect()
            })
        })
        .collect()
}

fn snake_case(name: &str) -> String {
    let characters: Vec<_> = name.chars().collect();
    let mut result = String::new();
    for (index, character) in characters.iter().copied().enumerate() {
        if character == '_' {
            if !result.is_empty() && !result.ends_with('_') {
                result.push('_');
            }
            continue;
        }
        if character.is_uppercase() {
            let previous_is_lower =
                index > 0 && characters[index - 1] != '_' && characters[index - 1].is_lowercase();
            let acronym_boundary = index > 0
                && characters[index - 1].is_uppercase()
                && characters
                    .get(index + 1)
                    .is_some_and(|next| next.is_lowercase());
            if (previous_is_lower || acronym_boundary) && !result.ends_with('_') {
                result.push('_');
            }
            result.extend(character.to_lowercase());
        } else {
            result.push(character);
        }
    }
    while result.ends_with('_') {
        result.pop();
    }
    result
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
    use quote::quote;

    use crate::Declaration;

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
        assert!(message.contains("generated Rust name"), "{message}");

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
            raw_element.contains("generated Rust identifier `walk_r#type`")
                && raw_element.contains("walker"),
            "{raw_element}"
        );

        validate(quote! {
            construction where: Cat { element WhereNode {} form where = "where"; }
            root Cat { punctuation = "."; eoi = true; standalone_render = true; }
        })
        .expect("a DSL keyword is valid when every derived Rust identifier is valid");
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
            construction leaf_node: Leaf { element LeafNode {} form leaf = "leaf"; }
            construction only: Cat { element Only { leaf: Leaf, } form only = lex(leaf); }
            root Cat { punctuation = "."; eoi = true; standalone_render = true; }
        });
        assert!(
            category_as_lex.contains("category field `leaf` used as a lexical role"),
            "{category_as_lex}"
        );

        let lexeme_as_plain_lex = error(quote! {
            lexeme Verbs { Be, }
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
            lexeme Verbs { Be, }
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
            vocab_variant.contains("unknown variant `Missing` for vocab `Word`"),
            "{vocab_variant}"
        );

        let wrong_domain = error(quote! {
            lexeme Verbs { Be, }
            construction leaf_node: Leaf { element LeafNode {} derive agreement = Anything::Bare; form leaf = verb(Verbs::Be); }
            construction only: Cat { element Only { leaf: Leaf, } require leaf is Be; form only = leaf; }
            root Cat { punctuation = "."; eoi = true; standalone_render = true; }
        });
        assert!(
            wrong_domain.contains("is not a variant of category `Leaf`"),
            "{wrong_domain}"
        );
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
    fn resolves_one_use_selected_verb_lexeme_provider_without_a_name_switch() {
        let validated = validate(quote! {
            lexeme Actions { Go, }
            construction only: Cat {
                element Only {}
                derive agreement = verb.agreement;
                derive verb.agreement = Values::Bare;
                form only = verb(Actions::Go);
            }
            root Cat { punctuation = "."; eoi = true; standalone_render = true; }
        })
        .expect("a sole used lexeme is the verb provider regardless of its name");
        let actions = validated
            .contributions()
            .terminals()
            .iter()
            .find(|terminal| terminal.name() == "Actions")
            .expect("Actions terminal is inventoried");
        assert!(actions.supports_verb_atom());

        let inconsistent = error(quote! {
            lexeme VerbLexeme { Destroy, }
            lexeme NounLexeme { Player, }
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
    fn unused_name_only_lexemes_retain_traversal_without_direct_atom_capability() {
        let validated = validate(quote! {
            lexeme Objects { Thing, }
            construction only: Cat { element Only {} form only = "only"; }
            root Cat { punctuation = "."; eoi = true; standalone_render = true; }
        })
        .expect("an unused name-only lexeme still contributes its visitor traversal");
        let terminal = validated
            .contributions()
            .terminals()
            .first()
            .expect("lexeme terminal is inventoried");
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
    fn rejects_incomplete_or_open_escape_hatch_metadata() {
        let checked = error(quote! {
            vocab Word { One = "one", }
            construction only: Cat {
                element Only { word: lex Word, }
                checked {
                    visibility missing = private;
                    constructor = Only::new(word, word);
                }
                form only = lex(word);
            }
            root Cat { punctuation = "."; eoi = true; standalone_render = true; }
        });
        assert!(
            checked.contains("unknown checked visibility field `missing`"),
            "{checked}"
        );
        assert!(
            checked.contains("constructor argument `word` is repeated"),
            "{checked}"
        );

        let binding = error(quote! {
            codec Number {
                atom = lex;
                value_type = Number;
                lexical = Lexical::Number;
                render = render_number;
                build {
                    pattern = BuildValue::Number(value, _);
                    construct = arbitrary + rust;
                }
                traversal {
                    part whole = scalar(other.whole);
                    visit missing;
                }
            }
            construction only: Cat { element Only { number: lex Number, } form only = lex(number); }
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
            lexeme Verbs { Be, }
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
            construction leaf_node: Leaf { element LeafNode {} form leaf = "leaf"; }
            construction parent_node: Parent {
                element ParentNode { leaf: Leaf, }
                derive agreement = leaf.agreement;
                form parent = leaf;
            }
            root Parent { punctuation = "."; eoi = true; standalone_render = true; }
        });
        assert!(unbound.contains("does not provide agreement"), "{unbound}");

        let cycle = error(quote! {
            construction leaf_node: Leaf { element LeafNode {} derive agreement = Anything::Bare; form leaf = "leaf"; }
            construction parent_node: Parent {
                element ParentNode { left: Leaf, right: Leaf, }
                derive left.agreement = right.agreement;
                derive right.agreement = left.agreement;
                form parent = left right;
            }
            root Parent { punctuation = "."; eoi = true; standalone_render = true; }
        });
        assert!(cycle.contains("feature equation cycle"), "{cycle}");

        let atoms = error(quote! {
            lexeme Verbs { Be, }
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
            lexeme Actions { Exist, }
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
        assert_eq!(validated.declaration_count(), 7);
        assert_eq!(
            validated.declaration_names(),
            [
                "Pointing",
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
            lexeme Actions { Destroy, Be, Control, }

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
            .raw()
            .declarations
            .iter()
            .find_map(|declaration| match declaration {
                Declaration::Construction(construction) if construction.name == "demonstrative" => {
                    Some(construction)
                }
                _ => None,
            })
            .expect("demonstrative construction remains in the sealed IR");
        assert_eq!(construction.form.name, "demonstrative");
    }

    #[test]
    fn rejects_number_reads_from_fixed_and_projected_verb_slots() {
        let fixed = error(quote! {
            lexeme Verbs { Be, }
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
            lexeme Verbs { Be, }
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
    fn parser_fence_rejects_incomplete_bindings_and_general_require() {
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

        let general = error(quote! {
            construction only: Cat {
                element Only { value: Cat, }
                require value != nothing;
                form only = value;
            }
        });
        assert!(general.contains("general require"), "{general}");
        assert!(general.contains("unimplemented in MVP"), "{general}");
    }

    #[test]
    fn projected_verb_is_rejected_before_backend_planning() {
        let generated = crate::generate(quote! {
            lexeme Verbs { Be, }
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
                checked {
                    visibility payload = pub(crate);
                    constructor = Only::new(payload);
                }
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
                    lexeme Verbs { Act, }
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
            lexeme Verbs { Act, }
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
            lexeme Verbs { Act, }
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
        let inventory = validated.contributions();
        assert_eq!(inventory.constructions().len(), 1);
        let construction = &inventory.constructions()[0];
        assert_eq!(construction.construction_id(), "recursive");
        assert_eq!(construction.element_type(), "Recursive");
        assert_eq!(construction.category_variant(), "Recursive");
        assert!(construction.is_complete());
        assert_eq!(inventory.terminals().len(), 1);
        let terminal = &inventory.terminals()[0];
        assert!(terminal.has_direct_render());
        assert!(terminal.has_direct_build());
        assert!(terminal.has_traversal());
        assert!(terminal.has_direct_render_build_traversal());
        assert_eq!(inventory.roots().len(), 1);
    }

    #[test]
    fn synthetic_projection_fixture_validates_mechanisms_without_product_names() {
        let tokens = crate::test_support::synthetic_projection_tokens();
        let raw = crate::parse_declarations(tokens.clone())
            .expect("synthetic projection declarations parse through the public API");
        let validated = crate::validate_declarations(raw)
            .expect("synthetic projection declarations validate together");
        let expansion = crate::generate(tokens).expect("synthetic projection declarations emit");

        assert_eq!(validated.contributions().constructions().len(), 6);
        assert_eq!(validated.contributions().terminals().len(), 8);
        assert_eq!(validated.contributions().roots().len(), 1);
        assert_eq!(expansion.plan().items().len(), 45);
        assert!(
            validated
                .boxed_fields()
                .contains(&("nested".to_owned(), "next".to_owned()))
        );
        assert!(validated.dynamic_number_constructions().contains("leaf"));

        let refinements = validated
            .raw()
            .declarations
            .iter()
            .filter_map(|declaration| match declaration {
                Declaration::Construction(construction) => construction
                    .requirements
                    .first()
                    .map(|requirement| requirement.variant.to_string()),
                _ => None,
            })
            .collect::<Vec<_>>();
        assert_eq!(refinements, ["Solo", "Leaf"]);

        let terminal = |name| {
            validated
                .contributions()
                .terminals()
                .iter()
                .find(|terminal| terminal.name() == name)
                .unwrap_or_else(|| panic!("synthetic terminal `{name}` is inventoried"))
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
                .contributions()
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
}
