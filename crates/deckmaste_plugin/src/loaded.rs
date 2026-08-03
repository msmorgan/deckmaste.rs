//! The loader's dual result: one parse, both projections.
//!
//! The authored term is the source
//! (`docs/decisions/authoring-spelling-lowering.md` §3); the core value is its
//! image under `lower`, and is a compiled artifact. Both halves come from a
//! single read so nothing downstream can be comparing two parses that drifted.
//!
//! `lower` erases every remembered macro invocation (spec §12,
//! `runtime-prose-link`), so `core` IS the expanded engine image — what the
//! Idris mirror emits and `xtask card` prints by default. There is no longer
//! a separate expanded projection to compute: `authored.clone().expand_all()
//! .lower() == authored.clone().lower()`, because erasure means `lower`
//! already drops every `Expansion` wrapper down to its body. Prose recovers
//! the authored spelling a `core` value erased through the provenance index
//! (`deckmaste_plugin::provenance::ProvenanceIndex`) instead.

/// A card as loaded: the authored term and its engine image.
#[derive(Debug)]
pub struct LoadedCard {
    /// The authored term — what the spelling side and the Idris mirror read.
    pub authored: deckmaste_authoring::Card,
    /// The engine image: `authored` lowered, with every remembered macro
    /// invocation erased (spec §12).
    pub core: deckmaste_card::Card,
}

/// A token as loaded. Mirrors [`LoadedCard`].
#[derive(Debug)]
pub struct LoadedToken {
    pub authored: deckmaste_authoring::Token,
    pub core: deckmaste_core::Token,
}
