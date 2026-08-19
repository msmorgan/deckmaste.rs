use std::collections::BTreeSet;
use std::collections::HashMap;
use std::collections::HashSet;

use proc_macro2::Span;
use syn::spanned::Spanned;

use crate::feature;
use crate::feature::Feature;
use crate::feature::FeatureExpr;
use crate::feature::FeaturePlace;
use crate::identifier::key as identifier_key;
use crate::identifier::pascal_case;
use crate::identifier::path_key;
use crate::identifier::snake_case;
use crate::model::Declaration;
use crate::model::Declarations;
use crate::model::Form;
use crate::model::FormAtom;
use crate::model::VerbOperand;
use crate::plan::DeclarationKey;
use crate::plan::DeclarationKind;
use crate::validate::AtomContribution;
use crate::validate::CategoryRenderCapability;

/// The one sealed semantic authority produced after validation succeeds.
#[derive(Debug)]
pub(crate) struct SemanticPlan {
    declaration_keys: Vec<DeclarationKey>,
    constructions: Vec<ConstructionPlan>,
    terminals: Vec<TerminalPlan>,
    runtime: RuntimeEmissionPlan,
    roots: Vec<RootPlan>,
    features: FeaturePlan,
}

#[derive(Debug)]
#[allow(
    clippy::large_enum_variant,
    dead_code,
    reason = "typed terminal rows deliberately own their complete lowering payloads"
)]
pub(crate) struct ConstructionPlan {
    source_index: usize,
    origin_span: Span,
    category_span: Span,
    construction_id: String,
    category: String,
    category_variant: String,
    element_type: String,
    form: String,
    rule_id: String,
    build_arm: String,
    render_arm: String,
    visitor_method: String,
    walker: String,
    checked_constructor: bool,
    private_fields: bool,
    fields: Vec<ConstructionFieldPlan>,
    refinements: Vec<RefinementPlan>,
    constructor: Option<CheckedConstructorPlan>,
    atoms: Vec<AtomPlan>,
}

#[derive(Debug)]
pub(crate) struct ConstructionFieldPlan {
    name: syn::Ident,
    kind: ConstructionFieldKind,
    terminal: String,
    value_type: syn::Path,
    visibility: FieldVisibilityPlan,
    accessor: Option<syn::Ident>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum ConstructionFieldKind {
    Category,
    Lex,
    Identity,
}

#[derive(Debug)]
pub(crate) enum FieldVisibilityPlan {
    Public,
    Private,
    Restricted(syn::Visibility),
}

#[derive(Debug)]
pub(crate) struct RefinementPlan {
    role: syn::Ident,
    variant: syn::Ident,
}

#[derive(Debug)]
pub(crate) struct CheckedConstructorPlan {
    path: syn::Path,
    arguments: Vec<ConstructorArgumentPlan>,
}

#[derive(Debug)]
pub(crate) enum ConstructorArgumentPlan {
    Role(syn::Ident),
    VecRole(syn::Ident),
    Context,
}

#[derive(Debug, Clone)]
pub(crate) enum AtomPlan {
    Literal(String),
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
        path: syn::Path,
    },
    OpenDeclaration(OpenDeclarationAtomPlan),
}

#[derive(Debug, Clone)]
pub(crate) struct OpenDeclarationAtomPlan {
    kind: macro_ron::v2::DeclarationKind,
    name: String,
    position: macro_ron::v2::GrammarPosition,
}

#[derive(Debug)]
#[allow(
    clippy::large_enum_variant,
    dead_code,
    reason = "typed terminal rows deliberately own their complete lowering payloads"
)]
pub(crate) enum TerminalPlan {
    Vocab(VocabPlan),
    Lexeme(LexemePlan),
    Binding(BindingPlan),
    SignedDecimal(SignedDecimalPlan),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum UnsignedPrimitive {
    U32,
}

#[derive(Debug)]
pub(crate) struct SignedDecimalPlan {
    source_index: usize,
    origin: DeclarationKey,
    origin_span: Span,
    codec_name: String,
    codec_ident: syn::Ident,
    sign_type: syn::Ident,
    positive_variant: syn::Ident,
    negative_variant: syn::Ident,
    magnitude: UnsignedPrimitive,
}

/// The sealed terminal-capability projection consumed by runtime emission.
#[derive(Debug)]
struct RuntimeEmissionPlan {
    vocab_indices: Vec<usize>,
    noun_lexeme_index: Option<usize>,
    verb_lexeme_index: Option<usize>,
    noun_binding_index: Option<usize>,
    direct_binding_indices: Vec<usize>,
    opaque_binding_indices: Vec<usize>,
    signed_decimal_index: Option<usize>,
    punctuation_literals: Vec<String>,
    scanner_origin_indices: Vec<usize>,
}

impl RuntimeEmissionPlan {
    fn seal(
        terminals: &[TerminalPlan],
        constructions: &[ConstructionPlan],
        roots: &[RootPlan],
    ) -> syn::Result<Self> {
        let mut plan = Self {
            vocab_indices: Vec::new(),
            noun_lexeme_index: None,
            verb_lexeme_index: None,
            noun_binding_index: None,
            direct_binding_indices: Vec::new(),
            opaque_binding_indices: Vec::new(),
            signed_decimal_index: None,
            punctuation_literals: Vec::new(),
            scanner_origin_indices: Vec::new(),
        };
        for (index, terminal) in terminals.iter().enumerate() {
            match terminal {
                TerminalPlan::Vocab(_) => plan.vocab_indices.push(index),
                TerminalPlan::Lexeme(lexeme) if lexeme.is_verb_provider() => {
                    if plan.verb_lexeme_index.replace(index).is_some() {
                        return Err(syn::Error::new(
                            lexeme.name_ident().span(),
                            "sealed runtime inventory has multiple verb lexeme providers",
                        ));
                    }
                }
                TerminalPlan::Lexeme(lexeme) => {
                    if plan.noun_lexeme_index.replace(index).is_some() {
                        return Err(syn::Error::new(
                            lexeme.name_ident().span(),
                            "sealed runtime inventory has multiple noun lexeme providers",
                        ));
                    }
                }
                TerminalPlan::Binding(binding)
                    if binding.codec_atom() == Some(crate::model::CodecAtomClass::Noun)
                        && binding.lexical_variant().is_some() =>
                {
                    if plan.noun_binding_index.replace(index).is_some() {
                        return Err(syn::Error::new(
                            binding.origin_span(),
                            "sealed runtime inventory has multiple noun terminal bindings",
                        ));
                    }
                }
                TerminalPlan::Binding(binding) if binding.lexical_variant().is_some() => {
                    if binding.build().is_some_and(|build| {
                        build.slots().len() == 1 && build.construct_is_direct_slot()
                    }) {
                        plan.direct_binding_indices.push(index);
                    } else {
                        plan.opaque_binding_indices.push(index);
                    }
                }
                TerminalPlan::Binding(_) => {}
                TerminalPlan::SignedDecimal(codec) => {
                    if plan.signed_decimal_index.replace(index).is_some() {
                        return Err(syn::Error::new(
                            codec.origin_span(),
                            "sealed runtime inventory has multiple signed_decimal codecs",
                        ));
                    }
                }
            }
        }
        plan.punctuation_literals = roots
            .iter()
            .map(|root| root.punctuation.clone())
            .chain(constructions.iter().flat_map(|construction| {
                construction.atoms.iter().filter_map(|atom| match atom {
                    AtomPlan::Literal(literal) if is_punctuation_literal(literal) => {
                        Some(literal.clone())
                    }
                    AtomPlan::Literal(_)
                    | AtomPlan::Category { .. }
                    | AtomPlan::Lex { .. }
                    | AtomPlan::Identity { .. }
                    | AtomPlan::Noun { .. }
                    | AtomPlan::VerbFixed { .. }
                    | AtomPlan::OpenDeclaration(_) => None,
                })
            }))
            .collect::<BTreeSet<_>>()
            .into_iter()
            .collect();
        let mut scanner_origin_indices = plan
            .vocab_indices
            .iter()
            .chain(plan.noun_lexeme_index.iter())
            .chain(plan.verb_lexeme_index.iter())
            .chain(plan.noun_binding_index.iter())
            .chain(&plan.direct_binding_indices)
            .chain(&plan.opaque_binding_indices)
            .chain(plan.signed_decimal_index.iter())
            .map(|&index| terminals[index].source_index())
            .collect::<BTreeSet<_>>();
        scanner_origin_indices.extend(roots.iter().map(RootPlan::source_index));
        scanner_origin_indices.extend(
            constructions
                .iter()
                .filter(|construction| {
                    construction.atoms.iter().any(
                        |atom| matches!(atom, AtomPlan::Literal(literal) if is_punctuation_literal(literal)),
                    )
                })
                .map(ConstructionPlan::source_index),
        );
        plan.scanner_origin_indices = scanner_origin_indices.into_iter().collect();
        Ok(plan)
    }
}

fn is_punctuation_literal(literal: &str) -> bool {
    literal.chars().count() == 1
        && literal
            .chars()
            .all(|character| character.is_ascii_punctuation())
}

pub(crate) enum AtomTerminal<'a> {
    Vocab(&'a VocabPlan),
    Binding(&'a BindingPlan),
    SignedDecimal(&'a SignedDecimalPlan),
}

#[derive(Debug)]
#[allow(
    dead_code,
    reason = "sealed rows are the next generation phase's semantic input"
)]
pub(crate) struct VocabPlan {
    source_index: usize,
    name: syn::Ident,
    name_key: String,
    variants: Vec<VocabVariantPlan>,
}

#[derive(Debug)]
pub(crate) struct VocabVariantPlan {
    name: syn::Ident,
    word: syn::LitStr,
}

#[derive(Debug)]
#[allow(
    dead_code,
    reason = "sealed rows are the next generation phase's semantic input"
)]
pub(crate) struct LexemePlan {
    source_index: usize,
    name: syn::Ident,
    name_key: String,
    variants: Vec<syn::Ident>,
    verb_provider: bool,
}

#[derive(Debug)]
#[allow(
    dead_code,
    reason = "sealed rows are the next generation phase's semantic input"
)]
pub(crate) struct BindingPlan {
    source_index: usize,
    origin: DeclarationKey,
    origin_span: Span,
    name: syn::Ident,
    name_key: String,
    kind: crate::model::TerminalBindingKind,
    codec_atom: Option<crate::model::CodecAtomClass>,
    value_type_name: syn::Ident,
    lexical_variant: Option<syn::Path>,
    render: Option<BindingRenderPlan>,
    build: Option<BindingBuildPlan>,
    traversal: BindingTraversalPlan,
    stored_spelling: bool,
}

#[derive(Debug)]
pub(crate) enum BindingRenderPlan {
    Runtime(syn::Path),
    ContextIdentity(Vec<ContextIdentityPlan>),
}

#[derive(Debug)]
pub(crate) struct ContextIdentityPlan {
    variant: syn::Ident,
    accessor: syn::Ident,
}

#[derive(Debug)]
pub(crate) struct BindingBuildPlan {
    variant: syn::Ident,
    slots: Vec<syn::Ident>,
    construct: BindingBuildExprPlan,
    construct_is_direct_slot: bool,
}

#[derive(Debug)]
pub(crate) enum BindingBuildExprPlan {
    Slot(String),
    Field {
        base: Box<BindingBuildExprPlan>,
        member: syn::Member,
    },
    Call {
        function: syn::Path,
        arguments: Vec<BindingBuildExprPlan>,
    },
    Parenthesized(Box<BindingBuildExprPlan>),
}

#[derive(Debug)]
pub(crate) struct BindingTraversalPlan {
    mode: crate::model::VisitMode,
    argument: String,
    fields: Vec<TraversalFieldPlan>,
    recipe: BindingTraversalRecipe,
    leaf_callbacks: Vec<LeafCallbackPlan>,
}

#[derive(Debug)]
pub(crate) enum BindingTraversalRecipe {
    Branches(Vec<TraversalBranchPlan>),
    Variants(Vec<syn::Ident>),
    Calls(Vec<TraversalCallPlan>),
}

#[derive(Debug)]
pub(crate) struct TraversalFieldPlan {
    name: syn::Ident,
}

#[derive(Debug)]
pub(crate) struct TraversalCallPlan {
    callback: syn::Path,
    mode: crate::model::VisitMode,
    value: TraversalValuePlan,
}

#[derive(Debug)]
pub(crate) struct TraversalBranchPlan {
    value: TraversalValuePlan,
    arms: Vec<TraversalBranchArmPlan>,
}

#[derive(Debug)]
pub(crate) enum TraversalValuePlan {
    Root(String),
    Field {
        base: Box<TraversalValuePlan>,
        member: syn::Member,
    },
    Parenthesized(Box<TraversalValuePlan>),
}

#[derive(Debug)]
pub(crate) struct TraversalBranchArmPlan {
    variant: syn::Path,
    binding: syn::Ident,
    call: TraversalCallPlan,
}

#[derive(Debug)]
pub(crate) struct LeafCallbackPlan {
    name: syn::Ident,
    value_type: syn::Type,
    mode: crate::model::VisitMode,
}

#[derive(Debug)]
#[allow(
    dead_code,
    reason = "sealed rows are the next generation phase's semantic input"
)]
pub(crate) struct RootPlan {
    source_index: usize,
    category: String,
    punctuation: String,
    parse_entry: bool,
    render_entry: bool,
}

#[derive(Debug)]
pub(crate) struct FeaturePlan {
    boxed_fields: HashSet<(String, String)>,
    dynamic_numbers: HashSet<String>,
    category_reads: HashMap<String, HashSet<Feature>>,
    equations: HashMap<String, Vec<feature::FeatureEquation>>,
    resolutions: HashMap<String, HashMap<feature::FeaturePlace, feature::FeatureResolution>>,
    category_render: HashMap<String, CategoryRenderCapability>,
    number_carry_categories: HashSet<String>,
}

impl SemanticPlan {
    #[allow(
        clippy::too_many_arguments,
        reason = "validation seals its independent facts together"
    )]
    pub(crate) fn new(
        source: &Declarations,
        boxed_fields: HashSet<(String, String)>,
        dynamic_numbers: HashSet<String>,
        category_reads: HashMap<String, HashSet<Feature>>,
        equations: HashMap<String, Vec<feature::FeatureEquation>>,
        resolutions: HashMap<String, HashMap<feature::FeaturePlace, feature::FeatureResolution>>,
        category_render: HashMap<String, CategoryRenderCapability>,
        mut atoms_by_construction: HashMap<String, (Span, Vec<AtomContribution>)>,
        verb_lexeme_provider: Option<&str>,
    ) -> syn::Result<Self> {
        let declaration_keys = source
            .declarations
            .iter()
            .map(DeclarationKey::from_source)
            .collect();
        let constructions =
            source
                .declarations
                .iter()
                .enumerate()
                .filter_map(|(source_index, declaration)| match declaration {
                    Declaration::Construction(construction) => Some((source_index, construction)),
                    Declaration::Vocab(_)
                    | Declaration::Lexeme(_)
                    | Declaration::Codec(_)
                    | Declaration::Identity(_)
                    | Declaration::Root(_) => None,
                })
                .map(|(source_index, construction)| {
                    let construction_id = identifier_key(&construction.name);
                    let (_, atoms) = atoms_by_construction.remove(&construction_id).ok_or_else(|| {
                    syn::Error::new(
                        construction.name.span(),
                        format!(
                            "sealed semantic plan is missing construction `{construction_id}`"
                        ),
                    )
                })?;
                    ConstructionPlan::from_source(source_index, construction, &atoms)
                })
                .collect::<syn::Result<Vec<_>>>()?;
        if let Some((name, (span, _))) = atoms_by_construction.into_iter().next() {
            return Err(syn::Error::new(
                span,
                format!("sealed semantic plan has surplus construction `{name}`"),
            ));
        }
        let number_carry_categories = number_carry_categories(&constructions, &equations);

        let terminals = source
            .declarations
            .iter()
            .enumerate()
            .filter_map(|(source_index, declaration)| match declaration {
                Declaration::Vocab(vocab) => Some(Ok(TerminalPlan::Vocab(VocabPlan::from_source(
                    source_index,
                    vocab,
                )))),
                Declaration::Lexeme(lexeme) => Some(Ok(TerminalPlan::Lexeme(
                    LexemePlan::from_source(source_index, lexeme, verb_lexeme_provider),
                ))),
                Declaration::Codec(binding) if binding.generated.is_some() => {
                    Some(Ok(TerminalPlan::SignedDecimal(
                        SignedDecimalPlan::from_source(source_index, binding),
                    )))
                }
                Declaration::Codec(binding) | Declaration::Identity(binding) => {
                    Some(BindingPlan::from_source(source_index, binding).map(TerminalPlan::Binding))
                }
                Declaration::Construction(_) | Declaration::Root(_) => None,
            })
            .collect::<syn::Result<Vec<_>>>()?;
        let roots: Vec<RootPlan> = source
            .declarations
            .iter()
            .enumerate()
            .filter_map(|(source_index, declaration)| match declaration {
                Declaration::Root(root) => Some(RootPlan {
                    source_index,
                    category: crate::identifier::path_key(&root.category),
                    punctuation: root.punctuation.value(),
                    parse_entry: root.eoi,
                    render_entry: root.standalone_render,
                }),
                Declaration::Construction(_)
                | Declaration::Vocab(_)
                | Declaration::Lexeme(_)
                | Declaration::Codec(_)
                | Declaration::Identity(_) => None,
            })
            .collect();
        let runtime = RuntimeEmissionPlan::seal(&terminals, &constructions, &roots)?;

        Ok(Self {
            declaration_keys,
            constructions,
            terminals,
            runtime,
            roots,
            features: FeaturePlan {
                boxed_fields,
                dynamic_numbers,
                category_reads,
                equations,
                resolutions,
                category_render,
                number_carry_categories,
            },
        })
    }

    pub(crate) fn declaration_count(&self) -> usize {
        self.declaration_keys.len()
    }

    pub(crate) fn declaration_keys(&self) -> &[DeclarationKey] {
        &self.declaration_keys
    }

    #[allow(
        dead_code,
        reason = "sealed rows are the next generation phase's semantic input"
    )]
    pub(crate) fn constructions(&self) -> &[ConstructionPlan] {
        &self.constructions
    }

    pub(crate) fn required_open_declarations(
        &self,
    ) -> impl Iterator<Item = (&ConstructionPlan, &OpenDeclarationAtomPlan)> {
        self.constructions.iter().flat_map(|construction| {
            construction
                .atoms
                .iter()
                .filter_map(move |atom| match atom {
                    AtomPlan::OpenDeclaration(open) => Some((construction, open)),
                    _ => None,
                })
        })
    }

    pub(crate) fn has_open_declarations(&self) -> bool {
        self.required_open_declarations().next().is_some()
    }

    #[allow(
        dead_code,
        reason = "sealed rows are the next generation phase's semantic input"
    )]
    pub(crate) fn terminals(&self) -> &[TerminalPlan] {
        &self.terminals
    }

    pub(crate) fn runtime_vocabs(&self) -> impl Iterator<Item = &VocabPlan> {
        self.runtime
            .vocab_indices
            .iter()
            .map(|&index| match &self.terminals[index] {
                TerminalPlan::Vocab(vocab) => vocab,
                TerminalPlan::Lexeme(_)
                | TerminalPlan::Binding(_)
                | TerminalPlan::SignedDecimal(_) => {
                    unreachable!("sealed runtime vocab index changed terminal kind")
                }
            })
    }

    pub(crate) fn runtime_punctuation_literals(&self) -> &[String] {
        &self.runtime.punctuation_literals
    }

    pub(crate) fn runtime_scanner_origins(&self) -> Vec<DeclarationKey> {
        self.runtime
            .scanner_origin_indices
            .iter()
            .map(|&index| self.declaration_keys[index].clone())
            .collect()
    }

    pub(crate) fn runtime_noun_lexeme(&self) -> Option<&LexemePlan> {
        self.runtime_lexeme(self.runtime.noun_lexeme_index)
    }

    pub(crate) fn runtime_verb_lexeme(&self) -> Option<&LexemePlan> {
        self.runtime_lexeme(self.runtime.verb_lexeme_index)
    }

    pub(crate) fn runtime_noun_binding(&self) -> Option<&BindingPlan> {
        self.runtime.noun_binding_index.map(|index| {
            let TerminalPlan::Binding(binding) = &self.terminals[index] else {
                unreachable!("sealed runtime noun-binding index changed terminal kind")
            };
            binding
        })
    }

    pub(crate) fn runtime_direct_bindings(&self) -> impl Iterator<Item = &BindingPlan> {
        self.runtime_bindings(&self.runtime.direct_binding_indices)
    }

    pub(crate) fn runtime_opaque_bindings(&self) -> impl Iterator<Item = &BindingPlan> {
        self.runtime_bindings(&self.runtime.opaque_binding_indices)
    }

    pub(crate) fn runtime_signed_decimal(&self) -> Option<&SignedDecimalPlan> {
        self.runtime.signed_decimal_index.map(|index| {
            let TerminalPlan::SignedDecimal(codec) = &self.terminals[index] else {
                unreachable!("sealed runtime signed-decimal index changed terminal kind")
            };
            codec
        })
    }

    fn runtime_lexeme(&self, index: Option<usize>) -> Option<&LexemePlan> {
        index.map(|index| {
            let TerminalPlan::Lexeme(lexeme) = &self.terminals[index] else {
                unreachable!("sealed runtime lexeme index changed terminal kind")
            };
            lexeme
        })
    }

    fn runtime_bindings<'a>(
        &'a self,
        indices: &'a [usize],
    ) -> impl Iterator<Item = &'a BindingPlan> + 'a {
        indices.iter().map(|&index| {
            let TerminalPlan::Binding(binding) = &self.terminals[index] else {
                unreachable!("sealed runtime binding index changed terminal kind")
            };
            binding
        })
    }

    #[allow(
        dead_code,
        reason = "sealed rows are the next generation phase's semantic input"
    )]
    pub(crate) fn roots(&self) -> &[RootPlan] {
        &self.roots
    }

    pub(crate) fn feature_equations(&self, construction: &str) -> &[feature::FeatureEquation] {
        self.features
            .equations
            .get(construction)
            .map_or(&[], Vec::as_slice)
    }

    pub(crate) fn feature_resolution(
        &self,
        construction: &str,
        place: &feature::FeaturePlace,
    ) -> Option<feature::FeatureResolution> {
        self.features
            .resolutions
            .get(construction)
            .and_then(|resolutions| resolutions.get(place))
            .copied()
    }

    pub(crate) fn category_render_capability(&self, category: &str) -> CategoryRenderCapability {
        self.features
            .category_render
            .get(category)
            .copied()
            .unwrap_or_default()
    }

    pub(crate) fn category_reads_feature(&self, category: &str, feature: Feature) -> bool {
        self.features
            .category_reads
            .get(category)
            .is_some_and(|features| features.contains(&feature))
    }

    pub(crate) fn category_carries_agreement(&self, category: &str) -> bool {
        self.category_render_capability(category)
            .carries_agreement()
    }

    pub(crate) fn category_requires_external_agreement(&self, category: &str) -> bool {
        self.category_render_capability(category)
            .requires_external_agreement()
    }

    pub(crate) fn boxed_fields(&self) -> &HashSet<(String, String)> {
        &self.features.boxed_fields
    }

    pub(crate) fn dynamic_number_constructions(&self) -> &HashSet<String> {
        &self.features.dynamic_numbers
    }

    pub(crate) fn category_carries_number(&self, category: &str) -> bool {
        self.features.number_carry_categories.contains(category)
    }

    #[cfg(test)]
    pub(crate) fn test_only_replace_planned_literal(
        &mut self,
        construction_id: &str,
        atom_index: usize,
        literal: &str,
    ) {
        let construction = self
            .constructions
            .iter_mut()
            .find(|row| row.construction_id == construction_id)
            .expect("test construction is present");
        construction.atoms[atom_index] = AtomPlan::Literal(literal.to_owned());
    }

    #[cfg(test)]
    pub(crate) fn test_only_replace_open_declaration_name(
        &mut self,
        construction_id: &str,
        atom_index: usize,
        name: &str,
    ) {
        let construction = self
            .constructions
            .iter_mut()
            .find(|row| row.construction_id == construction_id)
            .expect("test construction is present");
        let AtomPlan::OpenDeclaration(open) = &mut construction.atoms[atom_index] else {
            panic!("test atom is an open declaration")
        };
        open.name = name.to_owned();
    }

    #[cfg(test)]
    pub(crate) fn test_only_replace_vocab_spelling(
        &mut self,
        vocabulary: &str,
        variant: &str,
        spelling: &str,
    ) {
        let vocab = self
            .terminals
            .iter_mut()
            .find_map(|terminal| match terminal {
                TerminalPlan::Vocab(vocab) if vocab.name() == vocabulary => Some(vocab),
                TerminalPlan::Vocab(_)
                | TerminalPlan::Lexeme(_)
                | TerminalPlan::Binding(_)
                | TerminalPlan::SignedDecimal(_) => None,
            })
            .expect("test vocabulary is present");
        let word = vocab
            .variants
            .iter_mut()
            .find(|candidate| identifier_key(&candidate.name) == variant)
            .expect("test vocabulary variant is present");
        word.word = syn::LitStr::new(spelling, word.word.span());
    }

    #[cfg(test)]
    pub(crate) fn test_only_replace_vocab_name(&mut self, old: &str, new: &str) {
        let vocab = self
            .terminals
            .iter_mut()
            .find_map(|terminal| match terminal {
                TerminalPlan::Vocab(vocab) if vocab.name() == old => Some(vocab),
                TerminalPlan::Vocab(_)
                | TerminalPlan::Lexeme(_)
                | TerminalPlan::Binding(_)
                | TerminalPlan::SignedDecimal(_) => None,
            })
            .expect("test vocabulary is present");
        vocab.name = syn::Ident::new(new, vocab.name.span());
        vocab.name_key = new.to_owned();
    }

    #[cfg(test)]
    pub(crate) fn test_only_replace_binding_name(&mut self, old: &str, new: &str) {
        let binding = self
            .terminals
            .iter_mut()
            .find_map(|terminal| match terminal {
                TerminalPlan::Binding(binding) if binding.name() == old => Some(binding),
                TerminalPlan::Vocab(_)
                | TerminalPlan::Lexeme(_)
                | TerminalPlan::Binding(_)
                | TerminalPlan::SignedDecimal(_) => None,
            })
            .expect("test binding is present");
        binding.name = syn::Ident::new(new, binding.name.span());
        binding.name_key = new.to_owned();
        binding.origin = DeclarationKey::new(binding.origin.kind(), new);
    }

    #[cfg(test)]
    pub(crate) fn test_only_replace_signed_decimal_sign_shape(
        &mut self,
        sign_type: &str,
        positive: &str,
        negative: &str,
    ) {
        let codec = self
            .terminals
            .iter_mut()
            .find_map(|terminal| match terminal {
                TerminalPlan::SignedDecimal(codec) => Some(codec),
                TerminalPlan::Vocab(_) | TerminalPlan::Lexeme(_) | TerminalPlan::Binding(_) => None,
            })
            .expect("test signed_decimal codec is present");
        codec.sign_type = syn::Ident::new(sign_type, codec.sign_type.span());
        codec.positive_variant = syn::Ident::new(positive, codec.positive_variant.span());
        codec.negative_variant = syn::Ident::new(negative, codec.negative_variant.span());
    }

    #[cfg(test)]
    pub(crate) fn test_only_replace_binding_build_variant(&mut self, name: &str, variant: &str) {
        let binding = self
            .terminals
            .iter_mut()
            .find_map(|terminal| match terminal {
                TerminalPlan::Binding(binding) if binding.name() == name => Some(binding),
                TerminalPlan::Vocab(_)
                | TerminalPlan::Lexeme(_)
                | TerminalPlan::Binding(_)
                | TerminalPlan::SignedDecimal(_) => None,
            })
            .expect("test binding is present");
        let build = binding
            .build
            .as_mut()
            .expect("test binding build is present");
        build.variant = syn::Ident::new(variant, build.variant.span());
    }

    #[cfg(test)]
    pub(crate) fn test_only_replace_binding_value_type_name(&mut self, name: &str, value: &str) {
        let binding = self
            .terminals
            .iter_mut()
            .find_map(|terminal| match terminal {
                TerminalPlan::Binding(binding) if binding.name() == name => Some(binding),
                TerminalPlan::Vocab(_)
                | TerminalPlan::Lexeme(_)
                | TerminalPlan::Binding(_)
                | TerminalPlan::SignedDecimal(_) => None,
            })
            .expect("test binding is present");
        binding.value_type_name = syn::Ident::new(value, binding.value_type_name.span());
    }

    #[cfg(test)]
    pub(crate) fn test_only_mismatch_binding_origin(&mut self, name: &str) {
        let binding = self
            .terminals
            .iter_mut()
            .find_map(|terminal| match terminal {
                TerminalPlan::Binding(binding) if binding.name() == name => Some(binding),
                TerminalPlan::Vocab(_)
                | TerminalPlan::Lexeme(_)
                | TerminalPlan::Binding(_)
                | TerminalPlan::SignedDecimal(_) => None,
            })
            .expect("test binding is present");
        binding.origin = DeclarationKey::new(DeclarationKind::Identity, binding.name());
    }

    #[cfg(test)]
    pub(crate) fn test_only_remove_runtime_vocab(&mut self, name: &str) {
        let index = self
            .terminals
            .iter()
            .position(
                |terminal| matches!(terminal, TerminalPlan::Vocab(vocab) if vocab.name() == name),
            )
            .expect("test runtime vocabulary is present");
        self.runtime
            .vocab_indices
            .retain(|candidate| *candidate != index);
    }

    #[cfg(test)]
    pub(crate) fn test_only_seal_construction_atoms(
        form: &Form,
        atoms: &[AtomContribution],
    ) -> syn::Result<Vec<AtomPlan>> {
        seal_atoms(form, atoms)
    }

    pub(crate) fn parse_root(&self, category: &str) -> Option<&RootPlan> {
        self.roots
            .iter()
            .find(|root| root.parse_entry && root.category == category)
    }

    pub(crate) fn atom_terminal(&self, name: &str) -> syn::Result<AtomTerminal<'_>> {
        for terminal in &self.terminals {
            match terminal {
                TerminalPlan::Vocab(row) if row.name() == name => {
                    return Ok(AtomTerminal::Vocab(row));
                }
                TerminalPlan::Binding(row) if row.name() == name => {
                    return Ok(AtomTerminal::Binding(row));
                }
                TerminalPlan::SignedDecimal(row) if row.codec_name() == name => {
                    return Ok(AtomTerminal::SignedDecimal(row));
                }
                TerminalPlan::Vocab(_)
                | TerminalPlan::Lexeme(_)
                | TerminalPlan::Binding(_)
                | TerminalPlan::SignedDecimal(_) => {}
            }
        }
        Err(sealed_error("resolved atom terminal"))
    }

    #[cfg(test)]
    pub(crate) fn snapshot(&self) -> SemanticSnapshot {
        let declaration_keys = self
            .declaration_keys
            .iter()
            .map(|key| (key.kind(), key.name().to_owned()))
            .collect();
        let constructions = self
            .constructions
            .iter()
            .map(|plan| {
                (
                    plan.construction_id.clone(),
                    plan.element_type.clone(),
                    plan.form.clone(),
                    plan.atoms.iter().map(AtomPlan::snapshot).collect(),
                )
            })
            .collect();
        let terminals = self.terminals.iter().map(TerminalPlan::snapshot).collect();
        let roots = self
            .roots
            .iter()
            .map(|root| (root.category.clone(), root.parse_entry, root.render_entry))
            .collect();
        let feature_equations = self
            .constructions
            .iter()
            .filter_map(|construction| {
                let equations = self.feature_equations(&construction.construction_id);
                (!equations.is_empty()).then(|| {
                    (
                        construction.construction_id.clone(),
                        equations
                            .iter()
                            .map(feature::FeatureEquation::snapshot)
                            .collect::<Vec<_>>()
                            .join("; "),
                    )
                })
            })
            .collect();
        let mut boxed_fields = self.boxed_fields().iter().cloned().collect::<Vec<_>>();
        boxed_fields.sort();
        let mut dynamic_number_constructions = self
            .dynamic_number_constructions()
            .iter()
            .cloned()
            .collect::<Vec<_>>();
        dynamic_number_constructions.sort();
        SemanticSnapshot {
            declaration_keys,
            constructions,
            terminals,
            roots,
            feature_equations,
            boxed_fields,
            dynamic_number_constructions,
        }
    }

    #[cfg(test)]
    pub(crate) fn feature_resolutions_snapshot(&self) -> Vec<(String, Vec<(String, String)>)> {
        self.constructions
            .iter()
            .map(|construction| {
                let mut resolutions = self
                    .features
                    .resolutions
                    .get(&construction.construction_id)
                    .into_iter()
                    .flat_map(|resolutions| resolutions.iter())
                    .map(|(place, resolution)| (place.snapshot(), resolution.snapshot()))
                    .collect::<Vec<_>>();
                resolutions.sort();
                (construction.construction_id.clone(), resolutions)
            })
            .collect()
    }

    #[cfg(test)]
    pub(crate) fn category_render_capabilities_snapshot(&self) -> Vec<(String, bool, bool, bool)> {
        let mut capabilities = self
            .features
            .category_render
            .iter()
            .map(|(category, capability)| {
                (
                    category.clone(),
                    capability.carries_agreement(),
                    capability.requires_external_agreement(),
                    capability.requires_context(),
                )
            })
            .collect::<Vec<_>>();
        capabilities.sort();
        capabilities
    }
}

impl ConstructionPlan {
    fn from_source(
        source_index: usize,
        source: &crate::Construction,
        resolved_atoms: &[AtomContribution],
    ) -> syn::Result<Self> {
        let construction_id = identifier_key(&source.name);
        let category = path_key(&source.category);
        let category_variant = pascal_case(&construction_id);
        let element_type = identifier_key(&source.element.name);
        let rule_id = format!("{}{category_variant}", pascal_case(&category));
        let private_fields = source.checked.as_ref().is_some_and(|checked| {
            checked.visibilities.iter().any(|visibility| {
                matches!(
                    visibility.visibility,
                    crate::model::NonPublicVisibility::Private(_)
                )
            })
        });
        let fields = source
            .element
            .fields
            .iter()
            .map(|field| {
                let (kind, terminal, value_type) = match &field.kind {
                    crate::model::FieldKind::Category(path) => (
                        ConstructionFieldKind::Category,
                        path_key(path),
                        path.clone(),
                    ),
                    crate::model::FieldKind::Lex(path) => {
                        (ConstructionFieldKind::Lex, path_key(path), path.clone())
                    }
                    crate::model::FieldKind::Identity(path) => (
                        ConstructionFieldKind::Identity,
                        path_key(path),
                        path.clone(),
                    ),
                };
                let visibility = if let Some(checked) = &source.checked {
                    let visibility = checked
                        .visibilities
                        .iter()
                        .find(|visibility| {
                            identifier_key(&visibility.role) == identifier_key(&field.name)
                        })
                        .ok_or_else(|| {
                            syn::Error::new(
                                field.name.span(),
                                "sealed checked field visibility is absent",
                            )
                        })?;
                    match &visibility.visibility {
                        crate::model::NonPublicVisibility::Private(_) => {
                            FieldVisibilityPlan::Private
                        }
                        crate::model::NonPublicVisibility::Restricted(visibility) => {
                            FieldVisibilityPlan::Restricted(visibility.clone())
                        }
                    }
                } else {
                    FieldVisibilityPlan::Public
                };
                let accessor = source.checked.as_ref().and_then(|checked| {
                    checked
                        .accessors
                        .iter()
                        .find(|accessor| {
                            identifier_key(&accessor.role) == identifier_key(&field.name)
                        })
                        .map(|accessor| accessor.method.clone())
                });
                Ok(ConstructionFieldPlan {
                    name: field.name.clone(),
                    kind,
                    terminal,
                    value_type,
                    visibility,
                    accessor,
                })
            })
            .collect::<syn::Result<Vec<_>>>()?;
        let refinements = source
            .requirements
            .iter()
            .map(|requirement| RefinementPlan {
                role: requirement.role.clone(),
                variant: requirement.variant.clone(),
            })
            .collect();
        let constructor = source
            .checked
            .as_ref()
            .map(|checked| CheckedConstructorPlan {
                path: checked.constructor.path.clone(),
                arguments: checked
                    .constructor
                    .arguments
                    .iter()
                    .map(|argument| match argument {
                        crate::model::ConstructorArgument::Role(role) => {
                            ConstructorArgumentPlan::Role(role.clone())
                        }
                        crate::model::ConstructorArgument::VecRole { role, .. } => {
                            ConstructorArgumentPlan::VecRole(role.clone())
                        }
                        crate::model::ConstructorArgument::Context(_) => {
                            ConstructorArgumentPlan::Context
                        }
                    })
                    .collect(),
            });
        let atoms = seal_atoms(&source.form, resolved_atoms)?;
        Ok(Self {
            source_index,
            origin_span: source.name.span(),
            category_span: source
                .category
                .segments
                .last()
                .map_or_else(Span::call_site, |segment| segment.ident.span()),
            construction_id,
            category,
            category_variant,
            element_type: element_type.clone(),
            form: identifier_key(&source.form.name),
            rule_id: rule_id.clone(),
            build_arm: rule_id,
            render_arm: element_type.clone(),
            visitor_method: format!("visit_{}", snake_case(&element_type)),
            walker: format!("walk_{}", snake_case(&element_type)),
            checked_constructor: source.checked.is_some(),
            private_fields,
            fields,
            refinements,
            constructor,
            atoms,
        })
    }

    #[allow(
        dead_code,
        reason = "future emitters retain source order through this index"
    )]
    pub(crate) fn source_index(&self) -> usize {
        self.source_index
    }

    pub(crate) fn construction_id(&self) -> &str {
        &self.construction_id
    }

    pub(crate) fn form(&self) -> &str {
        &self.form
    }

    pub(crate) fn category(&self) -> &str {
        &self.category
    }

    pub(crate) fn category_variant(&self) -> &str {
        &self.category_variant
    }

    pub(crate) fn element_type(&self) -> &str {
        &self.element_type
    }

    pub(crate) fn rule_id(&self) -> &str {
        &self.rule_id
    }

    pub(crate) fn build_arm(&self) -> &str {
        &self.build_arm
    }

    pub(crate) fn has_checked_constructor(&self) -> bool {
        self.checked_constructor
    }

    pub(crate) fn atoms(&self) -> &[AtomPlan] {
        &self.atoms
    }

    pub(crate) fn origin_span(&self) -> Span {
        self.origin_span
    }

    pub(crate) fn category_span(&self) -> Span {
        self.category_span
    }

    pub(crate) fn fields(&self) -> &[ConstructionFieldPlan] {
        &self.fields
    }

    pub(crate) fn field(&self, role: &str) -> syn::Result<&ConstructionFieldPlan> {
        self.fields
            .iter()
            .find(|field| field.name_key() == role)
            .ok_or_else(|| syn::Error::new(self.origin_span, "sealed construction field is absent"))
    }

    pub(crate) fn refinements(&self) -> &[RefinementPlan] {
        &self.refinements
    }

    pub(crate) fn has_private_fields(&self) -> bool {
        self.private_fields
    }

    pub(crate) fn constructor(&self) -> Option<&CheckedConstructorPlan> {
        self.constructor.as_ref()
    }
}

impl ConstructionFieldPlan {
    pub(crate) fn name(&self) -> &syn::Ident {
        &self.name
    }

    pub(crate) fn name_key(&self) -> String {
        identifier_key(&self.name)
    }

    pub(crate) fn kind(&self) -> ConstructionFieldKind {
        self.kind
    }

    pub(crate) fn terminal(&self) -> &str {
        &self.terminal
    }

    pub(crate) fn accessor(&self) -> Option<&syn::Ident> {
        self.accessor.as_ref()
    }

    pub(crate) fn has_accessor(&self) -> bool {
        self.accessor.is_some()
    }

    pub(crate) fn value_type(&self) -> &syn::Path {
        &self.value_type
    }

    pub(crate) fn visibility(&self) -> &FieldVisibilityPlan {
        &self.visibility
    }
}

impl RefinementPlan {
    pub(crate) fn role(&self) -> &syn::Ident {
        &self.role
    }

    pub(crate) fn variant(&self) -> &syn::Ident {
        &self.variant
    }
}

impl CheckedConstructorPlan {
    pub(crate) fn path(&self) -> &syn::Path {
        &self.path
    }

    pub(crate) fn arguments(&self) -> &[ConstructorArgumentPlan] {
        &self.arguments
    }
}

impl AtomPlan {
    fn from_source(source: &FormAtom, resolved: &AtomContribution) -> syn::Result<Self> {
        match (source, resolved) {
            (FormAtom::Literal(literal), AtomContribution::Literal) => {
                Ok(Self::Literal(literal.value()))
            }
            (FormAtom::Role(authored), AtomContribution::Category { role, category }) => {
                ensure_atom_name(authored, role, "category atom role name")?;
                Ok(Self::Category {
                    role: role.clone(),
                    category: category.clone(),
                })
            }
            (FormAtom::Lex(authored), AtomContribution::Lex { role, terminal }) => {
                ensure_atom_name(authored, role, "lex atom role name")?;
                Ok(Self::Lex {
                    role: role.clone(),
                    terminal: terminal.clone(),
                })
            }
            (FormAtom::Identity(authored), AtomContribution::Identity { role, terminal }) => {
                ensure_atom_name(authored, role, "identity atom role name")?;
                Ok(Self::Identity {
                    role: role.clone(),
                    terminal: terminal.clone(),
                })
            }
            (FormAtom::Noun(authored), AtomContribution::Noun { role, terminal }) => {
                ensure_atom_name(authored, role, "noun atom role name")?;
                Ok(Self::Noun {
                    role: role.clone(),
                    terminal: terminal.clone(),
                })
            }
            (
                FormAtom::Verb(VerbOperand::Fixed(path)),
                AtomContribution::VerbFixed { terminal, variant },
            ) => {
                let mut segments = path.segments.iter().rev();
                let authored_variant = segments.next().ok_or_else(|| {
                    syn::Error::new(path.span(), "sealed fixed-verb path is empty")
                })?;
                let authored_terminal = segments.next().ok_or_else(|| {
                    syn::Error::new(path.span(), "sealed fixed-verb path has no terminal name")
                })?;
                if identifier_key(&authored_terminal.ident) != *terminal {
                    return Err(syn::Error::new(
                        authored_terminal.ident.span(),
                        "sealed fixed-verb terminal name is inconsistent",
                    ));
                }
                if identifier_key(&authored_variant.ident) != *variant {
                    return Err(syn::Error::new(
                        authored_variant.ident.span(),
                        "sealed fixed-verb variant name is inconsistent",
                    ));
                }
                Ok(Self::VerbFixed {
                    terminal: terminal.clone(),
                    variant: variant.clone(),
                    path: path.clone(),
                })
            }
            (FormAtom::OpenVerb(authored), AtomContribution::OpenDeclaration { kind, name }) => {
                if authored.name.value() != *name {
                    return Err(syn::Error::new(
                        authored.name.span(),
                        "sealed open declaration name is inconsistent",
                    ));
                }
                Ok(Self::OpenDeclaration(OpenDeclarationAtomPlan {
                    kind: *kind,
                    name: name.clone(),
                    position: macro_ron::v2::GrammarPosition::Verb,
                }))
            }
            _ => Err(syn::Error::new(
                form_atom_span(source),
                "sealed construction atom kind is inconsistent",
            )),
        }
    }

    #[cfg(test)]
    fn snapshot(&self) -> String {
        match self {
            Self::Literal(_) => "literal".to_owned(),
            Self::Category { role, category } => format!("category({role}: {category})"),
            Self::Lex { role, .. } => format!("lex({role})"),
            Self::Identity { role, .. } => format!("identity({role})"),
            Self::Noun { role, .. } => format!("noun({role})"),
            Self::VerbFixed {
                terminal, variant, ..
            } => format!("verb({terminal}::{variant})"),
            Self::OpenDeclaration(open) => format!(
                "open_verb({:?}, {}, {:?})",
                open.kind, open.name, open.position
            ),
        }
    }
}

impl OpenDeclarationAtomPlan {
    pub(crate) fn kind(&self) -> macro_ron::v2::DeclarationKind {
        self.kind
    }

    pub(crate) fn name(&self) -> &str {
        &self.name
    }

    pub(crate) fn position(&self) -> macro_ron::v2::GrammarPosition {
        self.position
    }
}

fn ensure_atom_name(authored: &syn::Ident, resolved: &str, fact: &str) -> syn::Result<()> {
    if identifier_key(authored) == resolved {
        Ok(())
    } else {
        Err(syn::Error::new(
            authored.span(),
            format!("sealed construction {fact} is inconsistent"),
        ))
    }
}

fn seal_atoms(form: &Form, resolved: &[AtomContribution]) -> syn::Result<Vec<AtomPlan>> {
    if form.atoms.len() != resolved.len() {
        return Err(syn::Error::new(
            form.name.span(),
            "sealed construction atom count is inconsistent",
        ));
    }
    form.atoms
        .iter()
        .zip(resolved)
        .map(|(source, resolved)| AtomPlan::from_source(source, resolved))
        .collect()
}

fn form_atom_span(atom: &FormAtom) -> Span {
    match atom {
        FormAtom::Literal(literal) => literal.span(),
        FormAtom::Role(role)
        | FormAtom::Lex(role)
        | FormAtom::Identity(role)
        | FormAtom::Noun(role)
        | FormAtom::Verb(VerbOperand::Projected(role)) => role.span(),
        FormAtom::Verb(VerbOperand::Fixed(path)) => path.span(),
        FormAtom::OpenVerb(open) => open.name.span(),
    }
}

fn number_carry_categories(
    constructions: &[ConstructionPlan],
    equations: &HashMap<String, Vec<feature::FeatureEquation>>,
) -> HashSet<String> {
    let mut carried = HashSet::new();
    loop {
        let before = carried.len();
        for construction in constructions {
            let output_is_needed = construction
                .atoms
                .iter()
                .any(|atom| matches!(atom, AtomPlan::Noun { .. }))
                || carried.contains(&construction.category);
            if !output_is_needed {
                continue;
            }
            let Some(FeatureExpr::FromRole {
                role,
                feature: Feature::Number,
            }) = equations
                .get(&construction.construction_id)
                .into_iter()
                .flatten()
                .find(|equation| equation.target() == &FeaturePlace::Construction(Feature::Number))
                .map(feature::FeatureEquation::value)
            else {
                continue;
            };
            if let Some(category) = construction.atoms.iter().find_map(|atom| match atom {
                AtomPlan::Category {
                    role: found,
                    category,
                } if found == &identifier_key(role) => Some(category.clone()),
                AtomPlan::Literal(_)
                | AtomPlan::Category { .. }
                | AtomPlan::Lex { .. }
                | AtomPlan::Identity { .. }
                | AtomPlan::Noun { .. }
                | AtomPlan::VerbFixed { .. }
                | AtomPlan::OpenDeclaration(_) => None,
            }) {
                carried.insert(category);
            }
        }
        if carried.len() == before {
            return carried;
        }
    }
}

fn sealed_error(fact: &str) -> syn::Error {
    syn::Error::new(
        proc_macro2::Span::call_site(),
        format!("sealed semantic plan has an inconsistent {fact}"),
    )
}

impl TerminalPlan {
    fn source_index(&self) -> usize {
        match self {
            Self::Vocab(plan) => plan.source_index(),
            Self::Lexeme(plan) => plan.source_index(),
            Self::Binding(plan) => plan.source_index(),
            Self::SignedDecimal(plan) => plan.source_index(),
        }
    }
}

#[cfg(test)]
impl TerminalPlan {
    pub(crate) fn name(&self) -> &str {
        match self {
            Self::Vocab(plan) => plan.name(),
            Self::Lexeme(plan) => plan.name(),
            Self::Binding(plan) => plan.name(),
            Self::SignedDecimal(plan) => plan.codec_name(),
        }
    }

    #[cfg(test)]
    fn snapshot(&self) -> (String, String, Vec<String>) {
        let kind = match self {
            Self::Vocab(_) => "vocab",
            Self::Lexeme(_) => "lexeme",
            Self::Binding(binding)
                if binding.kind() == crate::model::TerminalBindingKind::Identity =>
            {
                "identity"
            }
            Self::Binding(_) | Self::SignedDecimal(_) => "codec",
        };
        let mut capabilities = Vec::new();
        if self.supports_lex_atom() {
            capabilities.push("lex".to_owned());
        }
        if self.supports_identity_atom() {
            capabilities.push("identity".to_owned());
        }
        if self.supports_noun_atom() {
            capabilities.push("noun".to_owned());
        }
        if self.supports_verb_atom() {
            capabilities.push("verb".to_owned());
        }
        if self.has_direct_render() {
            capabilities.push("render".to_owned());
        }
        if self.has_direct_build() {
            capabilities.push("build".to_owned());
        }
        capabilities.push("traversal".to_owned());
        (self.name().to_owned(), kind.to_owned(), capabilities)
    }

    pub(crate) fn supports_lex_atom(&self) -> bool {
        matches!(self, Self::Vocab(_) | Self::SignedDecimal(_))
            || matches!(self, Self::Binding(binding) if binding.codec_atom() == Some(crate::model::CodecAtomClass::Lex))
    }

    pub(crate) fn supports_identity_atom(&self) -> bool {
        matches!(self, Self::Binding(binding) if binding.kind() == crate::model::TerminalBindingKind::Identity)
    }

    pub(crate) fn supports_noun_atom(&self) -> bool {
        matches!(self, Self::Binding(binding) if binding.codec_atom() == Some(crate::model::CodecAtomClass::Noun))
    }

    pub(crate) fn supports_verb_atom(&self) -> bool {
        matches!(self, Self::Lexeme(lexeme) if lexeme.is_verb_provider())
    }

    pub(crate) fn has_direct_render(&self) -> bool {
        matches!(self, Self::Vocab(_) | Self::SignedDecimal(_))
            || matches!(self, Self::Binding(binding) if binding.render().is_some())
    }

    pub(crate) fn has_direct_build(&self) -> bool {
        matches!(self, Self::Vocab(_) | Self::SignedDecimal(_))
            || matches!(self, Self::Binding(binding) if binding.build().is_some())
    }

    #[allow(
        clippy::unused_self,
        reason = "every closed terminal row owns a traversal recipe"
    )]
    pub(crate) fn has_traversal(&self) -> bool {
        true
    }
}

impl VocabPlan {
    fn from_source(source_index: usize, source: &crate::Vocab) -> Self {
        Self {
            source_index,
            name: source.name.clone(),
            name_key: identifier_key(&source.name),
            variants: source
                .variants
                .iter()
                .map(|variant| VocabVariantPlan {
                    name: variant.name.clone(),
                    word: variant.word.clone(),
                })
                .collect(),
        }
    }

    pub(crate) fn name(&self) -> &str {
        &self.name_key
    }

    pub(crate) fn name_ident(&self) -> &syn::Ident {
        &self.name
    }

    pub(crate) fn variants(&self) -> &[VocabVariantPlan] {
        &self.variants
    }

    #[allow(
        dead_code,
        reason = "future emitters retain source order through this index"
    )]
    pub(crate) fn source_index(&self) -> usize {
        self.source_index
    }
}

impl LexemePlan {
    fn from_source(
        source_index: usize,
        source: &crate::Lexeme,
        verb_provider: Option<&str>,
    ) -> Self {
        Self {
            source_index,
            name: source.name.clone(),
            name_key: identifier_key(&source.name),
            variants: source.variants.clone(),
            verb_provider: verb_provider
                .is_some_and(|provider| provider == identifier_key(&source.name)),
        }
    }

    pub(crate) fn name(&self) -> &str {
        &self.name_key
    }

    pub(crate) fn name_ident(&self) -> &syn::Ident {
        &self.name
    }

    pub(crate) fn variants(&self) -> &[syn::Ident] {
        &self.variants
    }

    pub(crate) fn is_verb_provider(&self) -> bool {
        self.verb_provider
    }
    #[allow(
        dead_code,
        reason = "future emitters retain source order through this index"
    )]
    pub(crate) fn source_index(&self) -> usize {
        self.source_index
    }
}

impl SignedDecimalPlan {
    fn from_source(source_index: usize, source: &crate::TerminalBinding) -> Self {
        let Some(crate::model::GeneratedCodecRecipe::SignedDecimal(recipe)) = &source.generated
        else {
            unreachable!("validated generated codec has the signed_decimal recipe")
        };
        let sign = recipe
            .sign_type_slots
            .first()
            .expect("validated signed_decimal has one sign_type");
        let positive_variant = sign
            .roles
            .iter()
            .find(|role| identifier_key(&role.variant) == "Positive")
            .expect("validated signed_decimal has a positive role")
            .variant
            .clone();
        let negative_variant = sign
            .roles
            .iter()
            .find(|role| identifier_key(&role.variant) == "Negative")
            .expect("validated signed_decimal has a negative role")
            .variant
            .clone();
        Self {
            source_index,
            origin: DeclarationKey::new(DeclarationKind::Codec, identifier_key(&source.name)),
            origin_span: source.name.span(),
            codec_name: identifier_key(&source.name),
            codec_ident: source.name.clone(),
            sign_type: sign.name.clone(),
            positive_variant,
            negative_variant,
            magnitude: UnsignedPrimitive::U32,
        }
    }

    pub(crate) fn source_index(&self) -> usize {
        self.source_index
    }

    pub(crate) fn origin(&self) -> &DeclarationKey {
        &self.origin
    }

    pub(crate) fn origin_span(&self) -> Span {
        self.origin_span
    }

    pub(crate) fn codec_name(&self) -> &str {
        &self.codec_name
    }

    pub(crate) fn codec_ident(&self) -> &syn::Ident {
        &self.codec_ident
    }

    pub(crate) fn sign_type(&self) -> &syn::Ident {
        &self.sign_type
    }

    pub(crate) fn positive_variant(&self) -> &syn::Ident {
        &self.positive_variant
    }

    pub(crate) fn negative_variant(&self) -> &syn::Ident {
        &self.negative_variant
    }

    pub(crate) fn magnitude(&self) -> UnsignedPrimitive {
        self.magnitude
    }
}

impl BindingPlan {
    fn from_source(source_index: usize, source: &crate::TerminalBinding) -> syn::Result<Self> {
        let render = source.render.as_ref().map(|render| match render {
            crate::model::RenderBinding::Runtime(path) => BindingRenderPlan::Runtime(path.clone()),
            crate::model::RenderBinding::ContextIdentity(arms) => {
                BindingRenderPlan::ContextIdentity(
                    arms.iter()
                        .map(|arm| ContextIdentityPlan {
                            variant: arm.variant.clone(),
                            accessor: arm.accessor.clone(),
                        })
                        .collect(),
                )
            }
        });
        let build = source
            .build
            .as_ref()
            .map(BindingBuildPlan::from_source)
            .transpose()?;
        let value_type_name = binding_value_type_name(&source.value_type)?;
        let mode = source.traversal.callback_mode.ok_or_else(|| {
            syn::Error::new(
                source.name.span(),
                "sealed terminal traversal mode is absent",
            )
        })?;
        let argument = source.traversal.argument.as_ref().map_or_else(
            || default_leaf_argument(&identifier_key(&source.name)),
            identifier_key,
        );
        let fields = source
            .traversal
            .fields
            .iter()
            .map(|field| TraversalFieldPlan {
                name: field.name.clone(),
            })
            .collect();
        let recipe = if !source.traversal.branches.is_empty() {
            BindingTraversalRecipe::Branches(
                source
                    .traversal
                    .branches
                    .iter()
                    .map(TraversalBranchPlan::from_source)
                    .collect::<syn::Result<Vec<_>>>()?,
            )
        } else if !source.traversal.variants.is_empty() {
            BindingTraversalRecipe::Variants(source.traversal.variants.clone())
        } else {
            BindingTraversalRecipe::Calls(
                source
                    .traversal
                    .calls
                    .iter()
                    .map(TraversalCallPlan::from_source)
                    .collect::<syn::Result<Vec<_>>>()?,
            )
        };
        let leaf_callbacks = source
            .traversal
            .leaf_callbacks
            .iter()
            .map(|leaf| LeafCallbackPlan {
                name: leaf.name.clone(),
                value_type: leaf.value_type.clone(),
                mode: leaf.mode,
            })
            .collect();
        Ok(Self {
            source_index,
            origin: DeclarationKey::new(
                match source.kind {
                    crate::model::TerminalBindingKind::Codec => DeclarationKind::Codec,
                    crate::model::TerminalBindingKind::Identity => DeclarationKind::Identity,
                },
                identifier_key(&source.name),
            ),
            origin_span: source.name.span(),
            name: source.name.clone(),
            name_key: identifier_key(&source.name),
            kind: source.kind,
            codec_atom: source.codec_atom,
            value_type_name,
            lexical_variant: source.lexical_variant.clone(),
            stored_spelling: matches!(&render, Some(BindingRenderPlan::ContextIdentity(arms)) if arms.len() >= 2),
            render,
            build,
            traversal: BindingTraversalPlan {
                mode,
                argument,
                fields,
                recipe,
                leaf_callbacks,
            },
        })
    }

    pub(crate) fn name(&self) -> &str {
        &self.name_key
    }

    pub(crate) fn kind(&self) -> crate::model::TerminalBindingKind {
        self.kind
    }

    pub(crate) fn codec_atom(&self) -> Option<crate::model::CodecAtomClass> {
        self.codec_atom
    }

    pub(crate) fn value_type_name(&self) -> &syn::Ident {
        &self.value_type_name
    }

    pub(crate) fn lexical_variant(&self) -> Option<&syn::Path> {
        self.lexical_variant.as_ref()
    }

    pub(crate) fn render(&self) -> Option<&BindingRenderPlan> {
        self.render.as_ref()
    }

    pub(crate) fn build(&self) -> Option<&BindingBuildPlan> {
        self.build.as_ref()
    }

    pub(crate) fn traversal(&self) -> &BindingTraversalPlan {
        &self.traversal
    }

    pub(crate) fn has_stored_spelling(&self) -> bool {
        self.stored_spelling
    }

    pub(crate) fn origin(&self) -> &DeclarationKey {
        &self.origin
    }

    pub(crate) fn origin_span(&self) -> Span {
        self.origin_span
    }
    #[allow(
        dead_code,
        reason = "future emitters retain source order through this index"
    )]
    pub(crate) fn source_index(&self) -> usize {
        self.source_index
    }
}

impl VocabVariantPlan {
    pub(crate) fn name(&self) -> &syn::Ident {
        &self.name
    }

    pub(crate) fn word(&self) -> &syn::LitStr {
        &self.word
    }
}

impl BindingBuildPlan {
    fn from_source(source: &crate::model::BuildLeaf) -> syn::Result<Self> {
        let syn::Pat::TupleStruct(pattern) = &source.pattern else {
            return Err(syn::Error::new_spanned(
                &source.pattern,
                "sealed binding build pattern kind is inconsistent",
            ));
        };
        let variant = pattern
            .path
            .segments
            .last()
            .ok_or_else(|| {
                syn::Error::new_spanned(
                    &source.pattern,
                    "sealed binding build pattern path is empty",
                )
            })?
            .ident
            .clone();
        let slots = pattern
            .elems
            .iter()
            .map(|slot| match slot {
                syn::Pat::Ident(slot) => Ok(slot.ident.clone()),
                _ => Err(syn::Error::new_spanned(
                    slot,
                    "sealed binding build pattern slot kind is inconsistent",
                )),
            })
            .collect::<syn::Result<Vec<_>>>()?;
        let construct = BindingBuildExprPlan::from_source(&source.construct, true)?;
        let construct_is_direct_slot = construct.is_direct_slot();
        Ok(Self {
            variant,
            slots,
            construct,
            construct_is_direct_slot,
        })
    }

    pub(crate) fn variant(&self) -> &syn::Ident {
        &self.variant
    }

    pub(crate) fn slots(&self) -> &[syn::Ident] {
        &self.slots
    }

    pub(crate) fn recipe(&self) -> &BindingBuildExprPlan {
        &self.construct
    }

    pub(crate) fn construct_is_direct_slot(&self) -> bool {
        self.construct_is_direct_slot
    }
}

impl BindingBuildExprPlan {
    fn from_source(source: &syn::Expr, allow_calls: bool) -> syn::Result<Self> {
        match source {
            syn::Expr::Path(path) if path.qself.is_none() && path.path.segments.len() == 1 => {
                Ok(Self::Slot(identifier_key(&path.path.segments[0].ident)))
            }
            syn::Expr::Field(field) => Ok(Self::Field {
                base: Box::new(Self::from_source(&field.base, false)?),
                member: field.member.clone(),
            }),
            syn::Expr::Call(call) if allow_calls => {
                let syn::Expr::Path(function) = &*call.func else {
                    return Err(syn::Error::new_spanned(
                        &call.func,
                        "sealed binding build call target kind is inconsistent",
                    ));
                };
                if function.qself.is_some() || function.path.segments.len() <= 1 {
                    return Err(syn::Error::new_spanned(
                        &call.func,
                        "sealed binding build call target path is inconsistent",
                    ));
                }
                Ok(Self::Call {
                    function: function.path.clone(),
                    arguments: call
                        .args
                        .iter()
                        .map(|argument| Self::from_source(argument, true))
                        .collect::<syn::Result<Vec<_>>>()?,
                })
            }
            syn::Expr::Paren(paren) => Ok(Self::Parenthesized(Box::new(Self::from_source(
                &paren.expr,
                allow_calls,
            )?))),
            _ => Err(syn::Error::new_spanned(
                source,
                "sealed binding build expression kind is inconsistent",
            )),
        }
    }

    fn is_direct_slot(&self) -> bool {
        match self {
            Self::Slot(_) => true,
            Self::Parenthesized(inner) => inner.is_direct_slot(),
            Self::Field { .. } | Self::Call { .. } => false,
        }
    }
}

impl ContextIdentityPlan {
    pub(crate) fn variant(&self) -> &syn::Ident {
        &self.variant
    }

    pub(crate) fn accessor(&self) -> &syn::Ident {
        &self.accessor
    }
}

impl BindingTraversalPlan {
    pub(crate) fn mode(&self) -> crate::model::VisitMode {
        self.mode
    }

    pub(crate) fn argument(&self) -> &str {
        &self.argument
    }

    pub(crate) fn fields(&self) -> &[TraversalFieldPlan] {
        &self.fields
    }

    pub(crate) fn recipe(&self) -> &BindingTraversalRecipe {
        &self.recipe
    }

    pub(crate) fn leaf_callbacks(&self) -> &[LeafCallbackPlan] {
        &self.leaf_callbacks
    }
}

impl TraversalCallPlan {
    fn from_source(source: &crate::model::TraversalCall) -> syn::Result<Self> {
        Ok(Self {
            callback: source.callback.clone(),
            mode: source.mode,
            value: TraversalValuePlan::from_source(&source.value)?,
        })
    }

    pub(crate) fn callback(&self) -> &syn::Path {
        &self.callback
    }

    pub(crate) fn mode(&self) -> crate::model::VisitMode {
        self.mode
    }

    pub(crate) fn value(&self) -> &TraversalValuePlan {
        &self.value
    }

    pub(crate) fn mentions(&self, name: &str) -> bool {
        self.value.mentions(name)
    }
}

impl TraversalFieldPlan {
    pub(crate) fn name(&self) -> &syn::Ident {
        &self.name
    }
}

impl TraversalBranchPlan {
    fn from_source(source: &crate::model::TraversalBranch) -> syn::Result<Self> {
        Ok(Self {
            value: TraversalValuePlan::from_source(&source.value)?,
            arms: source
                .arms
                .iter()
                .map(TraversalBranchArmPlan::from_source)
                .collect::<syn::Result<Vec<_>>>()?,
        })
    }

    pub(crate) fn value(&self) -> &TraversalValuePlan {
        &self.value
    }

    pub(crate) fn arms(&self) -> &[TraversalBranchArmPlan] {
        &self.arms
    }
}

impl TraversalBranchArmPlan {
    fn from_source(source: &crate::model::TraversalBranchArm) -> syn::Result<Self> {
        Ok(Self {
            variant: source.variant.clone(),
            binding: source.binding.clone(),
            call: TraversalCallPlan::from_source(&source.call)?,
        })
    }

    pub(crate) fn variant(&self) -> &syn::Path {
        &self.variant
    }

    pub(crate) fn binding(&self) -> &syn::Ident {
        &self.binding
    }

    pub(crate) fn call(&self) -> &TraversalCallPlan {
        &self.call
    }
}

impl TraversalValuePlan {
    fn from_source(source: &syn::Expr) -> syn::Result<Self> {
        match source {
            syn::Expr::Path(path)
                if path.qself.is_none()
                    && path.path.leading_colon.is_none()
                    && path.path.segments.len() == 1 =>
            {
                let root =
                    path.path.segments.first().ok_or_else(|| {
                        syn::Error::new(source.span(), "traversal root is absent")
                    })?;
                Ok(Self::Root(identifier_key(&root.ident)))
            }
            syn::Expr::Field(field) => Ok(Self::Field {
                base: Box::new(Self::from_source(&field.base)?),
                member: field.member.clone(),
            }),
            syn::Expr::Paren(paren) => Ok(Self::Parenthesized(Box::new(Self::from_source(
                &paren.expr,
            )?))),
            _ => Err(syn::Error::new(
                source.span(),
                "sealed traversal value is outside the validated field-path grammar",
            )),
        }
    }

    pub(crate) fn mentions(&self, name: &str) -> bool {
        match self {
            Self::Root(root) => root == name,
            Self::Field { base, .. } | Self::Parenthesized(base) => base.mentions(name),
        }
    }
}

impl LeafCallbackPlan {
    pub(crate) fn name(&self) -> &syn::Ident {
        &self.name
    }

    pub(crate) fn value_type(&self) -> &syn::Type {
        &self.value_type
    }

    pub(crate) fn mode(&self) -> crate::model::VisitMode {
        self.mode
    }
}

fn binding_value_type_name(value_type: &syn::Type) -> syn::Result<syn::Ident> {
    let syn::Type::Path(value_type) = value_type else {
        return Err(syn::Error::new_spanned(
            value_type,
            "sealed binding value type kind is inconsistent",
        ));
    };
    value_type
        .path
        .segments
        .last()
        .map(|segment| segment.ident.clone())
        .ok_or_else(|| {
            syn::Error::new_spanned(value_type, "sealed binding value type path is empty")
        })
}

fn default_leaf_argument(name: &str) -> String {
    let snake = snake_case(name);
    if snake.ends_with("_word") {
        "word".to_owned()
    } else if snake.ends_with("_spelling") {
        "spelling".to_owned()
    } else if let Some(prefix) = snake.strip_suffix("_lexeme") {
        prefix.to_owned()
    } else if snake.ends_with("_identity") {
        "identity".to_owned()
    } else if snake.ends_with("_number") {
        "number".to_owned()
    } else {
        snake
    }
}

impl RootPlan {
    #[allow(
        dead_code,
        reason = "future emitters retain source order through this index"
    )]
    pub(crate) fn source_index(&self) -> usize {
        self.source_index
    }

    pub(crate) fn category(&self) -> &str {
        &self.category
    }

    pub(crate) fn is_parse_entry(&self) -> bool {
        self.parse_entry
    }

    pub(crate) fn is_render_entry(&self) -> bool {
        self.render_entry
    }

    pub(crate) fn punctuation(&self) -> &str {
        &self.punctuation
    }
}

#[cfg(test)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct SemanticSnapshot {
    pub declaration_keys: Vec<(DeclarationKind, String)>,
    pub constructions: Vec<(String, String, String, Vec<String>)>,
    pub terminals: Vec<(String, String, Vec<String>)>,
    pub roots: Vec<(String, bool, bool)>,
    pub feature_equations: Vec<(String, String)>,
    pub boxed_fields: Vec<(String, String)>,
    pub dynamic_number_constructions: Vec<String>,
}
