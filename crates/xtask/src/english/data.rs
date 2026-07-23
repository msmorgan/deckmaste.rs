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
use deckmaste_english::normalize_roll_row_dashes;
use deckmaste_english::normalize_self_references;
use deckmaste_english::normalize_typographic_quotes;
use deckmaste_english::strip_reminder_text;
use rayon::prelude::*;
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

    /// Override the directory containing generated English catalogs.
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

pub(super) fn map_supported_faces<T: Send>(
    faces: &[CardFace],
    map: impl Fn(usize, &CardFace) -> T + Send + Sync,
) -> Vec<T> {
    let supported: Vec<_> = faces
        .iter()
        .enumerate()
        .filter(|(_, card)| card.supported)
        .collect();
    supported
        .par_iter()
        .map(|&(index, card)| map(index, card))
        .collect()
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
        let normalized_source =
            normalize_roll_row_dashes(&normalize_typographic_quotes(&source_text));
        let normalized_name = normalize_typographic_quotes(printed_name);
        let oracle_text = strip_reminder_text(&normalize_self_references(
            &normalized_source,
            &normalized_name,
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
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../../data/gen/catalogs")
}

fn load_catalogs(path: &Path) -> Result<Catalogs> {
    let mut catalogs = Catalogs::default();
    for (kind, file) in CATALOG_FILES {
        catalogs = catalogs.with_catalog(kind, load_catalog(path, file)?);
    }
    Ok(catalogs)
}

fn load_catalog(path: &Path, name: &str) -> Result<Vec<String>> {
    let path = path.join(format!("{name}.txt"));
    let file = File::open(&path).with_context(|| {
        format!(
            "could not open generated catalog {}; run `cargo xtask catalogs` first",
            path.display()
        )
    })?;
    BufReader::new(file)
        .lines()
        .collect::<std::io::Result<Vec<_>>>()
        .with_context(|| format!("could not read generated catalog {}", path.display()))
}

#[cfg(test)]
mod tests {
    use std::sync::Mutex;
    use std::sync::mpsc::sync_channel;
    use std::time::Duration;

    use deckmaste_english::syntax::AbilityKind;
    use deckmaste_english::syntax::IndependentClause;
    use deckmaste_english::syntax::NominalModifier;
    use deckmaste_english::syntax::NounPhrase;
    use deckmaste_english::syntax::Predicate;
    use deckmaste_english::syntax::PredicateObject;
    use deckmaste_english::syntax::SentenceBody;
    use deckmaste_english::word::Noun;
    use deckmaste_english::word::NounInstance;

    use super::*;

    fn face(name: &str, supported: bool) -> CardFace {
        CardFace {
            card_name: name.to_owned(),
            face_name: None,
            is_legendary: false,
            supported,
            source_text: String::new(),
            oracle_text: String::new(),
        }
    }

    #[test]
    fn supported_face_map_overlaps_work_and_preserves_source_order() {
        let faces = [
            face("first", true),
            face("skipped", false),
            face("second", true),
        ];
        let (sender, receiver) = sync_channel(1);
        let receiver = Mutex::new(receiver);
        let pool = rayon::ThreadPoolBuilder::new()
            .num_threads(2)
            .build()
            .unwrap();

        let results = pool.install(|| {
            map_supported_faces(&faces, |index, card| match card.printed_name() {
                "first" => format!(
                    "{index}:{}",
                    receiver
                        .lock()
                        .unwrap()
                        .recv_timeout(Duration::from_secs(1))
                        .unwrap()
                ),
                "second" => {
                    sender.send("released").unwrap();
                    format!("{index}:second")
                }
                name => panic!("unexpected mapped face {name}"),
            })
        });

        assert_eq!(results, ["0:released", "2:second"]);
    }

    #[test]
    fn generated_catalogs_classify_current_artifact_types() {
        let catalogs = load_catalogs(&default_catalogs_path()).unwrap();

        for artifact_type in ["Lander", "Mutagen"] {
            let source = format!("Create a {artifact_type} token.");
            let report = deckmaste_english::parse_with_catalogs(&source, &catalogs);
            let [ability] = report.ast().abilities.as_slice() else {
                panic!("expected one ability: {:#?}", report.ast());
            };
            let AbilityKind::Paragraph(paragraph) = &ability.kind else {
                panic!("expected a paragraph ability: {:#?}", ability.kind);
            };
            let [sentence] = paragraph.sentences.as_slice() else {
                panic!("expected one sentence: {paragraph:#?}");
            };
            let SentenceBody::Independent(IndependentClause::Imperative(Predicate::Transitive(
                predicate,
            ))) = &sentence.body
            else {
                panic!("expected an imperative create clause: {:#?}", sentence.body);
            };
            let PredicateObject::NounPhrase(NounPhrase::Nominal(object)) = &predicate.object else {
                panic!("expected a nominal object: {:#?}", predicate.object);
            };
            let [
                NominalModifier::Noun {
                    noun: NounInstance::Singular(Noun::Catalog(atom)),
                    ..
                },
            ] = object.modifiers.as_slice()
            else {
                panic!(
                    "expected one catalog noun modifier: {:#?}",
                    object.modifiers
                );
            };
            assert_eq!(atom.kind, CatalogKind::ArtifactType);
            assert_eq!(atom.canonical(), artifact_type);
            assert_eq!(
                report.into_ast().render("Test Card", false).unwrap(),
                source
            );
        }
    }
}
