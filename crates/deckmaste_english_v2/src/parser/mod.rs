use deckmaste_construction_core::macro_def::DeclarationKind;
use deckmaste_construction_core::macro_def::GrammarPosition;
use deckmaste_construction_core::macro_def::SurfaceFeature;
use engine::ChartFailure;
use materialize::materialize_checked;
use materialize::materialize_observed_checked;
use ownership::validate_ownership;
use scan::SliceGrammar;
use scan::parse_forest;
use scan::parse_forest_observed;
use selection::analyze_selection;

use crate::ast::Ability;
use crate::ast::CardinalQuantity;
use crate::ast::CountReference;
use crate::ast::MannerReference;
use crate::ast::OracleText;
use crate::ast::ScalarReference;
use crate::ast::Sentence;
use crate::constructions::CatalogProvider;
use crate::constructions::Category;
use crate::constructions::GeneratedParseRoot;
use crate::constructions::REQUIRED_CATALOG_PROVIDERS;
use crate::context::ParseContext;
use crate::environment::DeclarationId;
use crate::environment::ParserEnvironment;

mod diagnostic;
macro_rules! engine_unit_tests {
    ($tests:item) => {
        $tests
    };
}
mod engine;
#[cfg(test)]
mod homonym_pipeline;
mod materialize;
pub(crate) mod ownership;
mod scan;
mod selection;

pub(crate) use engine::LexicalMatch;
pub(crate) use engine::Rule;
pub(crate) use engine::RulePosition;
pub(crate) use scan::ScanInput;

#[allow(
    unused_imports,
    reason = "the parser module preserves the generated materialization carrier import boundary"
)]
pub(crate) use crate::constructions::BuildValue;
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
pub use diagnostic::ScannerMatch;
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
pub use error::ParseError;
pub use error::TextSpan;
pub use ownership::ByteMismatchScope;
pub use ownership::InvalidSpanKind;
pub use ownership::LexicalClaim;
pub use ownership::OwnershipFailure;
pub use ownership::OwnershipSummary;
pub use ownership::SelectedOwnership;
pub use selection::SelectionExceptionInfo;
pub use selection::SelectionExceptionInventoryError;
#[cfg(feature = "test-support")]
pub use selection::exception_decision_for_test;
pub use selection::selection_exception_inventory;

pub use crate::constructions::BuildRejection;
pub use crate::constructions::BuildViolation;
pub use crate::constructions::LexicalProvenanceKind;
pub use crate::constructions::NonterminalCategory;
pub use crate::constructions::TerminalClass;

#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum ParserBuildError {
    #[error("required catalog provider {provider:?} is missing from the parser environment")]
    MissingCatalogProvider { provider: CatalogProvider },
    #[error("required declaration {kind} `{name}` is missing from the parser environment")]
    MissingDeclaration { kind: DeclarationKind, name: String },
    #[error(
        "required declaration {identity} has grammar position {actual:?}, expected {expected:?}"
    )]
    WrongGrammarPosition {
        identity: DeclarationId,
        expected: GrammarPosition,
        actual: Option<GrammarPosition>,
    },
    #[error("required declaration {identity} has no {feature:?} surface")]
    MissingSurfaceFeature {
        identity: DeclarationId,
        feature: SurfaceFeature,
    },
}

mod error;
#[cfg(feature = "parser-metrics")]
mod metrics;
#[cfg(feature = "parser-metrics")]
pub use metrics::ConstructionMetrics;
#[cfg(feature = "parser-metrics")]
pub use metrics::parser_metrics;
#[cfg(feature = "parser-metrics")]
pub use metrics::parser_work_metrics;

#[cfg(test)]
#[derive(Clone, Copy)]
enum PipelineStage {
    Parse,
    Materialize,
    Specificity,
    Selection,
    Ownership,
}

#[cfg(test)]
thread_local! {
    static PIPELINE_COUNTS: std::cell::Cell<[usize; 5]> = const {
        std::cell::Cell::new([0; 5])
    };
}

#[cfg(feature = "test-support")]
thread_local! {
    static PARSER_ENTRY_CALLS: std::cell::RefCell<Vec<(ParserEntryPoint, String, String)>> = const {
        std::cell::RefCell::new(Vec::new())
    };
}

/// A public parser entry point observed by cross-crate tests.
#[doc(hidden)]
#[cfg(feature = "test-support")]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ParserEntryPoint {
    AnalyzeAbility,
    AnalyzeSentence,
    AnalyzeOracleText,
    TraceAbility,
    TraceSentence,
    TraceOracleText,
}

#[cfg(feature = "test-support")]
fn record_parser_entry_call(entry: ParserEntryPoint, text: &str, context: &ParseContext<'_>) {
    PARSER_ENTRY_CALLS.with(|calls| {
        calls
            .borrow_mut()
            .push((entry, text.to_owned(), context.card_name().to_owned()));
    });
}

/// Clears the production parser-entry log used by cross-crate tests.
#[doc(hidden)]
#[cfg(feature = "test-support")]
pub fn reset_parser_entry_calls_for_test() {
    PARSER_ENTRY_CALLS.with(|calls| calls.borrow_mut().clear());
}

/// Takes the exact `(entry point, text, context)` sequence observed by public
/// parser entry methods.
#[doc(hidden)]
#[cfg(feature = "test-support")]
#[must_use]
pub fn take_parser_entry_calls_for_test() -> Vec<(ParserEntryPoint, String, String)> {
    PARSER_ENTRY_CALLS.with(|calls| std::mem::take(&mut *calls.borrow_mut()))
}

/// Runs a cross-crate test with the production ownership-inspection failure
/// path enabled for ordinary `Parser::analyze` calls.
#[doc(hidden)]
#[cfg(feature = "test-support")]
pub fn with_forced_ownership_inspection_failure_for_test<T>(run: impl FnOnce() -> T) -> T {
    struct ResetOwnershipInspectionFailure;

    impl Drop for ResetOwnershipInspectionFailure {
        fn drop(&mut self) {
            ownership::force_inspection_corruption(false);
        }
    }

    ownership::force_inspection_corruption(true);
    let _reset = ResetOwnershipInspectionFailure;
    run()
}

#[cfg(test)]
fn count_pipeline_stage(stage: PipelineStage) {
    PIPELINE_COUNTS.with(|counts| {
        let mut next = counts.get();
        next[stage as usize] += 1;
        counts.set(next);
    });
}

#[derive(Debug, Clone)]
pub struct Parser {
    environment: ParserEnvironment,
}

impl Parser {
    /// Builds a parser after validating every catalog provider requested by
    /// the generated grammar.
    ///
    /// # Errors
    /// Returns a typed error when a required catalog provider is absent from
    /// the supplied environment.
    pub fn new(environment: ParserEnvironment) -> Result<Self, ParserBuildError> {
        validate_required_declarations(&environment)?;
        Ok(Self { environment })
    }

    #[must_use]
    pub fn environment(&self) -> &ParserEnvironment {
        &self.environment
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
        #[cfg(feature = "test-support")]
        record_parser_entry_call(ParserEntryPoint::AnalyzeAbility, text, context);
        self.analyze_root::<Ability>(text, context)
    }

    /// Parses one focused sentence from exact rendered text.
    ///
    /// # Errors
    ///
    /// Returns a structured failure when the checked chart cannot consume the
    /// input. Returns an ambiguity when structural selection cannot choose one
    /// reading.
    pub fn parse_sentence(
        &self,
        text: &str,
        context: &ParseContext<'_>,
    ) -> Result<Sentence, ParseError> {
        self.analyze_sentence(text, context).into_parse_result()
    }

    /// Parses one focused sentence and retains its complete selection decision.
    #[must_use]
    pub fn analyze_sentence(
        &self,
        text: &str,
        context: &ParseContext<'_>,
    ) -> ParseAnalysis<Sentence> {
        #[cfg(feature = "test-support")]
        record_parser_entry_call(ParserEntryPoint::AnalyzeSentence, text, context);
        self.analyze_root::<Sentence>(text, context)
    }

    /// Parses one canonical English cardinal quantity.
    ///
    /// # Errors
    ///
    /// Returns a structured failure when the input is not the canonical
    /// spelling of a value in the generated unsigned magnitude domain.
    pub fn parse_cardinal_quantity(
        &self,
        text: &str,
        context: &ParseContext<'_>,
    ) -> Result<CardinalQuantity, ParseError> {
        self.analyze_cardinal_quantity(text, context)
            .into_parse_result()
    }

    /// Parses one canonical English cardinal and retains its selection
    /// decision.
    #[must_use]
    pub fn analyze_cardinal_quantity(
        &self,
        text: &str,
        context: &ParseContext<'_>,
    ) -> ParseAnalysis<CardinalQuantity> {
        self.analyze_root::<CardinalQuantity>(text, context)
    }

    /// Parses the closed manner deictic category.
    ///
    /// # Errors
    ///
    /// Returns a structured failure unless the input is a complete generated
    /// manner reference.
    pub fn parse_manner_reference(
        &self,
        text: &str,
        context: &ParseContext<'_>,
    ) -> Result<MannerReference, ParseError> {
        self.analyze_manner_reference(text, context)
            .into_parse_result()
    }

    /// Parses a manner reference and retains its complete selection decision.
    #[must_use]
    pub fn analyze_manner_reference(
        &self,
        text: &str,
        context: &ParseContext<'_>,
    ) -> ParseAnalysis<MannerReference> {
        self.analyze_root::<MannerReference>(text, context)
    }

    /// Parses the closed count-deictic category.
    ///
    /// # Errors
    ///
    /// Returns a structured failure unless the input is a complete generated
    /// count reference.
    pub fn parse_count_reference(
        &self,
        text: &str,
        context: &ParseContext<'_>,
    ) -> Result<CountReference, ParseError> {
        self.analyze_count_reference(text, context)
            .into_parse_result()
    }

    /// Parses a count reference and retains its complete selection decision.
    #[must_use]
    pub fn analyze_count_reference(
        &self,
        text: &str,
        context: &ParseContext<'_>,
    ) -> ParseAnalysis<CountReference> {
        self.analyze_root::<CountReference>(text, context)
    }

    /// Parses the closed scalar-deictic category.
    ///
    /// # Errors
    ///
    /// Returns a structured failure unless the input is a complete generated
    /// scalar reference.
    pub fn parse_scalar_reference(
        &self,
        text: &str,
        context: &ParseContext<'_>,
    ) -> Result<ScalarReference, ParseError> {
        self.analyze_scalar_reference(text, context)
            .into_parse_result()
    }

    /// Parses a scalar reference and retains its complete selection decision.
    #[must_use]
    pub fn analyze_scalar_reference(
        &self,
        text: &str,
        context: &ParseContext<'_>,
    ) -> ParseAnalysis<ScalarReference> {
        self.analyze_root::<ScalarReference>(text, context)
    }

    /// Parses one complete normalized Oracle-text document.
    ///
    /// # Errors
    ///
    /// Returns a structured failure when the checked chart cannot consume the
    /// input. Returns an ambiguity when structural selection cannot choose one
    /// reading.
    pub fn parse_oracle_text(
        &self,
        text: &str,
        context: &ParseContext<'_>,
    ) -> Result<OracleText, ParseError> {
        self.analyze_oracle_text(text, context).into_parse_result()
    }

    /// Parses one complete normalized Oracle-text document and retains its
    /// complete selection decision.
    #[must_use]
    pub fn analyze_oracle_text(
        &self,
        text: &str,
        context: &ParseContext<'_>,
    ) -> ParseAnalysis<OracleText> {
        #[cfg(feature = "test-support")]
        record_parser_entry_call(ParserEntryPoint::AnalyzeOracleText, text, context);
        self.analyze_root::<OracleText>(text, context)
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
        #[cfg(feature = "test-support")]
        record_parser_entry_call(ParserEntryPoint::TraceAbility, text, context);
        let (analysis, trace) = self.analyze_root_with_trace::<Ability>(text, context, limits);
        ParserTrace::from_parts(
            analysis,
            trace.structural,
            trace.materialization,
            limits,
            context,
            &self.environment,
        )
    }

    /// Parses one focused sentence while retaining independently bounded
    /// diagnostic projections.
    #[must_use]
    pub fn trace_sentence(
        &self,
        text: &str,
        context: &ParseContext<'_>,
        limits: TraceLimits,
    ) -> ParserTrace<Sentence> {
        #[cfg(feature = "test-support")]
        record_parser_entry_call(ParserEntryPoint::TraceSentence, text, context);
        let (analysis, trace) = self.analyze_root_with_trace::<Sentence>(text, context, limits);
        ParserTrace::from_parts(
            analysis,
            trace.structural,
            trace.materialization,
            limits,
            context,
            &self.environment,
        )
    }

    /// Parses one complete normalized Oracle-text document while retaining
    /// independently bounded diagnostic projections.
    #[must_use]
    pub fn trace_oracle_text(
        &self,
        text: &str,
        context: &ParseContext<'_>,
        limits: TraceLimits,
    ) -> ParserTrace<OracleText> {
        #[cfg(feature = "test-support")]
        record_parser_entry_call(ParserEntryPoint::TraceOracleText, text, context);
        let (analysis, trace) = self.analyze_root_with_trace::<OracleText>(text, context, limits);
        ParserTrace::from_parts(
            analysis,
            trace.structural,
            trace.materialization,
            limits,
            context,
            &self.environment,
        )
    }

    /// Exercises the production ownership-inspection failure path for
    /// cross-crate diagnostic authentication.
    #[doc(hidden)]
    #[cfg(feature = "test-support")]
    #[must_use]
    pub fn trace_with_ownership_inspection_failure_for_test(
        &self,
        text: &str,
        context: &ParseContext<'_>,
        limits: TraceLimits,
    ) -> ParserTrace {
        ownership::force_inspection_corruption(true);
        let (analysis, trace) = self.analyze_root_with_trace::<Ability>(text, context, limits);
        ownership::force_inspection_corruption(false);
        ParserTrace::from_parts(
            analysis,
            trace.structural,
            trace.materialization,
            limits,
            context,
            &self.environment,
        )
    }

    /// Exercises a selected trace with typed ownership-failure evidence for
    /// cross-crate diagnostic authentication.
    #[doc(hidden)]
    #[cfg(any(test, feature = "test-support"))]
    #[must_use]
    pub fn trace_with_ownership_failure_for_test(
        &self,
        text: &str,
        context: &ParseContext<'_>,
        limits: TraceLimits,
    ) -> ParserTrace {
        ownership::force_synthetic_failure(true);
        let (analysis, trace) = self.analyze_root_with_trace::<Ability>(text, context, limits);
        ownership::force_synthetic_failure(false);
        ParserTrace::from_parts(
            analysis,
            trace.structural,
            trace.materialization,
            limits,
            context,
            &self.environment,
        )
    }

    #[cfg(test)]
    pub(crate) fn observe_structural(
        &self,
        text: &str,
        context: &ParseContext<'_>,
        limits: TraceLimits,
    ) -> (ParseAnalysis, diagnostic::StructuralTrace) {
        let (analysis, trace) = self.analyze_root_with_trace::<Ability>(text, context, limits);
        (analysis, trace.structural)
    }

    fn analyze_root<R: GeneratedParseRoot>(
        &self,
        text: &str,
        context: &ParseContext<'_>,
    ) -> ParseAnalysis<R> {
        let grammar = self.grammar(context);
        parse_forest::<R>(&grammar, text).map_or_else(
            |failure| ParseAnalysis::from_result(Err(chart_failure(text, failure)), None),
            |forest| {
                analyze_materialization_with_ownership::<R>(
                    text,
                    materialize_checked::<R>(&forest, context, &self.environment),
                    context,
                    &self.environment,
                )
            },
        )
    }

    fn analyze_root_with_trace<R: GeneratedParseRoot>(
        &self,
        text: &str,
        context: &ParseContext<'_>,
        limits: TraceLimits,
    ) -> (ParseAnalysis<R>, TraceParts) {
        let grammar = self.grammar(context);
        let (forest, mut structural) = parse_forest_observed::<R>(&grammar, text, limits);
        let (analysis, materialization) = match forest {
            Ok(forest) => {
                let (result, mut materialization) =
                    materialize_observed_checked::<R>(&forest, context, &self.environment, limits);
                structural.project_terminal_build_rejection(result.first_rejection);
                materialization.project_terminal_build_rejection(result.first_rejection);
                if let Some(rejection) = result.first_rejection.as_ref() {
                    debug_assert_eq!(structural.first_build_rejection(), Some(rejection));
                    debug_assert_eq!(materialization.first_build_rejection(), Some(rejection));
                }
                (
                    analyze_materialization_with_ownership::<R>(
                        text,
                        result,
                        context,
                        &self.environment,
                    ),
                    materialization,
                )
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
    ) -> Vec<LexicalMatch<crate::constructions::Leaf, crate::constructions::LexicalOwner>> {
        self.grammar(context).scan(terminal, text, offset)
    }
}

fn validate_required_declarations(environment: &ParserEnvironment) -> Result<(), ParserBuildError> {
    for &provider in REQUIRED_CATALOG_PROVIDERS {
        if !environment.has_catalog_provider(provider) {
            return Err(ParserBuildError::MissingCatalogProvider { provider });
        }
    }
    Ok(())
}

struct TraceParts {
    structural: diagnostic::StructuralTrace,
    materialization: diagnostic::MaterializationTrace,
}

#[cfg(test)]
pub(crate) fn analyze_materialized(candidates: Vec<materialize::Candidate>) -> ParseAnalysis {
    analyze_selected_candidate(candidates).map_or_else(
        |error| ParseAnalysis::from_result(Err(error), None),
        |(result, decision)| {
            ParseAnalysis::from_result(result.map(|candidate| candidate.value), decision)
        },
    )
}

type SelectedCandidate<V> = Result<
    (
        Result<materialize::Candidate<V>, ParseError>,
        Option<SelectionDecision>,
    ),
    ParseError,
>;

#[cfg(test)]
fn analyze_selected_candidate<V>(
    candidates: Vec<materialize::Candidate<V>>,
) -> SelectedCandidate<V> {
    analyze_selected_candidate_with_rejection(candidates, None, TextSpan { start: 0, end: 0 })
}

fn analyze_selected_candidate_with_rejection<V>(
    candidates: Vec<materialize::Candidate<V>>,
    first_rejection: Option<BuildRejection>,
    span: TextSpan,
) -> SelectedCandidate<V> {
    #[cfg(test)]
    count_pipeline_stage(PipelineStage::Selection);
    analyze_selection(candidates).map(|selection| {
        let (result, decision) = selection.into_result_and_decision();
        let result = result.and_then(|candidate| {
            candidate.ok_or_else(|| {
                first_rejection.map_or(ParseError::ValidatedRootDidNotMaterialize, |rejection| {
                    ParseError::BuildRejected { span, rejection }
                })
            })
        });
        (result, decision)
    })
}

fn analyze_materialization_with_ownership<R: GeneratedParseRoot>(
    text: &str,
    materialized: materialize::MaterializationResult<R>,
    context: &ParseContext<'_>,
    environment: &ParserEnvironment,
) -> ParseAnalysis<R> {
    analyze_materialized_with_ownership_and_rejection(
        text,
        materialized.candidates,
        materialized.first_rejection,
        context,
        environment,
    )
}

#[cfg(test)]
fn analyze_materialized_with_ownership<R: GeneratedParseRoot>(
    text: &str,
    candidates: Vec<materialize::Candidate<R>>,
    context: &ParseContext<'_>,
    environment: &ParserEnvironment,
) -> ParseAnalysis<R> {
    analyze_materialized_with_ownership_and_rejection(text, candidates, None, context, environment)
}

fn analyze_materialized_with_ownership_and_rejection<R: GeneratedParseRoot>(
    text: &str,
    candidates: Vec<materialize::Candidate<R>>,
    first_rejection: Option<BuildRejection>,
    context: &ParseContext<'_>,
    environment: &ParserEnvironment,
) -> ParseAnalysis<R> {
    let (result, decision) = match analyze_selected_candidate_with_rejection(
        candidates,
        first_rejection,
        TextSpan {
            start: 0,
            end: text.len(),
        },
    ) {
        Ok(selected) => selected,
        Err(error) => return ParseAnalysis::from_result(Err(error), None),
    };
    match result {
        Ok(candidate) => {
            let (rendered_text, rendered_claims) =
                R::render_with_claims(&candidate.value, context, environment);
            #[cfg(test)]
            count_pipeline_stage(PipelineStage::Ownership);
            let ownership = validate_ownership(
                text,
                &candidate.claims,
                &candidate.synthetic_claims,
                rendered_text,
                &rendered_claims,
            );
            match ownership {
                Ok(ownership) => ParseAnalysis::from_result(Ok(candidate.value), decision)
                    .with_ownership(ownership),
                Err(_) => {
                    ParseAnalysis::from_result(Err(ParseError::OwnershipInspection), decision)
                }
            }
        }
        Err(error) => ParseAnalysis::from_result(Err(error), decision),
    }
}

#[cfg(test)]
mod structural_trace_tests {
    use super::PIPELINE_COUNTS;
    use super::Parser;
    use super::TraceLimits;
    use crate::context::ParseContext;
    use crate::environment::ParserEnvironment;
    use crate::environment::canonical_test_environment;

    fn environment() -> ParserEnvironment {
        canonical_test_environment()
    }

    fn assert_pipeline_once(run: impl FnOnce()) {
        PIPELINE_COUNTS.with(|counts| counts.set([0; 5]));
        run();
        PIPELINE_COUNTS.with(|counts| {
            assert_eq!(
                counts.get(),
                [1, 1, 1, 1, 1],
                "scan/parse, materialize, specificity, selection, and ownership each run once",
            );
        });
    }

    #[test]
    fn realized_possessive_surfaces_select_render_visit_and_own_exactly() {
        use crate::constructions::Possessive;
        use crate::render::Render as _;
        use crate::visit::Visitor;

        #[derive(Default)]
        struct Recorder {
            self_references: Vec<crate::constructions::SelfReferenceSpelling>,
            nouns: Vec<crate::constructions::CommonNoun>,
            declarations: Vec<(
                deckmaste_construction_core::macro_def::DeclarationKind,
                String,
            )>,
        }

        impl Visitor for Recorder {
            fn visit_self_reference_spelling(
                &mut self,
                spelling: crate::constructions::SelfReferenceSpelling,
            ) {
                self.self_references.push(spelling);
            }

            fn visit_common_noun(&mut self, noun: crate::constructions::CommonNoun) {
                self.nouns.push(noun);
            }

            fn visit_declaration(
                &mut self,
                noun: &deckmaste_construction_core::macro_def::DeclarationIdentity,
            ) {
                self.declarations
                    .push((noun.kind(), noun.name().to_owned()));
            }
        }

        let parser = Parser::new(environment()).expect("canonical environment satisfies grammar");
        let cases = [
            ("Daxos's", "Daxos, Blessed by the Sun"),
            ("Players'", "Context Card"),
            ("Merfolk's", "Context Card"),
            ("Equipment's", "Context Card"),
        ];
        for (text, card_name) in cases {
            let context = ParseContext::new(
                card_name,
                card_name == "Daxos, Blessed by the Sun",
                deckmaste_construction_core::macro_def::Onset::Consonant,
            )
            .unwrap();
            let analysis = parser.analyze_root::<Possessive>(text, &context);
            let selected = analysis.selected().unwrap_or_else(|| {
                panic!("{text:?} did not select exactly one possessive: {analysis:#?}")
            });
            assert_eq!(
                selected.render(&context, parser.environment()),
                text,
                "possessive realization is byte-exact",
            );
            let ownership = analysis
                .ownership()
                .expect("selected possessive has ownership");
            assert!(ownership.failures().is_empty(), "{text}: {ownership:#?}");
            assert!(ownership.summary().covered());
            assert_eq!(
                ownership
                    .parsed_claims()
                    .iter()
                    .map(|claim| (claim.span(), claim.kind(), claim.stable_owner_id()))
                    .collect::<Vec<_>>(),
                ownership
                    .rendered_claims()
                    .iter()
                    .map(|claim| (claim.span(), claim.kind(), claim.stable_owner_id()))
                    .collect::<Vec<_>>(),
            );
            assert_eq!(ownership.parsed_claims().len(), 2);
            let suffix_len = if text == "Players'" { 1 } else { 2 };
            assert_eq!(ownership.parsed_claims()[0].span().start, 0);
            assert_eq!(
                ownership.parsed_claims()[0].span().end,
                text.len() - suffix_len,
            );
            assert_eq!(
                ownership.parsed_claims()[1].span().start,
                text.len() - suffix_len,
            );
            assert_eq!(ownership.parsed_claims()[1].span().end, text.len());

            let mut visitor = Recorder::default();
            crate::visit::walk_possessive(&mut visitor, selected);
            match text {
                "Daxos's" => assert_eq!(
                    visitor.self_references,
                    [crate::constructions::SelfReferenceSpelling::Abbreviated],
                ),
                "Players'" => {
                    assert_eq!(visitor.nouns, [crate::constructions::CommonNoun::Player]);
                }
                "Merfolk's" => assert_eq!(
                    visitor.declarations,
                    [(
                        deckmaste_construction_core::macro_def::DeclarationKind::Subtype(
                            deckmaste_construction_core::macro_def::SubtypeCategory::Creature,
                        ),
                        "Merfolk".to_owned(),
                    )],
                ),
                "Equipment's" => assert_eq!(
                    visitor.declarations,
                    [(
                        deckmaste_construction_core::macro_def::DeclarationKind::Subtype(
                            deckmaste_construction_core::macro_def::SubtypeCategory::Artifact,
                        ),
                        "Equipment".to_owned(),
                    )],
                ),
                _ => unreachable!(),
            }
        }
    }

    #[test]
    fn genitive_possessors_require_a_licensed_noun_phrase() {
        use super::ParseAnalysisOutcome;
        use super::SelectionResolution;

        let parser = Parser::new(environment()).expect("canonical environment satisfies grammar");
        let context = ParseContext::new(
            "Novel Context",
            false,
            deckmaste_construction_core::macro_def::Onset::Consonant,
        )
        .expect("valid context");

        for text in ["Destroy creature.", "Destroy creature's controller."] {
            assert_eq!(
                parser.analyze(text, &context).outcome(),
                ParseAnalysisOutcome::ParseFailure,
                "{text:?} must not license a bare singular count noun",
            );
        }

        for text in [
            "Destroy target creature's controller.",
            "Destroy that creature's owner.",
            "Destroy their owners' libraries.",
        ] {
            let analysis = parser.analyze(text, &context);
            assert_eq!(
                analysis.outcome(),
                ParseAnalysisOutcome::Selected,
                "{text:?}"
            );
            assert_eq!(
                analysis
                    .decision()
                    .expect("selected parse has a decision")
                    .resolution(),
                SelectionResolution::Unique,
                "{text:?}",
            );
        }

        for text in ["Destroy Merfolk's controller.", "Destroy control's owner."] {
            assert_eq!(
                parser.analyze(text, &context).outcome(),
                ParseAnalysisOutcome::Selected,
                "{text:?} keeps the proper/mass nominal exception",
            );
        }
    }

    #[test]
    fn ability_and_sentence_public_calls_each_use_one_pipeline() {
        let parser = Parser::new(environment()).expect("canonical environment satisfies grammar");
        let context = ParseContext::new(
            "Context Card",
            false,
            deckmaste_construction_core::macro_def::Onset::Consonant,
        )
        .unwrap();
        let text = "Destroy target creature.";

        assert_pipeline_once(|| {
            assert!(parser.analyze(text, &context).selected().is_some());
        });
        assert_pipeline_once(|| {
            assert!(parser.analyze_sentence(text, &context).selected().is_some());
        });
        assert_pipeline_once(|| {
            assert!(
                parser
                    .trace(text, &context, TraceLimits::new(0))
                    .into_parse_result()
                    .is_ok()
            );
        });
        assert_pipeline_once(|| {
            assert!(
                parser
                    .trace_sentence(text, &context, TraceLimits::new(0))
                    .into_parse_result()
                    .is_ok()
            );
        });
    }

    #[test]
    fn oracle_text_public_calls_each_use_one_pipeline() {
        use crate::constructions::OracleText;

        let parser = Parser::new(environment()).expect("canonical environment satisfies grammar");
        let context = ParseContext::new(
            "Context Card",
            false,
            deckmaste_construction_core::macro_def::Onset::Consonant,
        )
        .unwrap();
        let text = "Destroy target creature. You gain 2 life.";

        assert_pipeline_once(|| {
            assert!(parser.parse_oracle_text(text, &context).is_ok());
        });
        assert_pipeline_once(|| {
            let analysis: super::ParseAnalysis<OracleText> =
                parser.analyze_oracle_text(text, &context);
            assert!(analysis.selected().is_some());
        });
        assert_pipeline_once(|| {
            let trace: super::ParserTrace<OracleText> =
                parser.trace_oracle_text(text, &context, TraceLimits::new(0));
            assert!(trace.into_parse_result().is_ok());
        });
    }

    #[test]
    fn oracle_text_internal_failure_names_its_generated_root() {
        let parser = Parser::new(environment()).expect("canonical environment satisfies grammar");
        let context = ParseContext::new(
            "Context Card",
            false,
            deckmaste_construction_core::macro_def::Onset::Consonant,
        )
        .unwrap();
        super::ownership::force_inspection_corruption(true);

        let trace = parser.trace_oracle_text(
            "Destroy target creature. You gain 2 life.",
            &context,
            TraceLimits::new(usize::MAX),
        );

        super::ownership::force_inspection_corruption(false);
        assert_eq!(trace.root_name(), "OracleText");
        let super::BoundedParseOutcome::InternalFailure(failure) = trace.outcome() else {
            panic!("forced ownership corruption is an internal failure");
        };
        assert!(failure.message().contains("OracleText"));
    }

    #[test]
    fn generated_sentence_root_rejects_an_ability_build_value() {
        use crate::constructions::BuildValue;
        use crate::constructions::GeneratedRoot;
        use crate::constructions::Sentence;

        let parser = Parser::new(environment()).expect("canonical environment satisfies grammar");
        let context = ParseContext::new(
            "Context Card",
            false,
            deckmaste_construction_core::macro_def::Onset::Consonant,
        )
        .unwrap();
        let ability = parser
            .parse("Destroy target creature.", &context)
            .expect("fixture parses as an Ability");

        assert_eq!(
            Sentence::from_build(BuildValue::Ability(
                ability,
                crate::constructions::FeatureConstraint::Any,
            )),
            None
        );
    }

    #[test]
    fn sentence_internal_failure_names_its_generated_root() {
        let parser = Parser::new(environment()).expect("canonical environment satisfies grammar");
        let context = ParseContext::new(
            "Context Card",
            false,
            deckmaste_construction_core::macro_def::Onset::Consonant,
        )
        .unwrap();
        super::ownership::force_inspection_corruption(true);

        let trace = parser.trace_sentence(
            "Destroy target creature.",
            &context,
            TraceLimits::new(usize::MAX),
        );

        super::ownership::force_inspection_corruption(false);
        assert_eq!(trace.root_name(), "Sentence");
        let super::BoundedParseOutcome::InternalFailure(failure) = trace.outcome() else {
            panic!("forced ownership corruption is an internal failure");
        };
        assert!(failure.message().contains("Sentence"));
    }

    #[test]
    fn selected_ownership_pipeline_runs_each_semantic_pass_once() {
        let parser = Parser::new(environment()).expect("canonical environment satisfies grammar");
        let context = ParseContext::new(
            "Context Card",
            false,
            deckmaste_construction_core::macro_def::Onset::Consonant,
        )
        .unwrap();
        PIPELINE_COUNTS.with(|counts| counts.set([0; 5]));
        super::materialize::set_materialized_candidate_copies(3);
        super::materialize::reset_specificity_candidate_evaluations();

        let analysis = parser.analyze(
            "You gain X life, where X is the number of creatures you control with power 2 or less.",
            &context,
        );
        super::materialize::set_materialized_candidate_copies(1);

        assert!(analysis.ownership().is_some());
        PIPELINE_COUNTS.with(|counts| {
            assert_eq!(
                counts.get(),
                [1, 1, 1, 1, 1],
                "parse, materialize, specificity, selection, and ownership each run once"
            );
        });
        assert_eq!(super::materialize::finalized_candidate_count(), 3);
        assert_eq!(
            super::materialize::specificity_candidate_evaluations(),
            super::materialize::finalized_candidate_count(),
        );
    }

    #[test]
    fn parser_trace_lexical_ownership_projection_is_lazy_at_zero_and_one() {
        let parser = Parser::new(environment()).expect("canonical environment satisfies grammar");
        let context = ParseContext::new(
            "Context Card",
            false,
            deckmaste_construction_core::macro_def::Onset::Consonant,
        )
        .unwrap();
        for (limit, expected_constructions) in [(0, 0), (1, 1)] {
            super::diagnostic::reset_selected_claim_projection_constructions();

            let trace = parser.trace("Destroy target Spirit.", &context, TraceLimits::new(limit));

            assert!(trace.ownership().is_some());
            assert!(trace.selected_lexical_claims().total() > 1);
            assert_eq!(
                super::diagnostic::selected_claim_projection_constructions(),
                expected_constructions,
                "limit {limit}",
            );
        }
    }

    #[test]
    fn parser_trace_lexical_ownership_limits_keep_complete_selected_facts() {
        fn bounded<T>(value: &super::Bounded<T>, limit: usize) {
            assert_eq!(value.total(), value.shown() + value.omitted());
            assert_eq!(value.shown(), value.items().len());
            assert!(value.shown() <= limit);
        }

        fn scanner_match_type_is_public(_: &super::ScannerMatch) {}

        let parser = Parser::new(environment()).expect("canonical environment satisfies grammar");
        let context = ParseContext::new(
            "Context Card",
            false,
            deckmaste_construction_core::macro_def::Onset::Consonant,
        )
        .unwrap();
        let text = "Destroy target Spirit.";
        let complete = parser.analyze(text, &context);
        let complete = complete.ownership().expect("selected ownership");
        let expected_claims = complete.parsed_claims().to_vec();
        let expected_summary = complete.summary().clone();
        let expected_failures = complete.failures().to_vec();
        let total = expected_claims.len();

        for limit in [0, 1, total, total + 1] {
            let trace = parser.trace(text, &context, TraceLimits::new(limit));
            bounded(trace.scanner_matches(), limit);
            bounded(trace.selected_lexical_claims(), limit);
            assert_eq!(trace.selected_lexical_claims().total(), total);
            assert_eq!(
                trace.selected_lexical_claims().items(),
                &expected_claims[..limit.min(total)],
            );
            assert_eq!(trace.ownership(), Some(&expected_summary));
            assert_eq!(trace.ownership_failures(), expected_failures);
            if let Some(scanner_match) = trace.scanner_matches().items().first() {
                scanner_match_type_is_public(scanner_match);
            }
        }
    }

    #[test]
    fn parser_trace_lexical_ownership_failures_are_complete_and_cap_invariant() {
        fn assert_complete_summary(summary: &super::OwnershipSummary) {
            assert!(!summary.covered());
            assert_eq!(summary.claims(), 4);
            assert_eq!(summary.claimed_bytes(), 22);
            assert_eq!(summary.form_literal_claims(), 1);
            assert_eq!(summary.form_literal_bytes(), 1);
            assert_eq!(summary.vocab_claims(), 1);
            assert_eq!(summary.vocab_bytes(), 7);
            assert_eq!(summary.lexeme_claims(), 2);
            assert_eq!(summary.lexeme_bytes(), 14);
            assert_eq!(summary.codec_claims(), 0);
            assert_eq!(summary.codec_bytes(), 0);
            assert_eq!(summary.identity_claims(), 0);
            assert_eq!(summary.identity_bytes(), 0);
            assert_eq!(summary.gap_spans(), 0);
            assert_eq!(summary.gap_bytes(), 0);
            assert_eq!(summary.overlap_spans(), 0);
            assert_eq!(summary.overlap_bytes(), 0);
            assert_eq!(summary.synthetic_claims(), 1);
            assert_eq!(summary.provenance_plan_mismatches(), 0);
        }

        let parser = Parser::new(environment()).expect("canonical environment satisfies grammar");
        let context = ParseContext::new(
            "Context Card",
            false,
            deckmaste_construction_core::macro_def::Onset::Consonant,
        )
        .unwrap();
        let text = "Destroy target Spirit.";
        let expected_failures = vec![super::OwnershipFailure::Synthetic {
            span: super::TextSpan {
                start: text.len(),
                end: text.len(),
            },
        }];
        let complete = parser.trace_with_ownership_failure_for_test(
            text,
            &context,
            TraceLimits::new(usize::MAX),
        );
        let expected_summary = complete.ownership().expect("selected ownership").clone();
        assert_complete_summary(&expected_summary);

        for limit in [0, 1, 4, 5] {
            let trace = parser.trace_with_ownership_failure_for_test(
                text,
                &context,
                TraceLimits::new(limit),
            );
            assert_eq!(trace.ownership(), Some(&expected_summary));
            assert_complete_summary(trace.ownership().expect("selected ownership"));
            assert_eq!(trace.ownership_failures(), expected_failures);
            assert!(!trace.ownership_failures().is_empty());
        }
    }

    #[test]
    fn parser_trace_lexical_ownership_keeps_matches_from_losing_derivations() {
        let parser = Parser::new(environment()).expect("canonical environment satisfies grammar");
        let context = ParseContext::new(
            "Context Card",
            false,
            deckmaste_construction_core::macro_def::Onset::Consonant,
        )
        .unwrap();

        let trace = parser.trace(
            "Destroy target Spirit",
            &context,
            TraceLimits::new(usize::MAX),
        );

        assert!(trace.clone().into_parse_result().is_err());
        let semantic_matches = trace
            .scanner_matches()
            .items()
            .iter()
            .map(|matched| (matched.start(), matched.end(), matched.value_label_v1()))
            .collect::<std::collections::BTreeSet<_>>();
        assert_eq!(
            semantic_matches,
            std::collections::BTreeSet::from([
                (
                    0,
                    7,
                    "TransitiveVerb { verb: DeclarationTransitiveVerb { reference: Declaration(DeclarationIdentity { kind: KeywordAction, name: \"Destroy\" }) }, concord_class: Other, inflectional_form: Plain, onset: Consonant }",
                ),
                (7, 14, "TargetingMarker(Target)"),
                (
                    7,
                    14,
                    "Noun { noun: Target, number: Singular, onset: Consonant, possessive_ending: Other }",
                ),
                (
                    14,
                    21,
                    "Noun { noun: Declaration(DeclarationNoun { id: DeclarationIdentity { kind: Subtype(Creature), name: \"Spirit\" }, locative_temporal_license: ObjectAttachmentLicensed, relationality: QualifiedRelational, number_invariant: false }), number: Singular, onset: Consonant, possessive_ending: Other }",
                ),
            ])
        );
        assert_eq!(trace.selected_lexical_claims().total(), 0);
        assert_eq!(trace.ownership(), None);
        assert!(trace.ownership_failures().is_empty());
    }

    #[test]
    fn generated_owner_keeps_the_scanner_hot_carrier_compact() {
        assert_eq!(
            std::mem::size_of::<crate::constructions::LexicalOwner>(),
            24,
            "the generated owner size is part of the scanner hot-path contract",
        );
        assert_eq!(
            std::mem::size_of::<(
                crate::constructions::Leaf,
                Option<crate::constructions::LexicalOwner>,
            )>(),
            72,
            "the production scanner value/owner carrier size is part of the hot-path contract",
        );
    }

    #[test]
    fn impossible_selected_ownership_corruption_maps_to_internal_failure() {
        let parser = Parser::new(environment()).expect("canonical environment satisfies grammar");
        let context = ParseContext::new(
            "Context Card",
            false,
            deckmaste_construction_core::macro_def::Onset::Consonant,
        )
        .unwrap();
        super::ownership::force_inspection_corruption(true);

        let analysis = parser.analyze("Destroy target creature.", &context);

        super::ownership::force_inspection_corruption(false);
        assert_eq!(
            analysis.outcome(),
            super::ParseAnalysisOutcome::InternalFailure(
                super::InternalFailureKind::OwnershipInspection,
            )
        );
        assert!(analysis.ownership().is_none());
    }

    #[test]
    fn cloned_parser_shares_frozen_environment_storage() {
        let parser = Parser::new(environment()).expect("canonical environment satisfies grammar");
        let cloned = parser.clone();

        assert!(
            parser
                .environment
                .test_only_shares_storage_with(&cloned.environment)
        );
    }

    #[test]
    fn structural_trace_observation_is_repeatable_and_inert() {
        let parser = Parser::new(environment()).expect("canonical environment satisfies grammar");
        let context = ParseContext::new(
            "Trace Card",
            false,
            deckmaste_construction_core::macro_def::Onset::Consonant,
        )
        .expect("valid context");
        let text = "Whenever a player connives, you gain X life.";
        let (analysis, first) = parser.observe_structural(text, &context, TraceLimits::new(1));
        let (_, second) = parser.observe_structural(text, &context, TraceLimits::new(1));
        assert_eq!(parser.parse(text, &context), analysis.into_parse_result());
        assert_eq!(first, second);
        assert!(first.scanner_matches().total() > 0);
        assert_eq!(first.scanner_matches().shown(), 1);
        let first_match = &first.scanner_matches().items()[0];
        assert_eq!((first_match.start(), first_match.end()), (0, 8));
        assert_eq!(first_match.value_label_v1(), "TriggerMarker(Whenever)");
    }

    #[test]
    fn public_trace_rejects_a_finite_clause_in_the_typed_where_slot_before_materialization() {
        let parser = Parser::new(environment()).expect("canonical environment satisfies grammar");
        let context = ParseContext::new(
            "Trace Card",
            false,
            deckmaste_construction_core::macro_def::Onset::Consonant,
        )
        .expect("valid context");
        let text = "You gain X life, a player connives.";
        let (analysis, trace) = parser.analyze_root_with_trace::<crate::ast::Sentence>(
            text,
            &context,
            TraceLimits::new(usize::MAX),
        );
        assert_eq!(
            analysis.outcome(),
            super::ParseAnalysisOutcome::ParseFailure
        );
        assert!(analysis.build_rejection().is_none());
        let _rejection = trace
            .structural
            .first_build_rejection()
            .expect("the partial chart retains its first checked-completion rejection");
        assert!(trace.materialization.first_build_rejection().is_none());
        assert_eq!(
            parser
                .trace_sentence(text, &context, TraceLimits::new(usize::MAX))
                .build_rejection(),
            None,
        );
    }

    #[test]
    fn synthetic_structural_rejection_is_raw_only_for_a_selected_typed_where_clause() {
        let environment = environment();
        let parser =
            Parser::new(environment.clone()).expect("canonical environment satisfies grammar");
        let context = ParseContext::new(
            "Trace Card",
            false,
            deckmaste_construction_core::macro_def::Onset::Consonant,
        )
        .expect("valid context");
        let text =
            "You gain X life, where X is the number of creatures you control with power 2 or less.";
        let scan_first = crate::constructions::BuildRejection::new(
            "BInjectedCompletion",
            "injected",
            crate::constructions::BuildViolation::Invariant {
                identity: "injected completion is rejected",
            },
        );
        super::scan::with_checked_completion_rejection_for_test(scan_first, || {
            let grammar = super::SliceGrammar {
                environment: &environment,
                context: &context,
            };
            let (forest, raw_structural) = super::parse_forest_observed::<crate::ast::Sentence>(
                &grammar,
                text,
                TraceLimits::new(usize::MAX),
            );
            assert!(forest.is_ok());
            assert_eq!(raw_structural.first_build_rejection(), Some(&scan_first));

            let (analysis, trace) = parser.analyze_root_with_trace::<crate::ast::Sentence>(
                text,
                &context,
                TraceLimits::new(usize::MAX),
            );
            assert_eq!(analysis.outcome(), super::ParseAnalysisOutcome::Selected);
            assert!(analysis.build_rejection().is_none());
            assert!(trace.structural.first_build_rejection().is_none());
            assert!(trace.materialization.first_build_rejection().is_none());
        });
    }

    #[test]
    fn structural_trace_success_collections_and_nested_caps_are_exact() {
        fn bounded<T>(value: &super::Bounded<T>, limit: usize) {
            assert_eq!(value.total(), value.shown() + value.omitted());
            assert_eq!(value.shown(), value.items().len());
            assert!(value.shown() <= limit);
        }
        let parser = Parser::new(environment()).expect("canonical environment satisfies grammar");
        let context = ParseContext::new(
            "Trace Card",
            false,
            deckmaste_construction_core::macro_def::Onset::Consonant,
        )
        .expect("valid context");
        let text = "Whenever a player connives, you gain X life.";
        for limit in [0, 1, usize::MAX] {
            let (_, trace) = parser.observe_structural(text, &context, TraceLimits::new(limit));
            bounded(trace.scanner_matches(), limit);
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
        let parser = Parser::new(environment()).expect("canonical environment satisfies grammar");
        let context = ParseContext::new(
            "Trace Card",
            false,
            deckmaste_construction_core::macro_def::Onset::Consonant,
        )
        .expect("context");
        let text = "Whenever a player connives, you gain X life";
        for limit in [0, 1, usize::MAX] {
            let (analysis, trace) =
                parser.observe_structural(text, &context, TraceLimits::new(limit));
            assert!(analysis.selected().is_none());
            assert!(analysis.ownership().is_none());
            assert_eq!(parser.parse(text, &context), analysis.into_parse_result());
            bounded(trace.scanner_matches(), limit);
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
            assert!(trace.forest().total() > 0);
            let (_, repeated) = parser.observe_structural(text, &context, TraceLimits::new(limit));
            assert_eq!(trace, repeated);
        }
    }

    #[test]
    fn structural_trace_observed_engine_matches_noop_success_and_failure() {
        let environment = environment();
        let parser =
            Parser::new(environment.clone()).expect("canonical environment satisfies grammar");
        let context = ParseContext::new(
            "Trace Card",
            false,
            deckmaste_construction_core::macro_def::Onset::Consonant,
        )
        .expect("context");
        for text in [
            "Whenever a player connives, you gain X life.",
            "Whenever a player connives, you gain X life",
        ] {
            let grammar = super::SliceGrammar {
                environment: &environment,
                context: &context,
            };
            let ordinary = super::parse_forest::<crate::ast::Ability>(&grammar, text);
            let (observed, trace) = super::parse_forest_observed::<crate::ast::Ability>(
                &grammar,
                text,
                TraceLimits::new(usize::MAX),
            );
            assert_eq!(ordinary, observed);
            let (analysis, _) =
                parser.observe_structural(text, &context, TraceLimits::new(usize::MAX));
            if text.ends_with("life") {
                assert!(analysis.selected().is_none());
                assert!(analysis.ownership().is_none());
            }
            assert_eq!(parser.parse(text, &context), analysis.into_parse_result());
            if text.ends_with("life") {
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
        RulePosition::Nonterminal(category) | RulePosition::AdjacentNonterminal(category) => {
            Expectation::Nonterminal(category.nonterminal_category())
        }
        RulePosition::Lexical(Lexical::Literal(literal)) => Expectation::Literal(literal),
        RulePosition::Lexical(lexical) => Expectation::Terminal(terminal_class(lexical)),
    }
}

const fn terminal_class(lexical: Lexical) -> TerminalClass {
    lexical.class()
}
