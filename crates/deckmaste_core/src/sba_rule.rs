use serde::Deserialize;
use serde::Serialize;

use crate::Condition;
use crate::OneShotEffect;
use crate::Predicate;
use crate::Region;

/// A rules-defined state-based action ([CR#704]) authored as data under a
/// plugin's `rules/sba/` directory. Read it as: *for every battlefield object
/// matching `scope`, with `This` bound to that object, if `when` holds the
/// engine performs `then`*. `scope` is the binding domain for `This` (a
/// `Predicate` over object class — `Type(Creature)`, etc.), checked before
/// `when`. This is the same `condition`/`effect` shape a type or subtype's own
/// [CR#704] rule uses (see [`crate::Property::StateBased`]), lifted to a
/// global, scoped rule so the rule set is swappable (variant Magic) without
/// touching the engine.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub struct SbaRule {
    pub region: Region<SbaBody>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub struct SbaBody {
    pub scope: Predicate,
    pub when: Condition,
    pub then: OneShotEffect,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::CharacteristicPredicate;
    use crate::Condition;
    use crate::OneShotEffect;
    use crate::Predicate;
    use crate::Type;

    #[test]
    fn sba_rule_has_scope_when_then() {
        // Construct directly to pin the field names and types.
        let rule = SbaRule {
            region: Region::closed(SbaBody {
                scope: Predicate::Characteristic(CharacteristicPredicate::Type(
                    Type::Creature.into(),
                )),
                when: Condition::YourTurn,
                then: OneShotEffect::Sequentially(vec![].into()),
            }),
        };
        assert!(matches!(
            rule.region.body.scope,
            Predicate::Characteristic(_)
        ));
        assert!(matches!(rule.region.body.when, Condition::YourTurn));
        assert!(matches!(
            rule.region.body.then,
            OneShotEffect::Sequentially(_)
        ));
    }
}
