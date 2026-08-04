//! Construction declaration compiler: typed declaration model, layer-2
//! validation, and (in a later milestone) deterministic code emission.
//! The proc-macro facade lives in `deckmaste_constructions_macro`;
//! this crate is a plain library so every stage is directly testable.

pub mod diag;
pub mod emit;
pub mod model;
pub mod parse;
pub mod render;
pub mod runtime;
pub mod validate;
