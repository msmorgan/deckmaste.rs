//! The reader against `plugins_v2/testing`: a plugin with one card per shape
//! the fixture needs, a token, and one file in each of the three rules tables.
//!
//! Card text is real Magic text, taken from `data/mtgjson/AtomicCards.json`.

use std::path::Path;
use std::path::PathBuf;

use deckmaste_semantics_v2::abilities::Ability;
use deckmaste_semantics_v2::abilities::Instruction;
use deckmaste_semantics_v2::card::Card;
use deckmaste_semantics_v2::phrase::Amount;
use deckmaste_semantics_v2::phrase::DetPhrase;
use deckmaste_semantics_v2::phrase::NounPhrase;
use deckmaste_semantics_v2::phrase::Predicate;
use deckmaste_semantics_v2::reader::Plugin;
use deckmaste_semantics_v2::words::CardType;
use deckmaste_semantics_v2::words::Color;
use deckmaste_semantics_v2::words::Subtype;

fn testing_plugin() -> Plugin {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../plugins_v2/testing");
    Plugin::load(root).expect("the testing plugin loads")
}

fn face(card: &Card) -> &deckmaste_semantics_v2::card::CardFace {
    match card {
        Card::SingleFaced { face } => face,
        other => panic!("expected a single-faced card, got {other:?}"),
    }
}

/// The two macros register, and the nested one names the other in its body.
#[test]
fn the_plugins_macros_register_under_their_kinds() {
    let plugin = testing_plugin();
    assert!(plugin.macros.get("NounPhrase", "AnyTarget").is_some());
    let nested = plugin
        .macros
        .get("Instruction", "DealsDamageToAnyTarget")
        .expect("the damage macro registers at Instruction");
    assert!(
        nested.body().contains("AnyTarget"),
        "the nested macro's body invokes the other macro"
    );
}

/// "Lightning Bolt deals 3 damage to any target." — the card's one ability
/// reads through two levels of macro: the instruction macro, and the noun
/// phrase its body invokes.
#[test]
fn a_card_reads_through_nested_macro_expansion() {
    let plugin = testing_plugin();
    let card = plugin
        .cards
        .get("Lightning Bolt")
        .expect("the card file is indexed by its stem");
    let face = face(card);
    assert_eq!(face.characteristics.name.as_deref(), Some("Lightning Bolt"));
    assert_eq!(face.characteristics.types, vec![CardType::Instant]);

    let [Ability::Spell { instruction, .. }] = face.characteristics.text.as_slice() else {
        panic!(
            "expected one spell ability, got {:?}",
            face.characteristics.text
        );
    };
    let Instruction::DealDamage {
        source,
        amount,
        recipient,
    } = &**instruction
    else {
        panic!("the instruction macro expanded to {instruction:?}");
    };
    assert_eq!(*source, NounPhrase::This);
    assert_eq!(*amount, Amount::Lit { value: 3 });

    // The recipient came from the nested `AnyTarget` expansion: a targeted
    // description whose predicate is a disjunction over kinds [CR#115.4].
    let NounPhrase::Described {
        determiner,
        predicate,
    } = recipient
    else {
        panic!("the nested macro expanded to {recipient:?}");
    };
    assert!(matches!(**determiner, DetPhrase::Target { .. }));
    let Predicate::Or { disjuncts } = &**predicate else {
        panic!("expected a disjunction, got {predicate:?}");
    };
    assert_eq!(disjuncts.len(), 4);
    assert!(disjuncts.contains(&Predicate::AnyPlayer));
    assert!(disjuncts.contains(&Predicate::HasType {
        r#type: CardType::Creature
    }));
}

/// A vanilla creature writes only the characteristics it has; every absent one
/// takes the structure's own default, as it does in Lean.
#[test]
fn a_vanilla_card_leaves_its_absent_characteristics_absent() {
    let plugin = testing_plugin();
    let face = face(plugin.cards.get("Grizzly Bears").expect("indexed"));
    let characteristics = &face.characteristics;
    assert_eq!(characteristics.power, Some(Amount::Lit { value: 2 }));
    assert_eq!(characteristics.toughness, Some(Amount::Lit { value: 2 }));
    assert_eq!(
        characteristics.subtypes,
        vec![Subtype::Of {
            host: CardType::Creature,
            label: "Bear".to_string(),
        }]
    );
    assert!(characteristics.text.is_empty());
    assert_eq!(characteristics.loyalty, None);
    assert_eq!(characteristics.defense, None);
    assert!(face.choices.is_empty());
}

/// A token is the characteristics an effect writes, qualities included
/// [CR#111.3].
#[test]
fn a_token_reads_as_the_bundle_an_effect_writes() {
    let plugin = testing_plugin();
    let token = plugin.tokens.get("Soldier").expect("indexed");
    assert_eq!(token.characteristics.colors, vec![Color::White]);
    assert_eq!(token.characteristics.types, vec![CardType::Creature]);
    assert_eq!(token.characteristics.power, Some(Amount::Lit { value: 1 }));
    assert!(token.qualities.is_empty());
}

/// All three rules tables load, one row each.
#[test]
fn the_three_rules_tables_load() {
    let plugin = testing_plugin();
    assert_eq!(plugin.rules.sba.len(), 1);
    assert_eq!(plugin.rules.conferral.len(), 1);
    assert_eq!(plugin.rules.damage_result.len(), 1);

    let sba = &plugin.rules.sba[0];
    assert_eq!(
        sba.scope,
        Predicate::HasType {
            r#type: CardType::Creature
        }
    );
    assert!(matches!(sba.then, Instruction::Move { .. }));

    let conferral = &plugin.rules.conferral[0];
    assert!(matches!(conferral.confer, Ability::Static { .. }));

    let damage = &plugin.rules.damage_result[0];
    assert_eq!(
        damage.recipient,
        Predicate::HasType {
            r#type: CardType::Planeswalker
        }
    );
}

/// A plugin loaded over a prelude sees the prelude's macros. The builtin
/// declarations are the prelude every other plugin reads through (§15).
#[test]
fn a_prelude_carries_its_macros_into_the_plugin_loaded_over_it() {
    let builtin: PathBuf = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../plugins_v2/builtin");
    let prelude = Plugin::load(builtin).expect("the builtin declarations load");
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../plugins_v2/testing");
    let plugin = Plugin::load_with_prelude(&prelude, root).expect("the testing plugin loads");
    assert!(
        plugin.macros.get("Type", "Creature").is_some(),
        "a builtin declaration stays in scope under the plugin loaded over it"
    );
    assert!(plugin.macros.get("NounPhrase", "AnyTarget").is_some());
}
