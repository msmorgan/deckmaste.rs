//! The loader's dual result: one parse, both projections.
//!
//! The semantic term is the source
//! (`docs/decisions/semantics-spelling-lowering.md` §3); the core value is its
//! image under `lower`, and is a compiled artifact. Both halves come from a
//! single read so nothing downstream can be comparing two parses that drifted.
//!
//! `lower` erases every remembered macro invocation (spec §12,
//! `runtime-prose-link`), so `core` IS the expanded engine image — what the
//! Idris mirror emits and `xtask card` prints by default. There is no longer
//! a separate expanded projection to compute: `semantic.clone().expand_all()
//! .lower() == semantic.clone().lower()`, because erasure means `lower`
//! already drops every `Expansion` wrapper down to its body. Prose recovers
//! the semantic spelling a `core` value erased through the provenance index
//! (`deckmaste_plugin::provenance::ProvenanceIndex`) instead.

/// A card as loaded: the semantic term and its engine image.
#[derive(Debug)]
pub struct LoadedCard {
    /// The semantic term — what the spelling side and the Idris mirror read.
    pub semantic: deckmaste_semantics::Card,
    /// The engine image: `semantic` lowered, with every remembered macro
    /// invocation erased (spec §12).
    pub core: deckmaste_card::Card,
}

/// A token as loaded. Mirrors [`LoadedCard`].
#[derive(Debug)]
pub struct LoadedToken {
    pub semantic: deckmaste_semantics::Token,
    pub core: deckmaste_core::Token,
}

/// One card's RESOLVER verdict, from [`Plugin::card_resolution_from_str`](
/// crate::plugin::Plugin::card_resolution_from_str).
///
/// The resolver half of the certifier/resolver differential
/// (`semantics-spelling-lowering.md` §17): lowering's refusal travels as data,
/// paired with the card's printed name so the gate can report it.
#[derive(Debug, Clone)]
pub struct CardResolution {
    /// The card's printed name — its identity in a gate report.
    pub name: String,
    /// Lowering's verdict: the engine image, or the refusal.
    pub lowered: Result<deckmaste_card::Card, deckmaste_lowering::Diagnostic>,
}
