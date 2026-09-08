//! Raw supported-corpus census for the generated v3 grammar.

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
use deckmaste_data::mtgjson::AtomicCards;
use deckmaste_english_v3::grammar::Category;
use deckmaste_english_v3::grammar::Grammar;
use deckmaste_english_v3::parse;
use deckmaste_lexical::Lexicon;
use rayon::prelude::*;

use crate::english_v3::report::Census;
use crate::english_v3::report::Enumeration;
use crate::english_v3::report::FaceReport;
use crate::english_v3::report::Measurements;
use crate::english_v3::report::Report;
use crate::english_v3::report::Sample;
use crate::english_v3::report::UnknownWord;
use crate::english_v3::validation::Issue;
use crate::english_v3::validation::Tracing;
use crate::english_v3::validation::validate;
use crate::raw_corpus::SupportedFace;
use crate::raw_corpus::digest;
use crate::raw_corpus::supported_faces;

#[derive(Debug, Args)]
pub struct EnglishV3Args {
    /// Raw MTGJSON `AtomicCards` snapshot; text is analyzed without
    /// normalization.
    #[arg(long, default_value = "data/mtgjson/AtomicCards.json")]
    pub data: PathBuf,
    /// Destination for the reproducible face census and diagnostics.
    #[arg(long)]
    pub output: PathBuf,
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
    let started = Instant::now();
    let bytes =
        std::fs::read(&args.data).with_context(|| format!("reading {}", args.data.display()))?;
    let cards = AtomicCards::parse(&bytes).context("parsing raw AtomicCards snapshot")?;
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
    let sources = deckmaste_lexical_source::load_workspace(&root)?;
    let lexicon = Lexicon::new(sources.lexemes).context("indexing declared lexical inventory")?;
    let grammar = Grammar::default();
    let setup_ns = started.elapsed().as_nanos();
    let inventory_sha256 = digest(&serde_json::to_vec(lexicon.lexemes())?);
    let load = report::host_load();
    let started = Instant::now();
    let faces = analyze_cards(&cards, &lexicon, &grammar, args)?;
    let corpus_ns = started.elapsed().as_nanos();
    let report = Report::new(
        args,
        &bytes,
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
    let file = std::fs::File::create(&args.output)
        .with_context(|| format!("creating {}", args.output.display()))?;
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
    writeln!(output, "Report: {}", args.output.display())?;
    ensure!(
        report.totals.issues == 0,
        "English v3 issues recorded in {}",
        args.output.display()
    );
    Ok(())
}

fn analyze_cards(
    cards: &AtomicCards<'_>,
    lexicon: &Lexicon,
    grammar: &Grammar,
    args: &EnglishV3Args,
) -> Result<Vec<FaceReport>> {
    let pool = rayon::ThreadPoolBuilder::new()
        .num_threads(args.workers.get())
        .build()?;
    pool.install(|| {
        supported_faces(cards)
            .par_iter()
            .map(|face| analyze_face(face, lexicon, grammar, args))
            .collect()
    })
}

fn analyze_face(
    face: &SupportedFace<'_, '_>,
    lexicon: &Lexicon,
    grammar: &Grammar,
    args: &EnglishV3Args,
) -> Result<FaceReport> {
    let cpu_start = report::thread_cpu_ns();
    let started = Instant::now();
    let raw = face.card.text.as_deref().unwrap_or("");
    let mut result = FaceReport::new(face)?;
    let analyzed = lexicon.analyze_source(raw, None);
    for range in analyzed.unknown_words() {
        let bytes = analyzed
            .byte_range(range.start, range.end)
            .context("unknown lexical word has invalid coordinates")?;
        result.unknown_words.push(UnknownWord {
            text: raw[bytes.clone()].to_owned(),
            start: bytes.start,
            end: bytes.end,
        });
    }
    result.lexical_wall_ns = started.elapsed().as_nanos();
    let started = Instant::now();
    match parse(grammar, lexicon, &analyzed, &Category::Document) {
        Ok(forest) => {
            result.chart_wall_ns = started.elapsed().as_nanos();
            result.chart = report::chart_metrics(forest.metrics());
            let started = Instant::now();
            let mut values = forest.readings(Tracing(grammar));
            let mut checked = BTreeSet::new();
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
                match validate(&value, raw, lexicon) {
                    Ok(reading) => {
                        if !checked.insert(reading.clone()) {
                            result.issues.push(Issue::DuplicateReading);
                            continue;
                        }
                        for node in &value.nodes {
                            *result.constructions.entry(node.construction).or_default() += 1;
                        }
                        if result.samples.len() < args.samples_per_face {
                            result.samples.push(Sample::new(reading, &value, lexicon));
                        }
                    }
                    Err(issue) => result.issues.push(issue),
                }
            }
            result.readings = report::reading_metrics(values.metrics());
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
