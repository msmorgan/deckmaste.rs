use anyhow::Result;
use anyhow::bail;
use clap::Args;
use deckmaste_english::Catalogs;
use deckmaste_english::parse_with_identity;
use serde::Serialize;

use crate::english::data::CardFace;
use crate::english::data::OracleDataArgs;
use crate::english::data::map_supported_faces_with_workers;
use crate::english::data::supported_face_jobs;

#[derive(Debug, Args)]
pub(super) struct RoundtripArgs {
    #[command(flatten)]
    data: OracleDataArgs,

    /// Emit the summary as JSON instead of a human-readable table.
    #[arg(long)]
    json: bool,

    /// List each non-clean face (`mismatch`/`error` and its printed name).
    #[arg(long)]
    list: bool,

    /// Fail unless every supported face round-trips clean.
    #[arg(long)]
    require_clean: bool,

    /// Maximum parser workers for this corpus run (reported with the result).
    #[arg(long, default_value_t = 4)]
    workers: usize,
}

/// Outcome of round-tripping one supported face through render.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Outcome {
    /// The rendered text normalizes back to the parse input.
    Clean,
    /// Render succeeded but disagreed with the parse input.
    Mismatch,
    /// Render itself failed.
    Error,
}

impl Outcome {
    /// The `--list` label for a non-clean outcome, or `None` when clean.
    fn list_label(self) -> Option<&'static str> {
        match self {
            Self::Clean => None,
            Self::Mismatch => Some("mismatch"),
            Self::Error => Some("error"),
        }
    }
}

/// Round-trips one supported face: parse its normalized, name-bearing Oracle
/// text as that face, render it back with the same identity, and compare
/// byte-for-byte. Self-references are recognized during the parse and
/// re-emitted as the face's own name, so the comparison is a direct equality in
/// the name-bearing domain.
fn classify(card: &CardFace, catalogs: &Catalogs) -> Outcome {
    let report = parse_with_identity(
        &card.oracle_text,
        catalogs,
        card.printed_name(),
        card.is_legendary,
    );
    let Ok(rendered) = report
        .into_ast()
        .render(card.printed_name(), card.is_legendary)
    else {
        return Outcome::Error;
    };
    if rendered == card.oracle_text { Outcome::Clean } else { Outcome::Mismatch }
}

#[derive(Debug, Default, PartialEq, Eq, Serialize)]
struct Summary {
    workers: usize,
    supported_faces: usize,
    clean: usize,
    mismatched: usize,
    render_errors: usize,
}

impl Summary {
    fn observe(&mut self, outcome: Outcome) {
        self.supported_faces += 1;
        match outcome {
            Outcome::Clean => self.clean += 1,
            Outcome::Mismatch => self.mismatched += 1,
            Outcome::Error => self.render_errors += 1,
        }
    }

    fn non_clean(&self) -> usize {
        self.mismatched + self.render_errors
    }
}

pub(super) fn run(args: &RoundtripArgs) -> Result<()> {
    if args.workers == 0 {
        bail!("--workers must be at least 1");
    }
    let data = args.data.load()?;
    let workers = supported_face_jobs(
        args.workers,
        data.faces.iter().filter(|card| card.supported).count(),
    );
    let outcomes = map_supported_faces_with_workers(&data.faces, args.workers, |_, card| {
        let outcome = classify(card, &data.catalogs);
        // Capture the printed name only for the faces `--list` reports.
        let name = outcome.list_label().map(|_| card.printed_name().to_owned());
        (outcome, name)
    });

    let mut summary = Summary {
        workers,
        ..Summary::default()
    };
    for &(outcome, _) in &outcomes {
        summary.observe(outcome);
    }

    if args.json {
        println!("{}", serde_json::to_string_pretty(&summary)?);
    } else {
        print_human(&summary);
    }

    if args.list {
        for (outcome, name) in &outcomes {
            if let Some(label) = outcome.list_label() {
                println!("{label}\t{}", name.as_deref().unwrap_or_default());
            }
        }
    }

    if args.require_clean && summary.non_clean() > 0 {
        bail!(
            "supported corpus does not round-trip clean: {} mismatched, {} render errors",
            summary.mismatched,
            summary.render_errors
        );
    }
    Ok(())
}

fn print_human(summary: &Summary) {
    println!(
        "audited {} supported faces with {} parser worker(s)",
        summary.supported_faces, summary.workers
    );
    println!("round-trip:");
    print_count("clean", summary.clean);
    print_count("mismatched", summary.mismatched);
    print_count("render errors", summary.render_errors);
}

fn print_count(label: &str, count: usize) {
    println!("  {label:<18} {count:>6} faces");
}

#[cfg(test)]
mod tests {
    use deckmaste_english::normalize_loyalty_minus;
    use deckmaste_english::normalize_roll_row_dashes;
    use deckmaste_english::normalize_sentence_case;
    use deckmaste_english::normalize_typographic_quotes;
    use deckmaste_english::strip_reminder_text;

    use super::*;

    /// Builds a supported face exactly as `data::CardFace::from` does, so its
    /// `oracle_text` is the real name-bearing parse input.
    fn face(name: &str, source: &str, is_legendary: bool) -> CardFace {
        let normalized_source = normalize_loyalty_minus(&normalize_roll_row_dashes(
            &normalize_typographic_quotes(source),
        ));
        let oracle_text = normalize_sentence_case(&strip_reminder_text(&normalized_source));
        CardFace {
            card_name: name.to_owned(),
            face_name: None,
            is_legendary,
            supported: true,
            source_text: source.to_owned(),
            oracle_text,
        }
    }

    #[test]
    fn parseable_sentence_round_trips_clean() {
        let card = face("Fakename Sproket", "Draw a card.", false);
        assert_eq!(classify(&card, &Catalogs::default()), Outcome::Clean);
    }

    #[test]
    fn recovered_text_round_trips_clean_by_construction() {
        // Structural recovery preserves the source span verbatim, so unparsed
        // text renders back byte-for-byte and must classify clean.
        let card = face("Fakename Sproket", "You frobnitz a card.", false);
        assert_eq!(classify(&card, &Catalogs::default()), Outcome::Clean);
    }

    #[test]
    fn self_reference_round_trips_in_the_name_bearing_domain() {
        // The source text contains the face's own name; `oracle_text` keeps it
        // verbatim, the parse recognizes it as a self-reference, and render
        // re-emits it, so the comparison is a direct equality.
        let card = face("Test Card", "Test Card deals 1 damage to you.", false);
        assert_eq!(card.oracle_text, "Test Card deals 1 damage to you.");
        assert_eq!(classify(&card, &Catalogs::default()), Outcome::Clean);
    }

    #[test]
    fn legendary_self_reference_round_trips() {
        // Exercises the full-name and the pre-comma shortened-name paths in the
        // name-bearing domain.
        let card = face(
            "Hero, the Bold",
            "Hero, the Bold deals damage. Hero attacks.",
            true,
        );
        assert_eq!(
            card.oracle_text,
            "Hero, the Bold deals damage. Hero attacks."
        );
        assert_eq!(classify(&card, &Catalogs::default()), Outcome::Clean);
    }

    #[test]
    fn mathise_hyphen_roll_rows_round_trip_clean_in_the_en_dash_domain() {
        // Mathise, Surge Channeler is the only card printing hyphen roll-row
        // ranges. Normalization rewrites them to en dashes, and the renderer
        // emits en dashes, so the row keys round-trip clean in that domain.
        let card = face(
            "Mathise, Surge Channeler",
            "1-9 | Each player draws a card.\n10-19 | You draw a card.\n20 | Draw a card.",
            true,
        );
        assert_eq!(
            card.oracle_text,
            "1\u{2013}9 | Each player draws a card.\n10\u{2013}19 | You draw a card.\n20 | Draw a card."
        );
        assert_eq!(classify(&card, &Catalogs::default()), Outcome::Clean);
    }

    #[test]
    fn summary_counts_each_outcome() {
        let mut summary = Summary::default();
        summary.observe(Outcome::Clean);
        summary.observe(Outcome::Mismatch);
        summary.observe(Outcome::Error);
        summary.observe(Outcome::Clean);

        assert_eq!(summary.supported_faces, 4);
        assert_eq!(summary.clean, 2);
        assert_eq!(summary.mismatched, 1);
        assert_eq!(summary.render_errors, 1);
        assert_eq!(summary.non_clean(), 2);
    }
}
