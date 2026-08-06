use std::io::Write;
use std::io::{self};

use anyhow::Result;
use anyhow::bail;
use clap::Args;
use deckmaste_english::Catalogs;
use deckmaste_english::ConstructionBackend;
use deckmaste_english::ConstructionEvidenceKind;
use deckmaste_english::ConstructionOwner;
use deckmaste_english::ParseCost;
use deckmaste_english::ParseCostDimension;
use deckmaste_english::ParseReport;
use deckmaste_english::SelectionReason;
use deckmaste_english::parse_with_identity;

use super::data::CardFace;
use super::data::OracleDataArgs;

#[derive(Debug, Args)]
pub(super) struct InspectArgs {
    /// Exact card or face name (matched case-insensitively).
    card: String,

    #[command(flatten)]
    data: OracleDataArgs,

    #[command(flatten)]
    output_config: OutputConfig,
}

#[derive(Debug, Args)]
struct OutputConfig {
    /// Include diagnostics and parse provenance with their byte spans.
    #[arg(short, long)]
    verbose: bool,

    /// Print only the parsed abilities and any diagnostics.
    #[arg(short, long)]
    abilities_only: bool,
}

pub(super) fn run(args: &InspectArgs) -> Result<()> {
    let data = args.data.load()?;
    let cards = find_cards(&data.faces, &args.card);

    if cards.is_empty() {
        bail!(
            "no exact card or face named {:?} in {}",
            args.card,
            data.data_path.display()
        );
    }

    write_cards(
        io::stdout().lock(),
        &cards,
        &data.catalogs,
        &args.output_config,
    )
}

pub(super) fn find_cards(faces: &[CardFace], query: &str) -> Vec<CardFace> {
    let mut standalone = Vec::new();
    let mut whole_card = Vec::new();
    let mut face = Vec::new();

    for card in faces {
        match &card.face_name {
            None if card.card_name.eq_ignore_ascii_case(query) => standalone.push(card.clone()),
            Some(_) if card.card_name.eq_ignore_ascii_case(query) => whole_card.push(card.clone()),
            Some(face_name) if face_name.eq_ignore_ascii_case(query) => face.push(card.clone()),
            _ => {}
        }
    }

    if !standalone.is_empty() {
        standalone
    } else if !whole_card.is_empty() {
        whole_card
    } else {
        face
    }
}

fn write_cards(
    mut writer: impl Write,
    cards: &[CardFace],
    catalogs: &Catalogs,
    output_config: &OutputConfig,
) -> Result<()> {
    for (index, card) in cards.iter().enumerate() {
        if index != 0 {
            writeln!(writer)?;
        }

        match &card.face_name {
            Some(face_name) => writeln!(writer, "{} — {face_name}", card.card_name)?,
            None => writeln!(writer, "{}", card.card_name)?,
        }
        writeln!(writer, "\nOracle text:\n{}", card.oracle_text)?;
        let report = parse_with_identity(
            &card.oracle_text,
            catalogs,
            card.printed_name(),
            card.is_legendary,
        );
        let ast = report.ast();
        if output_config.abilities_only {
            writeln!(writer, "\nAbilities:\n{:#?}", ast.abilities)?;
        } else {
            writeln!(writer, "\nAST:\n{ast:#?}")?;
        }
        write_diagnostics(
            &mut writer,
            &report,
            &card.oracle_text,
            output_config.verbose,
        )?;
        if output_config.verbose && !report.provenance().selections().is_empty() {
            write_provenance(&mut writer, &report)?;
        }
    }

    Ok(())
}

fn write_provenance(mut writer: impl Write, report: &ParseReport) -> Result<()> {
    writeln!(writer, "\nProvenance:")?;
    for selection in report.provenance().selections() {
        for decision in selection.constructions() {
            let span = decision.span();
            let evidence = decision.evidence();
            writeln!(
                writer,
                "bytes {}..{} {} owner={} backend={} evidence={}:{} reason={} cost={}",
                span.start,
                span.end,
                decision.selected(),
                owner_name(decision.owner()),
                backend_name(decision.backend()),
                evidence_kind_name(evidence.kind()),
                evidence.label(),
                reason_name(decision.reason()),
                cost_text(decision.cost()),
            )?;
            for alternative in decision.alternatives() {
                writeln!(
                    writer,
                    "  alternative {}#{} dominated={}",
                    alternative.id(),
                    alternative.production_ordinal(),
                    alternative.is_dominated(),
                )?;
            }
        }
    }
    Ok(())
}

const fn owner_name(owner: ConstructionOwner) -> &'static str {
    match owner {
        ConstructionOwner::Handwritten => "handwritten",
        ConstructionOwner::Generated => "generated",
    }
}

const fn backend_name(backend: ConstructionBackend) -> &'static str {
    match backend {
        ConstructionBackend::Chart => "chart",
        ConstructionBackend::Ability => "ability",
    }
}

const fn evidence_kind_name(kind: ConstructionEvidenceKind) -> &'static str {
    match kind {
        ConstructionEvidenceKind::Structural => "structural",
        ConstructionEvidenceKind::Guard => "guard",
        ConstructionEvidenceKind::Feature => "feature",
        ConstructionEvidenceKind::Role => "role",
    }
}

fn reason_name(reason: SelectionReason) -> String {
    match reason {
        SelectionReason::Unique => "unique".to_owned(),
        SelectionReason::Cost(dimension) => format!("cost:{}", cost_dimension_name(dimension)),
        SelectionReason::Dominance => "dominance".to_owned(),
        SelectionReason::StableIdentity => "stable_identity".to_owned(),
    }
}

const fn cost_dimension_name(dimension: ParseCostDimension) -> &'static str {
    match dimension {
        ParseCostDimension::OpaqueWords => "opaque_words",
        ParseCostDimension::OpaqueLexemes => "opaque_lexemes",
        ParseCostDimension::GenericRules => "generic_rules",
        ParseCostDimension::ReadingDispreference => "reading_dispreference",
        ParseCostDimension::AttachmentCount => "attachment_count",
        ParseCostDimension::AttachmentDistance => "attachment_distance",
        ParseCostDimension::AttachmentExtent => "attachment_extent",
        ParseCostDimension::Precedence => "precedence",
    }
}

fn cost_text(cost: ParseCost) -> String {
    format!(
        "{{opaque_words:{},opaque_lexemes:{},generic_rules:{},reading_dispreference:{},\
         attachment_count:{},attachment_distance:{},attachment_extent:{},precedence:{}}}",
        cost.opaque_words(),
        cost.opaque_lexemes(),
        cost.generic_rules(),
        cost.reading_dispreference(),
        cost.attachment_count(),
        cost.attachment_distance(),
        cost.attachment_extent(),
        cost.precedence(),
    )
}

fn write_diagnostics(
    mut writer: impl Write,
    report: &ParseReport,
    source: &str,
    verbose: bool,
) -> Result<()> {
    if report.diagnostics().is_empty() {
        return Ok(());
    }
    if verbose {
        writeln!(writer, "\nDiagnostics:\n{:#?}", report.diagnostics())?;
        return Ok(());
    }

    writeln!(writer, "\nDiagnostics:")?;
    for diagnostic in report.diagnostics() {
        let text = diagnostic
            .span()
            .text(source)
            .unwrap_or("<invalid source span>");
        writeln!(writer, "  {:?} at {text:?}", diagnostic.kind())?;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use std::io::Cursor;
    use std::path::Path;
    use std::time::Duration;

    use deckmaste_english::normalize_roll_row_dashes;
    use deckmaste_english::normalize_sentence_case;
    use deckmaste_english::normalize_typographic_quotes;
    use deckmaste_english::strip_reminder_text;

    use super::*;
    use crate::english::data::map_supported_faces;
    use crate::english::data::read_card_faces;

    const DATA_PATH: &str = "test-cards.jsonl";

    struct RoundTripOutcome {
        index: usize,
        printed_name: String,
        elapsed: Duration,
        failure: Option<String>,
    }

    fn cards(data: &str) -> Vec<CardFace> {
        read_card_faces(Cursor::new(data), Path::new(DATA_PATH)).unwrap()
    }

    #[test]
    fn standalone_name_wins_over_a_face_with_the_same_name() {
        let data = concat!(
            r#"{"name":"Borrow","face":null,"text":"Draw a card."}"#,
            "\n",
            r#"{"name":"Borrow // Return","face":"Borrow","text":"Return target creature."}"#,
            "\n",
        );

        let cards = find_cards(&cards(data), "borrow");

        assert_eq!(
            cards,
            [CardFace {
                card_name: "Borrow".to_owned(),
                face_name: None,
                is_legendary: false,
                supported: false,
                source_text: "Draw a card.".to_owned(),
                oracle_text: "Draw a card.".to_owned(),
            }]
        );
    }

    #[test]
    fn combined_name_returns_every_face() {
        let data = concat!(
            r#"{"name":"Fire // Ice","face":"Fire","text":"Deal damage."}"#,
            "\n",
            r#"{"name":"Fire // Ice","face":"Ice","text":"Tap a permanent."}"#,
            "\n",
        );

        let cards = find_cards(&cards(data), "FIRE // ICE");

        assert_eq!(cards.len(), 2);
        assert_eq!(cards[0].face_name.as_deref(), Some("Fire"));
        assert_eq!(cards[1].face_name.as_deref(), Some("Ice"));
    }

    #[test]
    fn face_name_returns_only_that_face() {
        let data = concat!(
            r#"{"name":"Fire // Ice","face":"Fire","text":"Deal damage."}"#,
            "\n",
            r#"{"name":"Fire // Ice","face":"Ice","text":"Tap a permanent."}"#,
            "\n",
        );

        let cards = find_cards(&cards(data), "ice");

        assert_eq!(cards.len(), 1);
        assert_eq!(cards[0].face_name.as_deref(), Some("Ice"));
    }

    #[test]
    fn incoming_legendary_names_are_kept_verbatim_in_the_name_bearing_domain() {
        let data = concat!(
            r#"{"name":"Aang, A Lot to Learn","face":null,"supertypes":["Legendary"],"text":"Aang attacks. Aang, A Lot to Learn's power is 3."}"#,
            "\n",
        );

        let cards = find_cards(&cards(data), "Aang, A Lot to Learn");

        // The self-reference is recognized during the parse, not rewritten on
        // the way in, so the Oracle text keeps the face's own name verbatim.
        assert_eq!(
            cards[0].oracle_text,
            "Aang attacks. Aang, A Lot to Learn's power is 3."
        );
    }

    #[test]
    fn incoming_double_faced_cards_keep_the_face_name_verbatim() {
        let data = concat!(
            r#"{"name":"Aang, Swift Savior // Aang and La, Ocean's Fury","face":"Aang, Swift Savior","supertypes":["Legendary"],"text":"Aang transforms. Aang, Swift Savior has flying."}"#,
            "\n",
        );

        let cards = find_cards(&cards(data), "Aang, Swift Savior");

        assert_eq!(
            cards[0].oracle_text,
            "Aang transforms. Aang, Swift Savior has flying."
        );
    }

    #[test]
    fn incoming_typographic_quotes_normalize_before_parsing() {
        let data = concat!(
            r#"{"name":"A Realm Reborn","face":null,"text":"Other permanents you control have “{T}: Add one mana of any color.”"}"#,
            "\n",
        );

        let cards = cards(data);

        assert_eq!(
            cards[0].oracle_text,
            "Other permanents you control have \"{T}: Add one mana of any color.\""
        );
    }

    #[test]
    fn normal_output_resolves_spans_while_verbose_output_keeps_them() {
        let cards = [CardFace {
            card_name: "Test Card".to_owned(),
            face_name: None,
            is_legendary: false,
            supported: false,
            source_text: "Draw a card.".to_owned(),
            oracle_text: "Draw a card.".to_owned(),
        }];
        let mut normal = Vec::new();
        let mut verbose = Vec::new();
        let catalogs = Catalogs::default();
        let normal_config = OutputConfig {
            verbose: false,
            abilities_only: false,
        };
        let verbose_config = OutputConfig {
            verbose: true,
            abilities_only: false,
        };

        write_cards(&mut normal, &cards, &catalogs, &normal_config).unwrap();
        write_cards(&mut verbose, &cards, &catalogs, &verbose_config).unwrap();
        let normal = String::from_utf8(normal).unwrap();
        let verbose = String::from_utf8(verbose).unwrap();

        assert!(normal.contains("verb: VerbInstance {"));
        assert!(normal.contains("Word(\n"));
        assert!(normal.contains("Draw,"));
        assert!(!normal.contains("Span"));
        assert!(!normal.contains("ChartStats"));
        assert!(!normal.contains("ForestStats"));
        assert!(verbose.contains("Provenance:"));
        assert!(verbose.contains("bytes 0.."));
        // Mutation caught: restore either handwritten Sentence production.
        // Inspect is the public provenance surface and must attribute this
        // root to the generated family, independently of inner handwritten
        // clause constructions.
        assert!(verbose.contains("sentence owner=generated backend=chart"));
        assert!(verbose.contains("owner=handwritten"));
        assert!(verbose.contains("cost={opaque_words:"));
        assert!(!verbose.contains("Span {"));
        assert!(!verbose.contains("ChartStats"));
        assert!(!verbose.contains("ForestStats"));
    }

    #[test]
    fn verbose_output_explains_declared_construction_dominance() {
        let cards = [CardFace {
            card_name: "Test Card".to_owned(),
            face_name: None,
            is_legendary: false,
            supported: false,
            source_text: "This creature has protection from artifacts.".to_owned(),
            oracle_text: "This creature has protection from artifacts.".to_owned(),
        }];
        let mut rendered = Vec::new();
        let catalogs = Catalogs::default().with_catalog(
            deckmaste_english::CatalogKind::KeywordAbility,
            ["Protection"],
        );

        write_cards(
            &mut rendered,
            &cards,
            &catalogs,
            &OutputConfig {
                verbose: true,
                abilities_only: false,
            },
        )
        .unwrap();

        let rendered = String::from_utf8(rendered).unwrap();
        assert!(rendered.contains("nominal_prepositional owner=handwritten backend=chart"));
        assert!(rendered.contains("evidence=feature:nominal attachment phase"));
        assert!(rendered.contains("reason=dominance"));
        assert!(rendered.contains("cost={opaque_words:0,opaque_lexemes:0,generic_rules:0"));
        assert!(
            rendered.contains("alternative nominal_keyword_predicated_argument#0 dominated=true")
        );
    }

    #[test]
    fn abilities_only_omits_the_full_ast_wrapper() {
        let cards = [CardFace {
            card_name: "Test Card".to_owned(),
            face_name: None,
            is_legendary: false,
            supported: false,
            source_text: "Draw a card.".to_owned(),
            oracle_text: "Draw a card.".to_owned(),
        }];
        let output = OutputConfig {
            verbose: false,
            abilities_only: true,
        };
        let mut rendered = Vec::new();

        write_cards(&mut rendered, &cards, &Catalogs::default(), &output).unwrap();

        let rendered = String::from_utf8(rendered).unwrap();
        assert!(rendered.contains("Abilities:"));
        assert!(!rendered.contains("AST:"));
        assert!(!rendered.contains("OracleText {"));
        assert!(!rendered.contains("Span"));
    }

    #[test]
    #[cfg_attr(
        not(all(derived_cards, gen_catalogs)),
        ignore = "needs data/derived/cards.jsonl and data/gen/catalogs"
    )]
    fn local_card_snapshot_structurally_round_trips_without_source_text() {
        use std::time::Instant;

        let data = OracleDataArgs::default()
            .load()
            .expect("release corpus data must be available for the structural round-trip gate");

        let outcomes = map_supported_faces(&data.faces, |index, card| {
            let started = Instant::now();
            let report = parse_with_identity(
                &card.oracle_text,
                &data.catalogs,
                card.printed_name(),
                card.is_legendary,
            );
            let elapsed = started.elapsed();
            let printed_name = card.printed_name().to_owned();
            let failure = match report
                .into_ast()
                .render(card.printed_name(), card.is_legendary)
            {
                Ok(rebuilt)
                    if normalized_rules_text(&rebuilt)
                        == normalized_rules_text(&card.source_text) =>
                {
                    None
                }
                Ok(rebuilt) => Some(format!(
                    "row {} ({}):\n  rendered: {rebuilt:?}\n  expected: {:?}",
                    index + 1,
                    card.printed_name(),
                    normalized_rules_text(&card.source_text)
                )),
                Err(error) => Some(format!(
                    "row {} ({}): render error: {error}",
                    index + 1,
                    card.printed_name()
                )),
            };
            RoundTripOutcome {
                index,
                printed_name,
                elapsed,
                failure,
            }
        });

        let mut failures = Vec::new();
        for outcome in outcomes {
            if outcome.elapsed.as_millis() >= 100 {
                eprintln!(
                    "slow row {} ({}) {:?}",
                    outcome.index + 1,
                    outcome.printed_name,
                    outcome.elapsed
                );
            }
            if outcome.index % 1_000 == 0 {
                eprintln!(
                    "reached row {} ({})",
                    outcome.index + 1,
                    outcome.printed_name
                );
            }
            if let Some(failure) = outcome.failure {
                failures.push(failure);
            }
        }

        assert!(
            failures.is_empty(),
            "{} structural round-trip failures (first 20):\n{}",
            failures.len(),
            failures
                .iter()
                .take(20)
                .cloned()
                .collect::<Vec<_>>()
                .join("\n")
        );
    }

    fn normalized_rules_text(text: &str) -> String {
        // Sentence case is normalized last, after reminder text is gone, so a
        // stripped reminder can never shift a sentence boundary's position.
        normalize_sentence_case(&normalize_roll_row_dashes(&normalize_typographic_quotes(
            &strip_reminder_text(text),
        )))
    }
}
