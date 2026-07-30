use std::collections::HashSet;
use std::fmt;
use std::sync::Arc;

use deckmaste_core::DeciderSpec;
use deckmaste_core::KeywordAbility;
use deckmaste_core::LockPoint;
use deckmaste_core::Uint;
use deckmaste_core::Visibility;

use crate::object::ObjectId;
use crate::player::PlayerId;

pub(crate) mod pending;

use pending::ArrangePile;
use pending::AssignCombatDamage;
use pending::CallFlip;
use pending::ChooseCostOptions;
use pending::ChooseManaColor;
use pending::ChooseManaMode;
use pending::ChooseModes;
use pending::ChooseNewTargets;
use pending::ChooseNoteCardName;
use pending::ChooseNoteNumber;
use pending::ChooseObjects;
use pending::ChooseReplacement;
use pending::ChooseTargets;
use pending::ChooseXValue;
use pending::DeclareAttackers;
use pending::DeclareBlockers;
use pending::DiscardCards;
use pending::DiscardToHandSize;
use pending::Division;
use pending::LegendRule;
use pending::OrderReplacements;
use pending::OrderTriggers;
use pending::PayMana;
use pending::PreGame;
use pending::Priority;
use pending::Vote;
use pending::YesNo;

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
            PendingDecision::Priority(_) => DeciderSpec::PriorityHolder,
            PendingDecision::DeclareAttackers(_) => DeciderSpec::ActivePlayer,
            PendingDecision::DeclareBlockers(_) => DeciderSpec::DefendingPlayer,
            PendingDecision::Vote(_) => DeciderSpec::EachInTurnOrder,
            _ => DeciderSpec::Controller,
        }
    }

    /// The CONCRETE player this decision is waiting on (every kind
    /// carries one) — the nominal-role view is [`Self::decider_spec`].
    #[must_use]
    pub fn decider_player(&self) -> PlayerId {
        match self {
            PendingDecision::Priority(h) => h.player,
            PendingDecision::DiscardToHandSize(h) => h.player,
            PendingDecision::DiscardCards(h) => h.player,
            PendingDecision::CallFlip(h) => h.player,
            PendingDecision::ChooseManaColor(h) => h.player,
            PendingDecision::ChooseManaMode(h) => h.player,
            PendingDecision::ChooseTargets(h) => h.player,
            PendingDecision::ChooseNewTargets(h) => h.player,
            PendingDecision::PayMana(h) => h.player,
            PendingDecision::OrderTriggers(h) => h.player,
            PendingDecision::DeclareAttackers(h) => h.player,
            PendingDecision::DeclareBlockers(h) => h.player,
            PendingDecision::AssignCombatDamage(h) => h.player,
            PendingDecision::ChooseModes(h) => h.player,
            PendingDecision::Division(h) => h.player,
            PendingDecision::Vote(h) => h.player,
            PendingDecision::YesNo(h) => h.player,
            PendingDecision::OrderReplacements(h) => h.player,
            PendingDecision::ChooseCostOptions(h) => h.player,
            PendingDecision::ChooseXValue(h) => h.player,
            PendingDecision::ChooseNoteNumber(h) => h.player,
            PendingDecision::ChooseNoteCardName(h) => h.player,
            PendingDecision::ChooseObjects(h) => h.player,
            PendingDecision::PreGame(h) => h.player,
            PendingDecision::LegendRule(h) => h.player,
            PendingDecision::ArrangePile(h) => h.player,
            PendingDecision::ChooseReplacement(h) => h.chooser,
        }
    }

    /// The choices.md §2 lock stage on the shared `LockPoint` axis.
    #[must_use]
    pub fn lock(&self) -> LockPoint {
        match self {
            PendingDecision::ChooseTargets(_)
            | PendingDecision::ChooseModes(_)
            | PendingDecision::ChooseCostOptions(_)
            | PendingDecision::ChooseXValue(_)
            | PendingDecision::Division(_) => LockPoint::Announce,
            PendingDecision::PayMana(_) => LockPoint::Payment,
            PendingDecision::OrderTriggers(_) => LockPoint::StackPlacement,
            PendingDecision::DeclareAttackers(_)
            | PendingDecision::DeclareBlockers(_)
            | PendingDecision::AssignCombatDamage(_) => LockPoint::Declaration,
            PendingDecision::PreGame(_) => LockPoint::PreGame,
            // Priority actions and resolution-stage choices bind as applied.
            _ => LockPoint::Resolution,
        }
    }

    /// The choices.md §3 visibility class. London bottoming is the one
    /// committed-hidden pre-game payload ([CR#103.5]).
    #[must_use]
    pub fn visibility(&self) -> Visibility {
        match self {
            PendingDecision::PreGame(PreGame {
                kind: PreGameKind::Mulligan,
                ..
            }) => Visibility::CommittedHidden,
            _ => Visibility::Open,
        }
    }
}

/// What the engine is waiting on. `step()` returns `NeedsDecision` (without
/// mutating) until `submit_decision` answers it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PendingDecision {
    Priority(Priority),
    DiscardToHandSize(DiscardToHandSize),
    DiscardCards(DiscardCards),
    CallFlip(CallFlip),
    ChooseManaColor(ChooseManaColor),
    ChooseManaMode(ChooseManaMode),
    ChooseTargets(ChooseTargets),
    ChooseNewTargets(ChooseNewTargets),
    PayMana(PayMana),
    OrderTriggers(OrderTriggers),
    DeclareAttackers(DeclareAttackers),
    DeclareBlockers(DeclareBlockers),
    AssignCombatDamage(AssignCombatDamage),
    ChooseModes(ChooseModes),
    Division(Division),
    Vote(Vote),
    YesNo(YesNo),
    ChooseCostOptions(ChooseCostOptions),
    ChooseXValue(ChooseXValue),
    ChooseNoteNumber(ChooseNoteNumber),
    ChooseNoteCardName(ChooseNoteCardName),
    OrderReplacements(OrderReplacements),
    ChooseReplacement(ChooseReplacement),
    PreGame(PreGame),
    ChooseObjects(ChooseObjects),
    LegendRule(LegendRule),
    ArrangePile(ArrangePile),
}

/// One pending-decision kind's answer-validate-and-apply behavior, dispatched
/// by `submit_decision`'s routing match — the whole dispatch, one arm per
/// `PendingDecision` variant. Each newtype payload struct under `pending/`
/// implements exactly this one method.
pub(crate) trait DecisionHandler {
    /// Validate `answer` against this pending decision and apply it. On success
    /// the handler sets `g.pending = None`; on ANY error it leaves `g.pending`
    /// untouched (`submit_decision`'s contract: a rejected decision stays
    /// open).
    fn resolve(self, g: &mut GameState, answer: Decision) -> Result<(), DecisionError>;
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
    /// Answers `ChooseManaMode`: the chosen multi-symbol run, by option index
    /// into the offered runs ([CR#106.1b]).
    ManaMode(Uint),
    /// Answers `ChooseTargets`: one chosen SET per `TargetSpec` slot (singleton
    /// for a quantity-one slot, several for a plural slot). The submission
    /// carries the per-slot counts, so `submit_decision` locks them
    /// ([CR#601.2c]).
    Targets(Vec<Vec<ObjectId>>),
    /// Answers `PayMana`: how the pool covers the cost.
    Pay(crate::cast::Payment),
    /// Answers `OrderTriggers`: a permutation of `0..triggers.len()` giving the
    /// placement order ([CR#603.3b]).
    Order(Vec<usize>),
    /// Answers `DeclareAttackers` ([CR#508.1a,508.1b]): `(attacker, target)`
    /// pairs (possibly empty). Each `target` is what that attacker is attacking
    /// — the defending player's proxy object or a planeswalker they control
    /// ([CR#506.3,508.1b]).
    Attackers(Vec<(ObjectId, ObjectId)>),
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
    /// Answers `ArrangePile` ([CR#401.4]): the pile's cards in the chosen order
    /// (top → down) — a permutation of the offered pile.
    Arranged(Vec<ObjectId>),
    /// Answers `ChooseXValue`: the chosen value of X ([CR#601.2b]).
    XValue(Uint),
    /// Answers `ChooseNoteCardName`.
    CardName(String),
    /// Answers `ChooseReplacement` ([CR#616.1]): which replacement the affected
    /// player applies first. The key must be in the `applicable` list.
    ReplacementChoice(crate::replace_registry::ReplacementKey),
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
pub(crate) fn unless_cost_action(
    component: &deckmaste_core::CostComponent,
    who: &deckmaste_core::Reference,
) -> deckmaste_core::Action {
    use deckmaste_core::Action;
    use deckmaste_core::CostComponent;
    use deckmaste_core::PlayerAction;
    use deckmaste_core::Reference;
    match component {
        // A verb cost is paid by `who` performing it ([CR#601.2h]) — re-agent
        // the implicit-you action onto `who`: a player verb swaps its `By`
        // agent; the chosen discard composite ([CR#701.9]) is rebuilt around
        // `who` (who rides the body's `With` binder's `filter`/`by`), while the bound
        // "discard this card" form ([CR#702.29a]) is paid by its patient's own
        // controller, so it needs only the named card.
        CostComponent::Do(action) => match &**action {
            Action::By(_, pa) => Action::By(who.clone(), pa.clone()),
            Action::Composite { name, body } if name.as_str() == "Discard" => {
                match deckmaste_core::discard_body_what(body) {
                    // The bound form's performer is its patient's controller
                    // (`who` discards their own named card), so the re-agented
                    // rebuild needs only the card ([CR#702.29a]).
                    Some(what) => Action::discard_what(what.clone()),
                    None => Action::discard(
                        who.clone(),
                        deckmaste_core::discard_body_count(body)
                            .cloned()
                            .unwrap_or(deckmaste_core::Count::Literal(0)),
                        deckmaste_core::discard_body_random(body),
                    ),
                }
            }
            other => other.clone(),
        },
        // {T}/{Q} tap/untap the source permanent the cost rides on.
        CostComponent::Tap => Action::By(who.clone(), PlayerAction::Tap(Reference::This)),
        CostComponent::Untap => Action::By(who.clone(), PlayerAction::Untap(Reference::This)),
        CostComponent::Expanded(e) => unless_cost_action(&e.value, who),
        // A cost-side `With` ([CR#601.2b]) is a choose-then-pay step with no
        // single-`Action` rendering — it must surface a payment-time choice and
        // bind `That`/`Those`. Every caller that can see a `With` routes through
        // `unless_cost_effect` (which renders it as an `OneShotEffect::With`), so this
        // Action-only path is never reached for one.
        CostComponent::With { .. } => unreachable!(
            "a cost-side With ([CR#601.2b]) is rendered by unless_cost_effect as an \
             OneShotEffect::With, never as a single Action"
        ),
        // The `unless` cost list is `Cost::normalize`d at the `OneShotEffect::Unless`
        // boundary (see `resolve.rs`), which splices every nested `Cost` flat,
        // so a `Cost` component never survives to here.
        CostComponent::Cost(_) => {
            unreachable!("nested Cost is spliced away by normalize at the Unless boundary")
        }
        // An aggregate-stat (tap-total) cost ([CR#702.122a] Crew) as an
        // 'unless' cost needs the payer's subset choice, which has no single
        // `Action` rendering. Crew-family costs are activation costs, never
        // 'unless' costs, so this seam is unreached today.
        CostComponent::TapTotal { .. } => todo!(
            "engine-resolve-effects seam: an aggregate-stat (tap-total) 'unless' cost \
             ([CR#118.12a,702.122a]) needs a payment-time subset choice"
        ),
        // The `Unless`/`MayPay` continuations route `Mana` components to
        // `WorkItem::TollMana` (see `toll_item`) before this walk; a caller
        // that reaches here with one (the `AdditionalCost` arm) is still the
        // announce-slot-bound seam, like "equal to its mana cost"
        // ([CR#202.1]).
        CostComponent::Mana(_) | CostComponent::ManaCostOf(_) => todo!(
            "engine-resolve-effects seam: a mid-resolution mana cost outside the \
             Unless/MayPay toll path ([CR#118.12a]) — announce-slot-bound"
        ),
    }
}

/// One toll cost component as an agenda item ([CR#118.12a]): a `Mana`
/// component surfaces a mid-resolution `PayMana` (`WorkItem::TollMana`,
/// paid by the resolved `payer` from their pool); every other component is
/// the payer\'s action/effect via [`unless_cost_effect`].
pub(crate) fn toll_item(
    component: &deckmaste_core::CostComponent,
    who: &deckmaste_core::Reference,
    payer: crate::player::PlayerId,
    frame: &crate::stack::Frame,
) -> WorkItem {
    match component {
        deckmaste_core::CostComponent::Mana(mc) => WorkItem::TollMana {
            player: payer,
            cost: mc.clone(),
            subject: frame.source,
        },
        deckmaste_core::CostComponent::Expanded(e) => toll_item(&e.value, who, payer, frame),
        other => WorkItem::RunEffect {
            effect: Arc::new(unless_cost_effect(other, who)),
            frame: frame.clone(),
        },
    }
}

/// One cost component rendered as the effect `who` runs to pay it
/// ([CR#118.12a,601.2b]) — the entry point every cost-to-effect payment walk
/// (the `Unless`/`MayPay` continuations, the `AdditionalCost` arm, and the
/// activation cost-`With` step) uses. Most components are a single payer
/// `Action`, so they wrap [`unless_cost_action`] in [`OneShotEffect::Act`]; a
/// cost-side [`With`](deckmaste_core::CostComponent::With) is a choose-then-pay
/// step with no single-`Action` rendering, so it becomes an
/// [`OneShotEffect::With`]: the binder surfaces the controller's
/// `ChooseObjects` choice (bound as `That`/`Those`), then the body pays against
/// it — exactly the effect-side `With` machinery, reused here so cost choosing
/// stays OUT of the verb.
pub(crate) fn unless_cost_effect(
    component: &deckmaste_core::CostComponent,
    who: &deckmaste_core::Reference,
) -> deckmaste_core::OneShotEffect {
    use deckmaste_core::CostComponent;
    use deckmaste_core::OneShotEffect;
    match component {
        // [CR#601.2b]: the binder's choice binds `That`/`Those`; the body pays
        // against that binding. Recurse on the body (a nested `With` still
        // surfaces its own choice) and reuse the `OneShotEffect::With` interpreter.
        CostComponent::With { binder, body } => OneShotEffect::With(deckmaste_core::With {
            binder: (**binder).clone(),
            body: Arc::new(cost_body_effect(body, who)),
        }),
        // Look through a remembered macro invocation so a wrapped `With` is
        // still intercepted here (not delegated to the Action-only path).
        CostComponent::Expanded(e) => unless_cost_effect(&e.value, who),
        other => OneShotEffect::Act(unless_cost_action(other, who)),
    }
}

/// A cost-`With`'s body (a [`Cost`](deckmaste_core::Cost)) as the single effect
/// `who` runs to pay it ([CR#601.2b]): each component rendered by
/// [`unless_cost_effect`], sequenced in order (a lone component stays bare).
fn cost_body_effect(
    body: &deckmaste_core::Cost,
    who: &deckmaste_core::Reference,
) -> deckmaste_core::OneShotEffect {
    let mut effects: Vec<deckmaste_core::OneShotEffect> =
        body.iter().map(|c| unless_cost_effect(c, who)).collect();
    match effects.len() {
        1 => effects.pop().expect("len 1"),
        _ => deckmaste_core::OneShotEffect::Sequentially(effects.into()),
    }
}

use deckmaste_core::Agency;
use deckmaste_core::Zone;

use crate::agenda::FinalizeWatch;
use crate::agenda::WorkItem;
use crate::derive;
use crate::event::AbilityActivated;
use crate::event::Act;
use crate::event::Cause;
use crate::event::DamageDealt;
use crate::event::GameEvent;
use crate::event::ManaAdded;
use crate::event::Occurrence;
use crate::event::PlayerLost;
use crate::event::Tapped;
use crate::event::ZoneChange;
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
        // The whole dispatch: one arm per `PendingDecision` variant, each
        // routing to that kind's `DecisionHandler::resolve`. `.clone()` is
        // needed because `self.pending` can't be moved out of while `self`
        // is passed to `h.resolve` mutably.
        match self.pending.clone().expect("checked Some") {
            PendingDecision::Priority(h) => h.resolve(self, decision),
            PendingDecision::DiscardToHandSize(h) => h.resolve(self, decision),
            PendingDecision::DiscardCards(h) => h.resolve(self, decision),
            PendingDecision::CallFlip(h) => h.resolve(self, decision),
            PendingDecision::ChooseManaColor(h) => h.resolve(self, decision),
            PendingDecision::ChooseManaMode(h) => h.resolve(self, decision),
            PendingDecision::ChooseTargets(h) => h.resolve(self, decision),
            PendingDecision::ChooseNewTargets(h) => h.resolve(self, decision),
            PendingDecision::PayMana(h) => h.resolve(self, decision),
            PendingDecision::OrderTriggers(h) => h.resolve(self, decision),
            PendingDecision::DeclareAttackers(h) => h.resolve(self, decision),
            PendingDecision::DeclareBlockers(h) => h.resolve(self, decision),
            PendingDecision::AssignCombatDamage(h) => h.resolve(self, decision),
            PendingDecision::ChooseModes(h) => h.resolve(self, decision),
            PendingDecision::Division(h) => h.resolve(self, decision),
            PendingDecision::Vote(h) => h.resolve(self, decision),
            PendingDecision::YesNo(h) => h.resolve(self, decision),
            PendingDecision::ChooseCostOptions(h) => h.resolve(self, decision),
            PendingDecision::ChooseXValue(h) => h.resolve(self, decision),
            PendingDecision::ChooseNoteNumber(h) => h.resolve(self, decision),
            PendingDecision::ChooseNoteCardName(h) => h.resolve(self, decision),
            PendingDecision::OrderReplacements(h) => h.resolve(self, decision),
            PendingDecision::ChooseReplacement(h) => h.resolve(self, decision),
            PendingDecision::PreGame(h) => h.resolve(self, decision),
            PendingDecision::ChooseObjects(h) => h.resolve(self, decision),
            PendingDecision::LegendRule(h) => h.resolve(self, decision),
            PendingDecision::ArrangePile(h) => h.resolve(self, decision),
        }
    }

    /// [CR#514.1]: cleanup's hand-size discard — validate that exactly
    /// `count` distinct in-hand cards were chosen, then emit one `Discarded`
    /// per card. (The keyword-action discard's own chosen-form choice rides
    /// the general `With`+`Choose` binder machinery instead, Task 8 —
    /// `DiscardCards` is this decision's now-unreachable sibling.)
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
        // ONE simultaneous batch: a multi-discard's moves commit together
        // ([CR#603.3b]), and the clause's amount — its card count, the
        // entailment row's `amount` — fixes "that many" for a following
        // draw ([CR#107.3]; the `apply_occurrence` funnel counts the batch).
        self.schedule_discard_acts(player, objects);
        Ok(())
    }

    /// [CR#701.9a]: schedule cleanup's hand-size discard as PER-CARD
    /// `Act(Discard)` windows — ONE simultaneous batch (the choice was
    /// batched, [CR#603.3b]) but minted per-card, because each card's
    /// discard is its own replaceable/cantable moment ([CR#616.1]): madness
    /// reroutes ITS card's Hand→Graveyard to exile and no other's
    /// ([CR#702.35a]), and "whenever a player discards a card" fires once
    /// per card. Each surviving `Act` is dual-facet (performer + patient +
    /// the Hand→Graveyard body facet) and COMMITS its move atomically at
    /// apply — the destroy shape, no second replaceable future-form
    /// `ZoneChange` below it — plus its own `FinalizeAct` watcher, so a
    /// redirected discard still records its name-fact and a suppressed one
    /// records none. (The keyword-action discard's per-card events now come
    /// from the SAME bound single-move construction the macro-driven
    /// `With`+`Choose`/`Random` lane recurses into — this helper is
    /// cleanup's own, [CR#514.1], not authored card data.)
    pub(crate) fn schedule_discard_acts(&mut self, player: PlayerId, objects: Vec<ObjectId>) {
        if objects.is_empty() {
            return;
        }
        let acts: Vec<GameEvent> = objects
            .into_iter()
            .map(|object| {
                GameEvent::Act(Act {
                    verb: deckmaste_core::VerbName::from("Discard"),
                    who: Some(player),
                    on: vec![object],
                    from: Some(Zone::Hand),
                    to: Some(Zone::Graveyard),
                    cause: Some(Cause::discard(Agency::EffectInstruction, None)),
                    // Each per-card discard is a FUTURE window ([CR#616.1]) whose
                    // apply commits the single Hand → Graveyard move; a bound
                    // single move needs no `contents`.
                    committed: false,
                    contents: None,
                    batch: None,
                    inherited: std::collections::HashSet::new(),
                    contained: false,
                })
            })
            .collect();
        let mark = self.finalize_mark();
        let mut items = Vec::with_capacity(acts.len() + 1);
        items.push(WorkItem::Emit(Occurrence::Batch(acts.clone())));
        for act in acts {
            if let GameEvent::Act(Act { on, .. }) = &act
                && let Some(&on) = on.first()
            {
                items.push(WorkItem::FinalizeAct {
                    act,
                    watch: FinalizeWatch::Patients(vec![on]),
                    mark,
                });
            }
        }
        self.schedule_front(items);
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
    /// source, [CR#702.19b] additionally requires that the player or
    /// planeswalker it's attacking be assigned damage only once every blocker
    /// recipient has at least lethal —
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
    /// or (trample) assign the player/planeswalker it's attacking while a
    /// blocker recipient is below lethal.
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
        // [CR#702.19b]: a trample source may assign damage to what it's
        // attacking (a player OR a planeswalker) only after every blocker
        // recipient has lethal. The spill target is whatever recipient is NOT a
        // blocker of this source — so this holds uniformly whether the source is
        // trampling over a player proxy or a planeswalker ([CR#702.19f]).
        let view = self.layers();
        if crate::combat::has_keyword(&view, source, &KeywordAbility::Trample) {
            let blockers = self.combat.blockers_of(source);
            let assigned = |id: ObjectId| {
                amounts
                    .iter()
                    .find(|&&(t, _)| t == id)
                    .map_or(0, |&(_, n)| n)
            };
            let to_spill: Uint = recipients
                .iter()
                .filter(|&r| !blockers.contains(r))
                .map(|&r| assigned(r))
                .sum();
            if to_spill > 0 {
                for &b in blockers {
                    if assigned(b) < self.lethal_for(&view, source, b) {
                        return Err(DecisionError::Illegal {
                            reason: "trample: each blocker must be assigned lethal before the \
                                     player or planeswalker it's attacking ([CR#702.19b])"
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
                cd.buffer.push(GameEvent::DamageDealt(DamageDealt {
                    source,
                    target,
                    amount,
                    // The combat-damage step's assignment ([CR#510.1]).
                    combat: true,
                }));
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
                #[expect(clippy::cast_sign_loss, reason = "t > 0 in this arm")]
                let toughness = t as Uint;
                toughness.saturating_sub(self.objects.obj(blocker).total_damage())
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
            GameEvent::PlayerLost(PlayerLost {
                player,
                reason: crate::event::LossReason::Conceded,
            }),
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
                let mut items = vec![WorkItem::Emit(Occurrence::single(GameEvent::ZoneChange(
                    ZoneChange {
                        snapshot: None,
                        object: *object,
                        from: Some(Zone::Hand),
                        to: Zone::Battlefield,
                        enters: None,
                        position: None,
                        face: None,
                        cause: Some(Cause::play(Agency::SpecialAction, None)),
                    },
                )))];
                items.extend(Self::priority_tail());
                self.schedule_front(items);
            }
            Action::ActivateAbility { object, ability } => {
                let abilities = derive::usable_abilities(self, *object);
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
                        WorkItem::Emit(Occurrence::single(GameEvent::Tapped(Tapped {
                            object: *object,
                            cause: Some(Cause::tap(Agency::CostPayment, None)),
                        }))),
                        WorkItem::Emit(Occurrence::single(GameEvent::ManaAdded(ManaAdded {
                            player,
                            mana,
                            amount,
                            riders,
                        }))),
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
                        GameEvent::AbilityActivated(AbilityActivated {
                            source: *object,
                            ability: *ability,
                        }),
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
        let mut items = Self::announce_schedule_no_priority(begin, cast_event);
        items.extend(Self::priority_tail());
        items
    }

    /// [CR#608.2g]: the [CR#601.2a..601.2i] announce procedure WITHOUT the
    /// priority tail — a spell cast during resolution follows the same
    /// announce steps, "except no player receives priority after it's cast."
    /// The currently-resolving spell or ability continues once the cast spell
    /// is the topmost stack object, so this omits the `CheckSbas`/
    /// `PlaceTriggers`/`OpenPriority` [`priority_tail`](Self::priority_tail)
    /// (the resolution's own trailing sweep runs at its end). The shared body
    /// of [`announce_schedule`](Self::announce_schedule), which appends the
    /// tail for the priority-restarting hand-cast/activation path.
    #[must_use]
    pub(crate) fn announce_schedule_no_priority(
        begin: WorkItem,
        cast_event: GameEvent,
    ) -> Vec<WorkItem> {
        vec![
            begin,
            WorkItem::AnnounceOptionalCosts { index: 0 },
            WorkItem::AnnounceX,
            WorkItem::AnnounceTargets,
            WorkItem::ChooseCostOptions,
            WorkItem::PayCost,
            WorkItem::Emit(Occurrence::single(cast_event)),
        ]
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
        // Never-crash: the source may have ceased to exist by the time this
        // runs (a token that left the game, or an LKI-only snapshot id). Both
        // `objects.obj` and `layers().get` panic on a stale id, so guard the
        // lookup with the non-panicking `objects.get` accessor and fizzle to
        // no provenance — an object that is gone is not a snow permanent. A
        // non-battlefield object short-circuits here too, keeping `layers()`
        // (which also panics on a non-live id) off the path unless the source
        // is a live permanent.
        let Some(obj) = self.objects.get(source) else {
            return vec![];
        };
        if obj.zone != Some(Zone::Battlefield) {
            return vec![];
        }
        let is_snow = self
            .layers()
            .get(source)
            .supertypes
            .contains(&deckmaste_core::Supertype::Snow);
        if is_snow { vec![deckmaste_core::ManaRider::Snow] } else { vec![] }
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

#[cfg(test)]
mod snow_provenance_tests {
    use crate::object::ObjectId;
    use crate::player::PlayerId;
    use crate::state::GameConfig;
    use crate::state::GameState;
    use crate::state::PlayerConfig;
    use crate::state::StartingPlayer;

    fn game() -> GameState {
        GameState::new(GameConfig {
            players: vec![PlayerConfig { deck: vec![] }, PlayerConfig { deck: vec![] }],
            seed: 1,
            starting_life: 20,
            starting_player: StartingPlayer::Fixed(PlayerId(0)),
            sba_rules: vec![],
            conferral_rules: vec![],
            damage_result_rules: vec![],
            counter_decls: std::collections::HashMap::new(),
            subtypes: std::collections::HashMap::new(),
            types: std::collections::HashMap::new(),
        })
    }

    /// Never-crash ([CR#107.4h]): a mana ability can resolve after its source
    /// has ceased to exist (a token that left the game, or an LKI-only
    /// snapshot id) — `frame.source` then names an object absent from the
    /// store. `snow_provenance` must fizzle to no provenance rather than
    /// panic in the `objects.obj` / `layers().get` lookups. Regression for a
    /// latent panic surfaced while writing an `AmongColorsOf` test.
    #[test]
    fn snow_provenance_fizzles_on_ceased_source() {
        let state = game();
        // A fabricated id that was never in the store stands in for a source
        // that has ceased to exist by resolution time.
        let ceased = ObjectId::from_raw(9999);
        let riders = state.snow_provenance(ceased);
        assert!(
            riders.is_empty(),
            "a ceased/absent source contributes no snow provenance (must not \
             panic): {riders:?}"
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::decide::pending::Priority;
    use crate::state::GameConfig;
    use crate::state::PlayerConfig;
    use crate::state::StartingPlayer;

    fn game() -> GameState {
        GameState::new(GameConfig {
            players: vec![PlayerConfig { deck: vec![] }, PlayerConfig { deck: vec![] }],
            seed: 1,
            starting_life: 20,
            starting_player: StartingPlayer::Fixed(PlayerId(0)),
            sba_rules: vec![],
            conferral_rules: vec![],
            damage_result_rules: vec![],
            counter_decls: std::collections::HashMap::new(),
            subtypes: std::collections::HashMap::new(),
            types: std::collections::HashMap::new(),
        })
    }

    /// `submit_decision`'s contract: a REJECTED submission leaves the
    /// decision OPEN (`self.pending` unchanged), so the runner can re-ask.
    /// Pins the clone-then-clear-on-success design.
    #[test]
    fn rejected_submission_leaves_pending_open() {
        // Open a Priority pending with a known legal list, then submit an
        // action that is NOT in it.
        let mut state = game();
        let player = PlayerId(0);
        state.pending = Some(PendingDecision::Priority(Priority {
            player,
            legal: vec![Action::Pass],
        }));
        let before = state.pending.clone();
        let result = state.submit_decision(Decision::Act(Action::PlayLand {
            object: ObjectId::from_raw(9999),
        }));
        let DecisionError::Illegal { reason } = result.as_ref().unwrap_err() else {
            panic!("expected illegal action rejection, got {result:?}");
        };
        assert!(!reason.is_empty());
        assert_eq!(
            state.pending, before,
            "the decision stays pending after rejection"
        );
    }
}
