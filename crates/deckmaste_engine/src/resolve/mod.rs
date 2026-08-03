//! Resolution ([CR#608]): dispatch a stack object, and walk its `OneShotEffect`
//! AST as reified agenda work. Stage 3 wires the corpus's arms; the rest are
//! `todo!`.

use std::sync::Arc;

use deckmaste_core::Ability;
use deckmaste_core::Agency;
use deckmaste_core::OneShotEffect;
use deckmaste_core::TargetSpec;
use deckmaste_core::Zone;

use crate::agenda::WorkItem;
use crate::event::AbilityCountered;
use crate::event::Cause;
use crate::event::GameEvent;
use crate::event::Occurrence;
use crate::event::ZoneChange;
use crate::object::ObjectId;
use crate::object::ObjectSource;
use crate::stack::Anaphora;
use crate::stack::Frame;
use crate::stack::StackObject;
use crate::state::GameState;

mod action;
mod count;
mod effect;
mod player_action;
mod query;
mod targets;

// The mill apply re-derives its top-slice group off the stored body facet.
pub(crate) use action::composite_body_group;
// Target-set machinery moved to `targets`; external callers keep the old
// `crate::resolve::…` paths.
pub(crate) use targets::announce_satisfiable;
pub(crate) use targets::distinct_siblings;
pub(crate) use targets::slot_count_bounds;
pub(crate) use targets::target_spec_filter;
pub(crate) use targets::validate_target_set;

impl GameState {
    /// [CR#608]: resolve the committed stack entry whose `id` is `id`. Schedules
    /// the work and the trailing cleanup event.
    ///
    /// Keyed on `StackEntry.id` (not the backing object) so it resolves both
    /// spells and triggered abilities (which have no backing object).
    ///
    /// # Panics
    ///
    /// Panics if no entry has that id — engine invariant, not caller input.
    #[expect(
        clippy::too_many_lines,
        reason = "single resolution dispatch kept as one match over stack-entry kinds; splitting would scatter the cohesive per-kind arms"
    )]
    pub(crate) fn resolve_object(&mut self, id: ObjectId) {
        let entry = self
            .stack
            .iter()
            .find(|e| e.id == id)
            .expect("entry on stack")
            .clone();
        // A fresh resolution has no amount fixed yet — `Count::ThatMuch` may
        // only read an amount an earlier instruction of THIS resolution fixed,
        // never one leaking in from combat or a prior resolution — EXCEPT for
        // a triggered ability, whose firing event's magnitude seeds the
        // register ("whenever you gain life, … that much"); a later
        // amount-carrying apply of this resolution still re-fixes it.
        self.that_much = match &entry.object {
            StackObject::Triggered { bindings, .. } => bindings.that_much,
            _ => None,
        };
        // [CR#400.7j] is scoped to ONE effect: a fresh resolution starts with
        // an empty move record, so a later effect can never find (or follow)
        // an earlier effect's moves.
        self.moved_chain.clear();
        // [CR#603.12]: a reflexive trigger looks back only over events of the
        // resolution that CREATES it, so the window resets each resolution.
        self.resolution_events.clear();
        // The aggregate finalization ledger has the same one-resolution
        // lifetime: no contained action from an earlier stack entry may
        // satisfy a later aggregate's watcher.
        self.resolution_contained_act_commits.clear();
        self.resolution_contained_act_serial = 0;
        // [CR#608.2c]: resolution note slots exist only within the resolving
        // entry's instruction sequence. This fresh-resolution boundary is the
        // one canonical clear point for ALL resolution-scoped registers (see
        // `moved_chain`/`resolution_events` above): clearing here — before any
        // of this resolution's work items run — is equivalent to and simpler
        // than clearing at the previous entry's completion, because no note
        // READER (`Count::Noted`/`AmongNoted`) ever runs outside a resolution,
        // so a note can never be observed after its own resolution ends and
        // before the next begins. A value that must OUTLIVE resolution is a
        // linked ability ([CR#607]) or an as-enters choice — a separate store.
        self.resolution_notes.clear();
        match &entry.object {
            StackObject::Spell(spell) => {
                let spell = *spell;
                if entry.copy && self.is_permanent_spell(spell) {
                    // [CR#707.10f]: a resolving permanent-spell copy should
                    // become a token permanent; until that support lands
                    // (follow-up ticket), it vanishes like any other copy
                    // ([CR#707.10a]).
                    self.schedule_front(vec![WorkItem::Emit(Occurrence::single(
                        GameEvent::AbilityResolved(spell),
                    ))]);
                } else if self.is_permanent_spell(spell) {
                    // [CR#608.3]: a permanent spell enters the battlefield.
                    // Host resolution by entry context (spec §4, [CR#303.4]): a
                    // permanent SPELL that enters attached (the Enchant
                    // `AsEnters`) attaches to its resolving spell's CHOSEN TARGET
                    // — the Aura's enchant target — not an arbitrary candidate.
                    // Carry that host in the `EnterStatus`; `apply_zone_will_change`
                    // prefers it over the candidate-set fallback (`.or`).
                    // The Aura's enchant target is slot 0's single member.
                    let enters = if self.enters_attached_self(self.objects.obj(spell).source)
                        && let Some(&host) = entry.targets.first().and_then(|slot| slot.first())
                    {
                        Some(crate::event::EnterStatus {
                            attach_to: Some(host),
                            ..crate::event::EnterStatus::default()
                        })
                    } else {
                        None
                    };
                    self.schedule_front(vec![WorkItem::Emit(Occurrence::single(
                        GameEvent::ZoneChange(ZoneChange {
                            snapshot: None,
                            object: spell,
                            from: Some(Zone::Stack),
                            to: Zone::Battlefield,
                            enters,
                            position: None,
                            face: None,
                            cause: None,
                        }),
                    ))]);
                } else if self.targets_still_legal(&entry) {
                    // Instant/sorcery with all targets still legal: run its effect.
                    let frame = Frame {
                        source: spell,
                        controller: entry.controller,
                        this: None,
                        defending_player: None,
                        payment: None,
                        anaphora: Anaphora {
                            targets: entry.targets.clone(),
                            x: entry.x,
                            ..Anaphora::empty()
                        },
                    };
                    let effect = self
                        .spell_effect(spell)
                        .expect("an instant/sorcery has a Spell ability");
                    let leave = if entry.copy {
                        // [CR#707.10a]: a copy leaves the stack by CEASING to exist
                        // — no zone move, no card. Same shape as a triggered
                        // ability vanishing ([CR#608.2n]).
                        GameEvent::AbilityResolved(spell)
                    } else {
                        GameEvent::ZoneChange(ZoneChange {
                            snapshot: None,
                            object: spell,
                            from: Some(Zone::Stack),
                            to: Zone::Graveyard,
                            enters: None,
                            position: None,
                            face: None,
                            cause: None,
                        })
                    };
                    self.schedule_front(vec![
                        WorkItem::RunEffect {
                            effect: Arc::new(effect),
                            frame,
                        },
                        WorkItem::Emit(Occurrence::single(leave)),
                    ]);
                } else if entry.copy {
                    // [CR#707.10a]: a copy leaves the stack by CEASING to exist
                    // — no zone move, no card. Same shape as a triggered
                    // ability vanishing ([CR#608.2n]).
                    self.schedule_front(vec![WorkItem::Emit(Occurrence::single(
                        GameEvent::AbilityCountered(AbilityCountered {
                            id: spell,
                            cause: Cause::counter(Agency::StateBasedAction, None),
                        }),
                    ))]);
                } else {
                    // [CR#608.2b]: all targets illegal — the spell fizzles.
                    self.schedule_front(vec![WorkItem::Emit(Occurrence::single(
                        GameEvent::ZoneChange(ZoneChange {
                            snapshot: None,
                            object: spell,
                            from: Some(Zone::Stack),
                            to: Zone::Graveyard,
                            enters: None,
                            position: None,
                            face: None,
                            cause: Some(Cause::counter(Agency::StateBasedAction, None)),
                        }),
                    ))]);
                }
            }
            // [CR#608.2n]: a triggered ability resolves its effect, then vanishes
            // — no zone move, the source untouched. The minted stack id is just
            // discarded when `AbilityResolved` removes the entry.
            StackObject::Triggered {
                source,
                ability,
                created,
                bindings,
            } => {
                // A delayed/reflexive trigger carries its body by value
                // ([CR#603.7,603.12]); a printed one is read by index — its
                // text may have changed under layers, so re-derive it fresh.
                // The index read uses `.get` (never `[]`): a source that left the
                // battlefield and reminted shorter — or whose current face is
                // shorter than the fired index — yields `None`, and the trigger
                // FIZZLES (no-op, vanishes) rather than panicking, mirroring the
                // intervening-if / illegal-target vanish below and
                // `step::trigger`'s graceful `.get`. (Back-face-sourced printed
                // triggers are captured by value at fire time, so they never
                // reach this fallback; this guards the residual gone/short cases.)
                let t = match created {
                    Some(t) => t.as_ref().clone(),
                    None => {
                        if let Some(Ability::Triggered(t)) =
                            crate::derive::abilities_of_source(self, *source).get(*ability)
                        {
                            t.as_ref().clone()
                        } else {
                            self.schedule_front(vec![WorkItem::Emit(Occurrence::single(
                                GameEvent::AbilityResolved(entry.id),
                            ))]);
                            return;
                        }
                    }
                };
                let frame = Frame {
                    // [CR#608.2,603.10a]: `~`/`This` is the firing object's
                    // last-known self; the live source may be gone.
                    source: bindings.this.as_ref().map_or(entry.id, |s| s.object),
                    controller: entry.controller,
                    // Exophoric: the firing object's LKI and the combat defender.
                    this: bindings.this.clone(),
                    defending_player: bindings.defending_player,
                    payment: None,
                    // Endophoric: the targets plus the event's bound roles.
                    anaphora: Anaphora {
                        targets: entry.targets.clone(),
                        // "where X is …" rides the resolving ability's text
                        // and is evaluated at RESOLUTION ([CR#702.21b]).
                        where_x: t.where_x.clone(),
                        that_object: bindings.that_object.clone(),
                        that_player: bindings.that_player,
                        that_patient: bindings.that_patient.clone(),
                        crossed: bindings.crossed,
                        ..Anaphora::empty()
                    },
                };
                // [CR#603.4]: an intervening-if is rechecked as the ability
                // resolves. If it no longer holds, the ability is removed from
                // the stack and does nothing (the rule mirrors the illegal-target
                // fizzle) — schedule only the `AbilityResolved` that discards the
                // entry, never the effect.
                if t.condition
                    .as_ref()
                    .is_some_and(|c| !self.condition_holds(c, &frame))
                {
                    self.schedule_front(vec![WorkItem::Emit(Occurrence::single(
                        GameEvent::AbilityResolved(entry.id),
                    ))]);
                } else if self.targets_still_legal(&entry) {
                    self.schedule_front(vec![
                        WorkItem::RunEffect {
                            effect: Arc::new(t.effect),
                            frame,
                        },
                        WorkItem::Emit(Occurrence::single(GameEvent::AbilityResolved(entry.id))),
                    ]);
                } else {
                    // [CR#608.2b]: every target illegal — fizzle, vanish.
                    self.schedule_front(vec![WorkItem::Emit(Occurrence::single(
                        GameEvent::AbilityCountered(AbilityCountered {
                            id: entry.id,
                            cause: Cause::counter(Agency::StateBasedAction, None),
                        }),
                    ))]);
                }
            }
            // [CR#602.2a]: an activated ability resolves its carried text,
            // then vanishes like a trigger — no zone move.
            StackObject::Activated {
                ability, bindings, ..
            } => {
                if self.targets_still_legal(&entry) {
                    let this = bindings
                        .this
                        .as_ref()
                        .expect("begin_activate captures the source snapshot unconditionally");
                    let frame = Frame {
                        // [CR#608.2]: `~` is the source's announce-time
                        // snapshot; the live object may be gone.
                        source: this.object,
                        controller: entry.controller,
                        // Exophoric: the source snapshot and combat defender.
                        this: Some(this.clone()),
                        defending_player: bindings.defending_player,
                        payment: None,
                        // Endophoric: targets, announced X, and event roles.
                        anaphora: Anaphora {
                            targets: entry.targets.clone(),
                            x: entry.x,
                            that_object: bindings.that_object.clone(),
                            that_player: bindings.that_player,
                            that_patient: bindings.that_patient.clone(),
                            ..Anaphora::empty()
                        },
                    };
                    self.schedule_front(vec![
                        WorkItem::RunEffect {
                            effect: Arc::new(ability.effect.clone()),
                            frame,
                        },
                        WorkItem::Emit(Occurrence::single(GameEvent::AbilityResolved(entry.id))),
                    ]);
                } else {
                    // [CR#608.2b]: every target illegal — fizzle, vanish.
                    self.schedule_front(vec![WorkItem::Emit(Occurrence::single(
                        GameEvent::AbilityCountered(AbilityCountered {
                            id: entry.id,
                            cause: Cause::counter(Agency::StateBasedAction, None),
                        }),
                    ))]);
                }
            }
        }
    }

    /// The [`ItBinding`] for one element — the iteration/projection anaphor
    /// ([CR#608.2]), kind-poly over the element's source ([CR#120.3]). A
    /// card/token captures its LKI snapshot (reads survive its removal); a
    /// player proxy is zoneless, so it binds as the bare player id.
    ///
    /// [`ItBinding`]: crate::stack::ItBinding
    pub(crate) fn it_binding(&self, obj: ObjectId) -> crate::stack::ItBinding {
        match self.objects.obj(obj).source {
            ObjectSource::Player(p) => crate::stack::ItBinding::Player(p),
            ObjectSource::Card(_) => {
                crate::stack::ItBinding::Object(crate::lki::LkiSnapshot::capture(self, obj))
            }
        }
    }

    /// Follow the resolution-scoped move record ([CR#400.7j]) to the id the
    /// object became: `a→b→c` chases `a` to `c`. Identity on unrecorded ids.
    /// Remint never reuses a live id, so the walk terminates.
    ///
    /// SEAM (engine-delayed-reflexive-triggers): a delayed trigger created
    /// mid-resolution captures bound roles as ids; that registry must
    /// FINALIZE its captured ids via this chase before the resolution ends
    /// (the record clears when the next resolution begins).
    pub(crate) fn chase_moved(&self, id: ObjectId) -> ObjectId {
        let mut cur = id;
        while let Some(&(_, new)) = self.moved_chain.iter().find(|&&(old, _)| old == cur) {
            cur = new;
        }
        cur
    }

    /// The `ObjectSource` that anchors `Ref(This)`/`Ref(You)` in live filter
    /// evaluation for `frame`: the announce-time snapshot's source when the
    /// frame carries bindings (the live object may be gone, [CR#603.10a]),
    /// else the live source object's.
    pub(crate) fn frame_watcher(&self, frame: &Frame) -> crate::object::ObjectSource {
        frame
            .this
            .as_ref()
            .map_or_else(|| self.objects.obj(frame.source).source, |s| s.source)
    }

    /// True iff any of the card's printed types is a PERMANENT type
    /// ([CR#608.3] — a spell of this type enters the battlefield on
    /// resolution instead of resolving as a one-shot effect). Reads the
    /// `TypeDef.permanent` flag, not an enum name-list. Grizzly Bears →
    /// true; Instant `DealDamage` `AnyTarget` → false. The
    /// battlefield-entry-vs-resolve control-flow fork this feeds
    /// (`resolve.rs:93`) stays hardcoded.
    #[must_use]
    pub(crate) fn is_permanent_spell(&self, id: ObjectId) -> bool {
        crate::derive::face(self.def(id))
            .types
            .iter()
            .any(|t| t.permanent)
    }

    /// Returns the effect of the spell's first `Ability::Spell(SpellAbility {
    /// effect, .. })`, cloned. Looks through `Ability::Expanded` the way
    /// `derive::tap_mana_ability` does. Returns `None` if there is no Spell
    /// ability.
    #[must_use]
    pub(crate) fn spell_effect(&self, id: ObjectId) -> Option<OneShotEffect> {
        crate::derive::abilities(self, id)
            .iter()
            .find_map(|a| spell_ability_effect(a))
            .cloned()
    }
}

/// The `SpellAbility.targets` of the spell (empty for permanent spells).
/// Used by the cast checks and `targets_still_legal`. Reads the caller's
/// derived view — the legality loop checks every hand card against one
/// view instead of re-deriving the board per card.
#[must_use]
pub(crate) fn spell_targets(view: &crate::layer::LayeredView, id: ObjectId) -> Vec<TargetSpec> {
    // Targets live on a top-level `OneShotEffect::Targeted` wrapper in the spell
    // ability's effect ([CR#115.1,601.2c]).
    view.get(id)
        .abilities
        .iter()
        .find_map(spell_ability_effect)
        .map_or_else(Vec::new, |e| top_targets(e).to_vec())
}

/// Extracts the `OneShotEffect` from the first `Ability::Spell` arm, looking
/// through `Ability::Expanded`.
fn spell_ability_effect(ability: &Ability) -> Option<&OneShotEffect> {
    match ability {
        Ability::Spell(s) => Some(&s.effect),
        _ => None,
    }
}

/// Look through `Quantity` macro expansions (`Exactly`, `AtLeast`, …) to
/// the underlying `Range` primitive.
fn deref_quantity(q: &deckmaste_core::Quantity) -> &deckmaste_core::Quantity {
    match q {
        range @ deckmaste_core::Quantity::Range(..) => range,
        // Provenance is erased at `lower` (`deckmaste_lowering`), so no
        // loaded value reaches here wrapped. The arm survives only because
        // the variant does; `core-demacro` deletes both.
        deckmaste_core::Quantity::Expanded(_) => unreachable!("provenance erased at lower"),
    }
}

/// The targets declared on a top-level `Targeted` wrapper (peeling
/// `Expanded`), or `&[]` when the effect isn't a wrapper — the announce-list
/// home after the migration ([CR#115.1,601.2c]). A single top-level wrapper is
/// the only shape today; a nested wrapper would need a per-scope target stack.
pub(crate) fn top_targets(effect: &OneShotEffect) -> &[TargetSpec] {
    match effect {
        OneShotEffect::Targeted(te) => &te.targets,
        _ => &[],
    }
}

/// One event → `Single`; several → a simultaneous `Batch`.
fn occurrence_of(mut events: Vec<GameEvent>) -> crate::event::Occurrence {
    use crate::event::Occurrence;
    if events.len() == 1 {
        Occurrence::Single(events.pop().expect("len 1"))
    } else {
        Occurrence::Batch(events)
    }
}

#[cfg(test)]
mod fixtures;

#[cfg(test)]
mod tests {

    use std::sync::Arc;

    use deckmaste_card::Card;
    use deckmaste_card::CardFace;
    use deckmaste_core::Action;
    use deckmaste_core::Count;
    use deckmaste_core::OneShotEffect;
    use deckmaste_core::Predicate;
    use deckmaste_core::Reference;
    use deckmaste_core::Type;
    use deckmaste_core::Zone;

    use crate::object::ObjectId;
    use crate::object::ObjectSource;
    use crate::player::PlayerId;
    use crate::resolve::fixtures::*;
    use crate::stack::RefKind;
    use crate::stack::StackEntry;
    use crate::stack::StackObject;
    use crate::state::GameState;

    /// `chase_moved` follows the resolution-scoped old→new move record
    /// transitively ([CR#400.7j]) and is the identity on unrecorded ids.
    #[test]
    fn chase_moved_follows_chain_transitively() {
        let (mut state, a, b) = two_permanents_on_field();
        // Unrecorded: identity.
        assert_eq!(state.chase_moved(a), a);
        // a -> b -> a2. The third id MUST come from the SAME state: a fresh
        // GameState mints in the same order, so its ids numerically collide
        // with this state's and a cross-state id would close the chain into a
        // cycle — chase_moved's termination leans on the within-state
        // ids-never-repeat invariant.
        let a2 = state.objects.mint(
            crate::object::ObjectSource::Player(PlayerId(1)),
            PlayerId(1),
            Some(Zone::Battlefield),
        );
        state.moved_chain.push((a, b));
        assert_eq!(state.chase_moved(a), b, "one hop");
        state.moved_chain.push((b, a2));
        assert_eq!(state.chase_moved(a), a2, "two hops");
        assert_eq!(state.chase_moved(b), a2, "mid-chain entry");
    }

    /// A fresh resolution clears the record ([CR#400.7j] is per-effect): ids
    /// recorded by resolution 1 must not chase in resolution 2 (Ephemerate
    /// safety at the mechanism level).
    #[test]
    fn moved_chain_resets_when_a_resolution_begins() {
        let (mut state, a, b) = two_permanents_on_field();
        state.moved_chain.push((a, b));

        // Drive a REAL resolution: mint a vanilla creature spell object and
        // push its `StackEntry` by hand, then resolve it.
        let card = Card::Normal(CardFace {
            name: "Test Bear".into(),
            types: vec![Type::Creature.def()],
            power: Some(deckmaste_core::StatValue::Number(2)),
            toughness: Some(deckmaste_core::StatValue::Number(2)),
            ..CardFace::default()
        });
        let cid = state.cards.push(Arc::new(card), PlayerId(0));
        let spell = state
            .objects
            .mint(ObjectSource::Card(cid), PlayerId(0), Some(Zone::Stack));
        state.stack.push(StackEntry {
            id: spell,
            object: StackObject::Spell(spell),
            controller: PlayerId(0),
            targets: vec![],
            x: None,
            paid_costs: vec![],
            copy: false,
        });

        state.resolve_object(spell);

        assert!(
            state.moved_chain.is_empty(),
            "a fresh resolution clears the move record ([CR#400.7j] is per-effect)"
        );
    }

    /// `top_targets` reads the targets off a top-level `Targeted` (peeling
    /// `Expanded`) and returns empty for a non-wrapper effect
    /// ([CR#115.1,601.2c]).
    #[test]
    fn top_targets_reads_wrapper_and_peels_expanded() {
        let spec = deckmaste_core::TargetSpec::Target(
            deckmaste_core::Quantity::one(),
            Predicate::creature(),
        );
        let wrapped = OneShotEffect::Targeted(deckmaste_core::Targeted::new(
            vec![spec.clone()].into(),
            OneShotEffect::Act(Action::deal_damage(Reference::It, Count::Literal(3))),
        ));
        assert_eq!(super::top_targets(&wrapped), std::slice::from_ref(&spec));
        let bare = OneShotEffect::Act(Action::deal_damage(Reference::It, Count::Literal(1)));
        assert!(super::top_targets(&bare).is_empty());
    }

    /// Ticket: the iteration anaphor's KIND ([CR#120.3]) tracks the element's
    /// source — a card/token element binds as an object (carrying its LKI
    /// snapshot, so reads survive its removal), a player proxy binds as a
    /// player (zoneless, no snapshot). The kind a `With`/`Each`/`Distribute`
    /// element exposes through `It`.
    #[test]
    fn it_binding_kind_distinguishes_player_and_object() {
        use crate::stack::ItBinding;

        let (state, creature) = bear_on_field();
        let player = state.player(PlayerId(0)).object;

        let obj = state.it_binding(creature);
        assert_eq!(
            obj.kind(),
            RefKind::Object,
            "a creature element is an object"
        );
        assert!(
            matches!(obj, ItBinding::Object(_)),
            "an object element carries its LKI snapshot"
        );

        let ply = state.it_binding(player);
        assert_eq!(
            ply.kind(),
            RefKind::Player,
            "a player proxy element is a player"
        );
        assert!(
            matches!(ply, ItBinding::Player(_)),
            "a player element is zoneless — no snapshot"
        );
    }

    /// Mints a stack-zone spell object (player 0) whose printed face carries
    /// exactly `types`, for `is_permanent_spell` fixtures.
    fn spell_with_types(state: &mut GameState, types: Vec<deckmaste_core::TypeDef>) -> ObjectId {
        let card = Card::Normal(CardFace {
            name: "Test Spell".into(),
            types,
            ..CardFace::default()
        });
        let card_id = state.cards.push(Arc::new(card), PlayerId(0));
        state
            .objects
            .mint(ObjectSource::Card(card_id), PlayerId(0), Some(Zone::Stack))
    }

    /// [CR#608.3]: a permanent spell enters the battlefield on resolution.
    /// Baseline: a creature spell (Grizzly-Bears-shaped) is permanent; an
    /// instant spell (bolt-shaped) is not. `Type::def()` carries the correct
    /// `permanent` flag, so no plugin load is needed.
    #[test]
    fn is_permanent_spell_reads_the_permanent_flag() {
        let mut state = game();
        let bear = spell_with_types(&mut state, vec![Type::Creature.def()]);
        assert!(
            state.is_permanent_spell(bear),
            "a creature spell is permanent"
        );
        let bolt = spell_with_types(&mut state, vec![Type::Instant.def()]);
        assert!(
            !state.is_permanent_spell(bolt),
            "an instant spell is not permanent"
        );
    }

    /// The real proof `is_permanent_spell` reads `TypeDef.permanent` and not
    /// an enum name-list: a NON-canonical type name ("Contraption" — not one
    /// of `Type`'s ten variants, so no hardcoded name-list could ever
    /// recognize it) flagged `permanent: true` is treated as permanent. And
    /// the converse pins it isn't secretly still keying off the name: a
    /// `TypeDef` named "Land" — a name a hardcoded list WOULD recognize —
    /// but flagged `permanent: false` is NOT treated as permanent.
    #[test]
    fn is_permanent_spell_follows_the_flag_not_a_hardcoded_name_list() {
        let mut state = game();
        let contraption = deckmaste_core::TypeDef {
            name: "Contraption".into(),
            permanent: true,
            confers: vec![].into(),
        };
        let novel = spell_with_types(&mut state, vec![contraption]);
        assert!(
            state.is_permanent_spell(novel),
            "a novel type name flagged permanent:true is permanent — no hardcoded \
             name-list could ever recognize \"Contraption\""
        );

        let fake_land = deckmaste_core::TypeDef {
            name: "Land".into(),
            permanent: false,
            confers: vec![].into(),
        };
        let non_permanent_land = spell_with_types(&mut state, vec![fake_land]);
        assert!(
            !state.is_permanent_spell(non_permanent_land),
            "a TypeDef named \"Land\" but flagged permanent:false is NOT permanent — \
             the old hardcoded name-list would have said true for this name alone"
        );
    }

    // ========================================================================
    // P0.W3: uncalled coin flips, dice rolls, random discard — the work-item
    // machinery drawing from the seeded rng (`engine-randomness` Task 3).
    // ========================================================================
}
