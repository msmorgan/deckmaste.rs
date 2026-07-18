//! `EventApply` handler for `TriggerFired`: notes a fired trigger into the
//! pending-trigger queue. Inert until `PlaceTriggers` places it on the
//! stack.

use crate::event::AbilityUsed;
use crate::event::GameEvent;
use crate::event::TriggerFired;
use crate::state::GameState;
use crate::step::EventApply;

impl EventApply for TriggerFired {
    // [CR#603.2]: applying a `TriggerFired` *notes* the trigger. It is
    // inert until the `PlaceTriggers` barrier (a later task) puts it on
    // the stack. Nothing else happens here.
    fn apply(&self, g: &mut GameState) -> Option<GameEvent> {
        let source = self.source;
        let ability = self.ability;
        let controller = self.controller;
        // A delayed/reflexive ([CR#603.7,603.12]) trigger carries its
        // body by value: it has no printed index, so the use-limit gate
        // and `AbilityUsed` history (both keyed by `object`+printed
        // index) do not apply — its firing is governed by the registry
        // (delayed) or the immediate look-back (reflexive). [CR#603.7h]
        // — a delayed ability limited to "the Nth resolution this turn"
        // — is not modeled. Note it and move on.
        if let Some(created) = &self.created {
            g.pending_triggers.push(crate::trigger::NotedTrigger {
                source,
                ability: 0,
                created: Some(created.clone()),
                controller,
                bindings: self.bindings.as_ref().clone(),
            });
            return None;
        }
        // [CR#603.2h]: a "once each turn" / once-per-game triggered
        // ability is noted at most that often. The gate lives HERE, at
        // note time — NOT at scan-emit — because a single
        // multi-occurrence event ([CR#603.2c], e.g. two creatures
        // dying simultaneously) emits both `TriggerFired`s in one scan
        // pass before either applies; only at sequential apply-time
        // does the second see the first's recorded `AbilityUsed`. The
        // limit is per firing object ([CR#400.7]).
        let obj = self.bindings.this.as_ref().map(|t| t.object);
        // Collect the firing ability's limits into an owned vec BEFORE
        // the `pending_triggers`/`history` mutations below (the
        // `abilities_of_source` borrow must not overlap them).
        let limits: Vec<deckmaste_core::UseLimit> =
            match crate::derive::abilities_of_source(g, source).get(ability as usize) {
                Some(deckmaste_core::Ability::Triggered(t)) => t.limits.clone(),
                _ => Vec::new(),
            };
        if let Some(obj) = obj {
            for limit in &limits {
                let window = match limit {
                    deckmaste_core::UseLimit::OncePerTurn => deckmaste_core::Lookback::ThisTurn,
                    deckmaste_core::UseLimit::OncePerGame => deckmaste_core::Lookback::ThisGame,
                    // `LoyaltyOncePerTurn` ([CR#606.3]) is an
                    // ACTIVATED-ability limit (it names loyalty
                    // abilities SHARED across a permanent, gated in
                    // `GameState::can_activate`) — no authoring path
                    // puts it on a `TriggeredAbility`. Exhaustiveness
                    // only: fall back to the per-ability `ThisTurn`
                    // window `OncePerTurn` uses, rather than panicking
                    // on a malformed card.
                    deckmaste_core::UseLimit::LoyaltyOncePerTurn => {
                        deckmaste_core::Lookback::ThisTurn
                    }
                };
                if g.ability_used_count(obj, ability, window) >= 1 {
                    // The limit is spent: the trigger does NOT fire —
                    // note nothing, record nothing.
                    return None;
                }
            }
        }
        g.pending_triggers.push(crate::trigger::NotedTrigger {
            source,
            ability: ability as usize,
            created: None,
            controller,
            bindings: self.bindings.as_ref().clone(),
        });
        // Record the substantive "this ability was used" fact directly
        // so history reads (use-limit counts, EventCount) can find it.
        // Not routed through the occurrence pipeline — must not trigger
        // anything and must not be re-recorded ([CR#608.2i]).
        if let Some(this) = &self.bindings.this {
            let used = GameEvent::AbilityUsed(AbilityUsed {
                object: this.object,
                ability,
            });
            g.record_history_fact(g.turn.turn_number, None, used);
        }
        None
    }
}
