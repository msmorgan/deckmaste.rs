use std::collections::BTreeSet;

use engine::ChartFailure;
use engine::RulePosition;
use grammar::Category;
use grammar::Lexical;
use grammar::SliceGrammar;
use grammar::materialize;
use grammar::parse_forest;
use selection::select;

use crate::ast::Ability;
use crate::catalogs::ParserCatalogs;
use crate::context::ParseContext;

mod engine;
mod grammar;
mod selection;

pub use error::Expectation;
pub use error::NonterminalCategory;
pub use error::ParseError;
pub use error::TerminalClass;
pub use error::TextSpan;

mod error;

#[derive(Debug, Clone)]
pub struct Parser {
    catalogs: ParserCatalogs,
}

impl Parser {
    #[must_use]
    pub fn new(catalogs: ParserCatalogs) -> Self {
        Self { catalogs }
    }

    /// Parses one complete ability from exact rendered text.
    ///
    /// # Errors
    ///
    /// Returns a structured failure when the chart cannot consume the input or
    /// checked lowering rejects every reading. Returns an ambiguity when
    /// structural selection cannot choose one reading.
    pub fn parse(&self, text: &str, context: &ParseContext<'_>) -> Result<Ability, ParseError> {
        let grammar = SliceGrammar {
            catalogs: &self.catalogs,
            context,
        };
        let forest = parse_forest(&grammar, text).map_err(chart_failure)?;
        select(materialize(&forest))?.ok_or_else(|| ParseError::Failure {
            span: TextSpan {
                start: text.len(),
                end: text.len(),
            },
            expectations: BTreeSet::new(),
        })
    }
}

fn chart_failure(failure: ChartFailure<Category, Lexical>) -> ParseError {
    ParseError::Failure {
        span: TextSpan {
            start: failure.offset,
            end: failure.offset,
        },
        expectations: failure.live.into_iter().map(expectation).collect(),
    }
}

const fn expectation(position: RulePosition<Category, Lexical>) -> Expectation {
    match position {
        RulePosition::Nonterminal(category) => {
            Expectation::Nonterminal(nonterminal_category(category))
        }
        RulePosition::Lexical(Lexical::Literal(literal)) => Expectation::Literal(literal),
        RulePosition::Lexical(lexical) => Expectation::Terminal(terminal_class(lexical)),
    }
}

const fn nonterminal_category(category: Category) -> NonterminalCategory {
    match category {
        Category::Ability => NonterminalCategory::Ability,
        Category::Sentence => NonterminalCategory::Sentence,
        Category::Clause => NonterminalCategory::Clause,
        Category::NounPhrase => NonterminalCategory::NounPhrase,
        Category::VerbPhrase => NonterminalCategory::VerbPhrase,
        Category::Amount => NonterminalCategory::Amount,
    }
}

const fn terminal_class(lexical: Lexical) -> TerminalClass {
    match lexical {
        Lexical::Literal(_) => unreachable!(),
        Lexical::TriggerWord => TerminalClass::TriggerWord,
        Lexical::Article => TerminalClass::Article,
        Lexical::Demonstrative => TerminalClass::Demonstrative,
        Lexical::Pronoun => TerminalClass::Pronoun,
        Lexical::Variable => TerminalClass::Variable,
        Lexical::Noun(_) => TerminalClass::Noun,
        Lexical::Verb(_) => TerminalClass::VerbLexeme,
        Lexical::SignedNumber => TerminalClass::SignedNumber,
        Lexical::SelfReference => TerminalClass::SelfReference,
    }
}
