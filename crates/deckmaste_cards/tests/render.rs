use std::path::Path;
use std::path::PathBuf;

use deckmaste_cards::plugin::Plugin;
use deckmaste_cards::render::CardView;
use deckmaste_cards::render::RenderedCard;
use deckmaste_cards::render::render;
use deckmaste_cards::render::render_card_face;
use deckmaste_core::Ability;
use deckmaste_core::Card;
use deckmaste_core::CardFace;
use deckmaste_core::KeywordAbility;
use deckmaste_core::StatValue;
use deckmaste_core::Subtype;
use deckmaste_core::Type;

fn canon_path() -> PathBuf { Path::new(env!("CARGO_MANIFEST_DIR")).join("../../plugins/canon") }

fn face(name: &str) -> CardFace {
    let plugin = Plugin::load_with_sibling_prelude(canon_path()).unwrap();
    match plugin.card(name).unwrap() {
        Card::Normal(f) => f,
        other @ Card::ModalDfc(..) => panic!("expected a normal card, got {other:?}"),
    }
}

#[test]
fn renders_vanilla_grizzly_bears() {
    let r = render_card_face(&face("Grizzly Bears"));
    assert_eq!(
        r,
        RenderedCard {
            name: "Grizzly Bears".to_string(),
            mana_cost: "{1}{G}".to_string(),
            type_line: "Creature — Bear".to_string(),
            rules: vec![],
            pt: Some("2/2".to_string()),
        }
    );
}

#[test]
fn renders_keyword_only_wall_of_stone() {
    let r = render_card_face(&face("Wall of Stone"));
    assert_eq!(r.type_line, "Creature — Wall");
    assert_eq!(r.pt, Some("0/8".to_string()));
    assert_eq!(r.rules, vec!["Defender".to_string()]);
}

#[test]
fn renders_spell_lightning_bolt() {
    let r = render_card_face(&face("Lightning Bolt"));
    assert_eq!(r.mana_cost, "{R}");
    assert_eq!(r.type_line, "Instant");
    assert_eq!(r.rules, vec!["Deal 3 damage to any target.".to_string()]);
}

#[test]
fn renders_keywords_and_etb_trigger_baleful_strix() {
    let r = render_card_face(&face("Baleful Strix"));
    assert_eq!(r.mana_cost, "{U}{B}");
    assert_eq!(r.type_line, "Artifact Creature — Bird");
    assert_eq!(r.pt, Some("1/1".to_string()));
    assert_eq!(
        r.rules,
        vec![
            "Flying, Deathtouch".to_string(),
            "When Baleful Strix enters, draw a card.".to_string(),
        ]
    );
}

#[test]
fn renders_state_trigger_goblin_medics() {
    let r = render_card_face(&face("Goblin Medics"));
    assert_eq!(
        r.rules,
        vec!["Whenever Goblin Medics becomes tapped, deal 1 damage to any target.".to_string()]
    );
}

#[test]
fn renders_anthem_glorious_anthem() {
    let r = render_card_face(&face("Glorious Anthem"));
    assert_eq!(r.type_line, "Enchantment");
    assert_eq!(
        r.rules,
        vec!["Creatures you control get +1/+1.".to_string()]
    );
}

#[test]
fn renders_elesh_norn() {
    let r = render_card_face(&face("Elesh Norn, Grand Cenobite"));
    assert_eq!(r.type_line, "Legendary Creature — Phyrexian Praetor");
    assert_eq!(r.pt, Some("4/7".to_string()));
    assert_eq!(
        r.rules,
        vec![
            "Vigilance".to_string(),
            "Other creatures you control get +2/+2.".to_string(),
            "Creatures your opponents control get -2/-2.".to_string(),
        ]
    );
}

#[test]
fn renders_must_attack_goblin_brigand() {
    let r = render_card_face(&face("Goblin Brigand"));
    assert_eq!(
        r.rules,
        vec!["Goblin Brigand attacks each combat if able.".to_string()]
    );
}

#[test]
fn renders_pacifism() {
    let r = render_card_face(&face("Pacifism"));
    assert_eq!(r.type_line, "Enchantment — Aura");
    assert!(
        r.rules
            .contains(&"Enchanted creature can't attack.".to_string())
    );
    assert!(
        r.rules
            .contains(&"Enchanted creature can't block.".to_string())
    );
    // No leaked fallback markers anywhere (the Enchant keyword line must render
    // too):
    assert!(
        r.rules.iter().all(|line| !line.contains("[unrendered")),
        "rules: {:?}",
        r.rules
    );
}

#[test]
fn renders_sequence_brainstorm() {
    let r = render_card_face(&face("Brainstorm"));
    assert_eq!(r.mana_cost, "{U}");
    assert_eq!(
        r.rules,
        vec!["Draw 3 cards, then put 2 cards from your hand on top of your library.".to_string()]
    );
}

// ── Derived / token path ─────────────────────────────────────────────────────

/// A synthesized token: no mana cost, no printed text, but a valid type line
/// and P/T from the runtime-assembled `CardView`.
#[test]
fn renders_a_synthesized_token() {
    let types = [Type::Creature];
    let subs = [Subtype {
        name: "Goblin".into(),
        types: vec![Type::Creature, Type::Kindred],
        confers: vec![],
    }];
    let p = StatValue::Number(1);
    let t = StatValue::Number(1);
    let view = CardView {
        name: "Goblin",
        mana_cost: None,
        supertypes: &[],
        types: &types,
        subtypes: &subs,
        power: Some(&p),
        toughness: Some(&t),
        abilities: &[],
    };
    let r = render(&view);
    assert_eq!(r.mana_cost, "");
    assert_eq!(r.type_line, "Creature — Goblin");
    assert_eq!(r.pt, Some("1/1".to_string()));
    assert!(r.rules.is_empty());
}

/// A derived live object: base stats replaced by layer-applied values, a
/// keyword granted by a continuous effect present on the view.  No printed
/// mana cost (the view carries `None`).
#[test]
fn renders_a_derived_pumped_flier() {
    let types = [Type::Creature];
    let p = StatValue::Number(4);
    let t = StatValue::Number(4);
    let fly = Ability::Keyword(KeywordAbility::Composite {
        name: "Flying".into(),
        abilities: vec![],
    });
    let abilities = [fly];
    let view = CardView {
        name: "Grizzly Bears",
        mana_cost: None,
        supertypes: &[],
        types: &types,
        subtypes: &[],
        power: Some(&p),
        toughness: Some(&t),
        abilities: &abilities,
    };
    let r = render(&view);
    assert_eq!(r.pt, Some("4/4".to_string()));
    assert_eq!(r.rules, vec!["Flying".to_string()]);
}

// ── Full-canon breadth sweep ─────────────────────────────────────────────────

fn canon_card_names() -> Vec<String> {
    std::fs::read_dir(canon_path().join("cards"))
        .unwrap()
        .filter_map(Result::ok)
        .filter_map(|e| {
            let p = e.path();
            (p.extension().and_then(|x| x.to_str()) == Some("ron"))
                .then(|| p.file_stem().map(|s| s.to_string_lossy().into_owned()))
                .flatten()
        })
        .collect()
}

/// Totality sweep: the renderer must never panic on any canon card face.
/// Faces that use out-of-scope grammar will produce `[unrendered: …]` markers,
/// which is intentional — this test only asserts no panic and a non-empty type
/// line.  The `eprintln!` output shows the marker count when run with
/// `-- --nocapture`.
#[test]
fn renders_every_canon_card_without_panicking() {
    let plugin = Plugin::load_with_sibling_prelude(canon_path()).unwrap();
    let mut markers = 0usize;
    let mut total = 0usize;
    for name in canon_card_names() {
        let Ok(card) = plugin.card(&name) else { continue };
        let faces: Vec<CardFace> = match card {
            Card::Normal(f) => vec![f],
            Card::ModalDfc(a, b) => vec![a, b],
        };
        for f in faces {
            let r = render_card_face(&f);
            assert!(!r.type_line.is_empty(), "{name}: empty type line");
            total += 1;
            if r.rules.iter().any(|l| l.contains("[unrendered")) {
                markers += 1;
            }
        }
    }
    eprintln!(
        "rendered {total} canon faces; {markers} still contain an [unrendered] marker (out-of-scope grammar)"
    );
    assert!(total > 0, "no canon cards found");
}

/// Anchor completeness: the ten cards exercised by the golden tests must
/// render with NO `[unrendered]` markers (they are in-scope by definition).
#[test]
fn anchor_cards_fully_rendered() {
    for name in [
        "Lightning Bolt",
        "Baleful Strix",
        "Glorious Anthem",
        "Elesh Norn, Grand Cenobite",
        "Goblin Brigand",
        "Brainstorm",
        "Wall of Stone",
        "Grizzly Bears",
        "Goblin Medics",
        "Pacifism",
    ] {
        let r = render_card_face(&face(name));
        let blob = format!("{} {} {:?}", r.mana_cost, r.type_line, r.rules);
        assert!(
            !blob.contains("[unrendered"),
            "{name} leaked a marker: {blob}"
        );
    }
}

// ── Coverage A: GainLife / LoseLife, Destroy, each-<noun> selections ─────────

#[test]
fn renders_gain_life_deepwood_tantiv() {
    let r = render_card_face(&face("Deepwood Tantiv"));
    // Whenever it becomes blocked, gain 2 life.
    assert!(
        r.rules
            .iter()
            .any(|l| l == "Whenever Deepwood Tantiv becomes blocked, gain 2 life."),
        "rules: {:?}",
        r.rules
    );
}

#[test]
fn renders_damage_to_each_creature_pyroclasm() {
    let r = render_card_face(&face("Pyroclasm"));
    assert_eq!(r.rules, vec!["Deal 2 damage to each creature.".to_string()]);
}

#[test]
fn renders_damage_to_each_player_flame_rift() {
    let r = render_card_face(&face("Flame Rift"));
    assert_eq!(r.rules, vec!["Deal 4 damage to each player.".to_string()]);
}

#[test]
fn renders_synthesized_lose_life_and_destroy() {
    use deckmaste_core::Ability;
    use deckmaste_core::Action;
    use deckmaste_core::CardFace;
    use deckmaste_core::CharacteristicFilter;
    use deckmaste_core::Count;
    use deckmaste_core::Effect;
    use deckmaste_core::Filter;
    use deckmaste_core::PlayerAction;
    use deckmaste_core::Quantity;
    use deckmaste_core::Reference;
    use deckmaste_core::Selection;
    use deckmaste_core::SpellAbility;
    use deckmaste_core::TargetSpec;
    use deckmaste_core::Type;
    // "Lose 3 life." spell
    let lose = CardFace {
        name: "Test Drain".into(),
        types: vec![Type::Sorcery],
        abilities: vec![Ability::Spell(SpellAbility {
            targets: vec![],
            effect: Effect::Act(Action::By(
                Reference::You,
                PlayerAction::LoseLife(Count::Literal(3)),
            )),
        })],
        ..CardFace::default()
    };
    assert_eq!(
        render_card_face(&lose).rules,
        vec!["Lose 3 life.".to_string()]
    );

    // "Destroy target creature." spell
    let destroy = CardFace {
        name: "Test Smite".into(),
        types: vec![Type::Sorcery],
        abilities: vec![Ability::Spell(SpellAbility {
            targets: vec![TargetSpec::Target(
                Quantity::Exactly(Count::Literal(1)),
                Filter::Characteristic(CharacteristicFilter::Type(Type::Creature)),
            )],
            effect: Effect::Act(Action::Destroy(Selection::Ref(Reference::Target(0)))),
        })],
        ..CardFace::default()
    };
    assert_eq!(
        render_card_face(&destroy).rules,
        vec!["Destroy target creature.".to_string()]
    );
}

// ── Coverage B: "dies" event + non-self trigger subjects ─────────────────────

#[test]
fn renders_dies_trigger_footlight_fiend() {
    let r = render_card_face(&face("Footlight Fiend"));
    assert!(
        r.rules
            .iter()
            .any(|l| l == "When Footlight Fiend dies, deal 1 damage to any target."),
        "rules: {:?}",
        r.rules
    );
}

#[test]
fn renders_creature_dies_trigger_moonlit_wake() {
    let r = render_card_face(&face("Moonlit Wake"));
    assert!(
        r.rules
            .iter()
            .any(|l| l == "Whenever a creature dies, gain 1 life."),
        "rules: {:?}",
        r.rules
    );
}

// ── Coverage C: Scope::Of + full Modification vocabulary ────────────────────

#[test]
fn renders_set_colors_darkest_hour() {
    assert_eq!(
        render_card_face(&face("Darkest Hour")).rules,
        vec!["Creatures are black.".to_string()]
    );
}

#[test]
fn renders_gain_ability_serras_blessing() {
    assert_eq!(
        render_card_face(&face("Serra's Blessing")).rules,
        vec!["Creatures you control have vigilance.".to_string()]
    );
}

#[test]
fn renders_humility() {
    assert_eq!(
        render_card_face(&face("Humility")).rules,
        vec!["Creatures lose all abilities and have base power and toughness 1/1.".to_string()]
    );
}

#[test]
fn renders_scope_of_singular() {
    use deckmaste_core::Ability;
    use deckmaste_core::CardFace;
    use deckmaste_core::Count;
    use deckmaste_core::Modification;
    use deckmaste_core::Reference;
    use deckmaste_core::Scope;
    use deckmaste_core::StaticAbility;
    use deckmaste_core::StaticEffect;
    let face = CardFace {
        name: "Test Aura".into(),
        types: vec![Type::Enchantment],
        abilities: vec![Ability::Static(StaticAbility {
            condition: None,
            effects: vec![StaticEffect::Modify {
                of: Scope::Of(Reference::This),
                changes: vec![
                    Modification::AddPower(Count::Literal(1)),
                    Modification::AddToughness(Count::Literal(1)),
                ],
            }],
            characteristic_defining: false,
        })],
        ..CardFace::default()
    };
    assert_eq!(
        render_card_face(&face).rules,
        vec!["Test Aura gets +1/+1.".to_string()]
    );
}
