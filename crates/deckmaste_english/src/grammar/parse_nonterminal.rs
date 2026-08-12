#![allow(
    dead_code,
    reason = "diagnostic parse modes expose a broader category inventory than production callers use"
)]

use super::AdjectivePhrase;
use super::BestParse;
#[cfg(test)]
use super::CatalogKind;
use super::Catalogs;
use super::ChartResult;
use super::ChartStats;
use super::Clause;
use super::Conjunction;
use super::EnglishGrammar;
use super::EnglishLexicalSlot;
use super::EnglishSurfaceWitness;
use super::Features;
use super::ForestError;
use super::ForestStats;
use super::ForestSymbol;
use super::GrammarError;
use super::MeaningKey;
use super::NodeId;
use super::NominalAttachmentPhase;
use super::Nonterminal;
use super::NounPhrase;
use super::OpacityMode;
use super::ParseCost;
use super::ParseForest;
use super::PrepositionalPhrase;
use super::Quantity;
use super::RegistrationOrder;
use super::SelfReference;
use super::SimpleClause;
use super::Span;
use super::Token;
use super::collapse_full_names;
use super::lex;
use super::lowering::Lowered;
use super::lowering::lower;
use super::lowering::selected_rule_children;
use super::parse_chart;
use crate::construction::ConstructionAlternative;
use crate::construction::ConstructionDecision;

type EnglishChart =
    ChartResult<Nonterminal, EnglishLexicalSlot, Features, MeaningKey, EnglishSurfaceWitness>;
pub(super) type EnglishForest =
    ParseForest<Nonterminal, EnglishLexicalSlot, Features, MeaningKey, EnglishSurfaceWitness>;

#[derive(Debug)]
pub(crate) struct ParsedNonterminal {
    pub(crate) chart: EnglishChart,
    root: NodeId,
    best: BestParse,
    construction_decisions: Vec<ConstructionDecision>,
    constituent_spans: Vec<Span>,
    quoted_ability_spans: Vec<Span>,
    syntax: Lowered,
    opacity_mode: OpacityMode,
}

impl ParsedNonterminal {
    pub(crate) fn noun_phrase(&self) -> Option<&NounPhrase> {
        match &self.syntax {
            Lowered::NounPhrase(noun_phrase) => Some(noun_phrase),
            _ => None,
        }
    }

    pub(crate) fn sentence(&self) -> Option<&crate::syntax::Sentence> {
        match &self.syntax {
            Lowered::Sentence(sentence) => Some(sentence),
            _ => None,
        }
    }

    pub(super) fn simple_clause(&self) -> Option<&SimpleClause> {
        match &self.syntax {
            Lowered::SimpleClause(clause) => Some(clause),
            _ => None,
        }
    }

    pub(crate) fn clause(&self) -> Option<&Clause> {
        match &self.syntax {
            Lowered::Clause(clause) => Some(clause),
            _ => None,
        }
    }

    #[cfg(test)]
    pub(crate) fn infinitive_clause(&self) -> Option<&crate::syntax::InfinitiveClause> {
        match &self.syntax {
            Lowered::InfinitiveClause(clause) => Some(clause),
            _ => None,
        }
    }

    #[cfg(test)]
    pub(crate) fn gerund_clause(&self) -> Option<&crate::syntax::GerundClause> {
        match &self.syntax {
            Lowered::GerundClause(clause) => Some(clause),
            _ => None,
        }
    }

    pub(crate) fn quantity(&self) -> Option<&Quantity> {
        match &self.syntax {
            Lowered::Quantity(quantity) => Some(quantity),
            _ => None,
        }
    }

    pub(crate) fn prepositional_phrase(&self) -> Option<&PrepositionalPhrase> {
        match &self.syntax {
            Lowered::PrepositionalPhrase(preposition) => Some(preposition),
            _ => None,
        }
    }

    pub(crate) fn adjective_phrase(&self) -> Option<&AdjectivePhrase> {
        match &self.syntax {
            Lowered::AdjectivePhrase(adjective) => Some(adjective),
            _ => None,
        }
    }

    #[cfg(test)]
    pub(crate) fn coordinated_modifier(&self) -> Option<&crate::syntax::CoordinatedModifier> {
        match &self.syntax {
            Lowered::CoordinatedModifier(value) => Some(value),
            _ => None,
        }
    }

    #[cfg(test)]
    pub(crate) fn verb_phrase(&self) -> Option<&super::VerbPhrase> {
        match &self.syntax {
            Lowered::VerbPhrase(predicate) => Some(predicate),
            _ => None,
        }
    }

    pub(crate) fn root_rule(&self) -> Option<usize> {
        let node = self.chart.forest.node(self.root);
        let alternative = self.best.alternative(self.root)?;
        node.alternatives
            .get(alternative)?
            .rule
            .map(crate::chart::RuleId::index)
    }

    pub(crate) fn root_tied_alternatives(&self) -> &[usize] {
        self.best.tied_alternatives(self.root)
    }

    pub(crate) fn root_equal_cost_alternatives(&self) -> &[usize] {
        self.best.equal_cost_alternatives(self.root)
    }

    pub(crate) fn root_production(&self) -> Option<crate::construction::ProductionId> {
        self.best.selected_production(self.root)
    }

    pub(crate) fn root_reason(&self) -> Option<crate::forest::SelectionReason> {
        self.best.reason(self.root)
    }

    pub(crate) fn construction_decisions(&self) -> &[ConstructionDecision] {
        &self.construction_decisions
    }

    pub(crate) fn cost(&self) -> ParseCost {
        self.best
            .node_cost(self.root)
            .expect("a selected root must have an aggregate cost")
    }

    pub(crate) const fn chart_stats(&self) -> ChartStats {
        self.chart.stats
    }

    pub(crate) fn forest_stats(&self) -> ForestStats {
        self.chart.forest.stats()
    }

    pub(crate) fn constituent_spans(&self) -> &[Span] {
        &self.constituent_spans
    }

    pub(crate) fn quoted_ability_spans(&self) -> &[Span] {
        &self.quoted_ability_spans
    }

    pub(crate) const fn opacity_mode(&self) -> OpacityMode {
        self.opacity_mode
    }
}

#[derive(Debug)]
pub(crate) enum ParseNonterminalError {
    Grammar(GrammarError),
    NoCompleteParse(Nonterminal),
    Forest(ForestError),
    Lowering,
}

pub(crate) fn parse_nonterminal(
    source: &str,
    catalogs: &Catalogs,
    nonterminal: Nonterminal,
) -> Result<ParsedNonterminal, ParseNonterminalError> {
    parse_nonterminal_with_self_reference(source, catalogs, nonterminal, &SelfReference::default())
}

pub(crate) fn parse_nonterminal_with_self_reference(
    source: &str,
    catalogs: &Catalogs,
    nonterminal: Nonterminal,
    self_reference: &SelfReference,
) -> Result<ParsedNonterminal, ParseNonterminalError> {
    parse_nonterminal_with_self_reference_and_activation(
        source,
        catalogs,
        nonterminal,
        self_reference,
        super::generated::GeneratedActivation::Production,
    )
}

pub(crate) fn parse_nonterminal_with_self_reference_and_activation(
    source: &str,
    catalogs: &Catalogs,
    nonterminal: Nonterminal,
    self_reference: &SelfReference,
    activation: super::generated::GeneratedActivation,
) -> Result<ParsedNonterminal, ParseNonterminalError> {
    let surface = lex(source);
    let tokens = collapse_full_names(source, surface.tokens, self_reference.full_name());
    match parse_nonterminal_with_mode_and_registration_order(
        source,
        catalogs,
        nonterminal,
        &tokens,
        OpacityMode::Exact,
        self_reference,
        RegistrationOrder::Normal,
        activation,
    ) {
        Ok(parsed) => Ok(parsed),
        Err(ParseNonterminalError::NoCompleteParse(_)) => {
            parse_nonterminal_with_mode_and_registration_order(
                source,
                catalogs,
                nonterminal,
                &tokens,
                OpacityMode::OpaqueNouns,
                self_reference,
                RegistrationOrder::Normal,
                activation,
            )
        }
        Err(error) => Err(error),
    }
}

pub(super) fn parse_nonterminal_with_mode(
    source: &str,
    catalogs: &Catalogs,
    nonterminal: Nonterminal,
    tokens: &[Token],
    opacity_mode: OpacityMode,
    self_reference: &SelfReference,
) -> Result<ParsedNonterminal, ParseNonterminalError> {
    parse_nonterminal_with_mode_and_registration_order(
        source,
        catalogs,
        nonterminal,
        tokens,
        opacity_mode,
        self_reference,
        RegistrationOrder::Normal,
        super::generated::GeneratedActivation::Production,
    )
}

#[allow(
    clippy::too_many_arguments,
    reason = "internal generated-assembly plumbing; the activation parameter is the last of a threaded diagnostic-mode set, not an independent knob"
)]
fn parse_nonterminal_with_mode_and_registration_order(
    source: &str,
    catalogs: &Catalogs,
    nonterminal: Nonterminal,
    tokens: &[Token],
    opacity_mode: OpacityMode,
    self_reference: &SelfReference,
    registration_order: RegistrationOrder,
    activation: super::generated::GeneratedActivation,
) -> Result<ParsedNonterminal, ParseNonterminalError> {
    let grammar = EnglishGrammar::with_opacity_mode_and_registration_order(
        source,
        catalogs,
        nonterminal,
        opacity_mode,
        self_reference.clone(),
        registration_order,
        activation,
    );
    let selected_registry = select_registry(activation);
    let registry = selected_registry.as_ref();
    let chart = parse_chart(&grammar, tokens).map_err(ParseNonterminalError::Grammar)?;
    if chart.roots.is_empty() {
        return Err(ParseNonterminalError::NoCompleteParse(nonterminal));
    }
    let forest = &chart.forest;
    let mut syntax = None;
    let (root, best) = chart
        .forest
        .best_root_matching(chart.roots.iter().copied(), registry, |root, best| {
            syntax = lower(&grammar, forest, root, best);
            syntax.is_some()
        })
        .map_err(ParseNonterminalError::Forest)?
        .ok_or(ParseNonterminalError::Lowering)?;
    let syntax = syntax.ok_or(ParseNonterminalError::Lowering)?;
    let mut constituent_spans = Vec::new();
    let mut quoted_ability_spans = Vec::new();
    collect_selected_spans(
        &chart.forest,
        root,
        &best,
        tokens,
        &mut constituent_spans,
        &mut quoted_ability_spans,
    );
    let mut construction_decisions = Vec::new();
    collect_construction_decisions(
        &grammar,
        &chart.forest,
        root,
        &best,
        tokens,
        registry,
        &mut construction_decisions,
    );
    construction_decisions.sort_by_key(|decision| {
        let span = decision.span();
        (span.start, span.end, decision.selected())
    });
    let mut lowered_coordination_spans = Vec::new();
    let mut lowered_determiner_spans = Vec::new();
    collect_lowered_coordination_spans(
        &grammar,
        &chart.forest,
        root,
        &best,
        tokens,
        &mut lowered_coordination_spans,
        &mut lowered_determiner_spans,
    );
    normalize_lowered_coordination_spans(
        &mut constituent_spans,
        &mut lowered_coordination_spans,
        &mut lowered_determiner_spans,
    );
    quoted_ability_spans.sort_unstable_by_key(|span| (span.start, span.end));
    quoted_ability_spans.dedup();
    Ok(ParsedNonterminal {
        chart,
        root,
        best,
        construction_decisions,
        constituent_spans,
        quoted_ability_spans,
        syntax,
        opacity_mode,
    })
}

/// The registry a parse uses: the production static, or an isolated generated
/// assembly used by registration-order and construction-law tests.
pub(super) enum SelectedRegistry {
    Static(&'static crate::construction::ConstructionRegistry),
    Owned(crate::construction::ConstructionRegistry),
}

impl SelectedRegistry {
    pub(super) fn as_ref(&self) -> &crate::construction::ConstructionRegistry {
        match self {
            Self::Static(registry) => registry,
            Self::Owned(registry) => registry,
        }
    }
}

pub(super) fn select_registry(
    activation: super::generated::GeneratedActivation,
) -> SelectedRegistry {
    if activation.is_production() {
        return SelectedRegistry::Static(super::construction::registry());
    }
    SelectedRegistry::Owned(
        super::construction::registry_from_groups(activation.groups())
            .expect("generated test assembly must be valid"),
    )
}

#[cfg(test)]
pub(super) fn parse_nonterminal_with_registration_order(
    source: &str,
    catalogs: &Catalogs,
    nonterminal: Nonterminal,
    registration_order: RegistrationOrder,
    activation: super::generated::GeneratedActivation,
) -> Result<ParsedNonterminal, ParseNonterminalError> {
    let self_reference = SelfReference::default();
    parse_nonterminal_with_self_reference_and_registration_order(
        source,
        catalogs,
        nonterminal,
        &self_reference,
        registration_order,
        activation,
    )
}

#[cfg(test)]
pub(super) fn parse_nonterminal_with_self_reference_and_registration_order(
    source: &str,
    catalogs: &Catalogs,
    nonterminal: Nonterminal,
    self_reference: &SelfReference,
    registration_order: RegistrationOrder,
    activation: super::generated::GeneratedActivation,
) -> Result<ParsedNonterminal, ParseNonterminalError> {
    let surface = lex(source);
    let tokens = collapse_full_names(source, surface.tokens, self_reference.full_name());
    parse_nonterminal_with_mode_and_registration_order(
        source,
        catalogs,
        nonterminal,
        &tokens,
        OpacityMode::Exact,
        self_reference,
        registration_order,
        activation,
    )
}

// Lexes and collapses identically to `parse_nonterminal_with_self_reference`
// above, and funnels into the same assembly function, but deliberately does
// NOT reproduce that entry point's retry: on `NoCompleteParse` the
// production path above retries once under `OpacityMode::OpaqueNouns`; this
// harness entry point always uses `OpacityMode::Exact` and returns whatever
// that single attempt produces. That is what makes
// `generated_parse_fails_without_matching_input` an exact assertion (a
// missing-comma input must fail, full stop) rather than an approximate one
// that would also have to account for an opaque-noun retry masking the
// failure. Do not "fix" this by adding the retry.
#[cfg(test)]
pub(crate) fn parse_nonterminal_with_activation(
    source: &str,
    catalogs: &Catalogs,
    nonterminal: Nonterminal,
    activation: super::generated::GeneratedActivation,
) -> Result<ParsedNonterminal, ParseNonterminalError> {
    let self_reference = SelfReference::default();
    let surface = lex(source);
    let tokens = collapse_full_names(source, surface.tokens, self_reference.full_name());
    parse_nonterminal_with_mode_and_registration_order(
        source,
        catalogs,
        nonterminal,
        &tokens,
        OpacityMode::Exact,
        &self_reference,
        RegistrationOrder::Normal,
        activation,
    )
}

fn collect_construction_decisions(
    grammar: &EnglishGrammar<'_, '_>,
    forest: &EnglishForest,
    node: NodeId,
    best: &BestParse,
    tokens: &[Token],
    registry: &crate::construction::ConstructionRegistry,
    decisions: &mut Vec<ConstructionDecision>,
) {
    let forest_node = forest.node(node);
    if matches!(forest_node.key.symbol, ForestSymbol::Nonterminal(_))
        && let Some(production) = best.decision_production(node)
        && let Some(span) = token_range_span(tokens, forest_node.key.start, forest_node.key.end)
        && let Some(cost) = best.node_cost(node)
        && let Some(reason) = best.reason(node)
        && let Some(family) = registry.family(production.construction)
    {
        let viable = best.tied_alternatives(node);
        let mut alternatives = best
            .equal_cost_alternatives(node)
            .iter()
            .filter_map(|&index| {
                let candidate = best.candidate_production(node, index)?;
                Some(ConstructionAlternative::new(
                    candidate,
                    cost,
                    !viable.contains(&index),
                ))
            })
            .collect::<Vec<_>>();
        alternatives.sort_by_key(|alternative| {
            (
                alternative.id(),
                alternative.production_ordinal(),
                alternative.is_dominated(),
            )
        });
        let evidence_value =
            selected_generated_evidence_value(grammar, forest, node, best, production);
        assert!(
            family.evidence().kind() == crate::construction::ConstructionEvidenceKind::Structural
                || evidence_value.is_some(),
            "selected semantic evidence for {} must resolve to its concrete generated edge",
            production.construction,
        );
        decisions.push(
            ConstructionDecision::new(span, production, family, cost, reason, alternatives)
                .with_evidence_value(evidence_value),
        );
    }

    let Some(alternative) = best.alternative(node) else {
        return;
    };
    let Some(alternative) = forest_node.alternatives.get(alternative) else {
        return;
    };
    for &child in &alternative.children {
        collect_construction_decisions(grammar, forest, child, best, tokens, registry, decisions);
    }
}

fn selected_generated_evidence_value(
    grammar: &EnglishGrammar<'_, '_>,
    forest: &EnglishForest,
    node: NodeId,
    best: &BestParse,
    production: crate::construction::ProductionId,
) -> Option<String> {
    use deckmaste_construction_compiler::runtime::EvidenceSourceData;

    if let Some((super::rules::RuleImpl::Generated(rule), children)) =
        selected_impl_and_children(grammar, forest, node, best)
    {
        let construction = rule
            .group
            .constructions
            .get(rule.construction)
            .expect("selected generated rule retains its declaration construction");
        let form_ordinal = construction
            .forms
            .get(rule.form)
            .expect("selected generated rule retains its declaration form")
            .ordinal;
        if construction.id == production.construction.as_str()
            && form_ordinal == production.ordinal
            && let Some(evidence) = construction.evidence
        {
            let evaluate = || -> Option<String> {
                match evidence.source {
                    EvidenceSourceData::Requirement(path) => {
                        let fields = selected_generated_field_features(forest, rule, &children)?;
                        let field_index = construction
                            .fields
                            .iter()
                            .position(|field| field.name == path)?;
                        let feature = fields.get(field_index)?.as_ref()?;
                        let requirement =
                            construction.requirements.iter().find_map(|requirement| {
                                predicate_for_path(&requirement.predicate, path)
                            })?;
                        requirement_evidence_value(path, feature, requirement)
                    }
                    EvidenceSourceData::Output(path) => {
                        output_evidence_value(path, forest.node(node).key.constituent_features()?)
                    }
                    EvidenceSourceData::Field(path) => {
                        let (field_name, feature_path) = path.split_once('.')?;
                        let fields = selected_generated_field_features(forest, rule, &children)?;
                        let field_index = construction
                            .fields
                            .iter()
                            .position(|field| field.name == field_name)?;
                        field_evidence_value(
                            field_name,
                            feature_path,
                            fields.get(field_index)?.as_ref()?,
                        )
                    }
                    EvidenceSourceData::Category => {
                        Some(format!("category={}", construction.category))
                    }
                }
            };
            return Some(evaluate().unwrap_or_else(|| {
                panic!(
                    "validated evidence source {:?} for {} did not evaluate on its selected chart edge",
                    evidence.source, construction.id,
                )
            }));
        }
    }

    let forest_node = forest.node(node);
    let alternative = forest_node.alternatives.get(best.alternative(node)?)?;
    alternative.children.iter().find_map(|&child| {
        selected_generated_evidence_value(grammar, forest, child, best, production)
    })
}

fn selected_generated_field_features<'a>(
    forest: &'a EnglishForest,
    rule: super::rules::GeneratedRuleRef,
    children: &[NodeId],
) -> Option<Vec<Option<&'a Features>>> {
    use deckmaste_construction_compiler::runtime::AtomData;
    use deckmaste_construction_compiler::runtime::FieldKindData;

    let construction = rule.group.constructions.get(rule.construction)?;
    let form = construction.forms.get(rule.form)?;
    let mut fields = vec![None; construction.fields.len()];
    let mut child_index = usize::from(matches!(
        rule.context,
        super::rules::GeneratedRuleContext::SharedPreposition
    ));
    for (atom_index, atom) in form.atoms.iter().enumerate() {
        if matches!(atom, AtomData::Literal(_)) {
            child_index += 1;
            continue;
        }
        let path = match atom {
            AtomData::Hole(path) | AtomData::Lexeme(path) | AtomData::Identity(path) => *path,
            AtomData::Literal(_) => unreachable!(),
        };
        let field_index = construction
            .fields
            .iter()
            .position(|field| field.name == path)?;
        if matches!(
            construction.fields[field_index].kind,
            FieldKindData::Sequence { .. }
        ) && rule.sequence_atoms & (1_u64 << atom_index) == 0
        {
            continue;
        }
        let child = *children.get(child_index)?;
        child_index += 1;
        fields[field_index] = forest.node(child).key.constituent_features();
    }
    (child_index == children.len()).then_some(fields)
}

fn predicate_for_path<'a>(
    predicate: &'a deckmaste_construction_compiler::runtime::PredicateData,
    path: &str,
) -> Option<&'a deckmaste_construction_compiler::runtime::PredicateData> {
    use deckmaste_construction_compiler::runtime::PredicateData;

    match predicate {
        PredicateData::LenAtLeast {
            path: candidate, ..
        }
        | PredicateData::LenIs {
            path: candidate, ..
        }
        | PredicateData::In {
            path: candidate, ..
        }
        | PredicateData::IsSome { path: candidate }
        | PredicateData::IsNone { path: candidate }
            if *candidate == path =>
        {
            Some(predicate)
        }
        PredicateData::All(predicates) | PredicateData::Any(predicates) => predicates
            .iter()
            .find_map(|predicate| predicate_for_path(predicate, path)),
        _ => None,
    }
}

fn requirement_evidence_value(
    path: &str,
    feature: &Features,
    predicate: &deckmaste_construction_compiler::runtime::PredicateData,
) -> Option<String> {
    let deckmaste_construction_compiler::runtime::PredicateData::In { allowed, .. } = predicate
    else {
        return None;
    };
    let actual = match feature {
        Features::Conjunction(Conjunction::And) => "And",
        Features::Conjunction(Conjunction::Or) => "Or",
        Features::Conjunction(Conjunction::Then) => "Then",
        Features::Conjunction(Conjunction::Plus) => "Plus",
        Features::Conjunction(Conjunction::AndOr) => "AndOr",
        _ => return None,
    };
    Some(format!(
        "{path}={actual};allowed=[{}];matched={}",
        allowed.join(","),
        allowed.contains(&actual),
    ))
}

fn output_evidence_value(path: &str, feature: &Features) -> Option<String> {
    if path == "relative_signature"
        && let Features::RelativeClause {
            gap,
            marker,
            antecedent_agreement,
            contraction,
            distributive_each,
            copular,
            object_gap_requires_rules_object,
            bare_copular_tail,
            ..
        } = feature
    {
        return Some(format!(
            "gap={gap:?};marker={marker:?};agreement={antecedent_agreement:?};contraction={contraction:?};distributive_each={distributive_each};copular={copular:?};rules_object={object_gap_requires_rules_object};bare_copular_tail={bare_copular_tail}"
        ));
    }
    if path == "object_category"
        && let Features::PrepositionalObject {
            object_category, ..
        } = feature
    {
        let value = match object_category {
            super::PrepositionalObjectCategory::NounPhrase => "NounPhrase",
            super::PrepositionalObjectCategory::PrepositionalPhrase => "PrepositionalPhrase",
            super::PrepositionalObjectCategory::GerundClause => "GerundClause",
            super::PrepositionalObjectCategory::Adverb => "Adverb",
        };
        return Some(format!("{path}={value}"));
    }
    let Features::Nominal { attachment, .. } = feature else {
        return None;
    };
    if path != "attachment" {
        return None;
    }
    let value = match attachment {
        NominalAttachmentPhase::Open => "open".to_owned(),
        NominalAttachmentPhase::Prepositional {
            nearer_relative_host,
        } => format!("prepositional(nearer_relative_host={nearer_relative_host})"),
        NominalAttachmentPhase::Relative => "relative".to_owned(),
        NominalAttachmentPhase::RulesObjectRelative => "rules_object_relative".to_owned(),
        NominalAttachmentPhase::RelativeBareCopula => "relative_bare_copula".to_owned(),
        NominalAttachmentPhase::ReducedRecipientPassive => "reduced_recipient_passive".to_owned(),
        NominalAttachmentPhase::PostpositiveAdjective => "postpositive_adjective".to_owned(),
        NominalAttachmentPhase::Comparison => "comparison".to_owned(),
    };
    Some(format!("{path}={value}"))
}

fn field_evidence_value(field: &str, path: &str, feature: &Features) -> Option<String> {
    let Features::VerbPhrase {
        frame,
        object,
        indirect_object,
        ..
    } = feature
    else {
        return None;
    };
    if path != "frame" {
        return None;
    }
    Some(format!(
        "{field}.{path}={{recipient_passive:{},direct_object:{},indirect_object:{indirect_object}}}",
        frame.is_recipient_passive(),
        object.has_direct_object(),
    ))
}

fn collect_selected_spans(
    forest: &EnglishForest,
    node: NodeId,
    best: &BestParse,
    tokens: &[Token],
    constituent_spans: &mut Vec<Span>,
    quoted_ability_spans: &mut Vec<Span>,
) {
    let forest_node = forest.node(node);
    if matches!(forest_node.key.symbol, ForestSymbol::Nonterminal(_))
        && forest_node.key.start < forest_node.key.end
        && let (Some(first), Some(last)) = (
            tokens.get(forest_node.key.start),
            tokens.get(forest_node.key.end - 1),
        )
    {
        constituent_spans.push(Span::new(first.span.start, last.span.end));
    }
    if matches!(
        forest_node.key.symbol,
        ForestSymbol::Lexical(EnglishLexicalSlot::QuotedAbility)
    ) && let Some(MeaningKey::QuotedAbility(span)) = forest_node.key.lexical_value()
    {
        quoted_ability_spans.push(*span);
    }
    let Some(alternative) = best.alternative(node) else {
        return;
    };
    let Some(alternative) = forest_node.alternatives.get(alternative) else {
        return;
    };
    for child in &alternative.children {
        collect_selected_spans(
            forest,
            *child,
            best,
            tokens,
            constituent_spans,
            quoted_ability_spans,
        );
    }
}

/// A shared determiner recovered while lowering supersedes the parser's
/// legacy binary attachment, so its nominal coordination needs a provenance
/// span of its own. Recording that structural edit here keeps the bracket dump
/// aligned with the returned AST without introducing broad grammar
/// predictions that perturb unrelated packed-forest ties.
fn collect_lowered_coordination_spans(
    grammar: &EnglishGrammar<'_, '_>,
    forest: &EnglishForest,
    node: NodeId,
    best: &BestParse,
    tokens: &[Token],
    spans: &mut Vec<Span>,
    determiner_spans: &mut Vec<Span>,
) {
    let Some((rule_impl, children)) = selected_impl_and_children(grammar, forest, node, best)
    else {
        return;
    };
    for child in &children {
        collect_lowered_coordination_spans(
            grammar,
            forest,
            *child,
            best,
            tokens,
            spans,
            determiner_spans,
        );
    }

    if let super::rules::RuleImpl::Generated(rule) = rule_impl {
        let Some(construction) = rule.group.constructions.get(rule.construction) else {
            return;
        };
        let combinator =
            super::generated::GeneratedFeatureCombinator::from_construction(construction);
        if combinator
            == Some(super::generated::GeneratedFeatureCombinator::SharedDeterminerCoordination)
            && let (Some(first), Some(rest)) = (children.get(1), children.get(2))
            && let Some(Lowered::NounPhrase(group_phrase)) = lower(grammar, forest, node, best)
            && let crate::syntax::NounPhraseKind::CoordinatedNominal(group) = group_phrase.kind()
        {
            let core_end = first_group_relative(group.complements())
                .as_ref()
                .and_then(|relative| {
                    find_selected_relative(grammar, forest, *rest, best, relative)
                        .map(|relative_node| forest.node(relative_node).key.start)
                })
                .unwrap_or(forest.node(*rest).key.end);
            if let Some(span) = token_range_span(tokens, forest.node(*first).key.start, core_end) {
                spans.push(span);
            }
        }
        if combinator
            == Some(super::generated::GeneratedFeatureCombinator::CompleteNounPhraseCoordination)
            && let (Some(first_node), Some(next_node)) = (children.first(), children.get(1))
            && let Some(Lowered::NounPhrase(before)) = lower(grammar, forest, *first_node, best)
            && let Some(Lowered::NounPhrase(after)) = lower(grammar, forest, node, best)
        {
            collect_shared_determiner_edit_spans(
                grammar,
                forest,
                node,
                *first_node,
                *next_node,
                &before,
                &after,
                best,
                tokens,
                spans,
                determiner_spans,
            );
        }
    }
}

#[allow(
    clippy::too_many_arguments,
    reason = "coordination provenance threads one selected structural edit through forest lookup"
)]
fn collect_shared_determiner_edit_spans(
    grammar: &EnglishGrammar<'_, '_>,
    forest: &EnglishForest,
    node: NodeId,
    first_node: NodeId,
    next_node: NodeId,
    before: &NounPhrase,
    after: &NounPhrase,
    best: &BestParse,
    tokens: &[Token],
    spans: &mut Vec<Span>,
    determiner_spans: &mut Vec<Span>,
) {
    let Some(edit) = find_shared_determiner_edit(before, after) else {
        return;
    };
    let Some(origin_node) =
        find_selected_noun_phrase(grammar, forest, first_node, best, &edit.determined_first)
    else {
        return;
    };
    let origin_start = forest.node(origin_node).key.start;
    let Some(determiner_end) = find_selected_determiner_end(
        grammar,
        forest,
        origin_node,
        best,
        origin_start,
        &edit.determiner,
    ) else {
        return;
    };
    if let Some(span) = token_range_span(tokens, origin_start, determiner_end) {
        determiner_spans.push(span);
    }
    if let Some(span) = token_range_span(tokens, origin_start, forest.node(node).key.end) {
        spans.push(span);
    }
    let relative_start = edit.trailing_relative.as_ref().and_then(|relative| {
        find_selected_relative(grammar, forest, next_node, best, relative)
            .map(|relative_node| forest.node(relative_node).key.start)
    });
    if let Some(relative_start) = relative_start
        && let Some(span) =
            token_range_span(tokens, forest.node(next_node).key.start, relative_start)
    {
        spans.push(span);
    }
    let core_end = relative_start.unwrap_or(forest.node(node).key.end);
    if let Some(span) = token_range_span(tokens, determiner_end, core_end) {
        spans.push(span);
    }
}

fn selected_impl_and_children(
    grammar: &EnglishGrammar<'_, '_>,
    forest: &EnglishForest,
    node: NodeId,
    best: &BestParse,
) -> Option<(super::rules::RuleImpl, Vec<NodeId>)> {
    let forest_node = forest.node(node);
    let alternative = forest_node.alternatives.get(best.alternative(node)?)?;
    let rule = alternative.rule?;
    let rule_impl = *grammar.impls.get(rule.index())?;
    let [intermediate] = alternative.children.as_slice() else {
        return None;
    };
    let mut children = Vec::new();
    selected_rule_children(forest, *intermediate, best, &mut children)?;
    Some((rule_impl, children))
}

#[derive(Debug)]
struct SharedDeterminerEdit {
    determined_first: NounPhrase,
    determiner: super::Determiner,
    trailing_relative: Option<super::RelativeClause>,
}

/// Finds the one subtree changed by generated coordination projection. The
/// comparison is semantic: it follows the final PP object or final outer
/// conjunct that lowering is allowed to rewrite, never a surface substring.
fn find_shared_determiner_edit(
    before: &NounPhrase,
    after: &NounPhrase,
) -> Option<SharedDeterminerEdit> {
    match (before.kind(), after.kind()) {
        (
            crate::syntax::NounPhraseKind::Nominal(original),
            crate::syntax::NounPhraseKind::CoordinatedNominal(group),
        ) => {
            let (determiner, first) =
                crate::constructions::nominal::parts_nominal_determiner(original)?;
            if group.determiner() != &determiner || group.first().as_ref() != &first {
                return None;
            }
            Some(SharedDeterminerEdit {
                determined_first: before.clone(),
                determiner,
                trailing_relative: first_group_relative(group.complements()),
            })
        }
        (
            crate::syntax::NounPhraseKind::CoordinatedNominal(original),
            crate::syntax::NounPhraseKind::CoordinatedNominal(group),
        ) if group.determiner() == original.determiner()
            && group.first() == original.first()
            && group.rest().len() == original.rest().len() + 1
            && group.rest().starts_with(original.rest()) =>
        {
            let determined_first = crate::constructions::nominal::build_nominal_determiner(
                original.determiner().clone(),
                original.first().as_ref().clone(),
            )
            .ok()?;
            Some(SharedDeterminerEdit {
                determined_first: crate::constructions::noun_phrase::build_noun_phrase_nominal(
                    determined_first,
                )
                .ok()?,
                determiner: original.determiner().clone(),
                trailing_relative: first_group_relative(group.complements()),
            })
        }
        (
            crate::syntax::NounPhraseKind::Nominal(before),
            crate::syntax::NounPhraseKind::Nominal(after),
        ) if before.determiner() == after.determiner()
            && before.modifiers() == after.modifiers()
            && before.head() == after.head()
            && before.complements().len() == after.complements().len()
            && !before.complements().is_empty()
            && before.complements()[..before.complements().len() - 1]
                == after.complements()[..after.complements().len() - 1] =>
        {
            let (
                super::NominalComplement::Prepositional(before),
                super::NominalComplement::Prepositional(after),
            ) = (before.complements().last()?, after.complements().last()?)
            else {
                return None;
            };
            if before.head().preposition != after.head().preposition {
                return None;
            }
            let (
                crate::syntax::PrepositionalObjectKind::NounPhrase(before),
                crate::syntax::PrepositionalObjectKind::NounPhrase(after),
            ) = (before.head().object.kind(), after.head().object.kind())
            else {
                return None;
            };
            find_shared_determiner_edit(before.as_ref(), after.as_ref())
        }
        (
            crate::syntax::NounPhraseKind::Coordinated(before),
            crate::syntax::NounPhraseKind::Coordinated(after),
        ) if before.first() == after.first()
            && before.rest().len() == after.rest().len()
            && !before.rest().is_empty()
            && before.rest()[..before.rest().len() - 1]
                == after.rest()[..after.rest().len() - 1] =>
        {
            let before = before.rest().last()?;
            let after = after.rest().last()?;
            if before.conjunction != after.conjunction {
                return None;
            }
            find_shared_determiner_edit(&before.phrase, &after.phrase)
        }
        _ => None,
    }
}

fn first_group_relative(complements: &[super::NominalComplement]) -> Option<super::RelativeClause> {
    match complements.first() {
        Some(super::NominalComplement::Relative(relative)) => Some(relative.clone()),
        _ => None,
    }
}

fn find_selected_noun_phrase(
    grammar: &EnglishGrammar<'_, '_>,
    forest: &EnglishForest,
    node: NodeId,
    best: &BestParse,
    target: &NounPhrase,
) -> Option<NodeId> {
    if matches!(
        lower(grammar, forest, node, best),
        Some(Lowered::NounPhrase(ref candidate)) if candidate == target
    ) {
        return Some(node);
    }
    let (_, children) = selected_impl_and_children(grammar, forest, node, best)?;
    children
        .into_iter()
        .find_map(|child| find_selected_noun_phrase(grammar, forest, child, best, target))
}

fn find_selected_determiner_end(
    grammar: &EnglishGrammar<'_, '_>,
    forest: &EnglishForest,
    node: NodeId,
    best: &BestParse,
    origin_start: usize,
    target: &super::Determiner,
) -> Option<usize> {
    let forest_node = forest.node(node);
    let mut end = (forest_node.key.start == origin_start
        && matches!(
            lower(grammar, forest, node, best),
            Some(Lowered::Determiner(ref candidate)) if candidate == target
        ))
    .then_some(forest_node.key.end);
    if let Some((_, children)) = selected_impl_and_children(grammar, forest, node, best) {
        for child in children {
            if let Some(candidate) =
                find_selected_determiner_end(grammar, forest, child, best, origin_start, target)
            {
                end = Some(end.map_or(candidate, |current| current.max(candidate)));
            }
        }
    }
    end
}

fn find_selected_relative(
    grammar: &EnglishGrammar<'_, '_>,
    forest: &EnglishForest,
    node: NodeId,
    best: &BestParse,
    target: &super::RelativeClause,
) -> Option<NodeId> {
    if matches!(
        lower(grammar, forest, node, best),
        Some(Lowered::RelativeClause(ref candidate)) if candidate == target
    ) {
        return Some(node);
    }
    let (_, children) = selected_impl_and_children(grammar, forest, node, best)?;
    children
        .into_iter()
        .find_map(|child| find_selected_relative(grammar, forest, child, best, target))
}

fn token_range_span(tokens: &[Token], start: usize, end: usize) -> Option<Span> {
    if start >= end {
        return None;
    }
    Some(Span::new(
        tokens.get(start)?.span.start,
        tokens.get(end.checked_sub(1)?)?.span.end,
    ))
}

fn normalize_lowered_coordination_spans(
    selected: &mut Vec<Span>,
    lowered: &mut Vec<Span>,
    determiner_spans: &mut Vec<Span>,
) {
    lowered.sort_unstable_by_key(|span| (span.start, std::cmp::Reverse(span.end)));
    lowered.dedup();
    let mut maximal = Vec::with_capacity(lowered.len());
    for span in lowered.drain(..) {
        if maximal
            .last()
            .is_some_and(|previous: &Span| previous.start == span.start && previous.end >= span.end)
        {
            continue;
        }
        maximal.push(span);
    }
    determiner_spans.sort_unstable_by_key(|span| (span.start, span.end));
    determiner_spans.dedup();
    selected.retain(|selected| {
        !determiner_spans.contains(selected)
            && maximal
                .iter()
                .all(|lowered| !constituent_spans_cross(*selected, *lowered))
    });
    selected.extend(maximal);
    selected.sort_unstable_by_key(|span| (span.start, span.end));
    selected.dedup();
}

fn constituent_spans_cross(left: Span, right: Span) -> bool {
    left.start < right.start && right.start < left.end && left.end < right.end
        || right.start < left.start && left.start < right.end && right.end < left.end
}

#[cfg(test)]
mod root_lowering_tests {
    use super::*;
    use crate::Numeral;
    use crate::features::NounCardinality;
    use crate::features::Number;
    use crate::syntax::ComparativeWord;
    use crate::syntax::IndependentClause;
    use crate::syntax::QuantityValue;

    /// The pre-ticket algorithm: choose exactly one root by cost and stable
    /// node order, then let a lowering decline fail the whole parse.
    fn parse_single_best_root(
        source: &str,
        catalogs: &Catalogs,
        nonterminal: Nonterminal,
    ) -> Result<ParsedNonterminal, ParseNonterminalError> {
        let surface = lex(source);
        let grammar = EnglishGrammar::with_opacity_mode(
            source,
            catalogs,
            nonterminal,
            OpacityMode::Exact,
            SelfReference::default(),
        );
        let chart =
            parse_chart(&grammar, &surface.tokens).map_err(ParseNonterminalError::Grammar)?;
        let (root, best) = chart
            .forest
            .best_root(
                chart.roots.iter().copied(),
                super::super::construction::registry(),
            )
            .map_err(ParseNonterminalError::Forest)?
            .ok_or(ParseNonterminalError::NoCompleteParse(nonterminal))?;
        let syntax =
            lower(&grammar, &chart.forest, root, &best).ok_or(ParseNonterminalError::Lowering)?;
        Ok(ParsedNonterminal {
            chart,
            root,
            best,
            construction_decisions: Vec::new(),
            constituent_spans: Vec::new(),
            quoted_ability_spans: Vec::new(),
            syntax,
            opacity_mode: OpacityMode::Exact,
        })
    }

    fn fixture_catalogs() -> Catalogs {
        Catalogs::default()
            .with_catalog(CatalogKind::CardType, ["Artifact", "Creature", "Land"])
            .with_catalog(CatalogKind::CreatureType, ["Goblin", "Human"])
    }

    #[test]
    fn contracted_relative_exact_alternative_lowers_and_reports_its_own_evidence() {
        let source = "that's attacking";
        let catalogs = fixture_catalogs();
        let surface = lex(source);
        let grammar = EnglishGrammar::with_opacity_mode(
            source,
            &catalogs,
            Nonterminal::RelativeClause,
            OpacityMode::Exact,
            SelfReference::default(),
        );
        let chart = parse_chart(&grammar, &surface.tokens).expect("relative chart builds");
        let mut remaining = 10_000;
        let mut found = false;
        for &root in &chart.roots {
            for selection in chart
                .forest
                .enumerate_selections(root, &mut remaining)
                .expect("relative exact alternatives stay within budget")
            {
                let Some(alternative_index) =
                    crate::forest::AlternativeSelection::alternative(&selection, root)
                else {
                    continue;
                };
                let Some(rule) = chart.forest.node(root).alternatives[alternative_index].rule
                else {
                    continue;
                };
                let Some(super::super::rules::RuleImpl::Generated(generated)) =
                    grammar.impls.get(rule.index()).copied()
                else {
                    continue;
                };
                let construction = &generated.group.constructions[generated.construction];
                if construction.id != "relative_subject_contracted_auxiliary" {
                    continue;
                }
                let declaration_evidence = construction
                    .evidence
                    .expect("contracted relative declares semantic evidence");
                assert_eq!(
                    declaration_evidence.label,
                    "decisive relative form signature",
                );
                assert!(matches!(
                    declaration_evidence.source,
                    deckmaste_construction_compiler::runtime::EvidenceSourceData::Output(
                        "relative_signature",
                    ),
                ));
                assert!(
                    matches!(
                        lower(&grammar, &chart.forest, root, &selection),
                        Some(Lowered::RelativeClause(_)),
                    ),
                    "the construction's exact alternative must lower through its own generated builder",
                );
                let evidence = output_evidence_value(
                    "relative_signature",
                    chart
                        .forest
                        .node(root)
                        .key
                        .constituent_features()
                        .expect("relative root retains typed features"),
                )
                .expect("relative construction evaluates its output evidence");
                assert!(evidence.contains("gap=Subject"), "{evidence}");
                assert!(evidence.contains("marker=That"), "{evidence}");
                assert!(evidence.contains("agreement=Some("), "{evidence}");
                assert!(
                    evidence.contains("contraction=SubjectAuxiliary"),
                    "{evidence}",
                );
                assert!(evidence.contains("distributive_each=false"), "{evidence}");
                assert!(evidence.contains("copular=NonCopular"), "{evidence}");
                found = true;
            }
        }
        assert!(
            found,
            "the packed contracted-auxiliary construction was not enumerated"
        );
    }

    #[test]
    fn sentence_root_reports_one_generated_family_and_declared_form_ordinals() {
        // Mutation caught: introduce another Sentence production.
        // Both real surfaces must select the same construction;
        // their independently declared ordinals preserve exact form identity.
        for (source, expected_ordinal) in [("Draw a card.", 0), ("Draw a card", 1)] {
            let parsed = parse_nonterminal(source, &fixture_catalogs(), Nonterminal::Sentence)
                .unwrap_or_else(|error| panic!("failed to parse {source:?}: {error:?}"));
            let decision = parsed
                .construction_decisions()
                .iter()
                .find(|decision| decision.selected().as_str() == "sentence")
                .unwrap_or_else(|| panic!("no sentence construction decision: {parsed:#?}"));
            assert_eq!(decision.backend(), crate::ConstructionBackend::Chart);
            assert_eq!(
                decision.selected_production_ordinal(),
                expected_ordinal,
                "{source:?}"
            );
        }
    }

    #[test]
    fn quantity_root_reports_the_generated_scalar_construction() {
        let parsed = parse_nonterminal("three", &fixture_catalogs(), Nonterminal::Quantity)
            .expect("the generated scalar quantity parses");
        assert!(matches!(
            parsed.quantity().map(|quantity| quantity.kind()),
            Some(crate::syntax::QuantityKind::Exact(
                super::super::NumberLiteral { value: 3, .. }
            ))
        ));
        let decision = parsed
            .construction_decisions()
            .iter()
            .find(|decision| decision.selected().as_str() == "quantity_exact")
            .unwrap_or_else(|| panic!("no generated quantity decision: {parsed:#?}"));
        assert_eq!(decision.backend(), crate::ConstructionBackend::Chart);
        assert_eq!(decision.selected_production_ordinal(), 0);
    }

    #[test]
    fn generated_determiner_rows_lower_with_their_stable_generated_owners() {
        let fixtures: &[(&str, &[&str])] = &[
            ("the creature", &["determiner_closed"]),
            ("target creature", &["determiner_target"]),
            (
                "up to two target creatures",
                &["determiner_quantified_target"],
            ),
            ("two creatures", &["determiner_quantity"]),
            ("your creature", &["determiner_closed"]),
            (
                "creature's power",
                &["possessive_noun_base", "determiner_possessive_noun"],
            ),
            (
                "the creature's power",
                &[
                    "possessive_noun_base",
                    "possessive_noun_determined",
                    "determiner_possessive_noun",
                ],
            ),
            (
                "exiled creature's power",
                &[
                    "possessive_noun_base",
                    "possessive_noun_adjective",
                    "determiner_possessive_noun",
                ],
            ),
        ];
        for (source, expected) in fixtures {
            let parsed = parse_nonterminal(source, &fixture_catalogs(), Nonterminal::NounPhrase)
                .unwrap_or_else(|error| panic!("failed to parse {source:?}: {error:?}"));
            for id in *expected {
                let decision = parsed
                    .construction_decisions()
                    .iter()
                    .find(|decision| decision.selected().as_str() == *id)
                    .unwrap_or_else(|| {
                        panic!("missing determiner selection {id} for {source:?}: {parsed:#?}")
                    });
                assert_eq!(decision.backend(), crate::ConstructionBackend::Chart);
                assert_eq!(decision.selected_production_ordinal(), 0, "{source:?}/{id}");
            }
        }

        let self_reference = SelfReference::new("Nissa Revane", true);
        for source in ["Nissa Revane's power", "Nissa's power"] {
            let parsed = parse_nonterminal_with_self_reference(
                source,
                &fixture_catalogs(),
                Nonterminal::NounPhrase,
                &self_reference,
            )
            .unwrap_or_else(|error| panic!("failed to parse {source:?}: {error:?}"));
            let decision = parsed
                .construction_decisions()
                .iter()
                .find(|decision| decision.selected().as_str() == "determiner_possessive_this_card")
                .unwrap_or_else(|| panic!("missing self-reference determiner row: {parsed:#?}"));
            assert_eq!(decision.backend(), crate::ConstructionBackend::Chart);
            assert_eq!(decision.selected_production_ordinal(), 0, "{source:?}");
        }
    }

    #[test]
    fn generated_quantity_family_parses_every_shape_notation_and_comparative() {
        let number = |value, numeral| super::super::NumberLiteral { value, numeral };
        let literal = |number| QuantityValue::Literal(number);
        let fixtures = [
            (
                "three",
                Quantity::unchecked_exact(number(3, Numeral::Cardinal)),
                "quantity_exact",
                0,
            ),
            (
                "third",
                Quantity::unchecked_exact(number(3, Numeral::Ordinal)),
                "quantity_exact",
                0,
            ),
            (
                "3",
                Quantity::unchecked_exact(number(3, Numeral::Arabic(false))),
                "quantity_exact",
                0,
            ),
            (
                "1,000",
                Quantity::unchecked_exact(number(1_000, Numeral::Arabic(true))),
                "quantity_exact",
                0,
            ),
            (
                "III",
                Quantity::unchecked_exact(number(3, Numeral::Roman)),
                "quantity_exact",
                0,
            ),
            (
                "at least two",
                Quantity::unchecked_at_least(literal(number(2, Numeral::Cardinal))),
                "quantity_at_least",
                0,
            ),
            (
                "two or fewer",
                Quantity::unchecked_or_comparison(
                    literal(number(2, Numeral::Cardinal)),
                    ComparativeWord::Fewer,
                ),
                "quantity_at_least",
                1,
            ),
            (
                "2 or greater",
                Quantity::unchecked_or_comparison(
                    literal(number(2, Numeral::Arabic(false))),
                    ComparativeWord::Greater,
                ),
                "quantity_at_least",
                1,
            ),
            (
                "two or less",
                Quantity::unchecked_or_comparison(
                    literal(number(2, Numeral::Cardinal)),
                    ComparativeWord::Less,
                ),
                "quantity_at_least",
                1,
            ),
            (
                "2 or more",
                Quantity::unchecked_or_comparison(
                    literal(number(2, Numeral::Arabic(false))),
                    ComparativeWord::More,
                ),
                "quantity_at_least",
                1,
            ),
            (
                "one or 2",
                Quantity::unchecked_or(
                    number(1, Numeral::Cardinal),
                    number(2, Numeral::Arabic(false)),
                ),
                "quantity_or",
                0,
            ),
            ("X", Quantity::unchecked_x(), "quantity_x", 0),
            ("both", Quantity::unchecked_both(), "quantity_both", 0),
            (
                "up to X",
                Quantity::unchecked_up_to(QuantityValue::Variable),
                "quantity_up_to",
                0,
            ),
            (
                "that many",
                Quantity::unchecked_that_many(),
                "quantity_that_many",
                0,
            ),
            (
                "that much",
                Quantity::unchecked_that_much(),
                "quantity_that_much",
                0,
            ),
            (
                "more than X",
                Quantity::unchecked_more_than(QuantityValue::Variable),
                "quantity_more_than",
                0,
            ),
            (
                "fewer than two",
                Quantity::unchecked_fewer_than(literal(number(2, Numeral::Cardinal))),
                "quantity_fewer_than",
                0,
            ),
        ];
        for (source, expected, id, ordinal) in fixtures {
            let parsed = parse_nonterminal(source, &fixture_catalogs(), Nonterminal::Quantity)
                .unwrap_or_else(|error| panic!("failed generated quantity {source:?}: {error:?}"));
            assert_eq!(parsed.quantity(), Some(&expected), "{source:?}");
            let decision = parsed
                .construction_decisions()
                .iter()
                .find(|decision| decision.selected().as_str() == id)
                .unwrap_or_else(|| panic!("missing {id} decision for {source:?}: {parsed:#?}"));
            assert_eq!(decision.backend(), crate::ConstructionBackend::Chart);
            assert_eq!(
                decision.selected_production_ordinal(),
                ordinal,
                "{source:?}"
            );
        }
    }

    #[test]
    fn generated_quantity_features_preserve_cardinality_and_agreement() {
        let fixtures = [
            (
                "one",
                NounCardinality::SingularOrMass,
                Number::Singular,
                true,
            ),
            (
                "up to one",
                NounCardinality::SingularOrMass,
                Number::Singular,
                false,
            ),
            (
                "more than one",
                NounCardinality::SingularOrMass,
                Number::Singular,
                false,
            ),
            (
                "fewer than one",
                NounCardinality::SingularOrMass,
                Number::Singular,
                false,
            ),
            (
                "one or one",
                NounCardinality::SingularOrMass,
                Number::Singular,
                false,
            ),
            ("X", NounCardinality::PluralOrMass, Number::Singular, false),
            (
                "at least one",
                NounCardinality::SingularOrMass,
                Number::Plural,
                false,
            ),
            (
                "at least two",
                NounCardinality::PluralOrMass,
                Number::Plural,
                false,
            ),
            (
                "at least X",
                NounCardinality::PluralOrMass,
                Number::Plural,
                false,
            ),
            (
                "one or more",
                NounCardinality::PluralOrMass,
                Number::Plural,
                false,
            ),
            ("both", NounCardinality::PluralOrMass, Number::Plural, false),
            (
                "that many",
                NounCardinality::PluralCount,
                Number::Plural,
                false,
            ),
            ("that much", NounCardinality::Mass, Number::Singular, false),
        ];
        for (source, cardinality, standalone_number, is_one) in fixtures {
            let parsed = parse_nonterminal(source, &fixture_catalogs(), Nonterminal::Quantity)
                .unwrap_or_else(|error| {
                    panic!("failed quantity feature fixture {source:?}: {error:?}")
                });
            assert_eq!(
                parsed.quantity().map(|value| value.noun_cardinality()),
                Some(cardinality)
            );
            assert!(
                matches!(
                    parsed.chart.forest.node(parsed.root).key.constituent_features(),
                    Some(Features::Quantity(super::super::QuantityFeatures {
                        cardinality: actual_cardinality,
                        standalone_number: actual_number,
                        is_one: actual_one,
                    })) if *actual_cardinality == cardinality
                        && *actual_number == standalone_number
                        && *actual_one == is_one
                ),
                "wrong generated features for {source:?}: {parsed:#?}"
            );
        }

        // Keep direct producer assertions alongside the whole-NP consumer
        // coverage below so a failure identifies which side of the quantity
        // cardinality contract regressed.
        for source in ["two", "at least two", "one or more", "both", "that many"] {
            let parsed = parse_nonterminal(source, &fixture_catalogs(), Nonterminal::Quantity)
                .unwrap_or_else(|error| {
                    panic!("failed negative feature fixture {source:?}: {error:?}")
                });
            assert_ne!(
                parsed.quantity().map(|value| value.noun_cardinality()),
                Some(NounCardinality::SingularOrMass),
                "generated quantity exposed singular agreement for {source:?}"
            );
        }
        let mass = parse_nonterminal("that much", &fixture_catalogs(), Nonterminal::Quantity)
            .expect("the generated mass quantity parses");
        assert_ne!(
            mass.quantity().map(|value| value.noun_cardinality()),
            Some(NounCardinality::PluralCount),
            "the generated mass quantity exposed plural-count agreement"
        );
    }

    #[test]
    fn generated_quantity_cardinality_gates_nominal_consumers() {
        let catalogs = fixture_catalogs();
        let rejected_supported = [
            "one creature",
            "two creatures",
            "at least one creature",
            "at least two creatures",
            "at least two damage",
            "that many creatures",
            "that much damage",
        ]
        .into_iter()
        .filter(|source| parse_nonterminal(source, &catalogs, Nonterminal::NounPhrase).is_err())
        .collect::<Vec<_>>();
        assert!(
            rejected_supported.is_empty(),
            "supported quantity/noun agreement failed: {rejected_supported:?}"
        );
        let admitted_mismatches = ["two creature", "that much creatures"]
            .into_iter()
            .filter(|source| parse_nonterminal(source, &catalogs, Nonterminal::NounPhrase).is_ok())
            .collect::<Vec<_>>();
        assert!(
            admitted_mismatches.is_empty(),
            "quantity/noun cardinality mismatches parsed: {admitted_mismatches:?}"
        );

        // `that many damage` is not a quantity-family negative: after rejecting
        // the quantity reading, it still has the independent ordinary-noun
        // reading `that [many] damage`. Unknown-word opacity must likewise
        // remain available while a recognized numeral is kept structural.
        let opaque = parse_nonterminal("blorple creature", &catalogs, Nonterminal::NounPhrase)
            .expect("a genuinely unknown noun modifier remains opaque");
        assert_eq!(opaque.opacity_mode(), OpacityMode::OpaqueNouns);
    }

    #[test]
    fn generated_quantity_rejects_incomplete_and_literal_x_combinations() {
        for source in [
            "at least", "one or", "X or two", "up to", "1,00", "1, 000", "1,,000",
        ] {
            assert!(
                parse_nonterminal(source, &fixture_catalogs(), Nonterminal::Quantity).is_err(),
                "invalid quantity parsed: {source:?}"
            );
        }
    }

    #[test]
    fn lowering_decline_falls_through_to_the_next_ranked_root() {
        let catalogs = fixture_catalogs();
        assert!(matches!(
            parse_single_best_root("copy that spell", &catalogs, Nonterminal::Clause),
            Err(ParseNonterminalError::Lowering)
        ));

        let parsed = parse_nonterminal("copy that spell", &catalogs, Nonterminal::Clause)
            .expect("a lowerable imperative root follows the rejected nominal reading");
        assert!(matches!(
            parsed.clause(),
            Some(Clause::Independent(IndependentClause::Finite(finite)))
                if finite.subject().is_none()
        ));
    }

    #[test]
    fn successful_single_root_selections_are_unchanged() {
        let catalogs = fixture_catalogs();
        for source in [
            "you draw a card",
            "target creature can't block this turn",
            "it is your turn",
            "there are no creatures on the battlefield",
            "damage can't be prevented",
        ] {
            let before = parse_single_best_root(source, &catalogs, Nonterminal::Clause)
                .unwrap_or_else(|error| {
                    panic!("control failed before retry for {source:?}: {error:?}")
                });
            let after =
                parse_nonterminal(source, &catalogs, Nonterminal::Clause).unwrap_or_else(|error| {
                    panic!("control failed after retry for {source:?}: {error:?}")
                });
            assert_eq!(after.root, before.root, "root changed for {source:?}");
            assert_eq!(
                after.root_rule(),
                before.root_rule(),
                "rule changed for {source:?}"
            );
            assert_eq!(after.cost(), before.cost(), "cost changed for {source:?}");
            assert_eq!(
                after.root_tied_alternatives(),
                before.root_tied_alternatives(),
                "root tie changed for {source:?}",
            );
            assert_eq!(
                after.chart_stats(),
                before.chart_stats(),
                "chart changed for {source:?}"
            );
            assert_eq!(
                after.forest_stats(),
                before.forest_stats(),
                "forest changed for {source:?}"
            );
            assert_eq!(
                after.clause(),
                before.clause(),
                "syntax changed for {source:?}"
            );
        }
    }
}

#[cfg(test)]
mod registration_order_tests {
    use super::super::generated::GeneratedActivation;
    use super::*;
    use crate::construction::ConstructionEvidenceKind;
    use crate::construction::ConstructionId;
    use crate::constructions::probe;
    use crate::forest::SelectionReason;
    use crate::grammar::rules::RegistrationOrder;

    #[test]
    fn comparison_standard_ambiguity_is_registration_order_neutral() {
        for (source, selected_form) in [("than target", 1), ("than target player", 0)] {
            let activation = if selected_form == 0 {
                GeneratedActivation::Production
            } else {
                GeneratedActivation::Groups(crate::constructions::adjective::GROUPS)
            };
            let parses = crate::grammar::parse_nonterminal_with_activation_in_both_orders(
                source,
                &Catalogs::default(),
                Nonterminal::ComparisonComplement,
                &crate::identity::SelfReference::default(),
                activation,
            );
            assert_eq!(
                format!("{:#?}", parses[0].syntax),
                format!("{:#?}", parses[1].syntax),
                "the selected typed standard must not depend on registration order: {source:?}",
            );
            assert_eq!(
                parses[0].root_tied_alternatives(),
                parses[1].root_tied_alternatives(),
                "root ties changed with registration order: {source:?}",
            );
            let Lowered::ComparisonComplement(selected) = &parses[0].syntax else {
                panic!(
                    "the generated marker did not lower: {:#?}",
                    parses[0].syntax
                )
            };
            assert_eq!(selected.marker(), crate::syntax::ComparisonMarker::Than);
            assert!(
                matches!(
                    (selected_form, selected.standard()),
                    (0, crate::syntax::Phrase::NounPhrase(_))
                        | (1, crate::syntax::Phrase::AdjectivePhrase(_))
                ),
                "unexpected selected standard for {source:?}: {selected:#?}",
            );

            let mut signatures = Vec::new();
            for parsed in &parses {
                let decision = parsed
                    .construction_decisions()
                    .iter()
                    .find(|decision| decision.selected().as_str() == "comparison_standard")
                    .unwrap_or_else(|| {
                        panic!(
                            "the typed standard ambiguity was not packed for {source:?}: {:#?}",
                            parsed.construction_decisions(),
                        )
                    });
                assert_eq!(
                    decision.reason(),
                    if decision.alternatives().len() > 1 {
                        SelectionReason::StableIdentity
                    } else {
                        SelectionReason::Unique
                    }
                );
                assert_eq!(decision.selected_production_ordinal(), selected_form);
                let candidates = decision
                    .alternatives()
                    .iter()
                    .map(|alternative| {
                        (
                            alternative.id().as_str(),
                            alternative.production_ordinal(),
                            alternative.is_dominated(),
                        )
                    })
                    .collect::<Vec<_>>();
                assert!(
                    candidates
                        .iter()
                        .all(|(id, _, dominated)| *id == "comparison_standard" && !dominated),
                    "every packed standard must remain undominated for {source:?}: {candidates:?}",
                );
                signatures.push((decision.selected_production_ordinal(), candidates));
            }
            assert_eq!(signatures[0], signatures[1]);
        }
    }

    const ORDER_INVARIANT_FIXTURES: &[(Nonterminal, &str)] = &[
        (
            Nonterminal::Sentence,
            "This creature has protection from artifacts.",
        ),
        (Nonterminal::Clause, "copy that spell"),
        (
            Nonterminal::Sentence,
            "Any number of target creatures gets +2/+2 until end of turn.",
        ),
        (
            Nonterminal::Sentence,
            "Activate only as a sorcery and only once each turn.",
        ),
        (Nonterminal::NounPhrase, "the declare attackers step"),
        (
            Nonterminal::Sentence,
            "Enchanted creature has protection from black and from red.",
        ),
        (
            Nonterminal::Sentence,
            "Exile target artifact, creature, or planeswalker and target land or battle.",
        ),
    ];

    const NOMINAL_DOMINANCE_FIXTURES: &[(Nonterminal, &str, &str, &str)] = &[
        (
            Nonterminal::NounPhrase,
            "the top two cards of your library",
            "nominal_quantity_modifier",
            "nominal_prepositional",
        ),
        (
            Nonterminal::NounPhrase,
            "the top two creatures attacking",
            "nominal_quantity_modifier",
            "nominal_postpositive_adjective",
        ),
        (
            Nonterminal::NounPhrase,
            "the top two greater creatures than a card",
            "nominal_quantity_modifier",
            "nominal_comparison",
        ),
        (
            Nonterminal::NounPhrase,
            "at least four more creatures than you",
            "nominal_determiner",
            "nominal_comparison",
        ),
        (
            Nonterminal::NounPhrase,
            "mana of any color to cast that spell",
            "nominal_prepositional",
            "nominal_infinitive",
        ),
        (
            Nonterminal::NounPhrase,
            "target artifact or land card in your graveyard",
            "nominal_prepositional",
            "nominal_coordinated_modifier",
        ),
        (
            Nonterminal::NounPhrase,
            "protection from each of your opponents",
            "nominal_prepositional",
            "nominal_keyword_predicated_argument",
        ),
        (
            Nonterminal::NounPhrase,
            "card from your graveyard to your hand",
            "nominal_prepositional",
            "nominal_noun",
        ),
        (
            Nonterminal::Sentence,
            "If a creature dealt damage this way would die this turn, exile it instead.",
            "nominal_reduced_recipient_passive",
            "nominal_noun",
        ),
        (
            Nonterminal::NounPhrase,
            "a 1/1 white Ally creature token for each experience counter you have",
            "nominal_relative",
            "nominal_prepositional",
        ),
    ];

    #[derive(Debug, PartialEq, Eq)]
    struct NormalizedParse {
        syntax: String,
        cost: ParseCost,
        decisions: Vec<NormalizedDecision>,
    }

    #[derive(Debug, PartialEq, Eq)]
    struct NormalizedDecision {
        span: Span,
        selected: ConstructionId,
        reason: SelectionReason,
        candidates: Vec<(ConstructionId, u16, bool)>,
    }

    #[derive(Debug, PartialEq, Eq)]
    struct NormalizedLensParse {
        value: probe::ProbeLensRecord,
        rendered: String,
        decisions: Vec<NormalizedDecision>,
    }

    fn fixture_catalogs() -> Catalogs {
        Catalogs::default()
            .with_catalog(CatalogKind::KeywordAbility, ["Protection"])
            .with_catalog(CatalogKind::CreatureType, ["Ally"])
            .with_catalog(
                CatalogKind::CardType,
                [
                    "Artifact",
                    "Battle",
                    "Creature",
                    "Land",
                    "Planeswalker",
                    "Sorcery",
                ],
            )
    }

    fn normalized_parse(
        source: &str,
        nonterminal: Nonterminal,
        order: RegistrationOrder,
        activation: GeneratedActivation,
    ) -> NormalizedParse {
        let parsed = parse_nonterminal_with_registration_order(
            source,
            &fixture_catalogs(),
            nonterminal,
            order,
            activation,
        )
        .unwrap_or_else(|error| panic!("failed to parse {source:?}: {error:?}"));
        NormalizedParse {
            syntax: format!("{:?}", parsed.syntax),
            cost: parsed.cost(),
            decisions: parsed
                .construction_decisions()
                .iter()
                .map(|decision| NormalizedDecision {
                    span: decision.span(),
                    selected: decision.selected(),
                    reason: decision.reason(),
                    candidates: decision
                        .alternatives()
                        .iter()
                        .map(|candidate| {
                            (
                                candidate.id(),
                                candidate.production_ordinal(),
                                candidate.is_dominated(),
                            )
                        })
                        .collect(),
                })
                .collect(),
        }
    }

    #[test]
    fn semantic_selections_survive_family_registration_permutations() {
        for &(nonterminal, source) in ORDER_INVARIANT_FIXTURES {
            let normal = normalized_parse(
                source,
                nonterminal,
                RegistrationOrder::Normal,
                GeneratedActivation::Production,
            );
            assert_eq!(
                normalized_parse(
                    source,
                    nonterminal,
                    RegistrationOrder::Reversed,
                    GeneratedActivation::Production,
                ),
                normal,
                "reversed registration changed {source:?}",
            );
            assert_eq!(
                normalized_parse(
                    source,
                    nonterminal,
                    RegistrationOrder::FixedShuffle,
                    GeneratedActivation::Production,
                ),
                normal,
                "shuffled registration changed {source:?}",
            );
        }
    }

    #[test]
    fn every_nominal_dominance_edge_survives_family_registration_permutations() {
        for &(nonterminal, source, winner, loser) in NOMINAL_DOMINANCE_FIXTURES {
            let normal = normalized_parse(
                source,
                nonterminal,
                RegistrationOrder::Normal,
                GeneratedActivation::Production,
            );
            let runtime_dominance = normal.decisions.iter().any(|decision| {
                decision.selected.as_str() == winner
                    && decision.reason == SelectionReason::Dominance
                    && decision
                        .candidates
                        .iter()
                        .any(|(id, _, dominated)| id.as_str() == loser && *dominated)
            });
            if winner == "nominal_reduced_recipient_passive" {
                // Its generated predicate child now declares an attachment
                // cost, so the opaque-noun candidate is pruned before this
                // edge needs to decide the packed set. Keep both obligations:
                // the edge is still registered and the semantic owner wins
                // under every registration permutation.
                assert!(
                    super::super::construction::registry()
                        .dominates(ConstructionId::new(winner), ConstructionId::new(loser),)
                );
                assert!(
                    normal
                        .decisions
                        .iter()
                        .any(|decision| { decision.selected.as_str() == winner }),
                    "fixture {source:?} did not select {winner}: {normal:#?}"
                );
            } else {
                assert!(
                    runtime_dominance,
                    "fixture {source:?} did not exercise declared edge {winner}>{loser}: {normal:#?}",
                );
            }
            for order in [RegistrationOrder::Reversed, RegistrationOrder::FixedShuffle] {
                assert_eq!(
                    normalized_parse(source, nonterminal, order, GeneratedActivation::Production,),
                    normal,
                    "{winner}>{loser} changed under {order:?} registration for {source:?}",
                );
            }
        }
    }

    #[test]
    fn declaration_evidence_values_follow_the_selected_semantic_edges() {
        let positives = [
            (
                "This creature has protection from red and from blue.",
                "predicated_argument_from_extend",
                ConstructionEvidenceKind::Guard,
                "keyword-grant conjunction gate",
                "conjunction=And;allowed=[And];matched=true",
            ),
            (
                "This card deals damage to you and creatures you control that are tapped.",
                "rules_object_noun_phrase",
                ConstructionEvidenceKind::Role,
                "rules-object attachment role",
                "category=RulesObjectNounPhrase",
            ),
            (
                "If a creature dealt damage this way would die this turn, exile it instead.",
                "nominal_reduced_recipient_passive",
                ConstructionEvidenceKind::Guard,
                "reduced-recipient-passive frame",
                "predicate.frame={recipient_passive:true,direct_object:true,indirect_object:false}",
            ),
            (
                "This creature has protection from artifacts.",
                "nominal_prepositional",
                ConstructionEvidenceKind::Feature,
                "nominal attachment phase",
                "attachment=prepositional(nearer_relative_host=false)",
            ),
        ];
        for (source, id, kind, label, value) in positives {
            let parsed = parse_nonterminal(source, &fixture_catalogs(), Nonterminal::Sentence)
                .unwrap_or_else(|error| panic!("positive evidence fixture {source:?}: {error:?}"));
            let decision = parsed
                .construction_decisions()
                .iter()
                .find(|decision| decision.selected().as_str() == id)
                .unwrap_or_else(|| panic!("missing {id} decision for {source:?}: {parsed:#?}"));
            assert_eq!(decision.evidence().kind(), kind, "{source:?}");
            assert_eq!(decision.evidence().label(), label, "{source:?}");
            assert_eq!(decision.evidence_value(), Some(value), "{source:?}");
        }

        for (source, excluded) in [
            (
                "This creature has protection from red.",
                "predicated_argument_from_extend",
            ),
            (
                "This card deals damage to a creature.",
                "rules_object_noun_phrase",
            ),
            (
                "If a creature attacks this turn, exile it instead.",
                "nominal_reduced_recipient_passive",
            ),
            ("A card is red.", "nominal_prepositional"),
        ] {
            let parsed = parse_nonterminal(source, &fixture_catalogs(), Nonterminal::Sentence)
                .unwrap_or_else(|error| panic!("negative evidence fixture {source:?}: {error:?}"));
            assert!(
                parsed.sentence().is_some(),
                "{source:?} did not lower as a sentence"
            );
            assert!(
                parsed
                    .construction_decisions()
                    .iter()
                    .any(|decision| decision.selected().as_str() == "sentence"),
                "{source:?} lacks its expected neighboring sentence decision: {parsed:#?}",
            );
            assert!(
                parsed
                    .construction_decisions()
                    .iter()
                    .all(|decision| decision.selected().as_str() != excluded),
                "negative fixture unexpectedly selected {excluded}: {parsed:#?}",
            );
        }
    }

    fn normalized_parse_probe(
        order: RegistrationOrder,
        groups: &'static [&'static deckmaste_construction_compiler::runtime::GroupData],
    ) -> NormalizedParse {
        let cats = super::super::generated::internal_categories(groups);
        normalized_parse(
            "and, or",
            Nonterminal::Generated(cats["ProbePairRoot"]),
            order,
            GeneratedActivation::Groups(groups),
        )
    }

    fn normalized_lens_parse(
        order: RegistrationOrder,
        groups: &'static [&'static deckmaste_construction_compiler::runtime::GroupData],
    ) -> NormalizedLensParse {
        let cats = super::super::generated::internal_categories(groups);
        let parsed = parse_nonterminal_with_registration_order(
            "and or then",
            &Catalogs::default(),
            Nonterminal::Generated(cats["ProbeLensRecord"]),
            order,
            GeneratedActivation::Groups(groups),
        )
        .expect("the recursive lens family parses under every registration permutation");
        let Lowered::Generated(value) = &parsed.syntax else {
            panic!("recursive lens parsing did not use generated lowering");
        };
        let value = value
            .downcast_ref::<probe::ProbeLensRecord>()
            .expect("the generated lens builder returns its declared owner")
            .clone();
        let rendered = probe::linearize_lens_record(&value)
            .expect("the generated lens family dispatcher selects an exact inverse");
        let decisions = parsed
            .construction_decisions()
            .iter()
            .map(|decision| NormalizedDecision {
                span: decision.span(),
                selected: decision.selected(),
                reason: decision.reason(),
                candidates: decision
                    .alternatives()
                    .iter()
                    .map(|candidate| {
                        (
                            candidate.id(),
                            candidate.production_ordinal(),
                            candidate.is_dominated(),
                        )
                    })
                    .collect(),
            })
            .collect();
        NormalizedLensParse {
            value,
            rendered,
            decisions,
        }
    }

    #[test]
    fn generated_selections_survive_family_registration_permutations() {
        let normal = normalized_parse_probe(RegistrationOrder::Normal, probe::GROUPS);
        assert_eq!(
            normal,
            normalized_parse_probe(RegistrationOrder::Reversed, probe::GROUPS)
        );
        assert_eq!(
            normal,
            normalized_parse_probe(RegistrationOrder::FixedShuffle, probe::GROUPS)
        );
    }

    #[test]
    fn generated_selections_survive_group_list_permutations() {
        assert_eq!(
            normalized_parse_probe(RegistrationOrder::Normal, probe::GROUPS),
            normalized_parse_probe(RegistrationOrder::Normal, probe::GROUPS_REVERSED),
        );
    }

    #[test]
    fn recursive_lens_family_survives_registration_and_group_order_permutations() {
        // Mutations guarded: retain a registration-local rule index during
        // lens lowering, or resolve the owner category from group slice
        // position. Either makes at least one matrix entry differ even though
        // the declaration and input bytes are unchanged.
        let normal = normalized_lens_parse(RegistrationOrder::Normal, probe::GROUPS);
        assert_eq!(
            normal
                .value
                .prefix
                .iter()
                .map(|token| token.word)
                .collect::<Vec<_>>(),
            [
                crate::features::Conjunction::And,
                crate::features::Conjunction::Or,
            ],
        );
        assert_eq!(normal.value.head.word, crate::features::Conjunction::Then);
        assert!(normal.value.suffix.is_empty());
        assert_eq!(normal.rendered, "and or then");

        for (group_order, groups) in [
            ("declared", probe::GROUPS),
            ("reversed", probe::GROUPS_REVERSED),
        ] {
            for registration_order in [
                RegistrationOrder::Normal,
                RegistrationOrder::Reversed,
                RegistrationOrder::FixedShuffle,
            ] {
                assert_eq!(
                    normalized_lens_parse(registration_order, groups),
                    normal,
                    "recursive lens family changed under {group_order} groups and {registration_order:?} registration",
                );
            }
        }
    }

    #[test]
    fn internal_category_ids_ignore_group_order() {
        assert_eq!(
            super::super::generated::internal_categories(probe::GROUPS),
            super::super::generated::internal_categories(probe::GROUPS_REVERSED),
        );
    }
}

#[cfg(test)]
mod generated_adapter_tests {
    use super::super::generated::GeneratedActivation;
    use super::*;
    use crate::catalog::Catalogs;
    use crate::constructions::coordination;
    use crate::constructions::noun;
    use crate::constructions::probe;
    use crate::word::BareNominalAdjunct;
    use crate::word::Noun;
    use crate::word::Vocab;

    static COORDINATION_GROUPS: &[&deckmaste_construction_compiler::runtime::GroupData] = &[
        noun::GROUPS[0],
        crate::constructions::determiner::GROUPS[0],
        crate::constructions::nominal::GROUPS[0],
        crate::constructions::noun_phrase::GROUPS[0],
        crate::constructions::prepositional::GROUPS[0],
        coordination::GROUPS[0],
    ];

    fn probe_category(name: &str) -> Nonterminal {
        let cats = super::super::generated::internal_categories(probe::GROUPS);
        Nonterminal::Generated(cats[name])
    }

    fn coordination_catalogs() -> Catalogs {
        Catalogs::default().with_catalog(CatalogKind::CardType, ["Artifact", "Creature", "Land"])
    }

    fn generated_coordination(source: &str) -> ParsedNonterminal {
        let parsed = parse_nonterminal_with_activation(
            source,
            &coordination_catalogs(),
            Nonterminal::NounPhrase,
            GeneratedActivation::Groups(COORDINATION_GROUPS),
        )
        .unwrap_or_else(|error| {
            panic!("generated coordination did not parse {source:?}: {error:?}")
        });
        assert!(
            parsed.construction_decisions().iter().any(|decision| {
                matches!(
                    decision.selected().as_str(),
                    "noun_phrase_coordination" | "shared_determiner_nominal"
                )
            }),
            "no generated coordination owner recorded for {source:?}: {:#?}",
            parsed.construction_decisions(),
        );
        parsed
    }

    #[test]
    fn real_generated_binary_coordination_parses() {
        let parsed = generated_coordination("an artifact or a creature");
        let Some(noun_phrase) = parsed.noun_phrase() else {
            panic!(
                "binary coordination lowered to the wrong syntax: {:?}",
                parsed.syntax
            );
        };
        let crate::syntax::NounPhraseKind::Coordinated(coordination) = noun_phrase.kind() else {
            panic!(
                "binary coordination lowered to the wrong syntax: {:?}",
                parsed.syntax
            );
        };
        assert_eq!(coordination.rest().len(), 1);
        assert_eq!(
            coordination.rest()[0].conjunction,
            Some(crate::features::Conjunction::Or)
        );
    }

    #[test]
    fn real_generated_oxford_coordination_parses() {
        let parsed = generated_coordination("an artifact, a creature, and a land");
        let Some(noun_phrase) = parsed.noun_phrase() else {
            panic!(
                "Oxford coordination lowered to the wrong syntax: {:?}",
                parsed.syntax
            );
        };
        let crate::syntax::NounPhraseKind::Coordinated(coordination) = noun_phrase.kind() else {
            panic!(
                "Oxford coordination lowered to the wrong syntax: {:?}",
                parsed.syntax
            );
        };
        assert_eq!(coordination.rest().len(), 2);
        assert_eq!(coordination.rest()[0].conjunction, None);
        assert_eq!(
            coordination.rest()[1].conjunction,
            Some(crate::features::Conjunction::And)
        );
    }

    #[test]
    fn real_generated_nested_coordination_parses() {
        let parsed = generated_coordination("an artifact and a creature or a land");
        let Some(noun_phrase) = parsed.noun_phrase() else {
            panic!(
                "nested coordination lowered to the wrong syntax: {:?}",
                parsed.syntax
            );
        };
        let crate::syntax::NounPhraseKind::Coordinated(coordination) = noun_phrase.kind() else {
            panic!(
                "nested coordination lowered to the wrong syntax: {:?}",
                parsed.syntax
            );
        };
        assert!(
            matches!(
                coordination.first().kind(),
                crate::syntax::NounPhraseKind::Coordinated(_)
            ) || coordination.rest().iter().any(|member| matches!(
                member.phrase.kind(),
                crate::syntax::NounPhraseKind::Coordinated(_)
            )),
            "one binary group must be nested inside the other: {coordination:#?}",
        );
    }

    #[test]
    fn real_generated_shared_determiner_coordination_parses() {
        let parsed = generated_coordination("target artifact or creature");
        let Some(noun_phrase) = parsed.noun_phrase() else {
            panic!(
                "shared-determiner coordination lowered to the wrong syntax: {:?}\n{:#?}",
                parsed.syntax,
                parsed.construction_decisions(),
            );
        };
        let crate::syntax::NounPhraseKind::CoordinatedNominal(coordination) = noun_phrase.kind()
        else {
            panic!("shared-determiner coordination lowered to the wrong syntax");
        };
        assert_eq!(coordination.determiner(), &crate::determiner::target(None));
        assert_eq!(coordination.rest().len(), 1);
    }

    #[test]
    fn generated_noun_reduction_and_lowering_preserve_every_inherent_feature() {
        // Mutations caught: replace identity-feature propagation in
        // generated_construction_features with Features::None, drop one noun
        // feature there, or lower a value other than the recognized identity.
        let catalogs = Catalogs::default().with_catalog(CatalogKind::CreatureType, ["Elf"]);
        for source in ["Elf", "turn", "damage"] {
            let parsed =
                parse_nonterminal(source, &catalogs, Nonterminal::Noun).unwrap_or_else(|error| {
                    panic!("generated noun did not parse {source:?}: {error:?}")
                });
            assert_eq!(parsed.opacity_mode, OpacityMode::Exact, "{source}");
            assert!(
                parsed
                    .construction_decisions
                    .iter()
                    .any(|decision| { decision.selected().as_str() == "noun" })
            );
            let features = parsed
                .chart
                .forest
                .node(parsed.root)
                .key
                .constituent_features()
                .expect("selected generated root carries constituent features");

            match (source, &parsed.syntax, features) {
                (
                    "Elf",
                    Lowered::Noun(lowered),
                    Features::Noun {
                        identity: Some(feature_identity),
                        coordination_domain: Some(super::super::CoordinationDomain::Entity),
                        form: super::super::NounForm::Singular,
                        initial_sound: crate::word::InitialSound::Vowel,
                        adjunct: None,
                        opaque: false,
                        recipient_passive_theme: false,
                    },
                ) if matches!(
                    lowered.kind(),
                    crate::word::NounInstanceKind::Singular(Noun::Catalog(lowered))
                        if lowered == feature_identity
                ) => {}
                (
                    "turn",
                    Lowered::Noun(lowered),
                    Features::Noun {
                        identity: None,
                        coordination_domain: None,
                        form: super::super::NounForm::Singular,
                        initial_sound: crate::word::InitialSound::Consonant,
                        adjunct: Some(BareNominalAdjunct::Temporal),
                        opaque: false,
                        recipient_passive_theme: false,
                    },
                ) if matches!(
                    lowered.kind(),
                    crate::word::NounInstanceKind::Singular(Noun::Word(Vocab::Turn))
                ) => {}
                (
                    "damage",
                    Lowered::Noun(lowered),
                    Features::Noun {
                        identity: None,
                        coordination_domain: Some(super::super::CoordinationDomain::Damage),
                        form: super::super::NounForm::Mass,
                        initial_sound: crate::word::InitialSound::Consonant,
                        adjunct: None,
                        opaque: false,
                        recipient_passive_theme: true,
                    },
                ) if matches!(
                    lowered.kind(),
                    crate::word::NounInstanceKind::Mass(Noun::Word(Vocab::Damage))
                ) => {}
                other => panic!("generated noun lost inherent state: {other:#?}"),
            }
        }
    }

    #[test]
    fn generated_group_parses_and_records_generated_decisions() {
        let parsed = parse_nonterminal_with_activation(
            "and, or",
            &Catalogs::default(),
            probe_category("ProbePairRoot"),
            GeneratedActivation::Groups(probe::GROUPS),
        )
        .expect("the probe pair parses");
        let decisions = parsed.construction_decisions();
        let pair = decisions
            .iter()
            .find(|decision| decision.selected().as_str() == "probe_pair")
            .expect("probe_pair decision recorded");
        assert_eq!(pair.backend(), crate::ConstructionBackend::Chart);
        let pick = decisions
            .iter()
            .find(|decision| decision.selected().as_str() == "probe_pick")
            .expect("dominance selects probe_pick");
        assert!(
            pick.alternatives().iter().any(|alternative| {
                alternative.id().as_str() == "probe_pick_shadow" && alternative.is_dominated()
            }),
            "the shadow alternative is recorded and dominated: {decisions:?}"
        );
        assert_eq!(
            pick.alternatives()
                .iter()
                .map(ConstructionAlternative::production_ordinal)
                .max(),
            Some(0)
        );
        assert!(
            matches!(
                parsed.syntax,
                Lowered::Generated(super::super::lowering::GeneratedValue::Typed(_))
            ),
            "probe lowering reaches the emitted typed builder: {:?}",
            parsed.syntax
        );
        let Lowered::Generated(value) = &parsed.syntax else {
            unreachable!();
        };
        assert!(value.is::<crate::constructions::probe::ProbePairNode>());
    }

    #[test]
    fn generated_form_ordinal_is_the_declared_ordinal_not_a_counter() {
        // `probe_word`'s `padded` form declares ordinal 7 — non-zero and
        // non-consecutive with `only`'s ordinal 0, a value no incremental
        // per-construction counter could produce. This is the plan's
        // ordinal-provenance invariant (`rules.rs`, `generated.rs`'s
        // `register_generated`) made testable: replacing `ordinal:
        // form.ordinal` in `generated.rs` with an incremental counter must
        // make this assertion fail.
        let parsed = parse_nonterminal_with_activation(
            ", and",
            &Catalogs::default(),
            probe_category("ProbeItem"),
            GeneratedActivation::Groups(probe::GROUPS),
        )
        .expect("the padded probe_word form parses");
        let decisions = parsed.construction_decisions();
        let word = decisions
            .iter()
            .find(|decision| decision.selected().as_str() == "probe_word")
            .expect("probe_word decision recorded");
        assert_eq!(
            word.selected_production_ordinal(),
            7,
            "the padded form's declared ordinal must reach the decision"
        );
    }

    #[test]
    fn generated_sequence_lowers_through_element_and_sequence_builders() {
        for (source, comma) in [
            ("or and or", crate::features::Comma::Absent),
            ("or, and or", crate::features::Comma::Present),
        ] {
            let parsed = parse_nonterminal_with_activation(
                source,
                &Catalogs::default(),
                probe_category("ProbeSequenceRoot"),
                GeneratedActivation::Groups(probe::GROUPS),
            )
            .expect("the generated sequence probe parses and lowers");
            let Lowered::Generated(value) = &parsed.syntax else {
                panic!(
                    "sequence probe did not use generated lowering: {:?}",
                    parsed.syntax
                );
            };
            let value = value
                .downcast_ref::<crate::constructions::probe::ProbeSequenceNode>()
                .expect("the root builder returns its declared own-mode type");
            assert_eq!(value.rest()[0].comma, comma);
            assert_eq!(
                value.rest()[0].conjunction,
                Some(crate::features::Conjunction::And)
            );
        }
    }

    #[test]
    fn recursive_lens_round_trips_through_generated_reduction() {
        let parse = |source| {
            parse_nonterminal_with_activation(
                source,
                &Catalogs::default(),
                probe_category("ProbeLensRecord"),
                GeneratedActivation::Groups(probe::GROUPS),
            )
            .expect("the recursive lens probe parses and lowers")
        };

        let parsed = parse("and or then");
        let Lowered::Generated(value) = &parsed.syntax else {
            panic!(
                "lens probe did not use generated lowering: {:?}",
                parsed.syntax
            );
        };
        let value = value
            .downcast_ref::<crate::constructions::probe::ProbeLensRecord>()
            .expect("the emitted erased builder returns the flattened lens owner");
        assert_eq!(
            value
                .prefix
                .iter()
                .map(|token| token.word)
                .collect::<Vec<_>>(),
            [
                crate::features::Conjunction::And,
                crate::features::Conjunction::Or
            ],
        );
        assert_eq!(value.head.word, crate::features::Conjunction::Then);
        assert!(value.suffix.is_empty());

        let rendered = crate::constructions::probe::linearize_lens_record(value)
            .expect("the generated lens linearizer preserves the recursive surface order");
        assert_eq!(rendered, "and or then");
        let reparsed = parse(&rendered);
        let Lowered::Generated(reparsed) = &reparsed.syntax else {
            panic!("linearized lens surface did not lower through the generated adapter");
        };
        assert_eq!(
            reparsed.downcast_ref::<crate::constructions::probe::ProbeLensRecord>(),
            Some(value),
        );
    }

    #[test]
    fn generated_parse_fails_without_matching_input() {
        // Contrast case: the comma-bearing input parses under these same
        // active groups, so a failure below cannot be attributed to no
        // generated rules having been registered at all — only to the
        // missing comma.
        parse_nonterminal_with_activation(
            "and, or",
            &Catalogs::default(),
            probe_category("ProbePairRoot"),
            GeneratedActivation::Groups(probe::GROUPS),
        )
        .expect("the comma-bearing pair parses under the same active groups");
        let error = parse_nonterminal_with_activation(
            "and or",
            &Catalogs::default(),
            probe_category("ProbePairRoot"),
            GeneratedActivation::Groups(probe::GROUPS),
        );
        assert!(
            matches!(error, Err(ParseNonterminalError::NoCompleteParse(_))),
            "the pair form requires its comma: {error:?}"
        );
    }
}
