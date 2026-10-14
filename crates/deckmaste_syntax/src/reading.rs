//! Readings: enumeration from the Packed Forest, Admission without parsing,
//! Realization, and Production Cost.

use std::collections::BTreeMap;
use std::collections::BTreeSet;
use std::collections::HashMap;

use deckmaste_lexical::Binding;
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

/// One whole tree: a grammatical analysis of an expression.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum Reading {
    Node(ReadingNode),
    Leaf(LexicalReading),
}

/// A Constituent formed by one Production.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
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
    /// Number of distinct trees in the forest, counted without building them
    /// (saturating).
    #[must_use]
    pub fn derivation_count(&self) -> u128 {
        let mut memo = HashMap::new();
        let mut active = BTreeSet::new();
        self.forest
            .roots
            .iter()
            .map(|&r| self.count_node(r, &mut memo, &mut active))
            .fold(0u128, u128::saturating_add)
    }

    fn count_node(
        &self,
        node: usize,
        memo: &mut HashMap<usize, u128>,
        active: &mut BTreeSet<usize>,
    ) -> u128 {
        if let Some(&c) = memo.get(&node) {
            return c;
        }
        if !active.insert(node) {
            return 0;
        }
        let mut total = 0u128;
        for &(_, item) in &self.forest.nodes[node].families {
            total = total.saturating_add(self.count_item(item, memo, active));
        }
        active.remove(&node);
        memo.insert(node, total);
        total
    }

    fn count_item(
        &self,
        item: usize,
        memo: &mut HashMap<usize, u128>,
        active: &mut BTreeSet<usize>,
    ) -> u128 {
        let mut total = 0u128;
        for &(previous, child) in &self.forest.items[item].families {
            let before = previous.map_or(1, |p| self.count_item(p, memo, active));
            let here = match child {
                ChildRef::Node(n) => self.count_node(n, memo, active),
                ChildRef::Leaf(_) | ChildRef::Punct => 1,
            };
            total = total.saturating_add(before.saturating_mul(here));
        }
        total
    }

    /// Every Reading, deduplicated and ranked by total Production Cost, then
    /// by a deterministic order. Enumeration is for tests and display; it
    /// builds every tree, so callers bound it by the forest's size.
    #[must_use]
    pub fn readings(&self) -> Vec<Reading> {
        let mut memo = HashMap::new();
        let mut active = BTreeSet::new();
        let mut all = BTreeSet::new();
        for &root in &self.forest.roots {
            all.extend(self.node_trees(root, &mut memo, &mut active));
        }
        let mut ranked: Vec<(u64, Reading)> = all
            .into_iter()
            .map(|r| (self.grammar.cost(&r), r))
            .collect();
        ranked.sort();
        ranked.into_iter().map(|(_, r)| r).collect()
    }

    fn node_trees(
        &self,
        node: usize,
        memo: &mut HashMap<usize, Vec<Reading>>,
        active: &mut BTreeSet<usize>,
    ) -> Vec<Reading> {
        if let Some(trees) = memo.get(&node) {
            return trees.clone();
        }
        if !active.insert(node) {
            return Vec::new();
        }
        let grammar = self.grammar;
        let packed = &self.forest.nodes[node];
        let mut trees = BTreeSet::new();
        for &(production, item) in &packed.families {
            let p = &grammar.productions[production];
            for children in self.item_sequences(item, memo, active) {
                trees.insert(Reading::Node(ReadingNode {
                    production: p.id.clone(),
                    category: grammar.categories[p.lhs].clone(),
                    features: grammar.feature_map(p.lhs, &packed.summary),
                    gap: packed.summary.gap,
                    selected_frame: p.frame.map(|f| grammar.frames[f].clone()),
                    children,
                }));
            }
        }
        active.remove(&node);
        let trees: Vec<Reading> = trees.into_iter().collect();
        memo.insert(node, trees.clone());
        trees
    }

    fn item_sequences(
        &self,
        item: usize,
        memo: &mut HashMap<usize, Vec<Reading>>,
        active: &mut BTreeSet<usize>,
    ) -> Vec<Vec<Reading>> {
        let mut out = Vec::new();
        for &(previous, child) in &self.forest.items[item].families {
            let prefixes = match previous {
                Some(p) => self.item_sequences(p, memo, active),
                None => vec![Vec::new()],
            };
            let lasts: Vec<Option<Reading>> = match child {
                ChildRef::Node(n) => self
                    .node_trees(n, memo, active)
                    .into_iter()
                    .map(Some)
                    .collect(),
                ChildRef::Leaf(l) => {
                    vec![Some(Reading::Leaf(self.forest.leaves[l].reading.clone()))]
                }
                ChildRef::Punct => vec![None],
            };
            for prefix in &prefixes {
                for last in &lasts {
                    let mut sequence = prefix.clone();
                    sequence.extend(last.clone());
                    out.push(sequence);
                }
            }
        }
        out
    }
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
        let (category, summary) = self.summarize(reading, lexicon)?;
        if category != self.root || summary.gap != Gap::None {
            return Err(AdmissionError::Root(self.categories[category].clone()));
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
                Symbol::Punct(_) => production.accept(self, &state, position, &ChildView::Punct),
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
        self.atoms(reading, lexicon, &mut atoms)?;
        let mut out = String::new();
        let mut glue_next = true;
        for (text, binding, punct) in atoms {
            let glue = glue_next || punct || matches!(binding, Binding::Suffix | Binding::Bound);
            if !glue {
                out.push(' ');
            }
            out.push_str(&text);
            glue_next = matches!(binding, Binding::Prefix | Binding::Bound) && !punct;
        }
        Ok(out)
    }

    fn atoms(
        &self,
        reading: &Reading,
        lexicon: &Lexicon,
        out: &mut Vec<(String, Binding, bool)>,
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
                let mut children = node.children.iter();
                for symbol in &self.productions[index].rhs {
                    if let Symbol::Punct(c) = symbol {
                        out.push((c.to_string(), Binding::Free, true));
                    } else {
                        let child = children.next().ok_or_else(|| AdmissionError::Rejected {
                            production: node.production.clone(),
                            reason: "missing child".to_owned(),
                        })?;
                        self.atoms(child, lexicon, out)?;
                    }
                }
            }
        }
        Ok(())
    }
}
