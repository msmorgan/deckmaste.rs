use serde::{Deserialize, Serialize};

use crate::cost::CostComponent;
use crate::effect::Effect;
use crate::{Ident, Selection};

// Temporary types.
type ParamValue = String;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub struct Expanded<T> {
    pub params: Vec<ParamValue>,
    pub value: Box<T>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub struct KeywordAbility {
    pub keyword: Ident,
    pub expanded: Expanded<Ability>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub struct SpellAbility {
    pub targets: Vec<Selection>,
    pub effect: Effect,
}

/// An activated ability: paid with a cost and produces an effect. Targets
/// are collected here for now; the `Resolvable` wrapper arrives with Modal.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub struct ActivatedAbility {
    pub cost: Vec<CostComponent>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub targets: Vec<Selection>,
    pub effect: Effect,
}

// The struct-carrying variants read flat in RON — `Spell(targets: ..., ...)`,
// not `Spell((targets: ...))` — via the unwrap_variant_newtypes extension.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub enum Ability {
    Static,
    Activated(ActivatedAbility),
    Triggered,
    Spell(SpellAbility),
    Keyword(KeywordAbility),
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::action::Action;
    use crate::cost::CostComponent;
    use crate::effect::Effect;

    fn read_ability(source: &str) -> Ability { crate::ron::options().from_str(source).unwrap() }

    #[test]
    fn activated_ability_parses() {
        let ability = read_ability("Activated(cost: [Tap], effect: DrawCards(1))");
        assert_eq!(
            ability,
            Ability::Activated(ActivatedAbility {
                cost: vec![CostComponent::Tap],
                targets: vec![],
                effect: Effect::Act(Action::DrawCards(1)),
            })
        );
    }

    #[test]
    fn activated_ability_round_trips() {
        let ability = Ability::Activated(ActivatedAbility {
            cost: vec![CostComponent::Tap],
            targets: vec![],
            effect: Effect::Act(Action::DrawCards(1)),
        });
        let written = crate::ron::options().to_string(&ability).unwrap();
        let reparsed = read_ability(&written);
        assert_eq!(ability, reparsed);
    }
}
