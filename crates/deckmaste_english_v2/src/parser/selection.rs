use std::cmp::Ordering;

use super::ParseError;
use super::engine::RulePosition;
use super::materialize::Candidate;
use super::rules::Construction;
use super::rules::Lexical;
use crate::ast::Ability;

const SELECTION_EXCEPTIONS: &[(Construction, Construction, Ordering)] = &[];

#[derive(Debug, Clone, PartialEq, Eq, Ord, PartialOrd)]
struct Specificity(Vec<PositionSpecificity>);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Ord, PartialOrd)]
enum PositionSpecificity {
    Nonterminal,
    TypedLexical,
    Literal,
}

pub(crate) fn select(candidates: Vec<Candidate>) -> Result<Option<Ability>, ParseError> {
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
    exceptions: &[(C, C, Ordering)],
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
    exceptions: &[(C, C, Ordering)],
) -> Ordering {
    let Some((left, right)) = first_difference(left, right) else {
        return Ordering::Equal;
    };
    exceptions
        .iter()
        .find_map(|&(exception_left, exception_right, ordering)| {
            if (left, right) == (exception_left, exception_right) {
                Some(ordering)
            } else if (left, right) == (exception_right, exception_left) {
                Some(ordering.reverse())
            } else {
                None
            }
        })
        .unwrap_or(Ordering::Equal)
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
    use super::select_ranked;
    use super::structural_specificity;
    use crate::parser::ParseError;
    use crate::parser::engine::Child;
    use crate::parser::engine::Forest;
    use crate::parser::engine::LexicalMatch;
    use crate::parser::engine::PackedNode;
    use crate::parser::engine::Rule;
    use crate::parser::engine::RulePosition;
    use crate::parser::engine::parse;

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

    const SHARED_FORM_RULES: &[Rule<TestCategory, TestLexical, TestRuleId>] = &[
        Rule {
            id: TestRuleId::SharedLiteral,
            lhs: TestCategory::Start,
            rhs: &[RulePosition::Lexical(TestLexical::LiteralAlpha)],
        },
        Rule {
            id: TestRuleId::SharedTyped,
            lhs: TestCategory::Start,
            rhs: &[RulePosition::Lexical(TestLexical::Word)],
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

    fn parse_and_materialize_shared_forms(text: &str) -> (usize, Vec<TestCandidate>) {
        let forest = parse(
            SHARED_FORM_RULES,
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
        .expect("the shared-form toy grammar accepts alpha");
        let roots = forest.accepted_roots().collect::<Vec<_>>();
        let candidates = roots
            .iter()
            .map(|root| candidate_from_root(&forest, root))
            .collect();
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
                .map(|candidate| candidate.ability)
                .collect::<std::collections::BTreeSet<_>>(),
            std::collections::BTreeSet::from(["literal form", "typed form"]),
        );

        let selected = select_test_candidates(candidates).unwrap();
        assert_eq!(selected.ability, "literal form");
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
}
