use std::sync::Arc;

use crate::Count;
use crate::CounterRef;
use crate::Expansion;
use crate::Reference;
use crate::Selection;
use crate::SupportsMacros;
use crate::TokenSpec;
use crate::mana::ManaProduction;

/// A position within a library ([CR#401.7]): an offset counted from the top or
/// from the bottom. `FromTop(0)` is the very top, `FromBottom(0)` the very
/// bottom; larger offsets index inward. The Idris `Anchor = FromTop Count |
/// FromBottom Count`.
#[derive(Debug, Clone, PartialEq, Eq, Hash, SupportsMacros)]
pub enum Anchor {
    /// `n` cards from the top (`0` = on top).
    FromTop(Count),
    /// `n` cards from the bottom (`0` = on the bottom) — the placement no
    /// from-top index could name without first knowing the library's size.
    FromBottom(Count),
}

/// Where a relocation puts an object ([CR#400.7]). A plain zone (the object's
/// *owner's* graveyard/hand/library, the shared exile, the battlefield), or the
/// library at an anchored position ([CR#401.7]). The Idris `Destination =
/// ToZone Zone | ToLibrary Anchor`; this unifies `Move`'s zone change and the
/// former `PutInLibrary`'s library-position placement into one notion (and adds
/// bottom-of-library by construction).
///
/// A bare zone name reads as [`Destination::Zone`] — the
/// `#[macro_ron(flatten)]` marker lifts [`Zone`](crate::Zone)'s variant names
/// into `Destination`'s dispatch — so `Move(This, Graveyard)` is unchanged; the
/// library form is `Move(This, Library(FromTop(0)))`.
///
/// `exclude(Library, Stack)` narrows the flattened set: the library is an
/// ORDERED zone, so it is a destination only AT a position (the dedicated
/// `Library(Anchor)` form is the single canonical spelling — a bare `Library`
/// is not a valid destination), and the stack is never a `Move` destination
/// ([CR#401.7,405.1]). Both are rejected on read, mirroring the Idris
/// `DestinationOk` gate (`ToZone Library` / `ToZone Stack` = `Void`); without
/// the exclusion a library destination would be expressible two ways
/// (`Destination::Zone(Zone::Library)` vs `Destination::Library(_)`), a
/// dual-representation smell.
#[derive(Debug, Clone, PartialEq, Eq, Hash, SupportsMacros)]
pub enum Destination {
    /// A plain zone change ([CR#400.7]) — written as the bare zone name. The
    /// ordered library and the stack are excluded (see the type docs).
    #[macro_ron(flatten, exclude(Library, Stack))]
    Zone(crate::Zone),
    /// The library at an anchored position ([CR#401.7]).
    Library(Anchor),
}

/// An entry rider on a relocation/creation verb: how the object arrives on the
/// battlefield ([CR#614.12] — effects that modify how a permanent enters). The
/// rider list rides [`Action::Move`]/[`Action::MoveGroup`]/
/// [`PlayerAction::Create`] (and the player-agent `Move`); it is
/// **battlefield-only** — riders on any non-battlefield destination are
/// ill-formed (a card in a graveyard has no tapped/attacking state to arrive
/// in, [CR#110.5,614.12]) and rejected by the Idris re-emit gate.
#[derive(Debug, Clone, PartialEq, Eq, Hash, SupportsMacros)]
pub enum EnterRider {
    /// "enters tapped" ([CR#603.6d] wording; applied via [CR#614.12]) — the
    /// Path-to-Exile-style search-tapped and Rampant Growth's "onto the
    /// battlefield tapped".
    Tapped,
    /// Enters face down ([CR#708]) — the manifest/morph arrival state.
    FaceDown,
    /// Enters under the named player's control instead of the default
    /// ([CR#110.2a] — the instructing player's, unless the effect states
    /// otherwise).
    UnderControlOf(Reference),
    /// "under its owner's control" ([CR#110.2a] override by owner) — the
    /// blink/return-at-end-of-turn wording, spelled without naming a player.
    UnderOwnersControl,
    /// Enters attacking ([CR#508.4]) — Ninjutsu's "tapped and attacking",
    /// Myriad/Encore. The optional reference names WHOM it attacks; `None`
    /// leaves the [CR#508.4] controller choice open.
    Attacking(Option<Reference>),
    /// "with N [kind] counters on it" ([CR#614.12,122.1]) — the reanimation/
    /// Otherworldly-Journey "+1/+1 counter on it" arrival.
    WithCounters(CounterRef, Count),
    /// "enters as a copy of [source]" ([CR#707.5]) — a Clone-style arrival:
    /// the object becomes a copy of `source` AS it enters, never entering
    /// first and copying a beat later ([CR#707.5]). Carries the shared
    /// [`crate::CopySpec`] payload (source + "except" exceptions,
    /// [CR#707.9]) like the other three copy delivery sites
    /// ([`crate::TokenSpec::Copy`], [`PlayerAction::CastCopy`], and the
    /// becomes-a-copy [`crate::continuous::StaticEffect::BecomesCopy`]).
    /// This rider is a layer-1a copy INPUT only ([CR#613.2]): applying it —
    /// deriving and installing the copiable values — is the
    /// `engine-layers-1-copy-facedown-text` seam in
    /// `deckmaste_engine::layer::base_values`; consuming it here (the
    /// rider-application step on `Move`/`Create`) is a documented fizzle,
    /// never a panic.
    AsCopy(crate::CopySpec),
}

/// How a GROUP landing in an ordered position is arranged ([CR#401.4] — the
/// owner may arrange simultaneous arrivals; effects fix or randomize that).
/// Rides [`Action::MoveGroup`].
#[derive(Debug, Clone, PartialEq, Eq, Hash, SupportsMacros)]
pub enum Arrangement {
    /// The named player chooses the order ("in the order of your choice") —
    /// the chooser is explicit ([CR#401.4]).
    ChosenOrder(Reference),
    /// "in any order" — the cards' owner arranges them, the [CR#401.4]
    /// default for a simultaneous group (Brainstorm's "on top of your library
    /// in any order").
    AnyOrder,
    /// The group keeps the order it was selected in ("in the same order").
    SameOrder,
    /// "in a random order" ([CR#401.4] — the order is randomized, not
    /// chosen).
    RandomOrder,
}

/// An intrinsic game verb ([CR#700,701]) whose **agent is the source object or
/// the effect itself**, not a player: the source deals the damage, the effect
/// destroys/returns the object. Player-performed verbs (draw, sacrifice, …)
/// carry an explicit agent and live on [`PlayerAction`], reached through `By`.
///
/// Authored RON writes a bare [`PlayerAction`] in an effect slot
/// (`Tap(This)`, `Sacrifice(This)`) and it reads as `By(Reference::You, …)` —
/// the implicit-you default, declared by `By`'s `#[macro_ron(embed)]` marker
/// with `#[macro_ron(default = "Reference::You")]` on the agent field, via
/// the macro layer's `embeds_untagged` hook (`Action` is registered with
/// `.embeds_untagged()`). An explicit different agent is written
/// `By(It, Sacrifice(This))` and read natively. Both serde
/// impls are generated by `#[derive(SupportsMacros)]` to carry that: the
/// reader tries `Action`'s own variants first, then falls through to a
/// `PlayerAction`; the writer emits the bare `PlayerAction`.
#[derive(Debug, Clone, PartialEq, Eq, Hash, SupportsMacros)]
pub enum Action {
    /// Deal an amount of damage to the patient object or player ([CR#120.1]).
    /// Fields read in printed-sentence order: `source` deals `amount` to
    /// `target` — `DealDamage(This, 3, It)` for Lightning Bolt. `source` is the
    /// **dealer** — the object whose damage this is; it is **required** and
    /// always spelled (`This` for the ability's source object / the resolving
    /// spell, the implicit agent). An explicit non-`This` source expresses
    /// redirected/arbitrary-source damage — e.g. each half of a fight, where a
    /// creature deals damage equal to its power to the other ([CR#701.14a]).
    DealDamage(Reference, Count, Reference),
    /// Counter the referenced spell or ability on the stack ([CR#701.6a]) — a
    /// countered spell moves to its owner's graveyard; a countered ability
    /// simply ceases. "Can't be countered" is deontic-layer territory, not
    /// part of the verb.
    Counter(Reference),
    /// Turn a transforming double-faced permanent to its other face
    /// ([CR#701.27a]). No-op on a non-DFC permanent ([CR#701.27c]) or when the
    /// destination face is an instant/sorcery ([CR#701.27d]). Convert
    /// ([CR#701.28a]) is the same operation. Identity is preserved — a
    /// transform does not remint ([CR#712.18]).
    Transform(Reference),
    /// The referenced object ceases to exist ([CR#704.5d,707.10a]) — no zone
    /// move, no card left behind. Today the only reachable target is a
    /// card-less stack copy (`StackEntry.copy`): a copy of a spell stranded
    /// anywhere other than the stack ceases this way ([CR#707.10a]). This is
    /// the data-usable shape the copy-cease SBA (`sba.rs`) speaks through —
    /// its scan still walks `state.stack` natively (the generic `SbaRule`
    /// domain is battlefield objects only, `sba_rule.rs`'s doc), but the
    /// removal it emits now names its own verb rather than borrowing
    /// `Counter`'s. A reference that isn't on the stack is a no-op (bad
    /// authoring/state drift fizzles, never panics).
    Cease(Reference),
    /// Attach `what` to `to` ([CR#701.3a..701.3b]) — the one verb the whole
    /// attachment family shares (Equipment, Auras, Fortifications). The
    /// attachment RELATION (storage; the illegal-attachment SBAs,
    /// [CR#704.5m..704.5n]) is engine machinery.
    Attach { what: Reference, to: Reference },
    /// Unattach the referenced attachment from its host ([CR#701.3d]) — the
    /// inverse of [`Attach`](Action::Attach). Used by Reconfigure's unattach
    /// ability and the illegal-attachment SBA's unattach path. No-op on a
    /// reference that isn't attached.
    Unattach(Reference),
    /// Move the referenced object to a [`Destination`] ([CR#400.7]) — a plain
    /// zone change (emits the future-form `ZoneChange`), NOT destruction (so
    /// indestructible does not apply, distinct from [`Action::destroy`])
    /// and NOT a sacrifice. A graveyard/hand/library destination is the
    /// object's *owner's*; exile is the shared exile zone. The destination
    /// is a bare zone name (`Move(This, Graveyard)` — the [CR#704.5m] Aura
    /// graveyard SBA) or the library at an anchor (`Move(This,
    /// Library(FromTop(0)))` — top of library, the former `PutInLibrary`;
    /// `Library(FromBottom(0))` — bottom, [CR#401.7]). This one verb
    /// subsumes the old `Move`/`PutInLibrary` split, and (`Move(_, Hand)`)
    /// the former dedicated `ReturnToHand` bounce verb — a hand destination
    /// is exactly as unremarkable as any other zone. The `riders` list
    /// ([`EnterRider`], default `[]`, omitted on write when empty) spells
    /// arrival state for a BATTLEFIELD destination — "onto the battlefield
    /// tapped / under its owner's control / with a +1/+1 counter
    /// on it" ([CR#614.12]); riders on any other destination are ill-formed
    /// (rejected by the Idris re-emit gate).
    ///
    /// The trailing `from` slot is an optional FIZZLE-GUARD ([CR#701.8a]):
    /// when present, the move commits only if the object is CURRENTLY in
    /// that zone — the zone precondition destroy/discard/mill state
    /// declaratively (a permanent no longer on the battlefield when destroy
    /// resolves, a card no longer in hand when the named-card discard
    /// resolves, [CR#701.8a,701.9a,701.17a]) — and a mismatch fizzles the
    /// move silently: no event, no fact, no trigger, never a panic
    /// (authoring/state-drift never crashes the engine). Default `None`
    /// (unguarded, the common case) is omitted on write.
    Move(
        Reference,
        Destination,
        #[macro_ron(default = "crate::empty_arc()")] Arc<[EnterRider]>,
        #[macro_ron(default = "None")] Option<crate::Zone>,
    ),
    /// Move a GROUP to a destination as one event, with an [`Arrangement`]
    /// fixing how the simultaneous arrivals are ordered ([CR#401.4]) — the
    /// order only EMERGES for a group landing in an ordered position, so the
    /// group verb carries it while single [`Move`](Action::Move) does not.
    /// Brainstorm's "put two cards … on top of your library in any order" =
    /// `MoveGroup(group: That, arrangement: AnyOrder, to:
    /// Library(FromTop(0)))`. `riders` as on [`Move`](Action::Move)
    /// (battlefield-only, [CR#614.12]). A struct variant: four fields exceed
    /// the `SupportsMacros` tuple arity (mirrors
    /// [`CostComponent::TapTotal`](crate::CostComponent)).
    MoveGroup {
        group: Selection,
        arrangement: Arrangement,
        to: Destination,
        #[serde(default, skip_serializing_if = "crate::slice_is_empty")]
        riders: Arc<[EnterRider]>,
    },
    /// The patient object comes under the control of the referenced player
    /// — a one-shot control TRANSITION ([CR#701.12b]; a control change is
    /// never a zone change, [CR#613.1b]). A same-controller grant is a
    /// no-op ("the exchange effect does nothing", [CR#701.12b]). The
    /// exchange-family primitive: restricted to the exchange macros'
    /// bodies — DURATION-bounded control effects
    /// ("gain control until end of turn") are the continuous layer-2 form,
    /// not this verb.
    GainControl(Reference, Reference),
    /// Add an extra phase of the given kind to the referenced player's turn,
    /// directly after the current phase ([CR#500.8]).
    ExtraPhase(crate::PhaseKind, Reference),
    /// "It becomes day." ([CR#731.1]) — the game gains the day designation.
    BecomeDay,
    /// "It becomes night." ([CR#731.1]).
    BecomeNight,
    /// "The Ring tempts [player]" ([CR#701.54a]) — the game tempts the
    /// referenced player (Ring-bearer choice and The Ring emblem are the
    /// engine's); a footing verb: the shape lands here, execution later.
    TheRingTempts(Reference),
    /// Move counters from one object onto another ([CR#122] — counters move
    /// object→object as a single operation, distinct from a separate
    /// remove-then-put). The [`CounterSpec`](crate::CounterSpec) names a
    /// specific kind+count (Power Conduit / Leech Bonder) or `AllKinds`
    /// (Ozolith / Fate Transfer — every counter of every kind at once, the
    /// case single-kind remove+put can't reach atomically); `from`/`to` are
    /// the source and destination objects.
    MoveCounters(crate::CounterSpec, Reference, Reference),
    /// Register a floating replacement effect ([CR#614.3]) — "the next time …"
    /// shields (regeneration, one-shot prevention). The protected permanent is
    /// the object bound as `That` by the enclosing `With` — the shield freezes
    /// that resolved binding at creation (an LKI snapshot of identity), so a
    /// shield is authored `With(binder: TheRef(<subject>), body:
    /// CreateReplacement(…))`; there is no authored `subject:` field.
    /// `one_shot` consumes the shield on first use ([CR#614.3]).
    /// [CR#701.19a,614.8]
    CreateReplacement {
        replacement: Arc<crate::replacement::Replacement>,
        duration: crate::continuous::Duration,
        one_shot: bool,
    },
    /// A named keyword action ([CR#701]) — a verb `name` ([`crate::VerbName`])
    /// whose meaning IS its `body` effect, run when this resolves (the discard
    /// IS the Hand → Graveyard move, named "Discard"). The keyword-action
    /// macros (`Destroy`, `Scry`, `Discard`, …) desugar to a `Composite` so
    /// there are no bespoke `Scry`/`Surveil` verbs; the engine dispatches
    /// on the NAME plus the body's shape, not a typed atom. Mirrors the
    /// Idris `Composite : KeywordActionSpec b -> OneShotEffect b -> Action
    /// b` and the [`KeywordAbility::Composite`](crate::KeywordAbility) `{
    /// name, abilities }` precedent (a struct variant, read all-named).
    /// Resolving runs `body`, then (when the body actually acts — scry 0
    /// does nothing, [CR#701.22b]) commits the present-tense `Act`
    /// name-fact a "whenever you scry/surveil/discard" trigger reads.
    /// `body` is boxed to break the `Action` → `OneShotEffect` →
    /// `Action` size cycle.
    Composite {
        name: crate::VerbName,
        body: Arc<crate::OneShotEffect>,
    },
    /// A named player performs the [`PlayerAction`] ([CR#608.2]). `By(You, …)`
    /// is the implicit-you default and is written bare in RON.
    #[macro_ron(embed)]
    By(
        #[macro_ron(default = "Reference::You")] Reference,
        PlayerAction,
    ),
}

/// A verb a **player** performs, carrying an explicit agent via
/// [`Action::By`]. Authored bare in effect slots (the agent defaults to
/// `You`); a cost (`CostComponent::Do`) holds a bare `PlayerAction` whose
/// agent is implicitly the payer.
///
/// Both serde impls are generated by `#[derive(SupportsMacros)]`: `Expanded`
/// writes the invocation back.
#[derive(Debug, Clone, PartialEq, Eq, Hash, SupportsMacros)]
pub enum PlayerAction {
    /// Gain an amount of life ([CR#119.3]).
    GainLife(Count),
    /// Lose an amount of life — pay-life when in a cost ([CR#119.3]).
    LoseLife(Count),
    /// Add mana to the player's mana pool ([CR#106.4]) — the production
    /// carries optional unit riders ([CR#106.6]).
    AddMana(Count, ManaProduction),
    /// Create a number of token permanents ([CR#111.1,701.7]). The trailing
    /// `riders` list ([`EnterRider`], default `[]`, omitted when empty) spells
    /// arrival state — "create a … token tapped and attacking" ([CR#508.4]);
    /// a created token always enters the battlefield, so every rider is legal
    /// here ([CR#614.12]).
    Create(
        Count,
        TokenSpec,
        #[macro_ron(default = "crate::empty_arc()")] Arc<[EnterRider]>,
    ),
    /// Sacrifice the referenced permanent ([CR#701.21]).
    Sacrifice(Reference),
    /// A player-performed relocation to a [`Destination`] ([CR#400.7]) — the
    /// player-agent twin of [`Action::Move`], reachable from a cost
    /// (`CostComponent::Do`). Exiling is a pure zone move ([CR#701.13]), so it
    /// has no dedicated verb: cost-side/self exile is `Move(This, Exile)`
    /// (Scavenge), and bottom-of-library tuck, etc., ride the same verb.
    /// `riders` as on [`Action::Move`] (battlefield-only, [CR#614.12]).
    Move(
        Reference,
        Destination,
        #[macro_ron(default = "crate::empty_arc()")] Arc<[EnterRider]>,
    ),
    /// "[Player] ventures into the dungeon" ([CR#701.49a]) — a footing verb:
    /// the venture-marker/dungeon machinery is the engine's; the agent rides
    /// [`Action::By`].
    VentureIntoDungeon,
    /// Tap the referenced object ([CR#701.26a]).
    Tap(Reference),
    /// Untap the referenced object ([CR#701.26b]).
    Untap(Reference),
    /// "You get an emblem with [abilities]" — a command-zone object that
    /// never touches the battlefield (rules-taxonomy §6: a degenerate
    /// token definition; [CR#114.1,114.4]).
    GetEmblem(Arc<[crate::Ability]>),
    /// "[Player] gets the named designation" ([CR#702.131c] — the city's
    /// blessing; the generic player-scope grant verb). v1 handles the
    /// **player-scope flag** case only; single-holder designations (monarch /
    /// the initiative — they evict the prior holder) and object-scope grants
    /// (goad, suspected) are seams the verb does not yet cover.
    GetDesignation(crate::Ident),
    /// A resolution choice stored under a note key ([CR#608.2d] choice +
    /// [CR#607.2] slot): "choose a color" and kin.
    ChooseAndNote(crate::Ident, crate::NotedKind),
    /// Put a copy of the referenced spell on the stack ([CR#707.10] — a
    /// copy on the stack, NOT casting one; [CR#707.12] casting rides the
    /// 601 pipeline).
    CopySpell(Reference),
    /// "Cast a copy of [source]" ([CR#707.12]) — NOT `CopySpell`'s
    /// stack-copy: this follows the full [CR#601.2a..601.2h] casting
    /// pipeline, created in `source`'s own zone and cast while another spell
    /// or ability resolves ([CR#707.12]), so it passes through legality,
    /// costs, and targeting like any other cast rather than skipping
    /// straight to the stack. The engine's resolve arm currently fizzles
    /// without producing events; it never panics.
    CastCopy(crate::CopySpec),
    /// "[Player] may cast [the referenced card]" as an effect ([CR#608.2g]) —
    /// the resolution-time cast primitive. The named player casts the
    /// referenced object by following the [CR#601.2a..601.2i] casting steps
    /// (the same pipeline `legal.rs`/`cast.rs` drive for a hand cast), EXCEPT
    /// no player receives priority after it's cast: the cast spell becomes the
    /// topmost object on the stack and the currently-resolving spell or ability
    /// continues to resolve ([CR#608.2g]). The effect GRANTS the permission
    /// ([CR#608.2g] — "specifically instructs or allows"), so this cast
    /// bypasses the normal timing/zone [`DeonticAction::Cast`] gate for the
    /// referenced card; the card is moved to the stack "from where it is"
    /// ([CR#601.2a]). The referenced card is bound by the surrounding effect;
    /// `May { effect: Cast(<that card>), if_not: <else> }` offers its "yes"
    /// branch only when a legal, payable cast
    /// exists ([CR#608.2g] — the offer is empty otherwise, so the `if_not`
    /// branch runs). A reference that resolves to no castable object
    /// fizzles (authoring mistakes never crash the engine).
    ///
    /// The trailing slot is an optional ALTERNATIVE COST ([CR#118.9,702.35a]):
    /// when present, the cast pays this cost RATHER THAN the card's mana cost —
    /// madness's "cast it by paying its madness cost" is `Cast(That(Card),
    /// [Mana(…)])`. A trailing-default tuple slot preserves bare
    /// `Cast(That(Card))` serialization, with `SinglePlusDefault`
    /// distinguishing the one-required-plus-one-default tuple from a
    /// newtype.
    Cast(
        Reference,
        #[macro_ron(default = "None")] Option<crate::Cost>,
    ),
    /// `by` picks new targets for the stack object `of`, bound by its
    /// original targetspec ([CR#115.7d,707.10c] — Bolt Bend, Redirect,
    /// copy-with-new-targets). Each target slot may be LEFT UNCHANGED even
    /// if the current target is illegal; a CHANGED slot must pick a legal
    /// target ([CR#707.10c]).
    ChooseNewTargets { of: Reference, by: Reference },
    /// Flip that many coins ([CR#705.1]); the win/loss result rides the
    /// pushed `amountAnte` antecedent, like `RollDice`. `called` splits
    /// [CR#705.2]'s two kinds: `true` = the flipper calls heads/tails and
    /// wins or loses the flip; `false` = the effect reads only
    /// heads/tails and no player wins or loses.
    FlipCoins(Count, bool),
    /// Roll that many dice with the given number of sides ([CR#706.1]);
    /// an IGNORED roll is considered to have never happened ([CR#706.6]).
    RollDice(Count, crate::Uint),
    /// Roll the (Planechase) planar die as a special action ([CR#901.9]).
    /// NO numeric result ([CR#901.9d]) — unlike `RollDice`/`FlipCoins` this
    /// introduces nothing for a later `ThatMany` read.
    RollPlanarDie,
    /// Put counters of the named kind on the referenced object/player
    /// ([CR#122.1] — counters go on objects AND players). The kind is a bare
    /// `CounterRef` (`PutCounters(~, P1P1Counter, 2)`), not a string.
    PutCounters(Reference, crate::CounterRef, Count),
    /// Remove counters of the named kind ([CR#122.1]; cost-eligible —
    /// "Remove a +1/+1 counter from this creature:").
    RemoveCounters(Reference, crate::CounterRef, Count),
    /// "[Player] wins the game" ([CR#104.2b]) — immediate on resolution,
    /// suppressed by a matching `CantWin` outcome gate ([CR#101.1]
    /// precedence; the last-player-standing win [CR#104.2a] never rides
    /// this verb and pierces gates).
    WinGame,
    /// "[Player] loses the game" ([CR#104.3e]) — immediate on resolution,
    /// suppressed by a matching `CantLose` outcome gate. A player who
    /// would win and lose simultaneously loses ([CR#104.3f]).
    LoseGame,
    /// Restart the game ([CR#727.1]) — a TERMINAL with explicit carryover
    /// (every card involved comes along, ownership unchanged [CR#727.2];
    /// effects may exempt cards [CR#727.5]; trailing instructions execute
    /// just before the new first untap step [CR#727.4]), never a state
    /// reset. The actor binding IS [CR#727.1a]: the restarting effect's
    /// controller starts the new game. The restarted game ends with no
    /// winner, loser, or draw. Subgames ([CR#729]) are a different,
    /// deferred concept (a context push, not a restart).
    RestartGame,
    /// Shuffle the actor's library ([CR#701.24a]) — an INFORMATION event
    /// too: order knowledge is destroyed for everyone, and revealed cards
    /// in the library stop being revealed and become new objects
    /// ([CR#701.20d]). Why library actions never rewind: [CR#733.1].
    Shuffle,
    /// Set the actor's life total to N ([CR#119.5]): resolves as a gain
    /// or loss of the necessary difference — triggers see the gain/loss,
    /// never a "set" event. Equal totals = no event (transition-only).
    SetLife(Count),
    /// Reveal the referenced object to all players ([CR#701.20a]); `to` names a
    /// player instead = "look at" — same operation shown to a subset
    /// ([CR#701.20e]). Revealing never moves the card ([CR#701.20b]).
    /// The reveal WINDOW (how long it stays shown, [CR#701.20a]) is the
    /// engine's effect-instance machinery, not grammar.
    Reveal {
        what: Reference,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        to: Option<Reference>,
    },
    /// Remove all marked damage from the referenced object ([CR#614.8]
    /// regeneration body; [CR#701.19a] heal clause). Applied in the
    /// regeneration `instead` body before tapping to restore the permanent to a
    /// clean damage state.
    RemoveDamage(Reference),
    /// A remembered `PlayerAction` macro invocation.
    #[macro_ron(expanded)]
    Expanded(Expansion<PlayerAction>),
}

impl Action {
    /// A player verb performed by the implicit "you" (`By(You, …)`) — the
    /// default agent a bare player verb reads as ([CR#608.2]). The explicit
    /// `By(other, …)` form names a different agent.
    #[must_use]
    pub fn by_you(action: PlayerAction) -> Action {
        Action::By(Reference::You, action)
    }

    /// `DealDamage` from the implicit source (`This`) — the common case, where
    /// the dealer is the ability's source object / the resolving spell. The
    /// enum form spells `source` explicitly (`DealDamage(This, amount,
    /// target)`); this ctor fills it in.
    #[must_use]
    pub fn deal_damage(target: Reference, amount: Count) -> Action {
        Action::DealDamage(Reference::This, amount, target)
    }

    /// `Move` to a plain zone — the common relocation (`Move(This,
    /// Graveyard)`), without spelling the `Destination::Zone` wrapper, the
    /// (empty) rider list, or the (absent) `from` fizzle-guard.
    #[must_use]
    pub fn move_to(what: Reference, zone: crate::Zone) -> Action {
        Action::Move(what, Destination::Zone(zone), [].into(), None)
    }

    /// `Move` guarded by the object's CURRENT zone ([CR#701.8a]) — moves
    /// `what` to `to` only if it is presently in `from`, else the whole move
    /// fizzles silently (no event, no panic). The zone precondition
    /// destroy/discard/mill state declaratively
    /// ([CR#701.8a,701.9a,701.17a]).
    #[must_use]
    pub fn move_if_in(what: Reference, from: crate::Zone, to: crate::Zone) -> Action {
        Action::Move(what, Destination::Zone(to), [].into(), Some(from))
    }

    /// "Destroy [permanent]" ([CR#701.8a]) — the keyword action as data: a
    /// [`Composite`](Action::Composite) named `"Destroy"` whose body IS the
    /// Battlefield → owner's-graveyard [`Move`](Action::Move). There is no
    /// bespoke `Destroy` verb; the engine reads the body facet ("→Graveyard")
    /// off this stored move and commits it atomically on the one `Act(Destroy)`
    /// event ([CR#616.1]). Authored via the `Destroy` macro so the card still
    /// writes "Destroy target creature".
    #[must_use]
    pub fn destroy(what: Reference) -> Action {
        Action::Composite {
            name: crate::VerbName::from("Destroy"),
            body: Arc::new(crate::OneShotEffect::Act(Action::move_to(
                what,
                crate::Zone::Graveyard,
            ))),
        }
    }

    /// One card of a mill ([CR#701.17a]) — the PER-UNIT keyword action a mill's
    /// `Batch` contains: a [`Composite`](Action::Composite) named `"Mill"`
    /// whose body describes the top slice of `who`'s library moving to
    /// their graveyard (a [`MoveGroup`](Action::MoveGroup) over
    /// [`TopOfLibrary`](crate::Selection::TopOfLibrary), the late-bound slice).
    /// `whose` rides the body's selection so the engine reads the performer off
    /// it. A whole mill is
    /// [`OneShotEffect::mill`](crate::OneShotEffect::mill) — `Batch(count,
    /// Act(Mill))` — and the engine commits the whole slice as ONE simultaneous
    /// batch of per-card, individually-redirectable moves ([CR#701.17a,616.1]),
    /// NOT the per-card sequence a draw is ([CR#121.2]).
    #[must_use]
    pub fn mill_one(who: Reference) -> Action {
        Action::Composite {
            name: crate::VerbName::from("Mill"),
            body: Arc::new(crate::OneShotEffect::Act(Action::MoveGroup {
                group: Selection::TopOfLibrary {
                    count: Count::Literal(1),
                    whose: who,
                },
                arrangement: crate::Arrangement::AnyOrder,
                to: Destination::Zone(crate::Zone::Graveyard),
                riders: [].into(),
            })),
        }
    }

    /// One card of a draw ([CR#121.1,121.2]) — the PER-UNIT keyword action a
    /// draw's `Batch` contains: a [`Composite`](Action::Composite) named
    /// `"Draw"` whose body DESCRIBES the top-of-library → Hand relocation (an
    /// [`Each`](crate::Each) over
    /// [`TopOfLibrary`](crate::Selection::TopOfLibrary)); `whose` rides the
    /// body's selection so the engine reads the performer off it. A whole draw
    /// is [`OneShotEffect::draw`](crate::OneShotEffect::draw) — `Batch(count,
    /// Act(Draw))`, `count` SEQUENTIAL single-card draws ([CR#121.2], each
    /// seeing prior state), UNLIKE mill's one simultaneous batch. The body is
    /// NOT the executor — the `Act(Draw)` apply binds the library top LATE and
    /// empty-checks BEFORE its move (an empty library loses the game,
    /// [CR#120.3,104.3c] — an executing body would silently no-op and MISS that
    /// loss); the stored body is the faithful render/re-emit facet only.
    #[must_use]
    pub fn draw_one(who: Reference) -> Action {
        Action::Composite {
            name: crate::VerbName::from("Draw"),
            body: Arc::new(crate::OneShotEffect::Each(crate::Each {
                binder: crate::Binder::Existing(Selection::TopOfLibrary {
                    count: Count::Literal(1),
                    whose: who,
                }),
                effect: Arc::new(crate::OneShotEffect::Act(Action::move_to(
                    Reference::It,
                    crate::Zone::Hand,
                ))),
            })),
        }
    }

    /// "`who` discards `count`" ([CR#701.9a]) — the keyword action as data:
    /// a [`Composite`](Action::Composite) named `"Discard"` whose body IS
    /// the executor (unlike draw/mill's flat-coordinate bodies): a
    /// [`With`](crate::With) choose-then-act step over `who`'s hand
    /// ([CR#701.9b] — the affected player chooses by default), reusing the
    /// GENERAL-purpose binder machinery every other card effect's choice
    /// rides. `random: true` swaps the `Choose` binder for
    /// [`Selection::Random`] over the same hand filter — no choice exists,
    /// so the engine samples the seeded rng instead of surfacing a
    /// decision. Either way the body's `Each` realizes the bound group as
    /// PER-CARD `Move`s, each recursively dispatched through THIS SAME
    /// `"Discard"` composite name (the bound single-move form,
    /// [`discard_what`](Action::discard_what)) — so "whenever a player
    /// discards a card" fires once per card, and a replacement (madness,
    /// [CR#702.35a]) reroutes ITS card only. Authored via the
    /// `Discard`/`Discards` macros so the card still writes "Discard two
    /// cards." / "Each opponent discards a card."
    #[must_use]
    pub fn discard(who: Reference, count: Count, random: bool) -> Action {
        let filter = discard_hand_filter(who.clone());
        let quantity = crate::Quantity::Range(Some(count.clone()), Some(count));
        let binder = if random {
            crate::Binder::Existing(Selection::Random(quantity, filter))
        } else {
            crate::Binder::Choose {
                quantity,
                filter,
                by: who,
            }
        };
        Action::Composite {
            name: crate::VerbName::from("Discard"),
            body: Arc::new(crate::OneShotEffect::With(crate::With {
                binder,
                body: Arc::new(crate::OneShotEffect::Each(crate::Each {
                    binder: crate::Binder::Existing(Selection::They),
                    effect: Arc::new(crate::OneShotEffect::Act(Action::discard_what(
                        Reference::It,
                    ))),
                })),
            })),
        }
    }

    /// "Discard [this card / that card]" ([CR#701.9a]) — the degenerate
    /// NO-CHOICE discard of a named card: cycling's "Discard this card" cost
    /// ([CR#702.29a]) and the bound "that player discards that card" form.
    /// A [`Composite`](Action::Composite) named `"Discard"` whose body IS the
    /// single Hand → owner's-graveyard [`Move`](Action::Move) — the destroy
    /// shape: the engine reads the body facet off the stored move and commits
    /// it atomically on the ONE dual-facet `Act(Discard)` event
    /// ([CR#616.1]), where madness's replacement bites ([CR#702.35a]). The
    /// discarding player is the card's own controller (the card is in that
    /// player's hand), so it rides the patient, not a separate `who` slot.
    #[must_use]
    pub fn discard_what(what: Reference) -> Action {
        Action::Composite {
            name: crate::VerbName::from("Discard"),
            body: Arc::new(crate::OneShotEffect::Act(Action::move_to(
                what,
                crate::Zone::Graveyard,
            ))),
        }
    }
}

/// `who`'s hand ([CR#701.9a] discard's domain) as a
/// [`Predicate`](crate::Predicate) — the expanded shape of the `InHand(who)`
/// macro (`plugins/builtin/macros/filter/InHand.ron`): `And([InZone(Hand),
/// Owner(Ref(who))])`. Builds the filter [`Action::discard`]'s `Choose`/
/// `Random` binder carries; [`discard_body_whose`] reads `who` back off it
/// for the at-random form, which (unlike `Choose`) carries no separate `by`.
fn discard_hand_filter(who: Reference) -> crate::Predicate {
    crate::Predicate::And(
        vec![
            crate::Predicate::State(crate::StatePredicate::InZone(crate::Zone::Hand)),
            crate::Predicate::Relation(crate::RelationPredicate::Owner(Arc::new(
                crate::Predicate::Ref(who),
            ))),
        ]
        .into(),
    )
}

/// The `who` an `InHand(who)`-shaped filter names — the hand-owning
/// `Reference` under a discard binder's `Owner(Ref(who))` sub-predicate.
/// Shared by [`discard_body_whose`]'s at-random arm, which has no separate
/// `by` field to read (unlike [`Binder::Choose`](crate::Binder::Choose)).
fn hand_owner_ref(filter: &crate::Predicate) -> Option<&Reference> {
    use crate::Predicate as P;
    match filter {
        P::Expanded(e) => hand_owner_ref(&e.value),
        P::And(parts) => parts.iter().find_map(hand_owner_ref),
        P::Relation(crate::RelationPredicate::Owner(inner)) => match inner.as_ref() {
            P::Ref(r) => Some(r),
            P::Expanded(e) => match e.value.as_ref() {
                P::Ref(r) => Some(r),
                _ => None,
            },
            _ => None,
        },
        _ => None,
    }
}

/// The BOUND-form patient of a discard composite's stored body
/// ([CR#702.29a] "discard this card"): the reference its body's HEAD moves,
/// when that head is a single relocation (`Move(This, Graveyard)` →
/// `Some(This)`). The chosen/random form (a `With` choose-then-act step,
/// [`Action::discard`]) → `None`. Read off the stored body ("matches the
/// expanded body") — shared by the engine's resolve lane, the renderer, and
/// the Idris emitter, so the three can never disagree on which form a
/// discard is.
#[must_use]
pub fn discard_body_what(body: &crate::OneShotEffect) -> Option<&Reference> {
    use crate::OneShotEffect as Ose;
    match body {
        Ose::Expanded(e) => discard_body_what(&e.value),
        Ose::Act(Action::Move(what, Destination::Zone(_), _, _)) => Some(what),
        _ => None,
    }
}

/// Whether a discard composite's stored body selects AT RANDOM
/// ([CR#701.9b]): its `With` binder is [`Selection::Random`] over the hand
/// filter, rather than a [`Binder::Choose`](crate::Binder::Choose). The
/// at-random detail is the binder's, not the tag's — shared like
/// [`discard_body_what`].
#[must_use]
pub fn discard_body_random(body: &crate::OneShotEffect) -> bool {
    use crate::OneShotEffect as Ose;
    match body {
        Ose::Expanded(e) => discard_body_random(&e.value),
        Ose::With(with) => {
            matches!(&with.binder, crate::Binder::Existing(Selection::Random(..)))
        }
        _ => false,
    }
}

/// The `count` of a CHOSEN/RANDOM discard composite's stored body
/// ([CR#701.9b]) — the upper bound of the `With` binder's `Quantity`
/// (`Choose`'s or `Selection::Random`'s). `None` for a bound single-move body
/// (`discard this card`, always one card). Read off the stored body, sharing
/// the descent with [`discard_body_what`]/[`discard_body_random`] so
/// re-agenting a discard cost ([CR#601.2h]) reads its count without a typed
/// atom.
#[must_use]
pub fn discard_body_count(body: &crate::OneShotEffect) -> Option<&Count> {
    use crate::OneShotEffect as Ose;
    match body {
        Ose::Expanded(e) => discard_body_count(&e.value),
        Ose::With(with) => match &with.binder {
            crate::Binder::Choose { quantity, .. }
            | crate::Binder::Existing(Selection::Random(quantity, _)) => quantity.bounds().1,
            _ => None,
        },
        _ => None,
    }
}

/// The discarding performer of a CHOSEN/RANDOM discard composite's stored
/// body ([CR#701.9b]) — [`Binder::Choose`](crate::Binder::Choose)'s `by`
/// (the chooser IS the discarding player by default), or, for the at-random
/// form (which carries no `by`), the hand-owning `who` its
/// [`Selection::Random`] filter names ([`hand_owner_ref`]). `None` for a
/// bound single-move body (its performer rides the patient, not a separate
/// slot) or any other shape. Read off the stored body — shared by the
/// engine's resolve lane, the renderer, and the Idris emitter.
#[must_use]
pub fn discard_body_whose(body: &crate::OneShotEffect) -> Option<&Reference> {
    use crate::OneShotEffect as Ose;
    match body {
        Ose::Expanded(e) => discard_body_whose(&e.value),
        Ose::With(with) => match &with.binder {
            crate::Binder::Choose { by, .. } => Some(by),
            crate::Binder::Existing(Selection::Random(_, filter)) => hand_owner_ref(filter),
            _ => None,
        },
        _ => None,
    }
}

/// The two fighters a `Fight` body names — the sources of its reciprocal
/// `DealDamage` halves under the `If`-guard's `then` batch ([CR#701.14a]).
/// ONE shared read-off-the-body facet (the `discard_body_what` precedent):
/// the engine's resolve lane and the Idris emitter both consume this, so
/// the fighters are read off the stored body identically everywhere.
#[must_use]
pub fn fight_body_fighters(body: &crate::OneShotEffect) -> Option<(&Reference, &Reference)> {
    use crate::Action as A;
    use crate::OneShotEffect as Ose;
    match body {
        Ose::Expanded(e) => fight_body_fighters(&e.value),
        Ose::If(iff) => fight_body_fighters(&iff.then),
        Ose::Simultaneously(parts) => match parts.as_ref() {
            [
                Ose::Act(A::DealDamage(a, _, _)),
                Ose::Act(A::DealDamage(b, _, _)),
                ..,
            ] => Some((a, b)),
            _ => None,
        },
        _ => None,
    }
}

impl PlayerAction {
    /// Whether this verb may appear in a cost (`CostComponent::Do`): the
    /// payer performs it, nothing targets ([CR#601.2b..601.2c]). Cost-eligible
    /// verbs are the self-directed ones a player can pay with — sacrifice,
    /// relocate (`Move`, e.g. self-exile to pay — [CR#701.13]), tap, untap,
    /// discard, pay-life (`LoseLife`), reveal ("reveal a blue card from
    /// your hand:" — a reveal that is part of a cost stays shown until the
    /// spell leaves the stack, [CR#701.20a]), and put/remove counters. Counter
    /// costs ride these two verbs rather than a dedicated cost verb: "remove a
    /// +1/+1 counter:" is `RemoveCounters`, "pay {E}" is `RemoveCounters` on a
    /// player, and a planeswalker loyalty ability's `+N`/`−N` cost is
    /// `PutCounters`/`RemoveCounters` of the loyalty counter on its source
    /// ([CR#606.4] — the cost to activate a loyalty ability is to put on or
    /// remove that many loyalty counters). The Idris grammar makes the same
    /// call: loyalty costs reuse `Do (PutCounters/RemoveCounters
    /// loyaltyCounter N This)`, "so there is no duplicate counter-cost
    /// verb" (`idris/src/Core.idr`, the `Cost` `Do` and
    /// `PutCounters`/`RemoveCounters` doc comments).
    #[must_use]
    pub fn is_cost_eligible(&self) -> bool {
        matches!(
            self,
            PlayerAction::Sacrifice(_)
                | PlayerAction::Move(..)
                | PlayerAction::Tap(_)
                | PlayerAction::Untap(_)
                | PlayerAction::LoseLife(_)
                | PlayerAction::PutCounters(..)
                | PlayerAction::RemoveCounters(..)
                | PlayerAction::Reveal { .. }
        )
    }
}

impl Action {
    /// Whether this action may appear in a cost (`CostComponent::Do`) —
    /// the [`Action`]-level twin of [`PlayerAction::is_cost_eligible`],
    /// needed since `Do` holds a full [`Action`] (the Idris `Do : Action b
    /// -> Cost b`). A player verb defers to its own eligibility (the agent
    /// is the payer); the ONE cost-eligible composite is the discard
    /// keyword action ("Discard a card:", cycling's "Discard this card:" —
    /// [CR#701.9,702.29a]): the payer performs it, nothing targets
    /// ([CR#601.2b..601.2c]).
    #[must_use]
    pub fn is_cost_eligible(&self) -> bool {
        match self {
            Action::By(_, pa) => pa.is_cost_eligible(),
            // The discard composite, plus the DIRECT `Action::Move`:
            // `Do(Move(This, Exile))` (Scavenge's self-exile) reads as the
            // direct variant — in `Action` position it shadows the
            // `By(You, …)` embed — and is the same payer-performed
            // relocation ([CR#701.13]).
            Action::Composite { name, .. } if name.as_str() == "Discard" => true,
            Action::Move(..) => true,
            _ => false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::count::Count;
    use crate::reference::Reference;
    use crate::selection::Selection;

    fn read(source: &str) -> Action {
        crate::ron::options().from_str(source).unwrap()
    }
    fn write(action: &Action) -> String {
        crate::ron::options().to_string(action).unwrap()
    }

    #[test]
    fn is_cost_eligible_covers_self_directed_verbs() {
        assert!(PlayerAction::Sacrifice(Reference::This).is_cost_eligible());
        assert!(
            PlayerAction::Move(
                Reference::This,
                Destination::Zone(crate::Zone::Exile),
                vec![].into()
            )
            .is_cost_eligible()
        );
        assert!(PlayerAction::Tap(Reference::This).is_cost_eligible());
        assert!(PlayerAction::Untap(Reference::This).is_cost_eligible());
        assert!(PlayerAction::LoseLife(Count::Literal(1)).is_cost_eligible());
        // The Action-level twin ([CR#601.2b]): a `By` defers to its verb;
        // the ONE cost-eligible composite is the discard keyword action
        // ("Discard a card:", cycling's bound form — [CR#701.9,702.29a]).
        assert!(Action::discard(Reference::You, Count::Literal(1), false).is_cost_eligible());
        assert!(Action::discard_what(Reference::This).is_cost_eligible());
        // A non-discard composite (destroy) is not cost-eligible.
        assert!(!Action::destroy(Reference::This).is_cost_eligible());
        // Counter costs ride these two verbs (loyalty `+N`/`−N`, "remove a
        // counter:", "pay {E}") — no dedicated counter-cost verb
        // ([CR#606.4]).
        assert!(
            PlayerAction::PutCounters(
                Reference::This,
                crate::CounterRef::from("LoyaltyCounter"),
                Count::Literal(1),
            )
            .is_cost_eligible()
        );
        assert!(
            PlayerAction::RemoveCounters(
                Reference::This,
                crate::CounterRef::from("LoyaltyCounter"),
                Count::Literal(2),
            )
            .is_cost_eligible()
        );

        assert!(!PlayerAction::GainLife(Count::Literal(3)).is_cost_eligible());
        assert!(
            !PlayerAction::AddMana(Count::Literal(1), crate::ManaSpec::AnyColor.into())
                .is_cost_eligible()
        );
    }

    /// The discard keyword action as data ([CR#701.9]): [`Action::discard`]
    /// builds `Composite(name: "Discard", body: With(Choose/Random, Each …))`
    /// — the CHOSEN form, the affected player picking `n` from hand
    /// ([CR#701.9b]) — and
    /// [`Action::discard_what`] the BOUND single-move form ("discard this
    /// card", cycling's cost [CR#702.29a]), the destroy shape. Both
    /// round-trip structurally, and the body facets (who/count/random/patient)
    /// read back through the shared descent helpers.
    #[test]
    fn discard_composite_forms_round_trip() {
        let chosen = Action::discard(Reference::You, Count::Literal(2), false);
        let Action::Composite { name, body } = &chosen else {
            panic!("expected a discard Composite, got {chosen:?}");
        };
        assert_eq!(name.as_str(), "Discard");
        assert_eq!(discard_body_count(body), Some(&Count::Literal(2)));
        assert!(
            discard_body_what(body).is_none(),
            "chosen form binds no card"
        );
        assert!(!discard_body_random(body));
        assert_eq!(read(&write(&chosen)), chosen);

        let bound = Action::discard_what(Reference::This);
        let Action::Composite { name, body } = &bound else {
            panic!("expected a discard Composite, got {bound:?}");
        };
        assert_eq!(name.as_str(), "Discard");
        assert_eq!(
            discard_body_count(body),
            None,
            "a bound discard is one card — no chosen count"
        );
        assert_eq!(discard_body_what(body), Some(&Reference::This));
        assert_eq!(read(&write(&bound)), bound);
    }

    /// [`fight_body_fighters`] reads the reciprocal `DealDamage` pair off a
    /// `Fight` body's `If`-guarded `then` batch ([CR#701.14a]) — the shape
    /// `plugins/builtin/macros/effect/Fight.ron` expands to (mirrored by the
    /// engine's `fight_effect` test builder at
    /// `crates/deckmaste_engine/src/resolve/effect.rs`). The guard's exact
    /// condition is irrelevant to the reader — it only descends into `then` —
    /// so a minimal `Compare` stands in for the real creature/battlefield
    /// check.
    #[test]
    fn fight_body_fighters_reads_both_reciprocal_sources() {
        use crate::Action;
        use crate::OneShotEffect;
        let half = |src: Reference, tgt: Reference| {
            OneShotEffect::Act(Action::DealDamage(
                src.clone(),
                Count::StatOf(src, crate::Stat::Power),
                tgt,
            ))
        };
        let body = OneShotEffect::If(crate::If {
            condition: crate::Condition::Compare(
                Count::Literal(1),
                crate::Cmp::Eq,
                Count::Literal(1),
            ),
            then: Arc::new(OneShotEffect::Simultaneously(
                vec![
                    half(Reference::Target(0), Reference::Target(1)),
                    half(Reference::Target(1), Reference::Target(0)),
                ]
                .into(),
            )),
            otherwise: None,
        });
        let (a, b) = fight_body_fighters(&body).expect("both fighters");
        assert_eq!(a, &Reference::Target(0));
        assert_eq!(b, &Reference::Target(1));
        // Not a fight body → None, never a panic.
        assert!(
            fight_body_fighters(&OneShotEffect::Act(Action::deal_damage(
                Reference::Target(0),
                Count::Literal(1)
            )))
            .is_none()
        );
    }

    /// A bare player verb reads as `By(You, …)` — the implicit-you default:
    /// the derive-generated embed dispatch handles unknown identifiers even
    /// without a `MacroSet` in play (plain `ron` read).
    #[test]
    fn bare_player_verb_defaults_to_you() {
        assert_eq!(
            read("Sacrifice(This)"),
            Action::By(Reference::You, PlayerAction::Sacrifice(Reference::This),),
        );
        assert_eq!(
            read("Tap(This)"),
            Action::By(Reference::You, PlayerAction::Tap(Reference::This),),
        );
    }

    /// An explicit different agent reads natively — the triggering event's
    /// actor as the agent ("that player sacrifices a creature").
    #[test]
    fn explicit_agent_reads_natively() {
        assert_eq!(
            read("By(EventActor, Sacrifice(This))"),
            Action::By(
                Reference::EventActor,
                PlayerAction::Sacrifice(Reference::This),
            ),
        );
    }

    /// The source-agent verbs read natively. `DealDamage(source, n, target)`
    /// spells its source first — the common case names `This`.
    #[test]
    fn source_verbs_read_natively() {
        assert_eq!(
            read("DealDamage(This, Literal(3), It)"),
            Action::DealDamage(Reference::This, Count::Literal(3), Reference::It,),
        );
        // Destroy is no longer a bespoke verb — it is the `Composite` the
        // `Action::destroy` ctor builds (body = the Battlefield → Graveyard
        // `Move`), now a struct variant named by its verb; the raw Composite
        // spelling round-trips through plain RON.
        assert_eq!(
            read("Composite(name: Destroy, body: Move(This,Graveyard))"),
            Action::destroy(Reference::This),
        );
    }

    /// `DealDamage`'s `source` is required and always spelled first; the common
    /// `This`-source case round-trips, and an explicit non-`This` source (the
    /// "fight" / redirected-damage case) round-trips the same way.
    #[test]
    fn deal_damage_source_required_and_round_trips() {
        // Common source: `This` is spelled, not omitted.
        let common = Action::deal_damage(Reference::It, Count::Literal(3));
        let written = write(&common);
        assert_eq!(written, "DealDamage(This,3,It)", "source spelled first");
        assert_eq!(read(&written), common);

        // Explicit source: the redirected-damage shape — the sorted anaphor is
        // both the dealer (first slot) and the recipient (last slot).
        let sourced = Action::DealDamage(
            Reference::That(crate::Sort::OfType(crate::Type::Creature)),
            Count::StatOf(
                Reference::That(crate::Sort::OfType(crate::Type::Creature)),
                crate::count::Stat::Power,
            ),
            Reference::This,
        );
        let written = write(&sourced);
        assert!(
            written.starts_with("DealDamage(That(Creature),") && written.ends_with("This)"),
            "source written in the first slot, target last: {written}"
        );
        assert_eq!(read(&written), sourced);
        // Long-form read: source, amount, target.
        assert_eq!(
            read("DealDamage(That(Creature), StatOf(That(Creature), Power), This)"),
            sourced,
        );
    }

    /// `Attach`/`Unattach` (the source-agent attachment verbs) read natively
    /// and round-trip.
    #[test]
    fn attach_verbs_round_trip() {
        let attach = Action::Attach {
            what: Reference::This,
            to: Reference::It,
        };
        assert_eq!(read(&write(&attach)), attach);
        let unattach = Action::Unattach(Reference::This);
        assert_eq!(read("Unattach(This)"), unattach);
        assert_eq!(read(&write(&unattach)), unattach);
    }

    /// `Move(This, Graveyard)` (the source-agent plain-relocation verb the
    /// [CR#704.5m] Aura SBA uses) reads natively with a bare zone name (the
    /// flattened `Destination::Zone`) and round-trips.
    #[test]
    fn move_verb_round_trips() {
        use crate::Zone;
        let mv = Action::move_to(Reference::This, Zone::Graveyard);
        assert_eq!(read("Move(This, Graveyard)"), mv);
        // The bare zone name is the flattened Destination::Zone — written bare.
        assert_eq!(write(&mv), "Move(This,Graveyard)");
        assert_eq!(read(&write(&mv)), mv);
    }

    /// `Move`'s optional `from` fizzle-guard ([CR#701.8a]). Design note (the
    /// tuple-vs-struct fork this variant's Step 1 settled): a struct variant
    /// would let `from`/`to` read as named fields (`Move(what: …, from: …,
    /// to: …)`), but this crate's struct-variant helpers ALWAYS read
    /// all-named via `unwrap_variant_newtypes` (`ron`'s
    /// `handle_struct_after_name` only ever calls `visit_map`, never
    /// `visit_seq`) — converting would break the bare positional
    /// `Move(This, Graveyard)` spelling the whole corpus uses. So `Move`
    /// stays a tuple variant and `from` rides as a fourth, trailing,
    /// positionally-read slot (after `riders`, both defaulted); the
    /// `from:`/`to:` NAMED spelling belongs to the verb-macro parameter
    /// layer (later tasks), not this raw variant's RON syntax.
    #[test]
    fn move_from_guard_round_trips() {
        use crate::Zone;

        // Existing spelling unchanged: no fourth slot, no guard.
        assert_eq!(
            read("Move(This, Graveyard)"),
            Action::move_to(Reference::This, Zone::Graveyard)
        );

        // Guarded spelling: the trailing fizzle-guard slot (after the rider
        // list) reads and writes back. `IMPLICIT_SOME` ([`crate::ron::options`])
        // writes a present `Option` bare (no `Some(...)` wrapper), so the
        // written guard is just the zone name.
        let guarded = Action::move_if_in(Reference::This, Zone::Hand, Zone::Graveyard);
        let written = write(&guarded);
        assert_eq!(read(&written), guarded);
        assert_eq!(written, "Move(This,Graveyard,[],Hand)");
    }

    /// `GetDesignation` is a player verb: bare it reads as `By(You, …)`, and it
    /// round-trips. The designation name is a quoted Ident ([CR#702.131c]).
    #[test]
    fn get_designation_round_trips_bare() {
        let v = Action::By(
            Reference::You,
            PlayerAction::GetDesignation("CitysBlessing".into()),
        );
        let written = write(&v);
        assert!(
            !written.contains("By("),
            "By(You, …) should write bare, got {written}"
        );
        assert_eq!(read(&written), v);
        assert_eq!(
            read(r#"GetDesignation("CitysBlessing")"#),
            v,
            "a bare player verb reads as By(You, …)"
        );
    }

    /// [CR#114.1]: `GetEmblem` is a player verb carrying the emblem's
    /// abilities. Bare it reads as `By(You, …)`, writes bare, and the ability
    /// payload round-trips text → `GetEmblem` → text.
    #[test]
    fn get_emblem_round_trips_bare() {
        let ron = "GetEmblem([Static(Modify(It, Power(Up(1))))])";
        let v = read(ron);
        assert!(
            matches!(
                &v,
                Action::By(Reference::You, PlayerAction::GetEmblem(a)) if a.len() == 1
            ),
            "a bare player verb reads as By(You, …) with one ability, got {v:?}"
        );
        let written = write(&v);
        assert!(
            !written.contains("By("),
            "By(You, …) should write bare, got {written}"
        );
        assert_eq!(read(&written), v, "text → GetEmblem → text round-trips");
    }

    /// `By(You, …)` writes the player action bare and round-trips.
    #[test]
    fn by_you_round_trips_bare() {
        let v = Action::By(Reference::You, PlayerAction::GainLife(Count::Literal(1)));
        let written = write(&v);
        assert!(
            !written.contains("By("),
            "By(You, …) should write bare, got {written}"
        );
        assert_eq!(read(&written), v);
    }

    /// A non-`You` agent writes the explicit `By(other, …)` form and
    /// round-trips.
    #[test]
    fn by_other_round_trips() {
        let v = Action::By(
            Reference::EventActor,
            PlayerAction::GainLife(Count::Literal(3)),
        );
        let written = write(&v);
        assert!(
            written.contains("By("),
            "expected explicit By, got {written}"
        );
        assert_eq!(read(&written), v);
    }

    /// Library destinations: top-of-library (the former `PutInLibrary`, now
    /// `Move(sel, Library(FromTop(0)))`) and bottom-of-library
    /// (`Library(FromBottom(0))`, newly expressible) read natively and
    /// round-trip ([CR#401.7]).
    #[test]
    fn move_to_library_anchors_round_trip() {
        let top = Action::Move(
            Reference::This,
            Destination::Library(Anchor::FromTop(Count::Literal(0))),
            vec![].into(),
            None,
        );
        assert_eq!(read("Move(This, Library(FromTop(0)))"), top);
        assert_eq!(read(&write(&top)), top);

        let bottom = Action::Move(
            Reference::This,
            Destination::Library(Anchor::FromBottom(Count::Literal(0))),
            vec![].into(),
            None,
        );
        assert_eq!(read("Move(This, Library(FromBottom(0)))"), bottom);
        assert_eq!(read(&write(&bottom)), bottom);
    }

    /// The library and the stack are excluded from `Destination`'s flattened
    /// zone set (`exclude(Library, Stack)`), mirroring the Idris
    /// `DestinationOk` gate: the library is a destination only at an
    /// [`Anchor`] (the anchored `Library(Anchor)` is canonical — a bare
    /// `Library` is not a `Move` destination, so a library destination has
    /// ONE spelling), and the stack is never a `Move` target
    /// ([CR#401.7,405.1]). The other zones still flatten.
    #[test]
    fn library_and_stack_are_not_flattened_destinations() {
        use crate::Zone;
        // Bare `Library` is rejected — the anchored form is the only spelling.
        let err = crate::ron::options()
            .from_str::<Action>("Move(This, Library)")
            .unwrap_err();
        let err = err.to_string();
        assert!(
            err.contains("Expected opening `(`"),
            "unexpected parse error for bare Library: {err}"
        );
        // The stack is never a Move destination.
        let err = crate::ron::options()
            .from_str::<Action>("Move(This, Stack)")
            .unwrap_err();
        let err = err.to_string();
        assert!(
            err.contains("neither") || err.contains("Expected opening `(`"),
            "unexpected parse error for bare Stack: {err}"
        );
        // Neither name lifts into `Destination`'s dispatch set.
        assert!(!Destination::ALL_VARIANTS.contains(&"Stack"));
        assert_eq!(
            Destination::ALL_VARIANTS
                .iter()
                .filter(|n| **n == "Library")
                .count(),
            1,
            "`Library` is Destination's OWN (anchored) variant only, not also a flattened Zone",
        );
        // The remaining zones still flatten and round-trip.
        for zone in [
            Zone::Battlefield,
            Zone::Command,
            Zone::Exile,
            Zone::Graveyard,
            Zone::Hand,
        ] {
            let mv = Action::move_to(Reference::This, zone);
            assert_eq!(read(&write(&mv)), mv, "zone {zone:?} should still flatten");
        }
    }

    /// A `Move` to the battlefield may carry entry riders ([CR#614.12]): the
    /// short two-slot form still reads (riders default `[]`) and writes back
    /// short; an explicit rider list reads via the third slot and
    /// round-trips — the Path-to-Exile search-tapped / reanimation-
    /// under-owner's-control encodings.
    #[test]
    fn move_riders_default_and_round_trip() {
        use crate::Zone;
        // Short form: no riders — written without the third slot.
        let bare = Action::move_to(Reference::This, Zone::Battlefield);
        assert_eq!(read("Move(This, Battlefield)"), bare);
        assert_eq!(write(&bare), "Move(This,Battlefield)");

        // Riders read positionally and round-trip.
        let tapped = Action::Move(
            Reference::That(crate::Sort::Card),
            Destination::Zone(Zone::Battlefield),
            vec![EnterRider::Tapped, EnterRider::UnderOwnersControl].into(),
            None,
        );
        assert_eq!(
            read("Move(That(Card), Battlefield, [Tapped, UnderOwnersControl])"),
            tapped,
        );
        assert_eq!(read(&write(&tapped)), tapped);

        // The counter/attacking riders carry their payloads.
        let countered = Action::Move(
            Reference::That(crate::Sort::Card),
            Destination::Zone(Zone::Battlefield),
            vec![
                EnterRider::WithCounters(crate::CounterRef::from("P1P1Counter"), Count::Literal(1)),
                EnterRider::Attacking(Some(Reference::Opponent)),
            ]
            .into(),
            None,
        );
        assert_eq!(read(&write(&countered)), countered);
    }

    /// `EnterRider::AsCopy(CopySpec)` ([CR#707.5]) — "enters as a copy of
    /// [source]" — carries the shared `CopySpec` payload and round-trips as
    /// a `Move` rider, mirroring `copyspec_round_trips_all_exception_kinds`
    /// (`copy.rs`).
    #[test]
    fn enter_rider_as_copy_round_trips() {
        use crate::CopySource;
        use crate::CopySpec;
        use crate::Zone;

        let mv = Action::Move(
            Reference::That(crate::Sort::Card),
            Destination::Zone(Zone::Battlefield),
            vec![EnterRider::AsCopy(CopySpec {
                source: CopySource::Object(Reference::Target(0)),
                exceptions: vec![],
            })]
            .into(),
            None,
        );
        assert_eq!(read(&write(&mv)), mv, "EnterRider::AsCopy round-trips");
    }

    /// `PlayerAction::CastCopy(CopySpec)` ([CR#707.12]) — "cast a copy of
    /// [source]" — sits beside `CopySpell` and round-trips.
    #[test]
    fn cast_copy_round_trips() {
        use crate::CopySource;
        use crate::CopySpec;

        let cast = PlayerAction::CastCopy(CopySpec {
            source: CopySource::Object(Reference::Target(0)),
            exceptions: vec![],
        });
        let ron = crate::ron::options().to_string(&cast).unwrap();
        let back: PlayerAction = crate::ron::options().from_str(&ron).unwrap();
        assert_eq!(back, cast, "PlayerAction::CastCopy round-trips: {ron}");
    }

    /// `MoveGroup` — the group relocation with an [`Arrangement`]
    /// ([CR#401.4]) — reads flat as a struct variant, omits an empty rider
    /// list, and round-trips: Brainstorm's "on top of your library in any
    /// order".
    #[test]
    fn move_group_arrangements_round_trip() {
        use crate::Selection;
        let brainstorm = Action::MoveGroup {
            group: Selection::They,
            arrangement: Arrangement::AnyOrder,
            to: Destination::Library(Anchor::FromTop(Count::Literal(0))),
            riders: vec![].into(),
        };
        assert_eq!(
            read("MoveGroup(group: They, arrangement: AnyOrder, to: Library(FromTop(0)))"),
            brainstorm,
        );
        let written = write(&brainstorm);
        assert!(
            !written.contains("riders"),
            "empty riders omitted: {written}"
        );
        assert_eq!(read(&written), brainstorm);

        // The chooser-carrying arrangement and a rider list round-trip.
        let arranged = Action::MoveGroup {
            group: Selection::They,
            arrangement: Arrangement::ChosenOrder(Reference::Opponent),
            to: Destination::Zone(crate::Zone::Battlefield),
            riders: vec![EnterRider::Tapped].into(),
        };
        assert_eq!(read(&write(&arranged)), arranged);
        for arrangement in ["SameOrder", "RandomOrder"] {
            let v = read(&format!(
                "MoveGroup(group: They, arrangement: {arrangement}, to: Library(FromBottom(0)))"
            ));
            assert_eq!(read(&write(&v)), v, "round-trip failed for {arrangement}");
        }
    }

    /// The new verb shapes — `ExtraPhase` ([CR#500.8]), day/night
    /// ([CR#731.1]), `TheRingTempts` ([CR#701.54a]), and the player verb
    /// `VentureIntoDungeon` ([CR#701.49a]) — read and round-trip; the player
    /// verb reads bare as `By(You, …)`. (Fight is now a grammar macro over
    /// `DealDamage`, not a primitive verb; Mill is now the `Composite`
    /// keyword-action atom `Action::mill`, not a `PlayerAction`.)
    #[test]
    fn new_verb_shapes_round_trip() {
        let phase = Action::ExtraPhase(crate::PhaseKind::Combat, Reference::You);
        assert_eq!(read("ExtraPhase(Combat, You)"), phase);
        assert_eq!(read(&write(&phase)), phase);

        for (source, want) in [
            ("BecomeDay", Action::BecomeDay),
            ("BecomeNight", Action::BecomeNight),
        ] {
            assert_eq!(read(source), want);
            assert_eq!(read(&write(&want)), want);
        }

        let tempt = Action::TheRingTempts(Reference::You);
        assert_eq!(read("TheRingTempts(You)"), tempt);
        assert_eq!(read(&write(&tempt)), tempt);

        let venture = Action::by_you(PlayerAction::VentureIntoDungeon);
        assert_eq!(read("VentureIntoDungeon"), venture);
        assert_eq!(read(&write(&venture)), venture);
    }

    /// The at-random distinction ([CR#701.9b]) rides the body's `With`
    /// binder shape, not the atom: `Choose` (chosen) vs. `Selection::Random`
    /// (at random) — both round-trip; the chosen form's write never mentions
    /// "random" — "discard a card at random".
    #[test]
    fn discard_random_rides_the_selection_and_round_trips() {
        let chosen = Action::discard(Reference::You, Count::Literal(1), false);
        assert!(!write(&chosen).contains("random"), "false random omitted");

        let random = Action::discard(Reference::You, Count::Literal(1), true);
        let Action::Composite { body, .. } = &random else {
            panic!("expected a discard Composite, got {random:?}");
        };
        assert!(discard_body_random(body));
        assert_eq!(read(&write(&random)), random);
    }

    /// `MoveCounters(spec, from, to)` reads natively for both a named
    /// kind+count (Power Conduit / Leech Bonder) and `AllKinds` (Ozolith /
    /// Fate Transfer), and round-trips ([CR#122]).
    #[test]
    fn move_counters_round_trips() {
        let named = Action::MoveCounters(
            crate::CounterSpec::Named(crate::CounterRef::from("P1P1Counter"), Count::Literal(1)),
            Reference::Target(0),
            Reference::Target(1),
        );
        assert_eq!(
            read("MoveCounters(Named(P1P1Counter, 1), Target(0), Target(1))"),
            named,
        );
        assert_eq!(read(&write(&named)), named);

        let all = Action::MoveCounters(
            crate::CounterSpec::AllKinds,
            Reference::Target(0),
            Reference::Target(1),
        );
        assert_eq!(read("MoveCounters(AllKinds, Target(0), Target(1))"), all);
        assert_eq!(read(&write(&all)), all);
    }

    /// [CR#608.2g]: `Cast` is a player verb — bare it reads as `By(You, …)`
    /// (the implicit-you caster) and round-trips. "That card" is the
    /// surrounding effect's anaphor. The optional alternative-cost slot
    /// ([CR#118.9,702.35a]) is a trailing tuple default: omitting it keeps the
    /// bare `Cast(That(Card))` spelling reading (Chandra), an explicit cost
    /// reads positionally `Cast(That(Card), [Mana(…)])` (madness) — the
    /// tuple/struct decision, whose deciding criterion is that bare read.
    #[test]
    fn cast_round_trips_bare() {
        let v = Action::by_you(PlayerAction::Cast(Reference::That(crate::Sort::Card), None));
        let written = write(&v);
        assert!(
            !written.contains("By("),
            "By(You, …) should write bare, got {written}"
        );
        assert!(
            !written.contains("None"),
            "the absent alternative cost is omitted on write, got {written}"
        );
        assert_eq!(
            read("Cast(That(Card))"),
            v,
            "the bare mana-cost cast reads with the alternative-cost slot defaulted to None"
        );
        assert_eq!(read(&written), v, "text → Cast → text round-trips");

        // An explicit caster reads native (e.g. `By(It, Cast(It))`).
        let explicit = Action::By(Reference::It, PlayerAction::Cast(Reference::It, None));
        assert_eq!(read("By(It, Cast(It))"), explicit);
        assert_eq!(read(&write(&explicit)), explicit);

        // [CR#118.9,702.35a]: the alternative-cost form (madness's madness
        // cost) reads the trailing slot positionally and round-trips.
        let for_cost = Action::by_you(PlayerAction::Cast(
            Reference::That(crate::Sort::Card),
            Some(crate::Cost(
                vec![crate::CostComponent::Mana(crate::ManaCost::from(Arc::<
                    [crate::ManaSymbol],
                >::from(
                    vec![
                    crate::ManaSymbol::Simple(crate::SimpleManaSymbol::Generic(1)),
                ]
                )))]
                .into(),
            )),
        ));
        assert_eq!(
            read("Cast(That(Card), [Mana([Generic(1)])])"),
            for_cost,
            "an explicit alternative cost reads positionally"
        );
        assert_eq!(
            read(&write(&for_cost)),
            for_cost,
            "the for-cost form round-trips"
        );
    }

    /// `Composite { name, body }` ([CR#701]) reads all-named (the
    /// struct-variant spelling, like `KeywordAbility::Composite`) and
    /// round-trips — the keyword-action the scry/surveil/fateseal macros
    /// desugar to. The `name` is a bareword [`crate::VerbName`]; the
    /// performer rides the body's selection (`whose`).
    #[test]
    fn composite_round_trips() {
        let scry = Action::Composite {
            name: crate::VerbName::from("Scry"),
            body: Arc::new(crate::OneShotEffect::Each(crate::Each {
                binder: crate::Binder::Existing(Selection::TopOfLibrary {
                    count: Count::Literal(2),
                    whose: Reference::You,
                }),
                effect: Arc::new(crate::OneShotEffect::Act(Action::Move(
                    Reference::It,
                    Destination::Library(Anchor::FromTop(Count::Literal(0))),
                    vec![].into(),
                    None,
                ))),
            })),
        };
        assert_eq!(read(&write(&scry)), scry);
        // The verb name renders BAREWORD, all-named struct-variant fields.
        assert!(
            write(&scry).starts_with("Composite(name:Scry,body:"),
            "verb must render bareword-named, got {}",
            write(&scry)
        );
        assert_eq!(
            read(
                "Composite(name:Scry,body:Each(binder:Existing(TopOfLibrary(count:2,whose:You)),effect:Move(It,Library(FromTop(0)))))"
            ),
            scry
        );
    }

    /// [`Action::mill_one`] ([CR#701.17a]) is the PER-UNIT `Composite { name:
    /// "Mill", body: <group move> }` — a `MoveGroup` of the top slice of
    /// `who`'s library to their graveyard, the late-bound slice. `who` rides
    /// the group selection's `whose`. It round-trips all-named, and the whole
    /// mill wraps it in a `Batch` ([`OneShotEffect::mill`]).
    #[test]
    fn mill_composite_round_trips() {
        let mill = Action::mill_one(Reference::It);
        assert_eq!(
            mill,
            Action::Composite {
                name: crate::VerbName::from("Mill"),
                body: Arc::new(crate::OneShotEffect::Act(Action::MoveGroup {
                    group: Selection::TopOfLibrary {
                        count: Count::Literal(1),
                        whose: Reference::It,
                    },
                    arrangement: Arrangement::AnyOrder,
                    to: Destination::Zone(crate::Zone::Graveyard),
                    riders: [].into(),
                })),
            }
        );
        assert_eq!(read(&write(&mill)), mill);
        assert!(
            write(&mill).starts_with("Composite(name:Mill,body:"),
            "verb renders bareword-named, got {}",
            write(&mill)
        );

        // The whole mill is `Batch(count, Act(Mill))` — the aggregate window.
        let whole = crate::OneShotEffect::mill(Reference::It, Count::Literal(5));
        assert_eq!(
            whole,
            crate::OneShotEffect::Batch(
                Count::Literal(5),
                Arc::new(crate::OneShotEffect::Act(Action::mill_one(Reference::It)))
            )
        );
    }
}
