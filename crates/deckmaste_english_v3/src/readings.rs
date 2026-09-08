use std::collections::BTreeSet;

use crate::CompletedId;
use crate::Forest;
use crate::Leaf;
use crate::forest::Child;
use crate::forest::Edge;
use crate::forest::IntermediateId;

/// Generated structural values define Reading equality: lexical identities,
/// constituent structure, grammatical features and retained spelling variants.
/// Exclude source coordinates and derivation/production identity when two
/// productions construct the same grammatical value. Never equate by rendered
/// text alone. Builders must realize already-admitted paths, not perform a
/// deferred grammatical validity check.
pub trait Materializer<S, T> {
    type Reading: Clone + Ord;
    type Error;

    /// # Errors
    /// Reports an internal failure to construct an already-admitted leaf.
    fn leaf(&mut self, leaf: &Leaf, summary: Option<&S>) -> Result<Self::Reading, Self::Error>;

    /// Children are in production order, including declared literal leaves.
    /// # Errors
    /// Reports an internal failure to construct an already-admitted Reading.
    fn build(
        &mut self,
        production: usize,
        summary: &S,
        state: &T,
        children: Vec<Self::Reading>,
    ) -> Result<Self::Reading, Self::Error>;
}

#[derive(Debug, PartialEq, Eq, thiserror::Error)]
pub enum MaterializeError<E> {
    #[error("cyclic derivation through a shared completed node")]
    Cycle,
    #[error("materialization of an admitted derivation failed: {0}")]
    Build(E),
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct ReadingMetrics {
    pub requests: usize,
    pub derivations: usize,
    pub readings: usize,
    pub duplicates: usize,
    pub cyclic_derivations: usize,
    pub internal_failures: usize,
    pub builds: usize,
}

#[derive(Clone, Copy)]
enum Task {
    Completed(CompletedId),
    Intermediate(IntermediateId),
    Leaf(usize),
    Build(CompletedId, usize),
    Leave(CompletedId),
    Family(CompletedId, usize),
    Edge(IntermediateId, usize),
}

#[derive(Clone)]
struct Work<V> {
    tasks: Vec<Task>,
    values: Vec<V>,
    active: BTreeSet<CompletedId>,
}

/// Demand-driven depth-first traversal, with explicit continuations for sibling
/// alternatives. A cycle reports an error for that path; subsequent requests
/// still visit finite siblings. It does not silently truncate a cyclic grammar
/// to a successful finite census. Work and deduplication grow with requested
/// derivations, which can be exponential in packed-forest size.
pub struct Readings<'a, C, S, T, M: Materializer<S, T>> {
    forest: &'a Forest<C, S, T>,
    materializer: M,
    pending: Vec<Work<M::Reading>>,
    seen: BTreeSet<M::Reading>,
    metrics: ReadingMetrics,
}

impl<'a, C, S, T, M: Materializer<S, T>> Readings<'a, C, S, T, M> {
    pub(crate) fn new(forest: &'a Forest<C, S, T>, materializer: M) -> Self {
        let pending = forest
            .roots
            .iter()
            .rev()
            .map(|&id| Work {
                tasks: vec![Task::Completed(id)],
                values: Vec::new(),
                active: BTreeSet::new(),
            })
            .collect();
        Self {
            forest,
            materializer,
            pending,
            seen: BTreeSet::new(),
            metrics: ReadingMetrics::default(),
        }
    }

    #[must_use]
    pub const fn metrics(&self) -> ReadingMetrics {
        self.metrics
    }

    fn sibling(&mut self, work: &Work<M::Reading>, task: Task) {
        let mut next = work.clone();
        next.tasks.push(task);
        self.pending.push(next);
    }

    fn step(
        &mut self,
        work: &mut Work<M::Reading>,
        task: Task,
    ) -> Result<(), MaterializeError<M::Error>> {
        match task {
            Task::Completed(id) => {
                if !work.active.insert(id) {
                    self.metrics.cyclic_derivations += 1;
                    return Err(MaterializeError::Cycle);
                }
                work.tasks.push(Task::Leave(id));
                work.tasks.push(Task::Family(id, 0));
            }
            Task::Leave(id) => {
                work.active.remove(&id);
            }
            Task::Family(id, index) => {
                let node = &self.forest.completed[id.0];
                let family = node.families[index];
                if index + 1 < node.families.len() {
                    self.sibling(work, Task::Family(id, index + 1));
                }
                work.tasks.push(Task::Build(id, index));
                work.tasks.push(Task::Intermediate(family.prefix));
            }
            Task::Intermediate(id) => work.tasks.push(Task::Edge(id, 0)),
            Task::Edge(id, index) => {
                let node = &self.forest.intermediate[id.0];
                let edge = node.edges[index];
                if index + 1 < node.edges.len() {
                    self.sibling(work, Task::Edge(id, index + 1));
                }
                if let Edge::Append { prefix, child } = edge {
                    work.tasks.push(match child {
                        Child::Completed(id) => Task::Completed(id),
                        Child::Leaf(id) => Task::Leaf(id),
                    });
                    work.tasks.push(Task::Intermediate(prefix));
                }
            }
            Task::Leaf(id) => {
                let value = self
                    .materializer
                    .leaf(
                        &self.forest.leaves[id],
                        self.forest.leaf_summaries[id].as_ref(),
                    )
                    .map_err(MaterializeError::Build)?;
                work.values.push(value);
            }
            Task::Build(id, index) => {
                let node = &self.forest.completed[id.0];
                let family = node.families[index];
                let production = family.production;
                let children = work
                    .values
                    .split_off(work.values.len() - self.forest.arities[production]);
                self.metrics.builds += 1;
                let value = self
                    .materializer
                    .build(
                        production,
                        &node.summary,
                        &self.forest.intermediate[family.prefix.0].state,
                        children,
                    )
                    .map_err(MaterializeError::Build)?;
                work.values.push(value);
            }
        }
        Ok(())
    }
}

impl<C, S, T, M: Materializer<S, T>> Iterator for Readings<'_, C, S, T, M> {
    type Item = Result<M::Reading, MaterializeError<M::Error>>;

    fn next(&mut self) -> Option<Self::Item> {
        self.metrics.requests += 1;
        while let Some(mut work) = self.pending.pop() {
            while let Some(task) = work.tasks.pop() {
                if let Err(error) = self.step(&mut work, task) {
                    if matches!(&error, MaterializeError::Build(_)) {
                        self.metrics.internal_failures += 1;
                    }
                    return Some(Err(error));
                }
            }
            let value = work
                .values
                .pop()
                .expect("a completed root constructs one value");
            self.metrics.derivations += 1;
            if self.seen.insert(value.clone()) {
                self.metrics.readings += 1;
                return Some(Ok(value));
            }
            self.metrics.duplicates += 1;
        }
        None
    }
}
