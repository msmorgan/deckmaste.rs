//! Models the academyruins.com API.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(transparent)]
pub(crate) struct RulesMap(HashMap<String, Rule>);

impl RulesMap {
    pub fn inner(&self) -> &HashMap<String, Rule> {
        &self.0
    }

    pub fn into_inner(self) -> HashMap<String, Rule> {
        self.0
    }

    pub fn find_rule(&self, rule_number: &str) -> Option<&Rule> {
        self.0.get(rule_number)
    }

    pub fn find_rule_section(&self, rule_number: &str) -> Option<Vec<&Rule>> {
        let mut rule = self.0.get(rule_number)?;
        let mut section = vec![rule];
        while let Some(next) = &rule.navigation.next_rule {
            if !next.starts_with(rule_number) {
                break;
            }
            rule = self.0.get(next)?;
            section.push(rule);
        }
        Some(section)
    }

    pub fn find_keyword_ability_rule_number(&self, keyword_ability: &str) -> Option<&str> {
        let keyword_ability = keyword_ability.to_lowercase();
        for rule in self.find_rule_section("702")? {
            if rule.rule_text.to_lowercase() == keyword_ability {
                return Some(&rule.rule_number);
            }
        }
        None
    }
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
