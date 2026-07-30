//! `cargo xtask english probe` — ways to run the existing parser and report
//! what it does; nothing here synthesizes text of its own.
//!
//! `--text` is a free-text entry point: every other english subcommand that
//! examines a parse (`bracket`, `inspect`) is card-keyed, taking a name
//! already in the Oracle-text snapshot, so this is the only way to hand the
//! parser a literal string that was never printed. `--calibrate` derives the
//! invariant-5 ceilings (`max_alternatives`, `constituent_nodes`) from the
//! printed corpus, at the 99.9th percentile. `--printed` runs the structural
//! laws over printed faces, unmodified — its purpose is to establish whether
//! a law discriminates on printed text at all before its findings are
//! trusted elsewhere: a law that fires just as often on ordinary printed
//! cards as on whatever it was meant to catch is measuring something true of
//! the grammar in general, not a genuine defect. Running exactly this is
//! what invalidated `no_tie` as a defect signal — see its doc comment in
//! `invariants.rs`.

mod calibrate;
mod facts;
mod invariants;

use std::collections::BTreeMap;
use std::collections::BTreeSet;

use anyhow::Result;
use clap::Args;
use invariants::Finding;
use invariants::Thresholds;

use super::data::OracleData;
use super::data::OracleDataArgs;
use super::data::map_supported_faces;

#[derive(Debug, Args)]
pub(super) struct ProbeArgs {
    /// Parse one literal Oracle-text string.
    #[arg(long, value_name = "TEXT")]
    text: Option<String>,

    /// Print the invariant-5 ceilings derived from the printed corpus, then
    /// exit.
    #[arg(long, conflicts_with = "text")]
    calibrate: bool,

    /// Run the structural laws over PRINTED faces, unmodified.
    ///
    /// This is how a law's printed-text baseline gets established before its
    /// findings elsewhere are trusted: if a law fires just as often on
    /// ordinary, unmodified printed cards as it does on whatever it was
    /// meant to catch, the law is measuring something true of the grammar in
    /// general, not a genuine defect. Running exactly this is what
    /// invalidated `no_tie` as a defect signal — see its doc comment in
    /// `invariants.rs`.
    #[arg(long, conflicts_with_all = ["text", "calibrate"])]
    printed: bool,

    /// Override the calibrated `max_alternatives` ceiling. Only takes effect
    /// when `--constituent-nodes` is also given — thresholds are calibrated
    /// as a package, never a calibrated/overridden mix.
    #[arg(long)]
    max_alternatives: Option<usize>,

    /// Override the calibrated `constituent_nodes` ceiling. Only takes effect
    /// when `--max-alternatives` is also given.
    #[arg(long)]
    constituent_nodes: Option<usize>,

    /// Exit non-zero if any gating law is violated.
    #[arg(long)]
    deny: bool,

    #[command(flatten)]
    data: OracleDataArgs,
}

/// A group of findings sharing a law and a signature — one ticket's worth of
/// work.
///
/// Private, not `pub(super)`: [`Finding`] itself is only visible within this
/// module (`invariants::Finding` is `pub(super)` there, i.e. scoped to
/// `probe`), and nothing outside `probe` uses `Cluster` — the English module
/// only reaches this subcommand through [`ProbeArgs`] and [`run`].
#[derive(Debug, Clone, PartialEq, Eq)]
struct Cluster {
    law: &'static str,
    signature: String,
    count: usize,
    examples: Vec<String>,
    /// Distinct carriers behind `count` instances — see [`cluster`].
    distinct_carriers: usize,
}

/// Group by the law and the signature the finding was born with.
///
/// `detail` is never inspected — clustering must not depend on display
/// wording.
///
/// `distinct_carriers` counts distinct values of a finding's provenance key
/// (the tuple's second element), not raw `count`. In `--printed` mode that
/// key is the card name itself, so `distinct_carriers` is the number of
/// distinct cards behind a cluster, while `count` is the number of finding
/// instances (a single card can contribute more than one, e.g. two separate
/// rule-176 ties in one multi-sentence card).
fn cluster(findings: Vec<(String, String, Finding)>) -> Vec<Cluster> {
    // Running count, capped examples, and distinct-provenance set for one
    // (law, signature) key.
    type Accumulator = (usize, Vec<String>, BTreeSet<String>);

    let mut grouped: BTreeMap<(&'static str, String), Accumulator> = BTreeMap::new();
    for (text, provenance, finding) in findings {
        let key = (finding.law, finding.signature);
        let entry = grouped
            .entry(key)
            .or_insert_with(|| (0, Vec::new(), BTreeSet::new()));
        entry.0 += 1;
        // Deduplicated: the same text (in `--printed` mode, the same card)
        // can produce several findings under one (law, signature) — e.g. two
        // separate rule-176 ties in one multi-sentence card — and showing it
        // twice would waste a slot instead of illustrating the cluster.
        if entry.1.len() < 3 && !entry.1.contains(&text) {
            entry.1.push(text);
        }
        entry.2.insert(provenance);
    }
    grouped
        .into_iter()
        .map(|((law, signature), (count, examples, carriers))| Cluster {
            law,
            signature,
            count,
            examples,
            distinct_carriers: carriers.len(),
        })
        .collect()
}

/// Printed once before `no_tie`'s clusters — the number is a measurement,
/// not a running computation, because establishing it means parsing the
/// whole printed corpus (see `--printed`'s doc comment); it is not something
/// re-derived on every run. See [`invariants::no_tie`]'s doc comment for how
/// this was measured and why it means `no_tie` can never be a defect signal.
const BASELINE_RELATIVE_HEADER: &str = "baseline_relative (NOT defects: 93.07% of 31,685 printed faces also tie; \
     see the no_tie doc comment)";

/// Print a law's cluster list, each with up to three example texts. In
/// `--printed` mode an example is a card name and stands on its own; a
/// newline inside one is rendered visibly.
fn report(clusters: &[Cluster]) {
    for group in clusters {
        println!(
            "\n{} [{}] × {} ({} distinct carriers)",
            group.law, group.signature, group.count, group.distinct_carriers
        );
        for example in &group.examples {
            println!("  {}", example.replace('\n', " ⏎ "));
        }
    }
}

/// One printed face's contribution to the `--printed` baseline: whether it
/// ties at all (the headline question), its baseline-relative (`no_tie`)
/// findings, and its gating (`forest_growth`) findings — kept separate so
/// the caller never has to re-sort them back apart.
struct FaceResult {
    has_tie: bool,
    ties: Vec<(String, String, Finding)>,
    gating: Vec<(String, String, Finding)>,
}

/// `part` as a percentage of `whole`, or 0.0 for an empty corpus.
#[expect(
    clippy::cast_precision_loss,
    reason = "part and whole are corpus-sized face counts, far below f64's 2^53 exact-integer range"
)]
fn percentage_of(part: usize, whole: usize) -> f64 {
    if whole == 0 { 0.0 } else { (part as f64 / whole as f64) * 100.0 }
}

/// Do printed cards already violate a gating law on their own, before any
/// synthesis?
///
/// Only `forest_growth` gates here. `no_tie` still runs — this very mode is
/// in fact what measured its printed-corpus baseline, see its doc comment in
/// `invariants.rs` — but its findings are baseline-relative, never gating;
/// see [`BASELINE_RELATIVE_HEADER`]. `ability_independence` has nothing to
/// compare a face against (there is no solo re-parse distinct from the one
/// already being examined — every printed face *is* the baseline), and
/// `recovery_frontier` is already the whole subject of
/// `cargo xtask english recovery`, so repeating it here would just be a
/// slower way to ask a question this tool answers elsewhere.
fn run_printed(data: &OracleData, limits: &Thresholds, overridden: bool, deny: bool) -> Result<()> {
    let per_face = map_supported_faces(&data.faces, |_, card| {
        let facts = facts::facts_for(&card.oracle_text, &data.catalogs);
        let name = card.printed_name().to_string();
        let tie_findings = invariants::no_tie(&facts);
        let has_tie = !tie_findings.is_empty();
        let ties: Vec<_> = tie_findings
            .into_iter()
            .map(|violation| (name.clone(), name.clone(), violation))
            .collect();
        let gating: Vec<_> = invariants::forest_growth(&facts, limits)
            .into_iter()
            .map(|violation| (name.clone(), name.clone(), violation))
            .collect();
        FaceResult {
            has_tie,
            ties,
            gating,
        }
    });

    let checked = per_face.len();
    let faces_with_tie = per_face.iter().filter(|result| result.has_tie).count();
    let percentage = percentage_of(faces_with_tie, checked);

    let mut baseline_relative = Vec::new();
    let mut gating = Vec::new();
    for result in per_face {
        baseline_relative.extend(result.ties);
        gating.extend(result.gating);
    }

    println!("printed_checked\t{checked}");
    println!("printed_ties\t{faces_with_tie} ({percentage:.2}%)");
    println!(
        "ceilings\t{}max_alternatives={} constituent_nodes={}",
        if overridden { "OVERRIDDEN (not calibrated) " } else { "" },
        limits.max_alternatives,
        limits.constituent_nodes
    );

    let gating_clusters = cluster(gating);
    let baseline_relative_clusters = cluster(baseline_relative);
    println!("printed_gating_clusters\t{}", gating_clusters.len());
    println!(
        "printed_baseline_relative_clusters\t{}",
        baseline_relative_clusters.len()
    );

    report(&gating_clusters);

    println!("\n{BASELINE_RELATIVE_HEADER}");
    report(&baseline_relative_clusters);

    match baseline_relative_clusters
        .iter()
        .find(|group| group.law == "no-tie" && group.signature == "rule 176")
    {
        // `group.count` is tie *instances* (a face can tie more than once
        // under the same rule); `group.distinct_carriers` is the actual
        // face count, per `cluster`'s doc comment. Both are reported so
        // neither number is mistaken for the other.
        Some(group) => println!(
            "\nrule-176-on-printed-cards\t{} faces ({} tie instances total; examples: {})",
            group.distinct_carriers,
            group.count,
            group.examples.join(", ")
        ),
        None => {
            println!("\nrule-176-on-printed-cards\tabsent — no printed face ties under rule 176");
        }
    }

    if deny && !gating_clusters.is_empty() {
        anyhow::bail!(
            "{} printed-baseline gating cluster(s) violated; no_tie's baseline-relative \
             clusters are reported, not gated",
            gating_clusters.len()
        );
    }
    Ok(())
}

pub(super) fn run(args: &ProbeArgs) -> Result<()> {
    let data = args.data.load()?;

    if args.calibrate {
        let limits = calibrate::calibrate(&data);
        println!("max_alternatives\t{}", limits.max_alternatives);
        println!("constituent_nodes\t{}", limits.constituent_nodes);
        return Ok(());
    }

    if let Some(text) = args.text.as_deref() {
        let facts = facts::facts_for(text, &data.catalogs);
        println!("tokens\t{}", facts.source_tokens);
        println!("abilities\t{}", facts.abilities.len());
        println!("ties\t{}", facts.ties.len());
        println!("max_alternatives\t{}", facts.max_alternatives);
        println!("constituent_nodes\t{}", facts.constituent_nodes);
        println!("chart_items\t{}", facts.chart_items);
        println!("recovered\t{}", facts.recovered.len());
        return Ok(());
    }

    // Thresholds must never be hand-picked, so calibration from the printed
    // corpus is the only default source. It parses the whole corpus and
    // takes minutes, so both override flags exist for fast iteration — but
    // they are a package deal: giving only one would silently calibrate the
    // other, which would make an "overridden" run look half-calibrated
    // without saying so. Either flag absent means a real calibration run.
    let (limits, overridden) = match (args.max_alternatives, args.constituent_nodes) {
        (Some(max_alternatives), Some(constituent_nodes)) => (
            Thresholds {
                max_alternatives,
                constituent_nodes,
            },
            true,
        ),
        _ => (calibrate::calibrate(&data), false),
    };

    if args.printed {
        return run_printed(&data, &limits, overridden, args.deny);
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn finding(law: &'static str, signature: &str, detail: &str) -> Finding {
        Finding {
            law,
            signature: signature.to_string(),
            detail: detail.to_string(),
        }
    }

    #[test]
    fn clustering_groups_by_law_and_signature_not_by_instance() {
        let findings = vec![
            (
                "Draw a card.".to_string(),
                "recipe A".to_string(),
                finding("no-tie", "rule 7", "2 derivations tied at bytes 0..4"),
            ),
            (
                "You gain 1 life.".to_string(),
                "recipe B".to_string(),
                finding("no-tie", "rule 7", "2 derivations tied at bytes 9..13"),
            ),
            (
                "Draw a card.".to_string(),
                "recipe C".to_string(),
                finding("forest-growth", "max_alternatives", "99 exceeds ceiling 8"),
            ),
        ];
        let clusters = cluster(findings);
        assert_eq!(
            clusters.len(),
            2,
            "same law and same signature is one cluster, whatever the bytes"
        );
        let tie = clusters
            .iter()
            .find(|cluster| cluster.law == "no-tie")
            .expect("tie cluster");
        assert_eq!(tie.count, 2);
        assert_eq!(tie.examples.len(), 2);
    }

    #[test]
    fn clustering_separates_two_signatures_under_one_law() {
        let findings = vec![
            (
                "a".to_string(),
                "recipe".to_string(),
                finding("no-tie", "rule 3", "tied"),
            ),
            (
                "b".to_string(),
                "recipe".to_string(),
                finding("no-tie", "rule 9", "tied"),
            ),
        ];
        assert_eq!(
            cluster(findings).len(),
            2,
            "different productions are different tickets"
        );
    }

    #[test]
    fn clustering_keeps_at_most_three_examples() {
        let findings: Vec<_> = (0..10)
            .map(|index| {
                (
                    format!("text {index}"),
                    format!("recipe {index}"),
                    finding("no-tie", "rule 3", "tied"),
                )
            })
            .collect();
        let clusters = cluster(findings);
        assert_eq!(clusters.len(), 1);
        assert_eq!(clusters[0].count, 10, "the count is the whole cluster");
        assert_eq!(
            clusters[0].examples.len(),
            3,
            "examples are capped for readability"
        );
    }

    /// One host's recipe/provenance string encodes the host, never the
    /// donor, so one host repeated 1561 times under the same provenance key
    /// must collapse to one distinct carrier — not be reported as 1561
    /// independent findings.
    #[test]
    fn distinct_carriers_collapses_one_host_many_donors() {
        let findings: Vec<_> = (0..1561)
            .map(|index| {
                (
                    format!("Aang, donor {index}, and La attack"),
                    "extend `and` coordination".to_string(),
                    finding("no-tie", "rule 177", "tied"),
                )
            })
            .collect();
        let clusters = cluster(findings);
        assert_eq!(clusters.len(), 1);
        assert_eq!(clusters[0].count, 1561, "every instance is still counted");
        assert_eq!(
            clusters[0].distinct_carriers, 1,
            "one host spliced with 1561 different donors is one underlying finding"
        );
    }

    /// The other shape that must not break: genuinely different hosts (here,
    /// different provenance keys) must not collapse together just because
    /// they share a law and signature.
    #[test]
    fn distinct_carriers_reports_genuinely_varied_texts_as_many() {
        let findings: Vec<_> = (0..5)
            .map(|index| {
                (
                    format!("text {index}"),
                    format!("substitute `phrase {index}` with rule-3 fragment"),
                    finding("no-tie", "rule 3", "tied"),
                )
            })
            .collect();
        let clusters = cluster(findings);
        assert_eq!(clusters.len(), 1);
        assert_eq!(clusters[0].count, 5);
        assert_eq!(
            clusters[0].distinct_carriers, 5,
            "five different hosts must not collapse into one carrier"
        );
    }

    #[test]
    fn percentage_of_a_whole_corpus() {
        assert!((percentage_of(1561, 31_685) - 4.926_78).abs() < 0.001);
        assert!(percentage_of(0, 100).abs() < f64::EPSILON);
        assert!((percentage_of(100, 100) - 100.0).abs() < f64::EPSILON);
    }

    #[test]
    fn percentage_of_an_empty_corpus_is_zero_not_a_panic() {
        assert!(percentage_of(0, 0).abs() < f64::EPSILON);
    }
}
