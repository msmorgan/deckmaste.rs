use serde::{Deserialize, Serialize};

use crate::{Ident, Type};

/// A subtype: its name and the card types it can appear on (CR 205.3).
///
/// Subtypes are open-ended data, declared by plugins (usually through subtype
/// macros like `LandType("Forest")`) rather than baked in as Rust variants.
/// Plain serde on both sides; card files reference declared subtypes by bare
/// name (`Forest`), which the macro-aware reader expands to the full
/// declaration before this type ever sees it.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub struct Subtype {
    pub name: Ident,
    pub types: Vec<Type>,
}
