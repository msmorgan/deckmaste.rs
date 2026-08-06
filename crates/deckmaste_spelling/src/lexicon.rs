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
//!    [`macro_fragment_kind`], which is where that table lives.
//! 2. **What a frame belongs to.** An [`Entry`] pairs a compiled frame with the
//!    RON head symbol it renders (`"Draws"`, `"Target"`) and that symbol's
//!    declared param types, which is what lets the unifier turn a successful
//!    match back into an invocation.
//!
//! One entry per *frame*, not per macro: `Draws` has four frames (the
//! `You`-guarded imperative, the declarative, and a count-1 literal of
//! each), and they compete as independent candidates at match time, exactly
//! as the guard model intends.

use std::collections::BTreeSet;

use deckmaste_english::Catalogs;
use deckmaste_english::FragmentKind;
use macro_ron::Ident;
use macro_ron::MacroDef;
use macro_ron::MacroSet;
use macro_ron::Params;
use macro_ron::frames::ConstructorFrames;
use macro_ron::frames::FrameKind;
use macro_ron::frames::FramePosition;

use crate::CompiledFrame;
use crate::View;
use crate::compile;

/// Where an entry's frames were authored.
///
/// `Ord` is load-bearing, not incidental: it is the first key
/// [`Lexicon::assemble`] sorts entries by, so macro entries always precede
/// constructor entries in the assembled order every specificity tiebreak
/// falls back on.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Ord, PartialOrd)]
pub enum Origin {
    /// A `MacroDef`'s own `frames:` list.
    Macro,
    /// A `ConstructorFrames` catalog entry — a raw `deckmaste_semantics`
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
    /// The value an invocation of this entry stands for, as a body term with
    /// `Param(i)` leaves — [`ConstructorFrames::body`], carried through.
    /// `None` is the implicit positional application `name(arg0, …)`, which
    /// is what every macro entry uses (a macro's own definition body is the
    /// macro set's business, reached by expanding that application) and what
    /// a constructor entry without a `body:` means.
    ///
    /// This is what [`Recovered::Invocation`](crate::unify::Recovered) carries
    /// away from a match, so a recovery can spell itself without holding the
    /// lexicon it came from.
    pub body: Option<String>,
    /// Whether an invocation of this entry **is a target announcement** —
    /// [`ConstructorFrames::announcement`], carried through. Always `false`
    /// for a macro entry (see that field's own doc).
    pub announcement: bool,
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
    /// A macro's category comes from [`macro_fragment_kind`]; a constructor
    /// entry's comes from its required `kind:` field
    /// ([`ConstructorFrames::kind`], converted via [`fragment_kind_of`]), so a
    /// constructor frame contributes exactly one entry, same as a
    /// single-kind macro definition. Registering at every category an entry
    /// happens to parse cleanly at (most parse cleanly at three or four of
    /// the five, since e.g. an imperative reads as a nominal too) was
    /// considered and rejected: it inflates the lexicon 4-5x per entry, and
    /// `try_entry` is `O(lexicon × tree)`, so every extra entry costs a
    /// unification attempt against a category the target could never
    /// actually be rooted at.
    ///
    /// # Authored identity is checked, and the result is sorted
    ///
    /// Two invariants the rest of the crate reads off the returned lexicon,
    /// established here because this is the only place that can:
    ///
    /// 1. **`(name, frame_index, origin)` uniquely names an authored frame.**
    ///    Both directions key on that triple to tell "one authored frame
    ///    registered at several categories" (benign) from "two different
    ///    wordings tied on specificity" (an error):
    ///    `unify::same_authored_frame` uses it to suppress a spurious ambiguity
    ///    report, and `render::select_frame` uses it to decide a D8 tie is
    ///    benign and render `first` anyway — so a *collision* there is not a
    ///    lost diagnostic but a silently wrong render. Nothing else enforces
    ///    it: [`load_constructor_frames`](macro_ron::frames::load_constructor_frames)
    ///    has no uniqueness check at all, and [`MacroSet`] deliberately permits
    ///    one name under several kinds (`Draw`, `Draws` and `Creature` all do
    ///    it in the live corpus, harmlessly — only one of each pair carries
    ///    `frames:`). Two *framed* defs sharing a name, or two catalog entries
    ///    sharing a `constructor`, are rejected here.
    /// 2. **Entry order is deterministic.** [`MacroSet::iter`] documents its
    ///    own order as unspecified (it follows the backing hash maps), and
    ///    `unify`'s specificity ranking falls back to assembly order as its
    ///    final tiebreak — so without a sort, which frame wins a tie varies
    ///    between processes, which is exactly what the round's own G5 finding 4
    ///    observed from the other end. Entries are therefore sorted by
    ///    `(origin, name, frame_index, kind)` before the lexicon is built.
    ///    Sorting last, over the finished vector, is what keeps every
    ///    `Matched.entry` index consistent.
    /// 3. **Frame selection has a unique answer for every invocation.** See
    ///    [`selection_is_unambiguous`]: the render direction picks the
    ///    most-specific frame whose guards an invocation's arguments satisfy,
    ///    and a frame set in which some argument assignment leaves two
    ///    candidates equally specific has no principled winner. Checked over
    ///    the compiled frames, so it is a property of the *lexicon*, not of
    ///    whichever invocation first happens to hit it.
    ///
    /// # Errors
    /// If any frame fails to compile — for a constructor frame, if it
    /// compiles at no category at all — if two framed macro definitions
    /// share a name, if two catalog entries share a `constructor`, or if some
    /// argument assignment leaves two frames tied for most specific.
    pub fn assemble(
        defs: &MacroSet,
        constructors: &[ConstructorFrames],
        catalogs: &Catalogs,
    ) -> anyhow::Result<Lexicon> {
        let mut entries = Vec::new();

        // Identity first, compiling second: invariant 1 is a fact about the
        // *authoring*, so it is checked over the whole input before a single
        // frame is compiled. A collision would otherwise be reported only
        // after (and could be masked by) an unrelated compile failure in
        // whichever definition `MacroSet::iter` happened to yield first.
        let framed = framed_definitions(defs)?;
        constructor_names_are_unique(constructors)?;

        for def in framed {
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
                    body: None,
                    announcement: false,
                });
            }
        }

        for catalog_entry in constructors {
            let kind = fragment_kind_of(catalog_entry.kind);
            if let Some(body) = catalog_entry.body.as_deref() {
                body_covers_declared_params(catalog_entry, body, defs)?;
            }
            for (frame_index, spec) in catalog_entry.frames.iter().enumerate() {
                let frame = compile::compile(spec, kind, &catalog_entry.params, catalogs, defs)
                    .map_err(|error| {
                        error.context(format!(
                            "compiling frame [{frame_index}] of constructor `{}` at {kind:?}",
                            catalog_entry.constructor
                        ))
                    })?;
                entries.push(Entry {
                    name: catalog_entry.constructor.clone(),
                    params: catalog_entry.params.clone(),
                    frame_index,
                    origin: Origin::Constructor,
                    frame,
                    body: catalog_entry.body.clone(),
                    announcement: catalog_entry.announcement,
                });
            }
        }

        // Invariant 2 above. `FragmentKind` is `deckmaste_english`'s and is
        // corpus-gated (no `Ord` to derive on it, and this crate may not add
        // one), so the category key is `kind_rank`'s position in
        // `CONSTRUCTOR_KINDS` — a total, stable order over exactly the five
        // categories a frame can be registered at.
        entries.sort_by(|left, right| {
            (
                left.origin,
                &left.name,
                left.frame_index,
                kind_rank(left.frame.kind),
            )
                .cmp(&(
                    right.origin,
                    &right.name,
                    right.frame_index,
                    kind_rank(right.frame.kind),
                ))
        });

        // Invariant 3 above. After compiling, because specificity is decided
        // on each guard's expanded canonical value — the thing satisfaction
        // is defined on — not on its authored spelling.
        selection_is_unambiguous(&entries)?;

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

    /// The entries that **are** target announcements — the `TargetSpec` and
    /// pro-form wordings a `targets:` list holds.
    ///
    /// This is the class the announce discipline in [`crate::unify`] tests a
    /// filler against, and it is *catalog data*: which English counts as an
    /// announcement is decided by which entries declare
    /// [`ConstructorFrames::announcement`], not by a node shape hardcoded in
    /// this crate. So a lexicon that declares none has no announce discipline
    /// to enforce and matches exactly as it did before the mark existed.
    pub fn announcements(&self) -> impl Iterator<Item = &Entry> {
        self.entries.iter().filter(|entry| entry.announcement)
    }
}

/// The English category a macro's frames are parsed at, from its declared
/// macro `kinds:`.
///
/// **The single definition of that mapping.** `cargo xtask macro inspect`
/// takes a `--kind` on the command line for the one parse it is asked for,
/// but a lexicon has no command line to take one from: an entry's
/// registration category has to be derivable from the macro itself. This is
/// the crate that owns the macro-to-English bridge, so the derivation lives
/// here rather than in either endpoint, and every caller reads it from here
/// instead of keeping a second copy.
///
/// It is a *heuristic over kind names*, not a total function on the kind
/// space — the pilot's own framed macros are what it is verified against. The
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

/// Every category a constructor frame can declare via `kind:`, in the fixed
/// order [`kind_rank`] keys its sort on. Not every-category registration —
/// a constructor entry registers at exactly its declared `kind:` (see
/// [`Lexicon::assemble`]) — just the total space `FrameKind`/`FragmentKind`
/// range over.
const CONSTRUCTOR_KINDS: [FragmentKind; 5] = [
    FragmentKind::Nominal,
    FragmentKind::Sentence,
    FragmentKind::Cost,
    FragmentKind::KeywordLine,
    FragmentKind::Ability,
];

/// Whether `kind` is one [`CONSTRUCTOR_KINDS`] covers. Exhaustive on purpose:
/// a new [`FragmentKind`] variant must fail to compile here rather than
/// silently escape [`selection_is_unambiguous`]'s per-kind narrowing sweep
/// and sort last through [`kind_rank`]'s fallthrough. Both of those read the
/// table, and neither can tell a category it was never given from one that
/// does not exist.
const fn kind_is_swept(kind: FragmentKind) -> bool {
    match kind {
        FragmentKind::Nominal
        | FragmentKind::Sentence
        | FragmentKind::Cost
        | FragmentKind::KeywordLine
        | FragmentKind::Ability => true,
    }
}

/// A total order over [`FragmentKind`], for [`Lexicon::assemble`]'s sort.
///
/// Public because it is the *only* definition of that order, and a tool that
/// resolves a name to one of several same-named definitions has to break its
/// tie the same way the assembled lexicon does or the two disagree about
/// which definition a name means.
///
/// `FragmentKind` belongs to the corpus-gated `deckmaste_english` crate,
/// which this round must not modify, so it carries no `Ord` to derive from.
/// [`CONSTRUCTOR_KINDS`] already enumerates every category a frame can be
/// registered at, so its index is a total, stable key. The `usize::MAX`
/// fallthrough is unreachable — every category a frame can carry is in the
/// table (a macro frame's comes from [`macro_fragment_kind`], a constructor
/// frame's from its declared `kind:`), and [`kind_is_swept`] is what keeps
/// that true, by refusing to compile against a `FragmentKind` the table has
/// not been extended for. It exists only because `position` returns an
/// `Option` and this function is total.
#[must_use]
pub fn kind_rank(kind: FragmentKind) -> usize {
    CONSTRUCTOR_KINDS
        .iter()
        .position(|candidate| *candidate == kind)
        .unwrap_or(usize::MAX)
}

/// The schema-to-engine bridge for a constructor entry's declared category:
/// [`FrameKind`] is `macro_ron`'s dependency-free mirror of this crate's
/// [`FragmentKind`] (see [`FrameKind`]'s own doc for why the duplication),
/// so something on this side has to say what each variant means. Exhaustive
/// on purpose — a `FrameKind` variant with no arm here must fail the build,
/// not silently fall through to some default category.
///
/// A plain function, not `impl From<FrameKind> for FragmentKind`: both types
/// are foreign to this crate (`FrameKind` to `macro_ron`, `FragmentKind` to
/// `deckmaste_english`), and `From` is foreign too, so that impl is an
/// orphan-rule violation no matter which of the three crates it is written
/// in — the two owning crates can't take the impl either, without a
/// dependency arrow this round has already ruled out (`macro_ron` gains no
/// new dependency; `deckmaste_english` stays a leaf). This function is the
/// exhaustive conversion in the one place that already depends on both
/// types.
pub(crate) fn fragment_kind_of(kind: FrameKind) -> FragmentKind {
    match kind {
        FrameKind::Nominal => FragmentKind::Nominal,
        FrameKind::Sentence => FragmentKind::Sentence,
        FrameKind::Cost => FragmentKind::Cost,
        FrameKind::KeywordLine => FragmentKind::KeywordLine,
        FrameKind::Ability => FragmentKind::Ability,
    }
}

/// Every framed macro definition in `defs`, once each, with
/// [`Lexicon::assemble`]'s invariant 1 checked over the whole set.
///
/// [`MacroSet::iter`] yields one pair *per kind*, so a multi-kind definition
/// appears several times and is deduplicated by `(name, sorted kinds)` — the
/// real identity of a definition. Two *different* definitions sharing a name
/// are the collision: they would produce entries indistinguishable under
/// `(name, frame_index, origin)`.
///
/// # Errors
/// If two different framed definitions share a name.
fn framed_definitions(defs: &MacroSet) -> anyhow::Result<Vec<&MacroDef>> {
    let mut seen: Vec<(&str, Vec<&str>)> = Vec::new();
    let mut unique = Vec::new();
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
        if let Some((_, other_kinds)) = seen.iter().find(|(name, _)| *name == identity.0) {
            anyhow::bail!(
                "two different framed macro definitions are both named `{}` (kinds {:?} and \
                 {:?}); an entry's semantic identity is `(name, frame_index, origin)`, which \
                 both the match and the render direction rely on being unique — see \
                 `Lexicon::assemble`'s own doc. Rename one, or drop its `frames:` list.",
                identity.0,
                other_kinds,
                identity.1,
            );
        }
        seen.push(identity);
        unique.push(def);
    }
    Ok(unique)
}

/// [`Lexicon::assemble`]'s invariant 1 for the constructor catalog:
/// `load_constructor_frames` reads a directory of RON files with no
/// uniqueness check of its own, so two entries — in one file or across two —
/// can name the same constructor.
///
/// # Errors
/// If two catalog entries share a `constructor` name.
fn constructor_names_are_unique(constructors: &[ConstructorFrames]) -> anyhow::Result<()> {
    let mut seen: Vec<&str> = Vec::new();
    for entry in constructors {
        anyhow::ensure!(
            !seen.contains(&entry.constructor.as_str()),
            "two constructor catalog entries are both named `{}`; an entry's semantic identity \
             is `(name, frame_index, origin)`, which both the match and the render direction \
             rely on being unique — see `Lexicon::assemble`'s own doc. Merge their `frames:` \
             lists into one entry.",
            entry.constructor,
        );
        seen.push(&entry.constructor);
    }
    Ok(())
}

/// Every syntactic position a render may be requested at. The whole space,
/// not a sample: [`selection_is_unambiguous`] has to quantify over it, and a
/// position no frame currently keys on is exactly where an unnoticed
/// ambiguity would sit.
const POSITIONS: [FramePosition; 2] = [FramePosition::Main, FramePosition::Trigger];

/// Whether `position` is one [`POSITIONS`] sweeps. Exhaustive on purpose: a
/// new [`FramePosition`] variant must fail to compile here rather than
/// silently escape the sweep.
const fn position_is_swept(position: FramePosition) -> bool {
    match position {
        FramePosition::Main | FramePosition::Trigger => true,
    }
}

/// Rejects a frame set in which some invocation would have no *unique*
/// most-specific frame.
///
/// # The rule this decides
///
/// Rendering picks among the entries sharing the invocation's head symbol:
/// those registered at the caller's category (when it has one) and whose
/// `position` key admits the caller's position, narrowed to those whose every
/// guard the arguments satisfy, and among those the one whose satisfied
/// guard-param set strictly contains every rival's (`render::select_frame`).
/// Two candidates neither of which dominates the other is a tie with no
/// principled winner, and this is where that is refused — over the lexicon,
/// for every argument assignment, rather than on whichever invocation first
/// happens to supply the offending arguments.
///
/// Totality is a *different* property and is deliberately not checked here:
/// an argument assignment that satisfies **no** frame's guards is a coverage
/// gap (the render says so, by name), not an ambiguity. A catalog entry that
/// applies only at one constant — a singular "target `<predicate>`" wording
/// guarding its quantity — is exactly that shape and must keep assembling.
///
/// # Why the quantifier is finite
///
/// "Every argument assignment" is infinite; the *distinctions it can draw*
/// are not. A guard holds iff the argument's canonical form equals the
/// guard's, so at each guarded param the only cases are: equals one of the
/// distinct constants some frame guards there, or equals none of them. The
/// sweep enumerates exactly that product, which is complete for selection —
/// no third behaviour exists — and small (guarded params per name are a
/// handful, constants per param fewer).
///
/// # Why a pairwise test would be wrong
///
/// Comparability of two frames' guard-param sets, taken in isolation, is
/// *not* the condition. A pair guarding disjoint params is incomparable and
/// still unambiguous whenever a third frame guarding the union is viable
/// wherever both of them are — which is the shape a subject-guarded wording,
/// a count-guarded wording, and the wording guarding both naturally take.
/// Specificity is a property of the whole candidate set at an assignment.
///
/// # Errors
/// Naming both tied frames, the assignment that ties them, and the position.
fn selection_is_unambiguous(entries: &[Entry]) -> anyhow::Result<()> {
    debug_assert!(POSITIONS.iter().copied().all(position_is_swept));
    debug_assert!(CONSTRUCTOR_KINDS.iter().copied().all(kind_is_swept));
    let mut names: Vec<&str> = entries.iter().map(|entry| entry.name.as_str()).collect();
    names.sort_unstable();
    names.dedup();
    for name in names {
        let same_name: Vec<&Entry> = entries.iter().filter(|entry| entry.name == name).collect();
        if same_name.len() < 2 {
            continue;
        }
        // A *nested filler* render passes no category at all (its text is
        // substituted, never parsed), so every same-named entry competes
        // regardless of where it is registered. A top-level render filters to
        // the caller's category first, and narrowing the candidate set can
        // *create* a tie the wider one had a dominating frame for — so
        // neither grouping subsumes the other and both are swept.
        positions_are_unambiguous(&same_name)?;
        for kind in CONSTRUCTOR_KINDS {
            let at_kind: Vec<&Entry> = same_name
                .iter()
                .copied()
                .filter(|entry| entry.frame.kind == kind)
                .collect();
            if at_kind.len() >= 2 && at_kind.len() < same_name.len() {
                positions_are_unambiguous(&at_kind)?;
            }
        }
    }
    Ok(())
}

/// [`selection_is_unambiguous`] for one candidate group, at each position the
/// group can be rendered in. A frame with no `position` key competes
/// everywhere; a keyed one competes only at its own.
fn positions_are_unambiguous(candidates: &[&Entry]) -> anyhow::Result<()> {
    for position in POSITIONS {
        let here: Vec<&Entry> = candidates
            .iter()
            .copied()
            .filter(|entry| {
                entry
                    .frame
                    .spec
                    .position
                    .is_none_or(|required| required == position)
            })
            .collect();
        if here.len() >= 2 {
            assignments_are_unambiguous(&here, position)?;
        }
    }
    Ok(())
}

/// [`selection_is_unambiguous`] for one candidate group at one position:
/// enumerate the argument assignments the guards can tell apart, and require
/// a single maximal candidate under each.
fn assignments_are_unambiguous(
    candidates: &[&Entry],
    position: FramePosition,
) -> anyhow::Result<()> {
    let mut params: Vec<usize> = candidates
        .iter()
        .flat_map(|entry| entry.frame.guards.iter().map(|guard| guard.param))
        .collect();
    params.sort_unstable();
    params.dedup();
    let constants: Vec<Vec<&View>> = params
        .iter()
        .map(|param| distinct_guard_values(candidates, *param))
        .collect();

    // Mixed radix over the params: each takes one of its mentioned constants,
    // or the extra option standing for every value none of them equals.
    let total: usize = constants.iter().map(|values| values.len() + 1).product();
    for code in 0..total {
        let mut rest = code;
        let choice: Vec<usize> = constants
            .iter()
            .map(|values| {
                let radix = values.len() + 1;
                let picked = rest % radix;
                rest /= radix;
                picked
            })
            .collect();
        let viable: Vec<(&Entry, BTreeSet<usize>)> = candidates
            .iter()
            .filter_map(|entry| {
                entry
                    .frame
                    .guards
                    .iter()
                    .map(|guard| {
                        let slot = params.iter().position(|param| *param == guard.param)?;
                        let values = &constants[slot];
                        let picked = choice[slot];
                        (picked < values.len() && *values[picked] == guard.value)
                            .then_some(guard.param)
                    })
                    .collect::<Option<BTreeSet<usize>>>()
                    .map(|satisfied| (*entry, satisfied))
            })
            .collect();
        // The maximal elements under the ⊆ order on satisfied guard-param
        // sets — the identical relation the render direction ranks by, an
        // actual set relation rather than a guard count, which cannot tell
        // two same-sized incomparable sets apart.
        let maximal: Vec<&Entry> = viable
            .iter()
            .filter(|(_, set)| {
                !viable
                    .iter()
                    .any(|(_, other)| other.len() > set.len() && other.is_superset(set))
            })
            .map(|(entry, _)| *entry)
            .collect();
        anyhow::ensure!(
            maximal.len() <= 1,
            "no unique most-specific frame for `{}` at position {position:?} when {}: {} \
             frame(s) are equally specific — {}. Selection is deterministic data: give one a \
             strictly narrower guard set, guard them apart on a shared param, or key them to \
             different positions.",
            maximal[0].name,
            describe(&params, &constants, &choice, candidates),
            maximal.len(),
            maximal
                .iter()
                .map(|entry| entry.label())
                .collect::<Vec<_>>()
                .join(", "),
        );
    }
    Ok(())
}

/// The distinct canonical constants any candidate guards `param` at — the
/// cases an argument at that position can be in, short of matching none of
/// them.
fn distinct_guard_values<'entries>(
    candidates: &[&'entries Entry],
    param: usize,
) -> Vec<&'entries View> {
    let mut values: Vec<&View> = Vec::new();
    for entry in candidates {
        for guard in &entry.frame.guards {
            if guard.param == param && !values.contains(&&guard.value) {
                values.push(&guard.value);
            }
        }
    }
    values
}

/// One enumerated assignment, in the authored spellings a reader can find in
/// the catalog — never the expanded canonical form the comparison runs on,
/// which is unreadable and is not what anyone would edit.
fn describe(
    params: &[usize],
    constants: &[Vec<&View>],
    choice: &[usize],
    candidates: &[&Entry],
) -> String {
    params
        .iter()
        .enumerate()
        .map(|(slot, param)| {
            let values = &constants[slot];
            let picked = choice[slot];
            match values.get(picked) {
                Some(value) => {
                    let source = candidates
                        .iter()
                        .flat_map(|entry| &entry.frame.guards)
                        .find(|guard| guard.param == *param && guard.value == **value)
                        .map_or("?", |guard| guard.source.as_str());
                    format!("Param({param}) is `{source}`")
                }
                None => format!("Param({param}) is anything no frame guards"),
            }
        })
        .collect::<Vec<_>>()
        .join(" and ")
}

/// A `body:` entry's body must hole **every** param the entry declares, and
/// none it does not.
///
/// Both halves catch a real semantic-input error rather than a hypothetical
/// one. A param the body never holes is an argument the match recovers and then
/// silently drops — the recovery would look complete while carrying less than
/// the English said. A `Param(i)` past the declared arity has no argument to
/// fill it, which would fail at emission time, once, on whichever card
/// happened to reach the entry; here it fails at load, for everyone. The
/// spelled-out-as-a-string near-miss (`body: "By(Param(0), …)"`, which
/// `RawValue` captures with its quotes) lands in the first half: a string
/// literal holes nothing.
///
/// # Errors
/// If the body is unreadable, holes a named param, holes a param the entry
/// does not declare, or leaves a declared param unholed.
fn body_covers_declared_params(
    entry: &ConstructorFrames,
    body: &str,
    macros: &MacroSet,
) -> anyhow::Result<()> {
    let holed = macro_ron::frames::body_param_indices(body, macros).map_err(|error| {
        anyhow::anyhow!(
            "constructor entry `{}` body `{body}`: {error}",
            entry.constructor
        )
    })?;
    for param in &holed {
        anyhow::ensure!(
            *param < entry.params.len(),
            "constructor entry `{}` body holes `Param({param})`, but only {} param(s) are \
             declared",
            entry.constructor,
            entry.params.len(),
        );
    }
    for param in 0..entry.params.len() {
        anyhow::ensure!(
            holed.contains(&param),
            "constructor entry `{}` declares param {param} (`{}`) but its body `{body}` never \
             holes it, so a recovered argument would be dropped; hole every declared param, or \
             drop the param. (A `body:` is semantic as a bare term — `body: By(Param(0), …)` — \
             never as a quoted string, which holes nothing at all.)",
            entry.constructor,
            entry.params[param],
        );
    }
    Ok(())
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

#[cfg(test)]
mod tests {
    use macro_ron::frames::FrameKind;
    use macro_ron::frames::FrameSpec;

    use super::*;

    /// A `MacroSet` holding exactly the two definitions below, both framed.
    /// `guard::core_reader` supplies the registered kind space (`Predicate`
    /// and `Selection` are both real `deckmaste_semantics` macro kinds) with no
    /// macros in it, so nothing but the fixture is in scope.
    fn two_framed_defs_named(name: &str) -> MacroSet {
        let mut macros = crate::guard::core_reader().clone();
        for source in [
            format!(
                r#"(name: "{name}", kinds: [Predicate], frames: ["creature"], body: PermanentOfType(Creature))"#
            ),
            format!(
                r#"(name: "{name}", kinds: [Selection], frames: ["creature"], body: SelectAll(Creature))"#
            ),
        ] {
            let def: MacroDef = macros
                .read_str(&source)
                .unwrap_or_else(|error| panic!("reading fixture {source}: {error}"));
            macros
                .insert(&def)
                .unwrap_or_else(|error| panic!("inserting fixture {source}: {error:?}"));
        }
        macros
    }

    /// Invariant 1, macro half. `MacroSet` deliberately permits one name
    /// under several kinds, so nothing below this function rejects it — and
    /// two entries colliding on `(name, frame_index, origin)` do not merely
    /// lose a diagnostic: `render::select_frame` treats a collision as "the
    /// same authored frame registered twice" and renders one of the two
    /// wordings silently.
    #[test]
    fn assemble_rejects_two_framed_macro_definitions_sharing_a_name() {
        let macros = two_framed_defs_named("DuplicateFixture");
        let error = Lexicon::assemble(&macros, &[], &Catalogs::default())
            .expect_err("two framed defs named `DuplicateFixture` must be rejected");
        let message = format!("{error:#}");
        assert!(
            message.contains("both named `DuplicateFixture`"),
            "{message}"
        );
    }

    /// The control for the test above: one framed definition under *two*
    /// kinds at once is the legal case `MacroSet::iter`'s per-kind yield
    /// produces, and must still assemble. Without it the rejection test
    /// could pass against a check that refused every multi-kind corpus macro
    /// (`Draw`/`Draws` are both `OneShotEffect` + `KeywordAction`).
    #[test]
    fn assemble_accepts_one_framed_definition_registered_under_two_kinds() {
        let mut macros = crate::guard::core_reader().clone();
        let def: MacroDef = macros
            .read_str(
                r#"(name: "MultiKindFixture", kinds: [Predicate, Selection],
                    frames: ["creature"], body: PermanentOfType(Creature))"#,
            )
            .unwrap_or_else(|error| panic!("reading fixture: {error}"));
        macros
            .insert(&def)
            .unwrap_or_else(|error| panic!("inserting fixture: {error:?}"));
        // `Catalogs::default()` cannot parse the catalog noun "creature", so
        // the compile refuses — but it must be the *compile* that refuses,
        // not the identity check.
        let message = Lexicon::assemble(&macros, &[], &Catalogs::default())
            .err()
            .map(|error| format!("{error:#}"))
            .unwrap_or_default();
        assert!(
            !message.contains("both named"),
            "a single two-kind definition is not a duplicate: {message}"
        );
    }

    /// Registering a constructor entry at every category it happens to
    /// parse cleanly at (rather than only its declared `kind:`) costs
    /// several entries where one is meant. `DealDamage`'s own frame text is
    /// the demonstrating case: it parses cleanly at four of the five
    /// categories (Nominal, Sentence, Cost, Ability), so adding one such
    /// entry must move the `Lexicon`'s entry count at its declared category
    /// only — every other category's count is unchanged by adding it.
    #[test]
    fn assemble_registers_a_constructor_entry_at_its_declared_kind_only() {
        let empty = Lexicon::assemble(crate::guard::core_reader(), &[], &Catalogs::default())
            .unwrap_or_else(|error| panic!("assembling empty catalog: {error:#}"));
        let before: Vec<(FragmentKind, usize)> = CONSTRUCTOR_KINDS
            .iter()
            .map(|&kind| (kind, empty.at(kind).count()))
            .collect();

        let catalog = [ConstructorFrames {
            constructor: "DealDamage".to_string(),
            params: vec![
                "Reference".to_string(),
                "Count".to_string(),
                "Reference".to_string(),
            ],
            frames: vec![FrameSpec::bare(
                "<Param(0)> deals <Param(1)> damage to <Param(2)>",
            )],
            kind: FrameKind::Sentence,
            body: None,
            announcement: false,
        }];
        let with_entry =
            Lexicon::assemble(crate::guard::core_reader(), &catalog, &Catalogs::default())
                .unwrap_or_else(|error| panic!("assembling one-entry catalog: {error:#}"));

        for &(kind, before_count) in &before {
            let after_count = with_entry.at(kind).count();
            if kind == FragmentKind::Sentence {
                assert_eq!(
                    after_count,
                    before_count + 1,
                    "declared category Sentence should gain exactly one entry"
                );
            } else {
                assert_eq!(
                    after_count, before_count,
                    "category {kind:?} must be unchanged by adding a Sentence-only entry"
                );
            }
        }
    }

    /// A pump-shaped catalog entry over `(Reference, Count, Count)`, for the
    /// specificity tests below: every frame either holes or guards each of
    /// the three params, and the texts parse at `Sentence` against an empty
    /// catalog (no catalog noun anywhere).
    fn pump_entry(name: &str, frames: Vec<FrameSpec>) -> ConstructorFrames {
        ConstructorFrames {
            constructor: name.to_string(),
            params: vec![
                "Reference".to_string(),
                "Count".to_string(),
                "Count".to_string(),
            ],
            frames,
            kind: FrameKind::Sentence,
            body: None,
            announcement: false,
        }
    }

    fn guarded(text: &str, when: &[(usize, &str)]) -> FrameSpec {
        FrameSpec {
            text: text.to_string(),
            when: when
                .iter()
                .map(|(param, source)| (*param, (*source).to_string()))
                .collect(),
            position: None,
            announced: Vec::new(),
        }
    }

    /// Selection uniqueness, the build-time half. Two frames guarding the
    /// same param at the same constant are equally specific for every
    /// argument that satisfies them, and nothing else in the entry is more
    /// specific — so an invocation with `Param(0) = You` has two maximal
    /// candidates and no principled winner. Refused at assembly, naming both
    /// frames, rather than on whichever invocation first happens to supply
    /// `You`.
    #[test]
    fn assemble_rejects_two_frames_that_tie_on_guard_specificity() {
        let catalog = [pump_entry(
            "TieFixture",
            vec![
                guarded(
                    "~ gets +<Param(1)>/+<Param(2)> until end of turn",
                    &[(0, "You")],
                ),
                guarded("~ and ~ get +<Param(1)>/+<Param(2)>", &[(0, "You")]),
            ],
        )];
        let error = Lexicon::assemble(crate::guard::core_reader(), &catalog, &Catalogs::default())
            .expect_err("two equally-specific frames must be refused at assembly");
        let message = format!("{error:#}");
        assert!(
            message.contains("no unique most-specific frame"),
            "{message}"
        );
        assert!(message.contains("`TieFixture`[0]"), "{message}");
        assert!(message.contains("`TieFixture`[1]"), "{message}");
        // The assignment is reported in the authored spelling, so the report
        // names something findable in the catalog.
        assert!(message.contains("Param(0) is `You`"), "{message}");
    }

    /// The control the check exists to *not* fire on, and the reason it
    /// cannot be a pairwise comparability test over guard-param sets: the
    /// `You`-guarded frame `[0]` and the count-guarded frame `[3]` guard
    /// disjoint params, so neither's guard set contains the other's — yet
    /// the entry is unambiguous, because the frame guarding *both* params
    /// `[2]` is viable exactly when both of them are and strictly dominates
    /// the pair. Uniqueness is a property of the whole candidate set at an
    /// argument assignment, never of a pair in isolation.
    #[test]
    fn assemble_accepts_incomparable_guards_a_third_frame_dominates() {
        let catalog = [pump_entry(
            "DominatedFixture",
            vec![
                guarded(
                    "~ gets +<Param(1)>/+<Param(2)> until end of turn",
                    &[(0, "You")],
                ),
                guarded(
                    "<Param(0)> gets +<Param(1)>/+<Param(2)> until end of turn",
                    &[],
                ),
                guarded(
                    "~ gets +1/+<Param(2)> until end of turn",
                    &[(0, "You"), (1, "1")],
                ),
                guarded(
                    "<Param(0)> gets +1/+<Param(2)> until end of turn",
                    &[(1, "1")],
                ),
            ],
        )];
        Lexicon::assemble(crate::guard::core_reader(), &catalog, &Catalogs::default())
            .unwrap_or_else(|error| panic!("a dominated pair is not an ambiguity: {error:#}"));
    }

    /// Invariant 1, constructor half. `load_constructor_frames` walks a
    /// directory with no uniqueness check, so this is the only thing
    /// standing between a second `DealDamage` catalog entry and a silently
    /// wrong render.
    #[test]
    fn assemble_rejects_two_constructor_entries_sharing_a_name() {
        let catalog = [
            ConstructorFrames {
                constructor: "DealDamage".to_string(),
                params: vec!["Reference".to_string()],
                frames: vec![FrameSpec::bare("<Param(0)> is dealt damage")],
                kind: FrameKind::Sentence,
                body: None,
                announcement: false,
            },
            ConstructorFrames {
                constructor: "DealDamage".to_string(),
                params: vec!["Reference".to_string()],
                frames: vec![FrameSpec::bare("damage is dealt to <Param(0)>")],
                kind: FrameKind::Sentence,
                body: None,
                announcement: false,
            },
        ];
        let error = Lexicon::assemble(crate::guard::core_reader(), &catalog, &Catalogs::default())
            .expect_err("two catalog entries named `DealDamage` must be rejected");
        let message = format!("{error:#}");
        assert!(message.contains("both named `DealDamage`"), "{message}");
    }
}
