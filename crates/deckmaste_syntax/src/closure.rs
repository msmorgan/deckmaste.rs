//! Closures decided on the whole chart.
//!
//! Some Derivations are ruled out by a longer analysis of the words that
//! follow them, which a left-to-right step cannot see when it completes them:
//!
//! - an `unextended` first child does not end where one Constituent of its
//!   Category spans it and the child after it;
//! - a fused-head Constituent is not formed where its twin over the same words
//!   is followed by its head;
//! - a converted head does not begin with a lone converted noun where its
//!   twin premodifies a head after the same earlier children.
//!
//! Once the chart is complete, the closed Derivations are removed and every
//! Constituent left without a Derivation goes with them.

use std::collections::HashMap;
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
        let dead_heads = self.premodifying_twins(forest);
        if dead_items.is_empty() && fused.is_empty() && dead_heads.is_empty() {
            return;
        }
        let closed_family = |node: usize, production: usize| {
            let n = &forest.nodes[node];
            fused.contains(&(production, n.start, n.end))
        };
        let mut node_live = vec![false; forest.nodes.len()];
        let mut item_live = vec![false; forest.items.len()];
        let child_live = |node_live: &[bool], item: usize, child: ChildRef| match child {
            ChildRef::Node(node) => node_live[node] && !dead_heads.contains(&(item, node)),
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
                        previous.is_none_or(|p| item_live[p])
                            && child_live(&node_live, index, child)
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
        for (index, item) in forest.items.iter_mut().enumerate() {
            item.families.retain(|&(previous, child)| {
                previous.is_none_or(|p| item_live[p]) && child_live(&node_live, index, child)
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

    /// (completed item, `head` Constituent) of a `converted_head` Production
    /// where that `head` begins with a lone converted noun as its own head
    /// and the same Production, over the same earlier children, has a longer
    /// `head` that the noun's twin begins as a premodifier: the noun does
    /// not end a Constituent before the Nominal its twin premodifies.
    fn premodifying_twins(&self, forest: &PackedForest) -> HashSet<(usize, usize)> {
        let Some(conversion) = self.conversion else {
            return HashSet::new();
        };
        let completed: Vec<usize> = forest
            .nodes
            .iter()
            .flat_map(|n| n.families.iter().map(|&(_, item)| item))
            .filter(|&item| {
                let item = &forest.items[item];
                self.converted_heads.contains(&item.production)
                    && item.dot == self.productions[item.production].rhs.len()
            })
            .collect();
        let mut initial = Initial {
            grammar: self,
            forest,
            conversion,
            memo: HashMap::new(),
        };
        // (Production, start, first word of the `head`) wherever that word
        // premodifies the rest of the `head`.
        let mut twins = HashSet::new();
        for &index in &completed {
            let item = &forest.items[index];
            for &(_, child) in &item.families {
                let ChildRef::Node(head) = child else { continue };
                if initial.of(head).premodifier {
                    twins.insert((item.production, item.start, forest.nodes[head].start));
                }
            }
        }
        let mut dead = HashSet::new();
        for &index in &completed {
            let item = &forest.items[index];
            for &(_, child) in &item.families {
                let ChildRef::Node(head) = child else { continue };
                let found = initial.of(head);
                if found.lone_head
                    && !found.premodifier
                    && twins.contains(&(item.production, item.start, forest.nodes[head].start))
                {
                    dead.insert((index, head));
                }
            }
        }
        dead
    }
}

/// What a Constituent's first word is, down its head line.
#[derive(Clone, Copy, Default)]
struct Found {
    /// A one-word converted Constituent heading it.
    lone_head: bool,
    /// A one-word converted dependent of a following `head`.
    premodifier: bool,
}

struct Initial<'a> {
    grammar: &'a CompiledGrammar,
    forest: &'a PackedForest,
    conversion: (usize, u16),
    memo: HashMap<usize, Found>,
}

impl Initial<'_> {
    fn converted(&self, node: usize) -> bool {
        let n = &self.forest.nodes[node];
        n.summary.leaves == 1
            && self.grammar.category_features[n.category]
                .iter()
                .position(|&f| f == self.conversion.0)
                .is_some_and(|at| n.summary.values[at].get() == Some(self.conversion.1))
    }

    fn of(&mut self, node: usize) -> Found {
        if let Some(&known) = self.memo.get(&node) {
            return known;
        }
        // A cycle through unary Productions finds nothing new.
        self.memo.insert(node, Found::default());
        let forest = self.forest;
        let start = forest.nodes[node].start;
        let mut found = Found {
            lone_head: self.converted(node),
            premodifier: false,
        };
        for &(production, item) in &forest.nodes[node].families {
            let names = &self.grammar.productions[production].names;
            let head_first = names[0].as_str() == "head";
            let premodifies = !head_first && names[1..].iter().any(|n| n.as_str() == "head");
            for child in first_children(forest, item) {
                let ChildRef::Node(child) = child else { continue };
                if forest.nodes[child].start != start {
                    continue;
                }
                if head_first {
                    let below = self.of(child);
                    found.lone_head |= below.lone_head;
                    found.premodifier |= below.premodifier;
                } else if premodifies && self.converted(child) {
                    found.premodifier = true;
                }
            }
        }
        self.memo.insert(node, found);
        found
    }
}

/// The first child of each Derivation of completed `item`.
fn first_children(forest: &PackedForest, item: usize) -> Vec<ChildRef> {
    let mut found = Vec::new();
    let mut stack = vec![item];
    let mut seen = HashSet::new();
    while let Some(index) = stack.pop() {
        if !seen.insert(index) {
            continue;
        }
        for &(previous, child) in &forest.items[index].families {
            match previous {
                Some(previous) => stack.push(previous),
                None => found.push(child),
            }
        }
    }
    found
}
