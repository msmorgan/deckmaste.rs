use std::collections::BTreeMap;
use std::collections::BTreeSet;
use std::fs;
use std::io::BufRead;
use std::path::Path;

use anyhow::Context;
use anyhow::bail;
use deckmaste_data::scryfall::OracleCardReader;

use crate::cr;
use crate::io;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd)]
pub enum LegacyCatalogKind {
    AbilityWords,
    ArtifactTypes,
    BattleTypes,
    CardTypes,
    CreatureTypes,
    EnchantmentTypes,
    KeywordAbilities,
    KeywordActions,
    LandTypes,
    PlaneswalkerTypes,
    SpellTypes,
    Supertypes,
    FlavorWords,
}

impl LegacyCatalogKind {
    pub const GENERATED: [Self; 12] = [
        Self::AbilityWords,
        Self::ArtifactTypes,
        Self::BattleTypes,
        Self::CardTypes,
        Self::CreatureTypes,
        Self::EnchantmentTypes,
        Self::KeywordAbilities,
        Self::KeywordActions,
        Self::LandTypes,
        Self::PlaneswalkerTypes,
        Self::SpellTypes,
        Self::Supertypes,
    ];

    pub const ALL: [Self; 13] = [
        Self::AbilityWords,
        Self::ArtifactTypes,
        Self::BattleTypes,
        Self::CardTypes,
        Self::CreatureTypes,
        Self::EnchantmentTypes,
        Self::KeywordAbilities,
        Self::KeywordActions,
        Self::LandTypes,
        Self::PlaneswalkerTypes,
        Self::SpellTypes,
        Self::Supertypes,
        Self::FlavorWords,
    ];

    #[must_use]
    pub const fn filename(self) -> &'static str {
        match self {
            Self::AbilityWords => "ability-words.txt",
            Self::ArtifactTypes => "artifact-types.txt",
            Self::BattleTypes => "battle-types.txt",
            Self::CardTypes => "card-types.txt",
            Self::CreatureTypes => "creature-types.txt",
            Self::EnchantmentTypes => "enchantment-types.txt",
            Self::KeywordAbilities => "keyword-abilities.txt",
            Self::KeywordActions => "keyword-actions.txt",
            Self::LandTypes => "land-types.txt",
            Self::PlaneswalkerTypes => "planeswalker-types.txt",
            Self::SpellTypes => "spell-types.txt",
            Self::Supertypes => "supertypes.txt",
            Self::FlavorWords => "flavor-words.json",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LegacyCatalogSet {
    catalogs: BTreeMap<LegacyCatalogKind, BTreeSet<String>>,
}

impl LegacyCatalogSet {
    /// Builds the twelve legacy generated catalogs and augments keyword
    /// abilities with CR-authorized variants observed on Vintage-playable
    /// cards.
    ///
    /// # Errors
    ///
    /// Returns an error when the CR or Scryfall Oracle Cards input is malformed
    /// or the complete legacy inventory cannot be built.
    pub fn generate(cr: &str, oracle_cards: impl BufRead) -> anyhow::Result<Self> {
        let extracted = cr::extract_shared(cr)?;
        let mut entries = BTreeMap::from([
            (LegacyCatalogKind::AbilityWords, extracted.ability_words),
            (LegacyCatalogKind::ArtifactTypes, extracted.artifact_types),
            (LegacyCatalogKind::BattleTypes, extracted.battle_types),
            (LegacyCatalogKind::CardTypes, extracted.card_types),
            (LegacyCatalogKind::CreatureTypes, extracted.creature_types),
            (
                LegacyCatalogKind::EnchantmentTypes,
                extracted.enchantment_types,
            ),
            (
                LegacyCatalogKind::KeywordAbilities,
                extracted.keyword_abilities,
            ),
            (LegacyCatalogKind::KeywordActions, extracted.keyword_actions),
            (LegacyCatalogKind::LandTypes, extracted.land_types),
            (
                LegacyCatalogKind::PlaneswalkerTypes,
                extracted.planeswalker_types,
            ),
            (LegacyCatalogKind::SpellTypes, extracted.spell_types),
            (LegacyCatalogKind::Supertypes, extracted.supertypes),
            (LegacyCatalogKind::FlavorWords, BTreeSet::new()),
        ]);

        let authority = normalized_authority(cr);
        let type_names: BTreeSet<String> = LegacyCatalogKind::GENERATED
            .into_iter()
            .filter(|kind| {
                kind.filename().contains("-types") || *kind == LegacyCatalogKind::Supertypes
            })
            .flat_map(|kind| entries[&kind].iter().map(|value| value.to_lowercase()))
            .collect();
        let abilities = entries
            .get_mut(&LegacyCatalogKind::KeywordAbilities)
            .context("legacy catalog set has no keyword-abilities catalog")?;

        for card in OracleCardReader::new(oracle_cards) {
            let card = card.context("parsing Scryfall Oracle Cards JSONL")?;
            if card.vintage_playable() {
                for unit in card.oracle_units() {
                    for keyword in unit.keywords() {
                        let keyword = normalize_variant_name(keyword);
                        if cr_authorizes_variant(&authority, &type_names, &keyword) {
                            abilities.insert(keyword);
                        }
                    }
                }
            }
        }

        Self::from_entries(entries)
    }

    /// Loads the generated legacy catalogs and the isolated Scryfall
    /// flavor-word catalog.
    ///
    /// # Errors
    ///
    /// Returns an error when a required plain-text file or `flavor-words.json`
    /// is missing or malformed.
    pub fn load(
        generated_dir: impl AsRef<Path>,
        scryfall_dir: impl AsRef<Path>,
    ) -> anyhow::Result<Self> {
        let generated_dir = generated_dir.as_ref();
        let mut entries = BTreeMap::new();
        for kind in LegacyCatalogKind::GENERATED {
            entries.insert(
                kind,
                io::read_line_catalog(&generated_dir.join(kind.filename()))?,
            );
        }

        let flavor_path = scryfall_dir
            .as_ref()
            .join(LegacyCatalogKind::FlavorWords.filename());
        let flavor_bytes =
            fs::read(&flavor_path).with_context(|| format!("reading {}", flavor_path.display()))?;
        let flavor_catalog = deckmaste_data::scryfall::Catalog::parse(&flavor_bytes)
            .with_context(|| format!("parsing {}", flavor_path.display()))?;
        entries.insert(
            LegacyCatalogKind::FlavorWords,
            flavor_catalog
                .data
                .into_iter()
                .map(|value| value.as_str().to_owned())
                .collect(),
        );

        Self::from_entries(entries)
    }

    /// Replaces a directory with exactly the twelve generated legacy line
    /// files.
    ///
    /// # Errors
    ///
    /// Returns an error when rendering, staging, or replacing the output fails.
    pub fn write_to(&self, output: impl AsRef<Path>) -> anyhow::Result<()> {
        io::write_line_directory(
            output.as_ref(),
            LegacyCatalogKind::GENERATED
                .into_iter()
                .map(|kind| (kind.filename(), self.get(kind))),
        )
    }

    /// Builds a legacy set only when all generated and loaded kinds are
    /// present.
    ///
    /// # Errors
    ///
    /// Returns an error naming the first missing legacy catalog.
    pub fn from_entries(
        catalogs: BTreeMap<LegacyCatalogKind, BTreeSet<String>>,
    ) -> anyhow::Result<Self> {
        for kind in LegacyCatalogKind::ALL {
            if !catalogs.contains_key(&kind) {
                bail!("missing {} catalog", kind.filename());
            }
        }
        Ok(Self { catalogs })
    }

    #[must_use]
    pub fn get(&self, kind: LegacyCatalogKind) -> &BTreeSet<String> {
        &self.catalogs[&kind]
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

fn cr_authorizes_variant(authority: &str, type_names: &BTreeSet<String>, candidate: &str) -> bool {
    let candidate = candidate.to_lowercase();

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

#[cfg(test)]
mod tests {
    use std::fs;

    use super::LegacyCatalogKind;
    use super::LegacyCatalogSet;

    // cite: noncompliant begin -- verbatim CR parser fixture, not prose claims
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
    // cite: noncompliant end

    const ORACLE_FIXTURE: &str = concat!(
        r#"{"object":"card","id":"printing-legal","oracle_id":"oracle-legal","name":"Legal","layout":"normal","type_line":"Creature","keywords":["Islandwalk","Friends forever","Attack","Wind Walk"],"legalities":{"vintage":"legal"}}"#,
        "\n",
        r#"{"object":"card","id":"printing-restricted","oracle_id":"oracle-restricted","name":"Restricted","layout":"normal","type_line":"Artifact","keywords":["Basic landcycling","Hexproof from","Choose a background"],"legalities":{"vintage":"restricted"}}"#,
        "\n",
        r#"{"object":"card","id":"printing-banned","oracle_id":"oracle-banned","name":"Banned","layout":"normal","type_line":"Creature","keywords":["Forestwalk"],"legalities":{"vintage":"banned"}}"#,
        "\n",
        r#"{"object":"card","id":"printing-missing","oracle_id":"oracle-missing","name":"Missing","layout":"normal","type_line":"Creature","keywords":["Swampwalk"],"legalities":{}}"#,
        "\n",
    );

    #[test]
    fn generation_adds_only_cr_authorized_variants_from_vintage_cards() {
        let catalogs = LegacyCatalogSet::generate(CR_FIXTURE, ORACLE_FIXTURE.as_bytes()).unwrap();
        let abilities = catalogs.get(LegacyCatalogKind::KeywordAbilities);

        for authorized in [
            "Islandwalk",
            "Basic landcycling",
            "Friends forever",
            "Hexproof from",
            "Choose a Background",
        ] {
            assert!(abilities.contains(authorized));
        }
        for rejected in [
            "Choose a background",
            "Attack",
            "Wind Walk",
            "Forestwalk",
            "Swampwalk",
        ] {
            assert!(!abilities.contains(rejected));
        }
    }

    #[test]
    fn output_contains_exactly_the_old_twelve_generated_files() {
        let catalogs = LegacyCatalogSet::generate(CR_FIXTURE, ORACLE_FIXTURE.as_bytes()).unwrap();
        let root = tempfile::tempdir().unwrap();
        let output = root.path().join("catalogs");

        catalogs.write_to(&output).unwrap();

        let mut names = fs::read_dir(&output)
            .unwrap()
            .map(|entry| entry.unwrap().file_name().into_string().unwrap())
            .collect::<Vec<_>>();
        names.sort();
        assert_eq!(
            names,
            [
                "ability-words.txt",
                "artifact-types.txt",
                "battle-types.txt",
                "card-types.txt",
                "creature-types.txt",
                "enchantment-types.txt",
                "keyword-abilities.txt",
                "keyword-actions.txt",
                "land-types.txt",
                "planeswalker-types.txt",
                "spell-types.txt",
                "supertypes.txt",
            ]
        );
        assert!(!output.join("card-names.txt").exists());
        assert!(!output.join("counter-kind-phrases.txt").exists());
    }

    #[test]
    fn load_combines_generated_lines_with_only_the_scryfall_flavor_catalog() {
        let generated = LegacyCatalogSet::generate(CR_FIXTURE, ORACLE_FIXTURE.as_bytes()).unwrap();
        let root = tempfile::tempdir().unwrap();
        let generated_dir = root.path().join("generated");
        let scryfall_dir = root.path().join("scryfall");
        generated.write_to(&generated_dir).unwrap();
        fs::create_dir(&scryfall_dir).unwrap();
        fs::write(
            scryfall_dir.join("flavor-words.json"),
            r#"{"data":["Chaos","Sanctified Rules of Combat"]}"#,
        )
        .unwrap();

        let loaded = LegacyCatalogSet::load(&generated_dir, &scryfall_dir).unwrap();

        for kind in LegacyCatalogKind::GENERATED {
            assert_eq!(loaded.get(kind), generated.get(kind));
        }
        assert_eq!(
            loaded
                .get(LegacyCatalogKind::FlavorWords)
                .iter()
                .map(String::as_str)
                .collect::<Vec<_>>(),
            ["Chaos", "Sanctified Rules of Combat"]
        );
    }
}
