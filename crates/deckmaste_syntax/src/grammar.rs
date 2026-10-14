//! The grammar data file: Schemas and Productions as authored, before binding.

use std::collections::BTreeMap;
use std::path::Path;

use serde::Deserialize;

use crate::LoadError;
use crate::frames::SlotKind;

/// An authored grammar data file.
#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Grammar {
    /// The Category a whole input is parsed as.
    pub root: String,
    /// Finite feature domains by feature name. The Gap feature is built in.
    #[serde(default)]
    pub features: BTreeMap<String, Vec<String>>,
    pub categories: BTreeMap<String, CategoryDecl>,
    /// Closed Words by grammar name, each naming one core closed-class lexeme id.
    #[serde(default)]
    pub closed_words: BTreeMap<String, String>,
    /// Lexical Classes by grammar name, each listing core closed-class lexeme ids.
    #[serde(default)]
    pub lexical_classes: BTreeMap<String, Vec<String>>,
    /// The Category that fills each kind of Frame Slot.
    #[serde(default)]
    pub slot_categories: BTreeMap<SlotKind, String>,
    #[serde(default)]
    pub productions: Vec<ProductionDecl>,
    #[serde(default)]
    pub schemas: Vec<SchemaDecl>,
}

impl Grammar {
    /// # Errors
    /// Malformed RON.
    pub fn from_ron_str(text: &str) -> Result<Self, LoadError> {
        ron::from_str(text).map_err(|error| LoadError::Format(error.to_string()))
    }

    /// # Errors
    /// Unreadable file or malformed RON.
    pub fn from_path(path: &Path) -> Result<Self, LoadError> {
        let text = std::fs::read_to_string(path)
            .map_err(|error| LoadError::Format(format!("{}: {error}", path.display())))?;
        Self::from_ron_str(&text)
    }
}

/// A syntactic Category and the features its summary carries.
#[derive(Debug, Clone, Default, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CategoryDecl {
    #[serde(default)]
    pub features: Vec<String>,
    /// Whether a Constituent of this Category may contain an unfilled Gap.
    #[serde(default)]
    pub gap: bool,
}

/// One authored Production.
#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ProductionDecl {
    pub name: String,
    pub lhs: String,
    pub rhs: Vec<SymbolDecl>,
    #[serde(default)]
    pub equations: Vec<Equation>,
    #[serde(default = "default_cost")]
    pub cost: u32,
    /// Children through which a Gap may pass to the mother.
    #[serde(default)]
    pub transparent: Vec<String>,
}

pub(crate) fn default_cost() -> u32 {
    1
}

/// One right-hand-side element. Every element but `Punct` names its child.
#[derive(Debug, Clone, Deserialize)]
pub enum SymbolDecl {
    /// A Constituent of a Category.
    Cat(String, String),
    /// Any word of a lexical Category.
    Lex(String, deckmaste_lexical::Category),
    /// The Closed Word of that grammar name.
    Word(String, String),
    /// Any member of the Lexical Class of that grammar name.
    Class(String, String),
    /// Literal punctuation: it may decide an analysis but is not stored in a Reading.
    Punct(char),
}

/// A feature equation over `$.Feature` (the mother) or `child.Feature` paths.
/// A child named `head` implicitly agrees with the mother on every feature both carry.
#[derive(Debug, Clone, Deserialize)]
pub enum Equation {
    Agree(String, String),
    Require(String, String),
}

/// An instance of one of the engine's built-in Schema kinds.
#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub enum SchemaDecl {
    /// Frame-driven phrases headed by every lexeme of a lexical Category.
    Valence {
        head: deckmaste_lexical::Category,
        phrase: String,
        #[serde(default)]
        cost: u32,
        /// Derive passive phrases from active verb frames, naming the Closed
        /// Word that marks the Internalised Complement (verbs only).
        #[serde(default)]
        passive: Option<String>,
    },
    /// Unselected attachment of an adjunct Category to a host.
    Adjunct {
        host: String,
        adjunct: String,
        side: Side,
        host_kind: HostKind,
        #[serde(default = "default_cost")]
        cost: u32,
    },
    /// Binary and serial coordination of a Category, with Right Node Raising
    /// when the Category may contain a Gap.
    Coordination {
        category: String,
        coordinators: String,
        #[serde(default = "default_cost")]
        cost: u32,
    },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
pub enum Side {
    Before,
    After,
}

/// The kind of host an Adjunct schema attaches to.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Ord, PartialOrd, Deserialize)]
pub enum HostKind {
    Nominal,
    Predicate,
}
