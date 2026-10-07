use std::collections::BTreeMap;
use std::fs;
use std::path::Path;

use deckmaste_catalogs::CardNameNicknames;
use deckmaste_catalogs::CatalogSet;

const RECORD: &str = r#"{"object":"card","id":"printing","oracle_id":"oracle","name":"Dina, Soul Steeper","layout":"normal","type_line":"Legendary Creature — Dryad Druid","oracle_text":"Whenever you gain life, each opponent loses 1 life.\n{1}, Sacrifice another creature: Dina gets +X/+0 until end of turn, where X is the sacrificed creature's power.","legalities":{"vintage":"legal"}}"#;

fn fixture() -> (tempfile::TempDir, std::path::PathBuf, std::path::PathBuf) {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
    let evidence = root.join("target/catalogs-tests");
    fs::create_dir_all(&evidence).unwrap();
    let temp = tempfile::tempdir_in(evidence).unwrap();
    let corpus = temp.path().join("cards.jsonl");
    let output = temp.path().join("catalogs");
    fs::write(&corpus, RECORD).unwrap();
    let catalogs = CatalogSet::load(root.join("data/gen/catalogs")).unwrap();
    let nicknames = CardNameNicknames::generate(RECORD.as_bytes()).unwrap();
    catalogs
        .write_to_with_nicknames(&output, &nicknames)
        .unwrap();
    (temp, corpus, output)
}

#[test]
fn nickname_catalog_preserves_the_derived_map() {
    let (_temp, corpus, output) = fixture();
    let loaded = CardNameNicknames::load(&output, &corpus).unwrap();
    assert_eq!(
        loaded.nicknames(),
        &BTreeMap::from([("Dina, Soul Steeper".into(), "Dina".into())])
    );
    assert_eq!(
        loaded,
        CardNameNicknames::generate(&fs::read(corpus).unwrap()).unwrap()
    );
}

#[test]
fn stale_size_and_same_size_content_request_regeneration() {
    for replacement in [format!("{RECORD}\n"), RECORD.replace("Dryad", "Human")] {
        let (_temp, corpus, output) = fixture();
        fs::write(&corpus, replacement).unwrap();
        let error = CardNameNicknames::load(&output, &corpus)
            .unwrap_err()
            .to_string();
        assert!(error.contains("stale"), "{error}");
        assert!(
            error.contains("regenerate with `cargo xtask catalogs generate`"),
            "{error}"
        );
    }
}

#[test]
fn missing_malformed_and_old_version_catalogs_request_regeneration() {
    for corruption in ["missing", "malformed", "version"] {
        let (_temp, corpus, output) = fixture();
        let path = output.join(CardNameNicknames::FILENAME);
        match corruption {
            "missing" => fs::remove_file(&path).unwrap(),
            "malformed" => fs::write(&path, "{").unwrap(),
            "version" => {
                let bytes = fs::read_to_string(&path).unwrap();
                fs::write(&path, bytes.replace("\"version\":1", "\"version\":0")).unwrap();
            }
            _ => unreachable!(),
        }
        let error = CardNameNicknames::load(&output, &corpus)
            .unwrap_err()
            .to_string();
        assert!(
            error.contains("regenerate with `cargo xtask catalogs generate`"),
            "{error}"
        );
        assert!(error.contains("card-name-nicknames.json"), "{error}");
    }
}
