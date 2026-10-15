//! The supported Oracle corpus and its Corpus Units.
//!
//! The supported corpus is every Vintage-playable (legal or restricted) Oracle
//! record outside Scryfall `set_type: funny`
//! (`docs/decisions/english-lexical-analysis.md` §1). Each Card Face record
//! yields one Type Line unit and one unit per paragraph line of its rules text
//! after balanced reminder text is removed (a line break inside a quotation
//! does not end a line); a face's count is a sum over its units. The split
//! is deterministic, so a text has one segmentation into units.

use std::collections::BTreeSet;
use std::io::BufRead;

use deckmaste_data::oracle_text::strip_reminder_text;
use deckmaste_data::scryfall::OracleCardReader;
use deckmaste_lexical::LexemeId;

/// One Card Face record of the supported corpus.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Face {
    /// The Oracle identity and face discriminator (`ORACLE_ID#face:N`).
    pub identity: String,
    /// The face's displayed name.
    pub name: String,
    /// The whole card's name.
    pub card: String,
    /// Card-name lexemes in scope on this face: every name of its card
    /// [CR#201.5c].
    pub scope: BTreeSet<LexemeId>,
    pub type_line: Option<String>,
    /// Rules text as printed, reminder text included.
    pub oracle_text: Option<String>,
}

/// What a Corpus Unit is.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd, serde::Serialize)]
pub enum UnitKind {
    TypeLine,
    /// One paragraph line of rules text.
    Line,
}

/// One Corpus Unit: the smallest span the census enumerates, validates and
/// counts.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Unit {
    /// Index of its face in [`Corpus::faces`].
    pub face: usize,
    pub kind: UnitKind,
    /// Position among the face's rules-text units; 0 for the type line.
    pub index: usize,
    pub text: String,
}

/// The supported corpus, split into units.
#[derive(Debug, Clone, Default)]
pub struct Corpus {
    pub faces: Vec<Face>,
    pub units: Vec<Unit>,
}

/// Why the corpus could not be read.
#[derive(Debug, thiserror::Error)]
pub enum CorpusError {
    #[error(transparent)]
    Data(#[from] deckmaste_data::DataError),
    #[error("{0}")]
    Read(String),
}

/// The lexeme id of a card name, as `deckmaste_lexical_source_v4` declares it.
#[must_use]
pub fn card_name_lexeme(name: &str) -> LexemeId {
    LexemeId::new(&format!("name:{name}"))
}

/// The rules-text units of a face: its paragraph lines after balanced
/// reminder text is removed, trimmed, without empty lines. A line break
/// inside a quotation that closes later in the text does not end a line.
#[must_use]
pub fn split_lines(oracle_text: &str) -> Vec<String> {
    let text = strip_reminder_text(oracle_text);
    let lines: Vec<&str> = text.lines().collect();
    let quotes = |line: &str| line.matches('"').count();
    let mut units = Vec::new();
    let mut start = 0;
    while start < lines.len() {
        let mut end = start + 1;
        let mut open = quotes(lines[start]) % 2 == 1;
        while open && let Some(close) = lines[end..].iter().position(|l| quotes(l) > 0) {
            end += close + 1;
            open = lines[start..end].iter().map(|l| quotes(l)).sum::<usize>() % 2 == 1;
        }
        let unit = lines[start..end].join("\n");
        let unit = unit.trim();
        if !unit.is_empty() {
            units.push(unit.to_owned());
        }
        start = end;
    }
    units
}

impl Corpus {
    /// Reads the supported corpus from an Oracle Cards JSON-lines stream,
    /// keeping the faces for which `keep(face)` holds.
    ///
    /// # Errors
    /// An unreadable or invalid record.
    pub fn read(reader: impl BufRead, keep: impl Fn(&Face) -> bool) -> Result<Self, CorpusError> {
        let mut corpus = Self::default();
        for card in OracleCardReader::new(reader) {
            let card = card.map_err(|e| CorpusError::Read(e.to_string()))?;
            if !card.supported() {
                continue;
            }
            let mut scope: BTreeSet<LexemeId> = card
                .oracle_units()
                .map(|unit| card_name_lexeme(unit.display_name()))
                .collect();
            scope.insert(card_name_lexeme(&card.name));
            for unit in card.oracle_units() {
                let face = Face {
                    identity: unit.identity().to_string(),
                    name: unit.display_name().to_owned(),
                    card: unit.group_name().to_owned(),
                    scope: scope.clone(),
                    type_line: unit.type_line().map(str::to_owned),
                    oracle_text: unit.oracle_text().map(str::to_owned),
                };
                if keep(&face) {
                    corpus.push(face);
                }
            }
        }
        Ok(corpus)
    }

    /// Reads the pinned snapshot at `data/scryfall/oracle-cards.jsonl`.
    ///
    /// # Errors
    /// As [`Corpus::read`], or a missing snapshot.
    pub fn load(keep: impl Fn(&Face) -> bool) -> Result<Self, CorpusError> {
        let reader = deckmaste_data::scryfall::oracle_cards()?;
        Self::read(reader.into_inner(), keep)
    }

    /// Adds a face and its units.
    pub fn push(&mut self, face: Face) {
        let index = self.faces.len();
        if let Some(type_line) = &face.type_line {
            self.units.push(Unit {
                face: index,
                kind: UnitKind::TypeLine,
                index: 0,
                text: type_line.trim().to_owned(),
            });
        }
        if let Some(text) = &face.oracle_text {
            for (line, text) in split_lines(text).into_iter().enumerate() {
                self.units.push(Unit {
                    face: index,
                    kind: UnitKind::Line,
                    index: line,
                    text,
                });
            }
        }
        self.faces.push(face);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lines_drop_reminder_text_and_blank_lines() {
        assert_eq!(
            split_lines(
                "Flying (This creature can't be blocked.)\n(Reminder only.)\n{T}: Add {G}."
            ),
            ["Flying", "{T}: Add {G}."]
        );
    }

    #[test]
    fn a_line_break_inside_a_quotation_does_not_end_a_line() {
        assert_eq!(
            split_lines("Creatures you control have \"{T}: Add {G}.\n{T}: Untap this.\"\nFlying"),
            [
                "Creatures you control have \"{T}: Add {G}.\n{T}: Untap this.\"",
                "Flying"
            ]
        );
        // A quotation that never closes does not swallow the lines after it.
        assert_eq!(
            split_lines("Say \"hello\nFlying"),
            ["Say \"hello", "Flying"]
        );
        assert_eq!(
            split_lines("\"A\" and \"B\nC\"\nD"),
            ["\"A\" and \"B\nC\"", "D"]
        );
    }

    #[test]
    fn a_face_yields_its_type_line_and_each_line() {
        let mut corpus = Corpus::default();
        corpus.push(Face {
            identity: "x#card".to_owned(),
            name: "Seedborn Muse".to_owned(),
            card: "Seedborn Muse".to_owned(),
            scope: BTreeSet::new(),
            type_line: Some("Creature — Spirit".to_owned()),
            oracle_text: Some(
                "Untap all permanents you control during each other player's untap step."
                    .to_owned(),
            ),
        });
        let units: Vec<_> = corpus
            .units
            .iter()
            .map(|u| (u.kind, u.text.as_str()))
            .collect();
        assert_eq!(
            units,
            [
                (UnitKind::TypeLine, "Creature — Spirit"),
                (
                    UnitKind::Line,
                    "Untap all permanents you control during each other player's untap step."
                ),
            ]
        );
    }
}
