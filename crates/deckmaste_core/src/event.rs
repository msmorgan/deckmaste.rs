use std::sync::Arc;

use serde::Deserialize;
use serde::Serialize;

use crate::Condition;
use crate::CountBound;
use crate::CounterRef;
use crate::Ident;
use crate::Lookback;
use crate::Predicate;
use crate::Reference;
use crate::Uint;
use crate::Zone;

/// A phase KIND — the five phases of a turn without their step breakdown
/// ([CR#500.1]). The grain effects that add whole phases speak at
/// ([CR#500.8], `Action::ExtraPhase` — "an additional combat phase");
/// [`PhaseStep`] names a phase-AND-step position instead. Extra STEPS
/// ([CR#500.9]) accrete a step-grained twin when a card needs one.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub enum PhaseKind {
    /// The beginning phase ([CR#501]).
    Beginning,
    /// The precombat main phase ([CR#505]).
    PrecombatMain,
    /// The combat phase ([CR#506]).
    Combat,
    /// The postcombat main phase ([CR#505]).
    PostcombatMain,
    /// The ending phase ([CR#512]).
    Ending,
}

/// A turn step, carried by its phase (the 5xx turn structure). `StepBegins`
/// triggers key off these. Each phase carries its constituent step(s); a
/// phase that is a single step (the main phases — [CR#505.1]) is a bare
/// variant. Nested enums round-trip in RON as `Beginning(Upkeep)`,
/// `Combat(DeclareAttackers)`, `PostcombatMain`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub enum PhaseStep {
    /// The beginning phase ([CR#501]): untap, upkeep, draw.
    Beginning(BeginningStep),
    /// The precombat main phase ([CR#505], one step).
    PrecombatMain,
    /// The combat phase ([CR#506]): its five (or six, with first strike) steps.
    Combat(CombatStep),
    /// The postcombat main phase ([CR#505], one step).
    PostcombatMain,
    /// The ending phase ([CR#512]): end step, cleanup.
    Ending(EndingStep),
}

/// The steps of the beginning phase ([CR#501,502,503]).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub enum BeginningStep {
    /// [CR#502].
    Untap,
    /// [CR#503].
    Upkeep,
    /// [CR#504].
    Draw,
}

/// The steps of the combat phase ([CR#506,507,508,509,510,511]).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub enum CombatStep {
    /// [CR#507].
    BeginningOfCombat,
    /// [CR#508].
    DeclareAttackers,
    /// [CR#509].
    DeclareBlockers,
    /// The first combat-damage step ([CR#510.4]), present only when an
    /// attacker or blocker has first strike or double strike.
    FirstCombatDamage,
    /// [CR#510].
    CombatDamage,
    /// [CR#511].
    EndOfCombat,
}

/// The steps of the ending phase ([CR#512,513,514]).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub enum EndingStep {
    /// [CR#513].
    End,
    /// [CR#514].
    Cleanup,
}

/// Whose turn a step-based trigger watches ([CR#503.1] "your upkeep", etc.).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub enum WhoseTurn {
    /// The controller's own turn.
    Your,
    /// Every player's turn of that step.
    EachPlayers,
    /// An opponent's turn.
    AnOpponents,
}

/// The state a `StateBecame` transition watches ([CR#603.2e]) — transitions
/// of an object's own status. Combat onsets are their own master forms
/// (`AttackDeclared`/`BlockDeclared`), control changes are `ControlChanged`,
/// designation gains are `DesignationChanged`: this enum carries only the
/// residual status deltas.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub enum StateChange {
    /// Becomes tapped ([CR#603.2e]).
    Tapped,
    /// Becomes untapped ([CR#603.2e]).
    Untapped,
    /// Phases out/in ([CR#702.26b] — a status change, explicitly NOT a
    /// zone change; a phased-out permanent is treated as though it
    /// doesn't exist).
    Phased(crate::Phasing),
    /// Is turned to the given face ([CR#708]; on turn-up, copiable values
    /// revert and ETB abilities don't fire again, [CR#708.8]).
    TurnedFace(crate::Face),
    /// Turned to its other face by transform/convert
    /// ([CR#701.27a,701.28a]) — front↔back, distinct from the morph
    /// `TurnedFace` ([CR#708]).
    Transformed,
}

/// The machinery that demanded an event — the cause triple's AGENCY
/// coordinate (mtg-rules events.md §3). Closed CR vocabulary.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub enum Agency {
    /// {T}/cost components being paid ([CR#107.5], "sacrificed to pay").
    CostPayment,
    /// The declare-attackers procedure's tap ([CR#508.1f] — explicitly
    /// not a cost).
    AttackDeclaration,
    /// An effect's instruction ("tap target creature", [CR#701.26a]).
    EffectInstruction,
    /// A turn-based action ([CR#703] — cleanup discard, phasing).
    TurnBasedAction,
    /// A state-based action ([CR#704] — lethal damage, deathtouch).
    StateBasedAction,
    /// A resolving mana ability ("tapped for mana", [CR#106.12]).
    ManaAbilityResolution,
    /// A special action ([CR#116.2] — land plays, foretell, …). NOTE: absent
    /// from the skill's events.md §3 agency list (erratum filed); the land
    /// play view ([CR#701.18a]) is unrepresentable without it.
    SpecialAction,
}

/// A cause VERB NAME — a bareword name-atom drawn from the closed grammar
/// vocabulary behind "sacrificed"/"destroyed"/"discarded" views (and the
/// present-tense `Act`'s verb tag). Each verb's fact form is one CR-cited row
/// of the emitted entailment table
/// (`crates/deckmaste_plugin/tables/entailments.ron`): `Sacrifice` entails
/// `ZoneChange { from: Battlefield, to: Graveyard }` [CR#701.21a], `Mill`
/// entails `ZoneChange { from: Library, to: Graveyard }` [CR#701.17a] — so
/// `Dies` matches a sacrifice structurally, and a cause-narrowed pattern admits
/// exactly its verb's occurrences.
///
/// An [`Ident`]-backed newtype (the retired `CauseVerb` closed enum, folded
/// into the one keyword-action/cause NAME namespace) authored BAREWORD, exactly
/// like [`CostTag`](crate::CostTag)/[`KeywordRef`](crate::KeywordRef) — a use
/// reads `Cause(verb: Destroy)`, never `verb: "Destroy"`. Typo-safety is no
/// longer the type's job (any bareword parses) but the GATE's: the
/// `entailments.ron`-membership check rejects a verb outside the closed vocab
/// (`no_dead_grammar`'s `cause_verbs_are_entailment_rows`).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct VerbName(pub Ident);

impl VerbName {
    /// The canonical fact-side spelling — the engine's cause triples and `Act`
    /// verb tags store this string; a pattern verb matches a fact verb by it.
    #[must_use]
    pub fn as_str(&self) -> &'static str {
        self.0.as_str()
    }
}

impl From<&str> for VerbName {
    fn from(s: &str) -> Self {
        VerbName(s.into())
    }
}

impl Serialize for VerbName {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        // A unit variant writes as a bare identifier in RON.
        serializer.serialize_unit_variant("VerbName", 0, self.0.as_str())
    }
}

impl<'de> Deserialize<'de> for VerbName {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        // A bare identifier is a unit enum variant in the serde data model —
        // the same channel `CostTag`/`KeywordRef` read through.
        struct NameVisitor;
        impl<'de> serde::de::Visitor<'de> for NameVisitor {
            type Value = VerbName;
            fn expecting(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                f.write_str("a cause/act verb name (bare identifier)")
            }
            fn visit_enum<A: serde::de::EnumAccess<'de>>(
                self,
                data: A,
            ) -> Result<Self::Value, A::Error> {
                use serde::de::VariantAccess;
                let (ident, variant) = data.variant_seed(crate::IdentSeed)?;
                variant.unit_variant()?;
                Ok(VerbName(ident))
            }
        }
        deserializer.deserialize_enum("", &[], NameVisitor)
    }
}

/// A trigger-side predicate over an event's cause triple (verb, agency,
/// agent). Every omitted coordinate matches anything. `verb` names the
/// entailed view ([`VerbName`] — `Destroy`, `Sacrifice`, …); `agent`
/// filters the causing object/controller (Karmic Justice's "a spell or
/// ability an opponent controls"). Agent-IDENTITY equality ("destroyed this
/// way") is a binding concern, not a pattern — it rides the event log.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub struct CausePattern {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub verb: Option<VerbName>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub agency: Option<Agency>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub agent: Option<Predicate>,
}

/// The cause-narrowing position on an event pattern. One variant today —
/// the conjunctive [`CausePattern`] — but the position is an ENUM on
/// purpose: boolean structure over cause coordinates ("destroyed or
/// sacrificed", "not by a spell") is expressible in English and within
/// the rules; that no card demands it yet is just the current state of
/// things. When one does, `AnyOf`/`Not` variants accrete HERE without
/// respelling existing files. RON requires enum variant names, so the
/// position always reads `Cause(verb: Destroy)` — never a bare tuple.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub enum Cause {
    /// Every PRESENT coordinate must match (the conjunction).
    Cause(CausePattern),
}

/// The unified event-query language ([CR#603.2] and kin): master-form
/// records over the one engine fact record, plus a lane-gated algebra.
///
/// Each master form carries ONLY the fields its kind supplies — dead facets
/// (`what:` on a step event, an amount on a designation change) are
/// unrepresentable by construction, not checker-caught. Forms lower to a
/// field-atom normal form over the single engine fact record `{kind, object,
/// patient, actor, source, from, to, cause, amount, counter, batch, before,
/// after, time}`, evaluated by the engine's ONE lane-parameterized
/// evaluator (`deckmaste_engine`'s `eval.rs`).
/// Declared event names (`Dies`, `Enters`, `Sacrificed`) are macros over
/// these forms. Omitted filter fields default to match-anything; omitted
/// `Option` refinements are unconstrained.
///
/// The algebra tail (`AllOf`/`OneOf`/`Not`/`OneOrMore`/`Nth`/`When`/
/// `Within`) is LANE-GATED by the Idris model (an ill-placed atom is
/// unrepresentable there): live lanes
/// (trigger/replacement/duration) refuse `Within`; history lanes
/// (`Happened`/`EventCount`) refuse `OneOrMore`; disjunction pairs kind
/// with filters per-disjunct and must bottom out in master forms in
/// kind-anchored lanes.
#[derive(Debug, Clone, PartialEq, Eq, Hash, serde::Deserialize, serde::Serialize)]
pub enum EventFilter {
    /// An object changed zones ([CR#603.6]). `Dies` = `from: Battlefield,
    /// to: Graveyard` is a prelude macro over this. `cause` narrows by the
    /// cause triple ("destroyed" admits exactly two causes, [CR#701.8b];
    /// "sacrificed" is never destruction, [CR#701.21a]); omitted = any
    /// cause ("dies", [CR#700.4]). Each [`VerbName`]'s entailed fact form
    /// is an emitted entailment-table row.
    ZoneChange {
        #[serde(default = "Predicate::any")]
        what: Predicate,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        from: Option<Zone>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        to: Option<Zone>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        cause: Option<Cause>,
    },
    /// Damage was dealt ([CR#120.1]): `source` is the damage SOURCE, `to`
    /// the recipient — an object or a player ([CR#120.3], the kind-poly
    /// patient). `combat` narrows combat vs noncombat damage ([CR#510.1]);
    /// `amount` bounds the dealt amount.
    Damage {
        #[serde(default = "Predicate::any")]
        source: Predicate,
        #[serde(default = "Predicate::any")]
        to: Predicate,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        combat: Option<bool>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        amount: Option<CountBound>,
    },
    /// A player gained life ([CR#119.3]). `who` narrows the gaining
    /// player — the event's PATIENT ([CR#119.9]: "a source CAUSES [a
    /// player] to gain life" — the player is acted upon, no agent role).
    LifeGained {
        #[serde(default = "Predicate::any")]
        who: Predicate,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        amount: Option<CountBound>,
    },
    /// A player lost life ([CR#119.3]). `who` narrows as `LifeGained`'s
    /// does — the patient, no agent role (the CR names no lose-life
    /// mirror of [CR#119.9,119.10]'s trigger/replacement rewrite, but the
    /// same "a source causes the player to lose life" shape applies).
    LifeLost {
        #[serde(default = "Predicate::any")]
        who: Predicate,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        amount: Option<CountBound>,
    },
    /// A player drew ([CR#121.1]). Per-fact granularity is one card
    /// ([CR#121.2] — cards are drawn one at a time, so a multi-draw is N
    /// facts, never one amount-N fact): the amount CHANNEL sums draws
    /// ("for each card drawn"), while a multi-card `amount` BOUND stays
    /// bridge-capped (`Drawn:amount`).
    Drawn {
        #[serde(default = "Predicate::any")]
        who: Predicate,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        amount: Option<CountBound>,
    },
    /// A NAMED keyword action ([CR#701]) — the ONE present-tense event
    /// (`GameEvent::Act`) that is both the guardable/replaceable moment and the
    /// "whenever you scry/surveil/…" trigger fact. The MASTER FORM over the
    /// verb family: `verb` names the keyword action (a [`VerbName`] — `Scry`,
    /// `Destroy`, …); `who` narrows its performer (the player-report verbs'
    /// actor — "whenever an opponent draws"); `on` narrows its patient (the
    /// object verbs' affected card — indestructible's `Destroy(Ref(This))`,
    /// [CR#702.12b]); `cause` narrows the cause triple (a discard narrowed to
    /// cycling-cost agency fires once per cycle). Omitted `who`/`on` default to
    /// match-anything; the COUNT is never carried — a keyword-action pattern
    /// matches any amount. A verb with more than one SUBJECT commits one
    /// NAME-fact per subject ([CR#701.14a]'s Fight, two combatants — a
    /// self-fight is still one subject, [CR#701.14c]), so `on` matches
    /// PER-SUBJECT — `Fight(pred)` fires once for whichever combatant `pred`
    /// names, either fighter, independently — never both slots at once (Fight
    /// has no performer, and no printed card narrows "the other fighter").
    ///
    /// Authored through the bare-verb pattern TWINS at the `EventFilter`
    /// position — `Destroy(Ref(This))`, `Discard(Ref(You), Any)`, `Scry(You)`,
    /// `Cant(Destroy(..))` — one hand-authored macro per verb expanding to this
    /// form, the `Dies`/`Destroyed` precedent.
    ///
    /// LANE-SPLIT matching ([CR#616.1,616.1f]): the trigger/history lanes read
    /// the finalized NAME-fact only (`verb`/`who`/`on`/`cause`) — a redirected
    /// madness discard still fires "whenever you discard" ([CR#701.9c]); the
    /// replacement (would) lane ADDITIONALLY requires the event's realized
    /// content facets to still match the verb's canonical zone shape, so a
    /// content already modified away from canonical stops matching
    /// automatically (stacked madness / Leyline-first — the auto-guard, no
    /// explicit semantic conjunction). Distinct from the RESULT-side
    /// `ZoneChange`/ `Drawn` filters "destroyed"/"drawn" triggers key on
    /// ([CR#700.4]).
    Act {
        verb: VerbName,
        #[serde(default = "Predicate::any")]
        who: Predicate,
        #[serde(default = "Predicate::any")]
        on: Predicate,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        cause: Option<Cause>,
    },
    /// Counters were placed on an object or player ([CR#122.1]). An omitted
    /// `kind` watches any counter kind; `cause` narrows the cause triple the
    /// event already carries ("a counter removed as a cost").
    CounterPlaced {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        kind: Option<CounterRef>,
        #[serde(default = "Predicate::any")]
        on: Predicate,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        amount: Option<CountBound>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        cause: Option<Cause>,
    },
    /// Counters were removed from an object or player ([CR#122.1]). `cause`
    /// narrows as `CounterPlaced`'s does, above.
    CounterRemoved {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        kind: Option<CounterRef>,
        #[serde(default = "Predicate::any")]
        on: Predicate,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        amount: Option<CountBound>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        cause: Option<Cause>,
    },
    /// A spell became cast ([CR#601.2i]) — the onset family ([CR#603.2]).
    /// `who` is the casting player, `what` the spell on the stack.
    Cast {
        #[serde(default = "Predicate::any")]
        who: Predicate,
        #[serde(default = "Predicate::any")]
        what: Predicate,
    },
    /// A spell or ability was copied onto the stack ([CR#707.10]) — the
    /// magecraft family's "or copy" half; `Cast` does NOT fire for copies
    /// ("a copy of a spell isn't cast").
    Copied {
        #[serde(default = "Predicate::any")]
        who: Predicate,
        #[serde(default = "Predicate::any")]
        what: Predicate,
    },
    /// A card was played — the land drop ([CR#701.18a]; a special action,
    /// [CR#116.2a]).
    Played {
        #[serde(default = "Predicate::any")]
        who: Predicate,
        #[serde(default = "Predicate::any")]
        what: Predicate,
    },
    /// An activated ability was activated ([CR#602.2a]). `what` matches
    /// the ability's SOURCE object.
    ActivatedAb {
        #[serde(default = "Predicate::any")]
        who: Predicate,
        #[serde(default = "Predicate::any")]
        what: Predicate,
    },
    /// A lowering-classified activated mana ability was activated. Kept
    /// distinct from `ActivatedAb` so [CR#605.1b] classification can prove the
    /// trigger is caused only by a mana ability.
    ManaAbilityActivated {
        #[serde(default = "Predicate::any")]
        what: Predicate,
        #[serde(default = "Predicate::any")]
        by: Predicate,
    },
    /// A mana ability finished a concrete production action.
    ManaProduced {
        #[serde(default = "Predicate::any")]
        what: Predicate,
        #[serde(default = "Predicate::any")]
        by: Predicate,
    },
    /// Mana was added to a player's pool, whether or not its source was a mana
    /// ability ([CR#605.1b]). `what` filters the producing source when known.
    ManaAdded {
        #[serde(default = "Predicate::any")]
        what: Predicate,
        #[serde(default = "Predicate::any")]
        by: Predicate,
    },
    /// A creature was declared as an attacker ([CR#508.1k] — it becomes an
    /// attacking creature). `against` matches the DEFENDING player
    /// ([CR#506.2,508.5]).
    AttackDeclared {
        #[serde(default = "Predicate::any")]
        by: Predicate,
        #[serde(default = "Predicate::any")]
        against: Predicate,
    },
    /// A block was declared — one fact, two views ([CR#509.1g..509.1h]):
    /// `by` is the BLOCKER ("whenever ~ blocks", [CR#509.3a]), `of` the
    /// blocked ATTACKER ("becomes blocked", [CR#509.3c]). Bushido
    /// ([CR#702.45a]) unions the two spellings.
    BlockDeclared {
        #[serde(default = "Predicate::any")]
        by: Predicate,
        #[serde(default = "Predicate::any")]
        of: Predicate,
    },
    /// An Aura/Equipment/Fortification became attached ([CR#701.3a];
    /// "becomes attached", [CR#603.2e]). `what` is the attachment, `to`
    /// the host.
    Attached {
        #[serde(default = "Predicate::any")]
        what: Predicate,
        #[serde(default = "Predicate::any")]
        to: Predicate,
    },
    /// An object's own status changed — transitions only ([CR#603.2e]).
    /// `cause` narrows by the transition's cause triple where the event
    /// carries one (`Tapped`/`Untapped`, "becomes tapped by an effect");
    /// `Transformed`/`Phased`/`TurnedFace` carry none, so a cause-narrowed
    /// pattern over those never matches.
    StateBecame {
        #[serde(default = "Predicate::any")]
        of: Predicate,
        becomes: StateChange,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        cause: Option<Cause>,
    },
    /// An object became the target of a spell/ability ([CR#601.2c]
    /// announce-time; ward is the family exemplar, [CR#702.21a]). TWO
    /// ARMS: `by` narrows the targeting STACK OBJECT (shroud), `source`
    /// narrows its SOURCE (hexproof-from) [CR#702.11d,702.16b].
    BecomesTarget {
        #[serde(default = "Predicate::any")]
        what: Predicate,
        #[serde(default = "Predicate::any")]
        by: Predicate,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        source: Option<Predicate>,
    },
    /// A step or phase began ([CR#603.2b], "at the beginning of …"). The
    /// `whose` coordinate ([CR#503.1] "your upkeep") narrows by whose turn
    /// it is, relative to the watching ability's controller.
    StepBegins { at: PhaseStep, whose: WhoseTurn },
    /// An object came under the control of a matching player ([CR#603.2e]
    /// transitions-only; a control change is never a zone change — the
    /// object keeps its identity, [CR#613.1b]).
    ControlChanged {
        #[serde(default = "Predicate::any")]
        of: Predicate,
        #[serde(default = "Predicate::any")]
        to: Predicate,
    },
    /// A named designation changed: "becomes the monarch" /
    /// "becomes goaded" — `of` matches the gaining carrier (player proxy or
    /// object). `to` narrows an enum designation's new value; game-scope
    /// day/night is `DesignationChanged(name: "DayNight", to: "Day")`
    /// ([CR#731.1]).
    DesignationChanged {
        name: Ident,
        #[serde(default = "Predicate::any")]
        of: Predicate,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        to: Option<Ident>,
    },
    /// A token was created ([CR#701.7a,111.2]). `what` matches the created
    /// token, `by` its creator.
    TokenCreated {
        #[serde(default = "Predicate::any")]
        what: Predicate,
        #[serde(default = "Predicate::any")]
        by: Predicate,
    },
    /// A library or face-down pile was shuffled ([CR#701.24a]) — an
    /// INFORMATION event, no participant beyond the shuffling player.
    /// `by` narrows the shuffler (Psychic Surgery's "whenever a player
    /// shuffles their library").
    Shuffled {
        #[serde(default = "Predicate::any")]
        by: Predicate,
    },
    /// Cards were revealed ([CR#701.20a]) — `what` narrows ∃-over the
    /// revealed set. The revealer is not a modeled participant: [CR#701.20]
    /// names no performer role (any effect can reveal), so there is no
    /// `by`/actor coordinate — the revealer stays derived from context, not
    /// stored on the fact.
    Revealed {
        #[serde(default = "Predicate::any")]
        what: Predicate,
    },
    /// An ability of `of` was used — a triggered ability fired ([CR#603.2])
    /// or an activated ability was activated ([CR#602.2a]). Matches the
    /// engine's `AbilityUsed` history fact, object-scoped ([CR#400.7]);
    /// counted via `EventCount` ([CR#608.2i]).
    Used { of: Reference },
    /// A coin was flipped ([CR#705.1]). `won` narrows by the flipper
    /// winning/losing the flip ([CR#705.2]).
    CoinFlipped {
        #[serde(default = "Predicate::any")]
        by: Predicate,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        won: Option<bool>,
    },
    /// A die was rolled ([CR#706.1]).
    DiceRolled {
        #[serde(default = "Predicate::any")]
        by: Predicate,
    },
    /// A land was tapped for mana ([CR#106.12]) — the ONE event whose Idris
    /// `EventCaps.producesMana` cap is `True` (it IS a mana ability
    /// resolving), gating
    /// [`ManaSpec::ProducedByEvent`](crate::ManaSpec::ProducedByEvent)
    /// (Dictate of Karametra/Vorinclex's "add one mana of any type that land
    /// produced", [CR#106.12a]). `what` narrows the tapped permanent
    /// (almost always left `Any` in practice — the event kind itself is
    /// already land-scoped); `by` narrows the tapper (its controller). Rust
    /// has no compile-time gate mirroring Idris's `producesMana` auto-proof
    /// (the soundness check is re-emit + `idris-check`, per this repo's
    /// established "sound data SHAPE, not soundness to Rust" policy) — an
    /// semantic-input error that reaches for `ProducedByEvent` outside a
    /// `TapForMana` body simply fails to typecheck on the Idris side.
    TapForMana {
        #[serde(default = "Predicate::any")]
        what: Predicate,
        #[serde(default = "Predicate::any")]
        by: Predicate,
    },
    /// The (Planechase) planar die was rolled ([CR#901.9]) — `face` narrows
    /// by the rolled face ([`PlanarFace`](crate::PlanarFace); `None` = any
    /// face). No amount of its own ([CR#901.9d] — the face is not numeric).
    /// `by` narrows the roller.
    RollPlanarDie {
        #[serde(default = "Predicate::any")]
        by: Predicate,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        face: Option<crate::PlanarFace>,
    },
    /// Every sub-pattern matches the same occurrence ([CR#603.2]) — a
    /// refinement conjunction. All conjuncts must agree on one master-form
    /// kind (a disagreement is unrepresentable in the Idris model).
    AllOf(Arc<[EventFilter]>),
    /// Any of several events ([CR#603.2], "whenever … or …"); still fires
    /// once per matching occurrence ([CR#603.2c]). Kind↔filter pairing is
    /// kept per disjunct; caps guarantee only the meet.
    OneOf(Arc<[EventFilter]>),
    /// The occurrence does NOT match the operand — a refinement, never an
    /// anchor: the operand must bottom out in master forms and `Not`
    /// itself never kind-anchors a live lane ([CR#603.2]; no
    /// freeze-everything `CantHappen`).
    Not(Arc<EventFilter>),
    /// One or more matching occurrences in one event, matched as the
    /// BATCH: the pattern matches the occurrence once, not per member
    /// ([CR#603.2c]). Trigger/replacement lanes only.
    OneOrMore(Arc<EventFilter>),
    /// The nth matching occurrence within a lookback ([CR#603.2]; a dead
    /// 0th occurrence never happens, [CR#603.2g]) — "the second spell you
    /// cast this turn". `n` is 1-based (≥ 1, gated by the Idris model).
    Nth {
        n: Uint,
        of: Arc<EventFilter>,
        within: Lookback,
    },
    /// A game-state refinement on the occurrence ("during your turn") —
    /// the condition is evaluated as the event occurs ([CR#603.4] states
    /// the ability-level twin; this is the pattern-level residue).
    When(Arc<EventFilter>, Arc<Condition>),
    /// A history-lane window refinement ([CR#608.2i]) — legal ONLY under
    /// `Happened`/`EventCount`/`EventSum` (history lanes); vacuous, and
    /// refused, in live lanes ([CR#603.2]).
    Within(Arc<EventFilter>, Lookback),
    /// The storm refinement ([CR#702.40a]): the occurrence happened strictly
    /// BEFORE the referenced object's OWN cast this window — keyed on cast
    /// ORDER ([CR#601.2i]), never resolution time. `Before(This)` delivers
    /// storm's "each OTHER spell cast before it this turn": the storm spell's
    /// own cast is not before itself (so "other" falls out), and a spell cast
    /// in RESPONSE — a later cast the turn's tally still holds, placed above
    /// the storm trigger ([CR#603.3]) — is excluded. A history-lane
    /// refinement, legal only under `EventCount`/`EventSum`; a live fact has
    /// no recorded cast position to compare and never matches.
    Before(Reference),
}
