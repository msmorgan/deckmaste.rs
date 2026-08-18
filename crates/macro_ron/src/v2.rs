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
use serde::de::Deserializer;
use serde::de::Error as _;
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
        #[serde(default, skip_serializing_if = "DerivedSurface::is_derived")]
        third_person: DerivedSurface,
        valence: VerbValence,
    },
    Noun {
        singular: String,
        #[serde(default, skip_serializing_if = "DerivedSurface::is_derived")]
        plural: DerivedSurface,
    },
    FixedTerm {
        surface: String,
    },
    FixedClause {
        surface: String,
    },
    FixedKeyword {
        surface: String,
    },
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
}

/// A category-safe declaration name.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Ord, PartialOrd)]
pub struct DeclarationIdentity {
    kind: DeclarationKind,
    name: String,
}

impl DeclarationIdentity {
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

/// The grammatical feature attached to a realized surface.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SurfaceFeature {
    Bare,
    ThirdPersonSingular,
    Singular,
    Plural,
    Fixed,
}

/// One complete scan/render surface row.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RealizedSurface {
    feature: SurfaceFeature,
    text: String,
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
    let declaration = ron_options()
        .from_str::<Declaration>(source)
        .map_err(|source| ReadError::Parse {
            path: path.clone(),
            source: Box::new(source),
        })?;
    normalize(path, source, declaration)
}

/// Reads, path-sorts, normalizes, and identity-deduplicates owned sources.
///
/// # Errors
/// If any source is invalid or two sources declare the same category-safe
/// identity. The duplicate error points at the later path in sorted order and
/// names the first.
pub fn read_sources(
    mut sources: Vec<DeclarationSource>,
) -> Result<Vec<NormalizedDeclaration>, ReadError> {
    sources.sort_by(|left, right| left.path.cmp(&right.path));
    let mut first_by_identity: HashMap<DeclarationIdentity, PathBuf> = HashMap::new();
    let mut declarations = Vec::with_capacity(sources.len());
    for source in sources {
        let declaration = read_str(source.path.clone(), &source.source)?;
        if let Some(first_path) = first_by_identity.get(&declaration.identity) {
            return Err(validation_error(
                &source.path,
                &source.source,
                "name",
                ValidationError::DuplicateIdentity {
                    identity: declaration.identity,
                    first_path: first_path.clone(),
                },
            ));
        }
        first_by_identity.insert(declaration.identity.clone(), source.path);
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
        return Err(validation_error(
            root,
            "",
            "",
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

    let declarations = read_sources(sources)?;
    for declaration in &declarations {
        let (expected_kind, expected_name) = expected_builtin_identity(&nursery, declaration)?;
        if declaration.identity.kind != expected_kind {
            let source =
                std::fs::read_to_string(&declaration.provenance.path).map_err(|source| {
                    ReadError::Io {
                        path: declaration.provenance.path.clone(),
                        source,
                    }
                })?;
            return Err(validation_error_at(
                &declaration.provenance.path,
                builtin_kind_mismatch_position(&source, declaration.identity.kind),
                ValidationError::DeclarationKindMismatch {
                    expected: expected_kind,
                    actual: declaration.identity.kind,
                },
            ));
        }
        if declaration.identity.name != expected_name {
            return Err(validation_error(
                &declaration.provenance.path,
                &std::fs::read_to_string(&declaration.provenance.path).map_err(|source| {
                    ReadError::Io {
                        path: declaration.provenance.path.clone(),
                        source,
                    }
                })?,
                "name",
                ValidationError::DeclarationNameMismatch {
                    expected: expected_name,
                    actual: declaration.identity.name.clone(),
                },
            ));
        }
    }
    Ok(declarations)
}

fn normalize(
    path: PathBuf,
    source: &str,
    declaration: Declaration,
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
        return Err(validation_error(
            &path,
            source,
            "name",
            ValidationError::InvalidName { name },
        ));
    }
    if body.is_some() && params.is_none() {
        return Err(validation_error(
            &path,
            source,
            "body",
            ValidationError::BodyWithoutSignature,
        ));
    }

    let spelling_parts = parse_spelling(&spelling).map_err(|reason| {
        validation_error(
            &path,
            source,
            "spelling",
            ValidationError::InvalidSpelling { reason },
        )
    })?;
    validate_spelling_params(&path, source, &spelling_parts, params.as_deref())?;
    validate_body_params(&path, source, body.as_deref(), params.as_deref())?;

    let identity = DeclarationIdentity { kind, name };
    let grammar = grammar
        .map(|grammar| normalize_grammar(&path, source, &spelling_parts, grammar))
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
    source: &str,
    spelling: &[SpellingPart],
    grammar: Grammar,
) -> Result<GrammarRow, ReadError> {
    let (grammar_head, recipe, surfaces) = match grammar {
        Grammar::Verb {
            bare,
            third_person,
            valence,
        } => {
            validate_surface(path, source, "bare", &bare)?;
            validate_valence(path, source, &valence)?;
            let third_person = realize_derived_surface(
                path,
                source,
                "third_person",
                english_verb(&bare),
                third_person,
            )?;
            let mut surfaces = vec![RealizedSurface {
                feature: SurfaceFeature::Bare,
                text: bare.clone(),
            }];
            if let Some(text) = third_person {
                surfaces.push(RealizedSurface {
                    feature: SurfaceFeature::ThirdPersonSingular,
                    text,
                });
            }
            (bare, GrammarRecipe::Verb { valence }, surfaces)
        }
        Grammar::Noun { singular, plural } => {
            validate_surface(path, source, "singular", &singular)?;
            let plural =
                realize_derived_surface(path, source, "plural", english_noun(&singular), plural)?;
            let mut surfaces = vec![RealizedSurface {
                feature: SurfaceFeature::Singular,
                text: singular.clone(),
            }];
            if let Some(text) = plural {
                surfaces.push(RealizedSurface {
                    feature: SurfaceFeature::Plural,
                    text,
                });
            }
            (singular, GrammarRecipe::Noun, surfaces)
        }
        Grammar::FixedTerm { surface } => {
            fixed_grammar(path, source, surface, GrammarRecipe::FixedTerm)?
        }
        Grammar::FixedClause { surface } => {
            fixed_grammar(path, source, surface, GrammarRecipe::FixedClause)?
        }
        Grammar::FixedKeyword { surface } => {
            fixed_grammar(path, source, surface, GrammarRecipe::FixedKeyword)?
        }
    };

    let spelling_head = spelling_head(spelling);
    if spelling_head != grammar_head {
        return Err(validation_error(
            path,
            source,
            "spelling",
            ValidationError::GrammarSpellingMismatch {
                spelling_head,
                grammar_head,
            },
        ));
    }

    Ok(GrammarRow { recipe, surfaces })
}

fn fixed_grammar(
    path: &Path,
    source: &str,
    surface: String,
    recipe: GrammarRecipe,
) -> Result<(String, GrammarRecipe, Vec<RealizedSurface>), ReadError> {
    validate_surface(path, source, "surface", &surface)?;
    Ok((
        surface.clone(),
        recipe,
        vec![RealizedSurface {
            feature: SurfaceFeature::Fixed,
            text: surface,
        }],
    ))
}

fn realize_derived_surface(
    path: &Path,
    source: &str,
    field: &'static str,
    derived: String,
    authored: DerivedSurface,
) -> Result<Option<String>, ReadError> {
    match authored {
        DerivedSurface::Derived => Ok(Some(derived)),
        DerivedSurface::Unavailable => Ok(None),
        DerivedSurface::Override(surface) => {
            validate_surface(path, source, field, &surface)?;
            if surface == derived {
                return Err(validation_error(
                    path,
                    source,
                    field,
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

fn english_noun(singular: &str) -> String {
    format!("{singular}s")
}

fn validate_valence(path: &Path, source: &str, valence: &VerbValence) -> Result<(), ReadError> {
    let VerbValence::Custom { shapes } = valence else {
        return Ok(());
    };
    if shapes.is_empty() {
        return Err(validation_error(
            path,
            source,
            "Custom",
            ValidationError::EmptyCustomShapeSet,
        ));
    }
    let mut seen = HashSet::new();
    for shape in shapes {
        if !seen.insert(shape) {
            return Err(validation_error(
                path,
                source,
                "Custom",
                ValidationError::DuplicateCustomShape {
                    shape: shape.clone(),
                },
            ));
        }
        for atom in shape {
            if let CustomTailAtom::Literal(literal) = atom
                && !valid_surface(literal)
            {
                return Err(validation_error(
                    path,
                    source,
                    "Literal",
                    ValidationError::InvalidCustomLiteral,
                ));
            }
        }
    }
    Ok(())
}

fn validate_surface(
    path: &Path,
    source: &str,
    field: &'static str,
    surface: &str,
) -> Result<(), ReadError> {
    if !valid_surface(surface) {
        return Err(validation_error(
            path,
            source,
            field,
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
    source: &str,
    spelling: &[SpellingPart],
    params: Option<&[ParameterType]>,
) -> Result<(), ReadError> {
    for part in spelling {
        let SpellingPart::Param(index) = part else {
            continue;
        };
        let len = params.map_or(0, <[ParameterType]>::len);
        if params.is_none() || *index >= len {
            return Err(validation_error(
                path,
                source,
                &format!("<Param({index})>"),
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
    source: &str,
    body: Option<&RawValue>,
    params: Option<&[ParameterType]>,
) -> Result<(), ReadError> {
    let Some(body) = body else {
        return Ok(());
    };
    let mut keys = Vec::new();
    crate::expand::collect_param_keys(body.get_ron(), &ron_options(), &mut keys).map_err(
        |reason| {
            validation_error(
                path,
                source,
                "body",
                ValidationError::InvalidBody { reason },
            )
        },
    )?;
    let len = params.map_or(0, <[ParameterType]>::len);
    for key in keys {
        match key {
            crate::expand::ParamKey::Index(index) if index < len => {}
            crate::expand::ParamKey::Index(index) => {
                return Err(validation_error(
                    path,
                    source,
                    &format!("Param({index})"),
                    ValidationError::ParamOutOfRange {
                        location: "body",
                        index,
                        len,
                    },
                ));
            }
            crate::expand::ParamKey::Name(name) => {
                return Err(validation_error(
                    path,
                    source,
                    &format!("Param({name})"),
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

fn builtin_kind_mismatch_position(source: &str, kind: DeclarationKind) -> SourcePosition {
    match kind {
        DeclarationKind::Subtype(category) => locate_offset(
            source,
            subtype_category_value_offset(source, category)
                .expect("subtype category field must occur in source"),
        ),
        kind => locate(source, declaration_kind_needle(kind)),
    }
}

fn declaration_kind_needle(kind: DeclarationKind) -> &'static str {
    match kind {
        DeclarationKind::KeywordAction => "KeywordAction",
        DeclarationKind::KeywordAbility => "KeywordAbility",
        DeclarationKind::Type => "Type",
        DeclarationKind::CounterKind => "CounterKind",
        DeclarationKind::Designation => "Designation",
        DeclarationKind::Subtype(_) => unreachable!("subtypes use their category field"),
    }
}

fn validation_error(path: &Path, source: &str, needle: &str, error: ValidationError) -> ReadError {
    validation_error_at(path, locate(source, needle), error)
}

fn validation_error_at(path: &Path, position: SourcePosition, error: ValidationError) -> ReadError {
    ReadError::Validate {
        path: path.to_owned(),
        position,
        source: error,
    }
}

fn locate(source: &str, needle: &str) -> SourcePosition {
    let offset = if needle.is_empty() {
        0
    } else {
        source
            .find(needle)
            .expect("validation location needle must occur in source")
    };
    locate_offset(source, offset)
}

fn locate_offset(source: &str, offset: usize) -> SourcePosition {
    let before = &source[..offset];
    let line = before.bytes().filter(|byte| *byte == b'\n').count() + 1;
    let column = before.rsplit_once('\n').map_or_else(
        || before.chars().count() + 1,
        |(_, line)| line.chars().count() + 1,
    );
    SourcePosition { line, column }
}

fn subtype_category_value_offset(source: &str, category: SubtypeCategory) -> Option<usize> {
    let expected = match category {
        SubtypeCategory::Artifact => "Artifact",
        SubtypeCategory::Battle => "Battle",
        SubtypeCategory::Creature => "Creature",
        SubtypeCategory::Enchantment => "Enchantment",
        SubtypeCategory::Land => "Land",
        SubtypeCategory::Planeswalker => "Planeswalker",
        SubtypeCategory::Spell => "Spell",
    };
    let mut offset = 0;
    while offset < source.len() {
        if let Some(next) = skip_ron_string_or_comment(source, offset) {
            offset = next;
            continue;
        }
        if source[offset..].starts_with("category")
            && identifier_boundary(source, offset, "category")
        {
            let mut value = skip_ron_trivia(source, offset + "category".len());
            if source[value..].starts_with(':') {
                value = skip_ron_trivia(source, value + 1);
                if source[value..].starts_with(expected)
                    && identifier_boundary(source, value, expected)
                {
                    return Some(value);
                }
            }
        }
        offset += source[offset..].chars().next().unwrap().len_utf8();
    }
    None
}

fn identifier_boundary(source: &str, offset: usize, token: &str) -> bool {
    let before = source[..offset].chars().next_back();
    let after = source[offset + token.len()..].chars().next();
    !before.is_some_and(is_ron_ident_char) && !after.is_some_and(is_ron_ident_char)
}

fn is_ron_ident_char(character: char) -> bool {
    character.is_ascii_alphanumeric() || character == '_'
}

fn skip_ron_trivia(source: &str, mut offset: usize) -> usize {
    loop {
        while let Some(character) = source[offset..].chars().next() {
            if !character.is_whitespace() {
                break;
            }
            offset += character.len_utf8();
        }
        if source[offset..].starts_with("//") {
            offset = source[offset..]
                .find('\n')
                .map_or(source.len(), |end| offset + end + 1);
            continue;
        }
        if source[offset..].starts_with("/*") {
            let end = source[offset + 2..]
                .find("*/")
                .expect("terminated RON block comment");
            offset += end + 4;
            continue;
        }
        return offset;
    }
}

fn skip_ron_string_or_comment(source: &str, offset: usize) -> Option<usize> {
    if source[offset..].starts_with('"') {
        let mut next = offset + 1;
        while next < source.len() {
            let character = source[next..].chars().next().unwrap();
            next += character.len_utf8();
            if character == '\\' {
                next += source[next..].chars().next().unwrap().len_utf8();
            } else if character == '"' {
                return Some(next);
            }
        }
        panic!("terminated RON string");
    }
    if source[offset..].starts_with("//") || source[offset..].starts_with("/*") {
        return Some(skip_ron_trivia(source, offset));
    }
    None
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
) -> Result<(DeclarationKind, String), ReadError> {
    let path = &declaration.provenance.path;
    let relative = path.strip_prefix(nursery).map_err(|_| {
        validation_error(
            path,
            "",
            "",
            ValidationError::UnexpectedBuiltinLocation {
                relative: path.clone(),
            },
        )
    })?;
    let components = relative
        .iter()
        .map(|component| component.to_str())
        .collect::<Option<Vec<_>>>();
    let Some(components) = components else {
        return Err(validation_error(
            path,
            "",
            "",
            ValidationError::UnexpectedBuiltinLocation {
                relative: relative.to_owned(),
            },
        ));
    };
    let Some(stem) = path.file_stem().and_then(|stem| stem.to_str()) else {
        return Err(validation_error(
            path,
            "",
            "",
            ValidationError::UnexpectedBuiltinLocation {
                relative: relative.to_owned(),
            },
        ));
    };
    let kind = match components.as_slice() {
        ["keyword_actions", _] => DeclarationKind::KeywordAction,
        ["keyword_abilities", _] => DeclarationKind::KeywordAbility,
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
                return Err(validation_error(
                    path,
                    "",
                    "",
                    ValidationError::UnexpectedBuiltinLocation {
                        relative: relative.to_owned(),
                    },
                ));
            }
        }),
        _ => {
            return Err(validation_error(
                path,
                "",
                "",
                ValidationError::UnexpectedBuiltinLocation {
                    relative: relative.to_owned(),
                },
            ));
        }
    };
    Ok((kind, stem.to_owned()))
}

#[cfg(test)]
mod tests;
