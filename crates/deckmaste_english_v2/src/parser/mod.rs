use engine::ChartFailure;
use materialize::materialize;
use materialize::materialize_observed;
use scan::SliceGrammar;
use scan::parse_forest;
use scan::parse_forest_observed;
use selection::analyze_selection;

use crate::ast::Ability;
use crate::constructions::Category;
use crate::context::ParseContext;
use crate::environment::ParserEnvironment;

mod diagnostic;
mod engine;
mod materialize;
mod scan;
mod selection;

pub(crate) use engine::LexicalMatch;
pub(crate) use engine::Rule;
pub(crate) use engine::RulePosition;
pub(crate) use materialize::BuildValue;
pub(crate) use scan::ScanInput;
pub(crate) use scan::scan_bound_terminal;

pub(crate) use crate::constructions::Lexical;

#[cfg(test)]
mod rules_tests {
    use crate::constructions::RULES;
    use crate::constructions::RuleId;

    #[test]
    fn slice_table_has_exactly_one_row_and_build_arm_per_rule_id() {
        assert_eq!(RULES.len(), RuleId::COUNT);
        assert!(
            RULES
                .iter()
                .enumerate()
                .all(|(index, rule)| rule.id.index() == index)
        );
    }
}

pub use diagnostic::Bounded;
pub use diagnostic::BoundedParseOutcome;
pub use diagnostic::BoundedSelectionCandidate;
pub use diagnostic::BoundedSelectionComparison;
pub use diagnostic::BoundedSelectionDecision;
pub use diagnostic::ChartItem;
pub use diagnostic::CheckedCompletionRejection;
pub use diagnostic::ExpectationInfo;
pub use diagnostic::FamilyIdentity;
pub use diagnostic::FamilyIdentityChild;
pub use diagnostic::ForestChild;
pub use diagnostic::ForestFamily;
pub use diagnostic::ForestNode;
pub use diagnostic::InternalFailureKind;
pub use diagnostic::InternalFailureOutcome;
pub use diagnostic::MaterializationCycle;
pub use diagnostic::MaterializedCandidateInfo;
pub use diagnostic::ParseAnalysis;
pub use diagnostic::ParseAnalysisOutcome;
pub use diagnostic::ParseFailureOutcome;
pub use diagnostic::ParserTrace;
pub use diagnostic::ScannedToken;
pub use diagnostic::SelectedParseOutcome;
pub use diagnostic::SelectionCandidate;
pub use diagnostic::SelectionComparison;
pub use diagnostic::SelectionDecision;
pub use diagnostic::SelectionDecisive;
pub use diagnostic::SelectionLoserReason;
pub use diagnostic::SelectionResolution;
pub use diagnostic::SpecificityTier;
pub use diagnostic::TraceLimits;
pub use diagnostic::UnresolvedAmbiguityOutcome;
pub use diagnostic::UnselectedCandidate;
pub use error::Expectation;
pub use error::NonterminalCategory;
pub use error::ParseError;
pub use error::TextSpan;
pub use selection::SelectionExceptionInfo;
pub use selection::SelectionExceptionInventoryError;
pub use selection::selection_exception_inventory;

pub use crate::constructions::TerminalClass;

mod error;

#[derive(Debug, Clone)]
pub struct Parser {
    environment: ParserEnvironment,
}

impl Parser {
    #[must_use]
    pub fn new(environment: ParserEnvironment) -> Self {
        Self { environment }
    }

    /// Parses one complete ability from exact rendered text.
    ///
    /// # Errors
    ///
    /// Returns a structured failure when the checked chart cannot consume the
    /// input. Returns an ambiguity when structural selection cannot choose one
    /// reading.
    pub fn parse(&self, text: &str, context: &ParseContext<'_>) -> Result<Ability, ParseError> {
        self.analyze(text, context).into_parse_result()
    }

    /// Parses one complete ability and retains its complete selection decision.
    #[must_use]
    pub fn analyze(&self, text: &str, context: &ParseContext<'_>) -> ParseAnalysis {
        let grammar = self.grammar(context);
        parse_forest(&grammar, text).map_or_else(
            |failure| ParseAnalysis::from_result(Err(chart_failure(text, failure)), None),
            |forest| analyze_materialized(materialize(&forest, context)),
        )
    }

    /// Parses once while retaining independently bounded diagnostic
    /// projections.
    #[must_use]
    pub fn trace(
        &self,
        text: &str,
        context: &ParseContext<'_>,
        limits: TraceLimits,
    ) -> ParserTrace {
        let (analysis, trace) = self.analyze_with_trace(text, context, limits);
        ParserTrace::from_parts(
            analysis,
            trace.structural,
            trace.materialization,
            limits,
            context,
        )
    }

    #[cfg(test)]
    pub(crate) fn observe_structural(
        &self,
        text: &str,
        context: &ParseContext<'_>,
        limits: TraceLimits,
    ) -> (ParseAnalysis, diagnostic::StructuralTrace) {
        let (analysis, trace) = self.analyze_with_trace(text, context, limits);
        (analysis, trace.structural)
    }

    fn analyze_with_trace(
        &self,
        text: &str,
        context: &ParseContext<'_>,
        limits: TraceLimits,
    ) -> (ParseAnalysis, TraceParts) {
        let grammar = self.grammar(context);
        let (forest, structural) = parse_forest_observed(&grammar, text, limits);
        let (analysis, materialization) = match forest {
            Ok(forest) => {
                let (candidates, materialization) = materialize_observed(&forest, context, limits);
                (analyze_materialized(candidates), materialization)
            }
            Err(failure) => (
                ParseAnalysis::from_result(Err(chart_failure(text, failure)), None),
                diagnostic::MaterializationTrace::empty(limits.per_collection()),
            ),
        };
        (
            analysis,
            TraceParts {
                structural,
                materialization,
            },
        )
    }

    fn grammar<'a>(&'a self, context: &'a ParseContext<'a>) -> SliceGrammar<'a> {
        SliceGrammar {
            environment: &self.environment,
            context,
        }
    }

    #[cfg(test)]
    pub(crate) fn test_only_scan_terminal<'a>(
        &'a self,
        text: &'a str,
        context: &'a ParseContext<'a>,
        terminal: crate::constructions::LexicalTerminal,
        offset: usize,
    ) -> Vec<LexicalMatch<crate::constructions::Leaf>> {
        self.grammar(context).scan(terminal, text, offset)
    }
}

struct TraceParts {
    structural: diagnostic::StructuralTrace,
    materialization: diagnostic::MaterializationTrace,
}

pub(crate) fn analyze_materialized(candidates: Vec<materialize::Candidate>) -> ParseAnalysis {
    let result = analyze_selection(candidates).map(|selection| {
        let (result, decision) = selection.into_result_and_decision();
        let result = result
            .and_then(|candidate| candidate.ok_or(ParseError::ValidatedRootDidNotMaterialize));
        (result, decision)
    });
    match result {
        Ok((result, decision)) => ParseAnalysis::from_result(result, decision),
        Err(error) => ParseAnalysis::from_result(Err(error), None),
    }
}

#[cfg(test)]
mod structural_trace_tests {
    use super::Parser;
    use super::TraceLimits;
    use crate::catalogs::canonical_test_environment;
    use crate::context::ParseContext;
    use crate::environment::ParserEnvironment;

    fn environment() -> ParserEnvironment {
        canonical_test_environment()
    }

    #[test]
    fn cloned_parser_shares_frozen_environment_storage() {
        let parser = Parser::new(environment());
        let cloned = parser.clone();

        assert!(
            parser
                .environment
                .test_only_shares_storage_with(&cloned.environment)
        );
    }

    #[test]
    fn structural_trace_observation_is_repeatable_and_inert() {
        let parser = Parser::new(environment());
        let context = ParseContext::new("Trace Card").expect("valid context");
        let text = "Whenever a player connives, you gain X life.";
        let (analysis, first) = parser.observe_structural(text, &context, TraceLimits::new(1));
        let (_, second) = parser.observe_structural(text, &context, TraceLimits::new(1));
        assert_eq!(parser.parse(text, &context), analysis.into_parse_result());
        assert_eq!(first, second);
        assert_eq!(first.tokens().total(), 11);
        assert_eq!(first.tokens().shown(), 1);
    }

    #[test]
    fn structural_trace_success_collections_and_nested_caps_are_exact() {
        fn bounded<T>(value: &super::Bounded<T>, limit: usize) {
            assert_eq!(value.total(), value.shown() + value.omitted());
            assert_eq!(value.shown(), value.items().len());
            assert!(value.shown() <= limit);
        }
        let parser = Parser::new(environment());
        let context = ParseContext::new("Trace Card").expect("valid context");
        let text = "Whenever a player connives, you gain X life.";
        for limit in [0, 1, usize::MAX] {
            let (_, trace) = parser.observe_structural(text, &context, TraceLimits::new(limit));
            bounded(trace.tokens(), limit);
            bounded(trace.chart(), limit);
            bounded(trace.forest(), limit);
            bounded(trace.accepted_roots(), limit);
            bounded(trace.checked_completion_rejections(), limit);
            for node in trace.forest().items() {
                bounded(node.families(), limit);
                for family in node.families().items() {
                    bounded(family.children(), limit);
                }
            }
            if limit == 0 {
                assert!(trace.forest().items().is_empty());
            }
            if limit == usize::MAX {
                assert_eq!(trace.forest().omitted(), 0);
                assert_eq!(trace.accepted_roots().omitted(), 0);
                assert!(trace.forest().total() > 0);
                assert!(trace.accepted_roots().total() > 0);
            }
            let (_, repeated) = parser.observe_structural(text, &context, TraceLimits::new(limit));
            assert_eq!(trace, repeated);
        }
    }

    #[test]
    fn structural_trace_failure_retains_partial_forest_with_exact_bounds() {
        fn bounded<T>(value: &super::Bounded<T>, limit: usize) {
            assert_eq!(value.total(), value.shown() + value.omitted());
            assert_eq!(value.shown(), value.items().len());
            assert!(value.shown() <= limit);
        }
        let parser = Parser::new(environment());
        let context = ParseContext::new("Trace Card").expect("context");
        let text = "Whenever a player connives, you gain X life";
        for limit in [0, 1, usize::MAX] {
            let (analysis, trace) =
                parser.observe_structural(text, &context, TraceLimits::new(limit));
            assert_eq!(parser.parse(text, &context), analysis.into_parse_result());
            bounded(trace.tokens(), limit);
            bounded(trace.chart(), limit);
            bounded(trace.forest(), limit);
            bounded(trace.accepted_roots(), limit);
            bounded(trace.checked_completion_rejections(), limit);
            for node in trace.forest().items() {
                bounded(node.families(), limit);
                for family in node.families().items() {
                    bounded(family.children(), limit);
                }
            }
            assert_eq!(trace.accepted_roots().total(), 0);
            assert!(trace.forest().total() > 0);
            let (_, repeated) = parser.observe_structural(text, &context, TraceLimits::new(limit));
            assert_eq!(trace, repeated);
        }
    }

    #[test]
    fn structural_trace_observed_engine_matches_noop_success_and_failure() {
        let environment = environment();
        let parser = Parser::new(environment.clone());
        let context = ParseContext::new("Trace Card").expect("context");
        for text in [
            "Whenever a player connives, you gain X life.",
            "Whenever a player connives, you gain X life",
        ] {
            let grammar = super::SliceGrammar {
                environment: &environment,
                context: &context,
            };
            let ordinary = super::parse_forest(&grammar, text);
            let (observed, trace) =
                super::parse_forest_observed(&grammar, text, TraceLimits::new(usize::MAX));
            assert_eq!(ordinary, observed);
            let (analysis, _) =
                parser.observe_structural(text, &context, TraceLimits::new(usize::MAX));
            assert_eq!(parser.parse(text, &context), analysis.into_parse_result());
            if text.ends_with("life") {
                assert_eq!(trace.accepted_roots().total(), 0);
                assert!(trace.forest().total() > 0);
            }
        }
    }
}

fn chart_failure(text: &str, failure: ChartFailure<Category, Lexical>) -> ParseError {
    ParseError::Failure {
        span: failure_span(text, failure.offset),
        expectations: failure.live.into_iter().map(expectation).collect(),
    }
}

fn failure_span(text: &str, offset: usize) -> TextSpan {
    if offset == text.len() {
        return TextSpan {
            start: offset,
            end: offset,
        };
    }

    let start = text[offset..]
        .char_indices()
        .find(|&(_, character)| !character.is_whitespace())
        .map_or(text.len(), |(index, _)| offset + index);
    if start == text.len() {
        return TextSpan { start, end: start };
    }

    let mut characters = text[start..].char_indices();
    let (_, first) = characters.next().expect("nonempty failure span");
    if matches!(first, ',' | '.') {
        return TextSpan {
            start,
            end: start + first.len_utf8(),
        };
    }

    let end = characters
        .find(|&(_, character)| character.is_whitespace() || matches!(character, ',' | '.'))
        .map_or(text.len(), |(index, _)| start + index);
    TextSpan { start, end }
}

const fn expectation(position: RulePosition<Category, Lexical>) -> Expectation {
    match position {
        RulePosition::Nonterminal(category) => {
            Expectation::Nonterminal(nonterminal_category(category))
        }
        RulePosition::Lexical(Lexical::Literal(literal)) => Expectation::Literal(literal),
        RulePosition::Lexical(lexical) => Expectation::Terminal(terminal_class(lexical)),
    }
}

const fn nonterminal_category(category: Category) -> NonterminalCategory {
    match category {
        Category::Ability => NonterminalCategory::Ability,
        Category::Sentence => NonterminalCategory::Sentence,
        Category::Clause => NonterminalCategory::Clause,
        Category::NounPhrase => NonterminalCategory::NounPhrase,
        Category::VerbPhrase => NonterminalCategory::VerbPhrase,
        Category::Amount => NonterminalCategory::Amount,
    }
}

const fn terminal_class(lexical: Lexical) -> TerminalClass {
    lexical.class()
}
