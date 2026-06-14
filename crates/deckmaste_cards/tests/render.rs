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
