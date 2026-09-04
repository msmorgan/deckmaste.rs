//! The card-domain macro configuration: which semantics types are macroable,
//! and with what reader policy, glued onto [`macro_ron`]. Definitions live in
//! `plugins/*/macros/**/*.ron` (paths and file names are organizational
//! only) and are invoked by name where a value of one of their kinds is
//! expected — see [`crate::plugin`] for loading and the `macro_ron` crate
//! docs for the language itself.
//!
//! Subtype definitions are ordinary macro files, usually written as
//! meta-macro invocations (`CreatureType(name: "TimeLord", template:
//! "Time Lord")`) — `name` is the registration ident cards invoke,
//! `template` the printed name carried in the value.

// The kind registry, param-type registry, and default macro set are
// `deckmaste_semantics`'s — see `deckmaste_semantics::macros` for the
// definitions and their docs.
pub use deckmaste_semantics::macros::kinds;
pub use deckmaste_semantics::macros::macro_set;
pub use deckmaste_semantics::macros::param_types;
pub use macro_ron::InsertError;
pub use macro_ron::MacroDef;
pub use macro_ron::MacroSet;
pub use macro_ron::ParamType;
pub use macro_ron::ParamTypeSet;
pub use macro_ron::Params;

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use deckmaste_lowering::Lower;
    use deckmaste_semantics::Ability;
    use deckmaste_semantics::Action;
    use deckmaste_semantics::AsThough;
    use deckmaste_semantics::CardFace;
    use deckmaste_semantics::CharacteristicPredicate;
    use deckmaste_semantics::ColorOrColorless;
    use deckmaste_semantics::Condition;
    use deckmaste_semantics::CostComponent;
    use deckmaste_semantics::Count;
    use deckmaste_semantics::Counter;
    use deckmaste_semantics::Destination;
    use deckmaste_semantics::EventFilter;
    use deckmaste_semantics::KeywordAbility;
    use deckmaste_semantics::LifeOp;
    use deckmaste_semantics::ManaProduction;
    use deckmaste_semantics::ManaRider;
    use deckmaste_semantics::ManaSpec;
    use deckmaste_semantics::ManaSymbol;
    use deckmaste_semantics::Modification;
    use deckmaste_semantics::ObjectKind;
    use deckmaste_semantics::OneShotEffect;
    use deckmaste_semantics::Predicate;
    use deckmaste_semantics::Quantity;
    use deckmaste_semantics::Reference;
    use deckmaste_semantics::Replacement;
    use deckmaste_semantics::Selection;
    use deckmaste_semantics::SimpleManaSymbol;
    use deckmaste_semantics::StatValue;
    use deckmaste_semantics::StatePredicate;
    use deckmaste_semantics::StaticEffect;
    use deckmaste_semantics::Subtype;
    use deckmaste_semantics::TargetSpec;
    use deckmaste_semantics::Type;
    use deckmaste_semantics::Zone;

    use super::*;

    /// Parses a definition the way plugin loading does: from RON source in
    /// deckmaste's dialect. (`MacroDef`'s body field is crate-private in
    /// `macro_ron`, so file-shaped source is the construction path here —
    /// which is also what real definitions are.)
    fn def(source: &str) -> MacroDef {
        deckmaste_semantics::ron::options()
            .from_str(source)
            .unwrap()
    }

    /// The semantics grammar owns the kind registry
    /// (`deckmaste_semantics::macros::kinds`); this pins its name roster. The
    /// derived `kind()`s self-name from the Rust ident, so they track renames
    /// by construction; the tie this pins is the hand-registered struct kinds
    /// (`CardFace`, `Subtype`, `TypeDef`, `Macro`), which a Rust rename would
    /// strand without a compile error — plus the policy list's completeness
    /// (the length check).
    #[test]
    fn kind_names_track_the_semantics_types() {
        fn name_of<T>() -> &'static str {
            std::any::type_name::<T>().rsplit("::").next().unwrap()
        }
        let names = [
            name_of::<Ability>(),
            name_of::<Action>(),
            name_of::<AsThough>(),
            name_of::<CardFace>(),
            name_of::<Condition>(),
            name_of::<Count>(),
            name_of::<CostComponent>(),
            name_of::<Destination>(),
            name_of::<Zone>(),
            name_of::<OneShotEffect>(),
            name_of::<EventFilter>(),
            name_of::<Predicate>(),
            name_of::<KeywordAbility>(),
            name_of::<ManaRider>(),
            // The mana specs / production wrapper and the symbol leaves embed
            // their inner value untagged (`#[macro_ron(embed)]`); `StatValue`
            // reads a bare-integer `literal` — all registered so those spellings
            // stay flat and macro-aware.
            name_of::<ManaProduction>(),
            name_of::<ManaSpec>(),
            name_of::<deckmaste_semantics::Color>(),
            name_of::<ColorOrColorless>(),
            name_of::<SimpleManaSymbol>(),
            name_of::<ManaSymbol>(),
            name_of::<StatValue>(),
            name_of::<Modification>(),
            name_of::<Quantity>(),
            name_of::<Reference>(),
            name_of::<Replacement>(),
            name_of::<Selection>(),
            name_of::<StaticEffect>(),
            // The spec §4 stragglers: the restricted root and the supertype
            // vocabulary, registered so the author-surface ban is uniform.
            name_of::<deckmaste_semantics::Card>(),
            name_of::<deckmaste_semantics::Supertype>(),
            name_of::<Subtype>(),
            name_of::<deckmaste_semantics::TypeDef>(),
            name_of::<TargetSpec>(),
            name_of::<Counter>(),
            name_of::<deckmaste_semantics::DesignationDecl>(),
            "Macro", // hand-registered: MacroDef's serde rename, loader-only
            // hand-registered: the keyword-action verb grouping kind (name-
            // erasing like `TypeDef`), collected into the plugin's verb table.
            "KeywordAction",
        ];
        let kinds = deckmaste_semantics::macros::kinds();
        for name in names {
            assert!(kinds.contains(name), "`{name}` is not a registered kind");
        }
        assert_eq!(kinds.len(), names.len());
    }

    /// Predicate's generated Deserialize calls `deserialize_enum` with the
    /// full flattened variant list (its own names plus the compartments'):
    /// that is what lets unknown names at Predicate positions fall through to
    /// the macro namespace.
    #[test]
    fn filter_positions_expand_macros() {
        let mut macros = macro_set();
        macros
            .insert(&def(r#"(
                    name: "AnyTargetish",
                    kinds: [Predicate],
                    body: Or([Kind(Player), And([InZone(Battlefield), Type(name:"Creature",permanent:true)])]),
                )"#))
            .unwrap();
        let filter: Predicate = macros.read_str("AnyTargetish").unwrap();
        // The invocation is remembered; the expansion lives under `.value`.
        let Predicate::Expanded(expanded) = filter else {
            panic!("expected a remembered filter, got {filter:?}");
        };
        assert_eq!(expanded.name, "AnyTargetish");
        let Predicate::Or(arms) = *expanded.value else {
            panic!("expected Or, got {:?}", expanded.value);
        };
        assert_eq!(arms[0], Predicate::Kind(ObjectKind::Player));
        // The nested arm proves Predicate positions *inside* an expansion stay
        // macro-aware too.
        assert_eq!(
            arms[1],
            Predicate::And(
                vec![
                    Predicate::State(StatePredicate::InZone(Zone::Battlefield)),
                    Predicate::creature(),
                ]
                .into()
            )
        );
        assert_eq!(arms.len(), 2);
    }

    /// A `Modification`-kind macro (`PowerAndToughnessUp`) expands at a
    /// `changes: [...]` slot: it is remembered as a `Modification::Expanded`,
    /// its body is the `Several` bundle, the invocation round-trips as
    /// written, and `Modification::flatten` (the engine boundary) splices it
    /// to the flat two-op list. The keystone of the change-bundling design.
    #[test]
    fn modification_positions_expand_and_flatten() {
        use deckmaste_semantics::Expand as _;
        use deckmaste_semantics::Modification;
        use deckmaste_semantics::NumericOp;

        let mut macros = macro_set();
        macros
            .insert(&def(r#"(
                    name: "PowerAndToughnessUp",
                    template: "gets +${0}/+${1}",
                    kinds: [Modification],
                    params: [Count, Count],
                    body: Several([Power(Up(Param(0))), Toughness(Up(Param(1)))]),
                )"#))
            .unwrap();

        // Read a whole `Modify` whose `Several` bundle mixes the bundling
        // macro with a plain ability grant — the Overrun shape.
        let effect: StaticEffect = macros
            .read_str(
                "Modify(This, Several([PowerAndToughnessUp(3, 3), GainAbility(Keyword(Trample))]))",
            )
            .unwrap();
        let StaticEffect::Modify(of, change) = &effect else {
            panic!("expected Modify, got {effect:?}");
        };
        assert_eq!(*of, Reference::This);
        let Modification::Several(changes) = change else {
            panic!("expected a Several bundle, got {change:?}");
        };
        // The macro is remembered as the first element; the grant is plain.
        let Modification::Expanded(exp) = &changes[0] else {
            panic!("expected a remembered modification, got {:?}", changes[0]);
        };
        assert_eq!(exp.name, "PowerAndToughnessUp");
        assert_eq!(
            *exp.value,
            Modification::Several(
                vec![
                    Modification::Power(NumericOp::Up(Count::Literal(3))),
                    Modification::Toughness(NumericOp::Up(Count::Literal(3))),
                ]
                .into()
            )
        );

        // Round-trips as the invocation, not the expansion.
        let written = deckmaste_semantics::ron::options()
            .to_string(&effect)
            .unwrap();
        assert_eq!(
            written,
            "Modify(This,Several([PowerAndToughnessUp(3,3),GainAbility(Keyword(Trample))]))"
        );

        // `expand_all` then `flatten` → the flat three-op engine-facing list:
        // the bundle is spliced into the two P/T ops, the grant follows.
        let StaticEffect::Modify(_, change) = effect.expand_all() else {
            unreachable!()
        };
        let Modification::Several(changes) = change else { unreachable!() };
        let flat = Modification::flatten(&changes);
        assert_eq!(flat.len(), 3, "Several spliced to flat ops: {flat:?}");
        assert_eq!(
            flat[0],
            Modification::Power(NumericOp::Up(Count::Literal(3)))
        );
        assert_eq!(
            flat[1],
            Modification::Toughness(NumericOp::Up(Count::Literal(3)))
        );
        assert!(
            matches!(flat[2], Modification::GainAbility(_)),
            "the grant follows the P/T ops: {:?}",
            flat[2]
        );
    }

    /// `PowerAndToughnessUp`/`Down` are the parse surfaces (arity-2, `Count,
    /// Count`); each delegates to the arity-2 `PowerAndToughness(NumericOp,
    /// NumericOp)` building block by FORWARDING its own params into the
    /// delegate's arguments (`PowerAndToughness(Up(Param(0)), Up(Param(1)))`)
    /// — the Task 1 nested-macro forwarding mechanism. Expanding all the way
    /// through both macro layers must land on the flat `Several` bundle, not
    /// stop at the delegate's own `Expanded` wrapper.
    #[test]
    fn pt_up_down_expand_through_delegate() {
        use deckmaste_semantics::Expand as _;
        use deckmaste_semantics::NumericOp;

        let mut macros = macro_set();
        macros
            .insert(&def(r#"(
                    name: "PowerAndToughness",
                    kinds: [Modification],
                    params: [NumericOp, NumericOp],
                    body: Several([Power(Param(0)), Toughness(Param(1))]),
                )"#))
            .unwrap();
        macros
            .insert(&def(r#"(
                    name: "PowerAndToughnessUp",
                    template: "gets +${0}/+${1}",
                    kinds: [Modification],
                    params: [Count, Count],
                    body: PowerAndToughness(Up(Param(0)), Up(Param(1))),
                )"#))
            .unwrap();
        macros
            .insert(&def(r#"(
                    name: "PowerAndToughnessDown",
                    template: "gets -${0}/-${1}",
                    kinds: [Modification],
                    params: [Count, Count],
                    body: PowerAndToughness(Down(Param(0)), Down(Param(1))),
                )"#))
            .unwrap();

        let up: Modification = macros.read_str("PowerAndToughnessUp(2, 2)").unwrap();
        assert_eq!(
            deckmaste_semantics::ron::options().to_string(&up).unwrap(),
            "PowerAndToughnessUp(2,2)"
        );
        assert_eq!(
            up.expand_all(),
            Modification::Several(
                vec![
                    Modification::Power(NumericOp::Up(Count::Literal(2))),
                    Modification::Toughness(NumericOp::Up(Count::Literal(2))),
                ]
                .into()
            )
        );

        let down: Modification = macros.read_str("PowerAndToughnessDown(2, 2)").unwrap();
        assert_eq!(
            deckmaste_semantics::ron::options()
                .to_string(&down)
                .unwrap(),
            "PowerAndToughnessDown(2,2)"
        );
        assert_eq!(
            down.expand_all(),
            Modification::Several(
                vec![
                    Modification::Power(NumericOp::Down(Count::Literal(2))),
                    Modification::Toughness(NumericOp::Down(Count::Literal(2))),
                ]
                .into()
            )
        );
    }

    /// End-to-end guard for the `macro-param-forwarding` capability on REAL
    /// card types: a `Cost`-typed (`Vec<CostComponent>`) whole-value `Param`
    /// forwarded through one macro into a nested one, landing at the `cost:
    /// Components(…)` slot — the exact shape the `Flashback.ron` lore once
    /// claimed the macro layer "can't forward" (it errored with "Expected
    /// opening `[`" before `forward_arg` landed). `CastFromGraveyard` factors
    /// Flashback's cast half; `RelayCast` stands in for a keyword body that
    /// forwards its own cost `Param` into that factored macro. Both fully
    /// expand to the same `May(Cast(…))`, differing only in the remembered
    /// invocation wrapper `expand_all` strips.
    #[test]
    fn cost_param_forwards_into_nested_macro() {
        use deckmaste_semantics::Expand;

        let mut set = macro_set();
        set.insert(&def(r#"(
            name: "CastFromGraveyard",
            kinds: [StaticEffect],
            params: [Cost],
            body: May(Cast(what: Ref(This), from: Graveyard, cost: Components(Param(0)), tag: Flashback)),
        )"#))
            .unwrap();
        set.insert(&def(r#"(
            name: "RelayCast",
            kinds: [StaticEffect],
            params: [Cost],
            body: CastFromGraveyard(Param(0)),
        )"#))
            .unwrap();

        let direct: StaticEffect = set
            .read_str("CastFromGraveyard([Mana([Generic(2)])])")
            .unwrap();
        let forwarded: StaticEffect = set.read_str("RelayCast([Mana([Generic(2)])])").unwrap();

        // Forwarding resolved the whole-value `Cost` param through both macro
        // layers: the fully-expanded permissions are identical.
        assert_eq!(direct.clone().expand_all(), forwarded.expand_all());
        // ...and it is the concrete alternative cost, not a dangling `Param`.
        let concrete: StaticEffect = set
            .read_str(
                "May(Cast(what: Ref(This), from: Graveyard, cost: Components([Mana([Generic(2)])]), tag: Flashback))",
            )
            .unwrap();
        assert_eq!(direct.expand_all(), concrete);
    }

    /// The `P1P1ForEach` macro (real `plugins/builtin` corpus — the nested
    /// `Permanent`/`PowerAndToughnessBoth` delegates must actually load, not
    /// a hand-inserted stand-in) is the stored form of "gets +1/+1 for each
    /// <selection>" pumps ([CR#613.4c] layer 7c, [CR#107.3] "for each"): its
    /// `Predicate` param wraps into `CountOf(Objects(...))` ONCE and forwards
    /// into `PowerAndToughnessBoth`'s single `NumericOp` slot, so both axes
    /// read the SAME computed count — genuinely two real scaling axes.
    /// Expanding all the way through both macro layers (`P1P1ForEach` →
    /// `PowerAndToughnessBoth`) must land on the flat two-axis `Several`.
    #[test]
    fn p1p1_for_each_expands_and_reserializes() {
        use deckmaste_semantics::Count;
        use deckmaste_semantics::Countable;
        use deckmaste_semantics::Expand as _;
        use deckmaste_semantics::NumericOp;

        let macros = builtin().macros;
        let pred_src = r"And([Permanent,Type(Creature),ControlledBy(Ref(You))])";
        // Expanded, matching how `m.expand_all()` below normalizes the SAME
        // predicate nested inside the macro's own `Param(0)` substitution
        // (`Permanent` is itself a macro; both sides must reduce it away).
        let pred: Predicate = macros.read_str(pred_src).unwrap();
        let pred = pred.expand_all();

        let ron = format!("P1P1ForEach({pred_src})");
        let m: Modification = macros.read_str(&ron).unwrap();
        let Modification::Expanded(exp) = &m else {
            panic!("expected a remembered modification, got {m:?}");
        };
        assert_eq!(exp.name, "P1P1ForEach");

        // Round-trips to the invocation, not the expansion.
        assert_eq!(
            deckmaste_semantics::ron::options().to_string(&m).unwrap(),
            ron
        );

        // Fully expanded (through the `PowerAndToughnessBoth` delegate)
        // lands on the flat two-axis `Several`, both sides reading the SAME
        // count — the "+1/+1 for each" shape Task 1's render arm consumes.
        let count = Count::CountOf(Countable::Objects(Arc::new(pred)));
        assert_eq!(
            m.expand_all(),
            Modification::Several(
                vec![
                    Modification::Power(NumericOp::Up(count.clone())),
                    Modification::Toughness(NumericOp::Up(count)),
                ]
                .into()
            )
        );
    }

    /// The `P1P0ForEach` macro — the lone-`Power` twin: only ONE axis
    /// genuinely scales (the printed "+1/+0"'s toughness half is
    /// template-literal text, not a real op), so its body is a bare
    /// `Power(Up(CountOf(Objects(...))))` — no `PowerAndToughnessBoth`/
    /// `Several` wrapping at all, unlike `P1P1ForEach`.
    #[test]
    fn p1p0_for_each_expands_to_lone_power() {
        use deckmaste_semantics::Count;
        use deckmaste_semantics::Countable;
        use deckmaste_semantics::Expand as _;
        use deckmaste_semantics::NumericOp;

        let macros = builtin().macros;
        let pred_src = r"And([Permanent,Type(Creature),ControlledBy(Ref(You))])";
        // Expanded, matching how `m.expand_all()` below normalizes the SAME
        // predicate nested inside the macro's own `Param(0)` substitution
        // (`Permanent` is itself a macro; both sides must reduce it away).
        let pred: Predicate = macros.read_str(pred_src).unwrap();
        let pred = pred.expand_all();

        let ron = format!("P1P0ForEach({pred_src})");
        let m: Modification = macros.read_str(&ron).unwrap();
        let Modification::Expanded(exp) = &m else {
            panic!("expected a remembered modification, got {m:?}");
        };
        assert_eq!(exp.name, "P1P0ForEach");
        assert_eq!(
            deckmaste_semantics::ron::options().to_string(&m).unwrap(),
            ron
        );

        let count = Count::CountOf(Countable::Objects(Arc::new(pred)));
        assert_eq!(m.expand_all(), Modification::Power(NumericOp::Up(count)));
    }
    /// Same pin for Selection positions: nothing exercises Selection macros
    /// in real data yet, and Plan 2 will make this path load-bearing.
    #[test]
    fn selection_positions_expand_macros() {
        let mut macros = macro_set();
        macros
            .insert(&def(r#"(
                    name: "EachCreature",
                    kinds: [Selection],
                    body: SelectAll(Type(name:"Creature",permanent:true)),
                )"#))
            .unwrap();
        let selection: Selection = macros.read_str("EachCreature").unwrap();
        let Selection::Expanded(expanded) = selection else {
            panic!("expected a remembered selection, got {selection:?}");
        };
        assert_eq!(expanded.name, "EachCreature");
        assert_eq!(
            *expanded.value,
            Selection::SelectAll(Predicate::r#type(Type::Creature))
        );
    }

    /// Loads the real `plugins/builtin` corpus — the layered-predicate pin
    /// below needs the actual on-disk `OtherCreatureYouControl`/
    /// `OtherCreaturesYouControl` files, not a hand-inserted stand-in.
    fn builtin() -> crate::plugin::Plugin {
        crate::plugin::Plugin::load(
            std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../plugins/builtin"),
        )
        .unwrap()
    }

    /// The bare-verb keyword-action pattern TWINS (`Destroy(Ref(This))`,
    /// `Scry(Ref(You))`, `Discard(Ref(You), Any)`) expand, at the `EventFilter`
    /// position, to the `Act` master form ([CR#701]) — the ruled spelling that
    /// replaces the retired nested `Act(Verb(..))`. Each twin shares its NAME
    /// with the same-named `KeywordAction` verb macro; the `(kind, name)`
    /// namespace keeps them distinct. The `CantHappen(Destroy(Ref(This)))`
    /// static (indestructible's migrated spelling) expands the same twin at a
    /// `StaticEffect`'s filter slot.
    #[test]
    fn keyword_action_pattern_twins_expand_to_the_act_master_form() {
        use deckmaste_semantics::EventFilter;
        use deckmaste_semantics::Predicate;
        use deckmaste_semantics::Reference;
        use deckmaste_semantics::StaticEffect;
        use deckmaste_semantics::VerbName;

        let macros = builtin().macros;

        // Single-slot object twin: `Destroy(Ref(This))` → the master form's
        // `on` slot; `who`/`cause` default.
        let destroy: EventFilter = macros.read_str("Destroy(Ref(This))").unwrap();
        let EventFilter::Expanded(exp) = &destroy else {
            panic!("expected a remembered twin, got {destroy:?}");
        };
        assert_eq!(exp.name, "Destroy");
        assert_eq!(
            *exp.value,
            EventFilter::Act {
                verb: VerbName::from("Destroy"),
                who: Predicate::Any,
                on: Predicate::Ref(Reference::This),
                cause: None,
            },
        );

        // Single-slot player-report twin: `Scry(Ref(You))` → the `who` slot.
        let scry: EventFilter = macros.read_str("Scry(Ref(You))").unwrap();
        let EventFilter::Expanded(exp) = &scry else {
            panic!("expected a remembered twin, got {scry:?}");
        };
        assert_eq!(
            *exp.value,
            EventFilter::Act {
                verb: VerbName::from("Scry"),
                who: Predicate::Ref(Reference::You),
                on: Predicate::Any,
                cause: None,
            },
        );

        // Two-slot twin: `Discard(Ref(You), Any)` → both performer and card.
        let discard: EventFilter = macros.read_str("Discard(Ref(You), Any)").unwrap();
        let EventFilter::Expanded(exp) = &discard else {
            panic!("expected a remembered twin, got {discard:?}");
        };
        assert_eq!(
            *exp.value,
            EventFilter::Act {
                verb: VerbName::from("Discard"),
                who: Predicate::Ref(Reference::You),
                on: Predicate::Any,
                cause: None,
            },
        );

        // Indestructible's migrated spelling: the twin at a StaticEffect slot.
        let cant: StaticEffect = macros.read_str("CantHappen(Destroy(Ref(This)))").unwrap();
        let StaticEffect::CantHappen(inner) = &cant else {
            panic!("expected CantHappen, got {cant:?}");
        };
        let EventFilter::Expanded(exp) = inner else {
            panic!("expected a remembered twin inside CantHappen, got {inner:?}");
        };
        assert_eq!(
            *exp.value,
            EventFilter::Act {
                verb: VerbName::from("Destroy"),
                who: Predicate::Any,
                on: Predicate::Ref(Reference::This),
                cause: None,
            },
        );
    }

    /// `OtherCreaturesYouControl` (Selection) nullary-nests the reusable
    /// `OtherCreatureYouControl` (Predicate) atom rather than repeating the
    /// `And([...])` filter inline — the layered-predicate refactor. The
    /// invocation round-trips through BOTH macro layers (each macro name
    /// remembered, not flattened away) and the predicate atom alone expands
    /// to its own `And([...])` filter.
    #[test]
    fn selection_wrappers_expand_through_predicate_atoms() {
        use deckmaste_semantics::Expand as _;
        use deckmaste_semantics::RelationPredicate;
        use deckmaste_semantics::Zone;

        let macros = builtin().macros;

        // The invocation round-trips exactly as authored (bare name, no
        // args) — same shape as `pt_up_down_expand_through_delegate`'s
        // `to_string`, which preserves the OUTER invocation regardless of
        // what its body nests.
        let sel: Selection = macros.read_str("OtherCreaturesYouControl").unwrap();
        assert_eq!(
            deckmaste_semantics::ron::options().to_string(&sel).unwrap(),
            "OtherCreaturesYouControl"
        );

        // One level down, `SelectAll`'s argument is itself a REMEMBERED
        // `Predicate` macro invocation — proof the wrapper nullary-nests the
        // atom rather than inlining its `And([...])` body.
        let Selection::Expanded(sel_exp) = &sel else {
            panic!("expected a remembered selection, got {sel:?}");
        };
        let Selection::SelectAll(inner) = sel_exp.value.as_ref() else {
            panic!("expected SelectAll, got {:?}", sel_exp.value);
        };
        let Predicate::Expanded(pred_exp) = inner else {
            panic!("expected the nested predicate atom to stay remembered, got {inner:?}");
        };
        assert_eq!(pred_exp.name, "OtherCreatureYouControl");

        // Fully expanding through BOTH macro layers (the selection wrapper,
        // then the predicate atom, then `Creature`'s own nested `Permanent`)
        // lands on the flat filter — the keystone this test pins.
        let creature = Predicate::And(
            vec![
                Predicate::State(StatePredicate::InZone(Zone::Battlefield)),
                Predicate::creature(),
            ]
            .into(),
        );
        let expected_atom = Predicate::And(
            vec![
                creature,
                Predicate::Not(Arc::new(Predicate::Ref(Reference::This))),
                Predicate::Relation(RelationPredicate::ControlledBy(Arc::new(Predicate::Ref(
                    Reference::You,
                )))),
            ]
            .into(),
        );
        assert_eq!(
            sel.expand_all(),
            Selection::SelectAll(expected_atom.clone())
        );

        // The predicate atom on its own expands to the same `And([...])`.
        let pred: Predicate = macros.read_str("OtherCreatureYouControl").unwrap();
        assert_eq!(
            deckmaste_semantics::ron::options()
                .to_string(&pred)
                .unwrap(),
            "OtherCreatureYouControl"
        );
        assert_eq!(pred.expand_all(), expected_atom);
    }

    /// A bare `Reference` reads natively at a `Reference` slot. Verb patients
    /// are single references now, so `This` / `It` /
    /// `ControllerOf(This)` read straight into `Reference` — the old
    /// `Selection::Ref` embed (a bare reference at a `Selection` slot) is gone.
    #[test]
    fn bare_reference_reads_at_reference_slot() {
        let macros = macro_set();
        assert_eq!(
            macros.read_str::<Reference>("This").unwrap(),
            Reference::This
        );
        assert_eq!(macros.read_str::<Reference>("It").unwrap(), Reference::It);
        assert_eq!(
            macros.read_str::<Reference>("ControllerOf(This)").unwrap(),
            Reference::ControllerOf(Arc::new(Reference::This)),
        );
    }

    /// A `Reference` *macro* invoked at a `Reference` slot expands through
    /// `Reference`'s own reader and is remembered as a `Reference::Expanded`.
    /// (The old `Selection::Ref` wrapper is gone — references live at reference
    /// slots now.)
    #[test]
    fn reference_macro_expands_at_reference_slot() {
        let mut macros = macro_set();
        macros
            .insert(&def(r#"(
                    name: "MyController",
                    kinds: [Reference],
                    body: ControllerOf(This),
                )"#))
            .unwrap();
        let reference: Reference = macros.read_str("MyController").unwrap();
        let Reference::Expanded(expanded) = reference else {
            panic!("expected Reference::Expanded(..), got {reference:?}");
        };
        assert_eq!(expanded.name, "MyController");
        assert_eq!(
            *expanded.value,
            Reference::ControllerOf(Arc::new(Reference::This)),
        );
    }

    /// A player verb at an `Action` slot reads NATIVELY — `PlayerAction`/`By`
    /// are gone, so `Sacrifice`/`Tap` are ordinary `Action` variants like any
    /// source-agent verb; the agent is spelled, never defaulted (Law 2).
    #[test]
    fn player_verb_reads_natively_at_action_slot() {
        let macros = macro_set();
        assert_eq!(
            macros.read_str::<Action>("Sacrifice(You, This)").unwrap(),
            Action::Sacrifice(Reference::You, Reference::This),
        );
        assert_eq!(
            macros.read_str::<Action>("Tap(This)").unwrap(),
            Action::Tap(Reference::This),
        );
    }

    /// A non-`You` agent reads natively too — no special-cased embed path.
    #[test]
    fn explicit_agent_reads_natively_at_action_slot() {
        let macros = macro_set();
        assert_eq!(
            macros.read_str::<Action>("ChangeLife(It, Up(3))").unwrap(),
            Action::ChangeLife(Reference::It, LifeOp::Up(Count::Literal(3))),
        );
    }

    /// A source-agent verb (`DealDamage`) reads natively at an `Action` slot;
    /// the `Selection` inside still embeds the bare reference.
    #[test]
    fn source_verb_reads_natively_at_action_slot() {
        let macros = macro_set();
        assert_eq!(
            macros
                .read_str::<Action>("DealDamage(This, 3, It)")
                .unwrap(),
            Action::deal_damage(Reference::It, Count::Literal(3)),
        );
    }

    /// An `Action` *macro* invoked at an `Action` slot routes through
    /// `Action`'s OWN `Expanded` variant (mirrors `OneShotEffect`/
    /// `Selection`'s own-macro reader — the `By`-embed fallthrough tier
    /// `PlayerAction::Expanded` used to inherit is gone; `Action` now
    /// remembers its own macros directly).
    #[test]
    fn action_macro_expands_at_action_slot() {
        let mut macros = macro_set();
        macros
            .insert(&def(r#"(
                    name: "GainTwo",
                    kinds: [Action],
                    body: ChangeLife(You, Up(2)),
                )"#))
            .unwrap();
        let action: Action = macros.read_str("GainTwo").unwrap();
        let Action::Expanded(expanded) = action else {
            panic!("expected Action::Expanded(..), got {action:?}");
        };
        assert_eq!(expanded.name, "GainTwo");
        assert_eq!(
            *expanded.value,
            Action::ChangeLife(Reference::You, LifeOp::Up(Count::Literal(2)))
        );
    }

    /// Same pin for the `TargetSpec` positions (the announce-list type).
    #[test]
    fn target_spec_positions_expand_macros() {
        let mut macros = macro_set();
        macros
            .insert(&def(r#"(
                    name: "TargetCreature",
                    kinds: [TargetSpec],
                    body: Target(Range(1, 1), Type(name:"Creature",permanent:true)),
                )"#))
            .unwrap();
        let spec: TargetSpec = macros.read_str("TargetCreature").unwrap();
        let TargetSpec::Expanded(expanded) = spec else {
            panic!("expected a remembered target spec, got {spec:?}");
        };
        assert_eq!(expanded.name, "TargetCreature");
        assert_eq!(
            *expanded.value,
            TargetSpec::Target(Quantity::one(), Predicate::creature(),)
        );
    }

    /// And for Reference positions.
    #[test]
    fn reference_positions_expand_macros() {
        let mut macros = macro_set();
        macros
            .insert(&def(r#"(
                    name: "MyController",
                    kinds: [Reference],
                    body: ControllerOf(This),
                )"#))
            .unwrap();
        let reference: Reference = macros.read_str("MyController").unwrap();
        let Reference::Expanded(expanded) = reference else {
            panic!("expected a remembered reference, got {reference:?}");
        };
        assert_eq!(expanded.name, "MyController");
        assert_eq!(
            *expanded.value,
            Reference::ControllerOf(Arc::new(Reference::This))
        );
    }

    /// `CostComponent` positions participate in macro expansion: a
    /// registered `CostComponent` macro is expanded in place of its name.
    #[test]
    fn cost_positions_expand_macros() {
        let mut macros = macro_set();
        macros
            .insert(&def(r#"(
                    name: "SacThis",
                    kinds: [CostComponent],
                    body: Do(Sacrifice(You, This)),
                )"#))
            .unwrap();
        let cost: deckmaste_semantics::CostComponent = macros.read_str("SacThis").unwrap();
        let deckmaste_semantics::CostComponent::Expanded(expanded) = &cost else {
            panic!("expected a remembered cost component, got {cost:?}");
        };
        assert_eq!(expanded.name, "SacThis");
        assert_eq!(
            *expanded.value,
            deckmaste_semantics::CostComponent::do_action(deckmaste_semantics::Action::Sacrifice(
                deckmaste_semantics::Reference::You,
                deckmaste_semantics::Reference::This,
            ))
        );

        let lowered = deckmaste_semantics::Cost(std::sync::Arc::from([cost.clone()])).lower();
        let [deckmaste_core::CostComponent::Act { dest: None, action }] = &*lowered.0 else {
            panic!("expected one runnable Act cost component, got {lowered:?}");
        };
        assert_eq!(
            **action,
            deckmaste_core::Action::Sacrifice(
                deckmaste_core::Reference::Reg(deckmaste_core::RefId(1)),
                deckmaste_core::Reference::Reg(deckmaste_core::RefId(0)),
            )
        );
    }

    /// A remembered invocation round-trips through the semantic types'
    /// `Serialize` impls: a nullary Ability
    /// macro serializes back to its bare name, a parameterized Predicate macro
    /// to the original call text — not the expansion.
    #[test]
    fn remembered_invocations_round_trip_as_invocations() {
        let mut macros = macro_set();
        macros
            .insert(&def(r#"(
                    name: "Flying",
                    kinds: [Ability],
                    body: Static(Cant(Attack(by: Ref(This)))),
                )"#))
            .unwrap();
        // A `TypeDef`-kind macro the parameterized `OfType` forwards its bare
        // argument into — the arg is validated and expanded as a real type.
        macros
            .insert(&def(r#"(
                    name: "Creature",
                    kinds: [TypeDef],
                    body: TypeDef(name: "Creature", permanent: true),
                )"#))
            .unwrap();
        macros
            .insert(&def(r#"(
                    name: "OfType",
                    kinds: [Predicate],
                    params: [TypeDef],
                    body: Type(Param(0)),
                )"#))
            .unwrap();

        let ability: Ability = macros.read_str("Flying").unwrap();
        assert_eq!(
            deckmaste_semantics::ron::options()
                .to_string(&ability)
                .unwrap(),
            "Flying"
        );

        // The parameterized Predicate macro round-trips to its call text — the
        // bare type name, not the expanded struct.
        let filter: Predicate = macros.read_str("OfType(Creature)").unwrap();
        assert_eq!(
            deckmaste_semantics::ron::options()
                .to_string(&filter)
                .unwrap(),
            "OfType(Creature)"
        );
    }

    /// The macro-aware reader expands a bare card-TYPE name and a bare builtin
    /// SUBTYPE name at a `Predicate` filter position to the resolved-def filter
    /// (`TypeRef`/`SubtypeRef`, keyed by name); an undeclared name has no macro
    /// and fails to parse — the load-time validation, for free.
    #[test]
    fn bare_type_and_subtype_names_expand_at_filter_positions() {
        let macros = builtin().macros;

        // A bare card type expands to the resolved `TypeRef` filter, equal by
        // name to the structural `Predicate::r#type` helper.
        let creature: Predicate = macros.read_str("Type(Creature)").unwrap();
        assert_eq!(creature, Predicate::r#type(Type::Creature));

        // A bare builtin subtype (`Zombie`, a `CreatureType` meta invocation)
        // expands to the resolved `SubtypeRef` filter, keyed on its name.
        let zombie: Predicate = macros.read_str("Subtype(Zombie)").unwrap();
        let Predicate::Characteristic(CharacteristicPredicate::Subtype(s)) = &zombie else {
            panic!("expected a Subtype ref, got {zombie:?}");
        };
        assert_eq!(s.name().as_str(), "Zombie");

        // An undeclared subtype has no macro — parse fails.
        let err = macros
            .read_str::<Predicate>("Subtype(SomeBogusName)")
            .unwrap_err();
        assert!(err.to_string().contains("SomeBogusName"));
    }

    /// `expand_all` over a real converted type: an `OneShotEffect` read through
    /// the card macro set carries the remembered `Expanded` node (here
    /// nested in a `Sequentially`, proving recursion through real grammar
    /// containers), and expanding strips it — the result serializes as the
    /// fully-expanded RON, not the invocation.
    #[test]
    fn expand_all_strips_remembered_invocations() {
        use deckmaste_semantics::Expand as _;

        let mut macros = macro_set();
        macros
            .insert(&def(r#"(
                    name: "GainTwo",
                    kinds: [OneShotEffect],
                    body: ChangeLife(You, Up(2)),
                )"#))
            .unwrap();
        let effect: OneShotEffect = macros.read_str("Sequentially([GainTwo])").unwrap();
        let OneShotEffect::Sequentially(steps) = &effect else {
            panic!("expected a sequence, got {effect:?}");
        };
        assert!(matches!(steps[0], OneShotEffect::Expanded(_)));

        let expanded = effect.expand_all();
        assert_eq!(
            expanded,
            OneShotEffect::Sequentially(
                vec![OneShotEffect::Act(Action::ChangeLife(
                    Reference::You,
                    LifeOp::Up(Count::Literal(2)),
                ))]
                .into()
            )
        );
        let written = deckmaste_semantics::ron::options()
            .to_string(&expanded)
            .unwrap();
        assert!(!written.contains("GainTwo"), "macro name leaked: {written}");
        // A `Count` literal writes bare — `2`, never `Literal(2)`.
        assert_eq!(written, "Sequentially([ChangeLife(You,Up(2))])");
    }

    /// The literal sugar applies to `Count` through the glue's registry:
    /// a bare numeral splices to `Literal`, identifier-led variants pass
    /// straight through.
    #[test]
    fn count_sugar_applies_to_core_count() {
        use deckmaste_semantics::Count as SemValue;

        let count: Count = macro_set().read_str("3").unwrap();
        assert_eq!(count, Count::Literal(3));
        let count: Count = macro_set().read_str("X").unwrap();
        assert_eq!(count, SemValue::X);
    }

    /// A `Count` macro expands at the *anchor* slot of a `Move`-to-library
    /// destination, read natively at an `Action` slot. This is the seam that
    /// lets a `CardsInLibrary(...)`-style macro name a library position — the
    /// `Count` stays macro-aware inside `Library(FromTop(...))`.
    #[test]
    fn count_macro_expands_at_library_anchor_position() {
        use deckmaste_semantics::Anchor;
        use deckmaste_semantics::Destination;
        let mut macros = macro_set();
        macros
            .insert(&def(r#"(
                    name: "DeckSize",
                    kinds: [Count],
                    body: CountOf(Objects(InZone(Library))),
                )"#))
            .unwrap();
        let action: Action = macros
            .read_str("Move(This, Library(FromTop(DeckSize)))")
            .unwrap();
        let Action::Move(sel, Destination::Library(Anchor::FromTop(pos)), _, _) = action else {
            panic!("expected Move(.., Library(FromTop(..))), got {action:?}");
        };
        assert_eq!(sel, Reference::This);
        let Count::Expanded(expanded) = pos else {
            panic!("expected a remembered count at the anchor, got {pos:?}");
        };
        assert_eq!(expanded.name, "DeckSize");
    }

    /// `Color` is a registered param type, so a definition may declare it.
    #[test]
    fn color_is_a_registered_param_type() {
        let mut macros = macro_set();
        macros
            .insert(&def(r#"(
                    name: "TapsForColor",
                    kinds: [Subtype],
                    params: [Color],
                    body: Subtype(name: Param(0), types: [Land]),
                )"#))
            .expect("a Color param type should be registered");
    }

    /// [typed-holes delta 1] Every macroable kind is a registered param type,
    /// so a macro may declare `OneShotEffect`/`Selection`/`Modification`/…
    /// slots — and an argument that isn't that grammar is rejected at the
    /// call site, before the body expands. Retyping the `Any` slots to real
    /// types is the point.
    #[test]
    fn every_macroable_kind_is_a_typed_param() {
        let param_types = param_types();
        for name in [
            "OneShotEffect",
            "Ability",
            "Action",
            "Condition",
            "CostComponent",
            "Count",
            "Destination",
            "EventFilter",
            "Predicate",
            "KeywordAbility",
            "ManaRider",
            "Modification",
            "Quantity",
            "Reference",
            "Replacement",
            "Selection",
            "StaticEffect",
            "TargetSpec",
            "Subtype",
            "Zone",
            "AsThough",
            // Shaped exceptions.
            "Color",
            "Cost",
            "Abilities",
        ] {
            assert!(
                param_types.contains(name),
                "`{name}` should be a registered param type"
            );
        }
    }

    /// An `OneShotEffect`-typed slot rejects a non-OneShotEffect at the call
    /// site — the delta-1 headline for a kind that had no validator before.
    #[test]
    fn effect_param_rejects_a_non_effect() {
        let mut macros = macro_set();
        macros
            .insert(&def(r#"(
                    name: "DoThen",
                    kinds: [Replacement],
                    params: [OneShotEffect],
                    body: Also(would: ZoneChange(what: Any, to: Battlefield), also: Param(0)),
                )"#))
            .unwrap();
        // `Purple` is neither a OneShotEffect variant nor a OneShotEffect
        // macro.
        let error = macros
            .read_str::<deckmaste_semantics::Replacement>("DoThen(Purple)")
            .unwrap_err()
            .to_string();
        assert!(
            error.contains("DoThen") && error.contains("OneShotEffect"),
            "unexpected error: {error}"
        );
        // A real OneShotEffect argument is accepted (remembered as `Expanded`).
        let ok: deckmaste_semantics::Replacement = macros.read_str("DoThen(Tap(This))").unwrap();
        let deckmaste_semantics::Replacement::Expanded(exp) = ok else {
            panic!("expected a remembered replacement, got {ok:?}");
        };
        assert!(matches!(
            *exp.value,
            deckmaste_semantics::Replacement::Also { .. }
        ));
    }

    /// `Reference` is a registered param type, so a macro may take one — the
    /// hook `SharesColorWith(ref)` / `SharesColor(a, b)` rely on.
    #[test]
    fn reference_is_a_registered_param_type() {
        let mut macros = macro_set();
        macros
            .insert(&def(r#"(
                    name: "IsThat",
                    kinds: [Predicate],
                    params: [Reference],
                    body: Ref(Param(0)),
                )"#))
            .expect("a Reference param type should be registered");
    }

    /// `SupportsMacros` guarantees `DeserializeOwned`: a function generic over
    /// the trait can deserialize a value of that type through the macro layer.
    /// This compiles only because the supertrait bond holds — the param-type
    /// validators (`read_str::<T>`) rely on exactly that guarantee.
    #[test]
    fn supports_macros_guarantees_deserialize_owned() {
        fn read_one<T: deckmaste_semantics::SupportsMacros>(
            macros: &MacroSet,
            src: &str,
        ) -> Result<T, ron::error::SpannedError> {
            macros.read_str::<T>(src)
        }
        let macros = macro_set();
        let parsed: Reference = read_one(&macros, "This").expect("This is a Reference");
        assert_eq!(parsed, Reference::This);
    }

    /// A `Color` param rejects a non-color at the call site, before the body
    /// ever expands — the headline behavior. (Body simplified; the real
    /// `confers` ability is exercised by the builtin validation test.)
    #[test]
    fn basic_land_type_rejects_non_color() {
        let mut macros = macro_set();
        macros
            .insert(&def(r#"(
                    name: "BasicLandType",
                    kinds: [Subtype],
                    params: [String, Color],
                    body: Subtype(name: Param(0), types: [Land]),
                )"#))
            .unwrap();
        let error = macros
            .read_str::<Subtype>(r#"BasicLandType("Plains", Purple)"#)
            .unwrap_err();
        let msg = error.to_string();
        assert!(
            msg.contains("BasicLandType") && msg.contains("Color"),
            "unexpected error: {msg}"
        );
    }

    /// A real color is accepted and the name binds.
    #[test]
    fn basic_land_type_accepts_color() {
        let mut macros = macro_set();
        macros
            .insert(&def(r#"(
                    name: "BasicLandType",
                    kinds: [Subtype],
                    params: [String, Color],
                    body: Subtype(name: Param(0), types: [Land]),
                )"#))
            .unwrap();
        let subtype: deckmaste_semantics::Subtype = macros
            .read_str(r#"BasicLandType("Plains", White)"#)
            .unwrap();
        assert_eq!(subtype.name, "Plains");
    }

    /// A quoted-string name at a `[String]` subtype macro still expands — the
    /// seven unchanged subtype macros keep working.
    #[test]
    fn creature_type_still_expands() {
        let mut macros = macro_set();
        macros
            .insert(&def(r#"(
                    name: "CreatureType",
                    kinds: [Subtype],
                    params: [String],
                    body: Subtype(name: Param(0), types: [Creature]),
                )"#))
            .unwrap();
        let subtype: deckmaste_semantics::Subtype =
            macros.read_str(r#"CreatureType("Bear")"#).unwrap();
        assert_eq!(subtype.name, "Bear");
    }

    /// The real subtype-meta shape: `template` defaults to `name`, so the
    /// majority of subtype files omit it.
    #[test]
    fn subtype_meta_template_defaults_to_name() {
        let mut macros = macro_set();
        macros
            .insert(&def(r#"(
                    name: "CreatureType",
                    kinds: [Macro],
                    params: { "name": String, "template": Default(String, Param(name)) },
                    body: (
                        name: Param(name),
                        kinds: [Subtype],
                        body: Subtype(name: Param(template), types: [Creature]),
                    ),
                )"#))
            .unwrap();
        let produced: MacroDef = macros.read_str(r#"CreatureType(name: "Zombie")"#).unwrap();
        macros.insert(&produced).unwrap();
        let subtype: deckmaste_semantics::Subtype = macros.read_str("Zombie").unwrap();
        assert_eq!(subtype.name, "Zombie");
    }

    /// A `Condition` macro at a strategy's `when:` position expands through the
    /// macro reader even though it sits deep inside plain-serde
    /// `Strategy`/`Rule` — sensing positions are macro-aware for free
    /// (core's `Condition` does the fall-through). Lets strategy-guide
    /// vocabulary (`Always`, …) be macros.
    ///
    /// The real `Strategy`/`Rule` types now live in `deckmaste_engine` (moved
    /// out of `deckmaste_core` by `plugin-repoint` — strategy RON is engine
    /// config, not card content), a crate this one does not depend on; these
    /// local stand-ins mirror their shape (a `when: Condition` nested under an
    /// outer struct) closely enough to prove the same point without that
    /// dependency.
    #[test]
    fn strategy_when_position_expands_a_condition_macro() {
        use deckmaste_semantics::Condition;

        #[derive(serde::Deserialize)]
        struct Rule {
            when: Condition,
        }
        #[derive(serde::Deserialize)]
        struct Strategy {
            rules: Vec<Rule>,
        }

        let mut macros = macro_set();
        macros
            .insert(&def(
                r#"(name: "Always", kinds: [Condition], body: And([]))"#,
            ))
            .unwrap();
        let s: Strategy = macros
            .read_str(r#"(name: "M", rules: [(when: Always, prefer: Pass)])"#)
            .unwrap();
        let Condition::Expanded(exp) = &s.rules[0].when else {
            panic!("expected expanded condition, got {:?}", s.rules[0].when);
        };
        assert_eq!(exp.name, "Always");
    }
}
