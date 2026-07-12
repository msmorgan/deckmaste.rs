//! Resolution ([CR#608]): dispatch a stack object, and walk its `OneShotEffect`
//! AST as reified agenda work. Stage 3 wires the corpus's arms; the rest are
//! `todo!`.

use deckmaste_core::Ability;
use deckmaste_core::Agency;
use deckmaste_core::OneShotEffect;
use deckmaste_core::TargetSpec;
use deckmaste_core::Zone;

use crate::agenda::WorkItem;
use crate::event::Cause;
use crate::event::GameEvent;
use crate::event::Occurrence;
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
                        GameEvent::ZoneWillChange {
                            object: spell,
                            from: Some(Zone::Stack),
                            to: Zone::Battlefield,
                            enters,
                            position: None,
                            face: None,
                            cause: None,
                        },
                    ))]);
                } else if self.targets_still_legal(&entry) {
                    // Instant/sorcery with all targets still legal: run its effect.
                    let frame = Frame {
                        source: spell,
                        controller: entry.controller,
                        this: None,
                        defending_player: None,
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
                        GameEvent::ZoneWillChange {
                            object: spell,
                            from: Some(Zone::Stack),
                            to: Zone::Graveyard,
                            enters: None,
                            position: None,
                            face: None,
                            cause: None,
                        }
                    };
                    self.schedule_front(vec![
                        WorkItem::RunEffect {
                            effect: Box::new(effect),
                            frame,
                        },
                        WorkItem::Emit(Occurrence::single(leave)),
                    ]);
                } else if entry.copy {
                    // [CR#707.10a]: a copy leaves the stack by CEASING to exist
                    // — no zone move, no card. Same shape as a triggered
                    // ability vanishing ([CR#608.2n]).
                    self.schedule_front(vec![WorkItem::Emit(Occurrence::single(
                        GameEvent::AbilityCountered {
                            id: spell,
                            cause: Cause::counter(Agency::StateBasedAction, None),
                        },
                    ))]);
                } else {
                    // [CR#608.2b]: all targets illegal — the spell fizzles.
                    self.schedule_front(vec![WorkItem::Emit(Occurrence::single(
                        GameEvent::ZoneWillChange {
                            object: spell,
                            from: Some(Zone::Stack),
                            to: Zone::Graveyard,
                            enters: None,
                            position: None,
                            face: None,
                            cause: Some(Cause::counter(Agency::StateBasedAction, None)),
                        },
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
                let t = match created {
                    Some(t) => (**t).clone(),
                    None => match &crate::derive::abilities_of_source(self, *source)[*ability] {
                        Ability::Triggered(t) => t.clone(),
                        other => unreachable!(
                            "a Triggered stack object indexes a Triggered ability, got {other:?}"
                        ),
                    },
                };
                let frame = Frame {
                    // [CR#608.2,603.10a]: `~`/`This` is the firing object's
                    // last-known self; the live source may be gone.
                    source: bindings.this.as_ref().map_or(entry.id, |s| s.object),
                    controller: entry.controller,
                    // Exophoric: the firing object's LKI and the combat defender.
                    this: bindings.this.clone(),
                    defending_player: bindings.defending_player,
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
                            effect: Box::new(t.effect),
                            frame,
                        },
                        WorkItem::Emit(Occurrence::single(GameEvent::AbilityResolved(entry.id))),
                    ]);
                } else {
                    // [CR#608.2b]: every target illegal — fizzle, vanish.
                    self.schedule_front(vec![WorkItem::Emit(Occurrence::single(
                        GameEvent::AbilityCountered {
                            id: entry.id,
                            cause: Cause::counter(Agency::StateBasedAction, None),
                        },
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
                            effect: Box::new(ability.effect.clone()),
                            frame,
                        },
                        WorkItem::Emit(Occurrence::single(GameEvent::AbilityResolved(entry.id))),
                    ]);
                } else {
                    // [CR#608.2b]: every target illegal — fizzle, vanish.
                    self.schedule_front(vec![WorkItem::Emit(Occurrence::single(
                        GameEvent::AbilityCountered {
                            id: entry.id,
                            cause: Cause::counter(Agency::StateBasedAction, None),
                        },
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
        Ability::Expanded(e) => spell_ability_effect(&e.value),
        _ => None,
    }
}

/// Look through a [`Binder::Expanded`](deckmaste_core::Binder::Expanded) macro
/// invocation to the structural binder underneath — the binder twin of the
/// effect/selection `peel`s elsewhere.
pub(crate) fn peel_binder(binder: &deckmaste_core::Binder) -> &deckmaste_core::Binder {
    match binder {
        deckmaste_core::Binder::Expanded(e) => peel_binder(&e.value),
        other => other,
    }
}

/// Look through an
/// [`OneShotEffect::Expanded`](deckmaste_core::OneShotEffect::Expanded) macro
/// invocation to the structural effect underneath.
/// Look through `Quantity` macro expansions (`Exactly`, `AtLeast`, …) to
/// the underlying `Range` primitive.
fn deref_quantity(q: &deckmaste_core::Quantity) -> &deckmaste_core::Quantity {
    match q {
        deckmaste_core::Quantity::Expanded(e) => deref_quantity(&e.value),
        range @ deckmaste_core::Quantity::Range(..) => range,
    }
}

pub(crate) fn peel_effect(
    effect: &deckmaste_core::OneShotEffect,
) -> &deckmaste_core::OneShotEffect {
    match effect {
        deckmaste_core::OneShotEffect::Expanded(e) => peel_effect(&e.value),
        other => other,
    }
}

/// The targets declared on a top-level `Targeted` wrapper (peeling
/// `Expanded`), or `&[]` when the effect isn't a wrapper — the announce-list
/// home after the migration ([CR#115.1,601.2c]). A single top-level wrapper is
/// the only shape today; a nested wrapper would need a per-scope target stack.
pub(crate) fn top_targets(effect: &OneShotEffect) -> &[TargetSpec] {
    match effect {
        OneShotEffect::Targeted(te) => &te.targets,
        OneShotEffect::Expanded(e) => top_targets(&e.value),
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
mod tests {
    use std::path::Path;
    use std::sync::Arc;

    use deckmaste_cards::plugin::Plugin;
    use deckmaste_core::Action;
    use deckmaste_core::Binder;
    use deckmaste_core::Card;
    use deckmaste_core::CharacteristicPredicate;
    use deckmaste_core::Count;
    use deckmaste_core::Countable;
    use deckmaste_core::Lookback;
    use deckmaste_core::ObjectKind;
    use deckmaste_core::OneShotEffect;
    use deckmaste_core::PlayerAction;
    use deckmaste_core::Predicate;
    use deckmaste_core::Reference;
    use deckmaste_core::Selection;
    use deckmaste_core::StatePredicate;
    use deckmaste_core::Type;
    use deckmaste_core::Zone;

    use crate::agenda::WorkItem;
    use crate::event::GameEvent;
    use crate::event::Occurrence;
    use crate::matches as obj_matches;
    use crate::object::ObjectId;
    use crate::object::ObjectSource;
    use crate::player::PlayerId;
    use crate::stack::Anaphora;
    use crate::stack::Cardinality;
    use crate::stack::Frame;
    use crate::stack::RefKind;
    use crate::stack::StackEntry;
    use crate::stack::StackObject;
    use crate::stack::ThatBinding;
    use crate::state::GameConfig;
    use crate::state::GameState;
    use crate::state::PlayerConfig;
    use crate::state::StartingPlayer;
    use crate::step::Progress;
    use crate::step::StepOutcome;
    use crate::test_support::frame_for;
    use crate::test_support::frame_src;
    use crate::test_support::frame_src_targets;
    use crate::trigger::TriggerBindings;

    fn builtin() -> Plugin {
        Plugin::load(Path::new(env!("CARGO_MANIFEST_DIR")).join("../../plugins/builtin")).unwrap()
    }

    fn canon() -> Plugin {
        Plugin::load_with_sibling_prelude(
            Path::new(env!("CARGO_MANIFEST_DIR")).join("../../plugins/canon"),
        )
        .unwrap()
    }

    fn deck(card: &Arc<Card>, n: usize) -> Vec<Arc<Card>> {
        vec![Arc::clone(card); n]
    }

    fn game() -> GameState {
        GameState::new(GameConfig {
            players: vec![PlayerConfig { deck: vec![] }, PlayerConfig { deck: vec![] }],
            seed: 7,
            starting_life: 20,
            starting_player: StartingPlayer::Fixed(PlayerId(0)),
            sba_rules: vec![],
            conferral_rules: vec![],
            damage_result_rules: vec![],
            counter_decls: std::collections::HashMap::new(),
            subtypes: std::collections::HashMap::new(),
            types: std::collections::HashMap::new(),
        })
    }

    /// Pulls a second creature out of player 0's opening hand, drops it onto
    /// the battlefield, and hands it to player 1 — owner stays player 0,
    /// controller becomes player 1. Returns the object so tests can read
    /// both sides.
    fn second_bear_to_player_1(state: &mut GameState) -> ObjectId {
        let theirs = *state.zones.hands[0]
            .iter()
            .find(|&&o| obj_matches(state, o, &Predicate::creature()))
            .expect("a second Grizzly Bears in the opening hand");
        state.zones.hands[PlayerId(0).index()].retain(|&o| o != theirs);
        state.objects.obj_mut(theirs).zone = Some(Zone::Battlefield);
        state.objects.obj_mut(theirs).controller = PlayerId(1);
        state.zones.battlefield.push(theirs);
        theirs
    }

    /// Card-authoring mistakes never crash the engine
    /// ([[engine-never-crashes-on-authoring-mistakes]]): an unresolvable
    /// authored reference degrades to the null object id (the effect
    /// fizzles) rather than panicking. Soundness — that a well-formed
    /// card's references always resolve — is the Idris re-emit gate's job,
    /// not a runtime panic.
    #[test]
    fn unbound_reference_degrades_to_null_not_panic() {
        use deckmaste_core::Reference;
        use slotmap::Key;

        let state = game();
        let frame = frame_for(&state, PlayerId(0));
        // `It` outside any binder with no lone announced target — was a panic.
        assert!(state.eval_reference(&Reference::It, &frame).is_null());
        // Event roles read outside any trigger — were `.expect()` panics.
        assert!(
            state
                .eval_reference(&Reference::EventObject, &frame)
                .is_null()
        );
        assert!(
            state
                .eval_reference(&Reference::EventActor, &frame)
                .is_null()
        );
        assert!(
            state
                .eval_reference(&Reference::DefendingPlayer, &frame)
                .is_null()
        );
        // A derived reference over an unbound inner stays null, not a secondary
        // panic in `layers().controller()`.
        assert!(
            state
                .eval_reference(&Reference::ControllerOf(Box::new(Reference::It)), &frame)
                .is_null()
        );
    }

    /// History tallies via `EventCount`/`EventSum` — the general primitives
    /// that subsume the old `Count::Query`/`eval_query` scalar family
    /// ([CR#608.2i]). Fixtures are the same events; assertions use the
    /// replacements.
    #[expect(
        clippy::too_many_lines,
        reason = "one fixture exercises all five history tallies (storm/draws/lands/life-lost/life-gained) end-to-end"
    )]
    #[test]
    fn history_tallies_via_event_count_sum() {
        use deckmaste_core::Agency;
        use deckmaste_core::Count;
        use deckmaste_core::EventFilter;
        use deckmaste_core::Predicate;
        use deckmaste_core::Reference;

        use crate::event::Cause;
        use crate::lki::LkiSnapshot;

        let mut state = game();
        state.turn.turn_number = 1;
        let p = PlayerId(0);
        // A frame anchored on player p — Ref(You) resolves to p's proxy.
        let frame = frame_for(&state, p);

        // Three spells cast this turn (game-wide); `EventCount(Cast, ThisTurn)`
        // returns the FULL count (3). The storm "−1/before this one"
        // self-exclusion is deferred until a storm card exists — no card
        // consumes it yet ([CR#702.40a]).
        // Mint real objects for the Cast path (performed_matches reads their
        // controllers via objects.obj, which panics on stale IDs).
        let sp1 = state.objects.mint(ObjectSource::Player(p), p, None);
        let sp2 = state.objects.mint(ObjectSource::Player(p), p, None);
        let sp3 = state.objects.mint(ObjectSource::Player(p), p, None);
        state.record_history_fact(1, None, GameEvent::SpellCast(sp1));
        state.record_history_fact(1, None, GameEvent::SpellCast(sp2));
        state.record_history_fact(1, None, GameEvent::SpellCast(sp3));
        let cast_event = EventFilter::Cast {
            who: Predicate::Any,
            what: Predicate::Any,
        };
        assert_eq!(
            state.eval_count(
                &Count::EventCount(Box::new(cast_event), Lookback::ThisTurn),
                &frame
            ),
            3,
            "storm: all casts this turn (full count; −1 self-exclusion deferred)"
        );

        // Two draws by p this turn → EventCount(Draw, by: Ref(You)) = 2.
        state.record_history_fact(
            1,
            None,
            GameEvent::WillDraw {
                player: p,
                source: None,
            },
        );
        state.record_history_fact(
            1,
            None,
            GameEvent::WillDraw {
                player: p,
                source: None,
            },
        );
        let draw_event = EventFilter::Drawn {
            who: Predicate::Ref(Reference::You),
            amount: None,
        };
        assert_eq!(
            state.eval_count(
                &Count::EventCount(Box::new(draw_event), Lookback::ThisTurn),
                &frame
            ),
            2,
            "draws by p this turn"
        );

        // One land played by p (a Play-caused battlefield entry).
        // `lands_played_this_turn` is the direct helper; EventCount(Play,
        // by: Ref(You)) is the generic equivalent.
        let land = state
            .objects
            .mint(ObjectSource::Player(p), p, Some(Zone::Battlefield));
        state.record_history_fact(
            1,
            None,
            GameEvent::ZoneChanged {
                snapshot: LkiSnapshot::capture(&state, land),
                from: Some(Zone::Hand),
                to: Zone::Battlefield,
                face: None,
                cause: Some(Cause {
                    verb: "Play".into(),
                    agency: Agency::SpecialAction,
                    agent: None,
                }),
            },
        );
        assert_eq!(
            state.lands_played_this_turn(p),
            1,
            "lands played by p this turn (direct helper)"
        );
        let play_event = EventFilter::Played {
            who: Predicate::Ref(Reference::You),
            what: Predicate::Any,
        };
        assert_eq!(
            state.eval_count(
                &Count::EventCount(Box::new(play_event), Lookback::ThisTurn),
                &frame
            ),
            1,
            "lands played by p via EventCount"
        );

        // Life: lost 3 then 2 (=5), gained 4.
        // EventSum(LoseLife, by: Ref(You)) sums the amounts; EventSum(GainLife)
        // likewise ([CR#119.3]).
        state.record_history_fact(
            1,
            None,
            GameEvent::LifeLost {
                player: p,
                amount: 3,
            },
        );
        state.record_history_fact(
            1,
            None,
            GameEvent::LifeLost {
                player: p,
                amount: 2,
            },
        );
        state.record_history_fact(
            1,
            None,
            GameEvent::LifeGained {
                player: p,
                amount: 4,
            },
        );
        let lose_event = EventFilter::LifeLost {
            who: Predicate::Ref(Reference::You),
            amount: None,
        };
        let gain_event = EventFilter::LifeGained {
            who: Predicate::Ref(Reference::You),
            amount: None,
        };
        assert_eq!(
            state.eval_count(
                &Count::EventSum(Box::new(lose_event), Lookback::ThisTurn),
                &frame
            ),
            5,
            "life lost by p this turn"
        );
        assert_eq!(
            state.eval_count(
                &Count::EventSum(Box::new(gain_event), Lookback::ThisTurn),
                &frame
            ),
            4,
            "life gained by p this turn"
        );

        // Prior-turn entries are excluded once the turn advances.
        state.turn.turn_number = 2;
        let cast_event2 = EventFilter::Cast {
            who: Predicate::Any,
            what: Predicate::Any,
        };
        let draw_event2 = EventFilter::Drawn {
            who: Predicate::Ref(Reference::You),
            amount: None,
        };
        assert_eq!(
            state.eval_count(
                &Count::EventCount(Box::new(cast_event2), Lookback::ThisTurn),
                &frame
            ),
            0,
            "storm resets on new turn"
        );
        assert_eq!(
            state.eval_count(
                &Count::EventCount(Box::new(draw_event2), Lookback::ThisTurn),
                &frame
            ),
            0,
            "draws reset on new turn"
        );
    }

    /// `ability_used_count` counts `AbilityUsed` events keyed by (object,
    /// ability) and respects the `Lookback` filter — `ThisTurn` excludes
    /// prior-turn entries, `ThisGame` includes them all.
    #[test]
    fn ability_used_count_keys_object_ability_window() {
        let mut state = game();
        state.turn.turn_number = 1;

        let obj_a = ObjectId::from_raw(10);
        let obj_b = ObjectId::from_raw(20);

        // Two uses of ability 0 on obj_a, one use of ability 1 on obj_a —
        // all on the current turn.
        state.record_history_fact(
            1,
            None,
            GameEvent::AbilityUsed {
                object: obj_a,
                ability: 0,
            },
        );
        state.record_history_fact(
            1,
            None,
            GameEvent::AbilityUsed {
                object: obj_a,
                ability: 0,
            },
        );
        state.record_history_fact(
            1,
            None,
            GameEvent::AbilityUsed {
                object: obj_a,
                ability: 1,
            },
        );

        assert_eq!(state.ability_used_count(obj_a, 0, Lookback::ThisGame), 2);
        assert_eq!(state.ability_used_count(obj_a, 1, Lookback::ThisGame), 1);
        // obj_b has no uses recorded.
        assert_eq!(state.ability_used_count(obj_b, 0, Lookback::ThisGame), 0);

        // A use of (obj_a, 0) on a DIFFERENT turn.
        state.record_history_fact(
            2,
            None,
            GameEvent::AbilityUsed {
                object: obj_a,
                ability: 0,
            },
        );

        // ThisTurn (still turn 1) excludes the turn-2 entry.
        assert_eq!(state.ability_used_count(obj_a, 0, Lookback::ThisTurn), 2);
        // ThisGame includes it.
        assert_eq!(state.ability_used_count(obj_a, 0, Lookback::ThisGame), 3);
    }

    /// A two-player game; player 0's deck is Grizzly Bears.
    /// Returns the state plus a creature object forced onto the battlefield.
    fn bear_on_field() -> (GameState, ObjectId) {
        let bears = Arc::new(canon().card("Grizzly Bears").unwrap());
        let forest = Arc::new(builtin().card("Forest").unwrap());
        let mut state = GameState::new(GameConfig {
            players: vec![
                PlayerConfig {
                    deck: deck(&bears, 10),
                },
                PlayerConfig {
                    deck: deck(&forest, 10),
                },
            ],
            seed: 1,
            starting_life: 20,
            starting_player: StartingPlayer::Fixed(PlayerId(0)),
            sba_rules: vec![],
            conferral_rules: vec![],
            damage_result_rules: vec![],
            counter_decls: std::collections::HashMap::new(),
            subtypes: std::collections::HashMap::new(),
            types: std::collections::HashMap::new(),
        });
        let bear = *state.zones.hands[0]
            .iter()
            .find(|&&o| {
                obj_matches(
                    &state,
                    o,
                    &Predicate::Characteristic(deckmaste_core::CharacteristicPredicate::Type(
                        Type::Creature.name(),
                    )),
                )
            })
            .expect("a Grizzly Bears in the opening hand");
        state.zones.hands[PlayerId(0).index()].retain(|&o| o != bear);
        state.objects.obj_mut(bear).zone = Some(Zone::Battlefield);
        state.zones.battlefield.push(bear);
        (state, bear)
    }

    /// A two-player game with player 0's deck = Darksteel Myr (an
    /// indestructible 0/1), one forced onto the battlefield.
    fn myr_on_field() -> (GameState, ObjectId) {
        let myr = Arc::new(canon().card("Darksteel Myr").unwrap());
        let forest = Arc::new(builtin().card("Forest").unwrap());
        let mut state = GameState::new(GameConfig {
            players: vec![
                PlayerConfig {
                    deck: deck(&myr, 10),
                },
                PlayerConfig {
                    deck: deck(&forest, 10),
                },
            ],
            seed: 1,
            starting_life: 20,
            starting_player: StartingPlayer::Fixed(PlayerId(0)),
            sba_rules: vec![],
            conferral_rules: vec![],
            damage_result_rules: vec![],
            counter_decls: std::collections::HashMap::new(),
            subtypes: std::collections::HashMap::new(),
            types: std::collections::HashMap::new(),
        });
        let m = *state.zones.hands[0]
            .iter()
            .find(|&&o| obj_matches(&state, o, &Predicate::creature()))
            .expect("a Darksteel Myr in the opening hand");
        state.zones.hands[PlayerId(0).index()].retain(|&o| o != m);
        state.objects.obj_mut(m).zone = Some(Zone::Battlefield);
        state.zones.battlefield.push(m);
        (state, m)
    }

    /// Two Grizzly Bears (player 0) forced onto the battlefield — `(a, b)`.
    /// The attachment subsystem is type-agnostic in Stage 1 ([CR#701.3b]'s
    /// type-based attachability is Task 4.x), so two bare permanents exercise
    /// the relation/verb/event mechanism directly.
    fn two_permanents_on_field() -> (GameState, ObjectId, ObjectId) {
        let (mut state, a) = bear_on_field();
        let b = *state.zones.hands[0]
            .iter()
            .find(|&&o| obj_matches(&state, o, &Predicate::creature()))
            .expect("a second Grizzly Bears in the opening hand");
        state.zones.hands[PlayerId(0).index()].retain(|&o| o != b);
        state.objects.obj_mut(b).zone = Some(Zone::Battlefield);
        state.zones.battlefield.push(b);
        (state, a, b)
    }

    /// Mint (on the battlefield, player 0) an Equipment-shaped artifact
    /// carrying the default-deny `Innate(May(Attach(what: Ref(This), to:
    /// Creature)))` grant — an attachment that may legally attach to a
    /// creature host.
    fn may_attach_creature_equipment(state: &mut GameState) -> ObjectId {
        use deckmaste_core::Ability;
        use deckmaste_core::Card;
        use deckmaste_core::CardFace;
        use deckmaste_core::Deontic;
        use deckmaste_core::DeonticAction;
        use deckmaste_core::StaticEffect;
        let card = Card::Normal(CardFace {
            name: "Test Equipment".into(),
            types: vec![Type::Artifact.def()],
            abilities: vec![Ability::Innate(Box::new(Ability::Static(
                StaticEffect::Deontic(Deontic::May(DeonticAction::Attach {
                    what: Predicate::Ref(Reference::This),
                    to: Predicate::creature(),
                })),
            )))],
            ..CardFace::default()
        });
        let card_id = state.cards.push(Arc::new(card), PlayerId(0));
        let id = state.objects.mint(
            ObjectSource::Card(card_id),
            PlayerId(0),
            Some(Zone::Battlefield),
        );
        state.zones.battlefield.push(id);
        id
    }

    /// Drives the agenda forward a bounded number of steps — assert on the
    /// post-condition, not the iteration count. A pending decision stops it.
    fn drain(state: &mut GameState) {
        for _ in 0..30 {
            if matches!(state.step(), StepOutcome::NeedsDecision(_)) {
                break;
            }
        }
    }

    /// Process exactly a test-injected effect's front-scheduled work, WITHOUT
    /// advancing the turn structure. `run_effect` schedules its work
    /// (`Emit`/`RunEffect`/…) at the front of the agenda; this steps while the
    /// front item is such injected work and stops the moment a turn-structure
    /// item (`BeginStep`/`CheckSbas`/`OpenPriority`/…) or a decision would run.
    /// A test-only driver for sequentially-injected effects: it never parks a
    /// priority (so the next `run_effect`'s front work isn't blocked) and never
    /// drains the turn loop dry.
    fn run_injected(state: &mut GameState) {
        for _ in 0..30 {
            let injected = matches!(
                state.agenda.front(),
                Some(
                    WorkItem::Emit(_)
                        | WorkItem::RunEffect { .. }
                        | WorkItem::Resolve(_)
                        | WorkItem::BeginNote { .. }
                        | WorkItem::EndNote
                        | WorkItem::ChooseNoteNumber { .. }
                        | WorkItem::ChooseNoteObjects { .. }
                )
            );
            if !injected || state.pending.is_some() {
                return;
            }
            let _ = state.step();
        }
    }

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

    /// Bound-role reads chase the move record ([CR#400.7j]): a One `that`
    /// binding, the `It` bindings, and the lone-target `It` fallback resolve
    /// to the object's latest same-resolution incarnation; `Target(n)` never
    /// chases (the announced slot stays positional).
    #[test]
    fn bound_role_reads_chase_the_move_record() {
        let (mut state, a, b) = two_permanents_on_field();
        state.moved_chain.push((a, b));

        // That(Sort) over a One binding chases a -> b.
        let mut frame = frame_src(a);
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
        frame.anaphora.targets = vec![vec![a]];
        assert_eq!(state.eval_reference(&Reference::Target(0), &frame), a);

        // The `It` Object binding chases.
        let snap = crate::lki::LkiSnapshot::capture(&state, a);
        frame.anaphora.it = Some(crate::stack::ItBinding::Object(snap));
        assert_eq!(state.eval_reference(&Reference::It, &frame), b);

        // The lone-target `It` fallback chases too ("exile target creature,
        // … return IT").
        let lone = frame_src_targets(a, vec![a]);
        assert_eq!(state.eval_reference(&Reference::It, &lone), b);
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
        let mut frame = frame_src(a);
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
        let mut frame = frame_src(a);
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
        let frame = frame_src(a);

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
                Reference::This,
                deckmaste_core::Destination::Zone(Zone::Exile),
                vec![],
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
        let pframe = frame_src(product);
        state.run_effect(
            OneShotEffect::Act(Action::Move(
                Reference::This,
                deckmaste_core::Destination::Zone(Zone::Hand),
                vec![],
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
        let frame = frame_src(a);
        state.run_effect(
            OneShotEffect::With(With {
                binder: Binder::Produce(Box::new(Action::Move(
                    Reference::This,
                    Destination::Zone(Zone::Exile),
                    vec![],
                ))),
                body: Box::new(OneShotEffect::Act(Action::Move(
                    Reference::That(deckmaste_core::Sort::Card),
                    Destination::Zone(Zone::Battlefield),
                    vec![],
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

    /// Whether the history log holds a fact matching `pred` (game-wide).
    fn logged(state: &GameState, pred: impl Fn(&GameEvent) -> bool) -> bool {
        state
            .history
            .scan(deckmaste_core::Lookback::ThisGame, state.turn.turn_number)
            .any(pred)
    }

    /// [CR#701.3a]: `Attach` sets the attachment→host relation and records the
    /// `Attached` fact. Under default-deny the attachment carries a
    /// `May(Attach to: Creature)` grant so the (creature) host is legal.
    #[test]
    fn attach_sets_the_relation_and_emits_attached() {
        let (mut state, _bear, b) = two_permanents_on_field();
        // `b` is a creature (Grizzly Bears); mint a granted attachment for `a`.
        let a = may_attach_creature_equipment(&mut state);
        let frame = frame_src_targets(a, vec![b]);
        state.run_effect(
            OneShotEffect::Act(Action::Attach {
                what: Reference::This,
                to: Reference::It,
            }),
            &frame,
        );
        drain(&mut state);
        assert_eq!(
            state.objects.obj(a).attached_to,
            Some(b),
            "a is attached to b"
        );
        assert!(
            logged(
                &state,
                |e| matches!(e, GameEvent::Attached { attachment, host }
                if *attachment == a && *host == b)
            ),
            "Attached fact recorded"
        );
    }

    /// [CR#701.3a]: attaching to the host it is already on is a no-op — no
    /// second `Attached` fact (transition-only, [CR#603.2e]).
    #[test]
    fn attach_to_current_host_is_a_noop() {
        let (mut state, _bear, b) = two_permanents_on_field();
        // Under default-deny the attachment needs a `May(Attach to: Creature)`
        // grant, or the drain's SBA sweep would unattach it from the (creature)
        // host `b` before the re-attach no-op is even observed.
        let a = may_attach_creature_equipment(&mut state);
        state.objects.obj_mut(a).attached_to = Some(b);
        let frame = frame_src_targets(a, vec![b]);
        state.run_effect(
            OneShotEffect::Act(Action::Attach {
                what: Reference::This,
                to: Reference::It,
            }),
            &frame,
        );
        drain(&mut state);
        assert_eq!(state.objects.obj(a).attached_to, Some(b));
        assert!(
            !logged(&state, |e| matches!(e, GameEvent::Attached { .. })),
            "no Attached fact for a re-attach to the current host"
        );
    }

    /// [CR#303.4d]: an attachment can't be attached to itself — a no-op.
    #[test]
    fn attach_to_self_is_a_noop() {
        let (mut state, a, _b) = two_permanents_on_field();
        let frame = frame_src_targets(a, vec![a]);
        state.run_effect(
            OneShotEffect::Act(Action::Attach {
                what: Reference::This,
                to: Reference::It,
            }),
            &frame,
        );
        drain(&mut state);
        assert_eq!(state.objects.obj(a).attached_to, None, "host == what no-op");
        assert!(
            !logged(&state, |e| matches!(e, GameEvent::Attached { .. })),
            "no Attached fact for a self-attach"
        );
    }

    /// [CR#701.3b]: `Attach` no-ops on an illegal host — under default-deny the
    /// attachment carries a conferred `Innate(May(Attach(what: Ref(This), to:
    /// Creature)))` grant (the Equipment-subtype shape), and the host is a
    /// non-creature, so no grant covers the pair: the link stays `None` and no
    /// `Attached` fact is recorded.
    #[test]
    fn attach_illegal_noop() {
        use deckmaste_core::CardFace;

        let mut state = game();
        // The attachment: an Equipment-shaped artifact whose May(Attach) grant
        // only covers creature hosts (mirrors the Equipment subtype confer).
        let equip = may_attach_creature_equipment(&mut state);

        // The host: a non-creature artifact "Rock".
        let rock_card = Card::Normal(CardFace {
            name: "Rock".into(),
            types: vec![Type::Artifact.def()],
            ..CardFace::default()
        });
        let rock_id = state.cards.push(Arc::new(rock_card), PlayerId(0));
        let rock = state.objects.mint(
            ObjectSource::Card(rock_id),
            PlayerId(0),
            Some(Zone::Battlefield),
        );
        state.zones.battlefield.push(rock);

        let frame = frame_src_targets(equip, vec![rock]);
        state.run_effect(
            OneShotEffect::Act(Action::Attach {
                what: Reference::This,
                to: Reference::It,
            }),
            &frame,
        );
        drain(&mut state);
        assert_eq!(
            state.objects.obj(equip).attached_to,
            None,
            "illegal attach no-ops ([CR#701.3b])"
        );
        assert!(
            !logged(&state, |e| matches!(e, GameEvent::Attached { .. })),
            "no Attached fact for an illegal host"
        );
    }

    /// [CR#701.3d]: `Unattach` clears the relation and records the `Unattached`
    /// fact carrying the former host.
    #[test]
    fn unattach_clears_the_relation_and_emits_unattached() {
        let (mut state, a, b) = two_permanents_on_field();
        state.objects.obj_mut(a).attached_to = Some(b);
        let frame = frame_src(a);
        state.run_effect(
            OneShotEffect::Act(Action::Unattach(Reference::This)),
            &frame,
        );
        drain(&mut state);
        assert_eq!(
            state.objects.obj(a).attached_to,
            None,
            "a is now unattached"
        );
        assert!(
            logged(
                &state,
                |e| matches!(e, GameEvent::Unattached { attachment, former_host }
                if *attachment == a && *former_host == b)
            ),
            "Unattached fact records the former host"
        );
    }

    /// [CR#701.3d]: unattaching an attachment that isn't attached is a no-op —
    /// no `Unattached` fact (transition-only, [CR#603.2e]).
    #[test]
    fn unattach_of_an_unattached_object_is_a_noop() {
        let (mut state, a, _b) = two_permanents_on_field();
        let frame = frame_src(a);
        state.run_effect(
            OneShotEffect::Act(Action::Unattach(Reference::This)),
            &frame,
        );
        drain(&mut state);
        assert_eq!(state.objects.obj(a).attached_to, None);
        assert!(
            !logged(&state, |e| matches!(e, GameEvent::Unattached { .. })),
            "no Unattached fact for an already-unattached object"
        );
    }

    /// [CR#301.5]: `AttachHostOf(This)` from the attachment resolves to its
    /// host.
    #[test]
    fn eval_reference_attach_host_of() {
        let (mut state, a, b) = two_permanents_on_field();
        state.objects.obj_mut(a).attached_to = Some(b);

        let frame_a = frame_src(a);
        assert_eq!(
            state.eval_reference(
                &Reference::AttachHostOf(Box::new(Reference::This)),
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
        let creatures = Predicate::And(vec![
            Predicate::State(StatePredicate::InZone(Zone::Battlefield)),
            Predicate::creature(),
        ]);
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

    /// A `Random` group bound into the frame drives a verb with NO surfaced
    /// decision: `Each(Random(Exactly 1, creature), Destroy(It))`
    /// over a frame whose `chosen` holds the RNG's pick destroys exactly that
    /// one creature. (Verbs take a single `Reference`, so plurality/choice is
    /// the enclosing `Each`; the `Random` inline-RNG resolution is a dormant
    /// seam that reads `frame.anaphora.chosen` — [CR#608.2d].)
    #[test]
    fn destroy_random_destroys_one_without_a_decision() {
        use deckmaste_core::Each;
        use deckmaste_core::Quantity;

        use crate::step::StepOutcome;

        let (mut state, bear) = bear_on_field();
        let theirs = second_bear_to_player_1(&mut state);

        let creatures = Predicate::And(vec![
            Predicate::State(StatePredicate::InZone(Zone::Battlefield)),
            Predicate::creature(),
        ]);
        // The RNG's pick is bound into the frame before the group is read.
        let mut frame = frame_src(bear);
        frame.anaphora.chosen = Some(vec![theirs]);
        let before = [bear, theirs]
            .iter()
            .filter(|o| state.zones.battlefield.contains(o))
            .count();
        assert_eq!(before, 2);

        state.run_effect(
            OneShotEffect::Each(Each {
                binder: Binder::Existing(Selection::Random(Quantity::one(), creatures)),
                effect: Box::new(OneShotEffect::Act(Action::Destroy(Reference::It))),
            }),
            &frame,
        );
        // No decision: the Random group is already bound in the frame.
        assert!(
            !matches!(state.step(), StepOutcome::NeedsDecision(_)),
            "a bound Random group surfaces no decision"
        );
        // Pump the agenda to completion (bounded safety cap; assert on the
        // post-condition, not the iteration count).
        for _ in 0..30 {
            let alive = [bear, theirs]
                .iter()
                .filter(|o| state.zones.battlefield.contains(o))
                .count();
            if alive == 1 {
                break;
            }
            let _ = state.step();
        }
        let alive = [bear, theirs]
            .iter()
            .filter(|o| state.zones.battlefield.contains(o))
            .count();
        assert_eq!(alive, 1, "exactly one creature destroyed (the bound pick)");
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
        let creatures = Predicate::And(vec![
            Predicate::State(StatePredicate::InZone(Zone::Battlefield)),
            Predicate::creature(),
        ]);
        let frame = frame_src(bear);
        state.run_effect(
            OneShotEffect::With(With {
                binder: Binder::ChooseOne {
                    filter: creatures,
                    by: Reference::Opponent,
                },
                body: Box::new(OneShotEffect::Act(Action::Destroy(Reference::That(
                    deckmaste_core::Sort::Permanent,
                )))),
            }),
            &frame,
        );
        let StepOutcome::NeedsDecision(PendingDecision::ChooseObjects { player, .. }) =
            state.step()
        else {
            panic!("expected ChooseObjects, got {:?}", state.pending);
        };
        assert_eq!(
            player,
            PlayerId(1),
            "the opponent (the binder's `by`) makes the pick"
        );
    }

    /// `With(ChooseOne(creature), Destroy(That))` surfaces `ChooseObjects`; an
    /// out-of-range count and an out-of-pool object are rejected; a legal pick
    /// destroys exactly that creature ([CR#608.2d]). Choosing is a pre-step
    /// (`With`) bound as `That`, never part of the verb.
    #[test]
    fn destroy_choose_surfaces_decision_validates_and_destroys() {
        use deckmaste_core::Binder;
        use deckmaste_core::With;

        use crate::decide::Decision;
        use crate::decide::PendingDecision;
        use crate::step::StepOutcome;

        let (mut state, bear) = bear_on_field();
        let theirs = second_bear_to_player_1(&mut state);

        let creatures = Predicate::And(vec![
            Predicate::State(StatePredicate::InZone(Zone::Battlefield)),
            Predicate::creature(),
        ]);
        let frame = frame_src(bear);
        state.run_effect(
            OneShotEffect::With(With {
                binder: Binder::ChooseOne {
                    filter: creatures,
                    by: Reference::You,
                },
                body: Box::new(OneShotEffect::Act(Action::Destroy(Reference::That(
                    deckmaste_core::Sort::Permanent,
                )))),
            }),
            &frame,
        );

        let StepOutcome::NeedsDecision(PendingDecision::ChooseObjects {
            player,
            candidates,
            min,
            max,
        }) = state.step()
        else {
            panic!("expected ChooseObjects, got {:?}", state.pending);
        };
        assert_eq!(player, PlayerId(0));
        assert_eq!((min, max), (1, 1));
        assert_eq!(
            candidates.len(),
            2,
            "both battlefield creatures are candidates"
        );

        // Too many (count 2 > max 1).
        assert!(
            state
                .submit_decision(Decision::Chosen(candidates.clone()))
                .is_err(),
            "count must be within [min, max]"
        );
        // Out of pool (a player proxy is not a creature).
        assert!(
            state
                .submit_decision(Decision::Chosen(vec![state.player(PlayerId(0)).object]))
                .is_err(),
            "every chosen object must be a candidate"
        );

        // Legal: destroy player 1's creature.
        state
            .submit_decision(Decision::Chosen(vec![theirs]))
            .unwrap();
        // Pump the agenda to completion (bounded safety cap; we assert on the
        // post-condition, not the iteration count).
        for _ in 0..30 {
            if !state.zones.battlefield.contains(&theirs) {
                break;
            }
            let _ = state.step();
        }
        assert!(
            !state.zones.battlefield.contains(&theirs),
            "the chosen creature is destroyed"
        );
        assert!(
            state.zones.battlefield.contains(&bear),
            "the unchosen creature survives"
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

        let frame = Frame {
            this: Some(crate::lki::LkiSnapshot::capture(&state, bear)),
            anaphora: Anaphora {
                targets: vec![vec![theirs]],
                that_object: Some(crate::lki::LkiSnapshot::capture(&state, theirs)),
                that_player: Some(PlayerId(1)),
                ..Anaphora::empty()
            },
            ..Frame::bare(bear, PlayerId(0))
        };

        assert_eq!(
            state.eval_reference(
                &Reference::ControllerOf(Box::new(Reference::Target(0))),
                &frame
            ),
            state.player(PlayerId(1)).object,
            "controller of player 1's creature is player 1"
        );
        assert_eq!(
            state.eval_reference(&Reference::OwnerOf(Box::new(Reference::Target(0))), &frame),
            state.player(PlayerId(0)).object,
            "owner is still player 0"
        );
        // The provenance-explicit event roles ([CR#603.2e]): the OBJECT (the
        // moved/acting object) and the ACTOR (the responsible player).
        assert_eq!(
            state.eval_reference(&Reference::EventObject, &frame),
            theirs,
            "EventObject is the bound snapshot's object (the moved/acting object)"
        );
        assert_eq!(
            state.eval_reference(&Reference::EventActor, &frame),
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
        let object_patient = Frame {
            this: Some(crate::lki::LkiSnapshot::capture(&state, bear)),
            defending_player: Some(PlayerId(1)),
            anaphora: Anaphora {
                that_patient: Some(crate::trigger::EventPatient::Object(
                    crate::lki::LkiSnapshot::capture(&state, theirs),
                )),
                ..Anaphora::empty()
            },
            ..Frame::bare(bear, PlayerId(0))
        };
        assert_eq!(
            state.eval_reference(&Reference::EventPatient, &object_patient),
            theirs,
            "an object patient resolves to its object"
        );
        assert_eq!(
            state.eval_reference(&Reference::DefendingPlayer, &object_patient),
            state.player(PlayerId(1)).object,
            "DefendingPlayer is always the player proxy"
        );

        // A PLAYER patient (a damage recipient player) resolves to the proxy.
        let player_patient = Frame {
            anaphora: Anaphora {
                that_patient: Some(crate::trigger::EventPatient::Player(PlayerId(1))),
                ..Anaphora::empty()
            },
            ..Frame::bare(bear, PlayerId(0))
        };
        assert_eq!(
            state.eval_reference(&Reference::EventPatient, &player_patient),
            state.player(PlayerId(1)).object,
            "a player patient resolves to the player proxy"
        );
    }

    /// [CR#702.12b]: an indestructible permanent can't be destroyed — the
    /// `Destroy` action's `WillDestroy` intent is suppressed by the
    /// event-side cant pass ([CR#614.17]) in `apply_occurrence`, so the
    /// Myr stays on the battlefield.
    #[test]
    fn indestructible_survives_destroy_action() {
        let (mut state, myr) = myr_on_field();
        let frame = frame_src(myr);
        state.run_effect(OneShotEffect::Act(Action::Destroy(Reference::This)), &frame);
        // WillDestroy applies and schedules no zone move (replaced to nothing).
        let _ = state.step();
        assert!(
            state.objects.get(myr).is_some(),
            "indestructible object still exists"
        );
        assert!(
            state.zones.battlefield.contains(&myr),
            "still on the battlefield"
        );
        assert!(state.zones.graveyards[0].is_empty(), "not destroyed");
    }

    /// A destructible creature still dies: `Destroy` → `WillDestroy` (nothing
    /// replaces it) → `ZoneWillChange(Battlefield → Graveyard)` →
    /// `ZoneChanged`, reminting it into its owner's graveyard.
    #[test]
    fn destroy_action_sends_a_normal_creature_to_its_graveyard() {
        let (mut state, bear) = bear_on_field();
        let frame = frame_src(bear);
        state.run_effect(OneShotEffect::Act(Action::Destroy(Reference::This)), &frame);
        // WillDestroy → ZoneWillChange → ZoneChanged.
        for _ in 0..3 {
            let _ = state.step();
        }
        assert!(state.objects.get(bear).is_none(), "old battlefield id gone");
        assert!(!state.zones.battlefield.contains(&bear));
        assert_eq!(state.zones.graveyards[0].len(), 1);
    }

    /// [CR#400.7]: `Move(This, Graveyard)` is a PLAIN relocation — no
    /// `WillDestroy` intent, so it's a direct `ZoneWillChange(Battlefield →
    /// Graveyard)` → `ZoneChanged`, reminting the object into its OWNER's
    /// graveyard. (Indestructible would not save it — but a plain Grizzly Bears
    /// exercises the move path.)
    #[test]
    fn move_sends_this_to_owner_graveyard() {
        let (mut state, bear) = bear_on_field();
        let frame = frame_src(bear);
        state.run_effect(
            OneShotEffect::Act(Action::move_to(Reference::This, Zone::Graveyard)),
            &frame,
        );
        // ZoneWillChange → ZoneChanged (one fewer step than Destroy — no
        // WillDestroy replace stage).
        for _ in 0..2 {
            let _ = state.step();
        }
        assert!(state.objects.get(bear).is_none(), "old battlefield id gone");
        assert!(!state.zones.battlefield.contains(&bear));
        assert_eq!(
            state.zones.graveyards[0].len(),
            1,
            "moved into owner's graveyard"
        );
    }

    #[test]
    fn action_items_for_tap_draw_loselife() {
        let (state, src) = bear_on_field();
        let frame = frame_src(src);

        // By(You, Tap(This)) -> one Single(Tapped(src)) carrying the
        // effect-instruction cause triple (events.md §3).
        let items = state.action_items(&Action::by_you(PlayerAction::Tap(Reference::This)), &frame);
        assert_eq!(
            items,
            vec![WorkItem::Emit(Occurrence::Single(GameEvent::Tapped {
                object: src,
                cause: Some(crate::event::Cause {
                    verb: "Tap".into(),
                    agency: deckmaste_core::Agency::EffectInstruction,
                    agent: Some((src, PlayerId(0))),
                }),
            }))]
        );

        // By(You, Draw(2)) -> two sequential Single(WillDraw) for the controller
        let items = state.action_items(
            &Action::by_you(PlayerAction::Draw(Count::Literal(2))),
            &frame,
        );
        assert_eq!(items.len(), 2);
        assert!(items.iter().all(|item| matches!(
            item,
            WorkItem::Emit(Occurrence::Single(GameEvent::WillDraw {
                player: PlayerId(0),
                ..
            }))
        )));

        // By(You, LoseLife(3)) -> one Single(LifeLost{player0, 3})
        let items = state.action_items(
            &Action::by_you(PlayerAction::LoseLife(Count::Literal(3))),
            &frame,
        );
        assert_eq!(
            items,
            vec![WorkItem::Emit(Occurrence::Single(GameEvent::LifeLost {
                player: PlayerId(0),
                amount: 3,
            }))]
        );
    }

    /// [CR#701.26a]: only an untapped permanent can be tapped — a tap
    /// instruction on an already-tapped object is a no-op, and a no-op is
    /// no event ([CR#603.2e] "becomes tapped" fires on the transition only).
    #[test]
    fn tap_effect_skips_already_tapped() {
        let (mut state, src) = bear_on_field();
        state.objects.obj_mut(src).tapped = true;
        let frame = frame_src(src);
        let items = state.action_items(&Action::by_you(PlayerAction::Tap(Reference::This)), &frame);
        assert_eq!(
            items,
            vec![],
            "tapping an already-tapped object emits nothing"
        );
    }

    /// [CR#701.26b]: the untap mirror — untapping an untapped object is a
    /// no-op, no event.
    #[test]
    fn untap_effect_skips_already_untapped() {
        let (state, src) = bear_on_field();
        let frame = frame_src(src);
        let items = state.action_items(
            &Action::by_you(PlayerAction::Untap(Reference::This)),
            &frame,
        );
        assert_eq!(
            items,
            vec![],
            "untapping an already-untapped object emits nothing"
        );
    }

    /// An explicit agent: `By(Target(0), Draw(2))` draws for the targeted
    /// player, not the controller. Targets player 1's proxy.
    #[test]
    fn action_items_explicit_agent_draws_for_target() {
        let (state, src) = bear_on_field();
        let p1_proxy = state.players[1].object;
        let frame = frame_src_targets(src, vec![p1_proxy]);
        let items = state.action_items(
            &Action::By(Reference::It, PlayerAction::Draw(Count::Literal(2))),
            &frame,
        );
        assert_eq!(items.len(), 2);
        assert!(items.iter().all(|item| matches!(
            item,
            WorkItem::Emit(Occurrence::Single(GameEvent::WillDraw {
                player: PlayerId(1),
                ..
            }))
        )));
    }

    /// `CountOf` is the filter's live cardinality; a `ControlledBy(Ref(You))`
    /// relation anchors to the frame's side via the watcher.
    #[test]
    fn count_of_counts_live_matching_objects() {
        let (mut state, bear) = bear_on_field();
        // A second bear onto the battlefield, then handed to player 1.
        let _ = second_bear_to_player_1(&mut state);

        let frame = frame_src(bear);
        let creatures = Predicate::And(vec![
            Predicate::State(StatePredicate::InZone(Zone::Battlefield)),
            Predicate::creature(),
        ]);
        assert_eq!(
            state.eval_count(
                &Count::CountOf(Countable::Objects(Box::new(creatures.clone()))),
                &frame
            ),
            2
        );

        // "Creatures you control": only the frame side's bear.
        let yours = Predicate::And(vec![
            creatures,
            Predicate::Relation(deckmaste_core::RelationPredicate::ControlledBy(Box::new(
                Predicate::Ref(Reference::You),
            ))),
        ]);
        assert_eq!(
            state.eval_count(&Count::CountOf(Countable::Objects(Box::new(yours))), &frame),
            1
        );
    }

    /// Mint a battlefield permanent with the given printed mana cost, with no
    /// other characteristics — the fixture the pip-count (devotion,
    /// [CR#700.5]) tests below drive.
    fn permanent_with_cost(state: &mut GameState, mana_cost: &str) -> ObjectId {
        let card = Card::Normal(CardFace {
            name: "Test Permanent".into(),
            mana_cost: mana_cost.parse().unwrap(),
            types: vec![Type::Artifact.def()],
            ..CardFace::default()
        });
        let cid = state.cards.push(Arc::new(card), PlayerId(0));
        let obj = state.objects.mint(
            ObjectSource::Card(cid),
            PlayerId(0),
            Some(Zone::Battlefield),
        );
        state.zones.battlefield.push(obj);
        obj
    }

    /// `CountOf(ManaSymbols(..))` ([CR#700.5] devotion) counts matching pips
    /// in the referenced object's printed cost: a plain colored count, a
    /// hybrid pip counting toward EACH of its colors, and an `Or` disjunction
    /// counting either.
    #[test]
    fn count_of_mana_symbols_counts_devotion_pips() {
        use deckmaste_core::SymbolPred;

        let mut state = game();
        let gg1 = permanent_with_cost(&mut state, "{G}{G}{1}");
        let frame = frame_src(gg1);
        assert_eq!(
            state.eval_count(
                &Count::CountOf(Countable::ManaSymbols(
                    Box::new(Reference::This),
                    SymbolPred::CountsAs(deckmaste_core::Color::Green),
                )),
                &frame,
            ),
            2,
            "{{G}}{{G}}{{1}} has two green pips"
        );

        let gwgw = permanent_with_cost(&mut state, "{G/W}{G/W}");
        let frame = frame_src(gwgw);
        assert_eq!(
            state.eval_count(
                &Count::CountOf(Countable::ManaSymbols(
                    Box::new(Reference::This),
                    SymbolPred::CountsAs(deckmaste_core::Color::Green),
                )),
                &frame,
            ),
            2,
            "each {{G/W}} hybrid pip counts toward green devotion"
        );
        assert_eq!(
            state.eval_count(
                &Count::CountOf(Countable::ManaSymbols(
                    Box::new(Reference::This),
                    SymbolPred::CountsAs(deckmaste_core::Color::White),
                )),
                &frame,
            ),
            2,
            "…and toward white devotion too"
        );
        assert_eq!(
            state.eval_count(
                &Count::CountOf(Countable::ManaSymbols(
                    Box::new(Reference::This),
                    SymbolPred::Or(vec![
                        SymbolPred::CountsAs(deckmaste_core::Color::White),
                        SymbolPred::CountsAs(deckmaste_core::Color::Black),
                    ]),
                )),
                &frame,
            ),
            2,
            "Or([White, Black]) over {{G/W}}{{G/W}} matches on the White half of each pip"
        );
    }

    /// Mint a battlefield creature with an explicit power/toughness — the
    /// fixture the aggregate-fold (`SumOf` over `StatOf`) test drives.
    fn creature_with_power(state: &mut GameState, power: deckmaste_core::Int) -> ObjectId {
        let card = Card::Normal(CardFace {
            name: "Test Creature".into(),
            types: vec![Type::Creature.def()],
            power: Some(deckmaste_core::StatValue::Number(power)),
            toughness: Some(deckmaste_core::StatValue::Number(power)),
            ..CardFace::default()
        });
        let cid = state.cards.push(Arc::new(card), PlayerId(0));
        let obj = state.objects.mint(
            ObjectSource::Card(cid),
            PlayerId(0),
            Some(Zone::Battlefield),
        );
        state.zones.battlefield.push(obj);
        obj
    }

    /// `Count::Aggregate(op, Projection)` folds a per-element `Count` over the
    /// projected set ([CR#107.1]), mirroring `Selection::Pick`'s per-candidate
    /// `It` binding. Devotion decomposes to `Aggregate(SumOf, Project(<your
    /// permanents>, CountOf(ManaSymbols(It, CountsAs(Green)))))`
    /// ([CR#700.5]); `SumOf` over `StatOf(It, Power)` totals power; every
    /// `AggregateOp` folds the empty set to 0 (never-crash).
    #[test]
    fn aggregate_folds_a_projection_over_a_selection() {
        use deckmaste_core::AggregateOp;
        use deckmaste_core::Color;
        use deckmaste_core::Projection;
        use deckmaste_core::RelationPredicate;
        use deckmaste_core::StatePredicate;
        use deckmaste_core::SymbolPred;

        let mut state = game();
        let _ = permanent_with_cost(&mut state, "{2}{G}");
        let _ = permanent_with_cost(&mut state, "{G}{G}");
        let src = permanent_with_cost(&mut state, "{1}");
        let frame = frame_src(src);

        let your_permanents = Predicate::And(vec![
            Predicate::State(StatePredicate::InZone(Zone::Battlefield)),
            Predicate::Relation(RelationPredicate::ControlledBy(Box::new(Predicate::Ref(
                Reference::You,
            )))),
        ]);

        let devotion_green = Count::Aggregate(
            AggregateOp::SumOf,
            Projection {
                of: Countable::Objects(Box::new(your_permanents)),
                by: Box::new(Count::CountOf(Countable::ManaSymbols(
                    Box::new(Reference::It),
                    SymbolPred::CountsAs(Color::Green),
                ))),
            },
        );
        assert_eq!(
            state.eval_count(&devotion_green, &frame),
            3,
            "{{2}}{{G}} + {{G}}{{G}} + {{1}} = 3 green pips total"
        );

        // `SumOf` over `StatOf(It, Power)`: total power of your creatures.
        let mut power_state = game();
        let src = permanent_with_cost(&mut power_state, "{1}");
        let _ = creature_with_power(&mut power_state, 2);
        let _ = creature_with_power(&mut power_state, 5);
        let frame = frame_src(src);
        let total_power = Count::Aggregate(
            AggregateOp::SumOf,
            Projection {
                of: Countable::Objects(Box::new(Predicate::And(vec![
                    Predicate::State(StatePredicate::InZone(Zone::Battlefield)),
                    Predicate::creature(),
                ]))),
                by: Box::new(Count::StatOf(Reference::It, deckmaste_core::Stat::Power)),
            },
        );
        assert_eq!(power_state.eval_count(&total_power, &frame), 7);

        // The empty set folds every `AggregateOp` to 0.
        let empty = Countable::Objects(Box::new(Predicate::Not(Box::new(Predicate::Any))));
        for op in [
            AggregateOp::SumOf,
            AggregateOp::MinOf,
            AggregateOp::MaxOf,
            AggregateOp::AverageOf(deckmaste_core::RoundMode::RoundUp),
        ] {
            let empty_fold = Count::Aggregate(
                op,
                Projection {
                    of: empty.clone(),
                    by: Box::new(Count::StatOf(Reference::It, deckmaste_core::Stat::Power)),
                },
            );
            assert_eq!(
                power_state.eval_count(&empty_fold, &frame),
                0,
                "{op:?} over the empty set is 0"
            );
        }
    }

    /// The cross-player fold ([CR#119.1] Arbiter of Knollridge): `Aggregate`
    /// over a `Countable::Players` source reads each matching player's
    /// `PlayerStatOf(It, Life)` and folds per `AggregateOp` — the
    /// player-sourced twin of `aggregate_folds_a_projection_over_a_selection`'s
    /// object-sourced coverage above. `MaxOf` reads the higher of the two
    /// players' life totals ("the highest life total among all players");
    /// every `AggregateOp` folds an EMPTY player set to 0 (never-crash) —
    /// exercised on a real `Countable::Players` source, not just the object
    /// analog, since `Iterator::min`/`max`'s `None` case is the concrete
    /// panic risk (`.unwrap()` on an empty iterator) the never-crash ruling
    /// guards against.
    #[test]
    fn player_aggregate_folds_life_totals_and_fizzles_to_zero_on_empty() {
        use deckmaste_core::AggregateOp;
        use deckmaste_core::ObjectKind;
        use deckmaste_core::PlayerAttr;
        use deckmaste_core::Projection;

        let mut state = game();
        let src = permanent_with_cost(&mut state, "{1}");
        let frame = frame_src(src);
        state.player_mut(PlayerId(0)).life = 12;
        state.player_mut(PlayerId(1)).life = 20;

        let all_players = Predicate::Kind(ObjectKind::Player);
        let highest_life = |op: AggregateOp| {
            Count::Aggregate(
                op,
                Projection {
                    of: Countable::Players(Box::new(all_players.clone())),
                    by: Box::new(Count::PlayerStatOf(Reference::It, PlayerAttr::Life)),
                },
            )
        };
        assert_eq!(
            state.eval_count(&highest_life(AggregateOp::MaxOf), &frame),
            20,
            "the highest life total among all players"
        );
        assert_eq!(
            state.eval_count(&highest_life(AggregateOp::MinOf), &frame),
            12,
            "the lowest life total among all players"
        );
        assert_eq!(
            state.eval_count(&highest_life(AggregateOp::SumOf), &frame),
            32,
            "the total life across all players"
        );

        // An empty player set (an authoring mistake, but must stay safe on
        // ANY input, not just the real ≥1-player fixtures) folds every op to
        // 0 rather than panicking on `Iterator::min`/`max` of an empty set.
        let no_players = Predicate::Not(Box::new(Predicate::Any));
        for op in [
            AggregateOp::SumOf,
            AggregateOp::MinOf,
            AggregateOp::MaxOf,
            AggregateOp::AverageOf(deckmaste_core::RoundMode::RoundUp),
        ] {
            let empty_fold = Count::Aggregate(
                op,
                Projection {
                    of: Countable::Players(Box::new(no_players.clone())),
                    by: Box::new(Count::PlayerStatOf(Reference::It, PlayerAttr::Life)),
                },
            );
            assert_eq!(
                state.eval_count(&empty_fold, &frame),
                0,
                "{op:?} over the empty player set is 0"
            );
        }
    }

    /// The player-scope stat predicate ([CR#119.1]): `CountOf(Players(
    /// PlayerStatCmp(Life, AtMost, N)))` counts players whose life total is
    /// at most `N` — "the number of players with 13 or less life". Threshold
    /// 13 catches only the 10-life player; threshold 5 catches neither;
    /// threshold 20 catches both. Also exercises the predicate's
    /// player-proxy-only match: the fixture mints a permanent (`src`) too, so
    /// a wrongly-matching non-player object would inflate the count.
    #[test]
    fn players_countable_counts_by_life_threshold() {
        use deckmaste_core::Cmp;
        use deckmaste_core::PlayerAttr;

        let mut state = game();
        let src = permanent_with_cost(&mut state, "{1}");
        let frame = frame_src(src);
        state.player_mut(PlayerId(0)).life = 20;
        state.player_mut(PlayerId(1)).life = 10;

        let count_at_most = |threshold| {
            Count::CountOf(Countable::Players(Box::new(Predicate::PlayerStatCmp(
                PlayerAttr::Life,
                Cmp::AtMost,
                Count::Literal(threshold),
            ))))
        };

        assert_eq!(
            state.eval_count(&count_at_most(13), &frame),
            1,
            "only the 10-life player has 13 or less life"
        );
        assert_eq!(
            state.eval_count(&count_at_most(5), &frame),
            0,
            "neither player has 5 or less life"
        );
        assert_eq!(
            state.eval_count(&count_at_most(20), &frame),
            2,
            "both players have at most 20 life"
        );
    }

    /// Devotion end-to-end ([CR#700.5]), the two cases
    /// `aggregate_folds_a_projection_over_a_selection` doesn't already cover:
    /// a two-color disjunction (`Or([White, Black])`) summed across SEPARATE
    /// permanents (not just one object's multiple pips), and an actually
    /// empty battlefield (no permanents minted at all, not a `Not(Any)`
    /// filter trick).
    #[test]
    fn devotion_sums_a_color_disjunction_across_permanents_and_fizzles_to_zero_on_empty() {
        use deckmaste_core::AggregateOp;
        use deckmaste_core::Color;
        use deckmaste_core::Projection;
        use deckmaste_core::RelationPredicate;
        use deckmaste_core::StatePredicate;
        use deckmaste_core::SymbolPred;

        let your_permanents = || {
            Predicate::And(vec![
                Predicate::State(StatePredicate::InZone(Zone::Battlefield)),
                Predicate::Relation(RelationPredicate::ControlledBy(Box::new(Predicate::Ref(
                    Reference::You,
                )))),
            ])
        };
        let devotion_wb = |of| {
            Count::Aggregate(
                AggregateOp::SumOf,
                Projection {
                    of,
                    by: Box::new(Count::CountOf(Countable::ManaSymbols(
                        Box::new(Reference::It),
                        SymbolPred::Or(vec![
                            SymbolPred::CountsAs(Color::White),
                            SymbolPred::CountsAs(Color::Black),
                        ]),
                    ))),
                },
            )
        };

        // `{W}{B}{W/B}` split across three separate permanents you control =
        // 1 + 1 + 1 = 3 (the hybrid pip counts toward both W and B devotion,
        // but only once per object — `Or` matches, it doesn't double-count).
        let mut state = game();
        let _ = permanent_with_cost(&mut state, "{W}");
        let _ = permanent_with_cost(&mut state, "{B}");
        let src = permanent_with_cost(&mut state, "{W/B}");
        let frame = frame_src(src);
        assert_eq!(
            state.eval_count(
                &devotion_wb(Countable::Objects(Box::new(your_permanents()))),
                &frame
            ),
            3,
            "{{W}} + {{B}} + {{W/B}} = 3 devotion to white-and-black"
        );

        // An empty battlefield — no permanents at all, not an artificial
        // never-matching filter — folds to 0 (never-crash). `src` itself
        // lives in hand, so `InZone(Battlefield)` matches nothing.
        let mut empty_state = game();
        let card = Card::Normal(CardFace {
            name: "Test Card".into(),
            mana_cost: "{1}".parse().unwrap(),
            types: vec![Type::Artifact.def()],
            ..CardFace::default()
        });
        let cid = empty_state.cards.push(Arc::new(card), PlayerId(0));
        let src = empty_state
            .objects
            .mint(ObjectSource::Card(cid), PlayerId(0), Some(Zone::Hand));
        let frame = frame_src(src);
        assert_eq!(
            empty_state.eval_count(
                &devotion_wb(Countable::Objects(Box::new(your_permanents()))),
                &frame
            ),
            0,
            "no permanents on the battlefield → devotion 0"
        );
    }

    /// `StatOf` reads the DERIVED stat (a pump shows through) and the
    /// printed mana value ([CR#202.3]).
    #[test]
    fn stat_of_reads_derived_stats() {
        let (mut state, bear) = bear_on_field();
        let frame = frame_src_targets(bear, vec![bear]);

        let power = Count::StatOf(Reference::It, deckmaste_core::Stat::Power);
        assert_eq!(state.eval_count(&power, &frame), 2);
        assert_eq!(
            state.eval_count(
                &Count::StatOf(Reference::This, deckmaste_core::Stat::ManaValue),
                &frame
            ),
            2,
            "Grizzly Bears costs {{1}}{{G}}"
        );

        // A +1/+0 continuous effect shows the read rides the layer view.
        let timestamp = state.objects.next_timestamp();
        state.continuous.push(crate::layer::ContinuousEffect {
            timestamp,
            controller: PlayerId(0),
            scope: crate::layer::ScopeResolved::Locked(vec![bear]),
            changes: vec![deckmaste_core::Modification::Power(
                deckmaste_core::NumericOp::Up(Count::Literal(1)),
            )],
            duration: deckmaste_core::Duration::EndOfGame,
            rows: vec![],
            origin: None,
            is_cda: false,
        });
        assert_eq!(state.eval_count(&power, &frame), 3);
    }

    /// "That much" reads the amount the damage instruction fixed: the two
    /// instructions run through the agenda, the `DamageDealt` apply records
    /// 3, and the later `GainLife(ThatMuch)` evaluation reads it back.
    #[test]
    fn that_much_gains_life_equal_to_damage_dealt() {
        let (mut state, bear) = bear_on_field();
        let frame = frame_src_targets(bear, vec![bear]);
        state.run_effect(
            OneShotEffect::Sequentially(vec![
                OneShotEffect::Act(Action::deal_damage(Reference::It, Count::Literal(3))),
                OneShotEffect::act_by_you(PlayerAction::GainLife(Count::ThatMuch)),
            ]),
            &frame,
        );
        // RunEffect(damage) → Emit(DamageDealt) → RunEffect(gain) → Emit(LifeGained).
        for _ in 0..4 {
            let _ = state.step();
        }
        assert_eq!(state.objects.obj(bear).total_damage(), 3);
        assert_eq!(state.players[0].life, 23);
    }

    /// A `Targeted` wrapper is transparent at resolution — the inner
    /// instruction runs with `frame.anaphora.targets` already bound, so
    /// `Target(0)` resolves and damage lands ([CR#115.1,608]).
    #[test]
    fn targeted_effect_resolves_its_inner_effect() {
        let (mut state, bear) = bear_on_field();
        let frame = frame_src_targets(bear, vec![bear]);
        state.run_effect(
            OneShotEffect::Targeted(deckmaste_core::Targeted::new(
                vec![],
                OneShotEffect::Act(Action::deal_damage(Reference::It, Count::Literal(3))),
            )),
            &frame,
        );
        // RunEffect(Targeted) → RunEffect(DealDamage) → Emit(DamageDealt).
        for _ in 0..3 {
            let _ = state.step();
        }
        assert_eq!(state.objects.obj(bear).total_damage(), 3);
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
            vec![spec.clone()],
            OneShotEffect::Act(Action::deal_damage(Reference::It, Count::Literal(3))),
        ));
        assert_eq!(super::top_targets(&wrapped), std::slice::from_ref(&spec));
        let bare = OneShotEffect::Act(Action::deal_damage(Reference::It, Count::Literal(1)));
        assert!(super::top_targets(&bare).is_empty());
    }

    #[test]
    fn count_x_reads_announced_value() {
        let (state, src) = bear_on_field();
        let frame = Frame {
            anaphora: Anaphora {
                x: Some(3),
                ..Anaphora::empty()
            },
            ..Frame::bare(src, PlayerId(0))
        };
        assert_eq!(state.eval_count(&Count::X, &frame), 3);
    }

    #[test]
    fn each_creature_yields_all_battlefield_creatures() {
        let (mut state, a) = bear_on_field();
        // Force a second Grizzly Bears from player 0's hand onto the battlefield.
        let b = *state.zones.hands[0]
            .iter()
            .find(|&&o| {
                obj_matches(
                    &state,
                    o,
                    &Predicate::Characteristic(deckmaste_core::CharacteristicPredicate::Type(
                        Type::Creature.name(),
                    )),
                )
            })
            .expect("a second Grizzly Bears in the opening hand");
        state.zones.hands[PlayerId(0).index()].retain(|&o| o != b);
        state.objects.obj_mut(b).zone = Some(Zone::Battlefield);
        state.zones.battlefield.push(b);

        let frame = frame_src(a);
        let filter = Predicate::And(vec![
            Predicate::State(StatePredicate::InZone(Zone::Battlefield)),
            Predicate::creature(),
        ]);
        let mut got = state.eval_selection_set(&Selection::SelectAll(filter), &frame);
        got.sort();
        let mut want = vec![a, b];
        want.sort();
        assert_eq!(got, want);
    }

    /// `Each(Kind(Player), DealDamage(This, 20, It))` deals 20 damage to
    /// each of the two players. A verb's patient is a single `Reference`, so
    /// the spread over the player set is the enclosing `Each` — one
    /// `DamageDealt` per element rather than the old single multi-target
    /// `Batch` ([CR#608.2,120.1]).
    #[test]
    fn each_player_deal_damage_hits_both_players() {
        let (mut state, src) = bear_on_field();
        let frame = frame_src(src);

        let effect = OneShotEffect::Each(deckmaste_core::Each {
            binder: Binder::Existing(Selection::SelectAll(Predicate::Kind(ObjectKind::Player))),
            effect: Box::new(OneShotEffect::Act(Action::deal_damage(
                Reference::It,
                Count::Literal(20),
            ))),
        });
        state.run_effect(effect, &frame);

        // Collect every DamageDealt across the per-element runs.
        let p0_obj = state.players[0].object;
        let p1_obj = state.players[1].object;
        let mut got = collect_damage_dealt(&mut state, 40);
        got.sort();
        let mut want = vec![(p0_obj, 20u32), (p1_obj, 20u32)];
        want.sort();
        assert_eq!(got, want, "each player takes 20 damage, one event apiece");
    }

    /// `Each(And([InZone(Battlefield), Type(Creature)]), DealDamage(
    /// It, 2))` deals 2 damage to each of the two battlefield creatures
    /// — one `DamageDealt` per iterated element (the verb's patient is a single
    /// `Reference`).
    #[test]
    fn each_creature_deal_damage_hits_both_creatures() {
        let (mut state, a) = bear_on_field();
        // Force a second creature onto the battlefield.
        let b = *state.zones.hands[0]
            .iter()
            .find(|&&o| {
                obj_matches(
                    &state,
                    o,
                    &Predicate::Characteristic(deckmaste_core::CharacteristicPredicate::Type(
                        Type::Creature.name(),
                    )),
                )
            })
            .expect("a second Grizzly Bears in the opening hand");
        state.zones.hands[PlayerId(0).index()].retain(|&o| o != b);
        state.objects.obj_mut(b).zone = Some(Zone::Battlefield);
        state.zones.battlefield.push(b);

        let frame = frame_src(a);
        let effect = OneShotEffect::Each(deckmaste_core::Each {
            binder: Binder::Existing(Selection::SelectAll(Predicate::And(vec![
                Predicate::State(StatePredicate::InZone(Zone::Battlefield)),
                Predicate::creature(),
            ]))),
            effect: Box::new(OneShotEffect::Act(Action::deal_damage(
                Reference::It,
                Count::Literal(2),
            ))),
        });
        state.run_effect(effect, &frame);

        // Simultaneity ([CR#700.1]): a single-verb `Each` body resolves for
        // every element at once — both `DamageDealt`s ride ONE `Occurrence::Batch`
        // (not two sequential singles), so death triggers / SBAs see them
        // together. This is the "to each" simultaneity the old verb-over-`Predicate`
        // carried, restored after the verb→`Reference` split.
        match state.agenda.front() {
            Some(WorkItem::Emit(crate::event::Occurrence::Batch(evs))) => {
                assert_eq!(evs.len(), 2, "both hits land in one simultaneous batch");
            }
            other => panic!("expected one simultaneous Emit(Batch), got {other:?}"),
        }

        // Both creatures took 2 damage — one DamageDealt per iterated element.
        let mut got = collect_damage_dealt(&mut state, 40);
        got.sort();
        let mut want = vec![(a, 2u32), (b, 2u32)];
        want.sort();
        assert_eq!(got, want);
    }

    #[test]
    fn each_over_choice_bearing_body_schedules_its_work_items() {
        // Regression: the `Each` single-`Act` batch path must NOT batch a
        // choice-bearing body. `By(You, Discard)` yields a `DiscardCards` work
        // item (not an `Emit`); the old code kept only `Emit`s, so the discards
        // were silently dropped ("each player discards a card" would no-op). Here
        // the body must instead be scheduled, one item per element. (Synthetic
        // "for each creature, you discard a card" shape — chosen to exercise the
        // non-`Emit` seam without standing up a player-matching `over`.)
        let (mut state, a) = bear_on_field();
        let b = *state.zones.hands[0]
            .iter()
            .find(|&&o| {
                obj_matches(
                    &state,
                    o,
                    &Predicate::Characteristic(deckmaste_core::CharacteristicPredicate::Type(
                        Type::Creature.name(),
                    )),
                )
            })
            .expect("a second Grizzly Bears in the opening hand");
        state.zones.hands[PlayerId(0).index()].retain(|&o| o != b);
        state.objects.obj_mut(b).zone = Some(Zone::Battlefield);
        state.zones.battlefield.push(b);

        let frame = frame_src(a);
        let effect = OneShotEffect::Each(deckmaste_core::Each {
            binder: Binder::Existing(Selection::SelectAll(Predicate::And(vec![
                Predicate::State(StatePredicate::InZone(Zone::Battlefield)),
                Predicate::creature(),
            ]))),
            effect: Box::new(OneShotEffect::act_by_you(PlayerAction::Discard {
                count: Count::Literal(1),
                what: None,
                random: false,
            })),
        });
        state.run_effect(effect, &frame);

        // Each element's choice-bearing item is scheduled — not folded into (and
        // lost by) a simultaneous `Emit` batch.
        let discards = state
            .agenda
            .iter()
            .filter(|item| matches!(item, WorkItem::DiscardCards { .. }))
            .count();
        assert_eq!(
            discards, 2,
            "each element's discard work item is scheduled, not dropped"
        );
        assert!(
            !matches!(state.agenda.front(), Some(WorkItem::Emit(_))),
            "a choice-bearing Each body must not collapse to a simultaneous Emit batch"
        );
    }

    /// [CR#608.2] "[body], [count] times": a plain (non-choice) body runs the
    /// evaluated count total — `count` is read ONCE, up front (`eval_count`),
    /// never re-evaluated mid-loop.
    #[test]
    fn repeat_runs_a_plain_body_count_times() {
        let mut state = game();
        let p0 = PlayerId(0);
        let frame = frame_for(&state, p0);
        let life0 = state.player(p0).life;

        let body = OneShotEffect::act_by_you(PlayerAction::GainLife(Count::Literal(2)));
        state.run_effect(
            OneShotEffect::Repeat(Count::Literal(3), Box::new(body)),
            &frame,
        );
        let _ = drain_progress(&mut state, 40);

        assert_eq!(
            state.player(p0).life,
            life0 + 6,
            "three iterations of +2 life = +6 total"
        );
    }

    /// A count of zero schedules nothing — never a panic, never a hang; the
    /// CRITICAL never-crash ruling applies to a degenerate `Repeat` exactly
    /// as it does to every other new arm.
    #[test]
    fn repeat_zero_is_a_no_op() {
        let mut state = game();
        let p0 = PlayerId(0);
        let frame = frame_for(&state, p0);
        let life0 = state.player(p0).life;
        let agenda_before = state.agenda.len();

        let body = OneShotEffect::act_by_you(PlayerAction::GainLife(Count::Literal(2)));
        state.run_effect(
            OneShotEffect::Repeat(Count::Literal(0), Box::new(body)),
            &frame,
        );

        assert_eq!(
            state.agenda.len(),
            agenda_before,
            "count=0 schedules no ADDITIONAL work (a fresh game already has its own \
             turn-structure agenda queued, so this compares the delta, not raw emptiness)"
        );
        let _ = drain_progress(&mut state, 5);
        assert_eq!(state.player(p0).life, life0, "no iterations ran");
    }

    /// A choice-bearing body (`May`) surfaces its OWN decision on EVERY
    /// repetition — never auto-resolved (the engine-steppable ruling): the
    /// second iteration's yes/no is not decided until the first iteration's
    /// answer has actually been submitted and applied.
    #[test]
    fn repeat_over_a_choice_bearing_body_steps_each_iteration_independently() {
        use crate::decide::Decision;
        use crate::decide::PendingDecision;

        // Pump steps until a decision surfaces (a `RunEffect` work item's own
        // `step()` call only *sets* `self.pending` as a side effect and
        // returns `Progress::Resolving` for that step; `NeedsDecision` is
        // reported on the NEXT `step()` call, which sees `pending` already
        // set — so this may take more than one `step()`).
        fn step_to_decision(state: &mut GameState) -> crate::decide::PendingDecision {
            loop {
                match state.step() {
                    StepOutcome::NeedsDecision(d) => return d,
                    StepOutcome::Progress(_) => {}
                    StepOutcome::GameOver(o) => panic!("unexpected game over: {o:?}"),
                }
            }
        }

        let mut state = game();
        let p0 = PlayerId(0);
        let frame = frame_for(&state, p0);
        let life0 = state.player(p0).life;

        let may_gain_3 = || {
            OneShotEffect::May(deckmaste_core::May {
                effect: Box::new(OneShotEffect::act_by_you(PlayerAction::GainLife(
                    Count::Literal(3),
                ))),
                if_did: None,
                if_not: None,
            })
        };
        state.run_effect(
            OneShotEffect::Repeat(Count::Literal(2), Box::new(may_gain_3())),
            &frame,
        );

        // First iteration's decision.
        let PendingDecision::YesNo { player } = step_to_decision(&mut state) else {
            panic!("expected the first iteration's YesNo");
        };
        assert_eq!(player, p0);
        state.submit_decision(Decision::Answer(true)).unwrap();
        let _ = drain_progress(&mut state, 10);
        assert_eq!(
            state.player(p0).life,
            life0 + 3,
            "first iteration applied alone"
        );

        // Second iteration's OWN decision — not skipped, not pre-answered.
        let PendingDecision::YesNo { player } = step_to_decision(&mut state) else {
            panic!("expected the second iteration's own YesNo");
        };
        assert_eq!(player, p0);
        state.submit_decision(Decision::Answer(true)).unwrap();
        let _ = drain_progress(&mut state, 10);
        assert_eq!(state.player(p0).life, life0 + 6, "both iterations applied");
    }

    /// A saturated/huge count must never eagerly allocate — the CRITICAL
    /// never-crash/never-hang ruling applies exactly as much to a mistaken
    /// `Repeat(Literal(4_000_000_000), body)` as to a panic: `eval_count`
    /// has no clamp (arithmetic `Count`s *saturate* toward `u32::MAX`), so
    /// the lazy self-rescheduling continuation must schedule exactly TWO
    /// work items per step — this iteration's `body` plus a
    /// `Repeat(Literal(n - 1), body)` tail — never `n` of them.
    #[test]
    fn repeat_with_huge_count_does_not_eagerly_allocate() {
        let mut state = game();
        let p0 = PlayerId(0);
        let frame = frame_for(&state, p0);
        let agenda_before = state.agenda.len();

        let body = OneShotEffect::act_by_you(PlayerAction::GainLife(Count::Literal(1)));
        state.run_effect(
            OneShotEffect::Repeat(Count::Literal(1_000_000), Box::new(body)),
            &frame,
        );

        assert_eq!(
            state.agenda.len(),
            agenda_before + 2,
            "a single Repeat step schedules exactly two work items — this iteration's body \
             plus a Repeat(n-1, body) tail continuation — never count-many eagerly \
             materialized RunEffect items"
        );
        match (&state.agenda[0], &state.agenda[1]) {
            (
                WorkItem::RunEffect { effect: first, .. },
                WorkItem::RunEffect { effect: second, .. },
            ) => {
                assert!(
                    matches!(**first, OneShotEffect::Act(_)),
                    "the front item is this iteration's own body, not another Repeat layer"
                );
                match &**second {
                    OneShotEffect::Repeat(Count::Literal(n), _) => {
                        assert_eq!(
                            *n, 999_999,
                            "the tail carries the DECREMENTED remaining count"
                        );
                    }
                    other => panic!("expected a Repeat(Literal(n - 1), body) tail, got {other:?}"),
                }
            }
            other => panic!("expected two RunEffect work items at the agenda front, got {other:?}"),
        }
    }

    /// [CR#702.85,701.57] `RevealUntil` fizzles to a graceful no-op — the
    /// Reveal seam (`PlayerAction::Reveal`/`GameEvent::Revealed`) is
    /// genuinely unbuilt (see the doc comment on this arm in `run_effect`),
    /// so `body` never runs and nothing is scheduled — never a panic,
    /// matching the CRITICAL never-crash ruling.
    #[test]
    fn reveal_until_fizzles_to_a_no_op() {
        let (mut state, a) = bear_on_field();
        let frame = frame_src(a);
        let life0 = state.player(PlayerId(0)).life;
        let agenda_before = state.agenda.len();

        let effect = OneShotEffect::RevealUntil(deckmaste_core::RevealUntil {
            whose: Reference::You,
            matches: Predicate::creature(),
            body: Box::new(OneShotEffect::act_by_you(PlayerAction::GainLife(
                Count::Literal(99),
            ))),
        });
        state.run_effect(effect, &frame);

        assert_eq!(
            state.agenda.len(),
            agenda_before,
            "RevealUntil schedules no ADDITIONAL work — the absent-subsystem no-op (compares \
             the delta, since a live game already has its own turn-structure agenda queued)"
        );
        let _ = drain_progress(&mut state, 5);
        assert_eq!(state.player(PlayerId(0)).life, life0, "body never runs");
    }

    /// The Blood-Money shape ([CR#607.2a] fact-backed product groups): a
    /// `Noting`-wrapped destroy-all over three creatures, one of which
    /// can't be destroyed — "destroyed this way" is exactly the clause's
    /// enacted destroy-caused `ZoneChanged` facts, so the survivor is
    /// excluded BY CONSTRUCTION (its `WillDestroy` was canted; no move
    /// fact exists), and the two dies-facts share one history batch id
    /// ([CR#603.3b]).
    #[test]
    fn destroyed_this_way_product_group_excludes_indestructible_survivor() {
        let (mut state, a, b) = two_permanents_on_field();
        // The indestructible shape ([CR#702.12b] — destruction can't
        // happen), as the canted static.
        let survivor = {
            let source = "Normal(name: \"Darksteel Test\", types: [Creature], abilities: [\
                 Static(CantHappen(ZoneChange(what: Ref(This), \
                 from: Battlefield, to: Graveyard)))])";
            let card = builtin()
                .macros
                .read_str::<deckmaste_core::Card>(source)
                .unwrap();
            mint_on_field(&mut state, card)
        };

        let effect = OneShotEffect::Noting(deckmaste_core::Noting {
            key: "destroyed".into(),
            effect: Box::new(OneShotEffect::Each(deckmaste_core::Each {
                binder: deckmaste_core::Binder::Existing(Selection::SelectAll(Predicate::And(
                    vec![
                        Predicate::State(deckmaste_core::StatePredicate::InZone(Zone::Battlefield)),
                        Predicate::creature(),
                    ],
                ))),
                effect: Box::new(OneShotEffect::Act(Action::Destroy(Reference::It))),
            })),
        });
        let frame = frame_src(a);
        state.run_effect(effect, &frame);
        run_injected(&mut state);

        assert!(
            state.objects.get(survivor).is_some()
                && state.objects.obj(survivor).zone == Some(Zone::Battlefield),
            "the can't-be-destroyed creature survived"
        );
        let group = &state.noted[&deckmaste_core::Ident::from("destroyed")];
        assert_eq!(
            group.len(),
            2,
            "the product group is the ENACTED destroy facts, not the gathered set"
        );
        let members: Vec<ObjectId> = group.iter().map(|m| m.snapshot.object).collect();
        assert!(
            members.contains(&a) && members.contains(&b),
            "exactly the two destroyed creatures, by LKI"
        );
        // The dies-facts committed as ONE batch ([CR#603.3b]).
        let ids: Vec<Option<deckmaste_core::Uint>> = state
            .history
            .entries()
            .filter(|e| matches!(e.fact, GameEvent::ZoneChanged { .. }))
            .map(|e| e.batch)
            .collect();
        assert_eq!(ids.len(), 2);
        assert!(ids[0].is_some() && ids[0] == ids[1], "one shared batch id");
    }

    /// "Cards milled this way" ([CR#701.17a,701.17c,607.2a]): a
    /// `Noting`-wrapped mill commits the three moves as ONE cause-carried
    /// batch, populates the product group from the enacted facts, and a
    /// following clause ACTS on the group through `AmongNoted` — exiling
    /// exactly the milled cards.
    #[test]
    fn cards_milled_this_way_reads_the_enacted_product_group() {
        let (mut state, a) = bear_on_field();
        let libsize = state.zones.libraries[0].len();
        assert!(libsize >= 3, "the harness deck has cards to mill");

        let effect = OneShotEffect::Sequentially(vec![
            OneShotEffect::Noting(deckmaste_core::Noting {
                key: "milled".into(),
                effect: Box::new(OneShotEffect::act_by_you(PlayerAction::Mill(
                    Count::Literal(3),
                ))),
            }),
            OneShotEffect::Each(deckmaste_core::Each {
                binder: deckmaste_core::Binder::Existing(Selection::AmongNoted(
                    "milled".into(),
                    deckmaste_core::Quantity::Range(None, None),
                )),
                effect: Box::new(OneShotEffect::Act(Action::Move(
                    Reference::It,
                    deckmaste_core::Destination::Zone(Zone::Exile),
                    vec![],
                ))),
            }),
        ]);
        let frame = frame_src(a);
        state.run_effect(effect, &frame);
        run_injected(&mut state);

        let group = &state.noted[&deckmaste_core::Ident::from("milled")];
        assert_eq!(group.len(), 3, "three enacted mill facts");
        assert!(
            logged(&state, |e| matches!(
                e,
                GameEvent::ZoneChanged { cause: Some(c), to: Zone::Graveyard, .. }
                    if c.verb.as_str() == "Mill"
            )),
            "the moves carry the Mill cause ([CR#701.17a])"
        );
        assert_eq!(
            state.zones.libraries[0].len(),
            libsize - 3,
            "three cards left the library"
        );
        assert_eq!(
            state.zones.exile.len(),
            3,
            "the follow-on clause exiled exactly the cards milled this way"
        );
        assert!(
            state.zones.graveyards[0].is_empty(),
            "the milled cards moved on from the graveyard"
        );
    }

    // --- P0.W5 resolution note slots ([CR#608.2c,607.2]) --------------------

    /// [CR#608.2c,607.2] the resolution note store round-trips a NUMBER choice:
    /// `ChooseAndNote(Number)` surfaces a resolution-time number decision; the
    /// submitted value lands in `resolution_notes`; a LATER clause of the SAME
    /// resolution reads it back via `Count::Noted`. The two clauses run as
    /// separate `Sequentially` children — each carrying its OWN frame clone
    /// (`Sequentially` clones the frame per child up front) — so this asserts
    /// the note rides the resolution-scoped STORE, not the frame (the
    /// cloned-frame trap the store exists to avoid).
    #[test]
    fn choose_and_note_number_round_trips_through_the_store() {
        use crate::decide::Decision;
        use crate::decide::PendingDecision;

        let (mut state, a) = bear_on_field();
        let key = deckmaste_core::Ident::from("n");
        let effect = OneShotEffect::Sequentially(vec![
            OneShotEffect::act_by_you(PlayerAction::ChooseAndNote(
                key,
                deckmaste_core::NotedKind::Number,
            )),
            OneShotEffect::act_by_you(PlayerAction::Mill(Count::Noted(key))),
        ]);
        let frame = frame_src(a);
        state.run_effect(effect, &frame);
        run_injected(&mut state);

        // The first child surfaced the number choice.
        let Some(PendingDecision::ChooseNoteNumber { player, key: pk }) = state.pending.clone()
        else {
            panic!("expected ChooseNoteNumber, got {:?}", state.pending);
        };
        assert_eq!(
            player,
            PlayerId(0),
            "the effect's controller notes the number"
        );
        assert_eq!(pk, key);

        state
            .submit_decision(Decision::XValue(2))
            .expect("a non-negative note number is always legal");
        assert_eq!(
            state.resolution_notes.get(&key),
            Some(&crate::state::NotedValue::Number(2)),
            "the submitted value is stored under the note key"
        );

        // The SECOND child reads it back: Mill(Noted(key)) mills exactly 2.
        run_injected(&mut state);
        assert_eq!(
            state.zones.graveyards[0].len(),
            2,
            "Count::Noted read 2 from the store in the second child's frame clone"
        );
    }

    /// [CR#608.2c] scope: a resolution note lives ONLY within its resolution.
    /// A note written in one resolution is GONE when the next begins —
    /// `resolve_object` clears `resolution_notes` at the fresh-resolution
    /// boundary — so a `Count::Noted` read in the next resolution finds nothing
    /// and fizzles to 0 (never a stale value, never a panic). Mirrors
    /// `moved_chain_resets_when_a_resolution_begins`.
    #[test]
    fn resolution_notes_clear_when_a_fresh_resolution_begins() {
        use deckmaste_core::CardFace;
        use deckmaste_core::StatValue;

        let (mut state, _a) = bear_on_field();
        let key = deckmaste_core::Ident::from("n");
        state
            .resolution_notes
            .insert(key, crate::state::NotedValue::Number(5));

        // Drive a REAL resolution: mint a vanilla creature spell, push its
        // `StackEntry`, and resolve it.
        let card = Card::Normal(CardFace {
            name: "Test Bear".into(),
            types: vec![Type::Creature.def()],
            power: Some(StatValue::Number(2)),
            toughness: Some(StatValue::Number(2)),
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
            state.resolution_notes.is_empty(),
            "a fresh resolution clears the note store ([CR#608.2c])"
        );
        // The consuming read fizzles to 0 (not the stale 5), never panics.
        let frame = frame_src(spell);
        assert_eq!(
            state.eval_count(&Count::Noted(key), &frame),
            0,
            "Count::Noted on a cleared note fizzles to 0"
        );
    }

    /// [CR#607.2a,608.2d] `ChooseAndNote(Objects)`: the picks are recorded into
    /// the fact-backed `noted` product group (live members), and
    /// `Selection::AmongNoted` reads them back. The grammar carries no
    /// narrowing predicate, so the chooser's domain is the battlefield-wide
    /// default (choose any number).
    #[test]
    fn choose_and_note_objects_writes_group_read_by_among_noted() {
        use crate::decide::Decision;
        use crate::decide::PendingDecision;

        let (mut state, a, _b) = two_permanents_on_field();
        let key = deckmaste_core::Ident::from("chosen");
        let effect = OneShotEffect::act_by_you(PlayerAction::ChooseAndNote(
            key,
            deckmaste_core::NotedKind::Objects,
        ));
        let frame = frame_src(a);
        state.run_effect(effect, &frame);
        run_injected(&mut state);

        let Some(PendingDecision::ChooseObjects {
            player,
            candidates,
            min,
            max,
        }) = state.pending.clone()
        else {
            panic!("expected ChooseObjects, got {:?}", state.pending);
        };
        assert_eq!(player, PlayerId(0));
        assert_eq!(
            (min, max),
            (0, 2),
            "battlefield-wide domain, choose any number of the two permanents"
        );
        assert!(candidates.contains(&a));

        state
            .submit_decision(Decision::Chosen(vec![a]))
            .expect("a is a battlefield candidate");

        // The pick is recorded into the `noted` group, live.
        let group = &state.noted[&key];
        assert_eq!(group.len(), 1);
        assert_eq!(group[0].now, Some(a));

        // AmongNoted (unconstrained) reads the whole live group back.
        assert_eq!(
            state.eval_selection_set(
                &Selection::AmongNoted(key, deckmaste_core::Quantity::Range(None, None)),
                &frame,
            ),
            vec![a],
            "AmongNoted reads the noted objects back"
        );
    }

    /// [CR#607.2a,608.2d] a CONSTRAINING `AmongNoted` quantity ("destroy one of
    /// them") surfaces a `ChooseObjects` chooser over the noted group's LIVE
    /// members, honors the quantity's bounds, and binds the picks into the
    /// re-run body — exactly the chooser the unconstrained full-group read does
    /// not need.
    #[test]
    fn among_noted_constrained_quantity_surfaces_and_binds_chooser() {
        use crate::decide::Decision;
        use crate::decide::PendingDecision;

        let (mut state, a, b) = two_permanents_on_field();
        let key = deckmaste_core::Ident::from("grp");
        // Seed the noted product group with both live permanents.
        let ma = crate::state::NotedMember {
            snapshot: crate::lki::LkiSnapshot::capture(&state, a),
            now: Some(a),
        };
        let mb = crate::state::NotedMember {
            snapshot: crate::lki::LkiSnapshot::capture(&state, b),
            now: Some(b),
        };
        state.noted.insert(key, vec![ma, mb]);

        // "Destroy exactly one of them" — a constraining AmongNoted quantity.
        let effect = OneShotEffect::Each(deckmaste_core::Each {
            binder: Binder::Existing(Selection::AmongNoted(
                key,
                deckmaste_core::Quantity::Range(Some(Count::Literal(1)), Some(Count::Literal(1))),
            )),
            effect: Box::new(OneShotEffect::Act(Action::Destroy(Reference::It))),
        });
        let frame = frame_src(a);
        state.run_effect(effect, &frame);

        let Some(PendingDecision::ChooseObjects {
            player,
            candidates,
            min,
            max,
        }) = state.pending.clone()
        else {
            panic!("expected ChooseObjects, got {:?}", state.pending);
        };
        assert_eq!(
            player,
            PlayerId(0),
            "the controller chooses (AmongNoted has no `by`)"
        );
        assert_eq!((min, max), (1, 1), "exactly one, from the quantity bounds");
        let mut got = candidates.clone();
        got.sort();
        let mut want = vec![a, b];
        want.sort();
        assert_eq!(got, want, "candidates = the noted group's live members");

        // Bind the pick and run the body: exactly `a` is destroyed.
        state
            .submit_decision(Decision::Chosen(vec![a]))
            .expect("a is a live member");
        run_injected(&mut state);
        assert!(
            !state.zones.battlefield.contains(&a),
            "the chosen member was destroyed"
        );
        assert!(
            state.zones.battlefield.contains(&b),
            "the un-chosen member is untouched"
        );
    }

    /// [CR#607.2,608.2c] a `Count::Noted` read of an ABSENT key is an authoring
    /// mistake — it fizzles to 0 (engine-stat-none-fizzle), never a panic and
    /// never a silent-but-plausible value.
    #[test]
    fn count_noted_missing_key_fizzles_to_zero() {
        let (state, a) = bear_on_field();
        let frame = frame_src(a);
        assert_eq!(
            state.eval_count(&Count::Noted(deckmaste_core::Ident::from("absent")), &frame),
            0
        );
    }

    /// [CR#607.2] write-only note kinds with NO reader grammar stay LOUD
    /// per-kind: `Color`/`CardName`/`Piles` trip a labeled `todo!` seam rather
    /// than wire dead machinery.
    #[test]
    #[should_panic(expected = "ChooseAndNote(Color)")]
    fn choose_and_note_color_is_a_loud_seam() {
        let (mut state, a) = bear_on_field();
        let frame = frame_src(a);
        state.run_effect(
            OneShotEffect::act_by_you(PlayerAction::ChooseAndNote(
                deckmaste_core::Ident::from("c"),
                deckmaste_core::NotedKind::Color,
            )),
            &frame,
        );
    }

    #[test]
    #[should_panic(expected = "ChooseAndNote(CardName)")]
    fn choose_and_note_card_name_is_a_loud_seam() {
        let (mut state, a) = bear_on_field();
        let frame = frame_src(a);
        state.run_effect(
            OneShotEffect::act_by_you(PlayerAction::ChooseAndNote(
                deckmaste_core::Ident::from("cn"),
                deckmaste_core::NotedKind::CardName,
            )),
            &frame,
        );
    }

    #[test]
    #[should_panic(expected = "ChooseAndNote(Piles)")]
    fn choose_and_note_piles_is_a_loud_seam() {
        let (mut state, a) = bear_on_field();
        let frame = frame_src(a);
        state.run_effect(
            OneShotEffect::act_by_you(PlayerAction::ChooseAndNote(
                deckmaste_core::Ident::from("p"),
                deckmaste_core::NotedKind::Piles,
            )),
            &frame,
        );
    }

    /// P0.W3 seam filled for the new decision kind: the mechanical strategy
    /// notes the minimum (0, the X=0 default, via the reused `Decision::XValue`
    /// answer), and the seat resolver names the deciding player.
    #[test]
    fn strategy_and_seat_cover_the_note_number_choice() {
        use crate::decide::Decision;
        use crate::decide::PendingDecision;

        let (state, _a) = bear_on_field();
        let pending = PendingDecision::ChooseNoteNumber {
            player: PlayerId(1),
            key: deckmaste_core::Ident::from("n"),
        };
        assert_eq!(
            crate::sim::mechanical(&state, &pending),
            Decision::XValue(0)
        );
        assert_eq!(crate::sim::pending_player(&pending), PlayerId(1));
    }

    /// The exchange-control card, through `Simultaneously`
    /// ([CR#701.12a..701.12b]): the `ExchangeControl` macro's two halves
    /// read ONE pre-application snapshot, land as one `ControlChanged`
    /// batch, and the two creatures swap controllers — each
    /// summoning-sick for its new controller ([CR#302.6]).
    #[test]
    fn exchange_control_swaps_controllers_through_one_simultaneous_batch() {
        let (mut state, mine, other) = two_permanents_on_field();
        // Re-home `other` to player 1 so the exchange crosses seats.
        state.objects.obj_mut(other).controller = PlayerId(1);

        let effect: OneShotEffect = builtin()
            .macros
            .read_str(r"ExchangeControl(Target(0), Target(1))")
            .unwrap();
        let frame = frame_src_targets(mine, vec![mine, other]);
        state.run_effect(effect, &frame);

        // ONE batch of two ControlChanged facts.
        let front = state.agenda.front().cloned();
        let Some(WorkItem::Emit(Occurrence::Batch(events))) = front else {
            panic!("expected one ControlChanged batch, got {front:?}");
        };
        assert_eq!(events.len(), 2, "both halves in one occurrence");
        assert!(
            events
                .iter()
                .all(|e| matches!(e, GameEvent::ControlChanged { .. })),
            "the exchange is a batch of control transitions, got {events:?}"
        );

        run_injected(&mut state);
        assert_eq!(
            state.objects.obj(mine).controller,
            PlayerId(1),
            "player 1 gained control of player 0's creature"
        );
        assert_eq!(
            state.objects.obj(other).controller,
            PlayerId(0),
            "player 0 gained control of player 1's creature — both halves read \
             the PRE-exchange controllers (one snapshot)"
        );
        assert!(
            state.objects.obj(mine).summoning_sick && state.objects.obj(other).summoning_sick,
            "newly controlled permanents are summoning-sick ([CR#302.6])"
        );
    }

    /// [CR#701.12b]: exchanging control of two permanents the SAME player
    /// controls does nothing — each half is a no-transition no-op, and the
    /// all-or-nothing rule ([CR#701.12a]) voids the empty set.
    #[test]
    fn same_controller_exchange_does_nothing() {
        let (mut state, mine, other) = two_permanents_on_field();
        let effect: OneShotEffect = builtin()
            .macros
            .read_str(r"ExchangeControl(Target(0), Target(1))")
            .unwrap();
        let frame = frame_src_targets(mine, vec![mine, other]);
        state.run_effect(effect, &frame);
        assert!(
            !state.agenda.iter().any(|w| matches!(w, WorkItem::Emit(_))),
            "a same-controller exchange emits nothing ([CR#701.12b])"
        );
    }

    /// The `Fight` grammar macro's expansion ([CR#701.14a]): `Composite Fight`
    /// wrapping `If (both fighters are creatures on the battlefield —
    /// [CR#701.14b]) (Simultaneously [each deals its power to the OTHER, source
    /// = itself])`. Slots `x`/`y` are the two fighters. Mirrors
    /// `plugins/builtin/macros/effect/Fight.ron` (the guard's `Permanent` is
    /// spelled here as `InZone(Battlefield)`, an equivalent for the test).
    fn fight_effect(x: &Reference, y: &Reference) -> OneShotEffect {
        use deckmaste_core::CharacteristicPredicate;
        use deckmaste_core::Condition;
        use deckmaste_core::Predicate;
        use deckmaste_core::Stat;
        use deckmaste_core::StatePredicate;
        let is_creature = |r: &Reference| {
            Condition::Matches(
                r.clone(),
                Predicate::And(vec![
                    Predicate::Characteristic(CharacteristicPredicate::Type(Type::Creature.name())),
                    Predicate::State(StatePredicate::InZone(Zone::Battlefield)),
                ]),
            )
        };
        let half = |tgt: &Reference, src: &Reference| {
            OneShotEffect::Act(Action::DealDamage(
                src.clone(),
                Count::StatOf(src.clone(), Stat::Power),
                tgt.clone(),
            ))
        };
        OneShotEffect::Act(Action::Composite {
            name: "Fight".into(),
            body: Box::new(OneShotEffect::If(deckmaste_core::If {
                condition: Condition::And(vec![is_creature(x), is_creature(y)]),
                then: Box::new(OneShotEffect::Simultaneously(vec![half(y, x), half(x, y)])),
                otherwise: None,
            })),
        })
    }

    /// [CR#701.14a]: a fight — each creature deals damage equal to its power to
    /// the other, as ONE simultaneous batch of noncombat ([CR#701.14d]) damage
    /// facts; SBAs run after the whole batch. The `Composite Fight` fires its
    /// keyword-action fact once the guarded body acts.
    #[test]
    fn fight_deals_each_others_power_as_one_noncombat_batch() {
        let (mut state, a, b) = two_permanents_on_field();
        let frame = frame_src_targets(a, vec![a, b]);
        state.run_effect(
            fight_effect(&Reference::Target(0), &Reference::Target(1)),
            &frame,
        );
        // Both packets land as ONE applied batch occurrence.
        let batch = drain_progress(&mut state, 30)
            .into_iter()
            .find_map(|p| match p {
                Progress::Applied(Occurrence::Batch(evs))
                    if evs
                        .iter()
                        .all(|e| matches!(e, GameEvent::DamageDealt { .. })) =>
                {
                    Some(evs)
                }
                _ => None,
            })
            .expect("one batch of fight damage");
        assert_eq!(batch.len(), 2, "both damage packets in one occurrence");
        assert!(
            batch
                .iter()
                .all(|e| matches!(e, GameEvent::DamageDealt { combat: false, .. })),
            "fight damage is noncombat damage ([CR#701.14d]), got {batch:?}"
        );
        assert_eq!(state.objects.obj(a).total_damage(), 2, "a took b's power");
        assert_eq!(state.objects.obj(b).total_damage(), 2, "b took a's power");
        assert!(
            logged(&state, |e| matches!(
                e,
                GameEvent::KeywordActionPerformed { .. }
            )),
            "the fight fired its 'fights' keyword-action fact"
        );
    }

    /// [CR#701.14b]: both-or-neither — a fighter that is no longer a creature on
    /// the battlefield when the fight would occur means NEITHER deals damage,
    /// and no "fights" fact fires (the `If` guard is false).
    #[test]
    fn fight_with_a_gone_fighter_deals_no_damage_at_all() {
        let (mut state, a, b) = two_permanents_on_field();
        // b leaves before the fight resolves.
        state.zones.battlefield.retain(|&o| o != b);
        state.objects.remove(b);
        let frame = frame_src_targets(a, vec![a, b]);
        state.run_effect(
            fight_effect(&Reference::Target(0), &Reference::Target(1)),
            &frame,
        );
        run_injected(&mut state);
        assert!(
            !logged(&state, |e| matches!(e, GameEvent::DamageDealt { .. })),
            "neither creature deals damage ([CR#701.14b])"
        );
        assert!(
            !logged(&state, |e| matches!(
                e,
                GameEvent::KeywordActionPerformed { .. }
            )),
            "no fight occurred, so no 'fights' fact"
        );
        assert_eq!(state.objects.obj(a).total_damage(), 0);
    }

    /// [CR#701.14c]: a creature fighting itself deals damage to itself equal to
    /// TWICE its power — the macro's two `X -> X` packets coalesce to one
    /// instance in the simultaneous batch.
    #[test]
    fn self_fight_deals_twice_its_power_to_itself() {
        let (mut state, a, _b) = two_permanents_on_field();
        let frame = frame_src_targets(a, vec![a, a]);
        state.run_effect(
            fight_effect(&Reference::Target(0), &Reference::Target(1)),
            &frame,
        );
        run_injected(&mut state);
        assert_eq!(
            state.objects.obj(a).total_damage(),
            4,
            "a 2/2 fighting itself takes 2 x 2 = 4 ([CR#701.14c])"
        );
        assert!(
            logged(&state, |e| matches!(
                e,
                GameEvent::DamageDealt {
                    amount: 4,
                    combat: false,
                    ..
                }
            )),
            "one coalesced damage instance of twice its power"
        );
    }

    /// [CR#120.1,701.14a]: `DealDamage`'s explicit `source` is the dealer — the
    /// emitted `DamageDealt` carries it, NOT `frame.source`. The fight shape:
    /// `b` (the "second" slot) deals damage equal to its power to `a` (the
    /// "first" slot), with the frame source set to `a`.
    #[test]
    fn deal_damage_uses_explicit_source_not_frame_source() {
        let (mut state, a, b) = two_permanents_on_field();
        let frame = frame_src_targets(a, vec![a, b]);
        state.run_effect(
            OneShotEffect::Act(Action::DealDamage(
                Reference::Target(1),
                Count::StatOf(Reference::Target(1), deckmaste_core::Stat::Power),
                Reference::Target(0),
            )),
            &frame,
        );
        run_injected(&mut state);
        assert_eq!(
            state.objects.obj(a).total_damage(),
            2,
            "a took b's power (2) in damage"
        );
        assert!(
            logged(&state, |e| matches!(
                e,
                GameEvent::DamageDealt { source, target, amount, .. }
                    if *source == b && *target == a && *amount == 2
            )),
            "DamageDealt carries the explicit source b, not frame.source a"
        );
    }

    /// [CR#611.2]/[CR#611.2c]: `OneShotEffect::Continuously(Modify(Matching(...), ...),
    /// UntilEndOfTurn)` — the resolve arm pushes one `ContinuousEffect` with a
    /// `ScopeResolved::Floating` scope and the right duration/changes.
    #[test]
    fn continuously_matching_registers_floating_scope() {
        use deckmaste_core::Continuously;
        use deckmaste_core::Count;
        use deckmaste_core::Duration;
        use deckmaste_core::Modification;
        use deckmaste_core::NumericOp;
        use deckmaste_core::OneShotEffect;
        use deckmaste_core::Predicate;
        use deckmaste_core::Reference;
        use deckmaste_core::Selection;
        use deckmaste_core::StaticEffect;

        let (mut state, src) = bear_on_field();
        let frame = frame_src(src);

        assert!(state.continuous.is_empty(), "no effects before resolve");

        let filter = Predicate::creature();
        let effect = OneShotEffect::Continuously(Continuously {
            effect: Box::new(StaticEffect::Each(
                Selection::SelectAll(filter.clone()),
                Box::new(StaticEffect::Modify(
                    Reference::It,
                    Modification::Power(NumericOp::Up(Count::Literal(1))),
                )),
            )),
            duration: Duration::FixedUntil(deckmaste_core::TurnMarker::EndOfTurn),
        });
        state.run_effect(effect, &frame);

        assert_eq!(state.continuous.len(), 1, "one effect registered");
        let ce = &state.continuous[0];
        assert!(
            matches!(&ce.scope, crate::layer::ScopeResolved::Floating(f) if f == &filter),
            "scope is Floating(creature filter)"
        );
        assert_eq!(
            ce.duration,
            Duration::FixedUntil(deckmaste_core::TurnMarker::EndOfTurn)
        );
        assert_eq!(
            ce.changes,
            vec![Modification::Power(NumericOp::Up(Count::Literal(1)))]
        );
        assert!(!ce.is_cda);
    }

    /// [CR#611.2c]: `OneShotEffect::Continuously(Modify(Of(This), ...), ...)` locks
    /// the id at creation — `ScopeResolved::Locked(vec![src])`.
    #[test]
    fn continuously_of_this_registers_locked_scope() {
        use deckmaste_core::Continuously;
        use deckmaste_core::Count;
        use deckmaste_core::Duration;
        use deckmaste_core::Modification;
        use deckmaste_core::NumericOp;
        use deckmaste_core::OneShotEffect;
        use deckmaste_core::Reference;
        use deckmaste_core::StaticEffect;

        let (mut state, src) = bear_on_field();
        let frame = frame_src(src);

        let effect = OneShotEffect::Continuously(Continuously {
            effect: Box::new(StaticEffect::Modify(
                Reference::This,
                Modification::Toughness(NumericOp::Up(Count::Literal(2))),
            )),
            duration: Duration::FixedUntil(deckmaste_core::TurnMarker::EndOfTurn),
        });
        state.run_effect(effect, &frame);

        assert_eq!(state.continuous.len(), 1, "one effect registered");
        let ce = &state.continuous[0];
        assert!(
            matches!(&ce.scope, crate::layer::ScopeResolved::Locked(ids) if ids == &vec![src]),
            "scope is Locked([src])"
        );
        assert_eq!(
            ce.changes,
            vec![Modification::Toughness(NumericOp::Up(Count::Literal(2)))]
        );
    }

    /// [CR#611.2c]: a granted `Deontic` ("target creature can't block this
    /// turn") mints a static ROW, not a layer change — its subject `Target(0)`
    /// resolves to the locked object at mint and is rewritten to `Ref(It)`.
    #[test]
    fn continuously_deontic_mints_locked_row_rewriting_subject_to_it() {
        use deckmaste_core::Continuously;
        use deckmaste_core::Deontic;
        use deckmaste_core::DeonticAction;
        use deckmaste_core::Duration;
        use deckmaste_core::OneShotEffect;
        use deckmaste_core::Predicate;
        use deckmaste_core::Reference;
        use deckmaste_core::StaticEffect;
        use deckmaste_core::TurnMarker;

        let (mut state, src) = bear_on_field();
        // The bear is the lone announced target — the restriction's subject.
        let frame = frame_src_targets(src, vec![src]);
        let effect = OneShotEffect::Continuously(Continuously {
            effect: Box::new(StaticEffect::Deontic(Deontic::Cant(DeonticAction::Block {
                by: Predicate::Ref(Reference::Target(0)),
                on: Predicate::Any,
                count: None,
            }))),
            duration: Duration::FixedUntil(TurnMarker::EndOfTurn),
        });
        state.run_effect(effect, &frame);

        assert_eq!(state.continuous.len(), 1);
        let ce = &state.continuous[0];
        assert!(
            matches!(&ce.scope, crate::layer::ScopeResolved::Locked(ids) if ids == &vec![src]),
            "subject Target(0) locked to the bear at mint"
        );
        assert!(
            ce.changes.is_empty(),
            "a Deontic grant carries no layer changes"
        );
        assert!(
            matches!(
                &ce.rows[..],
                [StaticEffect::Deontic(Deontic::Cant(DeonticAction::Block { by, .. }))]
                    if *by == Predicate::Ref(Reference::It)
            ),
            "subject rewritten to Ref(It), got {:?}",
            ce.rows
        );
    }

    /// A granted `CostModifier` is self-filtered (empty lock) and lands as a
    /// row.
    #[test]
    fn continuously_cost_modifier_mints_self_filtered_row() {
        use deckmaste_core::Continuously;
        use deckmaste_core::CostChange;
        use deckmaste_core::Duration;
        use deckmaste_core::OneShotEffect;
        use deckmaste_core::Predicate;
        use deckmaste_core::StaticEffect;
        use deckmaste_core::TurnMarker;

        let (mut state, src) = bear_on_field();
        let frame = frame_src(src);
        let effect = OneShotEffect::Continuously(Continuously {
            effect: Box::new(StaticEffect::CostModifier {
                of: Predicate::creature(),
                change: CostChange::Increase(vec![]),
            }),
            duration: Duration::FixedUntil(TurnMarker::EndOfTurn),
        });
        state.run_effect(effect, &frame);
        let ce = &state.continuous[0];
        assert!(
            matches!(&ce.scope, crate::layer::ScopeResolved::Locked(ids) if ids.is_empty()),
            "self-filtered → empty lock"
        );
        assert!(ce.changes.is_empty());
        assert!(matches!(&ce.rows[..], [StaticEffect::CostModifier { .. }]));
    }

    /// A granted `CantHappen` is self-filtered and lands as a row.
    #[test]
    fn continuously_cant_happen_mints_self_filtered_row() {
        use deckmaste_core::Continuously;
        use deckmaste_core::Duration;
        use deckmaste_core::EventFilter;
        use deckmaste_core::OneShotEffect;
        use deckmaste_core::Predicate;
        use deckmaste_core::StaticEffect;
        use deckmaste_core::TurnMarker;

        let (mut state, src) = bear_on_field();
        let frame = frame_src(src);
        let filter = EventFilter::Damage {
            source: Predicate::Any,
            to: Predicate::Any,
            combat: None,
            amount: None,
        };
        let effect = OneShotEffect::Continuously(Continuously {
            effect: Box::new(StaticEffect::CantHappen(filter)),
            duration: Duration::FixedUntil(TurnMarker::EndOfTurn),
        });
        state.run_effect(effect, &frame);
        let ce = &state.continuous[0];
        assert!(ce.changes.is_empty());
        assert!(matches!(&ce.rows[..], [StaticEffect::CantHappen(_)]));
    }

    /// A granted `Prevention` is LOUD — engine-prevention owns the machinery.
    #[test]
    #[should_panic(expected = "engine-prevention")]
    fn continuously_prevention_is_loud() {
        use deckmaste_core::Continuously;
        use deckmaste_core::Duration;
        use deckmaste_core::OneShotEffect;
        use deckmaste_core::Predicate;
        use deckmaste_core::Prevention;
        use deckmaste_core::StaticEffect;
        use deckmaste_core::TurnMarker;

        let (mut state, src) = bear_on_field();
        let frame = frame_src(src);
        let effect = OneShotEffect::Continuously(Continuously {
            effect: Box::new(StaticEffect::Prevention(Box::new(
                Prevention::PreventNextInstance {
                    from: Predicate::Any,
                    to: Predicate::Any,
                },
            ))),
            duration: Duration::FixedUntil(TurnMarker::EndOfTurn),
        });
        state.run_effect(effect, &frame);
    }

    /// [CR#611.2b]: a `ForAsLongAs` whose condition is already false at creation
    /// never starts — no instance is pushed.
    #[test]
    fn for_as_long_as_false_at_mint_never_starts() {
        use deckmaste_core::Condition;
        use deckmaste_core::Continuously;
        use deckmaste_core::Count;
        use deckmaste_core::Duration;
        use deckmaste_core::Modification;
        use deckmaste_core::NumericOp;
        use deckmaste_core::OneShotEffect;
        use deckmaste_core::Predicate;
        use deckmaste_core::Reference;
        use deckmaste_core::StaticEffect;

        let (mut state, src) = bear_on_field();
        let frame = frame_src(src);
        // "for as long as an object matching not-anything exists" — always false.
        let cond = Condition::Exists(Predicate::Not(Box::new(Predicate::Any)));
        let effect = OneShotEffect::Continuously(Continuously {
            effect: Box::new(StaticEffect::Modify(
                Reference::This,
                Modification::Power(NumericOp::Up(Count::Literal(1))),
            )),
            duration: Duration::ForAsLongAs(cond),
        });
        state.run_effect(effect, &frame);
        assert!(
            state.continuous.is_empty(),
            "a false-at-mint ForAsLongAs pushes no instance"
        );
    }

    /// [CR#611.2b]: `sweep_condition_durations` removes an instance whose
    /// condition has lapsed — the once-stopped-never-resumes latch is the
    /// removal itself.
    #[test]
    fn sweep_condition_durations_removes_lapsed_for_as_long_as() {
        use deckmaste_core::Condition;
        use deckmaste_core::Count;
        use deckmaste_core::Duration;
        use deckmaste_core::Modification;
        use deckmaste_core::NumericOp;
        use deckmaste_core::Predicate;

        let (mut state, src) = bear_on_field();
        let cond = Condition::Exists(Predicate::Not(Box::new(Predicate::Any)));
        state.continuous.push(crate::layer::ContinuousEffect {
            timestamp: crate::object::Timestamp(1),
            controller: PlayerId(0),
            scope: crate::layer::ScopeResolved::Locked(vec![src]),
            changes: vec![Modification::Power(NumericOp::Up(Count::Literal(1)))],
            rows: vec![],
            duration: Duration::ForAsLongAs(cond),
            origin: Some(Box::new(frame_src(src))),
            is_cda: false,
        });
        state.sweep_condition_durations();
        assert!(
            state.continuous.is_empty(),
            "a lapsed ForAsLongAs is removed"
        );
    }

    /// [CR#611.2a],[CR#701.19c]: a `Sequentially` folds an `Until(ForThisEvent,
    /// [Cant(Regenerate)])` child onto the IMMEDIATELY-preceding sibling — an
    /// `InstallRiders` armed with the destroy subject is scheduled just before
    /// that sibling's `RunEffect`, and the rider mints no continuous instance.
    #[test]
    fn sequentially_folds_for_this_event_rider_before_preceding_sibling() {
        use deckmaste_core::Action;
        use deckmaste_core::Count;
        use deckmaste_core::Deontic;
        use deckmaste_core::DeonticAction;
        use deckmaste_core::Duration;
        use deckmaste_core::OneShotEffect;
        use deckmaste_core::PlayerAction;
        use deckmaste_core::Predicate;
        use deckmaste_core::Reference;
        use deckmaste_core::StaticEffect;

        let (mut state, src) = bear_on_field();
        let frame = frame_src(src);
        let seq = OneShotEffect::Sequentially(vec![
            OneShotEffect::Act(Action::by_you(PlayerAction::GainLife(Count::Literal(1)))),
            OneShotEffect::Until(
                Duration::ForThisEvent,
                vec![StaticEffect::Deontic(Deontic::Cant(
                    DeonticAction::Regenerate {
                        by: Predicate::Any,
                        on: Predicate::Ref(Reference::This),
                    },
                ))],
            ),
        ]);
        state.run_effect(seq, &frame);

        let front: Vec<&WorkItem> = state.agenda.iter().take(2).collect();
        assert!(
            matches!(front[0], WorkItem::InstallRiders { no_regen } if no_regen == &vec![src]),
            "InstallRiders armed with the destroy subject, got {:?}",
            front[0]
        );
        assert!(
            matches!(front[1], WorkItem::RunEffect { .. }),
            "the preceding sibling runs next"
        );
        assert!(
            state.continuous.is_empty(),
            "the rider child mints no continuous instance"
        );
    }

    /// [CR#611.2a]: a `ForThisEvent` rider with NO preceding sibling is an
    /// authoring mistake — it fizzles (dropped), never mints or panics.
    #[test]
    fn for_this_event_rider_without_preceding_sibling_fizzles() {
        use deckmaste_core::Deontic;
        use deckmaste_core::DeonticAction;
        use deckmaste_core::Duration;
        use deckmaste_core::OneShotEffect;
        use deckmaste_core::Predicate;
        use deckmaste_core::Reference;
        use deckmaste_core::StaticEffect;

        let (mut state, src) = bear_on_field();
        let frame = frame_src(src);
        let seq = OneShotEffect::Sequentially(vec![OneShotEffect::Until(
            Duration::ForThisEvent,
            vec![StaticEffect::Deontic(Deontic::Cant(
                DeonticAction::Regenerate {
                    by: Predicate::Any,
                    on: Predicate::Ref(Reference::This),
                },
            ))],
        )]);
        state.run_effect(seq, &frame);
        assert!(
            !state
                .agenda
                .iter()
                .any(|w| matches!(w, WorkItem::InstallRiders { .. })),
            "a rider with no host instruction is dropped"
        );
    }

    /// [CR#611.2,614.3]: the shared sweepable-duration guard makes a
    /// non-sweepable (`ForThisEvent`) shield LOUD at `create_shield` — a rider
    /// duration never mints a stored instance.
    #[test]
    #[should_panic(expected = "non-sweepable duration")]
    fn create_shield_rejects_non_sweepable_duration() {
        use deckmaste_core::BeginningStep;
        use deckmaste_core::Duration;
        use deckmaste_core::PhaseStep;
        use deckmaste_core::Reference;
        use deckmaste_core::Replacement;

        let (mut state, src) = bear_on_field();
        let frame = frame_src(src);
        state.create_shield(
            Replacement::Skip {
                what: PhaseStep::Beginning(BeginningStep::Untap),
            },
            &Reference::This,
            Duration::ForThisEvent,
            false,
            &frame,
        );
    }

    /// [CR#511.2]: an "until end of combat" instance is swept by
    /// `expire_end_of_combat`, and by the cleanup catch-all when combat is
    /// skipped.
    #[test]
    fn until_end_of_combat_expires_at_end_of_combat_and_via_cleanup_catch_all() {
        use deckmaste_core::Count;
        use deckmaste_core::Duration;
        use deckmaste_core::Modification;
        use deckmaste_core::NumericOp;
        use deckmaste_core::TurnMarker;

        let (mut state, src) = bear_on_field();
        let mint = |state: &mut GameState| {
            state.continuous.push(crate::layer::ContinuousEffect {
                timestamp: crate::object::Timestamp(1),
                controller: PlayerId(0),
                scope: crate::layer::ScopeResolved::Locked(vec![src]),
                changes: vec![Modification::Power(NumericOp::Up(Count::Literal(1)))],
                rows: vec![],
                duration: Duration::FixedUntil(TurnMarker::EndOfCombat),
                origin: None,
                is_cda: false,
            });
        };
        mint(&mut state);
        state.expire_end_of_combat();
        assert!(state.continuous.is_empty(), "swept at end of combat");
        // Combat-skipped turn: cleanup's catch-all still removes it.
        mint(&mut state);
        state.expire_end_of_turn();
        assert!(
            state.continuous.is_empty(),
            "cleanup catch-all sweeps a leaked EndOfCombat effect"
        );
    }

    /// [CR#611.2a]: an "until your next turn" instance survives an opponent's
    /// turn and expires as its controller's next turn begins.
    #[test]
    fn until_your_next_turn_survives_opponent_expires_at_controller() {
        use deckmaste_core::Count;
        use deckmaste_core::Duration;
        use deckmaste_core::Modification;
        use deckmaste_core::NumericOp;
        use deckmaste_core::TurnMarker;

        let (mut state, src) = bear_on_field();
        state.continuous.push(crate::layer::ContinuousEffect {
            timestamp: crate::object::Timestamp(1),
            controller: PlayerId(0),
            scope: crate::layer::ScopeResolved::Locked(vec![src]),
            changes: vec![Modification::Power(NumericOp::Up(Count::Literal(1)))],
            rows: vec![],
            duration: Duration::FixedUntil(TurnMarker::YourNextTurn),
            origin: None,
            is_cda: false,
        });
        state.expire_your_next_turn(PlayerId(1));
        assert_eq!(state.continuous.len(), 1, "survives the opponent's turn");
        state.expire_your_next_turn(PlayerId(0));
        assert!(
            state.continuous.is_empty(),
            "expires as the controller's turn begins"
        );
    }

    /// [CR#610.3]: an `UntilEvent` instance is removed once its awaited event is
    /// applied, and survives a non-matching one.
    #[test]
    fn until_event_removed_on_match_survives_nonmatch() {
        use deckmaste_core::Count;
        use deckmaste_core::Duration;
        use deckmaste_core::EventFilter;
        use deckmaste_core::Modification;
        use deckmaste_core::NumericOp;
        use deckmaste_core::Predicate;

        let (mut state, src) = bear_on_field();
        let filter = EventFilter::Damage {
            source: Predicate::Any,
            to: Predicate::Any,
            combat: None,
            amount: None,
        };
        state.continuous.push(crate::layer::ContinuousEffect {
            timestamp: crate::object::Timestamp(1),
            controller: PlayerId(0),
            scope: crate::layer::ScopeResolved::Locked(vec![src]),
            changes: vec![Modification::Power(NumericOp::Up(Count::Literal(1)))],
            rows: vec![],
            duration: Duration::UntilEvent(filter),
            origin: Some(Box::new(frame_src(src))),
            is_cda: false,
        });
        // A non-matching fact (a life gain) leaves it in place.
        state.sweep_event_durations(&crate::event::Occurrence::single(GameEvent::LifeGained {
            player: PlayerId(0),
            amount: 1,
        }));
        assert_eq!(
            state.continuous.len(),
            1,
            "survives a non-matching occurrence"
        );
        // The awaited damage fact ends it.
        state.sweep_event_durations(&crate::event::Occurrence::single(GameEvent::DamageDealt {
            source: src,
            target: src,
            amount: 1,
            combat: false,
        }));
        assert!(
            state.continuous.is_empty(),
            "removed once the awaited event happens"
        );
    }

    /// `By(You, GainLife(3))` → one `LifeGained`; `By(You, Untap(This))` → one
    /// `Untapped` — the mirrors of `LoseLife`/`Tap` above. The bear is tapped
    /// first: untapping is transition-only ([CR#701.26b]).
    #[test]
    fn action_items_for_gainlife_untap() {
        let (mut state, src) = bear_on_field();
        state.objects.obj_mut(src).tapped = true;
        let frame = frame_src(src);

        let items = state.action_items(
            &Action::by_you(PlayerAction::GainLife(Count::Literal(3))),
            &frame,
        );
        assert_eq!(
            items,
            vec![WorkItem::Emit(Occurrence::Single(GameEvent::LifeGained {
                player: PlayerId(0),
                amount: 3,
            }))]
        );

        let items = state.action_items(
            &Action::by_you(PlayerAction::Untap(Reference::This)),
            &frame,
        );
        assert_eq!(
            items,
            vec![WorkItem::Emit(Occurrence::Single(GameEvent::Untapped(src)))]
        );
    }

    /// [CR#122.1]: `PutCounters(This, P1P1Counter, 2)` emits one `CounterPlaced`
    /// per selected object, carrying the effect-instruction cause
    /// (events.md §3) — its agent is the resolving source's controller.
    /// Counter kinds are bare `CounterRef` idents, not symbolic strings.
    #[test]
    fn put_counters_emits_counter_placed() {
        use deckmaste_core::Agency;

        use crate::event::Cause;

        let (state, bear) = bear_on_field();
        let frame = frame_src(bear);
        let items = state.action_items(
            &Action::by_you(PlayerAction::PutCounters(
                Reference::This,
                "P1P1Counter".into(),
                Count::Literal(2),
            )),
            &frame,
        );
        assert_eq!(
            items,
            vec![WorkItem::Emit(Occurrence::Single(
                GameEvent::CounterPlaced {
                    object: bear,
                    kind: "P1P1Counter".into(),
                    amount: 2,
                    before: 0,
                    after: 0,
                    cause: Some(Cause::put_counters(
                        Agency::EffectInstruction,
                        Some((bear, PlayerId(0))),
                    )),
                }
            ))]
        );
    }

    /// [CR#122.1]: putting zero counters is a no-op — no event (so no
    /// "counter is put on" trigger fires for nothing).
    #[test]
    fn put_zero_counters_emits_nothing() {
        let (state, bear) = bear_on_field();
        let frame = frame_src(bear);
        let items = state.action_items(
            &Action::by_you(PlayerAction::PutCounters(
                Reference::This,
                "P1P1Counter".into(),
                Count::Literal(0),
            )),
            &frame,
        );
        assert_eq!(items, vec![]);
    }

    /// Applying `CounterPlaced` adds to the object's counter map, and a second
    /// placement of the same kind sums ([CR#122.1] — counters are
    /// interchangeable).
    #[test]
    fn counter_placed_apply_is_additive() {
        let (mut state, bear) = bear_on_field();
        state
            .objects
            .obj_mut(bear)
            .counters
            .insert("P1P1Counter".into(), 1);
        let frame = frame_src(bear);
        state.run_effect(
            OneShotEffect::act_by_you(PlayerAction::PutCounters(
                Reference::This,
                "P1P1Counter".into(),
                Count::Literal(2),
            )),
            &frame,
        );
        let _ = state.step(); // applies CounterPlaced
        assert_eq!(
            state
                .objects
                .obj(bear)
                .counters
                .get(&deckmaste_core::Ident::from("P1P1Counter"))
                .copied(),
            Some(3)
        );
    }

    /// [CR#107.14,122.1]: "you get {E}{E}" is `PutCounters(You, Energy, N)` —
    /// a player-borne counter placement ([CR#122.1] — a counter is a marker on
    /// an object OR player). `Reference::You` resolves to the controller's
    /// proxy object, so the same apply path lands the energy on the PLAYER
    /// (energy sits on the player, [CR#107.14]), and a second gain sums
    /// ([CR#122.1] — counters are interchangeable).
    #[test]
    fn get_energy_adds_counters_to_the_player_proxy() {
        let (mut state, bear) = bear_on_field();
        let proxy = state.player(PlayerId(0)).object;
        let frame = frame_src(bear); // controller is player 0, so `You` = P0's proxy
        state.run_effect(
            OneShotEffect::act_by_you(PlayerAction::PutCounters(
                Reference::You,
                "Energy".into(),
                Count::Literal(2),
            )),
            &frame,
        );
        let _ = state.step(); // applies CounterPlaced onto the player proxy
        assert_eq!(
            state
                .objects
                .obj(proxy)
                .counters
                .get(&deckmaste_core::Ident::from("Energy"))
                .copied(),
            Some(2),
            "you get {{E}}{{E}} places two energy on the player's proxy"
        );

        // A second "get {E}" sums with the first.
        state.run_effect(
            OneShotEffect::act_by_you(PlayerAction::PutCounters(
                Reference::You,
                "Energy".into(),
                Count::Literal(1),
            )),
            &frame,
        );
        let _ = state.step();
        assert_eq!(
            state
                .objects
                .obj(proxy)
                .counters
                .get(&deckmaste_core::Ident::from("Energy"))
                .copied(),
            Some(3),
            "energy gains accumulate on the player [CR#122.1]"
        );

        // And `Count::CounterCount(You, Energy)` reads it back — "if you have
        // N energy" ([CR#107.14]).
        assert_eq!(
            state.eval_count(
                &Count::CounterCount(Box::new(Reference::You), "Energy".into()),
                &frame
            ),
            3,
            "CounterCount(You, Energy) reads the player's energy total"
        );
    }

    /// Removing more counters than present clamps to zero and DROPS the key,
    /// so `HasCounter` and the layer-7c P/T read both see absence ([CR#122.1]).
    #[test]
    fn counter_removed_clamps_and_drops_key() {
        let (mut state, bear) = bear_on_field();
        state
            .objects
            .obj_mut(bear)
            .counters
            .insert("P1P1Counter".into(), 1);
        let frame = frame_src(bear);
        state.run_effect(
            OneShotEffect::act_by_you(PlayerAction::RemoveCounters(
                Reference::This,
                "P1P1Counter".into(),
                Count::Literal(2),
            )),
            &frame,
        );
        let _ = state.step(); // applies CounterRemoved
        assert!(
            !state
                .objects
                .obj(bear)
                .counters
                .contains_key(&deckmaste_core::Ident::from("P1P1Counter")),
            "a counter kind dropped to zero leaves no key behind"
        );
    }

    /// [CR#122.1]: `Count::CounterCount(ref, kind)` reads how many `kind`
    /// counters sit on the resolved object/player proxy; an absent kind is 0.
    /// Counter kinds are rusty idents (`P1P1Counter`), not symbolic strings.
    #[test]
    fn counter_count_reads_the_objects_counter_map() {
        let (mut state, bear) = bear_on_field();
        state
            .objects
            .obj_mut(bear)
            .counters
            .insert("P1P1Counter".into(), 3);
        let frame = frame_src(bear);
        assert_eq!(
            state.eval_count(
                &Count::CounterCount(Box::new(Reference::This), "P1P1Counter".into()),
                &frame
            ),
            3
        );
        assert_eq!(
            state.eval_count(
                &Count::CounterCount(Box::new(Reference::This), "M1M1Counter".into()),
                &frame
            ),
            0,
            "an absent counter kind reads as zero"
        );
    }

    /// [CR#603.10a,702.43a]: when the object a `CounterCount(This, _)` names is
    /// GONE (a dies trigger — Modular's "for each +1/+1 counter on this
    /// permanent" resolves after the creature left the battlefield), the count
    /// comes from the trigger's last-known snapshot, not the stale id. Without
    /// the LKI bridge `eval_count` would panic dereferencing the dead object.
    #[test]
    fn counter_count_reads_lki_when_the_object_is_gone() {
        let (mut state, bear) = bear_on_field();
        state
            .objects
            .obj_mut(bear)
            .counters
            .insert("P1P1Counter".into(), 2);
        // Snapshot the creature, then remove it — `bear` is now a stale id, the
        // exact state a dies trigger's `This` resolves to ([CR#603.10a]).
        let snapshot = crate::lki::LkiSnapshot::capture(&state, bear);
        state.objects.remove(bear);
        assert!(state.objects.get(bear).is_none(), "the object is gone");

        let frame = Frame {
            this: Some(snapshot),
            ..Frame::bare(bear, PlayerId(0))
        };

        assert_eq!(
            state.eval_count(
                &Count::CounterCount(Box::new(Reference::This), "P1P1Counter".into()),
                &frame
            ),
            2,
            "the dying creature's last-known +1/+1 counter count"
        );
        assert_eq!(
            state.eval_count(
                &Count::CounterCount(Box::new(Reference::This), "M1M1Counter".into()),
                &frame
            ),
            0,
            "an absent kind on the snapshot reads as zero"
        );
    }

    /// [CR#209.1,306.5a]: `StatOf(_, Loyalty)` reads the PRINTED loyalty
    /// characteristic off the card face, NOT the live loyalty-counter count.
    /// A planeswalker printed at loyalty 4 carrying a single loyalty counter
    /// reads 4 (printed), never 1 (counters). Current on-battlefield loyalty is
    /// the separate `CounterCount(This, LoyaltyCounter)` read exercised below.
    #[test]
    fn stat_of_loyalty_reads_printed_loyalty() {
        use deckmaste_core::Stat;

        let (mut state, _bear) = bear_on_field();
        let card = Card::Normal(CardFace {
            name: "Test Walker".into(),
            types: vec![Type::Planeswalker.def()],
            loyalty: Some(deckmaste_core::StatValue::Number(4)),
            ..CardFace::default()
        });
        let cid = state.cards.push(Arc::new(card), PlayerId(0));
        let walker = state.objects.mint(
            ObjectSource::Card(cid),
            PlayerId(0),
            Some(Zone::Battlefield),
        );
        state.zones.battlefield.push(walker);
        // A single loyalty counter — the printed read must IGNORE it.
        state
            .objects
            .obj_mut(walker)
            .counters
            .insert("LoyaltyCounter".into(), 1);
        let frame = frame_src(walker);
        assert_eq!(
            state.eval_count(&Count::StatOf(Reference::This, Stat::Loyalty), &frame),
            4,
            "printed loyalty (4), not the loyalty-counter count (1)"
        );
    }

    /// [CR#306.5c,122.1e]: CURRENT on-battlefield loyalty IS the loyalty-counter
    /// count, spelled `CounterCount(This, LoyaltyCounter)` — the companion to
    /// the printed `StatOf(_, Loyalty)` read above, now that
    /// `Stat::Loyalty` no longer means the counter count.
    #[test]
    fn counter_count_reads_current_loyalty() {
        let (mut state, bear) = bear_on_field();
        state
            .objects
            .obj_mut(bear)
            .counters
            .insert("LoyaltyCounter".into(), 4);
        let frame = frame_src(bear);
        assert_eq!(
            state.eval_count(
                &Count::CounterCount(Box::new(Reference::This), "LoyaltyCounter".into()),
                &frame
            ),
            4,
            "current loyalty = the live loyalty-counter count"
        );
    }

    /// [CR#701.21a]: `Sacrifice(This)` emits the verb fact, which evolves into
    /// the Battlefield→Graveyard move — old id gone, fresh object in the
    /// owner's graveyard.
    #[test]
    fn sacrifice_this_remints_to_owners_graveyard() {
        let (mut state, bear) = bear_on_field();
        let frame = frame_src(bear);
        state.run_effect(
            OneShotEffect::act_by_you(PlayerAction::Sacrifice(Reference::This)),
            &frame,
        );
        // Sacrificed → ZoneWillChange → ZoneChanged.
        for _ in 0..3 {
            let _ = state.step();
        }
        assert!(state.objects.get(bear).is_none(), "old battlefield id gone");
        assert!(!state.zones.battlefield.contains(&bear));
        assert_eq!(state.zones.graveyards[0].len(), 1);
        assert_ne!(state.zones.graveyards[0][0], bear, "reminted");
    }

    /// A sacrifice rides the same death pipeline as a destroy: the sacrificed
    /// creature's own dies-trigger fires ([CR#603.6c] — the leaving object
    /// watches its own departure).
    #[test]
    fn sacrifice_fires_the_dying_objects_dies_trigger() {
        let card = Arc::new(canon().card("Footlight Fiend").unwrap());
        let forest = Arc::new(builtin().card("Forest").unwrap());
        let mut state = GameState::new(GameConfig {
            players: vec![
                PlayerConfig {
                    deck: deck(&forest, 10),
                },
                PlayerConfig {
                    deck: deck(&forest, 10),
                },
            ],
            seed: 1,
            starting_life: 20,
            starting_player: StartingPlayer::Fixed(PlayerId(0)),
            sba_rules: vec![],
            conferral_rules: vec![],
            damage_result_rules: vec![],
            counter_decls: std::collections::HashMap::new(),
            subtypes: std::collections::HashMap::new(),
            types: std::collections::HashMap::new(),
        });
        let card_id = state.cards.push(card, PlayerId(0));
        let gob = state.objects.mint(
            ObjectSource::Card(card_id),
            PlayerId(0),
            Some(Zone::Battlefield),
        );
        state.zones.battlefield.push(gob);

        let frame = frame_src(gob);
        state.run_effect(
            OneShotEffect::act_by_you(PlayerAction::Sacrifice(Reference::This)),
            &frame,
        );
        for _ in 0..10 {
            if !state.pending_triggers.is_empty() {
                break;
            }
            let _ = state.step();
        }
        assert_eq!(
            state.pending_triggers.len(),
            1,
            "the self-dies trigger must be noted"
        );
        assert!(state.objects.get(gob).is_none(), "the sacrifice happened");
    }

    /// [CR#701.13a,406.2]: exile moves an object to the shared exile zone —
    /// from the battlefield, and (via the graveyard source arm) from a
    /// graveyard.
    #[test]
    fn exile_moves_objects_from_battlefield_and_graveyard() {
        let (mut state, bear) = bear_on_field();
        let frame = frame_src(bear);
        state.run_effect(
            OneShotEffect::act_by_you(PlayerAction::Move(
                Reference::This,
                deckmaste_core::Destination::Zone(Zone::Exile),
                vec![],
            )),
            &frame,
        );
        // ZoneWillChange → ZoneChanged.
        for _ in 0..2 {
            let _ = state.step();
        }
        assert!(state.objects.get(bear).is_none(), "old id gone");
        assert_eq!(state.zones.exile.len(), 1);
        let exiled = state.zones.exile[0];
        assert_eq!(state.objects.obj(exiled).zone, Some(Zone::Exile));

        // From the graveyard: force a hand card into the graveyard, exile it.
        let card = *state.zones.hands[0].first().expect("a card in hand");
        state.zones.hands[0].retain(|&o| o != card);
        state.objects.obj_mut(card).zone = Some(Zone::Graveyard);
        state.zones.graveyards[0].push(card);
        let frame = frame_src(card);
        state.run_effect(
            OneShotEffect::act_by_you(PlayerAction::Move(
                Reference::This,
                deckmaste_core::Destination::Zone(Zone::Exile),
                vec![],
            )),
            &frame,
        );
        for _ in 0..2 {
            let _ = state.step();
        }
        assert!(state.objects.get(card).is_none(), "old graveyard id gone");
        assert!(state.zones.graveyards[0].is_empty());
        assert_eq!(state.zones.exile.len(), 2);
    }

    /// [CR#400.7]: `Move(This, Hand)` moves the source to its owner's hand,
    /// reminting it — the old id is gone and a fresh object sits in hand
    /// (the bounce family, subsuming the retired `ReturnToHand` verb). The
    /// graveyard arm proves the move reads each object's current zone (like
    /// `Exile`), not a hard-coded battlefield source.
    #[test]
    fn return_to_hand_from_battlefield_and_graveyard() {
        let (mut state, bear) = bear_on_field();
        let hand_before = state.zones.hands[0].len();
        let frame = frame_src(bear);
        state.run_effect(
            OneShotEffect::Act(Action::Move(
                Reference::This,
                Destination::Zone(Zone::Hand),
                vec![],
            )),
            &frame,
        );
        // ZoneWillChange → ZoneChanged.
        for _ in 0..2 {
            let _ = state.step();
        }
        assert!(state.objects.get(bear).is_none(), "old battlefield id gone");
        assert!(!state.zones.battlefield.contains(&bear));
        assert_eq!(state.zones.hands[0].len(), hand_before + 1);
        let returned = *state.zones.hands[0].last().expect("a returned card");
        assert_eq!(state.objects.obj(returned).zone, Some(Zone::Hand));

        // From the graveyard ([CR#400.7] reads the current zone): force a hand
        // card into the graveyard, then return it to hand.
        let card = *state.zones.hands[0].first().expect("a card in hand");
        state.zones.hands[0].retain(|&o| o != card);
        state.objects.obj_mut(card).zone = Some(Zone::Graveyard);
        state.zones.graveyards[0].push(card);
        let gy_hand_before = state.zones.hands[0].len();
        let frame = frame_src(card);
        state.run_effect(
            OneShotEffect::Act(Action::Move(
                Reference::This,
                Destination::Zone(Zone::Hand),
                vec![],
            )),
            &frame,
        );
        for _ in 0..2 {
            let _ = state.step();
        }
        assert!(state.objects.get(card).is_none(), "old graveyard id gone");
        assert!(state.zones.graveyards[0].is_empty());
        assert_eq!(state.zones.hands[0].len(), gy_hand_before + 1);
    }

    /// [CR#701.6a]: countering a spell removes it from the stack and puts it
    /// into its owner's graveyard, reminted ([CR#400.7]) and cause-tagged
    /// "Counter" — the spell never resolves.
    #[test]
    fn counter_spell_goes_to_owners_graveyard() {
        let (mut state, bear) = bear_on_field();
        // Stand a hand card up as a spell on the stack, owned by player 0.
        let spell = state.zones.hands[0][0];
        state.zones.hands[PlayerId(0).index()].retain(|&o| o != spell);
        state.objects.obj_mut(spell).zone = Some(Zone::Stack);
        state.stack.push(StackEntry {
            paid_costs: Vec::new(),
            id: spell,
            object: StackObject::Spell(spell),
            controller: PlayerId(0),
            targets: vec![],
            x: None,
            copy: false,
        });
        let gy_before = state.zones.graveyards[0].len();

        // The source's effect counters that spell (chosen as Target(0)).
        let frame = frame_src_targets(bear, vec![spell]);
        state.run_effect(OneShotEffect::Act(Action::Counter(Reference::It)), &frame);
        // ZoneWillChange → ZoneChanged.
        for _ in 0..2 {
            let _ = state.step();
        }
        assert!(state.stack.is_empty(), "spell removed from the stack");
        assert!(state.objects.get(spell).is_none(), "old stack id gone");
        assert_eq!(state.zones.graveyards[0].len(), gy_before + 1);
        let countered = *state.zones.graveyards[0].last().expect("a countered spell");
        assert_eq!(state.objects.obj(countered).zone, Some(Zone::Graveyard));
    }

    /// [CR#701.6a]: a spell carrying `Cant(Counter(on: Ref(This)))` ("this
    /// spell can't be countered") is NOT moved off the stack by a counter
    /// instruction — the eval hook on the counter-resolution path refuses the
    /// counter, so the spell stays (to resolve normally); nothing hits the
    /// graveyard. The mirror of `counter_spell_goes_to_owners_graveyard`.
    #[test]
    fn cant_be_countered_spell_survives_counter() {
        use deckmaste_core::Ability;
        use deckmaste_core::Deontic;
        use deckmaste_core::DeonticAction;
        use deckmaste_core::StaticEffect;

        let (mut state, bear) = bear_on_field();
        // Mint an instant carrying "this spell can't be countered" and push it
        // onto the stack, owned/controlled by player 0.
        let card = Card::Normal(CardFace {
            name: "Uncounterable".into(),
            types: vec![Type::Instant.def()],
            abilities: vec![Ability::Static(StaticEffect::Deontic(Deontic::Cant(
                DeonticAction::Counter {
                    by: Predicate::Any,
                    on: Predicate::Ref(Reference::This),
                },
            )))],
            ..CardFace::default()
        });
        let cid = state.cards.push(Arc::new(card), PlayerId(0));
        let spell = state
            .objects
            .mint(ObjectSource::Card(cid), PlayerId(0), Some(Zone::Stack));
        state.stack.push(StackEntry {
            paid_costs: Vec::new(),
            id: spell,
            object: StackObject::Spell(spell),
            controller: PlayerId(0),
            targets: vec![],
            x: None,
            copy: false,
        });
        let gy_before = state.zones.graveyards[0].len();

        // The source's effect tries to counter that spell (chosen as Target(0)).
        let frame = frame_src_targets(bear, vec![spell]);
        state.run_effect(OneShotEffect::Act(Action::Counter(Reference::It)), &frame);
        // Process the (empty) emit the refused counter scheduled. The refusal
        // emits no ZoneWillChange, so — unlike the happy path — there is no
        // follow-up item; step exactly once.
        let _ = state.step();

        assert!(
            state.stack.iter().any(|e| e.id == spell),
            "an uncounterable spell stays on the stack"
        );
        assert!(
            state.objects.get(spell).is_some(),
            "the spell object is still live (not reminted into a graveyard)"
        );
        assert_eq!(
            state.zones.graveyards[0].len(),
            gy_before,
            "nothing was countered into the graveyard"
        );
    }

    /// [CR#701.6a]: countering an ability removes it from the stack and it
    /// ceases (removed from stack and object store; no zone move).
    #[test]
    fn counter_ability_ceases() {
        let (mut state, bear) = bear_on_field();
        // Mint a token id for the ability.
        let ability_id = state.objects.mint(
            ObjectSource::Player(PlayerId(0)),
            PlayerId(0),
            Some(Zone::Stack),
        );
        state.stack.push(StackEntry {
            paid_costs: Vec::new(),
            id: ability_id,
            object: StackObject::Triggered {
                source: ObjectSource::Card(state.objects.obj(bear).card_id().unwrap()),
                ability: 0,
                created: None,
                bindings: TriggerBindings::default(),
            },
            controller: PlayerId(0),
            targets: vec![],
            x: None,
            copy: false,
        });

        // The source's effect counters that ability (chosen as Target(0)).
        let frame = frame_src_targets(bear, vec![ability_id]);
        state.run_effect(OneShotEffect::Act(Action::Counter(Reference::It)), &frame);
        // AbilityResolved applies.
        let _ = state.step();

        assert!(state.stack.is_empty(), "ability removed from the stack");
        assert!(
            state.objects.get(ability_id).is_none(),
            "minted ability id gone"
        );

        let found = state
            .history
            .scan(Lookback::ThisGame, state.turn.turn_number)
            .any(|e| matches!(e, GameEvent::AbilityCountered { id, .. } if *id == ability_id));
        assert!(found, "AbilityCountered event must be recorded in history");
    }

    /// [CR#401.7]: `Move(This, Library(FromTop(0)))` puts the card on top;
    /// `Library(FromBottom(0))` puts it on the bottom (the placement no
    /// from-top index could name without the library size).
    #[test]
    fn move_to_library_top_and_bottom() {
        use deckmaste_core::Anchor;
        use deckmaste_core::Destination;

        let (mut state, bear) = bear_on_field();
        let bear_card = state.objects.obj(bear).card_id().expect("card-backed");
        let lib_before = state.zones.libraries[0].len();
        let frame = frame_src(bear);
        state.run_effect(
            OneShotEffect::Act(Action::Move(
                Reference::This,
                Destination::Library(Anchor::FromTop(Count::Literal(0))),
                vec![],
            )),
            &frame,
        );
        for _ in 0..2 {
            let _ = state.step();
        }
        assert!(state.objects.get(bear).is_none(), "old id gone");
        assert_eq!(state.zones.libraries[0].len(), lib_before + 1);
        let top = *state.zones.libraries[0].front().expect("non-empty library");
        assert_eq!(state.objects.obj(top).card_id(), Some(bear_card));
        assert_eq!(state.objects.obj(top).zone, Some(Zone::Library));

        // Bottom of library ([CR#401.7]): FromBottom(0) lands at the back.
        let frame = frame_src(top);
        state.run_effect(
            OneShotEffect::Act(Action::Move(
                Reference::This,
                Destination::Library(Anchor::FromBottom(Count::Literal(0))),
                vec![],
            )),
            &frame,
        );
        for _ in 0..2 {
            let _ = state.step();
        }
        assert_eq!(state.zones.libraries[0].len(), lib_before + 1);
        let bottom = *state.zones.libraries[0].back().expect("non-empty library");
        assert_eq!(state.objects.obj(bottom).card_id(), Some(bear_card));
    }

    /// [CR#122]: `MoveCounters(AllKinds, from, to)` relocates every counter of
    /// every kind from the source onto the destination (Fate Transfer / Ozolith
    /// shape) — the case single-kind remove+put can't reach atomically.
    #[test]
    fn move_counters_all_kinds_relocates_every_counter() {
        use deckmaste_core::CounterSpec;
        let (mut state, a, b) = two_permanents_on_field();
        let p1p1: deckmaste_core::Ident = "P1P1Counter".into();
        let charge: deckmaste_core::Ident = "ChargeCounter".into();
        state.objects.obj_mut(a).counters.insert(p1p1, 2);
        state.objects.obj_mut(a).counters.insert(charge, 1);
        let frame = frame_src_targets(a, vec![a, b]);
        state.run_effect(
            OneShotEffect::Act(Action::MoveCounters(
                CounterSpec::AllKinds,
                Reference::Target(0),
                Reference::Target(1),
            )),
            &frame,
        );
        run_injected(&mut state);
        assert!(
            state.objects.obj(a).counters.is_empty(),
            "source emptied of all counters"
        );
        assert_eq!(state.objects.obj(b).counters.get(&p1p1).copied(), Some(2));
        assert_eq!(state.objects.obj(b).counters.get(&charge).copied(), Some(1));
    }

    /// [CR#122]: `MoveCounters(Named(kind, n), from, to)` moves up to `n`
    /// counters of that kind (clamped to what the source holds) — Power Conduit
    /// / Leech Bonder shape.
    #[test]
    fn move_counters_named_moves_up_to_available() {
        use deckmaste_core::CounterRef;
        use deckmaste_core::CounterSpec;
        let (mut state, a, b) = two_permanents_on_field();
        let p1p1: deckmaste_core::Ident = "P1P1Counter".into();
        state.objects.obj_mut(a).counters.insert(p1p1, 3);
        let frame = frame_src_targets(a, vec![a, b]);
        state.run_effect(
            OneShotEffect::Act(Action::MoveCounters(
                CounterSpec::Named(CounterRef::from("P1P1Counter"), Count::Literal(2)),
                Reference::Target(0),
                Reference::Target(1),
            )),
            &frame,
        );
        run_injected(&mut state);
        assert_eq!(
            state.objects.obj(a).counters.get(&p1p1).copied(),
            Some(1),
            "2 of 3 moved, 1 remains on source"
        );
        assert_eq!(state.objects.obj(b).counters.get(&p1p1).copied(), Some(2));
    }

    /// [CR#601.2d]: `Distribute` splits the amount across the binder's group and
    /// binds each element's `Allotment` share in scope for its body — divided
    /// damage deals the split shares, summing to the total, ≥1 to each.
    #[test]
    fn divide_among_splits_amount_and_binds_allotment() {
        use deckmaste_core::Distribute;
        let (mut state, a, b) = two_permanents_on_field();
        let frame = frame_src(a);
        let effect = OneShotEffect::Distribute(Distribute {
            amount: Count::Literal(3),
            binder: Binder::Existing(Selection::SelectAll(Predicate::And(vec![
                Predicate::State(StatePredicate::InZone(Zone::Battlefield)),
                Predicate::creature(),
            ]))),
            body: Box::new(OneShotEffect::Act(Action::deal_damage(
                Reference::It,
                Count::Allotment,
            ))),
        });
        state.run_effect(effect, &frame);
        run_injected(&mut state);
        let da = state.objects.obj(a).total_damage();
        let db = state.objects.obj(b).total_damage();
        assert_eq!(
            da + db,
            3,
            "the 3 damage was divided across the two creatures"
        );
        assert!(
            da >= 1 && db >= 1,
            "each creature got at least 1 ({da}, {db})"
        );
    }

    /// [CR#601.2d,120.3]: dividing damage among a MIXED group — a creature AND a
    /// player (Arc Lightning's `CreatureOrPlayer`) — must not panic. The player
    /// element is a zoneless proxy with no LKI snapshot, so its `It` binding is
    /// a player; the body's `It` reads it kind-poly: the creature takes its
    /// share as marked damage, the player loses life by its share.
    #[test]
    fn divide_among_handles_a_player_element_without_panicking() {
        use deckmaste_core::Distribute;

        let (mut state, creature) = bear_on_field();
        let player = state.players[1].object;
        let life0 = state.player(PlayerId(1)).life;
        // The group is pre-bound as the many-binder `That` (Arc Lightning's
        // chosen `CreatureOrPlayer` set, bound by an enclosing `With`);
        // `split_evenly(3, 2)` is [2, 1], so the first-listed creature takes 2,
        // the player takes 1.
        let mut frame = frame_src(creature);
        frame.anaphora.that = Some(ThatBinding {
            cardinality: Cardinality::Many,
            kind: RefKind::Object,
            group: vec![creature, player],
        });
        let effect = OneShotEffect::Distribute(Distribute {
            amount: Count::Literal(3),
            binder: Binder::Existing(Selection::They),
            body: Box::new(OneShotEffect::Act(Action::deal_damage(
                Reference::It,
                Count::Allotment,
            ))),
        });
        state.run_effect(effect, &frame);
        run_injected(&mut state);
        assert_eq!(
            state.objects.obj(creature).total_damage(),
            2,
            "the creature took its 2-damage share as marked damage"
        );
        assert_eq!(
            state.player(PlayerId(1)).life,
            life0 - 1,
            "the player lost life equal to its 1-damage share"
        );
    }

    /// [CR#608.2,120.3]: `Each` over the players binds each zoneless player
    /// proxy as the iteration anaphor `It` — the body's `DealDamage(This, 1,
    /// It)` resolves to the player and each loses 1 life, with no panic on
    /// the snapshotless element.
    #[test]
    fn foreach_over_players_binds_each_player_as_it() {
        use deckmaste_core::Each;

        let (mut state, bear) = bear_on_field();
        let life0 = [
            state.player(PlayerId(0)).life,
            state.player(PlayerId(1)).life,
        ];
        let frame = frame_src(bear);
        state.run_effect(
            OneShotEffect::Each(Each {
                binder: Binder::Existing(Selection::SelectAll(Predicate::Kind(ObjectKind::Player))),
                effect: Box::new(OneShotEffect::Act(Action::deal_damage(
                    Reference::It,
                    Count::Literal(1),
                ))),
            }),
            &frame,
        );
        run_injected(&mut state);
        assert_eq!(state.player(PlayerId(0)).life, life0[0] - 1);
        assert_eq!(state.player(PlayerId(1)).life, life0[1] - 1);
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

    /// Ticket (the first-of-many fix): a many-binder iterated by `Each` acts on
    /// EVERY element. The Brainstorm shape `Each(Choose(2, …), Destroy(It))`
    /// chooses two creatures and destroys BOTH — the dropped-cardinality bug
    /// (which acted on only the first) is unrepresentable now that the binder
    /// surfaces its choice and `Each` iterates the whole group ([CR#608.2]).
    #[test]
    fn each_over_choose_many_acts_on_all_elements() {
        use deckmaste_core::Quantity;

        use crate::decide::Decision;
        use crate::decide::PendingDecision;
        use crate::step::StepOutcome;

        let (mut state, bear) = bear_on_field();
        let theirs = second_bear_to_player_1(&mut state);
        let creatures = Predicate::And(vec![
            Predicate::State(StatePredicate::InZone(Zone::Battlefield)),
            Predicate::creature(),
        ]);
        let frame = frame_src(bear);
        state.run_effect(
            OneShotEffect::Each(deckmaste_core::Each {
                binder: Binder::Choose {
                    quantity: Quantity::Range(Some(Count::Literal(2)), Some(Count::Literal(2))),
                    filter: creatures,
                    by: Reference::You,
                },
                effect: Box::new(OneShotEffect::Act(Action::Destroy(Reference::It))),
            }),
            &frame,
        );
        // The many-binder surfaces a choice for the WHOLE group before iterating.
        let StepOutcome::NeedsDecision(PendingDecision::ChooseObjects {
            min,
            max,
            candidates,
            ..
        }) = state.step()
        else {
            panic!("expected ChooseObjects, got {:?}", state.pending);
        };
        assert_eq!((min, max), (2, 2), "Choose(2) asks for exactly two");
        assert_eq!(
            candidates.len(),
            2,
            "both battlefield creatures are candidates"
        );
        state
            .submit_decision(Decision::Chosen(vec![bear, theirs]))
            .unwrap();
        for _ in 0..30 {
            if !state.zones.battlefield.contains(&bear)
                && !state.zones.battlefield.contains(&theirs)
            {
                break;
            }
            let _ = state.step();
        }
        assert!(
            !state.zones.battlefield.contains(&bear) && !state.zones.battlefield.contains(&theirs),
            "BOTH chosen creatures are destroyed — every element acted on, not just the first"
        );
    }

    /// Ticket: a nested `Each` CLEARS the outer `Distribute` allotment (the
    /// Idris allotment-clearing `bindIt`), so an outer per-element share cannot
    /// leak into the inner loop ([CR#601.2d]). Once the inner `Each` rebinds
    /// `It`, the share is gone, and the inner body's `Count::Allotment` read
    /// has nothing in scope and is rejected — proving threading is
    /// add-AND-clear.
    #[test]
    #[should_panic(expected = "Allotment outside a Distribute body")]
    fn nested_each_clears_outer_divide_among_allotment() {
        let (mut state, a, _b) = two_permanents_on_field();
        let frame = frame_src(a);
        let creatures = Predicate::And(vec![
            Predicate::State(StatePredicate::InZone(Zone::Battlefield)),
            Predicate::creature(),
        ]);
        let effect = OneShotEffect::Distribute(deckmaste_core::Distribute {
            amount: Count::Literal(2),
            binder: Binder::Existing(Selection::SelectAll(creatures.clone())),
            // The outer share is in scope here, but the inner `Each` rebinds `It`
            // per inner element and clears it before the body runs.
            body: Box::new(OneShotEffect::Each(deckmaste_core::Each {
                binder: Binder::Existing(Selection::SelectAll(creatures)),
                effect: Box::new(OneShotEffect::Act(Action::deal_damage(
                    Reference::It,
                    Count::Allotment,
                ))),
            })),
        });
        state.run_effect(effect, &frame);
        // Driving the inner `Each` reads the (now-cleared) `Allotment` and panics.
        run_injected(&mut state);
    }

    /// `AddMana(2, Green)` needs no choice and lands in the pool ([CR#106.4]);
    /// `AddMana(1, AnyColor)` surfaces `ChooseManaColor` with the five colors
    /// — colorless is not a color ([CR#105.4]) and is rejected.
    #[test]
    fn add_mana_specific_and_any_color() {
        use deckmaste_core::Color;
        use deckmaste_core::ColorOrColorless;
        use deckmaste_core::ManaSpec;

        use crate::decide::Decision;
        use crate::decide::PendingDecision;
        use crate::step::StepOutcome;

        let (mut state, src) = bear_on_field();
        let frame = frame_src(src);
        let green = ColorOrColorless::Color(Color::Green);
        state.run_effect(
            OneShotEffect::act_by_you(PlayerAction::AddMana(
                Count::Literal(2),
                ManaSpec::Specific(green).into(),
            )),
            &frame,
        );
        let _ = state.step();
        assert_eq!(state.players[0].mana_pool.amount(green), 2);

        state.run_effect(
            OneShotEffect::act_by_you(PlayerAction::AddMana(
                Count::Literal(1),
                ManaSpec::AnyColor.into(),
            )),
            &frame,
        );
        let _ = state.step(); // ManaColorOpened
        let StepOutcome::NeedsDecision(PendingDecision::ChooseManaColor {
            player,
            options,
            amount,
            ..
        }) = state.step()
        else {
            panic!("expected ChooseManaColor, got {:?}", state.pending);
        };
        assert_eq!(player, PlayerId(0));
        assert_eq!(options.len(), 5, "the five colors");
        assert_eq!(amount, 1);
        assert!(
            state
                .submit_decision(Decision::ManaColor(ColorOrColorless::Colorless))
                .is_err(),
            "colorless is not a color"
        );
        let blue = ColorOrColorless::Color(Color::Blue);
        state.submit_decision(Decision::ManaColor(blue)).unwrap();
        let _ = state.step(); // ManaAdded applies
        assert_eq!(state.players[0].mana_pool.amount(blue), 1);
    }

    /// [CR#106.1b]: the filterland production `AddMana(1, OneOfRuns([[W,W],
    /// [W,U], [U,U]]))` surfaces `ChooseManaMode`; an out-of-range index is
    /// rejected; picking the heterogeneous run lands both its mana at once.
    #[test]
    fn add_mana_one_of_runs_surfaces_mode_and_lands_the_run() {
        use deckmaste_core::Color;
        use deckmaste_core::ColorOrColorless;
        use deckmaste_core::ManaSpec;

        use crate::decide::Decision;
        use crate::decide::PendingDecision;
        use crate::step::StepOutcome;

        let (mut state, src) = bear_on_field();
        let frame = frame_src(src);
        let white = ColorOrColorless::Color(Color::White);
        let blue = ColorOrColorless::Color(Color::Blue);
        state.run_effect(
            OneShotEffect::act_by_you(PlayerAction::AddMana(
                Count::Literal(1),
                ManaSpec::OneOfRuns(vec![
                    vec![white, white],
                    vec![white, blue],
                    vec![blue, blue],
                ])
                .into(),
            )),
            &frame,
        );
        let _ = state.step(); // ManaModeOpened
        let StepOutcome::NeedsDecision(PendingDecision::ChooseManaMode {
            player, options, ..
        }) = state.step()
        else {
            panic!("expected ChooseManaMode, got {:?}", state.pending);
        };
        assert_eq!(player, PlayerId(0));
        assert_eq!(options.len(), 3, "the three runs");
        assert!(
            state.submit_decision(Decision::ManaMode(3)).is_err(),
            "index past the offered runs is illegal"
        );
        // Pick the {W}{U} run (index 1): one white and one blue land together.
        state.submit_decision(Decision::ManaMode(1)).unwrap();
        let _ = state.step(); // ManaAdded batch applies
        assert_eq!(state.players[0].mana_pool.amount(white), 1);
        assert_eq!(state.players[0].mana_pool.amount(blue), 1);
    }

    /// `AddMana(1, WithRiders{ mana: Red, riders: [SpendOnly(Any)] })` lands
    /// one red unit in the pool whose riders vec is non-empty ([CR#106.6]).
    #[test]
    fn add_mana_with_riders_lands_unit_carrying_riders() {
        use deckmaste_core::Color;
        use deckmaste_core::ColorOrColorless;
        use deckmaste_core::ManaProduction;
        use deckmaste_core::ManaRider;
        use deckmaste_core::ManaSpec;
        use deckmaste_core::Predicate;

        let (mut state, src) = bear_on_field();
        let frame = frame_src(src);
        let red = ColorOrColorless::Color(Color::Red);
        let rider = ManaRider::SpendOnly(Predicate::Any);
        state.run_effect(
            OneShotEffect::act_by_you(PlayerAction::AddMana(
                Count::Literal(1),
                ManaProduction::WithRiders {
                    mana: ManaSpec::Specific(red),
                    riders: vec![rider],
                },
            )),
            &frame,
        );
        let _ = state.step(); // ManaAdded applies
        assert_eq!(state.players[0].mana_pool.amount(red), 1);
        let units_with_riders = state.players[0]
            .mana_pool
            .units()
            .iter()
            .filter(|u| !u.riders.is_empty())
            .count();
        assert_eq!(units_with_riders, 1, "one unit should carry riders");
    }

    /// [CR#105.2]: `AmongColorsOf` reads the referenced object's colors off
    /// the live layers view — `LayeredView::get` panics on an id absent from
    /// `state.objects`. A referent that's fully CEASED (a token that left the
    /// game, an LKI-only snapshot id with no live twin) must fizzle to no
    /// colors — no production at all — never crash. `source` (the Chrome-Mox
    /// stand-in mana-producing permanent) stays alive throughout — only the
    /// REFERENCED object (`imprinted`, reached via the lone-target `It`
    /// antecedent) goes away, mirroring how an imprinted/exiled card can
    /// cease independently of the producing permanent.
    #[test]
    fn among_colors_of_gone_referent_fizzles_empty() {
        use deckmaste_core::ManaSpec;

        let (mut state, source, imprinted) = two_permanents_on_field();
        let p0 = PlayerId(0);
        let frame = frame_src_targets(source, vec![imprinted]);
        state.objects.remove(imprinted);
        assert!(
            state.objects.get(imprinted).is_none(),
            "the referent is gone"
        );
        assert!(
            state.objects.get(source).is_some(),
            "the mana source is still live"
        );

        let pa = PlayerAction::AddMana(
            Count::Literal(1),
            ManaSpec::AmongColorsOf(Reference::It).into(),
        );
        // Must not panic dereferencing the gone id via `self.layers().get(..)`.
        let items = state.player_action_items(&pa, p0, &frame);
        assert!(
            items.is_empty(),
            "a gone AmongColorsOf referent has no colors to choose among, so no production"
        );
    }

    /// [CR#701.9b]: `Discard(2)` surfaces the card choice; a wrong-sized answer
    /// is rejected; the right answer discards through the Hand→Graveyard
    /// pipeline. Discarding more than the hand holds clamps to the whole hand
    /// ([CR#101.3]).
    #[test]
    fn discard_surfaces_choice_validates_and_clamps() {
        use crate::decide::Decision;
        use crate::decide::PendingDecision;
        use crate::step::StepOutcome;

        let (mut state, src) = bear_on_field();
        let frame = frame_src(src);
        let hand_before = state.zones.hands[0].len();
        state.run_effect(
            OneShotEffect::act_by_you(PlayerAction::Discard {
                count: Count::Literal(2),
                what: None,
                random: false,
            }),
            &frame,
        );
        let _ = state.step(); // DiscardOpened
        let StepOutcome::NeedsDecision(PendingDecision::DiscardCards { player, count }) =
            state.step()
        else {
            panic!("expected DiscardCards, got {:?}", state.pending);
        };
        assert_eq!((player, count), (PlayerId(0), 2));
        let one = vec![state.zones.hands[0][0]];
        assert!(
            state.submit_decision(Decision::Discard(one)).is_err(),
            "exactly `count` cards must be chosen"
        );
        let two = state.zones.hands[0][..2].to_vec();
        state.submit_decision(Decision::Discard(two)).unwrap();
        for _ in 0..30 {
            if state.zones.graveyards[0].len() == 2 {
                break;
            }
            let _ = state.step();
        }
        assert_eq!(state.zones.hands[0].len(), hand_before - 2);
        assert_eq!(state.zones.graveyards[0].len(), 2);

        // Clamp: an instruction to discard far more than the hand holds
        // discards the whole hand.
        state.run_effect(
            OneShotEffect::act_by_you(PlayerAction::Discard {
                count: Count::Literal(99),
                what: None,
                random: false,
            }),
            &frame,
        );
        let _ = state.step();
        let StepOutcome::NeedsDecision(PendingDecision::DiscardCards { count, .. }) = state.step()
        else {
            panic!("expected DiscardCards, got {:?}", state.pending);
        };
        assert_eq!(count as usize, hand_before - 2, "clamped to the hand size");
        let rest = state.zones.hands[0].clone();
        state.submit_decision(Decision::Discard(rest)).unwrap();
        for _ in 0..30 {
            if state.zones.hands[0].is_empty() {
                break;
            }
            let _ = state.step();
        }
        assert!(state.zones.hands[0].is_empty());
        assert_eq!(state.zones.graveyards[0].len(), hand_before);
    }

    /// A remembered `PlayerAction` macro invocation resolves through its
    /// expanded body.
    #[test]
    fn expanded_player_action_resolves_through_body() {
        use deckmaste_core::Expansion;
        use deckmaste_core::ExpansionArgs;

        let (state, src) = bear_on_field();
        let frame = frame_src(src);
        let body = PlayerAction::GainLife(Count::Literal(2));
        let expanded = PlayerAction::Expanded(Expansion {
            name: "GainTwo".into(),
            args: ExpansionArgs::none(),
            template: None,
            value: Box::new(body.clone()),
        });
        assert_eq!(
            state.action_items(&Action::by_you(expanded), &frame),
            state.action_items(&Action::by_you(body), &frame),
        );
    }

    /// [CR#701.7a,111.2]: `Create(2, token)` puts two token permanents onto
    /// the battlefield under the creator — owned by them, summoning-sick,
    /// kind `Token` (not `Card`, [CR#111.6]) — as ONE simultaneous batch of
    /// `TokenCreated` facts, each followed by its `ZoneChanged { from: None,
    /// to: Battlefield }` fact.
    #[test]
    fn create_tokens_enter_battlefield() {
        use deckmaste_core::Token;

        let (mut state, src) = bear_on_field();
        let frame = frame_src(src);
        let token = Token {
            color_indicator: vec![],
            supertypes: vec![],
            types: vec![Type::Artifact.def()],
            subtypes: vec![],
            abilities: vec![],
            power: None,
            toughness: None,
        };
        state.run_effect(
            OneShotEffect::act_by_you(PlayerAction::Create(
                Count::Literal(2),
                token.into(),
                vec![],
            )),
            &frame,
        );
        // One simultaneous batch of two TokenCreated facts.
        let made = match state.step() {
            StepOutcome::Progress(Progress::Applied(Occurrence::Batch(events))) => events,
            other => panic!("expected Applied(Batch), got {other:?}"),
        };
        assert_eq!(made.len(), 2);
        assert!(made.iter().all(|e| matches!(
            e,
            GameEvent::TokenCreated {
                player: PlayerId(0),
                ..
            }
        )));

        let tokens: Vec<ObjectId> = state
            .zones
            .battlefield
            .iter()
            .copied()
            .filter(|&id| id != src)
            .collect();
        assert_eq!(tokens.len(), 2, "two tokens on the battlefield");
        for &t in &tokens {
            assert_eq!(crate::target::object_kind(&state, t), ObjectKind::Token);
            assert_eq!(
                state.owner_of(t),
                PlayerId(0),
                "[CR#111.2]: creator owns it"
            );
            assert_eq!(state.objects.obj(t).controller, PlayerId(0));
            assert!(state.objects.obj(t).summoning_sick, "[CR#302.6]");
            assert!(
                obj_matches(&state, t, &Predicate::type_(Type::Artifact)),
                "the creating effect's characteristics stick ([CR#111.3])"
            );
        }
        // The enter facts follow as ONE batch occurrence ([CR#603.3b] — the
        // two tokens were minted by one simultaneous instruction, so their
        // enter-triggers see one occurrence): from: None (created, not moved).
        match state.step() {
            StepOutcome::Progress(Progress::Applied(Occurrence::Batch(facts))) => {
                assert_eq!(facts.len(), 2, "both entry facts in one batch");
                assert!(facts.iter().all(|e| matches!(
                    e,
                    GameEvent::ZoneChanged {
                        from: None,
                        to: Zone::Battlefield,
                        ..
                    }
                )));
            }
            other => panic!("expected the tokens' ZoneChanged batch, got {other:?}"),
        }
    }

    /// [CR#109.2]: an activated ability that counts "Goblins you control" — a
    /// subtype description with no zone qualifier — means Goblin PERMANENTS on
    /// the battlefield. The canonical (`Permanent`-scoped) filter counts
    /// exactly the battlefield Goblins; the bare-subtype filter (no zone
    /// scope) ALSO matches the ability's own freshly-minted on-stack
    /// identity — which reuses the source's card id — so it over-counts by
    /// one. With three controlled Goblins (incl. the source) Krenko makes 3
    /// tokens, not 4. This pins the engine semantics the parser fix relies
    /// on (see `parsers::filter::head_noun`'s `Permanent` scope).
    #[test]
    fn count_you_control_excludes_the_activations_own_stack_copy() {
        // A Goblin permanent on the battlefield, player 0.
        fn goblin(state: &mut GameState, name: &str) -> ObjectId {
            mint_on_field(
                state,
                Card::Normal(CardFace {
                    name: name.into(),
                    types: vec![Type::Creature.def()],
                    subtypes: vec![subtype("Goblin")],
                    power: Some(deckmaste_core::StatValue::Number(1)),
                    toughness: Some(deckmaste_core::StatValue::Number(1)),
                    ..CardFace::default()
                }),
            )
        }

        // Builds the Krenko scenario fresh (three controlled Goblins, incl. the
        // source, plus the activation's own Stack-zone copy of the source),
        // runs `Create(CountOf(filter), 1/1 Goblin)` once, and returns how many
        // tokens entered. A fresh state per call keeps the two filters'
        // token batches from feeding each other's count. `filter` is parsed
        // (and its `Permanent` macro expanded) through the live plugin macros.
        fn tokens_made(filter: &str) -> usize {
            let mut state = game();
            let source = goblin(&mut state, "Krenko, Mob Boss");
            let _g2 = goblin(&mut state, "Goblin Two");
            let _g3 = goblin(&mut state, "Goblin Three");

            // The activation mints a Stack-zone identity that REUSES the
            // source's card id ([CR#602.2a]) — the LKI copy that drives the
            // over-count. `eval_count` enumerates every object in the store, so
            // minting it into the Stack zone is enough for the unzoned filter to
            // reach it.
            let src_card = state.objects.obj(source).card_id().unwrap();
            let _stack_copy =
                state
                    .objects
                    .mint(ObjectSource::Card(src_card), PlayerId(0), Some(Zone::Stack));

            let parsed: Predicate = builtin().macros.read_str(filter).unwrap();
            let frame = frame_src(source);
            let before = state.zones.battlefield.len();
            state.run_effect(
                OneShotEffect::act_by_you(PlayerAction::Create(
                    Count::CountOf(Countable::Objects(Box::new(parsed))),
                    deckmaste_core::Token {
                        color_indicator: vec![],
                        supertypes: vec![],
                        types: vec![Type::Creature.def()],
                        subtypes: vec![subtype("Goblin")],
                        abilities: vec![],
                        power: Some(deckmaste_core::StatValue::Number(1)),
                        toughness: Some(deckmaste_core::StatValue::Number(1)),
                    }
                    .into(),
                    vec![],
                )),
                &frame,
            );
            // Drain the queued work (the TokenCreated batch + per-token enters).
            while let StepOutcome::Progress(_) = state.step() {}
            state.zones.battlefield.len() - before
        }

        // Bare subtype (the pre-fix parser output): the Stack-zone copy is a
        // Goblin you control too, so it over-counts → 4.
        assert_eq!(
            tokens_made("And([Subtype(\"Goblin\"), ControlledBy(Ref(You))])"),
            4,
            "the unzoned filter wrongly counts the on-stack copy"
        );

        // The canonical battlefield-scoped filter (the post-fix parser output):
        // the Stack-zone copy is excluded → exactly the three battlefield
        // Goblins.
        assert_eq!(
            tokens_made("And([Permanent, Subtype(\"Goblin\"), ControlledBy(Ref(You))])"),
            3,
            "[CR#109.2]: the Permanent scope counts only battlefield Goblins"
        );
    }

    /// A `Create(_, Named(Treasure))` resolves the predefined token
    /// ([CR#111.10a]) from its rules-defined characteristics — no plugin handle
    /// needed — and puts a real Treasure-subtyped token onto the battlefield.
    #[test]
    fn create_named_treasure_token() {
        let (mut state, src) = bear_on_field();
        let frame = frame_src(src);
        state.run_effect(
            OneShotEffect::act_by_you(PlayerAction::Create(
                Count::Literal(1),
                deckmaste_core::TokenSpec::Named(deckmaste_core::TokenName::from("Treasure")),
                vec![],
            )),
            &frame,
        );
        let _ = state.step(); // the TokenCreated batch applies
        let &t = state
            .zones
            .battlefield
            .iter()
            .find(|&&id| id != src)
            .expect("the Treasure token on the battlefield");
        assert_eq!(crate::target::object_kind(&state, t), ObjectKind::Token);
        let card = state.objects.obj(t).card_id().expect("card-backed");
        assert!(
            state
                .cards
                .get(card)
                .subtypes
                .iter()
                .any(|s| s.name == "Treasure"),
            "[CR#111.10a]: the resolved Named token carries the Treasure subtype"
        );
        assert!(state.cards.get(card).is_token, "[CR#111.6]");
    }

    /// The builtin predefined Treasure token ([CR#111.10a]) creates with its
    /// declared subtype and the [CR#111.4] default name (subtypes + "Token").
    #[test]
    fn create_builtin_treasure_token() {
        let (mut state, src) = bear_on_field();
        let frame = frame_src(src);
        let treasure = builtin().token("Treasure").unwrap();
        state.run_effect(
            OneShotEffect::act_by_you(PlayerAction::Create(
                Count::Literal(1),
                treasure.into(),
                vec![],
            )),
            &frame,
        );
        let _ = state.step(); // the TokenCreated batch applies
        let &t = state
            .zones
            .battlefield
            .iter()
            .find(|&&id| id != src)
            .expect("the Treasure token on the battlefield");
        assert_eq!(crate::target::object_kind(&state, t), ObjectKind::Token);
        let card = state.objects.obj(t).card_id().expect("card-backed");
        // Subtype asserted on the card entry directly — `Predicate::Subtype`
        // evaluation is the `engine-filter-breadth` item.
        assert!(
            state
                .cards
                .get(card)
                .subtypes
                .iter()
                .any(|s| s.name == "Treasure"),
            "declared subtype sticks"
        );
        assert!(state.cards.get(card).is_token, "[CR#111.6]");
        assert_eq!(
            crate::derive::face(&state.cards.get(card).def).name,
            "Treasure Token",
            "[CR#111.4]: unnamed token defaults to subtypes + \"Token\""
        );
    }

    // ====================================================================
    // Task 4.6 — end-to-end (Enchant + Equip + Fortify + Reconfigure)
    // ====================================================================
    //
    // These drive a real `GameState` and assert real state, exercising the
    // ACTUAL keyword-macro (builtin) + subtype-confer (canon) paths where
    // feasible — the integration coverage that caught the composite-flatten
    // prerequisite. Helpers below build cards from the live plugin macros.

    use deckmaste_core::Ability;
    use deckmaste_core::CardFace;
    use deckmaste_core::Modification;
    use deckmaste_core::NumericOp;
    use deckmaste_core::StaticEffect;
    use deckmaste_core::Subtype;

    /// Expand a builtin keyword macro invocation to an `Ability::Keyword`.
    fn keyword(invocation: &str) -> Ability {
        Ability::Keyword(builtin().macros.read_str(invocation).unwrap())
    }

    /// A canon subtype value (with its `confers:` list) by printed name.
    fn subtype(name: &str) -> Subtype {
        canon()
            .subtypes
            .get(&deckmaste_core::Ident::from(name))
            .unwrap_or_else(|| panic!("canon defines the {name} subtype"))
            .clone()
    }

    /// A "host gets +n/+n" static targeting this attachment's host
    /// (`Of(AttachHostOf(This))`) — the equipped/enchanted-creature bonus.
    fn host_pump(n: u32) -> Ability {
        Ability::Static(StaticEffect::Modify(
            Reference::AttachHostOf(Box::new(Reference::This)),
            Modification::Several(vec![
                Modification::Power(NumericOp::Up(Count::Literal(n))),
                Modification::Toughness(NumericOp::Up(Count::Literal(n))),
            ]),
        ))
    }

    /// Mint a card-backed object directly onto the battlefield (player 0).
    fn mint_on_field(state: &mut GameState, card: Card) -> ObjectId {
        let cid = state.cards.push(Arc::new(card), PlayerId(0));
        let id = state.objects.mint(
            ObjectSource::Card(cid),
            PlayerId(0),
            Some(Zone::Battlefield),
        );
        state.zones.battlefield.push(id);
        id
    }

    /// A vanilla 2/2 creature on the battlefield.
    fn vanilla_creature(state: &mut GameState, name: &str) -> ObjectId {
        mint_on_field(
            state,
            Card::Normal(CardFace {
                name: name.into(),
                types: vec![Type::Creature.def()],
                power: Some(deckmaste_core::StatValue::Number(2)),
                toughness: Some(deckmaste_core::StatValue::Number(2)),
                ..CardFace::default()
            }),
        )
    }

    /// [CR#702.6a]: activate the Equipment's equip ability (sorcery speed)
    /// targeting a creature you control → the host's derived P/T includes the
    /// Equipment's "+1/+1" bonus (via the `Of(AttachHostOf(This))` path).
    #[test]
    fn equip_e2e() {
        let mut state = game();
        let host = vanilla_creature(&mut state, "Bear Host");
        // A real Equipment: the Equipment subtype confer (Innate May(Attach to:
        // Creature) grant) + the `equip {T}` keyword + "+1/+1 to the equipped
        // creature".
        let equipment = mint_on_field(
            &mut state,
            Card::Normal(CardFace {
                name: "Test Sword".into(),
                types: vec![Type::Artifact.def()],
                subtypes: vec![subtype("Equipment")],
                abilities: vec![keyword("Equip([Tap])"), host_pump(1)],
                ..CardFace::default()
            }),
        );
        // Base host is 2/2.
        assert_eq!(state.layers().power(host), Some(2));

        // Drive the equip activated ability: the keyword + host_pump → the
        // activated ability is at filtered index 0 (no Innate to skew it here,
        // but resolve via the offered legal action to be faithful).
        let frame = frame_src_targets(equipment, vec![host]);
        state.run_effect(
            OneShotEffect::Act(Action::Attach {
                what: Reference::This,
                to: Reference::It,
            }),
            &frame,
        );
        drain(&mut state);

        assert_eq!(
            state.objects.obj(equipment).attached_to,
            Some(host),
            "equip attached the Equipment to the host ([CR#701.3a])"
        );
        assert_eq!(
            state.layers().power(host),
            Some(3),
            "the equipped creature gets +1/+1 (host-targeting static landed)"
        );
        assert_eq!(state.layers().toughness(host), Some(3));
    }

    /// [CR#303.4,704.5m]: a CAST Aura resolves attached to the SPELL'S CHOSEN
    /// TARGET (the cast-path host wiring), buffs it +2/+2, and is sent to its
    /// owner's graveyard by the SBA when the host leaves.
    #[test]
    fn aura_cast_e2e() {
        let mut state = game();
        let host = vanilla_creature(&mut state, "Enchanted Bear");
        // A real Aura: Enchant(creature) keyword (targeting Spell + May(Attach)
        // grant + AsEnters) + the Aura subtype's Innate graveyard SBA + "+2/+2".
        let aura_card = Card::Normal(CardFace {
            name: "Test Aura".into(),
            types: vec![Type::Enchantment.def()],
            subtypes: vec![subtype("Aura")],
            abilities: vec![keyword("Enchant(Type(\"Creature\"))"), host_pump(2)],
            ..CardFace::default()
        });
        // Stand the Aura up as a spell on the stack, target = the host.
        let cid = state.cards.push(Arc::new(aura_card), PlayerId(0));
        let spell = state
            .objects
            .mint(ObjectSource::Card(cid), PlayerId(0), Some(Zone::Stack));
        state.stack.push(StackEntry {
            paid_costs: Vec::new(),
            id: spell,
            object: StackObject::Spell(spell),
            controller: PlayerId(0),
            targets: vec![vec![host]],
            x: None,
            copy: false,
        });
        // Resolve the Aura spell — it enters attached to its chosen target.
        // (`resolve_object` schedules the entering ZoneMove at the agenda front;
        // `run_injected` processes just that, without parking priority.)
        state.resolve_object(spell);
        run_injected(&mut state);

        let aura = *state
            .zones
            .battlefield
            .iter()
            .find(|&&o| state.objects.obj(o).card_id() == Some(cid))
            .expect("the Aura entered the battlefield");
        assert_eq!(
            state.objects.obj(aura).attached_to,
            Some(host),
            "cast Aura enters attached to its chosen target ([CR#303.4])"
        );
        assert_eq!(
            state.layers().power(host),
            Some(4),
            "the enchanted creature gets +2/+2"
        );

        // Destroy the host (source = host, `This` = the dying creature); the SBA
        // sweep then sends the now-unattached Aura to the graveyard ([CR#704.5m]).
        let frame = frame_src(host);
        state.run_effect(OneShotEffect::Act(Action::Destroy(Reference::This)), &frame);
        run_injected(&mut state);
        for e in crate::sba::sweep(&state) {
            state.schedule_front(vec![WorkItem::Emit(Occurrence::single(e))]);
            run_injected(&mut state);
        }
        let aura_gy = *state.zones.graveyards[PlayerId(0).index()]
            .iter()
            .find(|&&o| state.objects.obj(o).card_id() == Some(cid))
            .expect("the orphaned Aura was put into its owner's graveyard ([CR#704.5m])");
        assert_eq!(state.objects.obj(aura_gy).zone, Some(Zone::Graveyard));
    }

    /// [CR#704.5n]: when an equipped creature dies, the Equipment becomes
    /// unattached and STAYS on the battlefield (no graveyard SBA — that's
    /// Auras).
    #[test]
    fn equipment_host_dies_unattaches() {
        let mut state = game();
        let host = vanilla_creature(&mut state, "Doomed Bear");
        let equipment = mint_on_field(
            &mut state,
            Card::Normal(CardFace {
                name: "Sticky Sword".into(),
                types: vec![Type::Artifact.def()],
                subtypes: vec![subtype("Equipment")],
                abilities: vec![keyword("Equip([Tap])")],
                ..CardFace::default()
            }),
        );
        state.objects.obj_mut(equipment).attached_to = Some(host);

        // Host dies.
        let frame = frame_src_targets(equipment, vec![host]);
        state.run_effect(OneShotEffect::Act(Action::Destroy(Reference::It)), &frame);
        drain(&mut state);
        for e in crate::sba::sweep(&state) {
            state.schedule_front(vec![WorkItem::Emit(Occurrence::single(e))]);
            drain(&mut state);
        }
        assert_eq!(
            state.objects.obj(equipment).attached_to,
            None,
            "the Equipment became unattached when its host died ([CR#704.5n])"
        );
        assert!(
            state.zones.battlefield.contains(&equipment),
            "the Equipment STAYS on the battlefield (not graveyarded)"
        );
    }

    /// [CR#702.16d]: a creature that gains protection from a color drops a
    /// colored Equipment attached to it — the SBA re-runs `attachment_legal`
    /// (host-side protection `Cant(Attach)`) and unattaches.
    #[test]
    fn protection_drops_equipment() {
        use deckmaste_core::Color;
        use deckmaste_core::Deontic;
        use deckmaste_core::DeonticAction;

        let mut state = game();
        // The host gains protection from red: a host-side `Cant(Attach(what:
        // red, to: This))` (the Protection-conferred shape, [CR#702.16d]).
        let host = mint_on_field(
            &mut state,
            Card::Normal(CardFace {
                name: "Protected Bear".into(),
                types: vec![Type::Creature.def()],
                power: Some(deckmaste_core::StatValue::Number(2)),
                toughness: Some(deckmaste_core::StatValue::Number(2)),
                abilities: vec![Ability::Static(StaticEffect::Deontic(Deontic::Cant(
                    DeonticAction::Attach {
                        what: Predicate::Characteristic(CharacteristicPredicate::ColorIs(
                            Color::Red,
                        )),
                        to: Predicate::Ref(Reference::This),
                    },
                )))],
                ..CardFace::default()
            }),
        );
        // A RED Equipment attached to the host.
        let equipment = mint_on_field(
            &mut state,
            Card::Normal(CardFace {
                name: "Red Sword".into(),
                types: vec![Type::Artifact.def()],
                color_indicator: vec![Color::Red],
                subtypes: vec![subtype("Equipment")],
                abilities: vec![keyword("Equip([Tap])")],
                ..CardFace::default()
            }),
        );
        state.objects.obj_mut(equipment).attached_to = Some(host);
        // Sanity: it is currently illegal (protection) — the SBA will catch it.
        assert!(!crate::legal::attachment_legal(&state, equipment, host));

        for e in crate::sba::sweep(&state) {
            state.schedule_front(vec![WorkItem::Emit(Occurrence::single(e))]);
            drain(&mut state);
        }
        assert_eq!(
            state.objects.obj(equipment).attached_to,
            None,
            "the colored Equipment fell off the protected creature ([CR#702.16d])"
        );
    }

    /// [CR#702.67a]: a Fortification with `fortify` activated, targeting a land
    /// you control → attached to that land.
    #[test]
    fn fortify_attaches_to_land() {
        let mut state = game();
        let land = mint_on_field(
            &mut state,
            Card::Normal(CardFace {
                name: "Target Land".into(),
                types: vec![Type::Land.def()],
                ..CardFace::default()
            }),
        );
        let fortification = mint_on_field(
            &mut state,
            Card::Normal(CardFace {
                name: "Test Banner".into(),
                types: vec![Type::Artifact.def()],
                subtypes: vec![subtype("Fortification")],
                abilities: vec![keyword("Fortify([Tap])")],
                ..CardFace::default()
            }),
        );
        let frame = frame_src_targets(fortification, vec![land]);
        state.run_effect(
            OneShotEffect::Act(Action::Attach {
                what: Reference::This,
                to: Reference::It,
            }),
            &frame,
        );
        drain(&mut state);
        assert_eq!(
            state.objects.obj(fortification).attached_to,
            Some(land),
            "fortify attached the Fortification to the land ([CR#702.67a])"
        );
    }

    /// [CR#702.151b]: a reconfigured Equipment attached to a creature stops
    /// being a creature; unattaching restores it. SEAM: the
    /// creature-suppression static needs condition-gated layer-4 type
    /// removal the engine doesn't have yet (see Reconfigure.ron) — so the
    /// suppression assertion is `#[ignore]`d; the attach/unattach mechanics
    /// are exercised here unignored.
    #[test]
    fn reconfigure_attaches_and_unattaches() {
        let mut state = game();
        let host = vanilla_creature(&mut state, "Recon Host");
        // A reconfigure Equipment creature (it IS a creature when unattached).
        let equip_creature = mint_on_field(
            &mut state,
            Card::Normal(CardFace {
                name: "Living Weapon".into(),
                types: vec![Type::Artifact.def(), Type::Creature.def()],
                subtypes: vec![subtype("Equipment")],
                power: Some(deckmaste_core::StatValue::Number(1)),
                toughness: Some(deckmaste_core::StatValue::Number(1)),
                abilities: vec![keyword("Reconfigure([Tap])")],
                ..CardFace::default()
            }),
        );
        // Attach via reconfigure's first ability shape (Attach to a creature).
        let frame = frame_src_targets(equip_creature, vec![host]);
        state.run_effect(
            OneShotEffect::Act(Action::Attach {
                what: Reference::This,
                to: Reference::It,
            }),
            &frame,
        );
        run_injected(&mut state);
        assert_eq!(
            state.objects.obj(equip_creature).attached_to,
            Some(host),
            "reconfigure attached the Equipment to the creature ([CR#702.151a])"
        );

        // Unattach (reconfigure's second ability).
        let frame = frame_src(equip_creature);
        state.run_effect(
            OneShotEffect::Act(Action::Unattach(Reference::This)),
            &frame,
        );
        run_injected(&mut state);
        assert_eq!(
            state.objects.obj(equip_creature).attached_to,
            None,
            "reconfigure unattached the Equipment ([CR#702.151a])"
        );
    }

    /// [CR#702.151b]: SEAM — the creature-suppression static (a reconfigured
    /// Equipment isn't a creature while attached) needs condition-gated layer-4
    /// type removal the layer pipeline doesn't have yet (Reconfigure.ron seam).
    /// Ignored until that engine support lands.
    #[test]
    #[ignore = "engine-attach seam: conditional layer-4 type removal not built ([CR#702.151b]) — see Reconfigure.ron"]
    fn reconfigure_suppresses_creature() {
        let mut state = game();
        let host = vanilla_creature(&mut state, "Recon Host");
        let equip_creature = mint_on_field(
            &mut state,
            Card::Normal(CardFace {
                name: "Living Weapon".into(),
                types: vec![Type::Artifact.def(), Type::Creature.def()],
                subtypes: vec![subtype("Equipment")],
                power: Some(deckmaste_core::StatValue::Number(1)),
                toughness: Some(deckmaste_core::StatValue::Number(1)),
                abilities: vec![keyword("Reconfigure([Tap])")],
                ..CardFace::default()
            }),
        );
        state.objects.obj_mut(equip_creature).attached_to = Some(host);
        // Would-be: attached → not a creature.
        let view = state.layers();
        assert!(
            !view.get(equip_creature).has_type(Type::Creature),
            "attached reconfigure Equipment is not a creature ([CR#702.151b])"
        );
    }

    /// [CR#702.131c]: the grant verb emits one `GotDesignation` for a player
    /// who lacks the designation, and nothing for one who already holds it
    /// (idempotent — keeps the SBA sweep convergent and avoids spurious facts).
    #[test]
    fn get_designation_emits_once_then_nothing() {
        use crate::state::DesignationValue;

        let mut state = game();
        let p0 = PlayerId(0);
        let frame = frame_for(&state, p0);
        let pa = deckmaste_core::PlayerAction::GetDesignation("CitysBlessing".into());

        let items = state.player_action_items(&pa, p0, &frame);
        assert_eq!(items.len(), 1, "first grant emits exactly one fact");

        // Grant it for real, then re-run: no event.
        state
            .designations
            .players
            .insert((p0, "CitysBlessing".into()), DesignationValue::Flag);
        let items = state.player_action_items(&pa, p0, &frame);
        assert!(items.is_empty(), "already-held designation emits nothing");
    }

    /// [CR#608.2c]: `OneShotEffect::If` evaluates its condition WHEN it resolves and
    /// runs the taken branch — `then` on true, `otherwise` on false, and
    /// nothing when false with no `otherwise`. Driven via `GainLife` (a
    /// choice-free, library-free player action) so the assertion is a clean
    /// life delta.
    #[test]
    fn run_effect_if_takes_the_right_branch() {
        use deckmaste_core::Cmp;
        use deckmaste_core::Condition;
        use deckmaste_core::If;

        // Trivially-true and trivially-false comparisons over literals.
        let yes = Condition::Compare(Count::Literal(1), Cmp::AtLeast, Count::Literal(0));
        let no = Condition::Compare(Count::Literal(0), Cmp::AtLeast, Count::Literal(1));
        let gain = |n| {
            OneShotEffect::Act(Action::By(
                Reference::You,
                PlayerAction::GainLife(Count::Literal(n)),
            ))
        };

        let p0 = PlayerId(0);

        // true → then (gain 3), otherwise NOT taken.
        let mut state = game();
        let frame = frame_for(&state, p0);
        let life0 = state.player(p0).life;
        state.run_effect(
            OneShotEffect::If(If {
                condition: yes.clone(),
                then: Box::new(gain(3)),
                otherwise: Some(Box::new(gain(5))),
            }),
            &frame,
        );
        let _ = drain_progress(&mut state, 40);
        assert_eq!(state.player(p0).life, life0 + 3, "true → then branch");

        // false → otherwise (gain 5), then NOT taken.
        let mut state = game();
        let frame = frame_for(&state, p0);
        let life0 = state.player(p0).life;
        state.run_effect(
            OneShotEffect::If(If {
                condition: no.clone(),
                then: Box::new(gain(3)),
                otherwise: Some(Box::new(gain(5))),
            }),
            &frame,
        );
        let _ = drain_progress(&mut state, 40);
        assert_eq!(state.player(p0).life, life0 + 5, "false → otherwise branch");

        // false + no otherwise → nothing runs (life unchanged).
        let mut state = game();
        let frame = frame_for(&state, p0);
        let life0 = state.player(p0).life;
        state.run_effect(
            OneShotEffect::If(If {
                condition: no.clone(),
                then: Box::new(gain(3)),
                otherwise: None,
            }),
            &frame,
        );
        let _ = drain_progress(&mut state, 40);
        assert_eq!(
            state.player(p0).life,
            life0,
            "false + no otherwise → no change"
        );
    }

    /// [CR#608.2]: `OneShotEffect::Each` evaluates its binder once at resolution and
    /// runs the inner effect once per matched object, binding each iterated
    /// object as the anaphor `It` (a per-iteration `frame.anaphora.it`).
    /// Proven via `Destroy(It)` over the battlefield creatures: every
    /// creature dies, which can only happen if each iteration's `It`
    /// resolves to that iteration's object.
    #[test]
    fn run_effect_foreach_binds_each_match_as_it() {
        use deckmaste_core::Each;

        let (mut state, bear) = bear_on_field();
        let theirs = second_bear_to_player_1(&mut state);
        let creatures = Predicate::And(vec![
            Predicate::State(StatePredicate::InZone(Zone::Battlefield)),
            Predicate::creature(),
        ]);
        let frame = frame_src(bear);
        state.run_effect(
            OneShotEffect::Each(Each {
                binder: Binder::Existing(Selection::SelectAll(creatures)),
                effect: Box::new(OneShotEffect::Act(Action::Destroy(Reference::It))),
            }),
            &frame,
        );
        let _ = drain_progress(&mut state, 80);
        assert!(
            !state.zones.battlefield.contains(&bear) && !state.zones.battlefield.contains(&theirs),
            "every iterated creature is destroyed via its It binding"
        );
    }

    /// [CR#608.2]: `OneShotEffect::Each` runs the inner effect once per match — a
    /// non-binding body (gain 1 life) over two creatures gains 2 life.
    #[test]
    fn run_effect_foreach_runs_once_per_match() {
        use deckmaste_core::Each;

        let (mut state, bear) = bear_on_field();
        let _theirs = second_bear_to_player_1(&mut state);
        let creatures = Predicate::And(vec![
            Predicate::State(StatePredicate::InZone(Zone::Battlefield)),
            Predicate::creature(),
        ]);
        let frame = frame_src(bear);
        let life0 = state.player(PlayerId(0)).life;
        state.run_effect(
            OneShotEffect::Each(Each {
                binder: Binder::Existing(Selection::SelectAll(creatures)),
                effect: Box::new(OneShotEffect::Act(Action::By(
                    Reference::You,
                    PlayerAction::GainLife(Count::Literal(1)),
                ))),
            }),
            &frame,
        );
        let _ = drain_progress(&mut state, 80);
        assert_eq!(
            state.player(PlayerId(0)).life,
            life0 + 2,
            "two creatures → inner effect runs twice"
        );
    }

    /// [CR#118.12]: `OneShotEffect::May` surfaces a yes/no to the controller. Yes runs
    /// `effect` then `if_did`; no runs `if_not` (nothing when absent). Driven
    /// via `GainLife` so each branch reads as a clean life delta.
    #[test]
    fn run_effect_may_branches_on_the_answer() {
        use deckmaste_core::May;

        use crate::decide::Decision;
        use crate::decide::PendingDecision;

        let gain = |n| {
            OneShotEffect::Act(Action::By(
                Reference::You,
                PlayerAction::GainLife(Count::Literal(n)),
            ))
        };
        let may = || May {
            effect: Box::new(gain(3)),
            if_did: Some(Box::new(gain(10))),
            if_not: Some(Box::new(gain(1))),
        };
        let p0 = PlayerId(0);

        // yes → effect (3) + if_did (10) = +13; surfaces YesNo to the controller.
        let mut state = game();
        let frame = frame_for(&state, p0);
        let life0 = state.player(p0).life;
        state.run_effect(OneShotEffect::May(may()), &frame);
        let StepOutcome::NeedsDecision(PendingDecision::YesNo { player }) = state.step() else {
            panic!("expected YesNo, got {:?}", state.pending);
        };
        assert_eq!(player, p0, "the controller decides");
        state.submit_decision(Decision::Answer(true)).unwrap();
        let _ = drain_progress(&mut state, 40);
        assert_eq!(state.player(p0).life, life0 + 13, "yes → effect + if_did");

        // no → if_not (1) = +1.
        let mut state = game();
        let frame = frame_for(&state, p0);
        let life0 = state.player(p0).life;
        state.run_effect(OneShotEffect::May(may()), &frame);
        state.submit_decision(Decision::Answer(false)).unwrap();
        let _ = drain_progress(&mut state, 40);
        assert_eq!(state.player(p0).life, life0 + 1, "no → if_not");

        // no + no if_not → nothing.
        let mut state = game();
        let frame = frame_for(&state, p0);
        let life0 = state.player(p0).life;
        state.run_effect(
            OneShotEffect::May(May {
                effect: Box::new(gain(3)),
                if_did: None,
                if_not: None,
            }),
            &frame,
        );
        state.submit_decision(Decision::Answer(false)).unwrap();
        let _ = drain_progress(&mut state, 40);
        assert_eq!(state.player(p0).life, life0, "no + no if_not → no change");
    }

    /// [CR#700.2]: `OneShotEffect::Modal` surfaces `ChooseModes`; the chosen modes'
    /// effects run in written order. "Choose one" of three life-gain modes —
    /// picking index 1 gains 5; "choose two" runs both picks (+3+7); bad picks
    /// (too many, out of range) are rejected.
    #[test]
    fn run_effect_modal_runs_chosen_modes() {
        use deckmaste_core::ChooseSpec;
        use deckmaste_core::Modal;
        use deckmaste_core::Mode;

        use crate::decide::Decision;
        use crate::decide::PendingDecision;

        let gain_mode = |n| Mode {
            effect: OneShotEffect::Act(Action::By(
                Reference::You,
                PlayerAction::GainLife(Count::Literal(n)),
            )),
            cost: None,
        };
        let modes = || vec![gain_mode(3), gain_mode(5), gain_mode(7)];
        let spec = |count, up_to| ChooseSpec {
            count: deckmaste_core::Quantity::Range(
                Some(Count::Literal(count)),
                Some(Count::Literal(count)),
            ),
            up_to,
            repeats: false,
            chooser: Reference::You,
            rider: None,
        };
        let p0 = PlayerId(0);

        // choose one → ChooseModes(options 3, [1,1]); pick mode 1 → +5.
        let mut state = game();
        let frame = frame_for(&state, p0);
        let life0 = state.player(p0).life;
        state.run_effect(
            OneShotEffect::Modal(Modal {
                choose: spec(1, false),
                modes: modes(),
            }),
            &frame,
        );
        let StepOutcome::NeedsDecision(PendingDecision::ChooseModes {
            player,
            options,
            min,
            max,
            repeats,
        }) = state.step()
        else {
            panic!("expected ChooseModes, got {:?}", state.pending);
        };
        assert_eq!((player, options, min, max, repeats), (p0, 3, 1, 1, false));
        assert!(
            state.submit_decision(Decision::Modes(vec![0, 1])).is_err(),
            "too many modes"
        );
        assert!(
            state.submit_decision(Decision::Modes(vec![5])).is_err(),
            "mode index out of range"
        );
        state.submit_decision(Decision::Modes(vec![1])).unwrap();
        let _ = drain_progress(&mut state, 40);
        assert_eq!(
            state.player(p0).life,
            life0 + 5,
            "chosen mode's effect runs"
        );

        // choose two → both picks run (+3 +7 = +10).
        let mut state = game();
        let frame = frame_for(&state, p0);
        let life0 = state.player(p0).life;
        state.run_effect(
            OneShotEffect::Modal(Modal {
                choose: spec(2, false),
                modes: modes(),
            }),
            &frame,
        );
        state.submit_decision(Decision::Modes(vec![0, 2])).unwrap();
        let _ = drain_progress(&mut state, 40);
        assert_eq!(state.player(p0).life, life0 + 10, "both chosen modes run");
    }

    /// [CR#118.12a]: `OneShotEffect::MustPay` — the Mana Leak punisher over the full
    /// `Cost` (the English "unless" order is the `Unless` macro over this
    /// node). Pay → the cost runs and `or_else` is skipped; decline →
    /// `or_else` runs.
    #[test]
    fn run_effect_must_pay_pays_or_suffers_or_else() {
        use deckmaste_core::Cost;
        use deckmaste_core::CostComponent;
        use deckmaste_core::MustPay;

        use crate::decide::Decision;
        use crate::decide::PendingDecision;

        let p0 = PlayerId(0);
        let must_pay = || MustPay {
            actor: Reference::You,
            cost: Cost(vec![CostComponent::do_(PlayerAction::LoseLife(
                Count::Literal(2),
            ))]),
            or_else: Box::new(OneShotEffect::Act(Action::By(
                Reference::You,
                PlayerAction::GainLife(Count::Literal(10)),
            ))),
        };

        // "I'll pay" → lose 2, the punisher (gain 10) skipped.
        let mut state = game();
        let frame = frame_for(&state, p0);
        let life0 = state.player(p0).life;
        state.run_effect(OneShotEffect::MustPay(must_pay()), &frame);
        let StepOutcome::NeedsDecision(PendingDecision::YesNo { player }) = state.step() else {
            panic!("expected YesNo, got {:?}", state.pending);
        };
        assert_eq!(player, p0, "the payer decides");
        state.submit_decision(Decision::Answer(true)).unwrap();
        let _ = drain_progress(&mut state, 40);
        assert_eq!(
            state.player(p0).life,
            life0 - 2,
            "pay → cost paid, or_else skipped"
        );

        // "won't pay" → the punisher runs (gain 10).
        let mut state = game();
        let frame = frame_for(&state, p0);
        let life0 = state.player(p0).life;
        state.run_effect(OneShotEffect::MustPay(must_pay()), &frame);
        state.submit_decision(Decision::Answer(false)).unwrap();
        let _ = drain_progress(&mut state, 40);
        assert_eq!(state.player(p0).life, life0 + 10, "decline → or_else runs");
    }

    /// [CR#603,608]: `OneShotEffect::MayPay` — a resolution-time kicker. Pay → the cost
    /// runs THEN `and_then`; decline → `or_else`. The PAID branch running a
    /// follow-up effect is what `Unless`/`MustPay` cannot express.
    #[test]
    fn run_effect_may_pay_runs_and_then_on_pay_or_else_on_decline() {
        use deckmaste_core::Cost;
        use deckmaste_core::CostComponent;
        use deckmaste_core::MayPay;

        use crate::decide::Decision;
        use crate::decide::PendingDecision;

        let p0 = PlayerId(0);
        let may_pay = || MayPay {
            actor: Reference::You,
            cost: Cost(vec![CostComponent::do_(PlayerAction::LoseLife(
                Count::Literal(2),
            ))]),
            and_then: Box::new(OneShotEffect::Act(Action::By(
                Reference::You,
                PlayerAction::GainLife(Count::Literal(10)),
            ))),
            or_else: Some(Box::new(OneShotEffect::Act(Action::By(
                Reference::You,
                PlayerAction::GainLife(Count::Literal(1)),
            )))),
        };

        // "I'll pay" → lose 2 THEN gain 10 (net +8) — the kicker fires.
        let mut state = game();
        let frame = frame_for(&state, p0);
        let life0 = state.player(p0).life;
        state.run_effect(OneShotEffect::MayPay(may_pay()), &frame);
        let StepOutcome::NeedsDecision(PendingDecision::YesNo { player }) = state.step() else {
            panic!("expected YesNo, got {:?}", state.pending);
        };
        assert_eq!(player, p0, "the payer decides");
        state.submit_decision(Decision::Answer(true)).unwrap();
        let _ = drain_progress(&mut state, 40);
        assert_eq!(
            state.player(p0).life,
            life0 + 8,
            "pay → cost (−2) then and_then (+10)"
        );

        // "won't pay" → or_else runs (gain 1).
        let mut state = game();
        let frame = frame_for(&state, p0);
        let life0 = state.player(p0).life;
        state.run_effect(OneShotEffect::MayPay(may_pay()), &frame);
        state.submit_decision(Decision::Answer(false)).unwrap();
        let _ = drain_progress(&mut state, 40);
        assert_eq!(state.player(p0).life, life0 + 1, "decline → or_else runs");
    }

    /// [CR#601.2f,118.8]: `OneShotEffect::AdditionalCost` (nested, resolution-time) —
    /// the cost is PAID (the source is sacrificed) and the body then reads the
    /// paid object through the event reference `EventObject`. The bear is
    /// sacrificed carrying three +1/+1 counters; the body gains life equal to
    /// the SACRIFICED creature's counter count, read via its last-known
    /// snapshot ([CR#603.10a]) — proving the paid object is bound for the
    /// body even after it has left the battlefield.
    #[test]
    fn run_effect_additional_cost_binds_paid_object_for_body() {
        use deckmaste_core::AdditionalCost;
        use deckmaste_core::Cost;
        use deckmaste_core::CostComponent;

        let (mut state, bear) = bear_on_field();
        state
            .objects
            .obj_mut(bear)
            .counters
            .insert("P1P1Counter".into(), 3);
        let p0 = PlayerId(0);
        let life0 = state.player(p0).life;
        let frame = frame_src(bear);

        state.run_effect(
            OneShotEffect::AdditionalCost(AdditionalCost {
                pay: Cost(vec![CostComponent::do_(PlayerAction::Sacrifice(
                    Reference::This,
                ))]),
                body: Box::new(OneShotEffect::act_by_you(PlayerAction::GainLife(
                    Count::CounterCount(Box::new(Reference::EventObject), "P1P1Counter".into()),
                ))),
            }),
            &frame,
        );
        let _ = drain_progress(&mut state, 40);

        assert!(
            state.objects.get(bear).is_none(),
            "the additional cost was paid: the source is sacrificed (old id gone)"
        );
        assert_eq!(
            state.zones.graveyards[0].len(),
            1,
            "the sacrificed creature is in its owner's graveyard"
        );
        assert_eq!(
            state.player(p0).life,
            life0 + 3,
            "the body read the sacrificed object's counters via EventObject"
        );
    }

    // --- Ascend (spell form) e2e ([CR#702.131a]) -------------------------------
    //
    // The spell form of Ascend folds into `Sequentially([If(<gate>,
    // GetDesignation), If(Is(You,Designated), Draw(3), otherwise: Draw(2))])`
    // (Task 7). The `OneShotEffect::If` interpreter is now live (see the
    // `OneShotEffect::If` arm in `run_effect` — it evaluates `condition_holds`,
    // then schedules `then`/`otherwise`), so these run unignored. They prove
    // the grant-then-read ordering ([CR#608.2c]): draws 3 at ten, 2 at nine,
    // and 2 at ten-then-nine (no high-water mark). The fixture is isolated by
    // `diag_setup_is_sound`, which proves the gate reads 10/9 correctly and a
    // bare `Draw(3)` lands three.

    /// The folded Ascend gate ([CR#702.131a]) built typed — the exact shape
    /// `deckmaste_migrations::resolve::fold_spell_ascend` prepends to a spell's
    /// effect: "ten battlefield permanents you control AND you don't already
    /// have the city's blessing".
    fn ascend_gate() -> deckmaste_core::Condition {
        use deckmaste_core::Cmp;
        use deckmaste_core::Condition;
        use deckmaste_core::RelationPredicate;

        Condition::And(vec![
            Condition::Compare(
                Count::CountOf(Countable::Objects(Box::new(Predicate::And(vec![
                    Predicate::State(StatePredicate::InZone(Zone::Battlefield)),
                    Predicate::Relation(RelationPredicate::ControlledBy(Box::new(Predicate::Ref(
                        Reference::You,
                    )))),
                ])))),
                Cmp::AtLeast,
                Count::Literal(10),
            ),
            Condition::Not(Box::new(Condition::Matches(
                Reference::You,
                Predicate::State(StatePredicate::Designated("CitysBlessing".into())),
            ))),
        ])
    }

    /// Secrets of the Golden City's resolved shape — the folded grant followed
    /// by the blessing-conditioned draw ("Ascend. Draw two cards. If you have
    /// the city's blessing, draw three instead."):
    ///
    /// ```text
    /// Sequentially([
    ///   If(gate, then: GetDesignation("CitysBlessing")),          // folded Ascend
    ///   If(Is(You, Designated), then: Draw(3), otherwise: Draw(2)),
    /// ])
    /// ```
    fn secrets_effect() -> OneShotEffect {
        use deckmaste_core::Condition;
        use deckmaste_core::If;

        OneShotEffect::Sequentially(vec![
            OneShotEffect::If(If {
                condition: ascend_gate(),
                then: Box::new(OneShotEffect::act_by_you(PlayerAction::GetDesignation(
                    "CitysBlessing".into(),
                ))),
                otherwise: None,
            }),
            OneShotEffect::If(If {
                condition: Condition::Matches(
                    Reference::You,
                    Predicate::State(StatePredicate::Designated("CitysBlessing".into())),
                ),
                then: Box::new(OneShotEffect::act_by_you(PlayerAction::Draw(
                    Count::Literal(3),
                ))),
                otherwise: Some(Box::new(OneShotEffect::act_by_you(PlayerAction::Draw(
                    Count::Literal(2),
                )))),
            }),
        ])
    }

    /// Builds a game where p0 controls `permanents` battlefield objects, has a
    /// fat library to draw from, an EMPTY hand (so the draw delta is the
    /// post-resolution hand size), and the synthetic Secrets-of-the-Golden-City
    /// spell on the stack (its first/only ability the `secrets` effect).
    /// Returns `(state, p0, library_before)`.
    fn secrets_on_stack(permanents: usize) -> (GameState, PlayerId, usize) {
        use deckmaste_core::Ability;
        use deckmaste_core::CardFace;
        use deckmaste_core::SpellAbility;

        let mut state = game();
        let p0 = PlayerId(0);

        // A stocked library and an empty hand, so the post-resolution hand size
        // IS the number of cards drawn. Mint plain library objects under p0;
        // their identity is irrelevant — a draw just remints the top.
        let dummy = Card::Normal(CardFace {
            name: "Library Filler".into(),
            ..CardFace::default()
        });
        let dummy_card = state.cards.push(Arc::new(dummy), p0);
        for _ in 0..10 {
            let id = state
                .objects
                .mint(ObjectSource::Card(dummy_card), p0, Some(Zone::Library));
            state.zones.libraries[p0.index()].push_back(id);
        }
        let library_before = state.zones.libraries[p0.index()].len();
        assert!(
            state.zones.hands[p0.index()].is_empty(),
            "empty starting hand"
        );

        // p0's battlefield: `permanents` plain artifacts. Card-backed (mirrors
        // a real board), all controlled by p0 — the gate counts these.
        for i in 0..permanents {
            let perm = Card::Normal(CardFace {
                name: format!("Permanent {i}"),
                types: vec![Type::Artifact.def()],
                ..CardFace::default()
            });
            let card_id = state.cards.push(Arc::new(perm), p0);
            let id = state
                .objects
                .mint(ObjectSource::Card(card_id), p0, Some(Zone::Battlefield));
            state.zones.battlefield.push(id);
        }
        assert_eq!(state.zones.battlefield.len(), permanents);

        // The synthetic Secrets-of-the-Golden-City spell on the stack.
        let spell_card = Card::Normal(CardFace {
            name: "Secrets of the Golden City".into(),
            types: vec![Type::Sorcery.def()],
            abilities: vec![Ability::Spell(SpellAbility {
                ability_word: None,
                effect: secrets_effect(),
            })],
            ..CardFace::default()
        });
        let spell_card_id = state.cards.push(Arc::new(spell_card), p0);
        let spell = state
            .objects
            .mint(ObjectSource::Card(spell_card_id), p0, Some(Zone::Stack));
        state.stack.push(StackEntry {
            paid_costs: Vec::new(),
            id: spell,
            object: StackObject::Spell(spell),
            controller: p0,
            targets: vec![],
            x: None,
            copy: false,
        });

        (state, p0, library_before)
    }

    /// Steps the agenda to a stop (decision / game-over) or until `n` steps
    /// elapse, returning the `Progress` trace — the in-crate analogue of
    /// `skeleton::drain_progress`.
    fn drain_progress(state: &mut GameState, n: usize) -> Vec<Progress> {
        let mut out = Vec::new();
        for _ in 0..n {
            match state.step() {
                StepOutcome::Progress(p) => out.push(p),
                StepOutcome::NeedsDecision(_) | StepOutcome::GameOver(_) => break,
            }
        }
        out
    }

    /// Pump up to `n` steps, collecting every `(target, amount)` of the
    /// `DamageDealt` events applied along the way — the per-element emissions a
    /// `Each(.., DealDamage(This, .., It))` produces (a verb deals to a
    /// single `Reference`, so the spread is the iterator).
    fn collect_damage_dealt(state: &mut GameState, n: usize) -> Vec<(ObjectId, u32)> {
        let mut got = Vec::new();
        for p in drain_progress(state, n) {
            if let Progress::Applied(occ) = p {
                let events = match occ {
                    Occurrence::Single(e) => vec![e],
                    Occurrence::Batch(es) => es,
                };
                for e in events {
                    if let GameEvent::DamageDealt { target, amount, .. } = e {
                        got.push((target, amount));
                    }
                }
            }
        }
        got
    }

    /// Isolation guard: proves the spell-form fixture is sound independent of
    /// the `OneShotEffect::If` interpreter — the gate reads true at ten / false
    /// at nine for these minted battlefield objects, and a bare `Draw(3)`
    /// from the stocked library lands three cards in hand. So any failure
    /// of the three behavioral cases below points at the interpreter, not
    /// the fixture.
    #[test]
    fn diag_setup_is_sound() {
        // Gate at ten: true.
        let (state, p0, _lib) = secrets_on_stack(10);
        let frame = frame_for(&state, p0);
        assert!(
            state.condition_holds(&ascend_gate(), &frame),
            "gate true at ten permanents"
        );

        // Gate at nine: false.
        let (state9, p0, _lib) = secrets_on_stack(9);
        let frame9 = frame_for(&state9, p0);
        assert!(
            !state9.condition_holds(&ascend_gate(), &frame9),
            "gate false at nine permanents"
        );

        // A bare Draw(3) lands three cards in hand from the stocked library.
        let (mut sd, p0, lib_before) = secrets_on_stack(10);
        let dframe = frame_for(&sd, p0);
        sd.run_effect(
            OneShotEffect::Act(Action::By(
                Reference::You,
                PlayerAction::Draw(Count::Literal(3)),
            )),
            &dframe,
        );
        let _ = drain_progress(&mut sd, 40);
        assert_eq!(
            sd.zones.hands[p0.index()].len(),
            3,
            "bare Draw(3) drew three"
        );
        assert_eq!(sd.zones.libraries[p0.index()].len(), lib_before - 3);
    }

    /// [CR#702.131a,608.2c]: on a SPELL, the folded Ascend grant ([CR#702.131a])
    /// fires DURING resolution, and because the controller follows the spell's
    /// instructions in written order ([CR#608.2c]), the DOWNSTREAM "if you have
    /// the city's blessing" read sees that fresh grant — at ten permanents the
    /// player gets the blessing AND draws three (not two). This is the crux:
    /// the grant must be applied before the later read. No high-water mark
    /// — only the count at resolution matters (see the sibling cases).
    #[test]
    fn ascend_spell_grants_then_reads_at_ten() {
        let (mut state, p0, lib_before) = secrets_on_stack(10);
        let name: deckmaste_core::Ident = "CitysBlessing".into();

        state
            .agenda
            .push_front(WorkItem::Resolve(state.stack[0].id));
        let _trace = drain_progress(&mut state, 40);

        assert!(
            state.designations.players.contains_key(&(p0, name)),
            "the folded Ascend grant fired during resolution ([CR#702.131a])"
        );
        let drawn = state.zones.hands[p0.index()].len();
        assert_eq!(
            drawn, 3,
            "the downstream read saw the fresh blessing → drew three ([CR#608.2c]); drew {drawn}"
        );
        assert_eq!(
            state.zones.libraries[p0.index()].len(),
            lib_before - 3,
            "three cards left the library"
        );
    }

    /// At NINE permanents the gate is false: no grant, the downstream read is
    /// false, the player draws two and never holds the blessing.
    #[test]
    fn ascend_spell_no_blessing_below_ten() {
        let (mut state, p0, lib_before) = secrets_on_stack(9);
        let name: deckmaste_core::Ident = "CitysBlessing".into();

        state
            .agenda
            .push_front(WorkItem::Resolve(state.stack[0].id));
        let _trace = drain_progress(&mut state, 40);

        assert!(
            !state.designations.players.contains_key(&(p0, name)),
            "no blessing below ten permanents"
        );
        let drawn = state.zones.hands[p0.index()].len();
        assert_eq!(drawn, 2, "no blessing → drew two; drew {drawn}");
        assert_eq!(
            state.zones.libraries[p0.index()].len(),
            lib_before - 2,
            "two cards left the library"
        );
    }

    /// [CR#702.131a]: NO high-water mark. Reach ten permanents, then drop one
    /// back to nine BEFORE the spell resolves: the gate reads nine at
    /// resolution, so no blessing and a two-card draw. A momentary ten does not
    /// count.
    #[test]
    fn ascend_spell_no_high_water_mark() {
        let (mut state, p0, lib_before) = secrets_on_stack(10);
        let name: deckmaste_core::Ident = "CitysBlessing".into();

        // Drop one permanent (10 → 9) before resolution.
        let dropped = state.zones.battlefield.pop().expect("a permanent to drop");
        state.objects.obj_mut(dropped).zone = None;
        assert_eq!(
            state.zones.battlefield.len(),
            9,
            "back to nine at resolution"
        );

        state
            .agenda
            .push_front(WorkItem::Resolve(state.stack[0].id));
        let _trace = drain_progress(&mut state, 40);

        assert!(
            !state.designations.players.contains_key(&(p0, name)),
            "a momentary ten doesn't grant — only the resolution count matters ([CR#702.131a])"
        );
        let drawn = state.zones.hands[p0.index()].len();
        assert_eq!(drawn, 2, "nine at resolution → drew two; drew {drawn}");
        assert_eq!(
            state.zones.libraries[p0.index()].len(),
            lib_before - 2,
            "two cards left the library"
        );
    }

    /// `Count::EventCount` is the count-valued twin of `Condition::Happened`:
    /// it scans the history log within the given window and returns how many
    /// facts match the `Event` pattern via `event_matches` ([CR#608.2i]).
    /// Two creature-death facts recorded this turn → count == 2; a non-matching
    /// pattern (zone-enter) or a turn with no facts → count == 0.
    #[test]
    fn event_count_counts_matching_history() {
        use deckmaste_core::EventFilter;

        use crate::lki::LkiSnapshot;

        let bears = Arc::new(canon().card("Grizzly Bears").unwrap());
        let forest = Arc::new(builtin().card("Forest").unwrap());
        let mut state = GameState::new(GameConfig {
            players: vec![
                PlayerConfig {
                    deck: vec![Arc::clone(&bears); 10],
                },
                PlayerConfig {
                    deck: vec![Arc::clone(&forest); 10],
                },
            ],
            seed: 1,
            starting_life: 20,
            starting_player: StartingPlayer::Fixed(PlayerId(0)),
            sba_rules: vec![],
            conferral_rules: vec![],
            damage_result_rules: vec![],
            counter_decls: std::collections::HashMap::new(),
            subtypes: std::collections::HashMap::new(),
            types: std::collections::HashMap::new(),
        });
        state.turn.turn_number = 1;

        // Build two creature-death GameEvents (same shape as the morbid test).
        let first_bear_card = state.cards.push(Arc::clone(&bears), PlayerId(0));
        let first_bear = state.objects.mint(
            ObjectSource::Card(first_bear_card),
            PlayerId(0),
            Some(Zone::Battlefield),
        );
        state.zones.battlefield.push(first_bear);
        let death1 = GameEvent::ZoneChanged {
            snapshot: LkiSnapshot::capture(&state, first_bear),
            from: Some(Zone::Battlefield),
            to: Zone::Graveyard,
            face: None,
            cause: None,
        };

        let second_bear_card = state.cards.push(Arc::clone(&bears), PlayerId(0));
        let second_bear = state.objects.mint(
            ObjectSource::Card(second_bear_card),
            PlayerId(0),
            Some(Zone::Battlefield),
        );
        state.zones.battlefield.push(second_bear);
        let death2 = GameEvent::ZoneChanged {
            snapshot: LkiSnapshot::capture(&state, second_bear),
            from: Some(Zone::Battlefield),
            to: Zone::Graveyard,
            face: None,
            cause: None,
        };

        // The creature-death event pattern (same as morbid Condition::Happened).
        let death_pattern = EventFilter::ZoneChange {
            what: Predicate::creature(),
            from: Some(Zone::Battlefield),
            to: Some(Zone::Graveyard),
            cause: None,
        };

        // A non-matching pattern: creatures entering the battlefield.
        let enter_pattern = EventFilter::ZoneChange {
            what: Predicate::creature(),
            from: None,
            to: Some(Zone::Battlefield),
            cause: None,
        };

        let frame = frame_for(&state, PlayerId(0));

        // No deaths recorded yet → 0.
        assert_eq!(
            state.eval_count(
                &Count::EventCount(Box::new(death_pattern.clone()), Lookback::ThisTurn),
                &frame
            ),
            0,
            "no deaths recorded yet"
        );

        // Record two deaths this turn.
        state.record_history_fact(1, None, death1);
        state.record_history_fact(1, None, death2);

        // Both deaths match → 2.
        assert_eq!(
            state.eval_count(
                &Count::EventCount(Box::new(death_pattern.clone()), Lookback::ThisTurn),
                &frame
            ),
            2,
            "two creature deaths this turn"
        );

        // A non-matching pattern → 0.
        assert_eq!(
            state.eval_count(
                &Count::EventCount(Box::new(enter_pattern), Lookback::ThisTurn),
                &frame
            ),
            0,
            "enter pattern does not match death facts"
        );

        // Advance to turn 2: ThisTurn sees 0, ThisGame sees 2.
        state.turn.turn_number = 2;
        assert_eq!(
            state.eval_count(
                &Count::EventCount(Box::new(death_pattern.clone()), Lookback::ThisTurn),
                &frame
            ),
            0,
            "ThisTurn no longer sees last turn's deaths"
        );
        assert_eq!(
            state.eval_count(
                &Count::EventCount(Box::new(death_pattern), Lookback::ThisGame),
                &frame
            ),
            2,
            "ThisGame still sees last turn's deaths"
        );
    }

    /// `EventSum` sums the `amount` field of matching `LifeLost` facts within
    /// the window ([CR#608.2i,119.3]). Two losses of 2 and 3 by the same player
    /// total 5; a third loss by an opponent does not contribute. After a turn
    /// advance `ThisTurn` reads 0 while `ThisGame` still reads 5.
    #[test]
    fn event_sum_totals_amounts() {
        use deckmaste_core::EventFilter;

        let mut state = GameState::new(GameConfig {
            players: vec![
                PlayerConfig { deck: Vec::new() },
                PlayerConfig { deck: Vec::new() },
            ],
            seed: 1,
            starting_life: 20,
            starting_player: StartingPlayer::Fixed(PlayerId(0)),
            sba_rules: vec![],
            conferral_rules: vec![],
            damage_result_rules: vec![],
            counter_decls: std::collections::HashMap::new(),
            subtypes: std::collections::HashMap::new(),
            types: std::collections::HashMap::new(),
        });
        state.turn.turn_number = 1;

        let frame = frame_for(&state, PlayerId(0));

        let lose_life_pattern = EventFilter::LifeLost {
            who: deckmaste_core::Predicate::Ref(deckmaste_core::Reference::You),
            amount: None,
        };

        // No facts yet → 0.
        assert_eq!(
            state.eval_count(
                &Count::EventSum(Box::new(lose_life_pattern.clone()), Lookback::ThisTurn),
                &frame
            ),
            0,
            "no life-loss facts yet"
        );

        // Record two life-loss facts for player 0 (you) and one for player 1
        // (opponent).
        state.record_history_fact(
            1,
            None,
            GameEvent::LifeLost {
                player: PlayerId(0),
                amount: 2,
            },
        );
        state.record_history_fact(
            1,
            None,
            GameEvent::LifeLost {
                player: PlayerId(0),
                amount: 3,
            },
        );
        state.record_history_fact(
            1,
            None,
            GameEvent::LifeLost {
                player: PlayerId(1),
                amount: 10,
            },
        );

        // Only player 0's losses sum → 5.
        assert_eq!(
            state.eval_count(
                &Count::EventSum(Box::new(lose_life_pattern.clone()), Lookback::ThisTurn),
                &frame
            ),
            5,
            "two life-loss facts for you: 2 + 3 = 5"
        );

        // Advance to turn 2: ThisTurn sees 0, ThisGame sees 5.
        state.turn.turn_number = 2;
        assert_eq!(
            state.eval_count(
                &Count::EventSum(Box::new(lose_life_pattern.clone()), Lookback::ThisTurn),
                &frame
            ),
            0,
            "ThisTurn no longer sees last turn's life losses"
        );
        assert_eq!(
            state.eval_count(
                &Count::EventSum(Box::new(lose_life_pattern), Lookback::ThisGame),
                &frame
            ),
            5,
            "ThisGame still sees 5 total life lost by you"
        );
    }

    /// `EventCount(Used(by: This))` is OBJECT-scoped: it resolves `by` to the
    /// frame's source `ObjectId` and counts that object's `AbilityUsed` facts,
    /// NOT a watcher-pattern match ([CR#608.2i,603.2]). Two uses by the frame
    /// object this turn → 2 (a third use by a DIFFERENT object is excluded);
    /// after a turn advance `ThisTurn` reads 0 while `ThisGame` still reads 2.
    #[test]
    fn event_count_used_counts_object_ability_uses() {
        use deckmaste_core::EventFilter;

        let mut state = GameState::new(GameConfig {
            players: vec![
                PlayerConfig { deck: Vec::new() },
                PlayerConfig { deck: Vec::new() },
            ],
            seed: 1,
            starting_life: 20,
            starting_player: StartingPlayer::Fixed(PlayerId(0)),
            sba_rules: vec![],
            conferral_rules: vec![],
            damage_result_rules: vec![],
            counter_decls: std::collections::HashMap::new(),
            subtypes: std::collections::HashMap::new(),
            types: std::collections::HashMap::new(),
        });
        state.turn.turn_number = 1;

        // The frame's source is `obj`; `Used(by: This)` resolves `This` to it.
        let obj = ObjectId::from_raw(1);
        let other = ObjectId::from_raw(2);
        let frame = frame_src(obj);

        let used = |n| {
            Count::EventCount(
                Box::new(EventFilter::Used {
                    of: Reference::This,
                }),
                n,
            )
        };

        // No uses recorded yet → 0.
        assert_eq!(
            state.eval_count(&used(Lookback::ThisTurn), &frame),
            0,
            "no ability uses recorded yet"
        );

        // Two uses by `obj` this turn, and one by `other`.
        state.record_history_fact(
            1,
            None,
            GameEvent::AbilityUsed {
                object: obj,
                ability: 0,
            },
        );
        state.record_history_fact(
            1,
            None,
            GameEvent::AbilityUsed {
                object: obj,
                ability: 0,
            },
        );
        state.record_history_fact(
            1,
            None,
            GameEvent::AbilityUsed {
                object: other,
                ability: 0,
            },
        );

        // Only `obj`'s two uses count (`This` == frame.source == obj).
        assert_eq!(
            state.eval_count(&used(Lookback::ThisTurn), &frame),
            2,
            "two uses by the frame object; the other object's use is excluded"
        );

        // Advance to turn 2: ThisTurn sees 0, ThisGame still sees the 2.
        state.turn.turn_number = 2;
        assert_eq!(
            state.eval_count(&used(Lookback::ThisTurn), &frame),
            0,
            "ThisTurn no longer sees last turn's uses"
        );
        assert_eq!(
            state.eval_count(&used(Lookback::ThisGame), &frame),
            2,
            "ThisGame still sees last turn's two uses"
        );
    }

    /// The card-facing payoff: a self-use count drives a branching condition
    /// (`If(Compare(EventCount(Used(by: This), ThisTurn), Eq, 2), then,
    /// else)`). The `Compare` is FALSE after one recorded use of the frame
    /// object and TRUE after the second — the ability's own use-count keys
    /// the branch ([CR#608.2i]).
    #[test]
    fn event_count_self_drives_branching_condition() {
        use deckmaste_core::Cmp;
        use deckmaste_core::Condition;
        use deckmaste_core::EventFilter;

        let mut state = GameState::new(GameConfig {
            players: vec![
                PlayerConfig { deck: Vec::new() },
                PlayerConfig { deck: Vec::new() },
            ],
            seed: 1,
            starting_life: 20,
            starting_player: StartingPlayer::Fixed(PlayerId(0)),
            sba_rules: vec![],
            conferral_rules: vec![],
            damage_result_rules: vec![],
            counter_decls: std::collections::HashMap::new(),
            subtypes: std::collections::HashMap::new(),
            types: std::collections::HashMap::new(),
        });
        state.turn.turn_number = 1;

        let obj = ObjectId::from_raw(1);
        let frame = frame_src(obj);

        // "if this object's abilities have been used exactly twice this turn".
        let twice = Condition::Compare(
            Count::EventCount(
                Box::new(EventFilter::Used {
                    of: Reference::This,
                }),
                Lookback::ThisTurn,
            ),
            Cmp::Eq,
            Count::Literal(2),
        );

        // One use → not yet two → branch is FALSE.
        state.record_history_fact(
            1,
            None,
            GameEvent::AbilityUsed {
                object: obj,
                ability: 0,
            },
        );
        assert!(
            !state.condition_holds(&twice, &frame),
            "one self-use does not satisfy `== 2`"
        );

        // Second use → exactly two → branch is TRUE.
        state.record_history_fact(
            1,
            None,
            GameEvent::AbilityUsed {
                object: obj,
                ability: 0,
            },
        );
        assert!(
            state.condition_holds(&twice, &frame),
            "two self-uses satisfy `== 2`"
        );
    }

    /// `TopOfLibrary` returns the top N cards in order (front of library =
    /// top); `OneShotEffect::With` binds them so `Selection::That` resolves to
    /// the same ordered vec inside the body frame.
    #[test]
    fn with_binds_those_and_top_of_library_is_ordered() {
        use deckmaste_core::CardFace;
        use deckmaste_core::With;

        let mut state = game();
        let p0 = PlayerId(0);

        // Build three distinct library cards and mint them in order a→b→c
        // (a at front = top).
        let make_card = |name: &str| {
            Card::Normal(CardFace {
                name: name.into(),
                ..CardFace::default()
            })
        };
        let card_a = state.cards.push(Arc::new(make_card("Alpha")), p0);
        let card_b = state.cards.push(Arc::new(make_card("Beta")), p0);
        let card_c = state.cards.push(Arc::new(make_card("Gamma")), p0);

        let a = state
            .objects
            .mint(ObjectSource::Card(card_a), p0, Some(Zone::Library));
        let b = state
            .objects
            .mint(ObjectSource::Card(card_b), p0, Some(Zone::Library));
        let c = state
            .objects
            .mint(ObjectSource::Card(card_c), p0, Some(Zone::Library));

        // Push in top→bottom order: a at front (index 0) = top of library.
        state.zones.libraries[p0.index()].push_back(a);
        state.zones.libraries[p0.index()].push_back(b);
        state.zones.libraries[p0.index()].push_back(c);

        // A source object for the frame — use p0's proxy.
        let source = state.player(p0).object;
        let frame = Frame::bare(source, p0);

        // TopOfLibrary(count:2, whose:You) → top two in order.
        let top2 = state.eval_selection_set(
            &Selection::TopOfLibrary {
                count: Count::Literal(2),
                whose: deckmaste_core::Reference::You,
            },
            &frame,
        );
        assert_eq!(top2, vec![a, b], "top 2 are a then b, top→down");

        // With binds them as the many-binder `That`; the body frame sees the same
        // ordered group.
        let mut bound = frame.clone();
        bound.anaphora.that = Some(ThatBinding {
            cardinality: Cardinality::Many,
            kind: RefKind::Object,
            group: top2.clone(),
        });
        assert_eq!(
            state.eval_selection_set(&Selection::They, &bound),
            vec![a, b],
            "Selection::They inside a With frame returns the bound group in order"
        );

        // OneShotEffect::With end-to-end: run_effect schedules a body that reads
        // Selection::That and verifies the binding survives round-trip through
        // the agenda.
        // We check indirectly by scheduling a no-op body and confirming no panic.
        state.run_effect(
            OneShotEffect::With(With {
                binder: deckmaste_core::Binder::Existing(Selection::TopOfLibrary {
                    count: Count::Literal(2),
                    whose: deckmaste_core::Reference::You,
                }),
                body: Box::new(OneShotEffect::Sequentially(vec![])),
            }),
            &frame,
        );
        // Drain the agenda — the empty Sequentially body completes without a
        // decision, proving With schedules correctly.
        for _ in 0..10 {
            state.step();
        }
    }

    /// `BottomOfLibrary` mirrors `TopOfLibrary` from the other end (bottom→up
    /// order), and `Union` concatenates member groups order-preserved with an
    /// object in more than one member appearing once (the Idris `Union` /
    /// `BottomOfLibrary` constructors).
    #[test]
    fn bottom_of_library_and_union_resolve_as_groups() {
        use deckmaste_core::CardFace;

        let mut state = game();
        let p0 = PlayerId(0);
        let make_card = |name: &str| {
            Card::Normal(CardFace {
                name: name.into(),
                ..CardFace::default()
            })
        };
        let card_a = state.cards.push(Arc::new(make_card("Alpha")), p0);
        let card_b = state.cards.push(Arc::new(make_card("Beta")), p0);
        let card_c = state.cards.push(Arc::new(make_card("Gamma")), p0);
        let a = state
            .objects
            .mint(ObjectSource::Card(card_a), p0, Some(Zone::Library));
        let b = state
            .objects
            .mint(ObjectSource::Card(card_b), p0, Some(Zone::Library));
        let c = state
            .objects
            .mint(ObjectSource::Card(card_c), p0, Some(Zone::Library));
        state.zones.libraries[p0.index()].push_back(a);
        state.zones.libraries[p0.index()].push_back(b);
        state.zones.libraries[p0.index()].push_back(c);
        let source = state.player(p0).object;
        let frame = Frame::bare(source, p0);

        // BottomOfLibrary(count:2) → the bottom two, nearest-to-bottom first.
        let bottom2 = state.eval_selection_set(
            &Selection::BottomOfLibrary {
                count: Count::Literal(2),
                whose: deckmaste_core::Reference::You,
            },
            &frame,
        );
        assert_eq!(bottom2, vec![c, b], "bottom 2 are c then b, bottom→up");

        // Union of the top-2 and bottom-2 windows: b appears in both members
        // and is kept once, at its first position.
        let union = state.eval_selection_set(
            &Selection::Union(vec![
                Selection::TopOfLibrary {
                    count: Count::Literal(2),
                    whose: deckmaste_core::Reference::You,
                },
                Selection::BottomOfLibrary {
                    count: Count::Literal(2),
                    whose: deckmaste_core::Reference::You,
                },
            ]),
            &frame,
        );
        assert_eq!(union, vec![a, b, c], "order-preserving union, b deduped");
    }

    /// `TopOfGraveyard` reads the top `count` cards of a graveyard, top→down
    /// ([CR#404.2]): `zones.graveyards` is push-appended, so the LAST-pushed
    /// card (the most recent addition) is physically on top. A non-player
    /// `of` fizzles to the empty group — never a panic, unlike
    /// `TopOfLibrary`/`BottomOfLibrary`'s established (and here deliberately
    /// NOT repeated) panic baseline.
    #[test]
    fn top_of_graveyard_resolves_top_down_and_fizzles_on_bad_of() {
        use deckmaste_core::CardFace;

        let mut state = game();
        let p0 = PlayerId(0);
        let make_card = |name: &str| {
            Card::Normal(CardFace {
                name: name.into(),
                ..CardFace::default()
            })
        };
        let card_a = state.cards.push(Arc::new(make_card("Alpha")), p0);
        let card_b = state.cards.push(Arc::new(make_card("Beta")), p0);
        let card_c = state.cards.push(Arc::new(make_card("Gamma")), p0);
        let a = state
            .objects
            .mint(ObjectSource::Card(card_a), p0, Some(Zone::Graveyard));
        let b = state
            .objects
            .mint(ObjectSource::Card(card_b), p0, Some(Zone::Graveyard));
        let c = state
            .objects
            .mint(ObjectSource::Card(card_c), p0, Some(Zone::Graveyard));
        // Put in bottom→top order: a first (bottom), c last (top).
        state.zones.graveyards[p0.index()].push(a);
        state.zones.graveyards[p0.index()].push(b);
        state.zones.graveyards[p0.index()].push(c);
        let source = state.player(p0).object;
        let frame = Frame::bare(source, p0);

        let top1 = state.eval_selection_set(
            &Selection::TopOfGraveyard {
                count: Count::Literal(1),
                of: deckmaste_core::Reference::You,
            },
            &frame,
        );
        assert_eq!(top1, vec![c], "the top card is the most recently put one");

        let top2 = state.eval_selection_set(
            &Selection::TopOfGraveyard {
                count: Count::Literal(2),
                of: deckmaste_core::Reference::You,
            },
            &frame,
        );
        assert_eq!(top2, vec![c, b], "top 2, top→down");

        // `of` resolving to a non-player (a card, not a player proxy) fizzles
        // to the empty group rather than panicking.
        let bad = state.eval_selection_set(
            &Selection::TopOfGraveyard {
                count: Count::Literal(1),
                of: deckmaste_core::Reference::This,
            },
            &Frame::bare(a, p0),
        );
        assert_eq!(
            bad,
            Vec::<ObjectId>::new(),
            "non-player `of` fizzles, never panics"
        );
    }

    // ---- scry / arrange (the recomposed keyword-action path) ----------------
    use deckmaste_core::Anchor;
    use deckmaste_core::Destination;
    use deckmaste_core::Uint;

    use crate::decide::Decision;
    use crate::decide::PendingDecision;

    /// Mint a fresh card-backed object into `owner`'s library at the BOTTOM
    /// (`push_back`; the front is the top). Returns its id.
    fn mint_in_library(state: &mut GameState, owner: PlayerId, name: &str) -> ObjectId {
        let cid = state.cards.push(
            Arc::new(Card::Normal(CardFace {
                name: name.into(),
                types: vec![Type::Creature.def()],
                ..CardFace::default()
            })),
            owner,
        );
        let id = state
            .objects
            .mint(ObjectSource::Card(cid), owner, Some(Zone::Library));
        state.zones.libraries[owner.index()].push_back(id);
        id
    }

    /// The recomposed `scry n` effect ([CR#701.22a]): the committed north-star
    /// shape — `Composite Scry (Each (Existing (TopOfLibrary n)) (Modal 1-of-2
    /// [Move(It, Library(FromTop 0)), Move(It, Library(FromBottom 0))]))`.
    fn scry_effect(n: Uint) -> OneShotEffect {
        let mode = |anchor| deckmaste_core::Mode {
            effect: OneShotEffect::Act(Action::Move(
                Reference::It,
                Destination::Library(anchor),
                vec![],
            )),
            cost: None,
        };
        OneShotEffect::Act(Action::Composite {
            name: "Scry".into(),
            body: Box::new(OneShotEffect::Each(deckmaste_core::Each {
                binder: deckmaste_core::Binder::Existing(Selection::TopOfLibrary {
                    count: Count::Literal(n),
                    whose: Reference::You,
                }),
                effect: Box::new(OneShotEffect::Modal(deckmaste_core::Modal {
                    choose: deckmaste_core::ChooseSpec {
                        count: deckmaste_core::Quantity::Range(
                            Some(Count::Literal(1)),
                            Some(Count::Literal(1)),
                        ),
                        up_to: false,
                        repeats: false,
                        chooser: Reference::You,
                        rider: None,
                    },
                    modes: vec![
                        mode(Anchor::FromTop(Count::Literal(0))),
                        mode(Anchor::FromBottom(Count::Literal(0))),
                    ],
                })),
            })),
        })
    }

    /// Step until a decision surfaces (or `n` steps elapse), returning the
    /// applied events seen along the way.
    fn drain_events(state: &mut GameState, n: usize) -> Vec<GameEvent> {
        let mut out = Vec::new();
        for p in drain_progress(state, n) {
            if let Progress::Applied(occ) = p {
                match occ {
                    Occurrence::Single(e) => out.push(e),
                    Occurrence::Batch(v) => out.extend(v),
                }
            }
        }
        out
    }

    /// [CR#701.22a,401.7]: scry-1 to the BOTTOM repositions the peeked card
    /// within the SAME library — the `ObjectId` is preserved, no `ZoneChanged`
    /// fires (a pile of one surfaces no arrange decision), and the
    /// keyword-action event fires once the pick lands ([CR#701.22d]).
    #[test]
    fn scry_reposition_keeps_id_and_fires_no_zone_change() {
        let p0 = PlayerId(0);
        let mut state = game();
        let a = mint_in_library(&mut state, p0, "A");
        let b = mint_in_library(&mut state, p0, "B");
        let c = mint_in_library(&mut state, p0, "C");
        // library top→bottom = [a, b, c].
        let frame = frame_for(&state, p0);
        state.run_effect(scry_effect(1), &frame);
        drain_events(&mut state, 60); // → the single Modal decision
        assert!(
            matches!(state.pending, Some(PendingDecision::ChooseModes { .. })),
            "scry surfaces the per-card top/bottom pick, got {:?}",
            state.pending
        );
        // The looker sees the peeked card.
        assert!(state.look_grants.contains(&(p0, a)), "peek grants look");
        // Pick mode 1 (bottom).
        state.submit_decision(Decision::Modes(vec![1])).unwrap();
        let events = drain_events(&mut state, 60);
        // `a` moved to the bottom, SAME id, no zone change.
        assert_eq!(
            state.zones.libraries[p0.index()]
                .iter()
                .copied()
                .collect::<Vec<_>>(),
            vec![b, c, a],
            "a repositioned to the bottom, keeping its id"
        );
        assert!(
            state.objects.get(a).is_some(),
            "the repositioned object id is preserved (not reminted)"
        );
        assert!(
            !events
                .iter()
                .any(|e| matches!(e, GameEvent::ZoneChanged { .. })),
            "a same-library reposition fires no ZoneChanged"
        );
        assert!(
            events.iter().any(|e| matches!(
                e,
                GameEvent::KeywordActionPerformed { name, .. } if name.as_str() == "Scry"
            )),
            "scry-1 fires the keyword-action event"
        );
    }

    /// [CR#701.22b]: scry 0 does nothing and fires NO keyword event; scry N>0
    /// fires exactly one `KeywordActionPerformed`.
    #[test]
    fn scry_zero_fires_no_event_but_nonzero_does() {
        let p0 = PlayerId(0);

        // scry 0 over a stocked library: no decision, no event.
        let mut state = game();
        mint_in_library(&mut state, p0, "A");
        mint_in_library(&mut state, p0, "B");
        let frame = frame_for(&state, p0);
        state.run_effect(scry_effect(0), &frame);
        let events = drain_events(&mut state, 60);
        assert!(
            !events
                .iter()
                .any(|e| matches!(e, GameEvent::KeywordActionPerformed { .. })),
            "scry 0 emits no keyword event ([CR#701.22b])"
        );

        // scry 1 fires exactly one.
        let mut state = game();
        mint_in_library(&mut state, p0, "A");
        mint_in_library(&mut state, p0, "B");
        let frame = frame_for(&state, p0);
        state.run_effect(scry_effect(1), &frame);
        drain_events(&mut state, 60);
        state.submit_decision(Decision::Modes(vec![0])).unwrap();
        let events = drain_events(&mut state, 60);
        let scries = events
            .iter()
            .filter(|e| {
                matches!(
                    e,
                    GameEvent::KeywordActionPerformed { name, .. } if name.as_str() == "Scry"
                )
            })
            .count();
        assert_eq!(scries, 1, "scry 1 fires exactly one keyword event");
    }

    /// [CR#401.4]: a pile of MORE THAN ONE card at a library end surfaces one
    /// arrange decision (both-on-top), while a scry whose picks split one card
    /// to each end surfaces NONE (every pile is a single card).
    #[test]
    fn scry_arrange_surfaces_only_for_multi_card_piles() {
        let p0 = PlayerId(0);

        // Both on top → one arrange decision over the two-card pile.
        let mut state = game();
        let a = mint_in_library(&mut state, p0, "A");
        let b = mint_in_library(&mut state, p0, "B");
        mint_in_library(&mut state, p0, "C");
        let frame = frame_for(&state, p0);
        state.run_effect(scry_effect(2), &frame);
        drain_events(&mut state, 60);
        state.submit_decision(Decision::Modes(vec![0])).unwrap(); // a → top
        drain_events(&mut state, 60);
        state.submit_decision(Decision::Modes(vec![0])).unwrap(); // b → top
        drain_events(&mut state, 60);
        let Some(PendingDecision::ArrangePile { player, objects }) = state.pending.clone() else {
            panic!("expected an ArrangePile decision, got {:?}", state.pending);
        };
        assert_eq!(player, p0, "the scrying player arranges");
        assert_eq!(
            objects
                .iter()
                .copied()
                .collect::<std::collections::HashSet<_>>(),
            [a, b].into_iter().collect(),
            "the top pile holds both peeked cards"
        );

        // One top, one bottom → two singleton piles → no arrange decision.
        let mut state = game();
        mint_in_library(&mut state, p0, "A");
        mint_in_library(&mut state, p0, "B");
        mint_in_library(&mut state, p0, "C");
        let frame = frame_for(&state, p0);
        state.run_effect(scry_effect(2), &frame);
        drain_events(&mut state, 60);
        state.submit_decision(Decision::Modes(vec![0])).unwrap(); // a → top
        drain_events(&mut state, 60);
        state.submit_decision(Decision::Modes(vec![1])).unwrap(); // b → bottom
        let events = drain_events(&mut state, 60);
        assert!(
            !matches!(state.pending, Some(PendingDecision::ArrangePile { .. })),
            "two singleton piles surface no arrange decision, got {:?}",
            state.pending
        );
        assert!(
            events.iter().any(|e| matches!(
                e,
                GameEvent::KeywordActionPerformed { name, .. } if name.as_str() == "Scry"
            )),
            "the keyword event still fires"
        );
    }

    /// [CR#701.22a]: a full scry-2 both-on-top round trip — the arrange decision
    /// orders the top pile, and the library ends up in the chosen order above
    /// the untouched rest, every id preserved.
    #[test]
    fn scry_two_both_top_round_trip() {
        let p0 = PlayerId(0);
        let mut state = game();
        let a = mint_in_library(&mut state, p0, "A");
        let b = mint_in_library(&mut state, p0, "B");
        let c = mint_in_library(&mut state, p0, "C");
        let d = mint_in_library(&mut state, p0, "D");
        // top→bottom = [a, b, c, d].
        let frame = frame_for(&state, p0);
        state.run_effect(scry_effect(2), &frame);
        drain_events(&mut state, 60);
        state.submit_decision(Decision::Modes(vec![0])).unwrap(); // a → top
        drain_events(&mut state, 60);
        state.submit_decision(Decision::Modes(vec![0])).unwrap(); // b → top
        drain_events(&mut state, 60);
        // Arrange the top pile as b, then a (top → down).
        state
            .submit_decision(Decision::Arranged(vec![b, a]))
            .unwrap();
        drain_events(&mut state, 60);
        assert_eq!(
            state.zones.libraries[p0.index()]
                .iter()
                .copied()
                .collect::<Vec<_>>(),
            vec![b, a, c, d],
            "the chosen order sits on top, the rest untouched, ids preserved"
        );
    }

    /// Mint a fresh card-backed object into `owner`'s hand. Returns its id.
    fn mint_in_hand(state: &mut GameState, owner: PlayerId, name: &str) -> ObjectId {
        let cid = state.cards.push(
            Arc::new(Card::Normal(CardFace {
                name: name.into(),
                types: vec![Type::Creature.def()],
                ..CardFace::default()
            })),
            owner,
        );
        let id = state
            .objects
            .mint(ObjectSource::Card(cid), owner, Some(Zone::Hand));
        state.zones.hands[owner.index()].push(id);
        id
    }

    /// [CR#401.4]: Brainstorm's group put-back — `MoveGroup(AnyOrder)` from hand
    /// onto the top of the library moves the whole group (reminted, a real zone
    /// change) and surfaces ONE arrange decision over the landed pile, which
    /// then sits on top of the untouched rest of the library. The same
    /// ordered-landing surface as scry, generalized beyond it.
    #[test]
    fn move_group_any_order_surfaces_arrange_over_landed_pile() {
        let p0 = PlayerId(0);
        let mut state = game();
        let lib = mint_in_library(&mut state, p0, "Lib"); // one card on top
        mint_in_hand(&mut state, p0, "H1");
        mint_in_hand(&mut state, p0, "H2");
        let frame = frame_for(&state, p0);
        let effect = OneShotEffect::Act(Action::MoveGroup {
            group: Selection::SelectAll(Predicate::State(deckmaste_core::StatePredicate::InZone(
                Zone::Hand,
            ))),
            arrangement: deckmaste_core::Arrangement::AnyOrder,
            to: Destination::Library(Anchor::FromTop(Count::Literal(0))),
            riders: vec![],
        });
        state.run_effect(effect, &frame);
        drain_events(&mut state, 60);
        let Some(PendingDecision::ArrangePile { player, objects }) = state.pending.clone() else {
            panic!("expected an ArrangePile decision, got {:?}", state.pending);
        };
        assert_eq!(player, p0, "the owner arranges an 'any order' group");
        assert_eq!(objects.len(), 2, "both moved cards form the top pile");
        assert!(
            state.zones.hands[p0.index()].is_empty(),
            "the hand cards left the hand"
        );
        assert_eq!(
            state.zones.libraries[p0.index()].len(),
            3,
            "library grew by the two moved cards"
        );
        // Arrange the pile, then it sits on top of the pre-existing card.
        state
            .submit_decision(Decision::Arranged(objects.clone()))
            .unwrap();
        drain_events(&mut state, 60);
        let top: Vec<_> = state.zones.libraries[p0.index()]
            .iter()
            .copied()
            .take(2)
            .collect();
        assert_eq!(top, objects, "the arranged pile sits on top");
        assert_eq!(
            state.zones.libraries[p0.index()].iter().copied().nth(2),
            Some(lib),
            "the pre-existing card is untouched beneath the pile"
        );
    }

    /// `Count` arithmetic ([CR#107.1]) evaluates structurally — no board
    /// needed. `Minus` floors at 0 ([CR#107.1b]); `Half` rounds per the mode.
    #[test]
    fn count_arithmetic_evaluates() {
        let state = game();
        let frame = frame_for(&state, PlayerId(0));
        let lit = |n| Box::new(Count::Literal(n));
        let ev = |c: &Count| state.eval_count(c, &frame);
        assert_eq!(ev(&Count::Plus(lit(2), lit(3))), 5);
        assert_eq!(ev(&Count::Minus(lit(2), lit(5))), 0, "a count floors at 0");
        assert_eq!(ev(&Count::Times(lit(2), lit(3))), 6);
        assert_eq!(ev(&Count::Max(lit(2), lit(3))), 3);
        assert_eq!(
            ev(&Count::Half(deckmaste_core::RoundMode::RoundUp, lit(3))),
            2,
        );
        assert_eq!(
            ev(&Count::Half(deckmaste_core::RoundMode::RoundDown, lit(3))),
            1,
        );
    }

    /// Build a one-player game whose deck holds `names` (padded with Bears so
    /// the opening draw never empties the library), force each named card onto
    /// P0's battlefield, and return their ids.
    fn battlefield_with(names: &[&str]) -> (GameState, Vec<ObjectId>) {
        let mut deck: Vec<Arc<Card>> = names
            .iter()
            .map(|n| Arc::new(canon().card(n).unwrap()))
            .collect();
        let bears = Arc::new(canon().card("Grizzly Bears").unwrap());
        while deck.len() < 12 {
            deck.push(Arc::clone(&bears));
        }
        let mut state = GameState::new(GameConfig {
            players: vec![PlayerConfig { deck }, PlayerConfig { deck: vec![] }],
            seed: 7,
            starting_life: 20,
            starting_player: StartingPlayer::Fixed(PlayerId(0)),
            sba_rules: vec![],
            conferral_rules: vec![],
            damage_result_rules: vec![],
            counter_decls: std::collections::HashMap::new(),
            subtypes: std::collections::HashMap::new(),
            types: std::collections::HashMap::new(),
        });
        let mut ids = Vec::new();
        for name in names {
            let p = PlayerId(0).index();
            let obj = state.zones.hands[p]
                .iter()
                .copied()
                .find(|&o| {
                    state.objects.obj(o).card_id().is_some()
                        && matches!(state.def(o), Card::Normal(f) | Card::TwoFaced { front: f, .. } if f.name == *name)
                })
                .or_else(|| {
                    state.zones.libraries[p].iter().copied().find(|&o| {
                        state.objects.obj(o).card_id().is_some()
                            && matches!(state.def(o), Card::Normal(f) | Card::TwoFaced { front: f, .. } if f.name == *name)
                    })
                })
                .unwrap_or_else(|| panic!("no {name} in P0's hand or library"));
            state.zones.hands[p].retain(|&o| o != obj);
            state.zones.libraries[p].retain(|&o| o != obj);
            state.objects.obj_mut(obj).zone = Some(Zone::Battlefield);
            state.zones.battlefield.push(obj);
            ids.push(obj);
        }
        (state, ids)
    }

    /// A battlefield-scoped creature filter (canonical card filters carry
    /// their own zone narrowing) — keeps the deck's hand/library bears out of
    /// the matched set.
    fn creatures_in_play() -> Predicate {
        Predicate::And(vec![
            Predicate::State(StatePredicate::InZone(Zone::Battlefield)),
            Predicate::creature(),
        ])
    }

    /// `CountDistinct` ([CR#107.3]) is the distinct-union size of an axis over
    /// the matched set: three creatures of powers 2/2/3 give two distinct
    /// powers (Coven), and three distinct toughnesses 2/4/3.
    #[test]
    fn count_distinct_over_creatures() {
        let (state, _) = battlefield_with(&["Grizzly Bears", "Giant Spider", "Centaur Courser"]);
        let frame = frame_for(&state, PlayerId(0));
        let powers = Count::CountDistinct(
            deckmaste_core::Characteristic::Power,
            Countable::Objects(Box::new(creatures_in_play())),
        );
        assert_eq!(
            state.eval_count(&powers, &frame),
            2,
            "distinct powers {{2,3}}"
        );
        let toughnesses = Count::CountDistinct(
            deckmaste_core::Characteristic::Toughness,
            Countable::Objects(Box::new(creatures_in_play())),
        );
        assert_eq!(
            state.eval_count(&toughnesses, &frame),
            3,
            "distinct toughnesses {{2,4,3}}",
        );
    }

    /// `Selection::Pick` ([CR#107.1]) takes the extremal element: the creature
    /// with the greatest power is the 3/3 Centaur Courser; the least-power pick
    /// is the whole tied 2-power group.
    #[test]
    fn pick_extremal_creature_by_power() {
        let (state, ids) = battlefield_with(&["Grizzly Bears", "Giant Spider", "Centaur Courser"]);
        let courser = ids[2];
        let frame = frame_for(&state, PlayerId(0));
        let greatest = Selection::Pick {
            op: deckmaste_core::AggregateOp::MaxOf,
            proj: deckmaste_core::Projection {
                of: deckmaste_core::Countable::Objects(Box::new(creatures_in_play())),
                by: Box::new(Count::StatOf(Reference::It, deckmaste_core::Stat::Power)),
            },
        };
        assert_eq!(
            state.eval_selection_set(&greatest, &frame),
            vec![courser],
            "Centaur Courser (3 power) is the unique greatest",
        );
        let least = Selection::Pick {
            op: deckmaste_core::AggregateOp::MinOf,
            proj: deckmaste_core::Projection {
                of: deckmaste_core::Countable::Objects(Box::new(creatures_in_play())),
                by: Box::new(Count::StatOf(Reference::It, deckmaste_core::Stat::Power)),
            },
        };
        let picked = state.eval_selection_set(&least, &frame);
        assert_eq!(picked.len(), 2, "the two 2-power creatures tie for least");
        assert!(
            !picked.contains(&courser),
            "the 3-power creature is not least"
        );
    }

    /// [CR#104.2b]: `WinGame` resolves to a first-class `PlayerWon` event
    /// that ends the game with the actor as winner.
    #[test]
    fn win_game_verb_sets_win_outcome() {
        let (mut state, _bear) = bear_on_field();
        let frame = frame_for(&state, PlayerId(0));
        state.run_effect(OneShotEffect::act_by_you(PlayerAction::WinGame), &frame);
        let _ = state.step();
        assert_eq!(
            state.outcome,
            Some(crate::state::GameOutcome::Win(PlayerId(0)))
        );
    }

    /// Platinum Angel's "your opponents can't win the game" static
    /// ([CR#101.1]) suppresses the `WinGame` verb for the gated opponent —
    /// a graceful no-op, no event emitted.
    #[test]
    fn cant_win_gated_win_game_is_a_noop() {
        let (state, _ids) = battlefield_with(&["Platinum Angel"]);
        let frame = frame_for(&state, PlayerId(1));
        assert_eq!(
            state.action_items(&Action::by_you(PlayerAction::WinGame), &frame),
            vec![],
            "Platinum Angel's opponents-can't-win gate suppresses the WinGame verb"
        );
    }

    /// [CR#104.3e]: `LoseGame` resolves to `PlayerLost { reason: Effect }`;
    /// in a 2-player game the survivor wins last-standing ([CR#104.2a]).
    #[test]
    fn lose_game_verb_sets_loss_and_opponent_wins() {
        let (mut state, _bear) = bear_on_field();
        let frame = frame_for(&state, PlayerId(0));
        state.run_effect(OneShotEffect::act_by_you(PlayerAction::LoseGame), &frame);
        let _ = state.step();
        assert!(state.players[0].lost);
        assert_eq!(
            state.outcome,
            Some(crate::state::GameOutcome::Win(PlayerId(1)))
        );
    }

    /// Platinum Angel's "you can't lose the game" static suppresses the
    /// `LoseGame` verb for its controller — a graceful no-op.
    #[test]
    fn cant_lose_gated_lose_game_is_a_noop() {
        let (state, _ids) = battlefield_with(&["Platinum Angel"]);
        let frame = frame_for(&state, PlayerId(0));
        assert_eq!(
            state.action_items(&Action::by_you(PlayerAction::LoseGame), &frame),
            vec![],
            "Platinum Angel's you-can't-lose gate suppresses the LoseGame verb"
        );
    }

    /// [CR#104.3f]: a player who would simultaneously win and lose in the
    /// same batch loses instead — the arbitration in `apply_occurrence`
    /// drops the `PlayerWon` fact before the batch applies, so the OTHER
    /// player takes the last-standing win.
    #[test]
    fn win_and_lose_batch_arbitration_drops_the_win() {
        let (mut state, _bear) = bear_on_field();
        state.schedule_front(vec![WorkItem::Emit(Occurrence::Batch(vec![
            GameEvent::PlayerWon {
                player: PlayerId(0),
            },
            GameEvent::PlayerLost {
                player: PlayerId(0),
                reason: crate::event::LossReason::LifeZero,
            },
        ]))]);
        let _ = state.step();
        assert!(state.players[0].lost, "the loss still applies");
        assert_eq!(
            state.outcome,
            Some(crate::state::GameOutcome::Win(PlayerId(1))),
            "the dropped win lets player 1 take the last-standing win instead"
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
            confers: vec![],
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
            confers: vec![],
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

    /// [CR#705.1]: an uncalled `FlipCoins(3, false)` draws 3 coins straight
    /// from the seeded rng with NO decision and no winner/loser, as ONE
    /// simultaneous batch; the applied batch fixes "that many" to the number
    /// of heads. Seed-pinned (via `game()`'s seed 7): same seed ⇒ same draw,
    /// so the assertions are exact, not just shape checks.
    #[test]
    fn uncalled_flip_emits_batch_and_fixes_that_many_to_heads() {
        let mut state = game();
        let p0 = PlayerId(0);
        let frame = frame_for(&state, p0);
        state.run_effect(
            OneShotEffect::act_by_you(PlayerAction::FlipCoins(Count::Literal(3), false)),
            &frame,
        );
        drain_progress(&mut state, 20);

        let flips: Vec<(bool, Option<bool>)> = state
            .history
            .entries()
            .filter_map(|e| match &e.fact {
                GameEvent::CoinFlipped { heads, won, .. } => Some((*heads, *won)),
                _ => None,
            })
            .collect();
        assert_eq!(flips.len(), 3, "3 CoinFlipped facts, one per drawn coin");
        assert!(
            flips.iter().all(|&(_, won)| won.is_none()),
            "an uncalled flip never records a winner/loser"
        );
        let heads = Uint::try_from(flips.iter().filter(|&&(h, _)| h).count())
            .expect("heads count fits Uint");
        assert_eq!(
            state.that_much,
            Some(heads),
            "\"that many\" is fixed to the number of heads"
        );
    }

    /// [CR#706.1]: `RollDice(3, 6)` draws 3 naturals in `1..=6` from the
    /// seeded rng, each `result == natural` (no modifier pipeline yet), as
    /// ONE simultaneous batch; the applied batch fixes "that many" to the
    /// summed results.
    #[test]
    fn dice_roll_emits_per_die_and_fixes_that_many_to_sum() {
        let mut state = game();
        let p0 = PlayerId(0);
        let frame = frame_for(&state, p0);
        state.run_effect(
            OneShotEffect::act_by_you(PlayerAction::RollDice(Count::Literal(3), 6)),
            &frame,
        );
        drain_progress(&mut state, 20);

        let rolls: Vec<(Uint, Uint)> = state
            .history
            .entries()
            .filter_map(|e| match &e.fact {
                GameEvent::DieRolled {
                    natural, result, ..
                } => Some((*natural, *result)),
                _ => None,
            })
            .collect();
        assert_eq!(rolls.len(), 3, "3 DieRolled facts, one per drawn die");
        assert!(
            rolls
                .iter()
                .all(|&(natural, result)| (1..=6).contains(&natural) && natural == result),
            "every natural lands in 1..=6 and result == natural (no modifier pipeline yet)"
        );
        let sum: Uint = rolls.iter().map(|&(_, result)| result).sum();
        assert_eq!(
            state.that_much,
            Some(sum),
            "\"that many\" is fixed to the summed results"
        );
    }

    /// [CR#701.9b]: `Discard { random: true, .. }` samples straight from the
    /// seeded rng — no `DiscardCards` decision surfaces (no choice exists for
    /// a random discard), and the sampled cards move Hand→Graveyard.
    #[test]
    fn random_discard_samples_without_decision() {
        let mut state = game();
        let p0 = PlayerId(0);
        for i in 0..5 {
            mint_in_hand(&mut state, p0, &format!("Random Discard Card {i}"));
        }
        let frame = frame_for(&state, p0);
        state.run_effect(
            OneShotEffect::act_by_you(PlayerAction::Discard {
                count: Count::Literal(2),
                what: None,
                random: true,
            }),
            &frame,
        );
        // Drain until the discard resolves (graveyard gains 2) or SOME
        // decision surfaces first — a `DiscardCards` decision here would mean
        // the "random" path wrongly asked the player to choose. The normal
        // game's own `Priority` window (reached once resolution completes) is
        // not that decision, so it is not itself a failure.
        for _ in 0..20 {
            if state.zones.graveyards[p0.index()].len() == 2 {
                break;
            }
            if matches!(state.step(), StepOutcome::NeedsDecision(_)) {
                break;
            }
        }
        assert!(
            !matches!(state.pending, Some(PendingDecision::DiscardCards { .. })),
            "a random discard surfaces no DiscardCards decision (no choice exists): {:?}",
            state.pending
        );
        assert_eq!(
            state.zones.hands[p0.index()].len(),
            3,
            "2 of 5 cards left the hand"
        );
        assert_eq!(
            state.zones.graveyards[p0.index()].len(),
            2,
            "the sampled 2 cards landed in the graveyard"
        );
    }

    /// The seeded rng is deterministic: two identical states (same seed),
    /// driven through the identical `FlipCoins` + `RollDice` sequence, draw
    /// identical heads/naturals.
    #[test]
    fn randomness_is_seed_deterministic() {
        fn drive() -> (Vec<bool>, Vec<Uint>) {
            let mut state = game();
            let p0 = PlayerId(0);
            let frame = frame_for(&state, p0);
            state.run_effect(
                OneShotEffect::act_by_you(PlayerAction::FlipCoins(Count::Literal(3), false)),
                &frame,
            );
            drain_progress(&mut state, 20);
            state.run_effect(
                OneShotEffect::act_by_you(PlayerAction::RollDice(Count::Literal(3), 6)),
                &frame,
            );
            drain_progress(&mut state, 20);

            let heads: Vec<bool> = state
                .history
                .entries()
                .filter_map(|e| match &e.fact {
                    GameEvent::CoinFlipped { heads, .. } => Some(*heads),
                    _ => None,
                })
                .collect();
            let naturals: Vec<Uint> = state
                .history
                .entries()
                .filter_map(|e| match &e.fact {
                    GameEvent::DieRolled { natural, .. } => Some(*natural),
                    _ => None,
                })
                .collect();
            (heads, naturals)
        }

        assert_eq!(
            drive(),
            drive(),
            "same seed ⇒ identical flip/roll sequence across independent states"
        );
    }

    // ========================================================================
    // `engine-randomness` Task 4: the `CallFlip` decision — called flips
    // ([CR#705.2]) draw per submitted call, scoring `won`, and either
    // re-surface the next call (multi-coin) or front-schedule the
    // accumulated batch.
    // ========================================================================

    /// [CR#705.2]: a single CALLED flip surfaces one `CallFlip` decision (no
    /// draw yet); submitting the call draws the coin and scores `won = (call
    /// == heads)`. Seed-pinned (via `game()`'s seed 7): the draw is
    /// deterministic, so the assertion is exact, not just a shape check.
    #[test]
    fn called_flip_surfaces_call_and_scores_won() {
        use crate::decide::Decision;
        use crate::decide::PendingDecision;

        let mut state = game();
        let p0 = PlayerId(0);
        let frame = frame_for(&state, p0);
        state.run_effect(
            OneShotEffect::act_by_you(PlayerAction::FlipCoins(Count::Literal(1), true)),
            &frame,
        );
        drain_progress(&mut state, 20);
        let Some(PendingDecision::CallFlip { player }) = state.pending.clone() else {
            panic!("expected a pending CallFlip, got {:?}", state.pending);
        };
        assert_eq!(player, p0);

        state.submit_decision(Decision::Answer(true)).unwrap();
        drain_progress(&mut state, 20);

        let flips: Vec<(bool, Option<bool>)> = state
            .history
            .entries()
            .filter_map(|e| match &e.fact {
                GameEvent::CoinFlipped { heads, won, .. } => Some((*heads, *won)),
                _ => None,
            })
            .collect();
        assert_eq!(flips.len(), 1, "exactly one CoinFlipped fact");
        let (heads, won) = flips[0];
        assert_eq!(
            won,
            Some(heads),
            "the call was heads: won iff the draw landed heads"
        );
        assert_eq!(
            state.that_much,
            Some(Uint::from(won == Some(true))),
            "\"that many\" is fixed to the win count (0 or 1)"
        );
    }

    /// [CR#705.2]: a 3-coin CALLED flip pauses per coin — three sequential
    /// `CallFlip` decisions, each drawing (and scoring) only when its call is
    /// submitted — then front-schedules ONE simultaneous batch
    /// ([CR#603.3b]) once all three are called.
    #[test]
    fn multi_coin_called_flip_pauses_per_coin() {
        use crate::decide::Decision;
        use crate::decide::PendingDecision;

        let mut state = game();
        let p0 = PlayerId(0);
        let frame = frame_for(&state, p0);
        state.run_effect(
            OneShotEffect::act_by_you(PlayerAction::FlipCoins(Count::Literal(3), true)),
            &frame,
        );
        drain_progress(&mut state, 20);

        for (i, call) in [true, false, true].into_iter().enumerate() {
            let Some(PendingDecision::CallFlip { player }) = state.pending.clone() else {
                panic!(
                    "coin {i}: expected a pending CallFlip, got {:?}",
                    state.pending
                );
            };
            assert_eq!(player, p0, "coin {i}");
            state.submit_decision(Decision::Answer(call)).unwrap();
            drain_progress(&mut state, 20);
        }

        let flips: Vec<(bool, Option<bool>)> = state
            .history
            .entries()
            .filter_map(|e| match &e.fact {
                GameEvent::CoinFlipped { heads, won, .. } => Some((*heads, *won)),
                _ => None,
            })
            .collect();
        assert_eq!(flips.len(), 3, "3 CoinFlipped facts, one per called coin");
        assert!(
            flips.iter().all(|&(_, won)| won.is_some()),
            "every called flip records a winner/loser"
        );
        let won_count = Uint::try_from(flips.iter().filter(|&&(_, won)| won == Some(true)).count())
            .expect("win count fits Uint");
        assert_eq!(
            state.that_much,
            Some(won_count),
            "\"that many\" is fixed to the win count across the whole batch"
        );
    }

    /// A `CallFlip` decision only answers `Decision::Answer` — any other
    /// decision kind (here, a stray `Discard`) is rejected as `WrongKind`,
    /// leaving the `CallFlip` decision (and its stashed continuation) intact.
    #[test]
    fn call_flip_rejects_wrong_decision_kind() {
        use crate::decide::Decision;
        use crate::decide::DecisionError;
        use crate::decide::PendingDecision;

        let mut state = game();
        let p0 = PlayerId(0);
        let frame = frame_for(&state, p0);
        state.run_effect(
            OneShotEffect::act_by_you(PlayerAction::FlipCoins(Count::Literal(1), true)),
            &frame,
        );
        drain_progress(&mut state, 20);
        assert!(matches!(
            state.pending,
            Some(PendingDecision::CallFlip { .. })
        ));

        assert_eq!(
            state.submit_decision(Decision::Discard(vec![])),
            Err(DecisionError::WrongKind)
        );
        assert!(
            matches!(state.pending, Some(PendingDecision::CallFlip { .. })),
            "a rejected wrong-kind decision leaves the CallFlip pending"
        );
    }

    // ========================================================================
    // `engine-randomness` Task 6: trigger integration ([CR#705.2,706.1]) +
    // full-random-surface reproducibility. Unlike `trigger.rs`'s
    // `scan_event`/`scan_triggers`-direct tests, these drive the REAL
    // `FlipCoins`/`RollDice`/`CallFlip` pipeline end to end (`run_effect` +
    // `drain_progress` + `submit_decision`, this module's own harness) so the
    // rng draws and the `won` scoring are genuine, not hand-built facts.
    // ========================================================================

    /// A synthetic in-Rust creature whose sole ability is a "draw a card"
    /// trigger watching `event` — the fire-detection body Task 6's trigger
    /// tests register on the battlefield. Mirrors `trigger.rs`'s
    /// `draw_on`/`put_synthetic_on_field` pair.
    fn put_watcher(
        state: &mut GameState,
        controller: PlayerId,
        event: deckmaste_core::EventFilter,
    ) -> ObjectSource {
        use deckmaste_core::Ability;
        use deckmaste_core::CardFace;
        use deckmaste_core::TriggeredAbility;

        let card = Card::Normal(CardFace {
            name: "Randomness Watcher".into(),
            types: vec![Type::Creature.def()],
            abilities: vec![Ability::Triggered(TriggeredAbility {
                ability_word: None,
                where_x: None,
                from: None,
                event,
                condition: None,
                limits: Vec::new(),
                effect: OneShotEffect::Act(Action::By(
                    Reference::You,
                    PlayerAction::Draw(Count::Literal(1)),
                )),
            })],
            ..CardFace::default()
        });
        let card_id = state.cards.push(Arc::new(card), controller);
        let id = state.objects.mint(
            ObjectSource::Card(card_id),
            controller,
            Some(Zone::Battlefield),
        );
        state.zones.battlefield.push(id);
        state.objects.obj(id).source
    }

    /// Steps `state` forward exactly `n` times, asserting every call makes
    /// `Progress` (never surfaces a decision or ends the game) — the
    /// surgical alternative to `drain_progress`'s open-ended draining for a
    /// test that drives MULTIPLE actions through one shared state.
    /// `drain_progress`'s generous bound is fine for a test that drives a
    /// single action and then only reads `state.history` (the Task 3/4
    /// tests above), but draining generously a SECOND time on the same
    /// `game()` fixture risks running past the action under test into this
    /// bare fixture's leftover ambient turn-structure agenda (the one
    /// `BeginStep` `GameState::new()` schedules), surfacing an unrelated
    /// `Priority`/`DeclareAttackers` decision that then blocks the next
    /// `run_effect` — `step()` returns `NeedsDecision` idempotently without
    /// popping the agenda while ANY decision is pending, including a stray
    /// one this test never answers. Panics with the offending outcome if a
    /// step count assumption is wrong, rather than failing many calls later
    /// with a confusing "wrong decision kind" mismatch.
    fn step_n(state: &mut GameState, n: usize) {
        for i in 0..n {
            match state.step() {
                StepOutcome::Progress(_) => {}
                other => panic!("step {i}/{n}: expected Progress, got {other:?}"),
            }
        }
    }

    /// [CR#705.2]: a trigger on `CoinFlipped { won: Some(true) }` notes only
    /// for a CALLED flip the flipper WON — never a loss, and (per the sibling
    /// test) never an uncalled flip at all. Seed-pinned (`game()`'s seed 7):
    /// drives single called flips — always calling heads — until this
    /// deterministic sequence lands a win, asserting along the way that every
    /// LOSS left the trigger silent, then that the WIN noted it exactly once.
    /// The retry count is bounded (50 — astronomically unreachable for a fair
    /// coin, so a real regression fails loud) but the actual path taken is
    /// fixed by the seed, so the test is not flaky.
    #[test]
    fn win_flip_trigger_fires_on_won_called_flip_only() {
        use deckmaste_core::EventFilter;

        use crate::decide::Decision;
        use crate::decide::PendingDecision;

        let mut state = game();
        let p0 = PlayerId(0);
        let watcher = put_watcher(
            &mut state,
            p0,
            EventFilter::CoinFlipped {
                by: Predicate::Ref(Reference::You),
                won: Some(true),
            },
        );
        let frame = frame_for(&state, p0);

        let mut won = false;
        for i in 0..50 {
            state.run_effect(
                OneShotEffect::act_by_you(PlayerAction::FlipCoins(Count::Literal(1), true)),
                &frame,
            );
            // Pop exactly the front-scheduled `FlipCoins` work item — it
            // sets `pending` directly; no decision surfaces on this step.
            step_n(&mut state, 1);
            let Some(PendingDecision::CallFlip { player }) = state.pending.clone() else {
                panic!(
                    "attempt {i}: expected a pending CallFlip, got {:?}",
                    state.pending
                );
            };
            assert_eq!(player, p0, "attempt {i}");
            state.submit_decision(Decision::Answer(true)).unwrap();
            // Pop exactly the front-scheduled `CoinFlipped` batch — one
            // step, which also runs the trigger scan synchronously
            // (scheduling a `TriggerFired` at the front on a match).
            step_n(&mut state, 1);

            let flips: Vec<Option<bool>> = state
                .history
                .entries()
                .filter_map(|e| match &e.fact {
                    GameEvent::CoinFlipped { won, .. } => Some(*won),
                    _ => None,
                })
                .collect();
            assert_eq!(
                flips.len(),
                i + 1,
                "attempt {i}: one CoinFlipped fact per called flip so far"
            );
            if flips[i] == Some(true) {
                won = true;
                // The trigger matched — its `TriggerFired` sits at the
                // agenda front (scheduled by the scan inside the batch-apply
                // step above); one more step notes it into
                // `pending_triggers`.
                step_n(&mut state, 1);
                assert_eq!(
                    state.pending_triggers.len(),
                    1,
                    "attempt {i}: a WIN must note the win-only trigger exactly once: {:?}",
                    state.pending_triggers
                );
                assert_eq!(state.pending_triggers[0].source, watcher, "attempt {i}");
                assert_eq!(state.pending_triggers[0].controller, p0, "attempt {i}");
                break;
            }
            assert!(
                state.pending_triggers.is_empty(),
                "attempt {i}: a LOSS must not note the win-only trigger: {:?}",
                state.pending_triggers
            );
        }
        assert!(
            won,
            "seed 7's called-flip sequence never won within 50 attempts"
        );
    }

    /// [CR#705.2]: nobody wins or loses an UNCALLED flip — the same win-only
    /// trigger (`CoinFlipped { won: Some(true) }`) must stay silent on
    /// `FlipCoins(1, false)`, whatever the coin lands.
    #[test]
    fn win_flip_trigger_ignores_uncalled_flips() {
        use deckmaste_core::EventFilter;

        let mut state = game();
        let p0 = PlayerId(0);
        put_watcher(
            &mut state,
            p0,
            EventFilter::CoinFlipped {
                by: Predicate::Ref(Reference::You),
                won: Some(true),
            },
        );
        let frame = frame_for(&state, p0);
        state.run_effect(
            OneShotEffect::act_by_you(PlayerAction::FlipCoins(Count::Literal(1), false)),
            &frame,
        );
        drain_progress(&mut state, 20);

        let flips: Vec<Option<bool>> = state
            .history
            .entries()
            .filter_map(|e| match &e.fact {
                GameEvent::CoinFlipped { won, .. } => Some(*won),
                _ => None,
            })
            .collect();
        assert_eq!(
            flips,
            vec![None],
            "the uncalled flip drew but crowned no winner"
        );
        assert!(
            state.pending_triggers.is_empty(),
            "[CR#705.2]: an uncalled flip must never fire a win-only trigger: {:?}",
            state.pending_triggers
        );
    }

    /// [CR#706.1]: `DiceRolled` fires once PER DIE — a 2-die roll notes the
    /// watching trigger twice, not once for the whole roll.
    #[test]
    fn dice_trigger_fires_per_die() {
        use deckmaste_core::EventFilter;

        let mut state = game();
        let p0 = PlayerId(0);
        put_watcher(
            &mut state,
            p0,
            EventFilter::DiceRolled {
                by: Predicate::Ref(Reference::You),
            },
        );
        let frame = frame_for(&state, p0);
        state.run_effect(
            OneShotEffect::act_by_you(PlayerAction::RollDice(Count::Literal(2), 6)),
            &frame,
        );
        drain_progress(&mut state, 20);

        assert_eq!(
            state.pending_triggers.len(),
            2,
            "2 dice rolled must note the per-die trigger twice: {:?}",
            state.pending_triggers
        );
    }

    /// The ENTIRE random surface — uncalled flips, a called flip, dice, and a
    /// random discard — replays bit for bit from the same seed given the same
    /// scripted decisions. Two independent `GameState`s (same seed, same
    /// setup), driven through the identical script, log identical
    /// `CoinFlipped`/`DieRolled` facts and sample the identical hand cards
    /// for the random discard.
    ///
    /// The discard sample is compared by POSITION in each drive's own
    /// freshly-minted hand (`minted`), not by raw `ObjectId` — a zone move
    /// remints the object (a fresh `ObjectId`), so only the underlying
    /// `CardId`/`ObjectSource` spine is a stable identity; comparing by
    /// per-drive position sidesteps relying on that spine lining up
    /// numerically across two independently-constructed `GameState`s (it
    /// does, since nothing but the rng draws differs between them, but the
    /// position comparison needs no such assumption).
    ///
    /// The zone-move fact actually compared is `GameEvent::ZoneChanged`
    /// (`from: Hand, to: Graveyard`) — the committed FACT a discard emits;
    /// `ZoneWillChange` is the pre-evolution INTENT and is never recorded to
    /// history (`record_history` skips it on purpose, folded into its
    /// downstream `ZoneChanged`).
    #[test]
    fn full_random_surface_is_replayable() {
        use deckmaste_core::Uint;

        use crate::decide::Decision;
        use crate::decide::PendingDecision;

        /// One drive's recorded surface: `(heads, won)` per coin, `(natural,
        /// result)` per die, and the discard sample's positions in `minted`.
        type DriveResult = (Vec<(bool, Option<bool>)>, Vec<(Uint, Uint)>, Vec<usize>);

        fn drive() -> DriveResult {
            let mut state = game();
            let p0 = PlayerId(0);
            let frame = frame_for(&state, p0);

            let minted: Vec<ObjectSource> = (0..5)
                .map(|i| {
                    let id = mint_in_hand(&mut state, p0, &format!("Replay Card {i}"));
                    state.objects.obj(id).source
                })
                .collect();

            // 2 uncalled flips — one PlayerAction, one `CoinFlipped` batch:
            // one step to dispatch `WorkItem::FlipCoins`, one to apply the
            // batch. (Precise `step_n`, not `drain_progress`'s open-ended
            // bound — see `step_n`'s doc: this test drives FOUR actions
            // through one shared state, and draining generously after each
            // one risks falling through into this bare `game()` fixture's
            // leftover ambient turn-structure agenda, surfacing a stray
            // `Priority` that then blocks the next `run_effect`.)
            state.run_effect(
                OneShotEffect::act_by_you(PlayerAction::FlipCoins(Count::Literal(2), false)),
                &frame,
            );
            step_n(&mut state, 2);

            // 1 called flip — the same call answer on both drives.
            state.run_effect(
                OneShotEffect::act_by_you(PlayerAction::FlipCoins(Count::Literal(1), true)),
                &frame,
            );
            step_n(&mut state, 1);
            assert!(matches!(
                state.pending,
                Some(PendingDecision::CallFlip { .. })
            ));
            state.submit_decision(Decision::Answer(true)).unwrap();
            step_n(&mut state, 1);

            // RollDice(2, 20).
            state.run_effect(
                OneShotEffect::act_by_you(PlayerAction::RollDice(Count::Literal(2), 20)),
                &frame,
            );
            step_n(&mut state, 2);

            // Random discard of 2 from the 5-card hand. THREE steps, not
            // two: dispatch `WorkItem::DiscardRandom` (samples + schedules
            // the `ZoneWillChange` batch), apply that intent batch (which
            // captures LKI/remints and collects the evolved `ZoneChanged`
            // batch into `evolving_batch` rather than applying it inline —
            // `schedule_evolution`'s batch path front-schedules it as its
            // own follow-on `WorkItem::Emit`), then apply THAT batch (which
            // actually records the `ZoneChanged` facts to history).
            state.run_effect(
                OneShotEffect::act_by_you(PlayerAction::Discard {
                    count: Count::Literal(2),
                    what: None,
                    random: true,
                }),
                &frame,
            );
            step_n(&mut state, 3);

            let flips: Vec<(bool, Option<bool>)> = state
                .history
                .entries()
                .filter_map(|e| match &e.fact {
                    GameEvent::CoinFlipped { heads, won, .. } => Some((*heads, *won)),
                    _ => None,
                })
                .collect();
            let rolls: Vec<(Uint, Uint)> = state
                .history
                .entries()
                .filter_map(|e| match &e.fact {
                    GameEvent::DieRolled {
                        natural, result, ..
                    } => Some((*natural, *result)),
                    _ => None,
                })
                .collect();
            let discarded: Vec<usize> = state
                .history
                .entries()
                .filter_map(|e| match &e.fact {
                    GameEvent::ZoneChanged {
                        snapshot,
                        from: Some(Zone::Hand),
                        to: Zone::Graveyard,
                        ..
                    } => minted.iter().position(|&m| m == snapshot.source),
                    _ => None,
                })
                .collect();

            (flips, rolls, discarded)
        }

        let (a_flips, a_rolls, a_discarded) = drive();
        let (b_flips, b_rolls, b_discarded) = drive();
        assert_eq!(
            a_flips.len(),
            3,
            "2 uncalled + 1 called ⇒ 3 CoinFlipped facts"
        );
        assert_eq!(
            a_flips, b_flips,
            "same seed ⇒ identical coin sequence (heads + won)"
        );
        assert_eq!(a_rolls.len(), 2, "2 dice ⇒ 2 DieRolled facts");
        assert_eq!(
            a_rolls, b_rolls,
            "same seed ⇒ identical die sequence (natural + result)"
        );
        assert_eq!(a_discarded.len(), 2, "2 cards sampled for the discard");
        assert_eq!(
            a_discarded, b_discarded,
            "same seed ⇒ identical random-discard sample"
        );
    }
}
