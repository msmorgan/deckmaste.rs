//! The reader against `plugins_v2/testing`: a plugin with one card per shape
//! the fixture needs, a token, and one file in each of the three rules tables.
//!
//! Card text is real Magic text, taken from the pinned Scryfall Oracle Cards
//! snapshot.

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
use deckmaste_semantics_v2::words::DesignationLabel;
use deckmaste_semantics_v2::words::ManaSymbol;
use deckmaste_semantics_v2::words::ProjAxis;
use deckmaste_semantics_v2::words::Reach;
use deckmaste_semantics_v2::words::SimpleManaSymbol;
use deckmaste_semantics_v2::words::Stat;
use deckmaste_semantics_v2::words::Subtype;
use deckmaste_semantics_v2::words::Zone;

/// The testing plugin over the builtin prelude, which is how every consumer
/// loads it: since §11.1's macro-only rule a card's constructors are written
/// as the macros `plugins_v2/builtin` declares, so the testing cards no longer
/// read in isolation.
fn testing_plugin() -> Plugin {
    let builtin = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../plugins_v2/builtin");
    let prelude = Plugin::load(builtin).expect("the builtin declarations load");
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../plugins_v2/testing");
    Plugin::load_with_prelude(&prelude, root).expect("the testing plugin loads")
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
    assert!(plugin.macros.get("NounPhrase", "anyTarget").is_some());
    let nested = plugin
        .macros
        .get("Instruction", "dealsDamageToAnyTarget")
        .expect("the damage macro registers at Instruction");
    assert!(
        nested.body().contains("anyTarget"),
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
    assert_eq!(
        characteristics.text,
        [] as [deckmaste_semantics_v2::abilities::Ability; 0]
    );
    assert_eq!(characteristics.loyalty, None);
    assert_eq!(characteristics.defense, None);
    assert_eq!(
        face.choices,
        [] as [deckmaste_semantics_v2::words::QualitySort; 0]
    );
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
    assert_eq!(
        token.qualities,
        [] as [deckmaste_semantics_v2::abilities::TokenQuality; 0]
    );
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
            from: ZoneExpr::Wherever,
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
                    name: "loyaltyCounter".to_string(),
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
    // The counter is a registry key: the Lean checker refuses a name the
    // counter facts do not declare, and requires its declared holder to be the
    // kind the recipient binds.
    assert_eq!(
        damage.remove,
        CounterKind::Named {
            name: "loyaltyCounter".to_string(),
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
        plugin.macros.get("Type", "creature").is_some(),
        "a builtin declaration stays in scope under the plugin loaded over it"
    );
    assert!(plugin.macros.get("NounPhrase", "anyTarget").is_some());
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

/// A numeral reads at its leaf: `3` at an amount position is `Lit(value: 3)`,
/// the constructor Lean marks `semantic_literal`.
#[test]
fn a_bare_numeral_reads_as_an_amount() {
    let macros = deckmaste_semantics_v2::ron::macro_set();
    let amount: Amount = macros.read_str("3").expect("a bare numeral is an amount");
    assert_eq!(amount, Amount::Lit { value: 3 });
    let written = deckmaste_semantics_v2::ron::raw_options()
        .to_string(&amount)
        .expect("an amount writes");
    assert_eq!(written, "3");
}

/// The numeral leaf reached through the injection chain, which is what makes
/// `[2, Green, Blue]` a mana cost.
#[test]
fn a_mana_cost_is_numerals_and_colours() {
    let macros = deckmaste_semantics_v2::ron::macro_set();
    let cost: Vec<ManaSymbol> = macros
        .read_str("[2, Green, Blue]")
        .expect("a mana cost of a numeral and two colours");
    assert_eq!(
        cost,
        vec![
            ManaSymbol::Simple {
                symbol: SimpleManaSymbol::Generic { amount: 2 }
            },
            green_symbol(),
            ManaSymbol::Simple {
                symbol: SimpleManaSymbol::Specific {
                    color: ColorOrColorless::Of { color: Color::Blue }
                }
            },
        ]
    );
    let written = deckmaste_semantics_v2::ron::raw_options()
        .to_string(&cost)
        .expect("a mana cost writes");
    assert_eq!(written, "[2,Green,Blue]");
}

/// The sugar adds a spelling; the written-out leaf still reads.
#[test]
fn the_written_out_numeral_leaf_still_reads() {
    let macros = deckmaste_semantics_v2::ron::macro_set();
    let amount: Amount = macros
        .read_str("Lit(value: 3)")
        .expect("the written leaf reads");
    assert_eq!(amount, Amount::Lit { value: 3 });
}

/// A value that is not a numeral gets no sugar: the amount grammar refuses it.
#[test]
fn a_non_numeral_gets_no_leaf_sugar() {
    let macros = deckmaste_semantics_v2::ron::macro_set();
    macros
        .read_str::<Amount>("\"three\"")
        .expect_err("a string is not an amount");
}

/// A constructor may be applied positionally, in its declared binder order —
/// which is what the Lean bench writes.
#[test]
fn a_constructor_may_be_applied_positionally() {
    let macros = deckmaste_semantics_v2::ron::macro_set();
    let hybrid: ManaSymbol = macros
        .read_str(r"Hybrid(Generic(amount: 1), Red)")
        .expect("a two-field constructor applied positionally");
    assert_eq!(
        hybrid,
        ManaSymbol::Hybrid {
            left: SimpleManaSymbol::Generic { amount: 1 },
            right: Color::Red,
        }
    );
    let subtype: Subtype = macros
        .read_str(r#"Of(Creature, "Gargoyle")"#)
        .expect("a subtype applied positionally");
    assert_eq!(
        subtype,
        Subtype::Of {
            host: CardType::Creature,
            label: "Gargoyle".to_owned(),
        }
    );
}

/// The named form is unchanged, and the two agree.
#[test]
fn the_named_and_positional_forms_agree() {
    let macros = deckmaste_semantics_v2::ron::macro_set();
    let named: ManaSymbol = macros
        .read_str(r"Hybrid(left: Generic(amount: 1), right: Red)")
        .expect("the named form reads");
    let positional: ManaSymbol = macros
        .read_str(r"Hybrid(Generic(amount: 1), Red)")
        .expect("the positional form reads");
    assert_eq!(named, positional);
}

/// The two forms are never mixed: a named argument after a positional one is
/// refused, so an author cannot half-skip a binder.
#[test]
fn a_mixed_application_is_refused() {
    let macros = deckmaste_semantics_v2::ron::macro_set();
    macros
        .read_str::<ManaSymbol>(r"Hybrid(Generic(amount: 1), right: Red)")
        .expect_err("positional then named is neither form");
    macros
        .read_str::<ManaSymbol>(r"Hybrid(left: Generic(amount: 1), Red)")
        .expect_err("named then positional is neither form");
}

/// A constructor of ONE field applies positionally too. ron cannot tell
/// `CountOf(x)` from a newtype — its `handle_any_struct` turns the bare
/// identifier into a unit value, discarding the name — so the reader
/// classifies the argument list itself: binder-led is the named form, and
/// anything else is the single argument.
#[test]
fn a_one_field_constructor_may_be_applied_positionally() {
    let macros = deckmaste_semantics_v2::ron::macro_set();
    let named: Amount = macros
        .read_str("CountOf(group: Pro(Bare, One, Whole))")
        .expect("the named form reads");
    let positional: Amount = macros
        .read_str("CountOf(Pro(Bare, One, Whole))")
        .expect("the positional form reads");
    assert_eq!(named, positional);
    assert!(matches!(positional, Amount::CountOf { .. }));
}

/// A one-field constructor whose argument is itself binder-led is still read
/// positionally: the classification is the ARGUMENT LIST's leading token, and
/// `Pro(...)` opens with a constructor, not a binder.
#[test]
fn a_one_field_argument_may_be_a_named_application() {
    let macros = deckmaste_semantics_v2::ron::macro_set();
    let positional: Amount = macros
        .read_str("CountOf(Pro(reach: Bare, plurality: One, window: Whole))")
        .expect("a named application as the one positional argument");
    let named: Amount = macros
        .read_str("CountOf(group: Pro(Bare, One, Whole))")
        .expect("the named form reads");
    assert_eq!(named, positional);
}

/// A card may write only macros (§11.1, Lean's `Authoring.Form.onlyMacros`):
/// at a `semantic_expression` kind a constructor's own name has no native
/// candidacy, and the macro of that name stands in its place.
#[test]
fn a_card_writes_a_macro_where_the_basis_has_a_constructor() {
    let builtin =
        Plugin::load(Path::new(env!("CARGO_MANIFEST_DIR")).join("../../plugins_v2/builtin"))
            .expect("the builtin declarations load");
    let ability: deckmaste_semantics_v2::abilities::Ability = builtin
        .macros
        .read_str_restricted(r#"keyword(label: "Flying")"#)
        .expect("the macro reads under restriction");
    assert_eq!(
        ability,
        deckmaste_semantics_v2::abilities::Ability::Keyword {
            keyword: "Flying".to_owned(),
            params: vec![],
            body: vec![],
        }
    );
}

/// The refusal half: the constructor the macro stands for is not author
/// vocabulary, and the message names it and the kind it belongs to.
#[test]
fn a_raw_constructor_in_a_card_is_refused_by_name() {
    let builtin =
        Plugin::load(Path::new(env!("CARGO_MANIFEST_DIR")).join("../../plugins_v2/builtin"))
            .expect("the builtin declarations load");
    let error = builtin
        .macros
        .read_str_restricted::<deckmaste_semantics_v2::abilities::Ability>(
            r#"Keyword(keyword: "Flying", params: [], body: [])"#,
        )
        .expect_err("a raw constructor is not author vocabulary");
    let message = error.to_string();
    assert!(
        message.contains("Keyword") && message.contains("Ability"),
        "the refusal must name the constructor and its kind: {message}"
    );
}

/// A definition's name denotes its term (Lean
/// `Semantics.Definition.subtypeTerm`).
///
/// The macro a subtype declaration registers expands to the declaration's
/// `Definition` node, so at a `Subtype` TERM position the position takes the
/// subtype the definition names rather than the node that defines it.
#[test]
fn a_subtype_macro_denotes_the_subtype_its_definition_names() {
    let builtin =
        Plugin::load(Path::new(env!("CARGO_MANIFEST_DIR")).join("../../plugins_v2/builtin"))
            .expect("the builtin declarations load");
    let creature: Subtype = builtin
        .macros
        .read_str("gargoyle")
        .expect("a subtype declaration's name reads at a subtype position");
    assert_eq!(
        creature,
        Subtype::Of {
            host: deckmaste_semantics_v2::words::CardType::Creature,
            label: "Gargoyle".to_owned(),
        }
    );
    // The spell half, whose definition node names no host card type
    // [CR#205.3k].
    let spell: Subtype = builtin
        .macros
        .read_str("adventure")
        .expect("a spell subtype declaration's name reads at a subtype position");
    assert_eq!(
        spell,
        Subtype::Spell {
            label: "Adventure".to_owned(),
        }
    );
}

/// The counter half of the rule above (Lean
/// `Semantics.Definition.counterTerm`; ruling 2026-10-03, "p1p1Counter should
/// read bare, with Printed the default constructor").
///
/// A counter declaration's macro expands to its `Definition::Counter` node, so
/// at a `CounterKind` position the position takes the kind it defines; and
/// `CounterKindSource::Printed` is an injection (§11.1), so at a
/// `CounterKindSource` position the bare kind reads as `Printed`. The bare
/// name, the declaration under a written-out `Printed`, and the constructor
/// under a written-out `Printed` are one value.
#[test]
fn a_counter_macro_denotes_the_kind_its_definition_names_and_reads_bare_as_printed() {
    let builtin =
        Plugin::load(Path::new(env!("CARGO_MANIFEST_DIR")).join("../../plugins_v2/builtin"))
            .expect("the builtin declarations load");
    let named = |name: &str| CounterKind::Named {
        name: name.to_owned(),
    };

    // At a `CounterKind` position: the definition's `kind`, which is the
    // declaration's own name, for every counter (a +X/+Y counter, a keyword
    // counter and a marker counter alike).
    for (source, expected) in [
        ("p1p1Counter", named("p1p1Counter")),
        ("m1m1Counter", named("m1m1Counter")),
        ("flyingCounter", named("flyingCounter")),
        ("chargeCounter", named("chargeCounter")),
        ("poison", named("poison")),
    ] {
        let read: CounterKind = builtin
            .macros
            .read_str_restricted(source)
            .unwrap_or_else(|error| panic!("{source} reads at a `CounterKind` position: {error}"));
        assert_eq!(read, expected, "{source}");
    }

    // At a `CounterKindSource` position: every spelling is one value. The
    // declaration's name reads in a card; the constructor spellings are how a
    // macro BODY could write it.
    let printed = CounterKindSource::Printed {
        kind: named("p1p1Counter"),
    };
    for (source, in_a_card) in [
        ("p1p1Counter", true),
        ("Printed(p1p1Counter)", true),
        ("Printed(kind: p1p1Counter)", true),
        (r#"Printed(Named(name: "p1p1Counter"))"#, false),
        (r#"Named(name: "p1p1Counter")"#, false),
    ] {
        let read: Result<CounterKindSource, _> = if in_a_card {
            builtin.macros.read_str_restricted(source)
        } else {
            builtin.macros.read_str(source)
        };
        let read = read.unwrap_or_else(|error| {
            panic!("{source} reads at a `CounterKindSource` position: {error}")
        });
        assert_eq!(read, printed, "{source}");
    }
    // The source's own constructors still win over the fall-through.
    let own: CounterKindSource = builtin
        .macros
        .read_str_restricted("Those")
        .expect("a native `CounterKindSource` constructor reads");
    assert_eq!(own, CounterKindSource::Those);

    // The injection WRITES bare too, and the bare text reads back.
    let options = deckmaste_semantics_v2::ron::raw_options();
    let written = options
        .to_string(&printed)
        .expect("a counter kind source writes");
    assert!(
        !written.contains("Printed"),
        "the dialect elides the injection: {written}"
    );
    let read_back: CounterKindSource = builtin
        .macros
        .read_str(&written)
        .expect("the bare written form reads back");
    assert_eq!(read_back, printed);

    // A card writes the declaration's name, never the node it expands to.
    let error = builtin
        .macros
        .read_str_restricted::<CounterKind>(
            r#"Counter(kind: Named(name: "chargeCounter"), holder: Object, confers: [])"#,
        )
        .expect_err("a counter definition is not author vocabulary");
    assert!(
        error.to_string().contains("Counter"),
        "the refusal must name the constructor: {error}"
    );
}

/// A designation declaration's name denotes the designation its definition
/// names (Lean `Semantics.Definition.designationTerm`, ruling 2026-10-05).
///
/// The declaration's macro expands to its `Definition::Designation` node, so
/// at a `DesignationLabel` position the position takes the label, which is the
/// declaration's own name, not the phrase it prints ("the monarch"). A
/// declaration that lists members (`dayNight`) registers each member's name
/// [CR#731.1], and its own name, which names no single designation, does not
/// read there.
#[test]
fn a_designation_macro_denotes_the_designation_its_definition_names() {
    let builtin =
        Plugin::load(Path::new(env!("CARGO_MANIFEST_DIR")).join("../../plugins_v2/builtin"))
            .expect("the builtin declarations load");
    for name in [
        "goaded",
        "monarch",
        "ringBearer",
        "leftHalfUnlocked",
        "commander",
        "day",
        "night",
    ] {
        let read: DesignationLabel =
            builtin
                .macros
                .read_str_restricted(name)
                .unwrap_or_else(|error| {
                    panic!("{name} reads at a `DesignationLabel` position: {error}")
                });
        assert_eq!(
            read,
            DesignationLabel::Named {
                name: name.to_owned()
            },
            "{name}"
        );
    }
    assert!(
        builtin
            .macros
            .read_str_restricted::<DesignationLabel>("dayNight")
            .is_err(),
        "a declaration that lists members is no designation of its own"
    );
    // The definition node is not author vocabulary: a card writing it raw at
    // a designation position is refused by name (§11.1).
    let error = builtin
        .macros
        .read_str_restricted::<DesignationLabel>(
            r#"Designation(label: Named(name: "goaded"), scope: HeldByGame, effectful: true, zone: None, type: None, half: None)"#,
        )
        .expect_err("a designation definition is not author vocabulary");
    assert!(
        error.to_string().contains("Designation"),
        "the refusal must name the constructor: {error}"
    );
    // The constructor is how a macro BODY could write it; it is one value.
    let written: DesignationLabel = builtin
        .macros
        .read_str(r#"Named(name: "goaded")"#)
        .expect("the constructor reads");
    assert_eq!(
        written,
        DesignationLabel::Named {
            name: "goaded".to_owned()
        }
    );
}

/// A counter declaration's conferral helpers (`macros/conferrals/`) write what
/// ONE counter confers, and the model applies it once per counter held: a
/// +1/+1 counter's is "+1/+1" with no count in it [CR#122.1a], a keyword
/// counter's is the keyword [CR#122.1b].
#[test]
fn a_counter_conferral_helper_writes_what_one_counter_confers() {
    use deckmaste_semantics_v2::rules::Conferral;
    use deckmaste_semantics_v2::words::Delta;
    let builtin =
        Plugin::load(Path::new(env!("CARGO_MANIFEST_DIR")).join("../../plugins_v2/builtin"))
            .expect("the builtin declarations load");
    let read = |source: &str| -> Conferral {
        builtin
            .macros
            .read_str(source)
            .unwrap_or_else(|error| panic!("{source} reads as a conferral: {error}"))
    };
    let one = || Amount::Lit { value: 1 };
    assert_eq!(
        read("boost(up(1), up(1))"),
        Conferral::Property {
            spec: StaticSpec::PtModification {
                subject: NounPhrase::This,
                power: Delta::Up { amount: one() },
                toughness: Delta::Up { amount: one() },
            },
        }
    );
    assert_eq!(
        read(r#"grants(keyword("Flying"))"#),
        Conferral::Property {
            spec: StaticSpec::AbilityGrant {
                subject: NounPhrase::This,
                ability: Box::new(Ability::Keyword {
                    keyword: "Flying".to_owned(),
                    params: vec![],
                    body: vec![],
                }),
            },
        }
    );
}

/// `NounWord::Type` is an injection (§11.1): "that creature" is
/// `that(Creature)`, the same value as the written-out forms, while the
/// pronoun word's own constructors keep winning.
#[test]
fn a_bare_card_type_reads_as_the_pronoun_word_it_types() {
    let builtin =
        Plugin::load(Path::new(env!("CARGO_MANIFEST_DIR")).join("../../plugins_v2/builtin"))
            .expect("the builtin declarations load");
    let read = |source: &str| -> NounPhrase {
        builtin
            .macros
            .read_str_restricted(source)
            .unwrap_or_else(|error| panic!("{source} reads in a card: {error}"))
    };
    let bare = read("that(Creature)");
    assert_eq!(bare, read("that(Type(Creature))"));
    assert_eq!(bare, read("that(w: Type(type: Creature))"));
    let NounPhrase::Pro { reach, .. } = &bare else {
        panic!("that(…) is a pronoun: {bare:?}");
    };
    assert_eq!(
        reach,
        &Reach::Word {
            word: deckmaste_semantics_v2::words::NounWord::Type {
                r#type: CardType::Creature,
            },
        }
    );

    // The word's own constructors are untouched by the fall-through.
    let player = read("that(Player)");
    let NounPhrase::Pro { reach, .. } = &player else {
        panic!("that(…) is a pronoun: {player:?}");
    };
    assert_eq!(
        reach,
        &Reach::Word {
            word: deckmaste_semantics_v2::words::NounWord::Player,
        }
    );
    let join = read("that(w: Join)");
    let NounPhrase::Pro { reach, .. } = &join else {
        panic!("that(…) is a pronoun: {join:?}");
    };
    assert_eq!(
        reach,
        &Reach::Word {
            word: deckmaste_semantics_v2::words::NounWord::Join,
        }
    );

    // The injection writes bare, and the bare text reads back.
    let written = deckmaste_semantics_v2::ron::raw_options()
        .to_string(&bare)
        .expect("a pronoun writes");
    assert!(
        !written.contains("Type("),
        "the dialect elides the injection: {written}"
    );
    let read_back: NounPhrase = builtin
        .macros
        .read_str(&written)
        .expect("the bare written form reads back");
    assert_eq!(read_back, bare);
}

/// `amass` written over the helper macros expands to exactly the basis term
/// its body spells constructor by constructor, `Param`s substituted
/// [CR#701.47a]: the deed "Amass" done by the actor, whose reminder text reads
/// "you" as the actor.
#[test]
fn amass_expands_to_the_term_its_constructor_body_spelled() {
    let builtin =
        Plugin::load(Path::new(env!("CARGO_MANIFEST_DIR")).join("../../plugins_v2/builtin"))
            .expect("the builtin declarations load");
    let amass: Instruction = builtin
        .macros
        .read_str_restricted("amass(zombie, 2)")
        .expect("amass reads in a card");
    let basis: Instruction = builtin
        .macros
        .read_str(
            r#"
            Enact(verb: Action("Amass"), instruction: Sequentially([
                    DoIf(
                        condition: Not(Exists(Described(
                            determiner: Bare,
                            predicate: And([
                                HasSubtype(Of(host: Creature, label: "Army")),
                                HasType(Creature),
                                HasPossessor(axis: Controller, possessor: Actor),
                            ]),
                        ))),
                        instruction: CreateObject(
                            count: 1,
                            spec: Token(
                                spec: Written((characteristics: (
                                    colors: [Black],
                                    types: [Creature],
                                    subtypes: [Of(host: Creature, label: "Zombie"), Of(host: Creature, label: "Army")],
                                    power: 0,
                                    toughness: 0,
                                ))),
                                riders: [],
                            ),
                            agent: Actor,
                        ),
                        otherwise: None,
                    ),
                    Choose(
                        first: None,
                        chosen: Described(
                            determiner: A(Unmarked),
                            predicate: And([
                                HasSubtype(Of(host: Creature, label: "Army")),
                                HasType(Creature),
                                HasPossessor(axis: Controller, possessor: Actor),
                            ]),
                        ),
                        disclosure: Openly,
                        when: None,
                        agent: Some(Actor),
                    ),
                    PutCounters(
                        amount: Lit(value: 2),
                        kind: Printed(Named(name: "p1p1Counter")),
                        on: Pro(reach: Word(Type(Creature)), plurality: One, window: Whole),
                    ),
                    DoIf(
                        condition: Not(Matches(
                            subject: Pro(reach: Bare, plurality: One, window: Whole),
                            predicate: HasSubtype(Of(host: Creature, label: "Zombie")),
                        )),
                        instruction: Establish(
                            spec: CharacteristicChange(
                                subject: Pro(reach: Bare, plurality: One, window: Whole),
                                edits: [
                                    TypeLine(op: Adds, changes: (subtypes: [Of(host: Creature, label: "Zombie")])),
                                ],
                            ),
                            duration: None,
                        ),
                        otherwise: None,
                    ),
                ]), agent: Some(Actor))
            "#,
        )
        .expect("the constructor-spelled body reads");
    assert_eq!(amass, basis);
}

/// The two helpers that spell amass's token and type change [CR#701.47a]
/// expand to exactly the basis terms they stand for.
#[test]
fn the_create_token_and_add_subtype_helpers_expand_to_their_basis_terms() {
    let builtin =
        Plugin::load(Path::new(env!("CARGO_MANIFEST_DIR")).join("../../plugins_v2/builtin"))
            .expect("the builtin declarations load");

    // The token `amass` creates, with its `Param(0)` taken as `zombie`.
    let helper: Instruction = builtin
        .macros
        .read_str_restricted("createToken(creatureToken(0, 0, [Black], [zombie, army]))")
        .expect("createToken reads in a card");
    let basis: Instruction = builtin
        .macros
        .read_str(
            r#"CreateObject(
                count: 1,
                spec: Token(
                    spec: Written((characteristics: (
                        colors: [Black],
                        types: [Creature],
                        subtypes: [Of(host: Creature, label: "Zombie"), Of(host: Creature, label: "Army")],
                        power: 0,
                        toughness: 0,
                    ))),
                    riders: [],
                ),
                agent: Actor,
            )"#,
        )
        .expect("the basis term reads");
    assert_eq!(helper, basis);

    // The count defaults to one, and takes a caller's amount.
    let two: Instruction = builtin
        .macros
        .read_str_restricted("createToken(creatureToken(0, 0, [Black], [zombie, army]), 2)")
        .expect("createToken takes a count");
    let Instruction::CreateObject { count, .. } = &two else {
        panic!("createToken is a CreateObject: {two:?}");
    };
    assert_eq!(count, &Amount::Lit { value: 2 });

    let helper: StaticSpec = builtin
        .macros
        .read_str_restricted("addSubtype(this, zombie)")
        .expect("addSubtype reads in a card");
    let basis: StaticSpec = builtin
        .macros
        .read_str(
            r#"CharacteristicChange(
                subject: this,
                edits: [TypeLine(op: Adds, changes: (subtypes: [Of(host: Creature, label: "Zombie")]))],
            )"#,
        )
        .expect("the basis term reads");
    assert_eq!(helper, basis);
}

/// Only a `semantic_expression` kind restricts. A word type registered as a
/// kind for its own macro dispatch — a colour, a counter kind, a turn part —
/// keeps every constructor it always had, which is what Lean's `onlyMacros`
/// does.
#[test]
fn a_word_types_constructor_still_reads_under_restriction() {
    let builtin =
        Plugin::load(Path::new(env!("CARGO_MANIFEST_DIR")).join("../../plugins_v2/builtin"))
            .expect("the builtin declarations load");
    let colour: Color = builtin
        .macros
        .read_str_restricted("White")
        .expect("a colour is not a semantic expression");
    assert_eq!(colour, Color::White);
    let counter: deckmaste_semantics_v2::words::CounterKind = builtin
        .macros
        .read_str_restricted(r#"Named(name: "chargeCounter")"#)
        .expect("a counter kind is not a semantic expression");
    assert_eq!(
        counter,
        deckmaste_semantics_v2::words::CounterKind::Named {
            name: "chargeCounter".to_owned(),
        }
    );
}

/// The subtype half of the test above, re-spelled: a subtype is macro-only
/// since `plugins-v2-subtypes-macro-only`, so where a card once wrote
/// `Of(host: Creature, label: "Goblin")` it writes the declaration's name, and
/// the constructor is refused by name at the kind it belongs to.
#[test]
fn a_subtype_in_a_card_is_the_declarations_name_and_not_its_constructor() {
    let builtin =
        Plugin::load(Path::new(env!("CARGO_MANIFEST_DIR")).join("../../plugins_v2/builtin"))
            .expect("the builtin declarations load");
    let subtype: Subtype = builtin
        .macros
        .read_str_restricted("goblin")
        .expect("a subtype declaration's name is author vocabulary");
    assert_eq!(
        subtype,
        Subtype::Of {
            host: deckmaste_semantics_v2::words::CardType::Creature,
            label: "Goblin".to_owned(),
        }
    );
    let error = builtin
        .macros
        .read_str_restricted::<Subtype>(r#"Of(host: Creature, label: "Goblin")"#)
        .expect_err("a subtype constructor is not author vocabulary");
    let message = error.to_string();
    assert!(
        message.contains("Of") && message.contains("Subtype"),
        "the refusal must name the constructor and its kind: {message}"
    );
    // The definition node the macro expands to is refused at the same
    // position, for the same reason: a card writes the name, not what the
    // name denotes.
    let error = builtin
        .macros
        .read_str_restricted::<Subtype>(
            r#"Subtype(subtype: Of(host: Creature, label: "Goblin"), rules: [])"#,
        )
        .expect_err("a subtype definition is not author vocabulary either");
    assert!(
        error.to_string().contains("Subtype"),
        "the refusal must name the kind: {error}"
    );
}

/// "Target creature gets +2/+2 until end of turn" is one `PtModification`
/// whose subject is written once, so the one target is written once.
#[test]
fn gets_writes_one_stat_change_naming_its_subject_once() {
    let builtin =
        Plugin::load(Path::new(env!("CARGO_MANIFEST_DIR")).join("../../plugins_v2/builtin"))
            .expect("the builtin declarations load");
    let gets: Instruction = builtin
        .macros
        .read_str_restricted("gets(target(creature), up(2), up(2), untilEndOfTurn)")
        .expect("gets reads in a card");
    let basis: Instruction = builtin
        .macros
        .read_str_restricted(
            "establish(ptModification(target(creature), up(2), up(2)), untilEndOfTurn)",
        )
        .expect("the constructor aliases read in a card");
    assert_eq!(gets, basis);
    let Instruction::Establish { spec, .. } = &gets else {
        panic!("gets establishes a static clause: {gets:?}");
    };
    assert!(
        matches!(**spec, StaticSpec::PtModification { .. }),
        "one node, not a conjunction of two modifications: {spec:?}"
    );
}

/// `getsBoth(subject, delta, duration)` is `gets` with the one delta written
/// for both stats.
#[test]
fn gets_both_is_gets_with_the_same_delta_twice() {
    let builtin =
        Plugin::load(Path::new(env!("CARGO_MANIFEST_DIR")).join("../../plugins_v2/builtin"))
            .expect("the builtin declarations load");
    let both: Instruction = builtin
        .macros
        .read_str_restricted(
            "getsBoth(thisCreature, up(times(2, countOf(creature))), untilEndOfTurn)",
        )
        .expect("getsBoth reads in a card");
    let gets: Instruction = builtin
        .macros
        .read_str_restricted(
            "gets(thisCreature, up(times(2, countOf(creature))), up(times(2, countOf(creature))), untilEndOfTurn)",
        )
        .expect("gets reads in a card");
    assert_eq!(both, gets);
}
