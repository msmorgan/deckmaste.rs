use std::path::{Path, PathBuf};
use std::sync::LazyLock;

use regex::Regex;

use crate::layout::PluginLayout;

mod _000_keyword_ability_todos;
mod _001_keyword_action_todos;
mod _002_ability_word_todos;
mod _003_subtypes;
pub(crate) mod _004_card_todos;
mod _005_basic_lands;
mod _006_vanilla_creatures;
mod _007_simple_lands;
mod _008_french_vanilla_creatures;
mod _009_simple_mana_rocks;
mod _010_simple_mana_dorks;
mod card_todo;
mod creature_face;
mod keyword_ability;
mod keyword_todos;
mod mana_ability;

trait Migration {
    fn apply(&self, plugin: &PluginLayout) -> anyhow::Result<()>;
}

/// Whether a definition still needs generating, keyed on its *finished* `.ron`
/// path. A stub writer (re)generates its `<name>.todo.ron` only while neither
/// the finished `<name>.ron` nor a parked `<name>.ron.pending` (a Blocked
/// draft) exists; once either is present the stub writers leave it alone.
fn is_unimplemented(final_path: &Path) -> bool {
    !final_path.exists() && !pending_path(final_path).exists()
}

/// The parked sibling of a finished `.ron` path: `Foo.ron` ->
/// `Foo.ron.pending`.
fn pending_path(final_path: &Path) -> PathBuf {
    let mut name = final_path.file_name().unwrap_or_default().to_os_string();
    name.push(".pending");
    final_path.with_file_name(name)
}

const MIGRATIONS: &[&dyn Migration] = &[
    &_000_keyword_ability_todos::KeywordAbilityTodos,
    &_001_keyword_action_todos::KeywordActionTodos,
    &_002_ability_word_todos::AbilityWordTodos,
    &_003_subtypes::Subtypes,
    &_004_card_todos::CardTodos,
    &_005_basic_lands::BasicLands,
    &_006_vanilla_creatures::VanillaCreatures,
    &_007_simple_lands::SimpleLands,
    &_008_french_vanilla_creatures::FrenchVanillaCreatures,
    &_009_simple_mana_rocks::SimpleManaRocks,
    &_010_simple_mana_dorks::SimpleManaDorks,
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
    fn pending_card_counts_as_implemented() {
        use super::is_unimplemented;
        let dir = tempfile::tempdir().unwrap();
        let final_path = dir.path().join("Serra Angel.ron");
        // Nothing on disk: needs a stub.
        assert!(is_unimplemented(&final_path));
        // A parked draft beside it: no longer needs a stub.
        std::fs::write(dir.path().join("Serra Angel.ron.pending"), "x").unwrap();
        assert!(!is_unimplemented(&final_path));
        // A finished card: also implemented.
        std::fs::remove_file(dir.path().join("Serra Angel.ron.pending")).unwrap();
        std::fs::write(&final_path, "x").unwrap();
        assert!(!is_unimplemented(&final_path));
    }
}
