use std::collections::BTreeMap;
use std::collections::BTreeSet;
use std::path::Path;

use anyhow::Context;
use anyhow::bail;
use deckmaste_data::mtgjson::AtomicCards;
use regex::Regex;

pub const CATALOG_NAMES: [&str; 12] = [
    "ability-words",
    "artifact-types",
    "battle-types",
    "card-types",
    "creature-types",
    "enchantment-types",
    "keyword-abilities",
    "keyword-actions",
    "land-types",
    "planeswalker-types",
    "spell-types",
    "supertypes",
];

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CatalogSet {
    catalogs: BTreeMap<&'static str, BTreeSet<String>>,
}

impl CatalogSet {
    /// Extract every catalog consumed by deckmaste from the Comprehensive
    /// Rules.
    ///
    /// # Errors
    ///
    /// Returns an error when an authoritative rule or heading cannot be found
    /// in its expected shape.
    pub fn from_cr(cr: &str) -> anyhow::Result<Self> {
        let lines: Vec<&str> = cr.lines().map(|line| line.trim_end_matches('\r')).collect();
        let mut catalogs = BTreeMap::new();
        catalogs.insert(
            "card-types",
            parse_list_rule(&lines, "205.2a", "The card types are ", "card-types")?,
        );
        catalogs.insert(
            "supertypes",
            parse_list_rule(&lines, "205.4a", "The supertypes are ", "supertypes")?,
        );
        catalogs.insert(
            "artifact-types",
            parse_subtype_rule(&lines, "205.3g", "artifact", "artifact-types")?,
        );
        catalogs.insert(
            "enchantment-types",
            parse_subtype_rule(&lines, "205.3h", "enchantment", "enchantment-types")?,
        );
        catalogs.insert(
            "land-types",
            parse_subtype_rule(&lines, "205.3i", "land", "land-types")?,
        );
        catalogs.insert(
            "planeswalker-types",
            parse_subtype_rule(&lines, "205.3j", "planeswalker", "planeswalker-types")?,
        );
        catalogs.insert(
            "spell-types",
            parse_subtype_rule(&lines, "205.3k", "spell", "spell-types")?,
        );
        catalogs.insert("creature-types", parse_creature_types(&lines)?);
        catalogs.insert("battle-types", parse_battle_type(&lines)?);
        catalogs.insert(
            "ability-words",
            parse_list_rule(&lines, "207.2c", "The ability words are ", "ability-words")?,
        );
        catalogs.insert(
            "keyword-actions",
            parse_headings(&lines, 701, &["Tap and Untap"], "keyword-actions")?,
        );
        catalogs.insert(
            "keyword-abilities",
            parse_headings(
                &lines,
                702,
                &["Daybound and Nightbound"],
                "keyword-abilities",
            )?,
        );
        if catalogs.len() != CATALOG_NAMES.len() {
            bail!(
                "expected {} catalogs, built {}",
                CATALOG_NAMES.len(),
                catalogs.len()
            );
        }
        Ok(Self { catalogs })
    }

    pub fn names(&self) -> impl Iterator<Item = &'static str> + '_ {
        self.catalogs.keys().copied()
    }

    pub fn get(&self, name: &str) -> Vec<&str> {
        self.catalogs[name].iter().map(String::as_str).collect()
    }

    /// Add only concrete keyword-ability variants observed on Vintage cards
    /// and independently authorized by the Comprehensive Rules.
    ///
    /// # Errors
    ///
    /// Returns an error when the `AtomicCards` JSON cannot be parsed or this
    /// set does not contain its required keyword-ability catalog.
    pub fn add_atomic_variants(&mut self, atomic: &[u8], cr: &str) -> anyhow::Result<()> {
        let atomic = AtomicCards::parse(atomic).context("parsing AtomicCards.json")?;
        let authority = normalized_authority(cr);
        let type_names: BTreeSet<String> = self
            .catalogs
            .iter()
            .filter(|(name, _)| name.ends_with("-types") || **name == "supertypes")
            .flat_map(|(_, values)| values.iter().map(|value| value.to_lowercase()))
            .collect();
        let abilities = self
            .catalogs
            .get_mut("keyword-abilities")
            .context("catalog set has no keyword-abilities catalog")?;

        for card in atomic.data.values().flatten().filter(|card| {
            matches!(
                card.legalities.vintage.as_deref(),
                Some("Legal" | "Restricted")
            )
        }) {
            for keyword in &card.keywords {
                let keyword = normalize_variant_name(keyword.as_str());
                if cr_authorizes_variant(&authority, &type_names, &keyword) {
                    abilities.insert(keyword);
                }
            }
        }
        Ok(())
    }

    /// Write one sorted, deduplicated, newline-terminated bare text file per
    /// catalog. The output directory contains exactly the catalogs we use.
    ///
    /// # Errors
    ///
    /// Returns an error when a required catalog is absent or an output
    /// directory/file cannot be created or written.
    pub fn write_to(&self, output: &Path) -> anyhow::Result<()> {
        std::fs::create_dir_all(output)
            .with_context(|| format!("creating {}", output.display()))?;
        for name in CATALOG_NAMES {
            let values = self
                .catalogs
                .get(name)
                .with_context(|| format!("missing {name} catalog"))?;
            let mut contents = values.iter().cloned().collect::<Vec<_>>().join("\n");
            contents.push('\n');
            let path = output.join(format!("{name}.txt"));
            std::fs::write(&path, contents)
                .with_context(|| format!("writing {}", path.display()))?;
        }
        Ok(())
    }
}

fn normalized_authority(cr: &str) -> String {
    cr.replace(['‘', '’'], "'")
        .replace(['“', '”'], "\"")
        .to_lowercase()
}

fn normalize_variant_name(name: &str) -> String {
    match normalize_name(name).as_str() {
        "Choose a background" => "Choose a Background".to_owned(),
        name => name.to_owned(),
    }
}

fn cr_authorizes_variant(authority: &str, type_names: &BTreeSet<String>, candidate: &str) -> bool {
    let candidate = candidate.to_lowercase();

    // The CR defines these as parameterized families rather than giving every
    // concrete spelling its own rule heading.
    if candidate != "landwalk"
        && authority.contains("[type]walk")
        && let Some(prefix) = candidate.strip_suffix("walk")
        && (is_type_phrase(prefix.trim(), type_names)
            || authority.contains(&format!("\"{candidate}\"")))
    {
        return true;
    }
    if candidate != "cycling"
        && authority.contains("[type]cycling")
        && let Some(prefix) = candidate.strip_suffix("cycling")
        && (is_type_phrase(prefix.trim(), type_names)
            || authority.contains(&format!("\"{candidate}\"")))
    {
        return true;
    }
    if candidate == "hexproof from" && authority.contains("hexproof from [quality]") {
        return true;
    }

    // Named variants are either stated as variants or enumerated by a family
    // rule (currently the partner family).
    authority.contains(&format!("{candidate} is a variant of"))
        || authority.contains(&format!("{candidate}\" is a variant of"))
        || matches!(
            candidate.as_str(),
            "partner with" | "choose a background" | "doctor's companion"
        ) && authority.contains(&candidate)
        || authority.contains(&format!("partner—{candidate}\""))
}

fn is_type_phrase(phrase: &str, type_names: &BTreeSet<String>) -> bool {
    !phrase.is_empty()
        && type_names.iter().any(|name| {
            phrase == name
                || phrase
                    .strip_prefix(name)
                    .and_then(|tail| tail.strip_prefix(' '))
                    .is_some_and(|tail| is_type_phrase(tail, type_names))
        })
}

fn numbered_rule<'a>(lines: &'a [&str], number: &str, catalog: &str) -> anyhow::Result<&'a str> {
    let prefix = format!("{number} ");
    let matches: Vec<&str> = lines
        .iter()
        .filter_map(|line| line.strip_prefix(&prefix))
        .collect();
    match matches.as_slice() {
        [text] => Ok(*text),
        _ => bail!(
            "{catalog}: expected one {number} body rule, found {}",
            matches.len()
        ),
    }
}

fn sentence_after<'a>(text: &'a str, lead: &str, catalog: &str) -> anyhow::Result<&'a str> {
    let (_, tail) = text
        .split_once(lead)
        .with_context(|| format!("{catalog}: expected {lead:?}"))?;
    let mut parenthesis_depth = 0_u32;
    for (index, character) in tail.char_indices() {
        match character {
            '(' => parenthesis_depth += 1,
            ')' => parenthesis_depth = parenthesis_depth.saturating_sub(1),
            '.' if parenthesis_depth == 0 => return Ok(&tail[..index]),
            _ => {}
        }
    }
    bail!("{catalog}: expected list sentence to end with a period")
}

fn normalize_name(name: &str) -> String {
    let name = name
        .trim()
        .replace(['‘', '’'], "'")
        .replace(['“', '”'], "\"");
    let mut characters = name.chars();
    characters.next().map_or_else(String::new, |first| {
        first.to_uppercase().chain(characters).collect()
    })
}

fn english_list(source: &str, catalog: &str) -> anyhow::Result<BTreeSet<String>> {
    let parenthetical = Regex::new(r"\s*\([^)]*\)").expect("fixed parenthetical regex");
    let source = parenthetical.replace_all(source, "");
    let source = source.replace(", and ", ", ");
    let values: BTreeSet<String> = source
        .split(',')
        .map(normalize_name)
        .filter(|value| !value.is_empty())
        .collect();
    if values.is_empty() {
        bail!("{catalog}: empty CR list");
    }
    Ok(values)
}

fn parse_list_rule(
    lines: &[&str],
    number: &str,
    lead: &str,
    catalog: &str,
) -> anyhow::Result<BTreeSet<String>> {
    let text = numbered_rule(lines, number, catalog)?;
    english_list(sentence_after(text, lead, catalog)?, catalog)
}

fn parse_subtype_rule(
    lines: &[&str],
    number: &str,
    category: &str,
    catalog: &str,
) -> anyhow::Result<BTreeSet<String>> {
    parse_list_rule(
        lines,
        number,
        &format!("The {category} types are "),
        catalog,
    )
}

fn parse_creature_types(lines: &[&str]) -> anyhow::Result<BTreeSet<String>> {
    const CATALOG: &str = "creature-types";
    let text = numbered_rule(lines, "205.3m", CATALOG)?;
    let mut values = english_list(
        sentence_after(text, "One creature type is two words long: ", CATALOG)?,
        CATALOG,
    )?;
    values.extend(english_list(
        sentence_after(
            text,
            "All other creature types are one word long: ",
            CATALOG,
        )?,
        CATALOG,
    )?);
    Ok(values)
}

fn parse_battle_type(lines: &[&str]) -> anyhow::Result<BTreeSet<String>> {
    const CATALOG: &str = "battle-types";
    let text = numbered_rule(lines, "205.3q", CATALOG)?;
    english_list(
        sentence_after(text, "That battle type is ", CATALOG)?,
        CATALOG,
    )
}

fn parse_headings(
    lines: &[&str],
    section: u16,
    compound_names: &[&str],
    catalog: &str,
) -> anyhow::Result<BTreeSet<String>> {
    let heading = Regex::new(&format!(r"^{section}\.([0-9]+)\. (.+)$"))
        .expect("section number makes a valid heading regex");
    let mut values = BTreeSet::new();
    for captures in lines.iter().filter_map(|line| heading.captures(line)) {
        if &captures[1] == "1" {
            continue;
        }
        let name = &captures[2];
        if compound_names.contains(&name) {
            values.extend(name.split(" and ").map(normalize_name));
        } else if name == "∞ (Infinity)" {
            values.insert("∞".to_owned());
        } else {
            values.insert(normalize_name(name));
        }
    }
    if values.is_empty() {
        bail!("{catalog}: found no {section} keyword headings");
    }
    Ok(values)
}

#[cfg(test)]
mod tests {
    use super::*;

    const CR_FIXTURE: &str = "\
205.2a The card types are artifact, creature, land, and sorcery.\n\
205.3g Artifacts have their own unique set of subtypes; these subtypes are called artifact types. The artifact types are Clue (see rule 111), and Vibranium.\n\
205.3h Enchantments have their own unique set of subtypes; these subtypes are called enchantment types. The enchantment types are Aura, and Saga.\n\
205.3i Lands have their own unique set of subtypes; these subtypes are called land types. The land types are Forest, Island, and Urza’s. Of that list, Forest is a basic land type.\n\
205.3j Planeswalkers have their own unique set of subtypes; these subtypes are called planeswalker types. The planeswalker types are Ajani, and Jace.\n\
205.3k Instants and sorceries share their lists of subtypes; these subtypes are called spell types. The spell types are Arcane, and Trap.\n\
205.3m Creatures and kindreds share their lists of subtypes; these subtypes are called creature types. One creature type is two words long: Time Lord. All other creature types are one word long: Advisor.\n\
205.3q Battles have a unique subtype, called a battle type. That battle type is Siege.\n\
205.4a An object can have one or more supertypes. A card’s supertypes are printed directly before its card types. The supertypes are basic, legendary, and snow.\n\
207.2c An ability word appears in italics at the beginning of some abilities. The ability words are landfall, and threshold.\n\
701.1. Keyword action introduction\n\
701.2. Scry\n\
701.3. Tap and Untap\n\
702.1. Keyword ability introduction\n\
702.2. Daybound and Nightbound\n\
702.3. ∞ (Infinity)\n\
702.4a Landwalk is a generic term that appears within an object’s rules text as “[type]walk,” where [type] is usually a subtype.\n\
702.5a Typecycling is a variant of the cycling ability. “[Type]cycling [cost]” means “Pay [cost], Discard this card: Search your library for a [type] card, reveal it, put it into your hand, then shuffle.”\n\
702.6a Partner is a keyword ability that has several variants: partner with [name], choose a Background, and Doctor’s companion. “Partner—Friends forever” represents one such variant.\n\
702.7a Hexproof from is a variant of the hexproof ability.\n";

    const ATOMIC_FIXTURE: &str = r#"{
        "data": {
            "Legal": [{
                "name": "Legal", "layout": "normal",
                "types": ["Creature"], "supertypes": [], "subtypes": [],
                "keywords": ["Islandwalk", "Friends forever", "Attack", "Wind Walk"],
                "legalities": {"vintage": "Legal"}
            }],
            "Restricted": [{
                "name": "Restricted", "layout": "normal",
                "types": ["Artifact"], "supertypes": [], "subtypes": [],
                "keywords": ["Basic landcycling", "Hexproof from", "Choose a background"],
                "legalities": {"vintage": "Restricted"}
            }],
            "Banned": [{
                "name": "Banned", "layout": "normal",
                "types": ["Creature"], "supertypes": [], "subtypes": [],
                "keywords": ["Forestwalk"],
                "legalities": {"vintage": "Banned"}
            }],
            "Missing": [{
                "name": "Missing", "layout": "normal",
                "types": ["Creature"], "supertypes": [], "subtypes": [],
                "keywords": ["Swampwalk"],
                "legalities": {}
            }]
        }
    }"#;

    #[test]
    fn cr_builds_all_twelve_catalogs() {
        let catalogs = CatalogSet::from_cr(CR_FIXTURE).unwrap();

        assert_eq!(catalogs.names().collect::<Vec<_>>(), CATALOG_NAMES);
        assert_eq!(catalogs.get("artifact-types"), ["Clue", "Vibranium"]);
        assert_eq!(catalogs.get("creature-types"), ["Advisor", "Time Lord"]);
        assert_eq!(catalogs.get("keyword-actions"), ["Scry", "Tap", "Untap"]);
        assert_eq!(
            catalogs.get("keyword-abilities"),
            ["Daybound", "Nightbound", "∞"]
        );
        assert_eq!(catalogs.get("land-types"), ["Forest", "Island", "Urza's"]);
    }

    #[test]
    fn changed_cr_list_shape_is_an_error() {
        let error =
            CatalogSet::from_cr(&CR_FIXTURE.replace("The card types are", "Card types include"))
                .unwrap_err();

        assert!(error.to_string().contains("card-types"));
    }

    #[test]
    fn atomic_adds_only_cr_authorized_variants_from_vintage_cards() {
        let mut catalogs = CatalogSet::from_cr(CR_FIXTURE).unwrap();
        catalogs
            .add_atomic_variants(ATOMIC_FIXTURE.as_bytes(), CR_FIXTURE)
            .unwrap();

        let abilities = catalogs.get("keyword-abilities");
        assert!(abilities.contains(&"Islandwalk"));
        assert!(abilities.contains(&"Basic landcycling"));
        assert!(abilities.contains(&"Friends forever"));
        assert!(abilities.contains(&"Hexproof from"));
        assert!(abilities.contains(&"Choose a Background"));
        assert!(!abilities.contains(&"Choose a background"));
        assert!(!abilities.contains(&"Attack"));
        assert!(!abilities.contains(&"Wind Walk"));
        assert!(!abilities.contains(&"Forestwalk"));
        assert!(!abilities.contains(&"Swampwalk"));
    }

    #[test]
    fn writes_exactly_twelve_sorted_bare_text_catalogs() {
        let catalogs = CatalogSet::from_cr(CR_FIXTURE).unwrap();
        let output = tempfile::tempdir().unwrap();

        catalogs.write_to(output.path()).unwrap();

        let mut names: Vec<String> = std::fs::read_dir(output.path())
            .unwrap()
            .map(|entry| entry.unwrap().file_name().into_string().unwrap())
            .collect();
        names.sort();
        assert_eq!(
            names,
            CATALOG_NAMES.map(|name| format!("{name}.txt")).to_vec()
        );
        assert_eq!(
            std::fs::read_to_string(output.path().join("battle-types.txt")).unwrap(),
            "Siege\n"
        );
    }
}
