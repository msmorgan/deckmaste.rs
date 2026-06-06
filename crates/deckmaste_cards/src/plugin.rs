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
        // it and fills the subtype table.
        let mut subtypes = HashMap::new();
        for path in ron_files_recursive(&root.join("types"))? {
            let declaration = read(&path)?;
            let subtype: Subtype = macros
                .read_str(&declaration)
                .with_context(|| format!(r#"expanding "{}""#, path.display()))?;
            macros
                .declare(subtype.name, &declaration)
                .with_context(|| format!(r#"declaring "{}""#, path.display()))?;
            subtypes.insert(subtype.name, subtype);
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
        self.root.join("cards").join(format!("{name}.ron"))
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

fn read(path: &Path) -> anyhow::Result<String> {
    std::fs::read_to_string(path).with_context(|| format!(r#"reading "{}""#, path.display()))
}

/// The `.ron` files under `dir` at any depth, sorted; an absent directory is
/// empty.
fn ron_files_recursive(dir: &Path) -> anyhow::Result<Vec<PathBuf>> {
    let mut files = entries(dir, |path| path.extension().is_some_and(|ext| ext == "ron"))?;
    for subdir in entries(dir, std::path::Path::is_dir)? {
        files.extend(ron_files_recursive(&subdir)?);
    }
    files.sort();
    Ok(files)
}

fn entries(dir: &Path, keep: impl Fn(&Path) -> bool) -> anyhow::Result<Vec<PathBuf>> {
    if !dir.exists() {
        return Ok(vec![]);
    }
    let mut paths = dir
        .read_dir()
        .and_then(|entries| {
            entries
                .map(|entry| entry.map(|entry| entry.path()))
                .collect::<Result<Vec<_>, _>>()
        })
        .with_context(|| format!(r#"reading "{}""#, dir.display()))?;
    paths.retain(|path| keep(path));
    paths.sort();
    Ok(paths)
}
