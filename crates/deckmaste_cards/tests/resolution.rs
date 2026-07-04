//! The per-card resolution-table AGREEMENT ([[idris-tables-fixtures-v2]]):
//! `idris/src/Resolutions.idr` walks selected `Cards.idr` encodings with the
//! Core resolution machinery and exports each card's anaphor resolution
//! table (reference → bound antecedent) to `tests/resolution/<Name>.txt`;
//! this test replays the CANON RON twin of the same card
//! (`plugins/canon/cards/<Name>.ron`) through
//! [`elaborate::elaborate_with_resolutions`] and demands byte-for-byte
//! agreement on the anaphor rows — the mechanical both-ways gate: the Idris
//! model EXPORTS what every reference binds to, the Rust elaborator must
//! REPRODUCE it.
//!
//! Row filter: the fixture pins exactly the `… -> #<depth> …` rows (anaphor
//! reads with a stack depth). The Rust `--dump` view also records
//! registry/class annotations (subtype categories, counter scopes, keyword
//! shapes, `Continuously part` classes, depthless event-role reads) — those
//! have no Idris-side twin table and are excluded by the ` -> #` marker.
//!
//! A card here must exist on BOTH sides with MIRRORING encodings; the export
//! list lives in `Resolutions.idr` (`resolutionFixtures`). Regenerate with
//! `idris/scripts/emit-tables`; drift on either side turns this red.

use std::path::Path;
use std::path::PathBuf;

use deckmaste_cards::elaborate;
use deckmaste_cards::plugin::Plugin;
use deckmaste_core::Card;

fn plugins_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../../plugins")
}

fn resolution_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/resolution")
}

#[test]
fn idris_resolution_fixtures_are_reproduced() {
    let root = plugins_root();
    let plugin = Plugin::load_with_sibling_prelude(root.join("canon")).unwrap();
    let registries = plugin.registries();

    let mut fixtures: Vec<PathBuf> = std::fs::read_dir(resolution_root())
        .expect("tests/resolution exists — regenerate with idris/scripts/emit-tables")
        .map(|entry| entry.unwrap().path())
        .filter(|p| p.extension().is_some_and(|e| e == "txt"))
        .collect();
    fixtures.sort();
    assert!(
        !fixtures.is_empty(),
        "no resolution fixtures — regenerate with idris/scripts/emit-tables"
    );

    for fixture in fixtures {
        let name = fixture.file_stem().unwrap().to_str().unwrap().to_owned();
        let card_path = root.join("canon/cards").join(format!("{name}.ron"));
        let source = std::fs::read_to_string(&card_path).unwrap_or_else(|e| {
            panic!(
                "resolution fixture {name:?} names no canon twin card at {}: {e}",
                card_path.display()
            )
        });
        let card: Card = plugin
            .macros
            .read_str(&source)
            .unwrap_or_else(|e| panic!("parsing {}: {e}", card_path.display()));
        let (result, resolutions) = elaborate::elaborate_with_resolutions(&card, &registries);
        result.unwrap_or_else(|errors| {
            panic!("canon twin {name:?} must elaborate clean, got: {errors:?}")
        });

        let rendered: String = resolutions
            .iter()
            .filter(|r| r.description.contains(" -> #"))
            .map(|r| format!("{r}\n"))
            .collect();
        let expected: String = std::fs::read_to_string(&fixture)
            .unwrap()
            .lines()
            .filter(|l| !l.starts_with('#') && !l.trim().is_empty())
            .map(|l| format!("{l}\n"))
            .collect();
        if rendered != expected {
            let divergence = rendered
                .lines()
                .zip(expected.lines())
                .position(|(a, b)| a != b)
                .map_or_else(
                    || "one side has extra trailing rows".to_owned(),
                    |i| {
                        format!(
                            "first divergence at row {}:\n  rust:  {}\n  idris: {}",
                            i + 1,
                            rendered.lines().nth(i).unwrap_or("<none>"),
                            expected.lines().nth(i).unwrap_or("<none>"),
                        )
                    },
                );
            panic!(
                "resolution table for {name:?} diverged between the Idris export and the \
                 Rust elaborator — the two resolution algorithms moved apart (or the \
                 encodings stopped mirroring). {divergence}\n\
                 rust rows:\n{rendered}\nidris rows:\n{expected}"
            );
        }
    }
}
