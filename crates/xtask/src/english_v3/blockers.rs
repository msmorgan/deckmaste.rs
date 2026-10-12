//! Diagnostic line verdicts and blocker rankings, independent of admission.

use std::collections::BTreeMap;
use std::collections::BTreeSet;
use std::io::BufReader;
use std::io::Write;
use std::num::NonZeroUsize;
use std::path::Path;
use std::path::PathBuf;

use anyhow::Context;
use anyhow::Result;
use anyhow::ensure;
use clap::Args;
use deckmaste_english_v3::grammar::Grammar;
use deckmaste_lexical::Lexicon;
use rayon::prelude::*;
use serde::Deserialize;
use serde::Serialize;

use super::EnglishV3Args;
use super::SourceField;
use super::analyze_face_with_progress;
use super::diff::CensusFace;
use super::report::Enumeration;
use super::report::UnknownWord;
use crate::raw_corpus::CorpusSelectionArgs;
use crate::raw_corpus::SelectedFace;
use crate::raw_corpus::SelectionRequest;
use crate::raw_corpus::digest;
use crate::raw_corpus::load_selected;

#[derive(Debug, Args)]
pub struct BlockersArgs {
    /// Complete schema-9 text census made with the current grammar and lexicon.
    #[arg(long)]
    census: PathBuf,
    /// Write every face verdict and the uncapped ranking as JSON.
    #[arg(long, value_name = "OUT")]
    json: Option<PathBuf>,
    /// Override the printed limit for each of the three ranked tables.
    #[arg(long)]
    limit: Option<usize>,
}

#[derive(Deserialize)]
struct Input {
    schema_version: usize,
    field: SourceField,
    workers: NonZeroUsize,
    change_id: Option<String>,
    faces: Vec<InputFace>,
}

#[derive(Deserialize)]
struct InputFace {
    #[serde(flatten)]
    census: CensusFace,
    analyzed_source: String,
    ability_spans: Vec<(usize, usize)>,
    enumeration: Enumeration,
}

#[derive(Debug, Serialize)]
struct Position {
    token_index: usize,
    previous_token: String,
    failing_token: String,
}

#[derive(Debug, Serialize)]
struct ContextTokens {
    before: Vec<String>,
    after: Vec<String>,
}

#[derive(Debug, Serialize)]
struct Line {
    span: (usize, usize),
    text: String,
    reads: bool,
    reading_count: usize,
    unknown_words: Vec<UnknownWord>,
    position: Option<Position>,
    context: Option<ContextTokens>,
    keys: BTreeSet<String>,
}

#[derive(Debug, Serialize)]
struct Face {
    id: String,
    name: String,
    covered: bool,
    segmentation_disagrees: bool,
    lines: Vec<Line>,
}

#[derive(Debug, Serialize)]
struct RankedKey {
    key: String,
    faces: usize,
    sole_blocker_faces: usize,
    examples: Vec<Example>,
}

#[derive(Debug, Serialize)]
struct Example {
    id: String,
    name: String,
}

#[derive(Debug, Default, Serialize)]
struct Summary {
    unread_faces: usize,
    single_failing_line_faces: usize,
    vocabulary_only_faces: usize,
    reading_lines: usize,
    lines: usize,
    segmentation_disagreements: usize,
}

#[derive(Serialize)]
struct Report {
    census: PathBuf,
    census_change_id: Option<String>,
    workers: NonZeroUsize,
    summary: Summary,
    #[serde(flatten)]
    rankings: Rankings,
    faces: Vec<Face>,
}

fn newline_spans(source: &str) -> Vec<(usize, usize)> {
    let mut start = 0;
    source
        .split('\n')
        .map(|line| {
            let span = (start, start + line.len());
            start += line.len() + 1;
            span
        })
        .collect()
}

struct KeyToken {
    start: usize,
    end: usize,
    text: String,
}

fn key_tokens(text: &str) -> Vec<KeyToken> {
    let chars: Vec<_> = text.chars().collect();
    let mut tokens = Vec::new();
    let mut start = 0;
    let word = |ch: char| ch.is_alphanumeric() || matches!(ch, '\'' | '’' | '-' | '‐' | '‑');
    while start < chars.len() {
        if chars[start].is_whitespace() {
            start += 1;
            continue;
        }
        let mut end = start + 1;
        if chars[start] == '{' {
            if let Some(close) = chars[end..].iter().position(|ch| *ch == '}') {
                end += close + 1;
            }
        } else if word(chars[start]) {
            while end < chars.len() && word(chars[end]) {
                end += 1;
            }
        }
        tokens.push(KeyToken {
            start,
            end,
            text: chars[start..end].iter().collect::<String>().to_lowercase(),
        });
        start = end;
    }
    tokens
}

fn locate(text: &str, furthest: usize) -> (Position, ContextTokens) {
    let mut tokens = key_tokens(text);
    let end = text.chars().count();
    tokens.push(KeyToken {
        start: end,
        end,
        text: "<end>".into(),
    });
    let index = tokens
        .iter()
        .position(|token| token.start >= furthest || token.end > furthest)
        .unwrap_or(tokens.len() - 1);
    let position = Position {
        token_index: furthest,
        previous_token: index
            .checked_sub(1)
            .map_or_else(|| "<start>".into(), |i| tokens[i].text.clone()),
        failing_token: tokens
            .get(index)
            .map_or_else(|| "<end>".into(), |token| token.text.clone()),
    };
    let context = ContextTokens {
        before: tokens[index.saturating_sub(2)..index]
            .iter()
            .map(|token| token.text.clone())
            .collect(),
        after: tokens
            .iter()
            .skip(index + 1)
            .take(2)
            .map(|token| token.text.clone())
            .collect(),
    };
    (position, context)
}

fn inspect_face(
    input: &InputFace,
    face: &SelectedFace,
    lexicon: &Lexicon,
    grammar: &Grammar,
    args: &EnglishV3Args,
) -> Result<Face> {
    ensure!(
        input.enumeration == Enumeration::Complete,
        "incomplete census for {}",
        face.identity
    );
    let source =
        super::report::analyzed_source(face.text.as_deref().unwrap_or(""), SourceField::Text);
    ensure!(
        source == input.analyzed_source
            && digest(source.as_bytes()) == input.census.analyzed_source_sha256,
        "census source differs for {}",
        face.identity
    );
    let covered = input.census.checked_readings > 0;
    let newline = newline_spans(&source);
    // Covered Documents already identify top-level abilities, including quoted
    // multiline abilities and modal lists. Unread Documents have no segments.
    let spans = if covered { input.ability_spans.clone() } else { newline.clone() };
    let segmentation_disagrees = covered && spans != newline;
    let mut lines = Vec::new();
    for (start, end) in spans {
        let text = source.get(start..end).context("invalid census line span")?;
        let mut selected = face.clone();
        selected.text = Some(text.into());
        let (result, furthest) = analyze_face_with_progress(&selected, lexicon, grammar, args)?;
        ensure!(
            result.issues.is_empty() && result.enumeration == Enumeration::Complete,
            "line analysis failed internally for {} at {start}: {:?}",
            face.identity,
            result.issues
        );
        let reads = result.checked_readings > 0;
        let mut keys = BTreeSet::new();
        let (position, context) = if reads {
            (None, None)
        } else {
            let (position, context) = locate(text, furthest);
            if result.unknown_words.is_empty() {
                keys.insert(format!(
                    "at:{} {}",
                    position.previous_token, position.failing_token
                ));
            } else {
                for word in &result.unknown_words {
                    keys.insert(format!("word:{}", word.text.to_lowercase()));
                }
            }
            (Some(position), Some(context))
        };
        lines.push(Line {
            span: (start, end),
            text: text.into(),
            reads,
            reading_count: result.checked_readings,
            unknown_words: result.unknown_words,
            position,
            context,
            keys,
        });
    }
    ensure!(
        lines.iter().all(|line| line.reads) == covered,
        "blockers/census coverage mismatch for {} ({})",
        face.identity,
        face.name
    );
    Ok(Face {
        id: face.identity.clone(),
        name: face.face_name.as_ref().unwrap_or(&face.name).clone(),
        covered,
        segmentation_disagrees,
        lines,
    })
}

#[derive(Default, Serialize)]
struct Rankings {
    word_keys: Vec<RankedKey>,
    at_keys: Vec<RankedKey>,
    failing_tokens: Vec<RankedKey>,
}

fn rank(entries: BTreeMap<String, RankedKey>) -> Vec<RankedKey> {
    let mut ranked: Vec<_> = entries.into_values().collect();
    ranked.sort_by(|a, b| {
        b.sole_blocker_faces
            .cmp(&a.sole_blocker_faces)
            .then_with(|| b.faces.cmp(&a.faces))
            .then_with(|| a.key.cmp(&b.key))
    });
    ranked
}

fn touch(entries: &mut BTreeMap<String, RankedKey>, key: &str, face: &Face, sole: bool) {
    let entry = entries.entry(key.into()).or_insert_with(|| RankedKey {
        key: key.into(),
        faces: 0,
        sole_blocker_faces: 0,
        examples: Vec::new(),
    });
    entry.faces += 1;
    entry.sole_blocker_faces += usize::from(sole);
    if entry.examples.len() < 3 {
        entry.examples.push(Example {
            id: face.id.clone(),
            name: face.name.clone(),
        });
    }
}

fn summarize(faces: &[Face]) -> (Summary, Rankings) {
    let mut summary = Summary::default();
    let mut ranked = BTreeMap::<String, RankedKey>::new();
    let mut failing_tokens = BTreeMap::<String, RankedKey>::new();
    for face in faces {
        summary.lines += face.lines.len();
        summary.reading_lines += face.lines.iter().filter(|line| line.reads).count();
        summary.segmentation_disagreements += usize::from(face.segmentation_disagrees);
        let failed: Vec<_> = face.lines.iter().filter(|line| !line.reads).collect();
        if failed.is_empty() {
            continue;
        }
        summary.unread_faces += 1;
        summary.single_failing_line_faces += usize::from(failed.len() == 1);
        summary.vocabulary_only_faces +=
            usize::from(failed.iter().all(|line| !line.unknown_words.is_empty()));
        let sole = failed.len() == 1 && failed[0].keys.len() == 1;
        let keys: BTreeSet<_> = failed.iter().flat_map(|line| &line.keys).collect();
        for key in keys {
            touch(&mut ranked, key, face, sole);
        }
        let tokens: BTreeSet<_> = failed
            .iter()
            .filter(|line| line.unknown_words.is_empty())
            .filter_map(|line| line.position.as_ref().map(|p| p.failing_token.as_str()))
            .collect();
        for token in tokens {
            touch(&mut failing_tokens, token, face, sole);
        }
    }
    let (words, positions) = ranked
        .into_iter()
        .partition(|(key, _)| key.starts_with("word:"));
    (
        summary,
        Rankings {
            word_keys: rank(words),
            at_keys: rank(positions),
            failing_tokens: rank(failing_tokens),
        },
    )
}

fn write_summary(report: &Report, limit: Option<usize>, out: &mut dyn Write) -> Result<()> {
    let s = &report.summary;
    writeln!(
        out,
        "{} unread faces; {} with exactly one failing line; {} vocabulary-only faces",
        s.unread_faces, s.single_failing_line_faces, s.vocabulary_only_faces
    )?;
    writeln!(
        out,
        "Line coverage: {} / {}; {} covered-face segmentation disagreements",
        s.reading_lines, s.lines, s.segmentation_disagreements
    )?;
    for (title, ranked, default_limit) in [
        ("Vocabulary keys", &report.rankings.word_keys, 60),
        ("Position keys", &report.rankings.at_keys, 80),
        ("Failing tokens", &report.rankings.failing_tokens, 40),
    ] {
        writeln!(
            out,
            "{title} (sole-blocker faces, touched faces; examples):"
        )?;
        for key in ranked.iter().take(limit.unwrap_or(default_limit)) {
            writeln!(
                out,
                "{}: {}, {}; {}",
                key.key,
                key.sole_blocker_faces,
                key.faces,
                key.examples
                    .iter()
                    .map(|e| e.name.as_str())
                    .collect::<Vec<_>>()
                    .join(" | ")
            )?;
        }
    }
    Ok(())
}

pub(super) fn run(args: &BlockersArgs, data: &Path, out: &mut dyn Write) -> Result<()> {
    let input: Input = serde_json::from_reader(BufReader::new(std::fs::File::open(&args.census)?))
        .with_context(|| format!("reading census {}", args.census.display()))?;
    ensure!(
        input.schema_version == super::report::SCHEMA_VERSION && input.field == SourceField::Text,
        "blockers requires a schema-9 text census"
    );
    let corpus = load_selected(
        data,
        &SelectionRequest {
            face_identities: input.faces.iter().map(|f| f.census.id.clone()).collect(),
            ..SelectionRequest::default()
        },
        None,
    )?;
    let selected: BTreeMap<_, _> = corpus
        .faces
        .iter()
        .map(|f| (f.identity.as_str(), f))
        .collect();
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
    let lexicon = Lexicon::new(deckmaste_lexical_source::load_workspace(&root)?.lexemes)?;
    let environment = deckmaste_english_v3::grammar::GrammarEnvironment::new(&lexicon);
    let analysis_args = EnglishV3Args {
        command: None,
        field: SourceField::Text,
        data: data.into(),
        selection: CorpusSelectionArgs::default(),
        output: None,
        incremental_from: None,
        migration_from: None,
        reading_limit: None,
        samples_per_face: 0,
        workers: input.workers,
    };
    let pool = rayon::ThreadPoolBuilder::new()
        .num_threads(input.workers.get())
        .build()?;
    let faces: Vec<_> = pool.install(|| {
        input
            .faces
            .par_iter()
            .map(|face| {
                let selected = selected
                    .get(face.census.id.as_str())
                    .context("census face absent from supported corpus")?;
                inspect_face(
                    face,
                    selected,
                    &lexicon,
                    environment.grammar(),
                    &analysis_args,
                )
            })
            .collect::<Result<_>>()
    })?;
    let (summary, rankings) = summarize(&faces);
    let report = Report {
        census: args.census.clone(),
        census_change_id: input.change_id,
        workers: input.workers,
        summary,
        rankings,
        faces,
    };
    if let Some(path) = &args.json {
        let mut file = std::io::BufWriter::new(std::fs::File::create(path)?);
        serde_json::to_writer(&mut file, &report)?;
        writeln!(file)?;
        file.flush()?;
    }
    write_summary(&report, args.limit, out)
}

#[cfg(test)]
mod tests {
    use super::super::analyze_face;
    use super::super::diff::CensusLine;
    use super::super::tests::args;
    use super::super::tests::lexicon;
    use super::super::tests::selected_corpus;
    use super::*;

    fn inspect(text: &str) -> Face {
        let corpus = selected_corpus(&serde_json::to_vec(&serde_json::json!({
            "data": {"Fixture": [{"layout": "normal", "legalities": {"vintage": "Legal"}, "text": text}]}
        })).unwrap());
        let face = &corpus.faces[0];
        let environment = deckmaste_english_v3::grammar::GrammarEnvironment::new(lexicon());
        let mut args = args(None);
        args.samples_per_face = 0;
        let census = analyze_face(face, lexicon(), environment.grammar(), &args).unwrap();
        let input = InputFace {
            analyzed_source: census.analyzed_source,
            ability_spans: census.ability_spans,
            enumeration: census.enumeration,
            census: CensusFace {
                id: census.id,
                name: census.name,
                face_name: census.face_name,
                analyzed_source_sha256: census.analyzed_source_sha256,
                checked_readings: census.checked_readings,
                readings_sha256: census.readings_sha256,
                lines: Some(
                    census
                        .lines
                        .into_iter()
                        .map(|line| CensusLine {
                            span: line.span,
                            checked_readings: line.checked_readings,
                            readings_sha256: line.readings_sha256,
                        })
                        .collect(),
                ),
            },
        };
        inspect_face(&input, face, lexicon(), environment.grammar(), &args).unwrap()
    }

    #[test]
    fn word_positions_keep_unicode_braces_numerals_and_context() {
        let text = "Élan’s well-known 007 {2/W} next, done.";
        let inside = "Élan’s well-k".chars().count();
        let (position, context) = locate(text, inside);
        assert_eq!(position.previous_token, "élan’s");
        assert_eq!(position.failing_token, "well-known");
        assert_eq!(context.before, ["élan’s"]);
        assert_eq!(context.after, ["007", "{2/w}"]);
        let whitespace = "Élan’s well-known".chars().count();
        let (position, context) = locate(text, whitespace);
        assert_eq!(position.failing_token, "007");
        assert_eq!(context.before, ["élan’s", "well-known"]);
        assert_eq!(context.after, ["{2/w}", "next"]);
        assert_eq!(
            locate(text, "Élan’s well-known 007 {2/".chars().count())
                .0
                .failing_token,
            "{2/w}"
        );
        let (position, context) = locate(text, text.chars().count());
        assert_eq!(position.previous_token, ".");
        assert_eq!(position.failing_token, "<end>");
        assert_eq!(context.before, ["done", "."]);
        assert_eq!(context.after, Vec::<String>::new());
        assert_eq!(locate(text, text.chars().count() - 1).1.after, ["<end>"]);
    }

    #[test]
    fn token_boundaries_select_the_next_token_and_empty_input_ends() {
        let text = "Draw, a card.";
        assert_eq!(locate(text, "Draw".chars().count()).0.failing_token, ",");
        assert_eq!(locate(text, "Draw,".chars().count()).0.failing_token, "a");
        assert_eq!(locate(text, 0).0.previous_token, "<start>");
        assert_eq!(locate("", 0).0.failing_token, "<end>");
    }

    #[test]
    fn failing_token_ranking_groups_previous_tokens_and_deduplicates_faces() {
        let sole = inspect("Draw Draw.");
        let mut multiple = inspect("Draw Draw.\nDraw Draw.");
        multiple.lines[1].position.as_mut().unwrap().previous_token = "other".into();
        multiple.lines[1].keys = BTreeSet::from(["at:other .".into()]);
        let (_, rankings) = summarize(&[sole, multiple]);
        assert_eq!(rankings.at_keys.len(), 2);
        assert_eq!(rankings.failing_tokens.len(), 1);
        let token = &rankings.failing_tokens[0];
        assert_eq!(token.key, ".");
        assert_eq!(token.faces, 2);
        assert_eq!(token.sole_blocker_faces, 1);
    }

    #[test]
    fn unknown_second_line_is_the_sole_blocker() {
        let face = inspect("Draw a card.\nZzyzx.");
        assert!(face.lines[0].reads);
        assert!(face.lines[0].reading_count > 0);
        assert!(!face.lines[1].reads);
        assert_eq!(face.lines[1].keys, BTreeSet::from(["word:zzyzx".into()]));
        let (summary, ranked) = summarize(&[face]);
        assert_eq!(summary.single_failing_line_faces, 1);
        assert_eq!(summary.vocabulary_only_faces, 1);
        assert_eq!(ranked.word_keys[0].sole_blocker_faces, 1);
    }

    #[test]
    fn declared_words_in_wrong_order_record_the_chart_boundary() {
        let face = inspect("Draw Draw.");
        let line = &face.lines[0];
        assert!(!line.reads);
        assert!(line.unknown_words.is_empty());
        let position = line.position.as_ref().unwrap();
        assert_eq!(position.previous_token, "draw");
        assert_eq!(position.failing_token, ".");
        assert_eq!(line.context.as_ref().unwrap().before, ["draw", "draw"]);
        assert_eq!(line.keys, BTreeSet::from(["at:draw .".into()]));
        assert_eq!(summarize(&[face]).1.at_keys[0].sole_blocker_faces, 1);
    }

    #[test]
    fn covered_face_has_no_failing_line() {
        let face = inspect("Flying\nDraw a card.");
        assert!(face.covered);
        assert!(
            face.lines
                .iter()
                .all(|line| line.reads && line.keys.is_empty())
        );
        let (summary, ranked) = summarize(&[face]);
        assert_eq!(summary.unread_faces, 0);
        assert_eq!(summary.reading_lines, summary.lines);
        assert!(
            ranked.word_keys.is_empty()
                && ranked.at_keys.is_empty()
                && ranked.failing_tokens.is_empty()
        );
    }

    #[test]
    fn two_failing_lines_rank_both_keys_without_a_sole_blocker() {
        let face = inspect("Draw a card.\nZzyzx.\nQqxyz.");
        assert!(face.lines[0].reads);
        assert_eq!(face.lines[1].keys, BTreeSet::from(["word:zzyzx".into()]));
        assert_eq!(face.lines[2].keys, BTreeSet::from(["word:qqxyz".into()]));
        let (_, ranked) = summarize(&[face]);
        assert_eq!(ranked.word_keys.len(), 2);
        assert!(
            ranked
                .word_keys
                .iter()
                .all(|key| key.faces == 1 && key.sole_blocker_faces == 0)
        );
    }

    #[test]
    fn repeated_unknown_word_touches_a_face_once() {
        let face = inspect("Zzyzx.\nZzyzx.");
        let (_, ranked) = summarize(&[face]);
        assert_eq!(ranked.word_keys.len(), 1);
        assert_eq!(ranked.word_keys[0].faces, 1);
        assert_eq!(ranked.word_keys[0].sole_blocker_faces, 0);
    }

    #[test]
    fn covered_modal_list_keeps_the_census_segment_and_reports_disagreement() {
        let face = inspect("Choose one —\n• Draw a card.\n• Gain 1 life.");
        assert!(face.covered);
        assert!(face.segmentation_disagrees);
        assert!(face.lines.iter().all(|line| line.reads));
        assert_eq!(summarize(&[face]).0.segmentation_disagreements, 1);
    }
}
