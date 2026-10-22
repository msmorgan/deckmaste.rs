//! Closures decided on the whole chart.
//!
//! Some Derivations are ruled out by a longer analysis of the words that
//! follow them, which a left-to-right step cannot see when it completes them:
//!
//! - an `unextended` first child does not end where one Constituent of its
//!   Category spans it and the child after it;
//! - a fused-head Constituent is not formed where its twin over the same words
//!   is followed by its head.
//!
//! Once the chart is complete, the closed Derivations are removed and every
//! Constituent left without a Derivation goes with them.

use std::collections::HashSet;

use crate::chart::ChildRef;
use crate::chart::PackedForest;
use crate::compile::CompiledGrammar;
use crate::step::Symbol;

impl CompiledGrammar {
    /// Removes the Derivations a chart closure rules out from `forest`.
    pub(crate) fn close(&self, forest: &mut PackedForest) {
        if self.chart_closed.is_empty() {
            return;
        }
        let dead_items = self.unextended_items(forest);
        let fused = self.headed_twins(forest);
        if dead_items.is_empty() && fused.is_empty() {
            return;
        }
        let closed_family = |node: usize, production: usize| {
            let n = &forest.nodes[node];
            fused.contains(&(production, n.start, n.end))
        };
        let mut node_live = vec![false; forest.nodes.len()];
        let mut item_live = vec![false; forest.items.len()];
        let child_live = |node_live: &[bool], child: ChildRef| match child {
            ChildRef::Node(node) => node_live[node],
            ChildRef::Leaf(_) | ChildRef::Punct => true,
        };
        // Least fixpoint: a Constituent lives when it has a finite Derivation
        // that no closure rules out.
        loop {
            let mut changed = false;
            for (index, item) in forest.items.iter().enumerate() {
                if item_live[index] || dead_items.contains(&index) {
                    continue;
                }
                if item.dot == 0
                    || item.families.iter().any(|&(previous, child)| {
                        previous.is_none_or(|p| item_live[p]) && child_live(&node_live, child)
                    })
                {
                    item_live[index] = true;
                    changed = true;
                }
            }
            for (index, node) in forest.nodes.iter().enumerate() {
                if node_live[index] {
                    continue;
                }
                if node
                    .families
                    .iter()
                    .any(|&(production, item)| item_live[item] && !closed_family(index, production))
                {
                    node_live[index] = true;
                    changed = true;
                }
            }
            if !changed {
                break;
            }
        }
        let fused_closed: Vec<Vec<usize>> = forest
            .nodes
            .iter()
            .enumerate()
            .map(|(index, node)| {
                node.families
                    .iter()
                    .map(|&(production, _)| production)
                    .filter(|&production| closed_family(index, production))
                    .collect()
            })
            .collect();
        for (index, node) in forest.nodes.iter_mut().enumerate() {
            node.families.retain(|&(production, item)| {
                item_live[item] && !fused_closed[index].contains(&production)
            });
        }
        for item in &mut forest.items {
            item.families.retain(|&(previous, child)| {
                previous.is_none_or(|p| item_live[p]) && child_live(&node_live, child)
            });
        }
    }

    /// Items of a Production with an `unextended` first child, past the child
    /// after it, where one Constituent of the first child's Category spans
    /// both.
    fn unextended_items(&self, forest: &PackedForest) -> HashSet<usize> {
        let spans: HashSet<(usize, usize, usize)> = forest
            .nodes
            .iter()
            .map(|n| (n.category, n.start, n.end))
            .collect();
        forest
            .items
            .iter()
            .enumerate()
            .filter(|(_, item)| {
                let production = &self.productions[item.production];
                item.dot == 2
                    && production.unextended_first
                    && matches!(production.rhs[0], Symbol::Cat { category, .. }
                        if spans.contains(&(category, item.start, item.end)))
            })
            .map(|(index, _)| index)
            .collect()
    }

    /// (fused-head Production, start, end) wherever its twin's Constituent is
    /// followed by its head in a Derivation of a completed Constituent.
    fn headed_twins(&self, forest: &PackedForest) -> HashSet<(usize, usize, usize)> {
        let mut fused_by_twin: Vec<Vec<usize>> = vec![Vec::new(); self.productions.len()];
        for (index, production) in self.productions.iter().enumerate() {
            if let Some(twin) = production.fused_twin {
                fused_by_twin[twin].push(index);
            }
        }
        let mut closed = HashSet::new();
        let mut visited = vec![false; forest.items.len()];
        let mut stack: Vec<usize> = forest
            .nodes
            .iter()
            .flat_map(|n| n.families.iter().map(|&(_, item)| item))
            .collect();
        while let Some(index) = stack.pop() {
            if std::mem::replace(&mut visited[index], true) {
                continue;
            }
            let item = &forest.items[index];
            let production = &self.productions[item.production];
            let headed = item.dot >= 2 && production.names[item.dot - 1].as_str() == "head";
            for &(previous, _) in &item.families {
                let Some(previous) = previous else { continue };
                stack.push(previous);
                if !headed {
                    continue;
                }
                for &(_, child) in &forest.items[previous].families {
                    let ChildRef::Node(dependent) = child else { continue };
                    let node = &forest.nodes[dependent];
                    for &(twin, _) in &node.families {
                        for &fused in &fused_by_twin[twin] {
                            closed.insert((fused, node.start, node.end));
                        }
                    }
                }
            }
        }
        closed
    }
}
