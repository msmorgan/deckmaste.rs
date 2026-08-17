use std::io::Write;

use anyhow::Context;
use anyhow::bail;
use deckmaste_english_v2::catalogs::ParserCatalogs;
use deckmaste_english_v2::parser::Parser;
use serde::Serialize;

use super::RoundtripArgs;
use super::audit::AuditReport;
use super::audit::AuditRow;
use super::audit::AuditStatus;
use super::corpus::Corpus;

pub(super) fn run(args: &RoundtripArgs, output: &mut dyn Write) -> anyhow::Result<()> {
    let corpus = Corpus::load(&args.corpus.data)
        .with_context(|| format!("loading corpus from {}", args.corpus.data.display()))?;
    let catalogs = ParserCatalogs::load(&args.corpus.catalogs).with_context(|| {
        format!(
            "loading parser catalogs from {}",
            args.corpus.catalogs.display()
        )
    })?;
    let parser = Parser::new(catalogs);
    let report = AuditReport::run(&corpus, &parser);

    render_report(&report, args.json, output)?;
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
        writeln!(
            output,
            "mismatch\t{}\t{}\t{}\t{}",
            row.id(),
            row.printed_face(),
            row.text(),
            row.rendered().unwrap_or_default(),
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
        AuditReport::from_statuses_for_test(&[
            AuditStatus::Clean,
            AuditStatus::Mismatch,
            AuditStatus::ParseFailure,
            AuditStatus::Ambiguous,
            AuditStatus::InternalFailure,
        ])
    }

    #[test]
    fn render_report_enumerates_every_mismatch_and_keeps_all_rows_in_json() {
        let report = complete_fixture_report();
        let mut human = Vec::new();

        render_report(&report, false, &mut human).unwrap();

        assert_eq!(
            String::from_utf8(human).unwrap(),
            concat!(
                "mismatch\t",
                "0000000000000000000000000000000000000000000000000000000000000002\t",
                "Fixture 2\tFixture 2 text.\tDifferent fixture 2 text.\n",
                "English v2 accepted-set round trip\n",
                "  parse accepted         2\n",
                "  clean                  1\n",
                "  mismatched             1\n",
                "  not parse accepted     3\n",
            )
        );

        let mut json = Vec::new();
        render_report(&report, true, &mut json).unwrap();
        let json: serde_json::Value = serde_json::from_slice(&json).unwrap();
        assert_eq!(json["rows"].as_array().unwrap().len(), 5);
        assert_eq!(json["rows"][1]["status"], "mismatch");
        assert_eq!(json["rows"][1]["text"], "Fixture 2 text.");
        assert_eq!(json["rows"][1]["rendered"], "Different fixture 2 text.");
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
