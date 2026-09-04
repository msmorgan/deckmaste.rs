use std::sync::Arc;

use deckmaste_core::PhaseStep;

use crate::event::GameEvent;
use crate::event::Occurrence;

/// Which end of a library an [`crate::action::Anchor`]-placed card sits at —
/// the pile axis a post-pick arrangement groups by ([CR#401.4]). Derived from
/// the [`deckmaste_core::Anchor`] variant (`FromTop`→`Top`, `FromBottom`→
/// `Bottom`); the numeric offset rides alongside.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum LibraryEnd {
    Top,
    Bottom,
}

/// What must have committed since a `FinalizeAct`'s `mark` for the keyword
/// action's PAST name-fact to record ([CR#616.1] finalization). A future `Act`
/// opens the ONE replacement window; its `FinalizeAct` watcher then OBSERVES
/// the outcome — whether the action passed unchanged, was redirected, or was
/// replaced away — and records the name-fact iff the verb's characteristic
/// change actually landed. This is what makes a redirected discard/mill still a
/// discard/mill ([CR#701.9c,701.17c]) while a fully-replaced one leaves no fact
/// or trigger ([CR#614.17]) — with no facet-rewriting machinery.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FinalizeWatch {
    /// ≥1 of these patients zone-changed since `mark`, to ANY destination
    /// ([CR#701.9c,701.17c]: a redirected card is still discarded/milled) —
    /// the move verbs (destroy/discard/mill). Resolved to concrete ids before
    /// the window, so it observes the outcome whether the action passed or was
    /// replaced.
    Patients(Vec<crate::object::ObjectId>),
    /// A `Draw`-caused move for this player committed since `mark` (draw,
    /// [CR#121.2]) — an empty-library attempt commits none, so no draw fact.
    Performer(crate::player::PlayerId),
    /// The body's arrange ran (scry/surveil/fateseal, [CR#701.22d]).
    /// Scheduled only by the passed action's apply (a replaced action never
    /// reaches it), so it records unconditionally — the body definitely ran.
    BodyRan,
    /// [CR#616.1g,121.2a]: ≥1 of the `n` per-entity contained futures a
    /// `Batch` aggregate window's PASSED apply scheduled itself committed
    /// since `mark` — the aggregate finalizes iff ANY of its contents did
    /// (Bruvac's mill-6 might see some cards redirected elsewhere; the
    /// aggregate still finalizes off the ones that landed, mirroring
    /// `Patients`' "any destination" leniency one level up). Planted from
    /// the aggregate's own PASSED apply, like Draw and the reorder verbs
    /// (never at resolve time — a replaced-to-nothing aggregate's apply
    /// never runs, so it never plants a `FinalizeAct` that could later see
    /// an unrelated same-verb resolution and spuriously finalize;
    /// [CR#614.1]). So `mark` covers only what THIS aggregate's own `n`
    /// contained futures do. Direct aggregate lanes (currently Mill's one
    /// simultaneous move batch) report through their cause-tagged committed
    /// event; repeated contained `Act` lanes report the success decision made
    /// by each contained future's OWN verb-appropriate `FinalizeWatch`. That
    /// second signal is what lets a body-only verb such as Fight finalize
    /// without guessing from damage or adding a per-verb branch here. Scoped
    /// by the aggregate's own verb so an unrelated nested action can't be
    /// mistaken for one of this aggregate's contents.
    AnyContained(deckmaste_core::VerbName),
}

/// The two resolution-scoped cursors a [`FinalizeWatch`] starts observing at.
/// Ordinary keyword actions read `events`; an aggregate's `AnyContained`
/// watcher also reads `contained_act_serial`, the private success ledger's
/// monotonic cursor. Keeping both cursors makes the boundary exact even when
/// several successful body-only actions produce no public game event between
/// them, without retaining one marker per batch element.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FinalizeMark {
    pub(crate) events: usize,
    pub(crate) contained_act_serial: u64,
}

/// The runtime result a producing instruction writes after its action and all
/// immediately-caused work have drained.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MagnitudeSource {
    CoinFlips { called: bool },
    DiceRolls,
    ZoneChanges(deckmaste_core::VerbName),
}

/// One unit of engine work. `step()` pops exactly one; handlers schedule
/// follow-ups at the agenda *front*, ahead of previously queued work.
#[derive(Debug, Clone, PartialEq, Eq)]
#[allow(
    clippy::large_enum_variant,
    reason = "engine work items are processed one-by-one on the hot path"
)]
pub enum WorkItem {
    /// The interception seam: cant → replacements → apply → trigger-match.
    /// A `Single` event or a simultaneous `Batch`, applied together.
    Emit(Occurrence),
    /// [CR#704.3]: apply a state-based-action batch produced by `check_sbas`'s
    /// sweep, and re-schedule `CheckSbas` ONLY when the apply produced ≥1
    /// fact. A batch where every event was suppressed (e.g. an `Act(Destroy)`
    /// canted by indestructible) applies to an empty
    /// `Occurrence::Batch(vec![])` — that "nothing changed" signal
    /// terminates the SBA loop without looping forever on a persistent
    /// condition.
    EmitSbaBatch(Vec<GameEvent>),
    /// Turn-structure transition plus that step's schedule.
    BeginStep(PhaseStep),
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
    /// [CR#608.2g]: open the announce slot for a spell cast DURING resolution
    /// — the same [CR#601.2a] move-to-stack, but from `object`'s current
    /// `origin` zone (Chandra's just-exiled card) under `caster`'s control,
    /// since the effect grants the permission ([CR#608.2g]). The opening shell
    /// of [`GameState::cast_as_effect_items`](crate::state::GameState); the
    /// shared announce items follow, but no priority tail ([CR#608.2g]).
    BeginCastFromResolution {
        object: crate::object::ObjectId,
        origin: deckmaste_core::Zone,
        caster: crate::player::PlayerId,
        /// [CR#118.9,702.35a]: an alternative base cost the granting effect
        /// supplied ("cast it by paying its madness cost"), paid RATHER THAN
        /// the card's mana cost; `None` for a plain `Cast(<ref>)`.
        alternative_cost: Option<deckmaste_core::Cost>,
        /// The resolving instruction queue beneath this speculative announce.
        /// Announcement decline restores this queue, never the unfinished
        /// announce tail that follows this work item.
        resume: Arc<[WorkItem]>,
        /// A `May(Cast)` branch that runs only if its announcement payment is
        /// declined. The successful `if_did` branch rides after `SpellCast`
        /// in the ordinary announce schedule instead.
        if_not: Option<(
            Arc<deckmaste_core::Instruction>,
            Box<crate::stack::ExecutionFrame>,
        )>,
    },
    /// [CR#602.2a,602.2b]: stage a non-mana activated ability — snapshot the
    /// ability text + source LKI into the announce slot. The shared
    /// `AnnounceTargets`/`PayCost` items follow; `AbilityActivated` promotes
    /// it onto the stack.
    BeginActivate {
        object: crate::object::ObjectId,
        ability: usize,
    },
    /// [CR#601.2b,602.2b,700.2]: choose a modal spell or ability's modes.
    /// Runs before optional costs, X, targets, and payment; a nonmodal
    /// announce is a no-op.
    AnnounceModes,
    /// [CR#601.2b]: announce the in-flight spell's tagged OPTIONAL
    /// additional costs (kicker/multikicker, [CR#702.33a,702.33c]) — one
    /// `YesNo` per declared `CostOption` row, starting at `index`; a
    /// repeatable row re-offers after each yes. No-op for activations and
    /// spells with no rows.
    AnnounceOptionalCosts { index: usize },
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
    OpenPayment,
    /// Resume the payment command boundary after every event and ordinary
    /// subdecision produced by one fulfillment has completed.
    FinishPaymentFulfillment(crate::payment::IouId),
    /// Resolve a submitted activated mana ability without putting it on the
    /// stack, using the ordinary announced modes, targets, and effect runner.
    BeginManaAction(crate::player::ManaActionId),
    /// Close a stackless mana action after its effect and caused immediate
    /// work have drained, then restore its containing payment prompt.
    FinishManaAction(crate::player::ManaActionId),
    /// Resolve a lowering-classified triggered mana ability at the causing
    /// event's immediate queue rather than noting it for stack placement.
    ResolveTriggeredMana {
        source: crate::object::ObjectSource,
        ability: usize,
        triggered: Arc<deckmaste_core::TriggeredAbility>,
        controller: crate::player::PlayerId,
        bindings: crate::trigger::TriggerBindings,
    },
    /// Emit the aggregate production fact for one stackless triggered mana
    /// action after its effect drains.
    FinishTriggeredMana {
        action: crate::player::ManaActionId,
        source: crate::lki::LkiSnapshot,
        controller: crate::player::PlayerId,
    },
    /// Close a triggered mana action after its production fact and anything it
    /// immediately caused have drained.
    CompleteTriggeredMana(crate::player::ManaActionId),
    /// [CR#705.1]: a resolving `FlipCoins` — draw `count` coins for `player`
    /// from the seeded rng and emit the `CoinFlipped` batch. `called`
    /// ([CR#705.2]) routes each coin through a `CallFlip` decision first
    /// (the flipper calls heads or tails and wins or loses); an uncalled
    /// flip draws directly and no player wins or loses.
    FlipCoins {
        player: crate::player::PlayerId,
        count: deckmaste_core::Uint,
        called: bool,
    },
    /// [CR#706.1]: a resolving `RollDice` — draw `count` naturals in
    /// `1..=sides` for `player` from the seeded rng and emit the `DieRolled`
    /// batch. `result = natural`: the modifier pipeline
    /// ([CR#706.2a..706.2b]) is the engine-replace-roll ticket's.
    RollDice {
        player: crate::player::PlayerId,
        count: deckmaste_core::Uint,
        sides: deckmaste_core::Uint,
    },
    /// [CR#106.1b]: a resolving `AddMana` whose production is a choice ("any
    /// color", "{W} or {U}") — surface a `ChooseManaColor` decision for
    /// `player` to pick one of `options`.
    ChooseManaColor {
        player: crate::player::PlayerId,
        options: Vec<deckmaste_core::ColorOrColorless>,
        amount: deckmaste_core::Uint,
        riders: Vec<deckmaste_core::ManaRider>,
        provenance: crate::player::ManaProvenance,
    },
    /// [CR#106.1b]: a resolving `AddMana` whose production is a choice among
    /// multi-symbol runs (the filterland "{W}{W}, {W}{U}, or {U}{U}") —
    /// surface a `ChooseManaMode` decision for `player` to pick one of
    /// `options` (each a run of mana). The chosen run's whole sequence lands.
    ChooseManaMode {
        player: crate::player::PlayerId,
        options: Vec<Vec<deckmaste_core::ColorOrColorless>>,
        amount: deckmaste_core::Uint,
        riders: Vec<deckmaste_core::ManaRider>,
        provenance: crate::player::ManaProvenance,
    },
    /// Legacy mid-resolution mana-toll work. New `May(Pay(cost))` effects use
    /// a full optional payment frame; this remains while older announcement
    /// and direct-toll callers migrate to that protocol.
    TollMana {
        player: crate::player::PlayerId,
        cost: deckmaste_core::ManaCost,
        subject: crate::object::ObjectId,
    },
    /// [CR#707.10c,115.7d]: re-target a COMMITTED stack entry — surface a
    /// `Retarget` decision whose per-slot legal set is the fresh
    /// legal candidates PLUS the current target (leaving a slot unchanged
    /// is always allowed, even when the current target is illegal; a
    /// CHANGED slot must be legal).
    Retarget {
        player: crate::player::PlayerId,
        entry: crate::object::ObjectId,
    },
    /// [CR#608.2c,608.2d]: a resolving `ChooseValue(who, Number, key)` —
    /// surface a resolution-time NUMBER choice for `player`, whose answer is
    /// stored in the armed activation register for a later indexed read. The
    /// verb (`&self`) can only schedule this; the handler (`&mut self`)
    /// surfaces the decision.
    ChooseNoteNumber {
        player: crate::player::PlayerId,
        key: deckmaste_core::Ident,
    },
    /// A resolving `ChooseValue(who, CardName, key)` choice.
    ChooseNoteCardName {
        player: crate::player::PlayerId,
        key: deckmaste_core::Ident,
    },
    /// Resolve the named committed stack object ([CR#608]). Reads `self.stack`.
    Resolve(crate::object::ObjectId),
    /// Interpret one `Instruction` node against a resolution frame
    /// ([CR#608.2]).
    RunEffect {
        effect: Arc<deckmaste_core::Instruction>,
        frame: crate::stack::ExecutionFrame,
    },
    /// [CR#611.2a,701.19c]: install the instruction-scoped "can't be
    /// regenerated" rider for the destroy that runs IMMEDIATELY next. Minted
    /// only by `Sequentially` lowering when a `ForThisEvent`
    /// `Cant(Regenerate)` child folds onto its preceding destroy sibling —
    /// scheduled just before that sibling's `RunEffect` so the subjects are set
    /// when the destroy's occurrence is applied, then cleared at the end of
    /// that `apply_occurrence`. `no_regen` is the rider's subjects resolved
    /// to ids at lowering time.
    InstallRiders {
        no_regen: Vec<crate::object::ObjectId>,
    },
    /// [CR#401.7]: reposition a card ALREADY in its owner's library to an
    /// anchored end of that same library — a same-zone move that is NOT a zone
    /// change ([CR#400.7]): the `ObjectId` is preserved, no `ZoneChange` fires
    /// and no zone-change trigger sees it (scry never removes a card from the
    /// library, [CR#701.22a]). `offset` is the count from `end`. When a
    /// post-pick arrange scope is armed the landing is recorded so the
    /// finalizer can order the pile ([CR#401.4]).
    RepositionLibrary {
        object: crate::object::ObjectId,
        end: LibraryEnd,
        offset: deckmaste_core::Uint,
    },
    /// [CR#401.4]: after an `Each`/`MoveGroup` batch of ordered-library
    /// landings, surface one arrange decision per pile of more than one card
    /// (owner/controller orders it), then reorder those cards in the library.
    ArrangePiles,
    /// [CR#401.4]: arrange a `MoveGroup`'s landing in an ordered library
    /// position — the `count` cards now at `end` of `library_owner`'s library
    /// (freshly reminted by the group move). `ChosenOrder`/`AnyOrder` surface
    /// an arrange decision (`arranger` orders them); `RandomOrder` shuffles
    /// the pile ([MTR 3.10]); `SameOrder` leaves it. Scheduled directly
    /// after the group's move batch.
    ArrangeGroupLanding {
        arranger: crate::player::PlayerId,
        arrangement: deckmaste_core::Arrangement,
        library_owner: crate::player::PlayerId,
        end: LibraryEnd,
        count: deckmaste_core::Uint,
    },
    /// [CR#616.1]: the finalization watcher planted alongside a future `Act`
    /// window. After the window resolves (the action passed, was redirected, or
    /// was replaced away) this observes the resolution-scoped cursors in
    /// `mark` for `watch`'s characteristic committed change and, on a hit,
    /// emits the committed PAST `Act` fact (`act` with `committed: true`) —
    /// recording it and firing its "whenever you …" triggers. A miss records
    /// nothing (a replaced-away mill, a regenerated destroy, an empty-library
    /// draw). `act` is the future window event; the handler flips its phase
    /// marker.
    FinalizeAct {
        act: GameEvent,
        watch: FinalizeWatch,
        mark: FinalizeMark,
    },
    /// Publish one runtime-produced number into the instruction's own region
    /// register. `mark` bounds the read to facts caused after this instruction
    /// began, so nested and sibling instructions cannot share or overwrite a
    /// global magnitude slot.
    WriteMagnitude {
        activation: crate::activation::ActivationId,
        dest: deckmaste_core::DefId,
        source: MagnitudeSource,
        mark: usize,
    },
}
