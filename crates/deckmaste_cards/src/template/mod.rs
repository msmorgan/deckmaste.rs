//! The macro template grammar as a *parse* source: compiling a macro's
//! render `template` into a [`pattern::ParsePattern`] and indexing those
//! patterns by kind so oracle text can be matched back to the macro that
//! produced it (`English → macro`, the reverse of [`crate::render::template`]).

pub mod index;
pub mod pattern;
