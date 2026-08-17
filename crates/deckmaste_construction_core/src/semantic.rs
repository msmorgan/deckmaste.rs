use std::collections::HashMap;
use std::collections::HashSet;

use crate::feature;
use crate::feature::Feature;
use crate::identifier::key as identifier_key;
use crate::model::Declaration;
use crate::model::Declarations;
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
    atoms: Vec<AtomPlan>,
}

pub(crate) type AtomPlan = AtomContribution;

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
}

#[derive(Debug)]
#[allow(
    dead_code,
    reason = "sealed rows are the next generation phase's semantic input"
)]
pub(crate) struct RootPlan {
    source_index: usize,
    category: String,
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
    ) -> Self {
        let construction_indexes = source
            .declarations
            .iter()
            .enumerate()
            .filter_map(|(index, declaration)| {
                matches!(declaration, Declaration::Construction(_)).then_some(index)
            })
            .collect::<Vec<_>>();
        let constructions = construction_indexes
            .into_iter()
            .zip(contributions.constructions())
            .map(|(source_index, record)| ConstructionPlan::from_record(source_index, record))
            .collect();

        let mut terminal_records = contributions.terminals().iter();
        let terminals = source
            .declarations
            .iter()
            .enumerate()
            .filter_map(|(source_index, declaration)| match declaration {
                Declaration::Vocab(_) => terminal_records.next().map(|terminal| {
                    TerminalPlan::Vocab(VocabPlan {
                        source_index,
                        terminal: terminal.clone(),
                    })
                }),
                Declaration::Lexeme(_) => terminal_records.next().map(|terminal| {
                    TerminalPlan::Lexeme(LexemePlan {
                        source_index,
                        terminal: terminal.clone(),
                    })
                }),
                Declaration::Codec(_) | Declaration::Identity(_) => {
                    terminal_records.next().map(|terminal| {
                        TerminalPlan::Binding(BindingPlan {
                            source_index,
                            terminal: terminal.clone(),
                        })
                    })
                }
                Declaration::Construction(_) | Declaration::Root(_) => None,
            })
            .collect();
        debug_assert!(terminal_records.next().is_none());

        let root_indexes = source
            .declarations
            .iter()
            .enumerate()
            .filter_map(|(index, declaration)| {
                matches!(declaration, Declaration::Root(_)).then_some(index)
            })
            .collect::<Vec<_>>();
        let roots = root_indexes
            .into_iter()
            .zip(contributions.roots())
            .map(|(source_index, root)| RootPlan {
                source_index,
                category: root.category().to_owned(),
                parse_entry: root.is_parse_entry(),
                render_entry: root.is_render_entry(),
            })
            .collect();

        Self {
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
            },
            contributions,
        }
    }

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

    pub(crate) fn contributions(&self) -> &ContributionInventory {
        &self.contributions
    }

    #[allow(
        dead_code,
        reason = "future emitters resolve source rows through their owning plan"
    )]
    pub(crate) fn construction_source(&self, row: &ConstructionPlan) -> &crate::Construction {
        match self.source.source_at(row.source_index) {
            Declaration::Construction(value) => value,
            _ => unreachable!("construction semantic row points at its construction source"),
        }
    }

    #[cfg(test)]
    pub(crate) fn test_only_replace_literal(
        &mut self,
        construction_id: &str,
        atom_index: usize,
        literal: &str,
    ) {
        let construction = self
            .constructions
            .iter()
            .find(|row| row.construction_id == construction_id)
            .expect("test construction is present");
        let Declaration::Construction(source) =
            &mut self.source.declarations[construction.source_index]
        else {
            unreachable!("construction semantic row points at its construction source");
        };
        source.form.atoms[atom_index] =
            crate::FormAtom::Literal(syn::LitStr::new(literal, proc_macro2::Span::call_site()));
    }

    #[allow(
        dead_code,
        reason = "future emitters resolve source rows through their owning plan"
    )]
    pub(crate) fn vocab_source(&self, row: &VocabPlan) -> &crate::Vocab {
        match self.source.source_at(row.source_index) {
            Declaration::Vocab(value) => value,
            _ => unreachable!("vocab semantic row points at its vocab source"),
        }
    }

    #[allow(
        dead_code,
        reason = "future emitters resolve source rows through their owning plan"
    )]
    pub(crate) fn lexeme_source(&self, row: &LexemePlan) -> &crate::Lexeme {
        match self.source.source_at(row.source_index) {
            Declaration::Lexeme(value) => value,
            _ => unreachable!("lexeme semantic row points at its lexeme source"),
        }
    }

    #[allow(
        dead_code,
        reason = "future emitters resolve source rows through their owning plan"
    )]
    pub(crate) fn binding_source(&self, row: &BindingPlan) -> &crate::TerminalBinding {
        match self.source.source_at(row.source_index) {
            Declaration::Codec(value) | Declaration::Identity(value) => value,
            _ => unreachable!("binding semantic row points at its binding source"),
        }
    }

    #[allow(
        dead_code,
        reason = "future emitters resolve source rows through their owning plan"
    )]
    pub(crate) fn root_source(&self, row: &RootPlan) -> &crate::Root {
        match self.source.source_at(row.source_index) {
            Declaration::Root(value) => value,
            _ => unreachable!("root semantic row points at its root source"),
        }
    }

    pub(crate) fn parse_root(&self, category: &str) -> Option<&RootPlan> {
        self.roots
            .iter()
            .find(|root| root.parse_entry && root.category == category)
    }

    pub(crate) fn vocab(&self, name: &str) -> syn::Result<&crate::Vocab> {
        self.terminals
            .iter()
            .find_map(|terminal| match terminal {
                TerminalPlan::Vocab(row)
                    if identifier_key(&self.vocab_source(row).name) == name =>
                {
                    Some(self.vocab_source(row))
                }
                TerminalPlan::Vocab(_) | TerminalPlan::Lexeme(_) | TerminalPlan::Binding(_) => None,
            })
            .ok_or_else(|| {
                syn::Error::new(
                    proc_macro2::Span::call_site(),
                    "sealed semantic plan has no resolved vocabulary",
                )
            })
    }

    pub(crate) fn binding(&self, name: &str) -> syn::Result<&crate::TerminalBinding> {
        self.terminals
            .iter()
            .find_map(|terminal| match terminal {
                TerminalPlan::Binding(row)
                    if identifier_key(&self.binding_source(row).name) == name =>
                {
                    Some(self.binding_source(row))
                }
                TerminalPlan::Vocab(_) | TerminalPlan::Lexeme(_) | TerminalPlan::Binding(_) => None,
            })
            .ok_or_else(|| {
                syn::Error::new(
                    proc_macro2::Span::call_site(),
                    "sealed semantic plan has no resolved terminal binding",
                )
            })
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
                    plan.atoms.iter().map(AtomContribution::snapshot).collect(),
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
    fn from_record(source_index: usize, record: &ConstructionContribution) -> Self {
        Self {
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
            atoms: record.atoms().to_vec(),
        }
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
    #[allow(
        dead_code,
        reason = "future emitters retain source order through this index"
    )]
    pub(crate) fn source_index(&self) -> usize {
        self.source_index
    }
}

impl BindingPlan {
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
