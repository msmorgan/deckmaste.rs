use regex::Regex;
use serde::Serialize;
use std::sync::LazyLock;

use crate::data::academyruins;

pub(super) struct KeywordAbilityTodos;

#[derive(Serialize)]
enum KeywordAbility {
    Todo {
        name: String,
        template: String,
        rule: String,
    },
}

/// Converts a keyword ability name to a Rust identifier, e.g.
/// "Cumulative upkeep" -> "CumulativeUpkeep", "Jump-start" -> "JumpStart".
fn to_rust_ident(name: &str) -> String {
    static SPLIT: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"[\s|-]+").unwrap());

    SPLIT
        .split(name)
        .flat_map(|word| {
            let mut chars = word.chars();
            chars
                .next()
                .into_iter()
                .flat_map(char::to_uppercase)
                .chain(chars)
                .filter(char::is_ascii_alphanumeric)
        })
        .collect::<String>()
}

/// Multi-line rule text is written verbatim (a plain string with literal
/// newlines) rather than `\n`-escaped onto one line.
fn pretty_config() -> ron::ser::PrettyConfig {
    ron::ser::PrettyConfig::default().escape_strings(false)
}

impl super::Migration for KeywordAbilityTodos {
    fn apply(&self, plugin: &super::PluginLayout) -> anyhow::Result<()> {
        let keyword_abilities = crate::data::keywords()?.keyword_abilities;
        let rules = crate::data::comprehensive_rules()?;
        let dest_dir = plugin.keyword_abilities_dir()?;

        // Only (over)write files that are still unimplemented stubs.
        let todo_pattern = Regex::new(r"(?m)^\s*Todo\(")?;

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
            let serialized = ron::ser::to_string_pretty(&todo, pretty_config())?;
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
        assert_eq!(to_rust_ident("Jump-start"), "JumpStart");
        assert_eq!(to_rust_ident("Doctor's companion"), "DoctorsCompanion");
    }

    #[test]
    fn todo_serializes_with_verbatim_rule_text() {
        let rule = "\n702.9. Flying\n\n702.9a Flying is an evasion ability.\n";
        let todo = KeywordAbility::Todo {
            name: "Flying".to_owned(),
            template: "Flying".to_owned(),
            rule: rule.to_owned(),
        };
        let serialized = ron::ser::to_string_pretty(&todo, pretty_config()).unwrap();
        assert_eq!(
            serialized,
            r#"Todo(
    name: "Flying",
    template: "Flying",
    rule: "
702.9. Flying

702.9a Flying is an evasion ability.
",
)"#
        );

        // The literal newlines must survive a round trip through the parser.
        #[derive(serde::Deserialize)]
        enum Parsed {
            Todo {
                name: String,
                template: String,
                rule: String,
            },
        }
        let Parsed::Todo {
            name,
            template,
            rule: parsed,
        } = ron::from_str(&serialized).unwrap();
        assert_eq!(name, "Flying");
        assert_eq!(template, "Flying");
        assert_eq!(parsed, rule);
    }
}
