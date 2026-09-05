//! Oracle-text English grammar — **the authoritative implementation**.
//!
//! Supersedes `deckmaste_english`, whose name this crate takes at cutover. New
//! architecture and features belong here, not in the v1 crate retained to hold
//! the roof up until then. This crate receives immutable typed provider rows
//! from its caller and performs no catalog discovery or file access itself.
//!
//! Authority: `docs/decisions/english-v2-rewrite.md`.

// The generated category types nest as deeply as the grammar does, so proving
// an auto trait such as `Send` for `BuildValue` exceeds the default evaluation
// depth. Raising the limit keeps those proofs available to callers and lints.
#![recursion_limit = "256"]

pub mod ast;
mod constructions;
pub mod context;
pub mod environment;
mod orthography;
pub mod parser;
pub mod render;
pub mod visit;
