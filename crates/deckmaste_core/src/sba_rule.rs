use serde::Deserialize;
use serde::Serialize;

use crate::Condition;
use crate::Expand;
use crate::OneShotEffect;
use crate::Predicate;

/// A rules-defined state-based action ([CR#704]) authored as data under a
/// plugin's `rules/sba/` directory. Read it as: *for every battlefield object
/// matching `scope`, with `This` bound to that object, if `when` holds the
/// engine performs `then`*. `scope` is the binding domain for `This` (a
/// `Predicate` over object class — `Type(Creature)`, etc.), checked before
/// `when`. This is the same `when`/`then` shape conferred statics use (see
/// [`crate::StaticEffect::Sba`]), lifted to a global, scoped rule so the rule
/// set is swappable (variant Magic) without touching the engine.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Expand, Serialize)]
pub struct SbaRule {
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
        // Construct directly — pins the field names/types. (RON round-trip
        // through the macro reader is covered by the loader test in Task A2.)
        let rule = SbaRule {
            scope: Predicate::Characteristic(CharacteristicPredicate::Type(Type::Creature.name())),
            when: Condition::YourTurn,
            then: OneShotEffect::Sequentially(vec![]),
        };
        assert!(matches!(rule.scope, Predicate::Characteristic(_)));
        assert!(matches!(rule.when, Condition::YourTurn));
        assert!(matches!(rule.then, OneShotEffect::Sequentially(_)));
    }
}
