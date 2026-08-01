//! The compiled frame lexicon: every framed macro definition and every
//! constructor-catalog entry, compiled once, indexed for the unifier.
//!
//! [`compile`](crate::compile()) turns *one* authored frame into a
//! [`CompiledFrame`]. Matching needs all of them at once, because the frame
//! that recovers a given English tree is not known in advance — the unifier
//! tries every entry and keeps what survives. This module is that
//! assembly step, and it owns the two facts a raw `FrameSpec` does not
//! carry:
//!
//! 1. **Which English category a frame is parsed at.** A `MacroDef` names its
//!    *macro* kinds (`kinds: [OneShotEffect]`), not a [`FragmentKind`] — see
//!    [`macro_fragment_kind`], the first production instance of that table.
//! 2. **What a frame belongs to.** An [`Entry`] pairs a compiled frame with the
//!    RON head symbol it renders (`"Draws"`, `"Target"`) and that symbol's
//!    declared param types, which is what lets the unifier turn a successful
//!    match back into an invocation.
//!
//! One entry per *frame*, not per macro: `Draws` has two frames (the
//! `You`-guarded imperative and the declarative), and they compete as
//! independent candidates at match time, exactly as the guard model
//! intends.

use deckmaste_english::Catalogs;
use deckmaste_english::FragmentKind;
use macro_ron::Ident;
use macro_ron::MacroDef;
use macro_ron::MacroSet;
use macro_ron::Params;
use macro_ron::frames::ConstructorFrames;

use crate::CompiledFrame;
use crate::compile;

/// Where an entry's frames were authored.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Origin {
    /// A `MacroDef`'s own `frames:` list.
    Macro,
    /// A `ConstructorFrames` catalog entry — a raw `deckmaste_core`
    /// constructor with no macro definition behind it.
    Constructor,
}

/// One compiled frame, with everything needed to rebuild the invocation it
/// renders.
#[derive(Debug, Clone)]
pub struct Entry {
    /// The RON head symbol this frame renders: a macro name (`"Draws"`) or a
    /// constructor name (`"DealDamage"`). This is what
    /// [`Recovered::Invocation`](crate::unify::Recovered::Invocation) reports.
    pub name: String,
    /// The declared positional param types, in index order. Its length is the
    /// invocation's arity — every slot is filled either by a hole or by a
    /// guard, which [`compile`](crate::compile()) already enforces.
    pub params: Vec<String>,
    /// This frame's index in its owner's `frames:` list. Kept for
    /// diagnostics: an ambiguity report has to be able to say *which*
    /// `Draws` frame won.
    pub frame_index: usize,
    pub origin: Origin,
    pub frame: CompiledFrame,
}

impl Entry {
    /// The invocation arity — how many arguments a recovery of this entry
    /// produces.
    #[must_use]
    pub fn arity(&self) -> usize {
        self.params.len()
    }

    /// A short human name for diagnostics: `` `Draws`[0] ``.
    #[must_use]
    pub fn label(&self) -> String {
        format!("`{}`[{}]", self.name, self.frame_index)
    }
}

/// Every frame the unifier may match against.
#[derive(Debug, Clone)]
pub struct Lexicon {
    entries: Vec<Entry>,
    /// The macro set every frame here was compiled against — retained so a
    /// *render-time* guard comparison can reach it too, not just a
    /// compile-time one. `render::guard_satisfied` is the reason this exists:
    /// guard satisfaction is defined on fully-expanded canonical form
    /// (`crate::guard::normalized`/`normalize_source`, the round's one
    /// authority), and expanding a guard's readable macro sugar (the seeded
    /// `Target` entry's `Exactly(1)`) needs a `MacroSet` that knows it — the
    /// same one `assemble` already required to compile the lexicon's own
    /// guards in the first place. Cloned once, here, rather than re-threading
    /// a `&MacroSet` through every render call site.
    macros: MacroSet,
}

impl Lexicon {
    /// Compiles every framed macro in `defs` and every frame in
    /// `constructors`.
    ///
    /// `defs` does double duty, and deliberately: it is both the set of macro
    /// definitions to draw frames from *and* the [`MacroSet`] that
    /// [`compile`](crate::compile()) reads guard constants through. A guard's
    /// preferred spelling is the readable macro sugar (the seeded `Target`
    /// entry guards `Exactly(1)`, a plugin macro), so a lexicon assembled
    /// against a macro set that cannot see those macros could not compile its
    /// own catalog.
    ///
    /// A macro registered under several kinds (`Draw` is both
    /// `OneShotEffect` and `KeywordAction`) is visited once: [`MacroSet::iter`]
    /// yields one pair *per kind*, so entries are deduplicated by
    /// `(name, kind set)` — the real identity of a definition — before
    /// compiling, or a two-kind macro's frames would enter the lexicon twice
    /// and every match against them would report a spurious ambiguity.
    ///
    /// A macro's category comes from [`macro_fragment_kind`]. A constructor
    /// entry declares none, so — rather than guessing one, which
    /// [`compile_constructor_frame`] shows cannot be done soundly — its frame
    /// is registered at *every* category it parses cleanly at, and the
    /// target's own category picks at match time. One constructor frame can
    /// therefore contribute several entries.
    ///
    /// # Errors
    /// If any frame fails to compile — for a constructor frame, if it
    /// compiles at no category at all.
    pub fn assemble(
        defs: &MacroSet,
        constructors: &[ConstructorFrames],
        catalogs: &Catalogs,
    ) -> anyhow::Result<Lexicon> {
        let mut entries = Vec::new();

        let mut seen: Vec<(&str, Vec<&str>)> = Vec::new();
        for (_, def) in defs.iter() {
            if def.frames().is_empty() {
                continue;
            }
            let mut kinds: Vec<&str> = def.kinds.iter().map(Ident::as_str).collect();
            kinds.sort_unstable();
            let identity = (def.name.as_str(), kinds);
            if seen.contains(&identity) {
                continue;
            }
            seen.push(identity);

            let params = positional_params(def)?;
            let kind = macro_fragment_kind(def);
            for (frame_index, spec) in def.frames().iter().enumerate() {
                let frame =
                    compile::compile(spec, kind, &params, catalogs, defs).map_err(|error| {
                        error.context(format!(
                            "compiling frame [{frame_index}] of macro `{}` at {kind:?}",
                            def.name.as_str()
                        ))
                    })?;
                entries.push(Entry {
                    name: def.name.as_str().to_string(),
                    params: params.clone(),
                    frame_index,
                    origin: Origin::Macro,
                    frame,
                });
            }
        }

        for catalog_entry in constructors {
            for (frame_index, spec) in catalog_entry.frames.iter().enumerate() {
                let compiled =
                    compile_constructor_frame(spec, &catalog_entry.params, catalogs, defs)
                        .map_err(|error| {
                            error.context(format!(
                                "compiling frame [{frame_index}] of constructor `{}`",
                                catalog_entry.constructor
                            ))
                        })?;
                entries.extend(compiled.into_iter().map(|frame| Entry {
                    name: catalog_entry.constructor.clone(),
                    params: catalog_entry.params.clone(),
                    frame_index,
                    origin: Origin::Constructor,
                    frame,
                }));
            }
        }

        Ok(Lexicon {
            entries,
            macros: defs.clone(),
        })
    }

    /// A lexicon over frames compiled elsewhere.
    ///
    /// [`assemble`](Self::assemble) is the corpus path; this is for a caller
    /// that already holds the frames it wants to match against — a narrowed
    /// lexicon for one category, a single hand-compiled frame under test.
    /// `macros` is what any guard on those frames is expanded through at
    /// render time (see [`Lexicon::macros`]) — pass the same set they were
    /// compiled against, or [`crate::guard::core_reader`] for a guard-free
    /// fixture.
    #[must_use]
    pub fn from_entries(entries: Vec<Entry>, macros: MacroSet) -> Lexicon {
        Lexicon { entries, macros }
    }

    /// Every compiled frame, in assembly order.
    #[must_use]
    pub fn entries(&self) -> &[Entry] {
        &self.entries
    }

    /// The macro set every frame here was compiled against — see the field
    /// doc. Guard satisfaction at render time (`crate::render`) reads through
    /// this, never a second, independently-loaded `MacroSet`.
    #[must_use]
    pub fn macros(&self) -> &MacroSet {
        &self.macros
    }

    /// How many frames the lexicon holds — one per authored frame, not per
    /// macro.
    #[must_use]
    pub fn len(&self) -> usize {
        self.entries.len()
    }

    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    /// The entries whose frames were parsed at `kind`.
    pub fn at(&self, kind: FragmentKind) -> impl Iterator<Item = &Entry> {
        self.entries
            .iter()
            .filter(move |entry| entry.frame.kind == kind)
    }
}

/// The English category a macro's frames are parsed at, from its declared
/// macro `kinds:`.
///
/// **The first production instance of this table.** Until now the mapping
/// lived only in Task 6's pilot integration test
/// (`crates/deckmaste_frames/tests/pilot.rs`), because nothing in production
/// needed it: `cargo xtask macro inspect` takes `--kind` on the command line.
/// A lexicon cannot, so the table has to exist somewhere, and this is the
/// crate that owns the macro-to-English bridge.
///
/// It is a *heuristic over kind names*, not a total function on the kind
/// space — the pilot's own nine macros are what it is verified against. The
/// order matters: `Draw` is both `OneShotEffect` and `KeywordAction` and must
/// come out `Sentence`, while `SacrificeThis` is `CostComponent` and must
/// come out `Cost`, so the more specific kinds are tested first and the
/// sentence case is the fallthrough.
#[must_use]
pub fn macro_fragment_kind(def: &MacroDef) -> FragmentKind {
    let kinds: Vec<&str> = def.kinds.iter().map(Ident::as_str).collect();
    if kinds.contains(&"KeywordAbility") {
        FragmentKind::KeywordLine
    } else if kinds.contains(&"CostComponent") {
        FragmentKind::Cost
    } else if kinds.iter().any(|kind| {
        matches!(
            *kind,
            "Predicate" | "Selection" | "TargetSpec" | "DesignationDecl"
        )
    }) {
        FragmentKind::Nominal
    } else if kinds.contains(&"Ability") {
        FragmentKind::Ability
    } else {
        FragmentKind::Sentence
    }
}

/// Every category a constructor frame is offered at.
const CONSTRUCTOR_KINDS: [FragmentKind; 5] = [
    FragmentKind::Nominal,
    FragmentKind::Sentence,
    FragmentKind::Cost,
    FragmentKind::KeywordLine,
    FragmentKind::Ability,
];

/// Compiles a constructor-catalog frame at **every** category it parses
/// cleanly at, rather than guessing one.
///
/// # Why there is no single answer to guess
///
/// A `MacroDef` names its macro `kinds:`, which [`macro_fragment_kind`] can
/// read a category off. A [`ConstructorFrames`] entry carries only
/// `constructor`, `params` and `frames` — a raw `deckmaste_core` constructor
/// has no macro definition behind it, so there is nothing to read. Task 6's
/// pilot test papered over this with a `constructor == "Target"` special
/// case, which is fine in a test and not fine in a lexicon.
///
/// The obvious repair — try the categories in a fixed order and keep the
/// first clean compile — is **unsound**, and measurably so rather than in
/// principle. Every one of the three seeded entries parses at three or four
/// of the five categories:
///
/// | entry | parses cleanly at |
/// |---|---|
/// | `<Param(0)> deals <Param(1)> damage to <Param(2)>` | Nominal, Sentence, Cost, Ability |
/// | `<Param(0)> gains <Param(1)> life` | Sentence, Cost, Ability |
/// | `target <Param(1)>` | Nominal, Sentence, Cost, Ability |
///
/// `target zzhole1` is an imperative sentence as readily as a nominal
/// (`target` is a verb), and the damage clause is a reduced relative as
/// readily as a sentence — so *no* fixed order gets both right, and the one
/// that reads most natural (Nominal first) silently compiles `DealDamage`
/// as a noun phrase.
///
/// # What this does instead
///
/// It compiles the frame at each accepting category and registers them all.
/// Nothing is guessed: a target's *own* category picks, because a frame's
/// tree is rooted in its `Fragment::<kind>` wrapper and the unifier only
/// aligns roots that agree. The extra entries are inert — a `Cost`-rooted
/// `target <Param(1)>` can only ever match a cost line that reads "target
/// …", which oracle text does not contain — and where two do compete, the
/// tie lands in
/// [`Recovered::ambiguities`](crate::unify::Recovered::Invocation::ambiguities)
/// like any other.
///
/// A `kind:` field on the catalog entry would let this pick exactly one, and
/// is worth a follow-up: `macro_ron::frames` could carry it verbatim as an
/// opaque string, the same way it already carries hole sigils and guard
/// spellings it never interprets, without learning anything about English.
///
/// # Errors
/// If the frame compiles at no category at all, reporting every refusal — a
/// broken authoring must fail the build, not go silently missing from the
/// lexicon.
fn compile_constructor_frame(
    spec: &macro_ron::frames::FrameSpec,
    params: &[String],
    catalogs: &Catalogs,
    macros: &MacroSet,
) -> anyhow::Result<Vec<CompiledFrame>> {
    let mut compiled = Vec::new();
    let mut refusals = Vec::new();
    for kind in CONSTRUCTOR_KINDS {
        match compile::compile(spec, kind, params, catalogs, macros) {
            Ok(frame) => compiled.push(frame),
            Err(error) => refusals.push(format!("{kind:?}: {error:#}")),
        }
    }
    anyhow::ensure!(
        !compiled.is_empty(),
        "frame {:?} parses cleanly at none of the {} fragment categories:\n  {}",
        spec.text,
        CONSTRUCTOR_KINDS.len(),
        refusals.join("\n  "),
    );
    Ok(compiled)
}

/// A macro's positional param type names, in index order.
///
/// # Errors
/// If the macro has a named param signature, which the frame sigil
/// `<Param(i)>` cannot address.
fn positional_params(def: &MacroDef) -> anyhow::Result<Vec<String>> {
    match &def.params {
        Params::Positional(types) => Ok(types
            .iter()
            .map(|param| param.name.as_str().to_string())
            .collect()),
        Params::Named(_) => anyhow::bail!(
            "macro `{}` has a named param signature, which the positional \
             `<Param(i)>` frame sigil cannot address; drop its `frames:` list \
             or give it a positional signature",
            def.name.as_str()
        ),
    }
}
