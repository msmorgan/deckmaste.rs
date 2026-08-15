use std::collections::BTreeMap;
use std::collections::BTreeSet;
use std::collections::VecDeque;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Ord, PartialOrd)]
pub(crate) enum RulePosition<N, L> {
    Nonterminal(N),
    Lexical(L),
}

pub(crate) struct Rule<N: 'static, L: 'static, R> {
    pub id: R,
    pub lhs: N,
    pub rhs: &'static [RulePosition<N, L>],
}

pub(crate) struct LexicalMatch<T> {
    pub end: usize,
    pub value: T,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum Child<T> {
    Node(NodeId),
    Lexical(T),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Ord, PartialOrd)]
pub(crate) struct NodeId(pub usize);

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct Family<T> {
    pub children: Vec<Child<T>>,
}

pub(crate) struct PackedNode<R, T> {
    pub rule: R,
    pub start: usize,
    pub end: usize,
    pub families: Vec<Family<T>>,
}

pub(crate) struct Forest<R, T> {
    nodes: Vec<PackedNode<R, T>>,
    accepted_roots: Vec<NodeId>,
}

impl<R, T> Forest<R, T> {
    pub(crate) fn node(&self, id: NodeId) -> &PackedNode<R, T> {
        &self.nodes[id.0]
    }

    #[cfg(test)]
    pub(crate) fn accepted_roots(&self) -> impl Iterator<Item = &PackedNode<R, T>> {
        self.accepted_roots
            .iter()
            .map(|&NodeId(index)| &self.nodes[index])
    }

    pub(crate) fn accepted_root_ids(&self) -> impl Iterator<Item = NodeId> + '_ {
        self.accepted_roots.iter().copied()
    }

    #[cfg(test)]
    pub(crate) fn from_test_parts(
        nodes: Vec<PackedNode<R, T>>,
        accepted_roots: Vec<NodeId>,
    ) -> Self {
        Self {
            nodes,
            accepted_roots,
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub(crate) struct ChartFailure<N, L> {
    pub offset: usize,
    pub live: BTreeSet<RulePosition<N, L>>,
}

pub(crate) fn parse<N, L, R, T, Scan, ValidateCompletion>(
    rules: &'static [Rule<N, L, R>],
    start: N,
    input_length: usize,
    mut scan: Scan,
    mut validate_completion: ValidateCompletion,
) -> Result<Forest<R, T>, ChartFailure<N, L>>
where
    N: Copy + Eq + Ord + 'static,
    L: Copy + Eq + Ord + 'static,
    R: Copy + Eq,
    T: Clone + Eq,
    Scan: FnMut(L, usize) -> Vec<LexicalMatch<T>>,
    ValidateCompletion: FnMut(R, &Family<T>, &Forest<R, T>) -> bool,
{
    let mut forest = Forest {
        nodes: Vec::new(),
        accepted_roots: Vec::new(),
    };

    let mut chart = (0..=input_length)
        .map(|_| BTreeMap::<ItemKey, Vec<Family<T>>>::new())
        .collect::<Vec<_>>();
    let mut agenda = VecDeque::new();
    let mut completed_by_start = (0..=input_length)
        .map(|_| Vec::<(N, NodeId)>::new())
        .collect::<Vec<_>>();

    seed_chart(rules, start, &mut chart, &mut agenda);

    while let Some((column, (rule_index, dot, origin), family)) = agenda.pop_front() {
        let rule = &rules[rule_index];
        if dot == rule.rhs.len() {
            if !validate_completion(rule.id, &family, &forest) {
                continue;
            }
            let node_id = forest
                .nodes
                .iter()
                .position(|node| node.rule == rule.id && node.start == origin && node.end == column)
                .map_or_else(
                    || {
                        let node_id = NodeId(forest.nodes.len());
                        forest.nodes.push(PackedNode {
                            rule: rule.id,
                            start: origin,
                            end: column,
                            families: Vec::new(),
                        });
                        node_id
                    },
                    NodeId,
                );
            let node = &mut forest.nodes[node_id.0];
            if !insert_family(&mut node.families, family) {
                continue;
            }
            requeue_completed_items_after_forest_growth(&chart, rules, &mut agenda);

            if !completed_by_start[origin].contains(&(rule.lhs, node_id)) {
                completed_by_start[origin].push((rule.lhs, node_id));
            }
            if completed_rule_is_root(&rule.lhs, &start, origin, column, input_length)
                && !forest.accepted_roots.contains(&node_id)
            {
                forest.accepted_roots.push(node_id);
            }

            let waiters = chart[origin]
                .iter()
                .filter(|&(&(waiter_rule, waiter_dot, _), _)| {
                    matches!(
                        rules[waiter_rule].rhs.get(waiter_dot),
                        Some(RulePosition::Nonterminal(category)) if *category == rule.lhs
                    )
                })
                .map(|(&key, families)| (key, families.clone()))
                .collect::<Vec<_>>();
            for ((waiter_rule, waiter_dot, waiter_origin), waiter_families) in waiters {
                for waiter_family in waiter_families {
                    let mut children = waiter_family.children;
                    children.push(Child::Node(node_id));
                    insert_item(
                        &mut chart,
                        &mut agenda,
                        column,
                        (waiter_rule, waiter_dot + 1, waiter_origin),
                        Family { children },
                    );
                }
            }
            continue;
        }

        match rule.rhs[dot] {
            RulePosition::Nonterminal(category) => {
                for (predicted_rule, predicted) in rules.iter().enumerate() {
                    if predicted.lhs == category {
                        insert_item(
                            &mut chart,
                            &mut agenda,
                            column,
                            (predicted_rule, 0, column),
                            Family {
                                children: Vec::new(),
                            },
                        );
                    }
                }

                let completed = completed_by_start[column]
                    .iter()
                    .copied()
                    .filter(|(completed_category, _)| *completed_category == category)
                    .collect::<Vec<_>>();
                for (_, node_id) in completed {
                    let mut children = family.children.clone();
                    children.push(Child::Node(node_id));
                    let node = &forest.nodes[node_id.0];
                    insert_item(
                        &mut chart,
                        &mut agenda,
                        node.end,
                        (rule_index, dot + 1, origin),
                        Family { children },
                    );
                }
            }
            RulePosition::Lexical(lexical) => {
                for lexical_match in scan(lexical, column) {
                    if !(column..=input_length).contains(&lexical_match.end) {
                        continue;
                    }
                    let mut children = family.children.clone();
                    children.push(Child::Lexical(lexical_match.value));
                    insert_item(
                        &mut chart,
                        &mut agenda,
                        lexical_match.end,
                        (rule_index, dot + 1, origin),
                        Family { children },
                    );
                }
            }
        }
    }

    if forest.accepted_roots.is_empty() {
        Err(chart_failure(&chart, rules))
    } else {
        Ok(forest)
    }
}

fn seed_chart<N, L, R, T>(
    rules: &[Rule<N, L, R>],
    start: N,
    chart: &mut [BTreeMap<ItemKey, Vec<Family<T>>>],
    agenda: &mut VecDeque<(usize, ItemKey, Family<T>)>,
) where
    N: Copy + Eq,
    T: Clone + Eq,
{
    for (rule_index, rule) in rules.iter().enumerate() {
        if rule.lhs == start {
            insert_item(
                chart,
                agenda,
                0,
                (rule_index, 0, 0),
                Family {
                    children: Vec::new(),
                },
            );
        }
    }
}

fn completed_rule_is_root<N: Eq>(
    lhs: &N,
    start: &N,
    origin: usize,
    column: usize,
    input_length: usize,
) -> bool {
    lhs == start && origin == 0 && column == input_length
}

type ItemKey = (usize, usize, usize);

fn insert_item<T: Clone + Eq>(
    chart: &mut [BTreeMap<ItemKey, Vec<Family<T>>>],
    agenda: &mut VecDeque<(usize, ItemKey, Family<T>)>,
    column: usize,
    item: ItemKey,
    family: Family<T>,
) {
    let families = chart[column].entry(item).or_default();
    if insert_family(families, family.clone()) {
        agenda.push_back((column, item, family));
    }
}

fn insert_family<T: Eq>(families: &mut Vec<Family<T>>, family: Family<T>) -> bool {
    if families.contains(&family) {
        false
    } else {
        families.push(family);
        true
    }
}

fn requeue_completed_items_after_forest_growth<N, L, R, T: Clone>(
    chart: &[BTreeMap<ItemKey, Vec<Family<T>>>],
    rules: &[Rule<N, L, R>],
    agenda: &mut VecDeque<(usize, ItemKey, Family<T>)>,
) {
    for (column, items) in chart.iter().enumerate() {
        for (&item @ (rule_index, dot, _), families) in items {
            if dot != rules[rule_index].rhs.len() {
                continue;
            }
            for family in families {
                agenda.push_back((column, item, family.clone()));
            }
        }
    }
}

fn chart_failure<N, L, R, T>(
    chart: &[BTreeMap<ItemKey, Vec<Family<T>>>],
    rules: &[Rule<N, L, R>],
) -> ChartFailure<N, L>
where
    N: Clone + Ord,
    L: Clone + Ord,
{
    chart
        .iter()
        .enumerate()
        .rev()
        .find_map(|(offset, column)| {
            let live = live_expectations(column, rules);
            (!live.is_empty()).then_some(ChartFailure { offset, live })
        })
        .unwrap_or(ChartFailure {
            offset: 0,
            live: BTreeSet::new(),
        })
}

fn live_expectations<N, L, R, T>(
    column: &BTreeMap<ItemKey, Vec<Family<T>>>,
    rules: &[Rule<N, L, R>],
) -> BTreeSet<RulePosition<N, L>>
where
    N: Clone + Ord,
    L: Clone + Ord,
{
    column
        .iter()
        .filter_map(|(&(rule_index, dot, _), families)| {
            (!families.is_empty())
                .then(|| rules[rule_index].rhs.get(dot))
                .flatten()
                .cloned()
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeSet;

    use super::*;

    #[derive(Debug, Clone, Copy, PartialEq, Eq, Ord, PartialOrd)]
    enum ToyCategory {
        Start,
        DirectParent,
        Wrapper,
        Child,
        Delay,
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    enum ToyRuleId {
        Start,
        DirectStart,
        WrapperStart,
        DirectParent,
        Wrapper,
        FastChild,
        DelayedChild,
        Delay,
    }

    const SCAN_RULES: &[Rule<ToyCategory, &'static str, ToyRuleId>] = &[Rule {
        id: ToyRuleId::Start,
        lhs: ToyCategory::Start,
        rhs: &[RulePosition::Lexical("alpha beta")],
    }];

    const SEQUENCE_RULES: &[Rule<ToyCategory, &'static str, ToyRuleId>] = &[Rule {
        id: ToyRuleId::Start,
        lhs: ToyCategory::Start,
        rhs: &[
            RulePosition::Lexical("alpha"),
            RulePosition::Lexical("beta"),
        ],
    }];

    const BACKWARDS_SCAN_RULES: &[Rule<ToyCategory, &'static str, ToyRuleId>] = &[Rule {
        id: ToyRuleId::Start,
        lhs: ToyCategory::Start,
        rhs: &[
            RulePosition::Lexical("alpha"),
            RulePosition::Lexical("backward"),
            RulePosition::Lexical("finish"),
        ],
    }];

    const DELAYED_PACKING_RULES: &[Rule<ToyCategory, &'static str, ToyRuleId>] = &[
        Rule {
            id: ToyRuleId::DirectStart,
            lhs: ToyCategory::Start,
            rhs: &[RulePosition::Nonterminal(ToyCategory::DirectParent)],
        },
        Rule {
            id: ToyRuleId::WrapperStart,
            lhs: ToyCategory::Start,
            rhs: &[RulePosition::Nonterminal(ToyCategory::Wrapper)],
        },
        Rule {
            id: ToyRuleId::DirectParent,
            lhs: ToyCategory::DirectParent,
            rhs: &[RulePosition::Nonterminal(ToyCategory::Child)],
        },
        Rule {
            id: ToyRuleId::Wrapper,
            lhs: ToyCategory::Wrapper,
            rhs: &[RulePosition::Nonterminal(ToyCategory::Child)],
        },
        Rule {
            id: ToyRuleId::FastChild,
            lhs: ToyCategory::Child,
            rhs: &[RulePosition::Lexical("fast")],
        },
        Rule {
            id: ToyRuleId::DelayedChild,
            lhs: ToyCategory::Child,
            rhs: &[RulePosition::Nonterminal(ToyCategory::Delay)],
        },
        Rule {
            id: ToyRuleId::Delay,
            lhs: ToyCategory::Delay,
            rhs: &[RulePosition::Lexical("delayed")],
        },
    ];

    fn reaches_delayed_child(
        forest: &Forest<ToyRuleId, &'static str>,
        family: &Family<&'static str>,
    ) -> bool {
        family.children.iter().any(|child| {
            let Child::Node(node_id) = child else {
                return false;
            };
            let node = forest.node(*node_id);
            (matches!(node.rule, ToyRuleId::FastChild | ToyRuleId::DelayedChild)
                && node
                    .families
                    .iter()
                    .any(|family| matches!(family.children.as_slice(), [Child::Node(_)])))
                || node
                    .families
                    .iter()
                    .any(|family| reaches_delayed_child(forest, family))
        })
    }

    fn scan_words(
        input: &str,
        literal: &'static str,
        start: usize,
    ) -> Vec<LexicalMatch<&'static str>> {
        let Some(remainder) = input.get(start..) else {
            return Vec::new();
        };
        if remainder.starts_with(literal)
            && remainder
                .get(literal.len()..)
                .is_none_or(|following| following.is_empty() || following.starts_with(' '))
        {
            vec![LexicalMatch {
                end: start + literal.len(),
                value: literal,
            }]
        } else {
            Vec::new()
        }
    }

    fn parse_toy(
        input: &str,
    ) -> Result<Forest<ToyRuleId, &'static str>, ChartFailure<ToyCategory, &'static str>> {
        parse(
            SCAN_RULES,
            ToyCategory::Start,
            input.len(),
            |literal, start| {
                (input.get(start..) == Some(literal))
                    .then_some(vec![LexicalMatch {
                        end: start + literal.len(),
                        value: literal,
                    }])
                    .unwrap_or_default()
            },
            |_, _, _| true,
        )
    }

    fn parse_sequence_toy(
        input: &str,
    ) -> Result<Forest<ToyRuleId, &'static str>, ChartFailure<ToyCategory, &'static str>> {
        parse(
            SEQUENCE_RULES,
            ToyCategory::Start,
            input.len(),
            |literal, start| scan_words(input, literal, start),
            |_, _, _| true,
        )
    }

    fn parse_with_backwards_scan()
    -> Result<Forest<ToyRuleId, &'static str>, ChartFailure<ToyCategory, &'static str>> {
        parse(
            BACKWARDS_SCAN_RULES,
            ToyCategory::Start,
            10,
            |literal, start| match (literal, start) {
                ("alpha", 0) => vec![LexicalMatch {
                    end: 5,
                    value: literal,
                }],
                ("backward", 5) => vec![LexicalMatch {
                    end: 4,
                    value: literal,
                }],
                ("finish", 4) => vec![LexicalMatch {
                    end: 10,
                    value: literal,
                }],
                _ => Vec::new(),
            },
            |_, _, _| true,
        )
    }

    #[test]
    fn scan_injection_can_consume_a_multi_word_span() {
        let forest = parse_toy("alpha beta").unwrap();
        let roots = forest.accepted_roots().collect::<Vec<_>>();
        assert_eq!(roots.len(), 1);
        let root = roots[0];
        assert_eq!((root.start, root.end), (0, 10));
        assert_eq!(root.families.len(), 1);
        assert_eq!(
            root.families[0].children,
            vec![Child::Lexical("alpha beta")]
        );
    }

    #[test]
    fn failure_reports_the_furthest_live_column() {
        let Err(failure) = parse_sequence_toy("alpha nope") else {
            panic!("the incomplete sequence must fail");
        };
        assert_eq!(failure.offset, 5);
        assert_eq!(
            failure.live,
            BTreeSet::from([RulePosition::Lexical("beta")])
        );
    }

    #[test]
    fn backwards_lexical_matches_are_rejected() {
        assert!(parse_with_backwards_scan().is_err());
    }

    #[test]
    fn packed_family_growth_revalidates_all_completed_items() {
        let forest = parse(
            DELAYED_PACKING_RULES,
            ToyCategory::Start,
            1,
            |literal, start| {
                (start == 0)
                    .then_some(vec![LexicalMatch {
                        end: 1,
                        value: literal,
                    }])
                    .unwrap_or_default()
            },
            |rule, family, forest| match rule {
                ToyRuleId::DirectParent | ToyRuleId::DirectStart | ToyRuleId::WrapperStart => {
                    reaches_delayed_child(forest, family)
                }
                _ => true,
            },
        )
        .expect("delayed packed growth must revive every affected completion");

        let child_nodes = forest
            .nodes
            .iter()
            .filter(|node| matches!(node.rule, ToyRuleId::FastChild | ToyRuleId::DelayedChild))
            .collect::<Vec<_>>();
        assert_eq!(child_nodes.len(), 2);
        assert!(child_nodes.iter().all(|node| node.families.len() == 1));

        let roots = forest.accepted_roots().collect::<Vec<_>>();
        assert_eq!(roots.len(), 2);
        assert!(roots.iter().all(|root| root.families.len() == 1));
        assert!(
            forest
                .nodes
                .iter()
                .any(|node| node.rule == ToyRuleId::DirectParent)
        );
    }
}
