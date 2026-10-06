//! Direct fragment inspection without corpus selection or normalization.

use std::collections::BTreeMap;
use std::io::Write;
use std::path::Path;

use anyhow::{Context, Result, ensure};
use clap::{Args, Subcommand};
use deckmaste_english_v3::Grammar as _;
use deckmaste_english_v3::grammar::{Category, Grammar};
use deckmaste_english_v3::parse;
use deckmaste_lexical::{LexicalMatch, Lexicon};
use serde::Serialize;

use super::report::{UnknownWord, chart_metrics};
use super::validation::{Issue, Tracing, validate};

#[derive(Debug, Subcommand)]
pub enum Command {
    /// Inspect lexical alternatives and retained grammatical readings of a fragment.
    Probe(ProbeArgs),
}

#[derive(Debug, Args)]
pub struct ProbeArgs {
    /// Exact input; no reminder stripping or other normalization is applied.
    #[arg(long)]
    text: String,
    /// Case-insensitive kebab-case grammatical start Category.
    #[arg(long, default_value = "document")]
    category: String,
    /// Maximum distinct readings to inspect; zero only recognizes the fragment.
    #[arg(long, default_value_t = 4)]
    readings: usize,
    /// Include chart counters and a dump of packed nodes and families.
    #[arg(long)]
    packed: bool,
    /// Emit one structured JSON report on stdout.
    #[arg(long)]
    json: bool,
}

fn category_name(category: Category) -> String {
    let mut name = String::new();
    for (index, ch) in format!("{category:?}").chars().enumerate() {
        if index != 0 && ch.is_uppercase() {
            name.push('-');
        }
        name.extend(ch.to_lowercase());
    }
    name
}

fn resolve_category(grammar: &Grammar, name: &str) -> Result<Category> {
    let categories: BTreeMap<_, _> = grammar
        .productions()
        .iter()
        .map(|production| (category_name(production.category), production.category))
        .collect();
    categories
        .get(&name.to_ascii_lowercase())
        .copied()
        .with_context(|| {
            format!(
                "unknown category {name:?}; available: {}",
                categories.keys().cloned().collect::<Vec<_>>().join(", ")
            )
        })
}

#[derive(Serialize)]
struct InspectedReading {
    tree: String,
    realized: Option<String>,
    byte_exact: bool,
}

#[derive(Serialize)]
struct ProbeReport {
    text: String,
    category: String,
    lexical_alternatives: Vec<LexicalMatch>,
    vocabulary_gaps: Vec<UnknownWord>,
    admitted_root_count: usize,
    /// Number inspected, not a complete ambiguity census when the cap is reached.
    reading_count: usize,
    reading_limit: usize,
    enumeration_complete: bool,
    readings: Vec<InspectedReading>,
    issues: Vec<Issue>,
    #[serde(skip_serializing_if = "Option::is_none")]
    metrics: Option<BTreeMap<&'static str, usize>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    packed: Option<String>,
}

pub(super) fn run(args: &ProbeArgs, output: &mut dyn Write) -> Result<()> {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
    let sources = deckmaste_lexical_source::load_workspace(&root)?;
    let lexicon = Lexicon::new(sources.lexemes).context("indexing declared lexical inventory")?;
    let environment = deckmaste_english_v3::grammar::GrammarEnvironment::new(&lexicon);
    let grammar = environment.grammar();
    let category = resolve_category(grammar, &args.category)?;
    inspect(args, &lexicon, grammar, category, output)
}

fn inspect(
    args: &ProbeArgs,
    lexicon: &Lexicon,
    grammar: &Grammar,
    category: Category,
    output: &mut dyn Write,
) -> Result<()> {
    let analyzed = lexicon.analyze(&args.text);
    let vocabulary_gaps = analyzed
        .unknown_words()
        .into_iter()
        .map(|range| {
            let bytes = analyzed
                .byte_range(range.start, range.end)
                .context("invalid lexical coordinates")?;
            Ok(UnknownWord {
                text: args.text[bytes.clone()].into(),
                start: bytes.start,
                end: bytes.end,
            })
        })
        .collect::<Result<Vec<_>>>()?;
    let forest = parse(grammar, lexicon, &analyzed, &category)?;
    let mut report = ProbeReport {
        text: args.text.clone(),
        category: category_name(category),
        lexical_alternatives: analyzed.matches,
        vocabulary_gaps,
        admitted_root_count: forest.roots().len(),
        reading_count: 0,
        reading_limit: args.readings,
        enumeration_complete: forest.roots().is_empty(),
        readings: Vec::new(),
        issues: Vec::new(),
        metrics: args.packed.then(|| chart_metrics(forest.metrics())),
        packed: None,
    };
    if args.packed {
        let mut dump = Vec::new();
        forest.write_packed(&mut dump)?;
        report.packed = Some(String::from_utf8(dump)?);
    }
    let mut readings = forest.readings(Tracing::new(grammar));
    for _ in 0..args.readings {
        let Some(value) = readings.next() else {
            report.enumeration_complete = true;
            break;
        };
        match value {
            Err(error) => report
                .issues
                .push(Issue::Materialization(error.to_string())),
            Ok(value) => {
                let realized = match &*value.value {
                    deckmaste_english_v3::grammar::Value::Reading(reading) => {
                        reading.realize(lexicon).ok()
                    }
                    _ => None,
                };
                if let Err(issue) = validate(&value, &args.text, lexicon, category, grammar) {
                    report.issues.push(issue);
                }
                report.readings.push(InspectedReading {
                    tree: format!("{:#?}", value.value),
                    byte_exact: realized.as_deref() == Some(args.text.as_str()),
                    realized,
                });
            }
        }
    }
    report.reading_count = report.readings.len();
    if args.json {
        serde_json::to_writer_pretty(&mut *output, &report)?;
        writeln!(output)?;
    } else {
        writeln!(
            output,
            "Category: {}\nLexical alternatives:",
            report.category
        )?;
        for alternative in &report.lexical_alternatives {
            writeln!(output, "  {alternative:?}")?;
        }
        for gap in &report.vocabulary_gaps {
            writeln!(
                output,
                "Unknown word {:?} at bytes {}..{}",
                gap.text, gap.start, gap.end
            )?;
        }
        writeln!(
            output,
            "{} admitted roots; {} inspected readings (limit {}; enumeration complete: {})",
            report.admitted_root_count,
            report.reading_count,
            args.readings,
            report.enumeration_complete
        )?;
        for (index, reading) in report.readings.iter().enumerate() {
            writeln!(
                output,
                "Reading {}: byte-exact realization: {}\n{}",
                index + 1,
                reading.byte_exact,
                reading.tree
            )?;
        }
        for issue in &report.issues {
            writeln!(output, "Issue: {}", serde_json::to_string(issue)?)?;
        }
        if let Some(metrics) = &report.metrics {
            writeln!(output, "Chart metrics: {metrics:?}")?;
        }
        if let Some(packed) = &report.packed {
            write!(output, "{packed}")?;
        }
    }
    ensure!(
        report.issues.is_empty(),
        "probe detected {} internal or validation issues",
        report.issues.len()
    );
    Ok(())
}

#[cfg(test)]
mod tests {
    use clap::Parser;

    use super::*;
    use crate::english_v3::EnglishV3Args;

    #[derive(Parser)]
    struct Cli {
        #[command(flatten)]
        args: EnglishV3Args,
    }

    #[test]
    fn probe_needs_no_corpus_output_and_accepts_zero_readings() {
        let cli = Cli::try_parse_from([
            "english-v3",
            "probe",
            "--text",
            "Draw a card.",
            "--category",
            "SENTENCE",
            "--readings",
            "0",
            "--json",
            "--packed",
        ])
        .unwrap();
        let Some(Command::Probe(args)) = cli.args.command else {
            panic!("probe command")
        };
        assert_eq!(args.readings, 0);
        assert_eq!(
            resolve_category(&Grammar::default(), &args.category).unwrap(),
            Category::Sentence
        );
        assert!(Cli::try_parse_from(["english-v3", "--all"]).is_err());
        assert!(Cli::try_parse_from(["english-v3", "probe"]).is_err());
        assert!(resolve_category(&Grammar::default(), "not-a-category").is_err());
        assert_eq!(
            resolve_category(&Grammar::default(), "FINITE-PREDICATE").unwrap(),
            Category::FinitePredicate
        );
    }

    #[test]
    fn fragment_reports_validate_readings_and_recognition_only() {
        let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
        let lexicon = Lexicon::new(
            deckmaste_lexical_source::load_workspace(&root)
                .unwrap()
                .lexemes,
        )
        .unwrap();
        let grammar = Grammar::with_lexicon(&lexicon);
        for (text, category) in [
            ("Draw a card.", Category::Sentence),
            ("target tapped creature", Category::NounPhrase),
        ] {
            let mut args = ProbeArgs {
                text: text.into(),
                category: category_name(category),
                readings: 4,
                packed: true,
                json: true,
            };
            let mut output = Vec::new();
            inspect(&args, &lexicon, &grammar, category, &mut output).unwrap();
            let report: serde_json::Value = serde_json::from_slice(&output).unwrap();
            assert!(report["admitted_root_count"].as_u64().unwrap() > 0);
            assert!(report["reading_count"].as_u64().unwrap() > 0);
            assert_eq!(
                report["issues"].as_array().unwrap().as_slice(),
                [] as [serde_json::Value; 0]
            );
            for reading in report["readings"].as_array().unwrap() {
                assert_eq!(reading["realized"], text);
                assert_eq!(reading["byte_exact"], true);
            }
            args.readings = 0;
            output.clear();
            inspect(&args, &lexicon, &grammar, category, &mut output).unwrap();
            let report: serde_json::Value = serde_json::from_slice(&output).unwrap();
            assert_eq!(report["reading_count"], 0);
            assert_eq!(report["enumeration_complete"], false);
            assert!(report["metrics"]["completed_families"].as_u64().unwrap() > 0);
        }
        let args = ProbeArgs {
            text: "🦇 glorpword".into(),
            category: "noun-phrase".into(),
            readings: 4,
            packed: false,
            json: true,
        };
        let mut output = Vec::new();
        inspect(&args, &lexicon, &grammar, Category::NounPhrase, &mut output).unwrap();
        let report: serde_json::Value = serde_json::from_slice(&output).unwrap();
        assert_eq!(report["admitted_root_count"], 0);
        assert_eq!(report["vocabulary_gaps"][0]["text"], "glorpword");
        assert_eq!(report["vocabulary_gaps"][0]["start"], 5);
    }
}
