use anyhow::Result;
use anyhow::bail;
use clap::Args;
use deckmaste_english::Catalogs;
use deckmaste_english::normalize_self_references;
use deckmaste_english::parse_with_catalogs;
use serde::Serialize;

use crate::english::data::CardFace;
use crate::english::data::OracleDataArgs;
use crate::english::data::map_supported_faces;

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

/// Round-trips one supported face: parse its normalized Oracle text, render it
/// back with the real card identity, and compare in the self-reference
/// normalized domain (so `~`-vs-name differences never read as mismatches).
fn classify(card: &CardFace, catalogs: &Catalogs) -> Outcome {
    let report = parse_with_catalogs(&card.oracle_text, catalogs);
    let Ok(rendered) = report
        .into_ast()
        .render(card.printed_name(), card.is_legendary)
    else {
        return Outcome::Error;
    };
    if normalize_self_references(&rendered, card.printed_name(), card.is_legendary)
        == card.oracle_text
    {
        Outcome::Clean
    } else {
        Outcome::Mismatch
    }
}

#[derive(Debug, Default, PartialEq, Eq, Serialize)]
struct Summary {
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
    let data = args.data.load()?;
    let outcomes = map_supported_faces(&data.faces, |_, card| {
        let outcome = classify(card, &data.catalogs);
        // Capture the printed name only for the faces `--list` reports.
        let name = outcome.list_label().map(|_| card.printed_name().to_owned());
        (outcome, name)
    });

    let mut summary = Summary::default();
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
    println!("audited {} supported faces", summary.supported_faces);
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
    use deckmaste_english::normalize_typographic_quotes;
    use deckmaste_english::strip_reminder_text;

    use super::*;

    /// Builds a supported face exactly as `data::CardFace::from` does, so its
    /// `oracle_text` is the real normalized parse input.
    fn face(name: &str, source: &str, is_legendary: bool) -> CardFace {
        let normalized_source = normalize_typographic_quotes(source);
        let normalized_name = normalize_typographic_quotes(name);
        let oracle_text = strip_reminder_text(&normalize_self_references(
            &normalized_source,
            &normalized_name,
            is_legendary,
        ));
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
    fn self_reference_round_trips_in_normalized_domain() {
        // The source text contains the face's own name; its `oracle_text` holds
        // `~`, render re-expands the real name, and normalization maps it back.
        // This is the test that fails if the comparison uses the wrong domain.
        let card = face("Test Card", "Test Card deals 1 damage to you.", false);
        assert_eq!(card.oracle_text, "~ deals 1 damage to you.");
        assert_eq!(classify(&card, &Catalogs::default()), Outcome::Clean);
    }

    #[test]
    fn legendary_self_reference_round_trips() {
        // Exercises the `~~` (full name) / `~` (pre-comma short name) paths
        // through render + normalize.
        let card = face(
            "Hero, the Bold",
            "Hero, the Bold deals damage. Hero attacks.",
            true,
        );
        assert_eq!(card.oracle_text, "~~ deals damage. ~ attacks.");
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
