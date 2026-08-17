use std::cmp::Ordering;

use super::Expectation;
use super::ParseError;
use super::selection::construction_name_v1;
use crate::ast::Ability;
use crate::constructions::Construction;
use crate::context::ParseContext;
use crate::render::Render;

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
pub struct ScannedToken {
    pub(crate) start: usize,
    pub(crate) end: usize,
    pub(crate) terminal_name_v1: String,
    pub(crate) value_label_v1: String,
}

impl ScannedToken {
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
pub(crate) struct SemanticTokenInventory<Terminal, Value> {
    entries: Vec<(usize, usize, Terminal, Value)>,
}

impl<Terminal, Value> Default for SemanticTokenInventory<Terminal, Value> {
    fn default() -> Self {
        Self {
            entries: Vec::new(),
        }
    }
}

impl<Terminal: PartialEq, Value: PartialEq> SemanticTokenInventory<Terminal, Value> {
    pub(crate) fn record(&mut self, start: usize, end: usize, terminal: Terminal, value: Value) {
        let entry = (start, end, terminal, value);
        if !self.entries.contains(&entry) {
            self.entries.push(entry);
        }
    }

    pub(crate) fn into_bounded_by(
        mut self,
        limit: usize,
        mut terminal_cmp: impl FnMut(&Terminal, &Terminal) -> Ordering,
        mut value_cmp: impl FnMut(&Value, &Value) -> Ordering,
        mut terminal_name_v1: impl FnMut(Terminal) -> String,
        mut value_label_v1: impl FnMut(&Value) -> String,
    ) -> Bounded<ScannedToken> {
        order_bounded_prefix(&mut self.entries, limit, |left, right| {
            left.0
                .cmp(&right.0)
                .then_with(|| left.1.cmp(&right.1))
                .then_with(|| terminal_cmp(&left.2, &right.2))
                .then_with(|| value_cmp(&left.3, &right.3))
        });
        let mut tokens = Bounded::new(limit);
        for (start, end, terminal, value) in self.entries {
            tokens.push_with(|| ScannedToken {
                start,
                end,
                terminal_name_v1: terminal_name_v1(terminal),
                value_label_v1: value_label_v1(&value),
            });
        }
        tokens
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
    tokens: Bounded<ScannedToken>,
    chart: Bounded<ChartItem>,
    forest: Bounded<ForestNode>,
    accepted_roots: Bounded<usize>,
    checked_completion_rejections: Bounded<CheckedCompletionRejection>,
}

impl StructuralTrace {
    #[cfg(test)]
    pub(crate) fn empty() -> Self {
        Self {
            tokens: Bounded::new(0),
            chart: Bounded::new(0),
            forest: Bounded::new(0),
            accepted_roots: Bounded::new(0),
            checked_completion_rejections: Bounded::new(0),
        }
    }
    pub(crate) fn new(
        tokens: Bounded<ScannedToken>,
        chart: Bounded<ChartItem>,
        forest: Bounded<ForestNode>,
        accepted_roots: Bounded<usize>,
        checked_completion_rejections: Bounded<CheckedCompletionRejection>,
    ) -> Self {
        Self {
            tokens,
            chart,
            forest,
            accepted_roots,
            checked_completion_rejections,
        }
    }
    pub(crate) const fn tokens(&self) -> &Bounded<ScannedToken> {
        &self.tokens
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
}

impl SelectionCandidate {
    pub(crate) fn new(
        ordinal: usize,
        construction_path: Vec<String>,
        specificity: Vec<SpecificityTier>,
    ) -> Self {
        Self {
            ordinal,
            construction_path,
            specificity,
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

/// A typed internal parser failure suitable for diagnostics.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InternalFailureKind {
    ValidatedRootDidNotMaterialize,
    SelectionConfiguration,
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
pub struct ParseAnalysis {
    result: Result<Ability, ParseError>,
    outcome: ParseAnalysisOutcome,
    decision: Option<SelectionDecision>,
}

impl ParseAnalysis {
    pub(crate) fn from_result(
        result: Result<Ability, ParseError>,
        decision: Option<SelectionDecision>,
    ) -> Self {
        let outcome = match &result {
            Ok(_) => ParseAnalysisOutcome::Selected,
            Err(ParseError::Failure { .. }) => ParseAnalysisOutcome::ParseFailure,
            Err(ParseError::Ambiguous { .. }) => ParseAnalysisOutcome::UnresolvedAmbiguity,
            Err(ParseError::ValidatedRootDidNotMaterialize) => {
                ParseAnalysisOutcome::InternalFailure(
                    InternalFailureKind::ValidatedRootDidNotMaterialize,
                )
            }
            Err(ParseError::InvalidSelectionExceptionConfiguration(_)) => {
                ParseAnalysisOutcome::InternalFailure(InternalFailureKind::SelectionConfiguration)
            }
        };
        Self {
            result,
            outcome,
            decision,
        }
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
    pub fn selected(&self) -> Option<&Ability> {
        self.result.as_ref().ok()
    }

    /// # Errors
    ///
    /// Returns the original parser error for an unselected analysis.
    pub fn into_parse_result(self) -> Result<Ability, ParseError> {
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
            &[Construction::AbilitySpell],
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
}

impl MaterializationTrace {
    pub(crate) fn empty(limit: usize) -> Self {
        Self {
            candidates: Bounded::new(limit),
            cycles: Bounded::new(limit),
        }
    }
    pub(crate) const fn candidates(&self) -> &Bounded<MaterializedCandidateInfo> {
        &self.candidates
    }
    pub(crate) const fn cycles(&self) -> &Bounded<MaterializationCycle> {
        &self.cycles
    }
}

pub(crate) struct MaterializationTraceBuilder {
    limit: usize,
    candidates: Bounded<MaterializedCandidateInfo>,
    cycles: Bounded<MaterializationCycle>,
    cycle_identities: Vec<(usize, Vec<Construction>)>,
}

impl MaterializationTraceBuilder {
    pub(crate) fn new(limits: TraceLimits) -> Self {
        let limit = limits.per_collection();
        Self {
            limit,
            candidates: Bounded::new(limit),
            cycles: Bounded::new(limit),
            cycle_identities: Vec::new(),
        }
    }

    pub(crate) fn record_candidate_with(
        &mut self,
        make_candidate: impl FnOnce() -> MaterializedCandidateInfo,
    ) {
        self.candidates.push_with(make_candidate);
    }

    pub(crate) fn record_cycle(&mut self, node_ordinal: usize, construction_path: &[Construction]) {
        if self.cycle_identities.iter().any(|(seen_node, seen_path)| {
            *seen_node == node_ordinal && seen_path == construction_path
        }) {
            return;
        }
        self.cycle_identities
            .push((node_ordinal, construction_path.to_vec()));
        self.cycles.push_with(|| MaterializationCycle {
            node_ordinal,
            construction_path: bounded_copy(construction_path, self.limit, |construction| {
                construction_name_v1(*construction)
            }),
        });
    }

    pub(crate) fn finish(self) -> MaterializationTrace {
        MaterializationTrace {
            candidates: self.candidates,
            cycles: self.cycles,
        }
    }
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
/// Its [`Debug`](std::fmt::Debug) representation is also a bounded public
/// projection: it formats only the outcome and trace sections available from
/// the public read-only accessors, never the private analysis or raw error.
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
#[derive(Clone, PartialEq, Eq)]
pub struct ParserTrace {
    analysis: ParseAnalysis,
    outcome: BoundedParseOutcome,
    structural: StructuralTrace,
    materialization: MaterializationTrace,
}

impl std::fmt::Debug for ParserTrace {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("ParserTrace")
            .field("outcome", self.outcome())
            .field("tokens", self.tokens())
            .field("chart", self.chart())
            .field("forest", self.forest())
            .field("accepted_roots", self.accepted_roots())
            .field(
                "checked_completion_rejections",
                self.checked_completion_rejections(),
            )
            .field("materialized_candidates", self.materialized_candidates())
            .field("materialization_cycles", self.materialization_cycles())
            .finish()
    }
}

impl ParserTrace {
    pub(crate) fn from_parts(
        analysis: ParseAnalysis,
        structural: StructuralTrace,
        materialization: MaterializationTrace,
        limits: TraceLimits,
        context: &ParseContext<'_>,
    ) -> Self {
        let limit = limits.per_collection();
        let outcome = match &analysis.result {
            Ok(ability) => {
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
                        || ability.render(context),
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
                    message: error.to_string(),
                })
            }
            Err(error @ ParseError::InvalidSelectionExceptionConfiguration(_)) => {
                BoundedParseOutcome::InternalFailure(InternalFailureOutcome {
                    kind: InternalFailureKind::SelectionConfiguration,
                    message: error.to_string(),
                })
            }
        };
        Self {
            analysis,
            outcome,
            structural,
            materialization,
        }
    }

    #[must_use]
    pub const fn outcome(&self) -> &BoundedParseOutcome {
        &self.outcome
    }
    #[must_use]
    pub const fn tokens(&self) -> &Bounded<ScannedToken> {
        self.structural.tokens()
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
    pub const fn materialized_candidates(&self) -> &Bounded<MaterializedCandidateInfo> {
        self.materialization.candidates()
    }
    #[must_use]
    pub const fn materialization_cycles(&self) -> &Bounded<MaterializationCycle> {
        self.materialization.cycles()
    }

    /// # Errors
    ///
    /// Returns the complete original parser error, unaffected by public caps.
    pub fn into_parse_result(self) -> Result<Ability, ParseError> {
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
    use crate::parser::Expectation;
    use crate::parser::ParseError;
    use crate::parser::SelectionExceptionInventoryError;
    use crate::parser::TextSpan;

    #[test]
    fn parser_analysis_keeps_internal_failure_kinds_distinct_from_parse_failure() {
        let materialization =
            ParseAnalysis::from_result(Err(ParseError::ValidatedRootDidNotMaterialize), None);
        let selection = ParseAnalysis::from_result(
            Err(ParseError::InvalidSelectionExceptionConfiguration(
                SelectionExceptionInventoryError::BlankId,
            )),
            None,
        );
        let parse_failure = ParseAnalysis::from_result(
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
    fn parser_trace_internal_failure_projection_keeps_both_kinds_typed_and_messages_stable() {
        let context = ParseContext::new("Trace Card").expect("context");
        for (error, expected_kind, expected_message) in [
            (
                ParseError::ValidatedRootDidNotMaterialize,
                InternalFailureKind::ValidatedRootDidNotMaterialize,
                "validated chart root did not materialize",
            ),
            (
                ParseError::InvalidSelectionExceptionConfiguration(
                    SelectionExceptionInventoryError::BlankId,
                ),
                InternalFailureKind::SelectionConfiguration,
                "invalid selection exception configuration: entry id is empty or whitespace-only",
            ),
        ] {
            let expected_error = error.clone();
            let trace = ParserTrace::from_parts(
                ParseAnalysis::from_result(Err(error), None),
                StructuralTrace::empty(),
                MaterializationTrace::empty(0),
                TraceLimits::new(0),
                &context,
            );
            let BoundedParseOutcome::InternalFailure(failure) = trace.outcome() else {
                panic!("typed internal outcome");
            };
            assert_eq!(failure.kind(), expected_kind);
            assert_eq!(failure.message(), expected_message);
            assert_eq!(trace.into_parse_result(), Err(expected_error));
        }
    }

    #[test]
    fn parser_trace_debug_redacts_private_analysis_and_raw_error() {
        const PRIVATE_SENTINEL: &str = "PRIVATE_TRACE_DEBUG_SENTINEL";

        let context = ParseContext::new("Trace Card").expect("context");
        let trace = ParserTrace::from_parts(
            ParseAnalysis::from_result(
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
    fn structural_trace_family_identity_is_structured_and_ordered() {
        use super::FamilyIdentity;
        use super::FamilyIdentityChild;
        let identity = FamilyIdentity(vec![
            FamilyIdentityChild::Node(7),
            FamilyIdentityChild::Lexical(
                "SignedNumber(SignedNumber { sign: Negative, magnitude: 3 })".to_owned(),
            ),
        ]);
        assert_eq!(identity.0[0], FamilyIdentityChild::Node(7));
        assert_eq!(
            identity.0[1],
            FamilyIdentityChild::Lexical(
                "SignedNumber(SignedNumber { sign: Negative, magnitude: 3 })".to_owned()
            )
        );
        assert!(identity < FamilyIdentity(vec![FamilyIdentityChild::Node(8)]));
    }
}
