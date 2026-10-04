use std::collections::BTreeMap;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;

use deckmaste_data::scryfall::OracleCardReader;
use deckmaste_lexical::{Numeral, NumeralCodec};

use crate::LoadError;

pub(crate) fn load(root: &Path) -> Result<BTreeMap<String, String>, LoadError> {
    let path = root.join("data/scryfall/oracle-cards.jsonl");
    let file = File::open(&path).map_err(|source| LoadError::Io {
        operation: "opening",
        path,
        source,
    })?;
    let mut names = BTreeMap::new();
    for card in OracleCardReader::new(BufReader::new(file)) {
        let card = card?;
        if !card.supported() {
            continue;
        }
        for unit in card.oracle_units() {
            if legendary_permanent(unit.type_line()) {
                let name = unit.display_name();
                if let Some(short) = short_name(name) {
                    names.insert(name.to_owned(), short.to_owned());
                }
            }
        }
    }
    Ok(names)
}

fn legendary_permanent(type_line: Option<&str>) -> bool {
    let head = type_line.unwrap_or("").split('—').next().unwrap_or("");
    let words = head.split_whitespace().collect::<Vec<_>>();
    words.contains(&"Legendary")
        && words.iter().any(|word| {
            matches!(
                *word,
                "Artifact" | "Battle" | "Creature" | "Enchantment" | "Land" | "Planeswalker"
            )
        })
}

// Shortened self-reference has the full name's referent [CR#201.5c]. The
// truncation patterns are Oracle editorial conventions, not grammatical rules.
// The full and shortened spellings remain distinct variants of one Lexeme.
fn short_name(name: &str) -> Option<&str> {
    let candidate = if let Some((prefix, _)) = name.split_once(',') {
        prefix.trim()
    } else if name.starts_with("The ") {
        return None;
    } else if let Some((prefix, last)) = name.rsplit_once(' ')
        && Numeral::Roman.parse(last).is_ok()
    {
        prefix.trim_end()
    } else if let Some(index) = [" the ", " of "]
        .into_iter()
        .filter_map(|separator| name.find(separator))
        .min()
    {
        &name[..index]
    } else {
        name.split(' ').next().unwrap_or(name)
    };
    (candidate != name && !candidate.is_empty()).then_some(candidate)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn oracle_short_names_preserve_multiword_prefixes_and_use_the_numeral_codec() {
        for (name, expected) in [
            ("Gut, True Soul Zealot", Some("Gut")),
            ("The Balrog, Durin's Bane", Some("The Balrog")),
            ("The First Sliver", None),
            ("King Darien XLVIII", Some("King Darien")),
            ("Grand Arbiter Augustin IV", Some("Grand Arbiter Augustin")),
            ("Tor Wauki the Younger", Some("Tor Wauki")),
            ("Sidar Jabari of Zhalfir", Some("Sidar Jabari")),
            ("Nissa Revane", Some("Nissa")),
            ("Progenitus", None),
        ] {
            assert_eq!(short_name(name), expected, "{name}");
        }
        assert!(legendary_permanent(Some(
            "Legendary Artifact Creature — Golem"
        )));
        assert!(!legendary_permanent(Some("Legendary Sorcery")));
        assert!(!legendary_permanent(Some("Creature — Legendary")));
    }
}
