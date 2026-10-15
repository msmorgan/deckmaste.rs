//! Oracle English: the grammar data file and its binding into the syntax
//! engine.
//!
//! The grammar names only Categories, features, Schemas and core closed-class
//! lexemes. Vocabulary and Subcategorization Frames come from the lexical
//! environment the caller binds it with.

use deckmaste_lexical::Lexicon;
use deckmaste_syntax::CompiledGrammar;
use deckmaste_syntax::FrameCatalog;
use deckmaste_syntax::Grammar;
use deckmaste_syntax::LoadError;

/// The Oracle English grammar data file.
pub const GRAMMAR: &str = include_str!("../grammar/english.ron");

/// Parses the Oracle English grammar data file.
///
/// # Errors
/// Malformed grammar data.
pub fn grammar() -> Result<Grammar, LoadError> {
    Grammar::from_ron_str(GRAMMAR)
}

/// Binds the Oracle English grammar to a lexical environment.
///
/// # Errors
/// Any binding error, such as a closed word missing from the lexicon or a
/// grammar reference to catalog vocabulary.
pub fn bind(lexicon: &Lexicon, frames: &FrameCatalog) -> Result<CompiledGrammar, LoadError> {
    CompiledGrammar::bind(&grammar()?, lexicon, frames)
}

pub mod corpus;
pub mod judgment;
pub mod profile;

/// The highest Complexity Stratum whose gate has been met, if any. Judgments
/// at or below it are enforced, and the census reports a unit at or below it
/// that is not read as a regression.
pub const COMPLETED_STRATUM: Option<profile::Stratum> = Some(0);

/// The directory of the judgment set.
#[must_use]
pub fn judgments_dir() -> std::path::PathBuf {
    std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("judgments")
}
pub mod census;
