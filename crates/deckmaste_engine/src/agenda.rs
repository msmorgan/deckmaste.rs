use deckmaste_core::Phase;

use crate::event::Occurrence;

/// One unit of engine work. `step()` pops exactly one; handlers schedule
/// follow-ups at the agenda *front*, ahead of previously queued work.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WorkItem {
    /// The interception seam: cant → replacements → apply → trigger-match.
    /// A `Single` event or a simultaneous `Batch`, applied together.
    Emit(Occurrence),
    /// Turn-structure transition plus that step's schedule.
    BeginStep(Phase),
    /// [CR#704.3]: state-based actions, checked before anyone gets priority.
    CheckSbas,
    /// [CR#603.3]: place noted triggers on the stack (APNAP, with an
    /// `OrderTriggers` decision and target choice at placement). Sits between
    /// the SBA loop and `OpenPriority`.
    PlaceTriggers,
    /// Cleanup's turn-based action ([CR#514.1]).
    CheckHandSize,
    /// [CR#508.1]: the Declare Attackers step's turn-based action — surface a
    /// `DeclareAttackers` decision for the active player.
    DeclareAttackers,
    /// [CR#509.1]: the Declare Blockers step's turn-based action — surface a
    /// `DeclareBlockers` decision for the defending player.
    DeclareBlockers,
    /// [CR#510.1]: the Combat Damage step's turn-based action — assign every
    /// source's combat damage (auto-resolving forced sources, surfacing an
    /// `AssignCombatDamage` decision for each multi-recipient one), then deal
    /// it all as one simultaneous batch ([CR#510.2]).
    AssignCombatDamage,
    /// [CR#511.3]: the End of Combat step's turn-based action — remove every
    /// creature from combat by clearing the combat-state registry.
    EndOfCombat,
    /// Surface `pending = Priority { .. }`.
    OpenPriority,
    /// [CR#601.2a,601.2b]: move the spell to the stack and open the announce slot.
    BeginCast(crate::object::ObjectId),
    /// [CR#602.2a,602.2b]: stage a non-mana activated ability — snapshot the
    /// ability text + source LKI into the announce slot. The shared
    /// `AnnounceTargets`/`PayCost` items follow; `AbilityActivated` promotes
    /// it onto the stack.
    BeginActivate {
        object: crate::object::ObjectId,
        ability: usize,
    },
    /// [CR#601.2c]: surface `ChooseTargets` if the in-flight spell targets.
    AnnounceTargets,
    /// [CR#601.2f,601.2g,601.2h]: pay the in-flight spell's cost (surface `PayMana` if there
    /// is a choice; auto-pay when forced).
    PayCost,
    /// [CR#701.9b]: a resolving discard — surface a `DiscardCards` decision
    /// for `player` to choose which `count` cards to discard (clamped to the
    /// hand size when the item applies; an empty hand surfaces nothing).
    DiscardCards {
        player: crate::player::PlayerId,
        count: deckmaste_core::Uint,
    },
    /// [CR#106.1b]: a resolving `AddMana` whose production is a choice ("any
    /// color", "{W} or {U}") — surface a `ChooseManaColor` decision for
    /// `player` to pick one of `options`.
    ChooseManaColor {
        player: crate::player::PlayerId,
        options: Vec<deckmaste_core::ColorOrColorless>,
        amount: deckmaste_core::Uint,
    },
    /// Resolve the named committed stack object ([CR#608]). Reads `self.stack`.
    Resolve(crate::object::ObjectId),
    /// Interpret one `Effect` node against a resolution frame ([CR#608.2]).
    RunEffect {
        effect: Box<deckmaste_core::Effect>,
        frame: crate::stack::Frame,
    },
}
