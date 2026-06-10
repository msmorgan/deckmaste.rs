//! Resolution ([CR#608]): dispatch a stack object, and walk its `Effect` AST as
//! reified agenda work. Stage 3 wires the corpus's arms; the rest are `todo!`.

use deckmaste_core::{
    Ability, Action, Color, ColorOrColorless, Count, Effect, ManaSpec, PlayerAction, Reference,
    Scope, Selection, StaticEffect, TargetSpec, Type, Uint, Zone,
};

use crate::agenda::WorkItem;
use crate::event::{GameEvent, Occurrence};
use crate::layer::{ContinuousEffect, ScopeResolved};
use crate::object::ObjectId;
use crate::stack::{Frame, StackEntry, StackObject};
use crate::state::GameState;

/// "Any color" ([CR#106.1b]): the five colors ([CR#105.1]) — a player asked to
/// choose a color may not choose colorless ([CR#105.4]).
const ANY_COLOR: [ColorOrColorless; 5] = [
    ColorOrColorless::Color(Color::White),
    ColorOrColorless::Color(Color::Blue),
    ColorOrColorless::Color(Color::Black),
    ColorOrColorless::Color(Color::Red),
    ColorOrColorless::Color(Color::Green),
];

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
    pub(crate) fn resolve_object(&mut self, id: ObjectId) {
        let entry = self
            .stack
            .iter()
            .find(|e| e.id == id)
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
                            position: None,
                        },
                    ))]);
                } else if self.targets_still_legal(&entry) {
                    // Instant/sorcery with all targets still legal: run its effect.
                    let frame = Frame {
                        source: spell,
                        controller: entry.controller,
                        targets: entry.targets.clone(),
                        bindings: None,
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
                            position: None,
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
                            position: None,
                        },
                    ))]);
                }
            }
            // [CR#603.8]: a triggered ability resolves its effect, then vanishes
            // — no zone move, the source untouched. The minted stack id is just
            // discarded when `AbilityResolved` removes the entry.
            StackObject::Triggered {
                source,
                ability,
                bindings,
            } => {
                let t = match &crate::derive::abilities_of_source(self, *source)[*ability] {
                    Ability::Triggered(t) => t.clone(),
                    other => unreachable!(
                        "a Triggered stack object indexes a Triggered ability, got {other:?}"
                    ),
                };
                let frame = Frame {
                    // [CR#608.2,603.10a]: `~`/`This` is the firing object's
                    // last-known self; the live source may be gone.
                    source: bindings.this.as_ref().map_or(entry.id, |s| s.object),
                    controller: entry.controller,
                    targets: entry.targets.clone(),
                    bindings: Some(bindings.clone()),
                };
                self.schedule_front(vec![
                    WorkItem::RunEffect {
                        effect: Box::new(t.effect),
                        frame,
                    },
                    WorkItem::Emit(Occurrence::single(GameEvent::AbilityResolved(entry.id))),
                ]);
            }
            StackObject::Activated { .. } => {
                todo!("activation pipeline task: resolve activated abilities")
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
            Effect::Continuously(e) => {
                // [CR#611.2]/[CR#611.2c]: stamp at creation; lock the object set
                // for non-floating scopes, leave `Matching` floating.
                let timestamp = self.objects.next_timestamp();
                if let StaticEffect::Modify { of, changes } = &*e.effect {
                    let scope = match of {
                        Scope::Matching(f) => ScopeResolved::Floating(f.clone()),
                        Scope::Of(r) => ScopeResolved::Locked(vec![self.eval_reference(r, frame)]),
                        Scope::These(rs) => ScopeResolved::Locked(
                            rs.iter().map(|r| self.eval_reference(r, frame)).collect(),
                        ),
                    };
                    self.continuous.push(ContinuousEffect {
                        timestamp,
                        scope,
                        changes: changes.clone(),
                        duration: e.duration.clone(),
                        is_cda: false,
                    });
                }
                // Non-Modify static effects (Restriction/Permission/...) are a
                // later seam — explicit no-op, not silently
                // dropped.
            }
            other => todo!("stage 3 does not interpret effect {other:?} (the choice seam)"),
        }
    }

    /// The `Emit` work item(s) a single-instruction `Action` produces. The
    /// source verbs (`DealDamage`, …) act with the source object as agent; the
    /// player verbs live under `By(who, …)`, where `who` resolves to the acting
    /// player and replaces the previously hard-coded `frame.controller`. Damage
    /// to a multi-valued selection is one simultaneous `Batch` (a later task);
    /// drawing N is N sequential `Single`s ([CR#121.1] — drawn one at a time).
    pub(crate) fn action_items(&self, action: &Action, frame: &Frame) -> Vec<WorkItem> {
        match action {
            Action::DealDamage(sel, qty) => {
                let amount = self.eval_count(qty, frame);
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
            // The named player performs the verb: resolve `who` to the acting
            // player, then dispatch the `PlayerAction`. `By(You, …)` (the
            // implicit-you default) resolves to `frame.controller` — identical
            // to the previous hard-coded behavior.
            Action::By(who, pa) => {
                let actor = self.acting_player(who, frame);
                self.player_action_items(pa, actor, frame)
            }
            other => todo!("stage 3 does not perform action {other:?}"),
        }
    }

    /// The `Emit` work item(s) one `PlayerAction` produces, performed by
    /// `actor` (the agent the enclosing `By` resolved to).
    #[expect(
        clippy::too_many_lines,
        reason = "one arm per player verb; splitting would scatter the dispatch"
    )]
    fn player_action_items(
        &self,
        action: &PlayerAction,
        actor: crate::player::PlayerId,
        frame: &Frame,
    ) -> Vec<WorkItem> {
        use crate::event::Occurrence;
        match action {
            PlayerAction::Tap(sel) => {
                let events: Vec<GameEvent> = self
                    .eval_selection_set(sel, frame)
                    .into_iter()
                    .map(GameEvent::Tapped)
                    .collect();
                vec![WorkItem::Emit(occurrence_of(events))]
            }
            PlayerAction::Draw(qty) => {
                let n = self.eval_count(qty, frame);
                (0..n)
                    .map(|_| {
                        WorkItem::Emit(Occurrence::Single(GameEvent::WillDraw {
                            player: actor,
                            source: Some(frame.source),
                        }))
                    })
                    .collect()
            }
            PlayerAction::LoseLife(qty) => {
                let amount = self.eval_count(qty, frame);
                vec![WorkItem::Emit(Occurrence::Single(GameEvent::LifeLost {
                    player: actor,
                    amount,
                }))]
            }
            PlayerAction::GainLife(qty) => {
                let amount = self.eval_count(qty, frame);
                vec![WorkItem::Emit(Occurrence::Single(GameEvent::LifeGained {
                    player: actor,
                    amount,
                }))]
            }
            PlayerAction::Untap(sel) => {
                // [CR#701.26b]: the mirror of `Tap` above.
                let events: Vec<GameEvent> = self
                    .eval_selection_set(sel, frame)
                    .into_iter()
                    .map(GameEvent::Untapped)
                    .collect();
                vec![WorkItem::Emit(occurrence_of(events))]
            }
            PlayerAction::Sacrifice(sel) => {
                // [CR#701.21a]: the actor moves each selected permanent to its
                // owner's graveyard — the `Sacrificed` verb fact evolves into
                // the zone move at apply. That the selection names permanents
                // the actor controls is the grammar's contract; a legality
                // pass is a later seam.
                let events: Vec<GameEvent> = self
                    .eval_selection_set(sel, frame)
                    .into_iter()
                    .map(|object| GameEvent::Sacrificed {
                        player: actor,
                        object,
                    })
                    .collect();
                vec![WorkItem::Emit(occurrence_of(events))]
            }
            PlayerAction::Exile(sel) => {
                // [CR#701.13a]: move each selected object to exile from
                // whatever zone it's in ([CR#406.2]) — bound at schedule time,
                // like every selection here.
                let events: Vec<GameEvent> = self
                    .eval_selection_set(sel, frame)
                    .into_iter()
                    .map(|object| GameEvent::ZoneWillChange {
                        object,
                        from: Some(self.objects.obj(object).zone.expect("exile a zoned object")),
                        to: Zone::Exile,
                        enters: None,
                        position: None,
                    })
                    .collect();
                vec![WorkItem::Emit(occurrence_of(events))]
            }
            PlayerAction::PutInLibrary(sel, pos) => {
                // [CR#401.7]: `pos` indexes from the top (0 = top), clamped to
                // the bottom at apply. [CR#401.4]: the owner's arrangement
                // choice for a multi-card selection is a seam — cards move one
                // at a time in selection order, each inserting at the same
                // index (so the last placed ends up frontmost of the group).
                let position = self.eval_count(pos, frame);
                self.eval_selection_set(sel, frame)
                    .into_iter()
                    .map(|object| {
                        WorkItem::Emit(Occurrence::Single(GameEvent::ZoneWillChange {
                            object,
                            from: Some(self.objects.obj(object).zone.expect("a zoned object")),
                            to: Zone::Library,
                            enters: None,
                            position: Some(position),
                        }))
                    })
                    .collect()
            }
            PlayerAction::AddMana(qty, spec) => {
                let amount = self.eval_count(qty, frame);
                match spec {
                    // A fixed production needs no choice.
                    ManaSpec::Specific(mana) => {
                        vec![WorkItem::Emit(Occurrence::Single(GameEvent::ManaAdded {
                            player: actor,
                            mana: *mana,
                            amount,
                        }))]
                    }
                    // [CR#106.1b]: the actor chooses on resolution — surfaced
                    // explicitly even when only one option exists (engine
                    // policy: every choice surfaces).
                    ManaSpec::AnyColor => vec![WorkItem::ChooseManaColor {
                        player: actor,
                        options: ANY_COLOR.to_vec(),
                        amount,
                    }],
                    ManaSpec::OneOf(options) => vec![WorkItem::ChooseManaColor {
                        player: actor,
                        options: options.clone(),
                        amount,
                    }],
                }
            }
            PlayerAction::Discard(qty) => {
                // [CR#701.9b]: the actor chooses which cards — surfaced as a
                // decision when the work item applies (the hand may change
                // before then).
                let count = self.eval_count(qty, frame);
                vec![WorkItem::DiscardCards {
                    player: actor,
                    count,
                }]
            }
            // Look through a remembered macro invocation.
            PlayerAction::Expanded(e) => self.player_action_items(&e.value, actor, frame),
            // `Create` belongs to the `engine-tokens` item: token identity
            // (`ObjectSource` has no token arm), the layer-base plumbing, and
            // the ceases-to-exist SBA land together there.
            other @ PlayerAction::Create(..) => {
                todo!("stage 3 does not perform player action {other:?}")
            }
        }
    }

    /// Resolves the agent of a `By(who, …)` to the acting `PlayerId`. `who` is
    /// a [`Reference`] that resolves (via `eval_reference`) to a player proxy
    /// object; this maps that proxy back to its `PlayerId`.
    ///
    /// # Panics
    ///
    /// Panics if `who` resolves to a non-player object — a player verb's agent
    /// must be a player ([CR#608.2]).
    fn acting_player(&self, who: &Reference, frame: &Frame) -> crate::player::PlayerId {
        use crate::object::ObjectSource;
        let object = self.eval_reference(who, frame);
        match self.objects.get(object).map(|o| o.source) {
            Some(ObjectSource::Player(p)) => p,
            other => panic!("a player verb's agent must be a player, got {other:?}"),
        }
    }

    /// A selection resolved to its full set ([CR#608.2d]). `Each` is the
    /// distributive "for each matching object" and enumerates the set.
    /// `Selection::Filter` is set-valued but stays an unreached seam here
    /// (set-wide consumers take a `Filter` directly). Unary references resolve
    /// to a 1-element set.
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
            // A bound reference lifted into a choice slot: funnel to the
            // reference resolver.
            Selection::Ref(reference) => self.eval_reference(reference, frame),
            other => todo!("stage 3 does not evaluate selection {other:?}"),
        }
    }

    /// Resolve a [`Reference`] to an `ObjectId` (the bound-object resolver
    /// `Selection::Ref` funnels through).
    ///
    /// # Panics
    ///
    /// Panics on a `Reference` not wired for Stage 3, or an out-of-range
    /// `Target(n)` index.
    fn eval_reference(&self, reference: &Reference, frame: &Frame) -> ObjectId {
        match reference {
            Reference::Target(n) => *frame
                .targets
                .get(*n)
                .expect("announced target index in bounds"),
            // [CR#603.10a]: for a triggered ability, `~`/`This` is the firing
            // object's last-known self (the live source may be gone); for a
            // spell frame (no bindings) it is the live source.
            Reference::This => frame
                .bindings
                .as_ref()
                .and_then(|b| b.this.as_ref())
                .map_or(frame.source, |s| s.object),
            Reference::You => self.player(frame.controller).object,
            other => todo!("stage 3 does not evaluate reference {other:?}"),
        }
    }

    /// Evaluate a `Count` to a concrete number.
    ///
    /// # Panics
    ///
    /// Panics on a `Count` not wired for Stage 3.
    #[expect(
        clippy::unused_self,
        reason = "future Count arms (X, StatOf, …) will read self"
    )]
    fn eval_count(&self, qty: &Count, _frame: &Frame) -> Uint {
        match qty {
            Count::Literal(n) => *n,
            other => todo!("stage 3 does not evaluate count {other:?}"),
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
            .iter()
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
    /// only single-target `Target(_, _)` is wired (multi-target is Stage 4).
    #[must_use]
    pub(crate) fn targets_still_legal(&self, entry: &StackEntry) -> bool {
        let spell = entry.object.object();
        let specs = spell_targets(&self.layers(), spell);
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
}

/// The `SpellAbility.targets` of the spell (empty for permanent spells).
/// Used by the cast checks and `targets_still_legal`. Reads the caller's
/// derived view — the legality loop checks every hand card against one
/// view instead of re-deriving the board per card.
#[must_use]
pub(crate) fn spell_targets(view: &crate::layer::LayeredView, id: ObjectId) -> Vec<TargetSpec> {
    view.get(id)
        .abilities
        .iter()
        .find_map(|a| spell_targets_list(a))
        .cloned()
        .unwrap_or_default()
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
/// `TargetSpec::Target(Exactly(Literal(1)), filter)` (and `Expanded` wrappers
/// around it).
///
/// This is the single authoritative site for TargetSpec→Filter extraction;
/// both `cast::legal_targets` (announce time) and `targets_still_legal`
/// (resolution time) funnel through here so they stay in sync.
///
/// # Panics
///
/// Panics on `TargetSpec` quantities not wired for Stage 3.
pub(crate) fn target_spec_filter(spec: &TargetSpec) -> &deckmaste_core::Filter {
    match spec {
        TargetSpec::Target(_quantity, f) => {
            // TODO(stage-4): enforce quantity; for now, Stage 3 only exercises
            // single targets and callers expect exactly one target slot.
            f
        }
        TargetSpec::Expanded(e) => target_spec_filter(&e.value),
    }
}

#[cfg(test)]
mod tests {
    use std::path::Path;
    use std::sync::Arc;

    use deckmaste_cards::plugin::Plugin;
    use deckmaste_core::{
        Action, Card, CharacteristicFilter, Count, Effect, Filter, ObjectKind, PlayerAction,
        Reference, Selection, StateFilter, Type, Zone,
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

    /// `Action::By(You, pa)` — the implicit-you default a bare player verb
    /// reads as.
    fn by_you(pa: PlayerAction) -> Action { Action::By(Reference::You, pa) }

    /// `Selection::Ref(This)` — the common "this object" selection.
    fn sel_this() -> Selection { Selection::Ref(Reference::This) }

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
            bindings: None,
        };

        // By(You, Tap(This)) -> one Single(Tapped(src))
        let items = state.action_items(&by_you(PlayerAction::Tap(sel_this())), &frame);
        assert_eq!(
            items,
            vec![WorkItem::Emit(Occurrence::Single(GameEvent::Tapped(src)))]
        );

        // By(You, Draw(2)) -> two sequential Single(WillDraw) for the controller
        let items = state.action_items(&by_you(PlayerAction::Draw(Count::Literal(2))), &frame);
        assert_eq!(items.len(), 2);
        assert!(items.iter().all(|item| matches!(
            item,
            WorkItem::Emit(Occurrence::Single(GameEvent::WillDraw {
                player: PlayerId(0),
                ..
            }))
        )));

        // By(You, LoseLife(3)) -> one Single(LifeLost{player0, 3})
        let items = state.action_items(&by_you(PlayerAction::LoseLife(Count::Literal(3))), &frame);
        assert_eq!(
            items,
            vec![WorkItem::Emit(Occurrence::Single(GameEvent::LifeLost {
                player: PlayerId(0),
                amount: 3,
            }))]
        );
    }

    /// An explicit agent: `By(Target(0), Draw(2))` draws for the targeted
    /// player, not the controller. Targets player 1's proxy.
    #[test]
    fn action_items_explicit_agent_draws_for_target() {
        let (state, src) = bear_on_field();
        let p1_proxy = state.players[1].object;
        let frame = Frame {
            source: src,
            controller: PlayerId(0),
            targets: vec![p1_proxy],
            bindings: None,
        };
        let items = state.action_items(
            &Action::By(Reference::Target(0), PlayerAction::Draw(Count::Literal(2))),
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
            bindings: None,
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
            bindings: None,
        };

        // Build the effect directly: DealDamage(Each(Kind(Player)), 20)
        let effect = Effect::Act(Action::DealDamage(
            Selection::Each(Filter::Kind(ObjectKind::Player)),
            Count::Literal(20),
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
            bindings: None,
        };
        let effect = Effect::Act(Action::DealDamage(
            Selection::Each(Filter::AllOf(vec![
                Filter::State(StateFilter::InZone(Zone::Battlefield)),
                Filter::Characteristic(CharacteristicFilter::Type(Type::Creature)),
            ])),
            Count::Literal(2),
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

    /// [CR#611.2]/[CR#611.2c]: `Effect::Continuously(Modify(Matching(...), ...),
    /// UntilEndOfTurn)` — the resolve arm pushes one `ContinuousEffect` with a
    /// `ScopeResolved::Floating` scope and the right duration/changes.
    #[test]
    fn continuously_matching_registers_floating_scope() {
        use deckmaste_core::{
            CharacteristicFilter, ContinuouslyEffect, Count, Duration, Effect, Filter,
            Modification, Scope, StaticEffect, Type,
        };

        let (mut state, src) = bear_on_field();
        let frame = Frame {
            source: src,
            controller: PlayerId(0),
            targets: vec![],
            bindings: None,
        };

        assert!(state.continuous.is_empty(), "no effects before resolve");

        let filter = Filter::Characteristic(CharacteristicFilter::Type(Type::Creature));
        let effect = Effect::Continuously(ContinuouslyEffect {
            effect: Box::new(StaticEffect::Modify {
                of: Scope::Matching(filter.clone()),
                changes: vec![Modification::AddPower(Count::Literal(1))],
            }),
            duration: Duration::UntilEndOfTurn,
        });
        state.run_effect(effect, &frame);

        assert_eq!(state.continuous.len(), 1, "one effect registered");
        let ce = &state.continuous[0];
        assert!(
            matches!(&ce.scope, crate::layer::ScopeResolved::Floating(f) if f == &filter),
            "scope is Floating(creature filter)"
        );
        assert_eq!(ce.duration, Duration::UntilEndOfTurn);
        assert_eq!(ce.changes, vec![Modification::AddPower(Count::Literal(1))]);
        assert!(!ce.is_cda);
    }

    /// [CR#611.2c]: `Effect::Continuously(Modify(Of(This), ...), ...)` locks
    /// the id at creation — `ScopeResolved::Locked(vec![src])`.
    #[test]
    fn continuously_of_this_registers_locked_scope() {
        use deckmaste_core::{
            ContinuouslyEffect, Count, Duration, Effect, Modification, Reference, Scope,
            StaticEffect,
        };

        let (mut state, src) = bear_on_field();
        let frame = Frame {
            source: src,
            controller: PlayerId(0),
            targets: vec![],
            bindings: None,
        };

        let effect = Effect::Continuously(ContinuouslyEffect {
            effect: Box::new(StaticEffect::Modify {
                of: Scope::Of(Reference::This),
                changes: vec![Modification::AddToughness(Count::Literal(2))],
            }),
            duration: Duration::UntilEndOfTurn,
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
            vec![Modification::AddToughness(Count::Literal(2))]
        );
    }

    /// `By(You, GainLife(3))` → one `LifeGained`; `By(You, Untap(This))` → one
    /// `Untapped` — the mirrors of `LoseLife`/`Tap` above.
    #[test]
    fn action_items_for_gainlife_untap() {
        let (state, src) = bear_on_field();
        let frame = Frame {
            source: src,
            controller: PlayerId(0),
            targets: vec![],
            bindings: None,
        };

        let items = state.action_items(&by_you(PlayerAction::GainLife(Count::Literal(3))), &frame);
        assert_eq!(
            items,
            vec![WorkItem::Emit(Occurrence::Single(GameEvent::LifeGained {
                player: PlayerId(0),
                amount: 3,
            }))]
        );

        let items = state.action_items(&by_you(PlayerAction::Untap(sel_this())), &frame);
        assert_eq!(
            items,
            vec![WorkItem::Emit(Occurrence::Single(GameEvent::Untapped(src)))]
        );
    }

    /// [CR#701.21a]: `Sacrifice(This)` emits the verb fact, which evolves into
    /// the Battlefield→Graveyard move — old id gone, fresh object in the
    /// owner's graveyard.
    #[test]
    fn sacrifice_this_remints_to_owners_graveyard() {
        let (mut state, bear) = bear_on_field();
        let frame = Frame {
            source: bear,
            controller: PlayerId(0),
            targets: vec![],
            bindings: None,
        };
        state.run_effect(
            Effect::Act(by_you(PlayerAction::Sacrifice(sel_this()))),
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
    /// creature's own dies-trigger fires ([CR#603.6d] — the leaving object
    /// watches its own departure).
    #[test]
    fn sacrifice_fires_the_dying_objects_dies_trigger() {
        use crate::object::ObjectSource;

        let card = Arc::new(
            testing()
                .card("Creature dies-trigger DealDamage AnyTarget")
                .unwrap(),
        );
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
        });
        let card_id = state.cards.push(card, PlayerId(0));
        let gob = state.objects.mint(
            ObjectSource::Card(card_id),
            PlayerId(0),
            Some(Zone::Battlefield),
        );
        state.zones.battlefield.push(gob);

        let frame = Frame {
            source: gob,
            controller: PlayerId(0),
            targets: vec![],
            bindings: None,
        };
        state.run_effect(
            Effect::Act(by_you(PlayerAction::Sacrifice(sel_this()))),
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
        let frame = Frame {
            source: bear,
            controller: PlayerId(0),
            targets: vec![],
            bindings: None,
        };
        state.run_effect(Effect::Act(by_you(PlayerAction::Exile(sel_this()))), &frame);
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
        let frame = Frame {
            source: card,
            controller: PlayerId(0),
            targets: vec![],
            bindings: None,
        };
        state.run_effect(Effect::Act(by_you(PlayerAction::Exile(sel_this()))), &frame);
        for _ in 0..2 {
            let _ = state.step();
        }
        assert!(state.objects.get(card).is_none(), "old graveyard id gone");
        assert!(state.zones.graveyards[0].is_empty());
        assert_eq!(state.zones.exile.len(), 2);
    }

    /// [CR#401.7]: `PutInLibrary(This, 0)` puts the card on top; an index past
    /// the bottom clamps to the bottom.
    #[test]
    fn put_in_library_top_and_clamped_bottom() {
        let (mut state, bear) = bear_on_field();
        let bear_card = state.objects.obj(bear).card_id().expect("card-backed");
        let lib_before = state.zones.libraries[0].len();
        let frame = Frame {
            source: bear,
            controller: PlayerId(0),
            targets: vec![],
            bindings: None,
        };
        state.run_effect(
            Effect::Act(by_you(PlayerAction::PutInLibrary(
                sel_this(),
                Count::Literal(0),
            ))),
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

        // Past the bottom ([CR#401.7]): index 99 places it on the bottom.
        let frame = Frame {
            source: top,
            controller: PlayerId(0),
            targets: vec![],
            bindings: None,
        };
        state.run_effect(
            Effect::Act(by_you(PlayerAction::PutInLibrary(
                sel_this(),
                Count::Literal(99),
            ))),
            &frame,
        );
        for _ in 0..2 {
            let _ = state.step();
        }
        assert_eq!(state.zones.libraries[0].len(), lib_before + 1);
        let bottom = *state.zones.libraries[0].back().expect("non-empty library");
        assert_eq!(state.objects.obj(bottom).card_id(), Some(bear_card));
    }

    /// `AddMana(2, Green)` needs no choice and lands in the pool ([CR#106.4]);
    /// `AddMana(1, AnyColor)` surfaces `ChooseManaColor` with the five colors
    /// — colorless is not a color ([CR#105.4]) and is rejected.
    #[test]
    fn add_mana_specific_and_any_color() {
        use deckmaste_core::{Color, ColorOrColorless, ManaSpec};

        use crate::decide::{Decision, PendingDecision};
        use crate::step::StepOutcome;

        let (mut state, src) = bear_on_field();
        let frame = Frame {
            source: src,
            controller: PlayerId(0),
            targets: vec![],
            bindings: None,
        };
        let green = ColorOrColorless::Color(Color::Green);
        state.run_effect(
            Effect::Act(by_you(PlayerAction::AddMana(
                Count::Literal(2),
                ManaSpec::Specific(green),
            ))),
            &frame,
        );
        let _ = state.step();
        assert_eq!(state.players[0].mana_pool.amount(green), 2);

        state.run_effect(
            Effect::Act(by_you(PlayerAction::AddMana(
                Count::Literal(1),
                ManaSpec::AnyColor,
            ))),
            &frame,
        );
        let _ = state.step(); // ManaColorOpened
        let StepOutcome::NeedsDecision(PendingDecision::ChooseManaColor {
            player,
            options,
            amount,
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

    /// [CR#701.9b]: `Discard(2)` surfaces the card choice; a wrong-sized answer
    /// is rejected; the right answer discards through the Hand→Graveyard
    /// pipeline. Discarding more than the hand holds clamps to the whole hand
    /// ([CR#101.3]).
    #[test]
    fn discard_surfaces_choice_validates_and_clamps() {
        use crate::decide::{Decision, PendingDecision};
        use crate::step::StepOutcome;

        let (mut state, src) = bear_on_field();
        let frame = Frame {
            source: src,
            controller: PlayerId(0),
            targets: vec![],
            bindings: None,
        };
        let hand_before = state.zones.hands[0].len();
        state.run_effect(
            Effect::Act(by_you(PlayerAction::Discard(Count::Literal(2)))),
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
            Effect::Act(by_you(PlayerAction::Discard(Count::Literal(99)))),
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
        use deckmaste_core::{Expansion, ExpansionArgs};

        let (state, src) = bear_on_field();
        let frame = Frame {
            source: src,
            controller: PlayerId(0),
            targets: vec![],
            bindings: None,
        };
        let body = PlayerAction::GainLife(Count::Literal(2));
        let expanded = PlayerAction::Expanded(Expansion {
            name: "GainTwo".into(),
            args: ExpansionArgs::none(),
            value: Box::new(body.clone()),
        });
        assert_eq!(
            state.action_items(&by_you(expanded), &frame),
            state.action_items(&by_you(body), &frame),
        );
    }
}
