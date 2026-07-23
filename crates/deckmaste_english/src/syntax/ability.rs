use super::clause::DependentClause;
use super::clause::IndependentClause;
use super::phrase::NounPhrase;
use super::phrase::Phrase;
use super::phrase::RecoveredText;
use crate::catalog::CatalogAtom;

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct OracleText {
    pub abilities: Vec<Ability>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Ability {
    pub ability_word: Option<CatalogAtom>,
    pub kind: AbilityKind,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AbilityKind {
    Activated(ActivatedAbility),
    ClassLevel(ClassLevelAbility),
    Triggered(TriggeredAbility),
    Loyalty(LoyaltyAbility),
    Modal(ModalAbility),
    Keyword(KeywordAbilityList),
    Paragraph(Paragraph),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ClassLevelAbility {
    pub cost: Cost,
    pub level: super::phrase::NumberLiteral,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ActivatedAbility {
    pub cost: Cost,
    pub effect: Paragraph,
    pub effect_initial_uppercase: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Cost {
    Components(Vec<Phrase>),
    SymbolList(String),
}

impl Default for Cost {
    fn default() -> Self {
        Self::Components(Vec::new())
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TriggeredAbility {
    pub introducer: TriggerWord,
    pub event: TriggerEvent,
    pub intervening_condition: Option<DependentClause>,
    pub effect: Paragraph,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TriggerEvent {
    Clause(IndependentClause),
    Temporal(NounPhrase),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TriggerWord {
    When,
    Whenever,
    At,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LoyaltyAbility {
    pub cost: LoyaltyCost,
    pub effect: Paragraph,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LoyaltyCost {
    pub sign: LoyaltyCostSign,
    pub value: LoyaltyCostValue,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LoyaltyCostSign {
    None,
    Plus,
    Minus,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LoyaltyCostValue {
    Number(u32),
    X,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ModalAbility {
    pub frame: ModalFrame,
    pub header: Paragraph,
    pub header_suffix: ModalHeaderSuffix,
    pub modes: Vec<Mode>,
}

#[allow(
    clippy::large_enum_variant,
    reason = "larger enum shapes are part of the serialized card-ability model"
)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ModalFrame {
    Unframed,
    Preamble {
        body: Paragraph,
        separator: ModalPreambleSeparator,
    },
    Activated(Cost),
    Triggered {
        introducer: TriggerWord,
        event: TriggerEvent,
        intervening_condition: Option<DependentClause>,
    },
    Loyalty(LoyaltyCost),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ModalHeaderSuffix {
    None,
    SpacedEmDash,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ModalPreambleSeparator {
    None,
    Space,
    CommaSpace,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Mode {
    pub body: Paragraph,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct KeywordAbilityList {
    pub abilities: Vec<KeywordAbility>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct KeywordAbility {
    pub preceding_separator: Option<KeywordListSeparator>,
    pub ability: CatalogAtom,
    pub argument_separator: Option<KeywordArgumentSeparator>,
    pub argument: Option<Phrase>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum KeywordListSeparator {
    Comma,
    Semicolon,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum KeywordArgumentSeparator {
    Space,
    EmDash,
    SpacedEmDash,
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Paragraph {
    /// Semantically inert flavor text carried before an em dash in header
    /// position (ability start or saga chapter body). Licensed lexical opacity:
    /// the surface is preserved verbatim and reproduced with its em-dash
    /// separator, never parsed structurally.
    pub flavor_header: Option<FlavorHeader>,
    pub sentences: Vec<Sentence>,
}

/// A flavor junk-before-dash header: an arbitrary token run reproduced
/// verbatim.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FlavorHeader {
    text: String,
    source_tokens: usize,
}

impl FlavorHeader {
    #[must_use]
    pub fn new(text: impl Into<String>, source_tokens: usize) -> Self {
        Self {
            text: text.into(),
            source_tokens,
        }
    }

    #[must_use]
    pub fn text(&self) -> &str {
        &self.text
    }

    #[must_use]
    pub const fn source_tokens(&self) -> usize {
        self.source_tokens
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Sentence {
    pub initial_uppercase: bool,
    pub body: SentenceBody,
    pub ending: SentenceEnding,
}

#[allow(
    clippy::large_enum_variant,
    reason = "sentence bodies can legitimately hold a large independent clause payload"
)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SentenceBody {
    Independent(IndependentClause),
    Recovered(RecoveredText),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SentenceEnding {
    None,
    Period,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct QuotedAbility {
    pub ability: Box<Ability>,
    pub initial_uppercase: bool,
    pub closed: bool,
}
