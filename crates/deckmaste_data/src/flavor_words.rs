//! Corpus-derived flavor-word inventory.

use std::collections::BTreeSet;

use crate::mtgjson::AtomicCards;

fn chapter_prefix(text: &str) -> bool {
    text.split(", ").all(|part| {
        !part.is_empty()
            && part
                .chars()
                .all(|character| matches!(character, 'I' | 'V' | 'X'))
    })
}

/// Returns the flavor-word surfaces attested in Vintage-playable Oracle text.
///
/// The independent Scryfall flavor-word catalog licenses ordinary ability
/// labels. Mode and Saga chapter labels also admit the measured single-word
/// forms used by those syntactic positions. Declared ability words are
/// excluded because they occupy the same printed slot but belong to a
/// different declaration family.
#[must_use]
pub fn census<'cards>(
    cards: &'cards AtomicCards<'cards>,
    catalog: &BTreeSet<&str>,
    ability_words: &BTreeSet<&str>,
) -> BTreeSet<&'cards str> {
    let mut surfaces = BTreeSet::new();
    for card in cards
        .data
        .values()
        .flatten()
        .filter(|card| card.vintage_playable())
    {
        for line in card.text.as_deref().unwrap_or_default().lines() {
            if let Some(mode) = line.strip_prefix("• ") {
                let Some((label, _body)) = mode.split_once(" — ") else {
                    continue;
                };
                if !label.chars().all(char::is_numeric)
                    && (catalog.contains(label) || !label.contains(char::is_whitespace))
                    && !ability_words.contains(label)
                {
                    surfaces.insert(label);
                }
                continue;
            }

            let Some((head, tail)) = line.split_once(" — ") else {
                continue;
            };
            if chapter_prefix(head) {
                let Some((label, _body)) = tail.split_once(" — ") else {
                    continue;
                };
                if (catalog.contains(label)
                    || (!label.contains(char::is_whitespace) && !label.contains('"')))
                    && !ability_words.contains(label)
                {
                    surfaces.insert(label);
                }
            } else if catalog.contains(head) && !ability_words.contains(head) {
                surfaces.insert(head);
            }
        }
    }
    surfaces
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn census_shares_the_three_position_predicate_and_vintage_filter() {
        let text = r#"{
            "data": {
                "Legal": [{
                    "name": "Legal", "layout": "normal",
                    "types": [], "supertypes": [], "subtypes": [],
                    "legalities": {"vintage": "Legal"},
                    "text": "Known Label — body\nUnknown Label — body\n• Modeword — body\n• Two Words — body\n• 12 — body\nI, II — Chapterword — body\nIII — Catalog Chapter — body\nIV — \"Quoted\" — body\nDomain — body"
                }],
                "Unsupported": [{
                    "name": "Unsupported", "layout": "normal",
                    "types": [], "supertypes": [], "subtypes": [],
                    "legalities": {"vintage": "Not Legal"},
                    "text": "Unsupported Label — body"
                }]
            }
        }"#;
        let cards = AtomicCards::parse(text.as_bytes()).unwrap();
        let catalog = [
            "Known Label",
            "Catalog Chapter",
            "Unsupported Label",
            "Domain",
        ]
        .into_iter()
        .collect();
        let ability_words = ["Domain"].into_iter().collect();

        assert_eq!(
            census(&cards, &catalog, &ability_words),
            ["Catalog Chapter", "Chapterword", "Known Label", "Modeword"]
                .into_iter()
                .collect(),
        );
    }
}
