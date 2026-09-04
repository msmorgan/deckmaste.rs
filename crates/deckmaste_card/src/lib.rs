//! The engine's unit of card definitions: `Card`, `Characteristics`, and
//! `CardFace` (a `Characteristics` plus whatever its layout adds) — the
//! packaging of `deckmaste_core`'s loose primitives into a playable unit.
//! Depends on core; core never depends on this crate.

/// `skip_serializing_if` helper for slice-backed fields (`Vec<T>` and
/// `Arc<[T]>` alike, via deref coercion at the call site).
//
// A private three-line copy of `deckmaste_core`'s identical helper. Core's is
// `pub(crate)`; exporting a serde helper just to share it would widen core's
// public API for no gain.
pub(crate) fn slice_is_empty<T>(s: &[T]) -> bool {
    s.is_empty()
}

mod card;
pub use card::Card;
pub use card::CardFace;
pub use card::Characteristics;
pub use card::DoubleFacedLayout;
