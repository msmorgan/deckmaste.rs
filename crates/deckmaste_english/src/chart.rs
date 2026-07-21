use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;
use std::hash::Hash;

use crate::forest::ForestSymbol;
use crate::forest::NodeId;
use crate::forest::NodeKey;
use crate::forest::PackedAlternative;
use crate::forest::ParseCost;
use crate::forest::ParseForest;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd)]
pub(crate) struct RuleId(usize);

impl RuleId {
    pub(crate) const fn new(index: usize) -> Self {
        Self(index)
    }

    pub(crate) const fn index(self) -> usize {
        self.0
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub(crate) enum Expected<N, L> {
    Nonterminal(N),
    Lexical(L),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct Rule<N, L> {
    pub(crate) lhs: N,
    pub(crate) rhs: Vec<Expected<N, L>>,
    pub(crate) local_cost: ParseCost,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct LexicalMatch<F, M> {
    pub(crate) end: usize,
    pub(crate) features: F,
    pub(crate) meaning: M,
    pub(crate) local_cost: ParseCost,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct Reduction<F, M> {
    pub(crate) features: F,
    pub(crate) meaning: M,
    pub(crate) local_cost: ParseCost,
}

pub(crate) trait Grammar {
    type Nonterminal: Clone + Copy + Eq + Hash;
    type LexicalSlot: Clone + Copy + Eq + Hash;
    type Token;
    type Features: Clone + Eq + Hash;
    type Meaning: Clone + Eq + Hash;

    fn start(&self) -> Self::Nonterminal;
    fn rules(&self) -> &[Rule<Self::Nonterminal, Self::LexicalSlot>];
    fn rules_for(&self, lhs: Self::Nonterminal) -> &[RuleId];
    fn scan(
        &self,
        slot: Self::LexicalSlot,
        tokens: &[Self::Token],
        start: usize,
    ) -> Vec<LexicalMatch<Self::Features, Self::Meaning>>;
    fn reduce(
        &self,
        rule: RuleId,
        children: &[Child<'_, Self>],
    ) -> Option<Reduction<Self::Features, Self::Meaning>>;
}

#[derive(Debug, Clone, Copy)]
pub(crate) struct Child<'a, G>
where
    G: Grammar + ?Sized,
{
    #[allow(
        dead_code,
        reason = "typed English meaning reducers consume child identity during migration"
    )]
    pub(crate) node: NodeId,
    #[cfg(test)]
    pub(crate) symbol: &'a ForestSymbol<G::Nonterminal, G::LexicalSlot>,
    pub(crate) features: &'a G::Features,
    pub(crate) meaning: &'a G::Meaning,
}

#[derive(Debug, Clone)]
pub(crate) struct ChartResult<N, L, F, M> {
    pub(crate) forest: ParseForest<N, L, F, M>,
    pub(crate) roots: Vec<NodeId>,
}

type GrammarChartResult<G> = ChartResult<
    <G as Grammar>::Nonterminal,
    <G as Grammar>::LexicalSlot,
    <G as Grammar>::Features,
    <G as Grammar>::Meaning,
>;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum GrammarError {
    EmptyRule(RuleId),
    MissingRuleIndex(RuleId),
    MismatchedRuleIndex(RuleId),
    ZeroLengthCycle,
    InvalidLexicalMatch { start: usize, end: usize },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct ItemKey {
    rule: RuleId,
    dot: usize,
    origin: usize,
    partial: PartialDerivationId,
}

#[derive(Debug, Clone, Default)]
struct ChartColumn {
    seen: HashSet<ItemKey>,
    items: Vec<ItemKey>,
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
struct PartialDerivationId(usize);

impl PartialDerivationId {
    const EMPTY: Self = Self(0);
}

#[derive(Debug, Clone, Copy)]
struct PartialDerivation {
    previous: PartialDerivationId,
    child: NodeId,
    len: usize,
}

#[derive(Debug, Clone, Default)]
struct PartialDerivations {
    derivations: Vec<PartialDerivation>,
    ids: HashMap<(PartialDerivationId, NodeId), PartialDerivationId>,
}

impl PartialDerivations {
    fn new() -> Self {
        Self::default()
    }

    #[cfg(test)]
    const fn empty() -> PartialDerivationId {
        PartialDerivationId::EMPTY
    }

    fn push(&mut self, previous: PartialDerivationId, child: NodeId) -> PartialDerivationId {
        if let Some(&derivation) = self.ids.get(&(previous, child)) {
            return derivation;
        }
        let len = if previous == PartialDerivationId::EMPTY {
            1
        } else {
            self.derivations[previous.0 - 1].len + 1
        };
        let derivation = PartialDerivationId(self.derivations.len() + 1);
        self.derivations.push(PartialDerivation {
            previous,
            child,
            len,
        });
        self.ids.insert((previous, child), derivation);
        derivation
    }

    fn materialize(&self, mut derivation: PartialDerivationId) -> Vec<NodeId> {
        if derivation == PartialDerivationId::EMPTY {
            return Vec::new();
        }
        let mut children = Vec::with_capacity(self.derivations[derivation.0 - 1].len);
        while derivation != PartialDerivationId::EMPTY {
            let current = self.derivations[derivation.0 - 1];
            children.push(current.child);
            derivation = current.previous;
        }
        children.reverse();
        children
    }
}

pub(crate) fn parse_chart<G>(
    grammar: &G,
    tokens: &[G::Token],
) -> Result<GrammarChartResult<G>, GrammarError>
where
    G: Grammar,
{
    validate_grammar(grammar)?;
    ChartParser::new(grammar, tokens).run()
}

#[derive(Debug, Clone, Copy)]
struct LexicalEdge {
    end: usize,
    node: NodeId,
}

type GrammarForest<G> = ParseForest<
    <G as Grammar>::Nonterminal,
    <G as Grammar>::LexicalSlot,
    <G as Grammar>::Features,
    <G as Grammar>::Meaning,
>;

struct ChartParser<'grammar, 'tokens, G>
where
    G: Grammar,
{
    grammar: &'grammar G,
    tokens: &'tokens [G::Token],
    forest: GrammarForest<G>,
    chart: Vec<ChartColumn>,
    agenda: VecDeque<(usize, ItemKey)>,
    completed: HashMap<(usize, G::Nonterminal), Vec<NodeId>>,
    waiting: HashMap<(usize, G::Nonterminal), Vec<ItemKey>>,
    partials: PartialDerivations,
    scans: HashMap<(G::LexicalSlot, usize), Vec<LexicalEdge>>,
}

impl<'grammar, 'tokens, G> ChartParser<'grammar, 'tokens, G>
where
    G: Grammar,
{
    fn new(grammar: &'grammar G, tokens: &'tokens [G::Token]) -> Self {
        Self {
            grammar,
            tokens,
            forest: ParseForest::new(),
            chart: vec![ChartColumn::default(); tokens.len() + 1],
            agenda: VecDeque::new(),
            completed: HashMap::new(),
            waiting: HashMap::new(),
            partials: PartialDerivations::new(),
            scans: HashMap::new(),
        }
    }

    fn run(mut self) -> Result<GrammarChartResult<G>, GrammarError> {
        self.seed();
        while let Some((position, item)) = self.agenda.pop_front() {
            self.process_item(position, item)?;
        }

        let roots = self
            .completed
            .get(&(0, self.grammar.start()))
            .into_iter()
            .flatten()
            .copied()
            .filter(|&root| self.forest.node(root).key.end == self.tokens.len())
            .collect();
        Ok(ChartResult {
            forest: self.forest,
            roots,
        })
    }

    fn seed(&mut self) {
        for &rule in self.grammar.rules_for(self.grammar.start()) {
            enqueue(
                &mut self.chart,
                &mut self.agenda,
                0,
                ItemKey {
                    rule,
                    dot: 0,
                    origin: 0,
                    partial: PartialDerivationId::EMPTY,
                },
            );
        }
    }

    fn process_item(&mut self, position: usize, item: ItemKey) -> Result<(), GrammarError> {
        let rule = &self.grammar.rules()[item.rule.index()];
        let lhs = rule.lhs;
        let local_cost = rule.local_cost;
        match rule.rhs.get(item.dot).copied() {
            None => {
                self.complete_item(position, item, lhs, local_cost);
                Ok(())
            }
            Some(Expected::Nonterminal(nonterminal)) => {
                self.predict_and_advance(position, item, nonterminal);
                Ok(())
            }
            Some(Expected::Lexical(slot)) => self.scan_and_advance(position, item, slot),
        }
    }

    fn complete_item(
        &mut self,
        position: usize,
        item: ItemKey,
        lhs: G::Nonterminal,
        rule_cost: ParseCost,
    ) {
        let child_ids = self.partials.materialize(item.partial);
        let reduction = {
            let children = child_ids
                .iter()
                .map(|&child| {
                    let node = self.forest.node(child);
                    Child {
                        node: child,
                        #[cfg(test)]
                        symbol: &node.key.symbol,
                        features: &node.key.features,
                        meaning: &node.key.meaning,
                    }
                })
                .collect::<Vec<_>>();
            self.grammar.reduce(item.rule, &children)
        };
        let Some(reduction) = reduction else {
            return;
        };
        let interned = self.forest.intern_node(
            NodeKey {
                symbol: ForestSymbol::Nonterminal(lhs),
                start: item.origin,
                end: position,
                features: reduction.features,
                meaning: reduction.meaning,
            },
            PackedAlternative {
                rule: Some(item.rule),
                children: child_ids,
                local_cost: rule_cost + reduction.local_cost,
            },
        );
        if !interned.node_was_new {
            return;
        }

        self.completed
            .entry((item.origin, lhs))
            .or_default()
            .push(interned.node);
        let Some(waiters) = self.waiting.get(&(item.origin, lhs)) else {
            return;
        };
        for &waiter in waiters {
            let mut waiter = waiter;
            waiter.dot += 1;
            waiter.partial = self.partials.push(waiter.partial, interned.node);
            enqueue(&mut self.chart, &mut self.agenda, position, waiter);
        }
    }

    fn predict_and_advance(&mut self, position: usize, item: ItemKey, nonterminal: G::Nonterminal) {
        self.waiting
            .entry((position, nonterminal))
            .or_default()
            .push(item);
        for &predicted_rule in self.grammar.rules_for(nonterminal) {
            enqueue(
                &mut self.chart,
                &mut self.agenda,
                position,
                ItemKey {
                    rule: predicted_rule,
                    dot: 0,
                    origin: position,
                    partial: PartialDerivationId::EMPTY,
                },
            );
        }

        if let Some(existing) = self.completed.get(&(position, nonterminal)) {
            for &child in existing {
                let end = self.forest.node(child).key.end;
                let mut advanced = item;
                advanced.dot += 1;
                advanced.partial = self.partials.push(advanced.partial, child);
                enqueue(&mut self.chart, &mut self.agenda, end, advanced);
            }
        }
    }

    fn scan_and_advance(
        &mut self,
        position: usize,
        item: ItemKey,
        slot: G::LexicalSlot,
    ) -> Result<(), GrammarError> {
        for edge in self.lexical_edges(slot, position)? {
            let mut advanced = item;
            advanced.dot += 1;
            advanced.partial = self.partials.push(advanced.partial, edge.node);
            enqueue(&mut self.chart, &mut self.agenda, edge.end, advanced);
        }
        Ok(())
    }

    fn lexical_edges(
        &mut self,
        slot: G::LexicalSlot,
        position: usize,
    ) -> Result<Vec<LexicalEdge>, GrammarError> {
        if let Some(edges) = self.scans.get(&(slot, position)) {
            return Ok(edges.clone());
        }

        let mut edges = Vec::new();
        for lexical_match in self.grammar.scan(slot, self.tokens, position) {
            if lexical_match.end <= position || lexical_match.end > self.tokens.len() {
                return Err(GrammarError::InvalidLexicalMatch {
                    start: position,
                    end: lexical_match.end,
                });
            }
            let interned = self.forest.intern_node(
                NodeKey {
                    symbol: ForestSymbol::Lexical(slot),
                    start: position,
                    end: lexical_match.end,
                    features: lexical_match.features,
                    meaning: lexical_match.meaning,
                },
                PackedAlternative {
                    rule: None,
                    children: Vec::new(),
                    local_cost: lexical_match.local_cost,
                },
            );
            edges.push(LexicalEdge {
                end: lexical_match.end,
                node: interned.node,
            });
        }
        self.scans.insert((slot, position), edges.clone());
        Ok(edges)
    }
}

fn enqueue(
    chart: &mut [ChartColumn],
    agenda: &mut VecDeque<(usize, ItemKey)>,
    position: usize,
    item: ItemKey,
) {
    if chart[position].seen.insert(item) {
        chart[position].items.push(item);
        agenda.push_back((position, item));
    }
}

fn validate_grammar<G: Grammar>(grammar: &G) -> Result<(), GrammarError> {
    let rules = grammar.rules();
    let mut unit_edges = HashMap::<G::Nonterminal, Vec<G::Nonterminal>>::new();
    let mut nonterminals = HashSet::from([grammar.start()]);
    for (index, rule) in rules.iter().enumerate() {
        let rule_id = RuleId::new(index);
        if rule.rhs.is_empty() {
            return Err(GrammarError::EmptyRule(rule_id));
        }
        if !grammar.rules_for(rule.lhs).contains(&rule_id) {
            return Err(GrammarError::MissingRuleIndex(rule_id));
        }
        nonterminals.insert(rule.lhs);
        nonterminals.extend(rule.rhs.iter().filter_map(|expected| match expected {
            Expected::Nonterminal(nonterminal) => Some(*nonterminal),
            Expected::Lexical(_) => None,
        }));
        if let [Expected::Nonterminal(target)] = rule.rhs.as_slice() {
            unit_edges.entry(rule.lhs).or_default().push(*target);
        }
    }
    for nonterminal in nonterminals {
        for &rule_id in grammar.rules_for(nonterminal) {
            if rules.get(rule_id.index()).is_none() {
                return Err(GrammarError::MissingRuleIndex(rule_id));
            }
            if rules[rule_id.index()].lhs != nonterminal {
                return Err(GrammarError::MismatchedRuleIndex(rule_id));
            }
        }
    }

    let mut visiting = HashSet::new();
    let mut visited = HashSet::new();
    for &node in unit_edges.keys() {
        if unit_graph_has_cycle(node, &unit_edges, &mut visiting, &mut visited) {
            return Err(GrammarError::ZeroLengthCycle);
        }
    }
    Ok(())
}

fn unit_graph_has_cycle<N>(
    node: N,
    edges: &HashMap<N, Vec<N>>,
    visiting: &mut HashSet<N>,
    visited: &mut HashSet<N>,
) -> bool
where
    N: Clone + Copy + Eq + Hash,
{
    if visited.contains(&node) {
        return false;
    }
    if !visiting.insert(node) {
        return true;
    }
    if edges.get(&node).is_some_and(|targets| {
        targets
            .iter()
            .any(|&target| unit_graph_has_cycle(target, edges, visiting, visited))
    }) {
        return true;
    }
    visiting.remove(&node);
    visited.insert(node);
    false
}

#[cfg(test)]
mod tests {
    use std::cell::RefCell;
    use std::collections::HashMap;

    use super::Child;
    use super::Expected;
    use super::Grammar;
    use super::LexicalMatch;
    use super::PartialDerivations;
    use super::Reduction;
    use super::Rule;
    use super::RuleId;
    use super::parse_chart;
    use crate::forest::NodeId;
    use crate::forest::ParseCost;

    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    enum N {
        Start,
        List,
        Atom,
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    enum L {
        A,
        And,
        Each,
        Other,
    }

    #[derive(Debug, Clone, PartialEq, Eq, Hash)]
    enum Meaning {
        Atom(&'static str),
        List,
        EachOther,
    }

    struct TestGrammar {
        rules: Vec<Rule<N, L>>,
        rules_by_lhs: HashMap<N, Vec<RuleId>>,
        scans: RefCell<HashMap<(L, usize), usize>>,
        reduced_children: RefCell<Vec<NodeId>>,
    }

    impl TestGrammar {
        fn new() -> Self {
            let rules = vec![
                rule(N::Start, vec![Expected::Nonterminal(N::List)]),
                rule(N::List, vec![Expected::Nonterminal(N::Atom)]),
                rule(
                    N::List,
                    vec![
                        Expected::Nonterminal(N::List),
                        Expected::Lexical(L::And),
                        Expected::Nonterminal(N::Atom),
                    ],
                ),
                rule(
                    N::List,
                    vec![
                        Expected::Nonterminal(N::List),
                        Expected::Nonterminal(N::Atom),
                    ],
                ),
                rule(N::Atom, vec![Expected::Lexical(L::A)]),
                rule(N::Atom, vec![Expected::Lexical(L::Each)]),
                rule(N::Atom, vec![Expected::Lexical(L::Other)]),
            ];
            let mut rules_by_lhs = HashMap::<N, Vec<RuleId>>::new();
            for (index, rule) in rules.iter().enumerate() {
                rules_by_lhs
                    .entry(rule.lhs)
                    .or_default()
                    .push(RuleId::new(index));
            }
            Self {
                rules,
                rules_by_lhs,
                scans: RefCell::new(HashMap::new()),
                reduced_children: RefCell::new(Vec::new()),
            }
        }
    }

    fn rule(lhs: N, rhs: Vec<Expected<N, L>>) -> Rule<N, L> {
        Rule {
            lhs,
            rhs,
            local_cost: ParseCost::default(),
        }
    }

    impl Grammar for TestGrammar {
        type Features = ();
        type LexicalSlot = L;
        type Meaning = Meaning;
        type Nonterminal = N;
        type Token = &'static str;

        fn start(&self) -> Self::Nonterminal {
            N::Start
        }

        fn rules(&self) -> &[Rule<Self::Nonterminal, Self::LexicalSlot>] {
            &self.rules
        }

        fn rules_for(&self, lhs: Self::Nonterminal) -> &[RuleId] {
            self.rules_by_lhs.get(&lhs).map_or(&[], Vec::as_slice)
        }

        fn scan(
            &self,
            slot: Self::LexicalSlot,
            tokens: &[Self::Token],
            start: usize,
        ) -> Vec<LexicalMatch<Self::Features, Self::Meaning>> {
            *self.scans.borrow_mut().entry((slot, start)).or_default() += 1;
            let Some(token) = tokens.get(start) else {
                return Vec::new();
            };
            let meaning = match (slot, *token) {
                (L::A, "a") => Meaning::Atom("a"),
                (L::And, "and") => Meaning::Atom("and"),
                (L::Each, "each") => Meaning::Atom("each"),
                (L::Other, "other") => Meaning::Atom("other"),
                _ => return Vec::new(),
            };
            let mut matches = vec![LexicalMatch {
                end: start + 1,
                features: (),
                meaning,
                local_cost: ParseCost::default(),
            }];
            if slot == L::Each && tokens.get(start + 1) == Some(&"other") {
                matches.push(LexicalMatch {
                    end: start + 2,
                    features: (),
                    meaning: Meaning::EachOther,
                    local_cost: ParseCost::default(),
                });
            }
            matches
        }

        fn reduce(
            &self,
            rule: RuleId,
            children: &[Child<'_, Self>],
        ) -> Option<Reduction<Self::Features, Self::Meaning>> {
            if let Some(first) = children.first() {
                let _ = (first.symbol, first.features);
                self.reduced_children.borrow_mut().push(first.node);
            }
            let meaning = match rule.index() {
                0..=3 => Meaning::List,
                4..=6 => children.first()?.meaning.clone(),
                _ => return None,
            };
            Some(Reduction {
                features: (),
                meaning,
                local_cost: ParseCost::default(),
            })
        }
    }

    #[test]
    fn partial_derivations_share_prefixes_and_materialize_once() {
        let mut partials = PartialDerivations::new();
        let empty = PartialDerivations::empty();
        let first_child = NodeId::new(7);
        let second_child = NodeId::new(11);
        let sibling_child = NodeId::new(13);

        let prefix = partials.push(empty, first_child);
        let complete = partials.push(prefix, second_child);
        let sibling = partials.push(prefix, sibling_child);

        assert_eq!(partials.push(empty, first_child), prefix);
        assert_eq!(
            partials.materialize(complete),
            vec![first_child, second_child]
        );
        assert_eq!(
            partials.materialize(sibling),
            vec![first_child, sibling_child]
        );
    }

    #[test]
    fn child_node_identity_is_available_to_reducers() {
        let grammar = TestGrammar::new();

        parse_chart(&grammar, &["a"]).expect("the artificial grammar parses one atom");

        assert!(!grammar.reduced_children.borrow().is_empty());
    }

    #[test]
    fn left_recursive_list_terminates_and_spans_the_input() {
        let grammar = TestGrammar::new();
        let tokens = ["a", "and", "a", "and", "a"];

        let result = parse_chart(&grammar, &tokens).expect("grammar and scans are valid");

        assert_eq!(result.roots.len(), 1);
        let root = result.forest.node(result.roots[0]);
        assert_eq!((root.key.start, root.key.end), (0, tokens.len()));
        assert_eq!(root.key.meaning, Meaning::List);
    }

    #[test]
    fn multiword_and_component_analyses_coexist_and_scans_are_memoized() {
        let grammar = TestGrammar::new();

        let result = parse_chart(&grammar, &["each", "other"])
            .expect("the ambiguous input has complete derivations");

        assert!(result.forest.nodes().any(|node| {
            node.key.symbol == crate::forest::ForestSymbol::Lexical(L::Each)
                && (node.key.start, node.key.end) == (0, 2)
                && node.key.meaning == Meaning::EachOther
        }));
        assert!(result.forest.nodes().any(|node| {
            node.key.symbol == crate::forest::ForestSymbol::Lexical(L::Each)
                && (node.key.start, node.key.end) == (0, 1)
                && node.key.meaning == Meaning::Atom("each")
        }));
        assert!(result.forest.nodes().any(|node| {
            node.key.symbol == crate::forest::ForestSymbol::Lexical(L::Other)
                && (node.key.start, node.key.end) == (1, 2)
        }));
        let packed_list = result
            .forest
            .nodes()
            .find(|node| {
                node.key.symbol == crate::forest::ForestSymbol::Nonterminal(N::List)
                    && (node.key.start, node.key.end) == (0, 2)
                    && node.key.meaning == Meaning::List
            })
            .expect("both analyses lower to the same list node");
        assert_eq!(packed_list.alternatives.len(), 2);
        assert!(grammar.scans.borrow().values().all(|&calls| calls == 1));
    }

    #[test]
    fn empty_rules_are_rejected() {
        let mut grammar = TestGrammar::new();
        grammar.rules[0].rhs.clear();

        let error = parse_chart(&grammar, &[]).expect_err("empty rules are nullable");

        assert_eq!(error, super::GrammarError::EmptyRule(RuleId::new(0)));
    }

    #[test]
    fn zero_length_unit_cycles_are_rejected() {
        let mut grammar = TestGrammar::new();
        grammar.rules = vec![
            rule(N::Start, vec![Expected::Nonterminal(N::List)]),
            rule(N::List, vec![Expected::Nonterminal(N::Start)]),
        ];
        grammar.rules_by_lhs.clear();
        grammar.rules_by_lhs.insert(N::Start, vec![RuleId::new(0)]);
        grammar.rules_by_lhs.insert(N::List, vec![RuleId::new(1)]);

        let error = parse_chart(&grammar, &[]).expect_err("the forest would be cyclic");

        assert_eq!(error, super::GrammarError::ZeroLengthCycle);
    }
}
