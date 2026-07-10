use serde::Deserialize;
use serde::Serialize;

use crate::Expand;
use crate::Predicate;
use crate::Property;

/// A rules-defined conferral authored as data under a plugin's `rules/grant/`
/// directory. Read it as: *every object matching `scope` confers `confer`* —
/// the same `Predicate`-scoped shape as [`crate::SbaRule`]'s `scope`, lifted
/// to a bare grant with no `when`/`then` gate (the conferred [`Property`]
/// carries its own conditionality, e.g. a `Replacement`'s trigger event). This
/// is a global, scoped rule so the rule set is swappable (variant Magic)
/// without touching the engine.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Expand, Serialize)]
pub struct ConferralRule {
    pub scope: Predicate,
    pub confer: Property,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn conferral_rule_round_trips() {
        let ron = r"ConferralRule(
            scope: Type(Planeswalker),
            confer: Ability(Static(Replacement(Also(
                would: ZoneChange(what: Ref(This), to: Battlefield),
                also: PutCounters(This, LoyaltyCounter, StatOf(This, Loyalty)),
            )))),
        )";
        let rule: ConferralRule = crate::ron::options().from_str(ron).unwrap();
        let written = crate::ron::options().to_string(&rule).unwrap();
        let reparsed: ConferralRule = crate::ron::options().from_str(&written).unwrap();
        assert_eq!(reparsed, rule);
    }
}
