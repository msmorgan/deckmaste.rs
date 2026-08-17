use std::fmt::Write as _;
use std::fs;
use std::path::Path;

use anyhow::Context;
use deckmaste_data::mtgjson::AtomicCards;
use deckmaste_english_v2::context::ParseContext;
use sha2::Digest;
use sha2::Sha256;

const ID_DOMAIN: &[u8] = b"deckmaste:english-v2:corpus-unit:v1";

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize)]
pub(super) struct CorpusUnit {
    id: String,
    card_name: String,
    face_name: Option<String>,
    side: Option<String>,
    context_name: String,
    text: String,
}

impl CorpusUnit {
    pub(super) fn id(&self) -> &str {
        &self.id
    }

    pub(super) fn card_name(&self) -> &str {
        &self.card_name
    }

    pub(super) fn face_name(&self) -> Option<&str> {
        self.face_name.as_deref()
    }

    pub(super) fn side(&self) -> Option<&str> {
        self.side.as_deref()
    }

    pub(super) fn context_name(&self) -> &str {
        &self.context_name
    }

    pub(super) fn text(&self) -> &str {
        &self.text
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) struct Corpus {
    source_fingerprint: String,
    units: Vec<CorpusUnit>,
}

impl Corpus {
    pub(super) fn from_bytes(bytes: &[u8]) -> anyhow::Result<Self> {
        let cards = AtomicCards::parse(bytes).context("parsing MTGJSON atomic-card snapshot")?;
        let mut units = cards
            .data
            .values()
            .flat_map(|cards| cards.iter())
            .filter(|card| card.vintage_playable())
            .filter_map(|card| {
                let text = card.text.as_deref().filter(|text| !text.is_empty())?;
                let card_name = card.name.to_string();
                let face_name = card.face_name.as_deref().map(str::to_owned);
                let side = card.side.as_deref().map(str::to_owned);
                let context_name = face_name.clone().unwrap_or_else(|| card_name.clone());
                let text = normalize_oracle_text(text);
                Some(corpus_unit(
                    &card_name,
                    face_name.as_deref(),
                    side.as_deref(),
                    &context_name,
                    &text,
                ))
            })
            .collect::<Vec<_>>();
        units.sort_by(|left, right| corpus_sort_key(left).cmp(&corpus_sort_key(right)));
        validate_contexts(&units)?;

        Ok(Self {
            source_fingerprint: sha256_hex(&Sha256::digest(bytes)),
            units,
        })
    }

    pub(super) fn load(path: &Path) -> anyhow::Result<Self> {
        let bytes = fs::read(path).with_context(|| format!("reading {}", path.display()))?;
        Self::from_bytes(&bytes)
    }

    pub(super) fn source_fingerprint(&self) -> &str {
        &self.source_fingerprint
    }

    pub(super) fn units(&self) -> &[CorpusUnit] {
        &self.units
    }
}

fn validate_contexts(units: &[CorpusUnit]) -> anyhow::Result<()> {
    for unit in units {
        if ParseContext::new(unit.context_name()).is_some() {
            continue;
        }
        let reason = if unit.context_name().is_empty() {
            "context name is empty"
        } else {
            "card-name abbreviation before comma is empty"
        };
        anyhow::bail!(
            "invalid parser context: {reason}; card name {:?}, face name {:?}, side {:?}",
            unit.card_name(),
            unit.face_name(),
            unit.side(),
        );
    }
    Ok(())
}

fn corpus_unit(
    card_name: &str,
    face_name: Option<&str>,
    side: Option<&str>,
    context_name: &str,
    text: &str,
) -> CorpusUnit {
    let mut hasher = Sha256::new();
    hasher.update(ID_DOMAIN);
    for field in [
        card_name,
        face_name.unwrap_or_default(),
        side.unwrap_or_default(),
        context_name,
        text,
    ] {
        hasher.update((field.len() as u64).to_be_bytes());
        hasher.update(field.as_bytes());
    }

    CorpusUnit {
        id: sha256_hex(&hasher.finalize()),
        card_name: card_name.to_owned(),
        face_name: face_name.map(str::to_owned),
        side: side.map(str::to_owned),
        context_name: context_name.to_owned(),
        text: text.to_owned(),
    }
}

fn sha256_hex(digest: &[u8]) -> String {
    let mut hexadecimal = String::with_capacity(digest.len() * 2);
    for byte in digest {
        write!(&mut hexadecimal, "{byte:02x}").expect("writing to String cannot fail");
    }
    hexadecimal
}

fn corpus_sort_key(unit: &CorpusUnit) -> (&str, Option<&str>, Option<&str>, &str, &str, &str) {
    (
        &unit.card_name,
        unit.face_name.as_deref(),
        unit.side.as_deref(),
        &unit.context_name,
        &unit.text,
        &unit.id,
    )
}

fn normalize_oracle_text(text: &str) -> String {
    normalize_roll_row_dashes(&deckmaste_data::academyruins::normalize_quotes(text))
}

fn normalize_roll_row_dashes(text: &str) -> String {
    text.split('\n')
        .map(normalize_roll_row_line)
        .collect::<Vec<_>>()
        .join("\n")
}

fn normalize_roll_row_line(line: &str) -> String {
    let low_end = line
        .find(|character: char| !character.is_ascii_digit())
        .unwrap_or(line.len());
    if low_end == 0 {
        return line.to_owned();
    }
    let Some(dash) = line[low_end..].chars().next() else {
        return line.to_owned();
    };
    if !matches!(dash, '-' | '—') {
        return line.to_owned();
    }
    let high_start = low_end + dash.len_utf8();
    let high_len = line[high_start..]
        .find(|character: char| !character.is_ascii_digit())
        .unwrap_or(line.len() - high_start);
    if high_len == 0 || !line[high_start + high_len..].starts_with(" |") {
        return line.to_owned();
    }
    format!("{}–{}", &line[..low_end], &line[high_start..])
}

#[cfg(test)]
impl CorpusUnit {
    pub(super) fn for_test(card_name: &str, text: &str) -> Self {
        corpus_unit(
            card_name,
            None,
            None,
            card_name,
            &normalize_oracle_text(text),
        )
    }
}

#[cfg(test)]
impl Corpus {
    pub(super) fn from_units_for_test(mut units: Vec<CorpusUnit>) -> Self {
        units.sort_by(|left, right| corpus_sort_key(left).cmp(&corpus_sort_key(right)));
        validate_contexts(&units).expect("test corpus must contain valid parser contexts");
        Self {
            source_fingerprint: "0".repeat(64),
            units,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const SNAPSHOT_A: &[u8] = br#"{
        "data": {
            "Restricted": [{
                "name": "Restricted", "layout": "normal", "types": ["Creature"],
                "supertypes": [], "subtypes": [], "legalities": {"vintage": "Restricted"},
                "text": "Restricted text."
            }],
            "Front // Back": [{
                "name": "Front // Back", "faceName": "Front", "side": "a",
                "layout": "modal_dfc", "types": ["Creature"], "supertypes": [],
                "subtypes": [], "legalities": {"vintage": "Legal"},
                "text": "1\u20142 | Choose one."
            }, {
                "name": "Front // Back", "faceName": "Back", "side": "b",
                "layout": "modal_dfc", "types": ["Creature"], "supertypes": [],
                "subtypes": [], "legalities": {"vintage": "Banned"},
                "text": "Banned back."
            }],
            "Alpha": [{
                "name": "Alpha", "layout": "normal", "types": ["Creature"],
                "supertypes": [], "subtypes": [], "legalities": {"vintage": "Legal"},
                "text": "\u2018Alpha\u2019"
            }],
            "Banned": [{
                "name": "Banned", "layout": "normal", "types": ["Creature"],
                "supertypes": [], "subtypes": [], "legalities": {"vintage": "Banned"},
                "text": "Banned text."
            }],
            "Empty": [{
                "name": "Empty", "layout": "normal", "types": ["Creature"],
                "supertypes": [], "subtypes": [], "legalities": {"vintage": "Legal"}
            }]
        }
    }"#;

    const SNAPSHOT_B: &[u8] = br#"{
        "data": {
            "Empty": [{
                "name": "Empty", "layout": "normal", "types": ["Creature"],
                "supertypes": [], "subtypes": [], "legalities": {"vintage": "Legal"}
            }],
            "Banned": [{
                "name": "Banned", "layout": "normal", "types": ["Creature"],
                "supertypes": [], "subtypes": [], "legalities": {"vintage": "Banned"},
                "text": "Banned text."
            }],
            "Alpha": [{
                "name": "Alpha", "layout": "normal", "types": ["Creature"],
                "supertypes": [], "subtypes": [], "legalities": {"vintage": "Legal"},
                "text": "\u2018Alpha\u2019"
            }],
            "Front // Back": [{
                "name": "Front // Back", "faceName": "Back", "side": "b",
                "layout": "modal_dfc", "types": ["Creature"], "supertypes": [],
                "subtypes": [], "legalities": {"vintage": "Banned"},
                "text": "Banned back."
            }, {
                "name": "Front // Back", "faceName": "Front", "side": "a",
                "layout": "modal_dfc", "types": ["Creature"], "supertypes": [],
                "subtypes": [], "legalities": {"vintage": "Legal"},
                "text": "1\u20142 | Choose one."
            }],
            "Restricted": [{
                "name": "Restricted", "layout": "normal", "types": ["Creature"],
                "supertypes": [], "subtypes": [], "legalities": {"vintage": "Restricted"},
                "text": "Restricted text."
            }]
        }
    }"#;

    #[test]
    fn normalization_changes_only_quotes_and_roll_row_range_dashes() {
        let input = "‘Choose’.\n1—9 | Draw a card.\n2-10 | Don’t strip (reminder text.)\nDeal 3-4 damage.\n[-2]: Act.";
        assert_eq!(
            normalize_oracle_text(input),
            "'Choose'.\n1–9 | Draw a card.\n2–10 | Don't strip (reminder text.)\nDeal 3-4 damage.\n[-2]: Act."
        );
    }

    #[test]
    fn normalization_preserves_complete_document_structure() {
        let input = "Choose one —\n• Draw a card.\n• Create a token.\n(Fixed reminder.)";
        assert_eq!(normalize_oracle_text(input), input);
    }

    #[test]
    fn corpus_filters_normalizes_and_sorts_equivalent_snapshots() {
        let left = Corpus::from_bytes(SNAPSHOT_A).unwrap();
        let right = Corpus::from_bytes(SNAPSHOT_B).unwrap();
        assert_eq!(left.units(), right.units());
        assert_eq!(left.units().len(), 3);
        assert_eq!(
            left.units()
                .iter()
                .map(CorpusUnit::context_name)
                .collect::<Vec<_>>(),
            ["Alpha", "Front", "Restricted"]
        );
        assert!(left.units().iter().all(|unit| unit.id().len() == 64));

        let front = &left.units()[1];
        assert_eq!(front.card_name(), "Front // Back");
        assert_eq!(front.face_name(), Some("Front"));
        assert_eq!(front.side.as_deref(), Some("a"));
        assert_eq!(front.text(), "1–2 | Choose one.");
    }

    #[test]
    fn corpus_rejects_context_names_that_cannot_construct_parse_contexts() {
        for (name, expected_reason) in [
            ("", "context name is empty"),
            (", Leading", "card-name abbreviation before comma is empty"),
        ] {
            let snapshot = format!(
                r#"{{"data": {{"Fixture": [{{"name": "{name}", "layout": "normal", "types": ["Creature"], "supertypes": [], "subtypes": [], "legalities": {{"vintage": "Legal"}}, "text": "Fixture text."}}]}}}}"#
            );

            let error = Corpus::from_bytes(snapshot.as_bytes())
                .expect_err("invalid parser context must reject the MTGJSON source")
                .to_string();

            assert!(error.contains("invalid parser context"));
            assert!(error.contains(expected_reason));
            assert!(error.contains(&format!("card name {name:?}")));
        }
    }

    #[test]
    #[should_panic(expected = "invalid parser context")]
    fn test_corpus_constructor_rejects_invalid_contexts() {
        let _ =
            Corpus::from_units_for_test(vec![CorpusUnit::for_test(", Leading", "Fixture text.")]);
    }
}
