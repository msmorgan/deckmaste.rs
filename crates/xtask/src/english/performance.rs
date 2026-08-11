//! Single-input parser work audit.
//!
//! Wall time and peak RSS are deliberately measured by an already-built
//! `cargo-xtask` process (documented in `docs/english-parser-performance.md`).
//! Keeping compilation outside that process makes the external measurement
//! meaningful; this command supplies the deterministic work counters that a
//! cross-machine comparison can actually gate.

use std::fs::File;
use std::io;
use std::io::Write;
use std::path::PathBuf;
use std::time::Instant;

use anyhow::Context;
use anyhow::Result;
use anyhow::bail;
use clap::Args;
use deckmaste_english::Catalogs;
use deckmaste_english::ParseReport;
use deckmaste_english::parse_with_identity;
use serde::Deserialize;
use serde::Serialize;

use super::data::CardFace;
use super::data::OracleDataArgs;
use super::inspect::find_cards;

const SHORT_INPUT: &str = "Draw a card.";
const STRESS_FACE: &str = "Ballroom Brawlers";

#[derive(Debug, Args)]
pub(super) struct PerformanceArgs {
    #[command(flatten)]
    data: OracleDataArgs,

    /// Emit the audit as JSON, suitable for a parent/current comparison.
    #[arg(long)]
    json: bool,

    /// A JSON audit produced by the parent change to compare against.
    #[arg(long, value_name = "PATH")]
    baseline: Option<PathBuf>,

    /// Reject deterministic work that exceeds the reviewed growth allowance.
    #[arg(long)]
    check: bool,

    /// Reviewed percentage allowance above each parent work counter.
    #[arg(long, default_value_t = 10)]
    max_work_growth_percent: usize,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
struct PerformanceAudit {
    inputs: Vec<InputAudit>,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
struct InputAudit {
    label: String,
    source_tokens: usize,
    elapsed_millis: u128,
    work: WorkCounters,
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Deserialize, Serialize)]
struct WorkCounters {
    chart_unique_items: usize,
    chart_max_column_width: usize,
    forest_constituent_nodes: usize,
    forest_intermediate_nodes: usize,
    forest_packed_alternatives: usize,
    forest_max_alternatives: usize,
}

impl WorkCounters {
    fn observe(&mut self, report: &ParseReport) {
        let work = report.work();
        self.chart_unique_items += work.chart_unique_items();
        self.chart_max_column_width = self
            .chart_max_column_width
            .max(work.chart_max_column_width());
        self.forest_constituent_nodes += work.forest_constituent_nodes();
        self.forest_intermediate_nodes += work.forest_intermediate_nodes();
        self.forest_packed_alternatives += work.forest_packed_alternatives();
        self.forest_max_alternatives = self
            .forest_max_alternatives
            .max(work.forest_max_alternatives());
    }

    fn checked_dimensions(self) -> [(&'static str, usize); 6] {
        [
            ("chart_unique_items", self.chart_unique_items),
            ("chart_max_column_width", self.chart_max_column_width),
            ("forest_constituent_nodes", self.forest_constituent_nodes),
            ("forest_intermediate_nodes", self.forest_intermediate_nodes),
            (
                "forest_packed_alternatives",
                self.forest_packed_alternatives,
            ),
            ("forest_max_alternatives", self.forest_max_alternatives),
        ]
    }
}

pub(super) fn run(args: &PerformanceArgs) -> Result<()> {
    let data = args.data.load()?;
    let stress_face = stress_face(&data.faces)?;
    let current = PerformanceAudit {
        inputs: vec![
            audit_text("short input", SHORT_INPUT, &data.catalogs, "", false),
            audit_face(&stress_face, &data.catalogs),
        ],
    };

    if args.json {
        write_json_audit(io::stdout().lock(), &current)?;
    } else {
        print_human(&current);
    }

    if args.check {
        let Some(baseline_path) = args.baseline.as_ref() else {
            bail!("--check requires --baseline PATH");
        };
        let baseline = serde_json::from_reader(File::open(baseline_path).with_context(|| {
            format!(
                "could not open performance baseline {}",
                baseline_path.display()
            )
        })?)
        .with_context(|| {
            format!(
                "could not parse performance baseline {}",
                baseline_path.display()
            )
        })?;
        check_against(&baseline, &current, args.max_work_growth_percent)?;
        write_check_success(
            io::stderr().lock(),
            baseline_path,
            args.max_work_growth_percent,
        )?;
    }
    Ok(())
}

/// Write the machine-readable audit to stdout without any gate-status prose.
///
/// A parent/current command redirects stdout into its next JSON baseline, while
/// a check result belongs on stderr with the timing diagnostics.
fn write_json_audit(mut writer: impl Write, audit: &PerformanceAudit) -> Result<()> {
    serde_json::to_writer_pretty(&mut writer, audit)?;
    writeln!(writer)?;
    Ok(())
}

fn write_check_success(
    mut writer: impl Write,
    baseline_path: &std::path::Path,
    allowance_percent: usize,
) -> Result<()> {
    writeln!(
        writer,
        "performance check passed against {} with {allowance_percent}% reviewed work allowance",
        baseline_path.display(),
    )?;
    Ok(())
}

fn stress_face(faces: &[CardFace]) -> Result<CardFace> {
    find_cards(faces, STRESS_FACE)
        .into_iter()
        .find(|face| face.supported)
        .ok_or_else(|| {
            anyhow::anyhow!("no supported face named {STRESS_FACE:?} in the Oracle snapshot")
        })
}

fn audit_face(card: &CardFace, catalogs: &Catalogs) -> InputAudit {
    audit_text(
        card.printed_name(),
        &card.oracle_text,
        catalogs,
        card.printed_name(),
        card.is_legendary,
    )
}

fn audit_text(
    label: &str,
    source: &str,
    catalogs: &Catalogs,
    name: &str,
    is_legendary: bool,
) -> InputAudit {
    let started = Instant::now();
    let report = parse_with_identity(source, catalogs, name, is_legendary);
    let elapsed_millis = started.elapsed().as_millis();
    let mut work = WorkCounters::default();
    work.observe(&report);
    InputAudit {
        label: label.to_owned(),
        source_tokens: report.source_tokens(),
        elapsed_millis,
        work,
    }
}

fn check_against(
    baseline: &PerformanceAudit,
    current: &PerformanceAudit,
    allowance_percent: usize,
) -> Result<()> {
    for current_input in &current.inputs {
        let baseline_input = baseline
            .inputs
            .iter()
            .find(|input| input.label == current_input.label)
            .ok_or_else(|| {
                anyhow::anyhow!("baseline is missing input {:?}", current_input.label)
            })?;
        for ((dimension, current_value), (_, baseline_value)) in current_input
            .work
            .checked_dimensions()
            .into_iter()
            .zip(baseline_input.work.checked_dimensions())
        {
            let allowance = baseline_value.saturating_mul(allowance_percent) / 100;
            let maximum = baseline_value.saturating_add(allowance);
            if current_value > maximum {
                bail!(
                    "{} {dimension} regressed from {baseline_value} to {current_value}; reviewed allowance is {maximum} ({allowance_percent}%)",
                    current_input.label
                );
            }
        }
    }
    Ok(())
}

fn print_human(audit: &PerformanceAudit) {
    println!("single-input parser audit (run before corpus concurrency):");
    for input in &audit.inputs {
        println!("{}:", input.label);
        println!("  source tokens             {}", input.source_tokens);
        println!("  parser wall milliseconds  {}", input.elapsed_millis);
        for (dimension, value) in input.work.checked_dimensions() {
            println!("  {dimension:<25} {value}");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn audit(label: &str, work: WorkCounters) -> PerformanceAudit {
        PerformanceAudit {
            inputs: vec![InputAudit {
                label: label.to_owned(),
                source_tokens: 1,
                elapsed_millis: 0,
                work,
            }],
        }
    }

    #[test]
    fn single_input_audit_observes_chart_work_from_a_real_parse() {
        let audit = audit_text("short input", SHORT_INPUT, &Catalogs::default(), "", false);

        assert_eq!(audit.source_tokens, 4);
        assert!(
            audit.work.chart_unique_items > 0,
            "the audit must expose work the chart parser actually performed"
        );
        assert!(
            audit.work.chart_max_column_width > 0,
            "the audit must expose the chart's maximum column width"
        );
    }

    #[test]
    fn predicate_activation_stays_within_reviewed_short_input_work_growth() {
        // Mutation caught: activate declaration-owned predicate productions
        // without pruning their generated chart fan-out back to the
        // pre-activation work envelope.
        //
        // The clause-coordination retrofit increased the clause declaration
        // from 20 to 39 rows. On this input, constituent nodes remain 59 and
        // maximum alternatives remain 1; intermediate nodes and packed
        // alternatives each increase by exactly 55. That paired increase is
        // the deterministic forest structure for the newly predicted rows,
        // not new ambiguity. Avoiding it would require chart-level factoring
        // across generated constructions rather than a narrower clause rule.
        let parent = audit(
            "short input",
            WorkCounters {
                chart_unique_items: 936,
                chart_max_column_width: 381,
                forest_constituent_nodes: 59,
                forest_intermediate_nodes: 400,
                forest_packed_alternatives: 459,
                forest_max_alternatives: 1,
            },
        );
        let current = PerformanceAudit {
            inputs: vec![audit_text(
                "short input",
                SHORT_INPUT,
                &Catalogs::default(),
                "",
                false,
            )],
        };

        check_against(&parent, &current, 10)
            .expect("predicate activation must stay within reviewed deterministic work growth");
    }

    #[test]
    fn abandoned_chart_attempt_increases_gated_work() {
        let source = "Suspend 3, definitely not a keyword.";
        let catalogs = Catalogs::default()
            .with_catalog(deckmaste_english::CatalogKind::KeywordAbility, ["Suspend"]);
        let report = parse_with_identity(source, &catalogs, "", false);
        let accepted_chart_items = report
            .provenance()
            .selections()
            .iter()
            .map(|selection| selection.chart_stats().unique_items())
            .sum::<usize>();
        let current_input = audit_text("abandoned branch", source, &catalogs, "", false);

        assert!(
            current_input.work.chart_unique_items > accepted_chart_items,
            "abandoned chart work must exceed the accepted-only provenance control: total={} accepted={accepted_chart_items}",
            current_input.work.chart_unique_items,
        );

        let mut accepted_only = current_input.work;
        accepted_only.chart_unique_items = accepted_chart_items;
        let parent = audit("abandoned branch", accepted_only);
        let current = PerformanceAudit {
            inputs: vec![current_input],
        };
        let error = check_against(&parent, &current, 0).unwrap_err();
        assert!(error.to_string().contains("chart_unique_items regressed"));
    }

    #[test]
    fn json_audit_stdout_remains_a_parseable_baseline_after_a_clean_check() {
        let audit = audit(
            "stress",
            WorkCounters {
                chart_unique_items: 100,
                ..WorkCounters::default()
            },
        );
        let mut stdout = Vec::new();
        write_json_audit(&mut stdout, &audit).unwrap();
        let mut stderr = Vec::new();
        write_check_success(&mut stderr, std::path::Path::new("parent.json"), 10).unwrap();

        let decoded: PerformanceAudit = serde_json::from_slice(&stdout).unwrap();
        assert_eq!(decoded, audit);
        assert!(
            !String::from_utf8(stdout)
                .unwrap()
                .contains("performance check passed"),
            "the redirected JSON baseline must contain no check-status prose"
        );
        assert!(
            String::from_utf8(stderr)
                .unwrap()
                .contains("performance check passed"),
            "the check-status prose belongs with stderr timing diagnostics"
        );
    }

    #[test]
    fn unchanged_parent_work_passes_the_checked_gate() {
        let parent = audit(
            "stress",
            WorkCounters {
                chart_unique_items: 100,
                chart_max_column_width: 10,
                ..WorkCounters::default()
            },
        );
        assert!(check_against(&parent, &parent, 10).is_ok());
    }

    #[test]
    fn reviewed_regression_fixture_is_rejected_by_the_checked_gate() {
        let parent = audit(
            "stress",
            WorkCounters {
                chart_unique_items: 100,
                chart_max_column_width: 10,
                ..WorkCounters::default()
            },
        );
        let regressed = audit(
            "stress",
            WorkCounters {
                chart_unique_items: 111,
                chart_max_column_width: 10,
                ..WorkCounters::default()
            },
        );
        let error = check_against(&parent, &regressed, 10).unwrap_err();
        assert!(error.to_string().contains("chart_unique_items regressed"));
    }
}
