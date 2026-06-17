//! Recursive tree rewrites over grammar values.
//!
//! Two traits live here, both consuming `self -> Self` and rebuilding the tree
//! without a clone:
//!
//! - [`Expand::expand_all`] rebuilds a value tree with every
//!   `Expanded(Expansion { value, .. })` node replaced by its stored `value` —
//!   provenance (name + args) discarded, recursion covers macro bodies that
//!   invoke macros. Implemented by `#[derive(SupportsMacros)]` for macro enums
//!   and `#[derive(Expand)]` for plain grammar types.
//! - [`Normalize::normalize`] rewrites a value into a simpler,
//!   semantically-equivalent form: redundant structural wrappers collapsed,
//!   algebraic identities applied. NEVER derived — every impl is hand-written.
//!
//! This module supplies the container/leaf impls for both: leaves are
//! identity, containers recurse into their elements.

use std::collections::BTreeMap;
use std::collections::HashMap;
use std::hash::BuildHasher;
use std::hash::Hash;

use crate::Ident;

/// Recursively replaces every `Expanded` node with its stored value.
pub trait Expand: Sized {
    #[must_use]
    fn expand_all(self) -> Self;
}

/// Rewrites a value into a simpler, semantically-equivalent form: redundant
/// structural wrappers collapsed, algebraic identities applied. Distinct from
/// [`Expand`] (which strips macro-invocation provenance). A fully-canonical
/// value is `x.expand_all().normalize()`.
///
/// Contract: `x.normalize()` MUST be observably equivalent to `x` under the
/// engine's semantics. An impl may apply any rewrite it can DEMONSTRATE
/// preserves meaning; the floor is identity. Consuming `self -> Self` (like
/// [`Expand`]): rebuilds the tree, no clone, keeps only the normalized value.
///
/// Hand-written per type, NEVER derived — normalization rules need human
/// judgment about equivalence, so there is no proc-macro. The blanket
/// container/leaf impls below are helpers a hand impl calls to recurse into its
/// children (normalize children first, then collapse locally), NOT a
/// derivation.
pub trait Normalize: Sized {
    #[must_use]
    fn normalize(self) -> Self;
}

macro_rules! traverse_leaf {
    ($($t:ty),* $(,)?) => {
        $(
            impl Expand for $t {
                fn expand_all(self) -> Self { self }
            }
            impl Normalize for $t {
                fn normalize(self) -> Self { self }
            }
        )*
    };
}

traverse_leaf!(
    u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize, f32, f64, bool, char, String,
    Ident,
);

impl<T: Expand> Expand for Box<T> {
    fn expand_all(self) -> Self {
        Box::new((*self).expand_all())
    }
}

impl<T: Expand> Expand for Option<T> {
    fn expand_all(self) -> Self {
        self.map(Expand::expand_all)
    }
}

impl<T: Expand> Expand for Vec<T> {
    fn expand_all(self) -> Self {
        self.into_iter().map(Expand::expand_all).collect()
    }
}

impl<K: Eq + Hash, V: Expand, S: BuildHasher + Default> Expand for HashMap<K, V, S> {
    fn expand_all(self) -> Self {
        self.into_iter().map(|(k, v)| (k, v.expand_all())).collect()
    }
}

impl<K: Ord, V: Expand> Expand for BTreeMap<K, V> {
    fn expand_all(self) -> Self {
        self.into_iter().map(|(k, v)| (k, v.expand_all())).collect()
    }
}

impl<A: Expand, B: Expand> Expand for (A, B) {
    fn expand_all(self) -> Self {
        (self.0.expand_all(), self.1.expand_all())
    }
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

impl<T: Normalize> Normalize for Box<T> {
    fn normalize(self) -> Self {
        Box::new((*self).normalize())
    }
}

impl<T: Normalize> Normalize for Option<T> {
    fn normalize(self) -> Self {
        self.map(Normalize::normalize)
    }
}

impl<T: Normalize> Normalize for Vec<T> {
    fn normalize(self) -> Self {
        self.into_iter().map(Normalize::normalize).collect()
    }
}

impl<K: Eq + Hash, V: Normalize, S: BuildHasher + Default> Normalize for HashMap<K, V, S> {
    fn normalize(self) -> Self {
        self.into_iter().map(|(k, v)| (k, v.normalize())).collect()
    }
}

impl<K: Ord, V: Normalize> Normalize for BTreeMap<K, V> {
    fn normalize(self) -> Self {
        self.into_iter().map(|(k, v)| (k, v.normalize())).collect()
    }
}

impl<A: Normalize, B: Normalize> Normalize for (A, B) {
    fn normalize(self) -> Self {
        (self.0.normalize(), self.1.normalize())
    }
}

impl<A: Normalize, B: Normalize, C: Normalize> Normalize for (A, B, C) {
    fn normalize(self) -> Self {
        (self.0.normalize(), self.1.normalize(), self.2.normalize())
    }
}
