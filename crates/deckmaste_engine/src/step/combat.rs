//! `EventApply` handlers for the combat-flavored events: damage, attacking,
//! blocking, and damage removal.

use deckmaste_core::KeywordAbility;

use crate::agenda::WorkItem;
use crate::event::Attacking;
use crate::event::Blocked;
use crate::event::DamageDealt;
use crate::event::DamageRemoved;
use crate::event::GameEvent;
use crate::event::Occurrence;
use crate::event::Tapped;
use crate::object::ObjectSource;
use crate::state::GameState;
use crate::step::EventApply;

impl EventApply for DamageDealt {
    // [CR#120.3]: damage to a player is life loss; to a creature it is
    // marked damage. `Int` is `i32`; `Uint` is `u32` — `try_from`
    // is required because u32 does not fit into i32 via `From`.
    // One view for the mark's deal-time abilities and the lifelink
    // check below (built before the damage lands; neither depends
    // on the marked total).
    fn apply(&self, g: &mut GameState) -> Option<GameEvent> {
        let source = self.source;
        let target = self.target;
        let amount = self.amount;
        let view = g.layers();
        match g.objects.obj(target).source {
            ObjectSource::Player(p) => {
                // [CR#120.3a]: damage to a player is life loss.
                g.player_mut(p).life -=
                    deckmaste_core::Int::try_from(amount).expect("damage fits in i32");
            }
            ObjectSource::Card(_) => {
                // [CR#120.3]: damage to a permanent has "one or more
                // results" — the intrinsic creature result and every
                // matching data-driven counter-removal result are applied
                // IN ADDITION (a creature-planeswalker is marked AND loses
                // loyalty), no longer mutually exclusive.

                // [CR#120.3e,120.3c]: a COMBATANT (a permanent PLAYING the
                // role, not one that merely holds some combat
                // permission) has its damage marked, tagged with the
                // source's identity and abilities AS THEY ARE NOW — the
                // deal-time snapshot the lethal-damage SBA's deathtouch
                // clause reads ([CR#704.5h]), correct even if the source
                // later loses the ability or leaves; a stale (gone) source
                // contributes no abilities. A non-combatant permanent (a
                // plain planeswalker) is NOT marked; it loses loyalty
                // instead ([CR#120.3c]).
                if crate::legal::is_combatant(g, &view, target) {
                    let (src, abilities) = match g.objects.get(source) {
                        // Card-backed source: capture its identity and
                        // deal-time abilities from the layered view.
                        Some(o) if o.card_id().is_some() => {
                            (Some(o.source), view.get(source).abilities.as_ref().clone())
                        }
                        // A player proxy carries no abilities; a gone
                        // (reminted) source contributes none either.
                        Some(o) => (Some(o.source), Vec::new()),
                        None => (None, Vec::new()),
                    };
                    g.objects
                        .obj_mut(target)
                        .mark_damage(src, abilities, amount);
                }

                // [CR#120.3c,120.3h]: data-driven counter-removal results —
                // e.g. planeswalker loyalty (`rules/damage/`). Remove that
                // many counters of each matching rule's kind, clamped at 0
                // (the shared counter-removal path); the 0-counter SBA
                // ([CR#704.5i]) handles death. Collect matches first:
                // `matches` borrows `&self` while `remove_counters_clamped`
                // needs `&mut self`.
                let to_remove: Vec<deckmaste_core::Ident> = g
                    .damage_result_rules
                    .iter()
                    .filter(|rule| crate::matches(g, target, &rule.recipient))
                    .map(|rule| rule.remove.0)
                    .collect();
                for kind in to_remove {
                    g.remove_counters_clamped(target, &kind, amount);
                }
            }
        }
        // [CR#702.15]: if the source is a card-backed object with lifelink,
        // its controller gains life equal to the damage dealt. This applies
        // to combat damage and any other damage from a lifelink source.
        // Guard: use `get` (not `obj`) because a dies-trigger's source id
        // may be a stale (reminted) id that is no longer in the store.
        if g.objects.get(source).is_some_and(|o| o.card_id().is_some())
            && crate::combat::has_keyword_named(&view, source, "Lifelink")
        {
            let controller = g.objects.obj(source).controller;
            g.player_mut(controller).life +=
                deckmaste_core::Int::try_from(amount).expect("damage fits in i32");
        }
        // [CR#704.5h]: deathtouch is no longer a bespoke flag — it rides the
        // mark's captured deal-time abilities (recorded above), and the
        // lethal-damage SBA reads it via
        // `DealtDamageBy(subject, Has(Deathtouch))`.
        None
    }
}

impl EventApply for Attacking {
    // [CR#508.1a]: record the attacker; [CR#508.1f]: declaring it as an
    // attacker taps it (not a cost — attacking simply taps).
    // [CR#702.20]: a creature with vigilance is NOT tapped when it attacks.
    fn apply(&self, g: &mut GameState) -> Option<GameEvent> {
        let o = self.attacker;
        let defending = self.defending;
        g.combat.declare_attacker(o, defending);
        if !crate::combat::has_keyword(&g.layers(), o, &KeywordAbility::Vigilance)
            && !g.objects.obj(o).tapped
        {
            g.objects.obj_mut(o).tapped = true;
            // The declaration's tap is a real "becomes tapped"
            // transition ([CR#603.2e]), distinguishable by its cause
            // ([CR#508.1f] — not a cost): emit the fact in the
            // declaration's wake so becomes-tapped triggers see it.
            // Re-applying it is an idempotent flip.
            g.schedule_front(vec![WorkItem::Emit(Occurrence::single(GameEvent::Tapped(
                Tapped {
                    object: o,
                    cause: Some(crate::event::Cause::tap(
                        deckmaste_core::Agency::AttackDeclaration,
                        None,
                    )),
                },
            )))]);
        }
        None
    }
}

impl EventApply for Blocked {
    // [CR#509.1a]: record the block; [CR#509.1h]: the attacker becomes a
    // blocked creature (sticky). Declaring a blocker does NOT tap it. The
    // "becomes blocked" trigger seam matches on this fact.
    fn apply(&self, g: &mut GameState) -> Option<GameEvent> {
        g.combat.declare_block(self.blocker, self.attacker);
        None
    }
}

impl EventApply for DamageRemoved {
    // [CR#614.8,701.19a]: the regeneration heal clause — zero damage
    // and remove from combat. If the object is already at 0 damage
    // this is still a no-op (removal from combat is still correct —
    // the shield fired, so the permanent would have been in combat
    // when the destroy was imminent). `remove_object` is idempotent
    // for non-combat objects. Clearing the marks drops their deal-time
    // deathtouch provenance too ([CR#704.5h]) — a healed creature is no
    // longer "dealt damage by a deathtouch source", so a later SBA check
    // does not re-destroy it.
    fn apply(&self, g: &mut GameState) -> Option<GameEvent> {
        let object = self.object;
        if g.objects.get(object).is_some() {
            g.objects.obj_mut(object).clear_damage();
        }
        g.combat.remove_object(object);
        None
    }
}
