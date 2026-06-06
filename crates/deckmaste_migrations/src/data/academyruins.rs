use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(transparent)]
pub(crate) struct RulesMap(pub HashMap<String, Rule>);

impl RulesMap {
    pub fn inner(&self) -> &HashMap<String, Rule> { &self.0 }

    pub fn into_inner(self) -> HashMap<String, Rule> { self.0 }

    pub fn find_rule(&self, rule_number: &str) -> Option<&Rule> { self.0.get(rule_number) }

    /// Returns the rule and the subrules that follow it, e.g. "702.9"
    /// yields 702.9, 702.9a, 702.9b, and 702.9c.
    pub fn find_rule_section(&self, rule_number: &str) -> Option<Vec<&Rule>> {
        let mut rule = self.find_rule(rule_number)?;
        let mut section = vec![rule];
        while let Some(next) = &rule.navigation.next_rule {
            if !next.starts_with(rule_number) {
                break;
            }
            if let Some(next_rule) = self.find_rule(next) {
                rule = next_rule;
                section.push(rule);
            } else {
                break;
            }
        }
        Some(section)
    }

    /// Finds the rule within the given section (e.g. "702.") whose text is
    /// exactly the keyword's name (e.g. "Flying" -> "702.9").
    pub fn find_keyword_rule_number(&self, section_prefix: &str, keyword: &str) -> Option<&str> {
        let keyword = keyword.to_lowercase();
        self.0
            .values()
            .find(|rule| {
                rule.rule_number.starts_with(section_prefix)
                    && rule.rule_text.to_lowercase() == keyword
            })
            .map(|rule| rule.rule_number.as_str())
    }
}

/// Replaces typographic quotation marks with their ASCII equivalents.
pub(crate) fn normalize_quotes(text: &str) -> String {
    text.replace(['‘', '’'], "'").replace(['“', '”'], "\"")
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Rule {
    pub rule_number: String,
    pub examples: Option<Vec<String>>,
    pub rule_text: String,
    pub fragment: String,
    pub navigation: Navigation,
}

impl Rule {
    /// Formats the rule like the cr.txt layout: numbered rules ("100.2") get
    /// a trailing dot; lettered subrules ("100.2a") do not. Examples follow
    /// on their own lines. Typographic quotes are normalized to ASCII.
    pub fn format(&self) -> String {
        let separator = if self.rule_number.ends_with(|c: char| c.is_ascii_lowercase()) {
            " "
        } else {
            ". "
        };
        let mut formatted = format!("{}{}{}", self.rule_number, separator, self.rule_text);
        for example in self.examples.iter().flatten() {
            formatted.push_str("\nExample: ");
            formatted.push_str(example);
        }
        normalize_quotes(&formatted)
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Navigation {
    pub next_rule: Option<String>,
    pub previous_rule: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Keywords {
    pub keyword_abilities: Vec<String>,
    pub keyword_actions: Vec<String>,
    pub ability_words: Vec<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn format_normalizes_quotes() {
        let rule = Rule {
            rule_number: "702.9b".to_owned(),
            examples: Some(vec!["The ‘fox’ said “hi.”".to_owned()]),
            rule_text:
                "A creature with flying can’t be blocked. (See rule 509, “Declare Blockers Step.”)"
                    .to_owned(),
            fragment: "9b".to_owned(),
            navigation: Navigation {
                next_rule: None,
                previous_rule: None,
            },
        };
        assert_eq!(
            rule.format(),
            "702.9b A creature with flying can't be blocked. (See rule 509, \"Declare Blockers Step.\")\nExample: The 'fox' said \"hi.\""
        );
    }
}
