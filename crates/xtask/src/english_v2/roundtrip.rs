use std::io::Write;

use anyhow::Context;
use anyhow::bail;
use serde::Serialize;

use super::RoundtripArgs;
use super::audit::AuditReport;
use super::audit::AuditRow;
use super::audit::AuditStatus;
use super::corpus::Corpus;

pub(super) fn run(args: &RoundtripArgs, output: &mut dyn Write) -> anyhow::Result<()> {
    let started = std::time::Instant::now();
    let workers = args.corpus.workers()?;
    let corpus = Corpus::load(&args.corpus.data, &args.corpus.selection)
        .with_context(|| format!("loading corpus from {}", args.corpus.data.display()))?;
    let parser = crate::english_v2::parser_from_builtin_v2()?;
    let report = AuditReport::run(&corpus, &parser, workers, false);

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
    output
        .flush()
        .context("flushing English-v2 round-trip report")?;
    super::corpus::write_corpus_performance("roundtrip", started.elapsed(), report.performance())?;
    if args.require_clean {
        require_clean(&report)?;
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
            .context("writing English-v2 round-trip JSON")?;
        writeln!(output).context("writing English-v2 round-trip JSON terminator")?;
        return Ok(());
    }

    for row in report
        .rows()
        .iter()
        .filter(|row| row.status() == AuditStatus::Mismatch)
    {
        let id = serde_json::to_string(row.id())
            .context("encoding English-v2 round-trip mismatch ID")?;
        let face = serde_json::to_string(row.printed_face())
            .context("encoding English-v2 round-trip mismatch face")?;
        let expected = serde_json::to_string(row.text())
            .context("encoding English-v2 round-trip mismatch expected text")?;
        let actual = serde_json::to_string(row.rendered().unwrap_or_default())
            .context("encoding English-v2 round-trip mismatch actual text")?;
        writeln!(
            output,
            "mismatch id={id} face={face} expected={expected} actual={actual}",
        )
        .context("writing English-v2 round-trip mismatch")?;
    }

    let summary = report.summary();
    writeln!(output, "English v2 accepted-set round trip")
        .context("writing English-v2 round-trip summary")?;
    writeln!(
        output,
        "  parse accepted         {}",
        report.accepted_ids().len()
    )
    .context("writing English-v2 round-trip summary")?;
    writeln!(output, "  clean                  {}", summary.clean)
        .context("writing English-v2 round-trip summary")?;
    writeln!(output, "  mismatched             {}", summary.mismatched)
        .context("writing English-v2 round-trip summary")?;
    writeln!(
        output,
        "  not parse accepted     {}",
        summary.parse_failures + summary.ambiguous + summary.internal_failures,
    )
    .context("writing English-v2 round-trip summary")?;
    Ok(())
}

fn require_clean(report: &AuditReport) -> anyhow::Result<()> {
    let mismatched = report.summary().mismatched;
    if mismatched == 0 {
        return Ok(());
    }

    bail!(
        "accepted parse round trip is not clean: {}",
        counted(mismatched, "mismatch", "mismatches"),
    )
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
    use super::render_report;
    use super::require_clean;
    use crate::english_v2::audit::AuditReport;
    use crate::english_v2::audit::AuditStatus;

    fn complete_fixture_report() -> AuditReport {
        AuditReport::from_rows_for_test(&[
            (
                AuditStatus::Clean,
                "Fixture 1",
                "Fixture 1 text.",
                Some("Fixture 1 text."),
            ),
            (
                AuditStatus::Mismatch,
                "Fixture 2",
                "Fixture 2 expected.\nsecond\tcolumn",
                Some("Different \"fixture\" 2 text.\\"),
            ),
            (
                AuditStatus::Mismatch,
                "Fixture 3",
                "Fixture 3 expected.",
                Some("Different fixture 3 text."),
            ),
            (
                AuditStatus::ParseFailure,
                "Fixture 4",
                "Fixture 4 text.",
                None,
            ),
            (AuditStatus::Ambiguous, "Fixture 5", "Fixture 5 text.", None),
            (
                AuditStatus::InternalFailure,
                "Fixture 6",
                "Fixture 6 text.",
                None,
            ),
        ])
    }

    #[test]
    fn render_report_keeps_every_mismatch_single_line_and_all_json_rows() {
        let report = complete_fixture_report();
        let mut human = Vec::new();

        render_report(&report, false, &mut human).unwrap();
        let output = String::from_utf8(human).unwrap();

        assert_eq!(
            output,
            concat!(
                "mismatch id=\"0000000000000000000000000000000000000000000000000000000000000002\" ",
                "face=\"Fixture 2\" expected=\"Fixture 2 expected.\\nsecond\\tcolumn\" ",
                "actual=\"Different \\\"fixture\\\" 2 text.\\\\\"\n",
                "mismatch id=\"0000000000000000000000000000000000000000000000000000000000000003\" ",
                "face=\"Fixture 3\" expected=\"Fixture 3 expected.\" ",
                "actual=\"Different fixture 3 text.\"\n",
                "English v2 accepted-set round trip\n",
                "  parse accepted         3\n",
                "  clean                  1\n",
                "  mismatched             2\n",
                "  not parse accepted     3\n",
            )
        );

        assert_eq!(output.matches("mismatch id=").count(), 2);
        assert!(!output.contains("Fixture 2 expected.\nsecond\tcolumn"));

        let mut json = Vec::new();
        render_report(&report, true, &mut json).unwrap();
        let json: serde_json::Value = serde_json::from_slice(&json).unwrap();
        assert_eq!(json["rows"].as_array().unwrap().len(), 6);
        assert_eq!(json["rows"][1]["status"], "mismatch");
        assert_eq!(
            json["rows"][1]["text"],
            "Fixture 2 expected.\nsecond\tcolumn"
        );
        assert_eq!(
            json["rows"][1]["rendered"],
            "Different \"fixture\" 2 text.\\"
        );
        assert_eq!(json["rows"][2]["status"], "mismatch");
        assert!(
            json["rows"]
                .as_array()
                .unwrap()
                .iter()
                .all(|row| row.get("message").is_none()),
            "roundtrip JSON must not emit meaningless null message fields"
        );
        assert_eq!(json["summary"]["parse_failures"], 1);
        assert_eq!(json["summary"]["ambiguous"], 1);
        assert_eq!(json["summary"]["internal_failures"], 1);
    }

    #[test]
    fn require_clean_rejects_only_accepted_render_mismatches() {
        let clean = AuditReport::from_statuses_for_test(&[AuditStatus::Clean]);
        let nonaccepted = AuditReport::from_statuses_for_test(&[
            AuditStatus::ParseFailure,
            AuditStatus::Ambiguous,
            AuditStatus::InternalFailure,
        ]);
        let mismatch = AuditReport::from_statuses_for_test(&[AuditStatus::Mismatch]);

        assert!(require_clean(&clean).is_ok());
        assert!(require_clean(&nonaccepted).is_ok());
        assert_eq!(
            require_clean(&mismatch).unwrap_err().to_string(),
            "accepted parse round trip is not clean: 1 mismatch"
        );
    }
}
