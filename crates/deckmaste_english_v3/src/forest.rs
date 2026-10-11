use std::collections::BTreeMap;
use std::collections::BTreeSet;
use std::ops::Range;

use deckmaste_lexical::LexicalMatch;
use deckmaste_lexical::Source;

use crate::Grammar;
use crate::Materializer;
use crate::Readings;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Ord, PartialOrd)]
pub struct CompletedId(pub(crate) usize);

/// A constituent span and the packed completed nodes contributing its Readings.
/// Coordinates are half-open Unicode-scalar offsets, as in the chart.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Segment {
    pub span: Range<usize>,
    pub nodes: BTreeSet<CompletedId>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, thiserror::Error)]
pub enum SegmentationError {
    #[error("root derivations disagree on constituent spans")]
    Ambiguous,
    #[error("cyclic derivation outside the requested segment Category")]
    Cycle,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Ord, PartialOrd)]
pub(crate) struct IntermediateId(pub usize);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Ord, PartialOrd)]
pub(crate) enum Child {
    Completed(CompletedId),
    Leaf(usize),
}

/// Surface evidence belongs to leaves. Composite nodes carry only chart
/// coordinates and grammatical summaries, not source slices for rendering.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Leaf {
    Lexical {
        occurrence: LexicalMatch,
        provenance: Option<Source>,
    },
    Literal {
        start: usize,
        end: usize,
        text: String,
    },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Ord, PartialOrd)]
pub(crate) enum Edge {
    Empty,
    Append {
        prefix: IntermediateId,
        child: Child,
    },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Ord, PartialOrd)]
pub(crate) struct Family {
    pub production: usize,
    pub prefix: IntermediateId,
}

pub(crate) struct Intermediate<T> {
    pub state: T,
    pub edges: Vec<Edge>,
    pub edge_set: BTreeSet<Edge>,
}

pub(crate) struct Completed<C, S> {
    pub category: C,
    pub start: usize,
    pub end: usize,
    pub summary: S,
    pub families: Vec<Family>,
    pub family_set: BTreeSet<Family>,
}

/// Recognition counters. `families` includes binary intermediate edges and
/// completed families; `completion_work` counts waiter/completed-node pairs.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct Metrics {
    pub lexical_alternatives: usize,
    pub lexical_projections: usize,
    pub items: usize,
    /// Distinct dot-0 items inserted by prediction, included in `items`.
    pub predicted_items: usize,
    pub intermediate_nodes: usize,
    pub completed_nodes: usize,
    pub intermediate_edges: usize,
    pub completed_families: usize,
    pub families: usize,
    pub completion_work: usize,
    pub scan_work: usize,
    /// Furthest reached boundary in the input's Unicode-scalar token sequence.
    /// Predicted items alone do not advance this position.
    pub furthest_position: usize,
}

/// Packed recognizer output. Growth of a shared child's families is visible
/// through existing edges and does not schedule its parents again.
pub struct Forest<C, S, T> {
    pub(crate) intermediate: Vec<Intermediate<T>>,
    pub(crate) completed: Vec<Completed<C, S>>,
    pub(crate) leaves: Vec<Leaf>,
    pub(crate) leaf_summaries: Vec<Option<S>>,
    pub(crate) roots: Vec<CompletedId>,
    pub(crate) arities: Vec<usize>,
    pub(crate) metrics: Metrics,
}

impl<C, S, T> Forest<C, S, T> {
    #[must_use]
    pub const fn metrics(&self) -> Metrics {
        self.metrics
    }

    #[must_use]
    pub fn roots(&self) -> &[CompletedId] {
        &self.roots
    }

    /// A root's grammatical summary, independent of its surface coordinates.
    #[must_use]
    pub fn summary(&self, id: CompletedId) -> Option<&S> {
        self.completed.get(id.0).map(|node| &node.summary)
    }

    /// Request Readings without enumerating the forest first. Equality of the
    /// materializer's value identifies duplicate derivations, not admission.
    pub fn readings<M: Materializer<S, T>>(&self, materializer: M) -> Readings<'_, C, S, T, M> {
        Readings::new(self, materializer)
    }

    /// Enumerate the union of Readings of completed nodes from this forest.
    /// Nodes obtained from a segment share its Category and span. Nested
    /// constituents still materialize normally; no parent combinations are
    /// built.
    pub fn readings_from<M: Materializer<S, T>>(
        &self,
        nodes: &[CompletedId],
        materializer: M,
    ) -> Readings<'_, C, S, T, M> {
        Readings::from_nodes(self, nodes, materializer)
    }
}

impl<C: PartialEq, S, T> Forest<C, S, T> {
    /// Walk packed root derivations, stopping at the first constituent of the
    /// requested Category on each path. Its descendants are not segmented.
    /// Alternative summaries at the same span contribute to the same segment.
    /// No roots, or a root with no such constituents, yields no segments.
    ///
    /// # Errors
    /// Reports disagreeing segment spans or a cycle outside a segment. It
    /// never enumerates the Cartesian product of constituent Readings.
    pub fn segments(&self, category: &C) -> Result<Vec<Segment>, SegmentationError> {
        let mut walk = SegmentWalk {
            forest: self,
            category,
            completed: BTreeMap::new(),
            intermediate: BTreeMap::new(),
            active: BTreeSet::new(),
        };
        let mut result = None;
        for &root in &self.roots {
            merge_segments(&mut result, walk.completed(root)?)?;
        }
        Ok(result.unwrap_or_default())
    }
}

/// Alternatives must have identical ordered spans. Union only the completed
/// nodes at corresponding spans, retaining every grammatical summary.
fn merge_segments(
    result: &mut Option<Vec<Segment>>,
    alternative: Vec<Segment>,
) -> Result<(), SegmentationError> {
    let Some(current) = result else {
        *result = Some(alternative);
        return Ok(());
    };
    if current.len() != alternative.len()
        || current
            .iter()
            .zip(&alternative)
            .any(|(a, b)| a.span != b.span)
    {
        return Err(SegmentationError::Ambiguous);
    }
    for (current, alternative) in current.iter_mut().zip(alternative) {
        current.nodes.extend(alternative.nodes);
    }
    Ok(())
}

struct SegmentWalk<'a, C, S, T> {
    forest: &'a Forest<C, S, T>,
    category: &'a C,
    completed: BTreeMap<CompletedId, Vec<Segment>>,
    intermediate: BTreeMap<IntermediateId, Vec<Segment>>,
    active: BTreeSet<CompletedId>,
}

impl<C: PartialEq, S, T> SegmentWalk<'_, C, S, T> {
    fn completed(&mut self, id: CompletedId) -> Result<Vec<Segment>, SegmentationError> {
        if let Some(segments) = self.completed.get(&id) {
            return Ok(segments.clone());
        }
        let node = &self.forest.completed[id.0];
        if &node.category == self.category {
            return Ok(vec![Segment {
                span: node.start..node.end,
                nodes: BTreeSet::from([id]),
            }]);
        }
        if !self.active.insert(id) {
            return Err(SegmentationError::Cycle);
        }
        let mut result = None;
        for family in &node.families {
            merge_segments(&mut result, self.intermediate(family.prefix)?)?;
        }
        self.active.remove(&id);
        let result = result.unwrap_or_default();
        self.completed.insert(id, result.clone());
        Ok(result)
    }

    fn intermediate(&mut self, id: IntermediateId) -> Result<Vec<Segment>, SegmentationError> {
        if let Some(segments) = self.intermediate.get(&id) {
            return Ok(segments.clone());
        }
        let mut result = None;
        for edge in &self.forest.intermediate[id.0].edges {
            let mut alternative = Vec::new();
            if let Edge::Append { prefix, child } = *edge {
                alternative = self.intermediate(prefix)?;
                if let Child::Completed(child) = child {
                    alternative.extend(self.completed(child)?);
                }
            }
            merge_segments(&mut result, alternative)?;
        }
        let result = result.unwrap_or_default();
        self.intermediate.insert(id, result.clone());
        Ok(result)
    }
}

impl<C: Clone + Ord, S: Clone + Ord, T> Forest<C, S, T> {
    pub(crate) fn new<G: Grammar<Category = C, Summary = S>>(grammar: &G) -> Self {
        Self {
            intermediate: Vec::new(),
            completed: Vec::new(),
            leaves: Vec::new(),
            leaf_summaries: Vec::new(),
            roots: Vec::new(),
            arities: grammar
                .productions()
                .iter()
                .map(|p| p.symbols.len())
                .collect(),
            metrics: Metrics::default(),
        }
    }
}

impl<C: std::fmt::Debug, S: std::fmt::Debug, T: std::fmt::Debug> Forest<C, S, T> {
    /// Write the shared nodes and their references without expanding paths or
    /// materializing Readings. Work is proportional to the packed
    /// representation.
    ///
    /// # Errors
    /// Forwards errors from the output writer.
    pub fn write_packed(&self, output: &mut impl std::io::Write) -> std::io::Result<()> {
        writeln!(output, "roots: {:?}", self.roots)?;
        for (id, node) in self.completed.iter().enumerate() {
            writeln!(
                output,
                "completed {id}: {:?} {}..{} {:?} {:?}",
                node.category, node.start, node.end, node.summary, node.families
            )?;
        }
        for (id, node) in self.intermediate.iter().enumerate() {
            writeln!(
                output,
                "intermediate {id}: {:?} {:?}",
                node.state, node.edges
            )?;
        }
        for (id, (leaf, summary)) in self.leaves.iter().zip(&self.leaf_summaries).enumerate() {
            writeln!(output, "leaf {id}: {leaf:?} {summary:?}")?;
        }
        Ok(())
    }
}
