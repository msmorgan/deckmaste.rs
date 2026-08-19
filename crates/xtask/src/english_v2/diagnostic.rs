use std::cmp::Ordering;
use std::io::Write;

use anyhow::Context;
use deckmaste_english_v2::parser::Bounded;
use deckmaste_english_v2::parser::BoundedParseOutcome;
use deckmaste_english_v2::parser::BoundedSelectionCandidate as RuntimeSelectionCandidate;
use deckmaste_english_v2::parser::BoundedSelectionComparison as RuntimeSelectionComparison;
use deckmaste_english_v2::parser::BoundedSelectionDecision as RuntimeSelectionDecision;
use deckmaste_english_v2::parser::ChartItem as RuntimeChartItem;
use deckmaste_english_v2::parser::CheckedCompletionRejection as RuntimeCheckedRejection;
use deckmaste_english_v2::parser::ExpectationInfo as RuntimeExpectation;
use deckmaste_english_v2::parser::FamilyIdentityChild as RuntimeFamilyIdentityChild;
use deckmaste_english_v2::parser::ForestChild as RuntimeForestChild;
use deckmaste_english_v2::parser::ForestFamily as RuntimeForestFamily;
use deckmaste_english_v2::parser::ForestNode as RuntimeForestNode;
use deckmaste_english_v2::parser::InternalFailureKind as RuntimeInternalFailureKind;
use deckmaste_english_v2::parser::MaterializationCycle as RuntimeMaterializationCycle;
use deckmaste_english_v2::parser::MaterializedCandidateInfo as RuntimeMaterializedCandidate;
use deckmaste_english_v2::parser::NonterminalCategory;
use deckmaste_english_v2::parser::ParserTrace;
use deckmaste_english_v2::parser::ScannedToken as RuntimeScannedToken;
use deckmaste_english_v2::parser::SelectionDecisive as RuntimeSelectionDecisive;
use deckmaste_english_v2::parser::SelectionResolution as RuntimeSelectionResolution;
use deckmaste_english_v2::parser::SpecificityTier as RuntimeSpecificityTier;
use deckmaste_english_v2::parser::TerminalClass;
use deckmaste_english_v2::parser::UnselectedCandidate as RuntimeUnselectedCandidate;
use serde::Deserialize;
use serde::Serialize;

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub(super) struct DiagnosticReport {
    schema_version: u32,
    source: DiagnosticSource,
    trace: DiagnosticTrace,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
enum DiagnosticSource {
    Probe {
        text: String,
        context: String,
    },
    Corpus {
        id: String,
        card: String,
        face: Option<String>,
        side: Option<String>,
        text: String,
        context: String,
    },
}

#[cfg(test)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum SourceKind {
    Probe,
    Corpus,
}

#[cfg(test)]
impl DiagnosticSource {
    const fn kind(&self) -> SourceKind {
        match self {
            Self::Probe { .. } => SourceKind::Probe,
            Self::Corpus { .. } => SourceKind::Corpus,
        }
    }

    fn text(&self) -> &str {
        match self {
            Self::Probe { text, .. } | Self::Corpus { text, .. } => text,
        }
    }

    fn context(&self) -> &str {
        match self {
            Self::Probe { context, .. } | Self::Corpus { context, .. } => context,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub(super) struct DiagnosticTrace {
    tokens: Counted<ScannedToken>,
    final_chart: Counted<ChartItem>,
    forest_nodes: Counted<ForestNode>,
    forest_roots: Counted<usize>,
    checked_completion_rejections: Counted<CheckedCompletionRejection>,
    materialized_candidates: Counted<MaterializedCandidate>,
    materialization_cycles: Counted<MaterializationCycle>,
    outcome: DiagnosticOutcome,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
struct Counted<T> {
    total: usize,
    shown: usize,
    omitted: usize,
    items: Vec<T>,
}

#[derive(Debug)]
struct BoundedSource<'a, T> {
    total: usize,
    shown: usize,
    omitted: usize,
    items: &'a [T],
}

impl<T> Clone for BoundedSource<'_, T> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<T> Copy for BoundedSource<'_, T> {}

fn runtime_bounded<T>(bounded: &Bounded<T>) -> BoundedSource<'_, T> {
    BoundedSource {
        total: bounded.total(),
        shown: bounded.shown(),
        omitted: bounded.omitted(),
        items: bounded.items(),
    }
}

trait TokenSource {
    fn start(&self) -> usize;
    fn end(&self) -> usize;
    fn terminal_name_v1(&self) -> &str;
    fn value_label_v1(&self) -> &str;
}

trait ChartItemSource {
    fn column(&self) -> usize;
    fn rule_name_v1(&self) -> &str;
    fn dot(&self) -> usize;
    fn origin(&self) -> usize;
    fn family_count(&self) -> usize;
}

trait ForestChildSource {
    fn node_id(&self) -> Option<usize>;
    fn value_label_v1(&self) -> Option<&str>;
}

trait ForestFamilySource {
    type Child: ForestChildSource;

    fn children(&self) -> BoundedSource<'_, Self::Child>;
}

trait ForestNodeSource {
    type Family: ForestFamilySource;

    fn id(&self) -> usize;
    fn rule_name_v1(&self) -> &str;
    fn start(&self) -> usize;
    fn end(&self) -> usize;
    fn families(&self) -> BoundedSource<'_, Self::Family>;
}

trait CheckedRejectionSource {
    fn rule_name_v1(&self) -> &str;
    fn start(&self) -> usize;
    fn end(&self) -> usize;
    fn family_identity_children(&self) -> &[RuntimeFamilyIdentityChild];
}

trait MaterializedCandidateSource {
    fn ordinal(&self) -> usize;
    fn rendered(&self) -> &str;
    fn ast_debug_v1(&self) -> &str;
    fn construction_path(&self) -> BoundedSource<'_, String>;
    fn specificity(&self) -> BoundedSource<'_, RuntimeSpecificityTier>;
}

trait MaterializationCycleSource {
    fn node_ordinal(&self) -> usize;
    fn construction_path(&self) -> BoundedSource<'_, String>;
}

trait SelectionCandidateSource {
    fn ordinal(&self) -> usize;
    fn construction_path(&self) -> BoundedSource<'_, String>;
    fn specificity(&self) -> BoundedSource<'_, RuntimeSpecificityTier>;
}

trait SelectionComparisonSource {
    fn left_ordinal(&self) -> usize;
    fn right_ordinal(&self) -> usize;
    fn ordering(&self) -> Ordering;
    fn decisive(&self) -> RuntimeSelectionDecisive;
    fn exception_id(&self) -> Option<&str>;
}

trait UnselectedCandidateSource {
    type Candidate: SelectionCandidateSource;
    type Comparison: SelectionComparisonSource;

    fn candidate(&self) -> &Self::Candidate;
    fn reason(&self) -> deckmaste_english_v2::parser::SelectionLoserReason;
    fn evidence(&self) -> BoundedSource<'_, Self::Comparison>;
}

trait SelectionDecisionSource {
    type Candidate: SelectionCandidateSource;
    type Comparison: SelectionComparisonSource;
    type Unselected: UnselectedCandidateSource<Candidate = Self::Candidate, Comparison = Self::Comparison>;

    fn candidates(&self) -> BoundedSource<'_, Self::Candidate>;
    fn comparisons(&self) -> BoundedSource<'_, Self::Comparison>;
    fn survivors(&self) -> BoundedSource<'_, usize>;
    fn selected(&self) -> Option<&Self::Candidate>;
    fn resolution(&self) -> RuntimeSelectionResolution;
    fn exception_uses(&self) -> BoundedSource<'_, String>;
    fn unselected_candidates(&self) -> BoundedSource<'_, Self::Unselected>;
}

enum TraceOutcomeSource<'a, Selection> {
    Selected {
        rendered: &'a str,
        selection: &'a Selection,
    },
    ParseFailure {
        start: usize,
        end: usize,
        expectations: BoundedSource<'a, RuntimeExpectation>,
    },
    UnresolvedAmbiguity {
        selection: &'a Selection,
    },
    InternalFailure {
        kind: RuntimeInternalFailureKind,
        message: &'a str,
    },
}

impl<Selection> Clone for TraceOutcomeSource<'_, Selection> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<Selection> Copy for TraceOutcomeSource<'_, Selection> {}

trait TraceSourceView {
    type Token: TokenSource;
    type ChartItem: ChartItemSource;
    type ForestNode: ForestNodeSource;
    type CheckedRejection: CheckedRejectionSource;
    type MaterializedCandidate: MaterializedCandidateSource;
    type MaterializationCycle: MaterializationCycleSource;
    type Selection: SelectionDecisionSource;

    fn tokens(&self) -> BoundedSource<'_, Self::Token>;
    fn chart(&self) -> BoundedSource<'_, Self::ChartItem>;
    fn forest(&self) -> BoundedSource<'_, Self::ForestNode>;
    fn accepted_roots(&self) -> BoundedSource<'_, usize>;
    fn checked_completion_rejections(&self) -> BoundedSource<'_, Self::CheckedRejection>;
    fn materialized_candidates(&self) -> BoundedSource<'_, Self::MaterializedCandidate>;
    fn materialization_cycles(&self) -> BoundedSource<'_, Self::MaterializationCycle>;
    fn outcome(&self) -> TraceOutcomeSource<'_, Self::Selection>;
}

impl<T> Counted<T> {
    fn from_source<U>(source: BoundedSource<'_, U>, project: impl Fn(&U) -> T) -> Self {
        Self {
            total: source.total,
            shown: source.shown,
            omitted: source.omitted,
            items: source.items.iter().map(project).collect(),
        }
    }

    #[cfg(test)]
    fn fixture(total: usize, items: Vec<T>) -> Self {
        let shown = items.len();
        Self {
            total,
            shown,
            omitted: total - shown,
            items,
        }
    }
}

impl TokenSource for RuntimeScannedToken {
    fn start(&self) -> usize {
        RuntimeScannedToken::start(self)
    }

    fn end(&self) -> usize {
        RuntimeScannedToken::end(self)
    }

    fn terminal_name_v1(&self) -> &str {
        RuntimeScannedToken::terminal_name_v1(self)
    }

    fn value_label_v1(&self) -> &str {
        RuntimeScannedToken::value_label_v1(self)
    }
}

impl ChartItemSource for RuntimeChartItem {
    fn column(&self) -> usize {
        RuntimeChartItem::column(self)
    }

    fn rule_name_v1(&self) -> &str {
        RuntimeChartItem::rule_name_v1(self)
    }

    fn dot(&self) -> usize {
        RuntimeChartItem::dot(self)
    }

    fn origin(&self) -> usize {
        RuntimeChartItem::origin(self)
    }

    fn family_count(&self) -> usize {
        RuntimeChartItem::family_count(self)
    }
}

impl ForestChildSource for RuntimeForestChild {
    fn node_id(&self) -> Option<usize> {
        RuntimeForestChild::node_id(self)
    }

    fn value_label_v1(&self) -> Option<&str> {
        RuntimeForestChild::value_label_v1(self)
    }
}

impl ForestFamilySource for RuntimeForestFamily {
    type Child = RuntimeForestChild;

    fn children(&self) -> BoundedSource<'_, Self::Child> {
        runtime_bounded(RuntimeForestFamily::children(self))
    }
}

impl ForestNodeSource for RuntimeForestNode {
    type Family = RuntimeForestFamily;

    fn id(&self) -> usize {
        RuntimeForestNode::id(self)
    }

    fn rule_name_v1(&self) -> &str {
        RuntimeForestNode::rule_name_v1(self)
    }

    fn start(&self) -> usize {
        RuntimeForestNode::start(self)
    }

    fn end(&self) -> usize {
        RuntimeForestNode::end(self)
    }

    fn families(&self) -> BoundedSource<'_, Self::Family> {
        runtime_bounded(RuntimeForestNode::families(self))
    }
}

impl CheckedRejectionSource for RuntimeCheckedRejection {
    fn rule_name_v1(&self) -> &str {
        RuntimeCheckedRejection::rule_name_v1(self)
    }

    fn start(&self) -> usize {
        RuntimeCheckedRejection::start(self)
    }

    fn end(&self) -> usize {
        RuntimeCheckedRejection::end(self)
    }

    fn family_identity_children(&self) -> &[RuntimeFamilyIdentityChild] {
        RuntimeCheckedRejection::family_identity_v1(self).children()
    }
}

impl MaterializedCandidateSource for RuntimeMaterializedCandidate {
    fn ordinal(&self) -> usize {
        RuntimeMaterializedCandidate::ordinal(self)
    }

    fn rendered(&self) -> &str {
        RuntimeMaterializedCandidate::rendered(self)
    }

    fn ast_debug_v1(&self) -> &str {
        RuntimeMaterializedCandidate::ast_debug_v1(self)
    }

    fn construction_path(&self) -> BoundedSource<'_, String> {
        runtime_bounded(RuntimeMaterializedCandidate::construction_path(self))
    }

    fn specificity(&self) -> BoundedSource<'_, RuntimeSpecificityTier> {
        runtime_bounded(RuntimeMaterializedCandidate::specificity(self))
    }
}

impl MaterializationCycleSource for RuntimeMaterializationCycle {
    fn node_ordinal(&self) -> usize {
        RuntimeMaterializationCycle::node_ordinal(self)
    }

    fn construction_path(&self) -> BoundedSource<'_, String> {
        runtime_bounded(RuntimeMaterializationCycle::construction_path(self))
    }
}

impl SelectionCandidateSource for RuntimeSelectionCandidate {
    fn ordinal(&self) -> usize {
        RuntimeSelectionCandidate::ordinal(self)
    }

    fn construction_path(&self) -> BoundedSource<'_, String> {
        runtime_bounded(RuntimeSelectionCandidate::construction_path(self))
    }

    fn specificity(&self) -> BoundedSource<'_, RuntimeSpecificityTier> {
        runtime_bounded(RuntimeSelectionCandidate::specificity(self))
    }
}

impl SelectionComparisonSource for RuntimeSelectionComparison {
    fn left_ordinal(&self) -> usize {
        RuntimeSelectionComparison::left_ordinal(self)
    }

    fn right_ordinal(&self) -> usize {
        RuntimeSelectionComparison::right_ordinal(self)
    }

    fn ordering(&self) -> Ordering {
        RuntimeSelectionComparison::ordering(self)
    }

    fn decisive(&self) -> RuntimeSelectionDecisive {
        RuntimeSelectionComparison::decisive(self)
    }

    fn exception_id(&self) -> Option<&str> {
        RuntimeSelectionComparison::exception_id(self)
    }
}

impl UnselectedCandidateSource for RuntimeUnselectedCandidate {
    type Candidate = RuntimeSelectionCandidate;
    type Comparison = RuntimeSelectionComparison;

    fn candidate(&self) -> &Self::Candidate {
        RuntimeUnselectedCandidate::candidate(self)
    }

    fn reason(&self) -> deckmaste_english_v2::parser::SelectionLoserReason {
        RuntimeUnselectedCandidate::reason(self)
    }

    fn evidence(&self) -> BoundedSource<'_, Self::Comparison> {
        runtime_bounded(RuntimeUnselectedCandidate::evidence(self))
    }
}

impl SelectionDecisionSource for RuntimeSelectionDecision {
    type Candidate = RuntimeSelectionCandidate;
    type Comparison = RuntimeSelectionComparison;
    type Unselected = RuntimeUnselectedCandidate;

    fn candidates(&self) -> BoundedSource<'_, Self::Candidate> {
        runtime_bounded(RuntimeSelectionDecision::candidates(self))
    }

    fn comparisons(&self) -> BoundedSource<'_, Self::Comparison> {
        runtime_bounded(RuntimeSelectionDecision::comparisons(self))
    }

    fn survivors(&self) -> BoundedSource<'_, usize> {
        runtime_bounded(RuntimeSelectionDecision::survivors(self))
    }

    fn selected(&self) -> Option<&Self::Candidate> {
        RuntimeSelectionDecision::selected(self)
    }

    fn resolution(&self) -> RuntimeSelectionResolution {
        RuntimeSelectionDecision::resolution(self)
    }

    fn exception_uses(&self) -> BoundedSource<'_, String> {
        runtime_bounded(RuntimeSelectionDecision::exception_uses(self))
    }

    fn unselected_candidates(&self) -> BoundedSource<'_, Self::Unselected> {
        runtime_bounded(RuntimeSelectionDecision::unselected_candidates(self))
    }
}

impl TraceSourceView for ParserTrace {
    type Token = RuntimeScannedToken;
    type ChartItem = RuntimeChartItem;
    type ForestNode = RuntimeForestNode;
    type CheckedRejection = RuntimeCheckedRejection;
    type MaterializedCandidate = RuntimeMaterializedCandidate;
    type MaterializationCycle = RuntimeMaterializationCycle;
    type Selection = RuntimeSelectionDecision;

    fn tokens(&self) -> BoundedSource<'_, Self::Token> {
        runtime_bounded(ParserTrace::tokens(self))
    }

    fn chart(&self) -> BoundedSource<'_, Self::ChartItem> {
        runtime_bounded(ParserTrace::chart(self))
    }

    fn forest(&self) -> BoundedSource<'_, Self::ForestNode> {
        runtime_bounded(ParserTrace::forest(self))
    }

    fn accepted_roots(&self) -> BoundedSource<'_, usize> {
        runtime_bounded(ParserTrace::accepted_roots(self))
    }

    fn checked_completion_rejections(&self) -> BoundedSource<'_, Self::CheckedRejection> {
        runtime_bounded(ParserTrace::checked_completion_rejections(self))
    }

    fn materialized_candidates(&self) -> BoundedSource<'_, Self::MaterializedCandidate> {
        runtime_bounded(ParserTrace::materialized_candidates(self))
    }

    fn materialization_cycles(&self) -> BoundedSource<'_, Self::MaterializationCycle> {
        runtime_bounded(ParserTrace::materialization_cycles(self))
    }

    fn outcome(&self) -> TraceOutcomeSource<'_, Self::Selection> {
        match ParserTrace::outcome(self) {
            BoundedParseOutcome::Selected(selected) => TraceOutcomeSource::Selected {
                rendered: selected.rendered(),
                selection: selected.selection(),
            },
            BoundedParseOutcome::ParseFailure(failure) => TraceOutcomeSource::ParseFailure {
                start: failure.span().start,
                end: failure.span().end,
                expectations: runtime_bounded(failure.expectations()),
            },
            BoundedParseOutcome::UnresolvedAmbiguity(unresolved) => {
                TraceOutcomeSource::UnresolvedAmbiguity {
                    selection: unresolved.selection(),
                }
            }
            BoundedParseOutcome::InternalFailure(failure) => TraceOutcomeSource::InternalFailure {
                kind: failure.kind(),
                message: failure.message(),
            },
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
struct ScannedToken {
    start: usize,
    end: usize,
    terminal_name_v1: String,
    value_label_v1: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
struct ChartItem {
    column: usize,
    rule_name_v1: String,
    dot: usize,
    origin: usize,
    family_count: usize,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
struct ForestNode {
    id: usize,
    rule_name_v1: String,
    start: usize,
    end: usize,
    families: Counted<ForestFamily>,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
struct ForestFamily {
    children: Counted<ForestChild>,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
struct ForestChild {
    node_id: Option<usize>,
    value_label_v1: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
struct CheckedCompletionRejection {
    rule_name_v1: String,
    start: usize,
    end: usize,
    family_identity_v1: FamilyIdentity,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
struct FamilyIdentity {
    children: Vec<FamilyIdentityChild>,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(tag = "kind", content = "value", rename_all = "snake_case")]
enum FamilyIdentityChild {
    Node(usize),
    Lexical(String),
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
struct MaterializedCandidate {
    ordinal: usize,
    rendered: String,
    ast_debug_v1: String,
    construction_path: Counted<String>,
    specificity: Counted<SpecificityTier>,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
struct MaterializationCycle {
    node_ordinal: usize,
    construction_path: Counted<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
enum SpecificityTier {
    Nonterminal,
    TypedLexical,
    Literal,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(tag = "status", rename_all = "snake_case")]
enum DiagnosticOutcome {
    Selected(SelectedOutcome),
    ParseFailure(ParseFailureOutcome),
    UnresolvedAmbiguity(UnresolvedOutcome),
    InternalFailure(InternalFailureOutcome),
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
struct SelectedOutcome {
    rendered: String,
    selection: SelectionDecision,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
struct ParseFailureOutcome {
    span: TextSpan,
    expectations: Counted<Expectation>,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
struct TextSpan {
    start: usize,
    end: usize,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(tag = "kind", content = "value", rename_all = "snake_case")]
enum Expectation {
    Nonterminal(NonterminalKind),
    Terminal(TerminalKind),
    Literal(String),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
enum NonterminalKind {
    Ability,
    Sentence,
    Clause,
    NounPhrase,
    VerbPhrase,
    Amount,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
enum TerminalKind {
    Article,
    Declaration,
    Demonstrative,
    EndOfInput,
    Noun,
    Pronoun,
    SelfReference,
    SignedNumber,
    TriggerWord,
    Variable,
    VerbLexeme,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
struct UnresolvedOutcome {
    selection: SelectionDecision,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
struct InternalFailureOutcome {
    kind: InternalFailureKind,
    message: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub(super) enum InternalFailureKind {
    ValidatedRootDidNotMaterialize,
    SelectionConfiguration,
    OwnershipInspection,
}

impl InternalFailureKind {
    pub(super) const fn as_str(self) -> &'static str {
        match self {
            Self::ValidatedRootDidNotMaterialize => "validated_root_did_not_materialize",
            Self::SelectionConfiguration => "selection_configuration",
            Self::OwnershipInspection => "ownership_inspection",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
struct SelectionDecision {
    candidates: Counted<SelectionCandidate>,
    comparisons: Counted<SelectionComparison>,
    survivors: Counted<usize>,
    exception_uses: Counted<String>,
    selected: Option<SelectionCandidate>,
    resolution: SelectionResolution,
    unselected_candidates: Counted<UnselectedCandidate>,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
struct SelectionCandidate {
    ordinal: usize,
    construction_path: Counted<String>,
    specificity: Counted<SpecificityTier>,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
struct SelectionComparison {
    left_ordinal: usize,
    right_ordinal: usize,
    ordering: OrderingKind,
    decisive: SelectionDecisive,
    exception_id: Option<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
enum OrderingKind {
    Less,
    Equal,
    Greater,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(tag = "kind", content = "index", rename_all = "snake_case")]
enum SelectionDecisive {
    Position(usize),
    VectorExhaustion(usize),
    Tie,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
enum SelectionResolution {
    Unique,
    Specificity,
    Exception,
    UnresolvedTie,
}

impl SelectionResolution {
    const fn as_str(self) -> &'static str {
        match self {
            Self::Unique => "unique",
            Self::Specificity => "specificity",
            Self::Exception => "exception",
            Self::UnresolvedTie => "unresolved_tie",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
struct UnselectedCandidate {
    candidate: SelectionCandidate,
    reason: SelectionLoserReason,
    evidence: Counted<SelectionComparison>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
enum SelectionLoserReason {
    LessSpecific,
    ExceptionLoser,
    UnresolvedSurvivor,
}

impl DiagnosticReport {
    pub(super) fn from_probe(text: &str, context: &str, trace: &ParserTrace) -> Self {
        Self::from_probe_source(text, context, trace)
    }

    fn from_probe_source<Source: TraceSourceView>(
        text: &str,
        context: &str,
        trace: &Source,
    ) -> Self {
        Self {
            schema_version: 1,
            source: DiagnosticSource::Probe {
                text: text.to_owned(),
                context: context.to_owned(),
            },
            trace: DiagnosticTrace::from_source(trace),
        }
    }

    pub(super) fn from_corpus(unit: &super::corpus::CorpusUnit, trace: &ParserTrace) -> Self {
        Self {
            schema_version: 1,
            source: corpus_source(unit),
            trace: DiagnosticTrace::from_source(trace),
        }
    }

    #[cfg(test)]
    pub(super) fn with_corpus_source_for_test(mut self, unit: &super::corpus::CorpusUnit) -> Self {
        self.source = corpus_source(unit);
        self
    }

    #[cfg(test)]
    pub(super) fn trace_payload(&self) -> &DiagnosticTrace {
        &self.trace
    }

    pub(super) fn internal_failure(&self) -> Option<(InternalFailureKind, &str)> {
        match &self.trace.outcome {
            DiagnosticOutcome::InternalFailure(failure) => {
                Some((failure.kind, failure.message.as_str()))
            }
            DiagnosticOutcome::Selected(_)
            | DiagnosticOutcome::ParseFailure(_)
            | DiagnosticOutcome::UnresolvedAmbiguity(_) => None,
        }
    }
}

fn corpus_source(unit: &super::corpus::CorpusUnit) -> DiagnosticSource {
    DiagnosticSource::Corpus {
        id: unit.id().to_owned(),
        card: unit.card_name().to_owned(),
        face: unit.face_name().map(str::to_owned),
        side: unit.side().map(str::to_owned),
        text: unit.text().to_owned(),
        context: unit.context_name().to_owned(),
    }
}

impl DiagnosticTrace {
    fn from_source<Source: TraceSourceView>(trace: &Source) -> Self {
        Self {
            tokens: Counted::from_source(trace.tokens(), |token| ScannedToken {
                start: token.start(),
                end: token.end(),
                terminal_name_v1: token.terminal_name_v1().to_owned(),
                value_label_v1: token.value_label_v1().to_owned(),
            }),
            final_chart: Counted::from_source(trace.chart(), |item| ChartItem {
                column: item.column(),
                rule_name_v1: item.rule_name_v1().to_owned(),
                dot: item.dot(),
                origin: item.origin(),
                family_count: item.family_count(),
            }),
            forest_nodes: Counted::from_source(trace.forest(), |node| ForestNode {
                id: node.id(),
                rule_name_v1: node.rule_name_v1().to_owned(),
                start: node.start(),
                end: node.end(),
                families: Counted::from_source(node.families(), |family| ForestFamily {
                    children: Counted::from_source(family.children(), |child| ForestChild {
                        node_id: child.node_id(),
                        value_label_v1: child.value_label_v1().map(str::to_owned),
                    }),
                }),
            }),
            forest_roots: Counted::from_source(trace.accepted_roots(), |root| *root),
            checked_completion_rejections: Counted::from_source(
                trace.checked_completion_rejections(),
                |rejection| CheckedCompletionRejection {
                    rule_name_v1: rejection.rule_name_v1().to_owned(),
                    start: rejection.start(),
                    end: rejection.end(),
                    family_identity_v1: family_identity(rejection.family_identity_children()),
                },
            ),
            materialized_candidates: Counted::from_source(
                trace.materialized_candidates(),
                |candidate| MaterializedCandidate {
                    ordinal: candidate.ordinal(),
                    rendered: candidate.rendered().to_owned(),
                    ast_debug_v1: candidate.ast_debug_v1().to_owned(),
                    construction_path: Counted::from_source(
                        candidate.construction_path(),
                        Clone::clone,
                    ),
                    specificity: Counted::from_source(candidate.specificity(), |tier| {
                        specificity(*tier)
                    }),
                },
            ),
            materialization_cycles: Counted::from_source(trace.materialization_cycles(), |cycle| {
                MaterializationCycle {
                    node_ordinal: cycle.node_ordinal(),
                    construction_path: Counted::from_source(
                        cycle.construction_path(),
                        Clone::clone,
                    ),
                }
            }),
            outcome: outcome(trace.outcome()),
        }
    }
}

fn family_identity(children: &[RuntimeFamilyIdentityChild]) -> FamilyIdentity {
    FamilyIdentity {
        children: children
            .iter()
            .map(|child| match child {
                RuntimeFamilyIdentityChild::Node(node) => FamilyIdentityChild::Node(*node),
                RuntimeFamilyIdentityChild::Lexical(value) => {
                    FamilyIdentityChild::Lexical(value.clone())
                }
            })
            .collect(),
    }
}

fn outcome<Selection: SelectionDecisionSource>(
    outcome: TraceOutcomeSource<'_, Selection>,
) -> DiagnosticOutcome {
    match outcome {
        TraceOutcomeSource::Selected {
            rendered,
            selection: source_selection,
        } => DiagnosticOutcome::Selected(SelectedOutcome {
            rendered: rendered.to_owned(),
            selection: selection(source_selection),
        }),
        TraceOutcomeSource::ParseFailure {
            start,
            end,
            expectations,
        } => DiagnosticOutcome::ParseFailure(ParseFailureOutcome {
            span: TextSpan { start, end },
            expectations: Counted::from_source(expectations, expectation),
        }),
        TraceOutcomeSource::UnresolvedAmbiguity {
            selection: source_selection,
        } => DiagnosticOutcome::UnresolvedAmbiguity(UnresolvedOutcome {
            selection: selection(source_selection),
        }),
        TraceOutcomeSource::InternalFailure { kind, message } => {
            DiagnosticOutcome::InternalFailure(InternalFailureOutcome {
                kind: internal_kind(kind),
                message: message.to_owned(),
            })
        }
    }
}

fn expectation(expectation: &RuntimeExpectation) -> Expectation {
    match expectation {
        RuntimeExpectation::Nonterminal(kind) => Expectation::Nonterminal(nonterminal(*kind)),
        RuntimeExpectation::Terminal(kind) => Expectation::Terminal(terminal(*kind)),
        RuntimeExpectation::Literal(value) => Expectation::Literal((*value).to_owned()),
    }
}

fn nonterminal(kind: NonterminalCategory) -> NonterminalKind {
    match kind {
        NonterminalCategory::Ability => NonterminalKind::Ability,
        NonterminalCategory::Sentence => NonterminalKind::Sentence,
        NonterminalCategory::Clause => NonterminalKind::Clause,
        NonterminalCategory::NounPhrase => NonterminalKind::NounPhrase,
        NonterminalCategory::VerbPhrase => NonterminalKind::VerbPhrase,
        NonterminalCategory::Amount => NonterminalKind::Amount,
    }
}

fn terminal(kind: TerminalClass) -> TerminalKind {
    match kind {
        TerminalClass::Article => TerminalKind::Article,
        TerminalClass::Declaration(_) => TerminalKind::Declaration,
        TerminalClass::Demonstrative => TerminalKind::Demonstrative,
        TerminalClass::EndOfInput => TerminalKind::EndOfInput,
        TerminalClass::Noun => TerminalKind::Noun,
        TerminalClass::Pronoun => TerminalKind::Pronoun,
        TerminalClass::SelfReference => TerminalKind::SelfReference,
        TerminalClass::SignedNumber => TerminalKind::SignedNumber,
        TerminalClass::TriggerWord => TerminalKind::TriggerWord,
        TerminalClass::Variable => TerminalKind::Variable,
        TerminalClass::VerbLexeme => TerminalKind::VerbLexeme,
    }
}

fn selection<Source: SelectionDecisionSource>(decision: &Source) -> SelectionDecision {
    SelectionDecision {
        candidates: Counted::from_source(decision.candidates(), selection_candidate),
        comparisons: Counted::from_source(decision.comparisons(), selection_comparison),
        survivors: Counted::from_source(decision.survivors(), |ordinal| *ordinal),
        exception_uses: Counted::from_source(decision.exception_uses(), Clone::clone),
        selected: decision.selected().map(selection_candidate),
        resolution: selection_resolution(decision.resolution()),
        unselected_candidates: Counted::from_source(
            decision.unselected_candidates(),
            unselected_candidate,
        ),
    }
}

fn selection_candidate<Source: SelectionCandidateSource>(candidate: &Source) -> SelectionCandidate {
    SelectionCandidate {
        ordinal: candidate.ordinal(),
        construction_path: Counted::from_source(candidate.construction_path(), Clone::clone),
        specificity: Counted::from_source(candidate.specificity(), |tier| specificity(*tier)),
    }
}

fn selection_comparison<Source: SelectionComparisonSource>(
    comparison: &Source,
) -> SelectionComparison {
    SelectionComparison {
        left_ordinal: comparison.left_ordinal(),
        right_ordinal: comparison.right_ordinal(),
        ordering: ordering(comparison.ordering()),
        decisive: decisive(comparison.decisive()),
        exception_id: comparison.exception_id().map(str::to_owned),
    }
}

fn unselected_candidate<Source: UnselectedCandidateSource>(
    candidate: &Source,
) -> UnselectedCandidate {
    UnselectedCandidate {
        candidate: selection_candidate(candidate.candidate()),
        reason: match candidate.reason() {
            deckmaste_english_v2::parser::SelectionLoserReason::LessSpecific => {
                SelectionLoserReason::LessSpecific
            }
            deckmaste_english_v2::parser::SelectionLoserReason::ExceptionLoser => {
                SelectionLoserReason::ExceptionLoser
            }
            deckmaste_english_v2::parser::SelectionLoserReason::UnresolvedSurvivor => {
                SelectionLoserReason::UnresolvedSurvivor
            }
        },
        evidence: Counted::from_source(candidate.evidence(), selection_comparison),
    }
}

fn ordering(value: Ordering) -> OrderingKind {
    match value {
        Ordering::Less => OrderingKind::Less,
        Ordering::Equal => OrderingKind::Equal,
        Ordering::Greater => OrderingKind::Greater,
    }
}

fn decisive(value: RuntimeSelectionDecisive) -> SelectionDecisive {
    match value {
        RuntimeSelectionDecisive::Position(index) => SelectionDecisive::Position(index),
        RuntimeSelectionDecisive::VectorExhaustion(index) => {
            SelectionDecisive::VectorExhaustion(index)
        }
        RuntimeSelectionDecisive::Tie => SelectionDecisive::Tie,
    }
}

fn specificity(value: RuntimeSpecificityTier) -> SpecificityTier {
    match value {
        RuntimeSpecificityTier::Nonterminal => SpecificityTier::Nonterminal,
        RuntimeSpecificityTier::TypedLexical => SpecificityTier::TypedLexical,
        RuntimeSpecificityTier::Literal => SpecificityTier::Literal,
    }
}

fn selection_resolution(value: RuntimeSelectionResolution) -> SelectionResolution {
    match value {
        RuntimeSelectionResolution::Unique => SelectionResolution::Unique,
        RuntimeSelectionResolution::Specificity => SelectionResolution::Specificity,
        RuntimeSelectionResolution::Exception => SelectionResolution::Exception,
        RuntimeSelectionResolution::UnresolvedTie => SelectionResolution::UnresolvedTie,
    }
}

fn internal_kind(value: RuntimeInternalFailureKind) -> InternalFailureKind {
    match value {
        RuntimeInternalFailureKind::ValidatedRootDidNotMaterialize => {
            InternalFailureKind::ValidatedRootDidNotMaterialize
        }
        RuntimeInternalFailureKind::SelectionConfiguration => {
            InternalFailureKind::SelectionConfiguration
        }
        RuntimeInternalFailureKind::OwnershipInspection => InternalFailureKind::OwnershipInspection,
    }
}

pub(super) fn render(
    report: &DiagnosticReport,
    json: bool,
    output: &mut dyn Write,
) -> anyhow::Result<()> {
    if json {
        serde_json::to_writer_pretty(&mut *output, report)
            .context("write English-v2 diagnostic JSON")?;
        writeln!(output).context("writing English-v2 diagnostic JSON terminator")?;
    } else {
        render_human(report, output)?;
    }
    Ok(())
}

fn render_human(report: &DiagnosticReport, output: &mut dyn Write) -> anyhow::Result<()> {
    match &report.source {
        DiagnosticSource::Probe { text, context } => writeln!(
            output,
            "English v2 diagnostic schema_version={} source_kind=probe text={} context={}",
            report.schema_version,
            json_value(text)?,
            json_value(context)?,
        ),
        DiagnosticSource::Corpus {
            id,
            card,
            face,
            side,
            text,
            context,
        } => writeln!(
            output,
            "English v2 diagnostic schema_version={} source_kind=corpus id={} card={} face={} side={} text={} context={}",
            report.schema_version,
            json_value(id)?,
            json_value(card)?,
            json_value(face)?,
            json_value(side)?,
            json_value(text)?,
            json_value(context)?,
        ),
    }
    .context("writing English-v2 diagnostic human header")?;
    let trace = &report.trace;
    write_counted("tokens", &trace.tokens, output)?;
    write_items("token", &trace.tokens.items, output)?;
    write_counted("final_chart", &trace.final_chart, output)?;
    write_items("chart_item", &trace.final_chart.items, output)?;
    write_counted("forest_nodes", &trace.forest_nodes, output)?;
    for (node_index, node) in trace.forest_nodes.items.iter().enumerate() {
        writeln!(
            output,
            "forest_node index={node_index} id={} rule_name_v1={} start={} end={}",
            node.id,
            json_value(&node.rule_name_v1)?,
            node.start,
            node.end,
        )
        .context("writing English-v2 diagnostic forest node")?;
        write_counted(
            &format!("forest_nodes[{node_index}].families"),
            &node.families,
            output,
        )?;
        for (family_index, family) in node.families.items.iter().enumerate() {
            let name = format!("forest_nodes[{node_index}].families[{family_index}].children");
            write_counted(&name, &family.children, output)?;
            write_items(&format!("{name}.child"), &family.children.items, output)?;
        }
    }
    write_counted("forest_roots", &trace.forest_roots, output)?;
    write_items("forest_root", &trace.forest_roots.items, output)?;
    write_counted(
        "checked_completion_rejections",
        &trace.checked_completion_rejections,
        output,
    )?;
    write_items(
        "checked_completion_rejection",
        &trace.checked_completion_rejections.items,
        output,
    )?;
    write_counted(
        "materialized_candidates",
        &trace.materialized_candidates,
        output,
    )?;
    for (index, candidate) in trace.materialized_candidates.items.iter().enumerate() {
        writeln!(
            output,
            "materialized_candidate index={index} ordinal={} rendered={} ast_debug_v1={}",
            candidate.ordinal,
            json_value(&candidate.rendered)?,
            json_value(&candidate.ast_debug_v1)?,
        )
        .context("writing English-v2 diagnostic materialized candidate")?;
        render_candidate_nested("materialized_candidates", index, candidate, output)?;
    }
    write_counted(
        "materialization_cycles",
        &trace.materialization_cycles,
        output,
    )?;
    for (index, cycle) in trace.materialization_cycles.items.iter().enumerate() {
        writeln!(
            output,
            "materialization_cycle index={index} node_ordinal={}",
            cycle.node_ordinal
        )
        .context("writing English-v2 diagnostic materialization cycle")?;
        let name = format!("materialization_cycles[{index}].construction_path");
        write_counted(&name, &cycle.construction_path, output)?;
        write_items(
            &format!("{name}.item"),
            &cycle.construction_path.items,
            output,
        )?;
    }
    render_outcome(&trace.outcome, output)
}

fn render_candidate_nested(
    section: &str,
    index: usize,
    candidate: &MaterializedCandidate,
    output: &mut dyn Write,
) -> anyhow::Result<()> {
    let path = format!("{section}[{index}].construction_path");
    write_counted(&path, &candidate.construction_path, output)?;
    write_items(
        &format!("{path}.item"),
        &candidate.construction_path.items,
        output,
    )?;
    let specificity = format!("{section}[{index}].specificity");
    write_counted(&specificity, &candidate.specificity, output)?;
    write_items(
        &format!("{specificity}.item"),
        &candidate.specificity.items,
        output,
    )
}

fn render_outcome(outcome: &DiagnosticOutcome, output: &mut dyn Write) -> anyhow::Result<()> {
    match outcome {
        DiagnosticOutcome::Selected(selected) => {
            writeln!(
                output,
                "outcome status=selected rendered={}",
                json_value(&selected.rendered)?
            )
            .context("writing English-v2 diagnostic selected outcome")?;
            render_selection(&selected.selection, output)
        }
        DiagnosticOutcome::ParseFailure(failure) => {
            writeln!(
                output,
                "outcome status=parse_failure span_start={} span_end={}",
                failure.span.start, failure.span.end
            )
            .context("writing English-v2 diagnostic parse-failure outcome")?;
            write_counted("outcome.expectations", &failure.expectations, output)?;
            write_items("outcome.expectation", &failure.expectations.items, output)
        }
        DiagnosticOutcome::UnresolvedAmbiguity(unresolved) => {
            writeln!(output, "outcome status=unresolved_ambiguity")
                .context("writing English-v2 diagnostic unresolved outcome")?;
            render_selection(&unresolved.selection, output)
        }
        DiagnosticOutcome::InternalFailure(failure) => writeln!(
            output,
            "outcome status=internal_failure kind={} message={}",
            failure.kind.as_str(),
            json_value(&failure.message)?
        )
        .context("writing English-v2 diagnostic internal outcome"),
    }
}

fn render_selection(selection: &SelectionDecision, output: &mut dyn Write) -> anyhow::Result<()> {
    writeln!(
        output,
        "selection resolution={} selected={}",
        selection.resolution.as_str(),
        json_value(&selection.selected)?,
    )
    .context("writing English-v2 diagnostic selection")?;
    write_counted("selection.candidates", &selection.candidates, output)?;
    for (index, candidate) in selection.candidates.items.iter().enumerate() {
        writeln!(
            output,
            "selection_candidate index={index} ordinal={}",
            candidate.ordinal
        )
        .context("writing English-v2 diagnostic selection candidate")?;
        render_selection_candidate_nested(
            &format!("selection.candidates[{index}]"),
            candidate,
            output,
        )?;
    }
    write_counted("selection.comparisons", &selection.comparisons, output)?;
    write_items("selection_comparison", &selection.comparisons.items, output)?;
    write_counted("selection.survivors", &selection.survivors, output)?;
    write_items("selection_survivor", &selection.survivors.items, output)?;
    write_counted(
        "selection.exception_uses",
        &selection.exception_uses,
        output,
    )?;
    write_items(
        "selection_exception_use",
        &selection.exception_uses.items,
        output,
    )?;
    if let Some(selected) = &selection.selected {
        render_selection_candidate_nested("selection.selected", selected, output)?;
    }
    write_counted(
        "selection.unselected_candidates",
        &selection.unselected_candidates,
        output,
    )?;
    for (index, unselected) in selection.unselected_candidates.items.iter().enumerate() {
        writeln!(
            output,
            "selection_unselected_candidate index={index} item={}",
            json_value(unselected)?
        )
        .context("writing English-v2 diagnostic unselected candidate")?;
        render_selection_candidate_nested(
            &format!("selection.unselected_candidates[{index}].candidate"),
            &unselected.candidate,
            output,
        )?;
        let evidence = format!("selection.unselected_candidates[{index}].evidence");
        write_counted(&evidence, &unselected.evidence, output)?;
        write_items(
            &format!("{evidence}.comparison"),
            &unselected.evidence.items,
            output,
        )?;
    }
    Ok(())
}

fn render_selection_candidate_nested(
    section: &str,
    candidate: &SelectionCandidate,
    output: &mut dyn Write,
) -> anyhow::Result<()> {
    let path = format!("{section}.construction_path");
    write_counted(&path, &candidate.construction_path, output)?;
    write_items(
        &format!("{path}.item"),
        &candidate.construction_path.items,
        output,
    )?;
    let specificity = format!("{section}.specificity");
    write_counted(&specificity, &candidate.specificity, output)?;
    write_items(
        &format!("{specificity}.item"),
        &candidate.specificity.items,
        output,
    )
}

fn write_counted<T>(
    name: &str,
    counted: &Counted<T>,
    output: &mut dyn Write,
) -> anyhow::Result<()> {
    writeln!(
        output,
        "{name} shown={} total={} omitted={}",
        counted.shown, counted.total, counted.omitted
    )
    .with_context(|| format!("writing English-v2 diagnostic {name} counts"))
}

fn write_items<T: Serialize>(
    name: &str,
    items: &[T],
    output: &mut dyn Write,
) -> anyhow::Result<()> {
    for (index, item) in items.iter().enumerate() {
        writeln!(output, "{name} index={index} item={}", json_value(item)?)
            .with_context(|| format!("writing English-v2 diagnostic {name}"))?;
    }
    Ok(())
}

fn json_value<T: Serialize>(value: &T) -> anyhow::Result<String> {
    serde_json::to_string(value).context("encoding English-v2 diagnostic human field")
}

#[cfg(test)]
fn render_to_vec(report: &DiagnosticReport, json: bool) -> anyhow::Result<Vec<u8>> {
    let mut output = Vec::new();
    render(report, json, &mut output)?;
    Ok(output)
}

#[cfg(test)]
#[derive(Debug, Clone, Copy, Default)]
pub(super) enum FixtureOutcome {
    #[default]
    Selected,
    ParseFailure,
    UnresolvedAmbiguity,
    ValidatedRootDidNotMaterialize,
    SelectionConfiguration,
}

#[cfg(test)]
#[expect(
    clippy::too_many_lines,
    reason = "one complete schema fixture keeps every nested bounded section visible"
)]
pub(super) fn fixture_report(outcome: FixtureOutcome) -> DiagnosticReport {
    fn candidate(ordinal: usize) -> SelectionCandidate {
        SelectionCandidate {
            ordinal,
            construction_path: Counted::fixture(2, vec![format!("path {ordinal}\nline")]),
            specificity: Counted::fixture(2, vec![SpecificityTier::Literal]),
        }
    }
    fn comparison(
        left: usize,
        right: usize,
        ordering: OrderingKind,
        exception_id: Option<&str>,
    ) -> SelectionComparison {
        SelectionComparison {
            left_ordinal: left,
            right_ordinal: right,
            ordering,
            decisive: if exception_id.is_some() {
                SelectionDecisive::Tie
            } else {
                SelectionDecisive::Position(0)
            },
            exception_id: exception_id.map(str::to_owned),
        }
    }
    fn selection_fixture() -> SelectionDecision {
        let less = comparison(1, 0, OrderingKind::Less, None);
        let exception = comparison(2, 0, OrderingKind::Less, Some("fixture\nexception"));
        let tie = comparison(3, 0, OrderingKind::Equal, None);
        SelectionDecision {
            candidates: Counted::fixture(
                4,
                vec![candidate(0), candidate(1), candidate(2), candidate(3)],
            ),
            comparisons: Counted::fixture(3, vec![less.clone(), exception.clone(), tie.clone()]),
            survivors: Counted::fixture(2, vec![0]),
            exception_uses: Counted::fixture(2, vec!["fixture\nexception".to_owned()]),
            selected: Some(candidate(0)),
            resolution: SelectionResolution::Exception,
            unselected_candidates: Counted::fixture(
                3,
                vec![
                    UnselectedCandidate {
                        candidate: candidate(1),
                        reason: SelectionLoserReason::LessSpecific,
                        evidence: Counted::fixture(1, vec![less]),
                    },
                    UnselectedCandidate {
                        candidate: candidate(2),
                        reason: SelectionLoserReason::ExceptionLoser,
                        evidence: Counted::fixture(1, vec![exception]),
                    },
                    UnselectedCandidate {
                        candidate: candidate(3),
                        reason: SelectionLoserReason::UnresolvedSurvivor,
                        evidence: Counted::fixture(1, vec![tie]),
                    },
                ],
            ),
        }
    }

    let selection = selection_fixture();
    let outcome = match outcome {
        FixtureOutcome::Selected => DiagnosticOutcome::Selected(SelectedOutcome {
            rendered: "selected\nrender".to_owned(),
            selection,
        }),
        FixtureOutcome::ParseFailure => DiagnosticOutcome::ParseFailure(ParseFailureOutcome {
            span: TextSpan { start: 4, end: 5 },
            expectations: Counted::fixture(
                3,
                vec![
                    Expectation::Nonterminal(NonterminalKind::Ability),
                    Expectation::Terminal(TerminalKind::Noun),
                    Expectation::Literal("literal\nvalue".to_owned()),
                ],
            ),
        }),
        FixtureOutcome::UnresolvedAmbiguity => {
            DiagnosticOutcome::UnresolvedAmbiguity(UnresolvedOutcome { selection })
        }
        FixtureOutcome::ValidatedRootDidNotMaterialize => {
            DiagnosticOutcome::InternalFailure(InternalFailureOutcome {
                kind: InternalFailureKind::ValidatedRootDidNotMaterialize,
                message: "validated\nroot failure".to_owned(),
            })
        }
        FixtureOutcome::SelectionConfiguration => {
            DiagnosticOutcome::InternalFailure(InternalFailureOutcome {
                kind: InternalFailureKind::SelectionConfiguration,
                message: "selection\nconfiguration failure".to_owned(),
            })
        }
    };
    DiagnosticReport {
        schema_version: 1,
        source: DiagnosticSource::Probe {
            text: "line\nbreak".to_owned(),
            context: "context\rbreak".to_owned(),
        },
        trace: DiagnosticTrace {
            tokens: Counted::fixture(
                2,
                vec![ScannedToken {
                    start: 0,
                    end: 4,
                    terminal_name_v1: "terminal\nname".to_owned(),
                    value_label_v1: "value\rlabel".to_owned(),
                }],
            ),
            final_chart: Counted::fixture(
                1,
                vec![ChartItem {
                    column: 1,
                    rule_name_v1: "rule\nname".to_owned(),
                    dot: 2,
                    origin: 0,
                    family_count: 2,
                }],
            ),
            forest_nodes: Counted::fixture(
                2,
                vec![ForestNode {
                    id: 7,
                    rule_name_v1: "forest\nrule".to_owned(),
                    start: 0,
                    end: 4,
                    families: Counted::fixture(
                        2,
                        vec![ForestFamily {
                            children: Counted::fixture(
                                2,
                                vec![ForestChild {
                                    node_id: None,
                                    value_label_v1: Some("child\nvalue".to_owned()),
                                }],
                            ),
                        }],
                    ),
                }],
            ),
            forest_roots: Counted::fixture(0, vec![]),
            checked_completion_rejections: Counted::fixture(
                1,
                vec![CheckedCompletionRejection {
                    rule_name_v1: "rejected\nrule".to_owned(),
                    start: 1,
                    end: 3,
                    family_identity_v1: FamilyIdentity {
                        children: vec![
                            FamilyIdentityChild::Node(7),
                            FamilyIdentityChild::Lexical("lexical\nvalue".to_owned()),
                        ],
                    },
                }],
            ),
            materialized_candidates: Counted::fixture(
                2,
                vec![MaterializedCandidate {
                    ordinal: 0,
                    rendered: "candidate\nrender".to_owned(),
                    ast_debug_v1: "candidate\ndebug".to_owned(),
                    construction_path: Counted::fixture(2, vec!["candidate\npath".to_owned()]),
                    specificity: Counted::fixture(2, vec![SpecificityTier::TypedLexical]),
                }],
            ),
            materialization_cycles: Counted::fixture(
                1,
                vec![MaterializationCycle {
                    node_ordinal: 9,
                    construction_path: Counted::fixture(2, vec!["cycle\npath".to_owned()]),
                }],
            ),
            outcome,
        },
    }
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeSet;

    use deckmaste_english_v2::context::ParseContext;
    use deckmaste_english_v2::parser::Bounded;
    use deckmaste_english_v2::parser::BoundedParseOutcome;
    use deckmaste_english_v2::parser::Parser;
    use deckmaste_english_v2::parser::ParserTrace;
    use deckmaste_english_v2::parser::TraceLimits;
    use serde_json::Value;

    use super::*;

    #[derive(Debug, Clone)]
    struct FixtureBounded<T> {
        total: usize,
        shown: usize,
        omitted: usize,
        items: Vec<T>,
    }

    impl<T> FixtureBounded<T> {
        fn new(total: usize, items: Vec<T>) -> Self {
            let shown = items.len();
            Self {
                total,
                shown,
                omitted: total - shown,
                items,
            }
        }

        fn view(&self) -> BoundedSource<'_, T> {
            BoundedSource {
                total: self.total,
                shown: self.shown,
                omitted: self.omitted,
                items: &self.items,
            }
        }
    }

    #[derive(Debug, Clone)]
    struct FixtureToken {
        start: usize,
        end: usize,
        terminal: String,
        value: String,
    }

    impl TokenSource for FixtureToken {
        fn start(&self) -> usize {
            self.start
        }

        fn end(&self) -> usize {
            self.end
        }

        fn terminal_name_v1(&self) -> &str {
            &self.terminal
        }

        fn value_label_v1(&self) -> &str {
            &self.value
        }
    }

    #[derive(Debug, Clone)]
    struct FixtureChartItem {
        column: usize,
        rule: String,
        dot: usize,
        origin: usize,
        family_count: usize,
    }

    impl ChartItemSource for FixtureChartItem {
        fn column(&self) -> usize {
            self.column
        }

        fn rule_name_v1(&self) -> &str {
            &self.rule
        }

        fn dot(&self) -> usize {
            self.dot
        }

        fn origin(&self) -> usize {
            self.origin
        }

        fn family_count(&self) -> usize {
            self.family_count
        }
    }

    #[derive(Debug, Clone)]
    struct FixtureForestChild {
        node_id: Option<usize>,
        value: Option<String>,
    }

    impl ForestChildSource for FixtureForestChild {
        fn node_id(&self) -> Option<usize> {
            self.node_id
        }

        fn value_label_v1(&self) -> Option<&str> {
            self.value.as_deref()
        }
    }

    #[derive(Debug, Clone)]
    struct FixtureForestFamily {
        children: FixtureBounded<FixtureForestChild>,
    }

    impl ForestFamilySource for FixtureForestFamily {
        type Child = FixtureForestChild;

        fn children(&self) -> BoundedSource<'_, Self::Child> {
            self.children.view()
        }
    }

    #[derive(Debug, Clone)]
    struct FixtureForestNode {
        id: usize,
        rule: String,
        start: usize,
        end: usize,
        families: FixtureBounded<FixtureForestFamily>,
    }

    impl ForestNodeSource for FixtureForestNode {
        type Family = FixtureForestFamily;

        fn id(&self) -> usize {
            self.id
        }

        fn rule_name_v1(&self) -> &str {
            &self.rule
        }

        fn start(&self) -> usize {
            self.start
        }

        fn end(&self) -> usize {
            self.end
        }

        fn families(&self) -> BoundedSource<'_, Self::Family> {
            self.families.view()
        }
    }

    #[derive(Debug, Clone)]
    struct FixtureRejection {
        rule: String,
        start: usize,
        end: usize,
        family: Vec<deckmaste_english_v2::parser::FamilyIdentityChild>,
    }

    impl CheckedRejectionSource for FixtureRejection {
        fn rule_name_v1(&self) -> &str {
            &self.rule
        }

        fn start(&self) -> usize {
            self.start
        }

        fn end(&self) -> usize {
            self.end
        }

        fn family_identity_children(&self) -> &[deckmaste_english_v2::parser::FamilyIdentityChild] {
            &self.family
        }
    }

    #[derive(Debug, Clone)]
    struct FixtureMaterializedCandidate {
        ordinal: usize,
        rendered: String,
        debug: String,
        path: FixtureBounded<String>,
        specificity: FixtureBounded<deckmaste_english_v2::parser::SpecificityTier>,
    }

    impl MaterializedCandidateSource for FixtureMaterializedCandidate {
        fn ordinal(&self) -> usize {
            self.ordinal
        }

        fn rendered(&self) -> &str {
            &self.rendered
        }

        fn ast_debug_v1(&self) -> &str {
            &self.debug
        }

        fn construction_path(&self) -> BoundedSource<'_, String> {
            self.path.view()
        }

        fn specificity(&self) -> BoundedSource<'_, deckmaste_english_v2::parser::SpecificityTier> {
            self.specificity.view()
        }
    }

    #[derive(Debug, Clone)]
    struct FixtureCycle {
        node_ordinal: usize,
        path: FixtureBounded<String>,
    }

    impl MaterializationCycleSource for FixtureCycle {
        fn node_ordinal(&self) -> usize {
            self.node_ordinal
        }

        fn construction_path(&self) -> BoundedSource<'_, String> {
            self.path.view()
        }
    }

    #[derive(Debug, Clone)]
    struct FixtureSelectionCandidate {
        ordinal: usize,
        path: FixtureBounded<String>,
        specificity: FixtureBounded<deckmaste_english_v2::parser::SpecificityTier>,
    }

    impl SelectionCandidateSource for FixtureSelectionCandidate {
        fn ordinal(&self) -> usize {
            self.ordinal
        }

        fn construction_path(&self) -> BoundedSource<'_, String> {
            self.path.view()
        }

        fn specificity(&self) -> BoundedSource<'_, deckmaste_english_v2::parser::SpecificityTier> {
            self.specificity.view()
        }
    }

    #[derive(Debug, Clone)]
    struct FixtureComparison {
        left: usize,
        right: usize,
        ordering: Ordering,
        decisive: deckmaste_english_v2::parser::SelectionDecisive,
        exception_id: Option<String>,
    }

    impl SelectionComparisonSource for FixtureComparison {
        fn left_ordinal(&self) -> usize {
            self.left
        }

        fn right_ordinal(&self) -> usize {
            self.right
        }

        fn ordering(&self) -> Ordering {
            self.ordering
        }

        fn decisive(&self) -> deckmaste_english_v2::parser::SelectionDecisive {
            self.decisive
        }

        fn exception_id(&self) -> Option<&str> {
            self.exception_id.as_deref()
        }
    }

    #[derive(Debug, Clone)]
    struct FixtureUnselected {
        candidate: FixtureSelectionCandidate,
        reason: deckmaste_english_v2::parser::SelectionLoserReason,
        evidence: FixtureBounded<FixtureComparison>,
    }

    impl UnselectedCandidateSource for FixtureUnselected {
        type Candidate = FixtureSelectionCandidate;
        type Comparison = FixtureComparison;

        fn candidate(&self) -> &Self::Candidate {
            &self.candidate
        }

        fn reason(&self) -> deckmaste_english_v2::parser::SelectionLoserReason {
            self.reason
        }

        fn evidence(&self) -> BoundedSource<'_, Self::Comparison> {
            self.evidence.view()
        }
    }

    #[derive(Debug, Clone)]
    struct FixtureSelection {
        candidates: FixtureBounded<FixtureSelectionCandidate>,
        comparisons: FixtureBounded<FixtureComparison>,
        survivors: FixtureBounded<usize>,
        selected: Option<FixtureSelectionCandidate>,
        resolution: deckmaste_english_v2::parser::SelectionResolution,
        exception_uses: FixtureBounded<String>,
        unselected: FixtureBounded<FixtureUnselected>,
    }

    impl SelectionDecisionSource for FixtureSelection {
        type Candidate = FixtureSelectionCandidate;
        type Comparison = FixtureComparison;
        type Unselected = FixtureUnselected;

        fn candidates(&self) -> BoundedSource<'_, Self::Candidate> {
            self.candidates.view()
        }

        fn comparisons(&self) -> BoundedSource<'_, Self::Comparison> {
            self.comparisons.view()
        }

        fn survivors(&self) -> BoundedSource<'_, usize> {
            self.survivors.view()
        }

        fn selected(&self) -> Option<&Self::Candidate> {
            self.selected.as_ref()
        }

        fn resolution(&self) -> deckmaste_english_v2::parser::SelectionResolution {
            self.resolution
        }

        fn exception_uses(&self) -> BoundedSource<'_, String> {
            self.exception_uses.view()
        }

        fn unselected_candidates(&self) -> BoundedSource<'_, Self::Unselected> {
            self.unselected.view()
        }
    }

    #[derive(Debug, Clone)]
    enum FixtureTraceOutcome {
        ParseFailure {
            start: usize,
            end: usize,
            expectations: FixtureBounded<deckmaste_english_v2::parser::ExpectationInfo>,
        },
        Unresolved(FixtureSelection),
        Internal {
            kind: deckmaste_english_v2::parser::InternalFailureKind,
            message: String,
        },
    }

    #[derive(Debug, Clone)]
    struct FixtureTraceSource {
        tokens: FixtureBounded<FixtureToken>,
        chart: FixtureBounded<FixtureChartItem>,
        forest: FixtureBounded<FixtureForestNode>,
        roots: FixtureBounded<usize>,
        rejections: FixtureBounded<FixtureRejection>,
        candidates: FixtureBounded<FixtureMaterializedCandidate>,
        cycles: FixtureBounded<FixtureCycle>,
        outcome: FixtureTraceOutcome,
    }

    impl TraceSourceView for FixtureTraceSource {
        type Token = FixtureToken;
        type ChartItem = FixtureChartItem;
        type ForestNode = FixtureForestNode;
        type CheckedRejection = FixtureRejection;
        type MaterializedCandidate = FixtureMaterializedCandidate;
        type MaterializationCycle = FixtureCycle;
        type Selection = FixtureSelection;

        fn tokens(&self) -> BoundedSource<'_, Self::Token> {
            self.tokens.view()
        }

        fn chart(&self) -> BoundedSource<'_, Self::ChartItem> {
            self.chart.view()
        }

        fn forest(&self) -> BoundedSource<'_, Self::ForestNode> {
            self.forest.view()
        }

        fn accepted_roots(&self) -> BoundedSource<'_, usize> {
            self.roots.view()
        }

        fn checked_completion_rejections(&self) -> BoundedSource<'_, Self::CheckedRejection> {
            self.rejections.view()
        }

        fn materialized_candidates(&self) -> BoundedSource<'_, Self::MaterializedCandidate> {
            self.candidates.view()
        }

        fn materialization_cycles(&self) -> BoundedSource<'_, Self::MaterializationCycle> {
            self.cycles.view()
        }

        fn outcome(&self) -> TraceOutcomeSource<'_, Self::Selection> {
            match &self.outcome {
                FixtureTraceOutcome::ParseFailure {
                    start,
                    end,
                    expectations,
                } => TraceOutcomeSource::ParseFailure {
                    start: *start,
                    end: *end,
                    expectations: expectations.view(),
                },
                FixtureTraceOutcome::Unresolved(selection) => {
                    TraceOutcomeSource::UnresolvedAmbiguity { selection }
                }
                FixtureTraceOutcome::Internal { kind, message } => {
                    TraceOutcomeSource::InternalFailure {
                        kind: *kind,
                        message,
                    }
                }
            }
        }
    }

    fn fixture_selection_source() -> FixtureSelection {
        let candidate = |ordinal, path: &str, tier| FixtureSelectionCandidate {
            ordinal,
            path: FixtureBounded::new(2, vec![path.to_owned()]),
            specificity: FixtureBounded::new(2, vec![tier]),
        };
        let comparison = FixtureComparison {
            left: 4,
            right: 9,
            ordering: Ordering::Equal,
            decisive: deckmaste_english_v2::parser::SelectionDecisive::Tie,
            exception_id: None,
        };
        FixtureSelection {
            candidates: FixtureBounded::new(
                3,
                vec![
                    candidate(
                        4,
                        "source candidate four\npath",
                        deckmaste_english_v2::parser::SpecificityTier::Literal,
                    ),
                    candidate(
                        9,
                        "source candidate nine path",
                        deckmaste_english_v2::parser::SpecificityTier::Nonterminal,
                    ),
                ],
            ),
            comparisons: FixtureBounded::new(2, vec![comparison.clone()]),
            survivors: FixtureBounded::new(3, vec![4, 9]),
            selected: None,
            resolution: deckmaste_english_v2::parser::SelectionResolution::UnresolvedTie,
            exception_uses: FixtureBounded::new(1, vec![]),
            unselected: FixtureBounded::new(
                3,
                vec![FixtureUnselected {
                    candidate: candidate(
                        4,
                        "source loser\npath",
                        deckmaste_english_v2::parser::SpecificityTier::TypedLexical,
                    ),
                    reason: deckmaste_english_v2::parser::SelectionLoserReason::UnresolvedSurvivor,
                    evidence: FixtureBounded::new(2, vec![comparison]),
                }],
            ),
        }
    }

    fn fixture_trace_source(outcome: FixtureTraceOutcome) -> FixtureTraceSource {
        FixtureTraceSource {
            tokens: FixtureBounded::new(
                2,
                vec![FixtureToken {
                    start: 2,
                    end: 8,
                    terminal: "source terminal\nname".to_owned(),
                    value: "source value".to_owned(),
                }],
            ),
            chart: FixtureBounded::new(
                2,
                vec![FixtureChartItem {
                    column: 8,
                    rule: "source rule".to_owned(),
                    dot: 3,
                    origin: 2,
                    family_count: 5,
                }],
            ),
            forest: FixtureBounded::new(
                2,
                vec![FixtureForestNode {
                    id: 17,
                    rule: "source forest".to_owned(),
                    start: 2,
                    end: 8,
                    families: FixtureBounded::new(
                        2,
                        vec![FixtureForestFamily {
                            children: FixtureBounded::new(
                                2,
                                vec![FixtureForestChild {
                                    node_id: None,
                                    value: Some("source child\nvalue".to_owned()),
                                }],
                            ),
                        }],
                    ),
                }],
            ),
            roots: FixtureBounded::new(2, vec![17]),
            rejections: FixtureBounded::new(
                2,
                vec![FixtureRejection {
                    rule: "source rejected rule\nname".to_owned(),
                    start: 2,
                    end: 8,
                    family: vec![
                        deckmaste_english_v2::parser::FamilyIdentityChild::Node(17),
                        deckmaste_english_v2::parser::FamilyIdentityChild::Lexical(
                            "source family\nlexical".to_owned(),
                        ),
                        deckmaste_english_v2::parser::FamilyIdentityChild::Node(3),
                    ],
                }],
            ),
            candidates: FixtureBounded::new(
                2,
                vec![FixtureMaterializedCandidate {
                    ordinal: 11,
                    rendered: "source candidate\nrender".to_owned(),
                    debug: "source candidate\ndebug".to_owned(),
                    path: FixtureBounded::new(
                        3,
                        vec!["source path one".to_owned(), "source\npath two".to_owned()],
                    ),
                    specificity: FixtureBounded::new(
                        4,
                        vec![
                            deckmaste_english_v2::parser::SpecificityTier::Literal,
                            deckmaste_english_v2::parser::SpecificityTier::TypedLexical,
                            deckmaste_english_v2::parser::SpecificityTier::Nonterminal,
                        ],
                    ),
                }],
            ),
            cycles: FixtureBounded::new(
                2,
                vec![FixtureCycle {
                    node_ordinal: 23,
                    path: FixtureBounded::new(
                        3,
                        vec![
                            "source cycle one".to_owned(),
                            "source\ncycle two".to_owned(),
                        ],
                    ),
                }],
            ),
            outcome,
        }
    }

    fn parser() -> Parser {
        crate::english_v2::parser_from_builtin_v2()
    }

    fn trace(text: &str, context: &str, limit: usize) -> ParserTrace {
        let context = ParseContext::new(context).expect("fixture context is valid");
        parser().trace(text, &context, TraceLimits::new(limit))
    }

    fn assert_counts<T, U>(runtime: &Bounded<T>, projected: &Counted<U>) {
        assert_eq!(projected.total, runtime.total());
        assert_eq!(projected.shown, runtime.shown());
        assert_eq!(projected.omitted, runtime.omitted());
        assert_eq!(projected.items.len(), runtime.items().len());
    }

    fn assert_selection_candidate(
        runtime: &deckmaste_english_v2::parser::BoundedSelectionCandidate,
        projected: &SelectionCandidate,
    ) {
        assert_eq!(projected.ordinal, runtime.ordinal());
        assert_counts(runtime.construction_path(), &projected.construction_path);
        assert_eq!(
            projected.construction_path.items,
            runtime.construction_path().items()
        );
        assert_counts(runtime.specificity(), &projected.specificity);
        for (actual, dto) in runtime
            .specificity()
            .items()
            .iter()
            .zip(&projected.specificity.items)
        {
            assert_eq!(*dto, specificity(*actual));
        }
    }

    fn assert_selection(
        runtime: &deckmaste_english_v2::parser::BoundedSelectionDecision,
        projected: &SelectionDecision,
    ) {
        assert_counts(runtime.candidates(), &projected.candidates);
        for (actual, dto) in runtime
            .candidates()
            .items()
            .iter()
            .zip(&projected.candidates.items)
        {
            assert_selection_candidate(actual, dto);
        }
        assert_counts(runtime.comparisons(), &projected.comparisons);
        for (actual, dto) in runtime
            .comparisons()
            .items()
            .iter()
            .zip(&projected.comparisons.items)
        {
            assert_eq!(dto.left_ordinal, actual.left_ordinal());
            assert_eq!(dto.right_ordinal, actual.right_ordinal());
            assert_eq!(dto.ordering, ordering(actual.ordering()));
            assert_eq!(dto.decisive, decisive(actual.decisive()));
            assert_eq!(dto.exception_id.as_deref(), actual.exception_id());
        }
        assert_counts(runtime.survivors(), &projected.survivors);
        assert_eq!(projected.survivors.items, runtime.survivors().items());
        assert_counts(runtime.exception_uses(), &projected.exception_uses);
        assert_eq!(
            projected.exception_uses.items,
            runtime.exception_uses().items()
        );
        assert_eq!(
            projected.resolution,
            selection_resolution(runtime.resolution())
        );
        match (runtime.selected(), projected.selected.as_ref()) {
            (Some(actual), Some(dto)) => assert_selection_candidate(actual, dto),
            (None, None) => {}
            _ => panic!("selected metadata presence changed during projection"),
        }
        assert_counts(
            runtime.unselected_candidates(),
            &projected.unselected_candidates,
        );
        for (actual, dto) in runtime
            .unselected_candidates()
            .items()
            .iter()
            .zip(&projected.unselected_candidates.items)
        {
            assert_selection_candidate(actual.candidate(), &dto.candidate);
            assert_eq!(dto.reason, loser_reason(actual.reason()));
            assert_counts(actual.evidence(), &dto.evidence);
            for (actual_evidence, dto_evidence) in
                actual.evidence().items().iter().zip(&dto.evidence.items)
            {
                assert_eq!(dto_evidence.left_ordinal, actual_evidence.left_ordinal());
                assert_eq!(dto_evidence.right_ordinal, actual_evidence.right_ordinal());
                assert_eq!(dto_evidence.ordering, ordering(actual_evidence.ordering()));
                assert_eq!(dto_evidence.decisive, decisive(actual_evidence.decisive()));
                assert_eq!(
                    dto_evidence.exception_id.as_deref(),
                    actual_evidence.exception_id()
                );
            }
        }
    }

    fn loser_reason(
        reason: deckmaste_english_v2::parser::SelectionLoserReason,
    ) -> SelectionLoserReason {
        match reason {
            deckmaste_english_v2::parser::SelectionLoserReason::LessSpecific => {
                SelectionLoserReason::LessSpecific
            }
            deckmaste_english_v2::parser::SelectionLoserReason::ExceptionLoser => {
                SelectionLoserReason::ExceptionLoser
            }
            deckmaste_english_v2::parser::SelectionLoserReason::UnresolvedSurvivor => {
                SelectionLoserReason::UnresolvedSurvivor
            }
        }
    }

    #[test]
    #[expect(
        clippy::too_many_lines,
        reason = "one field-by-field assertion authenticates the complete public trace projection"
    )]
    fn projection_authenticates_every_public_runtime_counter_at_caps_zero_one_and_above_total() {
        let text = "Whenever a player connives, you gain X life.";
        let context = "Probe Card";

        for limit in [0, 1, usize::MAX] {
            let runtime = trace(text, context, limit);
            let projected = DiagnosticReport::from_probe(text, context, &runtime);
            assert_eq!(projected.schema_version, 1);
            assert_eq!(projected.source.kind(), SourceKind::Probe);
            assert_eq!(projected.source.text(), text);
            assert_eq!(projected.source.context(), context);
            assert_counts(runtime.tokens(), &projected.trace.tokens);
            assert_counts(runtime.chart(), &projected.trace.final_chart);
            assert_counts(runtime.forest(), &projected.trace.forest_nodes);
            assert_counts(runtime.accepted_roots(), &projected.trace.forest_roots);
            assert_counts(
                runtime.checked_completion_rejections(),
                &projected.trace.checked_completion_rejections,
            );
            assert_counts(
                runtime.materialized_candidates(),
                &projected.trace.materialized_candidates,
            );
            assert_counts(
                runtime.materialization_cycles(),
                &projected.trace.materialization_cycles,
            );

            for (actual, dto) in runtime
                .tokens()
                .items()
                .iter()
                .zip(&projected.trace.tokens.items)
            {
                assert_eq!(dto.start, actual.start());
                assert_eq!(dto.end, actual.end());
                assert_eq!(dto.terminal_name_v1, actual.terminal_name_v1());
                assert_eq!(dto.value_label_v1, actual.value_label_v1());
            }
            for (actual, dto) in runtime
                .chart()
                .items()
                .iter()
                .zip(&projected.trace.final_chart.items)
            {
                assert_eq!(dto.column, actual.column());
                assert_eq!(dto.rule_name_v1, actual.rule_name_v1());
                assert_eq!(dto.dot, actual.dot());
                assert_eq!(dto.origin, actual.origin());
                assert_eq!(dto.family_count, actual.family_count());
            }
            for (actual, dto) in runtime
                .forest()
                .items()
                .iter()
                .zip(&projected.trace.forest_nodes.items)
            {
                assert_eq!(dto.id, actual.id());
                assert_eq!(dto.rule_name_v1, actual.rule_name_v1());
                assert_eq!(dto.start, actual.start());
                assert_eq!(dto.end, actual.end());
                assert_counts(actual.families(), &dto.families);
                for (actual_family, dto_family) in
                    actual.families().items().iter().zip(&dto.families.items)
                {
                    assert_counts(actual_family.children(), &dto_family.children);
                    for (actual_child, dto_child) in actual_family
                        .children()
                        .items()
                        .iter()
                        .zip(&dto_family.children.items)
                    {
                        assert_eq!(dto_child.node_id, actual_child.node_id());
                        assert_eq!(
                            dto_child.value_label_v1.as_deref(),
                            actual_child.value_label_v1()
                        );
                    }
                }
            }
            assert_eq!(
                projected.trace.forest_roots.items,
                runtime.accepted_roots().items()
            );
            for (actual, dto) in runtime
                .checked_completion_rejections()
                .items()
                .iter()
                .zip(&projected.trace.checked_completion_rejections.items)
            {
                assert_eq!(dto.rule_name_v1, actual.rule_name_v1());
                assert_eq!(dto.start, actual.start());
                assert_eq!(dto.end, actual.end());
                for (actual_child, dto_child) in actual
                    .family_identity_v1()
                    .children()
                    .iter()
                    .zip(&dto.family_identity_v1.children)
                {
                    match (actual_child, dto_child) {
                        (
                            deckmaste_english_v2::parser::FamilyIdentityChild::Node(actual),
                            FamilyIdentityChild::Node(dto),
                        ) => assert_eq!(actual, dto),
                        (
                            deckmaste_english_v2::parser::FamilyIdentityChild::Lexical(actual),
                            FamilyIdentityChild::Lexical(dto),
                        ) => assert_eq!(actual, dto),
                        _ => panic!("family identity child kind changed during projection"),
                    }
                }
            }
            for (actual, dto) in runtime
                .materialized_candidates()
                .items()
                .iter()
                .zip(&projected.trace.materialized_candidates.items)
            {
                assert_eq!(dto.ordinal, actual.ordinal());
                assert_eq!(dto.rendered, actual.rendered());
                assert_eq!(dto.ast_debug_v1, actual.ast_debug_v1());
                assert_counts(actual.construction_path(), &dto.construction_path);
                assert_eq!(
                    dto.construction_path.items,
                    actual.construction_path().items()
                );
                assert_counts(actual.specificity(), &dto.specificity);
            }
            for (actual, dto) in runtime
                .materialization_cycles()
                .items()
                .iter()
                .zip(&projected.trace.materialization_cycles.items)
            {
                assert_eq!(dto.node_ordinal, actual.node_ordinal());
                assert_counts(actual.construction_path(), &dto.construction_path);
                assert_eq!(
                    dto.construction_path.items,
                    actual.construction_path().items()
                );
            }
            let (
                BoundedParseOutcome::Selected(actual),
                DiagnosticOutcome::Selected(projected_outcome),
            ) = (runtime.outcome(), &projected.trace.outcome)
            else {
                panic!("fixture must remain selected");
            };
            assert_eq!(projected_outcome.rendered, actual.rendered());
            assert_selection(actual.selection(), &projected_outcome.selection);
            assert_eq!(
                DiagnosticReport::from_probe(text, context, &runtime),
                projected
            );
        }
    }

    #[test]
    fn projection_keeps_overlapping_spans_nested_omissions_rejections_and_failure_expectations() {
        let overlap = trace("You gain X life.", "You", usize::MAX);
        let overlap = DiagnosticReport::from_probe("You gain X life.", "You", &overlap);
        let spans: Vec<_> = overlap
            .trace
            .tokens
            .items
            .iter()
            .map(|token| (token.start, token.end))
            .collect();
        assert!(
            spans
                .iter()
                .any(|span| spans.iter().filter(|other| *other == span).count() > 1),
            "scanner tokens must retain overlapping successful spans: {spans:?}"
        );

        let bounded = trace(
            "Whenever a player connives, you gain X life.",
            "Probe Card",
            1,
        );
        let bounded = DiagnosticReport::from_probe(
            "Whenever a player connives, you gain X life.",
            "Probe Card",
            &bounded,
        );
        assert!(bounded.trace.forest_nodes.omitted > 0);
        assert!(bounded.trace.forest_nodes.items.iter().any(|node| {
            node.families
                .items
                .iter()
                .any(|family| family.children.omitted > 0)
                || node.families.omitted > 0
        }));

        let rejected_runtime = trace("You gains X life.", "Probe Card", usize::MAX);
        let rejected =
            DiagnosticReport::from_probe("You gains X life.", "Probe Card", &rejected_runtime);
        assert!(rejected.trace.checked_completion_rejections.total > 0);
        let DiagnosticOutcome::ParseFailure(failure) = rejected.trace.outcome else {
            panic!("fixture must remain an ordinary parse failure");
        };
        let BoundedParseOutcome::ParseFailure(actual) = rejected_runtime.outcome() else {
            panic!("runtime fixture must remain a parse failure");
        };
        assert_eq!(failure.span.start, actual.span().start);
        assert_eq!(failure.span.end, actual.span().end);
        assert_counts(actual.expectations(), &failure.expectations);
    }

    #[test]
    fn schema_v1_json_and_human_render_every_outcome_and_nested_bounded_section_deterministically()
    {
        for outcome in [
            FixtureOutcome::Selected,
            FixtureOutcome::ParseFailure,
            FixtureOutcome::UnresolvedAmbiguity,
            FixtureOutcome::ValidatedRootDidNotMaterialize,
            FixtureOutcome::SelectionConfiguration,
        ] {
            let report = fixture_report(outcome);
            let first = render_to_vec(&report, true).unwrap();
            let second = render_to_vec(&report, true).unwrap();
            assert_eq!(first, second);
            let value: Value = serde_json::from_slice(&first).unwrap();
            assert_eq!(value["schema_version"], 1);
            let round_trip: DiagnosticReport = serde_json::from_slice(&first).unwrap();
            assert_eq!(round_trip, report);

            let human = String::from_utf8(render_to_vec(&report, false).unwrap()).unwrap();
            assert!(!human.contains("..."));
            assert!(!human.contains("line\nbreak"));
            for section in [
                "tokens",
                "final_chart",
                "forest_nodes",
                "forest_roots",
                "checked_completion_rejections",
                "materialized_candidates",
                "materialization_cycles",
            ] {
                let row = human
                    .lines()
                    .find(|line| line.starts_with(section))
                    .unwrap();
                assert!(row.contains("shown="));
                assert!(row.contains("total="));
                assert!(row.contains("omitted="));
            }
        }
    }

    #[test]
    fn schema_v1_pins_status_reason_and_internal_kind_spellings() {
        let selected: Value = serde_json::from_slice(
            &render_to_vec(&fixture_report(FixtureOutcome::Selected), true).unwrap(),
        )
        .unwrap();
        assert_eq!(selected["trace"]["outcome"]["status"], "selected");
        let reasons: BTreeSet<_> =
            selected["trace"]["outcome"]["selection"]["unselected_candidates"]["items"]
                .as_array()
                .unwrap()
                .iter()
                .map(|item| item["reason"].as_str().unwrap())
                .collect();
        assert_eq!(
            reasons,
            BTreeSet::from(["exception_loser", "less_specific", "unresolved_survivor"])
        );
        for (fixture, spelling) in [
            (
                FixtureOutcome::ValidatedRootDidNotMaterialize,
                "validated_root_did_not_materialize",
            ),
            (
                FixtureOutcome::SelectionConfiguration,
                "selection_configuration",
            ),
        ] {
            let value: Value =
                serde_json::from_slice(&render_to_vec(&fixture_report(fixture), true).unwrap())
                    .unwrap();
            assert_eq!(value["trace"]["outcome"]["status"], "internal_failure");
            assert_eq!(value["trace"]["outcome"]["kind"], spelling);
        }
    }

    #[test]
    fn human_renderer_names_every_nested_bounded_section_with_exact_counts() {
        let selected = String::from_utf8(
            render_to_vec(&fixture_report(FixtureOutcome::Selected), false).unwrap(),
        )
        .unwrap();
        for exact_line in [
            "forest_nodes[0].families shown=1 total=2 omitted=1",
            "forest_nodes[0].families[0].children shown=1 total=2 omitted=1",
            "forest_roots shown=0 total=0 omitted=0",
            "materialized_candidates[0].construction_path shown=1 total=2 omitted=1",
            "materialized_candidates[0].specificity shown=1 total=2 omitted=1",
            "materialization_cycles[0].construction_path shown=1 total=2 omitted=1",
            "selection.candidates shown=4 total=4 omitted=0",
            "selection.comparisons shown=3 total=3 omitted=0",
            "selection.survivors shown=1 total=2 omitted=1",
            "selection.exception_uses shown=1 total=2 omitted=1",
            "selection.selected.construction_path shown=1 total=2 omitted=1",
            "selection.selected.specificity shown=1 total=2 omitted=1",
            "selection.unselected_candidates shown=3 total=3 omitted=0",
        ] {
            assert!(
                selected.lines().any(|line| line == exact_line),
                "missing nested count row {exact_line:?}\n{selected}"
            );
        }
        for index in 0..4 {
            for suffix in ["construction_path", "specificity"] {
                let exact_line =
                    format!("selection.candidates[{index}].{suffix} shown=1 total=2 omitted=1");
                assert!(
                    selected.lines().any(|line| line == exact_line),
                    "missing nested count row {exact_line:?}"
                );
            }
        }
        for index in 0..3 {
            for suffix in ["construction_path", "specificity"] {
                let exact_line = format!(
                    "selection.unselected_candidates[{index}].candidate.{suffix} shown=1 total=2 omitted=1"
                );
                assert!(
                    selected.lines().any(|line| line == exact_line),
                    "missing nested count row {exact_line:?}"
                );
            }
            let exact_line = format!(
                "selection.unselected_candidates[{index}].evidence shown=1 total=1 omitted=0"
            );
            assert!(
                selected.lines().any(|line| line == exact_line),
                "missing nested count row {exact_line:?}"
            );
        }
        for reason in [
            "\"reason\":\"less_specific\"",
            "\"reason\":\"exception_loser\"",
            "\"reason\":\"unresolved_survivor\"",
        ] {
            assert!(selected.contains(reason), "missing loser reason {reason}");
        }

        let failure = String::from_utf8(
            render_to_vec(&fixture_report(FixtureOutcome::ParseFailure), false).unwrap(),
        )
        .unwrap();
        assert!(
            failure
                .lines()
                .any(|line| { line == "outcome.expectations shown=3 total=3 omitted=0" })
        );
        assert!(!selected.contains("..."));
        assert!(!failure.contains("..."));
        assert!(!selected.contains("line\nbreak"));
        assert!(!failure.contains("literal\nvalue"));
    }

    #[test]
    fn source_view_mapper_preserves_cycles_candidates_rejections_and_expectation_kinds_exactly() {
        let source = fixture_trace_source(FixtureTraceOutcome::ParseFailure {
            start: 31,
            end: 37,
            expectations: FixtureBounded::new(
                4,
                vec![
                    deckmaste_english_v2::parser::ExpectationInfo::Literal("source literal"),
                    deckmaste_english_v2::parser::ExpectationInfo::Terminal(
                        deckmaste_english_v2::parser::TerminalClass::Noun,
                    ),
                    deckmaste_english_v2::parser::ExpectationInfo::Nonterminal(
                        deckmaste_english_v2::parser::NonterminalCategory::Clause,
                    ),
                ],
            ),
        });
        let report = DiagnosticReport::from_probe_source("source text", "source context", &source);

        assert_eq!(
            report.trace.materialization_cycles,
            Counted {
                total: 2,
                shown: 1,
                omitted: 1,
                items: vec![MaterializationCycle {
                    node_ordinal: 23,
                    construction_path: Counted {
                        total: 3,
                        shown: 2,
                        omitted: 1,
                        items: vec![
                            "source cycle one".to_owned(),
                            "source\ncycle two".to_owned(),
                        ],
                    },
                }],
            }
        );
        assert_eq!(
            report.trace.materialized_candidates,
            Counted {
                total: 2,
                shown: 1,
                omitted: 1,
                items: vec![MaterializedCandidate {
                    ordinal: 11,
                    rendered: "source candidate\nrender".to_owned(),
                    ast_debug_v1: "source candidate\ndebug".to_owned(),
                    construction_path: Counted {
                        total: 3,
                        shown: 2,
                        omitted: 1,
                        items: vec!["source path one".to_owned(), "source\npath two".to_owned(),],
                    },
                    specificity: Counted {
                        total: 4,
                        shown: 3,
                        omitted: 1,
                        items: vec![
                            SpecificityTier::Literal,
                            SpecificityTier::TypedLexical,
                            SpecificityTier::Nonterminal,
                        ],
                    },
                }],
            }
        );
        assert_eq!(
            report.trace.checked_completion_rejections,
            Counted {
                total: 2,
                shown: 1,
                omitted: 1,
                items: vec![CheckedCompletionRejection {
                    rule_name_v1: "source rejected rule\nname".to_owned(),
                    start: 2,
                    end: 8,
                    family_identity_v1: FamilyIdentity {
                        children: vec![
                            FamilyIdentityChild::Node(17),
                            FamilyIdentityChild::Lexical("source family\nlexical".to_owned(),),
                            FamilyIdentityChild::Node(3),
                        ],
                    },
                }],
            }
        );
        let DiagnosticOutcome::ParseFailure(failure) = report.trace.outcome else {
            panic!("source fixture outcome was remapped");
        };
        assert_eq!(failure.span, TextSpan { start: 31, end: 37 });
        assert_eq!(
            failure.expectations,
            Counted {
                total: 4,
                shown: 3,
                omitted: 1,
                items: vec![
                    Expectation::Literal("source literal".to_owned()),
                    Expectation::Terminal(TerminalKind::Noun),
                    Expectation::Nonterminal(NonterminalKind::Clause),
                ],
            }
        );
    }

    #[test]
    fn source_view_mapper_preserves_the_complete_unresolved_bounded_decision() {
        let source =
            fixture_trace_source(FixtureTraceOutcome::Unresolved(fixture_selection_source()));
        let report = DiagnosticReport::from_probe_source("source text", "source context", &source);
        let DiagnosticOutcome::UnresolvedAmbiguity(unresolved) = report.trace.outcome else {
            panic!("source fixture outcome was remapped");
        };
        let selection = unresolved.selection;
        let comparison = SelectionComparison {
            left_ordinal: 4,
            right_ordinal: 9,
            ordering: OrderingKind::Equal,
            decisive: SelectionDecisive::Tie,
            exception_id: None,
        };
        let candidate = |ordinal, path: &str, specificity| SelectionCandidate {
            ordinal,
            construction_path: Counted {
                total: 2,
                shown: 1,
                omitted: 1,
                items: vec![path.to_owned()],
            },
            specificity: Counted {
                total: 2,
                shown: 1,
                omitted: 1,
                items: vec![specificity],
            },
        };
        assert_eq!(
            selection,
            SelectionDecision {
                candidates: Counted {
                    total: 3,
                    shown: 2,
                    omitted: 1,
                    items: vec![
                        candidate(4, "source candidate four\npath", SpecificityTier::Literal,),
                        candidate(
                            9,
                            "source candidate nine path",
                            SpecificityTier::Nonterminal,
                        ),
                    ],
                },
                comparisons: Counted {
                    total: 2,
                    shown: 1,
                    omitted: 1,
                    items: vec![comparison.clone()],
                },
                survivors: Counted {
                    total: 3,
                    shown: 2,
                    omitted: 1,
                    items: vec![4, 9],
                },
                exception_uses: Counted {
                    total: 1,
                    shown: 0,
                    omitted: 1,
                    items: vec![],
                },
                selected: None,
                resolution: SelectionResolution::UnresolvedTie,
                unselected_candidates: Counted {
                    total: 3,
                    shown: 1,
                    omitted: 2,
                    items: vec![UnselectedCandidate {
                        candidate: candidate(
                            4,
                            "source loser\npath",
                            SpecificityTier::TypedLexical,
                        ),
                        reason: SelectionLoserReason::UnresolvedSurvivor,
                        evidence: Counted {
                            total: 2,
                            shown: 1,
                            omitted: 1,
                            items: vec![comparison],
                        },
                    }],
                },
            }
        );
    }

    #[test]
    fn source_view_mapper_preserves_all_internal_kinds_and_exact_messages() {
        for (runtime_kind, expected_kind, message) in [
            (
                deckmaste_english_v2::parser::InternalFailureKind::ValidatedRootDidNotMaterialize,
                InternalFailureKind::ValidatedRootDidNotMaterialize,
                "source validated\nroot message",
            ),
            (
                deckmaste_english_v2::parser::InternalFailureKind::SelectionConfiguration,
                InternalFailureKind::SelectionConfiguration,
                "source selection\rconfiguration message",
            ),
            (
                deckmaste_english_v2::parser::InternalFailureKind::OwnershipInspection,
                InternalFailureKind::OwnershipInspection,
                "source ownership inspection message",
            ),
        ] {
            let source = fixture_trace_source(FixtureTraceOutcome::Internal {
                kind: runtime_kind,
                message: message.to_owned(),
            });
            let report =
                DiagnosticReport::from_probe_source("source text", "source context", &source);
            let DiagnosticOutcome::InternalFailure(failure) = report.trace.outcome else {
                panic!("source fixture outcome was remapped");
            };
            assert_eq!(failure.kind, expected_kind);
            assert_eq!(failure.message, message);
        }
    }

    #[test]
    fn real_parser_trace_source_view_delegates_every_public_section_and_outcome() {
        fn assert_delegated<T>(actual: &Bounded<T>, delegated: BoundedSource<'_, T>) {
            assert_eq!(delegated.total, actual.total());
            assert_eq!(delegated.shown, actual.shown());
            assert_eq!(delegated.omitted, actual.omitted());
            assert!(std::ptr::eq(delegated.items, actual.items()));
        }

        let runtime = trace("You gains X life.", "Probe Card", 1);
        let source = &runtime
            as &dyn TraceSourceView<
                Token = deckmaste_english_v2::parser::ScannedToken,
                ChartItem = deckmaste_english_v2::parser::ChartItem,
                ForestNode = deckmaste_english_v2::parser::ForestNode,
                CheckedRejection = deckmaste_english_v2::parser::CheckedCompletionRejection,
                MaterializedCandidate = deckmaste_english_v2::parser::MaterializedCandidateInfo,
                MaterializationCycle = deckmaste_english_v2::parser::MaterializationCycle,
                Selection = deckmaste_english_v2::parser::BoundedSelectionDecision,
            >;
        assert_delegated(runtime.tokens(), source.tokens());
        assert_delegated(runtime.chart(), source.chart());
        assert_delegated(runtime.forest(), source.forest());
        assert_delegated(runtime.accepted_roots(), source.accepted_roots());
        assert_delegated(
            runtime.checked_completion_rejections(),
            source.checked_completion_rejections(),
        );
        assert_delegated(
            runtime.materialized_candidates(),
            source.materialized_candidates(),
        );
        assert_delegated(
            runtime.materialization_cycles(),
            source.materialization_cycles(),
        );
        let TraceOutcomeSource::ParseFailure {
            start,
            end,
            expectations,
        } = source.outcome()
        else {
            panic!("real outcome accessor was not delegated");
        };
        let BoundedParseOutcome::ParseFailure(actual) = runtime.outcome() else {
            panic!("fixture must remain a parse failure");
        };
        assert_eq!((start, end), (actual.span().start, actual.span().end));
        assert_eq!(expectations.total, actual.expectations().total());
        assert_eq!(expectations.shown, actual.expectations().shown());
        assert_eq!(expectations.omitted, actual.expectations().omitted());
        assert_eq!(expectations.items, actual.expectations().items());
    }
}
