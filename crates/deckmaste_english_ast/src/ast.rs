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
    SelfReference,
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
    /// The complete parenthesized reminder, including `(` and `)`.
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
    /// The canonical keyword name from the Scryfall catalog.
    pub name: String,
    /// The keyword name as printed in the source, including its original case.
    pub printed_name: Phrase,
    /// Printed parameters or alternative costs following the keyword name.
    /// Reminder text is kept separately on the containing [`Ability`].
    pub argument: Option<Phrase>,
}

/// Rules text nested inside another syntactic construct. `span` includes the
/// delimiters when the rules were quoted; the child ability's span covers only
/// its parseable body.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EmbeddedRules {
    pub span: Span,
    pub ability: Box<Ability>,
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
    /// The bracketed loyalty symbol, such as `[+1]` or `[−X]`.
    pub cost: Span,
    pub effect: Paragraph,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ModalAbility {
    pub frame: ModalFrame,
    pub header: Paragraph,
    pub modes: Vec<Mode>,
}

/// Syntax surrounding a modal header. Keeping this outside the mode bodies
/// lets a later pass lower “choose one” uniformly whether it is a spell line,
/// an activated ability, a triggered ability, or a loyalty ability.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ModalFrame {
    Unframed,
    Activated(Cost),
    Triggered {
        introducer: TriggerWord,
        introducer_span: Span,
        event: SimpleClause,
    },
    Loyalty(Span),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Mode {
    pub span: Span,
    pub bullet: Span,
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
    pub terminal: Option<Span>,
    pub clause: Clause,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Clause {
    Simple(SimpleClause),
    Conditional(ConditionalClause),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ConditionalClause {
    pub span: Span,
    pub subordinator: Subordinator,
    pub subordinator_span: Span,
    pub position: ConditionalPosition,
    pub condition: SimpleClause,
    pub consequence: SimpleClause,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ConditionalPosition {
    BeforeConsequence,
    AfterConsequence,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Subordinator {
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
    pub clause: Box<SimpleClause>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CoordinatedPredicate {
    pub conjunction: PredicateConjunction,
    pub conjunction_span: Span,
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
    pub auxiliary: Option<Span>,
    pub verb: Phrase,
    pub verb_kind: VerbKind,
    pub complement: Option<Phrase>,
    pub negated: bool,
}

/// A phrase that has not yet been assigned a more specific English syntactic
/// role. Unknown phrases remain whole instead of pretending their lexical
/// tokens are meaningful phrase structure.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Phrase {
    UnknownPhrase(String),
    EmbeddedRulesPhrase {
        text: String,
        embedded_rules: Vec<EmbeddedRules>,
    },
}

impl Phrase {
    #[must_use]
    pub fn text(&self) -> &str {
        match self {
            Self::UnknownPhrase(text) | Self::EmbeddedRulesPhrase { text, .. } => text,
        }
    }

    #[must_use]
    pub fn embedded_rules(&self) -> &[EmbeddedRules] {
        match self {
            Self::UnknownPhrase(_) => &[],
            Self::EmbeddedRulesPhrase { embedded_rules, .. } => embedded_rules,
        }
    }
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
