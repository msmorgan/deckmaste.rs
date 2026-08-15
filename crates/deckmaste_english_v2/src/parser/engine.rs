#![allow(
    dead_code,
    reason = "later parser tasks consume these crate-internal engine contracts"
)]

use std::collections::BTreeMap;
use std::collections::BTreeSet;
use std::collections::VecDeque;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Ord, PartialOrd)]
pub(crate) enum RulePosition<N, L> {
    Nonterminal(N),
    Lexical(L),
}

pub(crate) struct Rule<N: 'static, L: 'static, C> {
    pub lhs: N,
    pub rhs: &'static [RulePosition<N, L>],
    pub construction: C,
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

pub(crate) struct PackedNode<C, T> {
    pub construction: C,
    pub start: usize,
    pub end: usize,
    pub families: Vec<Family<T>>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum SeedPolicy {
    StartOnly,
    AllRules,
}

pub(crate) struct Forest<C, T> {
    nodes: Vec<PackedNode<C, T>>,
    accepted_roots: Vec<NodeId>,
}

impl<C, T> Forest<C, T> {
    pub(crate) fn accepted_roots(&self) -> impl Iterator<Item = &PackedNode<C, T>> {
        self.accepted_roots
            .iter()
            .map(|&NodeId(index)| &self.nodes[index])
    }

    #[cfg(test)]
    fn nodes_for_span(&self, start: usize, end: usize) -> impl Iterator<Item = &PackedNode<C, T>> {
        self.nodes
            .iter()
            .filter(move |node| (node.start, node.end) == (start, end))
    }
}

#[derive(Debug, PartialEq, Eq)]
pub(crate) struct ChartFailure<N, L> {
    pub offset: usize,
    pub live: BTreeSet<RulePosition<N, L>>,
}

pub(crate) fn parse<N, L, C, T, Scan>(
    rules: &'static [Rule<N, L, C>],
    start: N,
    seed_policy: SeedPolicy,
    input_length: usize,
    mut scan: Scan,
) -> Result<Forest<C, T>, ChartFailure<N, L>>
where
    N: Copy + Eq + Ord + 'static,
    L: Copy + Eq + Ord + 'static,
    C: Clone + Eq,
    T: Clone + Eq,
    Scan: FnMut(L, usize) -> Vec<LexicalMatch<T>>,
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

    for column in 0..=input_length {
        if matches!(seed_policy, SeedPolicy::StartOnly) && column != 0 {
            continue;
        }
        for (rule_index, rule) in rules.iter().enumerate() {
            if matches!(seed_policy, SeedPolicy::StartOnly) && rule.lhs != start {
                continue;
            }
            insert_item(
                &mut chart,
                &mut agenda,
                column,
                (rule_index, 0, column),
                Family {
                    children: Vec::new(),
                },
            );
        }
    }

    while let Some((column, (rule_index, dot, origin), family)) = agenda.pop_front() {
        let rule = &rules[rule_index];
        if dot == rule.rhs.len() {
            let node_id = forest
                .nodes
                .iter()
                .position(|node| {
                    node.construction == rule.construction
                        && node.start == origin
                        && node.end == column
                })
                .map(NodeId)
                .unwrap_or_else(|| {
                    let node_id = NodeId(forest.nodes.len());
                    forest.nodes.push(PackedNode {
                        construction: rule.construction.clone(),
                        start: origin,
                        end: column,
                        families: Vec::new(),
                    });
                    node_id
                });
            let node = &mut forest.nodes[node_id.0];
            if !insert_family(&mut node.families, family) {
                continue;
            }

            if !completed_by_start[origin].contains(&(rule.lhs, node_id)) {
                completed_by_start[origin].push((rule.lhs, node_id));
            }
            if origin == 0
                && column == input_length
                && (matches!(seed_policy, SeedPolicy::AllRules) || rule.lhs == start)
                && !forest.accepted_roots.contains(&node_id)
            {
                forest.accepted_roots.push(node_id);
            }

            let waiters = chart[origin]
                .iter()
                .filter_map(|(&(waiter_rule, waiter_dot, waiter_origin), families)| {
                    matches!(
                        rules[waiter_rule].rhs.get(waiter_dot),
                        Some(RulePosition::Nonterminal(category)) if *category == rule.lhs
                    )
                    .then(|| ((waiter_rule, waiter_dot, waiter_origin), families.clone()))
                })
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

fn chart_failure<N, L, C, T>(
    chart: &[BTreeMap<ItemKey, Vec<Family<T>>>],
    rules: &[Rule<N, L, C>],
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
            let live = column
                .iter()
                .filter_map(|(&(rule_index, dot, _), families)| {
                    (!families.is_empty())
                        .then(|| rules[rule_index].rhs.get(dot))
                        .flatten()
                        .cloned()
                })
                .collect::<BTreeSet<_>>();
            (!live.is_empty()).then_some(ChartFailure { offset, live })
        })
        .unwrap_or(ChartFailure {
            offset: 0,
            live: BTreeSet::new(),
        })
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeSet;

    use super::*;

    #[derive(Debug, Clone, Copy, PartialEq, Eq, Ord, PartialOrd)]
    enum ToyCategory {
        Start,
        Left,
        Right,
        Fragment,
    }

    #[derive(Debug, Clone, PartialEq, Eq)]
    enum ToyConstruction {
        Start,
        Left,
        Right,
        Fragment,
    }

    const SCAN_RULES: &[Rule<ToyCategory, &'static str, ToyConstruction>] = &[Rule {
        lhs: ToyCategory::Start,
        rhs: &[RulePosition::Lexical("alpha beta")],
        construction: ToyConstruction::Start,
    }];

    const AMBIGUOUS_RULES: &[Rule<ToyCategory, &'static str, ToyConstruction>] = &[
        Rule {
            lhs: ToyCategory::Start,
            rhs: &[RulePosition::Nonterminal(ToyCategory::Left)],
            construction: ToyConstruction::Start,
        },
        Rule {
            lhs: ToyCategory::Start,
            rhs: &[RulePosition::Nonterminal(ToyCategory::Right)],
            construction: ToyConstruction::Start,
        },
        Rule {
            lhs: ToyCategory::Left,
            rhs: &[RulePosition::Lexical("alpha")],
            construction: ToyConstruction::Left,
        },
        Rule {
            lhs: ToyCategory::Right,
            rhs: &[RulePosition::Lexical("alpha")],
            construction: ToyConstruction::Right,
        },
    ];

    const SEQUENCE_RULES: &[Rule<ToyCategory, &'static str, ToyConstruction>] = &[Rule {
        lhs: ToyCategory::Start,
        rhs: &[
            RulePosition::Lexical("alpha"),
            RulePosition::Lexical("beta"),
        ],
        construction: ToyConstruction::Start,
    }];

    const FRAGMENT_RULES: &[Rule<ToyCategory, &'static str, ToyConstruction>] = &[
        Rule {
            lhs: ToyCategory::Start,
            rhs: &[RulePosition::Lexical("alpha")],
            construction: ToyConstruction::Start,
        },
        Rule {
            lhs: ToyCategory::Fragment,
            rhs: &[RulePosition::Lexical("beta")],
            construction: ToyConstruction::Fragment,
        },
    ];

    const BACKWARDS_SCAN_RULES: &[Rule<ToyCategory, &'static str, ToyConstruction>] = &[Rule {
        lhs: ToyCategory::Start,
        rhs: &[
            RulePosition::Lexical("alpha"),
            RulePosition::Lexical("backward"),
            RulePosition::Lexical("finish"),
        ],
        construction: ToyConstruction::Start,
    }];

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
        seed_policy: SeedPolicy,
    ) -> Result<Forest<ToyConstruction, &'static str>, ChartFailure<ToyCategory, &'static str>>
    {
        parse(
            SCAN_RULES,
            ToyCategory::Start,
            seed_policy,
            input.len(),
            |literal, start| {
                (input.get(start..) == Some(literal))
                    .then_some(vec![LexicalMatch {
                        end: start + literal.len(),
                        value: literal,
                    }])
                    .unwrap_or_default()
            },
        )
    }

    fn parse_ambiguous_toy(
        input: &str,
    ) -> Result<Forest<ToyConstruction, &'static str>, ChartFailure<ToyCategory, &'static str>>
    {
        parse(
            AMBIGUOUS_RULES,
            ToyCategory::Start,
            SeedPolicy::StartOnly,
            input.len(),
            |literal, start| scan_words(input, literal, start),
        )
    }

    fn parse_sequence_toy(
        input: &str,
    ) -> Result<Forest<ToyConstruction, &'static str>, ChartFailure<ToyCategory, &'static str>>
    {
        parse(
            SEQUENCE_RULES,
            ToyCategory::Start,
            SeedPolicy::StartOnly,
            input.len(),
            |literal, start| scan_words(input, literal, start),
        )
    }

    fn parse_fragment_toy(
        input: &str,
        seed_policy: SeedPolicy,
    ) -> Result<Forest<ToyConstruction, &'static str>, ChartFailure<ToyCategory, &'static str>>
    {
        parse(
            FRAGMENT_RULES,
            ToyCategory::Start,
            seed_policy,
            input.len(),
            |literal, start| scan_words(input, literal, start),
        )
    }

    fn parse_with_backwards_scan()
    -> Result<Forest<ToyConstruction, &'static str>, ChartFailure<ToyCategory, &'static str>> {
        parse(
            BACKWARDS_SCAN_RULES,
            ToyCategory::Start,
            SeedPolicy::StartOnly,
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
        )
    }

    #[test]
    fn scan_injection_can_consume_a_multi_word_span() {
        let forest = parse_toy("alpha beta", SeedPolicy::StartOnly).unwrap();
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
    fn same_construction_derivations_pack_into_one_root_family_set() {
        let forest = parse_ambiguous_toy("alpha").unwrap();
        let roots = forest.accepted_roots().collect::<Vec<_>>();
        assert_eq!(roots.len(), 1);
        let root = roots[0];
        assert_eq!(root.construction, ToyConstruction::Start);
        assert_eq!(root.families.len(), 2);
        assert!(
            root.families
                .iter()
                .all(|family| matches!(family.children.as_slice(), [Child::Node(_)]))
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
    fn all_rule_seeding_is_a_policy_not_a_second_parser() {
        assert!(parse_fragment_toy("beta", SeedPolicy::StartOnly).is_err());
        assert!(parse_fragment_toy("beta", SeedPolicy::AllRules).is_ok());
    }

    #[test]
    fn backwards_lexical_matches_are_rejected() {
        assert!(parse_with_backwards_scan().is_err());
    }
}
