use anyhow::Result;
use anyhow::bail;
use clap::Args;
use deckmaste_english::ParseReport;
use deckmaste_english::parse_with_identity;
use deckmaste_english::syntax::LexicalOpacityKind;
use deckmaste_english::syntax::RecoveryRole;
use serde::Serialize;

use crate::english::data::OracleDataArgs;
use crate::english::data::map_supported_faces_with_workers;
use crate::english::data::supported_face_jobs;

#[derive(Debug, Args)]
pub(super) struct RecoveryArgs {
    #[command(flatten)]
    data: OracleDataArgs,

    /// Emit the census as JSON instead of a human-readable table.
    #[arg(long)]
    json: bool,

    /// Fail unless the supported corpus has no structural recovery.
    #[arg(long)]
    require_complete: bool,

    /// Maximum parser workers for this corpus run (reported with the result).
    #[arg(long, default_value_t = 4)]
    workers: usize,
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize)]
struct Counts {
    occurrences: usize,
    source_tokens: usize,
    maximum_source_tokens: usize,
}

impl Counts {
    fn observe(&mut self, source_tokens: usize) {
        self.occurrences += 1;
        self.source_tokens += source_tokens;
        self.maximum_source_tokens = self.maximum_source_tokens.max(source_tokens);
    }

    fn add(&mut self, other: Self) {
        self.occurrences += other.occurrences;
        self.source_tokens += other.source_tokens;
        self.maximum_source_tokens = self.maximum_source_tokens.max(other.maximum_source_tokens);
    }
}

#[derive(Debug, Default, PartialEq, Eq, Serialize)]
struct RecoveryCounts {
    total: Counts,
    clause: Counts,
    nominal: Counts,
    activation_cost: Counts,
    keyword_argument: Counts,
    modal_header: Counts,
    embedded_rules: Counts,
}

impl RecoveryCounts {
    fn role_mut(&mut self, role: RecoveryRole) -> &mut Counts {
        match role {
            RecoveryRole::Clause => &mut self.clause,
            RecoveryRole::NominalComplement => &mut self.nominal,
            RecoveryRole::ActivationCost => &mut self.activation_cost,
            RecoveryRole::KeywordArgument => &mut self.keyword_argument,
            RecoveryRole::ModalHeader => &mut self.modal_header,
            RecoveryRole::EmbeddedRules => &mut self.embedded_rules,
        }
    }

    fn observe(&mut self, role: RecoveryRole, source_tokens: usize) {
        self.total.observe(source_tokens);
        self.role_mut(role).observe(source_tokens);
    }

    fn add(&mut self, other: &Self) {
        self.total.add(other.total);
        self.clause.add(other.clause);
        self.nominal.add(other.nominal);
        self.activation_cost.add(other.activation_cost);
        self.keyword_argument.add(other.keyword_argument);
        self.modal_header.add(other.modal_header);
        self.embedded_rules.add(other.embedded_rules);
    }
}

#[derive(Debug, Default, PartialEq, Eq, Serialize)]
struct LexicalOpacityCounts {
    total: Counts,
    noun: Counts,
    flavor_header: Counts,
}

/// Packed-forest ambiguity retained by the selected parses.
///
/// Equal-cost alternatives are measured rather than rejected: declared
/// dominance and stable construction identity still make selection
/// deterministic. The census keeps both the number of tied selections and the
/// widest packed set visible for review.
#[derive(Debug, Default, PartialEq, Eq, Serialize)]
struct AmbiguityCounts {
    selections: usize,
    tied_selections: usize,
    tied_alternatives: usize,
    maximum_tied_alternatives: usize,
    maximum_forest_alternatives: usize,
}

impl AmbiguityCounts {
    fn observe(&mut self, tied_alternatives: usize, forest_alternatives: usize) {
        self.selections += 1;
        if tied_alternatives > 1 {
            self.tied_selections += 1;
            self.tied_alternatives += tied_alternatives;
            self.maximum_tied_alternatives = self.maximum_tied_alternatives.max(tied_alternatives);
        }
        self.maximum_forest_alternatives =
            self.maximum_forest_alternatives.max(forest_alternatives);
    }

    fn add(&mut self, other: &Self) {
        self.selections += other.selections;
        self.tied_selections += other.tied_selections;
        self.tied_alternatives += other.tied_alternatives;
        self.maximum_tied_alternatives = self
            .maximum_tied_alternatives
            .max(other.maximum_tied_alternatives);
        self.maximum_forest_alternatives = self
            .maximum_forest_alternatives
            .max(other.maximum_forest_alternatives);
    }
}

impl LexicalOpacityCounts {
    fn observe(&mut self, kind: LexicalOpacityKind, source_tokens: usize) {
        self.total.observe(source_tokens);
        match kind {
            LexicalOpacityKind::Noun => self.noun.observe(source_tokens),
            LexicalOpacityKind::FlavorHeader => self.flavor_header.observe(source_tokens),
        }
    }

    fn add(&mut self, other: &Self) {
        self.total.add(other.total);
        self.noun.add(other.noun);
        self.flavor_header.add(other.flavor_header);
    }
}

#[derive(Debug, Default, PartialEq, Eq, Serialize)]
struct Census {
    workers: usize,
    supported_faces: usize,
    source_tokens: usize,
    recovery: RecoveryCounts,
    lexical_opacity: LexicalOpacityCounts,
    ambiguity: AmbiguityCounts,
}

impl Census {
    fn observe_report(&mut self, report: &ParseReport) {
        self.supported_faces += 1;
        self.source_tokens += report.source_tokens();
        for recovery in report.ast().recoveries() {
            self.recovery.observe(recovery.role, recovery.source_tokens);
        }
        for opaque in report.ast().lexical_opacity() {
            self.lexical_opacity
                .observe(opaque.kind, opaque.source_tokens);
        }
        for selection in report.provenance().selections() {
            self.ambiguity.observe(
                selection.tied_alternatives().len(),
                selection.forest_stats().max_alternatives(),
            );
        }
    }

    fn add(&mut self, other: &Self) {
        self.supported_faces += other.supported_faces;
        self.source_tokens += other.source_tokens;
        self.recovery.add(&other.recovery);
        self.lexical_opacity.add(&other.lexical_opacity);
        self.ambiguity.add(&other.ambiguity);
    }
}

pub(super) fn run(args: &RecoveryArgs) -> Result<()> {
    if args.workers == 0 {
        bail!("--workers must be at least 1");
    }
    let data = args.data.load()?;
    let workers = supported_face_jobs(
        args.workers,
        data.faces.iter().filter(|card| card.supported).count(),
    );
    let census = map_supported_faces_with_workers(&data.faces, args.workers, |_, card| {
        let report = parse_with_identity(
            &card.oracle_text,
            &data.catalogs,
            card.printed_name(),
            card.is_legendary,
        );
        let mut census = Census::default();
        census.observe_report(&report);
        census
    })
    .into_iter()
    .fold(
        Census {
            workers,
            ..Census::default()
        },
        |mut census, card| {
            census.add(&card);
            census
        },
    );

    if args.json {
        println!("{}", serde_json::to_string_pretty(&census)?);
    } else {
        print_human(&census);
    }

    if args.require_complete && census.recovery.total.occurrences != 0 {
        bail!(
            "structural English recovery remains: {} spans covering {} source tokens",
            census.recovery.total.occurrences,
            census.recovery.total.source_tokens
        );
    }
    Ok(())
}

fn print_human(census: &Census) {
    println!(
        "audited {} supported faces ({} source tokens) with {} parser worker(s)",
        census.supported_faces, census.source_tokens, census.workers
    );
    println!("structural recovery:");
    print_counts("clause", census.recovery.clause);
    print_counts("nominal", census.recovery.nominal);
    print_counts("activation cost", census.recovery.activation_cost);
    print_counts("keyword argument", census.recovery.keyword_argument);
    print_counts("modal header", census.recovery.modal_header);
    print_counts("embedded rules", census.recovery.embedded_rules);
    print_counts("total", census.recovery.total);
    println!("licensed lexical opacity:");
    print_counts("noun", census.lexical_opacity.noun);
    print_counts("flavor header", census.lexical_opacity.flavor_header);
    print_counts("total", census.lexical_opacity.total);
    println!("packed ambiguity:");
    println!("  selections         {:>6}", census.ambiguity.selections);
    println!(
        "  tied selections    {:>6}",
        census.ambiguity.tied_selections
    );
    println!(
        "  tied alternatives  {:>6}",
        census.ambiguity.tied_alternatives
    );
    println!(
        "  max tied width      {:>6}",
        census.ambiguity.maximum_tied_alternatives
    );
    println!(
        "  max forest width    {:>6}",
        census.ambiguity.maximum_forest_alternatives
    );
}

fn print_counts(label: &str, counts: Counts) {
    println!(
        "  {label:<18} {:>6} spans  {:>8} source tokens  max {:>3}",
        counts.occurrences, counts.source_tokens, counts.maximum_source_tokens
    );
}

#[cfg(test)]
mod tests {
    use deckmaste_english::parse;

    use super::*;

    #[test]
    fn census_separates_clause_recovery_from_opaque_nouns() {
        let mut census = Census::default();
        census.observe_report(&parse("You frobnitz a card."));
        census.observe_report(&parse("Draw a blorple."));

        assert_eq!(census.supported_faces, 2);
        assert_eq!(census.recovery.total.occurrences, 1);
        assert_eq!(census.recovery.clause.source_tokens, 5);
        assert_eq!(census.lexical_opacity.noun.occurrences, 1);
        assert_eq!(census.lexical_opacity.noun.source_tokens, 1);
    }

    #[test]
    fn census_reports_flavor_header_opacity_without_clause_recovery() {
        let mut census = Census::default();
        census.observe_report(&parse("Zorbo Rampage! — Draw a card."));

        assert_eq!(census.recovery.total.occurrences, 0);
        assert_eq!(census.lexical_opacity.flavor_header.occurrences, 1);
        assert_eq!(census.lexical_opacity.flavor_header.source_tokens, 3);
        assert_eq!(census.lexical_opacity.noun.occurrences, 0);
        assert_eq!(census.lexical_opacity.total.occurrences, 1);
    }

    #[test]
    fn ambiguity_census_counts_tied_selections_and_maxima() {
        let mut census = AmbiguityCounts::default();
        census.observe(1, 4);
        census.observe(3, 7);
        census.observe(2, 5);

        assert_eq!(census.selections, 3);
        assert_eq!(census.tied_selections, 2);
        assert_eq!(census.tied_alternatives, 5);
        assert_eq!(census.maximum_tied_alternatives, 3);
        assert_eq!(census.maximum_forest_alternatives, 7);
    }
}
