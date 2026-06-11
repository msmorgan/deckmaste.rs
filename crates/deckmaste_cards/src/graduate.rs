//! Graduating definitions. A `<name>.ron.todo` is a Card-shaped file that may
//! reference not-yet-real macros or carry `Unparsed(…)` placeholders; it
//! graduates — renamed to `<name>.ron` — the moment it parses cleanly through
//! the macro reader. This is the universal completion gate from the
//! ability-resolution-pipeline design: no per-card logic, just "does it parse."
//!
//! Plan-1 scope: `cards/` only, single pass. Cards aren't part of the macro
//! scope, so graduating one never unblocks another — no fixpoint needed here.
//! Cross-directory graduation (subtype/keyword definitions) and the
//! retry-until-stable loop generalize in a later plan.

use std::collections::BTreeMap;
use std::path::Path;
use std::path::PathBuf;
use std::sync::LazyLock;

use deckmaste_core::Card;
use deckmaste_core::plugin::CARDS_DIR;
use deckmaste_core::plugin::graduated_name;
use deckmaste_core::plugin::is_ron_todo_file;
use regex::Regex;

use crate::plugin::Plugin;
use crate::plugin::read;

/// Outcome of a graduation run.
#[derive(Debug)]
pub struct GraduateReport {
    /// Files renamed `<name>.ron.todo` -> `<name>.ron`.
    pub graduated: Vec<PathBuf>,
    /// `.ron.todo` files that did not parse this run (still in progress).
    pub remaining: usize,
    /// Of the remaining: cards still carrying `Unparsed(...)` oracle lines.
    pub unresolved: usize,
    /// Of the remaining: count of cards blocked per missing macro name (the
    /// macro the user could build next to unblock that many cards).
    pub blocked_on_macro: BTreeMap<String, usize>,
    /// Of the remaining: failures that don't match the "unregistered macro"
    /// pattern (path + first line of the error), for visibility.
    pub other: Vec<(PathBuf, String)>,
}

/// Classifies a graduation failure by its macro-reader error message.
/// Couples to `macro_ron::expand`'s "`X` is neither a variant of `K` nor a
/// known `K` macro" message — a stable internal contract. Anything that
/// doesn't match degrades to `Other` (still surfaced, just not bucketed).
enum Blocker {
    Unresolved,
    Macro(String),
    Other,
}

/// Captures the IDENT from a macro-reader "neither a variant ... nor a known
/// ... macro" error. The message may be prefixed with "in the expansion of
/// `M`:"; we match the inner clause anywhere in the string. IDENT is the
/// backtick-delimited run between the leading backtick and " is neither a
/// variant of ".
static UNREGISTERED_MACRO: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"`([^`]+)` is neither a variant of `[^`]+` nor a known `[^`]+` macro")
        .expect("valid regex")
});

fn classify(error: &str) -> Blocker {
    match UNREGISTERED_MACRO.captures(error) {
        Some(caps) => {
            let ident = &caps[1];
            // A still-present `Unparsed(...)` placeholder fails this same way
            // with IDENT == "Unparsed"; bucket those as unresolved oracle text,
            // not as a macro the user should build.
            if ident == "Unparsed" {
                Blocker::Unresolved
            } else {
                Blocker::Macro(ident.to_owned())
            }
        }
        None => Blocker::Other,
    }
}

/// The `*.ron.todo` files directly under `dir`, sorted; an absent directory is
/// empty. `cards/` is flat, so this is non-recursive (matches the todo
/// writers).
fn ron_todo_files(dir: &Path) -> anyhow::Result<Vec<PathBuf>> {
    if !dir.exists() {
        return Ok(vec![]);
    }
    let mut files: Vec<PathBuf> = std::fs::read_dir(dir)?
        .map(|entry| entry.map(|e| e.path()))
        .collect::<Result<_, _>>()?;
    files.retain(|path| path.is_file() && is_ron_todo_file(path));
    files.sort();
    Ok(files)
}

/// Graduates every `cards/*.ron.todo` in `plugin_dir` that the macro reader
/// parses as a [`Card`] — renaming it to `<name>.ron`. The plugin's builtin
/// sibling prelude is in scope. A file that fails to parse (an `Unparsed(…)`
/// placeholder, or a referenced macro/subtype that isn't real yet) is left in
/// place and counted as `remaining`.
///
/// # Panics
/// If a file that passed [`is_ron_todo_file`] somehow lacks a valid UTF-8
/// file-name component — which the filesystem conventions make impossible in
/// practice.
///
/// # Errors
/// If the plugin (or its prelude) fails to load, or a file isn't readable or
/// renamable. A file that *reads but doesn't parse* is `remaining`, not an
/// error.
pub fn graduate_plugin(plugin_dir: &Path) -> anyhow::Result<GraduateReport> {
    let plugin = Plugin::load_with_sibling_prelude(plugin_dir)?;
    let mut graduated = Vec::new();
    let mut remaining = 0;
    let mut unresolved = 0;
    let mut blocked_on_macro: BTreeMap<String, usize> = BTreeMap::new();
    let mut other = Vec::new();

    for path in ron_todo_files(&plugin_dir.join(CARDS_DIR))? {
        let source = read(&path)?;
        match plugin.macros.read_str::<Card>(&source) {
            Ok(_) => {
                // is_ron_todo_file guarantees a `.ron.todo` name, so
                // graduated_name is always present.
                let final_name = path
                    .file_name()
                    .and_then(|name| name.to_str())
                    .and_then(graduated_name)
                    .expect("a .ron.todo file has a graduated name");
                let final_path = path.with_file_name(final_name);
                std::fs::rename(&path, &final_path)?;
                graduated.push(final_path);
            }
            Err(e) => {
                remaining += 1;
                let message = e.to_string();
                match classify(&message) {
                    Blocker::Unresolved => unresolved += 1,
                    Blocker::Macro(ident) => *blocked_on_macro.entry(ident).or_default() += 1,
                    Blocker::Other => {
                        other.push((path, message.lines().next().unwrap_or("").to_owned()));
                    }
                }
            }
        }
    }
    Ok(GraduateReport {
        graduated,
        remaining,
        unresolved,
        blocked_on_macro,
        other,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Writes `source` to `<root>/<plugin>/cards/<file>`.
    fn write_card(root: &Path, plugin: &str, file: &str, source: &str) {
        let cards = root.join(plugin).join("cards");
        std::fs::create_dir_all(&cards).unwrap();
        std::fs::write(cards.join(file), source).unwrap();
    }

    /// A `.ron.todo` whose body parses as a Card graduates: renamed to `.ron`,
    /// the `.ron.todo` gone.
    #[test]
    fn parseable_card_graduates() {
        let root = tempfile::tempdir().unwrap();
        write_card(
            root.path(),
            "wizards",
            "Wastes.ron.todo",
            r#"Normal(name: "Wastes", types: [Land])"#,
        );
        let report = graduate_plugin(&root.path().join("wizards")).unwrap();

        let cards = root.path().join("wizards").join("cards");
        assert!(cards.join("Wastes.ron").exists(), "graduated to .ron");
        assert!(!cards.join("Wastes.ron.todo").exists(), ".ron.todo removed");
        assert_eq!(report.graduated.len(), 1);
        assert_eq!(report.remaining, 0);
    }

    /// A card carrying an `Unparsed(…)` placeholder fails to parse, so it stays
    /// a `.ron.todo`.
    #[test]
    fn unparsed_card_stays() {
        let root = tempfile::tempdir().unwrap();
        write_card(
            root.path(),
            "wizards",
            "Mystery.ron.todo",
            r#"Normal(name: "Mystery", types: [Creature], abilities: [Unparsed("Draw a card.")], power: 1, toughness: 1)"#,
        );
        let report = graduate_plugin(&root.path().join("wizards")).unwrap();

        let cards = root.path().join("wizards").join("cards");
        assert!(cards.join("Mystery.ron.todo").exists(), "still a .ron.todo");
        assert!(!cards.join("Mystery.ron").exists());
        assert_eq!(report.graduated.len(), 0);
        assert_eq!(report.remaining, 1);
        // An `Unparsed(...)` placeholder is bucketed as unresolved oracle text,
        // NOT as a macro the user should build.
        assert_eq!(report.unresolved, 1);
        assert!(!report.blocked_on_macro.contains_key("Unparsed"));
    }

    /// A card whose ability references a macro that isn't registered (here
    /// `Flying`, absent from the bare test plugin) fails to expand and lands in
    /// `blocked_on_macro` keyed by the missing macro name.
    #[test]
    fn card_blocked_on_macro_is_tallied() {
        let root = tempfile::tempdir().unwrap();
        write_card(
            root.path(),
            "wizards",
            "Bird.ron.todo",
            r#"Normal(name: "Bird", types: [Creature], abilities: [Flying], power: 1, toughness: 1)"#,
        );
        let report = graduate_plugin(&root.path().join("wizards")).unwrap();

        let cards = root.path().join("wizards").join("cards");
        assert!(cards.join("Bird.ron.todo").exists(), "still a .ron.todo");
        assert_eq!(report.remaining, 1);
        assert_eq!(report.unresolved, 0);
        assert_eq!(report.blocked_on_macro.get("Flying"), Some(&1));
    }

    /// A card referencing a subtype that has no declaration in scope fails to
    /// expand, so it stays a `.ron.todo` (the "blocked on a missing macro"
    /// state — what used to be `.ron.pending`).
    #[test]
    fn card_with_unknown_subtype_stays() {
        let root = tempfile::tempdir().unwrap();
        write_card(
            root.path(),
            "wizards",
            "Elf.ron.todo",
            r#"Normal(name: "Elf Token", types: [Creature], subtypes: [ZzzNotARealSubtype], power: 1, toughness: 1)"#,
        );
        let report = graduate_plugin(&root.path().join("wizards")).unwrap();
        let cards = root.path().join("wizards").join("cards");
        assert!(cards.join("Elf.ron.todo").exists());
        assert_eq!(report.remaining, 1);
        // An undeclared subtype is a struct-position ron-native parser error,
        // which doesn't match the "unregistered macro" pattern — so it's
        // surfaced under `other`, not bucketed as a missing macro.
        assert_eq!(report.other.len(), 1);
        assert!(report.blocked_on_macro.is_empty());
    }

    /// Finished `.ron` cards are not touched (only `.ron.todo` is considered).
    #[test]
    fn finished_ron_is_left_alone() {
        let root = tempfile::tempdir().unwrap();
        write_card(
            root.path(),
            "wizards",
            "Done.ron",
            r#"Normal(name: "Done", types: [Land])"#,
        );
        let report = graduate_plugin(&root.path().join("wizards")).unwrap();
        assert_eq!(report.graduated.len(), 0);
        assert_eq!(report.remaining, 0);
        assert!(
            root.path()
                .join("wizards")
                .join("cards")
                .join("Done.ron")
                .exists()
        );
    }
}
