use std::collections::BTreeMap;
use std::io;
use std::io::Write;

use anyhow::Context;
use anyhow::Result;
use anyhow::bail;
use anyhow::ensure;
use clap::Args;
use deckmaste_english::Catalogs;
use deckmaste_english::ParseReport;
use deckmaste_english::Span;
use deckmaste_english::parse_with_identity;
use rayon::prelude::*;

use super::data::CardFace;
use super::data::OracleDataArgs;
use super::inspect::find_cards;

#[derive(Debug, Args)]
pub struct BracketArgs {
    /// Exact card or face name (matched case-insensitively).
    #[arg(
        value_name = "CARD",
        required_unless_present = "all",
        conflicts_with = "all"
    )]
    card: Option<String>,

    /// Emit tab-separated records for every card face in the snapshot.
    #[arg(long)]
    all: bool,

    /// With --all, restrict the dump to faces marked supported.
    #[arg(long, requires = "all")]
    supported_only: bool,

    #[command(flatten)]
    data: OracleDataArgs,
}

struct BracketRecord {
    face_index: usize,
    card_name: String,
    face_name: Option<String>,
    ability_index: usize,
    supported: bool,
    bracketed: String,
}

struct BracketedAbility {
    text: String,
}

pub(super) fn run(args: &BracketArgs) -> Result<()> {
    let data = args.data.load()?;
    let stdout = io::stdout();
    let mut writer = stdout.lock();

    if args.all {
        let selected = data
            .faces
            .iter()
            .enumerate()
            .filter(|(_, card)| !args.supported_only || card.supported)
            .collect::<Vec<_>>();
        let records = selected
            .par_iter()
            .map(|&(face_index, card)| bracket_records(face_index, card, &data.catalogs))
            .collect::<Vec<_>>()
            .into_iter()
            .collect::<Result<Vec<_>>>()?;

        writeln!(writer, "record\tcard\tface\tability\tsupported\tbracketed")?;
        for record in records.into_iter().flatten() {
            writeln!(
                writer,
                "{}:{}\t{}\t{}\t{}\t{}\t{}",
                record.face_index,
                record.ability_index,
                escape_field(&record.card_name),
                escape_field(record.face_name.as_deref().unwrap_or("")),
                record.ability_index,
                record.supported,
                escape_field(&record.bracketed),
            )?;
        }
        return Ok(());
    }

    let query = args.card.as_deref().expect("clap requires CARD or --all");
    let cards = find_cards(&data.faces, query);
    if cards.is_empty() {
        bail!(
            "no exact card or face named {query:?} in {}",
            data.data_path.display()
        );
    }
    for card in &cards {
        let report = parse_card(card, &data.catalogs);
        for bracketed in bracketed_abilities(card, &report)? {
            writeln!(writer, "{}", escape_field(&bracketed.text))?;
        }
    }
    Ok(())
}

fn bracket_records(
    face_index: usize,
    card: &CardFace,
    catalogs: &Catalogs,
) -> Result<Vec<BracketRecord>> {
    let report = parse_card(card, catalogs);
    bracketed_abilities(card, &report)?
        .into_iter()
        .enumerate()
        .map(|(ability_index, bracketed)| {
            let ability_index = ability_index + 1;
            Ok(BracketRecord {
                face_index,
                card_name: card.card_name.clone(),
                face_name: card.face_name.clone(),
                ability_index,
                supported: card.supported,
                bracketed: bracketed.text,
            })
        })
        .collect()
}

fn parse_card(card: &CardFace, catalogs: &Catalogs) -> ParseReport {
    parse_with_identity(
        &card.oracle_text,
        catalogs,
        card.printed_name(),
        card.is_legendary,
    )
}

fn bracketed_abilities(card: &CardFace, report: &ParseReport) -> Result<Vec<BracketedAbility>> {
    ensure!(
        report.ast().abilities.len() == report.ability_spans().len(),
        "parser returned mismatched ability and span counts for {:?}",
        card.printed_name()
    );
    let delimiters = BracketDelimiters::for_source(&card.oracle_text)?;
    report
        .ability_spans()
        .iter()
        .map(|&ability_span| {
            let mut constituents = report
                .provenance()
                .selections()
                .iter()
                .flat_map(|selection| selection.constituent_spans().iter().copied())
                .filter(|constituent| contains(ability_span, *constituent))
                .collect::<Vec<_>>();
            // The ability layer is handwritten rather than chart-parsed, but
            // this span is the source extent of the actual top-level Ability
            // node paired with report.ast().abilities.
            constituents.push(ability_span);
            let text = bracket_source(&card.oracle_text, ability_span, &constituents, delimiters)
                .with_context(|| format!("could not bracket {:?}", card.printed_name()))?;
            Ok(BracketedAbility { text })
        })
        .collect()
}

fn contains(outer: Span, inner: Span) -> bool {
    outer.start <= inner.start && inner.end <= outer.end
}

#[derive(Debug, Clone, Copy)]
struct BracketDelimiters {
    open: char,
    close: char,
}

impl BracketDelimiters {
    fn for_source(source: &str) -> Result<Self> {
        if !source.contains(['<', '>']) {
            return Ok(Self {
                open: '<',
                close: '>',
            });
        }
        if !source.contains(['‹', '›']) {
            return Ok(Self {
                open: '‹',
                close: '›',
            });
        }
        bail!("Oracle text contains both angle-bracket delimiter pairs")
    }
}

fn bracket_source(
    source: &str,
    outer: Span,
    constituents: &[Span],
    delimiters: BracketDelimiters,
) -> Result<String> {
    let surface = outer
        .text(source)
        .context("ability span is not a UTF-8 boundary in the Oracle text")?;
    let mut candidates = constituents.to_vec();
    for span in &candidates {
        ensure!(
            !span.is_empty() && contains(outer, *span),
            "constituent span {span:?} lies outside ability span {outer:?}"
        );
    }
    // Unary grammar chains frequently give several selected nonterminals the
    // exact same source extent. They carry type information internally, but
    // unlabeled brackets cannot distinguish it, so one pair per distinct span
    // conveys the full visible constituency without redundant chevrons.
    candidates.sort_unstable_by_key(|span| (span.start, std::cmp::Reverse(span.end)));
    candidates.dedup();
    let mut ancestors = Vec::<Span>::new();
    for span in &candidates {
        while ancestors
            .last()
            .is_some_and(|ancestor| ancestor.end <= span.start)
        {
            ancestors.pop();
        }
        if let Some(ancestor) = ancestors.last() {
            ensure!(
                span.end <= ancestor.end,
                "crossing constituent spans {ancestor:?} and {span:?}"
            );
        }
        ancestors.push(*span);
    }

    let mut openings = BTreeMap::<usize, usize>::new();
    let mut closings = BTreeMap::<usize, usize>::new();
    for span in candidates {
        *openings.entry(span.start - outer.start).or_default() += 1;
        *closings.entry(span.end - outer.start).or_default() += 1;
    }
    let mut boundaries = openings
        .keys()
        .chain(closings.keys())
        .copied()
        .collect::<Vec<_>>();
    boundaries.sort_unstable();
    boundaries.dedup();

    let mut bracketed = String::with_capacity(surface.len() + constituents.len() * 2);
    let mut cursor = 0;
    for boundary in boundaries {
        let text = surface
            .get(cursor..boundary)
            .context("constituent span is not a UTF-8 boundary")?;
        bracketed.push_str(text);
        for _ in 0..closings.get(&boundary).copied().unwrap_or_default() {
            bracketed.push(delimiters.close);
        }
        for _ in 0..openings.get(&boundary).copied().unwrap_or_default() {
            bracketed.push(delimiters.open);
        }
        cursor = boundary;
    }
    bracketed.push_str(
        surface
            .get(cursor..)
            .context("constituent span is not a UTF-8 boundary")?,
    );
    ensure!(
        strip_structural_brackets(&bracketed, delimiters) == surface,
        "bracketing changed the Oracle surface"
    );
    Ok(bracketed)
}

fn strip_structural_brackets(bracketed: &str, delimiters: BracketDelimiters) -> String {
    bracketed
        .chars()
        .filter(|character| *character != delimiters.open && *character != delimiters.close)
        .collect()
}

fn escape_field(field: &str) -> String {
    let mut escaped = String::with_capacity(field.len());
    for character in field.chars() {
        match character {
            '\\' => escaped.push_str("\\\\"),
            '\t' => escaped.push_str("\\t"),
            '\n' => escaped.push_str("\\n"),
            '\r' => escaped.push_str("\\r"),
            _ => escaped.push(character),
        }
    }
    escaped
}

#[cfg(test)]
mod tests {
    use deckmaste_english::CatalogKind;
    use deckmaste_english::Catalogs;
    use deckmaste_english::Span;
    use deckmaste_english::parse_with_catalogs;

    use super::BracketDelimiters;
    use super::OracleDataArgs;
    use super::bracket_records;
    use super::bracket_source;
    use super::escape_field;
    use super::strip_structural_brackets;
    use crate::english::data::map_supported_faces;

    fn bracket_synthetic(source: &str, catalogs: &Catalogs) -> String {
        let report = parse_with_catalogs(source, catalogs);
        assert!(
            report.ast().recoveries().is_empty(),
            "synthetic fixture recovered: {report:#?}"
        );
        let mut constituents = report
            .provenance()
            .selections()
            .iter()
            .flat_map(|selection| selection.constituent_spans().iter().copied())
            .collect::<Vec<_>>();
        constituents.push(Span::new(0, source.len()));
        bracket_source(
            source,
            Span::new(0, source.len()),
            &constituents,
            BracketDelimiters {
                open: '<',
                close: '>',
            },
        )
        .expect("synthetic fixture must have laminar provenance")
    }

    #[test]
    fn tsv_fields_escape_record_separators_and_backslashes() {
        assert_eq!(escape_field("a\\b\tc\nd\re"), "a\\\\b\\tc\\nd\\re");
    }

    #[test]
    fn structural_brackets_strip_without_touching_oracle_punctuation() {
        assert_eq!(
            strip_structural_brackets(
                "<<Whenever <this>>, <that>>.",
                BracketDelimiters {
                    open: '<',
                    close: '>',
                },
            ),
            "Whenever this, that."
        );
    }

    #[test]
    fn crossing_selected_nodes_are_an_invariant_failure() {
        let source = "He gains vigilance, indestructible";
        let error = bracket_source(
            source,
            Span::new(0, source.len()),
            &[
                Span::new(0, source.len()),
                Span::new(0, source.len()),
                Span::new(0, 18),
                Span::new(3, source.len()),
            ],
            BracketDelimiters {
                open: '<',
                close: '>',
            },
        )
        .unwrap_err();

        assert!(
            error.to_string().contains(
                "crossing constituent spans Span { start: 0, end: 18 } and Span { start: 3"
            ),
            "unexpected error: {error:#}"
        );
    }

    #[test]
    fn bonfire_dump_exposes_shared_target_and_full_recipient_coordination() {
        let data = OracleDataArgs::default()
            .load()
            .expect("release corpus data must be available for the Bonfire fixture");
        let (face_index, card) = data
            .faces
            .iter()
            .enumerate()
            .find(|(_, card)| card.printed_name() == "Bonfire of the Damned")
            .expect("Bonfire of the Damned must be present in the release corpus");
        let records = bracket_records(face_index, card, &data.catalogs)
            .expect("Bonfire must have laminar parse provenance");
        let damage = &records
            .first()
            .expect("Bonfire must have a spell-effect ability")
            .bracketed;

        assert!(
            damage.contains("<to <target <<player> or <planeswalker>>> and <<each> <<creature>"),
            "unexpected Bonfire bracket tree: {damage}"
        );
        assert!(
            !damage.contains("<<<target> <player>> or <planeswalker>>"),
            "the first target head must not carry the shared determiner: {damage}"
        );
    }

    #[test]
    fn adversarial_shared_determiner_brackets_preserve_scope() {
        let catalogs = Catalogs::default()
            .with_catalog(
                CatalogKind::CardType,
                [
                    "Artifact",
                    "Battle",
                    "Creature",
                    "Enchantment",
                    "Land",
                    "Planeswalker",
                ],
            )
            .with_catalog(CatalogKind::CreatureType, ["Elf", "Orc"]);

        let damage = bracket_synthetic(
            concat!(
                "Choose target creature. That creature gets +1/+1 and deals 3 damage to ",
                "target player or planeswalker and 1 damage to each creature that player ",
                "or that planeswalker controls."
            ),
            &catalogs,
        );
        assert!(
            damage.contains("<target <<player> or <planeswalker>>>")
                && !damage.contains("<<<target> <player>> or <planeswalker>>"),
            "unexpected shared target scope: {damage}"
        );

        let exile = bracket_synthetic(
            "Exile target artifact, creature, or planeswalker and target land or battle.",
            &catalogs,
        );
        assert!(
            exile.contains("<target <<artifact>, <creature>, or <planeswalker>>>")
                && exile.contains("<target <<land> or <battle>>>")
                && !exile.contains("<artifact>, <creature>, or <planeswalker> and <target"),
            "the two target groups must remain distinct: {exile}"
        );

        let destroy =
            bracket_synthetic("Destroy target attacking or blocking creature.", &catalogs);
        assert!(
            destroy.contains("<<<attacking> or <blocking>> <creature>>"),
            "the coordinated participles must share the creature head: {destroy}"
        );
    }

    #[test]
    fn supported_corpus_selected_provenance_is_laminar() {
        let data = OracleDataArgs::default()
            .load()
            .expect("release corpus data must be available for the constituency gate");
        let outcomes = map_supported_faces(&data.faces, |face_index, card| {
            bracket_records(face_index, card, &data.catalogs).map_err(|error| {
                format!(
                    "row {} ({:?}): {error:#}",
                    face_index + 1,
                    card.printed_name()
                )
            })
        });
        let failures = outcomes
            .into_iter()
            .filter_map(Result::err)
            .collect::<Vec<_>>();

        assert!(failures.is_empty(), "{}", failures.join("\n"));
    }
}
