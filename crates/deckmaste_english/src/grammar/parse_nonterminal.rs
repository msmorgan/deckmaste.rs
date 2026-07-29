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
use super::SelfReference;
use super::SimpleClause;
use super::Span;
use super::Token;
use super::collapse_full_names;
use super::lex;
use super::lowering::Lowered;
use super::lowering::lower;
use super::parse_chart;

type EnglishChart = ChartResult<Nonterminal, EnglishLexicalSlot, Features, MeaningKey>;
pub(super) type EnglishForest = ParseForest<Nonterminal, EnglishLexicalSlot, Features, MeaningKey>;

#[derive(Debug)]
pub(crate) struct ParsedNonterminal {
    pub(crate) chart: EnglishChart,
    root: NodeId,
    best: BestParse,
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

    pub(crate) const fn cost(&self) -> ParseCost {
        self.best.cost
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
    let grammar = EnglishGrammar::with_opacity_mode(
        source,
        catalogs,
        nonterminal,
        opacity_mode,
        self_reference.clone(),
    );
    let chart = parse_chart(&grammar, tokens).map_err(ParseNonterminalError::Grammar)?;
    if chart.roots.is_empty() {
        return Err(ParseNonterminalError::NoCompleteParse(nonterminal));
    }
    let forest = &chart.forest;
    let mut syntax = None;
    let (root, best) = chart
        .forest
        .best_root_matching(chart.roots.iter().copied(), |root, best| {
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
    quoted_ability_spans.sort_unstable_by_key(|span| (span.start, span.end));
    quoted_ability_spans.dedup();
    Ok(ParsedNonterminal {
        chart,
        root,
        best,
        constituent_spans,
        quoted_ability_spans,
        syntax,
        opacity_mode,
    })
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
            .best_root(chart.roots.iter().copied())
            .map_err(ParseNonterminalError::Forest)?
            .ok_or(ParseNonterminalError::NoCompleteParse(nonterminal))?;
        let syntax =
            lower(&grammar, &chart.forest, root, &best).ok_or(ParseNonterminalError::Lowering)?;
        Ok(ParsedNonterminal {
            chart,
            root,
            best,
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
