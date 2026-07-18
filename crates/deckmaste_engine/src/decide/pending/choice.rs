use deckmaste_core::Uint;

use crate::decide::PreGameKind;
use crate::object::ObjectId;
use crate::player::PlayerId;

/// [CR#514.1]: discard down to maximum hand size.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DiscardToHandSize {
    pub player: PlayerId,
    pub count: Uint,
}

/// [CR#701.9b]: a resolving discard — `player` chooses which `count` cards
/// from their hand to discard (`count` already clamped to the hand size).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DiscardCards {
    pub player: PlayerId,
    pub count: Uint,
}

/// [CR#705.2]: a called coin flip — the flipper calls heads or tails
/// before the draw. Answered with `Decision::Answer` (`true` = heads).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CallFlip {
    pub player: PlayerId,
}

/// [CR#603.3b]: a player controlling several simultaneous triggers orders
/// them. The submitted `Order` is a permutation of `0..triggers.len()`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OrderTriggers {
    pub player: PlayerId,
    pub triggers: Vec<crate::trigger::NotedTrigger>,
}

/// Divide damage/counters among targets ([CR#601.2d,608.2d]) — shell.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Division {
    pub player: PlayerId,
    pub total: Uint,
    pub targets: Vec<ObjectId>,
}

/// Vote, each player in turn order ([CR#701.38a]) — shell.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Vote {
    pub player: PlayerId,
    pub options: Uint,
}

/// A fixed-window yes/no ("… unless you pay", [CR#608.2d]) — shell.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct YesNo {
    pub player: PlayerId,
}

/// [CR#608.2c,608.2d]: a resolution-time number choice for a
/// `ChooseAndNote(key, NotedKind::Number)` ("choose a number"). Any
/// nonnegative value is legal (unbounded, like `ChooseXValue`); the answer
/// — reusing the `Decision::XValue(Uint)` shape, which is exactly a chosen
/// nonnegative number — is stored in `resolution_notes[key]`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ChooseNoteNumber {
    pub player: PlayerId,
    pub key: deckmaste_core::Ident,
}

/// Choose a card name and note it for this resolution.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ChooseNoteCardName {
    pub player: PlayerId,
    pub key: deckmaste_core::Ident,
}

/// Order the replacement/prevention effects applicable to one event,
/// affected player/controller choosing ([CR#616.1]) — shell.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OrderReplacements {
    pub player: PlayerId,
    pub count: Uint,
}

/// [CR#616.1]: two or more replacement effects are applicable to one event;
/// the affected player chooses which to apply first. The loop resumes after
/// the choice via `ReplaceState` in `GameState.replace_state`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ChooseReplacement {
    pub chooser: PlayerId,
    /// The replacement keys the player may choose among (all applicable
    /// and not yet in the [CR#614.5] lineage set for this event chain).
    pub applicable: Vec<crate::replace_registry::ReplacementKey>,
}

/// A pre-game choice ([CR#103]) — shell.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PreGame {
    pub player: PlayerId,
    pub kind: PreGameKind,
}

/// [CR#608.2d]: choose objects at resolution. `candidates` is the matching
/// set; the answer picks between `min` and `max` of them (both clamped to
/// `candidates.len()` — "as many as able").
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ChooseObjects {
    pub player: PlayerId,
    pub candidates: Vec<ObjectId>,
    pub min: Uint,
    pub max: Uint,
}

/// [CR#704.5j] the legend rule: `player` controls two or more legendary
/// permanents with the same name (`candidates`); they choose exactly one to
/// keep and the rest are put into their owners' graveyards. Surfaced by the
/// SBA driver, resolved in `submit_decision`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LegendRule {
    pub player: PlayerId,
    pub candidates: Vec<ObjectId>,
}

/// [CR#401.4]: `player` orders a pile of more than one card that came to
/// rest at one end of a library — scry's "on top … in any order" and
/// Brainstorm's "in any order". `objects` is the pile in its current
/// library order; the answer ([`Decision::Arranged`]) is a permutation of
/// it (top → down).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ArrangePile {
    pub player: PlayerId,
    pub objects: Vec<ObjectId>,
}
