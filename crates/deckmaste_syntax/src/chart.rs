//! Earley chart parsing into a Packed Forest.

use std::collections::HashMap;
use std::hash::BuildHasherDefault;
use std::ops::Range;

use deckmaste_lexical::AnalyzedText;
use deckmaste_lexical::LexicalReading;
use deckmaste_lexical::Lexicon;

use crate::compile::CompiledGrammar;
use crate::hash::ChartHasher;
use crate::step::ChildView;
use crate::step::LeafFacts;
use crate::step::State;
use crate::step::Summary;
use crate::step::Symbol;

type ChartMap<K> = HashMap<K, usize, BuildHasherDefault<ChartHasher>>;

/// Sizes of one parse, for measurement only.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct Metrics {
    /// Distinct chart items (Production, dot, span, state), predictions
    /// included.
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
    pub start: usize,
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
    pub end: usize,
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
    /// The Category parsed for.
    pub(crate) root: usize,
    /// The analyzed text, as chars.
    pub(crate) tokens: Vec<char>,
    pub forest: PackedForest,
    /// Words no declared lexeme spells, as char ranges.
    pub vocabulary_gaps: Vec<Range<usize>>,
}

impl<'g> Parse<'g> {
    /// Parses another analyzed text as the Category this Parse was made for.
    /// Unlike [`CompiledGrammar::parse_category`] with [`Parse::category`],
    /// this keeps a Category that Readings report under another Category's
    /// name.
    #[must_use]
    pub fn reparse(&self, analyzed: &AnalyzedText) -> Parse<'g> {
        self.grammar.run(analyzed, self.root)
    }
}

impl CompiledGrammar {
    /// Parses `text` as the grammar's root Category.
    #[must_use]
    pub fn parse<'g>(&'g self, lexicon: &Lexicon, text: &str) -> Parse<'g> {
        self.parse_analyzed(&lexicon.analyze(text))
    }

    /// Parses an already analyzed text, such as one analyzed with a card's
    /// names in scope, as the grammar's root Category.
    #[must_use]
    pub fn parse_analyzed<'g>(&'g self, analyzed: &AnalyzedText) -> Parse<'g> {
        self.run(analyzed, self.root)
    }

    /// Parses an analyzed text as the named Category, or `None` when the
    /// grammar has no such Category.
    #[must_use]
    pub fn parse_category<'g>(
        &'g self,
        analyzed: &AnalyzedText,
        category: &str,
    ) -> Option<Parse<'g>> {
        let root = self.category_id(category)?;
        Some(self.run(analyzed, root))
    }

    pub(crate) fn atomic_spans(&self, analyzed: &AnalyzedText) -> Vec<std::ops::Range<usize>> {
        analyzed
            .matches
            .iter()
            .filter_map(|m| {
                let LexicalReading::Word(word) = &m.reading else { return None };
                let facts = self.lexemes.get(&word.lexeme)?;
                (m.start == 0 || !analyzed.tokens[m.start - 1].is_alphanumeric()).then_some(())?;
                (facts.category == deckmaste_lexical::Category::Noun
                    && analyzed.tokens[m.start..m.end].contains(&' '))
                .then_some(m.start..m.end)
            })
            .collect()
    }

    #[expect(clippy::too_many_lines, reason = "one Earley loop reads best whole")]
    pub(crate) fn run<'g>(&'g self, analyzed: &AnalyzedText, root: usize) -> Parse<'g> {
        let tokens = &analyzed.tokens;
        let n = tokens.len();
        let skip = |mut p: usize| {
            // A structural line break consumes its own boundary. Indentation
            // is not an interword space and has no atom to realize it.
            if p > 0 && matches!(tokens[p - 1], '\n' | '\r') {
                return p;
            }
            while p < n && tokens[p].is_whitespace() && tokens[p] != '\n' && tokens[p] != '\r' {
                p += 1;
            }
            p
        };
        let mut forest = PackedForest::default();
        let mut by_start: Vec<Vec<usize>> = vec![Vec::new(); n + 1];
        let atomic = self.atomic_spans(analyzed);
        for found in &analyzed.matches {
            if atomic.iter().any(|span| {
                (span.start < found.start && found.start < span.end)
                    || (span.start < found.end && found.end < span.end)
            }) {
                continue;
            }
            if let Some(facts) = self.leaf_facts(&found.reading) {
                by_start[found.start].push(forest.leaves.len());
                forest.leaves.push(Leaf {
                    start: found.start,
                    end: found.end,
                    reading: found.reading.clone(),
                    facts,
                });
            }
        }
        let mut sets: Vec<Vec<usize>> = vec![Vec::new(); n + 1];
        let mut item_index: Vec<ChartMap<(usize, usize, usize, State)>> =
            (0..=n).map(|_| ChartMap::default()).collect();
        let mut node_index: ChartMap<(usize, usize, usize, Summary)> = ChartMap::default();
        let categories = self.categories.len();
        let mut waiting: Vec<Vec<usize>> = vec![Vec::new(); (n + 1) * categories];
        let mut predicted_at = vec![usize::MAX; categories];

        let mut add = |forest: &mut PackedForest,
                       sets: &mut Vec<Vec<usize>>,
                       production: usize,
                       dot: usize,
                       start: usize,
                       end: usize,
                       state: State,
                       family: Option<(Option<usize>, ChildRef)>| {
            let key = (production, dot, start, state);
            let item_index = &mut item_index[end];
            let index = if let Some(&i) = item_index.get(&key) {
                i
            } else {
                let i = forest.items.len();
                forest.items.push(Item {
                    production,
                    dot,
                    start,
                    end,
                    state: key.3,
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

        let mut active = self.prediction.signature();
        for position in 0..=n {
            if position != 0 && sets[position].is_empty() {
                continue;
            }
            active.fill(0);
            let at = skip(position);
            for &leaf in &by_start[at] {
                self.prediction
                    .add_leaf(self, &forest.leaves[leaf].facts, &mut active);
            }
            self.prediction.add_punctuation(
                tokens.get(position).copied(),
                tokens.get(at).copied(),
                &mut active,
            );
            if position == 0 {
                predicted_at[root] = position;
                for &p in &self.by_lhs[root] {
                    if self.prediction.production(p, &active) {
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
                }
            }
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
                        summary: key.3,
                        families: vec![(production_id, id)],
                    });
                    node_index.insert(key, node);
                    for &parent in &waiting[start * categories + production.lhs] {
                        let item = &forest.items[parent];
                        let parent_production = &self.productions[item.production];
                        let view = ChildView::Node(&forest.nodes[node].summary);
                        if !parent_production.spacing_matches(
                            &item.state,
                            item.dot,
                            &view,
                            &tokens[item.end..skip(item.end)],
                        ) {
                            continue;
                        }
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
                        waiting[position * categories + category].push(id);
                        if predicted_at[*category] == position {
                            continue;
                        }
                        predicted_at[*category] = position;
                        if !self.prediction.category(*category, &active) {
                            continue;
                        }
                        for &p in &self.by_lhs[*category] {
                            if self.prediction.production(p, &active) {
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
                            if !production.spacing_matches(
                                &forest.items[id].state,
                                dot,
                                &view,
                                &tokens[position..at],
                            ) {
                                continue;
                            }
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
                    Symbol::Punct(c, _) => {
                        let at = if c.is_whitespace() { position } else { skip(position) };
                        if at < n
                            && tokens[at] == *c
                            && production.spacing_matches(
                                &forest.items[id].state,
                                dot,
                                &ChildView::Punct,
                                &tokens[position..at],
                            )
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
        self.close(&mut forest);
        forest.roots = (0..forest.nodes.len())
            .filter(|&i| {
                let node = &forest.nodes[i];
                !node.families.is_empty()
                    && node.category == root
                    && node.start == 0
                    && skip(node.end) == n
                    && node.summary.gap == crate::Gap::None
            })
            .collect();
        Parse {
            grammar: self,
            root,
            tokens: tokens.clone(),
            forest,
            vocabulary_gaps: analyzed.unknown_words(),
        }
    }
}
