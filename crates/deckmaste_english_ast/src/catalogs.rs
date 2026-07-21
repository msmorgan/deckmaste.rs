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

impl Catalogs {
    /// Builds the lexical catalogs from Scryfall catalog values.
    #[must_use]
    pub fn new(
        keyword_abilities: impl IntoIterator<Item = impl Into<String>>,
        keyword_actions: impl IntoIterator<Item = impl Into<String>>,
        ability_words: impl IntoIterator<Item = impl Into<String>>,
    ) -> Self {
        Self {
            keyword_abilities: sorted_longest_first(keyword_abilities),
            keyword_actions: sorted_longest_first(keyword_actions),
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
        let catalog = match kind {
            CatalogKind::KeywordAbility => &mut self.keyword_abilities,
            CatalogKind::KeywordAction => &mut self.keyword_actions,
            CatalogKind::AbilityWord => &mut self.ability_words,
            CatalogKind::ArtifactType => &mut self.artifact_types,
            CatalogKind::BattleType => &mut self.battle_types,
            CatalogKind::CreatureType => &mut self.creature_types,
            CatalogKind::EnchantmentType => &mut self.enchantment_types,
            CatalogKind::LandType => &mut self.land_types,
            CatalogKind::PlaneswalkerType => &mut self.planeswalker_types,
            CatalogKind::SpellType => &mut self.spell_types,
            CatalogKind::Supertype => &mut self.supertypes,
            CatalogKind::CardType => &mut self.card_types,
        };
        *catalog = sorted_longest_first(values);
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

    pub(crate) fn is_ability_word(&self, text: &str) -> bool {
        self.ability_words
            .iter()
            .any(|entry| entry.eq_ignore_ascii_case(text))
    }

    pub(crate) fn exact_term(&self, text: &str) -> Option<(CatalogKind, &str)> {
        exact(&self.keyword_abilities, text)
            .map(|canonical| (CatalogKind::KeywordAbility, canonical))
            .or_else(|| {
                exact(&self.keyword_actions, text)
                    .map(|canonical| (CatalogKind::KeywordAction, canonical))
            })
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
    catalog
        .iter()
        .find(|entry| entry.as_str() == text)
        .map(|canonical| (kind, canonical.as_str()))
}

fn exact_lowercase<'catalog>(
    catalog: &'catalog [String],
    text: &str,
    kind: CatalogKind,
) -> Option<(CatalogKind, &'catalog str)> {
    text.chars()
        .all(|character| !character.is_uppercase())
        .then(|| exact(catalog, text))
        .flatten()
        .map(|canonical| (kind, canonical))
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

fn longest_prefix<'catalog>(catalog: &'catalog [String], text: &str) -> Option<&'catalog str> {
    catalog.iter().map(String::as_str).find(|entry| {
        text.get(..entry.len())
            .is_some_and(|prefix| prefix.eq_ignore_ascii_case(entry))
            && text[entry.len()..]
                .chars()
                .next()
                .is_none_or(|next| !next.is_alphanumeric() && !matches!(next, '-' | '\'' | '’'))
    })
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
            catalogs.exact_term("manifest DREAD"),
            Some((CatalogKind::KeywordAction, "Manifest dread"))
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
        assert_eq!(catalogs.exact_term("LEGENDARY"), None);
        assert_eq!(catalogs.exact_term("Creature"), None);
    }
}
