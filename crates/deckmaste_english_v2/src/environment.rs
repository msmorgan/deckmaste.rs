//! Immutable, data-defined grammar vocabulary for the English v2 parser.

use std::collections::BTreeMap;
use std::path::Path;
use std::path::PathBuf;
use std::sync::Arc;

use macro_ron::v2::CustomTailAtom;
pub use macro_ron::v2::DeclarationIdentity as DeclarationId;
use macro_ron::v2::DeclarationKind;
use macro_ron::v2::DeterminativeFusedHeadLicense;
use macro_ron::v2::DeterminativeNominalLicense;
use macro_ron::v2::DeterminativeNumberLicense;
use macro_ron::v2::DeterminativePhraseNumber;
pub use macro_ron::v2::GrammarPosition;
use macro_ron::v2::GrammarRecipe;
use macro_ron::v2::NormalizedDeclaration;
use macro_ron::v2::Onset;
use macro_ron::v2::SurfaceFeature;
use macro_ron::v2::VerbValence;

use crate::constructions::CatalogProvider;
use crate::constructions::VerbFrameAtom;
use crate::constructions::VerbFrameClass;
use crate::constructions::VerbFrameKey;
use crate::orthography::initial_surface;

/// One declaration and its validated grammar metadata.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DeclarationRecord {
    id: DeclarationId,
    recipe: Option<GrammarRecipe>,
    surfaces: Vec<(SurfaceFeature, Onset, Arc<str>)>,
    params: Vec<Arc<str>>,
    determinative: Option<DeterminativeRecord>,
    provenance: PathBuf,
}

/// Stable identity of an ordinary core verb inventory row. This is not a
/// declaration kind: core verbs never masquerade as plugin declarations.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd)]
pub enum CoreVerbIdentity {
    Add,
    Attack,
    Become,
    Block,
    Can,
    Cant,
    Cause,
    Choose,
    Control,
    Copy,
    Cost,
    Cycle,
    Deal,
    Die,
    Didnt,
    Do,
    Draw,
    Enter,
    Flip,
    Gain,
    Get,
    Have,
    Leave,
    Look,
    Lose,
    May,
    Must,
    Own,
    Pay,
    Prevent,
    Put,
    Remove,
    Return,
    Roll,
    Skip,
    Turn,
    Unattach,
    Would,
}

impl CoreVerbIdentity {
    #[must_use]
    pub const fn owner_id(self) -> &'static str {
        match self {
            Self::Add => "core-verb:Add",
            Self::Attack => "core-verb:Attack",
            Self::Become => "core-verb:Become",
            Self::Block => "core-verb:Block",
            Self::Can => "core-verb:Can",
            Self::Cant => "core-verb:Cant",
            Self::Cause => "core-verb:Cause",
            Self::Choose => "core-verb:Choose",
            Self::Control => "core-verb:Control",
            Self::Copy => "core-verb:Copy",
            Self::Cost => "core-verb:Cost",
            Self::Cycle => "core-verb:Cycle",
            Self::Deal => "core-verb:Deal",
            Self::Die => "core-verb:Die",
            Self::Didnt => "core-verb:Didnt",
            Self::Do => "core-verb:Do",
            Self::Draw => "core-verb:Draw",
            Self::Enter => "core-verb:Enter",
            Self::Flip => "core-verb:Flip",
            Self::Gain => "core-verb:Gain",
            Self::Get => "core-verb:Get",
            Self::Have => "core-verb:Have",
            Self::Leave => "core-verb:Leave",
            Self::Look => "core-verb:Look",
            Self::Lose => "core-verb:Lose",
            Self::May => "core-verb:May",
            Self::Must => "core-verb:Must",
            Self::Own => "core-verb:Own",
            Self::Pay => "core-verb:Pay",
            Self::Prevent => "core-verb:Prevent",
            Self::Put => "core-verb:Put",
            Self::Remove => "core-verb:Remove",
            Self::Return => "core-verb:Return",
            Self::Roll => "core-verb:Roll",
            Self::Skip => "core-verb:Skip",
            Self::Turn => "core-verb:Turn",
            Self::Unattach => "core-verb:Unattach",
            Self::Would => "core-verb:Would",
        }
    }
}

/// The normalized source of one grammar verb reading.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Ord, PartialOrd)]
pub enum VerbInventoryRef {
    Core(CoreVerbIdentity),
    Declaration(DeclarationId),
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum VerbProvenance {
    Core(CoreVerbIdentity),
    Declaration(PathBuf),
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct VerbInventoryRecord {
    reference: VerbInventoryRef,
    frames: Vec<OwnedVerbFrameKey>,
    surfaces: Vec<(SurfaceFeature, Onset, Arc<str>)>,
    provenance: VerbProvenance,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct OwnedVerbFrameKey {
    class: VerbFrameClass,
    atoms: Vec<OwnedVerbFrameAtom>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum OwnedVerbFrameAtom {
    Literal(String),
    Amount,
    ObjectNounPhrase,
    PredicativeComplement,
    Role(String),
    OptionalRole(String),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct VerbInventoryReading {
    reference: VerbInventoryRef,
    onset: Onset,
}

impl VerbInventoryReading {
    pub(crate) fn reference(&self) -> &VerbInventoryRef {
        &self.reference
    }

    pub(crate) const fn onset(&self) -> Onset {
        self.onset
    }
}

/// The normalized selection and realization facts for one declaration-backed
/// Determinative lemma.
#[derive(Debug, Clone, PartialEq, Eq)]
struct DeterminativeRecord {
    number_license: DeterminativeNumberLicense,
    nominal_license: DeterminativeNominalLicense,
    fused_head_license: DeterminativeFusedHeadLicense,
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

    /// Returns the declaration's exact positional parameter signature.
    #[must_use]
    pub fn params(&self) -> &[Arc<str>] {
        &self.params
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

    /// Returns whether this declaration-backed Determinative may head a
    /// fused partitive, when declared.
    #[must_use]
    pub fn determinative_fused_head_license(&self) -> Option<DeterminativeFusedHeadLicense> {
        self.determinative
            .as_ref()
            .map(|row| row.fused_head_license)
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
    fused_head_license: DeterminativeFusedHeadLicense,
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
    pub const fn fused_head_license(&self) -> DeterminativeFusedHeadLicense {
        self.fused_head_license
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
    verb_inventory: BTreeMap<VerbInventoryRef, VerbInventoryRecord>,
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
    #[expect(
        clippy::too_many_lines,
        reason = "environment freezing validates and indexes every declaration-backed provider in one transaction"
    )]
    pub fn try_from_parts(
        declarations: impl IntoIterator<Item = NormalizedDeclaration>,
        catalog_providers: impl IntoIterator<Item = CatalogProviderRows>,
    ) -> Result<Self, ParserEnvironmentError> {
        let mut records = BTreeMap::<DeclarationKind, BTreeMap<Arc<str>, DeclarationRecord>>::new();
        for declaration in declarations {
            let id =
                DeclarationId::new(declaration.identity().kind(), declaration.identity().name());
            let provenance = declaration.provenance().path().to_owned();
            let params = declaration
                .params()
                .unwrap_or_default()
                .iter()
                .map(|param| Arc::from(param.as_str()))
                .collect();
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
                        fused_head_license: row.fused_head_license(),
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
                    params,
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
        let mut determinative_readings = BTreeMap::<Arc<str>, Vec<DeterminativeReading>>::new();
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
                            fused_head_license: determinative.fused_head_license,
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
        let verb_inventory = normalized_verb_inventory(&records);

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
                verb_inventory,
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

    /// Returns whether one normalized inventory row licenses a generated frame.
    #[must_use]
    pub(crate) fn verb_frame_licenses(
        &self,
        reference: &VerbInventoryRef,
        frame: VerbFrameKey,
    ) -> bool {
        self.data
            .verb_inventory
            .get(reference)
            .is_some_and(|record| {
                record
                    .frames
                    .iter()
                    .any(|candidate| candidate.matches(frame))
            })
    }

    pub(crate) fn verb_inventory_surface(
        &self,
        reference: &VerbInventoryRef,
        feature: SurfaceFeature,
    ) -> Option<&str> {
        self.data
            .verb_inventory
            .get(reference)?
            .surfaces
            .iter()
            .find_map(|(candidate, _, surface)| (*candidate == feature).then_some(surface.as_ref()))
    }

    pub(crate) fn verb_inventory_onset(
        &self,
        reference: &VerbInventoryRef,
        feature: SurfaceFeature,
    ) -> Option<Onset> {
        self.data
            .verb_inventory
            .get(reference)?
            .surfaces
            .iter()
            .find_map(|(candidate, onset, _)| (*candidate == feature).then_some(*onset))
    }

    pub(crate) fn verb_inventory_owner_id(
        &self,
        reference: &VerbInventoryRef,
    ) -> Option<&'static str> {
        match self.verb_inventory_provenance(reference)? {
            VerbProvenance::Core(identity) => Some(identity.owner_id()),
            VerbProvenance::Declaration(path) => {
                let _ = path;
                None
            }
        }
    }

    fn verb_inventory_provenance(&self, reference: &VerbInventoryRef) -> Option<&VerbProvenance> {
        self.data
            .verb_inventory
            .get(reference)
            .map(|record| &record.provenance)
    }

    pub(crate) fn verb_inventory_readings(
        &self,
        surface: &str,
        feature: SurfaceFeature,
        frame: VerbFrameKey,
    ) -> Vec<VerbInventoryReading> {
        self.verb_inventory_readings_from(surface, feature, frame, false)
    }

    fn verb_inventory_readings_from(
        &self,
        surface: &str,
        feature: SurfaceFeature,
        frame: VerbFrameKey,
        initial: bool,
    ) -> Vec<VerbInventoryReading> {
        self.data
            .verb_inventory
            .values()
            .filter(|record| {
                record
                    .frames
                    .iter()
                    .any(|candidate| candidate.matches(frame))
            })
            .filter_map(|record| {
                record
                    .surfaces
                    .iter()
                    .find(|(candidate, _, text)| {
                        *candidate == feature
                            && if initial {
                                initial_surface(text) == surface
                            } else {
                                text.as_ref() == surface
                            }
                    })
                    .map(|(_, onset, _)| VerbInventoryReading {
                        reference: record.reference.clone(),
                        onset: *onset,
                    })
            })
            .collect()
    }

    pub(crate) fn initial_verb_inventory_readings(
        &self,
        surface: &str,
        feature: SurfaceFeature,
        frame: VerbFrameKey,
    ) -> Vec<VerbInventoryReading> {
        self.verb_inventory_readings_from(surface, feature, frame, true)
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

    pub(crate) fn initial_determinative_readings(&self, surface: &str) -> &[DeterminativeReading] {
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

impl OwnedVerbFrameKey {
    fn from_runtime(class: VerbFrameClass, atoms: &[VerbFrameAtom]) -> Self {
        Self {
            class,
            atoms: atoms
                .iter()
                .copied()
                .map(OwnedVerbFrameAtom::from)
                .collect(),
        }
    }

    fn predicate(atoms: &[VerbFrameAtom]) -> Self {
        Self::from_runtime(VerbFrameClass::Predicate, atoms)
    }

    fn matches(&self, frame: VerbFrameKey) -> bool {
        self.class == frame.class()
            && self.atoms.len() == frame.atoms().len()
            && self
                .atoms
                .iter()
                .zip(frame.atoms())
                .all(|(owned, runtime)| owned.matches(*runtime))
    }
}

impl From<VerbFrameAtom> for OwnedVerbFrameAtom {
    fn from(atom: VerbFrameAtom) -> Self {
        match atom {
            VerbFrameAtom::Literal(value) => Self::Literal(value.to_owned()),
            VerbFrameAtom::Amount => Self::Amount,
            VerbFrameAtom::ObjectNounPhrase => Self::ObjectNounPhrase,
            VerbFrameAtom::PredicativeComplement => Self::PredicativeComplement,
            VerbFrameAtom::Role(value) => Self::Role(value.to_owned()),
            VerbFrameAtom::OptionalRole(value) => Self::OptionalRole(value.to_owned()),
        }
    }
}

impl OwnedVerbFrameAtom {
    fn matches(&self, runtime: VerbFrameAtom) -> bool {
        match (self, runtime) {
            (Self::Literal(owned), VerbFrameAtom::Literal(runtime))
            | (Self::Role(owned), VerbFrameAtom::Role(runtime))
            | (Self::OptionalRole(owned), VerbFrameAtom::OptionalRole(runtime)) => owned == runtime,
            (Self::Amount, VerbFrameAtom::Amount)
            | (Self::ObjectNounPhrase, VerbFrameAtom::ObjectNounPhrase)
            | (Self::PredicativeComplement, VerbFrameAtom::PredicativeComplement) => true,
            _ => false,
        }
    }
}

fn normalize_plugin_valence(valence: &VerbValence) -> Vec<OwnedVerbFrameKey> {
    let shapes = match valence {
        VerbValence::Intransitive => vec![Vec::new()],
        VerbValence::Transitive => vec![vec![CustomTailAtom::ObjectNounPhrase]],
        VerbValence::Numerative => vec![vec![CustomTailAtom::Amount]],
        VerbValence::Custom { shapes } => shapes.clone(),
    };
    let mut frames = Vec::new();
    for shape in shapes {
        let atoms = shape
            .into_iter()
            .map(|atom| match atom {
                CustomTailAtom::Literal(value) => OwnedVerbFrameAtom::Literal(value),
                CustomTailAtom::Amount => OwnedVerbFrameAtom::Amount,
                CustomTailAtom::ObjectNounPhrase => OwnedVerbFrameAtom::ObjectNounPhrase,
                CustomTailAtom::PredicativeComplement => OwnedVerbFrameAtom::PredicativeComplement,
            })
            .collect();
        let frame = OwnedVerbFrameKey {
            class: VerbFrameClass::Predicate,
            atoms,
        };
        if !frames.contains(&frame) {
            frames.push(frame);
        }
    }
    frames
}

fn normalized_verb_inventory(
    declarations: &BTreeMap<DeclarationKind, BTreeMap<Arc<str>, DeclarationRecord>>,
) -> BTreeMap<VerbInventoryRef, VerbInventoryRecord> {
    let mut inventory = core_verb_seed_records()
        .into_iter()
        .map(|record| (record.reference.clone(), record))
        .collect::<BTreeMap<_, _>>();
    for record in declarations.values().flat_map(BTreeMap::values) {
        let Some(valence) = record.valence() else {
            continue;
        };
        let reference = VerbInventoryRef::Declaration(record.id.clone());
        inventory.insert(
            reference.clone(),
            VerbInventoryRecord {
                reference,
                frames: normalize_plugin_valence(valence),
                surfaces: record.surfaces.clone(),
                provenance: VerbProvenance::Declaration(record.provenance.clone()),
            },
        );
    }
    inventory
}

#[expect(
    clippy::too_many_lines,
    reason = "the sealed core verb inventory is intentionally visible as one exhaustive data table"
)]
fn core_verb_seed_records() -> Vec<VerbInventoryRecord> {
    use VerbFrameAtom::OptionalRole;
    use VerbFrameAtom::Role;

    let seed = |bare: &str,
                third_person: &str,
                identity: CoreVerbIdentity,
                frames: Vec<Vec<VerbFrameAtom>>| VerbInventoryRecord {
        reference: VerbInventoryRef::Core(identity),
        surfaces: vec![
            (SurfaceFeature::Bare, Onset::Consonant, Arc::from(bare)),
            (
                SurfaceFeature::ThirdPersonSingular,
                Onset::Consonant,
                Arc::from(third_person),
            ),
        ],
        frames: frames
            .iter()
            .map(|atoms| OwnedVerbFrameKey::predicate(atoms))
            .collect(),
        provenance: VerbProvenance::Core(identity),
    };
    let class_seed =
        |bare: &str, third_person: &str, identity: CoreVerbIdentity, class: VerbFrameClass| {
            let mut record = seed(bare, third_person, identity, Vec::new());
            record.frames = vec![OwnedVerbFrameKey {
                class,
                atoms: Vec::new(),
            }];
            record
        };
    let mut records = vec![
        seed(
            "add",
            "adds",
            CoreVerbIdentity::Add,
            vec![
                vec![Role("ManaPhrase")],
                vec![
                    Role("CardinalQuantity"),
                    VerbFrameAtom::Literal("mana"),
                    VerbFrameAtom::Literal("of"),
                    VerbFrameAtom::Literal("any"),
                    Role("FlexibleManaKind"),
                ],
            ],
        ),
        seed(
            "attack",
            "attacks",
            CoreVerbIdentity::Attack,
            vec![vec![], vec![VerbFrameAtom::ObjectNounPhrase]],
        ),
        seed(
            "block",
            "blocks",
            CoreVerbIdentity::Block,
            vec![vec![], vec![VerbFrameAtom::ObjectNounPhrase]],
        ),
        seed(
            "choose",
            "chooses",
            CoreVerbIdentity::Choose,
            vec![vec![VerbFrameAtom::ObjectNounPhrase]],
        ),
        seed(
            "control",
            "controls",
            CoreVerbIdentity::Control,
            vec![vec![VerbFrameAtom::ObjectNounPhrase]],
        ),
        seed(
            "copy",
            "copies",
            CoreVerbIdentity::Copy,
            vec![vec![VerbFrameAtom::ObjectNounPhrase]],
        ),
        seed("cycle", "cycles", CoreVerbIdentity::Cycle, vec![vec![]]),
        seed(
            "deal",
            "deals",
            CoreVerbIdentity::Deal,
            vec![
                vec![
                    VerbFrameAtom::Amount,
                    VerbFrameAtom::Literal("damage"),
                    Role("ToPhrase"),
                ],
                vec![
                    Role("DistributedDamageAmount"),
                    VerbFrameAtom::Literal("damage"),
                    Role("DamageDistribution"),
                    OptionalRole("DistributionReplacement"),
                ],
                vec![Role("DamageKind"), OptionalRole("ToPhrase")],
                vec![
                    VerbFrameAtom::Literal("damage"),
                    Role("ScalarEquality"),
                    Role("ToPhrase"),
                ],
                vec![
                    VerbFrameAtom::Literal("damage"),
                    Role("ToPhrase"),
                    Role("ScalarEquality"),
                ],
                vec![VerbFrameAtom::Amount],
                vec![
                    VerbFrameAtom::ObjectNounPhrase,
                    VerbFrameAtom::Literal("to"),
                    VerbFrameAtom::ObjectNounPhrase,
                ],
            ],
        ),
        seed("die", "dies", CoreVerbIdentity::Die, vec![vec![]]),
        seed(
            "draw",
            "draws",
            CoreVerbIdentity::Draw,
            vec![
                vec![Role("CardQuantity")],
                vec![VerbFrameAtom::Literal("cards"), Role("ScalarEquality")],
                vec![VerbFrameAtom::ObjectNounPhrase],
                vec![VerbFrameAtom::Amount],
            ],
        ),
        seed(
            "enter",
            "enters",
            CoreVerbIdentity::Enter,
            vec![
                vec![],
                vec![VerbFrameAtom::PredicativeComplement],
                vec![
                    VerbFrameAtom::Literal("with"),
                    Role("CounterQuantity"),
                    Role("OnPhrase"),
                ],
                vec![
                    Role("Object"),
                    OptionalRole("PredicativeComplement"),
                    OptionalRole("ControlPostmodifier"),
                ],
                vec![Role("ControlPostmodifier")],
            ],
        ),
        seed(
            "flip",
            "flips",
            CoreVerbIdentity::Flip,
            vec![vec![VerbFrameAtom::ObjectNounPhrase]],
        ),
        seed(
            "gain",
            "gains",
            CoreVerbIdentity::Gain,
            vec![
                vec![VerbFrameAtom::Amount, VerbFrameAtom::Literal("life")],
                vec![VerbFrameAtom::Literal("life")],
                vec![VerbFrameAtom::Literal("life"), Role("ScalarEquality")],
            ],
        ),
        seed(
            "get",
            "gets",
            CoreVerbIdentity::Get,
            vec![vec![
                Role("PowerToughnessAdjustment"),
                OptionalRole("DurationPhrase"),
            ]],
        ),
        seed(
            "have",
            "has",
            CoreVerbIdentity::Have,
            vec![
                vec![Role("QuotedAbility")],
                vec![
                    VerbFrameAtom::Literal("base"),
                    VerbFrameAtom::Literal("power"),
                    VerbFrameAtom::Literal("and"),
                    VerbFrameAtom::Literal("toughness"),
                    Role("PredicativePowerToughnessComplement"),
                ],
                vec![Role("ScalarComparison"), VerbFrameAtom::Literal("life")],
                vec![Role("Object"), Role("VerbPhrase")],
                vec![VerbFrameAtom::ObjectNounPhrase],
            ],
        ),
        seed(
            "leave",
            "leaves",
            CoreVerbIdentity::Leave,
            vec![vec![], vec![VerbFrameAtom::ObjectNounPhrase]],
        ),
        seed(
            "look",
            "looks",
            CoreVerbIdentity::Look,
            vec![vec![VerbFrameAtom::Literal("at"), Role("Object")]],
        ),
        seed(
            "lose",
            "loses",
            CoreVerbIdentity::Lose,
            vec![
                vec![VerbFrameAtom::Amount, VerbFrameAtom::Literal("life")],
                vec![VerbFrameAtom::Literal("life"), Role("ScalarEquality")],
                vec![VerbFrameAtom::ObjectNounPhrase],
            ],
        ),
        seed(
            "own",
            "owns",
            CoreVerbIdentity::Own,
            vec![vec![VerbFrameAtom::ObjectNounPhrase]],
        ),
        seed(
            "pay",
            "pays",
            CoreVerbIdentity::Pay,
            vec![
                vec![VerbFrameAtom::Amount, VerbFrameAtom::Literal("life")],
                vec![Role("ManaPhrase")],
            ],
        ),
        seed(
            "prevent",
            "prevents",
            CoreVerbIdentity::Prevent,
            vec![vec![VerbFrameAtom::ObjectNounPhrase]],
        ),
        seed(
            "put",
            "puts",
            CoreVerbIdentity::Put,
            vec![
                vec![Role("CounterQuantity"), Role("OnPhrase")],
                vec![
                    Role("Object"),
                    OptionalRole("FromPhrase"),
                    Role("IntoPhrase"),
                ],
                vec![
                    Role("Object"),
                    OptionalRole("FromPhrase"),
                    Role("OntoPhrase"),
                    OptionalRole("PredicativeComplement"),
                    OptionalRole("ControlPostmodifier"),
                ],
                vec![Role("Object"), OptionalRole("FromPhrase"), Role("OnPhrase")],
                vec![Role("Object"), Role("ToPhrase")],
                vec![
                    VerbFrameAtom::ObjectNounPhrase,
                    VerbFrameAtom::Literal("into"),
                    VerbFrameAtom::ObjectNounPhrase,
                ],
                vec![
                    VerbFrameAtom::ObjectNounPhrase,
                    VerbFrameAtom::Literal("on"),
                    VerbFrameAtom::ObjectNounPhrase,
                ],
            ],
        ),
        seed(
            "remove",
            "removes",
            CoreVerbIdentity::Remove,
            vec![vec![Role("CounterQuantity"), Role("FromPhrase")]],
        ),
        seed(
            "return",
            "returns",
            CoreVerbIdentity::Return,
            vec![vec![
                Role("Object"),
                OptionalRole("FromPhrase"),
                Role("ToPhrase"),
                OptionalRole("PredicativeComplement"),
                OptionalRole("ControlPostmodifier"),
            ]],
        ),
        seed(
            "roll",
            "rolls",
            CoreVerbIdentity::Roll,
            vec![vec![Role("DieObject")]],
        ),
        seed(
            "skip",
            "skips",
            CoreVerbIdentity::Skip,
            vec![vec![VerbFrameAtom::ObjectNounPhrase]],
        ),
        seed(
            "turn",
            "turns",
            CoreVerbIdentity::Turn,
            vec![vec![
                VerbFrameAtom::Literal("face"),
                VerbFrameAtom::Literal("up"),
            ]],
        ),
        seed(
            "unattach",
            "unattaches",
            CoreVerbIdentity::Unattach,
            vec![vec![VerbFrameAtom::ObjectNounPhrase]],
        ),
    ];

    records.extend([
        seed(
            "become",
            "becomes",
            CoreVerbIdentity::Become,
            vec![vec![VerbFrameAtom::PredicativeComplement]],
        ),
        seed(
            "cause",
            "causes",
            CoreVerbIdentity::Cause,
            vec![vec![
                Role("Object"),
                VerbFrameAtom::Literal("to"),
                Role("VerbPhrase"),
            ]],
        ),
        seed(
            "cost",
            "costs",
            CoreVerbIdentity::Cost,
            vec![vec![
                Role("ManaAmount"),
                Role("CostComparisonDirection"),
                Role("ControlledCostAction"),
                OptionalRole("ForEachCostBasis"),
            ]],
        ),
        class_seed(
            "may",
            "may",
            CoreVerbIdentity::May,
            VerbFrameClass::Auxiliary,
        ),
        class_seed(
            "can",
            "can",
            CoreVerbIdentity::Can,
            VerbFrameClass::Auxiliary,
        ),
        class_seed(
            "can't",
            "can't",
            CoreVerbIdentity::Cant,
            VerbFrameClass::Auxiliary,
        ),
        class_seed(
            "must",
            "must",
            CoreVerbIdentity::Must,
            VerbFrameClass::Auxiliary,
        ),
        class_seed(
            "didn't",
            "didn't",
            CoreVerbIdentity::Didnt,
            VerbFrameClass::Auxiliary,
        ),
        class_seed(
            "would",
            "would",
            CoreVerbIdentity::Would,
            VerbFrameClass::Auxiliary,
        ),
        class_seed("do", "does", CoreVerbIdentity::Do, VerbFrameClass::ProVerb),
    ]);

    for (identity, atoms) in [
        (CoreVerbIdentity::Choose, vec![Role("InfinitiveComplement")]),
        (
            CoreVerbIdentity::Put,
            vec![
                Role("Object"),
                OptionalRole("FromPhrase"),
                Role("OnPhrase"),
                VerbFrameAtom::Literal("in"),
                Role("ObjectOrder"),
                VerbFrameAtom::Literal("order"),
            ],
        ),
        (CoreVerbIdentity::Have, vec![Role("CounterfactualAbility")]),
        (CoreVerbIdentity::Pay, vec![Role("ManaCostReference")]),
    ] {
        records
            .iter_mut()
            .find(|record| record.reference == VerbInventoryRef::Core(identity))
            .expect("the extended core verb row exists")
            .frames
            .push(OwnedVerbFrameKey::predicate(&atoms));
    }

    for (identity, surface) in [
        (CoreVerbIdentity::Attack, "attacked"),
        (CoreVerbIdentity::Choose, "chosen"),
        (CoreVerbIdentity::Deal, "dealt"),
        (CoreVerbIdentity::Draw, "drawn"),
        (CoreVerbIdentity::Prevent, "prevented"),
        (CoreVerbIdentity::Put, "put"),
        (CoreVerbIdentity::Turn, "turned"),
    ] {
        records
            .iter_mut()
            .find(|record| record.reference == VerbInventoryRef::Core(identity))
            .expect("the participle's core verb row exists")
            .surfaces
            .push((
                SurfaceFeature::Participle,
                Onset::Consonant,
                Arc::from(surface),
            ));
    }

    records
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
    fn core_verb_seed_rows_have_exact_surfaces_and_deduplicated_frame_census() {
        let records = core_verb_seed_records();
        let expected = [
            (CoreVerbIdentity::Add, "add", "adds", None, 2),
            (
                CoreVerbIdentity::Attack,
                "attack",
                "attacks",
                Some("attacked"),
                2,
            ),
            (CoreVerbIdentity::Block, "block", "blocks", None, 2),
            (
                CoreVerbIdentity::Choose,
                "choose",
                "chooses",
                Some("chosen"),
                2,
            ),
            (CoreVerbIdentity::Control, "control", "controls", None, 1),
            (CoreVerbIdentity::Copy, "copy", "copies", None, 1),
            (CoreVerbIdentity::Cycle, "cycle", "cycles", None, 1),
            (CoreVerbIdentity::Deal, "deal", "deals", Some("dealt"), 7),
            (CoreVerbIdentity::Die, "die", "dies", None, 1),
            (CoreVerbIdentity::Draw, "draw", "draws", Some("drawn"), 4),
            (CoreVerbIdentity::Enter, "enter", "enters", None, 5),
            (CoreVerbIdentity::Flip, "flip", "flips", None, 1),
            (CoreVerbIdentity::Gain, "gain", "gains", None, 3),
            (CoreVerbIdentity::Get, "get", "gets", None, 1),
            (CoreVerbIdentity::Have, "have", "has", None, 6),
            (CoreVerbIdentity::Leave, "leave", "leaves", None, 2),
            (CoreVerbIdentity::Look, "look", "looks", None, 1),
            (CoreVerbIdentity::Lose, "lose", "loses", None, 3),
            (CoreVerbIdentity::Own, "own", "owns", None, 1),
            (CoreVerbIdentity::Pay, "pay", "pays", None, 3),
            (
                CoreVerbIdentity::Prevent,
                "prevent",
                "prevents",
                Some("prevented"),
                1,
            ),
            (CoreVerbIdentity::Put, "put", "puts", Some("put"), 8),
            (CoreVerbIdentity::Remove, "remove", "removes", None, 1),
            (CoreVerbIdentity::Return, "return", "returns", None, 1),
            (CoreVerbIdentity::Roll, "roll", "rolls", None, 1),
            (CoreVerbIdentity::Skip, "skip", "skips", None, 1),
            (CoreVerbIdentity::Turn, "turn", "turns", Some("turned"), 1),
            (
                CoreVerbIdentity::Unattach,
                "unattach",
                "unattaches",
                None,
                1,
            ),
            (CoreVerbIdentity::Become, "become", "becomes", None, 1),
            (CoreVerbIdentity::Cause, "cause", "causes", None, 1),
            (CoreVerbIdentity::Cost, "cost", "costs", None, 1),
            (CoreVerbIdentity::May, "may", "may", None, 1),
            (CoreVerbIdentity::Can, "can", "can", None, 1),
            (CoreVerbIdentity::Cant, "can't", "can't", None, 1),
            (CoreVerbIdentity::Must, "must", "must", None, 1),
            (CoreVerbIdentity::Didnt, "didn't", "didn't", None, 1),
            (CoreVerbIdentity::Would, "would", "would", None, 1),
            (CoreVerbIdentity::Do, "do", "does", None, 1),
        ];

        assert_eq!(records.len(), expected.len());
        for (record, (identity, bare, third_person, participle, frame_count)) in
            records.iter().zip(expected)
        {
            assert_eq!(record.reference, VerbInventoryRef::Core(identity));
            assert_eq!(
                record
                    .surfaces
                    .iter()
                    .find_map(|(feature, _, surface)| (*feature == SurfaceFeature::Bare)
                        .then_some(surface.as_ref())),
                Some(bare),
            );
            assert_eq!(
                record.surfaces.iter().find_map(|(feature, _, surface)| {
                    (*feature == SurfaceFeature::ThirdPersonSingular).then_some(surface.as_ref())
                }),
                Some(third_person),
            );
            assert_eq!(
                record.surfaces.iter().find_map(|(feature, _, surface)| {
                    (*feature == SurfaceFeature::Participle).then_some(surface.as_ref())
                }),
                participle,
            );
            assert_eq!(
                record
                    .surfaces
                    .iter()
                    .map(|(feature, _, _)| *feature)
                    .collect::<std::collections::HashSet<_>>()
                    .len(),
                record.surfaces.len(),
                "{identity:?} repeats a realized feature",
            );
            assert_eq!(record.frames.len(), frame_count, "{identity:?}");
            assert_eq!(
                record
                    .frames
                    .iter()
                    .collect::<std::collections::HashSet<_>>()
                    .len(),
                record.frames.len(),
                "{identity:?} repeats a frame",
            );
        }
    }

    #[test]
    fn normalized_verb_inventory_uses_one_frame_lookup_for_core_and_plugin_rows() {
        let declaration = macro_ron::v2::read_str(
            "/synthetic/Act.ron",
            r#"KeywordAction(name:"Act",spelling:"act",grammar:Verb(bare:"act",valence:Transitive))"#,
        )
        .expect("synthetic keyword action is valid");
        let environment = ParserEnvironment::try_from_declarations([declaration])
            .expect("core and plugin verb rows normalize together");
        let plugin = VerbInventoryRef::Declaration(DeclarationId::new(
            DeclarationKind::KeywordAction,
            "Act",
        ));
        let core = VerbInventoryRef::Core(CoreVerbIdentity::Attack);
        let transitive = VerbFrameKey::new(&[VerbFrameAtom::ObjectNounPhrase]);
        let intransitive = VerbFrameKey::new(&[]);
        let auxiliary = VerbFrameKey::with_class(VerbFrameClass::Auxiliary, &[]);

        assert!(environment.verb_frame_licenses(&plugin, transitive));
        assert!(!environment.verb_frame_licenses(&plugin, intransitive));
        assert!(!environment.verb_frame_licenses(&plugin, auxiliary));
        assert!(environment.verb_frame_licenses(&core, transitive));
        assert!(environment.verb_frame_licenses(&core, intransitive));
        assert!(!environment.verb_frame_licenses(&core, auxiliary));
        assert_eq!(
            environment.verb_inventory_surface(&plugin, SurfaceFeature::ThirdPersonSingular),
            Some("acts"),
        );
        assert!(matches!(
            environment
                .data
                .verb_inventory
                .get(&plugin)
                .map(|record| &record.provenance),
            Some(VerbProvenance::Declaration(_)),
        ));
        assert!(matches!(
            environment
                .data
                .verb_inventory
                .get(&core)
                .map(|record| &record.provenance),
            Some(VerbProvenance::Core(CoreVerbIdentity::Attack)),
        ));
    }

    #[test]
    fn special_core_verb_frames_are_exact_and_class_separated() {
        use VerbFrameAtom::Literal;
        use VerbFrameAtom::OptionalRole;
        use VerbFrameAtom::Role;

        let environment = ParserEnvironment::try_from_declarations([])
            .expect("the core verb inventory freezes without plugins");
        let core = |identity| VerbInventoryRef::Core(identity);
        let predicate = |atoms| VerbFrameKey::new(atoms);
        let auxiliary = VerbFrameKey::with_class(VerbFrameClass::Auxiliary, &[]);
        let pro_verb = VerbFrameKey::with_class(VerbFrameClass::ProVerb, &[]);
        let ordinary_empty = VerbFrameKey::new(&[]);

        assert!(environment.verb_frame_licenses(
            &core(CoreVerbIdentity::Become),
            predicate(&[VerbFrameAtom::PredicativeComplement]),
        ));
        assert!(environment.verb_frame_licenses(
            &core(CoreVerbIdentity::Cause),
            predicate(&[Role("Object"), Literal("to"), Role("VerbPhrase")]),
        ));
        assert!(environment.verb_frame_licenses(
            &core(CoreVerbIdentity::Choose),
            predicate(&[Role("InfinitiveComplement")]),
        ));
        assert!(environment.verb_frame_licenses(
            &core(CoreVerbIdentity::Put),
            predicate(&[
                Role("Object"),
                OptionalRole("FromPhrase"),
                Role("OnPhrase"),
                Literal("in"),
                Role("ObjectOrder"),
                Literal("order"),
            ]),
        ));
        assert!(environment.verb_frame_licenses(
            &core(CoreVerbIdentity::Have),
            predicate(&[Role("CounterfactualAbility")]),
        ));
        assert!(environment.verb_frame_licenses(
            &core(CoreVerbIdentity::Pay),
            predicate(&[Role("ManaCostReference")]),
        ));
        assert!(environment.verb_frame_licenses(
            &core(CoreVerbIdentity::Cost),
            predicate(&[
                Role("ManaAmount"),
                Role("CostComparisonDirection"),
                Role("ControlledCostAction"),
                OptionalRole("ForEachCostBasis"),
            ]),
        ));
        for identity in [
            CoreVerbIdentity::May,
            CoreVerbIdentity::Can,
            CoreVerbIdentity::Cant,
            CoreVerbIdentity::Must,
            CoreVerbIdentity::Didnt,
            CoreVerbIdentity::Would,
        ] {
            assert!(environment.verb_frame_licenses(&core(identity), auxiliary));
            assert!(!environment.verb_frame_licenses(&core(identity), ordinary_empty));
        }
        assert!(environment.verb_frame_licenses(&core(CoreVerbIdentity::Do), pro_verb));
        assert!(!environment.verb_frame_licenses(&core(CoreVerbIdentity::Do), ordinary_empty));
        assert_eq!(
            environment.verb_inventory_surface(
                &core(CoreVerbIdentity::Do),
                SurfaceFeature::ThirdPersonSingular,
            ),
            Some("does"),
        );
    }

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
        assert_eq!(
            environment
                .declaration(DeclarationKind::KeywordAbility, "Equip")
                .expect("Equip declaration is retained")
                .determinative_fused_head_license(),
            Some(DeterminativeFusedHeadLicense::NominalOnly),
        );

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
            reading.fused_head_license(),
            DeterminativeFusedHeadLicense::NominalOnly
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
        assert!(
            environment
                .readings(GrammarPosition::FixedKeyword, "equipped")
                .is_empty()
        );
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
            fused_head_license: FusedHead,
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
