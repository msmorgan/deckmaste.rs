//! Migrations that build and refine the plugin data directories.

mod data;
pub mod extract;
mod ident;
mod layout;
pub(crate) mod parsers;
pub mod resolve;
mod ron_output;
pub mod stubs;
pub mod todo_card;
