use std::sync::Arc;

use crate::Action;
use crate::Predicate;
use crate::Quantity;
use crate::Reference;
use crate::Selection;
use crate::Zone;

/// The binder feeding [`OneShotEffect::With`](crate::With),
/// [`Each`](crate::Each), and [`Distribute`](crate::Distribute) — the Idris
/// `Bindable`, collapsed (cardinality is encoded by the variant, not a type
/// index). A one-binder (`TheRef`/`ChooseOne`) binds a single object read as
/// [`Reference::That`](crate::Reference::That); a many-binder
/// (`Choose`/`Existing`) binds a group read as
/// [`Selection::That`](crate::Selection::That). `Each`/`Distribute` take a
/// many-binder and expose each element in turn as
/// [`Reference::It`](crate::Reference::It).
///
/// The `Produce` binder is wired into engine resolution ([CR#400.7j]): a
/// `With(Produce(action), body)` runs the action and binds its moved product as
/// the singular `That`, chased through the same-resolution move record — the
/// "…this way" linkage madness's exile-and-cast rides. Only a `Move` action
/// produces-and-captures in this cut; other producer actions stay a labeled
/// seam. The search binders (`Search`/`SearchOne`) are wired into engine
/// resolution too ([CR#701.23,701.24]): the engine surfaces a `ChooseObjects`
/// decision over the (possibly hidden) `from` zones of `whose`, honoring the
/// three fail-to-find shapes ([CR#701.23b..701.23d]), and binds the result as
/// `That`/group — `if_none` runs, unbound, on a failed find. Reveal and
/// shuffle are NOT binder-level behavior: they ride the body as ordinary
/// `Reveal`/`Shuffle` steps, same as any other effect (see
/// `resolve/effect.rs`). All (de)serialize and round-trip here.
#[derive(Debug, Clone, PartialEq, Eq, Hash, serde::Deserialize, serde::Serialize)]
pub enum Binder {
    /// Bind an existing single reference (a captured target, `This`) — One →
    /// `That`.
    TheRef(Reference),
    /// The chooser picks exactly one match — One → `That`. ("sacrifice a
    /// creature") `by` names who chooses, mirroring the Idris `{default You
    /// by}`: the default is the controller; a foreign chooser ("that player
    /// sacrifices a creature of their choice", [CR#608.2d,701.21a]) overrides
    /// it.
    ChooseOne {
        /// What may be picked (the Idris `Predicate`).
        filter: Predicate,
        /// Who chooses (default `You`).
        #[serde(default = "ref_you", skip_serializing_if = "ref_is_you")]
        by: Reference,
    },
    /// Run an [`Action`] for effect and bind its product — the moved/created
    /// object — as the singular [`Reference::That`](crate::Reference::That) —
    /// One → `That`. The Idris `Produce : Action b -> Bindable b One AnObject`
    /// (Cavern-of-Souls-style "exile it":
    /// `With(Produce(Move(It, Exile)), …)`). Boxed: an open [`Action`] is the
    /// largest leaf enum (`clippy::large_enum_variant`).
    Produce(Arc<Action>),
    /// Search `whose`'s `from`-zones (hidden libraries/graveyards) for exactly
    /// ONE match, bound as `That` — One → `That`. The Idris `SearchOne`;
    /// `by`/`whose` default to `You`, `from` to `[Library]` (each omitted on
    /// write when default).
    SearchOne {
        /// The card sought (the Idris `Predicate`).
        filter: Predicate,
        /// Who performs the search (default `You`).
        #[serde(default = "ref_you", skip_serializing_if = "ref_is_you")]
        by: Reference,
        /// Whose zones are searched (default `You` — own library) — e.g.
        /// Bribery searches an opponent's library.
        #[serde(default = "ref_you", skip_serializing_if = "ref_is_you")]
        whose: Reference,
        /// The zones searched (default `[Library]`; e.g. library and/or
        /// graveyard).
        #[serde(default = "from_library", skip_serializing_if = "is_from_library")]
        from: Arc<[Zone]>,
        /// The explicit WHIFF branch ([CR#701.23b] — a search may fail to
        /// find): what happens when nothing is found. Whiff semantics are
        /// DATA — a whiffed search skips the product-dependent body, and the
        /// branch here runs instead; it runs WITHOUT the product
        /// binding, so reading the search's `That` inside it is unsound
        /// (rejected by the Idris re-emit gate).
        #[serde(default, skip_serializing_if = "Option::is_none")]
        if_none: Option<Arc<crate::OneShotEffect>>,
    },
    /// The chooser picks a quantity of matches — Many → `That` (group).
    /// ("choose two cards") Same `by` default as
    /// [`ChooseOne`](Self::ChooseOne).
    Choose {
        /// How many to pick (the Idris `Quantity`).
        quantity: Quantity,
        /// What may be picked (the Idris `Predicate`).
        filter: Predicate,
        /// Who chooses (default `You`).
        #[serde(default = "ref_you", skip_serializing_if = "ref_is_you")]
        by: Reference,
    },
    /// An existing group/selection — Many → `That` (group).
    Existing(Selection),
    /// Search `whose`'s `from`-zones for a [`Quantity`] of matches, bound as a
    /// group `That` (`Each`-iterated) — Many → `That`. The Idris `Search`,
    /// like [`Choose`](Self::Choose) but over (hidden) searched zones the
    /// engine reveals/shuffles. Same `by`/`whose`/`from` defaults as
    /// [`SearchOne`](Self::SearchOne).
    Search {
        /// How many cards to find (the Idris `Quantity`).
        quantity: Quantity,
        /// The cards sought (the Idris `Predicate`).
        filter: Predicate,
        /// Who performs the search (default `You`).
        #[serde(default = "ref_you", skip_serializing_if = "ref_is_you")]
        by: Reference,
        /// Whose zones are searched (default `You`).
        #[serde(default = "ref_you", skip_serializing_if = "ref_is_you")]
        whose: Reference,
        /// The zones searched (default `[Library]`).
        #[serde(default = "from_library", skip_serializing_if = "is_from_library")]
        from: Arc<[Zone]>,
        /// The explicit whiff branch ([CR#701.23b]) — see
        /// [`SearchOne::if_none`](Binder::SearchOne). Elaborates without the
        /// searched group bound.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        if_none: Option<Arc<crate::OneShotEffect>>,
    },
}

/// The `by`/`whose` default — the searching/choosing player is the controller
/// ([`Reference::You`]). Mirrors the Idris `{default You by}` / `{default You
/// whose}` on `ChooseOne`/`Choose`/`Search`/`SearchOne`.
fn ref_you() -> Reference {
    Reference::You
}

/// Whether a [`Reference`] is the default `You` (so it is omitted on write).
fn ref_is_you(r: &Reference) -> bool {
    matches!(r, Reference::You)
}

/// The default search domain — a player's library. Mirrors the Idris
/// `{default [Library] from}` on `Search`/`SearchOne`.
fn from_library() -> Arc<[Zone]> {
    vec![Zone::Library].into()
}

/// Whether `from` is the default single-`Library` domain (so it is omitted on
/// write).
fn is_from_library(zones: &[Zone]) -> bool {
    zones == [Zone::Library]
}
