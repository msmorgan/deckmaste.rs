//! A lossless syntactic representation of Magic card text.
//!
//! This crate sits deliberately before deckmaste's rules grammar. It knows
//! that an activated ability has a cost and an effect, that a triggered
//! ability has an event and an effect, and that English clauses have subjects,
//! predicates, and subordinators. It does not decide that “draw a card” is a
//! `deckmaste_core::Effect::Draw`, and it never emits RON.
//!
//! Every node refers back to the input with a byte [`Span`]. Unknown or only
//! partly understood English therefore remains available to later parser
//! passes instead of being flattened or discarded. [`parse`] is intentionally
//! total: malformed structure produces [`Diagnostic`] values and the most
//! conservative AST the parser can construct.
//!
//! [`parse_with_catalogs`] accepts current Scryfall catalog values without
//! coupling this crate to a particular data directory or downloader. It uses
//! them to recognize keyword-ability lines, keyword-action predicates, and
//! ability-word labels. The simpler [`parse`] entry point remains available
//! when catalog data is not present.
//!
//! To inspect a card from the repository's local data snapshot, run:
//!
//! ```text
//! cargo run -p deckmaste_english_ast --example inspect -- "Lightning Bolt"
//! ```
//!
//! The inspector loads the repository's local Scryfall catalogs automatically.
//! Pass `--verbose` to include the underlying byte spans.

mod ast;
mod catalogs;
mod parser;
mod source_debug;
mod span;

pub use ast::Ability;
pub use ast::AbilityKind;
pub use ast::ActivatedAbility;
pub use ast::Clause;
pub use ast::ConditionalClause;
pub use ast::ConditionalPosition;
pub use ast::Cost;
pub use ast::Diagnostic;
pub use ast::DiagnosticKind;
pub use ast::KeywordAbility;
pub use ast::KeywordAbilityList;
pub use ast::LoyaltyAbility;
pub use ast::ModalAbility;
pub use ast::ModalFrame;
pub use ast::Mode;
pub use ast::OracleText;
pub use ast::Paragraph;
pub use ast::Predicate;
pub use ast::Sentence;
pub use ast::SimpleClause;
pub use ast::Subordinator;
pub use ast::Token;
pub use ast::TokenKind;
pub use ast::TriggerWord;
pub use ast::TriggeredAbility;
pub use ast::VerbKind;
pub use catalogs::Catalogs;
pub use parser::parse;
pub use parser::parse_with_catalogs;
pub use source_debug::SourceDebug;
pub use span::Span;
