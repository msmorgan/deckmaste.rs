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
//!    `zzhole0` for a phrasal hole, a reserved numeral `41` for a count hole,
//!    the face name `Zzframeself` for `~`. The result is ordinary English.
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
    /// erases the N-bar the parse forest had, so in `target <Param(0)>` the
    /// determiner belongs to the *frame* and the hole covers the remaining
    /// three fields. There is no single node to point at, so the hole's
    /// [`Hole::path`] addresses the owning `NominalPhrase` and `claimed`
    /// names the fields the hole takes.
    ///
    /// # How to read one
    ///
    /// `View::Hole` appears **once per claimed field** — three times, for the
    /// only shape the pilot produces — each carrying the same `index` and the
    /// same `class`. The owning node therefore keeps its exact arity and
    /// field names, and the frame's own `determiner` sits *beside* the holes
    /// rather than being spliced around them.
    ///
    /// So a consumer walking the two trees in lockstep needs no special case:
    /// at a `Hole`-valued field it binds the card node's same-named field,
    /// at any other field it compares. The three fields are one hole, not
    /// three — **treat them as a unit**: bind all of `claimed` or none, and do
    /// not treat a per-field match as a hole match on its own. `claimed` is
    /// the authority on the grouping; the repetition is a convenience for the
    /// walk, not three independent bindings.
    FieldSlice { claimed: Vec<&'static str> },
    /// The hole is a bare number: the `value` of a `NumberLiteral`. The
    /// sibling `numeral` field (Arabic vs. spelled-out) is the frame's, not
    /// the filler's, so it is deliberately left outside the hole.
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
    VerbAgreement {
        person: &'static str,
        number: &'static str,
    },
    /// A plural head noun was reset to singular.
    NounNumber { from: &'static str },
    /// A hole-bearing `NominalModifier::Quantity` was moved into the empty
    /// `determiner` slot as `Determiner::Quantity`.
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
/// `deckmaste_core` constructor at all. A compiler that cannot see the macros
/// therefore cannot evaluate the guards it is handed. Pass
/// [`guard::core_reader`] only for a frame set known to guard with bare core
/// constructors.
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
    let mut tree = parse_witnessed(&plan.text, kind, catalogs).map_err(context)?;

    let mut placed = Vec::new();
    for planned in plan.holes {
        placed.push(place(&tree, planned).map_err(context)?);
    }

    let mut agreement = agreement_deps(&tree, &placed);
    relocate(&mut tree, &placed);
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
}

struct Plan {
    text: String,
    holes: Vec<PlannedHole>,
}

fn plan_holes(spec: &FrameSpec, params: &[String]) -> anyhow::Result<Plan> {
    let reserved = witness::reserved_tokens(&spec.text);
    anyhow::ensure!(
        reserved.is_empty(),
        "spells the reserved witness token(s) {reserved:?}; `zz…` and the numerals {:?} \
         are reserved for the frame compiler's own substitutions",
        witness::RESERVED_NUMERALS,
    );

    let mut text = String::with_capacity(spec.text.len());
    let mut by_param: Vec<Option<Witness>> = vec![None; params.len()];
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
                let witness = witness::numeral(numeric_slot)?;
                numeric_slot += 1;
                witness
            } else {
                witness::lexeme(param_type, param)
            };
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

    let mut holes: Vec<PlannedHole> = order
        .into_iter()
        .filter_map(|param| {
            by_param[param].clone().map(|witness| PlannedHole {
                index: 0,
                param: Some(param),
                witness,
                occurrences: 1,
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
        });
    }
    for (index, hole) in holes.iter_mut().enumerate() {
        hole.index = index;
    }
    Ok(Plan { text, holes })
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
fn place(tree: &View, planned: PlannedHole) -> anyhow::Result<PlacedHole> {
    let witness = &planned.witness;
    let markers: Vec<TreePath> = tree
        .walk()
        .into_iter()
        .filter(|(_, node)| is_marker(node, witness))
        .map(|(path, _)| path)
        .collect();
    anyhow::ensure!(
        markers.len() == planned.occurrences,
        "witness `{witness}` for hole {} was found {} time(s) in the parse, expected {}",
        planned.index,
        markers.len(),
        planned.occurrences,
    );

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

/// Whether this node is where a witness bottomed out.
fn is_marker(node: &View, witness: &Witness) -> bool {
    match witness.kind {
        WitnessKind::Lexeme => {
            matches!(node, View::Scalar { kind: "str", repr } if *repr == witness.text)
        }
        WitnessKind::Numeral => matches!(
            node,
            View::Scalar { kind, repr }
                if *repr == witness.text
                    && matches!(
                        *kind,
                        "u8" | "u16" | "u32" | "u64" | "i8" | "i16" | "i32" | "i64"
                    )
        ),
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
/// a `Determiner::Target` — which is what `target <Param(0)>` contributes —
/// stops it. That single distinction is what separates a
/// [`HoleClass::Subtree`] hole from a [`HoleClass::FieldSlice`] one without
/// any per-category table.
fn witness_only(node: &View, witness: &Witness) -> bool {
    if is_marker(node, witness) {
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
    if witness.kind == WitnessKind::Numeral {
        return Ok((HoleClass::Numeral, site));
    }
    if parent_name == Some("NominalPhrase") && last == Some(PathStep::Field("head")) {
        // Spec D7: the determiner belongs to the frame and the hole is the
        // determinerless nominal core. Anything the frame put in the other
        // two fields would have to be merged with the filler's, which the
        // pilot does not do — so say so rather than compile something whose
        // meaning is not defined.
        let claimed = ["modifiers", "head", "complements"];
        for field in ["modifiers", "complements"] {
            let empty = parent
                .then(PathStep::Field(field))
                .resolve(tree)
                .is_some_and(View::is_vacuous);
            anyhow::ensure!(
                empty,
                "puts frame material in a nominal's `{field}` beside the hole at {site}; \
                 a field-slice hole must claim {claimed:?} whole"
            );
        }
        return Ok((
            HoleClass::FieldSlice {
                claimed: claimed.to_vec(),
            },
            parent,
        ));
    }
    Ok((HoleClass::Subtree, site))
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
fn normalize_all(tree: &mut View, agreement: &mut [AgreementDep]) {
    for index in 0..agreement.len() {
        let site = agreement[index].site.clone();
        let kind = agreement[index].kind;
        let (applied, removal) = normalize_citation(tree, &site, kind);
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
) -> (Vec<Normalization>, Option<Removal>) {
    let mut applied = Vec::new();
    let mut removal = None;
    let Some(site) = at.resolve_mut(tree) else {
        return (applied, None);
    };
    match kind {
        AgreeKind::VerbWithHole(_) => {
            let View::Node { fields, .. } = site else {
                return (applied, None);
            };
            let Some((_, View::Node { fields: slot, .. })) =
                fields.iter_mut().find(|(name, _)| *name == "slot")
            else {
                return (applied, None);
            };
            let mut was = ("Third", "Singular");
            let mut changed = false;
            for (name, citation, keep) in [
                ("person", "Third", &mut was.0),
                ("number", "Singular", &mut was.1),
            ] {
                let Some((_, value)) = slot.iter_mut().find(|(field, _)| *field == name) else {
                    continue;
                };
                let View::Unit { variant, .. } = value else {
                    continue;
                };
                if let Some(current) = *variant {
                    *keep = current;
                    if current != citation {
                        changed = true;
                    }
                }
                *variant = Some(citation);
            }
            if changed {
                applied.push(Normalization::VerbAgreement {
                    person: was.0,
                    number: was.1,
                });
            }
        }
        AgreeKind::NounNumberFromHole(hole) => {
            let View::Node { fields, .. } = site else {
                return (applied, None);
            };
            if let Some((moved, at_index)) = take_quantity_modifier(fields, hole) {
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
                && *variant == "Plural"
            {
                *variant = "Singular";
                applied.push(Normalization::NounNumber { from: "Plural" });
            }
        }
    }
    (applied, removal)
}

/// Takes the sole `NominalModifier::Quantity` bearing hole `hole` out of a
/// nominal's `modifiers`, if its `determiner` is empty, and returns it
/// re-wrapped as a `Determiner::Quantity`, with the index it came from.
fn take_quantity_modifier(
    fields: &mut [(&'static str, View)],
    hole: usize,
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
            && item
                .walk()
                .iter()
                .any(|(_, node)| matches!(node, View::Hole { index, .. } if *index == hole))
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

        // And the difference is recorded rather than silently discarded: the
        // singular authoring needed both rewrites, the plural one only the
        // head's number.
        let kinds: Vec<AgreeKind> = a.agreement.iter().map(|dep| dep.kind).collect();
        assert!(kinds.contains(&AgreeKind::VerbWithHole(0)), "{kinds:?}");
        assert!(
            kinds.contains(&AgreeKind::NounNumberFromHole(1)),
            "{kinds:?}"
        );
        let normalizations: Vec<&Normalization> =
            a.agreement.iter().flat_map(|dep| &dep.normalized).collect();
        assert!(
            normalizations.contains(&&Normalization::QuantityToDeterminer),
            "{normalizations:?}"
        );
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
        assert_eq!(guarded.guards[0].source, "You", "stored as authored");
        assert_eq!(
            guarded.guards[0].value,
            guard::normalized(deckmaste_core::Reference::You),
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
    /// `deckmaste_cards`'s job, and this crate sits below it.
    fn reader() -> MacroSet {
        let mut macros = MacroSet::new(deckmaste_core::ron::kinds())
            .with_options(deckmaste_core::ron::raw_options());
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
            "but the authored spelling is kept"
        );
        assert_eq!(
            sugar.value,
            guard::normalized(deckmaste_core::Quantity::one()),
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

        normalize_all(&mut tree, &mut agreement);

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
                .contains(&Normalization::NounNumber { from: "Plural" }),
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

    /// The authorable half of the same hazard: a second modifier really does
    /// sit after the count in `41 <Param> card`, so the removal really does
    /// renumber a live sibling — and the hole that moved must still be found
    /// at its recorded path.
    #[test]
    fn a_second_modifier_survives_the_count_being_lifted() {
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
        assert!(
            singular.agreement.iter().any(|dep| dep
                .normalized
                .contains(&Normalization::QuantityToDeterminer)),
            "the singular authoring should have lifted its count: {:?}",
            singular.agreement
        );
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

    /// The field-slice guard: a frame may claim the determiner, but if it
    /// also puts material in a nominal's `modifiers` or `complements` the
    /// hole is no longer the whole determinerless core, and the merge that
    /// would need is not defined. Both fields are checked, both are
    /// authorable, and both are refused by name rather than mis-compiled.
    #[test]
    fn frame_material_beside_a_field_slice_hole_is_refused() {
        for (text, field) in [
            ("target white <Param(0)>", "modifiers"),
            ("target <Param(0)> you control", "complements"),
        ] {
            let error = compile(
                &bare(text),
                FragmentKind::Nominal,
                &["Predicate".to_string()],
                &Catalogs::default(),
                &reader(),
            )
            .unwrap_err();
            let message = format!("{error:#}");
            assert!(
                message.contains(field),
                "{text:?} should name `{field}`: {message}"
            );
            assert!(message.contains(text), "and name the frame: {message}");
        }
    }

    /// The frame catalog's own seeded entries have to compile, or Task 6 has
    /// nothing to author against.
    #[test]
    fn the_seeded_constructor_catalog_compiles() {
        let dir =
            std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../plugins/builtin/frames");
        let catalog = macro_ron::frames::load_constructor_frames(&dir).unwrap();
        assert!(!catalog.is_empty());
        let macros = reader();
        for entry in &catalog {
            for spec in &entry.frames {
                // `Target` is a noun phrase; the two effect constructors are
                // sentences. (Task 5's catalog schema will carry the category
                // rather than inferring it; the pilot has three entries.)
                let kind = if entry.constructor == "Target" {
                    FragmentKind::Nominal
                } else {
                    FragmentKind::Sentence
                };
                let frame = compile(spec, kind, &entry.params, &Catalogs::default(), &macros)
                    .unwrap_or_else(|error| panic!("{}: {error:#}", entry.constructor));
                assert!(
                    !frame.holes.is_empty(),
                    "{} compiled with no holes",
                    entry.constructor
                );
                assert_sites_resolve(&frame);
            }
        }
    }
}
