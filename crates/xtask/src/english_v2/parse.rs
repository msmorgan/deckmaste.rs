use std::fs;
use std::io::Write;
use std::path::Path;

use anyhow::Context;
use anyhow::anyhow;
use serde::Serialize;

use super::ParseArgs;
use super::audit::AuditReport;
use super::audit::AuditRow;
use super::audit::AuditStatus;
use super::corpus::Corpus;

pub(super) fn run(args: &ParseArgs, output: &mut dyn Write) -> anyhow::Result<()> {
    let started = std::time::Instant::now();
    let workers = args.corpus.workers()?;
    let corpus = Corpus::load(&args.corpus.data, &args.corpus.selection)
        .with_context(|| format!("loading corpus from {}", args.corpus.data.display()))?;
    let parser = crate::english_v2::parser_from_builtin_v2()?;
    let report = AuditReport::run(&corpus, &parser, workers, true);

    if args.json {
        let rendered = JsonReport {
            schema_version: report.schema_version(),
            source_fingerprint: report.source_fingerprint(),
            rows: report.rows(),
            summary: report.summary(),
        };
        super::corpus::write_provenanced_json(&rendered, &corpus, workers, output)?;
    } else {
        render_report(&report, false, output)?;
    }
    output.flush().context("flushing English-v2 parse census")?;
    super::corpus::write_corpus_performance("parse", started.elapsed(), report.performance())?;
    if args.require_complete
        && let CompletionCheck::Incomplete(outcome) =
            completion_check(&report, corpus.units().len())
    {
        let error = incomplete_error(&outcome);
        if let Some(path) = std::env::var_os(super::REQUIRE_COMPLETE_OUTCOME_ENV) {
            write_completion_outcome(&outcome, Path::new(&path))?;
        }
        return Err(error);
    }
    Ok(())
}

fn render_report(report: &AuditReport, json: bool, output: &mut dyn Write) -> anyhow::Result<()> {
    if json {
        let rendered = JsonReport {
            schema_version: report.schema_version(),
            source_fingerprint: report.source_fingerprint(),
            rows: report.rows(),
            summary: report.summary(),
        };
        serde_json::to_writer_pretty(&mut *output, &rendered)
            .context("writing English-v2 parse census JSON")?;
        writeln!(output).context("writing English-v2 parse census JSON terminator")?;
        return Ok(());
    }

    for row in report
        .rows()
        .iter()
        .filter(|row| !matches!(row.status(), AuditStatus::Clean | AuditStatus::Mismatch))
    {
        writeln!(
            output,
            "{}\t{}\t{}\t{}",
            status_name(row.status()),
            row.id(),
            row.printed_face(),
            row.message().unwrap_or_default(),
        )
        .context("writing English-v2 parse census row")?;
    }

    let summary = report.summary();
    writeln!(output, "English v2 parse census").context("writing English-v2 parse census")?;
    writeln!(output, "  total                 {}", summary.total)
        .context("writing English-v2 parse census")?;
    writeln!(
        output,
        "  accepted              {}",
        report.accepted_ids().len()
    )
    .context("writing English-v2 parse census")?;
    writeln!(output, "  parse failures        {}", summary.parse_failures)
        .context("writing English-v2 parse census")?;
    writeln!(output, "  ambiguities           {}", summary.ambiguous)
        .context("writing English-v2 parse census")?;
    writeln!(
        output,
        "  internal failures     {}",
        summary.internal_failures
    )
    .context("writing English-v2 parse census")?;
    Ok(())
}

#[cfg(test)]
fn require_complete(report: &AuditReport, corpus_size: usize) -> anyhow::Result<()> {
    let CompletionCheck::Incomplete(outcome) = completion_check(report, corpus_size) else {
        return Ok(());
    };
    Err(incomplete_error(&outcome))
}

fn completion_check(report: &AuditReport, corpus_size: usize) -> CompletionCheck<'_> {
    let summary = report.summary();
    let accepted = report.accepted_ids().len();
    if accepted == corpus_size
        && summary.parse_failures == 0
        && summary.ambiguous == 0
        && summary.internal_failures == 0
    {
        return CompletionCheck::Complete;
    }

    CompletionCheck::Incomplete(CompletionOutcome {
        schema_version: report.schema_version(),
        source_fingerprint: report.source_fingerprint(),
        total: corpus_size,
        accepted,
        parse_failures: summary.parse_failures,
        ambiguous: summary.ambiguous,
        internal_failures: summary.internal_failures,
    })
}

fn incomplete_error(outcome: &CompletionOutcome<'_>) -> anyhow::Error {
    anyhow!(
        "corpus parse is incomplete: accepted {accepted} of {corpus_size}, {}, {}, {}",
        counted(outcome.parse_failures, "parse failure", "parse failures"),
        counted(outcome.ambiguous, "ambiguity", "ambiguities"),
        counted(
            outcome.internal_failures,
            "internal failure",
            "internal failures"
        ),
        accepted = outcome.accepted,
        corpus_size = outcome.total,
    )
}

fn write_completion_outcome(outcome: &CompletionOutcome<'_>, path: &Path) -> anyhow::Result<()> {
    let rendered = serde_json::to_vec(outcome).context("rendering require-complete outcome")?;
    fs::write(path, rendered)
        .with_context(|| format!("writing require-complete outcome to {}", path.display()))
}

fn status_name(status: AuditStatus) -> &'static str {
    match status {
        AuditStatus::Clean => "clean",
        AuditStatus::Mismatch => "mismatch",
        AuditStatus::ParseFailure => "parse_failure",
        AuditStatus::Ambiguous => "ambiguous",
        AuditStatus::InternalFailure => "internal_failure",
    }
}

fn counted(count: usize, singular: &str, plural: &str) -> String {
    format!("{count} {}", if count == 1 { singular } else { plural })
}

#[derive(Serialize)]
struct JsonReport<'a> {
    schema_version: u32,
    source_fingerprint: &'a str,
    rows: &'a [AuditRow],
    summary: super::audit::AuditSummary,
}

#[derive(Serialize)]
struct CompletionOutcome<'a> {
    schema_version: u32,
    source_fingerprint: &'a str,
    total: usize,
    accepted: usize,
    parse_failures: usize,
    ambiguous: usize,
    internal_failures: usize,
}

enum CompletionCheck<'a> {
    Complete,
    Incomplete(CompletionOutcome<'a>),
}

#[cfg(test)]
mod tests {
    use std::fs;
    use std::path::Path;

    use super::CompletionCheck;
    use super::completion_check;
    use super::render_report;
    use super::require_complete;
    use super::run;
    use super::write_completion_outcome;
    use crate::english_v2::CorpusArgs;
    use crate::english_v2::ParseArgs;
    use crate::english_v2::audit::AuditReport;
    use crate::english_v2::audit::AuditStatus;

    const CLEAN_TEXT: &str = "Whenever a player connives, you gain X life.";

    fn snapshot(entries: &[(&str, &str)]) -> String {
        entries
            .iter()
            .enumerate()
            .map(|(index, (name, text))| {
                serde_json::json!({
                    "object": "card",
                    "id": format!("printing-{index}"),
                    "oracle_id": format!("oracle-{index}"),
                    "name": name,
                    "layout": "normal",
                    "type_line": "Creature",
                    "legalities": {"vintage": "legal"},
                    "oracle_text": text,
                })
                .to_string()
            })
            .collect::<Vec<_>>()
            .join("\n")
            + "\n"
    }

    fn args(data: &Path, json: bool, require_complete: bool) -> ParseArgs {
        ParseArgs {
            corpus: CorpusArgs {
                data: data.to_owned(),
                selection: crate::raw_corpus::CorpusSelectionArgs::all(),
                workers: Some(1),
            },
            json,
            require_complete,
        }
    }

    fn complete_fixture_report() -> AuditReport {
        AuditReport::from_statuses_for_test(&[
            AuditStatus::Clean,
            AuditStatus::Mismatch,
            AuditStatus::ParseFailure,
            AuditStatus::Ambiguous,
            AuditStatus::InternalFailure,
        ])
    }

    #[test]
    fn render_report_preserves_every_ordered_row_and_summary_in_json() {
        let report = complete_fixture_report();
        let mut output = Vec::new();
        render_report(&report, true, &mut output).unwrap();
        let json: serde_json::Value = serde_json::from_slice(&output).unwrap();
        assert_eq!(json["schema_version"], 1);
        assert_eq!(json["source_fingerprint"], "0".repeat(64));
        assert_eq!(
            json["rows"]
                .as_array()
                .unwrap()
                .iter()
                .map(|row| row["status"].as_str().unwrap())
                .collect::<Vec<_>>(),
            [
                "clean",
                "mismatch",
                "parse_failure",
                "ambiguous",
                "internal_failure"
            ]
        );
        assert!(
            json["rows"][0].get("message").is_none(),
            "parse JSON omits a null message from a clean row",
        );
        assert_eq!(json["rows"][1]["message"], "rendered bytes differ");
        assert_eq!(
            json["summary"],
            serde_json::json!({
                "total": 5,
                "clean": 1,
                "mismatched": 1,
                "parse_failures": 1,
                "ambiguous": 1,
                "internal_failures": 1,
            })
        );
    }

    #[test]
    fn render_report_lists_all_parse_failures_before_the_human_summary() {
        let report = complete_fixture_report();
        let mut output = Vec::new();
        render_report(&report, false, &mut output).unwrap();
        let output = String::from_utf8(output).unwrap();
        assert!(output.contains("parse_failure"));
        assert!(output.contains("ambiguous"));
        assert!(output.contains("internal_failure"));
        assert!(
            output.find("parse_failure").unwrap() < output.find("English v2 parse census").unwrap()
        );
        assert!(!output.contains("..."));
    }

    #[test]
    fn require_complete_rejects_only_incomplete_outcomes_or_missing_units() {
        let clean = AuditReport::from_statuses_for_test(&[AuditStatus::Clean]);
        let mismatch = AuditReport::from_statuses_for_test(&[AuditStatus::Mismatch]);
        let failures = AuditReport::from_statuses_for_test(&[
            AuditStatus::ParseFailure,
            AuditStatus::Ambiguous,
            AuditStatus::InternalFailure,
        ]);
        assert!(require_complete(&clean, 1).is_ok());
        assert!(require_complete(&mismatch, 1).is_ok());
        let error = require_complete(&failures, 3).unwrap_err().to_string();
        assert!(error.contains("accepted 0 of 3"));
        assert!(error.contains("1 parse failure"));
        assert!(error.contains("1 ambiguity"));
        assert!(error.contains("1 internal failure"));
        assert!(
            require_complete(&clean, 2)
                .unwrap_err()
                .to_string()
                .contains("accepted 1 of 2")
        );
    }

    #[test]
    fn completion_outcome_is_a_compact_exact_structured_census() {
        let report = complete_fixture_report();
        let outcome = tempfile::NamedTempFile::new().unwrap();
        let CompletionCheck::Incomplete(census) = completion_check(&report, 5) else {
            panic!("mixed fixture is incomplete");
        };
        write_completion_outcome(&census, outcome.path()).unwrap();
        let json: serde_json::Value =
            serde_json::from_slice(&fs::read(outcome.path()).unwrap()).unwrap();
        assert_eq!(
            json,
            serde_json::json!({
                "schema_version": 1,
                "source_fingerprint": "0".repeat(64),
                "total": 5,
                "accepted": 2,
                "parse_failures": 1,
                "ambiguous": 1,
                "internal_failures": 1,
            })
        );
        assert!(json.get("rows").is_none());
    }

    #[test]
    fn run_prints_the_complete_report_before_a_completeness_error() {
        let directory = tempfile::tempdir().unwrap();
        let data = directory.path().join("cards.json");
        fs::write(
            &data,
            snapshot(&[
                ("Seven Dwarves", CLEAN_TEXT),
                ("Grizzly Bears", "You frobnitz a card."),
            ]),
        )
        .unwrap();
        let mut output = Vec::new();
        let error = run(&args(&data, false, true), &mut output)
            .unwrap_err()
            .to_string();
        let output = String::from_utf8(output).unwrap();
        assert!(error.contains("corpus parse is incomplete"));
        assert!(output.contains("parse_failure"));
        assert!(output.contains("English v2 parse census"));
    }
}
