use std::collections::BTreeMap;
use std::collections::BTreeSet;
use std::collections::VecDeque;

use super::TextSpan;

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

pub(crate) struct LexicalMatch<T, O = ()> {
    pub end: usize,
    pub value: T,
    pub owner: Option<O>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct SpannedLexical<T, O = ()> {
    pub(crate) span: TextSpan,
    pub(crate) value: T,
    pub(crate) owner: Option<O>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum Child<T, O = ()> {
    Node(NodeId),
    Lexical(SpannedLexical<T, O>),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Ord, PartialOrd)]
pub(crate) struct NodeId(pub usize);

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct Family<T, O = ()> {
    pub children: Vec<Child<T, O>>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct PackedNode<R, T, O = ()> {
    pub rule: R,
    pub start: usize,
    pub end: usize,
    pub families: Vec<Family<T, O>>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct Forest<R, T, O = ()> {
    nodes: Vec<PackedNode<R, T, O>>,
    accepted_roots: Vec<NodeId>,
}

impl<R, T, O> Forest<R, T, O> {
    pub(crate) fn node(&self, id: NodeId) -> &PackedNode<R, T, O> {
        &self.nodes[id.0]
    }

    #[cfg(test)]
    pub(crate) fn accepted_roots(&self) -> impl Iterator<Item = &PackedNode<R, T, O>> {
        self.accepted_roots
            .iter()
            .map(|&NodeId(index)| &self.nodes[index])
    }

    pub(crate) fn accepted_root_ids(&self) -> impl Iterator<Item = NodeId> + '_ {
        self.accepted_roots.iter().copied()
    }

    pub(crate) fn nodes(&self) -> impl Iterator<Item = (NodeId, &PackedNode<R, T, O>)> {
        self.nodes
            .iter()
            .enumerate()
            .map(|(index, node)| (NodeId(index), node))
    }

    #[cfg(test)]
    pub(crate) fn from_test_parts(
        nodes: Vec<PackedNode<R, T, O>>,
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

pub(crate) trait Observation<R, T, L, O = ()> {
    fn scanned(&mut self, _start: usize, _terminal: L, _end: usize, _value: &T) {}
    fn checked_completion(
        &mut self,
        _rule: R,
        _start: usize,
        _end: usize,
        _family: &Family<T, O>,
        _accepted: bool,
    ) {
    }
    fn chart_item(
        &mut self,
        _column: usize,
        _rule: R,
        _dot: usize,
        _origin: usize,
        _family_count: usize,
    ) {
    }
    fn final_forest(&mut self, _forest: &Forest<R, T, O>) {}
}

impl<R, T, L, O> Observation<R, T, L, O> for () {}

pub(crate) fn parse<N, L, R, T, O, Scan, ValidateCompletion>(
    rules: &'static [Rule<N, L, R>],
    start: N,
    input_length: usize,
    scan: Scan,
    validate_completion: ValidateCompletion,
) -> Result<Forest<R, T, O>, ChartFailure<N, L>>
where
    N: Copy + Eq + Ord + 'static,
    L: Copy + Eq + Ord + std::fmt::Debug + 'static,
    R: Copy + Eq,
    T: Clone + Eq,
    O: Clone + Eq,
    Scan: FnMut(L, usize) -> Vec<LexicalMatch<T, O>>,
    ValidateCompletion: FnMut(R, &Family<T, O>, &Forest<R, T, O>) -> bool,
{
    parse_observed(
        rules,
        start,
        input_length,
        scan,
        validate_completion,
        &mut (),
    )
}

pub(crate) fn parse_observed<N, L, R, T, Owner, Scan, ValidateCompletion, Obs>(
    rules: &'static [Rule<N, L, R>],
    start: N,
    input_length: usize,
    mut scan: Scan,
    mut validate_completion: ValidateCompletion,
    observation: &mut Obs,
) -> Result<Forest<R, T, Owner>, ChartFailure<N, L>>
where
    N: Copy + Eq + Ord + 'static,
    L: Copy + Eq + Ord + std::fmt::Debug + 'static,
    R: Copy + Eq,
    T: Clone + Eq,
    Owner: Clone + Eq,
    Scan: FnMut(L, usize) -> Vec<LexicalMatch<T, Owner>>,
    ValidateCompletion: FnMut(R, &Family<T, Owner>, &Forest<R, T, Owner>) -> bool,
    Obs: Observation<R, T, L, Owner>,
{
    let mut forest = Forest {
        nodes: Vec::new(),
        accepted_roots: Vec::new(),
    };

    let mut chart = (0..=input_length)
        .map(|_| BTreeMap::<ItemKey, Vec<Family<T, Owner>>>::new())
        .collect::<Vec<_>>();
    let mut agenda = VecDeque::new();
    let mut completed_by_start = (0..=input_length)
        .map(|_| Vec::<(N, NodeId)>::new())
        .collect::<Vec<_>>();

    seed_chart(rules, start, &mut chart, &mut agenda);

    while let Some((column, (rule_index, dot, origin), family)) = agenda.pop_front() {
        let rule = &rules[rule_index];
        if dot == rule.rhs.len() {
            let accepted = validate_completion(rule.id, &family, &forest);
            observation.checked_completion(rule.id, origin, column, &family, accepted);
            if !accepted {
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
                    observation.scanned(column, lexical, lexical_match.end, &lexical_match.value);
                    let mut children = family.children.clone();
                    children.push(Child::Lexical(SpannedLexical {
                        span: TextSpan {
                            start: column,
                            end: lexical_match.end,
                        },
                        value: lexical_match.value,
                        owner: lexical_match.owner,
                    }));
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

    for (column, items) in chart.iter().enumerate() {
        for (&(rule_index, dot, origin), families) in items {
            observation.chart_item(column, rules[rule_index].id, dot, origin, families.len());
        }
    }
    observation.final_forest(&forest);

    if forest.accepted_roots.is_empty() {
        Err(chart_failure(&chart, rules))
    } else {
        Ok(forest)
    }
}

fn seed_chart<N, L, R, T, O>(
    rules: &[Rule<N, L, R>],
    start: N,
    chart: &mut [BTreeMap<ItemKey, Vec<Family<T, O>>>],
    agenda: &mut VecDeque<(usize, ItemKey, Family<T, O>)>,
) where
    N: Copy + Eq,
    T: Clone + Eq,
    O: Clone + Eq,
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

fn insert_item<T: Clone + Eq, O: Clone + Eq>(
    chart: &mut [BTreeMap<ItemKey, Vec<Family<T, O>>>],
    agenda: &mut VecDeque<(usize, ItemKey, Family<T, O>)>,
    column: usize,
    item: ItemKey,
    family: Family<T, O>,
) {
    let families = chart[column].entry(item).or_default();
    if insert_family(families, family.clone()) {
        agenda.push_back((column, item, family));
    }
}

fn insert_family<T: Eq, O: Eq>(families: &mut Vec<Family<T, O>>, family: Family<T, O>) -> bool {
    if families.contains(&family) {
        false
    } else {
        families.push(family);
        true
    }
}

fn requeue_completed_items_after_forest_growth<N, L, R, T: Clone, O: Clone>(
    chart: &[BTreeMap<ItemKey, Vec<Family<T, O>>>],
    rules: &[Rule<N, L, R>],
    agenda: &mut VecDeque<(usize, ItemKey, Family<T, O>)>,
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

fn chart_failure<N, L, R, T, O>(
    chart: &[BTreeMap<ItemKey, Vec<Family<T, O>>>],
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

fn live_expectations<N, L, R, T, O>(
    column: &BTreeMap<ItemKey, Vec<Family<T, O>>>,
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

    #[test]
    fn selected_derivation_spans_retain_exact_owner_and_range() {
        let forest = parse(
            SCAN_RULES,
            ToyCategory::Start,
            10,
            |_, start| {
                (start == 0)
                    .then_some(vec![LexicalMatch {
                        end: 10,
                        value: "alpha beta",
                        owner: Some("toy:alpha-beta"),
                    }])
                    .unwrap_or_default()
            },
            |_, _, _| true,
        )
        .expect("owned lexical span accepts");
        let root = forest.accepted_roots().next().expect("one root");
        let Child::Lexical(lexical) = &root.families[0].children[0] else {
            panic!("root child is lexical");
        };
        assert_eq!(lexical.span, TextSpan { start: 0, end: 10 });
        assert_eq!(lexical.value, "alpha beta");
        assert_eq!(lexical.owner, Some("toy:alpha-beta"));
    }

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
                owner: None,
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
                        owner: None,
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
                    owner: None,
                }],
                ("backward", 5) => vec![LexicalMatch {
                    end: 4,
                    value: literal,
                    owner: None,
                }],
                ("finish", 4) => vec![LexicalMatch {
                    end: 10,
                    value: literal,
                    owner: None,
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
        let Child::Lexical(lexical) = &root.families[0].children[0] else {
            panic!("scan result is lexical");
        };
        assert_eq!(lexical.value, "alpha beta");
        assert_eq!(lexical.span, TextSpan { start: 0, end: 10 });
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
                        owner: None,
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

    #[test]
    fn structural_trace_delayed_growth_rechecks_and_chart_keys_are_final() {
        #[derive(Default)]
        struct Recording {
            checked: Vec<(ToyRuleId, usize, usize, Family<&'static str>, bool)>,
            chart: Vec<(usize, ToyRuleId, usize, usize, usize)>,
        }
        impl Observation<ToyRuleId, &'static str, &'static str> for Recording {
            fn checked_completion(
                &mut self,
                rule: ToyRuleId,
                start: usize,
                end: usize,
                family: &Family<&'static str>,
                accepted: bool,
            ) {
                self.checked
                    .push((rule, start, end, family.clone(), accepted));
            }
            fn chart_item(
                &mut self,
                column: usize,
                rule: ToyRuleId,
                dot: usize,
                origin: usize,
                families: usize,
            ) {
                self.chart.push((column, rule, dot, origin, families));
            }
        }
        let mut observed = Recording::default();
        let forest = parse_observed(
            DELAYED_PACKING_RULES,
            ToyCategory::Start,
            1,
            |literal, start| {
                (start == 0)
                    .then_some(vec![LexicalMatch {
                        end: 1,
                        value: literal,
                        owner: None,
                    }])
                    .unwrap_or_default()
            },
            |rule, family, forest| match rule {
                ToyRuleId::DirectParent | ToyRuleId::DirectStart | ToyRuleId::WrapperStart => {
                    reaches_delayed_child(forest, family)
                }
                _ => true,
            },
            &mut observed,
        )
        .expect("delayed growth succeeds");
        assert_eq!(forest.accepted_roots().count(), 2);
        assert!(observed.checked.iter().enumerate().any(|(index, event)| {
            !event.4
                && observed.checked[index + 1..].iter().any(|later| {
                    later.0 == event.0
                        && later.1 == event.1
                        && later.2 == event.2
                        && later.3 == event.3
                        && later.4
                })
        }));
        assert!(observed.chart.iter().all(|event| event.4 > 0));
        for (index, event) in observed.chart.iter().enumerate() {
            assert!(
                !observed.chart[..index]
                    .iter()
                    .any(|prior| prior.0 == event.0
                        && prior.1 == event.1
                        && prior.2 == event.2
                        && prior.3 == event.3)
            );
        }
    }

    #[test]
    fn structural_trace_grammar_scanner_retries_dedupes_and_keeps_overlaps() {
        const TOKEN_TRACE_RULES: &[Rule<ToyCategory, &'static str, ToyRuleId>] = &[
            Rule {
                id: ToyRuleId::DirectStart,
                lhs: ToyCategory::Start,
                rhs: &[RulePosition::Lexical("shared")],
            },
            Rule {
                id: ToyRuleId::WrapperStart,
                lhs: ToyCategory::Start,
                rhs: &[RulePosition::Lexical("shared")],
            },
        ];
        struct TokenRecording {
            inventory: super::super::diagnostic::SemanticTokenInventory<&'static str, &'static str>,
            callbacks: usize,
        }
        impl Observation<ToyRuleId, &'static str, &'static str> for TokenRecording {
            fn scanned(
                &mut self,
                start: usize,
                terminal: &'static str,
                end: usize,
                value: &&'static str,
            ) {
                self.callbacks += 1;
                self.inventory.record(start, end, terminal, *value);
            }
        }
        let mut recording = TokenRecording {
            inventory: super::super::diagnostic::SemanticTokenInventory::default(),
            callbacks: 0,
        };
        assert!(
            parse_observed(
                TOKEN_TRACE_RULES,
                ToyCategory::Start,
                2,
                |terminal, start| (terminal == "shared" && start == 0)
                    .then_some(vec![
                        LexicalMatch {
                            end: 1,
                            value: "same",
                            owner: None,
                        },
                        LexicalMatch {
                            end: 2,
                            value: "overlap",
                            owner: None,
                        }
                    ])
                    .unwrap_or_default(),
                |_, _, _| true,
                &mut recording
            )
            .is_ok()
        );
        assert_eq!(recording.callbacks, 4);
        for (limit, expected) in [
            (0, vec![]),
            (1, vec![(0, 1, "shared", "same")]),
            (
                8,
                vec![(0, 1, "shared", "same"), (0, 2, "shared", "overlap")],
            ),
        ] {
            let bounded = recording.inventory.clone().into_bounded_by(
                limit,
                Ord::cmp,
                Ord::cmp,
                str::to_owned,
                |value| (*value).to_owned(),
            );
            assert_eq!(
                (bounded.total(), bounded.shown(), bounded.omitted()),
                (2, expected.len(), 2 - expected.len())
            );
            assert_eq!(
                bounded
                    .items()
                    .iter()
                    .map(|item| (
                        item.start,
                        item.end,
                        item.terminal_name_v1.as_str(),
                        item.value_label_v1.as_str()
                    ))
                    .collect::<Vec<_>>(),
                expected
            );
        }
    }
}
