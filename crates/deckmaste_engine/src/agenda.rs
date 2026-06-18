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
    /// [CR#601.2b,602.2b]: announce the value of `{X}` in the in-flight cost
    /// (before targets, [CR#601.2c]). No-op when the cost has no `{X}`.
    AnnounceX,
    /// [CR#601.2c,602.2b]: surface `ChooseTargets` if the in-flight announce
    /// (spell or activated ability) has targets.
    AnnounceTargets,
    /// [CR#601.2b]: concretize the in-flight cost's hybrid/Phyrexian symbols.
    /// Surfaces `ChooseCostOptions` when the printed cost has any such symbol
    /// (the player announces each nonhybrid equivalent / color-or-2-life,
    /// [CR#107.4e,107.4f]); otherwise stashes the cost unchanged so `PayCost`
    /// uniformly reads the concretized stash. Sits between `AnnounceTargets`
    /// and `PayCost`. Reads/writes the announce slot.
    ChooseCostOptions,
    /// [CR#601.2f,601.2g,601.2h,602.2b]: pay the in-flight cost — mana +
    /// physical components ({T}/{Q}) for activations, mana only for spells.
    /// Surfaces `PayMana` when there is a choice; schedules tap/untap events
    /// for activations alongside it.
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
        riders: Vec<deckmaste_core::ManaRider>,
    },
    /// Resolve the named committed stack object ([CR#608]). Reads `self.stack`.
    Resolve(crate::object::ObjectId),
    /// Interpret one `Effect` node against a resolution frame ([CR#608.2]).
    RunEffect {
        effect: Box<deckmaste_core::Effect>,
        frame: crate::stack::Frame,
    },
    /// [CR#701.22a]: surface a `Distribute` decision — player sorts `window`
    /// into ordered `bins` (Top/Bottom/Graveyard). Dispatched from
    /// `player_action_items` after evaluating the looked-at group.
    OpenDistribute {
        player: crate::player::PlayerId,
        window: Vec<crate::object::ObjectId>,
        bins: Vec<deckmaste_core::Bin>,
        name: deckmaste_core::Ident,
    },
}
