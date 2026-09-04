//! Reference/selection evaluation: resolve `Reference`s and `Selection`s to
//! object sets against a frame.

use deckmaste_core::Count;
use deckmaste_core::Reference;
use deckmaste_core::Selection;
use deckmaste_core::Uint;
use slotmap::Key;

use crate::object::ObjectId;
use crate::object::ObjectSource;
use crate::stack::ExecutionFrame;
use crate::state::GameState;

impl GameState {
    /// Resolves the agent of a `By(who, …)` to the acting `PlayerId`. `who` is
    /// a [`Reference`] that resolves (via `eval_reference`) to a player proxy
    /// object; this maps that proxy back to its `PlayerId`.
    ///
    /// This is also where every DECIDER slot (`May.who`,
    /// `ChooseSpec.chooser`, …) is resolved to a concrete player
    /// ([CR#608.2d]). Every such `Reference` is evaluated SYNCHRONOUSLY over
    /// state+frame, so a QUANTIFIED decider — "an opponent [of your choice]
    /// chooses", requiring its own sub-choice of WHICH opponent before the
    /// named decision can even open — has no home here today; zero canon
    /// witnesses. The settled design is a binder node: the quantifying
    /// choice hoists to a preceding binder that binds an `Ident`, and the
    /// decider slot reads the resulting `Reference::Bound(ident)` like any
    /// other bound reference. A suspending `Reference::ChosenBy` variant was
    /// considered and REJECTED for the same reason — every `Reference` eval
    /// here is synchronous. The first witness implements the binder node,
    /// not this function.
    ///
    /// # Panics
    ///
    /// Panics if `who` resolves to a non-player object — a player verb's agent
    /// must be a player ([CR#608.2]).
    pub(crate) fn acting_player(
        &self,
        who: &Reference,
        frame: &ExecutionFrame,
    ) -> crate::player::PlayerId {
        let object = self.eval_reference(who, frame);
        match self.objects.get(object).map(|o| o.source) {
            Some(ObjectSource::Player(p)) => p,
            other => panic!("a player verb's agent must be a player, got {other:?}"),
        }
    }

    /// Resolve a `Reference` to the `PlayerId` of the player proxy it names, or
    /// `None` when it names a non-player object — the graceful (never-crash)
    /// twin of [`acting_player`](Self::acting_player), used by the player-side
    /// `Count` reads
    /// ([`Count::PlayerStatOf`](deckmaste_core::Count::PlayerStatOf)
    /// / [`Count::Opponents`](deckmaste_core::Count::Opponents)), where a
    /// non-player reference is a semantic-input error that fizzles to 0.
    pub(crate) fn eval_player_ref(
        &self,
        reference: &Reference,
        frame: &ExecutionFrame,
    ) -> Option<crate::player::PlayerId> {
        let id = self.eval_reference(reference, frame);
        match self.objects.get(id).map(|o| o.source) {
            Some(ObjectSource::Player(p)) => Some(p),
            Some(ObjectSource::Card(_)) | None => None,
        }
    }

    /// (min, max) objects to choose for `quantity`, clamped to `n` available —
    /// choose as many as able when fewer exist ([CR#608.2d]). The test-only
    /// `can_pay_verbs` helper also uses it to read a selection's required
    /// floor.
    pub(crate) fn choice_bounds(
        &self,
        quantity: &deckmaste_core::Quantity,
        n: usize,
        frame: &ExecutionFrame,
    ) -> (Uint, Uint) {
        let cap = Uint::try_from(n).expect("candidate count fits Uint");
        let ev = |c: &Count| self.eval_count(c, frame).min(cap);
        // `Quantity` collapsed to one `Range(lo, hi)` primitive (seen through a
        // remembered `Exactly`/`AtMost`/… macro by `bounds`): an absent lower
        // bound floors at 0, an absent upper bound caps at the candidate count
        // ([CR#608.2d] — choose as many as able).
        let (lo, hi) = quantity.bounds();
        (lo.map_or(0, ev), hi.map_or(cap, ev))
    }

    /// The nth announced target register's live members, in announce order.
    /// Departed (illegal) targets are
    /// excluded ([CR#608.2b] partial fizzle; mirrors `StatePredicate::Targets`
    /// ignoring a gone target), so a wholly-departed slot reads empty and its
    /// verb no-ops. An out-of-range index reads empty (never-crash).
    fn live_target_slot(&self, frame: &ExecutionFrame, n: usize) -> Vec<ObjectId> {
        self.activation_target_objects(frame.activation, n)
            .into_iter()
            .filter(|&target| self.objects.get(target).is_some())
            .collect()
    }

    /// [CR#707.10d]'s could-target set for the stack entry `spell` names —
    /// [`Selection::ValidTargetsFor`]'s whole computation, extracted so the
    /// selection dispatch stays a flat one-line-per-arm match.
    ///
    /// The entry's per-slot legal sets INTERSECTED (the same-object rule),
    /// keeping the first slot's candidate order. Reads the COMMITTED entry's
    /// stored specs — never re-derived from a possibly-changed source — through
    /// the same `Cant(Target)` + `AsThough` layering announce-time targeting
    /// uses, so hexproof and protection are honored identically here.
    ///
    /// `None` when the reference names no live stack entry (the caller
    /// fizzles). An entry with no target slots yields the EMPTY set: it could
    /// target nothing.
    fn could_target_set(&self, spell: &Reference, frame: &ExecutionFrame) -> Option<Vec<ObjectId>> {
        let id = self.eval_reference(spell, frame);
        let entry = self.stack.iter().find(|e| e.id == id)?;
        let view = self.layers();
        let specs =
            self.stack_object_target_specs(&view, &entry.object, entry.chosen_modes.as_ref());
        let per_slot = self.legal_targets_for_specs(&specs, entry.id, entry.activation);
        let Some((first, rest)) = per_slot.split_first() else {
            return Some(Vec::new());
        };
        let mut acc = first.clone();
        for slot in rest {
            acc.retain(|c| slot.contains(c));
        }
        Some(acc)
    }

    /// A selection (a GROUP) resolved to its full set ([CR#608.2d]) — the
    /// home of plurality now that verbs take a single [`Reference`].
    /// `Predicate` enumerates the matching set;
    /// `They`/`TheGroup`/`TopOfLibrary` name an already-bound group. A
    /// per-object instruction runs over this set via an enclosing `Each`/
    /// `Distribute`/`With`, never the verb itself.
    pub(crate) fn eval_selection_set(
        &self,
        sel: &Selection,
        frame: &ExecutionFrame,
    ) -> Vec<ObjectId> {
        match sel {
            Selection::Reg(reference) => {
                let values = self.activation_objects(frame.activation, *reference);
                if values.is_empty() && reference.0 >= 6 {
                    self.live_target_slot(frame, (reference.0 - 6) as usize)
                } else {
                    values
                }
            }
            // [CR#700.3b]: a pile register holds a labeled group whose members
            // are still individual objects, so iterating it yields those
            // members. Distinct from `Reg` only in the register shape it
            // reads; a pile is never a target slot, so there is no
            // announce-slot fallback.
            Selection::Pile(reference) => self.activation_objects(frame.activation, *reference),
            // Thread the carrier (like `Pick`) so a carrier-relative predicate
            // resolves rather than panicking frameless: "each opponent" =
            // `SelectAll(OpponentOf(Ref(You)))` reads `Ref(You)` off the watcher.
            Selection::SelectAll(f) => crate::target::candidates_region_with_activation(
                self,
                f,
                Some(self.frame_watcher(frame)),
                frame.activation,
            ),

            // [CR#107.1]: the extremal element(s) of a set, ranked by the
            // shared `Projection` ([`Count::Aggregate`]'s element-twin). Each
            // candidate is bound as the frame's iteration anaphor `It` so the
            // projection (`proj.by`) reads it via `Reference::It`; ties yield
            // the whole tied group (the usual single/choice path narrows
            // downstream). Mirrors `Count::Aggregate`'s candidate enumeration
            // exactly (`candidates_with` + the frame's watcher) rather than
            // the frameless `target::candidates`, so a `Ref(You)`-relative
            // `of` ("the creature YOU control with the greatest power")
            // resolves instead of panicking on a frameless position. `op` is
            // gated to the extremal ops; a non-extremal `op` fizzles to the
            // empty group.
            Selection::Pick { op, proj } => {
                use deckmaste_core::AggregateOp;
                use deckmaste_core::Countable;
                let candidates = match &proj.of {
                    Countable::Objects(filter) => {
                        let watcher = self.frame_watcher(frame);
                        crate::target::candidates_region_with_activation(
                            self,
                            filter,
                            Some(watcher),
                            frame.activation,
                        )
                    }
                    // Idris's `Pick` is pinned to `Projection b AnObject`
                    // ([CR#107.1] — no player-`Pick` consumer exists), so a
                    // `Players`-sourced `proj` here is semantics-invalid, like
                    // the non-`Projectable` `ManaSymbols`/`Singleton`/
                    // `ManaSpentMatching` sources — fizzle to the empty group
                    // (never-crash on a semantic-input error).
                    Countable::Players(..)
                    | Countable::ManaSymbols(..)
                    | Countable::Singleton(..)
                    | Countable::ManaSpentMatching(..) => Vec::new(),
                };
                let scored: Vec<(ObjectId, Uint)> = candidates
                    .into_iter()
                    .map(|id| {
                        let mut sub = frame.clone();
                        sub.activation = self.enter_candidate_region(&proj.by, frame, id);
                        (id, self.eval_count(&proj.by.body, &sub))
                    })
                    .collect();
                let extremum = match op {
                    AggregateOp::MaxOf => scored.iter().map(|(_, v)| *v).max(),
                    AggregateOp::MinOf => scored.iter().map(|(_, v)| *v).min(),
                    // A non-extremal op on Pick is malformed semantic input —
                    // fizzle to the empty group (never-crash).
                    AggregateOp::SumOf | AggregateOp::AverageOf(_) => None,
                };
                match extremum {
                    Some(target) => scored
                        .into_iter()
                        .filter(|(_, v)| *v == target)
                        .map(|(id, _)| id)
                        .collect(),
                    None => Vec::new(),
                }
            }
            // A random pick is a DECISION, so it is sampled by the mutating
            // resolver, never by this pure evaluator: `Each`/`Distribute`
            // sample it once through the seeded rng and write each element to
            // the loop region's parameter (the discard-at-random composite,
            // [CR#701.9b,608.2d]). The validator rejects a decision-bearing
            // selection in every pure position it could otherwise reach
            // (`Let`, [CR#608.2h]), so nothing reaches here — and an
            // unavailable group packs to empty rather than crashing.
            Selection::Random(..) => Vec::new(),
            // Provenance is erased at `lower` (`deckmaste_lowering`), so no
            // loaded value reaches here wrapped. The arm survives only because
            // the variant does; `core-demacro` deletes both.
            // [CR#707.10d]: every object the named spell could target — the
            // per-slot legal sets INTERSECTED (the same-object rule), so a
            // multi-slot spell yields only the objects legal in every slot at
            // once. Reads the COMMITTED entry's stored specs (never re-derived
            // from a possibly-changed source) through the same `Cant(Target)`
            // + `AsThough` layering that announce-time targeting uses, so
            // hexproof and protection are honored identically here.
            //
            // A reference naming no live stack entry fizzles to the empty
            // group; a spell with no target slots reads empty too (it could
            // target nothing).
            Selection::ValidTargetsFor(spell) => {
                self.could_target_set(spell, frame).unwrap_or_else(|| {
                    Self::unbound_group(sel, "ValidTargetsFor names no live stack entry")
                })
            }
            // The elements of the inner group, sequenced by `_by`'s choice
            // ([CR#707.10d]). The GROUP is exact; the chooser-driven SEQUENCE
            // is a recorded seam — no canon consumer demands it yet (the
            // for-each-could-target family this exists for is itself
            // unauthored), and the decision machinery is only reachable from a
            // binder that can surface a pending choice, which a bare selection
            // read has no channel for.
            //
            // Degrades to the inner group's natural order rather than to the
            // empty group: every ordering is a legal outcome of a free choice,
            // so an arbitrary one is a far weaker claim than "nothing
            // happens", and only the player-visible SEQUENCE is lost, never a
            // member. Wiring the choice replaces this arm's body, not its
            // shape.
            Selection::InChosenOrder(inner, _by) => self.eval_selection_set(inner, frame),
            // The top `count` cards of `of`'s library, front-to-back (top→down).
            // `of` resolves to a player via `eval_reference` → player proxy →
            // PlayerId. Supports `You` (controller's library) and `Opponent`
            // (the opponent's library, e.g. Fateseal [CR#701.29a]).
            Selection::TopOfLibrary { count, whose } => {
                let proxy = self.eval_reference(whose, frame);
                let pid = match self.objects.get(proxy).map(|o| o.source) {
                    Some(ObjectSource::Player(p)) => p,
                    other => {
                        panic!("TopOfLibrary.whose must resolve to a player proxy, got {other:?}")
                    }
                };
                let n = self.eval_count(count, frame) as usize;
                self.zones.libraries[pid.index()]
                    .iter()
                    .take(n)
                    .copied()
                    .collect()
            }
            // The bottom `count` cards of `whose`'s library, bottom→up — the
            // mirror of `TopOfLibrary` (the Idris `BottomOfLibrary`).
            Selection::BottomOfLibrary { count, whose } => {
                let proxy = self.eval_reference(whose, frame);
                let pid = match self.objects.get(proxy).map(|o| o.source) {
                    Some(ObjectSource::Player(p)) => p,
                    other => {
                        panic!(
                            "BottomOfLibrary.whose must resolve to a player proxy, got {other:?}"
                        )
                    }
                };
                let n = self.eval_count(count, frame) as usize;
                self.zones.libraries[pid.index()]
                    .iter()
                    .rev()
                    .take(n)
                    .copied()
                    .collect()
            }
            // The WHOLE of `whose`'s library, top→bottom — the un-sliced twin
            // of `TopOfLibrary`, and shuffle's own object ([CR#701.24a]).
            // Like `TopOfGraveyard` below (and unlike the panicking slice
            // family), a `whose` that fails to resolve to a player proxy
            // fizzles to the empty group: a semantic-input error must never
            // crash the engine.
            Selection::LibraryOf(whose) => {
                let proxy = self.eval_reference(whose, frame);
                match self.objects.get(proxy).map(|o| o.source) {
                    Some(ObjectSource::Player(p)) => {
                        self.zones.libraries[p.index()].iter().copied().collect()
                    }
                    _ => Vec::new(),
                }
            }
            // The top `count` cards of `of`'s graveyard, top→down
            // ([CR#404.2] — a graveyard is a single face-up pile in a fixed
            // order). `zones.graveyards` is push-appended as cards are put
            // into it, so the LAST-pushed card sits physically on top; a
            // front-to-back `Vec` read is bottom→up, so this reads from the
            // END, reversed. Unlike `TopOfLibrary`/`BottomOfLibrary`
            // (which panic on a non-player `of` — an established baseline
            // this arm does not repeat), an `of` that fails to resolve to a
            // player proxy fizzles to the empty group: a card-semantics
            // mistake must never crash the engine.
            Selection::TopOfGraveyard { count, of } => {
                let proxy = self.eval_reference(of, frame);
                match self.objects.get(proxy).map(|o| o.source) {
                    Some(ObjectSource::Player(p)) => {
                        let n = self.eval_count(count, frame) as usize;
                        self.zones.graveyards[p.index()]
                            .iter()
                            .rev()
                            .take(n)
                            .copied()
                            .collect()
                    }
                    _ => Vec::new(),
                }
            }
            // Several groups combined as ONE ([CR#608.2d], "each X and each
            // Y") — order-preserving concatenation; an object in more than
            // one member appears once (first position wins).
            Selection::Union(members) => {
                let mut seen = std::collections::HashSet::new();
                members
                    .iter()
                    .flat_map(|m| self.eval_selection_set(m, frame))
                    .filter(|id| seen.insert(*id))
                    .collect()
            } /* Grammar-valid but not yet wired: there is no `noted` object-set on
               * the frame to select among. An explicit named arm (not a catch-all)
               * so a future `Selection` variant is a compile error here rather than
               * a silent runtime panic.
               * [CR#607.2a]: the fact-backed product group — the members the
               * noting clause ACTUALLY moved, read through their post-move
               * identities ("cards milled this way"); a member that has since
               * left (a ceased token) drops out of the live read. */
        }
    }

    /// The object(s) a verb's [`Reference`] patient acts on — zero or one.
    /// Plurality is never the verb's: a "for each"/"all" instruction is an
    /// enclosing [`Instruction::Each`]/[`Instruction::Distribute`] whose
    /// body names a single reference per element ([CR#608.2]). A 1-element
    /// vector keeps the verb arms' batch-shaped `.into_iter()…` bodies; null
    /// and departed current-only patients become the empty set so no action
    /// event is constructed for them ([CR#608.2b]).
    pub(crate) fn eval_reference_set(
        &self,
        reference: &Reference,
        frame: &ExecutionFrame,
    ) -> Vec<ObjectId> {
        let object = self.eval_reference(reference, frame);
        self.objects
            .get(object)
            .map_or_else(Vec::new, |_| vec![object])
    }

    /// Resolve the complete object product carried by a reference. Declared
    /// region registers read their activation-table values.
    pub(crate) fn eval_reference_product(
        &self,
        reference: &Reference,
        frame: &ExecutionFrame,
    ) -> crate::activation::ReferenceProduct {
        if let Reference::Reg(register) = reference {
            return self
                .activation_product(frame.activation, *register)
                .unwrap_or(crate::activation::ReferenceProduct {
                    current: None,
                    lki: None,
                });
        }

        let current_id = self.eval_reference(reference, frame);
        let current = self.objects.get(current_id).map(|_| current_id);
        let lki = None;
        crate::activation::ReferenceProduct { current, lki }
    }

    /// Resolve a [`Reference`] to an `ObjectId`.
    ///
    /// # Panics
    ///
    /// Panics on a `Reference` not wired for Stage 3, an unbound slot
    /// anaphor, or an `AttachHostOf`/`AttachedTo` over an
    /// attachment/host with no live link (the reference is only well-defined
    /// where the relation is established).
    pub(crate) fn eval_reference(&self, reference: &Reference, frame: &ExecutionFrame) -> ObjectId {
        match reference {
            Reference::Reg(_) => {
                let product = self.eval_reference_product(reference, frame);
                product.current.unwrap_or_else(ObjectId::null)
            }
            Reference::OpponentOf(inner) => {
                let player_object = self.eval_reference(inner, frame);
                let Some(player) = self.players.iter().find(|p| p.object == player_object) else {
                    return ObjectId::null();
                };
                let opponent = self.next_live_after(player.id);
                self.player(opponent).object
            }
            // Demote a group-valued expression to one Entity. Empty or ambiguous
            // groups fail closed rather than silently choosing a member.
            Reference::Single(selection) => {
                let values = self.eval_selection_set(selection, frame);
                if let [only] = values.as_slice() { *only } else { ObjectId::null() }
            }
            // [CR#109.5]: the derived controller of a referenced object.
            Reference::ControllerOf(inner) => {
                let id = self.eval_reference(inner, frame);
                if id.is_null()
                    || self.players.iter().any(|p| p.object == id)
                    || self.objects.get(id).is_none()
                {
                    ObjectId::null()
                } else {
                    self.player(self.layers().controller(id)).object
                }
            }
            Reference::Coalesce(references) => references
                .iter()
                .map(|reference| self.eval_reference(reference, frame))
                .find(|id| !id.is_null())
                .unwrap_or_else(ObjectId::null),
            // [CR#108.3]: the owner of a referenced (card-backed) object.
            Reference::OwnerOf(inner) => {
                let id = self.eval_reference(inner, frame);
                if id.is_null() {
                    return id;
                }
                self.player(self.owner_of(id)).object
            }
            // [CR#301.5,303.4]: the host an attachment is attached to — read
            // the attachment→host relation directly off the resolved object.
            Reference::AttachHostOf(inner) => {
                let id = self.eval_reference(inner, frame);
                if id.is_null() {
                    return id;
                }
                self.objects.obj(id).attached_to.unwrap_or_else(|| {
                    Self::unbound_ref(reference, "AttachHostOf on an unattached object")
                })
            }
        }
    }

    /// Semantic-input errors in card data must never crash the engine (the
    /// Invalid semantic input fizzles decision,
    /// `docs/decisions/invalid-semantic-input-fizzles.md`): an unresolvable
    /// semantic reference degrades to the null [`ObjectId`] — a slotmap key
    /// that never maps to a live object, so the effect fizzles exactly as a
    /// reference to a departed/zone-changed object does — leaving an
    /// `eprintln!` breadcrumb. Soundness (a well-formed card's references
    /// always resolving) is the Idris re-emit gate's job, not a runtime
    /// panic; skipping that gate is UB whose worst case here is a silent
    /// no-op, never a crash.
    fn unbound_ref(reference: &Reference, why: &str) -> ObjectId {
        eprintln!(
            "deckmaste: reference {reference:?} did not resolve ({why}); treating as unbound — effect fizzles"
        );
        ObjectId::null()
    }

    /// The group twin of [`Self::unbound_ref`]: a selection that names nothing
    /// resolvable fizzles to the EMPTY group, so its verb no-ops. A semantic-
    /// input error never crashes the engine — the Idris re-emit gate is what
    /// refuses the shape.
    fn unbound_group(selection: &Selection, why: &str) -> Vec<ObjectId> {
        eprintln!(
            "deckmaste: selection {selection:?} did not resolve ({why}); treating as unbound — effect fizzles"
        );
        Vec::new()
    }
}

/// Whether `filter` states NO quality at all — "a card"/"N cards", the
/// bare-quantity search [CR#701.23d] compels finding that many (or as many as
/// exist); anything else is a STATED quality ([CR#701.23b]), which never
/// compels a find even when matches are present. `Kind(Card)` is what the
/// migrations parser emits for a bare "a card"; `Any` is its match-everything
/// twin.
pub(crate) fn search_is_bare_quantity(filter: &deckmaste_core::Predicate) -> bool {
    matches!(
        filter,
        deckmaste_core::Predicate::Class(deckmaste_core::ObjectClass::Card)
            | deckmaste_core::Predicate::Any
    )
}

#[cfg(test)]
mod tests {
    #![allow(
        clippy::empty_line_after_doc_comments,
        reason = "related behavioral test rationale is intentionally grouped"
    )]
    use std::sync::Arc;

    use deckmaste_core::Action;
    use deckmaste_core::Instruction;
    use deckmaste_core::Predicate;
    use deckmaste_core::Reference;
    use deckmaste_core::Selection;
    use deckmaste_core::StatePredicate;
    use deckmaste_core::Zone;
    use slotmap::Key;

    use crate::agenda::WorkItem;
    use crate::event::Occurrence;
    use crate::player::PlayerId;
    use crate::resolve::fixtures::*;
    use crate::test_support::frame_for;
    use crate::test_support::frame_src;
    use crate::test_support::frame_src_targets;

    #[test]
    fn activation_target_product_keeps_lki_after_departure() {
        use deckmaste_core::DefId;
        use deckmaste_core::Kind;
        use deckmaste_core::Param;
        use deckmaste_core::Provenance;
        use deckmaste_core::RefId;
        use deckmaste_core::Region;

        let (mut state, target) = bear_on_field();
        let source = state.player(PlayerId(0)).object;
        let provenances = [
            Provenance::Source,
            Provenance::Controller,
            Provenance::EventObject,
            Provenance::EventPatient,
            Provenance::EventActor,
            Provenance::DefendingPlayer,
            Provenance::AnnouncedTarget(0),
            Provenance::AnnouncedX,
        ];
        let params: Arc<[Param]> = provenances
            .into_iter()
            .enumerate()
            .map(|(index, provenance)| Param {
                def: DefId(u32::try_from(index).expect("fixture parameter index fits u32")),
                kind: if index == 6 {
                    Kind::Entities
                } else if index == 7 {
                    Kind::Number
                } else {
                    Kind::Entity
                },
                provenance,
            })
            .collect::<Vec<_>>()
            .into();
        let region = Region::new(params, ());
        let mut frame = frame_src_targets(&state, source, vec![target]);
        frame.activation = state.enter_region(&region, &frame);

        let snapshot = crate::lki::LkiSnapshot::capture(&state, target);
        state.activation_departed(target, &snapshot);
        state.objects.remove(target);
        let product = state.eval_reference_product(&Reference::Reg(RefId(6)), &frame);
        assert_eq!(product.current, None);
        assert_eq!(product.lki.as_ref().map(|lki| lki.object), Some(target));
        assert!(
            state
                .eval_reference_set(&Reference::Reg(RefId(6)), &frame)
                .is_empty(),
            "one-shot verbs pack a departed current value as the empty set"
        );
    }

    /// [CR#608.2b]: an announced target that has left its expected zone is no
    /// longer a resolvable current object. Singular reads become null and the
    /// action-patient set becomes empty, so instructions aimed at it fizzle.
    #[test]
    fn departed_target_reads_null_and_empty_patient_set() {
        let (mut state, source, departed) = two_permanents_on_field();
        let frame = frame_src_targets(&state, source, vec![departed]);
        state.zones.battlefield.retain(|&object| object != departed);
        state.objects.remove(departed);

        assert!(
            state
                .eval_reference(&Reference::Reg(deckmaste_core::RefId(6)), &frame)
                .is_null()
        );
        assert!(
            state
                .eval_reference_set(&Reference::Reg(deckmaste_core::RefId(6)), &frame)
                .is_empty()
        );
    }

    /// Bound-role reads chase the move record ([CR#400.7j]): a One `that`
    /// binding and explicit `It` bindings resolve to the object's latest
    /// same-resolution incarnation; announced target registers never chase
    /// (the announced slot stays positional).

    /// A Many `that` group chases per element via `Selection::They`.

    /// [CR#400.7,603.7c]: an object-op whose bound role resolved to a GONE id
    /// (hidden destination / stale) is a NO-OP, not a panic.

    /// The product-sited `That(Sort)` ([CR#400.7j] — "exile it, then return
    /// THAT CARD"): with no `that` binding, the read resolves to the newest
    /// same-resolution move product; once that product leaves for a hidden
    /// zone it is NOT found and the read degrades to null (never an older
    /// antecedent).

    /// `With(Produce(Move(...)), body)` binds the move's PRE-move id as a One
    /// `That`; after the move applies, the body's `That` chases to the
    /// product ([CR#400.7j]).

    /// [CR#301.5]: `AttachHostOf(This)` from the attachment resolves to its
    /// host.
    #[test]
    fn eval_reference_attach_host_of() {
        let (mut state, a, b) = two_permanents_on_field();
        state.objects.obj_mut(a).attached_to = Some(b);

        let frame_a = frame_src(&state, a);
        assert_eq!(
            state.eval_reference(
                &Reference::AttachHostOf(Arc::new(Reference::Reg(deckmaste_core::RefId(0)))),
                &frame_a
            ),
            b,
            "AttachHostOf(This) from a is its host b"
        );
    }

    /// `eval_selection_set` returns the bound set for a `Random` slot — the
    /// value the RNG wrote into the deciding instruction's OWN dest register.
    /// There is no shared choice slot and no presence flag to test: a chooser
    /// nested under another chooser writes a different register, so neither
    /// can read the other's pick by accident.

    /// A foreign chooser routes the `ChooseObjects` decision to the binder's
    /// resolved `by` player, not the spell's controller ([CR#608.2d] — "that
    /// player sacrifices a creature of their choice", [CR#701.21a]).

    /// The buildable-now references: `ControllerOf`/`OwnerOf` distinguish
    /// control from ownership ([CR#109.5,108.3]); `EventObject`/`EventActor`
    /// read the trigger bindings ([CR#603.10a]).
    #[test]
    fn references_resolve_controller_owner_and_trigger_bindings() {
        let (mut state, bear) = bear_on_field();
        // A second Grizzly Bears from player 0's hand onto the battlefield,
        // then handed to player 1: owner stays player 0, controller
        // becomes player 1.
        let theirs = second_bear_to_player_1(&mut state);

        let mut frame = frame_src_targets(&state, bear, vec![theirs]);
        state.frame_set_source_lki(
            &mut frame,
            Some(crate::lki::LkiSnapshot::capture(&state, bear)),
        );
        state.frame_set_event_bindings(
            &mut frame,
            Some(crate::lki::LkiSnapshot::capture(&state, theirs)),
            Some(PlayerId(1)),
            None,
        );

        assert_eq!(
            state.eval_reference(
                &Reference::ControllerOf(Arc::new(Reference::Reg(deckmaste_core::RefId(6)))),
                &frame
            ),
            state.player(PlayerId(1)).object,
            "controller of player 1's creature is player 1"
        );
        let payer = Reference::Coalesce(
            vec![
                Reference::ControllerOf(Arc::new(Reference::Reg(deckmaste_core::RefId(6)))),
                Reference::Reg(deckmaste_core::RefId(6)),
            ]
            .into(),
        );
        assert_eq!(
            state.eval_reference(&payer, &frame),
            state.player(PlayerId(1)).object,
            "a nonplayer target selects its controller"
        );
        let mut player_frame = state.fork_frame(&frame);
        let player_object = state.player(PlayerId(1)).object;
        state.frame_set_targets(&mut player_frame, &[vec![player_object]]);
        assert_eq!(
            state.eval_reference(&payer, &player_frame),
            state.player(PlayerId(1)).object,
            "a player target falls back to the player itself"
        );
        let only_you = Reference::Single(Arc::new(Selection::SelectAll(Arc::new(
            deckmaste_core::Region::candidate(Predicate::Ref(Reference::Reg(
                deckmaste_core::RefId(1),
            ))),
        ))));
        assert_eq!(
            state.eval_reference(&only_you, &frame),
            state.player(PlayerId(0)).object,
            "Single resolves a singleton selection"
        );
        let ambiguous_players = Reference::Single(Arc::new(Selection::SelectAll(Arc::new(
            deckmaste_core::Region::candidate(Predicate::Entity(
                deckmaste_core::EntityClass::Player,
            )),
        ))));
        assert!(
            state.eval_reference(&ambiguous_players, &frame).is_null(),
            "Single fails closed when the selection has multiple values"
        );
        assert_eq!(
            state.eval_reference(
                &Reference::OwnerOf(Arc::new(Reference::Reg(deckmaste_core::RefId(6)))),
                &frame
            ),
            state.player(PlayerId(0)).object,
            "owner is still player 0"
        );
        // The provenance-explicit event roles ([CR#603.2e]): the OBJECT (the
        // moved/acting object) and the ACTOR (the responsible player).
        assert_eq!(
            state.eval_reference(&Reference::Reg(deckmaste_core::RefId(2)), &frame),
            theirs,
            "EventObject is the bound snapshot's object (the moved/acting object)"
        );
        assert_eq!(
            state.eval_reference(&Reference::Reg(deckmaste_core::RefId(4)), &frame),
            state.player(PlayerId(1)).object,
            "EventActor is the responsible player"
        );
    }

    /// The provenance-explicit patient and defending-player roles resolve from
    /// their dedicated binding slots ([CR#608.2k,120.3,506.2]): a kind-poly
    /// `EventPatient` (object or player) and an always-player
    /// `DefendingPlayer`, both distinct from the agent/actor.
    #[test]
    fn event_patient_and_defending_player_resolve() {
        let (mut state, bear) = bear_on_field();
        let theirs = second_bear_to_player_1(&mut state);

        // An OBJECT patient (a damage recipient creature) distinct from the
        // agent — what makes a two-object event spellable.
        let mut object_patient = frame_src(&state, bear);
        state.frame_set_source_lki(
            &mut object_patient,
            Some(crate::lki::LkiSnapshot::capture(&state, bear)),
        );
        state.frame_set_defending_player(&mut object_patient, Some(PlayerId(1)));
        state.frame_set_event_bindings(
            &mut object_patient,
            None,
            None,
            Some(crate::trigger::EventPatient::Object(
                crate::lki::LkiSnapshot::capture(&state, theirs),
            )),
        );
        assert_eq!(
            state.eval_reference(&Reference::Reg(deckmaste_core::RefId(3)), &object_patient),
            theirs,
            "an object patient resolves to its object"
        );
        assert_eq!(
            state.eval_reference(&Reference::Reg(deckmaste_core::RefId(5)), &object_patient),
            state.player(PlayerId(1)).object,
            "DefendingPlayer is always the player proxy"
        );

        // A PLAYER patient (a damage recipient player) resolves to the proxy.
        let mut player_patient = frame_src(&state, bear);
        state.frame_set_event_bindings(
            &mut player_patient,
            None,
            None,
            Some(crate::trigger::EventPatient::Player(PlayerId(1))),
        );
        assert_eq!(
            state.eval_reference(&Reference::Reg(deckmaste_core::RefId(3)), &player_patient),
            state.player(PlayerId(1)).object,
            "a player patient resolves to the player proxy"
        );
    }

    /// `LibraryOf(whose)` reads the WHOLE library, top→bottom — the un-sliced
    /// twin of `TopOfLibrary` and shuffle's own object ([CR#701.24a]). Checked
    /// against the zone itself (order-exact, not merely set-equal) and against
    /// the slice family, whose read must be this one's prefix.

    /// `InChosenOrder(sel, by)` is a pure ORDERING wrapper — membership is
    /// exactly the inner group's. The chooser-driven sequence is a recorded
    /// seam ([CR#707.10d]), so today it degrades to the inner order rather
    /// than to the empty group: the player-visible sequence is lost, never a
    /// member.
    #[test]
    fn in_chosen_order_preserves_the_inner_group_pending_the_choice_seam() {
        let (state, _bear) = bear_on_field();
        let frame = frame_for(&state, PlayerId(0));

        let inner = Selection::LibraryOf(Reference::Reg(deckmaste_core::RefId(1)));
        let direct = state.eval_selection_set(&inner, &frame);
        assert!(!direct.is_empty(), "fixture leaves a non-empty library");
        assert_eq!(
            state.eval_selection_set(
                &Selection::InChosenOrder(
                    Arc::new(inner),
                    Reference::Reg(deckmaste_core::RefId(1))
                ),
                &frame,
            ),
            direct,
            "the wrapper changes no members while the choice is unwired"
        );
    }

    /// A per-candidate predicate region — the candidate at parameter zero,
    /// then the enclosing source and controller.
    fn candidate_region<T>(body: T) -> Arc<deckmaste_core::Region<T>> {
        Arc::new(deckmaste_core::Region::new(
            Arc::from([
                deckmaste_core::Param {
                    def: deckmaste_core::DefId(0),
                    kind: deckmaste_core::Kind::Entity,
                    provenance: deckmaste_core::Provenance::Candidate(
                        deckmaste_core::Domain::Entity,
                    ),
                },
                deckmaste_core::Param {
                    def: deckmaste_core::DefId(1),
                    kind: deckmaste_core::Kind::Entity,
                    provenance: deckmaste_core::Provenance::Source,
                },
                deckmaste_core::Param {
                    def: deckmaste_core::DefId(2),
                    kind: deckmaste_core::Kind::Entity,
                    provenance: deckmaste_core::Provenance::Controller,
                },
            ]),
            body,
        ))
    }

    /// Creatures on the battlefield — the filter the choice fixtures use.
    fn creatures_on_the_battlefield() -> Predicate {
        Predicate::And(
            vec![
                Predicate::State(StatePredicate::InZone(Zone::Battlefield)),
                Predicate::creature(),
            ]
            .into(),
        )
    }

    /// The first instruction definition of a `frame_src` activation: source(0),
    /// controller(1), the four event roles(2..=5), announced X(6).
    const FIRST_DEF: deckmaste_core::DefId = deckmaste_core::DefId(7);

    /// The first instruction definition of a one-target `frame_src_targets`
    /// activation, which inserts the announced slot at register 6 and X at 7.
    const FIRST_DEF_WITH_TARGET: deckmaste_core::DefId = deckmaste_core::DefId(8);

    /// ADR law 2, the provenance split: an instruction PRODUCT is a new object
    /// ([CR#400.7j]) and its register read chases the same-resolution move
    /// record to the object it became; an ANNOUNCED slot is positional and
    /// keeps the partial-fizzle read ([CR#608.2b]) — it never chases onto a
    /// successor. Re-spelled from `bound_role_reads_chase_the_move_record`,
    /// whose `That`/`It` bindings are these two registers.
    #[test]
    fn product_registers_chase_the_move_record_but_announced_slots_do_not() {
        let (mut state, a, b) = two_permanents_on_field();
        let frame = frame_src_targets(&state, a, vec![a]);
        state.activation_write_object(frame.activation, FIRST_DEF_WITH_TARGET, a);
        state.moved_chain.push((a, b));
        state.objects.remove(a);

        assert_eq!(
            state.eval_reference(&Reference::Reg(FIRST_DEF_WITH_TARGET.into()), &frame),
            b,
            "an instruction product reads its latest same-resolution incarnation"
        );
        assert!(
            state
                .eval_reference(&Reference::Reg(deckmaste_core::RefId(6)), &frame)
                .is_null(),
            "the announced slot stays positional: a departed target fizzles, it does not chase"
        );
    }

    /// A group read returns the register the deciding instruction wrote — the
    /// chooser writes its own `dest` and later instructions read it
    /// ([CR#601.2b]); no shared slot, no re-surfacing. A `Random` selection is
    /// a DECISION and is never evaluated by this pure reader
    /// ([CR#608.2h], ADR law 5). Re-spelled from
    /// `eval_selection_set_reads_bound_choice`.
    #[test]
    fn a_group_read_returns_the_register_the_decision_wrote() {
        let (state, bear) = bear_on_field();
        let frame = frame_src(&state, bear);
        state.activation_write_objects(frame.activation, FIRST_DEF, &[bear]);
        assert_eq!(
            state.eval_selection_set(&Selection::Reg(FIRST_DEF.into()), &frame),
            vec![bear]
        );
        assert!(
            state
                .eval_selection_set(
                    &Selection::Random(
                        deckmaste_core::Quantity::one(),
                        std::sync::Arc::new(deckmaste_core::Region::over(
                            creatures_on_the_battlefield()
                        ))
                    ),
                    &frame
                )
                .is_empty(),
            "a decision-bearing selection never resolves in a pure read"
        );
    }

    /// A foreign chooser routes the `ChooseObjects` decision to the choice
    /// instruction's resolved `by` player, not the spell's controller
    /// ([CR#608.2d] — "that player sacrifices a creature of their choice",
    /// [CR#701.21a]).
    #[test]
    fn foreign_by_routes_choice_to_that_player() {
        use crate::decide::DecisionPointKind;

        let (mut state, bear) = bear_on_field();
        let _theirs = second_bear_to_player_1(&mut state);
        let frame = frame_src(&state, bear);
        state.run_effect(
            Instruction::Sequentially(
                vec![
                    Instruction::Choose(deckmaste_core::Choose {
                        dest: FIRST_DEF,
                        by: Reference::OpponentOf(std::sync::Arc::new(
                            Reference::controller_parameter(),
                        )),
                        quantity: deckmaste_core::Quantity::one(),
                        filter: candidate_region(creatures_on_the_battlefield()),
                    }),
                    Instruction::act(Action::destroy(Reference::Reg(FIRST_DEF.into()))),
                ]
                .into(),
            ),
            &frame,
        );
        drain_progress(&mut state, 20);
        let Some(DecisionPointKind::ChooseObjects(crate::decide::pending::ChooseObjects {
            player,
            ..
        })) = state.pending.clone()
        else {
            panic!("expected ChooseObjects, got {:?}", state.pending);
        };
        assert_eq!(
            player,
            PlayerId(1),
            "the opponent (the choice instruction's `by`) makes the pick"
        );
    }

    /// A group register chases PER ELEMENT ([CR#400.7j]): each member reads
    /// its own latest same-resolution incarnation, and a member with no move
    /// record stays itself. Re-spelled from `group_read_chases_per_element`,
    /// whose Many `that` binding is this register.
    #[test]
    fn group_register_reads_chase_per_element() {
        let (mut state, a, b) = two_permanents_on_field();
        // `c` MUST come from the SAME state (see
        // `chase_moved_follows_chain_transitively` on cross-state id
        // collision).
        let c = state.objects.mint(
            crate::object::ObjectSource::Player(PlayerId(1)),
            PlayerId(1),
            Some(Zone::Battlefield),
        );
        let frame = frame_src(&state, b);
        state.activation_write_objects(frame.activation, FIRST_DEF, &[a, b]);
        state.moved_chain.push((a, c));
        state.objects.remove(a);
        assert_eq!(
            state.eval_selection_set(&Selection::Reg(FIRST_DEF.into()), &frame),
            vec![c, b],
            "a chases to c; b unrecorded stays b"
        );
    }

    /// `LibraryOf(whose)` reads the WHOLE library, top→bottom — the un-sliced
    /// twin of `TopOfLibrary` and shuffle's own object ([CR#701.24a]). Checked
    /// against the zone itself (order-exact, not merely set-equal) and against
    /// the slice family, whose read must be this one's prefix.
    #[test]
    fn library_of_reads_the_whole_library_top_to_bottom() {
        /// A register no activation declares — the frameless unbound read.
        const UNBOUND: deckmaste_core::RefId = deckmaste_core::RefId(20);

        let (state, _bear) = bear_on_field();
        let frame = frame_for(&state, PlayerId(0));

        let expected: Vec<_> = state.zones.libraries[PlayerId(0).index()]
            .iter()
            .copied()
            .collect();
        assert!(
            expected.len() > 2,
            "fixture must leave a multi-card library for an order check"
        );
        assert_eq!(
            state.eval_selection_set(
                &Selection::LibraryOf(Reference::controller_parameter()),
                &frame
            ),
            expected,
            "LibraryOf(controller) is the whole library, in zone order"
        );
        assert_eq!(
            state.eval_selection_set(
                &Selection::TopOfLibrary {
                    count: deckmaste_core::Count::Literal(2),
                    whose: Reference::controller_parameter(),
                },
                &frame,
            ),
            expected[..2].to_vec(),
            "the slice family reads a prefix of the same ordered zone"
        );

        // A `whose` that is not a player proxy fizzles to the empty group
        // rather than panicking — the `TopOfGraveyard` convention, not the
        // slice family's panic (semantic-input errors never crash the engine).
        assert!(
            state
                .eval_selection_set(&Selection::LibraryOf(Reference::Reg(UNBOUND)), &frame)
                .is_empty(),
            "an unresolvable whose fizzles, never panics"
        );
    }

    /// [CR#400.7,603.7c]: an object-op whose register resolved to a GONE id
    /// (hidden destination / stale) is a NO-OP, not a panic. Re-spelled from
    /// `move_of_a_gone_bound_role_is_a_noop`.
    #[test]
    fn move_of_a_gone_register_is_a_noop() {
        use deckmaste_core::Destination;

        let (mut state, a, b) = two_permanents_on_field();
        let frame = frame_src(&state, b);
        state.activation_write_object(frame.activation, FIRST_DEF, a);
        // Kill `a` outright (no record entry — e.g. it bounced to hand).
        state.objects.remove(a);
        let items = state.move_items(
            &Reference::Reg(FIRST_DEF.into()),
            &Destination::Zone(Zone::Exile),
            None,
            &[],
            &frame,
        );
        assert!(
            items.iter().all(|item| match item {
                WorkItem::Emit(Occurrence::Batch(v)) => v.is_empty(),
                WorkItem::Emit(Occurrence::Single(_)) => false,
                _ => true,
            }),
            "a gone register produces no zone-change emit",
        );
    }

    /// The move product's own definition ([CR#400.7j] — "exile it, then return
    /// THAT CARD"): an `Act` with a `dest` writes the object it became, and
    /// the register reads that product; once the product leaves for a HIDDEN
    /// zone nothing is recorded, so the read degrades to null rather than
    /// finding an older antecedent ([CR#400.7]). Re-spelled from
    /// `product_sited_that_reads_the_newest_live_move_product` — the
    /// product-sited `That` is a declared definition now.
    #[test]
    fn a_move_products_definition_reads_the_live_product_only() {
        use slotmap::Key;

        let (mut state, a, _) = two_permanents_on_field();
        let frame = frame_src(&state, a);

        // Nothing written yet: unbound.
        assert!(
            state
                .eval_reference(&Reference::Reg(FIRST_DEF.into()), &frame)
                .is_null()
        );

        // Exile `a` through the real effect machinery: the apply records the
        // public move and the instruction writes its product.
        state.run_effect(
            Instruction::Act {
                dest: Some(FIRST_DEF),
                action: Action::Move(
                    Reference::source_parameter(),
                    deckmaste_core::Destination::Zone(Zone::Exile),
                    vec![].into(),
                    None,
                ),
            },
            &frame,
        );
        run_injected(&mut state);
        let product = state.chase_moved(a);
        assert_ne!(product, a, "the exile reminted a new object");
        assert_eq!(
            state.eval_reference(&Reference::Reg(FIRST_DEF.into()), &frame),
            product,
            "the product definition reads the exile product"
        );

        // Send the product to a HIDDEN zone: nothing is recorded, so the
        // register finds nothing and does not fall back to an older object.
        let pframe = frame_src(&state, product);
        state.run_effect(
            Instruction::act(Action::Move(
                Reference::source_parameter(),
                deckmaste_core::Destination::Zone(Zone::Hand),
                vec![].into(),
                None,
            )),
            &pframe,
        );
        run_injected(&mut state);
        assert!(
            state
                .eval_reference(&Reference::Reg(FIRST_DEF.into()), &frame)
                .is_null(),
            "a product that left its public zone is NOT found ([CR#400.7])"
        );
    }

    /// Semantic-input errors in card data never crash the engine (the Invalid
    /// semantic input fizzles decision,
    /// `docs/decisions/invalid-semantic-input-fizzles.md`, and ADR law 10): a
    /// register read that finds nothing degrades to the null object id (the
    /// effect fizzles) rather than panicking. Validation at load makes "never
    /// bound" unrepresentable; what remains at run time is bound-but-departed,
    /// and that must fizzle.
    #[test]
    fn unbound_reference_degrades_to_null_not_panic() {
        use deckmaste_core::Reference;
        use slotmap::Key;

        let state = game();
        let frame = frame_for(&state, PlayerId(0));
        // An out-of-range positional read degrades on both channels —
        // never-crash, never a wrong slot.
        assert!(
            state
                .eval_reference(&Reference::Reg(deckmaste_core::RefId(6)), &frame)
                .is_null()
        );
        assert!(
            state
                .eval_selection_set(
                    &deckmaste_core::Selection::Reg(deckmaste_core::RefId(6)),
                    &frame
                )
                .is_empty()
        );
        // A register far past any declared parameter — the shape a stale
        // instruction product would take.
        assert!(
            state
                .eval_reference(&Reference::Reg(deckmaste_core::RefId(20)), &frame)
                .is_null()
        );
        assert!(
            state
                .eval_selection_set(
                    &deckmaste_core::Selection::Reg(deckmaste_core::RefId(20)),
                    &frame
                )
                .is_empty()
        );
        // Event roles read outside any trigger — were `.expect()` panics.
        for role in [2_u32, 4, 5] {
            assert!(
                state
                    .eval_reference(&Reference::Reg(deckmaste_core::RefId(role)), &frame)
                    .is_null(),
                "event role register {role} outside a trigger is null, not a panic"
            );
        }
        // A derived reference over an unbound inner stays null, not a secondary
        // panic in `layers().controller()`.
        assert!(
            state
                .eval_reference(
                    &Reference::ControllerOf(Arc::new(Reference::Reg(deckmaste_core::RefId(20)))),
                    &frame
                )
                .is_null()
        );
    }

    /// [CR#707.10d]: `ValidTargetsFor` INTERSECTS the spell's per-slot legal
    /// sets — "each object that the spell could target" means legal in EVERY
    /// slot at once, not in any one of them.
    ///
    /// The board holds two creatures and one land; the spell announces two
    /// slots, one admitting creatures and one admitting anything on the
    /// battlefield. A union would yield all three objects and a first-slot-only
    /// read would coincidentally also yield the creatures — so the land is what
    /// discriminates: it is legal for slot 1 and must still be absent.
    #[test]
    fn valid_targets_for_intersects_slots_rather_than_unioning_them() {
        use deckmaste_card::Card;
        use deckmaste_card::CardFace;
        use deckmaste_core::Ability;
        use deckmaste_core::Quantity;
        use deckmaste_core::SpellAbility;
        use deckmaste_core::TargetSpec;
        use deckmaste_core::Type;

        use crate::object::ObjectSource;
        use crate::stack::StackEntry;
        use crate::stack::StackObject;

        let (mut state, bear_a, bear_b) = two_permanents_on_field();
        let land = mint_on_field(
            &mut state,
            Card::Normal(CardFace {
                name: "Test Land".into(),
                types: vec![Type::Land.def()],
                ..CardFace::default()
            }),
        );

        let face = CardFace {
            name: "Two-Slot Spell".into(),
            types: vec![Type::Instant.def()],
            abilities: vec![Ability::Spell(Arc::new(SpellAbility {
                ability_word: None,
                cost: deckmaste_core::Cost([].into()),
                targets: vec![
                    // Slot 0: creatures only.
                    TargetSpec::Target(Quantity::one(), candidate_region(Predicate::creature())),
                    // Slot 1: anything on the battlefield — a strict superset.
                    TargetSpec::Target(
                        Quantity::one(),
                        candidate_region(Predicate::State(StatePredicate::InZone(
                            Zone::Battlefield,
                        ))),
                    ),
                ]
                .into(),
                // The body is irrelevant to the read under test; any
                // slot-referencing action keeps the declaration well-formed.
                effect: Instruction::act(Action::deal_damage(
                    Reference::Reg(deckmaste_core::RefId(6)),
                    deckmaste_core::Count::Literal(1),
                ))
                .into(),
            }))],
            ..CardFace::default()
        };
        let cid = state.cards.push(Arc::new(Card::Normal(face)), PlayerId(0));
        let spell = state
            .objects
            .mint(ObjectSource::Card(cid), PlayerId(0), Some(Zone::Stack));
        state.stack.push(StackEntry {
            activation: crate::ActivationId::NONE,
            id: spell,
            object: StackObject::Spell(spell),
            controller: PlayerId(0),
            targets: vec![],
            chosen_modes: std::sync::Arc::from([]),
            x: None,
            paid_costs: Vec::new(),
            copy: false,
        });

        // Premise check. Without it the intersection assertion below could pass
        // vacuously: a land legal for NEITHER slot would also be absent from
        // the result, proving nothing about the fold. Pin that the land really
        // is legal for slot 1, so a union would demonstrably admit it.
        let view = state.layers();
        let specs = state.stack_object_target_specs(&view, &StackObject::Spell(spell), &[]);
        let per_slot = state.legal_targets_for_specs(&specs, spell, crate::ActivationId::NONE);
        assert_eq!(per_slot.len(), 2, "the spell announces two slots");
        assert!(
            !per_slot[0].contains(&land),
            "slot 0 admits creatures only, so it excludes the land"
        );
        assert!(
            per_slot[1].contains(&land),
            "slot 1 admits the whole battlefield INCLUDING the land — this is what \
             makes the intersection assertion discriminating"
        );

        // The source register names the spell itself.
        let frame = crate::test_support::frame_src(&state, spell);
        let mut got = state.eval_selection_set(
            &Selection::ValidTargetsFor(Reference::source_parameter()),
            &frame,
        );
        got.sort_unstable();
        let mut want = vec![bear_a, bear_b];
        want.sort_unstable();
        assert_eq!(
            got, want,
            "the intersection is the creatures; the land is legal for slot 1 only and must drop"
        );
        assert!(
            !got.contains(&land),
            "a union fold would have admitted the land"
        );

        // A reference naming no live stack entry fizzles to the empty group.
        let bare = frame_for(&state, PlayerId(0));
        assert!(
            state
                .eval_selection_set(
                    &Selection::ValidTargetsFor(Reference::Reg(deckmaste_core::RefId(20))),
                    &bare
                )
                .is_empty(),
            "ValidTargetsFor over a non-stack reference fizzles, never panics"
        );
    }

    /// A producing instruction writes the MOVED OBJECT'S PRODUCT to its
    /// destination, and the next instruction reads that register
    /// ([CR#400.7j]) — the exile-and-return shape. Re-spelled from
    /// `with_produce_binds_the_moved_objects_product`, whose `Binder::Produce`
    /// is an `Act { dest }` now.
    #[test]
    fn a_producing_instruction_binds_the_moved_objects_product() {
        use deckmaste_core::Destination;

        let (mut state, a, _) = two_permanents_on_field();
        let frame = frame_src(&state, a);
        state.run_effect(
            Instruction::Sequentially(
                vec![
                    Instruction::Act {
                        dest: Some(FIRST_DEF),
                        action: Action::Move(
                            Reference::source_parameter(),
                            Destination::Zone(Zone::Exile),
                            vec![].into(),
                            None,
                        ),
                    },
                    Instruction::act(Action::Move(
                        Reference::Reg(FIRST_DEF.into()),
                        Destination::Zone(Zone::Battlefield),
                        vec![].into(),
                        None,
                    )),
                ]
                .into(),
            ),
            &frame,
        );
        run_injected(&mut state);
        // The original id is gone; a NEW object is back on the battlefield.
        assert!(state.objects.get(a).is_none(), "original exiled (stale)");
        let back = state.chase_moved(a);
        assert_ne!(back, a);
        assert_eq!(
            state.objects.get(back).unwrap().zone,
            Some(Zone::Battlefield)
        );
    }
}
