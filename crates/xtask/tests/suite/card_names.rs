use std::fmt::Write as FmtWrite;
use std::fs;
use std::path::Path;

use deckmaste_catalogs::CardNameNicknames;
use deckmaste_lexical::Lexicon;
use sha2::Digest;
use sha2::Sha256;

#[test]
fn nickname_catalog_preserves_the_complete_lexical_inventory() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
    let corpus = root.join("data/scryfall/oracle-cards.jsonl");
    let generated = CardNameNicknames::generate(&fs::read(&corpus).unwrap()).unwrap();
    let cached = CardNameNicknames::load(&root.join("data/gen/catalogs"), &corpus).unwrap();
    assert_eq!(cached.nicknames(), generated.nicknames());

    let sources = deckmaste_lexical_source::load_workspace(
        &root,
        &deckmaste_english_v3::grammar::lexical_registry(),
    )
    .unwrap();
    let mut reconstructed = sources.lexemes.clone();
    for lexeme in &mut reconstructed {
        if lexeme.source.path == "data/gen/catalogs/card-names.txt" {
            lexeme.forms[0].surfaces = generated
                .nicknames()
                .get(&lexeme.lemma)
                .map(|nickname| vec![lexeme.lemma.clone(), nickname.clone()]);
        }
    }
    let actual = Lexicon::new(sources.lexemes).unwrap();
    let expected = Lexicon::new(reconstructed).unwrap();
    let actual_sha = digest(&serde_json::to_vec(actual.lexemes()).unwrap());
    let expected_sha = digest(&serde_json::to_vec(expected.lexemes()).unwrap());
    assert_eq!(actual_sha, expected_sha);
    println!(
        "lexical_inventory_sha256={actual_sha}; nicknames={}",
        cached.nicknames().len()
    );
}

fn digest(bytes: &[u8]) -> String {
    let mut result = String::with_capacity(64);
    for byte in Sha256::digest(bytes) {
        write!(result, "{byte:02x}").unwrap();
    }
    result
}
