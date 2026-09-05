use std::cmp::Ordering;

use super::ParseError;
use super::SelectionCandidate;
use super::SelectionComparison;
use super::SelectionDecision;
use super::SelectionDecisive;
use super::SelectionResolution;
use super::SpecificityTier;
use super::engine::RulePosition;
use super::materialize::Candidate;
use crate::constructions::Construction;
use crate::constructions::Lexical;

const SELECTION_EXCEPTIONS: &[SelectionException<Construction>] = &[];

/// Stable public metadata for one validated selection exception.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SelectionExceptionInfo {
    /// Stable registry identifier.
    pub id: &'static str,
    /// Lexically first construction name in the exception pair.
    pub left: &'static str,
    /// Lexically second construction name in the exception pair.
    pub right: &'static str,
    /// Explicit construction name that wins the exception pair.
    pub winner: &'static str,
    /// Human-readable justification for the exception.
    pub rationale: &'static str,
}

/// Reports an invalid typed selection-exception registry.
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum SelectionExceptionInventoryError {
    /// An entry identifier was empty or whitespace-only.
    #[error("invalid selection exception configuration: entry id is empty or whitespace-only")]
    BlankId,
    /// Two entries use the same identifier.
    #[error("invalid selection exception configuration: duplicate entry id `{id}`")]
    DuplicateId { id: &'static str },
    /// Two entries describe the same unordered construction pair.
    #[error(
        "invalid selection exception configuration: duplicate unordered pair `{left}` and `{right}`"
    )]
    DuplicatePair {
        left: &'static str,
        right: &'static str,
    },
    /// An entry describes the same construction on both sides.
    #[error(
        "invalid selection exception configuration: self-pair for construction `{construction}`"
    )]
    SelfPair { construction: &'static str },
    /// An entry rationale was empty or whitespace-only.
    #[error(
        "invalid selection exception configuration: entry `{id}` has an empty or whitespace-only rationale"
    )]
    BlankRationale { id: &'static str },
    /// An entry winner is not one of its two constructions.
    #[error(
        "invalid selection exception configuration: entry `{id}` names a winner outside its pair"
    )]
    WinnerOutsidePair { id: &'static str },
}

#[derive(Debug, Clone, Copy)]
struct SelectionException<C> {
    id: &'static str,
    left: C,
    right: C,
    winner: C,
    rationale: &'static str,
}

/// Returns the validated, public metadata for selection exceptions.
///
/// # Errors
///
/// Returns an error when the internal selection exception registry is invalid.
pub fn selection_exception_inventory()
-> Result<Vec<SelectionExceptionInfo>, SelectionExceptionInventoryError> {
    selection_exception_inventory_for(SELECTION_EXCEPTIONS, Construction::name)
}

/// Builds one decision through the real exception-selection algorithm for
/// cross-crate gate tests.
///
/// # Panics
///
/// Panics if the sealed fixture stops producing a valid exception-resolved
/// decision.
#[doc(hidden)]
#[cfg(feature = "test-support")]
pub fn exception_decision_for_test() -> SelectionDecision {
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    enum FixtureConstruction {
        Left,
        Right,
    }

    struct FixtureCandidate {
        construction: FixtureConstruction,
        positions: Vec<RulePosition<(), ()>>,
    }

    const fn name(construction: FixtureConstruction) -> &'static str {
        match construction {
            FixtureConstruction::Left => "FixtureLeft",
            FixtureConstruction::Right => "FixtureRight",
        }
    }

    let candidates = [FixtureConstruction::Left, FixtureConstruction::Right]
        .into_iter()
        .map(|construction| FixtureCandidate {
            construction,
            positions: vec![RulePosition::Nonterminal(())],
        })
        .collect();
    let exceptions = [SelectionException {
        id: "fixture-left-over-right",
        left: FixtureConstruction::Left,
        right: FixtureConstruction::Right,
        winner: FixtureConstruction::Left,
        rationale: "authenticate exception evidence propagation",
    }];
    let (_, decision) = selection_analysis_with_exceptions(
        candidates,
        |candidate| std::slice::from_ref(&candidate.construction),
        |candidate| structural_specificity(&candidate.positions, |()| false, |()| false),
        |_| Vec::new(),
        |construction| name(construction).to_owned(),
        name,
        &exceptions,
    )
    .expect("the test exception inventory is valid")
    .into_result_and_decision();
    decision.expect("the test candidates produce a selection decision")
}

fn selection_exception_inventory_for<C>(
    exceptions: &[SelectionException<C>],
    construction_name: impl Fn(C) -> &'static str,
) -> Result<Vec<SelectionExceptionInfo>, SelectionExceptionInventoryError>
where
    C: Copy + Eq,
{
    validate_selection_exceptions(exceptions, &construction_name)?;
    Ok(exceptions
        .iter()
        .map(|exception| {
            let (left, right) = canonical_pair(
                construction_name(exception.left),
                construction_name(exception.right),
            );
            SelectionExceptionInfo {
                id: exception.id,
                left,
                right,
                winner: construction_name(exception.winner),
                rationale: exception.rationale,
            }
        })
        .collect())
}

fn validate_selection_exceptions<C>(
    exceptions: &[SelectionException<C>],
    construction_name: &impl Fn(C) -> &'static str,
) -> Result<(), SelectionExceptionInventoryError>
where
    C: Copy + Eq,
{
    for (index, exception) in exceptions.iter().enumerate() {
        if exception.id.trim().is_empty() {
            return Err(SelectionExceptionInventoryError::BlankId);
        }
        if exceptions[..index]
            .iter()
            .any(|previous| previous.id == exception.id)
        {
            return Err(SelectionExceptionInventoryError::DuplicateId { id: exception.id });
        }
        if exception.left == exception.right {
            return Err(SelectionExceptionInventoryError::SelfPair {
                construction: construction_name(exception.left),
            });
        }
        if exceptions[..index].iter().any(|previous| {
            (previous.left == exception.left && previous.right == exception.right)
                || (previous.left == exception.right && previous.right == exception.left)
        }) {
            let (left, right) = canonical_pair(
                construction_name(exception.left),
                construction_name(exception.right),
            );
            return Err(SelectionExceptionInventoryError::DuplicatePair { left, right });
        }
        if exception.rationale.trim().is_empty() {
            return Err(SelectionExceptionInventoryError::BlankRationale { id: exception.id });
        }
        if exception.winner != exception.left && exception.winner != exception.right {
            return Err(SelectionExceptionInventoryError::WinnerOutsidePair { id: exception.id });
        }
    }
    Ok(())
}

fn canonical_pair(left: &'static str, right: &'static str) -> (&'static str, &'static str) {
    if left <= right { (left, right) } else { (right, left) }
}

#[derive(Debug, Clone, PartialEq, Eq, Ord, PartialOrd)]
struct Specificity(Vec<SpecificityTier>);

pub(crate) struct SelectionAnalysis<T> {
    selected: Option<T>,
    decision: Option<SelectionDecision>,
    ambiguity: Option<ParseError>,
}

impl<T> SelectionAnalysis<T> {
    #[cfg(test)]
    pub(crate) fn decision(&self) -> Option<&SelectionDecision> {
        self.decision.as_ref()
    }

    pub(crate) fn into_result_and_decision(
        self,
    ) -> (Result<Option<T>, ParseError>, Option<SelectionDecision>) {
        let result = self.ambiguity.map_or_else(|| Ok(self.selected), Err);
        (result, self.decision)
    }
}

pub(crate) fn analyze_selection<V>(
    candidates: Vec<Candidate<V>>,
) -> Result<SelectionAnalysis<Candidate<V>>, ParseError> {
    selection_analysis_with_exceptions(
        candidates,
        |candidate| candidate.constructions.as_slice(),
        |candidate| Specificity(candidate.specificity.clone()),
        |candidate| candidate.leaf_path.clone(),
        construction_name_v1,
        Construction::name,
        SELECTION_EXCEPTIONS,
    )
}

fn selection_analysis_with_exceptions<T, C, Name>(
    candidates: Vec<T>,
    constructions: impl Fn(&T) -> &[C],
    specificity: impl Fn(&T) -> Specificity,
    leaf_path: impl Fn(&T) -> Vec<String>,
    construction_name: Name,
    legacy_construction_name: impl Fn(C) -> &'static str,
    exceptions: &[SelectionException<C>],
) -> Result<SelectionAnalysis<T>, ParseError>
where
    C: Copy + Eq,
    Name: Fn(C) -> String,
{
    validate_selection_exceptions(exceptions, &legacy_construction_name)
        .map_err(ParseError::InvalidSelectionExceptionConfiguration)?;
    if candidates.is_empty() {
        return Ok(SelectionAnalysis {
            selected: None,
            decision: None,
            ambiguity: None,
        });
    }
    Ok(decide_ranked(
        candidates,
        constructions,
        specificity,
        leaf_path,
        construction_name,
        legacy_construction_name,
        exceptions,
    ))
}

#[cfg(test)]
fn select_with_exceptions<T, C>(
    candidates: Vec<T>,
    constructions: impl Fn(&T) -> &[C],
    specificity: impl Fn(&T) -> Specificity,
    construction_name: impl Fn(C) -> &'static str,
    exceptions: &[SelectionException<C>],
) -> Result<Option<T>, ParseError>
where
    C: Copy + Eq,
{
    selection_analysis_with_exceptions(
        candidates,
        constructions,
        specificity,
        |_| Vec::new(),
        |construction| construction_name(construction).to_owned(),
        &construction_name,
        exceptions,
    )
    .and_then(|analysis| analysis.into_result_and_decision().0)
}

#[cfg(test)]
fn select_ranked<T, C>(
    candidates: Vec<T>,
    constructions: impl Fn(&T) -> &[C],
    specificity: impl Fn(&T) -> Specificity,
    construction_name: impl Fn(C) -> &'static str,
    exceptions: &[SelectionException<C>],
) -> Result<T, ParseError>
where
    C: Copy + Eq,
{
    let selected = select_with_exceptions(
        candidates,
        constructions,
        specificity,
        construction_name,
        exceptions,
    )?
    .expect("select_ranked requires at least one candidate");
    Ok(selected)
}

fn structural_specificity<N, L>(
    positions: &[RulePosition<N, L>],
    is_literal: impl Fn(&L) -> bool,
    is_identity: impl Fn(&L) -> bool,
) -> Specificity {
    Specificity(
        positions
            .iter()
            .map(|position| match position {
                RulePosition::Lexical(lexical) if is_literal(lexical) => SpecificityTier::Literal,
                RulePosition::Lexical(lexical) if is_identity(lexical) => SpecificityTier::Identity,
                RulePosition::Lexical(_) => SpecificityTier::TypedLexical,
                RulePosition::Nonterminal(_) | RulePosition::AdjacentNonterminal(_) => {
                    SpecificityTier::Nonterminal
                }
            })
            .collect(),
    )
}

pub(crate) fn specificity_tiers(
    positions: &[RulePosition<crate::constructions::Category, Lexical>],
) -> Vec<SpecificityTier> {
    structural_specificity(
        positions,
        |lexical| matches!(lexical, Lexical::Literal(_)),
        |lexical| lexical.is_identity(),
    )
    .0
}

fn decide_ranked<T, C, Name>(
    candidates: Vec<T>,
    constructions: impl Fn(&T) -> &[C],
    specificity: impl Fn(&T) -> Specificity,
    leaf_path: impl Fn(&T) -> Vec<String>,
    construction_name: Name,
    legacy_construction_name: impl Fn(C) -> &'static str,
    exceptions: &[SelectionException<C>],
) -> SelectionAnalysis<T>
where
    C: Copy + Eq,
    Name: Fn(C) -> String,
{
    let specifics = candidates.iter().map(&specificity).collect::<Vec<_>>();
    let best_specificity = specifics
        .iter()
        .max()
        .expect("selection requires at least one candidate");
    let candidates = candidates
        .into_iter()
        .enumerate()
        .map(|(ordinal, candidate)| RankedCandidate {
            candidate,
            ordinal,
            specificity: specifics[ordinal].clone(),
        })
        .collect::<Vec<_>>();
    let diagnostic_candidates = candidates
        .iter()
        .map(|candidate| {
            SelectionCandidate::new(
                candidate.ordinal,
                constructions(&candidate.candidate)
                    .iter()
                    .copied()
                    .map(&construction_name)
                    .collect(),
                candidate.specificity.0.clone(),
                leaf_path(&candidate.candidate),
            )
        })
        .collect::<Vec<_>>();
    let maximum = candidates
        .iter()
        .filter(|candidate| candidate.specificity == *best_specificity)
        .map(|candidate| candidate.ordinal)
        .collect::<Vec<_>>();

    let mut comparisons = Vec::new();
    let mut exception_uses = Vec::new();
    for left in 0..candidates.len() {
        for right in left + 1..candidates.len() {
            let (mut ordering, decisive) = compare_specificity(
                &candidates[left].specificity,
                &candidates[right].specificity,
            );
            let mut exception_id = None;
            if decisive == SelectionDecisive::Tie
                && maximum.contains(&left)
                && maximum.contains(&right)
                && let Some((id, exception_order)) = exception_order(
                    constructions(&candidates[left].candidate),
                    constructions(&candidates[right].candidate),
                    exceptions,
                )
            {
                ordering = exception_order;
                exception_id = Some(id.to_owned());
                exception_uses.push(id.to_owned());
            }
            comparisons.push(SelectionComparison::new(
                left,
                right,
                ordering,
                decisive,
                exception_id,
            ));
        }
    }

    let survivors = maximum
        .iter()
        .copied()
        .filter(|&ordinal| {
            maximum.iter().copied().all(|other| {
                other == ordinal
                    || comparison_ordering(&comparisons, ordinal, other) != Ordering::Less
            })
        })
        .collect::<Vec<_>>();
    let (selected, resolution) = if candidates.len() == 1 {
        (Some(0), SelectionResolution::Unique)
    } else if maximum.len() == 1 {
        (maximum.first().copied(), SelectionResolution::Specificity)
    } else if survivors.len() == 1 {
        (survivors.first().copied(), SelectionResolution::Exception)
    } else {
        (None, SelectionResolution::UnresolvedTie)
    };
    let decision = SelectionDecision::new(
        diagnostic_candidates,
        comparisons,
        survivors,
        selected,
        resolution,
        exception_uses,
    );
    if let Some(ordinal) = selected {
        let candidate = candidates
            .into_iter()
            .find(|candidate| candidate.ordinal == ordinal)
            .expect("selected ordinal is a materialized candidate")
            .candidate;
        return SelectionAnalysis {
            selected: Some(candidate),
            decision: Some(decision),
            ambiguity: None,
        };
    }
    let tied = if decision.survivors().is_empty() {
        maximum
    } else {
        decision.survivors().to_vec()
    };
    let (first, second) = ambiguity_names(
        tied.iter()
            .map(|&ordinal| constructions(&candidates[ordinal].candidate)),
        legacy_construction_name,
    );
    SelectionAnalysis {
        selected: None,
        decision: Some(decision),
        ambiguity: Some(ParseError::Ambiguous { first, second }),
    }
}

struct RankedCandidate<T> {
    candidate: T,
    ordinal: usize,
    specificity: Specificity,
}

fn compare_specificity(left: &Specificity, right: &Specificity) -> (Ordering, SelectionDecisive) {
    for (index, (left_tier, right_tier)) in left.0.iter().zip(&right.0).enumerate() {
        let ordering = left_tier.cmp(right_tier);
        if ordering != Ordering::Equal {
            return (ordering, SelectionDecisive::Position(index));
        }
    }
    if left.0.len() == right.0.len() {
        (Ordering::Equal, SelectionDecisive::Tie)
    } else {
        (
            left.0.len().cmp(&right.0.len()),
            SelectionDecisive::VectorExhaustion(left.0.len().min(right.0.len())),
        )
    }
}

fn comparison_ordering(comparisons: &[SelectionComparison], left: usize, right: usize) -> Ordering {
    let comparison = comparisons
        .iter()
        .find(|comparison| {
            (comparison.left_ordinal() == left && comparison.right_ordinal() == right)
                || (comparison.left_ordinal() == right && comparison.right_ordinal() == left)
        })
        .expect("every unordered pair has exactly one comparison");
    if comparison.left_ordinal() == left {
        comparison.ordering()
    } else {
        comparison.ordering().reverse()
    }
}

fn exception_order<C: Copy + Eq>(
    left: &[C],
    right: &[C],
    exceptions: &[SelectionException<C>],
) -> Option<(&'static str, Ordering)> {
    let (left, right) = first_difference(left, right)?;
    exceptions.iter().find_map(|exception| {
        ((left, right) == (exception.left, exception.right)
            || (left, right) == (exception.right, exception.left))
            .then_some((
                exception.id,
                if exception.winner == left { Ordering::Greater } else { Ordering::Less },
            ))
    })
}

fn first_difference<C: Copy + Eq>(left: &[C], right: &[C]) -> Option<(C, C)> {
    left.iter()
        .copied()
        .zip(right.iter().copied())
        .find(|(left, right)| left != right)
}

fn ambiguity_names<'a, C: Copy + Eq + 'a>(
    paths: impl Iterator<Item = &'a [C]>,
    construction_name: impl Fn(C) -> &'static str,
) -> (&'static str, &'static str) {
    let paths = paths.collect::<Vec<_>>();
    let mut pairs = Vec::new();
    for (left_index, left) in paths.iter().enumerate() {
        for right in &paths[left_index + 1..] {
            if let Some((left, right)) = first_difference(left, right) {
                let mut pair = (construction_name(left), construction_name(right));
                if pair.0 > pair.1 {
                    pair = (pair.1, pair.0);
                }
                pairs.push(pair);
            }
        }
    }
    pairs.sort_unstable();
    pairs.into_iter().next().unwrap_or_else(|| {
        let name = paths
            .first()
            .and_then(|path| path.last())
            .copied()
            .map_or("UnknownConstruction", &construction_name);
        (name, name)
    })
}

pub(crate) fn construction_name_v1(construction: Construction) -> String {
    construction.name().to_owned()
}

#[cfg(test)]
fn generated_constructions_v1() -> Vec<Construction> {
    let mut constructions = Vec::new();
    for rule in crate::constructions::RULES {
        let Some(construction) = rule.id.public_construction() else {
            continue;
        };
        if !constructions.contains(&construction) {
            constructions.push(construction);
        }
    }
    constructions
}

#[cfg(test)]
mod tests {
    use std::cmp::Ordering;
    use std::collections::BTreeSet;

    use super::super::Bounded;
    use super::super::diagnostic::BoundedParseOutcome;
    use super::super::diagnostic::BoundedSelectionDecision;
    use super::super::diagnostic::MaterializationTrace;
    use super::super::diagnostic::ParserTrace;
    use super::super::diagnostic::SelectionLoserReason;
    use super::super::diagnostic::StructuralTrace;
    use super::super::diagnostic::TraceLimits;
    use super::SelectionAnalysis;
    use super::SelectionDecision;
    use super::SelectionDecisive;
    use super::SelectionException;
    use super::SelectionExceptionInventoryError;
    use super::SelectionResolution;
    use super::SpecificityTier;
    use super::analyze_selection;
    use super::generated_constructions_v1;
    use super::select_ranked;
    use super::select_with_exceptions;
    use super::selection_analysis_with_exceptions;
    use super::selection_exception_inventory;
    use super::selection_exception_inventory_for;
    use super::structural_specificity;
    use crate::constructions::Category;
    use crate::constructions::Construction;
    use crate::constructions::Leaf;
    use crate::constructions::Lexical;
    use crate::constructions::LexicalOwner;
    use crate::constructions::LexicalProvenanceKind;
    use crate::context::ParseContext;
    use crate::environment::canonical_test_environment;
    use crate::parser::ParseAnalysis;
    use crate::parser::ParseError;
    use crate::parser::TextSpan;
    use crate::parser::engine::Child;
    use crate::parser::engine::Forest;
    use crate::parser::engine::LexicalMatch;
    use crate::parser::engine::PackedNode;
    use crate::parser::engine::Rule;
    use crate::parser::engine::RulePosition;
    use crate::parser::engine::parse;
    use crate::parser::materialize::Candidate;
    use crate::parser::materialize::materialize_with;
    use crate::parser::ownership::RawLexicalClaim;

    #[test]
    fn selected_candidate_ownership_returns_the_exact_candidate() {
        let environment = canonical_test_environment();
        let context = ParseContext::new(
            "Context Card",
            false,
            deckmaste_construction_core::macro_def::Onset::Consonant,
        )
        .unwrap();
        let ability = crate::parser::Parser::new(environment)
            .unwrap()
            .parse("Destroy target creature.", &context)
            .unwrap();
        let candidate = |specificity, owner_id: &'static str| Candidate {
            value: ability.clone(),
            constructions: vec![Construction::AbilityPlain],
            positions: Vec::new(),
            specificity: vec![specificity],
            leaf_path: Vec::new(),
            claims: vec![RawLexicalClaim {
                span: TextSpan { start: 0, end: 1 },
                value: Leaf::Literal("x"),
                owner: LexicalOwner::static_owner(LexicalProvenanceKind::FormLiteral, owner_id),
            }],
            synthetic_claims: Vec::new(),
        };
        let analysis: Result<SelectionAnalysis<Candidate>, ParseError> = analyze_selection(vec![
            candidate(SpecificityTier::TypedLexical, "loser"),
            candidate(SpecificityTier::Literal, "winner"),
        ]);
        let (selected, _) = analysis
            .expect("ranked candidate analysis is valid")
            .into_result_and_decision();
        let selected = selected
            .expect("selection result")
            .expect("one candidate wins");
        assert_eq!(selected.claims[0].owner.stable_id(), "winner");

        let unresolved = crate::parser::analyze_materialized(vec![
            candidate(SpecificityTier::Literal, "tie-a"),
            candidate(SpecificityTier::Literal, "tie-b"),
        ]);
        assert!(unresolved.ownership().is_none());
        assert!(matches!(
            unresolved.into_parse_result(),
            Err(ParseError::Ambiguous { .. })
        ));
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq, Ord, PartialOrd)]
    enum TestCategory {
        Start,
        Left,
        Right,
        Empty,
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq, Ord, PartialOrd)]
    enum TestLexical {
        LiteralAlpha,
        Word,
        Identity,
    }

    #[test]
    fn the_claim_kind_order_ranks_an_identity_above_a_typed_lexical() {
        let tier = |lexical| {
            structural_specificity(
                &[RulePosition::<TestCategory, TestLexical>::Lexical(lexical)],
                |lexical| matches!(lexical, TestLexical::LiteralAlpha),
                |lexical| matches!(lexical, TestLexical::Identity),
            )
            .0[0]
        };
        assert_eq!(tier(TestLexical::Word), SpecificityTier::TypedLexical);
        assert_eq!(tier(TestLexical::Identity), SpecificityTier::Identity);
        assert_eq!(tier(TestLexical::LiteralAlpha), SpecificityTier::Literal);
        assert!(SpecificityTier::TypedLexical < SpecificityTier::Identity);
        assert!(SpecificityTier::Identity < SpecificityTier::Literal);
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq, Ord, PartialOrd)]
    enum TestConstruction {
        LiteralAlpha,
        BroadAlpha,
        SharedForm,
        TestLeft,
        TestRight,
        TestThird,
        LateLiteral,
        EarlyTyped,
        LateLiteralChild,
        EarlyTypedChild,
        Empty,
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    enum TestRuleId {
        LiteralAlpha,
        BroadAlpha,
        SharedLiteral,
        SharedTyped,
        TestLeft,
        TestRight,
        LateLiteral,
        EarlyTyped,
        LateLiteralChild,
        EarlyTypedChild,
        Empty,
    }

    #[derive(Debug, Clone, PartialEq, Eq)]
    enum TestBuildValue {
        Ability(&'static str),
        Leaf(Lexical),
    }

    impl TestRuleId {
        const fn construction(self) -> TestConstruction {
            match self {
                Self::LiteralAlpha => TestConstruction::LiteralAlpha,
                Self::BroadAlpha => TestConstruction::BroadAlpha,
                Self::SharedLiteral | Self::SharedTyped => TestConstruction::SharedForm,
                Self::TestLeft => TestConstruction::TestLeft,
                Self::TestRight => TestConstruction::TestRight,
                Self::LateLiteral => TestConstruction::LateLiteral,
                Self::EarlyTyped => TestConstruction::EarlyTyped,
                Self::LateLiteralChild => TestConstruction::LateLiteralChild,
                Self::EarlyTypedChild => TestConstruction::EarlyTypedChild,
                Self::Empty => TestConstruction::Empty,
            }
        }

        const fn positions(self) -> &'static [RulePosition<TestCategory, TestLexical>] {
            match self {
                Self::LiteralAlpha | Self::SharedLiteral => {
                    &[RulePosition::Lexical(TestLexical::LiteralAlpha)]
                }
                Self::BroadAlpha
                | Self::SharedTyped
                | Self::TestLeft
                | Self::TestRight
                | Self::EarlyTypedChild => &[RulePosition::Lexical(TestLexical::Word)],
                Self::LateLiteral => &[RulePosition::Nonterminal(TestCategory::Left)],
                Self::EarlyTyped => &[RulePosition::Nonterminal(TestCategory::Right)],
                Self::LateLiteralChild => &[
                    RulePosition::Nonterminal(TestCategory::Empty),
                    RulePosition::Lexical(TestLexical::LiteralAlpha),
                ],
                Self::Empty => &[],
            }
        }

        const fn index(self) -> usize {
            match self {
                Self::SharedLiteral => 0,
                Self::SharedTyped => 1,
                Self::LiteralAlpha
                | Self::BroadAlpha
                | Self::TestLeft
                | Self::TestRight
                | Self::LateLiteral
                | Self::EarlyTyped
                | Self::LateLiteralChild
                | Self::EarlyTypedChild
                | Self::Empty => panic!("only shared-form rules are materialized"),
            }
        }
    }

    #[derive(Debug, Clone, PartialEq, Eq)]
    struct TestCandidate {
        ability: &'static str,
        construction: TestConstruction,
        constructions: Vec<TestConstruction>,
        positions: Vec<RulePosition<TestCategory, TestLexical>>,
    }

    const SPECIFICITY_RULES: &[Rule<TestCategory, TestLexical, TestRuleId>] = &[
        Rule {
            id: TestRuleId::LiteralAlpha,
            lhs: TestCategory::Start,
            rhs: &[RulePosition::Lexical(TestLexical::LiteralAlpha)],
        },
        Rule {
            id: TestRuleId::BroadAlpha,
            lhs: TestCategory::Start,
            rhs: &[RulePosition::Lexical(TestLexical::Word)],
        },
    ];

    const TIE_RULES: &[Rule<TestCategory, TestLexical, TestRuleId>] = &[
        Rule {
            id: TestRuleId::TestLeft,
            lhs: TestCategory::Start,
            rhs: &[RulePosition::Lexical(TestLexical::Word)],
        },
        Rule {
            id: TestRuleId::TestRight,
            lhs: TestCategory::Start,
            rhs: &[RulePosition::Lexical(TestLexical::Word)],
        },
    ];

    const SHARED_FORM_RULES: &[Rule<Category, Lexical, TestRuleId>] = &[
        Rule {
            id: TestRuleId::SharedLiteral,
            lhs: Category::Ability,
            rhs: &[RulePosition::Lexical(Lexical::Literal("alpha"))],
        },
        Rule {
            id: TestRuleId::SharedTyped,
            lhs: Category::Ability,
            rhs: &[RulePosition::Lexical(Lexical::Variable)],
        },
    ];

    const RECURSIVE_SPECIFICITY_RULES: &[Rule<TestCategory, TestLexical, TestRuleId>] = &[
        Rule {
            id: TestRuleId::LateLiteral,
            lhs: TestCategory::Start,
            rhs: &[RulePosition::Nonterminal(TestCategory::Left)],
        },
        Rule {
            id: TestRuleId::EarlyTyped,
            lhs: TestCategory::Start,
            rhs: &[RulePosition::Nonterminal(TestCategory::Right)],
        },
        Rule {
            id: TestRuleId::LateLiteralChild,
            lhs: TestCategory::Left,
            rhs: &[
                RulePosition::Nonterminal(TestCategory::Empty),
                RulePosition::Lexical(TestLexical::LiteralAlpha),
            ],
        },
        Rule {
            id: TestRuleId::EarlyTypedChild,
            lhs: TestCategory::Right,
            rhs: &[RulePosition::Lexical(TestLexical::Word)],
        },
        Rule {
            id: TestRuleId::Empty,
            lhs: TestCategory::Empty,
            rhs: &[],
        },
    ];

    fn parse_and_select_specificity_toy(text: &str) -> Result<TestCandidate, ParseError> {
        parse_and_select_toy(SPECIFICITY_RULES, text)
    }

    fn parse_and_select_tie_toy(text: &str) -> Result<TestCandidate, ParseError> {
        parse_and_select_toy(TIE_RULES, text)
    }

    #[allow(
        clippy::type_complexity,
        reason = "the explicit test type avoids recreating generated authority through an alias"
    )]
    fn parse_and_materialize_shared_forms(
        text: &str,
    ) -> (
        usize,
        Vec<
            crate::parser::materialize::MaterializedCandidate<
                TestBuildValue,
                Construction,
                Category,
                Lexical,
                Lexical,
                &'static str,
            >,
        >,
    ) {
        let forest = parse(
            SHARED_FORM_RULES,
            Category::Ability,
            text.len(),
            |lexical, offset| {
                (offset == 0 && text == "alpha")
                    .then_some(LexicalMatch {
                        end: text.len(),
                        value: lexical,
                        owner: Some(match lexical {
                            Lexical::Literal(_) => "literal-owner",
                            Lexical::Variable => "typed-owner",
                            _ => unreachable!("shared-form fixture has two terminals"),
                        }),
                    })
                    .into_iter()
                    .collect()
            },
            |_, _, _| true,
        )
        .expect("the shared-form toy grammar accepts alpha");
        let roots = forest.accepted_roots().collect::<Vec<_>>();
        let candidates = materialize_with(
            &forest,
            SHARED_FORM_RULES,
            TestRuleId::index,
            |_| Some(Construction::AmountNumber),
            std::convert::identity,
            |leaf| TestBuildValue::Leaf(*leaf),
            |rule, children| match (rule, children) {
                (TestRuleId::SharedLiteral, [TestBuildValue::Leaf(Lexical::Literal("alpha"))]) => {
                    Ok(Some(TestBuildValue::Ability("literal form")))
                }
                (TestRuleId::SharedTyped, [TestBuildValue::Leaf(Lexical::Variable)]) => {
                    Ok(Some(TestBuildValue::Ability("typed form")))
                }
                _ => Ok(None),
            },
        );
        (roots.len(), candidates)
    }

    fn parse_and_select_recursive_specificity_toy(text: &str) -> Result<TestCandidate, ParseError> {
        parse_and_select_toy(RECURSIVE_SPECIFICITY_RULES, text)
    }

    fn parse_and_select_toy(
        rules: &'static [Rule<TestCategory, TestLexical, TestRuleId>],
        text: &str,
    ) -> Result<TestCandidate, ParseError> {
        let forest = parse(
            rules,
            TestCategory::Start,
            text.len(),
            |lexical, offset| {
                (offset == 0 && text == "alpha")
                    .then_some(LexicalMatch {
                        end: text.len(),
                        value: lexical,
                        owner: None::<()>,
                    })
                    .into_iter()
                    .collect()
            },
            |_, _, _| true,
        )
        .expect("the toy grammar accepts alpha");
        let candidates = forest
            .accepted_roots()
            .map(|root| candidate_from_root(&forest, root))
            .collect::<Vec<_>>();
        assert!(
            candidates
                .windows(2)
                .all(|pair| pair[0].ability == pair[1].ability)
        );

        select_test_candidates(candidates)
    }

    fn select_test_candidates(candidates: Vec<TestCandidate>) -> Result<TestCandidate, ParseError> {
        select_ranked(
            candidates,
            |candidate| candidate.constructions.as_slice(),
            |candidate| {
                structural_specificity(
                    &candidate.positions,
                    |lexical| matches!(lexical, TestLexical::LiteralAlpha),
                    |lexical| matches!(lexical, TestLexical::Identity),
                )
            },
            construction_name,
            &[],
        )
    }

    fn candidate_from_root(
        forest: &Forest<TestRuleId, TestLexical>,
        root: &PackedNode<TestRuleId, TestLexical>,
    ) -> TestCandidate {
        let mut candidate = TestCandidate {
            ability: match root.rule {
                TestRuleId::SharedLiteral => "literal form",
                TestRuleId::SharedTyped => "typed form",
                TestRuleId::LiteralAlpha
                | TestRuleId::BroadAlpha
                | TestRuleId::TestLeft
                | TestRuleId::TestRight
                | TestRuleId::LateLiteral
                | TestRuleId::EarlyTyped
                | TestRuleId::LateLiteralChild
                | TestRuleId::EarlyTypedChild
                | TestRuleId::Empty => "same lowered ability",
            },
            construction: root.rule.construction(),
            constructions: vec![root.rule.construction()],
            positions: root.rule.positions().to_vec(),
        };
        let family = root.families.first().expect("toy nodes have one family");
        for child in &family.children {
            if let Child::Node(id) = child {
                let child = candidate_from_root(forest, forest.node(*id));
                candidate.constructions.extend(child.constructions);
                candidate.positions.extend(child.positions);
            }
        }
        candidate
    }

    const fn construction_name(construction: TestConstruction) -> &'static str {
        match construction {
            TestConstruction::LiteralAlpha => "LiteralAlpha",
            TestConstruction::BroadAlpha => "BroadAlpha",
            TestConstruction::SharedForm => "SharedForm",
            TestConstruction::TestLeft => "TestLeft",
            TestConstruction::TestRight => "TestRight",
            TestConstruction::TestThird => "TestThird",
            TestConstruction::LateLiteral => "LateLiteral",
            TestConstruction::EarlyTyped => "EarlyTyped",
            TestConstruction::LateLiteralChild => "LateLiteralChild",
            TestConstruction::EarlyTypedChild => "EarlyTypedChild",
            TestConstruction::Empty => "Empty",
        }
    }

    #[test]
    fn structural_specificity_selects_without_authored_dominance() {
        let selected = parse_and_select_specificity_toy("alpha").unwrap();
        assert_eq!(selected.construction, TestConstruction::LiteralAlpha);
    }

    #[test]
    fn selection_diagnostic_names_exhaustively_map_public_rules() {
        let constructions = generated_constructions_v1();
        assert!(!constructions.is_empty());

        let names = constructions
            .iter()
            .map(|construction| construction.name())
            .collect::<Vec<_>>();
        assert!(
            names
                .iter()
                .all(|name| !name.is_empty() && !name.contains(['\n', '\r']))
        );
        let unique_names = names.iter().copied().collect::<BTreeSet<_>>();
        assert_eq!(
            unique_names.len(),
            names.len(),
            "the generated diagnostic inventory contains each public construction once",
        );

        let public_rule_names = crate::constructions::RULES
            .iter()
            .filter_map(|rule| rule.id.public_construction())
            .map(Construction::name)
            .collect::<BTreeSet<_>>();
        assert_eq!(unique_names, public_rule_names);

        for stable_name in [
            "AbilityPlain",
            "SentenceImperative",
            "FiniteClausePlainFiniteClause",
            "WhereClauseCategoryWhere",
            "VerbPhraseBaseVerbPhrase",
        ] {
            assert!(
                unique_names.contains(stable_name),
                "missing stable public construction name {stable_name:?}",
            );
        }
    }

    #[test]
    fn an_unbroken_tie_names_both_test_constructions() {
        let error = parse_and_select_tie_toy("alpha").unwrap_err();
        assert_eq!(
            error,
            ParseError::Ambiguous {
                first: "TestLeft",
                second: "TestRight",
            }
        );
    }

    #[test]
    fn two_forms_of_one_construction_survive_to_selection_as_distinct_values() {
        let (roots, candidates) = parse_and_materialize_shared_forms("alpha");
        assert_eq!(roots, 2);
        assert_eq!(
            candidates
                .iter()
                .map(|candidate| match candidate.value.as_ref() {
                    TestBuildValue::Ability(ability) => *ability,
                    TestBuildValue::Leaf(_) => panic!("accepted roots build abilities"),
                })
                .collect::<std::collections::BTreeSet<_>>(),
            std::collections::BTreeSet::from(["literal form", "typed form"]),
        );
        assert!(
            candidates
                .iter()
                .all(|candidate| candidate.constructions.as_ref() == [Construction::AmountNumber])
        );
        assert_eq!(
            candidates
                .iter()
                .map(|candidate| candidate.claims[0].owner)
                .collect::<BTreeSet<_>>(),
            BTreeSet::from([Some("literal-owner"), Some("typed-owner")])
        );

        let selected = select_ranked(
            candidates,
            |candidate| candidate.constructions.as_ref(),
            |candidate| {
                structural_specificity(
                    &candidate.positions,
                    |lexical| matches!(lexical, Lexical::Literal(_)),
                    |lexical| lexical.is_identity(),
                )
            },
            |_| "SharedForm",
            &[],
        )
        .unwrap();
        assert_eq!(
            selected.value.as_ref(),
            &TestBuildValue::Ability("literal form")
        );
        assert_eq!(selected.claims[0].owner, Some("literal-owner"));
    }

    #[test]
    fn equal_semantic_values_with_different_segmentations_retain_both_claim_sequences() {
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Ord, PartialOrd)]
        enum SegCategory {
            Start,
        }
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Ord, PartialOrd)]
        enum SegLexical {
            Whole,
            Left,
            Right,
        }
        #[derive(Debug, Clone, Copy, PartialEq, Eq)]
        enum SegRule {
            Whole,
            Parts,
        }
        #[derive(Debug, Clone, Copy, PartialEq, Eq)]
        enum SegValue {
            Leaf,
            Same,
        }
        const RULES: &[Rule<SegCategory, SegLexical, SegRule>] = &[
            Rule {
                id: SegRule::Whole,
                lhs: SegCategory::Start,
                rhs: &[RulePosition::Lexical(SegLexical::Whole)],
            },
            Rule {
                id: SegRule::Parts,
                lhs: SegCategory::Start,
                rhs: &[
                    RulePosition::Lexical(SegLexical::Left),
                    RulePosition::Lexical(SegLexical::Right),
                ],
            },
        ];
        let forest = parse(
            RULES,
            SegCategory::Start,
            2,
            |terminal, start| match (terminal, start) {
                (SegLexical::Whole, 0) => vec![LexicalMatch {
                    end: 2,
                    value: terminal,
                    owner: Some("whole"),
                }],
                (SegLexical::Left, 0) => vec![LexicalMatch {
                    end: 1,
                    value: terminal,
                    owner: Some("left"),
                }],
                (SegLexical::Right, 1) => vec![LexicalMatch {
                    end: 2,
                    value: terminal,
                    owner: Some("right"),
                }],
                _ => Vec::new(),
            },
            |_, _, _| true,
        )
        .unwrap();
        let candidates = materialize_with(
            &forest,
            RULES,
            |rule| match rule {
                SegRule::Whole => 0,
                SegRule::Parts => 1,
            },
            |_| Some("same-construction"),
            std::convert::identity,
            |_| SegValue::Leaf,
            |_, _| Ok(Some(SegValue::Same)),
        );

        assert_eq!(candidates.len(), 2);
        assert!(
            candidates
                .iter()
                .all(|candidate| candidate.value.as_ref() == &SegValue::Same)
        );
        assert_eq!(
            candidates
                .iter()
                .map(|candidate| {
                    candidate
                        .claims
                        .iter()
                        .map(|claim| (claim.span, claim.owner))
                        .collect::<Vec<_>>()
                })
                .collect::<BTreeSet<_>>(),
            BTreeSet::from([
                vec![(TextSpan { start: 0, end: 2 }, Some("whole"))],
                vec![
                    (TextSpan { start: 0, end: 1 }, Some("left")),
                    (TextSpan { start: 1, end: 2 }, Some("right")),
                ],
            ])
        );
    }

    #[test]
    fn an_earlier_typed_position_outweighs_a_later_literal() {
        let selected = parse_and_select_recursive_specificity_toy("alpha").unwrap();
        assert_eq!(selected.construction, TestConstruction::EarlyTyped);
    }

    #[test]
    fn a_longer_exact_prefix_specificity_vector_wins() {
        let shorter = TestCandidate {
            ability: "shorter",
            construction: TestConstruction::TestLeft,
            constructions: vec![TestConstruction::TestLeft],
            positions: vec![RulePosition::Lexical(TestLexical::Word)],
        };
        let longer = TestCandidate {
            ability: "longer",
            construction: TestConstruction::TestRight,
            constructions: vec![TestConstruction::TestRight],
            positions: vec![
                RulePosition::Lexical(TestLexical::Word),
                RulePosition::Nonterminal(TestCategory::Empty),
            ],
        };

        let selected = select_ranked(
            vec![shorter, longer],
            |candidate| candidate.constructions.as_slice(),
            |candidate| {
                structural_specificity(
                    &candidate.positions,
                    |lexical| matches!(lexical, TestLexical::LiteralAlpha),
                    |lexical| matches!(lexical, TestLexical::Identity),
                )
            },
            construction_name,
            &[],
        )
        .unwrap();

        assert_eq!(selected.ability, "longer");
    }

    #[test]
    fn selection_exception_registry_selects_its_winner_in_both_orientations() {
        let exceptions = [SelectionException {
            id: "test-left-over-right",
            left: TestConstruction::TestLeft,
            right: TestConstruction::TestRight,
            winner: TestConstruction::TestLeft,
            rationale: "the synthetic left construction wins",
        }];

        let forward = select_ranked(
            test_tied_candidates(TestConstruction::TestLeft, TestConstruction::TestRight),
            |candidate| candidate.constructions.as_slice(),
            |candidate| structural_specificity(&candidate.positions, |_| false, |_| false),
            construction_name,
            &exceptions,
        )
        .unwrap();
        let reverse = select_ranked(
            test_tied_candidates(TestConstruction::TestRight, TestConstruction::TestLeft),
            |candidate| candidate.constructions.as_slice(),
            |candidate| structural_specificity(&candidate.positions, |_| false, |_| false),
            construction_name,
            &exceptions,
        )
        .unwrap();

        assert_eq!(forward.construction, TestConstruction::TestLeft);
        assert_eq!(reverse.construction, TestConstruction::TestLeft);
    }

    #[test]
    fn selection_diagnostic_records_complete_specificity_decisions() {
        let literal = TestCandidate {
            ability: "literal",
            construction: TestConstruction::LiteralAlpha,
            constructions: vec![TestConstruction::LiteralAlpha],
            positions: vec![RulePosition::Lexical(TestLexical::LiteralAlpha)],
        };
        let typed = TestCandidate {
            ability: "typed",
            construction: TestConstruction::BroadAlpha,
            constructions: vec![TestConstruction::BroadAlpha],
            positions: vec![RulePosition::Lexical(TestLexical::Word)],
        };

        let analysis = selection_analysis_with_exceptions(
            vec![typed, literal],
            |candidate| candidate.constructions.as_slice(),
            |candidate| {
                structural_specificity(
                    &candidate.positions,
                    |lexical| matches!(lexical, TestLexical::LiteralAlpha),
                    |lexical| matches!(lexical, TestLexical::Identity),
                )
            },
            |_| Vec::new(),
            |construction| format!("{construction:?}"),
            construction_name,
            &[],
        )
        .unwrap();
        let decision = analysis
            .decision()
            .expect("materialized candidates have a decision");

        assert_eq!(
            decision
                .candidates()
                .iter()
                .map(|candidate| (
                    candidate.ordinal(),
                    candidate.construction_path(),
                    candidate.specificity(),
                ))
                .collect::<Vec<_>>(),
            vec![
                (
                    0,
                    ["BroadAlpha".to_owned()].as_slice(),
                    [SpecificityTier::TypedLexical].as_slice()
                ),
                (
                    1,
                    ["LiteralAlpha".to_owned()].as_slice(),
                    [SpecificityTier::Literal].as_slice()
                ),
            ]
        );
        assert_eq!(decision.survivors(), &[1]);
        assert_eq!(decision.selected(), Some(1));
        assert_eq!(decision.resolution(), SelectionResolution::Specificity);
        assert!(decision.exception_uses().is_empty());
        assert_eq!(decision.comparisons().len(), 1);
        let comparison = &decision.comparisons()[0];
        assert_eq!(
            (comparison.left_ordinal(), comparison.right_ordinal()),
            (0, 1)
        );
        assert_eq!(comparison.ordering(), Ordering::Less);
        assert_eq!(comparison.decisive(), SelectionDecisive::Position(0));
        assert_eq!(comparison.exception_id(), None);
    }

    #[test]
    fn selection_diagnostic_records_late_and_exhausted_specificity_positions() {
        let late_literal = TestCandidate {
            ability: "late literal",
            construction: TestConstruction::LateLiteral,
            constructions: vec![TestConstruction::LateLiteral],
            positions: vec![
                RulePosition::Nonterminal(TestCategory::Start),
                RulePosition::Lexical(TestLexical::LiteralAlpha),
            ],
        };
        let late_typed = TestCandidate {
            ability: "late typed",
            construction: TestConstruction::EarlyTyped,
            constructions: vec![TestConstruction::EarlyTyped],
            positions: vec![
                RulePosition::Nonterminal(TestCategory::Start),
                RulePosition::Lexical(TestLexical::Word),
            ],
        };
        let late = selection_analysis_with_exceptions(
            vec![late_typed, late_literal],
            |candidate| candidate.constructions.as_slice(),
            |candidate| {
                structural_specificity(
                    &candidate.positions,
                    |lexical| matches!(lexical, TestLexical::LiteralAlpha),
                    |lexical| matches!(lexical, TestLexical::Identity),
                )
            },
            |_| Vec::new(),
            |construction| format!("{construction:?}"),
            construction_name,
            &[],
        )
        .unwrap();
        let late_comparison = &late.decision().unwrap().comparisons()[0];
        assert_eq!(late_comparison.ordering(), Ordering::Less);
        assert_eq!(late_comparison.decisive(), SelectionDecisive::Position(1));

        let shorter = TestCandidate {
            ability: "shorter",
            construction: TestConstruction::TestLeft,
            constructions: vec![TestConstruction::TestLeft],
            positions: vec![RulePosition::Lexical(TestLexical::Word)],
        };
        let longer = TestCandidate {
            ability: "longer",
            construction: TestConstruction::TestRight,
            constructions: vec![TestConstruction::TestRight],
            positions: vec![
                RulePosition::Lexical(TestLexical::Word),
                RulePosition::Nonterminal(TestCategory::Empty),
            ],
        };
        let exhausted = selection_analysis_with_exceptions(
            vec![shorter, longer],
            |candidate| candidate.constructions.as_slice(),
            |candidate| structural_specificity(&candidate.positions, |_| false, |_| false),
            |_| Vec::new(),
            |construction| format!("{construction:?}"),
            construction_name,
            &[],
        )
        .unwrap();
        let exhausted_comparison = &exhausted.decision().unwrap().comparisons()[0];
        assert_eq!(exhausted_comparison.ordering(), Ordering::Less);
        assert_eq!(
            exhausted_comparison.decisive(),
            SelectionDecisive::VectorExhaustion(1)
        );
    }

    #[test]
    fn selection_diagnostic_records_exception_orientation_and_complete_ties() {
        let exceptions = [SelectionException {
            id: "test-left-over-right",
            left: TestConstruction::TestLeft,
            right: TestConstruction::TestRight,
            winner: TestConstruction::TestLeft,
            rationale: "the synthetic left construction wins",
        }];
        for (candidates, expected_selected, expected_ordering) in [
            (
                test_tied_candidates(TestConstruction::TestLeft, TestConstruction::TestRight),
                0,
                Ordering::Greater,
            ),
            (
                test_tied_candidates(TestConstruction::TestRight, TestConstruction::TestLeft),
                1,
                Ordering::Less,
            ),
        ] {
            let analysis = selection_analysis_with_exceptions(
                candidates,
                |candidate| candidate.constructions.as_slice(),
                |candidate| structural_specificity(&candidate.positions, |_| false, |_| false),
                |_| Vec::new(),
                |construction| format!("{construction:?}"),
                construction_name,
                &exceptions,
            )
            .unwrap();
            let decision = analysis.decision().unwrap();
            assert_eq!(decision.selected(), Some(expected_selected));
            assert_eq!(decision.survivors(), &[expected_selected]);
            assert_eq!(decision.resolution(), SelectionResolution::Exception);
            assert_eq!(decision.exception_uses(), ["test-left-over-right"]);
            let comparison = &decision.comparisons()[0];
            assert_eq!(comparison.ordering(), expected_ordering);
            assert_eq!(comparison.decisive(), SelectionDecisive::Tie);
            assert_eq!(comparison.exception_id(), Some("test-left-over-right"));
        }

        let hard_tie = selection_analysis_with_exceptions(
            test_tied_candidates(TestConstruction::TestLeft, TestConstruction::TestRight),
            |candidate| candidate.constructions.as_slice(),
            |candidate| structural_specificity(&candidate.positions, |_| false, |_| false),
            |_| Vec::new(),
            |construction| format!("{construction:?}"),
            construction_name,
            &[],
        )
        .unwrap();
        let decision = hard_tie.decision().unwrap();
        assert_eq!(decision.survivors(), &[0, 1]);
        assert_eq!(decision.selected(), None);
        assert_eq!(decision.resolution(), SelectionResolution::UnresolvedTie);
        assert_eq!(decision.comparisons()[0].decisive(), SelectionDecisive::Tie);
        assert_eq!(decision.comparisons()[0].exception_id(), None);
    }

    #[test]
    fn selection_diagnostic_retains_all_pairs_and_empty_survivors_for_exception_cycles() {
        let candidates = [
            TestConstruction::TestLeft,
            TestConstruction::TestRight,
            TestConstruction::TestThird,
        ]
        .into_iter()
        .map(|construction| TestCandidate {
            ability: construction_name(construction),
            construction,
            constructions: vec![construction],
            positions: vec![RulePosition::Lexical(TestLexical::Word)],
        })
        .collect();
        let exceptions = [
            SelectionException {
                id: "left-over-right",
                left: TestConstruction::TestLeft,
                right: TestConstruction::TestRight,
                winner: TestConstruction::TestLeft,
                rationale: "synthetic cycle edge",
            },
            SelectionException {
                id: "right-over-third",
                left: TestConstruction::TestRight,
                right: TestConstruction::TestThird,
                winner: TestConstruction::TestRight,
                rationale: "synthetic cycle edge",
            },
            SelectionException {
                id: "third-over-left",
                left: TestConstruction::TestThird,
                right: TestConstruction::TestLeft,
                winner: TestConstruction::TestThird,
                rationale: "synthetic cycle edge",
            },
        ];

        let analysis = selection_analysis_with_exceptions(
            candidates,
            |candidate| candidate.constructions.as_slice(),
            |candidate| structural_specificity(&candidate.positions, |_| false, |_| false),
            |_| Vec::new(),
            |construction| format!("{construction:?}"),
            construction_name,
            &exceptions,
        )
        .unwrap();
        let decision = analysis.decision().unwrap();
        assert_eq!(decision.survivors(), &[] as &[usize]);
        assert_eq!(decision.selected(), None);
        assert_eq!(decision.resolution(), SelectionResolution::UnresolvedTie);
        assert_eq!(
            decision
                .comparisons()
                .iter()
                .map(|comparison| (
                    comparison.left_ordinal(),
                    comparison.right_ordinal(),
                    comparison.ordering(),
                    comparison.exception_id(),
                ))
                .collect::<Vec<_>>(),
            vec![
                (0, 1, Ordering::Greater, Some("left-over-right")),
                (0, 2, Ordering::Less, Some("third-over-left")),
                (1, 2, Ordering::Greater, Some("right-over-third")),
            ]
        );
        assert_eq!(
            decision.exception_uses(),
            ["left-over-right", "third-over-left", "right-over-third"]
        );
    }

    fn assert_bounded<T>(bounded: &Bounded<T>, limit: usize) {
        assert_eq!(bounded.total(), bounded.shown() + bounded.omitted());
        assert_eq!(bounded.shown(), bounded.items().len());
        assert!(bounded.shown() <= limit);
    }

    fn traced_decision(
        candidates: Vec<TestCandidate>,
        exceptions: &[SelectionException<TestConstruction>],
        limit: usize,
    ) -> BoundedSelectionDecision {
        BoundedSelectionDecision::from_complete(&complete_decision(candidates, exceptions), limit)
    }

    fn complete_decision(
        candidates: Vec<TestCandidate>,
        exceptions: &[SelectionException<TestConstruction>],
    ) -> SelectionDecision {
        let analysis = selection_analysis_with_exceptions(
            candidates,
            |candidate| candidate.constructions.as_slice(),
            |candidate| structural_specificity(&candidate.positions, |_| false, |_| false),
            |_| Vec::new(),
            |construction| format!("{construction:?}"),
            construction_name,
            exceptions,
        )
        .unwrap();
        analysis
            .decision()
            .expect("nonempty selection decision")
            .clone()
    }

    #[test]
    fn parser_trace_bounded_selection_authenticates_all_resolution_modes_and_loser_reasons() {
        let unique = traced_decision(
            vec![TestCandidate {
                ability: "unique",
                construction: TestConstruction::TestLeft,
                constructions: vec![TestConstruction::TestLeft, TestConstruction::Empty],
                positions: vec![
                    RulePosition::Lexical(TestLexical::Word),
                    RulePosition::Nonterminal(TestCategory::Empty),
                ],
            }],
            &[],
            8,
        );
        assert_eq!(unique.resolution(), SelectionResolution::Unique);
        assert_eq!(unique.selected().unwrap().ordinal(), 0);
        assert_eq!(
            unique.selected().unwrap().construction_path().items(),
            ["TestLeft", "Empty"]
        );
        assert_eq!(
            unique.selected().unwrap().specificity().items(),
            [SpecificityTier::TypedLexical, SpecificityTier::Nonterminal]
        );
        assert_eq!(unique.unselected_candidates().total(), 0);

        let specificity = traced_decision(
            vec![
                TestCandidate {
                    ability: "typed",
                    construction: TestConstruction::BroadAlpha,
                    constructions: vec![TestConstruction::BroadAlpha],
                    positions: vec![RulePosition::Lexical(TestLexical::Word)],
                },
                TestCandidate {
                    ability: "longer",
                    construction: TestConstruction::LiteralAlpha,
                    constructions: vec![TestConstruction::LiteralAlpha],
                    positions: vec![
                        RulePosition::Lexical(TestLexical::Word),
                        RulePosition::Nonterminal(TestCategory::Empty),
                    ],
                },
            ],
            &[],
            8,
        );
        assert_eq!(specificity.resolution(), SelectionResolution::Specificity);
        assert_eq!(specificity.selected().unwrap().ordinal(), 1);
        assert_eq!(specificity.comparisons().total(), 1);
        assert_eq!(specificity.survivors().items(), [1]);
        assert_eq!(specificity.unselected_candidates().total(), 1);
        let loser = &specificity.unselected_candidates().items()[0];
        assert_eq!(loser.candidate().ordinal(), 0);
        assert_eq!(loser.reason(), SelectionLoserReason::LessSpecific);
        assert_eq!(loser.evidence().total(), 1);

        let exception = SelectionException {
            id: "left-over-right",
            left: TestConstruction::TestLeft,
            right: TestConstruction::TestRight,
            winner: TestConstruction::TestLeft,
            rationale: "synthetic trace edge",
        };
        let exception_resolved = traced_decision(
            test_tied_candidates(TestConstruction::TestRight, TestConstruction::TestLeft),
            &[exception],
            8,
        );
        assert_eq!(
            exception_resolved.resolution(),
            SelectionResolution::Exception
        );
        assert_eq!(exception_resolved.selected().unwrap().ordinal(), 1);
        assert_eq!(
            exception_resolved.exception_uses().items(),
            ["left-over-right"]
        );
        let loser = &exception_resolved.unselected_candidates().items()[0];
        assert_eq!(loser.candidate().ordinal(), 0);
        assert_eq!(loser.reason(), SelectionLoserReason::ExceptionLoser);
        assert_eq!(
            loser.evidence().items()[0].exception_id(),
            Some("left-over-right")
        );

        let unresolved = traced_decision(
            test_tied_candidates(TestConstruction::TestLeft, TestConstruction::TestRight),
            &[],
            8,
        );
        assert_eq!(unresolved.resolution(), SelectionResolution::UnresolvedTie);
        assert!(unresolved.selected().is_none());
        assert_eq!(unresolved.survivors().items(), [0, 1]);
        assert_eq!(unresolved.unselected_candidates().total(), 2);
        assert!(
            unresolved
                .unselected_candidates()
                .items()
                .iter()
                .all(|loser| {
                    loser.reason() == SelectionLoserReason::UnresolvedSurvivor
                        && loser.evidence().total() == 1
                })
        );
    }

    #[test]
    fn parser_trace_selection_caps_every_outer_and_nested_collection_independently() {
        let mut candidates = vec![TestCandidate {
            ability: "less specific",
            construction: TestConstruction::BroadAlpha,
            constructions: vec![TestConstruction::BroadAlpha, TestConstruction::Empty],
            positions: vec![RulePosition::Nonterminal(TestCategory::Empty)],
        }];
        candidates.extend(
            [
                TestConstruction::TestLeft,
                TestConstruction::TestRight,
                TestConstruction::TestThird,
            ]
            .into_iter()
            .map(|construction| TestCandidate {
                ability: construction_name(construction),
                construction,
                constructions: vec![construction, TestConstruction::Empty],
                positions: vec![
                    RulePosition::Lexical(TestLexical::Word),
                    RulePosition::Nonterminal(TestCategory::Empty),
                ],
            }),
        );
        let exceptions = [
            SelectionException {
                id: "left-over-right",
                left: TestConstruction::TestLeft,
                right: TestConstruction::TestRight,
                winner: TestConstruction::TestLeft,
                rationale: "synthetic cycle edge",
            },
            SelectionException {
                id: "right-over-third",
                left: TestConstruction::TestRight,
                right: TestConstruction::TestThird,
                winner: TestConstruction::TestRight,
                rationale: "synthetic cycle edge",
            },
            SelectionException {
                id: "third-over-left",
                left: TestConstruction::TestThird,
                right: TestConstruction::TestLeft,
                winner: TestConstruction::TestThird,
                rationale: "synthetic cycle edge",
            },
        ];

        for limit in [0, 1, 8] {
            let projection = traced_decision(candidates.clone(), &exceptions, limit);
            assert_eq!(projection.resolution(), SelectionResolution::UnresolvedTie);
            assert_bounded(projection.candidates(), limit);
            assert_bounded(projection.comparisons(), limit);
            assert_bounded(projection.survivors(), limit);
            assert_bounded(projection.exception_uses(), limit);
            assert_bounded(projection.unselected_candidates(), limit);
            assert_eq!(projection.candidates().total(), 4);
            assert_eq!(projection.comparisons().total(), 6);
            assert_eq!(projection.exception_uses().total(), 3);
            assert_eq!(projection.unselected_candidates().total(), 4);
            for candidate in projection.candidates().items() {
                assert_bounded(candidate.construction_path(), limit);
                assert_bounded(candidate.specificity(), limit);
            }
            for loser in projection.unselected_candidates().items() {
                assert_bounded(loser.candidate().construction_path(), limit);
                assert_bounded(loser.candidate().specificity(), limit);
                assert_bounded(loser.evidence(), limit);
            }
            if limit > 0 {
                let loser = &projection.unselected_candidates().items()[0];
                assert_eq!(loser.reason(), SelectionLoserReason::LessSpecific);
                assert_eq!(loser.evidence().total(), 3);
                assert_eq!(loser.evidence().shown(), usize::min(limit, 3));
            }
            let repeated = traced_decision(candidates.clone(), &exceptions, limit);
            assert_eq!(projection, repeated);
        }

        let complete = complete_decision(candidates, &exceptions);
        let projection = BoundedSelectionDecision::from_complete(&complete, usize::MAX);
        assert_eq!(projection.resolution(), complete.resolution());
        assert_eq!(projection.survivors().items(), complete.survivors());
        assert_eq!(
            projection.exception_uses().items(),
            complete.exception_uses()
        );
        for (projected, complete) in projection
            .candidates()
            .items()
            .iter()
            .zip(complete.candidates())
        {
            assert_eq!(projected.ordinal(), complete.ordinal());
            assert_eq!(
                projected.construction_path().items(),
                complete.construction_path()
            );
            assert_eq!(projected.specificity().items(), complete.specificity());
        }
        for (projected, complete) in projection
            .comparisons()
            .items()
            .iter()
            .zip(complete.comparisons())
        {
            assert_eq!(projected.left_ordinal(), complete.left_ordinal());
            assert_eq!(projected.right_ordinal(), complete.right_ordinal());
            assert_eq!(projected.ordering(), complete.ordering());
            assert_eq!(projected.decisive(), complete.decisive());
            assert_eq!(projected.exception_id(), complete.exception_id());
        }
    }

    #[test]
    fn parser_trace_unresolved_outcome_contains_the_bounded_complete_decision() {
        let complete = complete_decision(
            test_tied_candidates(TestConstruction::TestLeft, TestConstruction::TestRight),
            &[],
        );
        let context = ParseContext::new(
            "Trace Card",
            false,
            deckmaste_construction_core::macro_def::Onset::Consonant,
        )
        .expect("context");
        let environment = canonical_test_environment();
        let trace = ParserTrace::from_parts(
            ParseAnalysis::<crate::ast::Ability>::from_result(
                Err(ParseError::Ambiguous {
                    first: "TestLeft",
                    second: "TestRight",
                }),
                Some(complete.clone()),
            ),
            StructuralTrace::empty(),
            MaterializationTrace::empty(1),
            TraceLimits::new(1),
            &context,
            &environment,
        );

        let BoundedParseOutcome::UnresolvedAmbiguity(unresolved) = trace.outcome() else {
            panic!("hard tie must project as unresolved");
        };
        assert_eq!(
            unresolved.selection(),
            &BoundedSelectionDecision::from_complete(&complete, 1)
        );
        assert_eq!(
            trace.into_parse_result(),
            Err(ParseError::Ambiguous {
                first: "TestLeft",
                second: "TestRight",
            })
        );
    }

    #[test]
    fn selection_exception_inventory_canonicalizes_pairs_and_keeps_winner() {
        let inventory = selection_exception_inventory_for(
            &[SelectionException {
                id: "test-right-over-left",
                left: TestConstruction::TestRight,
                right: TestConstruction::TestLeft,
                winner: TestConstruction::TestRight,
                rationale: "the synthetic right construction wins",
            }],
            construction_name,
        )
        .unwrap();

        assert_eq!(inventory.len(), 1);
        assert_eq!(inventory[0].id, "test-right-over-left");
        assert_eq!(inventory[0].left, "TestLeft");
        assert_eq!(inventory[0].right, "TestRight");
        assert_eq!(inventory[0].winner, "TestRight");
        assert_eq!(
            inventory[0].rationale,
            "the synthetic right construction wins"
        );
    }

    #[test]
    fn selection_exception_inventory_rejects_invalid_registry_entries() {
        let entry = |id, left, right, winner, rationale| SelectionException {
            id,
            left,
            right,
            winner,
            rationale,
        };
        let name = construction_name;

        assert!(matches!(
            selection_exception_inventory_for(
                &[entry(
                    " ",
                    TestConstruction::TestLeft,
                    TestConstruction::TestRight,
                    TestConstruction::TestLeft,
                    "valid rationale",
                )],
                name,
            ),
            Err(SelectionExceptionInventoryError::BlankId)
        ));
        assert!(matches!(
            selection_exception_inventory_for(
                &[
                    entry(
                        "duplicate",
                        TestConstruction::TestLeft,
                        TestConstruction::TestRight,
                        TestConstruction::TestLeft,
                        "first rationale",
                    ),
                    entry(
                        "duplicate",
                        TestConstruction::LateLiteral,
                        TestConstruction::EarlyTyped,
                        TestConstruction::LateLiteral,
                        "second rationale",
                    ),
                ],
                name,
            ),
            Err(SelectionExceptionInventoryError::DuplicateId { .. })
        ));
        assert!(matches!(
            selection_exception_inventory_for(
                &[
                    entry(
                        "first-pair",
                        TestConstruction::TestLeft,
                        TestConstruction::TestRight,
                        TestConstruction::TestLeft,
                        "first rationale",
                    ),
                    entry(
                        "same-pair",
                        TestConstruction::TestRight,
                        TestConstruction::TestLeft,
                        TestConstruction::TestRight,
                        "second rationale",
                    ),
                ],
                name,
            ),
            Err(SelectionExceptionInventoryError::DuplicatePair { .. })
        ));
        assert!(matches!(
            selection_exception_inventory_for(
                &[entry(
                    "self-pair",
                    TestConstruction::TestLeft,
                    TestConstruction::TestLeft,
                    TestConstruction::TestLeft,
                    "valid rationale",
                )],
                name,
            ),
            Err(SelectionExceptionInventoryError::SelfPair { .. })
        ));
        assert!(matches!(
            selection_exception_inventory_for(
                &[entry(
                    "blank-rationale",
                    TestConstruction::TestLeft,
                    TestConstruction::TestRight,
                    TestConstruction::TestLeft,
                    "\t",
                )],
                name,
            ),
            Err(SelectionExceptionInventoryError::BlankRationale { .. })
        ));
        assert!(matches!(
            selection_exception_inventory_for(
                &[entry(
                    "outside-winner",
                    TestConstruction::TestLeft,
                    TestConstruction::TestRight,
                    TestConstruction::LateLiteral,
                    "valid rationale",
                )],
                name,
            ),
            Err(SelectionExceptionInventoryError::WinnerOutsidePair { .. })
        ));
    }

    #[test]
    fn selection_exception_inventory_is_empty_in_production() {
        assert!(selection_exception_inventory().unwrap().is_empty());
    }

    #[test]
    fn validated_selection_rejects_an_invalid_synthetic_registry() {
        let error = select_with_exceptions(
            test_tied_candidates(TestConstruction::TestLeft, TestConstruction::TestRight),
            |candidate| candidate.constructions.as_slice(),
            |candidate| structural_specificity(&candidate.positions, |_| false, |_| false),
            construction_name,
            &[SelectionException {
                id: " ",
                left: TestConstruction::TestLeft,
                right: TestConstruction::TestRight,
                winner: TestConstruction::TestLeft,
                rationale: "valid rationale",
            }],
        )
        .unwrap_err();

        assert_eq!(
            error,
            ParseError::InvalidSelectionExceptionConfiguration(
                SelectionExceptionInventoryError::BlankId
            )
        );
    }

    fn test_tied_candidates(
        first: TestConstruction,
        second: TestConstruction,
    ) -> Vec<TestCandidate> {
        [first, second]
            .into_iter()
            .map(|construction| TestCandidate {
                ability: construction_name(construction),
                construction,
                constructions: vec![construction],
                positions: vec![RulePosition::Lexical(TestLexical::Word)],
            })
            .collect()
    }
}
