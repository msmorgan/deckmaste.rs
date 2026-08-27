//! Immutable, data-defined grammar vocabulary for the English v2 parser.

use std::collections::BTreeMap;
use std::path::Path;
use std::path::PathBuf;
use std::sync::Arc;

use macro_ron::v2::CustomTailAtom;
use macro_ron::v2::DeterminativeNominalLicense;
use macro_ron::v2::DeterminativeNumberLicense;
use macro_ron::v2::DeterminativePhraseNumber;
pub use macro_ron::v2::DeclarationIdentity as DeclarationId;
use macro_ron::v2::DeclarationKind;
pub use macro_ron::v2::GrammarPosition;
use macro_ron::v2::GrammarRecipe;
use macro_ron::v2::NormalizedDeclaration;
use macro_ron::v2::Onset;
use macro_ron::v2::SurfaceFeature;
use macro_ron::v2::VerbValence;

use crate::constructions::CatalogProvider;
use crate::orthography::initial_surface;

/// One declaration and its validated grammar metadata.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DeclarationRecord {
    id: DeclarationId,
    recipe: Option<GrammarRecipe>,
    surfaces: Vec<(SurfaceFeature, Onset, Arc<str>)>,
    determinative: Option<DeterminativeRecord>,
    provenance: PathBuf,
}

/// The normalized selection and realization facts for one declaration-backed
/// Determinative lemma.
#[derive(Debug, Clone, PartialEq, Eq)]
struct DeterminativeRecord {
    number_license: DeterminativeNumberLicense,
    nominal_license: DeterminativeNominalLicense,
    realizations: Vec<DeterminativeRealizationRecord>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct DeterminativeRealizationRecord {
    surface: Arc<str>,
    onset: Onset,
    phrase_number: Option<DeterminativePhraseNumber>,
    following_onset: Option<Onset>,
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

    /// Returns the declaration's Determinative selection facts, when declared.
    #[must_use]
    pub fn determinative_number_license(&self) -> Option<DeterminativeNumberLicense> {
        self.determinative.as_ref().map(|row| row.number_license)
    }

    /// Returns the nominal family selected by this declaration-backed
    /// Determinative, when declared.
    #[must_use]
    pub fn determinative_nominal_license(&self) -> Option<DeterminativeNominalLicense> {
        self.determinative.as_ref().map(|row| row.nominal_license)
    }

    /// Returns the normalized declaration's source path.
    #[must_use]
    pub fn provenance(&self) -> &Path {
        &self.provenance
    }

    fn surface(&self, feature: SurfaceFeature) -> Option<&str> {
        self.surfaces
            .iter()
            .find_map(|(candidate, _, surface)| (*candidate == feature).then_some(surface.as_ref()))
    }

    fn onset(&self, feature: SurfaceFeature) -> Option<Onset> {
        self.surfaces
            .iter()
            .find_map(|(candidate, onset, _)| (*candidate == feature).then_some(*onset))
    }
}

/// One exact surface reading retained by the environment's forward index.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DeclarationReading {
    id: DeclarationId,
    position: GrammarPosition,
    feature: SurfaceFeature,
    surface: Arc<str>,
    onset: Onset,
}

/// One exact surface reading of a declaration-backed Determinative.
///
/// The realization conditions are parser-private facts. Materialized syntax
/// stores only the declaration identity; rendering derives this row again from
/// the following nominal's number and effective onset.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DeterminativeReading {
    id: DeclarationId,
    surface: Arc<str>,
    onset: Onset,
    number_license: DeterminativeNumberLicense,
    nominal_license: DeterminativeNominalLicense,
    phrase_number: Option<DeterminativePhraseNumber>,
    following_onset: Option<Onset>,
}

impl DeterminativeReading {
    #[must_use]
    pub fn id(&self) -> &DeclarationId {
        &self.id
    }

    #[must_use]
    pub fn surface(&self) -> &str {
        &self.surface
    }

    #[must_use]
    pub const fn onset(&self) -> Onset {
        self.onset
    }

    #[must_use]
    pub const fn number_license(&self) -> DeterminativeNumberLicense {
        self.number_license
    }

    #[must_use]
    pub const fn nominal_license(&self) -> DeterminativeNominalLicense {
        self.nominal_license
    }

    #[must_use]
    pub const fn phrase_number(&self) -> Option<DeterminativePhraseNumber> {
        self.phrase_number
    }

    #[must_use]
    pub const fn following_onset(&self) -> Option<Onset> {
        self.following_onset
    }
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

    /// Returns the compiler-frozen phonetic onset for this realized surface.
    #[must_use]
    pub fn onset(&self) -> Onset {
        self.onset
    }
}

/// One immutable identity supplied by a named generated catalog provider.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CatalogProviderRow {
    canonical_identity: Arc<str>,
    canonical_surface: Arc<str>,
    onset: Onset,
}

impl CatalogProviderRow {
    /// Creates one typed provider row from its canonical identity and surface.
    #[must_use]
    pub fn new(
        canonical_identity: impl Into<Arc<str>>,
        canonical_surface: impl Into<Arc<str>>,
        onset: Onset,
    ) -> Self {
        Self {
            canonical_identity: canonical_identity.into(),
            canonical_surface: canonical_surface.into(),
            onset,
        }
    }

    /// Returns the provider's canonical identity key.
    #[must_use]
    pub fn canonical_identity(&self) -> &str {
        &self.canonical_identity
    }

    /// Returns the exact canonical surface emitted by the renderer.
    #[must_use]
    pub fn canonical_surface(&self) -> &str {
        &self.canonical_surface
    }

    /// Returns the adapter-frozen effective onset for the canonical surface.
    #[must_use]
    pub const fn onset(&self) -> Onset {
        self.onset
    }
}

/// All immutable rows supplied for one named generated catalog provider.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CatalogProviderRows {
    provider: CatalogProvider,
    rows: Vec<CatalogProviderRow>,
}

impl CatalogProviderRows {
    /// Creates one named provider group. Uniqueness is checked when freezing
    /// the parser environment.
    #[must_use]
    pub fn new(
        provider: CatalogProvider,
        rows: impl IntoIterator<Item = CatalogProviderRow>,
    ) -> Self {
        Self {
            provider,
            rows: rows.into_iter().collect(),
        }
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

    /// The caller supplied the same named generated provider twice.
    #[error("catalog provider {provider:?} was supplied more than once")]
    DuplicateCatalogProvider { provider: CatalogProvider },

    /// Two rows in one provider claim the same canonical identity.
    #[error("catalog provider {provider:?} repeats canonical identity `{canonical_identity}`")]
    DuplicateCatalogIdentity {
        provider: CatalogProvider,
        canonical_identity: String,
    },

    /// Two rows in one provider claim the same exact canonical surface.
    #[error("catalog provider {provider:?} repeats canonical surface `{canonical_surface}`")]
    DuplicateCatalogSurface {
        provider: CatalogProvider,
        canonical_surface: String,
    },
}

#[derive(Debug, PartialEq, Eq)]
struct CatalogProviderData {
    identities: BTreeMap<Arc<str>, CatalogProviderRow>,
    surfaces: BTreeMap<Arc<str>, Arc<str>>,
    surface_byte_limit: usize,
}

#[derive(Debug, PartialEq, Eq)]
struct EnvironmentData {
    declarations: BTreeMap<DeclarationKind, BTreeMap<Arc<str>, DeclarationRecord>>,
    readings: BTreeMap<GrammarPosition, BTreeMap<Arc<str>, Vec<DeclarationReading>>>,
    initial_readings: BTreeMap<GrammarPosition, BTreeMap<Arc<str>, Vec<DeclarationReading>>>,
    running_surface_byte_limits: BTreeMap<GrammarPosition, usize>,
    initial_surface_byte_limits: BTreeMap<GrammarPosition, usize>,
    determinative_readings: BTreeMap<Arc<str>, Vec<DeterminativeReading>>,
    initial_determinative_readings: BTreeMap<Arc<str>, Vec<DeterminativeReading>>,
    determinative_surface_byte_limit: usize,
    initial_determinative_surface_byte_limit: usize,
    catalog_providers: BTreeMap<CatalogProvider, CatalogProviderData>,
}

#[cfg(test)]
thread_local! {
    static READING_LOOKUP_COUNT: std::cell::Cell<usize> = const { std::cell::Cell::new(0) };
}

/// One immutable vocabulary and grammar-row environment for a parser.
///
/// Clones share the frozen indexes. Construction performs no file discovery,
/// catalog lookup, or process-global registration.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParserEnvironment {
    data: Arc<EnvironmentData>,
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
        Self::try_from_parts(declarations, [])
    }

    /// Freezes normalized declarations and typed generated-provider rows into
    /// deterministic immutable indexes.
    ///
    /// # Errors
    /// Returns a typed error for duplicate declaration identities, provider
    /// groups, provider identities, or provider surfaces.
    pub fn try_from_parts(
        declarations: impl IntoIterator<Item = NormalizedDeclaration>,
        catalog_providers: impl IntoIterator<Item = CatalogProviderRows>,
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

            let (recipe, surfaces, determinative) = match declaration.grammar() {
                Some(grammar) => {
                    let determinative = grammar.determinative().map(|row| DeterminativeRecord {
                        number_license: row.number_license(),
                        nominal_license: row.nominal_license(),
                        realizations: row
                            .realizations()
                            .iter()
                            .map(|realization| DeterminativeRealizationRecord {
                                surface: Arc::from(realization.surface()),
                                onset: realization.onset(),
                                phrase_number: realization.phrase_number(),
                                following_onset: realization.following_onset(),
                            })
                            .collect(),
                    });
                    (
                        Some(grammar.recipe().clone()),
                        collect_surfaces(
                            &id,
                            grammar.surfaces().iter().map(|surface| {
                                (surface.feature(), surface.onset(), surface.text())
                            }),
                        )?,
                        determinative,
                    )
                }
                None => (None, Vec::new(), None),
            };
            records_by_name.insert(
                Arc::from(id.name()),
                DeclarationRecord {
                    id,
                    recipe,
                    surfaces,
                    determinative,
                    provenance,
                },
            );
        }

        let mut readings =
            BTreeMap::<GrammarPosition, BTreeMap<Arc<str>, Vec<DeclarationReading>>>::new();
        let mut initial_readings =
            BTreeMap::<GrammarPosition, BTreeMap<Arc<str>, Vec<DeclarationReading>>>::new();
        let mut running_surface_byte_limits = BTreeMap::<GrammarPosition, usize>::new();
        let mut initial_surface_byte_limits = BTreeMap::<GrammarPosition, usize>::new();
        let mut determinative_readings =
            BTreeMap::<Arc<str>, Vec<DeterminativeReading>>::new();
        let mut initial_determinative_readings =
            BTreeMap::<Arc<str>, Vec<DeterminativeReading>>::new();
        let mut determinative_surface_byte_limit = 0;
        let mut initial_determinative_surface_byte_limit = 0;
        for records_by_name in records.values() {
            for record in records_by_name.values() {
                if let Some(recipe) = &record.recipe {
                    let position = recipe.position();
                    for (feature, onset, surface) in &record.surfaces {
                        let initial = initial_surface(surface);
                        let reading = DeclarationReading {
                            id: record.id.clone(),
                            position,
                            feature: *feature,
                            surface: Arc::clone(surface),
                            onset: *onset,
                        };
                        readings
                            .entry(position)
                            .or_default()
                            .entry(Arc::clone(surface))
                            .or_default()
                            .push(reading.clone());
                        running_surface_byte_limits
                            .entry(position)
                            .and_modify(|limit| *limit = (*limit).max(surface.len()))
                            .or_insert(surface.len());
                        initial_readings
                            .entry(position)
                            .or_default()
                            .entry(Arc::from(initial.as_str()))
                            .or_default()
                            .push(reading);
                        initial_surface_byte_limits
                            .entry(position)
                            .and_modify(|limit| *limit = (*limit).max(initial.len()))
                            .or_insert(initial.len());
                    }
                }
                if let Some(determinative) = &record.determinative {
                    for realization in &determinative.realizations {
                        let initial = initial_surface(&realization.surface);
                        let reading = DeterminativeReading {
                            id: record.id.clone(),
                            surface: Arc::clone(&realization.surface),
                            onset: realization.onset,
                            number_license: determinative.number_license,
                            nominal_license: determinative.nominal_license,
                            phrase_number: realization.phrase_number,
                            following_onset: realization.following_onset,
                        };
                        determinative_readings
                            .entry(Arc::clone(&realization.surface))
                            .or_default()
                            .push(reading.clone());
                        determinative_surface_byte_limit =
                            determinative_surface_byte_limit.max(realization.surface.len());
                        initial_determinative_readings
                            .entry(Arc::from(initial.as_str()))
                            .or_default()
                            .push(reading);
                        initial_determinative_surface_byte_limit =
                            initial_determinative_surface_byte_limit.max(initial.len());
                    }
                }
            }
        }

        let mut frozen_catalog_providers = BTreeMap::new();
        for supplied in catalog_providers {
            let provider = supplied.provider;
            if frozen_catalog_providers.contains_key(&provider) {
                return Err(ParserEnvironmentError::DuplicateCatalogProvider { provider });
            }
            let mut identities = BTreeMap::new();
            let mut surfaces = BTreeMap::new();
            let mut surface_byte_limit = 0;
            for row in supplied.rows {
                if identities.contains_key(row.canonical_identity()) {
                    return Err(ParserEnvironmentError::DuplicateCatalogIdentity {
                        provider,
                        canonical_identity: row.canonical_identity().to_owned(),
                    });
                }
                if surfaces.contains_key(row.canonical_surface()) {
                    return Err(ParserEnvironmentError::DuplicateCatalogSurface {
                        provider,
                        canonical_surface: row.canonical_surface().to_owned(),
                    });
                }
                surfaces.insert(
                    Arc::clone(&row.canonical_surface),
                    Arc::clone(&row.canonical_identity),
                );
                surface_byte_limit = surface_byte_limit.max(row.canonical_surface.len());
                identities.insert(Arc::clone(&row.canonical_identity), row);
            }
            frozen_catalog_providers.insert(
                provider,
                CatalogProviderData {
                    identities,
                    surfaces,
                    surface_byte_limit,
                },
            );
        }

        Ok(Self {
            data: Arc::new(EnvironmentData {
                declarations: records,
                readings,
                initial_readings,
                running_surface_byte_limits,
                initial_surface_byte_limits,
                determinative_readings,
                initial_determinative_readings,
                determinative_surface_byte_limit,
                initial_determinative_surface_byte_limit,
                catalog_providers: frozen_catalog_providers,
            }),
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

    pub(crate) fn grammar_recipe(&self, id: &DeclarationId) -> Option<&GrammarRecipe> {
        self.declaration(id.kind(), id.name())?.recipe()
    }

    /// Returns every exact reading for a surface in deterministic identity
    /// order.
    #[must_use]
    pub fn readings(&self, position: GrammarPosition, surface: &str) -> &[DeclarationReading] {
        record_reading_lookup();
        self.data
            .readings
            .get(&position)
            .and_then(|by_surface| by_surface.get(surface))
            .map_or(&[], Vec::as_slice)
    }

    /// Returns verb readings whose normalized declarations license one exact
    /// grammatical tail.
    #[must_use]
    pub fn declaration_verb_readings(
        &self,
        surface: &str,
        feature: SurfaceFeature,
        frame: &[CustomTailAtom],
    ) -> Vec<&DeclarationReading> {
        self.filter_declaration_verb_readings(
            self.readings(GrammarPosition::Verb, surface),
            feature,
            frame,
        )
    }

    pub(crate) fn initial_declaration_verb_readings(
        &self,
        surface: &str,
        feature: SurfaceFeature,
        frame: &[CustomTailAtom],
    ) -> Vec<&DeclarationReading> {
        self.filter_declaration_verb_readings(
            self.initial_readings(GrammarPosition::Verb, surface),
            feature,
            frame,
        )
    }

    fn filter_declaration_verb_readings<'a>(
        &'a self,
        readings: &'a [DeclarationReading],
        feature: SurfaceFeature,
        frame: &[CustomTailAtom],
    ) -> Vec<&'a DeclarationReading> {
        readings
            .iter()
            .filter(|reading| reading.feature() == feature)
            .filter(|reading| {
                self.declaration(reading.id().kind(), reading.id().name())
                    .and_then(DeclarationRecord::valence)
                    .is_some_and(|valence| valence_licenses_frame(valence, frame))
            })
            .collect()
    }

    pub(crate) fn initial_readings(
        &self,
        position: GrammarPosition,
        surface: &str,
    ) -> &[DeclarationReading] {
        record_reading_lookup();
        self.data
            .initial_readings
            .get(&position)
            .and_then(|by_surface| by_surface.get(surface))
            .map_or(&[], Vec::as_slice)
    }

    /// Returns every declaration-backed Determinative reading for one exact
    /// running-text surface.
    #[must_use]
    pub fn determinative_readings(&self, surface: &str) -> &[DeterminativeReading] {
        self.data
            .determinative_readings
            .get(surface)
            .map_or(&[], Vec::as_slice)
    }

    pub(crate) fn initial_determinative_readings(
        &self,
        surface: &str,
    ) -> &[DeterminativeReading] {
        self.data
            .initial_determinative_readings
            .get(surface)
            .map_or(&[], Vec::as_slice)
    }

    pub(crate) fn determinative_surface_byte_limit(&self) -> usize {
        self.data.determinative_surface_byte_limit
    }

    pub(crate) fn initial_determinative_surface_byte_limit(&self) -> usize {
        self.data.initial_determinative_surface_byte_limit
    }

    /// Derives the unique surface of a stored declaration-backed
    /// Determinative lemma for the following phrase.
    #[must_use]
    pub fn determinative_surface(
        &self,
        id: &DeclarationId,
        phrase_number: DeterminativePhraseNumber,
        following_onset: Option<Onset>,
    ) -> Option<&str> {
        let row = self
            .declaration(id.kind(), id.name())?
            .determinative
            .as_ref()?;
        row.realizations
            .iter()
            .find(|realization| {
                realization
                    .phrase_number
                    .is_none_or(|expected| expected == phrase_number)
                    && match (following_onset, realization.following_onset) {
                        (_, None) => true,
                        (Some(actual), Some(expected)) => expected == actual,
                        (None, Some(_)) => false,
                    }
            })
            .map(|realization| realization.surface.as_ref())
    }

    pub(crate) fn running_surface_byte_limit(&self, position: GrammarPosition) -> usize {
        self.data
            .running_surface_byte_limits
            .get(&position)
            .copied()
            .unwrap_or_default()
    }

    pub(crate) fn initial_surface_byte_limit(&self, position: GrammarPosition) -> usize {
        self.data
            .initial_surface_byte_limits
            .get(&position)
            .copied()
            .unwrap_or_default()
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

    /// Returns the compiler-frozen onset for a declaration's realized feature.
    #[must_use]
    pub fn onset(&self, id: &DeclarationId, feature: SurfaceFeature) -> Option<Onset> {
        self.data
            .declarations
            .get(&id.kind())
            .and_then(|records| records.get(id.name()))
            .and_then(|record| record.onset(feature))
    }

    /// Resolves one canonical identity within a named generated provider.
    #[must_use]
    pub fn catalog_identity(
        &self,
        provider: CatalogProvider,
        canonical_identity: &str,
    ) -> Option<Arc<str>> {
        self.data
            .catalog_providers
            .get(&provider)?
            .identities
            .get(canonical_identity)
            .map(|row| Arc::clone(&row.canonical_identity))
    }

    /// Resolves the canonical surface for one provider identity.
    #[must_use]
    pub fn catalog_surface(
        &self,
        provider: CatalogProvider,
        canonical_identity: &str,
    ) -> Option<&str> {
        self.data
            .catalog_providers
            .get(&provider)?
            .identities
            .get(canonical_identity)
            .map(CatalogProviderRow::canonical_surface)
    }

    /// Resolves the adapter-frozen onset for one provider identity.
    #[must_use]
    pub fn catalog_onset(
        &self,
        provider: CatalogProvider,
        canonical_identity: &str,
    ) -> Option<Onset> {
        self.data
            .catalog_providers
            .get(&provider)?
            .identities
            .get(canonical_identity)
            .map(CatalogProviderRow::onset)
    }

    pub(crate) fn catalog_row_for_surface(
        &self,
        provider: CatalogProvider,
        surface: &str,
    ) -> Option<&CatalogProviderRow> {
        let provider = self.data.catalog_providers.get(&provider)?;
        let canonical_identity = provider.surfaces.get(surface)?;
        provider.identities.get(canonical_identity)
    }

    pub(crate) fn catalog_surface_byte_limit(&self, provider: CatalogProvider) -> usize {
        self.data
            .catalog_providers
            .get(&provider)
            .map_or(0, |provider| provider.surface_byte_limit)
    }

    pub(crate) fn has_catalog_provider(&self, provider: CatalogProvider) -> bool {
        self.data.catalog_providers.contains_key(&provider)
    }

    #[cfg(test)]
    pub(crate) fn test_only_shares_storage_with(&self, other: &Self) -> bool {
        Arc::ptr_eq(&self.data, &other.data)
    }

    #[cfg(test)]
    pub(crate) fn test_only_duplicate_initial_readings(
        &mut self,
        position: GrammarPosition,
        surface: &str,
    ) {
        let data = Arc::get_mut(&mut self.data).expect("test environment storage is unique");
        let readings = data
            .initial_readings
            .get_mut(&position)
            .and_then(|by_surface| by_surface.get_mut(surface))
            .expect("test surface is indexed");
        readings.extend(readings.clone());
    }
}

fn valence_licenses_frame(valence: &VerbValence, frame: &[CustomTailAtom]) -> bool {
    match valence {
        VerbValence::Intransitive => frame.is_empty(),
        VerbValence::Transitive => frame == [CustomTailAtom::ObjectNounPhrase],
        VerbValence::Numerative => frame == [CustomTailAtom::Amount],
        VerbValence::Custom { shapes } => shapes.iter().any(|shape| shape.as_slice() == frame),
    }
}

#[cfg(test)]
fn record_reading_lookup() {
    READING_LOOKUP_COUNT.set(READING_LOOKUP_COUNT.get() + 1);
}

#[cfg(not(test))]
fn record_reading_lookup() {}

#[cfg(test)]
pub(crate) fn reset_reading_lookup_count() {
    READING_LOOKUP_COUNT.set(0);
}

#[cfg(test)]
pub(crate) fn canonical_test_environment() -> ParserEnvironment {
    let declarations = macro_ron::v2::read_builtin_v2(
        Path::new(env!("CARGO_MANIFEST_DIR")).join("../../plugins/builtin_v2"),
    )
    .expect("integrated builtin-v2 declarations load");
    ParserEnvironment::try_from_parts(declarations, [canonical_test_catalog_provider()])
        .expect("builtin-v2 declaration and catalog environment freezes")
}

#[cfg(test)]
pub(crate) fn canonical_test_catalog_provider() -> CatalogProviderRows {
    CatalogProviderRows::new(
        CatalogProvider::CardNames,
        [CatalogProviderRow::new(
            "seven-dwarves",
            "Seven Dwarves",
            Onset::Consonant,
        )],
    )
}

#[cfg(test)]
pub(crate) fn reading_lookup_count() -> usize {
    READING_LOOKUP_COUNT.get()
}

fn collect_surfaces<'a>(
    identity: &DeclarationId,
    surfaces: impl IntoIterator<Item = (SurfaceFeature, Onset, &'a str)>,
) -> Result<Vec<(SurfaceFeature, Onset, Arc<str>)>, ParserEnvironmentError> {
    let mut collected = Vec::new();
    for (feature, onset, surface) in surfaces {
        if collected
            .iter()
            .any(|(candidate, _, _)| *candidate == feature)
        {
            return Err(ParserEnvironmentError::DuplicateSurfaceFeature {
                identity: identity.clone(),
                feature,
            });
        }
        collected.push((feature, onset, Arc::from(surface)));
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
                (SurfaceFeature::Bare, Onset::Consonant, "scry"),
                (SurfaceFeature::Bare, Onset::Consonant, "scry again"),
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
    fn supplemental_determinative_rows_are_indexed_apart_from_keyword_surfaces() {
        let environment = canonical_test_environment();
        let keyword = environment.readings(GrammarPosition::FixedKeyword, "equip");
        assert_eq!(keyword.len(), 1);
        assert_eq!(keyword[0].feature(), SurfaceFeature::Fixed);

        let [reading] = environment.determinative_readings("equipped") else {
            panic!("Equip contributes one supplemental Determinative reading")
        };
        assert_eq!(reading.id().kind(), DeclarationKind::KeywordAbility);
        assert_eq!(reading.id().name(), "Equip");
        assert_eq!(
            reading.number_license(),
            DeterminativeNumberLicense::SingularOnly
        );
        assert_eq!(
            reading.nominal_license(),
            DeterminativeNominalLicense::BareSingularNoun
        );
        assert_eq!(
            environment.determinative_surface(
                reading.id(),
                DeterminativePhraseNumber::Singular,
                None,
            ),
            Some("equipped")
        );
        assert!(environment.determinative_readings("equip").is_empty());
        assert!(environment
            .readings(GrammarPosition::FixedKeyword, "equipped")
            .is_empty());
    }

    #[test]
    fn determinative_surface_requires_an_onset_for_conditional_realizations() {
        let declaration = macro_ron::v2::read_str(
            "/synthetic/Article.ron",
            r#"
KeywordAbility(
    name: "Article",
    spelling: "a",
    grammar: FixedKeyword(
        surface: "a",
        determinative: (
            number_license: SingularOnly,
            nominal_license: CountNominal,
            realizations: [
                (surface: "a", following_onset: Consonant),
                (surface: "an", following_onset: Vowel),
            ],
        ),
    ),
)
"#,
        )
        .expect("conditional article declaration is valid");
        let environment = ParserEnvironment::try_from_declarations([declaration])
            .expect("conditional article environment freezes");
        let id = DeclarationId::new(DeclarationKind::KeywordAbility, "Article");

        assert_eq!(
            environment.determinative_surface(&id, DeterminativePhraseNumber::Singular, None,),
            None,
            "an unknown following onset never guesses a conditional surface",
        );
        assert_eq!(
            environment.determinative_surface(
                &id,
                DeterminativePhraseNumber::Singular,
                Some(Onset::Consonant),
            ),
            Some("a"),
        );
        assert_eq!(
            environment.determinative_surface(
                &id,
                DeterminativePhraseNumber::Singular,
                Some(Onset::Vowel),
            ),
            Some("an"),
        );
    }

    #[test]
    fn production_sources_have_no_legacy_catalog_authority() {
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
        let forbidden = [
            concat!("Parser", "Catalogs"),
            concat!("Catalog", "Set"),
            concat!("Catalog", "Kind"),
            concat!("deckmaste_", "catalogs"),
        ];
        for path in sources {
            let relative = path
                .strip_prefix(&source_root)
                .expect("source is below root");
            let source = fs::read_to_string(&path).expect("Rust source is readable");
            for denied in forbidden {
                assert!(
                    !source.contains(denied),
                    "legacy catalog authority {denied:?} remains in {}",
                    relative.display()
                );
            }
        }
    }
}
