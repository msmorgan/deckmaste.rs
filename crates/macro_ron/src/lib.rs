//! Parameterized RON templates ("macros"), expanded during deserialization
//! so the data types stay plain serde derives.
//!
//! A definition is a bare struct naming the macro, the kinds of value it can
//! expand to, its parameter signature, and the expansion body with
//! `Param(...)` holes — see [`MacroDef`]. Kinds are the serde names of the
//! consumer's types, registered with their reader policy in a [`KindSet`];
//! see [`Kind`]. [`MacroSet::read_str`] is the macro-aware entry point. A
//! kind that [remembers](Kind::remembers_expansion) wraps each expansion in
//! that type's `Expanded` variant as an [`Expansion`], which serializes the
//! *invocation* back.

mod expand;
mod expansion;
mod ident;
mod kind;
mod set;

pub use expansion::{Expansion, ExpansionArgs};
pub use ident::{Ident, IdentSeed};
pub use kind::{Kind, KindSet};
pub use set::{InsertError, MacroDef, MacroSet, ParamType, Params};
