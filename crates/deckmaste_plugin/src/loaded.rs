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

impl LoadedCard {
    /// The engine image with every remembered macro invocation stripped — what
    /// the Idris mirror emits and `xtask card` prints by default.
    ///
    /// Expansion runs on the AUTHORED term: `Expand` is the authoring
    /// grammar's trait, and `deckmaste_card` does not depend on `macro_ron` at
    /// all. Lowering the expanded term (rather than expanding the lowered one)
    /// keeps `lower` inside this crate, where the erasure point lives.
    ///
    /// Swapping the order is value-preserving only because every `Expansion`
    /// arm in the lowering map is an identity and no arm case-analyses a
    /// child's constructor — `runtime-prose-link`, which makes those arms
    /// erase, has to re-establish it.
    #[must_use]
    pub fn expanded(&self) -> deckmaste_card::Card {
        use deckmaste_lowering::Lower as _;
        use macro_ron::Expand as _;

        self.authored.clone().expand_all().lower()
    }
}

/// A token as loaded. Mirrors [`LoadedCard`].
#[derive(Debug)]
pub struct LoadedToken {
    pub authored: deckmaste_authoring::Token,
    pub core: deckmaste_core::Token,
}
