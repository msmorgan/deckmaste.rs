use std::path::Path;

use serde::Serialize;

use crate::data::academyruins::{self, RulesMap};

#[derive(Serialize)]
enum KeywordTodo {
    Todo {
        name: String,
        template: String,
        rule: String,
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
            rule: format!("\n{}\n", academyruins::format_section(&section)),
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
    fn todo_serializes_with_verbatim_rule_text() {
        let rule = "\n702.9. Flying\n\n702.9a Flying is an evasion ability.\n";
        let todo = KeywordTodo::Todo {
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
