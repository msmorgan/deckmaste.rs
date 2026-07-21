use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::path::Path;
use std::path::PathBuf;

use anyhow::Context;
use anyhow::Result;
use clap::Args;
use deckmaste_english::CatalogKind;
use deckmaste_english::Catalogs;
use deckmaste_english::normalize_self_references;
use deckmaste_english::strip_reminder_text;
use serde::Deserialize;

const CATALOG_FILES: [(CatalogKind, &str); 12] = [
    (CatalogKind::KeywordAbility, "keyword-abilities"),
    (CatalogKind::KeywordAction, "keyword-actions"),
    (CatalogKind::AbilityWord, "ability-words"),
    (CatalogKind::ArtifactType, "artifact-types"),
    (CatalogKind::BattleType, "battle-types"),
    (CatalogKind::CreatureType, "creature-types"),
    (CatalogKind::EnchantmentType, "enchantment-types"),
    (CatalogKind::LandType, "land-types"),
    (CatalogKind::PlaneswalkerType, "planeswalker-types"),
    (CatalogKind::SpellType, "spell-types"),
    (CatalogKind::Supertype, "supertypes"),
    (CatalogKind::CardType, "card-types"),
];

#[derive(Debug, Default, Args)]
pub(super) struct OracleDataArgs {
    /// Override the derived card-data snapshot.
    #[arg(long, value_name = "PATH")]
    data: Option<PathBuf>,

    /// Override the directory containing Scryfall's English catalogs.
    #[arg(long, value_name = "DIR")]
    catalogs: Option<PathBuf>,
}

impl OracleDataArgs {
    pub(super) fn load(&self) -> Result<OracleData> {
        let data_path = self.data.clone().unwrap_or_else(default_data_path);
        let catalogs_path = self.catalogs.clone().unwrap_or_else(default_catalogs_path);
        let file = File::open(&data_path).with_context(|| {
            format!(
                "could not open {}; generate the repository's derived card data first",
                data_path.display()
            )
        })?;

        Ok(OracleData {
            faces: read_card_faces(BufReader::new(file), &data_path)?,
            catalogs: load_catalogs(&catalogs_path)?,
            data_path,
        })
    }
}

pub(super) struct OracleData {
    pub(super) faces: Vec<CardFace>,
    pub(super) catalogs: Catalogs,
    pub(super) data_path: PathBuf,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) struct CardFace {
    pub(super) card_name: String,
    pub(super) face_name: Option<String>,
    pub(super) is_legendary: bool,
    pub(super) supported: bool,
    pub(super) source_text: String,
    pub(super) oracle_text: String,
}

impl CardFace {
    pub(super) fn printed_name(&self) -> &str {
        self.face_name.as_deref().unwrap_or(&self.card_name)
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct RawCardFace {
    name: String,
    face: Option<String>,
    #[serde(default)]
    supertypes: Vec<String>,
    #[serde(default)]
    supported: bool,
    text: Option<String>,
}

impl From<RawCardFace> for CardFace {
    fn from(raw: RawCardFace) -> Self {
        let printed_name = raw.face.as_deref().unwrap_or(&raw.name);
        let is_legendary = raw.supertypes.iter().any(|kind| kind == "Legendary");
        let source_text = raw.text.unwrap_or_default();
        let oracle_text = strip_reminder_text(&normalize_self_references(
            &source_text,
            printed_name,
            is_legendary,
        ));
        Self {
            card_name: raw.name,
            face_name: raw.face,
            is_legendary,
            supported: raw.supported,
            source_text,
            oracle_text,
        }
    }
}

#[derive(Debug, Deserialize)]
struct Catalog {
    data: Vec<String>,
}

pub(super) fn read_card_faces(reader: impl BufRead, data_path: &Path) -> Result<Vec<CardFace>> {
    reader
        .lines()
        .enumerate()
        .map(|(index, line)| {
            let line_number = index + 1;
            let line = line.with_context(|| {
                format!(
                    "could not read line {line_number} of {}",
                    data_path.display()
                )
            })?;
            serde_json::from_str::<RawCardFace>(&line)
                .with_context(|| {
                    format!(
                        "invalid JSON on line {line_number} of {}",
                        data_path.display()
                    )
                })
                .map(CardFace::from)
        })
        .collect()
}

fn default_data_path() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../../data/derived/cards.jsonl")
}

fn default_catalogs_path() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../../data/catalogs")
}

fn load_catalogs(path: &Path) -> Result<Catalogs> {
    let mut catalogs = Catalogs::default();
    for (kind, file) in CATALOG_FILES {
        catalogs = catalogs.with_catalog(kind, load_catalog(path, file)?);
    }
    Ok(catalogs)
}

fn load_catalog(path: &Path, name: &str) -> Result<Vec<String>> {
    let path = path.join(format!("{name}.json"));
    let file = File::open(&path).with_context(|| {
        format!(
            "could not open Scryfall catalog {}; fetch the repository data first",
            path.display()
        )
    })?;
    let catalog: Catalog = serde_json::from_reader(BufReader::new(file))
        .with_context(|| format!("invalid Scryfall catalog {}", path.display()))?;
    Ok(catalog.data)
}
