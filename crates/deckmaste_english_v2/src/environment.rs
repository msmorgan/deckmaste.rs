//! Immutable, data-defined grammar vocabulary for the English v2 parser.

use std::collections::BTreeMap;
use std::fmt;
use std::path::Path;
use std::path::PathBuf;
use std::sync::Arc;

use macro_ron::v2::DeclarationKind;
use macro_ron::v2::GrammarRecipe;
use macro_ron::v2::NormalizedDeclaration;
use macro_ron::v2::SurfaceFeature;
use macro_ron::v2::VerbValence;

/// An owned, category-safe declaration identity.
///
/// Names are runtime data. Adding a declaration never adds a Rust enum
/// variant, and equal local names in different declaration kinds remain
/// distinct identities.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Ord, PartialOrd)]
pub struct DeclarationId {
    kind: DeclarationKind,
    name: Arc<str>,
}

impl DeclarationId {
    /// Constructs an owned declaration identity.
    #[must_use]
    pub fn new(kind: DeclarationKind, name: impl AsRef<str>) -> Self {
        Self {
            kind,
            name: Arc::from(name.as_ref()),
        }
    }

    /// Returns the open registry family containing this identity.
    #[must_use]
    pub fn kind(&self) -> DeclarationKind {
        self.kind
    }

    /// Returns the declaration's local name.
    #[must_use]
    pub fn name(&self) -> &str {
        &self.name
    }
}

impl fmt::Display for DeclarationId {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{} `{}`", self.kind, self.name)
    }
}

/// The closed grammatical position in which a declaration surface can scan.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd)]
pub enum GrammarPosition {
    Verb,
    Noun,
    FixedTerm,
    FixedClause,
    FixedKeyword,
}

impl GrammarPosition {
    fn from_recipe(recipe: &GrammarRecipe) -> Self {
        match recipe {
            GrammarRecipe::Verb { .. } => Self::Verb,
            GrammarRecipe::Noun => Self::Noun,
            GrammarRecipe::FixedTerm => Self::FixedTerm,
            GrammarRecipe::FixedClause => Self::FixedClause,
            GrammarRecipe::FixedKeyword => Self::FixedKeyword,
        }
    }
}

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
        let mut records = BTreeMap::<DeclarationKind, BTreeMap<Arc<str>, DeclarationRecord>>::new();
        for declaration in declarations {
            let id =
                DeclarationId::new(declaration.identity().kind(), declaration.identity().name());
            let provenance = declaration.provenance().path().to_owned();
            let records_by_name = records.entry(id.kind).or_default();
            if let Some(first) = records_by_name.get(id.name()) {
                return Err(ParserEnvironmentError::DuplicateIdentity {
                    identity: id,
                    first_path: first.provenance.clone(),
                    duplicate_path: provenance,
                });
            }

            let (recipe, surfaces) = match declaration.grammar() {
                Some(grammar) => {
                    let mut surfaces = Vec::with_capacity(grammar.surfaces().len());
                    for realized in grammar.surfaces() {
                        if surfaces
                            .iter()
                            .any(|(feature, _)| *feature == realized.feature())
                        {
                            return Err(ParserEnvironmentError::DuplicateSurfaceFeature {
                                identity: id,
                                feature: realized.feature(),
                            });
                        }
                        surfaces.push((realized.feature(), Arc::from(realized.text())));
                    }
                    (Some(grammar.recipe().clone()), surfaces)
                }
                None => (None, Vec::new()),
            };
            records_by_name.insert(
                Arc::clone(&id.name),
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
        for records_by_name in records.values() {
            for record in records_by_name.values() {
                let Some(recipe) = &record.recipe else {
                    continue;
                };
                let position = GrammarPosition::from_recipe(recipe);
                for (feature, surface) in &record.surfaces {
                    readings
                        .entry(position)
                        .or_default()
                        .entry(Arc::clone(surface))
                        .or_default()
                        .push(DeclarationReading {
                            id: record.id.clone(),
                            position,
                            feature: *feature,
                            surface: Arc::clone(surface),
                        });
                }
            }
        }

        Ok(Self {
            data: Arc::new(EnvironmentData {
                declarations: records,
                readings,
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

    /// Returns the exact surface for a declaration and realized feature.
    #[must_use]
    pub fn surface(&self, id: &DeclarationId, feature: SurfaceFeature) -> Option<&str> {
        self.data
            .declarations
            .get(&id.kind)
            .and_then(|records| records.get(id.name()))
            .and_then(|record| record.surface(feature))
    }
}
