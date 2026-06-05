use std::path::Path;
use std::sync::LazyLock;
use regex::Regex;
use crate::layout::PluginLayout;

mod _000_keyword_ability_todos;
mod _001_keyword_action_todos;
mod _002_ability_word_todos;
mod _003_subtypes;
mod _004_card_todos;
mod keyword_todos;

trait Migration {
    fn apply(&self, plugin: &PluginLayout) -> anyhow::Result<()>;
}

/// A file may be (over)written only while it is still an unimplemented stub.
/// (?m) anchors ^ at line starts: the Todo( line may follow a // CR comment
/// line, so it is not necessarily at the start of the file.
fn is_todo(path: &Path) -> anyhow::Result<bool> {
    static TODO_PATTERN: LazyLock<Regex> =
        LazyLock::new(|| Regex::new(r"(?m)^\s*Todo\(").unwrap());

    if !path.exists() {
        return Ok(true);
    }
    Ok(TODO_PATTERN.is_match(&std::fs::read_to_string(path)?))
}

const MIGRATIONS: &[&dyn Migration] = &[
    &_000_keyword_ability_todos::KeywordAbilityTodos,
    &_001_keyword_action_todos::KeywordActionTodos,
    &_002_ability_word_todos::AbilityWordTodos,
    &_003_subtypes::Subtypes,
    &_004_card_todos::CardTodos,
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
/// "Cumulative upkeep" -> "CumulativeUpkeep", "Jump-start" -> "JumpStart".
fn to_rust_ident(name: &str) -> String {
    static SPLIT: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"[\s|-]+").unwrap());

    SPLIT
        .split(name)
        .flat_map(|word| {
            let mut chars = word.chars();
            chars
                .next()
                .into_iter()
                .flat_map(|first| first.to_uppercase())
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