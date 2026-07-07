use serde::Deserialize;
use serde::Serialize;

use crate::Condition;
use crate::CountBound;
use crate::CounterRef;
use crate::Expand;
use crate::Expansion;
use crate::Ident;
use crate::Lookback;
use crate::Predicate;
use crate::Reference;
use crate::SupportsMacros;
use crate::Uint;
use crate::Zone;

/// A phase KIND — the five phases of a turn without their step breakdown
/// ([CR#500.1]). The grain effects that add whole phases speak at
/// ([CR#500.8], `Action::ExtraPhase` — "an additional combat phase");
/// [`PhaseStep`] names a phase-AND-step position instead. Extra STEPS
/// ([CR#500.9]) accrete a step-grained twin when a card needs one.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Deserialize, Expand, Serialize)]
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
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Deserialize, Expand, Serialize)]
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
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Deserialize, Expand, Serialize)]
pub enum BeginningStep {
    /// [CR#502].
    Untap,
    /// [CR#503].
    Upkeep,
    /// [CR#504].
    Draw,
}

/// The steps of the combat phase ([CR#506,507,508,509,510,511]).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Deserialize, Expand, Serialize)]
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
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Deserialize, Expand, Serialize)]
pub enum EndingStep {
    /// [CR#513].
    End,
    /// [CR#514].
    Cleanup,
}

/// Whose turn a step-based trigger watches ([CR#503.1] "your upkeep", etc.).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Deserialize, Expand, Serialize)]
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
#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Expand, Serialize)]
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
}

/// The machinery that demanded an event — the cause triple's AGENCY
/// coordinate (mtg-rules events.md §3). Closed CR vocabulary.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Deserialize, Expand, Serialize)]
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

/// The named cause VERBS a pattern may narrow by — the closed grammar
/// vocabulary behind "sacrificed"/"destroyed"/"discarded" views. Each verb's
/// fact form is one CR-cited row of the emitted entailment table
/// (`crates/deckmaste_cards/tables/entailments.ron`): `Sacrifice` entails
/// `ZoneChange { from: Battlefield, to: Graveyard }` [CR#701.21a], `Mill`
/// entails `ZoneChange { from: Library, to: Graveyard }` [CR#701.17a] — so
/// `Dies` matches a sacrifice structurally, and a cause-narrowed pattern
/// admits exactly its verb's occurrences. Grows only through the closed-verb
/// admission test.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Deserialize, Expand, Serialize)]
pub enum CauseVerb {
    /// [CR#701.21a] — never a destruction (regeneration can't replace it).
    Sacrifice,
    /// [CR#701.8a]; one of "destroyed"'s exactly two causes ([CR#701.8b]).
    Destroy,
    /// [CR#701.9a].
    Discard,
    /// [CR#701.13a].
    Exile,
    /// [CR#701.17a].
    Mill,
    /// [CR#701.18a] — the land-drop cause (an effect putting a land onto
    /// the battlefield is NOT a play).
    Play,
    /// [CR#701.14a] — fight damage is noncombat damage ([CR#701.14d]).
    Fight,
    /// [CR#701.44a].
    Explore,
    /// [CR#701.19a] — the visible fact of an applied regeneration shield
    /// is the tap.
    Regenerate,
}

impl CauseVerb {
    /// The canonical fact-side spelling — the engine's cause triples store
    /// verbs as `Ident`s (`crate::event::Cause` constructors on the engine
    /// side); a pattern verb matches a fact verb by this string.
    #[must_use]
    pub fn as_str(self) -> &'static str {
        match self {
            CauseVerb::Sacrifice => "Sacrifice",
            CauseVerb::Destroy => "Destroy",
            CauseVerb::Discard => "Discard",
            CauseVerb::Exile => "Exile",
            CauseVerb::Mill => "Mill",
            CauseVerb::Play => "Play",
            CauseVerb::Fight => "Fight",
            CauseVerb::Explore => "Explore",
            CauseVerb::Regenerate => "Regenerate",
        }
    }
}

/// A trigger-side predicate over an event's cause triple (verb, agency,
/// agent). Every omitted coordinate matches anything. `verb` names the
/// entailed view ([`CauseVerb`] — "Destroy", "Sacrifice", …); `agent`
/// filters the causing object/controller (Karmic Justice's "a spell or
/// ability an opponent controls"). Agent-IDENTITY equality ("destroyed this
/// way") is a binding concern, not a pattern — it rides the event log.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Expand, Serialize)]
pub struct CausePattern {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub verb: Option<CauseVerb>,
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
#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Expand, Serialize)]
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
/// evaluator (`engine-one-evaluator`, `deckmaste_engine`'s `eval.rs`).
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
///
/// Both serde impls are generated by `#[derive(SupportsMacros)]`: `Expanded`
/// writes the invocation back, and the struct variants read flat in RON
/// through generated helper structs (carrying the forwarded `#[serde(...)]`
/// field defaults) + `unwrap_variant_newtypes`.
#[derive(Debug, Clone, PartialEq, Eq, Hash, SupportsMacros)]
pub enum EventFilter {
    /// An object changed zones ([CR#603.6]). `Dies` = `from: Battlefield,
    /// to: Graveyard` is a prelude macro over this. `cause` narrows by the
    /// cause triple ("destroyed" admits exactly two causes, [CR#701.8b];
    /// "sacrificed" is never destruction, [CR#701.21a]); omitted = any
    /// cause ("dies", [CR#700.4]). Each [`CauseVerb`]'s entailed fact form
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
    /// A player gained life ([CR#119.3]).
    LifeGained {
        #[serde(default = "Predicate::any")]
        who: Predicate,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        amount: Option<CountBound>,
    },
    /// A player lost life ([CR#119.3]).
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
    /// Counters were placed on an object or player ([CR#122.1]). An omitted
    /// `kind` watches any counter kind.
    CounterPlaced {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        kind: Option<CounterRef>,
        #[serde(default = "Predicate::any")]
        on: Predicate,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        amount: Option<CountBound>,
    },
    /// Counters were removed from an object or player ([CR#122.1]).
    CounterRemoved {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        kind: Option<CounterRef>,
        #[serde(default = "Predicate::any")]
        on: Predicate,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        amount: Option<CountBound>,
    },
    /// A spell became cast ([CR#601.2i]) — the onset family ([CR#603.2]).
    /// `who` is the casting player, `what` the spell on the stack.
    Cast {
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
    StateBecame {
        #[serde(default = "Predicate::any")]
        of: Predicate,
        becomes: StateChange,
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
    /// A named designation changed hands ([CR#109.3]): "becomes the
    /// monarch" / "becomes goaded" — `of` matches the gaining carrier
    /// (player proxy or object). The GAME-scope day/night designation
    /// ([CR#731.1]) rides the expletive forms
    /// [`BecameDay`](EventFilter::BecameDay)/
    /// [`BecameNight`](EventFilter::BecameNight) instead.
    DesignationChanged {
        name: Ident,
        #[serde(default = "Predicate::any")]
        of: Predicate,
    },
    /// A token was created ([CR#701.7a,111.2]). `what` matches the created
    /// token, `by` its creator.
    TokenCreated {
        #[serde(default = "Predicate::any")]
        what: Predicate,
        #[serde(default = "Predicate::any")]
        by: Predicate,
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
    /// "It becomes day" — the game gained the day designation
    /// ([CR#731.1,731.1a]; an expletive-"it" verb, no participants).
    BecameDay,
    /// "It becomes night" ([CR#731.1,731.1a]).
    BecameNight,
    /// Every sub-pattern matches the same occurrence ([CR#603.2]) — a
    /// refinement conjunction. All conjuncts must agree on one master-form
    /// kind (a disagreement is unrepresentable in the Idris model).
    AllOf(Vec<EventFilter>),
    /// Any of several events ([CR#603.2], "whenever … or …"); still fires
    /// once per matching occurrence ([CR#603.2c]). Kind↔filter pairing is
    /// kept per disjunct; caps guarantee only the meet.
    OneOf(Vec<EventFilter>),
    /// The occurrence does NOT match the operand — a refinement, never an
    /// anchor: the operand must bottom out in master forms and `Not`
    /// itself never kind-anchors a live lane ([CR#603.2]; no
    /// freeze-everything `CantHappen`).
    Not(Box<EventFilter>),
    /// One or more matching occurrences in one event, matched as the
    /// BATCH: the pattern matches the occurrence once, not per member
    /// ([CR#603.2c]). Trigger/replacement lanes only.
    OneOrMore(Box<EventFilter>),
    /// The nth matching occurrence within a lookback ([CR#603.2]; a dead
    /// 0th occurrence never happens, [CR#603.2g]) — "the second spell you
    /// cast this turn". `n` is 1-based (≥ 1, gated by the Idris model).
    Nth {
        n: Uint,
        of: Box<EventFilter>,
        within: Lookback,
    },
    /// A game-state refinement on the occurrence ("during your turn") —
    /// the condition is evaluated as the event occurs ([CR#603.4] states
    /// the ability-level twin; this is the pattern-level residue).
    When(Box<EventFilter>, Box<Condition>),
    /// A history-lane window refinement ([CR#608.2i]) — legal ONLY under
    /// `Happened`/`EventCount`/`EventSum` (history lanes); vacuous, and
    /// refused, in live lanes ([CR#603.2]).
    Within(Box<EventFilter>, Lookback),
    /// A remembered `EventFilter` macro invocation (`Dies`, `Enters`,
    /// `Sacrificed`, …). Serialized as the invocation, not the struct.
    #[macro_ron(expanded)]
    Expanded(Expansion<EventFilter>),
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::CharacteristicPredicate;
    use crate::Type;

    fn read(source: &str) -> EventFilter {
        crate::ron::options().from_str(source).unwrap()
    }

    /// Predicate fields default to match-anything when omitted — `Cast()`
    /// reads as any-caster/any-spell.
    #[test]
    fn filter_fields_default_to_any() {
        assert_eq!(
            read("Cast(who: Ref(You))"),
            EventFilter::Cast {
                who: Predicate::Ref(crate::Reference::You),
                what: Predicate::Any,
            },
        );
        assert_eq!(
            read("AttackDeclared(by: Ref(This))"),
            EventFilter::AttackDeclared {
                by: Predicate::Ref(crate::Reference::This),
                against: Predicate::Any,
            },
        );
    }

    /// The cause position is an enum (single variant today) so the name
    /// is structural: it always reads `Cause(verb: …)` — a bare
    /// `(verb: …)` tuple does not parse (user ruling), and boolean
    /// variants can accrete without respelling files. The verb is the
    /// closed [`CauseVerb`] vocabulary, spelled bare.
    #[test]
    fn zone_change_cause_named_and_never_bare() {
        assert_eq!(
            read(
                "ZoneChange(what: Type(Creature), from: Battlefield, to: Graveyard, cause: Cause(verb: Destroy))"
            ),
            EventFilter::ZoneChange {
                what: Predicate::Characteristic(CharacteristicPredicate::Type(Type::Creature)),
                from: Some(Zone::Battlefield),
                to: Some(Zone::Graveyard),
                cause: Some(Cause::Cause(CausePattern {
                    verb: Some(CauseVerb::Destroy),
                    agency: None,
                    agent: None,
                })),
            },
        );
        assert!(
            crate::ron::options()
                .from_str::<EventFilter>("ZoneChange(what: Type(Creature), cause: (verb: Destroy))")
                .is_err(),
            "a bare cause tuple must not parse — the variant name is mandatory"
        );
    }

    #[test]
    fn zone_change_options_default_none() {
        assert_eq!(
            read("ZoneChange(what: Type(Creature), from: Battlefield, to: Graveyard)"),
            EventFilter::ZoneChange {
                what: Predicate::Characteristic(CharacteristicPredicate::Type(Type::Creature)),
                from: Some(Zone::Battlefield),
                to: Some(Zone::Graveyard),
                cause: None,
            },
        );
        assert_eq!(
            read("ZoneChange(what: Type(Creature))"),
            EventFilter::ZoneChange {
                what: Predicate::Characteristic(CharacteristicPredicate::Type(Type::Creature)),
                from: None,
                to: None,
                cause: None,
            },
        );
    }

    /// `StepBegins` carries ONLY the step and whose-turn coordinates — a
    /// step event has no `what:` by construction.
    #[test]
    fn step_begins_reads() {
        assert_eq!(
            read("StepBegins(at: Beginning(Upkeep), whose: Your)"),
            EventFilter::StepBegins {
                at: PhaseStep::Beginning(BeginningStep::Upkeep),
                whose: WhoseTurn::Your,
            },
        );
    }

    /// `Used(of: This)` reads and serializes back to the same invocation —
    /// the self/object-scoped ability-use pattern counted via `EventCount`
    /// ([CR#608.2i]).
    #[test]
    fn used_round_trips() {
        use crate::Reference;

        let v = EventFilter::Used {
            of: Reference::This,
        };
        assert_eq!(read("Used(of: This)"), v);
        let w = crate::ron::options().to_string(&v).unwrap();
        assert_eq!(read(&w), v);
    }

    /// The amount refinement is the comparator-headed [`CountBound`]
    /// (`amount: AtLeast(3)`), with the bare-literal `Count` sugar inside.
    #[test]
    fn amount_bounds_read_comparator_headed() {
        use crate::Count;

        assert_eq!(
            read("Damage(source: Ref(This), amount: AtLeast(3))"),
            EventFilter::Damage {
                source: Predicate::Ref(crate::Reference::This),
                to: Predicate::Any,
                combat: None,
                amount: Some(CountBound::AtLeast(Count::Literal(3))),
            },
        );
    }

    /// The lane-gated algebra round-trips: `Nth` carries its 1-based index
    /// and lookback; `Within` carries a lookback; `OneOrMore` boxes its
    /// batch operand.
    #[test]
    fn algebra_round_trips() {
        for source in [
            "Nth(n: 2, of: Cast(who: Ref(You)), within: ThisTurn)",
            "Within(LifeGained(who: Ref(You)), ThisTurn)",
            "OneOrMore(ZoneChange(what: Type(Creature), from: Battlefield, to: Graveyard))",
            "Not(ZoneChange(what: Any, to: Battlefield, cause: Cause(verb: Play)))",
            "AllOf([ZoneChange(what: Any, to: Battlefield), ZoneChange(what: Type(Land))])",
            "When(StepBegins(at: Ending(End), whose: EachPlayers), YourTurn)",
            "BecameDay",
            "BecameNight",
            "CoinFlipped(by: Ref(You), won: true)",
            "DiceRolled(by: Ref(You))",
        ] {
            let parsed = read(source);
            let written = crate::ron::options().to_string(&parsed).unwrap();
            assert_eq!(read(&written), parsed, "round-trip failed for: {source}");
        }
    }
}
