//! Reference/selection evaluation: resolve `Reference`s and `Selection`s to
//! object sets against a frame.

use deckmaste_core::Count;
use deckmaste_core::Reference;
use deckmaste_core::Selection;
use deckmaste_core::Uint;
use slotmap::Key;

use super::deref_quantity;
use crate::object::ObjectId;
use crate::object::ObjectSource;
use crate::stack::Anaphora;
use crate::stack::Frame;
use crate::state::GameState;

impl GameState {
    /// The product-sited `That(Sort)` antecedent ([CR#400.7j] — "exile it,
    /// then return THAT CARD"): the NEWEST object this resolution moved to a
    /// public zone (recency = the R1-nearest antecedent; the sort was proven
    /// compatible by the Idris re-emit gate, so runtime takes the binding).
    /// Chased through any later same-resolution moves; `None` when nothing
    /// moved, or when the product has since left its public zone
    /// ([CR#400.7] — a gone product is NOT found, never replaced by an older
    /// antecedent).
    fn newest_move_product(&self) -> Option<ObjectId> {
        let &(_, new) = self.moved_chain.last()?;
        let chased = self.chase_moved(new);
        self.objects.get(chased).is_some().then_some(chased)
    }

    /// Resolves the agent of a `By(who, …)` to the acting `PlayerId`. `who` is
    /// a [`Reference`] that resolves (via `eval_reference`) to a player proxy
    /// object; this maps that proxy back to its `PlayerId`.
    ///
    /// # Panics
    ///
    /// Panics if `who` resolves to a non-player object — a player verb's agent
    /// must be a player ([CR#608.2]).
    pub(crate) fn acting_player(&self, who: &Reference, frame: &Frame) -> crate::player::PlayerId {
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
    /// non-player reference is an authoring mistake that fizzles to 0.
    pub(crate) fn eval_player_ref(
        &self,
        reference: &Reference,
        frame: &Frame,
    ) -> Option<crate::player::PlayerId> {
        let id = self.eval_reference(reference, frame);
        match self.objects.get(id).map(|o| o.source) {
            Some(ObjectSource::Player(p)) => Some(p),
            Some(ObjectSource::Card(_)) | None => None,
        }
    }

    /// (min, max) objects to choose for `quantity`, clamped to `n` available —
    /// choose as many as able when fewer exist ([CR#608.2d]). Also used by the
    /// cost-payability gate (`can_pay_verbs`) to read a selection's required
    /// floor.
    pub(crate) fn choice_bounds(
        &self,
        quantity: &deckmaste_core::Quantity,
        n: usize,
        frame: &Frame,
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

    /// The announced targets flattened across every slot, with departed
    /// (illegal) targets excluded — the plural read-back for `They`/`Them`
    /// ([CR#608.2b] partial fizzle; mirrors `StatePredicate::Targets` ignoring
    /// a gone target).
    fn live_flat_targets(&self, frame: &Frame) -> Vec<ObjectId> {
        frame
            .anaphora
            .targets
            .iter()
            .flatten()
            .copied()
            .filter(|&t| self.objects.get(t).is_some())
            .collect()
    }

    /// A selection (a GROUP) resolved to its full set ([CR#608.2d]) — the
    /// home of plurality now that verbs take a single [`Reference`].
    /// `Predicate` enumerates the matching set;
    /// `They`/`TheGroup`/`TopOfLibrary` name an already-bound group. A
    /// per-object instruction runs over this set via an enclosing `Each`/
    /// `Distribute`/`With`, never the verb itself.
    pub(crate) fn eval_selection_set(&self, sel: &Selection, frame: &Frame) -> Vec<ObjectId> {
        match sel {
            Selection::SelectAll(f) => crate::target::candidates(self, f),

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
                        crate::target::candidates_with(self, filter, Some(watcher))
                    }
                    // Idris's `Pick` is pinned to `Projection b AnObject`
                    // ([CR#107.1] — no player-`Pick` consumer exists), so a
                    // `Players`-sourced `proj` here is authoring-invalid, like
                    // the non-`Projectable` `ManaSymbols`/`Singleton`/
                    // `ManaSpentMatching` sources — fizzle to the empty group
                    // (never-crash on an authoring mistake).
                    Countable::Players(..)
                    | Countable::ManaSymbols(..)
                    | Countable::Singleton(..)
                    | Countable::ManaSpentMatching(..) => Vec::new(),
                };
                let scored: Vec<(ObjectId, Uint)> = candidates
                    .into_iter()
                    .map(|id| {
                        let sub = Frame {
                            anaphora: Anaphora {
                                it: Some(self.it_binding(id)),
                                allotment: None,
                                ..frame.anaphora.clone()
                            },
                            ..frame.clone()
                        };
                        (id, self.eval_count(&proj.by, &sub))
                    })
                    .collect();
                let extremum = match op {
                    AggregateOp::MaxOf => scored.iter().map(|(_, v)| *v).max(),
                    AggregateOp::MinOf => scored.iter().map(|(_, v)| *v).min(),
                    // A non-extremal op on Pick is malformed authoring —
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
            // The RNG's picks, bound into the frame before the selection is
            // read ([CR#608.2d]). No current card surfaces a `Random` group
            // (player choice now lives in `With(ChooseOne/Choose, …)`); this
            // stays a dormant seam until one does.
            Selection::Random(..) => frame
                .anaphora
                .chosen
                .clone()
                .expect("a Random selection is bound into the frame before it is read"),
            Selection::Expanded(e) => self.eval_selection_set(&e.value, frame),
            // The ordered plural group bound by the enclosing many-binder
            // (`OneShotEffect::With`/`Each`/`Distribute`). Reads the `(Many, k)`
            // `that` slot, order-preserved exactly as bound (top→down for a
            // library window). A `(One, k)` binding has NO group read — the
            // singular `Reference::That` is its only reader — so a single object
            // can never be silently splayed into a group here. Panics outside a
            // many-binder `With` — always a bug.
            Selection::They | Selection::Them(_) => {
                // The sort is verified by the Idris re-emit gate; the frame's
                // bound group is the value. Without a `With` binding, the
                // plural slot read: the announced target list is the one Many
                // antecedent (Arc Lightning's 1–3 targets read back as
                // `They` — the runtime twin of the Idris model's R1, sound
                // because R2 refused any second Many candidate). A
                // product-sited plural read (create-two-tokens … They) is
                // [[engine-bound-references]] work.
                let Some(that) = frame.anaphora.that.as_ref() else {
                    // The announced targets are the one Many antecedent — the
                    // FLATTENED live set across every slot, a departed target
                    // excluded (partial fizzle, [CR#608.2b]).
                    if !frame.anaphora.targets.is_empty() {
                        return self.live_flat_targets(frame);
                    }
                    todo!(
                        "engine-bound-references: a product-sited They/Them(Sort) at \
                         runtime (no enclosing With binding, no announced targets)"
                    )
                };
                assert_eq!(
                    that.cardinality,
                    crate::stack::Cardinality::Many,
                    "They/Them reads a group, but the bound choice is a single object \
                     (a one-binder) — read it as That(Sort)",
                );
                // [CR#400.7j]: chase each element through the same-resolution
                // move record — order-preserved.
                that.group.iter().map(|&id| self.chase_moved(id)).collect()
            }
            Selection::PilesOf { .. } => {
                todo!(
                    "engine-piles: labeled pile groups at runtime land with the piles \
                     engine subsystem"
                )
            }
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
            // The top `count` cards of `of`'s graveyard, top→down
            // ([CR#404.2] — a graveyard is a single face-up pile in a fixed
            // order). `zones.graveyards` is push-appended as cards are put
            // into it, so the LAST-pushed card sits physically on top; a
            // front-to-back `Vec` read is bottom→up, so this reads from the
            // END, reversed. Unlike `TopOfLibrary`/`BottomOfLibrary`
            // (which panic on a non-player `of` — an established baseline
            // this arm does not repeat), an `of` that fails to resolve to a
            // player proxy fizzles to the empty group: a card-authoring
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
            }
            // Grammar-valid but not yet wired: there is no `noted` object-set on
            // the frame to select among. An explicit named arm (not a catch-all)
            // so a future `Selection` variant is a compile error here rather than
            // a silent runtime panic.
            // [CR#607.2a]: the fact-backed product group — the members the
            // noting clause ACTUALLY moved, read through their post-move
            // identities ("cards milled this way"); a member that has since
            // left (a ceased token) drops out of the live read.
            Selection::AmongNoted(label, quantity) => {
                let live = self.live_noted_members(label);
                if matches!(
                    deref_quantity(quantity),
                    deckmaste_core::Quantity::Range(None, None)
                ) {
                    // Unconstrained: the whole live group ("exile them").
                    live
                } else {
                    // [CR#608.2d]: a CONSTRAINING quantity ("exile two of them")
                    // is a choice. `binder_choice` (consulted before this arm,
                    // on the FIRST pass) surfaces a `ChooseObjects` over `live`
                    // and re-runs the binder with the picks bound in `chosen`;
                    // this arm reads them back on the re-run, clamped to the
                    // still-live group. If nothing bound them — a constrained
                    // AmongNoted evaluated OUTSIDE a binder, or the group emptied
                    // between surface and re-run — the read is the empty group
                    // (fizzle), never a panic (authoring mistakes never crash).
                    frame
                        .anaphora
                        .chosen
                        .iter()
                        .flatten()
                        .copied()
                        .filter(|id| live.contains(id))
                        .collect()
                }
            }
        }
    }

    /// The object(s) a verb's [`Reference`] patient acts on — exactly one.
    /// Plurality is never the verb's: a "for each"/"all" instruction is an
    /// enclosing [`OneShotEffect::Each`]/[`OneShotEffect::Distribute`] whose
    /// body names a single reference per element ([CR#608.2]). A 1-element
    /// vector so the verb arms keep their batch-shaped `.into_iter()…`
    /// bodies.
    pub(crate) fn eval_reference_set(&self, reference: &Reference, frame: &Frame) -> Vec<ObjectId> {
        vec![self.eval_reference(reference, frame)]
    }

    /// Resolve a [`Reference`] to an `ObjectId`.
    ///
    /// # Panics
    ///
    /// Panics on a `Reference` not wired for Stage 3, an unbound slot
    /// anaphor, or an `AttachHostOf`/`AttachedTo` over an
    /// attachment/host with no live link (the reference is only well-defined
    /// where the relation is established).
    pub(crate) fn eval_reference(&self, reference: &Reference, frame: &Frame) -> ObjectId {
        match reference {
            // [CR#603.10a]: for a triggered ability, `~`/`This` is the firing
            // object's last-known self (the live source may be gone); for a
            // spell frame (no snapshot) it is the live source.
            Reference::This => frame.this.as_ref().map_or(frame.source, |s| s.object),
            Reference::You => self.player(frame.controller).object,
            // [CR#102.1]: in a 2-player game the opponent is the only other
            // live player; picks the first opponent in turn order for >2.
            Reference::Opponent => {
                let opp = self.next_live_after(frame.controller);
                self.player(opp).object
            }
            // The current iteration / projection element — "it" ([CR#608.2]).
            // Bound per element by an enclosing `Each`/`Distribute` loop, and by
            // `Predicate::Where` / `Selection::Pick` while testing a candidate (the
            // role the old `Subject` named). Kind-poly ([CR#120.3]): a card/token
            // element resolves to its (last-known) id, a player element to its
            // proxy. Referenced at a frameless position it is a malformed read.
            Reference::It => {
                if let Some(binding) = frame.anaphora.it.as_ref() {
                    return match binding {
                        // [CR#400.7j]: chase the same-resolution move record —
                        // the bound object may have been reminted by THIS
                        // effect's moves.
                        crate::stack::ItBinding::Object(snap) => self.chase_moved(snap.object),
                        crate::stack::ItBinding::Player(p) => self.player(*p).object,
                    };
                }
                // The slot-bound read: outside every loop binder, a lone
                // announced target is `It`'s unique antecedent — the runtime
                // twin of the Idris model's R1 resolution, sound because the
                // R2 gate refused any second candidate. The guard on
                // the other singular bindings keeps this from ever guessing:
                // a frame carrying an event role or a `With` choice can't
                // take the fallback (such a read would be ambiguous —
                // unrepresentable in the Idris model — so it can't reach here).
                let no_other_singular = frame.anaphora.that.is_none()
                    && frame.anaphora.that_object.is_none()
                    && frame.anaphora.that_patient.is_none()
                    && frame.anaphora.that_player.is_none();
                // The lone-target antecedent is a SINGLE target across all
                // slots (one quantity-one slot with one member).
                let flat: Vec<ObjectId> =
                    frame.anaphora.targets.iter().flatten().copied().collect();
                if no_other_singular && flat.len() == 1 {
                    // [CR#400.7j]: "exile target creature, … return IT" — the
                    // lone-target antecedent chases to the object it became.
                    return self.chase_moved(flat[0]);
                }
                Self::unbound_ref(
                    reference,
                    "It outside an Each/Distribute/Where/Pick element and no lone announced target",
                )
            }
            // The single object bound by an enclosing `OneShotEffect::With`/cost
            // `With` one-binder (`TheRef`/`ChooseOne`) — the choice made BEFORE
            // the verb, so the verb reads an already-bound reference
            // ([CR#608.2]). Reads the `(One, k)` `that` slot. A many-binder's
            // group has NO singular read — it is read as `Selection::That` and
            // iterated with `Each` — so this never silently takes the first of
            // many. Panics outside an enclosing one-binder `With` — always a bug.
            Reference::That(_) => {
                // The sort is verified by the Idris re-emit gate; at runtime
                // the frame's binding is the value. A PRODUCT-sited
                // `That(Sort)` (the exile-and-return chain, [CR#400.7j]) has
                // no frame binding — it reads the newest object THIS
                // resolution moved to a public zone; a product that has since
                // left is NOT found ([CR#400.7]) and the read no-ops.
                let Some(that) = frame.anaphora.that.as_ref() else {
                    if let Some(product) = self.newest_move_product() {
                        return product;
                    }
                    return Self::unbound_ref(
                        reference,
                        "That(Sort) with no enclosing With binding and no live \
                         same-resolution move product",
                    );
                };
                if that.cardinality != crate::stack::Cardinality::One {
                    return Self::unbound_ref(
                        reference,
                        "That(Sort) bound to a group — read as They and iterate with Each",
                    );
                }
                // [CR#400.7j]: chase the same-resolution move record — the
                // bound object may have been reminted by THIS effect's moves.
                that.group.first().map_or_else(
                    || Self::unbound_ref(reference, "That(Sort) bound to an empty group"),
                    |&id| self.chase_moved(id),
                )
            }
            // The nth announced target SLOT ([CR#115.3,601.2c]) read as a
            // single object — its first still-live member (a quantity-one slot
            // has exactly one; a departed member is skipped, partial fizzle
            // [CR#608.2b], falling back to the first so downstream appliers
            // no-op on it). A plural slot's full set is read via `They`.
            Reference::Target(n) => frame
                .anaphora
                .targets
                .get(*n)
                .and_then(|slot| {
                    slot.iter()
                        .find(|&&t| self.objects.get(t).is_some())
                        .or_else(|| slot.first())
                        .copied()
                })
                .unwrap_or_else(|| {
                    Self::unbound_ref(reference, "announced target index out of range")
                }),
            // [CR#603.10a,603.2e,608.2k]: the trigger's provenance-explicit
            // roles, read from the bindings the fired trigger carried. The
            // event OBJECT (the moved/acting object) — `EventObject`.
            Reference::EventObject => {
                // The OBJECT is normally the bound LKI snapshot. A replacement
                // recipient that is a player proxy carries no snapshot
                // ([CR#120.3]: the patient is kind-poly and a player is
                // zoneless), so when `that_object` is unset it falls back to the
                // patient — leaving existing snapshot-bearing reads unchanged.
                match &frame.anaphora.that_object {
                    Some(s) => s.object,
                    None => match frame.anaphora.that_patient.as_ref() {
                        Some(crate::trigger::EventPatient::Object(s)) => s.object,
                        Some(crate::trigger::EventPatient::Player(p)) => self.player(*p).object,
                        None => Self::unbound_ref(reference, "EventObject outside a trigger"),
                    },
                }
            }
            // The event ACTOR (the responsible player) — `EventActor`.
            Reference::EventActor => match frame.anaphora.that_player {
                Some(p) => self.player(p).object,
                None => Self::unbound_ref(reference, "EventActor outside a trigger"),
            },
            // [CR#608.2k,120.3]: the PATIENT (the acted-upon thing) —
            // kind-poly, an object or a player proxy.
            Reference::EventPatient => match frame.anaphora.that_patient.as_ref() {
                Some(crate::trigger::EventPatient::Object(s)) => s.object,
                Some(crate::trigger::EventPatient::Player(p)) => self.player(*p).object,
                None => Self::unbound_ref(reference, "EventPatient outside a trigger"),
            },
            // [CR#506.2,508.5]: the combat DEFENDING player — always a player.
            Reference::DefendingPlayer => match frame.defending_player {
                Some(p) => self.player(p).object,
                None => Self::unbound_ref(reference, "DefendingPlayer outside combat"),
            },
            // [CR#109.5]: the derived controller of a referenced object.
            Reference::ControllerOf(inner) => {
                let id = self.eval_reference(inner, frame);
                if id.is_null() {
                    return id;
                }
                self.player(self.layers().controller(id)).object
            }
            // [CR#108.3]: the owner of a referenced (card-backed) object.
            Reference::OwnerOf(inner) => {
                let id = self.eval_reference(inner, frame);
                if id.is_null() {
                    return id;
                }
                self.player(self.owner_of(id)).object
            }
            // Look through a remembered macro invocation.
            Reference::Expanded(e) => self.eval_reference(&e.value, frame),
            // engine-resolve-selections follow-ups: these need stores that do
            // not exist yet — an authored read fizzles until then.
            Reference::Bound(_) => {
                Self::unbound_ref(reference, "Bound(...) named-role binding store not wired")
            }
            Reference::Linked(_) => {
                Self::unbound_ref(reference, "Linked(...) linked-ability store not wired")
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
            // [CR#120.3]: a set-valued deal-time binding read only inside
            // `Is(Source, …)` (which handles it before ever reaching here);
            // there is no single live object to resolve it to, so a stray use
            // fizzles (never crashes).
            Reference::Source => Self::unbound_ref(
                reference,
                "Source is only meaningful inside Matches(Source, …)",
            ),
        }
    }

    /// Card-authoring mistakes must never crash the engine
    /// ([[engine-never-crashes-on-authoring-mistakes]]): an unresolvable
    /// authored reference degrades to the null [`ObjectId`] — a slotmap key
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
}
