//! Resolution ([CR#608]): dispatch a stack object, and walk its `Effect` AST as
//! reified agenda work. Stage 3 wires the corpus's arms; the rest are `todo!`.

use deckmaste_core::{Ability, Action, Effect, Quantity, Selection, TargetSpec, Type, Uint, Zone};

use crate::agenda::WorkItem;
use crate::event::{GameEvent, Occurrence};
use crate::object::ObjectId;
use crate::stack::{Frame, StackEntry, StackObject};
use crate::state::GameState;

impl GameState {
    /// [CR#608]: resolve the committed stack object `obj`. Schedules the work
    /// and the trailing cleanup event.
    ///
    /// # Panics
    ///
    /// Panics if `obj` is not on the stack — engine invariant, not caller
    /// input.
    pub(crate) fn resolve_object(&mut self, obj: ObjectId) {
        let entry = self
            .stack
            .iter()
            .find(|e| e.object.object() == obj)
            .expect("entry on stack")
            .clone();
        match &entry.object {
            StackObject::Spell(spell) => {
                let spell = *spell;
                if self.is_permanent_spell(spell) {
                    // [CR#608.3]: a permanent spell enters the battlefield.
                    self.schedule_front(vec![WorkItem::Emit(Occurrence::single(
                        GameEvent::ZoneWillChange {
                            object: spell,
                            from: Some(Zone::Stack),
                            to: Zone::Battlefield,
                            enters: None,
                        },
                    ))]);
                } else if self.targets_still_legal(&entry) {
                    // Instant/sorcery with all targets still legal: run its effect.
                    let frame = Frame {
                        source: spell,
                        controller: entry.controller,
                        targets: entry.targets.clone(),
                    };
                    let effect = self
                        .spell_effect(spell)
                        .expect("an instant/sorcery has a Spell ability");
                    self.schedule_front(vec![
                        WorkItem::RunEffect {
                            effect: Box::new(effect),
                            frame,
                        },
                        WorkItem::Emit(Occurrence::single(GameEvent::ZoneWillChange {
                            object: spell,
                            from: Some(Zone::Stack),
                            to: Zone::Graveyard,
                            enters: None,
                        })),
                    ]);
                } else {
                    // [CR#608.2b]: all targets illegal — the spell fizzles.
                    self.schedule_front(vec![WorkItem::Emit(Occurrence::single(
                        GameEvent::ZoneWillChange {
                            object: spell,
                            from: Some(Zone::Stack),
                            to: Zone::Graveyard,
                            enters: None,
                        },
                    ))]);
                }
            }
        }
    }

    /// Interpret one `Effect` node ([CR#608.2]). `Act` becomes one or more
    /// `Emit` work items (via `action_items`); `Sequence` expands to one
    /// `RunEffect` per child.
    ///
    /// # Panics
    ///
    /// Panics on any `Effect` variant not wired for Stage 3.
    pub(crate) fn run_effect(&mut self, effect: Effect, frame: &Frame) {
        match effect {
            Effect::Act(action) => {
                let items = self.action_items(&action, frame);
                self.schedule_front(items);
            }
            Effect::Sequence(children) => {
                let items: Vec<WorkItem> = children
                    .into_iter()
                    .map(|e| WorkItem::RunEffect {
                        effect: Box::new(e),
                        frame: frame.clone(),
                    })
                    .collect();
                self.schedule_front(items);
            }
            other => todo!("stage 3 does not interpret effect {other:?} (the choice seam)"),
        }
    }

    /// The `Emit` work item(s) a single-instruction `Action` produces. Damage
    /// to a multi-valued selection is one simultaneous `Batch` (a later
    /// task); drawing N is N sequential `Single`s ([CR#121.1] — drawn one at
    /// a time).
    pub(crate) fn action_items(&self, action: &Action, frame: &Frame) -> Vec<WorkItem> {
        use crate::event::Occurrence;
        match action {
            Action::DealDamage(sel, qty) => {
                let amount = self.eval_quantity(qty, frame);
                let targets = self.eval_selection_set(sel, frame);
                let events: Vec<GameEvent> = targets
                    .into_iter()
                    .map(|target| GameEvent::DamageDealt {
                        source: frame.source,
                        target,
                        amount,
                    })
                    .collect();
                vec![WorkItem::Emit(occurrence_of(events))]
            }
            Action::Tap(sel) => {
                let events: Vec<GameEvent> = self
                    .eval_selection_set(sel, frame)
                    .into_iter()
                    .map(GameEvent::Tapped)
                    .collect();
                vec![WorkItem::Emit(occurrence_of(events))]
            }
            Action::DrawCards(qty) => {
                let n = self.eval_quantity(qty, frame);
                (0..n)
                    .map(|_| {
                        WorkItem::Emit(Occurrence::Single(GameEvent::CardDrawn {
                            player: frame.controller,
                            object: None,
                        }))
                    })
                    .collect()
            }
            Action::LoseLife(qty) => {
                let amount = self.eval_quantity(qty, frame);
                vec![WorkItem::Emit(Occurrence::Single(GameEvent::LifeLost {
                    player: frame.controller,
                    amount,
                }))]
            }
            other => todo!("stage 3 does not perform action {other:?}"),
        }
    }

    /// A selection resolved to its full set ([CR#608.2d]). `Each` is the
    /// distributive "for each matching object" and enumerates the set. A
    /// `Filter` is already set-valued, so `Selection::All` carries nothing a
    /// bare filter doesn't — it stays an unreached seam (set-wide consumers
    /// take a `Filter` directly). Unary references resolve to a 1-element
    /// set.
    pub(crate) fn eval_selection_set(&self, sel: &Selection, frame: &Frame) -> Vec<ObjectId> {
        match sel {
            Selection::Each(f) => crate::target::candidates(self, f),
            other => vec![self.eval_selection(other, frame)],
        }
    }

    /// Resolve a unary `Selection` to an `ObjectId` ([CR#608.2d] / references).
    ///
    /// # Panics
    ///
    /// Panics on a `Selection` not wired for Stage 3, or an out-of-range
    /// `Target(n)` index.
    fn eval_selection(&self, sel: &Selection, frame: &Frame) -> ObjectId {
        match sel {
            Selection::Target(n) => *frame
                .targets
                .get(*n)
                .expect("announced target index in bounds"),
            Selection::This => frame.source,
            Selection::You => self.player(frame.controller).object,
            other => todo!("stage 3 does not evaluate selection {other:?}"),
        }
    }

    /// Evaluate a `Quantity` to a concrete number.
    ///
    /// # Panics
    ///
    /// Panics on a `Quantity` not wired for Stage 3.
    #[expect(
        clippy::unused_self,
        reason = "future Quantity arms (X, StatOf, …) will read self"
    )]
    fn eval_quantity(&self, qty: &Quantity, _frame: &Frame) -> Uint {
        match qty {
            Quantity::Literal(n) => *n,
            other => todo!("stage 3 does not evaluate quantity {other:?}"),
        }
    }

    /// True iff the card's printed types include a permanent type
    /// (Creature/Artifact/Enchantment/Land/Planeswalker/Battle) and NOT
    /// Instant or Sorcery.
    ///
    /// [CR#110.1]: a permanent spell is one that would enter the battlefield on
    /// resolution. Vanilla Creature → true; Instant `DealDamage` `AnyTarget` →
    /// false.
    #[must_use]
    pub(crate) fn is_permanent_spell(&self, id: ObjectId) -> bool {
        let types = &crate::derive::face(self.def(id)).types;
        let is_permanent_type = types.iter().any(|t| {
            matches!(
                t,
                Type::Creature
                    | Type::Artifact
                    | Type::Enchantment
                    | Type::Land
                    | Type::Planeswalker
                    | Type::Battle
            )
        });
        let is_non_permanent = types
            .iter()
            .any(|t| matches!(t, Type::Instant | Type::Sorcery));
        is_permanent_type && !is_non_permanent
    }

    /// Returns the effect of the spell's first `Ability::Spell(SpellAbility {
    /// effect, .. })`, cloned. Looks through `Ability::Expanded` the way
    /// `derive::tap_mana_ability` does. Returns `None` if there is no Spell
    /// ability.
    #[must_use]
    pub(crate) fn spell_effect(&self, id: ObjectId) -> Option<Effect> {
        crate::derive::abilities(self, id)
            .into_iter()
            .find_map(|a| spell_ability_effect(a))
            .cloned()
    }

    /// [CR#608.2b]: for each chosen target, it still matches its `TargetSpec`'s
    /// filter. Returns `true` if all chosen targets are still legal (or there
    /// are no targets). Stage 2: single target, so "all legal" == "the one
    /// target legal".
    ///
    /// **Announce invariant**: the zip assumes one chosen target per
    /// `TargetSpec` — exactly what the Stage-2 announce flow guarantees. If
    /// you add multi-target targeting, update both sides of the zip.
    ///
    /// # Panics
    ///
    /// Panics on `TargetSpec` variants other than `Target` or `Expanded` —
    /// only `Target(filter)` is wired for Stage 2.
    #[must_use]
    pub(crate) fn targets_still_legal(&self, entry: &StackEntry) -> bool {
        let spell = entry.object.object();
        let specs = self.spell_targets(spell);
        debug_assert_eq!(
            specs.len(),
            entry.targets.len(),
            "announce fills exactly one chosen target per TargetSpec",
        );
        specs.iter().zip(&entry.targets).all(|(spec, &chosen)| {
            // [CR#608.2b]: a target that no longer exists (reminted on zone
            // change) is trivially illegal — the filter can't be satisfied.
            if self.objects.get(chosen).is_none() {
                return false;
            }
            let filter = target_spec_filter(spec);
            crate::target::matches(self, chosen, filter)
        })
    }

    /// The `SpellAbility.targets` of the spell (empty for permanent spells).
    /// Used internally and by `targets_still_legal`.
    #[must_use]
    pub(crate) fn spell_targets(&self, id: ObjectId) -> Vec<TargetSpec> {
        crate::derive::abilities(self, id)
            .into_iter()
            .find_map(|a| spell_targets_list(a))
            .cloned()
            .unwrap_or_default()
    }
}

/// Extracts the `Effect` from the first `Ability::Spell` arm, looking through
/// `Ability::Expanded`.
fn spell_ability_effect(ability: &Ability) -> Option<&Effect> {
    match ability {
        Ability::Spell(s) => Some(&s.effect),
        Ability::Expanded(e) => spell_ability_effect(&e.value),
        _ => None,
    }
}

/// Extracts the `targets` list from the first `Ability::Spell` arm, looking
/// through `Ability::Expanded`.
fn spell_targets_list(ability: &Ability) -> Option<&Vec<TargetSpec>> {
    match ability {
        Ability::Spell(s) => Some(&s.targets),
        Ability::Expanded(e) => spell_targets_list(&e.value),
        _ => None,
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

/// Extracts the `Filter` from a `TargetSpec`. Stage 3 only handles
/// `TargetSpec::Target(filter)` (and `Expanded` wrappers around it).
///
/// This is the single authoritative site for TargetSpec→Filter extraction;
/// both `cast::legal_targets` (announce time) and `targets_still_legal`
/// (resolution time) funnel through here so they stay in sync.
///
/// # Panics
///
/// Panics on `TargetSpec` variants not wired for Stage 3.
pub(crate) fn target_spec_filter(spec: &TargetSpec) -> &deckmaste_core::Filter {
    match spec {
        TargetSpec::Target(f) => f,
        TargetSpec::Expanded(e) => target_spec_filter(&e.value),
        other => todo!("stage 3 does not handle target spec {other:?}"),
    }
}

#[cfg(test)]
mod tests {
    use std::path::Path;
    use std::sync::Arc;

    use deckmaste_cards::plugin::Plugin;
    use deckmaste_core::{
        Action, Card, CharacteristicFilter, Effect, Filter, ObjectKind, Quantity, Selection,
        StateFilter, Type, Zone,
    };

    use crate::agenda::WorkItem;
    use crate::event::{GameEvent, Occurrence};
    use crate::matches as obj_matches;
    use crate::object::ObjectId;
    use crate::player::PlayerId;
    use crate::stack::Frame;
    use crate::state::{GameConfig, GameState, PlayerConfig, StartingPlayer};
    use crate::step::{Progress, StepOutcome};

    fn builtin() -> Plugin {
        Plugin::load(Path::new(env!("CARGO_MANIFEST_DIR")).join("../../plugins/builtin")).unwrap()
    }

    fn testing() -> Plugin {
        Plugin::load_with_sibling_prelude(
            Path::new(env!("CARGO_MANIFEST_DIR")).join("../../plugins/testing"),
        )
        .unwrap()
    }

    fn deck(card: &Arc<Card>, n: usize) -> Vec<Arc<Card>> { vec![Arc::clone(card); n] }

    /// A two-player game; player 0's deck is Vanilla Creature.
    /// Returns the state plus a creature object forced onto the battlefield.
    fn bear_on_field() -> (GameState, ObjectId) {
        let bears = Arc::new(testing().card("Vanilla Creature").unwrap());
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
        });
        let bear = *state.zones.hands[0]
            .iter()
            .find(|&&o| {
                obj_matches(
                    &state,
                    o,
                    &Filter::Characteristic(deckmaste_core::CharacteristicFilter::Type(
                        Type::Creature,
                    )),
                )
            })
            .expect("a Vanilla Creature in the opening hand");
        state.zones.hands[PlayerId(0).index()].retain(|&o| o != bear);
        state.objects.obj_mut(bear).zone = Some(Zone::Battlefield);
        state.zones.battlefield.push(bear);
        (state, bear)
    }

    #[test]
    fn action_items_for_tap_draw_loselife() {
        let (state, src) = bear_on_field();
        let frame = Frame {
            source: src,
            controller: PlayerId(0),
            targets: vec![],
        };

        // Tap(This) -> one Single(Tapped(src))
        let items = state.action_items(&Action::Tap(Selection::This), &frame);
        assert_eq!(
            items,
            vec![WorkItem::Emit(Occurrence::Single(GameEvent::Tapped(src)))]
        );

        // DrawCards(2) -> two sequential Single(CardDrawn) for the controller
        let items = state.action_items(&Action::DrawCards(Quantity::Literal(2)), &frame);
        assert_eq!(items.len(), 2);
        assert!(items.iter().all(|item| matches!(
            item,
            WorkItem::Emit(Occurrence::Single(GameEvent::CardDrawn {
                player: PlayerId(0),
                ..
            }))
        )));

        // LoseLife(3) -> one Single(LifeLost{player0, 3})
        let items = state.action_items(&Action::LoseLife(Quantity::Literal(3)), &frame);
        assert_eq!(
            items,
            vec![WorkItem::Emit(Occurrence::Single(GameEvent::LifeLost {
                player: PlayerId(0),
                amount: 3,
            }))]
        );
    }

    #[test]
    fn each_creature_yields_all_battlefield_creatures() {
        let (mut state, a) = bear_on_field();
        // Force a second Vanilla Creature from player 0's hand onto the battlefield.
        let b = *state.zones.hands[0]
            .iter()
            .find(|&&o| {
                obj_matches(
                    &state,
                    o,
                    &Filter::Characteristic(deckmaste_core::CharacteristicFilter::Type(
                        Type::Creature,
                    )),
                )
            })
            .expect("a second Vanilla Creature in the opening hand");
        state.zones.hands[PlayerId(0).index()].retain(|&o| o != b);
        state.objects.obj_mut(b).zone = Some(Zone::Battlefield);
        state.zones.battlefield.push(b);

        let frame = Frame {
            source: a,
            controller: PlayerId(0),
            targets: vec![],
        };
        let filter = Filter::AllOf(vec![
            Filter::State(StateFilter::InZone(Zone::Battlefield)),
            Filter::Characteristic(CharacteristicFilter::Type(Type::Creature)),
        ]);
        let mut got = state.eval_selection_set(&Selection::Each(filter), &frame);
        got.sort();
        let mut want = vec![a, b];
        want.sort();
        assert_eq!(got, want);
    }

    /// `Each(Kind(Player))` yields exactly the two player proxies (no card
    /// objects), and `DealDamage` wraps them in ONE simultaneous `Batch`.
    #[test]
    fn each_player_deal_damage_emits_one_batch() {
        let (mut state, src) = bear_on_field();
        let frame = Frame {
            source: src,
            controller: PlayerId(0),
            targets: vec![],
        };

        // Build the effect directly: DealDamage(Each(Kind(Player)), 20)
        let effect = Effect::Act(Action::DealDamage(
            Selection::Each(Filter::Kind(ObjectKind::Player)),
            Quantity::Literal(20),
        ));
        state.run_effect(effect, &frame);

        // The agenda front should now have a single Emit(Batch([...])) item.
        let outcome = state.step();
        let Progress::Applied(Occurrence::Batch(events)) = (match outcome {
            StepOutcome::Progress(p) => p,
            other => panic!("expected Progress, got {other:?}"),
        }) else {
            panic!("expected Applied(Batch(…))");
        };

        // Both players took 20 damage, order-independent.
        let p0_obj = state.players[0].object;
        let p1_obj = state.players[1].object;
        let mut got: Vec<_> = events
            .iter()
            .map(|e| match e {
                GameEvent::DamageDealt { target, amount, .. } => (*target, *amount),
                other => panic!("unexpected event {other:?}"),
            })
            .collect();
        got.sort();
        let mut want = vec![(p0_obj, 20u32), (p1_obj, 20u32)];
        want.sort();
        assert_eq!(got, want);
    }

    /// `DealDamage(Each(AllOf([InZone(Battlefield), Type(Creature)])), 2)` with
    /// two creatures on the field emits ONE `Batch` of two `DamageDealt`
    /// events — the sweep fixture drives simultaneous deaths later.
    #[test]
    fn each_creature_deal_damage_emits_one_batch() {
        let (mut state, a) = bear_on_field();
        // Force a second creature onto the battlefield.
        let b = *state.zones.hands[0]
            .iter()
            .find(|&&o| {
                obj_matches(
                    &state,
                    o,
                    &Filter::Characteristic(deckmaste_core::CharacteristicFilter::Type(
                        Type::Creature,
                    )),
                )
            })
            .expect("a second Vanilla Creature in the opening hand");
        state.zones.hands[PlayerId(0).index()].retain(|&o| o != b);
        state.objects.obj_mut(b).zone = Some(Zone::Battlefield);
        state.zones.battlefield.push(b);

        let frame = Frame {
            source: a,
            controller: PlayerId(0),
            targets: vec![],
        };
        let effect = Effect::Act(Action::DealDamage(
            Selection::Each(Filter::AllOf(vec![
                Filter::State(StateFilter::InZone(Zone::Battlefield)),
                Filter::Characteristic(CharacteristicFilter::Type(Type::Creature)),
            ])),
            Quantity::Literal(2),
        ));
        state.run_effect(effect, &frame);

        let outcome = state.step();
        let Progress::Applied(Occurrence::Batch(events)) = (match outcome {
            StepOutcome::Progress(p) => p,
            other => panic!("expected Progress, got {other:?}"),
        }) else {
            panic!("expected Applied(Batch(…))");
        };

        // Both creatures took 2 damage.
        let mut got: Vec<_> = events
            .iter()
            .map(|e| match e {
                GameEvent::DamageDealt { target, amount, .. } => (*target, *amount),
                other => panic!("unexpected event {other:?}"),
            })
            .collect();
        got.sort();
        let mut want = vec![(a, 2u32), (b, 2u32)];
        want.sort();
        assert_eq!(got, want);
    }
}
