use std::collections::HashMap;
use std::collections::HashSet;
use std::sync::Arc;

use super::diagnostic::MaterializationTrace;
use super::diagnostic::MaterializationTraceBuilder;
use super::diagnostic::MaterializedCandidateInfo;
use super::diagnostic::TraceLimits;
use super::engine::Child;
use super::engine::Family;
use super::engine::Forest;
use super::engine::NodeId;
use super::engine::Rule;
use super::engine::RulePosition;
use super::engine::SpannedLexical;
use super::ownership::RawLexicalClaim;
use super::selection::specificity_tiers;
use crate::ast::Ability;
use crate::constructions::BuildRejection;
use crate::constructions::BuildValue;
use crate::constructions::Category;
use crate::constructions::Construction;
use crate::constructions::GeneratedParseRoot;
use crate::constructions::Leaf;
use crate::constructions::Lexical;
use crate::constructions::LexicalOwner;
use crate::constructions::LexicalTerminal;
#[cfg(test)]
use crate::constructions::RULES;
use crate::constructions::RuleId;
use crate::constructions::build_checked;
use crate::context::ParseContext;
use crate::parser::scan::RootForest;
use crate::parser::scan::RootRuleId;
use crate::parser::scan::rules_for_root;

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) struct MaterializedCandidate<V, C, K = Category, M = Lexical, T = (), O = ()> {
    pub(super) value: Arc<V>,
    pub(super) constructions: Arc<[C]>,
    pub(super) positions: Arc<[RulePosition<K, M>]>,
    pub(super) leaves: Arc<[M]>,
    pub(super) claims: Arc<[SpannedLexical<T, O>]>,
}

type MaterializedValues<V, C, K, M, T, O> = Arc<[MaterializedCandidate<V, C, K, M, T, O>]>;
type MaterializedMemo<V, C, K, M, T, O> = HashMap<NodeId, MaterializedValues<V, C, K, M, T, O>>;

#[cfg(test)]
thread_local! {
    static MATERIALIZED_CANDIDATE_COPIES: std::cell::Cell<usize> = const { std::cell::Cell::new(1) };
    static SPECIFICITY_CANDIDATE_EVALUATIONS: std::cell::Cell<usize> = const { std::cell::Cell::new(0) };
    static FINALIZED_CANDIDATE_COUNT: std::cell::Cell<usize> = const { std::cell::Cell::new(0) };
}

#[cfg(test)]
pub(super) fn set_materialized_candidate_copies(copies: usize) {
    MATERIALIZED_CANDIDATE_COPIES.with(|value| value.set(copies));
}

#[cfg(test)]
pub(super) fn reset_specificity_candidate_evaluations() {
    SPECIFICITY_CANDIDATE_EVALUATIONS.with(|value| value.set(0));
    FINALIZED_CANDIDATE_COUNT.with(|value| value.set(0));
}

#[cfg(test)]
pub(super) fn specificity_candidate_evaluations() -> usize {
    SPECIFICITY_CANDIDATE_EVALUATIONS.with(std::cell::Cell::get)
}

#[cfg(test)]
pub(super) fn finalized_candidate_count() -> usize {
    FINALIZED_CANDIDATE_COUNT.with(std::cell::Cell::get)
}

#[cfg(test)]
fn count_specificity_candidate() {
    SPECIFICITY_CANDIDATE_EVALUATIONS.with(|value| value.set(value.get() + 1));
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct Candidate<V = Ability> {
    pub value: V,
    pub constructions: Vec<Construction>,
    pub positions: Vec<RulePosition<Category, Lexical>>,
    pub specificity: Vec<super::SpecificityTier>,
    pub leaf_path: Vec<String>,
    pub claims: Vec<RawLexicalClaim>,
    pub synthetic_claims: Vec<crate::parser::TextSpan>,
}

pub(crate) struct MaterializationResult<R: GeneratedParseRoot> {
    pub(crate) candidates: Vec<Candidate<R>>,
    pub(crate) first_rejection: Option<BuildRejection>,
}

trait MaterializationObservation<R, E = BuildRejection> {
    const ENABLED: bool;
    fn cycle_pruned(&mut self, _node_id: NodeId, _rule_path: &[R]) {}
    fn build_rejected(&mut self, _rule_path: &[R], _rejection: &E) {}
}

impl<R, E> MaterializationObservation<R, E> for () {
    const ENABLED: bool = false;
}

impl MaterializationObservation<RuleId> for MaterializationTraceBuilder {
    const ENABLED: bool = true;

    fn cycle_pruned(&mut self, node_id: NodeId, rule_path: &[RuleId]) {
        self.record_cycle(node_id.0, rule_path);
    }

    fn build_rejected(&mut self, rule_path: &[RuleId], rejection: &BuildRejection) {
        self.record_build_rejection(rule_path, *rejection);
    }
}

impl MaterializationObservation<RootRuleId> for MaterializationTraceBuilder {
    const ENABLED: bool = true;

    fn cycle_pruned(&mut self, node_id: NodeId, rule_path: &[RootRuleId]) {
        self.record_cycle_with(
            node_id.0,
            rule_path,
            |rule| rule.index(),
            |rule| match rule {
                RootRuleId::Grammar(rule) => format!("{rule:?}"),
                RootRuleId::Adapter => "RootAdapter".to_owned(),
            },
        );
    }

    fn build_rejected(&mut self, rule_path: &[RootRuleId], rejection: &BuildRejection) {
        self.record_build_rejection_with(
            rule_path,
            *rejection,
            |rule| rule.index(),
            |rule| match rule {
                RootRuleId::Grammar(rule) => format!("{rule:?}"),
                RootRuleId::Adapter => "RootAdapter".to_owned(),
            },
        );
    }
}

struct MaterializationStateFor<V, C, K = Category, M = Lexical, T = (), O = ()> {
    memo: MaterializedMemo<V, C, K, M, T, O>,
    in_progress: HashSet<NodeId>,
}

impl<V, C, K, M, T, O> Default for MaterializationStateFor<V, C, K, M, T, O> {
    fn default() -> Self {
        Self {
            memo: HashMap::new(),
            in_progress: HashSet::new(),
        }
    }
}

struct MaterializationOutcomeFor<
    V,
    C,
    K = Category,
    M = Lexical,
    T = (),
    O = (),
    E = BuildRejection,
> {
    values: MaterializedValues<V, C, K, M, T, O>,
    cycle_pruned: bool,
    first_rejection: Option<E>,
}

struct MaterializationResultFor<V, C, K = Category, M = Lexical, T = (), O = (), E = BuildRejection>
{
    values: Vec<MaterializedCandidate<V, C, K, M, T, O>>,
    first_rejection: Option<E>,
}

struct MaterializationKernel<'a, R, T, V, C, K: 'static, L: 'static, M, Build, E = BuildRejection> {
    rules: &'a [Rule<K, L, R>],
    rule_index: fn(R) -> usize,
    public_construction: fn(R) -> Option<C>,
    lexical_matcher: fn(L) -> M,
    build_leaf: fn(&T) -> V,
    build: Build,
    rejection: std::marker::PhantomData<fn() -> E>,
}

impl<R, T, V, C, K, L, M, Build, E> MaterializationKernel<'_, R, T, V, C, K, L, M, Build, E>
where
    R: Copy,
    V: Clone + PartialEq,
    C: Copy + PartialEq,
    K: Copy + PartialEq + 'static,
    L: Copy + 'static,
    M: Clone + PartialEq,
    T: Clone + PartialEq,
    E: Copy,
    Build: Fn(R, &[V]) -> Result<Option<V>, E>,
{
    fn materialize<Owner, O>(
        &self,
        forest: &Forest<R, T, Owner>,
        observation: &mut O,
    ) -> MaterializationResultFor<V, C, K, M, T, Owner, E>
    where
        O: MaterializationObservation<R, E>,
        Owner: Clone + PartialEq,
    {
        let mut candidates = Vec::new();
        let mut state = MaterializationStateFor::default();
        let mut rule_path = Vec::new();
        let mut first_rejection = None;
        for root in forest.accepted_root_ids() {
            let outcome =
                self.materialize_node(forest, root, &mut state, &mut rule_path, observation);
            if first_rejection.is_none() {
                first_rejection = outcome.first_rejection;
            }
            for built in outcome.values.iter().cloned() {
                push_unique(&mut candidates, built);
            }
        }
        MaterializationResultFor {
            first_rejection: candidates.is_empty().then_some(first_rejection).flatten(),
            values: candidates,
        }
    }

    fn materialize_node<Owner, O>(
        &self,
        forest: &Forest<R, T, Owner>,
        node_id: NodeId,
        state: &mut MaterializationStateFor<V, C, K, M, T, Owner>,
        rule_path: &mut Vec<R>,
        observation: &mut O,
    ) -> MaterializationOutcomeFor<V, C, K, M, T, Owner, E>
    where
        O: MaterializationObservation<R, E>,
        Owner: Clone + PartialEq,
    {
        let node = forest.node(node_id);
        if let Some(values) = state.memo.get(&node_id) {
            #[cfg(feature = "parser-metrics")]
            super::metrics::record(
                (self.rule_index)(node.rule),
                super::metrics::MetricEvent::CloneHeavy,
            );
            return MaterializationOutcomeFor {
                values: Arc::clone(values),
                cycle_pruned: false,
                first_rejection: None,
            };
        }
        #[cfg(feature = "parser-metrics")]
        super::metrics::record(
            (self.rule_index)(node.rule),
            super::metrics::MetricEvent::MemoMiss,
        );
        if !state.in_progress.insert(node_id) {
            observation.cycle_pruned(node_id, rule_path);
            return MaterializationOutcomeFor {
                values: Arc::from([]),
                cycle_pruned: true,
                first_rejection: None,
            };
        }

        let mut values = Vec::new();
        let mut cycle_pruned = false;
        let mut first_rejection = None;
        for family in &node.families {
            let outcome =
                self.materialize_family(forest, node.rule, family, state, rule_path, observation);
            cycle_pruned |= outcome.cycle_pruned;
            if first_rejection.is_none() {
                first_rejection = outcome.first_rejection;
            }
            for built in outcome.values.iter().cloned() {
                push_unique(&mut values, built);
            }
        }
        state.in_progress.remove(&node_id);
        if !cycle_pruned && first_rejection.is_none() {
            let values = Arc::from(values);
            state.memo.insert(node_id, Arc::clone(&values));
            let first_rejection = values.is_empty().then_some(first_rejection).flatten();
            return MaterializationOutcomeFor {
                values,
                cycle_pruned,
                first_rejection,
            };
        }
        let first_rejection = values.is_empty().then_some(first_rejection).flatten();
        MaterializationOutcomeFor {
            values: Arc::from(values),
            cycle_pruned,
            first_rejection,
        }
    }

    fn materialize_family<Owner, O>(
        &self,
        forest: &Forest<R, T, Owner>,
        rule_id: R,
        family: &Family<T, Owner>,
        state: &mut MaterializationStateFor<V, C, K, M, T, Owner>,
        rule_path: &mut Vec<R>,
        observation: &mut O,
    ) -> MaterializationOutcomeFor<V, C, K, M, T, Owner, E>
    where
        O: MaterializationObservation<R, E>,
        Owner: Clone + PartialEq,
    {
        #[cfg(feature = "parser-metrics")]
        super::metrics::record(
            (self.rule_index)(rule_id),
            super::metrics::MetricEvent::Materialization,
        );
        let rule = &self.rules[(self.rule_index)(rule_id)];
        let public_construction = (self.public_construction)(rule_id);
        if O::ENABLED {
            rule_path.push(rule_id);
        }
        let mut combinations = vec![Vec::new()];
        let mut cycle_pruned = false;
        let mut first_rejection = None;
        for (index, child) in family.children.iter().enumerate() {
            let child_values = match child {
                Child::Node(id) => {
                    let outcome = self.materialize_node(forest, *id, state, rule_path, observation);
                    cycle_pruned |= outcome.cycle_pruned;
                    if first_rejection.is_none() {
                        first_rejection = outcome.first_rejection;
                    }
                    outcome.values
                }
                Child::Lexical(lexical) => {
                    let position = rule
                        .rhs
                        .get(index)
                        .expect("lexical family child retains its rule position");
                    let RulePosition::Lexical(matcher) = position else {
                        unreachable!("lexical family child retains its lexical rule position")
                    };
                    Arc::from([MaterializedCandidate {
                        value: Arc::new((self.build_leaf)(&lexical.value)),
                        constructions: Arc::from([]),
                        positions: Arc::from([]),
                        leaves: Arc::from([(self.lexical_matcher)(*matcher)]),
                        claims: Arc::from([lexical.clone()]),
                    }])
                }
            };
            let mut next = Vec::new();
            for combination in combinations {
                for child_value in child_values.iter() {
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
                .map(|child| child.value.as_ref().clone())
                .collect::<Vec<_>>();
            match (self.build)(rule_id, &child_values) {
                Ok(Some(value)) => {
                    let mut constructions = public_construction.into_iter().collect::<Vec<_>>();
                    let mut positions = if public_construction.is_some() {
                        rule.rhs
                            .iter()
                            .map(|position| match *position {
                                RulePosition::Nonterminal(category) => {
                                    RulePosition::Nonterminal(category)
                                }
                                RulePosition::AdjacentNonterminal(category) => {
                                    RulePosition::AdjacentNonterminal(category)
                                }
                                RulePosition::Lexical(lexical) => {
                                    RulePosition::Lexical((self.lexical_matcher)(lexical))
                                }
                            })
                            .collect::<Vec<_>>()
                    } else {
                        Vec::new()
                    };
                    let mut claims = Vec::new();
                    let mut leaves = Vec::new();
                    for child in children {
                        constructions.extend(child.constructions.iter().copied());
                        positions.extend(child.positions.iter().cloned());
                        leaves.extend(child.leaves.iter().cloned());
                        claims.extend(child.claims.iter().cloned());
                    }
                    claims.sort_by_key(|claim| (claim.span.start, claim.span.end));
                    push_unique(
                        &mut values,
                        MaterializedCandidate {
                            value: Arc::new(value),
                            constructions: Arc::from(constructions),
                            positions: Arc::from(positions),
                            leaves: Arc::from(leaves),
                            claims: Arc::from(claims),
                        },
                    );
                }
                Ok(None) => {}
                Err(rejection) => {
                    if first_rejection.is_none() {
                        first_rejection = Some(rejection);
                    }
                    observation.build_rejected(rule_path, &rejection);
                }
            }
        }
        if O::ENABLED {
            rule_path.pop();
        }
        let first_rejection = values.is_empty().then_some(first_rejection).flatten();
        MaterializationOutcomeFor {
            values: Arc::from(values),
            cycle_pruned,
            first_rejection,
        }
    }
}

#[cfg(test)]
pub(super) fn materialize_with<R, T, O, V, C, K, L, M, Build>(
    forest: &Forest<R, T, O>,
    rules: &[Rule<K, L, R>],
    rule_index: fn(R) -> usize,
    public_construction: fn(R) -> Option<C>,
    lexical_matcher: fn(L) -> M,
    build_leaf: fn(&T) -> V,
    build: Build,
) -> Vec<MaterializedCandidate<V, C, K, M, T, O>>
where
    R: Copy,
    V: Clone + PartialEq,
    C: Copy + PartialEq,
    K: Copy + PartialEq + 'static,
    L: Copy + 'static,
    M: Clone + PartialEq,
    T: Clone + PartialEq,
    O: Clone + PartialEq,
    Build: Fn(R, &[V]) -> Result<Option<V>, BuildRejection>,
{
    MaterializationKernel {
        rules,
        rule_index,
        public_construction,
        lexical_matcher,
        build_leaf,
        build,
        rejection: std::marker::PhantomData,
    }
    .materialize(forest, &mut ())
    .values
}

#[cfg(test)]
type MaterializedRejectionOutput<V, C, K, M, T, O, E> =
    (Vec<MaterializedCandidate<V, C, K, M, T, O>>, Option<E>);

#[cfg(test)]
pub(super) fn materialize_with_rejection<R, T, O, V, C, K, L, M, Build, E>(
    forest: &Forest<R, T, O>,
    rules: &[Rule<K, L, R>],
    rule_index: fn(R) -> usize,
    public_construction: fn(R) -> Option<C>,
    lexical_matcher: fn(L) -> M,
    build_leaf: fn(&T) -> V,
    build: Build,
) -> MaterializedRejectionOutput<V, C, K, M, T, O, E>
where
    R: Copy,
    V: Clone + PartialEq,
    C: Copy + PartialEq,
    K: Copy + PartialEq + 'static,
    L: Copy + 'static,
    M: Clone + PartialEq,
    T: Clone + PartialEq,
    O: Clone + PartialEq,
    E: Copy,
    Build: Fn(R, &[V]) -> Result<Option<V>, E>,
{
    let result = MaterializationKernel {
        rules,
        rule_index,
        public_construction,
        lexical_matcher,
        build_leaf,
        build,
        rejection: std::marker::PhantomData,
    }
    .materialize(forest, &mut ());
    (result.values, result.first_rejection)
}

#[cfg(test)]
struct ProjectedMaterializationTrace<'a, R> {
    builder: &'a mut MaterializationTraceBuilder,
    identity: fn(R) -> usize,
    label: fn(R) -> String,
}

#[cfg(test)]
impl<R: Copy> MaterializationObservation<R> for ProjectedMaterializationTrace<'_, R> {
    const ENABLED: bool = true;

    fn cycle_pruned(&mut self, node_id: NodeId, rule_path: &[R]) {
        self.builder.record_cycle_with(
            node_id.0,
            rule_path,
            |rule| (self.identity)(*rule),
            |rule| (self.label)(*rule),
        );
    }

    fn build_rejected(&mut self, rule_path: &[R], rejection: &BuildRejection) {
        self.builder.record_build_rejection_with(
            rule_path,
            *rejection,
            |rule| (self.identity)(*rule),
            |rule| (self.label)(*rule),
        );
    }
}

#[cfg(test)]
type MaterializedTraceOutput<V, C, K, M, T, O> = (
    Vec<MaterializedCandidate<V, C, K, M, T, O>>,
    MaterializationTrace,
);

#[cfg(test)]
#[allow(
    clippy::too_many_arguments,
    reason = "the generic test seam mirrors every production materialization authority"
)]
pub(super) fn materialize_with_trace<R, T, O, V, C, K, L, M, Build>(
    forest: &Forest<R, T, O>,
    rules: &[Rule<K, L, R>],
    rule_index: fn(R) -> usize,
    public_construction: fn(R) -> Option<C>,
    lexical_matcher: fn(L) -> M,
    build_leaf: fn(&T) -> V,
    build: Build,
    label: fn(R) -> String,
    limits: TraceLimits,
) -> MaterializedTraceOutput<V, C, K, M, T, O>
where
    R: Copy,
    V: Clone + PartialEq,
    C: Copy + PartialEq,
    K: Copy + PartialEq + 'static,
    L: Copy + 'static,
    M: Clone + PartialEq,
    T: Clone + PartialEq,
    O: Clone + PartialEq,
    Build: Fn(R, &[V]) -> Result<Option<V>, BuildRejection>,
{
    let mut builder = MaterializationTraceBuilder::new(limits);
    let outcome = MaterializationKernel {
        rules,
        rule_index,
        public_construction,
        lexical_matcher,
        build_leaf,
        build,
        rejection: std::marker::PhantomData,
    }
    .materialize(
        forest,
        &mut ProjectedMaterializationTrace {
            builder: &mut builder,
            identity: rule_index,
            label,
        },
    );
    builder.project_terminal_build_rejection(outcome.first_rejection);
    (outcome.values, builder.finish())
}

pub(crate) fn materialize_checked<R: GeneratedParseRoot>(
    forest: &RootForest,
    context: &ParseContext<'_>,
    environment: &crate::environment::ParserEnvironment,
) -> MaterializationResult<R> {
    #[cfg(test)]
    super::count_pipeline_stage(super::PipelineStage::Materialize);
    let rules = rules_for_root::<R>();
    let built = MaterializationKernel {
        rules: &rules,
        rule_index: RootRuleId::index,
        public_construction: RootRuleId::public_construction,
        lexical_matcher: |terminal: LexicalTerminal| terminal.matcher,
        build_leaf: |leaf: &Leaf| BuildValue::Leaf(leaf.clone()),
        build: |rule, children: &[BuildValue]| build_root_rule(rule, children, context),
        rejection: std::marker::PhantomData,
    }
    .materialize(forest.forest(), &mut ());
    let candidates = finalize_candidates::<R>(built.values, context, environment, None);
    MaterializationResult {
        first_rejection: candidates
            .is_empty()
            .then_some(built.first_rejection)
            .flatten(),
        candidates,
    }
}

pub(crate) fn materialize_observed_checked<R: GeneratedParseRoot>(
    forest: &RootForest,
    context: &ParseContext<'_>,
    environment: &crate::environment::ParserEnvironment,
    limits: TraceLimits,
) -> (MaterializationResult<R>, MaterializationTrace) {
    #[cfg(test)]
    super::count_pipeline_stage(super::PipelineStage::Materialize);
    let mut observation = MaterializationTraceBuilder::new(limits);
    let rules = rules_for_root::<R>();
    let built = MaterializationKernel {
        rules: &rules,
        rule_index: RootRuleId::index,
        public_construction: RootRuleId::public_construction,
        lexical_matcher: |terminal: LexicalTerminal| terminal.matcher,
        build_leaf: |leaf: &Leaf| BuildValue::Leaf(leaf.clone()),
        build: |rule, children: &[BuildValue]| build_root_rule(rule, children, context),
        rejection: std::marker::PhantomData,
    }
    .materialize(forest.forest(), &mut observation);
    let candidates = finalize_candidates::<R>(
        built.values,
        context,
        environment,
        Some((&mut observation, limits)),
    );
    let first_rejection = candidates
        .is_empty()
        .then_some(built.first_rejection)
        .flatten();
    observation.project_terminal_build_rejection(first_rejection);
    (
        MaterializationResult {
            candidates,
            first_rejection,
        },
        observation.finish(),
    )
}

#[cfg(test)]
fn materialize<R: GeneratedParseRoot>(
    forest: &RootForest,
    context: &ParseContext<'_>,
    environment: &crate::environment::ParserEnvironment,
) -> Vec<Candidate<R>> {
    materialize_checked(forest, context, environment).candidates
}

#[cfg(test)]
fn materialize_observed<R: GeneratedParseRoot>(
    forest: &RootForest,
    context: &ParseContext<'_>,
    environment: &crate::environment::ParserEnvironment,
    limits: TraceLimits,
) -> (Vec<Candidate<R>>, MaterializationTrace) {
    let (result, trace) = materialize_observed_checked(forest, context, environment, limits);
    (result.candidates, trace)
}

fn finalize_candidates<R: GeneratedParseRoot>(
    built_values: Vec<
        MaterializedCandidate<BuildValue, Construction, Category, Lexical, Leaf, LexicalOwner>,
    >,
    context: &ParseContext<'_>,
    environment: &crate::environment::ParserEnvironment,
    mut observation: Option<(&mut MaterializationTraceBuilder, TraceLimits)>,
) -> Vec<Candidate<R>> {
    #[cfg(test)]
    let built_values = MATERIALIZED_CANDIDATE_COPIES.with(|copies| {
        let copies = copies.get().max(1);
        let Some(first) = built_values.first().cloned() else {
            return built_values;
        };
        if copies == 1 {
            return built_values;
        }
        let nonterminal = first
            .positions
            .iter()
            .find(|position| {
                matches!(
                    position,
                    RulePosition::Nonterminal(_) | RulePosition::AdjacentNonterminal(_)
                )
            })
            .copied()
            .expect("a complete ability candidate traverses a nonterminal");
        let literal_positions = first
            .positions
            .iter()
            .enumerate()
            .filter_map(|(index, position)| {
                matches!(position, RulePosition::Lexical(Lexical::Literal(_))).then_some(index)
            })
            .collect::<Vec<_>>();
        assert!(
            copies - 1 <= literal_positions.len(),
            "test candidate copies require distinct specificity downgrades"
        );
        let mut varied = vec![first.clone()];
        for position_index in literal_positions.into_iter().take(copies - 1) {
            let mut candidate = first.clone();
            Arc::make_mut(&mut candidate.positions)[position_index] = nonterminal;
            varied.push(candidate);
        }
        varied
    });
    #[cfg(test)]
    super::count_pipeline_stage(super::PipelineStage::Specificity);
    let mut candidates = Vec::new();
    for built in built_values {
        if let Some(value) = R::from_build(Arc::unwrap_or_clone(built.value)) {
            let mut claims = Vec::new();
            let mut synthetic_claims = Vec::new();
            for claim in built.claims.iter() {
                match &claim.owner {
                    Some(owner) => claims.push(RawLexicalClaim {
                        span: claim.span,
                        value: claim.value.clone(),
                        owner: owner.clone(),
                    }),
                    None if !matches!(claim.value, Leaf::EndOfInput) => {
                        synthetic_claims.push(claim.span);
                    }
                    None => {}
                }
            }
            #[cfg(test)]
            count_specificity_candidate();
            let candidate = Candidate {
                value,
                constructions: built.constructions.as_ref().to_vec(),
                specificity: specificity_tiers(&built.positions),
                positions: built.positions.as_ref().to_vec(),
                leaf_path: built
                    .leaves
                    .iter()
                    .filter_map(|leaf| match leaf {
                        Lexical::Literal(_) | Lexical::EndOfInput => None,
                        lexical => Some(lexical.class().label().to_owned()),
                    })
                    .collect(),
                claims,
                synthetic_claims,
            };
            let ordinal = candidates.len();
            if push_unique(&mut candidates, candidate)
                && let Some((trace, limits)) = observation.as_mut()
            {
                let candidate = candidates.last().expect("just inserted candidate");
                trace.record_candidate_with(|| {
                    MaterializedCandidateInfo::new(
                        ordinal,
                        R::render_with_claims(&candidate.value, context, environment).0,
                        format!("{:?}", candidate.value),
                        &candidate.constructions,
                        &candidate.specificity,
                        limits.per_collection(),
                    )
                });
            }
        }
    }
    #[cfg(test)]
    FINALIZED_CANDIDATE_COUNT.with(|count| count.set(candidates.len()));
    candidates
}

fn build_root_rule(
    rule: RootRuleId,
    children: &[BuildValue],
    context: &ParseContext<'_>,
) -> Result<Option<BuildValue>, BuildRejection> {
    match rule {
        RootRuleId::Grammar(rule) => build_checked(rule, children, context),
        RootRuleId::Adapter => {
            let Some((value, adapter)) = children.split_first() else {
                return Ok(None);
            };
            Ok(adapter
                .iter()
                .all(|value| matches!(value, BuildValue::Leaf(_)))
                .then(|| value.clone()))
        }
    }
}

#[cfg(test)]
fn materialize_node(
    forest: &Forest<RuleId, Leaf>,
    node_id: NodeId,
    context: &ParseContext<'_>,
    state: &mut MaterializationStateFor<BuildValue, Construction, Category, Lexical, Leaf>,
) -> MaterializationOutcomeFor<BuildValue, Construction, Category, Lexical, Leaf> {
    let kernel: MaterializationKernel<
        '_,
        RuleId,
        Leaf,
        BuildValue,
        Construction,
        Category,
        LexicalTerminal,
        Lexical,
        _,
    > = MaterializationKernel {
        rules: RULES,
        rule_index: RuleId::index,
        public_construction: RuleId::public_construction,
        lexical_matcher: |terminal| terminal.matcher,
        build_leaf: |leaf: &Leaf| BuildValue::Leaf(leaf.clone()),
        build: |rule: RuleId, children: &[BuildValue]| build_checked(rule, children, context),
        rejection: std::marker::PhantomData,
    };
    kernel.materialize_node(forest, node_id, state, &mut Vec::new(), &mut ())
}

#[derive(Default)]
struct CheckedMaterializationState {
    memo: HashMap<NodeId, Arc<[Arc<BuildValue>]>>,
    in_progress: HashSet<NodeId>,
    parents_by_child: HashMap<NodeId, HashSet<NodeId>>,
}

impl CheckedMaterializationState {
    fn invalidate(&mut self, node_id: NodeId) {
        let mut pending = vec![node_id];
        let mut invalidated = HashSet::new();
        while let Some(node_id) = pending.pop() {
            if !invalidated.insert(node_id) {
                continue;
            }
            self.memo.remove(&node_id);
            if let Some(parents) = self.parents_by_child.remove(&node_id) {
                pending.extend(parents);
            }
        }
    }
}

struct CheckedMaterializationOutcome {
    values: Arc<[Arc<BuildValue>]>,
    cycle_pruned: bool,
    first_rejection: Option<BuildRejection>,
}

#[derive(Default)]
pub(super) struct CheckedCompletionState {
    materialization: CheckedMaterializationState,
    observed_extensions: usize,
}

pub(super) fn completion_has_checked_build(
    rules: &[Rule<Category, LexicalTerminal, RootRuleId>],
    rule: RootRuleId,
    family: &Family<Leaf, LexicalOwner>,
    forest: &Forest<RootRuleId, Leaf, LexicalOwner>,
    context: &ParseContext<'_>,
    state: &mut CheckedCompletionState,
) -> super::engine::CompletionDisposition<BuildRejection> {
    let extensions = forest.extended_nodes_since(state.observed_extensions);
    state.observed_extensions += extensions.len();
    for &node_id in extensions {
        state.materialization.invalidate(node_id);
    }
    let outcome = checked_materialize_family(
        rules,
        forest,
        rule,
        family,
        context,
        &mut state.materialization,
    );
    if !outcome.values.is_empty() {
        super::engine::CompletionDisposition::Accepted
    } else if let Some(rejection) = outcome.first_rejection {
        super::engine::CompletionDisposition::DeferredBuildRejection(rejection)
    } else {
        super::engine::CompletionDisposition::Rejected
    }
}

fn checked_materialize_node(
    rules: &[Rule<Category, LexicalTerminal, RootRuleId>],
    forest: &Forest<RootRuleId, Leaf, LexicalOwner>,
    node_id: NodeId,
    context: &ParseContext<'_>,
    state: &mut CheckedMaterializationState,
) -> CheckedMaterializationOutcome {
    let node = forest.node(node_id);
    if let Some(values) = state.memo.get(&node_id) {
        #[cfg(feature = "parser-metrics")]
        super::metrics::record(node.rule.index(), super::metrics::MetricEvent::CloneHeavy);
        return CheckedMaterializationOutcome {
            values: Arc::clone(values),
            cycle_pruned: false,
            first_rejection: None,
        };
    }
    #[cfg(feature = "parser-metrics")]
    super::metrics::record(node.rule.index(), super::metrics::MetricEvent::MemoMiss);
    if !state.in_progress.insert(node_id) {
        return CheckedMaterializationOutcome {
            values: Arc::from([]),
            cycle_pruned: true,
            first_rejection: None,
        };
    }

    for family in &node.families {
        for child in &family.children {
            if let Child::Node(child_id) = child {
                state
                    .parents_by_child
                    .entry(*child_id)
                    .or_default()
                    .insert(node_id);
            }
        }
    }
    let mut values = Vec::new();
    let mut cycle_pruned = false;
    let mut first_rejection = None;
    for family in &node.families {
        let outcome = checked_materialize_family(rules, forest, node.rule, family, context, state);
        cycle_pruned |= outcome.cycle_pruned;
        if first_rejection.is_none() {
            first_rejection = outcome.first_rejection;
        }
        for value in outcome.values.iter().cloned() {
            push_unique(&mut values, value);
        }
    }
    state.in_progress.remove(&node_id);
    let first_rejection = values.is_empty().then_some(first_rejection).flatten();
    let values = Arc::from(values);
    if !cycle_pruned && first_rejection.is_none() {
        state.memo.insert(node_id, Arc::clone(&values));
    }
    CheckedMaterializationOutcome {
        values,
        cycle_pruned,
        first_rejection,
    }
}

fn checked_materialize_family(
    rules: &[Rule<Category, LexicalTerminal, RootRuleId>],
    forest: &Forest<RootRuleId, Leaf, LexicalOwner>,
    rule_id: RootRuleId,
    family: &Family<Leaf, LexicalOwner>,
    context: &ParseContext<'_>,
    state: &mut CheckedMaterializationState,
) -> CheckedMaterializationOutcome {
    #[cfg(feature = "parser-metrics")]
    super::metrics::record(
        rule_id.index(),
        super::metrics::MetricEvent::Materialization,
    );
    let mut combinations = vec![Vec::new()];
    let mut cycle_pruned = false;
    let mut first_rejection = None;
    for child in &family.children {
        let child_values = match child {
            Child::Node(id) => {
                let outcome = checked_materialize_node(rules, forest, *id, context, state);
                cycle_pruned |= outcome.cycle_pruned;
                if first_rejection.is_none() {
                    first_rejection = outcome.first_rejection;
                }
                outcome.values
            }
            Child::Lexical(lexical) => {
                Arc::from([Arc::new(BuildValue::Leaf(lexical.value.clone()))])
            }
        };
        let mut next = Vec::new();
        for combination in combinations {
            for child_value in child_values.iter() {
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
            .map(|child| child.as_ref().clone())
            .collect::<Vec<_>>();
        match build_root_rule(rule_id, &child_values, context) {
            Ok(Some(value)) => {
                push_unique(&mut values, Arc::new(value));
            }
            Err(rejection) if first_rejection.is_none() => {
                first_rejection = Some(rejection);
            }
            Ok(None) | Err(_) => {}
        }
    }
    let first_rejection = values.is_empty().then_some(first_rejection).flatten();
    CheckedMaterializationOutcome {
        values: Arc::from(values),
        cycle_pruned,
        first_rejection,
    }
}

fn push_unique<T: PartialEq>(values: &mut Vec<T>, value: T) -> bool {
    if values.contains(&value) {
        false
    } else {
        values.push(value);
        true
    }
}

#[cfg(test)]
mod tests {

    use deckmaste_construction_core::macro_def::DeclarationKind;

    use super::BuildValue;
    use super::Category;
    use super::Construction;
    use super::Forest;
    use super::Leaf;
    use super::Lexical;
    use super::MaterializationStateFor;
    use super::RootRuleId;
    use super::RuleId;
    use super::build_root_rule;
    use super::materialize;
    use super::materialize_node;
    use super::materialize_observed;
    use crate::ast::BareSingularNominal;
    use crate::ast::BaseVerbPhrase;
    use crate::ast::CommonNoun;
    use crate::ast::DeclarationIntransitiveVerb;
    use crate::ast::FiniteClause;
    use crate::ast::Head;
    use crate::ast::Imperative;
    use crate::ast::IntransitiveLexicalVerbPhrase;
    use crate::ast::IntransitivePredicate;
    use crate::ast::LexicalVerbPhrase;
    use crate::ast::Noun;
    use crate::ast::NounSingularHead;
    use crate::ast::PersonalSubject;
    use crate::ast::PlainFiniteClause;
    use crate::ast::Predicate;
    use crate::ast::ScalarNumber;
    use crate::ast::SelfReferenceSpelling;
    use crate::ast::Sentence;
    use crate::ast::Subject;
    use crate::ast::SubjectPronoun;
    use crate::ast::UnqualifiedReference;
    use crate::ast::VerbPhrase;
    use crate::ast::WhereClause;
    use crate::ast::WhereClauseCategory;
    use crate::ast::WithWhere;
    use crate::constructions::ConcordClass;
    use crate::constructions::Determinative;
    use crate::constructions::DeterminativeHead;
    use crate::constructions::DeterminativeHeadLemma;
    use crate::constructions::DeterminerNumber;
    use crate::constructions::FeatureConstraint;
    use crate::constructions::FusedHeadLicense;
    use crate::constructions::LexicalOwnerTemplate;
    use crate::constructions::Nominal;
    use crate::constructions::NominalLicense;
    use crate::constructions::Number;
    use crate::constructions::Onset;
    use crate::constructions::PossessiveEnding;
    use crate::constructions::SingularSimpleDeterminative;
    use crate::context::ParseContext;
    use crate::environment::DeclarationId;
    use crate::environment::VerbInventoryRef;
    use crate::environment::canonical_test_environment;
    use crate::parser::SelectionResolution;
    use crate::parser::diagnostic::BoundedParseOutcome;
    use crate::parser::diagnostic::ParserTrace;
    use crate::parser::diagnostic::StructuralTrace;
    use crate::parser::diagnostic::TraceLimits;
    use crate::parser::engine::ChartFailure;
    use crate::parser::engine::Child;
    use crate::parser::engine::Family;
    use crate::parser::engine::NodeId;
    use crate::parser::engine::PackedNode;
    use crate::parser::engine::RulePosition;
    use crate::parser::engine::SpannedLexical;
    use crate::parser::scan::RootForest;
    use crate::parser::scan::SliceGrammar;
    use crate::parser::scan::parse_forest;
    use crate::render::Render;

    #[derive(Debug, Clone, PartialEq, Eq)]
    enum StableRulePosition {
        Nonterminal(Category),
        AdjacentNonterminal(Category),
        Lexical(Lexical),
        DeclarationDeterminative,
        DeclarationVerb(FeatureConstraint<ConcordClass>),
        DeclarationNoun(FeatureConstraint<Number>),
    }

    fn stable_positions(positions: &[RulePosition<Category, Lexical>]) -> Vec<StableRulePosition> {
        positions
            .iter()
            .map(|position| match position {
                RulePosition::Nonterminal(category) => StableRulePosition::Nonterminal(*category),
                RulePosition::AdjacentNonterminal(category) => {
                    StableRulePosition::AdjacentNonterminal(*category)
                }
                RulePosition::Lexical(Lexical::DeclarationVerb(_, feature)) => {
                    StableRulePosition::DeclarationVerb(*feature)
                }
                RulePosition::Lexical(Lexical::DeclarationDeterminative(_)) => {
                    StableRulePosition::DeclarationDeterminative
                }
                RulePosition::Lexical(Lexical::DeclarationNoun(_, feature)) => {
                    StableRulePosition::DeclarationNoun(*feature)
                }
                RulePosition::Lexical(lexical) => StableRulePosition::Lexical(*lexical),
            })
            .collect()
    }

    fn lexical(value: Leaf) -> Child<Leaf> {
        Child::Lexical(SpannedLexical {
            span: crate::parser::TextSpan { start: 0, end: 1 },
            value,
            owner: None,
        })
    }

    fn connive_phrase() -> VerbPhrase {
        let environment = canonical_test_environment();
        let declaration = DeclarationIntransitiveVerb::new(
            &environment,
            VerbInventoryRef::Declaration(DeclarationId::new(
                DeclarationKind::KeywordAction,
                "Connive",
            )),
        )
        .expect("the canonical environment declares intransitive Connive");
        VerbPhrase::BaseVerbPhrase(BaseVerbPhrase {
            frame: Box::new(LexicalVerbPhrase::IntransitiveLexicalVerbPhrase(
                IntransitiveLexicalVerbPhrase::IntransitivePredicate(IntransitivePredicate {
                    head: declaration,
                }),
            )),
        })
    }

    fn connive_leaf() -> Leaf {
        let environment = canonical_test_environment();
        let VerbPhrase::BaseVerbPhrase(BaseVerbPhrase { frame }) = connive_phrase() else {
            unreachable!("the helper constructs an intransitive predicate")
        };
        let LexicalVerbPhrase::IntransitiveLexicalVerbPhrase(
            IntransitiveLexicalVerbPhrase::IntransitivePredicate(predicate),
        ) = *frame
        else {
            unreachable!("the helper constructs an intransitive predicate")
        };
        let onset = environment
            .onset(
                &DeclarationId::new(DeclarationKind::KeywordAction, "Connive"),
                ::deckmaste_construction_core::macro_def::SurfaceFeature::PLAIN,
            )
            .expect("the canonical Connive row carries onset");
        Leaf::IntransitiveVerb {
            verb: predicate.head,
            concord_class: ConcordClass::Other,
            onset,
        }
    }

    fn slice_candidates(
        text: &str,
        card_name: &str,
    ) -> Result<RootForest, ChartFailure<Category, Lexical>> {
        let environment = canonical_test_environment();
        let context = context(card_name);
        let grammar = SliceGrammar {
            environment: &environment,
            context: &context,
        };

        parse_forest::<crate::ast::Ability>(&grammar, text)
    }

    fn context(card_name: &str) -> ParseContext<'_> {
        ParseContext::new(
            card_name,
            card_name == "Zacama, Primal Calamity",
            if card_name == "Artifact Avatar" {
                deckmaste_construction_core::macro_def::Onset::Vowel
            } else {
                deckmaste_construction_core::macro_def::Onset::Consonant
            },
        )
        .expect("test card names are valid parse contexts")
    }

    fn assert_acyclic_number_survives(forest: &Forest<RuleId, Leaf>) {
        let mut state =
            MaterializationStateFor::<BuildValue, Construction, Category, Lexical, Leaf>::default();
        let outcome = materialize_node(forest, NodeId(0), &context("Context Card"), &mut state);

        assert_eq!(outcome.values.len(), 1);
        assert!(matches!(
            outcome.values[0].value.as_ref(),
            BuildValue::Amount(
                crate::ast::Amount::Number(crate::ast::NumberAmount {
                    number: ScalarNumber { magnitude: 3 },
                }),
                _
            )
        ));
    }

    fn with_where_family(body: NodeId) -> Family<Leaf> {
        Family {
            children: vec![
                Child::Node(body),
                lexical(Leaf::Literal(",")),
                Child::Node(NodeId(5)),
            ],
        }
    }

    #[test]
    fn root_adapter_materialization_is_identity_over_one_semantic_child() {
        let value = BuildValue::Sentence(
            Sentence::Imperative(
                Imperative::new(Box::new(Predicate::Atomic(Box::new(connive_phrase()))))
                    .expect("bare test predicate satisfies imperative concord_class"),
            ),
            FeatureConstraint::Any,
        );
        let adapter_children = [
            value.clone(),
            BuildValue::Leaf(Leaf::Literal(".")),
            BuildValue::Leaf(Leaf::EndOfInput),
        ];

        assert_eq!(
            build_root_rule(
                RootRuleId::Adapter,
                &adapter_children,
                &context("Context Card"),
            ),
            Ok(Some(value.clone())),
        );
        assert_eq!(
            build_root_rule(
                RootRuleId::Adapter,
                &[value.clone(), value],
                &context("Context Card"),
            ),
            Ok(None),
            "a second semantic child cannot be hidden behind the root adapter",
        );
    }

    #[test]
    fn determined_nominal_build_derives_reference_onset_from_nominal() {
        let parse_context = context("Context Card");
        let children = |determiner_onset, nominal_onset| {
            let determiner =
                Determinative::SingularSimpleDeterminative(SingularSimpleDeterminative {
                    head: DeterminativeHead::Closed(DeterminativeHeadLemma::IndefiniteArticle),
                });
            [
                BuildValue::Determinative(
                    determiner,
                    ConcordClass::ThirdPersonSingular,
                    Number::Singular,
                    DeterminerNumber::SingularOnly,
                    FusedHeadLicense::NominalOnly,
                    NominalLicense::CountNominal,
                    determiner_onset,
                    FeatureConstraint::Any,
                ),
                BuildValue::Nominal(
                    Nominal::BareSingularNominal(
                        BareSingularNominal::new(Head::NounSingularHead(
                            NounSingularHead::new(Noun::Lexeme(CommonNoun::Player))
                                .expect("player is a count noun"),
                        ))
                        .expect("the Singular nominal accepts a Singular Head"),
                    ),
                    ConcordClass::ThirdPersonSingular,
                    Number::Singular,
                    nominal_onset,
                    PossessiveEnding::Other,
                    FeatureConstraint::Any,
                ),
            ]
        };

        for (determiner_onset, nominal_onset) in [
            (Onset::Vowel, Onset::Vowel),
            (Onset::Consonant, Onset::Consonant),
            (Onset::Vowel, Onset::Consonant),
            (Onset::Consonant, Onset::Vowel),
        ] {
            assert!(matches!(
                super::build_checked(
                    RuleId::UnqualifiedReferenceDeterminedNominalDetPresent,
                    &children(determiner_onset, nominal_onset),
                    &parse_context,
                ),
                Ok(Some(BuildValue::UnqualifiedReference(
                    UnqualifiedReference::DeterminedNominal(_),
                    ConcordClass::ThirdPersonSingular,
                    Number::Singular,
                    onset,
                    _,
                    _,
                ))) if onset == nominal_onset
            ));
        }

        let identity_context = context("Artifact Avatar");
        assert!(matches!(
            super::build_checked(
                RuleId::UnqualifiedReferenceSelfReference,
                &[BuildValue::Leaf(Leaf::SelfReference(
                    SelfReferenceSpelling::Full,
                ))],
                &identity_context,
            ),
            Ok(Some(BuildValue::UnqualifiedReference(
                _,
                _,
                _,
                Onset::Vowel,
                _,
                _
            )))
        ));
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
                        children: vec![lexical(Leaf::ScalarNumber(ScalarNumber { magnitude: 3 }))],
                    },
                ],
            }],
            vec![NodeId(0)],
        );

        assert_acyclic_number_survives(&forest);
    }

    #[test]
    fn memo_hits_share_materialized_values_and_provenance() {
        let forest = Forest::from_test_parts(
            vec![PackedNode {
                rule: RuleId::AmountNumber,
                start: 0,
                end: 1,
                families: vec![Family {
                    children: vec![lexical(Leaf::ScalarNumber(ScalarNumber { magnitude: 3 }))],
                }],
            }],
            vec![NodeId(0)],
        );
        let mut state =
            MaterializationStateFor::<BuildValue, Construction, Category, Lexical, Leaf>::default();

        let first = materialize_node(&forest, NodeId(0), &context("Context Card"), &mut state);
        let memo_hit = materialize_node(&forest, NodeId(0), &context("Context Card"), &mut state);

        assert_eq!(first.values, memo_hit.values);
        assert!(std::sync::Arc::ptr_eq(
            &first.values[0].value,
            &memo_hit.values[0].value,
        ));
        assert!(std::sync::Arc::ptr_eq(
            &first.values[0].constructions,
            &memo_hit.values[0].constructions,
        ));
        assert!(std::sync::Arc::ptr_eq(
            &first.values[0].positions,
            &memo_hit.values[0].positions,
        ));
        assert!(std::sync::Arc::ptr_eq(
            &first.values[0].claims,
            &memo_hit.values[0].claims,
        ));
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
                            children: vec![lexical(Leaf::ScalarNumber(ScalarNumber {
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
    #[allow(
        clippy::too_many_lines,
        reason = "the hand-built cyclic forest and its exact provenance oracle stay together"
    )]
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
                    rule: RuleId::PredicateAtomic,
                    start: 0,
                    end: 0,
                    families: vec![Family {
                        children: vec![Child::Node(NodeId(4))],
                    }],
                },
                PackedNode {
                    rule: RuleId::VerbPhraseBaseVerbPhrase,
                    start: 0,
                    end: 0,
                    families: vec![Family {
                        children: vec![Child::Node(NodeId(7))],
                    }],
                },
                PackedNode {
                    rule: RuleId::WhereClauseCategoryWhere,
                    start: 0,
                    end: 0,
                    families: vec![Family {
                        children: vec![lexical(Leaf::Literal("where")), Child::Node(NodeId(9))],
                    }],
                },
                PackedNode {
                    rule: RuleId::SubjectSubjectPronoun,
                    start: 0,
                    end: 0,
                    families: vec![Family {
                        children: vec![lexical(Leaf::SubjectPronoun(SubjectPronoun::You))],
                    }],
                },
                PackedNode {
                    rule: RuleId::LexicalVerbPhraseIntransitiveLexicalVerbPhrase,
                    start: 0,
                    end: 0,
                    families: vec![Family {
                        children: vec![Child::Node(NodeId(8))],
                    }],
                },
                PackedNode {
                    rule: RuleId::IntransitiveLexicalVerbPhraseIntransitivePredicate,
                    start: 0,
                    end: 0,
                    families: vec![Family {
                        children: vec![lexical(connive_leaf())],
                    }],
                },
                PackedNode {
                    rule: RuleId::FiniteClausePlainFiniteClause,
                    start: 0,
                    end: 0,
                    families: vec![Family {
                        children: vec![Child::Node(NodeId(6)), Child::Node(NodeId(3))],
                    }],
                },
            ],
            vec![NodeId(0), NodeId(1)],
        );
        let context = context("Context Card");
        let mut state =
            MaterializationStateFor::<BuildValue, Construction, Category, Lexical, Leaf>::default();
        let clause = WhereClauseCategory::Where(WhereClause {
            clause: FiniteClause::PlainFiniteClause(
                PlainFiniteClause::new(
                    Subject::SubjectPronoun(PersonalSubject {
                        word: SubjectPronoun::You,
                    }),
                    Box::new(Predicate::Atomic(Box::new(connive_phrase()))),
                )
                .expect("you and a bare predicate agree"),
            ),
        });
        let base = Sentence::Imperative(
            Imperative::new(Box::new(Predicate::Atomic(Box::new(connive_phrase()))))
                .expect("bare test predicate satisfies imperative concord_class"),
        );
        let once = Sentence::WithWhere(WithWhere {
            body: Box::new(base),
            clause: clause.clone(),
        });

        let first = materialize_node(&forest, NodeId(0), &context, &mut state);
        assert_eq!(first.values.len(), 1);
        assert_eq!(
            first.values[0].value.as_ref(),
            &BuildValue::Sentence(once.clone(), FeatureConstraint::Any)
        );

        let later = materialize_node(&forest, NodeId(1), &context, &mut state);
        assert_eq!(later.values.len(), 1);
        assert_eq!(
            later.values[0].value.as_ref(),
            &BuildValue::Sentence(
                Sentence::WithWhere(WithWhere {
                    body: Box::new(once),
                    clause,
                }),
                FeatureConstraint::Any
            )
        );
        assert_eq!(
            later.values[0].constructions.as_ref(),
            &[
                Construction::SentenceWithWhere,
                Construction::SentenceWithWhere,
                Construction::SentenceImperative,
                Construction::VerbPhraseBaseVerbPhrase,
                Construction::IntransitiveLexicalVerbPhraseIntransitivePredicate,
                Construction::WhereClauseCategoryWhere,
                Construction::FiniteClausePlainFiniteClause,
                Construction::SubjectSubjectPronoun,
                Construction::VerbPhraseBaseVerbPhrase,
                Construction::IntransitiveLexicalVerbPhraseIntransitivePredicate,
                Construction::WhereClauseCategoryWhere,
                Construction::FiniteClausePlainFiniteClause,
                Construction::SubjectSubjectPronoun,
                Construction::VerbPhraseBaseVerbPhrase,
                Construction::IntransitiveLexicalVerbPhraseIntransitivePredicate,
            ]
        );
        assert_eq!(
            stable_positions(&later.values[0].positions),
            stable_positions(&[
                RulePosition::Nonterminal(Category::Sentence),
                RulePosition::Lexical(Lexical::Literal(",")),
                RulePosition::Nonterminal(Category::WhereClauseCategory),
                RulePosition::Nonterminal(Category::Sentence),
                RulePosition::Lexical(Lexical::Literal(",")),
                RulePosition::Nonterminal(Category::WhereClauseCategory),
                RulePosition::Nonterminal(Category::Predicate),
                RulePosition::Nonterminal(Category::LexicalVerbPhrase),
                RulePosition::Lexical(Lexical::DeclarationVerb(0, FeatureConstraint::Any,)),
                RulePosition::Lexical(Lexical::Literal("where")),
                RulePosition::Nonterminal(Category::FiniteClause),
                RulePosition::Nonterminal(Category::Subject),
                RulePosition::Nonterminal(Category::Predicate),
                RulePosition::Lexical(Lexical::SubjectPronoun),
                RulePosition::Nonterminal(Category::LexicalVerbPhrase),
                RulePosition::Lexical(Lexical::DeclarationVerb(0, FeatureConstraint::Any,)),
                RulePosition::Lexical(Lexical::Literal("where")),
                RulePosition::Nonterminal(Category::FiniteClause),
                RulePosition::Nonterminal(Category::Subject),
                RulePosition::Nonterminal(Category::Predicate),
                RulePosition::Lexical(Lexical::SubjectPronoun),
                RulePosition::Nonterminal(Category::LexicalVerbPhrase),
                RulePosition::Lexical(Lexical::DeclarationVerb(0, FeatureConstraint::Any,)),
            ]),
        );
    }

    #[test]
    fn final_trace_rejection_ignores_a_rejected_packed_child_sibling() {
        #[derive(Debug, Clone, Copy, PartialEq, Eq)]
        enum TestCategory {
            Parent,
            Child,
        }
        #[derive(Debug, Clone, Copy, PartialEq, Eq)]
        enum TestLexical {
            Token,
        }
        #[derive(Debug, Clone, Copy, PartialEq, Eq)]
        enum TestRule {
            Parent,
            Child,
        }
        #[derive(Debug, Clone, Copy, PartialEq, Eq)]
        enum TestValue {
            Token(&'static str),
            Child,
        }

        let stale_child = crate::constructions::BuildRejection::new(
            "AChild",
            "guarded",
            crate::constructions::BuildViolation::Invariant {
                identity: "token is good",
            },
        );
        let current_parent = crate::constructions::BuildRejection::new(
            "CParent",
            "guarded",
            crate::constructions::BuildViolation::Invariant {
                identity: "child is accepted",
            },
        );
        let rules = [
            crate::parser::engine::Rule {
                id: TestRule::Parent,
                lhs: TestCategory::Parent,
                rhs: &[crate::parser::engine::RulePosition::Nonterminal(
                    TestCategory::Child,
                )],
            },
            crate::parser::engine::Rule {
                id: TestRule::Child,
                lhs: TestCategory::Child,
                rhs: &[crate::parser::engine::RulePosition::Lexical(
                    TestLexical::Token,
                )],
            },
        ];
        let child = PackedNode {
            rule: TestRule::Child,
            start: 0,
            end: 1,
            families: vec![
                Family {
                    children: vec![Child::Lexical(SpannedLexical {
                        span: crate::parser::TextSpan { start: 0, end: 1 },
                        value: "bad",
                        owner: None::<()>,
                    })],
                },
                Family {
                    children: vec![Child::Lexical(SpannedLexical {
                        span: crate::parser::TextSpan { start: 0, end: 1 },
                        value: "good",
                        owner: None::<()>,
                    })],
                },
            ],
        };
        let parent = PackedNode {
            rule: TestRule::Parent,
            start: 0,
            end: 1,
            families: vec![Family {
                children: vec![Child::Node(NodeId(1))],
            }],
        };
        let build = |rule, children: &[TestValue]| match (rule, children) {
            (TestRule::Child, [TestValue::Token("bad")]) => Err(stale_child),
            (TestRule::Child, [TestValue::Token("good")]) => Ok(Some(TestValue::Child)),
            (TestRule::Parent, [TestValue::Child]) => Err(current_parent),
            _ => Ok(None),
        };
        let rule_index = |rule| match rule {
            TestRule::Parent => 0,
            TestRule::Child => 1,
        };

        let rejected_root = Forest::from_test_parts(vec![parent, child.clone()], vec![NodeId(0)]);
        let (values, terminal_rejection) = super::materialize_with_rejection(
            &rejected_root,
            &rules,
            rule_index,
            |_| None::<()>,
            std::convert::identity,
            |token| TestValue::Token(token),
            build,
        );
        assert!(values.is_empty());
        assert_eq!(terminal_rejection, Some(current_parent));
        let (values, trace) = super::materialize_with_trace(
            &rejected_root,
            &rules,
            rule_index,
            |_| None::<()>,
            std::convert::identity,
            |token| TestValue::Token(token),
            build,
            |rule| format!("{rule:?}"),
            TraceLimits::new(4),
        );
        assert!(values.is_empty());
        assert_eq!(trace.first_build_rejection(), Some(&current_parent));

        let successful_root = Forest::from_test_parts(vec![child], vec![NodeId(0)]);
        let (values, terminal_rejection) = super::materialize_with_rejection(
            &successful_root,
            &rules,
            rule_index,
            |_| None::<()>,
            std::convert::identity,
            |token| TestValue::Token(token),
            build,
        );
        assert_eq!(values.len(), 1);
        assert!(terminal_rejection.is_none());
        let (values, trace) = super::materialize_with_trace(
            &successful_root,
            &rules,
            rule_index,
            |_| None::<()>,
            std::convert::identity,
            |token| TestValue::Token(token),
            build,
            |rule| format!("{rule:?}"),
            TraceLimits::new(4),
        );
        assert_eq!(values.len(), 1);
        assert!(trace.first_build_rejection().is_none());
    }

    #[test]
    fn materialize_builds_checked_slice_values() {
        for (text, card_name, expected_candidates) in [
            ("Destroy target creature.", "Context Card", 1),
            (
                "Whenever a player connives, you gain X life.",
                "Context Card",
                1,
            ),
            (
                "You gain X life, where X is the number of creatures you control with power 2 or less.",
                "Context Card",
                3,
            ),
            (
                "Zacama deals 3 damage to target creature.",
                "Zacama, Primal Calamity",
                1,
            ),
        ] {
            let forest = slice_candidates(text, card_name).expect("scanner accepts rendered input");
            let context = context(card_name);
            let environment = canonical_test_environment();
            let candidates = materialize::<crate::ast::Ability>(&forest, &context, &environment);
            assert_eq!(
                candidates.len(),
                expected_candidates,
                "unexpected candidates for {text:?}",
            );
            assert!(
                candidates
                    .iter()
                    .all(|candidate| candidate.value.render(&context, &environment) == text)
            );
            if text.contains("the number of creatures") {
                let parser = crate::parser::Parser::new(environment.clone())
                    .expect("canonical environment satisfies the grammar");
                let analysis = parser.analyze(text, &context);
                let decision = analysis.decision().expect("two readings require selection");
                assert_eq!(decision.resolution(), SelectionResolution::Specificity);
                assert!(decision.exception_uses().is_empty());
                assert_eq!(decision.survivors().len(), 1);
                let selected = decision
                    .selected()
                    .expect("specificity selects one reading");
                let selected_path = decision
                    .candidates()
                    .iter()
                    .find(|candidate| candidate.ordinal() == selected)
                    .expect("the selected ordinal names a candidate")
                    .construction_path();
                let selected_postmodifiers = selected_path
                    .iter()
                    .filter(|name| name.starts_with("PostmodifiedReference"))
                    .map(String::as_str)
                    .collect::<Vec<_>>();
                assert_eq!(
                    selected_postmodifiers,
                    [
                        "PostmodifiedReferenceUnqualifiedPostmodifiedReference",
                        "PostmodifiedReferenceRelationalQualifiedReference",
                        "PostmodifiedReferenceUnqualifiedPostmodifiedReference",
                        "PostmodifiedReferenceScalarQualifiedReference",
                        "PostmodifiedReferenceRelativeQualifiedReference",
                        "PostmodifiedReferenceUnqualifiedPostmodifiedReference",
                    ],
                    "specificity attaches the scalar phrase to creatures, not number",
                );
                assert_eq!(
                    analysis
                        .selected()
                        .expect("the analysis selected an ability")
                        .render(&context, &environment),
                    text,
                );
                let ownership = analysis.ownership().expect("selected input has ownership");
                assert_eq!(ownership.rendered_text(), text);
                let stable_claims = |claims: &[crate::parser::LexicalClaim]| {
                    claims
                        .iter()
                        .map(|claim| {
                            (
                                claim.span(),
                                claim.kind(),
                                claim.stable_owner_id().to_owned(),
                            )
                        })
                        .collect::<Vec<_>>()
                };
                assert_eq!(
                    stable_claims(ownership.parsed_claims()),
                    stable_claims(ownership.rendered_claims()),
                );
                assert!(ownership.failures().is_empty());
                assert!(ownership.summary().covered());
                assert_eq!(ownership.summary().claimed_bytes(), text.len());
            }
        }
    }
    #[test]
    fn materialize_accepts_a_plural_count_subject_with_a_bare_verb() {
        let text = "Creatures you control with power 2 or less gain X life.";
        let forest = slice_candidates(text, "Context Card").expect("scanner accepts words");
        let context = context("Context Card");
        let environment = canonical_test_environment();
        let candidates = materialize::<crate::ast::Ability>(&forest, &context, &environment);
        assert_eq!(candidates.len(), 1);
        assert!(
            candidates
                .iter()
                .all(|candidate| candidate.value.render(&context, &environment) == text)
        );
        let parser = crate::parser::Parser::new(environment)
            .expect("canonical environment satisfies the grammar");
        let analysis = parser.analyze(text, &context);
        assert_eq!(
            analysis
                .decision()
                .expect("the finite clause is the only reading")
                .resolution(),
            SelectionResolution::Unique,
        );
        assert_eq!(
            analysis
                .selected()
                .expect("the finite clause selects")
                .render(&context, parser.environment()),
            text,
        );
    }

    #[test]
    fn materialize_preserves_preorder_constructions_and_declared_positions() {
        let forest = slice_candidates("Destroy target creature.", "Context Card").unwrap();
        let environment = canonical_test_environment();
        let candidates =
            materialize::<crate::ast::Ability>(&forest, &context("Context Card"), &environment);

        assert_eq!(candidates.len(), 1);
        assert_eq!(
            candidates[0].constructions,
            vec![
                Construction::AbilityPlain,
                Construction::AbilityBodySentences,
                Construction::SentenceImperative,
                Construction::VerbPhraseBaseVerbPhrase,
                Construction::TransitiveLexicalVerbPhraseTransitivePredicate,
                Construction::ObjectObjectNominal,
                Construction::NounPhraseQualifiedNounPhrase,
                Construction::PostmodifiedReferenceUnqualifiedPostmodifiedReference,
                Construction::UnqualifiedReferenceDeterminedNominal,
                Construction::DeterminativeTargetingMarkerDeterminative,
                Construction::NominalBareSingularNominal,
                Construction::HeadNounSingularHead,
            ]
        );
        assert_eq!(
            stable_positions(&candidates[0].positions),
            stable_positions(&[
                RulePosition::Nonterminal(Category::AbilityBody),
                RulePosition::Nonterminal(Category::SentencesSentencesSequenceCategory),
                RulePosition::Nonterminal(Category::Predicate),
                RulePosition::Nonterminal(Category::LexicalVerbPhrase),
                RulePosition::Lexical(Lexical::DeclarationVerb(0, FeatureConstraint::Any,)),
                RulePosition::Nonterminal(Category::Object),
                RulePosition::Nonterminal(Category::NounPhrase),
                RulePosition::Nonterminal(Category::PostmodifiedReference),
                RulePosition::Nonterminal(Category::UnqualifiedReference),
                RulePosition::Nonterminal(Category::Determinative),
                RulePosition::Nonterminal(Category::Nominal),
                RulePosition::Lexical(Lexical::TargetingMarker),
                RulePosition::Nonterminal(Category::Head),
                RulePosition::Lexical(Lexical::DeclarationNoun(
                    0,
                    FeatureConstraint::Exact(Number::Singular),
                )),
            ]),
        );
    }

    #[test]
    fn materialize_selected_provenance_is_surface_ordered() {
        let forest = slice_candidates("Destroy target creature.", "Context Card").unwrap();
        let environment = canonical_test_environment();
        let candidates =
            materialize::<crate::ast::Ability>(&forest, &context("Context Card"), &environment);

        let claims = &candidates[0].claims;
        assert_eq!(
            claims
                .iter()
                .map(|claim| (claim.span, claim.owner.stable_id()))
                .collect::<Vec<_>>(),
            vec![
                (
                    crate::parser::TextSpan { start: 0, end: 7 },
                    "lexeme:keyword_action/Destroy/bare",
                ),
                (
                    crate::parser::TextSpan { start: 7, end: 14 },
                    "vocab:TargetingMarker/Target",
                ),
                (
                    crate::parser::TextSpan { start: 14, end: 23 },
                    "lexeme:type/Creature/singular",
                ),
                (
                    crate::parser::TextSpan { start: 23, end: 24 },
                    "structural:Sentences/sentences/terminator/0",
                ),
            ]
        );
    }

    #[test]
    fn owner_template_mismatch_becomes_public_synthetic_failure() {
        let forest = slice_candidates("Destroy target creature.", "Context Card").unwrap();
        let roots = forest.accepted_root_ids().collect::<Vec<_>>();
        let mut nodes = forest
            .nodes()
            .map(|(_, node)| node.clone())
            .collect::<Vec<_>>();
        let mut removed_span = None;
        for node in &mut nodes {
            for family in &mut node.families {
                for child in &mut family.children {
                    if let Child::Lexical(claim) = child
                        && matches!(claim.value, Leaf::TransitiveVerb { .. })
                        && claim.span == (crate::parser::TextSpan { start: 0, end: 7 })
                    {
                        claim.owner = LexicalOwnerTemplate::Vocab {
                            declaration: "Pronoun",
                        }
                        .instantiate(&claim.value);
                        removed_span = Some(claim.span);
                    }
                }
            }
        }
        let removed_span = removed_span.expect("real Destroy family child carried an owner");
        let forest = RootForest::from_test_forest(Forest::from_test_parts(nodes, roots));
        let context = context("Context Card");
        let environment = canonical_test_environment();
        let candidates = materialize::<crate::ast::Ability>(&forest, &context, &environment);
        let analysis = super::super::analyze_materialized_with_ownership(
            "Destroy target creature.",
            candidates,
            &context,
            &environment,
        );

        assert_eq!(
            analysis.outcome(),
            crate::parser::ParseAnalysisOutcome::Selected
        );
        assert!(
            analysis
                .ownership()
                .unwrap()
                .failures()
                .iter()
                .any(|failure| {
                    matches!(
                        failure,
                        crate::parser::OwnershipFailure::Synthetic { span } if *span == removed_span
                    )
                })
        );
    }

    fn ability_forest_with_duplicate_root_cycles() -> RootForest {
        let forest = slice_candidates("Destroy target creature.", "Context Card")
            .expect("ordinary slice forest");
        let root = forest
            .accepted_root_ids()
            .next()
            .expect("slice has an accepted root");
        let mut nodes = forest
            .nodes()
            .map(|(_, node)| node.clone())
            .collect::<Vec<_>>();
        let bridge = NodeId(nodes.len());
        nodes.push(PackedNode {
            rule: RootRuleId::Grammar(RuleId::SentenceImperative),
            start: 0,
            end: 0,
            families: vec![Family {
                children: vec![Child::Node(root)],
            }],
        });
        nodes[root.0].families.insert(
            0,
            Family {
                children: vec![Child::Node(bridge)],
            },
        );
        nodes[root.0].families.insert(
            1,
            Family {
                children: vec![Child::Node(bridge)],
            },
        );
        RootForest::from_test_forest(Forest::from_test_parts(nodes, vec![root, root]))
    }

    #[test]
    fn parser_trace_cycle_pruning_is_deduplicated_bounded_and_semantically_inert() {
        let context = context("Context Card");
        let environment = canonical_test_environment();
        for limit in [0, 1, 8] {
            let forest = ability_forest_with_duplicate_root_cycles();
            let ordinary_candidates =
                materialize::<crate::ast::Ability>(&forest, &context, &environment);
            let (candidates, materialization) =
                materialize_observed(&forest, &context, &environment, TraceLimits::new(limit));
            let (repeated_candidates, repeated_materialization) =
                materialize_observed(&forest, &context, &environment, TraceLimits::new(limit));
            assert_eq!(ordinary_candidates, candidates);
            assert_eq!(candidates, repeated_candidates);
            assert_eq!(materialization, repeated_materialization);
            let expected_debug = format!("{:?}", candidates[0].value);
            let analysis = crate::parser::analyze_materialized_with_ownership(
                "Destroy target creature.",
                candidates,
                &context,
                &environment,
            );
            let trace = ParserTrace::from_parts(
                analysis,
                StructuralTrace::empty(),
                materialization,
                TraceLimits::new(limit),
                &context,
                &environment,
            );

            let BoundedParseOutcome::Selected(selected) = trace.outcome() else {
                panic!("well-founded candidate survives the pruned cycle");
            };
            assert_eq!(selected.selection().unselected_candidates().total(), 0);
            assert_eq!(trace.materialization_cycles().total(), 1);
            assert_eq!(
                trace.materialization_cycles().shown(),
                usize::from(limit > 0)
            );
            assert_eq!(trace.materialized_candidates().total(), 1);
            assert_eq!(
                trace.materialized_candidates().shown(),
                usize::from(limit > 0)
            );
            if limit > 0 {
                let cycle = &trace.materialization_cycles().items()[0];
                assert_eq!(cycle.construction_path().total(), 2);
                assert_eq!(cycle.construction_path().shown(), usize::min(limit, 2));
                assert_eq!(
                    cycle.construction_path().items(),
                    &["RootAdapter".to_owned(), "SentenceImperative".to_owned(),]
                        [..usize::min(limit, 2)]
                );
                let candidate = &trace.materialized_candidates().items()[0];
                assert_eq!(candidate.rendered(), "Destroy target creature.");
                assert_eq!(candidate.ast_debug_v1(), expected_debug);
            }
            assert!(trace.into_parse_result().is_ok());
        }
    }

    #[test]
    fn parser_trace_internal_only_cycle_keeps_typed_materialization_failure() {
        let forest = RootForest::from_test_forest(Forest::from_test_parts(
            vec![PackedNode {
                rule: RootRuleId::Grammar(RuleId::AbilityPlain),
                start: 0,
                end: 0,
                families: vec![Family {
                    children: vec![Child::Node(NodeId(0))],
                }],
            }],
            vec![NodeId(0)],
        ));
        let context = context("Context Card");
        let environment = canonical_test_environment();
        let (candidates, materialization) =
            materialize_observed(&forest, &context, &environment, TraceLimits::new(8));
        let trace = ParserTrace::from_parts(
            crate::parser::analyze_materialized(candidates),
            StructuralTrace::empty(),
            materialization,
            TraceLimits::new(8),
            &context,
            &environment,
        );

        let BoundedParseOutcome::InternalFailure(failure) = trace.outcome() else {
            panic!("cycle-only root must remain an internal failure");
        };
        assert_eq!(
            failure.kind(),
            crate::parser::InternalFailureKind::ValidatedRootDidNotMaterialize
        );
        assert_eq!(
            failure.message(),
            "Ability root: validated chart root did not materialize"
        );
        assert_eq!(trace.materialization_cycles().total(), 1);
        assert_eq!(
            trace.into_parse_result(),
            Err(crate::parser::ParseError::ValidatedRootDidNotMaterialize)
        );
    }
}
