//! The canon cards (real cards — pipeline output plus marked hand-finished
//! entries, see docs/card-data.md) parsed through the macro-aware reader, on
//! top of the builtin prelude they depend on. Run by plain `cargo test`;
//! wizards is the explicit `cargo xtask validate plugins/wizards`.

use std::path::Path;
use std::path::PathBuf;
use std::sync::Arc;

use deckmaste_card::Card;
use deckmaste_core::Ability;
use deckmaste_core::Action;
use deckmaste_core::Count;
use deckmaste_core::Instruction;
use deckmaste_core::Reference;
use deckmaste_core::StatValue;
use deckmaste_core::Subtype;
use deckmaste_core::TargetSpec;
use deckmaste_core::Type;
use deckmaste_lowering::Lower;
use deckmaste_plugin::plugin::Plugin;
use macro_ron::Expand;

fn canon_path() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../../plugins/canon")
}

fn canon() -> Plugin {
    // canon sits on top of builtin: its cards reference builtin's macros
    // (CreatureType, AnyTarget) and subtype declarations, so it needs the
    // sibling `builtin/` prelude — the same convention validate_plugin uses.
    Plugin::load_with_sibling_prelude(canon_path()).unwrap()
}

#[test]
fn canon_cards_are_valid() {
    let validation = deckmaste_plugin::validate::validate_plugin(&canon_path()).unwrap();
    for failure in &validation.failures {
        eprintln!("{}: {}", failure.path.display(), failure.error);
    }
    for (path, msg) in &validation.lint_failures {
        eprintln!("{}: lint: {msg}", path.display());
    }
    assert!(validation.failures.is_empty());
    assert!(validation.lint_failures.is_empty());
    // The canon slice: 25 cards at the time of writing, growing per
    // docs/card-data.md.
    assert!(
        validation.valid >= 25,
        "only {} items checked",
        validation.valid
    );
}

/// The `CreatureType` macro path through real data: `subtypes: [Bear]`
/// resolves the declaration, which invokes `CreatureType("Bear")`.
#[test]
fn grizzly_bears_expand_the_creature_type_macro() {
    let plugin = canon();
    let card = plugin.card("Grizzly Bears").unwrap().core;
    let Card::Normal(face) = card else {
        panic!("Grizzly Bears should be single-faced");
    };
    // The `Creature` cardtype macro now expands to the full conferring `TypeDef`
    // — the combat-capability confers ([CR#508.1a,509.1a]) ride the type. Read
    // the SAME expansion the card's `types: [Creature]` produced, not the
    // empty-confer `Type::Creature.def()`.
    let creature_type: deckmaste_core::TypeDef = plugin
        .macros
        .read_str::<deckmaste_semantics::TypeDef>("Creature")
        .unwrap()
        .expand_all()
        .lower();
    assert_eq!(face.types, vec![creature_type]);
    assert_eq!(
        face.subtypes,
        vec![Subtype {
            name: "Bear".into(),
            types: vec![Type::Creature, Type::Kindred].into(),
            confers: vec![].into(),
        }]
    );
    assert_eq!(face.power, Some(StatValue::Number(2)));
    assert_eq!(face.toughness, Some(StatValue::Number(2)));
}

/// Target-position interception through real data: the bare `AnyTarget`
/// invocation expands at the ability's `TargetSpec` announce slot.
#[test]
fn lightning_bolt_expands_target_macros() {
    let plugin = canon();
    let Card::Normal(face) = plugin.card("Lightning Bolt").unwrap().core else {
        panic!("Lightning Bolt should be single-faced");
    };
    // The card's `targets` field is macro-aware: loading it expands the bare
    // `AnyTarget` exactly as reading the macro directly does — interior filter
    // expansions (Battle/Creature/…) and all. Comparing against a fresh read
    // keeps this robust to macro refactors instead of pinning the nested
    // expansion by hand. `lower` erases invocation provenance (spec §12), so
    // the loaded card carries `AnyTarget`'s BODY, not a remembered
    // invocation — `expand_all` strips the fresh read down to the same
    // shape.
    let any_target: TargetSpec = plugin
        .macros
        .read_str::<deckmaste_semantics::TargetSpec>("AnyTarget")
        .unwrap()
        .expand_all()
        .lower();
    let [Ability::Spell(spell)] = face.abilities.as_slice() else {
        panic!("expected one spell ability")
    };
    assert_eq!(spell.targets.as_ref(), std::slice::from_ref(&any_target));
    assert_eq!(
        spell.effect.body.as_ref(),
        [
            Instruction::Let(deckmaste_core::Let {
                dest: deckmaste_core::DefId(4),
                expr: deckmaste_core::Expr::Number(Count::Literal(3)),
            }),
            Instruction::act(Action::DealDamage(
                Reference::Reg(deckmaste_core::RefId(0)),
                Count::Reg(deckmaste_core::RefId(4)),
                Reference::Reg(deckmaste_core::RefId(2)),
            )),
        ]
    );
}

/// Do or Die's authored pile labels are a surface convenience only: lowering
/// allocates two pile registers, makes the choice read those registers, and
/// makes the nested `Each` iterate the chosen-pile register through the
/// PILE-domain selection ([CR#700.3a..700.3b] — the pile is not an Entity
/// group). Re-spelled from the Entity-group register read this stage splits
/// off; the registers and the shape are unchanged.
#[test]
fn do_or_die_lowers_pile_labels_to_registers() {
    let plugin = canon();
    let Card::Normal(face) = plugin.card("Do or Die").unwrap().core else {
        panic!("Do or Die should be single-faced");
    };
    let Ability::Spell(ref spell) = face.abilities[0] else {
        panic!("expected a spell ability");
    };
    let [Instruction::SeparatePiles(separate)] = spell.effect.body.as_ref() else {
        panic!("expected SeparatePiles, got {:?}", spell.effect.body);
    };
    assert_eq!(
        separate.dests.as_ref(),
        [deckmaste_core::DefId(4), deckmaste_core::DefId(5)]
    );
    let Some(then) = &separate.then else {
        panic!("the pile separation should carry its choice");
    };
    let Instruction::ChoosePile(choice) = then.as_ref() else {
        panic!("expected ChoosePile, got {then:?}");
    };
    assert_eq!(
        choice.from.as_ref(),
        [deckmaste_core::RefId(4), deckmaste_core::RefId(5)]
    );
    assert_eq!(choice.dest, deckmaste_core::DefId(6));
    let Instruction::Each(each) = choice.then.as_ref() else {
        panic!("expected Each, got {:?}", choice.then);
    };
    assert_eq!(
        each.over,
        deckmaste_core::Selection::Pile(deckmaste_core::RefId(6))
    );
    assert_eq!(
        each.over.collection_domain(),
        deckmaste_core::CollectionDomain::Pile
    );
}

/// The `Domain` count macro expands at a `Count` position through real data:
/// Tribal Flames' damage amount is the BODY of a `Domain` invocation — the
/// distinct-union count of the BASIC-land-type axis ([CR#205.3i]). `lower`
/// erases invocation provenance (spec §12), so the loaded card no longer
/// carries a remembered `Count::Expanded(Domain, …)` wrapper. Predicate
/// regions capture their enclosing ability ABI, so the witness checks the
/// defining `CountDistinct(BasicLandTypes, …)` shape rather than comparing it
/// with the same macro lowered outside that ability.
#[test]
fn tribal_flames_expands_the_domain_count() {
    let plugin = canon();
    let Card::Normal(face) = plugin.card("Tribal Flames").unwrap().core else {
        panic!("Tribal Flames should be single-faced");
    };
    let Ability::Spell(ref spell) = face.abilities[0] else {
        panic!("expected a spell ability");
    };
    let [
        Instruction::Let(deckmaste_core::Let {
            dest,
            expr: deckmaste_core::Expr::Number(count),
        }),
        Instruction::Act {
            action: Action::DealDamage(_, Count::Reg(amount), _),
            ..
        },
    ] = spell.effect.body.as_ref()
    else {
        panic!("expected DealDamage, got {:?}", spell.effect.body);
    };
    assert!(
        matches!(
            count,
            Count::CountDistinct(deckmaste_core::Characteristic::BasicLandTypes, _),
        ),
        "Tribal Flames' damage is Domain's body: {count:?}"
    );
    assert_eq!(
        *amount,
        (*dest).into(),
        "damage reads the pinned Domain count"
    );
}

/// `lower` erases invocation provenance (spec §12): `template:` from a macro
/// def used to ride the expansion all the way through the real loader
/// (`TargetSpec::Expanded(exp).template`); this task erases that wrapper, so
/// the loaded target position is `AnyTarget`'s BODY — the template no longer
/// lives on the loaded card. It is still on the macro DEFINITION itself
/// (`AnyTarget.ron` carries `template: "any target"`), which is where prose
/// recovery has to look now instead of the compiled card.
#[test]
fn any_target_body_replaces_its_expansion_on_the_loaded_card() {
    let plugin = canon();
    let Card::Normal(face) = plugin.card("Lightning Bolt").unwrap().core else {
        panic!("Lightning Bolt should be single-faced");
    };
    let Ability::Spell(ref spell) = face.abilities[0] else {
        panic!("expected a spell ability");
    };
    let any_target: deckmaste_semantics::TargetSpec = plugin.macros.read_str("AnyTarget").unwrap();
    let deckmaste_semantics::TargetSpec::Expanded(ref exp) = any_target else {
        panic!(
            "expected AnyTarget's own macro-def read to still be an expansion, got {any_target:?}"
        );
    };
    assert_eq!(
        exp.template.as_deref(),
        Some("any target"),
        "the macro DEFINITION still carries its template"
    );
    assert_eq!(
        spell.targets[0],
        any_target.expand_all().lower(),
        "but the loaded card carries AnyTarget's body, not its expansion wrapper"
    );
}

/// Mana Leak (hand-written canon) exercises the collapsed `May(Pay(cost))`
/// `MustPay` shape over the full `Cost` algebra: "counter target spell unless
/// its controller pays {3}" ([CR#118.12a]). The body reads to a `Targeted`
/// wrapper over `May { who: ControllerOf(Target(0)), effect: Pay({3}),
/// if_not: Counter }`.
#[test]
fn mana_leak_reads_to_a_must_pay_punisher() {
    use deckmaste_core::Cost;
    use deckmaste_core::CostComponent;
    use deckmaste_core::ManaCost;
    use deckmaste_core::ManaSymbol;
    use deckmaste_core::SimpleManaSymbol;

    let plugin = canon();
    let Card::Normal(face) = plugin.card("Mana Leak").unwrap().core else {
        panic!("Mana Leak should be single-faced");
    };
    let Ability::Spell(ref spell) = face.abilities[0] else {
        panic!("expected a spell ability");
    };
    let [Instruction::May(m)] = spell.effect.body.as_ref() else {
        panic!("expected May, got {:?}", spell.effect.body);
    };
    assert_eq!(
        m.who,
        Reference::ControllerOf(Arc::new(Reference::Reg(deckmaste_core::RefId(2)))),
        "the payer is the targeted spell's controller"
    );
    let Instruction::Act {
        action: Action::Pay(ref cost),
        ..
    } = *m.effect
    else {
        panic!("expected Act(Pay(cost)), got {:?}", m.effect);
    };
    assert_eq!(
        *cost,
        Cost(
            vec![CostComponent::Mana(ManaCost::from(
                Arc::<[ManaSymbol]>::from(vec![ManaSymbol::Simple(SimpleManaSymbol::Generic(3)),])
            ))]
            .into()
        ),
        "the toll is the full {{3}} Cost"
    );
    assert!(
        m.if_did.is_none(),
        "no positive branch — this is the punisher shape"
    );
    let Some(ref if_not) = m.if_not else {
        panic!("expected if_not, got None");
    };
    assert_eq!(
        **if_not,
        Instruction::act(Action::Counter(Reference::Reg(deckmaste_core::RefId(2)))),
        "unpaid → counter the spell"
    );
}

/// `lower` erases invocation provenance (spec §12): Brainstorm's
/// `Choose(Exactly(2), …)` used to re-serialize with `Exactly(2)` intact (the
/// no-card-churn guarantee, pre-erasure); now the loaded card carries
/// `Exactly(2)`'s BODY, so the plain-serde core RON shows the explicit
/// `Range(Literal(2), Literal(2))` primitive instead — the invocation spelling no longer lives in the
/// compiled card at all (it is still on the `Exactly` macro definition).
#[test]
fn brainstorm_exactly_two_round_trips() {
    let card = canon().card("Brainstorm").unwrap().core;
    let written = deckmaste_core::ron::options().to_string(&card).unwrap();
    assert!(
        written.contains("Range(Literal(2),Literal(2))")
            || written.contains("Range(Literal(2), Literal(2))"),
        "Exactly(2)'s body should round-trip as an explicit core Range, got: {written}"
    );
    assert!(
        !written.contains("Exactly("),
        "the Exactly invocation spelling should not survive `lower`, got: {written}"
    );
}

#[test]
fn fate_transfer_cost_is_hybrid_blue_black() {
    use deckmaste_core::Color;
    use deckmaste_core::ManaCost;
    use deckmaste_core::ManaSymbol;
    use deckmaste_core::SimpleManaSymbol;

    let plugin = canon();
    let Card::Normal(face) = plugin.card("Fate Transfer").unwrap().core else {
        panic!("Fate Transfer should be single-faced");
    };
    assert_eq!(
        face.mana_cost,
        ManaCost::from(Arc::<[ManaSymbol]>::from(vec![
            ManaSymbol::Simple(SimpleManaSymbol::Generic(1)),
            ManaSymbol::Hybrid(SimpleManaSymbol::from(Color::Blue), Color::Black),
        ]))
    );
}

#[test]
fn pounce_is_instant_type() {
    let plugin = canon();
    let Card::Normal(face) = plugin.card("Pounce").unwrap().core else {
        panic!("Pounce should be single-faced");
    };
    // The plugin-expanded Instant carries its May(Cast(InstantSpeed)) confer
    // (Task 6, casting-window), so compare by the canonical type NAME rather
    // than the confer-less `Type::Instant.def()` fixture.
    assert_eq!(face.types.len(), 1);
    assert_eq!(face.types[0].name, Type::Instant.name());
}

#[test]
fn arc_lightning_targets_any_target() {
    let plugin = canon();
    let Card::Normal(face) = plugin.card("Arc Lightning").unwrap().core else {
        panic!("Arc Lightning should be single-faced");
    };
    let Ability::Spell(ref spell) = face.abilities[0] else {
        panic!("expected a spell ability");
    };
    let TargetSpec::Target(count, filter) = &spell.targets[0] else {
        panic!("expected Target variant, got {:?}", spell.targets[0]);
    };
    // Between(1, 3): one, two, or three targets — never zero (the AnyNumber bug).
    assert_eq!(
        count.bounds(),
        (Some(&Count::Literal(1)), Some(&Count::Literal(3))),
        "Arc Lightning must target 1–3, not any number (which permits 0)"
    );
    // `lower` erases invocation provenance (spec §12): the loaded filter is
    // `AnyTarget`'s BODY, not a remembered `Predicate::Expanded(AnyTarget,
    // …)` — comparing against a fresh, expanded read of the macro keeps this
    // robust to macro refactors instead of pinning the shape by hand.
    let any_target: deckmaste_core::Predicate = plugin
        .macros
        .read_str::<deckmaste_semantics::Predicate>("AnyTarget")
        .unwrap()
        .expand_all()
        .lower();
    assert_eq!(
        filter.body, any_target,
        "Arc Lightning's target should be AnyTarget's body"
    );
}
