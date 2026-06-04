use std::path::Path;

mod _000_keyword_ability_todos;

trait Migration {
    fn apply(&self, plugin_dir: &Path) -> anyhow::Result<()>;
}

const MIGRATIONS: &[&dyn Migration] = &[
    &_000_keyword_ability_todos::KeywordAbilityTodos,
];

pub fn apply(migration_no: usize, plugin_dir: &Path) -> anyhow::Result<()> {
    if migration_no >= MIGRATIONS.len() {
        return Err(anyhow::anyhow!("Migration number out of range"));
    }

    MIGRATIONS[migration_no].apply(plugin_dir)
}

pub fn apply_all(plugin_dir: &Path) -> anyhow::Result<()> {
    for migration in MIGRATIONS {
        migration.apply(plugin_dir)?;
    }
    Ok(())
}