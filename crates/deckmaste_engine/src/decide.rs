use std::collections::HashMap;
use std::collections::HashSet;
use std::fmt;

use deckmaste_core::DeciderSpec;
use deckmaste_core::KeywordAbility;
use deckmaste_core::LockPoint;
use deckmaste_core::Uint;
use deckmaste_core::Visibility;

use crate::object::ObjectId;
use crate::object::ObjectSource;
use crate::player::PlayerId;

/// A pre-game decision ([CR#103]) — surfaced before turn one.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PreGameKind {
    /// Who takes the first turn ([CR#103.1]).
    FirstTurn,
    /// Keep or mulligan; London bottoming is the committed-hidden part
    /// ([CR#103.5]).
    Mulligan,
    /// Reveal a companion ([CR#103.2b]).
    Companion,
    /// Opening-hand (Leyline-style) actions, turn order ([CR#103.6]).
    OpeningHand,
}

/// A special action ([CR#116.2]) — taken with priority, no stack; the
/// closed CR list (land play is already `Action::PlayLand`). All arms are
/// P0.W3 shells: `legal_actions` never offers them yet.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SpecialAction {
    /// Turn a face-down creature face up ([CR#116.2b]).
    TurnFaceUp(ObjectId),
    /// End a continuous/delayed effect that allows it ([CR#116.2c]).
    EndEffect(ObjectId),
    /// Ignore a static ability's effect for a duration ([CR#116.2d]).
    IgnoreStatic(ObjectId),
    /// Exile a suspend card from hand ([CR#116.2f]).
    SuspendCast(ObjectId),
    /// Pay {3} to bring the companion to hand ([CR#116.2g]).
    CompanionToHand,
    /// Pay {2} to foretell ([CR#116.2h]).
    Foretell(ObjectId),
    /// Exile a plot card from hand ([CR#116.2k]).
    PlotExile(ObjectId),
    /// Pay an unlock cost ([CR#116.2m]).
    UnlockHalf(ObjectId),
}

/// The boundary record of a pending decision (mtg-rules choices.md §6):
/// the kind plus its schema row — nominal decider, lock stage, visibility.
/// Derived from the kind by the schema methods below (the choices table is
/// encoded once); UI/AI/replay consumers read THIS, while
/// `Progress::NeedsDecision` stays the kind-only notification.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DecisionPoint {
    pub pending: PendingDecision,
    pub decider: DeciderSpec,
    pub lock: LockPoint,
    pub visibility: Visibility,
}

impl DecisionPoint {
    /// The concrete player this decision is waiting on — convenience over
    /// `self.pending.decider_player()` so callers needn't reach through the
    /// `pending` field.
    #[must_use]
    pub fn decider_player(&self) -> PlayerId {
        self.pending.decider_player()
    }
}

impl PendingDecision {
    /// The choices.md §2 schema row: nominal decider. Refinements
    /// (delegation, APNAP per-combatant) arrive with the kinds' behavior.
    #[must_use]
    pub fn decider_spec(&self) -> DeciderSpec {
        match self {
            PendingDecision::Priority { .. } => DeciderSpec::PriorityHolder,
            PendingDecision::DeclareAttackers { .. } => DeciderSpec::ActivePlayer,
            PendingDecision::DeclareBlockers { .. } => DeciderSpec::DefendingPlayer,
            PendingDecision::Vote { .. } => DeciderSpec::EachInTurnOrder,
            _ => DeciderSpec::Controller,
        }
    }

    /// The CONCRETE player this decision is waiting on (every kind
    /// carries one) — the nominal-role view is [`Self::decider_spec`].
    #[must_use]
    pub fn decider_player(&self) -> PlayerId {
        match self {
            PendingDecision::Priority { player, .. }
            | PendingDecision::DiscardToHandSize { player, .. }
            | PendingDecision::DiscardCards { player, .. }
            | PendingDecision::ChooseManaColor { player, .. }
            | PendingDecision::ChooseTargets { player, .. }
            | PendingDecision::PayMana { player, .. }
            | PendingDecision::OrderTriggers { player, .. }
            | PendingDecision::DeclareAttackers { player, .. }
            | PendingDecision::DeclareBlockers { player, .. }
            | PendingDecision::AssignCombatDamage { player, .. }
            | PendingDecision::ChooseModes { player, .. }
            | PendingDecision::Division { player, .. }
            | PendingDecision::Vote { player, .. }
            | PendingDecision::YesNo { player, .. }
            | PendingDecision::OrderReplacements { player, .. }
            | PendingDecision::ChooseCostOptions { player, .. }
            | PendingDecision::ChooseXValue { player, .. }
            | PendingDecision::ChooseObjects { player, .. }
            | PendingDecision::PreGame { player, .. } => *player,
        }
    }

    /// The choices.md §2 lock stage on the shared `LockPoint` axis.
    #[must_use]
    pub fn lock(&self) -> LockPoint {
        match self {
            PendingDecision::ChooseTargets { .. }
            | PendingDecision::ChooseModes { .. }
            | PendingDecision::ChooseCostOptions { .. }
            | PendingDecision::ChooseXValue { .. }
            | PendingDecision::Division { .. } => LockPoint::Announce,
            PendingDecision::PayMana { .. } => LockPoint::Payment,
            PendingDecision::OrderTriggers { .. } => LockPoint::StackPlacement,
            PendingDecision::DeclareAttackers { .. }
            | PendingDecision::DeclareBlockers { .. }
            | PendingDecision::AssignCombatDamage { .. } => LockPoint::Declaration,
            PendingDecision::PreGame { .. } => LockPoint::PreGame,
            // Priority actions and resolution-stage choices bind as applied.
            _ => LockPoint::Resolution,
        }
    }

    /// The choices.md §3 visibility class. London bottoming is the one
    /// committed-hidden pre-game payload ([CR#103.5]).
    #[must_use]
    pub fn visibility(&self) -> Visibility {
        match self {
            PendingDecision::PreGame {
                kind: PreGameKind::Mulligan,
                ..
            } => Visibility::CommittedHidden,
            _ => Visibility::Open,
        }
    }
}

/// What the engine is waiting on. `step()` returns `NeedsDecision` (without
/// mutating) until `submit_decision` answers it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PendingDecision {
    /// [CR#117]: the holder may act or pass. `legal` is advisory UI data —
    /// submission re-validates.
    Priority {
        player: PlayerId,
        legal: Vec<Action>,
    },
    /// [CR#514.1]: discard down to maximum hand size.
    DiscardToHandSize { player: PlayerId, count: Uint },
    /// [CR#701.9b]: a resolving discard — `player` chooses which `count` cards
    /// from their hand to discard (`count` already clamped to the hand size).
    DiscardCards { player: PlayerId, count: Uint },
    /// [CR#106.1b]: a resolving `AddMana` whose production is a choice ("any
    /// color" offers the five colors per [CR#105.4]; "{W} or {U}" offers its
    /// printed set) — `player` picks one of `options`.
    ChooseManaColor {
        player: PlayerId,
        options: Vec<deckmaste_core::ColorOrColorless>,
        amount: Uint,
        riders: Vec<deckmaste_core::ManaRider>,
    },
    /// [CR#601.2c,115]: choose targets for the in-flight announce. `legal[i]`
    /// is the candidate set for `spec[i]`; `submit_decision` re-validates.
    ChooseTargets {
        player: PlayerId,
        spec: Vec<deckmaste_core::TargetSpec>,
        legal: Vec<Vec<ObjectId>>,
    },
    /// [CR#601.2g]: allocate pool mana to the in-flight cost. `subject` is the
    /// object being paid for — the spell, or an activated ability's source
    /// ([CR#106.6]) — so a `SpendOnly` rider can judge it at validation.
    PayMana {
        player: PlayerId,
        cost: deckmaste_core::ManaCost,
        pool: crate::player::ManaPool,
        subject: ObjectId,
    },
    /// [CR#603.3b]: a player controlling several simultaneous triggers orders
    /// them. The submitted `Order` is a permutation of `0..triggers.len()`.
    OrderTriggers {
        player: PlayerId,
        triggers: Vec<crate::trigger::NotedTrigger>,
    },
    /// [CR#508.1a]: the active player declares attackers. `legal` is the
    /// surfaced candidate set; `submit_decision` re-validates against it.
    DeclareAttackers {
        player: PlayerId,
        legal: Vec<ObjectId>,
    },
    /// [CR#509.1a]: the **defending** player declares blockers. `player` is the
    /// defender (the non-active player); `legal` is the surfaced candidate set
    /// of legal blockers; `submit_decision` re-validates against it.
    DeclareBlockers {
        player: PlayerId,
        legal: Vec<ObjectId>,
    },
    /// [CR#510.1c]: divide `source`'s combat damage among its `recipients`
    /// (free division — any split summing to `source`'s power is legal).
    /// Surfaced only for a multi-blocked attacker (≥ 2 recipients); forced
    /// cases auto-resolve. `player` is the source's controller.
    AssignCombatDamage {
        player: PlayerId,
        source: ObjectId,
        recipients: Vec<ObjectId>,
    },
    /// Choose a modal spell/ability's modes ([CR#700.2a..700.2b]). `options` is
    /// how many modes are offered; the answer ([`Decision::Modes`]) is a list
    /// of option indices, `min..=max` entries long, distinct unless
    /// `repeats` ([CR#700.2d]).
    ChooseModes {
        player: PlayerId,
        options: Uint,
        min: Uint,
        max: Uint,
        repeats: bool,
    },
    /// Divide damage/counters among targets ([CR#601.2d,608.2d]) — shell.
    Division {
        player: PlayerId,
        total: Uint,
        targets: Vec<ObjectId>,
    },
    /// Vote, each player in turn order ([CR#701.38a]) — shell.
    Vote { player: PlayerId, options: Uint },
    /// A fixed-window yes/no ("… unless you pay", [CR#608.2d]) — shell.
    YesNo { player: PlayerId },
    /// Announce-time cost intentions ([CR#601.2b]): the player announces the
    /// nonhybrid equivalent of each hybrid symbol ([CR#107.4e]) and, for each
    /// Phyrexian symbol, color-or-2-life ([CR#107.4f]). `options[i]` is the
    /// legal readings of the i-th choosable symbol of `cost` (cost order); the
    /// answer ([`Decision::CostOptions`]) supplies one pick per entry. Kicker
    /// and alternative-cost selection will join this kind in a later task.
    ChooseCostOptions {
        player: PlayerId,
        cost: deckmaste_core::ManaCost,
        // pre-computed from choosable(&cost); redundancy is intentional so the
        // player's answer can be validated without re-reading the cost.
        options: crate::cost_options::ChoosableOptions,
    },
    /// [CR#601.2b]: announce the value of `{X}` in the in-flight cost. Any value
    /// >= 0 is accepted; an unpayable announcement rewinds the cast ([CR#733]).
    ChooseXValue { player: PlayerId },
    /// Order the replacement/prevention effects applicable to one event,
    /// affected player/controller choosing ([CR#616.1]) — shell.
    OrderReplacements { player: PlayerId, count: Uint },
    /// A pre-game choice ([CR#103]) — shell.
    PreGame { player: PlayerId, kind: PreGameKind },
    /// [CR#608.2d]: choose objects at resolution. `candidates` is the matching
    /// set; the answer picks between `min` and `max` of them (both clamped to
    /// `candidates.len()` — "as many as able").
    ChooseObjects {
        player: PlayerId,
        candidates: Vec<ObjectId>,
        min: Uint,
        max: Uint,
    },
}

/// An answer to the pending decision.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Decision {
    /// Modes chosen, by option index ([CR#700.2]) — P0.W3 shell.
    Modes(Vec<Uint>),
    /// A division among targets ([CR#601.2d]) — shell.
    Divide(Vec<(ObjectId, Uint)>),
    /// A vote, by option index ([CR#701.38a]) — shell.
    VoteFor(Uint),
    /// A yes/no answer ([CR#608.2d]) — shell.
    Answer(bool),
    /// Answers `ChooseCostOptions`: one chosen reading per choosable symbol, in
    /// the order they appear in the cost ([CR#601.2b]).
    CostOptions(crate::cost_options::CostOptionChoices),
    /// Answers `Priority`.
    Act(Action),
    /// Answers `DiscardToHandSize` and `DiscardCards`: which cards to discard.
    Discard(Vec<ObjectId>),
    /// Answers `ChooseManaColor`: the chosen mana.
    ManaColor(deckmaste_core::ColorOrColorless),
    /// Answers `ChooseTargets`: one chosen object per `TargetSpec`.
    Targets(Vec<ObjectId>),
    /// Answers `PayMana`: how the pool covers the cost.
    Pay(crate::cast::Payment),
    /// Answers `OrderTriggers`: a permutation of `0..triggers.len()` giving the
    /// placement order ([CR#603.3b]).
    Order(Vec<usize>),
    /// Answers `DeclareAttackers`: which creatures attack (possibly empty).
    Attackers(Vec<ObjectId>),
    /// Answers `DeclareBlockers`: `(blocker, the attacker it blocks)` pairs
    /// (possibly empty). Each blocker blocks exactly one attacker
    /// ([CR#509.1a]).
    Blocks(Vec<(ObjectId, ObjectId)>),
    /// Answers `AssignCombatDamage`: `(recipient, amount)` pairs whose amounts
    /// sum to the source's power, each recipient drawn from the offered set
    /// ([CR#510.1c]).
    Assignment(Vec<(ObjectId, Uint)>),
    /// Answers `ChooseObjects`: the chosen objects ([CR#608.2d]).
    Chosen(Vec<ObjectId>),
    /// Answers `ChooseXValue`: the chosen value of X ([CR#601.2b]).
    XValue(Uint),
}

/// What a priority holder can do in the skeleton.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Action {
    Pass,
    /// Concede ([CR#104.3a]) — immediate and UNSTOPPABLE: the single
    /// exception to card-beats-rules ([CR#101.1]); no `CantLose` gate
    /// touches it and a controlled player's controller can't prevent it
    /// ([CR#723.6]). ENUMERATED in every priority legal list and accepted
    /// as the answer to EVERY pending decision (the `submit_decision`
    /// pre-check) — "at any time" means the engine always offers it;
    /// filtering is the runner's problem. The conceder is the pending
    /// decision's decider; out-of-band concession by a player who is NOT
    /// being asked anything is a runner-API seam.
    Concede,
    /// A special action ([CR#116.2]) — P0.W3 shell; never offered in the
    /// legal list yet, so submissions reject as Illegal.
    Special(SpecialAction),
    /// Special action, no stack ([CR#116.2a,305]).
    PlayLand {
        object: ObjectId,
    },
    /// Activate an ability of a permanent the player controls ([CR#602]).
    /// Mana abilities resolve stacklessly ([CR#605.3a]); all other activated
    /// abilities go on the stack via the announce flow ([CR#602.2]). `ability`
    /// indexes the object's derived ability list.
    ActivateAbility {
        object: ObjectId,
        ability: usize,
    },
    /// Cast a spell from hand ([CR#601]). The announce block (targets, cost) is
    /// reified onto the agenda by `take_priority_action`.
    CastSpell {
        object: ObjectId,
    },
}

/// Why a submission was rejected.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DecisionError {
    NothingPending,
    /// The decision kind doesn't answer the pending decision.
    WrongKind,
    Illegal {
        reason: String,
    },
}

impl fmt::Display for DecisionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DecisionError::NothingPending => f.write_str("no decision is pending"),
            DecisionError::WrongKind => f.write_str("decision doesn't answer what's pending"),
            DecisionError::Illegal { reason } => write!(f, "illegal: {reason}"),
        }
    }
}

impl std::error::Error for DecisionError {}

/// The action one `unless` cost component performs, paid by `who`
/// ([CR#118.12a]). v1 covers verb costs (`Do`) and {T}/{Q}; a mid-resolution
/// mana cost is a loud seam (the PayCost/PayMana flow is announce-slot-bound).
fn unless_cost_action(
    component: &deckmaste_core::CostComponent,
    who: &deckmaste_core::Reference,
) -> deckmaste_core::Action {
    use deckmaste_core::Action;
    use deckmaste_core::CostComponent;
    use deckmaste_core::PlayerAction;
    use deckmaste_core::Reference;
    use deckmaste_core::Selection;
    match component {
        // A verb cost is paid by `who` performing it ([CR#601.2h]).
        CostComponent::Do(pa) => Action::By(who.clone(), (**pa).clone()),
        // {T}/{Q} tap/untap the source permanent the cost rides on.
        CostComponent::Tap => Action::By(
            who.clone(),
            PlayerAction::Tap(Selection::Ref(Reference::This)),
        ),
        CostComponent::Untap => Action::By(
            who.clone(),
            PlayerAction::Untap(Selection::Ref(Reference::This)),
        ),
        CostComponent::Expanded(e) => unless_cost_action(&e.value, who),
        CostComponent::Mana(_) => todo!(
            "engine-resolve-effects seam: a mid-resolution mana 'unless' cost \
             ([CR#118.12a]) — the PayCost/PayMana flow is announce-slot-bound"
        ),
    }
}

use deckmaste_core::Agency;
use deckmaste_core::Zone;

use crate::agenda::WorkItem;
use crate::derive;
use crate::event::Cause;
use crate::event::GameEvent;
use crate::event::Occurrence;
use crate::state::GameState;

impl GameState {
    /// Answers the pending decision: validates, does the decision's
    /// bookkeeping, schedules the continuation at the agenda front, and
    /// clears `pending`. On error the decision stays pending.
    ///
    /// # Errors
    ///
    /// `NothingPending` with no decision open, `WrongKind` when the answer
    /// doesn't match the question, `Illegal` when it fails validation.
    ///
    /// # Panics
    ///
    /// Panics if a `ChooseTargets` decision is answered while no announce is in
    /// flight — an engine invariant (the announce slot is open across the
    /// decision), not caller input.
    // TODO: split by decision kind (priority / cast-procedure / combat) as the
    //   match grows past one screen.
    #[expect(clippy::too_many_lines)]
    pub fn submit_decision(&mut self, decision: Decision) -> Result<(), DecisionError> {
        let Some(pending) = &self.pending else {
            return Err(DecisionError::NothingPending);
        };
        // [CR#104.3a] "at any time": conceding answers EVERY decision —
        // the decider walks away mid-discard, mid-targeting, mid-payment.
        // The conceder is the pending decision's decider.
        if matches!(decision, Decision::Act(Action::Concede)) {
            let player = pending.decider_player();
            self.pending = None;
            self.concede(player);
            return Ok(());
        }
        match (pending, decision) {
            (PendingDecision::Priority { player, legal }, Decision::Act(action)) => {
                if !legal.contains(&action) {
                    return Err(DecisionError::Illegal {
                        reason: format!("{action:?} is not a legal action right now"),
                    });
                }
                let player = *player;
                self.pending = None;
                self.take_priority_action(player, &action);
                Ok(())
            }
            (
                PendingDecision::DiscardToHandSize { player, count }
                | PendingDecision::DiscardCards { player, count },
                Decision::Discard(objects),
            ) => {
                let (player, count) = (*player, *count);
                self.submit_discards(player, count, objects)
            }
            (
                PendingDecision::ChooseManaColor {
                    player,
                    options,
                    amount,
                    riders,
                },
                Decision::ManaColor(mana),
            ) => {
                // [CR#106.1b]: the choice is drawn from the offered set.
                if !options.contains(&mana) {
                    return Err(DecisionError::Illegal {
                        reason: format!("{mana:?} is not one of the offered mana options"),
                    });
                }
                let (player, amount, riders) = (*player, *amount, riders.clone());
                self.pending = None;
                self.schedule_front(vec![WorkItem::Emit(Occurrence::single(
                    GameEvent::ManaAdded {
                        player,
                        mana,
                        amount,
                        riders,
                    },
                ))]);
                Ok(())
            }
            (
                PendingDecision::ChooseTargets {
                    player: _,
                    spec,
                    legal,
                },
                Decision::Targets(chosen),
            ) => {
                // [CR#601.2c,115] / [CR#603.3d]: one chosen object per spec,
                // each drawn from that spec's legal candidate set.
                if chosen.len() != spec.len()
                    || chosen.iter().zip(legal).any(|(c, set)| !set.contains(c))
                {
                    return Err(DecisionError::Illegal {
                        reason: "illegal target selection".into(),
                    });
                }
                // Targeting requirements (Must(Target) rows — the
                // Flagbearer class, "must choose at least one … if able"):
                // for each row whose `by` matches the targeting object,
                // if any spec's candidate set holds an `on`-matching
                // object, the chosen targets must include at least one.
                // Disjoint multi-row conflicts would need the maximize
                // arbitration (identical rows — the printed class — are
                // jointly satisfied by one choice, so per-row checks are
                // exact today). Triggered abilities are exempt by the
                // printed wording, but `by` can't spell that
                // discrimination yet — a row matching a placing trigger's
                // source is a LOUD seam, not an evaluation.
                let view = self.layers();
                let must_rows = crate::legal::must_target_rows(self, &view);
                if !must_rows.is_empty() {
                    // Both staging slots carry a real stack identity: a
                    // placing trigger's is minted at placement
                    // ([CR#603.3d]), an announce's when it opened
                    // ([CR#602.2a] / a spell's own id).
                    let targeting = match &self.placing_trigger {
                        Some(t) => t.id,
                        None => self.announcing.as_ref().expect("an announce in flight").id,
                    };
                    for (carrier, by, on) in &must_rows {
                        if !self.filter_matches_live(by, targeting, *carrier) {
                            continue;
                        }
                        if self.placing_trigger.is_some() {
                            todo!(
                                "Must(Target) row matching a triggered ability — the by-filter \
                                 can't exempt triggers yet"
                            );
                        }
                        let able = legal.iter().any(|set| {
                            set.iter()
                                .any(|&t| self.filter_matches_live(on, t, *carrier))
                        });
                        let obeyed = chosen
                            .iter()
                            .any(|&t| self.filter_matches_live(on, t, *carrier));
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
                self.pending = None;
                // [CR#601.2c]: the chosen objects become targets NOW — one
                // fact per distinct target (an object chosen for two specs
                // becomes the target once), simultaneous as one occurrence.
                // The targeting object: a placing trigger's minted stack id
                // ([CR#603.3d]), the announcing spell itself (stack zone —
                // its remint is the deferred one), or an ability announce's
                // SOURCE ([CR#602.2a] — no stack id until promote).
                let targeting = if let Some(staged) = &self.placing_trigger {
                    staged.id
                } else {
                    match &self
                        .announcing
                        .as_ref()
                        .expect("an announce in flight")
                        .object
                    {
                        crate::stack::StackObject::Spell(o) => *o,
                        crate::stack::StackObject::Activated { source, .. } => *source,
                        crate::stack::StackObject::Triggered { .. } => {
                            unreachable!("triggers choose targets at placement, not announce")
                        }
                    }
                };
                let mut became: Vec<GameEvent> = Vec::new();
                for &target in &chosen {
                    let dup = became.iter().any(
                        |e| matches!(e, GameEvent::BecameTarget { target: t, .. } if *t == target),
                    );
                    if !dup {
                        became.push(GameEvent::BecameTarget {
                            target,
                            source: targeting,
                        });
                    }
                }
                if self.placing_trigger.is_some() {
                    // [CR#603.3d]: a triggered ability chose its targets at
                    // placement — commit it onto the stack and resume placement.
                    self.commit_placing_trigger(chosen);
                    self.schedule_front(vec![WorkItem::CheckSbas, WorkItem::PlaceTriggers]);
                } else {
                    self.announcing
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
                self.schedule_front(vec![WorkItem::Emit(occ)]);
                Ok(())
            }
            (
                PendingDecision::PayMana {
                    player,
                    cost,
                    pool: _,
                    subject,
                },
                Decision::Pay(payment),
            ) => {
                let player = *player;
                let cost = cost.clone();
                let subject = *subject;
                // [CR#106.6]: layer SpendOnly spendability on the structural
                // coverage check — each selected unit must be spendable on the
                // object being paid for.
                if !self.validate_spendable(player, &cost, &payment, subject) {
                    return Err(DecisionError::Illegal {
                        reason: "payment does not cover the cost".into(),
                    });
                }
                self.pending = None;
                crate::cast::apply_payment(&mut self.player_mut(player).mana_pool, &payment);
                Ok(())
            }
            (PendingDecision::OrderTriggers { player, triggers }, Decision::Order(order)) => {
                let (player, triggers) = (*player, triggers.clone());
                self.submit_order_triggers(player, &triggers, &order)
            }
            (PendingDecision::DeclareAttackers { player, legal }, Decision::Attackers(chosen)) => {
                // [CR#508.1a]: each chosen creature must be in the surfaced
                // legal set, and no creature attacks twice.
                let distinct: HashSet<_> = chosen.iter().copied().collect();
                if distinct.len() != chosen.len() || !chosen.iter().all(|o| legal.contains(o)) {
                    return Err(DecisionError::Illegal {
                        reason: "attackers must be distinct, from the legal set".into(),
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
                let view = self.layers();
                let rows = crate::legal::must_attack_rows(self, &view);
                let defender_proxy = self
                    .players
                    .iter()
                    .find(|p| p.id != *player)
                    .map(|p| p.object);
                if let Some(&required) = legal.iter().find(|&&c| {
                    !chosen.contains(&c)
                        && rows.iter().any(|(carrier, by, on)| {
                            self.filter_matches_live(by, c, *carrier)
                                && defender_proxy
                                    .is_some_and(|d| self.filter_matches_live(on, d, *carrier))
                        })
                }) {
                    return Err(DecisionError::Illegal {
                        reason: format!(
                            "a Must(Attack) requirement obliges {required:?} to attack"
                        ),
                    });
                }
                self.pending = None;
                // [CR#508.1f]: declaring taps the attacker. The whole declaration
                // is one simultaneous occurrence — a `Batch` (empty when no
                // attackers were declared, which schedules nothing observable).
                if !chosen.is_empty() {
                    self.schedule_front(vec![WorkItem::Emit(Occurrence::Batch(
                        chosen.into_iter().map(GameEvent::Attacking).collect(),
                    ))]);
                }
                Ok(())
            }
            (PendingDecision::DeclareBlockers { player: _, legal }, Decision::Blocks(pairs)) => {
                // [CR#509.1a]: each blocker is from the surfaced legal set, each
                // blocked creature is an attacker, and no creature blocks twice
                // (a creature blocks exactly one attacker).
                let distinct: HashSet<_> = pairs.iter().map(|&(b, _)| b).collect();
                let attackers = self.combat.attackers();
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
                let view = self.layers();
                let rows = crate::legal::cant_block_rows(self, &view);
                for &(blocker, attacker) in &pairs {
                    if let Some(carrier) =
                        crate::legal::block_forbidden_by(self, &rows, blocker, attacker)
                    {
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
                        crate::legal::arrangement_forbidden_by(self, &rows, *attacker, blockers)
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
                let must_rows = crate::legal::must_block_rows(self, &view);
                for &b in legal {
                    let mut demanded: Vec<Vec<ObjectId>> = Vec::new();
                    for row in &must_rows {
                        if row.count.is_some() {
                            todo!("a Must(Block) row carrying an arrangement bound");
                        }
                        if !self.filter_matches_live(&row.by, b, row.carrier) {
                            continue;
                        }
                        let set: Vec<ObjectId> = attackers
                            .iter()
                            .copied()
                            .filter(|&a| self.filter_matches_live(&row.on, a, row.carrier))
                            .filter(|&a| {
                                crate::legal::block_forbidden_by(self, &rows, b, a).is_none()
                            })
                            .collect();
                        let bounded = set.iter().any(|&a| {
                            rows.iter().any(|r| {
                                r.count.is_some() && self.filter_matches_live(&r.on, a, r.carrier)
                            })
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
                self.pending = None;
                // [CR#509.1h]: the whole block declaration is one simultaneous
                // occurrence — a `Batch` (skipped when empty, which schedules
                // nothing observable).
                if !pairs.is_empty() {
                    self.schedule_front(vec![WorkItem::Emit(Occurrence::Batch(
                        pairs
                            .into_iter()
                            .map(|(blocker, attacker)| GameEvent::Blocked { blocker, attacker })
                            .collect(),
                    ))]);
                }
                Ok(())
            }
            (
                PendingDecision::AssignCombatDamage {
                    player: _,
                    source,
                    recipients,
                },
                Decision::Assignment(amounts),
            ) => {
                let source = *source;
                let recipients = recipients.clone();
                self.submit_assign_combat_damage(source, &recipients, amounts)
            }
            (
                PendingDecision::ChooseObjects {
                    candidates,
                    min,
                    max,
                    ..
                },
                Decision::Chosen(chosen),
            ) => {
                // [CR#608.2d]: distinct, all from the offered set, count in range.
                let chosen_count = Uint::try_from(chosen.len()).expect("chosen count fits Uint");
                let distinct: HashSet<_> = chosen.iter().copied().collect();
                let legal = distinct.len() == chosen.len()
                    && chosen_count >= *min
                    && chosen_count <= *max
                    && chosen.iter().all(|id| candidates.contains(id));
                if !legal {
                    return Err(DecisionError::Illegal {
                        reason: "illegal object selection".into(),
                    });
                }
                // All reads of `pending` are done; safe to mutate self.
                self.pending = None;
                let crate::state::ChoiceContinuation::BindChoice { effect, mut frame } = self
                    .choice
                    .take()
                    .expect("a ChooseObjects decision stashed its continuation")
                else {
                    unreachable!("a ChooseObjects decision stashes a BindChoice continuation");
                };
                frame.chosen = Some(chosen);
                self.schedule_front(vec![WorkItem::RunEffect { effect, frame }]);
                Ok(())
            }
            (PendingDecision::ChooseXValue { player }, Decision::XValue(x)) => {
                let player = *player;
                // [CR#601.2b]: record the announced value in the open slot.
                self.announcing
                    .as_mut()
                    .expect("an announce in flight for ChooseXValue")
                    .x = Some(x);
                // [CR#601.2h,733]: an unpayable announcement reverses the cast.
                // Read the kind + base cost immutably, then decide.
                let pending = self.announcing.as_ref().expect("an announce in flight");
                let (subject, base) = match &pending.object {
                    crate::stack::StackObject::Spell(o) => {
                        (*o, self.mana_cost(*o).expect("a castable spell has a cost"))
                    }
                    crate::stack::StackObject::Activated {
                        source, ability, ..
                    } => (
                        *source,
                        crate::activate::cost_summary(&ability.cost)
                            .expect("can_activate vetted the cost")
                            .mana,
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
                let payable = self.affordable_concretization(
                    player,
                    &crate::cast::concretize_x(&base, x),
                    subject,
                );
                self.pending = None;
                // Writing `x` first is safe: `rewind_announce` discards the
                // whole announcing slot, including the `x` just written.
                if !payable {
                    self.rewind_announce();
                }
                Ok(())
            }
            (PendingDecision::ChooseCostOptions { cost, .. }, Decision::CostOptions(choices)) => {
                // [CR#601.2b]: apply the announced readings to the printed cost.
                // An illegal announce (wrong pick count, or a reading the symbol
                // doesn't offer) is rejected — the decision stays pending.
                let cost = cost.clone();
                let concrete = match crate::cost_options::concretize(&cost, &choices) {
                    Ok(c) => c,
                    Err(e) => {
                        return Err(DecisionError::Illegal {
                            reason: format!("illegal cost-option announce: {e:?}"),
                        });
                    }
                };
                // Stash the concretized (mana, Phyrexian-life verbs) on the
                // announce slot for `PayCost` to consume.
                self.announcing
                    .as_mut()
                    .expect("an announce is in flight across ChooseCostOptions")
                    .concretized = Some(concrete);
                self.pending = None;
                Ok(())
            }
            (PendingDecision::YesNo { .. }, Decision::Answer(yes)) => {
                self.pending = None;
                let cont = self
                    .choice
                    .take()
                    .expect("a YesNo decision stashed its continuation");
                match cont {
                    // [CR#118.12]: `Effect::May` — yes runs `effect` then
                    // `if_did`; no runs `if_not` (or nothing). Front-scheduled
                    // in order so `effect` precedes `if_did`.
                    crate::state::ChoiceContinuation::May { may, frame } => {
                        let branch: Vec<Box<deckmaste_core::Effect>> = if yes {
                            std::iter::once(may.effect).chain(may.if_did).collect()
                        } else {
                            may.if_not.into_iter().collect()
                        };
                        let items = branch
                            .into_iter()
                            .map(|effect| WorkItem::RunEffect {
                                effect,
                                frame: frame.clone(),
                            })
                            .collect();
                        self.schedule_front(items);
                    }
                    // [CR#118.12a,608.2d]: `Effect::Unless` — yes pays the cost
                    // (each component as the payer's action) and `effect` is
                    // skipped; no runs `effect`.
                    crate::state::ChoiceContinuation::Unless {
                        effect,
                        who,
                        unless,
                        frame,
                    } => {
                        let items: Vec<WorkItem> = if yes {
                            unless
                                .iter()
                                .map(|c| WorkItem::RunEffect {
                                    effect: Box::new(deckmaste_core::Effect::Act(
                                        unless_cost_action(c, &who),
                                    )),
                                    frame: frame.clone(),
                                })
                                .collect()
                        } else {
                            vec![WorkItem::RunEffect { effect, frame }]
                        };
                        self.schedule_front(items);
                    }
                    other => {
                        unreachable!("a YesNo decision stashed a non-YesNo continuation: {other:?}")
                    }
                }
                Ok(())
            }
            (
                PendingDecision::ChooseModes {
                    options,
                    min,
                    max,
                    repeats,
                    ..
                },
                Decision::Modes(picks),
            ) => {
                let (options, min, max, repeats) = (*options, *min, *max, *repeats);
                // [CR#700.2,700.2d]: count in [min,max], each index a real mode,
                // distinct unless the same mode may be chosen more than once.
                let n = Uint::try_from(picks.len()).expect("pick count fits Uint");
                let distinct = repeats || {
                    let set: HashSet<_> = picks.iter().copied().collect();
                    set.len() == picks.len()
                };
                let legal = n >= min && n <= max && picks.iter().all(|&i| i < options) && distinct;
                if !legal {
                    return Err(DecisionError::Illegal {
                        reason: "illegal mode selection".into(),
                    });
                }
                self.pending = None;
                let crate::state::ChoiceContinuation::Modal { modes, frame } = self
                    .choice
                    .take()
                    .expect("a ChooseModes decision stashed its continuation")
                else {
                    unreachable!("a ChooseModes decision stashes a Modal continuation");
                };
                // [CR#700.2]: apply the chosen modes' effects in pick order.
                let items = picks
                    .into_iter()
                    .map(|i| WorkItem::RunEffect {
                        effect: Box::new(modes[i as usize].effect.clone()),
                        frame: frame.clone(),
                    })
                    .collect();
                self.schedule_front(items);
                Ok(())
            }
            (
                PendingDecision::Division { .. }
                | PendingDecision::Vote { .. }
                | PendingDecision::PreGame { .. }
                | PendingDecision::OrderReplacements { .. },
                _,
            ) => todo!("P0.W4/W7: submission handling for shell decision kinds"),
            _ => Err(DecisionError::WrongKind),
        }
    }

    /// Shared by `DiscardToHandSize` ([CR#514.1]) and `DiscardCards`
    /// ([CR#701.9b]): validate that exactly `count` distinct in-hand cards were
    /// chosen, then emit one `Discarded` per card.
    ///
    /// # Errors
    ///
    /// `Illegal` when the chosen cards are not exactly `count` distinct cards
    /// from the player's hand.
    fn submit_discards(
        &mut self,
        player: PlayerId,
        count: Uint,
        objects: Vec<ObjectId>,
    ) -> Result<(), DecisionError> {
        let hand = &self.zones.hands[player.index()];
        let distinct: HashSet<_> = objects.iter().copied().collect();
        if objects.len() != count as usize
            || distinct.len() != objects.len()
            || !objects.iter().all(|o| hand.contains(o))
        {
            return Err(DecisionError::Illegal {
                reason: format!("discard exactly {count} distinct cards from hand"),
            });
        }
        self.pending = None;
        // [CR#701.9a]: discard = Hand→Graveyard carrying its cause triple
        // (the named "Discard" view; agency per demand site — this decision
        // serves effect-instructed and cleanup discards alike, so the
        // turn-based flavor rides the cleanup caller's context for now).
        self.schedule_front(
            objects
                .into_iter()
                .map(|object| {
                    WorkItem::Emit(Occurrence::single(GameEvent::ZoneWillChange {
                        object,
                        from: Some(Zone::Hand),
                        to: Zone::Graveyard,
                        enters: None,
                        position: None,
                        face: None,
                        cause: Some(Cause::discard(Agency::EffectInstruction, None)),
                    }))
                })
                .collect(),
        );
        Ok(())
    }

    /// [CR#603.3b]: apply an `OrderTriggers` answer — validate `order` is a
    /// permutation of `0..triggers.len()`, reorder this player's noted
    /// triggers, and immediately place the FIRST one. Placement of subsequent
    /// triggers is deferred to future `PlaceTriggers` barrier passes (one per
    /// `step()`), each preceded by a `CheckSbas` re-sweep.
    ///
    /// Placing the first trigger here (instead of rescheduling `PlaceTriggers`
    /// and letting it see `mine.len() > 1` again) avoids the `OrderTriggers`
    /// re-surface loop.
    ///
    /// # Errors
    ///
    /// `Illegal` when `order` is not a permutation of the offered indices.
    fn submit_order_triggers(
        &mut self,
        player: PlayerId,
        triggers: &[crate::trigger::NotedTrigger],
        order: &[usize],
    ) -> Result<(), DecisionError> {
        let len = triggers.len();
        let distinct: HashSet<usize> = order.iter().copied().collect();
        if order.len() != len || distinct.len() != len || order.iter().any(|&i| i >= len) {
            return Err(DecisionError::Illegal {
                reason: format!("order must be a permutation of 0..{len}"),
            });
        }
        // Reorder this player's noted triggers to the chosen order, leaving
        // other players' notes untouched.
        let ordered: Vec<crate::trigger::NotedTrigger> =
            order.iter().map(|&i| triggers[i].clone()).collect();
        self.pending = None;
        self.reorder_pending_triggers(player, ordered);
        // Immediately place the FIRST trigger in the ordered sequence so the
        // next `PlaceTriggers` call sees one fewer trigger and does not
        // re-surface `OrderTriggers`. Schedule CheckSbas + PlaceTriggers so
        // subsequent triggers (and any new SBA state) are handled normally.
        let noted = self.take_first_trigger_of(player);
        let placed = self.place_one_trigger(noted);
        self.schedule_front(vec![WorkItem::CheckSbas, WorkItem::PlaceTriggers]);
        let _ = placed; // placement success is tracked by CheckSbas/PlaceTriggers
        Ok(())
    }

    /// [CR#510.1c]: apply an `AssignCombatDamage` answer for the queue's front
    /// source. Validates that the amounts sum to the source's power and every
    /// named target is one of its recipients. Ordinary (non-trample) division
    /// is free — *any* such split is legal ([CR#510.1c]). For a **trample**
    /// source whose recipients include the defending player's proxy,
    /// [CR#702.19b] additionally requires that the player be assigned
    /// damage only once every blocker recipient has at least lethal —
    /// `lethal` is 1 when the source has deathtouch ([CR#702.2c]) and the
    /// blocker's toughness otherwise (less any damage already marked). On
    /// success it appends one `DamageDealt` per nonzero amount to the
    /// buffer, pops the source off the queue, and surfaces
    /// the next decision (or deals the batch when the queue empties) via
    /// `open_next_assignment`.
    ///
    /// # Errors
    ///
    /// `Illegal` when the amounts don't sum to the source's power, name a
    /// creature that isn't one of the source's recipients, repeat a recipient,
    /// or (trample) assign the player while a blocker recipient is below
    /// lethal.
    ///
    /// # Panics
    ///
    /// Panics if no combat-damage assignment is in flight, or the front source
    /// doesn't match `source` — engine invariants (the pending decision pins
    /// both), not caller input.
    fn submit_assign_combat_damage(
        &mut self,
        source: ObjectId,
        recipients: &[ObjectId],
        amounts: Vec<(ObjectId, Uint)>,
    ) -> Result<(), DecisionError> {
        let power = {
            let cd = self
                .combat_damage
                .as_ref()
                .expect("combat-damage in flight");
            let front = cd.queue.first().expect("a queued assignment");
            debug_assert_eq!(
                front.source, source,
                "front source matches the pending decision"
            );
            front.power
        };
        let total: Uint = amounts.iter().map(|&(_, n)| n).sum();
        if total != power {
            return Err(DecisionError::Illegal {
                reason: format!(
                    "assigned damage ({total}) must sum to the source's power ({power})"
                ),
            });
        }
        if !amounts.iter().all(|(t, _)| recipients.contains(t)) {
            return Err(DecisionError::Illegal {
                reason: "every assignment target must be one of the source's recipients".into(),
            });
        }
        let distinct: HashSet<ObjectId> = amounts.iter().map(|&(t, _)| t).collect();
        if distinct.len() != amounts.len() {
            return Err(DecisionError::Illegal {
                reason: "each recipient may appear at most once in a damage assignment".into(),
            });
        }
        // [CR#702.19b]: a trample source may assign damage to the defending
        // player only after every blocker recipient has lethal. Player proxies
        // among the recipients are identified by their `ObjectSource::Player`.
        let view = self.layers();
        if crate::combat::has_keyword(&view, source, &KeywordAbility::Trample) {
            let assigned = |id: ObjectId| {
                amounts
                    .iter()
                    .find(|&&(t, _)| t == id)
                    .map_or(0, |&(_, n)| n)
            };
            let to_player: Uint = recipients
                .iter()
                .filter(|&&r| matches!(self.objects.obj(r).source, ObjectSource::Player(_)))
                .map(|&r| assigned(r))
                .sum();
            if to_player > 0 {
                for &r in recipients {
                    if matches!(self.objects.obj(r).source, ObjectSource::Player(_)) {
                        continue; // the player, not a blocker
                    }
                    if assigned(r) < self.lethal_for(&view, source, r) {
                        return Err(DecisionError::Illegal {
                            reason: "trample: each blocker must be assigned lethal before the \
                                     defending player ([CR#702.19b])"
                                .into(),
                        });
                    }
                }
            }
        }
        self.pending = None;
        let cd = self
            .combat_damage
            .as_mut()
            .expect("combat-damage in flight");
        for (target, amount) in amounts {
            if amount > 0 {
                cd.buffer.push(GameEvent::DamageDealt {
                    source,
                    target,
                    amount,
                });
            }
        }
        cd.queue.remove(0);
        self.open_next_assignment();
        Ok(())
    }

    /// [CR#702.19b]: how much damage `source` must assign to the creature
    /// `blocker` before any excess may spill to the defending player. With
    /// deathtouch on the source, any nonzero amount is lethal, so `1`
    /// ([CR#702.2c]); otherwise the blocker's printed toughness, less any
    /// damage already marked on it (undamaged at assignment time in v1, but
    /// subtracted for correctness). A non-`Number` toughness (CDA / `*`) is
    /// treated as needing the source's whole power — effectively
    /// unsatisfiable below full assignment — by returning `Uint::MAX`;
    /// layers and `*`-toughness are a later stage.
    #[must_use]
    fn lethal_for(
        &self,
        view: &crate::layer::LayeredView,
        source: ObjectId,
        blocker: ObjectId,
    ) -> Uint {
        if crate::combat::has_keyword(view, source, &KeywordAbility::Deathtouch) {
            return 1; // [CR#702.2c]: any nonzero amount is lethal.
        }
        match view.toughness(blocker) {
            Some(t) if t > 0 => {
                #[expect(clippy::cast_sign_loss)]
                let toughness = t as Uint;
                toughness.saturating_sub(self.objects.obj(blocker).damage)
            }
            // toughness ≤ 0 is already a destroy SBA; None is a non-creature
            // or unmodeled case — require the full amount so the player can't
            // be reached.
            _ => Uint::MAX,
        }
    }

    /// The priority bookkeeping ([CR#117.3c,117.4]): a pass rotates or ends
    /// the round; an action emits, re-runs the barrier, and re-opens
    /// priority for the actor. Legality was checked by the caller.
    ///
    /// # Panics
    ///
    /// Panics if no priority round is open — engine invariant, not caller
    /// input.
    /// [CR#104.3a]: the loss is immediate and unstoppable ([CR#101.1]);
    /// `check_game_end` then terminalizes ([CR#104.1] — in two-player, the
    /// first loss ends the whole game). The multiplayer leave-game cleanup
    /// ([CR#800.4a]: owned objects leave, control effects end, residue
    /// exiled) is unbuilt — loud rather than a half-departed player
    /// haunting the table.
    fn concede(&mut self, player: PlayerId) {
        if self.live_count() > 2 {
            todo!("P0.W6: multiplayer leave-game cleanup ([CR#800.4a])");
        }
        self.schedule_front(vec![WorkItem::Emit(Occurrence::single(
            GameEvent::PlayerLost {
                player,
                reason: crate::event::LossReason::Conceded,
            },
        ))]);
    }

    fn take_priority_action(&mut self, player: PlayerId, action: &Action) {
        match action {
            // P0.W3 shell: special actions are never in the legal list, so
            // submission already rejected them as Illegal; loud if reached.
            Action::Special(_) => todo!("P0.W3: special actions ([CR#116.2] machinery)"),
            // Normally short-circuited by the `submit_decision` pre-check;
            // kept for exhaustiveness and direct callers.
            Action::Concede => self.concede(player),
            Action::Pass => {
                // Compute before borrowing the round mutably.
                let live = self.live_count();
                let next = self.next_live_after(player);
                let round = self.turn.priority.as_mut().expect("open priority round");
                round.consecutive_passes += 1;
                let all_passed = round.consecutive_passes >= live;
                if all_passed {
                    self.turn.priority = None;
                    if let Some(top) = self.stack.last() {
                        // [CR#608]: resolve the top; AP gets priority after
                        // ([CR#117.3b]). Keyed on `StackEntry.id` so a triggered
                        // ability (no backing object) resolves like a spell.
                        let id = top.id;
                        self.schedule_front(vec![
                            WorkItem::Resolve(id),
                            WorkItem::CheckSbas,
                            WorkItem::PlaceTriggers,
                            WorkItem::OpenPriority,
                        ]);
                    } else {
                        // [CR#117.4]: all-pass on an empty stack ends the step.
                        let items = self.end_of_step_items();
                        self.schedule_front(items);
                    }
                } else {
                    self.turn
                        .priority
                        .as_mut()
                        .expect("open priority round")
                        .holder = next;
                    self.schedule_front(vec![WorkItem::OpenPriority]);
                }
            }
            Action::PlayLand { object } => {
                self.reset_passes();
                // [CR#305.2,116.2a]: the land drop is recorded by the "Play"
                // cause on the move (special action — Agency::SpecialAction; an
                // effect putting a land onto the battlefield is NOT a play,
                // [CR#701.18a]). `LandsPlayedThisTurn` counts those Play-caused
                // battlefield entries in the history log.
                let mut items = vec![WorkItem::Emit(Occurrence::single(
                    GameEvent::ZoneWillChange {
                        object: *object,
                        from: Some(Zone::Hand),
                        to: Zone::Battlefield,
                        enters: None,
                        position: None,
                        face: None,
                        cause: Some(Cause::play(Agency::SpecialAction, None)),
                    },
                ))];
                items.extend(Self::priority_tail());
                self.schedule_front(items);
            }
            Action::ActivateAbility { object, ability } => {
                let abilities = derive::abilities(self, *object);
                let a = abilities.get(*ability).expect(
                    "ability index from the legal list is in bounds (state frozen by pending)",
                );
                self.reset_passes();
                if let Some((mana, amount)) = derive::tap_mana_ability(a) {
                    // [CR#605.3b]: mana abilities skip the stack entirely.
                    // [CR#107.4h]: mana from a snow source (a snow permanent —
                    // one with the Snow supertype) is snow mana; tag the unit so
                    // a later task can pay {S}. The ability text declares no
                    // riders on this path, so the source's snow-ness is the only
                    // contribution.
                    //
                    // NB: `snow_provenance` does a SECOND `layers()` rebuild
                    // (the first was the `derive::abilities` call above). Cheap
                    // per action; revisit if mana activation becomes a profiling
                    // hotspot (the repo has prior `layers()` perf history).
                    let riders = self.snow_provenance(*object);
                    let mut items = vec![
                        WorkItem::Emit(Occurrence::single(GameEvent::Tapped {
                            object: *object,
                            cause: Some(Cause::tap(Agency::CostPayment, None)),
                        })),
                        WorkItem::Emit(Occurrence::single(GameEvent::ManaAdded {
                            player,
                            mana,
                            amount,
                            riders,
                        })),
                    ];
                    items.extend(Self::priority_tail());
                    self.schedule_front(items);
                } else {
                    // [CR#602.2b]: the casting steps, for an ability.
                    let items = Self::announce_schedule(
                        WorkItem::BeginActivate {
                            object: *object,
                            ability: *ability,
                        },
                        GameEvent::AbilityActivated {
                            source: *object,
                            ability: *ability,
                        },
                    );
                    self.schedule_front(items);
                }
            }
            Action::CastSpell { object } => {
                // [CR#601.2]: reify the announce procedure. Targets and cost are
                // chosen by the staged WorkItems (surfacing decisions when
                // there is a choice); `SpellCast` is the becomes-cast moment
                // ([CR#601.2i]) that promotes the announce onto the stack; the
                // caster then regains priority ([CR#117.3c]).
                self.reset_passes();
                let items = Self::announce_schedule(
                    WorkItem::BeginCast(*object),
                    GameEvent::SpellCast(*object),
                );
                self.schedule_front(items);
            }
        }
    }

    /// The trailer every priority-restarting action ends with: re-check
    /// state-based actions ([CR#704.3]), place any waiting triggers
    /// ([CR#603.3]), then re-open priority for the actor ([CR#117.3c]). Shared
    /// by the announce schedule, `PlayLand`, and the mana-ability arm so the
    /// three never drift.
    #[must_use]
    pub(crate) fn priority_tail() -> Vec<WorkItem> {
        vec![
            WorkItem::CheckSbas,
            WorkItem::PlaceTriggers,
            WorkItem::OpenPriority,
        ]
    }

    /// [CR#601.2,602.2]: the full announce procedure, identical for casting a
    /// spell and activating a non-mana ability — only the opening shell
    /// (`BeginCast` vs `BeginActivate`) and the becomes-cast event
    /// (`SpellCast` / `AbilityActivated`) differ. `AnnounceX` precedes targets
    /// ([CR#601.2b,601.2c]); `ChooseCostOptions` concretizes hybrid/Phyrexian
    /// symbols ([CR#601.2b]) before `PayCost`; the becomes-cast event then
    /// promotes the announce onto the stack ([CR#601.2i,602.2a]) and the
    /// shared `priority_tail` re-opens priority for the actor.
    #[must_use]
    pub(crate) fn announce_schedule(begin: WorkItem, cast_event: GameEvent) -> Vec<WorkItem> {
        let mut items = vec![
            begin,
            WorkItem::AnnounceX,
            WorkItem::AnnounceTargets,
            WorkItem::ChooseCostOptions,
            WorkItem::PayCost,
            WorkItem::Emit(Occurrence::single(cast_event)),
        ];
        items.extend(Self::priority_tail());
        items
    }

    /// The per-unit provenance riders a `source` contributes to the mana it
    /// produces. A snow source is a snow PERMANENT — an object on the
    /// battlefield whose DERIVED supertypes include `Snow` ([CR#205.4g]) —
    /// whose mana can pay `{S}` ([CR#107.4h]), so it contributes
    /// `ManaRider::Snow`; anything else (a non-snow permanent, or a
    /// mana-producing spell/ability whose source is not a permanent)
    /// contributes none. Read off the layered view so a granted Snow supertype
    /// counts (rare, but the derived view is the consistent source of truth).
    /// Callers are responsible for merging these with any riders the producing
    /// ability declares.
    pub(crate) fn snow_provenance(&self, source: ObjectId) -> Vec<deckmaste_core::ManaRider> {
        let is_permanent = self.objects.obj(source).zone == Some(Zone::Battlefield);
        let is_snow = self
            .layers()
            .get(source)
            .supertypes
            .contains(&deckmaste_core::Supertype::Snow);
        if is_permanent && is_snow {
            vec![deckmaste_core::ManaRider::Snow]
        } else {
            vec![]
        }
    }

    /// [CR#117.3c]: taking an action restarts the pass count; the actor
    /// receives priority again afterward. `holder` is intentionally not
    /// touched — the scheduled `OpenPriority` reuses it as-is, and it is
    /// already the actor.
    ///
    /// # Panics
    ///
    /// Panics if no priority round is open — engine invariant, not caller
    /// input.
    fn reset_passes(&mut self) {
        self.turn
            .priority
            .as_mut()
            .expect("open priority round")
            .consecutive_passes = 0;
    }
}
