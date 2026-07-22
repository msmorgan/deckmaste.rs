use std::sync::Arc;

use super::ability::Ability;
use super::ability::Cost;
use super::ability::QuotedAbility;
use super::clause::Clause;
use super::clause::InfinitiveClause;
use super::clause::RelativeClause;
use crate::Numeral;
use crate::catalog::CatalogAtom;
use crate::word::Adjective;
use crate::word::ColorWord;
use crate::word::NounInstance;
use crate::word::Pronoun;
use crate::word::PronounCase;
use crate::word::Vocab;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct UnknownPhrase(pub String);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ThisCardForm {
    AbbreviatedName,
    FullName,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct NumberLiteral {
    pub value: i32,
    pub numeral: Numeral,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ScalarSign {
    None,
    Plus,
    Minus,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ScalarValue {
    Integer(u32),
    X,
    Star,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SignedScalar {
    pub sign: ScalarSign,
    pub value: ScalarValue,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct PowerToughness {
    pub power: SignedScalar,
    pub toughness: SignedScalar,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Quantity {
    Exact(NumberLiteral),
    AtLeast(NumberLiteral),
    Or(NumberLiteral, NumberLiteral),
    UpTo(NumberLiteral),
    MoreThan(NumberLiteral),
    FewerThan(NumberLiteral),
    X,
    Both,
    ThatMany,
    ThatMuch,
}

impl Quantity {
    #[must_use]
    pub const fn noun_cardinality(self) -> NounCardinality {
        match self {
            Self::Exact(number)
            | Self::UpTo(number)
            | Self::MoreThan(number)
            | Self::FewerThan(number)
                if number.value == 1 =>
            {
                NounCardinality::SingularOrMass
            }
            Self::Or(first, second) if first.value == 1 && second.value == 1 => {
                NounCardinality::SingularOrMass
            }
            Self::Exact(_)
            | Self::Or(_, _)
            | Self::UpTo(_)
            | Self::MoreThan(_)
            | Self::FewerThan(_)
            | Self::X
            | Self::Both => NounCardinality::PluralOrMass,
            Self::AtLeast(_) | Self::ThatMany => NounCardinality::PluralCount,
            Self::ThatMuch => NounCardinality::Mass,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Determiner {
    The,
    Each,
    Another,
    Indefinite(IndefiniteArticle),
    Demonstrative(Demonstrative),
    Target(Option<Quantity>),
    Quantity(Quantity),
    Possessive(Possessor),
    All,
    Any,
    No,
}

impl Determiner {
    #[must_use]
    pub const fn noun_cardinality(&self) -> NounCardinality {
        match self {
            Self::Each
            | Self::Another
            | Self::Indefinite(_)
            | Self::Target(None)
            | Self::Demonstrative(Demonstrative::This | Demonstrative::That) => {
                NounCardinality::SingularCount
            }
            Self::Demonstrative(Demonstrative::These | Demonstrative::Those) => {
                NounCardinality::PluralCount
            }
            Self::Target(Some(
                Quantity::Exact(number)
                | Quantity::UpTo(number)
                | Quantity::MoreThan(number)
                | Quantity::FewerThan(number),
            )) if number.value == 1 => NounCardinality::SingularCount,
            Self::Target(Some(
                Quantity::Exact(_)
                | Quantity::AtLeast(_)
                | Quantity::UpTo(_)
                | Quantity::MoreThan(_)
                | Quantity::FewerThan(_)
                | Quantity::ThatMany,
            )) => NounCardinality::PluralCount,
            Self::Target(Some(Quantity::Or(_, _) | Quantity::X | Quantity::Both)) => {
                NounCardinality::PluralCount
            }
            Self::Target(Some(Quantity::ThatMuch)) => NounCardinality::Mass,
            Self::Quantity(quantity) => quantity.noun_cardinality(),
            Self::All => NounCardinality::PluralOrMass,
            Self::The | Self::Possessive(_) | Self::Any | Self::No => {
                NounCardinality::Unconstrained
            }
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IndefiniteArticle {
    A,
    An,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
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
    SingularOrMass,
    PluralCount,
    Mass,
    PluralOrMass,
    Unconstrained,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum NounPhrase {
    Nominal(NominalPhrase),
    Pronoun { pronoun: Pronoun, case: PronounCase },
    Possessive(Possessor),
    Demonstrative(Demonstrative),
    Quantity(Quantity),
    ThisCard(ThisCardForm),
    Partitive(PartitiveNounPhrase),
    Coordinated(CoordinatedNounPhrase),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PartitiveNounPhrase {
    pub quantity: Quantity,
    pub whole: Box<NounPhrase>,
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
    Quantity(Quantity),
    PowerToughness(PowerToughness),
    Unknown(UnknownPhrase),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AdjectivePhrase {
    pub head: Adjective,
    pub complements: Vec<AdjectiveComplement>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ComparisonMarker {
    Than,
    ThanOrEqualTo,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ComparisonComplement {
    pub marker: ComparisonMarker,
    pub standard: Box<Phrase>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AdjectiveComplement {
    Comparison(ComparisonComplement),
    Prepositional(PrepositionalPhrase),
    Infinitive(InfinitiveClause),
    Unknown(UnknownPhrase),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum NominalComplement {
    Adjective(AdjectivePhrase),
    Prepositional(PrepositionalPhrase),
    Relative(RelativeClause),
    Quantity(Quantity),
    Unknown(UnknownPhrase),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PrepositionalPhrase {
    pub preposition: Preposition,
    pub object: Box<Phrase>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Preposition {
    Among,
    As,
    At,
    Before,
    By,
    During,
    For,
    From,
    In,
    Into,
    Of,
    On,
    Onto,
    To,
    Until,
    Under,
    With,
    Without,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Phrase {
    Clause(Box<Clause>),
    NounPhrase(Box<NounPhrase>),
    AdjectivePhrase(Box<AdjectivePhrase>),
    PrepositionalPhrase(Box<PrepositionalPhrase>),
    Quantity(Quantity),
    Adverb(Vocab),
    CatalogAtom(CatalogAtom),
    ColorWord(ColorWord),
    Cost(Cost),
    ThisCard(ThisCardForm),
    OracleSymbol(OracleSymbol),
    SymbolSequence(Vec<OracleSymbol>),
    NumberLiteral(NumberLiteral),
    SignedScalar(SignedScalar),
    PowerToughness(PowerToughness),
    EmbeddedAbility(Box<Ability>),
    QuotedAbility(Box<QuotedAbility>),
    UnknownPhrase(UnknownPhrase),
}
