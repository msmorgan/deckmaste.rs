use std::path::Path;
use std::sync::Arc;

use anyhow::Context;
use anyhow::Result;

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
    pub fn load(path: &Path) -> Result<Deck> {
        let text = std::fs::read_to_string(path)
            .with_context(|| format!("reading decklist {}", path.display()))?;
        Self::parse(&text).with_context(|| format!("parsing decklist {}", path.display()))
    }

    fn parse(text: &str) -> Result<Deck> {
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
            let (count, card) = line.split_once(char::is_whitespace).with_context(|| {
                format!(
                    "line {}: expected `<count> <card name>`, got {line:?}",
                    i + 1
                )
            })?;
            let count: usize = count
                .trim()
                .parse()
                .with_context(|| format!("line {}: invalid count {count:?}", i + 1))?;
            anyhow::ensure!(count > 0, "line {}: count must be greater than 0", i + 1);
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
    pub fn resolve(&self, plugins: &[&Plugin]) -> Result<Arc<[Arc<LoadedCard>]>> {
        let mut out = Vec::new();
        for entry in self.entries.iter() {
            let card = plugins
                .iter()
                .find_map(|p| p.card(&entry.card).ok())
                .with_context(|| format!("deck {:?}: unknown card {:?}", self.name, entry.card))?;
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
