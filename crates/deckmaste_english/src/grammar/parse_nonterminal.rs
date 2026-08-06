#![allow(
    dead_code,
    reason = "diagnostic parse modes and errors are staged for later grammar milestones"
)]

use super::AdjectivePhrase;
use super::BestParse;
#[cfg(test)]
use super::CatalogKind;
use super::Catalogs;
use super::ChartResult;
use super::ChartStats;
use super::Clause;
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
    let surface = lex(source);
    let tokens = collapse_full_names(source, surface.tokens, self_reference.full_name());
    match parse_nonterminal_with_mode(
        source,
        catalogs,
        nonterminal,
        &tokens,
        OpacityMode::Exact,
        self_reference,
    ) {
        Ok(parsed) => Ok(parsed),
        Err(ParseNonterminalError::NoCompleteParse(_)) => parse_nonterminal_with_mode(
            source,
            catalogs,
            nonterminal,
            &tokens,
            OpacityMode::OpaqueNouns,
            self_reference,
        ),
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
    reason = "internal assembly-mode plumbing; the M3 activation parameter is the last of a threaded diagnostic-mode set, not independent knobs"
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

/// The registry a parse actually uses: the production `registry()` static
/// when no generated group is active, or a freshly merged registry when a
/// test activates one. A distinct type (rather than inlining the match at
/// the call site) so the selection is a named, independently testable step —
/// `#[cfg(test)]` code can call [`select_registry`] directly and observe
/// which arm fired, rather than only being able to observe its downstream
/// effect on a parse.
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
    match activation.groups() {
        None => SelectedRegistry::Static(super::construction::handwritten_registry()),
        Some(groups) => SelectedRegistry::Owned(
            super::construction::merged_registry(groups)
                .expect("active generated groups must merge"),
        ),
    }
}

#[cfg(test)]
fn parse_nonterminal_with_registration_order(
    source: &str,
    catalogs: &Catalogs,
    nonterminal: Nonterminal,
    registration_order: RegistrationOrder,
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
        decisions.push(ConstructionDecision::new(
            span,
            production,
            family,
            cost,
            reason,
            alternatives,
        ));
    }

    let Some(alternative) = best.alternative(node) else {
        return;
    };
    let Some(alternative) = forest_node.alternatives.get(alternative) else {
        return;
    };
    for &child in &alternative.children {
        collect_construction_decisions(forest, child, best, tokens, registry, decisions);
    }
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
            && let Some(Lowered::NounPhrase(NounPhrase::CoordinatedNominal(group))) =
                lower(grammar, forest, node, best)
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
    match (before, after) {
        (NounPhrase::Nominal(original), NounPhrase::CoordinatedNominal(group)) => {
            let mut first = original.clone();
            let determiner = first.determiner.take()?;
            if group.determiner() != &determiner || group.first().as_ref() != &first {
                return None;
            }
            Some(SharedDeterminerEdit {
                determined_first: before.clone(),
                determiner,
                trailing_relative: first_group_relative(group.complements()),
            })
        }
        (NounPhrase::CoordinatedNominal(original), NounPhrase::CoordinatedNominal(group))
            if group.determiner() == original.determiner()
                && group.first() == original.first()
                && group.rest().len() == original.rest().len() + 1
                && group.rest().starts_with(original.rest()) =>
        {
            let mut determined_first = original.first().as_ref().clone();
            determined_first.determiner = Some(original.determiner().clone());
            Some(SharedDeterminerEdit {
                determined_first: NounPhrase::Nominal(determined_first),
                determiner: original.determiner().clone(),
                trailing_relative: first_group_relative(group.complements()),
            })
        }
        (NounPhrase::Nominal(before), NounPhrase::Nominal(after))
            if before.determiner == after.determiner
                && before.modifiers == after.modifiers
                && before.head == after.head
                && before.complements.len() == after.complements.len()
                && !before.complements.is_empty()
                && before.complements[..before.complements.len() - 1]
                    == after.complements[..after.complements.len() - 1] =>
        {
            let (
                super::NominalComplement::Prepositional(before),
                super::NominalComplement::Prepositional(after),
            ) = (before.complements.last()?, after.complements.last()?)
            else {
                return None;
            };
            if before.head().preposition != after.head().preposition {
                return None;
            }
            let (super::Phrase::NounPhrase(before), super::Phrase::NounPhrase(after)) =
                (before.head().object.as_ref(), after.head().object.as_ref())
            else {
                return None;
            };
            find_shared_determiner_edit(before, after)
        }
        (NounPhrase::Coordinated(before), NounPhrase::Coordinated(after))
            if before.first() == after.first()
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
    use crate::construction::ConstructionOwner;
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
    fn sentence_root_reports_one_generated_family_and_declared_form_ordinals() {
        // Mutation caught: restore either handwritten Sentence production.
        // Both real surfaces must select the same generated construction;
        // their independently declared ordinals preserve exact form identity.
        for (source, expected_ordinal) in [("Draw a card.", 0), ("Draw a card", 1)] {
            let parsed = parse_nonterminal(source, &fixture_catalogs(), Nonterminal::Sentence)
                .unwrap_or_else(|error| panic!("failed to parse {source:?}: {error:?}"));
            let decision = parsed
                .construction_decisions()
                .iter()
                .find(|decision| decision.selected().as_str() == "sentence")
                .unwrap_or_else(|| panic!("no sentence construction decision: {parsed:#?}"));
            assert_eq!(decision.owner(), ConstructionOwner::Generated);
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
            parsed.quantity(),
            Some(Quantity::Exact(super::super::NumberLiteral {
                value: 3,
                ..
            }))
        ));
        let decision = parsed
            .construction_decisions()
            .iter()
            .find(|decision| decision.selected().as_str() == "quantity_exact")
            .unwrap_or_else(|| panic!("no generated quantity decision: {parsed:#?}"));
        assert_eq!(decision.owner(), ConstructionOwner::Generated);
        assert_eq!(decision.selected_production_ordinal(), 0);
    }

    #[test]
    fn generated_quantity_family_parses_every_shape_notation_and_comparative() {
        let number = |value, numeral| super::super::NumberLiteral { value, numeral };
        let literal = |number| QuantityValue::Literal(number);
        let fixtures = [
            (
                "three",
                Quantity::Exact(number(3, Numeral::Cardinal)),
                "quantity_exact",
                0,
            ),
            (
                "third",
                Quantity::Exact(number(3, Numeral::Ordinal)),
                "quantity_exact",
                0,
            ),
            (
                "3",
                Quantity::Exact(number(3, Numeral::Arabic(false))),
                "quantity_exact",
                0,
            ),
            (
                "1,000",
                Quantity::Exact(number(1_000, Numeral::Arabic(true))),
                "quantity_exact",
                0,
            ),
            (
                "III",
                Quantity::Exact(number(3, Numeral::Roman)),
                "quantity_exact",
                0,
            ),
            (
                "at least two",
                Quantity::AtLeast(literal(number(2, Numeral::Cardinal))),
                "quantity_at_least",
                0,
            ),
            (
                "two or fewer",
                Quantity::OrComparison(
                    literal(number(2, Numeral::Cardinal)),
                    ComparativeWord::Fewer,
                ),
                "quantity_at_least",
                1,
            ),
            (
                "2 or greater",
                Quantity::OrComparison(
                    literal(number(2, Numeral::Arabic(false))),
                    ComparativeWord::Greater,
                ),
                "quantity_at_least",
                1,
            ),
            (
                "two or less",
                Quantity::OrComparison(
                    literal(number(2, Numeral::Cardinal)),
                    ComparativeWord::Less,
                ),
                "quantity_at_least",
                1,
            ),
            (
                "2 or more",
                Quantity::OrComparison(
                    literal(number(2, Numeral::Arabic(false))),
                    ComparativeWord::More,
                ),
                "quantity_at_least",
                1,
            ),
            (
                "one or 2",
                Quantity::Or(
                    number(1, Numeral::Cardinal),
                    number(2, Numeral::Arabic(false)),
                ),
                "quantity_or",
                0,
            ),
            ("X", Quantity::X, "quantity_x", 0),
            ("both", Quantity::Both, "quantity_both", 0),
            (
                "up to X",
                Quantity::UpTo(QuantityValue::Variable),
                "quantity_up_to",
                0,
            ),
            ("that many", Quantity::ThatMany, "quantity_that_many", 0),
            ("that much", Quantity::ThatMuch, "quantity_that_much", 0),
            (
                "more than X",
                Quantity::MoreThan(QuantityValue::Variable),
                "quantity_more_than",
                0,
            ),
            (
                "fewer than two",
                Quantity::FewerThan(literal(number(2, Numeral::Cardinal))),
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
            assert_eq!(decision.owner(), ConstructionOwner::Generated, "{source:?}");
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
                NounCardinality::PluralCount,
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

        // Whole-NP parsing is not a sound negative oracle here: the still-
        // handwritten nominal family can admit an independent attachment for
        // strings such as `two creature`. Q01 owns the quantity feature
        // boundary, so prove that the generated family itself never supplies
        // the singular cardinality that would license those combinations.
        for source in ["two", "at least one", "one or more", "both", "that many"] {
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
            Some(Clause::Independent(IndependentClause::Imperative(_)))
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
    use crate::construction::ConstructionId;
    use crate::constructions::probe;
    use crate::forest::SelectionReason;
    use crate::grammar::rules::RegistrationOrder;

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

    fn fixture_catalogs() -> Catalogs {
        Catalogs::default()
            .with_catalog(CatalogKind::KeywordAbility, ["Protection"])
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
    use crate::construction::ConstructionOwner;
    use crate::constructions::coordination;
    use crate::constructions::noun;
    use crate::constructions::probe;
    use crate::word::BareNominalAdjunct;
    use crate::word::Noun;
    use crate::word::NounInstance;
    use crate::word::Vocab;

    static COORDINATION_GROUPS: &[&deckmaste_construction_compiler::runtime::GroupData] =
        &[noun::GROUPS[0], coordination::GROUPS[0]];

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
                ) && decision.owner() == ConstructionOwner::Generated
            }),
            "no generated coordination owner recorded for {source:?}: {:#?}",
            parsed.construction_decisions(),
        );
        parsed
    }

    #[test]
    fn real_generated_binary_coordination_parses() {
        let parsed = generated_coordination("an artifact or a creature");
        let Some(NounPhrase::Coordinated(coordination)) = parsed.noun_phrase() else {
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
        let Some(NounPhrase::Coordinated(coordination)) = parsed.noun_phrase() else {
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
        let Some(NounPhrase::Coordinated(coordination)) = parsed.noun_phrase() else {
            panic!(
                "nested coordination lowered to the wrong syntax: {:?}",
                parsed.syntax
            );
        };
        assert!(
            matches!(coordination.first().as_ref(), NounPhrase::Coordinated(_))
                || coordination
                    .rest()
                    .iter()
                    .any(|member| matches!(member.phrase, NounPhrase::Coordinated(_))),
            "one binary group must be nested inside the other: {coordination:#?}",
        );
    }

    #[test]
    fn real_generated_shared_determiner_coordination_parses() {
        let parsed = generated_coordination("target artifact or creature");
        let Some(NounPhrase::CoordinatedNominal(coordination)) = parsed.noun_phrase() else {
            panic!(
                "shared-determiner coordination lowered to the wrong syntax: {:?}\n{:#?}",
                parsed.syntax,
                parsed.construction_decisions(),
            );
        };
        assert_eq!(
            coordination.determiner(),
            &crate::syntax::Determiner::Target(None)
        );
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
            assert!(parsed.construction_decisions.iter().any(|decision| {
                decision.selected().as_str() == "noun"
                    && decision.owner() == ConstructionOwner::Generated
            }));
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
                    Lowered::Noun(NounInstance::Singular(Noun::Catalog(lowered))),
                    Features::Noun {
                        identity: Some(feature_identity),
                        coordination_domain: Some(super::super::CoordinationDomain::Entity),
                        form: super::super::NounForm::Singular,
                        initial_sound: crate::word::InitialSound::Vowel,
                        adjunct: None,
                        opaque: false,
                        recipient_passive_theme: false,
                    },
                ) => assert_eq!(lowered, feature_identity),
                (
                    "turn",
                    Lowered::Noun(NounInstance::Singular(Noun::Word(Vocab::Turn))),
                    Features::Noun {
                        identity: None,
                        coordination_domain: None,
                        form: super::super::NounForm::Singular,
                        initial_sound: crate::word::InitialSound::Consonant,
                        adjunct: Some(BareNominalAdjunct::Temporal),
                        opaque: false,
                        recipient_passive_theme: false,
                    },
                )
                | (
                    "damage",
                    Lowered::Noun(NounInstance::Mass(Noun::Word(Vocab::Damage))),
                    Features::Noun {
                        identity: None,
                        coordination_domain: Some(super::super::CoordinationDomain::Damage),
                        form: super::super::NounForm::Mass,
                        initial_sound: crate::word::InitialSound::Consonant,
                        adjunct: None,
                        opaque: false,
                        recipient_passive_theme: true,
                    },
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
        assert_eq!(pair.owner(), ConstructionOwner::Generated);
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
