use engine::ChartFailure;
use materialize::materialize;
use scan::SliceGrammar;
use scan::parse_forest;
use scan::parse_forest_observed;
use selection::analyze_selection;

use crate::ast::Ability;
use crate::catalogs::ParserCatalogs;
use crate::constructions::Category;
use crate::context::ParseContext;

mod diagnostic;
mod engine;
mod lexical;
mod materialize;
mod scan;
mod selection;

pub(crate) use engine::Rule;
pub(crate) use engine::RulePosition;
pub(crate) use lexical::Lexical;
pub(crate) use lexical::NounNumber;
pub(crate) use materialize::BuildValue;
pub(crate) use scan::Leaf;

#[cfg(test)]
mod build {
    pub(crate) use crate::features::Agreement;
}

#[cfg(test)]
#[rustfmt::skip]
mod rules {
pub(crate) use super::lexical::Lexical;
pub(crate) use super::lexical::NounNumber;
pub(crate) use crate::constructions::Category;
pub(crate) use crate::constructions::Construction;
use crate::constructions::RULES;
use crate::constructions::RuleId;

#[cfg(test)]
mod tests {
    use super::RULES;
    use super::RuleId;

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
}

pub use diagnostic::Bounded;
pub use diagnostic::InternalFailureKind;
pub use diagnostic::ParseAnalysis;
pub use diagnostic::ParseAnalysisOutcome;
pub use diagnostic::SelectionCandidate;
pub use diagnostic::SelectionComparison;
pub use diagnostic::SelectionDecision;
pub use diagnostic::SelectionDecisive;
pub use diagnostic::SelectionResolution;
pub use diagnostic::SpecificityTier;
pub use diagnostic::TraceLimits;
pub use error::Expectation;
pub use error::NonterminalCategory;
pub use error::ParseError;
pub use error::TerminalClass;
pub use error::TextSpan;
pub use selection::SelectionExceptionInfo;
pub use selection::SelectionExceptionInventoryError;
pub use selection::selection_exception_inventory;

mod error;

#[derive(Debug, Clone)]
pub struct Parser {
    catalogs: ParserCatalogs,
}

impl Parser {
    #[must_use]
    pub fn new(catalogs: ParserCatalogs) -> Self {
        Self { catalogs }
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
        self.analyze_with_trace(text, context, None).0
    }

    #[allow(
        dead_code,
        reason = "Task 6 exposes this internal trace seam publicly."
    )]
    pub(crate) fn observe_structural(
        &self,
        text: &str,
        context: &ParseContext<'_>,
        limits: TraceLimits,
    ) -> (ParseAnalysis, diagnostic::StructuralTrace) {
        self.analyze_with_trace(text, context, Some(limits))
    }

    fn analyze_with_trace(
        &self,
        text: &str,
        context: &ParseContext<'_>,
        limits: Option<TraceLimits>,
    ) -> (ParseAnalysis, diagnostic::StructuralTrace) {
        let grammar = SliceGrammar {
            catalogs: &self.catalogs,
            context,
        };
        let (forest, trace) = match limits {
            Some(limits) => parse_forest_observed(&grammar, text, limits),
            None => (
                parse_forest(&grammar, text),
                diagnostic::StructuralTrace::empty(),
            ),
        };
        let result = forest
            .map_err(|failure| chart_failure(text, failure))
            .and_then(|forest| {
                let selection = analyze_selection(materialize(&forest, context))?;
                let (result, decision) = selection.into_result_and_decision();
                let result = result.and_then(|candidate| {
                    candidate.ok_or(ParseError::ValidatedRootDidNotMaterialize)
                });
                Ok((result, decision))
            });
        let analysis = match result {
            Ok((result, decision)) => ParseAnalysis::from_result(result, decision),
            Err(error) => ParseAnalysis::from_result(Err(error), None),
        };
        (analysis, trace)
    }
}

#[cfg(test)]
mod structural_trace_tests {
    use std::path::Path;

    use super::Parser;
    use super::TraceLimits;
    use crate::catalogs::ParserCatalogs;
    use crate::context::ParseContext;

    #[test]
    fn structural_trace_observation_is_repeatable_and_inert() {
        let catalogs = ParserCatalogs::load(
            &Path::new(env!("CARGO_MANIFEST_DIR")).join("../../data/gen/catalogs"),
        )
        .expect("canonical generated catalogs load");
        let parser = Parser::new(catalogs);
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
        let catalogs = ParserCatalogs::load(
            &Path::new(env!("CARGO_MANIFEST_DIR")).join("../../data/gen/catalogs"),
        )
        .expect("canonical generated catalogs load");
        let parser = Parser::new(catalogs);
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
        let catalogs = ParserCatalogs::load(
            &Path::new(env!("CARGO_MANIFEST_DIR")).join("../../data/gen/catalogs"),
        )
        .expect("catalogs");
        let parser = Parser::new(catalogs);
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
        let catalogs = ParserCatalogs::load(
            &Path::new(env!("CARGO_MANIFEST_DIR")).join("../../data/gen/catalogs"),
        )
        .expect("catalogs");
        let parser = Parser::new(catalogs.clone());
        let context = ParseContext::new("Trace Card").expect("context");
        for text in [
            "Whenever a player connives, you gain X life.",
            "Whenever a player connives, you gain X life",
        ] {
            let grammar = super::SliceGrammar {
                catalogs: &catalogs,
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
    match lexical {
        Lexical::Literal(_) => unreachable!(),
        Lexical::EndOfInput => TerminalClass::EndOfInput,
        Lexical::TriggerWord => TerminalClass::TriggerWord,
        Lexical::Article => TerminalClass::Article,
        Lexical::Demonstrative => TerminalClass::Demonstrative,
        Lexical::Pronoun => TerminalClass::Pronoun,
        Lexical::Variable => TerminalClass::Variable,
        Lexical::Noun(_) => TerminalClass::Noun,
        Lexical::Verb(_) => TerminalClass::VerbLexeme,
        Lexical::SignedNumber => TerminalClass::SignedNumber,
        Lexical::SelfReference => TerminalClass::SelfReference,
    }
}
