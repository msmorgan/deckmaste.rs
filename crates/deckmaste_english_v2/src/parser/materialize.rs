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
use super::selection::specificity_tiers;
use crate::ast::Ability;
use crate::ast::Amount;
use crate::ast::Clause;
use crate::ast::NounPhrase;
use crate::ast::Sentence;
use crate::ast::VerbPhrase;
use crate::constructions::Agreement;
use crate::constructions::Category;
use crate::constructions::Construction;
use crate::constructions::Leaf;
use crate::constructions::Lexical;
use crate::constructions::LexicalTerminal;
use crate::constructions::RULES;
use crate::constructions::RuleId;
use crate::constructions::build;
use crate::context::ParseContext;
use crate::render::Render;

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum BuildValue {
    Ability(Ability),
    Sentence(Sentence),
    Clause(Clause),
    NounPhrase(NounPhrase, Agreement),
    VerbPhrase(VerbPhrase, Agreement),
    Amount(Amount),
    Leaf(Leaf),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) struct MaterializedCandidate<V, C> {
    pub(super) value: V,
    pub(super) constructions: Vec<C>,
    pub(super) positions: Vec<RulePosition<Category, Lexical>>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct Candidate {
    pub ability: Ability,
    pub constructions: Vec<Construction>,
    pub positions: Vec<RulePosition<Category, Lexical>>,
    pub specificity: Vec<super::SpecificityTier>,
}

trait MaterializationObservation<C> {
    const ENABLED: bool;
    fn cycle_pruned(&mut self, _node_id: NodeId, _construction_path: &[C]) {}
}

impl<C> MaterializationObservation<C> for () {
    const ENABLED: bool = false;
}

impl MaterializationObservation<Construction> for MaterializationTraceBuilder {
    const ENABLED: bool = true;

    fn cycle_pruned(&mut self, node_id: NodeId, construction_path: &[Construction]) {
        self.record_cycle(node_id.0, construction_path);
    }
}

struct MaterializationStateFor<V, C> {
    memo: BTreeMap<NodeId, Vec<MaterializedCandidate<V, C>>>,
    in_progress: BTreeSet<NodeId>,
}

impl<V, C> Default for MaterializationStateFor<V, C> {
    fn default() -> Self {
        Self {
            memo: BTreeMap::new(),
            in_progress: BTreeSet::new(),
        }
    }
}

struct MaterializationOutcomeFor<V, C> {
    values: Vec<MaterializedCandidate<V, C>>,
    cycle_pruned: bool,
}

type MaterializationState = MaterializationStateFor<BuildValue, Construction>;
#[cfg(test)]
type MaterializationOutcome = MaterializationOutcomeFor<BuildValue, Construction>;

struct MaterializationKernel<'a, R, T, V, C, L: 'static, Build> {
    rules: &'a [Rule<Category, L, R>],
    rule_index: fn(R) -> usize,
    construction: fn(R) -> C,
    lexical_matcher: fn(L) -> Lexical,
    build_leaf: fn(&T) -> V,
    build: Build,
}

impl<R, T, V, C, L, Build> MaterializationKernel<'_, R, T, V, C, L, Build>
where
    R: Copy,
    V: Clone + PartialEq,
    C: Copy + PartialEq,
    L: Copy + 'static,
    Build: Fn(R, &[V]) -> Option<V>,
{
    fn materialize<O>(
        &self,
        forest: &Forest<R, T>,
        observation: &mut O,
    ) -> Vec<MaterializedCandidate<V, C>>
    where
        O: MaterializationObservation<C>,
    {
        let mut candidates = Vec::new();
        let mut state = MaterializationStateFor::default();
        let mut construction_path = Vec::new();
        for root in forest.accepted_root_ids() {
            for built in self
                .materialize_node(
                    forest,
                    root,
                    &mut state,
                    &mut construction_path,
                    observation,
                )
                .values
            {
                push_unique(&mut candidates, built);
            }
        }
        candidates
    }

    fn materialize_node<O>(
        &self,
        forest: &Forest<R, T>,
        node_id: NodeId,
        state: &mut MaterializationStateFor<V, C>,
        construction_path: &mut Vec<C>,
        observation: &mut O,
    ) -> MaterializationOutcomeFor<V, C>
    where
        O: MaterializationObservation<C>,
    {
        if let Some(values) = state.memo.get(&node_id) {
            return MaterializationOutcomeFor {
                values: values.clone(),
                cycle_pruned: false,
            };
        }
        if !state.in_progress.insert(node_id) {
            observation.cycle_pruned(node_id, construction_path);
            return MaterializationOutcomeFor {
                values: Vec::new(),
                cycle_pruned: true,
            };
        }

        let node = forest.node(node_id);
        let mut values = Vec::new();
        let mut cycle_pruned = false;
        for family in &node.families {
            let outcome = self.materialize_family(
                forest,
                node.rule,
                family,
                state,
                construction_path,
                observation,
            );
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

    fn materialize_family<O>(
        &self,
        forest: &Forest<R, T>,
        rule_id: R,
        family: &Family<T>,
        state: &mut MaterializationStateFor<V, C>,
        construction_path: &mut Vec<C>,
        observation: &mut O,
    ) -> MaterializationOutcomeFor<V, C>
    where
        O: MaterializationObservation<C>,
    {
        let rule = &self.rules[(self.rule_index)(rule_id)];
        let construction = (self.construction)(rule_id);
        if O::ENABLED {
            construction_path.push(construction);
        }
        let mut combinations = vec![Vec::new()];
        let mut cycle_pruned = false;
        for child in &family.children {
            let child_values = match child {
                Child::Node(id) => {
                    let outcome =
                        self.materialize_node(forest, *id, state, construction_path, observation);
                    cycle_pruned |= outcome.cycle_pruned;
                    outcome.values
                }
                Child::Lexical(leaf) => vec![MaterializedCandidate {
                    value: (self.build_leaf)(leaf),
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
            if let Some(value) = (self.build)(rule_id, &child_values) {
                let mut constructions = vec![construction];
                let mut positions = rule
                    .rhs
                    .iter()
                    .map(|position| match *position {
                        RulePosition::Nonterminal(category) => RulePosition::Nonterminal(category),
                        RulePosition::Lexical(lexical) => {
                            RulePosition::Lexical((self.lexical_matcher)(lexical))
                        }
                    })
                    .collect::<Vec<_>>();
                for child in children {
                    constructions.extend(child.constructions);
                    positions.extend(child.positions);
                }
                push_unique(
                    &mut values,
                    MaterializedCandidate {
                        value,
                        constructions,
                        positions,
                    },
                );
            }
        }
        if O::ENABLED {
            construction_path.pop();
        }
        MaterializationOutcomeFor {
            values,
            cycle_pruned,
        }
    }
}

#[cfg(test)]
pub(super) fn materialize_with<R, T, V, C, Build>(
    forest: &Forest<R, T>,
    rules: &[Rule<Category, Lexical, R>],
    rule_index: fn(R) -> usize,
    construction: fn(R) -> C,
    build_leaf: fn(&T) -> V,
    build: Build,
) -> Vec<MaterializedCandidate<V, C>>
where
    R: Copy,
    V: Clone + PartialEq,
    C: Copy + PartialEq,
    Build: Fn(R, &[V]) -> Option<V>,
{
    MaterializationKernel {
        rules,
        rule_index,
        construction,
        lexical_matcher: std::convert::identity,
        build_leaf,
        build,
    }
    .materialize(forest, &mut ())
}

pub(crate) fn materialize(
    forest: &Forest<RuleId, Leaf>,
    context: &ParseContext<'_>,
    environment: &crate::environment::ParserEnvironment,
) -> Vec<Candidate> {
    let built = MaterializationKernel {
        rules: RULES,
        rule_index: RuleId::index,
        construction: RuleId::construction,
        lexical_matcher: |terminal: LexicalTerminal| terminal.matcher,
        build_leaf: |leaf: &Leaf| BuildValue::Leaf(leaf.clone()),
        build: |rule: RuleId, children: &[BuildValue]| build(rule, children, context),
    }
    .materialize(forest, &mut ());
    finalize_candidates(built, context, environment, None)
}

pub(crate) fn materialize_observed(
    forest: &Forest<RuleId, Leaf>,
    context: &ParseContext<'_>,
    environment: &crate::environment::ParserEnvironment,
    limits: TraceLimits,
) -> (Vec<Candidate>, MaterializationTrace) {
    let mut observation = MaterializationTraceBuilder::new(limits);
    let built = MaterializationKernel {
        rules: RULES,
        rule_index: RuleId::index,
        construction: RuleId::construction,
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
    built_values: Vec<MaterializedCandidate<BuildValue, Construction>>,
    context: &ParseContext<'_>,
    environment: &crate::environment::ParserEnvironment,
    mut observation: Option<(&mut MaterializationTraceBuilder, TraceLimits)>,
) -> Vec<Candidate> {
    let mut candidates = Vec::new();
    for built in built_values {
        if let BuildValue::Ability(ability) = built.value {
            let candidate = Candidate {
                ability,
                constructions: built.constructions,
                specificity: specificity_tiers(&built.positions),
                positions: built.positions,
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
    candidates
}

#[cfg(test)]
fn materialize_node(
    forest: &Forest<RuleId, Leaf>,
    node_id: NodeId,
    context: &ParseContext<'_>,
    state: &mut MaterializationState,
) -> MaterializationOutcome {
    let kernel: MaterializationKernel<
        '_,
        RuleId,
        Leaf,
        BuildValue,
        Construction,
        LexicalTerminal,
        _,
    > = MaterializationKernel {
        rules: RULES,
        rule_index: RuleId::index,
        construction: RuleId::construction,
        lexical_matcher: |terminal| terminal.matcher,
        build_leaf: |leaf: &Leaf| BuildValue::Leaf(leaf.clone()),
        build: |rule: RuleId, children: &[BuildValue]| build(rule, children, context),
    };
    kernel.materialize_node(forest, node_id, state, &mut Vec::new(), &mut ())
}

pub(super) fn completion_has_checked_build(
    rule: RuleId,
    family: &Family<Leaf>,
    forest: &Forest<RuleId, Leaf>,
    context: &ParseContext<'_>,
) -> bool {
    let kernel: MaterializationKernel<
        '_,
        RuleId,
        Leaf,
        BuildValue,
        Construction,
        LexicalTerminal,
        _,
    > = MaterializationKernel {
        rules: RULES,
        rule_index: RuleId::index,
        construction: RuleId::construction,
        lexical_matcher: |terminal| terminal.matcher,
        build_leaf: |leaf: &Leaf| BuildValue::Leaf(leaf.clone()),
        build: |rule: RuleId, children: &[BuildValue]| build(rule, children, context),
    };
    !kernel
        .materialize_family(
            forest,
            rule,
            family,
            &mut MaterializationState::default(),
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
    use super::MaterializationState;
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
    use crate::catalogs::canonical_test_environment;
    use crate::constructions::Agreement;
    use crate::constructions::DeclarationLeaf;
    use crate::constructions::DeclarationMatcher;
    use crate::constructions::FeatureConstraint;
    use crate::constructions::Number;
    use crate::context::ParseContext;
    use crate::environment::DeclarationId;
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
    use crate::parser::scan::SliceGrammar;
    use crate::parser::scan::parse_forest;
    use crate::render::Render;

    fn slice_candidates(
        text: &str,
        card_name: &str,
    ) -> Result<Forest<RuleId, Leaf>, ChartFailure<Category, Lexical>> {
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
                        children: vec![Child::Lexical(Leaf::Declaration(DeclarationLeaf {
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
                RulePosition::Lexical(Lexical::Declaration(DeclarationMatcher {
                    kind: DeclarationKind::KeywordAction,
                    name: "Connive",
                    position: GrammarPosition::Verb,
                    feature: FeatureConstraint::Any,
                })),
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

    fn ability_forest_with_duplicate_root_cycles() -> Forest<RuleId, Leaf> {
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
            let analysis = crate::parser::analyze_materialized(candidates);
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
