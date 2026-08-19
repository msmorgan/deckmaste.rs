//! Immutable, data-defined grammar vocabulary for the English v2 parser.

use std::collections::BTreeMap;
use std::path::Path;
use std::path::PathBuf;
use std::sync::Arc;

pub use macro_ron::v2::DeclarationIdentity as DeclarationId;
use macro_ron::v2::DeclarationKind;
pub use macro_ron::v2::GrammarPosition;
use macro_ron::v2::GrammarRecipe;
use macro_ron::v2::NormalizedDeclaration;
use macro_ron::v2::SurfaceFeature;
use macro_ron::v2::VerbValence;

use crate::catalog_compatibility::CatalogCompatibility;
use crate::orthography::initial_surface;

/// One declaration and its validated grammar metadata.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DeclarationRecord {
    id: DeclarationId,
    recipe: Option<GrammarRecipe>,
    surfaces: Vec<(SurfaceFeature, Arc<str>)>,
    provenance: PathBuf,
}

impl DeclarationRecord {
    /// Returns this declaration's category-safe identity.
    #[must_use]
    pub fn id(&self) -> &DeclarationId {
        &self.id
    }

    /// Returns the declaration's normalized grammar recipe, when authored.
    #[must_use]
    pub fn recipe(&self) -> Option<&GrammarRecipe> {
        self.recipe.as_ref()
    }

    /// Returns the verb valence when this is a verb declaration.
    #[must_use]
    pub fn valence(&self) -> Option<&VerbValence> {
        match self.recipe.as_ref() {
            Some(GrammarRecipe::Verb { valence }) => Some(valence),
            Some(
                GrammarRecipe::Noun
                | GrammarRecipe::FixedTerm
                | GrammarRecipe::FixedClause
                | GrammarRecipe::FixedKeyword,
            )
            | None => None,
        }
    }

    /// Returns the normalized declaration's source path.
    #[must_use]
    pub fn provenance(&self) -> &Path {
        &self.provenance
    }

    fn surface(&self, feature: SurfaceFeature) -> Option<&str> {
        self.surfaces
            .iter()
            .find_map(|(candidate, surface)| (*candidate == feature).then_some(surface.as_ref()))
    }
}

/// One exact surface reading retained by the environment's forward index.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DeclarationReading {
    id: DeclarationId,
    position: GrammarPosition,
    feature: SurfaceFeature,
    surface: Arc<str>,
}

impl DeclarationReading {
    /// Returns the declaration selected by this reading.
    #[must_use]
    pub fn id(&self) -> &DeclarationId {
        &self.id
    }

    /// Returns the closed grammar position compatible with this reading.
    #[must_use]
    pub fn position(&self) -> GrammarPosition {
        self.position
    }

    /// Returns the normalized realized feature for this reading.
    #[must_use]
    pub fn feature(&self) -> SurfaceFeature {
        self.feature
    }

    /// Returns the exact authored or dumb-derived surface.
    #[must_use]
    pub fn surface(&self) -> &str {
        &self.surface
    }
}

/// A failure while freezing normalized declarations into a parser environment.
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum ParserEnvironmentError {
    /// Two supplied rows claim the same category-safe identity.
    #[error(
        "duplicate declaration {identity}; first supplied from `{first_path}`, duplicate from `{duplicate_path}`"
    )]
    DuplicateIdentity {
        identity: DeclarationId,
        first_path: PathBuf,
        duplicate_path: PathBuf,
    },

    /// A normalized declaration repeats one realized feature.
    ///
    /// The current schema prevents this, but retaining the check keeps the
    /// environment boundary sealed if the schema later gains richer recipes.
    #[error("declaration {identity} repeats realized feature {feature:?}")]
    DuplicateSurfaceFeature {
        identity: DeclarationId,
        feature: SurfaceFeature,
    },
}

#[derive(Debug, PartialEq, Eq)]
struct EnvironmentData {
    declarations: BTreeMap<DeclarationKind, BTreeMap<Arc<str>, DeclarationRecord>>,
    readings: BTreeMap<GrammarPosition, BTreeMap<Arc<str>, Vec<DeclarationReading>>>,
    initial_readings: BTreeMap<GrammarPosition, BTreeMap<Arc<str>, Vec<DeclarationReading>>>,
}

/// One immutable vocabulary and grammar-row environment for a parser.
///
/// Clones share the frozen indexes. Construction performs no file discovery,
/// catalog lookup, or process-global registration.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParserEnvironment {
    data: Arc<EnvironmentData>,
    // Deleted with the catalog compatibility module once type/subtype
    // declaration rows supply the noun inventory.
    catalog_compatibility: Option<CatalogCompatibility>,
}

impl ParserEnvironment {
    /// Freezes normalized declarations into deterministic bidirectional
    /// indexes.
    ///
    /// # Errors
    /// Returns a typed error if the supplied stream violates identity or
    /// feature uniqueness already guaranteed by an ordinary v2 source set.
    pub fn try_from_declarations(
        declarations: impl IntoIterator<Item = NormalizedDeclaration>,
    ) -> Result<Self, ParserEnvironmentError> {
        let mut records = BTreeMap::<DeclarationKind, BTreeMap<Arc<str>, DeclarationRecord>>::new();
        for declaration in declarations {
            let id =
                DeclarationId::new(declaration.identity().kind(), declaration.identity().name());
            let provenance = declaration.provenance().path().to_owned();
            let records_by_name = records.entry(id.kind()).or_default();
            if let Some(first) = records_by_name.get(id.name()) {
                return Err(ParserEnvironmentError::DuplicateIdentity {
                    identity: id,
                    first_path: first.provenance.clone(),
                    duplicate_path: provenance,
                });
            }

            let (recipe, surfaces) = match declaration.grammar() {
                Some(grammar) => (
                    Some(grammar.recipe().clone()),
                    collect_surfaces(
                        &id,
                        grammar
                            .surfaces()
                            .iter()
                            .map(|surface| (surface.feature(), surface.text())),
                    )?,
                ),
                None => (None, Vec::new()),
            };
            records_by_name.insert(
                Arc::from(id.name()),
                DeclarationRecord {
                    id,
                    recipe,
                    surfaces,
                    provenance,
                },
            );
        }

        let mut readings =
            BTreeMap::<GrammarPosition, BTreeMap<Arc<str>, Vec<DeclarationReading>>>::new();
        let mut initial_readings =
            BTreeMap::<GrammarPosition, BTreeMap<Arc<str>, Vec<DeclarationReading>>>::new();
        for records_by_name in records.values() {
            for record in records_by_name.values() {
                let Some(recipe) = &record.recipe else {
                    continue;
                };
                let position = recipe.position();
                for (feature, surface) in &record.surfaces {
                    let reading = DeclarationReading {
                        id: record.id.clone(),
                        position,
                        feature: *feature,
                        surface: Arc::clone(surface),
                    };
                    readings
                        .entry(position)
                        .or_default()
                        .entry(Arc::clone(surface))
                        .or_default()
                        .push(reading.clone());
                    initial_readings
                        .entry(position)
                        .or_default()
                        .entry(Arc::from(initial_surface(surface)))
                        .or_default()
                        .push(reading);
                }
            }
        }

        Ok(Self {
            data: Arc::new(EnvironmentData {
                declarations: records,
                readings,
                initial_readings,
            }),
            catalog_compatibility: None,
        })
    }

    /// Finds one declaration by category-safe identity.
    #[must_use]
    pub fn declaration(&self, kind: DeclarationKind, name: &str) -> Option<&DeclarationRecord> {
        self.data
            .declarations
            .get(&kind)
            .and_then(|records| records.get(name))
    }

    /// Returns every exact reading for a surface in deterministic identity
    /// order.
    #[must_use]
    pub fn readings(&self, position: GrammarPosition, surface: &str) -> &[DeclarationReading] {
        self.data
            .readings
            .get(&position)
            .and_then(|by_surface| by_surface.get(surface))
            .map_or(&[], Vec::as_slice)
    }

    pub(crate) fn initial_readings(
        &self,
        position: GrammarPosition,
        surface: &str,
    ) -> &[DeclarationReading] {
        self.data
            .initial_readings
            .get(&position)
            .and_then(|by_surface| by_surface.get(surface))
            .map_or(&[], Vec::as_slice)
    }

    /// Returns the exact surface for a declaration and realized feature.
    #[must_use]
    pub fn surface(&self, id: &DeclarationId, feature: SurfaceFeature) -> Option<&str> {
        self.data
            .declarations
            .get(&id.kind())
            .and_then(|records| records.get(id.name()))
            .and_then(|record| record.surface(feature))
    }

    pub(crate) fn with_catalog_compatibility(
        mut self,
        compatibility: CatalogCompatibility,
    ) -> Self {
        self.catalog_compatibility = Some(compatibility);
        self
    }

    pub(crate) fn catalog_spellings(
        &self,
        kind: deckmaste_catalogs::CatalogKind,
    ) -> impl Iterator<Item = &str> {
        self.catalog_compatibility
            .iter()
            .flat_map(move |compatibility| compatibility.spellings(kind))
    }

    pub(crate) fn contains_catalog_spelling(
        &self,
        kind: deckmaste_catalogs::CatalogKind,
        spelling: &str,
    ) -> bool {
        self.catalog_compatibility
            .as_ref()
            .is_some_and(|compatibility| compatibility.contains(kind, spelling))
    }

    #[cfg(test)]
    pub(crate) fn test_only_shares_storage_with(&self, other: &Self) -> bool {
        Arc::ptr_eq(&self.data, &other.data)
            && match (&self.catalog_compatibility, &other.catalog_compatibility) {
                (Some(left), Some(right)) => left.shares_storage_with(right),
                (None, None) => true,
                (Some(_), None) | (None, Some(_)) => false,
            }
    }
}

fn collect_surfaces<'a>(
    identity: &DeclarationId,
    surfaces: impl IntoIterator<Item = (SurfaceFeature, &'a str)>,
) -> Result<Vec<(SurfaceFeature, Arc<str>)>, ParserEnvironmentError> {
    let mut collected = Vec::new();
    for (feature, surface) in surfaces {
        if collected.iter().any(|(candidate, _)| *candidate == feature) {
            return Err(ParserEnvironmentError::DuplicateSurfaceFeature {
                identity: identity.clone(),
                feature,
            });
        }
        collected.push((feature, Arc::from(surface)));
    }
    Ok(collected)
}

#[cfg(test)]
mod tests {
    use std::fs;

    use super::*;

    #[test]
    fn parser_environment_rejects_duplicate_realized_features() {
        let identity = DeclarationId::new(DeclarationKind::KeywordAction, "Scry");
        let error = collect_surfaces(
            &identity,
            [
                (SurfaceFeature::Bare, "scry"),
                (SurfaceFeature::Bare, "scry again"),
            ],
        )
        .expect_err("a repeated realized feature fails closed");
        assert_eq!(
            error,
            ParserEnvironmentError::DuplicateSurfaceFeature {
                identity,
                feature: SurfaceFeature::Bare,
            }
        );
    }

    #[test]
    fn legacy_catalogs_are_fenced_to_the_named_provider_compatibility_boundary() {
        fn rust_sources(path: &Path, found: &mut Vec<PathBuf>) {
            for entry in fs::read_dir(path).expect("source directory is readable") {
                let path = entry.expect("source entry is readable").path();
                if path.is_dir() {
                    rust_sources(&path, found);
                } else if path.extension().is_some_and(|extension| extension == "rs") {
                    found.push(path);
                }
            }
        }

        let source_root = Path::new(env!("CARGO_MANIFEST_DIR")).join("src");
        let mut sources = Vec::new();
        rust_sources(&source_root, &mut sources);
        sources.sort();
        let legacy_type = concat!("Parser", "Catalogs");
        for path in sources {
            let relative = path
                .strip_prefix(&source_root)
                .expect("source is below root");
            let source = fs::read_to_string(&path).expect("Rust source is readable");
            if relative == Path::new("catalogs.rs") {
                assert!(source.contains(&format!("pub struct {legacy_type}")));
            } else {
                assert!(
                    !source.contains(legacy_type),
                    "legacy parser catalogs escaped the provider boundary into {}",
                    relative.display()
                );
            }
        }

        let environment = include_str!("environment.rs");
        let compatibility_field =
            concat!("catalog_compatibility: Option<", "CatalogCompatibility>");
        assert_eq!(
            environment.matches(compatibility_field).count(),
            1,
            "the environment owns exactly one transitional compatibility field"
        );
        assert!(
            include_str!("catalog_compatibility.rs")
                .contains("type and subtype declaration rows supply the noun inventory")
        );
    }
}
