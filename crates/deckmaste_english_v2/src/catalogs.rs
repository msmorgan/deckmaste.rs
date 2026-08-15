use std::path::Path;
use std::sync::Arc;

use deckmaste_catalogs::CatalogSet;

#[derive(Debug, Clone)]
pub struct ParserCatalogs(Arc<CatalogSet>);

impl ParserCatalogs {
    #[must_use]
    pub fn new(catalogs: CatalogSet) -> Self {
        Self(Arc::new(catalogs))
    }

    pub fn load(path: &Path) -> anyhow::Result<Self> {
        CatalogSet::load(path).map(Self::new)
    }

    #[must_use]
    pub fn set(&self) -> &CatalogSet {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeMap;
    use std::collections::BTreeSet;

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
}
