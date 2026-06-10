//! Casting ([CR#601]): the mana-payment solver plus the reified announce flow
//! (`begin_cast` → `announce_targets` → `pay_cost`), and the `can_cast`
//! legality gate that `legal::legal_actions` offers from.

use deckmaste_core::{
    ColorOrColorless, ManaCost, ManaSymbol, SimpleManaSymbol, TargetSpec, Type, Uint, Zone,
};

use crate::decide::PendingDecision;
use crate::object::ObjectId;
use crate::player::{ManaPool, PlayerId};
use crate::stack::{PendingStackEntry, StackObject};
use crate::state::GameState;
use crate::target::candidates;

/// How the pool's mana is spent on a cost's generic part: one entry per unit
/// of generic mana owed (colored pips are forced by color, so they need no
/// choice). Empty when the cost is all-colored.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Payment {
    pub generic: Vec<ColorOrColorless>,
}

/// The colored requirement (per color) and total generic of a cost, or `None`
/// if the cost uses an out-of-scope symbol (X, hybrid, phyrexian, snow).
fn requirement(cost: &ManaCost) -> Option<(Vec<(ColorOrColorless, Uint)>, Uint)> {
    let mut colored: Vec<(ColorOrColorless, Uint)> = Vec::new();
    let mut generic: Uint = 0;
    for symbol in cost.iter() {
        match symbol {
            ManaSymbol::Simple(SimpleManaSymbol::Generic(n)) => generic += n,
            ManaSymbol::Simple(SimpleManaSymbol::Specific(c)) => {
                match colored.iter_mut().find(|(k, _)| k == c) {
                    Some((_, v)) => *v += 1,
                    None => colored.push((*c, 1)),
                }
            }
            // X/Hybrid/Phyrexian/Snow are out of scope this stage.
            _ => return None,
        }
    }
    Some((colored, generic))
}

/// Whether `pool` can pay `cost` ([CR#601.2g]). Colored pips must be covered by
/// their color; the remaining mana must cover the generic.
///
/// Returns `false` for costs containing out-of-scope symbols (X, hybrid,
/// phyrexian, snow).
#[must_use]
pub fn can_pay(pool: &ManaPool, cost: &ManaCost) -> bool {
    let Some((colored, generic)) = requirement(cost) else { return false };
    let mut leftover: Uint = 0;
    // Sum leftover across all six kinds after colored deductions.
    for (kind, have) in pool_kinds(pool) {
        let need = colored
            .iter()
            .find(|(k, _)| *k == kind)
            .map_or(0, |(_, n)| *n);
        if have < need {
            return false;
        }
        leftover += have - need;
    }
    leftover >= generic
}

/// Whether `payment` legally covers `cost` from `pool`.
#[must_use]
pub fn validate_payment(pool: &ManaPool, cost: &ManaCost, payment: &Payment) -> bool {
    let Some((colored, generic)) = requirement(cost) else { return false };
    if payment.generic.len() != generic as usize {
        return false;
    }
    // Total spend per kind = colored need + generic chosen of that kind.
    for (kind, have) in pool_kinds(pool) {
        let colored_need = colored
            .iter()
            .find(|(k, _)| *k == kind)
            .map_or(0, |(_, n)| *n);
        let generic_need =
            u32::try_from(payment.generic.iter().filter(|&&c| c == kind).count()).unwrap();
        if have < colored_need + generic_need {
            return false;
        }
    }
    true
}

/// Deducts a validated `payment` from `pool` ([CR#601.2g,106.4]).
///
/// # Panics
///
/// Panics if the cost contains out-of-scope symbols, or if the pool does not
/// cover the payment (callers must validate first).
pub fn apply_payment(pool: &mut ManaPool, cost: &ManaCost, payment: &Payment) {
    let (colored, _) = requirement(cost).expect("validated cost");
    for (kind, n) in colored {
        pool.spend(kind, n);
    }
    for kind in &payment.generic {
        pool.spend(*kind, 1);
    }
}

/// The pool as (kind, amount) pairs over all six kinds.
fn pool_kinds(pool: &ManaPool) -> [(ColorOrColorless, Uint); 6] {
    use ColorOrColorless::{Color, Colorless};
    use deckmaste_core::Color::{Black, Blue, Green, Red, White};
    [
        Colorless,
        Color(White),
        Color(Blue),
        Color(Black),
        Color(Red),
        Color(Green),
    ]
    .map(|k| (k, pool.amount(k)))
}

impl GameState {
    /// [CR#601.3a,601.2g]: may `player` cast `object` now? Offered iff the
    /// object is in the holder's hand (the caller iterates the hand), timing
    /// permits (instant → any priority; otherwise sorcery-speed), the pool can
    /// pay the cost, and every target spec has at least one legal candidate.
    #[must_use]
    pub(crate) fn can_cast(
        &self,
        view: &crate::layer::LayeredView,
        player: PlayerId,
        object: ObjectId,
        in_main: bool,
        stack_empty: bool,
    ) -> bool {
        let face = crate::derive::face(self.def(object));
        let instant = face.types.contains(&Type::Instant);
        // Sorcery speed for non-instants ([CR#307.1,601.3a]): only the active
        // player, in a main phase, with the stack empty.
        let timing_ok = instant || (player == self.turn.active_player && in_main && stack_empty);
        if !timing_ok {
            return false;
        }
        let Some(cost) = self.mana_cost(object) else {
            return false;
        };
        if !can_pay(&self.player(player).mana_pool, &cost) {
            return false;
        }
        // If the spell targets, every spec must admit at least one candidate.
        crate::resolve::spell_targets(view, object)
            .iter()
            .all(|spec| !self.legal_targets(spec).is_empty())
    }

    /// [CR#601.2a,601.2b]: move the spell from its controller's hand to the stack and
    /// open the announce slot. Procedural — not an event.
    ///
    /// # Panics
    ///
    /// Panics if `object` is not in its controller's hand — engine invariant.
    pub(crate) fn begin_cast(&mut self, object: ObjectId) {
        let controller = self.objects.obj(object).controller;
        self.remove_from_hand(controller, object);
        self.objects.obj_mut(object).zone = Some(Zone::Stack);
        self.announcing = Some(PendingStackEntry {
            object: StackObject::Spell(object),
            controller,
            origin: Zone::Hand,
            targets: vec![],
        });
    }

    /// [CR#601.2c]: surface a `ChooseTargets` decision if the in-flight spell
    /// targets. Returns the number of target specs (0 = no decision surfaced).
    ///
    /// # Panics
    ///
    /// Panics if no announce is in flight, or if the spec count overflows
    /// `Uint` — engine invariants.
    #[must_use]
    pub(crate) fn announce_targets(&mut self) -> Uint {
        let pending = self.announcing.as_ref().expect("an announce in flight");
        let object = pending.object.object();
        let controller = pending.controller;
        let specs = crate::resolve::spell_targets(&self.layers(), object);
        if specs.is_empty() {
            return 0;
        }
        let legal: Vec<Vec<ObjectId>> = specs.iter().map(|s| self.legal_targets(s)).collect();
        let count = Uint::try_from(specs.len()).expect("target-spec count fits in Uint");
        self.pending = Some(PendingDecision::ChooseTargets {
            player: controller,
            spec: specs,
            legal,
        });
        count
    }

    /// [CR#601.2f,601.2g,601.2h]: pay the in-flight cost. Always surfaces a `PayMana`
    /// decision for any non-empty mana cost; the core never auto-pays.
    /// Auto-resolution (an Arena-style autotapper) is a future runner concern.
    ///
    /// # Panics
    ///
    /// Panics if no announce is in flight, or the spell has no payable cost —
    /// `can_cast` gated both.
    pub(crate) fn pay_cost(&mut self) {
        let pending = self.announcing.as_ref().expect("an announce in flight");
        let object = pending.object.object();
        let controller = pending.controller;
        let cost = self.mana_cost(object).expect("a castable spell has a cost");
        if !cost.is_empty() {
            let pool = self.player(controller).mana_pool.clone();
            self.pending = Some(PendingDecision::PayMana {
                player: controller,
                cost,
                pool,
            });
        }
        // Empty cost (no mana required): no decision surfaces, cast continues.
    }

    /// The card face's printed mana cost ([CR#202]). `None` would mark an
    /// uncastable object; every card face carries a (possibly empty) cost, so
    /// this is always `Some` today — the option leaves room for future
    /// faces/zones that have no castable cost (and lets `can_cast`/`pay_cost`
    /// share the `let Some(cost) = …` gate).
    #[must_use]
    #[expect(
        clippy::unnecessary_wraps,
        reason = "the Option is the cast-legality seam; future no-cost faces return None"
    )]
    pub(crate) fn mana_cost(&self, object: ObjectId) -> Option<ManaCost> {
        Some(crate::derive::face(self.def(object)).mana_cost.clone())
    }

    /// [CR#115]: the legal candidates for a single `TargetSpec` (its filter's
    /// matching objects, in id order).
    ///
    /// Delegates filter extraction to `resolve::target_spec_filter` so that
    /// announce-time and resolution-time `TargetSpec` handling stay in sync.
    ///
    /// # Panics
    ///
    /// Panics on `TargetSpec` variants other than `Target` or `Expanded` —
    /// only those are wired for Stage 2.
    #[must_use]
    pub(crate) fn legal_targets(&self, spec: &TargetSpec) -> Vec<ObjectId> {
        let filter = crate::resolve::target_spec_filter(spec);
        candidates(self, filter)
    }
}

#[cfg(test)]
mod tests {
    use deckmaste_core::Color;

    use super::*;

    fn pool(pairs: &[(ColorOrColorless, Uint)]) -> ManaPool {
        let mut p = ManaPool::default();
        for &(m, n) in pairs {
            p.add(m, n);
        }
        p
    }
    fn cost(s: &str) -> ManaCost { s.parse().unwrap() }
    fn red() -> ColorOrColorless { Color::Red.into() }
    fn green() -> ColorOrColorless { Color::Green.into() }

    #[test]
    fn colored_pip_needs_its_color() {
        assert!(can_pay(&pool(&[(red(), 1)]), &cost("{R}")));
        assert!(!can_pay(&pool(&[(green(), 1)]), &cost("{R}")));
        assert!(!can_pay(&ManaPool::default(), &cost("{R}")));
    }

    #[test]
    fn generic_pays_from_any_leftover() {
        assert!(can_pay(&pool(&[(green(), 2)]), &cost("{1}{G}"))); // G pays {G}, G pays {1}
        assert!(!can_pay(&pool(&[(green(), 1)]), &cost("{1}{G}"))); // nothing left for {1}
        assert!(can_pay(&pool(&[(green(), 1), (red(), 1)]), &cost("{1}{G}")));
    }

    #[test]
    fn validate_and_apply_round_trip() {
        let mut p = pool(&[(green(), 1), (red(), 1)]);
        let pay = Payment {
            generic: vec![red()],
        }; // {1}<-R, {G}<-G
        assert!(validate_payment(&p, &cost("{1}{G}"), &pay));
        apply_payment(&mut p, &cost("{1}{G}"), &pay);
        assert!(p.is_empty());
        // An allocation that double-spends a color it doesn't have is invalid.
        assert!(!validate_payment(
            &pool(&[(green(), 2)]),
            &cost("{1}{G}"),
            &Payment {
                generic: vec![red()]
            }
        ));
    }
}
