use std::collections::BTreeSet;
use std::collections::HashMap;
use std::path::PathBuf;

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
use crate::english::recovery_worklist::RuntimeRecoveryGroup;

#[derive(Debug, Args)]
pub(super) struct RecoveryArgs {
    #[command(flatten)]
    data: OracleDataArgs,

    /// Emit the census as JSON instead of a human-readable table.
    #[arg(long)]
    json: bool,

    /// List structural recovery grouped by role and exact recovered text.
    #[arg(long, conflicts_with = "json")]
    list: bool,

    /// Maximum grouped recovery rows to print (requires --list).
    #[arg(long, default_value_t = 25, requires = "list")]
    list_limit: usize,

    /// Export every exact-text recovery group to an annotatable JSON worklist.
    #[arg(
        long,
        value_name = "PATH",
        conflicts_with_all = ["json", "list", "verify_worklist"]
    )]
    worklist: Option<PathBuf>,

    /// Verify a previously exported and causally annotated worklist.
    #[arg(
        long,
        value_name = "PATH",
        conflicts_with_all = ["json", "list", "worklist"]
    )]
    verify_worklist: Option<PathBuf>,

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

const LISTED_FACE_NAMES: usize = 5;

#[derive(Debug, Clone, PartialEq, Eq)]
struct RecoveryGroup {
    role: RecoveryRole,
    text: String,
    occurrences: usize,
    source_tokens: usize,
    face_names: BTreeSet<String>,
}

impl RecoveryGroup {
    fn observe(&mut self, source_tokens: usize, face_name: &str) {
        self.occurrences += 1;
        self.source_tokens += source_tokens;
        self.face_names.insert(face_name.to_owned());
        while self.face_names.len() > LISTED_FACE_NAMES {
            self.face_names.pop_last();
        }
    }

    fn add(&mut self, other: Self) {
        self.occurrences += other.occurrences;
        self.source_tokens += other.source_tokens;
        self.face_names.extend(other.face_names);
        while self.face_names.len() > LISTED_FACE_NAMES {
            self.face_names.pop_last();
        }
    }
}

#[derive(Debug, Default)]
struct RecoveryListing {
    groups: HashMap<(RecoveryRole, String), RecoveryGroup>,
}

impl RecoveryListing {
    fn observe(&mut self, role: RecoveryRole, text: &str, source_tokens: usize, face_name: &str) {
        let key = (role, text.to_owned());
        let group = self.groups.entry(key).or_insert_with(|| RecoveryGroup {
            role,
            text: text.to_owned(),
            occurrences: 0,
            source_tokens: 0,
            face_names: BTreeSet::new(),
        });
        group.observe(source_tokens, face_name);
    }

    fn add(&mut self, other: Self) {
        for (key, group) in other.groups {
            match self.groups.entry(key) {
                std::collections::hash_map::Entry::Occupied(mut entry) => {
                    entry.get_mut().add(group);
                }
                std::collections::hash_map::Entry::Vacant(entry) => {
                    entry.insert(group);
                }
            }
        }
    }

    fn sorted_groups(&self) -> Vec<&RecoveryGroup> {
        let mut groups = self.groups.values().collect::<Vec<_>>();
        groups.sort_unstable_by(|left, right| {
            right
                .occurrences
                .cmp(&left.occurrences)
                .then_with(|| right.source_tokens.cmp(&left.source_tokens))
                .then_with(|| role_name(left.role).cmp(role_name(right.role)))
                .then_with(|| left.text.cmp(&right.text))
        });
        groups
    }

    fn runtime_groups(&self) -> Vec<RuntimeRecoveryGroup> {
        self.sorted_groups()
            .into_iter()
            .map(|group| RuntimeRecoveryGroup {
                role: role_name(group.role).to_owned(),
                text: group.text.clone(),
                occurrences: group.occurrences,
                source_tokens: group.source_tokens,
                face_names: group.face_names.iter().cloned().collect(),
            })
            .collect()
    }
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

#[derive(Debug, Default)]
struct FaceAudit {
    census: Census,
    recovery_listing: Option<RecoveryListing>,
}

impl FaceAudit {
    fn observe_report(&mut self, report: &ParseReport, face_name: &str, list: bool) {
        self.census.observe_report(report);
        if list {
            let listing = self
                .recovery_listing
                .get_or_insert_with(RecoveryListing::default);
            for recovery in report.ast().recoveries() {
                listing.observe(
                    recovery.role,
                    recovery.text,
                    recovery.source_tokens,
                    face_name,
                );
            }
        }
    }

    fn add(&mut self, other: Self) {
        self.census.add(&other.census);
        match (self.recovery_listing.as_mut(), other.recovery_listing) {
            (Some(listing), Some(other)) => listing.add(other),
            (None, Some(other)) => self.recovery_listing = Some(other),
            (_, None) => {}
        }
    }
}

pub(super) fn run(args: &RecoveryArgs) -> Result<()> {
    if args.workers == 0 {
        bail!("--workers must be at least 1");
    }
    if args.list && args.list_limit == 0 {
        bail!("--list-limit must be at least 1");
    }
    let data = args.data.load()?;
    let workers = supported_face_jobs(
        args.workers,
        data.faces.iter().filter(|card| card.supported).count(),
    );
    let audit = map_supported_faces_with_workers(&data.faces, args.workers, |_, card| {
        let report = parse_with_identity(
            &card.oracle_text,
            &data.catalogs,
            card.printed_name(),
            card.is_legendary,
        );
        let mut audit = FaceAudit::default();
        audit.observe_report(
            &report,
            card.printed_name(),
            args.list || args.worklist.is_some() || args.verify_worklist.is_some(),
        );
        audit
    })
    .into_iter()
    .fold(
        FaceAudit {
            census: Census {
                workers,
                ..Census::default()
            },
            recovery_listing: (args.list
                || args.worklist.is_some()
                || args.verify_worklist.is_some())
            .then(RecoveryListing::default),
        },
        |mut audit, card| {
            audit.add(card);
            audit
        },
    );
    let FaceAudit {
        census,
        recovery_listing,
    } = audit;

    if let Some(path) = args.worklist.as_deref() {
        let listing = recovery_listing
            .as_ref()
            .expect("worklist export requested recovery collection");
        let groups = listing.runtime_groups();
        std::fs::write(path, crate::english::recovery_worklist::export(&groups)?)?;
        println!(
            "exported {} recovery groups to {} (transient corpus worklist; do not commit)",
            groups.len(),
            path.display()
        );
    } else if let Some(path) = args.verify_worklist.as_deref() {
        let listing = recovery_listing
            .as_ref()
            .expect("worklist verification requested recovery collection");
        let summary = crate::english::recovery_worklist::verify(path, &listing.runtime_groups())?;
        println!(
            "verified {} audited recovery groups: {} implemented, {} assigned to follow-up; {} groups remain at runtime",
            summary.audited, summary.implemented, summary.follow_up, summary.current,
        );
    } else if args.json {
        println!("{}", serde_json::to_string_pretty(&census)?);
    } else {
        print_human(&census);
        if let Some(listing) = recovery_listing.as_ref() {
            print_recovery_listing(listing, args.list_limit);
        }
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

fn print_recovery_listing(listing: &RecoveryListing, limit: usize) {
    let groups = listing.sorted_groups();
    println!(
        "grouped structural recovery ({} groups; showing up to {limit}; up to {LISTED_FACE_NAMES} earliest face names per group):",
        groups.len(),
    );
    for (index, group) in groups.into_iter().take(limit).enumerate() {
        let face_names = group
            .face_names
            .iter()
            .map(String::as_str)
            .collect::<Vec<_>>();
        println!(
            "  {:>4}. {:<18} {:>6} occurrences  {:>8} recovered tokens  [{}] {:?}",
            index + 1,
            role_name(group.role),
            group.occurrences,
            group.source_tokens,
            face_names.join(", "),
            group.text,
        );
    }
}

const fn role_name(role: RecoveryRole) -> &'static str {
    match role {
        RecoveryRole::Clause => "clause",
        RecoveryRole::NominalComplement => "nominal",
        RecoveryRole::ActivationCost => "activation cost",
        RecoveryRole::KeywordArgument => "keyword argument",
        RecoveryRole::ModalHeader => "modal header",
        RecoveryRole::EmbeddedRules => "embedded rules",
    }
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

    #[test]
    fn recovery_listing_groups_exact_text_and_keeps_sorted_face_names_bounded() {
        let mut listing = RecoveryListing::default();
        for face_name in ["Zeta", "Gamma", "Epsilon", "Alpha", "Beta", "Delta"] {
            listing.observe(RecoveryRole::Clause, "You frobnitz a card.", 5, face_name);
        }
        listing.observe(RecoveryRole::Clause, "You glorp a card.", 5, "Other");
        listing.observe(
            RecoveryRole::ActivationCost,
            "Pay a mystery cost.",
            4,
            "Costly",
        );

        let groups = listing.sorted_groups();
        assert_eq!(groups.len(), 3);
        assert_eq!(groups[0].role, RecoveryRole::Clause);
        assert_eq!(groups[0].text, "You frobnitz a card.");
        assert_eq!(groups[0].occurrences, 6);
        assert_eq!(groups[0].source_tokens, 30);
        assert_eq!(
            groups[0]
                .face_names
                .iter()
                .map(String::as_str)
                .collect::<Vec<_>>(),
            ["Alpha", "Beta", "Delta", "Epsilon", "Gamma"]
        );
        assert_eq!(groups[1].text, "You glorp a card.");
        assert_eq!(groups[2].role, RecoveryRole::ActivationCost);
    }

    #[test]
    fn recovery_listing_merge_preserves_counts_and_bounded_face_names() {
        let mut left = RecoveryListing::default();
        left.observe(RecoveryRole::Clause, "You frobnitz a card.", 5, "Zeta");
        left.observe(RecoveryRole::Clause, "You frobnitz a card.", 5, "Gamma");
        let mut right = RecoveryListing::default();
        for face_name in ["Epsilon", "Alpha", "Beta", "Delta"] {
            right.observe(RecoveryRole::Clause, "You frobnitz a card.", 5, face_name);
        }

        left.add(right);

        let group = &left.sorted_groups()[0];
        assert_eq!(group.occurrences, 6);
        assert_eq!(group.source_tokens, 30);
        assert_eq!(
            group
                .face_names
                .iter()
                .map(String::as_str)
                .collect::<Vec<_>>(),
            ["Alpha", "Beta", "Delta", "Epsilon", "Gamma"]
        );
    }
}
