use std::path::Path;
use std::path::PathBuf;

use deckmaste_cards::plugin::Plugin;
use deckmaste_cards::render::RenderedCard;
use deckmaste_cards::render::render_card_face;
use deckmaste_core::Card;
use deckmaste_core::CardFace;

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
