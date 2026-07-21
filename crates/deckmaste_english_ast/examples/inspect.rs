use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::io::Write;
use std::io::{self};
use std::path::Path;
use std::path::PathBuf;

use anyhow::Context;
use anyhow::Result;
use anyhow::bail;
use clap::Parser;
use deckmaste_english_ast::parse;
use serde::Deserialize;

#[derive(Debug, Parser)]
#[command(about = "Parse a card's Oracle text from deckmaste's local data snapshot")]
struct Args {
    /// Exact card or face name (matched case-insensitively).
    card: String,

    /// Override the derived card-data snapshot.
    #[arg(long, value_name = "PATH")]
    data: Option<PathBuf>,

    /// Include the underlying byte spans instead of resolving them to text.
    #[arg(short, long)]
    verbose: bool,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Row {
    name: String,
    face: Option<String>,
    text: Option<String>,
}

#[derive(Debug, PartialEq, Eq)]
struct CardText {
    card_name: String,
    face_name: Option<String>,
    oracle_text: String,
}

fn main() -> Result<()> {
    let args = Args::parse();
    let data_path = args.data.unwrap_or_else(default_data_path);
    let file = File::open(&data_path).with_context(|| {
        format!(
            "could not open {}; generate the repository's derived card data first",
            data_path.display()
        )
    })?;
    let cards = find_cards(BufReader::new(file), &args.card, &data_path)?;

    if cards.is_empty() {
        bail!(
            "no exact card or face named {:?} in {}",
            args.card,
            data_path.display()
        );
    }

    write_cards(io::stdout().lock(), &cards, args.verbose)
}

fn default_data_path() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../../data/derived/cards.jsonl")
}

fn find_cards(reader: impl BufRead, query: &str, data_path: &Path) -> Result<Vec<CardText>> {
    let mut standalone = Vec::new();
    let mut whole_card = Vec::new();
    let mut face = Vec::new();

    for (index, line) in reader.lines().enumerate() {
        let line_number = index + 1;
        let line = line.with_context(|| {
            format!(
                "could not read line {line_number} of {}",
                data_path.display()
            )
        })?;
        let row: Row = serde_json::from_str(&line).with_context(|| {
            format!(
                "invalid JSON on line {line_number} of {}",
                data_path.display()
            )
        })?;

        let card = CardText {
            card_name: row.name.clone(),
            face_name: row.face.clone(),
            oracle_text: row.text.unwrap_or_default(),
        };

        match row.face {
            None if row.name.eq_ignore_ascii_case(query) => standalone.push(card),
            Some(_) if row.name.eq_ignore_ascii_case(query) => whole_card.push(card),
            Some(face_name) if face_name.eq_ignore_ascii_case(query) => face.push(card),
            _ => {}
        }
    }

    Ok(if !standalone.is_empty() {
        standalone
    } else if !whole_card.is_empty() {
        whole_card
    } else {
        face
    })
}

fn write_cards(mut writer: impl Write, cards: &[CardText], verbose: bool) -> Result<()> {
    for (index, card) in cards.iter().enumerate() {
        if index != 0 {
            writeln!(writer)?;
        }

        match &card.face_name {
            Some(face_name) => writeln!(writer, "{} — {face_name}", card.card_name)?,
            None => writeln!(writer, "{}", card.card_name)?,
        }
        writeln!(writer, "\nOracle text:\n{}", card.oracle_text)?;
        let ast = parse(&card.oracle_text);
        if verbose {
            writeln!(writer, "\nAST:\n{ast:#?}")?;
        } else {
            writeln!(writer, "\nAST:\n{:#?}", ast.source_debug(&card.oracle_text))?;
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use std::io::Cursor;

    use super::*;

    const DATA_PATH: &str = "test-cards.jsonl";

    #[test]
    fn standalone_name_wins_over_a_face_with_the_same_name() {
        let data = concat!(
            r#"{"name":"Borrow","face":null,"text":"Draw a card."}"#,
            "\n",
            r#"{"name":"Borrow // Return","face":"Borrow","text":"Return target creature."}"#,
            "\n",
        );

        let cards = find_cards(Cursor::new(data), "borrow", Path::new(DATA_PATH)).unwrap();

        assert_eq!(
            cards,
            [CardText {
                card_name: "Borrow".to_owned(),
                face_name: None,
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

        let cards = find_cards(Cursor::new(data), "FIRE // ICE", Path::new(DATA_PATH)).unwrap();

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

        let cards = find_cards(Cursor::new(data), "ice", Path::new(DATA_PATH)).unwrap();

        assert_eq!(cards.len(), 1);
        assert_eq!(cards[0].face_name.as_deref(), Some("Ice"));
    }

    #[test]
    fn normal_output_resolves_spans_while_verbose_output_keeps_them() {
        let cards = [CardText {
            card_name: "Test Card".to_owned(),
            face_name: None,
            oracle_text: "Draw a card.".to_owned(),
        }];
        let mut normal = Vec::new();
        let mut verbose = Vec::new();

        write_cards(&mut normal, &cards, false).unwrap();
        write_cards(&mut verbose, &cards, true).unwrap();
        let normal = String::from_utf8(normal).unwrap();
        let verbose = String::from_utf8(verbose).unwrap();

        assert!(normal.contains("text: \"Draw\""));
        assert!(normal.contains("verb: \"Draw\""));
        assert!(!normal.contains("Span"));
        assert!(verbose.contains("Span"));
        assert!(verbose.contains("start: 0"));
    }
}
