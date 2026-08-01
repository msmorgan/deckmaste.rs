//! The render path: a [`Recovered`] invocation in, English text out.
//!
//! This is the mirror of [`crate::unify`]. Where `unify` walks a target
//! [`View`] against every lexicon entry and reads the winner's holes back out
//! as arguments, [`render_invocation`] starts from those arguments and
//! rebuilds the text a frame would print for them: pick the most specific
//! frame whose guards the arguments satisfy (D8), substitute each hole's
//! filler, and hand the result to [`render_fragment`].
//!
//! # Why substitution is textual, not tree surgery
//!
//! [`crate::compile::CompiledFrame::tree`] is a [`View`] — the frame's parse
//! with holes punched in. It is tempting to substitute fillers into that tree
//! directly and hand the patched [`View`] to a renderer. That path does not
//! exist: [`View`] is a one-way projection ([`crate::view::of`] drives a
//! `Serialize` impl; nothing drives the reverse), and
//! `deckmaste_english::syntax`'s node types carry no `Deserialize` impl at
//! all — nor may this task add one, since `deckmaste_english` is corpus-gated
//! and must not be modified. So there is no way back from a patched `View` to
//! the typed [`Fragment`](deckmaste_english::Fragment) that
//! [`render_fragment`] requires.
//!
//! What *is* available, and is the actual mechanism here, is the frame's own
//! **authored text** ([`crate::compile::CompiledFrame::spec`]): the same
//! `<Param(i)>`/`~` sigil string [`crate::compile::compile`] witnessed and
//! parsed, still on hand. Rendering therefore mirrors compiling, in reverse:
//! substitute each sigil in that text for its filler's own rendered text
//! (recursively, all the way down to a leaf — a numeral or a guard's
//! constant), then run the *whole* assembled string through
//! [`parse_fragment`] once, at the winning frame's category, and hand the
//! resulting real, parser-built [`Fragment`] to [`render_fragment`].
//!
//! This is not a shortcut around agreement — it is why agreement comes out
//! right without this module knowing any morphology. D8's guard mechanism
//! already partitions subject-agreement classes across separate authored
//! frames: `Draws`'s imperative frame ("draw `<Param(1)>` cards") is
//! selected exactly when the subject is `You`, and its unguarded frame
//! ("`<Param(0)>` draws `<Param(1)>` cards") is written in citation form
//! (third-person singular), which is what *every* other possible subject —
//! `~`, `Target player`, any nested invocation — grammatically is. A count's
//! spelling ("three" vs the literal digit `3`) is not this module's problem
//! either: the digit is substituted verbatim, and [`parse_fragment`] +
//! [`render_fragment`]'s own round trip is what turns `3` into "three" — the
//! same way a real card's author never has to spell digits out by hand.
//!
//! # Residual fillers
//!
//! A hole whose filler recovered as [`Recovered::Residual`] carries a
//! captured [`View`] subtree with no lexicon entry behind it — by
//! definition, nothing rendered it into existence, so there is no frame text
//! to substitute *from*. [`render_residual_text`] recovers what it honestly
//! can: a bare nominal (no determiner, no modifiers, no complements) whose
//! head is a table-driven regular word
//! ([`deckmaste_english::word::Vocab::Regular`]) or an opaque/catalog lexeme,
//! both of which carry their own spelling *inside* the captured tree. Anything
//! else — a determiner, a modifier, a complement, or a hardcoded (non-table)
//! vocabulary word whose spelling lives only in a lookup table this crate must
//! not duplicate — is refused with an error rather than guessed at. This is a
//! real, intentional boundary, not an oversight: rendering it in full
//! generality is exactly the `View → Fragment` problem the module doc above
//! explains has no solution available to this crate.

use std::collections::BTreeSet;
use std::collections::HashMap;

use deckmaste_english::Catalogs;
use deckmaste_english::Numeral;
use deckmaste_english::parse_fragment;
use deckmaste_english::render_fragment;
use macro_ron::MacroSet;
use macro_ron::frames::FramePosition;

use crate::AgreeKind;
use crate::Hole;
use crate::HoleClass;
use crate::View;
use crate::compile::CompiledGuard;
use crate::guard;
use crate::lexicon::Entry;
use crate::lexicon::Lexicon;
use crate::unify::Recovered;

/// Renders the invocation a successful [`crate::unify::unify`] recovered,
/// back to English.
///
/// Uses the anonymous identity `("", false)` — see the module doc's
/// discussion of self-reference. A [`Recovered`] tree containing a `~` site
/// therefore cannot round-trip through this entry point (the substituted
/// text would have a literal gap where the card's name belongs, which fails
/// to parse); [`render_invocation_with`] takes a real identity for a caller
/// — such as the round's own gate tooling — that has one.
///
/// # Errors
/// If `inv` is not [`Recovered::Invocation`]; if no lexicon entry named its
/// head symbol has every guard satisfied by `inv`'s own arguments; if a
/// filler is (or recursively contains) a [`Recovered::Residual`] this module
/// cannot honestly reconstruct text for (see the module doc); or if the
/// fully substituted text does not parse cleanly at the winning frame's
/// category, or [`render_fragment`] itself refuses it.
pub fn render_invocation(
    inv: &Recovered,
    lexicon: &Lexicon,
    position: FramePosition,
) -> anyhow::Result<String> {
    render_invocation_with(inv, lexicon, position, &Catalogs::default(), "", false)
}

/// [`render_invocation`], generalized: a real card identity (needed for any
/// frame containing `~`) and a populated [`Catalogs`] (needed for a
/// `KeywordLine` frame — an empty catalog can never recognize a keyword atom
/// at all, per the round's own G5 finding 3). [`render_invocation`] is the
/// thin, brief-mandated wrapper over this with the anonymous placeholders;
/// this is the primitive it is built from, and the one the round's gate
/// tooling (G3/G4, in `xtask`) calls directly so it can test against real
/// canon cards under their own name and a real catalog set.
///
/// # Errors
/// See [`render_invocation`].
///
/// # Panics
/// Never in practice: the one `.expect()` in the body is guarded by an
/// `ensure!` on
/// [`FragmentReport::clean`](deckmaste_english::FragmentReport::clean)
/// immediately above it, which a clean report is defined to satisfy.
pub fn render_invocation_with(
    inv: &Recovered,
    lexicon: &Lexicon,
    position: FramePosition,
    catalogs: &Catalogs,
    name: &str,
    is_legendary: bool,
) -> anyhow::Result<String> {
    let Recovered::Invocation { entry, args, .. } = inv else {
        anyhow::bail!(
            "cannot render {inv:?}: only Recovered::Invocation is renderable at the top level"
        );
    };
    let chosen = select_frame(entry, args, lexicon, position)?;
    let substituted = render_frame_text(chosen, args, lexicon, position, name)?;
    let report = parse_fragment(
        &substituted,
        catalogs,
        chosen.frame.kind,
        name,
        is_legendary,
    );
    anyhow::ensure!(
        report.clean(),
        "rendering `{entry}`: substituted text {substituted:?} does not parse cleanly at \
         {:?}: {:?}",
        chosen.frame.kind,
        report.diagnostics(),
    );
    let fragment = report
        .into_fragment()
        .expect("a clean fragment report has a fragment");
    render_fragment(&fragment, name, is_legendary)
        .map_err(|error| anyhow::anyhow!("rendering `{entry}`: {error}"))
}

/// Picks the frame [`render_invocation_with`] (or a recursive filler render)
/// substitutes into: every lexicon entry named `entry_name`, filtered to
/// those whose `position` key (if any) matches, then to those whose every
/// guard `args` satisfies — through [`guard_satisfied`], the single
/// authority ([`crate::guard::normalize_source`]/`normalized`), exactly the
/// comparison [`crate::guard_holds`] makes one level up. A non-`Literal`
/// argument, or a `Literal` that fails to parse or expand at the guard's own
/// declared type, simply fails the guard rather than erroring — selection
/// falls through to a less-specific frame, never a mid-render error, exactly
/// [`crate::guard_holds`]'s own documented contract.
///
/// Among the survivors, [D8](../index.html) requires the *unique*
/// most-specific candidate: the one whose satisfied guard params are a
/// superset of every rival's. This is computed as an actual set relation
/// (not a bare guard *count*, which cannot tell two same-sized but
/// incomparable guard sets apart), and a non-unique result — two or more
/// candidates each maximal, dominating neither the other — is reported as an
/// error rather than resolved silently by assembly order.
///
/// # Errors
/// If no entry is named `entry_name` at `position`; if none of those has
/// every guard satisfied; or if more than one viable candidate is maximal
/// (no unique most-specific frame).
fn select_frame<'lexicon>(
    entry_name: &str,
    args: &[Recovered],
    lexicon: &'lexicon Lexicon,
    position: FramePosition,
) -> anyhow::Result<&'lexicon Entry> {
    let candidates: Vec<&Entry> = lexicon
        .entries()
        .iter()
        .filter(|candidate| candidate.name == entry_name)
        .filter(|candidate| {
            candidate
                .frame
                .spec
                .position
                .is_none_or(|required| required == position)
        })
        .collect();
    anyhow::ensure!(
        !candidates.is_empty(),
        "no lexicon entry named `{entry_name}` is registered at position {position:?}"
    );

    let macros = lexicon.macros();
    // Each viable candidate's *satisfied guard-param set* — `guards.len()`
    // params exactly, since a candidate only survives here when every one of
    // its guards holds; kept as the set (not just the count) because D8's
    // specificity order is a superset relation, not a size comparison.
    let viable: Vec<(&Entry, BTreeSet<usize>)> = candidates
        .into_iter()
        .filter_map(|candidate| {
            candidate
                .frame
                .guards
                .iter()
                .map(|guard| {
                    guard_satisfied(guard, args.get(guard.param), macros).then_some(guard.param)
                })
                .collect::<Option<BTreeSet<usize>>>()
                .map(|satisfied| (candidate, satisfied))
        })
        .collect();
    anyhow::ensure!(
        !viable.is_empty(),
        "no frame named `{entry_name}` has every guard satisfied by the recovered argument(s) \
         {args:?}"
    );

    // The maximal element(s) of `viable` under the ⊆ order on guard-param
    // sets. A finite nonempty partial order always has at least one.
    let maximal: Vec<(&Entry, &BTreeSet<usize>)> = viable
        .iter()
        .filter(|(_, set)| {
            !viable
                .iter()
                .any(|(_, other)| other.len() > set.len() && other.is_superset(set))
        })
        .map(|(entry, set)| (*entry, set))
        .collect();

    // More than one maximal candidate splits into two genuinely different
    // situations, and only one of them is the ambiguity D8 cares about. A
    // rival is benign — not a real tie — only when it is *the same authored
    // frame* as the first (same name, `frame_index`, and origin — the
    // identical identity `unify::same_authored_frame` keys on), registered a
    // second time purely because a constructor frame is compiled at every
    // `FragmentKind` it parses cleanly at
    // (`lexicon::compile_constructor_frame`, e.g. `Target`/`DealDamage`).
    // Those registrations share not just a guard-param set but the exact
    // same `spec.text`/holes, so picking whichever the lexicon assembled
    // first changes nothing about what gets rendered. Any OTHER rival — a
    // genuinely different authored frame (different `frame_index`, i.e.
    // different text) whose guard-param set merely happens to be equal or
    // incomparable to the first's — is a real D8 ambiguity: two different
    // wordings both claim to be the most specific match for these
    // arguments, and no order is more "assembled first" than semantic. (A
    // narrower carve-out than an earlier version of this function used: a
    // one-off pair of frames differing only in surface casing — since
    // fixed a different way, see `crate::unify::surface_only_fields` — would
    // have been wrongly swallowed by an equal-guard-set-only check; keying
    // on frame identity catches exactly the multi-`FragmentKind`-registration
    // case it exists for and nothing broader.)
    let (first, _) = maximal[0];
    let rivals: Vec<&Entry> = maximal
        .iter()
        .skip(1)
        .filter(|(entry, _)| !same_authored_frame(entry, first))
        .map(|(entry, _)| *entry)
        .collect();
    anyhow::ensure!(
        rivals.is_empty(),
        "no unique most-specific frame named `{entry_name}` at position {position:?}: {} \
         candidate(s) tie on guard specificity: {}",
        rivals.len() + 1,
        std::iter::once(first)
            .chain(rivals)
            .map(Entry::label)
            .collect::<Vec<_>>()
            .join(", "),
    );
    Ok(first)
}

/// Whether `a` and `b` are the *same* authored frame — identical `name`,
/// `frame_index`, and `origin` — as opposed to two different frames that
/// merely happen to be equally (or incomparably) specific. Mirrors
/// `unify::same_authored_frame` exactly (the match-direction sibling of this
/// same identity question): a constructor frame is registered once per
/// `FragmentKind` it parses cleanly at, so several `Entry`s can share this
/// identity while differing only in `frame.kind`.
fn same_authored_frame(a: &Entry, b: &Entry) -> bool {
    a.name == b.name && a.frame_index == b.frame_index && a.origin == b.origin
}

/// Whether `argument` (the recovered value at the guard's own param index)
/// satisfies `guard` — the single authority
/// ([`guard::normalize_source`]/[`guard::normalized`]'s expanded canonical
/// form), never a textual comparison against `guard.source`: two spellings
/// of the same constant (`"Literal(1)"` and `"1"`, `"Exactly(1)"` and
/// `"Range(Some(1), Some(1))"`) must both satisfy the same guard, exactly as
/// [`crate::guard_holds`] promises one level up. Only a [`Recovered::Literal`]
/// can ever satisfy a guard (an invocation or residual has no RON spelling
/// to read at the guard's declared type); a `Literal` that fails to parse or
/// expand at that type simply does not satisfy it, never an error.
fn guard_satisfied(guard: &CompiledGuard, argument: Option<&Recovered>, macros: &MacroSet) -> bool {
    let Some(Recovered::Literal(text)) = argument else {
        return false;
    };
    guard::normalize_source(macros, &guard.param_type, text).is_ok_and(|view| view == guard.value)
}

/// Substitutes `chosen`'s own frame text for the arguments filling its
/// holes, recursively — this is the one function both the top-level render
/// and a nested invocation filler call, since a nested filler needs nothing
/// more than its own fully-substituted text (see the module doc: only the
/// outermost call ever parses).
fn render_frame_text(
    chosen: &Entry,
    args: &[Recovered],
    lexicon: &Lexicon,
    position: FramePosition,
    name: &str,
) -> anyhow::Result<String> {
    let mut filler_by_param: HashMap<usize, String> = HashMap::new();
    for hole in &chosen.frame.holes {
        // The `~` hole (`param: None`) has no argument at all — `substitute`
        // handles every `~` site unconditionally, using `name` directly.
        let Some(param) = hole.param else { continue };
        let argument = args.get(param).ok_or_else(|| {
            anyhow::anyhow!(
                "frame `{}` holes param {param}, but only {} argument(s) were recovered",
                chosen.name,
                args.len()
            )
        })?;
        let spell = hole.class == HoleClass::Numeral && counts_a_pluralizable_noun(chosen, hole);
        let text = render_argument_text(argument, spell, lexicon, position, name)?;
        filler_by_param.insert(param, text);
    }
    substitute(&chosen.frame.spec.text, &filler_by_param, name)
}

/// Whether `hole` (a [`HoleClass::Numeral`] hole) drives a nominal's noun
/// number — [`AgreeKind::NounNumberFromHole`] in `chosen.frame.agreement`.
///
/// This is the signal this module uses to decide whether a count spells out
/// ("three cards") or stays a digit ("3 damage"), and it is not a guess: the
/// two pilot frames that hole a count differ on this exactly along this
/// line. `Draws`'s count agrees with "cards" (a real, pluralizable noun —
/// `compile::agreement_deps` records the dependency) and its own checked-in
/// legacy template spells it (`${1:card|cards}`,
/// `plugins/builtin/macros/action/Draws.ron`). `DealsDamageToEach`'s and
/// `PumpThisUntilEot`'s counts modify "damage" and a P/T value — a mass noun
/// and no noun at all — neither ever gets a `NounNumberFromHole` dependency
/// (`compile::agreement_deps` explicitly skips a mass head, since it "has no
/// number to take from anything"), and both macros' own legacy templates use
/// a bare `${i}`, digits only. So reading this off `agreement` — rather than
/// hardcoding a per-macro-name table — reuses the exact fact the compiler
/// already computed instead of re-deriving (and risking drifting from) it.
fn counts_a_pluralizable_noun(chosen: &Entry, hole: &Hole) -> bool {
    chosen
        .frame
        .agreement
        .iter()
        .any(|dep| matches!(dep.kind, AgreeKind::NounNumberFromHole(index) if index == hole.index))
}

/// The text one recovered argument contributes at its hole: a nested
/// invocation's own recursively-substituted text; a residual's best-effort
/// reconstruction ([`render_residual_text`]); or a literal — spelled out as
/// an English cardinal word when `spell_count` says so (see
/// [`counts_a_pluralizable_noun`]), digits verbatim otherwise.
///
/// A compiled frame's own witness for a `Count` hole is always the reserved
/// Arabic numeral `41` ([`crate::witness`]), and — the fact this function
/// exists to work around — [`render_fragment`] does not itself decide a
/// count's spelling from its magnitude; it faithfully reproduces whatever
/// numeral notation [`parse_fragment`] read the substituted text as. So
/// substituting the bare digit string and parsing it renders back as digits
/// even where the count should spell out. [`spelled_count`] fixes that
/// *before* substitution, so the parse+render round trip has already-correct
/// text to work from.
fn render_argument_text(
    argument: &Recovered,
    spell_count: bool,
    lexicon: &Lexicon,
    position: FramePosition,
    name: &str,
) -> anyhow::Result<String> {
    match argument {
        Recovered::Literal(text) if spell_count => spelled_count(text),
        Recovered::Literal(text) => Ok(text.clone()),
        Recovered::Invocation { entry, args, .. } => {
            let chosen = select_frame(entry, args, lexicon, position)?;
            render_frame_text(chosen, args, lexicon, position, name)
        }
        Recovered::Residual(view) => render_residual_text(view)
            .map_err(|error| anyhow::anyhow!("rendering a residual filler: {error}")),
    }
}

/// The spelling a `Count` hole's digit text renders as: one through twenty
/// as English cardinal words (matching the legacy template renderer's own
/// `number_word` threshold — `crates/deckmaste_cards/src/render/fragment.rs`
/// — so G3's shadow-parity comparison derives the same spelling on both
/// sides from the same value, independent of how the *card* happened to
/// spell it), anything else verbatim as digits.
fn spelled_count(digits: &str) -> anyhow::Result<String> {
    let value: i32 = digits.trim().parse().map_err(|_| {
        anyhow::anyhow!("a `Count` hole's recovered value {digits:?} is not an integer")
    })?;
    if (1..=20).contains(&value) {
        Ok(Numeral::Cardinal.format(value))
    } else {
        Ok(digits.to_string())
    }
}

/// Substitutes `text`'s own `<Param(i)>` and `~` sigils — the exact grammar
/// [`crate::compile::plan_holes`] parses, walked the same way — for
/// `filler_by_param`'s texts and `self_text` respectively.
///
/// # Errors
/// If a `<Param(` sigil is unterminated, its index is not a plain integer, or
/// names a param `filler_by_param` has no filler for (a hole the frame's own
/// `holes` list did not account for — unreachable through a lexicon this
/// crate compiled, kept as a defensive error rather than a panic).
fn substitute(
    text: &str,
    filler_by_param: &HashMap<usize, String>,
    self_text: &str,
) -> anyhow::Result<String> {
    let mut out = String::with_capacity(text.len());
    let mut rest = text;
    while !rest.is_empty() {
        if let Some(tail) = rest.strip_prefix('~') {
            out.push_str(self_text);
            rest = tail;
            continue;
        }
        if let Some(tail) = rest.strip_prefix("<Param(") {
            let end = tail.find(")>").ok_or_else(|| {
                anyhow::anyhow!("frame text {text:?} has an unterminated `<Param(` sigil")
            })?;
            let param: usize = tail[..end].trim().parse().map_err(|_| {
                anyhow::anyhow!(
                    "frame text {text:?} has a non-numeric hole index `{}`",
                    &tail[..end]
                )
            })?;
            let filler = filler_by_param.get(&param).ok_or_else(|| {
                anyhow::anyhow!(
                    "frame text {text:?} holes param {param}, but no filler was computed for it"
                )
            })?;
            out.push_str(filler);
            rest = &tail[end + 2..];
            continue;
        }
        let step = rest
            .char_indices()
            .nth(1)
            .map_or(rest.len(), |(offset, _)| offset);
        out.push_str(&rest[..step]);
        rest = &rest[step..];
    }
    Ok(out)
}

// ---------------------------------------------------------------------------
// Residual reconstruction — see the module doc's "Residual fillers" section.
// ---------------------------------------------------------------------------

/// Best-effort text for a captured [`Recovered::Residual`] subtree: a bare
/// nominal (no determiner, no modifiers, no complements) whose head's
/// spelling is recoverable straight from the tree.
///
/// # Errors
/// If `view` is not (after unwrapping newtype wrappers) a `NominalPhrase`
/// with a vacuous determiner, modifiers and complements, or its head's
/// spelling is not recoverable — see the module doc.
fn render_residual_text(view: &View) -> anyhow::Result<String> {
    match view {
        View::Newtype { inner, .. } => render_residual_text(inner),
        View::Node {
            name: "NominalPhrase",
            fields,
            ..
        } => render_nominal_phrase(fields),
        other => anyhow::bail!(
            "cannot render this residual shape (no lexicon entry covers it, and it is not a \
             bare nominal this module can reconstruct by hand): {other:?}"
        ),
    }
}

fn find_field<'a>(fields: &'a [(&'static str, View)], wanted: &str) -> Option<&'a View> {
    fields
        .iter()
        .find_map(|(name, value)| (*name == wanted).then_some(value))
}

fn render_nominal_phrase(fields: &[(&'static str, View)]) -> anyhow::Result<String> {
    for field in ["determiner", "modifiers", "complements"] {
        if let Some(value) = find_field(fields, field) {
            anyhow::ensure!(
                value.is_vacuous(),
                "cannot render a residual nominal with a non-empty `{field}` \
                 ({value:?}) — only a bare head is reconstructed by hand"
            );
        }
    }
    let head = find_field(fields, "head")
        .ok_or_else(|| anyhow::anyhow!("a residual NominalPhrase has no `head` field"))?;
    render_noun_instance(head)
}

fn render_noun_instance(view: &View) -> anyhow::Result<String> {
    let View::Newtype {
        name: "NounInstance",
        variant: Some(number),
        inner,
    } = view
    else {
        anyhow::bail!("expected a `NounInstance` head, got {view:?}");
    };
    let spelling = render_noun(inner)?;
    match *number {
        "Singular" | "Mass" => Ok(spelling),
        // English regular plural: every noun this reconstruction reaches
        // (D7's field-slice residuals, table-driven or opaque common nouns)
        // pluralizes this way in the pilot corpus. An irregular plural would
        // render wrong rather than erroring — a real limitation, not
        // silently masked: `crate::unify` never compares this rendering
        // against anything (it is the render *direction*'s own reconstruction,
        // never fed back through `unify`), and G3/G4 would surface a wrong
        // spelling as a diff/divergence if the pilot corpus ever exercised
        // one, which it does not today.
        "Plural" => Ok(format!("{spelling}s")),
        other => anyhow::bail!("unknown `NounInstance` number {other:?}"),
    }
}

fn render_noun(view: &View) -> anyhow::Result<String> {
    let View::Newtype {
        name: "Noun",
        variant: Some(kind),
        inner,
    } = view
    else {
        anyhow::bail!("expected a `Noun`, got {view:?}");
    };
    match *kind {
        "Word" => render_vocab(inner),
        "Opaque" => render_scalar_str(inner, "OpaqueLexeme"),
        "Catalog" => render_catalog_atom(inner),
        other => {
            anyhow::bail!("cannot recover the spelling of a `{other}`-kind noun from a residual")
        }
    }
}

/// [`deckmaste_english::word::Vocab::Regular`] carries its English spelling
/// *in* the tree directly (a `RegularVocab(&'static str)` newtype, generated
/// from `regular-vocabulary.tsv`); every other `Vocab` variant is a
/// hardcoded, fieldless enum case whose spelling lives only in
/// [`Vocab::spelling`](deckmaste_english::word::Vocab::spelling), a
/// compile-time lookup keyed by the variant itself, not by anything the
/// `View` carries. Rather than duplicate that table by hand (which would
/// drift the moment a new vocabulary word is added), this recovers the
/// variant the honest way: `Vocab::ALL` is the *complete*, public
/// enumeration of every hardcoded case, and `Vocab` derives `Debug` as
/// exactly its variant name (a fieldless variant's `Debug` output has no
/// other content to include) — which is precisely
/// [`View::variant_name`]'s own spelling of it. So the variant is found by
/// scanning `Vocab::ALL` for the one whose `Debug` matches, and its real
/// spelling is read off `.spelling()` — the same accessor the renderer
/// itself uses, never re-derived.
fn render_vocab(view: &View) -> anyhow::Result<String> {
    match view {
        View::Newtype {
            name: "Vocab",
            variant: Some("Regular"),
            inner,
        } => render_scalar_str(inner, "RegularVocab"),
        View::Unit {
            name: "Vocab",
            variant: Some(name),
        } => deckmaste_english::word::Vocab::ALL
            .iter()
            .find(|vocab| format!("{vocab:?}") == *name)
            .map(|vocab| vocab.spelling().to_string())
            .ok_or_else(|| {
                anyhow::anyhow!("no `Vocab` variant named `{name}` was found in `Vocab::ALL`")
            }),
        other => anyhow::bail!("expected a `Vocab`, got {other:?}"),
    }
}

/// A `CatalogAtom`'s `spelling` field, straight off the tree (it derives
/// `Serialize` with no custom impl, so its private field still appears —
/// serde derive does not respect Rust field visibility).
fn render_catalog_atom(view: &View) -> anyhow::Result<String> {
    let View::Node {
        name: "CatalogAtom",
        fields,
        ..
    } = view
    else {
        anyhow::bail!("expected a `CatalogAtom`, got {view:?}");
    };
    let spelling = find_field(fields, "spelling")
        .ok_or_else(|| anyhow::anyhow!("a `CatalogAtom` view has no `spelling` field"))?;
    render_scalar_str(spelling, "CatalogAtom.spelling")
}

/// Unwraps one newtype layer (if `view` is one) down to a `str` scalar,
/// naming what was expected in the error.
fn render_scalar_str(view: &View, expected: &str) -> anyhow::Result<String> {
    let scalar = match view {
        View::Newtype { inner, .. } => inner.as_ref(),
        other => other,
    };
    match scalar {
        View::Scalar { kind: "str", repr } => Ok(repr.clone()),
        other => anyhow::bail!("expected a `{expected}` string, got {other:?}"),
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
    use deckmaste_english::FragmentKind;
    use deckmaste_english::parse_fragment;
    use macro_ron::frames::load_constructor_frames;

    use super::*;
    use crate::view;

    fn plugin_dir() -> PathBuf {
        Path::new(env!("CARGO_MANIFEST_DIR")).join("../../plugins/builtin")
    }

    /// The generated, CR-derived catalogs the corpus tooling parses real
    /// oracle text against — identical to `unify.rs`'s own fixture helper of
    /// the same name (kept as a small, deliberate per-file duplication
    /// rather than a shared test-only crate export, matching this crate's
    /// existing fixture-duplication convention).
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
        lexicon: Lexicon,
    }

    fn fixture() -> &'static Fixture {
        static FIXTURE: LazyLock<Fixture> = LazyLock::new(|| {
            let plugin = Plugin::load_with_sibling_prelude(plugin_dir())
                .unwrap_or_else(|error| panic!("loading plugin: {error:#}"));
            let catalogs = real_catalogs();
            let constructors = load_constructor_frames(&plugin_dir().join("frames"))
                .unwrap_or_else(|error| panic!("loading constructor frames: {error:#}"));
            let lexicon = Lexicon::assemble(&plugin.macros, &constructors, &catalogs)
                .unwrap_or_else(|error| panic!("assembling the lexicon: {error:#}"));
            Fixture { catalogs, lexicon }
        });
        &FIXTURE
    }

    fn recover(text: &str, kind: FragmentKind, name: &str) -> Recovered {
        let report = parse_fragment(text, &fixture().catalogs, kind, name, false);
        assert!(
            report.clean(),
            "fixture text {text:?} must parse cleanly at {kind:?}: {:?}",
            report.diagnostics()
        );
        let target = view::of(
            &report
                .into_fragment()
                .expect("a clean report has a fragment"),
        );
        crate::unify(&target, &fixture().lexicon, FramePosition::Main)
    }

    /// The brief's Step 1 witness pair: guard selection (the `You`-guarded
    /// imperative frame beats the unguarded one) and agreement (the
    /// unguarded frame's citation-form "draws" is already correct for
    /// *any* third-person subject, `Target player` included) in one pair.
    #[test]
    fn draw_renders_the_imperative_and_the_declarative() {
        let imperative = recover("Draw three cards.", FragmentKind::Sentence, "");
        assert_eq!(
            render_invocation(&imperative, &fixture().lexicon, FramePosition::Main).unwrap(),
            "Draw three cards.",
            "the You-guarded frame is selected and rendered back exactly"
        );

        let declarative = recover(
            "Target player draws three cards.",
            FragmentKind::Sentence,
            "",
        );
        assert_eq!(
            render_invocation(&declarative, &fixture().lexicon, FramePosition::Main).unwrap(),
            "Target player draws three cards.",
            "the unguarded frame recurses into the nested `Target` invocation, whose own \
             residual `player` filler is reconstructed from the tree"
        );
    }

    /// The public, brief-mandated entry point uses the anonymous identity
    /// and so cannot round-trip a `~`-bearing frame: substituting an empty
    /// string for the sigil leaves the subject position blank, which fails
    /// to parse. This is a real, disclosed limitation (see the module doc),
    /// not a bug — `render_invocation_with`, given the card's real name, has
    /// no such gap and renders exactly what the legacy renderer would.
    #[test]
    fn self_reference_needs_a_real_identity() {
        let recovered = recover(
            "Lightning Bolt deals 3 damage to each creature.",
            FragmentKind::Sentence,
            "Lightning Bolt",
        );
        assert!(
            render_invocation(&recovered, &fixture().lexicon, FramePosition::Main).is_err(),
            "the anonymous identity cannot round-trip a self-reference"
        );
        assert_eq!(
            render_invocation_with(
                &recovered,
                &fixture().lexicon,
                FramePosition::Main,
                &fixture().catalogs,
                "Lightning Bolt",
                false,
            )
            .unwrap(),
            "Lightning Bolt deals 3 damage to each creature.",
        );
    }

    /// A `KeywordLine` frame needs a populated catalog to parse the
    /// substituted text back — `render_invocation`'s own `Catalogs::default()`
    /// cannot recognize any keyword atom at all (G5 finding 3), so
    /// `render_invocation_with`, given the real catalogs, is what a caller
    /// actually needs for this category.
    #[test]
    fn a_keyword_line_frame_needs_a_populated_catalog() {
        let recovered = recover("flying", FragmentKind::KeywordLine, "");
        assert!(render_invocation(&recovered, &fixture().lexicon, FramePosition::Main).is_err());
        assert_eq!(
            render_invocation_with(
                &recovered,
                &fixture().lexicon,
                FramePosition::Main,
                &fixture().catalogs,
                "",
                false,
            )
            .unwrap(),
            "Flying",
        );
    }

    /// The fix-round regression test for G3's Critical finding: real canon
    /// text is *never* the lowercase citation spelling the test above feeds
    /// back in (that only round-trips the frame's own convention) — a solo
    /// keyword line is always capitalized, line-initial, on a real card.
    /// `CatalogAtom.spelling` preserves the matched text's own case, so
    /// without `unify::surface_only_fields`'s `CatalogAtom.spelling` entry
    /// (excluding that field from the match so case can't sink it) this
    /// recovers as a residual (`unify` never reaches an `Invocation` at
    /// all), exactly the gap that made every canon `Keyword(Flying)` line
    /// silently invisible to G3/G4.
    #[test]
    fn a_keyword_line_matches_the_real_capitalized_corpus_spelling() {
        let recovered = recover("Flying", FragmentKind::KeywordLine, "");
        assert!(
            matches!(recovered, Recovered::Invocation { ref entry, .. } if entry == "Flying"),
            "capitalized, line-initial spelling must recover as `Flying`, not a residual: \
             {recovered:#?}"
        );
        assert_eq!(
            render_invocation_with(
                &recovered,
                &fixture().lexicon,
                FramePosition::Main,
                &fixture().catalogs,
                "",
                false,
            )
            .unwrap(),
            "Flying",
        );
    }

    /// A residual this module cannot honestly reconstruct (a determinerful
    /// nominal, here "any target") makes the whole render fail rather than
    /// guess — the render-direction mirror of `unify`'s own totality: a gap
    /// in coverage is a graceful error, never a panic or a fabricated
    /// answer.
    #[test]
    fn an_unreconstructable_residual_fails_the_render_rather_than_guessing() {
        let recovered = recover(
            "Lightning Bolt deals 3 damage to any target.",
            FragmentKind::Sentence,
            "Lightning Bolt",
        );
        let error = render_invocation_with(
            &recovered,
            &fixture().lexicon,
            FramePosition::Main,
            &fixture().catalogs,
            "Lightning Bolt",
            false,
        )
        .unwrap_err();
        assert!(format!("{error:#}").contains("residual"), "{error:#}");
    }

    // -- fix-round regression tests (guard comparison, D8 superset) --------

    /// The fix-round's exact counterexample: a `Recovered` argument spelled
    /// differently from the guard's own authored constant, but denoting the
    /// same value, must still satisfy it. `crates/deckmaste_core/src/
    /// count.rs` documents `Literal(1)` as accepted "for leniency" even
    /// though `Count` never emits it (a bare `1` is canonical) — a textual
    /// comparison against the guard's own `"1"` would miss this spelling,
    /// fall through to `Draws`'s unguarded plural frame, and spell
    /// "draw one cards" instead of "Draw a card." — the exact wrong output a
    /// textual guard comparison produces and a canonical-form one does not.
    #[test]
    fn a_guard_matches_any_ground_spelling_of_its_constant_not_just_its_own() {
        let tagged = Recovered::Invocation {
            entry: "Draws".to_string(),
            args: vec![
                Recovered::Literal("You".to_string()),
                Recovered::Literal("Literal(1)".to_string()),
            ],
            ambiguities: Vec::new(),
        };
        assert_eq!(
            render_invocation(&tagged, &fixture().lexicon, FramePosition::Main).unwrap(),
            "Draw a card.",
            "a differently-spelled but equal argument must still satisfy the guard, not fall \
             through to the wrong frame"
        );
    }

    /// The ruling's fall-through consequence, exercised end to end with a
    /// hand-built `Recovered` (not one `unify` happened to produce): a
    /// `Literal` argument that does *not* satisfy a more specific frame's
    /// guard makes selection fall through to a less specific one — never a
    /// mid-render error.
    #[test]
    fn a_mismatched_guard_falls_through_to_a_less_specific_frame_rather_than_erroring() {
        let recovered = Recovered::Invocation {
            entry: "Draw".to_string(),
            args: vec![Recovered::Literal("2".to_string())],
            ambiguities: Vec::new(),
        };
        assert_eq!(
            render_invocation(&recovered, &fixture().lexicon, FramePosition::Main).unwrap(),
            "Draw two cards.",
            "count = 2 fails the count-1-guarded frame and must fall through to the \
             unguarded plural frame, not error"
        );
    }

    /// D8's real ambiguity, distinguished from the benign kind (the *same*
    /// authored frame registered at several `FragmentKind`s, resolved by
    /// assembly order, see `select_frame`'s own doc and
    /// `same_authored_frame`): two genuinely *different* candidates whose
    /// satisfied guard-param sets are unequal and neither contains the other
    /// (one guards param 0, the other guards param 1) must be reported as a
    /// tie, not resolved silently by whichever the lexicon happened to
    /// assemble first.
    #[test]
    fn incomparable_guards_are_reported_rather_than_resolved_silently() {
        let macros = fixture().lexicon.macros();
        let params: Vec<String> = vec!["Reference".to_string(), "Count".to_string()];
        let frame_a = crate::compile(
            &macro_ron::frames::FrameSpec {
                text: "draw <Param(1)> cards".to_string(),
                when: vec![(0, "You".to_string())],
                position: None,
            },
            FragmentKind::Sentence,
            &params,
            &fixture().catalogs,
            macros,
        )
        .unwrap_or_else(|error| panic!("compiling frame A: {error:#}"));
        let frame_b = crate::compile(
            &macro_ron::frames::FrameSpec {
                text: "<Param(0)> draws a card".to_string(),
                when: vec![(1, "1".to_string())],
                position: None,
            },
            FragmentKind::Sentence,
            &params,
            &fixture().catalogs,
            macros,
        )
        .unwrap_or_else(|error| panic!("compiling frame B: {error:#}"));
        let lexicon = Lexicon::from_entries(
            vec![
                Entry {
                    name: "Ambiguous".to_string(),
                    params: params.clone(),
                    frame_index: 0,
                    origin: crate::lexicon::Origin::Macro,
                    frame: frame_a,
                },
                Entry {
                    name: "Ambiguous".to_string(),
                    params: params.clone(),
                    frame_index: 1,
                    origin: crate::lexicon::Origin::Macro,
                    frame: frame_b,
                },
            ],
            macros.clone(),
        );
        // Satisfies BOTH guards at once (subject = You, count = 1) — with no
        // third, dominating frame (unlike the real `Draws`), neither
        // candidate's guard-param set contains the other's.
        let recovered = Recovered::Invocation {
            entry: "Ambiguous".to_string(),
            args: vec![
                Recovered::Literal("You".to_string()),
                Recovered::Literal("1".to_string()),
            ],
            ambiguities: Vec::new(),
        };
        let error = render_invocation(&recovered, &lexicon, FramePosition::Main).unwrap_err();
        let message = format!("{error:#}");
        assert!(
            message.contains("no unique most-specific") && message.contains("Ambiguous"),
            "{message}"
        );
    }
}
