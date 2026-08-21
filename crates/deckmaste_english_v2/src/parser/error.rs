use std::collections::BTreeSet;
use std::fmt;

use super::SelectionExceptionInventoryError;
use crate::constructions::BuildRejection;
use crate::constructions::NonterminalCategory;
use crate::constructions::TerminalClass;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Ord, PartialOrd)]
pub struct TextSpan {
    pub start: usize,
    pub end: usize,
}

#[derive(Debug, Clone, PartialEq, Eq, Ord, PartialOrd)]
pub enum Expectation {
    Nonterminal(NonterminalCategory),
    Terminal(TerminalClass),
    Literal(&'static str),
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
    BuildRejected {
        span: TextSpan,
        rejection: BuildRejection,
    },
    Ambiguous {
        first: &'static str,
        second: &'static str,
    },
    InvalidSelectionExceptionConfiguration(SelectionExceptionInventoryError),
    ValidatedRootDidNotMaterialize,
    OwnershipInspection,
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
            Self::BuildRejected { rejection, .. } => rejection.fmt(formatter),
            Self::Ambiguous { first, second } => {
                write!(formatter, "ambiguous parse between {first} and {second}")
            }
            Self::InvalidSelectionExceptionConfiguration(error) => error.fmt(formatter),
            Self::ValidatedRootDidNotMaterialize => {
                formatter.write_str("validated chart root did not materialize")
            }
            Self::OwnershipInspection => {
                formatter.write_str("selected lexical ownership could not be inspected")
            }
        }
    }
}

impl std::error::Error for ParseError {}
