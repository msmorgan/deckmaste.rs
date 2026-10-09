//! Regex search over the supported corpus's analysed text.
//!
//! Searches the faces the census selects, over the parser input the census
//! records (`analyzed_source`), so attestation checks never see unsupported
//! cards or reminder text. A report, never a gate: every outcome exits 0 once
//! the pattern compiles and the inputs load.

use std::collections::BTreeMap;
use std::io::Write;
use std::path::Path;
use std::path::PathBuf;

use anyhow::Context;
use anyhow::Result;
use anyhow::ensure;
use clap::Args;
use regex::Regex;
use regex::RegexBuilder;
use serde::Serialize;

use super::SourceField;
use super::report::analyzed_source;
use crate::raw_corpus::SelectedFace;
use crate::raw_corpus::SelectionRequest;
use crate::raw_corpus::load_selected;

/// Characters of context printed on each side of a match.
const CONTEXT_CHARS: usize = 40;

#[derive(Debug, Args)]
pub struct GrepArgs {
    /// Rust `regex` pattern, case-sensitive unless `-i` is given.
    pattern: String,
    /// Match case-insensitively.
    #[arg(short = 'i', long)]
    ignore_case: bool,
    #[command(flatten)]
    output: OutputArgs,
    /// Face field to search, as the census analyses it.
    #[arg(long, value_enum, default_value = "text")]
    field: SourceField,
    #[command(flatten)]
    restriction: RestrictionArgs,
}

#[derive(Debug, Args)]
struct OutputArgs {
    /// Print every match of each face, not only the first.
    #[arg(long)]
    all_matches: bool,
    /// Print only the final `N faces, M matches` line.
    #[arg(long, conflicts_with = "names")]
    count: bool,
    /// Print only matching face names (card name / face name), sorted.
    #[arg(long)]
    names: bool,
    /// Maximum face lines printed; `--json` keeps every face.
    #[arg(long, value_name = "N", default_value_t = 200)]
    limit: usize,
    /// Write every matching face and match as JSON.
    #[arg(long, value_name = "OUT")]
    json: Option<PathBuf>,
}

#[derive(Debug, Args)]
struct RestrictionArgs {
    /// Census report restricting the search; requires `--unread` or `--covered`.
    #[arg(long, value_name = "FILE")]
    census: Option<PathBuf>,
    /// Only faces with no Reading in `--census`.
    #[arg(long, requires = "census", conflicts_with = "covered")]
    unread: bool,
    /// Only faces with at least one Reading in `--census`.
    #[arg(long, requires = "census")]
    covered: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum CensusFilter {
    Unread,
    Covered,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
struct Match {
    /// Byte offsets into the analysed source.
    start: usize,
    end: usize,
    text: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
struct FaceHit {
    identity: String,
    name: String,
    face_name: Option<String>,
    matches: Vec<Match>,
    /// Context lines, rendered from the analysed source.
    #[serde(skip)]
    contexts: Vec<String>,
}

impl FaceHit {
    /// `Card Name / face name`, or the card name for a single-face card.
    fn label(&self) -> String {
        match &self.face_name {
            Some(face) if *face != self.name => format!("{} / {face}", self.name),
            _ => self.name.clone(),
        }
    }
}

/// Matching faces in corpus order. `readings` restricts to faces present in
/// that census whose Reading count satisfies `filter`.
fn search(
    faces: &[SelectedFace],
    field: SourceField,
    regex: &Regex,
    restriction: Option<(&BTreeMap<String, usize>, CensusFilter)>,
) -> Vec<FaceHit> {
    faces
        .iter()
        .filter(|face| {
            restriction.is_none_or(|(readings, filter)| {
                readings
                    .get(&face.identity)
                    .is_some_and(|&count| match filter {
                        CensusFilter::Unread => count == 0,
                        CensusFilter::Covered => count > 0,
                    })
            })
        })
        .filter_map(|face| {
            let source = analyzed_source(field.source(face).unwrap_or(""), field);
            let (matches, contexts): (Vec<_>, Vec<_>) = regex
                .find_iter(&source)
                .map(|found| {
                    (
                        Match {
                            start: found.start(),
                            end: found.end(),
                            text: found.as_str().to_owned(),
                        },
                        context(&source, found.start(), found.end()),
                    )
                })
                .unzip();
            (!matches.is_empty()).then(|| FaceHit {
                identity: face.identity.clone(),
                name: face.group_name.clone(),
                face_name: face.face_name.clone(),
                matches,
                contexts,
            })
        })
        .collect()
}

/// Up to `CONTEXT_CHARS` characters either side of the match, with the match
/// in `«»`, line breaks as `⏎` and `…` where the source continues.
fn context(source: &str, start: usize, end: usize) -> String {
    let before = source[..start]
        .char_indices()
        .rev()
        .nth(CONTEXT_CHARS - 1)
        .map_or(0, |(index, _)| index);
    let after = source[end..]
        .char_indices()
        .nth(CONTEXT_CHARS)
        .map_or(source.len(), |(index, _)| end + index);
    format!(
        "{}{}«{}»{}{}",
        if before > 0 { "…" } else { "" },
        &source[before..start],
        &source[start..end],
        &source[end..after],
        if after < source.len() { "…" } else { "" },
    )
    .replace('\n', "⏎")
}

fn write_text(hits: &[FaceHit], args: &OutputArgs, out: &mut dyn Write) -> std::io::Result<()> {
    let total: usize = hits.iter().map(|hit| hit.matches.len()).sum();
    if args.names {
        let mut labels: Vec<String> = hits.iter().map(FaceHit::label).collect();
        labels.sort();
        labels.dedup();
        for label in &labels {
            writeln!(out, "{label}")?;
        }
        return Ok(());
    }
    if !args.count {
        for hit in hits.iter().take(args.limit) {
            let shown = if args.all_matches { hit.contexts.len() } else { 1 };
            let label = match &hit.face_name {
                Some(face) if *face != hit.name => format!("{} [{face}]", hit.name),
                _ => hit.name.clone(),
            };
            for context in &hit.contexts[..shown] {
                writeln!(out, "{label} | {context}")?;
            }
        }
        if hits.len() > args.limit {
            writeln!(out, "… and {} more", hits.len() - args.limit)?;
        }
    }
    writeln!(out, "{} faces, {total} matches", hits.len())
}

fn census_readings(path: &Path) -> Result<BTreeMap<String, usize>> {
    Ok(super::diff::load(path)?
        .faces
        .into_iter()
        .map(|face| (face.id, face.checked_readings))
        .collect())
}

pub(super) fn run(args: &GrepArgs, data: &Path, out: &mut dyn Write) -> Result<()> {
    let regex = RegexBuilder::new(&args.pattern)
        .case_insensitive(args.ignore_case)
        .build()
        .with_context(|| format!("invalid pattern {:?}", args.pattern))?;
    let restriction = &args.restriction;
    let filter = match (restriction.unread, restriction.covered) {
        (true, _) => Some(CensusFilter::Unread),
        (_, true) => Some(CensusFilter::Covered),
        _ => None,
    };
    ensure!(
        restriction.census.is_none() || filter.is_some(),
        "--census requires --unread or --covered"
    );
    let readings = restriction
        .census
        .as_deref()
        .map(census_readings)
        .transpose()?;
    let corpus = load_selected(
        data,
        &SelectionRequest {
            all: true,
            ..SelectionRequest::default()
        },
        None,
    )?;
    let hits = search(
        &corpus.faces,
        args.field,
        &regex,
        readings.as_ref().zip(filter),
    );
    if let Some(path) = &args.output.json {
        let file =
            std::fs::File::create(path).with_context(|| format!("creating {}", path.display()))?;
        serde_json::to_writer_pretty(std::io::BufWriter::new(file), &hits)
            .with_context(|| format!("writing {}", path.display()))?;
    }
    write_text(&hits, &args.output, out)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use clap::Parser;
    use serde_json::json;

    use super::*;

    #[derive(Parser)]
    struct Cli {
        #[command(flatten)]
        args: GrepArgs,
    }

    fn args(argv: &[&str]) -> GrepArgs {
        Cli::parse_from(std::iter::once("grep").chain(argv.iter().copied())).args
    }

    /// Scryfall records. Abundant Harvest, Bog Raiders and Fire // Ice carry
    /// their Oracle text; the two fixtures exist only to fail the selection.
    fn corpus() -> Vec<SelectedFace> {
        let record = |name: &str, text: &str, vintage: &str, set_type: &str| {
            json!({"object": "card", "id": format!("printing-{name}"),
                "oracle_id": format!("oracle-{name}"), "name": name, "layout": "normal",
                "legalities": {"vintage": vintage}, "set_type": set_type,
                "type_line": "Sorcery", "oracle_text": text, "color_identity": []})
        };
        let records = [
            record(
                "Abundant Harvest",
                "Choose land or nonland. Reveal cards from the top of your library until you reveal a card of the chosen kind. Put that card into your hand and the rest on the bottom of your library in a random order.",
                "legal",
                "expansion",
            ),
            record(
                "Bog Raiders",
                "Swampwalk (This creature can't be blocked as long as defending player controls a Swamp.)",
                "legal",
                "expansion",
            ),
            record(
                "Banned Fixture",
                "Destroy target nonland permanent.",
                "banned",
                "expansion",
            ),
            record(
                "Funny Fixture",
                "Destroy target nonland permanent.",
                "legal",
                "funny",
            ),
            json!({"object": "card", "id": "printing-split", "oracle_id": "oracle-split",
                "name": "Fire // Ice", "layout": "split", "legalities": {"vintage": "legal"},
                "set_type": "expansion", "color_identity": [], "card_faces": [
                    {"name": "Fire", "type_line": "Instant",
                     "oracle_text": "Fire deals 2 damage divided as you choose among one or two targets."},
                    {"name": "Ice", "type_line": "Instant",
                     "oracle_text": "Tap target permanent.\nDraw a card."}]}),
        ];
        let mut input = tempfile::NamedTempFile::new().unwrap();
        for record in records {
            writeln!(input, "{record}").unwrap();
        }
        load_selected(
            input.path(),
            &SelectionRequest {
                all: true,
                ..SelectionRequest::default()
            },
            None,
        )
        .unwrap()
        .faces
    }

    fn names(hits: &[FaceHit]) -> Vec<String> {
        hits.iter().map(FaceHit::label).collect()
    }

    fn render(hits: &[FaceHit], argv: &[&str]) -> String {
        let mut out = Vec::new();
        write_text(hits, &args(argv).output, &mut out).unwrap();
        String::from_utf8(out).unwrap()
    }

    #[test]
    fn unsupported_and_funny_cards_are_not_searched() {
        let regex = Regex::new("nonland").unwrap();
        let hits = search(&corpus(), SourceField::Text, &regex, None);
        assert_eq!(names(&hits), ["Abundant Harvest"]);
    }

    #[test]
    fn reminder_text_is_not_searched() {
        let faces = corpus();
        let reminder = Regex::new("defending player").unwrap();
        assert_eq!(search(&faces, SourceField::Text, &reminder, None), []);
        let keyword = Regex::new("Swampwalk").unwrap();
        let hits = search(&faces, SourceField::Text, &keyword, None);
        assert_eq!(names(&hits), ["Bog Raiders"]);
        assert_eq!(hits[0].contexts, ["«Swampwalk»"]);
    }

    #[test]
    fn type_line_field_searches_type_lines() {
        let regex = Regex::new("^Instant$").unwrap();
        let hits = search(&corpus(), SourceField::TypeLine, &regex, None);
        assert_eq!(names(&hits), ["Fire // Ice / Fire", "Fire // Ice / Ice"]);
    }

    #[test]
    fn census_filter_selects_unread_or_covered_faces() {
        let faces = corpus();
        let readings: BTreeMap<String, usize> = faces
            .iter()
            .map(|face| {
                let count = usize::from(face.group_name == "Abundant Harvest");
                (face.identity.clone(), count)
            })
            .collect();
        let regex = Regex::new("(?i)card").unwrap();
        let unread = search(
            &faces,
            SourceField::Text,
            &regex,
            Some((&readings, CensusFilter::Unread)),
        );
        assert_eq!(names(&unread), ["Fire // Ice / Ice"]);
        let covered = search(
            &faces,
            SourceField::Text,
            &regex,
            Some((&readings, CensusFilter::Covered)),
        );
        assert_eq!(names(&covered), ["Abundant Harvest"]);
        // Faces absent from the census satisfy neither filter.
        let empty = BTreeMap::new();
        assert_eq!(
            search(
                &faces,
                SourceField::Text,
                &regex,
                Some((&empty, CensusFilter::Unread))
            ),
            []
        );
    }

    #[test]
    fn output_modes_print_lines_count_and_names() {
        let regex = Regex::new("card").unwrap();
        let hits = search(&corpus(), SourceField::Text, &regex, None);
        assert_eq!(
            render(&hits, &["card"]),
            "Abundant Harvest | Choose land or nonland. Reveal «card»s from the top of your library until you…\n\
             Fire // Ice [Ice] | Tap target permanent.⏎Draw a «card».\n\
             2 faces, 4 matches\n"
        );
        assert_eq!(
            render(&hits, &["card", "--all-matches", "--limit", "1"]),
            "Abundant Harvest | Choose land or nonland. Reveal «card»s from the top of your library until you…\n\
             Abundant Harvest | … top of your library until you reveal a «card» of the chosen kind. Put that card into …\n\
             Abundant Harvest | …eal a card of the chosen kind. Put that «card» into your hand and the rest on the bott…\n\
             … and 1 more\n\
             2 faces, 4 matches\n"
        );
        assert_eq!(render(&hits, &["card", "--count"]), "2 faces, 4 matches\n");
        assert_eq!(
            render(&hits, &["card", "--names"]),
            "Abundant Harvest\nFire // Ice / Ice\n"
        );
    }

    #[test]
    fn census_flags_require_a_census() {
        let parse = |argv: &[&str]| {
            Cli::try_parse_from(std::iter::once("grep").chain(argv.iter().copied())).map(|_| ())
        };
        assert!(parse(&["x", "--unread"]).is_err());
        assert!(parse(&["x", "--census", "c.json", "--unread", "--covered"]).is_err());
        assert!(parse(&["x", "--census", "c.json", "--covered"]).is_ok());
    }
}
