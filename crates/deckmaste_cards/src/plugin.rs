//! Loading a plugin directory: macro definitions and cards.
//!
//! Directories carry a file's *role* (`macros/`, `cards/`);
//! everything below them is organizational, and names come from file
//! contents. Subtype definitions are ordinary macros, usually meta-produced.

use std::collections::HashMap;
use std::collections::HashSet;
use std::fmt;
use std::fmt::Write as _;
use std::path::Path;
use std::path::PathBuf;

use anyhow::Context;
use deckmaste_core::Card;
use deckmaste_core::Counter;
use deckmaste_core::DesignationDecl;
use deckmaste_core::Ident;
use deckmaste_core::KeywordDecl;
use deckmaste_core::ParamShape;
use deckmaste_core::Subtype;
use deckmaste_core::Token;
use deckmaste_core::plugin::CARDS_DIR;
use deckmaste_core::plugin::MACROS_DIR;
use deckmaste_core::plugin::RULES_DIR;
use deckmaste_core::plugin::TOKENS_DIR;
use deckmaste_core::plugin::card_path;
use deckmaste_core::plugin::is_todo_source;
use deckmaste_core::plugin::token_path;

use crate::elaborate;
use crate::elaborate::ElabError;
use crate::elaborate::Registries;
use crate::elaborate::Stage;
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
    /// The designations declared by `macros/` (`DesignationDecl`-kind,
    /// nullary), keyed by the designation's identity. The elaborator's
    /// designation-scope check consults this registry first, falling back to
    /// the emitted curated table for undeclared names (an open vocabulary).
    pub designations: HashMap<Ident, DesignationDecl>,
    /// The keyword registry ([CR#702]): one row per `KeywordAbility`-kind
    /// macro, its [`ParamShape`] DERIVED from the macro's typed parameter
    /// signature (`params: [Cost]` ⇒ `Costed`) — the macro file is the
    /// registry data, so the two can't drift. A `KeywordAbility` macro whose
    /// signature fits no shape fails the load: keyword one-liners take typed
    /// args from the closed shape vocabulary.
    pub keywords: HashMap<Ident, KeywordDecl>,
    /// Rules-defined state-based actions loaded from `rules/sba/`. Evaluated
    /// globally by the engine's SBA sweep ([CR#704.3]). See
    /// `deckmaste_core::SbaRule`.
    pub sba_rules: Vec<deckmaste_core::SbaRule>,
    /// The staged elaboration rollout this load ran under
    /// ([`Stage::for_root`]) — `Stage::Deny` for every plugin except a
    /// directory literally named `wizards`.
    pub elab_stage: Stage,
    /// The `Stage::Warn` counted report: empty for a `Stage::Deny` plugin
    /// (any finding there fails the load outright, per
    /// `elaborate_finished`, so none survive to be reported) or a totally
    /// clean `Stage::Warn` plugin. See `cargo xtask elaborate`.
    pub elab_report: ElabReport,
    /// The `Stage::Warn` names skipped by [`Plugin::card`]/[`Plugin::token`]
    /// — every name in `elab_report.findings`, kept alongside it as a fast
    /// lookup set.
    elab_skip: HashSet<String>,
}

/// One card/token that failed load-time elaboration ([`Plugin::elab_report`]).
#[derive(Debug, Clone)]
pub struct ElabFinding {
    pub path: PathBuf,
    /// The card/token's own name — what a lookup by [`Plugin::card`] /
    /// [`Plugin::token`] uses, not necessarily the file's stem (a card name
    /// with a filesystem-illegal character escapes in its file name; a
    /// token has no name field of its own, so its file stem stands in).
    pub name: String,
    pub errors: Vec<ElabError>,
}

impl fmt::Display for ElabFinding {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for (i, error) in self.errors.iter().enumerate() {
            if i > 0 {
                writeln!(f)?;
            }
            write!(f, "{error} at {} ({})", self.path.display(), self.name)?;
        }
        Ok(())
    }
}

/// The load-time elaboration walk's counted report ([`Plugin::elab_report`]):
/// how many finished cards/tokens were checked, and which ones failed.
#[derive(Debug, Clone, Default)]
pub struct ElabReport {
    /// Every finished (non-todo) card/token file the walk parsed.
    pub checked: usize,
    /// The subset that failed elaboration — empty for a `Stage::Deny`
    /// plugin, whose load fails outright instead of accumulating these.
    pub findings: Vec<ElabFinding>,
}

impl Plugin {
    /// # Errors
    /// If a macro definition or subtype declaration fails to read, expand,
    /// or register, or a directory isn't listable.
    pub fn load(root: impl Into<PathBuf>) -> anyhow::Result<Self> {
        Self::load_onto(macro_set(), Inherited::default(), root.into())
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
            Inherited {
                subtypes: prelude.subtypes.clone(),
                counters: prelude.counters.clone(),
                designations: prelude.designations.clone(),
                keywords: prelude.keywords.clone(),
            },
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
        inherited: Inherited,
        root: PathBuf,
    ) -> anyhow::Result<Self> {
        let Inherited {
            mut subtypes,
            mut counters,
            mut designations,
            mut keywords,
        } = inherited;
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
        // Nullary DesignationDecl-kind definitions, expanded into the
        // designation table.
        let mut declared_designations: Vec<Ident> = Vec::new();

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
                        if def
                            .kinds
                            .iter()
                            .any(|kind| kind.as_str() == "DesignationDecl")
                            && nullary(&def.params)
                        {
                            declared_designations.push(def.name);
                        }
                        // Every KeywordAbility-kind macro is a keyword
                        // registry row; its ParamShape derives from the typed
                        // parameter signature ([CR#702] one-liners).
                        if def
                            .kinds
                            .iter()
                            .any(|kind| kind.as_str() == "KeywordAbility")
                        {
                            let shape = keyword_shape(&def.params).ok_or_else(|| {
                                anyhow::anyhow!(
                                    "keyword {:?} declares a parameter signature fitting no ParamShape (the closed keyword-arg vocabulary)",
                                    def.name.as_str()
                                )
                            })
                            .with_context(|| format!(r#"loading "{}""#, path.display()))?;
                            keywords.insert(
                                def.name,
                                KeywordDecl {
                                    name: def.name,
                                    shape,
                                },
                            );
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

        // Expanding each declared designation validates its body and fills
        // the table — keyed by the decl's own name.
        for name in declared_designations {
            let decl: DesignationDecl = macros
                .read_str(name.as_str())
                .with_context(|| format!("expanding designation `{name}`"))?;
            designations.insert(decl.name, decl);
        }

        let sba_rules = load_sba_rules(&root, &macros)?;

        // The load-time elaboration gate ([[cards-elab-load-gate]]): every
        // finished (non-todo) card/token this plugin owns is parsed and
        // walked NOW, so a live engine can never resolve a card that would
        // panic on an unbound reference. The stage is a property of THIS
        // load call (`root`'s own name), not a global switch — see
        // `Stage::for_root`.
        let elab_stage = Stage::for_root(&root);
        let registries = Registries {
            subtypes: &subtypes,
            counters: &counters,
            designations: &designations,
            keywords: &keywords,
        };
        let elab_report = elaborate_finished(&root, &macros, &registries, elab_stage)?;
        let elab_skip = elab_report
            .findings
            .iter()
            .map(|f| f.name.clone())
            .collect();

        Ok(Self {
            root,
            macros,
            subtypes,
            counters,
            designations,
            keywords,
            sba_rules,
            elab_stage,
            elab_report,
            elab_skip,
        })
    }

    /// The elaboration registries over this plugin's loaded tables — the
    /// one construction point consumers (validate, tests, xtask) share.
    #[must_use]
    pub fn registries(&self) -> Registries<'_> {
        Registries {
            subtypes: &self.subtypes,
            counters: &self.counters,
            designations: &self.designations,
            keywords: &self.keywords,
        }
    }

    /// The file a card of this name would live in.
    #[must_use]
    pub fn card_path(&self, name: &str) -> PathBuf {
        card_path(&self.root, name)
    }

    /// Reads and parses `cards/<name>.ron`, with the plugin's macros in scope.
    ///
    /// Under [`Stage::Warn`] (the `wizards` corpus), a name that failed the
    /// load-time elaboration walk is refused here too — "the offending
    /// cards skipped": the engine can still never resolve it, even though
    /// the plugin's own load succeeded.
    ///
    /// # Errors
    /// If the file is missing, doesn't expand to a card, or (under
    /// [`Stage::Warn`]) named a card the load-time walk already flagged.
    pub fn card(&self, name: &str) -> anyhow::Result<Card> {
        self.reject_skipped(name)?;
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
    /// Under [`Stage::Warn`], a name that failed the load-time elaboration
    /// walk is refused here too (see [`Plugin::card`]).
    ///
    /// # Errors
    /// If the file is missing, doesn't expand to a token, or (under
    /// [`Stage::Warn`]) named a token the load-time walk already flagged.
    pub fn token(&self, name: &str) -> anyhow::Result<Token> {
        self.reject_skipped(name)?;
        let path = self.token_path(name);
        self.macros
            .read_str(&read(&path)?)
            .with_context(|| format!(r#"parsing "{}""#, path.display()))
    }

    /// The [`Stage::Warn`] "offending cards skipped" check shared by
    /// [`Plugin::card`]/[`Plugin::token`]: a no-op for [`Stage::Deny`]
    /// (`elab_skip` is always empty there — any finding already failed the
    /// whole load).
    fn reject_skipped(&self, name: &str) -> anyhow::Result<()> {
        if self.elab_skip.contains(name) {
            let finding = self
                .elab_report
                .findings
                .iter()
                .find(|f| f.name == name)
                .expect("elab_skip and elab_report.findings name the same set");
            anyhow::bail!(
                "{name:?} failed load-time elaboration and was skipped under Stage::Warn:\n{finding}"
            );
        }
        Ok(())
    }
}

/// The registry tables a prelude hands down to a dependent plugin's load —
/// last plugin wins per name, exactly like the macro scope.
#[derive(Default)]
struct Inherited {
    subtypes: HashMap<Ident, Subtype>,
    counters: HashMap<Ident, Counter>,
    designations: HashMap<Ident, DesignationDecl>,
    keywords: HashMap<Ident, KeywordDecl>,
}

/// The [`ParamShape`] a keyword macro's typed parameter signature spells
/// ([CR#702] keyword one-liners): nothing, a `Count`, a `Cost`, `Count` then
/// `Cost` (suspend/awaken), a `Filter` (landwalk, hexproof-from), `Filter`
/// then `Cost` (splice), or a `String` name (partner-with). `None` = the
/// signature fits no shape — a load error, keeping the arg vocabulary
/// closed. Named single-param signatures (`{"from": Default(Filter, Any)}`)
/// map by their one value type.
fn keyword_shape(params: &crate::macros::Params) -> Option<ParamShape> {
    use crate::macros::Params;
    let names: Vec<&str> = match params {
        Params::Positional(list) => list.iter().map(|p| p.name.as_str()).collect(),
        Params::Named(map) => {
            let mut names: Vec<&str> = map.values().map(|p| p.name.as_str()).collect();
            // A named signature is order-free; canonicalize before matching.
            names.sort_unstable();
            names
        }
    };
    Some(match names.as_slice() {
        [] => ParamShape::None,
        ["Count"] => ParamShape::Counted,
        ["Cost"] => ParamShape::Costed,
        ["Count", "Cost"] | ["Cost", "Count"] => ParamShape::CountedCost,
        ["Filter"] => ParamShape::Predicated,
        ["Filter", "Cost"] | ["Cost", "Filter"] => ParamShape::PredicatedCosted,
        ["String"] => ParamShape::Named,
        _ => return Option::None,
    })
}

/// Loads all `Vec<SbaRule>` files under `root/rules/sba/`, concatenating them
/// into a single list. An absent directory yields an empty vec.
///
/// # Errors
/// If a file is unreadable or doesn't parse as `Vec<SbaRule>`.
fn load_sba_rules(root: &Path, macros: &MacroSet) -> anyhow::Result<Vec<deckmaste_core::SbaRule>> {
    let dir = root.join(RULES_DIR).join("sba");
    let mut rules = Vec::new();
    for path in ron_files_recursive(&dir)? {
        let source = read(&path)?;
        let file: Vec<deckmaste_core::SbaRule> = macros
            .read_str(&source)
            .with_context(|| format!(r#"loading SBA rules from "{}""#, path.display()))?;
        rules.extend(file);
    }
    Ok(rules)
}

/// The load-time elaboration walk: every finished `cards/**/*.ron` and
/// `tokens/**/*.ron` under `root`, parsed with `macros` and elaborated
/// against `registries` — the same shape as `validate::validate_plugin`'s
/// card/token loop, run eagerly at load time instead of behind
/// `cargo xtask validate`.
///
/// A file that doesn't even PARSE is always a hard error, regardless of
/// stage — the staged rollout eases in the NEW elaboration checks, not
/// parsing, which every other code path already treats as fatal.
///
/// # Errors
/// If a file isn't readable or doesn't parse; under [`Stage::Deny`], also if
/// any file fails elaboration (one aggregated error naming every finding).
fn elaborate_finished(
    root: &Path,
    macros: &MacroSet,
    registries: &Registries,
    stage: Stage,
) -> anyhow::Result<ElabReport> {
    let mut report = ElabReport::default();
    for path in ron_files_recursive(&root.join(CARDS_DIR))? {
        let source = read(&path)?;
        if is_todo_source(&source) {
            continue;
        }
        let card: Card = macros
            .read_str(&source)
            .with_context(|| format!(r#"parsing "{}""#, path.display()))?;
        report.checked += 1;
        if let Err(errors) = elaborate::elaborate(&card, registries) {
            report.findings.push(ElabFinding {
                name: card_lookup_name(&card).to_owned(),
                path,
                errors,
            });
        }
    }
    for path in ron_files_recursive(&root.join(TOKENS_DIR))? {
        let source = read(&path)?;
        if is_todo_source(&source) {
            continue;
        }
        let token: Token = macros
            .read_str(&source)
            .with_context(|| format!(r#"parsing "{}""#, path.display()))?;
        report.checked += 1;
        if let Err(errors) = elaborate::elaborate_token(&token, registries) {
            // A token has no name field of its own — the file stem IS its
            // lookup name (`token_path`/`token_file` don't escape it).
            let name = path
                .file_stem()
                .and_then(|s| s.to_str())
                .unwrap_or_default()
                .to_owned();
            report.findings.push(ElabFinding { path, name, errors });
        }
    }
    if stage == Stage::Deny && !report.findings.is_empty() {
        let mut message = format!(
            "{} elaboration error(s) in \"{}\":\n",
            report
                .findings
                .iter()
                .map(|f| f.errors.len())
                .sum::<usize>(),
            root.display()
        );
        for finding in &report.findings {
            let _ = writeln!(message, "  {finding}");
        }
        anyhow::bail!(message);
    }
    Ok(report)
}

/// The name [`Plugin::card`] would use to find `card`'s own file — the
/// authoritative source (not the file path, which escapes filesystem-illegal
/// characters): a face's `name` field, or a two-faced card's front face.
/// Exposed crate-wide for `crate::lock`, which keys `cards.elab.lock` by
/// this same name.
pub(crate) fn card_lookup_name(card: &Card) -> &str {
    match card {
        Card::Normal(face) => &face.name,
        Card::TwoFaced { front, .. } => &front.name,
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
    use crate::elaborate::Code;

    fn plugins() -> PathBuf {
        Path::new(env!("CARGO_MANIFEST_DIR")).join("../../plugins")
    }

    #[test]
    fn builtin_loads_four_sba_rules() {
        let plugin = Plugin::load(
            std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../plugins/builtin"),
        )
        .unwrap();
        assert_eq!(
            plugin.sba_rules.len(),
            4,
            "toughness-0, loyalty-0, battle-defense-0, lethal-damage"
        );
        // Every row puts the object into a graveyard.
        assert!(
            plugin
                .sba_rules
                .iter()
                .all(|r| matches!(&r.then, deckmaste_core::Effect::Act(_)))
        );
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
    /// (`Power(Up(...))`) — not an ability. Reaches WIZARDS through the
    /// prelude.
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
                    if changes.iter().any(|m| matches!(m, Modification::Power(_))))),
            "confers a Continuous Power(Up) boost; got {:?}",
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

    fn bad_card_source() -> &'static str {
        // The E-BIND-TARGET reject fixture's shape: a 2nd target read where
        // only one was announced ([CR#115.3,601.2c]).
        r#"Normal(
            name: "Deliberately Bad Card",
            types: [Sorcery],
            abilities: [
                Spell(effect: Targeted(targets: [AnyTarget], effect: DealDamage(Target(1), 1))),
            ],
        )"#
    }

    /// [[cards-elab-load-gate]] `Stage::Deny`: a plugin with one malformed
    /// card fails the WHOLE load, naming the code, the file, and the card.
    #[test]
    fn deny_stage_load_fails_naming_the_code_path_and_card() {
        let builtin = Plugin::load(plugins().join("builtin")).unwrap();
        let root = tempfile::tempdir().unwrap();
        // Not named "wizards" (a random tempdir leaf never is) — Deny by
        // default, matching every hand-authored plugin and any ad hoc test
        // directory.
        let cards_dir = root.path().join("cards");
        std::fs::create_dir_all(&cards_dir).unwrap();
        std::fs::write(
            cards_dir.join("Deliberately Bad Card.ron"),
            bad_card_source(),
        )
        .unwrap();

        let err = Plugin::load_with_prelude(&builtin, root.path())
            .err()
            .expect("a malformed card must fail the load");
        let message = format!("{err:#}");
        assert!(message.contains("E-BIND-TARGET"), "{message}");
        assert!(message.contains("Deliberately Bad Card"), "{message}");
        assert!(message.contains("Deliberately Bad Card.ron"), "{message}");
    }

    /// [[cards-elab-load-gate]] `Stage::Warn`: a plugin directory literally
    /// named `wizards` still loads with a malformed card present — the
    /// counted report records it, and it's skipped (never fetchable via
    /// `Plugin::card`) — while an unrelated clean card in the same plugin
    /// loads normally.
    #[test]
    fn warn_stage_load_succeeds_and_skips_only_the_bad_card() {
        let builtin = Plugin::load(plugins().join("builtin")).unwrap();
        let root = tempfile::tempdir().unwrap();
        let wizards_dir = root.path().join("wizards");
        let cards_dir = wizards_dir.join("cards");
        std::fs::create_dir_all(&cards_dir).unwrap();
        std::fs::write(
            cards_dir.join("Deliberately Bad Card.ron"),
            bad_card_source(),
        )
        .unwrap();
        std::fs::write(
            cards_dir.join("Fine Bear.ron"),
            r#"Normal(name: "Fine Bear", types: [Creature], power: 2, toughness: 2)"#,
        )
        .unwrap();

        let plugin = Plugin::load_with_prelude(&builtin, &wizards_dir)
            .expect("Stage::Warn never fails the load");
        assert_eq!(plugin.elab_stage, Stage::Warn);
        assert_eq!(plugin.elab_report.checked, 2);
        assert_eq!(plugin.elab_report.findings.len(), 1);
        assert_eq!(plugin.elab_report.findings[0].name, "Deliberately Bad Card");
        assert_eq!(
            plugin.elab_report.findings[0].errors[0].code,
            Code::BindTarget
        );

        // Skipped: the engine can still never resolve it.
        let skipped = plugin.card("Deliberately Bad Card").unwrap_err();
        assert!(format!("{skipped:#}").contains("E-BIND-TARGET"));
        // An unrelated clean card in the same warn-stage plugin is
        // unaffected.
        assert!(plugin.card("Fine Bear").is_ok());
    }
}
