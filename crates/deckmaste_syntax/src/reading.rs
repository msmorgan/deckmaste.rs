//! Readings: enumeration from the Packed Forest, Admission without parsing,
//! Realization, and the Ranking Key.

use std::collections::BTreeMap;
use std::collections::BTreeSet;
use std::ops::Add;
use std::ops::Range;

use deckmaste_lexical::Binding;
use deckmaste_lexical::LexemeId;
use deckmaste_lexical::LexicalReading;
use deckmaste_lexical::LexicalValue;
use deckmaste_lexical::Lexicon;
use dpsi::Ident;
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
use crate::walk::ForestMemo;
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
    pub production: Ident,
    pub category: Ident,
    /// Bound features of the Constituent, by feature name.
    pub features: BTreeMap<Ident, Ident>,
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

/// The Ranking Key (`docs/decisions/english-v4.md` §4.3): the Preference that
/// orders a Parse's Readings, compared component by component in field order,
/// lower first. A Reading's own order is the final tie-break, so the cheapest
/// Reading is unique. Every component is a sum over the Reading's Productions
/// or attachments, which keeps extraction over the Packed Forest
/// compositional.
#[derive(
    Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash, Deserialize, Serialize,
)]
pub struct RankingKey {
    /// Attachments that violate a structural Attachment Class.
    pub attachment_class: u64,
    /// Right Association: the words between unselected dependents and the
    /// heads of their hosts, reversed for a dependent that attaches high.
    pub right_association_words: i64,
    /// The summed Production Cost.
    pub production_cost: u64,
}

impl Add for RankingKey {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            attachment_class: self.attachment_class.saturating_add(other.attachment_class),
            right_association_words: self
                .right_association_words
                .saturating_add(other.right_association_words),
            production_cost: self.production_cost.saturating_add(other.production_cost),
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
        let mut memo = ForestMemo::default();
        self.forest
            .roots
            .iter()
            .map(|&r| self.count_node(r, &mut memo))
            .fold(0u128, u128::saturating_add)
    }

    /// The Derivations of a completed Constituent.
    #[must_use]
    pub fn count_node(&self, node: usize, memo: &mut ForestMemo<u128, u128>) -> u128 {
        let frame = match memo.nodes.enter(node) {
            Visit::Known(count) => return count,
            Visit::Cycle => return 0,
            Visit::Open(frame) => frame,
        };
        let mut total = 0u128;
        for &(_, item) in &self.forest.nodes[node].families {
            total = total.saturating_add(self.count_item(item, memo));
        }
        memo.nodes.leave(frame, &total);
        total
    }

    fn count_item(&self, item: usize, memo: &mut ForestMemo<u128, u128>) -> u128 {
        if let Some(&count) = memo.items.get(&item) {
            return count;
        }
        let mark = memo.nodes.mark();
        let mut total = 0u128;
        for &(previous, child) in &self.forest.items[item].families {
            let before = previous.map_or(1, |p| self.count_item(p, memo));
            let here = match child {
                ChildRef::Node(n) => self.count_node(n, memo),
                ChildRef::Leaf(_) | ChildRef::Punct => 1,
            };
            total = total.saturating_add(before.saturating_mul(here));
        }
        if memo.nodes.settle(mark) {
            memo.items.insert(item, total);
        }
        total
    }

    /// Every Reading, deduplicated and ranked by its Ranking Key, then by its
    /// own order. This builds every tree; outside tests, use
    /// [`Parse::best_readings`] or [`Parse::derivation_count`].
    #[must_use]
    pub fn readings(&self) -> Vec<Reading> {
        self.ranked(None).into_iter().map(|(_, r)| r).collect()
    }

    /// The `k` best Readings by Ranking Key. Each forest node keeps only its
    /// `k` best trees, so the work is bounded by `k` and the forest's size,
    /// never by the number of attachment trees.
    #[must_use]
    pub fn best_readings(&self, k: usize) -> Vec<Reading> {
        self.ranked(Some(k)).into_iter().map(|(_, r)| r).collect()
    }

    /// The `k` best Readings with their Ranking Keys, best first.
    #[must_use]
    pub fn best_ranked(&self, k: usize) -> Vec<(RankingKey, Reading)> {
        self.ranked(Some(k))
    }

    /// The least Ranking Key in the forest and the number of Derivations that
    /// have it: the size of the cheapest tier before the final tie-break.
    /// Counted without building trees; `None` without a Derivation.
    #[must_use]
    pub fn cheapest_tier(&self) -> Option<(RankingKey, u128)> {
        let mut memo = ForestMemo::default();
        let mut best = None;
        for &root in &self.forest.roots {
            best = least(best, self.tier_node(root, &mut memo));
        }
        best
    }

    fn tier_node(&self, node: usize, memo: &mut TierMemo) -> Tier {
        let frame = match memo.nodes.enter(node) {
            Visit::Known(tier) => return tier,
            Visit::Cycle => return None,
            Visit::Open(frame) => frame,
        };
        let mut best = None;
        for &(production, item) in &self.forest.nodes[node].families {
            let own = self.grammar.production_key(production);
            for (_, tier) in self.tier_item(item, memo) {
                best = least(best, tier.map(|(key, n)| (key + own, n)));
            }
        }
        memo.nodes.leave(frame, &best);
        best
    }

    /// An item's cheapest tier for each [`Anchors`] its Derivations leave.
    fn tier_item(&self, item: usize, memo: &mut TierMemo) -> Vec<(Anchors, Tier)> {
        if let Some(tiers) = memo.items.get(&item) {
            return tiers.clone();
        }
        let mark = memo.nodes.mark();
        let mut best: BTreeMap<Anchors, Tier> = BTreeMap::new();
        for &(previous, child) in &self.forest.items[item].families {
            let befores = match previous {
                Some(p) => self.tier_item(p, memo),
                None => vec![(Anchors::default(), Some((RankingKey::default(), 1)))],
            };
            let here = match child {
                ChildRef::Node(n) => self.tier_node(n, memo),
                ChildRef::Leaf(l) => Some((self.leaf_key(item, l), 1)),
                ChildRef::Punct => Some((RankingKey::default(), 1)),
            };
            for (anchors, before) in befores {
                let edge = self.edge_key(item, anchors, previous, child);
                if let (Some((k1, n1)), Some((k2, n2))) = (before, here) {
                    let slot = best.entry(self.anchors(item, anchors, child)).or_default();
                    *slot = least(*slot, Some((k1 + k2 + edge, n1.saturating_mul(n2))));
                }
            }
        }
        let tiers: Vec<(Anchors, Tier)> = best.into_iter().collect();
        if memo.nodes.settle(mark) {
            memo.items.insert(item, tiers.clone());
        }
        tiers
    }

    /// The key leaf `leaf` adds as the child that `item` took last.
    fn leaf_key(&self, item: usize, leaf: usize) -> RankingKey {
        let it = &self.forest.items[item];
        self.grammar
            .leaf_key(it.production, it.dot - 1, self.forest.leaves[leaf].scoped)
    }

    /// The Ranking Key a child adds where an item takes it after `previous`,
    /// whose Derivation left `anchors`: its Right Association words
    /// ([`CompiledGrammar::association_words`]).
    fn edge_key(
        &self,
        item: usize,
        anchors: Anchors,
        previous: Option<usize>,
        child: ChildRef,
    ) -> RankingKey {
        let ChildRef::Node(n) = child else {
            return RankingKey::default();
        };
        let it = &self.forest.items[item];
        let node = &self.forest.nodes[n];
        let child = ChildFacts {
            words: self.words(node.start, node.end),
            coordination: node.summary.coordinator.is_some(),
            head_final: node.summary.head_final,
            high_attachment: self.high_attachment(n),
        };
        let head = || anchors.head.map(|(start, end)| self.words(start, end));
        let first = || {
            let (start, end) = anchors.first?;
            // Only punctuation and the coordinator, which keep it, follow the
            // first conjunct, so the previous item records whether the first
            // conjunct ends in its head, for this family.
            let head_final = self.forest.items[previous?].state.head_final;
            Some((self.words(start, end), head_final))
        };
        self.grammar
            .attachment_key(it.production, it.dot - 1, child, head, first, || {
                self.words(it.start, node.start)
            })
    }

    /// Whether a forest node attaches high by its Attachment Class, by the
    /// grammar's `high_attachment` declarations.
    fn high_attachment(&self, node: usize) -> bool {
        let packed = &self.forest.nodes[node];
        self.grammar.attaches_high(
            &self.grammar.feature_map(packed.category, &packed.summary),
            packed.category,
        )
    }

    /// The [`Anchors`] a Derivation leaves once `item` takes `child` after a
    /// prefix that left `anchors`: the child's extent when it is the head, or
    /// the first conjunct, whose words a later child's key reads.
    fn anchors(&self, item: usize, mut anchors: Anchors, child: ChildRef) -> Anchors {
        let it = &self.forest.items[item];
        let names = &self.grammar.productions[it.production].names;
        let at = it.dot - 1;
        let later = |read: &[&str]| names[at + 1..].iter().any(|n| read.contains(&n.as_str()));
        let extent = || match child {
            ChildRef::Node(n) => Some((self.forest.nodes[n].start, self.forest.nodes[n].end)),
            ChildRef::Leaf(l) => Some((self.forest.leaves[l].start, self.forest.leaves[l].end)),
            ChildRef::Punct => None,
        };
        match names[at].as_str() {
            "head" if later(&["adjunct", "modifier", "supplement"]) => anchors.head = extent(),
            "first" if later(&["last"]) => anchors.first = extent(),
            _ => {}
        }
        anchors
    }

    /// The words of the text between two chart positions: maximal runs of
    /// characters that are not whitespace.
    fn words(&self, start: usize, end: usize) -> u64 {
        let tokens = &self.tokens[start..end];
        let starts = tokens
            .iter()
            .enumerate()
            .filter(|&(i, c)| !c.is_whitespace() && (i == 0 || tokens[i - 1].is_whitespace()))
            .count();
        u64::try_from(starts).unwrap_or(u64::MAX)
    }

    fn ranked(&self, limit: Option<usize>) -> Vec<(RankingKey, Reading)> {
        let mut memo = ForestMemo::default();
        let mut all = BTreeSet::new();
        for &root in &self.forest.roots {
            all.extend(self.node_trees(root, limit, &mut memo));
        }
        truncate(all, limit)
    }

    fn node_trees(
        &self,
        node: usize,
        limit: Option<usize>,
        memo: &mut TreeMemo,
    ) -> Vec<(RankingKey, Reading)> {
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
            let own = grammar.production_key(production);
            for (key, children) in self
                .item_sequences(item, limit, memo)
                .into_iter()
                .flat_map(|(_, sequences)| sequences)
            {
                trees.insert((
                    key + own,
                    Reading::Node(ReadingNode {
                        production: p.id,
                        category: grammar.categories[p.lhs],
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

    /// An item's `limit` best child sequences for each [`Anchors`] their
    /// Derivations leave.
    fn item_sequences(
        &self,
        item: usize,
        limit: Option<usize>,
        memo: &mut TreeMemo,
    ) -> Vec<(Anchors, Sequences)> {
        if let Some(sequences) = memo.items.get(&item) {
            return sequences.clone();
        }
        let mark = memo.nodes.mark();
        let mut out: BTreeMap<Anchors, BTreeSet<(RankingKey, Vec<Reading>)>> = BTreeMap::new();
        for &(previous, child) in &self.forest.items[item].families {
            let prefixes = match previous {
                Some(p) => self.item_sequences(p, limit, memo),
                None => vec![(
                    Anchors::default(),
                    vec![(RankingKey::default(), Vec::new())],
                )],
            };
            let lasts: Vec<(RankingKey, Option<Reading>)> = match child {
                ChildRef::Node(n) => self
                    .node_trees(n, limit, memo)
                    .into_iter()
                    .map(|(k, r)| (k, Some(r)))
                    .collect(),
                ChildRef::Leaf(l) => vec![(
                    self.leaf_key(item, l),
                    Some(Reading::Leaf(self.forest.leaves[l].reading.clone())),
                )],
                ChildRef::Punct => vec![(RankingKey::default(), None)],
            };
            for (anchors, prefixes) in prefixes {
                let edge = self.edge_key(item, anchors, previous, child);
                let group = out.entry(self.anchors(item, anchors, child)).or_default();
                for (prefix_key, prefix) in &prefixes {
                    for (last_key, last) in &lasts {
                        let mut sequence = prefix.clone();
                        sequence.extend(last.clone());
                        group.insert((*prefix_key + *last_key + edge, sequence));
                    }
                }
            }
        }
        let sequences: Vec<_> = out
            .into_iter()
            .map(|(anchors, group)| (anchors, truncate(group, limit)))
            .collect();
        if memo.nodes.settle(mark) {
            memo.items.insert(item, sequences.clone());
        }
        sequences
    }
}

/// Child sequences, each with the Ranking Key it adds.
type Sequences = Vec<(RankingKey, Vec<Reading>)>;
type TreeMemo = ForestMemo<Vec<(RankingKey, Reading)>, Vec<(Anchors, Sequences)>>;

/// The extents, as chart positions, of the children of a Derivation's prefix
/// that a later child's Ranking Key reads: its head, read by an Adjunct,
/// Modifier or Supplement, and its first conjunct, read by the last. One
/// item packs prefixes that may divide its text differently, so the forest's
/// walks keep a result per `Anchors`, and each key is the one its tree has.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord)]
struct Anchors {
    head: Option<(usize, usize)>,
    first: Option<(usize, usize)>,
}

/// What the Ranking Key reads of a child Constituent.
#[derive(Debug, Clone, Copy)]
pub(crate) struct ChildFacts {
    /// Its length in words.
    pub words: u64,
    /// It is a coordination.
    pub coordination: bool,
    /// It ends in its head, not in a dependent.
    pub head_final: bool,
    /// It attaches high by its Attachment Class, by the grammar's
    /// `high_attachment` declarations (a distributive Preposition Phrase,
    /// *for each …*).
    pub high_attachment: bool,
}

/// A forest node's or item's cheapest tier: its least key and how many
/// Derivations have it.
type Tier = Option<(RankingKey, u128)>;
type TierMemo = ForestMemo<Tier, Vec<(Anchors, Tier)>>;

/// The lesser of two tiers, adding their counts when their keys are equal.
fn least(a: Tier, b: Tier) -> Tier {
    match (a, b) {
        (None, t) | (t, None) => t,
        (Some((ka, na)), Some((kb, nb))) => Some(match ka.cmp(&kb) {
            std::cmp::Ordering::Less => (ka, na),
            std::cmp::Ordering::Greater => (kb, nb),
            std::cmp::Ordering::Equal => (ka, na.saturating_add(nb)),
        }),
    }
}

/// The best `limit` entries of an ordered set, or all of them.
fn truncate<T: Ord>(set: BTreeSet<(RankingKey, T)>, limit: Option<usize>) -> Vec<(RankingKey, T)> {
    set.into_iter().take(limit.unwrap_or(usize::MAX)).collect()
}

impl CompiledGrammar {
    /// Whether a Constituent of `category` with these features attaches high
    /// by its Attachment Class: the grammar declares its Category with a
    /// value it carries in `high_attachment` (a Preposition Phrase whose
    /// Complement has a distributive determinative, *for each …*).
    pub(crate) fn attaches_high(&self, features: &BTreeMap<Ident, Ident>, category: usize) -> bool {
        self.high_attachment
            .iter()
            .any(|(declared, feature, value)| {
                self.categories[category] == *declared && features.get(feature) == Some(value)
            })
    }

    /// The Attachment Class and Right Association words
    /// (`docs/decisions/english-v4.md` §4.3) a child adds at position `index`
    /// of a Production. `head` gives the phrase's head's words; `first` a
    /// coordination's first conjunct's words and whether it ends in its head;
    /// `before` the words from the phrase's start to the child.
    /// - The head of a phrase that unselected dependents (`adjunct`,
    ///   `modifier`) follow is the host they attach to: the head adds its words
    ///   for each of them, unless it is a coordination, and each adds the words
    ///   between the host and itself (a selected Object before a postposed or
    ///   interleaved dependent).
    /// - A dependent whose class attaches high counts its distance from its
    ///   host's head reversed instead: a Supplement (`supplement`), and a
    ///   dependent the grammar declares in `high_attachment` (a distributive
    ///   adjunct, *for each …*), which also takes back what the head added for
    ///   it.
    /// - The last conjunct of a coordination adds the difference between its
    ///   length and its first conjunct's (conjunct balance), and violates an
    ///   Attachment Class when it ends in a dependent its first conjunct, which
    ///   ends in its head, lacks: that dependent belongs to the whole
    ///   coordination.
    /// - A dependent that precedes its head after another dependent of that
    ///   head violates an Attachment Class: it belongs inside the dependent
    ///   before it, where that can hold it (*At the beginning of each combat
    ///   this turn, …*).
    pub(crate) fn attachment_key(
        &self,
        production: usize,
        index: usize,
        child: ChildFacts,
        head: impl FnOnce() -> Option<u64>,
        first: impl FnOnce() -> Option<(u64, bool)>,
        before: impl FnOnce() -> u64,
    ) -> RankingKey {
        let names = &self.productions[production].names;
        let signed = |words: u64| i64::try_from(words).unwrap_or(i64::MAX);
        let after_head = || names[..index].iter().any(|name| name.as_str() == "head");
        let mut key = RankingKey::default();
        match names[index].as_str() {
            "head" if !child.coordination => {
                let dependents = names[index + 1..]
                    .iter()
                    .filter(|name| matches!(name.as_str(), "adjunct" | "modifier"))
                    .count();
                key.right_association_words = signed(child.words)
                    .saturating_mul(i64::try_from(dependents).unwrap_or(i64::MAX));
            }
            name @ ("adjunct" | "modifier" | "supplement") if after_head() => {
                if let Some(head_words) = head() {
                    let host = signed(head_words);
                    let between = signed(before().saturating_sub(head_words));
                    key.right_association_words = if name == "supplement" {
                        -host - between
                    } else if child.high_attachment {
                        // The head added the host for it, unless the host is
                        // a coordination; this does not depend on which.
                        -2 * host - between
                    } else {
                        between
                    };
                }
            }
            "last" => {
                if let Some((first_words, first_head_final)) = first() {
                    key.right_association_words = (signed(first_words) - signed(child.words)).abs();
                    key.attachment_class = u64::from(first_head_final && !child.head_final);
                }
            }
            _ => {}
        }
        let dependent = |name: &Ident| matches!(name.as_str(), "adjunct" | "modifier");
        if dependent(&names[index])
            && names[..index].iter().any(dependent)
            && names[index + 1..]
                .iter()
                .any(|name| name.as_str() == "head")
        {
            key.attachment_class += 1;
        }
        key
    }

    /// One occurrence of a Production's own contribution to a Ranking Key: its
    /// Production Cost. Right Association is added per child
    /// ([`CompiledGrammar::association_words`]); the Attachment Class is not
    /// computed yet, and is zero.
    pub(crate) fn production_key(&self, production: usize) -> RankingKey {
        RankingKey {
            production_cost: u64::from(self.productions[production].cost),
            ..RankingKey::default()
        }
    }

    /// The key a leaf adds as the child at right-hand-side position `at`:
    /// one more Production Cost where the Production declares it `scoped`
    /// and the leaf was not admitted through the face's scope, so that
    /// another card's full name ranks after the face's own name spelled
    /// alike.
    pub(crate) fn leaf_key(&self, production: usize, at: usize, scoped: bool) -> RankingKey {
        RankingKey {
            production_cost: u64::from(
                self.productions[production].scoped.contains(&at) && !scoped,
            ),
            ..RankingKey::default()
        }
    }

    /// Whether a leaf was admitted through the scope of the face whose text
    /// is parsed: one of the face's own names (`scope`), or a card-scoped
    /// spelling, which analysis admits only within its card's scope.
    pub(crate) fn scoped(&self, leaf: &LexicalReading, scope: &BTreeSet<LexemeId>) -> bool {
        matches!(leaf, LexicalReading::Word(value)
            if scope.contains(&value.lexeme)
                || self.card_scoped.contains(&(value.lexeme, value.variant)))
    }

    pub(crate) fn feature_map(&self, category: usize, summary: &Summary) -> BTreeMap<Ident, Ident> {
        self.category_features[category]
            .iter()
            .zip(summary.values.iter())
            .filter_map(|(&f, v)| {
                v.get()
                    .map(|v| (self.features[f], self.values[f][usize::from(v)]))
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

    /// Admission as Category `root`.
    pub(crate) fn admit_as(
        &self,
        reading: &Reading,
        lexicon: &Lexicon,
        root: usize,
    ) -> Result<(), AdmissionError> {
        let (category, summary) = self.summarize(reading, lexicon)?;
        if category != root || summary.gap != Gap::None {
            return Err(AdmissionError::Root(self.categories[category].to_string()));
        }
        let mut atoms = Vec::new();
        self.atoms(reading, lexicon, &mut atoms, &mut |_, _| {})?;
        let (text, offsets) = join(&atoms);
        let atomic = self.atomic_spans(&lexicon.analyze(&text));
        if atomic.is_empty() {
            return Ok(());
        }
        let mut chars = vec![0; text.len() + 1];
        for (count, (at, c)) in text.char_indices().enumerate() {
            chars[at] = count;
            chars[at + c.len_utf8()] = count + 1;
        }
        let atoms: Vec<Range<usize>> = offsets
            .iter()
            .map(|offset| chars[offset.start]..chars[offset.end])
            .collect();
        let mut next = 0;
        if self
            .atomic_reading(reading, &atoms, &atomic, &mut next)?
            .is_none()
        {
            return Err(AdmissionError::Rejected {
                production: "lexical atomicity".into(),
                reason: "a declared multiword noun's words rebuild it".into(),
            });
        }
        Ok(())
    }

    /// Lexical atomicity, decided on the Reading's own structure as the chart
    /// decides it (`docs/decisions/english-lexical-analysis.md` §2): a word
    /// inside a multiword noun of `atomic` stands alone only where the term's
    /// words may stand apart, and then only where they straddle a constituent
    /// boundary. In the smallest Constituent spanning the term, the child
    /// holding its last word must end after it and no child may begin where
    /// it begins. Returns the subtree's character span, or `None` where it
    /// rebuilds a term. `atoms` holds each atom's character span, in the
    /// order [`CompiledGrammar::atoms`] realizes them.
    fn atomic_reading(
        &self,
        reading: &Reading,
        atoms: &[Range<usize>],
        atomic: &[(Range<usize>, bool)],
        next: &mut usize,
    ) -> Result<Option<Range<usize>>, AdmissionError> {
        let inside = |at: usize, span: &Range<usize>| span.start < at && at < span.end;
        let node = match reading {
            Reading::Leaf(value) => {
                let at = atoms[*next].clone();
                *next += 1;
                let word = matches!(value, LexicalReading::Word(_));
                let parted = atomic.iter().any(|(span, splits)| {
                    (inside(at.start, span) || inside(at.end, span)) && !(*splits && word)
                });
                return Ok((!parted).then_some(at));
            }
            Reading::Node(node) => node,
        };
        let &index = self
            .by_id
            .get(&node.production)
            .ok_or_else(|| AdmissionError::UnknownProduction(node.production.to_string()))?;
        let mut children = node.children.iter();
        let mut spans = Vec::new();
        for symbol in &self.productions[index].rhs {
            let span = if let Symbol::Punct(..) = symbol {
                *next += 1;
                atoms[*next - 1].clone()
            } else {
                let child = children.next().ok_or_else(|| AdmissionError::Rejected {
                    production: node.production.to_string(),
                    reason: "missing child".to_owned(),
                })?;
                match self.atomic_reading(child, atoms, atomic, next)? {
                    Some(span) => span,
                    None => return Ok(None),
                }
            };
            spans.push(span);
        }
        let (Some(first), Some(last)) = (spans.first(), spans.last()) else {
            return Ok(Some(0..0));
        };
        let whole = first.start..last.end;
        let rebuilt = atomic
            .iter()
            .filter(|(term, splits)| *splits && whole.start <= term.start)
            .any(|(term, _)| {
                spans.iter().enumerate().any(|(i, child)| {
                    inside(child.start, term)
                        && term.end <= child.end
                        && (child.end == term.end
                            || spans[..i].iter().any(|before| before.start == term.start))
                })
            });
        Ok((!rebuilt).then_some(whole))
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
            .ok_or_else(|| AdmissionError::UnknownProduction(node.production.to_string()))?;
        let production = &self.productions[index];
        let reject = |reason: &str| AdmissionError::Rejected {
            production: node.production.to_string(),
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
        self.locate(reading, None, None, &mut out);
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
        function: Option<Ident>,
        out: &mut Vec<Constituent>,
    ) {
        let Reading::Node(node) = reading else {
            return;
        };
        let index = out.len();
        out.push(Constituent {
            category: node.category,
            span: 0..0,
            function,
            parent,
            head: self.head_lexeme(reading),
            gap: node.gap,
            also: Vec::new(),
        });
        // The head of a function Category bears its function in its phrase.
        let wraps = self.function_categories.contains(&node.category);
        let names = self.child_names(node);
        let raising = self
            .by_id
            .get(&node.production)
            .is_some_and(|&p| self.productions[p].requires_coordination.is_some());
        let mut sites = Vec::new();
        let mut object = None;
        for (child, name) in node.children.iter().zip(names) {
            let start = out.len();
            if name == "object" {
                object = Some(start);
            }
            // A Modifier after its phrase's Object is postposed out of it
            // (*put a number of counters on it [equal to …]*, *choose a card
            // at random [that …]*; CGEL, Ch. 12 §4, p. 1066): it modifies
            // the Object too.
            let postposed = object.filter(|_| name == "modifier");
            if raising && name == "head" {
                let mut at = start;
                let gap = child.node().map_or(Gap::None, |c| c.gap);
                Self::gap_sites(child, gap, &mut at, &mut sites);
            }
            if wraps && name == "head" {
                self.locate(child, parent, function, out);
            } else {
                self.locate(child, Some(index), Some(name), out);
            }
            if raising && name == "shared" {
                // A function Category's head bears the shared function too
                // (*be and become [legendary]*).
                let (parent, function) = (out[start].parent, out[start].function);
                for constituent in &mut out[start..] {
                    if constituent.parent == parent && constituent.function == function {
                        constituent.also.clone_from(&sites);
                    }
                }
            }
            if let Some(object) = postposed {
                // A function Category's head bears it too.
                for constituent in &mut out[start..] {
                    if constituent.parent == Some(index) && constituent.function == Some(name) {
                        constituent.also.push((name, object));
                    }
                }
            }
        }
    }

    /// The phrases along the `gap` path of `reading` that introduce the
    /// Gap, with the function of the extracted Frame Slot, by preorder
    /// index from `at`.
    fn gap_sites(reading: &Reading, gap: Gap, at: &mut usize, sites: &mut Vec<(Ident, usize)>) {
        let Reading::Node(node) = reading else { return };
        let index = *at;
        *at += 1;
        let on_path = gap != Gap::None && node.gap == gap;
        let extracted = node
            .production
            .rsplit_once("/gap")
            .and_then(|(_, slot)| slot.split('/').next()?.parse::<usize>().ok());
        if let (true, Some(slot)) = (on_path, extracted) {
            // A preposition's phrase records no Selected Frame: its NP is its
            // Complement.
            let relation = node
                .selected_frame
                .as_ref()
                .and_then(|frame| frame.slots.get(slot))
                .map_or(crate::frames::SlotRelation::Complement, |s| s.relation);
            sites.push((Ident::new(crate::compile::relation_name(relation)), index));
            Self::count(node, at);
            return;
        }
        for child in &node.children {
            if on_path && child.node().is_some_and(|c| c.gap == gap) {
                Self::gap_sites(child, gap, at, sites);
            } else {
                Self::count_reading(child, at);
            }
        }
    }

    fn count(node: &ReadingNode, at: &mut usize) {
        for child in &node.children {
            Self::count_reading(child, at);
        }
    }

    fn count_reading(reading: &Reading, at: &mut usize) {
        if let Reading::Node(node) = reading {
            *at += 1;
            Self::count(node, at);
        }
    }

    /// The names of a node's children, in order: the grammatical function
    /// each bears in it.
    pub(crate) fn child_names(&self, node: &ReadingNode) -> Vec<Ident> {
        self.by_id
            .get(&node.production)
            .map_or_else(Vec::new, |&p| {
                let production = &self.productions[p];
                production
                    .rhs
                    .iter()
                    .zip(&production.names)
                    .filter(|(symbol, _)| !matches!(symbol, Symbol::Punct(..)))
                    .map(|(_, name)| *name)
                    .collect()
            })
    }

    /// The lexeme reached by following `head` children down from `reading`.
    #[must_use]
    pub fn head_lexeme(&self, reading: &Reading) -> Option<LexemeId> {
        self.head_word(reading).map(|value| value.lexeme)
    }

    /// The word reached by following `head` children down from `reading`:
    /// its lexeme, Word Form and features. Two Readings that differ only in
    /// the form of a syncretic head (*cast* as present or preterite) differ
    /// here.
    #[must_use]
    pub fn head_word<'r>(&self, reading: &'r Reading) -> Option<&'r LexicalValue> {
        match reading {
            Reading::Leaf(LexicalReading::Word(value)) => Some(value),
            Reading::Leaf(_) => None,
            Reading::Node(node) => {
                let names = self.child_names(node);
                let at = names.iter().position(|n| n == "head")?;
                self.head_word(node.children.get(at)?)
            }
        }
    }

    /// Pushes a Reading's atoms (text, binding, punctuation) in order, and
    /// reports each node's preorder index with its atom range.
    pub(crate) fn atoms(
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
                let &index = self.by_id.get(&node.production).ok_or_else(|| {
                    AdmissionError::UnknownProduction(node.production.to_string())
                })?;
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
                            production: node.production.to_string(),
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
    pub category: Ident,
    /// Byte range in the realized text.
    pub span: Range<usize>,
    /// Its name in its mother's Production: the grammatical function it
    /// bears there (`subject`, `object`, `modifier`, ...); none for the
    /// root.
    pub function: Option<Ident>,
    /// Preorder index of its mother; the head of a function Category
    /// (`CategoryDecl::function`) takes that Category's mother and function.
    pub parent: Option<usize>,
    /// The lexeme reached through its `head` children.
    pub head: Option<LexemeId>,
    pub gap: Gap,
    /// The other functions it bears, each with the preorder index of the
    /// phrase it bears it in: a Noun Phrase shared by Right Node Raising
    /// fills each conjunct's Gap (*draw and discard [cards]*), and a
    /// Modifier postposed out of an Object modifies that Object.
    pub also: Vec<(Ident, usize)>,
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
