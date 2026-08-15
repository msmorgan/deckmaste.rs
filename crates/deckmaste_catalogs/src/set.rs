use std::collections::BTreeMap;
use std::collections::BTreeSet;

use anyhow::bail;

use crate::CatalogKind;
use crate::cards;
use crate::cr;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CatalogSet {
    catalogs: BTreeMap<CatalogKind, BTreeSet<String>>,
}

impl CatalogSet {
    /// Extracts every canonical catalog from the Comprehensive Rules and
    /// Vintage-playable `AtomicCards` faces.
    ///
    /// # Errors
    ///
    /// Returns an error when an authoritative input is malformed or cannot
    /// produce the complete canonical inventory.
    pub fn generate(cr: &str, atomic: &[u8]) -> anyhow::Result<Self> {
        let mut entries = cr::extract(cr)?;
        entries.insert(CatalogKind::CardNames, cards::extract(atomic)?);
        Self::from_entries(entries)
    }

    /// Builds a catalog set only when every canonical catalog is present.
    ///
    /// # Errors
    ///
    /// Returns an error naming the first missing catalog kind.
    pub fn from_entries(catalogs: BTreeMap<CatalogKind, BTreeSet<String>>) -> anyhow::Result<Self> {
        for kind in CatalogKind::ALL {
            if !catalogs.contains_key(&kind) {
                bail!("missing {} catalog", kind.filename());
            }
        }
        Ok(Self { catalogs })
    }

    #[must_use]
    pub fn get(&self, kind: CatalogKind) -> &BTreeSet<String> {
        &self.catalogs[&kind]
    }

    pub fn kinds(&self) -> impl Iterator<Item = CatalogKind> {
        CatalogKind::ALL.into_iter()
    }
}
