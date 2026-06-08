use serde::ser::Serializer;
use serde::{Deserialize, Serialize};

use crate::Expansion;
use crate::action::PlayerAction;
use crate::mana::ManaCost;

/// A single component of an ability's cost ([CR#601.2b]).
///
/// `Deserialize` is derived (the macro reader synthesizes the `Expanded`
/// stream — `SacrificeThis` and other Cost macros); `Serialize` is **manual**
/// so `Expanded` writes the invocation back rather than the literal struct.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize)]
pub enum CostComponent {
    /// Mana payment, e.g. `Mana([Generic(2)])`.
    Mana(ManaCost),
    /// The {T} symbol ([CR#107.5]).
    Tap,
    /// The {Q} symbol.
    Untap,
    /// Pay by performing a verb: the payer is implicitly you, so this holds a
    /// bare [`PlayerAction`] (no `By` wrapper). Only cost-eligible ones
    /// (`PlayerAction::is_cost_eligible`) belong here — enforced by the cards
    /// crate's validation lint, not the parser.
    Do(PlayerAction),
    /// A remembered `CostComponent` macro invocation (`SacrificeThis`, loyalty
    /// sugar, …).
    Expanded(Expansion<CostComponent>),
}

impl Serialize for CostComponent {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        // serialize_*_variant index arguments are ignored by RON.
        match self {
            CostComponent::Mana(m) => {
                serializer.serialize_newtype_variant("CostComponent", 0, "Mana", m)
            }
            CostComponent::Tap => serializer.serialize_unit_variant("CostComponent", 1, "Tap"),
            CostComponent::Untap => serializer.serialize_unit_variant("CostComponent", 2, "Untap"),
            CostComponent::Do(a) => {
                serializer.serialize_newtype_variant("CostComponent", 3, "Do", a)
            }
            // The invocation, not the struct.
            CostComponent::Expanded(e) => e.serialize(serializer),
        }
    }
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
        use crate::action::PlayerAction;

        assert_eq!(
            read("Mana([Generic(2)])"),
            CostComponent::Mana(ManaCost::from(vec![ManaSymbol::Simple(
                SimpleManaSymbol::Generic(2)
            )])),
        );
        assert_eq!(read("Tap"), CostComponent::Tap);
        assert_eq!(
            read("Do(Sacrifice(This))"),
            CostComponent::Do(PlayerAction::Sacrifice(Selection::from(Reference::This))),
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
