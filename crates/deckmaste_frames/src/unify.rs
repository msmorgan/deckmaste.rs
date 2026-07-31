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

use macro_ron::Expand;
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
/// compared structurally against a card's authored invocation — which is
/// exactly what the round's ground-truth recovery gate does.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Recovered {
    /// A lexicon entry matched: `entry` is the RON head symbol (a macro name
    /// or a raw constructor name), `args` are its arguments in param order.
    Invocation {
        entry: String,
        args: Vec<Recovered>,
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
///   `Determiner::Demonstrative(This)` and which carries no modifiers or
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
        if let Some(matched) = try_entry(index, entry, target) {
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
    // are one reading reached twice. A constructor frame is registered at
    // every category that accepts it (see `lexicon`), so `~` enters the
    // lexicon as both `This`@Nominal and `This`@Cost; at the depth a pro-form
    // matches, the two patterns are the identical bare hole and would report a
    // tie against themselves. An ambiguity is two frames disagreeing about
    // what the English *is*, so the report is built from the rivals that name
    // a different authored frame.
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
        .map(|param| recover_argument(entry, winner, param, lexicon, position, depth))
        .collect();
    Recovered::Invocation {
        entry: entry.name.clone(),
        args,
        ambiguities,
    }
}

/// The value of one declared param: from its hole if it has one, from its
/// guard if it does not.
fn recover_argument(
    entry: &Entry,
    matched: &Matched,
    param: usize,
    lexicon: &Lexicon,
    position: FramePosition,
    depth: usize,
) -> Recovered {
    if let Some(hole) = entry.frame.hole_for_param(param) {
        return match matched.bindings.get(&hole.index) {
            Some(Binding::Scalar(repr)) => Recovered::Literal(repr.clone()),
            Some(Binding::Node(node)) => unify_at(node, lexicon, position, depth + 1),
            // Unreachable through a successful match — every hole in the
            // pattern is visited by the walk — but recovery stays total
            // rather than panicking on a frame shape nobody has authored yet.
            None => Recovered::Residual(View::Absent),
        };
    }
    if let Some(guard) = entry.frame.guards.iter().find(|guard| guard.param == param) {
        // The AUTHORED spelling, not the expanded canonical form: the two are
        // equal as values (that is what `guard_holds` compares), and the
        // authored one is what the catalog reads like.
        return Recovered::Literal(guard.source.clone());
    }
    Recovered::Residual(View::Absent)
}

/// Whether two matches came from the same authored frame, registered twice.
///
/// Identity is the authoring, not the compiled entry: same owner name, same
/// slot in its `frames:` list, same origin. Only the per-category
/// registration can differ.
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
fn try_entry(index: usize, entry: &Entry, target: &View) -> Option<Matched> {
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
fn surface_only_fields(name: &str, fields: &[(&'static str, View)]) -> &'static [&'static str] {
    let numeric_hole = fields.iter().any(|(field, value)| {
        *field == "value"
            && matches!(
                value,
                View::Hole {
                    class: HoleClass::Numeral | HoleClass::PtHalf,
                    ..
                }
            )
    });
    if name == "NumberLiteral" && numeric_hole { &["numeral"] } else { &[] }
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
    let skipped = surface_only_fields(name, pattern);

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

    use deckmaste_cards::plugin::Plugin;
    use deckmaste_english::CatalogKind;
    use deckmaste_english::Catalogs;
    use deckmaste_english::FragmentKind;
    use deckmaste_english::parse_fragment;
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
    /// against it (see Task 6's G5 finding 3).
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

    /// The real pilot lexicon, assembled once: nine framed macros (ten
    /// frames — `Draws` has two) plus the three constructor-catalog entries.
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
        Lexicon::from_entries(vec![crate::Entry {
            name: name.to_string(),
            params: params.iter().map(|param| (*param).to_string()).collect(),
            frame_index: 0,
            origin: Origin::Macro,
            frame,
        }])
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

    // -- the brief's four ---------------------------------------------------

    /// The constructor catalog's three-hole sentence frame, recovered whole:
    /// a `Subtree` subject, a `Numeral` count, and a `Subtree` recipient.
    ///
    /// The recipient is "any target" rather than "target creature" on
    /// purpose. `to target creature` does not parse as a prepositional
    /// phrase at all — `target` is a verb in the grammar, so the parser reads
    /// `to target …` as an infinitive clause, and no `DealDamage`-shaped tree
    /// exists for it. "any target" is the wording the round's own brief names
    /// and it does parse as `to` + a nominal.
    ///
    /// Argument 0 is the card naming itself and recovers as the RON constant
    /// `This`, through the catalog's nullary pro-form entry. Argument 2 comes
    /// back as a residual because nothing in the pilot lexicon frames "any
    /// target" — recovery is still total, which is the point.
    #[test]
    fn deal_damage_recovers_all_three_args() {
        let target = parse(
            "Lightning Bolt deals 3 damage to any target.",
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
                .any(|(_, node)| node.variant_name() == Some("Any")),
            "arg 2 is the whole `any target` nominal: {recipient:#?}"
        );
    }

    /// The end-to-end case the review addendum was minted for, in the shape a
    /// ground-truth comparison actually needs: the argument the card's own
    /// subject fills recovers as a **structure**, `This`, comparable against
    /// the authored RON — not as a residual the comparison cannot read.
    ///
    /// Asserted on the whole `Recovered` rather than on the entry name, so it
    /// pins the nullary shape and the absence of a spurious self-ambiguity
    /// (the pro-form frame is registered at two categories) as well.
    #[test]
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
            },
            "two registrations of one authored frame are one reading, not a tie"
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
    /// deliberately frames `SacrificeThis` with the literal wording instead —
    /// Task 6's G5 finding 2. That finding is about *authoring*; matching has
    /// to cope with the form either way, since 99.2% of the corpus's
    /// "Sacrifice this `<TYPE>`" lines are spelled this way.
    #[test]
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
    fn a_subtree_hole_recurses_into_a_nested_entry() {
        let target = parse(
            "Target player draws three cards.",
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
        // "player" has no frame in the pilot lexicon, so it stays residual —
        // and the residual is the *partial* node the slice bound, carrying
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

    /// A zero-hole frame at a category that needs a populated catalog.
    #[test]
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
    fn guard_holds_compares_expanded_canonical_forms() {
        use deckmaste_core::Count;
        use deckmaste_core::Quantity;

        let entry = fixture()
            .lexicon
            .entries()
            .iter()
            .find(|entry| entry.name == "Target")
            .expect("the seeded catalog guards `Target`");
        let guard = &entry.frame.guards[0];
        assert_eq!(guard.source, "Exactly(1)", "the authored spelling is kept");

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

    /// A macro contributes one entry per *frame* at the one category its
    /// `kinds:` resolve to — `Draws` two, and a macro registered under
    /// several macro kinds still only once.
    #[test]
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
                "Draws[0]@Sentence",
                "Draws[1]@Sentence",
                "Flying[0]@KeywordLine",
                "Protection[0]@KeywordLine",
                "PumpThisUntilEot[0]@Sentence",
                "SacrificeThis[0]@Cost",
            ],
            "the pilot's framed macro set, or a kind resolution, changed"
        );
    }

    /// A constructor entry declares no category, and no order of trials can
    /// recover one: `target <Param(1)>` is as good an imperative sentence as
    /// a nominal, and the damage clause is as good a reduced relative as a
    /// sentence. So each constructor frame is registered at every category
    /// that accepts it and the target's own category picks — which is why
    /// `target creature` still recovers `Target` and not something else.
    #[test]
    fn a_constructor_frame_is_registered_at_every_category_that_accepts_it() {
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
        assert_eq!(
            kinds("DealDamage"),
            ["Ability", "Cost", "Nominal", "Sentence"]
        );
        assert_eq!(kinds("GainLife"), ["Ability", "Cost", "Sentence"]);
        assert_eq!(kinds("Target"), ["Ability", "Cost", "Nominal", "Sentence"]);
        // The pro-form: a lone `~` is a noun phrase or a cost line and
        // nothing else, which is why the two registrations it does get are
        // the same bare hole and must not read as an ambiguity.
        assert_eq!(kinds("This"), ["Cost", "Nominal"]);

        // The whole point of not guessing: only the `Nominal` registration of
        // `Target` can align with a nominal target, so the extra ones are
        // inert rather than competing.
        let recovered = unify(
            &parse("target creature", FragmentKind::Nominal, ""),
            lexicon,
            FramePosition::Main,
        );
        assert!(
            recovered.ambiguities().is_empty(),
            "the other registrations are rooted at other categories: {recovered:#?}"
        );
    }
}
