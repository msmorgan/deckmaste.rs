use serde::{Deserialize, Serialize};

use crate::action::Action;
use crate::mana::ManaCost;

/// A single component of an ability's cost (CR 601.2b).
#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub enum CostComponent {
    /// Mana payment, e.g. `Mana([Generic(2)])`.
    Mana(ManaCost),
    /// The {T} symbol (CR 107.5).
    Tap,
    /// The {Q} symbol.
    Untap,
    /// Pay by performing a verb: only cost-eligible Actions
    /// (`Action::is_cost_eligible`) belong here — enforced by the cards
    /// crate's validation lint, not the parser.
    Do(Action),
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mana::{ManaCost, ManaSymbol, SimpleManaSymbol};
    use crate::reference::Reference;
    use crate::selection::Selection;

    fn read(source: &str) -> CostComponent { crate::ron::options().from_str(source).unwrap() }

    #[test]
    fn cost_components_parse() {
        assert_eq!(
            read("Mana([Generic(2)])"),
            CostComponent::Mana(ManaCost::from(vec![ManaSymbol::Simple(
                SimpleManaSymbol::Generic(2)
            )])),
        );
        assert_eq!(read("Tap"), CostComponent::Tap);
        assert_eq!(
            read("Do(Sacrifice(This))"),
            CostComponent::Do(Action::Sacrifice(Selection::from(Reference::This))),
        );
    }

    #[test]
    fn cost_list_round_trips() {
        let source = "[Mana([Generic(2)]),Tap,Do(Sacrifice(This))]";
        let parsed: Vec<CostComponent> = crate::ron::options().from_str(source).unwrap();
        let written = crate::ron::options().to_string(&parsed).unwrap();
        let reparsed: Vec<CostComponent> = crate::ron::options().from_str(&written).unwrap();
        assert_eq!(parsed, reparsed);
    }
}
