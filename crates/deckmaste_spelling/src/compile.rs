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
//!    [`ProjectionTree::Hole`] back in its place. Which node the hole replaces
//!    is not the leaf the witness lexed to but the largest subtree that came
//!    *only* from that witness (see [`HoleClass`]).
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
use deckmaste_english::ProjectedAtom;
use deckmaste_english::features::Number;
use deckmaste_english::features::Person;
use deckmaste_english::parse_fragment;
use deckmaste_english::predicate::VerbAnalysis;
use deckmaste_english::project_fragment;
use deckmaste_english::syntax::ScalarValue;
use deckmaste_english::word::Noun;
use deckmaste_english::word::NounInstance;
use deckmaste_english::word::NounInstanceKind;
use macro_ron::MacroSet;
use macro_ron::frames::FrameSpec;

use crate::View;
use crate::guard;
use crate::projection::ProjectionPath;
use crate::projection::ProjectionStep;
use crate::projection::ProjectionTree;
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
    /// the `NounPhraseKind::ThisCard` projection the parser exposes, since the
    /// witness name never survives into the tree as a spelling.
    SelfRef,
}

/// One hole in a compiled frame.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Hole {
    /// This hole's slot in [`CompiledFrame::holes`], and the `index` its
    /// [`ProjectionTree::Hole`] nodes carry — `holes[h.index] == h`, always.
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
    /// Where the hole sits in [`CompiledFrame::tree`]: the node it replaced.
    ///
    /// **One path, even when the hole has several sites.** Every `~` in a
    /// frame is the same referent and shares one hole, so a frame saying `~`
    /// twice puts two `ProjectionTree::Hole` nodes in the tree and `path` keeps
    /// only the first. A consumer that must visit every occurrence — a
    /// renderer substituting a filler back — scans the tree for
    /// `ProjectionTree::Hole { index, .. }` rather than trusting `path`, which
    /// is a convenience for the single-site case and for diagnostics.
    pub path: ProjectionPath,
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
    pub site: ProjectionPath,
    pub kind: AgreeKind,
    /// What citation normalization actually changed at `site`, in the order
    /// applied. Empty when the authoring was already in citation form.
    pub normalized: Vec<Normalization>,
}

/// Which feature flows from which hole.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Ord, PartialOrd)]
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
    /// The frame's tree, with [`ProjectionTree::Hole`] where the witnesses were
    /// and hole-driven inflection normalized to citation form.
    pub tree: ProjectionTree,
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
    clear_hole_dependent_witnesses(&mut tree);
    normalize_all(&mut tree, &mut agreement);

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

fn parse_witnessed(
    text: &str,
    kind: FragmentKind,
    catalogs: &Catalogs,
) -> anyhow::Result<ProjectionTree> {
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
    let projection = project_fragment(&fragment)
        .map_err(|error| anyhow::anyhow!("cannot project witnessed frame {text:?}: {error}"))?;
    Ok(ProjectionTree::from_projection(projection))
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
    sites: Vec<ProjectionPath>,
}

impl PlacedHole {
    fn hole_node(&self) -> ProjectionTree {
        ProjectionTree::Hole {
            index: self.planned.index,
            class: self.class.clone(),
        }
    }

    /// The final record, with the path re-derived from the finished tree —
    /// citation normalization moves nodes around, so a path taken before it
    /// ran cannot be trusted.
    fn finish(self, tree: &ProjectionTree) -> anyhow::Result<Hole> {
        let index = self.planned.index;
        let found: Vec<ProjectionPath> = tree
            .walk()
            .into_iter()
            .filter(
                |(_, node)| matches!(node, ProjectionTree::Hole { index: at, .. } if *at == index),
            )
            .map(|(path, _)| path)
            .collect();
        let first = found.first().cloned().ok_or_else(|| {
            anyhow::anyhow!("hole {index} vanished from the tree during normalization")
        })?;
        Ok(Hole {
            index,
            param: self.planned.param,
            class: self.class,
            path: first,
            witness: self.planned.witness,
        })
    }
}

/// Locates one planned hole in the parsed tree and decides its class.
#[cfg(test)]
fn place(tree: &ProjectionTree, planned: PlannedHole) -> anyhow::Result<PlacedHole> {
    let markers = select_markers(tree, &planned)?;
    Ok(place_at_markers(tree, planned, markers))
}

fn select_markers(
    tree: &ProjectionTree,
    planned: &PlannedHole,
) -> anyhow::Result<Vec<ProjectionPath>> {
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
    tree: &ProjectionTree,
    planned: PlannedHole,
    markers: Vec<ProjectionPath>,
) -> PlacedHole {
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
        let (site_class, site) = classify(tree, site, witness);
        class.get_or_insert(site_class);
        sites.push(site);
    }
    PlacedHole {
        class: class.expect("at least one occurrence"),
        planned,
        sites,
    }
}

fn place_all(tree: &ProjectionTree, planned: Vec<PlannedHole>) -> anyhow::Result<Vec<PlacedHole>> {
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
        placed.push(place_at_markers(tree, hole, markers));
    }
    Ok(placed)
}

fn atom_value(atom: &ProjectedAtom) -> Option<&deckmaste_english::OwnedProjectionValue> {
    match atom {
        ProjectedAtom::Scalar { value, .. }
        | ProjectedAtom::Identity { value, .. }
        | ProjectedAtom::FlatSubtree { value, .. } => Some(value),
        ProjectedAtom::DerivedSequenceScalar { .. } => None,
    }
}

fn projected_noun_instance(node: &ProjectionTree) -> Option<&NounInstance> {
    let construction = node.construction()?;
    if construction.category != "Noun" {
        return None;
    }
    construction
        .roles
        .get("identity")?
        .atom()
        .and_then(atom_value)?
        .downcast_ref()
}

fn projected_noun_number(node: &ProjectionTree) -> Option<Number> {
    match projected_noun_instance(node)?.kind() {
        NounInstanceKind::Singular(_) => Some(Number::Singular),
        NounInstanceKind::Plural(_) => Some(Number::Plural),
        NounInstanceKind::Mass(_) => None,
    }
}

fn projected_opaque_noun_spelling(node: &ProjectionTree) -> Option<&str> {
    let Noun::Opaque(lexeme) = projected_noun_instance(node)?.noun() else {
        return None;
    };
    Some(lexeme.spelling())
}

fn numeric_scalar_value(node: &ProjectionTree) -> Option<i32> {
    let value = node.atom().and_then(atom_value)?;
    if let Some(value) = value.downcast_ref::<i32>() {
        return Some(*value);
    }
    if let Some(value) = value.downcast_ref::<u32>() {
        return i32::try_from(*value).ok();
    }
    match value.downcast_ref::<ScalarValue>() {
        Some(ScalarValue::Integer(value)) => i32::try_from(*value).ok(),
        Some(ScalarValue::X | ScalarValue::Star) | None => None,
    }
}

fn number_literal_notation_matches(
    tree: &ProjectionTree,
    path: &ProjectionPath,
    expected: Numeral,
) -> bool {
    if path.0.last() != Some(&ProjectionStep::Role("value")) {
        return false;
    }
    let Some(parent) = path.parent().and_then(|parent| parent.resolve(tree)) else {
        return false;
    };
    let Some(construction) = parent.construction() else {
        return false;
    };
    if construction.construction != "flat_number_literal" {
        return false;
    }
    construction
        .roles
        .get("notation")
        .and_then(ProjectionTree::atom)
        .and_then(atom_value)
        .and_then(|value| value.downcast_ref::<Numeral>())
        .is_some_and(|notation| *notation == expected)
}

fn is_number_literal_value(tree: &ProjectionTree, path: &ProjectionPath) -> bool {
    path.0.last() == Some(&ProjectionStep::Role("value"))
        && path
            .parent()
            .and_then(|parent| parent.resolve(tree))
            .and_then(ProjectionTree::construction)
            .is_some_and(|construction| construction.construction == "flat_number_literal")
}

/// Whether this node is where a witness bottomed out.
fn is_marker_at(
    tree: &ProjectionTree,
    path: &ProjectionPath,
    node: &ProjectionTree,
    witness: &Witness,
) -> bool {
    match witness.kind {
        WitnessKind::Lexeme => {
            projected_opaque_noun_spelling(node).is_some_and(|spelling| spelling == witness.text)
        }
        WitnessKind::Numeral { value, notation } => {
            numeric_scalar_value(node) == Some(value)
                && (!is_number_literal_value(tree, path)
                    || number_literal_notation_matches(tree, path, notation))
        }
        WitnessKind::SelfReference => node.construction().is_some_and(|construction| {
            matches!(
                construction.construction,
                "noun_phrase_this_card" | "noun_phrase_full_this_card"
            ) || construction.construction == "flat_phrase" && construction.form == "this_card"
        }),
    }
}

/// The shallowest ancestor of `marker` that still came only from `witness`.
///
/// Walking up stops at the first ancestor that holds frame material, and
/// never reaches the root, so `tree` itself never becomes a bare hole and
/// keeps its `Fragment::<kind>` wrapper.
fn hoist(tree: &ProjectionTree, marker: &ProjectionPath, witness: &Witness) -> ProjectionPath {
    let mut best = marker.clone();
    for depth in (0..marker.0.len()).rev() {
        let ancestor = ProjectionPath(marker.0[..depth].to_vec());
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
/// "Non-empty" is [`ProjectionTree::is_vacuous`]: absent optional roles and
/// empty sequences are not frame material, while an occupied grammatical role
/// contributed by the frame stops the hoist.
fn witness_only(node: &ProjectionTree, witness: &Witness) -> bool {
    let is_marker_value = match witness.kind {
        WitnessKind::Numeral { value, .. } => numeric_scalar_value(node) == Some(value),
        WitnessKind::Lexeme => {
            projected_opaque_noun_spelling(node).is_some_and(|spelling| spelling == witness.text)
        }
        WitnessKind::SelfReference => node.construction().is_some_and(|construction| {
            matches!(
                construction.construction,
                "noun_phrase_full_this_card" | "noun_phrase_abbreviated_this_card"
            )
        }),
    };
    if is_marker_value {
        return true;
    }
    if node.construction().is_some_and(|construction| {
        !construction.literals.is_empty() || !construction.witnesses.is_empty()
    }) {
        return false;
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

/// Decides what kind of hole the node at `site` is.
fn classify(
    tree: &ProjectionTree,
    site: ProjectionPath,
    witness: &Witness,
) -> (HoleClass, ProjectionPath) {
    if witness.kind == WitnessKind::SelfReference {
        return (HoleClass::SelfRef, site);
    }
    let last = site.0.last().copied();
    let parent = site.parent().unwrap_or_default();
    let parent_construction = parent
        .resolve(tree)
        .and_then(ProjectionTree::construction)
        .map(|construction| construction.construction);

    if parent_construction == Some("flat_signed_scalar")
        && last == Some(ProjectionStep::Role("value"))
    {
        return (HoleClass::PtHalf, site);
    }
    if matches!(witness.kind, WitnessKind::Numeral { .. }) {
        return (HoleClass::Numeral, site);
    }
    (HoleClass::Subtree, site)
}

/// Puts every hole into the tree, replacing what the witnesses left.
fn relocate(tree: &mut ProjectionTree, placed: &[PlacedHole]) {
    for hole in placed {
        let node = hole.hole_node();
        for site in &hole.sites {
            match &hole.class {
                HoleClass::Numeral => {
                    if site.0.last() == Some(&ProjectionStep::Role("value"))
                        && let Some(parent) = site.parent()
                        && parent
                            .resolve(tree)
                            .and_then(ProjectionTree::construction)
                            .is_some_and(|construction| {
                                construction.construction == "flat_number_literal"
                            })
                        && let Some(notation) = parent
                            .then(ProjectionStep::Role("notation"))
                            .resolve_mut(tree)
                    {
                        let citation = Numeral::Arabic(false);
                        *notation = ProjectionTree::Atom(ProjectedAtom::Scalar {
                            codec: "Numeral",
                            value: deckmaste_english::OwnedProjectionValue::new(&citation),
                        });
                    }
                    if let Some(slot) = site.resolve_mut(tree) {
                        *slot = node.clone();
                    }
                }
                HoleClass::Subtree | HoleClass::PtHalf | HoleClass::SelfRef => {
                    if let Some(slot) = site.resolve_mut(tree) {
                        *slot = node.clone();
                    }
                }
            }
        }
    }
}

/// Drops synthetic parse witnesses from any construction whose projected
/// value now contains a frame hole. Such a witness described the temporary
/// token used to compile the frame, not frame identity.
fn clear_hole_dependent_witnesses(tree: &mut ProjectionTree) -> bool {
    match tree {
        ProjectionTree::Construction(construction) => {
            let mut contains_hole = false;
            for role in construction.roles.values_mut() {
                contains_hole |= clear_hole_dependent_witnesses(role);
            }
            if contains_hole {
                construction.witnesses.clear();
            }
            contains_hole
        }
        ProjectionTree::Element(element) => {
            let mut contains_hole = false;
            for role in element.roles.values_mut() {
                contains_hole |= clear_hole_dependent_witnesses(role);
            }
            contains_hole
        }
        ProjectionTree::Variant(variant) => {
            let mut contains_hole = false;
            for role in variant.roles.values_mut() {
                contains_hole |= clear_hole_dependent_witnesses(role);
            }
            contains_hole
        }
        ProjectionTree::Product(product) => {
            let mut contains_hole = false;
            for role in product.roles.values_mut() {
                contains_hole |= clear_hole_dependent_witnesses(role);
            }
            contains_hole
        }
        ProjectionTree::Optional(value) => value
            .as_deref_mut()
            .is_some_and(clear_hole_dependent_witnesses),
        ProjectionTree::Sequence { members, .. } => {
            let mut contains_hole = false;
            for member in members {
                contains_hole |= clear_hole_dependent_witnesses(member);
            }
            contains_hole
        }
        ProjectionTree::Hole { .. } => true,
        ProjectionTree::Atom(_) => false,
    }
}

// ---------------------------------------------------------------------------
// Step 4 — the agreement side table and citation normalization
// ---------------------------------------------------------------------------

/// Finds the nodes whose inflection a hole decides. Runs on the *witnessed*
/// tree, before relocation, because the role markers it keys off — a
/// `Subject` beside a `HeadedPredicate` — are exactly the wrappers a hole may
/// be about to swallow.
fn agreement_deps(tree: &ProjectionTree, placed: &[PlacedHole]) -> Vec<AgreementDep> {
    let mut deps = Vec::new();
    for (path, node) in tree.walk() {
        let Some(construction) = node.construction() else {
            continue;
        };
        if construction.construction != "simple_clause_subject" {
            continue;
        }
        let subject = path.then(ProjectionStep::Role("subject"));
        let predicate = path.then(ProjectionStep::Role("predicate"));
        let Some(predicate_tree) = predicate.resolve(tree) else {
            continue;
        };
        let Some((relative_verb, _)) = predicate_tree.walk().into_iter().find(|(_, candidate)| {
            candidate.construction().is_some_and(|candidate| {
                candidate.construction == "verb"
                    && candidate.roles.get("head").is_some_and(|head| {
                        matches!(
                            head.atom(),
                            Some(ProjectedAtom::Identity {
                                provider: "LexicalVerb",
                                value_type: "VerbAnalysis",
                                ..
                            })
                        )
                    })
            })
        }) else {
            continue;
        };
        let site = join_paths(&predicate, &relative_verb).then(ProjectionStep::Role("head"));
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

    for hole in placed
        .iter()
        .filter(|hole| hole.class == HoleClass::Numeral)
    {
        for marker in &hole.sites {
            let Some(owner) = nearest_category_ancestor(tree, marker, "NominalPhrase") else {
                continue;
            };
            let Some(owner_tree) = owner.resolve(tree) else {
                continue;
            };
            for (relative_head, head) in owner_tree.walk() {
                if projected_noun_number(head).is_some()
                    && relative_head.0.last() == Some(&ProjectionStep::Role("head"))
                {
                    deps.push(AgreementDep {
                        site: join_paths(&owner, &relative_head),
                        kind: AgreeKind::NounNumberFromHole(hole.planned.index),
                        normalized: Vec::new(),
                    });
                }
            }
        }
    }
    deps.sort_by(|left, right| left.site.cmp(&right.site).then(left.kind.cmp(&right.kind)));
    deps.dedup_by(|left, right| left.site == right.site && left.kind == right.kind);
    deps
}

fn join_paths(parent: &ProjectionPath, child: &ProjectionPath) -> ProjectionPath {
    let mut steps = parent.0.clone();
    steps.extend_from_slice(&child.0);
    ProjectionPath(steps)
}

fn nearest_category_ancestor(
    tree: &ProjectionTree,
    path: &ProjectionPath,
    category: &str,
) -> Option<ProjectionPath> {
    (0..=path.0.len()).rev().find_map(|len| {
        let ancestor = ProjectionPath(path.0[..len].to_vec());
        ancestor
            .resolve(tree)
            .and_then(ProjectionTree::construction)
            .is_some_and(|construction| construction.category == category)
            .then_some(ancestor)
    })
}

/// Runs citation normalization over every agreement site. The same rewrite is
/// shared by frame compilation and card-side unification so both compare in
/// one canonical person/number form.
pub(crate) fn normalize_all(tree: &mut ProjectionTree, agreement: &mut [AgreementDep]) {
    for dep in agreement {
        dep.normalized = normalize_citation(tree, &dep.site, dep.kind);
    }
}

/// Rewrites one agreement site to citation form and returns what changed.
fn normalize_citation(
    tree: &mut ProjectionTree,
    at: &ProjectionPath,
    kind: AgreeKind,
) -> Vec<Normalization> {
    let mut applied = Vec::new();
    let Some(node) = at.resolve_mut(tree) else {
        return applied;
    };
    match kind {
        AgreeKind::VerbWithHole(_) => {
            let ProjectionTree::Atom(ProjectedAtom::Identity {
                provider: "LexicalVerb",
                value_type: "VerbAnalysis",
                value,
            }) = node
            else {
                return applied;
            };
            let Some(analysis) = value.downcast_ref::<VerbAnalysis>() else {
                return applied;
            };
            let deckmaste_english::features::VerbSlot::Present { person, number } =
                analysis.instance().slot
            else {
                return applied;
            };
            let was = (person, number);
            let citation = analysis.citation_form();
            if was != (Person::Third, Number::Singular) {
                applied.push(Normalization::VerbAgreement {
                    person: was.0,
                    number: was.1,
                });
            }
            *value = deckmaste_english::OwnedProjectionValue::new(&citation);
        }
        AgreeKind::NounNumberFromHole(_) => {
            let Some(instance) = projected_noun_instance(node) else {
                return applied;
            };
            let NounInstanceKind::Plural(noun) = instance.kind() else {
                return applied;
            };
            let Ok(singular) = NounInstance::try_singular(noun.clone()) else {
                return applied;
            };
            let Some(identity) = node.role_mut("identity") else {
                return applied;
            };
            let Some(ProjectedAtom::Identity {
                provider,
                value_type,
                ..
            }) = identity.atom()
            else {
                return applied;
            };
            let (provider, value_type) = (*provider, *value_type);
            *identity = ProjectionTree::Atom(ProjectedAtom::Identity {
                provider,
                value_type,
                value: deckmaste_english::OwnedProjectionValue::new(&singular),
            });
            applied.push(Normalization::NounNumber {
                from: Number::Plural,
            });
        }
    }
    applied
}

#[cfg(test)]
fn projected_scalar<T: std::any::Any>(node: Option<&ProjectionTree>) -> Option<&T> {
    let atom = node?.atom()?;
    atom_value(atom)?.downcast_ref::<T>()
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

    fn holes_in(tree: &ProjectionTree) -> Vec<usize> {
        tree.walk()
            .into_iter()
            .filter_map(|(_, node)| match node {
                ProjectionTree::Hole { index, .. } => Some(*index),
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
                matches!(at, ProjectionTree::Hole { index, .. } if *index == hole.index),
                "hole {} at {} resolved to {at:?}",
                hole.index,
                hole.path
            );
        }
    }

    #[test]
    fn filter_hole_uses_the_declared_nominal_role() {
        let frame = compile(
            &bare("target <Param(0)>"),
            FragmentKind::Nominal,
            &["Predicate".to_string()],
            &Catalogs::default(),
            &reader(),
        )
        .unwrap();
        assert_eq!(frame.holes[0].class, HoleClass::Subtree);
        assert!(matches!(
            frame.holes[0].path.0.last(),
            Some(ProjectionStep::Role("nominal"))
        ));
        assert!(matches!(
            frame.holes[0].path.resolve(&frame.tree),
            Some(ProjectionTree::Hole { index: 0, .. })
        ));
    }

    /// A bare `<Param(0)>` at the same category has no frame determiner, so
    /// the hole grows past the nominal and is an ordinary subtree — the
    /// control that shows the hoist is determined by grammatical frame
    /// material rather than by the category alone.
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
        assert!(matches!(power, ProjectionTree::Hole { .. }));
        let owner = frame.holes[0].path.parent().unwrap();
        assert_eq!(
            owner
                .resolve(&frame.tree)
                .and_then(ProjectionTree::construction)
                .map(|construction| construction.construction),
            Some("flat_signed_scalar")
        );
    }

    /// Every `~` in one frame is the same referent, so they share **one**
    /// hole — which means `ProjectionTree::Hole` appears once per occurrence
    /// while `holes` gains a single entry, and `Hole::path` keeps only the
    /// first site. A consumer that must visit every occurrence scans the
    /// tree for `ProjectionTree::Hole { index }` rather than trusting the
    /// path.
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
        assert_eq!(
            sites.len(),
            2,
            "one `ProjectionTree::Hole` per `~` occurrence"
        );
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
            Some(ProjectionTree::Hole { .. })
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
    /// dependency is about — the promise [`ProjectionPath`]'s own documentation
    /// makes. Cheap, so every successful compile in this module asserts it.
    fn assert_sites_resolve(frame: &CompiledFrame) {
        for dep in &frame.agreement {
            let node = dep.site.resolve(&frame.tree).unwrap_or_else(|| {
                panic!("{:?} site {} does not resolve at all", dep.kind, dep.site)
            });
            let construction = node.construction();
            let (actual, expected) = match dep.kind {
                AgreeKind::VerbWithHole(_) => (
                    node.atom().and_then(|atom| match atom {
                        ProjectedAtom::Identity { provider, .. } => Some(*provider),
                        _ => None,
                    }),
                    "LexicalVerb",
                ),
                AgreeKind::NounNumberFromHole(_) => (
                    construction.map(|construction| construction.category),
                    "Noun",
                ),
            };
            assert_eq!(
                actual,
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
                    matches!(at, ProjectionTree::Hole { index, .. } if *index == hole.index),
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
                .filter(|(_, node)| numeric_scalar_value(node) == Some(1))
                .count(),
            1,
            "the authored `1 card` must remain in the normalized tree"
        );
    }

    #[test]
    fn singular_witness_does_not_count_a_grouped_arabic_prefix_as_an_occurrence() {
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

        assert_eq!(
            singular
                .tree
                .walk()
                .iter()
                .filter(|(_, node)| {
                    node.construction().is_some_and(|construction| {
                        construction.construction == "flat_number_literal"
                            && construction
                                .roles
                                .get("value")
                                .and_then(numeric_scalar_value)
                                == Some(1_000)
                            && projected_scalar::<Numeral>(construction.roles.get("notation"))
                                == Some(&Numeral::Arabic(true))
                    })
                })
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

    /// A postmodified filter exposes the base nominal as a named role while
    /// retaining the relative-clause construction beside it.
    #[test]
    fn a_postmodified_filter_holes_the_declared_nominal_role() {
        let frame = compile_filter("<Param(0)> you control").expect("a real constituent");
        assert_eq!(frame.holes.len(), 1);
        assert_eq!(frame.holes[0].class, HoleClass::Subtree);
        assert_sites_resolve(&frame);
        assert!(matches!(
            frame.holes[0].path.0.last(),
            Some(ProjectionStep::Role("nominal"))
        ));
        assert!(frame.tree.walk().into_iter().any(|(_, node)| {
            node.construction()
                .is_some_and(|construction| construction.category == "RelativeClause")
        }));
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
