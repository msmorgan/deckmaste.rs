use std::collections::BTreeSet;

use deckmaste_lexical::LexicalMatch;
use deckmaste_lexical::Source;

use crate::Grammar;
use crate::Materializer;
use crate::Readings;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Ord, PartialOrd)]
pub struct CompletedId(pub(crate) usize);

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
    pub intermediate_nodes: usize,
    pub completed_nodes: usize,
    pub intermediate_edges: usize,
    pub completed_families: usize,
    pub families: usize,
    pub completion_work: usize,
    pub scan_work: usize,
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
