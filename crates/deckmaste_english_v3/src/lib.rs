//! Feature-sensitive Earley recognition and lazy grammatical Readings.
//!
//! See the crate README for the summary contract and measured complexity.

mod chart;
mod forest;
mod readings;

pub use chart::ParseError;
pub use chart::parse;
pub use forest::CompletedId;
pub use forest::Forest;
pub use forest::Leaf;
pub use forest::Metrics;
pub use readings::MaterializeError;
pub use readings::Materializer;
pub use readings::ReadingMetrics;
pub use readings::Readings;

/// The packed forest associated with a grammar's admission vocabulary.
pub type GrammarForest<G> =
    Forest<<G as Grammar>::Category, <G as Grammar>::Summary, <G as Grammar>::State>;

use deckmaste_lexical::Category;
use deckmaste_lexical::FeatureBundle;
use deckmaste_lexical::LexicalProperties;
use deckmaste_lexical::Numeral;
use deckmaste_lexical::WordForm;

/// A declaration's ordered grammatical or surface position.
#[derive(Debug, Clone)]
pub enum Symbol<C> {
    Nonterminal(C),
    Lexical(Category),
    Literal(String),
}

/// Its index in `Grammar::productions` is the runtime production identity.
#[derive(Debug, Clone)]
pub struct Production<C> {
    pub category: C,
    pub symbols: Vec<Symbol<C>>,
}

/// Declared grammatical information, without lexical identity or spelling.
/// Alternatives in a lexical occurrence remain correlated in one bundle.
pub enum LexicalFeatures<'a> {
    Word {
        category: Category,
        form: WordForm,
        features: &'a FeatureBundle,
        properties: &'a LexicalProperties,
    },
    Numeral {
        value: i32,
        notation: Numeral,
    },
}

/// Admission over summaries, never over materialized child values.
///
/// Equal summaries MUST be indistinguishable to every possible parent. Equal
/// states at the same production/dot MUST admit precisely the same suffixes
/// with the same resulting summaries. Retain unresolved agreement, selected
/// frames, extraction, sharing and recoverability obligations until the
/// governing context checks them. Missing lexical features are not wildcards.
///
/// These functions must be pure and depend only on their arguments and
/// immutable grammar data. The closure of reachable states and summaries must
/// be finite for every finite input, including nullable recursion. The compiler
/// must establish this contract for each generated grammar. Neither ASTs nor
/// source positions belong in `State` or `Summary`. The runtime cannot prove a
/// user's equivalence relation correct. Conservative distinctions cost space
/// but do not lose Readings; independent unions of correlated features are
/// unsound. States must also retain construction-local feature choices needed
/// by the materializer; child structure and leaf evidence stay on packed edges.
pub trait Grammar {
    type Category: Clone + Ord;
    type Summary: Clone + Ord;
    type State: Clone + Ord;

    fn productions(&self) -> &[Production<Self::Category>];
    fn lexical(&self, features: LexicalFeatures<'_>) -> Vec<Self::Summary>;
    fn begin(&self, production: usize) -> Vec<Self::State>;

    /// `child` is absent only at a declared Literal position. Its identity is
    /// already fixed by the production/dot, and no source text is inspected.
    fn advance(
        &self,
        production: usize,
        dot: usize,
        state: &Self::State,
        child: Option<&Self::Summary>,
    ) -> Vec<Self::State>;

    fn complete(&self, production: usize, state: &Self::State) -> Option<Self::Summary>;

    /// Discharge any obligations that cannot remain open at the requested root.
    fn root(&self, category: &Self::Category, summary: &Self::Summary) -> bool;
}
