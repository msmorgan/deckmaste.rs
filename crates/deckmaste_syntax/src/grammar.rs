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
    /// Equations on every Frame Slot child of that kind in a derived valence
    /// phrase, over the child `slot`: what fills the slot as a Complement
    /// (a Noun Phrase Complement is never nominative).
    #[serde(default)]
    pub slot_equations: BTreeMap<SlotKind, Vec<Equation>>,
    /// `(Category, feature, value)`: a dependent of that Category carrying
    /// that value attaches high by its Attachment Class, as a comma-set
    /// Supplement does (`docs/decisions/english-v4.md` §4.3): it counts its
    /// distance from its host's head reversed.
    #[serde(default)]
    pub high_attachment: Vec<(Ident, Ident, Ident)>,
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
    /// The Gap kinds a Constituent of this gap-capable Category never
    /// contains, as a non-finite Complement never passes up a predicative
    /// Gap (*can't be __*).
    #[serde(default)]
    pub closed_gaps: Vec<Ident>,
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
    /// A child that does not end, through its head line, in a Preposition
    /// Phrase dependent that this Production, or a sibling Production with
    /// the same children up to and including it, selects as a `PP`
    /// Complement right after it: the phrase is that Complement, never the
    /// child's postmodifier (an existential's Extension after its Displaced
    /// Subject, CGEL Ch. 16 §6.2.1, p. 1394). A child that no such Complement
    /// follows is a load error.
    #[serde(default)]
    pub unextended: Vec<Ident>,
    /// Children through which a Gap may pass to the mother.
    #[serde(default)]
    pub transparent: Vec<Ident>,
    /// Lexical children that cost one more where analysis did not admit
    /// their leaf through the scope of the face whose text is parsed (one of
    /// the face's own names, or a card-scoped spelling such as its Shortened
    /// Name [CR#201.5c]): another card's full name. A Production Cost
    /// preference, never an admission condition.
    #[serde(default)]
    pub scoped: Vec<Ident>,
    /// Children that are not themselves a coordination, where a coordination
    /// one level up is the same analysis (a chain of bare imperatives).
    #[serde(default)]
    pub uncoordinated: Vec<Ident>,
    /// Children that are a coordination delimited by punctuation before its
    /// coordinator (two independent clauses joined by *, and*) only within a
    /// comma-set post-dependent (*, except [A, and B]*): the mother carries
    /// it up through the phrases it is the Complement of to that dependent.
    /// Nor are they a Clause with a comma-set post-adjunct (a preposition's
    /// Clause Complement: *when [this creature dies], if …*).
    #[serde(default)]
    pub undelimited: Vec<Ident>,
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
    /// Excludes a combination of values: it fails only when every listed
    /// child carries its listed value. In a Coordination schema it is over
    /// `first` and `last`, the last conjunct and the one before it.
    ExcludeTogether(Vec<(Ident, Ident)>),
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
        /// Equations on the modifier one of the `forms` heads, over its
        /// lexical child `head` (a past participle that a declared Adjective
        /// spells is not one).
        #[serde(default)]
        equations: Vec<(deckmaste_lexical::WordForm, Vec<Equation>)>,
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
        /// The feature, carried by the phrase, that names the Slot Kind of
        /// its first Frame Slot: a non-verbal head's bare phrase names
        /// `Intransitive`, and a stranded Preposition Phrase the Slot Kind of
        /// the Complement it strands.
        #[serde(default)]
        complement_kind: Option<Ident>,
        /// Equations on every active phrase of a frame, over `$` and the
        /// lexical child `head` (a verb's Voice).
        #[serde(default)]
        active: Vec<Equation>,
        /// Derive passive phrases from active verb frames, naming the Closed
        /// Word that marks the Internalised Complement (verbs only).
        #[serde(default)]
        passive: Option<Ident>,
        /// Equations on every derived verbal passive, over `$` and the lexical
        /// child `head`: its Voice and its past participle. Required with
        /// `passive`.
        #[serde(default)]
        verbal_passive: Vec<Equation>,
        /// Equations on every derived prepositional passive, over `$` and the
        /// lexical child `head` (*each card looked at*). Required with
        /// `passive`.
        #[serde(default)]
        prepositional_passive: Vec<Equation>,
        /// Equations on the Internalised Complement of every derived
        /// passive, over the child `internalised`: what its Preposition
        /// Phrase may contain (CGEL, Ch. 16 §10.1.1, p. 1428).
        #[serde(default)]
        internalised: Vec<Equation>,
        /// Derive Adjectival Passives of this Category from verb frames whose
        /// passive leaves no Complement but the optional `by` phrase.
        #[serde(default)]
        adjectival: Option<Ident>,
        /// Equations on every derived Adjectival Passive, over `$` and the
        /// lexical child `head`. Required with `adjectival`.
        #[serde(default)]
        adjectival_passive: Vec<Equation>,
        /// Adjunct category permitted before a selected PP or shifted Object.
        #[serde(default)]
        interleaved: Option<Ident>,
        #[serde(default)]
        heavy_shift: bool,
        /// Function Category of a Modifier postposed out of the Object past
        /// the selected Preposition Phrase Complements after it, at the end
        /// of the phrase (*put a number of counters [on it] [equal to …]*;
        /// CGEL, Ch. 12 §4, p. 1066). It bears Modifier in the Object too,
        /// and agrees with the Object in each feature they both carry.
        #[serde(default)]
        postposed: Option<Ident>,
        /// A non-verbal head that declares a Frame realizes one of them in
        /// this phrase: only a head declaring none, or an Intransitive one,
        /// forms its bare phrase.
        #[serde(default)]
        obligatory: bool,
        /// `(phrase feature, slot feature)` pairs: the phrase's feature
        /// agrees with that feature of each Frame Slot child that carries it
        /// (a Preposition Phrase takes its Complement's Number).
        #[serde(default)]
        agree: Vec<(Ident, Ident)>,
        /// Coordinators of complement clusters: a verb with two or more
        /// Frame Slots heads a coordination of their sequences (*put one of
        /// them into your hand and the rest into your graveyard*; CGEL,
        /// Ch. 15 §4.3, p. 1341).
        #[serde(default)]
        clusters: Option<Ident>,
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
        /// Features every conjunct but the last shares, which the
        /// coordination resolves with its last conjunct's: their common value,
        /// or the listed one when they differ (an *and*-coordination of Noun
        /// Phrases is determined only when every conjunct is). A conjunct
        /// without a value does not decide it.
        #[serde(default)]
        resolve: Vec<(Ident, Ident)>,
        /// Punctuation that may also precede the coordinator of a binary
        /// coordination (two independent clauses joined by *, and*).
        #[serde(default)]
        delimiter: Option<char>,
        /// The binary coordination exists only with the delimiter (a
        /// declarative and then an imperative: *…, and put …*).
        #[serde(default)]
        delimited: bool,
        /// Whether serial coordinations are derived: by default for like
        /// conjuncts, not for unlike ones.
        #[serde(default)]
        serial: Option<bool>,
        /// The Category of the conjuncts, when the coordination is of a
        /// function its conjuncts fill (a depictive *tapped and attacking*).
        #[serde(default)]
        conjunct: Option<Ident>,
        /// A feature whose value the two conjuncts of a binary coordination
        /// must not share: a coordination of unlike categories in one
        /// function (CGEL, Ch. 15 §3.2, pp. 1326–1329); serial ones, derived
        /// only with `serial`, differ first to last.
        #[serde(default)]
        unlike: Option<Ident>,
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
