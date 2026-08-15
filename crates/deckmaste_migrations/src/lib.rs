//! Card data pipeline (extract, resolve) plus macro-stub generation for plugin
//! data directories.

pub mod catalogs;
pub mod extract;
pub mod graduate;
mod ident;
mod layout;
pub mod oracle_snapshot;
pub(crate) mod parsers;
pub mod resolve;
mod ron_output;
pub mod stubs;
pub mod todo_card;
