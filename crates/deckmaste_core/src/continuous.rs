use serde::Deserialize;
use serde::Serialize;

use crate::Ability;
use crate::AsThough;
use crate::Color;
use crate::Condition;
use crate::CostComponent;
use crate::Count;
use crate::Deontic;
use crate::Event;
use crate::Expand;
use crate::Expansion;
use crate::Filter;
use crate::Ident;
use crate::Reference;
use crate::Supertype;
use crate::SupportsMacros;
use crate::TurnMarker;
use crate::Type;
use crate::ability::is_false;
use crate::replacement::Prevention;
use crate::replacement::Replacement;

/// How long a one-shot-created continuous effect lasts ([CR#611.2]). Static
/// abilities don't carry this — their duration is implicit ("while it
/// functions", [CR#611.3]).
#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize, Expand)]
pub enum Duration {
    /// Ends at a fixed turn-structure marker ([CR#611.2a]): end of turn
    /// sweeps in cleanup ([CR#514.2]), end of combat at the combat phase's
    /// end ([CR#500.5a,511.2]).
    FixedUntil(TurnMarker),
    /// Until an event happens (the engine pairs the undo one-shot, [CR#610.3]).
    UntilEvent(Event),
    /// "For as long as" — a tracked predicate ([CR#611.2b]). The
    /// never-started / already-ended edge rules ride a `started` latch on
    /// the ENGINE's effect-instance record, not the card grammar; once
    /// stopped (including losing sight of a phased-out object,
    /// [CR#702.26f]) it never resumes.
    ForAsLongAs(Condition),
    /// For the rest of the game — the no-stated-duration default ([CR#611.2a]).
    EndOfGame,
}

/// The set of objects a `Modify` applies to ([CR#611.2c] vs [CR#611.3] —
/// lock-in is provenance the engine applies, not stored here).
///
/// `Of` wraps a single reference (the spec's dead `That` renamed); `These`
/// a fixed list; `Matching` a filter-shaped, possibly-floating set.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize, Expand)]
pub enum Scope {
    /// One referenced object.
    Of(Reference),
    /// A fixed list of referenced objects.
    These(Vec<Reference>),
    /// Every object matching a filter (anthem-shaped).
    Matching(Filter),
}

/// A flat primitive characteristic-change op ([CR#613]). Layers are DERIVED
/// from the op, never written: `Add*` stats → 7c, `Set*` stats → 7b (7a when
/// CDA-flagged), `Switch` → 7d, types → 4, colors → 5, abilities → 6,
/// controller → 2, text → 3 ([CR#613.1]). One effect's `changes` is a list
/// because it can span layers applied to the same set ([CR#613.6]).
#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize, Expand)]
pub enum Modification {
    SetPower(Count),
    AddPower(Count),
    SetToughness(Count),
    AddToughness(Count),
    SubtractPower(Count),
    SubtractToughness(Count),
    /// Switch power and toughness ([CR#613.4d]).
    SwitchPowerToughness,
    SetColors(Vec<Color>),
    AddColors(Vec<Color>),
    SetCardTypes(Vec<Type>),
    AddCardTypes(Vec<Type>),
    /// Subtypes by name (the class is derivable from the values — each card
    /// type has its own closed subtype set, [CR#205.3b]).
    SetSubtypes(Vec<Ident>),
    AddSubtypes(Vec<Ident>),
    SetSupertypes(Vec<Supertype>),
    AddSupertypes(Vec<Supertype>),
    /// Gain an ability ([CR#613.1f]). Boxed: `Ability` is the enum's largest
    /// variant by far, so indirection keeps `Modification` small.
    GainAbility(Box<Ability>),
    /// Lose a named keyword ability ([CR#613.1f]).
    LoseAbility(Ident),
    /// Lose all abilities ([CR#613.1f]).
    LoseAllAbilities,
    /// Can't have or gain the named ability ([CR#613.1f]).
    CantHaveAbility(Ident),
    /// Change controller ([CR#613.1b]).
    SetController(Reference),
    /// Change text ([CR#613.1c]).
    SetText(String),
    /// "Is every creature type" ([CR#702.73a] changeling, [CR#205.3m] the
    /// open creature-type set) — an open-set subtype FILL, not a list op;
    /// layer 4, normally CDA-flagged.
    AllCreatureTypes,
    /// Set base loyalty ([CR#306.5b..306.5c] — the printed-loyalty baseline
    /// the counters start from; no 613 layer covers loyalty).
    SetBaseLoyalty(Count),
    /// Set base defense (battle).
    SetBaseDefense(Count),
    /// The [CR#305.7] bundle: replace land types ∧ lose printed abilities ∧
    /// gain the basic-land mana ability (Blood Moon). One intrinsic, not
    /// reachable from the plain `Set*` ops.
    BecomeBasicLandType(Vec<Ident>),
}

/// A step in the total-cost pipeline ([CR#601.2f]): base → +additional and
/// increases → −reductions (any order) → floor → lock. `Additional` is
/// pipeline-positional ([CR#118.8] — any number may stack, [CR#118.8a]);
/// it never changes the mana cost itself ([CR#118.8d]). Alternative costs
/// are NOT here — they swap the base and ride `May(Cast(cost: …))` rows
/// ([CR#118.9]).
#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize, Expand)]
pub enum CostChange {
    Increase(Vec<CostComponent>),
    Reduce(Vec<CostComponent>),
    /// "As an additional cost …" ([CR#118.8]); `optional` is the kicker
    /// shape ("you may pay an additional …", [CR#118.8b]), announced at
    /// [CR#601.2b].
    Additional {
        components: Vec<CostComponent>,
        #[serde(default, skip_serializing_if = "is_false")]
        optional: bool,
    },
    /// A COUNT-SCALED change: the inner change applies `times` times at
    /// total-cost time ([CR#601.2f]). Covers both polarities — "costs {1}
    /// less for each artifact you control" ([CR#702.41a] affinity) and
    /// "costs {1} more for each …" taxers — and `times` is a [`Count`],
    /// so every counting form (`CountOf`, X, queries) composes. Boxed to
    /// break the self-reference.
    Scaled {
        change: Box<CostChange>,
        times: Count,
    },
}

/// The shared currency between an "anthem" static ability and a "+3/+3 until
/// end of turn" one-shot ([CR#611]). The difference is who wraps it: a static
/// ability (`StaticAbility`) or a one-shot `Effect::Continuously`.
///
/// Both serde impls are generated by `#[derive(SupportsMacros)]` for macro
/// interception (it bears `Expanded`): unknown names at `StaticEffect`
/// positions fall through to the macro layer, and the struct variants read
/// flat in RON through generated helper structs + `unwrap_variant_newtypes`.
#[derive(Debug, Clone, PartialEq, Eq, Hash, SupportsMacros)]
pub enum StaticEffect {
    /// Change a scope's characteristics (anthems, pumps).
    Modify {
        of: Scope,
        changes: Vec<Modification>,
    },
    /// A deontic clause ([CR#101.2,601.3]): May/Cant/Must/Gate read bare
    /// in RON (`effects: [Cant(…)]`) via the flatten dispatch.
    #[macro_ron(flatten)]
    Deontic(Deontic),
    /// A cost modifier ([CR#118.7]).
    CostModifier { of: Filter, change: CostChange },
    /// A replacement effect ([CR#614]).
    Replacement(Replacement),
    /// A prevention effect ([CR#615]).
    Prevention(Prevention),
    /// A scoped counterfactual premise ([CR#609.4]) — see [`AsThough`].
    AsThough(AsThough),
    /// A state-based action expressed as data ([CR#704]): whenever `when`
    /// holds (evaluated with `This` = the carrying object), perform `then` as
    /// part of the SBA sweep. The Aura must-be-attached rule ([CR#704.5m]) is
    /// `Sba { when: Not(LegallyAttached(Ref(This))), then: Move(Ref(This),
    /// Graveyard) }`; the universal SBA-as-data primitive generalizes (a Saga's
    /// [CR#714.4] sacrifice is `Sba(lore≥final, Sacrifice(Ref(This)))`). The
    /// SBA sweep reads these statics generically — it never branches on the
    /// Aura/Equipment/Fortification subtype. `then` is boxed (an `Effect`
    /// dominates `StaticEffect`'s size; `Box` only for the size cycle, per the
    /// "Box only for cycles" rule).
    Sba {
        when: Condition,
        then: Box<crate::Effect>,
    },
    /// An outcome gate: "[who] can't lose the game" / "can't win the game".
    /// NOT a deontic row — outcome-"can't" modifies the §104/§704 outcome
    /// machinery, not action legality (mtg-rules deontics §6 evicts the
    /// family). Semantics (skill U5, settled): precedence, not consumption
    /// ([CR#101.2]) — the gate suppresses each applicable outcome at each
    /// SBA check ([CR#704.3]) while it lasts, and survival past the
    /// effect's end is decided by each SBA's own predicate: the standing
    /// state predicates ([CR#704.5a] life, [CR#704.5c] poison) fire at the
    /// first check after the gate ends; the windowed empty-draw predicate
    /// ([CR#704.5b]) lapses with its window. Concession pierces every gate
    /// ([CR#101.1,104.3a]); the last-player-standing win pierces `CantWin`
    /// ([CR#104.2a]); simultaneous win∧lose = lose ([CR#104.3f]).
    OutcomeGate { who: Filter, gate: OutcomeGateKind },
    /// A remembered `StaticEffect` macro invocation. Serialized as the
    /// invocation, not the struct.
    #[macro_ron(expanded)]
    Expanded(Expansion<StaticEffect>),
}

/// Which outcome a [`StaticEffect::OutcomeGate`] suppresses.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Deserialize, Serialize, Expand)]
pub enum OutcomeGateKind {
    /// Suppresses the loss SBAs ([CR#704.5a..704.5c]) and "loses the
    /// game" effect outcomes ([CR#104.3e]) for matching players.
    CantLose,
    /// Suppresses "wins the game" effect outcomes ([CR#104.2b]); the
    /// all-opponents-left win ([CR#104.2a]) bypasses it.
    CantWin,
}

#[cfg(test)]
mod tests {
    use super::*;

    fn read(source: &str) -> StaticEffect { crate::ron::options().from_str(source).unwrap() }

    /// The anthem shape reads flat and round-trips.
    #[test]
    fn modify_reads_flat() {
        let parsed = read(
            "Modify(of: Matching(Type(Creature)), changes: [AddPower(Literal(1)), AddToughness(Literal(1))])",
        );
        assert_eq!(
            parsed,
            StaticEffect::Modify {
                of: Scope::Matching(Filter::Characteristic(crate::CharacteristicFilter::Type(
                    Type::Creature
                ))),
                changes: vec![
                    Modification::AddPower(Count::Literal(1)),
                    Modification::AddToughness(Count::Literal(1)),
                ],
            },
        );
        let written = crate::ron::options().to_string(&parsed).unwrap();
        assert_eq!(read(&written), parsed);
    }

    /// The negative-P/T shape (layer 7c) reads flat and round-trips.
    #[test]
    fn subtract_modify_round_trips() {
        let parsed = read(
            "Modify(of: Matching(Type(Creature)), changes: [SubtractPower(Literal(1)), SubtractToughness(Literal(1))])",
        );
        assert_eq!(
            parsed,
            StaticEffect::Modify {
                of: Scope::Matching(Filter::Characteristic(crate::CharacteristicFilter::Type(
                    Type::Creature
                ))),
                changes: vec![
                    Modification::SubtractPower(Count::Literal(1)),
                    Modification::SubtractToughness(Count::Literal(1)),
                ],
            },
        );
        let written = crate::ron::options().to_string(&parsed).unwrap();
        assert_eq!(read(&written), parsed);
    }

    /// The `Sba` state-based-action primitive round-trips: the Aura
    /// must-be-attached shape `Sba(when: Not(LegallyAttached(Ref(This))), then:
    /// Move(Ref(This), Graveyard))` ([CR#704.5m]).
    #[test]
    fn sba_roundtrip() {
        use crate::Action;
        use crate::Condition;
        use crate::Effect;
        use crate::Selection;
        use crate::Zone;

        let sba = StaticEffect::Sba {
            when: Condition::Not(Box::new(Condition::LegallyAttached(Reference::This))),
            then: Box::new(Effect::Act(Action::Move(
                Selection::Ref(Reference::This),
                Zone::Graveyard,
            ))),
        };
        let written = crate::ron::options().to_string(&sba).unwrap();
        assert_eq!(read(&written), sba, "Sba round-trips: {written}");
    }

    /// A deontic clause reads flat and serializes flat — the compartment
    /// tag never appears in RON.
    #[test]
    fn deontic_reads_flat() {
        let parsed = read("Cant(Attack(by: Ref(This)))");
        assert_eq!(
            parsed,
            StaticEffect::Deontic(Deontic::Cant(crate::DeonticAction::Attack {
                by: Filter::Ref(crate::Reference::This),
                on: Filter::Any,
            })),
        );
        let written = crate::ron::options().to_string(&parsed).unwrap();
        assert!(
            !written.contains("Deontic"),
            "compartment tag leaked: {written}"
        );
        assert_eq!(read(&written), parsed);
    }
}
