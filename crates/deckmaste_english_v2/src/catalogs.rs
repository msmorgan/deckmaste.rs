use std::path::Path;
use std::sync::Arc;

use deckmaste_catalogs::CatalogSet;

use crate::catalog_compatibility::CatalogCompatibility;
use crate::environment::ParserEnvironment;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParserCatalogs(Arc<CatalogSet>);

impl ParserCatalogs {
    #[must_use]
    pub fn new(catalogs: CatalogSet) -> Self {
        Self(Arc::new(catalogs))
    }

    /// Loads every parser catalog from `path`.
    ///
    /// # Errors
    ///
    /// Returns an error when a canonical catalog file is missing or malformed.
    pub fn load(path: &Path) -> anyhow::Result<Self> {
        CatalogSet::load(path).map(Self::new)
    }

    #[must_use]
    pub fn set(&self) -> &CatalogSet {
        &self.0
    }

    /// Attaches the temporary catalog-backed noun source at the provider
    /// boundary.
    ///
    /// This adapter disappears when type and subtype declarations replace the
    /// legacy catalogs. It deliberately requires an already-built environment
    /// rather than acting as a second environment constructor.
    #[must_use]
    pub fn attach_to(self, environment: ParserEnvironment) -> ParserEnvironment {
        environment.with_catalog_compatibility(CatalogCompatibility::new(self.0))
    }
}

#[cfg(test)]
pub(crate) fn canonical_test_environment() -> ParserEnvironment {
    let environment = ParserEnvironment::try_from_declarations([])
        .expect("empty declaration environment freezes");
    ParserCatalogs::load(&Path::new(env!("CARGO_MANIFEST_DIR")).join("../../data/gen/catalogs"))
        .expect("canonical generated catalogs load")
        .attach_to(environment)
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeMap;
    use std::collections::BTreeSet;
    use std::path::Path;

    use deckmaste_catalogs::CatalogKind;
    use deckmaste_catalogs::CatalogSet;
    use tempfile::tempdir;

    use super::ParserCatalogs;

    #[test]
    fn parser_catalogs_load_preserves_every_catalog_kind() {
        let expected = BTreeMap::from([
            (
                CatalogKind::AbilityWords,
                BTreeSet::from(["Ability".into()]),
            ),
            (
                CatalogKind::ArtifactTypes,
                BTreeSet::from(["Artifact".into()]),
            ),
            (CatalogKind::BattleTypes, BTreeSet::from(["Battle".into()])),
            (CatalogKind::CardNames, BTreeSet::from(["Card".into()])),
            (CatalogKind::CardTypes, BTreeSet::from(["Creature".into()])),
            (
                CatalogKind::CounterKindPhrases,
                BTreeSet::from(["Counter".into()]),
            ),
            (
                CatalogKind::CreatureTypes,
                BTreeSet::from(["Dragon".into()]),
            ),
            (
                CatalogKind::EnchantmentTypes,
                BTreeSet::from(["Aura".into()]),
            ),
            (
                CatalogKind::KeywordAbilities,
                BTreeSet::from(["Flying".into()]),
            ),
            (CatalogKind::KeywordActions, BTreeSet::from(["Scry".into()])),
            (CatalogKind::LandTypes, BTreeSet::from(["Island".into()])),
            (
                CatalogKind::PlaneswalkerTypes,
                BTreeSet::from(["Jace".into()]),
            ),
            (CatalogKind::SpellTypes, BTreeSet::from(["Arcane".into()])),
            (
                CatalogKind::Supertypes,
                BTreeSet::from(["Legendary".into()]),
            ),
        ]);
        let expected = CatalogSet::from_entries(expected).unwrap();

        let temporary = tempdir().unwrap();
        let path = temporary.path().join("catalogs");
        expected.write_to(&path).unwrap();

        let loaded = ParserCatalogs::load(&path).unwrap();

        for kind in CatalogKind::ALL {
            assert_eq!(loaded.set().get(kind), expected.get(kind));
        }
    }

    #[test]
    fn real_generated_catalogs_all_load() {
        let path = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../data/gen/catalogs");
        assert!(
            path.is_dir(),
            "missing real generated catalogs at {} — run `cargo xtask catalogs generate`",
            path.display()
        );

        let loaded = ParserCatalogs::load(&path).unwrap();

        for kind in CatalogKind::ALL {
            assert!(
                !loaded.set().get(kind).is_empty(),
                "catalog {} loaded empty from {}",
                kind.filename(),
                path.display()
            );
        }
    }
}
