//! The conventions of a plugin directory, shared by its readers
//! (`deckmaste_cards`) and writers (`deckmaste_migrations`).

use std::path::{Path, PathBuf};

// The directory roles under a plugin root.
pub const MACROS_DIR: &str = "macros";
pub const TYPES_DIR: &str = "types";
pub const CARDS_DIR: &str = "cards";
pub const KEYWORD_ABILITIES_DIR: &str = "keyword_abilities";
pub const KEYWORD_ACTIONS_DIR: &str = "keyword_actions";
pub const ABILITY_WORDS_DIR: &str = "ability_words";

pub const KEYWORD_ABILITIES_FILE: &str = "keyword_abilities.ron";

/// The file name a card of this name is stored under: [`card_filename`]
/// plus the extension.
#[must_use]
pub fn card_file(name: &str) -> String { format!("{}.ron", card_filename(name)) }

/// Where a card of this name lives under the plugin `root`.
#[must_use]
pub fn card_path(root: &Path, name: &str) -> PathBuf { root.join(CARDS_DIR).join(card_file(name)) }

/// Whether card-file source is still an unimplemented stub. A stub is any
/// file with a line starting (modulo indentation) with `Todo(` — checked
/// per line because the `Todo(` may follow a `// CR ...` comment line, so
/// it is not necessarily at the start of the file. A convention check, not
/// a parser: migrations may only overwrite files while this holds.
#[must_use]
pub fn is_todo_source(source: &str) -> bool {
    source
        .lines()
        .any(|line| line.trim_start().starts_with("Todo("))
}

/// Maps Windows-unsafe filename characters to their fullwidth equivalents,
/// e.g. "Fire // Ice" -> "Fire ／／ Ice". Card files are written and looked
/// up through this one mapping, which also keeps separators in a card's name
/// from escaping `cards/`.
#[must_use]
pub fn card_filename(name: &str) -> String {
    name.chars()
        .map(|c| match c {
            '<' => '＜',
            '>' => '＞',
            ':' => '：',
            '"' => '＂',
            '/' => '／',
            '\\' => '＼',
            '|' => '｜',
            '?' => '？',
            '*' => '＊',
            c => c,
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn filenames() {
        assert_eq!(card_filename("Fire // Ice"), "Fire ／／ Ice");
        assert_eq!(card_filename("Question?"), "Question？");
        assert_eq!(card_filename("Lightning Bolt"), "Lightning Bolt");
    }

    #[test]
    fn todo_sources() {
        assert!(is_todo_source("Todo(\n    layout: \"normal\",\n)"));
        // The Todo( line may follow a // CR comment line.
        assert!(is_todo_source(
            "// CR 205.3i\nTodo(\n    layout: \"normal\",\n)"
        ));
        assert!(is_todo_source("    Todo(layout: \"normal\")"));
        // str::lines strips the \r of CRLF endings.
        assert!(is_todo_source("Todo(\r\n    layout: \"normal\",\r\n)"));
        assert!(!is_todo_source(
            "Normal(\n    name: \"Plains\",\n    types: [Land],\n)"
        ));
        assert!(!is_todo_source(""));
    }
}
