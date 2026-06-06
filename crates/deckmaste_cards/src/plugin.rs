//! Loading a plugin directory: macro definitions, the subtype declarations
//! invoking them, and cards.
//!
//! Directories carry a file's *role* (`macros/`, `types/`, `cards/`);
//! everything below them is organizational, and names come from file
//! contents.

use std::collections::HashMap;
use std::path::{Path, PathBuf};

use anyhow::Context;
use deckmaste_core::{Card, Ident, Subtype};

use crate::macros::{MacroDef, MacroSet};

/// A plugin directory with its macro layer loaded and expanded.
pub struct Plugin {
    root: PathBuf,
    /// The macros in scope: definitions from `macros/`, plus every declared
    /// subtype as a nullary macro.
    pub macros: MacroSet,
    /// The subtypes declared by `types/`, fully expanded.
    pub subtypes: HashMap<Ident, Subtype>,
}

impl Plugin {
    /// # Errors
    /// If a macro definition or subtype declaration fails to read, expand,
    /// or register, or a directory isn't listable.
    pub fn load(root: impl Into<PathBuf>) -> anyhow::Result<Self> {
        let root = root.into();

        // Macro definitions are self-describing.
        let mut macros = MacroSet::default();
        for path in ron_files_recursive(&root.join("macros"))? {
            let def: MacroDef = deckmaste_core::ron::options()
                .from_str(&read(&path)?)
                .with_context(|| format!(r#"parsing macro "{}""#, path.display()))?;
            macros
                .insert(&def)
                .with_context(|| format!(r#"loading "{}""#, path.display()))?;
        }

        // A declaration joins the macro scope as a nullary macro whose body
        // is the file's source, verbatim; expanding it here both validates
        // it and fills the subtype table. Declarations may reference each
        // other regardless of file order, so failures are retried until a
        // pass stops making progress — only then is the first one real.
        let mut subtypes = HashMap::new();
        let mut pending = Vec::new();
        for path in ron_files_recursive(&root.join("types"))? {
            let declaration = read(&path)?;
            pending.push((path, declaration));
        }
        while !pending.is_empty() {
            let attempted = pending.len();
            let mut failures = Vec::new();
            for (path, declaration) in pending {
                match macros.read_str::<Subtype>(&declaration) {
                    Ok(subtype) => {
                        macros
                            .declare(subtype.name, &declaration)
                            .with_context(|| format!(r#"declaring "{}""#, path.display()))?;
                        subtypes.insert(subtype.name, subtype);
                    }
                    Err(error) => failures.push((path, declaration, error)),
                }
            }
            if failures.len() == attempted {
                let (path, _, error) = failures.swap_remove(0);
                return Err(error).with_context(|| format!(r#"expanding "{}""#, path.display()));
            }
            pending = failures
                .into_iter()
                .map(|(path, declaration, _)| (path, declaration))
                .collect();
        }

        Ok(Self {
            root,
            macros,
            subtypes,
        })
    }

    /// The file a card of this name would live in.
    #[must_use]
    pub fn card_path(&self, name: &str) -> PathBuf {
        self.root
            .join("cards")
            .join(format!("{}.ron", card_filename(name)))
    }

    /// Reads and parses `cards/<name>.ron`, with the plugin's macros in scope.
    ///
    /// # Errors
    /// If the file is missing or doesn't expand to a card.
    pub fn card(&self, name: &str) -> anyhow::Result<Card> {
        let path = self.card_path(name);
        self.macros
            .read_str(&read(&path)?)
            .with_context(|| format!(r#"parsing "{}""#, path.display()))
    }
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

fn read(path: &Path) -> anyhow::Result<String> {
    std::fs::read_to_string(path).with_context(|| format!(r#"reading "{}""#, path.display()))
}

/// The `.ron` files under `dir` at any depth, sorted; an absent directory is
/// empty. Entries are classified by [`std::fs::DirEntry::file_type`], so a
/// directory named like a file is recursed into, not read.
fn ron_files_recursive(dir: &Path) -> anyhow::Result<Vec<PathBuf>> {
    if !dir.exists() {
        return Ok(vec![]);
    }
    let context = || format!(r#"reading "{}""#, dir.display());
    let mut files = Vec::new();
    let mut subdirs = Vec::new();
    for entry in dir.read_dir().with_context(context)? {
        let entry = entry.with_context(context)?;
        let path = entry.path();
        if entry.file_type().with_context(context)?.is_dir() {
            subdirs.push(path);
        } else if path.extension().is_some_and(|ext| ext == "ron") && path.is_file() {
            // `is_file` follows symlinks so a linked file still loads;
            // `file_type` doesn't, so a linked directory can't form a cycle.
            files.push(path);
        }
    }
    subdirs.sort();
    for subdir in subdirs {
        files.extend(ron_files_recursive(&subdir)?);
    }
    files.sort();
    Ok(files)
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
