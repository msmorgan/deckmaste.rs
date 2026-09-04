//! The decision-schema vocabulary (mtg-rules choices.md §§1–3, §6): who
//! decides, how the choice is seen, when it locks (`LockPoint`). The
//! engine's `DecisionPoint` record carries these; cards never spell them
//! directly (the rules do), so the enums are closed.

use serde::Deserialize;
use serde::Serialize;

use crate::Expand;
use crate::Reference;

/// The NOMINAL decider of a decision ([CR#700.2a,608.2d,508.1a,509.1a],
/// vote: [CR#701.38a]); the engine resolves nominal → actual (delegation,
/// rebinding). `Rng` is the pseudo-decider of coin flips and die rolls
/// (choices.md §4).
#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Expand, Serialize)]
pub enum DeciderSpec {
    /// The spell/ability's controller (announce-stage choices).
    Controller,
    /// The active player (attack declaration, [CR#508.1a]).
    ActivePlayer,
    /// The defending player (block declaration, [CR#509.1a]).
    DefendingPlayer,
    /// A player the effect names ([CR#608.2d] resolution choices).
    Named(Reference),
    /// Each player in turn order from a specified player (votes,
    /// [CR#701.38a]; opening-hand actions, [CR#103.6]).
    EachInTurnOrder,
    /// The player holding priority (the engine's action loop — not a
    /// choices.md row; the priority window itself, [CR#117.1]).
    PriorityHolder,
    /// The RNG pseudo-player (flips/rolls — replays serialize the
    /// selections like any decider's).
    Rng,
}

/// How a choice is seen by other players (choices.md §3).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Deserialize, Expand, Serialize)]
pub enum Visibility {
    /// The default: made publicly; later deciders see earlier choices
    /// ([CR#101.4b]).
    Open,
    /// Chosen, binding, concealed (morph identity, [CR#708.2,708.5];
    /// hidden agenda, [CR#702.106a..702.106b]; London bottoming,
    /// [CR#103.5]). The reveal-for-audit duty ([CR#708.9]) is engine
    /// bookkeeping on the committed payload, not part of this value.
    CommittedHidden,
}

/// A CHOSEN VALUE's kind ([CR#607.2] linked slots; "the chosen color"
/// anaphora) — [`crate::Action::ChooseValue`]'s kind-space (né
/// `ChooseAndNote`/`NotedKind`, split per the action-role-reshape design,
/// 2026-08-01): a resolution CHOICE the player makes and the engine stores
/// under a note key, distinct from the persisted OBJECT-SET note kinds
/// ([`NotedKind`], written by
/// [`Noting`](crate::Noting)/[`SeparatePiles`](crate::SeparatePiles) and
/// staying store-side with their writers). During lowering, chosen scalar
/// values bind a region definition; object sets use `Reference::Linked(key)`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Deserialize, Expand, serde::Serialize)]
pub enum ChosenValueKind {
    Color,
    CardName,
    Number,
}

/// A persisted OBJECT-SET note's kind ([CR#607.2] linked slots) — the
/// store-side kinds [`ChosenValueKind`] does NOT cover: written by
/// [`Noting`](crate::Noting) (the object set an inner effect touched,
/// [CR#607.2a] exiled-with linkage) and `SeparatePiles { note, .. }`
/// (labeled pile groups, keyed by (note, label, divider) — read back via
/// `Selection::PilesOf`, [CR#700.3a]). Never a choice-node kind (that is
/// [`ChosenValueKind`]) — the note store's key-typing keeps both kind
/// vocabularies, split by writer.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Deserialize, Expand, serde::Serialize)]
pub enum NotedKind {
    Objects,
    Piles,
}
