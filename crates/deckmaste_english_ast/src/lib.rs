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
//! To inspect a card from the repository's local data snapshot, run:
//!
//! ```text
//! cargo run -p deckmaste_english_ast --example inspect -- "Lightning Bolt"
//! ```

mod ast;
mod parser;
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
pub use parser::parse;
pub use span::Span;
