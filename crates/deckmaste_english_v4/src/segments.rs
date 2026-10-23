//! The census's Segments: a parse's top-level stretches and their Derivations,
//! found by walking its Packed Forest.

use std::collections::BTreeMap;
use std::collections::BTreeSet;
use std::ops::Range;

use deckmaste_syntax::ChildRef;
use deckmaste_syntax::CompiledGrammar;
use deckmaste_syntax::LoadError;
use deckmaste_syntax::Parse;
use deckmaste_syntax::walk::ForestMemo;
use deckmaste_syntax::walk::Visit;
use serde::Serialize;

/// The Oracle English outer structure, resolved in a bound grammar.
///
/// # Errors
/// An outer Category the grammar does not declare.
pub fn segment_frame(grammar: &CompiledGrammar) -> Result<SegmentFrame, LoadError> {
    SegmentFrame::resolve(grammar, crate::OUTER_CATEGORIES, crate::SEGMENT_EDGES)
}

/// A grammar's outer structure, which delimits a parse's Segments: the
/// Categories a Segment walk descends through, and the structural
/// punctuation it ignores at a Segment's edges.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SegmentFrame {
    outer: BTreeSet<usize>,
    edges: Vec<char>,
}

impl SegmentFrame {
    /// Resolves an outer structure by Category name.
    ///
    /// # Errors
    /// A name that is not a Category of this grammar.
    pub fn resolve(
        grammar: &CompiledGrammar,
        outer: &[&str],
        edges: &[char],
    ) -> Result<Self, LoadError> {
        let outer = outer
            .iter()
            .map(|name| {
                grammar.category_id(name).ok_or_else(|| {
                    LoadError::Invalid(format!("unknown outer category `{name}`"))
                })
            })
            .collect::<Result<_, _>>()?;
        Ok(Self {
            outer,
            edges: edges.to_vec(),
        })
    }
}

/// A top-level stretch of a parse and its Derivations.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct Segment {
    /// Its extent in the analyzed text, in chars, without structural
    /// punctuation or spaces at its edges.
    pub span: Range<usize>,
    pub text: String,
    /// The Derivations of everything the walk reached at this extent.
    pub derivations: u128,
}

/// A parse's Segments, or why it has none.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub enum Segmentation {
    /// One segmentation whose counts multiply to the parse's Derivations;
    /// empty when there is no Derivation.
    Segments(Vec<Segment>),
    NotSegmentable(NotSegmentable),
}

/// Why a parse has no Segments.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub enum NotSegmentable {
    /// Its Derivations divide the text into Segments in more than one way,
    /// or in more ways than the walk keeps.
    Segmentations,
    /// One segmentation, under more than one outer structure: the Segments'
    /// counts do not multiply to the Derivations.
    NonProduct,
}

impl Segmentation {
    /// Σ(nᵢ − 1) over the Segments' Derivations; for a parse with no
    /// Segments, its Derivations less one.
    #[must_use]
    pub fn surplus(&self, derivations: u128) -> u128 {
        match self {
            Self::Segments(segments) if !segments.is_empty() => segments
                .iter()
                .map(|s| s.derivations.saturating_sub(1))
                .fold(0u128, u128::saturating_add),
            _ => derivations.saturating_sub(1),
        }
    }
}

/// The most segmentations a node or item keeps before the walk gives up.
const SEGMENTATION_BOUND: usize = 64;

/// Raw extents, in order, of one way to divide a span into Segments.
type Segmentations = BTreeSet<Vec<(usize, usize)>>;

/// What a Segment walk reached below the outer structure.
#[derive(Default)]
struct Reached {
    nodes: BTreeSet<usize>,
    leaves: BTreeSet<usize>,
    overflow: bool,
}

/// The parse's Segments: the walk descends from the roots through the
/// frame's outer Categories, and the first Constituent on each path of
/// another Category, or a leaf of an outer Production, is a Segment.
/// Segments are identified by extent alone, so two Categories over one
/// extent are Ambiguity inside one Segment. A sentence sequence nested
/// in a Segment stays in it and multiplies into its count.
#[must_use]
pub fn segments(parse: &Parse<'_>, frame: &SegmentFrame) -> Segmentation {
    let derivations = parse.derivation_count();
    if derivations == 0 {
        return Segmentation::Segments(Vec::new());
    }
    let walk = Walk { parse, frame };
    let forest = &parse.forest;
    let mut reached = Reached::default();
    let mut memo: ForestMemo<Segmentations, Segmentations> = ForestMemo::default();
    let mut all = Segmentations::new();
    for &root in forest.roots() {
        all.extend(walk.segment_node(root, &mut reached, &mut memo));
    }
    let trimmed: BTreeSet<Vec<(usize, usize)>> = all
        .iter()
        .map(|raw| {
            raw.iter()
                .map(|&(start, end)| walk.trim(start, end))
                .filter(|&(start, end)| start < end)
                .collect()
        })
        .collect();
    let mut trimmed = trimmed.into_iter();
    let (Some(only), None, false) = (trimmed.next(), trimmed.next(), reached.overflow) else {
        return Segmentation::NotSegmentable(NotSegmentable::Segmentations);
    };
    let mut counts: BTreeMap<(usize, usize), u128> = BTreeMap::new();
    let mut count_memo = ForestMemo::default();
    for &node in &reached.nodes {
        let span = forest.node_span(node);
        let count = parse.count_node(node, &mut count_memo);
        let entry = counts.entry(walk.trim(span.start, span.end)).or_default();
        *entry = entry.saturating_add(count);
    }
    for &leaf in &reached.leaves {
        let span = forest.leaf_span(leaf);
        let entry = counts.entry(walk.trim(span.start, span.end)).or_default();
        *entry = entry.saturating_add(1);
    }
    let segments: Vec<Segment> = only
        .into_iter()
        .map(|(start, end)| Segment {
            span: start..end,
            text: parse.tokens()[start..end].iter().collect(),
            derivations: counts.get(&(start, end)).copied().unwrap_or(0),
        })
        .collect();
    let product = segments
        .iter()
        .map(|s| s.derivations)
        .fold(1u128, u128::saturating_mul);
    if product == derivations {
        Segmentation::Segments(segments)
    } else {
        Segmentation::NotSegmentable(NotSegmentable::NonProduct)
    }
}

struct Walk<'a, 'g> {
    parse: &'a Parse<'g>,
    frame: &'a SegmentFrame,
}

impl Walk<'_, '_> {
    /// An extent without the frame's edge punctuation and spaces.
    fn trim(&self, mut start: usize, mut end: usize) -> (usize, usize) {
        let tokens = self.parse.tokens();
        let edge = |c: char| c.is_whitespace() || self.frame.edges.contains(&c);
        while start < end && edge(tokens[start]) {
            start += 1;
        }
        while end > start && edge(tokens[end - 1]) {
            end -= 1;
        }
        (start, end)
    }

    fn segment_node(
        &self,
        node: usize,
        reached: &mut Reached,
        memo: &mut ForestMemo<Segmentations, Segmentations>,
    ) -> Segmentations {
        let forest = &self.parse.forest;
        if !self.frame.outer.contains(&forest.node_category(node)) {
            reached.nodes.insert(node);
            let span = forest.node_span(node);
            return Segmentations::from([vec![(span.start, span.end)]]);
        }
        let open = match memo.nodes.enter(node) {
            Visit::Known(segmentations) => return segmentations,
            Visit::Cycle => return Segmentations::new(),
            Visit::Open(open) => open,
        };
        let mut out = Segmentations::new();
        for item in forest.node_items(node) {
            out.extend(self.segment_item(item, reached, memo));
        }
        bound(&mut out, reached);
        memo.nodes.leave(open, &out);
        out
    }

    fn segment_item(
        &self,
        item: usize,
        reached: &mut Reached,
        memo: &mut ForestMemo<Segmentations, Segmentations>,
    ) -> Segmentations {
        if let Some(known) = memo.items.get(&item) {
            return known.clone();
        }
        let forest = &self.parse.forest;
        let mark = memo.nodes.mark();
        let mut out = Segmentations::new();
        for (previous, child) in forest.item_families(item) {
            let before = match previous {
                None => Segmentations::from([Vec::new()]),
                Some(p) => self.segment_item(p, reached, memo),
            };
            let here = match child {
                ChildRef::Node(n) => self.segment_node(n, reached, memo),
                ChildRef::Leaf(l) => {
                    reached.leaves.insert(l);
                    let span = forest.leaf_span(l);
                    Segmentations::from([vec![(span.start, span.end)]])
                }
                ChildRef::Punct => Segmentations::from([Vec::new()]),
            };
            for b in &before {
                for h in &here {
                    let mut joined = b.clone();
                    joined.extend(h.iter().copied());
                    out.insert(joined);
                }
            }
            bound(&mut out, reached);
        }
        if memo.nodes.settle(mark) {
            memo.items.insert(item, out.clone());
        }
        out
    }
}

/// Keeps a walk's segmentations within [`SEGMENTATION_BOUND`].
fn bound(segmentations: &mut Segmentations, reached: &mut Reached) {
    while segmentations.len() > SEGMENTATION_BOUND {
        reached.overflow = true;
        segmentations.pop_last();
    }
}
