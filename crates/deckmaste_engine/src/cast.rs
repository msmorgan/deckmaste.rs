//! Casting ([CR#601]): the mana-payment solver plus the reified announce flow
//! (`begin_cast` → `announce_targets` → `pay_cost`), and the `can_cast`
//! legality gate that `legal::legal_actions` offers from. The announce flow
//! (`announce_targets` / `pay_cost`) is shared with activated abilities
//! ([CR#602.2b]); see `activate.rs` for the activation entry point.

use deckmaste_core::Agency;
use deckmaste_core::ColorOrColorless;
use deckmaste_core::ManaCost;
use deckmaste_core::ManaSymbol;
use deckmaste_core::SimpleManaSymbol;
use deckmaste_core::TargetSpec;
use deckmaste_core::Type;
use deckmaste_core::Uint;
use deckmaste_core::Window;
use deckmaste_core::Zone;

use crate::agenda::WorkItem;
use crate::decide::PendingDecision;
use crate::event::Cause;
use crate::event::GameEvent;
use crate::event::Occurrence;
use crate::object::ObjectId;
use crate::player::ManaPool;
use crate::player::PlayerId;
use crate::stack::PendingStackEntry;
use crate::stack::StackObject;
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
    use ColorOrColorless::Color;
    use ColorOrColorless::Colorless;
    use deckmaste_core::Color::Black;
    use deckmaste_core::Color::Blue;
    use deckmaste_core::Color::Green;
    use deckmaste_core::Color::Red;
    use deckmaste_core::Color::White;
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
    /// [CR#601.3,601.2g]: may `player` cast `object` now? Offered iff the
    /// object is in the holder's hand (the caller iterates the hand), the
    /// object is not a land ([CR#305.9]), timing permits (instant → any
    /// priority; otherwise sorcery-speed), the pool can pay the cost, and
    /// every target spec has at least one legal candidate.
    #[must_use]
    pub(crate) fn can_cast(
        &self,
        view: &crate::layer::LayeredView,
        player: PlayerId,
        object: ObjectId,
    ) -> bool {
        let face = crate::derive::face(self.def(object));
        // Lands are never cast as spells — playing a land is a special action
        // ([CR#305.9,116.2a]).
        if face.types.contains(&Type::Land) {
            return false;
        }
        let instant = face.types.contains(&Type::Instant);
        // Sorcery speed for non-instants ([CR#307.1,117.1a]), unless a
        // May(Cast(window: InstantSpeed)) row lifts the default
        // ([CR#702.8a] flash — the card's own row functions from the
        // hand; an Orrery-style battlefield grant rides the same shape).
        // Rows carrying `from`/`cost` slots are different unlocks
        // (cast-from-zones, alternative costs) and never lift timing.
        let proxy = self.player(player).object;
        let timing_ok = instant
            || self.sorcery_speed_ok(player)
            || crate::legal::may_cast_rows(self, view, object)
                .iter()
                .any(|r| {
                    r.window == Some(Window::InstantSpeed)
                        && r.from.is_none()
                        && r.cost.is_none()
                        && self.filter_matches_live(&r.what, object, r.carrier)
                        && self.filter_matches_live(&r.by, proxy, r.carrier)
                });
        if !timing_ok {
            return false;
        }
        // [CR#118.6]: an EMPTY mana cost is "no mana cost" — an unpayable
        // base. Attempting the cast is legal in the CR but pointless to
        // offer; an alternative cost ([CR#118.6a], May(Cast(cost: …)) rows)
        // is the future unlock. {0} is spelled [Generic(0)] and payable
        // ([CR#118.5]).
        if face.mana_cost.is_empty() {
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
            // [CR#405]: a spell's stack identity is its own object id.
            id: object,
            object: StackObject::Spell(object),
            controller,
            origin: Zone::Hand,
            targets: vec![],
        });
    }

    /// [CR#601.2c]: surface a `ChooseTargets` decision if the in-flight
    /// announce targets. A spell's specs derive from its `Spell` ability; an
    /// activated ability's ride the carried text ([CR#602.2b]). Returns the
    /// number of target specs (0 = no decision surfaced).
    ///
    /// # Panics
    ///
    /// Panics if no announce is in flight, if a `Triggered` object occupies
    /// the slot (triggers announce targets at placement, [CR#603.3d]), or if
    /// the spec count overflows `Uint` — engine invariants.
    #[must_use]
    pub(crate) fn announce_targets(&mut self) -> Uint {
        let pending = self.announcing.as_ref().expect("an announce in flight");
        let controller = pending.controller;
        let specs: Vec<TargetSpec> = match &pending.object {
            StackObject::Spell(o) => crate::resolve::spell_targets(&self.layers(), *o),
            // The carried ability text is authoritative — never re-derive
            // from the (possibly changed) source.
            StackObject::Activated { ability, .. } => ability.targets.clone(),
            StackObject::Triggered { .. } => {
                unreachable!("triggers announce targets at placement, not in the announce slot")
            }
        };
        if specs.is_empty() {
            return 0;
        }
        // The Cant(Target) filtering (hexproof, protection) and the
        // `ChooseTargets` construction live in `surface_target_choice`, shared
        // with trigger placement ([CR#603.3d]). `by` evaluates against the
        // announce's stack identity — a spell's own id, or the ability
        // identity minted when the announce opened ([CR#602.2a]) — so
        // stack-zone-keyed rows read the real object.
        let spell = pending.id;
        self.surface_target_choice(controller, specs, spell)
    }

    /// [CR#601.2c]: surface a `ChooseTargets` decision for `player` over
    /// `specs`, computing each spec's legal candidates with the `Cant(Target)`
    /// carriers ([CR#702.11b] hexproof, [CR#702.16b] protection's targeted
    /// clause) excluded. `targeting_id` is the live stack identity each
    /// forbidding row's `by` filter evaluates against — a spell's own id / an
    /// ability announce's minted id ([CR#602.2a]), or a placing trigger's
    /// freshly minted stack id ([CR#603.3d]); it must be a real object, since
    /// `by` reads the targeting object's controller (hexproof's "abilities
    /// your opponents control").
    ///
    /// Returns the spec count. The surfaced decision carries the per-spec
    /// legal sets; a caller that must drop on an empty set (a targeting
    /// trigger, [CR#603.3c]) inspects them off `self.pending`.
    ///
    /// # Panics
    ///
    /// Panics if the spec count overflows `Uint` — an engine invariant.
    #[must_use]
    pub(crate) fn surface_target_choice(
        &mut self,
        player: PlayerId,
        specs: Vec<TargetSpec>,
        targeting_id: ObjectId,
    ) -> Uint {
        let view = self.layers();
        let rows = crate::legal::cant_target_rows(self, &view);
        let legal: Vec<Vec<ObjectId>> = specs
            .iter()
            .map(|s| {
                self.legal_targets(s)
                    .into_iter()
                    .filter(|&t| {
                        crate::legal::target_forbidden_by(self, &rows, targeting_id, t).is_none()
                    })
                    .collect()
            })
            .collect();
        let count = Uint::try_from(specs.len()).expect("target-spec count fits in Uint");
        self.pending = Some(PendingDecision::ChooseTargets {
            player,
            spec: specs,
            legal,
        });
        count
    }

    /// [CR#601.2f,601.2g,601.2h]: pay the in-flight cost. Always surfaces a `PayMana`
    /// decision for any non-empty mana cost; the core never auto-pays.
    /// Auto-resolution (an Arena-style autotapper) is a future runner concern.
    /// For an activated ability ([CR#602.2b]) the cost's {T}/{Q} components
    /// are scheduled as events alongside the mana decision.
    ///
    /// # Panics
    ///
    /// Panics if no announce is in flight, the spell has no payable cost
    /// (`can_cast` gated it), the ability cost has an unpayable component
    /// (`can_activate` gated it), or a `Triggered` object occupies the slot.
    pub(crate) fn pay_cost(&mut self) {
        let pending = self.announcing.as_ref().expect("an announce in flight");
        let controller = pending.controller;
        match &pending.object {
            StackObject::Spell(o) => {
                let object = *o;
                let cost = self.mana_cost(object).expect("a castable spell has a cost");
                if !cost.is_empty() {
                    let pool = self.player(controller).mana_pool.clone();
                    self.pending = Some(PendingDecision::PayMana {
                        player: controller,
                        cost,
                        pool,
                    });
                }
                // Empty cost (no mana required): no decision surfaces, cast
                // continues.
            }
            StackObject::Activated {
                source, ability, ..
            } => {
                let source = *source;
                let summary = crate::activate::cost_summary(&ability.cost)
                    .expect("can_activate vetted the cost");
                // Costs are paid at [CR#601.2h,602.2b]: schedule the {T}/{Q}
                // events at the agenda FRONT — they sit behind the pending
                // mana decision (if any) and apply when it is answered.
                let mut events: Vec<WorkItem> = Vec::new();
                if summary.tap {
                    events.push(WorkItem::Emit(Occurrence::single(GameEvent::Tapped {
                        object: source,
                        cause: Some(Cause {
                            verb: "Tap".into(),
                            agency: Agency::CostPayment,
                            agent: Some((source, controller)),
                        }),
                    })));
                }
                if summary.untap {
                    events.push(WorkItem::Emit(Occurrence::single(GameEvent::Untapped(
                        source,
                    ))));
                }
                if !events.is_empty() {
                    self.schedule_front(events);
                }
                if !summary.mana.is_empty() {
                    let pool = self.player(controller).mana_pool.clone();
                    self.pending = Some(PendingDecision::PayMana {
                        player: controller,
                        cost: summary.mana,
                        pool,
                    });
                }
            }
            StackObject::Triggered { .. } => {
                unreachable!("a triggered ability has no cost and never occupies the announce slot")
            }
        }
    }

    /// The card face's printed mana cost ([CR#202]). `None` would mark an
    /// uncastable object; every card face carries a (possibly empty) cost, so
    /// this is always `Some` today — the option leaves room for future
    /// faces/zones that have no castable cost (and lets `can_cast`/`pay_cost`
    /// share the `let Some(cost) = …` gate).
    #[must_use]
    #[allow(
        clippy::unnecessary_wraps,
        reason = "the Option is the cast-legality seam; future no-cost faces return None (now a pub API, so clippy may not fire — keep the seam documented)"
    )]
    pub fn mana_cost(&self, object: ObjectId) -> Option<ManaCost> {
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
