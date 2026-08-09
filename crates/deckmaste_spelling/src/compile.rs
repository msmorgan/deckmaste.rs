//! The frame compiler: an authored frame string in, a parsed English tree
//! with typed holes in it out.
//!
//! # The trick
//!
//! A frame such as `"<Param(0)> deals <Param(1)> damage to <Param(2)>"` is
//! not English and cannot be parsed. Teaching the parser about hole sigils is
//! not an option — `deckmaste_english` is a leaf crate and must stay one — so
//! the compiler goes the other way round, in four steps:
//!
//! 1. **Witness.** Every sigil is replaced by a reserved token that parses at
//!    the position the hole occupies (`crate::witness`): an opaque noun
//!    `zzhole0` for a phrasal hole, normally a reserved numeral `41` for a
//!    count hole, and the face name `Zzframeself` for `~`. If strict noun
//!    agreement rejects a singular count frame, the compiler retries subsets of
//!    numeric holes with value-one witnesses. Repeated witnesses are
//!    distinguished by their source occurrence. The result is ordinary English.
//! 2. **Parse.** [`parse_fragment`] at the frame's category. The parse must be
//!    [`clean`](deckmaste_english::FragmentReport::clean) — a frame that only
//!    half-parses is a build error naming the frame, never a silently degraded
//!    lexicon entry.
//! 3. **Relocate.** Walk the tree, find where each witness landed, and put a
//!    [`View::Hole`] back in its place. Which node the hole replaces is not the
//!    leaf the witness lexed to but the largest subtree that came *only* from
//!    that witness (see [`HoleClass`]).
//! 4. **Normalize and record.** The same content authored with different
//!    agreement — `"<Param(0)> draw <Param(1)> card"` against `"<Param(0)>
//!    draws <Param(1)> cards"` — parses to structurally different trees. Where
//!    the difference is *driven by a hole*, the compiler rewrites the affected
//!    nodes to citation form and records the dependency in
//!    [`CompiledFrame::agreement`], so a renderer can put the right inflection
//!    back from the filler.
//!
//! # What a consumer gets
//!
//! [`CompiledFrame::tree`] is the frame's meaning: two authorings that differ
//! only in inflection compile to the *same* tree. [`CompiledFrame::holes`] is
//! the side table saying what each hole is and where it sits, and
//! [`CompiledFrame::agreement`] says which nodes take their inflection from
//! which hole. [`CompiledFrame::guards`] carries each guard's constant in the
//! canonical form defined by [`crate::guard`] — the one form both matching and
//! rendering compare against.

use deckmaste_english::Catalogs;
use deckmaste_english::FragmentKind;
use deckmaste_english::Numeral;
use deckmaste_english::features::Number;
use deckmaste_english::features::Person;
use deckmaste_english::parse_fragment;
use macro_ron::MacroSet;
use macro_ron::frames::FrameSpec;

use crate::View;
use crate::guard;
use crate::view;
use crate::view::PathStep;
use crate::view::TreePath;
use crate::witness;
use crate::witness::SELF_WITNESS;
use crate::witness::Witness;
use crate::witness::WitnessKind;

/// What a hole covers, and therefore how a filler is unified into it or
/// substituted back out.
///
/// The distinctions are not taxonomic: each one names a *different*
/// relationship between the hole and the node it was found at, and a consumer
/// that ignores the difference gets a wrong answer rather than a rough one.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum HoleClass {
    /// The hole is a whole subtree: everything at its path came from the
    /// witness. The ordinary case.
    Subtree,
    /// The hole is a **field subset of a flat node**, not a subtree.
    ///
    /// The hole-constituency audit's central finding, and 57 of the corpus's
    /// 128 holes: `syntax::NominalPhrase` is flat
    /// (`determiner`/`modifiers`/`head`/`complements`) because lowering
    /// erases the N-bar the parse forest had, so a frame that supplies part
    /// of a nominal leaves the hole covering the rest. There is no single
    /// node to point at, so the hole's [`Hole::path`] addresses the owning
    /// `NominalPhrase` and `claimed` names the fields the hole takes.
    ///
    /// # `claimed` is variable — read it, never assume it
    ///
    /// Two frame shapes are supported, and they claim **different** field
    /// sets:
    ///
    /// | frame | frame owns | `claimed` |
    /// |---|---|---|
    /// | `target <Param(0)>` | `determiner` | `["modifiers", "head", "complements"]` |
    /// | `<Param(0)> you control` | `complements` | `["modifiers", "head"]` |
    ///
    /// A consumer must bind exactly the fields `claimed` names. Assuming the
    /// three-field set — matching on it, or indexing past it — silently
    /// mis-binds the second shape, taking a postmodifier that belongs to the
    /// frame. `claimed` is always a contiguous run of the node's declaration
    /// order, and always contains `"head"`.
    ///
    /// # How to read one
    ///
    /// `View::Hole` appears **once per claimed field** — so three times for
    /// the determiner-owning shape and twice for the complement-owning one —
    /// each carrying the same `index` and the same `class`. The owning node
    /// therefore keeps its exact arity and field names, and whatever the
    /// frame owns sits *beside* the holes rather than being spliced around
    /// them.
    ///
    /// So a consumer walking the two trees in lockstep needs no special case:
    /// at a `Hole`-valued field it binds the card node's same-named field,
    /// at any other field it compares. The claimed fields are one hole, not
    /// several — **treat them as a unit**: bind all of `claimed` or none, and
    /// do not treat a per-field match as a hole match on its own. `claimed`
    /// is the authority on the grouping; the repetition is a convenience for
    /// the walk, not independent bindings.
    FieldSlice { claimed: Vec<&'static str> },
    /// The hole is a bare number: the `value` of a `NumberLiteral`. The
    /// sibling `numeral` field (Arabic vs. spelled-out) is surface-only, not
    /// the filler's, so relocation normalizes a synthetic retry notation to
    /// the Arabic citation form and leaves it outside the hole.
    Numeral,
    /// The hole is one half of a power/toughness token — the `value` of a
    /// `SignedScalar`, with the sign left in the frame.
    ///
    /// The audit called these "sub-lexical", and at the surface they are:
    /// `bracket` shows `+2/+2` as an atom with no internal structure. In the
    /// lowered AST they are reachable after all, but only *below* the phrase
    /// level, which is why they need their own class: a P/T hole's filler is
    /// a `ScalarValue`, not a phrase, and the `+`/`-` cannot come from it.
    PtHalf,
    /// The hole is the card's self-reference (`~`). Located structurally, by
    /// the `NounPhrase::ThisCard` node the parser builds, since the witness
    /// name never survives into the tree as a spelling.
    SelfRef,
}

/// One hole in a compiled frame.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Hole {
    /// This hole's slot in [`CompiledFrame::holes`], and the `index` its
    /// [`View::Hole`] nodes carry — `holes[h.index] == h`, always.
    ///
    /// **It is not the param index**, and a consumer that assumes it is will
    /// bind the wrong argument. The two coincide only when every declared
    /// param is holed. They part company in the two cases the pilot already
    /// contains:
    ///
    /// - a **guarded** param has no hole, so the holes after it shift down —
    ///   `(text: "draw <Param(1)> cards", when: [(0, "You")])` has one hole,
    ///   `index: 0`, `param: Some(1)`;
    /// - a **`~`** hole has no param at all.
    ///
    /// Recover the argument slot from [`Hole::param`], or go the other way
    /// with [`CompiledFrame::hole_for_param`]. A guarded param is recovered
    /// not from the holes at all but from
    /// [`CompiledFrame::guards`] — that is what a guard is for.
    pub index: usize,
    /// The `<Param(i)>` this hole came from; `None` for a `~` hole, which
    /// stands for the card rather than for an argument.
    pub param: Option<usize>,
    pub class: HoleClass,
    /// Where the hole sits in [`CompiledFrame::tree`]: the node it replaced,
    /// except for [`HoleClass::FieldSlice`], where it is the owning node
    /// whose fields the hole claims.
    ///
    /// **One path, even when the hole has several sites.** Every `~` in a
    /// frame is the same referent and shares one hole, so a frame saying `~`
    /// twice puts two `View::Hole` nodes in the tree and `path` keeps only
    /// the first. A consumer that must visit every occurrence — a renderer
    /// substituting a filler back — scans the tree for
    /// `View::Hole { index, .. }` rather than trusting `path`, which is a
    /// convenience for the single-site case and for diagnostics.
    pub path: TreePath,
    /// What was substituted into the frame text to find this hole. Kept for
    /// diagnostics — nothing downstream should need it.
    pub witness: Witness,
}

/// A node whose inflection is fixed by a hole rather than by the frame.
///
/// The frame compiler cannot leave such a node as parsed, because the same
/// frame authored with the other inflection would parse differently and the
/// two would not compare equal. So it rewrites the node to citation form and
/// records that it did, here.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AgreementDep {
    /// The node that was normalized — a `VerbInstance` for
    /// [`AgreeKind::VerbWithHole`], a `NominalPhrase` for
    /// [`AgreeKind::NounNumberFromHole`].
    pub site: TreePath,
    pub kind: AgreeKind,
    /// What citation normalization actually changed at `site`, in the order
    /// applied. Empty when the authoring was already in citation form.
    pub normalized: Vec<Normalization>,
}

/// Which feature flows from which hole.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AgreeKind {
    /// A verb whose subject position is (or contains) the hole with this
    /// index: its person and number come from the filler.
    VerbWithHole(usize),
    /// A nominal whose number is decided by the count hole with this index:
    /// `1` selects a singular head, anything else a plural one.
    NounNumberFromHole(usize),
}

/// One citation rewrite, recorded so a renderer knows what it has to put back
/// and a reviewer can see what the compiler took away.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Normalization {
    /// A `VerbSlot`'s agreement features were reset to the citation slot
    /// (third person singular). The fields carry what was there before.
    VerbAgreement { person: Person, number: Number },
    /// A plural head noun was reset to singular.
    NounNumber { from: Number },
    /// A hole-bearing `NominalModifier::Quantity` was moved into the empty
    /// `determiner` slot as `DeterminerKind::Quantity`.
    ///
    /// Not cosmetic: English puts a count *modifier* before a singular head
    /// (`41 card`) but a count *determiner* before a plural one (`41 cards`),
    /// so the two authorings of a `Count`-holed nominal differ by which field
    /// the number lands in. Citation form is the determiner.
    QuantityToDeterminer,
}

/// A guard's constant, in the canonical form guard satisfaction is defined on.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CompiledGuard {
    /// The param this guard pre-binds. It has no hole in the frame text.
    pub param: usize,
    /// The declared param type its spelling was read at.
    pub param_type: String,
    /// The spelling exactly as authored — never rewritten, so the catalog
    /// keeps its readable sugar.
    pub source: String,
    /// The expanded canonical [`View`], from [`guard::normalize_source`]. A
    /// card-side argument satisfies this guard iff [`guard::normalized`] of
    /// it equals this.
    pub value: View,
}

/// One authored frame, compiled.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CompiledFrame {
    /// The English category the frame was parsed at.
    pub kind: FragmentKind,
    /// The frame's tree, with [`View::Hole`] where the witnesses were and
    /// hole-driven inflection normalized to citation form.
    pub tree: View,
    /// One entry per hole: param holes first, in ascending param order, then
    /// the `~` hole if there is one.
    pub holes: Vec<Hole>,
    pub agreement: Vec<AgreementDep>,
    /// One entry per `when:` pre-binding, in the order authored.
    pub guards: Vec<CompiledGuard>,
    /// The frame exactly as authored.
    pub spec: FrameSpec,
}

impl CompiledFrame {
    /// The hole standing for `<Param(param)>`, if the frame has one.
    #[must_use]
    pub fn hole_for_param(&self, param: usize) -> Option<&Hole> {
        self.holes.iter().find(|hole| hole.param == Some(param))
    }

    /// Whether `hole` accepts **only** an announcement filler — the authored
    /// [`FrameSpec::announced`](macro_ron::frames::FrameSpec::announced) mark,
    /// resolved from the hole's param.
    ///
    /// Read off the spec rather than stored on the [`Hole`] because it is not
    /// a fact the parse discovered: a hole's [`HoleClass`] is what the English
    /// tree made it, and the announce mark is what the catalog *said* about
    /// what may fill it. Keeping them apart is what lets one wording appear in
    /// two entries with the same hole structure and different filler domains.
    /// A `~` hole (`param: None`) is never announced — it stands for the card,
    /// which no `targets:` list holds.
    #[must_use]
    pub fn announces(&self, hole: &Hole) -> bool {
        hole.param
            .is_some_and(|param| self.spec.announced.contains(&param))
    }
}

/// Compiles one frame.
///
/// `params` names each positional hole's declared type, in index order — the
/// common denominator of `MacroDef.params` and `ConstructorFrames.params`.
///
/// `macros` is **required**, not a convenience: guard satisfaction is defined
/// on fully-expanded canonical form (see [`crate::guard`]), and the catalog's
/// preferred guard spelling is the readable macro sugar — the seeded
/// `Target` entry guards `Exactly(1)`, which is a plugin macro and not a
/// `deckmaste_semantics` constructor at all. A compiler that cannot see the
/// macros therefore cannot evaluate the guards it is handed. Pass
/// [`guard::core_reader`] only for a frame set known to guard with bare
/// semantic constructors.
///
/// # Errors
/// If the frame spells a reserved witness token; if a hole names a param that
/// does not exist, or names one twice, or a declared param has neither a hole
/// nor a guard; if the witnessed text does not parse cleanly at `kind`; if a
/// witness cannot be found exactly once in the parse; or if a guard constant
/// is unreadable or not ground.
pub fn compile(
    spec: &FrameSpec,
    kind: FragmentKind,
    params: &[String],
    catalogs: &Catalogs,
    macros: &MacroSet,
) -> anyhow::Result<CompiledFrame> {
    let context = |error: anyhow::Error| error.context(format!("in frame {:?}", spec.text));

    let plan = plan_holes(spec, params).map_err(context)?;
    let (mut tree, placed) = match parse_witnessed(&plan.text, kind, catalogs) {
        Ok(tree) => {
            let placed = place_all(&tree, plan.holes).map_err(context)?;
            (tree, placed)
        }
        Err(primary_error) => {
            let mut singular_parse = None;
            let mut collision_error = None;
            for assignment in singular_witness_assignments(plan.numeric_holes) {
                let candidate =
                    plan_holes_with_singular(spec, params, &assignment).map_err(context)?;
                let Ok(tree) = parse_witnessed(&candidate.text, kind, catalogs) else {
                    continue;
                };
                match place_all(&tree, candidate.holes) {
                    Ok(placed) => {
                        singular_parse = Some((tree, placed));
                        break;
                    }
                    Err(error) => {
                        collision_error.get_or_insert(error);
                    }
                }
            }
            let Some(result) = singular_parse else {
                return Err(context(collision_error.unwrap_or(primary_error)));
            };
            result
        }
    };

    let mut agreement = agreement_deps(&tree, &placed);
    relocate(&mut tree, &placed);
    normalize_all(&mut tree, &mut agreement, Side::Frame);

    let holes = placed
        .into_iter()
        .map(|placed| placed.finish(&tree))
        .collect::<anyhow::Result<Vec<_>>>()
        .map_err(context)?;

    let mut guards = Vec::new();
    for (param, source) in &spec.when {
        let param_type = params.get(*param).ok_or_else(|| {
            context(anyhow::anyhow!(
                "guard pre-binds param {param}, but only {} are declared",
                params.len()
            ))
        })?;
        guards.push(CompiledGuard {
            param: *param,
            param_type: param_type.clone(),
            source: source.clone(),
            value: guard::normalize_source(macros, param_type, source).map_err(context)?,
        });
    }

    Ok(CompiledFrame {
        kind,
        tree,
        holes,
        agreement,
        guards,
        spec: spec.clone(),
    })
}

// ---------------------------------------------------------------------------
// Step 1 — witness substitution
// ---------------------------------------------------------------------------

/// A hole the frame text asks for, before the parse says where it landed.
#[derive(Debug)]
struct PlannedHole {
    index: usize,
    param: Option<usize>,
    witness: Witness,
    /// How many sigils asked for this hole. One, except for a `~` frame that
    /// says `~` more than once — every occurrence is the same referent, so
    /// they share a hole and relocation expects that many landing sites.
    occurrences: usize,
    /// A numeric witness's bounded surface occurrence in the witnessed text.
    /// The total includes authored identical surfaces; relocation requires
    /// the parse to expose exactly that many markers and selects only this
    /// ordinal, making repeated singular witnesses collision-safe.
    marker_occurrence: Option<MarkerOccurrence>,
}

#[derive(Debug, Clone, Copy)]
struct MarkerOccurrence {
    ordinal: usize,
    total: usize,
}

struct Plan {
    text: String,
    holes: Vec<PlannedHole>,
    numeric_holes: usize,
}

fn is_within_grouped_arabic_numeral(text: &str, offset: usize, len: usize) -> bool {
    let bytes = text.as_bytes();
    let mut start = offset;
    while start > 0 && matches!(bytes[start - 1], b'0'..=b'9' | b',') {
        start -= 1;
    }
    let mut end = offset + len;
    while end < bytes.len() && matches!(bytes[end], b'0'..=b'9' | b',') {
        end += 1;
    }

    let candidate = &text[start..end];
    let before = text[..start].chars().next_back();
    let after = text[end..].chars().next();
    candidate.contains(',')
        && !before.is_some_and(char::is_alphanumeric)
        && !after.is_some_and(char::is_alphanumeric)
        && Numeral::Arabic(true).parse(candidate).is_ok()
}

fn bounded_surface_offsets(text: &str, surface: &str) -> Vec<usize> {
    let is_digit_surface = surface.chars().all(|character| character.is_ascii_digit());
    let continues = |character: char| {
        if is_digit_surface {
            character.is_ascii_digit()
        } else {
            character.is_alphanumeric()
        }
    };
    text.match_indices(surface)
        .filter_map(|(offset, _)| {
            let before = text[..offset].chars().next_back();
            let after = text[offset + surface.len()..].chars().next();
            (!(before.is_some_and(&continues)
                || after.is_some_and(&continues)
                || is_digit_surface
                    && is_within_grouped_arabic_numeral(text, offset, surface.len())))
            .then_some(offset)
        })
        .collect()
}

fn singular_witness_assignments(numeric_holes: usize) -> Vec<Vec<bool>> {
    let mut assignments = Vec::new();
    for singular_count in 1..=numeric_holes {
        for selected in 1_u64..(1_u64 << numeric_holes) {
            if selected.count_ones() as usize != singular_count {
                continue;
            }
            assignments.push(
                (0..numeric_holes)
                    .map(|slot| selected & (1_u64 << slot) != 0)
                    .collect(),
            );
        }
    }
    assignments
}

fn plan_holes(spec: &FrameSpec, params: &[String]) -> anyhow::Result<Plan> {
    plan_holes_with_singular(spec, params, &[])
}

fn plan_holes_with_singular(
    spec: &FrameSpec,
    params: &[String],
    singular_numeric_slots: &[bool],
) -> anyhow::Result<Plan> {
    let reserved = witness::reserved_tokens(&spec.text);
    anyhow::ensure!(
        reserved.is_empty(),
        "spells the reserved witness token(s) {reserved:?}; `zz…` and the numerals {:?} \
         are reserved for the frame compiler's own substitutions",
        witness::RESERVED_NUMERALS,
    );

    let mut text = String::with_capacity(spec.text.len());
    let mut by_param: Vec<Option<Witness>> = vec![None; params.len()];
    let mut witness_offsets = vec![None; params.len()];
    let mut order = Vec::new();
    let mut self_refs = 0usize;
    let mut numeric_slot = 0usize;

    let mut rest = spec.text.as_str();
    while !rest.is_empty() {
        if let Some(tail) = rest.strip_prefix('~') {
            text.push_str(SELF_WITNESS);
            self_refs += 1;
            rest = tail;
            continue;
        }
        if let Some(tail) = rest.strip_prefix("<Param(") {
            let end = tail
                .find(")>")
                .ok_or_else(|| anyhow::anyhow!("has an unterminated `<Param(` sigil"))?;
            let param: usize = tail[..end]
                .trim()
                .parse()
                .map_err(|_| anyhow::anyhow!("has a non-numeric hole index `{}`", &tail[..end]))?;
            let param_type = params.get(param).ok_or_else(|| {
                anyhow::anyhow!(
                    "holes `<Param({param})>`, but only {} param(s) are declared",
                    params.len()
                )
            })?;
            anyhow::ensure!(
                by_param[param].is_none(),
                "holes `<Param({param})>` more than once; a frame must be linear \
                 (each param on exactly one constituent)"
            );
            let witness = if witness::is_numeric_param(param_type) {
                let witness = if singular_numeric_slots
                    .get(numeric_slot)
                    .copied()
                    .unwrap_or(false)
                {
                    witness::singular_numeral()
                } else {
                    witness::numeral(numeric_slot)?
                };
                numeric_slot += 1;
                witness
            } else {
                witness::lexeme(param_type, param)
            };
            witness_offsets[param] = Some(text.len());
            text.push_str(&witness.text);
            by_param[param] = Some(witness);
            order.push(param);
            rest = &tail[end + 2..];
            continue;
        }
        let step = rest
            .char_indices()
            .nth(1)
            .map_or(rest.len(), |(offset, _)| offset);
        text.push_str(&rest[..step]);
        rest = &rest[step..];
    }

    for (param, param_type) in params.iter().enumerate() {
        anyhow::ensure!(
            by_param[param].is_some() || spec.when.iter().any(|(bound, _)| *bound == param),
            "declares param {param} (`{param_type}`) but neither holes it nor guards it; \
             a param with no surface must be pre-bound by a `when:` guard"
        );
    }
    for (param, _) in &spec.when {
        anyhow::ensure!(
            params.get(*param).is_none() || by_param.get(*param).is_none_or(Option::is_none),
            "both holes and guards param {param}; a guarded param is pre-bound and \
             therefore has no surface of its own"
        );
    }
    for param in &spec.announced {
        anyhow::ensure!(
            *param < params.len(),
            "marks param {param} announced, but only {} param(s) are declared",
            params.len()
        );
        // An announced param is a restriction on what may *fill* its hole, so
        // it needs a hole to restrict. A guarded param has none by
        // construction, and its value is the guard's constant rather than
        // anything the card supplies.
        anyhow::ensure!(
            by_param[*param].is_some(),
            "marks param {param} announced, but that param has no hole; the \
             mark restricts what may fill a hole, and a guarded param is \
             pre-bound instead of filled"
        );
    }

    let mut holes: Vec<PlannedHole> = order
        .into_iter()
        .filter_map(|param| {
            by_param[param].clone().map(|witness| {
                let marker_occurrence =
                    matches!(witness.kind, WitnessKind::Numeral { .. }).then(|| {
                        let offsets = bounded_surface_offsets(&text, &witness.text);
                        let inserted = witness_offsets[param]
                            .expect("a planned param witness has an insertion offset");
                        let ordinal = offsets
                            .iter()
                            .position(|offset| *offset == inserted)
                            .expect("the inserted witness is a bounded surface occurrence");
                        MarkerOccurrence {
                            ordinal,
                            total: offsets.len(),
                        }
                    });
                PlannedHole {
                    index: 0,
                    param: Some(param),
                    witness,
                    occurrences: 1,
                    marker_occurrence,
                }
            })
        })
        .collect();
    holes.sort_by_key(|hole| hole.param);
    if self_refs > 0 {
        holes.push(PlannedHole {
            index: 0,
            param: None,
            witness: Witness::self_reference(),
            occurrences: self_refs,
            marker_occurrence: None,
        });
    }
    for (index, hole) in holes.iter_mut().enumerate() {
        hole.index = index;
    }
    Ok(Plan {
        text,
        holes,
        numeric_holes: numeric_slot,
    })
}

// ---------------------------------------------------------------------------
// Step 2 — parse
// ---------------------------------------------------------------------------

fn parse_witnessed(text: &str, kind: FragmentKind, catalogs: &Catalogs) -> anyhow::Result<View> {
    let report = parse_fragment(text, catalogs, kind, SELF_WITNESS, false);
    if !report.clean() {
        anyhow::bail!(
            "witnessed as {text:?} does not parse cleanly at {kind:?}: \
             diagnostics {:?}, recovered {:?}",
            report.diagnostics(),
            report
                .recoveries()
                .iter()
                .map(|recovery| recovery.text)
                .collect::<Vec<_>>(),
        );
    }
    let fragment = report
        .into_fragment()
        .expect("a clean fragment report has a fragment");
    Ok(view::of(&fragment))
}

// ---------------------------------------------------------------------------
// Step 3 — relocation
// ---------------------------------------------------------------------------

/// A hole once the parse has said where it is.
#[derive(Debug)]
struct PlacedHole {
    planned: PlannedHole,
    class: HoleClass,
    /// Where each occurrence sits. One entry unless the frame said `~` twice.
    sites: Vec<TreePath>,
}

impl PlacedHole {
    fn hole_node(&self) -> View {
        View::Hole {
            index: self.planned.index,
            class: self.class.clone(),
        }
    }

    /// The final record, with the path re-derived from the finished tree —
    /// citation normalization moves nodes around, so a path taken before it
    /// ran cannot be trusted.
    fn finish(self, tree: &View) -> anyhow::Result<Hole> {
        let index = self.planned.index;
        let found: Vec<TreePath> = tree
            .walk()
            .into_iter()
            .filter(|(_, node)| matches!(node, View::Hole { index: at, .. } if *at == index))
            .map(|(path, _)| path)
            .collect();
        let first = found.first().cloned().ok_or_else(|| {
            anyhow::anyhow!("hole {index} vanished from the tree during normalization")
        })?;
        // A field-slice hole is spelled once per claimed field; its path is
        // the owning node, one step above any of them.
        let path = if matches!(self.class, HoleClass::FieldSlice { .. }) {
            TreePath(first.0[..first.0.len().saturating_sub(1)].to_vec())
        } else {
            first
        };
        Ok(Hole {
            index,
            param: self.planned.param,
            class: self.class,
            path,
            witness: self.planned.witness,
        })
    }
}

/// Locates one planned hole in the parsed tree and decides its class.
#[cfg(test)]
fn place(tree: &View, planned: PlannedHole) -> anyhow::Result<PlacedHole> {
    let markers = select_markers(tree, &planned)?;
    place_at_markers(tree, planned, markers)
}

fn select_markers(tree: &View, planned: &PlannedHole) -> anyhow::Result<Vec<TreePath>> {
    let witness = &planned.witness;
    let markers = tree
        .walk()
        .into_iter()
        .filter(|(path, node)| is_marker_at(tree, path, node, witness))
        .map(|(path, _)| path)
        .collect::<Vec<_>>();
    let Some(occurrence) = planned.marker_occurrence else {
        anyhow::ensure!(
            markers.len() == planned.occurrences,
            "witness `{witness}` for hole {} was found {} time(s) in the parse, expected {}",
            planned.index,
            markers.len(),
            planned.occurrences,
        );
        return Ok(markers);
    };
    anyhow::ensure!(
        markers.len() == occurrence.total,
        "witness `{witness}` for hole {} was found {} time(s) in the parse, but the witnessed \
         source has {} bounded occurrence(s)",
        planned.index,
        markers.len(),
        occurrence.total,
    );
    let marker = markers.get(occurrence.ordinal).cloned().ok_or_else(|| {
        anyhow::anyhow!(
            "witness `{witness}` for hole {} has source occurrence {}, but the parse has only {} \
             marker(s)",
            planned.index,
            occurrence.ordinal,
            markers.len(),
        )
    })?;
    Ok(vec![marker])
}

fn place_at_markers(
    tree: &View,
    planned: PlannedHole,
    markers: Vec<TreePath>,
) -> anyhow::Result<PlacedHole> {
    let witness = &planned.witness;

    // Every site of one hole necessarily gets the same class: the only hole
    // that can have more than one site is `~` (a param may be holed exactly
    // once — `plan_holes` enforces linearity), and `classify` answers
    // `SelfRef` for every self-reference site unconditionally. So the class
    // is decided once, from the first site.
    let mut sites = Vec::new();
    let mut class = None;
    for marker in markers {
        let site = hoist(tree, &marker, witness);
        let (site_class, site) = classify(tree, site, witness)?;
        class.get_or_insert(site_class);
        sites.push(site);
    }
    Ok(PlacedHole {
        class: class.expect("at least one occurrence"),
        planned,
        sites,
    })
}

fn place_all(tree: &View, planned: Vec<PlannedHole>) -> anyhow::Result<Vec<PlacedHole>> {
    let mut claimed_markers = Vec::new();
    let mut placed = Vec::with_capacity(planned.len());
    for hole in planned {
        let markers = select_markers(tree, &hole)?;
        for marker in &markers {
            anyhow::ensure!(
                !claimed_markers.contains(marker),
                "witness `{}` for hole {} selected parse marker {marker}, which another hole \
                 already claimed",
                hole.witness,
                hole.index,
            );
        }
        claimed_markers.extend(markers.iter().cloned());
        placed.push(place_at_markers(tree, hole, markers)?);
    }
    Ok(placed)
}

fn number_literal_notation_matches(tree: &View, path: &TreePath, expected: Numeral) -> bool {
    if path.0.last() != Some(&PathStep::Field("value")) {
        return false;
    }
    let parent = TreePath(path.0[..path.0.len().saturating_sub(1)].to_vec());
    let Some(View::Node {
        name: "NumberLiteral",
        fields,
        ..
    }) = parent.resolve(tree)
    else {
        return false;
    };
    let Some((_, notation)) = fields.iter().find(|(field, _)| *field == "numeral") else {
        return false;
    };
    notation == &view::of(&expected)
}

fn is_number_literal_value(tree: &View, path: &TreePath) -> bool {
    if path.0.last() != Some(&PathStep::Field("value")) {
        return false;
    }
    let parent = TreePath(path.0[..path.0.len().saturating_sub(1)].to_vec());
    parent.resolve(tree).and_then(View::type_name) == Some("NumberLiteral")
}

fn numeric_scalar_value(node: &View) -> Option<i32> {
    let View::Scalar { kind, repr } = node else {
        return None;
    };
    matches!(
        *kind,
        "u8" | "u16" | "u32" | "u64" | "i8" | "i16" | "i32" | "i64"
    )
    .then(|| repr.parse().ok())
    .flatten()
}

/// Whether this node is where a witness bottomed out.
fn is_marker_at(tree: &View, path: &TreePath, node: &View, witness: &Witness) -> bool {
    match witness.kind {
        WitnessKind::Lexeme => {
            matches!(node, View::Scalar { kind: "str", repr } if *repr == witness.text)
        }
        WitnessKind::Numeral { value, notation } => {
            if numeric_scalar_value(node) != Some(value) {
                return false;
            }
            if is_number_literal_value(tree, path) {
                number_literal_notation_matches(tree, path, notation)
            } else {
                matches!(node, View::Scalar { repr, .. } if *repr == witness.text)
            }
        }
        // The face name is consumed by the parser's identity machinery and
        // never reaches the tree as a spelling, so the self-reference is
        // recognized by the node the parser builds for it instead.
        WitnessKind::SelfReference => {
            node.type_name() == Some("NounPhrase") && node.variant_name() == Some("ThisCard")
        }
    }
}

/// The shallowest ancestor of `marker` that still came only from `witness`.
///
/// Walking up stops at the first ancestor that holds frame material, and
/// never reaches the root, so `tree` itself never becomes a bare hole and
/// keeps its `Fragment::<kind>` wrapper.
fn hoist(tree: &View, marker: &TreePath, witness: &Witness) -> TreePath {
    let mut best = marker.clone();
    for depth in (1..marker.0.len()).rev() {
        let ancestor = TreePath(marker.0[..depth].to_vec());
        let Some(node) = ancestor.resolve(tree) else {
            break;
        };
        if witness_only(node, witness) {
            best = ancestor;
        } else {
            break;
        }
    }
    best
}

/// Whether every non-empty part of `node` came from `witness`.
///
/// "Non-empty" is [`View::is_vacuous`]: a `None` determiner and an empty
/// modifier list are not frame material, so a hole grows through them, while
/// a `DeterminerKind::Target` — which is what `target <Param(0)>` contributes —
/// stops it. That single distinction is what separates a
/// [`HoleClass::Subtree`] hole from a [`HoleClass::FieldSlice`] one without
/// any per-category table.
fn witness_only(node: &View, witness: &Witness) -> bool {
    let is_marker_value = match witness.kind {
        WitnessKind::Numeral { value, .. } => numeric_scalar_value(node) == Some(value),
        WitnessKind::Lexeme => {
            matches!(node, View::Scalar { kind: "str", repr } if *repr == witness.text)
        }
        WitnessKind::SelfReference => {
            node.type_name() == Some("NounPhrase") && node.variant_name() == Some("ThisCard")
        }
    };
    if is_marker_value {
        return true;
    }
    let children = node.children();
    if children.is_empty() {
        return false;
    }
    let mut found = false;
    for (_, child) in children {
        if witness_only(child, witness) {
            if found {
                return false;
            }
            found = true;
        } else if !child.is_vacuous() {
            return false;
        }
    }
    found
}

/// Decides what kind of hole the node at `site` is, adjusting `site` when the
/// hole turns out to be a field slice of the node above.
fn classify(
    tree: &View,
    site: TreePath,
    witness: &Witness,
) -> anyhow::Result<(HoleClass, TreePath)> {
    if witness.kind == WitnessKind::SelfReference {
        return Ok((HoleClass::SelfRef, site));
    }
    let last = site.0.last().copied();
    let parent = TreePath(site.0[..site.0.len().saturating_sub(1)].to_vec());
    let parent_name = parent.resolve(tree).and_then(View::type_name);

    if parent_name == Some("SignedScalar") && last == Some(PathStep::Field("value")) {
        return Ok((HoleClass::PtHalf, site));
    }
    if matches!(witness.kind, WitnessKind::Numeral { .. }) {
        return Ok((HoleClass::Numeral, site));
    }
    if parent_name == Some("NominalPhrase") && last == Some(PathStep::Field("head")) {
        return Ok((field_slice(tree, &parent, &site)?, parent));
    }
    Ok((HoleClass::Subtree, site))
}

/// Decides which fields of a flat `NominalPhrase` a hole at its `head`
/// claims, from which of the sibling fields the frame filled in.
///
/// Reaching here at all means the nominal is not witness-only, so at least
/// one sibling holds frame material — otherwise the hoist would have taken
/// the whole node and the hole would be a [`HoleClass::Subtree`].
///
/// Two asymmetries are supported, and they are the two the pilot lexicon
/// attests. Both leave the hole a **contiguous** run of the flat node:
///
/// | frame owns | example frame | hole claims |
/// |---|---|---|
/// | `determiner` | `target <Param(0)>` | `modifiers`, `head`, `complements` |
/// | `complements` | `<Param(0)> you control` | `modifiers`, `head` |
///
/// The first is spec D7 and 57 of the corpus's 128 holes. The second is its
/// mirror: `creature you control` really does lower to a `NominalPhrase`
/// whose `complements` carry a zero-marked object-gap `RelativeClause`
/// (`grammar/lowering.rs`), so the postmodifier is the frame's and the hole
/// is the premodifiers plus the head.
///
/// Note the deliberate asymmetry between the two rows: the determiner case
/// claims the empty `complements` (so a filler may bring its own
/// postmodifier) while the complement case does **not** claim the empty
/// `determiner`. A filter predicate is determinerless by construction — the
/// determiner is the *enclosing* frame's to supply — so handing it to the
/// filler would let an argument smuggle in a determiner the frame never
/// licensed.
///
/// Anything else refuses. A frame owning both ends (`target <Param(0)> you
/// control`) would leave the hole a discontinuous slice, which the
/// all-or-none binding contract cannot express; a frame owning only
/// `modifiers` (`white <Param(0)>`) is a coherent third shape that was
/// considered and deliberately left out of scope.
fn field_slice(tree: &View, parent: &TreePath, site: &TreePath) -> anyhow::Result<HoleClass> {
    let occupied = |field| {
        !parent
            .then(PathStep::Field(field))
            .resolve(tree)
            .is_some_and(View::is_vacuous)
    };
    let claimed: &[&'static str] = match (
        occupied("determiner"),
        occupied("modifiers"),
        occupied("complements"),
    ) {
        (true, false, false) => &["modifiers", "head", "complements"],
        (false, false, true) => &["modifiers", "head"],
        (determiner, modifiers, complements) => {
            let held: Vec<&str> = [
                ("determiner", determiner),
                ("modifiers", modifiers),
                ("complements", complements),
            ]
            .into_iter()
            .filter_map(|(name, occupied)| occupied.then_some(name))
            .collect();
            anyhow::bail!(
                "puts frame material in a nominal's {held:?} beside the hole at {site}; \
                 a field-slice hole must claim a contiguous run, so the frame may own the \
                 determiner alone (the hole claiming [\"modifiers\", \"head\", \"complements\"]) \
                 or the complements alone (the hole claiming [\"modifiers\", \"head\"]), \
                 but not this combination"
            );
        }
    };
    Ok(HoleClass::FieldSlice {
        claimed: claimed.to_vec(),
    })
}

/// Puts every hole into the tree, replacing what the witnesses left.
fn relocate(tree: &mut View, placed: &[PlacedHole]) {
    for hole in placed {
        let node = hole.hole_node();
        for site in &hole.sites {
            match &hole.class {
                HoleClass::FieldSlice { claimed } => {
                    for field in claimed {
                        if let Some(slot) = site.then(PathStep::Field(field)).resolve_mut(tree) {
                            *slot = node.clone();
                        }
                    }
                }
                HoleClass::Numeral => {
                    if site.0.last() == Some(&PathStep::Field("value")) {
                        let parent = TreePath(site.0[..site.0.len().saturating_sub(1)].to_vec());
                        if parent.resolve(tree).and_then(View::type_name) == Some("NumberLiteral")
                            && let Some(notation) =
                                parent.then(PathStep::Field("numeral")).resolve_mut(tree)
                        {
                            *notation = view::of(&Numeral::Arabic(false));
                        }
                    }
                    if let Some(slot) = site.resolve_mut(tree) {
                        *slot = node.clone();
                    }
                }
                _ => {
                    if let Some(slot) = site.resolve_mut(tree) {
                        *slot = node.clone();
                    }
                }
            }
        }
    }
}

// ---------------------------------------------------------------------------
// Step 4 — the agreement side table and citation normalization
// ---------------------------------------------------------------------------

/// Finds the nodes whose inflection a hole decides. Runs on the *witnessed*
/// tree, before relocation, because the role markers it keys off — a
/// `Subject` beside a `HeadedPredicate` — are exactly the wrappers a hole may
/// be about to swallow.
fn agreement_deps(tree: &View, placed: &[PlacedHole]) -> Vec<AgreementDep> {
    let mut deps = Vec::new();
    for (path, node) in tree.walk() {
        match node {
            View::Seq(items) => {
                let role =
                    |name: &str| items.iter().position(|item| item.type_name() == Some(name));
                let (Some(subject), Some(predicate)) = (role("Subject"), role("HeadedPredicate"))
                else {
                    continue;
                };
                let subject = path.then(PathStep::Index(subject));
                let site = path
                    .then(PathStep::Index(predicate))
                    .then(PathStep::Field("head"))
                    .then(PathStep::Field("verb"));
                if site.resolve(tree).and_then(View::type_name) != Some("VerbInstance") {
                    continue;
                }
                for hole in placed {
                    if hole.sites.iter().any(|at| subject.is_prefix_of(at)) {
                        deps.push(AgreementDep {
                            site: site.clone(),
                            kind: AgreeKind::VerbWithHole(hole.planned.index),
                            normalized: Vec::new(),
                        });
                    }
                }
            }
            View::Node { name, .. } if *name == "NominalPhrase" => {
                // A mass head has no number to take from anything — `41 life`
                // puts the count in the determiner exactly as `41 cards`
                // does, but there is no `lifes`. Recording a dependency here
                // would tell a renderer to inflect a noun that does not
                // inflect.
                let inflects = matches!(
                    path.then(PathStep::Field("head")).resolve(tree),
                    Some(head) if matches!(head.variant_name(), Some("Singular" | "Plural"))
                );
                if !inflects {
                    continue;
                }
                for hole in placed {
                    if hole.class != HoleClass::Numeral {
                        continue;
                    }
                    let counts = ["determiner", "modifiers"].iter().any(|field| {
                        let field = path.then(PathStep::Field(field));
                        hole.sites.iter().any(|at| field.is_prefix_of(at))
                    });
                    if counts {
                        deps.push(AgreementDep {
                            site: path.clone(),
                            kind: AgreeKind::NounNumberFromHole(hole.planned.index),
                            normalized: Vec::new(),
                        });
                    }
                }
            }
            _ => {}
        }
    }
    deps
}

/// A sequence element that citation normalization took out, so every
/// recorded path running through a later sibling can be renumbered.
struct Removal {
    sequence: TreePath,
    index: usize,
}

/// Which tree citation normalization is being applied to.
///
/// The rewrites are the same on both sides — that is the whole point of
/// sharing this code with the unifier rather than letting it grow its own
/// copy — but one rule needs to know where it is. Lifting a count out of a
/// nominal's `modifiers` has to identify *which* modifier is the count, and
/// on a frame that is "the one bearing this hole". A card-side tree has no
/// holes, so there it is "the one and only quantity modifier".
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Side {
    /// A compiled frame's tree, with [`View::Hole`] nodes in it.
    Frame,
    /// A card-side tree being neutralized before comparison.
    Card,
}

/// Runs citation normalization over every agreement site, keeping the whole
/// side table's paths valid as it goes.
///
/// One of the rewrites removes an element from a nominal's `modifiers`, which
/// renumbers its later siblings. [`PlacedHole::finish`] re-derives hole paths
/// from the finished tree afterwards and so is immune, but an
/// [`AgreementDep::site`] has no marker in the tree to re-find it by — so
/// each removal is reported back here and every site is repaired against it
/// immediately. Without that, a site addressing a later sibling would quietly
/// start pointing at its neighbour: `normalize_citation` would find the wrong
/// node shape, bail, record nothing, and two authorings of the same frame
/// would compile to different trees with no error raised.
pub(crate) fn normalize_all(tree: &mut View, agreement: &mut [AgreementDep], side: Side) {
    for index in 0..agreement.len() {
        let at = agreement[index].site.clone();
        let kind = agreement[index].kind;
        let (applied, removal) = normalize_citation(tree, &at, kind, side);
        agreement[index].normalized = applied;
        if let Some(removal) = removal {
            for dep in agreement.iter_mut() {
                dep.site
                    .shift_after_removal(&removal.sequence, removal.index);
            }
        }
    }
}

/// Rewrites one agreement site to citation form, returning what it changed
/// and any sequence element it removed.
fn normalize_citation(
    tree: &mut View,
    at: &TreePath,
    kind: AgreeKind,
    side: Side,
) -> (Vec<Normalization>, Option<Removal>) {
    let mut applied = Vec::new();
    let mut removal = None;
    let Some(node) = at.resolve_mut(tree) else {
        return (applied, None);
    };
    match kind {
        AgreeKind::VerbWithHole(_) => {
            let View::Node { fields, .. } = node else {
                return (applied, None);
            };
            let Some((_, View::Node { fields: slot, .. })) =
                fields.iter_mut().find(|(name, _)| *name == "slot")
            else {
                return (applied, None);
            };
            let mut was = (Person::Third, Number::Singular);
            let mut changed = false;
            if let Some((_, View::Unit { variant, .. })) =
                slot.iter_mut().find(|(field, _)| *field == "person")
                && let Some(current) = variant.and_then(person_from_variant)
            {
                was.0 = current;
                changed |= current != Person::Third;
                *variant = Some(person_variant(Person::Third));
            }
            if let Some((_, View::Unit { variant, .. })) =
                slot.iter_mut().find(|(field, _)| *field == "number")
                && let Some(current) = variant.and_then(number_from_variant)
            {
                was.1 = current;
                changed |= current != Number::Singular;
                *variant = Some(number_variant(Number::Singular));
            }
            if changed {
                applied.push(Normalization::VerbAgreement {
                    person: was.0,
                    number: was.1,
                });
            }
        }
        AgreeKind::NounNumberFromHole(hole) => {
            let View::Node { fields, .. } = node else {
                return (applied, None);
            };
            if let Some((moved, at_index)) = take_quantity_modifier(fields, hole, side) {
                set_field(fields, "determiner", moved);
                applied.push(Normalization::QuantityToDeterminer);
                removal = Some(Removal {
                    sequence: at.then(PathStep::Field("modifiers")),
                    index: at_index,
                });
            }
            if let Some((_, head)) = fields.iter_mut().find(|(name, _)| *name == "head")
                && let View::Newtype {
                    name: "NounInstance",
                    variant: Some(variant),
                    ..
                } = head
                && number_from_variant(variant) == Some(Number::Plural)
            {
                *variant = number_variant(Number::Singular);
                applied.push(Normalization::NounNumber {
                    from: Number::Plural,
                });
            }
        }
    }
    (applied, removal)
}

fn person_from_variant(variant: &str) -> Option<Person> {
    match variant {
        "Second" => Some(Person::Second),
        "Third" => Some(Person::Third),
        _ => None,
    }
}

const fn person_variant(person: Person) -> &'static str {
    match person {
        Person::Second => "Second",
        Person::Third => "Third",
    }
}

fn number_from_variant(variant: &str) -> Option<Number> {
    match variant {
        "Singular" => Some(Number::Singular),
        "Plural" => Some(Number::Plural),
        _ => None,
    }
}

const fn number_variant(number: Number) -> &'static str {
    match number {
        Number::Singular => "Singular",
        Number::Plural => "Plural",
    }
}

/// Takes the sole hole-bearing `NominalModifier::Quantity` out of a nominal's
/// `modifiers`, if its `determiner` is empty, and returns it re-wrapped as a
/// `DeterminerKind::Quantity`, with the index it came from.
///
/// On [`Side::Card`] there is no hole to key off — a card's tree is holeless
/// by construction — so the modifier is identified by being a quantity at
/// all. The two readings coincide on every frame the compiler produces: a
/// nominal whose number is hole-driven has exactly one count in it, which is
/// precisely why the agreement dependency was recorded for it.
fn take_quantity_modifier(
    fields: &mut [(&'static str, View)],
    hole: usize,
    side: Side,
) -> Option<(View, usize)> {
    let determiner_is_empty = fields
        .iter()
        .any(|(name, value)| *name == "determiner" && value.is_vacuous());
    if !determiner_is_empty {
        return None;
    }
    let (_, modifiers) = fields.iter_mut().find(|(name, _)| *name == "modifiers")?;
    let View::Seq(items) = modifiers else {
        return None;
    };
    let at = items.iter().position(|item| {
        item.type_name() == Some("NominalModifier")
            && item.variant_name() == Some("Quantity")
            && (side == Side::Card
                || item
                    .walk()
                    .iter()
                    .any(|(_, node)| matches!(node, View::Hole { index, .. } if *index == hole)))
    })?;
    let View::Newtype { inner, .. } = items.remove(at) else {
        return None;
    };
    Some((
        View::Newtype {
            name: "Determiner",
            variant: Some("Quantity"),
            inner,
        },
        at,
    ))
}

fn set_field(fields: &mut [(&'static str, View)], name: &str, value: View) {
    if let Some((_, slot)) = fields.iter_mut().find(|(field, _)| *field == name) {
        *slot = value;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The `DealDamage` constructor's declared param types, as
    /// `plugins/builtin/frames/constructors.ron` spells them.
    fn deal_damage_params() -> Vec<String> {
        ["Reference", "Count", "Reference"]
            .map(String::from)
            .to_vec()
    }

    fn draw_params() -> Vec<String> {
        ["Reference", "Count"].map(String::from).to_vec()
    }

    fn bare(text: &str) -> FrameSpec {
        FrameSpec::bare(text)
    }

    fn holes_in(tree: &View) -> Vec<usize> {
        tree.walk()
            .into_iter()
            .filter_map(|(_, node)| match node {
                View::Hole { index, .. } => Some(*index),
                _ => None,
            })
            .collect()
    }

    #[test]
    fn effect_frame_compiles_with_two_holes() {
        let frame = compile(
            &bare("<Param(0)> deals <Param(1)> damage to <Param(2)>"),
            FragmentKind::Sentence,
            &deal_damage_params(),
            &Catalogs::default(),
            &reader(),
        )
        .unwrap();
        assert_eq!(frame.holes.len(), 3);
        assert!(
            frame
                .holes
                .iter()
                .any(|hole| matches!(hole.class, HoleClass::Numeral))
        );
        assert_sites_resolve(&frame);
        // No witness survives: the compiled tree is witness-free by
        // construction, which is the property the whole design rests on.
        assert!(
            !format!("{:?}", frame.tree).contains("zz"),
            "a witness leaked into the compiled tree"
        );
        // Every hole is addressable, and its path really does land on it.
        for hole in &frame.holes {
            let at = hole.path.resolve(&frame.tree).expect("hole path resolves");
            assert!(
                matches!(at, View::Hole { index, .. } if *index == hole.index),
                "hole {} at {} resolved to {at:?}",
                hole.index,
                hole.path
            );
        }
    }

    #[test]
    fn filter_hole_is_a_field_slice() {
        let frame = compile(
            &bare("target <Param(0)>"),
            FragmentKind::Nominal,
            &["Predicate".to_string()],
            &Catalogs::default(),
            &reader(),
        )
        .unwrap();
        assert!(
            matches!(frame.holes[0].class, HoleClass::FieldSlice { .. }),
            "target <P> claims the determiner; the hole is the nominal core (spec D7): {:?}",
            frame.holes[0].class
        );
        let HoleClass::FieldSlice { claimed } = &frame.holes[0].class else {
            unreachable!()
        };
        assert_eq!(claimed, &["modifiers", "head", "complements"]);
        // The owning node keeps its shape: the frame's determiner sits beside
        // three hole-valued fields.
        let owner = frame.holes[0].path.resolve(&frame.tree).unwrap();
        assert_eq!(owner.type_name(), Some("NominalPhrase"));
        let View::Node { fields, .. } = owner else { panic!("{owner:?}") };
        assert_eq!(fields.len(), 4);
        assert_eq!(fields[0].0, "determiner");
        assert_eq!(fields[0].1.variant_name(), Some("Target"));
        for (name, value) in &fields[1..] {
            assert!(
                matches!(value, View::Hole { index: 0, .. }),
                "{name} should be part of the slice, got {value:?}"
            );
        }
    }

    /// A bare `<Param(0)>` at the same category has no frame determiner, so
    /// the hole grows past the nominal and is an ordinary subtree — the
    /// control that shows the field-slice promotion is triggered by the
    /// frame's material and not by the category.
    #[test]
    fn a_determinerless_nominal_hole_is_a_whole_subtree() {
        let frame = compile(
            &bare("<Param(0)>"),
            FragmentKind::Nominal,
            &["Predicate".to_string()],
            &Catalogs::default(),
            &reader(),
        )
        .unwrap();
        assert_eq!(frame.holes[0].class, HoleClass::Subtree);
    }

    #[test]
    fn differently_inflected_authorings_compile_identically() {
        let a = compile(
            &bare("<Param(0)> draw <Param(1)> card"),
            FragmentKind::Sentence,
            &draw_params(),
            &Catalogs::default(),
            &reader(),
        )
        .unwrap();
        let b = compile(
            &bare("<Param(0)> draws <Param(1)> cards"),
            FragmentKind::Sentence,
            &draw_params(),
            &Catalogs::default(),
            &reader(),
        )
        .unwrap();
        assert_eq!(a.tree, b.tree, "citation normalization (spec D5/D6)");
        assert_sites_resolve(&a);
        assert_sites_resolve(&b);

        // And the difference is recorded rather than silently discarded. The
        // singular-safe witness already parses the count in citation-form
        // determiner position; the plural authoring still records the head's
        // number rewrite.
        let kinds: Vec<AgreeKind> = a.agreement.iter().map(|dep| dep.kind).collect();
        assert!(kinds.contains(&AgreeKind::VerbWithHole(0)), "{kinds:?}");
        assert!(
            kinds.contains(&AgreeKind::NounNumberFromHole(1)),
            "{kinds:?}"
        );
        assert_eq!(a.holes[1].witness.text, "1");
        assert!(
            b.agreement
                .iter()
                .flat_map(|dep| &dep.normalized)
                .any(|applied| matches!(applied, Normalization::NounNumber { .. })),
            "{:?}",
            b.agreement
        );
    }

    /// A power/toughness pump: the two numerals sit below the phrase level,
    /// on the `value` of each `SignedScalar`, with the sign left in the
    /// frame. This is the audit's worst-offender class, and the class that
    /// decides whether one entry can replace `PowerAndToughnessUp`/`Down`.
    #[test]
    fn power_toughness_halves_are_their_own_hole_class() {
        let frame = compile(
            &bare("~ gets +<Param(0)>/+<Param(1)> until end of turn"),
            FragmentKind::Sentence,
            &["Count".to_string(), "Count".to_string()],
            &Catalogs::default(),
            &reader(),
        )
        .unwrap();
        assert_sites_resolve(&frame);
        assert_eq!(frame.holes.len(), 3, "two P/T halves plus the `~` hole");
        assert_eq!(frame.holes[0].class, HoleClass::PtHalf);
        assert_eq!(frame.holes[1].class, HoleClass::PtHalf);
        assert_eq!(frame.holes[2].class, HoleClass::SelfRef);
        assert_eq!(frame.holes[2].param, None, "`~` stands for no param");
        // The sign is frame material and stays put; only the value is holed.
        let power = frame.holes[0].path.resolve(&frame.tree).unwrap();
        assert!(matches!(power, View::Hole { .. }));
        let owner = TreePath(frame.holes[0].path.0[..frame.holes[0].path.0.len() - 1].to_vec());
        assert_eq!(
            owner.resolve(&frame.tree).and_then(View::type_name),
            Some("SignedScalar")
        );
    }

    /// Every `~` in one frame is the same referent, so they share **one**
    /// hole — which means `View::Hole` appears once per occurrence while
    /// `holes` gains a single entry, and `Hole::path` keeps only the first
    /// site. A consumer that must visit every occurrence scans the tree for
    /// `View::Hole { index }` rather than trusting the path.
    #[test]
    fn repeated_self_reference_sigils_share_one_hole() {
        let frame = compile(
            &bare("~ and ~ get +<Param(0)>/+<Param(1)>"),
            FragmentKind::Sentence,
            &["Count".to_string(), "Count".to_string()],
            &Catalogs::default(),
            &reader(),
        )
        .unwrap();
        let self_ref = frame
            .holes
            .iter()
            .find(|hole| hole.class == HoleClass::SelfRef)
            .expect("a `~` hole");
        assert_eq!(self_ref.param, None);
        let sites: Vec<usize> = holes_in(&frame.tree)
            .into_iter()
            .filter(|index| *index == self_ref.index)
            .collect();
        assert_eq!(sites.len(), 2, "one `View::Hole` per `~` occurrence");
        assert_eq!(
            frame
                .holes
                .iter()
                .filter(|hole| hole.class == HoleClass::SelfRef)
                .count(),
            1,
            "but one entry in `holes`"
        );
        // The stored path is the first site and resolves there.
        assert!(matches!(
            self_ref.path.resolve(&frame.tree),
            Some(View::Hole { .. })
        ));
    }

    #[test]
    fn a_frame_that_spells_a_reserved_token_is_a_build_error() {
        for text in ["zzhole0 draws a card", "<Param(0)> deals 41 damage"] {
            let error = compile(
                &bare(text),
                FragmentKind::Sentence,
                &deal_damage_params(),
                &Catalogs::default(),
                &reader(),
            )
            .unwrap_err();
            let message = format!("{error:#}");
            assert!(message.contains("reserved"), "{text:?}: {message}");
            assert!(
                message.contains(text),
                "the error names the frame: {message}"
            );
        }
    }

    #[test]
    fn a_frame_that_does_not_parse_is_a_build_error_naming_it() {
        let error = compile(
            &bare("<Param(0)> qqqq wwww <Param(1)>"),
            FragmentKind::Sentence,
            &draw_params(),
            &Catalogs::default(),
            &reader(),
        )
        .unwrap_err();
        let message = format!("{error:#}");
        assert!(message.contains("does not parse cleanly"), "{message}");
        assert!(message.contains("qqqq"), "{message}");
    }

    #[test]
    fn a_non_linear_frame_is_rejected() {
        // `DestroyNoRegen`'s shape: the same param on two constituents. The
        // audit's §4.2 finding — there is no node that covers it.
        let error = compile(
            &bare("<Param(0)> draws <Param(0)>"),
            FragmentKind::Sentence,
            &draw_params(),
            &Catalogs::default(),
            &reader(),
        )
        .unwrap_err();
        assert!(format!("{error:#}").contains("linear"), "{error:#}");
    }

    #[test]
    fn a_declared_param_with_no_hole_needs_a_guard() {
        let unguarded = compile(
            &bare("draw <Param(1)> cards"),
            FragmentKind::Sentence,
            &draw_params(),
            &Catalogs::default(),
            &reader(),
        )
        .unwrap_err();
        assert!(format!("{unguarded:#}").contains("guard"), "{unguarded:#}");

        let guarded = compile(
            &FrameSpec {
                text: "draw <Param(1)> cards".into(),
                when: vec![(0, "You".to_string())],
                position: None,
                announced: Vec::new(),
            },
            FragmentKind::Sentence,
            &draw_params(),
            &Catalogs::default(),
            &reader(),
        )
        .unwrap();
        assert_eq!(guarded.holes.len(), 1);
        assert_eq!(
            guarded.holes[0].index, 0,
            "the hole index is its slot in `holes`"
        );
        assert_eq!(
            guarded.holes[0].param,
            Some(1),
            "and `param` is what it actually binds"
        );
        assert_eq!(guarded.guards.len(), 1);
        assert_eq!(guarded.guards[0].source, "You", "stored as semantic");
        assert_eq!(
            guarded.guards[0].value,
            guard::normalized(deckmaste_semantics::Reference::You),
            "a card-side `You` argument satisfies it through the same function"
        );
        assert_eq!(holes_in(&guarded.tree), vec![0]);
    }

    #[test]
    fn holing_and_guarding_the_same_param_is_rejected() {
        let error = compile(
            &FrameSpec {
                text: "<Param(0)> draws <Param(1)> cards".into(),
                when: vec![(0, "You".to_string())],
                position: None,
                announced: Vec::new(),
            },
            FragmentKind::Sentence,
            &draw_params(),
            &Catalogs::default(),
            &reader(),
        )
        .unwrap_err();
        assert!(format!("{error:#}").contains("pre-bound"), "{error:#}");
    }

    /// A frame whose guard names a non-ground constant does not compile, and
    /// `compile` propagates the refusal rather than swallowing it.
    ///
    /// This is the *propagation* test; the groundness invariant itself is
    /// carried by `guard::tests::ensure_ground_rejects_a_free_param_term`,
    /// which drives `ensure_ground` directly. In this path the refusal comes
    /// from `macro_ron`'s reader, one layer below `ensure_ground` — see that
    /// function's reachability note — so the assertion is on the reader's own
    /// wording.
    ///
    /// It must be on *that* wording and not on "Param": the error's context
    /// is `in frame "draw <Param(1)> cards"`, which contains "Param" for
    /// every failure this frame could have. The control below pins exactly
    /// that — the same frame failing for an unrelated reason must not satisfy
    /// the assertion.
    #[test]
    fn a_frame_whose_guard_is_not_ground_does_not_compile() {
        const REFUSAL: &str = "outside any macro expansion";

        let frame = |guard: &str, text: &str| FrameSpec {
            text: text.into(),
            when: vec![(0, guard.to_string())],
            position: None,
            announced: Vec::new(),
        };
        let compile_one = |spec: FrameSpec| {
            compile(
                &spec,
                FragmentKind::Sentence,
                &draw_params(),
                &Catalogs::default(),
                &reader(),
            )
        };

        let error = compile_one(frame("Param(0)", "draw <Param(1)> cards")).unwrap_err();
        let message = format!("{error:#}");
        assert!(message.contains(REFUSAL), "{message}");

        // Control: the same frame family failing for an unrelated reason. Its
        // message still contains "Param" (from the context), which is why the
        // assertion above cannot be spelled that way.
        let other = compile_one(frame("You", "draw 41 <Param(1)> cards")).unwrap_err();
        let other = format!("{other:#}");
        assert!(other.contains("Param"), "the context always does: {other}");
        assert!(
            !other.contains(REFUSAL),
            "the assertion must not be satisfiable by an unrelated failure: {other}"
        );
    }

    /// A mass-noun frame exercises the verb half of the side table on its
    /// own: `life` has no number to agree with, so the only thing separating
    /// the two authorings is the verb, and citation normalization has to
    /// close that gap unaided.
    #[test]
    fn a_mass_noun_frame_normalizes_the_verb_alone() {
        let compile_one = |text: &str| {
            compile(
                &bare(text),
                FragmentKind::Sentence,
                &draw_params(),
                &Catalogs::default(),
                &reader(),
            )
            .unwrap_or_else(|error| panic!("{text}: {error:#}"))
        };
        let plural = compile_one("<Param(0)> gain <Param(1)> life");
        let singular = compile_one("<Param(0)> gains <Param(1)> life");
        assert_eq!(plural.tree, singular.tree);
        assert_sites_resolve(&plural);
        assert_sites_resolve(&singular);
        assert!(
            plural
                .agreement
                .iter()
                .any(|dep| dep.kind == AgreeKind::VerbWithHole(0)),
            "{:?}",
            plural.agreement
        );
        assert!(
            plural
                .agreement
                .iter()
                .all(|dep| !matches!(dep.kind, AgreeKind::NounNumberFromHole(_))),
            "a mass head has no number to take from the count hole: {:?}",
            plural.agreement
        );
    }

    /// Every recorded agreement site must still address the node kind its
    /// dependency is about — the promise `TreePath`'s own documentation
    /// makes. Cheap, so every successful compile in this module asserts it.
    fn assert_sites_resolve(frame: &CompiledFrame) {
        for dep in &frame.agreement {
            let node = dep.site.resolve(&frame.tree).unwrap_or_else(|| {
                panic!("{:?} site {} does not resolve at all", dep.kind, dep.site)
            });
            let expected = match dep.kind {
                AgreeKind::VerbWithHole(_) => "VerbInstance",
                AgreeKind::NounNumberFromHole(_) => "NominalPhrase",
            };
            assert_eq!(
                node.type_name(),
                Some(expected),
                "{:?} site {} resolved to {node:?}",
                dep.kind,
                dep.site
            );
        }
    }

    /// The macro set every test compiles against: deckmaste's RON dialect
    /// plus one plugin macro, so a guard may be authored with the readable
    /// sugar the catalog actually uses. `params: [Any]` rather than `[Count]`
    /// keeps this a fixture — registering the real param-type set is
    /// `deckmaste_plugin`'s job, and this crate sits below it.
    fn reader() -> MacroSet {
        let mut macros = MacroSet::new(deckmaste_semantics::ron::kinds())
            .with_options(deckmaste_semantics::ron::raw_options());
        let def: macro_ron::MacroDef = macros
            .read_str(r#"(name: "Exactly", kinds: [Quantity], params: [Any], body: Range(Param(0), Param(0)))"#)
            .expect("the fixture definition reads");
        macros.insert(&def).expect("and registers");
        macros
    }

    /// The ruling's headline property, end to end: a guard authored with the
    /// catalog's readable sugar compiles to the same canonical constant as
    /// the long spelling, and a card-side value reaches it through the same
    /// function.
    #[test]
    fn a_guard_holds_any_spelling_of_its_constant() {
        let compile_guard = |source: &str| {
            compile(
                &FrameSpec {
                    text: "target <Param(1)>".into(),
                    when: vec![(0, source.to_string())],
                    position: None,
                    announced: Vec::new(),
                },
                FragmentKind::Nominal,
                &["Quantity".to_string(), "Predicate".to_string()],
                &Catalogs::default(),
                &reader(),
            )
            .unwrap_or_else(|error| panic!("{source}: {error:#}"))
            .guards
            .remove(0)
        };
        let sugar = compile_guard("Exactly(1)");
        let long = compile_guard("Range(Some(1), Some(1))");
        assert_eq!(
            sugar.value, long.value,
            "guard satisfaction is on expanded form"
        );
        assert_eq!(
            sugar.source, "Exactly(1)",
            "but the semantic spelling is kept"
        );
        assert_eq!(
            sugar.value,
            guard::normalized(deckmaste_semantics::Quantity::one()),
            "and a card-side argument reaches it through `guard::normalized`"
        );
    }

    // -----------------------------------------------------------------
    // Citation normalization must keep the side table's own paths valid
    // -----------------------------------------------------------------

    /// Lifting a count out of `modifiers` renumbers every later sibling, so
    /// any agreement site recorded *through* one of them has to be repaired.
    ///
    /// Driven through the real normalization entry point on a hand-built
    /// tree, because English puts nothing that is itself an agreement site
    /// inside an attributive modifier (`NominalModifier` reaches no
    /// `NominalPhrase` or `VerbInstance`), so the shape cannot be reached by
    /// parsing — see `a_second_modifier_survives_the_count_being_lifted` for
    /// the closest authorable analogue. Without the repair the second
    /// dependency's site addresses its neighbour, `normalize_citation` finds
    /// the wrong node shape, bails, and records nothing — a silently
    /// unnormalized tree with no error raised.
    #[test]
    fn a_removed_modifier_repairs_later_agreement_sites() {
        let plural_head = || View::Newtype {
            name: "NounInstance",
            variant: Some("Plural"),
            inner: Box::new(View::Unit {
                name: "Vocab",
                variant: Some("Card"),
            }),
        };
        let nested = View::Node {
            name: "NominalPhrase",
            variant: None,
            fields: vec![
                ("determiner", View::Absent),
                ("modifiers", View::Seq(vec![])),
                ("head", plural_head()),
                ("complements", View::Seq(vec![])),
            ],
        };
        let mut tree = View::Node {
            name: "NominalPhrase",
            variant: None,
            fields: vec![
                ("determiner", View::Absent),
                (
                    "modifiers",
                    View::Seq(vec![
                        View::Newtype {
                            name: "NominalModifier",
                            variant: Some("Quantity"),
                            inner: Box::new(View::Newtype {
                                name: "Quantity",
                                variant: Some("Exact"),
                                inner: Box::new(View::Hole {
                                    index: 0,
                                    class: HoleClass::Numeral,
                                }),
                            }),
                        },
                        View::Newtype {
                            name: "NominalModifier",
                            variant: Some("Adjective"),
                            inner: Box::new(nested),
                        },
                    ]),
                ),
                ("head", plural_head()),
                ("complements", View::Seq(vec![])),
            ],
        };
        let inner_site = TreePath(vec![
            PathStep::Field("modifiers"),
            PathStep::Index(1),
            PathStep::Inner,
        ]);
        let mut agreement = vec![
            AgreementDep {
                site: TreePath::default(),
                kind: AgreeKind::NounNumberFromHole(0),
                normalized: Vec::new(),
            },
            AgreementDep {
                site: inner_site,
                kind: AgreeKind::NounNumberFromHole(1),
                normalized: Vec::new(),
            },
        ];

        normalize_all(&mut tree, &mut agreement, Side::Frame);

        // The outer nominal normalized: count lifted, head singularized.
        assert!(
            agreement[0]
                .normalized
                .contains(&Normalization::QuantityToDeterminer),
            "{:?}",
            agreement[0]
        );
        // The inner one still found its node, and its stored path is still
        // valid against the finished tree.
        assert!(
            agreement[1]
                .normalized
                .contains(&Normalization::NounNumber {
                    from: Number::Plural,
                }),
            "the later site was not normalized: {:?}",
            agreement[1]
        );
        let node = agreement[1]
            .site
            .resolve(&tree)
            .expect("the repaired site resolves");
        assert_eq!(node.type_name(), Some("NominalPhrase"));
        assert_eq!(
            node.children()
                .iter()
                .find_map(|(step, child)| (*step == PathStep::Field("head"))
                    .then(|| child.variant_name()))
                .flatten(),
            Some("Singular")
        );
    }

    /// A second modifier after a singular count survives the singular-witness
    /// retry, and every relocated path remains valid in the normalized tree.
    #[test]
    fn a_second_modifier_survives_the_singular_count_witness_retry() {
        let params = ["Reference", "Count", "Predicate"]
            .map(String::from)
            .to_vec();
        let compile_one = |text: &str| {
            compile(
                &bare(text),
                FragmentKind::Sentence,
                &params,
                &Catalogs::default(),
                &reader(),
            )
            .unwrap_or_else(|error| panic!("{text}: {error:#}"))
        };
        let singular = compile_one("<Param(0)> draw <Param(1)> <Param(2)> card");
        let plural = compile_one("<Param(0)> draws <Param(1)> <Param(2)> cards");
        assert_eq!(singular.tree, plural.tree);
        for frame in [&singular, &plural] {
            assert_sites_resolve(frame);
            for hole in &frame.holes {
                let at = hole.path.resolve(&frame.tree).unwrap_or_else(|| {
                    panic!("hole {} at {} does not resolve", hole.index, hole.path)
                });
                assert!(
                    matches!(at, View::Hole { index, .. } if *index == hole.index),
                    "hole {} at {} resolved to {at:?}",
                    hole.index,
                    hole.path
                );
            }
        }
        assert_eq!(singular.holes[1].witness.text, "1");
        assert!(
            singular
                .agreement
                .iter()
                .any(|dep| dep.kind == AgreeKind::NounNumberFromHole(1))
        );
    }

    #[test]
    fn two_singular_count_holes_share_a_collision_safe_witness_plan() {
        let params = ["Count", "Count"].map(String::from).to_vec();
        let compile_one = |text: &str| {
            compile(
                &bare(text),
                FragmentKind::Nominal,
                &params,
                &Catalogs::default(),
                &reader(),
            )
            .unwrap_or_else(|error| panic!("{text}: {error:#}"))
        };
        let singular = compile_one("<Param(0)> card and <Param(1)> card");
        let plural = compile_one("<Param(0)> cards and <Param(1)> cards");

        assert_eq!(singular.tree, plural.tree);
        assert_sites_resolve(&singular);
        assert_sites_resolve(&plural);
        assert_eq!(
            singular
                .holes
                .iter()
                .map(|hole| hole.witness.text.as_str())
                .collect::<Vec<_>>(),
            ["1", "1"]
        );
        let kinds = singular
            .agreement
            .iter()
            .map(|dependency| dependency.kind)
            .collect::<Vec<_>>();
        assert!(
            kinds.contains(&AgreeKind::NounNumberFromHole(0)),
            "{kinds:?}"
        );
        assert!(
            kinds.contains(&AgreeKind::NounNumberFromHole(1)),
            "{kinds:?}"
        );
    }

    #[test]
    fn four_singular_count_holes_have_a_bijective_witness_plan() {
        let params = ["Count", "Count", "Count", "Count"]
            .map(String::from)
            .to_vec();
        let compile_one = |text: &str| {
            compile(
                &bare(text),
                FragmentKind::Nominal,
                &params,
                &Catalogs::default(),
                &reader(),
            )
            .unwrap_or_else(|error| panic!("{text}: {error:#}"))
        };
        let plural = compile_one(
            "<Param(0)> cards, <Param(1)> cards, <Param(2)> cards, and <Param(3)> cards",
        );
        assert_sites_resolve(&plural);
        assert_eq!(
            plural
                .holes
                .iter()
                .map(|hole| hole.witness.text.as_str())
                .collect::<Vec<_>>(),
            ["41", "43", "47", "53"]
        );

        let singular =
            compile_one("<Param(0)> card, <Param(1)> card, <Param(2)> card, and <Param(3)> card");
        assert_eq!(singular.tree, plural.tree);
        assert_sites_resolve(&singular);
        assert_eq!(
            singular
                .holes
                .iter()
                .map(|hole| hole.witness.text.as_str())
                .collect::<Vec<_>>(),
            ["1", "1", "1", "1"]
        );
        let kinds = singular
            .agreement
            .iter()
            .map(|dependency| dependency.kind)
            .collect::<Vec<_>>();
        for hole in 0..4 {
            assert!(
                kinds.contains(&AgreeKind::NounNumberFromHole(hole)),
                "missing hole {hole}: {kinds:?}"
            );
        }
    }

    #[test]
    fn six_singular_count_holes_reach_the_supported_numeric_capacity() {
        let params = ["Count", "Count", "Count", "Count", "Count", "Count"]
            .map(String::from)
            .to_vec();
        let compile_one = |text: &str| {
            compile(
                &bare(text),
                FragmentKind::Nominal,
                &params,
                &Catalogs::default(),
                &reader(),
            )
            .unwrap_or_else(|error| panic!("{text}: {error:#}"))
        };
        let plural = compile_one(
            "<Param(0)> cards and <Param(1)> cards and <Param(2)> cards and \
             <Param(3)> cards and <Param(4)> cards and <Param(5)> cards",
        );
        assert_sites_resolve(&plural);
        assert_eq!(
            plural
                .holes
                .iter()
                .map(|hole| hole.witness.text.as_str())
                .collect::<Vec<_>>(),
            ["41", "43", "47", "53", "59", "61"]
        );

        let singular = compile_one(
            "<Param(0)> card and <Param(1)> card and <Param(2)> card and \
             <Param(3)> card and <Param(4)> card and <Param(5)> card",
        );
        assert_eq!(singular.tree, plural.tree);
        assert_sites_resolve(&singular);
        assert_eq!(
            singular
                .holes
                .iter()
                .map(|hole| hole.witness.text.as_str())
                .collect::<Vec<_>>(),
            ["1", "1", "1", "1", "1", "1"]
        );
        let kinds = singular
            .agreement
            .iter()
            .map(|dependency| dependency.kind)
            .collect::<Vec<_>>();
        for hole in 0..6 {
            assert!(
                kinds.contains(&AgreeKind::NounNumberFromHole(hole)),
                "missing hole {hole}: {kinds:?}"
            );
        }
    }

    #[test]
    fn repeated_singular_witness_does_not_claim_an_authored_one() {
        let params = ["Count".to_string()];
        let compile_one = |text: &str| {
            compile(
                &bare(text),
                FragmentKind::Nominal,
                &params,
                &Catalogs::default(),
                &reader(),
            )
            .unwrap_or_else(|error| panic!("{text}: {error:#}"))
        };
        let plural = compile_one("<Param(0)> cards and 1 card");
        let singular = compile_one("<Param(0)> card and 1 card");
        assert_eq!(singular.tree, plural.tree);
        assert_sites_resolve(&singular);
        assert_sites_resolve(&plural);
        assert_eq!(singular.holes[0].witness.text, "1");
        assert_eq!(holes_in(&singular.tree), [0]);
        assert_eq!(
            singular
                .tree
                .walk()
                .iter()
                .filter(|(_, node)| matches!(
                    node,
                    View::Scalar { kind: "i32", repr } if repr == "1"
                ))
                .count(),
            1,
            "the authored `1 card` must remain in the normalized tree"
        );
    }

    #[test]
    fn singular_witness_does_not_count_a_grouped_arabic_prefix_as_an_occurrence() {
        use deckmaste_english::syntax::NumberLiteral;

        let params = ["Count".to_string()];
        let compile_one = |text: &str| {
            compile(
                &bare(text),
                FragmentKind::Nominal,
                &params,
                &Catalogs::default(),
                guard::core_reader(),
            )
            .unwrap_or_else(|error| panic!("{text}: {error:#}"))
        };
        let plural = compile_one("<Param(0)> cards and 1,000 cards");
        let singular = compile_one("<Param(0)> card and 1,000 cards");

        assert_eq!(singular.tree, plural.tree);
        assert_sites_resolve(&singular);
        assert_sites_resolve(&plural);
        assert_eq!(singular.holes[0].witness.text, "1");
        assert_eq!(holes_in(&singular.tree), [0]);

        let authored = view::of(&NumberLiteral {
            value: 1_000,
            numeral: Numeral::Arabic(true),
        });
        assert_eq!(
            singular
                .tree
                .walk()
                .iter()
                .filter(|(_, node)| *node == &authored)
                .count(),
            1,
            "the authored grouped-Arabic numeral must remain untouched"
        );
    }

    #[test]
    fn mixed_singular_and_plural_count_holes_use_only_the_needed_retry() {
        let frame = compile(
            &bare("<Param(0)> card and <Param(1)> cards"),
            FragmentKind::Nominal,
            &["Count".to_string(), "Count".to_string()],
            &Catalogs::default(),
            &reader(),
        )
        .unwrap();
        assert_sites_resolve(&frame);
        assert_eq!(
            frame
                .holes
                .iter()
                .map(|hole| hole.witness.text.as_str())
                .collect::<Vec<_>>(),
            ["1", "43"]
        );
    }

    #[test]
    fn singular_count_retry_avoids_fixed_plus_one_literals() {
        let params = ["Count".to_string()];
        let compile_one = |text: &str| {
            compile(
                &bare(text),
                FragmentKind::Sentence,
                &params,
                &Catalogs::default(),
                &reader(),
            )
            .unwrap_or_else(|error| panic!("{text}: {error:#}"))
        };
        let singular = compile_one("Put a +1/+1 counter on <Param(0)> card");
        let plural = compile_one("Put a +1/+1 counter on <Param(0)> cards");
        assert_eq!(singular.tree, plural.tree);
        assert_sites_resolve(&singular);
        assert_sites_resolve(&plural);
        assert_eq!(singular.holes[0].witness.text, "1");
        assert_eq!(plural.holes[0].witness.text, "41");
        assert_eq!(holes_in(&singular.tree), [0]);
    }

    // -----------------------------------------------------------------
    // The relocation-stage checks, at their own entry point
    // -----------------------------------------------------------------

    /// The brief's "compile error if any witness is not found exactly once".
    ///
    /// Driven through `place` directly in both directions. An authored frame
    /// cannot reach the "twice" case — `plan_holes` rejects a repeated
    /// `<Param(i)>` as non-linear first (`a_non_linear_frame_is_rejected`
    /// covers that separate check) — and no pilot frame reaches the "never"
    /// case, since a clean parse keeps its opaque nouns. The trees here are
    /// real parser output; only the planner is bypassed.
    #[test]
    fn a_witness_must_be_found_exactly_once() {
        let planned = |witness: Witness| PlannedHole {
            index: 0,
            param: Some(0),
            witness,
            occurrences: 1,
            marker_occurrence: None,
        };

        let tree = parse_witnessed(
            "zzhole0 deals 41 damage to zzhole0",
            FragmentKind::Sentence,
            &Catalogs::default(),
        )
        .unwrap();
        let twice = place(&tree, planned(witness::lexeme("Predicate", 0))).unwrap_err();
        assert!(
            format!("{twice:#}").contains("found 2 time(s)"),
            "{twice:#}"
        );

        let tree = parse_witnessed(
            "zzhole0 draws a card",
            FragmentKind::Sentence,
            &Catalogs::default(),
        )
        .unwrap();
        let never = place(&tree, planned(witness::lexeme("Predicate", 7))).unwrap_err();
        assert!(
            format!("{never:#}").contains("found 0 time(s)"),
            "{never:#}"
        );
    }

    #[test]
    fn a_number_literal_marker_requires_the_witness_notation() {
        let witness = witness::singular_numeral();
        for (text, expected) in [("1 card", 1), ("one card", 0), ("I card", 0)] {
            let tree = parse_witnessed(text, FragmentKind::Nominal, &Catalogs::default())
                .unwrap_or_else(|error| panic!("{text}: {error:#}"));
            let found = tree
                .walk()
                .into_iter()
                .filter(|(path, node)| is_marker_at(&tree, path, node, &witness))
                .count();
            assert_eq!(found, expected, "{text}");
        }
    }

    /// Compiles a `Nominal` frame with one `Predicate` hole.
    fn compile_filter(text: &str) -> anyhow::Result<CompiledFrame> {
        compile(
            &bare(text),
            FragmentKind::Nominal,
            &["Predicate".to_string()],
            &Catalogs::default(),
            &reader(),
        )
    }

    /// The complement-side asymmetry: `<Param(0)> you control` is the mirror
    /// of `target <Param(0)>`, and the pilot lexicon needs both.
    ///
    /// `creature you control` lowers to a `NominalPhrase` whose `complements`
    /// hold a zero-marked object-gap `RelativeClause` — the postmodifier is
    /// the frame's — so the hole claims the premodifiers and the head only.
    /// The determiner stays outside the hole: a filter predicate is
    /// determinerless by construction, and the enclosing frame supplies it.
    #[test]
    fn a_postmodified_filter_hole_claims_only_the_premodifiers_and_head() {
        let frame = compile_filter("<Param(0)> you control").expect("a real constituent");
        assert_eq!(frame.holes.len(), 1);
        assert_eq!(
            frame.holes[0].class,
            HoleClass::FieldSlice {
                claimed: vec!["modifiers", "head"],
            }
        );
        assert_sites_resolve(&frame);

        let owner = frame.holes[0].path.resolve(&frame.tree).unwrap();
        assert_eq!(owner.type_name(), Some("NominalPhrase"));
        let View::Node { fields, .. } = owner else { panic!("{owner:?}") };
        assert_eq!(fields.len(), 4);
        // determiner: outside the hole, and still the frame's empty slot.
        assert_eq!(fields[0].0, "determiner");
        assert!(fields[0].1.is_vacuous());
        // modifiers + head: the hole.
        for (name, value) in &fields[1..3] {
            assert!(
                matches!(value, View::Hole { index: 0, .. }),
                "{name} should be part of the slice, got {value:?}"
            );
        }
        // complements: the frame's relative clause, untouched.
        assert_eq!(fields[3].0, "complements");
        assert!(
            !fields[3].1.is_vacuous(),
            "the frame's postmodifier must survive: {:?}",
            fields[3].1
        );
        assert!(
            !holes_in(&fields[3].1).contains(&0),
            "the hole must not have swallowed the frame's complement"
        );
    }

    /// The two supported asymmetries claim *different* field sets — the
    /// contract a consumer of a `FieldSlice` hole must read rather than
    /// assume.
    #[test]
    fn the_two_supported_asymmetries_claim_different_sets() {
        let determiner_side = compile_filter("target <Param(0)>").unwrap();
        let complement_side = compile_filter("<Param(0)> you control").unwrap();
        let claimed = |frame: &CompiledFrame| match &frame.holes[0].class {
            HoleClass::FieldSlice { claimed } => claimed.clone(),
            other => panic!("{other:?}"),
        };
        assert_eq!(
            claimed(&determiner_side),
            ["modifiers", "head", "complements"]
        );
        assert_eq!(claimed(&complement_side), ["modifiers", "head"]);
        assert_ne!(claimed(&determiner_side), claimed(&complement_side));
    }

    /// The field-slice guard still refuses everything outside the two
    /// supported asymmetries. These are each one edit away from a legal shape
    /// and must not be confused with them:
    ///
    /// - `target <Param(0)> you control` differs from the newly-legal
    ///   `<Param(0)> you control` **only** by the determiner — and that is
    ///   exactly what makes it illegal, since the hole would be a discontinuous
    ///   slice with frame material on both sides.
    /// - `white <Param(0)>` is the coherent third shape (frame owns `modifiers`
    ///   alone, hole would claim `["head", "complements"]`). It was considered
    ///   and deliberately left out of scope, so it must keep refusing rather
    ///   than quietly start working.
    #[test]
    fn frame_material_beside_a_field_slice_hole_is_refused() {
        for (text, held) in [
            // Frame owns both ends: discontinuous, refused.
            (
                "target <Param(0)> you control",
                vec!["determiner", "complements"],
            ),
            ("target white <Param(0)>", vec!["determiner", "modifiers"]),
            // Frame owns the premodifier alone: out of scope, refused.
            ("white <Param(0)>", vec!["modifiers"]),
        ] {
            let error = compile_filter(text).unwrap_err();
            let message = format!("{error:#}");
            // Assert on the exact list the error reports as frame material,
            // not on a bare field name: the message's explanatory suffix
            // names every field while spelling out the two legal claims, so a
            // substring test for `modifiers` would pass for any of these and
            // could not tell the three refusals apart.
            assert!(
                message.contains(&format!("{held:?}")),
                "{text:?} should report frame material in exactly {held:?}: {message}"
            );
            assert!(message.contains(text), "and name the frame: {message}");
        }
    }

    /// The frame catalog's own entries have to compile, or there is nothing
    /// for further entries to be authored against.
    #[test]
    fn the_seeded_constructor_catalog_compiles() {
        let dir =
            std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../plugins/builtin/frames");
        let catalog = macro_ron::frames::load_constructor_frames(&dir).unwrap();
        assert!(!catalog.is_empty());
        let macros = reader();
        for entry in &catalog {
            for spec in &entry.frames {
                // Read off the entry's own declared, required `kind:`
                // (`ConstructorFrames::kind`'s own doc: there is nothing to
                // guess it from, and registering at every category a frame
                // happens to parse cleanly at costs several entries where one
                // is meant) through the one schema-to-engine bridge, rather
                // than guessing a category from the constructor's name.
                let kind = crate::lexicon::fragment_kind_of(entry.kind);
                let frame = compile(spec, kind, &entry.params, &Catalogs::default(), &macros)
                    .unwrap_or_else(|error| panic!("{}: {error:#}", entry.constructor));
                // A hole-free literal frame is legitimate (the `Player`
                // macro's bare "player" is the existing precedent); only a
                // holed entry is required to have compiled at least one.
                assert!(
                    entry.params.is_empty() || !frame.holes.is_empty(),
                    "{} declares params but compiled with no holes",
                    entry.constructor
                );
                assert_sites_resolve(&frame);
            }
        }
    }
}
