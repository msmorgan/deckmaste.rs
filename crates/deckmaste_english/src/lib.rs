//! Oracle English: the grammar data file and its binding into the syntax
//! engine.
//!
//! The grammar names only Categories, features, Schemas and core closed-class
//! lexemes. Vocabulary and Subcategorization Frames come from the lexical
//! environment the caller binds it with.

use std::sync::LazyLock;

use deckmaste_lexical::Lexicon;
use deckmaste_lexical_source::data_text;
use deckmaste_syntax::CompiledGrammar;
use deckmaste_syntax::FrameCatalog;
use deckmaste_syntax::Grammar;
use deckmaste_syntax::LoadError;

/// The Oracle English grammar data file: compiled in for release and wasm
/// builds, read from `grammar/english.ron` at run time in dev builds, so a
/// grammar edit needs no rebuild ([`deckmaste_lexical_source::data_text!`]).
///
/// # Panics
/// On first use of a dev build, naming the path, if the file is unreadable.
pub static GRAMMAR: LazyLock<String> = data_text!("grammar/english.ron");

/// Parses the Oracle English grammar data file.
///
/// # Errors
/// Malformed grammar data.
pub fn grammar() -> Result<Grammar, LoadError> {
    Grammar::from_ron_str(&GRAMMAR)
}

/// Binds the Oracle English grammar to a lexical environment.
///
/// # Errors
/// Any binding error, such as a closed word missing from the lexicon or a
/// grammar reference to catalog vocabulary.
pub fn bind(lexicon: &Lexicon, frames: &FrameCatalog) -> Result<CompiledGrammar, LoadError> {
    CompiledGrammar::bind(&grammar()?, lexicon, frames)
}

/// The outer structure of a Corpus Unit, through which the census's Segment
/// walk descends: the line and its top-level sentence sequence, activation
/// costs and labels, modes and modal headers, and a Keyword Line's items and
/// parameters. A `Sentence` is not outer, so a nested sentence sequence stays
/// inside its Segment.
pub const OUTER_CATEGORIES: &[&str] = &[
    "Document",
    "Paragraphs",
    "Ability",
    "Sentences",
    "Effect",
    "Mode",
    "Modes",
    "ModalAbility",
    "ModeEffect",
    "KeywordList",
    "KeywordLineItem",
    "KeywordPhrase",
    "KeywordParameters",
];

/// Structural punctuation a Segment's extent ignores at its edges.
pub const SEGMENT_EDGES: &[char] = &['.', '•', '—', ':', '|', ','];

pub mod cleave;
pub mod corpus;
pub mod judgment;
pub mod profile;
pub mod segments;

/// The highest Complexity Stratum whose gate has been met, if any. Judgments
/// at or below it are enforced, and the census reports a unit at or below it
/// that is not read as a regression.
pub const COMPLETED_STRATUM: Option<profile::Stratum> = Some(7);

/// The tracked deferral list: units at or below the completed Stratum left
/// unread on purpose, each owned by a ticket (`census::deferrals`).
pub static DEFERRED: LazyLock<String> = data_text!("deferred.txt");

/// The tracked list of declined units: Reserved List cards whose
/// construction is unique to them and costly to support (`census::declined`).
/// Loaded like [`GRAMMAR`].
pub static DECLINED: LazyLock<String> = data_text!("declined.txt");

/// The tracked Editorial Corrections the census reads in place of Oracle text
/// (`corpus::editorial_corrections`). Loaded like [`GRAMMAR`].
pub static EDITORIAL_CORRECTIONS: LazyLock<String> = data_text!("corrections.txt");

/// The directory of the judgment set.
#[must_use]
pub fn judgments_dir() -> std::path::PathBuf {
    std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("judgments")
}
pub mod census;
