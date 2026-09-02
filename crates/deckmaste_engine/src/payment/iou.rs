use std::sync::Arc;

use deckmaste_core::Cmp;
use deckmaste_core::Color;
use deckmaste_core::PayAct;
use deckmaste_core::PipClass;
use deckmaste_core::Predicate;
use deckmaste_core::RunnableCostAction;
use deckmaste_core::Stat;
use deckmaste_core::Uint;

use crate::object::ObjectId;

/// Stable identity of one obligation within a locked payment frame.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd)]
pub struct IouId(pub u64);

/// Stable identity of one enacted payment operation in the frame-local replay
/// ledger.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd)]
pub struct PaymentRecordId(pub u64);

/// One payable mana pip after every announce-time multi-way symbol choice has
/// been made and generic amounts have been expanded.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ManaPip {
    Generic,
    Colored(Color),
    Colorless,
    Snow,
}

impl ManaPip {
    #[must_use]
    pub(crate) fn accepts_alternative(self, class: PipClass) -> bool {
        match (self, class) {
            (Self::Generic, PipClass::Generic) => true,
            (Self::Colored(a), PipClass::Colored(b)) => a == b,
            _ => false,
        }
    }
}

/// The runtime shape of one locked cost obligation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum IouKind {
    ManaPip(ManaPip),
    PayLife(Uint),
    Tap,
    Untap,
    /// A paying action ([CR#601.2h]); `dest` captures its product
    /// ([CR#400.7]) for the ability body to read.
    Act {
        dest: Option<deckmaste_core::DefId>,
        action: RunnableCostAction,
    },
    /// A payment-time object choice writing its destination register
    /// ([CR#601.2b]) — the verbs after it pay through that register.
    Choose(deckmaste_core::Choose),
    /// A payment-time hidden-zone search ([CR#701.23]) writing its register.
    Search(deckmaste_core::Search),
    /// A pure read pinned as a payment subject.
    Let(deckmaste_core::Let),
    TapTotal {
        stat: Stat,
        cmp: Cmp,
        count: Uint,
        filter: Arc<Predicate>,
    },
}

/// One stable obligation plus any payment-action alternatives attached to its
/// mana pip. Alternatives retain declaration order, so a witness can identify
/// one by index without duplicating the action value.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PaymentIou {
    pub id: IouId,
    pub kind: IouKind,
    pub alternatives: Vec<PayAct>,
}

/// The payer's exact answer for one `Fulfill` command. Mana names its already
/// locked coverage entry; chosen and aggregate costs carry the complete object
/// set so validation can precede all mutation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FulfillmentWitness {
    Bound,
    Objects(Vec<ObjectId>),
    CoveredMana,
    PayLife,
}
