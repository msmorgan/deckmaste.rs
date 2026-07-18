use std::collections::HashMap;
use std::collections::HashSet;

use crate::agenda::WorkItem;
use crate::decide::Decision;
use crate::decide::DecisionError;
use crate::decide::DecisionHandler;
use crate::event::Attacking;
use crate::event::Blocked;
use crate::event::GameEvent;
use crate::event::Occurrence;
use crate::object::ObjectId;
use crate::player::PlayerId;
use crate::state::GameState;

/// [CR#508.1a,508.1b]: the active player declares attackers, and what each
/// attacks. `legal` is the surfaced candidate attacker set; `legal_targets`
/// is what may be attacked — the defending player's proxy object plus every
/// planeswalker they control ([CR#506.3,508.1b]). `submit_decision`
/// re-validates each declared pair against both.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DeclareAttackers {
    pub player: PlayerId,
    pub legal: Vec<ObjectId>,
    pub legal_targets: Vec<ObjectId>,
}

impl DecisionHandler for DeclareAttackers {
    fn resolve(self, g: &mut GameState, answer: Decision) -> Result<(), DecisionError> {
        let Decision::Attackers(chosen) = answer else {
            return Err(DecisionError::WrongKind);
        };
        let DeclareAttackers {
            player,
            legal,
            legal_targets,
        } = self;
        // [CR#508.1a]: each chosen attacker must be in the surfaced
        // legal set, and no creature attacks twice. [CR#508.1b]:
        // each attacker's target must be in the surfaced legal-target
        // set (the defending player or a planeswalker they control).
        let distinct: HashSet<_> = chosen.iter().map(|&(a, _)| a).collect();
        if distinct.len() != chosen.len()
            || !chosen.iter().all(|(a, _)| legal.contains(a))
            || !chosen.iter().all(|(_, t)| legal_targets.contains(t))
        {
            return Err(DecisionError::Illegal {
                reason: "attackers must be distinct, from the legal set, attacking a legal target"
                    .into(),
            });
        }
        // [CR#508.1d]: attack requirements ("attacks if able",
        // goad) — every surfaced-legal creature matched by a
        // Must(Attack) row whose `on` matches the defender must be
        // among the chosen. The legal set already excludes
        // restricted creatures (tapped/sick/Cant rows), the Attack
        // pattern carries no arrangement bound, and Gate costs are
        // never forced (Gate rows still trip the presence guard) —
        // so requirements decompose per-creature and obeying all
        // of them is always possible: the maximize arbitration
        // reduces to a membership check.
        let view = g.layers();
        let rows = crate::legal::must_attack_rows(g, &view);
        let defender_proxy = g.players.iter().find(|p| p.id != player).map(|p| p.object);
        if let Some(&required) = legal.iter().find(|&&c| {
            !chosen.iter().any(|&(a, _)| a == c)
                && rows.iter().any(|(carrier, by, on)| {
                    g.filter_matches_live(by, c, *carrier)
                        && defender_proxy.is_some_and(|d| g.filter_matches_live(on, d, *carrier))
                })
        }) {
            return Err(DecisionError::Illegal {
                reason: format!("a Must(Attack) requirement obliges {required:?} to attack"),
            });
        }
        g.pending = None;
        // [CR#508.1f]: declaring taps the attacker. The whole declaration
        // is one simultaneous occurrence — a `Batch` (empty when no
        // attackers were declared, which schedules nothing observable).
        if !chosen.is_empty() {
            g.schedule_front(vec![WorkItem::Emit(Occurrence::Batch(
                chosen
                    .into_iter()
                    .map(|(attacker, defending)| {
                        GameEvent::Attacking(Attacking {
                            attacker,
                            defending,
                        })
                    })
                    .collect(),
            ))]);
        }
        Ok(())
    }
}

/// [CR#509.1a]: the **defending** player declares blockers. `player` is the
/// defender (the non-active player); `legal` is the surfaced candidate set
/// of legal blockers; `submit_decision` re-validates against it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DeclareBlockers {
    pub player: PlayerId,
    pub legal: Vec<ObjectId>,
}

impl DecisionHandler for DeclareBlockers {
    fn resolve(self, g: &mut GameState, answer: Decision) -> Result<(), DecisionError> {
        let Decision::Blocks(pairs) = answer else {
            return Err(DecisionError::WrongKind);
        };
        let DeclareBlockers { player: _, legal } = self;
        // [CR#509.1a]: each blocker is from the surfaced legal set, each
        // blocked creature is an attacker, and no creature blocks twice
        // (a creature blocks exactly one attacker).
        let distinct: HashSet<_> = pairs.iter().map(|&(b, _)| b).collect();
        let attackers = g.combat.attackers();
        if distinct.len() != pairs.len()
            || !pairs
                .iter()
                .all(|(b, a)| legal.contains(b) && attackers.contains(a))
        {
            return Err(DecisionError::Illegal {
                reason: "each blocker (once) blocks an attacker from the legal set".into(),
            });
        }
        // [CR#509.1b]: evaluate the point-wise Cant(Block) rows
        // (flying-family evasion) against each proposed pair — the
        // first deontic rows the engine evaluates instead of
        // guarding. Arrangement-level bounds (menace) are still the
        // legal_blockers presence guard's business.
        let view = g.layers();
        let rows = crate::legal::cant_block_rows(g, &view);
        for &(blocker, attacker) in &pairs {
            if let Some(carrier) = crate::legal::block_forbidden_by(g, &rows, blocker, attacker) {
                return Err(DecisionError::Illegal {
                    reason: format!("a Cant(Block) row on {carrier:?} forbids this block"),
                });
            }
        }
        // [CR#702.111b]-family: arrangement-level bounds judge each
        // attacker's WHOLE blocker set (menace — a lone blocker is a
        // forbidden arrangement; no blockers is no arrangement).
        let mut by_attacker: HashMap<ObjectId, Vec<ObjectId>> = HashMap::new();
        for &(blocker, attacker) in &pairs {
            by_attacker.entry(attacker).or_default().push(blocker);
        }
        for (attacker, blockers) in &by_attacker {
            if let Some(carrier) =
                crate::legal::arrangement_forbidden_by(g, &rows, *attacker, blockers)
            {
                return Err(DecisionError::Illegal {
                    reason: format!(
                        "a Cant(Block) arrangement bound on {carrier:?} forbids this \
                         blocker set"
                    ),
                });
            }
        }
        // [CR#509.1c]: block requirements ("blocks if able", "all
        // creatures able to block … do so") — each surfaced-legal
        // blocker matched by a Must(Block) row's `by` is demanded
        // to block an `on`-matching attacker it isn't point-wise
        // forbidden from blocking. A blocker obeys all its
        // instances by blocking inside the intersection of their
        // demanded sets; an instance whose demanded set is empty
        // is unsatisfiable and waived. The cases needing the full
        // maximize arbitration are LOUD seams, not approximations:
        // requirements interacting with an arrangement bound
        // ([CR#509.1c]'s menace example — both creatures must
        // block), conflicting instances (empty intersection), and
        // a requirement row carrying its own bound.
        let must_rows = crate::legal::must_block_rows(g, &view);
        for &b in &legal {
            let mut demanded: Vec<Vec<ObjectId>> = Vec::new();
            for row in &must_rows {
                if row.count.is_some() {
                    todo!("a Must(Block) row carrying an arrangement bound");
                }
                if !g.filter_matches_live(&row.by, b, row.carrier) {
                    continue;
                }
                let set: Vec<ObjectId> = attackers
                    .iter()
                    .copied()
                    .filter(|&a| g.filter_matches_live(&row.on, a, row.carrier))
                    .filter(|&a| crate::legal::block_forbidden_by(g, &rows, b, a).is_none())
                    .collect();
                let bounded = set.iter().any(|&a| {
                    rows.iter()
                        .any(|r| r.count.is_some() && g.filter_matches_live(&r.on, a, r.carrier))
                });
                if bounded {
                    todo!(
                        "Must(Block) × arrangement-bound arbitration \
                         ([CR#509.1c]'s menace example)"
                    );
                }
                if !set.is_empty() {
                    demanded.push(set);
                }
            }
            let Some(first) = demanded.first() else {
                continue;
            };
            let obeys: Vec<ObjectId> = first
                .iter()
                .copied()
                .filter(|a| demanded.iter().all(|s| s.contains(a)))
                .collect();
            if obeys.is_empty() {
                todo!("conflicting Must(Block) requirements need the maximize arbitration");
            }
            let blocks = pairs.iter().find(|&&(bb, _)| bb == b).map(|&(_, a)| a);
            if !blocks.is_some_and(|a| obeys.contains(&a)) {
                return Err(DecisionError::Illegal {
                    reason: format!("a Must(Block) requirement obliges {b:?} to block"),
                });
            }
        }
        g.pending = None;
        // [CR#509.1h]: the whole block declaration is one simultaneous
        // occurrence — a `Batch` (skipped when empty, which schedules
        // nothing observable).
        if !pairs.is_empty() {
            g.schedule_front(vec![WorkItem::Emit(Occurrence::Batch(
                pairs
                    .into_iter()
                    .map(|(blocker, attacker)| GameEvent::Blocked(Blocked { blocker, attacker }))
                    .collect(),
            ))]);
        }
        Ok(())
    }
}

/// [CR#510.1c]: divide `source`'s combat damage among its `recipients`
/// (free division — any split summing to `source`'s power is legal).
/// Surfaced only for a multi-blocked attacker (≥ 2 recipients); forced
/// cases auto-resolve. `player` is the source's controller.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AssignCombatDamage {
    pub player: PlayerId,
    pub source: ObjectId,
    pub recipients: Vec<ObjectId>,
}

impl DecisionHandler for AssignCombatDamage {
    fn resolve(self, g: &mut GameState, answer: Decision) -> Result<(), DecisionError> {
        let Decision::Assignment(amounts) = answer else {
            return Err(DecisionError::WrongKind);
        };
        let AssignCombatDamage {
            player: _,
            source,
            recipients,
        } = self;
        g.submit_assign_combat_damage(source, &recipients, amounts)
    }
}
