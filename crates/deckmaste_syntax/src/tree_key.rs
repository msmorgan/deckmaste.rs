//! A Reading's Ranking Key computed from its tree.
//!
//! The Packed Forest computes the same key without building trees
//! (`Parse::best_ranked`, `Parse::cheapest_tier`); this tree-side computation
//! exists to check it, and is compiled only for tests (the `tree-ranking-key`
//! feature, which the test targets of this workspace enable).

use std::collections::BTreeSet;

use deckmaste_lexical::LexemeId;
use deckmaste_lexical::Lexicon;

use crate::compile::CompiledGrammar;
use crate::reading::AdmissionError;
use crate::reading::ChildFacts;
use crate::reading::Constituent;
use crate::reading::RankingKey;
use crate::reading::Reading;
use crate::reading::join;
use crate::step::Symbol;

/// What computing a Reading's key from its tree reads: its realized text
/// and Constituents.
struct KeyContext<'r> {
    lexicon: &'r Lexicon,
    /// The card-name Lexemes the Reading's text is in the scope of.
    scope: &'r BTreeSet<LexemeId>,
    text: &'r str,
    constituents: &'r [Constituent],
}

impl KeyContext<'_> {
    /// The words of Constituent `index`.
    fn words(&self, index: usize) -> u64 {
        self.text_words(
            self.constituents[index].span.start,
            self.constituents[index].span.end,
        )
    }

    /// The words of the realized text between two byte offsets.
    fn text_words(&self, start: usize, end: usize) -> u64 {
        u64::try_from(
            self.text
                .get(start..end)
                .map_or(0, |text| text.split_whitespace().count()),
        )
        .unwrap_or(u64::MAX)
    }
}

impl CompiledGrammar {
    /// A Reading's Ranking Key, computed from the tree: the sum of its
    /// Productions' keys and of the Right Association words each of its
    /// children adds, counted on the Reading's realized text. `scope` is the
    /// card-name scope its text was analyzed in.
    ///
    /// # Errors
    /// The Reading cannot be realized with this lexicon.
    pub fn ranking_key(
        &self,
        reading: &Reading,
        lexicon: &Lexicon,
        scope: &BTreeSet<LexemeId>,
    ) -> Result<RankingKey, AdmissionError> {
        let (text, constituents) = self.constituents(reading, lexicon)?;
        let context = KeyContext {
            lexicon,
            scope,
            text: &text,
            constituents: &constituents,
        };
        let mut at = 0;
        self.key_of(reading, &context, &mut at)
    }

    /// The key of the Reading whose root is Constituent `*at` in preorder.
    fn key_of(
        &self,
        reading: &Reading,
        context: &KeyContext<'_>,
        at: &mut usize,
    ) -> Result<RankingKey, AdmissionError> {
        let Reading::Node(node) = reading else {
            return Ok(RankingKey::default());
        };
        let me = *at;
        *at += 1;
        let Some(&production) = self.by_id.get(&node.production) else {
            return Ok(RankingKey::default());
        };
        let mut key = self.production_key(production);
        // Each Node child's preorder index, by its position among the
        // children.
        let mut indices = Vec::with_capacity(node.children.len());
        for child in &node.children {
            indices.push(child.node().map(|_| *at));
            key = key + self.key_of(child, context, at)?;
        }
        let names = self.child_names(node);
        let positions: Vec<usize> = self.productions[production]
            .rhs
            .iter()
            .enumerate()
            .filter(|(_, symbol)| !matches!(symbol, Symbol::Punct(..)))
            .map(|(i, _)| i)
            .collect();
        for (child, &at) in node.children.iter().zip(&positions) {
            if let Reading::Leaf(leaf) = child {
                key = key + self.leaf_key(production, at, self.scoped(leaf, context.scope));
            }
        }
        let first = names
            .iter()
            .position(|name| name.as_str() == "first")
            .and_then(|i| Some((indices[i]?, &node.children[i])));
        let head = match names.iter().position(|name| name.as_str() == "head") {
            Some(i) => match &node.children[i] {
                Reading::Node(_) => indices[i].map(|index| context.words(index)),
                leaf @ Reading::Leaf(_) => Some(self.leaf_words(leaf, context.lexicon)?),
            },
            None => None,
        };
        let start = context.constituents[me].span.start;
        for (i, child) in node.children.iter().enumerate() {
            let Some(index) = indices[i] else {
                continue;
            };
            let facts = ChildFacts {
                words: context.words(index),
                coordination: self.is_coordination(child),
                head_final: self.head_final(child),
                high_attachment: self.is_high_attachment(child),
            };
            key = key
                + self.attachment_key(
                    production,
                    positions[i],
                    facts,
                    || head,
                    || first.map(|(index, first)| (context.words(index), self.head_final(first))),
                    || context.text_words(start, context.constituents[index].span.start),
                );
        }
        Ok(key)
    }

    /// Whether a Reading is formed by a coordination.
    fn is_coordination(&self, reading: &Reading) -> bool {
        reading.node().is_some_and(|node| {
            self.by_id
                .get(&node.production)
                .is_some_and(|&p| self.productions[p].coordinator.is_some())
        })
    }

    /// Whether a Reading ends in its head: its last child is in the head line
    /// and ends in its head (a leaf does), as the chart tracks it.
    fn head_final(&self, reading: &Reading) -> bool {
        let Reading::Node(node) = reading else {
            return true;
        };
        let Some(&p) = self.by_id.get(&node.production) else {
            return false;
        };
        let production = &self.productions[p];
        let last = production
            .rhs
            .iter()
            .enumerate()
            .filter(|(_, symbol)| !matches!(symbol, Symbol::Punct(..)))
            .map(|(i, _)| i)
            .zip(&node.children)
            .last();
        last.is_some_and(|(position, child)| {
            production.head_line[position] && self.head_final(child)
        })
    }

    /// Whether a Reading attaches high by its Attachment Class.
    fn is_high_attachment(&self, reading: &Reading) -> bool {
        reading.node().is_some_and(|node| {
            self.category_id(node.category.as_str())
                .is_some_and(|category| self.attaches_high(&node.features, category))
        })
    }

    /// The words a leaf realizes.
    fn leaf_words(&self, leaf: &Reading, lexicon: &Lexicon) -> Result<u64, AdmissionError> {
        let mut atoms = Vec::new();
        self.atoms(leaf, lexicon, &mut atoms, &mut |_, _| {})?;
        Ok(u64::try_from(join(&atoms).0.split_whitespace().count()).unwrap_or(u64::MAX))
    }
}
