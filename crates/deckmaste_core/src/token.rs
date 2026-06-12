use serde::Deserialize;
use serde::Serialize;

use crate::Expand;
use crate::Subtype;
use crate::Supertype;
use crate::Type;
use crate::ability::Ability;

/// A token permanent definition ([CR#111]). Name, colors, and P/T are omitted
/// here and join when a token definition needs them; the three predefined
/// tokens (Treasure, Clue, Food) don't.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize, Expand)]
pub struct Token {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub supertypes: Vec<Supertype>,
    pub types: Vec<Type>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub subtypes: Vec<Subtype>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub abilities: Vec<Ability>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ability::Ability;
    use crate::ability::ActivatedAbility;
    use crate::action::Action;
    use crate::action::PlayerAction;
    use crate::cost::CostComponent;
    use crate::effect::Effect;
    use crate::mana::ManaSpec;
    use crate::reference::Reference;
    use crate::selection::Selection;

    fn read(source: &str) -> Token { crate::ron::options().from_str(source).unwrap() }

    #[test]
    fn minimal_token_parses() {
        let token = read("Token(types: [Artifact])");
        assert_eq!(token.types, vec![Type::Artifact]);
        assert!(token.supertypes.is_empty());
        assert!(token.subtypes.is_empty());
        assert!(token.abilities.is_empty());
    }

    #[test]
    fn token_round_trips_with_empty_vecs_omitted() {
        let token = Token {
            supertypes: vec![],
            types: vec![Type::Artifact],
            subtypes: vec![],
            abilities: vec![],
        };
        let written = crate::ron::options().to_string(&token).unwrap();
        // Empty vecs must not appear in the output (skip_serializing_if is
        // load-bearing).
        assert!(
            !written.contains("supertypes"),
            "supertypes should be omitted when empty"
        );
        assert!(
            !written.contains("subtypes"),
            "subtypes should be omitted when empty"
        );
        assert!(
            !written.contains("abilities"),
            "abilities should be omitted when empty"
        );
        let reparsed = read(&written);
        assert_eq!(token, reparsed);
    }

    #[test]
    fn treasure_like_token_parses() {
        // Mirrors the structure of plugins/builtin/tokens/Treasure.ron with the
        // macro-expanded forms: `SacrificeThis` -> `Do(Sacrifice(This))`,
        // subtypes omitted (Subtype is a struct requiring plugin expansion).
        let source = "Token(\
            types: [Artifact],\
            abilities: [\
                Activated(\
                    cost: [Tap, Do(Sacrifice(This))],\
                    effect: AddMana(Literal(1), AnyColor),\
                )\
            ],\
        )";
        let token = read(source);
        assert_eq!(token.types, vec![Type::Artifact]);
        assert!(token.subtypes.is_empty());
        assert_eq!(
            token.abilities,
            vec![Ability::Activated(ActivatedAbility {
                window: None,
                cost: vec![
                    CostComponent::Tap,
                    CostComponent::Do(PlayerAction::Sacrifice(Selection::from(Reference::This))),
                ],
                condition: None,
                limits: vec![],
                targets: vec![],
                effect: Effect::Act(Action::By(
                    Reference::You,
                    PlayerAction::AddMana(crate::Count::Literal(1), ManaSpec::AnyColor),
                )),
            })]
        );
    }
}
