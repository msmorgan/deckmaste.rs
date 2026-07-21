use crate::CatalogKind;

/// The English-language Scryfall catalogs used as lexical knowledge by the
/// parser.
///
/// This type deliberately owns plain strings rather than reading repository
/// data itself. Applications can load the current Scryfall snapshots from
/// whatever data source they use, while this crate remains independent of
/// deckmaste's data layout and update process.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Catalogs {
    keyword_abilities: Vec<String>,
    keyword_actions: Vec<String>,
    keyword_action_forms: Vec<KeywordActionForm>,
    ability_words: Vec<String>,
    artifact_types: Vec<String>,
    battle_types: Vec<String>,
    creature_types: Vec<String>,
    enchantment_types: Vec<String>,
    land_types: Vec<String>,
    planeswalker_types: Vec<String>,
    spell_types: Vec<String>,
    supertypes: Vec<String>,
    card_types: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct KeywordActionForm {
    surface: String,
    canonical: String,
    inflection: KeywordActionInflection,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum KeywordActionInflection {
    ThirdPersonSingular,
    PastParticiple,
}

impl Catalogs {
    /// Builds the lexical catalogs from Scryfall catalog values.
    #[must_use]
    pub fn new(
        keyword_abilities: impl IntoIterator<Item = impl Into<String>>,
        keyword_actions: impl IntoIterator<Item = impl Into<String>>,
        ability_words: impl IntoIterator<Item = impl Into<String>>,
    ) -> Self {
        let keyword_actions = sorted_longest_first(keyword_actions);
        let keyword_action_forms = derived_keyword_action_forms(&keyword_actions);
        Self {
            keyword_abilities: sorted_longest_first(keyword_abilities),
            keyword_actions,
            keyword_action_forms,
            ability_words: sorted_longest_first(ability_words),
            ..Self::default()
        }
    }

    /// Replaces one lexical catalog and returns the updated collection.
    #[must_use]
    pub fn with_catalog(
        mut self,
        kind: CatalogKind,
        values: impl IntoIterator<Item = impl Into<String>>,
    ) -> Self {
        let values = sorted_longest_first(values);
        match kind {
            CatalogKind::KeywordAbility => self.keyword_abilities = values,
            CatalogKind::KeywordAction => {
                self.keyword_action_forms = derived_keyword_action_forms(&values);
                self.keyword_actions = values;
            }
            CatalogKind::AbilityWord => self.ability_words = values,
            CatalogKind::ArtifactType => self.artifact_types = values,
            CatalogKind::BattleType => self.battle_types = values,
            CatalogKind::CreatureType => self.creature_types = values,
            CatalogKind::EnchantmentType => self.enchantment_types = values,
            CatalogKind::LandType => self.land_types = values,
            CatalogKind::PlaneswalkerType => self.planeswalker_types = values,
            CatalogKind::SpellType => self.spell_types = values,
            CatalogKind::Supertype => self.supertypes = values,
            CatalogKind::CardType => self.card_types = values,
        }
        self
    }

    pub(crate) fn keyword_ability_prefix<'catalog>(
        &'catalog self,
        text: &str,
    ) -> Option<&'catalog str> {
        longest_prefix(&self.keyword_abilities, text)
    }

    pub(crate) fn keyword_action_prefix<'catalog>(
        &'catalog self,
        text: &str,
    ) -> Option<&'catalog str> {
        longest_prefix(&self.keyword_actions, text)
    }

    pub(crate) fn inflected_keyword_action_prefix(&self, text: &str) -> Option<(usize, &str)> {
        self.keyword_action_forms
            .iter()
            .filter(|form| form.inflection == KeywordActionInflection::ThirdPersonSingular)
            .find(|form| is_prefix_at_word_boundary(&form.surface, text))
            .map(|form| (form.surface.len(), form.canonical.as_str()))
    }

    pub(crate) fn keyword_action_form(&self, text: &str) -> Option<&str> {
        exact(&self.keyword_actions, text).or_else(|| {
            self.keyword_action_forms
                .iter()
                .find(|form| form.surface.eq_ignore_ascii_case(text))
                .map(|form| form.canonical.as_str())
        })
    }

    pub(crate) fn is_ability_word(&self, text: &str) -> bool {
        self.ability_words
            .iter()
            .any(|entry| entry.eq_ignore_ascii_case(text))
    }

    pub(crate) fn exact_term(&self, text: &str) -> Option<(CatalogKind, &str)> {
        exact(&self.keyword_abilities, text)
            .map(|canonical| (CatalogKind::KeywordAbility, canonical))
            .or_else(|| {
                exact(&self.ability_words, text)
                    .map(|canonical| (CatalogKind::AbilityWord, canonical))
            })
            .or_else(|| exact_subtype(&self.artifact_types, text, CatalogKind::ArtifactType))
            .or_else(|| exact_subtype(&self.battle_types, text, CatalogKind::BattleType))
            .or_else(|| exact_subtype(&self.creature_types, text, CatalogKind::CreatureType))
            .or_else(|| exact_subtype(&self.enchantment_types, text, CatalogKind::EnchantmentType))
            .or_else(|| exact_subtype(&self.land_types, text, CatalogKind::LandType))
            .or_else(|| {
                exact_subtype(
                    &self.planeswalker_types,
                    text,
                    CatalogKind::PlaneswalkerType,
                )
            })
            .or_else(|| exact_subtype(&self.spell_types, text, CatalogKind::SpellType))
            .or_else(|| exact_lowercase(&self.supertypes, text, CatalogKind::Supertype))
            .or_else(|| exact_lowercase(&self.card_types, text, CatalogKind::CardType))
    }
}

fn exact_subtype<'catalog>(
    catalog: &'catalog [String],
    text: &str,
    kind: CatalogKind,
) -> Option<(CatalogKind, &'catalog str)> {
    exact_case_sensitive_or_regular_plural(catalog, text)
        .map(|canonical| (kind, canonical.as_str()))
}

fn exact_lowercase<'catalog>(
    catalog: &'catalog [String],
    text: &str,
    kind: CatalogKind,
) -> Option<(CatalogKind, &'catalog str)> {
    text.chars()
        .all(|character| !character.is_uppercase())
        .then(|| exact_or_regular_plural(catalog, text))
        .flatten()
        .map(|canonical| (kind, canonical))
}

fn exact_case_sensitive_or_regular_plural<'catalog>(
    catalog: &'catalog [String],
    text: &str,
) -> Option<&'catalog String> {
    [Some(text), text.strip_suffix('s'), text.strip_suffix("es")]
        .into_iter()
        .flatten()
        .find_map(|candidate| catalog.iter().find(|entry| entry.as_str() == candidate))
}

fn exact_or_regular_plural<'catalog>(
    catalog: &'catalog [String],
    text: &str,
) -> Option<&'catalog str> {
    [Some(text), text.strip_suffix('s'), text.strip_suffix("es")]
        .into_iter()
        .flatten()
        .find_map(|candidate| exact(catalog, candidate))
}

fn exact<'catalog>(catalog: &'catalog [String], text: &str) -> Option<&'catalog str> {
    catalog
        .iter()
        .find(|entry| entry.eq_ignore_ascii_case(text))
        .map(String::as_str)
}

fn sorted_longest_first(values: impl IntoIterator<Item = impl Into<String>>) -> Vec<String> {
    let mut values: Vec<String> = values.into_iter().map(Into::into).collect();
    values.sort_unstable_by_key(|value| std::cmp::Reverse(value.len()));
    values.dedup_by(|left, right| left.eq_ignore_ascii_case(right));
    values
}

fn derived_keyword_action_forms(actions: &[String]) -> Vec<KeywordActionForm> {
    let mut forms = actions
        .iter()
        .flat_map(|canonical| {
            let (head, tail) = canonical
                .split_once(' ')
                .map_or((canonical.as_str(), ""), |(head, tail)| (head, tail));
            [
                (
                    inflect_third_person_singular(head),
                    KeywordActionInflection::ThirdPersonSingular,
                ),
                (
                    inflect_past_participle(head),
                    KeywordActionInflection::PastParticiple,
                ),
            ]
            .map(|(head, inflection)| KeywordActionForm {
                surface: if tail.is_empty() { head } else { format!("{head} {tail}") },
                canonical: canonical.clone(),
                inflection,
            })
        })
        .collect::<Vec<_>>();
    forms.sort_unstable_by_key(|form| std::cmp::Reverse(form.surface.len()));
    forms
}

fn inflect_third_person_singular(verb: &str) -> String {
    let lower = verb.to_ascii_lowercase();
    if lower.ends_with('y')
        && lower
            .chars()
            .rev()
            .nth(1)
            .is_some_and(|before| !is_vowel(before))
    {
        format!("{}ies", &verb[..verb.len() - 1])
    } else if ["s", "x", "z", "ch", "sh", "o"]
        .iter()
        .any(|ending| lower.ends_with(ending))
    {
        format!("{verb}es")
    } else {
        format!("{verb}s")
    }
}

fn inflect_past_participle(verb: &str) -> String {
    let lower = verb.to_ascii_lowercase();
    match lower.as_str() {
        "behold" => "beheld".to_owned(),
        "cast" | "set" => verb.to_owned(),
        "fight" => "fought".to_owned(),
        "plot" => format!("{verb}ted"),
        "seek" => "sought".to_owned(),
        "tap" => format!("{verb}ped"),
        _ if verb.ends_with('e') => format!("{verb}d"),
        _ if verb.ends_with('y')
            && lower
                .chars()
                .rev()
                .nth(1)
                .is_some_and(|before| !is_vowel(before)) =>
        {
            format!("{}ied", &verb[..verb.len() - 1])
        }
        _ => format!("{verb}ed"),
    }
}

const fn is_vowel(character: char) -> bool {
    matches!(character, 'a' | 'e' | 'i' | 'o' | 'u')
}

fn longest_prefix<'catalog>(catalog: &'catalog [String], text: &str) -> Option<&'catalog str> {
    catalog
        .iter()
        .map(String::as_str)
        .find(|entry| is_prefix_at_word_boundary(entry, text))
}

fn is_prefix_at_word_boundary(prefix: &str, text: &str) -> bool {
    text.get(..prefix.len())
        .is_some_and(|candidate| candidate.eq_ignore_ascii_case(prefix))
        && text[prefix.len()..]
            .chars()
            .next()
            .is_none_or(|next| !next.is_alphanumeric() && !matches!(next, '-' | '\'' | '’'))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn longest_catalog_entry_wins_at_a_word_boundary() {
        let catalogs = Catalogs::new(
            ["Landwalk", "Legendary landwalk"],
            ["Manifest", "Manifest dread"],
            ["Landfall"],
        );

        assert_eq!(
            catalogs.keyword_ability_prefix("Legendary landwalk"),
            Some("Legendary landwalk")
        );
        assert_eq!(
            catalogs.keyword_action_prefix("Manifest dread 2"),
            Some("Manifest dread")
        );
        assert_eq!(catalogs.keyword_action_prefix("Manifestation"), None);
        assert!(catalogs.is_ability_word("LANDFALL"));
        assert_eq!(
            catalogs.keyword_action_form("manifest DREAD"),
            Some("Manifest dread")
        );
    }

    #[test]
    fn specific_subtype_catalogs_take_precedence_over_broad_type_catalogs() {
        let catalogs = Catalogs::default()
            .with_catalog(CatalogKind::ArtifactType, ["Clue"])
            .with_catalog(CatalogKind::CreatureType, ["Hero"])
            .with_catalog(CatalogKind::LandType, ["Forest"])
            .with_catalog(CatalogKind::Supertype, ["Legendary"])
            .with_catalog(CatalogKind::CardType, ["Hero", "Creature"]);

        assert_eq!(
            catalogs.exact_term("Hero"),
            Some((CatalogKind::CreatureType, "Hero"))
        );
        assert_eq!(
            catalogs.exact_term("legendary"),
            Some((CatalogKind::Supertype, "Legendary"))
        );
        assert_eq!(
            catalogs.exact_term("hero"),
            Some((CatalogKind::CardType, "Hero"))
        );
        assert_eq!(
            catalogs.exact_term("creature"),
            Some((CatalogKind::CardType, "Creature"))
        );
        assert_eq!(
            catalogs.exact_term("creatures"),
            Some((CatalogKind::CardType, "Creature"))
        );
        assert_eq!(
            catalogs.exact_term("Heroes"),
            Some((CatalogKind::CreatureType, "Hero"))
        );
        assert_eq!(catalogs.exact_term("LEGENDARY"), None);
        assert_eq!(catalogs.exact_term("Creature"), None);
    }

    #[test]
    fn keyword_action_inflections_are_derived_from_the_catalog() {
        let catalogs = Catalogs::new(
            std::iter::empty::<&str>(),
            [
                "Behold",
                "Collect evidence",
                "Fight",
                "Goad",
                "Manifest dread",
                "Scry",
                "Seek",
                "Tap",
            ],
            std::iter::empty::<&str>(),
        );

        for (surface, canonical) in [
            ("Beheld", "Behold"),
            ("Collected evidence", "Collect evidence"),
            ("Fought", "Fight"),
            ("Goaded", "Goad"),
            ("Manifested dread", "Manifest dread"),
            ("Scries", "Scry"),
            ("Sought", "Seek"),
            ("Tapped", "Tap"),
        ] {
            assert_eq!(catalogs.keyword_action_form(surface), Some(canonical));
        }
        assert_eq!(
            catalogs.inflected_keyword_action_prefix("scries 2"),
            Some(("scries".len(), "Scry"))
        );
    }
}
