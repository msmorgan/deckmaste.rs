//! The supported Oracle corpus and its Corpus Units.
//!
//! The supported corpus is every Vintage-playable (legal or restricted) Oracle
//! record outside Scryfall `set_type: funny`
//! (`docs/decisions/english-lexical-analysis.md` §1). Each Card Face record
//! yields one Type Line unit and one unit per paragraph line of its rules text
//! after balanced reminder text is removed (a line break inside a quotation
//! does not end a line); a face's count is a sum over its units. The split
//! is deterministic, so a text has one segmentation into units.
//!
//! A face's text is read with its recorded Editorial Corrections applied
//! ([`crate::EDITORIAL_CORRECTIONS`]): where the Oracle text breaks a
//! convention its printed card keeps, the census reads the corrected text and
//! the grammar is not relaxed for the slip.

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
    /// Card-name lexemes in scope on this face: its own name and its
    /// card's [CR#201.5c].
    pub scope: BTreeSet<LexemeId>,
    pub type_line: Option<String>,
    /// Oracle rules text, reminder text included, with the face's Editorial
    /// Corrections applied.
    pub oracle_text: Option<String>,
}

impl Face {
    /// The face's units as (kind, index, text): its trimmed type line, then
    /// its rules-text lines ([`split_lines`]).
    fn units(&self) -> impl Iterator<Item = (UnitKind, usize, String)> {
        let type_line = self
            .type_line
            .iter()
            .map(|line| (UnitKind::TypeLine, 0, line.trim().to_owned()));
        let lines = self
            .oracle_text
            .iter()
            .flat_map(|text| split_lines(text).into_iter().enumerate())
            .map(|(line, text)| (UnitKind::Line, line, text));
        type_line.chain(lines)
    }
}

/// One Editorial Correction: `corrected` read in place of `oracle`, which
/// occurs exactly once in the Oracle text of the face named `card`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct EditorialCorrection {
    pub card: &'static str,
    pub oracle: &'static str,
    pub corrected: &'static str,
    /// The printed card, or the template update, that the correction cites.
    pub evidence: &'static str,
}

impl EditorialCorrection {
    /// The record as one line, for reports.
    #[must_use]
    pub fn describe(&self) -> String {
        format!("{}: {:?} -> {:?}", self.card, self.oracle, self.corrected)
    }
}

/// The tracked Editorial Corrections: blank-line-separated records of
/// `card:`, `oracle:`, `corrected:` and `evidence:` lines, with `#` comments.
///
/// # Panics
/// On a malformed record, which `tests/census.rs` rejects first.
#[must_use]
pub fn editorial_corrections() -> Vec<EditorialCorrection> {
    let mut records = Vec::new();
    let mut fields: Vec<(&'static str, &'static str)> = Vec::new();
    let mut flush = |fields: &mut Vec<(&'static str, &'static str)>| {
        if fields.is_empty() {
            return;
        }
        let field = |key: &str| {
            let values: Vec<_> = fields.iter().filter(|(k, _)| *k == key).collect();
            match values.as_slice() {
                [(_, value)] if !value.is_empty() => *value,
                _ => panic!("editorial correction {fields:?} needs one non-empty `{key}:`"),
            }
        };
        assert_eq!(
            fields.len(),
            4,
            "editorial correction {fields:?} has unknown fields"
        );
        records.push(EditorialCorrection {
            card: field("card"),
            oracle: field("oracle"),
            corrected: field("corrected"),
            evidence: field("evidence"),
        });
        fields.clear();
    };
    for line in crate::EDITORIAL_CORRECTIONS.lines() {
        if line.starts_with('#') {
            continue;
        }
        if line.trim().is_empty() {
            flush(&mut fields);
            continue;
        }
        let (key, value) = line
            .split_once(": ")
            .unwrap_or_else(|| panic!("editorial correction line `{line}` is not `KEY: VALUE`"));
        fields.push((key, value.trim_end()));
    }
    flush(&mut fields);
    records
}

/// `text` with every correction for `name` applied, marking each applied one
/// in `applied`. A correction applies only where its Oracle text occurs once.
fn correct(
    name: &str,
    text: &str,
    corrections: &[EditorialCorrection],
    applied: &mut [bool],
) -> String {
    let mut text = text.to_owned();
    for (correction, applied) in corrections.iter().zip(applied) {
        if correction.card == name && text.matches(correction.oracle).count() == 1 {
            text = text.replacen(correction.oracle, correction.corrected, 1);
            *applied = true;
        }
    }
    text
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
    /// Editorial Corrections that matched no face of the stream read: the
    /// Oracle text they correct has changed or gone.
    pub stale_corrections: Vec<EditorialCorrection>,
    /// Deferral and declined lines (`NAME | TEXT`) that name no unit of any
    /// face of the stream read, kept or not: the unit they name has changed
    /// or gone.
    pub unmatched_deferrals: Vec<String>,
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
/// reminder text is removed (a keyword line's semicolon separators becoming
/// commas with it), trimmed, without empty lines. A line break
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
        let corrections = editorial_corrections();
        let mut applied = vec![false; corrections.len()];
        let mut unmatched: BTreeSet<(&'static str, &'static str)> = crate::census::deferrals();
        unmatched.extend(crate::census::declined());
        let mut corpus = Self::default();
        for card in OracleCardReader::new(reader) {
            let card = card.map_err(|e| CorpusError::Read(e.to_string()))?;
            if !card.supported() {
                continue;
            }
            for unit in card.oracle_units() {
                // A face's text shortens its own name: the faces of a
                // double-faced card are named apart [CR#712.8d,712.8e,712.8f],
                // and two faces that shorten to one spelling (*Ral*) name one
                // object, not two.
                let scope = BTreeSet::from([
                    card_name_lexeme(unit.display_name()),
                    card_name_lexeme(&card.name),
                ]);
                let face = Face {
                    identity: unit.identity().to_string(),
                    name: unit.display_name().to_owned(),
                    card: unit.group_name().to_owned(),
                    scope,
                    type_line: unit.type_line().map(str::to_owned),
                    oracle_text: unit
                        .oracle_text()
                        .map(|text| correct(unit.display_name(), text, &corrections, &mut applied)),
                };
                if !unmatched.is_empty() {
                    let texts: Vec<String> = face.units().map(|(_, _, text)| text).collect();
                    unmatched.retain(|&(name, text)| {
                        name != face.name || !texts.iter().any(|t| t == text)
                    });
                }
                if keep(&face) {
                    corpus.push(face);
                }
            }
        }
        corpus.stale_corrections = corrections
            .into_iter()
            .zip(applied)
            .filter_map(|(correction, applied)| (!applied).then_some(correction))
            .collect();
        corpus.unmatched_deferrals = unmatched
            .into_iter()
            .map(|(name, text)| format!("{name} | {text}"))
            .collect();
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
        self.units
            .extend(face.units().map(|(kind, line, text)| Unit {
                face: index,
                kind,
                index: line,
                text,
            }));
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
    fn keyword_semicolons_become_commas_with_the_reminder_text() {
        // Shu General's Oracle text.
        assert_eq!(
            split_lines(
                "Vigilance; horsemanship (This creature can't be blocked except by \
                 creatures with horsemanship.)"
            ),
            ["Vigilance, horsemanship"]
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
    fn a_recorded_editorial_correction_is_read_in_place_of_the_oracle_text() {
        // Agency Outfitter's Oracle text drops the serial comma its printed
        // card has; the correction is recorded in `corrections.txt`.
        let input = r#"{"object":"card","id":"p","oracle_id":"o","name":"Agency Outfitter","layout":"normal","legalities":{"vintage":"legal"},"type_line":"Creature — Sphinx Detective","oracle_text":"Search your graveyard, hand and/or library for a card."}"#;
        let corpus = Corpus::read(std::io::Cursor::new(input), |_| true).unwrap();
        assert_eq!(
            corpus.faces[0].oracle_text.as_deref(),
            Some("Search your graveyard, hand, and/or library for a card.")
        );
        assert!(
            corpus
                .stale_corrections
                .iter()
                .all(|c| c.card != "Agency Outfitter")
        );
    }

    #[test]
    fn a_correction_whose_oracle_text_is_gone_is_stale() {
        let input = r#"{"object":"card","id":"p","oracle_id":"o","name":"Agency Outfitter","layout":"normal","legalities":{"vintage":"legal"},"type_line":"Creature — Sphinx Detective","oracle_text":"Search your graveyard, hand, and/or library for a card."}"#;
        let corpus = Corpus::read(std::io::Cursor::new(input), |_| true).unwrap();
        assert_eq!(
            corpus.faces[0].oracle_text.as_deref(),
            Some("Search your graveyard, hand, and/or library for a card.")
        );
        assert!(
            corpus
                .stale_corrections
                .iter()
                .any(|c| c.card == "Agency Outfitter")
        );
    }

    #[test]
    fn a_deferral_matches_a_unit_of_any_face_read_kept_or_not() {
        // Any deferred or declined unit serves as the witness, both lists
        // matched alike: the first listed one, so that the test names no unit
        // a landing will remove, and holds while `deferred.txt` is empty.
        let (name, text) = crate::census::deferrals()
            .into_iter()
            .chain(crate::census::declined())
            .next()
            .expect("deferred.txt or declined.txt lists a unit");
        let line = format!("{name} | {text}");
        let escape = |s: &str| s.replace('\\', "\\\\").replace('"', "\\\"");
        let input = format!(
            r#"{{"object":"card","id":"p","oracle_id":"o","name":"{}","layout":"normal","legalities":{{"vintage":"legal"}},"type_line":"Creature — Demon","oracle_text":"Fear\n{}"}}"#,
            escape(name),
            escape(text)
        );
        let corpus = Corpus::read(std::io::Cursor::new(input), |_| false).unwrap();
        assert!(!corpus.unmatched_deferrals.contains(&line));
        // Every other deferred or declined line names no unit of this stream.
        assert_eq!(
            corpus.unmatched_deferrals.len(),
            crate::census::deferrals().len() + crate::census::declined().len() - 1
        );
        let corpus = Corpus::read(std::io::Cursor::new(""), |_| true).unwrap();
        assert!(corpus.unmatched_deferrals.contains(&line));
    }

    #[test]
    fn a_face_has_its_own_name_in_scope_and_not_its_other_face_s() {
        // Both faces shorten to *Ral*; each face's text names only itself.
        let input = r#"{"object":"card","id":"p","oracle_id":"o","name":"Ral, Monsoon Mage // Ral, Leyline Prodigy","layout":"transform","legalities":{"vintage":"legal"},"card_faces":[{"name":"Ral, Monsoon Mage","type_line":"Creature — Human Wizard","oracle_text":"Transform Ral."},{"name":"Ral, Leyline Prodigy","type_line":"Legendary Planeswalker — Ral","oracle_text":"Ral deals 2 damage to any target."}]}"#;
        let corpus = Corpus::read(std::io::Cursor::new(input), |_| true).unwrap();
        let card = card_name_lexeme("Ral, Monsoon Mage // Ral, Leyline Prodigy");
        let scopes: Vec<_> = corpus.faces.iter().map(|f| f.scope.clone()).collect();
        assert_eq!(
            scopes,
            [
                BTreeSet::from([card_name_lexeme("Ral, Monsoon Mage"), card]),
                BTreeSet::from([card_name_lexeme("Ral, Leyline Prodigy"), card]),
            ]
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
