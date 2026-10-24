//! `cargo xtask english-v4`: a thin CLI over `deckmaste_english_v4`'s corpus,
//! stratum profiler and census.

use std::collections::BTreeMap;
use std::fs::File;
use std::io::BufReader;
use std::io::Write;
use std::path::PathBuf;

use anyhow::Context;
use anyhow::Result;
use clap::Args;
use clap::Subcommand;
use deckmaste_english_v4::census;
use deckmaste_english_v4::corpus::Corpus;
use deckmaste_lexical_source_v4::LexicalEnvironment;

#[derive(Debug, Args)]
pub struct EnglishV4Args {
    #[command(subcommand)]
    command: EnglishV4Command,
}

#[derive(Debug, Subcommand)]
enum EnglishV4Command {
    /// Profile every Corpus Unit into its Complexity Stratum and run the
    /// census: read, known-wrong-only and unread per Stratum.
    Strata(StrataArgs),
    /// Inspect judgments and their cheapest Readings.
    Parse(ParseArgs),
}

#[derive(Debug, Args)]
struct StrataArgs {
    /// The Oracle Cards snapshot.
    #[arg(long, default_value = "data/scryfall/oracle-cards.jsonl")]
    data: PathBuf,
    /// Only faces of cards with this name (repeatable).
    #[arg(long = "card")]
    cards: Vec<String>,
    /// A v3 census report (`cargo xtask english-v3 --output`) whose unread
    /// faces are accounted for by Stratum.
    #[arg(long)]
    baseline: Option<PathBuf>,
    /// Where to write the full report as JSON.
    #[arg(long, default_value = "target/english-v4/strata.json")]
    output: PathBuf,
    /// Skeletons listed per Stratum in the summary.
    #[arg(long, default_value_t = 15)]
    top: usize,
    /// Fail when a unit at or below the completed Stratum is neither read nor
    /// deferred, or when a deferred unit is read.
    #[arg(long)]
    check: bool,
}

/// The fields of a v3 census report the baseline needs.
#[derive(serde::Deserialize)]
struct V3Report {
    faces: Vec<V3Face>,
}

#[derive(serde::Deserialize)]
struct V3Face {
    id: String,
    census: String,
}

pub fn run(args: &EnglishV4Args, output: &mut dyn Write) -> Result<()> {
    match &args.command {
        EnglishV4Command::Strata(args) => strata(args, output),
        EnglishV4Command::Parse(args) => parse(args, output),
    }
}

fn strata(args: &StrataArgs, output: &mut dyn Write) -> Result<()> {
    let env = LexicalEnvironment::load().context("loading the v4 lexical environment")?;
    let reader = BufReader::new(
        File::open(&args.data).with_context(|| format!("opening {}", args.data.display()))?,
    );
    let corpus = Corpus::read(reader, |face| {
        args.cards.is_empty()
            || args
                .cards
                .iter()
                .any(|c| *c == face.card || *c == face.name)
    })?;
    let judgments: Vec<_> =
        deckmaste_english_v4::judgment::load_dir(&deckmaste_english_v4::judgments_dir())?
            .into_iter()
            .filter(|(name, _)| name != "seed.txt")
            .flat_map(|(_, entries)| entries)
            .collect();
    let baseline = args
        .baseline
        .as_ref()
        .map(|path| -> Result<BTreeMap<String, bool>> {
            let file = BufReader::new(
                File::open(path).with_context(|| format!("opening {}", path.display()))?,
            );
            let report: V3Report = serde_json::from_reader(file)
                .with_context(|| format!("reading {}", path.display()))?;
            Ok(report
                .faces
                .into_iter()
                .map(|f| (f.id, f.census != "no"))
                .collect())
        })
        .transpose()?;
    let report = census::run(&corpus, &env, &judgments, baseline.as_ref());
    if let Some(parent) = args.output.parent() {
        std::fs::create_dir_all(parent)?;
    }
    serde_json::to_writer(File::create(&args.output)?, &report)?;
    write!(output, "{}", report.summary(args.top))?;
    if !corpus.stale_corrections.is_empty() {
        writeln!(
            output,
            "\nstale editorial corrections (no face's Oracle text matches):"
        )?;
        for correction in &corpus.stale_corrections {
            writeln!(output, "  {}", correction.describe())?;
        }
    }
    writeln!(output, "report: {}", args.output.display())?;
    if args.check && !report.regressions.is_empty() {
        anyhow::bail!(
            "{} units at or below the completed stratum are not read",
            report.regressions.len()
        );
    }
    if args.check && !report.not_segmentable.is_empty() {
        anyhow::bail!(
            "{} units have no single product segmentation",
            report.not_segmentable.len()
        );
    }
    if args.check && !corpus.stale_corrections.is_empty() {
        anyhow::bail!(
            "{} editorial corrections match no face; update or remove them in corrections.txt",
            corpus.stale_corrections.len()
        );
    }
    if args.check && !report.stale_deferrals.is_empty() {
        anyhow::bail!(
            "{} deferred units are read; remove them from deferred.txt or declined.txt",
            report.stale_deferrals.len()
        );
    }
    Ok(())
}

#[derive(Debug, Args)]
struct ParseArgs {
    /// Judgment entries or bare Oracle text; quote each input separately.
    #[arg(required = true, num_args = 1..)]
    inputs: Vec<String>,
    /// Maximum number of cheapest Readings to display per input.
    #[arg(long, default_value_t = 8)]
    limit: usize,
}

fn parse(args: &ParseArgs, output: &mut dyn Write) -> Result<()> {
    use deckmaste_english_v4::corpus::UnitKind;
    use deckmaste_english_v4::judgment;
    use deckmaste_english_v4::judgment::Judge;
    use deckmaste_english_v4::profile::Profiler;

    let entries = args
        .inputs
        .iter()
        .enumerate()
        .map(|(index, input)| {
            let input = input.trim();
            anyhow::ensure!(!input.is_empty(), "input {} is empty", index + 1);
            let marked = input.starts_with(['+', '*']);
            let normalized = if marked { input.to_owned() } else { format!("+ {input}") };
            let mut entries = judgment::parse_file(&format!("input {}", index + 1), &normalized)?;
            anyhow::ensure!(
                entries.len() == 1,
                "each input must contain one judgment entry"
            );
            Ok((entries.remove(0), marked || input.contains(" => ")))
        })
        .collect::<Result<Vec<_>>>()?;
    let env = LexicalEnvironment::load().context("loading the v4 lexical environment")?;
    let grammar = deckmaste_english_v4::bind(&env.lexicon, &env.frames)?;
    let frame = deckmaste_english_v4::segments::segment_frame(&grammar)?;
    let judge = Judge {
        grammar: &grammar,
        lexicon: &env.lexicon,
    };
    let profiler = Profiler::new(&env);
    for (entry, evaluate) in entries {
        writeln!(output, "{}: {:?}", entry.location, entry.text)?;
        let text = deckmaste_data::oracle_text::strip_reminder_text(&entry.text);
        let analyzed = env
            .lexicon
            .analyze_source_scoped(&text, None, &judgment::scope(&entry));
        let kind = if entry.category.as_deref() == Some("TypeLine") {
            UnitKind::TypeLine
        } else {
            UnitKind::Line
        };
        let profile = profiler.profile(kind, &analyzed);
        writeln!(output, "Stratum: {}", profile.stratum)?;
        writeln!(
            output,
            "Syntactic Families: {}",
            profile
                .families
                .iter()
                .map(|f| f.name)
                .collect::<Vec<_>>()
                .join(", ")
        )?;
        if evaluate {
            writeln!(output, "Outcome: {}", judge.evaluate(&entry))?;
        }
        let Some(parse) = judge.parse(&entry) else {
            writeln!(
                output,
                "Unknown Category: {}",
                entry.category.as_deref().unwrap_or_default()
            )?;
            continue;
        };
        writeln!(output, "Vocabulary gaps: {:?}", parse.vocabulary_gaps)?;
        let derivations = parse.derivation_count();
        writeln!(output, "Packed Forest derivations: {derivations}")?;
        let segmentation = deckmaste_english_v4::segments::segments(&parse, &frame);
        let segments = match &segmentation {
            deckmaste_english_v4::segments::Segmentation::Segments(segments) => segments
                .iter()
                .map(|s| format!("{} {:?}", s.derivations, s.text))
                .collect::<Vec<_>>()
                .join(" | "),
            deckmaste_english_v4::segments::Segmentation::NotSegmentable(why) => {
                format!("not segmentable: {why:?}")
            }
        };
        writeln!(output, "Segments: {segments}")?;
        writeln!(output, "Surplus: {}", segmentation.surplus(derivations))?;
        match parse.cheapest_tier() {
            Some((key, tied)) => writeln!(
                output,
                "Cheapest tier: {tied} at ranking key {}",
                format_key(key)
            )?,
            None => writeln!(output, "Cheapest tier: none")?,
        }
        for (index, (key, reading)) in parse.best_ranked(args.limit).iter().enumerate() {
            parse.admit(reading, &env.lexicon)?;
            dump_reading(&grammar, &env, *key, reading, index + 1, output)?;
        }
        writeln!(output)?;
    }
    Ok(())
}

fn dump_reading(
    grammar: &deckmaste_syntax::CompiledGrammar,
    env: &LexicalEnvironment,
    key: deckmaste_syntax::RankingKey,
    reading: &deckmaste_syntax::Reading,
    number: usize,
    output: &mut dyn Write,
) -> Result<()> {
    let (text, constituents) = grammar.constituents(reading, &env.lexicon)?;
    writeln!(
        output,
        "Reading {} (ranking key {}): {text:?}",
        number,
        format_key(key)
    )?;
    let mut nodes = Vec::new();
    preorder_nodes(reading, &mut nodes);
    writeln!(
        output,
        "node\tparent\tProduction\tCategory\tspan (bytes)\tsurface\tfunction\thead lemma / lexeme / form\tGap"
    )?;
    for (index, (c, node)) in constituents.iter().zip(nodes).enumerate() {
        let production = node.node().map_or("", |n| n.production.as_str());
        let head = grammar.head_word(node).map_or_else(
            || "-".to_owned(),
            |word| {
                let lemma = env
                    .lexicon
                    .lexemes()
                    .get(&word.lexeme)
                    .map_or("?", |l| l.lemma.as_str());
                format!("{lemma} / {} / {:?}", word.lexeme.as_str(), word.form)
            },
        );
        writeln!(
            output,
            "{index}\t{:?}\t{production}\t{}\t{:?}\t{:?}\t{}\t{head}\t{:?}",
            c.parent,
            c.category,
            c.span,
            &text[c.span.clone()],
            c.function.map_or("", |function| function.as_str()),
            c.gap
        )?;
    }
    Ok(())
}

/// A Ranking Key as `class/association/cost`, its components in comparison
/// order.
fn format_key(key: deckmaste_syntax::RankingKey) -> String {
    format!(
        "{}/{}/{}",
        key.attachment_class, key.right_association_words, key.production_cost
    )
}

/// A Reading's nodes in preorder, as [`CompiledGrammar::constituents`]
/// lists their Constituents.
fn preorder_nodes<'a>(
    reading: &'a deckmaste_syntax::Reading,
    out: &mut Vec<&'a deckmaste_syntax::Reading>,
) {
    if let Some(node) = reading.node() {
        out.push(reading);
        for child in &node.children {
            preorder_nodes(child, out);
        }
    }
}

#[cfg(test)]
mod tests {
    use clap::Parser;

    use super::*;

    #[derive(Parser)]
    struct Cli {
        #[command(flatten)]
        args: EnglishV4Args,
    }

    fn dump(inputs: &[&str]) -> String {
        let cli = Cli::try_parse_from(std::iter::once("english-v4").chain(inputs.iter().copied()))
            .unwrap();
        let mut output = Vec::new();
        run(&cli.args, &mut output).unwrap();
        String::from_utf8(output).unwrap()
    }

    #[test]
    fn parse_dumps_structure_and_evaluates_all_inputs_with_a_display_limit() {
        let output = dump(&[
            "parse",
            "--limit",
            "1",
            "Destroy target creature.",
            "+NounPhrase target creature => NounPhrase \"target creature\" head creature",
            "*PrepositionPhrase target creature",
            "+ Destroy target creature. => NounPhrase \"target creature\" as Subject of destroy",
            "+ <card=Seedborn Muse> Untap all permanents you control.",
        ]);
        assert_eq!(output.matches("Stratum:").count(), 5);
        assert_eq!(output.matches("Syntactic Families:").count(), 5);
        assert_eq!(output.matches("Packed Forest derivations:").count(), 5);
        assert_eq!(output.matches("Segments:").count(), 5);
        assert_eq!(output.matches("Surplus:").count(), 5);
        assert_eq!(output.matches("Cheapest tier: ").count(), 5);
        assert!(
            output.contains("Segments: 1 \"Destroy target creature\"\nSurplus: 0"),
            "{output}"
        );
        assert_eq!(output.matches("Outcome:").count(), 4);
        assert!(output.contains("Outcome: holds"));
        assert!(output.contains("Outcome: no Reading satisfies"));
        assert!(output.contains("Reading 1 (ranking key"));
        assert!(!output.contains("Reading 2 (ranking key"));
        assert!(output.contains("Production\tCategory\tspan (bytes)"));
        assert!(output.contains("\"target creature\"\tobject\tcreature /"));
        assert!(
            output.contains("destroy / action:destroy / Plain"),
            "{output}"
        );
    }

    #[test]
    fn parse_reports_gaps_and_unknown_categories_without_aborting_other_inputs() {
        let output = dump(&[
            "parse",
            "--limit",
            "0",
            "+ Destroy target zzzunknownword.",
            "+MissingCategory target creature",
            "*PrepositionPhrase target creature",
        ]);
        assert!(output.contains("Outcome: vocabulary gaps"));
        assert!(output.contains("Unknown Category: MissingCategory"));
        assert!(output.contains("Outcome: holds"));
        assert!(!output.contains("Reading 1 (ranking key"));
    }

    #[test]
    fn parse_preserves_card_scope_and_evaluates_beyond_the_display_limit() {
        let output = dump(&[
            "parse",
            "--limit",
            "0",
            "+ <card=Jace, Cunning Castaway> Jace draws two cards. => NounPhrase \"Jace\" as Subject of draw",
            "* Jace draws two cards.",
            "Destroy target creature. => NounPhrase \"target creature\" as Object of destroy",
        ]);
        assert_eq!(output.matches("Outcome: holds").count(), 3, "{output}");
        assert!(!output.contains("Reading 1 (ranking key"));
    }

    #[test]
    fn parse_rejects_missing_inputs_bad_limits_and_malformed_entries() {
        for args in [
            vec!["english-v4", "parse"],
            vec!["english-v4", "parse", "--limit", "no", "text"],
        ] {
            assert!(Cli::try_parse_from(args).is_err());
        }
        for input in [
            "",
            "+ <card=Unclosed text",
            "+ Destroy target creature. => nonsense",
            "+ Flying\n+ Haste",
        ] {
            let args = ParseArgs {
                inputs: vec![input.to_owned()],
                limit: 8,
            };
            assert!(parse(&args, &mut Vec::new()).is_err(), "{input}");
        }
    }
}
