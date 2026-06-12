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
#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize, Expand)]
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
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Deserialize, Serialize, Expand)]
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

/// What kind of value a note slot stores ([CR#607.2] linked slots; "the
/// chosen color" anaphora). Writers: `ChooseAndNote` (a resolution choice
/// that stores) and `Effect::Noting` (stores the object set the inner
/// effect touched — exiled-with). Readers: `Reference::Linked(key)`,
/// `Count::Noted(key)`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Deserialize, serde::Serialize, Expand)]
pub enum NotedKind {
    Color,
    CardName,
    Number,
    Objects,
}
