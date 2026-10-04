use std::path::Path;
use std::sync::Arc;

/// Failure to read, parse, or resolve a decklist.
#[derive(Debug, thiserror::Error)]
pub enum DeckError {
    #[error("reading decklist {}: {source}", path.display())]
    Io {
        path: std::path::PathBuf,
        #[source]
        source: std::io::Error,
    },
    #[error("parsing decklist {}: {source}", path.display())]
    Parse {
        path: std::path::PathBuf,
        #[source]
        source: DeckParseError,
    },
    #[error("deck {deck:?}: unknown card {card:?}")]
    UnknownCard { deck: String, card: String },
}

#[derive(Debug, thiserror::Error)]
pub enum DeckParseError {
    #[error("line {line}: expected `<count> <card name>`, got {text:?}")]
    MissingCount { line: usize, text: String },
    #[error("line {line}: invalid count {count:?}: {source}")]
    InvalidCount {
        line: usize,
        count: String,
        #[source]
        source: std::num::ParseIntError,
    },
    #[error("line {line}: count must be greater than 0")]
    ZeroCount { line: usize },
}

use crate::LoadedCard;
use crate::plugin::Plugin;

/// A decklist: a name and the `(count, card-name)` entries it lists.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Deck {
    pub name: String,
    pub entries: Arc<[DeckEntry]>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DeckEntry {
    pub count: usize,
    pub card: String,
}

impl Deck {
    /// Reads and parses a decklist file.
    ///
    /// # Errors
    /// If the file can't be read or a line is malformed.
    pub fn load(path: &Path) -> Result<Deck, DeckError> {
        let text = std::fs::read_to_string(path).map_err(|source| DeckError::Io {
            path: path.into(),
            source,
        })?;
        Self::parse(&text).map_err(|source| DeckError::Parse {
            path: path.into(),
            source,
        })
    }

    fn parse(text: &str) -> Result<Deck, DeckParseError> {
        let mut name = String::new();
        let mut entries = Vec::new();
        for (i, raw) in text.lines().enumerate() {
            let line = raw.trim();
            if line.is_empty() || line.starts_with('#') {
                continue;
            }
            if let Some(rest) = line.strip_prefix("name:") {
                name = rest.trim().to_string();
                continue;
            }
            let (count, card) = line.split_once(char::is_whitespace).ok_or_else(|| {
                DeckParseError::MissingCount {
                    line: i + 1,
                    text: line.into(),
                }
            })?;
            let count: usize =
                count
                    .trim()
                    .parse()
                    .map_err(|source| DeckParseError::InvalidCount {
                        line: i + 1,
                        count: count.into(),
                        source,
                    })?;
            if count == 0 {
                return Err(DeckParseError::ZeroCount { line: i + 1 });
            }
            entries.push(DeckEntry {
                count,
                card: card.trim().to_string(),
            });
        }
        Ok(Deck {
            name,
            entries: entries.into(),
        })
    }

    /// Resolves every entry to `count` clones of its card, looking each name up
    /// across `plugins` in order (first match wins).
    ///
    /// Retains BOTH halves: the semantic term is what the render side links
    /// back to, and it cannot be recovered downstream without a reparse.
    ///
    /// # Errors
    /// If a card name resolves in none of the plugins.
    pub fn resolve(&self, plugins: &[&Plugin]) -> Result<Arc<[Arc<LoadedCard>]>, DeckError> {
        let mut out = Vec::new();
        for entry in self.entries.iter() {
            let card = plugins
                .iter()
                .find_map(|p| p.card(&entry.card).ok())
                .ok_or_else(|| DeckError::UnknownCard {
                    deck: self.name.clone(),
                    card: entry.card.clone(),
                })?;
            let card = Arc::new(card);
            out.extend(std::iter::repeat_n(Arc::clone(&card), entry.count));
        }
        Ok(out.into())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn plugins_dir(rel: &str) -> std::path::PathBuf {
        Path::new(env!("CARGO_MANIFEST_DIR")).join(rel)
    }

    #[test]
    fn loader_reports_path_line_count_and_source() {
        let directory = tempfile::tempdir().unwrap();
        let path = directory.path().join("deck.txt");
        std::fs::write(&path, "name: Example\ninvalid Card\n").unwrap();
        let error = Deck::load(&path).unwrap_err();
        match &error {
            DeckError::Parse {
                path: actual,
                source: DeckParseError::InvalidCount { line, count, .. },
            } => {
                assert_eq!(actual, &path);
                assert_eq!(*line, 2);
                assert_eq!(count, "invalid");
            }
            other => panic!("{other:?}"),
        }
        let source = std::error::Error::source(&error).unwrap();
        assert!(source.is::<DeckParseError>());
        assert!(source.source().unwrap().is::<std::num::ParseIntError>());
    }

    #[test]
    fn parses_name_counts_and_skips_blanks_and_comments() {
        let deck =
            Deck::parse("name: Goblins\n\n# comment\n16 Goblin Brigand\n12 Mountain\n").unwrap();
        assert_eq!(deck.name, "Goblins");
        assert_eq!(
            deck.entries,
            vec![
                DeckEntry {
                    count: 16,
                    card: "Goblin Brigand".to_string()
                },
                DeckEntry {
                    count: 12,
                    card: "Mountain".to_string()
                },
            ]
            .into()
        );
    }

    #[test]
    fn rejects_line_without_whitespace() {
        // No whitespace at all: the count/name split itself fails.
        let error = Deck::parse("GoblinBrigand\n").unwrap_err();
        assert!(
            error.to_string().contains("expected `<count> <card name>`"),
            "unexpected error: {error}"
        );
    }

    #[test]
    fn rejects_zero_count() {
        let error = Deck::parse("0 Goblin Brigand\n").unwrap_err();
        assert!(
            error.to_string().contains("count must be greater than 0"),
            "unexpected error: {error}"
        );
    }

    #[test]
    fn rejects_non_numeric_count() {
        let error = Deck::parse("x Goblin Brigand\n").unwrap_err();
        assert!(
            error.to_string().contains("invalid count"),
            "unexpected error: {error}"
        );
    }

    #[test]
    fn resolves_names_across_canon_and_builtin() {
        let canon = Plugin::load_with_sibling_prelude(plugins_dir("../../plugins/canon")).unwrap();
        let builtin = Plugin::load(plugins_dir("../../plugins/builtin")).unwrap();
        let deck = Deck::parse("3 Goblin Brigand\n2 Mountain\n").unwrap();
        let cards = deck.resolve(&[&canon, &builtin]).unwrap();
        assert_eq!(cards.len(), 5);
    }

    #[test]
    fn unknown_card_errs() {
        let canon = Plugin::load_with_sibling_prelude(plugins_dir("../../plugins/canon")).unwrap();
        let builtin = Plugin::load(plugins_dir("../../plugins/builtin")).unwrap();
        let deck = Deck::parse("1 Nonexistent Card\n").unwrap();
        let error = deck.resolve(&[&canon, &builtin]).unwrap_err();
        let msg = error.to_string();
        assert!(msg.contains("unknown card"), "unexpected error: {msg}");
        assert!(msg.contains("Nonexistent Card"), "unexpected error: {msg}");
    }

    /// Resolution keeps the semantic half. The composition layer downstream
    /// needs it to link engine cards back to what they were written as;
    /// collapsing here would make that unrecoverable without a reparse.
    #[test]
    fn resolve_retains_the_semantic_half() {
        let plugin = Plugin::load(plugins_dir("../../plugins/builtin")).unwrap();
        let deck = Deck::parse("1 Forest\n").unwrap();
        let resolved = deck.resolve(&[&plugin]).unwrap();
        assert_eq!(resolved.len(), 1);

        let deckmaste_semantics::Card::Normal(semantic_face) = &resolved[0].semantic else {
            panic!("Forest is a Normal card");
        };
        let deckmaste_card::Card::Normal(core_face) = &resolved[0].core else {
            panic!("Forest is a Normal card");
        };
        assert_eq!(semantic_face.name, core_face.characteristics.name);
    }
}
