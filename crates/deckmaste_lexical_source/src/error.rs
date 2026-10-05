use deckmaste_lexical::WordForm;
use std::path::PathBuf;

/// Failure to load or reconcile authored lexical declarations.
#[derive(Debug, thiserror::Error)]
pub enum LoadError {
    #[error("reading card-name source: {0}")]
    Oracle(#[from] deckmaste_data::scryfall::OracleCardReadError),
    #[error("{operation} {}: {source}", path.display())]
    Io {
        operation: &'static str,
        path: PathBuf,
        #[source]
        source: std::io::Error,
    },
    #[error("decoding {}: {source}", path.display())]
    Decode {
        path: PathBuf,
        #[source]
        source: Box<ron::error::SpannedError>,
    },
    #[error("reading vocabulary {}: {source}", path.display())]
    Vocabulary {
        path: PathBuf,
        #[source]
        source: syn::Error,
    },
    #[error("reading authored lexical metadata: {0}")]
    Declaration(#[from] deckmaste_construction_core::macro_def::ReadError),
    #[error("building declaration reader: {0}")]
    DeclarationReader(#[from] deckmaste_construction_core::macro_def::DeclarationReaderError),
    #[error("loading canonical lexical catalogs: {0}")]
    Catalog(#[from] deckmaste_catalogs::CatalogError),
    #[error("unresolved core verb paradigm owners: {owners:?}")]
    UnresolvedParadigms { owners: Vec<String> },
    #[error("duplicate lexical export identity {identity}")]
    DuplicateIdentity {
        identity: deckmaste_lexical::LexemeId,
    },
    #[error("unknown {property} owner {owner}")]
    UnknownOwner {
        property: &'static str,
        owner: String,
    },
    #[error("invalid adjective class member {owner:?}: {reason}")]
    InvalidAdjectiveClassMember { owner: String, reason: &'static str },
    #[error("participial adjective source {owner}: {source}")]
    ParticipialAdjectiveLexical {
        owner: String,
        #[source]
        source: deckmaste_lexical::LexicalError,
    },
    #[error("invalid compound noun class: {reason}")]
    InvalidCompoundClass { reason: &'static str },
    #[error("compound head {owner}: {source}")]
    CompoundHeadLexical {
        owner: String,
        #[source]
        source: deckmaste_lexical::LexicalError,
    },
    #[error("compound head {owner} must be a declared noun")]
    InvalidCompoundHead { owner: String },
    #[error("duplicate added frame for {owner}")]
    DuplicateFrame { owner: String },
    #[error("{owner}: empty frame kind")]
    EmptyFrameKind { owner: String },
    #[error("empty feature declaration {name:?} for {owner}")]
    EmptyFeature {
        owner: String,
        name: String,
        value: String,
    },
    #[error("duplicate feature {name} for {owner}")]
    DuplicateFeature { owner: String, name: String },
    #[error(
        "frame marker {vocabulary}/{member} must resolve to one declared identity (found {found})"
    )]
    MarkerIdentity {
        vocabulary: String,
        member: String,
        found: usize,
    },
    #[error(
        "frame marker {vocabulary}/{member} does not spell declared literal {surface:?} (spells {lemma:?})"
    )]
    MarkerSpelling {
        vocabulary: String,
        member: String,
        surface: String,
        lemma: String,
    },
    #[error("unresolved frame literal {surface:?}")]
    UnresolvedLiteral { surface: String },
    #[error("empty frame slot category")]
    EmptySlotCategory,
    #[error("frame for {owner}: {source}")]
    Frame {
        owner: String,
        #[source]
        source: Box<LoadError>,
    },
    #[error("{path}: unknown override owner {owner}")]
    UnknownOverrideOwner { path: PathBuf, owner: String },
    #[error("{path}: overlapping overrides for {owner} {form:?}")]
    OverlappingOverrides {
        path: PathBuf,
        owner: String,
        form: WordForm,
    },
    #[error(
        "{path}: override for {owner} {form:?} discards an authored surface {authored:?}; reconcile its source"
    )]
    DiscardedSurfaces {
        path: PathBuf,
        owner: String,
        form: WordForm,
        authored: Vec<String>,
    },
    #[error("{path}: no slot selected for {owner} {form:?}")]
    NoOverrideSlot {
        path: PathBuf,
        owner: String,
        form: WordForm,
    },
    #[error("no authored core verb inventory found in the transitional source tree")]
    MissingVerbInventory,
    #[error(
        "multiple authored core verb inventories found in the transitional source tree: {paths:?}"
    )]
    MultipleVerbInventories { paths: Vec<PathBuf> },
    #[error("{owner}: unmapped declared countability {value:?}")]
    Countability {
        owner: String,
        value: Option<String>,
    },
    #[error("{owner}: unmapped noun inflection {value}")]
    NounInflection { owner: String, value: String },
    #[error("{owner}: override for unavailable {form:?}")]
    UnavailableForm { owner: String, form: WordForm },
}

pub(crate) fn read(path: &std::path::Path) -> Result<String, LoadError> {
    std::fs::read_to_string(path).map_err(|source| LoadError::Io {
        operation: "reading",
        path: path.into(),
        source,
    })
}

pub(crate) fn decode<T: serde::de::DeserializeOwned>(
    path: &std::path::Path,
    text: &str,
) -> Result<T, LoadError> {
    ron::from_str(text).map_err(|source| LoadError::Decode {
        path: path.into(),
        source: Box::new(source),
    })
}
