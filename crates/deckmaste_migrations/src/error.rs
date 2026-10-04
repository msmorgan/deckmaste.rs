use std::path::PathBuf;

#[derive(Debug, thiserror::Error)]
pub enum LayoutError {
    #[error("{operation} {}: {source}", path.display())]
    Io {
        operation: &'static str,
        path: PathBuf,
        #[source]
        source: std::io::Error,
    },
    #[error("Plugin base path is not a directory: {}", path.display())]
    NotDirectory { path: PathBuf },
    #[error("path {} is outside of plugin layout {}", path.display(), root.display())]
    OutsideRoot { path: PathBuf, root: PathBuf },
}

#[derive(Debug, thiserror::Error)]
pub enum ExtractError {
    #[error(transparent)]
    Layout(#[from] LayoutError),
    #[error(transparent)]
    Data(#[from] deckmaste_data::DataError),
    #[error(transparent)]
    Catalog(#[from] deckmaste_catalogs::CatalogError),
    #[error("{operation} {}: {source}", path.display())]
    Io {
        operation: &'static str,
        path: PathBuf,
        #[source]
        source: std::io::Error,
    },
    #[error("reading {}: {source}", path.display())]
    Oracle {
        path: PathBuf,
        #[source]
        source: deckmaste_data::scryfall::OracleCardReadError,
    },
    #[error("decoding keyword snapshot: {0}")]
    Keywords(#[from] serde_json::Error),
    #[error("Scryfall Oracle unit {name:?} has no type_line")]
    MissingTypeLine { name: String },
    #[error("unrecognized color indicator: {code:?}")]
    ColorIndicator { code: String },
    #[error("parsing mana cost for {name:?}: {source}")]
    ManaCost {
        name: String,
        #[source]
        source: deckmaste_semantics::ParseManaError,
    },
    #[error("rendering extracted card: {0}")]
    Render(#[from] ron::Error),
}

#[derive(Debug, thiserror::Error)]
pub enum StubError {
    #[error(transparent)]
    Layout(#[from] LayoutError),
    #[error(transparent)]
    Data(#[from] deckmaste_data::DataError),
    #[error("parsing subtype catalog {category}: {source}")]
    Catalog {
        category: String,
        #[source]
        source: serde_json::Error,
    },
    #[error(
        "subtype idents collide: {first:?} and {second:?} both produce `{ident}` in {category}"
    )]
    IdentCollision {
        category: String,
        first: String,
        second: String,
        ident: String,
    },
    #[error("writing {}: {source}", path.display())]
    Write {
        path: PathBuf,
        #[source]
        source: std::io::Error,
    },
}
