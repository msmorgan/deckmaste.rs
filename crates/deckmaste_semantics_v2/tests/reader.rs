//! The reader against `plugins_v2/testing`: a plugin with one card per shape
//! the fixture needs, a token, and one file in each of the three rules tables.
//!
//! Card text is real Magic text, taken from `data/mtgjson/AtomicCards.json`.

use std::path::Path;
use std::path::PathBuf;

use deckmaste_semantics_v2::abilities::Ability;
use deckmaste_semantics_v2::abilities::CounterKindSource;
use deckmaste_semantics_v2::abilities::Instruction;
use deckmaste_semantics_v2::abilities::StaticSpec;
use deckmaste_semantics_v2::card::Card;
use deckmaste_semantics_v2::events::ReplUse;
use deckmaste_semantics_v2::phrase::Amount;
use deckmaste_semantics_v2::phrase::ColorTerm;
use deckmaste_semantics_v2::phrase::DetPhrase;
use deckmaste_semantics_v2::phrase::GameEvent;
use deckmaste_semantics_v2::phrase::NounPhrase;
use deckmaste_semantics_v2::phrase::ObservationPoint;
use deckmaste_semantics_v2::phrase::Predicate;
use deckmaste_semantics_v2::phrase::Quantity;
use deckmaste_semantics_v2::phrase::ZoneExpr;
use deckmaste_semantics_v2::phrase::ZoneScope;
use deckmaste_semantics_v2::reader::Plugin;
use deckmaste_semantics_v2::words::CardType;
use deckmaste_semantics_v2::words::Color;
use deckmaste_semantics_v2::words::ColorOrColorless;
use deckmaste_semantics_v2::words::CounterKind;
use deckmaste_semantics_v2::words::ManaSymbol;
use deckmaste_semantics_v2::words::ProjAxis;
use deckmaste_semantics_v2::words::SimpleManaSymbol;
use deckmaste_semantics_v2::words::Stat;
use deckmaste_semantics_v2::words::Subtype;
use deckmaste_semantics_v2::words::Zone;

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
    assert_eq!(
        **determiner,
        DetPhrase::Target {
            quantity: Box::new(Quantity::Range {
                low: Some(1),
                high: Some(1),
            }),
        }
    );
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

/// A catalog entry is the name an effect creates the token by [CR#111.10] and
/// the characteristics that effect writes, qualities included [CR#111.3].
#[test]
fn a_token_reads_as_a_named_catalog_entry() {
    let plugin = testing_plugin();
    let entry = plugin
        .tokens
        .iter()
        .find(|entry| entry.name == "Soldier")
        .expect("the catalog names the file's stem");
    let token = &entry.token;
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
    assert_eq!(
        sba.then,
        Instruction::Move {
            subject: NounPhrase::This,
            to: ZoneExpr::Zone {
                zone: Zone::Graveyard,
                scope: Box::new(ZoneScope::Bare),
            },
            riders: vec![],
        }
    );

    // [CR#306.5b]: the planeswalker's intrinsic "enters with a number of
    // loyalty counters equal to its printed loyalty number", a replacement
    // effect [CR#614.1c] observed after the move onto the battlefield.
    let conferral = &plugin.rules.conferral[0];
    let Ability::Static { spec } = &conferral.confer else {
        panic!("expected a static ability, got {:?}", conferral.confer);
    };
    let StaticSpec::Replacement {
        event,
        alternatives,
        timing,
        replacement,
        r#use,
        limit,
    } = &**spec
    else {
        panic!("expected a replacement, got {spec:?}");
    };
    assert_eq!(
        *event,
        GameEvent::ZoneChange {
            subject: Box::new(NounPhrase::This),
            from: None,
            to: Some(Box::new(ZoneExpr::Zone {
                zone: Zone::Battlefield,
                scope: Box::new(ZoneScope::Bare),
            })),
            observation: ObservationPoint::After,
        }
    );
    assert_eq!(*alternatives, vec![]);
    assert_eq!(*timing, None);
    assert_eq!(*r#use, ReplUse::Repeatedly);
    assert_eq!(*limit, None);
    assert_eq!(
        **replacement,
        Instruction::PutCounters {
            amount: Amount::StatOf {
                axis: ProjAxis::Stat {
                    stat: Stat::Loyalty
                },
                subject: Box::new(NounPhrase::This),
            },
            kind: CounterKindSource::Printed {
                kind: CounterKind::Named {
                    label: "Loyalty".to_string(),
                },
            },
            on: NounPhrase::This,
        }
    );

    let damage = &plugin.rules.damage_result[0];
    assert_eq!(
        damage.recipient,
        Predicate::HasType {
            r#type: CardType::Planeswalker
        }
    );
    // The counter is a registry key: the Lean checker refuses a label the
    // counter facts do not declare, and requires its declared holder to be the
    // kind the recipient binds.
    assert_eq!(
        damage.remove,
        CounterKind::Named {
            label: "Loyalty".to_string(),
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

/// A field the constructor does not declare is refused, naming both — never
/// skipped. serde's default is to ignore an unrecognized key, which read a
/// misspelling as an omission: Fading and Impending wrote `amount:` where the
/// constructor declares `quantity`, and the removal came out count-less
/// (`docs/decisions/semantics-v2.md` §11).
#[test]
fn an_undeclared_field_on_a_constructor_is_refused() {
    let macros = deckmaste_semantics_v2::ron::macro_set();
    let error = macros
        .read_str::<Instruction>("RerollStored(amount: Lit(value: 1), whose: You, agent: You)")
        .expect_err("a field the constructor does not declare is refused");
    let message = error.to_string();
    assert!(
        message.contains("`RerollStored`") && message.contains("`amount`"),
        "the refusal names the constructor and the field: {message}"
    );
    assert!(
        message.contains("`quantity`"),
        "the refusal lists the declared fields: {message}"
    );
}

/// The same refusal at a plain struct position, where the constructor named is
/// the struct itself.
#[test]
fn an_undeclared_field_on_a_struct_is_refused() {
    let macros = deckmaste_semantics_v2::ron::macro_set();
    let error = macros
        .read_str::<Card>(r#"SingleFaced(face: (characteristics: (name: "X", conferral: None)))"#)
        .expect_err("a field the struct does not declare is refused");
    let message = error.to_string();
    assert!(
        message.contains("`Characteristics`") && message.contains("`conferral`"),
        "the refusal names the struct and the field: {message}"
    );
}

/// A declared field still reads: the refusal is about names the constructor
/// does not have, not about named arguments as such.
#[test]
fn a_declared_field_still_reads() {
    let macros = deckmaste_semantics_v2::ron::macro_set();
    macros
        .read_str::<Instruction>("RerollStored(quantity: Range(low: 1), whose: You, agent: You)")
        .expect("the declared spelling reads");
}

/// The chain of one-field constructors between a mana-symbol position and a
/// written colour, spelled out.
fn green_symbol() -> ManaSymbol {
    ManaSymbol::Simple {
        symbol: SimpleManaSymbol::Specific {
            color: ColorOrColorless::Of {
                color: Color::Green,
            },
        },
    }
}

/// A constructor that carries only its payload may be left unwritten: `Green`
/// at a mana-symbol position is the whole chain, one hop at a time.
#[test]
fn a_bare_colour_reads_as_a_mana_symbol() {
    let macros = deckmaste_semantics_v2::ron::macro_set();
    let symbol: ManaSymbol = macros
        .read_str("Green")
        .expect("a bare colour reads at a mana-symbol position");
    assert_eq!(symbol, green_symbol());
}

/// The same elision one link down, and at the other type that injects a colour.
#[test]
fn a_bare_colour_reads_at_every_link_of_the_chain() {
    let macros = deckmaste_semantics_v2::ron::macro_set();
    let simple: SimpleManaSymbol = macros.read_str("Green").expect("at SimpleManaSymbol");
    assert_eq!(
        simple,
        SimpleManaSymbol::Specific {
            color: ColorOrColorless::Of {
                color: Color::Green
            }
        }
    );
    let term: ColorTerm = macros.read_str("Green").expect("at ColorTerm");
    assert_eq!(
        term,
        ColorTerm::Lit {
            color: Color::Green
        }
    );
}

/// The injection adds a spelling; it does not retire the written-out one the
/// binder names give.
#[test]
fn the_written_out_chain_still_reads() {
    let macros = deckmaste_semantics_v2::ron::macro_set();
    let symbol: ManaSymbol = macros
        .read_str("Simple(symbol: Specific(color: Of(color: Green)))")
        .expect("the fully written chain reads");
    assert_eq!(symbol, green_symbol());
}

/// Native dispatch wins: an identifier the position's own grammar claims is
/// never routed to the embedded type.
#[test]
fn a_native_constructor_is_not_routed() {
    let macros = deckmaste_semantics_v2::ron::macro_set();
    let symbol: ManaSymbol = macros.read_str("Snow").expect("`Snow` is ManaSymbol's own");
    assert_eq!(symbol, ManaSymbol::Snow);
}

/// An identifier no link of the chain claims is refused, not silently dropped.
#[test]
fn an_identifier_no_link_of_the_chain_claims_is_refused() {
    let macros = deckmaste_semantics_v2::ron::macro_set();
    let error = macros
        .read_str::<ManaSymbol>("Vigilance")
        .expect_err("`Vigilance` names no mana symbol and no colour");
    let message = error.to_string();
    assert!(
        message.contains("Vigilance"),
        "the refusal names the identifier it could not place: {message}"
    );
}

/// The writer elides exactly what the reader supplies, so a card file spells
/// the colour and nothing else.
#[test]
fn an_injection_writes_bare() {
    let text = deckmaste_semantics_v2::ron::raw_options()
        .to_string(&green_symbol())
        .expect("a mana symbol writes");
    assert_eq!(text, "Green");
}
