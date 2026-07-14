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

/// One unit of engine work. `step()` pops exactly one; handlers schedule
/// follow-ups at the agenda *front*, ahead of previously queued work.
#[derive(Debug, Clone, PartialEq, Eq)]
#[expect(
    clippy::large_enum_variant,
    reason = "agenda work items are consumed one-at-a-time, not stored in bulk; boxing the effect payload would add allocation churn on the hot step() path"
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
    },
    /// [CR#602.2a,602.2b]: stage a non-mana activated ability — snapshot the
    /// ability text + source LKI into the announce slot. The shared
    /// `AnnounceTargets`/`PayCost` items follow; `AbilityActivated` promotes
    /// it onto the stack.
    BeginActivate {
        object: crate::object::ObjectId,
        ability: usize,
    },
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
    PayCost,
    /// [CR#701.9b]: a resolving discard — surface a `DiscardCards` decision
    /// for `player` to choose which `count` cards to discard (clamped to the
    /// hand size when the item applies; an empty hand surfaces nothing).
    DiscardCards {
        player: crate::player::PlayerId,
        count: deckmaste_core::Uint,
    },
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
    /// [CR#701.9b]: a resolving RANDOM discard — no decision (no choice
    /// exists); uniformly sample `count` distinct cards from `player`'s hand
    /// (clamped when the item applies — the hand may change before then) and
    /// emit the same Hand→Graveyard batch a chosen discard emits.
    DiscardRandom {
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
    /// [CR#106.1b]: a resolving `AddMana` whose production is a choice among
    /// multi-symbol runs (the filterland "{W}{W}, {W}{U}, or {U}{U}") —
    /// surface a `ChooseManaMode` decision for `player` to pick one of
    /// `options` (each a run of mana). The chosen run's whole sequence lands.
    ChooseManaMode {
        player: crate::player::PlayerId,
        options: Vec<Vec<deckmaste_core::ColorOrColorless>>,
        amount: deckmaste_core::Uint,
        riders: Vec<deckmaste_core::ManaRider>,
    },
    /// [CR#118.12a]: a mid-resolution mana toll — a `MustPay`/`MayPay`
    /// continuation's `Mana(...)` cost component, paid by `player` from
    /// their pool. Surfaces a `PayMana` decision; `subject` is the
    /// resolving ability's source (a `SpendOnly` rider judges it). A ward
    /// toll's cost arrives here already priced ([CR#702.21b] — `{X}`
    /// resolved through `where_x` at `MustPay` execution).
    TollMana {
        player: crate::player::PlayerId,
        cost: deckmaste_core::ManaCost,
        subject: crate::object::ObjectId,
    },
    /// [CR#707.10c,115.7d]: re-target a COMMITTED stack entry — surface a
    /// `ChooseNewTargets` decision whose per-slot legal set is the fresh
    /// legal candidates PLUS the current target (leaving a slot unchanged
    /// is always allowed, even when the current target is illegal; a
    /// CHANGED slot must be legal).
    ChooseNewTargets {
        player: crate::player::PlayerId,
        entry: crate::object::ObjectId,
    },
    /// [CR#608.2c,608.2d]: a resolving `ChooseAndNote(key, NotedKind::Number)`
    /// — surface a resolution-time NUMBER choice for `player`, whose answer is
    /// stored in `resolution_notes[key]` (`Count::Noted` reads it back). The
    /// verb (`&self`) can only schedule this; the handler (`&mut self`)
    /// surfaces the decision.
    ChooseNoteNumber {
        player: crate::player::PlayerId,
        key: deckmaste_core::Ident,
    },
    /// A resolving `ChooseAndNote(key, CardName)` choice.
    ChooseNoteCardName {
        player: crate::player::PlayerId,
        key: deckmaste_core::Ident,
    },
    /// [CR#607.2a,608.2d]: a resolving `ChooseAndNote(key, NotedKind::Objects)`
    /// — surface a `ChooseObjects` pick for `player`; the submit records the
    /// chosen objects into the `noted` product group under `key` (read back by
    /// `Selection::AmongNoted`). The grammar carries no narrowing predicate,
    /// so the candidate domain is derived by the handler.
    ChooseNoteObjects {
        player: crate::player::PlayerId,
        key: deckmaste_core::Ident,
    },
    /// Resolve the named committed stack object ([CR#608]). Reads `self.stack`.
    Resolve(crate::object::ObjectId),
    /// Interpret one `OneShotEffect` node against a resolution frame
    /// ([CR#608.2]).
    RunEffect {
        effect: Box<deckmaste_core::OneShotEffect>,
        frame: crate::stack::Frame,
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
    /// Opens a `Noting` collection window ([CR#607.2a] linkage — the
    /// fact-backed product group): resets `key`'s group and pushes it onto
    /// the noting stack, so every enacted `ZoneChanged` fact until the
    /// matching `EndNote` joins the group. Scheduled around the noted
    /// effect's `RunEffect` by `OneShotEffect::Noting`.
    BeginNote { key: deckmaste_core::Ident },
    /// Closes the innermost `Noting` collection window ([CR#607.2a]).
    EndNote,
    /// [CR#401.7]: reposition a card ALREADY in its owner's library to an
    /// anchored end of that same library — a same-zone move that is NOT a zone
    /// change ([CR#400.7]): the `ObjectId` is preserved, no `ZoneChanged` fires
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
}
