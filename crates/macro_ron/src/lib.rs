//! Parameterized RON templates ("macros"), expanded during deserialization
//! so the data types stay plain serde derives.
//!
//! A definition is a bare struct naming the macro, the kinds of value it can
//! expand to, its parameter signature, and the expansion body with
//! `Param(...)` holes — see [`MacroDef`]; named params may declare defaults
//! (see [`ParamType`]). Kinds are the serde names of the consumer's types,
//! registered with their reader policy in a [`KindSet`]; see [`Kind`].
//! [`MacroSet::read_str`] is the macro-aware entry point. A
//! kind that [remembers](Kind::remembers_expansion) wraps each expansion in
//! that type's `Expanded` variant as an [`Expansion`], which serializes the
//! *invocation* back.
//!
//! A macro may produce *macros*: a definition with kind `Macro` is a
//! meta-macro whose body is itself a definition template, read at
//! [`MacroDef`] positions. Holes the meta's frame resolves are spliced
//! into the produced definition (including its raw `body`); holes it
//! doesn't own pass through to become the produced macro's own params.

// Lets generated code's `::macro_ron::` paths resolve inside this crate's
// own tests (the serde/serde_derive trick).
extern crate self as macro_ron;

mod expand;
mod expansion;
mod ident;
mod kind;
mod param;
mod set;
mod support;
#[cfg(test)]
mod tests;
mod traverse;

pub use expansion::Expansion;
pub use expansion::ExpansionArgs;
pub use ident::Ident;
pub use ident::IdentSeed;
pub use kind::Kind;
pub use kind::KindSet;
#[cfg(feature = "derive")]
pub use macro_ron_derive::Expand;
#[cfg(feature = "derive")]
pub use macro_ron_derive::SupportsMacros;
pub use param::ParamType;
pub use param::ParamTypeSet;
pub use param::Validator;
pub use set::InsertError;
pub use set::MacroDef;
pub use set::MacroSet;
pub use set::Params;
pub use support::Pair;
pub use support::SupportsMacros;
pub use support::Triple;
pub use support::concat_variants;
pub use traverse::Expand;
