use std::collections::HashMap;
use std::collections::HashSet;

use proc_macro2::Span;
use syn::spanned::Spanned;

use crate::feature;
use crate::feature::Feature;
use crate::feature::FeatureExpr;
use crate::feature::FeaturePlace;
use crate::identifier::key as identifier_key;
use crate::model::Declaration;
use crate::model::Declarations;
use crate::model::Form;
use crate::model::FormAtom;
use crate::model::VerbOperand;
#[cfg(test)]
use crate::plan::DeclarationKey;
#[cfg(test)]
use crate::plan::DeclarationKind;
use crate::validate::AtomContribution;
use crate::validate::CategoryRenderCapability;
use crate::validate::ConstructionContribution;
use crate::validate::ContributionInventory;
use crate::validate::TerminalContribution;

/// The one sealed semantic authority produced after validation succeeds.
#[derive(Debug)]
pub(crate) struct SemanticPlan {
    source: Declarations,
    constructions: Vec<ConstructionPlan>,
    terminals: Vec<TerminalPlan>,
    roots: Vec<RootPlan>,
    features: FeaturePlan,
    contributions: ContributionInventory,
}

#[derive(Debug)]
#[allow(
    dead_code,
    reason = "sealed rows are the next generation phase's semantic input"
)]
pub(crate) struct ConstructionPlan {
    source_index: usize,
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
    atoms: Vec<AtomPlan>,
}

#[derive(Debug, Clone)]
pub(crate) enum AtomPlan {
    Literal(String),
    Category { role: String, category: String },
    Lex { role: String, terminal: String },
    Identity { role: String, terminal: String },
    Noun { role: String, terminal: String },
    VerbFixed { terminal: String, variant: String },
}

#[derive(Debug)]
#[allow(
    dead_code,
    reason = "sealed rows are the next generation phase's semantic input"
)]
pub(crate) enum TerminalPlan {
    Vocab(VocabPlan),
    Lexeme(LexemePlan),
    Binding(BindingPlan),
}

pub(crate) enum AtomTerminal<'a> {
    Vocab(&'a crate::Vocab),
    Binding(&'a crate::TerminalBinding),
}

#[derive(Debug)]
#[allow(
    dead_code,
    reason = "sealed rows are the next generation phase's semantic input"
)]
pub(crate) struct VocabPlan {
    source_index: usize,
    terminal: TerminalContribution,
}

#[derive(Debug)]
#[allow(
    dead_code,
    reason = "sealed rows are the next generation phase's semantic input"
)]
pub(crate) struct LexemePlan {
    source_index: usize,
    terminal: TerminalContribution,
}

#[derive(Debug)]
#[allow(
    dead_code,
    reason = "sealed rows are the next generation phase's semantic input"
)]
pub(crate) struct BindingPlan {
    source_index: usize,
    terminal: TerminalContribution,
    kind: crate::model::TerminalBindingKind,
    stored_spelling: bool,
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
        source: Declarations,
        boxed_fields: HashSet<(String, String)>,
        dynamic_numbers: HashSet<String>,
        category_reads: HashMap<String, HashSet<Feature>>,
        equations: HashMap<String, Vec<feature::FeatureEquation>>,
        resolutions: HashMap<String, HashMap<feature::FeaturePlace, feature::FeatureResolution>>,
        category_render: HashMap<String, CategoryRenderCapability>,
        contributions: ContributionInventory,
    ) -> syn::Result<Self> {
        let construction_indexes = source
            .declarations
            .iter()
            .enumerate()
            .filter_map(|(index, declaration)| {
                matches!(declaration, Declaration::Construction(_)).then_some(index)
            })
            .collect::<Vec<_>>();
        if construction_indexes.len() != contributions.constructions().len() {
            return Err(sealed_error("construction contribution count"));
        }
        let constructions = construction_indexes
            .into_iter()
            .enumerate()
            .map(|(record_index, source_index)| {
                let record = contributions
                    .constructions()
                    .get(record_index)
                    .ok_or_else(|| sealed_error("construction contribution"))?;
                let Some(Declaration::Construction(construction)) =
                    source.declarations.get(source_index)
                else {
                    return Err(sealed_error("construction source"));
                };
                ConstructionPlan::from_record(source_index, construction, record)
            })
            .collect::<syn::Result<Vec<_>>>()?;
        let number_carry_categories = number_carry_categories(&constructions, &equations);

        let terminal_count = source
            .declarations
            .iter()
            .filter(|declaration| {
                matches!(
                    declaration,
                    Declaration::Vocab(_)
                        | Declaration::Lexeme(_)
                        | Declaration::Codec(_)
                        | Declaration::Identity(_)
                )
            })
            .count();
        if terminal_count != contributions.terminals().len() {
            return Err(sealed_error("terminal contribution count"));
        }
        let mut terminal_index = 0;
        let terminals = source.declarations.iter().enumerate().filter_map(|(source_index, declaration)| match declaration {
                Declaration::Vocab(_) => { let terminal = contributions.terminals().get(terminal_index)?; terminal_index += 1; Some(TerminalPlan::Vocab(VocabPlan { source_index, terminal: terminal.clone() })) }
                Declaration::Lexeme(_) => { let terminal = contributions.terminals().get(terminal_index)?; terminal_index += 1; Some(TerminalPlan::Lexeme(LexemePlan { source_index, terminal: terminal.clone() })) }
                Declaration::Codec(_) | Declaration::Identity(_) => { let terminal = contributions.terminals().get(terminal_index)?; terminal_index += 1; Some({
                        let (Declaration::Codec(binding) | Declaration::Identity(binding)) = declaration else {
                            return None;
                        };
                        TerminalPlan::Binding(BindingPlan {
                            source_index,
                            terminal: terminal.clone(),
                            kind: binding.kind,
                            stored_spelling: matches!(
                                binding.render,
                                Some(crate::model::RenderBinding::ContextIdentity(ref arms)) if arms.len() >= 2
                            ),
                        }) }) }
                Declaration::Construction(_) | Declaration::Root(_) => None,
            })
            .collect();

        let root_indexes = source
            .declarations
            .iter()
            .enumerate()
            .filter_map(|(index, declaration)| {
                matches!(declaration, Declaration::Root(_)).then_some(index)
            })
            .collect::<Vec<_>>();
        if root_indexes.len() != contributions.roots().len() {
            return Err(sealed_error("root contribution count"));
        }
        let roots = root_indexes
            .into_iter()
            .enumerate()
            .map(|(root_index, source_index)| {
                let root = contributions
                    .roots()
                    .get(root_index)
                    .ok_or_else(|| sealed_error("root contribution"))?;
                let Declaration::Root(source) = source.source_at(source_index) else {
                    return Err(sealed_error("root source kind"));
                };
                Ok(RootPlan {
                    source_index,
                    category: root.category().to_owned(),
                    punctuation: source.punctuation.value(),
                    parse_entry: root.is_parse_entry(),
                    render_entry: root.is_render_entry(),
                })
            })
            .collect::<syn::Result<Vec<_>>>()?;

        Ok(Self {
            source,
            constructions,
            terminals,
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
            contributions,
        })
    }

    #[cfg(test)]
    pub(crate) fn source(&self) -> &Declarations {
        &self.source
    }

    pub(crate) fn declaration_count(&self) -> usize {
        self.source.declarations.len()
    }

    #[allow(
        dead_code,
        reason = "sealed rows are the next generation phase's semantic input"
    )]
    pub(crate) fn constructions(&self) -> &[ConstructionPlan] {
        &self.constructions
    }

    #[allow(
        dead_code,
        reason = "sealed rows are the next generation phase's semantic input"
    )]
    pub(crate) fn terminals(&self) -> &[TerminalPlan] {
        &self.terminals
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

    pub(crate) fn contributions(&self) -> &ContributionInventory {
        &self.contributions
    }

    #[allow(
        dead_code,
        reason = "future emitters resolve source rows through their owning plan"
    )]
    pub(crate) fn construction_source(
        &self,
        row: &ConstructionPlan,
    ) -> syn::Result<&crate::Construction> {
        match self.source.declarations.get(row.source_index) {
            Some(Declaration::Construction(value)) => Ok(value),
            _ => Err(sealed_error("construction source")),
        }
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
    pub(crate) fn test_only_replace_vocab_spelling(
        &mut self,
        vocabulary: &str,
        variant: &str,
        spelling: &str,
    ) {
        let vocab = self
            .source
            .declarations
            .iter_mut()
            .find_map(|declaration| match declaration {
                Declaration::Vocab(vocab) if identifier_key(&vocab.name) == vocabulary => {
                    Some(vocab)
                }
                Declaration::Construction(_)
                | Declaration::Vocab(_)
                | Declaration::Lexeme(_)
                | Declaration::Codec(_)
                | Declaration::Identity(_)
                | Declaration::Root(_) => None,
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
    pub(crate) fn test_only_mispoint_construction_source(&mut self, construction_id: &str) {
        let root_index = self
            .source
            .declarations
            .iter()
            .position(|declaration| matches!(declaration, Declaration::Root(_)))
            .expect("test root is present");
        self.constructions
            .iter_mut()
            .find(|row| row.construction_id == construction_id)
            .expect("test construction is present")
            .source_index = root_index;
    }

    #[cfg(test)]
    pub(crate) fn test_only_mispoint_vocab_source(&mut self, name: &str) {
        let root_index = self
            .source
            .declarations
            .iter()
            .position(|declaration| matches!(declaration, Declaration::Root(_)))
            .expect("test root is present");
        let row = self
            .terminals
            .iter_mut()
            .find_map(|terminal| match terminal {
                TerminalPlan::Vocab(row) if row.terminal.name() == name => Some(row),
                TerminalPlan::Vocab(_) | TerminalPlan::Lexeme(_) | TerminalPlan::Binding(_) => None,
            })
            .expect("test vocabulary is present");
        row.source_index = root_index;
    }

    #[cfg(test)]
    pub(crate) fn test_only_seal_construction_atoms(
        form: &Form,
        atoms: &[AtomContribution],
    ) -> syn::Result<Vec<AtomPlan>> {
        seal_atoms(form, atoms)
    }

    #[allow(
        dead_code,
        reason = "future emitters resolve source rows through their owning plan"
    )]
    pub(crate) fn vocab_source(&self, row: &VocabPlan) -> syn::Result<&crate::Vocab> {
        match self.source.declarations.get(row.source_index) {
            Some(Declaration::Vocab(value)) => Ok(value),
            _ => Err(sealed_error("vocabulary source")),
        }
    }

    #[allow(
        dead_code,
        reason = "future emitters resolve source rows through their owning plan"
    )]
    pub(crate) fn lexeme_source(&self, row: &LexemePlan) -> syn::Result<&crate::Lexeme> {
        match self.source.declarations.get(row.source_index) {
            Some(Declaration::Lexeme(value)) => Ok(value),
            _ => Err(sealed_error("lexeme source")),
        }
    }

    #[allow(
        dead_code,
        reason = "future emitters resolve source rows through their owning plan"
    )]
    pub(crate) fn binding_source(&self, row: &BindingPlan) -> syn::Result<&crate::TerminalBinding> {
        match self.source.declarations.get(row.source_index) {
            Some(Declaration::Codec(value) | Declaration::Identity(value)) => Ok(value),
            _ => Err(sealed_error("terminal binding source")),
        }
    }

    #[allow(
        dead_code,
        reason = "future emitters resolve source rows through their owning plan"
    )]
    pub(crate) fn root_source(&self, row: &RootPlan) -> syn::Result<&crate::Root> {
        match self.source.declarations.get(row.source_index) {
            Some(Declaration::Root(value)) => Ok(value),
            _ => Err(sealed_error("root source")),
        }
    }

    pub(crate) fn parse_root(&self, category: &str) -> Option<&RootPlan> {
        self.roots
            .iter()
            .find(|root| root.parse_entry && root.category == category)
    }

    pub(crate) fn atom_terminal(&self, name: &str) -> syn::Result<AtomTerminal<'_>> {
        for terminal in &self.terminals {
            match terminal {
                TerminalPlan::Vocab(row) if row.terminal.name() == name => {
                    return self.vocab_source(row).map(AtomTerminal::Vocab);
                }
                TerminalPlan::Binding(row) if row.terminal.name() == name => {
                    return self.binding_source(row).map(AtomTerminal::Binding);
                }
                TerminalPlan::Vocab(_) | TerminalPlan::Lexeme(_) | TerminalPlan::Binding(_) => {}
            }
        }
        Err(sealed_error("resolved atom terminal"))
    }

    #[cfg(test)]
    pub(crate) fn snapshot(&self) -> SemanticSnapshot {
        let declaration_keys = self
            .source
            .declarations
            .iter()
            .map(DeclarationKey::from_source)
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
    fn from_record(
        source_index: usize,
        source: &crate::Construction,
        record: &ConstructionContribution,
    ) -> syn::Result<Self> {
        let atoms = seal_atoms(&source.form, record.atoms())?;
        Ok(Self {
            source_index,
            construction_id: record.construction_id().to_owned(),
            category: record.category().to_owned(),
            category_variant: record.category_variant().to_owned(),
            element_type: record.element_type().to_owned(),
            form: record.form().to_owned(),
            rule_id: record.rule_id().to_owned(),
            build_arm: record.build_arm().to_owned(),
            render_arm: record.render_arm().to_owned(),
            visitor_method: record.visitor_method().to_owned(),
            walker: record.walker().to_owned(),
            checked_constructor: source.checked.is_some(),
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
}

impl AtomPlan {
    fn from_source(source: &FormAtom, resolved: &AtomContribution) -> syn::Result<Self> {
        match (source, resolved) {
            (FormAtom::Literal(literal), AtomContribution::Literal) => {
                Ok(Self::Literal(literal.value()))
            }
            (FormAtom::Role(_), AtomContribution::Category { role, category }) => {
                Ok(Self::Category {
                    role: role.clone(),
                    category: category.clone(),
                })
            }
            (FormAtom::Lex(_), AtomContribution::Lex { role, terminal }) => Ok(Self::Lex {
                role: role.clone(),
                terminal: terminal.clone(),
            }),
            (FormAtom::Identity(_), AtomContribution::Identity { role, terminal }) => {
                Ok(Self::Identity {
                    role: role.clone(),
                    terminal: terminal.clone(),
                })
            }
            (FormAtom::Noun(_), AtomContribution::Noun { role, terminal }) => Ok(Self::Noun {
                role: role.clone(),
                terminal: terminal.clone(),
            }),
            (
                FormAtom::Verb(VerbOperand::Fixed(_)),
                AtomContribution::VerbFixed { terminal, variant },
            ) => Ok(Self::VerbFixed {
                terminal: terminal.clone(),
                variant: variant.clone(),
            }),
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
            Self::VerbFixed { terminal, variant } => format!("verb({terminal}::{variant})"),
        }
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
                | AtomPlan::VerbFixed { .. } => None,
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
    #[cfg(test)]
    fn terminal(&self) -> &TerminalContribution {
        match self {
            Self::Vocab(plan) => &plan.terminal,
            Self::Lexeme(plan) => &plan.terminal,
            Self::Binding(plan) => &plan.terminal,
        }
    }

    #[cfg(test)]
    fn snapshot(&self) -> (String, String, Vec<String>) {
        let terminal = self.terminal();
        let kind = match self {
            Self::Vocab(_) => "vocab",
            Self::Lexeme(_) => "lexeme",
            Self::Binding(_) if terminal.supports_identity_atom() => "identity",
            Self::Binding(_) => "codec",
        };
        let mut capabilities = Vec::new();
        if terminal.supports_lex_atom() {
            capabilities.push("lex".to_owned());
        }
        if terminal.supports_identity_atom() {
            capabilities.push("identity".to_owned());
        }
        if terminal.supports_noun_atom() {
            capabilities.push("noun".to_owned());
        }
        if terminal.supports_verb_atom() {
            capabilities.push("verb".to_owned());
        }
        if terminal.has_direct_render() {
            capabilities.push("render".to_owned());
        }
        if terminal.has_direct_build() {
            capabilities.push("build".to_owned());
        }
        if terminal.has_traversal() {
            capabilities.push("traversal".to_owned());
        }
        (terminal.name().to_owned(), kind.to_owned(), capabilities)
    }
}

impl VocabPlan {
    #[allow(
        dead_code,
        reason = "future emitters retain source order through this index"
    )]
    pub(crate) fn source_index(&self) -> usize {
        self.source_index
    }
}

impl LexemePlan {
    pub(crate) fn is_verb_provider(&self) -> bool {
        self.terminal.supports_verb_atom()
    }
    #[allow(
        dead_code,
        reason = "future emitters retain source order through this index"
    )]
    pub(crate) fn source_index(&self) -> usize {
        self.source_index
    }
}

impl BindingPlan {
    pub(crate) fn kind(&self) -> crate::model::TerminalBindingKind {
        self.kind
    }

    pub(crate) fn has_stored_spelling(&self) -> bool {
        self.stored_spelling
    }
    #[allow(
        dead_code,
        reason = "future emitters retain source order through this index"
    )]
    pub(crate) fn source_index(&self) -> usize {
        self.source_index
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
