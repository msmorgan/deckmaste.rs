use super::clause::DependentClause;
use super::clause::IndependentClause;
use super::phrase::NounPhrase;
use super::phrase::Phrase;
use super::phrase::UnknownPhrase;
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
    Triggered(TriggeredAbility),
    Loyalty(LoyaltyAbility),
    Modal(ModalAbility),
    Keyword(KeywordAbilityList),
    Paragraph(Paragraph),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ActivatedAbility {
    pub cost: Cost,
    pub effect: Paragraph,
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Cost {
    pub components: Vec<Phrase>,
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
    pub sentences: Vec<Sentence>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Sentence {
    pub body: SentenceBody,
    pub ending: SentenceEnding,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SentenceBody {
    Independent(IndependentClause),
    Unknown(UnknownPhrase),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SentenceEnding {
    None,
    Period(u8),
    Exclamation(u8),
    Question(u8),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct QuotedAbility {
    pub ability: Box<Ability>,
    pub closed: bool,
}
