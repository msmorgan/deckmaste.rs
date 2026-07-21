use crate::Span;

/// A lossless parse of one face's Oracle text.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OracleText {
    pub span: Span,
    pub tokens: Vec<Token>,
    pub abilities: Vec<Ability>,
    pub diagnostics: Vec<Diagnostic>,
}

/// One lexical token. Whitespace is represented by the gaps between spans;
/// newlines remain explicit because they delimit printed abilities.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Token {
    pub kind: TokenKind,
    pub span: Span,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TokenKind {
    Word,
    Number,
    /// A braced Oracle symbol such as `{G}`, `{T}`, `{E}`, or `{CHAOS}`.
    Symbol,
    /// The card's abbreviated self-reference, or an ordinary card's full name.
    SelfReference,
    /// A comma-legend's exact full-name self-reference.
    FullSelfReference,
    Bullet,
    Punctuation(char),
    Newline,
}

/// One printed ability, or a modal header and its following bullet lines.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Ability {
    pub span: Span,
    /// An ability-word label such as `Landfall`, without the following dash.
    pub ability_word: Option<Phrase>,
    /// Opaque parenthesized reminder text, preserved like source comments.
    pub reminder_text: Vec<ReminderText>,
    pub kind: AbilityKind,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ReminderText {
    /// The complete parenthesized reminder, including `(` and `)`, in the
    /// caller-owned source text.
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AbilityKind {
    Activated(ActivatedAbility),
    Triggered(TriggeredAbility),
    Loyalty(LoyaltyAbility),
    Modal(ModalAbility),
    /// One or more comma-separated keyword abilities recognized through the
    /// supplied Scryfall catalog.
    Keyword(KeywordAbilityList),
    /// Syntactically ordinary text. Card type and the later lowering pass
    /// decide whether it denotes a spell, static ability, keyword, or other
    /// rules construct.
    Paragraph(Paragraph),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct KeywordAbilityList {
    pub span: Span,
    pub abilities: Vec<KeywordAbility>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct KeywordAbility {
    pub span: Span,
    /// Separator before this item in a list; absent on the first item.
    pub preceding_separator: Option<KeywordListSeparator>,
    /// The canonical keyword name from the Scryfall catalog.
    pub name: String,
    /// The keyword name as printed in the source, including its original case.
    pub printed_name: Phrase,
    pub argument_separator: Option<KeywordArgumentSeparator>,
    /// Printed parameters or alternative costs following the keyword name.
    /// Reminder text is kept separately on the containing [`Ability`].
    pub argument: Option<Phrase>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum KeywordListSeparator {
    Comma,
    Semicolon,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum KeywordArgumentSeparator {
    Space,
    EmDash,
    SpacedEmDash,
}

/// Rules text nested inside another syntactic construct. `span` includes the
/// delimiters when the rules were quoted; the child ability's span covers only
/// its parseable body.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EmbeddedRules {
    pub span: Span,
    pub frame: EmbeddedRulesFrame,
    pub ability: Box<Ability>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum EmbeddedRulesFrame {
    Bare,
    DoubleQuoted { closed: bool },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ActivatedAbility {
    pub cost: Cost,
    pub effect: Paragraph,
}

/// The comma-separated surface components before an activation colon.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Cost {
    pub span: Span,
    pub components: Vec<Phrase>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TriggeredAbility {
    pub introducer: TriggerWord,
    pub introducer_span: Span,
    pub event: SimpleClause,
    pub effect: Paragraph,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct LoyaltyCost {
    pub span: Span,
    pub sign: LoyaltyCostSign,
    pub value: LoyaltyCostValue,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum LoyaltyCostSign {
    None,
    Plus,
    Minus,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
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

/// Concrete syntax between a modal header and its following mode list.
///
/// This belongs to the modal production rather than becoming a phrase of its
/// own.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ModalHeaderSuffix {
    None,
    SpacedEmDash,
}

/// Syntax surrounding a modal header. Keeping this outside the mode bodies
/// lets a later pass lower “choose one” uniformly whether it is a spell line,
/// an activated ability, a triggered ability, or a loyalty ability.
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
        introducer_span: Span,
        event: SimpleClause,
    },
    Loyalty(LoyaltyCost),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ModalPreambleSeparator {
    None,
    Space,
    CommaSpace,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Mode {
    pub span: Span,
    pub body: Paragraph,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Paragraph {
    pub span: Span,
    pub sentences: Vec<Sentence>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Sentence {
    pub span: Span,
    /// Final `.`, `!`, or `?`, when present.
    pub terminal: Option<SentenceTerminal>,
    pub clause: Clause,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SentenceTerminal {
    pub span: Span,
    pub kind: SentenceTerminalKind,
    pub repetitions: u8,
    pub suffix: SentenceTerminalSuffix,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SentenceTerminalKind {
    Period,
    ExclamationMark,
    QuestionMark,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SentenceTerminalSuffix {
    None,
    SingleQuote,
    DoubleQuote,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Clause {
    Simple(SimpleClause),
    Conditional(ConditionalClause),
    CommaSeparated(CommaSeparatedClause),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CommaSeparatedClause {
    pub span: Span,
    pub first: SimpleClause,
    pub second: SimpleClause,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ConditionalClause {
    pub span: Span,
    pub subordinator: Subordinator,
    pub subordinator_capitalization: Capitalization,
    pub subordinator_span: Span,
    pub position: ConditionalPosition,
    pub condition: SimpleClause,
    pub consequence: SimpleClause,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Capitalization {
    Lowercase,
    Capitalized,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ConditionalPosition {
    BeforeConsequence,
    AfterConsequence,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Subordinator {
    When,
    If,
    Unless,
    AsLongAs,
    Until,
    Because,
}

/// A shallow English clause. Spans retain the full phrase even when no known
/// verb lets the parser split it further.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SimpleClause {
    pub span: Span,
    /// Concrete syntax retained when no predicate can yet be identified.
    pub unparsed: Option<Phrase>,
    pub subject: Option<Phrase>,
    pub predicate: Option<Predicate>,
    /// Further predicates coordinated with `predicate` and sharing its
    /// subject, such as `and have haste` in `creatures get +1/+1 and have
    /// haste`.
    pub coordinated_predicates: Vec<CoordinatedPredicate>,
    /// Further clauses joined to this one but carrying their own subject.
    pub coordinated_clauses: Vec<CoordinatedClause>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CoordinatedClause {
    pub conjunction: PredicateConjunction,
    pub conjunction_span: Span,
    pub has_comma: bool,
    pub clause: Box<SimpleClause>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CoordinatedPredicate {
    pub conjunction: PredicateConjunction,
    pub conjunction_span: Span,
    pub has_comma: bool,
    pub predicate: Predicate,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PredicateConjunction {
    And,
    Or,
    Then,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Predicate {
    pub span: Span,
    pub auxiliary: Option<Auxiliary>,
    pub preverb_words: Vec<PreverbWord>,
    pub verb: Phrase,
    pub verb_kind: VerbKind,
    pub complement: Option<Phrase>,
    pub negated: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Auxiliary {
    pub span: Span,
    pub kind: AuxiliaryKind,
    pub inflection: AuxiliaryInflection,
    pub negation: AuxiliaryNegation,
    pub capitalization: Capitalization,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AuxiliaryKind {
    Can,
    Could,
    Do,
    May,
    Might,
    Must,
    Shall,
    Should,
    Will,
    Would,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AuxiliaryInflection {
    Base,
    ThirdPersonSingular,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AuxiliaryNegation {
    None,
    Contracted,
    Fused,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PreverbWord {
    Not,
    Also,
}

/// A leaf phrase in the shallow English syntax tree.
///
/// Exact Scryfall catalog matches carry their canonical spelling and catalog
/// kind. Small closed-class English forms and determined noun phrases are
/// structured directly; unresolved phrases remain whole.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Phrase {
    UnknownPhrase(String),
    Lexeme {
        text: String,
        lemma: String,
        part_of_speech: PartOfSpeech,
    },
    ColorWord {
        text: String,
        color: ColorWord,
    },
    ThisCard {
        text: String,
        form: ThisCardForm,
    },
    OracleSymbol {
        text: String,
        symbol: String,
    },
    SymbolSequence {
        text: String,
        symbols: Vec<String>,
    },
    NumberLiteral {
        text: String,
        value: ScalarValue,
        spelling: NumberSpelling,
    },
    QuantityPhrase(Box<QuantityPhrase>),
    PowerToughness(Box<PowerToughness>),
    NounPhrase(Box<NounPhrase>),
    ModifiedNounPhrase(Box<ModifiedNounPhrase>),
    CatalogTerm {
        text: String,
        canonical: String,
        kind: CatalogKind,
    },
    EmbeddedRulesPhrase {
        text: String,
        embedded_rules: Vec<EmbeddedRules>,
    },
}

impl Phrase {
    #[must_use]
    pub fn text(&self) -> &str {
        match self {
            Self::UnknownPhrase(text)
            | Self::Lexeme { text, .. }
            | Self::ColorWord { text, .. }
            | Self::ThisCard { text, .. }
            | Self::OracleSymbol { text, .. }
            | Self::SymbolSequence { text, .. }
            | Self::NumberLiteral { text, .. }
            | Self::CatalogTerm { text, .. }
            | Self::EmbeddedRulesPhrase { text, .. } => text,
            Self::QuantityPhrase(phrase) => &phrase.text,
            Self::PowerToughness(expression) => &expression.text,
            Self::NounPhrase(phrase) => &phrase.text,
            Self::ModifiedNounPhrase(phrase) => &phrase.text,
        }
    }

    #[must_use]
    pub fn embedded_rules(&self) -> &[EmbeddedRules] {
        match self {
            Self::UnknownPhrase(_)
            | Self::Lexeme { .. }
            | Self::ColorWord { .. }
            | Self::ThisCard { .. }
            | Self::OracleSymbol { .. }
            | Self::SymbolSequence { .. }
            | Self::NumberLiteral { .. }
            | Self::QuantityPhrase(_)
            | Self::PowerToughness(_)
            | Self::NounPhrase(_)
            | Self::ModifiedNounPhrase(_)
            | Self::CatalogTerm { .. } => &[],
            Self::EmbeddedRulesPhrase { embedded_rules, .. } => embedded_rules,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct QuantityPhrase {
    pub text: String,
    pub quantity: Box<Phrase>,
    pub unit: Box<Phrase>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PowerToughness {
    pub text: String,
    pub power: SignedScalar,
    pub toughness: SignedScalar,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SignedScalar {
    pub sign: ScalarSign,
    pub value: ScalarValue,
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
pub enum NumberSpelling {
    Digits,
    EnglishWord(Capitalization),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NounPhrase {
    pub text: String,
    pub determiner: Determiner,
    pub head: Box<Phrase>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ModifiedNounPhrase {
    pub text: String,
    pub modifier: Box<Phrase>,
    pub head: Box<Phrase>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ColorWord {
    White,
    Blue,
    Black,
    Red,
    Green,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PartOfSpeech {
    Adjective,
    Adverb,
    Conjunction,
    Determiner,
    Noun,
    Preposition,
    Pronoun,
    Verb,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ThisCardForm {
    AbbreviatedName,
    FullName,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Determiner {
    pub text: String,
    pub kind: DeterminerKind,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DeterminerKind {
    IndefiniteArticle,
    DefiniteArticle,
    Demonstrative,
    Distributive,
    Possessive,
    Targeting,
    Universal,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CatalogKind {
    KeywordAbility,
    KeywordAction,
    AbilityWord,
    ArtifactType,
    BattleType,
    CreatureType,
    EnchantmentType,
    LandType,
    PlaneswalkerType,
    SpellType,
    /// A lowercase adjective in Oracle text, such as `legendary` or `snow`.
    Supertype,
    /// A lowercase noun in Oracle text, such as `creature` or `instant`.
    CardType,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum VerbKind {
    Ordinary,
    /// A verb phrase recognized through Scryfall's keyword-actions catalog.
    KeywordAction,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Diagnostic {
    pub kind: DiagnosticKind,
    pub span: Span,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DiagnosticKind {
    UnclosedDelimiter(char),
    UnexpectedClosingDelimiter(char),
    UnterminatedQuote,
    MissingTriggerSeparator,
    EmptyActivationCost,
    EmptyActivationEffect,
    OrphanMode,
}
