use deckmaste_catalogs::CatalogKind;

use crate::catalogs::ParserCatalogs;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Ability {
    Spell(Spell),
    Triggered(Triggered),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Sentence {
    Imperative(Imperative),
    Declarative(Declarative),
    WithWhere(WithWhere),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Clause {
    Event(EventClause),
    Where(WhereClause),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum NounPhrase {
    Pronoun(PronounNp),
    Common(Common),
    Demonstrative(DemonstrativeNp),
    Target(TargetNp),
    SelfReference(SelfReferenceNp),
    Count(CountNp),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum VerbPhrase {
    Destroy(Destroy),
    Connive(Connive),
    DealDamage(DealDamage),
    GainLife(GainLife),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Amount {
    Number(NumberAmount),
    Variable(VariableAmount),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Spell {
    pub effect: Sentence,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Imperative {
    pub predicate: VerbPhrase,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Declarative {
    pub subject: NounPhrase,
    pub predicate: VerbPhrase,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WithWhere {
    pub body: Box<Sentence>,
    pub clause: Clause,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EventClause {
    pub subject: NounPhrase,
    pub predicate: VerbPhrase,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WhereClause {
    pub variable: Variable,
    pub value: NounPhrase,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PronounNp {
    pub word: Pronoun,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Common {
    pub article: Article,
    pub head: Noun,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DemonstrativeNp {
    pub word: Demonstrative,
    pub head: Noun,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TargetNp {
    pub head: Noun,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CountNp {
    pub head: Noun,
    pub controller: Pronoun,
    pub threshold: SignedNumber,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Destroy {
    pub object: NounPhrase,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Connive;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DealDamage {
    pub amount: Amount,
    pub to: NounPhrase,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GainLife {
    pub amount: Amount,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NumberAmount {
    pub number: SignedNumber,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VariableAmount {
    pub variable: Variable,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TriggerWord {
    Whenever,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Article {
    A,
    An,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Demonstrative {
    That,
    Those,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Pronoun {
    It,
    You,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Variable {
    X,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Sign {
    Positive,
    Negative,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SelfReferenceSpelling {
    Full,
    Abbreviated,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Noun {
    Lexeme(NounLexeme),
    Catalog(CatalogIdentity),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NounLexeme {
    Player,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VerbLexeme {
    Destroy,
    Connive,
    Deal,
    Gain,
    Control,
    Be,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SignedNumber {
    pub sign: Sign,
    pub magnitude: u32,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CatalogIdentity {
    pub(crate) kind: CatalogKind,
    pub(crate) spelling: String,
}

impl CatalogIdentity {
    #[must_use]
    pub fn new(
        catalogs: &ParserCatalogs,
        kind: CatalogKind,
        spelling: impl Into<String>,
    ) -> Option<Self> {
        let spelling = spelling.into();
        catalogs
            .set()
            .get(kind)
            .contains(&spelling)
            .then_some(Self { kind, spelling })
    }

    #[must_use]
    pub const fn kind(&self) -> CatalogKind {
        self.kind
    }

    #[must_use]
    pub fn spelling(&self) -> &str {
        &self.spelling
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SelfReferenceNp {
    pub spelling: SelfReferenceSpelling,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Triggered {
    pub(crate) trigger: TriggerWord,
    pub(crate) event: Clause,
    pub(crate) effect: Sentence,
}

impl Triggered {
    #[must_use]
    pub fn new(trigger: TriggerWord, event: Clause, effects: Vec<Sentence>) -> Option<Self> {
        // Multiple effects await grammar buildout.
        let [effect] = effects.try_into().ok()?;
        Some(Self {
            trigger,
            event,
            effect,
        })
    }

    #[must_use]
    pub const fn trigger(&self) -> TriggerWord {
        self.trigger
    }

    #[must_use]
    pub const fn event(&self) -> &Clause {
        &self.event
    }

    #[must_use]
    pub const fn effect(&self) -> &Sentence {
        &self.effect
    }
}
