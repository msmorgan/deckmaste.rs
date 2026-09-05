use std::cmp::Ordering;
use std::io::Write;

use anyhow::Context;
use deckmaste_english_v2::parser::Bounded;
use deckmaste_english_v2::parser::BoundedParseOutcome;
use deckmaste_english_v2::parser::BoundedSelectionCandidate as RuntimeSelectionCandidate;
use deckmaste_english_v2::parser::BoundedSelectionComparison as RuntimeSelectionComparison;
use deckmaste_english_v2::parser::BoundedSelectionDecision as RuntimeSelectionDecision;
use deckmaste_english_v2::parser::ByteMismatchScope as RuntimeByteMismatchScope;
use deckmaste_english_v2::parser::ChartItem as RuntimeChartItem;
use deckmaste_english_v2::parser::CheckedCompletionRejection as RuntimeCheckedRejection;
use deckmaste_english_v2::parser::ExpectationInfo as RuntimeExpectation;
use deckmaste_english_v2::parser::FamilyIdentityChild as RuntimeFamilyIdentityChild;
use deckmaste_english_v2::parser::ForestChild as RuntimeForestChild;
use deckmaste_english_v2::parser::ForestFamily as RuntimeForestFamily;
use deckmaste_english_v2::parser::ForestNode as RuntimeForestNode;
use deckmaste_english_v2::parser::InternalFailureKind as RuntimeInternalFailureKind;
use deckmaste_english_v2::parser::InvalidSpanKind as RuntimeInvalidSpanKind;
use deckmaste_english_v2::parser::LexicalClaim as RuntimeLexicalClaim;
use deckmaste_english_v2::parser::LexicalProvenanceKind as RuntimeProvenanceKind;
use deckmaste_english_v2::parser::MaterializationCycle as RuntimeMaterializationCycle;
use deckmaste_english_v2::parser::MaterializedCandidateInfo as RuntimeMaterializedCandidate;
use deckmaste_english_v2::parser::NonterminalCategory;
use deckmaste_english_v2::parser::OwnershipFailure as RuntimeOwnershipFailure;
use deckmaste_english_v2::parser::OwnershipSummary as RuntimeOwnershipSummary;
use deckmaste_english_v2::parser::ParserTrace;
use deckmaste_english_v2::parser::ScannerMatch as RuntimeScannerMatch;
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
    root: String,
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
    scanner_matches: Counted<ScannerMatch>,
    selected_lexical_claims: Counted<DiagnosticLexicalClaim>,
    ownership: Option<DiagnosticOwnership>,
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

trait ScannerMatchSource {
    fn start(&self) -> usize;
    fn end(&self) -> usize;
    fn terminal_name_v1(&self) -> &str;
    fn value_label_v1(&self) -> &str;
}

trait LexicalClaimSource {
    fn start(&self) -> usize;
    fn end(&self) -> usize;
    fn kind(&self) -> RuntimeProvenanceKind;
    fn stable_owner_id(&self) -> &str;
    fn semantic_summary(&self) -> &str;
}

trait OwnershipSummarySource {
    fn covered(&self) -> bool;
    fn claims(&self) -> usize;
    fn claimed_bytes(&self) -> usize;
    fn form_literal_claims(&self) -> usize;
    fn form_literal_bytes(&self) -> usize;
    fn vocab_claims(&self) -> usize;
    fn vocab_bytes(&self) -> usize;
    fn lexeme_claims(&self) -> usize;
    fn lexeme_bytes(&self) -> usize;
    fn codec_claims(&self) -> usize;
    fn codec_bytes(&self) -> usize;
    fn identity_claims(&self) -> usize;
    fn identity_bytes(&self) -> usize;
    fn gap_spans(&self) -> usize;
    fn gap_bytes(&self) -> usize;
    fn overlap_spans(&self) -> usize;
    fn overlap_bytes(&self) -> usize;
    fn synthetic_claims(&self) -> usize;
    fn provenance_plan_mismatches(&self) -> usize;
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
    type ScannerMatch: ScannerMatchSource;
    type LexicalClaim: LexicalClaimSource;
    type OwnershipSummary: OwnershipSummarySource;
    type ChartItem: ChartItemSource;
    type ForestNode: ForestNodeSource;
    type CheckedRejection: CheckedRejectionSource;
    type MaterializedCandidate: MaterializedCandidateSource;
    type MaterializationCycle: MaterializationCycleSource;
    type Selection: SelectionDecisionSource;

    fn root_name(&self) -> &'static str;
    fn scanner_matches(&self) -> BoundedSource<'_, Self::ScannerMatch>;
    fn selected_lexical_claims(&self) -> BoundedSource<'_, Self::LexicalClaim>;
    fn ownership(&self) -> Option<&Self::OwnershipSummary>;
    fn ownership_failures(&self) -> &[RuntimeOwnershipFailure];
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

impl ScannerMatchSource for RuntimeScannerMatch {
    fn start(&self) -> usize {
        RuntimeScannerMatch::start(self)
    }

    fn end(&self) -> usize {
        RuntimeScannerMatch::end(self)
    }

    fn terminal_name_v1(&self) -> &str {
        RuntimeScannerMatch::terminal_name_v1(self)
    }

    fn value_label_v1(&self) -> &str {
        RuntimeScannerMatch::value_label_v1(self)
    }
}

impl LexicalClaimSource for RuntimeLexicalClaim {
    fn start(&self) -> usize {
        RuntimeLexicalClaim::span(self).start
    }

    fn end(&self) -> usize {
        RuntimeLexicalClaim::span(self).end
    }

    fn kind(&self) -> RuntimeProvenanceKind {
        RuntimeLexicalClaim::kind(self)
    }

    fn stable_owner_id(&self) -> &str {
        RuntimeLexicalClaim::stable_owner_id(self)
    }

    fn semantic_summary(&self) -> &str {
        RuntimeLexicalClaim::semantic_summary(self)
    }
}

macro_rules! ownership_summary_source_accessors {
    ($($name:ident),* $(,)?) => {
        $(
            fn $name(&self) -> usize {
                RuntimeOwnershipSummary::$name(self)
            }
        )*
    };
}

impl OwnershipSummarySource for RuntimeOwnershipSummary {
    fn covered(&self) -> bool {
        RuntimeOwnershipSummary::covered(self)
    }

    ownership_summary_source_accessors!(
        claims,
        claimed_bytes,
        form_literal_claims,
        form_literal_bytes,
        vocab_claims,
        vocab_bytes,
        lexeme_claims,
        lexeme_bytes,
        codec_claims,
        codec_bytes,
        identity_claims,
        identity_bytes,
        gap_spans,
        gap_bytes,
        overlap_spans,
        overlap_bytes,
        synthetic_claims,
        provenance_plan_mismatches,
    );
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

impl<V> TraceSourceView for ParserTrace<V> {
    type ScannerMatch = RuntimeScannerMatch;
    type LexicalClaim = RuntimeLexicalClaim;
    type OwnershipSummary = RuntimeOwnershipSummary;
    type ChartItem = RuntimeChartItem;
    type ForestNode = RuntimeForestNode;
    type CheckedRejection = RuntimeCheckedRejection;
    type MaterializedCandidate = RuntimeMaterializedCandidate;
    type MaterializationCycle = RuntimeMaterializationCycle;
    type Selection = RuntimeSelectionDecision;

    fn root_name(&self) -> &'static str {
        ParserTrace::root_name(self)
    }

    fn scanner_matches(&self) -> BoundedSource<'_, Self::ScannerMatch> {
        runtime_bounded(ParserTrace::scanner_matches(self))
    }

    fn selected_lexical_claims(&self) -> BoundedSource<'_, Self::LexicalClaim> {
        runtime_bounded(ParserTrace::selected_lexical_claims(self))
    }

    fn ownership(&self) -> Option<&Self::OwnershipSummary> {
        ParserTrace::ownership(self)
    }

    fn ownership_failures(&self) -> &[RuntimeOwnershipFailure] {
        ParserTrace::ownership_failures(self)
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
struct ScannerMatch {
    start: usize,
    end: usize,
    terminal_name_v1: String,
    value_label_v1: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
struct DiagnosticLexicalClaim {
    start: usize,
    end: usize,
    kind: DiagnosticProvenanceKind,
    stable_owner_id: String,
    semantic_summary: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
enum DiagnosticProvenanceKind {
    FormLiteral,
    Vocab,
    Lexeme,
    Codec,
    Identity,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
struct DiagnosticOwnership {
    covered: bool,
    claims: usize,
    claimed_bytes: usize,
    form_literal_claims: usize,
    form_literal_bytes: usize,
    vocab_claims: usize,
    vocab_bytes: usize,
    lexeme_claims: usize,
    lexeme_bytes: usize,
    codec_claims: usize,
    codec_bytes: usize,
    identity_claims: usize,
    identity_bytes: usize,
    gap_spans: usize,
    gap_bytes: usize,
    overlap_spans: usize,
    overlap_bytes: usize,
    synthetic_claims: usize,
    provenance_plan_mismatches: usize,
    failures: Vec<DiagnosticOwnershipFailure>,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
enum DiagnosticOwnershipFailure {
    Gap {
        start: usize,
        end: usize,
    },
    Overlap {
        left_start: usize,
        left_end: usize,
        right_start: usize,
        right_end: usize,
        overlap_start: usize,
        overlap_end: usize,
    },
    InvalidSpan {
        start: usize,
        end: usize,
        span_kind: DiagnosticInvalidSpanKind,
    },
    Synthetic {
        start: usize,
        end: usize,
    },
    ProvenancePlanMismatch {
        index: usize,
        parsed: String,
        rendered: String,
    },
    ByteMismatch {
        scope: DiagnosticByteMismatchScope,
        expected: String,
        actual: String,
    },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
enum DiagnosticInvalidSpanKind {
    OutOfBounds,
    NonUtf8Boundary,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Serialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
enum DiagnosticByteMismatchScope {
    WholeRender,
    ClaimSlice { index: usize },
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
    Identity,
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
    FrameComplement,
    PrepositionalPhrase,
    PrepositionalComplement,
    ExistentialPredicateAdjunct,
    PredicateAdjunct,
    PredicateAdjunctPredicate,
    PurposePredicateAdjunct,
    DurationPredicateAdjunct,
    PrepositionalPredicateAdjunct,
    PrepositionalPredicateAdjunctHost,
    FrequencyPredicateAdjunct,
    MannerPredicateAdjunct,
    PreposedPredicateAdjunct,
    PreposedPredicateAdjunctPredicate,
    BareLocativeProform,
    Ability,
    AbilityBody,
    ModalMode,
    LevelBand,
    LevelRange,
    BoundedLevelRange,
    OpenLevelRange,
    FrequencyReference,
    PlainFrequency,
    ComparativeFrequency,
    ModeMarker,
    BulletMarker,
    WeightedMarker,
    FlavorWordModeMarker,
    AdditionalCostMark,
    ModalHead,
    DashHead,
    SentenceHead,
    ActivationCostComponent,
    SymbolRun,
    Loyalty,
    CostClause,
    CostSymbol,
    LoyaltyValue,
    TriggerPrefix,
    DocumentBlock,
    OracleText,
    Sentence,
    Clause,
    ClauseAttachment,
    StartingWithAttachment,
    PreposedAs,
    PreposedIf,
    PreposedIfPredicate,
    PostposedIf,
    PostposedIfPredicate,
    PostposedUnless,
    PostposedUnlessPredicate,
    PreposedAsLongAs,
    PreposedAsLongAsPredicate,
    PreposedWhile,
    PreposedWhilePredicate,
    PreposedUntil,
    PreposedUntilPredicate,
    ThenSequence,
    ThenPredicateSequence,
    BareThenPredicateSequence,
    AdditionalCostBody,
    AdditionalCost,
    AdditionalCostPredicateBody,
    AdditionalCostFiniteBody,
    CastingRestriction,
    OnlyIfRestriction,
    OnlyDuringRestriction,
    AlternativePredicate,
    WithoutGerundObjectPredicate,
    ControlledCostAction,
    CostComparisonPredicate,
    RestrictionTurn,
    ActionRestrictionPredicate,
    Predicate,
    BarePredicate,
    CoordinatedPredicate,
    BareCoordinatedPredicate,
    PredicateCoordination,
    BarePredicateCoordination,
    PredicativeComplement,
    PredicativeStatus,
    PassivePredicate,
    DistributionPhrase,
    DistributionRecipient,
    PredicativeAdjectiveComplement,
    PredicativeColorComplement,
    PredicativeDesignationComplement,
    PredicativeStatusComplement,
    PredicativeParticipialAdjectiveComplement,
    ParticipialAdjective,
    PredicativeDeclaredParticipleComplement,
    ParticipialByComplement,
    ParticipialExceptByComplement,
    PredicativeAbilityComplement,
    PredicativePowerToughnessComplement,
    PowerToughnessValue,
    PredicativeScalarComplement,
    PredicativeScalarEqualityComplement,
    KeywordLineItem,
    KeywordCost,
    KeywordCostSeparator,
    KeywordManaCost,
    KeywordManaClauseCost,
    SpacedKeywordCost,
    DashedKeywordCost,
    KeywordQuality,
    KeywordSubject,
    KeywordSubjectModifier,
    KeywordCostPredicate,
    BareKeywordLineItem,
    CostedKeywordLineItem,
    AmountKeywordLineItem,
    AmountCostKeywordLineItem,
    QualifiedKeywordLineItem,
    QualityCostKeywordLineItem,
    SubjectKeywordLineItem,
    KeywordQualityCoordination,
    KeywordLine,
    LabelledAbility,
    BlockLabel,
    LabelTerm,
    AbilityWordLabelTerm,
    FlavorWordLabelTerm,
    ChapterLabel,
    BareCopularPredicate,
    PassiveMovementPredicate,
    PassiveOrientationPredicate,
    DeclaredObjectPassivePredicate,
    DeclaredTransitivePassiveFromPredicate,
    BarePassivePredicate,
    ObjectDistributionRecipient,
    ChosenDistributionPhrase,
    EvenDistributionPhrase,
    ObjectInfinitivePredicate,
    InfinitiveComplement,
    RequirementPredicate,
    TransitiveRequirementPredicate,
    AsThoughPredicate,
    CounterfactualClause,
    IrrealisCopularClause,
    PossessiveComplement,
    KeywordPossessiveComplement,
    OrderedPredicate,
    InsteadPredicate,
    FiniteClause,
    ClauseCoordination,
    CoordinatedClause,
    WhereClauseCategory,
    Subject,
    Object,
    NounPhrase,
    UnqualifiedReference,
    CountReference,
    MassNoun,
    MannerReference,
    ScalarReference,
    ScalarValue,
    ScalarEquality,
    CounterKind,
    ManaAmount,
    PowerToughnessAdjustment,
    PowerToughnessAdjustmentMagnitude,
    PositiveCounterMagnitude,
    NegativeCounterMagnitude,
    Head,
    NominalModifier,
    NegativeNominalModifier,
    CoordinatedNominalModifier,
    SerialAndModifierTail,
    CoordinationMember,
    NominalCoordination,
    Nominal,
    Determinative,
    SingularSelector,
    PluralSelector,
    FullNounPhraseCoordination,
    LocativeNounPhraseCoordination,
    PartitiveSelection,
    BareLocative,
    BareLocativeNoun,
    EdgeOfPhrase,
    ScalarThreshold,
    ScalarMeasure,
    ScalarComparison,
    CountComparison,
    ScalarMeasureAssignedValue,
    ScalarMeasureValue,
    GrantedKeywordLine,
    DegreeMeasure,
    PostmodifiedReference,
    VerbPhrase,
    LexicalVerbPhrase,
    PredicativeComplementLexicalVerbPhrase,
    IntransitiveLexicalVerbPhrase,
    TransitiveLexicalVerbPhrase,
    GetPowerToughnessLexicalVerbPhrase,
    MeasureComplementLexicalVerbPhrase,
    PossessiveOwner,
    Possessive,
    Amount,
    CardinalQuantity,
    ConditionClause,
    FiniteCondition,
    QuotedAbility,
    QuotedBlock,
    CommonNounChoice,
    AuxiliaryHead,
    AuxiliaryObjectGapRelativeClause,
    AuxiliaryPredicate,
    BareNegativeObjectGapRelativeClause,
    ContractedPerfectObjectGapRelativeClause,
    ContractedPerfectAdjunctObjectGapRelativeClause,
    DeclaredToObjectPassivePredicate,
    DurationPhrase,
    FiniteCopularPredicate,
    FinitePassivePredicate,
    FixedDurationPhrase,
    ManaCoordination,
    ManaPhrase,
    SubjectGapRelativeClause,
    ModalPassiveSubjectGapRelativeClause,
    FiniteSubjectGapRelativeClause,
    ModalSubjectGapRelativeClause,
    CopularSubjectGapRelativeClause,
    ObjectAmountLexicalVerbPhrase,
    ObjectForObjectLexicalVerbPhrase,
    ObjectGapRelativeClause,
    ObjectIntoObjectLexicalVerbPhrase,
    ObjectWithObjectLexicalVerbPhrase,
    OrScalarDegreePhrase,
    PositiveObjectGapRelativeClause,
    PostposedForAsLongAsClause,
    PostposedWhileClause,
    PredicativeFaceOrientation,
    PredicativeNominalComplement,
    ScalarDegreePhrase,
    SingleScalarDegreePhrase,
    TemporalEndpoint,
    ThirdPersonNegativeObjectGapRelativeClause,
    UntilDurationPhrase,
    WithObjectLexicalVerbPhrase,
    AndManaCoordination,
    AndOrManaCoordination,
    OnlyTemporalClauseRestriction,
    OrManaCoordination,
    PostposedAsLongAs,
    PostposedAsLongAsPredicate,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(transparent)]
struct TerminalKind(String);

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
    pub(super) fn from_probe<V>(text: &str, context: &str, trace: &ParserTrace<V>) -> Self {
        Self::from_probe_source(text, context, trace)
    }

    fn from_probe_source<Source: TraceSourceView>(
        text: &str,
        context: &str,
        trace: &Source,
    ) -> Self {
        Self {
            schema_version: 2,
            root: trace.root_name().to_owned(),
            source: DiagnosticSource::Probe {
                text: text.to_owned(),
                context: context.to_owned(),
            },
            trace: DiagnosticTrace::from_source(trace),
        }
    }

    pub(super) fn from_corpus<V>(unit: &super::corpus::CorpusUnit, trace: &ParserTrace<V>) -> Self {
        Self {
            schema_version: 2,
            root: trace.root_name().to_owned(),
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

    pub(super) const fn accepted(&self) -> bool {
        matches!(&self.trace.outcome, DiagnosticOutcome::Selected(_))
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
            scanner_matches: Counted::from_source(trace.scanner_matches(), |scanner_match| {
                ScannerMatch {
                    start: scanner_match.start(),
                    end: scanner_match.end(),
                    terminal_name_v1: scanner_match.terminal_name_v1().to_owned(),
                    value_label_v1: scanner_match.value_label_v1().to_owned(),
                }
            }),
            selected_lexical_claims: Counted::from_source(
                trace.selected_lexical_claims(),
                |claim| DiagnosticLexicalClaim {
                    start: claim.start(),
                    end: claim.end(),
                    kind: provenance_kind(claim.kind()),
                    stable_owner_id: claim.stable_owner_id().to_owned(),
                    semantic_summary: claim.semantic_summary().to_owned(),
                },
            ),
            ownership: trace
                .ownership()
                .map(|summary| diagnostic_ownership(summary, trace.ownership_failures())),
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

fn provenance_kind(kind: RuntimeProvenanceKind) -> DiagnosticProvenanceKind {
    match kind {
        RuntimeProvenanceKind::FormLiteral => DiagnosticProvenanceKind::FormLiteral,
        RuntimeProvenanceKind::Vocab => DiagnosticProvenanceKind::Vocab,
        RuntimeProvenanceKind::Lexeme => DiagnosticProvenanceKind::Lexeme,
        RuntimeProvenanceKind::Codec => DiagnosticProvenanceKind::Codec,
        RuntimeProvenanceKind::Identity => DiagnosticProvenanceKind::Identity,
    }
}

fn diagnostic_ownership<Source: OwnershipSummarySource>(
    summary: &Source,
    failures: &[RuntimeOwnershipFailure],
) -> DiagnosticOwnership {
    DiagnosticOwnership {
        covered: summary.covered(),
        claims: summary.claims(),
        claimed_bytes: summary.claimed_bytes(),
        form_literal_claims: summary.form_literal_claims(),
        form_literal_bytes: summary.form_literal_bytes(),
        vocab_claims: summary.vocab_claims(),
        vocab_bytes: summary.vocab_bytes(),
        lexeme_claims: summary.lexeme_claims(),
        lexeme_bytes: summary.lexeme_bytes(),
        codec_claims: summary.codec_claims(),
        codec_bytes: summary.codec_bytes(),
        identity_claims: summary.identity_claims(),
        identity_bytes: summary.identity_bytes(),
        gap_spans: summary.gap_spans(),
        gap_bytes: summary.gap_bytes(),
        overlap_spans: summary.overlap_spans(),
        overlap_bytes: summary.overlap_bytes(),
        synthetic_claims: summary.synthetic_claims(),
        provenance_plan_mismatches: summary.provenance_plan_mismatches(),
        failures: failures.iter().map(ownership_failure).collect(),
    }
}

fn ownership_failure(failure: &RuntimeOwnershipFailure) -> DiagnosticOwnershipFailure {
    match failure {
        RuntimeOwnershipFailure::Gap { span } => DiagnosticOwnershipFailure::Gap {
            start: span.start,
            end: span.end,
        },
        RuntimeOwnershipFailure::Overlap {
            left,
            right,
            overlap,
        } => DiagnosticOwnershipFailure::Overlap {
            left_start: left.start,
            left_end: left.end,
            right_start: right.start,
            right_end: right.end,
            overlap_start: overlap.start,
            overlap_end: overlap.end,
        },
        RuntimeOwnershipFailure::InvalidSpan { span, kind } => {
            DiagnosticOwnershipFailure::InvalidSpan {
                start: span.start,
                end: span.end,
                span_kind: match kind {
                    RuntimeInvalidSpanKind::OutOfBounds => DiagnosticInvalidSpanKind::OutOfBounds,
                    RuntimeInvalidSpanKind::NonUtf8Boundary => {
                        DiagnosticInvalidSpanKind::NonUtf8Boundary
                    }
                },
            }
        }
        RuntimeOwnershipFailure::Synthetic { span } => DiagnosticOwnershipFailure::Synthetic {
            start: span.start,
            end: span.end,
        },
        RuntimeOwnershipFailure::ProvenancePlanMismatch {
            index,
            parsed,
            rendered,
        } => DiagnosticOwnershipFailure::ProvenancePlanMismatch {
            index: *index,
            parsed: parsed.clone(),
            rendered: rendered.clone(),
        },
        RuntimeOwnershipFailure::ByteMismatch {
            scope,
            expected,
            actual,
        } => DiagnosticOwnershipFailure::ByteMismatch {
            scope: match scope {
                RuntimeByteMismatchScope::WholeRender => DiagnosticByteMismatchScope::WholeRender,
                RuntimeByteMismatchScope::ClaimSlice { index } => {
                    DiagnosticByteMismatchScope::ClaimSlice { index: *index }
                }
            },
            expected: expected.clone(),
            actual: actual.clone(),
        },
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

#[expect(
    clippy::too_many_lines,
    reason = "the source-to-wire nonterminal mapping is deliberately exhaustive and literal"
)]
fn nonterminal(kind: NonterminalCategory) -> NonterminalKind {
    match kind {
        NonterminalCategory::FrameComplement => NonterminalKind::FrameComplement,
        NonterminalCategory::PrepositionalPhrase => NonterminalKind::PrepositionalPhrase,
        NonterminalCategory::PrepositionalComplement => NonterminalKind::PrepositionalComplement,
        NonterminalCategory::ExistentialPredicateAdjunct => {
            NonterminalKind::ExistentialPredicateAdjunct
        }
        NonterminalCategory::PredicateAdjunct => NonterminalKind::PredicateAdjunct,
        NonterminalCategory::PredicateAdjunctPredicate => {
            NonterminalKind::PredicateAdjunctPredicate
        }
        NonterminalCategory::PurposePredicateAdjunct => NonterminalKind::PurposePredicateAdjunct,
        NonterminalCategory::DurationPredicateAdjunct => NonterminalKind::DurationPredicateAdjunct,
        NonterminalCategory::PrepositionalPredicateAdjunct => {
            NonterminalKind::PrepositionalPredicateAdjunct
        }
        NonterminalCategory::PrepositionalPredicateAdjunctHost => {
            NonterminalKind::PrepositionalPredicateAdjunctHost
        }
        NonterminalCategory::FrequencyPredicateAdjunct => {
            NonterminalKind::FrequencyPredicateAdjunct
        }
        NonterminalCategory::MannerPredicateAdjunct => NonterminalKind::MannerPredicateAdjunct,
        NonterminalCategory::PreposedPredicateAdjunct => NonterminalKind::PreposedPredicateAdjunct,
        NonterminalCategory::PreposedPredicateAdjunctPredicate => {
            NonterminalKind::PreposedPredicateAdjunctPredicate
        }
        NonterminalCategory::BareLocativeProform => NonterminalKind::BareLocativeProform,
        NonterminalCategory::Ability => NonterminalKind::Ability,
        NonterminalCategory::AbilityBody => NonterminalKind::AbilityBody,
        NonterminalCategory::ModalMode => NonterminalKind::ModalMode,
        NonterminalCategory::LevelBand => NonterminalKind::LevelBand,
        NonterminalCategory::LevelRange => NonterminalKind::LevelRange,
        NonterminalCategory::BoundedLevelRange => NonterminalKind::BoundedLevelRange,
        NonterminalCategory::OpenLevelRange => NonterminalKind::OpenLevelRange,
        NonterminalCategory::FrequencyReference => NonterminalKind::FrequencyReference,
        NonterminalCategory::PlainFrequency => NonterminalKind::PlainFrequency,
        NonterminalCategory::ComparativeFrequency => NonterminalKind::ComparativeFrequency,
        NonterminalCategory::ModeMarker => NonterminalKind::ModeMarker,
        NonterminalCategory::BulletMarker => NonterminalKind::BulletMarker,
        NonterminalCategory::WeightedMarker => NonterminalKind::WeightedMarker,
        NonterminalCategory::FlavorWordModeMarker => NonterminalKind::FlavorWordModeMarker,
        NonterminalCategory::AdditionalCostMark => NonterminalKind::AdditionalCostMark,
        NonterminalCategory::ModalHead => NonterminalKind::ModalHead,
        NonterminalCategory::DashHead => NonterminalKind::DashHead,
        NonterminalCategory::SentenceHead => NonterminalKind::SentenceHead,
        NonterminalCategory::ActivationCostComponent => NonterminalKind::ActivationCostComponent,
        NonterminalCategory::SymbolRun => NonterminalKind::SymbolRun,
        NonterminalCategory::Loyalty => NonterminalKind::Loyalty,
        NonterminalCategory::CostClause => NonterminalKind::CostClause,
        NonterminalCategory::CostSymbol => NonterminalKind::CostSymbol,
        NonterminalCategory::LoyaltyValue => NonterminalKind::LoyaltyValue,
        NonterminalCategory::TriggerPrefix => NonterminalKind::TriggerPrefix,
        NonterminalCategory::DocumentBlock => NonterminalKind::DocumentBlock,
        NonterminalCategory::OracleText => NonterminalKind::OracleText,
        NonterminalCategory::Sentence => NonterminalKind::Sentence,
        NonterminalCategory::Clause => NonterminalKind::Clause,
        NonterminalCategory::ClauseAttachment => NonterminalKind::ClauseAttachment,
        NonterminalCategory::StartingWithAttachment => NonterminalKind::StartingWithAttachment,
        NonterminalCategory::PreposedAs => NonterminalKind::PreposedAs,
        NonterminalCategory::PreposedIf => NonterminalKind::PreposedIf,
        NonterminalCategory::PreposedIfPredicate => NonterminalKind::PreposedIfPredicate,
        NonterminalCategory::PostposedIf => NonterminalKind::PostposedIf,
        NonterminalCategory::PostposedIfPredicate => NonterminalKind::PostposedIfPredicate,
        NonterminalCategory::PostposedUnless => NonterminalKind::PostposedUnless,
        NonterminalCategory::PostposedUnlessPredicate => NonterminalKind::PostposedUnlessPredicate,
        NonterminalCategory::PreposedAsLongAs => NonterminalKind::PreposedAsLongAs,
        NonterminalCategory::PreposedAsLongAsPredicate => {
            NonterminalKind::PreposedAsLongAsPredicate
        }
        NonterminalCategory::ThenSequence => NonterminalKind::ThenSequence,
        NonterminalCategory::ThenPredicateSequence => NonterminalKind::ThenPredicateSequence,
        NonterminalCategory::BareThenPredicateSequence => {
            NonterminalKind::BareThenPredicateSequence
        }
        NonterminalCategory::AdditionalCostBody => NonterminalKind::AdditionalCostBody,
        NonterminalCategory::AdditionalCost => NonterminalKind::AdditionalCost,
        NonterminalCategory::AdditionalCostPredicateBody => {
            NonterminalKind::AdditionalCostPredicateBody
        }
        NonterminalCategory::AdditionalCostFiniteBody => NonterminalKind::AdditionalCostFiniteBody,
        NonterminalCategory::CastingRestriction => NonterminalKind::CastingRestriction,
        NonterminalCategory::OnlyIfRestriction => NonterminalKind::OnlyIfRestriction,
        NonterminalCategory::OnlyDuringRestriction => NonterminalKind::OnlyDuringRestriction,
        NonterminalCategory::AlternativePredicate => NonterminalKind::AlternativePredicate,
        NonterminalCategory::WithoutGerundObjectPredicate => {
            NonterminalKind::WithoutGerundObjectPredicate
        }
        NonterminalCategory::ControlledCostAction => NonterminalKind::ControlledCostAction,
        NonterminalCategory::CostComparisonPredicate => NonterminalKind::CostComparisonPredicate,
        NonterminalCategory::RestrictionTurn => NonterminalKind::RestrictionTurn,
        NonterminalCategory::ActionRestrictionPredicate => {
            NonterminalKind::ActionRestrictionPredicate
        }
        NonterminalCategory::Predicate => NonterminalKind::Predicate,
        NonterminalCategory::BarePredicate => NonterminalKind::BarePredicate,
        NonterminalCategory::CoordinatedPredicate => NonterminalKind::CoordinatedPredicate,
        NonterminalCategory::BareCoordinatedPredicate => NonterminalKind::BareCoordinatedPredicate,
        NonterminalCategory::PredicateCoordination => NonterminalKind::PredicateCoordination,
        NonterminalCategory::BarePredicateCoordination => {
            NonterminalKind::BarePredicateCoordination
        }
        NonterminalCategory::PredicativeComplement => NonterminalKind::PredicativeComplement,
        NonterminalCategory::PredicativeStatus => NonterminalKind::PredicativeStatus,
        NonterminalCategory::PassivePredicate => NonterminalKind::PassivePredicate,
        NonterminalCategory::DistributionPhrase => NonterminalKind::DistributionPhrase,
        NonterminalCategory::DistributionRecipient => NonterminalKind::DistributionRecipient,
        NonterminalCategory::PredicativeAdjectiveComplement => {
            NonterminalKind::PredicativeAdjectiveComplement
        }
        NonterminalCategory::PredicativeColorComplement => {
            NonterminalKind::PredicativeColorComplement
        }
        NonterminalCategory::PredicativeDesignationComplement => {
            NonterminalKind::PredicativeDesignationComplement
        }
        NonterminalCategory::PredicativeStatusComplement => {
            NonterminalKind::PredicativeStatusComplement
        }
        NonterminalCategory::PredicativeParticipialAdjectiveComplement => {
            NonterminalKind::PredicativeParticipialAdjectiveComplement
        }
        NonterminalCategory::ParticipialAdjective => NonterminalKind::ParticipialAdjective,
        NonterminalCategory::PredicativeDeclaredParticipleComplement => {
            NonterminalKind::PredicativeDeclaredParticipleComplement
        }
        NonterminalCategory::ParticipialByComplement => NonterminalKind::ParticipialByComplement,
        NonterminalCategory::ParticipialExceptByComplement => {
            NonterminalKind::ParticipialExceptByComplement
        }
        NonterminalCategory::PredicativePowerToughnessComplement => {
            NonterminalKind::PredicativePowerToughnessComplement
        }
        NonterminalCategory::PowerToughnessValue => NonterminalKind::PowerToughnessValue,
        NonterminalCategory::PredicativeScalarComplement => {
            NonterminalKind::PredicativeScalarComplement
        }
        NonterminalCategory::PredicativeScalarEqualityComplement => {
            NonterminalKind::PredicativeScalarEqualityComplement
        }
        NonterminalCategory::KeywordLineItem => NonterminalKind::KeywordLineItem,
        NonterminalCategory::KeywordCost => NonterminalKind::KeywordCost,
        NonterminalCategory::KeywordCostSeparator => NonterminalKind::KeywordCostSeparator,
        NonterminalCategory::KeywordManaCost => NonterminalKind::KeywordManaCost,
        NonterminalCategory::KeywordManaClauseCost => NonterminalKind::KeywordManaClauseCost,
        NonterminalCategory::SpacedKeywordCost => NonterminalKind::SpacedKeywordCost,
        NonterminalCategory::DashedKeywordCost => NonterminalKind::DashedKeywordCost,
        NonterminalCategory::KeywordQuality => NonterminalKind::KeywordQuality,
        NonterminalCategory::KeywordSubject => NonterminalKind::KeywordSubject,
        NonterminalCategory::KeywordSubjectModifier => NonterminalKind::KeywordSubjectModifier,
        NonterminalCategory::KeywordCostPredicate => NonterminalKind::KeywordCostPredicate,
        NonterminalCategory::BareKeywordLineItem => NonterminalKind::BareKeywordLineItem,
        NonterminalCategory::CostedKeywordLineItem => NonterminalKind::CostedKeywordLineItem,
        NonterminalCategory::AmountKeywordLineItem => NonterminalKind::AmountKeywordLineItem,
        NonterminalCategory::AmountCostKeywordLineItem => {
            NonterminalKind::AmountCostKeywordLineItem
        }
        NonterminalCategory::QualifiedKeywordLineItem => NonterminalKind::QualifiedKeywordLineItem,
        NonterminalCategory::SubjectKeywordLineItem => NonterminalKind::SubjectKeywordLineItem,
        NonterminalCategory::KeywordQualityCoordination => {
            NonterminalKind::KeywordQualityCoordination
        }
        NonterminalCategory::KeywordLine => NonterminalKind::KeywordLine,
        NonterminalCategory::LabelledAbility => NonterminalKind::LabelledAbility,
        NonterminalCategory::BlockLabel => NonterminalKind::BlockLabel,
        NonterminalCategory::LabelTerm => NonterminalKind::LabelTerm,
        NonterminalCategory::AbilityWordLabelTerm => NonterminalKind::AbilityWordLabelTerm,
        NonterminalCategory::FlavorWordLabelTerm => NonterminalKind::FlavorWordLabelTerm,
        NonterminalCategory::ChapterLabel => NonterminalKind::ChapterLabel,
        NonterminalCategory::BareCopularPredicate => NonterminalKind::BareCopularPredicate,
        NonterminalCategory::PassiveMovementPredicate => NonterminalKind::PassiveMovementPredicate,
        NonterminalCategory::PassiveOrientationPredicate => {
            NonterminalKind::PassiveOrientationPredicate
        }
        NonterminalCategory::DeclaredObjectPassivePredicate => {
            NonterminalKind::DeclaredObjectPassivePredicate
        }
        NonterminalCategory::DeclaredTransitivePassiveFromPredicate => {
            NonterminalKind::DeclaredTransitivePassiveFromPredicate
        }
        NonterminalCategory::BarePassivePredicate => NonterminalKind::BarePassivePredicate,
        NonterminalCategory::ObjectDistributionRecipient => {
            NonterminalKind::ObjectDistributionRecipient
        }
        NonterminalCategory::ChosenDistributionPhrase => NonterminalKind::ChosenDistributionPhrase,
        NonterminalCategory::EvenDistributionPhrase => NonterminalKind::EvenDistributionPhrase,
        NonterminalCategory::ObjectInfinitivePredicate => {
            NonterminalKind::ObjectInfinitivePredicate
        }
        NonterminalCategory::InfinitiveComplement => NonterminalKind::InfinitiveComplement,
        NonterminalCategory::RequirementPredicate => NonterminalKind::RequirementPredicate,
        NonterminalCategory::TransitiveRequirementPredicate => {
            NonterminalKind::TransitiveRequirementPredicate
        }
        NonterminalCategory::AsThoughPredicate => NonterminalKind::AsThoughPredicate,
        NonterminalCategory::CounterfactualClause => NonterminalKind::CounterfactualClause,
        NonterminalCategory::IrrealisCopularClause => NonterminalKind::IrrealisCopularClause,
        NonterminalCategory::PossessiveComplement => NonterminalKind::PossessiveComplement,
        NonterminalCategory::KeywordPossessiveComplement => {
            NonterminalKind::KeywordPossessiveComplement
        }
        NonterminalCategory::OrderedPredicate => NonterminalKind::OrderedPredicate,
        NonterminalCategory::InsteadPredicate => NonterminalKind::InsteadPredicate,
        NonterminalCategory::FiniteClause => NonterminalKind::FiniteClause,
        NonterminalCategory::ClauseCoordination => NonterminalKind::ClauseCoordination,
        NonterminalCategory::CoordinatedClause => NonterminalKind::CoordinatedClause,
        NonterminalCategory::WhereClauseCategory => NonterminalKind::WhereClauseCategory,
        NonterminalCategory::Subject => NonterminalKind::Subject,
        NonterminalCategory::Object => NonterminalKind::Object,
        NonterminalCategory::NounPhrase => NonterminalKind::NounPhrase,
        NonterminalCategory::UnqualifiedReference => NonterminalKind::UnqualifiedReference,
        NonterminalCategory::CountReference => NonterminalKind::CountReference,
        NonterminalCategory::MassNoun => NonterminalKind::MassNoun,
        NonterminalCategory::MannerReference => NonterminalKind::MannerReference,
        NonterminalCategory::ScalarReference => NonterminalKind::ScalarReference,
        NonterminalCategory::ScalarValue => NonterminalKind::ScalarValue,
        NonterminalCategory::ScalarEquality => NonterminalKind::ScalarEquality,
        NonterminalCategory::CounterKind => NonterminalKind::CounterKind,
        NonterminalCategory::ManaAmount => NonterminalKind::ManaAmount,
        NonterminalCategory::PowerToughnessAdjustment => NonterminalKind::PowerToughnessAdjustment,
        NonterminalCategory::PowerToughnessAdjustmentMagnitude => {
            NonterminalKind::PowerToughnessAdjustmentMagnitude
        }
        NonterminalCategory::PositiveCounterMagnitude => NonterminalKind::PositiveCounterMagnitude,
        NonterminalCategory::NegativeCounterMagnitude => NonterminalKind::NegativeCounterMagnitude,
        NonterminalCategory::Head => NonterminalKind::Head,
        NonterminalCategory::NominalModifier => NonterminalKind::NominalModifier,
        NonterminalCategory::NegativeNominalModifier => NonterminalKind::NegativeNominalModifier,
        NonterminalCategory::CoordinatedNominalModifier => {
            NonterminalKind::CoordinatedNominalModifier
        }
        NonterminalCategory::SerialAndModifierTail => NonterminalKind::SerialAndModifierTail,
        NonterminalCategory::CoordinationMember => NonterminalKind::CoordinationMember,
        NonterminalCategory::NominalCoordination => NonterminalKind::NominalCoordination,
        NonterminalCategory::Nominal => NonterminalKind::Nominal,
        NonterminalCategory::Determinative => NonterminalKind::Determinative,
        NonterminalCategory::SingularSelector => NonterminalKind::SingularSelector,
        NonterminalCategory::FullNounPhraseCoordination => {
            NonterminalKind::FullNounPhraseCoordination
        }
        NonterminalCategory::PartitiveSelection => NonterminalKind::PartitiveSelection,
        NonterminalCategory::BareLocative => NonterminalKind::BareLocative,
        NonterminalCategory::BareLocativeNoun => NonterminalKind::BareLocativeNoun,
        NonterminalCategory::EdgeOfPhrase => NonterminalKind::EdgeOfPhrase,
        NonterminalCategory::ScalarThreshold => NonterminalKind::ScalarThreshold,
        NonterminalCategory::ScalarMeasure => NonterminalKind::ScalarMeasure,
        NonterminalCategory::ScalarComparison => NonterminalKind::ScalarComparison,
        NonterminalCategory::CountComparison => NonterminalKind::CountComparison,
        NonterminalCategory::ScalarMeasureAssignedValue => {
            NonterminalKind::ScalarMeasureAssignedValue
        }
        NonterminalCategory::ScalarMeasureValue => NonterminalKind::ScalarMeasureValue,
        NonterminalCategory::GrantedKeywordLine => NonterminalKind::GrantedKeywordLine,
        NonterminalCategory::DegreeMeasure => NonterminalKind::DegreeMeasure,
        NonterminalCategory::PostmodifiedReference => NonterminalKind::PostmodifiedReference,
        NonterminalCategory::LocativeNounPhraseCoordination => {
            NonterminalKind::LocativeNounPhraseCoordination
        }
        NonterminalCategory::VerbPhrase => NonterminalKind::VerbPhrase,
        NonterminalCategory::LexicalVerbPhrase => NonterminalKind::LexicalVerbPhrase,
        NonterminalCategory::PredicativeComplementLexicalVerbPhrase => {
            NonterminalKind::PredicativeComplementLexicalVerbPhrase
        }
        NonterminalCategory::IntransitiveLexicalVerbPhrase => {
            NonterminalKind::IntransitiveLexicalVerbPhrase
        }
        NonterminalCategory::TransitiveLexicalVerbPhrase => {
            NonterminalKind::TransitiveLexicalVerbPhrase
        }
        NonterminalCategory::GetPowerToughnessLexicalVerbPhrase => {
            NonterminalKind::GetPowerToughnessLexicalVerbPhrase
        }
        NonterminalCategory::MeasureComplementLexicalVerbPhrase => {
            NonterminalKind::MeasureComplementLexicalVerbPhrase
        }
        NonterminalCategory::PossessiveOwner => NonterminalKind::PossessiveOwner,
        NonterminalCategory::Possessive => NonterminalKind::Possessive,
        NonterminalCategory::Amount => NonterminalKind::Amount,
        NonterminalCategory::CardinalQuantity => NonterminalKind::CardinalQuantity,
        NonterminalCategory::ConditionClause => NonterminalKind::ConditionClause,
        NonterminalCategory::FiniteCondition => NonterminalKind::FiniteCondition,
        NonterminalCategory::QuotedAbility => NonterminalKind::QuotedAbility,
        NonterminalCategory::QuotedBlock => NonterminalKind::QuotedBlock,
        NonterminalCategory::CommonNounChoice => NonterminalKind::CommonNounChoice,
        NonterminalCategory::AuxiliaryHead => NonterminalKind::AuxiliaryHead,
        NonterminalCategory::AuxiliaryObjectGapRelativeClause => {
            NonterminalKind::AuxiliaryObjectGapRelativeClause
        }
        NonterminalCategory::AuxiliaryPredicate => NonterminalKind::AuxiliaryPredicate,
        NonterminalCategory::BareNegativeObjectGapRelativeClause => {
            NonterminalKind::BareNegativeObjectGapRelativeClause
        }
        NonterminalCategory::ContractedPerfectObjectGapRelativeClause => {
            NonterminalKind::ContractedPerfectObjectGapRelativeClause
        }
        NonterminalCategory::ContractedPerfectAdjunctObjectGapRelativeClause => {
            NonterminalKind::ContractedPerfectAdjunctObjectGapRelativeClause
        }
        NonterminalCategory::DeclaredToObjectPassivePredicate => {
            NonterminalKind::DeclaredToObjectPassivePredicate
        }
        NonterminalCategory::DurationPhrase => NonterminalKind::DurationPhrase,
        NonterminalCategory::FiniteCopularPredicate => NonterminalKind::FiniteCopularPredicate,
        NonterminalCategory::FinitePassivePredicate => NonterminalKind::FinitePassivePredicate,
        NonterminalCategory::FixedDurationPhrase => NonterminalKind::FixedDurationPhrase,
        NonterminalCategory::ManaCoordination => NonterminalKind::ManaCoordination,
        NonterminalCategory::ManaPhrase => NonterminalKind::ManaPhrase,
        NonterminalCategory::SubjectGapRelativeClause => NonterminalKind::SubjectGapRelativeClause,
        NonterminalCategory::ModalPassiveSubjectGapRelativeClause => {
            NonterminalKind::ModalPassiveSubjectGapRelativeClause
        }
        NonterminalCategory::CopularSubjectGapRelativeClause => {
            NonterminalKind::CopularSubjectGapRelativeClause
        }
        NonterminalCategory::ObjectAmountLexicalVerbPhrase => {
            NonterminalKind::ObjectAmountLexicalVerbPhrase
        }
        NonterminalCategory::ObjectForObjectLexicalVerbPhrase => {
            NonterminalKind::ObjectForObjectLexicalVerbPhrase
        }
        NonterminalCategory::ObjectGapRelativeClause => NonterminalKind::ObjectGapRelativeClause,
        NonterminalCategory::ObjectIntoObjectLexicalVerbPhrase => {
            NonterminalKind::ObjectIntoObjectLexicalVerbPhrase
        }
        NonterminalCategory::OrScalarDegreePhrase => NonterminalKind::OrScalarDegreePhrase,
        NonterminalCategory::PositiveObjectGapRelativeClause => {
            NonterminalKind::PositiveObjectGapRelativeClause
        }
        NonterminalCategory::PostposedForAsLongAsClause => {
            NonterminalKind::PostposedForAsLongAsClause
        }
        NonterminalCategory::PostposedWhileClause => NonterminalKind::PostposedWhileClause,
        NonterminalCategory::PredicativeFaceOrientation => {
            NonterminalKind::PredicativeFaceOrientation
        }
        NonterminalCategory::PredicativeNominalComplement => {
            NonterminalKind::PredicativeNominalComplement
        }
        NonterminalCategory::ScalarDegreePhrase => NonterminalKind::ScalarDegreePhrase,
        NonterminalCategory::SingleScalarDegreePhrase => NonterminalKind::SingleScalarDegreePhrase,
        NonterminalCategory::TemporalEndpoint => NonterminalKind::TemporalEndpoint,
        NonterminalCategory::UntilDurationPhrase => NonterminalKind::UntilDurationPhrase,
        NonterminalCategory::OrManaCoordination => NonterminalKind::OrManaCoordination,
        NonterminalCategory::PostposedAsLongAs => NonterminalKind::PostposedAsLongAs,
        NonterminalCategory::PostposedAsLongAsPredicate => {
            NonterminalKind::PostposedAsLongAsPredicate
        }
        NonterminalCategory::PreposedWhile => NonterminalKind::PreposedWhile,
        NonterminalCategory::PreposedWhilePredicate => NonterminalKind::PreposedWhilePredicate,
        NonterminalCategory::PreposedUntil => NonterminalKind::PreposedUntil,
        NonterminalCategory::PreposedUntilPredicate => NonterminalKind::PreposedUntilPredicate,
        NonterminalCategory::PredicativeAbilityComplement => {
            NonterminalKind::PredicativeAbilityComplement
        }
        NonterminalCategory::QualityCostKeywordLineItem => {
            NonterminalKind::QualityCostKeywordLineItem
        }
        NonterminalCategory::FiniteSubjectGapRelativeClause => {
            NonterminalKind::FiniteSubjectGapRelativeClause
        }
        NonterminalCategory::ModalSubjectGapRelativeClause => {
            NonterminalKind::ModalSubjectGapRelativeClause
        }
        NonterminalCategory::ThirdPersonNegativeObjectGapRelativeClause => {
            NonterminalKind::ThirdPersonNegativeObjectGapRelativeClause
        }
        NonterminalCategory::AndManaCoordination => NonterminalKind::AndManaCoordination,
        NonterminalCategory::AndOrManaCoordination => NonterminalKind::AndOrManaCoordination,
        NonterminalCategory::OnlyTemporalClauseRestriction => {
            NonterminalKind::OnlyTemporalClauseRestriction
        }
        NonterminalCategory::WithObjectLexicalVerbPhrase => {
            NonterminalKind::WithObjectLexicalVerbPhrase
        }
        NonterminalCategory::ObjectWithObjectLexicalVerbPhrase => {
            NonterminalKind::ObjectWithObjectLexicalVerbPhrase
        }
    }
}

fn terminal(kind: TerminalClass) -> TerminalKind {
    let name = kind.to_string().replace(' ', "_");
    TerminalKind(name.strip_prefix("open_").unwrap_or(&name).to_owned())
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
        RuntimeSpecificityTier::Identity => SpecificityTier::Identity,
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
            "English v2 diagnostic schema_version={} root={} source_kind=probe text={} context={}",
            report.schema_version,
            report.root,
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
            "English v2 diagnostic schema_version={} root={} source_kind=corpus id={} card={} face={} side={} text={} context={}",
            report.schema_version,
            report.root,
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
    write_counted("scanner_matches", &trace.scanner_matches, output)?;
    write_items("scanner_match", &trace.scanner_matches.items, output)?;
    write_counted(
        "selected_lexical_claims",
        &trace.selected_lexical_claims,
        output,
    )?;
    write_items(
        "selected_lexical_claim",
        &trace.selected_lexical_claims.items,
        output,
    )?;
    render_ownership(trace.ownership.as_ref(), output)?;
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

fn render_ownership(
    ownership: Option<&DiagnosticOwnership>,
    output: &mut dyn Write,
) -> anyhow::Result<()> {
    let Some(ownership) = ownership else {
        return writeln!(output, "ownership none")
            .context("writing English-v2 diagnostic absent ownership");
    };
    writeln!(
        output,
        "ownership covered={} claims={} claimed_bytes={} form_literal_claims={} form_literal_bytes={} vocab_claims={} vocab_bytes={} lexeme_claims={} lexeme_bytes={} codec_claims={} codec_bytes={} identity_claims={} identity_bytes={} gap_spans={} gap_bytes={} overlap_spans={} overlap_bytes={} synthetic_claims={} provenance_plan_mismatches={}",
        ownership.covered,
        ownership.claims,
        ownership.claimed_bytes,
        ownership.form_literal_claims,
        ownership.form_literal_bytes,
        ownership.vocab_claims,
        ownership.vocab_bytes,
        ownership.lexeme_claims,
        ownership.lexeme_bytes,
        ownership.codec_claims,
        ownership.codec_bytes,
        ownership.identity_claims,
        ownership.identity_bytes,
        ownership.gap_spans,
        ownership.gap_bytes,
        ownership.overlap_spans,
        ownership.overlap_bytes,
        ownership.synthetic_claims,
        ownership.provenance_plan_mismatches,
    )
    .context("writing English-v2 diagnostic ownership summary")?;
    write_items("ownership.failure", &ownership.failures, output)
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
    OwnershipInspection,
}

#[cfg(test)]
#[expect(
    clippy::too_many_lines,
    reason = "one complete schema fixture keeps every nested bounded section visible"
)]
pub(super) fn fixture_report(outcome: FixtureOutcome) -> DiagnosticReport {
    fn fixture_diagnostic_ownership() -> DiagnosticOwnership {
        DiagnosticOwnership {
            covered: false,
            claims: 5,
            claimed_bytes: 20,
            form_literal_claims: 1,
            form_literal_bytes: 4,
            vocab_claims: 1,
            vocab_bytes: 4,
            lexeme_claims: 1,
            lexeme_bytes: 4,
            codec_claims: 1,
            codec_bytes: 4,
            identity_claims: 1,
            identity_bytes: 4,
            gap_spans: 1,
            gap_bytes: 2,
            overlap_spans: 1,
            overlap_bytes: 1,
            synthetic_claims: 1,
            provenance_plan_mismatches: 1,
            failures: vec![
                DiagnosticOwnershipFailure::Gap { start: 20, end: 22 },
                DiagnosticOwnershipFailure::Overlap {
                    left_start: 0,
                    left_end: 4,
                    right_start: 3,
                    right_end: 7,
                    overlap_start: 3,
                    overlap_end: 4,
                },
                DiagnosticOwnershipFailure::InvalidSpan {
                    start: 99,
                    end: 100,
                    span_kind: DiagnosticInvalidSpanKind::OutOfBounds,
                },
                DiagnosticOwnershipFailure::InvalidSpan {
                    start: 1,
                    end: 2,
                    span_kind: DiagnosticInvalidSpanKind::NonUtf8Boundary,
                },
                DiagnosticOwnershipFailure::Synthetic { start: 8, end: 8 },
                DiagnosticOwnershipFailure::ProvenancePlanMismatch {
                    index: 2,
                    parsed: "parsed\n\t\"\\é".to_owned(),
                    rendered: "rendered\rvalue".to_owned(),
                },
                DiagnosticOwnershipFailure::ByteMismatch {
                    scope: DiagnosticByteMismatchScope::WholeRender,
                    expected: "expected\nwhole".to_owned(),
                    actual: "actual\twhole".to_owned(),
                },
                DiagnosticOwnershipFailure::ByteMismatch {
                    scope: DiagnosticByteMismatchScope::ClaimSlice { index: 4 },
                    expected: "expected slice".to_owned(),
                    actual: "actual \\ slice".to_owned(),
                },
            ],
        }
    }

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
                    Expectation::Terminal(terminal(TerminalClass::DeclarationNoun(6))),
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
        FixtureOutcome::OwnershipInspection => {
            DiagnosticOutcome::InternalFailure(InternalFailureOutcome {
                kind: InternalFailureKind::OwnershipInspection,
                message: "ownership\tinspection failure".to_owned(),
            })
        }
    };
    let selected = matches!(&outcome, DiagnosticOutcome::Selected(_));
    DiagnosticReport {
        schema_version: 2,
        root: "Ability".to_owned(),
        source: DiagnosticSource::Probe {
            text: "line\nbreak".to_owned(),
            context: "context\rbreak".to_owned(),
        },
        trace: DiagnosticTrace {
            scanner_matches: Counted::fixture(
                2,
                vec![ScannerMatch {
                    start: 0,
                    end: 4,
                    terminal_name_v1: "terminal\nname".to_owned(),
                    value_label_v1: "value\rlabel".to_owned(),
                }],
            ),
            selected_lexical_claims: if selected {
                Counted::fixture(
                    5,
                    vec![
                        DiagnosticLexicalClaim {
                            start: 0,
                            end: 4,
                            kind: DiagnosticProvenanceKind::FormLiteral,
                            stable_owner_id: "form:\"hostile\"\\owner".to_owned(),
                            semantic_summary: "form\nsummary".to_owned(),
                        },
                        DiagnosticLexicalClaim {
                            start: 4,
                            end: 8,
                            kind: DiagnosticProvenanceKind::Vocab,
                            stable_owner_id: "vocab:tab\towner".to_owned(),
                            semantic_summary: "vocab summary".to_owned(),
                        },
                        DiagnosticLexicalClaim {
                            start: 8,
                            end: 12,
                            kind: DiagnosticProvenanceKind::Lexeme,
                            stable_owner_id: "lexeme:creature_subtype/Spírit/singular".to_owned(),
                            semantic_summary: "lexeme summary".to_owned(),
                        },
                        DiagnosticLexicalClaim {
                            start: 12,
                            end: 16,
                            kind: DiagnosticProvenanceKind::Codec,
                            stable_owner_id: "codec:SignedNumber".to_owned(),
                            semantic_summary: "codec summary".to_owned(),
                        },
                        DiagnosticLexicalClaim {
                            start: 16,
                            end: 20,
                            kind: DiagnosticProvenanceKind::Identity,
                            stable_owner_id: "identity:SelfReference".to_owned(),
                            semantic_summary: "identity summary".to_owned(),
                        },
                    ],
                )
            } else {
                Counted::fixture(0, Vec::new())
            },
            ownership: selected.then(fixture_diagnostic_ownership),
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
    struct FixtureScannerMatch {
        start: usize,
        end: usize,
        terminal: String,
        value: String,
    }

    impl ScannerMatchSource for FixtureScannerMatch {
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
    struct FixtureLexicalClaim {
        start: usize,
        end: usize,
        kind: deckmaste_english_v2::parser::LexicalProvenanceKind,
        stable_owner_id: String,
        semantic_summary: String,
    }

    impl LexicalClaimSource for FixtureLexicalClaim {
        fn start(&self) -> usize {
            self.start
        }

        fn end(&self) -> usize {
            self.end
        }

        fn kind(&self) -> deckmaste_english_v2::parser::LexicalProvenanceKind {
            self.kind
        }

        fn stable_owner_id(&self) -> &str {
            &self.stable_owner_id
        }

        fn semantic_summary(&self) -> &str {
            &self.semantic_summary
        }
    }

    #[derive(Debug, Clone)]
    struct FixtureOwnershipSummary {
        covered: bool,
        values: [usize; 18],
    }

    impl OwnershipSummarySource for FixtureOwnershipSummary {
        fn covered(&self) -> bool {
            self.covered
        }
        fn claims(&self) -> usize {
            self.values[0]
        }
        fn claimed_bytes(&self) -> usize {
            self.values[1]
        }
        fn form_literal_claims(&self) -> usize {
            self.values[2]
        }
        fn form_literal_bytes(&self) -> usize {
            self.values[3]
        }
        fn vocab_claims(&self) -> usize {
            self.values[4]
        }
        fn vocab_bytes(&self) -> usize {
            self.values[5]
        }
        fn lexeme_claims(&self) -> usize {
            self.values[6]
        }
        fn lexeme_bytes(&self) -> usize {
            self.values[7]
        }
        fn codec_claims(&self) -> usize {
            self.values[8]
        }
        fn codec_bytes(&self) -> usize {
            self.values[9]
        }
        fn identity_claims(&self) -> usize {
            self.values[10]
        }
        fn identity_bytes(&self) -> usize {
            self.values[11]
        }
        fn gap_spans(&self) -> usize {
            self.values[12]
        }
        fn gap_bytes(&self) -> usize {
            self.values[13]
        }
        fn overlap_spans(&self) -> usize {
            self.values[14]
        }
        fn overlap_bytes(&self) -> usize {
            self.values[15]
        }
        fn synthetic_claims(&self) -> usize {
            self.values[16]
        }
        fn provenance_plan_mismatches(&self) -> usize {
            self.values[17]
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
        Selected {
            rendered: String,
            selection: FixtureSelection,
        },
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
        scanner_matches: FixtureBounded<FixtureScannerMatch>,
        selected_lexical_claims: FixtureBounded<FixtureLexicalClaim>,
        ownership: Option<FixtureOwnershipSummary>,
        ownership_failures: Vec<deckmaste_english_v2::parser::OwnershipFailure>,
        chart: FixtureBounded<FixtureChartItem>,
        forest: FixtureBounded<FixtureForestNode>,
        roots: FixtureBounded<usize>,
        rejections: FixtureBounded<FixtureRejection>,
        candidates: FixtureBounded<FixtureMaterializedCandidate>,
        cycles: FixtureBounded<FixtureCycle>,
        outcome: FixtureTraceOutcome,
    }

    impl TraceSourceView for FixtureTraceSource {
        type ScannerMatch = FixtureScannerMatch;
        type LexicalClaim = FixtureLexicalClaim;
        type OwnershipSummary = FixtureOwnershipSummary;
        type ChartItem = FixtureChartItem;
        type ForestNode = FixtureForestNode;
        type CheckedRejection = FixtureRejection;
        type MaterializedCandidate = FixtureMaterializedCandidate;
        type MaterializationCycle = FixtureCycle;
        type Selection = FixtureSelection;

        fn root_name(&self) -> &'static str {
            "Ability"
        }

        fn scanner_matches(&self) -> BoundedSource<'_, Self::ScannerMatch> {
            self.scanner_matches.view()
        }

        fn selected_lexical_claims(&self) -> BoundedSource<'_, Self::LexicalClaim> {
            self.selected_lexical_claims.view()
        }

        fn ownership(&self) -> Option<&Self::OwnershipSummary> {
            self.ownership.as_ref()
        }

        fn ownership_failures(&self) -> &[deckmaste_english_v2::parser::OwnershipFailure] {
            &self.ownership_failures
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
                FixtureTraceOutcome::Selected {
                    rendered,
                    selection,
                } => TraceOutcomeSource::Selected {
                    rendered,
                    selection,
                },
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
            scanner_matches: FixtureBounded::new(
                2,
                vec![FixtureScannerMatch {
                    start: 2,
                    end: 8,
                    terminal: "source terminal\nname".to_owned(),
                    value: "source value".to_owned(),
                }],
            ),
            selected_lexical_claims: FixtureBounded::new(0, Vec::new()),
            ownership: None,
            ownership_failures: Vec::new(),
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

    fn fixture_selected_ownership_source(limit: usize, covered: bool) -> FixtureTraceSource {
        use deckmaste_english_v2::parser::ByteMismatchScope;
        use deckmaste_english_v2::parser::InvalidSpanKind;
        use deckmaste_english_v2::parser::LexicalProvenanceKind;
        use deckmaste_english_v2::parser::OwnershipFailure;
        use deckmaste_english_v2::parser::TextSpan;

        let mut selection = fixture_selection_source();
        selection.selected = selection.candidates.items.first().cloned();
        selection.resolution = deckmaste_english_v2::parser::SelectionResolution::Specificity;
        let mut source = fixture_trace_source(FixtureTraceOutcome::Selected {
            rendered: "source selected\nrender".to_owned(),
            selection,
        });
        let claims = [
            (LexicalProvenanceKind::FormLiteral, "form:one"),
            (LexicalProvenanceKind::Vocab, "vocab:two"),
            (
                LexicalProvenanceKind::Lexeme,
                "lexeme:creature_subtype/Spírit/singular",
            ),
            (LexicalProvenanceKind::Codec, "codec:four"),
            (LexicalProvenanceKind::Identity, "identity:five"),
        ];
        source.selected_lexical_claims = FixtureBounded::new(
            claims.len(),
            claims
                .into_iter()
                .enumerate()
                .take(limit.min(claims.len()))
                .map(|(index, (kind, owner))| FixtureLexicalClaim {
                    start: index * 4,
                    end: index * 4 + 4,
                    kind,
                    stable_owner_id: owner.to_owned(),
                    semantic_summary: format!("summary {index}\n\t\"\\"),
                })
                .collect(),
        );
        source.ownership = Some(FixtureOwnershipSummary {
            covered,
            values: if covered {
                [
                    31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 0, 0, 0, 0, 0, 0,
                ]
            } else {
                [
                    101, 102, 103, 104, 105, 106, 107, 108, 109, 110, 111, 112, 113, 114, 115, 116,
                    117, 118,
                ]
            },
        });
        source.ownership_failures = if covered {
            Vec::new()
        } else {
            vec![
                OwnershipFailure::Gap {
                    span: TextSpan { start: 20, end: 22 },
                },
                OwnershipFailure::Overlap {
                    left: TextSpan { start: 0, end: 4 },
                    right: TextSpan { start: 3, end: 7 },
                    overlap: TextSpan { start: 3, end: 4 },
                },
                OwnershipFailure::InvalidSpan {
                    span: TextSpan {
                        start: 99,
                        end: 100,
                    },
                    kind: InvalidSpanKind::OutOfBounds,
                },
                OwnershipFailure::InvalidSpan {
                    span: TextSpan { start: 1, end: 2 },
                    kind: InvalidSpanKind::NonUtf8Boundary,
                },
                OwnershipFailure::Synthetic {
                    span: TextSpan { start: 8, end: 8 },
                },
                OwnershipFailure::ProvenancePlanMismatch {
                    index: 2,
                    parsed: "parsed\n\t\"\\é".to_owned(),
                    rendered: "rendered\rvalue".to_owned(),
                },
                OwnershipFailure::ByteMismatch {
                    scope: ByteMismatchScope::WholeRender,
                    expected: "expected\nwhole".to_owned(),
                    actual: "actual\twhole".to_owned(),
                },
                OwnershipFailure::ByteMismatch {
                    scope: ByteMismatchScope::ClaimSlice { index: 4 },
                    expected: "expected slice".to_owned(),
                    actual: "actual \\ slice".to_owned(),
                },
            ]
        };
        source
    }

    fn expected_fixture_diagnostic_ownership(covered: bool) -> DiagnosticOwnership {
        DiagnosticOwnership {
            covered,
            claims: if covered { 31 } else { 101 },
            claimed_bytes: if covered { 32 } else { 102 },
            form_literal_claims: if covered { 33 } else { 103 },
            form_literal_bytes: if covered { 34 } else { 104 },
            vocab_claims: if covered { 35 } else { 105 },
            vocab_bytes: if covered { 36 } else { 106 },
            lexeme_claims: if covered { 37 } else { 107 },
            lexeme_bytes: if covered { 38 } else { 108 },
            codec_claims: if covered { 39 } else { 109 },
            codec_bytes: if covered { 40 } else { 110 },
            identity_claims: if covered { 41 } else { 111 },
            identity_bytes: if covered { 42 } else { 112 },
            gap_spans: if covered { 0 } else { 113 },
            gap_bytes: if covered { 0 } else { 114 },
            overlap_spans: if covered { 0 } else { 115 },
            overlap_bytes: if covered { 0 } else { 116 },
            synthetic_claims: if covered { 0 } else { 117 },
            provenance_plan_mismatches: if covered { 0 } else { 118 },
            failures: if covered {
                Vec::new()
            } else {
                vec![
                    DiagnosticOwnershipFailure::Gap { start: 20, end: 22 },
                    DiagnosticOwnershipFailure::Overlap {
                        left_start: 0,
                        left_end: 4,
                        right_start: 3,
                        right_end: 7,
                        overlap_start: 3,
                        overlap_end: 4,
                    },
                    DiagnosticOwnershipFailure::InvalidSpan {
                        start: 99,
                        end: 100,
                        span_kind: DiagnosticInvalidSpanKind::OutOfBounds,
                    },
                    DiagnosticOwnershipFailure::InvalidSpan {
                        start: 1,
                        end: 2,
                        span_kind: DiagnosticInvalidSpanKind::NonUtf8Boundary,
                    },
                    DiagnosticOwnershipFailure::Synthetic { start: 8, end: 8 },
                    DiagnosticOwnershipFailure::ProvenancePlanMismatch {
                        index: 2,
                        parsed: "parsed\n\t\"\\é".to_owned(),
                        rendered: "rendered\rvalue".to_owned(),
                    },
                    DiagnosticOwnershipFailure::ByteMismatch {
                        scope: DiagnosticByteMismatchScope::WholeRender,
                        expected: "expected\nwhole".to_owned(),
                        actual: "actual\twhole".to_owned(),
                    },
                    DiagnosticOwnershipFailure::ByteMismatch {
                        scope: DiagnosticByteMismatchScope::ClaimSlice { index: 4 },
                        expected: "expected slice".to_owned(),
                        actual: "actual \\ slice".to_owned(),
                    },
                ]
            },
        }
    }

    fn parser() -> Parser {
        crate::english_v2::parser_from_builtin_v2().unwrap()
    }

    fn trace(text: &str, context: &str, limit: usize) -> ParserTrace {
        let context = ParseContext::new(
            context,
            false,
            deckmaste_construction_core::macro_def::Onset::Consonant,
        )
        .expect("fixture context is valid");
        parser().trace(text, &context, TraceLimits::new(limit))
    }

    fn assert_counts<T, U>(runtime: &Bounded<T>, projected: &Counted<U>) {
        assert_eq!(projected.total, runtime.total());
        assert_eq!(projected.shown, runtime.shown());
        assert_eq!(projected.omitted, runtime.omitted());
        assert_eq!(projected.items.len(), runtime.items().len());
    }

    fn assert_no_selected_ownership(report: &DiagnosticReport) {
        assert_eq!(report.trace.selected_lexical_claims.total, 0);
        assert_eq!(report.trace.selected_lexical_claims.shown, 0);
        assert_eq!(report.trace.selected_lexical_claims.omitted, 0);
        assert!(report.trace.selected_lexical_claims.items.is_empty());
        assert!(report.trace.ownership.is_none());
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
            assert_eq!(projected.schema_version, 2);
            assert_eq!(projected.source.kind(), SourceKind::Probe);
            assert_eq!(projected.source.text(), text);
            assert_eq!(projected.source.context(), context);
            assert_counts(runtime.scanner_matches(), &projected.trace.scanner_matches);
            assert_counts(
                runtime.selected_lexical_claims(),
                &projected.trace.selected_lexical_claims,
            );
            let expected_ownership = runtime
                .ownership()
                .map(|summary| diagnostic_ownership(summary, runtime.ownership_failures()));
            assert_eq!(projected.trace.ownership, expected_ownership);
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
                .scanner_matches()
                .items()
                .iter()
                .zip(&projected.trace.scanner_matches.items)
            {
                assert_eq!(dto.start, actual.start());
                assert_eq!(dto.end, actual.end());
                assert_eq!(dto.terminal_name_v1, actual.terminal_name_v1());
                assert_eq!(dto.value_label_v1, actual.value_label_v1());
            }
            for (actual, dto) in runtime
                .selected_lexical_claims()
                .items()
                .iter()
                .zip(&projected.trace.selected_lexical_claims.items)
            {
                assert_eq!(dto.start, actual.span().start);
                assert_eq!(dto.end, actual.span().end);
                assert_eq!(dto.kind, provenance_kind(actual.kind()));
                assert_eq!(dto.stable_owner_id, actual.stable_owner_id());
                assert_eq!(dto.semantic_summary, actual.semantic_summary());
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
    fn schema_v2_shared_mapper_preserves_bounded_claims_complete_ownership_and_every_failure() {
        let expected_kinds = [
            DiagnosticProvenanceKind::FormLiteral,
            DiagnosticProvenanceKind::Vocab,
            DiagnosticProvenanceKind::Lexeme,
            DiagnosticProvenanceKind::Codec,
            DiagnosticProvenanceKind::Identity,
        ];
        for covered in [true, false] {
            for limit in [0, 1, 5, 6] {
                let source = fixture_selected_ownership_source(limit, covered);
                let report = DiagnosticReport::from_probe_source(
                    "hostile\ntext\t\"\\é",
                    "hostile\rcontext",
                    &source,
                );
                assert_eq!(report.schema_version, 2);
                assert_eq!(report.trace.selected_lexical_claims.total, 5);
                assert_eq!(report.trace.selected_lexical_claims.shown, limit.min(5),);
                assert_eq!(
                    report.trace.selected_lexical_claims.omitted,
                    5 - limit.min(5),
                );
                assert_eq!(
                    report
                        .trace
                        .selected_lexical_claims
                        .items
                        .iter()
                        .map(|claim| claim.kind)
                        .collect::<Vec<_>>(),
                    expected_kinds[..limit.min(5)],
                );
                assert_eq!(
                    report.trace.ownership,
                    Some(expected_fixture_diagnostic_ownership(covered)),
                );

                let first = render_to_vec(&report, true).unwrap();
                let second = render_to_vec(&report, true).unwrap();
                assert_eq!(first, second);
                assert_eq!(first.last(), Some(&b'\n'));
                let value: Value = serde_json::from_slice(&first).unwrap();
                assert!(value["trace"].get("tokens").is_none());
                assert!(value["trace"].get("scanner_matches").is_some());
                assert!(value["trace"].get("selected_lexical_claims").is_some());
                let round_trip: DiagnosticReport = serde_json::from_slice(&first).unwrap();
                assert_eq!(round_trip, report);
                if !covered {
                    let failures = value["trace"]["ownership"]["failures"]
                        .as_array()
                        .expect("failure array");
                    assert_eq!(
                        failures
                            .iter()
                            .map(|failure| failure["kind"].as_str().unwrap())
                            .collect::<Vec<_>>(),
                        [
                            "gap",
                            "overlap",
                            "invalid_span",
                            "invalid_span",
                            "synthetic",
                            "provenance_plan_mismatch",
                            "byte_mismatch",
                            "byte_mismatch",
                        ],
                    );
                    assert_eq!(failures[2]["span_kind"], "out_of_bounds");
                    assert_eq!(failures[3]["span_kind"], "non_utf8_boundary");
                    assert_eq!(failures[6]["scope"]["kind"], "whole_render");
                    assert_eq!(failures[7]["scope"]["kind"], "claim_slice");
                    assert_eq!(failures[7]["scope"]["index"], 4);
                }

                let human = String::from_utf8(render_to_vec(&report, false).unwrap()).unwrap();
                for exact_prefix in [
                    "scanner_matches shown=",
                    "selected_lexical_claims shown=",
                    "ownership covered=",
                ] {
                    assert!(human.lines().any(|line| line.starts_with(exact_prefix)));
                }
                assert!(!human.lines().any(|line| line.starts_with("tokens ")));
                assert!(!human.contains("hostile\ntext"));
            }
        }
    }

    #[test]
    fn projection_keeps_overlapping_spans_nested_omissions_rejections_and_failure_expectations() {
        let overlap = trace("You gain X life.", "You", usize::MAX);
        let overlap = DiagnosticReport::from_probe("You gain X life.", "You", &overlap);
        let spans: Vec<_> = overlap
            .trace
            .scanner_matches
            .items
            .iter()
            .map(|scanner_match| (scanner_match.start, scanner_match.end))
            .collect();
        assert!(
            spans
                .iter()
                .any(|span| spans.iter().filter(|other| *other == span).count() > 1),
            "scanner matches must retain overlapping successful spans: {spans:?}"
        );

        let bounded = trace("Destroy any target.", "Probe Card", 1);
        let bounded = DiagnosticReport::from_probe("Destroy any target.", "Probe Card", &bounded);
        assert!(bounded.trace.forest_nodes.shown <= 1);
        assert_eq!(
            bounded.trace.forest_nodes.omitted,
            bounded.trace.forest_nodes.total - bounded.trace.forest_nodes.shown,
        );
        for node in &bounded.trace.forest_nodes.items {
            assert!(node.families.shown <= 1);
            assert_eq!(
                node.families.omitted,
                node.families.total - node.families.shown
            );
            for family in &node.families.items {
                assert!(family.children.shown <= 1);
                assert_eq!(
                    family.children.omitted,
                    family.children.total - family.children.shown,
                );
            }
        }

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
    fn schema_v2_json_and_human_render_every_outcome_and_nested_bounded_section_deterministically()
    {
        for outcome in [
            FixtureOutcome::Selected,
            FixtureOutcome::ParseFailure,
            FixtureOutcome::UnresolvedAmbiguity,
            FixtureOutcome::ValidatedRootDidNotMaterialize,
            FixtureOutcome::SelectionConfiguration,
            FixtureOutcome::OwnershipInspection,
        ] {
            let report = fixture_report(outcome);
            let first = render_to_vec(&report, true).unwrap();
            let second = render_to_vec(&report, true).unwrap();
            assert_eq!(first, second);
            let value: Value = serde_json::from_slice(&first).unwrap();
            assert_eq!(value["schema_version"], 2);
            assert!(value["trace"].get("tokens").is_none());
            let round_trip: DiagnosticReport = serde_json::from_slice(&first).unwrap();
            assert_eq!(round_trip, report);

            let human = String::from_utf8(render_to_vec(&report, false).unwrap()).unwrap();
            assert!(!human.contains("..."));
            assert!(!human.contains("line\nbreak"));
            for section in [
                "scanner_matches",
                "selected_lexical_claims",
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
    fn schema_v2_pins_status_reason_provenance_and_internal_kind_spellings() {
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
            (FixtureOutcome::OwnershipInspection, "ownership_inspection"),
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
            "scanner_matches shown=1 total=2 omitted=1",
            "selected_lexical_claims shown=5 total=5 omitted=0",
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
        let scanner_rows = selected
            .lines()
            .filter(|line| line.starts_with("scanner_match index="))
            .collect::<Vec<_>>();
        assert_eq!(scanner_rows.len(), 1);
        assert!(scanner_rows[0].starts_with("scanner_match index=0 item={"));
        let claim_rows = selected
            .lines()
            .filter(|line| line.starts_with("selected_lexical_claim index="))
            .collect::<Vec<_>>();
        assert_eq!(claim_rows.len(), 5);
        for (index, row) in claim_rows.iter().enumerate() {
            assert!(row.starts_with(&format!("selected_lexical_claim index={index} item={{")));
        }
        let ownership_line = "ownership covered=false claims=5 claimed_bytes=20 form_literal_claims=1 form_literal_bytes=4 vocab_claims=1 vocab_bytes=4 lexeme_claims=1 lexeme_bytes=4 codec_claims=1 codec_bytes=4 identity_claims=1 identity_bytes=4 gap_spans=1 gap_bytes=2 overlap_spans=1 overlap_bytes=1 synthetic_claims=1 provenance_plan_mismatches=1";
        assert!(selected.lines().any(|line| line == ownership_line));
        let failure_lines = selected
            .lines()
            .filter(|line| line.starts_with("ownership.failure index="))
            .collect::<Vec<_>>();
        assert_eq!(failure_lines.len(), 8);
        for required_field in [
            "\"start\":20",
            "\"left_start\":0",
            "\"span_kind\":\"out_of_bounds\"",
            "\"span_kind\":\"non_utf8_boundary\"",
            "\"index\":2",
            "\"kind\":\"whole_render\"",
            "\"kind\":\"claim_slice\"",
            "\"expected\"",
            "\"actual\"",
        ] {
            assert!(
                failure_lines
                    .iter()
                    .any(|line| line.contains(required_field)),
                "missing ownership failure field {required_field:?}\n{selected}",
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
                        deckmaste_english_v2::parser::TerminalClass::DeclarationNoun(6),
                    ),
                    deckmaste_english_v2::parser::ExpectationInfo::Nonterminal(
                        deckmaste_english_v2::parser::NonterminalCategory::Clause,
                    ),
                ],
            ),
        });
        let report = DiagnosticReport::from_probe_source("source text", "source context", &source);
        assert_no_selected_ownership(&report);

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
                    Expectation::Terminal(terminal(TerminalClass::DeclarationNoun(6))),
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
        assert_no_selected_ownership(&report);
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
            assert_no_selected_ownership(&report);
            let DiagnosticOutcome::InternalFailure(failure) = report.trace.outcome else {
                panic!("source fixture outcome was remapped");
            };
            assert_eq!(failure.kind, expected_kind);
            assert_eq!(failure.message, message);
        }
    }

    #[test]
    fn source_view_maps_runtime_ownership_inspection_failure() {
        let context = ParseContext::new(
            "Probe Card",
            false,
            deckmaste_construction_core::macro_def::Onset::Consonant,
        )
        .expect("fixture context is valid");
        let runtime = parser().trace_with_ownership_inspection_failure_for_test(
            "Destroy target creature.",
            &context,
            TraceLimits::new(1),
        );

        let report =
            DiagnosticReport::from_probe("Destroy target creature.", "Probe Card", &runtime);
        let DiagnosticOutcome::InternalFailure(failure) = report.trace.outcome else {
            panic!("runtime ownership corruption was not mapped as an internal failure");
        };
        assert_eq!(failure.kind, InternalFailureKind::OwnershipInspection);
        assert_eq!(
            failure.message,
            "Ability root: selected lexical ownership could not be inspected"
        );
    }

    #[test]
    fn real_parser_trace_source_view_delegates_every_public_section_and_outcome() {
        fn assert_delegated<T>(actual: &Bounded<T>, delegated: BoundedSource<'_, T>) {
            assert_eq!(delegated.total, actual.total());
            assert_eq!(delegated.shown, actual.shown());
            assert_eq!(delegated.omitted, actual.omitted());
            assert!(std::ptr::eq(delegated.items, actual.items()));
        }

        let context = ParseContext::new(
            "Probe Card",
            false,
            deckmaste_construction_core::macro_def::Onset::Consonant,
        )
        .expect("fixture context is valid");
        let runtime = parser().trace_with_ownership_failure_for_test(
            "Destroy target Spirit.",
            &context,
            TraceLimits::new(1),
        );
        assert_eq!(
            runtime.ownership_failures(),
            [deckmaste_english_v2::parser::OwnershipFailure::Synthetic {
                span: deckmaste_english_v2::parser::TextSpan { start: 22, end: 22 },
            }],
        );
        assert!(!runtime.ownership_failures().is_empty());
        let source = &runtime
            as &dyn TraceSourceView<
                ScannerMatch = deckmaste_english_v2::parser::ScannerMatch,
                LexicalClaim = deckmaste_english_v2::parser::LexicalClaim,
                OwnershipSummary = deckmaste_english_v2::parser::OwnershipSummary,
                ChartItem = deckmaste_english_v2::parser::ChartItem,
                ForestNode = deckmaste_english_v2::parser::ForestNode,
                CheckedRejection = deckmaste_english_v2::parser::CheckedCompletionRejection,
                MaterializedCandidate = deckmaste_english_v2::parser::MaterializedCandidateInfo,
                MaterializationCycle = deckmaste_english_v2::parser::MaterializationCycle,
                Selection = deckmaste_english_v2::parser::BoundedSelectionDecision,
            >;
        assert_delegated(runtime.scanner_matches(), source.scanner_matches());
        assert_delegated(
            runtime.selected_lexical_claims(),
            source.selected_lexical_claims(),
        );
        assert!(std::ptr::eq(
            runtime.ownership().expect("runtime ownership"),
            source.ownership().expect("delegated ownership"),
        ));
        assert_eq!(source.ownership_failures(), runtime.ownership_failures());
        assert!(!source.ownership_failures().is_empty());
        assert!(std::ptr::eq(
            runtime.ownership_failures(),
            source.ownership_failures(),
        ));
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
        let TraceOutcomeSource::Selected {
            rendered,
            selection,
        } = source.outcome()
        else {
            panic!("real outcome accessor was not delegated");
        };
        let BoundedParseOutcome::Selected(actual) = runtime.outcome() else {
            panic!("fixture must remain selected");
        };
        assert_eq!(rendered, actual.rendered());
        assert!(std::ptr::eq(selection, actual.selection()));
    }
}
