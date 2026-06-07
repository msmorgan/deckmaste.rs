//! Parameterized RON templates ("macros"), expanded during deserialization
//! so the data types stay plain serde derives.

mod expansion;
mod ident;

pub use expansion::{Expansion, ExpansionArgs};
pub use ident::{Ident, IdentSeed};
