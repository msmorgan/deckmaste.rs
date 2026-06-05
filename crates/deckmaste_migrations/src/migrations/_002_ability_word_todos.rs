use super::keyword_todos::create_keyword_todos;

/// The rule that introduces ability words and lists them all; ability words
/// have no individual entries in the CR.
const ABILITY_WORD_RULE: &str = "207.2c";

pub(super) struct AbilityWordTodos;

impl super::Migration for AbilityWordTodos {
    fn apply(&self, plugin: &super::PluginLayout) -> anyhow::Result<()> {
        let keywords = crate::data::keywords()?;
        let rules = crate::data::comprehensive_rules()?;
        create_keyword_todos(
            &plugin.ability_words_dir()?,
            &keywords.ability_words,
            &rules,
            |_| Some(ABILITY_WORD_RULE),
        )
    }
}
