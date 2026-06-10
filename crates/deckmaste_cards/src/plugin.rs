//! Loading a plugin directory: macro definitions, the subtype declarations
//! invoking them, and cards.
//!
//! Directories carry a file's *role* (`macros/`, `types/`, `cards/`);
//! everything below them is organizational, and names come from file
//! contents.

use std::collections::{HashMap, HashSet};
use std::path::{Path, PathBuf};

use anyhow::Context;
use deckmaste_core::plugin::{MACROS_DIR, TYPES_DIR, card_path, token_path};
use deckmaste_core::{Card, Ident, Subtype, Token};

use crate::macros::{InsertError, MacroDef, MacroSet, macro_set};

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
        Self::load_onto(macro_set(), HashMap::new(), root.into())
    }

    /// Loads `root` with `prelude`'s macros and subtype declarations
    /// already in scope. Last plugin wins: `root`'s definitions override
    /// same-name entries from the prelude, while duplicates within `root`
    /// itself are still [`InsertError::Duplicate`](crate::macros::InsertError)
    /// errors.
    ///
    /// # Errors
    /// As [`Plugin::load`].
    pub fn load_with_prelude(prelude: &Plugin, root: impl Into<PathBuf>) -> anyhow::Result<Self> {
        Self::load_onto(
            prelude.macros.clone(),
            prelude.subtypes.clone(),
            root.into(),
        )
    }

    /// Loads `root` under the builtin convention: a sibling directory named
    /// `builtin` (that isn't `root` itself) is the prelude to every other
    /// plugin.
    ///
    /// Loads the sibling `builtin/` from disk on every call: when loading
    /// many plugins under one root, prefer [`Plugin::load`] for builtin and
    /// [`Plugin::load_with_prelude`] for the rest.
    ///
    /// # Errors
    /// As [`Plugin::load_with_prelude`]; `root` must exist.
    pub fn load_with_sibling_prelude(root: impl Into<PathBuf>) -> anyhow::Result<Self> {
        let root = root.into();
        let builtin = root.parent().unwrap_or(Path::new("")).join("builtin");
        if builtin.is_dir()
            && builtin.canonicalize()?
                != root
                    .canonicalize()
                    .with_context(|| format!(r#"resolving "{}""#, root.display()))?
        {
            let prelude = Plugin::load(&builtin)
                .with_context(|| format!(r#"loading prelude "{}""#, builtin.display()))?;
            return Self::load_with_prelude(&prelude, root);
        }
        Self::load(root)
    }

    fn load_onto(
        mut macros: MacroSet,
        mut subtypes: HashMap<Ident, Subtype>,
        root: PathBuf,
    ) -> anyhow::Result<Self> {
        // What this plugin itself defines, per kind. A name inherited from
        // the prelude may be overridden — last plugin wins — but two
        // definitions within one plugin still collide: file order here is
        // alphabetical happenstance, so "last" would be meaningless.
        let mut own: HashSet<(Ident, Ident)> = HashSet::new();

        // Nullary Subtype-kind definitions this plugin registers, expanded
        // into the subtype table once the scope settles.
        let mut declared: Vec<Ident> = Vec::new();

        // A definition file may invoke a meta-macro from a file that
        // hasn't loaded yet — file order is alphabetical happenstance — so
        // failures are retried until a pass stops making progress; only
        // then is the first one real.
        let mut pending = Vec::new();
        for path in ron_files_recursive(&root.join(MACROS_DIR))? {
            let source = read(&path)?;
            pending.push((path, source));
        }
        while !pending.is_empty() {
            let attempted = pending.len();
            let mut failures = Vec::new();
            for (path, source) in pending {
                match macros.read_str::<MacroDef>(&source) {
                    Ok(def) => {
                        for &kind in &def.kinds {
                            if !own.insert((kind, def.name)) {
                                return Err(InsertError::Duplicate {
                                    kind,
                                    name: def.name,
                                })
                                .with_context(|| format!(r#"loading "{}""#, path.display()));
                            }
                        }
                        if def.kinds.iter().any(|kind| kind.as_str() == "Subtype")
                            && nullary(&def.params)
                        {
                            declared.push(def.name);
                        }
                        macros
                            .replace(&def)
                            .with_context(|| format!(r#"loading "{}""#, path.display()))?;
                    }
                    Err(error) => failures.push((path, source, error)),
                }
            }
            if failures.len() == attempted {
                let (path, _, error) = failures.swap_remove(0);
                return Err(anyhow::Error::new(error))
                    .with_context(|| format!(r#"loading "{}""#, path.display()));
            }
            pending = failures
                .into_iter()
                .map(|(path, source, _)| (path, source))
                .collect();
        }

        // Expanding each declared subtype both validates its body and
        // fills the table — keyed by the value's printed name, which is
        // what card values carry and the lint looks up.
        for name in declared {
            let subtype: Subtype = macros
                .read_str(name.as_str())
                .with_context(|| format!("expanding subtype `{name}`"))?;
            subtypes.insert(subtype.name, subtype);
        }

        // A declaration joins the macro scope as a nullary macro whose body
        // is the file's source, verbatim; expanding it here both validates
        // it and fills the subtype table. Declarations may reference each
        // other regardless of file order, so failures are retried until a
        // pass stops making progress — only then is the first one real.
        let mut pending = Vec::new();
        for path in ron_files_recursive(&root.join(TYPES_DIR))? {
            let declaration = read(&path)?;
            pending.push((path, declaration));
        }
        while !pending.is_empty() {
            let attempted = pending.len();
            let mut failures = Vec::new();
            for (path, declaration) in pending {
                match macros.read_str::<Subtype>(&declaration) {
                    Ok(subtype) => {
                        if !own.insert(("Subtype".into(), subtype.name)) {
                            return Err(InsertError::Duplicate {
                                kind: "Subtype".into(),
                                name: subtype.name,
                            })
                            .with_context(|| format!(r#"declaring "{}""#, path.display()));
                        }
                        macros
                            .redeclare("Subtype", subtype.name, &declaration)
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
    pub fn card_path(&self, name: &str) -> PathBuf { card_path(&self.root, name) }

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

    /// The file a token of this name would live in.
    #[must_use]
    pub fn token_path(&self, name: &str) -> PathBuf { token_path(&self.root, name) }

    /// Reads and parses `tokens/<name>.ron`, with the plugin's macros in scope.
    ///
    /// # Errors
    /// If the file is missing or doesn't expand to a token.
    pub fn token(&self, name: &str) -> anyhow::Result<Token> {
        let path = self.token_path(name);
        self.macros
            .read_str(&read(&path)?)
            .with_context(|| format!(r#"parsing "{}""#, path.display()))
    }
}

pub(crate) fn read(path: &Path) -> anyhow::Result<String> {
    std::fs::read_to_string(path).with_context(|| format!(r#"reading "{}""#, path.display()))
}

/// Whether a signature takes no arguments, in either shape.
fn nullary(params: &crate::macros::Params) -> bool {
    match params {
        crate::macros::Params::Positional(types) => types.is_empty(),
        crate::macros::Params::Named(types) => types.is_empty(),
    }
}

/// The `.ron` files under `dir` at any depth, sorted; an absent directory is
/// empty. Entries are classified by [`std::fs::DirEntry::file_type`], so a
/// directory named like a file is recursed into, not read.
pub(crate) fn ron_files_recursive(dir: &Path) -> anyhow::Result<Vec<PathBuf>> {
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
    use deckmaste_core::Type;

    use super::*;

    fn plugins() -> PathBuf { Path::new(env!("CARGO_MANIFEST_DIR")).join("../../plugins") }

    #[test]
    fn sibling_prelude_brings_builtin_subtypes() {
        let wizards = Plugin::load_with_sibling_prelude(plugins().join("wizards")).unwrap();
        // Declared in builtin/types/land, visible through the prelude.
        assert!(wizards.subtypes.contains_key("Plains"));
        // wizards' own declarations load on top.
        assert!(wizards.subtypes.contains_key("Cave"));
    }

    #[test]
    fn builtin_loads_without_self_prelude() {
        let builtin = Plugin::load_with_sibling_prelude(plugins().join("builtin")).unwrap();
        assert!(builtin.subtypes.contains_key("Plains"));
    }

    /// Last plugin wins: a redeclaration overrides the prelude's version
    /// rather than erroring. wizards hits this for real — `_003` generates
    /// the full subtype set, overlapping builtin's declarations.
    #[test]
    fn redeclarations_override_the_prelude() {
        let mut prelude = Plugin::load(plugins().join("builtin")).unwrap();
        prelude.subtypes.get_mut("Plains").unwrap().types = vec![Type::Creature];
        let layered = Plugin::load_with_prelude(&prelude, plugins().join("builtin")).unwrap();
        // builtin's own LandType("Plains") declaration replaced the
        // doctored prelude entry.
        assert_eq!(layered.subtypes["Plains"].types, [Type::Land]);
    }

    /// A meta-invocation definition file loads regardless of file order:
    /// the retry loop defers it until its meta-macro registers, and the
    /// produced nullary Subtype macro fills the subtype table.
    #[test]
    fn meta_invocation_files_load_before_their_meta() {
        let root = tempfile::tempdir().unwrap();
        let macros_dir = root.path().join("macros");
        std::fs::create_dir_all(&macros_dir).unwrap();
        // `aa_` sorts before `zz_`: the invocation is attempted first.
        std::fs::write(
            macros_dir.join("aa_instance.ron"),
            r#"DeclareBear(name: "Bear")"#,
        )
        .unwrap();
        std::fs::write(
            macros_dir.join("zz_meta.ron"),
            r#"(
                name: "DeclareBear",
                kinds: [Macro],
                params: { "name": String },
                body: (
                    name: Param(name),
                    kinds: [Subtype],
                    body: Subtype(name: Param(name), types: [Creature]),
                ),
            )"#,
        )
        .unwrap();
        let plugin = Plugin::load(root.path()).unwrap();
        assert!(plugin.macros.get("Subtype", "Bear").is_some());
        assert_eq!(plugin.subtypes["Bear"].types, [Type::Creature]);
    }

    /// When no pass makes progress, the first remaining failure is real
    /// and names its file.
    #[test]
    fn an_unloadable_definition_reports_its_file() {
        let root = tempfile::tempdir().unwrap();
        let macros_dir = root.path().join("macros");
        std::fs::create_dir_all(&macros_dir).unwrap();
        std::fs::write(macros_dir.join("bad.ron"), r#"Nope(name: "X")"#).unwrap();
        let err = Plugin::load(root.path()).err().expect("expected an error");
        assert!(format!("{err:#}").contains("bad.ron"), "{err:#}");
    }

    /// Within one plugin, file order is alphabetical happenstance: two
    /// declarations of one name are an error, not "last wins".
    #[test]
    fn duplicates_within_a_plugin_error() {
        let root = tempfile::tempdir().unwrap();
        let types = root.path().join("types");
        std::fs::create_dir_all(&types).unwrap();
        std::fs::write(types.join("A.ron"), r#"Subtype(name: "X", types: [Land])"#).unwrap();
        std::fs::write(types.join("B.ron"), r#"Subtype(name: "X", types: [Land])"#).unwrap();
        let err = Plugin::load(root.path())
            .err()
            .expect("expected duplicate error");
        assert!(format!("{err:#}").contains("already defined"), "{err:#}");
    }
}
