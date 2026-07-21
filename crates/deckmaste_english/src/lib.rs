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
//! them to recognize keyword-ability lines, keyword-action predicates,
//! ability-word labels, card types, supertypes, and category-specific
//! subtypes. The simpler [`parse`] entry point remains available when catalog
//! data is not present.
//!
//! To inspect a card from the repository's local data snapshot, run:
//!
//! ```text
//! cargo xtask english inspect "Lightning Bolt"
//! ```
//!
//! The inspector loads the repository's local Scryfall catalogs automatically.
//! Pass `--verbose` to include the underlying byte spans.
//!
//! To rank unknown phrases across the complete local snapshot, run:
//!
//! ```text
//! cargo xtask english unknown
//! ```
//!
//! Card-data examples normalize self-references before parsing: `~` denotes a
//! comma-legend's abbreviated name and `~~` its full name.

mod ast;
mod catalogs;
mod input;
mod numeral;
mod parser;
mod render;
mod source_debug;
mod span;

pub use ast::Ability;
pub use ast::AbilityKind;
pub use ast::ActivatedAbility;
pub use ast::Auxiliary;
pub use ast::AuxiliaryInflection;
pub use ast::AuxiliaryKind;
pub use ast::AuxiliaryNegation;
pub use ast::Capitalization;
pub use ast::CatalogKind;
pub use ast::Clause;
pub use ast::ColorWord;
pub use ast::CommaSeparatedClause;
pub use ast::ConditionalClause;
pub use ast::ConditionalPosition;
pub use ast::CoordinatedClause;
pub use ast::CoordinatedPredicate;
pub use ast::Cost;
pub use ast::Determiner;
pub use ast::DeterminerKind;
pub use ast::Diagnostic;
pub use ast::DiagnosticKind;
pub use ast::EmbeddedRules;
pub use ast::EmbeddedRulesFrame;
pub use ast::KeywordAbility;
pub use ast::KeywordAbilityList;
pub use ast::KeywordArgumentSeparator;
pub use ast::KeywordListSeparator;
pub use ast::LoyaltyAbility;
pub use ast::LoyaltyCost;
pub use ast::LoyaltyCostSign;
pub use ast::LoyaltyCostValue;
pub use ast::ModalAbility;
pub use ast::ModalFrame;
pub use ast::ModalHeaderSuffix;
pub use ast::ModalPreambleSeparator;
pub use ast::Mode;
pub use ast::ModifiedNounPhrase;
pub use ast::NounPhrase;
pub use ast::NumberSpelling;
pub use ast::OracleText;
pub use ast::Paragraph;
pub use ast::PartOfSpeech;
pub use ast::Phrase;
pub use ast::PowerToughness;
pub use ast::Predicate;
pub use ast::PredicateConjunction;
pub use ast::PreverbWord;
pub use ast::QuantityPhrase;
pub use ast::ReminderText;
pub use ast::ScalarSign;
pub use ast::ScalarValue;
pub use ast::Sentence;
pub use ast::SentenceTerminal;
pub use ast::SentenceTerminalKind;
pub use ast::SentenceTerminalSuffix;
pub use ast::SignedScalar;
pub use ast::SimpleClause;
pub use ast::Subordinator;
pub use ast::ThisCardForm;
pub use ast::Token;
pub use ast::TokenKind;
pub use ast::TriggerWord;
pub use ast::TriggeredAbility;
pub use ast::VerbKind;
pub use catalogs::Catalogs;
pub use input::normalize_self_references;
pub use input::strip_reminder_text;
pub use numeral::Numeral;
pub use numeral::ParseNumeralError;
pub use parser::parse;
pub use parser::parse_with_catalogs;
pub use render::RenderError;
pub use source_debug::AbilitiesSourceDebug;
pub use source_debug::DiagnosticsSourceDebug;
pub use source_debug::SourceDebug;
pub use span::Span;
