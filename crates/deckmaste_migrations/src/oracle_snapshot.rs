//! Deterministic flat projection of MTGJSON's atomic-card data for Oracle
//! corpus consumers.

use std::io::Write;

use anyhow::Context;
use deckmaste_data::DataStr;
use deckmaste_data::mtgjson::AtomicCard;
use deckmaste_data::mtgjson::AtomicCards;
use serde::Serialize;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct DerivedCard<'a> {
    name: &'a str,
    face: Option<&'a str>,
    side: Option<&'a str>,
    layout: &'a str,
    mana_cost: Option<&'a str>,
    mana_value: Option<f64>,
    #[serde(rename = "type")]
    type_line: Option<&'a str>,
    types: Vec<&'a str>,
    supertypes: Vec<&'a str>,
    subtypes: Vec<&'a str>,
    text: Option<&'a str>,
    power: Option<&'a str>,
    toughness: Option<&'a str>,
    loyalty: Option<&'a str>,
    defense: Option<&'a str>,
    colors: Vec<&'a str>,
    color_identity: Vec<&'a str>,
    supported: bool,
}

impl<'a> From<&'a AtomicCard<'_>> for DerivedCard<'a> {
    fn from(card: &'a AtomicCard<'_>) -> Self {
        Self {
            name: card.name.as_str(),
            face: card.face_name.as_deref(),
            side: card.side.as_deref(),
            layout: card.layout.as_str(),
            mana_cost: card.mana_cost.as_deref(),
            mana_value: card.mana_value,
            type_line: card.type_line.as_deref(),
            types: strings(&card.types),
            supertypes: strings(&card.supertypes),
            subtypes: strings(&card.subtypes),
            text: card.text.as_deref(),
            power: card.power.as_deref(),
            toughness: card.toughness.as_deref(),
            loyalty: card.loyalty.as_deref(),
            defense: card.defense.as_deref(),
            colors: strings(&card.colors),
            color_identity: strings(&card.color_identity),
            supported: card.vintage_playable(),
        }
    }
}

fn strings<'a>(values: &'a [DataStr<'_>]) -> Vec<&'a str> {
    values.iter().map(DataStr::as_str).collect()
}

/// Write one deterministic JSON line per atomic card face.
///
/// # Errors
///
/// Returns an error when the atomic-card input is invalid JSON or when writing
/// the projected rows fails.
pub fn write(atomic: &[u8], mut output: impl Write) -> anyhow::Result<usize> {
    let atomic = AtomicCards::parse(atomic).context("parsing AtomicCards.json")?;
    let mut cards: Vec<&AtomicCard<'_>> = atomic.data.values().flatten().collect();
    cards.sort_by(|left, right| {
        left.name
            .as_str()
            .cmp(right.name.as_str())
            .then_with(|| left.side.as_deref().cmp(&right.side.as_deref()))
            .then_with(|| left.face_name.as_deref().cmp(&right.face_name.as_deref()))
    });

    for card in &cards {
        serde_json::to_writer(&mut output, &DerivedCard::from(*card))?;
        output.write_all(b"\n")?;
    }
    output.flush()?;
    Ok(cards.len())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn snapshot_is_sorted_flat_and_marks_supported_faces() {
        let input = r#"{
            "data": {
                "Zed": [{
                    "name": "Zed", "layout": "reversible_card",
                    "legalities": {"vintage": "Legal"}, "manaValue": 0.0,
                    "type": "Land", "types": ["Land"], "supertypes": [],
                    "subtypes": [], "colors": null, "colorIdentity": []
                }],
                "Alpha // Omega": [
                    {
                        "name": "Alpha // Omega", "faceName": "Alpha", "side": "a",
                        "layout": "transform", "legalities": {"vintage": "Legal"},
                        "manaCost": "{W}", "manaValue": 1.0,
                        "type": "Creature — Human", "types": ["Creature"],
                        "supertypes": [], "subtypes": ["Human"], "text": "Vigilance",
                        "power": "1", "toughness": "1", "colors": ["W"],
                        "colorIdentity": ["W"]
                    },
                    {
                        "name": "Alpha // Omega", "faceName": "Omega", "side": "b",
                        "layout": "transform", "legalities": {"vintage": "Legal"},
                        "manaValue": 1.0, "type": "Creature — Angel",
                        "types": ["Creature"], "supertypes": [],
                        "subtypes": ["Angel"], "colors": ["W"],
                        "colorIdentity": ["W"]
                    }
                ]
            }
        }"#;
        let mut output = Vec::new();
        assert_eq!(write(input.as_bytes(), &mut output).unwrap(), 3);
        let rows: Vec<serde_json::Value> = String::from_utf8(output)
            .unwrap()
            .lines()
            .map(|line| serde_json::from_str(line).unwrap())
            .collect();

        assert_eq!(rows[0]["face"], "Alpha");
        assert_eq!(rows[1]["face"], "Omega");
        assert_eq!(rows[2]["name"], "Zed");
        assert_eq!(rows[0]["manaCost"], "{W}");
        assert_eq!(rows[0]["supported"], true);
        assert_eq!(rows[2]["supported"], true);
        assert_eq!(rows[2]["colors"], serde_json::json!([]));
    }
}
