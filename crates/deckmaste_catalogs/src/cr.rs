use std::collections::BTreeMap;
use std::collections::BTreeSet;

use anyhow::Context;
use anyhow::bail;
use regex::Regex;

use crate::CatalogKind;

pub(crate) struct ExtractedCatalogs {
    pub(crate) ability_words: BTreeSet<String>,
    pub(crate) artifact_types: BTreeSet<String>,
    pub(crate) battle_types: BTreeSet<String>,
    pub(crate) card_types: BTreeSet<String>,
    pub(crate) creature_types: BTreeSet<String>,
    pub(crate) enchantment_types: BTreeSet<String>,
    pub(crate) keyword_abilities: BTreeSet<String>,
    pub(crate) keyword_actions: BTreeSet<String>,
    pub(crate) land_types: BTreeSet<String>,
    pub(crate) planeswalker_types: BTreeSet<String>,
    pub(crate) spell_types: BTreeSet<String>,
    pub(crate) supertypes: BTreeSet<String>,
}

pub(crate) fn extract(cr: &str) -> anyhow::Result<BTreeMap<CatalogKind, BTreeSet<String>>> {
    let extracted = extract_shared(cr)?;
    let lines: Vec<&str> = cr.lines().map(|line| line.trim_end_matches('\r')).collect();
    let mut catalogs = BTreeMap::new();
    catalogs.insert(CatalogKind::AbilityWords, extracted.ability_words);
    catalogs.insert(CatalogKind::ArtifactTypes, extracted.artifact_types);
    catalogs.insert(CatalogKind::BattleTypes, extracted.battle_types);
    catalogs.insert(CatalogKind::CardTypes, extracted.card_types);
    catalogs.insert(CatalogKind::CreatureTypes, extracted.creature_types);
    catalogs.insert(CatalogKind::EnchantmentTypes, extracted.enchantment_types);
    catalogs.insert(CatalogKind::KeywordAbilities, extracted.keyword_abilities);
    catalogs.insert(CatalogKind::KeywordActions, extracted.keyword_actions);
    catalogs.insert(CatalogKind::LandTypes, extracted.land_types);
    catalogs.insert(CatalogKind::PlaneswalkerTypes, extracted.planeswalker_types);
    catalogs.insert(CatalogKind::SpellTypes, extracted.spell_types);
    catalogs.insert(CatalogKind::Supertypes, extracted.supertypes);
    catalogs.insert(
        CatalogKind::CounterKindPhrases,
        parse_counter_kind_phrases(&lines)?,
    );
    Ok(catalogs)
}

pub(crate) fn extract_shared(cr: &str) -> anyhow::Result<ExtractedCatalogs> {
    let lines: Vec<&str> = cr.lines().map(|line| line.trim_end_matches('\r')).collect();
    Ok(ExtractedCatalogs {
        ability_words: parse_list_rule(
            &lines,
            "207.2c",
            "The ability words are ",
            "ability-words",
        )?,
        artifact_types: parse_subtype_rule(&lines, "205.3g", "artifact", "artifact-types")?,
        battle_types: parse_battle_type(&lines)?,
        card_types: parse_list_rule(&lines, "205.2a", "The card types are ", "card-types")?,
        creature_types: parse_creature_types(&lines)?,
        enchantment_types: parse_subtype_rule(
            &lines,
            "205.3h",
            "enchantment",
            "enchantment-types",
        )?,
        keyword_abilities: parse_headings(
            &lines,
            702,
            &["Daybound and Nightbound"],
            "keyword-abilities",
        )?,
        keyword_actions: parse_headings(&lines, 701, &["Tap and Untap"], "keyword-actions")?,
        land_types: parse_subtype_rule(&lines, "205.3i", "land", "land-types")?,
        planeswalker_types: parse_subtype_rule(
            &lines,
            "205.3j",
            "planeswalker",
            "planeswalker-types",
        )?,
        spell_types: parse_subtype_rule(&lines, "205.3k", "spell", "spell-types")?,
        supertypes: parse_list_rule(&lines, "205.4a", "The supertypes are ", "supertypes")?,
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

fn parse_counter_kind_phrases(lines: &[&str]) -> anyhow::Result<BTreeSet<String>> {
    let listed = sentence_after(
        numbered_rule(lines, "122.1b", "counter-kind-phrases")?,
        "The keywords that a keyword counter can be are ",
        "counter-kind-phrases",
    )?;
    let concrete = listed
        .strip_suffix(", as well as any variants of those keywords")
        .context("counter-kind-phrases: expected open-variant suffix")?;
    let mut values = english_list(concrete, "counter-kind-phrases")?;
    values.retain(|value| value.split_whitespace().count() > 1);
    Ok(values)
}
