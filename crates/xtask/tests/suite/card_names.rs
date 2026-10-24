use std::fs;
use std::path::Path;

use deckmaste_catalogs::CardNameNicknames;

#[test]
fn nickname_catalog_matches_its_generator() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
    let corpus = root.join("data/scryfall/oracle-cards.jsonl");
    let generated = CardNameNicknames::generate(&fs::read(&corpus).unwrap()).unwrap();
    let cached = CardNameNicknames::load(&root.join("data/gen/catalogs"), &corpus).unwrap();
    assert_eq!(cached.nicknames(), generated.nicknames());
}
