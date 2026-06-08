//! The conventions of a plugin directory, shared by its readers
//! (`deckmaste_cards`) and writers (`deckmaste_migrations`).

use std::path::{Path, PathBuf};

// The directory roles under a plugin root.
pub const MACROS_DIR: &str = "macros";
pub const TYPES_DIR: &str = "types";
pub const CARDS_DIR: &str = "cards";
pub const TOKENS_DIR: &str = "tokens";
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

/// The file name a token of this name is stored under.
#[must_use]
pub fn token_file(name: &str) -> String { format!("{name}.ron") }

/// Where a token of this name lives under the plugin `root`.
#[must_use]
pub fn token_path(root: &Path, name: &str) -> PathBuf {
    root.join(TOKENS_DIR).join(token_file(name))
}

/// The filename suffix marking a stub still awaiting implementation. The
/// finished definition drops `.todo`, living beside it at `<stem>.ron`.
pub const TODO_SUFFIX: &str = ".todo.ron";

/// The todo-stub file name for this `stem`, e.g. `todo_file("Flying")` is
/// `"Flying.todo.ron"`.
#[must_use]
pub fn todo_file(stem: &str) -> String { format!("{stem}{TODO_SUFFIX}") }

/// The todo-stub file name for a card of this name: [`card_filename`] under
/// the [`TODO_SUFFIX`].
#[must_use]
pub fn card_todo_file(name: &str) -> String { todo_file(&card_filename(name)) }

/// Whether `path` is a todo stub by filename convention (ends in
/// [`TODO_SUFFIX`]). The complement of a finished `.ron` definition.
#[must_use]
pub fn is_todo_file(path: &Path) -> bool {
    path.file_name()
        .and_then(|name| name.to_str())
        .is_some_and(|name| name.ends_with(TODO_SUFFIX))
}

/// The finished file name a todo stub graduates into: `"Foo.todo.ron"` ->
/// `"Foo.ron"`. `None` if `todo_name` isn't a [`TODO_SUFFIX`] stub name.
#[must_use]
pub fn final_for_todo(todo_name: &str) -> Option<String> {
    todo_name
        .strip_suffix(TODO_SUFFIX)
        .map(|stem| format!("{stem}.ron"))
}

/// The in-progress suffix for the resolution pipeline: a Card-shaped definition
/// that hasn't graduated yet. Distinct from the legacy [`TODO_SUFFIX`]
/// (`.todo.ron`) stub. Its extension is `.todo` (not `ron`), so every `*.ron`
/// glob — the plugin loader, `validate`, the macros/types reads — skips it for
/// free.
pub const RON_TODO_SUFFIX: &str = ".ron.todo";

/// Whether `path` names a `.ron.todo` (an ungraduated Card-shaped definition).
#[must_use]
pub fn is_ron_todo_file(path: &Path) -> bool {
    path.file_name()
        .and_then(|name| name.to_str())
        .is_some_and(|name| name.ends_with(RON_TODO_SUFFIX))
}

/// The graduated file name for a `.ron.todo`: `"Foo.ron.todo"` -> `"Foo.ron"`.
/// `None` if `name` isn't a [`RON_TODO_SUFFIX`] file.
#[must_use]
pub fn graduated_name(name: &str) -> Option<String> {
    name.strip_suffix(".todo").map(str::to_owned)
}

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

/// Maps Windows-unsafe filename characters to bracketed ASCII placeholders,
/// e.g. "Fire // Ice" -> "Fire {slash}{slash} Ice". Card files are written and
/// looked up through this one mapping, which also keeps separators in a card's
/// name from escaping `cards/`.
///
/// The tokens are ASCII, not fullwidth lookalikes (`／：？`…), so the names
/// survive Unicode compatibility normalization: NFKC folds `／` back into `/`,
/// which would reopen that escape. `{` never occurs in a card name, so the
/// encoding stays collision-free.
#[must_use]
pub fn card_filename(name: &str) -> String {
    let mut out = String::with_capacity(name.len());
    for c in name.chars() {
        match c {
            '<' => out.push_str("{less}"),
            '>' => out.push_str("{greater}"),
            ':' => out.push_str("{colon}"),
            '"' => out.push_str("{quote}"),
            '/' => out.push_str("{slash}"),
            '\\' => out.push_str("{backslash}"),
            '|' => out.push_str("{pipe}"),
            '?' => out.push_str("{question}"),
            '*' => out.push_str("{star}"),
            _ => out.push(c),
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn filenames() {
        assert_eq!(card_filename("Fire // Ice"), "Fire {slash}{slash} Ice");
        assert_eq!(card_filename("Question?"), "Question{question}");
        assert_eq!(
            card_filename("Summon: Esper Ramuh"),
            "Summon{colon} Esper Ramuh"
        );
        assert_eq!(card_filename("Lightning Bolt"), "Lightning Bolt");
    }

    #[test]
    fn todo_filenames() {
        assert_eq!(todo_file("Flying"), "Flying.todo.ron");
        // Card todos carry the same name sanitization as finished cards.
        assert_eq!(
            card_todo_file("Fire // Ice"),
            "Fire {slash}{slash} Ice.todo.ron"
        );
        assert_eq!(card_todo_file("Lightning Bolt"), "Lightning Bolt.todo.ron");
    }

    #[test]
    fn todo_file_recognition() {
        assert!(is_todo_file(Path::new("cards/Plains.todo.ron")));
        assert!(!is_todo_file(Path::new("cards/Plains.ron")));
        // A bare `.todo.ron` with no stem still counts; a `.ron` never does.
        assert!(is_todo_file(Path::new(".todo.ron")));
        assert!(!is_todo_file(Path::new("cards/")));
    }

    #[test]
    fn todo_graduates_to_final() {
        assert_eq!(
            final_for_todo("Plains.todo.ron").as_deref(),
            Some("Plains.ron")
        );
        assert_eq!(
            final_for_todo("Fire {slash}{slash} Ice.todo.ron").as_deref(),
            Some("Fire {slash}{slash} Ice.ron")
        );
        // Not a stub name: nothing to graduate.
        assert_eq!(final_for_todo("Plains.ron"), None);
    }

    #[test]
    fn ron_todo_recognition() {
        assert!(is_ron_todo_file(Path::new("cards/Sol Ring.ron.todo")));
        // A finished `.ron` and a legacy `.todo.ron` stub are not `.ron.todo`.
        assert!(!is_ron_todo_file(Path::new("cards/Sol Ring.ron")));
        assert!(!is_ron_todo_file(Path::new("cards/Sol Ring.todo.ron")));
    }

    #[test]
    fn graduated_names() {
        assert_eq!(
            graduated_name("Sol Ring.ron.todo").as_deref(),
            Some("Sol Ring.ron")
        );
        assert_eq!(
            graduated_name("Fire {slash}{slash} Ice.ron.todo").as_deref(),
            Some("Fire {slash}{slash} Ice.ron")
        );
        assert_eq!(graduated_name("Sol Ring.ron"), None);
    }

    #[test]
    fn todo_sources() {
        assert!(is_todo_source("Todo(\n    layout: \"normal\",\n)"));
        // The Todo( line may follow a // CR comment line.
        assert!(is_todo_source(
            "// [CR#205.3i]\nTodo(\n    layout: \"normal\",\n)"
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
