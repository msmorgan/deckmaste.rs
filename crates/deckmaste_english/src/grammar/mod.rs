#![allow(
    dead_code,
    reason = "later grammar milestones consume the staged clause and ability categories"
)]

pub(crate) mod ability;
mod clause;
mod nominal;
mod recovery;

use std::collections::HashMap;

use crate::Numeral;
use crate::Span;
use crate::catalog::CatalogSlot;
use crate::catalog::CatalogValue;
use crate::catalog::Catalogs;
use crate::chart::ChartResult;
use crate::chart::ChartStats;
use crate::chart::Child;
use crate::chart::Expected;
use crate::chart::Grammar;
use crate::chart::GrammarError;
use crate::chart::LexicalMatch;
use crate::chart::Reduction;
use crate::chart::Rule;
use crate::chart::RuleId;
use crate::chart::parse_chart;
use crate::forest::BestParse;
use crate::forest::ForestError;
use crate::forest::ForestStats;
use crate::forest::ForestSymbol;
use crate::forest::NodeId;
use crate::forest::ParseCost;
use crate::forest::ParseForest;
use crate::surface::Punctuation;
use crate::surface::Token;
use crate::surface::TokenKind;
use crate::surface::lex;
use crate::syntax::AdjectivePhrase;
use crate::syntax::Clause;
use crate::syntax::ComparisonComplement;
use crate::syntax::ComparisonMarker;
use crate::syntax::Demonstrative;
use crate::syntax::Determiner;
use crate::syntax::ExistentialForm;
use crate::syntax::FrequencyBound;
use crate::syntax::FrequencyCount;
use crate::syntax::FrequencyPhrase;
use crate::syntax::GerundClause;
use crate::syntax::IndefiniteArticle;
use crate::syntax::InfinitiveMarker;
use crate::syntax::NominalComplement;
use crate::syntax::NominalModifier;
use crate::syntax::NominalPhrase;
use crate::syntax::NounPhrase;
use crate::syntax::OracleSymbol;
use crate::syntax::Phrase;
use crate::syntax::Possessor;
use crate::syntax::PowerToughness;
use crate::syntax::Preposition;
use crate::syntax::PrepositionalPhrase;
use crate::syntax::PreverbModifier;
use crate::syntax::Quantity;
use crate::syntax::RelativeClause;
use crate::syntax::RelativeGap;
use crate::syntax::Sentence;
use crate::syntax::Subject;
use crate::syntax::ThisCardForm;
use crate::syntax::UnknownPhrase;
use crate::syntax::VerbParticle;
use crate::word::Adjective;
use crate::word::Auxiliary;
use crate::word::AuxiliaryInflection;
use crate::word::AuxiliaryInstance;
use crate::word::CardOrientation;
use crate::word::InitialSound;
use crate::word::LexicalSlot;
use crate::word::Noun;
use crate::word::NounInstance;
use crate::word::NounUsage;
use crate::word::Number;
use crate::word::Person;
use crate::word::Pronoun;
use crate::word::PronounCase;
use crate::word::PronounInstance;
use crate::word::VerbInstance;
use crate::word::VerbSlot;
use crate::word::Vocab;
use crate::word::Vocabulary;
use crate::word::WordMatch;

#[derive(Debug, Clone, PartialEq, Eq)]
struct VerbPhrase {
    auxiliaries: Vec<AuxiliaryInstance>,
    first_auxiliary_contracted_with_subject: bool,
    preverb_modifiers: Vec<PreverbModifier>,
    verb: VerbInstance,
    dependents: Vec<VerbDependent>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum VerbDependent {
    DirectObject(NounPhrase),
    IndirectObject(NounPhrase),
    PredicateComplement(Phrase),
    Scalar(Phrase),
    Statistic(Phrase),
    Prepositional(PrepositionalPhrase),
    Temporal(NounPhrase),
    Infinitive(InfinitiveClause),
    Subordinate(Box<Clause>),
    Adverbial(Phrase),
    Frequency(FrequencyPhrase),
    Particle(VerbParticle),
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct InfinitiveClause {
    negated: bool,
    marker: InfinitiveMarker,
    predicate: Box<VerbPhrase>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct SimpleClause {
    subject: Option<Subject>,
    predicate: VerbPhrase,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct ContractedSubjectAuxiliary {
    subject: Subject,
    auxiliary: AuxiliaryInstance,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub(crate) enum ContractedSubjectKey {
    Pronoun(Pronoun),
    Demonstrative(Demonstrative),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub(crate) enum Nonterminal {
    Quantity,
    Determiner,
    Adjective,
    AdjectivePhrase,
    ComparisonStandard,
    ComparisonComplement,
    Noun,
    Nominal,
    NounPhrase,
    FrequencyPhrase,
    PossessiveNounPhrase,
    PrepositionalPhrase,
    PrepositionalObject,
    RelativeClause,
    Verb,
    VerbPhrase,
    ObjectGapVerbPhrase,
    InfinitiveClause,
    GerundClause,
    SimpleClause,
    Clause,
    Sentence,
    Paragraph,
    Cost,
    KeywordAbility,
    KeywordAbilityList,
    ActivatedAbility,
    TriggeredAbility,
    LoyaltyAbility,
    ModalAbility,
    Ability,
    OracleText,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub(crate) enum EnglishLexicalSlot {
    Number(NumberNotation),
    Noun(NounUsage),
    PossessiveNoun,
    Verb(VerbSlot),
    Adjective,
    Adverb,
    VerbParticle(VerbParticle),
    Frequency,
    Pronoun(PronounCase),
    Auxiliary,
    AbilityItem,
    AbilityWord,
    Determiner,
    DeterminerTarget,
    QuantityAtLeast,
    QuantityOr,
    QuantityX,
    QuantityBoth,
    QuantityThatMany,
    QuantityThatMuch,
    QuantityBound(BoundedQuantityKind),
    Than,
    OrEqualTo,
    RelativeWho,
    Face,
    Up,
    Down,
    Not,
    To,
    Of,
    Reciprocal,
    ThisCard,
    FullThisCard,
    PossessiveThisCard,
    Preposition,
    OracleSymbol,
    PowerToughness,
    Punctuation(Punctuation),
    Subordinator,
    RatherThan,
    Conjunction,
    Plus,
    Existential,
    Copula,
    SubjectAuxiliary,
    Unknown(RecoverySlot),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub(crate) enum RecoveryMode {
    Exact,
    UnknownPhrases,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub(crate) enum BoundedQuantityKind {
    MoreThan,
    FewerThan,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum RecoveryProfile {
    Exact,
    Phrases,
}

const RECOVERY_STATE_LIMIT: usize = 50_000;

impl RecoveryProfile {
    const fn mode(self) -> RecoveryMode {
        match self {
            Self::Exact => RecoveryMode::Exact,
            Self::Phrases => RecoveryMode::UnknownPhrases,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub(crate) enum RecoverySlot {
    Noun(NounForm),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub(crate) enum NumberNotation {
    Cardinal,
    Ordinal,
    Arabic(bool),
    Roman,
}

impl NumberNotation {
    const fn numeral(self) -> Numeral {
        match self {
            Self::Cardinal => Numeral::Cardinal,
            Self::Ordinal => Numeral::Ordinal,
            Self::Arabic(commas) => Numeral::Arabic(commas),
            Self::Roman => Numeral::Roman,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub(crate) enum NounForm {
    Singular,
    Plural,
    Mass,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub(crate) enum Cardinality {
    SingularCount,
    SingularOrMass,
    PluralCount,
    Mass,
    PluralOrMass,
    Unconstrained,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub(crate) struct Agreement {
    person: Person,
    number: Number,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub(crate) enum PredicateForm {
    Imperative,
    Infinitive,
    Finite(Option<Agreement>),
    PresentParticiple,
    PastParticiple,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub(crate) enum PredicateObjectState {
    None,
    Direct,
    Ability,
    AbilityWithArgument,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub(crate) enum PredicateAttachmentPhase {
    Object,
    Tail,
    PrepositionalTail,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub(crate) enum NominalAttachmentPhase {
    Open,
    Prepositional,
    PostpositiveAdjective,
    Comparison,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub(crate) enum AdjectiveComparisonState {
    NotComparative,
    Pending,
    Complete,
}

impl PredicateObjectState {
    const fn has_direct_object(self) -> bool {
        !matches!(self, Self::None)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub(crate) enum Features {
    None,
    Number {
        is_one: bool,
    },
    Quantity(Cardinality),
    Determiner {
        cardinality: Cardinality,
        article: Option<IndefiniteArticleKey>,
    },
    Adjective {
        initial_sound: InitialSound,
        comparison: AdjectiveComparisonState,
        card_orientation: bool,
    },
    Noun {
        form: NounForm,
        initial_sound: InitialSound,
        temporal: bool,
    },
    Nominal {
        form: NounForm,
        initial_sound: InitialSound,
        determined: bool,
        leading_recovery: bool,
        attachment: NominalAttachmentPhase,
        comparison: AdjectiveComparisonState,
        temporal: bool,
    },
    NounPhrase {
        agreement: Option<Agreement>,
        pronoun_case: Option<PronounCase>,
        temporal: bool,
    },
    PossessiveThisCard {
        agreement: Agreement,
    },
    PossessiveNounPhrase {
        form: NounForm,
        initial_sound: InitialSound,
        determined: bool,
    },
    Verb {
        slot: VerbSlot,
        accepts_direct_object: bool,
        requires_direct_object: bool,
        proform: bool,
    },
    VerbPhrase {
        form: PredicateForm,
        object: PredicateObjectState,
        phase: PredicateAttachmentPhase,
        accepts_direct_object: bool,
        requires_direct_object: bool,
        proform: bool,
    },
    InfinitiveClause,
    GerundClause,
    SimpleClause {
        agreement: Option<Agreement>,
        has_subject: bool,
        standalone: bool,
        has_direct_object: bool,
    },
    Clause {
        agreement: Option<Agreement>,
        standalone: bool,
        finite: bool,
    },
    Sentence,
    Preposition(Preposition),
    PrepositionalObject {
        gerund: bool,
    },
    PrepositionalPhrase {
        nominal_attachment: bool,
    },
    RelativeClause {
        gap: RelativeGap,
        antecedent_agreement: Option<Agreement>,
    },
    Auxiliary(AuxiliaryInstance),
    Conjunction(crate::syntax::PredicateConjunction),
    Existential {
        number: Number,
    },
    Copula(Agreement),
    SubjectAuxiliary {
        subject: ContractedSubjectKey,
        agreement: Agreement,
        auxiliary: AuxiliaryInstance,
    },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub(crate) struct NumberKey {
    value: i32,
    notation: NumberNotation,
}

impl NumberKey {
    const fn literal(self) -> crate::syntax::NumberLiteral {
        crate::syntax::NumberLiteral {
            value: self.value,
            numeral: self.notation.numeral(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub(crate) enum QuantityKey {
    Exact(NumberKey),
    AtLeast(NumberKey),
    OrMore(NumberKey),
    Or(NumberKey, NumberKey),
    UpTo(NumberKey),
    MoreThan(NumberKey),
    FewerThan(NumberKey),
    X,
    Both,
    ThatMany,
    ThatMuch,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub(crate) struct FrequencyKey {
    bound: FrequencyBound,
    count: FrequencyCountKey,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub(crate) enum FrequencyCountKey {
    Once,
    Twice,
    Times(NumberKey),
}

impl FrequencyKey {
    const fn syntax(self) -> FrequencyPhrase {
        FrequencyPhrase {
            bound: self.bound,
            count: match self.count {
                FrequencyCountKey::Once => FrequencyCount::Once,
                FrequencyCountKey::Twice => FrequencyCount::Twice,
                FrequencyCountKey::Times(number) => FrequencyCount::Times(number.literal()),
            },
        }
    }
}

impl QuantityKey {
    const fn cardinality(self) -> Cardinality {
        match self {
            Self::Exact(number)
            | Self::UpTo(number)
            | Self::MoreThan(number)
            | Self::FewerThan(number)
                if number.value == 1 =>
            {
                Cardinality::SingularOrMass
            }
            Self::Or(first, second) if first.value == 1 && second.value == 1 => {
                Cardinality::SingularOrMass
            }
            Self::Exact(_)
            | Self::Or(_, _)
            | Self::UpTo(_)
            | Self::MoreThan(_)
            | Self::FewerThan(_)
            | Self::X
            | Self::Both => Cardinality::PluralOrMass,
            Self::AtLeast(_) | Self::OrMore(_) | Self::ThatMany => Cardinality::PluralCount,
            Self::ThatMuch => Cardinality::Mass,
        }
    }

    const fn syntax(self) -> Quantity {
        match self {
            Self::Exact(number) => Quantity::Exact(number.literal()),
            Self::AtLeast(number) => Quantity::AtLeast(number.literal()),
            Self::OrMore(number) => Quantity::OrMore(number.literal()),
            Self::Or(first, second) => Quantity::Or(first.literal(), second.literal()),
            Self::UpTo(number) => Quantity::UpTo(number.literal()),
            Self::MoreThan(number) => Quantity::MoreThan(number.literal()),
            Self::FewerThan(number) => Quantity::FewerThan(number.literal()),
            Self::X => Quantity::X,
            Self::Both => Quantity::Both,
            Self::ThatMany => Quantity::ThatMany,
            Self::ThatMuch => Quantity::ThatMuch,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub(crate) enum DeterminerKey {
    The,
    Each,
    Another,
    Indefinite(IndefiniteArticleKey),
    Demonstrative(DemonstrativeKey),
    Target(Option<QuantityKey>),
    Quantity(QuantityKey),
    Possessive(Pronoun),
    All,
    Any,
    No,
}

impl DeterminerKey {
    const fn cardinality(&self) -> Cardinality {
        match self {
            Self::Each
            | Self::Another
            | Self::Indefinite(_)
            | Self::Target(None)
            | Self::Demonstrative(DemonstrativeKey::This | DemonstrativeKey::That) => {
                Cardinality::SingularCount
            }
            Self::Demonstrative(DemonstrativeKey::These | DemonstrativeKey::Those) => {
                Cardinality::PluralCount
            }
            Self::Target(Some(
                QuantityKey::Exact(number)
                | QuantityKey::UpTo(number)
                | QuantityKey::MoreThan(number)
                | QuantityKey::FewerThan(number),
            )) if number.value == 1 => Cardinality::SingularCount,
            Self::Target(Some(
                QuantityKey::Exact(_)
                | QuantityKey::AtLeast(_)
                | QuantityKey::OrMore(_)
                | QuantityKey::Or(_, _)
                | QuantityKey::UpTo(_)
                | QuantityKey::MoreThan(_)
                | QuantityKey::FewerThan(_)
                | QuantityKey::X
                | QuantityKey::Both
                | QuantityKey::ThatMany,
            )) => Cardinality::PluralCount,
            Self::Target(Some(QuantityKey::ThatMuch)) => Cardinality::Mass,
            Self::Quantity(quantity) => quantity.cardinality(),
            Self::All => Cardinality::PluralOrMass,
            Self::The | Self::Possessive(_) | Self::Any | Self::No => Cardinality::Unconstrained,
        }
    }

    fn syntax(&self) -> Option<Determiner> {
        Some(match self {
            Self::The => Determiner::The,
            Self::Each => Determiner::Each,
            Self::Another => Determiner::Another,
            Self::Indefinite(IndefiniteArticleKey::A) => {
                Determiner::Indefinite(IndefiniteArticle::A)
            }
            Self::Indefinite(IndefiniteArticleKey::An) => {
                Determiner::Indefinite(IndefiniteArticle::An)
            }
            Self::Demonstrative(DemonstrativeKey::This) => {
                Determiner::Demonstrative(Demonstrative::This)
            }
            Self::Demonstrative(DemonstrativeKey::That) => {
                Determiner::Demonstrative(Demonstrative::That)
            }
            Self::Demonstrative(DemonstrativeKey::These) => {
                Determiner::Demonstrative(Demonstrative::These)
            }
            Self::Demonstrative(DemonstrativeKey::Those) => {
                Determiner::Demonstrative(Demonstrative::Those)
            }
            Self::Target(quantity) => Determiner::Target(quantity.map(QuantityKey::syntax)),
            Self::Quantity(quantity) => Determiner::Quantity(quantity.syntax()),
            Self::Possessive(pronoun) => Determiner::Possessive(Possessor::Pronoun(*pronoun)),
            Self::All => Determiner::All,
            Self::Any => Determiner::Any,
            Self::No => Determiner::No,
        })
    }

    const fn article(&self) -> Option<IndefiniteArticleKey> {
        match self {
            Self::Indefinite(article) => Some(*article),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub(crate) enum IndefiniteArticleKey {
    A,
    An,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub(crate) enum DemonstrativeKey {
    This,
    That,
    These,
    Those,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub(crate) enum LiteralKey {
    Face,
    Up,
    Down,
    Not,
    To,
    Target,
    Than,
    OrEqualTo,
    Who,
    Plus,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub(crate) enum MeaningKey {
    Literal(LiteralKey),
    Number(NumberKey),
    Quantity(QuantityKey),
    Determiner(DeterminerKey),
    Noun(NounInstance),
    Adjective(Adjective),
    Adverb(Vocab),
    VerbParticle(VerbParticle),
    Frequency(FrequencyKey),
    Pronoun(PronounInstance),
    Auxiliary(AuxiliaryInstance),
    Verb(VerbInstance),
    Catalog(crate::catalog::CatalogAtom),
    OracleSymbol(OracleSymbol),
    PowerToughness(PowerToughness),
    Punctuation(Punctuation),
    Conjunction(crate::syntax::PredicateConjunction),
    Subordinator(crate::syntax::Subordinator),
    Existential(ExistentialForm),
    SubjectAuxiliary(SubjectAuxiliaryKey),
    ThisCard(ThisCardForm),
    Preposition(Preposition),
    Unknown(UnknownKey),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub(crate) struct UnknownKey {
    slot: RecoverySlot,
    span: Span,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub(crate) struct SubjectAuxiliaryKey {
    subject: ContractedSubjectKey,
    auxiliary: AuxiliaryInstance,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum RuleTag {
    QuantityExact,
    QuantityAtLeast,
    QuantityOr,
    QuantityX,
    QuantityBoth,
    QuantityUpTo,
    QuantityThatMany,
    QuantityThatMuch,
    QuantityMoreThan,
    QuantityFewerThan,
    DeterminerClosed,
    DeterminerTarget,
    DeterminerQuantifiedTarget,
    DeterminerQuantity,
    DeterminerPossessiveThisCard,
    PossessiveNounBase,
    PossessiveNounDetermined,
    DeterminerPossessiveNoun,
    Adjective,
    AdjectivePhrase,
    AdjectivePhraseFaceUp,
    AdjectivePhraseFaceDown,
    AdjectivePhraseComparison,
    ComparisonStandard,
    ComparisonThan,
    ComparisonThanOrEqualTo,
    Noun,
    NominalNoun,
    NominalAdjective,
    NominalNounModifier,
    NominalQuantityModifier,
    NominalPowerToughnessModifier,
    NominalDeterminer,
    NominalPrepositional,
    NominalQuantityComplement,
    NominalRelative,
    NominalPostpositiveAdjective,
    NominalComparison,
    NounPhraseNominal,
    NounPhraseSubjectPronoun,
    NounPhraseObjectPronoun,
    NounPhraseReciprocal,
    NounPhraseQuantity,
    NounPhraseThisCard,
    NounPhraseFullThisCard,
    NounPhrasePossessiveThisCard,
    NounPhrasePartitive,
    NounPhraseCoordination,
    NounPhraseAdditiveCoordination,
    PrepositionalPhrase,
    PrepositionalObject,
    Verb,
    VerbPhraseBase,
    VerbPhraseAuxiliary,
    VerbPhraseDirectObject,
    VerbPhraseAdjective,
    VerbPhrasePrepositional,
    VerbPhraseInfinitive,
    VerbPhraseAdverb,
    VerbPhraseParticle,
    VerbPhraseFrequency,
    VerbPhraseAbility,
    VerbPhraseOracleSymbol,
    VerbPhrasePowerToughness,
    VerbPhraseQuantity,
    InfinitiveTo,
    InfinitiveNotTo,
    GerundClauseBase,
    GerundClauseSubordinateAfter,
    SimpleClauseSubject,
    SimpleClauseContractedSubject,
    SimpleClauseSubjectless,
    ClauseSimple,
    ClauseElliptical,
    ClauseCoordination,
    ClauseCoordinationComma,
    ClauseSubordinateBefore,
    ClauseSubordinateAfterElliptical,
    ClauseSubordinateAfter,
    ClauseSubordinateAfterInfinitive,
    ClauseExistential,
    ClauseCopularNoun,
    ClauseCopularAdjective,
    ClauseCopularPrepositional,
    ClauseContractedCopularNoun,
    ClauseContractedCopularAdjective,
    ClauseContractedCopularPrepositional,
    RelativeObject,
    RelativeObjectContractedSubject,
    RelativeSubjectContractedAuxiliary,
    RelativeSubject,
    RelativeContractedCopularNoun,
    RelativeContractedCopularAdjective,
    RelativeContractedCopularPrepositional,
    SentencePeriod,
    SentenceExclamation,
    SentenceQuestion,
    SentenceNone,
    NounUnknown,
    FrequencyPhrase,
}

pub(crate) struct EnglishGrammar<'source, 'catalogs> {
    source: &'source str,
    catalogs: &'catalogs Catalogs,
    start: Nonterminal,
    rules: Vec<Rule<Nonterminal, EnglishLexicalSlot>>,
    tags: Vec<RuleTag>,
    rules_by_lhs: HashMap<Nonterminal, Vec<RuleId>>,
    recovery_profile: RecoveryProfile,
}

impl<'source, 'catalogs> EnglishGrammar<'source, 'catalogs> {
    pub(crate) fn new(
        source: &'source str,
        catalogs: &'catalogs Catalogs,
        start: Nonterminal,
    ) -> Self {
        Self::with_recovery_profile(source, catalogs, start, RecoveryProfile::Exact)
    }

    fn with_recovery_profile(
        source: &'source str,
        catalogs: &'catalogs Catalogs,
        start: Nonterminal,
        recovery_profile: RecoveryProfile,
    ) -> Self {
        let mut builder = RuleBuilder::default();
        builder.add_nominal_rules();
        builder.add_clause_rules();
        if recovery_profile != RecoveryProfile::Exact {
            recovery::add_rules(&mut builder);
        }
        Self {
            source,
            catalogs,
            start,
            rules: builder.rules,
            tags: builder.tags,
            rules_by_lhs: builder.rules_by_lhs,
            recovery_profile,
        }
    }

    fn token_text<'tokens>(&self, tokens: &'tokens [Token], start: usize) -> Option<&'source str> {
        tokens.get(start)?.span.text(self.source)
    }

    fn one_token_match(&self, tokens: &[Token], start: usize, expected: &str) -> Option<usize> {
        self.token_text(tokens, start)?
            .eq_ignore_ascii_case(expected)
            .then_some(start + 1)
    }

    fn words_match(&self, tokens: &[Token], start: usize, expected: &[&str]) -> Option<usize> {
        let end = start.checked_add(expected.len())?;
        let actual = tokens.get(start..end)?;
        actual
            .iter()
            .zip(expected)
            .all(|(token, expected)| {
                token
                    .span
                    .text(self.source)
                    .is_some_and(|surface| surface.eq_ignore_ascii_case(expected))
            })
            .then_some(end)
    }

    fn catalog_end(tokens: &[Token], start: usize, byte_length: usize) -> Option<usize> {
        let byte_start = tokens.get(start)?.span.start;
        let byte_end = byte_start.checked_add(byte_length)?;
        tokens[start..]
            .iter()
            .position(|token| token.span.end == byte_end)
            .map(|offset| start + offset + 1)
    }

    fn word_matches(
        &self,
        tokens: &[Token],
        start: usize,
        slot: LexicalSlot,
    ) -> Vec<LexicalMatch<Features, MeaningKey>> {
        let Some(token) = tokens.get(start) else {
            return Vec::new();
        };
        if token.kind != TokenKind::Word {
            return Vec::new();
        }
        let Some(surface) = token.span.text(self.source) else {
            return Vec::new();
        };
        if matches!(slot, LexicalSlot::Noun(_) | LexicalSlot::Adjective)
            && !self.is_sentence_initial(tokens, start)
            && surface
                .as_bytes()
                .first()
                .is_some_and(u8::is_ascii_uppercase)
        {
            return Vec::new();
        }
        Vocabulary::new()
            .matches(surface, slot)
            .into_iter()
            .filter_map(|word| lexical_word_match(word, start + 1))
            .collect()
    }

    fn catalog_matches(
        &self,
        tokens: &[Token],
        start: usize,
        slot: CatalogSlot,
    ) -> Vec<LexicalMatch<Features, MeaningKey>> {
        let Some(token) = tokens.get(start) else {
            return Vec::new();
        };
        let Some(suffix) = self.source.get(token.span.start..) else {
            return Vec::new();
        };
        let mut catalog_matches = self.catalogs.matches(suffix, slot);
        if self.is_sentence_initial(tokens, start)
            && suffix
                .as_bytes()
                .first()
                .is_some_and(u8::is_ascii_uppercase)
        {
            let mut lowercased = suffix.to_owned();
            lowercased.make_ascii_lowercase();
            catalog_matches.extend(self.catalogs.matches(&lowercased, slot));
        }
        catalog_matches
            .into_iter()
            .filter_map(|catalog_match| {
                let end = Self::catalog_end(tokens, start, catalog_match.length)?;
                match catalog_match.value {
                    CatalogValue::Word(word) => lexical_word_match(word, end),
                    CatalogValue::Atom(atom) => Some(LexicalMatch {
                        end,
                        features: Features::None,
                        meaning: MeaningKey::Catalog(atom),
                        local_cost: ParseCost::default(),
                    }),
                }
            })
            .fold(Vec::new(), |mut matches, candidate| {
                if !matches.contains(&candidate) {
                    matches.push(candidate);
                }
                matches
            })
    }

    fn is_sentence_initial(&self, tokens: &[Token], start: usize) -> bool {
        start == 0
            || tokens.get(start.wrapping_sub(1)).is_some_and(|token| {
                matches!(
                    token.kind,
                    TokenKind::Newline
                        | TokenKind::Bullet
                        | TokenKind::Punctuation(
                            Punctuation::Period | Punctuation::Exclamation | Punctuation::Question
                        )
                )
            })
    }
}

impl Grammar for EnglishGrammar<'_, '_> {
    type Nonterminal = Nonterminal;
    type LexicalSlot = EnglishLexicalSlot;
    type Token = Token;
    type Features = Features;
    type Meaning = MeaningKey;

    fn start(&self) -> Self::Nonterminal {
        self.start
    }

    fn rules(&self) -> &[Rule<Self::Nonterminal, Self::LexicalSlot>] {
        &self.rules
    }

    fn rules_for(&self, lhs: Self::Nonterminal) -> &[RuleId] {
        self.rules_by_lhs.get(&lhs).map_or(&[], Vec::as_slice)
    }

    fn scan(
        &self,
        slot: Self::LexicalSlot,
        tokens: &[Self::Token],
        start: usize,
    ) -> Vec<LexicalMatch<Self::Features, Self::Meaning>> {
        match slot {
            EnglishLexicalSlot::Number(notation) => {
                let Some(surface) = self.token_text(tokens, start) else {
                    return Vec::new();
                };
                notation
                    .numeral()
                    .parse(surface)
                    .ok()
                    .map(|value| LexicalMatch {
                        end: start + 1,
                        features: Features::Number { is_one: value == 1 },
                        meaning: MeaningKey::Number(NumberKey { value, notation }),
                        local_cost: ParseCost {
                            precedence: u32::from(
                                notation == NumberNotation::Roman && surface == "X",
                            ),
                            ..ParseCost::default()
                        },
                    })
                    .into_iter()
                    .collect()
            }
            EnglishLexicalSlot::Noun(usage) => {
                let mut matches = self.word_matches(tokens, start, LexicalSlot::Noun(usage));
                matches.extend(self.catalog_matches(tokens, start, CatalogSlot::Noun(usage)));
                if usage != NounUsage::Mass
                    && let Some(surface) = self.token_text(tokens, start)
                    && let Some(sides) = surface.strip_prefix('d')
                    && let Ok(value) = Numeral::Arabic(false).parse(sides)
                    && value > 0
                    && let Some(candidate) = lexical_word_match(
                        WordMatch::Noun(NounInstance::Singular(Noun::Die(
                            crate::syntax::NumberLiteral {
                                value,
                                numeral: Numeral::Arabic(false),
                            },
                        ))),
                        start + 1,
                    )
                {
                    matches.push(candidate);
                }
                matches
            }
            EnglishLexicalSlot::Verb(verb_slot) => {
                let mut matches = self.word_matches(tokens, start, LexicalSlot::Verb(verb_slot));
                matches.extend(self.catalog_matches(tokens, start, CatalogSlot::Verb(verb_slot)));
                matches
            }
            EnglishLexicalSlot::Adjective => {
                let mut matches = self.word_matches(tokens, start, LexicalSlot::Adjective);
                matches.extend(self.catalog_matches(tokens, start, CatalogSlot::Adjective));
                matches
            }
            EnglishLexicalSlot::Adverb => self.word_matches(tokens, start, LexicalSlot::Adverb),
            EnglishLexicalSlot::VerbParticle(particle) => self
                .one_token_match(
                    tokens,
                    start,
                    match particle {
                        VerbParticle::In => "in",
                        VerbParticle::Out => "out",
                    },
                )
                .map(|end| LexicalMatch {
                    end,
                    features: Features::None,
                    meaning: MeaningKey::VerbParticle(particle),
                    local_cost: ParseCost::default(),
                })
                .into_iter()
                .collect(),
            EnglishLexicalSlot::Frequency => self.scan_frequency(tokens, start),
            EnglishLexicalSlot::Pronoun(case) => {
                self.word_matches(tokens, start, LexicalSlot::Pronoun(case))
            }
            EnglishLexicalSlot::Auxiliary => {
                self.word_matches(tokens, start, LexicalSlot::Auxiliary)
            }
            EnglishLexicalSlot::Copula => self
                .word_matches(tokens, start, LexicalSlot::Auxiliary)
                .into_iter()
                .filter_map(|mut candidate| {
                    let MeaningKey::Auxiliary(auxiliary) = candidate.meaning else {
                        return None;
                    };
                    let agreement = copula_agreement(auxiliary)?;
                    candidate.meaning = MeaningKey::Auxiliary(auxiliary);
                    candidate.features = Features::Copula(agreement);
                    Some(candidate)
                })
                .collect(),
            EnglishLexicalSlot::SubjectAuxiliary => self.scan_subject_auxiliary(tokens, start),
            EnglishLexicalSlot::AbilityItem => {
                self.catalog_matches(tokens, start, CatalogSlot::AbilityItem)
            }
            EnglishLexicalSlot::AbilityWord => {
                self.catalog_matches(tokens, start, CatalogSlot::AbilityWord)
            }
            EnglishLexicalSlot::Determiner => self.scan_determiner(tokens, start),
            EnglishLexicalSlot::DeterminerTarget => self
                .one_token_match(tokens, start, "target")
                .map(|end| LexicalMatch {
                    end,
                    features: Features::Determiner {
                        cardinality: Cardinality::SingularCount,
                        article: None,
                    },
                    meaning: MeaningKey::Literal(LiteralKey::Target),
                    local_cost: ParseCost::default(),
                })
                .into_iter()
                .collect(),
            EnglishLexicalSlot::QuantityAtLeast => self.scan_at_least_quantity(tokens, start),
            EnglishLexicalSlot::QuantityOr => self.scan_or_quantity(tokens, start),
            EnglishLexicalSlot::QuantityX => self
                .one_token_match(tokens, start, "X")
                .map(|end| quantity_match(end, QuantityKey::X))
                .into_iter()
                .collect(),
            EnglishLexicalSlot::QuantityBoth => self
                .one_token_match(tokens, start, "both")
                .map(|end| quantity_match(end, QuantityKey::Both))
                .into_iter()
                .collect(),
            EnglishLexicalSlot::QuantityThatMany => self
                .words_match(tokens, start, &["that", "many"])
                .map(|end| quantity_match(end, QuantityKey::ThatMany))
                .into_iter()
                .collect(),
            EnglishLexicalSlot::QuantityThatMuch => self
                .words_match(tokens, start, &["that", "much"])
                .map(|end| quantity_match(end, QuantityKey::ThatMuch))
                .into_iter()
                .collect(),
            EnglishLexicalSlot::QuantityBound(kind) => {
                self.scan_bounded_quantity(tokens, start, kind)
            }
            EnglishLexicalSlot::Than => self
                .one_token_match(tokens, start, "than")
                .map(|end| literal_match(end, LiteralKey::Than))
                .into_iter()
                .collect(),
            EnglishLexicalSlot::OrEqualTo => self
                .words_match(tokens, start, &["or", "equal", "to"])
                .map(|end| literal_match(end, LiteralKey::OrEqualTo))
                .into_iter()
                .collect(),
            EnglishLexicalSlot::RelativeWho => self
                .one_token_match(tokens, start, "who")
                .map(|end| literal_match(end, LiteralKey::Who))
                .into_iter()
                .collect(),
            EnglishLexicalSlot::Face => self
                .one_token_match(tokens, start, "face")
                .map(|end| literal_match(end, LiteralKey::Face))
                .into_iter()
                .collect(),
            EnglishLexicalSlot::Up => self
                .one_token_match(tokens, start, "up")
                .map(|end| literal_match(end, LiteralKey::Up))
                .into_iter()
                .collect(),
            EnglishLexicalSlot::Down => self
                .one_token_match(tokens, start, "down")
                .map(|end| literal_match(end, LiteralKey::Down))
                .into_iter()
                .collect(),
            EnglishLexicalSlot::Not => self
                .one_token_match(tokens, start, "not")
                .map(|end| literal_match(end, LiteralKey::Not))
                .into_iter()
                .collect(),
            EnglishLexicalSlot::To => self
                .one_token_match(tokens, start, "to")
                .map(|end| literal_match(end, LiteralKey::To))
                .into_iter()
                .collect(),
            EnglishLexicalSlot::Of => self
                .one_token_match(tokens, start, "of")
                .map(|end| LexicalMatch {
                    end,
                    features: Features::Preposition(Preposition::Of),
                    meaning: MeaningKey::Preposition(Preposition::Of),
                    local_cost: ParseCost::default(),
                })
                .into_iter()
                .collect(),
            EnglishLexicalSlot::Reciprocal => self
                .words_match(tokens, start, &["each", "other"])
                .map(|end| pronoun_match(end, Pronoun::EachOther, PronounCase::Object))
                .into_iter()
                .collect(),
            EnglishLexicalSlot::ThisCard => tokens
                .get(start)
                .filter(|token| token.kind == TokenKind::SelfReference)
                .map(|_| this_card_match(start + 1, ThisCardForm::AbbreviatedName))
                .into_iter()
                .collect(),
            EnglishLexicalSlot::FullThisCard => tokens
                .get(start)
                .filter(|token| token.kind == TokenKind::FullSelfReference)
                .map(|_| this_card_match(start + 1, ThisCardForm::FullName))
                .into_iter()
                .collect(),
            EnglishLexicalSlot::PossessiveThisCard => {
                let form = match tokens.get(start).map(|token| token.kind) {
                    Some(TokenKind::SelfReference) => Some(ThisCardForm::AbbreviatedName),
                    Some(TokenKind::FullSelfReference) => Some(ThisCardForm::FullName),
                    _ => None,
                };
                form.and_then(|form| {
                    self.one_token_match(tokens, start + 1, "'s")
                        .map(|end| LexicalMatch {
                            end,
                            features: Features::PossessiveThisCard {
                                agreement: Agreement {
                                    person: Person::Third,
                                    number: Number::Singular,
                                },
                            },
                            meaning: MeaningKey::ThisCard(form),
                            local_cost: ParseCost::default(),
                        })
                })
                .into_iter()
                .collect()
            }
            EnglishLexicalSlot::Preposition => self.scan_preposition(tokens, start),
            EnglishLexicalSlot::Existential => self.scan_existential(tokens, start),
            slot @ (EnglishLexicalSlot::PossessiveNoun
            | EnglishLexicalSlot::OracleSymbol
            | EnglishLexicalSlot::PowerToughness
            | EnglishLexicalSlot::Punctuation(_)
            | EnglishLexicalSlot::Subordinator
            | EnglishLexicalSlot::RatherThan
            | EnglishLexicalSlot::Conjunction
            | EnglishLexicalSlot::Plus) => self.scan_clause_lexical(slot, tokens, start),
            EnglishLexicalSlot::Unknown(slot)
                if self.recovery_profile != RecoveryProfile::Exact =>
            {
                let already_known = self.has_known_word(tokens, start);
                if already_known {
                    Vec::new()
                } else {
                    let mut matches = recovery::scan_unknown(self.source, tokens, start, slot);
                    matches.retain(|candidate| {
                        !((start + 1)..candidate.end)
                            .any(|index| self.has_known_word(tokens, index))
                    });
                    matches
                }
            }
            EnglishLexicalSlot::Unknown(_) => Vec::new(),
        }
    }

    fn reduce(
        &self,
        rule: RuleId,
        children: &[Child<'_, Self>],
    ) -> Option<Reduction<Self::Features>> {
        reduce(self.tags.get(rule.index()).copied()?, children)
    }

    fn accepts_prefix(
        &self,
        rule: RuleId,
        completed_children: usize,
        latest_child: &Self::Features,
    ) -> bool {
        self.tags.get(rule.index()).copied().is_some_and(|tag| {
            clause::accepts_predicate_prefix(tag, completed_children, latest_child)
        })
    }

    fn state_limit(&self) -> Option<usize> {
        (self.recovery_profile == RecoveryProfile::Phrases).then_some(RECOVERY_STATE_LIMIT)
    }
}

impl EnglishGrammar<'_, '_> {
    fn scan_clause_lexical(
        &self,
        slot: EnglishLexicalSlot,
        tokens: &[Token],
        start: usize,
    ) -> Vec<LexicalMatch<Features, MeaningKey>> {
        match slot {
            EnglishLexicalSlot::PossessiveNoun => self.scan_possessive_noun(tokens, start),
            EnglishLexicalSlot::OracleSymbol => self
                .magic_match(tokens, start, TokenKind::OracleSymbol)
                .and_then(|(surface, end)| {
                    OracleSymbol::new(surface).map(|symbol| LexicalMatch {
                        end,
                        features: Features::None,
                        meaning: MeaningKey::OracleSymbol(symbol),
                        local_cost: ParseCost::default(),
                    })
                })
                .into_iter()
                .collect(),
            EnglishLexicalSlot::PowerToughness => self
                .magic_match(tokens, start, TokenKind::PowerToughness)
                .and_then(|(surface, end)| {
                    parse_power_toughness(surface).map(|power_toughness| LexicalMatch {
                        end,
                        features: Features::None,
                        meaning: MeaningKey::PowerToughness(power_toughness),
                        local_cost: ParseCost::default(),
                    })
                })
                .into_iter()
                .collect(),
            EnglishLexicalSlot::Punctuation(expected) => tokens
                .get(start)
                .filter(|token| token.kind == TokenKind::Punctuation(expected))
                .map(|_| LexicalMatch {
                    end: start + 1,
                    features: Features::None,
                    meaning: MeaningKey::Punctuation(expected),
                    local_cost: ParseCost::default(),
                })
                .into_iter()
                .collect(),
            EnglishLexicalSlot::Subordinator => self.scan_subordinators(tokens, start),
            EnglishLexicalSlot::RatherThan => self
                .words_match(tokens, start, &["rather", "than"])
                .map(|end| LexicalMatch {
                    end,
                    features: Features::None,
                    meaning: MeaningKey::Subordinator(crate::syntax::Subordinator::RatherThan),
                    local_cost: ParseCost::default(),
                })
                .into_iter()
                .collect(),
            EnglishLexicalSlot::Conjunction => self.scan_conjunction(tokens, start),
            EnglishLexicalSlot::Plus => self
                .one_token_match(tokens, start, "plus")
                .map(|end| literal_match(end, LiteralKey::Plus))
                .into_iter()
                .collect(),
            _ => Vec::new(),
        }
    }

    fn has_known_word(&self, tokens: &[Token], start: usize) -> bool {
        if tokens
            .get(start)
            .is_none_or(|token| token.kind != TokenKind::Word)
        {
            return true;
        }
        let vocabulary_slots = [
            LexicalSlot::Noun(NounUsage::Either),
            LexicalSlot::Adjective,
            LexicalSlot::Adverb,
            LexicalSlot::Pronoun(PronounCase::Subject),
            LexicalSlot::Pronoun(PronounCase::Object),
            LexicalSlot::Auxiliary,
        ];
        vocabulary_slots
            .into_iter()
            .any(|slot| !self.word_matches(tokens, start, slot).is_empty())
            || crate::word::VERB_SLOTS.into_iter().any(|slot| {
                !self
                    .word_matches(tokens, start, LexicalSlot::Verb(slot))
                    .is_empty()
            })
            || !self.scan_determiner(tokens, start).is_empty()
            || !self.scan_preposition(tokens, start).is_empty()
            || !self.scan_conjunction(tokens, start).is_empty()
            || self.subordinator_at(tokens, start).is_some()
            || self.one_token_match(tokens, start, "plus").is_some()
            || self.one_token_match(tokens, start, "who").is_some()
            || !self
                .catalog_matches(tokens, start, CatalogSlot::Noun(NounUsage::Either))
                .is_empty()
            || !self
                .catalog_matches(tokens, start, CatalogSlot::Adjective)
                .is_empty()
            || !self
                .catalog_matches(tokens, start, CatalogSlot::AbilityItem)
                .is_empty()
            || crate::word::VERB_SLOTS.into_iter().any(|slot| {
                !self
                    .catalog_matches(tokens, start, CatalogSlot::Verb(slot))
                    .is_empty()
            })
    }

    fn scan_subordinators(
        &self,
        tokens: &[Token],
        start: usize,
    ) -> Vec<LexicalMatch<Features, MeaningKey>> {
        self.subordinator_at(tokens, start)
            .map(|(end, subordinator)| LexicalMatch {
                end,
                features: Features::None,
                meaning: MeaningKey::Subordinator(subordinator),
                local_cost: ParseCost::default(),
            })
            .into_iter()
            .collect()
    }

    fn subordinator_at(
        &self,
        tokens: &[Token],
        start: usize,
    ) -> Option<(usize, crate::syntax::Subordinator)> {
        let surface = self.token_text(tokens, start)?;
        if surface.eq_ignore_ascii_case("for")
            && let Some(end) = self.words_match(tokens, start, &["for", "as", "long", "as"])
        {
            return Some((end, crate::syntax::Subordinator::ForAsLongAs));
        }
        if surface.eq_ignore_ascii_case("as") {
            if let Some(end) = self.words_match(tokens, start, &["as", "long", "as"]) {
                return Some((end, crate::syntax::Subordinator::AsLongAs));
            }
            return Some((start + 1, crate::syntax::Subordinator::As));
        }
        let subordinator = if surface.eq_ignore_ascii_case("if") {
            crate::syntax::Subordinator::If
        } else if surface.eq_ignore_ascii_case("while") {
            crate::syntax::Subordinator::While
        } else if surface.eq_ignore_ascii_case("unless") {
            crate::syntax::Subordinator::Unless
        } else {
            return None;
        };
        Some((start + 1, subordinator))
    }

    fn scan_existential(
        &self,
        tokens: &[Token],
        start: usize,
    ) -> Vec<LexicalMatch<Features, MeaningKey>> {
        [
            ((&["there", "is"] as &[&str]), ExistentialForm::Is),
            ((&["there's"] as &[&str]), ExistentialForm::ContractedIs),
            ((&["there", "are"] as &[&str]), ExistentialForm::Are),
            ((&["there", "was"] as &[&str]), ExistentialForm::Was),
            ((&["there", "were"] as &[&str]), ExistentialForm::Were),
        ]
        .into_iter()
        .filter_map(|(words, form)| {
            self.words_match(tokens, start, words)
                .map(|end| LexicalMatch {
                    end,
                    features: Features::Existential {
                        number: match form {
                            ExistentialForm::Is
                            | ExistentialForm::ContractedIs
                            | ExistentialForm::Was => Number::Singular,
                            ExistentialForm::Are | ExistentialForm::Were => Number::Plural,
                        },
                    },
                    meaning: MeaningKey::Existential(form),
                    local_cost: ParseCost::default(),
                })
        })
        .collect()
    }

    fn scan_subject_auxiliary(
        &self,
        tokens: &[Token],
        start: usize,
    ) -> Vec<LexicalMatch<Features, MeaningKey>> {
        [
            (
                "you're",
                ContractedSubjectKey::Pronoun(Pronoun::You),
                Auxiliary::Be,
                Person::Second,
                Number::Singular,
            ),
            (
                "you've",
                ContractedSubjectKey::Pronoun(Pronoun::You),
                Auxiliary::Have,
                Person::Second,
                Number::Singular,
            ),
            (
                "he's",
                ContractedSubjectKey::Pronoun(Pronoun::It(crate::word::Gender::Masculine)),
                Auxiliary::Be,
                Person::Third,
                Number::Singular,
            ),
            (
                "he's",
                ContractedSubjectKey::Pronoun(Pronoun::It(crate::word::Gender::Masculine)),
                Auxiliary::Have,
                Person::Third,
                Number::Singular,
            ),
            (
                "she's",
                ContractedSubjectKey::Pronoun(Pronoun::It(crate::word::Gender::Feminine)),
                Auxiliary::Be,
                Person::Third,
                Number::Singular,
            ),
            (
                "she's",
                ContractedSubjectKey::Pronoun(Pronoun::It(crate::word::Gender::Feminine)),
                Auxiliary::Have,
                Person::Third,
                Number::Singular,
            ),
            (
                "it's",
                ContractedSubjectKey::Pronoun(Pronoun::It(crate::word::Gender::Neuter)),
                Auxiliary::Be,
                Person::Third,
                Number::Singular,
            ),
            (
                "it's",
                ContractedSubjectKey::Pronoun(Pronoun::It(crate::word::Gender::Neuter)),
                Auxiliary::Have,
                Person::Third,
                Number::Singular,
            ),
            (
                "that's",
                ContractedSubjectKey::Demonstrative(Demonstrative::That),
                Auxiliary::Be,
                Person::Third,
                Number::Singular,
            ),
            (
                "that's",
                ContractedSubjectKey::Demonstrative(Demonstrative::That),
                Auxiliary::Have,
                Person::Third,
                Number::Singular,
            ),
            (
                "they're",
                ContractedSubjectKey::Pronoun(Pronoun::They),
                Auxiliary::Be,
                Person::Third,
                Number::Plural,
            ),
            (
                "they've",
                ContractedSubjectKey::Pronoun(Pronoun::They),
                Auxiliary::Have,
                Person::Third,
                Number::Plural,
            ),
        ]
        .into_iter()
        .filter_map(|(surface, subject, auxiliary, person, number)| {
            self.one_token_match(tokens, start, surface).map(|end| {
                let auxiliary = AuxiliaryInstance {
                    auxiliary,
                    inflection: AuxiliaryInflection::Present { person, number },
                    contracted_negation: false,
                };
                LexicalMatch {
                    end,
                    features: Features::SubjectAuxiliary {
                        subject,
                        agreement: Agreement { person, number },
                        auxiliary,
                    },
                    meaning: MeaningKey::SubjectAuxiliary(SubjectAuxiliaryKey {
                        subject,
                        auxiliary,
                    }),
                    local_cost: ParseCost {
                        precedence: u32::from(
                            auxiliary.auxiliary == Auxiliary::Have && surface.ends_with("'s"),
                        ),
                        ..ParseCost::default()
                    },
                }
            })
        })
        .collect()
    }

    fn scan_possessive_noun(
        &self,
        tokens: &[Token],
        start: usize,
    ) -> Vec<LexicalMatch<Features, MeaningKey>> {
        let Some(token) = tokens
            .get(start)
            .filter(|token| token.kind == TokenKind::Word)
        else {
            return Vec::new();
        };
        let Some(surface) = token.span.text(self.source) else {
            return Vec::new();
        };
        let Some(stem) = surface.strip_suffix("'s") else {
            return Vec::new();
        };
        let mut matches = Vocabulary::new()
            .matches(stem, LexicalSlot::Noun(NounUsage::Either))
            .into_iter()
            .filter_map(|word| lexical_word_match(word, start + 1))
            .collect::<Vec<_>>();
        matches.extend(
            self.catalogs
                .matches(stem, CatalogSlot::Noun(NounUsage::Either))
                .into_iter()
                .filter(|catalog_match| catalog_match.length == stem.len())
                .filter_map(|catalog_match| match catalog_match.value {
                    CatalogValue::Word(word) => lexical_word_match(word, start + 1),
                    CatalogValue::Atom(_) => None,
                }),
        );
        matches
    }

    fn magic_match<'grammar>(
        &'grammar self,
        tokens: &[Token],
        start: usize,
        kind: TokenKind,
    ) -> Option<(&'grammar str, usize)> {
        let token = tokens.get(start).filter(|token| token.kind == kind)?;
        Some((token.span.text(self.source)?, start + 1))
    }

    fn scan_conjunction(
        &self,
        tokens: &[Token],
        start: usize,
    ) -> Vec<LexicalMatch<Features, MeaningKey>> {
        use crate::syntax::PredicateConjunction;

        let Some(surface) = self.token_text(tokens, start) else {
            return Vec::new();
        };
        let conjunction = if surface.eq_ignore_ascii_case("and") {
            PredicateConjunction::And
        } else if surface.eq_ignore_ascii_case("or") {
            PredicateConjunction::Or
        } else if surface.eq_ignore_ascii_case("then") {
            PredicateConjunction::Then
        } else {
            return Vec::new();
        };
        vec![LexicalMatch {
            end: start + 1,
            features: Features::Conjunction(conjunction),
            meaning: MeaningKey::Conjunction(conjunction),
            local_cost: ParseCost::default(),
        }]
    }

    fn scan_determiner(
        &self,
        tokens: &[Token],
        start: usize,
    ) -> Vec<LexicalMatch<Features, MeaningKey>> {
        let Some(surface) = self.token_text(tokens, start) else {
            return Vec::new();
        };
        let key = if surface.eq_ignore_ascii_case("the") {
            DeterminerKey::The
        } else if surface.eq_ignore_ascii_case("each") {
            DeterminerKey::Each
        } else if surface.eq_ignore_ascii_case("another") {
            DeterminerKey::Another
        } else if surface.eq_ignore_ascii_case("a") {
            DeterminerKey::Indefinite(IndefiniteArticleKey::A)
        } else if surface.eq_ignore_ascii_case("an") {
            DeterminerKey::Indefinite(IndefiniteArticleKey::An)
        } else if surface.eq_ignore_ascii_case("this") {
            DeterminerKey::Demonstrative(DemonstrativeKey::This)
        } else if surface.eq_ignore_ascii_case("that") {
            DeterminerKey::Demonstrative(DemonstrativeKey::That)
        } else if surface.eq_ignore_ascii_case("these") {
            DeterminerKey::Demonstrative(DemonstrativeKey::These)
        } else if surface.eq_ignore_ascii_case("those") {
            DeterminerKey::Demonstrative(DemonstrativeKey::Those)
        } else if surface.eq_ignore_ascii_case("all") {
            DeterminerKey::All
        } else if surface.eq_ignore_ascii_case("any") {
            DeterminerKey::Any
        } else if surface.eq_ignore_ascii_case("no") {
            DeterminerKey::No
        } else if surface.eq_ignore_ascii_case("your") {
            DeterminerKey::Possessive(Pronoun::You)
        } else if surface.eq_ignore_ascii_case("his") {
            DeterminerKey::Possessive(Pronoun::It(crate::word::Gender::Masculine))
        } else if surface.eq_ignore_ascii_case("her") {
            DeterminerKey::Possessive(Pronoun::It(crate::word::Gender::Feminine))
        } else if surface.eq_ignore_ascii_case("its") {
            DeterminerKey::Possessive(Pronoun::It(crate::word::Gender::Neuter))
        } else if surface.eq_ignore_ascii_case("their") {
            DeterminerKey::Possessive(Pronoun::They)
        } else {
            return Vec::new();
        };
        vec![LexicalMatch {
            end: start + 1,
            features: Features::Determiner {
                cardinality: key.cardinality(),
                article: key.article(),
            },
            meaning: MeaningKey::Determiner(key),
            local_cost: ParseCost::default(),
        }]
    }

    fn scan_at_least_quantity(
        &self,
        tokens: &[Token],
        start: usize,
    ) -> Vec<LexicalMatch<Features, MeaningKey>> {
        let (surface, end, at_least_surface) =
            if let Some(number_start) = self.words_match(tokens, start, &["at", "least"]) {
                let Some(surface) = self.token_text(tokens, number_start) else {
                    return Vec::new();
                };
                (surface, number_start + 1, true)
            } else {
                let Some(surface) = self.token_text(tokens, start) else {
                    return Vec::new();
                };
                let Some(end) = self.words_match(tokens, start + 1, &["or", "more"]) else {
                    return Vec::new();
                };
                (surface, end, false)
            };
        [
            NumberNotation::Cardinal,
            NumberNotation::Ordinal,
            NumberNotation::Arabic(false),
            NumberNotation::Arabic(true),
            NumberNotation::Roman,
        ]
        .into_iter()
        .filter_map(|notation| {
            notation.numeral().parse(surface).ok().map(|value| {
                let number = NumberKey { value, notation };
                quantity_match(
                    end,
                    if at_least_surface {
                        QuantityKey::AtLeast(number)
                    } else {
                        QuantityKey::OrMore(number)
                    },
                )
            })
        })
        .collect()
    }

    fn scan_bounded_quantity(
        &self,
        tokens: &[Token],
        start: usize,
        kind: BoundedQuantityKind,
    ) -> Vec<LexicalMatch<Features, MeaningKey>> {
        let opening = match kind {
            BoundedQuantityKind::MoreThan => "more",
            BoundedQuantityKind::FewerThan => "fewer",
        };
        let Some(number_start) = self.words_match(tokens, start, &[opening, "than"]) else {
            return Vec::new();
        };
        let Some(surface) = self.token_text(tokens, number_start) else {
            return Vec::new();
        };
        let end = number_start + 1;
        [
            NumberNotation::Cardinal,
            NumberNotation::Ordinal,
            NumberNotation::Arabic(false),
            NumberNotation::Arabic(true),
            NumberNotation::Roman,
        ]
        .into_iter()
        .filter_map(|notation| {
            notation.numeral().parse(surface).ok().map(|value| {
                let number = NumberKey { value, notation };
                let quantity = match kind {
                    BoundedQuantityKind::MoreThan => QuantityKey::MoreThan(number),
                    BoundedQuantityKind::FewerThan => QuantityKey::FewerThan(number),
                };
                quantity_match(end, quantity)
            })
        })
        .collect()
    }

    fn scan_frequency(
        &self,
        tokens: &[Token],
        start: usize,
    ) -> Vec<LexicalMatch<Features, MeaningKey>> {
        let (bound, count_start) =
            if let Some(end) = self.words_match(tokens, start, &["no", "more", "than"]) {
                (FrequencyBound::NoMoreThan, end)
            } else if let Some(end) = self.words_match(tokens, start, &["more", "than"]) {
                (FrequencyBound::MoreThan, end)
            } else {
                return Vec::new();
            };
        if let Some(end) = self.one_token_match(tokens, count_start, "once") {
            return vec![frequency_match(
                end,
                FrequencyKey {
                    bound,
                    count: FrequencyCountKey::Once,
                },
            )];
        }
        if let Some(end) = self.one_token_match(tokens, count_start, "twice") {
            return vec![frequency_match(
                end,
                FrequencyKey {
                    bound,
                    count: FrequencyCountKey::Twice,
                },
            )];
        }
        let Some(surface) = self.token_text(tokens, count_start) else {
            return Vec::new();
        };
        let Some(end) = self.one_token_match(tokens, count_start + 1, "times") else {
            return Vec::new();
        };
        [
            NumberNotation::Cardinal,
            NumberNotation::Ordinal,
            NumberNotation::Arabic(false),
            NumberNotation::Arabic(true),
            NumberNotation::Roman,
        ]
        .into_iter()
        .filter_map(|notation| {
            notation.numeral().parse(surface).ok().map(|value| {
                frequency_match(
                    end,
                    FrequencyKey {
                        bound,
                        count: FrequencyCountKey::Times(NumberKey { value, notation }),
                    },
                )
            })
        })
        .collect()
    }

    fn scan_or_quantity(
        &self,
        tokens: &[Token],
        start: usize,
    ) -> Vec<LexicalMatch<Features, MeaningKey>> {
        let Some(first_surface) = self.token_text(tokens, start) else {
            return Vec::new();
        };
        let Some(second_start) = self.one_token_match(tokens, start + 1, "or") else {
            return Vec::new();
        };
        let Some(second_surface) = self.token_text(tokens, second_start) else {
            return Vec::new();
        };
        let end = second_start + 1;
        let notations = [
            NumberNotation::Cardinal,
            NumberNotation::Ordinal,
            NumberNotation::Arabic(false),
            NumberNotation::Arabic(true),
            NumberNotation::Roman,
        ];
        let mut matches = Vec::new();
        for first_notation in notations {
            let Ok(first_value) = first_notation.numeral().parse(first_surface) else {
                continue;
            };
            for second_notation in notations {
                let Ok(second_value) = second_notation.numeral().parse(second_surface) else {
                    continue;
                };
                matches.push(quantity_match(
                    end,
                    QuantityKey::Or(
                        NumberKey {
                            value: first_value,
                            notation: first_notation,
                        },
                        NumberKey {
                            value: second_value,
                            notation: second_notation,
                        },
                    ),
                ));
            }
        }
        matches
    }

    fn scan_preposition(
        &self,
        tokens: &[Token],
        start: usize,
    ) -> Vec<LexicalMatch<Features, MeaningKey>> {
        let Some(surface) = self.token_text(tokens, start) else {
            return Vec::new();
        };
        let preposition = if surface.eq_ignore_ascii_case("among") {
            Preposition::Among
        } else if surface.eq_ignore_ascii_case("as") {
            Preposition::As
        } else if surface.eq_ignore_ascii_case("at") {
            Preposition::At
        } else if surface.eq_ignore_ascii_case("before") {
            Preposition::Before
        } else if surface.eq_ignore_ascii_case("by") {
            Preposition::By
        } else if surface.eq_ignore_ascii_case("during") {
            Preposition::During
        } else if surface.eq_ignore_ascii_case("for") {
            Preposition::For
        } else if surface.eq_ignore_ascii_case("from") {
            Preposition::From
        } else if surface.eq_ignore_ascii_case("in") {
            Preposition::In
        } else if surface.eq_ignore_ascii_case("into") {
            Preposition::Into
        } else if surface.eq_ignore_ascii_case("of") {
            Preposition::Of
        } else if surface.eq_ignore_ascii_case("on") {
            Preposition::On
        } else if surface.eq_ignore_ascii_case("onto") {
            Preposition::Onto
        } else if surface.eq_ignore_ascii_case("to") {
            Preposition::To
        } else if surface.eq_ignore_ascii_case("until") {
            Preposition::Until
        } else if surface.eq_ignore_ascii_case("under") {
            Preposition::Under
        } else if surface.eq_ignore_ascii_case("with") {
            Preposition::With
        } else if surface.eq_ignore_ascii_case("without") {
            Preposition::Without
        } else {
            return Vec::new();
        };
        vec![LexicalMatch {
            end: start + 1,
            features: Features::Preposition(preposition),
            meaning: MeaningKey::Preposition(preposition),
            local_cost: ParseCost::default(),
        }]
    }
}

#[derive(Default)]
struct RuleBuilder {
    rules: Vec<Rule<Nonterminal, EnglishLexicalSlot>>,
    tags: Vec<RuleTag>,
    rules_by_lhs: HashMap<Nonterminal, Vec<RuleId>>,
}

impl RuleBuilder {
    fn add(
        &mut self,
        tag: RuleTag,
        lhs: Nonterminal,
        rhs: impl IntoIterator<Item = Expected<Nonterminal, EnglishLexicalSlot>>,
    ) {
        self.add_with_cost(tag, lhs, rhs, ParseCost::default());
    }

    fn add_with_cost(
        &mut self,
        tag: RuleTag,
        lhs: Nonterminal,
        rhs: impl IntoIterator<Item = Expected<Nonterminal, EnglishLexicalSlot>>,
        local_cost: ParseCost,
    ) {
        let id = RuleId::new(self.rules.len());
        self.rules.push(Rule {
            lhs,
            rhs: rhs.into_iter().collect(),
            local_cost,
        });
        self.tags.push(tag);
        self.rules_by_lhs.entry(lhs).or_default().push(id);
    }

    fn add_nominal_rules(&mut self) {
        use EnglishLexicalSlot as L;
        use Expected::Lexical as l;
        use Expected::Nonterminal as n;
        use Nonterminal as N;

        for notation in [
            NumberNotation::Cardinal,
            NumberNotation::Ordinal,
            NumberNotation::Arabic(false),
            NumberNotation::Arabic(true),
            NumberNotation::Roman,
        ] {
            self.add(
                RuleTag::QuantityExact,
                N::Quantity,
                [l(L::Number(notation))],
            );
            self.add(
                RuleTag::QuantityUpTo,
                N::Quantity,
                [l(L::Up), l(L::To), l(L::Number(notation))],
            );
        }
        self.add(
            RuleTag::QuantityAtLeast,
            N::Quantity,
            [l(L::QuantityAtLeast)],
        );
        self.add(RuleTag::QuantityOr, N::Quantity, [l(L::QuantityOr)]);
        self.add(RuleTag::QuantityX, N::Quantity, [l(L::QuantityX)]);
        self.add(RuleTag::QuantityBoth, N::Quantity, [l(L::QuantityBoth)]);
        self.add(
            RuleTag::QuantityThatMany,
            N::Quantity,
            [l(L::QuantityThatMany)],
        );
        self.add(
            RuleTag::QuantityThatMuch,
            N::Quantity,
            [l(L::QuantityThatMuch)],
        );
        self.add(
            RuleTag::QuantityMoreThan,
            N::Quantity,
            [l(L::QuantityBound(BoundedQuantityKind::MoreThan))],
        );
        self.add(
            RuleTag::QuantityFewerThan,
            N::Quantity,
            [l(L::QuantityBound(BoundedQuantityKind::FewerThan))],
        );

        self.add(RuleTag::DeterminerClosed, N::Determiner, [l(L::Determiner)]);
        self.add(
            RuleTag::DeterminerTarget,
            N::Determiner,
            [l(L::DeterminerTarget)],
        );
        self.add(
            RuleTag::DeterminerQuantifiedTarget,
            N::Determiner,
            [n(N::Quantity), l(L::DeterminerTarget)],
        );
        self.add(RuleTag::DeterminerQuantity, N::Determiner, [n(N::Quantity)]);
        self.add(
            RuleTag::DeterminerPossessiveThisCard,
            N::Determiner,
            [l(L::PossessiveThisCard)],
        );
        self.add(
            RuleTag::DeterminerPossessiveNoun,
            N::Determiner,
            [n(N::PossessiveNounPhrase)],
        );

        self.add(RuleTag::Adjective, N::Adjective, [l(L::Adjective)]);
        self.add(
            RuleTag::AdjectivePhrase,
            N::AdjectivePhrase,
            [n(N::Adjective)],
        );
        self.add(
            RuleTag::AdjectivePhraseFaceUp,
            N::AdjectivePhrase,
            [l(L::Face), l(L::Up)],
        );
        self.add(
            RuleTag::AdjectivePhraseFaceDown,
            N::AdjectivePhrase,
            [l(L::Face), l(L::Down)],
        );
        self.add(
            RuleTag::ComparisonStandard,
            N::ComparisonStandard,
            [n(N::NounPhrase)],
        );
        self.add(
            RuleTag::ComparisonStandard,
            N::ComparisonStandard,
            [n(N::AdjectivePhrase)],
        );
        self.add(
            RuleTag::ComparisonStandard,
            N::ComparisonStandard,
            [n(N::Clause)],
        );
        self.add(
            RuleTag::ComparisonThan,
            N::ComparisonComplement,
            [l(L::Than), n(N::ComparisonStandard)],
        );
        self.add(
            RuleTag::ComparisonThanOrEqualTo,
            N::ComparisonComplement,
            [l(L::Than), l(L::OrEqualTo), n(N::ComparisonStandard)],
        );
        self.add(
            RuleTag::AdjectivePhraseComparison,
            N::AdjectivePhrase,
            [n(N::Adjective), n(N::ComparisonComplement)],
        );
        self.add(RuleTag::Noun, N::Noun, [l(L::Noun(NounUsage::Either))]);
        self.add(
            RuleTag::PossessiveNounBase,
            N::PossessiveNounPhrase,
            [l(L::PossessiveNoun)],
        );
        self.add(
            RuleTag::PossessiveNounDetermined,
            N::PossessiveNounPhrase,
            [n(N::Determiner), n(N::PossessiveNounPhrase)],
        );

        self.add(RuleTag::NominalNoun, N::Nominal, [n(N::Noun)]);
        self.add_with_cost(
            RuleTag::NominalAdjective,
            N::Nominal,
            [n(N::AdjectivePhrase), n(N::Nominal)],
            ParseCost {
                precedence: 1,
                ..ParseCost::default()
            },
        );
        self.add_with_cost(
            RuleTag::NominalNounModifier,
            N::Nominal,
            [n(N::Noun), n(N::Nominal)],
            ParseCost {
                precedence: 1,
                ..ParseCost::default()
            },
        );
        self.add_with_cost(
            RuleTag::NominalQuantityModifier,
            N::Nominal,
            [n(N::Quantity), n(N::Nominal)],
            ParseCost {
                precedence: 1,
                ..ParseCost::default()
            },
        );
        self.add_with_cost(
            RuleTag::NominalPowerToughnessModifier,
            N::Nominal,
            [l(L::PowerToughness), n(N::Nominal)],
            ParseCost {
                precedence: 1,
                ..ParseCost::default()
            },
        );
        self.add(
            RuleTag::NominalDeterminer,
            N::Nominal,
            [n(N::Determiner), n(N::Nominal)],
        );
        self.add(
            RuleTag::NominalPrepositional,
            N::Nominal,
            [n(N::Nominal), n(N::PrepositionalPhrase)],
        );
        self.add_with_cost(
            RuleTag::NominalQuantityComplement,
            N::Nominal,
            [n(N::Nominal), n(N::Quantity)],
            ParseCost {
                precedence: 1,
                ..ParseCost::default()
            },
        );
        self.add(
            RuleTag::NominalRelative,
            N::Nominal,
            [n(N::Nominal), n(N::RelativeClause)],
        );
        self.add(
            RuleTag::NominalPostpositiveAdjective,
            N::Nominal,
            [n(N::Nominal), n(N::AdjectivePhrase)],
        );
        self.add(
            RuleTag::NominalComparison,
            N::Nominal,
            [n(N::Nominal), n(N::ComparisonComplement)],
        );

        self.add(RuleTag::NounPhraseNominal, N::NounPhrase, [n(N::Nominal)]);
        self.add(
            RuleTag::NounPhraseSubjectPronoun,
            N::NounPhrase,
            [l(L::Pronoun(PronounCase::Subject))],
        );
        self.add(
            RuleTag::NounPhraseObjectPronoun,
            N::NounPhrase,
            [l(L::Pronoun(PronounCase::Object))],
        );
        self.add(
            RuleTag::NounPhraseReciprocal,
            N::NounPhrase,
            [l(L::Reciprocal)],
        );
        self.add(RuleTag::NounPhraseQuantity, N::NounPhrase, [n(N::Quantity)]);
        self.add(RuleTag::NounPhraseThisCard, N::NounPhrase, [l(L::ThisCard)]);
        self.add(
            RuleTag::NounPhraseFullThisCard,
            N::NounPhrase,
            [l(L::FullThisCard)],
        );
        self.add(
            RuleTag::NounPhrasePossessiveThisCard,
            N::NounPhrase,
            [l(L::PossessiveThisCard)],
        );
        self.add(
            RuleTag::NounPhrasePartitive,
            N::NounPhrase,
            [n(N::Quantity), l(L::Of), n(N::NounPhrase)],
        );
        self.add_with_cost(
            RuleTag::NounPhraseCoordination,
            N::NounPhrase,
            [n(N::NounPhrase), l(L::Conjunction), n(N::NounPhrase)],
            ParseCost {
                precedence: 1,
                ..ParseCost::default()
            },
        );
        self.add_with_cost(
            RuleTag::NounPhraseAdditiveCoordination,
            N::NounPhrase,
            [n(N::NounPhrase), l(L::Plus), n(N::NounPhrase)],
            ParseCost {
                precedence: 1,
                ..ParseCost::default()
            },
        );

        self.add(
            RuleTag::PrepositionalPhrase,
            N::PrepositionalPhrase,
            [l(L::Preposition), n(N::PrepositionalObject)],
        );
        self.add(
            RuleTag::PrepositionalObject,
            N::PrepositionalObject,
            [n(N::NounPhrase)],
        );
        self.add(
            RuleTag::PrepositionalObject,
            N::PrepositionalObject,
            [n(N::PrepositionalPhrase)],
        );
        self.add(
            RuleTag::PrepositionalObject,
            N::PrepositionalObject,
            [n(N::GerundClause)],
        );
        self.add(
            RuleTag::PrepositionalObject,
            N::PrepositionalObject,
            [l(L::Adverb)],
        );
    }

    fn add_clause_rules(&mut self) {
        clause::add_rules(self);
    }
}

fn lexical_word_match(word: WordMatch, end: usize) -> Option<LexicalMatch<Features, MeaningKey>> {
    let (features, meaning) = match word {
        WordMatch::Noun(noun) => {
            let form = noun_form(&noun);
            let initial_sound = noun_initial_sound(&noun)?;
            let temporal = noun_is_temporal(&noun);
            (
                Features::Noun {
                    form,
                    initial_sound,
                    temporal,
                },
                MeaningKey::Noun(noun),
            )
        }
        WordMatch::Verb(verb) => {
            let accepts_direct_object = !matches!(verb.verb, crate::word::Verb::Word(Vocab::Be));
            let requires_direct_object = matches!(verb.verb, crate::word::Verb::Word(Vocab::Have));
            let proform = matches!(verb.verb, crate::word::Verb::Word(Vocab::Do));
            (
                Features::Verb {
                    slot: verb.slot,
                    accepts_direct_object,
                    requires_direct_object,
                    proform,
                },
                MeaningKey::Verb(verb),
            )
        }
        WordMatch::Adjective(adjective) => (
            Features::Adjective {
                initial_sound: adjective_initial_sound(&adjective)?,
                comparison: adjective_comparison_state(&adjective),
                card_orientation: false,
            },
            MeaningKey::Adjective(adjective),
        ),
        WordMatch::Adverb(adverb) => (Features::None, MeaningKey::Adverb(adverb)),
        WordMatch::Pronoun(pronoun) => (
            noun_phrase_features(pronoun.pronoun, Some(pronoun.case)),
            MeaningKey::Pronoun(pronoun),
        ),
        WordMatch::Auxiliary(auxiliary) => (
            Features::Auxiliary(auxiliary),
            MeaningKey::Auxiliary(auxiliary),
        ),
    };
    Some(LexicalMatch {
        end,
        features,
        meaning,
        local_cost: ParseCost::default(),
    })
}

fn quantity_match(end: usize, quantity: QuantityKey) -> LexicalMatch<Features, MeaningKey> {
    LexicalMatch {
        end,
        features: Features::Quantity(quantity.cardinality()),
        meaning: MeaningKey::Quantity(quantity),
        local_cost: ParseCost::default(),
    }
}

fn frequency_match(end: usize, frequency: FrequencyKey) -> LexicalMatch<Features, MeaningKey> {
    LexicalMatch {
        end,
        features: Features::None,
        meaning: MeaningKey::Frequency(frequency),
        local_cost: ParseCost::default(),
    }
}

fn literal_match(end: usize, literal: LiteralKey) -> LexicalMatch<Features, MeaningKey> {
    LexicalMatch {
        end,
        features: Features::None,
        meaning: MeaningKey::Literal(literal),
        local_cost: ParseCost::default(),
    }
}

fn pronoun_match(
    end: usize,
    pronoun: Pronoun,
    case: PronounCase,
) -> LexicalMatch<Features, MeaningKey> {
    let instance = PronounInstance { pronoun, case };
    LexicalMatch {
        end,
        features: noun_phrase_features(pronoun, Some(case)),
        meaning: MeaningKey::Pronoun(instance),
        local_cost: ParseCost::default(),
    }
}

fn this_card_match(end: usize, form: ThisCardForm) -> LexicalMatch<Features, MeaningKey> {
    LexicalMatch {
        end,
        features: Features::NounPhrase {
            agreement: Some(Agreement {
                person: Person::Third,
                number: Number::Singular,
            }),
            pronoun_case: None,
            temporal: false,
        },
        meaning: MeaningKey::ThisCard(form),
        local_cost: ParseCost::default(),
    }
}

fn noun_phrase_features(pronoun: Pronoun, case: Option<PronounCase>) -> Features {
    let agreement = match pronoun {
        Pronoun::You => Some(Agreement {
            person: Person::Second,
            number: Number::Singular,
        }),
        Pronoun::It(_) => Some(Agreement {
            person: Person::Third,
            number: Number::Singular,
        }),
        Pronoun::They => Some(Agreement {
            person: Person::Third,
            number: Number::Plural,
        }),
        Pronoun::EachOther => None,
    };
    Features::NounPhrase {
        agreement,
        pronoun_case: case,
        temporal: false,
    }
}

const fn copula_agreement(auxiliary: AuxiliaryInstance) -> Option<Agreement> {
    if !matches!(auxiliary.auxiliary, Auxiliary::Be) {
        return None;
    }
    match auxiliary.inflection {
        AuxiliaryInflection::Present { person, number }
        | AuxiliaryInflection::Past { person, number } => Some(Agreement { person, number }),
        AuxiliaryInflection::Base
        | AuxiliaryInflection::PresentParticiple
        | AuxiliaryInflection::PastParticiple => None,
    }
}

fn noun_form(noun: &NounInstance) -> NounForm {
    match noun {
        NounInstance::Singular(_) => NounForm::Singular,
        NounInstance::Plural(_) => NounForm::Plural,
        NounInstance::Mass(_) => NounForm::Mass,
    }
}

fn noun_is_temporal(noun: &NounInstance) -> bool {
    let noun = match noun {
        NounInstance::Singular(noun) | NounInstance::Plural(noun) | NounInstance::Mass(noun) => {
            noun
        }
    };
    matches!(noun, Noun::Word(Vocab::Combat | Vocab::Turn))
}

fn noun_initial_sound(noun: &NounInstance) -> Option<InitialSound> {
    let noun_identity = match noun {
        NounInstance::Singular(noun) | NounInstance::Plural(noun) | NounInstance::Mass(noun) => {
            noun
        }
    };
    match noun_identity {
        Noun::Word(vocab) => Some(Vocabulary::new().initial_sound(*vocab)),
        Noun::Catalog(atom) => Some(surface_initial_sound(atom.canonical())),
        Noun::Die(_) => Some(InitialSound::Consonant),
        Noun::Gerund(_) => Vocabulary::new()
            .render_noun(noun)
            .map(|surface| surface_initial_sound(&surface)),
        Noun::Unknown(unknown) => Some(surface_initial_sound(&unknown.0)),
    }
}

fn adjective_initial_sound(adjective: &Adjective) -> Option<InitialSound> {
    match adjective {
        Adjective::Word(vocab) => Some(Vocabulary::new().initial_sound(*vocab)),
        _ => Vocabulary::new()
            .render_adjective(adjective)
            .map(|surface| surface_initial_sound(&surface)),
    }
}

fn adjective_comparison_state(adjective: &Adjective) -> AdjectiveComparisonState {
    match adjective {
        Adjective::Word(word)
            if matches!(
                word.spelling(),
                "fewer" | "greater" | "less" | "more" | "other"
            ) =>
        {
            AdjectiveComparisonState::Pending
        }
        _ => AdjectiveComparisonState::NotComparative,
    }
}

fn surface_initial_sound(surface: &str) -> InitialSound {
    if surface
        .chars()
        .next()
        .is_some_and(|first| matches!(first.to_ascii_lowercase(), 'a' | 'e' | 'i' | 'o' | 'u'))
    {
        InitialSound::Vowel
    } else {
        InitialSound::Consonant
    }
}

fn parse_power_toughness(surface: &str) -> Option<PowerToughness> {
    let (power, toughness) = surface.split_once('/')?;
    Some(PowerToughness {
        power: parse_signed_scalar(power)?,
        toughness: parse_signed_scalar(toughness)?,
    })
}

fn parse_signed_scalar(surface: &str) -> Option<crate::syntax::SignedScalar> {
    use crate::syntax::ScalarSign;
    use crate::syntax::ScalarValue;

    let (sign, body) = if let Some(body) = surface.strip_prefix('+') {
        (ScalarSign::Plus, body)
    } else if let Some(body) = surface
        .strip_prefix('-')
        .or_else(|| surface.strip_prefix('−'))
    {
        (ScalarSign::Minus, body)
    } else {
        (ScalarSign::None, surface)
    };
    let value = match body {
        "X" => ScalarValue::X,
        "*" => ScalarValue::Star,
        _ => ScalarValue::Integer(body.parse().ok()?),
    };
    Some(crate::syntax::SignedScalar { sign, value })
}

fn reduce(
    tag: RuleTag,
    children: &[Child<'_, EnglishGrammar<'_, '_>>],
) -> Option<Reduction<Features>> {
    let features = match tag {
        RuleTag::QuantityExact
        | RuleTag::QuantityAtLeast
        | RuleTag::QuantityOr
        | RuleTag::QuantityX
        | RuleTag::QuantityBoth
        | RuleTag::QuantityUpTo
        | RuleTag::QuantityThatMany
        | RuleTag::QuantityThatMuch
        | RuleTag::QuantityMoreThan
        | RuleTag::QuantityFewerThan
        | RuleTag::DeterminerClosed
        | RuleTag::DeterminerTarget
        | RuleTag::DeterminerQuantifiedTarget
        | RuleTag::DeterminerQuantity
        | RuleTag::DeterminerPossessiveThisCard => reduce_quantity_or_determiner(tag, children)?,
        RuleTag::FrequencyPhrase => Features::None,
        RuleTag::PossessiveNounBase
        | RuleTag::PossessiveNounDetermined
        | RuleTag::DeterminerPossessiveNoun => reduce_possessive_noun_phrase(tag, children)?,
        RuleTag::Adjective
        | RuleTag::AdjectivePhrase
        | RuleTag::AdjectivePhraseFaceUp
        | RuleTag::AdjectivePhraseFaceDown
        | RuleTag::AdjectivePhraseComparison
        | RuleTag::ComparisonStandard
        | RuleTag::ComparisonThan
        | RuleTag::ComparisonThanOrEqualTo
        | RuleTag::Noun
        | RuleTag::NominalNoun
        | RuleTag::NominalAdjective
        | RuleTag::NominalNounModifier
        | RuleTag::NominalQuantityModifier
        | RuleTag::NominalPowerToughnessModifier
        | RuleTag::NominalDeterminer
        | RuleTag::NominalPrepositional
        | RuleTag::NominalQuantityComplement
        | RuleTag::NominalRelative
        | RuleTag::NominalPostpositiveAdjective
        | RuleTag::NominalComparison => reduce_nominal(tag, children)?,
        RuleTag::NounPhraseNominal
        | RuleTag::NounPhraseSubjectPronoun
        | RuleTag::NounPhraseObjectPronoun
        | RuleTag::NounPhraseReciprocal
        | RuleTag::NounPhraseQuantity
        | RuleTag::NounPhraseThisCard
        | RuleTag::NounPhraseFullThisCard
        | RuleTag::NounPhrasePossessiveThisCard
        | RuleTag::NounPhrasePartitive
        | RuleTag::NounPhraseCoordination
        | RuleTag::NounPhraseAdditiveCoordination
        | RuleTag::PrepositionalPhrase
        | RuleTag::PrepositionalObject => reduce_phrase(tag, children)?,
        RuleTag::Verb
        | RuleTag::VerbPhraseBase
        | RuleTag::VerbPhraseAuxiliary
        | RuleTag::VerbPhraseDirectObject
        | RuleTag::VerbPhraseAdjective
        | RuleTag::VerbPhrasePrepositional
        | RuleTag::VerbPhraseInfinitive
        | RuleTag::VerbPhraseAdverb
        | RuleTag::VerbPhraseParticle
        | RuleTag::VerbPhraseFrequency
        | RuleTag::VerbPhraseAbility
        | RuleTag::VerbPhraseOracleSymbol
        | RuleTag::VerbPhrasePowerToughness
        | RuleTag::VerbPhraseQuantity
        | RuleTag::InfinitiveTo
        | RuleTag::InfinitiveNotTo
        | RuleTag::GerundClauseBase
        | RuleTag::GerundClauseSubordinateAfter
        | RuleTag::SimpleClauseSubject
        | RuleTag::SimpleClauseContractedSubject
        | RuleTag::SimpleClauseSubjectless
        | RuleTag::ClauseSimple
        | RuleTag::ClauseElliptical
        | RuleTag::ClauseCoordination
        | RuleTag::ClauseCoordinationComma
        | RuleTag::ClauseSubordinateBefore
        | RuleTag::ClauseSubordinateAfterElliptical
        | RuleTag::ClauseSubordinateAfter
        | RuleTag::ClauseSubordinateAfterInfinitive
        | RuleTag::ClauseExistential
        | RuleTag::ClauseCopularNoun
        | RuleTag::ClauseCopularAdjective
        | RuleTag::ClauseCopularPrepositional
        | RuleTag::ClauseContractedCopularNoun
        | RuleTag::ClauseContractedCopularAdjective
        | RuleTag::ClauseContractedCopularPrepositional
        | RuleTag::RelativeObject
        | RuleTag::RelativeObjectContractedSubject
        | RuleTag::RelativeSubjectContractedAuxiliary
        | RuleTag::RelativeSubject
        | RuleTag::RelativeContractedCopularNoun
        | RuleTag::RelativeContractedCopularAdjective
        | RuleTag::RelativeContractedCopularPrepositional
        | RuleTag::SentencePeriod
        | RuleTag::SentenceExclamation
        | RuleTag::SentenceQuestion
        | RuleTag::SentenceNone => clause::reduce_clause(tag, children)?,
        RuleTag::NounUnknown => recovery::reduce_recovery(tag, children)?,
    };
    Some(Reduction {
        features,
        local_cost: ParseCost::default(),
    })
}

fn reduce_possessive_noun_phrase(
    tag: RuleTag,
    children: &[Child<'_, EnglishGrammar<'_, '_>>],
) -> Option<Reduced> {
    match tag {
        RuleTag::PossessiveNounBase => {
            let Features::Noun {
                form,
                initial_sound,
                ..
            } = children.first()?.features
            else {
                return None;
            };
            Some(Features::PossessiveNounPhrase {
                form: *form,
                initial_sound: *initial_sound,
                determined: false,
            })
        }
        RuleTag::PossessiveNounDetermined => {
            let Features::Determiner {
                cardinality,
                article,
            } = children.first()?.features
            else {
                return None;
            };
            let Features::PossessiveNounPhrase {
                form,
                initial_sound,
                determined,
            } = children.get(1)?.features
            else {
                return None;
            };
            if *determined
                || !cardinality_accepts(*cardinality, *form)
                || !article_accepts(*article, *initial_sound)
            {
                return None;
            }
            Some(Features::PossessiveNounPhrase {
                form: *form,
                initial_sound: *initial_sound,
                determined: true,
            })
        }
        RuleTag::DeterminerPossessiveNoun => Some(Features::Determiner {
            cardinality: Cardinality::Unconstrained,
            article: None,
        }),
        _ => None,
    }
}

type Reduced = Features;

fn reduce_quantity_or_determiner(
    tag: RuleTag,
    children: &[Child<'_, EnglishGrammar<'_, '_>>],
) -> Option<Reduced> {
    match tag {
        RuleTag::QuantityExact => {
            let Features::Number { is_one } = children.first()?.features else {
                return None;
            };
            Some(Features::Quantity(number_cardinality(*is_one)))
        }
        RuleTag::QuantityAtLeast
        | RuleTag::QuantityOr
        | RuleTag::QuantityX
        | RuleTag::QuantityBoth
        | RuleTag::QuantityMoreThan
        | RuleTag::QuantityFewerThan => {
            let Features::Quantity(cardinality) = children.first()?.features else {
                return None;
            };
            Some(Features::Quantity(*cardinality))
        }
        RuleTag::QuantityUpTo => {
            let Features::Number { is_one } = children.get(2)?.features else {
                return None;
            };
            Some(Features::Quantity(number_cardinality(*is_one)))
        }
        RuleTag::QuantityThatMany | RuleTag::QuantityThatMuch => {
            let Features::Quantity(cardinality) = children.first()?.features else {
                return None;
            };
            Some(Features::Quantity(*cardinality))
        }
        RuleTag::DeterminerClosed => {
            let Features::Determiner {
                cardinality,
                article,
            } = children.first()?.features
            else {
                return None;
            };
            Some(Features::Determiner {
                cardinality: *cardinality,
                article: *article,
            })
        }
        RuleTag::DeterminerTarget => Some(Features::Determiner {
            cardinality: Cardinality::SingularCount,
            article: None,
        }),
        RuleTag::DeterminerQuantifiedTarget => {
            let Features::Quantity(cardinality) = children.first()?.features else {
                return None;
            };
            Some(Features::Determiner {
                cardinality: target_cardinality(*cardinality),
                article: None,
            })
        }
        RuleTag::DeterminerQuantity => {
            let Features::Quantity(cardinality) = children.first()?.features else {
                return None;
            };
            Some(Features::Determiner {
                cardinality: *cardinality,
                article: None,
            })
        }
        RuleTag::DeterminerPossessiveThisCard => {
            let Features::PossessiveThisCard { .. } = children.first()?.features else {
                return None;
            };
            Some(Features::Determiner {
                cardinality: Cardinality::Unconstrained,
                article: None,
            })
        }
        _ => None,
    }
}

const fn number_cardinality(is_one: bool) -> Cardinality {
    if is_one { Cardinality::SingularOrMass } else { Cardinality::PluralOrMass }
}

const fn target_cardinality(cardinality: Cardinality) -> Cardinality {
    match cardinality {
        Cardinality::SingularOrMass => Cardinality::SingularCount,
        Cardinality::PluralOrMass => Cardinality::PluralCount,
        other => other,
    }
}

fn reduce_nominal(tag: RuleTag, children: &[Child<'_, EnglishGrammar<'_, '_>>]) -> Option<Reduced> {
    match tag {
        RuleTag::Adjective | RuleTag::Noun => Some(propagate(children.first()?)),
        RuleTag::AdjectivePhraseFaceUp | RuleTag::AdjectivePhraseFaceDown => {
            Some(Features::Adjective {
                initial_sound: InitialSound::Consonant,
                comparison: AdjectiveComparisonState::NotComparative,
                card_orientation: true,
            })
        }
        RuleTag::AdjectivePhrase => {
            let Features::Adjective { .. } = children.first()?.features else {
                return None;
            };
            Some(children.first()?.features.clone())
        }
        RuleTag::AdjectivePhraseComparison => {
            let Features::Adjective {
                initial_sound,
                comparison: AdjectiveComparisonState::Pending,
                card_orientation,
            } = children.first()?.features
            else {
                return None;
            };
            Some(Features::Adjective {
                initial_sound: *initial_sound,
                comparison: AdjectiveComparisonState::Complete,
                card_orientation: *card_orientation,
            })
        }
        RuleTag::ComparisonStandard
        | RuleTag::ComparisonThan
        | RuleTag::ComparisonThanOrEqualTo => Some(Features::None),
        RuleTag::NominalNoun => {
            let child = children.first()?;
            let Features::Noun {
                form,
                initial_sound,
                temporal,
            } = child.features
            else {
                return None;
            };
            Some(Features::Nominal {
                form: *form,
                initial_sound: *initial_sound,
                determined: false,
                leading_recovery: false,
                attachment: NominalAttachmentPhase::Open,
                comparison: AdjectiveComparisonState::NotComparative,
                temporal: *temporal,
            })
        }
        RuleTag::NominalAdjective => {
            let Features::Adjective {
                initial_sound,
                comparison,
                card_orientation: false,
            } = children.first()?.features
            else {
                return None;
            };
            nominal_with_prefix(children.get(1)?, *initial_sound, false, *comparison)
        }
        RuleTag::NominalNounModifier => {
            let Features::Noun { initial_sound, .. } = children.first()?.features else {
                return None;
            };
            nominal_with_prefix(
                children.get(1)?,
                *initial_sound,
                false,
                AdjectiveComparisonState::NotComparative,
            )
        }
        RuleTag::NominalQuantityModifier => nominal_with_prefix(
            children.get(1)?,
            InitialSound::Consonant,
            false,
            AdjectiveComparisonState::NotComparative,
        ),
        RuleTag::NominalPowerToughnessModifier => nominal_with_prefix(
            children.get(1)?,
            InitialSound::Consonant,
            false,
            AdjectiveComparisonState::NotComparative,
        ),
        RuleTag::NominalDeterminer => {
            let Features::Determiner {
                cardinality,
                article,
            } = children.first()?.features
            else {
                return None;
            };
            let Features::Nominal {
                form,
                initial_sound,
                determined,
                attachment,
                comparison,
                temporal,
                ..
            } = children.get(1)?.features
            else {
                return None;
            };
            if *determined
                || !cardinality_accepts(*cardinality, *form)
                || !article_accepts(*article, *initial_sound)
            {
                return None;
            }
            Some(Features::Nominal {
                form: *form,
                initial_sound: *initial_sound,
                determined: true,
                leading_recovery: false,
                attachment: *attachment,
                comparison: *comparison,
                temporal: *temporal,
            })
        }
        RuleTag::NominalPrepositional => {
            let Features::Nominal {
                form,
                initial_sound,
                determined,
                leading_recovery,
                attachment,
                comparison,
                temporal,
            } = children.first()?.features
            else {
                return None;
            };
            let Features::PrepositionalPhrase {
                nominal_attachment: true,
            } = children.get(1)?.features
            else {
                return None;
            };
            if matches!(
                attachment,
                NominalAttachmentPhase::PostpositiveAdjective | NominalAttachmentPhase::Comparison
            ) {
                return None;
            }
            Some(Features::Nominal {
                form: *form,
                initial_sound: *initial_sound,
                determined: *determined,
                leading_recovery: *leading_recovery,
                attachment: NominalAttachmentPhase::Prepositional,
                comparison: *comparison,
                temporal: *temporal,
            })
        }
        RuleTag::NominalQuantityComplement | RuleTag::NominalRelative => {
            let Features::Nominal {
                form,
                initial_sound,
                determined,
                leading_recovery,
                attachment,
                comparison,
                temporal,
            } = children.first()?.features
            else {
                return None;
            };
            if matches!(
                attachment,
                NominalAttachmentPhase::PostpositiveAdjective | NominalAttachmentPhase::Comparison
            ) {
                return None;
            }
            if tag == RuleTag::NominalRelative
                && let Features::RelativeClause {
                    gap: RelativeGap::Subject,
                    antecedent_agreement: Some(agreement),
                } = children.get(1)?.features
                && agreement.number
                    != match form {
                        NounForm::Plural => Number::Plural,
                        NounForm::Singular | NounForm::Mass => Number::Singular,
                    }
            {
                return None;
            }
            Some(Features::Nominal {
                form: *form,
                initial_sound: *initial_sound,
                determined: *determined,
                leading_recovery: *leading_recovery,
                attachment: *attachment,
                comparison: *comparison,
                temporal: *temporal,
            })
        }
        RuleTag::NominalPostpositiveAdjective => {
            let Features::Adjective {
                card_orientation: false,
                ..
            } = children.get(1)?.features
            else {
                return None;
            };
            let Features::Nominal {
                form,
                initial_sound,
                determined,
                leading_recovery,
                attachment: NominalAttachmentPhase::Open,
                comparison: AdjectiveComparisonState::NotComparative,
                temporal,
            } = children.first()?.features
            else {
                return None;
            };
            Some(Features::Nominal {
                form: *form,
                initial_sound: *initial_sound,
                determined: *determined,
                leading_recovery: *leading_recovery,
                attachment: NominalAttachmentPhase::PostpositiveAdjective,
                comparison: AdjectiveComparisonState::NotComparative,
                temporal: *temporal,
            })
        }
        RuleTag::NominalComparison => {
            let Features::Nominal {
                form,
                initial_sound,
                determined,
                leading_recovery,
                attachment: NominalAttachmentPhase::Open | NominalAttachmentPhase::Prepositional,
                comparison: AdjectiveComparisonState::Pending,
                temporal,
            } = children.first()?.features
            else {
                return None;
            };
            Some(Features::Nominal {
                form: *form,
                initial_sound: *initial_sound,
                determined: *determined,
                leading_recovery: *leading_recovery,
                attachment: NominalAttachmentPhase::Comparison,
                comparison: AdjectiveComparisonState::Complete,
                temporal: *temporal,
            })
        }
        _ => None,
    }
}

fn reduce_phrase(tag: RuleTag, children: &[Child<'_, EnglishGrammar<'_, '_>>]) -> Option<Reduced> {
    match tag {
        RuleTag::NounPhraseNominal => {
            let Features::Nominal { form, temporal, .. } = children.first()?.features else {
                return None;
            };
            let agreement = Some(Agreement {
                person: Person::Third,
                number: match form {
                    NounForm::Plural => Number::Plural,
                    NounForm::Singular | NounForm::Mass => Number::Singular,
                },
            });
            Some(Features::NounPhrase {
                agreement,
                pronoun_case: None,
                temporal: *temporal,
            })
        }
        RuleTag::NounPhraseSubjectPronoun | RuleTag::NounPhraseObjectPronoun => {
            noun_phrase_from_pronoun(children.first()?)
        }
        RuleTag::NounPhraseReciprocal => noun_phrase_from_pronoun(children.first()?),
        RuleTag::NounPhraseQuantity => {
            let Features::Quantity(cardinality) = children.first()?.features else {
                return None;
            };
            Some(Features::NounPhrase {
                agreement: Some(Agreement {
                    person: Person::Third,
                    number: quantity_number(*cardinality),
                }),
                pronoun_case: None,
                temporal: false,
            })
        }
        RuleTag::NounPhraseThisCard | RuleTag::NounPhraseFullThisCard => {
            let Features::NounPhrase {
                agreement,
                pronoun_case,
                temporal,
            } = children.first()?.features
            else {
                return None;
            };
            Some(Features::NounPhrase {
                agreement: *agreement,
                pronoun_case: *pronoun_case,
                temporal: *temporal,
            })
        }
        RuleTag::NounPhrasePossessiveThisCard => {
            let Features::PossessiveThisCard { agreement } = children.first()?.features else {
                return None;
            };
            Some(Features::NounPhrase {
                agreement: Some(*agreement),
                pronoun_case: None,
                temporal: false,
            })
        }
        RuleTag::NounPhrasePartitive => {
            let Features::Quantity(cardinality) = children.first()?.features else {
                return None;
            };
            let number = quantity_number(*cardinality);
            Some(Features::NounPhrase {
                agreement: Some(Agreement {
                    person: Person::Third,
                    number,
                }),
                pronoun_case: None,
                temporal: false,
            })
        }
        RuleTag::NounPhraseCoordination | RuleTag::NounPhraseAdditiveCoordination => {
            let Features::NounPhrase {
                agreement: first_agreement,
                temporal: first_temporal,
                ..
            } = children.first()?.features
            else {
                return None;
            };
            let conjunction = if tag == RuleTag::NounPhraseAdditiveCoordination {
                crate::syntax::NounPhraseConjunction::Plus
            } else {
                let Features::Conjunction(conjunction) = children.get(1)?.features else {
                    return None;
                };
                match conjunction {
                    crate::syntax::PredicateConjunction::And => {
                        crate::syntax::NounPhraseConjunction::And
                    }
                    crate::syntax::PredicateConjunction::Or => {
                        crate::syntax::NounPhraseConjunction::Or
                    }
                    crate::syntax::PredicateConjunction::Then => return None,
                }
            };
            let Features::NounPhrase {
                agreement: next_agreement,
                temporal: next_temporal,
                ..
            } = children.get(2)?.features
            else {
                return None;
            };
            let agreement = match conjunction {
                crate::syntax::NounPhraseConjunction::And => Some(Agreement {
                    person: Person::Third,
                    number: Number::Plural,
                }),
                crate::syntax::NounPhraseConjunction::Or => *next_agreement,
                crate::syntax::NounPhraseConjunction::Plus => *first_agreement,
            };
            Some(Features::NounPhrase {
                agreement,
                pronoun_case: None,
                temporal: *first_temporal && *next_temporal,
            })
        }
        RuleTag::PrepositionalPhrase => {
            let Features::Preposition(preposition) = children.first()?.features else {
                return None;
            };
            let Features::PrepositionalObject { gerund } = children.get(1)?.features else {
                return None;
            };
            Some(Features::PrepositionalPhrase {
                nominal_attachment: !(*preposition == Preposition::By && *gerund),
            })
        }
        RuleTag::PrepositionalObject => Some(Features::PrepositionalObject {
            gerund: matches!(children.first()?.features, Features::GerundClause),
        }),
        _ => None,
    }
}

fn propagate(child: &Child<'_, EnglishGrammar<'_, '_>>) -> Reduced {
    child.features.clone()
}

fn nominal_with_prefix(
    nominal: &Child<'_, EnglishGrammar<'_, '_>>,
    initial_sound: InitialSound,
    leading_recovery: bool,
    prefix_comparison: AdjectiveComparisonState,
) -> Option<Reduced> {
    let Features::Nominal {
        form,
        determined,
        attachment,
        comparison,
        temporal,
        ..
    } = nominal.features
    else {
        return None;
    };
    if *determined {
        return None;
    }
    let comparison = match (prefix_comparison, *comparison) {
        (AdjectiveComparisonState::Pending, AdjectiveComparisonState::NotComparative) => {
            AdjectiveComparisonState::Pending
        }
        (AdjectiveComparisonState::Pending, _) => return None,
        (AdjectiveComparisonState::NotComparative | AdjectiveComparisonState::Complete, state) => {
            state
        }
    };
    Some(Features::Nominal {
        form: *form,
        initial_sound,
        determined: *determined,
        leading_recovery,
        attachment: *attachment,
        comparison,
        temporal: *temporal,
    })
}

fn noun_phrase_from_pronoun(child: &Child<'_, EnglishGrammar<'_, '_>>) -> Option<Reduced> {
    let Features::NounPhrase {
        agreement,
        pronoun_case,
        temporal,
    } = child.features
    else {
        return None;
    };
    Some(Features::NounPhrase {
        agreement: *agreement,
        pronoun_case: *pronoun_case,
        temporal: *temporal,
    })
}

const fn quantity_number(cardinality: Cardinality) -> Number {
    match cardinality {
        Cardinality::SingularCount | Cardinality::SingularOrMass => Number::Singular,
        Cardinality::PluralCount
        | Cardinality::Mass
        | Cardinality::PluralOrMass
        | Cardinality::Unconstrained => Number::Plural,
    }
}

fn cardinality_accepts(cardinality: Cardinality, form: NounForm) -> bool {
    match cardinality {
        Cardinality::SingularCount => form == NounForm::Singular,
        Cardinality::SingularOrMass => matches!(form, NounForm::Singular | NounForm::Mass),
        Cardinality::PluralCount => form == NounForm::Plural,
        Cardinality::Mass => form == NounForm::Mass,
        Cardinality::PluralOrMass => matches!(form, NounForm::Plural | NounForm::Mass),
        Cardinality::Unconstrained => true,
    }
}

fn article_accepts(article: Option<IndefiniteArticleKey>, sound: InitialSound) -> bool {
    match article {
        Some(IndefiniteArticleKey::A) => sound == InitialSound::Consonant,
        Some(IndefiniteArticleKey::An) => sound == InitialSound::Vowel,
        None => true,
    }
}

fn slot_agrees(slot: VerbSlot, agreement: Agreement) -> bool {
    matches!(
        slot,
        VerbSlot::Present { person, number } | VerbSlot::Past { person, number }
            if person == agreement.person && number == agreement.number
    )
}

type EnglishChart = ChartResult<Nonterminal, EnglishLexicalSlot, Features, MeaningKey>;
type EnglishForest = ParseForest<Nonterminal, EnglishLexicalSlot, Features, MeaningKey>;

#[derive(Debug)]
pub(crate) struct ParsedNonterminal {
    pub(crate) chart: EnglishChart,
    root: NodeId,
    best: BestParse,
    syntax: Lowered,
    recovery_mode: RecoveryMode,
}

impl ParsedNonterminal {
    pub(crate) fn noun_phrase(&self) -> Option<&NounPhrase> {
        match &self.syntax {
            Lowered::NounPhrase(noun_phrase) => Some(noun_phrase),
            _ => None,
        }
    }

    pub(crate) fn sentence(&self) -> Option<&crate::syntax::Sentence> {
        match &self.syntax {
            Lowered::Sentence(sentence) => Some(sentence),
            _ => None,
        }
    }

    fn simple_clause(&self) -> Option<&SimpleClause> {
        match &self.syntax {
            Lowered::SimpleClause(clause) => Some(clause),
            _ => None,
        }
    }

    pub(crate) fn clause(&self) -> Option<&Clause> {
        match &self.syntax {
            Lowered::Clause(clause) => Some(clause),
            _ => None,
        }
    }

    pub(crate) fn quantity(&self) -> Option<&Quantity> {
        match &self.syntax {
            Lowered::Quantity(quantity) => Some(quantity),
            _ => None,
        }
    }

    pub(crate) fn root_rule(&self) -> Option<usize> {
        let node = self.chart.forest.node(self.root);
        let alternative = self.best.alternative(self.root)?;
        node.alternatives
            .get(alternative)?
            .rule
            .map(crate::chart::RuleId::index)
    }

    pub(crate) fn root_tied_alternatives(&self) -> &[usize] {
        self.best.tied_alternatives(self.root)
    }

    pub(crate) const fn cost(&self) -> ParseCost {
        self.best.cost
    }

    pub(crate) const fn chart_stats(&self) -> ChartStats {
        self.chart.stats
    }

    pub(crate) fn forest_stats(&self) -> ForestStats {
        self.chart.forest.stats()
    }

    pub(crate) const fn recovery_mode(&self) -> RecoveryMode {
        self.recovery_mode
    }
}

#[derive(Debug)]
pub(crate) enum ParseNonterminalError {
    Grammar(GrammarError),
    NoCompleteParse(Nonterminal),
    Forest(ForestError),
    Lowering,
}

pub(crate) fn parse_nonterminal(
    source: &str,
    catalogs: &Catalogs,
    nonterminal: Nonterminal,
) -> Result<ParsedNonterminal, ParseNonterminalError> {
    let surface = lex(source);
    match parse_nonterminal_with_profile(
        source,
        catalogs,
        nonterminal,
        &surface.tokens,
        RecoveryProfile::Exact,
    ) {
        Ok(parsed) => Ok(parsed),
        Err(ParseNonterminalError::NoCompleteParse(_)) => parse_nonterminal_with_profile(
            source,
            catalogs,
            nonterminal,
            &surface.tokens,
            RecoveryProfile::Phrases,
        ),
        Err(error) => Err(error),
    }
}

fn parse_nonterminal_with_profile(
    source: &str,
    catalogs: &Catalogs,
    nonterminal: Nonterminal,
    tokens: &[Token],
    recovery_profile: RecoveryProfile,
) -> Result<ParsedNonterminal, ParseNonterminalError> {
    let grammar =
        EnglishGrammar::with_recovery_profile(source, catalogs, nonterminal, recovery_profile);
    let chart = parse_chart(&grammar, tokens).map_err(ParseNonterminalError::Grammar)?;
    let (root, best) = chart
        .forest
        .best_root(chart.roots.iter().copied())
        .map_err(ParseNonterminalError::Forest)?
        .ok_or(ParseNonterminalError::NoCompleteParse(nonterminal))?;
    let syntax =
        lower(&grammar, &chart.forest, root, &best).ok_or(ParseNonterminalError::Lowering)?;
    Ok(ParsedNonterminal {
        chart,
        root,
        best,
        syntax,
        recovery_mode: recovery_profile.mode(),
    })
}

#[derive(Debug)]
enum Lowered {
    Number(NumberKey),
    Quantity(Quantity),
    Determiner(Determiner),
    Adjective(Adjective),
    AdjectivePhrase(AdjectivePhrase),
    ComparisonComplement(ComparisonComplement),
    Noun(NounInstance),
    Nominal(NominalPhrase),
    PossessiveNominal(NominalPhrase),
    NounPhrase(NounPhrase),
    Catalog(crate::catalog::CatalogAtom),
    Adverb(Vocab),
    VerbParticle(VerbParticle),
    Frequency(FrequencyPhrase),
    Auxiliary(AuxiliaryInstance),
    SubjectAuxiliary(ContractedSubjectAuxiliary),
    OracleSymbol(OracleSymbol),
    PowerToughness(PowerToughness),
    Pronoun(PronounInstance),
    ThisCard(ThisCardForm),
    Preposition(Preposition),
    Phrase(Phrase),
    PrepositionalPhrase(PrepositionalPhrase),
    Verb(VerbInstance),
    VerbPhrase(VerbPhrase),
    InfinitiveClause(InfinitiveClause),
    GerundClause(GerundClause),
    SimpleClause(SimpleClause),
    EllipticalClause(crate::syntax::EllipticalClause),
    Clause(Clause),
    RelativeClause(RelativeClause),
    Sentence(Sentence),
    Conjunction(crate::syntax::PredicateConjunction),
    Subordinator(crate::syntax::Subordinator),
    Existential(ExistentialForm),
    Unknown(UnknownPhrase),
    Ignored,
}

fn lower(
    grammar: &EnglishGrammar<'_, '_>,
    forest: &EnglishForest,
    node: NodeId,
    best: &BestParse,
) -> Option<Lowered> {
    let forest_node = forest.node(node);
    match forest_node.key.symbol {
        ForestSymbol::Lexical(_) => {
            return lower_lexical(grammar, forest_node.key.lexical_value()?);
        }
        ForestSymbol::Intermediate { .. } => return None,
        ForestSymbol::Nonterminal(_) => {}
    }
    let alternative = forest_node.alternatives.get(best.alternative(node)?)?;
    let rule = alternative.rule?;
    let tag = *grammar.tags.get(rule.index())?;
    let [intermediate] = alternative.children.as_slice() else {
        return None;
    };
    let mut child_nodes = Vec::new();
    selected_rule_children(forest, *intermediate, best, &mut child_nodes)?;
    let mut children = child_nodes
        .iter()
        .map(|&child| lower(grammar, forest, child, best))
        .collect::<Option<Vec<_>>>()?;
    lower_rule(tag, &mut children)
}

fn selected_rule_children(
    forest: &EnglishForest,
    intermediate: NodeId,
    best: &BestParse,
    children: &mut Vec<NodeId>,
) -> Option<()> {
    let node = forest.node(intermediate);
    if !matches!(node.key.symbol, ForestSymbol::Intermediate { .. }) {
        return None;
    }
    let alternative = node.alternatives.get(best.alternative(intermediate)?)?;
    match alternative.children.as_slice() {
        [child] => children.push(*child),
        [previous, child] => {
            selected_rule_children(forest, *previous, best, children)?;
            children.push(*child);
        }
        _ => return None,
    }
    Some(())
}

fn lower_lexical(grammar: &EnglishGrammar<'_, '_>, meaning: &MeaningKey) -> Option<Lowered> {
    Some(match meaning {
        MeaningKey::Literal(_) | MeaningKey::Punctuation(_) => Lowered::Ignored,
        MeaningKey::Number(number) => Lowered::Number(*number),
        MeaningKey::Quantity(quantity) => Lowered::Quantity(quantity.syntax()),
        MeaningKey::Determiner(determiner) => Lowered::Determiner(determiner.syntax()?),
        MeaningKey::Noun(noun) => Lowered::Noun(noun.clone()),
        MeaningKey::Adjective(adjective) => Lowered::Adjective(adjective.clone()),
        MeaningKey::Adverb(adverb) => Lowered::Adverb(*adverb),
        MeaningKey::VerbParticle(particle) => Lowered::VerbParticle(*particle),
        MeaningKey::Frequency(frequency) => Lowered::Frequency(frequency.syntax()),
        MeaningKey::Pronoun(pronoun) => Lowered::Pronoun(*pronoun),
        MeaningKey::Auxiliary(auxiliary) => Lowered::Auxiliary(*auxiliary),
        MeaningKey::SubjectAuxiliary(subject_auxiliary) => {
            let subject = match subject_auxiliary.subject {
                ContractedSubjectKey::Pronoun(pronoun) => NounPhrase::Pronoun {
                    pronoun,
                    case: PronounCase::Subject,
                },
                ContractedSubjectKey::Demonstrative(demonstrative) => {
                    NounPhrase::Demonstrative(demonstrative)
                }
            };
            Lowered::SubjectAuxiliary(ContractedSubjectAuxiliary {
                subject: Subject(subject),
                auxiliary: subject_auxiliary.auxiliary,
            })
        }
        MeaningKey::Verb(verb) => Lowered::Verb(verb.clone()),
        MeaningKey::Catalog(atom) => Lowered::Catalog(atom.clone()),
        MeaningKey::OracleSymbol(symbol) => Lowered::OracleSymbol(symbol.clone()),
        MeaningKey::PowerToughness(power_toughness) => Lowered::PowerToughness(*power_toughness),
        MeaningKey::Conjunction(conjunction) => Lowered::Conjunction(*conjunction),
        MeaningKey::Subordinator(subordinator) => Lowered::Subordinator(*subordinator),
        MeaningKey::Existential(form) => Lowered::Existential(*form),
        MeaningKey::ThisCard(form) => Lowered::ThisCard(*form),
        MeaningKey::Preposition(preposition) => Lowered::Preposition(*preposition),
        MeaningKey::Unknown(key) => {
            let unknown = UnknownPhrase(key.span.text(grammar.source)?.to_owned());
            match key.slot {
                RecoverySlot::Noun(form) => {
                    let noun = Noun::Unknown(unknown);
                    Lowered::Noun(match form {
                        NounForm::Singular => NounInstance::Singular(noun),
                        NounForm::Plural => NounInstance::Plural(noun),
                        NounForm::Mass => NounInstance::Mass(noun),
                    })
                }
            }
        }
    })
}

fn lower_rule(tag: RuleTag, children: &mut [Lowered]) -> Option<Lowered> {
    match tag {
        RuleTag::QuantityExact
        | RuleTag::QuantityAtLeast
        | RuleTag::QuantityOr
        | RuleTag::QuantityX
        | RuleTag::QuantityBoth
        | RuleTag::QuantityUpTo
        | RuleTag::QuantityThatMany
        | RuleTag::QuantityThatMuch
        | RuleTag::QuantityMoreThan
        | RuleTag::QuantityFewerThan
        | RuleTag::DeterminerClosed
        | RuleTag::DeterminerTarget
        | RuleTag::DeterminerQuantifiedTarget
        | RuleTag::DeterminerQuantity
        | RuleTag::DeterminerPossessiveThisCard => lower_quantity_or_determiner(tag, children),
        RuleTag::FrequencyPhrase => take(children, 0),
        RuleTag::PossessiveNounBase
        | RuleTag::PossessiveNounDetermined
        | RuleTag::DeterminerPossessiveNoun => lower_possessive_noun_phrase(tag, children),
        RuleTag::Adjective
        | RuleTag::AdjectivePhrase
        | RuleTag::AdjectivePhraseFaceUp
        | RuleTag::AdjectivePhraseFaceDown
        | RuleTag::AdjectivePhraseComparison
        | RuleTag::ComparisonStandard
        | RuleTag::ComparisonThan
        | RuleTag::ComparisonThanOrEqualTo
        | RuleTag::Noun
        | RuleTag::NominalNoun
        | RuleTag::NominalAdjective
        | RuleTag::NominalNounModifier
        | RuleTag::NominalQuantityModifier
        | RuleTag::NominalPowerToughnessModifier
        | RuleTag::NominalDeterminer
        | RuleTag::NominalPrepositional
        | RuleTag::NominalQuantityComplement
        | RuleTag::NominalRelative
        | RuleTag::NominalPostpositiveAdjective
        | RuleTag::NominalComparison => lower_nominal(tag, children),
        RuleTag::NounPhraseNominal
        | RuleTag::NounPhraseSubjectPronoun
        | RuleTag::NounPhraseObjectPronoun
        | RuleTag::NounPhraseReciprocal
        | RuleTag::NounPhraseQuantity
        | RuleTag::NounPhraseThisCard
        | RuleTag::NounPhraseFullThisCard
        | RuleTag::NounPhrasePossessiveThisCard
        | RuleTag::NounPhrasePartitive
        | RuleTag::NounPhraseCoordination
        | RuleTag::NounPhraseAdditiveCoordination
        | RuleTag::PrepositionalPhrase
        | RuleTag::PrepositionalObject => lower_phrase(tag, children),
        RuleTag::Verb
        | RuleTag::VerbPhraseBase
        | RuleTag::VerbPhraseAuxiliary
        | RuleTag::VerbPhraseDirectObject
        | RuleTag::VerbPhraseAdjective
        | RuleTag::VerbPhrasePrepositional
        | RuleTag::VerbPhraseInfinitive
        | RuleTag::VerbPhraseAdverb
        | RuleTag::VerbPhraseParticle
        | RuleTag::VerbPhraseFrequency
        | RuleTag::VerbPhraseAbility
        | RuleTag::VerbPhraseOracleSymbol
        | RuleTag::VerbPhrasePowerToughness
        | RuleTag::VerbPhraseQuantity
        | RuleTag::InfinitiveTo
        | RuleTag::InfinitiveNotTo
        | RuleTag::GerundClauseBase
        | RuleTag::GerundClauseSubordinateAfter
        | RuleTag::SimpleClauseSubject
        | RuleTag::SimpleClauseContractedSubject
        | RuleTag::SimpleClauseSubjectless
        | RuleTag::ClauseSimple
        | RuleTag::ClauseElliptical
        | RuleTag::ClauseCoordination
        | RuleTag::ClauseCoordinationComma
        | RuleTag::ClauseSubordinateBefore
        | RuleTag::ClauseSubordinateAfterElliptical
        | RuleTag::ClauseSubordinateAfter
        | RuleTag::ClauseSubordinateAfterInfinitive
        | RuleTag::ClauseExistential
        | RuleTag::ClauseCopularNoun
        | RuleTag::ClauseCopularAdjective
        | RuleTag::ClauseCopularPrepositional
        | RuleTag::ClauseContractedCopularNoun
        | RuleTag::ClauseContractedCopularAdjective
        | RuleTag::ClauseContractedCopularPrepositional
        | RuleTag::RelativeObject
        | RuleTag::RelativeObjectContractedSubject
        | RuleTag::RelativeSubjectContractedAuxiliary
        | RuleTag::RelativeSubject
        | RuleTag::RelativeContractedCopularNoun
        | RuleTag::RelativeContractedCopularAdjective
        | RuleTag::RelativeContractedCopularPrepositional
        | RuleTag::SentencePeriod
        | RuleTag::SentenceExclamation
        | RuleTag::SentenceQuestion
        | RuleTag::SentenceNone => clause::lower_clause(tag, children),
        RuleTag::NounUnknown => recovery::lower_recovery(tag, children),
    }
}

fn lower_possessive_noun_phrase(tag: RuleTag, children: &mut [Lowered]) -> Option<Lowered> {
    match tag {
        RuleTag::PossessiveNounBase => {
            let Lowered::Noun(head) = take(children, 0)? else {
                return None;
            };
            Some(Lowered::PossessiveNominal(NominalPhrase {
                determiner: None,
                modifiers: Vec::new(),
                head,
                complements: Vec::new(),
            }))
        }
        RuleTag::PossessiveNounDetermined => {
            let Lowered::Determiner(determiner) = take(children, 0)? else {
                return None;
            };
            let Lowered::PossessiveNominal(mut nominal) = take(children, 1)? else {
                return None;
            };
            nominal.determiner = Some(determiner);
            Some(Lowered::PossessiveNominal(nominal))
        }
        RuleTag::DeterminerPossessiveNoun => {
            let Lowered::PossessiveNominal(possessor) = take(children, 0)? else {
                return None;
            };
            Some(Lowered::Determiner(Determiner::Possessive(
                Possessor::NounPhrase(Box::new(NounPhrase::Nominal(possessor))),
            )))
        }
        _ => None,
    }
}

fn lower_quantity_or_determiner(tag: RuleTag, children: &mut [Lowered]) -> Option<Lowered> {
    match tag {
        RuleTag::QuantityExact => {
            let Lowered::Number(number) = take(children, 0)? else {
                return None;
            };
            Some(Lowered::Quantity(Quantity::Exact(number.literal())))
        }
        RuleTag::QuantityAtLeast
        | RuleTag::QuantityOr
        | RuleTag::QuantityX
        | RuleTag::QuantityBoth
        | RuleTag::QuantityMoreThan
        | RuleTag::QuantityFewerThan => {
            let Lowered::Quantity(quantity) = take(children, 0)? else {
                return None;
            };
            Some(Lowered::Quantity(quantity))
        }
        RuleTag::QuantityUpTo => {
            let Lowered::Number(number) = take(children, 2)? else {
                return None;
            };
            Some(Lowered::Quantity(Quantity::UpTo(number.literal())))
        }
        RuleTag::QuantityThatMany => Some(Lowered::Quantity(Quantity::ThatMany)),
        RuleTag::QuantityThatMuch => Some(Lowered::Quantity(Quantity::ThatMuch)),
        RuleTag::DeterminerClosed => take(children, 0),
        RuleTag::DeterminerTarget => Some(Lowered::Determiner(Determiner::Target(None))),
        RuleTag::DeterminerQuantifiedTarget => {
            let Lowered::Quantity(quantity) = take(children, 0)? else {
                return None;
            };
            Some(Lowered::Determiner(Determiner::Target(Some(quantity))))
        }
        RuleTag::DeterminerQuantity => {
            let Lowered::Quantity(quantity) = take(children, 0)? else {
                return None;
            };
            Some(Lowered::Determiner(Determiner::Quantity(quantity)))
        }
        RuleTag::DeterminerPossessiveThisCard => {
            let Lowered::ThisCard(form) = take(children, 0)? else {
                return None;
            };
            Some(Lowered::Determiner(Determiner::Possessive(
                Possessor::NounPhrase(Box::new(NounPhrase::ThisCard(form))),
            )))
        }
        _ => None,
    }
}

fn lower_nominal(tag: RuleTag, children: &mut [Lowered]) -> Option<Lowered> {
    match tag {
        RuleTag::Adjective | RuleTag::Noun => take(children, 0),
        RuleTag::AdjectivePhraseFaceUp | RuleTag::AdjectivePhraseFaceDown => {
            let orientation = match tag {
                RuleTag::AdjectivePhraseFaceUp => CardOrientation::FaceUp,
                RuleTag::AdjectivePhraseFaceDown => CardOrientation::FaceDown,
                _ => return None,
            };
            Some(Lowered::AdjectivePhrase(AdjectivePhrase {
                head: Adjective::CardOrientation(orientation),
                complements: Vec::new(),
            }))
        }
        RuleTag::AdjectivePhrase => {
            let Lowered::Adjective(head) = take(children, 0)? else {
                return None;
            };
            Some(Lowered::AdjectivePhrase(AdjectivePhrase {
                head,
                complements: Vec::new(),
            }))
        }
        RuleTag::AdjectivePhraseComparison => {
            let Lowered::Adjective(head) = take(children, 0)? else {
                return None;
            };
            let Lowered::ComparisonComplement(comparison) = take(children, 1)? else {
                return None;
            };
            Some(Lowered::AdjectivePhrase(AdjectivePhrase {
                head,
                complements: vec![crate::syntax::AdjectiveComplement::Comparison(comparison)],
            }))
        }
        RuleTag::ComparisonStandard => {
            let standard = match take(children, 0)? {
                Lowered::Clause(clause) => Phrase::Clause(Box::new(clause)),
                Lowered::NounPhrase(noun_phrase) => Phrase::NounPhrase(Box::new(noun_phrase)),
                Lowered::AdjectivePhrase(adjective) => Phrase::AdjectivePhrase(Box::new(adjective)),
                _ => return None,
            };
            Some(Lowered::Phrase(standard))
        }
        RuleTag::ComparisonThan | RuleTag::ComparisonThanOrEqualTo => {
            let standard_index = if tag == RuleTag::ComparisonThan { 1 } else { 2 };
            let Lowered::Phrase(standard) = take(children, standard_index)? else {
                return None;
            };
            let marker = if tag == RuleTag::ComparisonThan {
                ComparisonMarker::Than
            } else {
                ComparisonMarker::ThanOrEqualTo
            };
            Some(Lowered::ComparisonComplement(ComparisonComplement {
                marker,
                standard: Box::new(standard),
            }))
        }
        RuleTag::NominalNoun => {
            let Lowered::Noun(head) = take(children, 0)? else {
                return None;
            };
            Some(Lowered::Nominal(NominalPhrase {
                determiner: None,
                modifiers: Vec::new(),
                head,
                complements: Vec::new(),
            }))
        }
        RuleTag::NominalAdjective => {
            let Lowered::AdjectivePhrase(adjective) = take(children, 0)? else {
                return None;
            };
            let Lowered::Nominal(mut nominal) = take(children, 1)? else {
                return None;
            };
            nominal
                .modifiers
                .insert(0, NominalModifier::Adjective(adjective));
            Some(Lowered::Nominal(nominal))
        }
        RuleTag::NominalNounModifier => {
            let Lowered::Noun(noun) = take(children, 0)? else {
                return None;
            };
            let Lowered::Nominal(mut nominal) = take(children, 1)? else {
                return None;
            };
            nominal.modifiers.insert(0, NominalModifier::Noun(noun));
            Some(Lowered::Nominal(nominal))
        }
        RuleTag::NominalQuantityModifier => {
            let Lowered::Quantity(quantity) = take(children, 0)? else {
                return None;
            };
            let Lowered::Nominal(mut nominal) = take(children, 1)? else {
                return None;
            };
            nominal
                .modifiers
                .insert(0, NominalModifier::Quantity(quantity));
            Some(Lowered::Nominal(nominal))
        }
        RuleTag::NominalPowerToughnessModifier => {
            let Lowered::PowerToughness(modifier) = take(children, 0)? else {
                return None;
            };
            let Lowered::Nominal(mut nominal) = take(children, 1)? else {
                return None;
            };
            nominal
                .modifiers
                .insert(0, NominalModifier::PowerToughness(modifier));
            Some(Lowered::Nominal(nominal))
        }
        RuleTag::NominalDeterminer => {
            let Lowered::Determiner(determiner) = take(children, 0)? else {
                return None;
            };
            let Lowered::Nominal(mut nominal) = take(children, 1)? else {
                return None;
            };
            nominal.determiner = Some(determiner);
            Some(Lowered::Nominal(nominal))
        }
        RuleTag::NominalPrepositional => {
            let Lowered::Nominal(mut nominal) = take(children, 0)? else {
                return None;
            };
            let Lowered::PrepositionalPhrase(preposition) = take(children, 1)? else {
                return None;
            };
            nominal
                .complements
                .push(NominalComplement::Prepositional(preposition));
            Some(Lowered::Nominal(nominal))
        }
        RuleTag::NominalQuantityComplement => {
            let Lowered::Nominal(mut nominal) = take(children, 0)? else {
                return None;
            };
            let Lowered::Quantity(quantity) = take(children, 1)? else {
                return None;
            };
            nominal
                .complements
                .push(NominalComplement::Quantity(quantity));
            Some(Lowered::Nominal(nominal))
        }
        RuleTag::NominalRelative => {
            let Lowered::Nominal(mut nominal) = take(children, 0)? else {
                return None;
            };
            let Lowered::RelativeClause(relative) = take(children, 1)? else {
                return None;
            };
            nominal
                .complements
                .push(NominalComplement::Relative(relative));
            Some(Lowered::Nominal(nominal))
        }
        RuleTag::NominalPostpositiveAdjective => {
            let Lowered::Nominal(mut nominal) = take(children, 0)? else {
                return None;
            };
            let Lowered::AdjectivePhrase(adjective) = take(children, 1)? else {
                return None;
            };
            nominal
                .complements
                .push(NominalComplement::Adjective(adjective));
            Some(Lowered::Nominal(nominal))
        }
        RuleTag::NominalComparison => {
            let Lowered::Nominal(mut nominal) = take(children, 0)? else {
                return None;
            };
            let Lowered::ComparisonComplement(comparison) = take(children, 1)? else {
                return None;
            };
            let adjective = nominal.modifiers.iter_mut().rev().find_map(|modifier| {
                let NominalModifier::Adjective(adjective) = modifier else {
                    return None;
                };
                (adjective_comparison_state(&adjective.head) == AdjectiveComparisonState::Pending
                    && !adjective.complements.iter().any(|complement| {
                        matches!(
                            complement,
                            crate::syntax::AdjectiveComplement::Comparison(_)
                                | crate::syntax::AdjectiveComplement::PostnominalComparison(_)
                        )
                    }))
                .then_some(adjective)
            })?;
            adjective
                .complements
                .push(crate::syntax::AdjectiveComplement::PostnominalComparison(
                    comparison,
                ));
            Some(Lowered::Nominal(nominal))
        }
        _ => None,
    }
}

fn lower_phrase(tag: RuleTag, children: &mut [Lowered]) -> Option<Lowered> {
    match tag {
        RuleTag::NounPhraseNominal => {
            let Lowered::Nominal(nominal) = take(children, 0)? else {
                return None;
            };
            Some(Lowered::NounPhrase(NounPhrase::Nominal(nominal)))
        }
        RuleTag::NounPhraseSubjectPronoun
        | RuleTag::NounPhraseObjectPronoun
        | RuleTag::NounPhraseReciprocal => {
            let Lowered::Pronoun(pronoun) = take(children, 0)? else {
                return None;
            };
            Some(Lowered::NounPhrase(NounPhrase::Pronoun {
                pronoun: pronoun.pronoun,
                case: pronoun.case,
            }))
        }
        RuleTag::NounPhraseQuantity => {
            let Lowered::Quantity(quantity) = take(children, 0)? else {
                return None;
            };
            Some(Lowered::NounPhrase(NounPhrase::Quantity(quantity)))
        }
        RuleTag::NounPhraseThisCard | RuleTag::NounPhraseFullThisCard => {
            let Lowered::ThisCard(form) = take(children, 0)? else {
                return None;
            };
            Some(Lowered::NounPhrase(NounPhrase::ThisCard(form)))
        }
        RuleTag::NounPhrasePossessiveThisCard => {
            let Lowered::ThisCard(form) = take(children, 0)? else {
                return None;
            };
            Some(Lowered::NounPhrase(NounPhrase::Possessive(
                Possessor::NounPhrase(Box::new(NounPhrase::ThisCard(form))),
            )))
        }
        RuleTag::NounPhrasePartitive => {
            let Lowered::Quantity(quantity) = take(children, 0)? else {
                return None;
            };
            let Lowered::Preposition(Preposition::Of) = take(children, 1)? else {
                return None;
            };
            let Lowered::NounPhrase(whole) = take(children, 2)? else {
                return None;
            };
            Some(Lowered::NounPhrase(NounPhrase::Partitive(
                crate::syntax::PartitiveNounPhrase {
                    quantity,
                    whole: Box::new(whole),
                },
            )))
        }
        RuleTag::NounPhraseCoordination | RuleTag::NounPhraseAdditiveCoordination => {
            let Lowered::NounPhrase(first) = take(children, 0)? else {
                return None;
            };
            let conjunction = if tag == RuleTag::NounPhraseAdditiveCoordination {
                crate::syntax::NounPhraseConjunction::Plus
            } else {
                let Lowered::Conjunction(conjunction) = take(children, 1)? else {
                    return None;
                };
                match conjunction {
                    crate::syntax::PredicateConjunction::And => {
                        crate::syntax::NounPhraseConjunction::And
                    }
                    crate::syntax::PredicateConjunction::Or => {
                        crate::syntax::NounPhraseConjunction::Or
                    }
                    crate::syntax::PredicateConjunction::Then => return None,
                }
            };
            let Lowered::NounPhrase(next) = take(children, 2)? else {
                return None;
            };
            let coordination = crate::syntax::NounPhraseCoordination {
                conjunction,
                phrase: next,
            };
            let coordinated = match first {
                NounPhrase::Coordinated(mut coordinated) => {
                    coordinated.rest.push(coordination);
                    coordinated
                }
                first => crate::syntax::CoordinatedNounPhrase {
                    first: Box::new(first),
                    rest: vec![coordination],
                },
            };
            Some(Lowered::NounPhrase(NounPhrase::Coordinated(coordinated)))
        }
        RuleTag::PrepositionalObject => {
            let phrase = match take(children, 0)? {
                Lowered::NounPhrase(object) => Phrase::NounPhrase(Box::new(object)),
                Lowered::PrepositionalPhrase(object) => {
                    Phrase::PrepositionalPhrase(Box::new(object))
                }
                Lowered::GerundClause(object) => Phrase::Clause(Box::new(Clause::Dependent(
                    crate::syntax::DependentClause::Gerund(object),
                ))),
                Lowered::Adverb(object) => Phrase::Adverb(object),
                _ => return None,
            };
            Some(Lowered::Phrase(phrase))
        }
        RuleTag::PrepositionalPhrase => {
            let Lowered::Preposition(preposition) = take(children, 0)? else {
                return None;
            };
            let Lowered::Phrase(object) = take(children, 1)? else {
                return None;
            };
            Some(Lowered::PrepositionalPhrase(PrepositionalPhrase {
                preposition,
                object: Box::new(object),
            }))
        }
        _ => None,
    }
}

fn take(children: &mut [Lowered], index: usize) -> Option<Lowered> {
    let child = children.get_mut(index)?;
    Some(std::mem::replace(child, Lowered::Ignored))
}
