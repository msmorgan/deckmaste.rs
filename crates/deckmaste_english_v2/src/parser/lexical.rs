use crate::ast::VerbLexeme;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Ord, PartialOrd)]
pub(crate) enum NounNumber {
    Singular,
    Plural,
    Either,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Ord, PartialOrd)]
pub(crate) enum Lexical {
    Literal(&'static str),
    EndOfInput,
    TriggerWord,
    Article,
    Demonstrative,
    Pronoun,
    Variable,
    Noun(NounNumber),
    Verb(VerbLexeme),
    SignedNumber,
    SelfReference,
}
