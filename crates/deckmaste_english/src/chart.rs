use std::collections::HashSet;
use std::collections::VecDeque;
use std::hash::Hash;
use std::rc::Rc;

use hashbrown::HashMap;
use hashbrown::hash_map::Entry;

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
pub(crate) struct Reduction<F> {
    pub(crate) features: F,
    pub(crate) local_cost: ParseCost,
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct ChartStats {
    unique_items: usize,
    max_column_width: usize,
}

impl ChartStats {
    #[must_use]
    pub const fn unique_items(self) -> usize {
        self.unique_items
    }

    #[must_use]
    pub const fn max_column_width(self) -> usize {
        self.max_column_width
    }
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
    ) -> Option<Reduction<Self::Features>>;

    /// Assign a cost to one concrete rule-prefix split. Unlike reduction
    /// cost, this stays on the packed intermediate alternative, so grammars
    /// can rank attachment boundaries without splitting chart or forest keys.
    fn intermediate_cost(
        &self,
        _rule: RuleId,
        _completed_children: &[Self::Features],
        _rule_start: usize,
        _latest_child_start: usize,
        _end: usize,
    ) -> ParseCost {
        ParseCost::default()
    }

    fn accepts_prefix(
        &self,
        _rule: RuleId,
        _completed_children: usize,
        _latest_child: &Self::Features,
    ) -> bool {
        true
    }

    fn state_limit(&self) -> Option<usize> {
        None
    }
}

#[derive(Debug, Clone, Copy)]
pub(crate) struct Child<'a, G>
where
    G: Grammar + ?Sized,
{
    pub(crate) features: &'a G::Features,
}

#[derive(Debug, Clone)]
pub(crate) struct ChartResult<N, L, F, M> {
    pub(crate) forest: ParseForest<N, L, F, M>,
    pub(crate) roots: Vec<NodeId>,
    pub(crate) stats: ChartStats,
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
    StateLimitExceeded { limit: usize },
    InvalidLexicalMatch { start: usize, end: usize },
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct ItemKey<F> {
    rule: RuleId,
    dot: usize,
    origin: usize,
    prefix_features: Rc<[F]>,
}

#[derive(Debug, Clone)]
struct ChartColumn<F> {
    seen: HashMap<ItemKey<F>, Option<NodeId>>,
}

impl<F> Default for ChartColumn<F> {
    fn default() -> Self {
        Self {
            seen: HashMap::new(),
        }
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
type WaitingKey<N> = (usize, N);

struct ChartParser<'grammar, 'tokens, G>
where
    G: Grammar,
{
    grammar: &'grammar G,
    tokens: &'tokens [G::Token],
    forest: GrammarForest<G>,
    chart: Vec<ChartColumn<G::Features>>,
    agenda: VecDeque<(usize, ItemKey<G::Features>)>,
    completed: HashMap<(usize, G::Nonterminal), Vec<NodeId>>,
    waiting: HashMap<WaitingKey<G::Nonterminal>, Vec<ItemKey<G::Features>>>,
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
            scans: HashMap::new(),
        }
    }

    fn run(mut self) -> Result<GrammarChartResult<G>, GrammarError> {
        self.seed();
        let mut processed = 0_usize;
        while let Some((position, item)) = self.agenda.pop_front() {
            if let Some(limit) = self.grammar.state_limit()
                && processed >= limit
            {
                return Err(GrammarError::StateLimitExceeded { limit });
            }
            self.process_item(position, &item)?;
            processed += 1;
        }

        let roots = self
            .completed
            .get(&(0, self.grammar.start()))
            .into_iter()
            .flatten()
            .copied()
            .filter(|&root| self.forest.node(root).key.end == self.tokens.len())
            .collect();
        let stats = ChartStats {
            unique_items: self.chart.iter().map(|column| column.seen.len()).sum(),
            max_column_width: self
                .chart
                .iter()
                .map(|column| column.seen.len())
                .max()
                .unwrap_or_default(),
        };
        Ok(ChartResult {
            forest: self.forest,
            roots,
            stats,
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
                    prefix_features: Rc::default(),
                },
                None,
            );
        }
    }

    fn process_item(
        &mut self,
        position: usize,
        item: &ItemKey<G::Features>,
    ) -> Result<(), GrammarError> {
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
        item: &ItemKey<G::Features>,
        lhs: G::Nonterminal,
        rule_cost: ParseCost,
    ) {
        let children = item
            .prefix_features
            .iter()
            .map(|features| Child { features })
            .collect::<Vec<_>>();
        let reduction = self.grammar.reduce(item.rule, &children);
        let Some(reduction) = reduction else {
            return;
        };
        let Some(intermediate) = self.chart[position]
            .seen
            .get(&item.clone())
            .copied()
            .flatten()
        else {
            return;
        };
        let interned = self.forest.intern_node(
            NodeKey::nonterminal(lhs, item.origin, position, reduction.features),
            PackedAlternative {
                rule: Some(item.rule),
                children: vec![intermediate],
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
        let Some(waiters) = self.waiting.get(&(item.origin, lhs)).cloned() else {
            return;
        };
        for waiter in waiters {
            self.advance_with_child(position, item.origin, &waiter, interned.node);
        }
    }

    fn predict_and_advance(
        &mut self,
        position: usize,
        item: &ItemKey<G::Features>,
        nonterminal: G::Nonterminal,
    ) {
        self.waiting
            .entry((position, nonterminal))
            .or_default()
            .push(item.clone());
        for &predicted_rule in self.grammar.rules_for(nonterminal) {
            enqueue(
                &mut self.chart,
                &mut self.agenda,
                position,
                ItemKey {
                    rule: predicted_rule,
                    dot: 0,
                    origin: position,
                    prefix_features: Rc::default(),
                },
                None,
            );
        }

        if let Some(existing) = self.completed.get(&(position, nonterminal)).cloned() {
            for child in existing {
                let end = self.forest.node(child).key.end;
                self.advance_with_child(end, position, item, child);
            }
        }
    }

    fn scan_and_advance(
        &mut self,
        position: usize,
        item: &ItemKey<G::Features>,
        slot: G::LexicalSlot,
    ) -> Result<(), GrammarError> {
        for edge in self.lexical_edges(slot, position)? {
            self.advance_with_child(edge.end, position, item, edge.node);
        }
        Ok(())
    }

    fn advance_with_child(
        &mut self,
        position: usize,
        item_position: usize,
        item: &ItemKey<G::Features>,
        child: NodeId,
    ) {
        let Some(child_features) = self.forest.node(child).key.constituent_features().cloned()
        else {
            return;
        };
        if !self
            .grammar
            .accepts_prefix(item.rule, item.dot + 1, &child_features)
        {
            return;
        }

        let previous = self.chart[item_position]
            .seen
            .get(&item.clone())
            .copied()
            .flatten();
        if item.dot > 0 && previous.is_none() {
            return;
        }

        let mut prefix_features = Vec::with_capacity(item.prefix_features.len() + 1);
        prefix_features.extend(item.prefix_features.iter().cloned());
        prefix_features.push(child_features);
        let intermediate_cost = self.grammar.intermediate_cost(
            item.rule,
            &prefix_features,
            item.origin,
            item_position,
            position,
        );
        let mut advanced = item.clone();
        advanced.dot += 1;
        advanced.prefix_features = prefix_features.into();
        let mut children = Vec::with_capacity(2);
        if let Some(previous) = previous {
            children.push(previous);
        }
        children.push(child);
        let intermediate = self.forest.intern_node(
            NodeKey::intermediate(
                advanced.rule,
                advanced.dot,
                advanced.origin,
                position,
                Rc::clone(&advanced.prefix_features),
            ),
            PackedAlternative {
                rule: None,
                children,
                local_cost: intermediate_cost,
            },
        );
        enqueue(
            &mut self.chart,
            &mut self.agenda,
            position,
            advanced,
            Some(intermediate.node),
        );
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
                NodeKey::lexical(
                    slot,
                    position,
                    lexical_match.end,
                    lexical_match.features,
                    lexical_match.meaning,
                ),
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

fn enqueue<F>(
    chart: &mut [ChartColumn<F>],
    agenda: &mut VecDeque<(usize, ItemKey<F>)>,
    position: usize,
    item: ItemKey<F>,
    intermediate: Option<NodeId>,
) where
    F: Clone + Eq + Hash,
{
    if let Entry::Vacant(entry) = chart[position].seen.entry(item.clone()) {
        entry.insert(intermediate);
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
    use super::Reduction;
    use super::Rule;
    use super::RuleId;
    use super::parse_chart;
    use crate::forest::ForestSymbol;
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
        EachOther,
    }

    struct TestGrammar {
        rules: Vec<Rule<N, L>>,
        rules_by_lhs: HashMap<N, Vec<RuleId>>,
        scans: RefCell<HashMap<(L, usize), usize>>,
        reduced_child_counts: RefCell<Vec<usize>>,
        reject_list_extension_prefix: bool,
        state_limit: Option<usize>,
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
                reduced_child_counts: RefCell::new(Vec::new()),
                reject_list_extension_prefix: false,
                state_limit: None,
            }
        }

        fn rejecting_list_extension_prefix(mut self) -> Self {
            self.reject_list_extension_prefix = true;
            self
        }

        fn with_state_limit(mut self, limit: usize) -> Self {
            self.state_limit = Some(limit);
            self
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
        ) -> Option<Reduction<Self::Features>> {
            self.reduced_child_counts.borrow_mut().push(children.len());
            (rule.index() <= 6).then_some(Reduction {
                features: (),
                local_cost: ParseCost::default(),
            })
        }

        fn accepts_prefix(
            &self,
            rule: RuleId,
            completed_children: usize,
            _latest_child: &Self::Features,
        ) -> bool {
            !(self.reject_list_extension_prefix && rule.index() == 3 && completed_children == 1)
        }

        fn state_limit(&self) -> Option<usize> {
            self.state_limit
        }
    }

    struct SameSpanAmbiguityGrammar {
        rules: Vec<Rule<N, L>>,
        rules_by_lhs: HashMap<N, Vec<RuleId>>,
    }

    struct SingleRuleGrammar {
        rules: Vec<Rule<N, L>>,
        start_rules: Vec<RuleId>,
    }

    impl SingleRuleGrammar {
        fn new() -> Self {
            Self {
                rules: vec![rule(N::Start, vec![Expected::Lexical(L::A)])],
                start_rules: vec![RuleId::new(0)],
            }
        }
    }

    impl Grammar for SingleRuleGrammar {
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
            if lhs == N::Start { &self.start_rules } else { &[] }
        }

        fn scan(
            &self,
            slot: Self::LexicalSlot,
            tokens: &[Self::Token],
            start: usize,
        ) -> Vec<LexicalMatch<Self::Features, Self::Meaning>> {
            if slot == L::A && tokens.get(start) == Some(&"a") {
                vec![LexicalMatch {
                    end: start + 1,
                    features: (),
                    meaning: Meaning::Atom("a"),
                    local_cost: ParseCost::default(),
                }]
            } else {
                Vec::new()
            }
        }

        fn reduce(
            &self,
            rule: RuleId,
            children: &[Child<'_, Self>],
        ) -> Option<Reduction<Self::Features>> {
            (rule == RuleId::new(0) && children.len() == 1).then_some(Reduction {
                features: (),
                local_cost: ParseCost::default(),
            })
        }
    }

    impl SameSpanAmbiguityGrammar {
        fn new() -> Self {
            let rules = vec![
                rule(N::Start, vec![Expected::Nonterminal(N::Atom); 6]),
                rule(N::Atom, vec![Expected::Lexical(L::A)]),
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
            }
        }
    }

    impl Grammar for SameSpanAmbiguityGrammar {
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
            if slot != L::A || tokens.get(start) != Some(&"a") {
                return Vec::new();
            }
            ["first", "second"]
                .into_iter()
                .map(|meaning| LexicalMatch {
                    end: start + 1,
                    features: (),
                    meaning: Meaning::Atom(meaning),
                    local_cost: ParseCost::default(),
                })
                .collect()
        }

        fn reduce(
            &self,
            rule: RuleId,
            _children: &[Child<'_, Self>],
        ) -> Option<Reduction<Self::Features>> {
            (rule.index() <= 1).then_some(Reduction {
                features: (),
                local_cost: ParseCost::default(),
            })
        }

        fn state_limit(&self) -> Option<usize> {
            Some(40)
        }
    }

    #[test]
    fn reducers_receive_finite_child_features() {
        let grammar = TestGrammar::new();

        parse_chart(&grammar, &["a"]).expect("the artificial grammar parses one atom");

        assert!(grammar.reduced_child_counts.borrow().contains(&1));
    }

    #[test]
    fn same_span_semantic_alternatives_do_not_multiply_parent_chart_items() {
        let grammar = SameSpanAmbiguityGrammar::new();

        let result = parse_chart(&grammar, &["a"; 6])
            .expect("same-span alternatives should remain packed below one recognizer state");

        assert_eq!(result.roots.len(), 1);
        assert!(result.forest.nodes().any(|node| {
            matches!(node.key.symbol, ForestSymbol::Intermediate { .. })
                && node.alternatives.len() == 2
        }));
    }

    #[test]
    fn packed_chart_statistics_are_derived_from_final_structures() {
        let result = parse_chart(&SingleRuleGrammar::new(), &["a"])
            .expect("the single-rule grammar parses one token");

        assert_eq!(result.stats.unique_items(), 2);
        assert_eq!(result.stats.max_column_width(), 1);
        assert_eq!(result.forest.stats().constituent_nodes(), 2);
        assert_eq!(result.forest.stats().intermediate_nodes(), 1);
        assert_eq!(result.forest.stats().packed_alternatives(), 3);
        assert_eq!(result.forest.stats().max_alternatives(), 1);

        let ambiguous = parse_chart(&SameSpanAmbiguityGrammar::new(), &["a"; 6])
            .expect("same-span ambiguity remains packed");
        assert!(ambiguous.stats.unique_items() <= 20);
        assert!(ambiguous.stats.max_column_width() <= 3);
        assert!(ambiguous.forest.stats().max_alternatives() <= 2);
    }

    #[test]
    fn left_recursive_list_terminates_and_spans_the_input() {
        let grammar = TestGrammar::new();
        let tokens = ["a", "and", "a", "and", "a"];

        let result = parse_chart(&grammar, &tokens).expect("grammar and scans are valid");

        assert_eq!(result.roots.len(), 1);
        let root = result.forest.node(result.roots[0]);
        assert_eq!((root.key.start, root.key.end), (0, tokens.len()));
        assert_eq!(root.key.constituent_features(), Some(&()));
    }

    #[test]
    fn grammar_can_prune_an_impossible_partial_rule() {
        let grammar = TestGrammar::new().rejecting_list_extension_prefix();

        let result = parse_chart(&grammar, &["a", "a"]).expect("the grammar is valid");

        assert!(result.roots.is_empty());
    }

    #[test]
    fn grammar_can_bound_chart_state_expansion() {
        let grammar = TestGrammar::new().with_state_limit(1);

        let error = parse_chart(&grammar, &["a", "and", "a"]).unwrap_err();

        assert_eq!(error, super::GrammarError::StateLimitExceeded { limit: 1 });
    }

    #[test]
    fn multiword_and_component_analyses_coexist_and_scans_are_memoized() {
        let grammar = TestGrammar::new();

        let result = parse_chart(&grammar, &["each", "other"])
            .expect("the ambiguous input has complete derivations");

        assert!(result.forest.nodes().any(|node| {
            node.key.symbol == ForestSymbol::Lexical(L::Each)
                && (node.key.start, node.key.end) == (0, 2)
                && node.key.lexical_value() == Some(&Meaning::EachOther)
        }));
        assert!(result.forest.nodes().any(|node| {
            node.key.symbol == ForestSymbol::Lexical(L::Each)
                && (node.key.start, node.key.end) == (0, 1)
                && node.key.lexical_value() == Some(&Meaning::Atom("each"))
        }));
        assert!(result.forest.nodes().any(|node| {
            node.key.symbol == ForestSymbol::Lexical(L::Other)
                && (node.key.start, node.key.end) == (1, 2)
        }));
        let packed_list = result
            .forest
            .nodes()
            .find(|node| {
                node.key.symbol == ForestSymbol::Nonterminal(N::List)
                    && (node.key.start, node.key.end) == (0, 2)
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
