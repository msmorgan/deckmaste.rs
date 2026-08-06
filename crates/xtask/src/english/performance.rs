//! Single-input parser work audit.
//!
//! Wall time and peak RSS are deliberately measured by an already-built
//! `cargo-xtask` process (documented in `docs/english-parser-performance.md`).
//! Keeping compilation outside that process makes the external measurement
//! meaningful; this command supplies the deterministic work counters that a
//! cross-machine comparison can actually gate.

use std::fs::File;
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
        for selection in report.provenance().selections() {
            let chart = selection.chart_stats();
            let forest = selection.forest_stats();
            self.chart_unique_items += chart.unique_items();
            self.chart_max_column_width = self.chart_max_column_width.max(chart.max_column_width());
            self.forest_constituent_nodes += forest.constituent_nodes();
            self.forest_intermediate_nodes += forest.intermediate_nodes();
            self.forest_packed_alternatives += forest.packed_alternatives();
            self.forest_max_alternatives =
                self.forest_max_alternatives.max(forest.max_alternatives());
        }
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
        println!("{}", serde_json::to_string_pretty(&current)?);
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
        println!(
            "performance check passed against {} with {}% reviewed work allowance",
            baseline_path.display(),
            args.max_work_growth_percent
        );
    }
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
