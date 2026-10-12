//! Raw supported-corpus census for the generated v3 grammar.

mod blockers;
mod card_scope;
mod decomposition;
mod diff;
mod grep;
mod incremental;
mod probe;
mod report;
mod validation;

#[cfg(test)]
mod tests;

use std::collections::BTreeMap;
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
use serde::Deserialize;
use serde::Serialize;

use crate::english_v3::report::FaceReport;
use crate::english_v3::report::Measurements;
use crate::english_v3::report::Report;
use crate::english_v3::report::Sample;
use crate::english_v3::report::UnknownWord;
use crate::english_v3::validation::Issue;
use crate::raw_corpus::CorpusSelectionArgs;
use crate::raw_corpus::SelectedFace;
use crate::raw_corpus::digest;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Serialize, ValueEnum)]
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
    /// Reuse a complete census for lexicon-only changes; incompatible bases
    /// or changes to grammar/runtime sources fall back to checking all selected
    /// faces.
    #[arg(long)]
    pub incremental_from: Option<PathBuf>,
    /// Verify the single schema-8 migration baseline against reconstructed
    /// region trees.
    #[arg(long)]
    pub migration_from: Option<PathBuf>,
    /// Cap candidate requests per enumeration unit. Omit for a complete census.
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
    match &args.command {
        Some(probe::Command::Probe(probe_args)) => {
            return probe::run(probe_args, &args.data, output);
        }
        Some(probe::Command::Diff(diff_args)) => return diff::run(diff_args, output),
        Some(probe::Command::Grep(grep_args)) => {
            return grep::run(grep_args, &args.data, output);
        }
        Some(probe::Command::Blockers(blocker_args)) => {
            return blockers::run(blocker_args, &args.data, output);
        }
        None => {}
    }
    ensure!(
        args.migration_from.is_none() || args.reading_limit.is_none(),
        "migration requires complete enumeration"
    );
    let started = Instant::now();
    let corpus = args.selection.load(&args.data)?;
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
    let sources = deckmaste_lexical_source::load_workspace(&root)?;
    let lexicon = Lexicon::new(sources.lexemes).context("indexing declared lexical inventory")?;
    let environment = deckmaste_english_v3::grammar::GrammarEnvironment::new(&lexicon);
    let grammar = environment.grammar();
    let inventory_sha256 = digest(&serde_json::to_vec(lexicon.lexemes())?);
    let snapshot = incremental::Snapshot::capture(&root, &lexicon)?;
    let base = args
        .incremental_from
        .as_deref()
        .map(incremental::Base::load)
        .transpose()?
        .flatten();
    let setup_ns = started.elapsed().as_nanos();
    let load = report::host_load();
    let started = Instant::now();
    let (faces, incremental) =
        incremental::analyze(&corpus, &lexicon, grammar, args, &snapshot, base)?;
    let corpus_ns = started.elapsed().as_nanos();
    let mut report = Report::new(
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
    report.incremental_snapshot = Some(snapshot);
    report.incremental = incremental;
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
        "{} supported faces: {} unread, {} read unambiguous, {} read ambiguous, {} incomplete; {} issues",
        report.faces.len(),
        report.totals.no,
        report.totals.one,
        report.totals.multiple,
        report.totals.undetermined,
        report.totals.issues
    )?;
    writeln!(
        output,
        "{} checked enumerated unit Readings (exact only for complete faces)",
        report.totals.checked_readings
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
    if report.incremental.used {
        writeln!(
            output,
            "Incremental: {} reused, {} rechecked; base {}",
            report.incremental.reused_faces.len(),
            report.incremental.rechecked_faces.len(),
            report
                .incremental
                .base_census
                .as_ref()
                .context("incremental base path")?
                .display()
        )?;
    } else if let Some(reason) = &report.incremental.fallback_reason {
        writeln!(
            output,
            "Incremental disabled: {reason}; checked all selected faces"
        )?;
    }
    writeln!(output, "Report: {}", destination.display())?;
    if let Some(path) = &args.migration_from {
        decomposition::verify_migration(path, report, output)?;
    }
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
    Ok(analyze_face_with_progress(face, lexicon, grammar, args)?.0)
}

fn analyze_face_with_progress(
    face: &SelectedFace,
    lexicon: &Lexicon,
    grammar: &Grammar,
    args: &EnglishV3Args,
) -> Result<(FaceReport, usize)> {
    let mut furthest_position = 0;
    let cpu_start = report::thread_cpu_ns();
    let started = Instant::now();
    let mut result = FaceReport::new(face, args.field);
    let scope = card_scope::for_face(face);
    let analyzed = lexicon.analyze_source_scoped(
        args.field.source(face).unwrap_or(""),
        Some(&result.analyzed_source),
        &scope,
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
            furthest_position = forest.metrics().furthest_position;
            result.chart = report::chart_metrics(forest.metrics());
            let started = Instant::now();
            let context = deckmaste_english_v3::grammar::RealizationContext::new_scoped(
                grammar,
                lexicon,
                analyzed.text(),
                &scope,
            );
            decomposition::analyze(
                &forest,
                &analyzed,
                grammar,
                lexicon,
                args,
                &mut result,
                &context,
            )?;
            result.validation_wall_ns = started.elapsed().as_nanos();
        }
        Err(error) => {
            result.chart_wall_ns = started.elapsed().as_nanos();
            result.issues.push(Issue::Parser(error.to_string()));
        }
    }
    result.finish_readings();
    result.thread_cpu_ns = cpu_start
        .zip(report::thread_cpu_ns())
        .map(|(start, end)| end - start);
    Ok((result, furthest_position))
}

fn sample_line_reading(
    reading: &deckmaste_english_v3::grammar::Reading,
    value: &crate::english_v3::validation::TracedValue,
    lexicon: &Lexicon,
    samples: &mut Vec<Sample>,
    limit: usize,
) -> Result<()> {
    if limit == 0 {
        return Ok(());
    }
    let cost = reading.total_cost().context("validated reading cost")?;
    if report::sample_cost_would_be_retained(samples, cost, limit)
        && report::sample_would_be_retained(samples, cost, value.identity(), limit)
    {
        report::retain_sample(samples, Sample::new(reading, value, lexicon, cost), limit);
    }
    Ok(())
}

fn sum_metrics<'a>(
    maps: impl Iterator<Item = &'a BTreeMap<String, usize>>,
) -> BTreeMap<String, usize> {
    let mut result = BTreeMap::new();
    for map in maps {
        for (key, &count) in map {
            *result.entry(key.clone()).or_default() += count;
        }
    }
    result
}
