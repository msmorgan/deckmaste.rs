//! The loader's dual result: one parse, both projections.
//!
//! The authored term is the source
//! (`docs/decisions/authoring-spelling-lowering.md` §3); the core value is its
//! image under `lower`, and is a compiled artifact. Both halves come from a
//! single read so nothing downstream can be comparing two parses that drifted.

/// A card as loaded: the authored term and its engine image.
#[derive(Debug)]
pub struct LoadedCard {
    /// The authored term — what the spelling side and the Idris mirror read.
    pub authored: deckmaste_authoring::Card,
    /// The engine image. `lower` erases nothing today; provenance erasure is
    /// `runtime-prose-link`'s (spec §12, sequenced 2026-08-02).
    pub core: deckmaste_card::Card,
}

/// A token as loaded. Mirrors [`LoadedCard`].
#[derive(Debug)]
pub struct LoadedToken {
    pub authored: deckmaste_authoring::Token,
    pub core: deckmaste_core::Token,
}
