//! Library side of xtask: one module per command, so integration tests and
//! the `cargo-xtask` binary can drive the command logic. xtask owns all of
//! the workspace's CLI parsing; the other crates are pure libraries.
pub mod card;
pub mod cite;
pub mod migrate;
pub mod validate;
