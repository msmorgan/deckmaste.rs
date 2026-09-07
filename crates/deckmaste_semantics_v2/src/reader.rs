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
use crate::rules::PredefinedToken;
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
    /// The predefined-token catalog under `tokens/` [CR#111.10], in file-stem
    /// order: each entry is the name an effect creates the token by and the
    /// characteristics it writes, qualities included [CR#111.3] — Lean
    /// `PredefinedToken`.
    pub tokens: Vec<PredefinedToken>,
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
    /// A declaration's name equals a native constructor of one of the kinds
    /// it registers under: the native reading always wins over a same-named
    /// macro (`macro_ron`'s `EnumIntercept` tries the position's own
    /// variants before the macro namespace), so the declaration would
    /// silently register and never be reachable
    /// (`semantics-spelling-lowering.md` §6's collision diagnostic is the v1
    /// precedent). Refused at load, naming both the declaration and the
    /// kind, rather than left to shadow silently.
    #[error(
        "declaration `{name}` in `{path}` collides with the native `{kind}` constructor of the \
         same name; it can never be invoked at a `{kind}` position — rename the declaration"
    )]
    NativeCollision {
        path: PathBuf,
        kind: Ident,
        name: Ident,
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
        let tokens = read_values::<CharacteristicBundle>(&macros, &root.join(TOKENS_DIR))?
            .into_iter()
            .map(|(name, token)| PredefinedToken { name, token })
            .collect();
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
    // The native dispatch set every registered kind's own Rust type carries
    // — a fresh registry, not `macros`'s (which has no public accessor for
    // it): see `NativeCollision`.
    let native_kinds = crate::ron::kinds();
    while !pending.is_empty() {
        let attempted = pending.len();
        let mut failures = Vec::new();
        for (path, source) in pending {
            match macros.read_str::<MacroDef<OpaqueMetadata>>(&source) {
                Ok(read) => {
                    let definition = read.erase_metadata();
                    // Two ways a same-named collision is harmless, so this
                    // check only refuses the third:
                    //  - IDENTITY: the body's own outermost identifier is the
                    //    declaration's own name, so reading it natively or
                    //    through the macro produces the same value. This is
                    //    how a `Subtype`/`CounterKind`/`TurnPart` declaration
                    //    attaches spelling metadata to its own native
                    //    constructor's name by design
                    //    (`a_turn_part_declaration_may_name_its_own_
                    //    constructor`; `macro_ron::MacroSet::check_cycles`
                    //    grants the same declaration the matching exemption
                    //    from being read as a self-cycle).
                    //  - BODYLESS: a meta-macro (`KeywordAction`, …) whose
                    //    `body` argument was omitted defaults to `()`
                    //    (`Default(Any, ())`), which is not identifier-led
                    //    (`body_head` reads `None`) and was never invocable
                    //    for real value in the first place — nothing is
                    //    silently shadowed because nothing meaningful was
                    //    ever reachable through it.
                    // A declaration whose body is a DIFFERENT, non-empty
                    // construction under the colliding name gets no such
                    // pass: it registers but can never be invoked
                    // (`Shuffle`'s original body, before this crate's fix —
                    // `semantics-v2-macro-bodies-keyword-actions`'s STOP 1),
                    // so the reader refuses it here.
                    let body_head = definition.body_head(macros);
                    let harmless = body_head.is_none() || body_head == Some(definition.name);
                    if !harmless {
                        for &kind in &definition.kinds {
                            if native_kinds
                                .get(kind.as_str())
                                .is_some_and(|k| k.variants().contains(&definition.name.as_str()))
                            {
                                return Err(LoadError::NativeCollision {
                                    path,
                                    kind,
                                    name: definition.name,
                                });
                            }
                        }
                    }
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

#[cfg(test)]
mod tests {
    use super::LoadError;
    use super::MACROS_DIR;
    use super::Plugin;

    /// A plugin whose only content is the given `macros/*.ron` files, under a
    /// scratch directory this call owns and removes on drop.
    struct TempPlugin(std::path::PathBuf);

    impl TempPlugin {
        fn new(name: &str, macros: &[(&str, &str)]) -> Self {
            let root = std::env::temp_dir().join(format!(
                "deckmaste_semantics_v2-reader-test-{name}-{:?}",
                std::thread::current().id()
            ));
            let dir = root.join(MACROS_DIR);
            std::fs::create_dir_all(&dir).expect("the scratch macros/ dir is creatable");
            for (file, source) in macros {
                std::fs::write(dir.join(file), source).expect("the scratch file is writable");
            }
            TempPlugin(root)
        }
    }

    impl Drop for TempPlugin {
        fn drop(&mut self) {
            let _ = std::fs::remove_dir_all(&self.0);
        }
    }

    /// A declaration whose name equals a native `Instruction` variant, and
    /// whose body does NOT reconstruct that same variant, registers but can
    /// never be invoked (native dispatch always wins) — the reader refuses
    /// it, naming both the declaration and the colliding kind.
    #[test]
    fn a_non_identity_native_collision_is_refused() {
        let plugin = TempPlugin::new(
            "collision",
            &[(
                "Collides.ron",
                r#"(
                    name: "Draw",
                    kinds: [Instruction],
                    params: [],
                    body: Shuffle(agent: You),
                )"#,
            )],
        );
        match Plugin::load(&plugin.0) {
            Err(LoadError::NativeCollision { kind, name, .. }) => {
                assert_eq!(kind.as_str(), "Instruction");
                assert_eq!(name.as_str(), "Draw");
            }
            Ok(_) => panic!("a name/native collision must be refused, not loaded"),
            Err(other) => panic!("expected a `NativeCollision`, got: {other}"),
        }
    }

    /// An IDENTITY declaration — its body's outermost identifier is its own
    /// name — is exempt: reading it natively or through the macro produces
    /// the same value, which is exactly how a `TurnPart`/`Subtype`/
    /// `CounterKind` declaration attaches spelling metadata to its own
    /// native constructor's name (`plugins_v2_declarations`'s
    /// `a_turn_part_declaration_may_name_its_own_constructor`).
    #[test]
    fn an_identity_native_collision_is_exempt() {
        let plugin = TempPlugin::new(
            "identity",
            &[(
                "Identity.ron",
                r#"(
                    name: "Draw",
                    kinds: [Instruction],
                    params: [],
                    body: Draw(amount: Lit(value: 1), agent: You),
                )"#,
            )],
        );
        Plugin::load(&plugin.0).expect("an identity declaration must load, not be refused");
    }

    /// Case is the mark (`semantics-v2.md` §11): a declaration under Lean's
    /// own camelCase name does not collide with the `PascalCase` constructor it
    /// is named for, and BOTH readings stay reachable at the same position —
    /// `draw(…)` the macro, `Draw(…)` the constructor. This is the pair that
    /// the collision refusal above used to make unreachable.
    #[test]
    fn a_camel_case_declaration_does_not_collide_with_its_constructor() {
        let plugin = TempPlugin::new(
            "camel",
            &[(
                "draw.ron",
                r#"(
                    name: "draw",
                    kinds: [Instruction],
                    params: [Amount],
                    body: Draw(Param(0), You),
                )"#,
            )],
        );
        let plugin = Plugin::load(&plugin.0).expect("a camelCase declaration must load");
        let through_macro = plugin
            .macros
            .read_str::<crate::abilities::Instruction>("draw(Lit(value: 1))")
            .expect("`draw` reads as the macro");
        let native = plugin
            .macros
            .read_str::<crate::abilities::Instruction>("Draw(amount: Lit(value: 1), agent: You)")
            .expect("`Draw` reads as the constructor");
        assert_eq!(through_macro, native);
    }

    /// A BODYLESS declaration (body `()`, a meta-macro's omitted-argument
    /// default) whose name equals a native variant is also exempt: nothing
    /// meaningful was ever reachable through it, so nothing is shadowed —
    /// exactly the shape `plugins_v2/builtin/macros/keyword_actions/
    /// {Exchange,Search,Vote}.ron` are in today (bodyless keyword actions
    /// whose declared name equals a native `Instruction` variant), which
    /// must keep loading unaltered.
    #[test]
    fn a_bodyless_native_collision_is_exempt() {
        let plugin = TempPlugin::new(
            "bodyless",
            &[(
                "Bodyless.ron",
                r#"(
                    name: "Draw",
                    kinds: [Instruction],
                    params: [],
                    body: (),
                )"#,
            )],
        );
        Plugin::load(&plugin.0).expect("a bodyless declaration must load, not be refused");
    }
}
