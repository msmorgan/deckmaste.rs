use serde::Deserialize;
use serde::Serialize;

use crate::Color;
use crate::Expand;
use crate::ManaCost;
use crate::Subtype;
use crate::Supertype;
use crate::SupportsMacros;
use crate::TypeDef;
use crate::ability::Ability;

#[derive(Debug, Clone, Default, PartialEq, Eq, Deserialize, Expand, Serialize)]
pub struct CardFace {
    pub name: String,

    #[serde(default, skip_serializing_if = "ManaCost::is_empty")]
    pub mana_cost: ManaCost,

    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub color_indicator: Vec<Color>,

    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub supertypes: Vec<Supertype>,

    pub types: Vec<TypeDef>,

    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub subtypes: Vec<Subtype>,

    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub abilities: Vec<Ability>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub power: Option<StatValue>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub toughness: Option<StatValue>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub loyalty: Option<StatValue>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub defense: Option<StatValue>,
}

/// How a two-faced card's faces are arranged — the Idris `FaceLayout`. The
/// layouts share ONE card shape (two full faces, [CR#712.8] — each face has
/// its own characteristics); what differs per layout — transforming, casting
/// either face, casting halves, the adventure exile state, flipping — is the
/// engine's job, not the grammar's. Single-faced "layouts" (saga, class,
/// leveler, …) are NOT here: their mechanics ride subtypes and abilities on a
/// `Normal` card.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Deserialize, Expand, Serialize)]
pub enum FaceLayout {
    /// A transforming (nonmodal) double-faced card ([CR#712.2]).
    Transforming,
    /// A modal double-faced card — play either face ([CR#712.3]).
    ModalDfc,
    /// A split card — two halves on one face ([CR#709.1]).
    Split,
    /// An adventurer card — a permanent face with an Adventure spell face
    /// ([CR#715.1]).
    Adventure,
    /// A Kamigawa-style flip card — one card, flipped half ([CR#710.1]).
    Flip,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Expand, Serialize)]
// `TwoFaced` (two full `CardFace`s) is inherently larger than `Normal` (one).
// Boxing a face would push `Box::new` into every construction/read site and
// complicate the RON derive for no runtime gain — a `Card` is a rarely-copied
// authoring value, not a hot enum.
#[expect(
    clippy::large_enum_variant,
    reason = "leaf card model; boxing a face buys nothing"
)]
pub enum Card {
    Normal(CardFace),
    /// A TWO-faced card: `front` (the primary/default face) and `back`,
    /// arranged per `layout` — the Idris `TwoFaced`. Each face is a full
    /// [`CardFace`] with its own characteristics ([CR#712.8]).
    TwoFaced {
        layout: FaceLayout,
        front: CardFace,
        back: CardFace,
    },
}

/// The `#[macro_ron(literal)]` Number variant reads/writes a bare integer —
/// `3`, not `Number(3)` — mirroring [`Count::Literal`](crate::Count). A literal
/// payload is a bare scalar, so (unlike an embed) it needs no `SupportsMacros`
/// on `Int`.
#[derive(Debug, Clone, PartialEq, Eq, Hash, SupportsMacros)]
pub enum StatValue {
    // Power or toughness set by a characteristic-defining ability ([CR#208.2a] —
    // the `*`, worded "[this creature's] power/toughness is equal to …", set per
    // the CDA rule [CR#604.3]). Any power or toughness containing * is
    // essentially reminder text.
    DefinedByAbility,

    // Loyalty set to X from casting cost.
    Variable,

    // A printed numeric value. Signed (`Int`, not `Uint`): a printed power or
    // toughness — and thus a base value — can be less than zero ([CR#107.1b],
    // e.g. Spinal Parasite's -1/-1), unlike the game's otherwise non-negative
    // numbers.
    #[macro_ron(literal)]
    Number(crate::Int),

    // A dynamic value drawn from the amount language — a base power/toughness
    // set to a computed count ([CR#208.2a], the CDA "*/*": Tarmogoyf's distinct
    // card types, etc.). Embeds [`Count`](crate::Count) so a bare `CountOf(…)` /
    // `CountDistinct(…)` at a stat position reads straight through. Mirrors the
    // Idris `CharValue Power = Count` settable-value type (`idris/src/Core.idr`).
    #[macro_ron(embed)]
    Count(crate::Count),
}

impl StatValue {
    /// The fixed scalar this value resolves to WITHOUT a game state — `Some`
    /// for a printed [`Number`](StatValue::Number) or a literal-count
    /// embed, `None` for a dynamic count, `X`
    /// ([`Variable`](StatValue::Variable)), or a CDA
    /// marker ([`DefinedByAbility`](StatValue::DefinedByAbility)). The pure
    /// characteristic fold uses this to apply a `Set` it can evaluate and skip
    /// one it can't.
    #[must_use]
    pub fn literal_value(&self) -> Option<crate::Int> {
        match self {
            StatValue::Number(n) => Some(*n),
            StatValue::Count(c) => c.literal_value().and_then(|u| crate::Int::try_from(u).ok()),
            StatValue::Variable | StatValue::DefinedByAbility => None,
        }
    }
}
