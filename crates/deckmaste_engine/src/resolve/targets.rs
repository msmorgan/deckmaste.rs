//! Target-set legality: spec bounds, distinctness, and resolution-time
//! re-checks ([CR#608.2b]).

use deckmaste_core::Count;
use deckmaste_core::TargetSpec;
use deckmaste_core::Uint;

use crate::object::ObjectId;
use crate::stack::StackEntry;
use crate::stack::StackObject;
use crate::state::GameState;

impl GameState {
    /// [CR#608.2b]: for each chosen target, it still matches its `TargetSpec`'s
    /// filter. Returns `true` if all chosen targets are still legal (or there
    /// are no targets). Stage 2: single target, so "all legal" == "the one
    /// target legal". A spell's specs derive from its `Spell` ability; an
    /// activated ability's ride the carried text ([CR#602.2b]).
    ///
    /// **Partial fizzle** ([CR#608.2b]): the entry fizzles iff EVERY target
    /// across all slots is now illegal; a single surviving legal target keeps
    /// it resolving (the illegal targets are then excluded from the reads).
    /// Counts are NOT re-checked — they lock at announce ([CR#601.2c]).
    /// Distinct is re-asserted on the final set as defense in depth
    /// ([CR#115.7e]); retarget validation should never let a violation reach
    /// here.
    #[must_use]
    pub(crate) fn targets_still_legal(&self, entry: &StackEntry) -> bool {
        let specs: Vec<TargetSpec> = match &entry.object {
            StackObject::Spell(o) => {
                let effect = self
                    .spell_effect(*o)
                    .expect("a spell stack entry has a Spell ability");
                crate::cast::announced_target_specs(
                    &effect,
                    &self.spell_targets(*o),
                    entry.chosen_modes.as_ref(),
                )
            }
            // The carried text is authoritative — never re-derive from the
            // (possibly gone, possibly changed) source.
            StackObject::Activated { ability, .. } => crate::cast::announced_target_specs(
                &ability.effect,
                &ability.targets,
                entry.chosen_modes.as_ref(),
            ),
            StackObject::Triggered {
                source,
                ability,
                created,
                ..
            } => {
                if let Some(t) = created {
                    // The delayed/reflexive body is authoritative
                    // ([CR#603.7,603.12]).
                    t.targets.to_vec()
                } else {
                    let abilities = crate::derive::abilities_of_source(self, *source);
                    let t = abilities[*ability].as_triggered().expect("trigger index");
                    t.targets.to_vec()
                }
            }
        };
        debug_assert_eq!(
            specs.len(),
            entry.targets.len(),
            "announce fills one chosen SET per TargetSpec slot",
        );
        // [CR#608.2b] re-checks the same Cant(Target) rows the announce
        // evaluated — a hexproof granted after announce fizzles the spell.
        let view = self.layers();
        let rows = crate::legal::cant_target_rows(self, &view);
        // The carrier is the stack object's source — for a trigger the stack id
        // is minted with the ability's `source`, so this anchors `Ref(This)` /
        // `StatOf(This, …)` in a target filter back to the source object, the
        // same carrier the announce path used (Mentor's lesser-power recheck,
        // [CR#702.134a,608.2b]).
        let carrier = Some(self.objects.obj(entry.id).source);
        // Whole-set scan: every target across every slot is judged; the entry
        // survives iff at least one is still legal (partial fizzle,
        // [CR#608.2b]). An entry that chose zero targets (all min-0 slots) has
        // no target to be illegal, so it resolves ([CR#608.2b] fizzles only
        // when targets exist and are ALL illegal).
        let mut any_target = false;
        let mut any_legal = false;
        for (spec, slot) in specs.iter().zip(&entry.targets) {
            let predicate = target_spec_predicate(spec);
            for &chosen in slot {
                any_target = true;
                // A target that no longer exists (reminted on zone change) is
                // trivially illegal — the filter can't be satisfied.
                let legal = self.objects.get(chosen).is_some()
                    && crate::target::matches_region_with_activation(
                        self,
                        chosen,
                        predicate,
                        carrier,
                        entry.activation,
                    )
                    && crate::legal::target_forbidden_by(self, &rows, entry.id, chosen).is_none();
                any_legal |= legal;
            }
        }
        (!any_target || any_legal) && target_set_distinct_ok(&specs, &entry.targets)
    }
}

/// Extracts the membership `Predicate` from a `TargetSpec` — the per-object
/// legality filter, unconcerned with count or co-target distinctness. Peels the
/// co-target [`TargetSpec::Distinct`] wrapper ([CR#115.7e]) to its inner
/// `Target`, exactly as the render-side twin (`fragment::target_spec_filter`)
/// does: distinctness is a SET constraint enforced at the three set-level sites
/// ([`validate_target_set`] on announce/retarget, [`Self::targets_still_legal`]
/// on the [CR#608.2b] re-check), never a per-object filter.
///
/// This is the single authoritative site for TargetSpec→Predicate extraction;
/// both `cast::legal_targets` (announce time) and `targets_still_legal`
/// (resolution time) funnel through here so they stay in sync.
pub(crate) fn target_spec_predicate(
    spec: &TargetSpec,
) -> &deckmaste_core::Region<deckmaste_core::Predicate> {
    match spec {
        TargetSpec::Target(_quantity, f) => f,
        // Distinctness lives in the SET checks, not the filter: peel to the
        // inner `Target`'s predicate ([CR#115.7e]).
        TargetSpec::Distinct(_, inner) => target_spec_predicate(inner),
        // Provenance is erased at `lower` (`deckmaste_lowering`), so no
        // loaded value reaches here wrapped. The arm survives only because
        // the variant does; `core-demacro` deletes both.
    }
}

/// The [`Quantity`](deckmaste_core::Quantity) governing a slot's target count
/// ([CR#601.2c]), peeling `Distinct`/`Expanded` to the leaf `Target` — the
/// count twin of [`target_spec_predicate`].
pub(crate) fn target_spec_quantity(spec: &TargetSpec) -> &deckmaste_core::Quantity {
    match spec {
        TargetSpec::Target(q, _) => q,
        TargetSpec::Distinct(_, inner) => target_spec_quantity(inner),
        // Provenance is erased at `lower` (`deckmaste_lowering`), so no
        // loaded value reaches here wrapped. The arm survives only because
        // the variant does; `core-demacro` deletes both.
    }
}

/// The sibling slot indices a [`TargetSpec::Distinct`] slot must stay disjoint
/// from ([CR#115.7e], Arc Trail's "any *other* target") — `&[]` for a plain
/// `Target`. Peels `Expanded`; the outermost `Distinct` is authoritative (a
/// nested `Distinct` is not a semantic shape).
pub(crate) fn distinct_siblings(spec: &TargetSpec) -> &[usize] {
    match spec {
        TargetSpec::Distinct(siblings, _) => siblings,
        TargetSpec::Target(..) => &[],
        // Provenance is erased at `lower` (`deckmaste_lowering`), so no
        // loaded value reaches here wrapped. The arm survives only because
        // the variant does; `core-demacro` deletes both.
    }
}

/// A LITERAL target-count bound the count check can evaluate frameless — the
/// count twin of the frameless `target::const_count`. A dynamic bound (`X`,
/// `CountOf`, …) would need a carrier `ExecutionFrame` the set-level sites don't hold.
///
/// AUDITED invariant (engine-candidate-frame-context), not a guess: every
/// `TargetSpec` `Quantity` bound across the corpus (canon, builtin, demo,
/// testing, and the generated wizards set) is a literal `Exactly`/`AtLeast`/
/// `AtMost`/`Between` count — no card spells "target X creatures" or a
/// `CountOf`/`StatOf`-bounded target quantity. `debug_assert!` (not `todo!`)
/// so a corpus regression trips loudly in tests while a release build
/// degrades to a `0` bound (an empty/immediately-satisfied range) instead of
/// crashing a game.
fn const_target_count(count: &Count) -> Uint {
    match count {
        Count::Literal(n) => *n,
        other => {
            debug_assert!(
                false,
                "engine invariant violated: dynamic target-count bound {other:?} — no corpus \
                 TargetSpec uses one; a carrier ExecutionFrame would be needed to evaluate it"
            );
            0
        }
    }
}

/// The `(min, max)` object count a target slot admits ([CR#601.2c]); `max =
/// None` is unbounded ("any number"). `min` is 0 for an unbounded-below slot
/// ("up to N" / "any number"). Literal bounds only (see
/// [`const_target_count`]).
pub(crate) fn slot_count_bounds(spec: &TargetSpec) -> (Uint, Option<Uint>) {
    let (lo, hi) = target_spec_quantity(spec).bounds();
    (lo.map_or(0, const_target_count), hi.map(const_target_count))
}

/// How many leading target slots must already be ANNOUNCED before every slot
/// can be enumerated ([CR#601.2c]) — 0 when no slot's filter reads another's
/// announced register, which is every card in the corpus today.
///
/// Core's telescope law (`deckmaste_core`'s `validate_telescope`) lets slot
/// `k`'s filter region declare `Provenance::AnnouncedTarget(0..k)` parameters
/// and nothing later, and a region declares exactly what it reads, so the
/// count of those parameters IS the number of earlier slots that slot reads.
/// The answer is the largest such count over all slots.
pub(crate) fn announced_prefix_len(specs: &[TargetSpec]) -> usize {
    specs
        .iter()
        .map(|spec| {
            target_spec_predicate(spec)
                .params
                .iter()
                .filter(|param| {
                    matches!(
                        param.provenance,
                        deckmaste_core::Provenance::AnnouncedTarget(_)
                    )
                })
                .count()
        })
        .max()
        .unwrap_or(0)
}

/// The cross-slot distinctness portion of the set check ([CR#115.7e]): every
/// [`TargetSpec::Distinct`] slot's chosen set is disjoint from the union of its
/// named sibling slots' sets. Malformed semantic input — a sibling index out of
/// range or naming the slot itself — is treated as a violation (`false`), so
/// the spec is unsatisfiable and fizzles rather than panicking (Idris proves
/// `distinctOk` statically; Rust catches it at runtime). Shared by
/// [`validate_target_set`] and the resolution re-check so both read the same
/// rule. Departed targets can only SHRINK a set, never create an overlap, so a
/// set valid at announce stays valid here.
pub(crate) fn target_set_distinct_ok(specs: &[TargetSpec], chosen: &[Vec<ObjectId>]) -> bool {
    for (i, spec) in specs.iter().enumerate() {
        for &sib in distinct_siblings(spec) {
            if sib == i || sib >= chosen.len() {
                return false;
            }
            if chosen[i].iter().any(|t| chosen[sib].contains(t)) {
                return false;
            }
        }
    }
    true
}

/// Validate a submitted per-slot target set against its specs
/// ([CR#601.2c,115.7e]) — the SHAPE checks that need no legal candidate sets
/// (per-slot membership is checked at the decision handler, which holds
/// `legal`; per-object legality at resolution): (a) one inner set per spec;
/// (b) each slot's count within its `Quantity` bounds — count-locking is
/// inherent, one submission carries the counts; (c) within-slot distinctness —
/// the same object can't be chosen twice for one "target" ([CR#601.2c]); (d)
/// each `Distinct` slot disjoint from its siblings ([CR#115.7e]). The same id
/// ACROSS unrelated slots stays legal (the artifact-land example). `Err`
/// carries the rejection reason; semantic-input errors (inverted/zero bounds,
/// bad sibling index) reject rather than panic.
pub(crate) fn validate_target_set(
    specs: &[TargetSpec],
    chosen: &[Vec<ObjectId>],
) -> Result<(), String> {
    if chosen.len() != specs.len() {
        return Err(format!(
            "target set has {} slots, the ability has {}",
            chosen.len(),
            specs.len()
        ));
    }
    for (i, (spec, slot)) in specs.iter().zip(chosen).enumerate() {
        let (min, max) = slot_count_bounds(spec);
        let count = Uint::try_from(slot.len()).expect("slot size fits Uint");
        if count < min {
            return Err(format!(
                "slot {i}: chose {count} target(s), minimum is {min}"
            ));
        }
        if let Some(max) = max
            && count > max
        {
            return Err(format!(
                "slot {i}: chose {count} target(s), maximum is {max}"
            ));
        }
        // [CR#601.2c]: no object twice within one "target" instance.
        for (j, &t) in slot.iter().enumerate() {
            if slot[..j].contains(&t) {
                return Err(format!("slot {i}: object chosen twice for one target"));
            }
        }
    }
    if !target_set_distinct_ok(specs, chosen) {
        return Err("a Distinct slot overlaps a sibling slot (or names a bad index)".into());
    }
    Ok(())
}

/// Whether an announce over `specs` can be completed given each slot's legal
/// candidate set `legal` ([CR#601.2c,602.2b,603.3c]) — the
/// castable/activatable/trigger-drop gate. Each slot must offer at least its
/// MINIMUM count of candidates, and the `Distinct` slots must admit a system of
/// DISTINCT representatives ([CR#115.7e]). A min-0 slot ("up to N" / "any
/// number") is satisfiable even with no candidates (choose zero).
///
/// The distinctness feasibility is EXACT for the semantic shape — quantity-one
/// slots linked by `Distinct` (Fate Transfer's two "target creature" slots) —
/// via bounded backtracking over just the distinct-linked quantity-one slots.
/// Their number is a small semantic constant, so the search is trivial. A
/// `Distinct` slot whose count is not exactly one is the unbuilt general case
/// (no canon card): it is skipped from the representative search and gated only
/// by its per-slot minimum — permissive rather than a panic or a false reject.
/// A malformed sibling index makes the spec unsatisfiable (uncastable).
#[must_use]
pub(crate) fn announce_satisfiable(specs: &[TargetSpec], legal: &[Vec<ObjectId>]) -> bool {
    if specs.len() != legal.len() {
        return false;
    }
    for (i, spec) in specs.iter().enumerate() {
        let (min, max) = slot_count_bounds(spec);
        // Inverted or zero-with-a-minimum bounds admit no legal count.
        if max.is_some_and(|hi| hi < min) {
            return false;
        }
        if legal[i].len() < usize::try_from(min).expect("min count fits usize") {
            return false;
        }
        // A malformed Distinct sibling index is ill-formed → unsatisfiable.
        for &sib in distinct_siblings(spec) {
            if sib == i || sib >= specs.len() {
                return false;
            }
        }
    }
    // Backtracking system-of-distinct-representatives over the quantity-one
    // slots that participate in any Distinct edge (as a Distinct spec or as a
    // named sibling). Edges to non-quantity-one slots are dropped (the unbuilt
    // general case), so the search covers exactly the semantic pairwise shape.
    let is_one = |i: usize| slot_count_bounds(&specs[i]) == (1, Some(1));
    let mut involved: Vec<usize> = Vec::new();
    let mut edges: Vec<(usize, usize)> = Vec::new();
    for (i, spec) in specs.iter().enumerate() {
        for &sib in distinct_siblings(spec) {
            if is_one(i) && is_one(sib) {
                edges.push((i, sib));
                for s in [i, sib] {
                    if !involved.contains(&s) {
                        involved.push(s);
                    }
                }
            }
        }
    }
    distinct_reps_exist(&involved, &edges, legal, &mut Vec::new())
}

/// Backtrack a distinct representative for each slot in `involved`, one legal
/// candidate apiece, honoring the `Distinct` `edges` (assigned reps of edge-
/// linked slots must differ). `assigned` accumulates `(slot, rep)`. Exponential
/// in `involved.len()`, which is a tiny semantic constant.
fn distinct_reps_exist(
    involved: &[usize],
    edges: &[(usize, usize)],
    legal: &[Vec<ObjectId>],
    assigned: &mut Vec<(usize, ObjectId)>,
) -> bool {
    let Some(&slot) = involved.get(assigned.len()) else {
        return true; // every involved slot has a representative
    };
    for &cand in &legal[slot] {
        let clashes = assigned.iter().any(|&(other, rep)| {
            rep == cand
                && edges
                    .iter()
                    .any(|&(a, b)| (a == slot && b == other) || (a == other && b == slot))
        });
        if clashes {
            continue;
        }
        assigned.push((slot, cand));
        if distinct_reps_exist(involved, edges, legal, assigned) {
            return true;
        }
        assigned.pop();
    }
    false
}

/// Pure set-level targeting checks ([CR#601.2c,115.7e]) — the announce/retarget
/// validator, the castable/activatable satisfiability gate, and the
/// `TargetSpec` peels. Opaque fabricated ids (no live store needed).
#[cfg(test)]
mod target_set_tests {
    use std::sync::Arc;

    use deckmaste_core::Count;
    use deckmaste_core::Predicate;
    use deckmaste_core::Quantity;
    use deckmaste_core::TargetSpec;
    use deckmaste_core::Type;

    use super::announce_satisfiable;
    use super::distinct_siblings;
    use super::slot_count_bounds;
    use super::target_spec_predicate;
    use super::validate_target_set;
    use crate::object::ObjectId;

    fn creature() -> Predicate {
        Predicate::r#type(Type::Creature)
    }

    fn t_one() -> TargetSpec {
        TargetSpec::Target(
            Quantity::one(),
            Arc::new(deckmaste_core::Region::candidate(creature())),
        )
    }

    fn t_range(lo: Option<u32>, hi: Option<u32>) -> TargetSpec {
        TargetSpec::Target(
            Quantity::Range(lo.map(Count::Literal), hi.map(Count::Literal)),
            Arc::new(deckmaste_core::Region::candidate(creature())),
        )
    }

    fn distinct(siblings: Vec<usize>, inner: TargetSpec) -> TargetSpec {
        TargetSpec::Distinct(siblings.into(), Arc::new(inner))
    }

    fn id(n: u64) -> ObjectId {
        ObjectId::from_raw(n)
    }

    /// `target_spec_predicate` peels the `Distinct` wrapper to the inner
    /// `Target`'s predicate — no panic (that seam is closed).
    #[test]
    fn filter_peels_distinct_to_the_inner_predicate() {
        let spec = distinct(vec![0], t_one());
        assert_eq!(&target_spec_predicate(&spec).body, &creature());
        assert_eq!(distinct_siblings(&spec), &[0]);
        assert_eq!(distinct_siblings(&t_one()), &[] as &[usize]);
    }

    /// `slot_count_bounds` reads the leaf `Quantity`, seeing through
    /// `Distinct`.
    #[test]
    fn slot_bounds_read_literal_quantities() {
        assert_eq!(slot_count_bounds(&t_one()), (1, Some(1)));
        assert_eq!(slot_count_bounds(&t_range(Some(1), Some(3))), (1, Some(3)));
        assert_eq!(slot_count_bounds(&t_range(None, Some(2))), (0, Some(2)));
        assert_eq!(slot_count_bounds(&t_range(None, None)), (0, None));
        // Peeled through Distinct.
        assert_eq!(slot_count_bounds(&distinct(vec![0], t_one())), (1, Some(1)));
    }

    /// [CR#601.2c]: the same object twice within ONE "target" is rejected.
    #[test]
    fn within_slot_duplicate_rejected() {
        let specs = [t_range(Some(1), Some(3))];
        let err = validate_target_set(&specs, &[vec![id(1), id(1)]]).unwrap_err();
        assert!(
            err.contains("object chosen twice"),
            "unexpected error: {err}"
        );
        assert_eq!(validate_target_set(&specs, &[vec![id(1), id(2)]]), Ok(()));
    }

    /// The same id across two UNRELATED specs stays legal (the artifact-land
    /// example) — only a `Distinct` slot must be disjoint.
    #[test]
    fn same_id_across_separate_specs_accepted() {
        let specs = [t_one(), t_one()];
        assert_eq!(
            validate_target_set(&specs, &[vec![id(1)], vec![id(1)]]),
            Ok(())
        );
    }

    /// [CR#601.2c]: counts outside the `Between(1, 3)` bounds are rejected (0
    /// and 4), inside accepted.
    #[test]
    fn count_bounds_enforced() {
        let specs = [t_range(Some(1), Some(3))];
        let err = validate_target_set(&specs, &[vec![]]).unwrap_err();
        assert_eq!(err, "slot 0: chose 0 target(s), minimum is 1");
        let err = validate_target_set(&specs, &[vec![id(1), id(2), id(3), id(4)]]).unwrap_err();
        assert_eq!(err, "slot 0: chose 4 target(s), maximum is 3");
        assert_eq!(validate_target_set(&specs, &[vec![id(1), id(2)]]), Ok(()));
    }

    /// [CR#115.7e]: a `Distinct` slot overlapping its sibling is rejected; a
    /// disjoint pair (the Fate Transfer shape) is accepted.
    #[test]
    fn distinct_overlap_rejected_disjoint_accepted() {
        let specs = [t_one(), distinct(vec![0], t_one())];
        let err = validate_target_set(&specs, &[vec![id(1)], vec![id(1)]]).unwrap_err();
        assert!(
            err.contains("a Distinct slot overlaps a sibling slot"),
            "unexpected error: {err}"
        );
        assert_eq!(
            validate_target_set(&specs, &[vec![id(1)], vec![id(2)]]),
            Ok(())
        );
    }

    /// Malformed semantic input — a sibling index out of range or naming the
    /// slot itself — is a rejected set, never a panic (Idris proves
    /// `distinctOk`; Rust catches it at runtime).
    #[test]
    fn malformed_sibling_index_rejected_not_panicking() {
        let out_of_range = [t_one(), distinct(vec![5], t_one())];
        let err = validate_target_set(&out_of_range, &[vec![id(1)], vec![id(2)]]).unwrap_err();
        assert!(
            err.contains("a Distinct slot overlaps a sibling slot"),
            "unexpected error: {err}"
        );
        let self_ref = [t_one(), distinct(vec![1], t_one())];
        let err = validate_target_set(&self_ref, &[vec![id(1)], vec![id(2)]]).unwrap_err();
        assert!(
            err.contains("a Distinct slot overlaps a sibling slot"),
            "unexpected error: {err}"
        );
    }

    /// The castability gate: a single quantity-one slot needs ≥1 candidate; a
    /// min-0 slot ("up to N") is satisfiable with none.
    #[test]
    fn satisfiable_respects_minimum_counts() {
        assert!(announce_satisfiable(&[t_one()], &[vec![id(1)]]));
        assert!(!announce_satisfiable(&[t_one()], &[vec![]]));
        assert!(announce_satisfiable(&[t_range(None, Some(2))], &[vec![]]));
    }

    /// [CR#115.7e]: a `Distinct` pair is satisfiable only when the two slots
    /// admit DISTINCT representatives — two shared-only candidates that
    /// collapse to one object are NOT castable.
    #[test]
    fn satisfiable_needs_distinct_representatives() {
        let specs = [t_one(), distinct(vec![0], t_one())];
        assert!(
            announce_satisfiable(&specs, &[vec![id(1), id(2)], vec![id(1), id(2)]]),
            "two distinct creatures → castable"
        );
        assert!(
            !announce_satisfiable(&specs, &[vec![id(1)], vec![id(1)]]),
            "only one creature, shared by both slots → uncastable"
        );
    }

    /// An out-of-range sibling index makes the announce unsatisfiable (the
    /// spell is uncastable), never a panic.
    #[test]
    fn satisfiable_false_on_malformed_sibling() {
        let specs = [t_one(), distinct(vec![9], t_one())];
        assert!(!announce_satisfiable(&specs, &[vec![id(1)], vec![id(2)]]));
    }
}
