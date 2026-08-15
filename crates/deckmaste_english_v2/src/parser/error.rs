use std::collections::BTreeSet;
use std::fmt;

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
    EndOfInput,
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

impl fmt::Display for ParseError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Failure { span, expectations } => write!(
                formatter,
                "parse failed at bytes {}..{}; expected {expectations:?}",
                span.start, span.end
            ),
            Self::Ambiguous { first, second } => {
                write!(formatter, "ambiguous parse between {first} and {second}")
            }
        }
    }
}

impl std::error::Error for ParseError {}
