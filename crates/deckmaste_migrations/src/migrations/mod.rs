use std::path::Path;
use std::sync::LazyLock;

use regex::Regex;

use crate::layout::PluginLayout;

mod _000_keyword_ability_todos;
mod _001_keyword_action_todos;
mod _002_ability_word_todos;
mod _003_subtypes;
pub(crate) mod keyword_ability;
mod keyword_todos;
pub(crate) mod mana_ability;

trait Migration {
    fn apply(&self, plugin: &PluginLayout) -> anyhow::Result<()>;
}

/// Whether a definition still needs generating, keyed on its finished `.ron`
/// path: a stub writer (re)generates its stub only while the finished `.ron`
/// doesn't exist.
fn is_unimplemented(final_path: &Path) -> bool { !final_path.exists() }

const MIGRATIONS: &[&dyn Migration] = &[
    &_000_keyword_ability_todos::KeywordAbilityTodos,
    &_001_keyword_action_todos::KeywordActionTodos,
    &_002_ability_word_todos::AbilityWordTodos,
    &_003_subtypes::Subtypes,
];

/// Apply every migration to the plugin in order.
///
/// # Errors
/// If the plugin layout is unusable or any migration fails.
pub fn apply_all(plugin_dir: &Path) -> anyhow::Result<()> {
    let plugin_layout = PluginLayout::new(plugin_dir)?;
    for migration in MIGRATIONS {
        migration.apply(&plugin_layout)?;
    }
    Ok(())
}

/// Apply a single migration by index.
///
/// # Errors
/// If `migration_number` is out of range, the plugin layout is unusable, or the
/// migration fails.
pub fn apply(plugin_dir: &Path, migration_number: usize) -> anyhow::Result<()> {
    if migration_number >= MIGRATIONS.len() {
        return Err(anyhow::anyhow!("Migration number out of range"));
    }

    MIGRATIONS[migration_number].apply(&PluginLayout::new(plugin_dir)?)
}

/// Converts a keyword ability name to a Rust identifier, e.g.
/// "Cumulative upkeep" -> "`CumulativeUpkeep`", "Jump-start" -> "`JumpStart`".
pub(crate) fn to_rust_ident(name: &str) -> String {
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
        })
        .filter(char::is_ascii_alphanumeric)
        .collect::<String>()
}

#[cfg(test)]
mod tests {
    use super::to_rust_ident;

    #[test]
    fn rust_idents() {
        assert_eq!(to_rust_ident("Flying"), "Flying");
        assert_eq!(to_rust_ident("Cumulative upkeep"), "CumulativeUpkeep");
        assert_eq!(to_rust_ident("Jump-start"), "JumpStart");
        assert_eq!(to_rust_ident("Doctor's companion"), "DoctorsCompanion");
    }

    #[test]
    fn unimplemented_tracks_finished_ron() {
        use super::is_unimplemented;
        let dir = tempfile::tempdir().unwrap();
        let final_path = dir.path().join("Serra Angel.ron");
        // Nothing on disk: needs a stub.
        assert!(is_unimplemented(&final_path));
        // A finished card: implemented.
        std::fs::write(&final_path, "x").unwrap();
        assert!(!is_unimplemented(&final_path));
    }
}
