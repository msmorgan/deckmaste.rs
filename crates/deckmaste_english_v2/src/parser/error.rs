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

impl fmt::Display for NonterminalCategory {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let label = match self {
            Self::Ability => "ability",
            Self::Sentence => "sentence",
            Self::Clause => "clause",
            Self::NounPhrase => "noun phrase",
            Self::VerbPhrase => "verb phrase",
            Self::Amount => "amount",
        };
        formatter.write_str(label)
    }
}

impl fmt::Display for TerminalClass {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let label = match self {
            Self::Article => "article",
            Self::Demonstrative => "demonstrative",
            Self::EndOfInput => "end of input",
            Self::Noun => "noun",
            Self::Pronoun => "pronoun",
            Self::SelfReference => "self-reference",
            Self::SignedNumber => "signed number",
            Self::TriggerWord => "trigger word",
            Self::Variable => "variable",
            Self::VerbLexeme => "verb lexeme",
        };
        formatter.write_str(label)
    }
}

impl fmt::Display for Expectation {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Nonterminal(category) => category.fmt(formatter),
            Self::Terminal(class) => class.fmt(formatter),
            Self::Literal(literal) => write!(formatter, "`{literal}`"),
        }
    }
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
    ValidatedRootDidNotMaterialize,
}

impl fmt::Display for ParseError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Failure { span, expectations } => {
                write!(
                    formatter,
                    "parse failed at bytes {}..{}; expected ",
                    span.start, span.end
                )?;
                let mut expectations = expectations.iter();
                if let Some(expectation) = expectations.next() {
                    write!(formatter, "{expectation}")?;
                }
                for expectation in expectations {
                    write!(formatter, ", {expectation}")?;
                }
                Ok(())
            }
            Self::Ambiguous { first, second } => {
                write!(formatter, "ambiguous parse between {first} and {second}")
            }
            Self::ValidatedRootDidNotMaterialize => {
                formatter.write_str("validated chart root did not materialize")
            }
        }
    }
}

impl std::error::Error for ParseError {}
