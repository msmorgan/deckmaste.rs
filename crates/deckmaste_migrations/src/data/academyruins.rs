use std::collections::HashMap;

use serde::Deserialize;

use super::DataStr;

/// Replaces typographic quotation marks with their ASCII equivalents.
pub(crate) fn normalize_quotes(text: &str) -> String {
    text.replace(['‘', '’'], "'").replace(['“', '”'], "\"")
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Rule<'a> {
    #[serde(borrow)]
    pub rule_number: DataStr<'a>,
    #[serde(borrow, default)]
    pub examples: Option<Vec<DataStr<'a>>>,
    #[serde(borrow)]
    pub rule_text: DataStr<'a>,
    #[serde(borrow)]
    pub fragment: DataStr<'a>,
    #[serde(borrow)]
    pub navigation: Navigation<'a>,
}

impl<'a> Rule<'a> {
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

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Navigation<'a> {
    #[serde(borrow)]
    pub next_rule: Option<DataStr<'a>>,
    #[serde(borrow)]
    pub previous_rule: Option<DataStr<'a>>,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Keywords<'a> {
    #[serde(borrow)]
    pub keyword_abilities: Vec<DataStr<'a>>,
    #[serde(borrow)]
    pub keyword_actions: Vec<DataStr<'a>>,
    #[serde(borrow)]
    pub ability_words: Vec<DataStr<'a>>,
}

impl<'a> Keywords<'a> {
    pub fn parse(bytes: &'a [u8]) -> serde_json::Result<Self> { serde_json::from_slice(bytes) }
}

#[derive(Clone, Debug, Deserialize)]
#[serde(transparent)]
pub(crate) struct RulesMap<'a>(#[serde(borrow)] pub HashMap<DataStr<'a>, Rule<'a>>);

impl<'a> RulesMap<'a> {
    pub fn parse(bytes: &'a [u8]) -> serde_json::Result<Self> { serde_json::from_slice(bytes) }

    pub fn into_inner(self) -> HashMap<DataStr<'a>, Rule<'a>> { self.0 }

    pub fn find_rule(&self, rule_number: &str) -> Option<&Rule<'a>> { self.0.get(rule_number) }

    /// Returns the rule and the subrules that follow it, e.g. "702.9"
    /// yields 702.9, 702.9a, 702.9b, and 702.9c.
    pub fn find_rule_section(&self, rule_number: &str) -> Option<Vec<&Rule<'a>>> {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn format_normalizes_quotes() {
        let rule = Rule {
            rule_number: "702.9b".into(),
            examples: Some(vec!["The ‘fox’ said “hi.”".into()]),
            rule_text:
                "A creature with flying can’t be blocked. (See rule 509, “Declare Blockers Step.”)"
                    .into(),
            fragment: "9b".into(),
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
