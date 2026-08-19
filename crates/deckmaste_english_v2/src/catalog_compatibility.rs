use std::sync::Arc;

use deckmaste_catalogs::CatalogKind;
use deckmaste_catalogs::CatalogSet;

/// Temporary legacy noun source until type and subtype declaration rows land.
///
/// Delete this entire module once type and subtype declaration rows supply the
/// noun inventory. It is never an independent grammar authority and is
/// reachable only through `ParserEnvironment`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct CatalogCompatibility(Arc<CatalogSet>);

impl CatalogCompatibility {
    pub(crate) fn new(catalogs: Arc<CatalogSet>) -> Self {
        Self(catalogs)
    }

    pub(crate) fn spellings(&self, kind: CatalogKind) -> impl Iterator<Item = &str> {
        self.0.get(kind).iter().map(String::as_str)
    }

    pub(crate) fn contains(&self, kind: CatalogKind, spelling: &str) -> bool {
        self.0.get(kind).contains(spelling)
    }

    #[cfg(test)]
    pub(crate) fn shares_storage_with(&self, other: &Self) -> bool {
        Arc::ptr_eq(&self.0, &other.0)
    }
}
