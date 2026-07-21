use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::path::Path;
use std::path::PathBuf;

use anyhow::Context;
use anyhow::Result;
use anyhow::bail;
use clap::Parser;
use deckmaste_english_ast::Ability;
use deckmaste_english_ast::AbilityKind;
use deckmaste_english_ast::Catalogs;
use deckmaste_english_ast::Clause;
use deckmaste_english_ast::ModalFrame;
use deckmaste_english_ast::OracleText;
use deckmaste_english_ast::Paragraph;
use deckmaste_english_ast::Phrase;
use deckmaste_english_ast::Predicate;
use deckmaste_english_ast::SimpleClause;
use deckmaste_english_ast::normalize_self_references;
use deckmaste_english_ast::parse_with_catalogs;
use serde::Deserialize;

#[derive(Debug, Parser)]
#[command(about = "Rank UnknownPhrase occurrences in local Oracle text by word count")]
struct Args {
    /// Override the derived card-data snapshot.
    #[arg(long, value_name = "PATH")]
    data: Option<PathBuf>,

    /// Override the directory containing Scryfall's English catalogs.
    #[arg(long, value_name = "DIR")]
    catalogs: Option<PathBuf>,

    /// Number of longest occurrences to print.
    #[arg(short, long, default_value_t = 50)]
    limit: usize,

    /// Ignore phrases shorter than this many words.
    #[arg(long, default_value_t = 0)]
    min_words: usize,

    /// Ignore phrases longer than this many words.
    #[arg(long)]
    max_words: Option<usize>,

    /// Sort by phrase text instead of by descending word count.
    #[arg(long)]
    alphabetical: bool,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Row {
    name: String,
    face: Option<String>,
    #[serde(default)]
    supertypes: Vec<String>,
    text: Option<String>,
}

#[derive(Debug, Deserialize)]
struct Catalog {
    data: Vec<String>,
}

#[derive(Debug, PartialEq, Eq)]
struct Occurrence {
    card: String,
    role: &'static str,
    text: String,
    words: usize,
}

fn main() -> Result<()> {
    let args = Args::parse();
    if args
        .max_words
        .is_some_and(|max_words| max_words < args.min_words)
    {
        bail!("--max-words must be greater than or equal to --min-words");
    }
    let data_path = args.data.unwrap_or_else(default_data_path);
    let catalogs_path = args.catalogs.unwrap_or_else(default_catalogs_path);
    let catalogs = load_catalogs(&catalogs_path)?;
    let file = File::open(&data_path)
        .with_context(|| format!("could not open card data {}", data_path.display()))?;
    let mut rows = Vec::new();
    for (index, line) in BufReader::new(file).lines().enumerate() {
        let line = line.with_context(|| format!("could not read data row {}", index + 1))?;
        rows.push(
            serde_json::from_str::<Row>(&line)
                .with_context(|| format!("invalid JSON on data row {}", index + 1))?,
        );
    }

    let mut occurrences = Vec::new();
    for row in &rows {
        let card = row.face.as_deref().unwrap_or(&row.name);
        let is_legendary = row.supertypes.iter().any(|kind| kind == "Legendary");
        let source =
            normalize_self_references(row.text.as_deref().unwrap_or_default(), card, is_legendary);
        let ast = parse_with_catalogs(&source, &catalogs);
        collect_oracle_text(&mut occurrences, card, &source, &ast);
    }
    sort_occurrences(&mut occurrences, args.alphabetical);

    let maximum = occurrences
        .iter()
        .map(|occurrence| occurrence.words)
        .max()
        .unwrap_or(0);
    let over_three = occurrences
        .iter()
        .filter(|occurrence| occurrence.words > 3)
        .count();
    let mut phrase_lengths = [0; 7];
    for occurrence in &occurrences {
        let bucket = match occurrence.words {
            0..=1 => 0,
            2 => 1,
            3 => 2,
            4..=5 => 3,
            6..=10 => 4,
            11..=20 => 5,
            _ => 6,
        };
        phrase_lengths[bucket] += 1;
    }
    let unit = if maximum == 1 { "word" } else { "words" };
    println!(
        "audited {} card faces; found {} UnknownPhrase occurrences; maximum {maximum} {unit}; {over_three} over 3 words",
        rows.len(),
        occurrences.len()
    );
    println!(
        "UnknownPhrase lengths: 0–1={}  2={}  3={}  4–5={}  6–10={}  11–20={}  21+={}",
        phrase_lengths[0],
        phrase_lengths[1],
        phrase_lengths[2],
        phrase_lengths[3],
        phrase_lengths[4],
        phrase_lengths[5],
        phrase_lengths[6]
    );
    let order = if args.alphabetical { "alphabetical" } else { "longest-first" };
    match args.max_words {
        Some(max_words) => println!(
            "UnknownPhrase occurrences from {} through {max_words} words ({order}):",
            args.min_words
        ),
        None => println!(
            "UnknownPhrase occurrences with at least {} words ({order}):",
            args.min_words
        ),
    }
    for (index, occurrence) in occurrences
        .iter()
        .filter(|occurrence| matches_word_bounds(occurrence.words, args.min_words, args.max_words))
        .take(args.limit)
        .enumerate()
    {
        println!(
            "{:>4}. {:>3} words  {:<18} {:<40} {:?}",
            index + 1,
            occurrence.words,
            occurrence.role,
            occurrence.card,
            occurrence.text
        );
    }

    Ok(())
}

fn default_data_path() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../../data/derived/cards.jsonl")
}

fn default_catalogs_path() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../../data/catalogs")
}

fn load_catalogs(path: &Path) -> Result<Catalogs> {
    Ok(Catalogs::new(
        load_catalog(path, "keyword-abilities")?,
        load_catalog(path, "keyword-actions")?,
        load_catalog(path, "ability-words")?,
    ))
}

fn load_catalog(path: &Path, name: &str) -> Result<Vec<String>> {
    let path = path.join(format!("{name}.json"));
    let file = File::open(&path)
        .with_context(|| format!("could not open Scryfall catalog {}", path.display()))?;
    let catalog: Catalog = serde_json::from_reader(BufReader::new(file))
        .with_context(|| format!("invalid Scryfall catalog {}", path.display()))?;
    Ok(catalog.data)
}

fn collect_oracle_text(
    occurrences: &mut Vec<Occurrence>,
    card: &str,
    source: &str,
    ast: &OracleText,
) {
    for ability in &ast.abilities {
        collect_ability(occurrences, card, source, ability, ast);
    }
}

fn collect_ability(
    occurrences: &mut Vec<Occurrence>,
    card: &str,
    source: &str,
    ability: &Ability,
    ast: &OracleText,
) {
    if let Some(word) = &ability.ability_word {
        collect_phrase(occurrences, card, source, "ability word", word, ast);
    }
    match &ability.kind {
        AbilityKind::Activated(ability) => {
            for component in &ability.cost.components {
                collect_phrase(occurrences, card, source, "activation cost", component, ast);
            }
            collect_paragraph(occurrences, card, source, &ability.effect, ast);
        }
        AbilityKind::Triggered(ability) => {
            collect_simple_clause(occurrences, card, source, &ability.event, ast);
            collect_paragraph(occurrences, card, source, &ability.effect, ast);
        }
        AbilityKind::Loyalty(ability) => {
            collect_paragraph(occurrences, card, source, &ability.effect, ast);
        }
        AbilityKind::Modal(ability) => {
            match &ability.frame {
                ModalFrame::Unframed | ModalFrame::Loyalty(_) => {}
                ModalFrame::Activated(cost) => {
                    for component in &cost.components {
                        collect_phrase(
                            occurrences,
                            card,
                            source,
                            "activation cost",
                            component,
                            ast,
                        );
                    }
                }
                ModalFrame::Triggered { event, .. } => {
                    collect_simple_clause(occurrences, card, source, event, ast);
                }
            }
            collect_paragraph(occurrences, card, source, &ability.header, ast);
            for mode in &ability.modes {
                collect_paragraph(occurrences, card, source, &mode.body, ast);
            }
        }
        AbilityKind::Keyword(list) => {
            for keyword in &list.abilities {
                collect_phrase(
                    occurrences,
                    card,
                    source,
                    "keyword name",
                    &keyword.printed_name,
                    ast,
                );
                if let Some(argument) = &keyword.argument {
                    collect_phrase(occurrences, card, source, "keyword argument", argument, ast);
                }
            }
        }
        AbilityKind::Paragraph(paragraph) => {
            collect_paragraph(occurrences, card, source, paragraph, ast);
        }
    }
}

fn collect_paragraph(
    occurrences: &mut Vec<Occurrence>,
    card: &str,
    source: &str,
    paragraph: &Paragraph,
    ast: &OracleText,
) {
    for sentence in &paragraph.sentences {
        match &sentence.clause {
            Clause::Simple(clause) => {
                collect_simple_clause(occurrences, card, source, clause, ast);
            }
            Clause::Conditional(clause) => {
                collect_simple_clause(occurrences, card, source, &clause.condition, ast);
                collect_simple_clause(occurrences, card, source, &clause.consequence, ast);
            }
        }
    }
}

fn collect_simple_clause(
    occurrences: &mut Vec<Occurrence>,
    card: &str,
    source: &str,
    clause: &SimpleClause,
    ast: &OracleText,
) {
    let Some(predicate) = &clause.predicate else {
        if let Some(unparsed) = &clause.unparsed {
            collect_phrase(occurrences, card, source, "unparsed clause", unparsed, ast);
        }
        return;
    };
    if let Some(subject) = &clause.subject {
        collect_phrase(occurrences, card, source, "subject", subject, ast);
    }
    collect_predicate(occurrences, card, source, predicate, ast);
    for coordinated in &clause.coordinated_predicates {
        collect_predicate(occurrences, card, source, &coordinated.predicate, ast);
    }
    for coordinated in &clause.coordinated_clauses {
        collect_simple_clause(occurrences, card, source, &coordinated.clause, ast);
    }
}

fn collect_predicate(
    occurrences: &mut Vec<Occurrence>,
    card: &str,
    source: &str,
    predicate: &Predicate,
    ast: &OracleText,
) {
    collect_phrase(occurrences, card, source, "verb", &predicate.verb, ast);
    if let Some(complement) = &predicate.complement {
        collect_phrase(occurrences, card, source, "complement", complement, ast);
    }
}

fn collect_phrase(
    occurrences: &mut Vec<Occurrence>,
    card: &str,
    source: &str,
    role: &'static str,
    phrase: &Phrase,
    ast: &OracleText,
) {
    match phrase {
        Phrase::UnknownPhrase(text) => occurrences.push(Occurrence {
            card: card.to_owned(),
            role,
            text: text.to_owned(),
            words: phrase_word_count(text),
        }),
        Phrase::EmbeddedRulesPhrase { embedded_rules, .. } => {
            for rules in embedded_rules {
                collect_ability(occurrences, card, source, &rules.ability, ast);
            }
        }
    }
}

fn phrase_word_count(text: &str) -> usize {
    text.split_whitespace().count()
}

fn matches_word_bounds(words: usize, min_words: usize, max_words: Option<usize>) -> bool {
    words >= min_words && max_words.is_none_or(|maximum| words <= maximum)
}

fn sort_occurrences(occurrences: &mut [Occurrence], alphabetical: bool) {
    if alphabetical {
        occurrences.sort_unstable_by(|left, right| {
            left.text
                .cmp(&right.text)
                .then_with(|| left.card.cmp(&right.card))
                .then_with(|| left.role.cmp(right.role))
                .then_with(|| right.words.cmp(&left.words))
        });
    } else {
        occurrences.sort_unstable_by(|left, right| {
            right
                .words
                .cmp(&left.words)
                .then_with(|| left.card.cmp(&right.card))
                .then_with(|| left.role.cmp(right.role))
                .then_with(|| left.text.cmp(&right.text))
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn unknown_phrase_occurrences_include_their_syntactic_role() {
        let source = "Destroy target artifact or enchantment.";
        let ast = parse_with_catalogs(source, &Catalogs::default());
        let mut occurrences = Vec::new();

        collect_oracle_text(&mut occurrences, "Test Card", source, &ast);

        let longest = occurrences
            .iter()
            .max_by_key(|occurrence| occurrence.words)
            .unwrap();
        assert_eq!(longest.role, "complement");
        assert_eq!(longest.words, 4);
        assert_eq!(longest.text, "target artifact or enchantment");
    }

    #[test]
    fn symbols_separated_from_nothing_else_count_as_one_word() {
        assert_eq!(phrase_word_count("+1/+1"), 1);
        assert_eq!(phrase_word_count("{W}{U}{B}{R}{G}"), 1);
    }

    #[test]
    fn word_bounds_are_inclusive() {
        assert!(matches_word_bounds(3, 3, Some(3)));
        assert!(matches_word_bounds(4, 3, None));
        assert!(!matches_word_bounds(2, 3, Some(5)));
        assert!(!matches_word_bounds(6, 3, Some(5)));
    }

    #[test]
    fn alphabetical_sort_uses_phrase_then_card_then_role() {
        let occurrence = |card: &str, role: &'static str, text: &str| Occurrence {
            card: card.to_owned(),
            role,
            text: text.to_owned(),
            words: phrase_word_count(text),
        };
        let mut occurrences = vec![
            occurrence("Card B", "subject", "zebra"),
            occurrence("Card B", "verb", "alpha"),
            occurrence("Card A", "subject", "alpha"),
        ];

        sort_occurrences(&mut occurrences, true);

        assert_eq!(
            occurrences
                .iter()
                .map(|occurrence| (occurrence.text.as_str(), occurrence.card.as_str()))
                .collect::<Vec<_>>(),
            [
                ("alpha", "Card A"),
                ("alpha", "Card B"),
                ("zebra", "Card B")
            ]
        );
    }

    #[test]
    fn embedded_rules_wrapper_is_not_reported_as_unknown() {
        let source = "Target creature gains \"Whenever this creature attacks, draw a card.\"";
        let ast = parse_with_catalogs(source, &Catalogs::default());
        let mut occurrences = Vec::new();

        collect_oracle_text(&mut occurrences, "Test Card", source, &ast);

        assert!(
            occurrences
                .iter()
                .all(|occurrence| !occurrence.text.starts_with('"'))
        );
        assert!(
            occurrences
                .iter()
                .any(|occurrence| occurrence.text == "this creature")
        );
    }
}
