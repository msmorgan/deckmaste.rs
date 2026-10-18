//! Both round-trip laws, checked over the Packed Forest without building its
//! trees.

use std::collections::BTreeSet;

use deckmaste_lexical::Binding;
use deckmaste_lexical::LexicalReading;
use deckmaste_lexical::Lexicon;

use crate::chart::ChildRef;
use crate::chart::Parse;
use crate::reading::AdmissionError;
use crate::reading::Reading;
use crate::reading::glues_after;
use crate::reading::glues_before;
use crate::walk::ForestMemo;
use crate::walk::Visit;

/// Where some Reading of a Parse would not realize as the analyzed text.
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum RealizationFault {
    #[error("the word at chars {start}..{end} realizes as `{realized}`, not `{source_text}`")]
    Spelling {
        start: usize,
        end: usize,
        realized: String,
        source_text: String,
    },
    #[error("the spacing at char {at} is not the spacing realization writes")]
    Spacing { at: usize },
    #[error("leading or trailing whitespace is not realized")]
    Edge,
}

/// The spacing behaviour of the first and last atom of a Derivation: whether
/// the first attaches to what precedes it, and whether the last attaches to
/// what follows it.
type Edges = (bool, bool);

/// Memoized edge sets of forest nodes and items.
type EdgeMemo = ForestMemo<BTreeSet<Edges>, BTreeSet<Edges>>;

impl Parse<'_> {
    /// Round-trip law 1 over the whole forest: every Reading realizes as the
    /// analyzed text. Each reachable leaf's spelling is checked once, and the
    /// spacing between every pair of atoms adjacent in some Derivation is
    /// checked once per pair of edge behaviours, so the work is bounded by
    /// the forest's size, never by its number of trees.
    ///
    /// # Errors
    /// The first fault found.
    pub fn verify_realization(&self, lexicon: &Lexicon) -> Result<(), RealizationFault> {
        let n = self.tokens.len();
        let mut memo = EdgeMemo::default();
        for &root in &self.forest.roots {
            if self.tokens.first().is_some_and(|c| c.is_whitespace())
                || self.forest.nodes[root].end != n
            {
                return Err(RealizationFault::Edge);
            }
            self.node_edges(root, lexicon, &mut memo)?;
        }
        Ok(())
    }

    fn node_edges(
        &self,
        node: usize,
        lexicon: &Lexicon,
        memo: &mut EdgeMemo,
    ) -> Result<BTreeSet<Edges>, RealizationFault> {
        let frame = match memo.nodes.enter(node) {
            Visit::Known(edges) => return Ok(edges),
            Visit::Cycle => return Ok(BTreeSet::new()),
            Visit::Open(frame) => frame,
        };
        let mut edges = BTreeSet::new();
        for &(_, item) in &self.forest.nodes[node].families {
            edges.extend(self.item_edges(item, lexicon, memo)?);
        }
        memo.nodes.leave(frame, &edges);
        Ok(edges)
    }

    fn item_edges(
        &self,
        item: usize,
        lexicon: &Lexicon,
        memo: &mut EdgeMemo,
    ) -> Result<BTreeSet<Edges>, RealizationFault> {
        if let Some(edges) = memo.items.get(&item) {
            return Ok(edges.clone());
        }
        let mark = memo.nodes.mark();
        let mut edges = BTreeSet::new();
        for &(previous, child) in &self.forest.items[item].families {
            let last: BTreeSet<Edges> = match child {
                ChildRef::Node(n) => {
                    let edges = self.node_edges(n, lexicon, memo)?;
                    let state = &self.forest.items[item];
                    if let crate::step::Symbol::Cat {
                        binding: Some(binding),
                        ..
                    } = &self.grammar.productions[state.production].rhs[state.dot - 1]
                    {
                        edges
                            .into_iter()
                            .map(|(before, after)| {
                                (
                                    before || glues_before(*binding),
                                    after || glues_after(*binding),
                                )
                            })
                            .collect()
                    } else {
                        edges
                    }
                }
                ChildRef::Leaf(l) => {
                    let binding = self.leaf_binding(l, lexicon)?;
                    BTreeSet::from([(glues_before(binding), glues_after(binding))])
                }
                ChildRef::Punct => {
                    let state = &self.forest.items[item];
                    let crate::step::Symbol::Punct(_, binding) =
                        &self.grammar.productions[state.production].rhs[state.dot - 1]
                    else {
                        unreachable!()
                    };
                    BTreeSet::from([(glues_before(*binding), glues_after(*binding))])
                }
            };
            let Some(previous) = previous else {
                edges.extend(last);
                continue;
            };
            let at = self.forest.items[previous].end;
            let symbol = &self.grammar.productions[self.forest.items[item].production].rhs
                [self.forest.items[item].dot - 1];
            let space = if matches!(symbol, crate::step::Symbol::Punct(c, _) if c.is_whitespace()) {
                Some(false)
            } else {
                self.space_after(at)
            };
            for (first, before) in self.item_edges(previous, lexicon, memo)? {
                for &(attaches, after) in &last {
                    let glue = before || attaches;
                    if space != Some(!glue) {
                        return Err(RealizationFault::Spacing { at });
                    }
                    edges.insert((first, after));
                }
            }
        }
        if memo.nodes.settle(mark) {
            memo.items.insert(item, edges.clone());
        }
        Ok(edges)
    }

    /// Whether one space (`Some(true)`) or nothing (`Some(false)`) follows
    /// `at` before the next atom; `None` for any other whitespace.
    fn space_after(&self, at: usize) -> Option<bool> {
        let run = self.tokens[at..]
            .iter()
            .take_while(|c| c.is_whitespace())
            .count();
        match run {
            0 => Some(false),
            1 if self.tokens[at] == ' ' => Some(true),
            _ => None,
        }
    }

    /// Checks a leaf's spelling against the text and returns its binding.
    fn leaf_binding(&self, leaf: usize, lexicon: &Lexicon) -> Result<Binding, RealizationFault> {
        let leaf = &self.forest.leaves[leaf];
        let source_text: String = self.tokens[leaf.start..leaf.end].iter().collect();
        let realized = lexicon.realize(&leaf.reading).unwrap_or_default();
        if realized != source_text {
            return Err(RealizationFault::Spelling {
                start: leaf.start,
                end: leaf.end,
                realized,
                source_text,
            });
        }
        Ok(match &leaf.reading {
            LexicalReading::Word(v) => self
                .grammar
                .lexemes
                .get(&v.lexeme)
                .map_or(Binding::Free, |f| f.binding),
            _ => Binding::Free,
        })
    }

    /// Round-trip law 2's membership test: whether `reading` is one of this
    /// forest's trees, found by walking the forest along the Reading rather
    /// than enumerating trees.
    #[must_use]
    pub fn contains(&self, reading: &Reading) -> bool {
        self.forest
            .roots
            .iter()
            .any(|&root| self.matches_node(root, reading))
    }

    fn matches_node(&self, node: usize, reading: &Reading) -> bool {
        let Reading::Node(wanted) = reading else {
            return false;
        };
        let grammar = self.grammar;
        let packed = &self.forest.nodes[node];
        grammar.categories[packed.category] == wanted.category
            && packed.summary.gap == wanted.gap
            && grammar.feature_map(packed.category, &packed.summary) == wanted.features
            && packed.families.iter().any(|&(production, item)| {
                grammar.productions[production].id == wanted.production
                    && self.matches_item(item, &wanted.children)
            })
    }

    /// Whether some family chain ending at `item` spells exactly `children`.
    fn matches_item(&self, item: usize, children: &[Reading]) -> bool {
        self.forest.items[item]
            .families
            .iter()
            .any(|&(previous, child)| {
                let rest = match child {
                    ChildRef::Punct => children,
                    ChildRef::Leaf(l) => match children.split_last() {
                        Some((Reading::Leaf(value), rest))
                            if *value == self.forest.leaves[l].reading =>
                        {
                            rest
                        }
                        _ => return false,
                    },
                    ChildRef::Node(n) => match children.split_last() {
                        Some((last, rest)) if self.matches_node(n, last) => rest,
                        _ => return false,
                    },
                };
                match previous {
                    Some(p) => self.matches_item(p, rest),
                    None => rest.is_empty(),
                }
            })
    }

    /// Admission of `reading` as the Category this Parse was made for.
    ///
    /// # Errors
    /// As [`crate::CompiledGrammar::admit`].
    pub fn admit(&self, reading: &Reading, lexicon: &Lexicon) -> Result<(), AdmissionError> {
        self.grammar.admit_as(reading, lexicon, self.root)
    }

    /// The Category this Parse was made for.
    #[must_use]
    pub fn category(&self) -> &str {
        &self.grammar.categories[self.root]
    }

    /// The analyzed text this Parse was made from.
    #[must_use]
    pub fn text(&self) -> String {
        self.tokens.iter().collect()
    }
}
