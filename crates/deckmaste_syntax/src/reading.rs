//! Readings: enumeration from the Packed Forest, Admission without parsing,
//! Realization, and Production Cost.

use std::collections::BTreeMap;
use std::collections::BTreeSet;
use std::collections::HashMap;
use std::ops::Range;

use deckmaste_lexical::Binding;
use deckmaste_lexical::LexemeId;
use deckmaste_lexical::LexicalReading;
use deckmaste_lexical::Lexicon;
use serde::Deserialize;
use serde::Serialize;

use crate::chart::ChildRef;
use crate::chart::Parse;
use crate::compile::CompiledGrammar;
use crate::frames::Frame;
use crate::step::ChildView;
use crate::step::Gap;
use crate::step::Summary;
use crate::step::Symbol;
use crate::walk::NodeMemo;
use crate::walk::Visit;

/// One whole tree: a grammatical analysis of an expression.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Ord, PartialOrd, Serialize)]
pub enum Reading {
    Node(ReadingNode),
    Leaf(LexicalReading),
}

/// A Constituent formed by one Production.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Ord, PartialOrd, Serialize)]
pub struct ReadingNode {
    /// The stable id of the Production.
    pub production: String,
    pub category: String,
    /// Bound features of the Constituent, by feature name.
    pub features: BTreeMap<String, String>,
    pub gap: Gap,
    /// The Selected Frame of a frame-headed phrase.
    pub selected_frame: Option<Frame>,
    /// Children in surface order; punctuation is not stored.
    pub children: Vec<Reading>,
}

impl Reading {
    #[must_use]
    pub fn node(&self) -> Option<&ReadingNode> {
        match self {
            Self::Node(node) => Some(node),
            Self::Leaf(_) => None,
        }
    }
}

/// Why a Reading is not admitted.
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum AdmissionError {
    #[error("unknown production `{0}`")]
    UnknownProduction(String),
    #[error("production `{production}`: {reason}")]
    Rejected { production: String, reason: String },
    #[error("root is `{0}`, not the grammar root without a gap")]
    Root(String),
    #[error("a leaf is not a value of this lexicon")]
    Leaf,
}

impl Parse<'_> {
    /// Number of Derivations in the forest, counted without building them
    /// (saturating). A Derivation never contains its own Constituent, so a
    /// unary cycle contributes no Derivation of its own.
    #[must_use]
    pub fn derivation_count(&self) -> u128 {
        let mut memo = NodeMemo::default();
        self.forest
            .roots
            .iter()
            .map(|&r| self.count_node(r, &mut memo))
            .fold(0u128, u128::saturating_add)
    }

    fn count_node(&self, node: usize, memo: &mut NodeMemo<u128>) -> u128 {
        let frame = match memo.enter(node) {
            Visit::Known(count) => return count,
            Visit::Cycle => return 0,
            Visit::Open(frame) => frame,
        };
        let mut total = 0u128;
        for &(_, item) in &self.forest.nodes[node].families {
            total = total.saturating_add(self.count_item(item, memo));
        }
        memo.leave(frame, &total);
        total
    }

    fn count_item(&self, item: usize, memo: &mut NodeMemo<u128>) -> u128 {
        let mut total = 0u128;
        for &(previous, child) in &self.forest.items[item].families {
            let before = previous.map_or(1, |p| self.count_item(p, memo));
            let here = match child {
                ChildRef::Node(n) => self.count_node(n, memo),
                ChildRef::Leaf(_) | ChildRef::Punct => 1,
            };
            total = total.saturating_add(before.saturating_mul(here));
        }
        total
    }

    /// Every Reading, deduplicated and ranked by total Production Cost, then
    /// by a deterministic order. This builds every tree; outside tests, use
    /// [`Parse::best_readings`] or [`Parse::derivation_count`].
    #[must_use]
    pub fn readings(&self) -> Vec<Reading> {
        self.ranked(None)
    }

    /// The `k` cheapest Readings by total Production Cost. Each forest node
    /// keeps only its `k` cheapest trees, so the work is bounded by `k` and
    /// the forest's size, never by the number of attachment trees.
    #[must_use]
    pub fn best_readings(&self, k: usize) -> Vec<Reading> {
        self.ranked(Some(k))
    }

    fn ranked(&self, limit: Option<usize>) -> Vec<Reading> {
        let mut memo = TreeMemo::default();
        let mut all = BTreeSet::new();
        for &root in &self.forest.roots {
            all.extend(self.node_trees(root, limit, &mut memo));
        }
        truncate(all, limit).into_iter().map(|(_, r)| r).collect()
    }

    fn node_trees(
        &self,
        node: usize,
        limit: Option<usize>,
        memo: &mut TreeMemo,
    ) -> Vec<(u64, Reading)> {
        let frame = match memo.nodes.enter(node) {
            Visit::Known(trees) => return trees,
            Visit::Cycle => return Vec::new(),
            Visit::Open(frame) => frame,
        };
        let grammar = self.grammar;
        let packed = &self.forest.nodes[node];
        let mut trees = BTreeSet::new();
        for &(production, item) in &packed.families {
            let p = &grammar.productions[production];
            for (cost, children) in self.item_sequences(item, limit, memo) {
                trees.insert((
                    cost + u64::from(p.cost),
                    Reading::Node(ReadingNode {
                        production: p.id.clone(),
                        category: grammar.categories[p.lhs].clone(),
                        features: grammar.feature_map(p.lhs, &packed.summary),
                        gap: packed.summary.gap,
                        selected_frame: p.frame.map(|f| grammar.frames[f].clone()),
                        children,
                    }),
                ));
            }
        }
        let trees = truncate(trees, limit);
        memo.nodes.leave(frame, &trees);
        trees
    }

    fn item_sequences(
        &self,
        item: usize,
        limit: Option<usize>,
        memo: &mut TreeMemo,
    ) -> Vec<(u64, Vec<Reading>)> {
        if let Some(sequences) = memo.items.get(&item) {
            return sequences.clone();
        }
        let mark = memo.nodes.mark();
        let mut out = BTreeSet::new();
        for &(previous, child) in &self.forest.items[item].families {
            let prefixes = match previous {
                Some(p) => self.item_sequences(p, limit, memo),
                None => vec![(0, Vec::new())],
            };
            let lasts: Vec<(u64, Option<Reading>)> = match child {
                ChildRef::Node(n) => self
                    .node_trees(n, limit, memo)
                    .into_iter()
                    .map(|(c, r)| (c, Some(r)))
                    .collect(),
                ChildRef::Leaf(l) => vec![(
                    0,
                    Some(Reading::Leaf(self.forest.leaves[l].reading.clone())),
                )],
                ChildRef::Punct => vec![(0, None)],
            };
            for (prefix_cost, prefix) in &prefixes {
                for (last_cost, last) in &lasts {
                    let mut sequence = prefix.clone();
                    sequence.extend(last.clone());
                    out.insert((prefix_cost + last_cost, sequence));
                }
            }
        }
        let sequences = truncate(out, limit);
        if memo.nodes.settle(mark) {
            memo.items.insert(item, sequences.clone());
        }
        sequences
    }
}

/// Memoized trees of forest nodes and child sequences of items. Items share
/// prefixes across Derivations, so each item's sequences are built once.
#[derive(Default)]
struct TreeMemo {
    nodes: NodeMemo<Vec<(u64, Reading)>>,
    items: HashMap<usize, Vec<(u64, Vec<Reading>)>>,
}

/// The cheapest `limit` entries of an ordered set, or all of them.
fn truncate<T: Ord>(set: BTreeSet<(u64, T)>, limit: Option<usize>) -> Vec<(u64, T)> {
    set.into_iter().take(limit.unwrap_or(usize::MAX)).collect()
}

impl CompiledGrammar {
    /// Total Production Cost of a Reading; leaves cost nothing.
    #[must_use]
    pub fn cost(&self, reading: &Reading) -> u64 {
        match reading {
            Reading::Leaf(_) => 0,
            Reading::Node(node) => {
                let own = self
                    .by_id
                    .get(&node.production)
                    .map_or(0, |&p| u64::from(self.productions[p].cost));
                own + node.children.iter().map(|c| self.cost(c)).sum::<u64>()
            }
        }
    }

    pub(crate) fn feature_map(
        &self,
        category: usize,
        summary: &Summary,
    ) -> BTreeMap<String, String> {
        self.category_features[category]
            .iter()
            .zip(summary.values.iter())
            .filter_map(|(&f, v)| {
                v.map(|v| {
                    (
                        self.features[f].clone(),
                        self.values[f][usize::from(v)].clone(),
                    )
                })
            })
            .collect()
    }

    /// Admission without parsing: whether `reading` is a Reading of the root
    /// Category that this grammar and lexicon accept.
    ///
    /// # Errors
    /// The first violation found.
    pub fn admit(&self, reading: &Reading, lexicon: &Lexicon) -> Result<(), AdmissionError> {
        self.admit_as(reading, lexicon, self.root)
    }

    /// Admission without parsing as the named Category instead of the root.
    ///
    /// # Errors
    /// The first violation found, or [`AdmissionError::Root`] when the grammar
    /// has no such Category.
    pub fn admit_category(
        &self,
        reading: &Reading,
        lexicon: &Lexicon,
        category: &str,
    ) -> Result<(), AdmissionError> {
        let root = self
            .category_id(category)
            .ok_or_else(|| AdmissionError::Root(category.to_owned()))?;
        self.admit_as(reading, lexicon, root)
    }

    pub(crate) fn admit_as(
        &self,
        reading: &Reading,
        lexicon: &Lexicon,
        root: usize,
    ) -> Result<(), AdmissionError> {
        let (category, summary) = self.summarize(reading, lexicon)?;
        if category != root || summary.gap != Gap::None {
            return Err(AdmissionError::Root(self.categories[category].clone()));
        }
        let mut atoms = Vec::new();
        self.atoms(reading, lexicon, &mut atoms, &mut |_, _| {})?;
        let (text, offsets) = join(&atoms);
        let analyzed = lexicon.analyze(&text);
        let atomic = self.atomic_spans(&analyzed);
        if offsets.iter().any(|offset| {
            let start = text[..offset.start].chars().count();
            let end = text[..offset.end].chars().count();
            atomic.iter().any(|span| {
                (span.start < start && start < span.end) || (span.start < end && end < span.end)
            })
        }) {
            return Err(AdmissionError::Rejected {
                production: "lexical atomicity".into(),
                reason: "a declared multiword noun is decomposed".into(),
            });
        }
        Ok(())
    }

    fn summarize(
        &self,
        reading: &Reading,
        lexicon: &Lexicon,
    ) -> Result<(usize, Summary), AdmissionError> {
        let Reading::Node(node) = reading else {
            return Err(AdmissionError::Leaf);
        };
        let &index = self
            .by_id
            .get(&node.production)
            .ok_or_else(|| AdmissionError::UnknownProduction(node.production.clone()))?;
        let production = &self.productions[index];
        let reject = |reason: &str| AdmissionError::Rejected {
            production: node.production.clone(),
            reason: reason.to_owned(),
        };
        let mut children = node.children.iter();
        let mut state = production.start();
        for (position, symbol) in production.rhs.iter().enumerate() {
            let next = match symbol {
                Symbol::Punct(..) => production.accept(self, &state, position, &ChildView::Punct),
                Symbol::Leaf(test) => {
                    let Some(Reading::Leaf(value)) = children.next() else {
                        return Err(reject("expected a lexical leaf"));
                    };
                    lexicon.realize(value).map_err(|_| AdmissionError::Leaf)?;
                    let facts = self.leaf_facts(value).ok_or(AdmissionError::Leaf)?;
                    if !self.leaf_matches(test, &facts) {
                        return Err(reject("leaf does not satisfy its symbol"));
                    }
                    production.accept(self, &state, position, &ChildView::Leaf(&facts))
                }
                Symbol::Cat { category, .. } => {
                    let Some(child) = children.next() else {
                        return Err(reject("missing child"));
                    };
                    let (child_category, summary) = self.summarize(child, lexicon)?;
                    if child_category != *category {
                        return Err(reject("child has the wrong category"));
                    }
                    production.accept(self, &state, position, &ChildView::Node(&summary))
                }
            };
            state = next.ok_or_else(|| reject("child does not fit"))?;
        }
        if children.next().is_some() {
            return Err(reject("extra children"));
        }
        let summary = production
            .finish(self, &state)
            .ok_or_else(|| reject("constituent does not complete"))?;
        if self.categories[production.lhs] != node.category
            || self.feature_map(production.lhs, &summary) != node.features
            || summary.gap != node.gap
            || production.frame.map(|f| &self.frames[f]) != node.selected_frame.as_ref()
        {
            return Err(reject("recorded category, features, gap or frame differ"));
        }
        Ok((production.lhs, summary))
    }

    /// Realizes a Reading as text: words through the lexicon, punctuation from
    /// the Productions.
    ///
    /// # Errors
    /// A leaf the lexicon does not license, or a tree that does not match its
    /// Productions.
    pub fn realize(&self, reading: &Reading, lexicon: &Lexicon) -> Result<String, AdmissionError> {
        let mut atoms = Vec::new();
        self.atoms(reading, lexicon, &mut atoms, &mut |_, _| {})?;
        Ok(join(&atoms).0)
    }

    /// Every Constituent of a Reading in preorder, located in its
    /// realization, which is returned with them.
    ///
    /// # Errors
    /// As [`CompiledGrammar::realize`].
    pub fn constituents(
        &self,
        reading: &Reading,
        lexicon: &Lexicon,
    ) -> Result<(String, Vec<Constituent>), AdmissionError> {
        let mut atoms = Vec::new();
        let mut spans: Vec<(usize, usize)> = Vec::new();
        self.atoms(reading, lexicon, &mut atoms, &mut |index, range| {
            if index == spans.len() {
                spans.push(range);
            } else {
                spans[index] = range;
            }
        })?;
        let (text, offsets) = join(&atoms);
        let mut out = Vec::new();
        self.locate(reading, None, "", &mut out);
        for (constituent, &(first, last)) in out.iter_mut().zip(&spans) {
            constituent.span = if first == last {
                let at = offsets.get(first).map_or(text.len(), |r| r.start);
                at..at
            } else {
                offsets[first].start..offsets[last - 1].end
            };
        }
        Ok((text, out))
    }

    /// Preorder Constituents without spans; `atoms` fills the spans in the
    /// same order.
    fn locate(
        &self,
        reading: &Reading,
        parent: Option<usize>,
        function: &str,
        out: &mut Vec<Constituent>,
    ) {
        let Reading::Node(node) = reading else {
            return;
        };
        let index = out.len();
        out.push(Constituent {
            category: node.category.clone(),
            span: 0..0,
            function: function.to_owned(),
            parent,
            head: self.head_lexeme(reading),
            gap: node.gap,
        });
        // The head of a function Category bears its function in its phrase.
        let wraps = self.function_categories.contains(&node.category);
        let names = self.child_names(node);
        for (child, name) in node.children.iter().zip(names) {
            if wraps && name == "head" {
                self.locate(child, parent, function, out);
            } else {
                self.locate(child, Some(index), &name, out);
            }
        }
    }

    /// The names of a node's children, in order: the grammatical function
    /// each bears in it.
    fn child_names(&self, node: &ReadingNode) -> Vec<String> {
        self.by_id
            .get(&node.production)
            .map_or_else(Vec::new, |&p| {
                let production = &self.productions[p];
                production
                    .rhs
                    .iter()
                    .zip(&production.names)
                    .filter(|(symbol, _)| !matches!(symbol, Symbol::Punct(..)))
                    .map(|(_, name)| name.clone())
                    .collect()
            })
    }

    /// The lexeme reached by following `head` children down from `reading`.
    #[must_use]
    pub fn head_lexeme(&self, reading: &Reading) -> Option<LexemeId> {
        match reading {
            Reading::Leaf(LexicalReading::Word(value)) => Some(value.lexeme),
            Reading::Leaf(_) => None,
            Reading::Node(node) => {
                let names = self.child_names(node);
                let at = names.iter().position(|n| n == "head")?;
                self.head_lexeme(node.children.get(at)?)
            }
        }
    }

    /// Pushes a Reading's atoms (text, binding, punctuation) in order, and
    /// reports each node's preorder index with its atom range.
    fn atoms(
        &self,
        reading: &Reading,
        lexicon: &Lexicon,
        out: &mut Vec<(String, Binding, bool)>,
        located: &mut dyn FnMut(usize, (usize, usize)),
    ) -> Result<(), AdmissionError> {
        let mut next = 0;
        self.atoms_from(reading, lexicon, out, located, &mut next)
    }

    fn atoms_from(
        &self,
        reading: &Reading,
        lexicon: &Lexicon,
        out: &mut Vec<(String, Binding, bool)>,
        located: &mut dyn FnMut(usize, (usize, usize)),
        next: &mut usize,
    ) -> Result<(), AdmissionError> {
        match reading {
            Reading::Leaf(value) => {
                let text = lexicon.realize(value).map_err(|_| AdmissionError::Leaf)?;
                let binding = match value {
                    LexicalReading::Word(v) => self
                        .lexemes
                        .get(&v.lexeme)
                        .map_or(Binding::Free, |f| f.binding),
                    _ => Binding::Free,
                };
                out.push((text, binding, false));
            }
            Reading::Node(node) => {
                let &index = self
                    .by_id
                    .get(&node.production)
                    .ok_or_else(|| AdmissionError::UnknownProduction(node.production.clone()))?;
                let me = *next;
                *next += 1;
                let first = out.len();
                located(me, (first, first));
                let mut children = node.children.iter();
                for symbol in &self.productions[index].rhs {
                    if let Symbol::Punct(c, binding) = symbol {
                        out.push((c.to_string(), *binding, false));
                    } else {
                        let child = children.next().ok_or_else(|| AdmissionError::Rejected {
                            production: node.production.clone(),
                            reason: "missing child".to_owned(),
                        })?;
                        let first = out.len();
                        self.atoms_from(child, lexicon, out, located, next)?;
                        if let Symbol::Cat {
                            binding: Some(binding),
                            ..
                        } = symbol
                        {
                            if glues_before(*binding) {
                                out[first].1 = with_before(out[first].1);
                            }
                            if glues_after(*binding) {
                                let last = out.len() - 1;
                                out[last].1 = with_after(out[last].1);
                            }
                        }
                    }
                }
                located(me, (first, out.len()));
            }
        }
        Ok(())
    }
}

/// One Constituent of a Reading, as [`CompiledGrammar::constituents`] locates
/// it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Constituent {
    pub category: String,
    /// Byte range in the realized text.
    pub span: Range<usize>,
    /// Its name in its mother's Production: the grammatical function it
    /// bears there (`subject`, `object`, `modifier`, ...); empty for the
    /// root.
    pub function: String,
    /// Preorder index of its mother; the head of a function Category
    /// (`CategoryDecl::function`) takes that Category's mother and function.
    pub parent: Option<usize>,
    /// The lexeme reached through its `head` children.
    pub head: Option<LexemeId>,
    pub gap: Gap,
}

/// Joins atoms with the spacing their bindings call for, returning the text
/// and each atom's byte range in it.
pub(crate) fn join(atoms: &[(String, Binding, bool)]) -> (String, Vec<Range<usize>>) {
    let mut out = String::new();
    let mut ranges = Vec::with_capacity(atoms.len());
    let mut glue_next = true;
    for (text, binding, punct) in atoms {
        if !(glue_next || *punct || glues_before(*binding)) {
            out.push(' ');
        }
        let start = out.len();
        out.push_str(text);
        ranges.push(start..out.len());
        glue_next = glues_after(*binding) && !punct;
    }
    (out, ranges)
}

/// Whether an atom with this binding attaches to the atom before it.
pub(crate) fn glues_before(binding: Binding) -> bool {
    matches!(binding, Binding::Suffix | Binding::Bound)
}

/// Whether an atom with this binding attaches to the atom after it.
pub(crate) fn glues_after(binding: Binding) -> bool {
    matches!(binding, Binding::Prefix | Binding::Bound)
}

pub(crate) fn with_before(binding: Binding) -> Binding {
    if glues_after(binding) { Binding::Bound } else { Binding::Suffix }
}
pub(crate) fn with_after(binding: Binding) -> Binding {
    if glues_before(binding) { Binding::Bound } else { Binding::Prefix }
}
