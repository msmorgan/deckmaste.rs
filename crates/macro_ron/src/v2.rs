//! Neutral source and normalized-row types for v2 plugin declarations.
//!
//! This module is deliberately separate from [`crate::MacroDef`]. V2 source
//! has one category-safe declaration kind, positional parameters, a semantic
//! spelling frame, and optional closed grammar data. It is data for later
//! providers and parser environments; it is not expanded through the legacy
//! macro reader.

use std::collections::HashMap;
use std::collections::HashSet;
use std::fmt;
use std::path::Path;
use std::path::PathBuf;

use ron::extensions::Extensions;
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

/// The RON dialect used by every v2 declaration read and write.
///
/// `implicit_some` keeps optional source fields flat. Unwrapped newtype
/// variants make `KeywordAction(name: ..., ...)` the spelling of an enum arm
/// carrying [`DeclarationFields`], rather than adding a second pair of
/// parentheses around the fields.
#[must_use]
pub fn ron_options() -> ron::Options {
    ron::Options::default()
        .with_default_extension(Extensions::IMPLICIT_SOME | Extensions::UNWRAP_VARIANT_NEWTYPES)
}

/// A complete v2 declaration source value.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub enum Declaration {
    KeywordAction(DeclarationFields),
    KeywordAbility(DeclarationFields),
    AbilityWord(DeclarationFields),
    Subtype(SubtypeDeclaration),
    Type(DeclarationFields),
    CounterKind(DeclarationFields),
    Designation(DeclarationFields),
}

impl Declaration {
    /// The declaration's category-safe identity kind.
    #[must_use]
    pub fn kind(&self) -> DeclarationKind {
        match self {
            Declaration::KeywordAction(_) => DeclarationKind::KeywordAction,
            Declaration::KeywordAbility(_) => DeclarationKind::KeywordAbility,
            Declaration::AbilityWord(_) => DeclarationKind::AbilityWord,
            Declaration::Subtype(declaration) => DeclarationKind::Subtype(declaration.category),
            Declaration::Type(_) => DeclarationKind::Type,
            Declaration::CounterKind(_) => DeclarationKind::CounterKind,
            Declaration::Designation(_) => DeclarationKind::Designation,
        }
    }

    fn into_parts(self) -> (DeclarationKind, DeclarationFields) {
        match self {
            Declaration::KeywordAction(fields) => (DeclarationKind::KeywordAction, fields),
            Declaration::KeywordAbility(fields) => (DeclarationKind::KeywordAbility, fields),
            Declaration::AbilityWord(fields) => (DeclarationKind::AbilityWord, fields),
            Declaration::Subtype(declaration) => (
                DeclarationKind::Subtype(declaration.category),
                declaration.into_fields(),
            ),
            Declaration::Type(fields) => (DeclarationKind::Type, fields),
            Declaration::CounterKind(fields) => (DeclarationKind::CounterKind, fields),
            Declaration::Designation(fields) => (DeclarationKind::Designation, fields),
        }
    }
}

/// Fields shared by each externally tagged declaration kind.
///
/// `params: None` means an untyped nursery record. `params: Some([])` is a
/// graduated nullary signature; the distinction is intentional.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct DeclarationFields {
    pub name: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub params: Option<Vec<ParameterType>>,
    pub spelling: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub grammar: Option<Grammar>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub body: Option<Box<RawValue>>,
}

/// A subtype declaration has the same source fields plus one semantic
/// subtype category. The category is part of its identity domain.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct SubtypeDeclaration {
    pub category: SubtypeCategory,
    pub name: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub params: Option<Vec<ParameterType>>,
    pub spelling: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub grammar: Option<Grammar>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub body: Option<Box<RawValue>>,
}

impl SubtypeDeclaration {
    fn into_fields(self) -> DeclarationFields {
        DeclarationFields {
            name: self.name,
            params: self.params,
            spelling: self.spelling,
            grammar: self.grammar,
            body: self.body,
        }
    }
}

/// The open registry family in which a declaration name is unique.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd)]
pub enum DeclarationKind {
    KeywordAction,
    KeywordAbility,
    AbilityWord,
    Subtype(SubtypeCategory),
    Type,
    CounterKind,
    Designation,
}

impl fmt::Display for DeclarationKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DeclarationKind::KeywordAction => f.write_str("keyword action"),
            DeclarationKind::KeywordAbility => f.write_str("keyword ability"),
            DeclarationKind::AbilityWord => f.write_str("ability word"),
            DeclarationKind::Subtype(category) => write!(f, "{category} subtype"),
            DeclarationKind::Type => f.write_str("type"),
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

/// One positional semantic parameter type, serialized as a bare RON
/// identifier (`Amount`, not `"Amount"`).
#[derive(Debug, Clone, PartialEq, Eq, Hash, Ord, PartialOrd)]
pub struct ParameterType(String);

impl ParameterType {
    /// Constructs a checked bare parameter type.
    ///
    /// # Errors
    /// If `name` is not an ASCII RON identifier.
    pub fn new(name: impl Into<String>) -> Result<Self, String> {
        let name = name.into();
        if !is_bare_ident(&name) {
            return Err(format!("parameter type `{name}` is not a bare identifier"));
        }
        Ok(Self(name))
    }

    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl<'de> Deserialize<'de> for ParameterType {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let raw = Box::<RawValue>::deserialize(deserializer)?;
        Self::new(raw.get_ron().trim()).map_err(D::Error::custom)
    }
}

impl Serialize for ParameterType {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        RawValue::from_ron(&self.0)
            .map_err(S::Error::custom)?
            .serialize(serializer)
    }
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
        valence: VerbValence,
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
        /// An additional determiner use of this keyword declaration.
        ///
        /// The fixed keyword remains the declaration's primary grammar row;
        /// this supplemental row is indexed separately so its participial
        /// surface can never scan as a keyword line.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        determinative: Option<DeterminativeGrammar>,
    },
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
    CountNominal,
    BareSingularNoun,
    MassOrPluralCount,
}

/// Whether a Determinative lemma may serve as a fused partitive head.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub enum DeterminativeFusedHeadLicense {
    NominalOnly,
    FusedHead,
}

/// The phrase-number condition of one Determinative surface realization.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub enum DeterminativePhraseNumber {
    Singular,
    Plural,
}

/// One authored surface selected from a Determinative lemma.
///
/// The conditions describe the following phrase, never a stored AST surface.
/// This lets one lemma realize as `a`/`an` or `that`/`those` without making
/// those spellings distinct AST members.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct DeterminativeRealization {
    pub surface: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub onset: Option<Onset>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub phrase_number: Option<DeterminativePhraseNumber>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub following_onset: Option<Onset>,
}

/// The required selection contract and realizations of one Determinative.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct DeterminativeGrammar {
    pub number_license: DeterminativeNumberLicense,
    pub nominal_license: DeterminativeNominalLicense,
    pub fused_head_license: DeterminativeFusedHeadLicense,
    pub realizations: Vec<DeterminativeRealization>,
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
        ron_options()
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

/// The grammatical complement family of one verb definition.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub enum VerbValence {
    Intransitive,
    Transitive,
    Numerative,
    Custom { shapes: Vec<Vec<CustomTailAtom>> },
}

/// The complete serialized atom vocabulary for a custom verb tail.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub enum CustomTailAtom {
    Literal(String),
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
    spelling: Vec<SpellingPart>,
    grammar: Option<GrammarRow>,
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

    #[must_use]
    pub fn spelling(&self) -> &[SpellingPart] {
        &self.spelling
    }

    #[must_use]
    pub fn grammar(&self) -> Option<&GrammarRow> {
        self.grammar.as_ref()
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
    determinative: Option<DeterminativeRow>,
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

    /// Returns the declaration's supplemental Determinative row, if it has one.
    #[must_use]
    pub fn determinative(&self) -> Option<&DeterminativeRow> {
        self.determinative.as_ref()
    }
}

/// A fully normalized Determinative grammar row.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DeterminativeRow {
    number_license: DeterminativeNumberLicense,
    nominal_license: DeterminativeNominalLicense,
    fused_head_license: DeterminativeFusedHeadLicense,
    realizations: Vec<RealizedDeterminativeSurface>,
}

impl DeterminativeRow {
    #[must_use]
    pub fn number_license(&self) -> DeterminativeNumberLicense {
        self.number_license
    }

    #[must_use]
    pub fn nominal_license(&self) -> DeterminativeNominalLicense {
        self.nominal_license
    }

    #[must_use]
    pub fn fused_head_license(&self) -> DeterminativeFusedHeadLicense {
        self.fused_head_license
    }

    #[must_use]
    pub fn realizations(&self) -> &[RealizedDeterminativeSurface] {
        &self.realizations
    }
}

/// One Determinative surface with its frozen effective onset.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RealizedDeterminativeSurface {
    surface: String,
    onset: Onset,
    onset_override: Option<Onset>,
    phrase_number: Option<DeterminativePhraseNumber>,
    following_onset: Option<Onset>,
}

impl RealizedDeterminativeSurface {
    #[must_use]
    pub fn surface(&self) -> &str {
        &self.surface
    }

    #[must_use]
    pub fn onset(&self) -> Onset {
        self.onset
    }

    #[must_use]
    pub fn onset_override(&self) -> Option<Onset> {
        self.onset_override
    }

    #[must_use]
    pub fn phrase_number(&self) -> Option<DeterminativePhraseNumber> {
        self.phrase_number
    }

    #[must_use]
    pub fn following_onset(&self) -> Option<Onset> {
        self.following_onset
    }
}

/// Closed recipe information retained after surface sealing.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum GrammarRecipe {
    Verb { valence: VerbValence },
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
    spelling: SourcePosition,
    grammar: Option<GrammarSourceMap>,
    body: Option<SourcePosition>,
}

enum GrammarSourceMap {
    Verb {
        bare: SourcePosition,
        third_person: Option<SourcePosition>,
        participle: Option<SourcePosition>,
        valence: SourcePosition,
    },
    Noun {
        singular: SourcePosition,
        plural: Option<SourcePosition>,
    },
    Fixed {
        surface: SourcePosition,
    },
}

#[derive(Deserialize)]
enum DiagnosticDeclaration<'a> {
    KeywordAction(#[serde(borrow)] DiagnosticFields<'a>),
    KeywordAbility(#[serde(borrow)] DiagnosticFields<'a>),
    AbilityWord(#[serde(borrow)] DiagnosticFields<'a>),
    Subtype(#[serde(borrow)] DiagnosticSubtype<'a>),
    Type(#[serde(borrow)] DiagnosticFields<'a>),
    CounterKind(#[serde(borrow)] DiagnosticFields<'a>),
    Designation(#[serde(borrow)] DiagnosticFields<'a>),
}

#[derive(Deserialize)]
struct DiagnosticFields<'a> {
    #[serde(borrow)]
    name: &'a RawValue,
    #[serde(borrow)]
    spelling: &'a RawValue,
    #[serde(default, borrow)]
    grammar: Option<DiagnosticGrammar<'a>>,
    #[serde(default, borrow)]
    body: Option<&'a RawValue>,
}

#[derive(Deserialize)]
struct DiagnosticSubtype<'a> {
    #[serde(borrow)]
    category: &'a RawValue,
    #[serde(borrow)]
    name: &'a RawValue,
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
        valence: &'a RawValue,
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
    },
}

struct LeadingVariant;

impl<'de> DeserializeSeed<'de> for LeadingVariant {
    type Value = crate::Ident;

    fn deserialize<D: Deserializer<'de>>(self, de: D) -> Result<Self::Value, D::Error> {
        de.deserialize_enum("", &[], self)
    }
}

impl<'de> Visitor<'de> for LeadingVariant {
    type Value = crate::Ident;

    fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("a declaration variant")
    }

    fn visit_enum<A: EnumAccess<'de>>(self, data: A) -> Result<Self::Value, A::Error> {
        let (ident, _variant) = data.variant_seed(crate::IdentSeed)?;
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
    #[error("Custom valence requires a nonempty shape set")]
    EmptyCustomShapeSet,
    #[error("Custom valence repeats tail shape {shape:?}")]
    DuplicateCustomShape { shape: Vec<CustomTailAtom> },
    #[error("Custom Literal atoms must contain a nonempty, trimmed, single-line terminal span")]
    InvalidCustomLiteral,
    #[error("Determinative grammar requires one or more realization rows")]
    EmptyDeterminativeRealizations,
    #[error(
        "Determinative realization `{surface}` is incompatible with number license `{number_license:?}`"
    )]
    IncompatibleDeterminativeRealization {
        surface: String,
        number_license: DeterminativeNumberLicense,
    },
    #[error("Determinative realizations `{first}` and `{second}` overlap")]
    OverlappingDeterminativeRealizations { first: String, second: String },
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
        let diagnostic = ron_options()
            .from_str::<DiagnosticDeclaration<'_>>(source)
            .map_err(|source| ReadError::Parse {
                path: path.to_owned(),
                source: Box::new(source),
            })?;

        match diagnostic {
            DiagnosticDeclaration::KeywordAction(fields)
            | DiagnosticDeclaration::KeywordAbility(fields)
            | DiagnosticDeclaration::AbilityWord(fields)
            | DiagnosticDeclaration::Type(fields)
            | DiagnosticDeclaration::CounterKind(fields)
            | DiagnosticDeclaration::Designation(fields) => Self::from_fields(
                path,
                source,
                declaration,
                DiagnosticFieldValues {
                    category: None,
                    name: fields.name,
                    spelling: fields.spelling,
                    grammar: fields.grammar,
                    body: fields.body,
                },
            ),
            DiagnosticDeclaration::Subtype(subtype) => Self::from_fields(
                path,
                source,
                declaration,
                DiagnosticFieldValues {
                    category: Some(subtype.category),
                    name: subtype.name,
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
                valence,
            } => Ok(Self::Verb {
                bare: raw_position(path, source, bare, declaration)?,
                third_person: third_person
                    .map(|value| raw_position(path, source, value, declaration))
                    .transpose()?,
                participle: participle
                    .map(|value| raw_position(path, source, value, declaration))
                    .transpose()?,
                valence: raw_position(path, source, valence, declaration)?,
            }),
            DiagnosticGrammar::Noun { singular, plural } => Ok(Self::Noun {
                singular: raw_position(path, source, singular, declaration)?,
                plural: plural
                    .map(|value| raw_position(path, source, value, declaration))
                    .transpose()?,
            }),
            DiagnosticGrammar::FixedTerm { surface }
            | DiagnosticGrammar::FixedClause { surface }
            | DiagnosticGrammar::FixedKeyword { surface } => Ok(Self::Fixed {
                surface: raw_position(path, source, surface, declaration)?,
            }),
        }
    }
}

fn leading_variant_position(path: &Path, source: &str) -> Result<SourcePosition, ReadError> {
    let options = ron_options();
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
    read_mapped(path.into(), source).map(|mapped| mapped.declaration)
}

struct MappedDeclaration {
    declaration: NormalizedDeclaration,
    source_map: ValidationSourceMap,
}

fn read_mapped(path: PathBuf, source: &str) -> Result<MappedDeclaration, ReadError> {
    let declaration = ron_options()
        .from_str::<Declaration>(source)
        .map_err(|source| ReadError::Parse {
            path: path.clone(),
            source: Box::new(source),
        })?;
    let source_map = ValidationSourceMap::read(&path, source)?;
    let declaration = normalize(path, declaration, &source_map)?;
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
    sources.sort_by(|left, right| left.path.cmp(&right.path));
    let mut first_by_identity: HashMap<DeclarationIdentity, PathBuf> = HashMap::new();
    let mut declarations = Vec::with_capacity(sources.len());
    for source in sources {
        let declaration = read_mapped(source.path.clone(), &source.source)?;
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

/// Reads the committed builtin-v2 nursery through the ordinary v2 reader.
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
    declaration: Declaration,
    source_map: &ValidationSourceMap,
) -> Result<NormalizedDeclaration, ReadError> {
    let (kind, fields) = declaration.into_parts();
    let DeclarationFields {
        name,
        params,
        spelling,
        grammar,
        body,
    } = fields;

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
        body.as_deref(),
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
        spelling: spelling_parts,
        grammar,
        body,
        provenance: SourceProvenance { path },
    })
}

fn normalize_grammar(
    path: &Path,
    source_map: &GrammarSourceMap,
    spelling_position: SourcePosition,
    spelling: &[SpellingPart],
    grammar: Grammar,
) -> Result<GrammarRow, ReadError> {
    let (grammar_head, recipe, surfaces, determinative) = match (grammar, source_map) {
        (
            Grammar::Verb {
                bare,
                bare_onset,
                third_person,
                third_person_onset,
                participle,
                participle_onset,
                valence,
            },
            GrammarSourceMap::Verb {
                bare: bare_position,
                third_person: third_person_position,
                participle: participle_position,
                valence: valence_position,
            },
        ) => {
            validate_surface(path, *bare_position, "bare", &bare)?;
            validate_valence(path, *valence_position, &valence)?;
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
            (bare, GrammarRecipe::Verb { valence }, surfaces, None)
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
        ) => {
            validate_surface(path, *singular_position, "singular", &singular)?;
            let plural = realize_derived_surface(
                path,
                *singular_position,
                *plural_position,
                "plural",
                english_noun(&singular),
                plural,
            )?;
            let mut surfaces = vec![RealizedSurface {
                feature: SurfaceFeature::Singular,
                onset: normalized_onset(path, *singular_position, &singular, singular_onset)?,
                onset_override: singular_onset,
                text: singular.clone(),
            }];
            if let Some(text) = plural {
                let position = plural_position.unwrap_or(*singular_position);
                surfaces.push(RealizedSurface {
                    feature: SurfaceFeature::Plural,
                    onset: normalized_onset(path, position, &text, plural_onset)?,
                    onset_override: plural_onset,
                    text,
                });
            }
            (singular, GrammarRecipe::Noun, surfaces, None)
        }
        (Grammar::FixedTerm { surface, onset }, GrammarSourceMap::Fixed { surface: position }) => {
            let (head, recipe, surfaces) =
                fixed_grammar(path, *position, surface, onset, GrammarRecipe::FixedTerm)?;
            (head, recipe, surfaces, None)
        }
        (
            Grammar::FixedClause { surface, onset },
            GrammarSourceMap::Fixed { surface: position },
        ) => {
            let (head, recipe, surfaces) =
                fixed_grammar(path, *position, surface, onset, GrammarRecipe::FixedClause)?;
            (head, recipe, surfaces, None)
        }
        (
            Grammar::FixedKeyword {
                surface,
                onset,
                determinative,
            },
            GrammarSourceMap::Fixed { surface: position },
        ) => {
            let (head, recipe, surfaces) =
                fixed_grammar(path, *position, surface, onset, GrammarRecipe::FixedKeyword)?;
            let determinative = determinative
                .map(|grammar| normalize_determinative(path, *position, grammar))
                .transpose()?;
            (head, recipe, surfaces, determinative)
        }
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
        grammar_head,
        recipe,
        surfaces,
        determinative,
    )
}

fn finish_grammar_normalization(
    path: &Path,
    spelling_position: SourcePosition,
    spelling: &[SpellingPart],
    grammar_head: String,
    recipe: GrammarRecipe,
    surfaces: Vec<RealizedSurface>,
    determinative: Option<DeterminativeRow>,
) -> Result<GrammarRow, ReadError> {
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
        determinative,
    })
}

fn normalize_determinative(
    path: &Path,
    position: SourcePosition,
    grammar: DeterminativeGrammar,
) -> Result<DeterminativeRow, ReadError> {
    if grammar.realizations.is_empty() {
        return Err(validation_error_at(
            path,
            position,
            ValidationError::EmptyDeterminativeRealizations,
        ));
    }
    let mut realizations = Vec::with_capacity(grammar.realizations.len());
    for realization in grammar.realizations {
        if !number_license_allows(grammar.number_license, realization.phrase_number) {
            return Err(validation_error_at(
                path,
                position,
                ValidationError::IncompatibleDeterminativeRealization {
                    surface: realization.surface,
                    number_license: grammar.number_license,
                },
            ));
        }
        validate_surface(
            path,
            position,
            "determinative realization",
            &realization.surface,
        )?;
        let onset = normalized_onset(path, position, &realization.surface, realization.onset)?;
        realizations.push(RealizedDeterminativeSurface {
            surface: realization.surface,
            onset,
            onset_override: realization.onset,
            phrase_number: realization.phrase_number,
            following_onset: realization.following_onset,
        });
    }
    for (index, left) in realizations.iter().enumerate() {
        for right in &realizations[index + 1..] {
            if determinative_conditions_overlap(left, right) {
                return Err(validation_error_at(
                    path,
                    position,
                    ValidationError::OverlappingDeterminativeRealizations {
                        first: left.surface.clone(),
                        second: right.surface.clone(),
                    },
                ));
            }
        }
    }
    Ok(DeterminativeRow {
        number_license: grammar.number_license,
        nominal_license: grammar.nominal_license,
        fused_head_license: grammar.fused_head_license,
        realizations,
    })
}

fn number_license_allows(
    license: DeterminativeNumberLicense,
    phrase_number: Option<DeterminativePhraseNumber>,
) -> bool {
    matches!(
        (license, phrase_number),
        (DeterminativeNumberLicense::Both, _)
            | (
                DeterminativeNumberLicense::SingularOnly,
                None | Some(DeterminativePhraseNumber::Singular)
            )
            | (
                DeterminativeNumberLicense::PluralOnly,
                None | Some(DeterminativePhraseNumber::Plural)
            )
    )
}

fn determinative_conditions_overlap(
    left: &RealizedDeterminativeSurface,
    right: &RealizedDeterminativeSurface,
) -> bool {
    (left.phrase_number.is_none()
        || right.phrase_number.is_none()
        || left.phrase_number == right.phrase_number)
        && (left.following_onset.is_none()
            || right.following_onset.is_none()
            || left.following_onset == right.following_onset)
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

fn validate_valence(
    path: &Path,
    position: SourcePosition,
    valence: &VerbValence,
) -> Result<(), ReadError> {
    let VerbValence::Custom { shapes } = valence else {
        return Ok(());
    };
    if shapes.is_empty() {
        return Err(validation_error_at(
            path,
            position,
            ValidationError::EmptyCustomShapeSet,
        ));
    }
    let mut seen = HashSet::new();
    for shape in shapes {
        if !seen.insert(shape) {
            return Err(validation_error_at(
                path,
                position,
                ValidationError::DuplicateCustomShape {
                    shape: shape.clone(),
                },
            ));
        }
        for atom in shape {
            if let CustomTailAtom::Literal(literal) = atom
                && !valid_surface(literal)
            {
                return Err(validation_error_at(
                    path,
                    position,
                    ValidationError::InvalidCustomLiteral,
                ));
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
    body: Option<&RawValue>,
    params: Option<&[ParameterType]>,
) -> Result<(), ReadError> {
    let Some(body) = body else {
        return Ok(());
    };
    let position = required_map_position(path, fallback, position, "body")?;
    let mut keys = Vec::new();
    crate::expand::collect_param_keys(body.get_ron(), &ron_options(), &mut keys).map_err(
        |reason| validation_error_at(path, position, ValidationError::InvalidBody { reason }),
    )?;
    let len = params.map_or(0, <[ParameterType]>::len);
    for key in keys {
        match key {
            crate::expand::ParamKey::Index(index) if index < len => {}
            crate::expand::ParamKey::Index(index) => {
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
            crate::expand::ParamKey::Name(name) => {
                return Err(validation_error_at(
                    path,
                    position,
                    ValidationError::InvalidBody {
                        reason: format!(
                            "holes `Param({name})`, but v2 declarations are positional"
                        ),
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
        ["types", _] => DeclarationKind::Type,
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
