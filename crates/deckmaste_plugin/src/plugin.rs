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
use deckmaste_core::Counter;
use deckmaste_core::DesignationDecl;
use deckmaste_core::Ident;
use deckmaste_core::KeywordDecl;
use deckmaste_core::Subtype;
use deckmaste_core::TypeDef;
use deckmaste_core::plugin::MACROS_DIR;
use deckmaste_core::plugin::RULES_DIR;
use deckmaste_core::plugin::TOKENS_DIR;
use deckmaste_core::plugin::card_path;
use deckmaste_core::plugin::token_path;
use deckmaste_lowering::Lower;

use crate::loaded::CardResolution;
use crate::loaded::LoadedCard;
use crate::loaded::LoadedToken;
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
    /// Semantic provenance for everything this plugin's registries confer,
    /// plus every token it defines and every predefined token ([CR#111.10]).
    ///
    /// Built at load because that is the only moment the semantic halves of
    /// the registry values exist: `subtypes`/`types`/`counters` below keep the
    /// LOWERED value, and the semantic one is dropped right after `lower`. A
    /// conferred ability appears on no card, and a token's abilities reach an
    /// object through a `CardId` past the end of the card companion table, so
    /// this is their only indexing opportunity.
    pub provenance: crate::provenance::ProvenanceIndex,
    /// Token files this load could not read or expand, with the reason. Each
    /// is a hole in `provenance`: the token still mints, but its abilities
    /// render as `[unrendered: …]`. Loading tokens is deliberately lenient
    /// (one malformed file must not fail the whole plugin), so this is where
    /// the leniency stays accountable — empty for a clean load.
    pub skipped_tokens: Vec<(PathBuf, String)>,
    pub subtypes: HashMap<Ident, Subtype>,
    /// The card types defined by `macros/`, fully expanded — keyed by the
    /// value's **printed name** ([CR#300.1]), mirroring `subtypes` exactly.
    /// The ten canonical types (`Artifact`..`Sorcery`) are declared as
    /// builtin `kinds: [TypeDef]` macros under `macros/cardtype/`.
    pub types: HashMap<Ident, TypeDef>,
    /// The counter kinds defined by `macros/`, fully expanded — keyed by the
    /// counter's identity (`P1P1Counter`), which is what a `CounterRef`
    /// resolves to. No load-time pass validates semantic `CounterRef`s against
    /// this registry yet — only the Idris emitter flags unknown counter refs
    /// yet.
    pub counters: HashMap<Ident, Counter>,
    /// The designations declared by `macros/` (`DesignationDecl`-kind,
    /// nullary), keyed by the designation's identity. The designation-scope
    /// check consults this registry first, falling back to the curated core
    /// vocabulary for undeclared names (an open vocabulary).
    pub designations: HashMap<Ident, DesignationDecl>,
    /// The keyword registry ([CR#702]): one row per `KeywordAbility`-kind
    /// macro, its [`deckmaste_core::ParamShape`] DERIVED from the macro's typed
    /// parameter signature (`params: [Cost]` ⇒ `Costed`) — the macro file
    /// is the registry data, so the two can't drift. A `KeywordAbility`
    /// macro whose signature fits no shape fails the load: keyword
    /// one-liners take typed args from the closed shape vocabulary.
    pub keywords: HashMap<Ident, KeywordDecl>,
    /// The keyword-action verb registry ([CR#701]): one row per
    /// `KeywordAction`-kind macro (`Destroy`, `Mill`, `Draw`, `Scry`, …),
    /// recording its name and typed parameter signature. The macro file IS the
    /// registry data (name + param shape + — held by reference in
    /// [`macros`](Self::macros) — its canonical expansion body), the
    /// coherence-by-construction source a later would-lane matcher indexes. A
    /// side-table cloned from the `keyword`-registry flow; no card position
    /// reads it yet.
    pub verbs: HashMap<Ident, VerbDecl>,
    /// Rules-defined state-based actions loaded from `rules/sba/`. Evaluated
    /// globally by the engine's SBA sweep ([CR#704.3]). See
    /// `deckmaste_core::SbaRule`.
    pub sba_rules: Vec<deckmaste_core::SbaRule>,
    /// Rules-defined conferrals loaded from `rules/grant/`: predicate-scoped
    /// grants applied globally, alongside a card's own printed/conferred
    /// abilities. See `deckmaste_core::ConferralRule`.
    pub conferral_rules: Vec<deckmaste_core::ConferralRule>,
    /// Rules-defined damage results loaded from `rules/damage/`: when damage is
    /// dealt to a permanent matching the rule's `recipient`, that many counters
    /// of `remove` are taken off it ([CR#120.3c] planeswalker loyalty). See
    /// `deckmaste_core::DamageResultRule`.
    pub damage_result_rules: Vec<deckmaste_core::DamageResultRule>,
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
                provenance: prelude.provenance.clone(),
                subtypes: prelude.subtypes.clone(),
                types: prelude.types.clone(),
                counters: prelude.counters.clone(),
                designations: prelude.designations.clone(),
                keywords: prelude.keywords.clone(),
                verbs: prelude.verbs.clone(),
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
            mut provenance,
            mut subtypes,
            mut types,
            mut counters,
            mut designations,
            mut keywords,
            mut verbs,
        } = inherited;
        // What this plugin itself defines, per kind. A name inherited from
        // the prelude may be overridden — last plugin wins — but two
        // definitions within one plugin still collide: file order here is
        // alphabetical happenstance, so "last" would be meaningless.
        let mut own: HashSet<(Ident, Ident)> = HashSet::new();

        // Nullary Subtype-kind definitions this plugin registers, expanded
        // into the subtype table once the scope settles.
        let mut declared: Vec<Ident> = Vec::new();
        // Nullary TypeDef-kind definitions, expanded into the types table.
        let mut declared_types: Vec<Ident> = Vec::new();
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
                        if def.kinds.iter().any(|kind| kind.as_str() == "TypeDef")
                            && nullary(&def.params)
                        {
                            declared_types.push(def.name);
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
                            .with_context(|| format!(r#"loading "{}""#, path.display()))?
                            .lower();
                            keywords.insert(
                                def.name,
                                KeywordDecl {
                                    name: def.name,
                                    shape,
                                },
                            );
                        }
                        // Every KeywordAction-kind macro is a verb registry
                        // row ([CR#701]): its name plus typed parameter
                        // signature. Cloning the keyword flow above; the
                        // canonical body stays in `macros` (recoverable by
                        // expanding the invocation).
                        if def
                            .kinds
                            .iter()
                            .any(|kind| kind.as_str() == "KeywordAction")
                        {
                            verbs.insert(
                                def.name,
                                VerbDecl {
                                    name: def.name,
                                    params: verb_params(&def.params),
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
            let subtype: deckmaste_semantics::Subtype = macros
                .read_str(name.as_str())
                .with_context(|| format!("expanding subtype `{name}`"))?;
            provenance.insert_subtype(&subtype);
            let subtype = subtype.lower();
            subtypes.insert(subtype.name, subtype);
        }

        // Expanding each declared type both validates its body and fills the
        // table — keyed by the value's printed name, mirroring the subtype
        // expansion above exactly.
        for name in declared_types {
            let type_def: deckmaste_semantics::TypeDef = macros
                .read_str(name.as_str())
                .with_context(|| format!("expanding type `{name}`"))?;
            provenance.insert_type_def(&type_def);
            let type_def = type_def.lower();
            types.insert(type_def.name, type_def);
        }

        expand_counters(&macros, declared_counters, &mut provenance, &mut counters)?;
        expand_designations(&macros, declared_designations, &mut designations)?;
        let skipped_tokens = index_tokens(&root, &macros, &mut provenance)?;

        let sba_rules = load_sba_rules(&root, &macros)?;
        let conferral_rules = load_conferral_rules(&root, &macros)?;
        let damage_result_rules = load_damage_result_rules(&root, &macros)?;

        Ok(Self {
            root,
            macros,
            provenance,
            skipped_tokens,
            subtypes,
            types,
            counters,
            designations,
            keywords,
            verbs,
            sba_rules,
            conferral_rules,
            damage_result_rules,
        })
    }

    /// The file a card of this name would live in.
    #[must_use]
    pub fn card_path(&self, name: &str) -> PathBuf {
        card_path(&self.root, name)
    }

    /// Reads and parses `cards/<name>.ron` as a semantic card, with the
    /// plugin's macros in scope, and lowers it to its engine image.
    ///
    /// This and its three siblings ([`Plugin::card_from_str`],
    /// [`Plugin::token`], [`Plugin::token_from_str`]) are the one restricted
    /// read API (spec §4): nothing else may `read_str` a typed card. The
    /// path-taking entries read the file and delegate, so there is a single
    /// read path, not two that drift.
    ///
    /// ONE further entry reads a typed card, and only because it reads this
    /// one first: [`Plugin::rendering_card_from_str`] runs the restricted read
    /// and propagates its error before re-reading free, so it cannot admit a
    /// spelling this API rejects. It is confined to the legacy renderer, and
    /// `tests/read_api_gate.rs` enforces that confinement by name.
    ///
    /// That gate holds the whole line mechanically. It parses the workspace
    /// and rejects a turbofish or annotated read of `Card`, `CardFace` or
    /// `Token` — wrapped, inside a macro body, or behind an aliased import —
    /// outside this API, and separately rejects a rendering-view call outside
    /// `deckmaste_legacy_render`. It is a strong check, not a total one; that
    /// test's module doc states exactly what it does and does not reach, and
    /// is the thing to read before assuming a bypass is impossible.
    ///
    /// # Errors
    /// If the file is missing or doesn't expand to a card.
    pub fn card(&self, name: &str) -> anyhow::Result<LoadedCard> {
        let path = self.card_path(name);
        self.card_from_str(&read(&path)?)
            .with_context(|| format!(r#"parsing "{}""#, path.display()))
    }

    /// Reads a semantic card from SOURCE TEXT rather than from the plugin's
    /// `cards/` directory — the entry test fixtures and the migration
    /// graduation path use. Same restriction, same erasure point.
    ///
    /// # Errors
    /// If the source doesn't expand to a card.
    pub fn card_from_str(&self, source: &str) -> anyhow::Result<LoadedCard> {
        let semantic: deckmaste_semantics::Card = self.macros.read_str_restricted(source)?;
        // Lowering owns the per-card diagnostic (ADR law 12): `lower_card`
        // installs the card as the resolver's diagnostic context, so an R1/R2
        // refusal comes back naming this card and the antecedents that
        // collided, instead of an anonymous panic from inside the tree walk.
        let core = deckmaste_lowering::lower_card(semantic.clone())
            .map_err(|diagnostic| anyhow::anyhow!("lowering {diagnostic}"))?;
        validate_card_regions(&core)?;
        Ok(LoadedCard { semantic, core })
    }

    /// The RESOLVER half of the certifier/resolver differential
    /// (`semantics-spelling-lowering.md` §17): read a card through the same
    /// restricted entry as [`Plugin::card_from_str`], but hand back lowering's
    /// VERDICT as data instead of propagating a refusal.
    ///
    /// A `lowered` of `Err` is a card whose text the resolver refuses (an
    /// ambiguous or unbound anaphor); the differential gate pairs that verdict
    /// against the Idris mirror's certification of the same card. A plain
    /// `Err` return is still a read failure — a card that does not parse has
    /// no resolver verdict to compare. The card's printed name comes back with
    /// the verdict so a caller never needs a second read to identify it.
    ///
    /// This is not a second read path: it runs the identical restricted read
    /// and the identical `lower_card`, and differs only in whether a refusal
    /// is returned or raised.
    ///
    /// # Errors
    /// If the source doesn't expand to a card.
    pub fn card_resolution_from_str(&self, source: &str) -> anyhow::Result<CardResolution> {
        let semantic: deckmaste_semantics::Card = self.macros.read_str_restricted(source)?;
        let name = crate::idris_emit::card_display_name(&semantic).to_owned();
        Ok(CardResolution {
            name,
            lowered: deckmaste_lowering::lower_card(semantic),
        })
    }

    /// A semantic card WITHOUT identity-macro invocation provenance: the value
    /// shape a card had before the ban routed mirrored spellings through
    /// identity macros (spec §5, "wrapper interaction at remembering kinds").
    ///
    /// The one caller is `deckmaste_legacy_render`, which matches semantic
    /// terms structurally at dozens of nested positions AND reads a real
    /// macro's remembered `template:` — so it needs the wrappers that carry a
    /// template and not the ones that carry nothing but the value's own
    /// variant name. A FREE read is exactly that value: native variants win,
    /// so an identity macro is never consulted, while a real macro still
    /// expands and is still remembered.
    ///
    /// Not a hole in the ban: the restricted read runs first and its error is
    /// what propagates, so nothing reaches the free read that the author
    /// surface would have rejected. It retires with the legacy renderer.
    ///
    /// **Confined on purpose, and enforced.** The two reads agree only while
    /// every variant-named macro is a faithful identity mirror. A non-identity
    /// macro shadowing a variant name — `macro-collision-diagnostic`'s open
    /// scope — would make this value differ from the one the engine loads, and
    /// a renderer grading rules text against it would report green on a card
    /// that does not exist. `tests/read_api_gate.rs` therefore rejects a call
    /// to this or [`Plugin::rendering_card`] outside
    /// `crates/deckmaste_legacy_render/`; the plain syn matcher there cannot
    /// see such a call (it names no `read_str` and no `Card`), so that scan is
    /// separate.
    ///
    /// # Errors
    /// As [`Plugin::card_from_str`].
    pub fn rendering_card_from_str(
        &self,
        source: &str,
    ) -> anyhow::Result<deckmaste_semantics::Card> {
        // The restricted read is the gate, and only the gate: its value is
        // discarded, so it reads at the semantic type and skips `lower`.
        self.macros
            .read_str_restricted::<deckmaste_semantics::Card>(source)?;
        Ok(self.macros.read_str(source)?)
    }

    /// [`Plugin::rendering_card_from_str`] for a card in this plugin's
    /// `cards/` directory — the rendering twin of [`Plugin::card`].
    ///
    /// # Errors
    /// As [`Plugin::card`].
    pub fn rendering_card(&self, name: &str) -> anyhow::Result<deckmaste_semantics::Card> {
        let path = self.card_path(name);
        self.rendering_card_from_str(&read(&path)?)
            .with_context(|| format!(r#"parsing "{}""#, path.display()))
    }

    /// The file a token of this name would live in.
    #[must_use]
    pub fn token_path(&self, name: &str) -> PathBuf {
        token_path(&self.root, name)
    }

    /// Reads and parses `tokens/<name>.ron` as a semantic token, with the
    /// plugin's macros in scope, and lowers it to its engine image.
    ///
    /// # Errors
    /// If the file is missing or doesn't expand to a token.
    pub fn token(&self, name: &str) -> anyhow::Result<LoadedToken> {
        let path = self.token_path(name);
        self.token_from_str(&read(&path)?)
            .with_context(|| format!(r#"parsing "{}""#, path.display()))
    }

    /// Reads a semantic token from SOURCE TEXT rather than from the plugin's
    /// `tokens/` directory. Mirrors [`Plugin::card_from_str`].
    ///
    /// # Errors
    /// If the source doesn't expand to a token.
    pub fn token_from_str(&self, source: &str) -> anyhow::Result<LoadedToken> {
        let semantic: deckmaste_semantics::Token = self.macros.read_str_restricted(source)?;
        let core = semantic.clone().lower();
        for ability in core.abilities.iter() {
            validate_ability_regions(ability)?;
        }
        Ok(LoadedToken { semantic, core })
    }
}

fn validate_card_regions(card: &deckmaste_card::Card) -> anyhow::Result<()> {
    let parts: Vec<&deckmaste_card::Characteristics> = match card {
        deckmaste_card::Card::Normal(face) => vec![&face.characteristics],
        deckmaste_card::Card::DoubleFaced { front, back, .. }
        | deckmaste_card::Card::Split {
            left: front,
            right: back,
        } => vec![&front.characteristics, &back.characteristics],
        deckmaste_card::Card::Flip {
            normal,
            alternative,
        }
        | deckmaste_card::Card::Adventurer {
            normal,
            adventure: alternative,
        } => vec![&normal.characteristics, alternative],
    };
    for part in parts {
        for ability in &part.abilities {
            if let Err(error) = validate_ability_regions(ability) {
                anyhow::bail!("validating regions on {}: {error}", part.name);
            }
        }
    }
    Ok(())
}

fn validate_ability_regions(ability: &deckmaste_core::Ability) -> anyhow::Result<()> {
    use deckmaste_core::Ability;
    match ability {
        Ability::Activated(ability) => {
            // [CR#601.2b]: the activation cost is part of the announcement, so
            // its instructions define into the region ahead of the body.
            deckmaste_core::validate_announced(&ability.effect, &ability.targets, &ability.cost)?;
        }
        Ability::Triggered(ability) => {
            deckmaste_core::validate_telescope(&ability.effect, &ability.targets)?;
        }
        Ability::Spell(ability) => {
            // [CR#118.8]: a printed additional cost is announced with the mana
            // cost, so it defines ahead of the body too.
            deckmaste_core::validate_announced(&ability.effect, &ability.targets, &ability.cost)?;
        }
        Ability::Static(region) => deckmaste_core::validate_static(region)?,
        Ability::Keyword(_) => {}
    }
    Ok(())
}

/// The registry tables a prelude hands down to a dependent plugin's load —
/// last plugin wins per name, exactly like the macro scope.
#[derive(Default)]
struct Inherited {
    /// The prelude's registry provenance, carried down so a dependent plugin
    /// can recover semantic terms for subtypes it never declared itself.
    provenance: crate::provenance::ProvenanceIndex,
    subtypes: HashMap<Ident, Subtype>,
    types: HashMap<Ident, TypeDef>,
    counters: HashMap<Ident, Counter>,
    designations: HashMap<Ident, DesignationDecl>,
    keywords: HashMap<Ident, KeywordDecl>,
    verbs: HashMap<Ident, VerbDecl>,
}

/// A keyword-action verb registry row ([CR#701]): the verb's name plus its
/// typed parameter signature (the param-type names, e.g. `[Reference]` for
/// `Destroy`, `[Reference, Count]` for `Mills`). Derived at plugin load from
/// each `KeywordAction`-kind macro, so the macro file IS the registry data;
/// the canonical expansion body stays in the [`Plugin::macros`] set. The
/// verb-side twin of [`KeywordDecl`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VerbDecl {
    pub name: Ident,
    pub params: Vec<Ident>,
}

/// The typed parameter signature of a verb macro — the param-type names in
/// positional order (`[Count]`, `[Reference, Count]`, `[]`). Named signatures
/// (rare for verbs) canonicalize by sorted value type, mirroring
/// [`keyword_shape`].
fn verb_params(params: &crate::macros::Params) -> Vec<Ident> {
    use crate::macros::Params;
    match params {
        Params::Positional(list) => list.iter().map(|p| p.name).collect(),
        Params::Named(map) => {
            let mut names: Vec<Ident> = map.values().map(|p| p.name).collect();
            names.sort_unstable_by(|a, b| a.as_str().cmp(b.as_str()));
            names
        }
    }
}

/// The [`deckmaste_semantics::ParamShape`] a keyword macro's typed parameter
/// signature spells ([CR#702] keyword one-liners): nothing, a `Count`, a
/// `Cost`, `Count` then `Cost` (suspend/awaken), a `Predicate` (landwalk,
/// hexproof-from), `Predicate` then `Cost` (splice), or a `String` name
/// (partner-with). `None` = the signature fits no shape — a load error,
/// keeping the arg vocabulary closed. Named single-param signatures
/// (`{"from": Default(Predicate, Any)}`) map by their one value type. The
/// caller lowers the result at the insert, like every other registry row.
fn keyword_shape(params: &crate::macros::Params) -> Option<deckmaste_semantics::ParamShape> {
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
        [] => deckmaste_semantics::ParamShape::None,
        ["Count"] => deckmaste_semantics::ParamShape::Counted,
        ["Cost"] => deckmaste_semantics::ParamShape::Costed,
        ["Count", "Cost"] | ["Cost", "Count"] => deckmaste_semantics::ParamShape::CountedCost,
        ["Predicate"] => deckmaste_semantics::ParamShape::Predicated,
        ["Predicate", "Cost"] | ["Cost", "Predicate"] => {
            deckmaste_semantics::ParamShape::PredicatedCosted
        }
        ["String"] => deckmaste_semantics::ParamShape::Named,
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
        let file: Vec<deckmaste_semantics::SbaRule> = macros
            .read_str(&source)
            .with_context(|| format!(r#"loading SBA rules from "{}""#, path.display()))?;
        for rule in file.into_iter().map(Lower::lower) {
            deckmaste_core::validate_sba(&rule.region)
                .with_context(|| format!(r#"validating SBA regions from "{}""#, path.display()))?;
            rules.push(rule);
        }
    }
    Ok(rules)
}

/// Loads all `Vec<ConferralRule>` files under `root/rules/grant/`,
/// concatenating them into a single list. An absent directory yields an
/// empty vec.
///
/// # Errors
/// If a file is unreadable or doesn't parse as `Vec<ConferralRule>`.
fn load_conferral_rules(
    root: &Path,
    macros: &MacroSet,
) -> anyhow::Result<Vec<deckmaste_core::ConferralRule>> {
    let dir = root.join(RULES_DIR).join("grant");
    let mut rules = Vec::new();
    for path in ron_files_recursive(&dir)? {
        let source = read(&path)?;
        let file: Vec<deckmaste_semantics::ConferralRule> = macros
            .read_str(&source)
            .with_context(|| format!(r#"loading conferral rules from "{}""#, path.display()))?;
        rules.extend(file.into_iter().map(Lower::lower));
    }
    Ok(rules)
}

/// Loads all `Vec<DamageResultRule>` files under `root/rules/damage/`,
/// concatenating them into a single list. An absent directory yields an
/// empty vec.
///
/// # Errors
/// If a file is unreadable or doesn't parse as `Vec<DamageResultRule>`.
fn load_damage_result_rules(
    root: &Path,
    macros: &MacroSet,
) -> anyhow::Result<Vec<deckmaste_core::DamageResultRule>> {
    let dir = root.join(RULES_DIR).join("damage");
    let mut rules = Vec::new();
    for path in ron_files_recursive(&dir)? {
        let source = read(&path)?;
        let file: Vec<deckmaste_semantics::DamageResultRule> = macros
            .read_str(&source)
            .with_context(|| format!(r#"loading damage result rules from "{}""#, path.display()))?;
        rules.extend(file.into_iter().map(Lower::lower));
    }
    Ok(rules)
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

/// Expands each declared counter, validating its body and filling the registry
/// — keyed by the counter's identity (the `name` field, what a `CounterRef`
/// resolves to). The semantic form is indexed on the way past: a keyword
/// counter's grant appears on no card, so this is its only chance.
fn expand_counters(
    macros: &MacroSet,
    declared: Vec<Ident>,
    provenance: &mut crate::provenance::ProvenanceIndex,
    counters: &mut HashMap<Ident, Counter>,
) -> anyhow::Result<()> {
    for name in declared {
        let counter: deckmaste_semantics::Counter = macros
            .read_str(name.as_str())
            .with_context(|| format!("expanding counter `{name}`"))?;
        provenance.insert_counter(&counter);
        let counter = counter.lower();
        counters.insert(counter.name, counter);
    }
    Ok(())
}

/// Expands each declared designation, validating its body and filling the
/// registry — keyed by the decl's own name.
fn expand_designations(
    macros: &MacroSet,
    declared: Vec<Ident>,
    designations: &mut HashMap<Ident, DesignationDecl>,
) -> anyhow::Result<()> {
    for name in declared {
        let decl: deckmaste_semantics::DesignationDecl = macros
            .read_str(name.as_str())
            .with_context(|| format!("expanding designation `{name}`"))?;
        let decl = decl.lower();
        designations.insert(decl.name, decl);
    }
    Ok(())
}

/// Indexes every token's abilities for provenance recovery.
///
/// A token's abilities reach an object verbatim through `Cards::push_token`,
/// and the `CardId` that mints sits past the end of the card companion table —
/// so the provenance index is a token permanent's ONLY prose channel. Two
/// sources: this plugin's `tokens/` files, and the rules-defined tokens
/// ([CR#111.10]) a `TokenSpec::Named` resolves to, which are built in code and
/// appear in no plugin file.
///
/// Unlike the registry expansions, a token file is loaded on demand, so this
/// pass indexes what it can and leaves a malformed token to fail at its own
/// load site rather than failing every plugin load.
/// A file this pass skipped is REPORTED, not silent: the token still mints,
/// but with no provenance entry its abilities render as `[unrendered: …]`
/// with nothing anywhere saying why. The skips are returned for the caller to
/// surface.
fn index_tokens(
    root: &Path,
    macros: &MacroSet,
    provenance: &mut crate::provenance::ProvenanceIndex,
) -> anyhow::Result<Vec<(PathBuf, String)>> {
    let mut skipped = Vec::new();
    for path in ron_files_recursive(&root.join(TOKENS_DIR))? {
        match read(&path) {
            Err(e) => skipped.push((path, format!("reading: {e}"))),
            Ok(source) => match macros.read_str::<deckmaste_semantics::Token>(&source) {
                Err(e) => skipped.push((path, format!("expanding: {e}"))),
                Ok(token) => provenance.insert_token(&token),
            },
        }
    }
    provenance.insert_predefined_tokens();
    Ok(skipped)
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
///
/// # Errors
/// If `dir` or one of its subdirectories cannot be read.
pub fn ron_files_recursive(dir: &Path) -> anyhow::Result<Vec<PathBuf>> {
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
    use std::sync::Arc;

    use deckmaste_core::Type;

    use super::*;

    fn plugins() -> PathBuf {
        Path::new(env!("CARGO_MANIFEST_DIR")).join("../../plugins")
    }

    /// The generated `wizards` corpus, loaded over the builtin sibling prelude.
    /// Only ever called from `#[cfg_attr(not(wizards_corpus), ignore)]` tests,
    /// so the corpus is guaranteed present when this runs (build.rs sets
    /// the `wizards_corpus` cfg from the directory's presence).
    fn wizards_corpus() -> Plugin {
        Plugin::load_with_sibling_prelude(plugins().join("wizards")).unwrap()
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
        // Every authored row, including the `Destroy(This)` macro invocation,
        // has been lowered to a runnable action before it reaches core.
        assert!(
            plugin
                .sba_rules
                .iter()
                .all(|r| matches!(r.region.body.then, deckmaste_core::Instruction::Act { .. }))
        );
    }

    /// `builtin/rules/grant/planeswalker-loyalty.ron` confers the
    /// enters-with-loyalty replacement ([CR#306.5b]) onto every
    /// planeswalker, as data.
    #[test]
    fn builtin_loads_planeswalker_loyalty_conferral() {
        let plugin = Plugin::load(
            std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../plugins/builtin"),
        )
        .unwrap();
        assert_eq!(plugin.conferral_rules.len(), 1);
        assert!(matches!(
            plugin.conferral_rules[0].scope,
            deckmaste_core::Predicate::Characteristic(
                deckmaste_core::CharacteristicPredicate::Type(ref n)
            ) if n.name() == Type::Planeswalker.name()
        ));
    }

    /// `builtin/rules/damage/planeswalker-loyalty.ron` loads as a
    /// `DamageResultRule` removing loyalty counters from any planeswalker dealt
    /// damage ([CR#120.3c]), as data.
    #[test]
    fn builtin_loads_planeswalker_loyalty_damage_rule() {
        let plugin = Plugin::load(
            std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../plugins/builtin"),
        )
        .unwrap();
        assert_eq!(plugin.damage_result_rules.len(), 1);
        let rule = &plugin.damage_result_rules[0];
        assert!(matches!(
            rule.recipient,
            deckmaste_core::Predicate::Characteristic(
                deckmaste_core::CharacteristicPredicate::Type(ref n)
            ) if n.name() == Type::Planeswalker.name()
        ));
        assert_eq!(
            rule.remove,
            deckmaste_core::CounterRef::from("LoyaltyCounter")
        );
    }

    /// A plugin with no `rules/damage/` dir loads with an empty
    /// `damage_result_rules`, mirroring the `rules/sba/` / `rules/grant/`
    /// loaders' absent-dir behavior.
    #[test]
    fn absent_rules_damage_dir_yields_no_damage_result_rules() {
        let root = tempfile::tempdir().unwrap();
        let plugin = Plugin::load(root.path()).unwrap();
        assert_eq!(plugin.damage_result_rules, vec![]);
    }

    #[test]
    #[cfg_attr(
        not(wizards_corpus),
        ignore = "requires generated plugins/wizards corpus"
    )]
    fn sibling_prelude_brings_builtin_subtypes() {
        let wizards = wizards_corpus();
        // Declared in builtin/macros/types/land, visible through the prelude.
        assert!(wizards.subtypes.contains_key("Plains"));
        // wizards' own declarations load on top.
        assert!(wizards.subtypes.contains_key("Cave"));
    }

    #[test]
    fn builtin_loads_ten_canonical_types_with_permanent_type_flags() {
        use deckmaste_core::TypeDef;
        let plugin = Plugin::load(
            std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../plugins/builtin"),
        )
        .unwrap();
        assert_eq!(plugin.types.len(), 10, "all ten canonical types registered");
        // Land confers its default-deny land-play marker (Task 8, the land-play
        // consumer): a May(Play(what: Ref(This))) row
        // ([CR#305.9,116.2a,701.18]) that legal.rs/cast.rs key
        // land-play + spell-non-castability on.
        assert_eq!(
            plugin.types["Land"],
            TypeDef {
                name: "Land".into(),
                permanent_type: true,
                confers: vec![deckmaste_core::Property::Ability(Arc::new(
                    deckmaste_core::Ability::r#static(deckmaste_core::StaticSpec::Deontic(
                        deckmaste_core::Deontic::May(deckmaste_core::DeonticAction::Play {
                            what: deckmaste_core::Predicate::Ref(deckmaste_core::Reference::Reg(
                                deckmaste_core::RefId(0)
                            )),
                            by: deckmaste_core::Predicate::Any,
                            from: None,
                        })
                    ))
                ))]
                .into(),
            }
        );
        // Instant confers its instant-speed casting window (Task 6, the
        // casting-window consumer): a flash-shaped May(Cast(InstantSpeed)) row
        // ([CR#307.1,117.1a,702.8a]) that flows the may_cast_rows collector.
        assert_eq!(
            plugin.types["Instant"],
            TypeDef {
                name: "Instant".into(),
                permanent_type: false,
                confers: vec![deckmaste_core::Property::Ability(Arc::new(
                    deckmaste_core::Ability::r#static(deckmaste_core::StaticSpec::Deontic(
                        deckmaste_core::Deontic::May(deckmaste_core::DeonticAction::Cast {
                            what: deckmaste_core::Predicate::Ref(deckmaste_core::Reference::Reg(
                                deckmaste_core::RefId(0)
                            )),
                            by: deckmaste_core::Predicate::Any,
                            from: None,
                            window: Some(deckmaste_core::Timing::InstantSpeed),
                            cost: None,
                            tag: None,
                        })
                    ))
                ))]
                .into(),
            }
        );
    }

    /// The builtin type declarations reach the wizards corpus via the sibling
    /// prelude. Split from
    /// `builtin_loads_ten_canonical_types_with_permanent_type_flags`
    /// so that test's builtin assertions still run on a bare checkout.
    #[test]
    #[cfg_attr(
        not(wizards_corpus),
        ignore = "requires generated plugins/wizards corpus"
    )]
    fn wizards_prelude_carries_builtin_types() {
        let wizards = wizards_corpus();
        assert!(wizards.types.contains_key("Creature"));
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

        fn contains_power(m: &Modification) -> bool {
            match m {
                Modification::Power(_) => true,
                Modification::Several(members) => members.iter().any(contains_power),
                _ => false,
            }
        }

        let builtin = Plugin::load_with_sibling_prelude(plugins().join("builtin")).unwrap();
        let counter = builtin
            .counters
            .get("P1P1Counter")
            .expect("P1P1Counter registered");
        assert_eq!(counter.name, Ident::from("P1P1Counter"));
        assert!(
            counter
                .confers
                .iter()
                .any(|p| matches!(p, Property::Continuous(_, change) if contains_power(change))),
            "confers a Continuous Power(Up) boost; got {:?}",
            counter.confers
        );
    }

    /// The `+1/+1` counter macro reaches the wizards corpus through the builtin
    /// prelude. Split from `builtin_defines_the_plus_one_counter` so that
    /// test's builtin assertions still run on a bare checkout.
    #[test]
    #[cfg_attr(
        not(wizards_corpus),
        ignore = "requires generated plugins/wizards corpus"
    )]
    fn wizards_prelude_carries_plus_one_counter() {
        let wizards = wizards_corpus();
        assert!(wizards.counters.contains_key("P1P1Counter"));
    }

    /// [CR#704.5q]: the `-1/-1` counter confers both the negative `Continuous`
    /// boost AND the annihilation as a `StateBased` SBA (a `Sequentially` of
    /// two `RemoveCounters`). Exercises the richer confer RON
    /// (`Is`/`HasCounter`, bare-embedded `RemoveCounters`, `CounterCount`).
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
    /// ABILITY-FREE `confers:` ([CR#704.5m] graveyard state-based rule;
    /// [CR#301.5,301.6] host-type `May(Attach)`) even in the WIZARDS corpus —
    /// the defs live in `builtin`, and the generator no longer emits
    /// confers-LESS wizards stubs for them, so under "last plugin wins"
    /// builtin's confers-bearing def is the one in scope. Regression guard: a
    /// confers-less wizards stub would silently strip these.
    #[test]
    #[cfg_attr(
        not(wizards_corpus),
        ignore = "requires generated plugins/wizards corpus"
    )]
    fn wizards_attachment_subtypes_carry_ability_free_confers() {
        use deckmaste_core::Property;

        let wizards = wizards_corpus();
        for name in ["Aura", "Equipment", "Fortification"] {
            let subtype = wizards
                .subtypes
                .get(name)
                .unwrap_or_else(|| panic!("wizards corpus knows the {name} subtype"));
            let has_rule = subtype
                .confers
                .iter()
                .any(|p| matches!(p, Property::Static(_) | Property::StateBased { .. }));
            assert!(
                has_rule,
                "{name} subtype confers an ability-free attachment rule in the wizards \
                 corpus; got confers: {:?}",
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
        prelude.subtypes.get_mut("Plains").unwrap().types = vec![Type::Creature].into();
        let layered = Plugin::load_with_prelude(&prelude, plugins().join("builtin")).unwrap();
        // builtin's own Plains definition replaced the doctored prelude entry.
        assert_eq!(layered.subtypes["Plains"].types, [Type::Land].into());
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
        assert_eq!(plugin.subtypes["Bear"].types, [Type::Creature].into());
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

    /// A malformed token file does not fail the load (leniency is deliberate)
    /// but is no longer dropped on the floor: it lands in `skipped_tokens`
    /// with its reason, so the `[unrendered: …]` the pane would show for that
    /// token has a stated cause.
    #[test]
    fn a_malformed_token_file_is_reported_not_silently_skipped() {
        let root = tempfile::tempdir().unwrap();
        let tokens_dir = root.path().join(TOKENS_DIR);
        std::fs::create_dir_all(&tokens_dir).unwrap();
        std::fs::write(tokens_dir.join("broken.ron"), "Token(this is not RON").unwrap();

        let plugin = Plugin::load(root.path()).unwrap();
        assert_eq!(
            plugin.skipped_tokens.len(),
            1,
            "the malformed file is reported: {:?}",
            plugin.skipped_tokens
        );
        assert!(
            plugin.skipped_tokens[0].0.ends_with("broken.ron"),
            "the report names the file: {:?}",
            plugin.skipped_tokens
        );
    }

    /// A plugin root with no `rules/grant/` directory at all loads with an
    /// empty `conferral_rules`, mirroring the `rules/sba/` loader's
    /// absent-directory tolerance.
    #[test]
    fn absent_rules_grant_dir_yields_no_conferral_rules() {
        let root = tempfile::tempdir().unwrap();
        let plugin = Plugin::load(root.path()).unwrap();
        assert_eq!(plugin.conferral_rules, vec![]);
    }

    /// A `rules/grant/*.ron` file parses as a `Vec<ConferralRule>` and lands
    /// on `Plugin::conferral_rules`, mirroring the `rules/sba/` loader.
    #[test]
    fn a_rules_grant_file_loads_a_conferral_rule() {
        let root = tempfile::tempdir().unwrap();
        let grant_dir = root.path().join("rules").join("grant");
        std::fs::create_dir_all(&grant_dir).unwrap();
        std::fs::write(
            grant_dir.join("planeswalker-loyalty.ron"),
            r#"[
                ConferralRule(
                    scope: Type(name:"Planeswalker",permanent:true),
                    confer: Ability(Static(Replacement(Also(
                        would: ZoneChange(what: Ref(This), to: Battlefield),
                        also: PutCounters(This, LoyaltyCounter, StatOf(This, Loyalty)),
                    )))),
                ),
            ]"#,
        )
        .unwrap();
        let plugin = Plugin::load(root.path()).unwrap();
        assert_eq!(plugin.conferral_rules.len(), 1);
        assert_eq!(
            plugin.conferral_rules[0].scope,
            deckmaste_core::Predicate::r#type(Type::Planeswalker)
        );
    }
}
