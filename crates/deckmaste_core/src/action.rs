use serde::{Deserialize, Serialize};

use crate::mana::ManaSpec;
use crate::{Selection, Uint};

/// An intrinsic game verb (CR 700, 701). Alphabetical. The performer is
/// implicitly the ability's controller unless a card specifies otherwise.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub enum Action {
    /// Add mana to the controller's mana pool (CR 106.1, 701.3).
    AddMana(Uint, ManaSpec),
    /// Deal an amount of damage to a selection (CR 119.1, 701.25).
    DealDamage(Selection, Uint),
    /// Draw a number of cards (CR 120.1, 701.4).
    DrawCards(Uint),
    /// Gain an amount of life (CR 119.7, 701.6).
    GainLife(Uint),
    /// Sacrifice a selected permanent (CR 701.16).
    Sacrifice(Selection),
}

impl Action {
    /// Whether this verb may appear in a cost (`CostComponent::Do`): the
    /// payer performs it, nothing targets (CR 601.2b-c).
    #[must_use]
    pub fn is_cost_eligible(&self) -> bool { matches!(self, Action::Sacrifice(_)) }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::reference::Reference;
    use crate::selection::Selection;

    fn read(source: &str) -> Action { crate::ron::options().from_str(source).unwrap() }

    #[test]
    fn is_cost_eligible_only_sacrifice() {
        assert!(Action::Sacrifice(Selection::That(Reference::This)).is_cost_eligible());
        assert!(!Action::DrawCards(1).is_cost_eligible());
        assert!(!Action::GainLife(3).is_cost_eligible());
        assert!(!Action::AddMana(1, ManaSpec::AnyColor).is_cost_eligible());
        assert!(!Action::DealDamage(Selection::That(Reference::This), 1).is_cost_eligible());
    }

    #[test]
    fn actions_round_trip() {
        let options = crate::ron::options();
        let cases = ["DrawCards(1)", "GainLife(3)", "Sacrifice(That(This))"];
        for source in cases {
            let parsed = read(source);
            let written = options.to_string(&parsed).unwrap();
            assert_eq!(read(&written), parsed, "round-trip failed for: {source}");
        }
    }
}
