//! The one-way compile: `deckmaste_authoring` → `deckmaste_core` /
//! `deckmaste_card`.
//!
//! The only edge between the two grammars; neither depends on the other
//! (`docs/decisions/authoring-spelling-lowering.md` §1, §9).
//!
//! This crate is the divergence ledger: arms are identities while the grammars
//! mirror each other, and any arm that stops being one carries its reason in
//! place. One module per grammar module.

use std::sync::Arc;

use macro_ron::Expansion;
use macro_ron::ExpansionArgs;
use macro_ron::Ident;

/// The total map from an authored value to its engine image.
///
/// Every field lowers by calling `.lower()` on it, so no arm needs to know what
/// type a field holds — which is what let the arms be scaffolded.
pub trait Lower {
    /// The engine-side type this lowers to.
    type Target;

    /// Compile this authored value to its engine image.
    fn lower(self) -> Self::Target;
}

/// Leaves that are the same type on both sides: primitives and the shared
/// `macro_ron` vocabulary.
macro_rules! lower_identity {
    ($($t:ty),* $(,)?) => {$(
        impl Lower for $t {
            type Target = $t;
            fn lower(self) -> Self::Target {
                self
            }
        }
    )*};
}

lower_identity!(
    bool,
    u32,
    i32,
    usize,
    String,
    Arc<str>,
    Ident,
    ExpansionArgs
);

impl<T: Lower> Lower for Option<T> {
    type Target = Option<T::Target>;

    fn lower(self) -> Self::Target {
        self.map(Lower::lower)
    }
}

impl<T: Lower> Lower for Box<T> {
    type Target = Box<T::Target>;

    fn lower(self) -> Self::Target {
        Box::new((*self).lower())
    }
}

impl<T: Lower> Lower for Vec<T> {
    type Target = Vec<T::Target>;

    fn lower(self) -> Self::Target {
        self.into_iter().map(Lower::lower).collect()
    }
}

// `T: Sized` is implied here, so this does not overlap the `Arc<[T]>` or
// `Arc<str>` impls — both of those are unsized payloads.
impl<T: Lower + Clone> Lower for Arc<T> {
    type Target = Arc<T::Target>;

    fn lower(self) -> Self::Target {
        // Clone only when the value is genuinely shared.
        let inner = Arc::try_unwrap(self).unwrap_or_else(|shared| (*shared).clone());
        Arc::new(inner.lower())
    }
}

impl<T: Lower + Clone> Lower for Arc<[T]> {
    type Target = Arc<[T::Target]>;

    fn lower(self) -> Self::Target {
        self.iter().cloned().map(Lower::lower).collect()
    }
}

/// Invocation provenance. Identity-shaped for now; `plugin-repoint` turns this
/// into the crate's first erasure arm (spec §12).
impl<T: Lower> Lower for Expansion<T> {
    type Target = Expansion<T::Target>;

    fn lower(self) -> Self::Target {
        Expansion {
            name: self.name,
            args: self.args,
            template: self.template,
            value: self.value.lower(),
        }
    }
}

/// The fallback for a type a pattern cannot reach: `ManaCost` wraps a private
/// field, which stable Rust cannot match across a crate boundary. Comparing
/// serializations gets the same evidence through the serde derives instead.
#[cfg(test)]
pub(crate) fn assert_lowers<A, C>(value: A)
where
    A: Lower<Target = C> + serde::Serialize,
    C: serde::Serialize,
{
    let authored = deckmaste_authoring::ron::options()
        .to_string(&value)
        .expect("authored value serializes");
    let lowered = deckmaste_core::ron::options()
        .to_string(&value.lower())
        .expect("lowered value serializes");
    assert_eq!(
        authored, lowered,
        "lowering changed the serialized form of this variant"
    );
}

#[cfg(test)]
mod minimal;

mod ability;
mod action;
mod binder;
mod card;
mod color;
mod condition;
mod conferral_rule;
mod continuous;
mod copy;
mod cost;
mod count;
mod counter;
mod damage_result_rule;
mod decision;
mod deontic;
mod designation;
mod effect;
mod event;
mod filter;
mod keyword;
mod mana;
mod property;
mod quantity;
mod reference;
mod replacement;
mod sba_rule;
mod selection;
mod sort;
mod stat_value;
mod status;
mod target_spec;
mod temporal;
mod token;
mod r#type;
mod zone;
