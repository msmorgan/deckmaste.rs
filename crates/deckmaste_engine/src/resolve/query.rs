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
    /// non-player reference is a semantic-input error that fizzles to 0.
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
    /// choose as many as able when fewer exist ([CR#608.2d]). The test-only
    /// `can_pay_verbs` helper also uses it to read a selection's required
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

    /// The nth announced target register's live members, in announce order.
    /// Departed (illegal) targets are
    /// excluded ([CR#608.2b] partial fizzle; mirrors `StatePredicate::Targets`
    /// ignoring a gone target), so a wholly-departed slot reads empty and its
    /// verb no-ops. An out-of-range index reads empty (never-crash).
    fn live_target_slot(&self, frame: &Frame, n: usize) -> Vec<ObjectId> {
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
    fn could_target_set(&self, spell: &Reference, frame: &Frame) -> Option<Vec<ObjectId>> {
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
    #[expect(
        clippy::too_many_lines,
        reason = "the flat one-arm-per-Selection-variant dispatch clippy.toml's threshold note \
                  blesses: the bulk is per-variant CR documentation, and carving arms out would \
                  scatter the taxonomy across helpers that each have exactly one caller. The one \
                  genuinely multi-step arm is already extracted to `could_target_set`."
    )]
    pub(crate) fn eval_selection_set(&self, sel: &Selection, frame: &Frame) -> Vec<ObjectId> {
        match sel {
            Selection::Reg(reference) => {
                let values = self.activation_objects(frame.activation, *reference);
                if values.is_empty() && reference.0 >= 6 {
                    self.live_target_slot(frame, (reference.0 - 6) as usize)
                } else {
                    values
                }
            }
            // Thread the carrier (like `Pick`) so a carrier-relative predicate
            // resolves rather than panicking frameless: "each opponent" =
            // `SelectAll(OpponentOf(Ref(You)))` reads `Ref(You)` off the watcher.
            Selection::SelectAll(f) => crate::target::candidates_with_activation(
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
                        crate::target::candidates_with_activation(
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
            // The RNG's picks, bound into the frame before the selection is
            // read ([CR#608.2d]): `Each`/`With`/`Distribute`'s handling
            // (`resolve/effect.rs`) samples an `Existing(Random(..))` binder
            // via the seeded rng and binds `frame.anaphora.chosen` BEFORE
            // this evaluator ever runs — the discard-at-random composite
            // ([CR#701.9b]) is the first live consumer.
            Selection::Random(..) => frame
                .anaphora
                .chosen
                .clone()
                .expect("a Random selection is bound into the frame before it is read"),
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
            // The ordered plural group bound by the enclosing many-binder
            // (`OneShotEffect::With`/`Each`/`Distribute`). Reads the `(Many, k)`
            // `that` slot, order-preserved exactly as bound (top→down for a
            // library window). A `(One, k)` binding has NO group read — the
            // singular `Reference::That` is its only reader — so a single object
            // can never be silently splayed into a group here.
            Selection::They | Selection::Them(_) => {
                // The sort is verified by the Idris re-emit gate; the frame's
                // bound group is the value. An announced target slot is NOT a
                // candidate here — a plural target is read positionally from
                // its region register. A product-sited plural read
                // (create-two-tokens … They) is [[engine-bound-references]] work.
                let Some(that) = frame.anaphora.that.as_ref() else {
                    // An unbound plural read — a bare `They` in a targeted body
                    // (rather than its region register), or the
                    // unbuilt product-sited read. A semantic-input error fizzles
                    // to the empty group; the Idris gate is what REFUSES the
                    // shape (`tBadTheyReadsNoTarget`), and nothing here crashes
                    // on a bad card.
                    return Self::unbound_group(
                        sel,
                        "They/Them with no enclosing With binding (a plural target reads a \
                         region register; a product-sited plural read is unbuilt)",
                    );
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
                    "engine seam: Selection::PilesOf ([CR#700.3a,700.3b]) — labeled pile groups \
                     have no runtime store to read back; owner: engine-piles"
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
                    // (fizzle), never a panic (semantic-input errors never crash).
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

    /// The object(s) a verb's [`Reference`] patient acts on — zero or one.
    /// Plurality is never the verb's: a "for each"/"all" instruction is an
    /// enclosing [`OneShotEffect::Each`]/[`OneShotEffect::Distribute`] whose
    /// body names a single reference per element ([CR#608.2]). A 1-element
    /// vector keeps the verb arms' batch-shaped `.into_iter()…` bodies; null
    /// and departed current-only patients become the empty set so no action
    /// event is constructed for them ([CR#608.2b]).
    pub(crate) fn eval_reference_set(&self, reference: &Reference, frame: &Frame) -> Vec<ObjectId> {
        let object = self.eval_reference(reference, frame);
        self.objects
            .get(object)
            .map_or_else(Vec::new, |_| vec![object])
    }

    /// Resolve the complete object product carried by a reference. Declared
    /// region registers read their activation-table values; residual bare
    /// rule and cost scopes read the same activation context until those
    /// scopes gain regions of their own.
    pub(crate) fn eval_reference_product(
        &self,
        reference: &Reference,
        frame: &Frame,
    ) -> crate::activation::ReferenceProduct {
        if let Reference::Reg(register) = reference {
            return self
                .activation_product(frame.activation, *register)
                .or_else(|| {
                    let provenance = match register.0 {
                        0 => deckmaste_core::Provenance::Source,
                        1 => deckmaste_core::Provenance::Controller,
                        2 => deckmaste_core::Provenance::EventObject,
                        3 => deckmaste_core::Provenance::EventPatient,
                        4 => deckmaste_core::Provenance::EventActor,
                        5 => deckmaste_core::Provenance::DefendingPlayer,
                        target => deckmaste_core::Provenance::AnnouncedTarget(target - 6),
                    };
                    self.activation_context_product(frame.activation, &provenance)
                })
                .unwrap_or(crate::activation::ReferenceProduct {
                    current: None,
                    lki: None,
                });
        }

        let current_id = self.eval_reference(reference, frame);
        let current = self.objects.get(current_id).map(|_| current_id);
        let lki = match reference {
            Reference::It => match frame.anaphora.it.as_ref() {
                Some(crate::stack::ItBinding::Object(snapshot)) => Some(snapshot.clone()),
                Some(crate::stack::ItBinding::Player(_)) | None => None,
            },
            _ => None,
        };
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
    pub(crate) fn eval_reference(&self, reference: &Reference, frame: &Frame) -> ObjectId {
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
                Self::unbound_ref(
                    reference,
                    "It outside an Each/Distribute/Where/Pick element",
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
            // Provenance is erased at `lower` (`deckmaste_lowering`), so no
            // loaded value reaches here wrapped. The arm survives only because
            // the variant does; `core-demacro` deletes both.
            // engine-resolve-selections follow-ups: these need stores that do
            // not exist yet — a semantic read fizzles until then.
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

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use deckmaste_core::Action;
    use deckmaste_core::Binder;
    use deckmaste_core::OneShotEffect;
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
    use crate::stack::Anaphora;
    use crate::stack::Frame;
    use crate::test_support::frame_for;
    use crate::test_support::frame_src;
    use crate::test_support::frame_src_targets;

    /// Semantic-input errors in card data never crash the engine (the Invalid
    /// semantic input fizzles decision,
    /// `docs/decisions/invalid-semantic-input-fizzles.md`): an unresolvable
    /// semantic reference degrades to the null object id (the
    /// effect fizzles) rather than panicking. Soundness — that a
    /// well-formed card's references always resolve — is the Idris re-emit
    /// gate's job, not a runtime panic.
    #[test]
    fn unbound_reference_degrades_to_null_not_panic() {
        use deckmaste_core::Reference;
        use slotmap::Key;

        let state = game();
        let frame = frame_for(&state, PlayerId(0));
        // `It` outside any binder — including in a targeted body, where it used
        // to fall back to the lone announced target. A target is read
        // positionally now, so this is simply an unbound read: null, no panic.
        assert!(state.eval_reference(&Reference::It, &frame).is_null());
        // ...and its plural twin: a bare `They` (the pre-positional spelling of
        // `Targets(n)`) with no binder fizzles to the EMPTY group rather than
        // reaching the product-sited `todo!()` it used to be rescued from by
        // the announced-targets fallback.
        assert!(
            state
                .eval_selection_set(&deckmaste_core::Selection::They, &frame)
                .is_empty()
        );
        // An out-of-range positional read degrades the same way, on both
        // channels — never-crash, never a wrong slot.
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
        // Event roles read outside any trigger — were `.expect()` panics.
        assert!(
            state
                .eval_reference(&Reference::Reg(deckmaste_core::RefId(2)), &frame)
                .is_null()
        );
        assert!(
            state
                .eval_reference(&Reference::Reg(deckmaste_core::RefId(4)), &frame)
                .is_null()
        );
        assert!(
            state
                .eval_reference(&Reference::Reg(deckmaste_core::RefId(5)), &frame)
                .is_null()
        );
        // A derived reference over an unbound inner stays null, not a secondary
        // panic in `layers().controller()`.
        assert!(
            state
                .eval_reference(&Reference::ControllerOf(Arc::new(Reference::It)), &frame)
                .is_null()
        );
    }

    #[test]
    fn activation_target_product_keeps_lki_after_departure() {
        use deckmaste_core::{DefId, Kind, Param, Provenance, RefId, Region};

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
                    Kind::Objects
                } else if index == 7 {
                    Kind::Number
                } else {
                    Kind::Object
                },
                provenance,
            })
            .collect::<Vec<_>>()
            .into();
        let region = Region::new(params, Arc::from([]));
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
    #[test]
    fn bound_role_reads_chase_the_move_record() {
        let (mut state, a, b) = two_permanents_on_field();
        state.moved_chain.push((a, b));

        // That(Sort) over a One binding chases a -> b.
        let mut frame = frame_src(&state, a);
        frame.anaphora.that = Some(crate::stack::ThatBinding {
            cardinality: crate::stack::Cardinality::One,
            kind: crate::stack::RefKind::Object,
            group: vec![a],
        });
        assert_eq!(
            state.eval_reference(&Reference::That(deckmaste_core::Sort::Card), &frame),
            b
        );

        // Target(n) does NOT chase.
        state.frame_set_targets(&mut frame, &[vec![a]]);
        assert_eq!(
            state.eval_reference(&Reference::Reg(deckmaste_core::RefId(6)), &frame),
            a
        );

        // The `It` Object binding chases.
        let snap = crate::lki::LkiSnapshot::capture(&state, a);
        frame.anaphora.it = Some(crate::stack::ItBinding::Object(snap));
        assert_eq!(state.eval_reference(&Reference::It, &frame), b);

        // A lone target does not implicitly bind the iteration anaphor.
        let lone = frame_src_targets(&state, a, vec![a]);
        assert!(state.eval_reference(&Reference::It, &lone).is_null());
    }

    /// A Many `that` group chases per element via `Selection::They`.
    #[test]
    fn group_read_chases_per_element() {
        let (mut state, a, b) = two_permanents_on_field();
        // `c` MUST come from the SAME state (see
        // `chase_moved_follows_chain_transitively` on cross-state id
        // collision).
        let c = state.objects.mint(
            crate::object::ObjectSource::Player(PlayerId(1)),
            PlayerId(1),
            Some(Zone::Battlefield),
        );
        state.moved_chain.push((a, c));
        let mut frame = frame_src(&state, a);
        frame.anaphora.that = Some(crate::stack::ThatBinding {
            cardinality: crate::stack::Cardinality::Many,
            kind: crate::stack::RefKind::Object,
            group: vec![a, b],
        });
        assert_eq!(
            state.eval_selection_set(&Selection::They, &frame),
            vec![c, b],
            "a chases to c; b unrecorded stays b"
        );
    }

    /// [CR#400.7,603.7c]: an object-op whose bound role resolved to a GONE id
    /// (hidden destination / stale) is a NO-OP, not a panic.
    #[test]
    fn move_of_a_gone_bound_role_is_a_noop() {
        use deckmaste_core::Destination;

        let (mut state, a, _) = two_permanents_on_field();
        let mut frame = frame_src(&state, a);
        frame.anaphora.that = Some(crate::stack::ThatBinding {
            cardinality: crate::stack::Cardinality::One,
            kind: crate::stack::RefKind::Object,
            group: vec![a],
        });
        // Kill `a` outright (no record entry — e.g. it bounced to hand).
        state.objects.remove(a);
        let items = state.move_items(
            &Reference::That(deckmaste_core::Sort::Card),
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
            "a gone bound role produces no zone-change emit",
        );
    }

    /// The product-sited `That(Sort)` ([CR#400.7j] — "exile it, then return
    /// THAT CARD"): with no `that` binding, the read resolves to the newest
    /// same-resolution move product; once that product leaves for a hidden
    /// zone it is NOT found and the read degrades to null (never an older
    /// antecedent).
    #[test]
    fn product_sited_that_reads_the_newest_live_move_product() {
        use slotmap::Key;

        let (mut state, a, _) = two_permanents_on_field();
        let frame = frame_src(&state, a);

        // Nothing moved yet: unbound.
        assert!(
            state
                .eval_reference(&Reference::That(deckmaste_core::Sort::Card), &frame)
                .is_null()
        );

        // Exile `a` through the real effect machinery: the apply records the
        // public move.
        state.run_effect(
            OneShotEffect::Act(Action::Move(
                Reference::Reg(deckmaste_core::RefId(0)),
                deckmaste_core::Destination::Zone(Zone::Exile),
                vec![].into(),
                None,
            )),
            &frame,
        );
        run_injected(&mut state);
        let product = state.chase_moved(a);
        assert_ne!(product, a, "the exile reminted a new object");
        assert_eq!(
            state.eval_reference(&Reference::That(deckmaste_core::Sort::Card), &frame),
            product,
            "the product-sited That reads the exile product"
        );

        // Send the product to a HIDDEN zone: not found, and no fallback to an
        // older antecedent.
        let pframe = frame_src(&state, product);
        state.run_effect(
            OneShotEffect::Act(Action::Move(
                Reference::Reg(deckmaste_core::RefId(0)),
                deckmaste_core::Destination::Zone(Zone::Hand),
                vec![].into(),
                None,
            )),
            &pframe,
        );
        run_injected(&mut state);
        assert!(
            state
                .eval_reference(&Reference::That(deckmaste_core::Sort::Card), &frame)
                .is_null(),
            "a product that left its public zone is NOT found ([CR#400.7])"
        );
    }

    /// `With(Produce(Move(...)), body)` binds the move's PRE-move id as a One
    /// `That`; after the move applies, the body's `That` chases to the
    /// product ([CR#400.7j]).
    #[test]
    fn with_produce_binds_the_moved_objects_product() {
        use deckmaste_core::Destination;
        use deckmaste_core::With;

        let (mut state, a, _) = two_permanents_on_field();
        let frame = frame_src(&state, a);
        state.run_effect(
            OneShotEffect::With(With {
                binder: Binder::Produce(Arc::new(Action::Move(
                    Reference::Reg(deckmaste_core::RefId(0)),
                    Destination::Zone(Zone::Exile),
                    vec![].into(),
                    None,
                ))),
                body: Arc::new(OneShotEffect::Act(Action::Move(
                    Reference::That(deckmaste_core::Sort::Card),
                    Destination::Zone(Zone::Battlefield),
                    vec![].into(),
                    None,
                ))),
            }),
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

    /// `eval_selection_set` returns the bound set for a `Random` slot (the
    /// value the RNG wrote into `frame.anaphora.chosen`), instead of
    /// surfacing. (Player choice now rides `With(ChooseOne/Choose, …)`,
    /// bound as `Those`.)
    #[test]
    fn eval_selection_set_reads_bound_choice() {
        use deckmaste_core::Quantity;

        let (state, bear) = bear_on_field();
        let creatures = Predicate::And(
            vec![
                Predicate::State(StatePredicate::InZone(Zone::Battlefield)),
                Predicate::creature(),
            ]
            .into(),
        );
        let frame = Frame {
            anaphora: Anaphora {
                chosen: Some(vec![bear]),
                ..Anaphora::empty()
            },
            ..Frame::bare(bear, PlayerId(0))
        };
        let sel = Selection::Random(Quantity::one(), creatures);
        assert_eq!(state.eval_selection_set(&sel, &frame), vec![bear]);
    }

    /// A foreign chooser routes the `ChooseObjects` decision to the binder's
    /// resolved `by` player, not the spell's controller ([CR#608.2d] — "that
    /// player sacrifices a creature of their choice", [CR#701.21a]).
    #[test]
    fn foreign_by_routes_choice_to_that_player() {
        use deckmaste_core::Binder;
        use deckmaste_core::With;

        use crate::decide::PendingDecision;
        use crate::step::StepOutcome;

        let (mut state, bear) = bear_on_field();
        let _theirs = second_bear_to_player_1(&mut state);
        let creatures = Predicate::And(
            vec![
                Predicate::State(StatePredicate::InZone(Zone::Battlefield)),
                Predicate::creature(),
            ]
            .into(),
        );
        let frame = frame_src(&state, bear);
        state.run_effect(
            OneShotEffect::With(With {
                binder: Binder::ChooseOne {
                    filter: creatures,
                    by: Reference::OpponentOf(std::sync::Arc::new(Reference::Reg(
                        deckmaste_core::RefId(1),
                    ))),
                },
                body: Arc::new(OneShotEffect::Act(Action::destroy(Reference::That(
                    deckmaste_core::Sort::Permanent,
                )))),
            }),
            &frame,
        );
        let StepOutcome::NeedsDecision(PendingDecision::ChooseObjects(
            crate::decide::pending::ChooseObjects { player, .. },
        )) = state.step()
        else {
            panic!("expected ChooseObjects, got {:?}", state.pending);
        };
        assert_eq!(
            player,
            PlayerId(1),
            "the opponent (the binder's `by`) makes the pick"
        );
    }

    /// The buildable-now references: `ControllerOf`/`OwnerOf` distinguish
    /// control from ownership ([CR#109.5,108.3]); `EventObject`/`EventActor`
    /// read the trigger bindings ([CR#603.10a]).
    #[test]
    fn references_resolve_controller_owner_and_trigger_bindings() {
        let (mut state, bear) = bear_on_field();
        // A second Grizzly Bears from player 0's hand onto the battlefield, then
        // handed to player 1: owner stays player 0, controller becomes player 1.
        let theirs = second_bear_to_player_1(&mut state);

        let mut frame = Frame::bare(bear, PlayerId(0));
        state.frame_set_source_lki(
            &mut frame,
            Some(crate::lki::LkiSnapshot::capture(&state, bear)),
        );
        state.frame_set_targets(&mut frame, &[vec![theirs]]);
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
        let only_you = Reference::Single(Arc::new(Selection::SelectAll(Predicate::Ref(
            Reference::Reg(deckmaste_core::RefId(1)),
        ))));
        assert_eq!(
            state.eval_reference(&only_you, &frame),
            state.player(PlayerId(0)).object,
            "Single resolves a singleton selection"
        );
        let ambiguous_players = Reference::Single(Arc::new(Selection::SelectAll(Predicate::Kind(
            deckmaste_core::ObjectKind::Player,
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
        let mut object_patient = Frame::bare(bear, PlayerId(0));
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
        let mut player_patient = Frame::bare(bear, PlayerId(0));
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
    #[test]
    fn library_of_reads_the_whole_library_top_to_bottom() {
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
                &Selection::LibraryOf(Reference::Reg(deckmaste_core::RefId(1))),
                &frame
            ),
            expected,
            "LibraryOf(You) is the whole library, in zone order"
        );
        assert_eq!(
            state.eval_selection_set(
                &Selection::TopOfLibrary {
                    count: deckmaste_core::Count::Literal(2),
                    whose: Reference::Reg(deckmaste_core::RefId(1)),
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
                .eval_selection_set(&Selection::LibraryOf(Reference::It), &frame)
                .is_empty(),
            "an unresolvable whose fizzles, never panics"
        );
    }

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
                targets: vec![
                    // Slot 0: creatures only.
                    TargetSpec::Target(Quantity::one(), Predicate::creature()),
                    // Slot 1: anything on the battlefield — a strict superset.
                    TargetSpec::Target(
                        Quantity::one(),
                        Predicate::State(StatePredicate::InZone(Zone::Battlefield)),
                    ),
                ]
                .into(),
                // The body is irrelevant to the read under test; any
                // slot-referencing action keeps the declaration well-formed.
                effect: OneShotEffect::Act(Action::deal_damage(
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

        // `This` names the spell itself off the frame's source.
        let frame = crate::test_support::frame_src(&state, spell);
        let mut got = state.eval_selection_set(
            &Selection::ValidTargetsFor(Reference::Reg(deckmaste_core::RefId(0))),
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
                .eval_selection_set(&Selection::ValidTargetsFor(Reference::It), &bare)
                .is_empty(),
            "ValidTargetsFor over a non-stack reference fizzles, never panics"
        );
    }
}
