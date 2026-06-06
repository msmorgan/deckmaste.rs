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
}
