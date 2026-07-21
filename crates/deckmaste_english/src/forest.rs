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
    pub(crate) const fn new(index: usize) -> Self {
        Self(index)
    }

    pub(crate) const fn index(self) -> usize {
        self.0
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub(crate) enum ForestSymbol<N, L> {
    Nonterminal(N),
    Lexical(L),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub(crate) struct NodeKey<N, L, F, M> {
    pub(crate) symbol: ForestSymbol<N, L>,
    pub(crate) start: usize,
    pub(crate) end: usize,
    pub(crate) features: F,
    pub(crate) meaning: M,
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

    pub(crate) fn nodes(&self) -> impl Iterator<Item = &ForestNode<N, L, F, M>> {
        self.nodes.iter()
    }

    pub(crate) fn best(&self, root: NodeId) -> Result<BestParse, ForestError> {
        let mut costs = vec![None; self.nodes.len()];
        let mut alternatives = vec![None; self.nodes.len()];
        let mut tied_alternatives = vec![Vec::new(); self.nodes.len()];
        let mut visiting = vec![false; self.nodes.len()];
        let cost = self.best_cost(
            root,
            &mut costs,
            &mut alternatives,
            &mut tied_alternatives,
            &mut visiting,
        )?;
        Ok(BestParse {
            cost,
            alternatives,
            tied_alternatives,
        })
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
    use super::ForestSymbol;
    use super::NodeKey;
    use super::PackedAlternative;
    use super::ParseCost;
    use super::ParseForest;
    use crate::chart::RuleId;

    #[test]
    fn equal_nodes_pack_alternatives_and_choose_the_lower_cost() {
        let mut forest = ParseForest::<&str, (), (), &str>::new();
        let key = NodeKey {
            symbol: ForestSymbol::Nonterminal("expression"),
            start: 0,
            end: 1,
            features: (),
            meaning: "same meaning",
        };

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

        let best = forest.best(first.node).expect("the node is acyclic");
        assert_eq!(best.cost.precedence, 1);
        assert_eq!(best.alternative(first.node), Some(1));
    }

    #[test]
    fn equal_cost_uses_stable_rule_order_and_preserves_all_ties() {
        let mut forest = ParseForest::<&str, (), (), &str>::new();
        let key = NodeKey {
            symbol: ForestSymbol::Nonterminal("expression"),
            start: 0,
            end: 1,
            features: (),
            meaning: "same meaning",
        };
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

        let best = forest.best(root).expect("the node is acyclic");
        assert_eq!(best.alternative(root), Some(1));
        assert_eq!(best.tied_alternatives(root), &[0, 1]);
    }
}
