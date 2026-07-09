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
/// Authored RON writes a bare [`PlayerAction`] in an effect slot (`Draw(1)`,
/// `Tap(This)`, `Sacrifice(This)`) and it reads as `By(Reference::You, …)` —
/// the implicit-you default, declared by `By`'s `#[macro_ron(embed)]` marker
/// with `#[macro_ron(default = "Reference::You")]` on the agent field, via
/// the macro layer's `embeds_untagged` hook (`Action` is registered with
/// `.embeds_untagged()`). An explicit different agent is written
/// `By(It, Draw(3))` (Ancestral Recall) and read natively. Both serde
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
    /// Destroy the referenced permanent ([CR#701.8]).
    Destroy(Reference),
    /// Return the referenced object to its owner's hand.
    ReturnToHand(Reference),
    /// Counter the referenced spell or ability on the stack ([CR#701.6a]) — a
    /// countered spell moves to its owner's graveyard; a countered ability
    /// simply ceases. "Can't be countered" is deontic-layer territory, not
    /// part of the verb.
    Counter(Reference),
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
    /// zone change (emits `ZoneWillChange`), NOT destruction (so indestructible
    /// does not apply, distinct from [`Destroy`](Action::Destroy)) and NOT a
    /// sacrifice. A graveyard/hand/library destination is the object's
    /// *owner's*; exile is the shared exile zone. The destination is a bare
    /// zone name (`Move(This, Graveyard)` — the [CR#704.5m] Aura graveyard SBA)
    /// or the library at an anchor (`Move(This, Library(FromTop(0)))` — top of
    /// library, the former `PutInLibrary`; `Library(FromBottom(0))` — bottom,
    /// [CR#401.7]). This one verb subsumes the old `Move`/`PutInLibrary` split.
    /// The trailing `riders` list ([`EnterRider`], default `[]`, omitted on
    /// write when empty) spells arrival state for a BATTLEFIELD destination —
    /// "onto the battlefield tapped / under its owner's control / with a +1/+1
    /// counter on it" ([CR#614.12]); riders on any other destination are
    /// ill-formed (rejected by the Idris re-emit gate).
    Move(
        Reference,
        Destination,
        #[macro_ron(default = "Vec::new()")] Vec<EnterRider>,
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
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        riders: Vec<EnterRider>,
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
    /// shields (regeneration, one-shot prevention). `subject` resolves to the
    /// protected permanent; `one_shot` consumes the shield on first use
    /// ([CR#614.3]). [CR#701.19a,614.8]
    CreateReplacement {
        replacement: Box<crate::replacement::Replacement>,
        subject: Reference,
        duration: crate::continuous::Duration,
        one_shot: bool,
    },
    /// A named keyword action ([CR#701]) — a printed keyword verb
    /// (`name`, e.g. "Scry"/"Surveil"/"Fateseal") whose meaning IS its
    /// `body` effect, run when this resolves. Mirrors the Idris `Composite :
    /// KeywordActionSpec -> OneShotEffect -> Action`: the keyword-action
    /// macros desugar to a `Composite` so there are no bespoke
    /// `Scry`/`Surveil` verbs. Resolving it runs `body`, then (when the body
    /// actually acts — scry 0 does nothing, [CR#701.22b]) emits the named
    /// keyword-action event a "whenever you scry/surveil" trigger reads,
    /// exactly like `KeywordAbility::Composite { name, .. }` on the ability
    /// side. `body` is boxed to break the `Action` → `OneShotEffect` → `Action`
    /// size cycle.
    Composite {
        name: crate::Ident,
        body: Box<crate::OneShotEffect>,
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
    /// Draw a number of cards ([CR#121.1]).
    Draw(Count),
    /// Discard cards ([CR#701.9]). `count` is how many; the optional `what`
    /// names *which* — omitted = the discarding player chooses `count` from
    /// hand (the common form, [CR#701.9b]). "Discard this card" (cycling's
    /// cost, [CR#702.29a]) is `Discard(count: Literal(1), what: This)`.
    /// `random: true` is the "discard at random" form ([CR#701.9b] — the
    /// affected player does not choose); senseless combined with a `what`.
    Discard {
        count: Count,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        what: Option<Reference>,
        #[serde(default, skip_serializing_if = "crate::ability::is_false")]
        random: bool,
    },
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
        #[macro_ron(default = "Vec::new()")] Vec<EnterRider>,
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
        #[macro_ron(default = "Vec::new()")] Vec<EnterRider>,
    ),
    /// Mill a number of cards ([CR#701.17a] — the player puts that many cards
    /// from the top of their library into their graveyard). The shape;
    /// engine execution is keyword-action work elsewhere.
    Mill(Count),
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
    GetEmblem(Vec<crate::Ability>),
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
    /// Flip that many coins ([CR#705.1]) — results are events; call/win
    /// framing is the consumer's ([CR#705.2]).
    FlipCoins(Count),
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
    /// Graveyard)`), without spelling the `Destination::Zone` wrapper or the
    /// (empty) rider list.
    #[must_use]
    pub fn move_to(what: Reference, zone: crate::Zone) -> Action {
        Action::Move(what, Destination::Zone(zone), Vec::new())
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
                | PlayerAction::Discard { .. }
                | PlayerAction::LoseLife(_)
                | PlayerAction::PutCounters(..)
                | PlayerAction::RemoveCounters(..)
                | PlayerAction::Reveal { .. }
        )
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
                vec![]
            )
            .is_cost_eligible()
        );
        assert!(PlayerAction::Tap(Reference::This).is_cost_eligible());
        assert!(PlayerAction::Untap(Reference::This).is_cost_eligible());
        assert!(
            PlayerAction::Discard {
                count: Count::Literal(1),
                what: None,
                random: false
            }
            .is_cost_eligible()
        );
        assert!(PlayerAction::LoseLife(Count::Literal(1)).is_cost_eligible());
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

        assert!(!PlayerAction::Draw(Count::Literal(1)).is_cost_eligible());
        assert!(!PlayerAction::GainLife(Count::Literal(3)).is_cost_eligible());
        assert!(
            !PlayerAction::AddMana(Count::Literal(1), crate::ManaSpec::AnyColor.into())
                .is_cost_eligible()
        );
    }

    /// `Discard` carries an OPTIONAL `what` naming *which* cards ([CR#701.9]):
    /// omitted = the discarding player chooses `count` from hand (the common
    /// form); present = those specific cards. "Discard this card" (cycling's
    /// cost, [CR#702.29a]) is `Discard(count: Literal(1), what: This)`.
    #[test]
    fn discard_takes_optional_selection() {
        // count-only: the `what` selection is absent.
        assert_eq!(
            read("Discard(count: Literal(2))"),
            Action::By(
                Reference::You,
                PlayerAction::Discard {
                    count: Count::Literal(2),
                    what: None,
                    random: false
                },
            ),
        );
        // "discard this card" names the specific card via `what`.
        assert_eq!(
            read("Discard(count: Literal(1), what: This)"),
            Action::By(
                Reference::You,
                PlayerAction::Discard {
                    count: Count::Literal(1),
                    what: Some(Reference::This),
                    random: false,
                },
            ),
        );
        // an absent `what` is omitted on write and round-trips; the This form too.
        let bare = Action::By(
            Reference::You,
            PlayerAction::Discard {
                count: Count::Literal(2),
                what: None,
                random: false,
            },
        );
        let written = write(&bare);
        assert!(
            !written.contains("what"),
            "absent `what` omitted: {written}"
        );
        assert_eq!(read(&written), bare);
        let this = Action::By(
            Reference::You,
            PlayerAction::Discard {
                count: Count::Literal(1),
                what: Some(Reference::This),
                random: false,
            },
        );
        assert_eq!(read(&write(&this)), this);
    }

    /// A bare player verb reads as `By(You, …)` — the implicit-you default:
    /// the derive-generated embed dispatch handles unknown identifiers even
    /// without a `MacroSet` in play (plain `ron` read).
    #[test]
    fn bare_player_verb_defaults_to_you() {
        assert_eq!(
            read("Draw(Literal(1))"),
            Action::By(Reference::You, PlayerAction::Draw(Count::Literal(1))),
        );
        assert_eq!(
            read("Sacrifice(This)"),
            Action::By(Reference::You, PlayerAction::Sacrifice(Reference::This),),
        );
        assert_eq!(
            read("Tap(This)"),
            Action::By(Reference::You, PlayerAction::Tap(Reference::This),),
        );
    }

    /// An explicit different agent reads natively — Ancestral Recall's
    /// `By(It, Draw(3))` (the announced-target anaphor as the agent).
    #[test]
    fn explicit_agent_reads_natively() {
        assert_eq!(
            read("By(It, Draw(Literal(3)))"),
            Action::By(Reference::It, PlayerAction::Draw(Count::Literal(3)),),
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
        assert_eq!(read("Destroy(This)"), Action::Destroy(Reference::This),);
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

    /// `By(You, …)` writes the player action bare and round-trips.
    #[test]
    fn by_you_round_trips_bare() {
        let v = Action::By(Reference::You, PlayerAction::Draw(Count::Literal(1)));
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
        let v = Action::By(Reference::EventActor, PlayerAction::Draw(Count::Literal(3)));
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
            vec![],
        );
        assert_eq!(read("Move(This, Library(FromTop(0)))"), top);
        assert_eq!(read(&write(&top)), top);

        let bottom = Action::Move(
            Reference::This,
            Destination::Library(Anchor::FromBottom(Count::Literal(0))),
            vec![],
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
        assert!(
            crate::ron::options()
                .from_str::<Action>("Move(This, Library)")
                .is_err(),
            "bare `Library` must not be a valid Move destination"
        );
        // The stack is never a Move destination.
        assert!(
            crate::ron::options()
                .from_str::<Action>("Move(This, Stack)")
                .is_err(),
            "the stack is never a Move destination"
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
            vec![EnterRider::Tapped, EnterRider::UnderOwnersControl],
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
            ],
        );
        assert_eq!(read(&write(&countered)), countered);
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
            riders: vec![],
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
            riders: vec![EnterRider::Tapped],
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
    /// ([CR#731.1]), `TheRingTempts` ([CR#701.54a]), and the player verbs
    /// `Mill` ([CR#701.17a]) / `VentureIntoDungeon` ([CR#701.49a]) — read and
    /// round-trip; the player verbs read bare as `By(You, …)`. (Fight is now a
    /// grammar macro over `DealDamage`, not a primitive verb.)
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

        let mill = Action::by_you(PlayerAction::Mill(Count::Literal(3)));
        assert_eq!(read("Mill(Literal(3))"), mill);
        assert_eq!(read(&write(&mill)), mill);

        let venture = Action::by_you(PlayerAction::VentureIntoDungeon);
        assert_eq!(read("VentureIntoDungeon"), venture);
        assert_eq!(read(&write(&venture)), venture);
    }

    /// `Discard`'s `random` flag ([CR#701.9b]) defaults false (omitted on
    /// write) and round-trips when set — "discard a card at random".
    #[test]
    fn discard_random_defaults_and_round_trips() {
        let chosen = read("Discard(count: Literal(1))");
        let Action::By(_, PlayerAction::Discard { random, .. }) = &chosen else {
            panic!("expected Discard, got {chosen:?}");
        };
        assert!(!random, "omitted random defaults to false");
        assert!(!write(&chosen).contains("random"), "false random omitted");

        let random = Action::by_you(PlayerAction::Discard {
            count: Count::Literal(1),
            what: None,
            random: true,
        });
        assert_eq!(read("Discard(count: Literal(1), random: true)"), random);
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

    /// `Composite name body` ([CR#701]) reads flat (the `Act` compartment is
    /// transparent) and round-trips — the keyword-action verb the
    /// scry/surveil/fateseal macros desugar to.
    #[test]
    fn composite_round_trips() {
        let scry = Action::Composite {
            name: crate::Ident::new("Scry"),
            body: Box::new(crate::OneShotEffect::Each(crate::Each {
                binder: crate::Binder::Existing(Selection::TopOfLibrary {
                    count: Count::Literal(2),
                    whose: Reference::You,
                }),
                effect: Box::new(crate::OneShotEffect::Act(Action::Move(
                    Reference::It,
                    Destination::Library(Anchor::FromTop(Count::Literal(0))),
                    vec![],
                ))),
            })),
        };
        assert_eq!(read(&write(&scry)), scry);
    }
}
