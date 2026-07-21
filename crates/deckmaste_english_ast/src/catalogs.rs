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
        }
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
    }
}
