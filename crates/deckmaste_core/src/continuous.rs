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
use crate::RelationFilter;
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
#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Expand, Serialize)]
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
#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Expand, Serialize)]
pub enum Scope {
    /// One referenced object.
    Of(Reference),
    /// A fixed list of referenced objects.
    These(Vec<Reference>),
    /// Every object matching a filter (anthem-shaped).
    Matching(Filter),
}

/// A shared op for the NUMERIC characteristic axes — power, toughness, base
/// defense, base loyalty. `Set` overwrites the base value (layer 7b, or 7a when
/// CDA-flagged), `Up`/`Down` are the ±N modifications (layer 7c)
/// ([CR#613.4a..613.4c]). The op↔axis pairing is the soundness gate: a numeric
/// axis variant (`Modification::Power`, …) takes a `NumericOp`, recovering
/// Idris's `Numeric` type-class gate (`idris/src/Core.idr`) structurally.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Expand, Serialize)]
pub enum NumericOp {
    /// Overwrite the base value — layer 7b, or 7a when CDA-flagged
    /// ([CR#613.4a,613.4b]).
    Set(Count),
    /// "+N" ([CR#613.4c], layer 7c).
    Up(Count),
    /// "−N" ([CR#613.4c], layer 7c).
    Down(Count),
}

/// A shared op for the SET-shaped characteristic axes — colors, card types,
/// subtypes, supertypes. `Set` overwrites the whole list; `Add`/`Remove`
/// affect a SINGLE element ([CR#613.1d,613.1e]). The op↔axis pairing is the
/// soundness gate: a collection axis variant (`Modification::Colors`, …) takes
/// a `CollectionOp`, which has no `Up`/`Down`, recovering Idris's `Collection`
/// type-class gate (`idris/src/Core.idr`) structurally.
///
/// Generic over the element type, so it can't `#[derive(Expand)]` (that derive
/// rejects generics); the `Expand` impl is hand-written just below. serde's
/// derive handles the generic fine.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub enum CollectionOp<T> {
    /// Overwrite the whole list (layer 4 types / layer 5 colors).
    Set(Vec<T>),
    /// Add one element.
    Add(T),
    /// Remove one element.
    Remove(T),
}

impl<T: Expand> Expand for CollectionOp<T> {
    fn expand_all(self) -> Self {
        match self {
            CollectionOp::Set(v) => CollectionOp::Set(v.expand_all()),
            CollectionOp::Add(t) => CollectionOp::Add(t.expand_all()),
            CollectionOp::Remove(t) => CollectionOp::Remove(t.expand_all()),
        }
    }
}

/// A flat primitive characteristic-change op ([CR#613]). Layers are DERIVED
/// from the op, never written: the numeric axes (`Power`/`Toughness`/…) carry a
/// [`NumericOp`] whose `Up`/`Down` → 7c and `Set` → 7b (7a when CDA-flagged);
/// the set-shaped axes (`Colors`/`CardTypes`/`Subtypes`/`Supertypes`) carry a
/// [`CollectionOp`] → layer 4 (types) / 5 (colors); `SwitchPowerToughness` →
/// 7d, abilities → 6, controller → 2, text → 3 ([CR#613.1]). One effect's
/// `changes` is a list because it can span layers applied to the same set
/// ([CR#613.6]).
///
/// The per-axis ops are factored into the two shared op enums ([`NumericOp`],
/// [`CollectionOp`]) but the AXIS stays named at the variant level — a
/// deliberately-partial unification of Idris's fully-unified `Alter
/// (Characteristic) (ModificationOp)` (`idris/src/Core.idr`): the variant↔op
/// pairing recovers the op↔axis soundness gate structurally (a `Colors` takes a
/// `CollectionOp`, which has no `Up`, so "raise a color" is unrepresentable),
/// while keeping RON readable (`Power(Up(1))`, `Colors(Add(Blue))`).
///
/// `SupportsMacros` (not plain `Expand`) so a change-bundling macro can stand
/// in a `changes: [...]` slot — the keystone being `AddPowerToughness(p, t)`,
/// which expands to `Several([Power(Up(p)), Toughness(Up(t))])`. `Several` is
/// the `Modification` analog of `Filter::AllOf`: a macro expands to ONE value,
/// so a macro that must contribute several ops bundles them into a `Several`.
/// Unlike `Filter::AllOf` (a conjunction the engine evaluates), `Several` is
/// semantically inert — `changes` is already a flat, layer-spanning list
/// ([CR#613.6]) — so it is flattened away once, at the engine boundary
/// ([`Modification::flatten`]), and the engine layer loops never see it.
#[derive(Debug, Clone, PartialEq, Eq, Hash, SupportsMacros)]
pub enum Modification {
    /// Power ([CR#613.4]): `Set` base (7a/7b), `Up`/`Down` (7c).
    Power(NumericOp),
    /// Toughness ([CR#613.4]): `Set` base (7a/7b), `Up`/`Down` (7c).
    Toughness(NumericOp),
    /// Switch power and toughness ([CR#613.4d]).
    SwitchPowerToughness,
    /// Colors ([CR#613.1e], layer 5).
    Colors(CollectionOp<Color>),
    /// Card types ([CR#613.1d], layer 4).
    CardTypes(CollectionOp<Type>),
    /// Subtypes by name (the class is derivable from the values — each card
    /// type has its own closed subtype set, [CR#205.3b]); layer 4.
    Subtypes(CollectionOp<Ident>),
    /// Supertypes ([CR#613.1d], layer 4).
    Supertypes(CollectionOp<Supertype>),
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
    /// Base loyalty ([CR#306.5b..306.5c] — the printed-loyalty baseline the
    /// counters start from; no 613 layer covers loyalty). Only `Set` is
    /// meaningful today; the `NumericOp` shares the numeric op vocabulary.
    BaseLoyalty(NumericOp),
    /// Base defense (battle). Only `Set` is meaningful today; the `NumericOp`
    /// shares the numeric op vocabulary.
    BaseDefense(NumericOp),
    /// The [CR#305.7] bundle: replace land types ∧ lose printed abilities ∧
    /// gain the basic-land mana ability (Blood Moon). One intrinsic, not
    /// reachable from the plain `Set*` ops.
    BecomeBasicLandType(Vec<Ident>),
    /// A bundle of ops contributed by one macro invocation — the analog of
    /// `Filter::AllOf`. A macro expands to a single value, so a change-bundling
    /// macro (`AddPowerToughness(p, t)`) produces `Several([Power(Up(p)),
    /// Toughness(Up(t))])`. Semantically inert: `changes` is already a flat,
    /// layer-spanning list ([CR#613.6]), so [`Modification::flatten`] splices a
    /// `Several` into its parent list at the engine boundary and the engine
    /// never sees this variant.
    Several(Vec<Modification>),
    /// A remembered `Modification` macro invocation. Serialized as the
    /// invocation, not the struct; `expand_all` strips it to the bundled value.
    #[macro_ron(expanded)]
    Expanded(Expansion<Modification>),
}

impl Modification {
    /// Splice every `Several` (recursively) into the parent list, the one
    /// flatten-away pass for change-bundling macros. Run AFTER `expand_all`
    /// (which turns `Expanded(AddPowerToughness(p, t))` into
    /// `Several([AddPower, AddToughness])`) and BEFORE the engine consumes
    /// `changes`: `changes` is semantically a flat, layer-spanning list
    /// ([CR#613.6]), so `Several` is a pure expansion artifact normalized
    /// away exactly once here. The engine's `layer_of`/`apply` then never
    /// see `Several`/`Expanded`.
    ///
    /// Element-wise `expand_all` first (a stored `changes` list may still hold
    /// `Expanded` invocations), then splice: a plain element passes through, a
    /// `Several` recurses and splices its (already-flattened) members in place.
    #[must_use]
    pub fn flatten(changes: Vec<Modification>) -> Vec<Modification> {
        use crate::Expand;
        let mut out = Vec::with_capacity(changes.len());
        for m in changes {
            match m.expand_all() {
                Modification::Several(inner) => out.extend(Modification::flatten(inner)),
                other => out.push(other),
            }
        }
        out
    }
}

/// A step in the total-cost pipeline ([CR#601.2f]): base → +additional and
/// increases → −reductions (any order) → floor → lock. `Additional` is
/// pipeline-positional ([CR#118.8] — any number may stack, [CR#118.8a]);
/// it never changes the mana cost itself ([CR#118.8d]). Alternative costs
/// are NOT here — they swap the base and ride `May(Cast(cost: …))` rows
/// ([CR#118.9]).
#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Expand, Serialize)]
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

/// The default `affected` for a [`StaticEffect::TriggerMultiplier`]: "you
/// control" — the source permanent's controller ([CR#603.2c]). The common case
/// (Panharmonicon / Yarok), so it is the serde default and is omitted from RON.
fn affected_you_control() -> Filter {
    Filter::Relation(RelationFilter::ControlledBy(Box::new(Filter::Ref(
        Reference::You,
    ))))
}

/// Whether an `affected` filter equals the "you control" default — skips it on
/// write so the common case stays flat.
fn is_affected_you_control(f: &Filter) -> bool {
    *f == affected_you_control()
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
    /// A trigger multiplier ([CR#603.2d] — "triggers additional times"):
    /// Panharmonicon, Yarok, and the trigger half of Doubling Season. A
    /// triggered ability whose trigger matches `cause`, carried by a permanent
    /// matching `affected` (default "you control"), fires `extra` ADDITIONAL
    /// times. It is NOT a copy — each firing chooses its own modes and targets
    /// — and multipliers ADD rather than compound (two Panharmonicons → 3×,
    /// not 4×; [CR#603.2d] "doesn't invoke itself repeatedly").
    /// Panharmonicon = `TriggerMultiplier(cause: ZoneMove(what:
    /// OneOf([Type(Artifact), Type(Creature)]), to: Battlefield), extra:
    /// 1)`.
    TriggerMultiplier {
        cause: Event,
        extra: Count,
        /// The affected ability's source permanent; defaults to "you control"
        /// (the source's controller), overridden for opponent/any-doublers.
        #[serde(
            default = "affected_you_control",
            skip_serializing_if = "is_affected_you_control"
        )]
        affected: Filter,
    },
    /// A continuous modification to a player's numeric attribute ([CR#611]):
    /// extra land plays (Exploration = `ModifyPlayer(Ref(You),
    /// Raise(LandPlaysPerTurn, 1))`, [CR#305.2]) or no maximum hand size
    /// (Reliquary Tower = `ModifyPlayer(Ref(You), NoMax(HandSizeLimit))`,
    /// [CR#402.2]). The object-modifying `Modify` touches objects only; this is
    /// its player-side twin. The `Reference` is the affected player ("you" by
    /// default — the source's controller).
    ModifyPlayer(Reference, PlayerMod),
    /// A replacement effect ([CR#614]). Boxed: `Replacement` is by far the
    /// largest payload here, so boxing keeps `StaticEffect` small
    /// (`clippy::large_enum_variant`).
    Replacement(Box<Replacement>),
    /// A prevention effect ([CR#615]). Boxed for the same size reason as
    /// `Replacement`.
    Prevention(Box<Prevention>),
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
        when: Box<Condition>,
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
    /// An event-side "can't happen" ([CR#614.17,702.12b]): the matching event
    /// can't occur. Distinct from `Deontic::Cant`, which is over player ACTIONS
    /// ([CR#101.2] action legality); destruction is an EVENT. Indestructible is
    /// `CantHappen(Destroyed(Ref(This)))`. Per [CR#614.17c] a can't-happen
    /// event can be touched only by a self-replacement, so the cant pass
    /// pre-empts the replacement registry entirely.
    CantHappen(Event),
    /// A remembered `StaticEffect` macro invocation. Serialized as the
    /// invocation, not the struct.
    #[macro_ron(expanded)]
    Expanded(Expansion<StaticEffect>),
}

/// Which outcome a [`StaticEffect::OutcomeGate`] suppresses.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Deserialize, Expand, Serialize)]
pub enum OutcomeGateKind {
    /// Suppresses the loss SBAs ([CR#704.5a..704.5c]) and "loses the
    /// game" effect outcomes ([CR#104.3e]) for matching players.
    CantLose,
    /// Suppresses "wins the game" effect outcomes ([CR#104.2b]); the
    /// all-opponents-left win ([CR#104.2a]) bypasses it.
    CantWin,
}

/// A player's numeric attribute a [`StaticEffect::ModifyPlayer`] adjusts — the
/// player-side twin of an object [`Modification`] axis. `HandSizeLimit`
/// (normally seven, [CR#402.2]) and `LandPlaysPerTurn` (normally one,
/// [CR#305.2]) are the caps continuous statics modify (Reliquary Tower /
/// Exploration); `Life` ([CR#119.1]) and `HandSize` ([CR#402.2]) round out the
/// readable player attributes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Deserialize, Expand, Serialize)]
pub enum PlayerAttr {
    Life,
    HandSize,
    HandSizeLimit,
    LandPlaysPerTurn,
}

/// A continuous modification to a player attribute, carried by
/// [`StaticEffect::ModifyPlayer`] — the player-side twin of [`NumericOp`]
/// ([CR#611]; players have no [CR#613] layers, so these apply directly).
/// `SetTo`/`Raise`/`Lower` adjust a count-valued attribute (Exploration =
/// `Raise(LandPlaysPerTurn, 1)`); `NoMax` removes a cap (Reliquary Tower =
/// `NoMax(HandSizeLimit)`, "no maximum hand size") — kept a dedicated op, not a
/// `Maybe Count` value, since a player attribute reads as a count.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Expand, Serialize)]
pub enum PlayerMod {
    /// Overwrite the attribute with a fixed value.
    SetTo(PlayerAttr, Count),
    /// "+N" the attribute.
    Raise(PlayerAttr, Count),
    /// "−N" the attribute.
    Lower(PlayerAttr, Count),
    /// Remove the attribute's maximum ("no maximum hand size").
    NoMax(PlayerAttr),
}

#[cfg(test)]
mod tests {
    use super::*;

    fn read(source: &str) -> StaticEffect {
        crate::ron::options().from_str(source).unwrap()
    }

    /// The anthem shape reads flat and round-trips.
    #[test]
    fn modify_reads_flat() {
        let parsed = read(
            "Modify(of: Matching(Type(Creature)), changes: [Power(Up(Literal(1))), Toughness(Up(Literal(1)))])",
        );
        assert_eq!(
            parsed,
            StaticEffect::Modify {
                of: Scope::Matching(Filter::Characteristic(crate::CharacteristicFilter::Type(
                    Type::Creature
                ))),
                changes: vec![
                    Modification::Power(NumericOp::Up(Count::Literal(1))),
                    Modification::Toughness(NumericOp::Up(Count::Literal(1))),
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
            "Modify(of: Matching(Type(Creature)), changes: [Power(Down(Literal(1))), Toughness(Down(Literal(1)))])",
        );
        assert_eq!(
            parsed,
            StaticEffect::Modify {
                of: Scope::Matching(Filter::Characteristic(crate::CharacteristicFilter::Type(
                    Type::Creature
                ))),
                changes: vec![
                    Modification::Power(NumericOp::Down(Count::Literal(1))),
                    Modification::Toughness(NumericOp::Down(Count::Literal(1))),
                ],
            },
        );
        let written = crate::ron::options().to_string(&parsed).unwrap();
        assert_eq!(read(&written), parsed);
    }

    /// A collection-axis op reads flat and round-trips: `Colors(Set([...]))`
    /// (layer 5) and the single-element `Add`/`Remove` forms ([CR#613.1d]).
    #[test]
    fn collection_op_round_trips() {
        let parsed = read(
            "Modify(of: Matching(Type(Creature)), changes: [Colors(Set([Black])), CardTypes(Add(Artifact)), Subtypes(Remove(\"Goblin\"))])",
        );
        assert_eq!(
            parsed,
            StaticEffect::Modify {
                of: Scope::Matching(Filter::Characteristic(crate::CharacteristicFilter::Type(
                    Type::Creature
                ))),
                changes: vec![
                    Modification::Colors(CollectionOp::Set(vec![Color::Black])),
                    Modification::CardTypes(CollectionOp::Add(Type::Artifact)),
                    Modification::Subtypes(CollectionOp::Remove("Goblin".into())),
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
            when: Box::new(Condition::Not(Box::new(Condition::LegallyAttached(
                Reference::This,
            )))),
            then: Box::new(Effect::Act(Action::move_to(
                Selection::Ref(Reference::This),
                Zone::Graveyard,
            ))),
        };
        let written = crate::ron::options().to_string(&sba).unwrap();
        assert_eq!(read(&written), sba, "Sba round-trips: {written}");
    }

    /// A `CantHappen` variant round-trips from RON.
    #[test]
    fn cant_happen_reads_flat() {
        let parsed = read(
            "CantHappen(ZoneMove(what: Ref(This), from: Battlefield, to: Graveyard, cause: Cause(verb: \"Destroy\")))",
        );
        assert!(matches!(parsed, StaticEffect::CantHappen(_)));
    }

    /// `Modification::flatten` splices a `Several` bundle into its parent list
    /// (recursively) and leaves plain ops untouched — the one flatten-away pass
    /// for change-bundling macros. A `Several([Power(Up), Toughness(Up)])`
    /// (what `AddPowerToughness(3, 3)` expands to) followed by a
    /// `GainAbility` becomes the flat three-op list the engine consumes.
    #[test]
    fn flatten_splices_several() {
        let changes = vec![
            Modification::Several(vec![
                Modification::Power(NumericOp::Up(Count::Literal(3))),
                Modification::Toughness(NumericOp::Up(Count::Literal(3))),
            ]),
            Modification::LoseAllAbilities,
            // A nested Several splices recursively.
            Modification::Several(vec![Modification::Several(vec![Modification::Colors(
                CollectionOp::Set(vec![Color::White]),
            )])]),
        ];
        assert_eq!(
            Modification::flatten(changes),
            vec![
                Modification::Power(NumericOp::Up(Count::Literal(3))),
                Modification::Toughness(NumericOp::Up(Count::Literal(3))),
                Modification::LoseAllAbilities,
                Modification::Colors(CollectionOp::Set(vec![Color::White])),
            ],
        );
    }

    /// `flatten` runs `expand_all` element-wise first, so a stored `changes`
    /// list still holding an `Expanded(Several([...]))` invocation (the
    /// `AddPowerToughness` shape) flattens to its bundled ops.
    #[test]
    fn flatten_strips_expanded_invocations() {
        use crate::Expansion;
        use crate::ExpansionArgs;
        let expanded = Modification::Expanded(Expansion {
            name: "AddPowerToughness".into(),
            args: ExpansionArgs::Positional(vec!["2".into(), "0".into()]),
            template: Some("gets +${0}/+${1}".into()),
            value: Box::new(Modification::Several(vec![
                Modification::Power(NumericOp::Up(Count::Literal(2))),
                Modification::Toughness(NumericOp::Up(Count::Literal(0))),
            ])),
        });
        assert_eq!(
            Modification::flatten(vec![expanded, Modification::SwitchPowerToughness]),
            vec![
                Modification::Power(NumericOp::Up(Count::Literal(2))),
                Modification::Toughness(NumericOp::Up(Count::Literal(0))),
                Modification::SwitchPowerToughness,
            ],
        );
    }

    /// `ModifyPlayer` reads flat and round-trips: the Exploration land-plays
    /// raise ([CR#305.2]) and the Reliquary Tower no-maximum-hand-size cap
    /// removal ([CR#402.2]).
    #[test]
    fn modify_player_round_trips() {
        let parsed = read("ModifyPlayer(You, Raise(LandPlaysPerTurn, 1))");
        assert_eq!(
            parsed,
            StaticEffect::ModifyPlayer(
                Reference::You,
                PlayerMod::Raise(PlayerAttr::LandPlaysPerTurn, Count::Literal(1)),
            ),
        );
        let written = crate::ron::options().to_string(&parsed).unwrap();
        assert_eq!(read(&written), parsed);

        let no_max = read("ModifyPlayer(You, NoMax(HandSizeLimit))");
        assert_eq!(
            no_max,
            StaticEffect::ModifyPlayer(Reference::You, PlayerMod::NoMax(PlayerAttr::HandSizeLimit),),
        );
        let written = crate::ron::options().to_string(&no_max).unwrap();
        assert_eq!(read(&written), no_max);
    }

    /// `TriggerMultiplier` round-trips: Panharmonicon's artifact/creature-ETB
    /// cause with the default "you control" affected omitted from RON
    /// ([CR#603.2d]), and an explicit non-default affected preserved.
    #[test]
    fn trigger_multiplier_round_trips() {
        let parsed = read(
            "TriggerMultiplier(cause: ZoneMove(what: OneOf([Type(Artifact), Type(Creature)]), to: Battlefield), extra: 1)",
        );
        let StaticEffect::TriggerMultiplier {
            extra, affected, ..
        } = &parsed
        else {
            panic!("expected TriggerMultiplier, got {parsed:?}");
        };
        assert_eq!(*extra, Count::Literal(1));
        assert_eq!(*affected, affected_you_control(), "default is you-control");
        let written = crate::ron::options().to_string(&parsed).unwrap();
        assert!(
            !written.contains("affected"),
            "the default affected is omitted: {written}"
        );
        assert_eq!(read(&written), parsed);

        // An explicit non-default affected (an opponent doubler) is preserved.
        let opp = read(
            "TriggerMultiplier(cause: ZoneMove(what: Type(Creature), to: Battlefield), extra: 1, affected: ControlledBy(OpponentOf(Ref(You))))",
        );
        let written = crate::ron::options().to_string(&opp).unwrap();
        assert!(written.contains("affected"), "non-default affected kept");
        assert_eq!(read(&written), opp);
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
