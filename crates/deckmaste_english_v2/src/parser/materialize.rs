use std::collections::BTreeMap;
use std::collections::BTreeSet;

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
use crate::constructions::BuildValue;
use crate::constructions::Category;
use crate::constructions::Construction;
use crate::constructions::Leaf;
use crate::constructions::Lexical;
use crate::constructions::LexicalOwner;
use crate::constructions::LexicalTerminal;
use crate::constructions::RULES;
use crate::constructions::RuleId;
use crate::constructions::build;
use crate::context::ParseContext;
use crate::render::Render;

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) struct MaterializedCandidate<V, C, K = Category, M = Lexical, T = (), O = ()> {
    pub(super) value: V,
    pub(super) constructions: Vec<C>,
    pub(super) positions: Vec<RulePosition<K, M>>,
    pub(super) claims: Vec<SpannedLexical<T, O>>,
}

type MaterializedMemo<V, C, K, M, T, O> =
    BTreeMap<NodeId, Vec<MaterializedCandidate<V, C, K, M, T, O>>>;

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
pub(crate) struct Candidate {
    pub ability: Ability,
    pub constructions: Vec<Construction>,
    pub positions: Vec<RulePosition<Category, Lexical>>,
    pub specificity: Vec<super::SpecificityTier>,
    pub claims: Vec<RawLexicalClaim>,
    pub synthetic_claims: Vec<crate::parser::TextSpan>,
}

trait MaterializationObservation<R> {
    const ENABLED: bool;
    fn cycle_pruned(&mut self, _node_id: NodeId, _rule_path: &[R]) {}
}

impl<R> MaterializationObservation<R> for () {
    const ENABLED: bool = false;
}

impl MaterializationObservation<RuleId> for MaterializationTraceBuilder {
    const ENABLED: bool = true;

    fn cycle_pruned(&mut self, node_id: NodeId, rule_path: &[RuleId]) {
        self.record_cycle(node_id.0, rule_path);
    }
}

struct MaterializationStateFor<V, C, K = Category, M = Lexical, T = (), O = ()> {
    memo: MaterializedMemo<V, C, K, M, T, O>,
    in_progress: BTreeSet<NodeId>,
}

impl<V, C, K, M, T, O> Default for MaterializationStateFor<V, C, K, M, T, O> {
    fn default() -> Self {
        Self {
            memo: BTreeMap::new(),
            in_progress: BTreeSet::new(),
        }
    }
}

struct MaterializationOutcomeFor<V, C, K = Category, M = Lexical, T = (), O = ()> {
    values: Vec<MaterializedCandidate<V, C, K, M, T, O>>,
    cycle_pruned: bool,
}

struct MaterializationKernel<'a, R, T, V, C, K: 'static, L: 'static, M, Build> {
    rules: &'a [Rule<K, L, R>],
    rule_index: fn(R) -> usize,
    public_construction: fn(R) -> Option<C>,
    lexical_matcher: fn(L) -> M,
    build_leaf: fn(&T) -> V,
    build: Build,
}

impl<R, T, V, C, K, L, M, Build> MaterializationKernel<'_, R, T, V, C, K, L, M, Build>
where
    R: Copy,
    V: Clone + PartialEq,
    C: Copy + PartialEq,
    K: Copy + PartialEq + 'static,
    L: Copy + 'static,
    M: Clone + PartialEq,
    T: Clone + PartialEq,
    Build: Fn(R, &[V]) -> Option<V>,
{
    fn materialize<Owner, O>(
        &self,
        forest: &Forest<R, T, Owner>,
        observation: &mut O,
    ) -> Vec<MaterializedCandidate<V, C, K, M, T, Owner>>
    where
        O: MaterializationObservation<R>,
        Owner: Clone + PartialEq,
    {
        let mut candidates = Vec::new();
        let mut state = MaterializationStateFor::default();
        let mut rule_path = Vec::new();
        for root in forest.accepted_root_ids() {
            for built in self
                .materialize_node(forest, root, &mut state, &mut rule_path, observation)
                .values
            {
                push_unique(&mut candidates, built);
            }
        }
        candidates
    }

    fn materialize_node<Owner, O>(
        &self,
        forest: &Forest<R, T, Owner>,
        node_id: NodeId,
        state: &mut MaterializationStateFor<V, C, K, M, T, Owner>,
        rule_path: &mut Vec<R>,
        observation: &mut O,
    ) -> MaterializationOutcomeFor<V, C, K, M, T, Owner>
    where
        O: MaterializationObservation<R>,
        Owner: Clone + PartialEq,
    {
        if let Some(values) = state.memo.get(&node_id) {
            return MaterializationOutcomeFor {
                values: values.clone(),
                cycle_pruned: false,
            };
        }
        if !state.in_progress.insert(node_id) {
            observation.cycle_pruned(node_id, rule_path);
            return MaterializationOutcomeFor {
                values: Vec::new(),
                cycle_pruned: true,
            };
        }

        let node = forest.node(node_id);
        let mut values = Vec::new();
        let mut cycle_pruned = false;
        for family in &node.families {
            let outcome =
                self.materialize_family(forest, node.rule, family, state, rule_path, observation);
            cycle_pruned |= outcome.cycle_pruned;
            for built in outcome.values {
                push_unique(&mut values, built);
            }
        }
        state.in_progress.remove(&node_id);
        if !cycle_pruned {
            state.memo.insert(node_id, values.clone());
        }
        MaterializationOutcomeFor {
            values,
            cycle_pruned,
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
    ) -> MaterializationOutcomeFor<V, C, K, M, T, Owner>
    where
        O: MaterializationObservation<R>,
        Owner: Clone + PartialEq,
    {
        let rule = &self.rules[(self.rule_index)(rule_id)];
        let public_construction = (self.public_construction)(rule_id);
        if O::ENABLED {
            rule_path.push(rule_id);
        }
        let mut combinations = vec![Vec::new()];
        let mut cycle_pruned = false;
        for child in &family.children {
            let child_values = match child {
                Child::Node(id) => {
                    let outcome = self.materialize_node(forest, *id, state, rule_path, observation);
                    cycle_pruned |= outcome.cycle_pruned;
                    outcome.values
                }
                Child::Lexical(lexical) => vec![MaterializedCandidate {
                    value: (self.build_leaf)(&lexical.value),
                    constructions: Vec::new(),
                    positions: Vec::new(),
                    claims: vec![lexical.clone()],
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
            if let Some(value) = (self.build)(rule_id, &child_values) {
                let mut constructions = public_construction.into_iter().collect::<Vec<_>>();
                let mut positions = if public_construction.is_some() {
                    rule.rhs
                        .iter()
                        .map(|position| match *position {
                            RulePosition::Nonterminal(category) => {
                                RulePosition::Nonterminal(category)
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
                for child in children {
                    constructions.extend(child.constructions);
                    positions.extend(child.positions);
                    claims.extend(child.claims);
                }
                claims.sort_by_key(|claim| (claim.span.start, claim.span.end));
                push_unique(
                    &mut values,
                    MaterializedCandidate {
                        value,
                        constructions,
                        positions,
                        claims,
                    },
                );
            }
        }
        if O::ENABLED {
            rule_path.pop();
        }
        MaterializationOutcomeFor {
            values,
            cycle_pruned,
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
    Build: Fn(R, &[V]) -> Option<V>,
{
    MaterializationKernel {
        rules,
        rule_index,
        public_construction,
        lexical_matcher,
        build_leaf,
        build,
    }
    .materialize(forest, &mut ())
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
    Build: Fn(R, &[V]) -> Option<V>,
{
    let mut builder = MaterializationTraceBuilder::new(limits);
    let values = MaterializationKernel {
        rules,
        rule_index,
        public_construction,
        lexical_matcher,
        build_leaf,
        build,
    }
    .materialize(
        forest,
        &mut ProjectedMaterializationTrace {
            builder: &mut builder,
            identity: rule_index,
            label,
        },
    );
    (values, builder.finish())
}

pub(crate) fn materialize(
    forest: &Forest<RuleId, Leaf, LexicalOwner>,
    context: &ParseContext<'_>,
    environment: &crate::environment::ParserEnvironment,
) -> Vec<Candidate> {
    #[cfg(test)]
    super::count_pipeline_stage(super::PipelineStage::Materialize);
    let built = MaterializationKernel {
        rules: RULES,
        rule_index: RuleId::index,
        public_construction: RuleId::public_construction,
        lexical_matcher: |terminal: LexicalTerminal| terminal.matcher,
        build_leaf: |leaf: &Leaf| BuildValue::Leaf(leaf.clone()),
        build: |rule: RuleId, children: &[BuildValue]| build(rule, children, context),
    }
    .materialize(forest, &mut ());
    finalize_candidates(built, context, environment, None)
}

pub(crate) fn materialize_observed(
    forest: &Forest<RuleId, Leaf, LexicalOwner>,
    context: &ParseContext<'_>,
    environment: &crate::environment::ParserEnvironment,
    limits: TraceLimits,
) -> (Vec<Candidate>, MaterializationTrace) {
    let mut observation = MaterializationTraceBuilder::new(limits);
    let built = MaterializationKernel {
        rules: RULES,
        rule_index: RuleId::index,
        public_construction: RuleId::public_construction,
        lexical_matcher: |terminal: LexicalTerminal| terminal.matcher,
        build_leaf: |leaf: &Leaf| BuildValue::Leaf(leaf.clone()),
        build: |rule: RuleId, children: &[BuildValue]| build(rule, children, context),
    }
    .materialize(forest, &mut observation);
    let candidates = finalize_candidates(
        built,
        context,
        environment,
        Some((&mut observation, limits)),
    );
    (candidates, observation.finish())
}

fn finalize_candidates(
    built_values: Vec<
        MaterializedCandidate<BuildValue, Construction, Category, Lexical, Leaf, LexicalOwner>,
    >,
    context: &ParseContext<'_>,
    environment: &crate::environment::ParserEnvironment,
    mut observation: Option<(&mut MaterializationTraceBuilder, TraceLimits)>,
) -> Vec<Candidate> {
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
            .find(|position| matches!(position, RulePosition::Nonterminal(_)))
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
            candidate.positions[position_index] = nonterminal;
            varied.push(candidate);
        }
        varied
    });
    #[cfg(test)]
    super::count_pipeline_stage(super::PipelineStage::Specificity);
    let mut candidates = Vec::new();
    for built in built_values {
        if let BuildValue::Ability(ability) = built.value {
            let mut claims = Vec::new();
            let mut synthetic_claims = Vec::new();
            for claim in built.claims {
                match claim.owner {
                    Some(owner) => claims.push(RawLexicalClaim {
                        span: claim.span,
                        value: claim.value,
                        owner,
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
                ability,
                constructions: built.constructions,
                specificity: specificity_tiers(&built.positions),
                positions: built.positions,
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
                        candidate.ability.render(context, environment),
                        format!("{:?}", candidate.ability),
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
        build: |rule: RuleId, children: &[BuildValue]| build(rule, children, context),
    };
    kernel.materialize_node(forest, node_id, state, &mut Vec::new(), &mut ())
}

pub(super) fn completion_has_checked_build(
    rule: RuleId,
    family: &Family<Leaf, LexicalOwner>,
    forest: &Forest<RuleId, Leaf, LexicalOwner>,
    context: &ParseContext<'_>,
) -> bool {
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
        build: |rule: RuleId, children: &[BuildValue]| build(rule, children, context),
    };
    !kernel
        .materialize_family(
            forest,
            rule,
            family,
            &mut MaterializationStateFor::<
                BuildValue,
                Construction,
                Category,
                Lexical,
                Leaf,
                LexicalOwner,
            >::default(),
            &mut Vec::new(),
            &mut (),
        )
        .values
        .is_empty()
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

    use macro_ron::v2::DeclarationKind;
    use macro_ron::v2::GrammarPosition;
    use macro_ron::v2::SurfaceFeature;

    use super::BuildValue;
    use super::Category;
    use super::Construction;
    use super::Forest;
    use super::Leaf;
    use super::Lexical;
    use super::MaterializationStateFor;
    use super::RuleId;
    use super::materialize;
    use super::materialize_node;
    use super::materialize_observed;
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
    use crate::constructions::Agreement;
    use crate::constructions::DeclarationLeaf;
    use crate::constructions::DeclarationMatcher;
    use crate::constructions::FeatureConstraint;
    use crate::constructions::LexicalOwnerTemplate;
    use crate::constructions::Number;
    use crate::context::ParseContext;
    use crate::environment::DeclarationId;
    use crate::environment::canonical_test_environment;
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
    use crate::parser::scan::SliceGrammar;
    use crate::parser::scan::parse_forest;
    use crate::render::Render;

    fn lexical(value: Leaf) -> Child<Leaf> {
        Child::Lexical(SpannedLexical {
            span: crate::parser::TextSpan { start: 0, end: 1 },
            value,
            owner: None,
        })
    }

    fn slice_candidates(
        text: &str,
        card_name: &str,
    ) -> Result<
        Forest<RuleId, Leaf, crate::constructions::LexicalOwner>,
        ChartFailure<Category, Lexical>,
    > {
        let environment = canonical_test_environment();
        let context = context(card_name);
        let grammar = SliceGrammar {
            environment: &environment,
            context: &context,
        };

        parse_forest(&grammar, text)
    }

    fn context(card_name: &str) -> ParseContext<'_> {
        ParseContext::new(card_name).expect("test card names are valid parse contexts")
    }

    fn assert_acyclic_number_survives(forest: &Forest<RuleId, Leaf>) {
        let mut state =
            MaterializationStateFor::<BuildValue, Construction, Category, Lexical, Leaf>::default();
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
                lexical(Leaf::Literal(",")),
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
                        children: vec![lexical(Leaf::SignedNumber(SignedNumber {
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
                            children: vec![lexical(Leaf::SignedNumber(SignedNumber {
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
                        children: vec![lexical(Leaf::Declaration(DeclarationLeaf {
                            id: DeclarationId::new(DeclarationKind::KeywordAction, "Connive"),
                            feature: SurfaceFeature::Bare,
                        }))],
                    }],
                },
                PackedNode {
                    rule: RuleId::ClauseWhere,
                    start: 0,
                    end: 0,
                    families: vec![Family {
                        children: vec![
                            lexical(Leaf::Literal("where")),
                            lexical(Leaf::Variable(Variable::X)),
                            lexical(Leaf::Verb {
                                lexeme: VerbLexeme::Be,
                                agreement: Agreement::ThirdPersonSingular,
                            }),
                            lexical(Leaf::Literal("the")),
                            lexical(Leaf::Literal("number")),
                            lexical(Leaf::Literal("of")),
                            Child::Node(NodeId(5)),
                        ],
                    }],
                },
                PackedNode {
                    rule: RuleId::NounPhrasePronoun,
                    start: 0,
                    end: 0,
                    families: vec![Family {
                        children: vec![lexical(Leaf::Pronoun(Pronoun::You))],
                    }],
                },
            ],
            vec![NodeId(0), NodeId(1)],
        );
        let context = context("Context Card");
        let mut state =
            MaterializationStateFor::<BuildValue, Construction, Category, Lexical, Leaf>::default();
        let clause = Clause::Where(WhereClause {
            variable: Variable::X,
            value: NounPhrase::Pronoun(PronounNp { word: Pronoun::You }),
        });
        let base = Sentence::Imperative(Imperative {
            predicate: VerbPhrase::Connive(Connive),
        });
        let once = Sentence::WithWhere(
            WithWhere::new(Box::new(base), clause.clone()).expect("Where clause is valid"),
        );

        let first = materialize_node(&forest, NodeId(0), &context, &mut state);
        assert_eq!(first.values.len(), 1);
        assert_eq!(first.values[0].value, BuildValue::Sentence(once.clone()));

        let later = materialize_node(&forest, NodeId(1), &context, &mut state);
        assert_eq!(later.values.len(), 1);
        assert_eq!(
            later.values[0].value,
            BuildValue::Sentence(Sentence::WithWhere(
                WithWhere::new(Box::new(once), clause).expect("Where clause is valid"),
            ))
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
                RulePosition::Lexical(Lexical::Declaration(DeclarationMatcher {
                    kind: DeclarationKind::KeywordAction,
                    name: "Connive",
                    position: GrammarPosition::Verb,
                    feature: FeatureConstraint::Any,
                })),
                RulePosition::Lexical(Lexical::Literal("where")),
                RulePosition::Lexical(Lexical::Variable),
                RulePosition::Lexical(Lexical::Verb(
                    VerbLexeme::Be,
                    FeatureConstraint::Exact(Agreement::ThirdPersonSingular),
                )),
                RulePosition::Lexical(Lexical::Literal("the")),
                RulePosition::Lexical(Lexical::Literal("number")),
                RulePosition::Lexical(Lexical::Literal("of")),
                RulePosition::Nonterminal(Category::NounPhrase),
                RulePosition::Lexical(Lexical::Pronoun),
                RulePosition::Lexical(Lexical::Literal("where")),
                RulePosition::Lexical(Lexical::Variable),
                RulePosition::Lexical(Lexical::Verb(
                    VerbLexeme::Be,
                    FeatureConstraint::Exact(Agreement::ThirdPersonSingular),
                )),
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
            let environment = canonical_test_environment();
            let candidates = materialize(&forest, &context, &environment);
            assert_eq!(candidates.len(), 1, "unexpected candidates for {text:?}");
            assert_eq!(candidates[0].ability.render(&context, &environment), text);
        }
    }
    #[test]
    fn materialize_accepts_a_plural_count_subject_with_a_bare_verb() {
        let text = "Creatures you control with power 2 or less gain X life.";
        let forest = slice_candidates(text, "Context Card").expect("scanner accepts words");
        let context = context("Context Card");
        let environment = canonical_test_environment();
        let candidates = materialize(&forest, &context, &environment);
        assert_eq!(candidates.len(), 1);
        assert_eq!(candidates[0].ability.render(&context, &environment), text);
    }

    #[test]
    fn materialize_preserves_preorder_constructions_and_declared_positions() {
        let forest = slice_candidates("Destroy target creature.", "Context Card").unwrap();
        let environment = canonical_test_environment();
        let candidates = materialize(&forest, &context("Context Card"), &environment);

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
                RulePosition::Lexical(Lexical::Declaration(DeclarationMatcher {
                    kind: DeclarationKind::KeywordAction,
                    name: "Destroy",
                    position: GrammarPosition::Verb,
                    feature: FeatureConstraint::Any,
                })),
                RulePosition::Nonterminal(Category::NounPhrase),
                RulePosition::Lexical(Lexical::Literal("target")),
                RulePosition::Lexical(Lexical::Noun(FeatureConstraint::Exact(Number::Singular))),
            ]
        );
    }

    #[test]
    fn materialize_selected_provenance_is_surface_ordered() {
        let forest = slice_candidates("Destroy target creature.", "Context Card").unwrap();
        let environment = canonical_test_environment();
        let candidates = materialize(&forest, &context("Context Card"), &environment);

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
                    "form:target/target/0",
                ),
                (
                    crate::parser::TextSpan { start: 14, end: 23 },
                    "lexeme:type/Creature/singular",
                ),
                (
                    crate::parser::TextSpan { start: 23, end: 24 },
                    "root:Ability/punctuation",
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
                        && matches!(claim.value, Leaf::Declaration(_))
                        && claim.span == (crate::parser::TextSpan { start: 0, end: 7 })
                    {
                        claim.owner = LexicalOwnerTemplate::Vocab {
                            declaration: "Article",
                        }
                        .instantiate(&claim.value);
                        removed_span = Some(claim.span);
                    }
                }
            }
        }
        let removed_span = removed_span.expect("real Destroy family child carried an owner");
        let forest = Forest::from_test_parts(nodes, roots);
        let context = context("Context Card");
        let environment = canonical_test_environment();
        let candidates = materialize(&forest, &context, &environment);
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

    fn ability_forest_with_duplicate_root_cycles()
    -> Forest<RuleId, Leaf, crate::constructions::LexicalOwner> {
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
            rule: RuleId::SentenceImperative,
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
        Forest::from_test_parts(nodes, vec![root, root])
    }

    #[test]
    fn parser_trace_cycle_pruning_is_deduplicated_bounded_and_semantically_inert() {
        let context = context("Context Card");
        let environment = canonical_test_environment();
        for limit in [0, 1, 8] {
            let forest = ability_forest_with_duplicate_root_cycles();
            let ordinary_candidates = materialize(&forest, &context, &environment);
            let (candidates, materialization) =
                materialize_observed(&forest, &context, &environment, TraceLimits::new(limit));
            let (repeated_candidates, repeated_materialization) =
                materialize_observed(&forest, &context, &environment, TraceLimits::new(limit));
            assert_eq!(ordinary_candidates, candidates);
            assert_eq!(candidates, repeated_candidates);
            assert_eq!(materialization, repeated_materialization);
            let expected_debug = format!("{:?}", candidates[0].ability);
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
                assert_eq!(cycle.node_ordinal(), 3);
                assert_eq!(cycle.construction_path().total(), 2);
                assert_eq!(cycle.construction_path().shown(), usize::min(limit, 2));
                assert_eq!(
                    cycle.construction_path().items(),
                    &["AbilitySpell".to_owned(), "SentenceImperative".to_owned(),]
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
        let forest = Forest::from_test_parts(
            vec![PackedNode {
                rule: RuleId::AbilitySpell,
                start: 0,
                end: 0,
                families: vec![Family {
                    children: vec![Child::Node(NodeId(0))],
                }],
            }],
            vec![NodeId(0)],
        );
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
            "validated chart root did not materialize"
        );
        assert_eq!(trace.materialization_cycles().total(), 1);
        assert_eq!(
            trace.into_parse_result(),
            Err(crate::parser::ParseError::ValidatedRootDidNotMaterialize)
        );
    }
}
