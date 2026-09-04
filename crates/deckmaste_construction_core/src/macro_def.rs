//! Consumer-owned metadata and normalized rows for semantic macro definitions.
//!
//! Source files are ordinary [`macro_ron::MacroDef`] values. Declaration
//! shorthands such as `KeywordAction(...)` are meta-macros expanded by the
//! same reader; this module owns only the English-v2 metadata and the
//! normalized parser boundary built from it.

use std::collections::HashMap;
use std::collections::HashSet;
use std::fmt;
use std::path::Path;
use std::path::PathBuf;

use ron::value::RawValue;
use serde::Deserialize;
use serde::Serialize;
use serde::de::DeserializeSeed;
use serde::de::Deserializer;
use serde::de::EnumAccess;
use serde::de::Error as _;
use serde::de::Visitor;
use serde::ser::Error as _;
use serde::ser::Serializer;

fn macro_options() -> ron::Options {
    ron::Options::default().with_default_extension(
        ron::extensions::Extensions::IMPLICIT_SOME
            | ron::extensions::Extensions::UNWRAP_VARIANT_NEWTYPES,
    )
}

const DECLARATION_META_MACROS: &[&str] = &[
    include_str!("../../../plugins/builtin_v2/macros/meta/AbilityWord.ron"),
    include_str!("../../../plugins/builtin_v2/macros/meta/CounterKind.ron"),
    include_str!("../../../plugins/builtin_v2/macros/meta/Designation.ron"),
    include_str!("../../../plugins/builtin_v2/macros/meta/FlavorWord.ron"),
    include_str!("../../../plugins/builtin_v2/macros/meta/KeywordAbility.ron"),
    include_str!("../../../plugins/builtin_v2/macros/meta/KeywordAction.ron"),
    include_str!("../../../plugins/builtin_v2/macros/meta/Subtype.ron"),
    include_str!("../../../plugins/builtin_v2/macros/meta/TurnPart.ron"),
    include_str!("../../../plugins/builtin_v2/macros/meta/Type.ron"),
];

fn declaration_reader() -> Result<macro_ron::MacroSet, String> {
    let mut kinds = macro_ron::KindSet::new();
    kinds.add(macro_ron::Kind::new("Macro"));
    let mut reader = macro_ron::MacroSet::new(kinds).with_options(macro_options());
    for source in DECLARATION_META_MACROS {
        let definition: macro_ron::MacroDef = reader
            .read_str(source)
            .map_err(|error| format!("invalid builtin declaration meta-macro: {error}"))?;
        reader
            .insert(&definition)
            .map_err(|error| format!("invalid builtin declaration meta-macro: {error}"))?;
    }
    Ok(reader)
}

/// Builds the ordinary macro reader with the declaration meta-macros in
/// scope. Providers that need authored metadata in addition to normalized
/// rows use this same reader rather than a parallel source schema.
///
/// # Errors
///
/// Returns an error if an embedded declaration meta-macro is invalid.
pub fn declaration_macro_set() -> Result<macro_ron::MacroSet, String> {
    declaration_reader()
}

/// English-v2 metadata attached to an ordinary macro definition.
#[derive(Debug, Clone, Default, PartialEq, Eq, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Metadata {
    pub spelling: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub grammar: Option<Grammar>,
    /// Noun-attachment facts declared once by each noun-bearing declaration
    /// class and inherited by every declaration in that class.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub noun_class: Option<NounClassSemantics>,
    /// Only subtype declarations carry a category; it forms part of their
    /// category-safe identity after normalization.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub category: Option<SubtypeCategory>,
}

/// The noun-attachment facts shared by one declaration class.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct NounClassSemantics {
    pub locative_temporal_license: NounLocativeTemporalLicense,
    pub relationality: NounRelationality,
}

/// The locative or temporal attachment family licensed by a noun class.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub enum NounLocativeTemporalLicense {
    Unlicensed,
    InLicensed,
    OnLicensed,
    InOrOnEdgeLicensed,
    ObjectAttachmentLicensed,
    TemporalLicensed,
}

/// Whether a noun class licenses an `of` complement.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub enum NounRelationality {
    NonRelational,
    QualifiedRelational,
    DeterminedRelational,
    Relational,
}

/// The open registry family in which a declaration name is unique.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd)]
pub enum DeclarationKind {
    KeywordAction,
    KeywordAbility,
    AbilityWord,
    FlavorWord,
    Subtype(SubtypeCategory),
    Type,
    TurnPart,
    CounterKind,
    Designation,
}

impl fmt::Display for DeclarationKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DeclarationKind::KeywordAction => f.write_str("keyword action"),
            DeclarationKind::KeywordAbility => f.write_str("keyword ability"),
            DeclarationKind::AbilityWord => f.write_str("ability word"),
            DeclarationKind::FlavorWord => f.write_str("flavor word"),
            DeclarationKind::Subtype(category) => write!(f, "{category} subtype"),
            DeclarationKind::Type => f.write_str("type"),
            DeclarationKind::TurnPart => f.write_str("turn part"),
            DeclarationKind::CounterKind => f.write_str("counter kind"),
            DeclarationKind::Designation => f.write_str("designation"),
        }
    }
}

/// Subtype categories currently representable by the semantic model.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Deserialize, Ord, PartialOrd, Serialize)]
pub enum SubtypeCategory {
    Artifact,
    Battle,
    Creature,
    Enchantment,
    Land,
    Planeswalker,
    Spell,
}

impl fmt::Display for SubtypeCategory {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SubtypeCategory::Artifact => f.write_str("artifact"),
            SubtypeCategory::Battle => f.write_str("battle"),
            SubtypeCategory::Creature => f.write_str("creature"),
            SubtypeCategory::Enchantment => f.write_str("enchantment"),
            SubtypeCategory::Land => f.write_str("land"),
            SubtypeCategory::Planeswalker => f.write_str("planeswalker"),
            SubtypeCategory::Spell => f.write_str("spell"),
        }
    }
}

/// One normalized positional semantic parameter type.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd)]
pub enum ParameterType {
    Ability,
    Amount,
    Condition,
    Cost,
    Power,
    Quality,
    Subject,
    Toughness,
}

impl ParameterType {
    /// Constructs a parameter type from its declaration spelling.
    ///
    /// # Errors
    /// If `name` is not in the closed v2 semantic parameter vocabulary.
    pub fn new(name: impl Into<String>) -> Result<Self, String> {
        let name = name.into();
        match name.as_str() {
            "Ability" => Ok(Self::Ability),
            "Amount" => Ok(Self::Amount),
            "Condition" => Ok(Self::Condition),
            "Cost" => Ok(Self::Cost),
            "Power" => Ok(Self::Power),
            "Quality" => Ok(Self::Quality),
            "Subject" => Ok(Self::Subject),
            "Toughness" => Ok(Self::Toughness),
            _ => Err(format!("unknown parameter type `{name}`")),
        }
    }

    #[must_use]
    pub fn as_str(&self) -> &str {
        match self {
            Self::Ability => "Ability",
            Self::Amount => "Amount",
            Self::Condition => "Condition",
            Self::Cost => "Cost",
            Self::Power => "Power",
            Self::Quality => "Quality",
            Self::Subject => "Subject",
            Self::Toughness => "Toughness",
        }
    }
}

/// Parameter families deliberately not admitted by the keyword-line grammar.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd)]
pub enum UnsupportedKeywordParameterClass {
    Ability,
    Condition,
    CostPowerToughness,
}

/// The closed relationship between a keyword declaration and its line codec.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd)]
pub enum KeywordParameterClass {
    Nullary,
    Amount,
    Cost,
    AmountCost,
    Quality,
    QualityCost,
    Subject,
    Unsupported(UnsupportedKeywordParameterClass),
}

/// Closed grammar recipes accepted from a v2 declaration.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub enum Grammar {
    Verb {
        bare: String,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        bare_onset: Option<Onset>,
        #[serde(default, skip_serializing_if = "DerivedSurface::is_derived")]
        third_person: DerivedSurface,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        third_person_onset: Option<Onset>,
        #[serde(default, skip_serializing_if = "DerivedSurface::is_derived")]
        participle: DerivedSurface,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        participle_onset: Option<Onset>,
        frame_set: VerbFrameSet,
    },
    Noun {
        singular: String,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        singular_onset: Option<Onset>,
        #[serde(default, skip_serializing_if = "DerivedSurface::is_derived")]
        plural: DerivedSurface,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        plural_onset: Option<Onset>,
    },
    FixedTerm {
        surface: String,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        onset: Option<Onset>,
    },
    FixedClause {
        surface: String,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        onset: Option<Onset>,
    },
    FixedKeyword {
        surface: String,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        onset: Option<Onset>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        participial_adjective: Option<ParticipialAdjectiveGrammar>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        block_label: Option<BlockLabelGrammar>,
    },
}

/// A supplemental participial-adjective use of a fixed keyword declaration.
///
/// The fixed keyword remains the declaration's primary grammar row. This
/// separate surface is for adjectival positions such as `enchanted creature`,
/// so it can never scan as a keyword line.
#[derive(Debug, Clone, Default, PartialEq, Eq, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ParticipialAdjectiveGrammar {
    /// Omission derives the English participle from the fixed keyword surface.
    #[serde(default, skip_serializing_if = "DerivedSurface::is_derived")]
    pub surface: DerivedSurface,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub onset: Option<Onset>,
}

/// A supplemental document block-label use of a fixed keyword declaration.
///
/// This spelling is distinct from the declaration's ordinary keyword-line
/// surface and is consumed only by a block-label grammar position.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct BlockLabelGrammar {
    pub surface: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub onset: Option<Onset>,
}

/// The nominal-number selection licensed by one Determinative realization.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub enum DeterminativeNumberLicense {
    SingularOnly,
    PluralOnly,
    Both,
}

/// The kind of nominal selected by one Determinative realization.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub enum DeterminativeNominalLicense {
    AnyNominal,
    CountNominal,
    BareSingularNoun,
    MassOrPluralCount,
}

/// Whether a Determinative lemma may serve as a fused partitive head.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub enum DeterminativeFusedHeadLicense {
    NominalOnly,
    PartitiveOnly,
    FusedHead,
    PluralPredeterminer,
}

/// The phrase-number condition of one Determinative surface realization.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub enum DeterminativePhraseNumber {
    Singular,
    Plural,
}

/// The effective initial sound of one complete realized surface.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Deserialize, Ord, PartialOrd, Serialize)]
pub enum Onset {
    Consonant,
    Vowel,
}

/// The authored state of a form supplied by a dumb morphology recipe.
///
/// Omission is [`Derived`](Self::Derived), a string is a replacement, and
/// `Unavailable` suppresses the derived row.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub enum DerivedSurface {
    #[default]
    Derived,
    Override(String),
    Unavailable,
}

impl DerivedSurface {
    fn is_derived(&self) -> bool {
        matches!(self, Self::Derived)
    }
}

impl<'de> Deserialize<'de> for DerivedSurface {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let raw = Box::<RawValue>::deserialize(deserializer)?;
        let source = raw.get_ron().trim();
        if source == "Unavailable" {
            return Ok(Self::Unavailable);
        }
        macro_options()
            .from_str::<String>(source)
            .map(Self::Override)
            .map_err(D::Error::custom)
    }
}

impl Serialize for DerivedSurface {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            DerivedSurface::Derived => Err(S::Error::custom(
                "a derived surface is represented by omitting its field",
            )),
            DerivedSurface::Override(surface) => surface.serialize(serializer),
            DerivedSurface::Unavailable => {
                serializer.serialize_unit_variant("DerivedSurface", 0, "Unavailable")
            }
        }
    }
}

/// One ordered lexical schema for the complements and fixed markers a verb licenses.
pub type VerbFrame = Vec<CustomTailAtom>;

/// The grammatical Verb Frames owned by one verb definition.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub enum VerbFrameSet {
    Intransitive,
    Transitive,
    MeasureComplement,
    Custom {
        frames: Vec<VerbFrame>,
    },
    /// The wrapped Verb Frame Set admits a post-head predicate adjunct.
    AdjunctLicensed(Box<VerbFrameSet>),
    /// The wrapped Verb Frame Set admits only a nonprepositional post-head adjunct.
    NonprepositionalAdjunctLicensed(Box<VerbFrameSet>),
}

impl VerbFrameSet {
    /// Returns the Verb Frame Set independently of its adjunct licence.
    #[must_use]
    pub fn frame_set(&self) -> &Self {
        match self {
            Self::AdjunctLicensed(frame_set) | Self::NonprepositionalAdjunctLicensed(frame_set) => {
                frame_set.frame_set()
            }
            frame_set => frame_set,
        }
    }

    /// Returns whether this Verb Frame Set admits a post-head predicate adjunct.
    #[must_use]
    pub fn prepositional_adjunct_licensed(&self) -> bool {
        matches!(self, Self::AdjunctLicensed(_))
    }

    /// Returns whether this Verb Frame Set admits a nonprepositional predicate adjunct.
    #[must_use]
    pub fn nonprepositional_adjunct_licensed(&self) -> bool {
        matches!(
            self,
            Self::AdjunctLicensed(_) | Self::NonprepositionalAdjunctLicensed(_)
        )
    }
}

/// The complete serialized atom vocabulary for a custom verb tail.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub enum CustomTailAtom {
    Literal(String),
    Lex(String, String),
    Amount,
    ObjectNounPhrase,
    PredicativeComplement,
}

/// A category-safe declaration name.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Ord, PartialOrd)]
pub struct DeclarationIdentity {
    kind: DeclarationKind,
    name: String,
}

impl DeclarationIdentity {
    /// Constructs one owned identity in its declaration-kind namespace.
    #[must_use]
    pub fn new(kind: DeclarationKind, name: impl Into<String>) -> Self {
        Self {
            kind,
            name: name.into(),
        }
    }

    #[must_use]
    pub fn kind(&self) -> DeclarationKind {
        self.kind
    }

    #[must_use]
    pub fn name(&self) -> &str {
        &self.name
    }
}

impl fmt::Display for DeclarationIdentity {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} `{}`", self.kind, self.name)
    }
}

/// Source provenance retained on every normalized row.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SourceProvenance {
    path: PathBuf,
}

impl SourceProvenance {
    #[must_use]
    pub fn path(&self) -> &Path {
        &self.path
    }
}

/// A validated declaration plus its optional normalized grammar row.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NormalizedDeclaration {
    identity: DeclarationIdentity,
    params: Option<Vec<ParameterType>>,
    keyword_parameter_class: Option<KeywordParameterClass>,
    spelling: Vec<SpellingPart>,
    grammar: Option<GrammarRow>,
    noun_class: Option<NounClassSemantics>,
    body: Option<Box<RawValue>>,
    provenance: SourceProvenance,
}

impl NormalizedDeclaration {
    #[must_use]
    pub fn identity(&self) -> &DeclarationIdentity {
        &self.identity
    }

    #[must_use]
    pub fn params(&self) -> Option<&[ParameterType]> {
        self.params.as_deref()
    }

    /// Returns the keyword-line codec class for an explicitly signed keyword.
    #[must_use]
    pub fn keyword_parameter_class(&self) -> Option<KeywordParameterClass> {
        self.keyword_parameter_class
    }

    #[must_use]
    pub fn spelling(&self) -> &[SpellingPart] {
        &self.spelling
    }

    #[must_use]
    pub fn grammar(&self) -> Option<&GrammarRow> {
        self.grammar.as_ref()
    }

    #[must_use]
    pub fn noun_class(&self) -> Option<NounClassSemantics> {
        self.noun_class
    }

    #[must_use]
    pub fn body(&self) -> Option<&RawValue> {
        self.body.as_deref()
    }

    #[must_use]
    pub fn provenance(&self) -> &SourceProvenance {
        &self.provenance
    }

    /// Whether this declaration has enough semantic information to become a
    /// spelling frame. Grammar-only nursery records deliberately return false.
    #[must_use]
    pub fn is_graduated(&self) -> bool {
        self.params.is_some() && self.body.is_some()
    }
}

/// One literal or positional hole in a checked semantic spelling frame.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SpellingPart {
    Literal(String),
    Param(usize),
}

/// The parser-facing normalized grammar contract.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GrammarRow {
    recipe: GrammarRecipe,
    surfaces: Vec<RealizedSurface>,
    participial_adjective: Option<RealizedSurface>,
    block_label: Option<RealizedSurface>,
}

impl GrammarRow {
    #[must_use]
    pub fn recipe(&self) -> &GrammarRecipe {
        &self.recipe
    }

    #[must_use]
    pub fn surfaces(&self) -> &[RealizedSurface] {
        &self.surfaces
    }

    /// Returns the declaration's supplemental participial-adjective surface.
    #[must_use]
    pub fn participial_adjective(&self) -> Option<&RealizedSurface> {
        self.participial_adjective.as_ref()
    }

    /// Returns the declaration's supplemental document block-label surface.
    #[must_use]
    pub fn block_label(&self) -> Option<&RealizedSurface> {
        self.block_label.as_ref()
    }
}

/// Closed recipe information retained after surface sealing.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum GrammarRecipe {
    Verb { frame_set: VerbFrameSet },
    Noun,
    FixedTerm,
    FixedClause,
    FixedKeyword,
}

impl GrammarRecipe {
    /// Returns the closed parser position occupied by this recipe.
    #[must_use]
    pub const fn position(&self) -> GrammarPosition {
        match self {
            Self::Verb { .. } => GrammarPosition::Verb,
            Self::Noun => GrammarPosition::Noun,
            Self::FixedTerm => GrammarPosition::FixedTerm,
            Self::FixedClause => GrammarPosition::FixedClause,
            Self::FixedKeyword => GrammarPosition::FixedKeyword,
        }
    }
}

/// The closed grammatical position in which a declaration surface can scan.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd)]
pub enum GrammarPosition {
    Verb,
    Noun,
    FixedTerm,
    FixedClause,
    FixedKeyword,
}

/// The grammatical feature attached to a realized surface.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd)]
pub enum SurfaceFeature {
    Bare,
    ThirdPersonSingular,
    Participle,
    Singular,
    Plural,
    Fixed,
    BlockLabel,
}

/// One complete scan/render surface row.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RealizedSurface {
    feature: SurfaceFeature,
    text: String,
    onset: Onset,
    onset_override: Option<Onset>,
}

impl RealizedSurface {
    #[must_use]
    pub fn feature(&self) -> SurfaceFeature {
        self.feature
    }

    #[must_use]
    pub fn text(&self) -> &str {
        &self.text
    }

    /// Returns the frozen effective onset used by every downstream consumer.
    #[must_use]
    pub fn onset(&self) -> Onset {
        self.onset
    }

    /// Returns the optional authored override retained for provenance.
    #[must_use]
    pub fn onset_override(&self) -> Option<Onset> {
        self.onset_override
    }
}

/// An owned in-memory declaration source, used by providers and tests.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DeclarationSource {
    pub path: PathBuf,
    pub source: String,
}

impl DeclarationSource {
    #[must_use]
    pub fn new(path: impl Into<PathBuf>, source: impl Into<String>) -> Self {
        Self {
            path: path.into(),
            source: source.into(),
        }
    }
}

/// A one-based source position for a validation diagnostic.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SourcePosition {
    pub line: usize,
    pub column: usize,
}

impl fmt::Display for SourcePosition {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}:{}", self.line, self.column)
    }
}

/// Parser-authenticated positions for every authored value used by
/// validation. The map is intentionally owned and short-lived: callers keep
/// the normalized declaration, while readers retain this only long enough to
/// report cross-source and builtin-path failures.
struct ValidationSourceMap {
    declaration: SourcePosition,
    category: Option<SourcePosition>,
    name: SourcePosition,
    params: Option<SourcePosition>,
    spelling: SourcePosition,
    grammar: Option<GrammarSourceMap>,
    body: Option<SourcePosition>,
}

enum GrammarSourceMap {
    Verb {
        bare: SourcePosition,
        third_person: Option<SourcePosition>,
        participle: Option<SourcePosition>,
        frame_set: SourcePosition,
    },
    Noun {
        singular: SourcePosition,
        plural: Option<SourcePosition>,
    },
    Fixed {
        surface: SourcePosition,
        participial_adjective_surface: Option<SourcePosition>,
        block_label_surface: Option<SourcePosition>,
    },
}

#[derive(Deserialize)]
enum DiagnosticInvocation<'a> {
    KeywordAction(#[serde(borrow)] DiagnosticFields<'a>),
    KeywordAbility(#[serde(borrow)] DiagnosticFields<'a>),
    AbilityWord(#[serde(borrow)] DiagnosticFields<'a>),
    FlavorWord(#[serde(borrow)] DiagnosticFields<'a>),
    Subtype(#[serde(borrow)] DiagnosticSubtype<'a>),
    Type(#[serde(borrow)] DiagnosticFields<'a>),
    TurnPart(#[serde(borrow)] DiagnosticFields<'a>),
    CounterKind(#[serde(borrow)] DiagnosticFields<'a>),
    Designation(#[serde(borrow)] DiagnosticFields<'a>),
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct DiagnosticFields<'a> {
    #[serde(borrow)]
    name: &'a RawValue,
    #[serde(default, borrow)]
    params: Option<&'a RawValue>,
    #[serde(borrow)]
    spelling: &'a RawValue,
    #[serde(default, borrow)]
    grammar: Option<DiagnosticGrammar<'a>>,
    #[serde(default, borrow)]
    body: Option<&'a RawValue>,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct DiagnosticSubtype<'a> {
    #[serde(borrow)]
    category: &'a RawValue,
    #[serde(borrow)]
    name: &'a RawValue,
    #[serde(default, borrow)]
    params: Option<&'a RawValue>,
    #[serde(borrow)]
    spelling: &'a RawValue,
    #[serde(default, borrow)]
    grammar: Option<DiagnosticGrammar<'a>>,
    #[serde(default, borrow)]
    body: Option<&'a RawValue>,
}

struct DiagnosticFieldValues<'a> {
    category: Option<&'a RawValue>,
    name: &'a RawValue,
    params: Option<&'a RawValue>,
    spelling: &'a RawValue,
    grammar: Option<DiagnosticGrammar<'a>>,
    body: Option<&'a RawValue>,
}

#[derive(Deserialize)]
enum DiagnosticGrammar<'a> {
    Verb {
        #[serde(borrow)]
        bare: &'a RawValue,
        #[serde(default, borrow)]
        third_person: Option<&'a RawValue>,
        #[serde(default, borrow)]
        participle: Option<&'a RawValue>,
        #[serde(borrow)]
        frame_set: &'a RawValue,
    },
    Noun {
        #[serde(borrow)]
        singular: &'a RawValue,
        #[serde(default, borrow)]
        plural: Option<&'a RawValue>,
    },
    FixedTerm {
        #[serde(borrow)]
        surface: &'a RawValue,
    },
    FixedClause {
        #[serde(borrow)]
        surface: &'a RawValue,
    },
    FixedKeyword {
        #[serde(borrow)]
        surface: &'a RawValue,
        #[serde(default, borrow)]
        participial_adjective: Option<DiagnosticParticipialAdjectiveGrammar<'a>>,
        #[serde(default, borrow)]
        block_label: Option<DiagnosticBlockLabelGrammar<'a>>,
    },
}

#[derive(Deserialize)]
struct DiagnosticParticipialAdjectiveGrammar<'a> {
    #[serde(default, borrow)]
    surface: Option<&'a RawValue>,
}

#[derive(Deserialize)]
struct DiagnosticBlockLabelGrammar<'a> {
    #[serde(borrow)]
    surface: &'a RawValue,
}

struct LeadingVariant;

impl<'de> DeserializeSeed<'de> for LeadingVariant {
    type Value = macro_ron::Ident;

    fn deserialize<D: Deserializer<'de>>(self, de: D) -> Result<Self::Value, D::Error> {
        de.deserialize_enum("", &[], self)
    }
}

impl<'de> Visitor<'de> for LeadingVariant {
    type Value = macro_ron::Ident;

    fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("a declaration variant")
    }

    fn visit_enum<A: EnumAccess<'de>>(self, data: A) -> Result<Self::Value, A::Error> {
        let (ident, _variant) = data.variant_seed(macro_ron::IdentSeed)?;
        Ok(ident)
    }
}

/// A schema or normalization failure with no filesystem transport details.
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum ValidationError {
    #[error("declaration name `{name}` is not a bare identifier")]
    InvalidName { name: String },
    #[error(
        "semantic body requires an explicit positional parameter signature; use `params: []` for a nullary declaration"
    )]
    BodyWithoutSignature,
    #[error("a semantic declaration must name exactly one supported declaration kind")]
    InvalidDeclarationKind,
    #[error("only subtype declarations may carry a subtype category")]
    UnexpectedSubtypeCategory,
    #[error("a subtype declaration requires a subtype category")]
    MissingSubtypeCategory,
    #[error("noun-bearing declaration class {kind} has no declared noun semantics")]
    MissingNounClassSemantics { kind: DeclarationKind },
    #[error("declaration class {kind} cannot carry noun semantics")]
    UnexpectedNounClassSemantics { kind: DeclarationKind },
    #[error("v2 semantic declarations require positional parameter signatures")]
    NamedParameters,
    #[error("v2 semantic declaration parameters must be plain type names")]
    DecoratedParameter,
    #[error("unknown v2 semantic parameter type `{name}`")]
    UnknownParameterType { name: String },
    #[error("keyword parameter signature [{signature}] has no consuming or deferred codec class")]
    UnsupportedKeywordParameterSignature { signature: String },
    #[error("invalid spelling: {reason}")]
    InvalidSpelling { reason: String },
    #[error("{location} references Param({index}), but the positional signature has length {len}")]
    ParamOutOfRange {
        location: &'static str,
        index: usize,
        len: usize,
    },
    #[error("invalid semantic body: {reason}")]
    InvalidBody { reason: String },
    #[error("{field} surface must be nonempty, trimmed, and single-line")]
    InvalidSurface { field: &'static str },
    #[error(
        "surface `{surface}` has no onset in the bounded pronunciation recipe; author an attested per-form override"
    )]
    UnknownOnset { surface: String },
    #[error("{field} override `{surface}` equals the dumb derived surface; omit it")]
    RedundantOverride {
        field: &'static str,
        surface: String,
    },
    #[error("Custom Verb Frame Set requires a nonempty frame set")]
    EmptyCustomVerbFrameSet,
    #[error("Custom Verb Frame Set repeats Verb Frame {frame:?}")]
    DuplicateVerbFrame { frame: VerbFrame },
    #[error("Custom Literal atoms must contain a nonempty, trimmed, single-line terminal span")]
    InvalidCustomLiteral,
    #[error(
        "spelling head `{spelling_head}` does not match this declaration's grammar head `{grammar_head}`"
    )]
    GrammarSpellingMismatch {
        spelling_head: String,
        grammar_head: String,
    },
    #[error("builtin_v2 root must be an existing directory named `builtin_v2`")]
    InvalidBuiltinRoot,
    #[error("`{relative}` is not a recognized builtin_v2 nursery declaration location")]
    UnexpectedBuiltinLocation { relative: PathBuf },
    #[error("builtin location requires {expected}, but the file declares {actual}")]
    DeclarationKindMismatch {
        expected: DeclarationKind,
        actual: DeclarationKind,
    },
    #[error(
        "builtin filename requires declaration name `{expected}`, but the file declares `{actual}`"
    )]
    DeclarationNameMismatch { expected: String, actual: String },
    #[error("duplicate declaration {identity}; first declared in `{first_path}`")]
    DuplicateIdentity {
        identity: DeclarationIdentity,
        first_path: PathBuf,
    },
}

/// A path-authenticated read or validation failure.
#[derive(Debug, thiserror::Error)]
pub enum ReadError {
    #[error("reading `{path}`: {source}")]
    Io {
        path: PathBuf,
        #[source]
        source: std::io::Error,
    },
    #[error("parsing `{path}` at {}: {source}", source.span)]
    Parse {
        path: PathBuf,
        #[source]
        source: Box<ron::error::SpannedError>,
    },
    #[error("validating `{path}` at {position}: {source}")]
    Validate {
        path: PathBuf,
        position: SourcePosition,
        #[source]
        source: ValidationError,
    },
}

impl ReadError {
    #[must_use]
    pub fn path(&self) -> &Path {
        match self {
            ReadError::Io { path, .. }
            | ReadError::Parse { path, .. }
            | ReadError::Validate { path, .. } => path,
        }
    }

    #[must_use]
    pub fn position(&self) -> Option<SourcePosition> {
        match self {
            ReadError::Io { .. } => None,
            ReadError::Parse { source, .. } => Some(SourcePosition {
                line: source.span.start.line,
                column: source.span.start.col,
            }),
            ReadError::Validate { position, .. } => Some(*position),
        }
    }

    #[must_use]
    pub fn validation(&self) -> Option<&ValidationError> {
        match self {
            ReadError::Validate { source, .. } => Some(source),
            ReadError::Io { .. } | ReadError::Parse { .. } => None,
        }
    }
}

impl ValidationSourceMap {
    fn read(path: &Path, source: &str) -> Result<Self, ReadError> {
        let declaration = leading_variant_position(path, source)?;
        let diagnostic = macro_options()
            .from_str::<DiagnosticInvocation<'_>>(source)
            .map_err(|source| ReadError::Parse {
                path: path.to_owned(),
                source: Box::new(source),
            })?;

        match diagnostic {
            DiagnosticInvocation::KeywordAction(fields)
            | DiagnosticInvocation::KeywordAbility(fields)
            | DiagnosticInvocation::AbilityWord(fields)
            | DiagnosticInvocation::FlavorWord(fields)
            | DiagnosticInvocation::Type(fields)
            | DiagnosticInvocation::TurnPart(fields)
            | DiagnosticInvocation::CounterKind(fields)
            | DiagnosticInvocation::Designation(fields) => Self::from_fields(
                path,
                source,
                declaration,
                DiagnosticFieldValues {
                    category: None,
                    name: fields.name,
                    params: fields.params,
                    spelling: fields.spelling,
                    grammar: fields.grammar,
                    body: fields.body,
                },
            ),
            DiagnosticInvocation::Subtype(subtype) => Self::from_fields(
                path,
                source,
                declaration,
                DiagnosticFieldValues {
                    category: Some(subtype.category),
                    name: subtype.name,
                    params: subtype.params,
                    spelling: subtype.spelling,
                    grammar: subtype.grammar,
                    body: subtype.body,
                },
            ),
        }
    }

    fn from_fields(
        path: &Path,
        source: &str,
        declaration: SourcePosition,
        fields: DiagnosticFieldValues<'_>,
    ) -> Result<Self, ReadError> {
        Ok(Self {
            declaration,
            category: fields
                .category
                .map(|category| raw_position(path, source, category, declaration))
                .transpose()?,
            name: raw_position(path, source, fields.name, declaration)?,
            params: fields
                .params
                .map(|params| raw_position(path, source, params, declaration))
                .transpose()?,
            spelling: raw_position(path, source, fields.spelling, declaration)?,
            grammar: fields
                .grammar
                .map(|grammar| {
                    GrammarSourceMap::from_diagnostic(path, source, declaration, &grammar)
                })
                .transpose()?,
            body: fields
                .body
                .map(|body| raw_position(path, source, body, declaration))
                .transpose()?,
        })
    }

    fn kind(&self, path: &Path, kind: DeclarationKind) -> Result<SourcePosition, ReadError> {
        match kind {
            DeclarationKind::Subtype(_) => {
                required_map_position(path, self.declaration, self.category, "Subtype.category")
            }
            _ => Ok(self.declaration),
        }
    }
}

impl GrammarSourceMap {
    fn from_diagnostic(
        path: &Path,
        source: &str,
        declaration: SourcePosition,
        diagnostic: &DiagnosticGrammar<'_>,
    ) -> Result<Self, ReadError> {
        match diagnostic {
            DiagnosticGrammar::Verb {
                bare,
                third_person,
                participle,
                frame_set,
            } => Ok(Self::Verb {
                bare: raw_position(path, source, bare, declaration)?,
                third_person: third_person
                    .map(|value| raw_position(path, source, value, declaration))
                    .transpose()?,
                participle: participle
                    .map(|value| raw_position(path, source, value, declaration))
                    .transpose()?,
                frame_set: raw_position(path, source, frame_set, declaration)?,
            }),
            DiagnosticGrammar::Noun { singular, plural } => Ok(Self::Noun {
                singular: raw_position(path, source, singular, declaration)?,
                plural: plural
                    .map(|value| raw_position(path, source, value, declaration))
                    .transpose()?,
            }),
            DiagnosticGrammar::FixedTerm { surface }
            | DiagnosticGrammar::FixedClause { surface } => Ok(Self::Fixed {
                surface: raw_position(path, source, surface, declaration)?,
                participial_adjective_surface: None,
                block_label_surface: None,
            }),
            DiagnosticGrammar::FixedKeyword {
                surface,
                participial_adjective,
                block_label,
            } => Ok(Self::Fixed {
                surface: raw_position(path, source, surface, declaration)?,
                participial_adjective_surface: participial_adjective
                    .as_ref()
                    .and_then(|grammar| grammar.surface)
                    .map(|value| raw_position(path, source, value, declaration))
                    .transpose()?,
                block_label_surface: block_label
                    .as_ref()
                    .map(|grammar| raw_position(path, source, grammar.surface, declaration))
                    .transpose()?,
            }),
        }
    }
}

fn leading_variant_position(path: &Path, source: &str) -> Result<SourcePosition, ReadError> {
    let options = macro_options();
    let mut deserializer =
        ron::Deserializer::from_str_with_options(source, &options).map_err(|source| {
            ReadError::Parse {
                path: path.to_owned(),
                source: Box::new(source),
            }
        })?;
    let ident = LeadingVariant
        .deserialize(&mut deserializer)
        .map_err(|source| ReadError::Parse {
            path: path.to_owned(),
            source: Box::new(deserializer.span_error(source)),
        })?;
    let consumed = source
        .len()
        .checked_sub(deserializer.remainder().len())
        .ok_or_else(|| ReadError::Parse {
            path: path.to_owned(),
            source: Box::new(deserializer.span_error(ron::error::Error::Message(
                "could not derive validation source map: RON parser remainder was not a declaration source subslice"
                    .to_owned(),
            ))),
        })?;
    let offset = consumed.checked_sub(ident.as_str().len()).ok_or_else(|| {
        source_map_parse_error(
            path,
            locate_offset(source, consumed),
            "top-level declaration variant was not a source subslice",
        )
    })?;
    Ok(locate_offset(source, offset))
}

fn raw_position(
    path: &Path,
    source: &str,
    raw: &RawValue,
    fallback: SourcePosition,
) -> Result<SourcePosition, ReadError> {
    let value = raw.trim().get_ron();
    let source_start = source.as_ptr() as usize;
    let value_start = value.as_ptr() as usize;
    let Some(offset) = value_start.checked_sub(source_start) else {
        return Err(source_map_parse_error(
            path,
            fallback,
            "authored value was not borrowed from its declaration source",
        ));
    };
    if offset > source.len() || value.len() > source.len() - offset {
        return Err(source_map_parse_error(
            path,
            fallback,
            "authored value extended outside its declaration source",
        ));
    }
    Ok(locate_offset(source, offset))
}

fn required_map_position(
    path: &Path,
    fallback: SourcePosition,
    position: Option<SourcePosition>,
    field: &str,
) -> Result<SourcePosition, ReadError> {
    position.ok_or_else(|| {
        source_map_parse_error(
            path,
            fallback,
            &format!("validation source map omitted required `{field}` value"),
        )
    })
}

fn source_map_parse_error(path: &Path, position: SourcePosition, reason: &str) -> ReadError {
    let position = ron::error::Position {
        line: position.line,
        col: position.column,
    };
    ReadError::Parse {
        path: path.to_owned(),
        source: Box::new(ron::error::SpannedError {
            code: ron::error::Error::Message(format!(
                "could not derive validation source map: {reason}"
            )),
            span: ron::error::Span {
                start: position,
                end: position,
            },
        }),
    }
}

/// Reads and normalizes one v2 declaration source.
///
/// # Errors
/// RON syntax errors and every schema/normalization invariant are returned
/// with `path` and a one-based source position.
pub fn read_str(
    path: impl Into<PathBuf>,
    source: &str,
) -> Result<NormalizedDeclaration, ReadError> {
    let path = path.into();
    let reader = declaration_reader().map_err(|reason| {
        source_map_parse_error(&path, SourcePosition { line: 1, column: 1 }, &reason)
    })?;
    read_mapped(&reader, path, source).map(|mapped| mapped.declaration)
}

struct MappedDeclaration {
    declaration: NormalizedDeclaration,
    source_map: ValidationSourceMap,
}

fn read_mapped(
    reader: &macro_ron::MacroSet,
    path: PathBuf,
    source: &str,
) -> Result<MappedDeclaration, ReadError> {
    let definition = reader
        .read_str::<macro_ron::MacroDef<Metadata>>(source)
        .map_err(|source| ReadError::Parse {
            path: path.clone(),
            source: Box::new(source),
        })?;
    let source_map = ValidationSourceMap::read(&path, source)?;
    let declaration = normalize(path, &definition, &source_map, reader)?;
    Ok(MappedDeclaration {
        declaration,
        source_map,
    })
}

/// Reads, path-sorts, normalizes, and identity-deduplicates owned sources.
///
/// # Errors
/// If any source is invalid or two sources declare the same category-safe
/// identity. The duplicate error points at the later path in sorted order and
/// names the first.
pub fn read_sources(
    sources: Vec<DeclarationSource>,
) -> Result<Vec<NormalizedDeclaration>, ReadError> {
    read_sources_mapped(sources).map(|declarations| {
        declarations
            .into_iter()
            .map(|mapped| mapped.declaration)
            .collect()
    })
}

fn read_sources_mapped(
    mut sources: Vec<DeclarationSource>,
) -> Result<Vec<MappedDeclaration>, ReadError> {
    let reader = declaration_reader().map_err(|reason| {
        let path = sources.first().map_or_else(
            || PathBuf::from("<declarations>"),
            |source| source.path.clone(),
        );
        source_map_parse_error(&path, SourcePosition { line: 1, column: 1 }, &reason)
    })?;
    sources.sort_by(|left, right| left.path.cmp(&right.path));
    let mut first_by_identity: HashMap<DeclarationIdentity, PathBuf> = HashMap::new();
    let mut declarations = Vec::with_capacity(sources.len());
    for source in sources {
        let declaration = read_mapped(&reader, source.path.clone(), &source.source)?;
        if let Some(first_path) = first_by_identity.get(&declaration.declaration.identity) {
            return Err(validation_error_at(
                &source.path,
                declaration.source_map.name,
                ValidationError::DuplicateIdentity {
                    identity: declaration.declaration.identity,
                    first_path: first_path.clone(),
                },
            ));
        }
        first_by_identity.insert(declaration.declaration.identity.clone(), source.path);
        declarations.push(declaration);
    }
    Ok(declarations)
}

/// Reads the committed builtin-v2 nursery through the ordinary macro reader.
///
/// This is intentionally narrow: `root` must itself be an existing directory
/// named `builtin_v2`, and only `.ron` files below `macros/stubs` are read.
/// The path fixes each file's declaration kind, subtype category, and name.
///
/// # Errors
/// On an invalid root, unreadable directory/file, unrecognized nursery path,
/// source error, path/content mismatch, or duplicate identity.
pub fn read_builtin_v2(root: impl AsRef<Path>) -> Result<Vec<NormalizedDeclaration>, ReadError> {
    let root = root.as_ref();
    if root.file_name().and_then(|name| name.to_str()) != Some("builtin_v2") || !root.is_dir() {
        return Err(validation_error_at(
            root,
            SourcePosition { line: 1, column: 1 },
            ValidationError::InvalidBuiltinRoot,
        ));
    }

    let nursery = root.join("macros").join("stubs");
    let paths = ron_files_recursive(&nursery)?;
    let mut sources = Vec::with_capacity(paths.len());
    for path in paths {
        let source = std::fs::read_to_string(&path).map_err(|source| ReadError::Io {
            path: path.clone(),
            source,
        })?;
        sources.push(DeclarationSource::new(path, source));
    }

    let declarations = read_sources_mapped(sources)?;
    for declaration in &declarations {
        let normalized = &declaration.declaration;
        let (expected_kind, expected_name) = expected_builtin_identity(&nursery, normalized)
            .map_err(|source| {
                validation_error_at(
                    &normalized.provenance.path,
                    declaration.source_map.declaration,
                    source,
                )
            })?;
        if normalized.identity.kind != expected_kind {
            return Err(validation_error_at(
                &normalized.provenance.path,
                declaration
                    .source_map
                    .kind(&normalized.provenance.path, normalized.identity.kind)?,
                ValidationError::DeclarationKindMismatch {
                    expected: expected_kind,
                    actual: normalized.identity.kind,
                },
            ));
        }
        if normalized.identity.name != expected_name {
            return Err(validation_error_at(
                &normalized.provenance.path,
                declaration.source_map.name,
                ValidationError::DeclarationNameMismatch {
                    expected: expected_name,
                    actual: normalized.identity.name.clone(),
                },
            ));
        }
    }
    Ok(declarations
        .into_iter()
        .map(|mapped| mapped.declaration)
        .collect())
}

fn normalize(
    path: PathBuf,
    definition: &macro_ron::MacroDef<Metadata>,
    source_map: &ValidationSourceMap,
    reader: &macro_ron::MacroSet,
) -> Result<NormalizedDeclaration, ReadError> {
    let name = definition.name.as_str().to_owned();
    let kind = normalized_kind(&path, source_map, definition)?;
    let params = normalized_params(&path, source_map, definition)?;
    let keyword_parameter_class = if kind == DeclarationKind::KeywordAbility {
        params
            .as_deref()
            .map(|params| normalized_keyword_parameter_class(&path, source_map, params))
            .transpose()?
    } else {
        None
    };
    let Metadata {
        spelling,
        grammar,
        noun_class,
        category: _,
    } = definition.metadata.clone();
    let noun_bearing = matches!(
        kind,
        DeclarationKind::Subtype(_)
            | DeclarationKind::Type
            | DeclarationKind::TurnPart
            | DeclarationKind::CounterKind
    );
    match (noun_bearing, noun_class) {
        (true, None) => {
            return Err(validation_error_at(
                &path,
                source_map.declaration,
                ValidationError::MissingNounClassSemantics { kind },
            ));
        }
        (false, Some(_)) => {
            return Err(validation_error_at(
                &path,
                source_map.declaration,
                ValidationError::UnexpectedNounClassSemantics { kind },
            ));
        }
        (true, Some(_)) | (false, None) => {}
    }
    let body = source_map
        .body
        .map(|_| {
            macro_options()
                .from_str::<Box<RawValue>>(definition.body())
                .map_err(|error| {
                    validation_error_at(
                        &path,
                        source_map.body.unwrap_or(source_map.declaration),
                        ValidationError::InvalidBody {
                            reason: error.to_string(),
                        },
                    )
                })
        })
        .transpose()?;

    if !is_bare_ident(&name) {
        return Err(validation_error_at(
            &path,
            source_map.name,
            ValidationError::InvalidName { name },
        ));
    }
    if body.is_some() && params.is_none() {
        return Err(validation_error_at(
            &path,
            required_map_position(&path, source_map.declaration, source_map.body, "body")?,
            ValidationError::BodyWithoutSignature,
        ));
    }

    let spelling_parts = parse_spelling(&spelling).map_err(|reason| {
        validation_error_at(
            &path,
            source_map.spelling,
            ValidationError::InvalidSpelling { reason },
        )
    })?;
    validate_spelling_params(
        &path,
        source_map.spelling,
        &spelling_parts,
        params.as_deref(),
    )?;
    validate_body_params(
        &path,
        source_map.declaration,
        source_map.body,
        definition,
        reader,
        params.as_deref(),
    )?;

    let identity = DeclarationIdentity { kind, name };
    let grammar = grammar
        .map(|grammar| {
            let grammar_map = source_map.grammar.as_ref().ok_or_else(|| {
                source_map_parse_error(
                    &path,
                    source_map.declaration,
                    "parsed grammar has no validation source map",
                )
            })?;
            normalize_grammar(
                &path,
                grammar_map,
                source_map.spelling,
                &spelling_parts,
                grammar,
            )
        })
        .transpose()?;

    Ok(NormalizedDeclaration {
        identity,
        params,
        keyword_parameter_class,
        spelling: spelling_parts,
        grammar,
        noun_class,
        body,
        provenance: SourceProvenance { path },
    })
}

fn normalized_kind(
    path: &Path,
    source_map: &ValidationSourceMap,
    definition: &macro_ron::MacroDef<Metadata>,
) -> Result<DeclarationKind, ReadError> {
    let [kind] = definition.kinds.as_slice() else {
        return Err(validation_error_at(
            path,
            source_map.declaration,
            ValidationError::InvalidDeclarationKind,
        ));
    };
    let category = definition.metadata.category;
    let kind = match kind.as_str() {
        "KeywordAction" => DeclarationKind::KeywordAction,
        "KeywordAbility" => DeclarationKind::KeywordAbility,
        "AbilityWord" => DeclarationKind::AbilityWord,
        "FlavorWord" => DeclarationKind::FlavorWord,
        "Subtype" => DeclarationKind::Subtype(category.ok_or_else(|| {
            validation_error_at(
                path,
                source_map.declaration,
                ValidationError::MissingSubtypeCategory,
            )
        })?),
        "Type" => DeclarationKind::Type,
        "TurnPart" => DeclarationKind::TurnPart,
        "CounterKind" => DeclarationKind::CounterKind,
        "Designation" => DeclarationKind::Designation,
        _ => {
            return Err(validation_error_at(
                path,
                source_map.declaration,
                ValidationError::InvalidDeclarationKind,
            ));
        }
    };
    if !matches!(kind, DeclarationKind::Subtype(_)) && category.is_some() {
        return Err(validation_error_at(
            path,
            source_map.declaration,
            ValidationError::UnexpectedSubtypeCategory,
        ));
    }
    Ok(kind)
}

fn normalized_params(
    path: &Path,
    source_map: &ValidationSourceMap,
    definition: &macro_ron::MacroDef<Metadata>,
) -> Result<Option<Vec<ParameterType>>, ReadError> {
    let Some(position) = source_map.params else {
        return Ok(None);
    };
    let macro_ron::Params::Positional(params) = &definition.params else {
        return Err(validation_error_at(
            path,
            position,
            ValidationError::NamedParameters,
        ));
    };
    params
        .iter()
        .map(|param| {
            if param.default.is_some() || param.elidable || !param.binds.is_empty() {
                return Err(validation_error_at(
                    path,
                    position,
                    ValidationError::DecoratedParameter,
                ));
            }
            ParameterType::new(param.name.as_str()).map_err(|_| {
                validation_error_at(
                    path,
                    position,
                    ValidationError::UnknownParameterType {
                        name: param.name.as_str().to_owned(),
                    },
                )
            })
        })
        .collect::<Result<Vec<_>, _>>()
        .map(Some)
}

fn normalized_keyword_parameter_class(
    path: &Path,
    source_map: &ValidationSourceMap,
    params: &[ParameterType],
) -> Result<KeywordParameterClass, ReadError> {
    use KeywordParameterClass as Class;
    use ParameterType as Type;
    use UnsupportedKeywordParameterClass as Unsupported;

    let class = match params {
        [] => Class::Nullary,
        [Type::Amount] => Class::Amount,
        [Type::Cost] => Class::Cost,
        [Type::Amount, Type::Cost] => Class::AmountCost,
        [Type::Quality] => Class::Quality,
        [Type::Quality, Type::Cost] => Class::QualityCost,
        [Type::Subject] => Class::Subject,
        [Type::Ability] => Class::Unsupported(Unsupported::Ability),
        [Type::Condition] => Class::Unsupported(Unsupported::Condition),
        [Type::Cost, Type::Power, Type::Toughness] => {
            Class::Unsupported(Unsupported::CostPowerToughness)
        }
        _ => {
            return Err(validation_error_at(
                path,
                source_map.params.unwrap_or(source_map.declaration),
                ValidationError::UnsupportedKeywordParameterSignature {
                    signature: params
                        .iter()
                        .map(ParameterType::as_str)
                        .collect::<Vec<_>>()
                        .join(", "),
                },
            ));
        }
    };
    Ok(class)
}

type NormalizedGrammarParts = (
    String,
    GrammarRecipe,
    Vec<RealizedSurface>,
    Option<RealizedSurface>,
    Option<RealizedSurface>,
);

#[derive(Clone, Copy)]
struct FixedGrammarPositions {
    surface: SourcePosition,
    participial_adjective: Option<SourcePosition>,
    block_label: Option<SourcePosition>,
}

fn normalize_grammar(
    path: &Path,
    source_map: &GrammarSourceMap,
    spelling_position: SourcePosition,
    spelling: &[SpellingPart],
    grammar: Grammar,
) -> Result<GrammarRow, ReadError> {
    let (grammar_head, recipe, surfaces, participial_adjective, block_label) =
        match (grammar, source_map) {
            (
                Grammar::Verb {
                    bare,
                    bare_onset,
                    third_person,
                    third_person_onset,
                    participle,
                    participle_onset,
                    frame_set,
                },
                GrammarSourceMap::Verb {
                    bare: bare_position,
                    third_person: third_person_position,
                    participle: participle_position,
                    frame_set: frame_set_position,
                },
            ) => {
                validate_surface(path, *bare_position, "bare", &bare)?;
                validate_frame_set(path, *frame_set_position, &frame_set)?;
                let third_person = realize_derived_surface(
                    path,
                    *bare_position,
                    *third_person_position,
                    "third_person",
                    english_verb(&bare),
                    third_person,
                )?;
                let participle = realize_derived_surface(
                    path,
                    *bare_position,
                    *participle_position,
                    "participle",
                    english_participle(&bare),
                    participle,
                )?;
                let mut surfaces = vec![RealizedSurface {
                    feature: SurfaceFeature::Bare,
                    onset: normalized_onset(path, *bare_position, &bare, bare_onset)?,
                    onset_override: bare_onset,
                    text: bare.clone(),
                }];
                if let Some(text) = third_person {
                    let position = third_person_position.unwrap_or(*bare_position);
                    surfaces.push(RealizedSurface {
                        feature: SurfaceFeature::ThirdPersonSingular,
                        onset: normalized_onset(path, position, &text, third_person_onset)?,
                        onset_override: third_person_onset,
                        text,
                    });
                }
                if let Some(text) = participle {
                    let position = participle_position.unwrap_or(*bare_position);
                    surfaces.push(RealizedSurface {
                        feature: SurfaceFeature::Participle,
                        onset: normalized_onset(path, position, &text, participle_onset)?,
                        onset_override: participle_onset,
                        text,
                    });
                }
                let recipe = GrammarRecipe::Verb { frame_set };
                (bare, recipe, surfaces, None, None)
            }
            (
                Grammar::Noun {
                    singular,
                    singular_onset,
                    plural,
                    plural_onset,
                },
                GrammarSourceMap::Noun {
                    singular: singular_position,
                    plural: plural_position,
                },
            ) => normalize_noun_grammar(
                path,
                *singular_position,
                *plural_position,
                singular,
                singular_onset,
                plural,
                plural_onset,
            )?,
            (
                Grammar::FixedTerm { surface, onset },
                GrammarSourceMap::Fixed {
                    surface: position, ..
                },
            ) => {
                normalize_fixed_grammar(path, *position, surface, onset, GrammarRecipe::FixedTerm)?
            }
            (
                Grammar::FixedClause { surface, onset },
                GrammarSourceMap::Fixed {
                    surface: position, ..
                },
            ) => normalize_fixed_grammar(
                path,
                *position,
                surface,
                onset,
                GrammarRecipe::FixedClause,
            )?,
            (
                Grammar::FixedKeyword {
                    surface,
                    onset,
                    participial_adjective,
                    block_label,
                },
                GrammarSourceMap::Fixed {
                    surface: position,
                    participial_adjective_surface,
                    block_label_surface,
                },
            ) => normalize_fixed_keyword_grammar(
                path,
                FixedGrammarPositions {
                    surface: *position,
                    participial_adjective: *participial_adjective_surface,
                    block_label: *block_label_surface,
                },
                surface,
                onset,
                participial_adjective,
                block_label,
            )?,
            _ => {
                return Err(source_map_parse_error(
                    path,
                    spelling_position,
                    "grammar recipe disagreed with its validation source map",
                ));
            }
        };

    finish_grammar_normalization(
        path,
        spelling_position,
        spelling,
        (
            grammar_head,
            recipe,
            surfaces,
            participial_adjective,
            block_label,
        ),
    )
}

fn normalize_noun_grammar(
    path: &Path,
    singular_position: SourcePosition,
    plural_position: Option<SourcePosition>,
    singular: String,
    singular_onset: Option<Onset>,
    plural: DerivedSurface,
    plural_onset: Option<Onset>,
) -> Result<NormalizedGrammarParts, ReadError> {
    validate_surface(path, singular_position, "singular", &singular)?;
    let plural = realize_derived_surface(
        path,
        singular_position,
        plural_position,
        "plural",
        english_noun(&singular),
        plural,
    )?;
    let mut surfaces = vec![RealizedSurface {
        feature: SurfaceFeature::Singular,
        onset: normalized_onset(path, singular_position, &singular, singular_onset)?,
        onset_override: singular_onset,
        text: singular.clone(),
    }];
    if let Some(text) = plural {
        let position = plural_position.unwrap_or(singular_position);
        surfaces.push(RealizedSurface {
            feature: SurfaceFeature::Plural,
            onset: normalized_onset(path, position, &text, plural_onset)?,
            onset_override: plural_onset,
            text,
        });
    }
    Ok((singular, GrammarRecipe::Noun, surfaces, None, None))
}

fn normalize_fixed_grammar(
    path: &Path,
    position: SourcePosition,
    surface: String,
    onset: Option<Onset>,
    recipe: GrammarRecipe,
) -> Result<NormalizedGrammarParts, ReadError> {
    let (head, recipe, surfaces) = fixed_grammar(path, position, surface, onset, recipe)?;
    Ok((head, recipe, surfaces, None, None))
}

fn normalize_fixed_keyword_grammar(
    path: &Path,
    positions: FixedGrammarPositions,
    surface: String,
    onset: Option<Onset>,
    participial_adjective: Option<ParticipialAdjectiveGrammar>,
    block_label: Option<BlockLabelGrammar>,
) -> Result<NormalizedGrammarParts, ReadError> {
    let (head, recipe, surfaces) = fixed_grammar(
        path,
        positions.surface,
        surface,
        onset,
        GrammarRecipe::FixedKeyword,
    )?;
    let participial_adjective = participial_adjective
        .map(|grammar| {
            normalize_participial_adjective(
                path,
                positions.surface,
                positions.participial_adjective,
                &head,
                grammar,
            )
        })
        .transpose()?;
    let block_label = block_label
        .map(|grammar| {
            normalize_block_label(path, positions.surface, positions.block_label, grammar)
        })
        .transpose()?;
    Ok((head, recipe, surfaces, participial_adjective, block_label))
}

fn finish_grammar_normalization(
    path: &Path,
    spelling_position: SourcePosition,
    spelling: &[SpellingPart],
    normalized: NormalizedGrammarParts,
) -> Result<GrammarRow, ReadError> {
    let (grammar_head, recipe, surfaces, participial_adjective, block_label) = normalized;
    let spelling_head = spelling_head(spelling);
    if spelling_head != grammar_head {
        return Err(validation_error_at(
            path,
            spelling_position,
            ValidationError::GrammarSpellingMismatch {
                spelling_head,
                grammar_head,
            },
        ));
    }

    Ok(GrammarRow {
        recipe,
        surfaces,
        participial_adjective,
        block_label,
    })
}

fn normalize_block_label(
    path: &Path,
    fallback: SourcePosition,
    position: Option<SourcePosition>,
    grammar: BlockLabelGrammar,
) -> Result<RealizedSurface, ReadError> {
    let position = position.unwrap_or(fallback);
    validate_surface(path, position, "block label", &grammar.surface)?;
    Ok(RealizedSurface {
        feature: SurfaceFeature::BlockLabel,
        onset: normalized_onset(path, position, &grammar.surface, grammar.onset)?,
        onset_override: grammar.onset,
        text: grammar.surface,
    })
}

fn normalize_participial_adjective(
    path: &Path,
    fallback: SourcePosition,
    position: Option<SourcePosition>,
    fixed_keyword_surface: &str,
    grammar: ParticipialAdjectiveGrammar,
) -> Result<RealizedSurface, ReadError> {
    let position = position.unwrap_or(fallback);
    let surface = match grammar.surface {
        DerivedSurface::Derived => english_participle(fixed_keyword_surface),
        DerivedSurface::Override(surface) => surface,
        DerivedSurface::Unavailable => {
            return Err(validation_error_at(
                path,
                position,
                ValidationError::InvalidSurface {
                    field: "participial_adjective.surface",
                },
            ));
        }
    };
    validate_surface(path, position, "participial adjective", &surface)?;
    Ok(RealizedSurface {
        feature: SurfaceFeature::Participle,
        onset: normalized_onset(path, position, &surface, grammar.onset)?,
        onset_override: grammar.onset,
        text: surface,
    })
}

fn fixed_grammar(
    path: &Path,
    position: SourcePosition,
    surface: String,
    onset_override: Option<Onset>,
    recipe: GrammarRecipe,
) -> Result<(String, GrammarRecipe, Vec<RealizedSurface>), ReadError> {
    validate_surface(path, position, "surface", &surface)?;
    let onset = normalized_onset(path, position, &surface, onset_override)?;
    Ok((
        surface.clone(),
        recipe,
        vec![RealizedSurface {
            feature: SurfaceFeature::Fixed,
            text: surface,
            onset,
            onset_override,
        }],
    ))
}

fn normalized_onset(
    path: &Path,
    position: SourcePosition,
    surface: &str,
    onset_override: Option<Onset>,
) -> Result<Onset, ReadError> {
    normalize_surface_onset(surface, onset_override).ok_or_else(|| {
        validation_error_at(
            path,
            position,
            ValidationError::UnknownOnset {
                surface: surface.to_owned(),
            },
        )
    })
}

/// Applies the normalization-owned bounded pronunciation recipe.
///
/// This is exposed for the declaration compiler to freeze its closed terminal
/// rows. Runtime scanners, builders, and renderers consume only those frozen
/// values and normalized [`RealizedSurface`] rows.
#[must_use]
pub fn normalize_surface_onset(surface: &str, onset_override: Option<Onset>) -> Option<Onset> {
    onset_override.or_else(|| bounded_surface_onset(surface))
}

fn bounded_surface_onset(surface: &str) -> Option<Onset> {
    let word = surface
        .split(|character: char| character.is_whitespace() || character == '-')
        .next()?;
    if word.is_empty() {
        return None;
    }

    let letters = word
        .chars()
        .take_while(char::is_ascii_alphabetic)
        .collect::<String>();
    if !letters.is_empty() && letters.chars().all(|letter| letter.is_ascii_uppercase()) {
        let first = letters.as_bytes()[0];
        return Some(
            if matches!(
                first,
                b'A' | b'E' | b'F' | b'H' | b'I' | b'L' | b'M' | b'N' | b'O' | b'R' | b'S' | b'X'
            ) {
                Onset::Vowel
            } else {
                Onset::Consonant
            },
        );
    }

    let lower = word.to_ascii_lowercase();
    if ["heir", "honest", "honor", "hour"]
        .iter()
        .any(|prefix| lower.starts_with(prefix))
    {
        return Some(Onset::Vowel);
    }
    if lower.starts_with("eu")
        || lower == "one"
        || lower.starts_with("once")
        || [
            "unit",
            "unite",
            "unity",
            "unicorn",
            "uniform",
            "unique",
            "union",
            "universe",
            "universal",
            "university",
            "use",
            "user",
            "usual",
            "utensil",
            "utility",
            "utopia",
        ]
        .iter()
        .any(|prefix| lower.starts_with(prefix))
    {
        return Some(Onset::Consonant);
    }

    match lower.as_bytes().first().copied() {
        Some(b'a' | b'e' | b'i' | b'o' | b'u') => Some(Onset::Vowel),
        Some(first) if first.is_ascii_alphabetic() => Some(Onset::Consonant),
        _ => None,
    }
}

fn realize_derived_surface(
    path: &Path,
    fallback: SourcePosition,
    position: Option<SourcePosition>,
    field: &'static str,
    derived: String,
    authored: DerivedSurface,
) -> Result<Option<String>, ReadError> {
    match authored {
        DerivedSurface::Derived => Ok(Some(derived)),
        DerivedSurface::Unavailable => Ok(None),
        DerivedSurface::Override(surface) => {
            let position = required_map_position(path, fallback, position, field)?;
            validate_surface(path, position, field, &surface)?;
            if surface == derived {
                return Err(validation_error_at(
                    path,
                    position,
                    ValidationError::RedundantOverride { field, surface },
                ));
            }
            Ok(Some(surface))
        }
    }
}

fn english_verb(bare: &str) -> String {
    format!("{bare}s")
}

fn english_participle(bare: &str) -> String {
    format!("{bare}ed")
}

fn english_noun(singular: &str) -> String {
    format!("{singular}s")
}

fn validate_frame_set(
    path: &Path,
    position: SourcePosition,
    frame_set: &VerbFrameSet,
) -> Result<(), ReadError> {
    let VerbFrameSet::Custom { frames } = frame_set.frame_set() else {
        return Ok(());
    };
    if frames.is_empty() {
        return Err(validation_error_at(
            path,
            position,
            ValidationError::EmptyCustomVerbFrameSet,
        ));
    }
    let mut seen = HashSet::new();
    for frame in frames {
        if !seen.insert(frame) {
            return Err(validation_error_at(
                path,
                position,
                ValidationError::DuplicateVerbFrame {
                    frame: frame.clone(),
                },
            ));
        }
        for atom in frame {
            match atom {
                CustomTailAtom::Literal(literal) if !valid_surface(literal) => {
                    return Err(validation_error_at(
                        path,
                        position,
                        ValidationError::InvalidCustomLiteral,
                    ));
                }
                CustomTailAtom::Lex(vocabulary, member)
                    if vocabulary.is_empty() || member.is_empty() =>
                {
                    return Err(validation_error_at(
                        path,
                        position,
                        ValidationError::InvalidCustomLiteral,
                    ));
                }
                CustomTailAtom::Literal(_)
                | CustomTailAtom::Lex(_, _)
                | CustomTailAtom::Amount
                | CustomTailAtom::ObjectNounPhrase
                | CustomTailAtom::PredicativeComplement => {}
            }
        }
    }
    Ok(())
}

fn validate_surface(
    path: &Path,
    position: SourcePosition,
    field: &'static str,
    surface: &str,
) -> Result<(), ReadError> {
    if !valid_surface(surface) {
        return Err(validation_error_at(
            path,
            position,
            ValidationError::InvalidSurface { field },
        ));
    }
    Ok(())
}

fn valid_surface(surface: &str) -> bool {
    !surface.is_empty()
        && surface.trim() == surface
        && !surface
            .chars()
            .any(|character| matches!(character, '\n' | '\r'))
}

fn parse_spelling(spelling: &str) -> Result<Vec<SpellingPart>, String> {
    if !valid_surface(spelling) {
        return Err("the frame must be nonempty, trimmed, and single-line".to_owned());
    }
    let mut parts = Vec::new();
    let mut cursor = 0;
    while let Some(relative_open) = spelling[cursor..].find('<') {
        let open = cursor + relative_open;
        if spelling[cursor..open].contains('>') {
            return Err("`>` may appear only as part of `<Param(n)>`".to_owned());
        }
        if open > cursor {
            parts.push(SpellingPart::Literal(spelling[cursor..open].to_owned()));
        }
        let suffix = &spelling[open..];
        let Some(close) = suffix.find(")>") else {
            return Err("an opening `<` must begin a complete `<Param(n)>` hole".to_owned());
        };
        let hole = &suffix[..close + 2];
        let Some(digits) = hole
            .strip_prefix("<Param(")
            .and_then(|inner| inner.strip_suffix(")>"))
        else {
            return Err("holes must use the exact spelling `<Param(n)>`".to_owned());
        };
        if digits.is_empty()
            || !digits.bytes().all(|byte| byte.is_ascii_digit())
            || (digits.len() > 1 && digits.starts_with('0'))
        {
            return Err("Param indices are canonical unsigned decimal integers".to_owned());
        }
        let index = digits
            .parse::<usize>()
            .map_err(|_| "Param index is too large".to_owned())?;
        parts.push(SpellingPart::Param(index));
        cursor = open + close + 2;
    }
    if spelling[cursor..].contains(['<', '>']) {
        return Err("angle brackets may appear only in `<Param(n)>` holes".to_owned());
    }
    if cursor < spelling.len() {
        parts.push(SpellingPart::Literal(spelling[cursor..].to_owned()));
    }
    Ok(parts)
}

fn validate_spelling_params(
    path: &Path,
    position: SourcePosition,
    spelling: &[SpellingPart],
    params: Option<&[ParameterType]>,
) -> Result<(), ReadError> {
    for part in spelling {
        let SpellingPart::Param(index) = part else {
            continue;
        };
        let len = params.map_or(0, <[ParameterType]>::len);
        if params.is_none() || *index >= len {
            return Err(validation_error_at(
                path,
                position,
                ValidationError::ParamOutOfRange {
                    location: "spelling",
                    index: *index,
                    len,
                },
            ));
        }
    }
    Ok(())
}

fn validate_body_params(
    path: &Path,
    fallback: SourcePosition,
    position: Option<SourcePosition>,
    definition: &macro_ron::MacroDef<Metadata>,
    reader: &macro_ron::MacroSet,
    params: Option<&[ParameterType]>,
) -> Result<(), ReadError> {
    if position.is_none() {
        return Ok(());
    }
    let position = required_map_position(path, fallback, position, "body")?;
    let keys = definition.body_param_keys(reader).map_err(|reason| {
        validation_error_at(path, position, ValidationError::InvalidBody { reason })
    })?;
    let len = params.map_or(0, <[ParameterType]>::len);
    for key in keys {
        match key.parse::<usize>() {
            Ok(index) if index < len => {}
            Ok(index) => {
                return Err(validation_error_at(
                    path,
                    position,
                    ValidationError::ParamOutOfRange {
                        location: "body",
                        index,
                        len,
                    },
                ));
            }
            Err(_) => {
                return Err(validation_error_at(
                    path,
                    position,
                    ValidationError::InvalidBody {
                        reason: format!("holes `Param({key})`, but v2 declarations are positional"),
                    },
                ));
            }
        }
    }
    Ok(())
}

fn spelling_head(spelling: &[SpellingPart]) -> String {
    match spelling.first() {
        Some(SpellingPart::Literal(literal)) => literal.trim_end().to_owned(),
        Some(SpellingPart::Param(_)) | None => String::new(),
    }
}

fn is_bare_ident(name: &str) -> bool {
    let mut characters = name.chars();
    characters
        .next()
        .is_some_and(|character| character.is_ascii_alphabetic() || character == '_')
        && characters.all(|character| character.is_ascii_alphanumeric() || character == '_')
}

fn validation_error_at(path: &Path, position: SourcePosition, error: ValidationError) -> ReadError {
    ReadError::Validate {
        path: path.to_owned(),
        position,
        source: error,
    }
}

fn locate_offset(source: &str, offset: usize) -> SourcePosition {
    let mut line = 1;
    let mut column = 1;
    for (index, character) in source.char_indices() {
        if index >= offset {
            break;
        }
        if character == '\n' {
            line += 1;
            column = 1;
        } else {
            column += 1;
        }
    }
    SourcePosition { line, column }
}

fn ron_files_recursive(dir: &Path) -> Result<Vec<PathBuf>, ReadError> {
    if !dir.exists() {
        return Ok(Vec::new());
    }
    let entries = dir.read_dir().map_err(|source| ReadError::Io {
        path: dir.to_owned(),
        source,
    })?;
    let mut files = Vec::new();
    let mut subdirectories = Vec::new();
    for entry in entries {
        let entry = entry.map_err(|source| ReadError::Io {
            path: dir.to_owned(),
            source,
        })?;
        let path = entry.path();
        let file_type = entry.file_type().map_err(|source| ReadError::Io {
            path: path.clone(),
            source,
        })?;
        if file_type.is_dir() {
            subdirectories.push(path);
        } else if path.extension().is_some_and(|extension| extension == "ron") && path.is_file() {
            files.push(path);
        }
    }
    subdirectories.sort();
    for subdirectory in subdirectories {
        files.extend(ron_files_recursive(&subdirectory)?);
    }
    files.sort();
    Ok(files)
}

fn expected_builtin_identity(
    nursery: &Path,
    declaration: &NormalizedDeclaration,
) -> Result<(DeclarationKind, String), ValidationError> {
    let path = &declaration.provenance.path;
    let relative =
        path.strip_prefix(nursery)
            .map_err(|_| ValidationError::UnexpectedBuiltinLocation {
                relative: path.clone(),
            })?;
    let components = relative
        .iter()
        .map(|component| component.to_str())
        .collect::<Option<Vec<_>>>();
    let Some(components) = components else {
        return Err(ValidationError::UnexpectedBuiltinLocation {
            relative: relative.to_owned(),
        });
    };
    let Some(stem) = path.file_stem().and_then(|stem| stem.to_str()) else {
        return Err(ValidationError::UnexpectedBuiltinLocation {
            relative: relative.to_owned(),
        });
    };
    let kind = match components.as_slice() {
        ["keyword_actions", _] => DeclarationKind::KeywordAction,
        ["keyword_abilities", _] => DeclarationKind::KeywordAbility,
        ["ability_words", _] => DeclarationKind::AbilityWord,
        ["flavor_words", _] => DeclarationKind::FlavorWord,
        ["types", _] => DeclarationKind::Type,
        ["turn_parts", _] => DeclarationKind::TurnPart,
        ["counter_kinds", _] => DeclarationKind::CounterKind,
        ["designations", _] => DeclarationKind::Designation,
        ["subtypes", category, _] => DeclarationKind::Subtype(match *category {
            "artifact" => SubtypeCategory::Artifact,
            "battle" => SubtypeCategory::Battle,
            "creature" => SubtypeCategory::Creature,
            "enchantment" => SubtypeCategory::Enchantment,
            "land" => SubtypeCategory::Land,
            "planeswalker" => SubtypeCategory::Planeswalker,
            "spell" => SubtypeCategory::Spell,
            _ => {
                return Err(ValidationError::UnexpectedBuiltinLocation {
                    relative: relative.to_owned(),
                });
            }
        }),
        _ => {
            return Err(ValidationError::UnexpectedBuiltinLocation {
                relative: relative.to_owned(),
            });
        }
    };
    Ok((kind, stem.to_owned()))
}

#[cfg(test)]
mod tests;
