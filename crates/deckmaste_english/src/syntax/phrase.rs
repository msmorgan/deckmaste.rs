use std::sync::Arc;

use super::ability::QuotedAbility;
use super::clause::InfinitiveClause;
use super::clause::RelativeClause;
use crate::Numeral;
use crate::catalog::CatalogAtom;
use crate::word::Adjective;
use crate::word::ColorWord;
use crate::word::NounInstance;
use crate::word::Pronoun;
use crate::word::PronounCase;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UnknownPhrase(pub String);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ThisCardForm {
    AbbreviatedName,
    FullName,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OracleSymbol(Arc<str>);

impl OracleSymbol {
    #[must_use]
    pub fn new(symbol: impl Into<Arc<str>>) -> Option<Self> {
        let symbol = symbol.into();
        let body = symbol.strip_prefix('{')?.strip_suffix('}')?;
        (!body.is_empty()
            && body
                .chars()
                .all(|character| !character.is_whitespace() && !matches!(character, '{' | '}')))
        .then_some(Self(symbol))
    }

    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct NumberLiteral {
    pub value: i32,
    pub numeral: Numeral,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ScalarSign {
    None,
    Plus,
    Minus,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ScalarValue {
    Integer(u32),
    X,
    Star,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SignedScalar {
    pub sign: ScalarSign,
    pub value: ScalarValue,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PowerToughness {
    pub power: SignedScalar,
    pub toughness: SignedScalar,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Quantity {
    Exact(NumberLiteral),
    UpTo(NumberLiteral),
    ThatMany,
    ThatMuch,
}

impl Quantity {
    #[must_use]
    pub const fn noun_cardinality(self) -> NounCardinality {
        match self {
            Self::Exact(number) | Self::UpTo(number) if number.value == 1 => {
                NounCardinality::SingularCount
            }
            Self::Exact(_) | Self::UpTo(_) | Self::ThatMany => NounCardinality::PluralCount,
            Self::ThatMuch => NounCardinality::Mass,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Determiner {
    The,
    Each,
    Indefinite(IndefiniteArticle),
    Demonstrative(Demonstrative),
    Target(Option<Quantity>),
    Possessive(Possessor),
    All,
}

impl Determiner {
    #[must_use]
    pub const fn noun_cardinality(&self) -> NounCardinality {
        match self {
            Self::Each
            | Self::Indefinite(_)
            | Self::Target(None)
            | Self::Demonstrative(Demonstrative::This | Demonstrative::That) => {
                NounCardinality::SingularCount
            }
            Self::Demonstrative(Demonstrative::These | Demonstrative::Those) => {
                NounCardinality::PluralCount
            }
            Self::Target(Some(quantity)) => quantity.noun_cardinality(),
            Self::All => NounCardinality::PluralOrMass,
            Self::The | Self::Possessive(_) => NounCardinality::Unconstrained,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IndefiniteArticle {
    A,
    An,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Demonstrative {
    This,
    That,
    These,
    Those,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Possessor {
    Pronoun(Pronoun),
    NounPhrase(Box<NounPhrase>),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NounCardinality {
    SingularCount,
    PluralCount,
    Mass,
    PluralOrMass,
    Unconstrained,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum NounPhrase {
    Nominal(NominalPhrase),
    Pronoun { pronoun: Pronoun, case: PronounCase },
    ThisCard(ThisCardForm),
    Coordinated(CoordinatedNounPhrase),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CoordinatedNounPhrase {
    pub first: Box<NounPhrase>,
    pub rest: Vec<NounPhraseCoordination>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NounPhraseCoordination {
    pub conjunction: NounPhraseConjunction,
    pub phrase: NounPhrase,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NounPhraseConjunction {
    And,
    Or,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NominalPhrase {
    pub determiner: Option<Determiner>,
    pub modifiers: Vec<NominalModifier>,
    pub head: NounInstance,
    pub complements: Vec<NominalComplement>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum NominalModifier {
    Adjective(AdjectivePhrase),
    Noun(NounInstance),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AdjectivePhrase {
    pub head: Adjective,
    pub complements: Vec<AdjectiveComplement>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AdjectiveComplement {
    Prepositional(PrepositionalPhrase),
    Infinitive(InfinitiveClause),
    Unknown(UnknownPhrase),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum NominalComplement {
    Prepositional(PrepositionalPhrase),
    Relative(RelativeClause),
    Unknown(UnknownPhrase),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PrepositionalPhrase {
    pub preposition: Preposition,
    pub object: Box<Phrase>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Preposition {
    At,
    By,
    For,
    From,
    In,
    Into,
    Of,
    On,
    Onto,
    To,
    Under,
    With,
    Without,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Phrase {
    NounPhrase(Box<NounPhrase>),
    AdjectivePhrase(Box<AdjectivePhrase>),
    PrepositionalPhrase(Box<PrepositionalPhrase>),
    Quantity(Quantity),
    CatalogAtom(CatalogAtom),
    ColorWord(ColorWord),
    ThisCard(ThisCardForm),
    OracleSymbol(OracleSymbol),
    SymbolSequence(Vec<OracleSymbol>),
    NumberLiteral(NumberLiteral),
    SignedScalar(SignedScalar),
    PowerToughness(PowerToughness),
    QuotedAbility(Box<QuotedAbility>),
    UnknownPhrase(UnknownPhrase),
}
