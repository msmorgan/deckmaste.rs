//! The unifier: English tree in, RON invocation out.
//!
//! Every stage before this one runs RON → frames → English. This is the
//! matching half. Given the [`View`] of a parsed English fragment and a
//! [`Lexicon`] of compiled frames, [`unify`] finds the frame whose tree the
//! target *is* — modulo its holes — and reads the fillers back out as the
//! arguments of the invocation that would render it.
//!
//! # Matching is total
//!
//! [`unify`] returns a [`Recovered`], never a `Result`. Text nothing in the
//! lexicon covers comes back as [`Recovered::Residual`] carrying the subtree
//! that was not accounted for, at whatever depth it stopped: a sentence with
//! no matching frame is one big residual, and a matched frame whose argument
//! is an unlexicalized nominal is an invocation with a residual argument.
//! That is what makes the unifier usable on the whole corpus long before the
//! lexicon covers it — coverage is a measurement, not a precondition.
//!
//! # What has to be neutralized before two trees can be compared
//!
//! A frame's tree is in *citation form* (see
//! [`CompiledFrame::agreement`](crate::CompiledFrame::agreement)): a
//! hole-driven inflection was rewritten to a fixed spelling at compile time so
//! that two authorings of the same frame compile identically. Card text is not
//! — a card really does say "You **draw** three **cards**" where the frame's
//! citation form says "draws … card". So before comparing, the same rewrites
//! are applied to the target, through the *same code path* the compiler used
//! ([`crate::compile`]'s citation normalizer, driven with the frame's own
//! [`AgreementDep`] list). Two further surface facts are neutralized:
//!
//! - **numeral orthography.** A `Count` hole's witness is always the Arabic
//!   numeral `41`, so every compiled frame claims `Numeral::Arabic` — but "Draw
//!   three cards" spells its count out. The `numeral` field beside a numeric
//!   hole is therefore not compared; see [`surface_only_fields`].
//! - **self-reference form.** `NounPhrase::ThisCard` carries a `ThisCardForm`
//!   (full vs. abbreviated name) and sits under whichever wrapper its position
//!   calls for, so two `~` sites in one frame capture structurally *different*
//!   subtrees for the same referent. The non-linear equality check therefore
//!   compares self-reference-hood, not raw trees.
//!
//! # Guards
//!
//! A guarded frame pre-binds a param that has no surface at all, so matching
//! cannot recover it from the text — it comes from the guard, as the spelling
//! the catalog authored ([`CompiledGuard::source`]), not the expanded form.
//! The expanded form ([`CompiledGuard::value`]) is what *satisfaction* is
//! defined on, and [`guard_holds`] is the one function that checks it: it
//! runs the card-side argument through [`guard::normalized`], the single
//! authority, exactly as compile time did for the authored spelling. There is
//! deliberately no second normalizer here.

use std::cmp::Reverse;
use std::collections::HashMap;

use deckmaste_english::word::Pronoun;
use deckmaste_english::word::PronounCase;
use deckmaste_english::word::PronounInstance;
use deckmaste_english::word::Vocabulary;
use macro_ron::Expand;
use macro_ron::MacroSet;
use macro_ron::frames::FramePosition;
use serde::Serialize;

use crate::CompiledGuard;
use crate::HoleClass;
use crate::View;
use crate::compile::AgreementDep;
use crate::guard;
use crate::lexicon::Entry;
use crate::lexicon::Lexicon;
use crate::view::PathStep;
use crate::view::TreePath;

/// What a stretch of English was recovered as.
///
/// The shape is what a RON writer needs and nothing more: a head symbol with
/// positional arguments, a leaf spelling, or an unrecovered subtree. An
/// invocation's `args` are in **declared param order** and its length is the
/// entry's arity, so `Recovered` can be printed as RON, read back and
/// compared structurally against a card's semantic invocation — which is
/// exactly what the round's ground-truth recovery gate does.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Recovered {
    /// A lexicon entry matched: `entry` is the entry's name (a macro name, or
    /// a constructor-catalog entry's `constructor`), `args` are its arguments
    /// in param order.
    Invocation {
        entry: String,
        args: Vec<Recovered>,
        /// The entry's body term, if it has one
        /// ([`Entry::body`](crate::Entry::body)) — carried here so a recovery
        /// can spell itself without the lexicon it came from.
        ///
        /// `None` means `entry` names the value: the recovery spells as
        /// `entry(arg, …)`. `Some(term)` means the entry's *name* is only a
        /// name, and the value is that term with its `Param(i)` leaves filled
        /// by `args` — which is how one English wording stands for a value
        /// the core repr spells in a different shape (a defaulted subject; a
        /// recipient hoisted into a `targets:` announce list). See
        /// [`Recovered::to_ron`].
        body: Option<String>,
        /// Every *other* frame that also matched at this node, recorded
        /// rather than silently discarded.
        ///
        /// This is the hook the round-2 coherence lint hangs off: an
        /// ambiguity is not an error (one frame still wins, deterministically
        /// — see [`unify`]'s specificity order), but two frames rendering the
        /// same English is a fact about the *lexicon* that a caller may want
        /// to report. Nested ambiguities are collected by
        /// [`Recovered::ambiguities`].
        ambiguities: Vec<String>,
    },
    /// A leaf, as a RON spelling: a numeral read back from the tree
    /// (`"3"`), or a guard's authored constant (`"You"`, `"Exactly(1)"`).
    Literal(String),
    /// Nothing in the lexicon covered this subtree. Never an error — see the
    /// module doc.
    Residual(View),
}

impl Recovered {
    /// Every ambiguity recorded anywhere in this recovery, outermost first.
    #[must_use]
    pub fn ambiguities(&self) -> Vec<&str> {
        let mut out = Vec::new();
        self.collect_ambiguities(&mut out);
        out
    }

    fn collect_ambiguities<'a>(&'a self, out: &mut Vec<&'a str>) {
        if let Recovered::Invocation {
            args, ambiguities, ..
        } = self
        {
            out.extend(ambiguities.iter().map(String::as_str));
            for arg in args {
                arg.collect_ambiguities(out);
            }
        }
    }

    /// Whether anything in this recovery is still unaccounted for.
    #[must_use]
    pub fn has_residual(&self) -> bool {
        match self {
            Recovered::Residual(_) => true,
            Recovered::Literal(_) => false,
            Recovered::Invocation { args, .. } => args.iter().any(Recovered::has_residual),
        }
    }

    /// Spells this recovery as RON source text — **the emission**, the value
    /// a match stands for.
    ///
    /// An invocation with no [`body`](Recovered::Invocation::body) spells as
    /// `entry` (bare, if nullary) or `entry(arg, arg, …)`, recursively; one
    /// with a body spells as that body with its `Param(i)` leaves filled by
    /// the same recursive spellings
    /// ([`macro_ron::frames::substitute_body`], which is `macro_ron`'s own
    /// hole splice and not a second one). A literal is already a valid RON
    /// leaf spelling (digits, or a guard's authored constant) and goes
    /// through verbatim.
    ///
    /// A [`Recovered::Residual`] is refused outright: it carries only a
    /// captured [`View`], which has no RON spelling at all. That refusal is
    /// the honest answer rather than a gap — a recovery containing one is
    /// *provably incomplete*, so it cannot be asserted equal to a fully
    /// concrete semantic value — and the error says **where**, because
    /// otherwise every incomplete recovery reports identically and one
    /// lexicon gap is indistinguishable from another.
    ///
    /// `macros` supplies the RON dialect a body is read in; pass the set the
    /// lexicon was assembled against ([`Lexicon::macros`]).
    ///
    /// # Errors
    /// If this is, or contains, a [`Recovered::Residual`], or if an entry's
    /// body cannot be filled from the recovered arguments.
    pub fn to_ron(&self, macros: &MacroSet) -> anyhow::Result<String> {
        self.to_ron_at(macros, &mut Vec::new())
    }

    /// [`to_ron`](Self::to_ron), tracking the argument path it is currently
    /// under so a refusal can say *where*.
    ///
    /// `path` is a stack of `entry arg i` steps, joined with ` -> ` for a
    /// nested filler. It is only ever read on the error path, so the pushes
    /// cost nothing that matters.
    fn to_ron_at(&self, macros: &MacroSet, path: &mut Vec<String>) -> anyhow::Result<String> {
        match self {
            Recovered::Literal(text) => Ok(text.clone()),
            Recovered::Invocation {
                entry, args, body, ..
            } => {
                let mut parts = Vec::with_capacity(args.len());
                for (index, argument) in args.iter().enumerate() {
                    path.push(format!("{entry} arg {index}"));
                    parts.push(argument.to_ron_at(macros, path)?);
                    path.pop();
                }
                match body {
                    Some(body) => {
                        let filled: Vec<&str> = parts.iter().map(String::as_str).collect();
                        macro_ron::frames::substitute_body(entry, body, &filled, macros)
                            .map_err(|error| anyhow::anyhow!("emitting `{entry}`: {error}"))
                    }
                    None if parts.is_empty() => Ok(entry.clone()),
                    None => Ok(format!("{entry}({})", parts.join(", "))),
                }
            }
            Recovered::Residual(view) => anyhow::bail!(
                "a residual filler at {} has no RON spelling; the unrecovered View was {}",
                if path.is_empty() { "the top level".to_string() } else { path.join(" -> ") },
                truncated_debug(view),
            ),
        }
    }
}

/// How much of a captured [`View`] an incompleteness report shows: enough to
/// recognize *which* constituent went unrecovered (its node type and head
/// word are near the front of the `Debug`), not the whole subtree — one
/// unrecovered nominal debug-prints to several hundred lines, and a page of
/// them would bury the report under it.
const RESIDUAL_DEBUG_BUDGET: usize = 240;

/// A one-line, length-capped `Debug` of `view`. Truncation is by
/// `char_indices`, never a byte slice, so a multi-byte character straddling
/// the budget cannot panic.
fn truncated_debug(view: &View) -> String {
    let full = format!("{view:?}");
    let flattened = full.split_whitespace().collect::<Vec<_>>().join(" ");
    match flattened.char_indices().nth(RESIDUAL_DEBUG_BUDGET) {
        Some((at, _)) => format!("{}… ({} chars total)", &flattened[..at], flattened.len()),
        None => flattened,
    }
}

/// How deep argument recovery may recurse.
///
/// A bound rather than a budget: recursion descends into a *captured*
/// subtree, which is strictly smaller than the tree it came from in every
/// frame the compiler can produce (a match must claim at least one node, so a
/// frame cannot bind its whole input to one hole). The cap exists so that a
/// future frame shape that broke that property would degrade to a residual
/// instead of overflowing the stack.
const MAX_DEPTH: usize = 16;

/// Recovers the invocation that renders `target`.
///
/// `target` is the [`View`] of a parsed
/// [`Fragment`](deckmaste_english::Fragment) — `view::of(&fragment)` — or, in
/// the recursive calls, of one captured constituent of one.
///
/// `position` is the syntactic-position key a guarded frame is matched
/// against: a frame carrying `position: Main` is only a candidate when
/// `position` is [`FramePosition::Main`]. A frame with no position key is a
/// candidate anywhere.
///
/// # Specificity
///
/// Every entry is tried; each surviving match is ranked by
///
/// 1. **nodes claimed** — how much of the target the frame's own material
///    accounted for, holes excluded. `~ deals N damage to each <P>` outranks
///    `<P0> deals <P1> damage to <P2>` on "… to each creature" because it
///    claims the determiner "each" too.
/// 2. **guards** — a frame that pre-binds a param is more specific than one
///    that spells it out, which is what makes imperative "Draw three cards."
///    recover `Draws(You, 3)` rather than the also-matching `Draw(3)`.
/// 3. assembly order, so the answer is deterministic.
///
/// The winner is returned and every other match is recorded in
/// [`Recovered::Invocation::ambiguities`].
#[must_use]
pub fn unify(target: &View, lexicon: &Lexicon, position: FramePosition) -> Recovered {
    unify_at(target, lexicon, position, 0)
}

/// Whether `argument` satisfies `guard`.
///
/// **The single authority, consumed not reimplemented.** Guard satisfaction
/// is defined on fully-expanded canonical form, because RON spellings are not
/// unique — `Exactly(1)` and `Range(Some(1), Some(1))` are the same
/// `Quantity`. [`guard::normalized`] is the one function that produces that
/// form; compile time ran the *authored spelling* through it to get
/// [`CompiledGuard::value`], and this runs the *card-side argument* through
/// the very same call. A separate match-side normalizer would be a defect,
/// not an optimization: the two would drift, and the drift would present as a
/// frame that quietly stopped matching.
///
/// A non-ground argument — one still holding a free `Param(…)` — makes the
/// guard **fail**, returning `false`. It is never an error: guard selection
/// falls through to a less-specific frame, and totality is preserved.
///
/// The unifier itself never calls this on a *recovered* argument, and cannot:
/// a guarded param has no surface in the frame text at all (the compiler
/// refuses a param that is both holed and guarded), so at match time its
/// value comes from the guard rather than from the card. This is the
/// primitive the other direction needs — frame *selection*, where the card's
/// real argument is in hand and the most specific satisfied guard wins.
pub fn guard_holds<T: Expand + Serialize>(guard: &CompiledGuard, argument: T) -> bool {
    let canonical = guard::normalized(argument);
    guard::ensure_ground(&canonical).is_ok() && canonical == guard.value
}

/// Whether a recovered argument satisfies a frame guard in expanded canonical
/// form. Frame selection and render reassembly share this path so a value's
/// alternative ground spellings retain identical guard semantics.
pub(crate) fn recovered_guard_holds(
    guard: &CompiledGuard,
    argument: Option<&Recovered>,
    macros: &MacroSet,
) -> bool {
    let Some(Recovered::Literal(text)) = argument else {
        return false;
    };
    guard::normalize_source(macros, &guard.param_type, text).is_ok_and(|view| view == guard.value)
}

/// Whether this subtree is a self-reference to the invoking card.
///
/// Two surface forms count, and they are genuinely different trees rather
/// than two spellings of one:
///
/// - the **`ThisCard` leaf** — `NounPhrase::ThisCard(ThisCardForm)`, what the
///   parser builds where a card repeats its own printed name ("Lightning Bolt
///   deals 3 damage …"). This is what the `~` sigil compiles to.
/// - the **demonstrative nominal** — "this permanent", "this creature": a
///   determinerful `NominalPhrase` whose determiner is
///   `DeterminerKind::Demonstrative(This)` and which carries no modifiers or
///   complements of its own. This is the form 99.2% of the corpus's cost lines
///   use, and the one the pilot's G5 report records as having no *authoring*
///   story yet — but it is a self-reference on the page whether or not a frame
///   can spell it, so matching accepts it.
///
/// Both are checked as **predicates on the tree**, not against hardcoded
/// node shapes, and the search runs down [`content_chain`] rather than at
/// `view` itself. That is not leniency, it is the mirror of how the hole got
/// there: the compiler grows a hole up through every ancestor that holds no
/// frame material, so a `~` in `Sacrifice ~` ends up owning the predicate's
/// whole `Transitive` node — empty pre-object list and all — not the noun
/// phrase inside it. The card-side subtree the hole is compared against
/// carries exactly the same empty scaffolding, so the self-reference has to
/// be looked for underneath it.
#[must_use]
pub fn is_self_reference(view: &View) -> bool {
    content_chain(view)
        .into_iter()
        .any(|node| is_this_card(node) || is_demonstrative_self(node))
}

/// `view`, then each node below it that carries all of its content, until a
/// node genuinely branches.
///
/// A step down is taken only when exactly one child is non-vacuous
/// ([`View::is_vacuous`]) — a newtype's payload, the one occupied field of a
/// node whose siblings are `None`/`[]`, the sole element of a sequence. Every
/// node on the chain therefore *is* the same content wearing more or less
/// scaffolding, which is the same judgement the frame compiler's hoist makes
/// when it decides how far up a witness's hole reaches.
fn content_chain(view: &View) -> Vec<&View> {
    let mut chain = vec![view];
    let mut node = view;
    loop {
        let mut occupied = node
            .children()
            .into_iter()
            .map(|(_, child)| child)
            .filter(|child| !child.is_vacuous());
        let Some(only) = occupied.next() else { break };
        if occupied.next().is_some() {
            break;
        }
        node = only;
        chain.push(node);
    }
    chain
}

fn is_this_card(node: &View) -> bool {
    node.type_name() == Some("NounPhrase") && node.variant_name() == Some("ThisCard")
}

fn is_demonstrative_self(node: &View) -> bool {
    let View::Node { name, fields, .. } = node else {
        return false;
    };
    if *name != "NominalPhrase" {
        return false;
    }
    let field = |wanted: &str| {
        fields
            .iter()
            .find_map(|(name, value)| (*name == wanted).then_some(value))
    };
    let demonstrative = field("determiner").is_some_and(|determiner| {
        determiner.type_name() == Some("Determiner")
            && determiner.variant_name() == Some("Demonstrative")
            && matches!(determiner, View::Newtype { inner, .. }
                if inner.variant_name() == Some("This"))
    });
    demonstrative
        && field("modifiers").is_none_or(View::is_vacuous)
        && field("complements").is_none_or(View::is_vacuous)
        && field("head").is_some_and(|head| !head.is_vacuous())
}

// ---------------------------------------------------------------------------
// Matching
// ---------------------------------------------------------------------------

/// What one hole captured.
#[derive(Debug, Clone, PartialEq, Eq)]
enum Binding {
    /// A whole subtree ([`HoleClass::Subtree`]), a partial node
    /// ([`HoleClass::FieldSlice`]) or a self-reference site
    /// ([`HoleClass::SelfRef`]).
    Node(View),
    /// A scalar read straight back out of the tree — a numeral or a P/T half.
    Scalar(String),
}

/// One entry that matched, with what it cost and what it caught.
struct Matched {
    entry: usize,
    claimed: usize,
    guards: usize,
    bindings: HashMap<usize, Binding>,
}

/// The state one match attempt threads through the walk.
struct Attempt {
    bindings: HashMap<usize, Binding>,
    claimed: usize,
}

impl Attempt {
    /// Binds a hole, or checks the new capture against what the hole already
    /// caught — the non-linear case, which only `~` can reach (a `<Param(i)>`
    /// may appear on exactly one constituent; the compiler enforces it).
    ///
    /// For a self-reference the check is *denotational*: every site of one
    /// `~` refers to the invoking card, so any two self-references agree by
    /// definition, and demanding tree equality would reject a perfectly
    /// well-formed frame — the subject site of `~ deals N damage to ~`
    /// captures `Subject(ThisCard)` while the object site captures
    /// `Phrase::NounPhrase(ThisCard)`. Each site has already been checked to
    /// *be* a self-reference before it gets here.
    fn bind(&mut self, hole: usize, class: &HoleClass, value: Binding) -> bool {
        match self.bindings.get(&hole) {
            None => {
                self.bindings.insert(hole, value);
                true
            }
            Some(_) if matches!(class, HoleClass::SelfRef) => true,
            Some(existing) => *existing == value,
        }
    }
}

fn unify_at(target: &View, lexicon: &Lexicon, position: FramePosition, depth: usize) -> Recovered {
    if depth >= MAX_DEPTH {
        return Recovered::Residual(target.clone());
    }
    // Recomputed per node rather than threaded down: it is a filter over the
    // whole lexicon, which is two orders of magnitude smaller than the
    // per-entry × per-alignment work below it, and threading it would put a
    // second copy of the lexicon's own state in every signature.
    let announcements: Vec<&Entry> = lexicon.announcements().collect();
    let mut matches: Vec<Matched> = Vec::new();
    for (index, entry) in lexicon.entries().iter().enumerate() {
        if entry
            .frame
            .spec
            .position
            .is_some_and(|required| required != position)
        {
            continue;
        }
        if let Some(matched) = try_entry(index, entry, target, Some(&announcements)) {
            matches.push(matched);
        }
    }
    let Some(best) = matches
        .iter()
        .enumerate()
        .max_by_key(|(order, matched)| {
            // Reverse the assembly order so `max_by_key` still prefers the
            // earliest entry once claim count and guard count have tied.
            (matched.claimed, matched.guards, usize::MAX - *order)
        })
        .map(|(order, _)| order)
    else {
        return Recovered::Residual(target.clone());
    };

    // Two *registrations of one authored frame* are not an ambiguity — they
    // are one reading reached twice. Every entry an assembled lexicon holds
    // registers at exactly one category (see `lexicon::Lexicon::assemble`),
    // so this can only ever distinguish two genuinely different authored
    // frames that happen to match the same target equally well; the
    // same-identity case it exists to swallow is live only against a
    // hand-built `Lexicon::from_entries` lexicon that does not enforce
    // assembly's uniqueness (two entries deliberately sharing `(name,
    // frame_index, origin)` at different categories would otherwise report
    // a tie against themselves). An ambiguity is two frames disagreeing
    // about what the English *is*, so the report is built from the rivals
    // that name a different authored frame.
    let rivals: Vec<usize> = (0..matches.len())
        .filter(|order| *order != best && !same_authored_frame(lexicon, &matches, *order, best))
        .collect();
    let ambiguities = if rivals.is_empty() {
        Vec::new()
    } else {
        vec![describe_tie(lexicon, &matches, best, &rivals)]
    };
    let winner = &matches[best];
    let entry = &lexicon.entries()[winner.entry];
    let args = (0..entry.arity())
        .map(|param| recover_argument(entry, &winner.bindings, param, lexicon, position, depth))
        .collect();
    Recovered::Invocation {
        entry: entry.name.clone(),
        args,
        body: entry.body.clone(),
        ambiguities,
    }
}

/// The value of one declared param: from its hole if it has one, from its
/// guard if it does not.
fn recover_argument(
    entry: &Entry,
    bindings: &HashMap<usize, Binding>,
    param: usize,
    lexicon: &Lexicon,
    position: FramePosition,
    depth: usize,
) -> Recovered {
    if let Some(hole) = entry.frame.hole_for_param(param) {
        return match bindings.get(&hole.index) {
            Some(Binding::Scalar(repr)) => Recovered::Literal(repr.clone()),
            Some(Binding::Node(node)) => unify_at(node, lexicon, position, depth + 1),
            // Unreachable through a successful match — every hole in the
            // pattern is visited by the walk — but recovery stays total
            // rather than panicking on a frame shape nobody has authored yet.
            None => Recovered::Residual(View::Absent),
        };
    }
    if let Some(guard) = entry.frame.guards.iter().find(|guard| guard.param == param) {
        // The SEMANTIC spelling, not the expanded canonical form: the two are
        // equal as values (that is what `guard_holds` compares), and the
        // authored one is what the catalog reads like.
        return Recovered::Literal(guard.source.clone());
    }
    Recovered::Residual(View::Absent)
}

/// Whether two matches came from the same authored frame.
///
/// Identity is the authoring, not the compiled entry: same owner name, same
/// slot in its `frames:` list, same origin — the same triple
/// `render::same_authored_frame` keys on, the render-direction sibling of
/// this question. An assembled lexicon never holds two entries sharing that
/// triple (see `lexicon::Lexicon::assemble`), so within one this only ever
/// compares an entry against itself; a hand-built `Lexicon::from_entries`
/// lexicon is the one place two distinct entries can still share it.
fn same_authored_frame(lexicon: &Lexicon, matches: &[Matched], left: usize, right: usize) -> bool {
    let entry = |order: usize| &lexicon.entries()[matches[order].entry];
    let (left, right) = (entry(left), entry(right));
    left.name == right.name && left.frame_index == right.frame_index && left.origin == right.origin
}

fn describe_tie(lexicon: &Lexicon, matches: &[Matched], best: usize, rivals: &[usize]) -> String {
    let describe = |order: usize| {
        let found = &matches[order];
        let entry = &lexicon.entries()[found.entry];
        format!(
            "{} {:?} (claims {}, {} guard(s))",
            entry.label(),
            entry.frame.spec.text,
            found.claimed,
            found.guards,
        )
    };
    let losers: Vec<String> = rivals.iter().copied().map(describe).collect();
    format!(
        "{} frames match: chose {}; also matched {}",
        rivals.len() + 1,
        describe(best),
        losers.join(", "),
    )
}

/// Tries one lexicon entry against `target`, at every alignment of the two
/// trees' outer wrappers.
///
/// A frame is compiled at a whole
/// [`FragmentKind`](deckmaste_english::FragmentKind), so its tree is wrapped in
/// `Fragment::<kind>(…)`; a target may be a whole fragment too, or — in the
/// recursive calls — a constituent captured from inside one, wearing whatever
/// wrapper chain its position gave it
/// (`Phrase::NounPhrase(NounPhrase::Nominal(…))` in an object slot, nothing
/// at all for a field slice's partial node). Those chains are pure category
/// plumbing: they carry no frame material and no argument.
///
/// So both sides are peeled through their newtype wrappers and every pair of
/// depths is tried, preferring the least-peeled alignment that matches.
/// Peeling is only ever applied at the *root* of an attempt; inside the walk,
/// wrappers are compared exactly.
///
/// # Bare-hole patterns
///
/// Peeling can strip a pattern down to nothing but its hole — that is what a
/// *pro-form* frame is, a frame whose entire text is one sigil. Such a pattern
/// is only admissible when the hole itself discriminates:
///
/// - a [`HoleClass::SelfRef`] hole does, and strongly. Its two predicates
///   ([`is_self_reference`]) accept a self-reference and nothing else, so
///   `(constructor: "This", frames: ["~"])` matches exactly the trees that
///   *are* the card naming itself. It is admitted, and its match counts one
///   claimed node.
/// - every other class does not. A bare [`HoleClass::Subtree`] hole accepts any
///   tree at all and would make a contentless frame match everywhere, so it is
///   refused here — and refused again by the `claimed == 0` check below, which
///   is the same guarantee stated a second way.
///
/// # `announcements`
///
/// `Some(entries)` applies the filler-class discipline
/// ([`announce_discipline_holds`]) to what the holes caught: the ordinary
/// call. `None` is **probe mode** — a structural "is this constituent that
/// entry's wording", used by the discipline itself to classify a filler. Its
/// `index` is discarded by the caller, so any value does.
fn try_entry(
    index: usize,
    entry: &Entry,
    target: &View,
    announcements: Option<&[&Entry]>,
) -> Option<Matched> {
    let mut best: Option<(usize, Matched)> = None;
    for (pattern_depth, pattern) in unwrappings(&entry.frame.tree).into_iter().enumerate() {
        if matches!(pattern, View::Hole { class, .. } if *class != HoleClass::SelfRef) {
            continue;
        }
        for (target_depth, candidate) in unwrappings(target).into_iter().enumerate() {
            if !roots_align(pattern, candidate) {
                continue;
            }
            let mut normalized = candidate.clone();
            neutralize_agreement(&mut normalized, &entry.frame.agreement, pattern_depth);
            let mut attempt = Attempt {
                bindings: HashMap::new(),
                claimed: 0,
            };
            if !match_node(pattern, &normalized, &mut attempt) || attempt.claimed == 0 {
                continue;
            }
            if let Some(announcements) = announcements
                && !announce_discipline_holds(entry, &attempt.bindings, announcements)
            {
                continue;
            }
            // Best alignment: the one that accounts for the most of the
            // target, and among equals the one that discarded the least
            // wrapping to get there.
            let total_peel = pattern_depth + target_depth;
            let rank = (attempt.claimed, Reverse(total_peel));
            let better = best
                .as_ref()
                .is_none_or(|(peel, found)| rank > (found.claimed, Reverse(*peel)));
            if better {
                best = Some((
                    total_peel,
                    Matched {
                        entry: index,
                        claimed: attempt.claimed,
                        guards: entry.frame.guards.len(),
                        bindings: attempt.bindings,
                    },
                ));
            }
        }
    }
    best.map(|(_, matched)| matched)
}

/// Whether `target` **is** `entry`'s own tree with its original arguments
/// filling its holes: every node outside a hole identical, and every captured
/// parameter recovering to the corresponding value in `args`.
///
/// This is the render direction's cross-check, and it is deliberately this
/// module's function rather than a comparison written over there. Rendering
/// substitutes into the frame's *text* and re-parses the whole string
/// ([`crate::render`]'s module doc explains why there is no tree-surgery
/// path), so nothing about that round trip guarantees the parser rebuilt the
/// frame's own constituency: a filler whose text coordinates, or trails a
/// modifier the frame's next word can attach to, re-brackets the sentence
/// around it into a well-formed parse of something else. Asking the question
/// here means it is asked with the same neutralizations a match uses —
/// citation-form agreement rewritten through the compiler's own normalizer,
/// surface-only fields excluded ([`surface_only_fields`]) — so a render is
/// not rejected for the two differences a *match* is defined to ignore.
///
/// Compared at depth 0 on both sides, with no wrapper peeling: the frame was
/// parsed at its own category and the substituted text is parsed back at that
/// same category, so the two roots are the same node kind by construction and
/// a difference there is a real one.
pub(crate) fn frame_reassembles(
    entry: &Entry,
    args: &[Recovered],
    lexicon: &Lexicon,
    position: FramePosition,
    target: &View,
) -> bool {
    let mut normalized = target.clone();
    neutralize_agreement(&mut normalized, &entry.frame.agreement, 0);
    let mut attempt = Attempt {
        bindings: HashMap::new(),
        claimed: 0,
    };
    match_node(&entry.frame.tree, &normalized, &mut attempt)
        && args.len() == entry.arity()
        && args.iter().enumerate().all(|(param, expected)| {
            if entry.frame.hole_for_param(param).is_some() {
                recover_argument(entry, &attempt.bindings, param, lexicon, position, 0) == *expected
            } else {
                entry
                    .frame
                    .guards
                    .iter()
                    .find(|guard| guard.param == param)
                    .is_some_and(|guard| {
                        recovered_guard_holds(guard, Some(expected), lexicon.macros())
                    })
            }
        })
}

/// Whether every hole in `entry`'s frame caught a filler of the class it
/// accepts — **the disambiguation rule**, and the reason two entries may
/// carry the same English frame text over different values.
///
/// The rule is one equality, read in both directions:
///
/// - a hole the frame marked announced
///   ([`CompiledFrame::announces`](crate::CompiledFrame::announces)) accepts
///   **only** an announcement filler;
/// - an unmarked hole **rejects** an announcement filler.
///
/// So the announced wording ("… deals N damage to any target", whose value
/// hoists the recipient into a `targets:` list and reads it back as
/// `Target(0)`) and the inline wording ("… deals N damage to it", whose value
/// names the recipient in place) cover disjoint English rather than competing
/// for the same sentences. Magic's own targeting rules are what make the two
/// domains disjoint rather than merely declared so: a targeted recipient is
/// always announced [CR#601.2c], so no sentence is both.
///
/// The rule is decided **semantic-blind** — matching never sees the card's
/// RON, only its English — which is what lets it run during ingestion of text
/// that has no semantic side at all.
///
/// # What counts as an announcement is catalog data
///
/// A filler is an announcement iff it matches an entry that declares itself
/// one ([`Entry::announcement`](crate::Entry::announcement)) — the two
/// pro-form wordings a `targets:` list holds. Deliberately not a node shape
/// hardcoded here: which determiner spells an announcement is a fact about
/// Magic's editorial English, which the catalog is the place to state, and a
/// lexicon that declares no announcements consequently has no discipline to
/// enforce and matches exactly as it did before the mark existed.
///
/// The probe runs with the discipline **off** (`announcements: None`), for
/// two reasons: it asks only "is this constituent that pro-form", which is a
/// question about the filler's own surface rather than about what fills the
/// pro-form's holes, and switching it off is what bounds the mutual recursion
/// between the two checks at one level.
///
/// A scalar binding (a numeral, a P/T half) is not a constituent and cannot
/// be an announcement, so it satisfies an unmarked hole and fails a marked
/// one, which is what a `Count` hole marked announced would deserve.
fn announce_discipline_holds(
    entry: &Entry,
    bindings: &HashMap<usize, Binding>,
    announcements: &[&Entry],
) -> bool {
    entry.frame.holes.iter().all(|hole| {
        let announced = entry.frame.announces(hole);
        match bindings.get(&hole.index) {
            Some(Binding::Node(node)) => announced == is_announcement(node, announcements),
            // A scalar is never an announcement. An unvisited hole cannot
            // occur in a successful match (every hole in the pattern is
            // visited by the walk); treating it as non-announcement keeps the
            // check total either way.
            Some(Binding::Scalar(_)) | None => !announced,
        }
    })
}

/// Whether `view` is a target announcement: some entry that declares itself
/// one matches it. See [`announce_discipline_holds`].
fn is_announcement(view: &View, announcements: &[&Entry]) -> bool {
    announcements
        .iter()
        .any(|entry| try_entry(0, entry, view, None).is_some())
}

/// A node and everything reachable from it by stripping newtype wrappers,
/// outermost first.
fn unwrappings(view: &View) -> Vec<&View> {
    let mut chain = vec![view];
    let mut node = view;
    while let View::Newtype { inner, .. } = node {
        node = inner;
        chain.push(node);
    }
    chain
}

/// A cheap pre-check before cloning the target: two roots can only match if
/// they name the same type and variant.
///
/// A **hole** pattern is the exception, and has to be: a hole names no type at
/// all, so type equality would reject every target that is not itself a hole.
/// A captured self-reference arrives wrapped as `Subject(…)` or
/// `Phrase::NounPhrase(…)` depending on where it sat, which is exactly the
/// wrapping a pro-form frame's hole is supposed to see through. What may
/// legitimately stand at a hole is the hole class's own business, and
/// [`match_hole`] is where that is decided; [`try_entry`] has already refused
/// the classes whose answer would be "anything".
fn roots_align(pattern: &View, target: &View) -> bool {
    matches!(pattern, View::Hole { .. })
        || (pattern.type_name() == target.type_name()
            && pattern.variant_name() == target.variant_name())
}

/// Rewrites the target's hole-driven inflections to citation form, exactly as
/// the compiler did to the frame.
///
/// The frame's [`AgreementDep`] sites are paths into its *whole* tree; a
/// pattern peeled `depth` newtype wrappers deep needs them rebased by
/// dropping that many leading [`PathStep::Inner`] steps. A site that does not
/// start that way does not address anything inside the peeled pattern and is
/// dropped.
///
/// The rewrite itself is [`crate::compile`]'s, called with its card-side
/// flag: the target has no holes for the quantity-lift rule to key off, so it
/// keys off "the sole quantity modifier" instead. Reusing the compiler's own
/// function is the point — a second implementation here would drift from the
/// citation form frames are actually compiled to.
fn neutralize_agreement(target: &mut View, agreement: &[AgreementDep], depth: usize) {
    let mut rebased: Vec<AgreementDep> = agreement
        .iter()
        .filter_map(|dep| {
            let steps = &dep.site.0;
            (steps.len() >= depth && steps[..depth].iter().all(|step| *step == PathStep::Inner))
                .then(|| AgreementDep {
                    site: TreePath(steps[depth..].to_vec()),
                    kind: dep.kind,
                    normalized: Vec::new(),
                })
        })
        .collect();
    crate::compile::normalize_all(target, &mut rebased, crate::compile::Side::Card);
}

/// The fields of `node` that carry the frame's own surface choices rather
/// than meaning, and so must not be compared.
///
/// One entry today, and it is not cosmetic. A numeric hole's witness is the
/// reserved Arabic numeral `41`, so **every** compiled frame with a count
/// hole records `Numeral::Arabic` beside it — while oracle text spells small
/// counts out ("Draw three cards"). Comparing the sibling would make
/// `Draw(3)` recoverable only from cards that happened to print a digit. The
/// spelling is the renderer's choice on the way out (which is exactly why
/// [`HoleClass::Numeral`] deliberately leaves it outside the hole) and
/// therefore carries no information on the way in.
fn surface_only_fields(
    name: &str,
    variant: Option<&str>,
    pattern: &[(&'static str, View)],
    target: &[(&'static str, View)],
) -> &'static [&'static str] {
    let numeric_hole = pattern.iter().any(|(field, value)| {
        *field == "value"
            && matches!(
                value,
                View::Hole {
                    class: HoleClass::Numeral | HoleClass::PtHalf,
                    ..
                }
            )
    });
    if name == "NumberLiteral" && numeric_hole {
        return &["numeral"];
    }
    // `CatalogAtom.spelling` is the matched input substring verbatim
    // (`Arc::from(&text[..length])`, `crates/deckmaste_english/src/
    // catalog.rs`), never the catalog's own canonical form (that is
    // `canonical`, which the parser's own `CatalogKind::case_policy()`
    // already normalizes case-insensitively) — so `spelling` is pure surface,
    // exactly the class this table exists for: it carries no information a
    // real match should be sensitive to, the same reason `NumberLiteral`'s
    // `numeral` field is excluded above ("the renderer's choice on the way
    // out ... carries no information on the way in"). Unconditional, not
    // gated on a hole nearby: unlike a numeral's spelling (which only
    // diverges from a frame's citation form when a *hole* drives it),
    // `spelling`'s case can differ from a frame's own authored text purely
    // from surface position (a solo keyword line is always capitalized,
    // line-initial, on a real card) with no hole involved at all.
    if name == "CatalogAtom" {
        return &["spelling"];
    }
    // Case belongs to the syntactic host, not the referent. A standalone
    // nullary frame for an invariant pronoun such as `you` selects one case
    // when parsed without a host, but must match that same spelling in either
    // a subject or object position. Keep case load-bearing for identities
    // whose forms differ (`he`/`him`, `they`/`them`).
    if name == "NounPhrase"
        && variant == Some("Pronoun")
        && case_invariant_pronouns_match(pattern, target)
    {
        return &["case"];
    }
    &[]
}

/// Whether both nodes carry the same pronoun identity and that identity has
/// one surface for both grammatical cases.
fn case_invariant_pronouns_match(
    pattern: &[(&'static str, View)],
    target: &[(&'static str, View)],
) -> bool {
    let (Some(pattern), Some(target)) = (find(pattern, "pronoun"), find(target, "pronoun")) else {
        return false;
    };
    if pattern != target {
        return false;
    }
    let Some(pronoun) = Pronoun::ALL
        .into_iter()
        .find(|pronoun| crate::view::of(pronoun) == *pattern)
    else {
        return false;
    };
    let vocabulary = Vocabulary::new();
    let surface = |case| vocabulary.render_pronoun(PronounInstance { pronoun, case });
    matches!(
        (surface(PronounCase::Subject), surface(PronounCase::Object)),
        (Some(subject), Some(object)) if subject == object
    )
}

/// The serde scalar kinds a numeric hole may have captured.
const INTEGER_KINDS: [&str; 8] = ["u8", "u16", "u32", "u64", "i8", "i16", "i32", "i64"];

fn match_node(pattern: &View, target: &View, attempt: &mut Attempt) -> bool {
    match pattern {
        View::Hole { index, class } => match_hole(*index, class, target, attempt),
        View::Node {
            name,
            variant,
            fields,
        } => {
            let View::Node {
                name: target_name,
                variant: target_variant,
                fields: target_fields,
            } = target
            else {
                return false;
            };
            if name != target_name || variant != target_variant {
                return false;
            }
            attempt.claimed += 1;
            match_fields(name, *variant, fields, target_fields, attempt)
        }
        View::Newtype {
            name,
            variant,
            inner,
        } => {
            let View::Newtype {
                name: target_name,
                variant: target_variant,
                inner: target_inner,
            } = target
            else {
                return false;
            };
            if name != target_name || variant != target_variant {
                return false;
            }
            attempt.claimed += 1;
            match_node(inner, target_inner, attempt)
        }
        View::Unit { .. } | View::Scalar { .. } | View::Absent => {
            let same = pattern == target;
            attempt.claimed += usize::from(same);
            same
        }
        View::Seq(items) => {
            let View::Seq(target_items) = target else {
                return false;
            };
            if items.len() != target_items.len() {
                return false;
            }
            attempt.claimed += 1;
            items
                .iter()
                .zip(target_items)
                .all(|(item, target_item)| match_node(item, target_item, attempt))
        }
        View::Map(entries) => {
            let View::Map(target_entries) = target else {
                return false;
            };
            if entries.len() != target_entries.len() {
                return false;
            }
            attempt.claimed += 1;
            entries
                .iter()
                .zip(target_entries)
                .all(|(pair, target_pair)| {
                    match_node(&pair.0, &target_pair.0, attempt)
                        && match_node(&pair.1, &target_pair.1, attempt)
                })
        }
    }
}

/// Compares one node's fields, binding a field-slice hole if the pattern has
/// one.
///
/// **Field sets may differ, and legitimately.** A field-slice hole binds a
/// *partial* node — only the fields its `claimed` list names — so a partial
/// node fed back in as a target has fewer fields than the frame pattern it is
/// matched against. A field present on one side only therefore matches iff it
/// is vacuous ([`View::is_vacuous`]): an absent determiner or an empty
/// complement list is not material, and neither side is claiming anything by
/// leaving it out. Anything else is a real difference and fails.
fn match_fields(
    name: &'static str,
    variant: Option<&'static str>,
    pattern: &[(&'static str, View)],
    target: &[(&'static str, View)],
    attempt: &mut Attempt,
) -> bool {
    let skipped = surface_only_fields(name, variant, pattern, target);

    // A field-slice hole is spelled once per claimed field, all with the same
    // index; `claimed` — never a fixed three-field guess — is the authority
    // on which fields belong to the hole, and binding is all-or-none across
    // exactly that set.
    let slice = pattern.iter().find_map(|(_, value)| match value {
        View::Hole {
            index,
            class: class @ HoleClass::FieldSlice { claimed },
        } => Some((*index, class, claimed)),
        _ => None,
    });
    if let Some((index, class, claimed)) = slice {
        let mut captured = Vec::with_capacity(claimed.len());
        for field in claimed {
            let Some(value) = find(target, field) else {
                return false;
            };
            captured.push((*field, value.clone()));
        }
        if !attempt.bind(
            index,
            class,
            Binding::Node(View::Node {
                name,
                variant,
                fields: captured,
            }),
        ) {
            return false;
        }
    }
    let claimed: &[&str] = slice.map_or(&[], |(_, _, claimed)| claimed);

    for (field, value) in pattern {
        if claimed.contains(field) || skipped.contains(field) {
            continue;
        }
        match find(target, field) {
            Some(target_value) => {
                if !match_node(value, target_value, attempt) {
                    return false;
                }
            }
            None if value.is_vacuous() => {}
            None => return false,
        }
    }
    for (field, value) in target {
        if claimed.contains(field) || skipped.contains(field) {
            continue;
        }
        if find(pattern, field).is_none() && !value.is_vacuous() {
            return false;
        }
    }
    true
}

/// One named field of a flat node, by name.
fn find<'fields>(fields: &'fields [(&'static str, View)], wanted: &str) -> Option<&'fields View> {
    fields
        .iter()
        .find_map(|(field, value)| (*field == wanted).then_some(value))
}

fn match_hole(index: usize, class: &HoleClass, target: &View, attempt: &mut Attempt) -> bool {
    match class {
        HoleClass::Subtree => attempt.bind(index, class, Binding::Node(target.clone())),
        HoleClass::Numeral | HoleClass::PtHalf => match target {
            View::Scalar { kind, repr } if INTEGER_KINDS.contains(kind) => {
                attempt.bind(index, class, Binding::Scalar(repr.clone()))
            }
            _ => false,
        },
        // The one hole class that *narrows* what may stand at it rather than
        // accepting whatever is there, so its match is content the frame
        // accounted for and counts toward `claimed` — which is what lets a
        // frame consisting of nothing but `~` clear the `claimed == 0` bar
        // that (rightly) stops a contentless frame matching everything.
        HoleClass::SelfRef => {
            if !is_self_reference(target) {
                return false;
            }
            attempt.claimed += 1;
            attempt.bind(index, class, Binding::Node(target.clone()))
        }
        // Reached only if a field-slice hole turns up somewhere other than as
        // a field of the node whose fields it claims, which relocation cannot
        // produce. `match_fields` is where the real handling lives.
        HoleClass::FieldSlice { .. } => false,
    }
}

#[cfg(test)]
mod tests {
    use std::fs::File;
    use std::io::BufRead;
    use std::io::BufReader;
    use std::path::Path;
    use std::path::PathBuf;
    use std::sync::LazyLock;

    use deckmaste_english::CatalogKind;
    use deckmaste_english::Catalogs;
    use deckmaste_english::FragmentKind;
    use deckmaste_english::parse_fragment;
    use deckmaste_plugin::plugin::Plugin;
    use macro_ron::MacroSet;
    use macro_ron::frames::FrameSpec;
    use macro_ron::frames::load_constructor_frames;

    use super::*;
    use crate::CompiledFrame;
    use crate::lexicon::Origin;
    use crate::view;

    fn plugin_dir() -> PathBuf {
        Path::new(env!("CARGO_MANIFEST_DIR")).join("../../plugins/builtin")
    }

    /// The generated, CR-derived catalogs the corpus tooling parses real
    /// oracle text against. Load-bearing rather than incidental:
    /// `Catalogs::default()` has zero entries in every catalog, so
    /// `KeywordLine` frames and catalog nouns such as "creature" cannot parse
    /// against it at all. Only ever reached from
    /// `#[cfg_attr(not(gen_catalogs), ignore)]` tests, so `data/gen/catalogs`
    /// is guaranteed present when this runs (build.rs sets the `gen_catalogs`
    /// cfg from the directory's presence).
    fn real_catalogs() -> Catalogs {
        let dir = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../data/gen/catalogs");
        let load = |name: &str| -> Vec<String> {
            let path = dir.join(format!("{name}.txt"));
            let file = File::open(&path)
                .unwrap_or_else(|error| panic!("opening {}: {error}", path.display()));
            BufReader::new(file)
                .lines()
                .collect::<std::io::Result<Vec<_>>>()
                .unwrap_or_else(|error| panic!("reading {}: {error}", path.display()))
        };
        Catalogs::default()
            .with_catalog(CatalogKind::KeywordAbility, load("keyword-abilities"))
            .with_catalog(CatalogKind::KeywordAction, load("keyword-actions"))
            .with_catalog(CatalogKind::AbilityWord, load("ability-words"))
            .with_catalog(CatalogKind::ArtifactType, load("artifact-types"))
            .with_catalog(CatalogKind::BattleType, load("battle-types"))
            .with_catalog(CatalogKind::CreatureType, load("creature-types"))
            .with_catalog(CatalogKind::EnchantmentType, load("enchantment-types"))
            .with_catalog(CatalogKind::LandType, load("land-types"))
            .with_catalog(CatalogKind::PlaneswalkerType, load("planeswalker-types"))
            .with_catalog(CatalogKind::SpellType, load("spell-types"))
            .with_catalog(CatalogKind::Supertype, load("supertypes"))
            .with_catalog(CatalogKind::CardType, load("card-types"))
    }

    struct Fixture {
        catalogs: Catalogs,
        macros: MacroSet,
        lexicon: Lexicon,
    }

    /// The real pilot lexicon, assembled once: ten framed macros (fourteen
    /// authored frames — `Draw` has two and `Draws` four) plus the seven
    /// constructor-catalog entries.
    fn fixture() -> &'static Fixture {
        static FIXTURE: LazyLock<Fixture> = LazyLock::new(|| {
            let plugin = Plugin::load_with_sibling_prelude(plugin_dir())
                .unwrap_or_else(|error| panic!("loading plugin: {error:#}"));
            let catalogs = real_catalogs();
            let constructors = load_constructor_frames(&plugin_dir().join("frames"))
                .unwrap_or_else(|error| panic!("loading constructor frames: {error:#}"));
            let lexicon = Lexicon::assemble(&plugin.macros, &constructors, &catalogs)
                .unwrap_or_else(|error| panic!("assembling the lexicon: {error:#}"));
            Fixture {
                catalogs,
                macros: plugin.macros,
                lexicon,
            }
        });
        &FIXTURE
    }

    /// The `View` of a cleanly-parsed fragment — the shape a caller hands
    /// [`unify`]. A parse that is not clean is a broken fixture, not a
    /// matching outcome, so it panics rather than degrading the test.
    fn parse(text: &str, kind: FragmentKind, name: &str) -> View {
        let report = parse_fragment(text, &fixture().catalogs, kind, name, false);
        assert!(
            report.clean(),
            "fixture text {text:?} must parse cleanly at {kind:?}: {:?}",
            report.diagnostics()
        );
        view::of(
            &report
                .into_fragment()
                .expect("a clean report has a fragment"),
        )
    }

    fn compiled(text: &str, kind: FragmentKind, params: &[&str]) -> CompiledFrame {
        let params: Vec<String> = params.iter().map(|param| (*param).to_string()).collect();
        let fixture = fixture();
        crate::compile(
            &FrameSpec::bare(text),
            kind,
            &params,
            &fixture.catalogs,
            &fixture.macros,
        )
        .unwrap_or_else(|error| panic!("compiling {text:?} at {kind:?}: {error:#}"))
    }

    /// A one-frame lexicon, for the shapes the pilot corpus deliberately does
    /// not author (the `~` self-reference cost frame, a non-linear frame).
    fn sole(name: &str, frame: CompiledFrame, params: &[&str]) -> Lexicon {
        Lexicon::from_entries(
            vec![crate::Entry {
                name: name.to_string(),
                params: params.iter().map(|param| (*param).to_string()).collect(),
                frame_index: 0,
                origin: Origin::Macro,
                frame,
                body: None,
                announcement: false,
            }],
            // None of these hand-built fixture frames carry a guard, so the
            // bare core reader (no plugin macros) is enough.
            guard::core_reader().clone(),
        )
    }

    fn invocation(recovered: &Recovered) -> (&str, &[Recovered]) {
        match recovered {
            Recovered::Invocation { entry, args, .. } => (entry.as_str(), args.as_slice()),
            other => panic!("expected an invocation, got {other:#?}"),
        }
    }

    fn literal(text: &str) -> Recovered {
        Recovered::Literal(text.to_string())
    }

    /// A lexicon over the **real** macro table and catalogs with a synthetic
    /// constructor catalog in place of the shipped one — the fixture a
    /// `body:` or announce-marked entry is exercised in without editing the
    /// catalog every other test in this file reads.
    ///
    /// Assembled, not hand-built: `Lexicon::assemble` is what enforces the
    /// authored-identity invariant, and the entries below deliberately differ
    /// in *name*, so nothing here leans on
    /// [`same_authored_frame`]'s tie carve-out — the announce discipline is
    /// what has to separate two entries carrying one wording.
    fn lexicon_over(constructors: &[macro_ron::frames::ConstructorFrames]) -> Lexicon {
        let fixture = fixture();
        Lexicon::assemble(&fixture.macros, constructors, &fixture.catalogs)
            .unwrap_or_else(|error| panic!("assembling the fixture catalog: {error:#}"))
    }

    /// One synthetic catalog entry, spelled the way a catalog file spells it.
    fn catalog_entry(source: &str) -> macro_ron::frames::ConstructorFrames {
        ron::Options::default()
            .from_str(source)
            .unwrap_or_else(|error| panic!("reading catalog entry {source}: {error}"))
    }

    /// The canonical `View` of a RON spelling read at `ron_type` — through
    /// [`guard::normalize_source`], the one normalizer, which is also what
    /// the ground-truth gate compares with.
    fn value_of(ron_type: &str, source: &str) -> View {
        guard::normalize_source(&fixture().macros, ron_type, source)
            .unwrap_or_else(|error| panic!("normalizing {source} as {ron_type}: {error:#}"))
    }

    /// What a recovery denotes: its emission, normalized. Panics rather than
    /// degrading, because an incomplete recovery is a broken fixture here.
    fn recovered_value(recovered: &Recovered, ron_type: &str) -> View {
        let emitted = recovered
            .to_ron(&fixture().macros)
            .unwrap_or_else(|error| panic!("emitting {recovered:#?}: {error:#}"));
        value_of(ron_type, &emitted)
    }

    // -- the brief's four ---------------------------------------------------

    /// The constructor catalog's three-hole sentence frame, recovered whole:
    /// a `Subtree` subject, a `Numeral` count, and a `Subtree` recipient.
    ///
    /// Argument 0 is the card naming itself and recovers as the RON constant
    /// `This`, through the catalog's nullary pro-form entry. Argument 2 comes
    /// back as a residual because nothing in the pilot lexicon frames the
    /// bare pronoun "it" as a `Reference` — recovery is still total, which is
    /// the point.
    #[test]
    #[cfg_attr(not(gen_catalogs), ignore = "needs generated data/gen/catalogs")]
    fn deal_damage_recovers_all_three_args() {
        let target = parse(
            "Lightning Bolt deals 3 damage to it.",
            FragmentKind::Sentence,
            "Lightning Bolt",
        );
        let recovered = unify(&target, &fixture().lexicon, FramePosition::Main);
        let (entry, args) = invocation(&recovered);
        assert_eq!(entry, "DealDamage");
        assert_eq!(args.len(), 3, "one argument per declared param: {args:#?}");

        let (subject, subject_args) = invocation(&args[0]);
        assert_eq!(subject, "This", "the subject is the card itself");
        assert!(subject_args.is_empty(), "`This` is a nullary constant");
        assert_eq!(args[1], literal("3"));
        let Recovered::Residual(recipient) = &args[2] else {
            panic!("arg 2 is the unlexicalized recipient: {:#?}", args[2]);
        };
        assert!(
            recipient
                .walk()
                .iter()
                .any(|(_, node)| node.variant_name() == Some("Pronoun")),
            "arg 2 is the whole `it` nominal: {recipient:#?}"
        );
    }

    /// The end-to-end case the review addendum was minted for, in the shape a
    /// ground-truth comparison actually needs: the argument the card's own
    /// subject fills recovers as a **structure**, `This`, comparable against
    /// the semantic RON — not as a residual the comparison cannot read.
    ///
    /// Asserted on the whole `Recovered` rather than on the entry name, so it
    /// pins the nullary shape and the absence of a spurious self-ambiguity
    /// (the pro-form frame is registered at two categories) as well.
    #[test]
    #[cfg_attr(not(gen_catalogs), ignore = "needs generated data/gen/catalogs")]
    fn a_self_referential_subject_recovers_as_the_this_constant() {
        let target = parse(
            "Lightning Bolt deals 3 damage to any target.",
            FragmentKind::Sentence,
            "Lightning Bolt",
        );
        let recovered = unify(&target, &fixture().lexicon, FramePosition::Main);
        let (_, args) = invocation(&recovered);
        assert_eq!(
            args[0],
            Recovered::Invocation {
                entry: "This".to_string(),
                args: Vec::new(),
                ambiguities: Vec::new(),
                body: None,
            },
            "two registrations of one semantic frame are one reading, not a tie"
        );

        // The same constant, recovered from the other self-reference surface.
        let demonstrative = parse("this creature", FragmentKind::Nominal, "");
        assert_eq!(
            invocation(&unify(
                &demonstrative,
                &fixture().lexicon,
                FramePosition::Main
            ))
            .0,
            "This"
        );
    }

    /// The blast radius of admitting a bare-hole pattern, pinned from both
    /// sides. A frame that peels down to one hole may match only when the hole
    /// *discriminates*: a `SelfRef` hole accepts a self-reference and nothing
    /// else, while a `Subtree` hole accepts anything and would make a
    /// contentless frame match everywhere.
    ///
    /// Both halves are driven at the same alignment — a *captured*
    /// constituent, which is the only place a bare-hole pattern is reachable
    /// (against a whole `Fragment` the unpeeled pattern aligns first).
    #[test]
    #[cfg_attr(not(gen_catalogs), ignore = "needs generated data/gen/catalogs")]
    fn a_bare_hole_frame_matches_only_when_its_hole_discriminates() {
        // A captured constituent: the payload of a fragment, with no
        // `Fragment` wrapper of its own.
        let captured = |text: &str, kind, name: &str| -> View {
            let View::Newtype { inner, .. } = parse(text, kind, name) else {
                unreachable!("a fragment is always a newtype wrapper")
            };
            *inner
        };

        let pro_form = sole("This", compiled("~", FragmentKind::Nominal, &[]), &[]);
        let discriminating = captured("this creature", FragmentKind::Nominal, "");
        assert_eq!(
            invocation(&unify(&discriminating, &pro_form, FramePosition::Main)).0,
            "This",
            "a bare `SelfRef` hole is admissible: its predicate is the frame's content"
        );
        // ... and it is a predicate, not a wildcard.
        let ordinary = captured("target creature", FragmentKind::Nominal, "");
        assert!(matches!(
            unify(&ordinary, &pro_form, FramePosition::Main),
            Recovered::Residual(_)
        ));

        // The guard that must not have been reopened: a frame whose whole
        // text is one ordinary param hole claims nothing and constrains
        // nothing, so it cannot match a captured constituent at all.
        let wildcard = sole(
            "Transparent",
            compiled("<Param(0)>", FragmentKind::Nominal, &["Predicate"]),
            &["Predicate"],
        );
        assert_eq!(
            wildcard.entries()[0].frame.holes[0].class,
            HoleClass::Subtree,
            "fixture check: this frame really is a bare non-`SelfRef` hole"
        );
        for target in [&discriminating, &ordinary] {
            assert!(
                matches!(
                    unify(target, &wildcard, FramePosition::Main),
                    Recovered::Residual(_)
                ),
                "a bare `Subtree` hole must stay unmatchable: {target:#?}"
            );
        }
    }

    /// The D8 guard case: the imperative frame holes only its count, and its
    /// subject comes back from the guard — as the spelling the catalog
    /// authored, not the expanded canonical form.
    #[test]
    #[cfg_attr(not(gen_catalogs), ignore = "needs generated data/gen/catalogs")]
    fn imperative_draw_recovers_the_you_guard() {
        let target = parse("Draw three cards.", FragmentKind::Sentence, "");
        let recovered = unify(&target, &fixture().lexicon, FramePosition::Main);
        let (entry, args) = invocation(&recovered);
        assert_eq!(
            entry, "Draws",
            "the guarded frame is the more specific match: {recovered:#?}"
        );
        assert_eq!(args, [literal("You"), literal("3")]);
    }

    /// D7's field slice, both halves of it: the `Target` entry's frame owns
    /// the determiner, the hole claims `modifiers`+`head`+`complements`, and
    /// the partial node it binds is itself matched — by `Creature`, whose own
    /// frame is a full four-field nominal.
    #[test]
    #[cfg_attr(not(gen_catalogs), ignore = "needs generated data/gen/catalogs")]
    fn filter_slice_hole_matches_inside_target() {
        let target = parse("target creature", FragmentKind::Nominal, "");
        let recovered = unify(&target, &fixture().lexicon, FramePosition::Main);
        let (entry, args) = invocation(&recovered);
        assert_eq!(entry, "Target");
        assert_eq!(args.len(), 2);
        // Param 0 is guarded, not holed: `Exactly(1)`, exactly as authored.
        assert_eq!(args[0], literal("Exactly(1)"));
        let (inner, inner_args) = invocation(&args[1]);
        assert_eq!(inner, "Creature");
        assert!(inner_args.is_empty(), "`Creature` is nullary");
        assert!(!recovered.has_residual(), "fully recovered: {recovered:#?}");
    }

    #[test]
    #[cfg_attr(not(gen_catalogs), ignore = "needs generated data/gen/catalogs")]
    fn unmatched_text_is_residual_not_error() {
        let target = parse("The sky is blue.", FragmentKind::Sentence, "");
        let recovered = unify(&target, &fixture().lexicon, FramePosition::Main);
        assert_eq!(
            recovered,
            Recovered::Residual(target),
            "an unframed sentence comes back whole, not as an error"
        );
    }

    // -- the rules the brief's four do not reach ----------------------------

    /// The complement-owning slice shape, whose `claimed` is
    /// `["modifiers", "head"]` — two fields, not three. A consumer that
    /// assumed the three-field set would bind the frame's own "you control"
    /// into the argument and `Creature` would not match what came out.
    #[test]
    #[cfg_attr(not(gen_catalogs), ignore = "needs generated data/gen/catalogs")]
    fn a_two_field_slice_binds_only_the_fields_it_claims() {
        let target = parse("creature you control", FragmentKind::Nominal, "");
        let recovered = unify(&target, &fixture().lexicon, FramePosition::Main);
        let (entry, args) = invocation(&recovered);
        assert_eq!(entry, "ControlledByYou");
        assert_eq!(args.len(), 1);
        assert_eq!(invocation(&args[0]).0, "Creature");
        assert!(!recovered.has_residual(), "{recovered:#?}");

        // The claimed set really is the two-field one, read off the compiled
        // frame rather than assumed here either.
        let entry = fixture()
            .lexicon
            .entries()
            .iter()
            .find(|entry| entry.name == "ControlledByYou")
            .expect("the pilot frames ControlledByYou");
        assert_eq!(
            entry.frame.holes[0].class,
            HoleClass::FieldSlice {
                claimed: vec!["modifiers", "head"]
            }
        );
    }

    /// A `~` hole against the `ThisCard` leaf — the form the sigil itself
    /// compiles to — with a field slice and a numeral in the same frame.
    #[test]
    #[cfg_attr(not(gen_catalogs), ignore = "needs generated data/gen/catalogs")]
    fn a_self_reference_hole_matches_the_this_card_leaf() {
        let target = parse(
            "Lightning Bolt deals 3 damage to each creature.",
            FragmentKind::Sentence,
            "Lightning Bolt",
        );
        let recovered = unify(&target, &fixture().lexicon, FramePosition::Main);
        let (entry, args) = invocation(&recovered);
        assert_eq!(
            entry, "DealsDamageToEach",
            "it claims the determiner `each` too, so it outranks the bare \
             `DealDamage` frame that also matches: {recovered:#?}"
        );
        // The `~` hole has no param, so it contributes no argument at all.
        assert_eq!(args.len(), 2);
        assert_eq!(args[0], literal("3"));
        assert_eq!(invocation(&args[1]).0, "Creature");
    }

    /// The other self-reference surface: "this permanent" is a demonstrative
    /// nominal, not a `ThisCard` leaf, and a `SelfRef` hole must accept it.
    ///
    /// Driven against a hand-compiled `Sacrifice ~` because the pilot
    /// deliberately frames `SacrificeThis` with the literal wording instead.
    /// That is an *authoring* choice; matching has
    /// to cope with the form either way, since 99.2% of the corpus's
    /// "Sacrifice this `<TYPE>`" lines are spelled this way.
    #[test]
    #[cfg_attr(not(gen_catalogs), ignore = "needs generated data/gen/catalogs")]
    fn a_self_reference_hole_matches_the_demonstrative_nominal() {
        let lexicon = sole(
            "SacrificeThis",
            compiled("Sacrifice ~", FragmentKind::Cost, &[]),
            &[],
        );
        let target = parse("Sacrifice this permanent", FragmentKind::Cost, "");
        let recovered = unify(&target, &lexicon, FramePosition::Main);
        assert_eq!(invocation(&recovered).0, "SacrificeThis");

        // And the predicate is a predicate, not a hardcoded tree: the same
        // nominal with a different head is still a self-reference, while an
        // ordinary determiner is not.
        assert!(is_self_reference(&parse(
            "this creature",
            FragmentKind::Nominal,
            ""
        )));
        assert!(!is_self_reference(&parse(
            "target creature",
            FragmentKind::Nominal,
            ""
        )));
    }

    /// A non-linear frame: one `~`, two sites. The captures are *not* equal as
    /// trees — the subject site is `Subject(ThisCard)` and the object site is
    /// `Phrase::NounPhrase(ThisCard)` — so the agreement check between them
    /// has to be denotational, and a frame like this would be unmatchable if
    /// it were raw tree equality.
    #[test]
    #[cfg_attr(not(gen_catalogs), ignore = "needs generated data/gen/catalogs")]
    fn a_non_linear_self_reference_agrees_across_differently_shaped_sites() {
        let lexicon = sole(
            "DealsDamageToSelf",
            compiled(
                "~ deals <Param(0)> damage to ~",
                FragmentKind::Sentence,
                &["Count"],
            ),
            &["Count"],
        );
        let both = parse(
            "Lightning Bolt deals 3 damage to Lightning Bolt.",
            FragmentKind::Sentence,
            "Lightning Bolt",
        );
        let recovered = unify(&both, &lexicon, FramePosition::Main);
        let (entry, args) = invocation(&recovered);
        assert_eq!(entry, "DealsDamageToSelf");
        // The `~` hole has no param, so the count is the only argument.
        assert_eq!(args, [literal("3")]);

        // The control: a second site that is not a self-reference does not
        // satisfy the hole, so the frame does not match at all.
        let other = parse(
            "Lightning Bolt deals 3 damage to each creature.",
            FragmentKind::Sentence,
            "Lightning Bolt",
        );
        assert!(matches!(
            unify(&other, &lexicon, FramePosition::Main),
            Recovered::Residual(_)
        ));
    }

    /// D5: a card's verb agreement is surface, not meaning. `GainLife`'s
    /// frame is in citation form ("gains", third singular); a second-person
    /// card ("You gain") and a third-person one ("Target player gains") must
    /// both reach it.
    #[test]
    #[cfg_attr(not(gen_catalogs), ignore = "needs generated data/gen/catalogs")]
    fn hole_driven_agreement_is_neutralized_before_comparison() {
        for text in ["You gain 3 life.", "Target player gains 3 life."] {
            let target = parse(text, FragmentKind::Sentence, "");
            let recovered = unify(&target, &fixture().lexicon, FramePosition::Main);
            let (entry, args) = invocation(&recovered);
            assert_eq!(entry, "GainLife", "{text}: {recovered:#?}");
            assert_eq!(args.len(), 2);
            assert_eq!(args[1], literal("3"), "{text}");
        }

        // The plural head is the other half of the side table: the frame says
        // "card" (citation), the card says "cards".
        let target = parse("Draw three cards.", FragmentKind::Sentence, "");
        assert_eq!(
            invocation(&unify(&target, &fixture().lexicon, FramePosition::Main)).0,
            "Draws"
        );
    }

    /// Numeral orthography is the frame's, not the filler's: a compiled frame
    /// always records `Arabic` (its witness is the numeral `41`), so a card
    /// that spells its count out has to reach the same recovery.
    #[test]
    #[cfg_attr(not(gen_catalogs), ignore = "needs generated data/gen/catalogs")]
    fn spelled_out_and_arabic_counts_recover_the_same_literal() {
        let spelled = unify(
            &parse("Draw three cards.", FragmentKind::Sentence, ""),
            &fixture().lexicon,
            FramePosition::Main,
        );
        let digits = unify(
            &parse("Draw 3 cards.", FragmentKind::Sentence, ""),
            &fixture().lexicon,
            FramePosition::Main,
        );
        assert_eq!(invocation(&spelled).1, [literal("You"), literal("3")]);
        assert_eq!(invocation(&spelled).1, invocation(&digits).1);
    }

    /// A guard's `position` key is consulted at match time: the `Draws`
    /// imperative frame is `position: Main`, so in a trigger's subordinate
    /// clause it is not a candidate and the unguarded `Draw` wins instead.
    #[test]
    #[cfg_attr(not(gen_catalogs), ignore = "needs generated data/gen/catalogs")]
    fn a_guards_position_key_gates_the_frame() {
        let target = parse("Draw three cards.", FragmentKind::Sentence, "");
        let main = unify(&target, &fixture().lexicon, FramePosition::Main);
        let trigger = unify(&target, &fixture().lexicon, FramePosition::Trigger);
        assert_eq!(invocation(&main).0, "Draws");
        assert_eq!(invocation(&trigger).0, "Draw");
        assert_eq!(invocation(&trigger).1, [literal("3")]);
        assert!(
            trigger.ambiguities().is_empty(),
            "gating leaves exactly one candidate: {trigger:#?}"
        );
    }

    /// Two frames really do render "Draw three cards." — `Draw`'s own and
    /// `Draws`' guarded one. The tie is broken deterministically (the guarded
    /// frame is more specific) and the loser is recorded rather than dropped.
    #[test]
    #[cfg_attr(not(gen_catalogs), ignore = "needs generated data/gen/catalogs")]
    fn competing_root_matches_are_recorded_as_an_ambiguity() {
        let target = parse("Draw three cards.", FragmentKind::Sentence, "");
        let recovered = unify(&target, &fixture().lexicon, FramePosition::Main);
        let ambiguities = recovered.ambiguities();
        assert_eq!(ambiguities.len(), 1, "{recovered:#?}");
        let report = ambiguities[0];
        assert!(report.contains("`Draws`[0]"), "{report}");
        assert!(report.contains("`Draw`[0]"), "{report}");
        assert!(report.contains("chose `Draws`[0]"), "{report}");
    }

    /// A subject hole recurses: the filler is itself an invocation, recovered
    /// from a frame at a different category.
    #[test]
    #[cfg_attr(not(gen_catalogs), ignore = "needs generated data/gen/catalogs")]
    fn a_subtree_hole_recurses_into_a_nested_entry() {
        // "artifact", not "player": a fix-round finding gave `Player` its own
        // frame (`filter/Player.ron` — `DealsDamageToEach`'s own recipient
        // param needed it to recover "each player" at all), so "player" no
        // longer demonstrates a residual — "artifact" still does, and
        // exercises the exact same field-slice recursion path.
        let target = parse(
            "Target artifact draws three cards.",
            FragmentKind::Sentence,
            "",
        );
        let recovered = unify(&target, &fixture().lexicon, FramePosition::Main);
        let (entry, args) = invocation(&recovered);
        assert_eq!(entry, "Draws");
        assert_eq!(args.len(), 2);
        let (subject, subject_args) = invocation(&args[0]);
        assert_eq!(subject, "Target");
        assert_eq!(subject_args[0], literal("Exactly(1)"));
        // "artifact" has no frame in the pilot lexicon, so it stays residual
        // — and the residual is the *partial* node the slice bound, carrying
        // only the fields the hole claimed.
        let Recovered::Residual(rest) = &subject_args[1] else {
            panic!("{:#?}", subject_args[1]);
        };
        let View::Node { name, fields, .. } = rest else { panic!("{rest:#?}") };
        assert_eq!(*name, "NominalPhrase");
        let names: Vec<&str> = fields.iter().map(|(field, _)| *field).collect();
        assert_eq!(names, ["modifiers", "head", "complements"]);
        assert_eq!(args[1], literal("3"));
    }

    /// The positive case the rename above leaves untested otherwise:
    /// "player" now recovers cleanly through its own pro-form-like `Player`
    /// frame (`filter/Player.ron`), the same way `Creature` already did.
    #[test]
    #[cfg_attr(not(gen_catalogs), ignore = "needs generated data/gen/catalogs")]
    fn player_now_recovers_through_its_own_frame() {
        let target = parse(
            "Target player draws three cards.",
            FragmentKind::Sentence,
            "",
        );
        let recovered = unify(&target, &fixture().lexicon, FramePosition::Main);
        let (entry, args) = invocation(&recovered);
        assert_eq!(entry, "Draws");
        let (subject, subject_args) = invocation(&args[0]);
        assert_eq!(subject, "Target");
        assert_eq!(
            invocation(&subject_args[1]),
            ("Player", [].as_slice()),
            "{recovered:#?}"
        );
        assert!(!recovered.has_residual(), "{recovered:#?}");
    }

    /// A zero-hole frame at a category that needs a populated catalog.
    #[test]
    #[cfg_attr(not(gen_catalogs), ignore = "needs generated data/gen/catalogs")]
    fn a_hole_free_keyword_frame_matches() {
        let target = parse("flying", FragmentKind::KeywordLine, "");
        let recovered = unify(&target, &fixture().lexicon, FramePosition::Main);
        let (entry, args) = invocation(&recovered);
        assert_eq!(entry, "Flying");
        assert!(args.is_empty());
    }

    // -- the guard primitive ------------------------------------------------

    /// The user ruling, exercised where it is defined: satisfaction is
    /// expanded-form equality, so the guard authored `Exactly(1)` is
    /// satisfied by a card-side `Range(Some(1), Some(1))` — the same value,
    /// a different spelling.
    #[test]
    #[cfg_attr(not(gen_catalogs), ignore = "needs generated data/gen/catalogs")]
    fn guard_holds_compares_expanded_canonical_forms() {
        use deckmaste_semantics::Count;
        use deckmaste_semantics::Quantity;

        let entry = fixture()
            .lexicon
            .entries()
            .iter()
            .find(|entry| entry.name == "Target")
            .expect("the seeded catalog guards `Target`");
        let guard = &entry.frame.guards[0];
        assert_eq!(guard.source, "Exactly(1)", "the semantic spelling is kept");

        assert!(guard_holds(guard, Quantity::one()));
        assert!(guard_holds(
            guard,
            Quantity::Range(Some(Count::Literal(1)), Some(Count::Literal(1)))
        ));
        assert!(!guard_holds(
            guard,
            Quantity::Range(Some(Count::Literal(2)), None)
        ));
    }

    /// The runtime edge the ruling names: a non-ground argument makes the
    /// guard *fail*, never error. Selection then falls through to a
    /// less-specific frame and totality is preserved.
    #[test]
    #[cfg_attr(not(gen_catalogs), ignore = "needs generated data/gen/catalogs")]
    fn a_non_ground_argument_fails_a_guard_rather_than_erroring() {
        /// Serializes exactly as `macro_ron`'s free-hole term does, which is
        /// what `guard::ensure_ground` refuses.
        #[derive(serde::Serialize)]
        struct Param(u32);
        impl Expand for Param {
            fn expand_all(self) -> Self {
                self
            }
        }

        let entry = fixture()
            .lexicon
            .entries()
            .iter()
            .find(|entry| entry.name == "Target")
            .expect("the seeded catalog guards `Target`");
        assert!(!guard_holds(&entry.frame.guards[0], Param(0)));
    }

    // -- assembly -----------------------------------------------------------

    /// [`unify`]'s specificity order ends in "assembly order, so the answer
    /// is deterministic", and [`Recovered::Invocation::ambiguities`]'s own
    /// doc promises "one frame still wins, deterministically". Neither was
    /// true while `Lexicon::assemble` consumed `MacroSet::iter` unsorted:
    /// that iterator documents its order as unspecified (it follows the
    /// backing hash maps), so which frame won a tie varied between
    /// processes — the same non-determinism the round's own G5 finding 4
    /// observed from the tooling end.
    ///
    /// `assemble` therefore sorts by `(origin, name, frame_index, kind)`.
    /// Asserting the whole entry list is sorted under that key is what makes
    /// the two doc claims above true, and is stronger than re-assembling and
    /// comparing: two assemblies in *one* process read the same `MacroSet`
    /// and so would agree even unsorted.
    #[test]
    #[cfg_attr(not(gen_catalogs), ignore = "needs generated data/gen/catalogs")]
    fn assembled_entry_order_is_total_and_deterministic() {
        let entries = fixture().lexicon.entries();
        let key = |entry: &Entry| {
            (
                entry.origin,
                entry.name.clone(),
                entry.frame_index,
                crate::lexicon::kind_rank(entry.frame.kind),
            )
        };
        for pair in entries.windows(2) {
            let (left, right) = (key(&pair[0]), key(&pair[1]));
            assert!(
                left < right,
                "entries must be strictly ordered by (origin, name, frame_index, kind); \
                 {left:?} does not precede {right:?}"
            );
        }
        // Non-vacuous: the corpus really does exercise the key's higher
        // tiers — both origins, several names each, several frames under
        // one name (`Draws`). `kind` never actually breaks a tie in this
        // fixture: a constructor entry's required `kind:` and invariant 1
        // (`(name, frame_index, origin)` is unique) together mean no two
        // entries ever share every earlier tier, so it stays in the key
        // only to keep the ordering total if that ever stopped holding.
        assert!(entries.len() > 15, "{} entries", entries.len());
        assert_eq!(entries[0].origin, Origin::Macro, "macros sort first");
        assert_eq!(
            entries[entries.len() - 1].origin,
            Origin::Constructor,
            "constructors sort last"
        );
    }

    /// A macro contributes one entry per *frame* at the one category its
    /// `kinds:` resolve to — `Draws` four, and a macro registered under
    /// several macro kinds still only once.
    #[test]
    #[cfg_attr(not(gen_catalogs), ignore = "needs generated data/gen/catalogs")]
    fn assemble_registers_each_macro_frame_once_at_its_resolved_category() {
        let lexicon = &fixture().lexicon;
        let mut labels: Vec<String> = lexicon
            .entries()
            .iter()
            .filter(|entry| entry.origin == Origin::Macro)
            .map(|entry| {
                format!(
                    "{}[{}]@{:?}",
                    entry.name, entry.frame_index, entry.frame.kind
                )
            })
            .collect();
        labels.sort();
        assert_eq!(
            labels,
            [
                "ControlledByYou[0]@Nominal",
                "Creature[0]@Nominal",
                "DealsDamageToEach[0]@Sentence",
                "Draw[0]@Sentence",
                // `Draw[1]`/`Draws[2]`/`Draws[3]`: "draw a card" (indefinite,
                // the corpus's most common count) reaches no hole-bearing
                // frame — a `Numeral` hole only ever matches a *quantity*
                // determiner — so each macro carries a zero-hole literal
                // guarded on count = 1 (`Draw.ron`/`Draws.ron` carry the
                // full story).
                "Draw[1]@Sentence",
                "Draws[0]@Sentence",
                "Draws[1]@Sentence",
                "Draws[2]@Sentence",
                "Draws[3]@Sentence",
                "Flying[0]@KeywordLine",
                // `Player[0]`: `filter/Player.ron` already existed (used
                // elsewhere) with no `frames:` — exactly `Creature`'s
                // original gap, closed the same way.
                "Player[0]@Nominal",
                "Protection[0]@KeywordLine",
                "PumpThisUntilEot[0]@Sentence",
                "SacrificeThis[0]@Cost",
            ],
            "the pilot's framed macro set, or a kind resolution, changed"
        );
    }

    // -- entry bodies -------------------------------------------------------

    /// A pronoun whose subject and object forms differ keeps that distinction
    /// when a nullary nominal frame is matched recursively. This is the
    /// control against treating every `NounPhrase.case` field as surface-only:
    /// masculine `he` and `him` share one pronoun identity, but not one frame.
    #[test]
    #[cfg_attr(not(gen_catalogs), ignore = "needs generated data/gen/catalogs")]
    fn case_distinguishing_pronouns_do_not_share_a_nullary_frame() {
        let lexicon = sole("He", compiled("he", FragmentKind::Nominal, &[]), &[]);

        let recovered = unify(
            &parse("him", FragmentKind::Nominal, ""),
            &lexicon,
            FramePosition::Main,
        );

        assert!(
            matches!(recovered, Recovered::Residual(_)),
            "the subject-only `he` frame must not claim object `him`: {recovered:#?}"
        );
    }

    /// A multi-param entry whose `body:` says how its constituents ASSEMBLE
    /// into a verb whose slot order differs from the English's.
    ///
    /// This used to be the embed-sugar test: `Action::By` carried a
    /// `#[macro_ron(embed)]` default subject, so "You gain 3 life." authored
    /// as the one-argument `GainLife(3)` while the recovery emitted the
    /// two-argument `By(You, GainLife(3))` — same value, different strings.
    /// The action role reshape deleted `By` and, with it, defaulted role slots
    /// entirely (Law 2: one canonical spelling per role value), so that
    /// asymmetry no longer exists anywhere in the grammar — the surviving
    /// `#[macro_ron(embed)]` users (`ColorOrColorless::Color`,
    /// `ManaSpec::Specific`, `StatValue::Count`) are transparent single-payload
    /// embeds with no defaulted head.
    ///
    /// What still needs covering is the half that outlived it: the English has
    /// two constituents, the verb nests one inside a `LifeOp`, and the `body:`
    /// is what bridges them. Mirrors the real catalog entry at
    /// `plugins/builtin/frames/constructors.ron`.
    #[test]
    #[cfg_attr(not(gen_catalogs), ignore = "needs generated data/gen/catalogs")]
    fn body_entry_assembles_params_into_the_canonical_spelling() {
        let lexicon = lexicon_over(&[
            catalog_entry(
                r#"(constructor: "GainLife", params: ["Reference", "Count"], kind: Sentence,
                    body: ChangeLife(Param(0), Up(Param(1))),
                    frames: ["<Param(0)> gains <Param(1)> life"])"#,
            ),
            catalog_entry(r#"(constructor: "You", params: [], kind: Nominal, frames: ["you"])"#),
        ]);

        let recovered = unify(
            &parse("You gain 3 life.", FragmentKind::Sentence, ""),
            &lexicon,
            FramePosition::Main,
        );
        let (entry, args) = invocation(&recovered);
        assert_eq!(entry, "GainLife");
        assert_eq!(invocation(&args[0]).0, "You", "{recovered:#?}");
        assert_eq!(args[1], literal("3"));
        assert!(!recovered.has_residual(), "{recovered:#?}");

        assert_eq!(
            recovered.to_ron(&fixture().macros).unwrap(),
            "ChangeLife(You, Up(3))",
            "the emission is the body with its params filled — the count nests \
             inside the LifeOp, it is not a flat second argument"
        );
        assert_eq!(
            recovered_value(&recovered, "OneShotEffect"),
            value_of("OneShotEffect", "ChangeLife(You, Up(3))"),
            "the emission and the semantic spelling are one value — and now, \
             with no defaulted role slot, also one string"
        );
    }

    /// The announce-list family. English states a targeted recipient inline
    /// ("… to any target"); RON hoists it into the `targets:` announce list
    /// and reads it back positionally as `Target(0)` [CR#601.2c]. The body is
    /// what expresses that, and its non-`Param` leaves — the literal
    /// `Target(0)`, the `Targeted`/`Act` spine — are fixed material the
    /// comparison holds the recovery to exactly.
    #[test]
    #[cfg_attr(not(gen_catalogs), ignore = "needs generated data/gen/catalogs")]
    fn body_entry_matches_announce_list_authoring() {
        let lexicon = lexicon_over(&announced_damage_catalog());

        let recovered = unify(
            &parse(
                "Lightning Bolt deals 3 damage to any target.",
                FragmentKind::Sentence,
                "Lightning Bolt",
            ),
            &lexicon,
            FramePosition::Main,
        );
        let (entry, args) = invocation(&recovered);
        assert_eq!(entry, "TargetedDealDamage", "{recovered:#?}");
        assert_eq!(invocation(&args[0]).0, "This", "the subject is the card");
        assert_eq!(args[1], literal("3"));
        assert_eq!(
            invocation(&args[2]).0,
            "AnyTarget",
            "param 2 binds the announcement pro-form"
        );

        assert_eq!(
            recovered_value(&recovered, "OneShotEffect"),
            value_of(
                "OneShotEffect",
                "Targeted(targets: [AnyTarget], effect: DealDamage(This, Literal(3), Target(0)))",
            ),
            "recovered {:?}",
            recovered.to_ron(&fixture().macros),
        );

        // The literal `Target(0)` really is fixed material: an authoring that
        // reads the announce list back at a different index is a different
        // value, and the comparison says so.
        assert_ne!(
            recovered_value(&recovered, "OneShotEffect"),
            value_of(
                "OneShotEffect",
                "Targeted(targets: [AnyTarget, AnyTarget], effect: DealDamage(This, Literal(3), Target(1)))",
            )
        );
    }

    /// **The disambiguation rule**, both directions, over two entries that
    /// carry the *same* English frame text and stand for different values.
    ///
    /// The two wordings are disjoint because Magic's targeting rules make
    /// them so — a targeted recipient is always announced [CR#601.2c] — and
    /// the mechanism that enforces it is filler class: the announce-list
    /// entry marks its recipient hole `announced: [2]` and takes only an
    /// announcement, while the flat entry leaves it unmarked and takes only a
    /// non-announcement. Neither entry's name matches the other's, so
    /// `same_authored_frame`'s tie carve-out is not what separates them.
    #[test]
    #[cfg_attr(not(gen_catalogs), ignore = "needs generated data/gen/catalogs")]
    fn announce_only_hole_rejects_plain_filler_and_plain_hole_rejects_announced() {
        let lexicon = lexicon_over(&announced_damage_catalog());

        let recover = |text: &str| {
            invocation(&unify(
                &parse(text, FragmentKind::Sentence, "Lightning Bolt"),
                &lexicon,
                FramePosition::Main,
            ))
            .0
            .to_string()
        };

        assert_eq!(
            recover("Lightning Bolt deals 1 damage to it."),
            "DealDamage",
            "an unannounced recipient reaches only the flat entry"
        );
        assert_eq!(
            recover("Lightning Bolt deals 3 damage to any target."),
            "TargetedDealDamage",
            "an announced recipient reaches only the announce-list entry"
        );

        // Neither result is a tie quietly resolved: with both entries in
        // scope, each sentence has exactly one match, so nothing was recorded
        // as an ambiguity either.
        for text in [
            "Lightning Bolt deals 1 damage to it.",
            "Lightning Bolt deals 3 damage to any target.",
        ] {
            let recovered = unify(
                &parse(text, FragmentKind::Sentence, "Lightning Bolt"),
                &lexicon,
                FramePosition::Main,
            );
            assert!(
                recovered.ambiguities().is_empty(),
                "{text}: the two entries must not compete: {recovered:#?}"
            );
        }

        // And the discipline is what does it, not the wording: drop the
        // announcement declaration and the announce-only hole has nothing it
        // accepts, so the announced sentence falls to the flat entry — the
        // recovery the whole family exists to stop.
        let undeclared: Vec<_> = announced_damage_catalog()
            .into_iter()
            .map(|mut entry| {
                entry.announcement = false;
                entry
            })
            .collect();
        assert_eq!(
            invocation(&unify(
                &parse(
                    "Lightning Bolt deals 3 damage to any target.",
                    FragmentKind::Sentence,
                    "Lightning Bolt",
                ),
                &lexicon_over(&undeclared),
                FramePosition::Main,
            ))
            .0,
            "DealDamage",
        );
    }

    /// The two same-wording damage entries plus the pro-forms they need:
    /// `This` for the subject, `AnyTarget` as the declared announcement.
    fn announced_damage_catalog() -> Vec<macro_ron::frames::ConstructorFrames> {
        vec![
            catalog_entry(
                r#"(constructor: "DealDamage", params: ["Reference", "Count", "Reference"],
                    kind: Sentence,
                    frames: ["<Param(0)> deals <Param(1)> damage to <Param(2)>"])"#,
            ),
            catalog_entry(
                r#"(constructor: "TargetedDealDamage", params: ["Reference", "Count", "Reference"],
                    kind: Sentence,
                    body: Targeted(targets: [Param(2)], effect: DealDamage(Param(0), Param(1), Target(0))),
                    frames: [(text: "<Param(0)> deals <Param(1)> damage to <Param(2)>", announced: [2])])"#,
            ),
            catalog_entry(r#"(constructor: "This", params: [], kind: Nominal, frames: ["~"])"#),
            catalog_entry(
                r#"(constructor: "AnyTarget", params: [], kind: Nominal, announcement: true,
                    frames: ["any target"])"#,
            ),
        ]
    }

    /// The shipped catalog declares exactly one announcement — `AnyTarget`,
    /// the only pro-form the four `DealDamage`-arg-2 lines need. `Target`
    /// (`target <predicate>`) is deliberately left undeclared: `Draws`'s
    /// unmarked subject hole still recovers whole "target <predicate>"
    /// nominals directly, and declaring `Target` here would need a matching
    /// `TargetedDraws` body first.
    #[test]
    #[cfg_attr(not(gen_catalogs), ignore = "needs generated data/gen/catalogs")]
    fn the_shipped_catalog_declares_exactly_any_target_as_an_announcement() {
        let names: Vec<&str> = fixture()
            .lexicon
            .announcements()
            .map(|entry| entry.name.as_str())
            .collect();
        assert_eq!(names, ["AnyTarget"], "{names:?}");
    }

    /// A `body:` must hole every param its entry declares, or a recovered
    /// argument is silently dropped. The near-miss the check is really for is
    /// a body authored as a *string*, which holes nothing at all.
    #[test]
    #[cfg_attr(not(gen_catalogs), ignore = "needs generated data/gen/catalogs")]
    fn assemble_rejects_a_body_that_drops_a_declared_param() {
        for (body, why) in [
            (r"body: GainLife(Param(1))", "param 0 unholed"),
            (r#"body: "By(Param(0), GainLife(Param(1)))""#, "quoted"),
        ] {
            let entry = catalog_entry(&format!(
                r#"(constructor: "GainLife", params: ["Reference", "Count"], kind: Sentence,
                    {body}, frames: ["<Param(0)> gains <Param(1)> life"])"#
            ));
            let error = Lexicon::assemble(&fixture().macros, &[entry], &fixture().catalogs)
                .expect_err("a body that drops a declared param must be rejected");
            let message = format!("{error:#}");
            assert!(
                message.contains("never holes it"),
                "{why}: expected a dropped-param refusal, got {message}"
            );
        }
    }

    // -- the emission -------------------------------------------------------

    fn built(entry: &str, args: Vec<Recovered>) -> Recovered {
        Recovered::Invocation {
            entry: entry.to_string(),
            args,
            body: None,
            ambiguities: Vec::new(),
        }
    }

    #[test]
    fn a_fully_recovered_tree_spells_as_ron() {
        assert_eq!(
            built(
                "DealDamage",
                vec![
                    built("This", Vec::new()),
                    literal("3"),
                    built("Creature", Vec::new()),
                ],
            )
            .to_ron(guard::core_reader())
            .unwrap(),
            "DealDamage(This, 3, Creature)"
        );
    }

    /// Two residuals at *different* argument positions must produce different
    /// text. Without the path they printed byte-identically, and the round's
    /// two known lexicon gaps — plus any third, unrelated one — were a single
    /// opaque bucket.
    #[test]
    fn a_residual_names_its_argument_position_and_shows_what_it_held() {
        let deal_damage = built(
            "DealDamage",
            vec![
                built("This", Vec::new()),
                literal("3"),
                Recovered::Residual(View::Unit {
                    name: "Determiner",
                    variant: Some("Any"),
                }),
            ],
        )
        .to_ron(guard::core_reader())
        .unwrap_err();
        let gain_life = built(
            "GainLife",
            vec![
                Recovered::Residual(View::Unit {
                    name: "Pronoun",
                    variant: Some("You"),
                }),
                literal("2"),
            ],
        )
        .to_ron(guard::core_reader())
        .unwrap_err();

        let deal_damage = format!("{deal_damage:#}");
        let gain_life = format!("{gain_life:#}");
        assert!(deal_damage.contains("DealDamage arg 2"), "{deal_damage}");
        assert!(deal_damage.contains("Any"), "{deal_damage}");
        assert!(gain_life.contains("GainLife arg 0"), "{gain_life}");
        assert!(gain_life.contains("You"), "{gain_life}");
        assert_ne!(
            deal_damage, gain_life,
            "two different gaps must not print identically — that is the whole finding"
        );
    }

    #[test]
    fn a_nested_residual_reports_the_whole_argument_path() {
        let error = built(
            "DealsDamageToEach",
            vec![
                literal("4"),
                built("ControlledByYou", vec![Recovered::Residual(View::Absent)]),
            ],
        )
        .to_ron(guard::core_reader())
        .unwrap_err();
        assert!(
            format!("{error:#}").contains("DealsDamageToEach arg 1 -> ControlledByYou arg 0"),
            "{error:#}"
        );
    }

    #[test]
    fn a_top_level_residual_says_so_rather_than_naming_an_argument() {
        let error = Recovered::Residual(View::Absent)
            .to_ron(guard::core_reader())
            .unwrap_err();
        assert!(
            format!("{error:#}").contains("at the top level"),
            "{error:#}"
        );
    }

    /// The `Debug` of one unrecovered nominal runs to hundreds of characters;
    /// a page of them would bury a report. Truncation is by `char_indices`, so
    /// a multi-byte character straddling the budget cannot panic the slice —
    /// this feeds one that does straddle it.
    #[test]
    fn a_long_residual_debug_is_truncated_on_a_char_boundary() {
        let view = View::Scalar {
            kind: "str",
            repr: "é".repeat(RESIDUAL_DEBUG_BUDGET * 2),
        };
        let printed = truncated_debug(&view);
        assert!(printed.ends_with("chars total)"), "{printed}");
        assert!(
            printed.chars().count() < RESIDUAL_DEBUG_BUDGET + 40,
            "{printed}"
        );
    }

    /// A constructor entry's required `kind:` field picks its one
    /// registration category — `DealDamage`'s frame parses cleanly at four
    /// of the five categories, but it registers only at its declared
    /// `Sentence`, not all four (an unnarrowed registration costs 4-5x
    /// entries and makes `try_entry` `O(lexicon × tree)` against categories
    /// the target could never actually be rooted at).
    #[test]
    #[cfg_attr(not(gen_catalogs), ignore = "needs generated data/gen/catalogs")]
    fn a_constructor_frame_is_registered_only_at_its_declared_kind() {
        let lexicon = &fixture().lexicon;
        let kinds = |name: &str| {
            let mut kinds: Vec<String> = lexicon
                .entries()
                .iter()
                .filter(|entry| entry.origin == Origin::Constructor && entry.name == name)
                .map(|entry| format!("{:?}", entry.frame.kind))
                .collect();
            kinds.sort();
            kinds
        };
        assert_eq!(kinds("DealDamage"), ["Sentence"]);
        assert_eq!(kinds("GainLife"), ["Sentence"]);
        assert_eq!(kinds("Target"), ["Nominal"]);
        assert_eq!(kinds("This"), ["Nominal"]);

        let recovered = unify(
            &parse("target creature", FragmentKind::Nominal, ""),
            lexicon,
            FramePosition::Main,
        );
        assert!(
            recovered.ambiguities().is_empty(),
            "a single Nominal registration of `Target` must not report an ambiguity: {recovered:#?}"
        );
    }
}
