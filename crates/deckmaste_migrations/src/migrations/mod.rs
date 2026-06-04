use std::path::Path;
use crate::layout::PluginLayout;

mod _000_keyword_ability_todos;

trait Migration {
    fn apply(&self, plugin: &PluginLayout) -> anyhow::Result<()>;
}

const MIGRATIONS: &[&dyn Migration] = &[
    &_000_keyword_ability_todos::KeywordAbilityTodos,
];

pub fn apply_all(plugin_dir: &Path) -> anyhow::Result<()> {
    let plugin_layout = PluginLayout::new(plugin_dir)?;
    for migration in MIGRATIONS {
        migration.apply(&plugin_layout)?;
    }
    Ok(())
}

pub fn apply(plugin_dir: &Path, migration_number: usize) -> anyhow::Result<()> {
    if migration_number >= MIGRATIONS.len() {
        return Err(anyhow::anyhow!("Migration number out of range"));
    }

    MIGRATIONS[migration_number].apply(&PluginLayout::new(plugin_dir)?)
}