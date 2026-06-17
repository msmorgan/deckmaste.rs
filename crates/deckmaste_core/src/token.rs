use serde::Deserialize;
use serde::Serialize;

use crate::Color;
use crate::Expand;
use crate::StatValue;
use crate::Subtype;
use crate::Supertype;
use crate::Type;
use crate::ability::Ability;

/// The token position on a `Create` instruction — a single-variant enum
/// on purpose (the accretion point): "create a Treasure token" by
/// PREDEFINED NAME ([CR#111.10]) is a foreseeable `Named(Ident)` sibling,
/// and it will land here without respelling existing files (which already
/// read `Token(types: …)` — the variant name is the struct name).
#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Expand, Serialize)]
pub enum TokenSpec {
    /// An inline token definition.
    Token(Token),
}

impl From<Token> for TokenSpec {
    fn from(token: Token) -> Self {
        TokenSpec::Token(token)
    }
}

/// A token permanent definition ([CR#111]). The creating effect defines the
/// token's characteristics ([CR#111.3] — functionally equivalent to printed
/// values). Color rides a color indicator ([CR#202.2e]: a token has no mana
/// cost, so its defined color is carried the same way `CardFace` carries a
/// printed indicator). Name is still omitted — it defaults to the subtypes
/// plus "Token" at synthesis ([CR#111.4]).
#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Expand, Serialize)]
pub struct Token {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub color_indicator: Vec<Color>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub supertypes: Vec<Supertype>,
    pub types: Vec<Type>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub subtypes: Vec<Subtype>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub abilities: Vec<Ability>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub power: Option<StatValue>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub toughness: Option<StatValue>,
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

    fn read(source: &str) -> Token {
        crate::ron::options().from_str(source).unwrap()
    }

    /// The `Create` position reads through the single-variant wrapper with
    /// the SAME spelling files always used — `Token(types: …)`, the struct
    /// name, is now the variant name.
    #[test]
    fn token_spec_reads_the_token_spelling() {
        let spec: TokenSpec = crate::ron::options()
            .from_str("Token(types: [Artifact])")
            .unwrap();
        assert_eq!(
            spec,
            TokenSpec::Token(Token {
                color_indicator: vec![],
                supertypes: vec![],
                types: vec![Type::Artifact],
                subtypes: vec![],
                abilities: vec![],
                power: None,
                toughness: None,
            })
        );
    }

    #[test]
    fn minimal_token_parses() {
        let token = read("Token(types: [Artifact])");
        assert_eq!(token.types, vec![Type::Artifact]);
        assert!(token.supertypes.is_empty());
        assert!(token.subtypes.is_empty());
        assert!(token.abilities.is_empty());
        assert!(token.color_indicator.is_empty());
        assert!(token.power.is_none());
        assert!(token.toughness.is_none());
    }

    #[test]
    fn token_round_trips_with_empty_vecs_omitted() {
        let token = Token {
            color_indicator: vec![],
            supertypes: vec![],
            types: vec![Type::Artifact],
            subtypes: vec![],
            abilities: vec![],
            power: None,
            toughness: None,
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
        assert!(
            !written.contains("color_indicator"),
            "color_indicator should be omitted when empty"
        );
        assert!(
            !written.contains("power"),
            "power should be omitted when None"
        );
        assert!(
            !written.contains("toughness"),
            "toughness should be omitted when None"
        );
        let reparsed = read(&written);
        assert_eq!(token, reparsed);
    }

    /// A creature token carries its defined color [CR#202.2e] and P/T
    /// [CR#111.3]; the new fields round-trip and stay omitted when empty.
    #[test]
    fn token_carries_color_and_pt() {
        let token =
            read("Token(color_indicator: [Red], types: [Creature], power: 1, toughness: 1)");
        assert_eq!(token.color_indicator, vec![Color::Red]);
        assert_eq!(token.types, vec![Type::Creature]);
        assert_eq!(token.power, Some(StatValue::Number(1)));
        assert_eq!(token.toughness, Some(StatValue::Number(1)));
        let written = crate::ron::options().to_string(&token).unwrap();
        assert_eq!(read(&written), token);
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
                    CostComponent::Do(Box::new(PlayerAction::Sacrifice(Selection::from(
                        Reference::This
                    )))),
                ],
                condition: None,
                limits: vec![],
                effect: Effect::Act(Action::By(
                    Reference::You,
                    PlayerAction::AddMana(crate::Count::Literal(1), ManaSpec::AnyColor.into()),
                )),
            })]
        );
    }
}
