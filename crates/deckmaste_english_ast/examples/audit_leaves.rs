use std::cmp::Reverse;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::path::Path;
use std::path::PathBuf;

use anyhow::Context;
use anyhow::Result;
use clap::Parser;
use deckmaste_english_ast::Ability;
use deckmaste_english_ast::AbilityKind;
use deckmaste_english_ast::Catalogs;
use deckmaste_english_ast::Clause;
use deckmaste_english_ast::ModalFrame;
use deckmaste_english_ast::OracleText;
use deckmaste_english_ast::Paragraph;
use deckmaste_english_ast::Phrase;
use deckmaste_english_ast::PhrasePart;
use deckmaste_english_ast::Predicate;
use deckmaste_english_ast::SimpleClause;
use deckmaste_english_ast::Span;
use deckmaste_english_ast::TokenKind;
use deckmaste_english_ast::parse_with_catalogs;
use serde::Deserialize;

#[derive(Debug, Parser)]
#[command(about = "Rank the longest unresolved semantic leaves in local Oracle text")]
struct Args {
    /// Override the derived card-data snapshot.
    #[arg(long, value_name = "PATH")]
    data: Option<PathBuf>,

    /// Override the directory containing Scryfall's English catalogs.
    #[arg(long, value_name = "DIR")]
    catalogs: Option<PathBuf>,

    /// Number of longest leaves to print.
    #[arg(short, long, default_value_t = 50)]
    limit: usize,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Row {
    name: String,
    face: Option<String>,
    text: Option<String>,
}

#[derive(Debug, Deserialize)]
struct Catalog {
    data: Vec<String>,
}

struct Leaf<'source> {
    card: String,
    kind: &'static str,
    span: Span,
    source: &'source str,
    words: usize,
}

fn main() -> Result<()> {
    let args = Args::parse();
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

    let mut leaves = Vec::new();
    for row in &rows {
        let source = row.text.as_deref().unwrap_or_default();
        let card = row.face.as_deref().unwrap_or(&row.name);
        let ast = parse_with_catalogs(source, &catalogs);
        collect_oracle_text(&mut leaves, card, source, &ast);
    }
    leaves.sort_unstable_by_key(|leaf| Reverse(leaf.words));

    let maximum = leaves.first().map_or(0, |leaf| leaf.words);
    let over_three = leaves.iter().filter(|leaf| leaf.words > 3).count();
    let unit = if maximum == 1 { "word" } else { "words" };
    println!(
        "audited {} card faces and {} semantic leaves; maximum {maximum} {unit}; {over_three} over 3 words",
        rows.len(),
        leaves.len()
    );
    println!("longest semantic leaves (reminder text excluded):");
    for leaf in leaves.into_iter().take(args.limit) {
        println!(
            "{:>3} words  {:<18} {:<40} {:?}",
            leaf.words,
            leaf.kind,
            leaf.card,
            leaf.span.text(leaf.source).unwrap_or_default()
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

fn collect_oracle_text<'source>(
    leaves: &mut Vec<Leaf<'source>>,
    card: &str,
    source: &'source str,
    ast: &OracleText,
) {
    for ability in &ast.abilities {
        collect_ability(leaves, card, source, ability, ast);
    }
}

fn collect_ability<'source>(
    leaves: &mut Vec<Leaf<'source>>,
    card: &str,
    source: &'source str,
    ability: &Ability,
    ast: &OracleText,
) {
    if let Some(word) = &ability.ability_word {
        collect_phrase(leaves, card, source, word, ast);
    }
    match &ability.kind {
        AbilityKind::Activated(ability) => {
            for component in &ability.cost.components {
                collect_phrase(leaves, card, source, component, ast);
            }
            collect_paragraph(leaves, card, source, &ability.effect, ast);
        }
        AbilityKind::Triggered(ability) => {
            collect_simple_clause(leaves, card, source, &ability.event, ast);
            collect_paragraph(leaves, card, source, &ability.effect, ast);
        }
        AbilityKind::Loyalty(ability) => {
            push_leaf(leaves, card, source, "loyalty cost", ability.cost, ast);
            collect_paragraph(leaves, card, source, &ability.effect, ast);
        }
        AbilityKind::Modal(ability) => {
            match &ability.frame {
                ModalFrame::Unframed => {}
                ModalFrame::Activated(cost) => {
                    for component in &cost.components {
                        collect_phrase(leaves, card, source, component, ast);
                    }
                }
                ModalFrame::Triggered { event, .. } => {
                    collect_simple_clause(leaves, card, source, event, ast);
                }
                ModalFrame::Loyalty(cost) => {
                    push_leaf(leaves, card, source, "loyalty cost", *cost, ast);
                }
            }
            collect_paragraph(leaves, card, source, &ability.header, ast);
            for mode in &ability.modes {
                collect_paragraph(leaves, card, source, &mode.body, ast);
            }
        }
        AbilityKind::Keyword(list) => {
            for keyword in &list.abilities {
                collect_phrase(leaves, card, source, &keyword.printed_name, ast);
                if let Some(argument) = &keyword.argument {
                    collect_phrase(leaves, card, source, argument, ast);
                }
            }
        }
        AbilityKind::Paragraph(paragraph) => {
            collect_paragraph(leaves, card, source, paragraph, ast);
        }
    }
}

fn collect_paragraph<'source>(
    leaves: &mut Vec<Leaf<'source>>,
    card: &str,
    source: &'source str,
    paragraph: &Paragraph,
    ast: &OracleText,
) {
    for sentence in &paragraph.sentences {
        match &sentence.clause {
            Clause::Simple(clause) => collect_simple_clause(leaves, card, source, clause, ast),
            Clause::Conditional(clause) => {
                collect_simple_clause(leaves, card, source, &clause.condition, ast);
                collect_simple_clause(leaves, card, source, &clause.consequence, ast);
            }
        }
    }
}

fn collect_simple_clause<'source>(
    leaves: &mut Vec<Leaf<'source>>,
    card: &str,
    source: &'source str,
    clause: &SimpleClause,
    ast: &OracleText,
) {
    let Some(predicate) = &clause.predicate else {
        if let Some(unparsed) = &clause.unparsed {
            collect_phrase(leaves, card, source, unparsed, ast);
        }
        return;
    };
    if let Some(subject) = &clause.subject {
        collect_phrase(leaves, card, source, subject, ast);
    }
    collect_predicate(leaves, card, source, predicate, ast);
    for coordinated in &clause.coordinated_predicates {
        collect_predicate(leaves, card, source, &coordinated.predicate, ast);
    }
    for coordinated in &clause.coordinated_clauses {
        collect_simple_clause(leaves, card, source, &coordinated.clause, ast);
    }
}

fn collect_predicate<'source>(
    leaves: &mut Vec<Leaf<'source>>,
    card: &str,
    source: &'source str,
    predicate: &Predicate,
    ast: &OracleText,
) {
    if let Some(auxiliary) = predicate.auxiliary {
        push_leaf(leaves, card, source, "auxiliary", auxiliary, ast);
    }
    collect_phrase(leaves, card, source, &predicate.verb, ast);
    if let Some(complement) = &predicate.complement {
        collect_phrase(leaves, card, source, complement, ast);
    }
}

fn collect_phrase<'source>(
    leaves: &mut Vec<Leaf<'source>>,
    card: &str,
    source: &'source str,
    phrase: &Phrase,
    ast: &OracleText,
) {
    for part in &phrase.parts {
        match part {
            PhrasePart::Token(token) => {
                push_leaf(leaves, card, source, "phrase token", token.span, ast);
            }
            PhrasePart::Reminder(_) => {}
            PhrasePart::EmbeddedRules(rules) => {
                collect_ability(leaves, card, source, &rules.ability, ast);
            }
        }
    }
}

fn push_leaf<'source>(
    leaves: &mut Vec<Leaf<'source>>,
    card: &str,
    source: &'source str,
    kind: &'static str,
    span: Span,
    ast: &OracleText,
) {
    let words = ast
        .tokens
        .iter()
        .filter(|token| {
            token.span.start >= span.start
                && token.span.end <= span.end
                && matches!(token.kind, TokenKind::Word)
                && !ast.abilities.iter().any(|ability| {
                    ability.reminder_text.iter().any(|reminder| {
                        token.span.start >= reminder.span.start
                            && token.span.end <= reminder.span.end
                    })
                })
        })
        .count();
    leaves.push(Leaf {
        card: card.to_owned(),
        kind,
        span,
        source,
        words,
    });
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn local_snapshot_has_no_semantic_leaf_over_three_words_when_available() {
        let data_path = default_data_path();
        let catalogs_path = default_catalogs_path();
        if !data_path.is_file() || !catalogs_path.is_dir() {
            return;
        }
        let catalogs = load_catalogs(&catalogs_path).unwrap();
        let file = File::open(data_path).unwrap();
        let rows = BufReader::new(file)
            .lines()
            .map(|line| serde_json::from_str::<Row>(&line.unwrap()).unwrap())
            .collect::<Vec<_>>();
        let mut leaves = Vec::new();

        for row in &rows {
            let source = row.text.as_deref().unwrap_or_default();
            let card = row.face.as_deref().unwrap_or(&row.name);
            let ast = parse_with_catalogs(source, &catalogs);
            collect_oracle_text(&mut leaves, card, source, &ast);
        }

        let longest = leaves.iter().max_by_key(|leaf| leaf.words).unwrap();
        assert!(
            longest.words <= 3,
            "{}-word {} leaf on {}: {:?}",
            longest.words,
            longest.kind,
            longest.card,
            longest.span.text(longest.source).unwrap()
        );
    }
}
