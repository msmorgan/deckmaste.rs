mod coverage;
mod iou;

use std::sync::Arc;

pub use coverage::ManaCoverage;
pub use coverage::ManaPayment;
use deckmaste_core::Action;
use deckmaste_core::Cost;
use deckmaste_core::CostComponent;
use deckmaste_core::LifeOp;
use deckmaste_core::ManaCost;
use deckmaste_core::ManaSymbol;
use deckmaste_core::PayAct;
use deckmaste_core::PipClass;
use deckmaste_core::Reference;
use deckmaste_core::SimpleManaSymbol;
pub use iou::FulfillmentWitness;
pub use iou::IouId;
pub use iou::IouKind;
pub use iou::ManaPip;
pub use iou::PaymentIou;
pub use iou::PaymentRecordId;

use crate::object::ObjectId;
use crate::player::ManaUnit;
use crate::player::PlayerId;
use crate::stack::Frame;
use crate::state::GameImage;
use crate::state::GameState;

/// The externally visible phase of one locked payment transaction.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PaymentStage {
    /// The payer may activate mana abilities, then submits complete mana-pip
    /// coverage. No cost has been paid yet ([CR#601.2g]).
    PrePayment,
    /// Coverage is locked (when applicable) and IOUs may be fulfilled in a
    /// payer-chosen legal order ([CR#601.2h]).
    Paying,
    /// Every IOU is fulfilled; only submission, editing, decline, or
    /// concession remains.
    Ready,
}

/// Why a payment frame exists. Decline and successful submission promote or
/// discard the working image differently for announcements and optional
/// payments.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PaymentPurpose {
    Announcement,
    Optional {
        if_did: Option<Arc<deckmaste_core::OneShotEffect>>,
        if_not: Option<Arc<deckmaste_core::OneShotEffect>>,
        frame: Box<Frame>,
    },
}

/// The spell, ability, or resolving effect whose cost is being paid. The
/// object returned by [`Self::object`] is the object mana spend restrictions
/// judge.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PaymentSubject {
    Spell(ObjectId),
    Activated { ability: ObjectId, source: ObjectId },
    Effect { source: ObjectId },
}

impl PaymentSubject {
    #[must_use]
    pub fn object(self) -> ObjectId {
        match self {
            Self::Spell(object) => object,
            Self::Activated { source, .. } | Self::Effect { source } => source,
        }
    }
}

/// A client command at the payment decision boundary. Concession remains the
/// universal ordinary decision and is intentionally not duplicated here.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PaymentCommand {
    BeginPayment(ManaCoverage),
    ActivateManaAbility {
        source: ObjectId,
        ability: usize,
    },
    Fulfill {
        iou: IouId,
        witness: FulfillmentWitness,
    },
    RescindFulfillment(IouId),
    SubmitPayment,
    DeclinePayment,
}

/// The facts a client needs to render a payment decision. It deliberately
/// contains no recommended source, order, or repair.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PaymentPrompt {
    pub payer: PlayerId,
    pub subject: PaymentSubject,
    pub stage: PaymentStage,
    pub outstanding: Vec<PaymentIou>,
    pub fulfilled: Vec<IouId>,
    pub floating_mana: Vec<ManaUnit>,
    pub coverage: Option<ManaCoverage>,
    pub mana_abilities: Vec<(ObjectId, usize)>,
    pub rescindable: Vec<IouId>,
}

/// The immutable IOU graph created after announce-time choices and total-cost
/// locking. `has_mana_payment` distinguishes an explicit `{0}` from an empty
/// cost, even though neither creates a payable pip IOU.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LockedPayment {
    pub payer: PlayerId,
    pub subject: PaymentSubject,
    pub frame: Frame,
    pub ious: Vec<PaymentIou>,
    pub stage: PaymentStage,
    pub has_mana_payment: bool,
}

impl LockedPayment {
    #[must_use]
    pub fn mana_pips(&self) -> Vec<&PaymentIou> {
        self.ious
            .iter()
            .filter(|iou| matches!(iou.kind, IouKind::ManaPip(_)))
            .collect()
    }

    fn proposal_placeholder(payer: PlayerId, subject: PaymentSubject, frame: Frame) -> Self {
        Self {
            payer,
            subject,
            frame,
            ious: Vec::new(),
            stage: PaymentStage::Ready,
            has_mana_payment: false,
        }
    }
}

/// One full speculative game image and the metadata needed to edit or commit
/// its locked payment. `payment_base` is established when a mana-paying frame
/// crosses `BeginPayment`; pure nonmana frames establish it on entry.
#[derive(Debug, Clone)]
pub struct PaymentFrame {
    pub(crate) working: GameImage,
    pub(crate) payment_base: Option<GameImage>,
    pub payer: PlayerId,
    pub subject: PaymentSubject,
    pub purpose: PaymentPurpose,
    pub locked: LockedPayment,
    pub stage: PaymentStage,
    pub coverage: Option<ManaCoverage>,
    pub fulfilled: Vec<(IouId, FulfillmentWitness)>,
    #[expect(
        dead_code,
        reason = "record ids are minted when the frame-local replay ledger lands"
    )]
    pub(crate) next_record: u64,
}

impl PaymentFrame {
    #[must_use]
    pub fn new(working: GameImage, purpose: PaymentPurpose, locked: LockedPayment) -> Self {
        let payment_base = (locked.stage != PaymentStage::PrePayment).then(|| working.clone());
        Self {
            working,
            payment_base,
            payer: locked.payer,
            subject: locked.subject,
            purpose,
            stage: locked.stage,
            locked,
            coverage: None,
            fulfilled: Vec::new(),
            next_record: 0,
        }
    }

    /// Start an isolated announcement before its total cost has locked. The
    /// placeholder is never exposed as a payment prompt; [`Self::initialize`]
    /// replaces it when `OpenPayment` runs.
    pub(crate) fn proposal(working: GameImage, payer: PlayerId) -> Self {
        let source = working.players[payer.index()].object;
        let subject = PaymentSubject::Effect { source };
        let frame = Frame::bare(source, payer);
        Self::new(
            working,
            PaymentPurpose::Announcement,
            LockedPayment::proposal_placeholder(payer, subject, frame),
        )
    }

    pub(crate) fn initialize(&mut self, locked: LockedPayment) {
        self.payment_base =
            (locked.stage != PaymentStage::PrePayment).then(|| self.working.clone());
        self.payer = locked.payer;
        self.subject = locked.subject;
        self.stage = locked.stage;
        self.locked = locked;
        self.coverage = None;
        self.fulfilled.clear();
    }

    #[must_use]
    pub(crate) fn prompt(&self) -> PaymentPrompt {
        let fulfilled: Vec<IouId> = self.fulfilled.iter().map(|(iou, _)| *iou).collect();
        let fulfilled_set: std::collections::HashSet<IouId> = fulfilled.iter().copied().collect();
        PaymentPrompt {
            payer: self.payer,
            subject: self.subject,
            stage: self.stage,
            outstanding: self
                .locked
                .ious
                .iter()
                .filter(|iou| !fulfilled_set.contains(&iou.id))
                .cloned()
                .collect(),
            fulfilled,
            floating_mana: self.working.players[self.payer.index()]
                .mana_pool
                .units()
                .to_vec(),
            coverage: self.coverage.clone(),
            mana_abilities: Vec::new(),
            rescindable: Vec::new(),
        }
    }
}

/// Transaction metadata lives outside every [`GameImage`], preventing image
/// clones from recursively cloning their owner.
#[derive(Debug, Clone, Default)]
pub struct PaymentController {
    pub(crate) frames: Vec<PaymentFrame>,
}

impl GameState {
    /// Clone the active image before any proposal mutation and route ordinary
    /// engine field access into that isolated working image.
    pub(crate) fn begin_payment_proposal(&mut self, payer: PlayerId) {
        let working = self.active().clone();
        let controller = self.payment.get_or_insert_with(PaymentController::default);
        controller
            .frames
            .push(PaymentFrame::proposal(working, payer));
    }

    /// Install a just-locked IOU graph on the top proposal frame and surface
    /// its first payment prompt.
    pub(crate) fn open_locked_payment(&mut self, locked: LockedPayment) {
        let controller = self
            .payment
            .as_mut()
            .expect("OpenPayment runs inside an isolated proposal");
        let frame = controller
            .frames
            .last_mut()
            .expect("OpenPayment has a proposal frame");
        frame.initialize(locked);
        let prompt = frame.prompt();
        frame.working.pending = Some(crate::decide::PendingDecision::Payment(prompt));
    }

    /// Apply one payment-protocol command. Every rejection occurs before any
    /// frame or image mutation, preserving the pending prompt verbatim.
    pub(crate) fn submit_payment_command(
        &mut self,
        command: PaymentCommand,
    ) -> Result<(), crate::decide::DecisionError> {
        match command {
            PaymentCommand::BeginPayment(coverage) => self.begin_payment(coverage),
            PaymentCommand::SubmitPayment => self.commit_payment(),
            PaymentCommand::DeclinePayment => {
                // Root announcement decline: the outer committed image still
                // contains the untouched priority prompt. Purpose-sensitive
                // optional/nested reconstruction is added in later tasks.
                self.payment = None;
                Ok(())
            }
            PaymentCommand::ActivateManaAbility { .. }
            | PaymentCommand::Fulfill { .. }
            | PaymentCommand::RescindFulfillment(_) => Err(crate::decide::DecisionError::Illegal {
                reason: "that payment operation is not available in this implementation slice"
                    .into(),
            }),
        }
    }

    fn begin_payment(
        &mut self,
        coverage: ManaCoverage,
    ) -> Result<(), crate::decide::DecisionError> {
        let controller = self
            .payment
            .as_ref()
            .expect("a Payment decision has a controller");
        let frame = controller
            .frames
            .last()
            .expect("a Payment decision has a frame");
        if frame.stage != PaymentStage::PrePayment {
            return Err(crate::decide::DecisionError::Illegal {
                reason: "BeginPayment is legal only during PrePayment".into(),
            });
        }
        frame.locked.validate_coverage(self, &coverage)?;

        // Validation above is read-only. From here on the command cannot fail.
        self.pending = None;
        let payment_base = self.active().clone();
        let controller = self.payment.as_mut().expect("controller remains live");
        let frame = controller.frames.last_mut().expect("frame remains live");
        frame.payment_base = Some(payment_base);
        frame.coverage = Some(coverage);
        frame.stage = if frame.locked.ious.is_empty() {
            PaymentStage::Ready
        } else {
            PaymentStage::Paying
        };
        let prompt = frame.prompt();
        frame.working.pending = Some(crate::decide::PendingDecision::Payment(prompt));
        Ok(())
    }

    fn commit_payment(&mut self) -> Result<(), crate::decide::DecisionError> {
        let controller = self
            .payment
            .as_ref()
            .expect("a Payment decision has a controller");
        let frame = controller
            .frames
            .last()
            .expect("a Payment decision has a frame");
        if frame.stage != PaymentStage::Ready {
            return Err(crate::decide::DecisionError::Illegal {
                reason: "SubmitPayment requires every IOU to be fulfilled".into(),
            });
        }
        if controller.frames.len() != 1 {
            return Err(crate::decide::DecisionError::Illegal {
                reason: "nested payment submission is not available yet".into(),
            });
        }

        self.pending = None;
        let mut controller = self.payment.take().expect("controller remains live");
        let frame = controller.frames.pop().expect("root frame remains live");
        self.committed = frame.working;
        Ok(())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, thiserror::Error)]
pub enum PaymentLockError {
    #[error("a multi-way or variable mana symbol was not concretized before payment locking")]
    UnconcretizedManaSymbol,
    #[error("ManaCostOf resolved to an object without a mana cost")]
    MissingManaCost,
    #[error("an expanded cost component survived lowering")]
    ExpandedCost,
}

/// Materialize the locked cost as stable runtime IOUs. The caller supplies the
/// announce/resolution frame so X and reference-bearing counts freeze against
/// the same bindings as the cost itself.
///
/// # Errors
///
/// Returns [`PaymentLockError`] if a multi-way or variable mana symbol has not
/// been concretized, `ManaCostOf` names a non-card object, or lowering left an
/// expanded cost wrapper at the runnable boundary.
pub fn lock_cost(
    state: &GameState,
    payer: PlayerId,
    subject: PaymentSubject,
    frame: &Frame,
    cost: &Cost,
    pay_pips: &[(PipClass, PayAct)],
) -> Result<LockedPayment, PaymentLockError> {
    let mut builder = LockBuilder {
        state,
        frame,
        pay_pips,
        ious: Vec::new(),
        next_iou: 0,
        has_mana_payment: false,
    };
    builder.lock_components(cost)?;
    let entry_stage = if builder.has_mana_payment {
        PaymentStage::PrePayment
    } else if builder.ious.is_empty() {
        PaymentStage::Ready
    } else {
        PaymentStage::Paying
    };
    Ok(LockedPayment {
        payer,
        subject,
        frame: frame.clone(),
        ious: builder.ious,
        stage: entry_stage,
        has_mana_payment: builder.has_mana_payment,
    })
}

struct LockBuilder<'a> {
    state: &'a GameState,
    frame: &'a Frame,
    pay_pips: &'a [(PipClass, PayAct)],
    ious: Vec<PaymentIou>,
    next_iou: u64,
    has_mana_payment: bool,
}

impl LockBuilder<'_> {
    fn lock_components(&mut self, cost: &Cost) -> Result<(), PaymentLockError> {
        for component in cost {
            match component {
                CostComponent::Mana(mana) => self.lock_mana(mana)?,
                CostComponent::ManaCostOf(reference) => {
                    let object = self.state.eval_reference(reference, self.frame);
                    if self
                        .state
                        .objects
                        .get(object)
                        .and_then(crate::object::GameObject::card_id)
                        .is_none()
                    {
                        return Err(PaymentLockError::MissingManaCost);
                    }
                    let mana = self
                        .state
                        .mana_cost(object)
                        .ok_or(PaymentLockError::MissingManaCost)?;
                    self.lock_mana(&mana)?;
                }
                CostComponent::Tap => self.push(IouKind::Tap, Vec::new()),
                CostComponent::Untap => self.push(IouKind::Untap, Vec::new()),
                CostComponent::Act(action) => self.lock_action(action),
                CostComponent::Cost(inner) => self.lock_components(inner)?,
                CostComponent::TapTotal {
                    stat,
                    cmp,
                    count,
                    filter,
                } => {
                    let count = self.state.eval_count(count, self.frame);
                    self.push(
                        IouKind::TapTotal {
                            stat: *stat,
                            cmp: *cmp,
                            count,
                            filter: Arc::clone(filter),
                        },
                        Vec::new(),
                    );
                }
                CostComponent::ChooseAndPay { binder, body } => self.push(
                    IouKind::ChooseAndPay {
                        binder: Arc::clone(binder),
                        body: body.clone(),
                    },
                    Vec::new(),
                ),
                CostComponent::Expanded(_) => return Err(PaymentLockError::ExpandedCost),
            }
        }
        Ok(())
    }

    fn lock_action(&mut self, action: &Arc<Action>) {
        let kind = match &**action {
            Action::ChangeLife(Reference::You, LifeOp::Down(count)) => {
                IouKind::PayLife(self.state.eval_count(count, self.frame))
            }
            _ => IouKind::Act(Arc::clone(action)),
        };
        self.push(kind, Vec::new());
    }

    fn lock_mana(&mut self, mana: &ManaCost) -> Result<(), PaymentLockError> {
        // An empty ManaCost is what remains when every Phyrexian symbol was
        // announced as life. `{0}` is nonempty (`Generic(0)`) and therefore
        // still grants the project's explicit prepayment window.
        self.has_mana_payment |= !mana.is_empty();
        for symbol in mana.iter() {
            match symbol {
                ManaSymbol::Simple(SimpleManaSymbol::Generic(amount)) => {
                    for _ in 0..*amount {
                        self.push_mana_pip(ManaPip::Generic);
                    }
                }
                ManaSymbol::Simple(SimpleManaSymbol::Specific(kind)) => match kind.color() {
                    Some(color) => self.push_mana_pip(ManaPip::Colored(color)),
                    None => self.push_mana_pip(ManaPip::Colorless),
                },
                ManaSymbol::Snow => self.push_mana_pip(ManaPip::Snow),
                ManaSymbol::Variable | ManaSymbol::Hybrid(..) | ManaSymbol::Phyrexian(..) => {
                    return Err(PaymentLockError::UnconcretizedManaSymbol);
                }
            }
        }
        Ok(())
    }

    fn push_mana_pip(&mut self, pip: ManaPip) {
        let alternatives = self
            .pay_pips
            .iter()
            .filter(|(class, _)| pip.accepts_alternative(*class))
            .map(|(_, act)| act.clone())
            .collect();
        self.push(IouKind::ManaPip(pip), alternatives);
    }

    fn push(&mut self, kind: IouKind, alternatives: Vec<PayAct>) {
        let id = IouId(self.next_iou);
        self.next_iou = self
            .next_iou
            .checked_add(1)
            .expect("payment IOU id overflow");
        self.ious.push(PaymentIou {
            id,
            kind,
            alternatives,
        });
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;
    use std::sync::Arc;

    use deckmaste_card::Card;
    use deckmaste_card::CardFace;
    use deckmaste_core::Color;
    use deckmaste_core::Cost;
    use deckmaste_core::CostComponent;
    use deckmaste_core::ManaCost;
    use deckmaste_core::ManaRider;
    use deckmaste_core::PayAct;
    use deckmaste_core::PipClass;
    use deckmaste_core::Predicate;
    use deckmaste_core::Reference;

    use super::*;
    use crate::CostOptionChoices;
    use crate::GameConfig;
    use crate::GameState;
    use crate::ManaProvenance;
    use crate::ObjectSource;
    use crate::PlayerConfig;
    use crate::PlayerId;
    use crate::StartingPlayer;
    use crate::SymbolChoice;
    use crate::concretize;
    use crate::stack::Frame;

    fn fixture(printed_mana: &str) -> (GameState, PlayerId, crate::ObjectId) {
        let payer = PlayerId(0);
        let card = Arc::new(Card::Normal(CardFace {
            name: "Payment subject".into(),
            mana_cost: printed_mana.parse().expect("test mana cost parses"),
            ..CardFace::default()
        }));
        let state = GameState::new(GameConfig {
            players: vec![
                PlayerConfig { deck: vec![card] },
                PlayerConfig { deck: vec![] },
            ],
            seed: 1,
            starting_life: 20,
            starting_player: StartingPlayer::Fixed(payer),
            sba_rules: vec![],
            conferral_rules: vec![],
            damage_result_rules: vec![],
            counter_decls: std::collections::HashMap::new(),
            subtypes: std::collections::HashMap::new(),
            types: std::collections::HashMap::new(),
        });
        let subject = state.zones.hands[payer.index()][0];
        (state, payer, subject)
    }

    fn cost(components: Vec<CostComponent>) -> Cost {
        Cost(components.into())
    }

    fn lock_components(printed_mana: &str, components: Vec<CostComponent>) -> LockedPayment {
        let (state, payer, subject) = fixture(printed_mana);
        lock_cost(
            &state,
            payer,
            PaymentSubject::Spell(subject),
            &Frame::bare(subject, payer),
            &cost(components),
            &[],
        )
        .expect("test cost locks")
    }

    fn lock_test_cost(mana: &str) -> LockedPayment {
        lock_components(
            "",
            vec![CostComponent::Mana(
                mana.parse::<ManaCost>().expect("test mana cost parses"),
            )],
        )
    }

    #[test]
    fn generic_mana_expands_to_stable_pips() {
        let locked = lock_test_cost("{2}{B}{G}");
        let pips = locked.mana_pips();

        assert_eq!(pips.len(), 4);
        assert_eq!(
            pips.iter().map(|iou| iou.id).collect::<HashSet<_>>().len(),
            4
        );
    }

    #[test]
    fn exact_coverage_is_atomic_and_unique() {
        let (mut state, payer, subject) = fixture("");
        let green = state.player_mut(payer).mana_pool.add(
            Color::Green.into(),
            2,
            ManaProvenance::default(),
        );
        let locked = lock_cost(
            &state,
            payer,
            PaymentSubject::Spell(subject),
            &Frame::bare(subject, payer),
            &cost(vec![CostComponent::Mana("{1}{G}".parse().unwrap())]),
            &[],
        )
        .unwrap();
        let pips = locked.mana_pips();
        let mut coverage = ManaCoverage::empty();
        coverage.insert(pips[0].id, ManaPayment::Floating(green[0]));
        coverage.insert(pips[1].id, ManaPayment::Floating(green[0]));
        let before = state.player(payer).mana_pool.clone();

        assert!(locked.validate_coverage(&state, &coverage).is_err());
        assert_eq!(state.player(payer).mana_pool, before);
    }

    #[test]
    fn zero_and_nonmana_costs_choose_the_required_entry_stage() {
        assert_eq!(lock_test_cost("{0}").stage, PaymentStage::PrePayment);
        assert_eq!(
            lock_components("", vec![CostComponent::Tap]).stage,
            PaymentStage::Paying
        );
        assert_eq!(lock_components("", vec![]).stage, PaymentStage::Ready);
    }

    #[test]
    fn mana_cost_of_and_pay_pips_become_concrete_pip_options() {
        let (state, payer, subject) = fixture("{2}{G}");
        let alternative = PayAct::TapToPay(Predicate::Any);
        let locked = lock_cost(
            &state,
            payer,
            PaymentSubject::Spell(subject),
            &Frame::bare(subject, payer),
            &cost(vec![CostComponent::ManaCostOf(Reference::This)]),
            &[
                (PipClass::Generic, alternative.clone()),
                (PipClass::Colored(Color::Green), alternative.clone()),
            ],
        )
        .unwrap();

        let pips = locked.mana_pips();
        assert_eq!(pips.len(), 3);
        assert!(
            pips.iter()
                .all(|pip| pip.alternatives.contains(&alternative))
        );
    }

    #[test]
    fn announced_phyrexian_life_skips_prepayment_but_snow_requires_provenance() {
        let original: ManaCost = "{G/P}".parse().unwrap();
        let (mana, mut verbs) = concretize(
            &original,
            &CostOptionChoices {
                picks: vec![SymbolChoice::Life],
            },
        )
        .unwrap();
        let mut components = vec![CostComponent::Mana(mana)];
        components.append(&mut verbs);
        let life = lock_components("", components);
        assert_eq!(life.stage, PaymentStage::Paying);
        assert!(matches!(life.ious[0].kind, IouKind::PayLife(2)));

        let (mut state, payer, subject) = fixture("");
        let plain = state.player_mut(payer).mana_pool.add(
            Color::Green.into(),
            1,
            ManaProvenance::default(),
        )[0];
        let snow = state.player_mut(payer).mana_pool.add_riders(
            Color::Green.into(),
            1,
            &[ManaRider::Snow],
            ManaProvenance::default(),
        )[0];
        let locked = lock_cost(
            &state,
            payer,
            PaymentSubject::Spell(subject),
            &Frame::bare(subject, payer),
            &cost(vec![CostComponent::Mana("{S}".parse().unwrap())]),
            &[],
        )
        .unwrap();
        let pip = locked.mana_pips()[0].id;
        let mut plain_coverage = ManaCoverage::empty();
        plain_coverage.insert(pip, ManaPayment::Floating(plain));
        let mut snow_coverage = ManaCoverage::empty();
        snow_coverage.insert(pip, ManaPayment::Floating(snow));

        assert!(locked.validate_coverage(&state, &plain_coverage).is_err());
        assert!(locked.validate_coverage(&state, &snow_coverage).is_ok());
    }

    #[test]
    fn hybrid_reading_is_locked_before_iou_creation() {
        let original: ManaCost = "{W/U}".parse().unwrap();
        let (mana, verbs) = concretize(
            &original,
            &CostOptionChoices {
                picks: vec![SymbolChoice::Mana(Color::White.into())],
            },
        )
        .unwrap();
        assert!(verbs.is_empty());
        let locked = lock_components("", vec![CostComponent::Mana(mana)]);

        assert!(matches!(
            locked.mana_pips()[0].kind,
            IouKind::ManaPip(ManaPip::Colored(Color::White))
        ));
    }

    #[test]
    fn spend_restrictions_and_pay_pips_objects_are_validated() {
        let (mut state, payer, subject) = fixture("");
        let restricted = state.player_mut(payer).mana_pool.add_riders(
            Color::Green.into(),
            1,
            &[ManaRider::SpendOnly(Predicate::Not(Arc::new(
                Predicate::Any,
            )))],
            ManaProvenance::default(),
        )[0];
        let green = lock_cost(
            &state,
            payer,
            PaymentSubject::Spell(subject),
            &Frame::bare(subject, payer),
            &cost(vec![CostComponent::Mana("{G}".parse().unwrap())]),
            &[],
        )
        .unwrap();
        let mut restricted_coverage = ManaCoverage::empty();
        restricted_coverage.insert(green.mana_pips()[0].id, ManaPayment::Floating(restricted));
        assert!(
            green
                .validate_coverage(&state, &restricted_coverage)
                .is_err()
        );

        let resource_card = state.cards.push(
            Arc::new(Card::Normal(CardFace {
                name: "Payment resource".into(),
                ..CardFace::default()
            })),
            payer,
        );
        let resource = state.objects.mint(
            ObjectSource::Card(resource_card),
            payer,
            Some(deckmaste_core::Zone::Battlefield),
        );
        state.zones.battlefield.push(resource);
        let generic = lock_cost(
            &state,
            payer,
            PaymentSubject::Spell(subject),
            &Frame::bare(subject, payer),
            &cost(vec![CostComponent::Mana("{1}".parse().unwrap())]),
            &[(PipClass::Generic, PayAct::TapToPay(Predicate::Any))],
        )
        .unwrap();
        let mut pay_pips = ManaCoverage::empty();
        pay_pips.insert(
            generic.mana_pips()[0].id,
            ManaPayment::PayPips {
                object: resource,
                alternative: 0,
            },
        );
        assert!(generic.validate_coverage(&state, &pay_pips).is_ok());

        state.objects.obj_mut(resource).tapped = true;
        assert!(generic.validate_coverage(&state, &pay_pips).is_err());
    }
}
