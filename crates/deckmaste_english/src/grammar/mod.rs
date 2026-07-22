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
use crate::syntax::Copula;
use crate::syntax::Demonstrative;
use crate::syntax::Determiner;
use crate::syntax::ExistentialForm;
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
use crate::word::Adjective;
use crate::word::Auxiliary;
use crate::word::AuxiliaryInflection;
use crate::word::AuxiliaryInstance;
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
    Infinitive(InfinitiveClause),
    Subordinate(Box<Clause>),
    Adverbial(Phrase),
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct InfinitiveClause {
    marker: InfinitiveMarker,
    predicate: Box<VerbPhrase>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct SimpleClause {
    subject: Option<Subject>,
    predicate: VerbPhrase,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct ContractedSubjectCopula {
    subject: Subject,
    copula: Copula,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub(crate) enum Nonterminal {
    Quantity,
    Determiner,
    Adjective,
    AdjectivePhrase,
    Noun,
    Nominal,
    NounPhrase,
    PossessiveNounPhrase,
    PrepositionalPhrase,
    RelativeClause,
    Verb,
    VerbPhrase,
    ObjectGapVerbPhrase,
    InfinitiveClause,
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
    Pronoun(PronounCase),
    Auxiliary,
    AbilityItem,
    AbilityWord,
    Determiner,
    DeterminerTarget,
    QuantityAtLeast,
    QuantityThatMany,
    QuantityThatMuch,
    Up,
    To,
    Reciprocal,
    ThisCard,
    FullThisCard,
    Preposition,
    OracleSymbol,
    PowerToughness,
    Punctuation(Punctuation),
    Subordinator,
    Conjunction,
    Existential,
    Copula,
    SubjectCopula,
    Unknown(RecoverySlot),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub(crate) enum RecoveryMode {
    Exact,
    UnknownPhrases,
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
    },
    Noun {
        form: NounForm,
        initial_sound: InitialSound,
    },
    Nominal {
        form: NounForm,
        initial_sound: InitialSound,
        determined: bool,
        leading_recovery: bool,
    },
    NounPhrase {
        agreement: Option<Agreement>,
        pronoun_case: Option<PronounCase>,
    },
    PossessiveNounPhrase {
        form: NounForm,
        initial_sound: InitialSound,
        determined: bool,
    },
    Verb {
        slot: VerbSlot,
        accepts_direct_object: bool,
    },
    VerbPhrase {
        form: PredicateForm,
        object: PredicateObjectState,
        phase: PredicateAttachmentPhase,
        accepts_direct_object: bool,
    },
    InfinitiveClause,
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
    PrepositionalPhrase,
    RelativeClause(RelativeGap),
    Auxiliary(AuxiliaryInstance),
    Conjunction(crate::syntax::PredicateConjunction),
    Existential {
        number: Number,
    },
    Copula(Agreement),
    SubjectCopula(Agreement),
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
    UpTo(NumberKey),
    ThatMany,
    ThatMuch,
}

impl QuantityKey {
    const fn cardinality(self) -> Cardinality {
        match self {
            Self::Exact(number) | Self::UpTo(number) if number.value == 1 => {
                Cardinality::SingularOrMass
            }
            Self::Exact(_) | Self::AtLeast(_) | Self::UpTo(_) | Self::ThatMany => {
                Cardinality::PluralCount
            }
            Self::ThatMuch => Cardinality::Mass,
        }
    }

    const fn syntax(self) -> Quantity {
        match self {
            Self::Exact(number) => Quantity::Exact(number.literal()),
            Self::AtLeast(number) => Quantity::AtLeast(number.literal()),
            Self::UpTo(number) => Quantity::UpTo(number.literal()),
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
            Self::Target(Some(QuantityKey::Exact(number) | QuantityKey::UpTo(number)))
                if number.value == 1 =>
            {
                Cardinality::SingularCount
            }
            Self::Target(Some(
                QuantityKey::Exact(_)
                | QuantityKey::AtLeast(_)
                | QuantityKey::UpTo(_)
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
    Up,
    To,
    Target,
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
    SubjectCopula(SubjectCopulaKey),
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
pub(crate) struct SubjectCopulaKey {
    pronoun: Pronoun,
    auxiliary: AuxiliaryInstance,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum RuleTag {
    QuantityExact,
    QuantityAtLeast,
    QuantityUpTo,
    QuantityThatMany,
    QuantityThatMuch,
    DeterminerClosed,
    DeterminerTarget,
    DeterminerQuantifiedTarget,
    DeterminerQuantity,
    PossessiveNounBase,
    PossessiveNounDetermined,
    DeterminerPossessiveNoun,
    Adjective,
    AdjectivePhrase,
    Noun,
    NominalNoun,
    NominalAdjective,
    NominalNounModifier,
    NominalPowerToughnessModifier,
    NominalDeterminer,
    NominalPrepositional,
    NominalRelative,
    NounPhraseNominal,
    NounPhraseSubjectPronoun,
    NounPhraseObjectPronoun,
    NounPhraseReciprocal,
    NounPhraseThisCard,
    NounPhraseFullThisCard,
    NounPhraseCoordination,
    PrepositionalPhrase,
    Verb,
    VerbPhraseBase,
    VerbPhraseAuxiliary,
    VerbPhraseDirectObject,
    VerbPhraseAdjective,
    VerbPhrasePrepositional,
    VerbPhraseInfinitive,
    VerbPhraseAdverb,
    VerbPhraseAbility,
    VerbPhraseOracleSymbol,
    VerbPhrasePowerToughness,
    VerbPhraseQuantity,
    InfinitiveTo,
    SimpleClauseSubject,
    SimpleClauseSubjectless,
    ClauseSimple,
    ClauseElliptical,
    ClauseCoordination,
    ClauseCoordinationComma,
    ClauseSubordinateBefore,
    ClauseSubordinateAfterElliptical,
    ClauseSubordinateAfter,
    ClauseExistential,
    ClauseCopularNoun,
    ClauseCopularAdjective,
    ClauseCopularPrepositional,
    ClauseContractedCopularNoun,
    ClauseContractedCopularAdjective,
    ClauseContractedCopularPrepositional,
    RelativeObject,
    SentencePeriod,
    SentenceExclamation,
    SentenceQuestion,
    SentenceNone,
    NounUnknown,
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
                        local_cost: ParseCost::default(),
                    })
                    .into_iter()
                    .collect()
            }
            EnglishLexicalSlot::Noun(usage) => {
                let mut matches = self.word_matches(tokens, start, LexicalSlot::Noun(usage));
                matches.extend(self.catalog_matches(tokens, start, CatalogSlot::Noun(usage)));
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
            EnglishLexicalSlot::SubjectCopula => self.scan_subject_copula(tokens, start),
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
            EnglishLexicalSlot::Up => self
                .one_token_match(tokens, start, "up")
                .map(|end| literal_match(end, LiteralKey::Up))
                .into_iter()
                .collect(),
            EnglishLexicalSlot::To => self
                .one_token_match(tokens, start, "to")
                .map(|end| literal_match(end, LiteralKey::To))
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
            EnglishLexicalSlot::Preposition => self.scan_preposition(tokens, start),
            EnglishLexicalSlot::Existential => self.scan_existential(tokens, start),
            slot @ (EnglishLexicalSlot::PossessiveNoun
            | EnglishLexicalSlot::OracleSymbol
            | EnglishLexicalSlot::PowerToughness
            | EnglishLexicalSlot::Punctuation(_)
            | EnglishLexicalSlot::Subordinator
            | EnglishLexicalSlot::Conjunction) => self.scan_clause_lexical(slot, tokens, start),
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
            EnglishLexicalSlot::Conjunction => self.scan_conjunction(tokens, start),
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

    fn scan_subject_copula(
        &self,
        tokens: &[Token],
        start: usize,
    ) -> Vec<LexicalMatch<Features, MeaningKey>> {
        [
            ("you're", Pronoun::You, Person::Second, Number::Singular),
            (
                "he's",
                Pronoun::It(crate::word::Gender::Masculine),
                Person::Third,
                Number::Singular,
            ),
            (
                "she's",
                Pronoun::It(crate::word::Gender::Feminine),
                Person::Third,
                Number::Singular,
            ),
            (
                "it's",
                Pronoun::It(crate::word::Gender::Neuter),
                Person::Third,
                Number::Singular,
            ),
            ("they're", Pronoun::They, Person::Third, Number::Plural),
        ]
        .into_iter()
        .filter_map(|(surface, pronoun, person, number)| {
            self.one_token_match(tokens, start, surface).map(|end| {
                let auxiliary = AuxiliaryInstance {
                    auxiliary: Auxiliary::Be,
                    inflection: AuxiliaryInflection::Present { person, number },
                    contracted_negation: false,
                };
                LexicalMatch {
                    end,
                    features: Features::SubjectCopula(Agreement { person, number }),
                    meaning: MeaningKey::SubjectCopula(SubjectCopulaKey { pronoun, auxiliary }),
                    local_cost: ParseCost::default(),
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
        let Some(surface) = self.token_text(tokens, start) else {
            return Vec::new();
        };
        let Some(end) = self.words_match(tokens, start + 1, &["or", "more"]) else {
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
                quantity_match(end, QuantityKey::AtLeast(NumberKey { value, notation }))
            })
        })
        .collect()
    }

    fn scan_preposition(
        &self,
        tokens: &[Token],
        start: usize,
    ) -> Vec<LexicalMatch<Features, MeaningKey>> {
        let Some(surface) = self.token_text(tokens, start) else {
            return Vec::new();
        };
        let preposition = if surface.eq_ignore_ascii_case("at") {
            Preposition::At
        } else if surface.eq_ignore_ascii_case("by") {
            Preposition::By
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
            features: Features::None,
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
        self.add(
            RuleTag::NominalRelative,
            N::Nominal,
            [n(N::Nominal), n(N::RelativeClause)],
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
        self.add(RuleTag::NounPhraseThisCard, N::NounPhrase, [l(L::ThisCard)]);
        self.add(
            RuleTag::NounPhraseFullThisCard,
            N::NounPhrase,
            [l(L::FullThisCard)],
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

        self.add(
            RuleTag::PrepositionalPhrase,
            N::PrepositionalPhrase,
            [l(L::Preposition), n(N::NounPhrase)],
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
            (
                Features::Noun {
                    form,
                    initial_sound,
                },
                MeaningKey::Noun(noun),
            )
        }
        WordMatch::Verb(verb) => {
            let accepts_direct_object = !matches!(verb.verb, crate::word::Verb::Word(Vocab::Be));
            (
                Features::Verb {
                    slot: verb.slot,
                    accepts_direct_object,
                },
                MeaningKey::Verb(verb),
            )
        }
        WordMatch::Adjective(adjective) => (
            Features::Adjective {
                initial_sound: adjective_initial_sound(&adjective)?,
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

fn noun_initial_sound(noun: &NounInstance) -> Option<InitialSound> {
    let noun_identity = match noun {
        NounInstance::Singular(noun) | NounInstance::Plural(noun) | NounInstance::Mass(noun) => {
            noun
        }
    };
    match noun_identity {
        Noun::Word(vocab) => Some(Vocabulary::new().initial_sound(*vocab)),
        Noun::Catalog(atom) => Some(surface_initial_sound(atom.canonical())),
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
        | RuleTag::QuantityUpTo
        | RuleTag::QuantityThatMany
        | RuleTag::QuantityThatMuch
        | RuleTag::DeterminerClosed
        | RuleTag::DeterminerTarget
        | RuleTag::DeterminerQuantifiedTarget
        | RuleTag::DeterminerQuantity => reduce_quantity_or_determiner(tag, children)?,
        RuleTag::PossessiveNounBase
        | RuleTag::PossessiveNounDetermined
        | RuleTag::DeterminerPossessiveNoun => reduce_possessive_noun_phrase(tag, children)?,
        RuleTag::Adjective
        | RuleTag::AdjectivePhrase
        | RuleTag::Noun
        | RuleTag::NominalNoun
        | RuleTag::NominalAdjective
        | RuleTag::NominalNounModifier
        | RuleTag::NominalPowerToughnessModifier
        | RuleTag::NominalDeterminer
        | RuleTag::NominalPrepositional
        | RuleTag::NominalRelative => reduce_nominal(tag, children)?,
        RuleTag::NounPhraseNominal
        | RuleTag::NounPhraseSubjectPronoun
        | RuleTag::NounPhraseObjectPronoun
        | RuleTag::NounPhraseReciprocal
        | RuleTag::NounPhraseThisCard
        | RuleTag::NounPhraseFullThisCard
        | RuleTag::NounPhraseCoordination
        | RuleTag::PrepositionalPhrase => reduce_phrase(tag, children)?,
        RuleTag::Verb
        | RuleTag::VerbPhraseBase
        | RuleTag::VerbPhraseAuxiliary
        | RuleTag::VerbPhraseDirectObject
        | RuleTag::VerbPhraseAdjective
        | RuleTag::VerbPhrasePrepositional
        | RuleTag::VerbPhraseInfinitive
        | RuleTag::VerbPhraseAdverb
        | RuleTag::VerbPhraseAbility
        | RuleTag::VerbPhraseOracleSymbol
        | RuleTag::VerbPhrasePowerToughness
        | RuleTag::VerbPhraseQuantity
        | RuleTag::InfinitiveTo
        | RuleTag::SimpleClauseSubject
        | RuleTag::SimpleClauseSubjectless
        | RuleTag::ClauseSimple
        | RuleTag::ClauseElliptical
        | RuleTag::ClauseCoordination
        | RuleTag::ClauseCoordinationComma
        | RuleTag::ClauseSubordinateBefore
        | RuleTag::ClauseSubordinateAfterElliptical
        | RuleTag::ClauseSubordinateAfter
        | RuleTag::ClauseExistential
        | RuleTag::ClauseCopularNoun
        | RuleTag::ClauseCopularAdjective
        | RuleTag::ClauseCopularPrepositional
        | RuleTag::ClauseContractedCopularNoun
        | RuleTag::ClauseContractedCopularAdjective
        | RuleTag::ClauseContractedCopularPrepositional
        | RuleTag::RelativeObject
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
        RuleTag::QuantityAtLeast => {
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
        _ => None,
    }
}

const fn number_cardinality(is_one: bool) -> Cardinality {
    if is_one { Cardinality::SingularOrMass } else { Cardinality::PluralCount }
}

const fn target_cardinality(cardinality: Cardinality) -> Cardinality {
    match cardinality {
        Cardinality::SingularOrMass => Cardinality::SingularCount,
        other => other,
    }
}

fn reduce_nominal(tag: RuleTag, children: &[Child<'_, EnglishGrammar<'_, '_>>]) -> Option<Reduced> {
    match tag {
        RuleTag::Adjective | RuleTag::Noun => Some(propagate(children.first()?)),
        RuleTag::AdjectivePhrase => {
            let Features::Adjective { .. } = children.first()?.features else {
                return None;
            };
            Some(children.first()?.features.clone())
        }
        RuleTag::NominalNoun => {
            let child = children.first()?;
            let Features::Noun {
                form,
                initial_sound,
            } = child.features
            else {
                return None;
            };
            Some(Features::Nominal {
                form: *form,
                initial_sound: *initial_sound,
                determined: false,
                leading_recovery: false,
            })
        }
        RuleTag::NominalAdjective => {
            let Features::Adjective { initial_sound } = children.first()?.features else {
                return None;
            };
            nominal_with_prefix(children.get(1)?, *initial_sound, false)
        }
        RuleTag::NominalNounModifier => {
            let Features::Noun { initial_sound, .. } = children.first()?.features else {
                return None;
            };
            nominal_with_prefix(children.get(1)?, *initial_sound, false)
        }
        RuleTag::NominalPowerToughnessModifier => {
            nominal_with_prefix(children.get(1)?, InitialSound::Consonant, false)
        }
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
            })
        }
        RuleTag::NominalPrepositional | RuleTag::NominalRelative => {
            let Features::Nominal {
                form,
                initial_sound,
                determined,
                leading_recovery,
            } = children.first()?.features
            else {
                return None;
            };
            Some(Features::Nominal {
                form: *form,
                initial_sound: *initial_sound,
                determined: *determined,
                leading_recovery: *leading_recovery,
            })
        }
        _ => None,
    }
}

fn reduce_phrase(tag: RuleTag, children: &[Child<'_, EnglishGrammar<'_, '_>>]) -> Option<Reduced> {
    match tag {
        RuleTag::NounPhraseNominal => {
            let Features::Nominal { form, .. } = children.first()?.features else {
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
            })
        }
        RuleTag::NounPhraseSubjectPronoun | RuleTag::NounPhraseObjectPronoun => {
            noun_phrase_from_pronoun(children.first()?)
        }
        RuleTag::NounPhraseReciprocal => noun_phrase_from_pronoun(children.first()?),
        RuleTag::NounPhraseThisCard | RuleTag::NounPhraseFullThisCard => {
            let Features::NounPhrase {
                agreement,
                pronoun_case,
            } = children.first()?.features
            else {
                return None;
            };
            Some(Features::NounPhrase {
                agreement: *agreement,
                pronoun_case: *pronoun_case,
            })
        }
        RuleTag::NounPhraseCoordination => {
            let Features::Conjunction(conjunction) = children.get(1)?.features else {
                return None;
            };
            if *conjunction == crate::syntax::PredicateConjunction::Then {
                return None;
            }
            let agreement = match conjunction {
                crate::syntax::PredicateConjunction::And => Some(Agreement {
                    person: Person::Third,
                    number: Number::Plural,
                }),
                crate::syntax::PredicateConjunction::Or => {
                    let Features::NounPhrase { agreement, .. } = children.get(2)?.features else {
                        return None;
                    };
                    *agreement
                }
                crate::syntax::PredicateConjunction::Then => return None,
            };
            Some(Features::NounPhrase {
                agreement,
                pronoun_case: None,
            })
        }
        RuleTag::PrepositionalPhrase => Some(Features::PrepositionalPhrase),
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
) -> Option<Reduced> {
    let Features::Nominal {
        form, determined, ..
    } = nominal.features
    else {
        return None;
    };
    if *determined {
        return None;
    }
    Some(Features::Nominal {
        form: *form,
        initial_sound,
        determined: *determined,
        leading_recovery,
    })
}

fn noun_phrase_from_pronoun(child: &Child<'_, EnglishGrammar<'_, '_>>) -> Option<Reduced> {
    let Features::NounPhrase {
        agreement,
        pronoun_case,
    } = child.features
    else {
        return None;
    };
    Some(Features::NounPhrase {
        agreement: *agreement,
        pronoun_case: *pronoun_case,
    })
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
    Noun(NounInstance),
    Nominal(NominalPhrase),
    PossessiveNominal(NominalPhrase),
    NounPhrase(NounPhrase),
    Catalog(crate::catalog::CatalogAtom),
    Adverb(Vocab),
    Auxiliary(AuxiliaryInstance),
    SubjectCopula(ContractedSubjectCopula),
    OracleSymbol(OracleSymbol),
    PowerToughness(PowerToughness),
    Pronoun(PronounInstance),
    ThisCard(ThisCardForm),
    Preposition(Preposition),
    PrepositionalPhrase(PrepositionalPhrase),
    Verb(VerbInstance),
    VerbPhrase(VerbPhrase),
    InfinitiveClause(InfinitiveClause),
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
        MeaningKey::Pronoun(pronoun) => Lowered::Pronoun(*pronoun),
        MeaningKey::Auxiliary(auxiliary) => Lowered::Auxiliary(*auxiliary),
        MeaningKey::SubjectCopula(subject_copula) => {
            Lowered::SubjectCopula(ContractedSubjectCopula {
                subject: Subject(NounPhrase::Pronoun {
                    pronoun: subject_copula.pronoun,
                    case: PronounCase::Subject,
                }),
                copula: Copula {
                    auxiliary: subject_copula.auxiliary,
                    contracted_with_subject: true,
                },
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
        | RuleTag::QuantityUpTo
        | RuleTag::QuantityThatMany
        | RuleTag::QuantityThatMuch
        | RuleTag::DeterminerClosed
        | RuleTag::DeterminerTarget
        | RuleTag::DeterminerQuantifiedTarget
        | RuleTag::DeterminerQuantity => lower_quantity_or_determiner(tag, children),
        RuleTag::PossessiveNounBase
        | RuleTag::PossessiveNounDetermined
        | RuleTag::DeterminerPossessiveNoun => lower_possessive_noun_phrase(tag, children),
        RuleTag::Adjective
        | RuleTag::AdjectivePhrase
        | RuleTag::Noun
        | RuleTag::NominalNoun
        | RuleTag::NominalAdjective
        | RuleTag::NominalNounModifier
        | RuleTag::NominalPowerToughnessModifier
        | RuleTag::NominalDeterminer
        | RuleTag::NominalPrepositional
        | RuleTag::NominalRelative => lower_nominal(tag, children),
        RuleTag::NounPhraseNominal
        | RuleTag::NounPhraseSubjectPronoun
        | RuleTag::NounPhraseObjectPronoun
        | RuleTag::NounPhraseReciprocal
        | RuleTag::NounPhraseThisCard
        | RuleTag::NounPhraseFullThisCard
        | RuleTag::NounPhraseCoordination
        | RuleTag::PrepositionalPhrase => lower_phrase(tag, children),
        RuleTag::Verb
        | RuleTag::VerbPhraseBase
        | RuleTag::VerbPhraseAuxiliary
        | RuleTag::VerbPhraseDirectObject
        | RuleTag::VerbPhraseAdjective
        | RuleTag::VerbPhrasePrepositional
        | RuleTag::VerbPhraseInfinitive
        | RuleTag::VerbPhraseAdverb
        | RuleTag::VerbPhraseAbility
        | RuleTag::VerbPhraseOracleSymbol
        | RuleTag::VerbPhrasePowerToughness
        | RuleTag::VerbPhraseQuantity
        | RuleTag::InfinitiveTo
        | RuleTag::SimpleClauseSubject
        | RuleTag::SimpleClauseSubjectless
        | RuleTag::ClauseSimple
        | RuleTag::ClauseElliptical
        | RuleTag::ClauseCoordination
        | RuleTag::ClauseCoordinationComma
        | RuleTag::ClauseSubordinateBefore
        | RuleTag::ClauseSubordinateAfterElliptical
        | RuleTag::ClauseSubordinateAfter
        | RuleTag::ClauseExistential
        | RuleTag::ClauseCopularNoun
        | RuleTag::ClauseCopularAdjective
        | RuleTag::ClauseCopularPrepositional
        | RuleTag::ClauseContractedCopularNoun
        | RuleTag::ClauseContractedCopularAdjective
        | RuleTag::ClauseContractedCopularPrepositional
        | RuleTag::RelativeObject
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
        RuleTag::QuantityAtLeast => {
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
        _ => None,
    }
}

fn lower_nominal(tag: RuleTag, children: &mut [Lowered]) -> Option<Lowered> {
    match tag {
        RuleTag::Adjective | RuleTag::Noun => take(children, 0),
        RuleTag::AdjectivePhrase => {
            let Lowered::Adjective(head) = take(children, 0)? else {
                return None;
            };
            Some(Lowered::AdjectivePhrase(AdjectivePhrase {
                head,
                complements: Vec::new(),
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
        RuleTag::NounPhraseThisCard | RuleTag::NounPhraseFullThisCard => {
            let Lowered::ThisCard(form) = take(children, 0)? else {
                return None;
            };
            Some(Lowered::NounPhrase(NounPhrase::ThisCard(form)))
        }
        RuleTag::NounPhraseCoordination => {
            let Lowered::NounPhrase(first) = take(children, 0)? else {
                return None;
            };
            let Lowered::Conjunction(conjunction) = take(children, 1)? else {
                return None;
            };
            let conjunction = match conjunction {
                crate::syntax::PredicateConjunction::And => {
                    crate::syntax::NounPhraseConjunction::And
                }
                crate::syntax::PredicateConjunction::Or => crate::syntax::NounPhraseConjunction::Or,
                crate::syntax::PredicateConjunction::Then => return None,
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
        RuleTag::PrepositionalPhrase => {
            let Lowered::Preposition(preposition) = take(children, 0)? else {
                return None;
            };
            let Lowered::NounPhrase(object) = take(children, 1)? else {
                return None;
            };
            Some(Lowered::PrepositionalPhrase(PrepositionalPhrase {
                preposition,
                object: Box::new(crate::syntax::Phrase::NounPhrase(Box::new(object))),
            }))
        }
        _ => None,
    }
}

fn take(children: &mut [Lowered], index: usize) -> Option<Lowered> {
    let child = children.get_mut(index)?;
    Some(std::mem::replace(child, Lowered::Ignored))
}
