//! Oracle-text English grammar — **the authoritative implementation**.
//!
//! Supersedes `deckmaste_english`, whose name this crate takes at cutover. New
//! architecture and features belong here, not in the v1 crate retained to hold
//! the roof up until then. This crate must never depend on a crate slated for
//! deletion; today its only direct dependency is the stable catalog layer.
//!
//! Authority: `docs/decisions/english-v2-rewrite.md`.

pub mod ast;
mod constructions;
pub mod context;
pub mod environment;
mod orthography;
pub mod parser;
pub mod render;
pub mod visit;
