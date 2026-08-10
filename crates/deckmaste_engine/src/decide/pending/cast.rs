use std::collections::HashSet;
use std::sync::Arc;

use deckmaste_core::Uint;

use crate::agenda::WorkItem;
use crate::decide::Decision;
use crate::decide::DecisionError;
use crate::decide::DecisionHandler;
use crate::event::BecameTarget;
use crate::event::GameEvent;
use crate::event::Occurrence;
use crate::object::ObjectId;
use crate::player::PlayerId;
use crate::state::GameState;

/// [CR#601.2c,115]: choose targets for the in-flight announce. `legal[i]`
/// is the candidate set for `spec[i]`; `submit_decision` re-validates.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ChooseTargets {
    pub player: PlayerId,
    pub spec: Vec<deckmaste_core::TargetSpec>,
    pub legal: Vec<Vec<ObjectId>>,
}

impl DecisionHandler for ChooseTargets {
    fn resolve(self, g: &mut GameState, answer: Decision) -> Result<(), DecisionError> {
        let Decision::Targets(chosen) = answer else {
            return Err(DecisionError::WrongKind);
        };
        let ChooseTargets {
            player: _,
            spec,
            legal,
        } = self;
        // [CR#601.2c,115] / [CR#603.3d]: one chosen SET per spec, each
        // member drawn from that spec's legal candidate set.
        if chosen.len() != spec.len()
            || chosen
                .iter()
                .zip(&legal)
                .any(|(picks, set)| picks.iter().any(|c| !set.contains(c)))
        {
            return Err(DecisionError::Illegal {
                reason: "illegal target selection".into(),
            });
        }
        // [CR#601.2c,115.7e]: per-slot count within bounds, within-slot
        // distinctness, and cross-slot Distinct disjointness.
        if let Err(reason) = crate::resolve::validate_target_set(&spec, &chosen) {
            return Err(DecisionError::Illegal { reason });
        }
        // Targeting requirements (Must(Target) rows — the
        // Flagbearer class, "must choose at least one … if able"):
        // for each row whose `by` matches the targeting object,
        // if any spec's candidate set holds an `on`-matching
        // object, the chosen targets must include at least one.
        // Disjoint multi-row conflicts would need the maximize
        // arbitration (identical rows — the printed class — are
        // jointly satisfied by one choice, so per-row checks are
        // exact today).
        //
        // [CR#601.2c,602.2a,603.3,603.3d]: a triggered ability is
        // exempt. The printed wording scopes the requirement to
        // choosing targets while casting a spell or activating an
        // ability, but a trigger instead chooses its targets as it
        // is PUT ON the stack — a moment that wording never names.
        // The exemption is the wording's, not the rules': [CR#603.3d]
        // routes placement through [CR#601.2c], obligation clause
        // included, so skipping wholesale is only sound while every
        // printed must-target effect carries that casting/activating
        // scope. One that didn't would need a per-row check here.
        // `g.placing_trigger` is `Some` only while a triggered
        // ability's placement-time target choice is in flight, an
        // orthogonal discriminator from the `by` filter's
        // `ObjectKind::Ability` collapse (which stays untouched —
        // it still lets a filter match "activated or triggered
        // ability" for Stifle-style effects).
        if g.placing_trigger.is_none() {
            let view = g.layers();
            let must_rows = crate::legal::must_target_rows(g, &view);
            if !must_rows.is_empty() {
                let targeting = g.announcing.as_ref().expect("an announce in flight").id;
                for (carrier, by, on) in &must_rows {
                    if !crate::legal::deed_agent_matches(g, by, targeting, *carrier) {
                        continue;
                    }
                    let able = legal
                        .iter()
                        .any(|set| set.iter().any(|&t| g.filter_matches_live(on, t, *carrier)));
                    let obeyed = chosen
                        .iter()
                        .flatten()
                        .any(|&t| g.filter_matches_live(on, t, *carrier));
                    if able && !obeyed {
                        return Err(DecisionError::Illegal {
                            reason: format!(
                                "a Must(Target) requirement on {carrier:?} obliges this \
                                 choice to include a matching target"
                            ),
                        });
                    }
                }
            }
        }
        g.pending = None;
        // [CR#601.2c]: the chosen objects become targets NOW — one
        // fact per distinct target (an object chosen for two specs
        // becomes the target once), simultaneous as one occurrence.
        // The targeting object: a placing trigger's minted stack id
        // ([CR#603.3d]), the announcing spell itself (stack zone —
        // its remint is the deferred one), or an ability announce's
        // SOURCE ([CR#602.2a] — no stack id until promote).
        let targeting = if let Some(staged) = &g.placing_trigger {
            staged.id
        } else {
            match &g.announcing.as_ref().expect("an announce in flight").object {
                crate::stack::StackObject::Spell(o) => *o,
                crate::stack::StackObject::Activated { source, .. } => *source,
                crate::stack::StackObject::Triggered { .. } => {
                    unreachable!("triggers choose targets at placement, not announce")
                }
            }
        };
        let mut became: Vec<GameEvent> = Vec::new();
        for &target in chosen.iter().flatten() {
            let dup = became
                .iter()
                .any(|e| matches!(e, GameEvent::BecameTarget(BecameTarget { target: t, .. }) if *t == target));
            if !dup {
                became.push(GameEvent::BecameTarget(BecameTarget {
                    target,
                    source: targeting,
                }));
            }
        }
        if g.placing_trigger.is_some() {
            // [CR#603.3d]: a triggered ability chose its targets at
            // placement — commit it onto the stack and resume placement.
            g.commit_placing_trigger(chosen);
            g.schedule_front(vec![WorkItem::CheckSbas, WorkItem::PlaceTriggers]);
        } else {
            g.announcing
                .as_mut()
                .expect("an announce in flight")
                .targets = chosen;
        }
        // Ahead of the resumed placement / cast continuation, so
        // becomes-target triggers (ward, [CR#702.21a]) note in this
        // lock's wake.
        let occ = if became.len() == 1 {
            Occurrence::Single(became.pop().expect("len 1"))
        } else {
            Occurrence::Batch(became)
        };
        g.schedule_front(vec![WorkItem::Emit(occ)]);
        Ok(())
    }
}

/// [CR#707.10c,115.7d]: re-target a COMMITTED stack entry — surface a
/// `Retarget` decision whose per-slot legal set is the fresh
/// legal candidates PLUS the current target (leaving a slot unchanged
/// is always allowed, even when the current target is illegal; a
/// CHANGED slot must be legal).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Retarget {
    pub player: PlayerId,
    pub entry: ObjectId,
    pub spec: Vec<deckmaste_core::TargetSpec>,
    pub legal: Vec<Vec<ObjectId>>,
}

impl DecisionHandler for Retarget {
    fn resolve(self, g: &mut GameState, answer: Decision) -> Result<(), DecisionError> {
        let Decision::Targets(chosen) = answer else {
            return Err(DecisionError::WrongKind);
        };
        let Retarget {
            player: _,
            entry,
            spec,
            legal,
        } = self;
        // [CR#707.10c]: same length/membership validation as
        // `ChooseTargets` above — each slot's answer is drawn from
        // `legal[i]`, which the handler already unioned with the
        // entry's CURRENT target (leaving a slot unchanged is always
        // legal, even when the current target no longer qualifies
        // fresh; a CHANGED slot must land on a fresh-legal
        // candidate).
        if chosen.len() != spec.len()
            || chosen
                .iter()
                .zip(&legal)
                .any(|(picks, set)| picks.iter().any(|c| !set.contains(c)))
        {
            return Err(DecisionError::Illegal {
                reason: "illegal target selection".into(),
            });
        }
        // [CR#601.2c,115.7e]: counts (locked at announce) unchanged,
        // within-slot + Distinct re-validated on the whole proposed set.
        if let Err(reason) = crate::resolve::validate_target_set(&spec, &chosen) {
            return Err(DecisionError::Illegal { reason });
        }
        g.pending = None;
        // [CR#707.10c]: the referenced entry may have left the stack
        // between this decision surfacing and its answer (e.g.
        // countered in response) — a no-op, not a crash.
        if let Some(e) = g.stack.iter_mut().find(|e| e.id == entry) {
            e.targets = chosen;
        }
        Ok(())
    }
}

/// [CR#601.2g]: allocate pool mana to the in-flight cost. `subject` is the
/// object being paid for — the spell, or an activated ability's source
/// ([CR#106.6]) — so a `SpendOnly` rider can judge it at validation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PayMana {
    pub player: PlayerId,
    pub cost: deckmaste_core::ManaCost,
    pub pool: crate::player::ManaPool,
    pub subject: ObjectId,
}

impl DecisionHandler for PayMana {
    fn resolve(self, g: &mut GameState, answer: Decision) -> Result<(), DecisionError> {
        let Decision::Pay(payment) = answer else {
            return Err(DecisionError::WrongKind);
        };
        let PayMana {
            player,
            cost,
            pool: _,
            subject,
        } = self;
        // [CR#106.6]: layer SpendOnly spendability on the structural
        // coverage check — each selected unit must be spendable on the
        // object being paid for.
        if !g.validate_spendable(player, &cost, &payment, subject) {
            return Err(DecisionError::Illegal {
                reason: "payment does not cover the cost".into(),
            });
        }
        g.pending = None;
        crate::cast::apply_payment(&mut g.player_mut(player).mana_pool, &payment);
        Ok(())
    }
}

/// Announce-time cost intentions ([CR#601.2b]): the player announces the
/// nonhybrid equivalent of each hybrid symbol ([CR#107.4e]) and, for each
/// Phyrexian symbol, color-or-2-life ([CR#107.4f]). `options[i]` is the
/// legal readings of the i-th choosable symbol of `cost` (cost order); the
/// answer ([`Decision::CostOptions`]) supplies one pick per entry. Kicker
/// and alternative-cost selection will join this kind in a later task.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ChooseCostOptions {
    pub player: PlayerId,
    pub cost: deckmaste_core::ManaCost,
    /// Already-concrete nonmana components contributed by an alternative
    /// cost or the announced modes. These join the Phyrexian-life components
    /// produced by concretizing `cost`.
    pub additional: Vec<deckmaste_core::CostComponent>,
    // pre-computed from choosable(&cost); redundancy is intentional so the
    // player's answer can be validated without re-reading the cost.
    pub options: crate::cost_options::ChoosableOptions,
}

impl DecisionHandler for ChooseCostOptions {
    fn resolve(self, g: &mut GameState, answer: Decision) -> Result<(), DecisionError> {
        let Decision::CostOptions(choices) = answer else {
            return Err(DecisionError::WrongKind);
        };
        let ChooseCostOptions {
            cost,
            mut additional,
            ..
        } = self;
        // [CR#601.2b]: apply the announced readings to the printed cost.
        // An illegal announce (wrong pick count, or a reading the symbol
        // doesn't offer) is rejected — the decision stays pending.
        let (mana, mut components) = match crate::cost_options::concretize(&cost, &choices) {
            Ok(c) => c,
            Err(e) => {
                return Err(DecisionError::Illegal {
                    reason: format!("illegal cost-option announce: {e:?}"),
                });
            }
        };
        components.append(&mut additional);
        // Stash the concretized (mana, Phyrexian-life verbs) on the
        // announce slot for `PayCost` to consume.
        g.announcing
            .as_mut()
            .expect("an announce is in flight across ChooseCostOptions")
            .concretized = Some((mana, components));
        g.pending = None;
        Ok(())
    }
}

/// [CR#601.2b]: announce the value of `{X}` in the in-flight cost. Any value
/// >= 0 is accepted; an unpayable announcement rewinds the cast ([CR#733]).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ChooseXValue {
    pub player: PlayerId,
}

impl DecisionHandler for ChooseXValue {
    fn resolve(self, g: &mut GameState, answer: Decision) -> Result<(), DecisionError> {
        let Decision::XValue(x) = answer else {
            return Err(DecisionError::WrongKind);
        };
        let ChooseXValue { player } = self;
        // [CR#601.2b]: record the announced value in the open slot.
        g.announcing
            .as_mut()
            .expect("an announce in flight for ChooseXValue")
            .x = Some(x);
        // [CR#601.2h,733]: an unpayable announcement reverses the cast.
        // Read the kind + base cost immutably, then decide.
        let pending = g.announcing.as_ref().expect("an announce in flight");
        // `pip_spell` names the spell whose `PayPips` statics (convoke /
        // delve / improvise) may cover pips of the X-concretized cost —
        // `Some` only for a spell; an activated ability has no `PayPips`.
        let (subject, base, pip_spell) = match &pending.object {
            crate::stack::StackObject::Spell(o) => (
                *o,
                // A face with no mana cost reads as mana value 0 here:
                // the `mana_cost` seam reserves `None` for a future
                // no-cost face, and an empty cost concretizes/affords as
                // a free base. The engine never panics on card data.
                g.mana_cost(*o).unwrap_or_default(),
                Some(*o),
            ),
            crate::stack::StackObject::Activated {
                source, ability, ..
            } => (
                *source,
                crate::activate::cost_summary(&ability.cost)
                    .expect("can_activate vetted the cost")
                    .mana,
                None,
            ),
            crate::stack::StackObject::Triggered { .. } => {
                unreachable!("triggers never occupy the announce slot")
            }
        };
        // [CR#601.2b,107.3a,107.4e,107.4f]: with X now fixed to its
        // announced value, the cost may STILL carry hybrid/Phyrexian
        // symbols (a `{X}{W/U}`-style cost composing engine-x-costs with
        // engine-cost-payment). A bare `can_pay` rejects any cost with a
        // choosable symbol (`requirement` returns `None`), so the
        // payability check must go through the reading-search gate —
        // "is SOME hybrid/Phyrexian reading of the X-concretized cost
        // payable?" — which subsumes `can_pay` for a plain/X-only cost.
        let payable = g.affordable_concretization(
            player,
            &crate::cast::concretize_x(&base, x),
            subject,
            pip_spell,
        );
        g.pending = None;
        // Writing `x` first is safe: `rewind_announce` discards the
        // whole announcing slot, including the `x` just written.
        if !payable {
            g.rewind_announce();
        }
        Ok(())
    }
}

/// Choose a modal spell/ability's modes ([CR#700.2a..700.2b]). `options` is
/// how many modes are offered; the answer ([`Decision::Modes`]) is a list
/// of option indices, `min..=max` entries long, distinct unless
/// `repeats` ([CR#700.2d]).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ChooseModes {
    pub player: PlayerId,
    pub options: Uint,
    pub min: Uint,
    pub max: Uint,
    pub repeats: bool,
    /// An entwine rider offers the additional all-modes alternative outside
    /// the printed `min..=max` choice.
    pub entwine: bool,
}

impl DecisionHandler for ChooseModes {
    fn resolve(self, g: &mut GameState, answer: Decision) -> Result<(), DecisionError> {
        let Decision::Modes(picks) = answer else {
            return Err(DecisionError::WrongKind);
        };
        let ChooseModes {
            options,
            min,
            max,
            repeats,
            entwine,
            ..
        } = self;
        // [CR#700.2,700.2d]: count in [min,max], each index a real mode,
        // distinct unless the same mode may be chosen more than once.
        let n = Uint::try_from(picks.len()).expect("pick count fits Uint");
        let distinct = repeats || {
            let set: HashSet<_> = picks.iter().copied().collect();
            set.len() == picks.len()
        };
        let normal = n >= min && n <= max && picks.iter().all(|&i| i < options) && distinct;
        let all_modes = entwine && n == options && picks.iter().copied().eq(0..options);
        let legal = normal || all_modes;
        if !legal {
            return Err(DecisionError::Illegal {
                reason: "illegal mode selection".into(),
            });
        }
        if matches!(
            g.choice,
            Some(crate::state::ChoiceContinuation::AnnounceModes)
        ) && !g.payment_mana_modes_legal(&picks)
        {
            return Err(DecisionError::Illegal {
                reason: "that mode selection does not produce a legal mana ability".into(),
            });
        }
        g.pending = None;
        let continuation = g
            .choice
            .take()
            .expect("a ChooseModes decision stashed its continuation");
        match continuation {
            crate::state::ChoiceContinuation::AnnounceModes => {
                g.announcing
                    .as_mut()
                    .expect("an announce is in flight across ChooseModes")
                    .chosen_modes = picks.into();
            }
            crate::state::ChoiceContinuation::Modal { modes, frame } => {
                // [CR#700.2]: a resolution-time modal instruction applies the
                // chosen modes' effects in pick order.
                let items = picks
                    .into_iter()
                    .map(|i| WorkItem::RunEffect {
                        effect: Arc::new(modes[i as usize].effect.clone()),
                        frame: frame.clone(),
                    })
                    .collect();
                g.schedule_front(items);
            }
            other => unreachable!("ChooseModes stashed a nonmodal continuation: {other:?}"),
        }
        Ok(())
    }
}
