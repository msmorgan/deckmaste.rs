//! The semantics v2 type universe: the encoded-English card grammar, mirrored
//! from the Lean workbench.
//!
//! `lean/Semantics/{Words,Events,Phrase,Triggers,Abilities,Card}.lean` is the
//! specification. Each of those files has a module here that mirrors it
//! constructor-for-constructor and field-for-field; a Lean change is a Rust
//! change, never the reverse, and `tests/lean_drift.rs` fails on any name the
//! two sides disagree on (`docs/decisions/semantics-v2.md` §10).
//!
//! The crate does no law checking and emits no refusals. [`reads`] exposes the
//! structural reads lowering needs — kind projection and binding resolution —
//! as plain functions ported from `lean/Semantics/Check/`; nothing that refuses
//! is ported. The Lean gate (§13) is the only checker.
//!
//! Naming: a Lean constructor becomes a Rust variant of the same word in
//! `UpperCamelCase`, a Lean field a Rust field of the same word in
//! `snake_case`. Lean's trailing-underscore keyword escape (`from_`, `while_`)
//! is dropped and Rust's own escape applied instead — a raw identifier
//! (`r#from`, `r#while`, `r#type`), which serde reads and writes under the bare
//! word. `Self` alone cannot be a raw identifier, so
//! [`Determiner::Self_`](words::Determiner::Self_) carries a `serde(rename)`
//! back to the Lean spelling.

pub mod abilities;
pub mod card;
pub mod events;
pub mod phrase;
pub mod reader;
pub mod reads;
pub mod ron;
pub mod rules;
pub mod triggers;
pub mod words;
