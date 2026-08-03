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

/// The assertion every generated per-variant test goes through.
///
/// Asserts that an authored value and its lowered image serialize to the same
/// RON. The comparison runs through the serde derives on BOTH sides, so it
/// shares no code with the mapping arm under test: an arm that picked the wrong
/// variant, or swapped two same-typed fields, changes the bytes and fails. That
/// is the mapping's one blind spot — the compiler enforces totality, not
/// correctness, so a wrong-but-well-typed arm is invisible to it
/// (`docs/decisions/authoring-spelling-lowering.md` §13.2).
///
/// It replaces the withdrawn raise-map round-trip property (§9): a second full
/// mapping would have covered only terms whose every node maps by identity, so
/// its reach shrank with each divergence, starting with `plugin-repoint`'s
/// erasure of the `Expansion` arms.
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

/// The same rendering assertion for grammar types that carry no `Serialize`.
///
/// Some types are never written to RON (computed costs, lookup enums), so the
/// serialized comparison is unavailable. Derived `Debug` is an equivalent
/// structural witness: it prints variant and field names WITHOUT crate
/// qualification, so the authored and engine renderings compare directly.
///
/// This runs alongside the `assert_matches!` shape check, not instead of it,
/// and the two catch different defects. The pattern pins the TOP-LEVEL variant
/// — what a diverged arm must state — while the rendering pins NESTED
/// structure, so a defect inside a minimal value fails every test whose value
/// contains it. Measured: mis-mapping one `Zone` variant fails 7 tests with the
/// rendering assertions in place and 1 without.
#[cfg(test)]
pub(crate) fn assert_lowers_debug<A, C>(value: A)
where
    A: Lower<Target = C> + std::fmt::Debug,
    C: std::fmt::Debug,
{
    let authored = format!("{value:?}");
    let lowered = format!("{:?}", value.lower());
    assert_eq!(
        authored, lowered,
        "lowering changed the structure of this variant"
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
