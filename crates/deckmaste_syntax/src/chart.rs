//! Earley chart parsing into a Packed Forest.

use std::collections::HashMap;
use std::collections::HashSet;
use std::ops::Range;

use deckmaste_lexical::LexicalReading;
use deckmaste_lexical::Lexicon;

use crate::compile::CompiledGrammar;
use crate::step::ChildView;
use crate::step::LeafFacts;
use crate::step::State;
use crate::step::Summary;
use crate::step::Symbol;

/// Sizes of one parse, for measurement only.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct Metrics {
    /// Distinct chart items (Production, dot, span, state), predictions included.
    pub chart_items: usize,
    /// Completed Constituents in the Packed Forest.
    pub completed_nodes: usize,
    /// Alternative Derivations recorded across completed Constituents.
    pub families: usize,
    /// Lexical alternatives offered to the chart.
    pub leaves: usize,
}

#[derive(Debug, Clone)]
pub(crate) struct Leaf {
    pub end: usize,
    pub reading: LexicalReading,
    pub facts: LeafFacts,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub(crate) enum ChildRef {
    Node(usize),
    Leaf(usize),
    Punct,
}

/// A partial Production: its found children are a chain of families.
#[derive(Debug)]
pub(crate) struct Item {
    pub production: usize,
    pub dot: usize,
    pub start: usize,
    pub state: State,
    /// (previous item, last child) alternatives.
    pub families: Vec<(Option<usize>, ChildRef)>,
}

/// A completed Constituent with its alternative Derivations.
#[derive(Debug)]
pub(crate) struct Node {
    pub category: usize,
    pub start: usize,
    pub end: usize,
    pub summary: Summary,
    /// (Production, completed item) alternatives.
    pub families: Vec<(usize, usize)>,
}

/// Every Derivation of an input, packed by (Category, span, summary).
#[derive(Debug, Default)]
pub struct PackedForest {
    pub(crate) items: Vec<Item>,
    pub(crate) nodes: Vec<Node>,
    pub(crate) leaves: Vec<Leaf>,
    pub(crate) roots: Vec<usize>,
}

impl PackedForest {
    #[must_use]
    pub fn metrics(&self) -> Metrics {
        Metrics {
            chart_items: self.items.len(),
            completed_nodes: self.nodes.len(),
            families: self.nodes.iter().map(|n| n.families.len()).sum(),
            leaves: self.leaves.len(),
        }
    }
}

/// The result of parsing one input.
#[derive(Debug)]
pub struct Parse<'g> {
    pub(crate) grammar: &'g CompiledGrammar,
    pub forest: PackedForest,
    /// Words no declared lexeme spells, as char ranges.
    pub vocabulary_gaps: Vec<Range<usize>>,
}

impl CompiledGrammar {
    /// Parses `text` as the grammar's root Category.
    #[must_use]
    #[expect(clippy::too_many_lines, reason = "one Earley loop reads best whole")]
    pub fn parse<'g>(&'g self, lexicon: &Lexicon, text: &str) -> Parse<'g> {
        let analyzed = lexicon.analyze(text);
        let tokens = &analyzed.tokens;
        let n = tokens.len();
        let skip = |mut p: usize| {
            while p < n && tokens[p].is_whitespace() {
                p += 1;
            }
            p
        };
        let mut forest = PackedForest::default();
        let mut by_start: Vec<Vec<usize>> = vec![Vec::new(); n + 1];
        for found in &analyzed.matches {
            if let Some(facts) = self.leaf_facts(&found.reading) {
                by_start[found.start].push(forest.leaves.len());
                forest.leaves.push(Leaf {
                    end: found.end,
                    reading: found.reading.clone(),
                    facts,
                });
            }
        }
        let mut sets: Vec<Vec<usize>> = vec![Vec::new(); n + 1];
        let mut item_index: HashMap<(usize, usize, usize, usize, State), usize> = HashMap::new();
        let mut node_index: HashMap<(usize, usize, usize, Summary), usize> = HashMap::new();
        let mut waiting: HashMap<(usize, usize), Vec<usize>> = HashMap::new();
        let mut predicted: HashSet<(usize, usize)> = HashSet::new();

        let mut add = |forest: &mut PackedForest,
                       sets: &mut Vec<Vec<usize>>,
                       production: usize,
                       dot: usize,
                       start: usize,
                       end: usize,
                       state: State,
                       family: Option<(Option<usize>, ChildRef)>| {
            let key = (production, dot, start, end, state);
            let index = if let Some(&i) = item_index.get(&key) {
                i
            } else {
                let i = forest.items.len();
                forest.items.push(Item {
                    production,
                    dot,
                    start,
                    state: key.4.clone(),
                    families: Vec::new(),
                });
                item_index.insert(key, i);
                sets[end].push(i);
                i
            };
            if let Some(family) = family {
                forest.items[index].families.push(family);
            }
        };

        for &p in &self.by_lhs[self.root] {
            predicted.insert((0, p));
            add(
                &mut forest,
                &mut sets,
                p,
                0,
                0,
                0,
                self.productions[p].start(),
                None,
            );
        }
        for position in 0..=n {
            let mut cursor = 0;
            while cursor < sets[position].len() {
                let id = sets[position][cursor];
                cursor += 1;
                let (production_id, dot, start) = {
                    let item = &forest.items[id];
                    (item.production, item.dot, item.start)
                };
                let production = &self.productions[production_id];
                let previous = (dot > 0).then_some(id);
                if dot == production.rhs.len() {
                    let Some(summary) = production.finish(self, &forest.items[id].state) else {
                        continue;
                    };
                    let key = (production.lhs, start, position, summary);
                    if let Some(&node) = node_index.get(&key) {
                        forest.nodes[node].families.push((production_id, id));
                        continue;
                    }
                    let node = forest.nodes.len();
                    forest.nodes.push(Node {
                        category: production.lhs,
                        start,
                        end: position,
                        summary: key.3.clone(),
                        families: vec![(production_id, id)],
                    });
                    node_index.insert(key, node);
                    let parents = waiting
                        .get(&(start, production.lhs))
                        .cloned()
                        .unwrap_or_default();
                    for parent in parents {
                        let item = &forest.items[parent];
                        let parent_production = &self.productions[item.production];
                        let view = ChildView::Node(&forest.nodes[node].summary);
                        if let Some(state) =
                            parent_production.accept(self, &item.state, item.dot, &view)
                        {
                            let (p, d, s) = (item.production, item.dot, item.start);
                            let prev = (d > 0).then_some(parent);
                            add(
                                &mut forest,
                                &mut sets,
                                p,
                                d + 1,
                                s,
                                position,
                                state,
                                Some((prev, ChildRef::Node(node))),
                            );
                        }
                    }
                    continue;
                }
                match &production.rhs[dot] {
                    Symbol::Cat { category, .. } => {
                        waiting.entry((position, *category)).or_default().push(id);
                        for &p in &self.by_lhs[*category] {
                            if predicted.insert((position, p)) {
                                add(
                                    &mut forest,
                                    &mut sets,
                                    p,
                                    0,
                                    position,
                                    position,
                                    self.productions[p].start(),
                                    None,
                                );
                            }
                        }
                    }
                    Symbol::Leaf(leaf_test) => {
                        let at = skip(position);
                        for &leaf in &by_start[at] {
                            let facts = &forest.leaves[leaf].facts;
                            if !self.leaf_matches(leaf_test, facts) {
                                continue;
                            }
                            let view = ChildView::Leaf(facts);
                            if let Some(state) =
                                production.accept(self, &forest.items[id].state, dot, &view)
                            {
                                let end = forest.leaves[leaf].end;
                                add(
                                    &mut forest,
                                    &mut sets,
                                    production_id,
                                    dot + 1,
                                    start,
                                    end,
                                    state,
                                    Some((previous, ChildRef::Leaf(leaf))),
                                );
                            }
                        }
                    }
                    Symbol::Punct(c) => {
                        let at = skip(position);
                        if at < n
                            && tokens[at] == *c
                            && let Some(state) = production.accept(
                                self,
                                &forest.items[id].state,
                                dot,
                                &ChildView::Punct,
                            )
                        {
                            add(
                                &mut forest,
                                &mut sets,
                                production_id,
                                dot + 1,
                                start,
                                at + 1,
                                state,
                                Some((previous, ChildRef::Punct)),
                            );
                        }
                    }
                }
            }
        }
        forest.roots = (0..forest.nodes.len())
            .filter(|&i| {
                let node = &forest.nodes[i];
                node.category == self.root
                    && node.start == 0
                    && skip(node.end) == n
                    && node.summary.gap == crate::Gap::None
            })
            .collect();
        Parse {
            grammar: self,
            forest,
            vocabulary_gaps: analyzed.unknown_words(),
        }
    }
}
