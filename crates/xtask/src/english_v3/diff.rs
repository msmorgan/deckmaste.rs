//! Face-by-face comparison of two census reports.
//!
//! A report, never a gate: every outcome exits 0 once both inputs parse.

use std::collections::BTreeMap;
use std::collections::BTreeSet;
use std::io::Write;
use std::path::Path;
use std::path::PathBuf;

use anyhow::Context;
use anyhow::Result;
use anyhow::ensure;
use clap::Args;
use serde::Deserialize;
use serde::Serialize;

use crate::raw_corpus::SelectionRequest;

/// Lines printed per face list on stdout; `--json` keeps every entry.
const LIST_CAP: usize = 50;

#[derive(Debug, Args)]
pub struct DiffArgs {
    /// Census report taken as the baseline.
    before: PathBuf,
    /// Census report compared against the baseline.
    after: PathBuf,
    /// Write the full comparison, uncapped, as JSON.
    #[arg(long, value_name = "OUT")]
    json: Option<PathBuf>,
    /// Write an identity manifest (`--identity-manifest` format) of every
    /// gained, lost, decreased and increased face plus `--random` controls.
    #[arg(long, value_name = "OUT")]
    manifest: Option<PathBuf>,
    /// Unchanged faces covered in BEFORE to add to the manifest as controls.
    #[arg(long, value_name = "N", default_value_t = 0, requires = "manifest")]
    random: usize,
    /// Seed for choosing the `--random` controls.
    #[arg(long, value_name = "S", default_value_t = 0, requires = "manifest")]
    seed: u64,
    /// Also list changed line counts and digests. Both reports must have line
    /// arrays.
    #[arg(long)]
    lines: bool,
}

/// The per-face fields of a census report that the comparison reads. Unknown
/// fields are ignored, so stitched reports without run metadata also load.
#[derive(Debug, Clone, Deserialize)]
pub(super) struct CensusFace {
    pub id: String,
    pub name: String,
    #[serde(default)]
    pub face_name: Option<String>,
    pub analyzed_source_sha256: String,
    pub checked_readings: usize,
    pub readings_sha256: Option<String>,
    #[serde(default)]
    pub lines: Option<Vec<CensusLine>>,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub(super) struct CensusLine {
    pub span: (usize, usize),
    pub checked_readings: usize,
    pub readings_sha256: Option<String>,
}

#[derive(Debug, Serialize)]
struct LineEntry {
    id: String,
    name: String,
    line_index: usize,
    before_span: Option<(usize, usize)>,
    after_span: Option<(usize, usize)>,
    before: Option<usize>,
    after: Option<usize>,
    before_digest: Option<String>,
    after_digest: Option<String>,
}

impl CensusFace {
    fn display_name(&self) -> &str {
        self.face_name.as_deref().unwrap_or(&self.name)
    }
}

#[derive(Debug, Deserialize)]
pub(super) struct CensusFile {
    #[serde(default)]
    pub change_id: Option<String>,
    pub faces: Vec<CensusFace>,
}

pub(super) fn load(path: &Path) -> Result<CensusFile> {
    let bytes = std::fs::read(path).with_context(|| format!("reading {}", path.display()))?;
    serde_json::from_slice(&bytes).with_context(|| format!("parsing census {}", path.display()))
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Change {
    Lost,
    Gained,
    Decrease,
    Increase,
    DigestOnly,
    Unchanged,
}

fn classify(before: &CensusFace, after: &CensusFace) -> Change {
    let (b, a) = (before.checked_readings, after.checked_readings);
    if b > 0 && a == 0 {
        Change::Lost
    } else if b == 0 && a > 0 {
        Change::Gained
    } else if a < b {
        Change::Decrease
    } else if a > b {
        Change::Increase
    } else if b > 0 && before.readings_sha256 != after.readings_sha256 {
        Change::DigestOnly
    } else {
        Change::Unchanged
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub(super) struct Entry {
    pub id: String,
    pub name: String,
    pub before: usize,
    pub after: usize,
}

#[derive(Debug, Serialize)]
pub(super) struct Side {
    pub path: PathBuf,
    pub change_id: Option<String>,
    pub faces: usize,
    /// Covered faces and Readings among the faces present in both reports.
    pub covered: usize,
    pub readings: usize,
}

#[derive(Debug, Serialize)]
pub(super) struct Comparison {
    pub before: Side,
    pub after: Side,
    pub compared: usize,
    pub unchanged: usize,
    pub only_in_before: Vec<String>,
    pub only_in_after: Vec<String>,
    pub source_changed: Vec<Entry>,
    pub lost: Vec<Entry>,
    pub gained: Vec<Entry>,
    pub decreases: Vec<Entry>,
    pub increases: Vec<Entry>,
    pub digest_only: Vec<Entry>,
    /// Faces covered in BEFORE and unchanged, eligible as manifest controls.
    #[serde(skip)]
    pub unchanged_covered: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    line_changes: Option<Vec<LineEntry>>,
}

pub(super) fn compare(
    before_path: &Path,
    before: &CensusFile,
    after_path: &Path,
    after: &CensusFile,
) -> Comparison {
    let after_faces: BTreeMap<&str, &CensusFace> = after
        .faces
        .iter()
        .map(|face| (face.id.as_str(), face))
        .collect();
    let before_ids: BTreeSet<&str> = before.faces.iter().map(|face| face.id.as_str()).collect();
    let side = |path: &Path, file: &CensusFile| Side {
        path: path.to_owned(),
        change_id: file.change_id.clone(),
        faces: file.faces.len(),
        covered: 0,
        readings: 0,
    };
    let mut result = Comparison {
        before: side(before_path, before),
        after: side(after_path, after),
        compared: 0,
        unchanged: 0,
        only_in_before: Vec::new(),
        only_in_after: after
            .faces
            .iter()
            .filter(|face| !before_ids.contains(face.id.as_str()))
            .map(|face| face.id.clone())
            .collect(),
        source_changed: Vec::new(),
        lost: Vec::new(),
        gained: Vec::new(),
        decreases: Vec::new(),
        increases: Vec::new(),
        digest_only: Vec::new(),
        unchanged_covered: Vec::new(),
        line_changes: None,
    };
    for b in &before.faces {
        let Some(a) = after_faces.get(b.id.as_str()) else {
            result.only_in_before.push(b.id.clone());
            continue;
        };
        result.compared += 1;
        result.before.covered += usize::from(b.checked_readings > 0);
        result.after.covered += usize::from(a.checked_readings > 0);
        result.before.readings += b.checked_readings;
        result.after.readings += a.checked_readings;
        let entry = Entry {
            id: b.id.clone(),
            name: a.display_name().to_owned(),
            before: b.checked_readings,
            after: a.checked_readings,
        };
        if b.analyzed_source_sha256 != a.analyzed_source_sha256 {
            result.source_changed.push(entry.clone());
        }
        match classify(b, a) {
            Change::Lost => result.lost.push(entry),
            Change::Gained => result.gained.push(entry),
            Change::Decrease => result.decreases.push(entry),
            Change::Increase => result.increases.push(entry),
            Change::DigestOnly => result.digest_only.push(entry),
            Change::Unchanged => {
                result.unchanged += 1;
                if b.checked_readings > 0 {
                    result.unchanged_covered.push(b.id.clone());
                }
            }
        }
    }
    for list in [
        &mut result.source_changed,
        &mut result.lost,
        &mut result.gained,
        &mut result.decreases,
        &mut result.increases,
        &mut result.digest_only,
    ] {
        list.sort_by(|x, y| x.name.cmp(&y.name).then_with(|| x.id.cmp(&y.id)));
    }
    result.only_in_before.sort();
    result.only_in_after.sort();
    result.unchanged_covered.sort();
    result
}

fn write_text(result: &Comparison, out: &mut dyn Write) -> std::io::Result<()> {
    let side = |side: &Side| match &side.change_id {
        Some(change) => format!(
            "{} (change {}, {} faces)",
            side.path.display(),
            short(change),
            side.faces
        ),
        None => format!("{} ({} faces)", side.path.display(), side.faces),
    };
    writeln!(out, "before: {}", side(&result.before))?;
    writeln!(out, "after:  {}", side(&result.after))?;
    let rows = [
        ("faces compared", result.compared),
        ("only in before", result.only_in_before.len()),
        ("only in after", result.only_in_after.len()),
        ("source-hash changes", result.source_changed.len()),
        ("lost", result.lost.len()),
        ("gained", result.gained.len()),
        ("decreases", result.decreases.len()),
        ("increases", result.increases.len()),
        ("digest-only", result.digest_only.len()),
        ("unchanged", result.unchanged),
    ];
    for (label, count) in rows {
        writeln!(out, "{label:<22}{count}")?;
    }
    writeln!(
        out,
        "{:<22}{} -> {}",
        "covered", result.before.covered, result.after.covered
    )?;
    writeln!(
        out,
        "{:<22}{} -> {}",
        "readings", result.before.readings, result.after.readings
    )?;
    for (label, list) in [
        ("lost", &result.lost),
        ("decreases", &result.decreases),
        ("increases", &result.increases),
        ("digest-only", &result.digest_only),
        ("gained", &result.gained),
    ] {
        if list.is_empty() {
            continue;
        }
        writeln!(out, "\n{label} ({}):", list.len())?;
        for entry in list.iter().take(LIST_CAP) {
            writeln!(out, "  {}  {} -> {}", entry.name, entry.before, entry.after)?;
        }
        if list.len() > LIST_CAP {
            writeln!(out, "  … and {} more", list.len() - LIST_CAP)?;
        }
    }
    Ok(())
}

fn compare_lines(before: &CensusFile, after: &CensusFile) -> Result<Vec<LineEntry>> {
    ensure!(
        before
            .faces
            .iter()
            .chain(&after.faces)
            .all(|face| face.lines.is_some()),
        "--lines requires line arrays on every face in both reports"
    );
    let after_faces: BTreeMap<_, _> = after.faces.iter().map(|face| (&face.id, face)).collect();
    let mut result = Vec::new();
    for b in &before.faces {
        let Some(a) = after_faces.get(&b.id) else {
            continue;
        };
        let b_lines = b.lines.as_ref().expect("line arrays checked");
        let a_lines = a.lines.as_ref().expect("line arrays checked");
        for index in 0..b_lines.len().max(a_lines.len()) {
            let b_line = b_lines.get(index);
            let a_line = a_lines.get(index);
            if b_line.map(|line| (line.checked_readings, &line.readings_sha256))
                == a_line.map(|line| (line.checked_readings, &line.readings_sha256))
            {
                continue;
            }
            result.push(LineEntry {
                id: b.id.clone(),
                name: a.display_name().to_owned(),
                line_index: index,
                before_span: b_line.map(|line| line.span),
                after_span: a_line.map(|line| line.span),
                before: b_line.map(|line| line.checked_readings),
                after: a_line.map(|line| line.checked_readings),
                before_digest: b_line.and_then(|line| line.readings_sha256.clone()),
                after_digest: a_line.and_then(|line| line.readings_sha256.clone()),
            });
        }
    }
    result.sort_by(|a, b| {
        a.name
            .cmp(&b.name)
            .then_with(|| a.id.cmp(&b.id))
            .then_with(|| a.line_index.cmp(&b.line_index))
    });
    Ok(result)
}

fn short(change: &str) -> &str {
    change.get(..12).unwrap_or(change)
}

/// `SplitMix64`: a fixed, dependency-free generator so a seed names one sample.
struct SplitMix64(u64);

impl SplitMix64 {
    fn next(&mut self) -> u64 {
        self.0 = self.0.wrapping_add(0x9e37_79b9_7f4a_7c15);
        let mut z = self.0;
        z = (z ^ (z >> 30)).wrapping_mul(0xbf58_476d_1ce4_e5b9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94d0_49bb_1331_11eb);
        z ^ (z >> 31)
    }
}

/// Every count-changed face plus `random` seeded controls drawn from the
/// faces covered in BEFORE and unchanged.
pub(super) fn manifest(result: &Comparison, random: usize, seed: u64) -> SelectionRequest {
    let mut ids: BTreeSet<String> = [
        &result.gained,
        &result.lost,
        &result.decreases,
        &result.increases,
    ]
    .into_iter()
    .flatten()
    .map(|entry| entry.id.clone())
    .collect();
    let mut pool = result.unchanged_covered.clone();
    let take = random.min(pool.len());
    let mut rng = SplitMix64(seed);
    for index in 0..take {
        let remaining = (pool.len() - index) as u64;
        let pick = index + usize::try_from(rng.next() % remaining).expect("index fits usize");
        pool.swap(index, pick);
    }
    ids.extend(pool.into_iter().take(take));
    SelectionRequest {
        face_identities: ids,
        ..SelectionRequest::default()
    }
}

/// Compare two census reports and print the face categories.
///
/// # Errors
/// Only when an input cannot be read or parsed, or an output cannot be written.
fn require_current_versions(path: &Path) -> Result<()> {
    let json: serde_json::Value = serde_json::from_slice(&std::fs::read(path)?)?;
    for (field, version) in [
        ("schema_version", super::report::SCHEMA_VERSION),
        (
            "readings_digest_version",
            super::report::READINGS_DIGEST_VERSION,
        ),
        (
            "decomposition_version",
            super::report::DECOMPOSITION_VERSION,
        ),
        ("context_version", super::report::CONTEXT_VERSION),
    ] {
        ensure!(
            json[field] == version,
            "comparison requires the new baseline and matching {field}; use --migration-from for the one re-baseline"
        );
    }
    Ok(())
}

pub fn run(args: &DiffArgs, out: &mut dyn Write) -> Result<()> {
    require_current_versions(&args.before)?;
    require_current_versions(&args.after)?;
    let before = load(&args.before)?;
    let after = load(&args.after)?;
    let mut result = compare(&args.before, &before, &args.after, &after);
    if args.lines {
        result.line_changes = Some(compare_lines(&before, &after)?);
    }
    write_text(&result, out)?;
    if let Some(lines) = &result.line_changes {
        writeln!(out, "\nchanged lines ({}):", lines.len())?;
        for line in lines.iter().take(LIST_CAP) {
            writeln!(
                out,
                "  {} line {} {:?} -> {:?}: {:?} -> {:?}",
                line.name,
                line.line_index,
                line.before_span,
                line.after_span,
                line.before,
                line.after
            )?;
        }
        if lines.len() > LIST_CAP {
            writeln!(out, "  … and {} more", lines.len() - LIST_CAP)?;
        }
    }
    if let Some(path) = &args.json {
        std::fs::write(path, serde_json::to_vec_pretty(&result)?)
            .with_context(|| format!("writing {}", path.display()))?;
    }
    if let Some(path) = &args.manifest {
        let request = manifest(&result, args.random, args.seed);
        std::fs::write(path, serde_json::to_vec_pretty(&request)?)
            .with_context(|| format!("writing {}", path.display()))?;
        writeln!(
            out,
            "\nmanifest: {} faces -> {}",
            request.face_identities.len(),
            path.display()
        )?;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn face(id: &str, readings: usize, digest: Option<&str>, source: &str) -> CensusFace {
        CensusFace {
            id: id.to_owned(),
            name: format!("Card {id}"),
            face_name: None,
            analyzed_source_sha256: source.to_owned(),
            checked_readings: readings,
            readings_sha256: digest.map(str::to_owned),
            lines: None,
        }
    }

    fn census(faces: Vec<CensusFace>) -> CensusFile {
        CensusFile {
            change_id: Some("abcdefabcdefabcdef".to_owned()),
            faces,
        }
    }

    fn ids(list: &[Entry]) -> Vec<&str> {
        list.iter().map(|entry| entry.id.as_str()).collect()
    }

    fn run_pair(before: Vec<CensusFace>, after: Vec<CensusFace>) -> Comparison {
        compare(
            Path::new("before.json"),
            &census(before),
            Path::new("after.json"),
            &census(after),
        )
    }

    fn fixture() -> Comparison {
        run_pair(
            vec![
                face("lost", 2, Some("d1"), "s"),
                face("gained", 0, Some("d0"), "s"),
                face("dec", 5, Some("d5"), "s"),
                face("inc", 1, Some("d1"), "s"),
                face("digest", 3, Some("a"), "s"),
                face("same", 4, Some("d4"), "s"),
                face("uncovered", 0, None, "s"),
                face("source", 1, Some("x"), "old"),
                face("before-only", 7, Some("d7"), "s"),
            ],
            vec![
                face("lost", 0, Some("d0"), "s"),
                face("gained", 3, Some("d3"), "s"),
                face("dec", 2, Some("d2"), "s"),
                face("inc", 4, Some("d4"), "s"),
                face("digest", 3, Some("b"), "s"),
                face("same", 4, Some("d4"), "s"),
                face("uncovered", 0, None, "s"),
                face("source", 1, Some("x"), "new"),
                face("after-only", 1, Some("d1"), "s"),
            ],
        )
    }

    #[test]
    fn each_count_change_lands_in_its_category() {
        let result = fixture();
        assert_eq!(ids(&result.lost), ["lost"]);
        assert_eq!(ids(&result.gained), ["gained"]);
        assert_eq!(ids(&result.decreases), ["dec"]);
        assert_eq!(ids(&result.increases), ["inc"]);
        assert_eq!(ids(&result.digest_only), ["digest"]);
        assert_eq!(result.unchanged, 3);
        assert_eq!(result.unchanged_covered, ["same", "source"]);
        assert_eq!(
            result.decreases[0],
            Entry {
                id: "dec".to_owned(),
                name: "Card dec".to_owned(),
                before: 5,
                after: 2
            }
        );
    }

    #[test]
    fn source_hash_change_is_reported_beside_the_count_category() {
        let result = fixture();
        assert_eq!(ids(&result.source_changed), ["source"]);
    }

    #[test]
    fn faces_present_in_one_report_are_listed_not_compared() {
        let result = fixture();
        assert_eq!(result.compared, 8);
        assert_eq!(result.only_in_before, ["before-only"]);
        assert_eq!(result.only_in_after, ["after-only"]);
        assert_eq!((result.before.faces, result.after.faces), (9, 9));
        // Totals cover compared faces only, so a bounded census diffs cleanly.
        assert_eq!((result.before.covered, result.after.covered), (6, 6));
        assert_eq!((result.before.readings, result.after.readings), (16, 17));
    }

    #[test]
    fn uncovered_faces_with_differing_digests_are_unchanged() {
        let result = run_pair(
            vec![face("x", 0, None, "s")],
            vec![face("x", 0, Some("empty"), "s")],
        );
        assert_eq!(result.digest_only, Vec::<Entry>::new());
        assert_eq!(result.unchanged, 1);
        assert_eq!(result.unchanged_covered, Vec::<String>::new());
    }

    #[test]
    fn text_lists_are_capped_with_a_remainder_line() {
        let before = (0..LIST_CAP + 3)
            .map(|n| face(&format!("{n:03}"), 0, None, "s"))
            .collect();
        let after = (0..LIST_CAP + 3)
            .map(|n| face(&format!("{n:03}"), 1, Some("d"), "s"))
            .collect();
        let result = run_pair(before, after);
        let mut text = Vec::new();
        write_text(&result, &mut text).unwrap();
        let text = String::from_utf8(text).unwrap();
        assert!(text.contains("gained                53\n"), "{text}");
        assert!(text.contains("\ngained (53):\n"), "{text}");
        assert!(text.contains("  Card 049  0 -> 1\n"), "{text}");
        assert!(!text.contains("Card 050"), "{text}");
        assert!(text.contains("  … and 3 more\n"), "{text}");
        assert!(text.contains("covered               0 -> 53\n"), "{text}");
    }

    #[test]
    fn manifest_holds_every_count_change_and_seeded_controls() {
        let mut before = vec![
            face("lost", 2, Some("a"), "s"),
            face("gained", 0, None, "s"),
            face("dec", 5, Some("a"), "s"),
            face("inc", 1, Some("a"), "s"),
            face("digest", 3, Some("a"), "s"),
        ];
        let mut after = vec![
            face("lost", 0, None, "s"),
            face("gained", 1, Some("b"), "s"),
            face("dec", 2, Some("b"), "s"),
            face("inc", 4, Some("b"), "s"),
            face("digest", 3, Some("b"), "s"),
        ];
        for n in 0..20 {
            before.push(face(&format!("c{n:02}"), 1, Some("a"), "s"));
            after.push(face(&format!("c{n:02}"), 1, Some("a"), "s"));
        }
        let result = run_pair(before, after);
        let first = manifest(&result, 5, 7);
        let controls: BTreeSet<_> = first
            .face_identities
            .iter()
            .filter(|id| id.starts_with('c'))
            .cloned()
            .collect();
        assert_eq!(controls.len(), 5);
        for id in ["lost", "gained", "dec", "inc"] {
            assert!(first.face_identities.contains(id), "{id}");
        }
        assert!(!first.face_identities.contains("digest"));
        assert_eq!(first.face_identities.len(), 9);
        assert_eq!(manifest(&result, 5, 7), first, "same seed, same sample");
        assert_ne!(manifest(&result, 5, 8), first, "seed selects the sample");
        assert_eq!(manifest(&result, 100, 7).face_identities.len(), 24);
        let json: SelectionRequest =
            serde_json::from_slice(&serde_json::to_vec(&first).unwrap()).unwrap();
        assert_eq!(json, first, "manifest reads back as an identity manifest");
    }

    #[test]
    fn stitched_reports_without_run_metadata_load() {
        let file: CensusFile = serde_json::from_str(
            r#"{"provenance": {}, "faces": [{"id": "x#card", "name": "X", "face_name": null,
                "analyzed_source_sha256": "s", "checked_readings": 1,
                "readings_sha256": "d", "samples": []}]}"#,
        )
        .unwrap();
        assert_eq!(file.change_id, None);
        assert_eq!(file.faces.len(), 1);
    }
}

#[cfg(test)]
mod line_tests {
    use super::*;

    fn census(count: usize, digest: &str) -> CensusFile {
        CensusFile {
            change_id: None,
            faces: vec![CensusFace {
                id: "x".into(),
                name: "X".into(),
                face_name: None,
                analyzed_source_sha256: "source".into(),
                checked_readings: count,
                readings_sha256: Some(digest.into()),
                lines: Some(vec![CensusLine {
                    span: (0, 4),
                    checked_readings: count,
                    readings_sha256: Some(digest.into()),
                }]),
            }],
        }
    }

    #[test]
    fn line_diff_reports_counts_digests_and_added_lines() {
        let before = census(2, "a");
        let count_change = compare_lines(&before, &census(3, "b")).unwrap();
        assert_eq!(
            (count_change[0].before, count_change[0].after),
            (Some(2), Some(3))
        );
        let digest_change = compare_lines(&before, &census(2, "b")).unwrap();
        assert_eq!(digest_change.len(), 1);
        assert_eq!(digest_change[0].before_digest.as_deref(), Some("a"));
        assert_eq!(digest_change[0].after_digest.as_deref(), Some("b"));
        assert!(compare_lines(&before, &census(2, "a")).unwrap().is_empty());
        let mut after = census(2, "a");
        after.faces[0].lines.as_mut().unwrap().push(CensusLine {
            span: (5, 9),
            checked_readings: 1,
            readings_sha256: Some("c".into()),
        });
        let added = compare_lines(&before, &after).unwrap();
        assert_eq!(
            (added[0].line_index, added[0].before, added[0].after),
            (1, None, Some(1))
        );
    }

    #[test]
    fn line_diff_rejects_a_report_without_line_arrays() {
        let mut before = census(2, "a");
        before.faces[0].lines = None;
        let after = census(2, "b");
        assert!(compare_lines(&before, &after).is_err());
        assert!(compare_lines(&after, &before).is_err());
    }
}

#[cfg(test)]
mod version_tests {
    use super::*;

    #[test]
    fn comparisons_require_the_new_baseline_and_all_matching_versions() {
        let current = serde_json::json!({"schema_version":super::super::report::SCHEMA_VERSION,
            "readings_digest_version":super::super::report::READINGS_DIGEST_VERSION,
            "decomposition_version":super::super::report::DECOMPOSITION_VERSION,
            "context_version":super::super::report::CONTEXT_VERSION});
        let file = tempfile::NamedTempFile::new().unwrap();
        std::fs::write(file.path(), serde_json::to_vec(&current).unwrap()).unwrap();
        require_current_versions(file.path()).unwrap();
        for field in [
            "schema_version",
            "readings_digest_version",
            "decomposition_version",
            "context_version",
        ] {
            let mut incompatible = current.clone();
            incompatible[field] = 0.into();
            std::fs::write(file.path(), serde_json::to_vec(&incompatible).unwrap()).unwrap();
            assert!(require_current_versions(file.path()).is_err(), "{field}");
        }
    }
}
