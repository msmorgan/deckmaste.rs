//! Immutable, data-defined grammar vocabulary for the English v2 parser.

use std::collections::BTreeMap;
use std::collections::BTreeSet;
use std::path::Path;
use std::path::PathBuf;
use std::sync::Arc;

use deckmaste_construction_core::macro_def::CustomTailAtom;
pub use deckmaste_construction_core::macro_def::DeclarationIdentity as DeclarationId;
use deckmaste_construction_core::macro_def::DeclarationKind;
pub use deckmaste_construction_core::macro_def::GrammarPosition;
use deckmaste_construction_core::macro_def::GrammarRecipe;
use deckmaste_construction_core::macro_def::NormalizedDeclaration;
use deckmaste_construction_core::macro_def::NounClassSemantics;
use deckmaste_construction_core::macro_def::NounLocativeTemporalLicense;
use deckmaste_construction_core::macro_def::NounRelationality;
use deckmaste_construction_core::macro_def::Onset;
use deckmaste_construction_core::macro_def::SurfaceFeature;
use deckmaste_construction_core::macro_def::VerbFrameSet;
use deckmaste_construction_core::macro_def::normalize_surface_onset;
use serde::Deserialize;

use crate::constructions::CatalogProvider;
use crate::constructions::FORM_LITERAL_SURFACES;
use crate::constructions::FormLiteralSurface;
use crate::constructions::HomographLicense;
use crate::constructions::LEXICON_SURFACES;
use crate::constructions::VERB_TAIL_LITERAL_SURFACES;
use crate::constructions::VOCAB_SURFACES;
use crate::constructions::VerbFrameAtom;
use crate::constructions::VerbFrameClass;
use crate::constructions::VerbFrameKey;
use crate::constructions::VerbTailLiteralSurface;
use crate::constructions::VocabSurface;
use crate::orthography::initial_surface;

/// One declaration and its validated grammar metadata.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DeclarationRecord {
    id: DeclarationId,
    recipe: Option<GrammarRecipe>,
    noun_class: Option<NounClassSemantics>,
    surfaces: Vec<(SurfaceFeature, Onset, Arc<str>)>,
    params: Vec<Arc<str>>,
    provenance: PathBuf,
}

/// Stable identity of an ordinary core verb inventory row. This is not a
/// declaration kind: core verbs never masquerade as plugin declarations.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Deserialize, Ord, PartialOrd)]
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
    Share,
    Skip,
    Turn,
    Unattach,
    Unlock,
    Win,
    Would,
}

impl CoreVerbIdentity {
    const ALL: [Self; 41] = [
        Self::Add,
        Self::Attack,
        Self::Become,
        Self::Block,
        Self::Can,
        Self::Cant,
        Self::Cause,
        Self::Choose,
        Self::Control,
        Self::Copy,
        Self::Cost,
        Self::Cycle,
        Self::Deal,
        Self::Die,
        Self::Didnt,
        Self::Do,
        Self::Draw,
        Self::Enter,
        Self::Flip,
        Self::Gain,
        Self::Get,
        Self::Have,
        Self::Leave,
        Self::Look,
        Self::Lose,
        Self::May,
        Self::Must,
        Self::Own,
        Self::Pay,
        Self::Prevent,
        Self::Put,
        Self::Remove,
        Self::Return,
        Self::Roll,
        Self::Share,
        Self::Skip,
        Self::Turn,
        Self::Unattach,
        Self::Unlock,
        Self::Win,
        Self::Would,
    ];
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
            Self::Share => "core-verb:Share",
            Self::Skip => "core-verb:Skip",
            Self::Turn => "core-verb:Turn",
            Self::Unattach => "core-verb:Unattach",
            Self::Unlock => "core-verb:Unlock",
            Self::Win => "core-verb:Win",
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

#[derive(Debug, Deserialize)]
struct CoreVerbDeclaration {
    identity: CoreVerbIdentity,
    bare: String,
    third_person: String,
    #[serde(default)]
    preterite: Option<String>,
    #[serde(default)]
    participle: Option<String>,
    frames: Vec<CoreVerbFrame>,
}

#[derive(Debug, Deserialize)]
enum CoreVerbFrame {
    Predicate(Vec<CoreVerbTailAtom>),
    Auxiliary,
    ProVerb,
}

#[derive(Debug, Deserialize)]
enum CoreVerbTailAtom {
    Literal(String),
    Lex(String, String),
    OptionalLex(String, String),
    MarkedRole(String, String, String),
    OptionalMarkedRole(String, String, String),
    Amount,
    ObjectNounPhrase,
    PredicativeComplement,
    Role(String),
    OptionalRole(String),
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct IndexedVerbInventoryReading {
    feature: SurfaceFeature,
    frames: Vec<OwnedVerbFrameKey>,
    reading: VerbInventoryReading,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct OwnedVerbFrameKey {
    class: VerbFrameClass,
    atoms: Vec<OwnedVerbFrameAtom>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum OwnedVerbFrameAtom {
    Literal(String),
    Lex(String, String),
    OptionalLex(String, String),
    MarkedRole(String, String, String),
    OptionalMarkedRole(String, String, String),
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
    frame_complement_pair_preposition: Option<(String, String)>,
}

impl VerbInventoryReading {
    pub(crate) fn reference(&self) -> &VerbInventoryRef {
        &self.reference
    }

    pub(crate) const fn onset(&self) -> Onset {
        self.onset
    }

    pub(crate) fn frame_complement_pair_preposition(&self) -> Option<(&str, &str)> {
        self.frame_complement_pair_preposition
            .as_ref()
            .map(|(terminal, member)| (terminal.as_str(), member.as_str()))
    }
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

    /// Returns the Verb Frame Set when this is a verb declaration.
    #[must_use]
    pub fn frame_set(&self) -> Option<&VerbFrameSet> {
        match self.recipe.as_ref() {
            Some(GrammarRecipe::Verb { frame_set }) => Some(frame_set),
            Some(
                GrammarRecipe::Noun
                | GrammarRecipe::FixedTerm
                | GrammarRecipe::FixedClause
                | GrammarRecipe::FixedKeyword { .. },
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
    /// The embedded ordinary-verb declarations are invalid.
    #[error("invalid core verb declarations: {detail}")]
    InvalidCoreVerbDeclarations { detail: String },

    /// A fixed grammar literal shadows an available noun or verb surface.
    #[error("literal surface `{surface}` from {literal_owner} collides with {lexical_owner}")]
    LiteralLexiconCollision {
        surface: String,
        literal_owner: String,
        lexical_owner: String,
    },

    /// A form literal declares a homograph licence with no vocabulary
    /// surface to govern.
    #[error("homograph licence on {literal_owner} governs no vocabulary surface `{surface}`")]
    UngovernedHomographLicense {
        surface: String,
        literal_owner: String,
    },

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
    bound_suffix_surfaces: BTreeSet<Arc<str>>,
    catalog_providers: BTreeMap<CatalogProvider, CatalogProviderData>,
    verb_inventory: BTreeMap<VerbInventoryRef, VerbInventoryRecord>,
    verb_inventory_readings: BTreeMap<Arc<str>, Vec<IndexedVerbInventoryReading>>,
    initial_verb_inventory_readings: BTreeMap<Arc<str>, Vec<IndexedVerbInventoryReading>>,
    licensed_vocab_lexicon_homographs: Vec<String>,
    form_literal_vocab_overlaps: usize,
}

fn record_bound_suffix_surface(
    surfaces: &mut BTreeSet<Arc<str>>,
    feature: SurfaceFeature,
    surface: &Arc<str>,
) {
    if feature == SurfaceFeature::BoundSuffix {
        surfaces.insert(Arc::clone(surface));
    }
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

            let (recipe, surfaces) = match declaration.grammar() {
                Some(grammar) => (
                    Some(grammar.recipe().clone()),
                    collect_surfaces(
                        &id,
                        grammar
                            .surfaces()
                            .iter()
                            .chain(grammar.participial_adjective())
                            .chain(grammar.block_label())
                            .map(|surface| (surface.feature(), surface.onset(), surface.text())),
                    )?,
                ),
                None => (None, Vec::new()),
            };
            records_by_name.insert(
                Arc::from(id.name()),
                DeclarationRecord {
                    id,
                    recipe,
                    noun_class: declaration.noun_class(),
                    surfaces,
                    params,
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
        let mut bound_suffix_surfaces = BTreeSet::<Arc<str>>::new();
        for records_by_name in records.values() {
            for record in records_by_name.values() {
                if let Some(recipe) = &record.recipe {
                    let position = recipe.position();
                    for (feature, onset, surface) in &record.surfaces {
                        record_bound_suffix_surface(&mut bound_suffix_surfaces, *feature, surface);
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
        let verb_inventory = normalized_verb_inventory(&records)?;
        let (licensed_vocab_lexicon_homographs, form_literal_vocab_overlaps) =
            reject_literal_lexicon_collisions(&records, &verb_inventory)?;
        let (verb_inventory_readings, initial_verb_inventory_readings) =
            index_verb_inventory_readings(&verb_inventory);

        Ok(Self {
            data: Arc::new(EnvironmentData {
                declarations: records,
                readings,
                initial_readings,
                running_surface_byte_limits,
                initial_surface_byte_limits,
                bound_suffix_surfaces,
                catalog_providers: frozen_catalog_providers,
                verb_inventory,
                verb_inventory_readings,
                initial_verb_inventory_readings,
                licensed_vocab_lexicon_homographs,
                form_literal_vocab_overlaps,
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

    pub(crate) fn declaration_noun_features(
        &self,
        id: &DeclarationId,
    ) -> Option<(NounLocativeTemporalLicense, NounRelationality, bool)> {
        let declaration = self.declaration(id.kind(), id.name())?;
        let semantics = declaration.noun_class?;
        let number_invariant = declaration.surface(SurfaceFeature::Singular)
            == declaration.surface(SurfaceFeature::Plural);
        Some((
            semantics.locative_temporal_license,
            semantics.relationality,
            number_invariant,
        ))
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
                    .any(|candidate| candidate.matches(frame).is_some())
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
        let index = if initial {
            &self.data.initial_verb_inventory_readings
        } else {
            &self.data.verb_inventory_readings
        };
        index
            .get(surface)
            .into_iter()
            .flatten()
            .filter(|indexed| indexed.feature == feature)
            .flat_map(|indexed| {
                indexed.frames.iter().filter_map(move |candidate| {
                    let matched = candidate.matches(frame)?;
                    let mut reading = indexed.reading.clone();
                    reading.frame_complement_pair_preposition = matched.pair_preposition;
                    Some(reading)
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
                    .and_then(DeclarationRecord::frame_set)
                    .is_some_and(|frame_set| frame_set_licenses_frame(frame_set, frame))
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

    pub(crate) fn bound_suffix_starts_at(&self, text: &str) -> bool {
        self.data
            .bound_suffix_surfaces
            .iter()
            .any(|surface| text.starts_with(surface.as_ref()))
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

    /// Counts licensed vocabulary/lexicon homographs observed by the
    /// environment checker.
    #[must_use]
    pub fn licensed_vocab_lexicon_homographs(&self) -> usize {
        self.data.licensed_vocab_lexicon_homographs.len()
    }

    /// Names each licensed vocabulary/lexicon homograph the environment
    /// checker admitted, so a changed census reports which rows it saw.
    #[must_use]
    pub fn licensed_vocab_lexicon_homograph_owners(&self) -> &[String] {
        &self.data.licensed_vocab_lexicon_homographs
    }

    /// Counts unlicensed form-literal/vocabulary overlaps observed by the
    /// environment checker.
    #[must_use]
    pub fn form_literal_vocab_overlaps(&self) -> usize {
        self.data.form_literal_vocab_overlaps
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

fn reject_literal_lexicon_collisions(
    records: &BTreeMap<DeclarationKind, BTreeMap<Arc<str>, DeclarationRecord>>,
    verb_inventory: &BTreeMap<VerbInventoryRef, VerbInventoryRecord>,
) -> Result<(Vec<String>, usize), ParserEnvironmentError> {
    reject_literal_lexicon_collisions_from_surfaces(
        records,
        verb_inventory,
        FORM_LITERAL_SURFACES,
        VOCAB_SURFACES,
        VERB_TAIL_LITERAL_SURFACES,
    )
}

fn reject_literal_lexicon_collisions_from_surfaces(
    records: &BTreeMap<DeclarationKind, BTreeMap<Arc<str>, DeclarationRecord>>,
    verb_inventory: &BTreeMap<VerbInventoryRef, VerbInventoryRecord>,
    form_literal_surfaces: &[FormLiteralSurface],
    vocab_surfaces: &[VocabSurface],
    verb_tail_literal_surfaces: &[VerbTailLiteralSurface],
) -> Result<(Vec<String>, usize), ParserEnvironmentError> {
    let mut lexical_owners = BTreeMap::<&str, String>::new();

    for row in LEXICON_SURFACES {
        lexical_owners.entry(row.surface).or_insert_with(|| {
            format!("{:?} lexeme {}::{}", row.position, row.terminal, row.member)
        });
    }
    for records_by_name in records.values() {
        for record in records_by_name.values() {
            let Some(recipe) = record.recipe() else {
                continue;
            };
            let position = recipe.position();
            if !matches!(position, GrammarPosition::Noun | GrammarPosition::Verb) {
                continue;
            }
            for (_, _, surface) in &record.surfaces {
                lexical_owners.entry(surface).or_insert_with(|| {
                    format!(
                        "{position:?} declaration {} from `{}`",
                        record.id(),
                        record.provenance().display()
                    )
                });
            }
        }
    }
    for record in verb_inventory.values() {
        for (_, _, surface) in &record.surfaces {
            lexical_owners
                .entry(surface)
                .or_insert_with(|| match &record.provenance {
                    VerbProvenance::Core(identity) => {
                        format!("Verb inventory {}", identity.owner_id())
                    }
                    VerbProvenance::Declaration(path) => format!(
                        "Verb declaration {:?} from `{}`",
                        record.reference,
                        path.display()
                    ),
                });
        }
    }

    let mut collisions = Vec::new();
    let mut licensed_vocab_lexicon_homographs = Vec::new();
    let mut form_literal_vocab_overlaps = 0usize;

    for row in vocab_surfaces {
        if let Some(lexical_owner) = lexical_owners.get(row.surface) {
            let error = ParserEnvironmentError::LiteralLexiconCollision {
                surface: row.surface.to_owned(),
                literal_owner: format!("vocab `{}::{}`", row.vocabulary, row.member),
                lexical_owner: lexical_owner.clone(),
            };
            match row.homograph_license {
                HomographLicense::Licensed => licensed_vocab_lexicon_homographs.push(format!(
                    "vocab `{}::{}` beside {lexical_owner}",
                    row.vocabulary, row.member
                )),
                HomographLicense::Unlicensed => collisions.push(error),
            }
        }
    }
    let mut vocab_owners = BTreeMap::<&str, String>::new();
    for row in vocab_surfaces {
        vocab_owners
            .entry(row.surface)
            .or_insert_with(|| format!("vocab `{}::{}`", row.vocabulary, row.member));
    }

    let collision = |surface: &str, literal_owner: String| {
        lexical_owners.get(surface).map(|lexical_owner| {
            ParserEnvironmentError::LiteralLexiconCollision {
                surface: surface.to_owned(),
                literal_owner,
                lexical_owner: lexical_owner.clone(),
            }
        })
    };

    for row in form_literal_surfaces {
        if let Some(error) = collision(
            row.surface,
            format!(
                "construction `{}` form `{}` atom {}",
                row.construction, row.form, row.atom_index
            ),
        ) {
            collisions.push(error);
        } else {
            match (
                vocab_owners.contains_key(row.surface),
                row.homograph_license,
            ) {
                (true, HomographLicense::Unlicensed) => form_literal_vocab_overlaps += 1,
                (false, HomographLicense::Licensed) => {
                    collisions.push(ParserEnvironmentError::UngovernedHomographLicense {
                        surface: row.surface.to_owned(),
                        literal_owner: format!(
                            "construction `{}` form `{}` atom {}",
                            row.construction, row.form, row.atom_index
                        ),
                    });
                }
                (true, HomographLicense::Licensed) | (false, HomographLicense::Unlicensed) => {}
            }
        }
    }
    for row in verb_tail_literal_surfaces {
        if let Some(error) = collision(
            row.surface,
            format!(
                "declaration-verb codec `{}` tail atom {}",
                row.codec, row.atom_index
            ),
        ) {
            collisions.push(error);
        }
    }
    for record in verb_inventory.values() {
        for (frame_index, frame) in record.frames.iter().enumerate() {
            for (atom_index, atom) in frame.atoms.iter().enumerate() {
                let OwnedVerbFrameAtom::Literal(surface) = atom else {
                    continue;
                };
                if let Some(error) = collision(
                    surface,
                    format!(
                        "verb inventory {:?} frame {frame_index} tail atom {atom_index}",
                        record.reference
                    ),
                ) {
                    collisions.push(error);
                }
            }
        }
    }
    collisions.into_iter().next().map_or(
        Ok((
            licensed_vocab_lexicon_homographs,
            form_literal_vocab_overlaps,
        )),
        Err,
    )
}

fn frame_set_licenses_frame(frame_set: &VerbFrameSet, candidate: &[CustomTailAtom]) -> bool {
    match frame_set {
        VerbFrameSet::Intransitive => candidate.is_empty(),
        VerbFrameSet::Transitive => candidate == [CustomTailAtom::ObjectNounPhrase],
        VerbFrameSet::MeasureComplement => candidate == [CustomTailAtom::Amount],
        VerbFrameSet::Custom { frames } => frames.iter().any(|frame| frame.as_slice() == candidate),
    }
}

impl OwnedVerbFrameKey {
    fn matches(&self, frame: VerbFrameKey) -> Option<OwnedVerbFrameMatch> {
        if self.class != frame.class() {
            return None;
        }
        if frame.atoms() == [VerbFrameAtom::FrameComplementPair] {
            return self
                .frame_complement_pair_preposition()
                .map(|pair_preposition| OwnedVerbFrameMatch {
                    pair_preposition: Some(pair_preposition),
                });
        }
        (self.atoms.len() == frame.atoms().len()
            && self
                .atoms
                .iter()
                .zip(frame.atoms())
                .all(|(owned, runtime)| owned.matches(*runtime)))
        .then_some(OwnedVerbFrameMatch {
            pair_preposition: None,
        })
    }

    fn frame_complement_pair_preposition(&self) -> Option<(String, String)> {
        let [object, rest @ ..] = self.atoms.as_slice() else {
            return None;
        };
        if !matches!(object, OwnedVerbFrameAtom::ObjectNounPhrase)
            && !matches!(object, OwnedVerbFrameAtom::Role(role) if role == "Object")
        {
            return None;
        }
        let marker_index = rest
            .iter()
            .position(|atom| !matches!(atom, OwnedVerbFrameAtom::OptionalLex(_, _)))?;
        let marker = &rest[marker_index];
        let following = &rest[marker_index + 1..];
        let (terminal, member, trailing) = match (marker, following) {
            (OwnedVerbFrameAtom::Lex(terminal, member), [complement, trailing @ ..])
                if complement.is_required_complement() =>
            {
                (terminal, member, trailing)
            }
            (OwnedVerbFrameAtom::MarkedRole(terminal, member, _), trailing) => {
                (terminal, member, trailing)
            }
            _ => return None,
        };
        trailing
            .iter()
            .all(OwnedVerbFrameAtom::is_optional)
            .then(|| (terminal.clone(), member.clone()))
    }
}

struct OwnedVerbFrameMatch {
    pair_preposition: Option<(String, String)>,
}

impl OwnedVerbFrameAtom {
    fn is_required_complement(&self) -> bool {
        matches!(self, Self::ObjectNounPhrase | Self::Role(_))
    }

    fn is_optional(&self) -> bool {
        matches!(
            self,
            Self::OptionalLex(_, _) | Self::OptionalMarkedRole(_, _, _) | Self::OptionalRole(_)
        )
    }

    fn matches(&self, runtime: VerbFrameAtom) -> bool {
        match (self, runtime) {
            (Self::Literal(owned), VerbFrameAtom::Literal(runtime))
            | (Self::Role(owned), VerbFrameAtom::Role(runtime))
            | (Self::OptionalRole(owned), VerbFrameAtom::OptionalRole(runtime)) => owned == runtime,
            (
                Self::Lex(owned_terminal, owned_variant),
                VerbFrameAtom::Lex(runtime_terminal, runtime_variant),
            )
            | (
                Self::OptionalLex(owned_terminal, owned_variant),
                VerbFrameAtom::OptionalLex(runtime_terminal, runtime_variant),
            ) => owned_terminal == runtime_terminal && owned_variant == runtime_variant,
            (
                Self::MarkedRole(owned_terminal, owned_variant, owned_role),
                VerbFrameAtom::MarkedRole(runtime_terminal, runtime_variant, runtime_role),
            )
            | (
                Self::OptionalMarkedRole(owned_terminal, owned_variant, owned_role),
                VerbFrameAtom::OptionalMarkedRole(runtime_terminal, runtime_variant, runtime_role),
            ) => {
                owned_terminal == runtime_terminal
                    && owned_variant == runtime_variant
                    && owned_role == runtime_role
            }
            (Self::Amount, VerbFrameAtom::Amount)
            | (Self::ObjectNounPhrase, VerbFrameAtom::ObjectNounPhrase)
            | (Self::PredicativeComplement, VerbFrameAtom::PredicativeComplement) => true,
            _ => false,
        }
    }
}

fn normalize_plugin_frame_set(frame_set: &VerbFrameSet) -> Vec<OwnedVerbFrameKey> {
    let declared_frames = match frame_set {
        VerbFrameSet::Intransitive => vec![Vec::new()],
        VerbFrameSet::Transitive => vec![vec![CustomTailAtom::ObjectNounPhrase]],
        VerbFrameSet::MeasureComplement => vec![vec![CustomTailAtom::Amount]],
        VerbFrameSet::Custom { frames } => frames.clone(),
    };
    let mut frames = Vec::new();
    for frame in declared_frames {
        let atoms = frame
            .into_iter()
            .map(|atom| match atom {
                CustomTailAtom::Literal(value) => OwnedVerbFrameAtom::Literal(value),
                CustomTailAtom::Lex(terminal, variant) => {
                    OwnedVerbFrameAtom::Lex(terminal, variant)
                }
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
) -> Result<BTreeMap<VerbInventoryRef, VerbInventoryRecord>, ParserEnvironmentError> {
    let mut inventory = core_verb_declaration_records()?
        .into_iter()
        .map(|record| (record.reference.clone(), record))
        .collect::<BTreeMap<_, _>>();
    for record in declarations.values().flat_map(BTreeMap::values) {
        let Some(frame_set) = record.frame_set() else {
            continue;
        };
        let reference = VerbInventoryRef::Declaration(record.id.clone());
        inventory.insert(
            reference.clone(),
            VerbInventoryRecord {
                reference,
                frames: normalize_plugin_frame_set(frame_set),
                surfaces: record.surfaces.clone(),
                provenance: VerbProvenance::Declaration(record.provenance.clone()),
            },
        );
    }
    Ok(inventory)
}

type VerbInventoryReadingIndex = BTreeMap<Arc<str>, Vec<IndexedVerbInventoryReading>>;

fn index_verb_inventory_readings(
    inventory: &BTreeMap<VerbInventoryRef, VerbInventoryRecord>,
) -> (VerbInventoryReadingIndex, VerbInventoryReadingIndex) {
    let mut running = VerbInventoryReadingIndex::new();
    let mut initial = VerbInventoryReadingIndex::new();
    for record in inventory.values() {
        for (feature, onset, surface) in &record.surfaces {
            let indexed = IndexedVerbInventoryReading {
                feature: *feature,
                frames: record.frames.clone(),
                reading: VerbInventoryReading {
                    reference: record.reference.clone(),
                    onset: *onset,
                    frame_complement_pair_preposition: None,
                },
            };
            running
                .entry(Arc::clone(surface))
                .or_default()
                .push(indexed.clone());
            initial
                .entry(Arc::from(initial_surface(surface)))
                .or_default()
                .push(indexed);
        }
    }
    (running, initial)
}

fn owned_core_verb_frame_atoms(atoms: Vec<CoreVerbTailAtom>) -> Vec<OwnedVerbFrameAtom> {
    atoms
        .into_iter()
        .map(|atom| match atom {
            CoreVerbTailAtom::Literal(value) => OwnedVerbFrameAtom::Literal(value),
            CoreVerbTailAtom::Lex(terminal, variant) => OwnedVerbFrameAtom::Lex(terminal, variant),
            CoreVerbTailAtom::OptionalLex(terminal, variant) => {
                OwnedVerbFrameAtom::OptionalLex(terminal, variant)
            }
            CoreVerbTailAtom::MarkedRole(terminal, variant, role) => {
                OwnedVerbFrameAtom::MarkedRole(terminal, variant, role)
            }
            CoreVerbTailAtom::OptionalMarkedRole(terminal, variant, role) => {
                OwnedVerbFrameAtom::OptionalMarkedRole(terminal, variant, role)
            }
            CoreVerbTailAtom::Amount => OwnedVerbFrameAtom::Amount,
            CoreVerbTailAtom::ObjectNounPhrase => OwnedVerbFrameAtom::ObjectNounPhrase,
            CoreVerbTailAtom::PredicativeComplement => OwnedVerbFrameAtom::PredicativeComplement,
            CoreVerbTailAtom::Role(value) => OwnedVerbFrameAtom::Role(value),
            CoreVerbTailAtom::OptionalRole(value) => OwnedVerbFrameAtom::OptionalRole(value),
        })
        .collect()
}

fn core_verb_declaration_records() -> Result<Vec<VerbInventoryRecord>, ParserEnvironmentError> {
    let declarations = ron::from_str::<Vec<CoreVerbDeclaration>>(include_str!("core_verbs.ron"))
        .map_err(
            |error| ParserEnvironmentError::InvalidCoreVerbDeclarations {
                detail: error.to_string(),
            },
        )?;
    let mut seen = BTreeSet::new();
    let mut records = Vec::with_capacity(declarations.len());

    for declaration in declarations {
        let CoreVerbDeclaration {
            identity,
            bare,
            third_person,
            preterite,
            participle,
            frames: declared_frames,
        } = declaration;
        if !seen.insert(identity) {
            return Err(ParserEnvironmentError::InvalidCoreVerbDeclarations {
                detail: format!("duplicate identity {identity:?}"),
            });
        }
        if declared_frames.is_empty() {
            return Err(ParserEnvironmentError::InvalidCoreVerbDeclarations {
                detail: format!("{identity:?} declares no Verb Frame"),
            });
        }

        let surface = |feature, text: String| {
            let onset = normalize_surface_onset(&text, None).ok_or_else(|| {
                ParserEnvironmentError::InvalidCoreVerbDeclarations {
                    detail: format!(
                        "{identity:?} surface \u{60}{text}\u{60} has no normalized onset"
                    ),
                }
            })?;
            Ok((feature, onset, Arc::from(text)))
        };
        let mut surfaces = vec![
            surface(SurfaceFeature::PLAIN, bare)?,
            surface(SurfaceFeature::THIRD_PERSON_SINGULAR_PRESENT, third_person)?,
        ];
        if let Some(preterite) = preterite {
            surfaces.push(surface(SurfaceFeature::PRETERITE, preterite)?);
        }
        if let Some(participle) = participle {
            surfaces.push(surface(SurfaceFeature::PAST_PARTICIPLE, participle)?);
        }

        let frames = declared_frames
            .into_iter()
            .map(|frame| match frame {
                CoreVerbFrame::Predicate(atoms) => OwnedVerbFrameKey {
                    class: VerbFrameClass::Predicate,
                    atoms: owned_core_verb_frame_atoms(atoms),
                },
                CoreVerbFrame::Auxiliary => OwnedVerbFrameKey {
                    class: VerbFrameClass::Auxiliary,
                    atoms: Vec::new(),
                },
                CoreVerbFrame::ProVerb => OwnedVerbFrameKey {
                    class: VerbFrameClass::ProVerb,
                    atoms: Vec::new(),
                },
            })
            .collect::<Vec<_>>();
        if frames
            .iter()
            .enumerate()
            .any(|(index, frame)| frames[..index].contains(frame))
        {
            return Err(ParserEnvironmentError::InvalidCoreVerbDeclarations {
                detail: format!("{identity:?} repeats a Verb Frame"),
            });
        }

        records.push(VerbInventoryRecord {
            reference: VerbInventoryRef::Core(identity),
            frames,
            surfaces,
            provenance: VerbProvenance::Core(identity),
        });
    }

    let expected = CoreVerbIdentity::ALL.into_iter().collect::<BTreeSet<_>>();
    if seen != expected {
        let missing = expected.difference(&seen).copied().collect::<Vec<_>>();
        return Err(ParserEnvironmentError::InvalidCoreVerbDeclarations {
            detail: format!("missing identities: {missing:?}"),
        });
    }

    Ok(records)
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
    let declarations = deckmaste_construction_core::macro_def::read_builtin_v2(
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
    fn frame_complement_pair_pattern_reads_the_declared_marker_structurally() {
        let pattern = VerbFrameKey::new(&[VerbFrameAtom::FrameComplementPair]);
        let frame = OwnedVerbFrameKey {
            class: VerbFrameClass::Predicate,
            atoms: vec![
                OwnedVerbFrameAtom::Role("Object".to_owned()),
                OwnedVerbFrameAtom::OptionalLex("Preposition".to_owned(), "From".to_owned()),
                OwnedVerbFrameAtom::Lex("Preposition".to_owned(), "On".to_owned()),
                OwnedVerbFrameAtom::Role("FrameComplement".to_owned()),
                OwnedVerbFrameAtom::OptionalMarkedRole(
                    "Preposition".to_owned(),
                    "Under".to_owned(),
                    "Object".to_owned(),
                ),
            ],
        };
        assert_eq!(
            frame
                .matches(pattern)
                .and_then(|matched| matched.pair_preposition),
            Some(("Preposition".to_owned(), "On".to_owned())),
        );

        let mut required_trailer = frame;
        required_trailer
            .atoms
            .push(OwnedVerbFrameAtom::Role("ScalarEquality".to_owned()));
        assert!(required_trailer.matches(pattern).is_none());
    }

    #[test]
    fn core_verb_declarations_have_exact_surfaces_and_deduplicated_frames() {
        let records = core_verb_declaration_records().expect("embedded declarations are valid");
        let expected = [
            (CoreVerbIdentity::Add, "add", "adds", None, 2),
            (
                CoreVerbIdentity::Attack,
                "attack",
                "attacks",
                Some("attacked"),
                2,
            ),
            (
                CoreVerbIdentity::Block,
                "block",
                "blocks",
                Some("blocked"),
                2,
            ),
            (
                CoreVerbIdentity::Choose,
                "choose",
                "chooses",
                Some("chosen"),
                3,
            ),
            (CoreVerbIdentity::Control, "control", "controls", None, 1),
            (CoreVerbIdentity::Copy, "copy", "copies", None, 1),
            (CoreVerbIdentity::Cycle, "cycle", "cycles", None, 2),
            (CoreVerbIdentity::Deal, "deal", "deals", Some("dealt"), 6),
            (CoreVerbIdentity::Die, "die", "dies", None, 1),
            (CoreVerbIdentity::Draw, "draw", "draws", Some("drawn"), 3),
            (CoreVerbIdentity::Enter, "enter", "enters", None, 5),
            (CoreVerbIdentity::Flip, "flip", "flips", None, 1),
            (CoreVerbIdentity::Gain, "gain", "gains", None, 3),
            (CoreVerbIdentity::Get, "get", "gets", None, 1),
            (CoreVerbIdentity::Have, "have", "has", None, 4),
            (CoreVerbIdentity::Leave, "leave", "leaves", None, 2),
            (CoreVerbIdentity::Look, "look", "looks", None, 1),
            (CoreVerbIdentity::Lose, "lose", "loses", None, 2),
            (CoreVerbIdentity::Own, "own", "owns", None, 1),
            (CoreVerbIdentity::Pay, "pay", "pays", None, 2),
            (
                CoreVerbIdentity::Prevent,
                "prevent",
                "prevents",
                Some("prevented"),
                1,
            ),
            (CoreVerbIdentity::Put, "put", "puts", Some("put"), 6),
            (CoreVerbIdentity::Remove, "remove", "removes", None, 1),
            (CoreVerbIdentity::Return, "return", "returns", None, 1),
            (CoreVerbIdentity::Roll, "roll", "rolls", None, 1),
            (CoreVerbIdentity::Share, "share", "shares", None, 2),
            (CoreVerbIdentity::Skip, "skip", "skips", None, 1),
            (CoreVerbIdentity::Turn, "turn", "turns", Some("turned"), 1),
            (
                CoreVerbIdentity::Unattach,
                "unattach",
                "unattaches",
                None,
                1,
            ),
            (CoreVerbIdentity::Unlock, "unlock", "unlocks", None, 1),
            (CoreVerbIdentity::Win, "win", "wins", None, 2),
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
        for (identity, bare, third_person, participle, frame_count) in expected {
            let record = records
                .iter()
                .find(|record| record.reference == VerbInventoryRef::Core(identity))
                .expect("every expected core verb is declared");
            assert_eq!(record.reference, VerbInventoryRef::Core(identity));
            assert_eq!(
                record
                    .surfaces
                    .iter()
                    .find_map(|(feature, _, surface)| (*feature == SurfaceFeature::PLAIN)
                        .then_some(surface.as_ref())),
                Some(bare),
            );
            assert_eq!(
                record.surfaces.iter().find_map(|(feature, _, surface)| {
                    (*feature == SurfaceFeature::THIRD_PERSON_SINGULAR_PRESENT)
                        .then_some(surface.as_ref())
                }),
                Some(third_person),
            );
            assert_eq!(
                record.surfaces.iter().find_map(|(feature, _, surface)| {
                    (*feature == SurfaceFeature::PAST_PARTICIPLE).then_some(surface.as_ref())
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
        let declaration = deckmaste_construction_core::macro_def::read_str(
            "/synthetic/Act.ron",
            r#"KeywordAction(name:"Act",spelling:"act",grammar:Verb(bare:"act",frame_set:Transitive))"#,
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
            environment
                .verb_inventory_surface(&plugin, SurfaceFeature::THIRD_PERSON_SINGULAR_PRESENT),
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
    fn literal_lexicon_tripwire_counts_vocab_and_rejects_form_and_tail_collisions_exactly() {
        let baseline = ParserEnvironment::try_from_declarations([])
            .expect("licensed homographs and closed-class overlaps load cleanly");
        assert_eq!(baseline.licensed_vocab_lexicon_homographs(), 1);

        let untap = deckmaste_construction_core::macro_def::read_str(
            "/synthetic/Untap.ron",
            r#"KeywordAction(name:"Untap",spelling:"untap",grammar:Verb(bare:"untap",frame_set:Transitive))"#,
        )
        .expect("synthetic untap verb is valid");
        let licensed = ParserEnvironment::try_from_declarations([untap])
            .expect("the per-member untap homograph license admits the verb declaration");
        assert_eq!(licensed.licensed_vocab_lexicon_homographs(), 2);
        assert_eq!(
            licensed.form_literal_vocab_overlaps(),
            baseline.form_literal_vocab_overlaps(),
        );

        let noun = deckmaste_construction_core::macro_def::read_str(
            "/synthetic/Long.ron",
            r#"Type(name:"Long",spelling:"long",grammar:Noun(singular:"long"))"#,
        )
        .expect("synthetic noun is valid");
        let form_collision = ParserEnvironment::try_from_declarations([noun]);
        assert!(
            matches!(
                &form_collision,
                Err(ParserEnvironmentError::LiteralLexiconCollision {
                    surface,
                    literal_owner,
                    lexical_owner,
                }) if surface == "long"
                    && literal_owner.contains("as_long_as")
                    && lexical_owner.contains("Noun declaration")
            ),
            "unexpected collision result: {form_collision:?}"
        );

        let colliding_tail = deckmaste_construction_core::macro_def::read_str(
            "/synthetic/Act.ron",
            r#"KeywordAction(name:"Act",spelling:"act",grammar:Verb(bare:"act",frame_set:Custom(frames:[[Literal("act")]])))"#,
        )
        .expect("synthetic verb is valid");
        assert!(matches!(
            ParserEnvironment::try_from_declarations([colliding_tail]),
            Err(ParserEnvironmentError::LiteralLexiconCollision {
                surface,
                literal_owner,
                lexical_owner,
            }) if surface == "act"
                && literal_owner.contains("tail atom")
                && lexical_owner.contains("Verb declaration")
        ));

        let case_distinct_tail = deckmaste_construction_core::macro_def::read_str(
            "/synthetic/Act.ron",
            r#"KeywordAction(name:"Act",spelling:"act",grammar:Verb(bare:"act",frame_set:Custom(frames:[[Literal("Act")]])))"#,
        )
        .expect("synthetic verb is valid");
        ParserEnvironment::try_from_declarations([case_distinct_tail])
            .expect("the literal/lexicon comparison is exact and case-sensitive");
    }

    #[test]
    fn vocab_collision_ownership_and_member_license_are_row_local() {
        let records = BTreeMap::new();
        let verb_inventory = BTreeMap::new();
        let scratch_card = [VocabSurface {
            surface: "card",
            vocabulary: "AttributiveAdjective",
            member: "ScratchCard",
            homograph_license: HomographLicense::Unlicensed,
        }];
        assert!(matches!(
            reject_literal_lexicon_collisions_from_surfaces(
                &records,
                &verb_inventory,
                &[],
                &scratch_card,
                &[],
            ),
            Err(ParserEnvironmentError::LiteralLexiconCollision {
                surface,
                literal_owner,
                lexical_owner,
            }) if surface == "card"
                && literal_owner.contains("AttributiveAdjective::ScratchCard")
                && lexical_owner.contains("CommonNoun::Card")
        ));

        let preposition = [VocabSurface {
            surface: "during",
            vocabulary: "Preposition",
            member: "During",
            homograph_license: HomographLicense::Unlicensed,
        }];
        let colliding_form = [FormLiteralSurface {
            surface: "during",
            construction: "synthetic_during",
            form: "plain",
            atom_index: 0,
            homograph_license: HomographLicense::Unlicensed,
        }];
        assert_eq!(
            reject_literal_lexicon_collisions_from_surfaces(
                &records,
                &verb_inventory,
                &colliding_form,
                &preposition,
                &[],
            ),
            Ok((Vec::new(), 1)),
            "the Preposition surface is a visible collision owner without turning a closed-class overlap into a load error",
        );

        let licensed_form = [FormLiteralSurface {
            homograph_license: HomographLicense::Licensed,
            ..colliding_form[0]
        }];
        assert_eq!(
            reject_literal_lexicon_collisions_from_surfaces(
                &records,
                &verb_inventory,
                &licensed_form,
                &preposition,
                &[],
            ),
            Ok((Vec::new(), 0)),
            "an explicitly licensed form homograph is separate from the overlap ceiling",
        );

        assert!(matches!(
            reject_literal_lexicon_collisions_from_surfaces(
                &records,
                &verb_inventory,
                &licensed_form,
                &[],
                &[],
            ),
            Err(ParserEnvironmentError::UngovernedHomographLicense { surface, literal_owner })
                if surface == "during" && literal_owner.contains("synthetic_during")
        ));
    }

    #[test]
    fn special_core_verb_frames_are_exact_and_class_separated() {
        use VerbFrameAtom::Lex;
        use VerbFrameAtom::Literal;
        use VerbFrameAtom::OptionalLex;
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
                OptionalLex("Preposition", "From"),
                Lex("Preposition", "On"),
                Role("FrameComplement"),
                Literal("in"),
                Role("ArbitraryDeterminer"),
                Literal("order"),
            ]),
        ));
        assert!(environment.verb_frame_licenses(
            &core(CoreVerbIdentity::Have),
            predicate(&[Role("GrantedAbility")]),
        ));
        assert!(environment.verb_frame_licenses(
            &core(CoreVerbIdentity::Gain),
            predicate(&[Role("GrantedAbility")]),
        ));
        assert!(environment.verb_frame_licenses(
            &core(CoreVerbIdentity::Cost),
            predicate(&[
                Role("ManaAmount"),
                Role("ComparisonDirection"),
                Role("ControlledCostAction"),
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
                SurfaceFeature::THIRD_PERSON_SINGULAR_PRESENT,
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
                (SurfaceFeature::PLAIN, Onset::Consonant, "scry"),
                (SurfaceFeature::PLAIN, Onset::Consonant, "scry again"),
            ],
        )
        .expect_err("a repeated realized feature fails closed");
        assert_eq!(
            error,
            ParserEnvironmentError::DuplicateSurfaceFeature {
                identity,
                feature: SurfaceFeature::PLAIN,
            }
        );
    }

    #[test]
    fn supplemental_participial_adjectives_are_indexed_by_their_own_feature() {
        let environment = canonical_test_environment();
        let keyword = environment.readings(GrammarPosition::FixedKeyword, "equip");
        assert_eq!(keyword.len(), 1);
        assert_eq!(keyword[0].feature(), SurfaceFeature::Fixed);
        let [reading] = environment.readings(GrammarPosition::FixedKeyword, "equipped") else {
            panic!("Equip contributes one supplemental participial-adjective reading")
        };
        assert_eq!(reading.id().kind(), DeclarationKind::KeywordAbility);
        assert_eq!(reading.id().name(), "Equip");
        assert_eq!(reading.feature(), SurfaceFeature::PAST_PARTICIPLE);
        let id = DeclarationId::new(DeclarationKind::KeywordAbility, "Equip");
        assert_eq!(
            environment.surface(&id, SurfaceFeature::PAST_PARTICIPLE),
            Some("equipped")
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
