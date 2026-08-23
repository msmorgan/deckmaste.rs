use std::collections::BTreeMap;
use std::collections::BTreeSet;
use std::collections::VecDeque;

use super::TextSpan;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Ord, PartialOrd)]
pub(crate) enum RulePosition<N, L> {
    Nonterminal(N),
    AdjacentNonterminal(N),
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

pub(crate) struct StatefulLexicalMatch<T, O, S> {
    pub lexical: LexicalMatch<T, O>,
    pub state: S,
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

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum CompletionDisposition<E> {
    Accepted,
    Rejected,
    DeferredBuildRejection(E),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum CompletionDependency {
    WholeForest,
    FamilyReachable,
}

#[derive(Debug, Clone, Copy)]
pub(crate) struct RootRule<R> {
    rule: R,
    completion_dependency: CompletionDependency,
}

impl<R> RootRule<R> {
    pub(crate) const fn family_reachable(rule: R) -> Self {
        Self {
            rule,
            completion_dependency: CompletionDependency::FamilyReachable,
        }
    }

    #[cfg(test)]
    const fn whole_forest(rule: R) -> Self {
        Self {
            rule,
            completion_dependency: CompletionDependency::WholeForest,
        }
    }
}

impl<E> CompletionDisposition<E> {
    pub(crate) const fn is_accepted(&self) -> bool {
        matches!(self, Self::Accepted | Self::DeferredBuildRejection(_))
    }
}

impl From<bool> for CompletionDisposition<()> {
    fn from(accepted: bool) -> Self {
        if accepted { Self::Accepted } else { Self::Rejected }
    }
}

pub(crate) trait CompletionResult {
    type Rejection;

    fn into_disposition(self) -> CompletionDisposition<Self::Rejection>;
}

impl CompletionResult for bool {
    type Rejection = ();

    fn into_disposition(self) -> CompletionDisposition<Self::Rejection> {
        self.into()
    }
}

impl<E> CompletionResult for CompletionDisposition<E> {
    type Rejection = E;

    fn into_disposition(self) -> CompletionDisposition<Self::Rejection> {
        self
    }
}

pub(crate) trait Observation<R, T, L, O = (), E = ()> {
    fn scanned(&mut self, _start: usize, _terminal: L, _end: usize, _value: &T) {}
    fn checked_completion(
        &mut self,
        _rule: R,
        _start: usize,
        _end: usize,
        _family: &Family<T, O>,
        _disposition: &CompletionDisposition<E>,
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

impl<R, T, L, O, E> Observation<R, T, L, O, E> for () {}

#[allow(
    dead_code,
    reason = "the stateless compatibility entry remains available to parser unit consumers"
)]
pub(crate) fn parse<N, L, R, T, O, D, Scan, ValidateCompletion>(
    rules: &[Rule<N, L, R>],
    start: N,
    input_length: usize,
    mut scan: Scan,
    validate_completion: ValidateCompletion,
) -> Result<Forest<R, T, O>, ChartFailure<N, L>>
where
    N: Copy + Eq + Ord + 'static,
    L: Copy + Eq + Ord + std::fmt::Debug + 'static,
    R: Copy + Eq,
    T: Clone + Eq,
    O: Clone + Eq,
    Scan: FnMut(L, usize) -> Vec<LexicalMatch<T, O>>,
    D: CompletionResult,
    ValidateCompletion: FnMut(R, &Family<T, O>, &Forest<R, T, O>) -> D,
{
    parse_with_state(
        rules,
        start,
        input_length,
        &(),
        move |terminal, offset, &(), _suppress_right_boundary| {
            scan(terminal, offset)
                .into_iter()
                .map(|lexical| StatefulLexicalMatch { lexical, state: () })
                .collect()
        },
        validate_completion,
    )
}

pub(crate) fn parse_with_state<N, L, R, T, O, D, S, Scan, ValidateCompletion>(
    rules: &[Rule<N, L, R>],
    start: N,
    input_length: usize,
    initial_state: &S,
    scan: Scan,
    validate_completion: ValidateCompletion,
) -> Result<Forest<R, T, O>, ChartFailure<N, L>>
where
    N: Copy + Eq + Ord + 'static,
    L: Copy + Eq + Ord + std::fmt::Debug + 'static,
    R: Copy + Eq,
    T: Clone + Eq,
    O: Clone + Eq,
    S: Clone + Eq + Ord,
    Scan: FnMut(L, usize, &S, bool) -> Vec<StatefulLexicalMatch<T, O, S>>,
    D: CompletionResult,
    ValidateCompletion: FnMut(R, &Family<T, O>, &Forest<R, T, O>) -> D,
{
    parse_observed_with_state(
        rules,
        start,
        input_length,
        initial_state,
        scan,
        validate_completion,
        &mut (),
    )
}

pub(crate) fn parse_root_with_state<N, L, R, T, O, D, S, Scan, ValidateCompletion>(
    rules: &[Rule<N, L, R>],
    root: RootRule<R>,
    input_length: usize,
    initial_state: &S,
    scan: Scan,
    validate_completion: ValidateCompletion,
) -> Result<Forest<R, T, O>, ChartFailure<N, L>>
where
    N: Copy + Eq + Ord + 'static,
    L: Copy + Eq + Ord + std::fmt::Debug + 'static,
    R: Copy + Eq,
    T: Clone + Eq,
    O: Clone + Eq,
    S: Clone + Eq + Ord,
    Scan: FnMut(L, usize, &S, bool) -> Vec<StatefulLexicalMatch<T, O, S>>,
    D: CompletionResult,
    ValidateCompletion: FnMut(R, &Family<T, O>, &Forest<R, T, O>) -> D,
{
    parse_root_observed_with_state(
        rules,
        root,
        input_length,
        initial_state,
        scan,
        validate_completion,
        &mut (),
    )
}

#[allow(
    dead_code,
    reason = "the stateless observed entry remains available to parser unit consumers"
)]
pub(crate) fn parse_observed<N, L, R, T, Owner, D, Scan, ValidateCompletion, Obs>(
    rules: &[Rule<N, L, R>],
    start: N,
    input_length: usize,
    mut scan: Scan,
    validate_completion: ValidateCompletion,
    observation: &mut Obs,
) -> Result<Forest<R, T, Owner>, ChartFailure<N, L>>
where
    N: Copy + Eq + Ord + 'static,
    L: Copy + Eq + Ord + std::fmt::Debug + 'static,
    R: Copy + Eq,
    T: Clone + Eq,
    Owner: Clone + Eq,
    Scan: FnMut(L, usize) -> Vec<LexicalMatch<T, Owner>>,
    D: CompletionResult,
    ValidateCompletion: FnMut(R, &Family<T, Owner>, &Forest<R, T, Owner>) -> D,
    Obs: Observation<R, T, L, Owner, D::Rejection>,
{
    parse_observed_with_state(
        rules,
        start,
        input_length,
        &(),
        move |terminal, offset, &(), _suppress_right_boundary| {
            scan(terminal, offset)
                .into_iter()
                .map(|lexical| StatefulLexicalMatch { lexical, state: () })
                .collect()
        },
        validate_completion,
        observation,
    )
}

pub(crate) fn parse_observed_with_state<N, L, R, T, Owner, D, S, Scan, ValidateCompletion, Obs>(
    rules: &[Rule<N, L, R>],
    start: N,
    input_length: usize,
    initial_state: &S,
    scan: Scan,
    validate_completion: ValidateCompletion,
    observation: &mut Obs,
) -> Result<Forest<R, T, Owner>, ChartFailure<N, L>>
where
    N: Copy + Eq + Ord + 'static,
    L: Copy + Eq + Ord + std::fmt::Debug + 'static,
    R: Copy + Eq,
    T: Clone + Eq,
    Owner: Clone + Eq,
    S: Clone + Eq + Ord,
    Scan: FnMut(L, usize, &S, bool) -> Vec<StatefulLexicalMatch<T, Owner, S>>,
    D: CompletionResult,
    ValidateCompletion: FnMut(R, &Family<T, Owner>, &Forest<R, T, Owner>) -> D,
    Obs: Observation<R, T, L, Owner, D::Rejection>,
{
    parse_observed_with_state_seed(
        rules,
        RootSeed::Category(start),
        input_length,
        initial_state,
        scan,
        validate_completion,
        observation,
    )
}

pub(crate) fn parse_root_observed_with_state<
    N,
    L,
    R,
    T,
    Owner,
    D,
    S,
    Scan,
    ValidateCompletion,
    Obs,
>(
    rules: &[Rule<N, L, R>],
    root: RootRule<R>,
    input_length: usize,
    initial_state: &S,
    scan: Scan,
    validate_completion: ValidateCompletion,
    observation: &mut Obs,
) -> Result<Forest<R, T, Owner>, ChartFailure<N, L>>
where
    N: Copy + Eq + Ord + 'static,
    L: Copy + Eq + Ord + std::fmt::Debug + 'static,
    R: Copy + Eq,
    T: Clone + Eq,
    Owner: Clone + Eq,
    S: Clone + Eq + Ord,
    Scan: FnMut(L, usize, &S, bool) -> Vec<StatefulLexicalMatch<T, Owner, S>>,
    D: CompletionResult,
    ValidateCompletion: FnMut(R, &Family<T, Owner>, &Forest<R, T, Owner>) -> D,
    Obs: Observation<R, T, L, Owner, D::Rejection>,
{
    parse_observed_with_state_seed(
        rules,
        RootSeed::Rule(root),
        input_length,
        initial_state,
        scan,
        validate_completion,
        observation,
    )
}

#[expect(
    clippy::too_many_lines,
    reason = "the Earley agenda closure remains one explicit state-transition loop"
)]
fn parse_observed_with_state_seed<N, L, R, T, Owner, D, S, Scan, ValidateCompletion, Obs>(
    rules: &[Rule<N, L, R>],
    seed: RootSeed<N, R>,
    input_length: usize,
    initial_state: &S,
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
    S: Clone + Eq + Ord,
    Scan: FnMut(L, usize, &S, bool) -> Vec<StatefulLexicalMatch<T, Owner, S>>,
    D: CompletionResult,
    ValidateCompletion: FnMut(R, &Family<T, Owner>, &Forest<R, T, Owner>) -> D,
    Obs: Observation<R, T, L, Owner, D::Rejection>,
{
    let mut stateful_forest = StatefulForest {
        forest: Forest {
            nodes: Vec::new(),
            accepted_roots: Vec::new(),
        },
        node_states: Vec::new(),
    };

    let mut chart = (0..=input_length)
        .map(|_| ChartColumn::<T, Owner, S>::new())
        .collect::<Vec<_>>();
    let mut agenda = VecDeque::new();
    let mut completed_by_start = (0..=input_length)
        .map(|_| Vec::<(N, bool, NodeId)>::new())
        .collect::<Vec<_>>();

    seed_chart(rules, seed, initial_state, &mut chart, &mut agenda);

    while let Some((column, item, family)) = agenda.pop_front() {
        let rule = &rules[item.rule_index];
        if item.dot == rule.rhs.len() {
            let Some((node_id, extended_existing_node)) = accept_completed_node(
                rule,
                column,
                &item,
                family,
                &mut stateful_forest,
                &mut validate_completion,
                observation,
            ) else {
                continue;
            };
            // Family-reachable validation cannot reference a node before it exists, so
            // only extending an existing packed node can change an earlier result.
            // Whole-forest validation retains the conservative global retry contract.
            if extended_existing_node
                || seed.completion_dependency() == CompletionDependency::WholeForest
            {
                requeue_completed_items_after_forest_growth(&chart, rules, &mut agenda);
            }

            if !register_completed_node(
                &mut stateful_forest,
                &mut completed_by_start,
                completed_rule_is_root(
                    &rule.id,
                    &rule.lhs,
                    seed,
                    item.origin,
                    column,
                    input_length,
                ),
                seed.excludes_from_prediction(&rule.id),
                rule.lhs,
                &item,
                node_id,
            ) {
                continue;
            }

            let waiters = chart[item.origin]
                .iter()
                .filter(|(waiter, _)| {
                    waiter.state == item.origin_state
                        && matches!(
                            rules[waiter.rule_index].rhs.get(waiter.dot),
                            Some(RulePosition::Nonterminal(category) | RulePosition::AdjacentNonterminal(category))
                                if *category == rule.lhs
                                    && expected_right_boundary(rules, waiter)
                                        == item.suppress_right_boundary
                        )
                })
                .map(|(key, families)| (key.clone(), families.clone()))
                .collect::<Vec<_>>();
            for (waiter, waiter_families) in waiters {
                for waiter_family in waiter_families {
                    let mut children = waiter_family.children;
                    children.push(Child::Node(node_id));
                    insert_item(
                        &mut chart,
                        &mut agenda,
                        column,
                        ItemKey {
                            rule_index: waiter.rule_index,
                            dot: waiter.dot + 1,
                            origin: waiter.origin,
                            origin_state: waiter.origin_state.clone(),
                            state: item.state.clone(),
                            suppress_right_boundary: waiter.suppress_right_boundary,
                        },
                        Family { children },
                    );
                }
            }
            continue;
        }

        match rule.rhs[item.dot] {
            RulePosition::Nonterminal(category) | RulePosition::AdjacentNonterminal(category) => {
                let suppress_right_boundary = expected_right_boundary(rules, &item);
                for (predicted_rule, predicted) in rules.iter().enumerate() {
                    if predicted.lhs == category && !seed.excludes_from_prediction(&predicted.id) {
                        insert_item(
                            &mut chart,
                            &mut agenda,
                            column,
                            ItemKey {
                                rule_index: predicted_rule,
                                dot: 0,
                                origin: column,
                                origin_state: item.state.clone(),
                                state: item.state.clone(),
                                suppress_right_boundary,
                            },
                            Family {
                                children: Vec::new(),
                            },
                        );
                    }
                }

                let completed = completed_by_start[column]
                    .iter()
                    .copied()
                    .filter(|(completed_category, completed_boundary, node_id)| {
                        *completed_category == category
                            && *completed_boundary == suppress_right_boundary
                            && stateful_forest.node_states[node_id.0].start == item.state
                    })
                    .collect::<Vec<_>>();
                for (_, _, node_id) in completed {
                    let mut children = family.children.clone();
                    children.push(Child::Node(node_id));
                    let node = &stateful_forest.forest.nodes[node_id.0];
                    insert_item(
                        &mut chart,
                        &mut agenda,
                        node.end,
                        ItemKey {
                            rule_index: item.rule_index,
                            dot: item.dot + 1,
                            origin: item.origin,
                            origin_state: item.origin_state.clone(),
                            state: stateful_forest.node_states[node_id.0].end.clone(),
                            suppress_right_boundary: item.suppress_right_boundary,
                        },
                        Family { children },
                    );
                }
            }
            RulePosition::Lexical(lexical) => {
                advance_lexical(
                    &PendingLexicalScan {
                        lexical,
                        suppress_right_boundary: item.suppress_right_boundary
                            && item.dot + 1 == rule.rhs.len(),
                        column,
                        input_length,
                        item: &item,
                        family: &family,
                    },
                    &mut chart,
                    &mut agenda,
                    &mut scan,
                    observation,
                );
            }
        }
    }

    observe_final_chart(&chart, rules, observation, &stateful_forest.forest);

    if stateful_forest.forest.accepted_roots.is_empty() {
        Err(chart_failure(&chart, rules))
    } else {
        Ok(stateful_forest.forest)
    }
}

fn register_completed_node<N: Copy + Eq, R, T, O, S>(
    stateful_forest: &mut StatefulForest<R, T, O, S>,
    completed_by_start: &mut [Vec<(N, bool, NodeId)>],
    is_root: bool,
    excluded_from_prediction: bool,
    lhs: N,
    item: &ItemKey<S>,
    node_id: NodeId,
) -> bool {
    if is_root && !stateful_forest.forest.accepted_roots.contains(&node_id) {
        stateful_forest.forest.accepted_roots.push(node_id);
    }
    if excluded_from_prediction {
        return false;
    }
    let completion = (lhs, item.suppress_right_boundary, node_id);
    if !completed_by_start[item.origin].contains(&completion) {
        completed_by_start[item.origin].push(completion);
    }
    true
}

fn expected_right_boundary<N, L, R, S>(rules: &[Rule<N, L, R>], item: &ItemKey<S>) -> bool {
    matches!(
        rules[item.rule_index].rhs.get(item.dot),
        Some(RulePosition::AdjacentNonterminal(_))
    ) || (item.suppress_right_boundary && item.dot + 1 == rules[item.rule_index].rhs.len())
}

fn accept_completed_node<N, L, R, T, O, D, S, ValidateCompletion, Obs>(
    rule: &Rule<N, L, R>,
    column: usize,
    item: &ItemKey<S>,
    family: Family<T, O>,
    stateful_forest: &mut StatefulForest<R, T, O, S>,
    validate_completion: &mut ValidateCompletion,
    observation: &mut Obs,
) -> Option<(NodeId, bool)>
where
    R: Copy + Eq,
    T: Clone + Eq,
    O: Clone + Eq,
    S: Clone + Eq,
    D: CompletionResult,
    ValidateCompletion: FnMut(R, &Family<T, O>, &Forest<R, T, O>) -> D,
    Obs: Observation<R, T, L, O, D::Rejection>,
{
    let disposition =
        validate_completion(rule.id, &family, &stateful_forest.forest).into_disposition();
    observation.checked_completion(rule.id, item.origin, column, &family, &disposition);
    if !disposition.is_accepted() {
        return None;
    }
    let (node_id, extended_existing_node) = stateful_forest
        .forest
        .nodes
        .iter()
        .enumerate()
        .position(|(index, node)| {
            node.rule == rule.id
                && node.start == item.origin
                && node.end == column
                && stateful_forest.node_states[index].start == item.origin_state
                && stateful_forest.node_states[index].end == item.state
                && stateful_forest.node_states[index].suppress_right_boundary
                    == item.suppress_right_boundary
        })
        .map_or_else(
            || {
                let node_id = NodeId(stateful_forest.forest.nodes.len());
                stateful_forest.forest.nodes.push(PackedNode {
                    rule: rule.id,
                    start: item.origin,
                    end: column,
                    families: Vec::new(),
                });
                stateful_forest.node_states.push(NodeState {
                    start: item.origin_state.clone(),
                    end: item.state.clone(),
                    suppress_right_boundary: item.suppress_right_boundary,
                });
                (node_id, false)
            },
            |index| (NodeId(index), true),
        );
    insert_family(
        &mut stateful_forest.forest.nodes[node_id.0].families,
        family,
    )
    .then_some((node_id, extended_existing_node))
}

fn observe_final_chart<N, L, R, T, O, E, S, Obs>(
    chart: &[ChartColumn<T, O, S>],
    rules: &[Rule<N, L, R>],
    observation: &mut Obs,
    forest: &Forest<R, T, O>,
) where
    R: Copy,
    Obs: Observation<R, T, L, O, E>,
{
    for (column, items) in chart.iter().enumerate() {
        for (item, families) in items {
            observation.chart_item(
                column,
                rules[item.rule_index].id,
                item.dot,
                item.origin,
                families.len(),
            );
        }
    }
    observation.final_forest(forest);
}

fn advance_lexical<L, R, T, O, E, S, Scan, Obs>(
    pending: &PendingLexicalScan<'_, L, T, O, S>,
    chart: &mut [ChartColumn<T, O, S>],
    agenda: &mut Agenda<T, O, S>,
    scan: &mut Scan,
    observation: &mut Obs,
) where
    L: Copy,
    T: Clone + Eq,
    O: Clone + Eq,
    S: Clone + Eq + Ord,
    Scan: FnMut(L, usize, &S, bool) -> Vec<StatefulLexicalMatch<T, O, S>>,
    Obs: Observation<R, T, L, O, E>,
{
    for stateful_match in scan(
        pending.lexical,
        pending.column,
        &pending.item.state,
        pending.suppress_right_boundary,
    ) {
        let lexical_match = stateful_match.lexical;
        if !(pending.column..=pending.input_length).contains(&lexical_match.end) {
            continue;
        }
        observation.scanned(
            pending.column,
            pending.lexical,
            lexical_match.end,
            &lexical_match.value,
        );
        let mut children = pending.family.children.clone();
        children.push(Child::Lexical(SpannedLexical {
            span: TextSpan {
                start: pending.column,
                end: lexical_match.end,
            },
            value: lexical_match.value,
            owner: lexical_match.owner,
        }));
        insert_item(
            chart,
            agenda,
            lexical_match.end,
            ItemKey {
                rule_index: pending.item.rule_index,
                dot: pending.item.dot + 1,
                origin: pending.item.origin,
                origin_state: pending.item.origin_state.clone(),
                state: stateful_match.state,
                suppress_right_boundary: pending.item.suppress_right_boundary,
            },
            Family { children },
        );
    }
}

#[derive(Clone, Copy)]
enum RootSeed<N, R> {
    Category(N),
    Rule(RootRule<R>),
}

impl<N, R: Eq> RootSeed<N, R> {
    fn excludes_from_prediction(self, rule: &R) -> bool {
        matches!(self, Self::Rule(root) if &root.rule == rule)
    }

    fn completion_dependency(self) -> CompletionDependency {
        match self {
            Self::Category(_) => CompletionDependency::WholeForest,
            Self::Rule(root) => root.completion_dependency,
        }
    }
}

fn seed_chart<N, L, R, T, O, S>(
    rules: &[Rule<N, L, R>],
    seed: RootSeed<N, R>,
    initial_state: &S,
    chart: &mut [ChartColumn<T, O, S>],
    agenda: &mut Agenda<T, O, S>,
) where
    N: Copy + Eq,
    R: Copy + Eq,
    T: Clone + Eq,
    O: Clone + Eq,
    S: Clone + Eq + Ord,
{
    for (rule_index, rule) in rules.iter().enumerate() {
        let selected = match seed {
            RootSeed::Category(start) => rule.lhs == start,
            RootSeed::Rule(root) => rule.id == root.rule,
        };
        if selected {
            insert_item(
                chart,
                agenda,
                0,
                ItemKey {
                    rule_index,
                    dot: 0,
                    origin: 0,
                    origin_state: initial_state.clone(),
                    state: initial_state.clone(),
                    suppress_right_boundary: false,
                },
                Family {
                    children: Vec::new(),
                },
            );
        }
    }
}

fn completed_rule_is_root<N: Eq, R: Eq>(
    rule: &R,
    lhs: &N,
    seed: RootSeed<N, R>,
    origin: usize,
    column: usize,
    input_length: usize,
) -> bool {
    let selected = match seed {
        RootSeed::Category(start) => lhs == &start,
        RootSeed::Rule(root) => rule == &root.rule,
    };
    selected && origin == 0 && column == input_length
}

#[derive(Debug, Clone, PartialEq, Eq, Ord, PartialOrd)]
struct ItemKey<S> {
    rule_index: usize,
    dot: usize,
    origin: usize,
    origin_state: S,
    state: S,
    suppress_right_boundary: bool,
}

struct NodeState<S> {
    start: S,
    end: S,
    suppress_right_boundary: bool,
}

struct StatefulForest<R, T, O, S> {
    forest: Forest<R, T, O>,
    node_states: Vec<NodeState<S>>,
}

struct PendingLexicalScan<'a, L, T, O, S> {
    lexical: L,
    suppress_right_boundary: bool,
    column: usize,
    input_length: usize,
    item: &'a ItemKey<S>,
    family: &'a Family<T, O>,
}

type ChartColumn<T, O, S> = BTreeMap<ItemKey<S>, Vec<Family<T, O>>>;
type Agenda<T, O, S> = VecDeque<(usize, ItemKey<S>, Family<T, O>)>;

fn insert_item<T: Clone + Eq, O: Clone + Eq, S: Clone + Eq + Ord>(
    chart: &mut [ChartColumn<T, O, S>],
    agenda: &mut Agenda<T, O, S>,
    column: usize,
    item: ItemKey<S>,
    family: Family<T, O>,
) {
    let families = chart[column].entry(item.clone()).or_default();
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

fn requeue_completed_items_after_forest_growth<N, L, R, T: Clone, O: Clone, S: Clone + Ord>(
    chart: &[ChartColumn<T, O, S>],
    rules: &[Rule<N, L, R>],
    agenda: &mut Agenda<T, O, S>,
) {
    for (column, items) in chart.iter().enumerate() {
        for (item, families) in items {
            if item.dot != rules[item.rule_index].rhs.len() {
                continue;
            }
            for family in families {
                agenda.push_back((column, item.clone(), family.clone()));
            }
        }
    }
}

fn chart_failure<N, L, R, T, O, S>(
    chart: &[ChartColumn<T, O, S>],
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

fn live_expectations<N, L, R, T, O, S>(
    column: &ChartColumn<T, O, S>,
    rules: &[Rule<N, L, R>],
) -> BTreeSet<RulePosition<N, L>>
where
    N: Clone + Ord,
    L: Clone + Ord,
{
    column
        .iter()
        .filter_map(|(item, families)| {
            (!families.is_empty())
                .then(|| rules[item.rule_index].rhs.get(item.dot))
                .flatten()
                .cloned()
        })
        .collect()
}

engine_unit_tests! {
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
        Value,
        DirectParent,
        Wrapper,
        Child,
        Delay,
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    enum ToyRuleId {
        Start,
        RootAdapter,
        ValueLeaf,
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

    #[test]
    fn deferred_build_rejection_preserves_a_complete_syntactic_root_and_typed_cause() {
        let mut rejections = Vec::new();
        let forest = parse_root_with_state(
            SCAN_RULES,
            RootRule::family_reachable(ToyRuleId::Start),
            10,
            &(),
            |_, start, &(), _suppress_right_boundary| {
                (start == 0)
                    .then_some(vec![StatefulLexicalMatch {
                        lexical: LexicalMatch {
                            end: 10,
                            value: "alpha beta",
                            owner: Some("toy:alpha-beta"),
                        },
                        state: (),
                    }])
                    .unwrap_or_default()
            },
            |_, _, _| {
                rejections.push("Root.guarded");
                CompletionDisposition::DeferredBuildRejection("Root.guarded")
            },
        )
        .expect("a checked-constructor rejection retains the syntactic root");

        assert_eq!(forest.accepted_root_ids().count(), 1);
        assert_eq!(rejections, ["Root.guarded"]);
    }

    const ROOT_BOUNDARY_RULES: &[Rule<ToyCategory, &'static str, ToyRuleId>] = &[
        Rule {
            id: ToyRuleId::RootAdapter,
            lhs: ToyCategory::Value,
            rhs: &[
                RulePosition::Nonterminal(ToyCategory::Value),
                RulePosition::Lexical("!"),
            ],
        },
        Rule {
            id: ToyRuleId::ValueLeaf,
            lhs: ToyCategory::Value,
            rhs: &[RulePosition::Lexical("a")],
        },
    ];

    #[derive(Clone, PartialEq, Eq, Ord, PartialOrd)]
    struct RootBoundaryState;

    fn scan_root_boundary(
        terminal: &'static str,
        start: usize,
        _state: &RootBoundaryState,
        _suppress_right_boundary: bool,
    ) -> Vec<StatefulLexicalMatch<&'static str, (), RootBoundaryState>> {
        match (terminal, start) {
            ("a", 0) => vec![StatefulLexicalMatch {
                lexical: LexicalMatch {
                    end: 1,
                    value: "a",
                    owner: None,
                },
                state: RootBoundaryState,
            }],
            ("!", 1 | 2) => vec![StatefulLexicalMatch {
                lexical: LexicalMatch {
                    end: start + 1,
                    value: "!",
                    owner: None,
                },
                state: RootBoundaryState,
            }],
            _ => Vec::new(),
        }
    }

    #[test]
    fn explicit_root_rule_is_the_unique_outer_seed() {
        let forest = parse_root_with_state(
            ROOT_BOUNDARY_RULES,
            RootRule::whole_forest(ToyRuleId::RootAdapter),
            2,
            &RootBoundaryState,
            scan_root_boundary,
            |_, _, _| true,
        )
        .expect("the explicit root rule consumes one semantic value and one adapter");

        let roots = forest.accepted_roots().collect::<Vec<_>>();
        assert_eq!(roots.len(), 1);
        assert_eq!(roots[0].rule, ToyRuleId::RootAdapter);
        assert!(matches!(
            roots[0].families[0].children.as_slice(),
            [Child::Node(_), Child::Lexical(SpannedLexical { value: "!", .. })]
        ));
    }

    #[test]
    fn explicit_root_rule_is_excluded_from_same_category_prediction() {
        let result =
            parse_root_with_state(
                ROOT_BOUNDARY_RULES,
                RootRule::whole_forest(ToyRuleId::RootAdapter),
                3,
                &RootBoundaryState,
                scan_root_boundary,
                |_, _, _| true,
            );
        assert!(
            result.is_err(),
            "a nested root-only rule must not consume a second adapter: {result:#?}",
        );
    }

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

    const FAMILY_REACHABLE_DELAYED_PACKING_RULES: &[Rule<
        ToyCategory,
        &'static str,
        ToyRuleId,
    >] = &[
        Rule {
            id: ToyRuleId::RootAdapter,
            lhs: ToyCategory::Start,
            rhs: &[RulePosition::Nonterminal(ToyCategory::Wrapper)],
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

    const WHOLE_FOREST_UNRELATED_GROWTH_RULES: &[Rule<
        ToyCategory,
        &'static str,
        ToyRuleId,
    >] = &[
        Rule {
            id: ToyRuleId::RootAdapter,
            lhs: ToyCategory::Start,
            rhs: &[RulePosition::Nonterminal(ToyCategory::Value)],
        },
        Rule {
            id: ToyRuleId::ValueLeaf,
            lhs: ToyCategory::Value,
            rhs: &[RulePosition::Lexical("fast")],
        },
        Rule {
            id: ToyRuleId::DelayedChild,
            lhs: ToyCategory::Value,
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
    fn family_reachable_root_retries_after_reachable_existing_node_gains_a_family() {
        let mut root_checks = Vec::new();
        let forest = parse_root_with_state(
            FAMILY_REACHABLE_DELAYED_PACKING_RULES,
            RootRule::family_reachable(ToyRuleId::RootAdapter),
            1,
            &(),
            |literal, start, &(), _suppress_right_boundary| {
                (start == 0)
                    .then_some(vec![StatefulLexicalMatch {
                        lexical: LexicalMatch {
                            end: 1,
                            value: literal,
                            owner: None::<()>,
                        },
                        state: (),
                    }])
                    .unwrap_or_default()
            },
            |rule, family, forest| {
                let accepted = rule != ToyRuleId::RootAdapter
                    || reaches_delayed_child(forest, family);
                if rule == ToyRuleId::RootAdapter {
                    root_checks.push(accepted);
                }
                accepted
            },
        )
        .expect("reachable packed-family growth must revive the rejected root");

        assert_eq!(root_checks, [false, true]);
        assert_eq!(forest.accepted_roots().count(), 1);
        let wrapper = forest
            .nodes
            .iter()
            .find(|node| node.rule == ToyRuleId::Wrapper)
            .expect("one reachable wrapper node");
        assert_eq!(wrapper.families.len(), 2);
    }

    #[test]
    fn whole_forest_root_retries_after_an_unrelated_new_node_changes_validation() {
        let mut root_checks = Vec::new();
        let forest = parse_root_with_state(
            WHOLE_FOREST_UNRELATED_GROWTH_RULES,
            RootRule::whole_forest(ToyRuleId::RootAdapter),
            1,
            &(),
            |literal, start, &(), _suppress_right_boundary| {
                (start == 0)
                    .then_some(vec![StatefulLexicalMatch {
                        lexical: LexicalMatch {
                            end: 1,
                            value: literal,
                            owner: None::<()>,
                        },
                        state: (),
                    }])
                    .unwrap_or_default()
            },
            |rule, family, forest| {
                if rule != ToyRuleId::RootAdapter {
                    return true;
                }
                let child_rule = family.children.iter().find_map(|child| {
                    let Child::Node(node_id) = child else {
                        return None;
                    };
                    Some(forest.node(*node_id).rule)
                });
                let unrelated_delay_exists = forest
                    .nodes
                    .iter()
                    .any(|node| node.rule == ToyRuleId::Delay);
                let accepted =
                    child_rule == Some(ToyRuleId::ValueLeaf) && unrelated_delay_exists;
                root_checks.push((child_rule, unrelated_delay_exists, accepted));
                accepted
            },
        )
        .expect("unrelated whole-forest growth must revive the rejected root");

        assert!(root_checks.contains(&(Some(ToyRuleId::ValueLeaf), false, false)));
        assert!(root_checks.contains(&(Some(ToyRuleId::ValueLeaf), true, true)));
        let root = forest.accepted_roots().next().expect("one accepted root");
        assert_eq!(forest.accepted_roots().count(), 1);
        let [Child::Node(value_id)] = root.families[0].children.as_slice() else {
            panic!("the accepted root has one value child");
        };
        let value = forest.node(*value_id);
        assert_eq!(value.rule, ToyRuleId::ValueLeaf);
        assert!(matches!(
            value.families[0].children.as_slice(),
            [Child::Lexical(SpannedLexical { value: "fast", .. })]
        ));
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
                disposition: &CompletionDisposition<()>,
            ) {
                self.checked
                    .push((rule, start, end, family.clone(), disposition.is_accepted()));
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
            inventory:
                super::super::diagnostic::SemanticScannerMatchInventory<&'static str, &'static str>,
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
            inventory: super::super::diagnostic::SemanticScannerMatchInventory::default(),
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

    #[test]
    fn scanner_state_is_path_local_across_ambiguous_same_offset_derivations() {
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Ord, PartialOrd)]
        enum ScanState {
            Initial,
            Left,
            Right,
        }

        const STATEFUL_RULES: &[Rule<ToyCategory, &'static str, ToyRuleId>] = &[
            Rule {
                id: ToyRuleId::Start,
                lhs: ToyCategory::Start,
                rhs: &[
                    RulePosition::Nonterminal(ToyCategory::Wrapper),
                    RulePosition::Nonterminal(ToyCategory::Child),
                    RulePosition::Lexical("finish"),
                ],
            },
            Rule {
                id: ToyRuleId::DirectParent,
                lhs: ToyCategory::Wrapper,
                rhs: &[RulePosition::Lexical("left")],
            },
            Rule {
                id: ToyRuleId::Wrapper,
                lhs: ToyCategory::Wrapper,
                rhs: &[RulePosition::Lexical("right")],
            },
            Rule {
                id: ToyRuleId::FastChild,
                lhs: ToyCategory::Child,
                rhs: &[RulePosition::Lexical("shared")],
            },
        ];

        fn lexical_inventories(
            forest: &Forest<ToyRuleId, &'static str>,
            family: &Family<&'static str>,
        ) -> Vec<Vec<&'static str>> {
            family
                .children
                .iter()
                .fold(vec![Vec::new()], |prefixes, child| {
                    let suffixes = match child {
                        Child::Lexical(lexical) => vec![vec![lexical.value]],
                        Child::Node(node_id) => forest
                            .node(*node_id)
                            .families
                            .iter()
                            .flat_map(|family| lexical_inventories(forest, family))
                            .collect(),
                    };
                    prefixes
                        .iter()
                        .flat_map(|prefix| {
                            suffixes.iter().map(|suffix| {
                                let mut inventory = prefix.clone();
                                inventory.extend(suffix);
                                inventory
                            })
                        })
                        .collect()
                })
        }

        let mut scans = Vec::new();
        let forest = parse_with_state(
            STATEFUL_RULES,
            ToyCategory::Start,
            3,
            &ScanState::Initial,
            |terminal, start, state, _suppress_right_boundary| {
                scans.push((terminal, start, *state));
                let (end, next_state, value) = match (terminal, start, state) {
                    ("left", 0, ScanState::Initial) => (1, ScanState::Left, "left"),
                    ("right", 0, ScanState::Initial) => (1, ScanState::Right, "right"),
                    ("shared", 1, ScanState::Left) => (2, ScanState::Left, "shared-left"),
                    ("shared", 1, ScanState::Right) => (2, ScanState::Right, "shared-right"),
                    ("finish", 2, ScanState::Left) => (3, ScanState::Left, "finish-left"),
                    ("finish", 2, ScanState::Right) => (3, ScanState::Right, "finish-right"),
                    _ => return Vec::new(),
                };
                vec![StatefulLexicalMatch {
                    lexical: LexicalMatch {
                        end,
                        value,
                        owner: None::<()>,
                    },
                    state: next_state,
                }]
            },
            |_, _, _| true,
        )
        .expect("both state-compatible paths accept");

        let complete_root_inventory = forest
            .accepted_root_ids()
            .map(|root_id| {
                forest
                    .node(root_id)
                    .families
                    .iter()
                    .flat_map(|family| lexical_inventories(&forest, family))
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();
        assert_eq!(
            complete_root_inventory,
            [
                vec![vec!["left", "shared-left", "finish-left"]],
                vec![vec!["right", "shared-right", "finish-right"]],
            ],
            "each root contains exactly its one state-compatible family",
        );
        assert!(scans.contains(&("shared", 1, ScanState::Left)));
        assert!(scans.contains(&("shared", 1, ScanState::Right)));
        assert!(scans.contains(&("finish", 2, ScanState::Left)));
        assert!(scans.contains(&("finish", 2, ScanState::Right)));
    }
}
}
