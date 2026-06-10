//! Library side of xtask: one module per command, so integration tests and
//! the `cargo-xtask` binary can drive the command logic. xtask owns all of
//! the workspace's CLI parsing; the other crates are pure libraries.

// Internal tooling: `# Errors`/`# Panics` doc sections aren't worth keeping
// accurate here, unlike in the library crates.
#![allow(clippy::missing_errors_doc, clippy::missing_panics_doc)]

pub mod card;
pub mod cite;
pub mod extract;
pub mod generate;
pub mod graduate;
pub mod resolve;
pub mod stubs;
pub mod validate;
