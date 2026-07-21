use std::collections::HashMap;

use anyhow::Result;
use anyhow::bail;
use clap::Args;
use deckmaste_english::Ability;
use deckmaste_english::AbilityKind;
use deckmaste_english::Clause;
use deckmaste_english::ModalFrame;
use deckmaste_english::OracleText;
use deckmaste_english::Paragraph;
use deckmaste_english::Phrase;
use deckmaste_english::Predicate;
use deckmaste_english::SimpleClause;
use deckmaste_english::parse_with_catalogs;

use super::data::OracleDataArgs;

#[derive(Debug, Args)]
pub(super) struct UnknownPhrasesArgs {
    #[command(flatten)]
    data: OracleDataArgs,

    /// Number of result rows to print.
    #[arg(short, long, default_value_t = 50)]
    limit: usize,

    /// Ignore phrases shorter than this many words.
    #[arg(long, default_value_t = 0)]
    min_words: usize,

    /// Ignore phrases longer than this many words.
    #[arg(long)]
    max_words: Option<usize>,

    /// Sort by phrase text instead of by descending word count.
    #[arg(long, conflicts_with = "sort_count")]
    alphabetical: bool,

    /// Group equal phrase text and syntactic roles, replacing card names with
    /// counts.
    #[arg(long)]
    unique: bool,

    /// Sort grouped phrases by descending occurrence count. Implies --unique.
    #[arg(long)]
    sort_count: bool,

    /// Only show phrases with at least this many occurrences. Implies --unique.
    #[arg(long)]
    min_count: Option<usize>,

    /// Only show phrases with at most this many occurrences. Implies --unique.
    #[arg(long)]
    max_count: Option<usize>,
}

#[derive(Debug, PartialEq, Eq)]
struct Occurrence {
    card: String,
    role: &'static str,
    text: String,
    words: usize,
}

#[derive(Debug, PartialEq, Eq)]
struct UniqueOccurrence {
    role: &'static str,
    text: String,
    words: usize,
    count: usize,
}

pub(super) fn run(args: &UnknownPhrasesArgs) -> Result<()> {
    if args
        .max_words
        .is_some_and(|max_words| max_words < args.min_words)
    {
        bail!("--max-words must be greater than or equal to --min-words");
    }
    if args
        .min_count
        .zip(args.max_count)
        .is_some_and(|(min_count, max_count)| max_count < min_count)
    {
        bail!("--max-count must be greater than or equal to --min-count");
    }
    let unique = uses_unique_mode(args);
    let data = args.data.load()?;

    let mut occurrences = Vec::new();
    let supported = data.faces.iter().filter(|card| card.supported);
    let card_count = supported.clone().count();
    for card in supported {
        let ast = parse_with_catalogs(&card.oracle_text, &data.catalogs);
        UnknownPhraseCollector::new(&mut occurrences, card.printed_name()).oracle_text(&ast);
    }
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
        card_count,
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
    if unique {
        let mut grouped = unique_occurrences(&occurrences);
        sort_unique_occurrences(&mut grouped, args.alphabetical, args.sort_count);
        let order = if args.sort_count {
            "most-common-first"
        } else if args.alphabetical {
            "alphabetical"
        } else {
            "longest-first"
        };
        print_result_header("unique groups", args, order);
        for (index, occurrence) in grouped
            .iter()
            .filter(|occurrence| {
                matches_word_bounds(occurrence.words, args.min_words, args.max_words)
                    && matches_count_bounds(
                        occurrence.count,
                        args.min_count.unwrap_or(1),
                        args.max_count,
                    )
            })
            .take(args.limit)
            .enumerate()
        {
            let count = format!("{} occurrences", occurrence.count);
            println!(
                "{:>4}. {:>3} words  {:<18} {:<40} {:?}",
                index + 1,
                occurrence.words,
                occurrence.role,
                count,
                occurrence.text
            );
        }
    } else {
        sort_occurrences(&mut occurrences, args.alphabetical);
        let order = if args.alphabetical { "alphabetical" } else { "longest-first" };
        print_result_header("occurrences", args, order);
        for (index, occurrence) in occurrences
            .iter()
            .filter(|occurrence| {
                matches_word_bounds(occurrence.words, args.min_words, args.max_words)
            })
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
    }

    Ok(())
}

fn print_result_header(kind: &str, args: &UnknownPhrasesArgs, order: &str) {
    let word_range = match args.max_words {
        Some(max_words) => format!("from {} through {max_words} words", args.min_words),
        None => format!("with at least {} words", args.min_words),
    };
    let count_range =
        if uses_unique_mode(args) && (args.min_count.is_some() || args.max_count.is_some()) {
            match args.max_count {
                Some(max_count) => format!(
                    "; from {} through {max_count} occurrences",
                    args.min_count.unwrap_or(1)
                ),
                None => format!(
                    "; with at least {} occurrences",
                    args.min_count.unwrap_or(1)
                ),
            }
        } else {
            String::new()
        };
    println!("UnknownPhrase {kind} {word_range}{count_range} ({order}):");
}

struct UnknownPhraseCollector<'a> {
    occurrences: &'a mut Vec<Occurrence>,
    card: &'a str,
}

impl<'a> UnknownPhraseCollector<'a> {
    fn new(occurrences: &'a mut Vec<Occurrence>, card: &'a str) -> Self {
        Self { occurrences, card }
    }

    fn oracle_text(&mut self, ast: &OracleText) {
        for ability in &ast.abilities {
            self.ability(ability);
        }
    }

    fn ability(&mut self, ability: &Ability) {
        if let Some(word) = &ability.ability_word {
            self.phrase("ability word", word);
        }
        match &ability.kind {
            AbilityKind::Activated(ability) => {
                for component in &ability.cost.components {
                    self.phrase("activation cost", component);
                }
                self.paragraph(&ability.effect);
            }
            AbilityKind::Triggered(ability) => {
                self.simple_clause(&ability.event);
                self.paragraph(&ability.effect);
            }
            AbilityKind::Loyalty(ability) => self.paragraph(&ability.effect),
            AbilityKind::Modal(ability) => {
                match &ability.frame {
                    ModalFrame::Unframed | ModalFrame::Loyalty(_) => {}
                    ModalFrame::Preamble { body, .. } => self.paragraph(body),
                    ModalFrame::Activated(cost) => {
                        for component in &cost.components {
                            self.phrase("activation cost", component);
                        }
                    }
                    ModalFrame::Triggered { event, .. } => self.simple_clause(event),
                }
                self.paragraph(&ability.header);
                for mode in &ability.modes {
                    self.paragraph(&mode.body);
                }
            }
            AbilityKind::Keyword(list) => {
                for keyword in &list.abilities {
                    self.phrase("keyword name", &keyword.printed_name);
                    if let Some(argument) = &keyword.argument {
                        self.phrase("keyword argument", argument);
                    }
                }
            }
            AbilityKind::Paragraph(paragraph) => self.paragraph(paragraph),
        }
    }

    fn paragraph(&mut self, paragraph: &Paragraph) {
        for sentence in &paragraph.sentences {
            match &sentence.clause {
                Clause::Simple(clause) => self.simple_clause(clause),
                Clause::CommaSeparated(clause) => {
                    self.simple_clause(&clause.first);
                    self.simple_clause(&clause.second);
                }
                Clause::Conditional(clause) => {
                    self.simple_clause(&clause.condition);
                    self.simple_clause(&clause.consequence);
                }
            }
        }
    }

    fn simple_clause(&mut self, clause: &SimpleClause) {
        let Some(predicate) = &clause.predicate else {
            if let Some(unparsed) = &clause.unparsed {
                self.phrase("unparsed clause", unparsed);
            }
            return;
        };
        if let Some(subject) = &clause.subject {
            self.phrase("subject", subject);
        }
        self.predicate(predicate);
        for coordinated in &clause.coordinated_predicates {
            self.predicate(&coordinated.predicate);
        }
        for coordinated in &clause.coordinated_clauses {
            self.simple_clause(&coordinated.clause);
        }
    }

    fn predicate(&mut self, predicate: &Predicate) {
        self.phrase("verb", &predicate.verb);
        if let Some(complement) = &predicate.complement {
            self.phrase("complement", complement);
        }
    }

    fn phrase(&mut self, role: &'static str, phrase: &Phrase) {
        match phrase {
            Phrase::UnknownPhrase(text) => self.occurrences.push(Occurrence {
                card: self.card.to_owned(),
                role,
                text: text.to_owned(),
                words: phrase_word_count(text),
            }),
            Phrase::NounPhrase(phrase) => self.phrase(role, &phrase.head),
            Phrase::ModifiedNounPhrase(phrase) => {
                self.phrase(role, &phrase.modifier);
                self.phrase(role, &phrase.head);
            }
            Phrase::QuantityPhrase(phrase) => {
                self.phrase(role, &phrase.quantity);
                self.phrase(role, &phrase.unit);
            }
            Phrase::Lexeme { .. }
            | Phrase::ThisCard { .. }
            | Phrase::OracleSymbol { .. }
            | Phrase::SymbolSequence { .. }
            | Phrase::NumberLiteral { .. }
            | Phrase::PowerToughness(_)
            | Phrase::CatalogTerm { .. } => {}
            Phrase::EmbeddedRulesPhrase { embedded_rules, .. } => {
                for rules in embedded_rules {
                    self.ability(&rules.ability);
                }
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

fn matches_count_bounds(count: usize, min_count: usize, max_count: Option<usize>) -> bool {
    count >= min_count && max_count.is_none_or(|maximum| count <= maximum)
}

fn uses_unique_mode(args: &UnknownPhrasesArgs) -> bool {
    args.unique || args.sort_count || args.min_count.is_some() || args.max_count.is_some()
}

fn unique_occurrences(occurrences: &[Occurrence]) -> Vec<UniqueOccurrence> {
    let mut counts = HashMap::new();
    for occurrence in occurrences {
        *counts
            .entry((occurrence.role, occurrence.text.as_str()))
            .or_insert(0) += 1;
    }
    counts
        .into_iter()
        .map(|((role, text), count)| UniqueOccurrence {
            role,
            text: text.to_owned(),
            words: phrase_word_count(text),
            count,
        })
        .collect()
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

fn sort_unique_occurrences(
    occurrences: &mut [UniqueOccurrence],
    alphabetical: bool,
    sort_count: bool,
) {
    occurrences.sort_unstable_by(|left, right| {
        if sort_count {
            right
                .count
                .cmp(&left.count)
                .then_with(|| left.text.cmp(&right.text))
                .then_with(|| left.role.cmp(right.role))
        } else if alphabetical {
            left.text
                .cmp(&right.text)
                .then_with(|| left.role.cmp(right.role))
                .then_with(|| right.words.cmp(&left.words))
        } else {
            right
                .words
                .cmp(&left.words)
                .then_with(|| left.text.cmp(&right.text))
                .then_with(|| left.role.cmp(right.role))
        }
    });
}

#[cfg(test)]
mod tests {
    use deckmaste_english::Catalogs;

    use super::*;

    #[test]
    fn unknown_phrase_occurrences_include_their_syntactic_role() {
        let source = "Destroy target artifact or enchantment.";
        let ast = parse_with_catalogs(source, &Catalogs::default());
        let mut occurrences = Vec::new();

        UnknownPhraseCollector::new(&mut occurrences, "Test Card").oracle_text(&ast);

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
    fn count_bounds_are_inclusive() {
        assert!(matches_count_bounds(3, 3, Some(3)));
        assert!(matches_count_bounds(4, 3, None));
        assert!(!matches_count_bounds(2, 3, Some(5)));
        assert!(!matches_count_bounds(6, 3, Some(5)));
    }

    #[test]
    fn count_arguments_imply_unique_mode() {
        fn args() -> UnknownPhrasesArgs {
            UnknownPhrasesArgs {
                data: OracleDataArgs::default(),
                limit: 50,
                min_words: 0,
                max_words: None,
                alphabetical: false,
                unique: false,
                sort_count: false,
                min_count: None,
                max_count: None,
            }
        }
        let min = UnknownPhrasesArgs {
            min_count: Some(2),
            ..args()
        };
        let max = UnknownPhrasesArgs {
            max_count: Some(4),
            ..args()
        };
        let sorted = UnknownPhrasesArgs {
            sort_count: true,
            ..args()
        };

        assert!(uses_unique_mode(&min));
        assert!(uses_unique_mode(&max));
        assert!(uses_unique_mode(&sorted));
    }

    #[test]
    fn unique_mode_groups_by_phrase_and_role() {
        let occurrence = |card: &str, role: &'static str, text: &str| Occurrence {
            card: card.to_owned(),
            role,
            text: text.to_owned(),
            words: phrase_word_count(text),
        };
        let occurrences = vec![
            occurrence("Card A", "subject", "target creature"),
            occurrence("Card B", "subject", "target creature"),
            occurrence("Card C", "complement", "target creature"),
        ];

        let mut grouped = unique_occurrences(&occurrences);
        sort_unique_occurrences(&mut grouped, false, true);

        assert_eq!(grouped.len(), 2);
        assert_eq!(grouped[0].role, "subject");
        assert_eq!(grouped[0].count, 2);
        assert_eq!(grouped[1].role, "complement");
        assert_eq!(grouped[1].count, 1);
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
        let source = "Target creature gains \"Whenever this creature attacks, draw a blorple.\"";
        let ast = parse_with_catalogs(source, &Catalogs::default());
        let mut occurrences = Vec::new();

        UnknownPhraseCollector::new(&mut occurrences, "Test Card").oracle_text(&ast);

        assert!(
            occurrences
                .iter()
                .all(|occurrence| !occurrence.text.starts_with('"'))
        );
        assert!(
            occurrences
                .iter()
                .any(|occurrence| occurrence.text == "blorple")
        );
    }

    #[test]
    fn catalog_terms_are_not_reported_as_unknown() {
        let source = "Creatures you control have haste.";
        let catalogs = Catalogs::new(
            ["Haste"],
            std::iter::empty::<&str>(),
            std::iter::empty::<&str>(),
        );
        let ast = parse_with_catalogs(source, &catalogs);
        let mut occurrences = Vec::new();

        UnknownPhraseCollector::new(&mut occurrences, "Test Card").oracle_text(&ast);

        assert!(
            occurrences
                .iter()
                .all(|occurrence| occurrence.text != "haste")
        );
    }
}
