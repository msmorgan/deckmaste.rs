//! Library side of xtask: one module per command, so integration tests and
//! the `cargo-xtask` binary can drive the command logic. xtask owns all of
//! the workspace's CLI parsing; the other crates are pure libraries.

#![allow(
    clippy::missing_errors_doc,
    clippy::missing_panics_doc,
    reason = "internal tooling: `# Errors`/`# Panics` doc sections aren't worth \
    keeping accurate here, unlike in the library crates"
)]

pub mod authoring;
pub mod card;
pub mod catalogs;
pub mod cite;
pub mod coverage;
pub mod derive_cards;
pub mod english;
pub mod english_v3;
pub mod extract;
pub mod facts;
pub mod fidelity;
pub mod gate;
pub mod generate;
pub mod graduate;
pub mod idris_check;
pub mod lean_check;
pub mod lexical;
pub mod macros;
pub mod map;
pub mod resolve;
pub mod scryfall_snapshot;
pub mod stubs;
pub mod validate;

mod raw_corpus;
