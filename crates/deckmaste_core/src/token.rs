use std::fmt;
use std::sync::Arc;

use serde::Deserialize;
use serde::Serialize;

use crate::Color;
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
#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
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
                let (ident, variant) = data.variant_seed(crate::IdentSeed)?;
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
#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
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
