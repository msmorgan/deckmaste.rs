//! Deterministic flat projection of Scryfall Oracle Cards JSONL for legacy
//! Oracle corpus consumers.

use std::io::BufRead;
use std::io::Write;

use anyhow::Context;
use deckmaste_catalogs::CatalogSet;
use deckmaste_data::scryfall::OracleCardReader;
use serde::Serialize;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct DerivedCard {
    identity: String,
    oracle_id: String,
    printing_id: String,
    name: String,
    face: Option<String>,
    face_index: Option<usize>,
    side: Option<String>,
    layout: String,
    unit_kind: String,
    mana_cost: Option<String>,
    mana_value: Option<f64>,
    #[serde(rename = "type")]
    type_line: Option<String>,
    types: Vec<String>,
    supertypes: Vec<String>,
    subtypes: Vec<String>,
    text: Option<String>,
    power: Option<String>,
    toughness: Option<String>,
    loyalty: Option<String>,
    defense: Option<String>,
    colors: Option<Vec<String>>,
    color_identity: Option<Vec<String>>,
    keywords: Vec<String>,
    supported: bool,
}

/// Write one deterministic JSON line per Scryfall Oracle unit.
///
/// The source is read one record at a time. Structured type labels are
/// derived against the declared CR catalogs, with longest matches for
/// multiword subtypes.
///
/// # Errors
///
/// Returns an error for invalid Scryfall records, type lines that cannot be
/// projected through the catalogs, or output write failures.
pub fn write(
    oracle_cards: impl BufRead,
    catalogs: &CatalogSet,
    mut output: impl Write,
) -> anyhow::Result<usize> {
    let mut cards = Vec::new();
    for card in OracleCardReader::new(oracle_cards) {
        let card = card.context("parsing Scryfall Oracle Cards JSONL")?;
        let supported = card.vintage_playable();
        for unit in card.oracle_units() {
            let type_line = unit.type_line();
            let parts = type_line
                .map(|type_line| catalogs.parse_type_line(type_line))
                .transpose()?
                .unwrap_or_default();
            let identity = unit.identity();
            cards.push(DerivedCard {
                identity: identity.to_string(),
                oracle_id: identity.oracle_id.to_owned(),
                printing_id: unit.printing_id().to_owned(),
                name: unit.group_name().to_owned(),
                face: unit.face_index().map(|_| unit.display_name().to_owned()),
                face_index: unit.face_index(),
                side: unit.face_index().map(face_side),
                layout: unit.layout().to_owned(),
                unit_kind: match unit.kind() {
                    deckmaste_data::scryfall::OracleUnitKind::CardFace => "card_face",
                    deckmaste_data::scryfall::OracleUnitKind::AlternativeCharacteristics => {
                        "alternative_characteristics"
                    }
                }
                .to_owned(),
                mana_cost: unit.mana_cost().map(str::to_owned),
                mana_value: unit.mana_value(),
                type_line: type_line.map(str::to_owned),
                types: parts.card_types,
                supertypes: parts.supertypes,
                subtypes: parts.subtypes,
                text: unit.oracle_text().map(str::to_owned),
                power: unit.power().map(str::to_owned),
                toughness: unit.toughness().map(str::to_owned),
                loyalty: unit.loyalty().map(str::to_owned),
                defense: unit.defense().map(str::to_owned),
                colors: unit.colors().map(<[String]>::to_vec),
                color_identity: unit.color_identity().map(<[String]>::to_vec),
                keywords: unit.keywords().to_vec(),
                supported,
            });
        }
    }
    cards.sort_by(|left, right| left.identity.cmp(&right.identity));
    for card in &cards {
        serde_json::to_writer(&mut output, card)?;
        output.write_all(b"\n")?;
    }
    output.flush()?;
    Ok(cards.len())
}

fn face_side(index: usize) -> String {
    u8::try_from(index)
        .ok()
        .and_then(|index| b'a'.checked_add(index))
        .filter(u8::is_ascii_lowercase)
        .map_or_else(|| index.to_string(), |side| char::from(side).to_string())
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeMap;
    use std::collections::BTreeSet;

    use super::*;

    fn catalogs() -> CatalogSet {
        use deckmaste_catalogs::CatalogKind::*;
        let mut entries = BTreeMap::new();
        for kind in deckmaste_catalogs::CatalogKind::ALL {
            entries.insert(kind, BTreeSet::new());
        }
        entries
            .get_mut(&CardTypes)
            .unwrap()
            .extend(["Creature", "Land"].into_iter().map(str::to_owned));
        entries
            .get_mut(&CreatureTypes)
            .unwrap()
            .insert("Time Lord".to_owned());
        CatalogSet::from_entries(entries).unwrap()
    }

    #[test]
    fn snapshot_is_sorted_flat_and_preserves_multiword_subtypes() {
        let input = concat!(
            r#"{"object":"card","id":"printing-z","oracle_id":"oracle-z","name":"Zed","layout":"normal","type_line":"Land","colors":[],"color_identity":[],"legalities":{"vintage":"legal"}}"#,
            "\n",
            r#"{"object":"card","id":"printing-a","oracle_id":"oracle-a","name":"Alpha // Omega","layout":"transform","color_identity":["W"],"legalities":{"vintage":"legal"},"card_faces":[{"name":"Alpha","mana_cost":"{W}","type_line":"Creature — Time Lord","oracle_text":"Vigilance","colors":["W"],"power":"1","toughness":"1"},{"name":"Omega","mana_cost":"","type_line":"Creature — Time Lord","oracle_text":"","colors":["W"]}]}"#,
            "\n",
        );
        let mut output = Vec::new();
        assert_eq!(
            write(input.as_bytes(), &catalogs(), &mut output).unwrap(),
            3
        );
        let rows = String::from_utf8(output)
            .unwrap()
            .lines()
            .map(|line| serde_json::from_str::<serde_json::Value>(line).unwrap())
            .collect::<Vec<_>>();
        assert_eq!(rows[0]["identity"], "oracle-a#face:0");
        assert_eq!(rows[0]["subtypes"], serde_json::json!(["Time Lord"]));
        assert_eq!(rows[1]["manaCost"], "");
        assert_eq!(rows[1]["text"], "");
        assert_eq!(rows[2]["identity"], "oracle-z#card");
    }
}
