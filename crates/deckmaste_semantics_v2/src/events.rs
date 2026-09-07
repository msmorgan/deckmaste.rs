//! Shared event parameters, deed sets, and play-window values. Mirrors
//! `lean/Semantics/Events.lean`.

use crate::words::Deed;
use crate::words::Window;
use macro_ron::Expand;
use serde::Deserialize;
use serde::Serialize;

/// The deeds a deontic rule ranges over ("can't attack or block"): each is a core rules deed,
/// a declared keyword action, or the verb a keyword ability defines.
pub type Deeds = Vec<Deed>;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Deserialize, Expand, Serialize)]
pub enum CounterMove {
    Put,
    Removed,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Deserialize, Expand, Serialize)]
pub enum CounterBatch {
    One,
    Many,
    /// Removal that leaves no counters of this kind. Emptiness is part of the trigger event,
    /// evaluated at trigger time [CR#310.12b,702.62a], not an intervening if rechecked on resolution
    /// [CR#603.4]. Suspend's separate "if it's exiled" is the intervening if [CR#702.62a].
    Emptying,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Deserialize, Expand, Serialize)]
pub enum DiceBatch {
    One,
    Many,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Deserialize, Expand, Serialize)]
pub enum PaymentOutcome {
    Paid,
    Unpaid,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Deserialize, Expand, Serialize)]
pub enum LifeMove {
    Up,
    Down,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Deserialize, Expand, Serialize)]
pub enum TallyOp {
    Count,
    Sum,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Deserialize, Expand, Serialize)]
pub enum ReplUse {
    Repeatedly,
    NextTimeOnly,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Deserialize, Expand, Serialize)]
pub enum DamageKind {
    Any,
    CombatOnly,
    NoncombatOnly,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Deserialize, Expand, Serialize)]
pub enum CondMarking {
    AsLongAs,
    Unless,
    IfSo,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Deserialize, Expand, Serialize)]
pub enum PlayLimit {
    OnceEachYourTurn,
    OnceEachTurn,
}

/// When a play permission applies.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Deserialize, Expand, Serialize)]
pub enum PlayTiming {
    WhileSearchingLibrary,
    DuringEachOfYourTurns,
}

/// How "a …" chooses.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Expand, Serialize)]
pub enum ChoiceMode {
    Unmarked,
    /// "of their choice": the chooser is read in a window of the stack, so a clause's own
    /// subject shadows any player named before it.
    TheirChoice {
        window: Window,
    },
    AtRandom,
    YourChoice,
}
