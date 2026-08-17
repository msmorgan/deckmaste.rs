use std::cmp::Ordering;

use super::ParseError;
use crate::ast::Ability;

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
pub(crate) struct ScannedToken {
    pub(crate) start: usize,
    pub(crate) end: usize,
    pub(crate) terminal_name_v1: String,
    pub(crate) value_label_v1: String,
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
    #[allow(dead_code, reason = "Task 6 owns the public structural trace facade.")]
    pub(crate) fn tokens(&self) -> &Bounded<ScannedToken> {
        &self.tokens
    }
    #[allow(dead_code, reason = "Task 6 owns the public structural trace facade.")]
    pub(crate) fn chart(&self) -> &Bounded<ChartItem> {
        &self.chart
    }
    #[allow(dead_code, reason = "Task 6 owns the public structural trace facade.")]
    pub(crate) fn forest(&self) -> &Bounded<ForestNode> {
        &self.forest
    }
    #[allow(dead_code, reason = "Task 6 owns the public structural trace facade.")]
    pub(crate) fn accepted_roots(&self) -> &Bounded<usize> {
        &self.accepted_roots
    }
    #[allow(dead_code, reason = "Task 6 owns the public structural trace facade.")]
    pub(crate) fn checked_completion_rejections(&self) -> &Bounded<CheckedCompletionRejection> {
        &self.checked_completion_rejections
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct ChartItem {
    pub(crate) column: usize,
    pub(crate) rule_name_v1: String,
    pub(crate) dot: usize,
    pub(crate) origin: usize,
    pub(crate) family_count: usize,
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct ForestNode {
    pub(crate) id: usize,
    pub(crate) rule_name_v1: String,
    pub(crate) start: usize,
    pub(crate) end: usize,
    pub(crate) families: Bounded<ForestFamily>,
}
impl ForestNode {
    pub(crate) fn families(&self) -> &Bounded<ForestFamily> {
        &self.families
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct ForestFamily {
    pub(crate) children: Bounded<ForestChild>,
}
impl ForestFamily {
    pub(crate) fn children(&self) -> &Bounded<ForestChild> {
        &self.children
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct ForestChild {
    pub(crate) node_id: Option<usize>,
    pub(crate) value_label_v1: Option<String>,
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct CheckedCompletionRejection {
    pub(crate) rule_name_v1: String,
    pub(crate) start: usize,
    pub(crate) end: usize,
    pub(crate) family_identity_v1: FamilyIdentity,
}

#[derive(Debug, Clone, PartialEq, Eq, Ord, PartialOrd)]
pub(crate) struct FamilyIdentity(pub(crate) Vec<FamilyIdentityChild>);

#[derive(Debug, Clone, PartialEq, Eq, Ord, PartialOrd)]
pub(crate) enum FamilyIdentityChild {
    Node(usize),
    Lexical(String),
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

#[cfg(test)]
mod tests {
    use std::collections::BTreeSet;

    use super::Bounded;
    use super::InternalFailureKind;
    use super::ParseAnalysis;
    use super::ParseAnalysisOutcome;
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
