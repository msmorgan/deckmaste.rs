pub(super) struct KeywordAbilityTodos;

enum KeywordAbility {
    Todo {
        name: String,
        template: String,
        rules: String,
    },
}

impl super::Migration for KeywordAbilityTodos {
    fn apply(&self, plugin: &super::PluginLayout) -> anyhow::Result<()> {
        let keyword_abilities = crate::data::keyword_abilities()?;
        let rules = crate::data::comprehensive_rules()?;

        // let mut file = std::fs::File::create(plugin.keyword_abilities_file()?)?;

        for ability in keyword_abilities {
            let rule_number = rules
                .find_keyword_ability_rule_number(&ability)
                .ok_or_else(|| {
                    anyhow::anyhow!("Keyword ability rule not found for: {}", &ability)
                })?;
            let rules_section = rules.find_rule_section(rule_number).unwrap();
            println!("keyword ability: {ability}");
            println!(
                "{}",
                rules_section
                    .into_iter()
                    .map(|line| format!("> {}", line.rule_text))
                    .collect::<Vec<_>>()
                    .join("\n")
            )
        }

        Ok(())
    }
}
