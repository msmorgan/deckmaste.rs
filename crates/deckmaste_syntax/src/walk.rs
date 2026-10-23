//! Memoized depth-first walks over a Packed Forest that may contain unary
//! cycles.
//!
//! A Derivation never contains its own Constituent, so a node met again on
//! the current path contributes nothing there. What a node below a cycle
//! contributes therefore depends on the path that reached it: a result is
//! memoized only when the walk below it met no cycle, so a node first
//! reached inside a cycle still shows every finite Derivation when reached
//! from elsewhere. Acyclic forests are memoized throughout.

use std::collections::HashMap;
use std::collections::HashSet;

/// Results by forest node, and the current path of open nodes.
pub struct NodeMemo<T> {
    done: HashMap<usize, T>,
    path: HashSet<usize>,
    /// Whether the walk met a cycle since the innermost open node or item
    /// began.
    cyclic: bool,
}

impl<T> Default for NodeMemo<T> {
    fn default() -> Self {
        Self {
            done: HashMap::new(),
            path: HashSet::new(),
            cyclic: false,
        }
    }
}

/// What a walk does with a node it reaches.
pub enum Visit<T> {
    /// Its result is known.
    Known(T),
    /// It is on the current path: no Derivation reaches it again here.
    Cycle,
    /// Compute it, then pass this to [`NodeMemo::leave`].
    Open(Frame),
}

#[derive(Clone, Copy)]
pub struct Frame {
    node: usize,
    outer: bool,
}

impl<T: Clone> NodeMemo<T> {
    pub fn enter(&mut self, node: usize) -> Visit<T> {
        if let Some(value) = self.done.get(&node) {
            return Visit::Known(value.clone());
        }
        if !self.path.insert(node) {
            self.cyclic = true;
            return Visit::Cycle;
        }
        Visit::Open(Frame {
            node,
            outer: self.mark(),
        })
    }

    /// Closes an open node with its result, memoized if no cycle lies below it.
    pub fn leave(&mut self, frame: Frame, value: &T) {
        self.path.remove(&frame.node);
        if self.settle(frame.outer) {
            self.done.insert(frame.node, value.clone());
        }
    }

    /// Starts a result that is not a node's (an item's), for
    /// [`NodeMemo::settle`].
    pub fn mark(&mut self) -> bool {
        std::mem::replace(&mut self.cyclic, false)
    }

    /// Ends a result begun by [`NodeMemo::mark`]: whether no cycle lies below
    /// it, so that it may be memoized.
    pub fn settle(&mut self, mark: bool) -> bool {
        let clean = !self.cyclic;
        self.cyclic |= mark;
        clean
    }
}

/// Results of forest nodes and of items. Items share prefixes across
/// Derivations, so each item's result is built once; like a node's, it is
/// kept only when no cycle lies below it ([`NodeMemo::mark`] and
/// [`NodeMemo::settle`]).
pub struct ForestMemo<N, I> {
    pub nodes: NodeMemo<N>,
    pub items: HashMap<usize, I>,
}

impl<N, I> Default for ForestMemo<N, I> {
    fn default() -> Self {
        Self {
            nodes: NodeMemo::default(),
            items: HashMap::new(),
        }
    }
}
