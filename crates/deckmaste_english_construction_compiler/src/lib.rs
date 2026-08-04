//! Construction declaration compiler: typed declaration model, layer-2
//! validation, and (in a later milestone) deterministic code emission.
//! The proc-macro facade lives in `deckmaste_english_construction_macros`;
//! this crate is a plain library so every stage is directly testable.

pub mod diag;
pub mod model;
pub mod runtime;
pub mod spike_emit;
pub mod validate;
