//! The fixture-twin CI gate ([[idris-tables-fixtures-v2]]): the Idris proof
//! suite and the Rust reject fixtures must twin each other in BOTH
//! directions, machine-checked — never hand-maintained comments.
//!
//! Direction A (code → Idris): the emitted checker-rule manifest
//! (`tables/checker-rules.ron`) carries a `twin:` disposition per E-code.
//! `Idris` means at least one `failing` block in `idris/src/Spec.idr` /
//! `Experimental.idr` twins the code — its `-- @twin` annotation names a
//! fixture under `tests/reject/<CODE>/`. `RustOnly(why)` means the rule
//! deliberately has no Idris twin, with a nonempty documented reason — and
//! no annotation may name it (a stale justification is a violation).
//!
//! Direction B (Idris → fixture): every `failing "<msg>"` block must carry
//! a `-- @twin` annotation on the line above — either a reject-fixture path
//! that exists, or `idris-only: <reason>` for invariants with no RON
//! spelling / no E-code.
//!
//! The pure checker (`violations`) is unit-tested RED on synthetic
//! violations below, so the gate's own teeth are pinned.

use std::collections::BTreeSet;
use std::path::Path;
use std::path::PathBuf;

use deckmaste_cards::elaborate::tables;
use deckmaste_cards::elaborate::tables::TwinDisposition;

/// One parsed `-- @twin` annotation (or the lack of one) for a `failing`
/// block: the file/line locate the block in error messages.
#[derive(Debug, Clone)]
enum TwinTarget {
    /// `-- @twin reject/<CODE>/<fixture>.ron`
    Fixture(String),
    /// `-- @twin idris-only: <reason>`
    IdrisOnly(String),
    /// The block has no annotation at all.
    Missing,
}

#[derive(Debug, Clone)]
struct BlockAnnotation {
    file: String,
    line: usize,
    target: TwinTarget,
}

/// Scans an Idris source for `failing "` blocks and their `-- @twin`
/// annotations (the immediately preceding non-blank line).
fn parse_annotations(file: &str, source: &str) -> Vec<BlockAnnotation> {
    let mut blocks = Vec::new();
    let lines: Vec<&str> = source.lines().collect();
    for (i, line) in lines.iter().enumerate() {
        // Doc-comment prose mentioning `failing "` starts with `|||` and is
        // skipped by the starts_with check.
        if !line.trim_start().starts_with("failing \"") {
            continue;
        }
        let prev = lines[..i].iter().rev().find(|l| !l.trim().is_empty());
        let target = match prev {
            Some(p) if p.trim_start().starts_with("-- @twin ") => {
                let rest = p.trim_start().trim_start_matches("-- @twin ").trim();
                if let Some(reason) = rest.strip_prefix("idris-only:") {
                    TwinTarget::IdrisOnly(reason.trim().to_owned())
                } else {
                    TwinTarget::Fixture(rest.to_owned())
                }
            }
            _ => TwinTarget::Missing,
        };
        blocks.push(BlockAnnotation {
            file: file.to_owned(),
            line: i + 1,
            target,
        });
    }
    blocks
}

/// The pure gate: every violation found across both directions.
fn violations(
    manifest: &[(String, TwinDisposition)],
    blocks: &[BlockAnnotation],
    fixture_exists: &dyn Fn(&str) -> bool,
) -> Vec<String> {
    let mut found = Vec::new();
    let codes: BTreeSet<&str> = manifest.iter().map(|(c, _)| c.as_str()).collect();
    let mut twinned_codes: BTreeSet<String> = BTreeSet::new();

    // Direction B: every failing block names its twin (or is idris-only).
    for block in blocks {
        let at = format!("{}:{}", block.file, block.line);
        match &block.target {
            TwinTarget::Missing => found.push(format!(
                "{at}: failing block without a `-- @twin` annotation \
                 (name its reject fixture, or declare `idris-only: <reason>`)"
            )),
            TwinTarget::IdrisOnly(reason) if reason.is_empty() => {
                found.push(format!("{at}: `idris-only:` twin annotation with no reason"));
            }
            TwinTarget::IdrisOnly(_) => {}
            TwinTarget::Fixture(path) => {
                let mut parts = path.split('/');
                let (root, code) = (parts.next().unwrap_or(""), parts.next().unwrap_or(""));
                if root != "reject" || parts.next().is_none() {
                    found.push(format!(
                        "{at}: twin annotation {path:?} is not a reject/<CODE>/<fixture>.ron path"
                    ));
                    continue;
                }
                if !codes.contains(code) {
                    found.push(format!(
                        "{at}: twin annotation names {code:?}, which is no manifest code"
                    ));
                    continue;
                }
                if !fixture_exists(path) {
                    found.push(format!(
                        "{at}: twin annotation names missing fixture {path:?}"
                    ));
                }
                twinned_codes.insert(code.to_owned());
            }
        }
    }

    // Direction A: the manifest dispositions hold against the annotations.
    for (code, disposition) in manifest {
        match disposition {
            TwinDisposition::Idris => {
                if !twinned_codes.contains(code) {
                    found.push(format!(
                        "manifest marks {code} as Idris-twinned, but no failing block's \
                         `-- @twin` annotation names a reject/{code}/ fixture"
                    ));
                }
            }
            TwinDisposition::RustOnly(why) => {
                if why.trim().is_empty() {
                    found.push(format!("manifest marks {code} RustOnly with an empty reason"));
                }
                if twinned_codes.contains(code) {
                    found.push(format!(
                        "manifest marks {code} RustOnly, but a failing block twins it — \
                         the justification is stale; flip the manifest row to Idris"
                    ));
                }
            }
        }
    }
    found
}

fn idris_src(file: &str) -> String {
    let path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../idris/src")
        .join(file);
    std::fs::read_to_string(&path)
        .unwrap_or_else(|e| panic!("reading {}: {e}", path.display()))
}

fn tests_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("tests")
}

/// The real gate over the committed manifest, proof suite, and fixtures.
#[test]
fn every_code_and_every_failing_block_has_its_twin() {
    let manifest: Vec<(String, TwinDisposition)> = tables::tables()
        .rules
        .iter()
        .map(|r| (r.code.clone(), r.twin.clone()))
        .collect();
    let mut blocks = parse_annotations("Spec.idr", &idris_src("Spec.idr"));
    blocks.extend(parse_annotations(
        "Experimental.idr",
        &idris_src("Experimental.idr"),
    ));
    assert!(
        blocks.len() >= 50,
        "the proof suite's failing blocks should all be found (got {})",
        blocks.len()
    );
    let root = tests_root();
    let exists = |path: &str| root.join(path).is_file();
    let found = violations(&manifest, &blocks, &exists);
    assert!(
        found.is_empty(),
        "twin-gate violations:\n  {}",
        found.join("\n  ")
    );
}

/// The gate's own teeth, pinned RED on synthetic violations.
#[cfg(test)]
mod red {
    use super::*;

    fn manifest_idris(code: &str) -> Vec<(String, TwinDisposition)> {
        vec![(code.to_owned(), TwinDisposition::Idris)]
    }

    fn block(target: TwinTarget) -> BlockAnnotation {
        BlockAnnotation {
            file: "Synthetic.idr".to_owned(),
            line: 1,
            target,
        }
    }

    #[test]
    fn a_failing_block_without_annotation_is_a_violation() {
        let found = violations(
            &manifest_idris("E-BIND-IT"),
            &[block(TwinTarget::Missing)],
            &|_| true,
        );
        assert!(
            found.iter().any(|v| v.contains("without a `-- @twin`")),
            "{found:?}"
        );
    }

    #[test]
    fn an_idris_code_with_no_twin_annotation_is_a_violation() {
        let found = violations(&manifest_idris("E-BIND-IT"), &[], &|_| true);
        assert!(
            found.iter().any(|v| v.contains("no failing block")),
            "{found:?}"
        );
    }

    #[test]
    fn a_missing_fixture_file_is_a_violation() {
        let found = violations(
            &manifest_idris("E-BIND-IT"),
            &[block(TwinTarget::Fixture(
                "reject/E-BIND-IT/nonexistent.ron".to_owned(),
            ))],
            &|_| false,
        );
        assert!(
            found.iter().any(|v| v.contains("missing fixture")),
            "{found:?}"
        );
    }

    #[test]
    fn a_stale_rust_only_justification_is_a_violation() {
        let manifest = vec![(
            "E-BIND-NOTE".to_owned(),
            TwinDisposition::RustOnly("some reason".to_owned()),
        )];
        let found = violations(
            &manifest,
            &[block(TwinTarget::Fixture(
                "reject/E-BIND-NOTE/some.ron".to_owned(),
            ))],
            &|_| true,
        );
        assert!(
            found.iter().any(|v| v.contains("justification is stale")),
            "{found:?}"
        );
    }

    #[test]
    fn an_unknown_code_in_an_annotation_is_a_violation() {
        let found = violations(
            &manifest_idris("E-BIND-IT"),
            &[
                block(TwinTarget::Fixture("reject/E-BOGUS/x.ron".to_owned())),
                block(TwinTarget::Fixture("reject/E-BIND-IT/x.ron".to_owned())),
            ],
            &|_| true,
        );
        assert!(
            found.iter().any(|v| v.contains("no manifest code")),
            "{found:?}"
        );
    }
}
