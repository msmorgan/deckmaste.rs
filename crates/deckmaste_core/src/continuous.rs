use std::sync::Arc;

use serde::Deserialize;
use serde::Serialize;

use crate::Ability;
use crate::AsThough;
use crate::Color;
use crate::Condition;
use crate::CostComponent;
use crate::Count;
use crate::Deontic;
use crate::EventFilter;
use crate::Ident;
use crate::Predicate;
use crate::Reference;
use crate::RelationPredicate;
use crate::StatValue;
use crate::Supertype;
use crate::TurnMarker;
use crate::replacement::Prevention;
use crate::replacement::Replacement;

/// How long a one-shot-created continuous effect lasts ([CR#611.2]). Static
/// abilities don't carry this — their duration is implicit ("while it
/// functions", [CR#611.3]).
#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub enum Duration {
    /// Ends at a fixed turn-structure marker ([CR#611.2a]): end of turn
    /// sweeps in cleanup ([CR#514.2]), end of combat at the combat phase's
    /// end ([CR#500.5a,511.2]).
    FixedUntil(TurnMarker),
    /// Until an event happens (the engine pairs the undo one-shot, [CR#610.3]).
    UntilEvent(EventFilter),
    /// "For as long as" — a tracked predicate ([CR#611.2b]). The
    /// never-started / already-ended edge rules ride a `started` latch on
    /// the ENGINE's effect-instance record, not the card grammar; once
    /// stopped (including losing sight of a phased-out object,
    /// [CR#702.26f]) it never resumes.
    ForAsLongAs(Condition),
    /// In force while the CARRYING instruction's event executes — an
    /// instruction-scoped rider duration: "Destroy target creature. It can't
    /// be regenerated." rides the destroy event itself ([CR#701.19c] — a
    /// can't-be-regenerated effect causes regeneration shields to not be
    /// applied to that destruction). The footing for the `DestroyNoRegen`
    /// macro (macro-first-wave).
    ForThisEvent,
    /// For the rest of the game — the no-stated-duration default ([CR#611.2a]).
    EndOfGame,
}

/// A shared op for the NUMERIC characteristic axes — power, toughness, base
/// defense, base loyalty. `Set` overwrites the base value (layer 7b, or 7a when
/// CDA-flagged), `Up`/`Down` are the ±N modifications (layer 7c)
/// ([CR#613.4a..613.4c]). The op↔axis pairing is the soundness gate: a numeric
/// axis variant (`Modification::Power`, …) takes a `NumericOp`, recovering
/// Idris's `Numeric` type-class gate (`idris/src/Semantics.idr`) structurally.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub enum NumericOp {
    /// Overwrite the base value — layer 7b, or 7a when CDA-flagged
    /// ([CR#613.4a,613.4b]). Carries a [`StatValue`], not a bare [`Count`]: the
    /// value SET can be a printed-style scalar (incl. negative, [CR#107.1b]), a
    /// CDA `*/*` marker, or a dynamic count — the same value type as a card's
    /// printed base P/T. (`Up`/`Down` deltas stay non-negative [`Count`].)
    Set(StatValue),
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
/// type-class gate (`idris/src/Semantics.idr`) structurally.
///
/// Generic over the element type; serde's derive handles the generic directly.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub enum CollectionOp<T> {
    /// Overwrite the whole list (layer 4 types / layer 5 colors).
    Set(Arc<[T]>),
    /// Add one element.
    Add(T),
    /// Remove one element.
    Remove(T),
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
/// (Characteristic) (ModificationOp)` (`idris/src/Semantics.idr`): the
/// variant↔op pairing recovers the op↔axis soundness gate structurally (a
/// `Colors` takes a `CollectionOp`, which has no `Up`, so "raise a color" is
/// unrepresentable), while keeping RON readable (`Power(Up(1))`,
/// `Colors(Add(Blue))`).
///
/// Semantic lowering may bundle several authored changes into
/// `Several([Power(Up(p)), Toughness(Up(t))])`. `Several` is the
/// `Modification` analog of `Predicate::And`, but unlike `Predicate::And` (a
/// conjunction the engine evaluates), `Several` is semantically inert:
/// `changes` is already a flat,
/// layer-spanning list ([CR#613.6]) — so it is flattened away once, at the
/// engine boundary ([`Modification::flatten`]), and the engine layer loops
/// never see it.
#[derive(Debug, Clone, PartialEq, Eq, Hash, serde::Deserialize, serde::Serialize)]
pub enum Modification {
    /// Power ([CR#613.4]): `Set` base (7a/7b), `Up`/`Down` (7c).
    Power(NumericOp),
    /// Toughness ([CR#613.4]): `Set` base (7a/7b), `Up`/`Down` (7c).
    Toughness(NumericOp),
    /// Switch power and toughness ([CR#613.4d]).
    SwitchPowerToughness,
    /// Colors ([CR#613.1e], layer 5).
    Colors(CollectionOp<Color>),
    /// Card types by name (matched against the expanded `TypeDef`s, like
    /// `Subtypes`); layer 4 ([CR#613.1d]). A layer-4 op ident is resolved
    /// through the engine's type registry so a granted type's `confers` ride
    /// along.
    CardTypes(CollectionOp<Ident>),
    /// Subtypes by name (the class is derivable from the values — each card
    /// type has its own closed subtype set, [CR#205.3b]); layer 4. The element
    /// is a [`SubtypeRef`] (a resolved-def ref that SERIALIZES bare and reads
    /// via the subtype-macro channel), so semantic `Subtypes(Add(Zombie))`
    /// validates the name against the declared subtypes; the engine keys the
    /// layer-4 op off `SubtypeRef::name`.
    Subtypes(CollectionOp<crate::SubtypeRef>),
    /// Supertypes ([CR#613.1d], layer 4).
    Supertypes(CollectionOp<Supertype>),
    /// Gain an ability ([CR#613.1f]). Boxed: `Ability` is the enum's largest
    /// variant by far, so indirection keeps `Modification` small.
    GainAbility(Arc<Ability>),
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
    /// gain the basic-land mana ability (Blood Moon). One primitive, not
    /// reachable from the plain `Set*` ops.
    BecomeBasicLandType(Arc<[Ident]>),
    /// A bundle of ops contributed by one semantic authoring form — the analog
    /// of `Predicate::And`. For example, `PowerAndToughnessUp(p, t)` lowers to
    /// `Several([Power(Up(p)), Toughness(Up(t))])`. Semantically inert:
    /// `changes` is already a flat, layer-spanning list ([CR#613.6]), so
    /// [`Modification::flatten`] splices a `Several` into its parent list
    /// at the engine boundary and the engine never sees this variant.
    Several(Arc<[Modification]>),
}

impl Modification {
    /// Recursively splice every `Several` into the parent list. `Several` is a
    /// semantic grouping artifact; the engine consumes a flat,
    /// layer-spanning `changes` list ([CR#613.6]).
    #[must_use]
    pub fn flatten(changes: &[Modification]) -> Arc<[Modification]> {
        let mut out: Vec<Modification> = Vec::with_capacity(changes.len());
        for m in changes.iter().cloned() {
            match m {
                Modification::Several(inner) => {
                    out.extend(Modification::flatten(&inner).iter().cloned());
                }
                other => out.push(other),
            }
        }
        out.into()
    }
}

/// A step in the total-cost pipeline ([CR#601.2f]): base → +additional and
/// increases → −reductions (any order) → floor → lock. `Additional` is
/// pipeline-positional ([CR#118.8] — any number may stack, [CR#118.8a]);
/// it never changes the mana cost itself ([CR#118.8d]). Alternative costs
/// are NOT here — they swap the base and ride `May(Cast(cost: …))` rows
/// ([CR#118.9]).
#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub enum CostChange {
    Increase(Arc<[CostComponent]>),
    Reduce(Arc<[CostComponent]>),
    /// A MANDATORY "as an additional cost …" ([CR#118.8]). The optional
    /// kicker-family shape ("you may pay an additional …", [CR#118.8b])
    /// is NOT a pipeline step — it is a declared
    /// [`OptionalCost`](crate::OptionalCost) (`StaticSpec::CostOption`),
    /// announced at [CR#601.2b] and folded into the total at [CR#601.2f].
    Additional {
        components: Arc<[CostComponent]>,
    },
    /// A COUNT-SCALED change: the inner change applies `times` times at
    /// total-cost time ([CR#601.2f]). Covers both polarities — "costs {1}
    /// less for each artifact you control" ([CR#702.41a] affinity) and
    /// "costs {1} more for each …" taxers — and `times` is a [`Count`],
    /// so every counting form (`CountOf`, X, queries) composes. Boxed to
    /// break the self-reference.
    Scaled {
        change: Arc<CostChange>,
        times: Count,
    },
}

/// The default `affected` for a [`StaticSpec::TriggerMultiplier`]: "you
/// control" — the source permanent's controller ([CR#603.2c]). The common case
/// (Panharmonicon / Yarok), so it is the serde default and is omitted from RON.
fn affected_you_control() -> Predicate {
    Predicate::Relation(RelationPredicate::ControlledBy(Arc::new(Predicate::Ref(
        Reference::Reg(crate::RefId(1)),
    ))))
}

/// Whether an `affected` filter equals the "you control" default — skips it on
/// write so the common case stays flat.
fn is_affected_you_control(f: &Predicate) -> bool {
    *f == affected_you_control()
}

/// What a static ability states, and what a one-shot puts in force for a
/// duration — the shared currency between an "anthem" static ability
/// ([`Ability::Static`](crate::Ability::Static), [CR#604.1]) and a "+3/+3
/// until end of turn" one-shot ([`Instruction::Until`](crate::Instruction),
/// [CR#611.2]). NOT itself an effect: an ability generates effects rather
/// than being one ([CR#609.1]); the continuous effect ([CR#611.1]) is what
/// applying this spec establishes, which is why the type is not named
/// "static effect".
///
/// Most variants describe a continuous effect directly.
/// [`Replacement`](StaticSpec::Replacement) ([CR#614.1]) and
/// [`Prevention`](StaticSpec::Prevention) ([CR#615.1]) are the two applicable
/// continuous-effect subfamilies; self-replacement effects are deliberately
/// NOT among them, being effects of a resolving spell or ability rather than
/// continuous effects ([CR#614.15]). [`Each`](StaticSpec::Each) and
/// [`Conditionally`](StaticSpec::Conditionally) are authoring structure over
/// the other variants, not effects of their own.
#[derive(Debug, Clone, PartialEq, Eq, Hash, serde::Deserialize, serde::Serialize)]
pub enum StaticSpec {
    /// Change ONE object's characteristics ([CR#613]) — `Modify(It,
    /// PowerAndToughnessUp(2, 2))`. Positional — a single target [`Reference`]
    /// and a single [`Modification`] (bundle several ops with
    /// [`Modification::Several`]); the meaning is unambiguous, so no field
    /// names. Plurality is NEVER implicit here: to affect a set, distribute a
    /// single-object `Modify` with [`Each`](StaticSpec::Each) over a
    /// [`crate::Selection`]. Mirrors Idris `Modify : Reference AnObject ->
    /// Modification -> StaticSpec`.
    Modify(Reference, Modification),
    /// "[object] becomes a copy of [source]" ([CR#707.4]) — a CONTINUOUS
    /// layer-1a copy effect: the object stays on the battlefield (no
    /// leaves-/enters-the-battlefield triggers fire, [CR#707.4]) and keeps
    /// any non-copy effects presently affecting it, while its copiable
    /// values are replaced for as long as this effect lasts. Positional,
    /// mirroring [`Modify`](StaticSpec::Modify) — a single affected
    /// [`Reference`] and the shared [`crate::CopySpec`] payload (the same
    /// source + "except" exceptions [CR#707.9] the other three copy
    /// delivery sites carry: [`crate::TokenSpec::Copy`],
    /// [`crate::action::EnterRider::AsCopy`],
    /// [`crate::Action::CastCopy`]).
    ///
    /// NOT a [`Modification`]: `deckmaste_engine::layer::Layer` has no L1
    /// variant, since layer 1
    /// reshapes the *base* copiable values
    /// (`deckmaste_engine::layer::base_values`) rather than applying a
    /// per-op characteristic change; a `BecomesCopy` static is therefore a
    /// distinct `StaticSpec` variant, not a `Modify(_, Modification::…)`
    /// op. Authored via the shared one-shot-continuous machinery —
    /// `Continuously(effect: BecomesCopy(...), duration: ...)` for a single
    /// part, `Until(duration, [BecomesCopy(...), ...])` alongside other
    /// parts — exactly like [`Modify`](StaticSpec::Modify) is. Its
    /// layer-1a APPLICATION (deriving and installing the copiable values
    /// from the gathered `BecomesCopy` statics) is the
    /// `engine-layers-1-copy-facedown-text` seam in `base_values`; gathering
    /// this variant into a no-op here is a documented fizzle, never a
    /// panic.
    BecomesCopy(Reference, crate::CopySpec),
    /// Distribute an inner Static Spec over a [`crate::Selection`] — "for
    /// each object in the selection, enter the body with that object in its
    /// declared [`Provenance::Candidate`](crate::Provenance::Candidate)
    /// parameter and apply the inner effect." The ONLY way a static reaches many
    /// objects (there is no implicit whole-set scope). The selection is
    /// re-evaluated every layer pass, so the affected set tracks state changes
    /// live ([CR#613.6]). Mirrors Idris `Each : Bindable Many -> StaticSpec
    /// -> StaticSpec`; the body reads its candidate by register.
    Each(crate::Selection, Arc<crate::Region<StaticSpec>>),
    /// A conditional static ([CR#611.3a]) — "as long as [condition],
    /// [effect]." Wraps an inner Static Spec with a game-state condition; the
    /// effect applies only while the condition holds (re-checked continuously,
    /// never locked in). The `condition:` field of the deleted `StaticAbility`
    /// struct, now a composable effect wrapper. The layer engine rechecks the
    /// condition against its in-progress derived view ([CR#611.3a]); non-layer
    /// static consumers use the same wrapper as their collection gate.
    Conditionally(Condition, Arc<StaticSpec>),
    /// A deontic clause ([CR#101.2,601.3]).
    Deontic(Deontic),
    /// `who` plays the named rules [`Role`] — a quality of the object
    /// ([CR#113.12]), neither an ability nor a characteristic. The role is the
    /// primitive; the permissions, restrictions and damage semantics that come
    /// with it are DERIVED from this row, never its witness.
    Role { who: Predicate, role: Role },
    /// A cost modifier ([CR#118.7]).
    CostModifier { of: Predicate, change: CostChange },
    /// A declared OPTIONAL cost on this object's own casting
    /// ([CR#118.8b,601.2b]) — the kicker/multikicker/buyback identity
    /// ([`OptionalCost`](crate::OptionalCost)): "you may pay an additional
    /// [cost] as you cast this spell", read back through the tag by
    /// `Condition::PaidCost` / `Count::TimesPaid` / `Predicate::WasPaidWith`
    /// ([CR#702.33d..702.33e,607.2]).
    CostOption(crate::OptionalCost),
    /// A trigger multiplier ([CR#603.2d] — "triggers additional times"):
    /// Panharmonicon, Yarok, and the trigger half of Doubling Season. A
    /// triggered ability whose trigger matches `cause`, carried by a permanent
    /// matching `affected` (default "you control"), fires `extra` ADDITIONAL
    /// times. It is NOT a copy — each firing chooses its own modes and targets
    /// — and multipliers ADD rather than compound (two Panharmonicons → 3×,
    /// not 4×; [CR#603.2d] "doesn't invoke itself repeatedly").
    /// Panharmonicon = `TriggerMultiplier(cause: ZoneChange(what:
    /// Or([Type(Artifact), Type(Creature)]), to: Battlefield), extra:
    /// 1)`.
    TriggerMultiplier {
        cause: EventFilter,
        extra: Count,
        /// The affected ability's source permanent; defaults to "you control"
        /// (the source's controller), overridden for opponent/any-doublers.
        #[serde(
            default = "affected_you_control",
            skip_serializing_if = "is_affected_you_control"
        )]
        affected: Predicate,
    },
    /// A continuous modification to a player's numeric attribute ([CR#611]):
    /// extra land plays (Exploration = `ModifyPlayer(Ref(You),
    /// Up(LandPlaysPerTurn, 1))`, [CR#305.2]) or no maximum hand size
    /// (Reliquary Tower = `ModifyPlayer(Ref(You), NoMax(HandSizeLimit))`,
    /// [CR#402.2]). The object-modifying `Modify` touches objects only; this is
    /// its player-side twin. The `Reference` is the affected player ("you" by
    /// default — the source's controller).
    ModifyPlayer(Reference, PlayerMod),
    /// A replacement effect ([CR#614]). Boxed: `Replacement` is by far the
    /// largest payload here, so boxing keeps `StaticSpec` small
    /// (`clippy::large_enum_variant`).
    Replacement(Arc<Replacement>),
    /// A prevention effect ([CR#615]). Boxed for the same size reason as
    /// `Replacement`.
    Prevention(Arc<Prevention>),
    /// "Damage … can't be prevented" ([CR#615.12]): matching damage —
    /// `from` the source, `to` the recipient — is unpreventable. Gates
    /// exactly the [`Prevention`] class and nothing else, BY CONSTRUCTION:
    /// prevention is its own marked class, so the gate never touches
    /// generic replacements ([CR#614] Instead/Skip/Also — which still apply
    /// their non-prevention riders per [CR#615.12], an engine concern).
    CantPrevent { from: Predicate, to: Predicate },
    /// The mana counterfactual channel ([CR#609.4b] payment freedom): mana
    /// matching `mana_from` (a filter over its PRODUCER — "mana produced by
    /// Smokebraider") "may be spent as though it were mana of any
    /// [color/type]" (`as_`). Changes only HOW a cost may be paid — never
    /// the cost, the pool, or what was actually spent.
    SpendAsThough {
        mana_from: Predicate,
        as_: crate::SymbolPred,
    },
    /// A scoped counterfactual premise ([CR#609.4]) — see [`AsThough`].
    AsThough(AsThough),
    /// A conditional static ([CR#604.1]) — "any time `when` holds, do
    /// `then`". The instruction-payload sibling of `Conditionally`, which
    /// gates another static instead: `when` is evaluated with `This` = the
    /// carrying object, and `then` runs as part of the SBA sweep. Ascend on a
    /// permanent is the type case — [CR#702.131b] calls it a static ability,
    /// and its "any time you control ten or more permanents" gate is exactly
    /// this shape.
    ///
    /// NOT the home for a rules-defined state-based action ([CR#704.1] — an
    /// SBA is a game action, not an ability of any kind). A type or subtype's
    /// [CR#704] rule is conferred ability-free as
    /// [`Property::StateBased`](crate::Property::StateBased) (the Aura
    /// must-be-attached rule, [CR#704.5m]); a global one is an
    /// [`SbaRule`](crate::SbaRule). The sweep reads all three generically — it
    /// never branches on the Aura/Equipment/Fortification subtype.
    ///
    /// `then` is boxed (an `Instruction` dominates `StaticSpec`'s size;
    /// `Box` only for the size cycle, per the "Box only for cycles" rule).
    ConditionallyDo {
        when: Arc<Condition>,
        then: Arc<crate::Instruction>,
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
    OutcomeGate {
        who: Predicate,
        gate: OutcomeGateKind,
    },
    /// An event-side "can't happen" ([CR#614.17,702.12b]): the matching event
    /// can't occur. Distinct from `Deontic::Cant`, which is over player ACTIONS
    /// ([CR#101.2] action legality); destruction is an EVENT. Indestructible is
    /// `CantHappen(Destroyed(Ref(This)))`. Per [CR#614.17c] a can't-happen
    /// event can be touched only by a self-replacement, so the cant pass
    /// pre-empts the replacement registry entirely.
    CantHappen(EventFilter),
    /// ROLL-MORE replacement ([CR#614.3] Krark's Thumb-family; [CR#706.6] the
    /// ignore-result semantics): "if you would roll/flip `query`, instead
    /// roll/flip `extra` more and ignore per `ignore`." Krark's Thumb =
    /// `ReplaceRoll { query: CoinFlipped(by: Ref(You)), extra: 1, ignore:
    /// IgnoreChosen(1) }`. Mirrors Idris `ReplaceRoll : (q : EventQuery b) ->
    /// {auto 0 rnd : isRandomnessQuery q = True} -> (extra : Count b) ->
    /// (ignore : IgnoreRule) -> StaticSpec b` — the `isRandomnessQuery`
    /// erased auto-proof (gating `query` to a `FlipCoin`/`RollDice` kind
    /// only, never the planar die, [CR#901.9d]) is Idris-ONLY, same as the
    /// `EventCaps` proofs `EventFilter::TapForMana`/`ManaSpec::
    /// ProducedByEvent` document: Rust holds no compile-time gate, so an
    /// semantic-input error that reaches for `ReplaceRoll` over a
    /// non-randomness query simply fails to typecheck on the Idris side
    /// (`idris-check`), the intended soundness gate. Held unboxed like its
    /// `TriggerMultiplier` sibling (same `EventFilter` + `Count` shape,
    /// already unboxed).
    ReplaceRoll {
        query: EventFilter,
        extra: Count,
        ignore: IgnoreRule,
    },
    /// ALTERNATIVE PAYMENT of individual cost pips — NOT a cost reduction and
    /// NOT mana production. "For each [`PipClass`] pip in this spell's total
    /// cost, you may [`PayAct`] rather than pay that mana": convoke
    /// ([CR#702.51a]), delve ([CR#702.66a]), improvise ([CR#702.126a]). It
    /// "isn't an additional or alternative cost and applies only after the
    /// total cost of the spell ... is determined" ([CR#702.51b]); the total
    /// cost and the mana value ([CR#202.3]) are NEVER mutated — paying a pip
    /// this way still counts as paying the original cost ([CR#118.7]). The
    /// engine reads this in exactly ONE step, the cost-payment window
    /// ([CR#601.2g..601.2h]) of the casting that grants it: it walks the
    /// locked-in cost's pips and offers each eligible pip its alternative,
    /// substituting how the pip is paid, never the total. Read by no other
    /// engine step, so the "while on the stack" lifetime ([CR#702.51a]) needs
    /// no separate is-active predicate.
    PayPips(PipClass, PayAct),
}

/// A bundled rules-domain membership an object plays ([Game Model glossary]),
/// conferred by [`StaticSpec::Role`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub enum Role {
    /// The creature-like combat role: power governs combat and fight damage
    /// ([CR#510.1a,701.14a]), damage is marked on it and compared with its
    /// toughness ([CR#120.3e,704.5g]), it is in the attacking and blocking domains
    /// ([CR#508.1a,509.1a]), and the summoning-sickness rule applies to its
    /// attacks and its tap/untap-symbol activations ([CR#302.6]).
    Combatant,
}

/// Which outcome a [`StaticSpec::OutcomeGate`] suppresses.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub enum OutcomeGateKind {
    /// Suppresses the loss SBAs ([CR#704.5a..704.5c]) and "loses the
    /// game" effect outcomes ([CR#104.3e]) for matching players.
    CantLose,
    /// Suppresses "wins the game" effect outcomes ([CR#104.2b]); the
    /// all-opponents-left win ([CR#104.2a]) bypasses it.
    CantWin,
}

/// Which flip(s)/roll(s) a [`StaticSpec::ReplaceRoll`] discards, once the
/// extras have been rolled ([CR#706.6]: "if a player is instructed to
/// ignore a roll ... the player chooses one of those rolls to be ignored"
/// when multiple tie for lowest). `IgnoreLowest` = an automatic
/// ignore-the-lower-result rule; `IgnoreChosen(n)` = the player picks which
/// `n` of the extras to discard (Krark's Thumb's own "ignore one" —
/// [CR#706.6] settles this as the flipper's choice, not a forced-lowest
/// rule). Mirrors Idris `IgnoreRule = IgnoreLowest | IgnoreChosen Nat`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub enum IgnoreRule {
    IgnoreLowest,
    IgnoreChosen(crate::Uint),
}

/// Which pips of a spell's locked-in total cost a [`StaticSpec::PayPips`]
/// alternative may pay ([CR#601.2g]). `Generic` matches a generic pip (delve /
/// improvise / convoke's generic clause); `Colored` a colored pip of the named
/// color (convoke's per-color clause — "an untapped creature of that color",
/// [CR#702.51a]). Spelled `Colored(Color)` rather than the Idris `ColorPip`'s
/// implicit color-match: the color is named here, and a convoke keyword expands
/// to one variant per color so the tapped creature's color is enforced
/// structurally.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub enum PipClass {
    /// A generic pip ({1}) — delve, improvise, convoke's generic clause.
    Generic,
    /// A colored pip of the named color — convoke's colored clause
    /// ([CR#702.51a]).
    Colored(Color),
}

/// The per-pip alternative-payment action a [`StaticSpec::PayPips`] performs
/// "rather than pay that mana" ([CR#702.51a,702.66a,702.126a]). The object is
/// chosen at payment time ([CR#601.2g]); `Predicate` is open (plugin-safe).
#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub enum PayAct {
    /// Tap an untapped permanent matching the filter you control
    /// ([CR#107.5]): convoke taps a creature ([CR#702.51a]), improvise an
    /// artifact ([CR#702.126a]).
    TapToPay(Predicate),
    /// Exile a matching card from your graveyard: delve ([CR#702.66a]).
    ExileToPay(Predicate),
}

/// A player's numeric attribute a [`StaticSpec::ModifyPlayer`] adjusts — the
/// player-side twin of an object [`Modification`] axis. `HandSizeLimit`
/// (normally seven, [CR#402.2]) and `LandPlaysPerTurn` (normally one,
/// [CR#305.2]) are the caps continuous statics modify (Reliquary Tower /
/// Exploration); `Life` ([CR#119.1]) and `HandSize` ([CR#402.2]) round out the
/// readable player attributes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub enum PlayerAttr {
    Life,
    HandSize,
    HandSizeLimit,
    LandPlaysPerTurn,
}

/// A continuous modification to a player attribute, carried by
/// [`StaticSpec::ModifyPlayer`] — the player-side twin of [`NumericOp`]
/// ([CR#611]; players have no [CR#613] layers, so these apply directly).
/// `Set`/`Up`/`Down` adjust a count-valued attribute (Exploration =
/// `Up(LandPlaysPerTurn, 1)`); `NoMax` removes a cap (Reliquary Tower =
/// `NoMax(HandSizeLimit)`, "no maximum hand size") — kept a dedicated op, not a
/// `Maybe Count` value, since a player attribute reads as a count.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub enum PlayerMod {
    /// Overwrite the attribute with a fixed value.
    Set(PlayerAttr, Count),
    /// "+N" the attribute.
    Up(PlayerAttr, Count),
    /// "−N" the attribute.
    Down(PlayerAttr, Count),
    /// Remove the attribute's maximum ("no maximum hand size").
    NoMax(PlayerAttr),
}
