use std::cmp::Ordering;

use super::ParseError;
use crate::ast::Ability;

/// The provisional Stage 4 specificity assigned to one materialized position.
///
/// Roadmap plan 06 replaces this producer with generated specificity metadata
/// while preserving this diagnostic meaning or bumping its machine schema.
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
}
