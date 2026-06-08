use super::keyword_todos::create_keyword_todos;
use crate::data::academyruins::{Keywords, Rule, RulesMap};

pub(super) fn generate(plugin: &super::PluginLayout) -> anyhow::Result<()> {
    let keywords_bytes = crate::data::academyruins::keywords_bytes()?;
    let keywords = Keywords::parse(&keywords_bytes)?;
    let rules_bytes = crate::data::academyruins::comprehensive_rules_bytes()?;
    let rules = RulesMap::parse(&rules_bytes)?;
    create_keyword_todos(
        &plugin.keyword_actions_dir()?,
        &keywords.keyword_actions,
        &rules,
        |keyword| rules.find_keyword_rule_number("701.", keyword),
        Rule::format,
    )
}
