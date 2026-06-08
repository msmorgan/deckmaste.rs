//! Resolution ([CR#608]): dispatch a stack object, and walk its `Effect` AST as
//! reified agenda work. Stage 2 wires the corpus's arms; the rest are `todo!`.

use deckmaste_core::{Ability, Action, Effect, Quantity, Selection, TargetSpec, Type, Uint};

use crate::agenda::WorkItem;
use crate::event::GameEvent;
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
                    self.schedule_front(vec![WorkItem::Emit(GameEvent::EntersBattlefield(spell))]);
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
                        WorkItem::Emit(GameEvent::SpellResolved(spell)),
                    ]);
                } else {
                    // [CR#608.2b]: all targets illegal — the spell fizzles.
                    self.schedule_front(vec![WorkItem::Emit(GameEvent::SpellResolved(spell))]);
                }
            }
        }
    }

    /// Interpret one `Effect` node ([CR#608.2]). `Act` becomes a concrete event
    /// (scheduled through the Emit pipe); `Sequence` expands to one
    /// `RunEffect` per child.
    ///
    /// # Panics
    ///
    /// Panics on any `Effect` variant not wired for Stage 2.
    pub(crate) fn run_effect(&mut self, effect: Effect, frame: &Frame) {
        match effect {
            Effect::Act(action) => {
                let event = self.action_event(&action, frame);
                self.schedule_front(vec![WorkItem::Emit(event)]);
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
            other => todo!("stage 2 does not interpret effect {other:?} (the choice seam)"),
        }
    }

    /// Build the concrete game event for a single-instruction `Action`.
    ///
    /// # Panics
    ///
    /// Panics on any `Action` variant not wired for Stage 2.
    fn action_event(&self, action: &Action, frame: &Frame) -> GameEvent {
        match action {
            Action::DealDamage(sel, qty) => GameEvent::DamageDealt {
                source: frame.source,
                target: self.eval_selection(sel, frame),
                amount: self.eval_quantity(qty, frame),
            },
            other => todo!("stage 2 does not perform action {other:?}"),
        }
    }

    /// Resolve a unary `Selection` to an `ObjectId` ([CR#608.2d] / references).
    ///
    /// # Panics
    ///
    /// Panics on a `Selection` not wired for Stage 2, or an out-of-range
    /// `Target(n)` index.
    fn eval_selection(&self, sel: &Selection, frame: &Frame) -> ObjectId {
        match sel {
            Selection::Target(n) => *frame
                .targets
                .get(*n)
                .expect("announced target index in bounds"),
            Selection::This => frame.source,
            Selection::You => self.player(frame.controller).object,
            other => todo!("stage 2 does not evaluate selection {other:?}"),
        }
    }

    /// Evaluate a `Quantity` to a concrete number.
    ///
    /// # Panics
    ///
    /// Panics on a `Quantity` not wired for Stage 2.
    #[expect(
        clippy::unused_self,
        reason = "future Quantity arms (X, StatOf, …) will read self"
    )]
    fn eval_quantity(&self, qty: &Quantity, _frame: &Frame) -> Uint {
        match qty {
            Quantity::Literal(n) => *n,
            other => todo!("stage 2 does not evaluate quantity {other:?}"),
        }
    }

    /// True iff the card's printed types include a permanent type
    /// (Creature/Artifact/Enchantment/Land/Planeswalker/Battle) and NOT
    /// Instant or Sorcery.
    ///
    /// [CR#110.1]: a permanent spell is one that would enter the battlefield on
    /// resolution. Grizzly Bears → true; Lightning Bolt → false.
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

/// Extracts the `Filter` from a `TargetSpec`. Stage 2 only handles
/// `TargetSpec::Target(filter)` (and `Expanded` wrappers around it).
///
/// This is the single authoritative site for TargetSpec→Filter extraction;
/// both `cast::legal_targets` (announce time) and `targets_still_legal`
/// (resolution time) funnel through here so they stay in sync.
///
/// # Panics
///
/// Panics on `TargetSpec` variants not wired for Stage 2.
pub(crate) fn target_spec_filter(spec: &TargetSpec) -> &deckmaste_core::Filter {
    match spec {
        TargetSpec::Target(f) => f,
        TargetSpec::Expanded(e) => target_spec_filter(&e.value),
        other => todo!("stage 2 does not handle target spec {other:?}"),
    }
}
