use std::cell::OnceCell;
use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::hash_map::DefaultHasher;
use std::hash::Hash;
use std::hash::Hasher;
use std::sync::Arc;

use super::TextSpan;
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
use super::ownership::RawRenderedClaim;
use super::selection::specificity_tiers;
use crate::ast::Ability;
use crate::constructions::AdmissibleSites;
use crate::constructions::AttachmentSitePath;
use crate::constructions::AttachmentSiteStep;
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
use crate::constructions::Visitor;
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
    static SCOPE_COLLAPSE_ENABLED: std::cell::Cell<bool> = const { std::cell::Cell::new(true) };
}

#[cfg(test)]
fn without_scope_collapse<T>(run: impl FnOnce() -> T) -> T {
    struct Reset;

    impl Drop for Reset {
        fn drop(&mut self) {
            SCOPE_COLLAPSE_ENABLED.with(|enabled| enabled.set(true));
        }
    }

    SCOPE_COLLAPSE_ENABLED.with(|enabled| enabled.set(false));
    let reset = Reset;
    let result = run();
    drop(reset);
    result
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
            push_unique(&mut candidates, candidate);
        }
    }
    #[cfg(test)]
    let collapse_enabled = SCOPE_COLLAPSE_ENABLED.with(std::cell::Cell::get);
    #[cfg(not(test))]
    let collapse_enabled = true;
    if collapse_enabled {
        candidates = collapse_scope_candidates(candidates, context, environment);
    }
    for (ordinal, candidate) in candidates.iter().enumerate() {
        #[cfg(test)]
        count_specificity_candidate();
        if let Some((trace, limits)) = observation.as_mut() {
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
    #[cfg(test)]
    FINALIZED_CANDIDATE_COUNT.with(|count| count.set(candidates.len()));
    candidates
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum ScopeNodeKind {
    Construction(&'static str, usize),
    Group,
    Leaf(usize),
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct ScopeNode {
    kind: ScopeNodeKind,
    children: Vec<ScopeChild>,
    span: Option<(usize, usize)>,
    domain: usize,
    verb_frame: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct ScopeChild {
    step: Option<ScopeStep>,
    mobile: Option<MobileRole>,
    node: ScopeNode,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum ScopeStep {
    Role(&'static str),
    Conjunct(&'static str, usize),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct MobileRole {
    role: &'static str,
    scope_sibling: Option<&'static str>,
}

enum ScopeFrame {
    Root {
        children: Vec<ScopeChild>,
    },
    Construction {
        identity: &'static str,
        index: usize,
        domain: usize,
        verb_frame: bool,
        children: Vec<ScopeChild>,
    },
    Edge {
        step: ScopeStep,
        mobile: Option<MobileRole>,
        domain: usize,
        children: Vec<ScopeChild>,
    },
    FrameRole {
        domain: usize,
        children: Vec<ScopeChild>,
    },
}

#[derive(Default)]
struct ScopeProjectionVisitor {
    frames: Vec<ScopeFrame>,
    root: Option<ScopeNode>,
    next_construction: usize,
    next_leaf: usize,
    next_domain: usize,
}

impl ScopeProjectionVisitor {
    fn current_domain(&self) -> usize {
        match self.frames.last() {
            Some(
                ScopeFrame::Construction { domain, .. }
                | ScopeFrame::Edge { domain, .. }
                | ScopeFrame::FrameRole { domain, .. },
            ) => *domain,
            Some(ScopeFrame::Root { .. }) | None => 0,
        }
    }

    fn push_node(&mut self, node: ScopeNode) {
        let child = ScopeChild {
            step: None,
            mobile: None,
            node,
        };
        match self.frames.last_mut() {
            Some(
                ScopeFrame::Root { children }
                | ScopeFrame::Construction { children, .. }
                | ScopeFrame::Edge { children, .. }
                | ScopeFrame::FrameRole { children, .. },
            ) => {
                children.push(child);
            }
            None => {
                debug_assert!(self.root.is_none());
                self.root = Some(child.node);
            }
        }
    }

    fn finish(mut self) -> ScopeProjection {
        if let Some(ScopeFrame::Root { children }) = self.frames.pop() {
            debug_assert!(self.frames.is_empty());
            let span = child_span(&children);
            return ScopeProjection {
                root: ScopeNode {
                    kind: ScopeNodeKind::Group,
                    children,
                    span,
                    domain: 0,
                    verb_frame: false,
                },
            };
        }
        debug_assert!(self.frames.is_empty());
        ScopeProjection {
            root: self
                .root
                .take()
                .expect("a generated root visits one construction"),
        }
    }
}

impl Visitor for ScopeProjectionVisitor {
    fn enter_construction(&mut self, construction: &'static str) {
        let index = self.next_construction;
        self.next_construction += 1;
        let domain = self.current_domain();
        self.frames.push(ScopeFrame::Construction {
            identity: construction,
            index,
            domain,
            verb_frame: false,
            children: Vec::new(),
        });
    }

    fn verb_frame(&mut self, construction: &'static str) {
        let Some(ScopeFrame::Construction {
            identity,
            verb_frame,
            ..
        }) = self.frames.last_mut()
        else {
            unreachable!("a Verb Frame callback belongs to a construction")
        };
        debug_assert_eq!(*identity, construction);
        *verb_frame = true;
    }

    fn enter_frame_role(&mut self, _role: &'static str) {
        self.next_domain += 1;
        self.frames.push(ScopeFrame::FrameRole {
            domain: self.next_domain,
            children: Vec::new(),
        });
    }

    fn exit_frame_role(&mut self) {
        let Some(ScopeFrame::FrameRole { domain, children }) = self.frames.pop() else {
            unreachable!("generated Verb Frame role visits are balanced")
        };
        let span = child_span(&children);
        self.push_node(ScopeNode {
            kind: ScopeNodeKind::Group,
            children,
            span,
            domain,
            verb_frame: false,
        });
    }

    fn exit_construction(&mut self) {
        let Some(ScopeFrame::Construction {
            identity,
            index,
            domain,
            verb_frame,
            children,
        }) = self.frames.pop()
        else {
            unreachable!("generated construction visits are balanced")
        };
        let span = child_span(&children);
        self.push_node(ScopeNode {
            kind: ScopeNodeKind::Construction(identity, index),
            children,
            span,
            domain,
            verb_frame,
        });
    }

    fn enter_role(
        &mut self,
        role: &'static str,
        scope_sibling: Option<&'static str>,
        admissible_sites: Option<&AdmissibleSites>,
    ) {
        let domain = self.current_domain();
        let fills_frame_role = matches!(self.frames.last(), Some(ScopeFrame::FrameRole { .. }));
        self.frames.push(ScopeFrame::Edge {
            step: ScopeStep::Role(role),
            mobile: (!fills_frame_role)
                .then_some(admissible_sites)
                .flatten()
                .map(|_| MobileRole {
                    role,
                    scope_sibling,
                }),
            domain,
            children: Vec::new(),
        });
    }

    fn exit_role(&mut self) {
        self.exit_edge();
    }

    fn enter_conjunct(&mut self, role: &'static str, ordinal: usize) {
        let domain = self.current_domain();
        self.frames.push(ScopeFrame::Edge {
            step: ScopeStep::Conjunct(role, ordinal),
            mobile: None,
            domain,
            children: Vec::new(),
        });
    }

    fn exit_conjunct(&mut self) {
        self.exit_edge();
    }

    fn scope_leaf(&mut self) {
        let index = self.next_leaf;
        self.next_leaf += 1;
        let domain = self.current_domain();
        self.push_node(ScopeNode {
            kind: ScopeNodeKind::Leaf(index),
            children: Vec::new(),
            span: Some((index, index + 1)),
            domain,
            verb_frame: false,
        });
    }
}

impl ScopeProjectionVisitor {
    fn exit_edge(&mut self) {
        let Some(ScopeFrame::Edge {
            step,
            mobile,
            domain,
            children,
        }) = self.frames.pop()
        else {
            unreachable!("generated role visits are balanced")
        };
        let span = child_span(&children);
        let node = ScopeNode {
            kind: ScopeNodeKind::Group,
            children,
            span,
            domain,
            verb_frame: false,
        };
        let Some(
            ScopeFrame::Root { children }
            | ScopeFrame::Construction { children, .. }
            | ScopeFrame::Edge { children, .. }
            | ScopeFrame::FrameRole { children, .. },
        ) = self.frames.last_mut()
        else {
            unreachable!("a role or Conjunct belongs to a construction")
        };
        children.push(ScopeChild {
            step: Some(step),
            mobile,
            node,
        });
    }
}

fn child_span(children: &[ScopeChild]) -> Option<(usize, usize)> {
    let mut spans = children.iter().filter_map(|child| child.node.span);
    let first = spans.next()?;
    Some(spans.fold(first, |span, next| (span.0.min(next.0), span.1.max(next.1))))
}

#[derive(Debug, Clone)]
struct ScopeProjection {
    root: ScopeNode,
}

impl ScopeProjection {
    fn of<R: GeneratedParseRoot>(value: &R) -> Self {
        let mut visitor = ScopeProjectionVisitor::default();
        visitor.frames.push(ScopeFrame::Root {
            children: Vec::new(),
        });
        value.visit_scope(&mut visitor);
        visitor.finish()
    }

    fn pruned(&self, affected: &HashSet<usize>, erase_all: bool) -> Vec<ScopeKey> {
        prune_scope_node(&self.root, affected, erase_all)
    }

    fn bucket_skeleton(&self, mobiles: &[MobileOccurrence]) -> Vec<ScopeKey> {
        let domains = mobiles
            .iter()
            .map(|mobile| mobile.domain)
            .collect::<HashSet<_>>();
        prune_scope_node_in_domains(&self.root, &HashSet::new(), Some(&domains))
    }

    fn construction_paths(&self) -> HashMap<usize, ConstructionPath> {
        let mut paths = HashMap::new();
        collect_construction_paths(&self.root, &[], &[], &[], &mut paths);
        paths
    }

    fn mobiles(&self) -> Vec<MobileOccurrence> {
        let paths = self.construction_paths();
        let mut mobiles = Vec::new();
        collect_mobiles(&self.root, None, None, &paths, &mut mobiles);
        mobiles.sort_by(|left, right| {
            left.signature
                .cmp(&right.signature)
                .then_with(|| left.host.skeleton.cmp(&right.host.skeleton))
                .then_with(|| left.role.cmp(right.role))
        });
        mobiles
    }

    fn coordinations(&self) -> Vec<CoordinationOccurrence> {
        let paths = self.construction_paths();
        let mut coordinations = Vec::new();
        collect_coordinations(&self.root, &paths, &mut coordinations);
        coordinations.sort_by(|left, right| left.anchor.cmp(&right.anchor));
        coordinations
    }

    fn leaf_sequence(&self) -> Vec<usize> {
        let mut leaves = Vec::new();
        collect_leaf_sequence(&self.root, &mut leaves);
        leaves
    }

    fn verb_frames(&self) -> Vec<&'static str> {
        let mut frames = Vec::new();
        collect_verb_frames(&self.root, &mut frames);
        frames
    }
}

fn collect_verb_frames(node: &ScopeNode, frames: &mut Vec<&'static str>) {
    if node.verb_frame {
        let ScopeNodeKind::Construction(identity, _) = node.kind else {
            unreachable!("only a construction can build a Verb Frame")
        };
        frames.push(identity);
    }
    for child in &node.children {
        collect_verb_frames(&child.node, frames);
    }
}

#[derive(Debug, Clone)]
struct CoordinationOccurrence {
    construction_index: usize,
    anchor: Vec<usize>,
    span: (usize, usize),
    sequence_role: &'static str,
    conjuncts: Vec<CoordinationConjunct>,
    path: ConstructionPath,
    domain: usize,
}

#[derive(Debug, Clone)]
struct CoordinationConjunct {
    ordinal: usize,
    span: Option<(usize, usize)>,
    node: ScopeNode,
}

fn collect_coordinations(
    node: &ScopeNode,
    paths: &HashMap<usize, ConstructionPath>,
    coordinations: &mut Vec<CoordinationOccurrence>,
) {
    if let ScopeNodeKind::Construction(_, construction_index) = node.kind
        && let Some((sequence_role, anchor, conjuncts)) = coordination_parts(node)
        && let Some(span) = coordination_yield_span(node)
    {
        coordinations.push(CoordinationOccurrence {
            construction_index,
            anchor,
            span,
            sequence_role,
            conjuncts,
            path: paths[&construction_index].clone(),
            domain: node.domain,
        });
    }
    for child in &node.children {
        collect_coordinations(&child.node, paths, coordinations);
    }
}

fn coordination_yield_span(node: &ScopeNode) -> Option<(usize, usize)> {
    fn collect(node: &ScopeNode, domain: usize, inside_conjunct: bool, leaves: &mut Vec<usize>) {
        if let ScopeNodeKind::Leaf(index) = node.kind {
            leaves.push(index);
        }
        for child in &node.children {
            if child.node.domain != domain {
                continue;
            }
            let child_inside_conjunct =
                inside_conjunct || matches!(child.step, Some(ScopeStep::Conjunct(_, _)));
            if child.mobile.is_some() && !child_inside_conjunct {
                continue;
            }
            collect(&child.node, domain, child_inside_conjunct, leaves);
        }
    }

    let mut leaves = Vec::new();
    collect(node, node.domain, false, &mut leaves);
    Some((*leaves.first()?, leaves.last()? + 1))
}

fn coordination_parts(
    node: &ScopeNode,
) -> Option<(&'static str, Vec<usize>, Vec<CoordinationConjunct>)> {
    enum Part {
        Constituent {
            role: Option<&'static str>,
            node: ScopeNode,
        },
        Coordinator(usize),
    }

    fn collect(node: &ScopeNode, domain: usize, parts: &mut Vec<Part>) {
        for child in &node.children {
            if child.node.domain != domain {
                continue;
            }
            if child.mobile.is_some() {
                continue;
            }
            if let Some(ScopeStep::Conjunct(role, _)) = child.step {
                parts.push(Part::Constituent {
                    role: Some(role),
                    node: child.node.clone(),
                });
                continue;
            }
            match child.node.kind {
                ScopeNodeKind::Leaf(index) => parts.push(Part::Coordinator(index)),
                ScopeNodeKind::Group => collect(&child.node, domain, parts),
                ScopeNodeKind::Construction(_, _) => parts.push(Part::Constituent {
                    role: None,
                    node: child.node.clone(),
                }),
            }
        }
    }

    let mut parts = Vec::new();
    collect(node, node.domain, &mut parts);
    let anchor = parts
        .iter()
        .filter_map(|part| match part {
            Part::Coordinator(index) => Some(*index),
            Part::Constituent { .. } => None,
        })
        .collect::<Vec<_>>();
    let role = parts.iter().find_map(|part| match part {
        Part::Constituent {
            role: Some(role), ..
        } => Some(*role),
        Part::Constituent { role: None, .. } | Part::Coordinator(_) => None,
    })?;
    let conjuncts = parts
        .into_iter()
        .filter_map(|part| match part {
            Part::Constituent { node, .. } => Some(node),
            Part::Coordinator(_) => None,
        })
        .enumerate()
        .map(|(ordinal, node)| CoordinationConjunct {
            ordinal,
            span: node.span,
            node,
        })
        .collect::<Vec<_>>();
    if anchor.is_empty() || conjuncts.len() != anchor.len() + 1 {
        return None;
    }
    Some((role, anchor, conjuncts))
}

fn collect_leaf_sequence(node: &ScopeNode, leaves: &mut Vec<usize>) {
    if let ScopeNodeKind::Leaf(index) = node.kind {
        leaves.push(index);
    }
    for child in &node.children {
        collect_leaf_sequence(&child.node, leaves);
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum ScopeKey {
    Construction {
        span: Option<(usize, usize)>,
        identity: Option<&'static str>,
        children: Vec<ScopeKey>,
    },
    Leaf(usize),
}

fn prune_scope_node(node: &ScopeNode, affected: &HashSet<usize>, erase_all: bool) -> Vec<ScopeKey> {
    let erased_domains = erase_all.then(|| HashSet::from([node.domain]));
    prune_scope_node_in_domains(node, affected, erased_domains.as_ref())
}

fn prune_scope_node_in_domains(
    node: &ScopeNode,
    affected: &HashSet<usize>,
    erased_domains: Option<&HashSet<usize>>,
) -> Vec<ScopeKey> {
    match node.kind {
        ScopeNodeKind::Leaf(index) => vec![ScopeKey::Leaf(index)],
        ScopeNodeKind::Group => node
            .children
            .iter()
            .filter(|child| child.mobile.is_none())
            .flat_map(|child| prune_scope_node_in_domains(&child.node, affected, erased_domains))
            .collect(),
        ScopeNodeKind::Construction(identity, index) => {
            let mut children = Vec::new();
            for child in node.children.iter().filter(|child| child.mobile.is_none()) {
                children.extend(prune_scope_node_in_domains(
                    &child.node,
                    affected,
                    erased_domains,
                ));
            }
            let span = key_span(&children);
            let erase_identity = affected.contains(&index)
                || erased_domains.is_some_and(|domains| domains.contains(&node.domain));
            let identity = (!erase_identity).then_some(identity);
            if identity.is_none() && children.len() == 1 {
                return children;
            }
            vec![ScopeKey::Construction {
                span,
                identity,
                children,
            }]
        }
    }
}

fn prune_scope_node_for_move_s(
    node: &ScopeNode,
    affected: &HashSet<usize>,
    regions: &[ScopeRegion],
) -> Vec<ScopeKey> {
    match node.kind {
        ScopeNodeKind::Leaf(index) => vec![ScopeKey::Leaf(index)],
        ScopeNodeKind::Group => node
            .children
            .iter()
            .filter(|child| {
                child.mobile.is_none()
                    || child.node.span.is_some_and(|span| {
                        regions.iter().any(|region| {
                            region.domain == child.node.domain
                                && interval_contains(region.span, span)
                        })
                    })
            })
            .flat_map(|child| prune_scope_node_for_move_s(&child.node, affected, regions))
            .collect(),
        ScopeNodeKind::Construction(identity, index) => {
            let children = node
                .children
                .iter()
                .filter(|child| {
                    child.mobile.is_none()
                        || child.node.span.is_some_and(|span| {
                            regions.iter().any(|region| {
                                region.domain == child.node.domain
                                    && interval_contains(region.span, span)
                            })
                        })
                })
                .flat_map(|child| prune_scope_node_for_move_s(&child.node, affected, regions))
                .collect::<Vec<_>>();
            if affected.contains(&index) {
                return children;
            }
            vec![ScopeKey::Construction {
                span: key_span(&children),
                identity: Some(identity),
                children,
            }]
        }
    }
}

fn key_span(children: &[ScopeKey]) -> Option<(usize, usize)> {
    fn span(key: &ScopeKey) -> Option<(usize, usize)> {
        match key {
            ScopeKey::Leaf(index) => Some((*index, *index + 1)),
            ScopeKey::Construction { span, .. } => *span,
        }
    }
    let mut spans = children.iter().filter_map(span);
    let first = spans.next()?;
    Some(spans.fold(first, |current, next| {
        (current.0.min(next.0), current.1.max(next.1))
    }))
}

#[derive(Debug, Clone)]
struct ConstructionPath {
    identity: &'static str,
    skeleton: Vec<usize>,
    attachment: Vec<ScopeStep>,
    anchor: Vec<ScopeStep>,
    domain: usize,
}

fn collect_construction_paths(
    node: &ScopeNode,
    skeleton: &[usize],
    attachment: &[ScopeStep],
    anchor: &[ScopeStep],
    paths: &mut HashMap<usize, ConstructionPath>,
) {
    if let ScopeNodeKind::Construction(identity, index) = node.kind {
        paths.insert(
            index,
            ConstructionPath {
                identity,
                skeleton: skeleton.to_vec(),
                attachment: attachment.to_vec(),
                anchor: anchor.to_vec(),
                domain: node.domain,
            },
        );
    }
    let mut child_index = 0;
    for child in &node.children {
        let mut child_attachment = attachment.to_vec();
        if let Some(step) = child.step {
            child_attachment.push(step);
        }
        let (child_skeleton, child_anchor) = if child.mobile.is_some() {
            (Vec::new(), child_attachment.clone())
        } else {
            let mut child_skeleton = skeleton.to_vec();
            if matches!(node.kind, ScopeNodeKind::Construction(_, _)) {
                child_skeleton.push(child_index);
                child_index += prune_scope_node(&child.node, &HashSet::new(), true).len();
            }
            (child_skeleton, anchor.to_vec())
        };
        collect_construction_paths(
            &child.node,
            &child_skeleton,
            &child_attachment,
            &child_anchor,
            paths,
        );
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct MobileSignature {
    span: Option<(usize, usize)>,
    subtree: Vec<ScopeKey>,
}

#[derive(Debug, Clone)]
struct MobileOccurrence {
    signature: MobileSignature,
    host_index: usize,
    host: ConstructionPath,
    role: &'static str,
    scope_sibling: Option<&'static str>,
    scope_region: Option<MobileSignature>,
    subtree: ScopeNode,
    scope_subtree: Option<ScopeNode>,
    first_conjunct_path: Option<Vec<ScopeStep>>,
    domain: usize,
}

fn collect_mobiles(
    node: &ScopeNode,
    host: Option<usize>,
    scope_region: Option<&MobileSignature>,
    paths: &HashMap<usize, ConstructionPath>,
    mobiles: &mut Vec<MobileOccurrence>,
) {
    let host = match node.kind {
        ScopeNodeKind::Construction(_, index) => Some(index),
        ScopeNodeKind::Group | ScopeNodeKind::Leaf(_) => host,
    };
    for (child_index, child) in node.children.iter().enumerate() {
        let mut child_region = scope_region.cloned();
        if let Some(mobile) = child.mobile {
            let host = host.expect("a mobile role belongs to a construction");
            let subtree = prune_scope_node(&child.node, &HashSet::new(), true);
            let signature = MobileSignature {
                span: key_span(&subtree),
                subtree,
            };
            let scope_child = mobile.scope_sibling.and_then(|scope_sibling| {
                node.children
                    .iter()
                    .enumerate()
                    .find(|(_, candidate)| candidate.step == Some(ScopeStep::Role(scope_sibling)))
            });
            let first_conjunct_path = scope_child.and_then(|scope_child| {
                let (scope_index, scope_child) = scope_child;
                let mut path = vec![scope_child.step.expect("a scope sibling is a role")];
                if child_index < scope_index {
                    find_first_conjunct(&scope_child.node, &mut path)
                } else {
                    find_last_conjunct(&scope_child.node, &mut path)
                }
            });
            mobiles.push(MobileOccurrence {
                signature: signature.clone(),
                host_index: host,
                host: paths[&host].clone(),
                role: mobile.role,
                scope_sibling: mobile.scope_sibling,
                scope_region: scope_region.cloned(),
                subtree: child.node.clone(),
                scope_subtree: scope_child.map(|(_, scope_child)| scope_child.node.clone()),
                first_conjunct_path,
                domain: child.node.domain,
            });
            child_region = Some(signature);
        }
        collect_mobiles(&child.node, host, child_region.as_ref(), paths, mobiles);
    }
}

fn find_first_conjunct(node: &ScopeNode, path: &mut Vec<ScopeStep>) -> Option<Vec<ScopeStep>> {
    for child in &node.children {
        if let Some(step) = child.step {
            path.push(step);
        }
        if matches!(child.step, Some(ScopeStep::Conjunct(_, 0))) {
            return Some(path.clone());
        }
        if let Some(found) = find_first_conjunct(&child.node, path) {
            return Some(found);
        }
        if child.step.is_some() {
            path.pop();
        }
    }
    None
}

fn find_last_conjunct(node: &ScopeNode, path: &mut Vec<ScopeStep>) -> Option<Vec<ScopeStep>> {
    for child in node.children.iter().rev() {
        if let Some(step) = child.step {
            path.push(step);
        }
        if matches!(child.step, Some(ScopeStep::Conjunct(_, _))) {
            return Some(path.clone());
        }
        if let Some(found) = find_last_conjunct(&child.node, path) {
            return Some(found);
        }
        if child.step.is_some() {
            path.pop();
        }
    }
    None
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct ScopeBucketKey(u64);

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct CoordinationBucketKey(u64);

fn structural_digest(value: &impl Hash) -> u64 {
    let mut digest = DefaultHasher::new();
    value.hash(&mut digest);
    digest.finish()
}

struct RenderedCandidate {
    text: String,
    claims: Vec<RawRenderedClaim>,
}

struct ScopeCandidateCaches {
    projections: Vec<OnceCell<ScopeProjection>>,
    mobiles: Vec<OnceCell<Vec<MobileOccurrence>>>,
    rendered: Vec<OnceCell<RenderedCandidate>>,
}

#[derive(Default)]
struct MobileInventoryVisitor {
    carries_mobile: bool,
    pending_frame_role: Option<&'static str>,
}

impl Visitor for MobileInventoryVisitor {
    fn enter_construction(&mut self, _construction: &'static str) {
        self.pending_frame_role = None;
    }

    fn enter_frame_role(&mut self, role: &'static str) {
        self.pending_frame_role = Some(role);
    }

    fn exit_frame_role(&mut self) {
        self.pending_frame_role = None;
    }

    fn enter_role(
        &mut self,
        role: &'static str,
        _scope_sibling: Option<&'static str>,
        admissible_sites: Option<&AdmissibleSites>,
    ) {
        let fills_frame_role = self.pending_frame_role.take() == Some(role);
        if !fills_frame_role && admissible_sites.is_some() {
            self.carries_mobile = true;
        }
    }

    fn enter_conjunct(&mut self, _role: &'static str, _ordinal: usize) {
        self.pending_frame_role = None;
    }
}

fn candidate_carries_mobile<R: GeneratedParseRoot>(candidate: &Candidate<R>) -> bool {
    let mut visitor = MobileInventoryVisitor::default();
    candidate.value.visit_scope(&mut visitor);
    visitor.carries_mobile
}

fn collapse_scope_candidates<R: GeneratedParseRoot>(
    candidates: Vec<Candidate<R>>,
    context: &ParseContext<'_>,
    environment: &crate::environment::ParserEnvironment,
) -> Vec<Candidate<R>> {
    if candidates.len() < 2 {
        return candidates;
    }
    let carries_mobile = candidates
        .iter()
        .map(candidate_carries_mobile)
        .collect::<Vec<_>>();
    if carries_mobile
        .iter()
        .filter(|&&carries| carries)
        .take(2)
        .count()
        < 2
    {
        return candidates;
    }
    let caches = ScopeCandidateCaches {
        projections: (0..candidates.len()).map(|_| OnceCell::new()).collect(),
        mobiles: (0..candidates.len()).map(|_| OnceCell::new()).collect(),
        rendered: (0..candidates.len()).map(|_| OnceCell::new()).collect(),
    };
    let mut buckets = HashMap::<ScopeBucketKey, Vec<usize>>::new();
    let mut coordination_buckets = HashMap::<CoordinationBucketKey, Vec<usize>>::new();
    for (index, candidate) in candidates.iter().enumerate() {
        if !carries_mobile[index] {
            continue;
        }
        let projection =
            caches.projections[index].get_or_init(|| ScopeProjection::of(&candidate.value));
        let candidate_mobiles = caches.mobiles[index].get_or_init(|| projection.mobiles());
        let mut signatures = candidate_mobiles
            .iter()
            .map(|mobile| mobile.signature.clone())
            .collect::<Vec<_>>();
        signatures.sort();
        if !signatures.is_empty() {
            buckets
                .entry(ScopeBucketKey(structural_digest(&(
                    signatures,
                    projection.bucket_skeleton(candidate_mobiles),
                ))))
                .or_default()
                .push(index);
        }
        let coordinations = projection.coordinations();
        if !coordinations.is_empty() {
            let anchors = coordinations
                .into_iter()
                .map(|coordination| coordination.anchor)
                .collect::<Vec<_>>();
            coordination_buckets
                .entry(CoordinationBucketKey(structural_digest(&(
                    projection.leaf_sequence(),
                    anchors,
                ))))
                .or_default()
                .push(index);
        }
    }
    let mut adjacent = HashMap::<usize, Vec<usize>>::new();
    let mut compared = HashSet::new();
    for bucket in buckets
        .values()
        .chain(coordination_buckets.values())
        .filter(|bucket| bucket.len() > 1)
    {
        for (offset, &left) in bucket.iter().enumerate() {
            for &right in &bucket[offset + 1..] {
                let pair = (left.min(right), left.max(right));
                if !compared.insert(pair) {
                    continue;
                }
                let verified = verify_scope_pair_cached(
                    left,
                    right,
                    &candidates,
                    &caches,
                    context,
                    environment,
                );
                if verified {
                    adjacent.entry(left).or_default().push(right);
                    adjacent.entry(right).or_default().push(left);
                }
            }
        }
    }
    let mut keep = vec![true; candidates.len()];
    let mut visited = HashSet::new();
    for start in 0..candidates.len() {
        if !visited.insert(start) {
            continue;
        }
        let mut component = vec![start];
        let mut pending = vec![start];
        while let Some(index) = pending.pop() {
            for &next in adjacent.get(&index).into_iter().flatten() {
                if visited.insert(next) {
                    component.push(next);
                }
            }
        }
        if component.len() > 1 {
            collapse_scope_component(
                &component,
                &adjacent,
                &candidates,
                &caches.projections,
                &caches.mobiles,
                &mut keep,
            );
        }
    }
    candidates
        .into_iter()
        .enumerate()
        .filter_map(|(index, candidate)| keep[index].then_some(candidate))
        .collect()
}

#[cfg(test)]
fn verify_scope_pair<R>(
    left: usize,
    right: usize,
    candidates: &[Candidate<R>],
    projections: &[ScopeProjection],
    mobiles: &[Vec<MobileOccurrence>],
    rendered: &[RenderedCandidate],
) -> bool {
    let left = ScopePairSide {
        candidate: &candidates[left],
        projection: &projections[left],
        mobiles: &mobiles[left],
        rendered: &rendered[left],
    };
    let right = ScopePairSide {
        candidate: &candidates[right],
        projection: &projections[right],
        mobiles: &mobiles[right],
        rendered: &rendered[right],
    };
    verify_scope_pair_parts(&left, &right)
}

fn verify_scope_pair_cached<R: GeneratedParseRoot>(
    left: usize,
    right: usize,
    candidates: &[Candidate<R>],
    caches: &ScopeCandidateCaches,
    context: &ParseContext<'_>,
    environment: &crate::environment::ParserEnvironment,
) -> bool {
    let left_projection = caches.projections[left]
        .get()
        .expect("a bucket member has a scope projection");
    let right_projection = caches.projections[right]
        .get()
        .expect("a bucket member has a scope projection");
    if left_projection.verb_frames() != right_projection.verb_frames()
        || candidates[left].synthetic_claims != candidates[right].synthetic_claims
    {
        return false;
    }
    let left_rendered = caches.rendered[left].get_or_init(|| {
        let (text, claims) = candidates[left]
            .value
            .render_with_claims(context, environment);
        RenderedCandidate { text, claims }
    });
    let right_rendered = caches.rendered[right].get_or_init(|| {
        let (text, claims) = candidates[right]
            .value
            .render_with_claims(context, environment);
        RenderedCandidate { text, claims }
    });
    let left = ScopePairSide {
        candidate: &candidates[left],
        projection: left_projection,
        mobiles: caches.mobiles[left]
            .get()
            .expect("a bucket member has a mobile inventory"),
        rendered: left_rendered,
    };
    let right = ScopePairSide {
        candidate: &candidates[right],
        projection: right_projection,
        mobiles: caches.mobiles[right]
            .get()
            .expect("a bucket member has a mobile inventory"),
        rendered: right_rendered,
    };
    verify_scope_pair_parts(&left, &right)
}

struct ScopePairSide<'a, R> {
    candidate: &'a Candidate<R>,
    projection: &'a ScopeProjection,
    mobiles: &'a [MobileOccurrence],
    rendered: &'a RenderedCandidate,
}

fn verify_scope_pair_parts<R>(left: &ScopePairSide<'_, R>, right: &ScopePairSide<'_, R>) -> bool {
    if left.projection.verb_frames() != right.projection.verb_frames()
        || left.candidate.synthetic_claims != right.candidate.synthetic_claims
        || left.rendered.text != right.rendered.text
    {
        return false;
    }
    if same_claimed_leaves(&left.candidate.claims, &right.candidate.claims)
        && verify_scope_pair_with_erasure(
            ScopeVerificationSide::ordinary(left.projection, left.mobiles),
            ScopeVerificationSide::ordinary(right.projection, right.mobiles),
            false,
        )
    {
        return true;
    }
    verify_coordination_scope_pair(left, right)
}

struct ScopeVerificationSide<'a> {
    projection: &'a ScopeProjection,
    all_mobiles: &'a [MobileOccurrence],
    mobiles: Vec<&'a MobileOccurrence>,
    affected: HashSet<usize>,
    regions: Option<&'a [ScopeRegion]>,
}

impl<'a> ScopeVerificationSide<'a> {
    fn ordinary(projection: &'a ScopeProjection, mobiles: &'a [MobileOccurrence]) -> Self {
        Self {
            projection,
            all_mobiles: mobiles,
            mobiles: mobiles.iter().collect(),
            affected: HashSet::new(),
            regions: None,
        }
    }
}

fn verify_scope_pair_with_erasure(
    mut left: ScopeVerificationSide<'_>,
    mut right: ScopeVerificationSide<'_>,
    mut moved: bool,
) -> bool {
    if left.mobiles.len() != right.mobiles.len()
        || left
            .mobiles
            .iter()
            .zip(&right.mobiles)
            .any(|(left, right)| left.signature != right.signature)
    {
        return false;
    }
    if left
        .mobiles
        .iter()
        .zip(&right.mobiles)
        .any(|(left, right)| left.domain != right.domain)
    {
        return false;
    }
    for (left_mobile, right_mobile) in left.mobiles.iter().zip(&right.mobiles) {
        if left_mobile.role == right_mobile.role
            && left_mobile.host.attachment == right_mobile.host.attachment
            && left.affected.contains(&left_mobile.host_index)
            && right.affected.contains(&right_mobile.host_index)
        {
            continue;
        }
        if left_mobile.host.identity == right_mobile.host.identity {
            if left_mobile.role != right_mobile.role {
                return false;
            }
            if left_mobile.scope_sibling == right_mobile.scope_sibling
                && left_mobile.scope_sibling.is_some()
                && (left_mobile.first_conjunct_path.is_some()
                    || right_mobile.first_conjunct_path.is_some())
            {
                let left_region = coordination_region(left.projection, left_mobile);
                let right_region = coordination_region(right.projection, right_mobile);
                if let (Some(left_region), Some(right_region)) = (left_region, right_region)
                    && (left_mobile.first_conjunct_path != right_mobile.first_conjunct_path
                        || prune_scope_node(left_region, &HashSet::new(), false)
                            != prune_scope_node(right_region, &HashSet::new(), false))
                {
                    moved = true;
                    mark_scope_region(left.projection, left_region, &mut left.affected);
                    mark_scope_region(right.projection, right_region, &mut right.affected);
                }
            } else if left_mobile.scope_sibling.is_none()
                && left_mobile.host.attachment != right_mobile.host.attachment
            {
                let ordered =
                    is_step_prefix(&left_mobile.host.attachment, &right_mobile.host.attachment)
                        || is_step_prefix(
                            &right_mobile.host.attachment,
                            &left_mobile.host.attachment,
                        );
                if ordered
                    && (move_is_licensed(left_mobile, right_mobile)
                        || move_is_licensed(right_mobile, left_mobile))
                {
                    moved = true;
                    left.affected.insert(left_mobile.host_index);
                    right.affected.insert(right_mobile.host_index);
                }
            } else if left_mobile.scope_sibling.is_none()
                && left_mobile.host.attachment == right_mobile.host.attachment
            {
                let left_host = find_construction(&left.projection.root, left_mobile.host_index)
                    .expect("a mobile host remains in its projection");
                let right_host = find_construction(&right.projection.root, right_mobile.host_index)
                    .expect("a mobile host remains in its projection");
                let before = (left.affected.len(), right.affected.len());
                if mark_identity_differences(
                    left_host,
                    right_host,
                    &mut left.affected,
                    &mut right.affected,
                ) && before != (left.affected.len(), right.affected.len())
                {
                    moved = true;
                }
            }
            continue;
        }
        moved = true;
        let (high, low) =
            if is_step_prefix(&left_mobile.host.attachment, &right_mobile.host.attachment) {
                (left_mobile, right_mobile)
            } else if is_step_prefix(&right_mobile.host.attachment, &left_mobile.host.attachment) {
                (right_mobile, left_mobile)
            } else {
                return false;
            };
        if !move_is_licensed(high, low) {
            return false;
        }
        for (scope_region, skeleton) in [
            (&left_mobile.scope_region, &left_mobile.host.skeleton),
            (&right_mobile.scope_region, &right_mobile.host.skeleton),
        ] {
            if !mark_identity_difference_at(
                (left.projection, left.all_mobiles),
                (right.projection, right.all_mobiles),
                scope_region.as_ref(),
                skeleton,
                &mut left.affected,
                &mut right.affected,
            ) {
                return false;
            }
        }
    }
    if !moved {
        return false;
    }
    let structures_match = if let (Some(left_regions), Some(right_regions)) =
        (left.regions, right.regions)
    {
        prune_scope_node_for_move_s(&left.projection.root, &left.affected, left_regions)
            == prune_scope_node_for_move_s(&right.projection.root, &right.affected, right_regions)
    } else {
        left.projection.pruned(&left.affected, false)
            == right.projection.pruned(&right.affected, false)
    };
    if !structures_match {
        return false;
    }
    for (left_mobile, right_mobile) in left.mobiles.iter().zip(&right.mobiles) {
        if mobile_identity_key(left_mobile) != mobile_identity_key(right_mobile) {
            return false;
        }
    }
    true
}

#[derive(Debug)]
struct CoordinationScopeComparison {
    left_affected: HashSet<usize>,
    right_affected: HashSet<usize>,
    left_regions: Vec<ScopeRegion>,
    right_regions: Vec<ScopeRegion>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct ScopeRegion {
    domain: usize,
    span: (usize, usize),
}

fn verify_coordination_scope_pair<R>(
    left: &ScopePairSide<'_, R>,
    right: &ScopePairSide<'_, R>,
) -> bool {
    let Some(comparison) = coordination_scope_comparison(
        left.projection,
        left.mobiles,
        right.projection,
        right.mobiles,
    ) else {
        return false;
    };
    let Some(left_claim_regions) =
        rendered_text_regions(&left.rendered.claims, &comparison.left_regions)
    else {
        return false;
    };
    let Some(right_claim_regions) =
        rendered_text_regions(&right.rendered.claims, &comparison.right_regions)
    else {
        return false;
    };
    if !same_claimed_leaves_in_regions(
        &left.candidate.claims,
        &right.candidate.claims,
        &left.rendered.text,
        &right.rendered.text,
        &left_claim_regions,
        &right_claim_regions,
    ) {
        return false;
    }
    let left_mobiles_outside = left
        .mobiles
        .iter()
        .filter(|mobile| !mobile_within_regions(mobile, &comparison.left_regions))
        .collect::<Vec<_>>();
    let right_mobiles_outside = right
        .mobiles
        .iter()
        .filter(|mobile| !mobile_within_regions(mobile, &comparison.right_regions))
        .collect::<Vec<_>>();
    verify_scope_pair_with_erasure(
        ScopeVerificationSide {
            projection: left.projection,
            all_mobiles: left.mobiles,
            mobiles: left_mobiles_outside,
            affected: comparison.left_affected,
            regions: Some(&comparison.left_regions),
        },
        ScopeVerificationSide {
            projection: right.projection,
            all_mobiles: right.mobiles,
            mobiles: right_mobiles_outside,
            affected: comparison.right_affected,
            regions: Some(&comparison.right_regions),
        },
        true,
    )
}

fn coordination_scope_comparison(
    left_projection: &ScopeProjection,
    left_mobiles: &[MobileOccurrence],
    right_projection: &ScopeProjection,
    right_mobiles: &[MobileOccurrence],
) -> Option<CoordinationScopeComparison> {
    let left_coordinations = left_projection.coordinations();
    let right_coordinations = right_projection.coordinations();
    if left_coordinations.is_empty() || left_coordinations.len() != right_coordinations.len() {
        return None;
    }
    if left_coordinations
        .windows(2)
        .any(|pair| pair[0].anchor == pair[1].anchor)
        || right_coordinations
            .windows(2)
            .any(|pair| pair[0].anchor == pair[1].anchor)
    {
        return None;
    }
    let mut left_affected = HashSet::new();
    let mut right_affected = HashSet::new();
    let mut left_regions = Vec::new();
    let mut right_regions = Vec::new();
    let mut moved = false;
    let mut declared_mobile_difference = false;
    for (left_coordination, right_coordination) in
        left_coordinations.iter().zip(&right_coordinations)
    {
        if left_coordination.anchor != right_coordination.anchor
            || left_coordination.domain != right_coordination.domain
        {
            return None;
        }
        let (left_span, right_span) = coordination_comparison_spans(
            left_projection,
            left_mobiles,
            left_coordination,
            right_projection,
            right_mobiles,
            right_coordination,
        );
        let differences = interval_symmetric_difference(left_span, right_span);
        if differences.is_empty() {
            continue;
        }
        if !coordination_difference_is_peripheral(left_span, right_span, &differences) {
            return None;
        }
        declared_mobile_difference |= differences.iter().any(|difference| {
            difference_is_declared_mobile_yield(*difference, left_mobiles)
                || difference_is_declared_mobile_yield(*difference, right_mobiles)
        });
        moved = true;
        mark_coordination_identity(left_projection, left_coordination, &mut left_affected);
        mark_coordination_identity(right_projection, right_coordination, &mut right_affected);
        let intersection = (left_span.0.max(right_span.0), left_span.1.min(right_span.1));
        let touches_prefix = differences
            .iter()
            .any(|difference| difference.1 == intersection.0);
        let touches_suffix = differences
            .iter()
            .any(|difference| difference.0 == intersection.1);
        mark_touched_conjuncts(
            left_coordination,
            &differences,
            touches_prefix,
            touches_suffix,
            &mut left_affected,
        );
        mark_touched_conjuncts(
            right_coordination,
            &differences,
            touches_prefix,
            touches_suffix,
            &mut right_affected,
        );
        let scope_region = (left_span.0.min(right_span.0), left_span.1.max(right_span.1));
        mark_partition_structure(
            &left_projection.root,
            scope_region,
            left_coordination,
            touches_prefix,
            touches_suffix,
            &mut left_affected,
        );
        mark_partition_structure(
            &right_projection.root,
            scope_region,
            right_coordination,
            touches_prefix,
            touches_suffix,
            &mut right_affected,
        );
        left_regions.push(ScopeRegion {
            domain: left_coordination.domain,
            span: scope_region,
        });
        right_regions.push(ScopeRegion {
            domain: right_coordination.domain,
            span: scope_region,
        });
    }
    (moved && declared_mobile_difference).then_some(CoordinationScopeComparison {
        left_affected,
        right_affected,
        left_regions: merge_scope_regions(left_regions),
        right_regions: merge_scope_regions(right_regions),
    })
}

fn difference_is_declared_mobile_yield(
    difference: (usize, usize),
    mobiles: &[MobileOccurrence],
) -> bool {
    merge_intervals(
        mobiles
            .iter()
            .filter_map(|mobile| mobile.signature.span)
            .filter(|span| interval_contains(difference, *span))
            .collect(),
    )
    .contains(&difference)
}

fn coordination_comparison_spans(
    left_projection: &ScopeProjection,
    left_mobiles: &[MobileOccurrence],
    left_coordination: &CoordinationOccurrence,
    right_projection: &ScopeProjection,
    right_mobiles: &[MobileOccurrence],
    right_coordination: &CoordinationOccurrence,
) -> ((usize, usize), (usize, usize)) {
    if left_coordination.span != right_coordination.span {
        return (left_coordination.span, right_coordination.span);
    }
    // A shared constituent can sit above an otherwise identical Coordination
    // yield. The highest related mobile is the candidate-specific boundary of
    // that Coordination; lower mobiles describe partitions inside that bound.
    (
        coordination_span_with_highest_mobile(left_projection, left_mobiles, left_coordination),
        coordination_span_with_highest_mobile(right_projection, right_mobiles, right_coordination),
    )
}

fn coordination_span_with_highest_mobile(
    projection: &ScopeProjection,
    mobiles: &[MobileOccurrence],
    coordination: &CoordinationOccurrence,
) -> (usize, usize) {
    let related = mobiles.iter().filter(|mobile| {
        mobile.domain == coordination.domain
            && (is_step_prefix(&mobile.host.attachment, &coordination.path.attachment)
                || is_step_prefix(&coordination.path.attachment, &mobile.host.attachment))
            && coordination_region(projection, mobile).is_some_and(|region| {
                contains_construction(region, coordination.construction_index)
            })
    });
    let Some(highest_depth) = related
        .clone()
        .map(|mobile| mobile.host.attachment.len())
        .min()
    else {
        return coordination.span;
    };
    related
        .filter(|mobile| mobile.host.attachment.len() == highest_depth)
        .filter_map(|mobile| mobile.signature.span)
        .fold(coordination.span, |span, mobile_span| {
            (span.0.min(mobile_span.0), span.1.max(mobile_span.1))
        })
}

fn mark_coordination_identity(
    projection: &ScopeProjection,
    coordination: &CoordinationOccurrence,
    affected: &mut HashSet<usize>,
) {
    affected.insert(coordination.construction_index);
    let node = find_construction(&projection.root, coordination.construction_index)
        .expect("a Coordination remains in its projection");
    let span = key_span(&prune_scope_node(node, &HashSet::new(), true));
    mark_same_span_ancestors(
        &projection.root,
        coordination.construction_index,
        span,
        coordination.domain,
        affected,
    );
}

fn interval_symmetric_difference(
    left: (usize, usize),
    right: (usize, usize),
) -> Vec<(usize, usize)> {
    if left == right {
        return Vec::new();
    }
    let intersection = (left.0.max(right.0), left.1.min(right.1));
    if intersection.0 >= intersection.1 {
        return vec![left, right];
    }
    let mut difference = Vec::new();
    let left_edge = (left.0.min(right.0), intersection.0);
    if left_edge.0 < left_edge.1 {
        difference.push(left_edge);
    }
    let right_edge = (intersection.1, left.1.max(right.1));
    if right_edge.0 < right_edge.1 {
        difference.push(right_edge);
    }
    difference
}

fn coordination_difference_is_peripheral(
    left: (usize, usize),
    right: (usize, usize),
    differences: &[(usize, usize)],
) -> bool {
    if differences.len() > 2 {
        return false;
    }
    let intersection = (left.0.max(right.0), left.1.min(right.1));
    intersection.0 < intersection.1
        && differences
            .iter()
            .all(|difference| difference.1 == intersection.0 || difference.0 == intersection.1)
}

fn mark_touched_conjuncts(
    coordination: &CoordinationOccurrence,
    differences: &[(usize, usize)],
    touches_prefix: bool,
    touches_suffix: bool,
    affected: &mut HashSet<usize>,
) {
    for (index, conjunct) in coordination.conjuncts.iter().enumerate() {
        if conjunct.span.is_some_and(|span| {
            differences.iter().any(|difference| {
                intervals_intersect(span, *difference)
                    || (index == 0 && touches_prefix)
                    || (index + 1 == coordination.conjuncts.len() && touches_suffix)
            })
        }) {
            mark_construction_subtree(&conjunct.node, coordination.domain, affected);
        }
    }
}

fn mark_partition_structure(
    node: &ScopeNode,
    scope_region: (usize, usize),
    coordination: &CoordinationOccurrence,
    touches_prefix: bool,
    touches_suffix: bool,
    affected: &mut HashSet<usize>,
) {
    let untouched_conjuncts = coordination
        .conjuncts
        .iter()
        .enumerate()
        .filter(|(index, _)| {
            !(*index == 0 && touches_prefix
                || *index + 1 == coordination.conjuncts.len() && touches_suffix)
        })
        .filter_map(|(_, conjunct)| conjunct.span)
        .collect::<Vec<_>>();
    mark_partition_structure_inner(
        node,
        scope_region,
        &untouched_conjuncts,
        coordination.domain,
        affected,
    );
}

fn mark_partition_structure_inner(
    node: &ScopeNode,
    scope_region: (usize, usize),
    untouched_conjuncts: &[(usize, usize)],
    domain: usize,
    affected: &mut HashSet<usize>,
) {
    if let ScopeNodeKind::Construction(_, index) = node.kind
        && node.domain == domain
        && node.span.is_some_and(|span| {
            interval_contains(scope_region, span)
                && !untouched_conjuncts
                    .iter()
                    .any(|conjunct| interval_contains(*conjunct, span))
        })
    {
        affected.insert(index);
    }
    for child in &node.children {
        mark_partition_structure_inner(
            &child.node,
            scope_region,
            untouched_conjuncts,
            domain,
            affected,
        );
    }
}

fn intervals_intersect(left: (usize, usize), right: (usize, usize)) -> bool {
    left.0 < right.1 && right.0 < left.1
}

fn interval_contains(outer: (usize, usize), inner: (usize, usize)) -> bool {
    outer.0 <= inner.0 && inner.1 <= outer.1
}

fn merge_intervals(mut intervals: Vec<(usize, usize)>) -> Vec<(usize, usize)> {
    intervals.sort_unstable();
    let mut merged: Vec<(usize, usize)> = Vec::new();
    for interval in intervals {
        if let Some(last) = merged.last_mut()
            && interval.0 <= last.1
        {
            last.1 = last.1.max(interval.1);
        } else {
            merged.push(interval);
        }
    }
    merged
}

fn merge_scope_regions(mut regions: Vec<ScopeRegion>) -> Vec<ScopeRegion> {
    regions.sort_unstable();
    let mut merged: Vec<ScopeRegion> = Vec::new();
    for region in regions {
        if let Some(last) = merged.last_mut()
            && last.domain == region.domain
            && region.span.0 <= last.span.1
        {
            last.span.1 = last.span.1.max(region.span.1);
        } else {
            merged.push(region);
        }
    }
    merged
}

fn mobile_within_regions(mobile: &MobileOccurrence, regions: &[ScopeRegion]) -> bool {
    mobile.signature.span.is_some_and(|span| {
        regions
            .iter()
            .any(|region| region.domain == mobile.domain && interval_contains(region.span, span))
    })
}

fn rendered_text_regions(
    claims: &[RawRenderedClaim],
    regions: &[ScopeRegion],
) -> Option<Vec<TextSpan>> {
    regions
        .iter()
        .map(|region| {
            let (start, end) = region.span;
            if start >= end {
                return None;
            }
            Some(TextSpan {
                start: claims.get(start)?.span.start,
                end: claims.get(end - 1)?.span.end,
            })
        })
        .collect()
}

fn same_claimed_leaves_in_regions(
    left: &[RawLexicalClaim],
    right: &[RawLexicalClaim],
    left_text: &str,
    right_text: &str,
    left_regions: &[TextSpan],
    right_regions: &[TextSpan],
) -> bool {
    left.len() == right.len()
        && left.iter().zip(right).all(|(left, right)| {
            let left_inside = left_regions
                .iter()
                .any(|region| text_span_contains(*region, left.span));
            let right_inside = right_regions
                .iter()
                .any(|region| text_span_contains(*region, right.span));
            if left_inside || right_inside {
                left_inside
                    && right_inside
                    && left_text[left.span.start..left.span.end].trim()
                        == right_text[right.span.start..right.span.end].trim()
            } else {
                left.span == right.span && left.value == right.value
            }
        })
}

fn text_span_contains(outer: TextSpan, inner: TextSpan) -> bool {
    outer.start <= inner.start && inner.end <= outer.end
}

fn mobile_identity_key(mobile: &MobileOccurrence) -> Vec<ScopeKey> {
    if contains_conjunct_in_domain(&mobile.subtree, mobile.domain) {
        return mobile.signature.subtree.clone();
    }
    let mut boundary = HashSet::new();
    mark_full_span_constructions(
        &mobile.subtree,
        mobile.signature.span,
        mobile.domain,
        &mut boundary,
    );
    prune_scope_node(&mobile.subtree, &boundary, false)
}

fn mark_full_span_constructions(
    node: &ScopeNode,
    span: Option<(usize, usize)>,
    domain: usize,
    affected: &mut HashSet<usize>,
) {
    if let ScopeNodeKind::Construction(_, index) = node.kind
        && node.domain == domain
        && key_span(&prune_scope_node(node, &HashSet::new(), true)) == span
    {
        affected.insert(index);
    }
    for child in &node.children {
        mark_full_span_constructions(&child.node, span, domain, affected);
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct IdentityShape {
    chain: Vec<(&'static str, usize)>,
    terminal: IdentityTerminal,
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum IdentityTerminal {
    Leaf(usize),
    Children(Vec<IdentityShape>),
    Opaque(Vec<ScopeKey>),
}

fn identity_shapes(node: &ScopeNode) -> Vec<IdentityShape> {
    identity_shapes_in_domain(node, node.domain)
}

fn identity_shapes_in_domain(node: &ScopeNode, domain: usize) -> Vec<IdentityShape> {
    if node.domain != domain {
        return vec![IdentityShape {
            chain: Vec::new(),
            terminal: IdentityTerminal::Opaque(prune_scope_node(node, &HashSet::new(), false)),
        }];
    }
    match node.kind {
        ScopeNodeKind::Leaf(index) => vec![IdentityShape {
            chain: Vec::new(),
            terminal: IdentityTerminal::Leaf(index),
        }],
        ScopeNodeKind::Group => node
            .children
            .iter()
            .filter(|child| child.mobile.is_none())
            .flat_map(|child| identity_shapes_in_domain(&child.node, domain))
            .collect(),
        ScopeNodeKind::Construction(identity, index) => {
            let mut children = node
                .children
                .iter()
                .filter(|child| child.mobile.is_none())
                .flat_map(|child| identity_shapes_in_domain(&child.node, domain))
                .collect::<Vec<_>>();
            if children.len() == 1 {
                let mut child = children.pop().expect("one projected child");
                child.chain.insert(0, (identity, index));
                vec![child]
            } else {
                vec![IdentityShape {
                    chain: vec![(identity, index)],
                    terminal: IdentityTerminal::Children(children),
                }]
            }
        }
    }
}

fn mark_identity_differences(
    left: &ScopeNode,
    right: &ScopeNode,
    left_affected: &mut HashSet<usize>,
    right_affected: &mut HashSet<usize>,
) -> bool {
    let left_shapes = identity_shapes(left);
    let right_shapes = identity_shapes(right);
    mark_shape_differences(&left_shapes, &right_shapes, left_affected, right_affected)
}

fn mark_shape_differences(
    left: &[IdentityShape],
    right: &[IdentityShape],
    left_affected: &mut HashSet<usize>,
    right_affected: &mut HashSet<usize>,
) -> bool {
    if left.len() != right.len() {
        return false;
    }
    for (left, right) in left.iter().zip(right) {
        let retained = common_identity_indices(&left.chain, &right.chain);
        for (ordinal, &(_, index)) in left.chain.iter().enumerate() {
            if !retained.0.contains(&ordinal) {
                left_affected.insert(index);
            }
        }
        for (ordinal, &(_, index)) in right.chain.iter().enumerate() {
            if !retained.1.contains(&ordinal) {
                right_affected.insert(index);
            }
        }
        match (&left.terminal, &right.terminal) {
            (IdentityTerminal::Leaf(left), IdentityTerminal::Leaf(right)) if left == right => {}
            (IdentityTerminal::Children(left), IdentityTerminal::Children(right)) => {
                if !mark_shape_differences(left, right, left_affected, right_affected) {
                    return false;
                }
            }
            (IdentityTerminal::Opaque(left), IdentityTerminal::Opaque(right)) if left == right => {}
            _ => return false,
        }
    }
    true
}

fn common_identity_indices(
    left: &[(&'static str, usize)],
    right: &[(&'static str, usize)],
) -> (HashSet<usize>, HashSet<usize>) {
    let mut lengths = vec![vec![0; right.len() + 1]; left.len() + 1];
    for left_index in (0..left.len()).rev() {
        for right_index in (0..right.len()).rev() {
            lengths[left_index][right_index] = if left[left_index].0 == right[right_index].0 {
                lengths[left_index + 1][right_index + 1] + 1
            } else {
                lengths[left_index + 1][right_index].max(lengths[left_index][right_index + 1])
            };
        }
    }
    let mut retained_left = HashSet::new();
    let mut retained_right = HashSet::new();
    let (mut left_index, mut right_index) = (0, 0);
    while left_index < left.len() && right_index < right.len() {
        if left[left_index].0 == right[right_index].0 {
            retained_left.insert(left_index);
            retained_right.insert(right_index);
            left_index += 1;
            right_index += 1;
        } else if lengths[left_index + 1][right_index] >= lengths[left_index][right_index + 1] {
            left_index += 1;
        } else {
            right_index += 1;
        }
    }
    (retained_left, retained_right)
}

fn coordination_region<'a>(
    projection: &'a ScopeProjection,
    mobile: &'a MobileOccurrence,
) -> Option<&'a ScopeNode> {
    if let Some(scope_subtree) = &mobile.scope_subtree
        && let Some(target) = first_conjunct_construction(scope_subtree)
    {
        return deepest_coordination_containing(scope_subtree, target, mobile.domain);
    }
    deepest_coordination_containing(&projection.root, mobile.host_index, mobile.domain)
}

fn first_conjunct_construction(node: &ScopeNode) -> Option<usize> {
    for child in &node.children {
        if matches!(child.step, Some(ScopeStep::Conjunct(_, 0)))
            && let Some(index) = first_construction(&child.node)
        {
            return Some(index);
        }
        if let Some(index) = first_conjunct_construction(&child.node) {
            return Some(index);
        }
    }
    None
}

fn first_construction(node: &ScopeNode) -> Option<usize> {
    if let ScopeNodeKind::Construction(_, index) = node.kind {
        return Some(index);
    }
    node.children
        .iter()
        .find_map(|child| first_construction(&child.node))
}

fn deepest_coordination_containing(
    node: &ScopeNode,
    target: usize,
    domain: usize,
) -> Option<&ScopeNode> {
    if !contains_construction(node, target) {
        return None;
    }
    if let Some(deeper) = node
        .children
        .iter()
        .find_map(|child| deepest_coordination_containing(&child.node, target, domain))
    {
        return Some(deeper);
    }
    (node.domain == domain
        && matches!(node.kind, ScopeNodeKind::Construction(_, _))
        && contains_conjunct_in_domain(node, domain))
    .then_some(node)
}

fn contains_construction(node: &ScopeNode, target: usize) -> bool {
    matches!(node.kind, ScopeNodeKind::Construction(_, index) if index == target)
        || node
            .children
            .iter()
            .any(|child| contains_construction(&child.node, target))
}

fn contains_conjunct_in_domain(node: &ScopeNode, domain: usize) -> bool {
    node.domain == domain
        && node.children.iter().any(|child| {
            matches!(child.step, Some(ScopeStep::Conjunct(_, _)))
                || contains_conjunct_in_domain(&child.node, domain)
        })
}

fn same_claimed_leaves(left: &[RawLexicalClaim], right: &[RawLexicalClaim]) -> bool {
    left.len() == right.len()
        && left
            .iter()
            .zip(right)
            .all(|(left, right)| left.span == right.span && left.value == right.value)
}

fn mark_scope_region(
    projection: &ScopeProjection,
    region: &ScopeNode,
    affected: &mut HashSet<usize>,
) {
    let domain = region.domain;
    mark_construction_subtree(region, domain, affected);
    let Some(region_index) = first_construction(region) else {
        return;
    };
    let region_span = key_span(&prune_scope_node(region, &HashSet::new(), true));
    mark_same_span_ancestors(
        &projection.root,
        region_index,
        region_span,
        domain,
        affected,
    );
}

fn mark_construction_subtree(node: &ScopeNode, domain: usize, affected: &mut HashSet<usize>) {
    if let ScopeNodeKind::Construction(_, index) = node.kind
        && node.domain == domain
    {
        affected.insert(index);
    }
    for child in &node.children {
        mark_construction_subtree(&child.node, domain, affected);
    }
}

fn mark_same_span_ancestors(
    node: &ScopeNode,
    target: usize,
    target_span: Option<(usize, usize)>,
    domain: usize,
    affected: &mut HashSet<usize>,
) {
    if !contains_construction(node, target) {
        return;
    }
    if let ScopeNodeKind::Construction(_, index) = node.kind
        && node.domain == domain
        && key_span(&prune_scope_node(node, &HashSet::new(), true)) == target_span
    {
        affected.insert(index);
    }
    for child in node.children.iter().filter(|child| child.mobile.is_none()) {
        mark_same_span_ancestors(&child.node, target, target_span, domain, affected);
    }
}

fn mark_identity_difference_at(
    left: (&ScopeProjection, &[MobileOccurrence]),
    right: (&ScopeProjection, &[MobileOccurrence]),
    scope_region: Option<&MobileSignature>,
    skeleton: &[usize],
    left_affected: &mut HashSet<usize>,
    right_affected: &mut HashSet<usize>,
) -> bool {
    let (left_projection, left_mobiles) = left;
    let (right_projection, right_mobiles) = right;
    let Some(left_node) =
        construction_node_at(left_projection, left_mobiles, scope_region, skeleton)
    else {
        return false;
    };
    let Some(right_node) =
        construction_node_at(right_projection, right_mobiles, scope_region, skeleton)
    else {
        return false;
    };
    let left_chain = unary_construction_chain(left_node);
    let right_chain = unary_construction_chain(right_node);
    let common_suffix = left_chain
        .iter()
        .rev()
        .zip(right_chain.iter().rev())
        .take_while(|(left, right)| construction_identity(left) == construction_identity(right))
        .count();
    for node in &left_chain[..left_chain.len() - common_suffix] {
        left_affected.insert(construction_index(node));
    }
    for node in &right_chain[..right_chain.len() - common_suffix] {
        right_affected.insert(construction_index(node));
    }
    true
}

fn construction_node_at<'a>(
    projection: &'a ScopeProjection,
    mobiles: &'a [MobileOccurrence],
    scope_region: Option<&MobileSignature>,
    skeleton: &[usize],
) -> Option<&'a ScopeNode> {
    let region = match scope_region {
        None => &projection.root,
        Some(signature) => {
            let mut matches = mobiles
                .iter()
                .filter(|mobile| &mobile.signature == signature)
                .map(|mobile| &mobile.subtree);
            let region = matches.next()?;
            if matches.next().is_some() {
                return None;
            }
            region
        }
    };
    let mut paths = HashMap::new();
    collect_construction_paths(region, &[], &[], &[], &mut paths);
    let mut matches = paths
        .into_iter()
        .filter(|(_, path)| path.anchor.is_empty() && path.skeleton == skeleton)
        .map(|(index, _)| index);
    let index = matches.next()?;
    if matches.next().is_some() {
        return None;
    }
    find_construction(region, index)
}

fn find_construction(node: &ScopeNode, target: usize) -> Option<&ScopeNode> {
    if matches!(node.kind, ScopeNodeKind::Construction(_, index) if index == target) {
        return Some(node);
    }
    node.children
        .iter()
        .find_map(|child| find_construction(&child.node, target))
}

fn unary_construction_chain(mut node: &ScopeNode) -> Vec<&ScopeNode> {
    let mut chain = vec![node];
    loop {
        let mut children = Vec::new();
        collect_nonmobile_nodes(node, &mut children);
        let [child] = children.as_slice() else {
            break;
        };
        if !matches!(child.kind, ScopeNodeKind::Construction(_, _)) {
            break;
        }
        node = child;
        chain.push(node);
    }
    chain
}

fn collect_nonmobile_nodes<'a>(node: &'a ScopeNode, nodes: &mut Vec<&'a ScopeNode>) {
    for child in node.children.iter().filter(|child| child.mobile.is_none()) {
        if matches!(child.node.kind, ScopeNodeKind::Group) {
            collect_nonmobile_nodes(&child.node, nodes);
        } else {
            nodes.push(&child.node);
        }
    }
}

fn construction_identity(node: &ScopeNode) -> &'static str {
    let ScopeNodeKind::Construction(identity, _) = node.kind else {
        unreachable!("a construction chain contains constructions")
    };
    identity
}

fn construction_index(node: &ScopeNode) -> usize {
    let ScopeNodeKind::Construction(_, index) = node.kind else {
        unreachable!("a construction chain contains constructions")
    };
    index
}

fn is_step_prefix(prefix: &[ScopeStep], path: &[ScopeStep]) -> bool {
    path.starts_with(prefix)
}

fn move_is_licensed(high: &MobileOccurrence, low: &MobileOccurrence) -> bool {
    if high.domain != low.domain {
        return false;
    }
    if let Some(scope_sibling) = high.scope_sibling {
        return low
            .host
            .attachment
            .get(high.host.attachment.len())
            .is_some_and(|step| match step {
                ScopeStep::Role(role) | ScopeStep::Conjunct(role, _) => *role == scope_sibling,
            });
    }
    // The declaration compiler has already proved that a Move-A role is the
    // host's final form position. Its residual span can be empty when every
    // other realized role is independently mobile, but that does not make the
    // declared right-edge relocation inadmissible.
    true
}

fn collapse_scope_component<R: GeneratedParseRoot>(
    component: &[usize],
    adjacent: &HashMap<usize, Vec<usize>>,
    candidates: &[Candidate<R>],
    projections: &[OnceCell<ScopeProjection>],
    mobiles: &[OnceCell<Vec<MobileOccurrence>>],
    keep: &mut [bool],
) {
    let mut remaining = component.to_vec();
    while !remaining.is_empty() {
        let maximal = remaining.iter().copied().filter(|&candidate| {
            !remaining.iter().copied().any(|other| {
                candidate != other
                    && adjacent
                        .get(&candidate)
                        .is_some_and(|neighbors| neighbors.contains(&other))
                    && candidate_at_least_as_high(
                        projections[other]
                            .get()
                            .expect("a component member has a scope projection"),
                        mobiles[other]
                            .get()
                            .expect("a component member has a mobile inventory"),
                        projections[candidate]
                            .get()
                            .expect("a component member has a scope projection"),
                        mobiles[candidate]
                            .get()
                            .expect("a component member has a mobile inventory"),
                    )
                    && !candidate_at_least_as_high(
                        projections[candidate]
                            .get()
                            .expect("a component member has a scope projection"),
                        mobiles[candidate]
                            .get()
                            .expect("a component member has a mobile inventory"),
                        projections[other]
                            .get()
                            .expect("a component member has a scope projection"),
                        mobiles[other]
                            .get()
                            .expect("a component member has a mobile inventory"),
                    )
            })
        });
        let representative = maximal
            .min_by_key(|&candidate| {
                representative_key(
                    projections[candidate]
                        .get()
                        .expect("a component member has a scope projection"),
                    mobiles[candidate]
                        .get()
                        .expect("a component member has a mobile inventory"),
                )
            })
            .unwrap_or_else(|| {
                *remaining
                    .iter()
                    .min_by_key(|&&candidate| {
                        representative_key(
                            projections[candidate]
                                .get()
                                .expect("a component member has a scope projection"),
                            mobiles[candidate]
                                .get()
                                .expect("a component member has a mobile inventory"),
                        )
                    })
                    .expect("a nonempty component has a representative")
            });
        let packed = remaining
            .iter()
            .copied()
            .filter(|&candidate| {
                candidate == representative
                    || adjacent
                        .get(&representative)
                        .is_some_and(|neighbors| neighbors.contains(&candidate))
            })
            .filter(|&candidate| {
                candidate == representative
                    || candidate_at_least_as_high(
                        projections[representative]
                            .get()
                            .expect("a representative has a scope projection"),
                        mobiles[representative]
                            .get()
                            .expect("a representative has a mobile inventory"),
                        projections[candidate]
                            .get()
                            .expect("a component member has a scope projection"),
                        mobiles[candidate]
                            .get()
                            .expect("a component member has a mobile inventory"),
                    )
            })
            .collect::<Vec<_>>();
        if packed.len() > 1 {
            // A member is absorbed only once the representative can name the
            // host it gave up; an inexpressible alternative degrades to two
            // ordinary candidates rather than to a tree with no signal.
            let represented = populate_component_sites(
                representative,
                &packed,
                &candidates[representative].value,
                projections,
                mobiles,
            );
            for &candidate in &packed {
                if candidate != representative && represented.contains(&candidate) {
                    keep[candidate] = false;
                }
            }
        }
        remaining.retain(|candidate| !packed.contains(candidate));
        if packed.is_empty() {
            remaining.retain(|candidate| *candidate != representative);
        }
    }
}

fn placement_at_least_as_high(high: &MobileOccurrence, low: &MobileOccurrence) -> bool {
    high.domain == low.domain
        && ((high.role == low.role
            && high.host.identity == low.host.identity
            && match (&high.first_conjunct_path, &low.first_conjunct_path) {
                (Some(high), Some(low)) => high.len() <= low.len(),
                (Some(_) | None, None) => true,
                (None, Some(_)) => false,
            })
            || (is_step_prefix(&high.host.attachment, &low.host.attachment)
                && move_is_licensed(high, low)))
}

fn candidate_at_least_as_high(
    high_projection: &ScopeProjection,
    high_mobiles: &[MobileOccurrence],
    low_projection: &ScopeProjection,
    low_mobiles: &[MobileOccurrence],
) -> bool {
    if high_mobiles.len() == low_mobiles.len()
        && high_mobiles
            .iter()
            .zip(low_mobiles)
            .all(|(high, low)| placement_at_least_as_high(high, low))
    {
        return true;
    }
    if let Some(comparison) =
        coordination_scope_comparison(high_projection, high_mobiles, low_projection, low_mobiles)
    {
        let high_coordinations = high_projection.coordinations();
        let low_coordinations = low_projection.coordinations();
        if high_coordinations
            .iter()
            .zip(&low_coordinations)
            .any(|(high, low)| !interval_contains(low.span, high.span))
        {
            return false;
        }
        let high_outside = high_mobiles
            .iter()
            .filter(|mobile| !mobile_within_regions(mobile, &comparison.left_regions))
            .collect::<Vec<_>>();
        let low_outside = low_mobiles
            .iter()
            .filter(|mobile| !mobile_within_regions(mobile, &comparison.right_regions))
            .collect::<Vec<_>>();
        return high_outside.len() == low_outside.len()
            && high_outside
                .iter()
                .zip(low_outside)
                .all(|(high, low)| high.signature == low.signature && high.role == low.role);
    }
    false
}

type CoordinationRepresentativeKey = Vec<(Vec<usize>, usize, std::cmp::Reverse<usize>, usize)>;
type MobileRepresentativeKey = Vec<(std::cmp::Reverse<usize>, usize, usize, Vec<ScopeStep>)>;
type RepresentativeKey = (CoordinationRepresentativeKey, MobileRepresentativeKey);

fn representative_key(
    projection: &ScopeProjection,
    mobiles: &[MobileOccurrence],
) -> RepresentativeKey {
    let coordinations = projection
        .coordinations()
        .into_iter()
        .map(|coordination| {
            (
                coordination.anchor,
                coordination.span.1.saturating_sub(coordination.span.0),
                std::cmp::Reverse(coordination.span.0),
                coordination.span.1,
            )
        })
        .collect::<Vec<_>>();
    let mut order = (0..mobiles.len()).collect::<Vec<_>>();
    order.sort_by_key(|&index| {
        let span = mobiles[index].signature.span.unwrap_or((0, 0));
        (std::cmp::Reverse(span.1.saturating_sub(span.0)), span.0)
    });
    let placements = order
        .into_iter()
        .map(|index| {
            let span = mobiles[index].signature.span.unwrap_or((0, 0));
            let host = mobiles[index]
                .first_conjunct_path
                .as_ref()
                .unwrap_or(&mobiles[index].host.attachment);
            (
                std::cmp::Reverse(span.1.saturating_sub(span.0)),
                span.0,
                host.len(),
                host.clone(),
            )
        })
        .collect();
    (coordinations, placements)
}

fn populate_component_sites<R: GeneratedParseRoot>(
    representative: usize,
    packed: &[usize],
    value: &R,
    projections: &[OnceCell<ScopeProjection>],
    mobiles: &[OnceCell<Vec<MobileOccurrence>>],
) -> HashSet<usize> {
    let representative_projection = projections[representative]
        .get()
        .expect("a representative has a scope projection");
    let representative_mobiles = mobiles[representative]
        .get()
        .expect("a representative has a mobile inventory");
    let mut represented = HashSet::new();
    for mobile in representative_mobiles {
        let paths = packed
            .iter()
            .copied()
            .filter(|candidate| *candidate != representative)
            .filter_map(|candidate| {
                let candidate_mobiles = mobiles[candidate]
                    .get()
                    .expect("a component member has a mobile inventory");
                let candidate_projection = projections[candidate]
                    .get()
                    .expect("a component member has a scope projection");
                let lower = candidate_mobiles.iter().find(|lower| {
                    lower.signature == mobile.signature
                        && lower.role == mobile.role
                        && lower.domain == mobile.domain
                });
                // Two hosts can share a construction identity at two heights,
                // so the attachment relation — not the identity — decides
                // whether the alternative is a height or a partition.
                lower
                    .and_then(|lower| {
                        if mobile.host.attachment != lower.host.attachment {
                            attachment_path_between(&mobile.host, &lower.host)
                        } else if mobile.first_conjunct_path != lower.first_conjunct_path {
                            mobile.first_conjunct_path.clone()
                        } else {
                            alternative_path_within_host(
                                representative_projection,
                                mobile,
                                candidate_projection,
                                lower,
                            )
                        }
                    })
                    .or_else(|| {
                        alternative_move_s_path(
                            representative_projection,
                            representative_mobiles,
                            mobile,
                            candidate_projection,
                            candidate_mobiles,
                        )
                    })
                    .filter(|path| !path.is_empty())
                    .map(|path| (candidate, path))
            })
            .collect::<Vec<_>>();
        represented.extend(paths.iter().map(|(candidate, _)| *candidate));
        let mut paths = paths.into_iter().map(|(_, path)| path).collect::<Vec<_>>();
        paths.sort_by_key(|path| (path.len(), path.clone()));
        paths.dedup();
        let mut paths = paths
            .into_iter()
            .map(|steps| {
                AttachmentSitePath::new(
                    steps
                        .into_iter()
                        .map(|step| match step {
                            ScopeStep::Role(role) => AttachmentSiteStep::Role(role),
                            ScopeStep::Conjunct(role, ordinal) => {
                                AttachmentSiteStep::Conjunct(role, ordinal)
                            }
                        })
                        .collect(),
                )
            })
            .collect::<Vec<_>>();
        paths.dedup();
        if !paths.is_empty() {
            let mut visitor = PopulateSitesVisitor {
                target_construction: mobile.host_index,
                target_role: mobile.role,
                paths: Some(paths),
                next_construction: 0,
                construction_stack: Vec::new(),
            };
            value.visit_scope(&mut visitor);
            debug_assert!(visitor.paths.is_none());
        }
    }
    represented
}

fn alternative_move_s_path(
    representative: &ScopeProjection,
    representative_mobiles: &[MobileOccurrence],
    mobile: &MobileOccurrence,
    lower: &ScopeProjection,
    lower_mobiles: &[MobileOccurrence],
) -> Option<Vec<ScopeStep>> {
    let mobile_span = mobile.signature.span?;
    let comparison = coordination_scope_comparison(
        representative,
        representative_mobiles,
        lower,
        lower_mobiles,
    )?;
    if !mobile_within_regions(mobile, &comparison.left_regions) {
        return None;
    }
    let representative_coordinations = representative.coordinations();
    let lower_coordinations = lower.coordinations();
    representative_coordinations
        .iter()
        .zip(&lower_coordinations)
        .find_map(|(representative_coordination, lower_coordination)| {
            if representative_coordination.anchor != lower_coordination.anchor
                || representative_coordination.domain != mobile.domain
                || lower_coordination.domain != mobile.domain
            {
                return None;
            }
            let (representative_span, lower_span) = coordination_comparison_spans(
                representative,
                representative_mobiles,
                representative_coordination,
                lower,
                lower_mobiles,
                lower_coordination,
            );
            if !interval_contains(
                (
                    representative_span.0.min(lower_span.0),
                    representative_span.1.max(lower_span.1),
                ),
                mobile_span,
            ) {
                return None;
            }
            if let Some(path) = &mobile.first_conjunct_path {
                return Some(path.clone());
            }
            let mut path = representative_coordination
                .path
                .attachment
                .strip_prefix(mobile.host.attachment.as_slice())?
                .to_vec();
            let ordinal = if mobile_span.1 <= representative_span.0 {
                0
            } else if mobile_span.0 >= representative_span.1 {
                representative_coordination.conjuncts.last()?.ordinal
            } else {
                return None;
            };
            path.push(ScopeStep::Role(representative_coordination.sequence_role));
            path.push(ScopeStep::Conjunct(
                representative_coordination.sequence_role,
                ordinal,
            ));
            Some(path)
        })
}

fn alternative_path_within_host(
    representative_projection: &ScopeProjection,
    representative: &MobileOccurrence,
    lower_projection: &ScopeProjection,
    lower: &MobileOccurrence,
) -> Option<Vec<ScopeStep>> {
    if representative.domain != lower.domain {
        return None;
    }
    let representative_host =
        find_construction(&representative_projection.root, representative.host_index)?;
    let lower_host = find_construction(&lower_projection.root, lower.host_index)?;
    let mut representative_affected = HashSet::new();
    let mut lower_affected = HashSet::new();
    if !mark_identity_differences(
        representative_host,
        lower_host,
        &mut representative_affected,
        &mut lower_affected,
    ) {
        return None;
    }
    let paths = representative_projection.construction_paths();
    paths
        .into_iter()
        .filter(|(index, path)| {
            representative_affected.contains(index)
                && path.domain == representative.domain
                && path.attachment.len() > representative.host.attachment.len()
                && path.attachment.starts_with(&representative.host.attachment)
        })
        .min_by_key(|(_, path)| path.attachment.len())
        .and_then(|(_, path)| {
            path.attachment
                .strip_prefix(representative.host.attachment.as_slice())
                .map(<[ScopeStep]>::to_vec)
        })
}

fn attachment_path_between(
    high: &ConstructionPath,
    low: &ConstructionPath,
) -> Option<Vec<ScopeStep>> {
    if high.domain != low.domain {
        return None;
    }
    low.attachment
        .strip_prefix(high.attachment.as_slice())
        .map(<[ScopeStep]>::to_vec)
}

struct PopulateSitesVisitor {
    target_construction: usize,
    target_role: &'static str,
    paths: Option<Vec<AttachmentSitePath>>,
    next_construction: usize,
    construction_stack: Vec<usize>,
}

impl Visitor for PopulateSitesVisitor {
    fn enter_construction(&mut self, _construction: &'static str) {
        let index = self.next_construction;
        self.next_construction += 1;
        self.construction_stack.push(index);
    }

    fn exit_construction(&mut self) {
        self.construction_stack.pop();
    }

    fn enter_role(
        &mut self,
        role: &'static str,
        _scope_sibling: Option<&'static str>,
        admissible_sites: Option<&AdmissibleSites>,
    ) {
        if self.construction_stack.last() == Some(&self.target_construction)
            && role == self.target_role
            && let (Some(sites), Some(paths)) = (admissible_sites, self.paths.take())
        {
            sites.replace(paths);
        }
    }
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
    use super::without_scope_collapse;
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
    use crate::parser::SpecificityTier;
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
            matches!(
                card_name,
                "Zacama, Primal Calamity" | "Daretti, Rocketeer Engineer"
            ),
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
                1,
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
                let decision = analysis
                    .decision()
                    .expect("the single reading still records a selection decision");
                assert_eq!(decision.resolution(), SelectionResolution::Unique);
                assert!(decision.exception_uses().is_empty());
                assert_eq!(decision.survivors().len(), 1);
                let selected = decision
                    .selected()
                    .expect("the unique reading is the selected one");
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
                        "PostmodifiedReferencePrepositionalQualifiedReference",
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

    fn ability_candidates(
        text: &str,
        card_name: &str,
        collapse: bool,
    ) -> Vec<super::Candidate<crate::ast::Ability>> {
        let forest = slice_candidates(text, card_name).expect("witness reaches an ability root");
        let environment = canonical_test_environment();
        let parse_context = context(card_name);
        if collapse {
            materialize(&forest, &parse_context, &environment)
        } else {
            without_scope_collapse(|| materialize(&forest, &parse_context, &environment))
        }
    }

    #[derive(Default)]
    struct ScopeWitnessVisitor {
        leaves: usize,
        construction_path: Vec<&'static str>,
        verb_frames: Vec<&'static str>,
        frame_roles: Vec<&'static str>,
        populated_frame_roles: Vec<&'static str>,
        populated_sites: Vec<(
            &'static str,
            Option<&'static str>,
            Vec<crate::constructions::AttachmentSitePath>,
        )>,
    }

    impl crate::constructions::Visitor for ScopeWitnessVisitor {
        fn enter_construction(&mut self, construction: &'static str) {
            self.construction_path.push(construction);
        }

        fn verb_frame(&mut self, construction: &'static str) {
            self.verb_frames.push(construction);
        }

        fn enter_frame_role(&mut self, role: &'static str) {
            self.frame_roles.push(role);
        }

        fn exit_frame_role(&mut self) {
            self.frame_roles.pop();
        }

        fn enter_role(
            &mut self,
            role: &'static str,
            scope_sibling: Option<&'static str>,
            admissible_sites: Option<&crate::constructions::AdmissibleSites>,
        ) {
            if self.frame_roles.last() == Some(&role)
                && admissible_sites.is_some_and(|sites| !sites.is_empty())
            {
                self.populated_frame_roles.push(role);
            }
            if let Some(sites) = admissible_sites
                && !sites.is_empty()
            {
                self.populated_sites
                    .push((role, scope_sibling, sites.paths().to_vec()));
            }
        }

        fn scope_leaf(&mut self) {
            self.leaves += 1;
        }
    }

    #[test]
    fn declared_frame_roles_are_opaque_to_scope_packing() {
        let text = "Search your library for a card.";
        let raw = ability_candidates(text, "Cynical Loner", false);
        let packed = ability_candidates(text, "Cynical Loner", true);
        assert_eq!(raw.len(), 2);
        assert_eq!(packed.len(), raw.len());
        assert_eq!(
            raw.iter()
                .filter(|candidate| {
                    !super::ScopeProjection::of(&candidate.value)
                        .verb_frames()
                        .is_empty()
                })
                .count(),
            1,
        );
        assert!(packed.iter().all(|candidate| {
            scope_witness(&candidate.value)
                .populated_frame_roles
                .is_empty()
        }));

        let analysis = super::super::selection::analyze_selection(packed)
            .expect("the canonical exception inventory is valid");
        let (selected, decision) = analysis.into_result_and_decision();
        let selected = selected
            .expect("the two frame readings remain resolvable")
            .expect("one frame reading is selected");
        assert_eq!(
            decision.expect("selection records its basis").resolution(),
            SelectionResolution::Specificity,
        );
        assert!(!scope_witness(&selected.value).verb_frames.is_empty());
    }

    #[test]
    fn opacity_preserves_adjunct_and_frame_internal_scope_packing() {
        // The measure has one host here, so the device must leave the unit
        // exactly as it found it: the Adjunct derivation survives untouched.
        let raw_adjunct = ability_candidates(
            "Draw a card for each Island you control.",
            "Flow of Ideas",
            false,
        );
        let adjunct = ability_candidates(
            "Draw a card for each Island you control.",
            "Flow of Ideas",
            true,
        );
        assert_eq!(raw_adjunct.len(), 1);
        assert_eq!(adjunct.len(), raw_adjunct.len());
        let adjunct_projection = super::ScopeProjection::of(&adjunct[0].value);
        assert!(adjunct_projection.verb_frames().is_empty());
        assert!(
            adjunct_projection
                .mobiles()
                .iter()
                .any(|mobile| mobile.role == "adjunct")
        );

        {
            let text = "Put all artifacts and creatures with mana value X or less into your hand.";
            let raw = ability_candidates(text, "Context Card", false);
            let packed = ability_candidates(text, "Context Card", true);
            assert!(
                raw.len() > packed.len(),
                "the scope alternatives behind an opaque edge did not pack for {text:?}: {} -> {}",
                raw.len(),
                packed.len(),
            );
            assert!(packed.iter().all(|candidate| {
                scope_witness(&candidate.value)
                    .populated_frame_roles
                    .is_empty()
            }));
        }

        {
            let text =
                "A creature card you control in exile with mana value 2 or less gains 2 life.";
            let raw = ability_candidates(text, "Context Card", false);
            let packed = ability_candidates(text, "Context Card", true);
            assert!(
                raw.len() > packed.len(),
                "the scope alternatives inside a frame Object did not pack for {text:?}: {} -> {}",
                raw.len(),
                packed.len(),
            );
        }
    }

    #[test]
    fn distributive_measures_have_only_the_predicate_attachment() {
        let witnesses = [
            ("Flow of Ideas", "Draw a card for each Island you control."),
            (
                "Collective Unconscious",
                "Draw a card for each creature you control.",
            ),
            (
                "Animal Friend",
                "Enchanted creature has \"Whenever this creature attacks, create a 1/1 green Squirrel creature token. Put a +1/+1 counter on that token for each Aura and Equipment attached to this creature other than Animal Friend.\"",
            ),
            (
                "General Leo Cristophe",
                "When General Leo Cristophe enters, return up to one target creature card with mana value 3 or less from your graveyard to the battlefield. Then put a +1/+1 counter on General Leo Cristophe for each creature you control.",
            ),
            (
                "Context Card",
                "Double the number of +1/+1 counters on each creature you control.",
            ),
        ];

        for (card_name, text) in witnesses {
            let candidates = ability_candidates(text, card_name, false);
            assert!(
                !candidates.is_empty(),
                "the witness did not parse for {card_name}"
            );
            let environment = canonical_test_environment();
            let parse_context = context(card_name);
            for candidate in candidates {
                assert_eq!(candidate.value.render(&parse_context, &environment), text);
                assert!(
                    super::ScopeProjection::of(&candidate.value)
                        .mobiles()
                        .iter()
                        .any(|mobile| mobile.role == "adjunct"),
                    "the measure did not attach to the Predicate for {card_name}",
                );
            }
        }
    }

    fn scope_witness(value: &crate::ast::Ability) -> ScopeWitnessVisitor {
        let mut visitor = ScopeWitnessVisitor::default();
        crate::constructions::GeneratedParseRoot::visit_scope(value, &mut visitor);
        visitor
    }

    #[test]
    fn declared_scope_variants_pack_and_preserve_rendered_bytes() {
        let witnesses = [
            (
                "Seedborn Muse",
                "Untap all permanents you control during each other player's untap step.",
            ),
            (
                "Context Card",
                "Destroy all artifacts and creatures with mana value X or less.",
            ),
            (
                "Grafdigger's Cage",
                "Players can't cast spells from graveyards or libraries.",
            ),
            (
                "Ground Seal",
                "Cards in graveyards can't be the targets of spells or abilities.",
            ),
            (
                "Mass Manipulation",
                "Gain control of X target creatures and/or planeswalkers.",
            ),
        ];

        for (card_name, text) in witnesses {
            let raw = ability_candidates(text, card_name, false);
            let packed = ability_candidates(text, card_name, true);
            assert!(
                raw.len() > packed.len(),
                "scope variants did not pack for {card_name}: {} -> {}",
                raw.len(),
                packed.len(),
            );
            let environment = canonical_test_environment();
            let parse_context = context(card_name);
            assert!(
                raw.iter()
                    .chain(&packed)
                    .all(|candidate| candidate.value.render(&parse_context, &environment) == text),
                "a packed unit changed its rendered bytes for {card_name}",
            );
            assert!(
                packed
                    .iter()
                    .any(|candidate| !scope_witness(&candidate.value).populated_sites.is_empty()),
                "the representative did not retain its alternative sites for {card_name}",
            );
        }
    }

    #[test]
    fn aquatic_alchemist_exposes_the_shared_constituent_coordination_site() {
        let card_name = "Aquatic Alchemist // Bubble Up";
        let text = "Whenever you cast your first instant or sorcery spell each turn, this creature gets +2/+0 until end of turn.";
        let raw = ability_candidates(text, card_name, false);
        let packed = ability_candidates(text, card_name, true);
        assert_eq!(raw.len(), 2);
        assert_eq!(packed.len(), 1);
        let witness = scope_witness(&packed[0].value);
        for role in ["first", "head"] {
            let paths = witness
                .populated_sites
                .iter()
                .find_map(|(candidate_role, _, paths)| (*candidate_role == role).then_some(paths))
                .unwrap_or_else(|| panic!("the shared {role} role carries an alternative site"));
            assert!(paths.iter().all(|path| {
                path.steps().iter().any(|step| {
                    matches!(
                        step,
                        crate::constructions::AttachmentSiteStep::Conjunct(_, _)
                    )
                })
            }));
        }
        let environment = canonical_test_environment();
        let parse_context = context(card_name);
        let packed_leaf_count = witness.leaves;
        assert!(raw.iter().all(|candidate| {
            candidate.value.render(&parse_context, &environment) == text
                && scope_witness(&candidate.value).leaves == packed_leaf_count
        }));
        assert_eq!(packed[0].value.render(&parse_context, &environment), text,);
    }

    fn assert_coordination_scope_witness(
        card_name: &str,
        text: &str,
        requires_conjunct_site: bool,
    ) {
        let raw = ability_candidates(text, card_name, false);
        let packed = ability_candidates(text, card_name, true);
        assert!(
            raw.len() > packed.len(),
            "the required coordination variants did not pack for {card_name}: {} -> {}",
            raw.len(),
            packed.len(),
        );
        let environment = canonical_test_environment();
        let parse_context = context(card_name);
        let leaf_count = scope_witness(&raw[0].value).leaves;
        for candidate in raw.iter().chain(&packed) {
            assert_eq!(
                candidate.value.render(&parse_context, &environment),
                text,
                "a packed unit changed its rendered bytes for {card_name}",
            );
            assert_eq!(
                scope_witness(&candidate.value).leaves,
                leaf_count,
                "packing changed leaf traversal for {card_name}",
            );
        }
        let populated_paths = packed
            .iter()
            .flat_map(|candidate| scope_witness(&candidate.value).populated_sites)
            .flat_map(|(_, _, paths)| paths)
            .collect::<Vec<_>>();
        assert!(
            !populated_paths.is_empty(),
            "the representative has no admissible alternative site for {card_name}",
        );
        if requires_conjunct_site {
            assert!(
                populated_paths
                    .iter()
                    .any(|path| path.steps().iter().any(|step| matches!(
                        step,
                        crate::constructions::AttachmentSiteStep::Conjunct(_, _)
                    ))),
                "no coordination scope site names a Conjunct for {card_name}",
            );
        }
        for (role, scope_sibling, paths) in packed
            .iter()
            .flat_map(|candidate| scope_witness(&candidate.value).populated_sites)
            .filter(|(_, scope_sibling, _)| scope_sibling.is_some())
        {
            assert!(
                paths.iter().all(|path| match path.steps().last() {
                    Some(crate::constructions::AttachmentSiteStep::Conjunct(_, _)) => true,
                    Some(crate::constructions::AttachmentSiteStep::Role(role)) => {
                        Some(*role) == scope_sibling
                    }
                    None => false,
                }),
                "site for {card_name} role {role} escapes {scope_sibling:?}: {paths:?}",
            );
        }
        let projections = packed
            .iter()
            .map(|candidate| super::ScopeProjection::of(&candidate.value))
            .collect::<Vec<_>>();
        let mobiles = projections
            .iter()
            .map(super::ScopeProjection::mobiles)
            .collect::<Vec<_>>();
        let rendered = packed
            .iter()
            .map(|candidate| {
                let (text, claims) = crate::constructions::GeneratedParseRoot::render_with_claims(
                    &candidate.value,
                    &parse_context,
                    &environment,
                );
                super::RenderedCandidate { text, claims }
            })
            .collect::<Vec<_>>();
        for left in 0..packed.len() {
            for right in left + 1..packed.len() {
                assert!(
                    !super::verify_scope_pair(
                        left,
                        right,
                        &packed,
                        &projections,
                        &mobiles,
                        &rendered,
                    ),
                    "a verified flat-versus-nested pair survived for {card_name}",
                );
            }
        }
    }

    #[test]
    fn flat_and_nested_coordination_units_pack() {
        let witnesses = [
            (
                "Grafdigger's Cage",
                "Players can't cast spells from graveyards or libraries.",
                false,
            ),
            (
                "Weathered Runestone",
                "Players can't cast spells from graveyards or libraries.",
                false,
            ),
            (
                "Ground Seal",
                "Cards in graveyards can't be the targets of spells or abilities.",
                false,
            ),
            (
                "Silent Gravestone",
                "Cards in graveyards can't be the targets of spells or abilities.",
                false,
            ),
            (
                "Mass Manipulation",
                "Gain control of X target creatures and/or planeswalkers.",
                false,
            ),
            (
                "Grand Abolisher",
                "During your turn, your opponents can't cast spells or activate abilities of artifacts, creatures, or enchantments.",
                true,
            ),
            (
                "Fury",
                "When this creature enters, it deals 4 damage divided as you choose among any number of target creatures and/or planeswalkers.",
                true,
            ),
            (
                "Reprocess",
                "Sacrifice any number of artifacts, creatures, and/or lands.",
                true,
            ),
            (
                "Lich-Knights' Conquest",
                "Sacrifice any number of artifacts, enchantments, and/or tokens.",
                true,
            ),
            (
                "Malevolent Witchkite",
                "When this creature enters, sacrifice any number of artifacts, enchantments, and/or tokens, then draw that many cards.",
                true,
            ),
            (
                "Boltbender",
                "When this creature is turned face up, you may choose new targets for any number of other spells and/or abilities.",
                true,
            ),
        ];

        for (card_name, text, requires_conjunct_site) in witnesses {
            assert_coordination_scope_witness(card_name, text, requires_conjunct_site);
        }
    }

    #[test]
    fn packing_keeps_leaf_traversal_and_uses_the_hoisted_construction_path() {
        let card_name = "Seedborn Muse";
        let text = "Untap all permanents you control during each other player's untap step.";
        let raw = ability_candidates(text, card_name, false);
        let packed = ability_candidates(text, card_name, true);
        let environment = canonical_test_environment();
        let parse_context = context(card_name);
        let projections = raw
            .iter()
            .map(|candidate| super::ScopeProjection::of(&candidate.value))
            .collect::<Vec<_>>();
        let mobiles = projections
            .iter()
            .map(super::ScopeProjection::mobiles)
            .collect::<Vec<_>>();
        let rendered = raw
            .iter()
            .map(|candidate| {
                let (text, claims) = crate::constructions::GeneratedParseRoot::render_with_claims(
                    &candidate.value,
                    &parse_context,
                    &environment,
                );
                super::RenderedCandidate { text, claims }
            })
            .collect::<Vec<_>>();

        for representative in &packed {
            let representative_index = raw
                .iter()
                .position(|candidate| candidate.constructions == representative.constructions)
                .expect("the canonical construction path comes from one packed member");
            let representative_walk = scope_witness(&representative.value);
            assert_eq!(
                representative_walk.construction_path,
                representative
                    .constructions
                    .iter()
                    .map(|construction| construction.name())
                    .collect::<Vec<_>>(),
            );
            for candidate in 0..raw.len() {
                if candidate != representative_index
                    && super::verify_scope_pair(
                        representative_index,
                        candidate,
                        &raw,
                        &projections,
                        &mobiles,
                        &rendered,
                    )
                {
                    assert!(
                        mobiles[representative_index]
                            .iter()
                            .zip(&mobiles[candidate])
                            .all(|(high, low)| super::placement_at_least_as_high(high, low)),
                    );
                    assert_eq!(
                        representative_walk.leaves,
                        scope_witness(&raw[candidate].value).leaves,
                    );
                }
            }
        }
        assert!(
            mobiles
                .iter()
                .flatten()
                .any(|mobile| mobile.scope_region.is_some()),
            "the witness keeps a mobile nested inside another mobile's region",
        );
    }

    #[test]
    fn construction_identity_outside_a_move_region_prevents_a_merge() {
        for (card_name, text) in [
            (
                "Daretti, Rocketeer Engineer",
                "Daretti's power is equal to the greatest mana value among artifacts you control.",
            ),
            (
                "Nightmare",
                "Nightmare's power and toughness are each equal to the number of Swamps you control.",
            ),
        ] {
            let candidates = ability_candidates(text, card_name, true);
            let identity = candidates
                .iter()
                .find(|candidate| {
                    candidate
                        .constructions
                        .contains(&Construction::PossessiveOwnerPossessiveSelfReference)
                })
                .expect("the identity construction remains a distinct candidate")
                .clone();
            let declared = candidates
                .iter()
                .find(|candidate| {
                    candidate
                        .constructions
                        .contains(&Construction::PossessiveOwnerPossessiveSingularNominal)
                })
                .expect("the declared-type construction remains a distinct candidate")
                .clone();
            let mut candidates = vec![identity, declared];
            for candidate in &mut candidates {
                for tier in &mut candidate.specificity {
                    if *tier == SpecificityTier::Identity {
                        *tier = SpecificityTier::TypedLexical;
                    }
                }
            }
            let analysis = super::super::selection::analyze_selection(candidates)
                .expect("the fixture uses the valid empty exception inventory");
            let (result, decision) = analysis.into_result_and_decision();
            assert!(matches!(
                result,
                Err(crate::parser::ParseError::Ambiguous { .. })
            ));
            assert_eq!(
                decision.expect("the tie remains visible").resolution(),
                SelectionResolution::UnresolvedTie,
            );
        }
    }

    #[test]
    fn equal_bracketing_family_swap_keeps_both_live_candidates() {
        let card_name = "Clarion Conqueror";
        let text =
            "Activated abilities of artifacts, creatures, and planeswalkers can't be activated.";
        let raw = ability_candidates(text, card_name, false);
        let packed = ability_candidates(text, card_name, true);
        assert_eq!(packed.len(), raw.len());

        let environment = canonical_test_environment();
        let parse_context = context(card_name);
        let projections = raw
            .iter()
            .map(|candidate| super::ScopeProjection::of(&candidate.value))
            .collect::<Vec<_>>();
        let mobiles = projections
            .iter()
            .map(super::ScopeProjection::mobiles)
            .collect::<Vec<_>>();
        let rendered = raw
            .iter()
            .map(|candidate| {
                let (text, claims) = crate::constructions::GeneratedParseRoot::render_with_claims(
                    &candidate.value,
                    &parse_context,
                    &environment,
                );
                super::RenderedCandidate { text, claims }
            })
            .collect::<Vec<_>>();
        let pair = (0..raw.len()).find_map(|left| {
            (left + 1..raw.len()).find_map(|right| {
                let left_coordinates = projections[left]
                    .coordinations()
                    .into_iter()
                    .map(|coordination| coordination.anchor)
                    .collect::<Vec<_>>();
                let right_coordinates = projections[right]
                    .coordinations()
                    .into_iter()
                    .map(|coordination| coordination.anchor)
                    .collect::<Vec<_>>();
                (!left_coordinates.is_empty()
                    && left_coordinates == right_coordinates
                    && raw[left].constructions != raw[right].constructions)
                    .then_some((left, right))
            })
        });
        let (left, right) =
            pair.expect("the live fixture retains its equal-bracketing family swap");
        assert!(!super::verify_scope_pair(
            left,
            right,
            &raw,
            &projections,
            &mobiles,
            &rendered,
        ));
    }

    #[test]
    fn coordination_associativity_breaks_the_anchor_bijection() {
        fn leaf(index: usize) -> super::ScopeNode {
            super::ScopeNode {
                kind: super::ScopeNodeKind::Leaf(index),
                children: Vec::new(),
                span: Some((index, index + 1)),
                domain: 0,
                verb_frame: false,
            }
        }

        fn child(step: Option<super::ScopeStep>, node: super::ScopeNode) -> super::ScopeChild {
            super::ScopeChild {
                step,
                mobile: None,
                node,
            }
        }

        fn coordination(
            identity: &'static str,
            index: usize,
            children: Vec<super::ScopeChild>,
        ) -> super::ScopeNode {
            let span = super::child_span(&children);
            super::ScopeNode {
                kind: super::ScopeNodeKind::Construction(identity, index),
                children,
                span,
                domain: 0,
                verb_frame: false,
            }
        }

        let role = "members";
        let flat = super::ScopeProjection {
            root: coordination(
                "FixtureFlat",
                0,
                vec![
                    child(Some(super::ScopeStep::Conjunct(role, 0)), leaf(0)),
                    child(None, leaf(1)),
                    child(Some(super::ScopeStep::Conjunct(role, 1)), leaf(2)),
                    child(None, leaf(3)),
                    child(Some(super::ScopeStep::Conjunct(role, 2)), leaf(4)),
                ],
            ),
        };
        let mut family_swap = flat.clone();
        family_swap.root.kind = super::ScopeNodeKind::Construction("FixtureFamilySwap", 0);
        assert!(
            super::coordination_scope_comparison(&flat, &[], &family_swap, &[]).is_none(),
            "equal Coordination spans must use ordinary identity comparison",
        );
        let nested_tail = coordination(
            "FixtureNestedTail",
            1,
            vec![
                child(Some(super::ScopeStep::Conjunct(role, 0)), leaf(2)),
                child(None, leaf(3)),
                child(Some(super::ScopeStep::Conjunct(role, 1)), leaf(4)),
            ],
        );
        let nested = super::ScopeProjection {
            root: coordination(
                "FixtureNested",
                0,
                vec![
                    child(Some(super::ScopeStep::Conjunct(role, 0)), leaf(0)),
                    child(None, leaf(1)),
                    child(Some(super::ScopeStep::Conjunct(role, 1)), nested_tail),
                ],
            ),
        };

        assert!(
            super::coordination_scope_comparison(&flat, &[], &nested, &[]).is_none(),
            "a nested Coordination must not pair with one flat Coordination",
        );

        let left_inner = coordination(
            "FixtureLeftInner",
            1,
            vec![
                child(Some(super::ScopeStep::Conjunct(role, 0)), leaf(0)),
                child(None, leaf(1)),
                child(Some(super::ScopeStep::Conjunct(role, 1)), leaf(2)),
            ],
        );
        let left_binary = super::ScopeProjection {
            root: coordination(
                "FixtureLeftOuter",
                0,
                vec![
                    child(Some(super::ScopeStep::Conjunct(role, 0)), left_inner),
                    child(None, leaf(3)),
                    child(Some(super::ScopeStep::Conjunct(role, 1)), leaf(4)),
                ],
            ),
        };
        let right_inner = coordination(
            "FixtureRightInner",
            1,
            vec![
                child(Some(super::ScopeStep::Conjunct(role, 0)), leaf(2)),
                child(None, leaf(3)),
                child(Some(super::ScopeStep::Conjunct(role, 1)), leaf(4)),
            ],
        );
        let right_binary = super::ScopeProjection {
            root: coordination(
                "FixtureRightOuter",
                0,
                vec![
                    child(Some(super::ScopeStep::Conjunct(role, 0)), leaf(0)),
                    child(None, leaf(1)),
                    child(Some(super::ScopeStep::Conjunct(role, 1)), right_inner),
                ],
            ),
        };
        assert!(
            super::coordination_scope_comparison(&left_binary, &[], &right_binary, &[]).is_none(),
            "binary rebracketing without a declared mobile yield must not pack",
        );
    }

    #[test]
    fn every_absorbed_member_leaves_an_alternative_site() {
        // Two hosts of one mobile can be instances of the same construction at
        // two heights. Nothing may be absorbed unless the representative can
        // name the host it gave up.
        for (card_name, text) in [
            (
                "Harabaz Druid",
                "{T}: Add X mana of any one color, where X is the number of Allies you control.",
            ),
            (
                "Hellkite Igniter",
                "{1}{R}: This creature gets +X/+0 until end of turn, where X is the number of artifacts you control.",
            ),
        ] {
            let raw = ability_candidates(text, card_name, false);
            let packed = ability_candidates(text, card_name, true);
            assert!(
                raw.len() > packed.len(),
                "the same-identity height pair did not pack for {card_name}: {} -> {}",
                raw.len(),
                packed.len(),
            );
            let sites = packed
                .iter()
                .flat_map(|candidate| scope_witness(&candidate.value).populated_sites)
                .collect::<Vec<_>>();
            assert_eq!(
                sites.len(),
                raw.len() - packed.len(),
                "an absorbed member left no alternative site for {card_name}: {sites:?}",
            );
        }
    }
}
