/// Core RON spells numeric values explicitly as `Number(3)`.
#[derive(Debug, Clone, PartialEq, Eq, Hash, serde::Deserialize, serde::Serialize)]
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
    Number(crate::Int),

    // A dynamic value drawn from the amount language — a base power/toughness
    // set to a computed count ([CR#208.2a], the CDA "*/*": Tarmogoyf's distinct
    // card types, etc.). Embeds [`Count`](crate::Count) so a bare `CountOf(…)` /
    // `CountDistinct(…)` at a stat position reads straight through. Mirrors the
    // Idris `CharValue Power = Count` settable-value type (`idris/src/Semantics.idr`).
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
