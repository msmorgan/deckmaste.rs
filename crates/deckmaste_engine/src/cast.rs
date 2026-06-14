//! Casting ([CR#601]): the mana-payment solver plus the reified announce flow
//! (`begin_cast` → `announce_x` → `announce_targets` → `pay_cost`), and the
//! `can_cast` legality gate that `legal::legal_actions` offers from. The
//! announce flow (`announce_targets` / `pay_cost`) is shared with activated
//! abilities ([CR#602.2b]); see `activate.rs` for the activation entry point.

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

/// The pool units spent on a cost ([CR#601.2g]): indices into the player's
/// mana pool at payment time. The decision is atomic, so indices into the
/// `PayMana` snapshot equal indices into the live pool.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Payment {
    pub units: Vec<usize>,
}

/// [CR#107.3a,107.3i]: substitute each `{X}` (`ManaSymbol::Variable`) in `cost`
/// with `Generic(x)` — all instances of X take the one announced value. A cost
/// with no `Variable` is returned unchanged, so callers may apply this
/// unconditionally.
#[must_use]
pub(crate) fn concretize_x(cost: &ManaCost, x: Uint) -> ManaCost {
    ManaCost::from(
        cost.iter()
            .map(|s| match s {
                ManaSymbol::Variable => ManaSymbol::Simple(SimpleManaSymbol::Generic(x)),
                other => *other,
            })
            .collect::<Vec<_>>(),
    )
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

/// Whether `payment`'s selected pool units legally cover `cost` from `pool`
/// ([CR#601.2g]).
///
/// The selected indices must be distinct and in range; the number of units
/// selected must equal the cost's mana value (colored need + generic); and each
/// color's colored requirement must be met by selected units of that color.
/// (Spendability/`SpendOnly` is not checked here yet — a later task.)
#[must_use]
pub fn validate_payment(pool: &ManaPool, cost: &ManaCost, payment: &Payment) -> bool {
    let Some((colored, generic)) = requirement(cost) else { return false };
    let units = pool.units();
    // Indices must be distinct and in range.
    let mut seen = std::collections::HashSet::with_capacity(payment.units.len());
    for &i in &payment.units {
        if i >= units.len() || !seen.insert(i) {
            return false;
        }
    }
    let colored_total: Uint = colored.iter().map(|(_, n)| *n).sum();
    // Exactly the cost's mana value: no under- or over-spend.
    if payment.units.len() != (colored_total + generic) as usize {
        return false;
    }
    let selected = || payment.units.iter().map(|&i| &units[i]);
    // Each color's colored pips must be covered by selected units of that color.
    for (c, n) in colored {
        if selected().filter(|u| u.kind == c).count() < n as usize {
            return false;
        }
    }
    true
}

/// Deducts a validated `payment`'s selected units from `pool`
/// ([CR#601.2g,106.4]). Callers must `validate_payment` first; out-of-range
/// indices are silently ignored, so an unvalidated payment may under-spend.
///
/// Seam: a spent unit's `GrantOnSpend`/`TriggerOnSpend` riders ([CR#106.6]) are
/// dropped here, not fired — on-spend effects need a "mana spent on X" event +
/// delayed triggers (deferred). `SpendOnly`/`Persistent` are already honored
/// (at payment / at emptying), so removal here is correct for them.
pub fn apply_payment(pool: &mut ManaPool, payment: &Payment) { pool.remove_units(&payment.units); }

/// Canonical auto-tap ([CR#601.2g], a runner/test convenience — the engine
/// surfaces the choice, this answers it): pick pool unit indices covering
/// `cost` (colored pips to matching-color units, generic pips to any
/// remaining). Caller ensures `can_pay` first.
///
/// Ignores spendability (`SpendOnly`): every unit is eligible. Use
/// [`auto_pay_spendable`] (via [`GameState::auto_pay_pending`]) to honor a
/// subject's spend restrictions.
///
/// # Panics
///
/// Panics if `cost` is out of scope or `pool` cannot cover it (call `can_pay`
/// first).
#[must_use]
#[allow(
    dead_code,
    reason = "public subject-free auto-tap helper; engine paths now route through auto_pay_pending (spendability-aware), but this stays the canonical pure form for runners/tests"
)]
pub fn auto_pay(pool: &ManaPool, cost: &ManaCost) -> Payment {
    auto_pay_spendable(pool, cost, &vec![true; pool.units().len()])
}

/// Like [`auto_pay`], but only units `i` with `spendable[i] == true` are
/// eligible — `take` skips the rest ([CR#106.6]). `spendable` must index the
/// same `pool` (length `== pool.units().len()`).
///
/// # Panics
///
/// Panics if `cost` is out of scope, or if the spendable units cannot cover it
/// (call `can_pay` over the spendable sub-pool first).
#[must_use]
pub fn auto_pay_spendable(pool: &ManaPool, cost: &ManaCost, spendable: &[bool]) -> Payment {
    let (colored, generic) = requirement(cost).expect("auto_pay on a payable cost");
    let units = pool.units();
    let mut used = vec![false; units.len()];
    let mut chosen = Vec::new();
    let mut take = |kind: Option<ColorOrColorless>, chosen: &mut Vec<usize>| {
        let i = units
            .iter()
            .enumerate()
            .position(|(i, u)| spendable[i] && !used[i] && kind.is_none_or(|k| u.kind == k))
            .expect("can_pay over the spendable sub-pool guarantees a covering unit");
        used[i] = true;
        chosen.push(i);
    };
    for (c, n) in colored {
        for _ in 0..n {
            take(Some(c), &mut chosen);
        }
    }
    for _ in 0..generic {
        take(None, &mut chosen);
    }
    Payment { units: chosen }
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
        // [CR#107.3a]: an {X} cost's floor is X=0; concretize at 0 so an
        // {X} spell is offered whenever its non-X part is affordable. The real
        // value is announced at the AnnounceX step ([CR#601.2b]).
        if !can_pay(
            &self.spendable_pool(player, object),
            &concretize_x(&cost, 0),
        ) {
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
            x: None,
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

    /// [CR#601.2b]: surface a `ChooseXValue` if the in-flight announce's cost has
    /// an `{X}` (`ManaSymbol::Variable`). Runs before `announce_targets`
    /// ([CR#601.2c]). No-op for an X-free cost, so the step is uniform.
    ///
    /// # Panics
    /// Panics if no announce is in flight, or a `Triggered` object occupies the
    /// slot — engine invariants.
    pub(crate) fn announce_x(&mut self) {
        let pending = self.announcing.as_ref().expect("an announce in flight");
        let controller = pending.controller;
        let has_x = match &pending.object {
            StackObject::Spell(o) => self
                .mana_cost(*o)
                .is_some_and(|c| c.iter().any(|s| matches!(s, ManaSymbol::Variable))),
            StackObject::Activated { ability, .. } => crate::activate::cost_summary(&ability.cost)
                .expect("can_activate vetted the cost")
                .mana
                .iter()
                .any(|s| matches!(s, ManaSymbol::Variable)),
            StackObject::Triggered { .. } => {
                unreachable!("a triggered ability never occupies the announce slot")
            }
        };
        if has_x {
            self.pending = Some(PendingDecision::ChooseXValue { player: controller });
        }
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
        let announced_x = pending.x.unwrap_or(0);
        match &pending.object {
            StackObject::Spell(o) => {
                let object = *o;
                let cost = concretize_x(
                    &self.mana_cost(object).expect("a castable spell has a cost"),
                    announced_x,
                );
                if !cost.is_empty() {
                    let pool = self.player(controller).mana_pool.clone();
                    self.pending = Some(PendingDecision::PayMana {
                        player: controller,
                        cost,
                        pool,
                        // [CR#106.6]: a spell's stack identity is its own id —
                        // the object SpendOnly riders judge.
                        subject: object,
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
                let mana = concretize_x(&summary.mana, announced_x);
                if !mana.is_empty() {
                    let pool = self.player(controller).mana_pool.clone();
                    self.pending = Some(PendingDecision::PayMana {
                        player: controller,
                        cost: mana,
                        pool,
                        // [CR#106.6]: an activated ability's mana is spent on
                        // its source — that is the object SpendOnly judges.
                        subject: source,
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

    /// Auto-tap the in-flight `PayMana` decision ([CR#601.2g,106.6]), honoring
    /// the subject's spend restrictions — only units spendable on the `PayMana`
    /// subject are eligible.
    ///
    /// # Panics
    ///
    /// Panics if the pending decision is not `PayMana`.
    #[must_use]
    pub fn auto_pay_pending(&self) -> Payment {
        match &self.pending {
            Some(PendingDecision::PayMana {
                cost,
                pool,
                subject,
                ..
            }) => {
                let mask: Vec<bool> = pool
                    .units()
                    .iter()
                    .map(|u| self.unit_spendable_on(u, *subject))
                    .collect();
                auto_pay_spendable(pool, cost, &mask)
            }
            other => panic!("auto_pay_pending called without a PayMana decision: {other:?}"),
        }
    }

    /// [CR#106.6]: may `unit` pay for `subject`? True unless a `SpendOnly`
    /// rider's filter rejects the object being paid for. Other rider kinds
    /// (`GrantOnSpend`/`TriggerOnSpend`/`Persistent`/`Expanded`) don't restrict
    /// spending.
    ///
    /// The watcher anchor is the subject's own `ObjectSource`: a `SpendOnly`
    /// filter today is object-shaped ("creature spell", "noncreature spell"),
    /// so it never reads the rider's grantor. A *relative* `SpendOnly` (a
    /// "your" reference back to the mana's producer — "spend only on a spell
    /// YOU cast") would need the producing source threaded onto the unit; that
    /// is a seam (riders carry no grantor today).
    fn unit_spendable_on(&self, unit: &crate::player::ManaUnit, subject: ObjectId) -> bool {
        let watcher = self.objects.obj(subject).source;
        unit.riders.iter().all(|r| match r {
            deckmaste_core::ManaRider::SpendOnly(f) => {
                self.filter_matches_live(f, subject, watcher)
            }
            _ => true,
        })
    }

    /// [CR#601.2g,106.6]: full payment validity — the structural coverage check
    /// ([`validate_payment`]) layered with spendability: every selected unit
    /// must be spendable on `subject`.
    #[must_use]
    pub(crate) fn validate_spendable(
        &self,
        player: PlayerId,
        cost: &ManaCost,
        payment: &Payment,
        subject: ObjectId,
    ) -> bool {
        let pool = &self.player(player).mana_pool;
        validate_payment(pool, cost, payment)
            && payment.units.iter().all(|&i| {
                pool.units()
                    .get(i)
                    .is_some_and(|u| self.unit_spendable_on(u, subject))
            })
    }

    /// [CR#106.6]: a clone of `player`'s pool holding only the units spendable
    /// on `subject` — the sub-pool an affordability check (`can_pay`) runs over
    /// so a spend-restricted unit can't fund an object it forbids.
    #[must_use]
    pub(crate) fn spendable_pool(&self, player: PlayerId, subject: ObjectId) -> ManaPool {
        let units = self
            .player(player)
            .mana_pool
            .units()
            .iter()
            .filter(|u| self.unit_spendable_on(u, subject))
            .cloned()
            .collect();
        ManaPool::from_units(units)
    }

    /// [CR#733.1,733.2]: reverse an in-flight announce whose announced cost can't
    /// be paid. A spell returns to its origin zone; an activated ability's
    /// minted stack identity is discarded (the source is untouched). No
    /// triggers fire (none were queued — targets are chosen after X), and
    /// the caster keeps priority. Drains this cast's continuation, still
    /// contiguous at the agenda front (`take_priority_action` pushed the
    /// whole block onto an empty agenda; no priority is held mid-announce),
    /// then reopens priority.
    ///
    /// # Panics
    /// Panics if no announce is in flight.
    pub(crate) fn rewind_announce(&mut self) {
        let pending = self.announcing.take().expect("an announce to rewind");
        match &pending.object {
            StackObject::Spell(o) => {
                let object = *o;
                self.objects.obj_mut(object).zone = Some(pending.origin);
                self.zones.hands[pending.controller.index()].push(object);
            }
            StackObject::Activated { .. } => {
                // The id begin_activate minted was never committed to the stack.
                self.objects.remove(pending.id);
            }
            StackObject::Triggered { .. } => {
                unreachable!("triggers never occupy the announce slot")
            }
        }
        while let Some(item) = self.agenda.pop_front() {
            debug_assert!(
                matches!(
                    item,
                    WorkItem::AnnounceTargets
                        | WorkItem::PayCost
                        | WorkItem::Emit(_)
                        | WorkItem::CheckSbas
                        | WorkItem::PlaceTriggers
                        | WorkItem::OpenPriority
                ),
                "rewind drained an unexpected agenda item: {item:?}"
            );
        }
        self.schedule_front(vec![WorkItem::OpenPriority]);
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
    fn validate_payment_selects_units() {
        // Pool [G, R] (indices 0, 1) against {1}{G}: covering selection valid.
        let p = pool(&[(green(), 1), (red(), 1)]);
        assert!(validate_payment(
            &p,
            &cost("{1}{G}"),
            &Payment { units: vec![0, 1] }
        ));
        // Too few units (mana value is 2, only one selected).
        assert!(!validate_payment(
            &p,
            &cost("{1}{G}"),
            &Payment { units: vec![0] }
        ));
        // Too many units (over-spend).
        assert!(!validate_payment(
            &pool(&[(green(), 1), (red(), 2)]),
            &cost("{1}{G}"),
            &Payment {
                units: vec![0, 1, 2]
            }
        ));
        // Out-of-range index.
        assert!(!validate_payment(
            &p,
            &cost("{1}{G}"),
            &Payment { units: vec![0, 9] }
        ));
        // Duplicate index (would select the same unit twice).
        assert!(!validate_payment(
            &p,
            &cost("{1}{G}"),
            &Payment { units: vec![0, 0] }
        ));
        // The colored {G} need is unmet: selecting two reds for {1}{G}.
        assert!(!validate_payment(
            &pool(&[(red(), 2)]),
            &cost("{1}{G}"),
            &Payment { units: vec![0, 1] }
        ));
    }

    #[test]
    fn validate_and_apply_round_trip() {
        let mut p = pool(&[(green(), 1), (red(), 1)]); // 0=G, 1=R
        let pay = Payment { units: vec![1, 0] }; // {1}<-R(1), {G}<-G(0)
        assert!(validate_payment(&p, &cost("{1}{G}"), &pay));
        apply_payment(&mut p, &pay);
        assert!(p.is_empty());
    }

    #[test]
    fn auto_pay_covers_colored_then_generic() {
        // Pool [G, G, R] (0,1,2), cost {1}{G}: {G} forced to a green, {1} to the
        // next unused unit (the other green). Deterministic, first-fit.
        let p = pool(&[(green(), 2), (red(), 1)]);
        let pay = auto_pay(&p, &cost("{1}{G}"));
        assert_eq!(pay.units, vec![0, 1]);
        assert!(validate_payment(&p, &cost("{1}{G}"), &pay));
    }

    #[test]
    fn concretize_x_substitutes_variable_with_generic() {
        // {X}{R} at X=3 -> {3}{R}; X=0 -> {0}{R}; a cost with no X is unchanged.
        assert_eq!(concretize_x(&cost("{X}{R}"), 3), cost("{3}{R}"));
        assert_eq!(concretize_x(&cost("{X}{R}"), 0), cost("{0}{R}"));
        assert_eq!(concretize_x(&cost("{1}{G}"), 5), cost("{1}{G}"));
    }
}
