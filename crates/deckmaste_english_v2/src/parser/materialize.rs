use std::collections::BTreeMap;
use std::collections::BTreeSet;

use super::build::BuildValue;
use super::build::build;
use super::engine::Child;
use super::engine::Family;
use super::engine::Forest;
use super::engine::NodeId;
use super::engine::RulePosition;
use super::rules::Category;
use super::rules::Construction;
use super::rules::Lexical;
use super::rules::RULES;
use super::rules::RuleId;
use super::scan::Leaf;
use crate::ast::Ability;
use crate::context::ParseContext;

#[derive(Debug, Clone, PartialEq, Eq)]
struct BuiltCandidate {
    value: BuildValue,
    constructions: Vec<Construction>,
    positions: Vec<RulePosition<Category, Lexical>>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct Candidate {
    pub ability: Ability,
    pub constructions: Vec<Construction>,
    pub positions: Vec<RulePosition<Category, Lexical>>,
}

#[derive(Default)]
struct MaterializationState {
    memo: BTreeMap<NodeId, Vec<BuiltCandidate>>,
    in_progress: BTreeSet<NodeId>,
}

struct MaterializationOutcome {
    values: Vec<BuiltCandidate>,
    cycle_pruned: bool,
}

pub(crate) fn materialize(
    forest: &Forest<RuleId, Leaf>,
    context: &ParseContext<'_>,
) -> Vec<Candidate> {
    let mut candidates = Vec::new();
    let mut state = MaterializationState::default();
    for root in forest.accepted_root_ids() {
        for built in materialize_node(forest, root, context, &mut state).values {
            if let BuildValue::Ability(ability) = built.value {
                push_unique(
                    &mut candidates,
                    Candidate {
                        ability,
                        constructions: built.constructions,
                        positions: built.positions,
                    },
                );
            }
        }
    }
    candidates
}

fn materialize_node(
    forest: &Forest<RuleId, Leaf>,
    node_id: NodeId,
    context: &ParseContext<'_>,
    state: &mut MaterializationState,
) -> MaterializationOutcome {
    if let Some(values) = state.memo.get(&node_id) {
        return MaterializationOutcome {
            values: values.clone(),
            cycle_pruned: false,
        };
    }
    if !state.in_progress.insert(node_id) {
        return MaterializationOutcome {
            values: Vec::new(),
            cycle_pruned: true,
        };
    }

    let node = forest.node(node_id);
    let mut values = Vec::new();
    let mut cycle_pruned = false;
    for family in &node.families {
        let outcome = materialize_family(forest, node.rule, family, context, state);
        cycle_pruned |= outcome.cycle_pruned;
        for built in outcome.values {
            push_unique(&mut values, built);
        }
    }
    state.in_progress.remove(&node_id);
    if !cycle_pruned {
        state.memo.insert(node_id, values.clone());
    }
    MaterializationOutcome {
        values,
        cycle_pruned,
    }
}

pub(super) fn completion_has_checked_build(
    rule: RuleId,
    family: &Family<Leaf>,
    forest: &Forest<RuleId, Leaf>,
    context: &ParseContext<'_>,
) -> bool {
    !materialize_family(
        forest,
        rule,
        family,
        context,
        &mut MaterializationState::default(),
    )
    .values
    .is_empty()
}

fn materialize_family(
    forest: &Forest<RuleId, Leaf>,
    rule_id: RuleId,
    family: &Family<Leaf>,
    context: &ParseContext<'_>,
    state: &mut MaterializationState,
) -> MaterializationOutcome {
    let rule = &RULES[rule_id.index()];
    let construction = rule_id.construction();
    let mut combinations = vec![Vec::new()];
    let mut cycle_pruned = false;
    for child in &family.children {
        let child_values = match child {
            Child::Node(id) => {
                let outcome = materialize_node(forest, *id, context, state);
                cycle_pruned |= outcome.cycle_pruned;
                outcome.values
            }
            Child::Lexical(leaf) => vec![BuiltCandidate {
                value: BuildValue::Leaf(leaf.clone()),
                constructions: Vec::new(),
                positions: Vec::new(),
            }],
        };
        let mut next = Vec::new();
        for combination in combinations {
            for child_value in &child_values {
                let mut combination = combination.clone();
                combination.push(child_value.clone());
                next.push(combination);
            }
        }
        combinations = next;
    }
    let mut values = Vec::new();
    for children in combinations {
        let child_values = children
            .iter()
            .map(|child| child.value.clone())
            .collect::<Vec<_>>();
        if let Some(value) = build(rule_id, &child_values, context) {
            let mut constructions = vec![construction];
            let mut positions = rule.rhs.to_vec();
            for child in children {
                constructions.extend(child.constructions);
                positions.extend(child.positions);
            }
            push_unique(
                &mut values,
                BuiltCandidate {
                    value,
                    constructions,
                    positions,
                },
            );
        }
    }
    MaterializationOutcome {
        values,
        cycle_pruned,
    }
}

fn push_unique<T: PartialEq>(values: &mut Vec<T>, value: T) {
    if !values.contains(&value) {
        values.push(value);
    }
}

#[cfg(test)]
mod tests {
    use std::path::Path;

    use super::BuildValue;
    use super::Category;
    use super::Construction;
    use super::Forest;
    use super::Leaf;
    use super::Lexical;
    use super::MaterializationState;
    use super::RuleId;
    use super::materialize;
    use super::materialize_node;
    use crate::ast::Clause;
    use crate::ast::Connive;
    use crate::ast::Imperative;
    use crate::ast::NounPhrase;
    use crate::ast::Pronoun;
    use crate::ast::PronounNp;
    use crate::ast::Sentence;
    use crate::ast::Sign;
    use crate::ast::SignedNumber;
    use crate::ast::Variable;
    use crate::ast::VerbLexeme;
    use crate::ast::VerbPhrase;
    use crate::ast::WhereClause;
    use crate::ast::WithWhere;
    use crate::catalogs::ParserCatalogs;
    use crate::context::ParseContext;
    use crate::parser::build::Agreement;
    use crate::parser::engine::ChartFailure;
    use crate::parser::engine::Child;
    use crate::parser::engine::Family;
    use crate::parser::engine::NodeId;
    use crate::parser::engine::PackedNode;
    use crate::parser::engine::RulePosition;
    use crate::parser::rules::NounNumber;
    use crate::parser::scan::SliceGrammar;
    use crate::parser::scan::parse_forest;
    use crate::render::Render;

    fn slice_candidates(
        text: &str,
        card_name: &str,
    ) -> Result<Forest<RuleId, Leaf>, ChartFailure<Category, Lexical>> {
        let catalogs = ParserCatalogs::load(
            &Path::new(env!("CARGO_MANIFEST_DIR")).join("../../data/gen/catalogs"),
        )
        .expect("canonical generated catalogs load");
        let context = context(card_name);
        let grammar = SliceGrammar {
            catalogs: &catalogs,
            context: &context,
        };

        parse_forest(&grammar, text)
    }

    fn context(card_name: &str) -> ParseContext<'_> {
        ParseContext::new(card_name).expect("test card names are valid parse contexts")
    }

    fn assert_acyclic_number_survives(forest: &Forest<RuleId, Leaf>) {
        let mut state = MaterializationState::default();
        let outcome = materialize_node(forest, NodeId(0), &context("Context Card"), &mut state);

        assert_eq!(outcome.values.len(), 1);
        assert!(matches!(
            outcome.values[0].value,
            BuildValue::Amount(crate::ast::Amount::Number(crate::ast::NumberAmount {
                number: SignedNumber {
                    sign: Sign::Positive,
                    magnitude: 3,
                },
            }))
        ));
    }

    fn with_where_family(body: NodeId) -> Family<Leaf> {
        Family {
            children: vec![
                Child::Node(body),
                Child::Lexical(Leaf::Literal(",")),
                Child::Node(NodeId(4)),
            ],
        }
    }
    #[test]
    fn materialization_rejects_a_direct_nullable_cycle_but_keeps_an_acyclic_family() {
        let forest = Forest::from_test_parts(
            vec![PackedNode {
                rule: RuleId::AmountNumber,
                start: 0,
                end: 0,
                families: vec![
                    Family {
                        children: vec![Child::Node(NodeId(0))],
                    },
                    Family {
                        children: vec![Child::Lexical(Leaf::SignedNumber(SignedNumber {
                            sign: Sign::Positive,
                            magnitude: 3,
                        }))],
                    },
                ],
            }],
            vec![NodeId(0)],
        );

        assert_acyclic_number_survives(&forest);
    }

    #[test]
    fn materialization_rejects_an_indirect_nullable_cycle_but_keeps_an_acyclic_family() {
        let forest = Forest::from_test_parts(
            vec![
                PackedNode {
                    rule: RuleId::AmountNumber,
                    start: 0,
                    end: 0,
                    families: vec![
                        Family {
                            children: vec![Child::Node(NodeId(1))],
                        },
                        Family {
                            children: vec![Child::Lexical(Leaf::SignedNumber(SignedNumber {
                                sign: Sign::Positive,
                                magnitude: 3,
                            }))],
                        },
                    ],
                },
                PackedNode {
                    rule: RuleId::AmountNumber,
                    start: 0,
                    end: 0,
                    families: vec![Family {
                        children: vec![Child::Node(NodeId(0))],
                    }],
                },
            ],
            vec![NodeId(0)],
        );

        assert_acyclic_number_survives(&forest);
    }

    #[test]
    fn cycle_tainted_results_are_recomputed_for_a_later_clean_root() {
        let forest = Forest::from_test_parts(
            vec![
                PackedNode {
                    rule: RuleId::SentenceWithWhere,
                    start: 0,
                    end: 0,
                    families: vec![with_where_family(NodeId(1)), with_where_family(NodeId(2))],
                },
                PackedNode {
                    rule: RuleId::SentenceWithWhere,
                    start: 0,
                    end: 0,
                    families: vec![with_where_family(NodeId(0))],
                },
                PackedNode {
                    rule: RuleId::SentenceImperative,
                    start: 0,
                    end: 0,
                    families: vec![Family {
                        children: vec![Child::Node(NodeId(3))],
                    }],
                },
                PackedNode {
                    rule: RuleId::VerbPhraseConnive,
                    start: 0,
                    end: 0,
                    families: vec![Family {
                        children: vec![Child::Lexical(Leaf::Verb {
                            lexeme: VerbLexeme::Connive,
                            agreement: Agreement::Bare,
                        })],
                    }],
                },
                PackedNode {
                    rule: RuleId::ClauseWhere,
                    start: 0,
                    end: 0,
                    families: vec![Family {
                        children: vec![
                            Child::Lexical(Leaf::Literal("where")),
                            Child::Lexical(Leaf::Variable(Variable::X)),
                            Child::Lexical(Leaf::Verb {
                                lexeme: VerbLexeme::Be,
                                agreement: Agreement::ThirdPersonSingular,
                            }),
                            Child::Lexical(Leaf::Literal("the")),
                            Child::Lexical(Leaf::Literal("number")),
                            Child::Lexical(Leaf::Literal("of")),
                            Child::Node(NodeId(5)),
                        ],
                    }],
                },
                PackedNode {
                    rule: RuleId::NounPhrasePronoun,
                    start: 0,
                    end: 0,
                    families: vec![Family {
                        children: vec![Child::Lexical(Leaf::Pronoun(Pronoun::You))],
                    }],
                },
            ],
            vec![NodeId(0), NodeId(1)],
        );
        let context = context("Context Card");
        let mut state = MaterializationState::default();
        let clause = Clause::Where(WhereClause {
            variable: Variable::X,
            value: NounPhrase::Pronoun(PronounNp { word: Pronoun::You }),
        });
        let base = Sentence::Imperative(Imperative {
            predicate: VerbPhrase::Connive(Connive),
        });
        let once = Sentence::WithWhere(WithWhere {
            body: Box::new(base),
            clause: clause.clone(),
        });

        let first = materialize_node(&forest, NodeId(0), &context, &mut state);
        assert_eq!(first.values.len(), 1);
        assert_eq!(first.values[0].value, BuildValue::Sentence(once.clone()));

        let later = materialize_node(&forest, NodeId(1), &context, &mut state);
        assert_eq!(later.values.len(), 1);
        assert_eq!(
            later.values[0].value,
            BuildValue::Sentence(Sentence::WithWhere(WithWhere {
                body: Box::new(once),
                clause,
            }))
        );
        assert_eq!(
            later.values[0].constructions,
            vec![
                Construction::SentenceWithWhere,
                Construction::SentenceWithWhere,
                Construction::SentenceImperative,
                Construction::VerbPhraseConnive,
                Construction::ClauseWhere,
                Construction::NounPhrasePronoun,
                Construction::ClauseWhere,
                Construction::NounPhrasePronoun,
            ]
        );
        assert_eq!(
            later.values[0].positions,
            vec![
                RulePosition::Nonterminal(Category::Sentence),
                RulePosition::Lexical(Lexical::Literal(",")),
                RulePosition::Nonterminal(Category::Clause),
                RulePosition::Nonterminal(Category::Sentence),
                RulePosition::Lexical(Lexical::Literal(",")),
                RulePosition::Nonterminal(Category::Clause),
                RulePosition::Nonterminal(Category::VerbPhrase),
                RulePosition::Lexical(Lexical::Verb(VerbLexeme::Connive)),
                RulePosition::Lexical(Lexical::Literal("where")),
                RulePosition::Lexical(Lexical::Variable),
                RulePosition::Lexical(Lexical::Verb(VerbLexeme::Be)),
                RulePosition::Lexical(Lexical::Literal("the")),
                RulePosition::Lexical(Lexical::Literal("number")),
                RulePosition::Lexical(Lexical::Literal("of")),
                RulePosition::Nonterminal(Category::NounPhrase),
                RulePosition::Lexical(Lexical::Pronoun),
                RulePosition::Lexical(Lexical::Literal("where")),
                RulePosition::Lexical(Lexical::Variable),
                RulePosition::Lexical(Lexical::Verb(VerbLexeme::Be)),
                RulePosition::Lexical(Lexical::Literal("the")),
                RulePosition::Lexical(Lexical::Literal("number")),
                RulePosition::Lexical(Lexical::Literal("of")),
                RulePosition::Nonterminal(Category::NounPhrase),
                RulePosition::Lexical(Lexical::Pronoun),
            ]
        );
    }
    #[test]
    fn materialize_builds_checked_slice_values() {
        for (text, card_name) in [
            ("Destroy target creature.", "Context Card"),
            (
                "Whenever a player connives, you gain X life.",
                "Context Card",
            ),
            (
                "You gain X life, where X is the number of creatures you control with power 2 or less.",
                "Context Card",
            ),
            (
                "Zacama deals 3 damage to target creature.",
                "Zacama, Primal Calamity",
            ),
        ] {
            let forest = slice_candidates(text, card_name).expect("scanner accepts rendered input");
            let context = context(card_name);
            let candidates = materialize(&forest, &context);
            assert_eq!(candidates.len(), 1, "unexpected candidates for {text:?}");
            assert_eq!(candidates[0].ability.render(&context), text);
        }
    }
    #[test]
    fn materialize_accepts_a_plural_count_subject_with_a_bare_verb() {
        let text = "Creatures you control with power 2 or less gain X life.";
        let forest = slice_candidates(text, "Context Card").expect("scanner accepts words");
        let context = context("Context Card");
        let candidates = materialize(&forest, &context);
        assert_eq!(candidates.len(), 1);
        assert_eq!(candidates[0].ability.render(&context), text);
    }

    #[test]
    fn materialize_preserves_preorder_constructions_and_declared_positions() {
        let forest = slice_candidates("Destroy target creature.", "Context Card").unwrap();
        let candidates = materialize(&forest, &context("Context Card"));

        assert_eq!(candidates.len(), 1);
        assert_eq!(
            candidates[0].constructions,
            vec![
                Construction::AbilitySpell,
                Construction::SentenceImperative,
                Construction::VerbPhraseDestroy,
                Construction::NounPhraseTarget,
            ]
        );
        assert_eq!(
            candidates[0].positions,
            vec![
                RulePosition::Nonterminal(Category::Sentence),
                RulePosition::Lexical(Lexical::Literal(".")),
                RulePosition::Lexical(Lexical::EndOfInput),
                RulePosition::Nonterminal(Category::VerbPhrase),
                RulePosition::Lexical(Lexical::Verb(VerbLexeme::Destroy)),
                RulePosition::Nonterminal(Category::NounPhrase),
                RulePosition::Lexical(Lexical::Literal("target")),
                RulePosition::Lexical(Lexical::Noun(NounNumber::Singular)),
            ]
        );
    }
}
