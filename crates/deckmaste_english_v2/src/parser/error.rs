use std::collections::BTreeSet;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Ord, PartialOrd)]
pub struct TextSpan {
    pub start: usize,
    pub end: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Ord, PartialOrd)]
pub enum NonterminalCategory {
    Ability,
    Sentence,
    Clause,
    NounPhrase,
    VerbPhrase,
    Amount,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Ord, PartialOrd)]
pub enum TerminalClass {
    Article,
    Demonstrative,
    Noun,
    Pronoun,
    SelfReference,
    SignedNumber,
    TriggerWord,
    Variable,
    VerbLexeme,
}

#[derive(Debug, Clone, PartialEq, Eq, Ord, PartialOrd)]
pub enum Expectation {
    Nonterminal(NonterminalCategory),
    Terminal(TerminalClass),
    Literal(&'static str),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ParseError {
    Failure {
        span: TextSpan,
        expectations: BTreeSet<Expectation>,
    },
    Ambiguous {
        first: &'static str,
        second: &'static str,
    },
}
