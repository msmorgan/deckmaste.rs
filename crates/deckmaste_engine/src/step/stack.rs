//! `EventApply` handlers (and, for the two bare-`ObjectId` variants with no
//! dedicated payload struct, plain dispatch functions) for the stack-flavored
//! events: copying, activating, casting, countering, and resolving.

use deckmaste_core::Uint;
use deckmaste_core::Zone;

use crate::event::AbilityActivated;
use crate::event::AbilityCountered;
use crate::event::AbilityUsed;
use crate::event::Copied;
use crate::event::GameEvent;
use crate::event::ManaAbilityActivated;
use crate::object::ObjectId;
use crate::stack::StackEntry;
use crate::stack::StackObject;
use crate::state::GameState;
use crate::step::EventApply;

impl EventApply for Copied {
    // [CR#707.10]: put a copy of the referenced stack object onto
    // the stack — "a copy of a spell is itself a spell", so a spell
    // copy mints a fresh backing object (its own `StackObject::Spell`
    // identity); an ability copy has no card behind it, so it keeps
    // the SAME `StackObject::{Triggered,Activated}` source as the
    // original ([CR#707.10b]) and only its `StackEntry.id` is fresh.
    // All cast decisions ride the clone (targets, X, paid costs).
    fn apply(&self, g: &mut GameState) -> Option<GameEvent> {
        let original = self.original;
        let controller = self.controller;
        let Some(entry) = g.stack.iter().find(|e| e.id == original).cloned() else {
            // The original vanished before this applied — fizzle
            // (semantic-input errors never crash the engine).
            return None;
        };
        let source = match &entry.object {
            // Safe: a live Spell entry's object id IS the entry's own id, and entry/object
            // removal is atomic — a live entry's Spell object can't be dead (contrast the
            // Activated arm below, whose SEPARATE source id can go stale).
            StackObject::Spell(obj) => g.objects.obj(*obj).source,
            StackObject::Triggered { source, .. } => *source,
            StackObject::Activated { source, .. } => {
                // The activated ability's source is carried by id
                // only and may be "possibly gone, possibly changed"
                // (`StackObject::Activated` doc) — a zone change
                // removes it from the store ([CR#400.7]). A stale id
                // here is a semantic-input-adjacent runtime state, not a
                // reason to crash: fizzle (the Invalid semantic input fizzles
                // decision, `docs/decisions/invalid-semantic-input-fizzles.md`).
                g.objects.get(*source)?.source
            }
        };
        let new = g.objects.mint(source, controller, Some(Zone::Stack));
        // [CR#700.2g]: a copy keeps the original spell or ability's announced
        // modes; its controller cannot choose again. Targets, X, and paid
        // optional costs are the other announcement-time choices cloned here.
        let copied = StackEntry {
            id: new,
            object: match &entry.object {
                StackObject::Spell(_) => StackObject::Spell(new),
                other => other.clone(),
            },
            controller,
            targets: entry.targets.clone(),
            chosen_modes: entry.chosen_modes.clone(),
            x: entry.x,
            paid_costs: entry.paid_costs.clone(),
            copy: true,
        };
        g.stack.push(copied);
        // The minted id rides the recorded event so history (and any
        // reader of the occurrence) can see what got created.
        Some(GameEvent::Copied(Copied {
            original,
            copy: Some(new),
            controller,
        }))
    }
}

impl EventApply for AbilityActivated {
    // [CR#602.2a]: promote the staged activation onto the stack
    // under the stack identity minted when the announce opened
    // ([CR#405], `begin_activate`).
    fn apply(&self, g: &mut GameState) -> Option<GameEvent> {
        let source = self.source;
        let ability = self.ability;
        let pending = g.promote_announce();
        debug_assert!(
            matches!(
                &pending.object,
                StackObject::Activated { source: s, .. } if *s == source
            ),
            "AbilityActivated event matches the staged announce"
        );
        // Record the substantive "this ability was used" fact directly
        // so history reads (use-limit counts, EventCount) can find it.
        // Not routed through the occurrence pipeline — must not trigger
        // anything and must not be re-recorded ([CR#608.2i]).
        //
        // Use the announce-time LKI snapshot rather than the live
        // object — the source may have been removed (e.g. self-sacrifice
        // cost) before this event applies ([CR#602.2a]).
        // `begin_activate` always captures `bindings.this`, so the
        // expect below should never fire in practice.
        let used_object = match &pending.object {
            StackObject::Activated { bindings, .. } => {
                bindings
                    .this
                    .as_ref()
                    .expect("begin_activate always captures a this snapshot")
                    .object
            }
            _ => panic!("AbilityActivated with non-Activated stack object"),
        };
        g.record_history_fact(
            g.turn.turn_number,
            None,
            GameEvent::AbilityUsed(AbilityUsed {
                object: used_object,
                ability: Uint::try_from(ability).expect("ability index fits in Uint"),
            }),
        );
        None
    }
}

impl EventApply for ManaAbilityActivated {
    fn apply(&self, g: &mut GameState) -> Option<GameEvent> {
        g.record_history_fact(
            g.turn.turn_number,
            None,
            GameEvent::AbilityUsed(AbilityUsed {
                object: self.source.object,
                ability: Uint::try_from(self.ability).expect("ability index fits in Uint"),
            }),
        );
        None
    }
}

impl EventApply for AbilityCountered {
    // [CR#608.2n,701.6a,707.10a]: the triggered/activated ability —
    // or a countered/resolved COPY of a spell or ability — vanishes:
    // remove its stack entry and discard the (minted, for a spell
    // copy freshly minted) backing object. No zone move; the source
    // (already gone for a dies-trigger) is untouched.
    fn apply(&self, g: &mut GameState) -> Option<GameEvent> {
        g.remove_stack_entry(self.id);
        g.objects.remove(self.id);
        None
    }
}

/// `SpellCast` carries a bare `ObjectId` — no dedicated payload struct
/// (Tasks 3.1–3.2 structified only multi-field variants) — so it dispatches
/// through a plain function rather than `EventApply`.
///
/// [CR#601.2i]: promote the staged announce onto the stack. [CR#405]: a
/// spell's stack identity is its own object id — unchanged from Stage 2, so
/// existing `Resolve(spell)` keying by `StackEntry.id` still finds it.
pub(crate) fn handle_spell_cast(g: &mut GameState, object: ObjectId) -> Option<GameEvent> {
    let pending = g.promote_announce();
    debug_assert_eq!(
        pending.object.object(),
        object,
        "SpellCast event matches the staged announce"
    );
    None
}

/// `AbilityResolved` carries a bare `ObjectId` — see `handle_spell_cast`.
///
/// [CR#608.2n]: a triggered or activated ability finished resolving and
/// vanishes — no zone move. [CR#707.10a]: a RESOLVED copy (of a spell)
/// vanishes the same way instead of moving to a graveyard — it has no card
/// to put there. Removes the stack entry whose `id` is the carried (minted)
/// token, and the backing object with it.
pub(crate) fn handle_ability_resolved(g: &mut GameState, id: ObjectId) -> Option<GameEvent> {
    g.remove_stack_entry(id);
    g.objects.remove(id);
    None
}
