//! Parameterized RON templates ("macros"), expanded during deserialization
//! so the data types stay plain serde derives.

mod ident;

pub use ident::{Ident, IdentSeed};
