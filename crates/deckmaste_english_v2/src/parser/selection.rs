use std::cmp::Ordering;

use super::ParseError;
use super::engine::RulePosition;
use super::lexical::Lexical;
use super::materialize::Candidate;
use crate::ast::Ability;
use crate::constructions::Construction;

const SELECTION_EXCEPTIONS: &[SelectionException<Construction>] = &[];

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SelectionExceptionInfo {
    pub id: &'static str,
    pub left: &'static str,
    pub right: &'static str,
    pub winner: &'static str,
    pub rationale: &'static str,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SelectionExceptionInventoryError {
    BlankId,
    DuplicateId {
        id: &'static str,
    },
    DuplicatePair {
        left: &'static str,
        right: &'static str,
    },
    SelfPair {
        construction: &'static str,
    },
    BlankRationale {
        id: &'static str,
    },
    WinnerOutsidePair {
        id: &'static str,
    },
}

impl std::fmt::Display for SelectionExceptionInventoryError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str("invalid selection exception configuration: ")?;
        match self {
            Self::BlankId => formatter.write_str("entry id is empty or whitespace-only"),
            Self::DuplicateId { id } => write!(formatter, "duplicate entry id `{id}`"),
            Self::DuplicatePair { left, right } => {
                write!(formatter, "duplicate unordered pair `{left}` and `{right}`")
            }
            Self::SelfPair { construction } => {
                write!(formatter, "self-pair for construction `{construction}`")
            }
            Self::BlankRationale { id } => {
                write!(
                    formatter,
                    "entry `{id}` has an empty or whitespace-only rationale"
                )
            }
            Self::WinnerOutsidePair { id } => {
                write!(formatter, "entry `{id}` names a winner outside its pair")
            }
        }
    }
}

impl std::error::Error for SelectionExceptionInventoryError {}

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
    selection_exception_inventory_for(SELECTION_EXCEPTIONS, construction_name)
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
struct Specificity(Vec<PositionSpecificity>);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Ord, PartialOrd)]
enum PositionSpecificity {
    Nonterminal,
    TypedLexical,
    Literal,
}

pub(crate) fn select(candidates: Vec<Candidate>) -> Result<Option<Ability>, ParseError> {
    validate_selection_exceptions(SELECTION_EXCEPTIONS, &construction_name)
        .map_err(ParseError::InvalidSelectionExceptionConfiguration)?;
    if candidates.is_empty() {
        return Ok(None);
    }
    select_ranked(
        candidates,
        |candidate| candidate.constructions.as_slice(),
        |candidate| {
            structural_specificity(&candidate.positions, |lexical| {
                matches!(lexical, Lexical::Literal(_))
            })
        },
        construction_name,
        SELECTION_EXCEPTIONS,
    )
    .map(|candidate| Some(candidate.ability))
}

fn structural_specificity<N, L>(
    positions: &[RulePosition<N, L>],
    is_literal: impl Fn(&L) -> bool,
) -> Specificity {
    Specificity(
        positions
            .iter()
            .map(|position| match position {
                RulePosition::Lexical(lexical) if is_literal(lexical) => {
                    PositionSpecificity::Literal
                }
                RulePosition::Lexical(_) => PositionSpecificity::TypedLexical,
                RulePosition::Nonterminal(_) => PositionSpecificity::Nonterminal,
            })
            .collect(),
    )
}

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
    let best_specificity = candidates
        .iter()
        .map(&specificity)
        .max()
        .expect("selection requires at least one candidate");
    let best = candidates
        .into_iter()
        .filter(|candidate| specificity(candidate) == best_specificity)
        .collect::<Vec<_>>();
    if best.len() == 1 {
        return Ok(best.into_iter().next().unwrap());
    }

    let survivors = best
        .iter()
        .enumerate()
        .filter(|(candidate_index, candidate)| {
            best.iter()
                .enumerate()
                .filter(|(other_index, _)| candidate_index != other_index)
                .all(|(_, other)| {
                    exception_order(constructions(candidate), constructions(other), exceptions)
                        != Ordering::Less
                })
        })
        .map(|(index, _)| index)
        .collect::<Vec<_>>();
    if survivors.len() == 1 {
        return Ok(best.into_iter().nth(survivors[0]).unwrap());
    }

    let tied = if survivors.is_empty() {
        (0..best.len()).collect::<Vec<_>>()
    } else {
        survivors
    };
    let (first, second) = ambiguity_names(
        tied.iter().map(|&index| constructions(&best[index])),
        construction_name,
    );
    Err(ParseError::Ambiguous { first, second })
}

fn exception_order<C: Copy + Eq>(
    left: &[C],
    right: &[C],
    exceptions: &[SelectionException<C>],
) -> std::cmp::Ordering {
    let Some((left, right)) = first_difference(left, right) else {
        return std::cmp::Ordering::Equal;
    };
    exceptions
        .iter()
        .find_map(|exception| {
            ((left, right) == (exception.left, exception.right)
                || (left, right) == (exception.right, exception.left))
                .then_some(if exception.winner == left {
                    std::cmp::Ordering::Greater
                } else {
                    std::cmp::Ordering::Less
                })
        })
        .unwrap_or(std::cmp::Ordering::Equal)
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

const fn construction_name(construction: Construction) -> &'static str {
    match construction {
        Construction::AbilitySpell => "AbilitySpell",
        Construction::AbilityTriggered => "AbilityTriggered",
        Construction::SentenceImperative => "SentenceImperative",
        Construction::SentenceDeclarative => "SentenceDeclarative",
        Construction::SentenceWithWhere => "SentenceWithWhere",
        Construction::ClauseEvent => "ClauseEvent",
        Construction::ClauseWhere => "ClauseWhere",
        Construction::NounPhrasePronoun => "NounPhrasePronoun",
        Construction::NounPhraseCommon => "NounPhraseCommon",
        Construction::NounPhraseDemonstrative => "NounPhraseDemonstrative",
        Construction::NounPhraseTarget => "NounPhraseTarget",
        Construction::NounPhraseSelfReference => "NounPhraseSelfReference",
        Construction::NounPhraseCount => "NounPhraseCount",
        Construction::VerbPhraseDestroy => "VerbPhraseDestroy",
        Construction::VerbPhraseConnive => "VerbPhraseConnive",
        Construction::VerbPhraseDealDamage => "VerbPhraseDealDamage",
        Construction::VerbPhraseGainLife => "VerbPhraseGainLife",
        Construction::AmountNumber => "AmountNumber",
        Construction::AmountVariable => "AmountVariable",
    }
}

#[cfg(test)]
mod tests {
    use super::SelectionException;
    use super::SelectionExceptionInventoryError;
    use super::select_ranked;
    use super::selection_exception_inventory;
    use super::selection_exception_inventory_for;
    use super::structural_specificity;
    use crate::parser::ParseError;
    use crate::parser::engine::Child;
    use crate::parser::engine::Forest;
    use crate::parser::engine::LexicalMatch;
    use crate::parser::engine::PackedNode;
    use crate::parser::engine::Rule;
    use crate::parser::engine::RulePosition;
    use crate::parser::engine::parse;
    use crate::parser::materialize::materialize_with;
    use crate::parser::rules::Category;
    use crate::parser::rules::Construction;
    use crate::parser::rules::Lexical;

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
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq, Ord, PartialOrd)]
    enum TestConstruction {
        LiteralAlpha,
        BroadAlpha,
        SharedForm,
        TestLeft,
        TestRight,
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

    fn parse_and_materialize_shared_forms(
        text: &str,
    ) -> (
        usize,
        Vec<crate::parser::materialize::MaterializedCandidate<TestBuildValue, Construction>>,
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
            |_| Construction::AmountNumber,
            |leaf| TestBuildValue::Leaf(*leaf),
            |rule, children| match (rule, children) {
                (TestRuleId::SharedLiteral, [TestBuildValue::Leaf(Lexical::Literal("alpha"))]) => {
                    Some(TestBuildValue::Ability("literal form"))
                }
                (TestRuleId::SharedTyped, [TestBuildValue::Leaf(Lexical::Variable)]) => {
                    Some(TestBuildValue::Ability("typed form"))
                }
                _ => None,
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
                structural_specificity(&candidate.positions, |lexical| {
                    matches!(lexical, TestLexical::LiteralAlpha)
                })
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
                .map(|candidate| match candidate.value {
                    TestBuildValue::Ability(ability) => ability,
                    TestBuildValue::Leaf(_) => panic!("accepted roots build abilities"),
                })
                .collect::<std::collections::BTreeSet<_>>(),
            std::collections::BTreeSet::from(["literal form", "typed form"]),
        );
        assert!(
            candidates
                .iter()
                .all(|candidate| candidate.constructions == [Construction::AmountNumber])
        );

        let selected = select_ranked(
            candidates,
            |candidate| candidate.constructions.as_slice(),
            |candidate| {
                structural_specificity(&candidate.positions, |lexical| {
                    matches!(lexical, Lexical::Literal(_))
                })
            },
            |_| "SharedForm",
            &[],
        )
        .unwrap();
        assert_eq!(selected.value, TestBuildValue::Ability("literal form"));
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
                structural_specificity(&candidate.positions, |lexical| {
                    matches!(lexical, TestLexical::LiteralAlpha)
                })
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
            |candidate| structural_specificity(&candidate.positions, |_| false),
            construction_name,
            &exceptions,
        )
        .unwrap();
        let reverse = select_ranked(
            test_tied_candidates(TestConstruction::TestRight, TestConstruction::TestLeft),
            |candidate| candidate.constructions.as_slice(),
            |candidate| structural_specificity(&candidate.positions, |_| false),
            construction_name,
            &exceptions,
        )
        .unwrap();

        assert_eq!(forward.construction, TestConstruction::TestLeft);
        assert_eq!(reverse.construction, TestConstruction::TestLeft);
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
