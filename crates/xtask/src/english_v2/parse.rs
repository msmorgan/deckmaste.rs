use std::fmt::Write as _;
use std::io::Write;
use std::path::Path;

use anyhow::Context;
use anyhow::bail;
use deckmaste_english_v2::catalogs::ParserCatalogs;
use serde::Serialize;

use super::ParseArgs;
use super::audit::AuditReport;
use super::audit::AuditRow;
use super::audit::AuditStatus;
use super::corpus::Corpus;
use super::coverage_lock::CoverageLock;

pub(super) fn run(args: &ParseArgs, output: &mut dyn Write) -> anyhow::Result<()> {
    let stderr = std::io::stderr();
    let mut diagnostics = stderr.lock();
    run_with_diagnostics(args, output, &mut diagnostics)
}

fn run_with_diagnostics(
    args: &ParseArgs,
    output: &mut dyn Write,
    diagnostics: &mut dyn Write,
) -> anyhow::Result<()> {
    let corpus = Corpus::load(&args.corpus.data)
        .with_context(|| format!("loading corpus from {}", args.corpus.data.display()))?;
    let catalogs = ParserCatalogs::load(&args.corpus.catalogs).with_context(|| {
        format!(
            "loading parser catalogs from {}",
            args.corpus.catalogs.display()
        )
    })?;
    let parser = crate::english_v2::parser_from_catalogs(catalogs);
    let report = AuditReport::run(&corpus, &parser);

    render_report(&report, args.json, output)?;
    apply_coverage_lock(
        &report,
        &args.lock,
        args.bless,
        args.json,
        output,
        diagnostics,
    )?;
    if args.require_complete {
        require_complete(&report, corpus.units().len())?;
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

fn apply_coverage_lock(
    report: &AuditReport,
    path: &Path,
    bless: bool,
    json: bool,
    output: &mut dyn Write,
    diagnostic_output: &mut dyn Write,
) -> anyhow::Result<()> {
    let accepted = report.accepted_ids();
    if !path.exists() {
        if !bless {
            bail!(
                "English-v2 coverage lock {} is missing; review the parse report and rerun with --bless",
                path.display(),
            );
        }

        let replacement = CoverageLock::new(accepted, report.source_fingerprint().to_owned())?;
        replacement.write(path)?;
        let mut diagnostics = String::new();
        for identity in replacement.accepted() {
            writeln!(diagnostics, "newly accepted\t{identity}")
                .expect("writing to String cannot fail");
        }
        writeln!(
            diagnostics,
            "coverage lock blessed: accepted 0 -> {}",
            replacement.accepted().len(),
        )
        .expect("writing to String cannot fail");
        return write_lock_diagnostic(json, output, diagnostic_output, &diagnostics);
    }

    let baseline = CoverageLock::read(path)?;
    if baseline.source_fingerprint() != report.source_fingerprint() {
        let mut fingerprint_diagnostic = String::new();
        writeln!(
            fingerprint_diagnostic,
            "coverage lock source fingerprint changed: old {} new {}",
            baseline.source_fingerprint(),
            report.source_fingerprint(),
        )
        .expect("writing to String cannot fail");
        write_lock_diagnostic(json, output, diagnostic_output, &fingerprint_diagnostic)?;
    }
    let mut diagnostics = String::new();
    if bless {
        let replacement = baseline
            .bless(&accepted, report.source_fingerprint())
            .map_err(|error| anyhow::anyhow!("coverage lock bless refused: {error}"))?;
        let old_count = baseline.accepted().len();
        for identity in replacement.accepted().difference(baseline.accepted()) {
            writeln!(diagnostics, "newly accepted\t{identity}")
                .expect("writing to String cannot fail");
        }
        replacement.write(path)?;
        writeln!(
            diagnostics,
            "coverage lock blessed: accepted {old_count} -> {}",
            replacement.accepted().len(),
        )
        .expect("writing to String cannot fail");
        return write_lock_diagnostic(json, output, diagnostic_output, &diagnostics);
    }

    baseline.check(&accepted)?;
    for identity in accepted.difference(baseline.accepted()) {
        writeln!(diagnostics, "newly accepted\t{identity}").expect("writing to String cannot fail");
    }
    write_lock_diagnostic(json, output, diagnostic_output, &diagnostics)
}

fn write_lock_diagnostic(
    json: bool,
    output: &mut dyn Write,
    diagnostic_output: &mut dyn Write,
    diagnostic: &str,
) -> anyhow::Result<()> {
    if diagnostic.is_empty() {
        return Ok(());
    }
    if json {
        diagnostic_output
            .write_all(diagnostic.as_bytes())
            .context("writing English-v2 coverage lock diagnostic")
    } else {
        output
            .write_all(diagnostic.as_bytes())
            .context("writing English-v2 coverage lock diagnostic")
    }
}

fn require_complete(report: &AuditReport, corpus_size: usize) -> anyhow::Result<()> {
    let summary = report.summary();
    let accepted = report.accepted_ids().len();
    if accepted == corpus_size
        && summary.parse_failures == 0
        && summary.ambiguous == 0
        && summary.internal_failures == 0
    {
        return Ok(());
    }

    bail!(
        "corpus parse is incomplete: accepted {accepted} of {corpus_size}, {}, {}, {}",
        counted(summary.parse_failures, "parse failure", "parse failures"),
        counted(summary.ambiguous, "ambiguity", "ambiguities"),
        counted(
            summary.internal_failures,
            "internal failure",
            "internal failures"
        ),
    )
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

#[cfg(test)]
mod tests {
    use std::collections::BTreeSet;
    use std::fs;
    use std::path::Path;

    use super::apply_coverage_lock;
    use super::render_report;
    use super::require_complete;
    use super::run_with_diagnostics;
    use crate::english_v2::CorpusArgs;
    use crate::english_v2::ParseArgs;
    use crate::english_v2::audit::AuditReport;
    use crate::english_v2::audit::AuditStatus;
    use crate::english_v2::corpus::Corpus;
    use crate::english_v2::coverage_lock::CoverageLock;

    const CLEAN_TEXT: &str = "Whenever a player connives, you gain X life.";

    fn id(digit: char) -> String {
        digit.to_string().repeat(64)
    }

    fn snapshot(entries: &[(&str, &str)]) -> String {
        let cards = entries
            .iter()
            .map(|(name, text)| {
                format!(
                    "\"{name}\": [{{\"name\": \"{name}\", \"layout\": \"normal\", \"types\": [\"Creature\"], \"supertypes\": [], \"subtypes\": [], \"legalities\": {{\"vintage\": \"Legal\"}}, \"text\": \"{text}\"}}]"
                )
            })
            .collect::<Vec<_>>()
            .join(",");
        format!("{{\"data\": {{{cards}}}}}")
    }

    fn args(
        data: &Path,
        lock: &Path,
        json: bool,
        bless: bool,
        require_complete: bool,
    ) -> ParseArgs {
        ParseArgs {
            corpus: CorpusArgs {
                data: data.to_owned(),
                catalogs: Path::new(env!("CARGO_MANIFEST_DIR")).join("../../data/gen/catalogs"),
            },
            json,
            require_complete,
            lock: lock.to_owned(),
            bless,
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
                "internal_failure",
            ]
        );
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
        assert_eq!(
            output,
            concat!(
                "parse_failure\t",
                "0000000000000000000000000000000000000000000000000000000000000003\t",
                "Fixture 3\tparse failed at bytes 0..7\n",
                "ambiguous\t",
                "0000000000000000000000000000000000000000000000000000000000000004\t",
                "Fixture 4\tambiguous parse between first and second\n",
                "internal_failure\t",
                "0000000000000000000000000000000000000000000000000000000000000005\t",
                "Fixture 5\tvalidated chart root did not materialize\n",
                "English v2 parse census\n",
                "  total                 5\n",
                "  accepted              2\n",
                "  parse failures        1\n",
                "  ambiguities           1\n",
                "  internal failures     1\n",
            )
        );
        assert!(!output.contains("..."));
    }

    #[test]
    fn require_complete_rejects_only_parse_failures_ambiguities_and_internal_failures() {
        let clean_only_report = AuditReport::from_statuses_for_test(&[AuditStatus::Clean]);
        let mismatch_only_report = AuditReport::from_statuses_for_test(&[AuditStatus::Mismatch]);
        let report_with_failure = AuditReport::from_statuses_for_test(&[
            AuditStatus::ParseFailure,
            AuditStatus::Ambiguous,
            AuditStatus::InternalFailure,
        ]);

        assert!(require_complete(&clean_only_report, 1).is_ok());
        assert!(require_complete(&mismatch_only_report, 1).is_ok());
        let error = require_complete(&report_with_failure, 3)
            .unwrap_err()
            .to_string();
        assert!(error.contains("corpus parse is incomplete"));
        assert!(error.contains("accepted 0 of 3"));
        assert!(error.contains("1 parse failure"));
        assert!(error.contains("1 ambiguity"));
        assert!(error.contains("1 internal failure"));
    }

    #[test]
    fn require_complete_rejects_missing_accepted_identity_even_without_failures() {
        let report = AuditReport::from_statuses_for_test(&[AuditStatus::Clean]);

        let error = require_complete(&report, 2).unwrap_err().to_string();

        assert!(error.contains("accepted 1 of 2"));
        assert!(error.contains("0 parse failures"));
        assert!(error.contains("0 ambiguities"));
        assert!(error.contains("0 internal failures"));
    }

    #[test]
    fn normal_lock_check_reports_growth_without_mutating_the_baseline() {
        let directory = tempfile::tempdir().unwrap();
        let path = directory.path().join("coverage.lock");
        let report =
            AuditReport::from_statuses_for_test(&[AuditStatus::Clean, AuditStatus::Mismatch]);
        let baseline = CoverageLock::new(
            [format!("{:064x}", 1)].into_iter().collect(),
            "1".repeat(64),
        )
        .unwrap();
        baseline.write(&path).unwrap();
        let before = fs::read(&path).unwrap();
        let mut output = Vec::new();

        let mut diagnostics = Vec::new();
        apply_coverage_lock(&report, &path, false, false, &mut output, &mut diagnostics).unwrap();

        assert_eq!(fs::read(&path).unwrap(), before);
        let output = String::from_utf8(output).unwrap();
        assert!(output.contains(&format!("newly accepted\t{:064x}", 2)));
        assert!(output.contains("coverage lock source fingerprint changed"));
    }

    #[test]
    fn bless_checks_then_persists_an_expanded_accepted_set() {
        let directory = tempfile::tempdir().unwrap();
        let path = directory.path().join("coverage.lock");
        let report =
            AuditReport::from_statuses_for_test(&[AuditStatus::Clean, AuditStatus::Mismatch]);
        CoverageLock::new(
            [format!("{:064x}", 1)].into_iter().collect::<BTreeSet<_>>(),
            "0".repeat(64),
        )
        .unwrap()
        .write(&path)
        .unwrap();
        let mut output = Vec::new();

        let mut diagnostics = Vec::new();
        apply_coverage_lock(&report, &path, true, false, &mut output, &mut diagnostics).unwrap();

        assert_eq!(CoverageLock::read(&path).unwrap().accepted().len(), 2);
        assert!(
            String::from_utf8(output)
                .unwrap()
                .contains("coverage lock blessed: accepted 1 -> 2")
        );
    }

    #[test]
    fn missing_lock_requires_review_and_explicit_bless() {
        let directory = tempfile::tempdir().unwrap();
        let path = directory.path().join("missing.lock");
        let report = AuditReport::from_statuses_for_test(&[AuditStatus::Clean]);
        let mut output = Vec::new();

        let mut diagnostics = Vec::new();
        let error =
            apply_coverage_lock(&report, &path, false, false, &mut output, &mut diagnostics)
                .unwrap_err()
                .to_string();

        assert!(error.contains("review the parse report"));
        assert!(error.contains("--bless"));
        assert!(!path.exists());
    }

    #[test]
    fn runner_bless_creates_a_canonical_baseline_after_rendering_its_report() {
        let directory = tempfile::tempdir().unwrap();
        let data = directory.path().join("cards.json");
        let path = directory.path().join("missing.lock");
        fs::write(&data, snapshot(&[("Clean", CLEAN_TEXT)])).unwrap();
        let corpus = Corpus::load(&data).unwrap();
        let mut output = Vec::new();
        let mut diagnostics = Vec::new();

        run_with_diagnostics(
            &args(&data, &path, false, true, false),
            &mut output,
            &mut diagnostics,
        )
        .unwrap();

        let lock = CoverageLock::read(&path).unwrap();
        assert_eq!(lock.accepted().len(), 1);
        assert_eq!(lock.source_fingerprint(), corpus.source_fingerprint());
        assert_eq!(
            lock.accepted(),
            &corpus
                .units()
                .iter()
                .map(|unit| unit.id().to_owned())
                .collect::<BTreeSet<_>>(),
        );
        let output = String::from_utf8(output).unwrap();
        assert!(output.contains("newly accepted"));
        assert!(output.contains("coverage lock blessed: accepted 0 -> 1"));
        assert!(
            output.find("English v2 parse census").unwrap()
                < output
                    .find("coverage lock blessed: accepted 0 -> 1")
                    .unwrap()
        );
        assert!(diagnostics.is_empty());
    }

    #[test]
    fn runner_bless_refuses_lost_identities_after_rendering_without_mutating_lock() {
        let directory = tempfile::tempdir().unwrap();
        let data = directory.path().join("cards.json");
        let path = directory.path().join("coverage.lock");
        fs::write(&data, snapshot(&[("Clean", CLEAN_TEXT)])).unwrap();
        let (a, b) = (id('a'), id('b'));
        CoverageLock::new([a.clone(), b.clone()].into_iter().collect(), id('1'))
            .unwrap()
            .write(&path)
            .unwrap();
        let before = fs::read(&path).unwrap();
        let mut output = Vec::new();
        let mut diagnostics = Vec::new();
        let error = run_with_diagnostics(
            &args(&data, &path, false, true, false),
            &mut output,
            &mut diagnostics,
        )
        .unwrap_err()
        .to_string();

        let output = String::from_utf8(output).unwrap();
        assert!(error.contains("coverage lock bless refused"));
        assert!(error.contains("lost 2 previously accepted corpus identities"));
        assert!(error.contains(&a));
        assert!(error.contains(&b));
        assert!(error.find(&a).unwrap() < error.find(&b).unwrap());
        assert_eq!(fs::read(&path).unwrap(), before);
        assert!(output.contains("English v2 parse census"));
        assert!(output.contains("coverage lock source fingerprint changed: old"));
        assert!(diagnostics.is_empty());
    }

    #[test]
    fn run_writes_human_fingerprint_diagnostic_before_a_lost_identity_error() {
        let directory = tempfile::tempdir().unwrap();
        let data = directory.path().join("cards.json");
        let lock = directory.path().join("coverage.lock");
        fs::write(&data, snapshot(&[("Clean", CLEAN_TEXT)])).unwrap();
        CoverageLock::new([id('a')].into_iter().collect(), id('1'))
            .unwrap()
            .write(&lock)
            .unwrap();
        let mut output = Vec::new();
        let mut diagnostics = Vec::new();

        let error = run_with_diagnostics(
            &args(&data, &lock, false, false, false),
            &mut output,
            &mut diagnostics,
        )
        .unwrap_err()
        .to_string();

        let output = String::from_utf8(output).unwrap();
        assert!(error.contains("lost 1 previously accepted corpus identity"));
        assert!(output.contains("English v2 parse census"));
        assert!(output.contains("coverage lock source fingerprint changed: old"));
        assert!(diagnostics.is_empty());
    }

    #[test]
    fn run_keeps_json_stdout_parseable_when_lock_check_fails() {
        let directory = tempfile::tempdir().unwrap();
        let data = directory.path().join("cards.json");
        let lock = directory.path().join("coverage.lock");
        fs::write(&data, snapshot(&[("Clean", CLEAN_TEXT)])).unwrap();
        CoverageLock::new([id('a')].into_iter().collect(), id('1'))
            .unwrap()
            .write(&lock)
            .unwrap();
        let mut output = Vec::new();
        let mut diagnostics = Vec::new();

        let error = run_with_diagnostics(
            &args(&data, &lock, true, false, false),
            &mut output,
            &mut diagnostics,
        )
        .unwrap_err()
        .to_string();

        assert!(error.contains("lost 1 previously accepted corpus identity"));
        assert_eq!(
            serde_json::from_slice::<serde_json::Value>(&output).unwrap()["summary"]["total"],
            1
        );
        let diagnostics = String::from_utf8(diagnostics).unwrap();
        assert!(diagnostics.contains("coverage lock source fingerprint changed: old"));
        assert!(!String::from_utf8(output).unwrap().contains("coverage lock"));
    }

    #[test]
    fn run_prints_the_complete_report_before_a_completeness_error() {
        let directory = tempfile::tempdir().unwrap();
        let data = directory.path().join("cards.json");
        let lock = directory.path().join("coverage.lock");
        fs::write(
            &data,
            snapshot(&[("Clean", CLEAN_TEXT), ("Failed", "You frobnitz a card.")]),
        )
        .unwrap();
        let corpus = Corpus::load(&data).unwrap();
        let accepted = corpus
            .units()
            .iter()
            .filter(|unit| unit.text() == CLEAN_TEXT)
            .map(|unit| unit.id().to_owned())
            .collect();
        CoverageLock::new(accepted, corpus.source_fingerprint().to_owned())
            .unwrap()
            .write(&lock)
            .unwrap();
        let mut output = Vec::new();
        let mut diagnostics = Vec::new();

        let error = run_with_diagnostics(
            &args(&data, &lock, false, false, true),
            &mut output,
            &mut diagnostics,
        )
        .unwrap_err()
        .to_string();

        let output = String::from_utf8(output).unwrap();
        assert!(error.contains("corpus parse is incomplete"));
        assert!(output.contains("parse_failure"));
        assert!(output.contains("English v2 parse census"));
        assert!(diagnostics.is_empty());
    }
}
