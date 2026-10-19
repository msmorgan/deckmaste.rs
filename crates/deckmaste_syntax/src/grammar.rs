//! The grammar data file: Schemas and Productions as authored, before binding.

use std::collections::BTreeMap;
use std::path::Path;

use dpsi::Ident;
use serde::Deserialize;

use crate::LoadError;
use crate::frames::SlotKind;

/// An authored grammar data file.
#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Grammar {
    /// The Category a whole input is parsed as.
    pub root: Ident,
    /// Finite feature domains by feature name. The Gap feature is built in.
    #[serde(default)]
    pub features: BTreeMap<Ident, Vec<Ident>>,
    pub categories: BTreeMap<Ident, CategoryDecl>,
    /// Closed Words by grammar name, each naming one core closed-class lexeme
    /// id.
    #[serde(default)]
    pub closed_words: BTreeMap<Ident, Ident>,
    /// Lexical Classes by grammar name, each listing core closed-class lexeme
    /// ids.
    #[serde(default)]
    pub lexical_classes: BTreeMap<Ident, Vec<Ident>>,
    /// Classes whose membership is declared by the lexical environment.
    #[serde(default)]
    pub typed_classes: Vec<Ident>,
    /// The Category that fills each kind of Frame Slot.
    #[serde(default)]
    pub slot_categories: BTreeMap<SlotKind, Ident>,
    #[serde(default)]
    pub productions: Vec<ProductionDecl>,
    #[serde(default)]
    pub schemas: Vec<SchemaDecl>,
    /// Correlative pairs: a marker (a Closed Word or Lexical Class) that may
    /// precede the first conjunct of a coordination joined by the paired
    /// Closed Word (*both … and*, *either … or*, *neither … nor*).
    #[serde(default)]
    pub correlatives: Vec<(Ident, Ident)>,
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
    /// The grammatical Category represented by this restricted production set.
    #[serde(default)]
    pub category: Option<Ident>,
    #[serde(default)]
    pub features: Vec<Ident>,
    /// Whether a Constituent of this Category may contain an unfilled Gap.
    #[serde(default)]
    pub gap: bool,
    /// Whether this Category names a grammatical function, as `Predicative`
    /// names the predicative Complement: its head child bears that function
    /// in the same phrase too.
    #[serde(default)]
    pub function: bool,
}

/// One authored Production.
#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ProductionDecl {
    pub name: Ident,
    pub lhs: Ident,
    pub rhs: Vec<SymbolDecl>,
    #[serde(default)]
    pub equations: Vec<Equation>,
    #[serde(default = "default_cost")]
    pub cost: u32,
    /// Children that must span more than one lexical leaf.
    #[serde(default)]
    pub heavy: Vec<Ident>,
    /// Children that must be exactly one lexical leaf, alone or projected.
    #[serde(default)]
    pub light: Vec<Ident>,
    /// Children through which a Gap may pass to the mother.
    #[serde(default)]
    pub transparent: Vec<Ident>,
    /// Children that are not themselves a coordination, where a coordination
    /// one level up is the same analysis (a chain of bare imperatives).
    #[serde(default)]
    pub uncoordinated: Vec<Ident>,
}

pub(crate) fn default_cost() -> u32 {
    1
}

/// One right-hand-side element. Every element but `Punct` names its child.
#[derive(Debug, Clone, Deserialize)]
pub enum SymbolDecl {
    /// A Constituent of a Category.
    Cat(Ident, Ident),
    /// A constituent attached at its outer boundaries (for symbol sequences).
    Attached(Ident, Ident, deckmaste_lexical::Binding),
    /// A Constituent of a Category whose first atom is a free word, never a
    /// bound clitic.
    Free(Ident, Ident),
    /// A numeral in a specific notation.
    Numeral(Ident, deckmaste_lexical::Numeral),
    /// Any word of a lexical Category.
    Lex(Ident, deckmaste_lexical::Category),
    FreeLex(Ident, deckmaste_lexical::Category),
    /// The Closed Word of that grammar name.
    Word(Ident, Ident),
    /// Any member of the Lexical Class of that grammar name.
    Class(Ident, Ident),
    /// A Constituent of a Category whose preposition, if any, admits the Host
    /// Category: a Preposition Phrase that could also modify such a host.
    Hosted(Ident, Ident, HostKind),
    /// A Preposition Phrase headed by one of the named Closed Words, as a
    /// Frame Slot `PP([...])` selects one.
    Selected(Ident, Ident, Vec<Ident>),
    /// Literal punctuation: it may decide an analysis but is not stored in a
    /// Reading.
    Punct(char),
    /// Structural punctuation with explicit attachment to adjacent atoms.
    Mark(char, deckmaste_lexical::Binding),
}

/// A feature equation over `$.Feature` (the mother) or `child.Feature` paths.
/// A child named `head` implicitly agrees with the mother on every feature both
/// carry.
#[derive(Debug, Clone, Deserialize)]
pub enum Equation {
    Agree(Ident, Ident),
    Require(Ident, Ident),
    /// Excludes a value on a child without requiring an inapplicable feature.
    Exclude(Ident, Ident),
    /// Requires a child to carry the value itself: an unvalued feature, which
    /// `Require` would value, does not satisfy it.
    Carry(Ident, Ident),
}

/// An instance of one of the engine's built-in Schema kinds.
#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub enum SchemaDecl {
    /// An attributive lexical head retains its phrasal Category as a Modifier.
    Premodification {
        head: deckmaste_lexical::Category,
        modifier: Ident,
        host: Ident,
        forms: Vec<deckmaste_lexical::WordForm>,
        #[serde(default)]
        prefix: bool,
    },
    /// Catalog-declared ordered parameter patterns around a lexical head.
    Parameters {
        head: deckmaste_lexical::Category,
        phrase: Ident,
        #[serde(default)]
        binding: Option<deckmaste_lexical::Binding>,
        /// Coordinators of a pattern's final marked parameter, repeated
        /// with its marker (*protection from black and from red*).
        #[serde(default)]
        coordinators: Option<Ident>,
    },
    /// Frame-driven phrases headed by every lexeme of a lexical Category.
    Valence {
        head: deckmaste_lexical::Category,
        phrase: Ident,
        #[serde(default)]
        cost: u32,
        /// Derive passive phrases from active verb frames, naming the Closed
        /// Word that marks the Internalised Complement (verbs only).
        #[serde(default)]
        passive: Option<Ident>,
        /// Derive Adjectival Passives of this Category from verb frames whose
        /// passive leaves no Complement but the optional `by` phrase.
        #[serde(default)]
        adjectival: Option<Ident>,
        /// Adjunct category permitted before a selected PP or shifted Object.
        #[serde(default)]
        interleaved: Option<Ident>,
        #[serde(default)]
        heavy_shift: bool,
        /// `(phrase feature, slot feature)` pairs: the phrase's feature
        /// agrees with that feature of each Frame Slot child that carries it
        /// (a Preposition Phrase takes its Complement's Number).
        #[serde(default)]
        agree: Vec<(Ident, Ident)>,
    },
    /// Unselected attachment of an adjunct Category to a host.
    Adjunct {
        host: Ident,
        adjunct: Ident,
        side: Side,
        host_kind: HostKind,
        #[serde(default)]
        delimiter: Option<char>,
        #[serde(default = "default_cost")]
        cost: u32,
        /// Equations over the `head` (host) and the adjunct's function child.
        #[serde(default)]
        equations: Vec<Equation>,
    },
    /// Binary and serial coordination of a Category, with Right Node Raising
    /// when the Category may contain a Gap.
    Coordination {
        category: Ident,
        coordinators: Ident,
        #[serde(default = "default_cost")]
        cost: u32,
        /// Features the conjuncts need not share, which the coordination
        /// leaves unvalued unless its `equations` value them.
        #[serde(default)]
        unshared: Vec<Ident>,
        /// Equations over the whole coordination (`$`): its resolved
        /// features (an *and*-coordination of Noun Phrases is plural).
        #[serde(default)]
        equations: Vec<Equation>,
        /// Features the coordination takes from its last conjunct, which the
        /// others need not share (proximity: *one or more cards*).
        #[serde(default)]
        proximate: Vec<Ident>,
        /// Punctuation that may also precede the coordinator of a binary
        /// coordination (two independent clauses joined by *, and*).
        #[serde(default)]
        delimiter: Option<char>,
    },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
pub enum Side {
    Before,
    After,
}

/// The kind of host an Adjunct schema attaches to.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Ord, PartialOrd)]
pub enum HostKind {
    Nominal,
    Predicate,
    /// A Clause or line that a comma-set Supplement is anchored to (CGEL,
    /// Ch. 15 §5.1, p. 1350).
    Supplement,
}
