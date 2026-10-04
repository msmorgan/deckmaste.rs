use std::path::{Path, PathBuf};

/// Failure to generate, load, compare, or replace a catalog inventory.
#[derive(Debug, thiserror::Error)]
pub enum CatalogError {
    #[error("{operation} {}: {source}", path.display())]
    Io {
        operation: &'static str,
        path: PathBuf,
        #[source]
        source: std::io::Error,
    },
    #[error("parsing {}: {source}", path.display())]
    Json {
        path: PathBuf,
        #[source]
        source: serde_json::Error,
    },
    #[error("parsing Scryfall Oracle Cards JSONL: {0}")]
    Oracle(#[from] deckmaste_data::scryfall::OracleCardReadError),
    #[error("missing {catalog} catalog")]
    MissingCatalog { catalog: String },
    #[error("blank entry in {} at line {line}", path.display())]
    BlankEntry { path: PathBuf, line: usize },
    #[error("catalog {catalog} must contain at least one entry")]
    EmptyCatalog { catalog: String },
    #[error("catalog {catalog} contains an empty or multiline entry {entry:?}")]
    InvalidEntry { catalog: String, entry: String },
    #[error("type line {type_line:?} contains no declared card type")]
    MissingCardType { type_line: String },
    #[error("card-names: composite name without a selected face: {name:?}")]
    CompositeCardName { name: String },
    #[error("{catalog}: expected one {rule} body rule, found {found}")]
    RuleCount {
        catalog: String,
        rule: String,
        found: usize,
    },
    #[error("{catalog}: expected {lead:?}")]
    MissingLead { catalog: String, lead: String },
    #[error("{catalog}: expected list sentence to end with a period")]
    UnterminatedList { catalog: String },
    #[error("{catalog}: blank CR list member at position {position}")]
    BlankListMember { catalog: String, position: usize },
    #[error("{catalog}: empty CR list")]
    EmptyList { catalog: String },
    #[error("{catalog}: expected {section}. {title}, found section-title lookalike {line:?}")]
    SectionTitle {
        catalog: String,
        section: u16,
        title: String,
        line: String,
    },
    #[error("{catalog}: malformed {section} keyword heading {line:?}")]
    MalformedHeading {
        catalog: String,
        section: u16,
        line: String,
    },
    #[error("{catalog}: invalid {section} heading number in {line:?}: {source}")]
    HeadingNumber {
        catalog: String,
        section: u16,
        line: String,
        #[source]
        source: std::num::ParseIntError,
    },
    #[error("{catalog}: invalid {section}.0 heading")]
    ZeroHeading { catalog: String, section: u16 },
    #[error("{catalog}: duplicate {section}.{number} heading")]
    DuplicateHeading {
        catalog: String,
        section: u16,
        number: u16,
    },
    #[error("{catalog}: blank {section}.{number} heading name")]
    BlankHeading {
        catalog: String,
        section: u16,
        number: u16,
    },
    #[error("{catalog}: found no {section} keyword headings")]
    MissingHeadings { catalog: String, section: u16 },
    #[error("counter-kind-phrases: expected open-variant suffix")]
    MissingCounterSuffix,
    #[error("catalog output {} {problem}", path.display())]
    InvalidOutput {
        path: PathBuf,
        problem: OutputProblem,
    },
    #[error("expected non-file entry {}", path.display())]
    ExpectedNonFile { path: PathBuf },
    #[error("shared non-file entry {}", path.display())]
    SharedNonFile { path: PathBuf },
    #[error("restoring catalog output {} after replacement failure: {replacement}: {source}", output.display())]
    Restore {
        output: PathBuf,
        backup: PathBuf,
        staging: PathBuf,
        replacement: std::io::Error,
        #[source]
        source: std::io::Error,
    },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, thiserror::Error)]
pub enum OutputProblem {
    #[error("must have a parent")]
    MissingParent,
    #[error("must name a directory")]
    MissingName,
    #[error("must not be empty")]
    Empty,
    #[error("must not be a filesystem root")]
    FilesystemRoot,
    #[error("must not be the current directory or its ancestor")]
    CurrentDirectory,
    #[error("must not be the project workspace root or its ancestor")]
    WorkspaceRoot,
    #[error("must be a real directory when it already exists")]
    NotDirectory,
    #[error("contains a non-regular entry")]
    NonRegularEntry,
    #[error("contains an unresolved path component")]
    UnresolvedComponent,
}

pub(crate) trait IoContext<T> {
    fn at(self, operation: &'static str, path: &Path) -> Result<T, CatalogError>;
}
impl<T> IoContext<T> for std::io::Result<T> {
    fn at(self, operation: &'static str, path: &Path) -> Result<T, CatalogError> {
        self.map_err(|source| CatalogError::Io {
            operation,
            path: path.into(),
            source,
        })
    }
}
