//! Copy effects ([CR#707]): the shared `CopySpec` that four delivery sites
//! reference (token copy, enters-as-a-copy, cast-a-copy, becomes-a-copy).

use std::sync::Arc;

use serde::Deserialize;
use serde::Serialize;

use crate::Ability;
use crate::Color;
use crate::ManaCost;
use crate::StatValue;
use crate::Subtype;
use crate::Supertype;
use crate::TypeDef;
use crate::action::EnterRider;
use crate::continuous::Modification;
use crate::count::Characteristic;
use crate::reference::Reference;

/// "A copy of SOURCE, except EXCEPTIONS" ([CR#707.2,707.9]) — the
/// copiable-value spec shared by every copy delivery site.
///
/// Embedded as an ordinary payload field in the `serde::Deserialize, serde::Serialize` enums that
/// reference it (`TokenSpec`/`EnterRider`/`PlayerAction`), the way
/// `NumericOp`/`Characteristic` are embedded in `Modification`/`Count` —
/// `#[derive(serde::Deserialize, serde::Serialize)]` applies to enums only (a struct target is a
/// compile error directing to `#[derive(Expand)]`), so this struct derives
/// `Expand` + serde like those embedded types, not `serde::Deserialize, serde::Serialize` itself.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub struct CopySpec {
    pub source: CopySource,
    #[serde(default, skip_serializing_if = "crate::slice_is_empty")]
    pub exceptions: Vec<CopyException>,
}

/// What a copy effect copies ([CR#707.1]).
#[derive(Debug, Clone, PartialEq, Eq, Hash, serde::Deserialize, serde::Serialize)]
pub enum CopySource {
    /// A referenced object — "a copy of target creature" (Clone, Populate).
    Object(Reference),
    /// The card doing the copying, read from its own zone — the graveyard/exile
    /// self-copy keywords (Embalm/Eternalize copy the exiled card; Offspring
    /// copies the just-cast creature).
    SelfCard,
}

/// An "…, except …" clause on a copy effect ([CR#707.9]). The two exception
/// kinds that ARE characteristic changes or enter-riders reuse the existing
/// grammars; only the copy-application SEMANTICS differ — an embedded
/// `Modification` is folded into the copiable values at layer 1a (not its
/// native layer), and providing/retaining a characteristic drops the source's
/// defining ability for it ([CR#707.9d]).
#[derive(Debug, Clone, PartialEq, Eq, Hash, serde::Deserialize, serde::Serialize)]
pub enum CopyException {
    /// A characteristic modification folded into the copiable values —
    /// "in addition to its other types" ([CR#707.9b]), "except it's 7/7"
    /// ([CR#707.9d]), "and it has [ability]" ([CR#707.9a]).
    Modify(Modification),
    /// "except it doesn't copy its [characteristic]" ([CR#707.9c,707.9d]) — the
    /// copy retains its own value for the named axis; the source's
    /// defining ability for it is not copied.
    Retain(Characteristic),
    /// "except it enters with N +1/+1 counters" ([CR#707.9e]) — explicitly an
    /// additional effect, NOT a characteristic modification; reuses
    /// `EnterRider`.
    AdditionalEffect(EnterRider),
}

/// The copiable characteristics of an object ([CR#707.2]) — a card face's
/// `Characteristics`, produced by the copy model and consumed by token
/// execution here and by `base_values` downstream.
#[derive(Debug, Clone, Default, PartialEq, Eq, Deserialize, Serialize)]
pub struct CopiableValues {
    pub name: Arc<str>,

    #[serde(default, skip_serializing_if = "ManaCost::is_empty")]
    pub mana_cost: ManaCost,

    #[serde(default, skip_serializing_if = "crate::slice_is_empty")]
    pub color_indicator: Vec<Color>,

    #[serde(default, skip_serializing_if = "crate::slice_is_empty")]
    pub supertypes: Vec<Supertype>,

    pub types: Vec<TypeDef>,

    #[serde(default, skip_serializing_if = "crate::slice_is_empty")]
    pub subtypes: Vec<Subtype>,

    #[serde(default, skip_serializing_if = "crate::slice_is_empty")]
    pub abilities: Vec<Ability>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub power: Option<StatValue>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub toughness: Option<StatValue>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub loyalty: Option<StatValue>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub defense: Option<StatValue>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Count;
    use crate::action::EnterRider;
    use crate::continuous::Modification;
    use crate::continuous::NumericOp;
    use crate::count::Characteristic;
    use crate::reference::Reference;

    fn round_trip(spec: &CopySpec) -> CopySpec {
        let ron = crate::ron::options().to_string(spec).unwrap();
        crate::ron::options().from_str(&ron).unwrap()
    }

    #[test]
    fn copyspec_round_trips_all_exception_kinds() {
        let spec = CopySpec {
            source: CopySource::Object(Reference::Reg(crate::RefId(6))),
            exceptions: vec![
                CopyException::Modify(Modification::Power(NumericOp::Set(StatValue::Number(7)))),
                CopyException::Retain(Characteristic::Colors),
                CopyException::AdditionalEffect(EnterRider::WithCounters(
                    "P1P1Counter".into(),
                    Count::Literal(2),
                )),
            ],
        };
        assert_eq!(
            round_trip(&spec),
            spec,
            "CopySpec must survive a RON round trip"
        );
    }
}
