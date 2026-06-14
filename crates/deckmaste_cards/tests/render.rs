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
