//! The one-way compile: `deckmaste_authoring` → `deckmaste_core` /
//! `deckmaste_card`.
//!
//! This crate is the only edge between the two grammars. Neither grammar crate
//! depends on the other; the build graph is the architecture
//! (`docs/decisions/authoring-spelling-lowering.md` §1, §9).
//!
//! **This crate IS the divergence ledger.** The mapping starts as an exact
//! mirror because the fork duplicated the freshly-reshaped grammar, so every
//! arm is currently an identity. Shapes stay mirrored unless a divergence earns
//! its mapping complexity, and every non-identity arm carries its justification
//! in place, in the module where it lives. Authored terms have no independent
//! semantics: a term means its image under `lower`.
//!
//! One arm per grammar module, mirroring the fork's own layout, so a change on
//! either side maps to exactly one file here.

use std::sync::Arc;

use macro_ron::Expansion;
use macro_ron::ExpansionArgs;
use macro_ron::Ident;

/// The total map from an authored value to its engine image.
///
/// Implemented for every type in the authored grammar (the generated arms in
/// this crate's modules) and for the containers and leaves those types are
/// built from (the blanket impls below). Uniformity is what let the arms be
/// scaffolded: every field lowers by calling `.lower()` on it, so no arm needs
/// to know what type a field holds.
pub trait Lower {
    /// The engine-side type this lowers to.
    type Target;

    /// Compile this authored value to its engine image.
    fn lower(self) -> Self::Target;
}

/// Leaves that are the SAME type on both sides: primitives, and the shared
/// `macro_ron` vocabulary both grammars depend on. These are identities in the
/// strict sense — target type equals source type — not merely identity-shaped.
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
        // Reuse the allocation's contents when this is the last handle; clone
        // only when the value is genuinely shared.
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

/// Invocation provenance: a remembered macro call carrying the value its body
/// expanded to.
///
/// Identity-shaped for now — the wrapper survives lowering, so a core value can
/// still carry `Expanded(…)`. `plugin-repoint` relocates provenance to the
/// authored side and turns this into the first ERASURE arm in the crate
/// (`self.value.lower()`, dropping name/args/template), at which point core
/// values carry no wrappers at all and the ~97 `Expanded(…)` match sites in
/// `deckmaste_engine` stop existing (spec §12).
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

/// The fallback assertion for a type a pattern cannot reach.
///
/// Every per-variant test states its expected engine shape with
/// `assert_matches!`. Exactly one type defeats that: `ManaCost` wraps a PRIVATE
/// field, and stable Rust cannot match a tuple struct with private fields from
/// another crate. Comparing the two serializations gets the same evidence by
/// another route — the comparison runs through the serde derives on BOTH sides,
/// so it shares no code with the arm under test.
///
/// Together these replace the withdrawn raise-map round-trip property (§9): a
/// second full mapping would have covered only terms whose every node maps by
/// identity, so its reach shrank with each divergence, starting with
/// `plugin-repoint`'s erasure of the `Expansion` arms.
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
