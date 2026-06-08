use serde::{Deserialize, Serialize};

use crate::mana::ManaSpec;
use crate::{Quantity, Selection, Token};

/// An intrinsic game verb ([CR#700,701]) — only verbs whose semantics can't
/// be data live here; [CR#701] keyword actions (Sacrifice, Investigate, …) are
/// plugin declarations carried via `Action::Expanded`. Alphabetical. The
/// performer is implicitly the ability's controller unless a slot says
/// otherwise ([CR#608.2]). Object slots are unary `Selection`s.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub enum Action {
    /// Add mana to the controller's mana pool ([CR#106.4]).
    AddMana(Quantity, ManaSpec),
    /// Create a number of token permanents ([CR#111.1,701.7]).
    Create(Quantity, Token),
    /// Deal an amount of damage to a selection ([CR#120.1]).
    DealDamage(Selection, Quantity),
    /// Destroy a selected permanent ([CR#701.8]).
    Destroy(Selection),
    /// You discard a number of cards ([CR#701.9]).
    Discard(Quantity),
    /// Draw a number of cards ([CR#121.1]).
    DrawCards(Quantity),
    /// Exile a selection ([CR#701.13]).
    Exile(Selection),
    /// Gain an amount of life ([CR#119.3]).
    GainLife(Quantity),
    /// Lose an amount of life — implicitly you ([CR#119.3]).
    LoseLife(Quantity),
    /// Return a selection to its owner's hand.
    ReturnToHand(Selection),
    /// Sacrifice a selected permanent ([CR#701.21]).
    Sacrifice(Selection),
    /// Tap a selection ([CR#701.26a]).
    Tap(Selection),
    /// Untap a selection ([CR#701.26b]).
    Untap(Selection),
}

impl Action {
    /// Whether this verb may appear in a cost (`CostComponent::Do`): the
    /// payer performs it, nothing targets ([CR#601.2b..601.2c]). Cost-eligible
    /// verbs are the self-directed ones a player can pay with — sacrifice,
    /// exile, tap, discard.
    #[must_use]
    pub fn is_cost_eligible(&self) -> bool {
        matches!(
            self,
            Action::Sacrifice(_) | Action::Exile(_) | Action::Tap(_) | Action::Discard(_)
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::reference::Reference;
    use crate::selection::Selection;

    #[test]
    fn is_cost_eligible_covers_self_directed_verbs() {
        assert!(Action::Sacrifice(Selection::This).is_cost_eligible());
        assert!(Action::Exile(Selection::This).is_cost_eligible());
        assert!(Action::Tap(Selection::This).is_cost_eligible());
        assert!(Action::Discard(Quantity::Literal(1)).is_cost_eligible());

        assert!(!Action::DrawCards(Quantity::Literal(1)).is_cost_eligible());
        assert!(!Action::GainLife(Quantity::Literal(3)).is_cost_eligible());
        assert!(!Action::AddMana(Quantity::Literal(1), ManaSpec::AnyColor).is_cost_eligible());
        assert!(
            !Action::DealDamage(Selection::from(Reference::This), Quantity::Literal(1))
                .is_cost_eligible()
        );
    }
}
