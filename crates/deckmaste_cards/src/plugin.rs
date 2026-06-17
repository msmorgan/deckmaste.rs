//! Loading a plugin directory: macro definitions and cards.
//!
//! Directories carry a file's *role* (`macros/`, `cards/`);
//! everything below them is organizational, and names come from file
//! contents. Subtype definitions are ordinary macros, usually meta-produced.

use std::collections::HashMap;
use std::collections::HashSet;
use std::path::Path;
use std::path::PathBuf;

use anyhow::Context;
use deckmaste_core::Card;
use deckmaste_core::Counter;
use deckmaste_core::Ident;
use deckmaste_core::Subtype;
use deckmaste_core::Token;
use deckmaste_core::plugin::MACROS_DIR;
use deckmaste_core::plugin::card_path;
use deckmaste_core::plugin::token_path;

use crate::macros::InsertError;
use crate::macros::MacroDef;
use crate::macros::MacroSet;
use crate::macros::macro_set;

/// A plugin directory with its macro layer loaded and expanded.
pub struct Plugin {
    root: PathBuf,
    /// The macros in scope: definitions from `macros/`, including every
    /// subtype definition, usually produced by a meta-macro.
    pub macros: MacroSet,
    /// The subtypes defined by `macros/`, fully expanded — keyed by the
    /// value's **printed name** (what card values carry and the lint looks
    /// up), not the macro's registration ident; the two differ for names
    /// like "Time Lord"/`TimeLord`.
    pub subtypes: HashMap<Ident, Subtype>,
    /// The counter kinds defined by `macros/`, fully expanded — keyed by the
    /// counter's identity (`P1P1Counter`), which is what a `CounterRef`
    /// resolves to. The post-load `validate_counter_refs` pass checks every
    /// authored `CounterRef` against this registry.
    pub counters: HashMap<Ident, Counter>,
}

impl Plugin {
    /// # Errors
    /// If a macro definition or subtype declaration fails to read, expand,
    /// or register, or a directory isn't listable.
    pub fn load(root: impl Into<PathBuf>) -> anyhow::Result<Self> {
        Self::load_onto(macro_set(), HashMap::new(), HashMap::new(), root.into())
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
            prelude.counters.clone(),
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
        mut counters: HashMap<Ident, Counter>,
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
        // Nullary Counter-kind definitions, expanded into the counter table.
        let mut declared_counters: Vec<Ident> = Vec::new();

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
                        if def.kinds.iter().any(|kind| kind.as_str() == "Counter")
                            && nullary(&def.params)
                        {
                            declared_counters.push(def.name);
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
                return Err(error).with_context(|| format!(r#"loading "{}""#, path.display()));
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

        // Expanding each declared counter validates its body and fills the
        // table — keyed by the counter's identity (the `name` field, what a
        // `CounterRef` resolves to).
        for name in declared_counters {
            let counter: Counter = macros
                .read_str(name.as_str())
                .with_context(|| format!("expanding counter `{name}`"))?;
            counters.insert(counter.name, counter);
        }

        Ok(Self {
            root,
            macros,
            subtypes,
            counters,
        })
    }

    /// The file a card of this name would live in.
    #[must_use]
    pub fn card_path(&self, name: &str) -> PathBuf {
        card_path(&self.root, name)
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

    /// The file a token of this name would live in.
    #[must_use]
    pub fn token_path(&self, name: &str) -> PathBuf {
        token_path(&self.root, name)
    }

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

/// Reads a plugin file to a string with path context on failure. Exposed for
/// the migration pipeline (`deckmaste_migrations::graduate`), which reads
/// `.ron.todo` candidates before handing them to a [`Plugin`]'s macro reader.
///
/// # Errors
/// If `path` isn't readable or doesn't contain valid UTF-8.
pub fn read(path: &Path) -> anyhow::Result<String> {
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

    fn plugins() -> PathBuf {
        Path::new(env!("CARGO_MANIFEST_DIR")).join("../../plugins")
    }

    #[test]
    fn sibling_prelude_brings_builtin_subtypes() {
        let wizards = Plugin::load_with_sibling_prelude(plugins().join("wizards")).unwrap();
        // Declared in builtin/macros/types/land, visible through the prelude.
        assert!(wizards.subtypes.contains_key("Plains"));
        // wizards' own declarations load on top.
        assert!(wizards.subtypes.contains_key("Cave"));
    }

    #[test]
    fn builtin_loads_without_self_prelude() {
        let builtin = Plugin::load_with_sibling_prelude(plugins().join("builtin")).unwrap();
        assert!(builtin.subtypes.contains_key("Plains"));
    }

    /// [CR#122.1a]: the `+1/+1` counter is a `Counter`-kind macro in
    /// `builtin/macros/counters`, expanded into the counter registry under its
    /// rusty identity `P1P1Counter`, conferring a `Continuous` P/T boost
    /// (`AddPower`) — not an ability. Reaches WIZARDS through the prelude.
    #[test]
    fn builtin_defines_the_plus_one_counter() {
        use deckmaste_core::Modification;
        use deckmaste_core::Property;

        let builtin = Plugin::load_with_sibling_prelude(plugins().join("builtin")).unwrap();
        let counter = builtin
            .counters
            .get("P1P1Counter")
            .expect("P1P1Counter registered");
        assert_eq!(counter.name, Ident::from("P1P1Counter"));
        assert!(
            counter.confers.iter().any(|p| matches!(p,
                Property::Continuous { changes, .. }
                    if changes.iter().any(|m| matches!(m, Modification::AddPower(_))))),
            "confers a Continuous AddPower boost; got {:?}",
            counter.confers
        );
        // Reaches the wizards corpus via the sibling prelude.
        let wizards = Plugin::load_with_sibling_prelude(plugins().join("wizards")).unwrap();
        assert!(wizards.counters.contains_key("P1P1Counter"));
    }

    /// [CR#704.5q]: the `-1/-1` counter confers both the negative `Continuous`
    /// boost AND the annihilation as a `StateBased` SBA (a `Sequence` of two
    /// `RemoveCounters`). Exercises the richer confer RON (`Is`/`HasCounter`,
    /// bare-embedded `RemoveCounters`, `CounterCount`).
    #[test]
    fn builtin_minus_one_counter_carries_annihilation_sba() {
        use deckmaste_core::Property;

        let builtin = Plugin::load_with_sibling_prelude(plugins().join("builtin")).unwrap();
        let counter = builtin
            .counters
            .get("M1M1Counter")
            .expect("M1M1Counter registered");
        assert!(
            counter
                .confers
                .iter()
                .any(|p| matches!(p, Property::StateBased { .. })),
            "confers a StateBased annihilation SBA; got {:?}",
            counter.confers
        );
    }

    /// The attachment-rule subtypes (Aura/Equipment/Fortification) carry their
    /// `Innate` `confers:` ([CR#704.5m] graveyard SBA; host-type
    /// `Cant(Attach)`) even in the WIZARDS corpus — the defs live in
    /// `builtin`, and the generator no longer emits confers-LESS wizards
    /// stubs for them, so under "last plugin wins" builtin's
    /// confers-bearing def is the one in scope. Regression guard: a
    /// confers-less wizards stub would silently strip these.
    #[test]
    fn wizards_attachment_subtypes_carry_innate_confers() {
        use deckmaste_core::Property;

        let wizards = Plugin::load_with_sibling_prelude(plugins().join("wizards")).unwrap();
        for name in ["Aura", "Equipment", "Fortification"] {
            let subtype = wizards
                .subtypes
                .get(name)
                .unwrap_or_else(|| panic!("wizards corpus knows the {name} subtype"));
            let has_innate = subtype
                .confers
                .iter()
                .any(|p| matches!(p, Property::Ability(a) if a.is_innate()));
            assert!(
                has_innate,
                "{name} subtype confers an Innate attachment rule in the wizards corpus; \
                 got confers: {:?}",
                subtype.confers
            );
        }
    }

    /// Last plugin wins: a redeclaration overrides the prelude's version
    /// rather than erroring. wizards hits this for real — it generates
    /// the full subtype set, overlapping builtin's declarations.
    #[test]
    fn redeclarations_override_the_prelude() {
        let mut prelude = Plugin::load(plugins().join("builtin")).unwrap();
        prelude.subtypes.get_mut("Plains").unwrap().types = vec![Type::Creature];
        let layered = Plugin::load_with_prelude(&prelude, plugins().join("builtin")).unwrap();
        // builtin's own Plains definition replaced the doctored prelude entry.
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
    /// definitions of one name are an error, not "last wins".
    #[test]
    fn duplicates_within_a_plugin_error() {
        let root = tempfile::tempdir().unwrap();
        let macros_dir = root.path().join("macros");
        std::fs::create_dir_all(&macros_dir).unwrap();
        let def = r#"(name: "X", kinds: [Subtype], body: Subtype(name: "X", types: [Land]))"#;
        std::fs::write(macros_dir.join("A.ron"), def).unwrap();
        std::fs::write(macros_dir.join("B.ron"), def).unwrap();
        let err = Plugin::load(root.path())
            .err()
            .expect("expected duplicate error");
        assert!(format!("{err:#}").contains("already defined"), "{err:#}");
    }
}
