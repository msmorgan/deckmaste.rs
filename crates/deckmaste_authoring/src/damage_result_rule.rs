use serde::Deserialize;
use serde::Serialize;

use crate::CounterRef;
use crate::Expand;
use crate::Predicate;

/// A rules-as-data damage result authored under a plugin's `rules/damage/`
/// directory. Read it as: *when `amount` damage is dealt to a permanent
/// matching `recipient`, remove that many `remove` counters from it* — the
/// data form of [CR#120.3c] (planeswalker loyalty) / [CR#120.3h] (battle
/// defense). It is applied at deal time IN ADDITION to any intrinsic result
/// ([CR#120.3]: damage has "one or more results" — a creature-planeswalker is
/// still marked as a creature *and* loses loyalty as a planeswalker). The
/// removed count is always the damage event's `amount` ("that many"), so no
/// count field is needed. This is a global, `Predicate`-scoped rule so the rule
/// set is swappable (variant Magic) without touching the engine — the same
/// lifecycle as [`crate::SbaRule`] / [`crate::ConferralRule`].
#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Expand, Serialize)]
pub struct DamageResultRule {
    pub recipient: Predicate,
    pub remove: CounterRef,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn damage_result_rule_round_trips() {
        let ron = "DamageResultRule(recipient: Supertype(Legendary), remove: LoyaltyCounter)";
        let rule: DamageResultRule = crate::ron::options().from_str(ron).unwrap();
        assert_eq!(rule.remove, CounterRef::from("LoyaltyCounter"));
        let written = crate::ron::options().to_string(&rule).unwrap();
        let reparsed: DamageResultRule = crate::ron::options().from_str(&written).unwrap();
        assert_eq!(reparsed, rule);
    }
}
