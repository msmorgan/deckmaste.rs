//! The canon cards (real cards — pipeline output plus marked hand-finished
//! entries, see docs/card-data.md) parsed through the macro-aware reader, on
//! top of the builtin prelude they depend on. Run by plain `cargo test`;
//! wizards is the explicit `cargo xtask validate plugins/wizards`.

use std::path::Path;
use std::path::PathBuf;

use deckmaste_cards::plugin::Plugin;
use deckmaste_core::Ability;
use deckmaste_core::Action;
use deckmaste_core::Card;
use deckmaste_core::Count;
use deckmaste_core::Effect;
use deckmaste_core::Reference;
use deckmaste_core::SpellAbility;
use deckmaste_core::StatValue;
use deckmaste_core::Subtype;
use deckmaste_core::TargetSpec;
use deckmaste_core::Type;

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
    let validation = deckmaste_cards::validate::validate_plugin(&canon_path()).unwrap();
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
    let card = canon().card("Grizzly Bears").unwrap();
    let Card::Normal(face) = card else {
        panic!("Grizzly Bears should be single-faced");
    };
    assert_eq!(face.types, vec![Type::Creature]);
    assert_eq!(
        face.subtypes,
        vec![Subtype {
            name: "Bear".into(),
            types: vec![Type::Creature, Type::Kindred],
            confers: vec![],
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
    let Card::Normal(face) = plugin.card("Lightning Bolt").unwrap() else {
        panic!("Lightning Bolt should be single-faced");
    };
    // The card's `targets` field is macro-aware: loading it expands the bare
    // `AnyTarget` exactly as reading the macro directly does — interior filter
    // expansions (Battle/Creature/…) and all. Comparing against a fresh read
    // keeps this robust to macro refactors instead of pinning the nested,
    // provenance-bearing expansion by hand.
    let any_target: TargetSpec = plugin.macros.read_str("AnyTarget").unwrap();
    assert_eq!(
        face.abilities,
        vec![Ability::Spell(SpellAbility {
            effect: Effect::Targeted(deckmaste_core::Targeted::new(
                vec![any_target],
                Effect::Act(Action::deal_damage(Reference::Target(0), Count::Literal(3),)),
            )),
        })]
    );
}

/// The `Domain` count macro expands at a `Count` position through real data:
/// Tribal Flames' damage amount is a remembered `Domain` invocation wrapping
/// the distinct-union count of the land-subtype axis.
#[test]
fn tribal_flames_expands_the_domain_count() {
    let plugin = canon();
    let Card::Normal(face) = plugin.card("Tribal Flames").unwrap() else {
        panic!("Tribal Flames should be single-faced");
    };
    let Ability::Spell(ref spell) = face.abilities[0] else {
        panic!("expected a spell ability");
    };
    let Effect::Targeted(ref te) = spell.effect else {
        panic!("expected a Targeted wrapper, got {:?}", spell.effect);
    };
    let Effect::Act(Action::DealDamage(_, count, _)) = te.effect.as_ref() else {
        panic!("expected DealDamage, got {:?}", te.effect);
    };
    let Count::Expanded(exp) = count else {
        panic!("expected a remembered Domain count, got {count:?}");
    };
    assert_eq!(exp.name, "Domain");
    assert!(matches!(
        exp.value.as_ref(),
        Count::CountDistinct(deckmaste_core::Characteristic::Subtypes, _),
    ));
}

/// End-to-end proof that `template:` from a macro def rides the expansion all
/// the way through the real loader. `AnyTarget.ron` carries `template: "any
/// target"`; after loading, `TargetSpec::Expanded(exp)` must have it.
#[test]
fn any_target_expansion_carries_its_template() {
    let plugin = canon();
    let Card::Normal(face) = plugin.card("Lightning Bolt").unwrap() else {
        panic!("Lightning Bolt should be single-faced");
    };
    let Ability::Spell(ref spell) = face.abilities[0] else {
        panic!("expected a spell ability");
    };
    let Effect::Targeted(ref te) = spell.effect else {
        panic!("expected a Targeted wrapper, got {:?}", spell.effect);
    };
    match &te.targets[0] {
        TargetSpec::Expanded(exp) => assert_eq!(
            exp.template.as_deref(),
            Some("any target"),
            "AnyTarget's template should ride the expansion"
        ),
        other => panic!("expected AnyTarget expansion, got {other:?}"),
    }
}

/// Mana Leak (hand-written canon) exercises the resolution-time `MustPay`
/// punisher over the full `Cost` algebra: "counter target spell unless its
/// controller pays {3}" ([CR#118.12a]). The body reads to a `Targeted` wrapper
/// over `MustPay { actor: ControllerOf(Target(0)), cost: {3}, or_else: Counter
/// }`.
#[test]
fn mana_leak_reads_to_a_must_pay_punisher() {
    use deckmaste_core::Cost;
    use deckmaste_core::CostComponent;
    use deckmaste_core::ManaCost;
    use deckmaste_core::ManaSymbol;
    use deckmaste_core::SimpleManaSymbol;

    let plugin = canon();
    let Card::Normal(face) = plugin.card("Mana Leak").unwrap() else {
        panic!("Mana Leak should be single-faced");
    };
    let Ability::Spell(ref spell) = face.abilities[0] else {
        panic!("expected a spell ability");
    };
    let Effect::Targeted(ref te) = spell.effect else {
        panic!("expected a Targeted wrapper, got {:?}", spell.effect);
    };
    let Effect::MustPay(ref m) = *te.effect else {
        panic!("expected MustPay, got {:?}", te.effect);
    };
    assert_eq!(
        m.actor,
        Reference::ControllerOf(Box::new(Reference::Target(0))),
        "the payer is the targeted spell's controller"
    );
    assert_eq!(
        m.cost,
        Cost(vec![CostComponent::Mana(ManaCost::from(vec![
            ManaSymbol::Simple(SimpleManaSymbol::Generic(3)),
        ]))]),
        "the toll is the full {{3}} Cost"
    );
    assert_eq!(
        *m.or_else,
        Effect::Act(Action::Counter(Reference::Target(0))),
        "unpaid → counter the spell"
    );
}

/// An existing card's named `Quantity` survives the collapse byte-for-byte:
/// Brainstorm's `Choose(Exactly(2), …)` re-serializes with `Exactly(2)`
/// intact, not the bare `Range(2, 2)` primitive — the no-card-churn guarantee.
#[test]
fn brainstorm_exactly_two_round_trips() {
    let card = canon().card("Brainstorm").unwrap();
    let written = deckmaste_core::ron::options().to_string(&card).unwrap();
    assert!(
        written.contains("Exactly(2)"),
        "Exactly(2) should round-trip intact, got: {written}"
    );
    assert!(
        !written.contains("Range("),
        "the Range primitive should not surface in card RON, got: {written}"
    );
}
