use std::collections::BTreeMap;
use std::collections::BTreeSet;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;

use deckmaste_data::scryfall::OracleCardReader;
use deckmaste_lexical::Numeral;
use deckmaste_lexical::NumeralCodec;

use crate::LoadError;

pub(crate) fn load(root: &Path) -> Result<BTreeMap<String, String>, LoadError> {
    let path = root.join("data/scryfall/oracle-cards.jsonl");
    let file = File::open(&path).map_err(|source| LoadError::Io {
        operation: "opening",
        path,
        source,
    })?;
    let mut titles = BTreeSet::new();
    let mut units = Vec::new();
    for card in OracleCardReader::new(BufReader::new(file)) {
        let card = card?;
        for unit in card.oracle_units() {
            titles.insert(unit.display_name().to_owned());
            if card.supported() && legendary_permanent(unit.type_line()) {
                units.push((
                    unit.display_name().to_owned(),
                    unit.oracle_text().unwrap_or("").to_owned(),
                ));
            }
        }
    }
    let titles = title_index(&titles);
    Ok(units
        .iter()
        .filter_map(|(name, text)| {
            attested_short_name(name, text, &titles).map(|short| (name.clone(), short.to_owned()))
        })
        .collect())
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
// truncation patterns propose candidates; the unit's Oracle text must attest
// the candidate outside an occurrence of any complete card title.
// The full and shortened spellings remain distinct variants of one Lexeme.
fn attested_short_name<'a>(name: &'a str, text: &str, titles: &TitleIndex<'_>) -> Option<&'a str> {
    let candidate = short_name(name)?;
    let candidate_spans = text
        .match_indices(candidate)
        .filter_map(|(start, _)| {
            let end = start + candidate.len();
            (!text[..start]
                .chars()
                .next_back()
                .is_some_and(name_word_character)
                && !text[end..].chars().next().is_some_and(name_word_character)
                && !text[start..].starts_with(name))
            .then_some(start..end)
        })
        .collect::<Vec<_>>();
    if candidate_spans.is_empty() {
        return None;
    }
    let first_word = name_words(candidate).next()?;
    let title_spans = titles
        .get(first_word)
        .into_iter()
        .flatten()
        .filter(|title| title.len() > candidate.len() && title.contains(candidate))
        .flat_map(|title| {
            text.match_indices(*title)
                .map(|(start, _)| start..start + title.len())
        })
        .filter(|span| {
            !text[..span.start]
                .chars()
                .next_back()
                .is_some_and(name_word_character)
                && !text[span.end..]
                    .chars()
                    .next()
                    .is_some_and(name_word_character)
        })
        .collect::<Vec<_>>();
    candidate_spans
        .iter()
        .any(|candidate_span| {
            !title_spans
                .iter()
                .any(|span| span.start <= candidate_span.start && span.end >= candidate_span.end)
        })
        .then_some(candidate)
}

fn name_word_character(character: char) -> bool {
    character.is_alphanumeric() || matches!(character, '_' | '-')
}

type TitleIndex<'a> = BTreeMap<&'a str, Vec<&'a str>>;

fn name_words(name: &str) -> impl Iterator<Item = &str> {
    name.split(|character: char| !character.is_alphanumeric())
        .filter(|word| !word.is_empty())
}

fn title_index(titles: &BTreeSet<String>) -> TitleIndex<'_> {
    let mut index: TitleIndex<'_> = BTreeMap::new();
    for title in titles {
        for word in name_words(title).collect::<BTreeSet<_>>() {
            index.entry(word).or_default().push(title);
        }
    }
    index
}

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

    #[test]
    fn only_standalone_short_references_in_pinned_oracle_text_become_aliases() {
        let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
        let file = File::open(root.join("data/scryfall/oracle-cards.jsonl")).unwrap();
        let expected = BTreeMap::from([
            ("Captain Sisay", None),
            ("Captain Lannery Storm", None),
            ("Brothers Yamazaki", None),
            ("Gut, True Soul Zealot", None),
            ("Nissa Revane", None),
            ("Dina, Soul Steeper", Some("Dina")),
            ("Uurg, Spawn of Turg", Some("Uurg")),
            ("The Balrog, Durin's Bane", Some("The Balrog")),
            ("Tor Wauki the Younger", Some("Tor Wauki")),
            ("King Darien XLVIII", Some("King Darien")),
        ]);
        let mut titles = BTreeSet::new();
        let mut observed = BTreeMap::new();
        for card in OracleCardReader::new(BufReader::new(file)) {
            let card = card.unwrap();
            for unit in card.oracle_units() {
                let name = unit.display_name();
                titles.insert(name.to_owned());
                if card.supported() && expected.contains_key(name) {
                    let text = unit.oracle_text().unwrap();
                    observed.insert(name.to_owned(), text.to_owned());
                }
            }
        }
        assert_eq!(observed.len(), expected.len());
        let aliases = load(&root).unwrap();
        let titles = title_index(&titles);
        assert!(observed["Nissa Revane"].contains("named Nissa's Chosen"));
        for (name, text) in observed {
            let short = expected[name.as_str()];
            assert_eq!(attested_short_name(&name, &text, &titles), short, "{name}");
            assert_eq!(aliases.get(&name).map(String::as_str), short, "{name}");
        }
    }

    #[test]
    fn embedded_fragments_are_not_short_reference_attestations() {
        // Negative probes exercise token boundaries, not new Oracle examples.
        let titles = BTreeSet::from(["Dina, Soul Steeper".to_owned(), "Nissa's Chosen".to_owned()]);
        let titles = title_index(&titles);
        for text in ["Dinosaur", "xDina", "Dina-like", "Dina, Soul Steeper"] {
            assert_eq!(
                attested_short_name("Dina, Soul Steeper", text, &titles),
                None
            );
        }
        // Nissa Revane refers to this full card title, not to herself as Nissa.
        assert_eq!(
            attested_short_name("Nissa Revane", "Nissa's Chosen", &titles),
            None
        );
    }
}
