use std::collections::HashMap;
use std::collections::HashSet;

use quote::ToTokens;
use syn::spanned::Spanned;

use crate::feature;
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
    #[allow(
        dead_code,
        reason = "sealed contribution inventory is consumed by Task 4 code generation"
    )]
    contributions: ContributionInventory,
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
    VerbProjected { role: String, terminal: String },
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
            | Self::VerbFixed { terminal, .. }
            | Self::VerbProjected { terminal, .. } => Some(terminal),
            Self::Literal | Self::Category { .. } => None,
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
            Self::VerbFixed { terminal, .. } | Self::VerbProjected { terminal, .. } => terminals
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
    let boxed_fields = validate_category_graph(&raw);
    validate_roots(&raw, &symbols)?;
    let contributions = validate_backend_completeness(&raw, &resolved)?;
    Ok(ValidatedDeclarations {
        raw,
        declaration_names,
        boxed_fields,
        dynamic_numbers,
        feature_equations,
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
                let name = construction.name.to_string();
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
                duplicate_name(
                    &mut source_names,
                    &name,
                    construction.name.span(),
                    &mut errors,
                );
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
                    &pascal_case(&construction.form.name.to_string()),
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
                    let field_name = field.name.to_string();
                    if !fields.insert(field_name.clone()) {
                        combine(
                            &mut errors,
                            syn::Error::new(
                                field.name.span(),
                                format!("duplicate field `{field_name}`"),
                            ),
                        );
                    }
                }
            }
            Declaration::Vocab(vocab) => {
                let name = vocab.name.to_string();
                declaration_names.push(name.clone());
                duplicate_name(&mut source_names, &name, vocab.name.span(), &mut errors);
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
                    let variant_name = variant.name.to_string();
                    if variants.insert(variant_name.clone()) {
                        variant_order.push(variant_name);
                    } else {
                        combine(
                            &mut errors,
                            syn::Error::new(
                                variant.name.span(),
                                format!("duplicate variant `{variant_name}`"),
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
                let name = lexeme.name.to_string();
                declaration_names.push(name.clone());
                duplicate_name(&mut source_names, &name, lexeme.name.span(), &mut errors);
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
                    let variant_name = variant.to_string();
                    if variants.insert(variant_name.clone()) {
                        variant_order.push(variant_name);
                    } else {
                        combine(
                            &mut errors,
                            syn::Error::new(
                                variant.span(),
                                format!("duplicate variant `{variant_name}`"),
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
                let name = binding.name.to_string();
                declaration_names.push(name.clone());
                duplicate_name(&mut source_names, &name, binding.name.span(), &mut errors);
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
    name: &str,
    span: proc_macro2::Span,
    errors: &mut Option<syn::Error>,
) {
    if names.insert(name.to_owned(), span).is_some() {
        combine(
            errors,
            syn::Error::new(span, format!("duplicate declaration `{name}`")),
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
    if let Some((previous, _)) = names.get(generated) {
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
        names.insert(generated.to_owned(), (owner.to_owned(), span));
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
            .map(|field| (field.name.to_string(), &field.kind))
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
                .find(|field| field.name == "verb")
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
                FormAtom::Identity(role) => match fields.get(&role.to_string()) {
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
            .map(|field| (field.name.to_string(), &field.kind))
            .collect();
        let mut atoms = Vec::new();
        for atom in &construction.form.atoms {
            let resolved = match atom {
                FormAtom::Literal(_) => Some(AtomContribution::Literal),
                FormAtom::Role(role) => match fields.get(&role.to_string()) {
                    Some(FieldKind::Category(path)) => Some(AtomContribution::Category {
                        role: role.to_string(),
                        category: path_name(path),
                    }),
                    _ => None,
                },
                FormAtom::Lex(role) => match fields.get(&role.to_string()) {
                    Some(FieldKind::Lex(path)) => Some(AtomContribution::Lex {
                        role: role.to_string(),
                        terminal: path_name(path),
                    }),
                    _ => None,
                },
                FormAtom::Identity(role) => match fields.get(&role.to_string()) {
                    Some(FieldKind::Identity(path)) => Some(AtomContribution::Identity {
                        role: role.to_string(),
                        terminal: path_name(path),
                    }),
                    _ => None,
                },
                FormAtom::Noun(role) => match fields.get(&role.to_string()) {
                    Some(FieldKind::Lex(path)) => Some(AtomContribution::Noun {
                        role: role.to_string(),
                        terminal: path_name(path),
                    }),
                    _ => None,
                },
                FormAtom::Verb(VerbOperand::Fixed(path)) => {
                    let segments: Vec<_> = path.segments.iter().collect();
                    let Some(terminal) = segments.iter().rev().nth(1) else { continue };
                    let Some(variant) = segments.last() else { continue };
                    let terminal = terminal.ident.to_string();
                    let variant = variant.ident.to_string();
                    record_verb_provider(
                        &terminal,
                        path.span(),
                        &mut seen_verb_providers,
                        &mut verb_providers,
                    );
                    Some(AtomContribution::VerbFixed { terminal, variant })
                }
                FormAtom::Verb(VerbOperand::Projected(role)) => {
                    match fields.get(&role.to_string()) {
                        Some(FieldKind::Lex(path)) => {
                            let terminal = path_name(path);
                            record_verb_provider(
                                &terminal,
                                role.span(),
                                &mut seen_verb_providers,
                                &mut verb_providers,
                            );
                            Some(AtomContribution::VerbProjected {
                                role: role.to_string(),
                                terminal,
                            })
                        }
                        _ => None,
                    }
                }
            };
            if let Some(resolved) = resolved {
                atoms.push(resolved);
            }
        }
        atoms_by_construction.insert(construction.name.to_string(), atoms);
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
    if let Some(FieldKind::Lex(path)) = fields.get(&role.to_string()) {
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
    let supported = match fields.get(&role.to_string()) {
        Some(FieldKind::Lex(path)) => symbols.terminals.get(&path_name(path)).is_some_and(|info| {
            info.kind == TerminalKind::Codec && info.codec_atom == Some(CodecAtomClass::Noun)
        }),
        Some(FieldKind::Identity(_)) => false,
        Some(FieldKind::Category(_)) | None => return,
    };
    if !supported {
        let declared = match fields.get(&role.to_string()) {
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
    if role == "verb" && has_fixed_verb {
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
    match fields.get(&role.to_string()) {
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
        Some(FieldKind::Lex(_)) if local_vocab_providers.contains(&(role.to_string(), feature)) => {
        }
        Some(FieldKind::Lex(path))
            if verb_operands.iter().any(
                |operand| matches!(operand, VerbOperand::Projected(field) if field == role),
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
            if field != role {
                return None;
            }
            let Some(FieldKind::Lex(path)) = fields.get(&role.to_string()) else {
                return None;
            };
            let terminal = symbols.terminals.get(&path_name(path))?;
            if terminal.kind != TerminalKind::Vocab {
                return None;
            }
            let variants = arms
                .iter()
                .map(|arm| arm.variant.to_string())
                .collect::<HashSet<_>>();
            (variants.len() == arms.len() && variants == terminal.variants)
                .then(|| (role.to_string(), *feature))
        })
        .collect()
}

fn check_role_kind(
    role: &syn::Ident,
    fields: &HashMap<String, &FieldKind>,
    category: bool,
    errors: &mut Option<syn::Error>,
) {
    match fields.get(&role.to_string()) {
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
    let supported = match fields.get(&role.to_string()) {
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
    if let Some(FieldKind::Lex(path)) = fields.get(&role.to_string()) {
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
    let terminal = segments[segments.len() - 2].ident.to_string();
    let variant = segments.last().expect("at least two").ident.to_string();
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
            .map(|field| (field.name.to_string(), 0))
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
                && let Some(count) = counts.get_mut(&role.to_string())
            {
                *count += 1;
            }
        }
        for field in &construction.element.fields {
            match counts[&field.name.to_string()] {
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
                    .map(|field| field.name.to_string())
                    .collect();
                let mut visible = HashSet::new();
                for visibility in &checked.visibilities {
                    let name = visibility.role.to_string();
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
                    if !visible.contains(&field.name.to_string()) {
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
                    let name = accessor.role.to_string();
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
                    let name = visibility.role.to_string();
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
                        let name = role.to_string();
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
                    if !arguments.contains(&field.name.to_string()) {
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
                let element = construction.element.name.to_string();
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
                        &binding.name.to_string(),
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
                leaf.name.to_string(),
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
    let ty = name.to_string();
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
    validate_context_identity(binding, errors);
    let mut bound = HashSet::new();
    let Some(build) = &binding.build else {
        bound.insert(binding.traversal.argument.as_ref().map_or_else(
            || snake_case(&binding.name.to_string()),
            ToString::to_string,
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
                .is_some_and(|(left, right)| left.ident == right.ident);
            let shapes_ok = tuple.elems.iter().all(|pat| match pat {
                syn::Pat::Ident(ident)
                    if ident.by_ref.is_none()
                        && ident.mutability.is_none()
                        && ident.subpat.is_none() =>
                {
                    bound.insert(ident.ident.to_string())
                }
                _ => false,
            });
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
        if !seen.insert(arm.variant.to_string()) {
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
        .map(ToString::to_string)
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
            if !names.insert(variant.to_string()) {
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
            || snake_case(&binding.name.to_string()),
            ToString::to_string,
        );
        call_bound.insert(argument.clone());
        let mut call_types = HashMap::from([(argument, simple_type_name(&binding.value_type))]);
        for field in &traversal.fields {
            let name = field.name.to_string();
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
            if !leaf.name.to_string().starts_with("visit_") {
                combine(
                    errors,
                    syn::Error::new(
                        leaf.name.span(),
                        "leaf callback name must start with `visit_`",
                    ),
                );
            }
            if !names.insert(leaf.name.to_string()) {
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
                    .map(|segment| segment.ident.to_string())
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
                    .map(|segment| segment.ident.to_string());
                if owner.as_ref() != matched_type.as_ref() {
                    combine(
                        errors,
                        syn::Error::new_spanned(
                            &arm.variant,
                            "traversal match variant type does not match the matched runtime value",
                        ),
                    );
                }
                let binding_name = arm.binding.to_string();
                arm_bound.insert(binding_name.clone());
                arm_types.insert(binding_name, type_name(&arm.value_type));
                validate_traversal_call(&arm.call, &arm_bound, &arm_types, callbacks, errors);
            }
            let declared = traversal
                .variants
                .iter()
                .map(ToString::to_string)
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
        let name = part.name.to_string();
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
        let name = visit.to_string();
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
        if !visited.contains(&part.name.to_string()) {
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
            call.callback.segments[0].ident.to_string(),
            &callbacks.walkers,
        ),
        2 if call.callback.leading_colon.is_none()
            && call.callback.segments[0].ident == "visitor" =>
        {
            (
                true,
                call.callback.segments[1].ident.to_string(),
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
        syn::Expr::Path(path) if path.qself.is_none() && path.path.segments.len() == 1 => {
            types.get(&path.path.segments[0].ident.to_string()).cloned()
        }
        syn::Expr::Paren(paren) => expr_type(&paren.expr, types),
        _ => None,
    }
}

fn simple_type_name(value_type: &syn::Type) -> String {
    match value_type {
        syn::Type::Path(path) => path.path.segments.last().map_or_else(
            || type_name(value_type),
            |segment| segment.ident.to_string(),
        ),
        _ => type_name(value_type),
    }
}

fn type_name(value_type: &syn::Type) -> String {
    value_type.to_token_stream().to_string().replace(' ', "")
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
                    .is_some_and(|segment| bound.contains(&segment.ident.to_string()))
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
            .map(|field| (field.name.to_string(), &field.kind))
            .collect();
        let mut refined = HashSet::new();
        for requirement in &construction.requirements {
            let role = requirement.role.to_string();
            if !refined.insert(role.clone()) {
                combine(
                    &mut errors,
                    syn::Error::new(
                        requirement.role.span(),
                        format!("duplicate refinement for role `{role}`"),
                    ),
                );
            }
            let variant = requirement.variant.to_string();
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
            .map(|field| (field.name.to_string(), &field.kind))
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
                combine(
                    &mut errors,
                    syn::Error::new(
                        construction.name.span(),
                        format!("duplicate writer for `{target_key}`"),
                    ),
                );
            }
            let target = match &equation.target {
                ParsedFeaturePlace::Construction(feature) => {
                    feature::FeaturePlace::Construction((*feature).into())
                }
                ParsedFeaturePlace::Role { field, feature } => {
                    if let Some(FieldKind::Category(path)) = fields.get(&field.to_string()) {
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
                    if let Some(FieldKind::Category(path)) = fields.get(&slot.role.to_string()) {
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
                    let source_key = format!("{}.{}", slot.role, feature_name(slot.feature));
                    edges.entry(target_key).or_default().push(source_key);
                    feature::FeatureExpr::FromRole {
                        role: slot.role.clone(),
                        feature: slot.feature.into(),
                    }
                }
                ParsedFeatureValue::Match { role, arms } => {
                    let mut lowered_arms = Vec::new();
                    let mut seen = HashSet::new();
                    let terminal = match fields.get(&role.to_string()) {
                        Some(FieldKind::Lex(path)) => symbols.terminals.get(&path_name(path)),
                        _ => None,
                    };
                    for arm in arms {
                        let variant = arm.variant.to_string();
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
                        dynamic_numbers.insert(construction.name.to_string());
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
                FormAtom::Verb(VerbOperand::Projected(role)) => Some(role.to_string()),
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
                    ) if source.role == slot && source.feature == ParsedFeature::Agreement
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
        lowered.insert(construction.name.to_string(), equations);
    }
    finish(errors)?;
    Ok((lowered, dynamic_numbers))
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
            format!("{field}.{}", feature_name(*feature))
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
                        boxed.insert((construction.name.to_string(), field.name.to_string()));
                    }
                }
            }
        }
    }
    boxed
}

fn validate_roots(raw: &Declarations, symbols: &Symbols) -> syn::Result<()> {
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
        if root.punctuation.value().is_empty() {
            combine(
                &mut errors,
                syn::Error::new(root.punctuation.span(), "root punctuation cannot be empty"),
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

    for declaration in &raw.declarations {
        match declaration {
            Declaration::Construction(construction) => {
                let construction_id = construction.name.to_string();
                let element_type = construction.element.name.to_string();
                let category = path_name(&construction.category);
                let category_variant = pascal_case(&construction_id);
                let form = construction.form.name.to_string();
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
                name: vocab.name.to_string(),
                lex_atom: true,
                identity_atom: false,
                noun_atom: false,
                verb_atom: false,
                direct_render: true,
                direct_build: true,
                traversal: true,
            }),
            Declaration::Lexeme(lexeme) => terminals.push(TerminalContribution {
                name: lexeme.name.to_string(),
                lex_atom: false,
                identity_atom: false,
                noun_atom: false,
                verb_atom: resolved
                    .verb_lexeme_provider
                    .as_ref()
                    .is_some_and(|provider| provider == &lexeme.name.to_string()),
                direct_render: false,
                direct_build: false,
                traversal: true,
            }),
            Declaration::Codec(binding) => terminals.push(TerminalContribution {
                name: binding.name.to_string(),
                lex_atom: binding.codec_atom == Some(CodecAtomClass::Lex),
                identity_atom: false,
                noun_atom: binding.codec_atom == Some(CodecAtomClass::Noun),
                verb_atom: false,
                direct_render: binding.render.is_some(),
                direct_build: binding.build.is_some(),
                traversal: true,
            }),
            Declaration::Identity(binding) => {
                terminals.push(TerminalContribution {
                    name: binding.name.to_string(),
                    lex_atom: false,
                    identity_atom: true,
                    noun_atom: false,
                    verb_atom: false,
                    direct_render: binding.render.is_some(),
                    direct_build: binding.build.is_some(),
                    traversal: true,
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

fn path_name(path: &syn::Path) -> String {
    path.segments
        .last()
        .map_or_else(String::new, |segment| segment.ident.to_string())
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

        let lexeme_domain = error(quote! {
            lexeme Verbs { Be, }
            construction only: Cat {
                element Only { word: lex Verbs, }
                require word is Be;
                derive word.agreement = Anything::Bare;
                form only = verb(word);
            }
            root Cat { punctuation = "."; eoi = true; standalone_render = true; }
        });
        assert!(
            lexeme_domain.contains("is not a category or vocab refinement domain"),
            "{lexeme_domain}"
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
                traversal { part value = identity(value); visit value; }
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
                traversal { part value = identity(value); visit value; }
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
                traversal { part value = identity(value); visit value; }
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
                form destroy = verb(VerbLexeme::Destroy);
            }
            construction player: Cat {
                element Player {}
                derive agreement = verb.agreement;
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
                traversal { part value = scalar(value); visit value; }
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
                traversal { part value = subtree(value); visit value; }
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
                traversal { part value = scalar(value); visit value; }
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
                traversal { part value = scalar(value); visit value; }
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
                traversal { part value = scalar(value); visit value; }
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
                traversal { part value = scalar(value); visit value; }
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
                traversal { part value = scalar(value); visit value; }
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
