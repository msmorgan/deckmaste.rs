use std::collections::HashMap;
use std::hash::Hash;
use std::ops::Add;
use std::ops::AddAssign;

use crate::chart::RuleId;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash, Ord, PartialOrd)]
pub(crate) struct ParseCost {
    pub(crate) unknown_words: u32,
    pub(crate) recoveries: u32,
    pub(crate) generic_rules: u32,
    pub(crate) precedence: u32,
}

impl Add for ParseCost {
    type Output = Self;

    fn add(mut self, rhs: Self) -> Self::Output {
        self += rhs;
        self
    }
}

impl AddAssign for ParseCost {
    fn add_assign(&mut self, rhs: Self) {
        self.unknown_words = self.unknown_words.saturating_add(rhs.unknown_words);
        self.recoveries = self.recoveries.saturating_add(rhs.recoveries);
        self.generic_rules = self.generic_rules.saturating_add(rhs.generic_rules);
        self.precedence = self.precedence.saturating_add(rhs.precedence);
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd)]
pub(crate) struct NodeId(usize);

impl NodeId {
    pub(crate) const fn index(self) -> usize {
        self.0
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub(crate) enum ForestSymbol<N, L> {
    Nonterminal(N),
    Lexical(L),
    Intermediate { rule: RuleId, dot: usize },
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub(crate) enum ForestFeatures<F> {
    Constituent(F),
    Prefix(Vec<F>),
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct ForestStats {
    constituent_nodes: usize,
    intermediate_nodes: usize,
    packed_alternatives: usize,
    max_alternatives: usize,
}

impl ForestStats {
    #[must_use]
    pub const fn constituent_nodes(self) -> usize {
        self.constituent_nodes
    }

    #[must_use]
    pub const fn intermediate_nodes(self) -> usize {
        self.intermediate_nodes
    }

    #[must_use]
    pub const fn packed_alternatives(self) -> usize {
        self.packed_alternatives
    }

    #[must_use]
    pub const fn max_alternatives(self) -> usize {
        self.max_alternatives
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub(crate) struct NodeKey<N, L, F, M> {
    pub(crate) symbol: ForestSymbol<N, L>,
    pub(crate) start: usize,
    pub(crate) end: usize,
    features: ForestFeatures<F>,
    lexical_value: Option<M>,
}

impl<N, L, F, M> NodeKey<N, L, F, M> {
    pub(crate) fn nonterminal(symbol: N, start: usize, end: usize, features: F) -> Self {
        Self {
            symbol: ForestSymbol::Nonterminal(symbol),
            start,
            end,
            features: ForestFeatures::Constituent(features),
            lexical_value: None,
        }
    }

    pub(crate) fn lexical(slot: L, start: usize, end: usize, features: F, value: M) -> Self {
        Self {
            symbol: ForestSymbol::Lexical(slot),
            start,
            end,
            features: ForestFeatures::Constituent(features),
            lexical_value: Some(value),
        }
    }

    pub(crate) fn intermediate(
        rule: RuleId,
        dot: usize,
        start: usize,
        end: usize,
        prefix_features: Vec<F>,
    ) -> Self {
        Self {
            symbol: ForestSymbol::Intermediate { rule, dot },
            start,
            end,
            features: ForestFeatures::Prefix(prefix_features),
            lexical_value: None,
        }
    }

    pub(crate) fn constituent_features(&self) -> Option<&F> {
        match &self.features {
            ForestFeatures::Constituent(features) => Some(features),
            ForestFeatures::Prefix(_) => None,
        }
    }

    pub(crate) fn lexical_value(&self) -> Option<&M> {
        self.lexical_value.as_ref()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct PackedAlternative {
    pub(crate) rule: Option<RuleId>,
    pub(crate) children: Vec<NodeId>,
    pub(crate) local_cost: ParseCost,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct ForestNode<N, L, F, M> {
    pub(crate) key: NodeKey<N, L, F, M>,
    pub(crate) alternatives: Vec<PackedAlternative>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct InternResult {
    pub(crate) node: NodeId,
    pub(crate) node_was_new: bool,
    pub(crate) alternative_was_new: bool,
}

#[derive(Debug, Clone)]
pub(crate) struct ParseForest<N, L, F, M> {
    nodes: Vec<ForestNode<N, L, F, M>>,
    node_ids: HashMap<NodeKey<N, L, F, M>, NodeId>,
}

impl<N, L, F, M> ParseForest<N, L, F, M>
where
    N: Clone + Eq + Hash,
    L: Clone + Eq + Hash,
    F: Clone + Eq + Hash,
    M: Clone + Eq + Hash,
{
    pub(crate) fn new() -> Self {
        Self {
            nodes: Vec::new(),
            node_ids: HashMap::new(),
        }
    }

    pub(crate) fn intern_node(
        &mut self,
        key: NodeKey<N, L, F, M>,
        alternative: PackedAlternative,
    ) -> InternResult {
        if let Some(&node) = self.node_ids.get(&key) {
            let alternatives = &mut self.nodes[node.index()].alternatives;
            let alternative_was_new = !alternatives.contains(&alternative);
            if alternative_was_new {
                alternatives.push(alternative);
            }
            return InternResult {
                node,
                node_was_new: false,
                alternative_was_new,
            };
        }

        let node = NodeId(self.nodes.len());
        self.node_ids.insert(key.clone(), node);
        self.nodes.push(ForestNode {
            key,
            alternatives: vec![alternative],
        });
        InternResult {
            node,
            node_was_new: true,
            alternative_was_new: true,
        }
    }

    pub(crate) fn node(&self, node: NodeId) -> &ForestNode<N, L, F, M> {
        &self.nodes[node.index()]
    }

    #[cfg(test)]
    pub(crate) fn nodes(&self) -> impl Iterator<Item = &ForestNode<N, L, F, M>> {
        self.nodes.iter()
    }

    pub(crate) fn stats(&self) -> ForestStats {
        let mut stats = ForestStats::default();
        for node in &self.nodes {
            match node.key.symbol {
                ForestSymbol::Nonterminal(_) | ForestSymbol::Lexical(_) => {
                    stats.constituent_nodes += 1;
                }
                ForestSymbol::Intermediate { .. } => stats.intermediate_nodes += 1,
            }
            stats.packed_alternatives += node.alternatives.len();
            stats.max_alternatives = stats.max_alternatives.max(node.alternatives.len());
        }
        stats
    }

    pub(crate) fn best_root(
        &self,
        roots: impl IntoIterator<Item = NodeId>,
    ) -> Result<Option<(NodeId, BestParse)>, ForestError> {
        let mut costs = vec![None; self.nodes.len()];
        let mut alternatives = vec![None; self.nodes.len()];
        let mut tied_alternatives = vec![Vec::new(); self.nodes.len()];
        let mut visiting = vec![false; self.nodes.len()];
        let mut selected = None;
        for root in roots {
            let cost = self.best_cost(
                root,
                &mut costs,
                &mut alternatives,
                &mut tied_alternatives,
                &mut visiting,
            )?;
            let candidate = (cost, root.index(), root);
            if selected.is_none_or(|current| candidate < current) {
                selected = Some(candidate);
            }
        }
        Ok(selected.map(|(cost, _, root)| {
            (
                root,
                BestParse {
                    cost,
                    alternatives,
                    tied_alternatives,
                },
            )
        }))
    }

    fn best_cost(
        &self,
        node: NodeId,
        costs: &mut [Option<ParseCost>],
        choices: &mut [Option<usize>],
        ties: &mut [Vec<usize>],
        visiting: &mut [bool],
    ) -> Result<ParseCost, ForestError> {
        if let Some(cost) = costs[node.index()] {
            return Ok(cost);
        }
        if visiting[node.index()] {
            return Err(ForestError::Cycle(node));
        }
        visiting[node.index()] = true;

        let forest_node = self.node(node);
        let mut best: Option<(ParseCost, usize, usize)> = None;
        for (alternative_index, alternative) in forest_node.alternatives.iter().enumerate() {
            let mut cost = alternative.local_cost;
            for &child in &alternative.children {
                cost += self.best_cost(child, costs, choices, ties, visiting)?;
            }
            let rule_order = alternative.rule.map_or(usize::MAX, RuleId::index);
            let candidate = (cost, rule_order, alternative_index);
            match best {
                None => {
                    best = Some(candidate);
                    ties[node.index()].push(alternative_index);
                }
                Some(current) if cost < current.0 => {
                    best = Some(candidate);
                    ties[node.index()].clear();
                    ties[node.index()].push(alternative_index);
                }
                Some(current) if cost == current.0 => {
                    ties[node.index()].push(alternative_index);
                    if candidate < current {
                        best = Some(candidate);
                    }
                }
                Some(_) => {}
            }
        }

        visiting[node.index()] = false;
        let (cost, _, alternative) = best.ok_or(ForestError::MissingAlternative(node))?;
        costs[node.index()] = Some(cost);
        choices[node.index()] = Some(alternative);
        Ok(cost)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct BestParse {
    pub(crate) cost: ParseCost,
    alternatives: Vec<Option<usize>>,
    tied_alternatives: Vec<Vec<usize>>,
}

impl BestParse {
    pub(crate) fn alternative(&self, node: NodeId) -> Option<usize> {
        self.alternatives.get(node.index()).copied().flatten()
    }

    pub(crate) fn tied_alternatives(&self, node: NodeId) -> &[usize] {
        self.tied_alternatives
            .get(node.index())
            .map_or(&[], Vec::as_slice)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum ForestError {
    Cycle(NodeId),
    MissingAlternative(NodeId),
}

#[cfg(test)]
mod tests {
    use super::NodeKey;
    use super::PackedAlternative;
    use super::ParseCost;
    use super::ParseForest;
    use crate::chart::RuleId;

    #[test]
    fn equal_nodes_pack_alternatives_and_choose_the_lower_cost() {
        let mut forest = ParseForest::<&str, (), (), &str>::new();
        let key = NodeKey::nonterminal("expression", 0, 1, ());

        let first = forest.intern_node(
            key.clone(),
            PackedAlternative {
                rule: Some(RuleId::new(4)),
                children: Vec::new(),
                local_cost: ParseCost {
                    precedence: 2,
                    ..ParseCost::default()
                },
            },
        );
        let second = forest.intern_node(
            key,
            PackedAlternative {
                rule: Some(RuleId::new(3)),
                children: Vec::new(),
                local_cost: ParseCost {
                    precedence: 1,
                    ..ParseCost::default()
                },
            },
        );

        assert_eq!(first.node, second.node);
        assert!(first.node_was_new);
        assert!(!second.node_was_new);
        assert_eq!(forest.node(first.node).alternatives.len(), 2);

        let (_, best) = forest
            .best_root([first.node])
            .expect("the node is acyclic")
            .expect("there is a root");
        assert_eq!(best.cost.precedence, 1);
        assert_eq!(best.alternative(first.node), Some(1));
    }

    #[test]
    fn equal_cost_uses_stable_rule_order_and_preserves_all_ties() {
        let mut forest = ParseForest::<&str, (), (), &str>::new();
        let key = NodeKey::nonterminal("expression", 0, 1, ());
        let root = forest
            .intern_node(
                key.clone(),
                PackedAlternative {
                    rule: Some(RuleId::new(4)),
                    children: Vec::new(),
                    local_cost: ParseCost::default(),
                },
            )
            .node;
        forest.intern_node(
            key,
            PackedAlternative {
                rule: Some(RuleId::new(3)),
                children: Vec::new(),
                local_cost: ParseCost::default(),
            },
        );

        let (_, best) = forest
            .best_root([root])
            .expect("the node is acyclic")
            .expect("there is a root");
        assert_eq!(best.alternative(root), Some(1));
        assert_eq!(best.tied_alternatives(root), &[0, 1]);
    }

    #[test]
    fn best_root_uses_cost_then_stable_node_order() {
        let mut forest = ParseForest::<&str, (), &str, &str>::new();
        let first = forest
            .intern_node(
                NodeKey::nonterminal("sentence", 0, 1, "first"),
                PackedAlternative {
                    rule: Some(RuleId::new(1)),
                    children: Vec::new(),
                    local_cost: ParseCost::default(),
                },
            )
            .node;
        let second = forest
            .intern_node(
                NodeKey::nonterminal("sentence", 0, 1, "second"),
                PackedAlternative {
                    rule: Some(RuleId::new(2)),
                    children: Vec::new(),
                    local_cost: ParseCost::default(),
                },
            )
            .node;

        let (selected, _) = forest
            .best_root([second, first])
            .expect("the nodes are acyclic")
            .expect("there are roots");

        assert_eq!(selected, first);
    }
}
