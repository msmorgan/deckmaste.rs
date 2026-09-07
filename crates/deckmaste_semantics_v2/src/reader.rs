//! Reading a `plugins_v2` plugin: `macros/`, `cards/`, `tokens/`, `rules/`.
//!
//! A directory carries a file's role; everything below it is organizational
//! and names come from file contents (`macros/`) or from the file name
//! (`cards/`, `tokens/`). Declaration files under `macros/meta/` are ordinary
//! definitions of kind `Macro`: a meta-macro whose body is itself a
//! definition, which is how `KeywordAction(name: …)` reads as one
//! (`docs/decisions/semantics-v2.md` §11).
//!
//! The declaration file is the shared contract: `deckmaste_english_v2` reads
//! its spelling and grammar, this crate reads its params and body. The
//! metadata half is opaque here — [`OpaqueMetadata`] accepts and discards any
//! shape — so neither crate depends on the other for the file.
//!
//! Nothing here validates. A file that will not read is an error with its path
//! and the reader's message; a file that reads is taken as written.

use std::collections::BTreeMap;
use std::path::Path;
use std::path::PathBuf;

use macro_ron::Ident;
use macro_ron::MacroDef;
use macro_ron::MacroSet;
use serde::Deserialize;
use serde::Deserializer;
use serde::de::IgnoredAny;

use crate::abilities::CharacteristicBundle;
use crate::card::Card;
use crate::rules::ConferralRule;
use crate::rules::DamageResultRule;
use crate::rules::RulesTables;
use crate::rules::SbaRule;

/// The `macros/` directory of a plugin.
pub const MACROS_DIR: &str = "macros";
/// The `cards/` directory of a plugin.
pub const CARDS_DIR: &str = "cards";
/// The `tokens/` directory of a plugin.
pub const TOKENS_DIR: &str = "tokens";
/// The `rules/` directory of a plugin.
pub const RULES_DIR: &str = "rules";

/// A declaration's consumer metadata, read and discarded.
///
/// `deckmaste_english_v2` types this field; this crate reads the same files
/// and has no use for it, so it accepts any shape and keeps nothing. That is
/// what makes the declaration file a shared contract rather than a dependency
/// (§11).
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct OpaqueMetadata;

impl<'de> Deserialize<'de> for OpaqueMetadata {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        IgnoredAny::deserialize(deserializer)?;
        Ok(OpaqueMetadata)
    }
}

/// A declaration as this crate sees it: the identity and signature, with the
/// definition itself for the expander.
#[derive(Debug, Clone)]
pub struct Declaration {
    /// The kind the declaration registers under — `KeywordAction`, `Subtype`,
    /// and the rest of [`crate::ron::DECLARATION_KINDS`], or a semantic kind
    /// for an ordinary phrase macro.
    pub kind: Ident,
    /// The definition, metadata erased.
    pub definition: MacroDef,
    /// The file it was read from.
    pub path: PathBuf,
}

/// A loaded `plugins_v2` plugin.
pub struct Plugin {
    root: PathBuf,
    /// The macros in scope: this plugin's definitions on top of whatever
    /// prelude it was loaded over.
    pub macros: MacroSet,
    /// Every declaration this plugin's `macros/` directory defines, keyed by
    /// `(kind, name)` — a definition naming several kinds appears once per
    /// kind, as it registers once per kind.
    pub declarations: BTreeMap<(Ident, Ident), Declaration>,
    /// The cards under `cards/`, keyed by file stem.
    pub cards: BTreeMap<String, Card>,
    /// The tokens under `tokens/`, keyed by file stem. A token is the
    /// characteristics an effect writes, qualities included [CR#111.3] — Lean
    /// `CharacteristicBundle`.
    pub tokens: BTreeMap<String, CharacteristicBundle>,
    /// The three rules tables under `rules/`.
    pub rules: RulesTables,
}

/// Why a plugin would not load.
#[derive(Debug, thiserror::Error)]
pub enum LoadError {
    #[error("reading `{path}`: {source}")]
    Io {
        path: PathBuf,
        source: std::io::Error,
    },
    #[error("reading `{path}`: {source}")]
    Ron {
        path: PathBuf,
        source: Box<::ron::error::SpannedError>,
    },
    #[error("registering `{path}`: {source}")]
    Register {
        path: PathBuf,
        source: macro_ron::InsertError,
    },
}

impl Plugin {
    /// Loads `root` with an empty macro scope.
    ///
    /// # Errors
    /// If a file is unreadable, a definition or value fails to read or expand,
    /// or a definition fails to register.
    pub fn load(root: impl Into<PathBuf>) -> Result<Self, LoadError> {
        Self::load_onto(crate::ron::macro_set(), root.into())
    }

    /// Loads `root` with `prelude`'s macros already in scope. Last plugin
    /// wins: `root`'s definitions override same-name entries from the prelude.
    ///
    /// # Errors
    /// As [`Plugin::load`].
    pub fn load_with_prelude(
        prelude: &Plugin,
        root: impl Into<PathBuf>,
    ) -> Result<Self, LoadError> {
        Self::load_onto(prelude.macros.clone(), root.into())
    }

    /// The plugin's root directory.
    #[must_use]
    pub fn root(&self) -> &Path {
        &self.root
    }

    fn load_onto(mut macros: MacroSet, root: PathBuf) -> Result<Self, LoadError> {
        let declarations = read_macros(&mut macros, &root)?;
        let cards = read_values(&macros, &root.join(CARDS_DIR))?;
        let tokens = read_values(&macros, &root.join(TOKENS_DIR))?;
        let rules = RulesTables {
            sba: read_table::<SbaRule>(&macros, &root.join(RULES_DIR).join("sba"))?,
            conferral: read_table::<ConferralRule>(&macros, &root.join(RULES_DIR).join("grant"))?,
            damage_result: read_table::<DamageResultRule>(
                &macros,
                &root.join(RULES_DIR).join("damage"),
            )?,
        };
        Ok(Plugin {
            root,
            macros,
            declarations,
            cards,
            tokens,
            rules,
        })
    }
}

/// Reads and registers every definition under `root/macros/`.
///
/// A definition file may invoke a meta-macro from a file that has not loaded
/// yet — file order is alphabetical happenstance — so failures are retried
/// until a pass stops making progress; only then is the first one real. The
/// same retry v1's loader runs, for the same reason.
fn read_macros(
    macros: &mut MacroSet,
    root: &Path,
) -> Result<BTreeMap<(Ident, Ident), Declaration>, LoadError> {
    let mut declarations = BTreeMap::new();
    let mut pending = Vec::new();
    for path in ron_files_recursive(&root.join(MACROS_DIR))? {
        let source = read(&path)?;
        pending.push((path, source));
    }
    while !pending.is_empty() {
        let attempted = pending.len();
        let mut failures = Vec::new();
        for (path, source) in pending {
            match macros.read_str::<MacroDef<OpaqueMetadata>>(&source) {
                Ok(read) => {
                    let definition = read.erase_metadata();
                    for &kind in &definition.kinds {
                        declarations.insert(
                            (kind, definition.name),
                            Declaration {
                                kind,
                                definition: definition.clone(),
                                path: path.clone(),
                            },
                        );
                    }
                    macros
                        .replace(&definition)
                        .map_err(|source| LoadError::Register {
                            path: path.clone(),
                            source,
                        })?;
                }
                Err(error) => failures.push((path, source, error)),
            }
        }
        if failures.len() == attempted {
            let (path, _, source) = failures.swap_remove(0);
            return Err(LoadError::Ron {
                path,
                source: Box::new(source),
            });
        }
        pending = failures
            .into_iter()
            .map(|(path, source, _)| (path, source))
            .collect();
    }
    Ok(declarations)
}

/// Reads every `.ron` file under `dir` as a `T`, keyed by file stem.
fn read_values<T: serde::de::DeserializeOwned>(
    macros: &MacroSet,
    dir: &Path,
) -> Result<BTreeMap<String, T>, LoadError> {
    let mut out = BTreeMap::new();
    for path in ron_files_recursive(dir)? {
        let source = read(&path)?;
        let value = macros
            .read_str::<T>(&source)
            .map_err(|source| LoadError::Ron {
                path: path.clone(),
                source: Box::new(source),
            })?;
        let stem = path
            .file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or_default()
            .to_string();
        out.insert(stem, value);
    }
    Ok(out)
}

/// Reads every `.ron` file under `dir` as a `Vec<T>`, concatenating them. An
/// absent directory yields an empty table.
fn read_table<T: serde::de::DeserializeOwned>(
    macros: &MacroSet,
    dir: &Path,
) -> Result<Vec<T>, LoadError> {
    let mut out = Vec::new();
    for path in ron_files_recursive(dir)? {
        let source = read(&path)?;
        let rows = macros
            .read_str::<Vec<T>>(&source)
            .map_err(|source| LoadError::Ron {
                path: path.clone(),
                source: Box::new(source),
            })?;
        out.extend(rows);
    }
    Ok(out)
}

/// Reads a plugin file to a string with path context on failure.
///
/// # Errors
/// If `path` is not readable or does not contain valid UTF-8.
pub fn read(path: &Path) -> Result<String, LoadError> {
    std::fs::read_to_string(path).map_err(|source| LoadError::Io {
        path: path.to_path_buf(),
        source,
    })
}

/// The `.ron` files under `dir` at any depth, sorted; an absent directory is
/// empty. Entries are classified by [`std::fs::DirEntry::file_type`], so a
/// directory named like a file is recursed into, not read.
///
/// # Errors
/// If `dir` or one of its subdirectories cannot be read.
pub fn ron_files_recursive(dir: &Path) -> Result<Vec<PathBuf>, LoadError> {
    if !dir.exists() {
        return Ok(vec![]);
    }
    let mut files = Vec::new();
    let mut subdirs = Vec::new();
    let entries = dir.read_dir().map_err(|source| LoadError::Io {
        path: dir.to_path_buf(),
        source,
    })?;
    for entry in entries {
        let entry = entry.map_err(|source| LoadError::Io {
            path: dir.to_path_buf(),
            source,
        })?;
        let path = entry.path();
        let file_type = entry.file_type().map_err(|source| LoadError::Io {
            path: path.clone(),
            source,
        })?;
        if file_type.is_dir() {
            subdirs.push(path);
        } else if path.extension().is_some_and(|ext| ext == "ron") && path.is_file() {
            // `is_file` follows symlinks so a linked file still loads;
            // `file_type` does not, so a linked directory cannot form a cycle.
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
