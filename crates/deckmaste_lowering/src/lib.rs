//! The one-way compile: `deckmaste_semantics` → `deckmaste_core` /
//! `deckmaste_card`.
//!
//! The only edge between the two grammars; neither depends on the other
//! (`docs/decisions/semantics-spelling-lowering.md` §1, §9).
//!
//! This crate is the divergence ledger: arms are identities while the grammars
//! mirror each other, and any arm that stops being one carries its reason in
//! place. One module per grammar module.

use std::sync::Arc;

use macro_ron::ExpansionArgs;
use macro_ron::Ident;

/// The total map from a semantic value to its engine image.
///
/// Every field lowers by calling `.lower()` on it, so no arm needs to know what
/// type a field holds — which is what let the arms be scaffolded.
pub trait Lower {
    /// The engine-side type this lowers to.
    type Target;

    /// Compile this semantic value to its engine image.
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

/// The fallback for a type a pattern cannot reach: `ManaCost` wraps a private
/// field, which stable Rust cannot match across a crate boundary. Comparing
/// serializations gets the same evidence through the serde derives instead.
#[cfg(test)]
pub(crate) fn assert_lowers<A, C>(value: A)
where
    A: Lower<Target = C> + serde::Serialize,
    C: serde::Serialize,
{
    let semantic = deckmaste_semantics::ron::options()
        .to_string(&value)
        .expect("semantic value serializes");
    let lowered = deckmaste_core::ron::options()
        .to_string(&value.lower())
        .expect("lowered value serializes");
    assert_eq!(
        semantic, lowered,
        "lowering changed the serialized form of this variant"
    );
}

/// A card whose text could not be compiled.
///
/// The ADR makes an unresolvable anaphor a COMPILATION error with provenance
/// (law 12: "resolution happens once, in lowering … R1/R2 as per-card
/// diagnostics"), never an eval-time refusal — so the resolver's R1/R2 verdict
/// reaches a corpus compiler as data naming the offending card, not as a bare
/// panic from somewhere inside the tree walk.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Diagnostic {
    /// The card that failed to compile — its front face's name.
    pub card: Arc<str>,
    /// The refusal, already prefixed with the card name.
    pub message: String,
}

impl std::fmt::Display for Diagnostic {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.message)
    }
}

impl std::error::Error for Diagnostic {}

/// The front face's name — the card's diagnostic identity.
fn card_name(card: &deckmaste_semantics::Card) -> Arc<str> {
    match card {
        deckmaste_semantics::Card::Normal(face) => Arc::clone(&face.name),
        deckmaste_semantics::Card::TwoFaced { front, .. } => Arc::clone(&front.name),
    }
}

#[cfg(test)]
fn lower_for_test<T>(f: impl FnOnce() -> T) -> Result<T, Diagnostic> {
    let card: Arc<str> = Arc::from("Lowering Test");
    region::in_card(&card, f).map_err(|message| Diagnostic { card, message })
}

/// Compile one card, returning a context-sensitive refusal as a [`Diagnostic`].
///
/// This is the entry a corpus compiler uses: a card whose text names an
/// antecedent the resolver cannot pin (R2) is reported against that card and
/// the rest of the corpus still compiles. `Lower` itself stays an infallible
/// recursive map; the card-level compiler context carries the refusal, and
/// this walk entry is the only layer that observes it as a `Result`.
///
/// # Errors
///
/// Returns a [`Diagnostic`] when the card's text cannot be lowered: an
/// ambiguous or unbound discourse anaphor, or another explicit resolver
/// refusal.
pub fn lower_card(card: deckmaste_semantics::Card) -> Result<deckmaste_card::Card, Diagnostic> {
    let name = card_name(&card);
    region::in_card(&name, || {
        // [CR#607.1]: linkage is between two abilities printed on ONE
        // object, so a cell read in one ability and written in another is
        // not knowable from either ability alone. Pass one collects the
        // card's cell reads and writes; pass two runs only when the card
        // actually reads a cell, and declares the surviving ones as
        // `Provenance::Linked` parameters (ADR law 8).
        let (plan, first) = region::collect_cells(|| card.clone().lower());
        if region::plan_is_empty(&plan) {
            first
        } else {
            region::with_cells(plan, || card.lower())
        }
    })
    .map_err(|message| Diagnostic {
        card: Arc::clone(&name),
        message,
    })
}

#[cfg(test)]
mod minimal;

mod ability;
mod action;
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
mod region;
mod replacement;
mod sba_rule;
mod selection;
mod stat_value;
mod status;
mod target_spec;
mod temporal;
mod token;
mod r#type;
mod zone;
