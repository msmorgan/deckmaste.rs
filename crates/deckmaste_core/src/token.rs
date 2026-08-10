use std::fmt;
use std::sync::Arc;

use serde::Deserialize;
use serde::Serialize;

use crate::Color;
use crate::Expand;
use crate::Ident;
use crate::StatValue;
use crate::Subtype;
use crate::Supertype;
use crate::Type;
use crate::ability::Ability;

/// The token position on a `Create` instruction. Two ways to define the token:
/// an inline `Token(types: …)` definition (which the variant name = struct name
/// keeps spelling exactly as files always have), or a PREDEFINED NAME
/// ([CR#111.10]) — `Named(Treasure)` — that the rules define a fixed token for.
/// The predefined definitions live in [`PredefinedToken`]; `Named(name)`
/// resolves to one with [`TokenName::resolve`].
#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Expand, Serialize)]
pub enum TokenSpec {
    /// An inline token definition.
    Token(Arc<Token>),
    /// A predefined token by name ([CR#111.10]) — `Named(Treasure)`. The name
    /// is a bare identifier (it is also the token's `Subtype`).
    Named(TokenName),
    /// A token that's a copy of an object ([CR#707.1]).
    Copy(Arc<crate::CopySpec>),
}

impl From<Token> for TokenSpec {
    fn from(token: Token) -> Self {
        TokenSpec::Token(Arc::new(token))
    }
}

/// The name of a predefined token ([CR#111.10]) — `Treasure`, `Food`, … —
/// spelled as a BARE identifier, exactly like [`KeywordRef`](crate::KeywordRef)
/// / [`CounterRef`](crate::CounterRef) (the name is also the token's
/// `Subtype`). [`resolve`](Self::resolve) maps it to the rules-defined
/// [`Token`] characteristics; an unknown name resolves to `None` (a link-time
/// concern, not a serde one — matching the other bare-ident refs).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct TokenName(pub Ident);

impl TokenName {
    #[must_use]
    pub fn as_str(&self) -> &'static str {
        self.0.as_str()
    }

    /// The rules-defined token this name creates ([CR#111.10]), or `None` if
    /// the name is not (yet) a token deckmaste defines.
    #[must_use]
    pub fn resolve(&self) -> Option<Token> {
        PredefinedToken::from_name(self.as_str()).map(PredefinedToken::token)
    }
}

impl From<&str> for TokenName {
    fn from(s: &str) -> Self {
        TokenName(s.into())
    }
}

impl Expand for TokenName {
    // A leaf: a name, never an expandable value.
    fn expand_all(self) -> Self {
        self
    }
}

impl Serialize for TokenName {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        // A unit variant writes as a bare identifier in RON.
        serializer.serialize_unit_variant("TokenName", 0, self.0.as_str())
    }
}

impl<'de> Deserialize<'de> for TokenName {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        // A bare identifier is a unit enum variant in the serde data model —
        // the same channel `KeywordRef`/`CounterRef` read through.
        struct NameVisitor;
        impl<'de> serde::de::Visitor<'de> for NameVisitor {
            type Value = TokenName;
            fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
                f.write_str("a predefined token name (bare identifier)")
            }
            fn visit_enum<A: serde::de::EnumAccess<'de>>(
                self,
                data: A,
            ) -> Result<Self::Value, A::Error> {
                use serde::de::VariantAccess;
                let (ident, variant) = data.variant_seed(macro_ron::IdentSeed)?;
                variant.unit_variant()?;
                Ok(TokenName(ident))
            }
        }
        deserializer.deserialize_enum("", &[], NameVisitor)
    }
}

/// The predefined tokens deckmaste creates from a bare `Named(<name>)`
/// ([CR#111.10]). Only the rules-defined tokens whose activated-ability bodies
/// use already-built effect primitives are listed; the rest (Powerstone's
/// restricted `{C}`, Map/Junk's explore/play, double-faced Incubator, the Aura
/// Role tokens, …) are deliberately absent and resolve to `None` until their
/// primitives land. Each token's defined characteristics ([CR#111.3]) match the
/// builtin `plugins/builtin/tokens/<Name>.ron` file (a colorless artifact whose
/// only subtype is its own name).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PredefinedToken {
    /// [CR#111.10a]
    Treasure,
    /// [CR#111.10b]
    Food,
    /// [CR#111.10c]
    Gold,
    /// [CR#111.10f]
    Clue,
    /// [CR#111.10g]
    Blood,
    /// [CR#111.10w]
    Vibranium,
}

impl PredefinedToken {
    /// Match a bare name to a predefined token, or `None` for any name not in
    /// the built set.
    #[must_use]
    pub fn from_name(name: &str) -> Option<Self> {
        Some(match name {
            "Treasure" => Self::Treasure,
            "Food" => Self::Food,
            "Gold" => Self::Gold,
            "Clue" => Self::Clue,
            "Blood" => Self::Blood,
            "Vibranium" => Self::Vibranium,
            _ => return None,
        })
    }

    /// The token's printed name (= its sole subtype, [CR#111.10]).
    #[must_use]
    pub fn name(self) -> &'static str {
        match self {
            Self::Treasure => "Treasure",
            Self::Food => "Food",
            Self::Gold => "Gold",
            Self::Clue => "Clue",
            Self::Blood => "Blood",
            Self::Vibranium => "Vibranium",
        }
    }

    /// The rules-defined characteristics ([CR#111.10]). A colorless artifact
    /// token whose single subtype is its own name, carrying the predefined
    /// activated ability (and, for Vibranium, a keyword ability too). The
    /// `Subtype` is reconstructed inline (an artifact-typed subtype, like the
    /// builtin token files expand to).
    #[must_use]
    pub fn token(self) -> Token {
        use crate::ColorOrColorless;
        use crate::Count;
        use crate::EventFilter;
        use crate::KeywordAbility;
        use crate::ManaProduction;
        use crate::ManaRider;
        use crate::ManaSpec;
        use crate::ObjectKind;
        use crate::Predicate;
        use crate::StaticEffect;
        use crate::ability::ActivatedAbility;
        use crate::action::Action;
        use crate::action::LifeOp;
        use crate::cost::CostComponent;
        use crate::effect::OneShotEffect;
        use crate::mana::ManaCost;
        use crate::mana::ManaSymbol;
        use crate::mana::SimpleManaSymbol;
        use crate::reference::Reference;

        let subtype = Subtype {
            name: self.name().into(),
            types: vec![Type::Artifact].into(),
            confers: [].into(),
        };
        let sac = CostComponent::do_action(Action::Sacrifice(Reference::You, Reference::This));
        let mana = |n: u32| {
            let symbols: Arc<[ManaSymbol]> =
                [ManaSymbol::Simple(SimpleManaSymbol::Generic(n))].into();
            CostComponent::Mana(ManaCost::from(symbols))
        };
        let add_any =
            || Action::AddMana(Reference::You, Count::Literal(1), ManaSpec::AnyColor.into());
        // Vibranium's "Add {C}. This mana can't be spent to cast a nonartifact
        // spell." ([CR#111.10w]) — a colorless unit carrying a `SpendOnly`
        // rider ([CR#106.6]) whose filter admits everything EXCEPT a
        // nonartifact spell (abilities and artifact spells stay payable).
        let restricted_colorless = || {
            let nonartifact_spell = Predicate::And(
                vec![
                    Predicate::Kind(ObjectKind::Spell),
                    Predicate::Not(Arc::new(Predicate::r#type(Type::Artifact))),
                ]
                .into(),
            );
            Action::AddMana(
                Reference::You,
                Count::Literal(1),
                ManaProduction::WithRiders {
                    mana: ManaSpec::Specific(ColorOrColorless::Colorless),
                    riders: vec![ManaRider::SpendOnly(Predicate::Not(Arc::new(
                        nonartifact_spell,
                    )))]
                    .into(),
                },
            )
        };
        // Indestructible ([CR#702.12b]) as a `Composite` keyword — the printed
        // name plus the event-side can't-happen it stands for ([CR#614.17]),
        // mirroring the builtin `Indestructible` keyword macro: the
        // `Act(Destroy(this))` keyword action can't happen.
        let indestructible = || {
            Ability::Keyword(KeywordAbility::Composite {
                name: "Indestructible".into(),
                abilities: vec![Ability::r#static(StaticEffect::CantHappen(
                    EventFilter::Act {
                        verb: crate::VerbName::from("Destroy"),
                        who: Predicate::Any,
                        on: Predicate::Ref(Reference::This),
                        cause: None,
                    },
                ))],
            })
        };

        // (leading keyword abilities, activated-ability cost, effect)
        let (keywords, cost, effect): (Arc<[Ability]>, Arc<[CostComponent]>, OneShotEffect) =
            match self {
                // [CR#111.10a] "{T}, Sacrifice this token: Add one mana of any color."
                Self::Treasure => (
                    [].into(),
                    vec![CostComponent::Tap, sac].into(),
                    OneShotEffect::Act(add_any()),
                ),
                // [CR#111.10b] "{2}, {T}, Sacrifice this token: You gain 3 life."
                Self::Food => (
                    [].into(),
                    vec![mana(2), CostComponent::Tap, sac].into(),
                    OneShotEffect::Act(Action::ChangeLife(
                        Reference::You,
                        LifeOp::Up(Count::Literal(3)),
                    )),
                ),
                // [CR#111.10c] "Sacrifice this token: Add one mana of any color."
                Self::Gold => ([].into(), vec![sac].into(), OneShotEffect::Act(add_any())),
                // [CR#111.10f] "{2}, Sacrifice this token: Draw a card."
                Self::Clue => (
                    [].into(),
                    vec![mana(2), sac].into(),
                    crate::OneShotEffect::draw(Reference::You, Count::Literal(1)),
                ),
                // [CR#111.10g] "{1}, {T}, Discard a card, Sacrifice this token: Draw a card."
                Self::Blood => (
                    [].into(),
                    vec![
                        mana(1),
                        CostComponent::Tap,
                        CostComponent::do_action(crate::Action::discard(
                            Reference::You,
                            Count::Literal(1),
                            false,
                        )),
                        sac,
                    ]
                    .into(),
                    crate::OneShotEffect::draw(Reference::You, Count::Literal(1)),
                ),
                // [CR#111.10w] indestructible; "{T}: Add {C}. This mana can't be
                // spent to cast a nonartifact spell."
                Self::Vibranium => (
                    vec![indestructible()].into(),
                    vec![CostComponent::Tap].into(),
                    OneShotEffect::Act(restricted_colorless()),
                ),
            };

        let mut abilities: Vec<Ability> = keywords.to_vec();
        abilities.push(Ability::activated(ActivatedAbility {
            ability_word: None,
            from: None,
            window: None,
            cost: cost.into(),
            condition: None,
            limits: [].into(),
            effect,
        }));

        Token {
            name: None,
            color_indicator: [].into(),
            supertypes: [].into(),
            types: vec![Type::Artifact.def()].into(),
            subtypes: vec![subtype].into(),
            abilities: abilities.into(),
            power: None,
            toughness: None,
        }
    }
}

/// A token permanent definition ([CR#111]). The creating effect defines the
/// token's characteristics ([CR#111.3] — functionally equivalent to printed
/// values). Color rides a color indicator ([CR#202.2e]: a token has no mana
/// cost, so its defined color is carried the same way `CardFace` carries a
/// printed indicator). `name` is usually omitted — an unnamed token defaults
/// to its subtypes plus "Token" at synthesis ([CR#111.4]) — except when the
/// creating effect DOES specify a name, which a copy effect always does: a
/// copy acquires the source's name as a copiable value ([CR#707.2]), so
/// `token_from_copiable` (`deckmaste_engine::copy`) sets this explicitly
/// rather than letting a copy token's name resynthesize from its subtypes.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Expand, Serialize)]
pub struct Token {
    /// An explicit name ([CR#111.3]/[CR#707.2]) — `None` synthesizes at
    /// [CR#111.4] (subtypes + "Token") the way an unnamed token always has.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<Arc<str>>,
    #[serde(default, skip_serializing_if = "crate::slice_is_empty")]
    pub color_indicator: Arc<[Color]>,
    #[serde(default, skip_serializing_if = "crate::slice_is_empty")]
    pub supertypes: Arc<[Supertype]>,
    pub types: Arc<[crate::TypeDef]>,
    #[serde(default, skip_serializing_if = "crate::slice_is_empty")]
    pub subtypes: Arc<[Subtype]>,
    #[serde(default, skip_serializing_if = "crate::slice_is_empty")]
    pub abilities: Arc<[Ability]>,
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
    use crate::cost::CostComponent;
    use crate::effect::OneShotEffect;
    use crate::mana::ManaSpec;
    use crate::reference::Reference;

    fn read(source: &str) -> Token {
        crate::ron::options().from_str(source).unwrap()
    }

    /// `Named(Treasure)` reads as a BARE identifier (the name is also a
    /// `Subtype`), round-trips bare, and a quoted string must NOT parse —
    /// matching the `KeywordRef`/`CounterRef` channel.
    #[test]
    fn token_spec_named_reads_bare() {
        let spec: TokenSpec = crate::ron::options().from_str("Named(Treasure)").unwrap();
        assert_eq!(spec, TokenSpec::Named(TokenName::from("Treasure")));
        let written = crate::ron::options().to_string(&spec).unwrap();
        assert_eq!(written, "Named(Treasure)", "writes bare, no quotes");
        let err = crate::ron::options()
            .from_str::<TokenSpec>("Named(\"Treasure\")")
            .unwrap_err();
        let err = err.to_string();
        assert!(
            err.contains("Expected"),
            "a quoted token name is not a valid TokenSpec: {err}"
        );
    }

    /// `TokenSpec::Copy(CopySpec{…})` reads/writes FLAT — the newtype variant
    /// unwraps the way `TokenSpec::Token(Token{…})` does
    /// (`UNWRAP_VARIANT_NEWTYPES`), so a macro/card spells `Copy(source: …,
    /// exceptions: […])`, never the double-paren `Copy((source: …))`.
    /// Proves the spelling the copy-consuming keyword macros
    /// (Populate/Embalm/Eternalize/Offspring) rely on.
    #[test]
    fn token_spec_copy_reads_flat() {
        use crate::CollectionOp;
        use crate::Color;
        use crate::CopyException;
        use crate::CopySource;
        use crate::CopySpec;
        use crate::Modification;
        use crate::NumericOp;

        let spec = TokenSpec::Copy(
            CopySpec {
                source: CopySource::SelfCard,
                exceptions: vec![
                    CopyException::Modify(Modification::Power(NumericOp::Set(StatValue::Number(
                        4,
                    )))),
                    CopyException::Modify(Modification::Colors(CollectionOp::Set(
                        vec![Color::Black].into(),
                    ))),
                    CopyException::Modify(Modification::Subtypes(CollectionOp::Add(
                        "Zombie".into(),
                    ))),
                ],
            }
            .into(),
        );
        let written = crate::ron::options().to_string(&spec).unwrap();
        assert_eq!(
            written,
            "Copy(source:SelfCard,exceptions:[Modify(Power(Set(4))),\
             Modify(Colors(Set([Black]))),Modify(Subtypes(Add(Zombie)))])",
            "TokenSpec::Copy spells FLAT (newtype unwrapped); the writer is compact — \
             the `Subtypes` axis writes its `SubtypeRef` as the bare name, the \
             reader accepts the resolved struct (or a subtype-macro name)"
        );
        // The `Subtypes` axis is a `SubtypeRef`: the bare write is re-read by a
        // macro-aware reader (a subtype-macro name) or, here in core, the
        // resolved fused struct — by-name identity makes it equal the name-only
        // build. The spaced, human-readable spelling macros/cards actually write.
        let spaced: TokenSpec = crate::ron::options()
            .from_str(
                "Copy(source: SelfCard, exceptions: [Modify(Power(Set(4))), \
                 Modify(Colors(Set([Black]))), \
                 Modify(Subtypes(Add(name: \"Zombie\", types: [Creature])))])",
            )
            .unwrap();
        assert_eq!(
            spaced, spec,
            "the resolved macro/card spelling parses to a by-name-equal ref"
        );
    }

    /// A predefined name resolves to its rules-defined [CR#111.10]
    /// characteristics — a colorless artifact whose sole subtype is its own
    /// name. Matches the builtin `Treasure.ron` (the [CR#111.10a] body).
    #[test]
    fn named_treasure_resolves_to_rules_token() {
        let token = TokenName::from("Treasure").resolve().unwrap();
        assert_eq!(token.types, vec![Type::Artifact.def()].into());
        assert_eq!(token.subtypes.len(), 1);
        assert_eq!(token.subtypes[0].name, "Treasure");
        assert!(token.color_indicator.is_empty(), "[CR#111.10a]: colorless");
        assert_eq!(
            token.abilities,
            vec![Ability::activated(ActivatedAbility {
                ability_word: None,
                from: None,
                window: None,
                cost: crate::Cost(
                    vec![
                        CostComponent::Tap,
                        CostComponent::do_action(Action::Sacrifice(
                            Reference::You,
                            Reference::This
                        )),
                    ]
                    .into(),
                ),
                condition: None,
                limits: vec![].into(),
                effect: OneShotEffect::Act(Action::AddMana(
                    Reference::You,
                    crate::Count::Literal(1),
                    ManaSpec::AnyColor.into()
                )),
            })]
            .into()
        );
    }

    /// An unbuilt predefined token (Powerstone) and a non-token name resolve
    /// to `None` — the parser must decline these, not emit a `Named` that
    /// can't resolve.
    #[test]
    fn unbuilt_or_unknown_names_resolve_to_none() {
        assert!(TokenName::from("Powerstone").resolve().is_none());
        assert!(TokenName::from("Bogus").resolve().is_none());
    }

    /// The `Create` position reads through the single-variant wrapper with
    /// the SAME spelling files always used — `Token(types: …)`, the struct
    /// name, is now the variant name.
    #[test]
    fn token_spec_reads_the_token_spelling() {
        let spec: TokenSpec = crate::ron::options()
            .from_str("Token(types: [TypeDef(name: \"Artifact\", permanent: true)])")
            .unwrap();
        assert_eq!(
            spec,
            TokenSpec::Token(
                Token {
                    name: None,
                    color_indicator: vec![].into(),
                    supertypes: vec![].into(),
                    types: vec![Type::Artifact.def()].into(),
                    subtypes: vec![].into(),
                    abilities: vec![].into(),
                    power: None,
                    toughness: None,
                }
                .into()
            )
        );
    }

    #[test]
    fn minimal_token_parses() {
        let token = read("Token(types: [TypeDef(name: \"Artifact\", permanent: true)])");
        assert_eq!(token.types, vec![Type::Artifact.def()].into());
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
            name: None,
            color_indicator: vec![].into(),
            supertypes: vec![].into(),
            types: vec![Type::Artifact.def()].into(),
            subtypes: vec![].into(),
            abilities: vec![].into(),
            power: None,
            toughness: None,
        };
        let written = crate::ron::options().to_string(&token).unwrap();
        // Empty vecs must not appear in the output (skip_serializing_if is
        // load-bearing).
        // `written` legitimately contains "name" as part of the nested
        // `TypeDef(name: "Artifact", ...)` — check specifically for the
        // Token-level `name:` key, not a bare substring match.
        assert!(
            !written.contains("Token(name:") && !written.contains(", name:"),
            "the Token's own `name` field should be omitted when None: {written}"
        );
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
        let token = read(
            "Token(color_indicator: [Red], types: [TypeDef(name: \"Creature\", permanent: true)], power: 1, toughness: 1)",
        );
        assert_eq!(token.color_indicator, vec![Color::Red].into());
        assert_eq!(token.types, vec![Type::Creature.def()].into());
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
            types: [TypeDef(name: \"Artifact\", permanent: true)],\
            abilities: [\
                Activated(\
                    cost: [Tap, Act(Sacrifice(You, This))],\
                    effect: AddMana(You, Literal(1), AnyColor),\
                )\
            ],\
        )";
        let token = read(source);
        assert_eq!(token.types, vec![Type::Artifact.def()].into());
        assert!(token.subtypes.is_empty());
        assert_eq!(
            token.abilities,
            vec![Ability::activated(ActivatedAbility {
                ability_word: None,
                from: None,
                window: None,
                cost: crate::Cost(
                    vec![
                        CostComponent::Tap,
                        CostComponent::do_action(Action::Sacrifice(
                            Reference::You,
                            Reference::This
                        )),
                    ]
                    .into(),
                ),
                condition: None,
                limits: vec![].into(),
                effect: OneShotEffect::Act(Action::AddMana(
                    Reference::You,
                    crate::Count::Literal(1),
                    ManaSpec::AnyColor.into(),
                )),
            })]
            .into()
        );
    }
}
