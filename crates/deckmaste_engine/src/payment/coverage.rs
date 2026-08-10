use std::collections::BTreeMap;
use std::collections::HashSet;

use deckmaste_core::ColorOrColorless;
use deckmaste_core::ManaRider;
use deckmaste_core::PayAct;
use deckmaste_core::Zone;

use super::IouId;
use super::IouKind;
use super::LockedPayment;
use super::ManaPip;
use crate::decide::DecisionError;
use crate::object::ObjectId;
use crate::player::FloatingManaId;
use crate::state::GameState;

/// The exact resource selected to satisfy one locked mana-pip IOU.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ManaPayment {
    Floating(FloatingManaId),
    PayPips {
        object: ObjectId,
        alternative: usize,
    },
}

/// Whole-batch coverage submitted at the end of `PrePayment`. Every mana-pip
/// IOU must occur exactly once; the map key makes duplicate IOU entries
/// structurally impossible, while validation rejects missing/extra keys and
/// resources reused across distinct pips.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct ManaCoverage {
    entries: BTreeMap<IouId, ManaPayment>,
}

impl ManaCoverage {
    #[must_use]
    pub fn empty() -> Self {
        Self::default()
    }

    pub fn insert(&mut self, iou: IouId, payment: ManaPayment) -> Option<ManaPayment> {
        self.entries.insert(iou, payment)
    }

    #[must_use]
    pub fn get(&self, iou: IouId) -> Option<&ManaPayment> {
        self.entries.get(&iou)
    }

    #[must_use]
    pub fn len(&self) -> usize {
        self.entries.len()
    }

    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    pub fn iter(&self) -> impl Iterator<Item = (&IouId, &ManaPayment)> {
        self.entries.iter()
    }
}

impl LockedPayment {
    /// Validate the complete coverage witness without reserving, spending, or
    /// otherwise changing the active game image.
    ///
    /// # Errors
    ///
    /// Returns [`DecisionError::Illegal`] if coverage is incomplete, names an
    /// extra or reused resource, violates a mana kind or rider, or selects an
    /// ineligible `PayPips` object/alternative.
    pub fn validate_coverage(
        &self,
        state: &GameState,
        coverage: &ManaCoverage,
    ) -> Result<(), DecisionError> {
        let mana_pips = self.mana_pips();
        if coverage.len() != mana_pips.len() {
            return illegal("coverage must name every mana-pip IOU exactly once");
        }
        let expected: HashSet<IouId> = mana_pips.iter().map(|iou| iou.id).collect();
        if coverage.iter().any(|(iou, _)| !expected.contains(iou)) {
            return illegal("coverage names an IOU that is not a mana pip");
        }

        let pool = &state.player(self.payer).mana_pool;
        let mut floating = HashSet::with_capacity(coverage.len());
        let mut objects = HashSet::with_capacity(coverage.len());
        for iou in mana_pips {
            let IouKind::ManaPip(pip) = iou.kind else {
                unreachable!("mana_pips returns only mana IOUs")
            };
            let Some(payment) = coverage.get(iou.id) else {
                return illegal("coverage omits a mana-pip IOU");
            };
            match *payment {
                ManaPayment::Floating(id) => {
                    if !floating.insert(id) {
                        return illegal("one floating mana unit cannot cover two pips");
                    }
                    let Some(unit) = pool.get(id) else {
                        return illegal("coverage names mana that is not in the payer's pool");
                    };
                    if !pip_accepts_unit(pip, unit.kind, &unit.riders) {
                        return illegal("floating mana does not match the covered pip");
                    }
                    if !state.unit_spendable_on(unit, self.subject.spend_object()) {
                        return illegal("floating mana's spend restriction rejects this payment");
                    }
                }
                ManaPayment::PayPips {
                    object,
                    alternative,
                } => {
                    if !objects.insert(object) {
                        return illegal("one alternative-payment object cannot cover two pips");
                    }
                    let Some(act) = iou.alternatives.get(alternative) else {
                        return illegal("coverage names an unavailable PayPips alternative");
                    };
                    validate_pay_pips_object(state, self, act, object)?;
                }
            }
        }
        Ok(())
    }
}

pub(super) fn pip_accepts_unit(pip: ManaPip, kind: ColorOrColorless, riders: &[ManaRider]) -> bool {
    match pip {
        ManaPip::Generic => true,
        ManaPip::Colored(color) => kind == ColorOrColorless::Color(color),
        ManaPip::Colorless => kind == ColorOrColorless::Colorless,
        ManaPip::Snow => riders.contains(&ManaRider::Snow),
    }
}

pub(super) fn validate_pay_pips_object(
    state: &GameState,
    locked: &LockedPayment,
    act: &PayAct,
    object: ObjectId,
) -> Result<(), DecisionError> {
    let Some(candidate) = state.objects.get(object) else {
        return illegal("PayPips coverage names a stale object");
    };
    let spend_object = locked.subject.spend_object();
    let watcher = state.objects.obj(spend_object).source;
    match act {
        PayAct::TapToPay(filter) => {
            if candidate.zone != Some(Zone::Battlefield)
                || candidate.controller != locked.payer
                || candidate.tapped
                || !crate::target::matches_with(state, object, filter, Some(watcher))
            {
                return illegal("object cannot pay this pip by tapping");
            }
        }
        PayAct::ExileToPay(filter) => {
            if candidate.zone != Some(Zone::Graveyard)
                || candidate.card_id().is_none()
                || state.owner_of(object) != locked.payer
                || !crate::target::matches_with(state, object, filter, Some(watcher))
            {
                return illegal("object cannot pay this pip by being exiled");
            }
        }
    }
    Ok(())
}

fn illegal<T>(reason: impl Into<String>) -> Result<T, DecisionError> {
    Err(DecisionError::Illegal {
        reason: reason.into(),
    })
}
