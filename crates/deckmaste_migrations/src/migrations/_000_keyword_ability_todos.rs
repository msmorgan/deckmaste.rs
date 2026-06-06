use super::keyword_todos::create_keyword_todos;

pub(super) struct KeywordAbilityTodos;

impl super::Migration for KeywordAbilityTodos {
    fn apply(&self, plugin: &super::PluginLayout) -> anyhow::Result<()> {
        let keywords = crate::data::keywords()?;
        let rules = crate::data::comprehensive_rules()?;
        create_keyword_todos(
            &plugin.keyword_abilities_dir()?,
            &keywords.keyword_abilities,
            &rules,
            |keyword| rules.find_keyword_rule_number("702.", keyword),
            crate::data::academyruins::Rule::format,
        )
    }
}
