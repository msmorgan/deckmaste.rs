use serde::Serialize;

use crate::data::academyruins;

pub(super) struct KeywordAbilityTodos;

#[derive(Serialize)]
enum KeywordAbility {
    Todo {
        name: String,
        template: String,
        #[serde(serialize_with = "ron::ser::raw_string")]
        rule: String,
    },
}

/// Converts a keyword ability name to a Rust identifier, e.g.
/// "Cumulative upkeep" -> "CumulativeUpkeep", "Jump-start" -> "Jumpstart".
fn to_rust_ident(name: &str) -> String {
    name.split(' ')
        .filter_map(|word| {
            let mut chars = word.chars();
            chars.next().map(|first| {
                first
                    .to_uppercase()
                    .chain(chars)
                    .filter(char::is_ascii_alphanumeric)
            })
        })
        .flatten()
        .collect::<String>()
}

impl super::Migration for KeywordAbilityTodos {
    fn apply(&self, plugin: &super::PluginLayout) -> anyhow::Result<()> {
        let keyword_abilities = crate::data::keywords()?.keyword_abilities;
        let rules = crate::data::comprehensive_rules()?;
        let dest_dir = plugin.keyword_abilities_dir()?;

        // Only (over)write files that are still unimplemented stubs.
        let todo_pattern = regex::Regex::new(r"(?m)^\s*Todo\(")?;

        for ability in keyword_abilities {
            let Some(rule_number) = rules.find_keyword_ability_rule_number(&ability) else {
                eprintln!("no CR 702 rule found for keyword ability {ability:?}; skipping");
                continue;
            };
            let section = rules
                .find_rule_section(rule_number)
                .expect("rule number came from the rules map");

            let name = to_rust_ident(&ability);
            let dest = dest_dir.join(format!("{name}.ron"));
            if dest.exists() && !todo_pattern.is_match(&std::fs::read_to_string(&dest)?) {
                continue;
            }

            let todo = KeywordAbility::Todo {
                name,
                template: ability.clone(),
                rule: format!("\n{}\n", academyruins::format_section(&section)),
            };
            let serialized = ron::ser::to_string_pretty(&todo, ron::ser::PrettyConfig::default())?;
            let contents = format!("// CR {rule_number}\n{serialized}\n");

            std::fs::write(&dest, contents)?;
            eprintln!("wrote {}", dest.display());
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rust_idents() {
        assert_eq!(to_rust_ident("Flying"), "Flying");
        assert_eq!(to_rust_ident("Cumulative upkeep"), "CumulativeUpkeep");
        assert_eq!(to_rust_ident("Jump-start"), "Jumpstart");
        assert_eq!(to_rust_ident("Doctor's companion"), "DoctorsCompanion");
    }

    #[test]
    fn todo_serializes_like_the_fish_template() {
        let todo = KeywordAbility::Todo {
            name: "Flying".to_owned(),
            template: "Flying".to_owned(),
            rule: "\n702.9. Flying\n\n702.9a Flying is an evasion ability.\n".to_owned(),
        };
        let serialized =
            ron::ser::to_string_pretty(&todo, ron::ser::PrettyConfig::default()).unwrap();
        assert_eq!(
            serialized,
            r##"Todo(
    name: "Flying",
    template: "Flying",
    rule: r#"
702.9. Flying

702.9a Flying is an evasion ability.
"#,
)"##
        );
    }
}
