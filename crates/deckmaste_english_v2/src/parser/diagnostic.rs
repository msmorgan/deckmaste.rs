use std::cmp::Ordering;

use super::Expectation;
use super::LexicalClaim;
use super::OwnershipFailure;
use super::OwnershipSummary;
use super::ParseError;
use super::SelectedOwnership;
use super::selection::construction_name_v1;
use crate::ast::Ability;
use crate::constructions::BuildRejection;
use crate::constructions::Construction;
use crate::constructions::GeneratedParseRoot;
use crate::constructions::RuleId;
use crate::context::ParseContext;

pub(crate) fn grammar_rule_name_v1(rule: RuleId) -> String {
    match (rule.public_construction(), rule.form_name()) {
        (Some(construction), Some(form)) => {
            format!("{} [form {form}]", construction.name())
        }
        _ => format!("{rule:?}"),
    }
}

/// The independent retention cap for each repeated diagnostic collection.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TraceLimits {
    per_collection: usize,
}

impl TraceLimits {
    #[must_use]
    pub const fn new(per_collection: usize) -> Self {
        Self { per_collection }
    }

    #[must_use]
    pub const fn per_collection(self) -> usize {
        self.per_collection
    }
}

/// A prefix-retained collection with exact, untruncated accounting.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Bounded<T> {
    total: usize,
    items: Vec<T>,
    limit: usize,
}

impl<T> Bounded<T> {
    #[must_use]
    pub(crate) fn new(limit: usize) -> Self {
        Self {
            total: 0,
            items: Vec::new(),
            limit,
        }
    }

    #[allow(
        dead_code,
        reason = "Tests exercise eager insertion alongside the production lazy seam."
    )]
    pub(crate) fn push(&mut self, item: T) {
        self.total += 1;
        if self.items.len() < self.limit {
            self.items.push(item);
        }
    }

    pub(crate) fn push_with(&mut self, make_item: impl FnOnce() -> T) {
        self.total += 1;
        if self.items.len() < self.limit {
            self.items.push(make_item());
        }
    }

    #[must_use]
    pub const fn total(&self) -> usize {
        self.total
    }
    #[must_use]
    pub fn shown(&self) -> usize {
        self.items.len()
    }
    #[must_use]
    pub fn omitted(&self) -> usize {
        self.total - self.shown()
    }
    #[must_use]
    pub fn items(&self) -> &[T] {
        &self.items
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ScannerMatch {
    pub(crate) start: usize,
    pub(crate) end: usize,
    pub(crate) terminal_name_v1: String,
    pub(crate) value_label_v1: String,
}

impl ScannerMatch {
    #[must_use]
    pub const fn start(&self) -> usize {
        self.start
    }
    #[must_use]
    pub const fn end(&self) -> usize {
        self.end
    }
    #[must_use]
    pub fn terminal_name_v1(&self) -> &str {
        &self.terminal_name_v1
    }
    #[must_use]
    pub fn value_label_v1(&self) -> &str {
        &self.value_label_v1
    }
}

#[derive(Debug, Clone)]
pub(crate) struct SemanticScannerMatchInventory<Terminal, Value> {
    entries: Vec<(usize, usize, Terminal, Value)>,
}

impl<Terminal, Value> Default for SemanticScannerMatchInventory<Terminal, Value> {
    fn default() -> Self {
        Self {
            entries: Vec::new(),
        }
    }
}

impl<Terminal: PartialEq, Value: PartialEq> SemanticScannerMatchInventory<Terminal, Value> {
    pub(crate) fn record(&mut self, start: usize, end: usize, terminal: Terminal, value: Value) {
        let entry = (start, end, terminal, value);
        if !self.entries.contains(&entry) {
            self.entries.push(entry);
        }
    }

    pub(crate) fn record_projected<SourceTerminal: Copy, SourceValue>(
        &mut self,
        start: usize,
        end: usize,
        terminal: SourceTerminal,
        value: &SourceValue,
        project_terminal: impl FnOnce(SourceTerminal) -> Terminal,
        project_value: impl FnOnce(SourceTerminal, &SourceValue) -> Value,
    ) {
        self.record(
            start,
            end,
            project_terminal(terminal),
            project_value(terminal, value),
        );
    }

    pub(crate) fn into_bounded_by(
        mut self,
        limit: usize,
        mut terminal_cmp: impl FnMut(&Terminal, &Terminal) -> Ordering,
        mut value_cmp: impl FnMut(&Value, &Value) -> Ordering,
        mut terminal_name_v1: impl FnMut(Terminal) -> String,
        mut value_label_v1: impl FnMut(&Value) -> String,
    ) -> Bounded<ScannerMatch> {
        order_bounded_prefix(&mut self.entries, limit, |left, right| {
            left.0
                .cmp(&right.0)
                .then_with(|| left.1.cmp(&right.1))
                .then_with(|| terminal_cmp(&left.2, &right.2))
                .then_with(|| value_cmp(&left.3, &right.3))
        });
        let mut scanner_matches = Bounded::new(limit);
        for (start, end, terminal, value) in self.entries {
            scanner_matches.push_with(|| ScannerMatch {
                start,
                end,
                terminal_name_v1: terminal_name_v1(terminal),
                value_label_v1: value_label_v1(&value),
            });
        }
        scanner_matches
    }
}

pub(crate) fn order_bounded_prefix<T>(
    entries: &mut [T],
    limit: usize,
    mut compare: impl FnMut(&T, &T) -> Ordering,
) {
    let retained = limit.min(entries.len());
    if retained == 0 {
        return;
    }
    if retained < entries.len() {
        entries.select_nth_unstable_by(retained, &mut compare);
    }
    entries[..retained].sort_by(compare);
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct StructuralTrace {
    scanner_matches: Bounded<ScannerMatch>,
    chart: Bounded<ChartItem>,
    forest: Bounded<ForestNode>,
    accepted_roots: Bounded<usize>,
    checked_completion_rejections: Bounded<CheckedCompletionRejection>,
    first_build_rejection: Option<BuildRejection>,
}

impl StructuralTrace {
    #[cfg(test)]
    pub(crate) fn empty() -> Self {
        Self {
            scanner_matches: Bounded::new(0),
            chart: Bounded::new(0),
            forest: Bounded::new(0),
            accepted_roots: Bounded::new(0),
            checked_completion_rejections: Bounded::new(0),
            first_build_rejection: None,
        }
    }
    pub(crate) fn new(
        scanner_matches: Bounded<ScannerMatch>,
        chart: Bounded<ChartItem>,
        forest: Bounded<ForestNode>,
        accepted_roots: Bounded<usize>,
        checked_completion_rejections: Bounded<CheckedCompletionRejection>,
        first_build_rejection: Option<BuildRejection>,
    ) -> Self {
        Self {
            scanner_matches,
            chart,
            forest,
            accepted_roots,
            checked_completion_rejections,
            first_build_rejection,
        }
    }
    pub(crate) const fn scanner_matches(&self) -> &Bounded<ScannerMatch> {
        &self.scanner_matches
    }
    pub(crate) const fn chart(&self) -> &Bounded<ChartItem> {
        &self.chart
    }
    pub(crate) const fn forest(&self) -> &Bounded<ForestNode> {
        &self.forest
    }
    pub(crate) const fn accepted_roots(&self) -> &Bounded<usize> {
        &self.accepted_roots
    }
    pub(crate) const fn checked_completion_rejections(
        &self,
    ) -> &Bounded<CheckedCompletionRejection> {
        &self.checked_completion_rejections
    }
    pub(crate) const fn first_build_rejection(&self) -> Option<&BuildRejection> {
        self.first_build_rejection.as_ref()
    }

    pub(crate) fn project_terminal_build_rejection(&mut self, rejection: Option<BuildRejection>) {
        self.first_build_rejection = rejection;
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ChartItem {
    pub(crate) column: usize,
    pub(crate) rule_name_v1: String,
    pub(crate) dot: usize,
    pub(crate) origin: usize,
    pub(crate) family_count: usize,
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ForestNode {
    pub(crate) id: usize,
    pub(crate) rule_name_v1: String,
    pub(crate) start: usize,
    pub(crate) end: usize,
    pub(crate) families: Bounded<ForestFamily>,
}
impl ForestNode {
    #[must_use]
    pub const fn id(&self) -> usize {
        self.id
    }
    #[must_use]
    pub fn rule_name_v1(&self) -> &str {
        &self.rule_name_v1
    }
    #[must_use]
    pub const fn start(&self) -> usize {
        self.start
    }
    #[must_use]
    pub const fn end(&self) -> usize {
        self.end
    }
    #[must_use]
    pub fn families(&self) -> &Bounded<ForestFamily> {
        &self.families
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ForestFamily {
    pub(crate) children: Bounded<ForestChild>,
}
impl ForestFamily {
    #[must_use]
    pub fn children(&self) -> &Bounded<ForestChild> {
        &self.children
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ForestChild {
    pub(crate) node_id: Option<usize>,
    pub(crate) value_label_v1: Option<String>,
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CheckedCompletionRejection {
    pub(crate) rule_name_v1: String,
    pub(crate) start: usize,
    pub(crate) end: usize,
    pub(crate) family_identity_v1: FamilyIdentity,
}

#[derive(Debug, Clone, PartialEq, Eq, Ord, PartialOrd)]
pub struct FamilyIdentity(pub(crate) Vec<FamilyIdentityChild>);

#[derive(Debug, Clone, PartialEq, Eq, Ord, PartialOrd)]
pub enum FamilyIdentityChild {
    Node(usize),
    Lexical(String),
}

impl ChartItem {
    #[must_use]
    pub const fn column(&self) -> usize {
        self.column
    }
    #[must_use]
    pub fn rule_name_v1(&self) -> &str {
        &self.rule_name_v1
    }
    #[must_use]
    pub const fn dot(&self) -> usize {
        self.dot
    }
    #[must_use]
    pub const fn origin(&self) -> usize {
        self.origin
    }
    #[must_use]
    pub const fn family_count(&self) -> usize {
        self.family_count
    }
}

impl ForestChild {
    #[must_use]
    pub const fn node_id(&self) -> Option<usize> {
        self.node_id
    }
    #[must_use]
    pub fn value_label_v1(&self) -> Option<&str> {
        self.value_label_v1.as_deref()
    }
}

impl CheckedCompletionRejection {
    #[must_use]
    pub fn rule_name_v1(&self) -> &str {
        &self.rule_name_v1
    }
    #[must_use]
    pub const fn start(&self) -> usize {
        self.start
    }
    #[must_use]
    pub const fn end(&self) -> usize {
        self.end
    }
    #[must_use]
    pub const fn family_identity_v1(&self) -> &FamilyIdentity {
        &self.family_identity_v1
    }
}

impl FamilyIdentity {
    #[must_use]
    pub fn children(&self) -> &[FamilyIdentityChild] {
        &self.0
    }
}

/// The provisional Stage 4 specificity assigned to one materialized position.
///
/// Future generated specificity metadata must preserve this diagnostic meaning
/// or bump its machine schema.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Ord, PartialOrd)]
pub enum SpecificityTier {
    Nonterminal,
    TypedLexical,
    /// A spelling supplied by the parse context or a catalog. It names one
    /// particular object, so it outranks a member of a declared type spelling
    /// the same bytes [CR#201.5,201.5c].
    Identity,
    Literal,
}

/// The first fact that determined a pair's specificity ordering.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SelectionDecisive {
    Position(usize),
    VectorExhaustion(usize),
    Tie,
}

/// How a complete candidate selection was resolved.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SelectionResolution {
    Unique,
    Specificity,
    Exception,
    UnresolvedTie,
}

/// One materialized candidate in materialization order.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SelectionCandidate {
    ordinal: usize,
    construction_path: Vec<String>,
    specificity: Vec<SpecificityTier>,
    leaf_path: Vec<String>,
}

impl SelectionCandidate {
    pub(crate) fn new(
        ordinal: usize,
        construction_path: Vec<String>,
        specificity: Vec<SpecificityTier>,
        leaf_path: Vec<String>,
    ) -> Self {
        Self {
            ordinal,
            construction_path,
            specificity,
            leaf_path,
        }
    }

    #[must_use]
    pub const fn ordinal(&self) -> usize {
        self.ordinal
    }

    #[must_use]
    pub fn construction_path(&self) -> &[String] {
        &self.construction_path
    }

    #[must_use]
    pub fn specificity(&self) -> &[SpecificityTier] {
        &self.specificity
    }

    #[must_use]
    pub fn leaf_path(&self) -> &[String] {
        &self.leaf_path
    }
}

/// One sorted unordered candidate-pair comparison.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SelectionComparison {
    left_ordinal: usize,
    right_ordinal: usize,
    ordering: Ordering,
    decisive: SelectionDecisive,
    exception_id: Option<String>,
}

impl SelectionComparison {
    pub(crate) fn new(
        left_ordinal: usize,
        right_ordinal: usize,
        ordering: Ordering,
        decisive: SelectionDecisive,
        exception_id: Option<String>,
    ) -> Self {
        Self {
            left_ordinal,
            right_ordinal,
            ordering,
            decisive,
            exception_id,
        }
    }

    #[must_use]
    pub const fn left_ordinal(&self) -> usize {
        self.left_ordinal
    }

    #[must_use]
    pub const fn right_ordinal(&self) -> usize {
        self.right_ordinal
    }

    #[must_use]
    pub const fn ordering(&self) -> Ordering {
        self.ordering
    }

    #[must_use]
    pub const fn decisive(&self) -> SelectionDecisive {
        self.decisive
    }

    #[must_use]
    pub fn exception_id(&self) -> Option<&str> {
        self.exception_id.as_deref()
    }
}

/// The complete selection facts for one nonempty materialized candidate set.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SelectionDecision {
    candidates: Vec<SelectionCandidate>,
    comparisons: Vec<SelectionComparison>,
    survivors: Vec<usize>,
    selected: Option<usize>,
    resolution: SelectionResolution,
    exception_uses: Vec<String>,
}

impl SelectionDecision {
    pub(crate) fn new(
        candidates: Vec<SelectionCandidate>,
        comparisons: Vec<SelectionComparison>,
        survivors: Vec<usize>,
        selected: Option<usize>,
        resolution: SelectionResolution,
        exception_uses: Vec<String>,
    ) -> Self {
        Self {
            candidates,
            comparisons,
            survivors,
            selected,
            resolution,
            exception_uses,
        }
    }

    #[must_use]
    pub fn candidates(&self) -> &[SelectionCandidate] {
        &self.candidates
    }

    #[must_use]
    pub fn comparisons(&self) -> &[SelectionComparison] {
        &self.comparisons
    }

    #[must_use]
    pub fn survivors(&self) -> &[usize] {
        &self.survivors
    }

    #[must_use]
    pub const fn selected(&self) -> Option<usize> {
        self.selected
    }

    #[must_use]
    pub const fn resolution(&self) -> SelectionResolution {
        self.resolution
    }

    #[must_use]
    pub fn exception_uses(&self) -> &[String] {
        &self.exception_uses
    }
}

/// A trace-bounded candidate projection.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BoundedSelectionCandidate {
    ordinal: usize,
    construction_path: Bounded<String>,
    specificity: Bounded<SpecificityTier>,
}

impl BoundedSelectionCandidate {
    fn from_complete(candidate: &SelectionCandidate, limit: usize) -> Self {
        Self {
            ordinal: candidate.ordinal(),
            construction_path: bounded_copy(candidate.construction_path(), limit, Clone::clone),
            specificity: bounded_copy(candidate.specificity(), limit, |tier| *tier),
        }
    }

    #[must_use]
    pub const fn ordinal(&self) -> usize {
        self.ordinal
    }
    #[must_use]
    pub const fn construction_path(&self) -> &Bounded<String> {
        &self.construction_path
    }
    #[must_use]
    pub const fn specificity(&self) -> &Bounded<SpecificityTier> {
        &self.specificity
    }
}

/// A trace-bounded copy of one complete pair comparison.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BoundedSelectionComparison {
    left_ordinal: usize,
    right_ordinal: usize,
    ordering: Ordering,
    decisive: SelectionDecisive,
    exception_id: Option<String>,
}

impl BoundedSelectionComparison {
    fn from_complete(comparison: &SelectionComparison) -> Self {
        Self {
            left_ordinal: comparison.left_ordinal(),
            right_ordinal: comparison.right_ordinal(),
            ordering: comparison.ordering(),
            decisive: comparison.decisive(),
            exception_id: comparison.exception_id().map(str::to_owned),
        }
    }

    #[must_use]
    pub const fn left_ordinal(&self) -> usize {
        self.left_ordinal
    }
    #[must_use]
    pub const fn right_ordinal(&self) -> usize {
        self.right_ordinal
    }
    #[must_use]
    pub const fn ordering(&self) -> Ordering {
        self.ordering
    }
    #[must_use]
    pub const fn decisive(&self) -> SelectionDecisive {
        self.decisive
    }
    #[must_use]
    pub fn exception_id(&self) -> Option<&str> {
        self.exception_id.as_deref()
    }
}

/// Why a materialized candidate was not selected.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SelectionLoserReason {
    LessSpecific,
    ExceptionLoser,
    UnresolvedSurvivor,
}

impl SelectionLoserReason {
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LessSpecific => "less_specific",
            Self::ExceptionLoser => "exception_loser",
            Self::UnresolvedSurvivor => "unresolved_survivor",
        }
    }
}

/// One unselected candidate and the comparisons that establish its reason.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UnselectedCandidate {
    candidate: BoundedSelectionCandidate,
    reason: SelectionLoserReason,
    evidence: Bounded<BoundedSelectionComparison>,
}

impl UnselectedCandidate {
    #[must_use]
    pub const fn candidate(&self) -> &BoundedSelectionCandidate {
        &self.candidate
    }
    #[must_use]
    pub const fn reason(&self) -> SelectionLoserReason {
        self.reason
    }
    #[must_use]
    pub const fn evidence(&self) -> &Bounded<BoundedSelectionComparison> {
        &self.evidence
    }
}

/// A bounded public projection of a complete selection decision.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BoundedSelectionDecision {
    candidates: Bounded<BoundedSelectionCandidate>,
    comparisons: Bounded<BoundedSelectionComparison>,
    survivors: Bounded<usize>,
    selected: Option<BoundedSelectionCandidate>,
    resolution: SelectionResolution,
    exception_uses: Bounded<String>,
    unselected_candidates: Bounded<UnselectedCandidate>,
}

impl BoundedSelectionDecision {
    pub(crate) fn from_complete(decision: &SelectionDecision, limit: usize) -> Self {
        let candidates = bounded_copy(decision.candidates(), limit, |candidate| {
            BoundedSelectionCandidate::from_complete(candidate, limit)
        });
        let comparisons = bounded_copy(decision.comparisons(), limit, |comparison| {
            BoundedSelectionComparison::from_complete(comparison)
        });
        let survivors = bounded_copy(decision.survivors(), limit, |ordinal| *ordinal);
        let selected = decision.selected().map(|ordinal| {
            BoundedSelectionCandidate::from_complete(candidate(decision, ordinal), limit)
        });
        let exception_uses = bounded_copy(decision.exception_uses(), limit, Clone::clone);
        let mut unselected_candidates = Bounded::new(limit);
        for complete_candidate in decision
            .candidates()
            .iter()
            .filter(|candidate| Some(candidate.ordinal()) != decision.selected())
        {
            unselected_candidates.push_with(|| {
                let ordinal = complete_candidate.ordinal();
                let reason = loser_reason(decision, ordinal);
                let evidence = bounded_copy(
                    &loser_evidence(decision, ordinal, reason),
                    limit,
                    |comparison| BoundedSelectionComparison::from_complete(comparison),
                );
                UnselectedCandidate {
                    candidate: BoundedSelectionCandidate::from_complete(complete_candidate, limit),
                    reason,
                    evidence,
                }
            });
        }
        Self {
            candidates,
            comparisons,
            survivors,
            selected,
            resolution: decision.resolution(),
            exception_uses,
            unselected_candidates,
        }
    }

    #[must_use]
    pub const fn candidates(&self) -> &Bounded<BoundedSelectionCandidate> {
        &self.candidates
    }
    #[must_use]
    pub const fn comparisons(&self) -> &Bounded<BoundedSelectionComparison> {
        &self.comparisons
    }
    #[must_use]
    pub const fn survivors(&self) -> &Bounded<usize> {
        &self.survivors
    }
    #[must_use]
    pub const fn selected(&self) -> Option<&BoundedSelectionCandidate> {
        self.selected.as_ref()
    }
    #[must_use]
    pub const fn resolution(&self) -> SelectionResolution {
        self.resolution
    }
    #[must_use]
    pub const fn exception_uses(&self) -> &Bounded<String> {
        &self.exception_uses
    }
    #[must_use]
    pub const fn unselected_candidates(&self) -> &Bounded<UnselectedCandidate> {
        &self.unselected_candidates
    }
}

fn candidate(decision: &SelectionDecision, ordinal: usize) -> &SelectionCandidate {
    decision
        .candidates()
        .iter()
        .find(|candidate| candidate.ordinal() == ordinal)
        .expect("selected ordinal names a complete candidate")
}

fn loser_reason(decision: &SelectionDecision, ordinal: usize) -> SelectionLoserReason {
    let loses_specificity = decision.comparisons().iter().any(|comparison| {
        relative_ordering(comparison, ordinal) == Some(Ordering::Less)
            && comparison.exception_id().is_none()
            && comparison.decisive() != SelectionDecisive::Tie
    });
    if loses_specificity {
        return SelectionLoserReason::LessSpecific;
    }
    let loses_exception = decision.comparisons().iter().any(|comparison| {
        relative_ordering(comparison, ordinal) == Some(Ordering::Less)
            && comparison.exception_id().is_some()
    });
    if loses_exception {
        SelectionLoserReason::ExceptionLoser
    } else {
        SelectionLoserReason::UnresolvedSurvivor
    }
}

fn loser_evidence(
    decision: &SelectionDecision,
    ordinal: usize,
    reason: SelectionLoserReason,
) -> Vec<&SelectionComparison> {
    decision
        .comparisons()
        .iter()
        .filter(|comparison| match reason {
            SelectionLoserReason::LessSpecific => {
                relative_ordering(comparison, ordinal) == Some(Ordering::Less)
                    && comparison.exception_id().is_none()
                    && comparison.decisive() != SelectionDecisive::Tie
            }
            SelectionLoserReason::ExceptionLoser => {
                relative_ordering(comparison, ordinal) == Some(Ordering::Less)
                    && comparison.exception_id().is_some()
            }
            SelectionLoserReason::UnresolvedSurvivor => {
                relative_ordering(comparison, ordinal) == Some(Ordering::Equal)
            }
        })
        .collect()
}

fn relative_ordering(comparison: &SelectionComparison, ordinal: usize) -> Option<Ordering> {
    if comparison.left_ordinal() == ordinal {
        Some(comparison.ordering())
    } else if comparison.right_ordinal() == ordinal {
        Some(comparison.ordering().reverse())
    } else {
        None
    }
}

fn bounded_copy<T, U>(values: &[T], limit: usize, project: impl Fn(&T) -> U) -> Bounded<U> {
    let mut bounded = Bounded::new(limit);
    for value in values {
        bounded.push_with(|| project(value));
    }
    bounded
}

#[cfg(test)]
thread_local! {
    static SELECTED_CLAIM_PROJECTION_CONSTRUCTIONS: std::cell::Cell<usize> =
        const { std::cell::Cell::new(0) };
}

#[cfg(test)]
pub(super) fn reset_selected_claim_projection_constructions() {
    SELECTED_CLAIM_PROJECTION_CONSTRUCTIONS.with(|constructions| constructions.set(0));
}

#[cfg(test)]
pub(super) fn selected_claim_projection_constructions() -> usize {
    SELECTED_CLAIM_PROJECTION_CONSTRUCTIONS.with(std::cell::Cell::get)
}

fn clone_selected_claim(claim: &LexicalClaim) -> LexicalClaim {
    #[cfg(test)]
    SELECTED_CLAIM_PROJECTION_CONSTRUCTIONS
        .with(|constructions| constructions.set(constructions.get() + 1));
    claim.clone()
}

/// A typed internal parser failure suitable for diagnostics.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InternalFailureKind {
    ValidatedRootDidNotMaterialize,
    SelectionConfiguration,
    OwnershipInspection,
}

/// The public outcome class for one parser analysis.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ParseAnalysisOutcome {
    Selected,
    ParseFailure,
    UnresolvedAmbiguity,
    InternalFailure(InternalFailureKind),
}

/// A complete parser run with its nonmutating selection diagnostic.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParseAnalysis<V = Ability> {
    result: Result<V, ParseError>,
    outcome: ParseAnalysisOutcome,
    decision: Option<SelectionDecision>,
    ownership: Option<SelectedOwnership>,
}

impl<V> ParseAnalysis<V> {
    pub(crate) fn from_result(
        result: Result<V, ParseError>,
        decision: Option<SelectionDecision>,
    ) -> Self {
        let outcome = match &result {
            Ok(_) => ParseAnalysisOutcome::Selected,
            Err(ParseError::Failure { .. } | ParseError::BuildRejected { .. }) => {
                ParseAnalysisOutcome::ParseFailure
            }
            Err(ParseError::Ambiguous { .. }) => ParseAnalysisOutcome::UnresolvedAmbiguity,
            Err(ParseError::ValidatedRootDidNotMaterialize) => {
                ParseAnalysisOutcome::InternalFailure(
                    InternalFailureKind::ValidatedRootDidNotMaterialize,
                )
            }
            Err(ParseError::InvalidSelectionExceptionConfiguration(_)) => {
                ParseAnalysisOutcome::InternalFailure(InternalFailureKind::SelectionConfiguration)
            }
            Err(ParseError::OwnershipInspection) => {
                ParseAnalysisOutcome::InternalFailure(InternalFailureKind::OwnershipInspection)
            }
        };
        Self {
            result,
            outcome,
            decision,
            ownership: None,
        }
    }

    pub(crate) fn with_ownership(mut self, ownership: SelectedOwnership) -> Self {
        self.ownership = Some(ownership);
        self
    }

    #[must_use]
    pub const fn outcome(&self) -> ParseAnalysisOutcome {
        self.outcome
    }

    #[must_use]
    pub fn decision(&self) -> Option<&SelectionDecision> {
        self.decision.as_ref()
    }

    #[must_use]
    pub fn selected(&self) -> Option<&V> {
        self.result.as_ref().ok()
    }

    #[must_use]
    pub fn error(&self) -> Option<&ParseError> {
        self.result.as_ref().err()
    }

    #[must_use]
    pub fn ownership(&self) -> Option<&SelectedOwnership> {
        self.ownership.as_ref()
    }

    #[must_use]
    pub fn build_rejection(&self) -> Option<&BuildRejection> {
        match &self.result {
            Err(ParseError::BuildRejected { rejection, .. }) => Some(rejection),
            _ => None,
        }
    }

    /// # Errors
    ///
    /// Returns the original parser error for an unselected analysis.
    pub fn into_parse_result(self) -> Result<V, ParseError> {
        self.result
    }
}

/// One materialized candidate as observed before selection.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MaterializedCandidateInfo {
    ordinal: usize,
    rendered: String,
    ast_debug_v1: String,
    construction_path: Bounded<String>,
    specificity: Bounded<SpecificityTier>,
}

impl MaterializedCandidateInfo {
    pub(crate) fn new(
        ordinal: usize,
        rendered: String,
        ast_debug_v1: String,
        construction_path: &[Construction],
        specificity: &[SpecificityTier],
        limit: usize,
    ) -> Self {
        Self {
            ordinal,
            rendered,
            ast_debug_v1,
            construction_path: bounded_copy(construction_path, limit, |construction| {
                construction_name_v1(*construction)
            }),
            specificity: bounded_copy(specificity, limit, |tier| *tier),
        }
    }

    #[cfg(test)]
    fn new_for_test(ordinal: usize) -> Self {
        Self::new(
            ordinal,
            format!("rendered {ordinal}"),
            format!("debug {ordinal}"),
            &[Construction::AbilityPlain],
            &[SpecificityTier::Literal],
            usize::MAX,
        )
    }

    #[must_use]
    pub const fn ordinal(&self) -> usize {
        self.ordinal
    }
    #[must_use]
    pub fn rendered(&self) -> &str {
        &self.rendered
    }
    #[must_use]
    pub fn ast_debug_v1(&self) -> &str {
        &self.ast_debug_v1
    }
    #[must_use]
    pub const fn construction_path(&self) -> &Bounded<String> {
        &self.construction_path
    }
    #[must_use]
    pub const fn specificity(&self) -> &Bounded<SpecificityTier> {
        &self.specificity
    }
}

/// One deduplicated materialization cycle pruned during recursion.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MaterializationCycle {
    node_ordinal: usize,
    construction_path: Bounded<String>,
}

impl MaterializationCycle {
    #[must_use]
    pub const fn node_ordinal(&self) -> usize {
        self.node_ordinal
    }
    #[must_use]
    pub const fn construction_path(&self) -> &Bounded<String> {
        &self.construction_path
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct MaterializationTrace {
    candidates: Bounded<MaterializedCandidateInfo>,
    cycles: Bounded<MaterializationCycle>,
    first_build_rejection: Option<BuildRejection>,
}

impl MaterializationTrace {
    pub(crate) fn empty(limit: usize) -> Self {
        Self {
            candidates: Bounded::new(limit),
            cycles: Bounded::new(limit),
            first_build_rejection: None,
        }
    }
    pub(crate) const fn candidates(&self) -> &Bounded<MaterializedCandidateInfo> {
        &self.candidates
    }
    pub(crate) const fn cycles(&self) -> &Bounded<MaterializationCycle> {
        &self.cycles
    }
    pub(crate) const fn first_build_rejection(&self) -> Option<&BuildRejection> {
        self.first_build_rejection.as_ref()
    }

    pub(crate) fn project_terminal_build_rejection(&mut self, rejection: Option<BuildRejection>) {
        self.first_build_rejection = rejection;
    }
}

pub(crate) struct MaterializationTraceBuilder {
    limit: usize,
    candidates: Bounded<MaterializedCandidateInfo>,
    cycles: Bounded<MaterializationCycle>,
    cycle_identities: Vec<(usize, Vec<usize>)>,
    first_build_rejection: Option<BuildRejection>,
}

impl MaterializationTraceBuilder {
    pub(crate) fn new(limits: TraceLimits) -> Self {
        let limit = limits.per_collection();
        Self {
            limit,
            candidates: Bounded::new(limit),
            cycles: Bounded::new(limit),
            cycle_identities: Vec::new(),
            first_build_rejection: None,
        }
    }

    pub(crate) fn record_candidate_with(
        &mut self,
        make_candidate: impl FnOnce() -> MaterializedCandidateInfo,
    ) {
        self.candidates.push_with(make_candidate);
    }

    pub(crate) fn record_cycle(&mut self, node_ordinal: usize, rule_path: &[RuleId]) {
        self.record_cycle_with(
            node_ordinal,
            rule_path,
            |rule| rule.index(),
            |rule| rule_label_v1(*rule),
        );
    }

    pub(crate) fn record_cycle_with<R>(
        &mut self,
        node_ordinal: usize,
        rule_path: &[R],
        identity: impl Fn(&R) -> usize,
        label: impl Fn(&R) -> String,
    ) {
        let identities = rule_path.iter().map(identity).collect::<Vec<_>>();
        if self
            .cycle_identities
            .iter()
            .any(|(seen_node, seen_path)| *seen_node == node_ordinal && seen_path == &identities)
        {
            return;
        }
        self.cycle_identities.push((node_ordinal, identities));
        self.cycles.push_with(|| MaterializationCycle {
            node_ordinal,
            construction_path: bounded_copy(rule_path, self.limit, label),
        });
    }

    pub(crate) fn record_build_rejection(
        &mut self,
        rule_path: &[RuleId],
        rejection: BuildRejection,
    ) {
        self.record_build_rejection_with(
            rule_path,
            rejection,
            |rule| rule.index(),
            |rule| rule_label_v1(*rule),
        );
    }

    pub(crate) fn record_build_rejection_with<R>(
        &mut self,
        _rule_path: &[R],
        rejection: BuildRejection,
        _identity: impl Fn(&R) -> usize,
        _label: impl Fn(&R) -> String,
    ) {
        self.first_build_rejection.get_or_insert(rejection);
    }

    pub(crate) fn project_terminal_build_rejection(&mut self, rejection: Option<BuildRejection>) {
        self.first_build_rejection = rejection;
    }

    pub(crate) fn finish(self) -> MaterializationTrace {
        MaterializationTrace {
            candidates: self.candidates,
            cycles: self.cycles,
            first_build_rejection: self.first_build_rejection,
        }
    }
}

fn rule_label_v1(rule: RuleId) -> String {
    rule_label_from_metadata(
        rule.public_construction().map(construction_name_v1),
        rule.owner(),
        rule.role(),
        rule.state(),
    )
}

pub(super) fn rule_label_from_metadata(
    public_construction: Option<String>,
    owner: &str,
    role: Option<&str>,
    state: &str,
) -> String {
    public_construction.unwrap_or_else(|| {
        role.map_or_else(
            || format!("{owner} [{state}]"),
            |role| format!("{owner}.{role} [{state}]"),
        )
    })
}

/// Stable trace projection of a parse expectation.
#[derive(Debug, Clone, PartialEq, Eq, Ord, PartialOrd)]
pub enum ExpectationInfo {
    Nonterminal(super::NonterminalCategory),
    Terminal(super::TerminalClass),
    Literal(&'static str),
}

impl From<&Expectation> for ExpectationInfo {
    fn from(expectation: &Expectation) -> Self {
        match expectation {
            Expectation::Nonterminal(category) => Self::Nonterminal(*category),
            Expectation::Terminal(class) => Self::Terminal(*class),
            Expectation::Literal(literal) => Self::Literal(literal),
        }
    }
}

/// Selected trace outcome data.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SelectedParseOutcome {
    rendered: String,
    selection: BoundedSelectionDecision,
}

impl SelectedParseOutcome {
    #[must_use]
    pub fn rendered(&self) -> &str {
        &self.rendered
    }
    #[must_use]
    pub const fn selection(&self) -> &BoundedSelectionDecision {
        &self.selection
    }
}

/// Ordinary parse-failure trace outcome data.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParseFailureOutcome {
    span: super::TextSpan,
    expectations: Bounded<ExpectationInfo>,
}

impl ParseFailureOutcome {
    #[must_use]
    pub const fn span(&self) -> super::TextSpan {
        self.span
    }
    #[must_use]
    pub const fn expectations(&self) -> &Bounded<ExpectationInfo> {
        &self.expectations
    }
}

/// Unresolved-ambiguity trace outcome data.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UnresolvedAmbiguityOutcome {
    selection: BoundedSelectionDecision,
}

impl UnresolvedAmbiguityOutcome {
    #[must_use]
    pub const fn selection(&self) -> &BoundedSelectionDecision {
        &self.selection
    }
}

/// Typed internal-failure trace outcome data.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InternalFailureOutcome {
    kind: InternalFailureKind,
    message: String,
}

impl InternalFailureOutcome {
    #[must_use]
    pub const fn kind(&self) -> InternalFailureKind {
        self.kind
    }
    #[must_use]
    pub fn message(&self) -> &str {
        &self.message
    }
}

/// The bounded public outcome of one traced parser run.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BoundedParseOutcome {
    Selected(SelectedParseOutcome),
    ParseFailure(ParseFailureOutcome),
    UnresolvedAmbiguity(UnresolvedAmbiguityOutcome),
    InternalFailure(InternalFailureOutcome),
}

/// One bounded parser trace over the ordinary parser pipeline.
///
/// The complete analysis is deliberately private and is consumed only by
/// [`ParserTrace::into_parse_result`].
/// Its [`Debug`](std::fmt::Debug) representation and [`PartialEq`] / [`Eq`]
/// semantics are also bounded public projections: they observe only the
/// outcome and trace sections available from the public read-only accessors,
/// never the private analysis or raw error.
///
/// ```compile_fail
/// use deckmaste_english_v2::parser::ParserTrace;
/// fn leak_analysis(trace: ParserTrace) { let _ = trace.analysis; }
/// ```
///
/// ```compile_fail
/// use deckmaste_english_v2::parser::ParserTrace;
/// fn leak_error(trace: ParserTrace) { let _ = trace.parse_error(); }
/// ```
#[derive(Clone)]
pub struct ParserTrace<V = Ability> {
    root_name: &'static str,
    analysis: ParseAnalysis<V>,
    outcome: BoundedParseOutcome,
    structural: StructuralTrace,
    materialization: MaterializationTrace,
    selected_lexical_claims: Bounded<LexicalClaim>,
    ownership: Option<OwnershipSummary>,
    ownership_failures: Vec<OwnershipFailure>,
}

impl<V> PartialEq for ParserTrace<V> {
    fn eq(&self, other: &Self) -> bool {
        self.outcome() == other.outcome()
            && self.root_name() == other.root_name()
            && self.scanner_matches() == other.scanner_matches()
            && self.chart() == other.chart()
            && self.forest() == other.forest()
            && self.accepted_roots() == other.accepted_roots()
            && self.checked_completion_rejections() == other.checked_completion_rejections()
            && self.materialized_candidates() == other.materialized_candidates()
            && self.materialization_cycles() == other.materialization_cycles()
            && self.selected_lexical_claims() == other.selected_lexical_claims()
            && self.ownership() == other.ownership()
            && self.ownership_failures() == other.ownership_failures()
    }
}

impl<V> Eq for ParserTrace<V> {}

impl<V> std::fmt::Debug for ParserTrace<V> {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("ParserTrace")
            .field("root_name", &self.root_name())
            .field("outcome", self.outcome())
            .field("scanner_matches", self.scanner_matches())
            .field("chart", self.chart())
            .field("forest", self.forest())
            .field("accepted_roots", self.accepted_roots())
            .field(
                "checked_completion_rejections",
                self.checked_completion_rejections(),
            )
            .field("materialized_candidates", self.materialized_candidates())
            .field("materialization_cycles", self.materialization_cycles())
            .field("selected_lexical_claims", self.selected_lexical_claims())
            .field("ownership", &self.ownership())
            .field("ownership_failures", &self.ownership_failures())
            .finish()
    }
}

impl<V> ParserTrace<V> {
    pub(crate) fn from_parts(
        analysis: ParseAnalysis<V>,
        structural: StructuralTrace,
        materialization: MaterializationTrace,
        limits: TraceLimits,
        context: &ParseContext<'_>,
        environment: &crate::environment::ParserEnvironment,
    ) -> Self
    where
        V: GeneratedParseRoot,
    {
        let limit = limits.per_collection();
        let selected_ownership = analysis.ownership();
        let selected_lexical_claims = selected_ownership.map_or_else(
            || Bounded::new(limit),
            |ownership| bounded_copy(ownership.parsed_claims(), limit, clone_selected_claim),
        );
        let ownership = selected_ownership.map(|ownership| ownership.summary().clone());
        let ownership_failures = selected_ownership
            .map(|ownership| ownership.failures().to_vec())
            .unwrap_or_default();
        let outcome = match &analysis.result {
            Ok(value) => {
                let decision = analysis
                    .decision()
                    .expect("a selected candidate has a selection decision");
                let selected_ordinal = decision
                    .selected()
                    .expect("a selected outcome has a selected ordinal");
                let rendered = materialization
                    .candidates()
                    .items()
                    .iter()
                    .find(|candidate| candidate.ordinal() == selected_ordinal)
                    .map_or_else(
                        || V::render_with_claims(value, context, environment).0,
                        |candidate| candidate.rendered.clone(),
                    );
                BoundedParseOutcome::Selected(SelectedParseOutcome {
                    rendered,
                    selection: BoundedSelectionDecision::from_complete(decision, limit),
                })
            }
            Err(ParseError::Failure { span, expectations }) => {
                let mut bounded_expectations = Bounded::new(limit);
                for expectation in expectations {
                    bounded_expectations.push_with(|| ExpectationInfo::from(expectation));
                }
                BoundedParseOutcome::ParseFailure(ParseFailureOutcome {
                    span: *span,
                    expectations: bounded_expectations,
                })
            }
            Err(ParseError::BuildRejected { span, .. }) => {
                BoundedParseOutcome::ParseFailure(ParseFailureOutcome {
                    span: *span,
                    expectations: Bounded::new(limit),
                })
            }
            Err(ParseError::Ambiguous { .. }) => {
                let decision = analysis
                    .decision()
                    .expect("an unresolved ambiguity has a selection decision");
                BoundedParseOutcome::UnresolvedAmbiguity(UnresolvedAmbiguityOutcome {
                    selection: BoundedSelectionDecision::from_complete(decision, limit),
                })
            }
            Err(error @ ParseError::ValidatedRootDidNotMaterialize) => {
                BoundedParseOutcome::InternalFailure(InternalFailureOutcome {
                    kind: InternalFailureKind::ValidatedRootDidNotMaterialize,
                    message: format!("{} root: {error}", V::NAME),
                })
            }
            Err(error @ ParseError::InvalidSelectionExceptionConfiguration(_)) => {
                BoundedParseOutcome::InternalFailure(InternalFailureOutcome {
                    kind: InternalFailureKind::SelectionConfiguration,
                    message: format!("{} root: {error}", V::NAME),
                })
            }
            Err(error @ ParseError::OwnershipInspection) => {
                BoundedParseOutcome::InternalFailure(InternalFailureOutcome {
                    kind: InternalFailureKind::OwnershipInspection,
                    message: format!("{} root: {error}", V::NAME),
                })
            }
        };
        Self {
            root_name: V::NAME,
            analysis,
            outcome,
            structural,
            materialization,
            selected_lexical_claims,
            ownership,
            ownership_failures,
        }
    }
}

impl<V> ParserTrace<V> {
    #[must_use]
    pub const fn root_name(&self) -> &'static str {
        self.root_name
    }

    #[must_use]
    pub const fn outcome(&self) -> &BoundedParseOutcome {
        &self.outcome
    }
    #[must_use]
    pub fn selected(&self) -> Option<&V> {
        self.analysis.selected()
    }
    #[must_use]
    pub const fn scanner_matches(&self) -> &Bounded<ScannerMatch> {
        self.structural.scanner_matches()
    }
    #[must_use]
    pub const fn chart(&self) -> &Bounded<ChartItem> {
        self.structural.chart()
    }
    #[must_use]
    pub const fn forest(&self) -> &Bounded<ForestNode> {
        self.structural.forest()
    }
    #[must_use]
    pub const fn accepted_roots(&self) -> &Bounded<usize> {
        self.structural.accepted_roots()
    }
    #[must_use]
    pub const fn checked_completion_rejections(&self) -> &Bounded<CheckedCompletionRejection> {
        self.structural.checked_completion_rejections()
    }
    #[must_use]
    pub fn build_rejection(&self) -> Option<&BuildRejection> {
        self.analysis.build_rejection()
    }
    #[must_use]
    pub const fn materialized_candidates(&self) -> &Bounded<MaterializedCandidateInfo> {
        self.materialization.candidates()
    }
    #[must_use]
    pub const fn materialization_cycles(&self) -> &Bounded<MaterializationCycle> {
        self.materialization.cycles()
    }
    #[must_use]
    pub const fn selected_lexical_claims(&self) -> &Bounded<LexicalClaim> {
        &self.selected_lexical_claims
    }
    #[must_use]
    pub const fn ownership(&self) -> Option<&OwnershipSummary> {
        self.ownership.as_ref()
    }
    #[must_use]
    pub fn ownership_failures(&self) -> &[OwnershipFailure] {
        &self.ownership_failures
    }

    /// # Errors
    ///
    /// Returns the complete original parser error, unaffected by public caps.
    pub fn into_parse_result(self) -> Result<V, ParseError> {
        self.analysis.into_parse_result()
    }
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeSet;

    use super::Bounded;
    use super::BoundedParseOutcome;
    use super::InternalFailureKind;
    use super::MaterializationTrace;
    use super::MaterializationTraceBuilder;
    use super::MaterializedCandidateInfo;
    use super::ParseAnalysis;
    use super::ParseAnalysisOutcome;
    use super::ParserTrace;
    use super::StructuralTrace;
    use super::TraceLimits;
    use crate::context::ParseContext;
    use crate::environment::canonical_test_environment;
    use crate::parser::Expectation;
    use crate::parser::ParseError;
    use crate::parser::Parser;
    use crate::parser::SelectionDecision;
    use crate::parser::SelectionExceptionInventoryError;
    use crate::parser::SelectionResolution;
    use crate::parser::TextSpan;

    #[test]
    fn parser_analysis_keeps_internal_failure_kinds_distinct_from_parse_failure() {
        let materialization = ParseAnalysis::<crate::ast::Ability>::from_result(
            Err(ParseError::ValidatedRootDidNotMaterialize),
            None,
        );
        let selection = ParseAnalysis::<crate::ast::Ability>::from_result(
            Err(ParseError::InvalidSelectionExceptionConfiguration(
                SelectionExceptionInventoryError::BlankId,
            )),
            None,
        );
        let parse_failure = ParseAnalysis::<crate::ast::Ability>::from_result(
            Err(ParseError::Failure {
                span: TextSpan { start: 0, end: 0 },
                expectations: BTreeSet::from([Expectation::Literal("synthetic")]),
            }),
            None,
        );

        assert_eq!(
            materialization.outcome(),
            ParseAnalysisOutcome::InternalFailure(
                InternalFailureKind::ValidatedRootDidNotMaterialize
            )
        );
        assert_eq!(
            selection.outcome(),
            ParseAnalysisOutcome::InternalFailure(InternalFailureKind::SelectionConfiguration)
        );
        assert_eq!(parse_failure.outcome(), ParseAnalysisOutcome::ParseFailure);
    }

    #[test]
    fn structural_trace_bounded_collector_keeps_exact_counts_and_prefix_only() {
        let mut bounded = Bounded::new(1);
        bounded.push("first".to_owned());
        bounded.push("second".to_owned());
        assert_eq!(bounded.total(), 2);
        assert_eq!(bounded.shown(), 1);
        assert_eq!(bounded.omitted(), 1);
        assert_eq!(bounded.items(), ["first"]);

        let zero = Bounded::<String>::new(0);
        assert_eq!((zero.total(), zero.shown(), zero.omitted()), (0, 0, 0));
    }

    #[test]
    fn structural_trace_bounded_caps_are_exact_and_lazy() {
        for (limit, expected_items) in [
            (0, Vec::<usize>::new()),
            (1, vec![10]),
            (8, vec![10, 20, 30]),
        ] {
            let mut bounded = Bounded::new(limit);
            let mut constructions = 0;
            for value in [10, 20, 30] {
                bounded.push_with(|| {
                    constructions += 1;
                    value
                });
            }
            assert_eq!(bounded.total(), 3);
            assert_eq!(bounded.shown(), expected_items.len());
            assert_eq!(bounded.omitted(), 3 - expected_items.len());
            assert_eq!(bounded.items(), expected_items);
            assert_eq!(constructions, expected_items.len());
        }
    }

    #[test]
    fn parser_trace_materialized_payloads_are_not_constructed_past_the_cap() {
        for (limit, expected_constructions) in [(0, 0), (1, 1), (8, 3)] {
            let mut builder = MaterializationTraceBuilder::new(TraceLimits::new(limit));
            let mut constructions = 0;
            for ordinal in 0..3 {
                builder.record_candidate_with(|| {
                    constructions += 1;
                    MaterializedCandidateInfo::new_for_test(ordinal)
                });
            }
            let trace = builder.finish();
            assert_eq!(trace.candidates().total(), 3);
            assert_eq!(trace.candidates().shown(), expected_constructions);
            assert_eq!(constructions, expected_constructions);
        }
    }

    #[test]
    fn materialization_cycle_deduplication_uses_rule_identity_not_projected_label() {
        let mut builder = MaterializationTraceBuilder::new(TraceLimits::new(8));
        builder.record_cycle_with(
            0,
            &[0usize],
            |rule| *rule,
            |_| "same projected label".to_owned(),
        );
        builder.record_cycle_with(
            0,
            &[1usize],
            |rule| *rule,
            |_| "same projected label".to_owned(),
        );

        let trace = builder.finish();
        assert_eq!(
            trace.cycles().total(),
            2,
            "distinct rule paths must survive even when their public labels coincide",
        );
    }

    #[test]
    fn parser_trace_lexical_ownership_nonselected_outcomes_are_empty() {
        let context = ParseContext::new(
            "Trace Card",
            false,
            deckmaste_construction_core::macro_def::Onset::Consonant,
        )
        .expect("context");
        let environment = canonical_test_environment();
        let unresolved = SelectionDecision::new(
            Vec::new(),
            Vec::new(),
            vec![0, 1],
            None,
            SelectionResolution::UnresolvedTie,
            Vec::new(),
        );
        let cases = [
            (
                ParseError::Failure {
                    span: TextSpan { start: 0, end: 0 },
                    expectations: BTreeSet::new(),
                },
                None,
                None,
            ),
            (
                ParseError::Ambiguous {
                    first: "first",
                    second: "second",
                },
                Some(unresolved),
                None,
            ),
            (
                ParseError::ValidatedRootDidNotMaterialize,
                None,
                Some(InternalFailureKind::ValidatedRootDidNotMaterialize),
            ),
            (
                ParseError::InvalidSelectionExceptionConfiguration(
                    SelectionExceptionInventoryError::BlankId,
                ),
                None,
                Some(InternalFailureKind::SelectionConfiguration),
            ),
            (
                ParseError::OwnershipInspection,
                None,
                Some(InternalFailureKind::OwnershipInspection),
            ),
        ];
        for (error, decision, expected_internal_kind) in cases {
            let trace = ParserTrace::from_parts(
                ParseAnalysis::<crate::ast::Ability>::from_result(Err(error), decision),
                StructuralTrace::empty(),
                MaterializationTrace::empty(0),
                TraceLimits::new(0),
                &context,
                &environment,
            );
            assert_eq!(trace.selected_lexical_claims().total(), 0);
            assert!(trace.selected_lexical_claims().items().is_empty());
            assert_eq!(trace.ownership(), None);
            assert!(trace.ownership_failures().is_empty());
            if let Some(expected_kind) = expected_internal_kind {
                let BoundedParseOutcome::InternalFailure(failure) = trace.outcome() else {
                    panic!("typed internal outcome");
                };
                assert_eq!(failure.kind(), expected_kind);
            }
        }
    }

    #[test]
    fn parser_trace_lexical_ownership_debug_and_equality_ignore_private_omissions() {
        let environment = canonical_test_environment();
        let parser = Parser::new(environment).expect("canonical environment satisfies grammar");
        let context = ParseContext::new(
            "Trace Card",
            false,
            deckmaste_construction_core::macro_def::Onset::Consonant,
        )
        .expect("context");
        let left = parser.trace("Destroy target creature.", &context, TraceLimits::new(1));
        let mut right = left.clone();
        right.analysis = parser.analyze("Destroy target Spirit.", &context);

        let left_private = left.analysis.ownership().expect("left ownership");
        let right_private = right.analysis.ownership().expect("right ownership");
        assert_ne!(
            &left_private.parsed_claims()[1..],
            &right_private.parsed_claims()[1..],
            "the omitted private claim payloads differ",
        );
        assert_eq!(left.selected_lexical_claims().shown(), 1);
        assert_eq!(
            left.selected_lexical_claims(),
            right.selected_lexical_claims()
        );
        assert_eq!(left.ownership(), right.ownership());
        assert_eq!(left.ownership_failures(), right.ownership_failures());
        assert_eq!(left, right);

        let debug = format!("{right:?}");
        assert!(!debug.contains("lexeme:creature_subtype/Spirit/singular"));
        assert!(!debug.contains("analysis:"));
        assert!(debug.contains("selected_lexical_claims"));
    }

    #[test]
    fn parser_trace_debug_redacts_private_analysis_and_raw_error() {
        const PRIVATE_SENTINEL: &str = "PRIVATE_TRACE_DEBUG_SENTINEL";

        let context = ParseContext::new(
            "Trace Card",
            false,
            deckmaste_construction_core::macro_def::Onset::Consonant,
        )
        .expect("context");
        let environment = canonical_test_environment();
        let trace = ParserTrace::from_parts(
            ParseAnalysis::<crate::ast::Ability>::from_result(
                Err(ParseError::Failure {
                    span: TextSpan { start: 0, end: 0 },
                    expectations: BTreeSet::from([Expectation::Literal(PRIVATE_SENTINEL)]),
                }),
                None,
            ),
            StructuralTrace::empty(),
            MaterializationTrace::empty(0),
            TraceLimits::new(0),
            &context,
            &environment,
        );
        let BoundedParseOutcome::ParseFailure(failure) = trace.outcome() else {
            panic!("parse-failure outcome");
        };
        assert_eq!(failure.expectations().total(), 1);
        assert!(failure.expectations().items().is_empty());

        let debug = format!("{trace:?}");
        assert!(!debug.contains(PRIVATE_SENTINEL), "{debug}");
        assert!(!debug.contains("analysis:"), "{debug}");
        assert!(debug.contains("ParseFailure"), "{debug}");
    }

    #[test]
    fn parser_trace_equality_observes_only_the_bounded_public_projection() {
        const LEFT_SENTINEL: &str = "PRIVATE_TRACE_EQUALITY_SENTINEL_LEFT";
        const RIGHT_SENTINEL: &str = "PRIVATE_TRACE_EQUALITY_SENTINEL_RIGHT";

        let context = ParseContext::new(
            "Trace Card",
            false,
            deckmaste_construction_core::macro_def::Onset::Consonant,
        )
        .expect("context");
        let environment = canonical_test_environment();
        let make_trace = |sentinel| {
            ParserTrace::from_parts(
                ParseAnalysis::<crate::ast::Ability>::from_result(
                    Err(ParseError::Failure {
                        span: TextSpan { start: 0, end: 0 },
                        expectations: BTreeSet::from([Expectation::Literal(sentinel)]),
                    }),
                    None,
                ),
                StructuralTrace::empty(),
                MaterializationTrace::empty(0),
                TraceLimits::new(0),
                &context,
                &environment,
            )
        };
        let left = make_trace(LEFT_SENTINEL);
        let right = make_trace(RIGHT_SENTINEL);

        assert_eq!(left.outcome(), right.outcome());
        assert_eq!(left.scanner_matches(), right.scanner_matches());
        assert_eq!(left.chart(), right.chart());
        assert_eq!(left.forest(), right.forest());
        assert_eq!(left.accepted_roots(), right.accepted_roots());
        assert_eq!(
            left.checked_completion_rejections(),
            right.checked_completion_rejections()
        );
        assert_eq!(
            left.materialized_candidates(),
            right.materialized_candidates()
        );
        assert_eq!(
            left.materialization_cycles(),
            right.materialization_cycles()
        );
        assert_eq!(left, right);

        assert_eq!(
            left.into_parse_result(),
            Err(ParseError::Failure {
                span: TextSpan { start: 0, end: 0 },
                expectations: BTreeSet::from([Expectation::Literal(LEFT_SENTINEL)]),
            })
        );
        assert_eq!(
            right.into_parse_result(),
            Err(ParseError::Failure {
                span: TextSpan { start: 0, end: 0 },
                expectations: BTreeSet::from([Expectation::Literal(RIGHT_SENTINEL)]),
            })
        );
    }

    #[test]
    fn structural_trace_family_identity_is_structured_and_ordered() {
        use super::FamilyIdentity;
        use super::FamilyIdentityChild;
        let identity = FamilyIdentity(vec![
            FamilyIdentityChild::Node(7),
            FamilyIdentityChild::Lexical("ScalarNumber(ScalarNumber { magnitude: 3 })".to_owned()),
        ]);
        assert_eq!(identity.0[0], FamilyIdentityChild::Node(7));
        assert_eq!(
            identity.0[1],
            FamilyIdentityChild::Lexical("ScalarNumber(ScalarNumber { magnitude: 3 })".to_owned())
        );
        assert!(identity < FamilyIdentity(vec![FamilyIdentityChild::Node(8)]));
    }
}
