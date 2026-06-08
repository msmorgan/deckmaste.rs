use std::path::Path;
use std::sync::LazyLock;

use regex::Regex;

use crate::layout::PluginLayout;

mod _000_keyword_ability_todos;
mod _001_keyword_action_todos;
mod _002_ability_word_todos;
mod _003_subtypes;
mod _004_card_todos;
mod _005_basic_lands;
mod _006_vanilla_creatures;
mod card_todo;
mod keyword_todos;

trait Migration {
    fn apply(&self, plugin: &PluginLayout) -> anyhow::Result<()>;
}

/// Whether a definition still needs generating, keyed on its *finished*
/// `.ron` path. A stub writer (re)generates its `<name>.todo.ron` only while
/// the finished `<name>.ron` is absent; once the final exists — converted by
/// a later migration or hand-written — the stub writers leave it alone.
///
/// Editing a `.todo.ron` in place offers no such protection: a stub is a
/// disposable draft, so promote finished work to `<name>.ron`.
fn is_unimplemented(final_path: &Path) -> bool { !final_path.exists() }

const MIGRATIONS: &[&dyn Migration] = &[
    &_000_keyword_ability_todos::KeywordAbilityTodos,
    &_001_keyword_action_todos::KeywordActionTodos,
    &_002_ability_word_todos::AbilityWordTodos,
    &_003_subtypes::Subtypes,
    &_004_card_todos::CardTodos,
    &_005_basic_lands::BasicLands,
    &_006_vanilla_creatures::VanillaCreatures,
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

/// Converts a keyword ability name to a Rust identifier, e.g.
/// "Cumulative upkeep" -> "`CumulativeUpkeep`", "Jump-start" -> "`JumpStart`".
fn to_rust_ident(name: &str) -> String {
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
}
