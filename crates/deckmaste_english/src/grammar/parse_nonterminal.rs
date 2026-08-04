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
        super::generated::GeneratedActivation::Inactive,
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
    match activation.groups() {
        None => SelectedRegistry::Static(super::construction::registry()),
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
            production.construction,
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
    let Some((tag, children)) = selected_tag_and_children(grammar, forest, node, best) else {
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

    if tag == super::RuleTag::SharedDeterminerNominal {
        let (Some(first), Some(last)) = (children.get(1), children.get(3)) else {
            return;
        };
        if let Some(span) = token_range_span(
            tokens,
            forest.node(*first).key.start,
            forest.node(*last).key.end,
        ) {
            spans.push(span);
        }
        return;
    }

    let (first_node, next_node, before, after) = match tag {
        super::RuleTag::NounPhraseCoordination | super::RuleTag::NounPhraseCoordinationOxford => {
            let next_index = if tag == super::RuleTag::NounPhraseCoordination { 2 } else { 3 };
            let (Some(first_node), Some(next_node)) = (children.first(), children.get(next_index))
            else {
                return;
            };
            let Some(Lowered::NounPhrase(before)) = lower(grammar, forest, *first_node, best)
            else {
                return;
            };
            let Some(Lowered::NounPhrase(after)) = lower(grammar, forest, node, best) else {
                return;
            };
            (*first_node, *next_node, before, after)
        }
        super::RuleTag::PrepositionalPhraseCoordinated
        | super::RuleTag::PrepositionalPhraseRulesObjectCoordinated => {
            let next_index = if children.len() == 5 { 4 } else { 3 };
            let (Some(first_node), Some(next_node)) = (children.get(1), children.get(next_index))
            else {
                return;
            };
            let Some(Lowered::NounPhrase(before)) = lower(grammar, forest, *first_node, best)
            else {
                return;
            };
            let Some(Lowered::PrepositionalPhrase(after)) = lower(grammar, forest, node, best)
            else {
                return;
            };
            let crate::syntax::PrepositionalPhrase::Simple(after) = after else {
                return;
            };
            let super::Phrase::NounPhrase(after) = *after.object else {
                return;
            };
            (*first_node, *next_node, before, *after)
        }
        _ => return,
    };
    let Some(edit) = find_shared_determiner_edit(&before, &after) else {
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

fn selected_tag_and_children(
    grammar: &EnglishGrammar<'_, '_>,
    forest: &EnglishForest,
    node: NodeId,
    best: &BestParse,
) -> Option<(super::RuleTag, Vec<NodeId>)> {
    let forest_node = forest.node(node);
    let alternative = forest_node.alternatives.get(best.alternative(node)?)?;
    let rule = alternative.rule?;
    let tag = match grammar.impls.get(rule.index())? {
        super::rules::RuleImpl::Handwritten(tag) => *tag,
        // Handwritten-only consumers (coordination-span collection); a
        // generated node simply isn't one of theirs.
        super::rules::RuleImpl::Generated(_) => return None,
    };
    let [intermediate] = alternative.children.as_slice() else {
        return None;
    };
    let mut children = Vec::new();
    selected_rule_children(forest, *intermediate, best, &mut children)?;
    Some((tag, children))
}

#[derive(Debug)]
struct SharedDeterminerEdit {
    determined_first: NounPhrase,
    determiner: super::Determiner,
    trailing_relative: Option<super::RelativeClause>,
}

/// Finds the one subtree changed by `push_noun_phrase_coordination`. The
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
            if group.determiner != determiner || group.first.as_ref() != &first {
                return None;
            }
            Some(SharedDeterminerEdit {
                determined_first: before.clone(),
                determiner,
                trailing_relative: first_group_relative(&group.complements),
            })
        }
        (NounPhrase::CoordinatedNominal(original), NounPhrase::CoordinatedNominal(group))
            if group.determiner == original.determiner
                && group.first == original.first
                && group.rest.len() == original.rest.len() + 1
                && group.rest.starts_with(&original.rest) =>
        {
            let mut determined_first = original.first.as_ref().clone();
            determined_first.determiner = Some(original.determiner.clone());
            Some(SharedDeterminerEdit {
                determined_first: NounPhrase::Nominal(determined_first),
                determiner: original.determiner.clone(),
                trailing_relative: first_group_relative(&group.complements),
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
            if before.first == after.first
                && before.rest.len() == after.rest.len()
                && !before.rest.is_empty()
                && before.rest[..before.rest.len() - 1] == after.rest[..after.rest.len() - 1] =>
        {
            let before = before.rest.last()?;
            let after = after.rest.last()?;
            if before.conjunction != after.conjunction || before.comma != after.comma {
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
    let (_, children) = selected_tag_and_children(grammar, forest, node, best)?;
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
    if let Some((_, children)) = selected_tag_and_children(grammar, forest, node, best) {
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
    let (_, children) = selected_tag_and_children(grammar, forest, node, best)?;
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
    use crate::syntax::IndependentClause;

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
                GeneratedActivation::Inactive,
            );
            assert_eq!(
                normalized_parse(
                    source,
                    nonterminal,
                    RegistrationOrder::Reversed,
                    GeneratedActivation::Inactive,
                ),
                normal,
                "reversed registration changed {source:?}",
            );
            assert_eq!(
                normalized_parse(
                    source,
                    nonterminal,
                    RegistrationOrder::FixedShuffle,
                    GeneratedActivation::Inactive,
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
    use crate::constructions::probe;

    fn probe_category(name: &str) -> Nonterminal {
        let cats = super::super::generated::internal_categories(probe::GROUPS);
        Nonterminal::Generated(cats[name])
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
            matches!(parsed.syntax, Lowered::Ignored),
            "probe lowering is Ignored: {:?}",
            parsed.syntax
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
