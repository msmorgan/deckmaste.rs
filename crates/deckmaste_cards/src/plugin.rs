//! Loading a plugin directory: macro definitions, the subtype declarations
//! invoking them, and cards.

use std::collections::HashMap;
use std::path::{Path, PathBuf};

use anyhow::Context;
use deckmaste_core::{Card, Ident, Subtype};

use crate::macros::{MacroDef, MacroFile, MacroSet};

/// A plugin directory with its macro layer loaded and expanded.
pub struct Plugin {
    root: PathBuf,
    /// The macros in scope: definitions from `macros/*/`, plus every declared
    /// subtype as a nullary macro.
    pub macros: MacroSet,
    /// The subtypes declared by `types/*/*.ron`, fully expanded.
    pub subtypes: HashMap<Ident, Subtype>,
}

impl Plugin {
    pub fn load(root: impl Into<PathBuf>) -> anyhow::Result<Self> {
        let root = root.into();

        // Macro definitions are named by their file stem.
        let mut macros = MacroSet::default();
        for path in subdirs(&root.join("macros"))?
            .iter()
            .map(|dir| ron_files(dir))
            .collect::<Result<Vec<_>, _>>()?
            .into_iter()
            .flatten()
        {
            let file: MacroFile = deckmaste_core::ron::options()
                .from_str(&read(&path)?)
                .with_context(|| format!("parsing macro {path:?}"))?;
            macros
                .insert(file_stem(&path)?, MacroDef::from(file))
                .with_context(|| format!("loading {path:?}"))?;
        }

        // A declaration joins the macro scope as a nullary macro whose body
        // is the file's source, verbatim; expanding it here both validates
        // it and fills the subtype table.
        let mut subtypes = HashMap::new();
        for path in subdirs(&root.join("types"))?
            .iter()
            .map(|dir| ron_files(dir))
            .collect::<Result<Vec<_>, _>>()?
            .into_iter()
            .flatten()
        {
            let declaration = read(&path)?;
            let subtype: Subtype = macros
                .from_str(&declaration)
                .with_context(|| format!("expanding {path:?}"))?;
            let stem = file_stem(&path)?;
            anyhow::ensure!(
                subtype.name == stem,
                "{path:?} declares subtype `{}`; the file stem must match",
                subtype.name,
            );
            macros
                .declare(stem, &declaration)
                .with_context(|| format!("declaring {path:?}"))?;
            subtypes.insert(subtype.name, subtype);
        }

        Ok(Self {
            root,
            macros,
            subtypes,
        })
    }

    /// The file a card of this name would live in.
    pub fn card_path(&self, name: &str) -> PathBuf {
        self.root.join("cards").join(format!("{name}.ron"))
    }

    /// Reads and parses `cards/<name>.ron`, with the plugin's macros in scope.
    pub fn card(&self, name: &str) -> anyhow::Result<Card> {
        let path = self.card_path(name);
        self.macros
            .from_str(&read(&path)?)
            .with_context(|| format!("parsing {path:?}"))
    }
}

fn read(path: &Path) -> anyhow::Result<String> {
    std::fs::read_to_string(path).with_context(|| format!("reading {path:?}"))
}

fn file_stem(path: &Path) -> anyhow::Result<Ident> {
    path.file_stem()
        .and_then(|stem| stem.to_str())
        .map(Ident::new)
        .with_context(|| format!("non-UTF-8 file name {path:?}"))
}

/// The `.ron` files directly in `dir`, sorted; an absent directory is empty.
fn ron_files(dir: &Path) -> anyhow::Result<Vec<PathBuf>> {
    entries(dir, |path| path.extension().is_some_and(|ext| ext == "ron"))
}

/// The subdirectories of `dir`, sorted; an absent directory is empty.
fn subdirs(dir: &Path) -> anyhow::Result<Vec<PathBuf>> { entries(dir, |path| path.is_dir()) }

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
        .with_context(|| format!("reading {dir:?}"))?;
    paths.retain(|path| keep(path));
    paths.sort();
    Ok(paths)
}
