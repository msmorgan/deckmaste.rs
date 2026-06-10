//! Recursive expansion-stripping: [`Expand::expand_all`] rebuilds a value
//! tree with every `Expanded(Expansion { value, .. })` node replaced by its
//! stored `value` — provenance (name + args) discarded, recursion covers
//! macro bodies that invoke macros. Implemented by `#[derive(SupportsMacros)]`
//! for macro enums and `#[derive(Expand)]` for plain grammar types; this
//! module supplies containers and leaves.

use std::collections::{BTreeMap, HashMap};
use std::hash::{BuildHasher, Hash};

use crate::Ident;

/// Recursively replaces every `Expanded` node with its stored value.
pub trait Expand: Sized {
    #[must_use]
    fn expand_all(self) -> Self;
}

macro_rules! expand_leaf {
    ($($t:ty),* $(,)?) => {
        $(impl Expand for $t {
            fn expand_all(self) -> Self { self }
        })*
    };
}

expand_leaf!(
    u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize, f32, f64, bool, char, String,
    Ident,
);

impl<T: Expand> Expand for Box<T> {
    fn expand_all(self) -> Self { Box::new((*self).expand_all()) }
}

impl<T: Expand> Expand for Option<T> {
    fn expand_all(self) -> Self { self.map(Expand::expand_all) }
}

impl<T: Expand> Expand for Vec<T> {
    fn expand_all(self) -> Self { self.into_iter().map(Expand::expand_all).collect() }
}

impl<K: Eq + Hash, V: Expand, S: BuildHasher + Default> Expand for HashMap<K, V, S> {
    fn expand_all(self) -> Self { self.into_iter().map(|(k, v)| (k, v.expand_all())).collect() }
}

impl<K: Ord, V: Expand> Expand for BTreeMap<K, V> {
    fn expand_all(self) -> Self { self.into_iter().map(|(k, v)| (k, v.expand_all())).collect() }
}

impl<A: Expand, B: Expand> Expand for (A, B) {
    fn expand_all(self) -> Self { (self.0.expand_all(), self.1.expand_all()) }
}

impl<A: Expand, B: Expand, C: Expand> Expand for (A, B, C) {
    fn expand_all(self) -> Self {
        (
            self.0.expand_all(),
            self.1.expand_all(),
            self.2.expand_all(),
        )
    }
}
