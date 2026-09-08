use std::collections::HashMap;
use std::fmt;
use std::io;
use std::io::BufRead;

use anyhow::Context;
use serde::Deserialize;
use serde::Serialize;
use thiserror::Error;

use crate::DataRoot;
use crate::DataStr;

/// A scryfall catalog: a named list of strings. Only the list is modeled.
#[derive(Debug, Clone, Deserialize)]
pub struct Catalog<'a> {
    #[serde(borrow)]
    pub data: Vec<DataStr<'a>>,
}

impl<'a> Catalog<'a> {
    /// Parses a Scryfall catalog snapshot.
    ///
    /// # Errors
    ///
    /// Returns an error when `bytes` is not valid catalog JSON.
    pub fn parse(bytes: &'a [u8]) -> serde_json::Result<Self> {
        serde_json::from_slice(bytes)
    }
}

/// Reads a scryfall catalog file (e.g. "creature-types"); parse with
/// [`Catalog::parse`], which borrows from the returned bytes.
///
/// # Errors
///
/// Returns an error when the named catalog snapshot cannot be read.
pub fn catalog_bytes(name: &str) -> anyhow::Result<Vec<u8>> {
    DataRoot::workspace_default().read(format!("catalogs/{name}.json"))
}

/// Default upper bound for one decompressed Oracle Cards JSONL record.
///
/// The reader never buffers more than this many record bytes. Scryfall card
/// objects are far smaller; the generous bound makes an upstream shape change
/// fail explicitly instead of allowing unbounded growth on a missing newline.
pub const MAX_ORACLE_CARD_BYTES: usize = 1024 * 1024;

/// One owned card object from Scryfall's Oracle Cards JSONL export.
///
/// `card_faces` is Scryfall transport data. It must not be interpreted as a
/// list of Game Model Card Faces: flip and adventurer cards use the second
/// record for Alternative Characteristics.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct OracleCard {
    object: String,
    /// The representative printing chosen by Scryfall for this export row.
    #[serde(rename = "id")]
    pub printing_id: String,
    /// The durable Oracle identity shared by printings of this card.
    #[serde(default)]
    pub oracle_id: Option<String>,
    pub name: String,
    pub layout: String,
    #[serde(default)]
    pub mana_cost: Option<String>,
    #[serde(default, rename = "cmc")]
    pub mana_value: Option<f64>,
    #[serde(default)]
    pub type_line: Option<String>,
    #[serde(default)]
    pub oracle_text: Option<String>,
    #[serde(default)]
    pub color_indicator: Option<Vec<String>>,
    #[serde(default)]
    pub colors: Option<Vec<String>>,
    #[serde(default)]
    pub color_identity: Option<Vec<String>>,
    #[serde(default)]
    pub keywords: Vec<String>,
    #[serde(default)]
    pub power: Option<String>,
    #[serde(default)]
    pub toughness: Option<String>,
    #[serde(default)]
    pub loyalty: Option<String>,
    #[serde(default)]
    pub defense: Option<String>,
    #[serde(default)]
    pub legalities: Legalities,
    #[serde(default)]
    pub card_faces: Vec<ScryfallCardFace>,
    #[serde(default)]
    pub all_parts: Vec<RelatedCard>,
}

impl OracleCard {
    #[must_use]
    pub fn vintage_playable(&self) -> bool {
        let root = matches!(
            self.legalities.vintage.as_deref(),
            Some("legal" | "restricted")
        );
        root || (self.layout == "reversible_card"
            && self.card_faces.iter().any(|face| {
                matches!(
                    face.legalities
                        .as_ref()
                        .and_then(|legalities| legalities.vintage.as_deref()),
                    Some("legal" | "restricted")
                )
            }))
    }

    /// Visits the one top-level source record, or each ordered Scryfall
    /// `card_faces` record when the object has several displayed records.
    #[must_use]
    pub fn oracle_units(&self) -> OracleUnits<'_> {
        OracleUnits {
            card: self,
            next_index: 0,
        }
    }
}

/// A record inside Scryfall's `card_faces` array.
///
/// This upstream spelling is deliberately retained because the record may
/// describe Alternative Characteristics rather than a Game Model Card Face.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ScryfallCardFace {
    #[serde(default)]
    pub oracle_id: Option<String>,
    pub name: String,
    #[serde(default)]
    pub layout: Option<String>,
    #[serde(default)]
    pub mana_cost: Option<String>,
    #[serde(default, rename = "cmc")]
    pub mana_value: Option<f64>,
    #[serde(default)]
    pub type_line: Option<String>,
    #[serde(default)]
    pub oracle_text: Option<String>,
    #[serde(default)]
    pub color_indicator: Option<Vec<String>>,
    #[serde(default)]
    pub colors: Option<Vec<String>>,
    #[serde(default)]
    pub color_identity: Option<Vec<String>>,
    #[serde(default)]
    pub keywords: Option<Vec<String>>,
    #[serde(default)]
    pub legalities: Option<Legalities>,
    #[serde(default)]
    pub power: Option<String>,
    #[serde(default)]
    pub toughness: Option<String>,
    #[serde(default)]
    pub loyalty: Option<String>,
    #[serde(default)]
    pub defense: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Legalities {
    #[serde(default)]
    pub vintage: Option<String>,
    /// Preserve unknown formats without making them part of the interface.
    #[serde(flatten)]
    _other: HashMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct RelatedCard {
    #[serde(rename = "id")]
    pub printing_id: String,
    pub component: String,
    pub name: String,
    pub type_line: String,
}

/// Whether a source record denotes printed Card Face characteristics or the
/// Alternative Characteristics carried by a flip/adventurer card.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OracleUnitKind {
    CardFace,
    AlternativeCharacteristics,
}

/// A borrowed source unit projected from one Oracle Cards record.
#[derive(Debug, Clone, Copy)]
pub struct OracleUnit<'a> {
    card: &'a OracleCard,
    face: Option<&'a ScryfallCardFace>,
    index: Option<usize>,
}

impl OracleUnit<'_> {
    /// Returns the durable Oracle identity and face discriminator for this
    /// unit.
    ///
    /// # Panics
    ///
    /// Panics if the unit did not come from [`OracleCardReader`]'s validated
    /// stream and has neither a card-level nor face-level Oracle ID.
    #[must_use]
    pub fn identity(&self) -> OracleUnitIdentity<'_> {
        let oracle_id = if self.card.layout == "reversible_card" {
            self.face_oracle_id().or(self.card.oracle_id.as_deref())
        } else {
            self.card
                .oracle_id
                .as_deref()
                .or_else(|| self.face_oracle_id())
        }
        .expect("OracleCardReader validates an Oracle identity for every source unit");
        OracleUnitIdentity {
            oracle_id,
            face_index: self.index,
        }
    }

    #[must_use]
    pub fn printing_id(&self) -> &str {
        &self.card.printing_id
    }

    #[must_use]
    pub fn group_name(&self) -> &str {
        &self.card.name
    }

    #[must_use]
    pub fn display_name(&self) -> &str {
        self.face.map_or(self.card.name.as_str(), |face| &face.name)
    }

    #[must_use]
    pub fn face_index(&self) -> Option<usize> {
        self.index
    }

    #[must_use]
    pub fn face_oracle_id(&self) -> Option<&str> {
        self.face.and_then(|face| face.oracle_id.as_deref())
    }

    #[must_use]
    pub fn kind(&self) -> OracleUnitKind {
        if self.index.is_some_and(|index| {
            index > 0 && matches!(self.card.layout.as_str(), "flip" | "adventure")
        }) {
            OracleUnitKind::AlternativeCharacteristics
        } else {
            OracleUnitKind::CardFace
        }
    }

    #[must_use]
    pub fn layout(&self) -> &str {
        self.face
            .and_then(|face| face.layout.as_deref())
            .unwrap_or(&self.card.layout)
    }

    #[must_use]
    pub fn mana_cost(&self) -> Option<&str> {
        self.face.map_or(self.card.mana_cost.as_deref(), |face| {
            face.mana_cost.as_deref()
        })
    }

    #[must_use]
    pub fn mana_value(&self) -> Option<f64> {
        self.face
            .and_then(|face| face.mana_value)
            .or(self.card.mana_value)
    }

    #[must_use]
    pub fn type_line(&self) -> Option<&str> {
        self.face.map_or(self.card.type_line.as_deref(), |face| {
            face.type_line.as_deref()
        })
    }

    #[must_use]
    pub fn oracle_text(&self) -> Option<&str> {
        self.face.map_or(self.card.oracle_text.as_deref(), |face| {
            face.oracle_text.as_deref()
        })
    }

    #[must_use]
    pub fn color_indicator(&self) -> Option<&[String]> {
        self.face
            .map_or(self.card.color_indicator.as_deref(), |face| {
                face.color_indicator.as_deref()
            })
    }

    #[must_use]
    pub fn colors(&self) -> Option<&[String]> {
        self.face
            .map_or(self.card.colors.as_deref(), |face| face.colors.as_deref())
    }

    #[must_use]
    pub fn color_identity(&self) -> Option<&[String]> {
        self.face
            .and_then(|face| face.color_identity.as_deref())
            .or(self.card.color_identity.as_deref())
    }

    #[must_use]
    pub fn keywords(&self) -> &[String] {
        self.face
            .and_then(|face| face.keywords.as_deref())
            .unwrap_or(&self.card.keywords)
    }

    #[must_use]
    pub fn power(&self) -> Option<&str> {
        self.face
            .map_or(self.card.power.as_deref(), |face| face.power.as_deref())
    }

    #[must_use]
    pub fn toughness(&self) -> Option<&str> {
        self.face.map_or(self.card.toughness.as_deref(), |face| {
            face.toughness.as_deref()
        })
    }

    #[must_use]
    pub fn loyalty(&self) -> Option<&str> {
        self.face
            .map_or(self.card.loyalty.as_deref(), |face| face.loyalty.as_deref())
    }

    #[must_use]
    pub fn defense(&self) -> Option<&str> {
        self.face
            .map_or(self.card.defense.as_deref(), |face| face.defense.as_deref())
    }

    #[must_use]
    pub fn related_cards(&self) -> &[RelatedCard] {
        &self.card.all_parts
    }
}

/// Durable Oracle identity plus the documented within-object discriminator.
///
/// `#card` denotes top-level characteristics. `#face:N` denotes the Nth
/// ordered Scryfall `card_faces` record. Printing IDs, names, and text hashes
/// remain separate provenance fields and do not participate in this key.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd)]
pub struct OracleUnitIdentity<'a> {
    pub oracle_id: &'a str,
    pub face_index: Option<usize>,
}

impl fmt::Display for OracleUnitIdentity<'_> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{}#", self.oracle_id)?;
        match self.face_index {
            Some(index) => write!(formatter, "face:{index}"),
            None => formatter.write_str("card"),
        }
    }
}

pub struct OracleUnits<'a> {
    card: &'a OracleCard,
    next_index: usize,
}

impl<'a> Iterator for OracleUnits<'a> {
    type Item = OracleUnit<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.card.card_faces.is_empty() {
            if self.next_index > 0 {
                return None;
            }
            self.next_index = 1;
            return Some(OracleUnit {
                card: self.card,
                face: None,
                index: None,
            });
        }
        let index = self.next_index;
        let face = self.card.card_faces.get(index)?;
        self.next_index += 1;
        Some(OracleUnit {
            card: self.card,
            face: Some(face),
            index: Some(index),
        })
    }
}

#[derive(Debug, Error)]
pub enum OracleCardReadError {
    #[error("reading Oracle Cards JSONL record {record}")]
    Read {
        record: usize,
        #[source]
        source: io::Error,
    },
    #[error("Oracle Cards JSONL record {record} exceeds the {max_bytes}-byte limit")]
    RecordTooLarge { record: usize, max_bytes: usize },
    #[error("Oracle Cards JSONL record {record} is empty")]
    EmptyRecord { record: usize },
    #[error("parsing Oracle Cards JSONL record {record}")]
    Json {
        record: usize,
        #[source]
        source: serde_json::Error,
    },
    #[error("Oracle Cards JSONL record {record} has object {actual:?}, expected \"card\"")]
    WrongObject { record: usize, actual: String },
    #[error("Oracle Cards JSONL record {record} has a source unit without an Oracle identity")]
    MissingOracleId { record: usize },
    #[error("duplicate Oracle identity {oracle_id:?} in records {first_record} and {record}")]
    DuplicateOracleId {
        oracle_id: String,
        first_record: usize,
        record: usize,
    },
    #[error(
        "duplicate representative printing identity {printing_id:?} in records {first_record} and {record}"
    )]
    DuplicatePrintingId {
        printing_id: String,
        first_record: usize,
        record: usize,
    },
}

/// Streaming decoder for Scryfall's decompressed Oracle Cards JSONL export.
///
/// It holds one bounded record buffer plus identity sets used to reject
/// duplicates; it never builds a card-name map or retains decoded cards.
pub struct OracleCardReader<R> {
    reader: R,
    max_record_bytes: usize,
    record: usize,
    failed: bool,
    seen_oracle_ids: HashMap<String, usize>,
    seen_printing_ids: HashMap<String, usize>,
}

impl<R: BufRead> OracleCardReader<R> {
    #[must_use]
    pub fn new(reader: R) -> Self {
        Self::with_max_record_bytes(reader, MAX_ORACLE_CARD_BYTES)
    }

    #[must_use]
    pub fn with_max_record_bytes(reader: R, max_record_bytes: usize) -> Self {
        Self {
            reader,
            max_record_bytes,
            record: 0,
            failed: false,
            seen_oracle_ids: HashMap::new(),
            seen_printing_ids: HashMap::new(),
        }
    }

    fn read_record(&mut self) -> Result<Option<Vec<u8>>, OracleCardReadError> {
        let record = self.record + 1;
        let mut bytes = Vec::new();
        loop {
            let available = self
                .reader
                .fill_buf()
                .map_err(|source| OracleCardReadError::Read { record, source })?;
            if available.is_empty() {
                if bytes.is_empty() {
                    return Ok(None);
                }
                break;
            }
            if let Some(newline) = available.iter().position(|byte| *byte == b'\n') {
                bytes.extend_from_slice(&available[..newline]);
                self.reader.consume(newline + 1);
                if bytes.last() == Some(&b'\r') {
                    bytes.pop();
                }
                if bytes.len() > self.max_record_bytes {
                    return Err(OracleCardReadError::RecordTooLarge {
                        record,
                        max_bytes: self.max_record_bytes,
                    });
                }
                break;
            }
            let remaining = self
                .max_record_bytes
                .saturating_add(1)
                .saturating_sub(bytes.len());
            let consumed = available.len().min(remaining);
            bytes.extend_from_slice(&available[..consumed]);
            self.reader.consume(consumed);
            if bytes.len() > self.max_record_bytes {
                return Err(OracleCardReadError::RecordTooLarge {
                    record,
                    max_bytes: self.max_record_bytes,
                });
            }
        }
        Ok(Some(bytes))
    }

    /// Returns the underlying buffered input after iteration completes.
    pub fn into_inner(self) -> R {
        self.reader
    }
}

impl<R: BufRead> Iterator for OracleCardReader<R> {
    type Item = Result<OracleCard, OracleCardReadError>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.failed {
            return None;
        }
        let bytes = match self.read_record() {
            Ok(Some(bytes)) => bytes,
            Ok(None) => return None,
            Err(error) => {
                self.failed = true;
                return Some(Err(error));
            }
        };
        self.record += 1;
        if bytes.is_empty() {
            self.failed = true;
            return Some(Err(OracleCardReadError::EmptyRecord {
                record: self.record,
            }));
        }
        let card: OracleCard = match serde_json::from_slice(&bytes) {
            Ok(card) => card,
            Err(source) => {
                self.failed = true;
                return Some(Err(OracleCardReadError::Json {
                    record: self.record,
                    source,
                }));
            }
        };
        if card.object != "card" {
            self.failed = true;
            return Some(Err(OracleCardReadError::WrongObject {
                record: self.record,
                actual: card.object,
            }));
        }
        let mut oracle_ids = Vec::new();
        if let Some(oracle_id) = card.oracle_id.as_ref() {
            oracle_ids.push(oracle_id);
        }
        for oracle_id in card
            .card_faces
            .iter()
            .filter_map(|face| face.oracle_id.as_ref())
        {
            if !oracle_ids.contains(&oracle_id) {
                oracle_ids.push(oracle_id);
            }
        }
        if oracle_ids.is_empty()
            || (card.oracle_id.is_none()
                && (card.card_faces.is_empty()
                    || card.card_faces.iter().any(|face| face.oracle_id.is_none())))
        {
            self.failed = true;
            return Some(Err(OracleCardReadError::MissingOracleId {
                record: self.record,
            }));
        }
        for oracle_id in oracle_ids {
            if let Some(first_record) = self.seen_oracle_ids.insert(oracle_id.clone(), self.record)
            {
                self.failed = true;
                return Some(Err(OracleCardReadError::DuplicateOracleId {
                    oracle_id: oracle_id.clone(),
                    first_record,
                    record: self.record,
                }));
            }
        }
        if let Some(first_record) = self
            .seen_printing_ids
            .insert(card.printing_id.clone(), self.record)
        {
            self.failed = true;
            return Some(Err(OracleCardReadError::DuplicatePrintingId {
                printing_id: card.printing_id,
                first_record,
                record: self.record,
            }));
        }
        Some(Ok(card))
    }
}

/// Opens the pinned decompressed Oracle Cards snapshot.
///
/// # Errors
///
/// Returns an error when `data/scryfall/oracle-cards.jsonl` cannot be opened.
pub fn oracle_cards() -> anyhow::Result<OracleCardReader<io::BufReader<std::fs::File>>> {
    let path = DataRoot::workspace_default()
        .0
        .join("scryfall/oracle-cards.jsonl");
    let file = std::fs::File::open(&path)
        .map_err(anyhow::Error::from)
        .with_context(|| format!("opening {}", path.display()))?;
    Ok(OracleCardReader::new(io::BufReader::new(file)))
}

#[cfg(test)]
mod tests {
    use std::io::Cursor;

    use super::*;

    #[test]
    fn oracle_jsonl_stream_preserves_single_face_source_fields() {
        let input = r#"{"object":"card","id":"printing-a","oracle_id":"oracle-a","name":"Café Mage","layout":"normal","mana_cost":"","cmc":2.0,"type_line":"Creature — Human Wizard","oracle_text":"First line\nSecond line ☃","colors":[],"color_identity":["U"],"keywords":["Ward"],"legalities":{"vintage":"restricted"}}"#;

        let cards = OracleCardReader::new(Cursor::new(input))
            .collect::<Result<Vec<_>, _>>()
            .unwrap();

        assert_eq!(cards.len(), 1);
        let card = &cards[0];
        assert_eq!(card.oracle_id.as_deref(), Some("oracle-a"));
        assert_eq!(card.printing_id, "printing-a");
        assert!(card.vintage_playable());
        let units = card.oracle_units().collect::<Vec<_>>();
        assert_eq!(units.len(), 1);
        assert_eq!(units[0].identity().to_string(), "oracle-a#card");
        assert_eq!(units[0].display_name(), "Café Mage");
        assert_eq!(units[0].mana_cost(), Some(""));
        assert_eq!(units[0].oracle_text(), Some("First line\nSecond line ☃"));
        assert_eq!(units[0].colors(), Some(&[][..]));
    }

    #[test]
    fn ordered_face_records_pin_layout_kinds_and_field_inheritance() {
        let layouts = [
            (
                "split",
                [OracleUnitKind::CardFace, OracleUnitKind::CardFace],
            ),
            (
                "adventure",
                [
                    OracleUnitKind::CardFace,
                    OracleUnitKind::AlternativeCharacteristics,
                ],
            ),
            (
                "flip",
                [
                    OracleUnitKind::CardFace,
                    OracleUnitKind::AlternativeCharacteristics,
                ],
            ),
            (
                "transform",
                [OracleUnitKind::CardFace, OracleUnitKind::CardFace],
            ),
            (
                "modal_dfc",
                [OracleUnitKind::CardFace, OracleUnitKind::CardFace],
            ),
            (
                "reversible_card",
                [OracleUnitKind::CardFace, OracleUnitKind::CardFace],
            ),
        ];

        for (layout, expected_kinds) in layouts {
            let input = r#"{"object":"card","id":"printing-$LAYOUT","oracle_id":"oracle-$LAYOUT","name":"Front // Back","layout":"$LAYOUT","mana_cost":"{9}","cmc":9.0,"oracle_text":"combined text must not leak","colors":["G"],"color_identity":["W","U"],"legalities":{"vintage":"legal"},"card_faces":[{"oracle_id":"face-front","name":"Front","mana_cost":"","cmc":1.0,"type_line":"Creature — Human","oracle_text":"front\ntext","colors":[]},{"oracle_id":"face-back","name":"Back","type_line":"Instant — Adventure","oracle_text":"","power":"*"}]}"#.replace("$LAYOUT", layout);
            let card = OracleCardReader::new(Cursor::new(input))
                .next()
                .unwrap()
                .unwrap();
            let units = card.oracle_units().collect::<Vec<_>>();

            assert_eq!(units.len(), 2, "{layout}");
            let front_oracle_id = if layout == "reversible_card" {
                "face-front".to_owned()
            } else {
                format!("oracle-{layout}")
            };
            assert_eq!(
                units[0].identity().to_string(),
                format!("{front_oracle_id}#face:0")
            );
            assert_eq!(
                units[1].identity().to_string(),
                if layout == "reversible_card" {
                    "face-back#face:1".to_owned()
                } else {
                    format!("oracle-{layout}#face:1")
                }
            );
            assert_eq!(units[0].face_oracle_id(), Some("face-front"));
            assert_eq!(units[1].face_oracle_id(), Some("face-back"));
            assert_eq!(units[0].kind(), expected_kinds[0]);
            assert_eq!(units[1].kind(), expected_kinds[1]);
            assert_eq!(units[0].oracle_text(), Some("front\ntext"));
            assert_eq!(units[1].oracle_text(), Some(""));
            assert_eq!(units[0].mana_cost(), Some(""));
            assert_eq!(units[1].mana_cost(), None);
            assert_eq!(units[0].mana_value(), Some(1.0));
            assert_eq!(units[1].mana_value(), Some(9.0));
            assert_eq!(units[0].colors(), Some(&[][..]));
            assert_eq!(units[1].colors(), None);
            assert_eq!(units[1].power(), Some("*"));
            assert_eq!(
                units[1]
                    .color_identity()
                    .unwrap()
                    .iter()
                    .map(String::as_str)
                    .collect::<Vec<_>>(),
                ["W", "U"]
            );
        }

        let meld = r#"{"object":"card","id":"printing-meld","oracle_id":"oracle-meld","name":"Meld Part","layout":"meld","cmc":3.0,"type_line":"Creature — Human","colors":["W"],"color_identity":["W"],"legalities":{"vintage":"legal"},"all_parts":[{"object":"related_card","id":"related-printing","component":"meld_part","name":"Other Half","type_line":"Creature — Angel","uri":"https://api.scryfall.com/cards/related-printing"}]}"#;
        let card = OracleCardReader::new(Cursor::new(meld))
            .next()
            .unwrap()
            .unwrap();
        let unit = card.oracle_units().next().unwrap();
        assert_eq!(unit.identity().to_string(), "oracle-meld#card");
        assert_eq!(unit.related_cards().len(), 1);
        assert_eq!(unit.related_cards()[0].component, "meld_part");
        assert_eq!(unit.related_cards()[0].name, "Other Half");
    }

    #[test]
    fn reader_bounds_records_and_reports_corrupt_or_duplicate_input() {
        let card = r#"{"object":"card","id":"printing","oracle_id":"oracle","name":"Test","layout":"normal","legalities":{"vintage":"legal"}}"#;
        let terminated = format!("{card}\n");
        let decoded =
            OracleCardReader::with_max_record_bytes(Cursor::new(terminated.as_bytes()), card.len())
                .collect::<Result<Vec<_>, _>>()
                .unwrap();
        assert_eq!(
            decoded.len(),
            1,
            "the JSONL terminator is not record content"
        );

        let too_small =
            OracleCardReader::with_max_record_bytes(Cursor::new(card.as_bytes()), card.len() - 1)
                .next()
                .unwrap()
                .unwrap_err();
        assert!(matches!(
            too_small,
            OracleCardReadError::RecordTooLarge { record: 1, .. }
        ));

        let duplicate = format!("{card}\n{card}\n");
        let error = OracleCardReader::new(Cursor::new(duplicate))
            .collect::<Result<Vec<_>, _>>()
            .unwrap_err();
        assert!(matches!(
            error,
            OracleCardReadError::DuplicateOracleId {
                first_record: 1,
                record: 2,
                ..
            }
        ));

        let truncated = &card[..card.len() - 1];
        let error = OracleCardReader::new(Cursor::new(truncated))
            .next()
            .unwrap()
            .unwrap_err();
        assert!(matches!(error, OracleCardReadError::Json { record: 1, .. }));
    }

    #[test]
    fn reversible_faces_keep_their_own_oracle_identities() {
        let input = r#"{"object":"card","id":"printing-reversible","name":"Front // Back","layout":"reversible_card","legalities":{"vintage":"legal"},"card_faces":[{"oracle_id":"oracle-front","name":"Front","layout":"normal","cmc":2.0,"mana_cost":"{2}","type_line":"Artifact","oracle_text":"Front text","colors":[]},{"oracle_id":"oracle-back","name":"Back","layout":"normal","cmc":4.0,"mana_cost":"{4}","type_line":"Creature — Robot","oracle_text":"Back text","colors":[]}]}"#;

        let card = OracleCardReader::new(Cursor::new(input))
            .next()
            .unwrap()
            .unwrap();
        let units = card.oracle_units().collect::<Vec<_>>();

        assert_eq!(units[0].identity().to_string(), "oracle-front#face:0");
        assert_eq!(units[1].identity().to_string(), "oracle-back#face:1");
        assert_eq!(units[0].mana_value(), Some(2.0));
        assert_eq!(units[1].mana_value(), Some(4.0));
    }
}
