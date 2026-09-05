use std::sync::Arc;

use crate::ChosenValueKind;
use crate::Count;
use crate::CounterRef;
use crate::Reference;
use crate::Selection;
use crate::TokenSpec;
use crate::mana::ManaProduction;

/// A position within a library ([CR#401.7]): an offset counted from the top or
/// from the bottom. `FromTop(0)` is the very top, `FromBottom(0)` the very
/// bottom; larger offsets index inward. The Idris `Anchor = FromTop Count |
/// FromBottom Count`.
#[derive(Debug, Clone, PartialEq, Eq, Hash, serde::Deserialize, serde::Serialize)]
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
/// Core RON keeps ordinary zones explicit as `Zone(Graveyard)`. The library is
/// an ordered destination only at an [`Anchor`](crate::Anchor), and the stack
/// is never a move destination ([CR#401.7,405.1]).
#[derive(Debug, Clone, PartialEq, Eq, Hash, serde::Deserialize, serde::Serialize)]
pub enum Destination {
    /// A plain zone change ([CR#400.7]) — written as the bare zone name. The
    /// ordered library and the stack are excluded (see the type docs).
    Zone(crate::Zone),
    /// The library at an anchored position ([CR#401.7]).
    Library(Anchor),
}

/// An entry rider on a relocation/creation verb: how the object arrives on the
/// battlefield ([CR#614.12] — effects that modify how a permanent enters). The
/// rider list rides [`Action::Move`]/[`Action::MoveGroup`]/[`Action::Create`];
/// it is
/// **battlefield-only** — riders on any non-battlefield destination are
/// ill-formed (a card in a graveyard has no tapped/attacking state to arrive
/// in, [CR#110.5,614.12]). Core carries no proofs, so nothing is enforced
/// here; the obligation lives on the semantic mirror
/// (`deckmaste_semantics::EnterRider`).
#[derive(Debug, Clone, PartialEq, Eq, Hash, serde::Deserialize, serde::Serialize)]
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
    /// ([`crate::TokenSpec::Copy`], [`Action::CastCopy`], and the
    /// becomes-a-copy [`crate::continuous::StaticSpec::BecomesCopy`]).
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
#[derive(Debug, Clone, PartialEq, Eq, Hash, serde::Deserialize, serde::Serialize)]
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

/// A game verb ([CR#700,701,608.2]) — one enum spanning both the
/// object/effect-agent verbs (the source deals the damage, the effect
/// destroys/returns the object) and the player-performed verbs (draw,
/// sacrifice, …), merged per the action-role-reshape design (2026-08-01):
/// `Action`/`PlayerAction`'s two-enum split and the `By(Reference,
/// PlayerAction)` wrapper are gone. Every slot appears in printed-sentence
/// order, subject first when the sentence has one (Law 1); a player verb
/// whose CR rule names a performer carries an explicit leading agent slot
/// (`Sacrifice(who, what)`, `DrawCard(who)`); a verb whose CR rule is
/// agent-silent carries none (`Tap`, `Untap`, `PutCounters`,
/// `RemoveCounters`, `Reveal`, `RemoveDamage`). No slot defaults exist on a
/// role field (Law 2) — a bare
/// `Sacrifice(This)` no longer exists; the agent is always spelled
/// (`Sacrifice(You, This)`).
#[derive(Debug, Clone, PartialEq, Eq, Hash, serde::Deserialize, serde::Serialize)]
pub enum Action {
    /// Deal an amount of damage to the patient object or player ([CR#120.1]).
    /// Fields read in printed-sentence order: `source` deals `amount` to
    /// `patient` — `DealDamage(This, 3, It)` for Lightning Bolt. A patient is
    /// not necessarily a [CR#115.1] target. `source` is the
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
    /// semantic/state drift fizzles, never panics).
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
    /// (see [`EnterRider`]).
    ///
    /// The trailing `from` slot is an optional FIZZLE-GUARD ([CR#701.8a]):
    /// when present, the move commits only if the object is CURRENTLY in
    /// that zone — the zone precondition destroy/discard/mill state
    /// declaratively (a permanent no longer on the battlefield when destroy
    /// resolves, a card no longer in hand when the named-card discard
    /// resolves, [CR#701.8a,701.9a,701.17a]) — and a mismatch fizzles the
    /// move silently: no event, no fact, no trigger, never a panic
    /// (semantic/state drift never crashes the engine). Default `None`
    /// (unguarded, the common case) is omitted on write.
    Move(
        Reference,
        Destination,
        #[serde(default = "crate::empty_arc")] Arc<[EnterRider]>,
        #[serde(default)] Option<crate::Zone>,
    ),
    /// Move a GROUP to a destination as one event, with an [`Arrangement`]
    /// fixing how the simultaneous arrivals are ordered ([CR#401.4]) — the
    /// order only EMERGES for a group landing in an ordered position, so the
    /// group verb carries it while single [`Move`](Action::Move) does not.
    /// Brainstorm's "put two cards … on top of your library in any order" =
    /// `MoveGroup(group: That, arrangement: AnyOrder, to:
    /// Library(FromTop(0)))`. `riders` as on [`Move`](Action::Move)
    /// (battlefield-only, [CR#614.12]). A struct variant: four fields exceed
    /// the `serde::Deserialize, serde::Serialize` tuple arity (mirrors
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
    /// Move counters from one object onto another ([CR#122] — counters move
    /// object→object as a single operation, distinct from a separate
    /// remove-then-put). The [`CounterSpec`](crate::CounterSpec) names a
    /// specific kind+count (Power Conduit / Leech Bonder) or `AllKinds`
    /// (Ozolith / Fate Transfer — every counter of every kind at once, the
    /// case single-kind remove+put can't reach atomically); `from`/`to` are
    /// the source and destination objects.
    MoveCounters(crate::CounterSpec, Reference, Reference),
    /// Register a floating replacement effect ([CR#614.1] — a replacement
    /// effect acts "like a shield around whatever [it's] affecting") — "the
    /// next time …" shields (regeneration, one-shot prevention). `subject`
    /// names the affected permanent as a register read; the shield freezes
    /// that resolved identity at creation. Semantics has no `subject:` field —
    /// English leaves the protected permanent to the discourse — so lowering
    /// resolves the anaphor and declares the register here, rather than the
    /// engine searching its register file at resolution (ADR law 4).
    /// `one_shot` consumes the shield on first use ([CR#614.3]).
    /// [CR#701.19a,614.8]
    CreateReplacement {
        subject: Reference,
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
    /// Idris `Composite : KeywordActionSpec b -> Instruction b -> Action
    /// b` and the [`KeywordAbility::Composite`](crate::KeywordAbility) `{
    /// name, abilities }` precedent (a struct variant, read all-named).
    /// Resolving runs `body`, then (when the body actually acts — scry 0
    /// does nothing, [CR#701.22b]) commits the present-tense `Act`
    /// name-fact a "whenever you scry/surveil/discard" trigger reads.
    /// `body` is boxed to break the `Action` → `Instruction` →
    /// `Action` size cycle.
    Composite {
        name: crate::VerbName,
        body: Arc<crate::Instruction>,
    },
    /// Change a player's life total ([CR#119.3,119.9]) — the merged
    /// `GainLife`/`LoseLife`/`SetLife` family (the funnel criterion, spec §6):
    /// every route into the life total is defined as a gain or a loss
    /// ([CR#119.2,119.4,119.5,119.7,119.8]), and [CR#119.9,119.10] name the
    /// player the PATIENT of a source's cause, not an agent. `patient` is the
    /// player whose total changes; [`LifeOp`] carries the direction (`Up`/
    /// `Down`) or the resolution-computed `Set` ([CR#119.5] — resolves as a
    /// gain or loss of the necessary difference; equal totals emit no event).
    /// Paying life is losing life ([CR#119.4]) — cost position spells `Down`.
    /// Cost-eligible for the whole family ([CR#119.7]; Invigorate, Skyshroud
    /// Cutter print gain-life costs).
    ChangeLife(Reference, LifeOp),
    /// Add mana to `recipient`'s mana pool ([CR#106.4]) — the production
    /// carries optional unit riders ([CR#106.6]).
    AddMana(Reference, Count, ManaProduction),
    /// `agent` creates `count` token permanents ([CR#111.1,701.7,701.7a] —
    /// "target player creates" is real information, so the creator is an
    /// explicit slot). The `riders` list ([`EnterRider`], default `[]`,
    /// omitted when empty) spells arrival state — "create a … token tapped
    /// and attacking" ([CR#508.4]); a created token always enters the
    /// battlefield, so every rider is legal here ([CR#614.12]). A struct
    /// variant: three required fields plus one trailing default exceed the
    /// `serde::Deserialize, serde::Serialize` tuple-default layout (mirrors
    /// [`MoveGroup`](Action::MoveGroup)).
    Create {
        agent: Reference,
        count: Count,
        token: TokenSpec,
        #[serde(default, skip_serializing_if = "crate::slice_is_empty")]
        riders: Arc<[EnterRider]>,
    },
    /// `agent` sacrifices `what` ([CR#701.21a] — "a player can sacrifice a
    /// permanent only if they control it"; a player can sacrifice only what
    /// they control, so `agent` is that permanent's controller).
    Sacrifice(Reference, Reference),
    /// "`agent` draws a card" ([CR#121.1]) — exactly ONE card ([CR#121.2]).
    /// "Draw N" is [`Instruction::draw`](crate::Instruction::draw) —
    /// `Batch(n, Act(DrawCard(who)))` — where the `Batch` is the *instruction*
    /// level a count-referring replacement modifies ([CR#121.2a]) and each
    /// element is one individual card draw ([CR#121.2]). The late
    /// top-of-library bind and the empty-library loss ([CR#121.4,104.3c]) are
    /// the engine's.
    ///
    /// Deliberately NOT [`Move`](Action::Move), and NOT a
    /// [`Composite`](Action::Composite) over one. Drawing is irreducible:
    /// [CR#121.5] states that moving cards from a library to a hand *without*
    /// the word "draw" is not a draw, so a Library → Hand body would denote a
    /// DIFFERENT event — missing draw triggers, draw replacements ([CR#121.6]),
    /// and the empty-library loss. Nor is drawing a keyword action: [CR#701]
    /// enumerates those and drawing is not among them; it is [CR#121], a
    /// game action, the same tier as damage ([CR#120]).
    ///
    /// Named `DrawCard`, not `Draw`: the `Draw(N)` authoring macro
    /// (`plugins/builtin/macros/action/Draw.ron`) owns the bare `Draw` ident
    /// — its own body is `Batch(Param(0), DrawCard(You))`, so `DrawCard`
    /// keeps the macro's *own* body unambiguous between the variant
    /// (`DrawCard(You)`, one required `Reference` slot) and the macro
    /// invocation (`Draw(1)`, one `Count` argument): same position, but the
    /// two read different argument shapes, never the same arity+type. Keep
    /// any new bare variant's name distinct from every macro name regardless.
    DrawCard(Reference),
    /// Tap the referenced object ([CR#701.26a]).
    Tap(Reference),
    /// Untap the referenced object ([CR#701.26b]).
    Untap(Reference),
    /// "`recipient` gets an emblem with [abilities]" — a command-zone object
    /// that never touches the battlefield (rules-taxonomy §6: a degenerate
    /// token definition; [CR#114.1,114.4]).
    GetEmblem(Reference, Arc<[crate::Ability]>),
    /// "`recipient` gets the named designation" ([CR#702.131c] — the city's
    /// blessing; the generic player-scope grant verb). v1 handles the
    /// **player-scope flag** case only; single-holder designations (monarch /
    /// the initiative — they evict the prior holder) and object-scope grants
    /// (goad, suspected) are seams the verb does not yet cover.
    GetDesignation(Reference, crate::Ident),
    /// Set a GAME-scope enum designation to the named value. Day/night is the
    /// founding consumer: `SetGameDesignation("DayNight", "Day")` replaces
    /// the mechanic-specific `BecomeDay` action ([CR#731.1]). Both names are
    /// open vocabulary backed by a [`crate::DesignationDecl`]; this primitive
    /// is not specific to day/night.
    SetGameDesignation(crate::Ident, crate::Ident),
    /// `who` makes a resolution choice stored under a note key ([CR#608.2d]
    /// choice + [CR#607.2] slot): "choose a color" and kin. `kind` narrows to
    /// the CHOSEN-VALUE kinds ([`ChosenValueKind`]) — the persisted
    /// object-set kinds ([`crate::NotedKind`]) stay store-side with their
    /// writers, not this choice node.
    ChooseValue(Reference, ChosenValueKind, crate::Ident),
    /// `controller` puts a copy of `spec` on the stack ([CR#707.10] — a copy
    /// on the stack, NOT casting one; [CR#707.12] casting is
    /// [`CastCopy`](Action::CastCopy)). `spec` is the shared [`CopySpec`]
    /// (source + "except" exceptions, [CR#707.9]) — the fifth copy-delivery
    /// site, joining [`TokenSpec::Copy`](crate::TokenSpec), `CastCopy`,
    /// [`EnterRider::AsCopy`], and
    /// [`StaticSpec::BecomesCopy`](crate::continuous::StaticSpec);
    /// Fork's "except that the copy is red" ([CR#707.10]'s own founding
    /// example) becomes spellable. `retarget` is a CREATION mode
    /// ([`CopyRetarget`]), not a post-hoc edit — [CR#707.10c]: "the copy is
    /// put onto the stack with those targets", never with the old ones first.
    CopySpell {
        controller: Reference,
        spec: crate::CopySpec,
        retarget: CopyRetarget,
    },
    /// "`agent` casts a copy of `spec`" ([CR#707.12]) — NOT `CopySpell`'s
    /// stack-copy: this follows the full [CR#601.2a..601.2h] casting
    /// pipeline, created in `spec`'s source's own zone and cast while another
    /// spell or ability resolves ([CR#707.12]), so it passes through
    /// legality, costs, and targeting like any other cast rather than
    /// skipping straight to the stack. The engine's resolve arm currently
    /// fizzles without producing events; it never panics.
    CastCopy(Reference, crate::CopySpec),
    /// "`agent` may cast `what`" as an effect ([CR#608.2g]) — the
    /// resolution-time cast primitive. `agent` casts the referenced object by
    /// following the [CR#601.2a..601.2i] casting steps (the same pipeline
    /// `legal.rs`/`cast.rs` drive for a hand cast), EXCEPT no player receives
    /// priority after it's cast: the cast spell becomes the topmost object on
    /// the stack and the currently-resolving spell or ability continues to
    /// resolve ([CR#608.2g]). The effect GRANTS the permission ([CR#608.2g] —
    /// "specifically instructs or allows"), so this cast bypasses the normal
    /// timing/zone [`DeonticAction::Cast`] gate for `what`; the card is moved
    /// to the stack "from where it is" ([CR#601.2a]). `what` is bound by the
    /// surrounding effect; `May { who: agent, effect: Act(Cast(agent,
    /// <that card>, None)), if_not: <else> }` offers its "yes" branch only
    /// when a legal, payable cast exists ([CR#608.2g] — the offer is empty
    /// otherwise, so the `if_not` branch runs). A reference that resolves to
    /// no castable object fizzles (semantic-input errors never crash the
    /// engine).
    ///
    /// The trailing slot is an optional ALTERNATIVE COST ([CR#118.9,702.35a]):
    /// when present, the cast pays this cost RATHER THAN the card's mana cost —
    /// madness's "cast it by paying its madness cost" is `Cast(You,
    /// That(Card), [Mana(…)])`. A trailing-default tuple slot preserves the
    /// bare `Cast(You, That(Card))` serialization (the alt-cost slot is a
    /// data default, not a role default — Law 2's scope note), with
    /// `SinglePlusDefault` distinguishing the required-plus-default tuple
    /// from a newtype.
    Cast(Reference, Reference, #[serde(default)] Option<crate::Cost>),
    /// `by` picks new targets for the stack object `of`, bound by its
    /// original targetspec ([CR#115.7a..115.7d,707.10c] — Bolt Bend,
    /// Redirect, copy-with-new-targets). `mode` carries the four-way
    /// discriminant ([`RetargetMode`]) — Bolt Bend (`ChangeOne`) and Redirect
    /// (`ChangeAny`) are different modes, not the same verb. Each target slot
    /// a chosen mode may LEAVE UNCHANGED even if the current target is
    /// illegal; a CHANGED slot must pick a legal target ([CR#707.10c]). Scope:
    /// EXISTING stack objects only — a copy's targets are chosen at creation
    /// ([`CopyRetarget`]), never routed through here.
    Retarget {
        mode: RetargetMode,
        of: Reference,
        by: Reference,
    },
    /// `agent` flips that many coins ([CR#705.1]); the win/loss result rides
    /// the pushed `amountAnte` antecedent, like `RollDice`. `called` splits
    /// [CR#705.2]'s two kinds: `true` = the flipper calls heads/tails and
    /// wins or loses the flip; `false` = the effect reads only
    /// heads/tails and no player wins or loses.
    FlipCoins(Reference, Count, bool),
    /// `agent` rolls that many dice with the given number of sides
    /// ([CR#706.1]); an IGNORED roll is considered to have never happened
    /// ([CR#706.6]).
    RollDice(Reference, Count, crate::Uint),
    /// `agent` rolls the (Planechase) planar die as a special action
    /// ([CR#901.9]). NO numeric result ([CR#901.9d]) — unlike `RollDice`/
    /// `FlipCoins` this introduces no numeric result for a later pinned read.
    RollPlanarDie(Reference),
    /// Put counters of the named kind on the referenced object/player
    /// ([CR#122.1] — counters go on objects AND players). The kind is a bare
    /// `CounterRef` (`PutCounters(~, P1P1Counter, 2)`), not a string.
    PutCounters(Reference, crate::CounterRef, Count),
    /// Remove counters of the named kind ([CR#122.1]; cost-eligible —
    /// "Remove a +1/+1 counter from this creature:").
    RemoveCounters(Reference, crate::CounterRef, Count),
    /// "`patient` wins the game" ([CR#104.2b]) — immediate on resolution,
    /// suppressed by a matching `CantWin` outcome gate ([CR#101.2]
    /// precedence; the last-player-standing win [CR#104.2a] never rides
    /// this verb and pierces gates).
    WinGame(Reference),
    /// "`patient` loses the game" ([CR#104.3e]) — immediate on resolution,
    /// suppressed by a matching `CantLose` outcome gate. A player who
    /// would win and lose simultaneously loses ([CR#104.3f]).
    LoseGame(Reference),
    /// Restart the game ([CR#727.1]) — a TERMINAL with explicit carryover
    /// (every card involved comes along, ownership unchanged [CR#727.2];
    /// effects may exempt cards [CR#727.5]; trailing instructions execute
    /// just before the new first untap step [CR#727.4]), never a state
    /// reset. Slotless (Law 3): the actor binding IS [CR#727.1a] — the
    /// restarting effect's controller (cause-context, not a role slot)
    /// starts the new game. The restarted game ends with no winner, loser,
    /// or draw. Subgames ([CR#729]) are a different, deferred concept (a
    /// context push, not a restart).
    RestartGame,
    /// Shuffle a collection ([CR#701.24a]: "to shuffle **a library or a
    /// face-down pile of cards**") — the collection patient, not an agent;
    /// the owner is derivable from the collection term (Law 3).
    /// [`Selection::LibraryOf`](crate::Selection) names a whole library; a
    /// temporary pile is read through a pile-valued [`Selection::Reg`]. An
    /// INFORMATION event too: order knowledge is destroyed for everyone, and
    /// revealed cards in the library stop being revealed and become new
    /// objects ([CR#701.20d]). Why library actions never rewind: [CR#733.1].
    Shuffle(Selection),
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
    /// Pay a cost as an action ([CR#118.12]) — a slotless `Cost` → `Action`
    /// adapter; the payer is rule-forced by every legal host, never a slot
    /// (spec §5): `May.who` is the payer ([CR#118.12a] — the doer decides),
    /// an ability's announced additional cost is paid by its controller
    /// ([CR#601.2b]), and `Do(…)`'s payer is the cost's payer by construction.
    /// Bare effect-position `Pay` is unspellable (no legal host); the engine's
    /// internal `decide::Decision::Pay` is an unrelated, non-serialized type.
    Pay(crate::Cost),
}

/// The operand of [`Action::ChangeLife`] ([CR#119.3,119.5]) — mirrors
/// [`NumericOp`](crate::continuous::NumericOp)'s naming (`Set`/`Up`/`Down`)
/// and op↔axis soundness-gate pattern, but is a distinct type: `Set` carries
/// a plain [`Count`] (no CDA markers on life), and it is a one-shot event
/// payload, not a layer-7 continuous op. The bare `Set`/`Up`/`Down` idents
/// are positionally established (`NumericOp`, `Face`) — safe here too: a
/// dedicated, non-flattened positional field, so dispatch is by static field
/// type, not name collision with `NumericOp::Set`/`CollectionOp::Set`/
/// `Face::Up`/`Down`.
#[derive(Debug, Clone, PartialEq, Eq, Hash, serde::Deserialize, serde::Serialize)]
pub enum LifeOp {
    /// Set the total to N ([CR#119.5]): resolves as a gain or loss of the
    /// necessary difference — triggers see the gain/loss, never a "set"
    /// event. Equal totals = no event (transition-only).
    Set(Count),
    /// Gain N life ([CR#119.3]).
    Up(Count),
    /// Lose N life ([CR#119.3]) — pay-life when in a cost ([CR#119.4]: paying
    /// life IS losing life).
    Down(Count),
}

/// The [CR#115.7a..115.7d] four-way discriminant [`Action::Retarget`]
/// collapsed — Bolt Bend and Redirect are different modes of the same verb,
/// not different verbs.
#[derive(Debug, Clone, PartialEq, Eq, Hash, serde::Deserialize, serde::Serialize)]
pub enum RetargetMode {
    /// "Change all targets of target spell or ability" ([CR#115.7a]) — every
    /// target changes to a legal one, or NONE change.
    ChangeAll,
    /// "Change target of target spell or ability" ([CR#115.7b]) — Bolt Bend:
    /// exactly one target slot changes.
    ChangeOne,
    /// "Change any number of targets of target spell or ability"
    /// ([CR#115.7c]) — Redirect: zero or more target slots change.
    ChangeAny,
    /// "Choose new targets for target spell or ability" ([CR#115.7d]) — any
    /// target slot may be left unchanged even if currently illegal; a
    /// changed slot must be legal.
    ChooseNew,
}

/// A copy's retarget mode ([CR#707.10c]) — a CREATION mode on
/// [`Action::CopySpell`], never a post-hoc edit: "Once the player has decided
/// what the copy's targets will be, the copy is put onto the stack with
/// those targets" — the copy never exists with old targets, so composing
/// with [`Action::Retarget`] would be operationally mis-sequenced. No
/// default — `AsIs` is spelled (Law 2).
#[derive(Debug, Clone, PartialEq, Eq, Hash, serde::Deserialize, serde::Serialize)]
pub enum CopyRetarget {
    /// The copy keeps the original's targets unchanged.
    AsIs,
    /// "You may choose new targets for the copy" — the near-universal form;
    /// the offering sentence is a frame fact, not part of this mode.
    MayChooseNew,
    /// All target slots retarget to the named object ([CR#707.10e]) — the
    /// copy is NOT CREATED if that object is illegal for any slot
    /// (replacement-multiplicity carve; the [CR#707.10d] for-each-could-target
    /// family composes `Each(InChosenOrder(ValidTargetsFor(spell), …),
    /// CopySpell(…, TargetsThat(It)))` rather than living here as a mode).
    TargetsThat(Reference),
}

impl Action {
    /// `DealDamage` from the implicit source (`This`) — the common case, where
    /// the dealer is the ability's source object / the resolving spell. The
    /// enum form spells `source` explicitly (`DealDamage(This, amount,
    /// patient)`); this ctor fills it in.
    #[must_use]
    pub fn deal_damage(patient: Reference, amount: Count) -> Action {
        Action::DealDamage(Reference::Reg(crate::RefId(0)), amount, patient)
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
            body: Arc::new(crate::Instruction::act(Action::move_to(
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
    /// [`Instruction::mill`](crate::Instruction::mill) — `Batch(count,
    /// Act(Mill))` — and the engine commits the whole slice as ONE simultaneous
    /// batch of per-card, individually-redirectable moves ([CR#701.17a,616.1]),
    /// NOT the per-card sequence a draw is ([CR#121.2]).
    #[must_use]
    pub fn mill_one(who: Reference) -> Action {
        Action::Composite {
            name: crate::VerbName::from("Mill"),
            body: Arc::new(crate::Instruction::act(Action::MoveGroup {
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

    /// One card of a draw ([CR#121.1,121.2]) — the PER-UNIT action a draw's
    /// `Batch` contains: `DrawCard(who)`, the agent in a real slot.
    ///
    /// This is deliberately NOT a [`Composite`](Action::Composite). Drawing is
    /// [CR#121], a game action, not one of the keyword actions [CR#701]
    /// enumerates; and it has no honest body — [CR#121.5] states that a
    /// Library → Hand move made *without* the word "draw" is not a draw, so
    /// such a body would denote a different event. See
    /// [`Action::DrawCard`]. A whole draw is
    /// [`Instruction::draw`](crate::Instruction::draw) — `Batch(count,
    /// Act(DrawCard(who)))`, `count` SEQUENTIAL single-card draws ([CR#121.2],
    /// each seeing prior state), UNLIKE mill's one simultaneous batch. The
    /// engine's `Act(DrawCard)` apply binds the library top LATE and
    /// empty-checks BEFORE its move ([CR#121.4,104.3c]).
    #[must_use]
    pub fn draw_one(who: Reference) -> Action {
        Action::DrawCard(who)
    }

    /// "`who` discards `count`" ([CR#701.9a]) — the keyword action as data:
    /// a [`Composite`](Action::Composite) named `"Discard"` whose body IS
    /// the executor (unlike draw/mill's flat-coordinate bodies): a
    /// explicit choose-then-act instructions over `who`'s hand
    /// ([CR#701.9b] — the affected player chooses by default).
    /// `random: true` uses [`Selection::Random`] over the same hand filter —
    /// no choice exists,
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
        let dest = crate::DefId(0);
        let over = if random {
            Selection::Random(
                quantity.clone(),
                Arc::new(crate::Region::candidate(filter.clone())),
            )
        } else {
            Selection::Reg(dest.into())
        };
        let mut instructions = Vec::new();
        if !random {
            instructions.push(crate::Instruction::Choose(crate::Choose {
                dest,
                by: who,
                quantity,
                filter: Arc::new(crate::Region::new(Arc::from([]), filter)),
            }));
        }
        instructions.push(crate::Instruction::Each(crate::Each {
            over,
            body: crate::Region::new(
                Arc::from([crate::Param {
                    def: crate::DefId(0),
                    kind: crate::Kind::Entity,
                    provenance: crate::Provenance::LoopElement,
                }]),
                crate::Instruction::act(Action::discard_what(Reference::Reg(crate::RefId(0))))
                    .into(),
            ),
        }));
        Action::Composite {
            name: crate::VerbName::from("Discard"),
            body: Arc::new(crate::Instruction::Sequentially(instructions.into())),
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
            body: Arc::new(crate::Instruction::act(Action::move_to(
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
/// `by` field to read (unlike an explicit [`crate::Choose`] instruction).
fn hand_owner_ref(filter: &crate::Predicate) -> Option<&Reference> {
    use crate::Predicate as P;
    match filter {
        P::And(parts) => parts.iter().find_map(hand_owner_ref),
        P::Relation(crate::RelationPredicate::Owner(inner)) => match inner.as_ref() {
            P::Ref(r) => Some(r),
            _ => None,
        },
        _ => None,
    }
}

/// The BOUND-form patient of a discard composite's stored body
/// ([CR#702.29a] "discard this card"): the reference its body's HEAD moves,
/// when that head is a single relocation (`Move(This, Graveyard)` →
/// `Some(This)`). The chosen/random form (an explicit choose or random
/// selection followed by an action, [`Action::discard`]) → `None`. Read off
/// the stored body ("matches the
/// expanded body") — shared by the engine's resolve lane, the renderer, and
/// the Idris emitter, so the three can never disagree on which form a
/// discard is.
#[must_use]
pub fn discard_body_what(body: &crate::Instruction) -> Option<&Reference> {
    use crate::Instruction as Ose;
    match body {
        Ose::Act {
            action: Action::Move(what, Destination::Zone(_), _, _),
            ..
        } => Some(what),
        _ => None,
    }
}

/// Whether a discard composite's stored body selects AT RANDOM
/// ([CR#701.9b]): its iterator uses [`Selection::Random`] over the hand
/// filter, rather than an explicit [`crate::Choose`]. The at-random detail
/// belongs to the selection, not the tag — shared like
/// [`discard_body_what`].
#[must_use]
pub fn discard_body_random(body: &crate::Instruction) -> bool {
    use crate::Instruction as Ose;
    match body {
        Ose::Each(crate::Each {
            over: Selection::Random(..),
            ..
        }) => true,
        Ose::Sequentially(parts) => matches!(
            parts.first(),
            Some(Ose::Each(crate::Each {
                over: Selection::Random(..),
                ..
            }))
        ),
        _ => false,
    }
}

/// The `count` of a CHOSEN/RANDOM discard composite's stored body
/// ([CR#701.9b]) — the upper bound of an explicit `Choose` instruction's or
/// `Selection::Random`'s `Quantity`. `None` for a bound single-move body
/// (`discard this card`, always one card). Read off the stored body, sharing
/// the descent with [`discard_body_what`]/[`discard_body_random`] so
/// re-agenting a discard cost ([CR#601.2h]) reads its count without a typed
/// atom.
#[must_use]
pub fn discard_body_count(body: &crate::Instruction) -> Option<&Count> {
    use crate::Instruction as Ose;
    match body {
        Ose::Each(crate::Each {
            over: Selection::Random(quantity, _),
            ..
        }) => quantity.bounds().1,
        Ose::Sequentially(parts) => parts.iter().find_map(|part| match part {
            Ose::Choose(choice) => choice.quantity.bounds().1,
            Ose::Each(crate::Each {
                over: Selection::Random(quantity, _),
                ..
            }) => quantity.bounds().1,
            _ => None,
        }),
        _ => None,
    }
}

/// The discarding performer of a CHOSEN/RANDOM discard composite's stored
/// body ([CR#701.9b]) — [`crate::Choose`]'s `by` (the chooser IS the
/// discarding player by default), or, for the at-random
/// form (which carries no `by`), the hand-owning `who` its
/// [`Selection::Random`] filter names ([`hand_owner_ref`]). `None` for a
/// bound single-move body (its performer rides the patient, not a separate
/// slot) or any other shape. Read off the stored body — shared by the
/// engine's resolve lane, the renderer, and the Idris emitter.
#[must_use]
pub fn discard_body_whose(body: &crate::Instruction) -> Option<&Reference> {
    use crate::Instruction as Ose;
    match body {
        Ose::Each(crate::Each {
            over: Selection::Random(_, filter),
            ..
        }) => hand_owner_ref(&filter.body),
        Ose::Sequentially(parts) => parts.iter().find_map(|part| match part {
            Ose::Choose(choice) => Some(&choice.by),
            Ose::Each(crate::Each {
                over: Selection::Random(_, filter),
                ..
            }) => hand_owner_ref(&filter.body),
            _ => None,
        }),
        _ => None,
    }
}

/// The two fighters a `Fight` body names — the sources of its reciprocal
/// `DealDamage` halves under the `If`-guard's `then` batch ([CR#701.14a]).
/// ONE shared read-off-the-body facet (the `discard_body_what` precedent):
/// the engine's resolve lane and the Idris emitter both consume this, so
/// the fighters are read off the stored body identically everywhere.
#[must_use]
pub fn fight_body_fighters(body: &crate::Instruction) -> Option<(&Reference, &Reference)> {
    use crate::Action as A;
    use crate::Instruction as Ose;
    match body {
        Ose::If(iff) => fight_body_fighters(&iff.then),
        Ose::Simultaneously(parts) => match parts.as_ref() {
            [
                Ose::Act {
                    action: A::DealDamage(a, _, _),
                    ..
                },
                Ose::Act {
                    action: A::DealDamage(b, _, _),
                    ..
                },
                ..,
            ] => Some((a, b)),
            _ => None,
        },
        _ => None,
    }
}

impl Action {
    /// Whether this action may appear in a cost (`CostComponent::Do`): the
    /// payer performs it, nothing targets ([CR#601.2b..601.2c]) — the ONE
    /// merged table (former `PlayerAction`/`Action` twins collapsed with the
    /// wrapper). Cost-eligible verbs are the self-directed ones a player can
    /// pay with — sacrifice, relocate (`Move`, e.g. self-exile to pay —
    /// [CR#701.13]), tap, untap, discard (the keyword-action composite),
    /// change-life (pay-life; [CR#119.7] — Invigorate, Skyshroud Cutter print
    /// gain-life costs, so the WHOLE `ChangeLife` family is eligible, not
    /// just the losing direction), reveal ("reveal a blue card from your
    /// hand:" — a reveal that is part of a cost stays shown until the spell
    /// leaves the stack, [CR#701.20a]), and put/remove counters. Counter
    /// costs ride these two verbs rather than a dedicated cost verb: "remove a
    /// +1/+1 counter:" is `RemoveCounters`, "pay {E}" is `RemoveCounters` on a
    /// player, and a planeswalker loyalty ability's `+N`/`−N` cost is
    /// `PutCounters`/`RemoveCounters` of the loyalty counter on its source
    /// ([CR#606.4] — the cost to activate a loyalty ability is to put on or
    /// remove that many loyalty counters). The Idris grammar makes the same
    /// call: loyalty costs reuse `Do (PutCounters/RemoveCounters
    /// loyaltyCounter N This)`, "so there is no duplicate counter-cost
    /// verb" (`idris/src/Semantics.idr`, the `Cost` `Do` and
    /// `PutCounters`/`RemoveCounters` doc comments).
    #[must_use]
    pub fn is_cost_eligible(&self) -> bool {
        match self {
            Action::Sacrifice(..)
            | Action::Move(..)
            | Action::Tap(_)
            | Action::Untap(_)
            | Action::ChangeLife(..)
            | Action::PutCounters(..)
            | Action::RemoveCounters(..)
            | Action::Reveal { .. } => true,
            // The ONE cost-eligible composite ([CR#701.9,702.29a]): the
            // payer performs it, nothing targets ([CR#601.2b..601.2c]).
            Action::Composite { name, .. } => name.as_str() == "Discard",
            _ => false,
        }
    }

    /// Whether this instruction produces a numeric result that is known only
    /// after it runs. The result is written to an `Act` destination register:
    /// heads/wins for coin flips, the sum of die results, or the number of
    /// cards a multi-card discard actually moved.
    #[must_use]
    pub fn produces_runtime_magnitude(&self) -> bool {
        match self {
            Self::FlipCoins(..) | Self::RollDice(..) => true,
            Self::Composite { name, body } => {
                name.as_str() == "Discard" && discard_body_count(body).is_some()
            }
            _ => false,
        }
    }
}
