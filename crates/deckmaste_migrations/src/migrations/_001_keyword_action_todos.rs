use super::keyword_todos::create_keyword_todos;

pub(super) struct KeywordActionTodos;

impl super::Migration for KeywordActionTodos {
    fn apply(&self, plugin: &super::PluginLayout) -> anyhow::Result<()> {
        let keywords = crate::data::keywords()?;
        let rules = crate::data::comprehensive_rules()?;
        create_keyword_todos(
            &plugin.keyword_actions_dir()?,
            &keywords.keyword_actions,
            &rules,
            |keyword| rules.find_keyword_rule_number("701.", keyword),
            crate::data::academyruins::Rule::format,
        )
    }
}
