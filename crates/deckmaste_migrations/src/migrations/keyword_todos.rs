use std::path::Path;

use serde::Serialize;

use crate::data::academyruins::RulesMap;

#[derive(Serialize)]
enum KeywordTodo {
    Todo {
        name: String,
        template: String,
        rules: Vec<String>,
    },
}

/// Multi-line rule text is written verbatim (a plain string with literal
/// newlines) rather than `\n`-escaped onto one line.
fn pretty_config() -> ron::ser::PrettyConfig {
    ron::ser::PrettyConfig::default().escape_strings(false)
}

/// Writes a `<RustIdent>.ron` Todo stub into `dest_dir` for every keyword,
/// with its CR rule section inlined. `rule_number_for` resolves a keyword to
/// the rule number its section starts at; keywords it cannot resolve are
/// skipped with a warning.
pub(super) fn create_keyword_todos<'r>(
    dest_dir: &Path,
    keywords: &[String],
    rules: &'r RulesMap,
    rule_number_for: impl Fn(&str) -> Option<&'r str>,
) -> anyhow::Result<()> {
    for keyword in keywords {
        let Some(rule_number) = rule_number_for(keyword) else {
            eprintln!("no CR rule found for keyword {keyword:?}; skipping");
            continue;
        };
        let section = rules
            .find_rule_section(rule_number)
            .ok_or_else(|| anyhow::anyhow!("rule {rule_number} not in the rules map"))?;

        let name = super::to_rust_ident(keyword);
        let dest = dest_dir.join(format!("{name}.ron"));
        if !super::is_todo(&dest)? {
            continue;
        }

        let todo = KeywordTodo::Todo {
            name,
            template: keyword.clone(),
            rules: section.iter().map(|rule| rule.format()).collect(),
        };
        let serialized = ron::ser::to_string_pretty(&todo, pretty_config())?;
        let contents = format!("// CR {rule_number}\n{serialized}\n");

        std::fs::write(&dest, contents)?;
        eprintln!("wrote {}", dest.display());
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn todo_serializes_with_one_rule_per_element() {
        let rules = vec![
            "702.9. Flying".to_owned(),
            "702.9b A creature with flying can't be blocked. (See \"Reach.\")".to_owned(),
            "104.3a A player can concede.\nExample: Someone scooped.".to_owned(),
        ];
        let todo = KeywordTodo::Todo {
            name: "Flying".to_owned(),
            template: "Flying".to_owned(),
            rules: rules.clone(),
        };
        let serialized = ron::ser::to_string_pretty(&todo, pretty_config()).unwrap();
        // Quote-bearing rules fall back to raw strings; embedded Example
        // lines keep their literal newlines.
        assert_eq!(
            serialized,
            r##"Todo(
    name: "Flying",
    template: "Flying",
    rules: [
        "702.9. Flying",
        r#"702.9b A creature with flying can't be blocked. (See "Reach.")"#,
        "104.3a A player can concede.
Example: Someone scooped.",
    ],
)"##
        );

        // Everything must survive a round trip through the parser.
        #[derive(serde::Deserialize)]
        enum Parsed {
            Todo {
                name: String,
                template: String,
                rules: Vec<String>,
            },
        }
        let Parsed::Todo {
            name,
            template,
            rules: parsed,
        } = ron::from_str(&serialized).unwrap();
        assert_eq!(name, "Flying");
        assert_eq!(template, "Flying");
        assert_eq!(parsed, rules);
    }
}
