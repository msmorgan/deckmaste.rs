use std::path::Path;

use crate::data;

pub(super) struct KeywordAbilityTodos;

impl super::Migration for KeywordAbilityTodos {
    fn apply(&self, plugin_dir: &Path) -> anyhow::Result<()> {
        let keyword_abilities = data::keyword_abilities()?;

        Ok(())
    }
}