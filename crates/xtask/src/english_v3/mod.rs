//! Raw supported-corpus census for the generated v3 grammar.

mod probe;
mod report;
mod validation;

#[cfg(test)]
mod tests;

use std::collections::BTreeMap;
use std::collections::BTreeSet;
use std::io::BufWriter;
use std::io::Write;
use std::num::NonZeroUsize;
use std::path::Path;
use std::path::PathBuf;
use std::time::Instant;

use anyhow::Context;
use anyhow::Result;
use anyhow::ensure;
use clap::Args;
use clap::ValueEnum;
use deckmaste_english_v3::grammar::Category;
use deckmaste_english_v3::grammar::Grammar;
use deckmaste_english_v3::parse;
use deckmaste_lexical::Lexicon;
use rayon::prelude::*;
use serde::Serialize;

use crate::english_v3::report::Census;
use crate::english_v3::report::Enumeration;
use crate::english_v3::report::FaceReport;
use crate::english_v3::report::Measurements;
use crate::english_v3::report::Report;
use crate::english_v3::report::Sample;
use crate::english_v3::report::UnknownWord;
use crate::english_v3::validation::Issue;
use crate::english_v3::validation::Tracing;
use crate::english_v3::validation::validate_in_context;
use crate::raw_corpus::CorpusSelectionArgs;
use crate::raw_corpus::SelectedFace;
use crate::raw_corpus::digest;

#[derive(Debug, Clone, Copy, Serialize, ValueEnum)]
#[serde(rename_all = "snake_case")]
pub enum SourceField {
    Text,
    TypeLine,
}

impl SourceField {
    fn category(self) -> Category {
        match self {
            Self::Text => Category::Document,
            Self::TypeLine => Category::TypeLine,
        }
    }

    fn source(self, face: &SelectedFace) -> Option<&str> {
        match self {
            Self::Text => face.text.as_deref(),
            Self::TypeLine => face.type_line.as_deref(),
        }
    }
}

#[derive(Debug, Args)]
#[command(subcommand_negates_reqs = true)]
pub struct EnglishV3Args {
    #[command(subcommand)]
    pub command: Option<probe::Command>,
    /// Face field to parse with its declared root Category.
    #[arg(long, value_enum, default_value = "text")]
    pub field: SourceField,
    /// Pinned Scryfall Oracle Cards JSONL snapshot; source is not normalized.
    #[arg(long, default_value = "data/scryfall/oracle-cards.jsonl")]
    pub data: PathBuf,
    #[command(flatten)]
    pub(crate) selection: CorpusSelectionArgs,
    /// Destination for the reproducible face census and diagnostics.
    #[arg(long, required = true)]
    pub output: Option<PathBuf>,
    /// Cap candidate requests per face. Omit for a complete census.
    #[arg(long)]
    pub reading_limit: Option<NonZeroUsize>,
    /// Retain this many checked grammatical trees per face.
    #[arg(long, default_value_t = 2)]
    pub samples_per_face: usize,
    /// Dedicated corpus worker threads.
    #[arg(long, default_value = "1")]
    pub workers: NonZeroUsize,
}

/// Run declaration admission, exact realization and structural traversal
/// checks. # Errors
/// Reports input/output failures and any internal or validation issue, after
/// writing the report when analysis completed. No Reading is a census result.
pub fn run(args: &EnglishV3Args, output: &mut dyn Write) -> Result<()> {
    if let Some(probe::Command::Probe(args)) = &args.command {
        return probe::run(args, output);
    }
    let started = Instant::now();
    let corpus = args.selection.load(&args.data)?;
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
    let sources = deckmaste_lexical_source::load_workspace(&root)?;
    let lexicon = Lexicon::new(sources.lexemes).context("indexing declared lexical inventory")?;
    let environment = deckmaste_english_v3::grammar::GrammarEnvironment::new(&lexicon);
    let grammar = environment.grammar();
    let setup_ns = started.elapsed().as_nanos();
    let inventory_sha256 = digest(&serde_json::to_vec(lexicon.lexemes())?);
    let load = report::host_load();
    let started = Instant::now();
    let faces = analyze_cards(&corpus.faces, &lexicon, grammar, args)?;
    let corpus_ns = started.elapsed().as_nanos();
    let report = Report::new(
        args,
        &corpus,
        inventory_sha256,
        sources.unmapped,
        faces,
        Measurements {
            setup_wall_ns: setup_ns,
            corpus_wall_ns: corpus_ns,
            host_load: load,
        },
    );
    write_report(args, &report, output)
}

fn write_report(args: &EnglishV3Args, report: &Report, output: &mut dyn Write) -> Result<()> {
    let destination = args
        .output
        .as_ref()
        .context("corpus evaluation requires --output")?;
    let file = std::fs::File::create(destination)
        .with_context(|| format!("creating {}", destination.display()))?;
    let mut writer = BufWriter::new(file);
    serde_json::to_writer(&mut writer, report).context("writing English v3 census")?;
    writer.write_all(b"\n")?;
    writer.flush()?;
    writeln!(
        output,
        "{} supported faces: {} no, {} one, {} multiple, {} undetermined; {} issues",
        report.faces.len(),
        report.totals.no,
        report.totals.one,
        report.totals.multiple,
        report.totals.undetermined,
        report.totals.issues
    )?;
    writeln!(
        output,
        "{} ns corpus wall; {} workers; host load {:?}; checked-text thread CPU {} ns/B",
        report.measurements.corpus_wall_ns,
        report.workers,
        report.measurements.host_load,
        report
            .totals
            .checked_text_cpu_ns_per_byte
            .map_or_else(|| "unavailable".into(), |n| n.to_string())
    )?;
    writeln!(output, "Report: {}", destination.display())?;
    ensure!(
        report.totals.issues == 0,
        "English v3 issues recorded in {}",
        destination.display()
    );
    Ok(())
}

fn analyze_cards(
    faces: &[SelectedFace],
    lexicon: &Lexicon,
    grammar: &Grammar,
    args: &EnglishV3Args,
) -> Result<Vec<FaceReport>> {
    let pool = rayon::ThreadPoolBuilder::new()
        .num_threads(args.workers.get())
        .build()?;
    pool.install(|| {
        faces
            .par_iter()
            .map(|face| analyze_face(face, lexicon, grammar, args))
            .collect()
    })
}

fn analyze_face(
    face: &SelectedFace,
    lexicon: &Lexicon,
    grammar: &Grammar,
    args: &EnglishV3Args,
) -> Result<FaceReport> {
    let cpu_start = report::thread_cpu_ns();
    let started = Instant::now();
    let mut result = FaceReport::new(face, args.field);
    let analyzed = lexicon.analyze_source(
        args.field.source(face).unwrap_or(""),
        Some(&result.analyzed_source),
    );
    for range in analyzed.unknown_words() {
        let bytes = analyzed
            .byte_range(range.start, range.end)
            .context("unknown lexical word has invalid coordinates")?;
        result.unknown_words.push(UnknownWord {
            text: result.analyzed_source[bytes.clone()].to_owned(),
            start: bytes.start,
            end: bytes.end,
        });
    }
    result.lexical_wall_ns = started.elapsed().as_nanos();
    let started = Instant::now();
    match parse(grammar, lexicon, &analyzed, &args.field.category()) {
        Ok(forest) => {
            result.chart_wall_ns = started.elapsed().as_nanos();
            result.chart = report::chart_metrics(forest.metrics());
            let started = Instant::now();
            let mut values = forest.readings(Tracing::new(grammar));
            let mut checked = BTreeSet::new();
            let context = std::cell::OnceCell::new();
            let mut requested = 0;
            loop {
                if args
                    .reading_limit
                    .is_some_and(|limit| requested >= limit.get())
                {
                    result.enumeration = Enumeration::Limited;
                    break;
                }
                let Some(value) = values.next() else {
                    result.enumeration = Enumeration::Complete;
                    break;
                };
                requested += 1;
                let value = match value {
                    Ok(value) => value,
                    Err(error) => {
                        result
                            .issues
                            .push(Issue::Materialization(error.to_string()));
                        continue;
                    }
                };
                match validate_in_context(
                    &value,
                    &result.analyzed_source,
                    args.field.category(),
                    context.get_or_init(|| {
                        deckmaste_english_v3::grammar::RealizationContext::new(
                            grammar,
                            lexicon,
                            &result.analyzed_source,
                        )
                    }),
                ) {
                    Ok(reading) => {
                        if !checked.insert((
                            value.identity().to_owned(),
                            std::sync::Arc::clone(&value.value),
                        )) {
                            result.issues.push(Issue::DuplicateReading);
                            continue;
                        }
                        for node in value.nodes.iter() {
                            if let deckmaste_english_v3::grammar::Value::Reading(node) = &**node {
                                *result.constructions.entry(node.construction()).or_default() += 1;
                            }
                        }
                        if args.samples_per_face != 0 {
                            let total_cost =
                                reading.total_cost().context("validated reading cost")?;
                            if !report::sample_cost_would_be_retained(
                                &result.samples,
                                total_cost,
                                args.samples_per_face,
                            ) {
                                continue;
                            }
                            let id = value.identity();
                            if report::sample_would_be_retained(
                                &result.samples,
                                total_cost,
                                id,
                                args.samples_per_face,
                            ) {
                                report::retain_sample(
                                    &mut result.samples,
                                    Sample::new(reading, &value, lexicon, total_cost),
                                    args.samples_per_face,
                                );
                            }
                        }
                    }
                    Err(issue) => result.issues.push(issue),
                }
            }
            result.readings = report::reading_metrics(values.metrics());
            result.materializations = values.materializer().metrics();
            result.checked_readings = checked.len();
            result.validation_wall_ns = started.elapsed().as_nanos();
        }
        Err(error) => {
            result.chart_wall_ns = started.elapsed().as_nanos();
            result.issues.push(Issue::Parser(error.to_string()));
        }
    }
    if !result.issues.is_empty() {
        result.enumeration = Enumeration::Failed;
    }
    result.exact_readings =
        (result.enumeration == Enumeration::Complete).then_some(result.checked_readings);
    result.census = match (result.checked_readings, result.enumeration) {
        (0, Enumeration::Complete) => Census::No,
        (1, Enumeration::Complete) => Census::One,
        (2.., _) => Census::Multiple,
        _ => Census::Undetermined,
    };
    result.thread_cpu_ns = cpu_start
        .zip(report::thread_cpu_ns())
        .map(|(start, end)| end - start);
    Ok(result)
}

fn sum_metrics<'a>(
    maps: impl Iterator<Item = &'a BTreeMap<&'static str, usize>>,
) -> BTreeMap<&'static str, usize> {
    let mut result = BTreeMap::new();
    for map in maps {
        for (&key, &count) in map {
            *result.entry(key).or_default() += count;
        }
    }
    result
}
