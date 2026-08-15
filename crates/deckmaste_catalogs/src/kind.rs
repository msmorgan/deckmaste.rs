#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd)]
pub enum CatalogKind {
    AbilityWords,
    ArtifactTypes,
    BattleTypes,
    CardNames,
    CardTypes,
    CounterKindPhrases,
    CreatureTypes,
    EnchantmentTypes,
    KeywordAbilities,
    KeywordActions,
    LandTypes,
    PlaneswalkerTypes,
    SpellTypes,
    Supertypes,
}

impl CatalogKind {
    pub const ALL: [Self; 14] = [
        Self::AbilityWords,
        Self::ArtifactTypes,
        Self::BattleTypes,
        Self::CardNames,
        Self::CardTypes,
        Self::CounterKindPhrases,
        Self::CreatureTypes,
        Self::EnchantmentTypes,
        Self::KeywordAbilities,
        Self::KeywordActions,
        Self::LandTypes,
        Self::PlaneswalkerTypes,
        Self::SpellTypes,
        Self::Supertypes,
    ];

    #[must_use]
    pub const fn filename(self) -> &'static str {
        match self {
            Self::AbilityWords => "ability-words.txt",
            Self::ArtifactTypes => "artifact-types.txt",
            Self::BattleTypes => "battle-types.txt",
            Self::CardNames => "card-names.txt",
            Self::CardTypes => "card-types.txt",
            Self::CounterKindPhrases => "counter-kind-phrases.txt",
            Self::CreatureTypes => "creature-types.txt",
            Self::EnchantmentTypes => "enchantment-types.txt",
            Self::KeywordAbilities => "keyword-abilities.txt",
            Self::KeywordActions => "keyword-actions.txt",
            Self::LandTypes => "land-types.txt",
            Self::PlaneswalkerTypes => "planeswalker-types.txt",
            Self::SpellTypes => "spell-types.txt",
            Self::Supertypes => "supertypes.txt",
        }
    }
}
