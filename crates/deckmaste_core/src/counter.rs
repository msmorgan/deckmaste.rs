use serde::Deserialize;
use serde::Serialize;

use crate::Ident;
use crate::continuous::StaticEffect;

/// A counter-kind declaration ([CR#122], §6): an open `Ident` vocabulary with
/// an optional payload (e.g. a keyword counter's `GainAbility(Flying)`, a stun
/// / shield counter's replacement payload). This is a declaration-file type
/// (like `MacroDef`); where Filters and Actions reference counters they use a
/// bare `Ident`. No loader wiring yet.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub struct CounterDecl {
    pub name: Ident,
    /// The static effect a counter of this kind confers, if any.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub payload: Option<StaticEffect>,
}
