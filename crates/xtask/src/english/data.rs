use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::path::Path;
use std::path::PathBuf;

use anyhow::Context;
use anyhow::Result;
use clap::Args;
use deckmaste_catalogs::legacy::LegacyCatalogSet;
use deckmaste_english::Catalogs;
use deckmaste_english::normalize_loyalty_minus;
use deckmaste_english::normalize_roll_row_dashes;
use deckmaste_english::normalize_sentence_case;
use deckmaste_english::normalize_typographic_quotes;
use deckmaste_english::strip_reminder_text;
use rayon::prelude::*;
use serde::Deserialize;

/// Parsing one face can briefly own a large chart and packed forest. Keep the
/// corpus tools parallel without multiplying a pathological face by every
/// hardware thread on the machine.
const MAX_SUPPORTED_FACE_JOBS: usize = 4;

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
    let available = std::thread::available_parallelism().map_or(1, usize::from);
    map_supported_faces_with_workers(faces, available, map)
}

/// Map the supported corpus with an explicit upper bound on parser workers.
///
/// Corpus runners use this when the worker count is part of a measurement. A
/// single input must be audited separately first: this bound limits only the
/// outer multiplier, never the work one parse creates.
pub(super) fn map_supported_faces_with_workers<T: Send>(
    faces: &[CardFace],
    requested_workers: usize,
    map: impl Fn(usize, &CardFace) -> T + Send + Sync,
) -> Vec<T> {
    let supported: Vec<_> = faces
        .iter()
        .enumerate()
        .filter(|(_, card)| card.supported)
        .collect();
    if supported.is_empty() {
        return Vec::new();
    }
    let jobs = supported_face_jobs(requested_workers, supported.len());
    rayon::ThreadPoolBuilder::new()
        .num_threads(jobs)
        .thread_name(|index| format!("english-corpus-{index}"))
        .build()
        .expect("bounded English corpus pool must build")
        .install(|| {
            supported
                .par_iter()
                .map(|&(index, card)| map(index, card))
                .collect()
        })
}

pub(super) fn supported_face_jobs(requested_workers: usize, supported: usize) -> usize {
    requested_workers
        .min(MAX_SUPPORTED_FACE_JOBS)
        .min(supported)
        .max(1)
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
        let is_legendary = raw.supertypes.iter().any(|kind| kind == "Legendary");
        let source_text = raw.text.unwrap_or_default();
        // The parser consumes normalized, name-bearing Oracle text: quotes and
        // roll-row dashes are canonicalized and reminder text is stripped, but
        // the face's own name is left intact for the parser to recognize as a
        // self-reference (rather than substituted for a sigil). Sentence case
        // is normalized last, after reminder text is gone, so a stripped
        // reminder can never shift a sentence boundary's position.
        let normalized_source = normalize_loyalty_minus(&normalize_roll_row_dashes(
            &normalize_typographic_quotes(&source_text),
        ));
        let oracle_text = normalize_sentence_case(&strip_reminder_text(&normalized_source));
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
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../../data/gen/catalogs-legacy")
}

fn load_catalogs(generated_path: &Path) -> Result<Catalogs> {
    let workspace_data = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../data");
    let raw = LegacyCatalogSet::load(generated_path, &workspace_data.join("catalogs"))?;
    let catalogs = Catalogs::from_legacy(&raw);
    Ok(catalogs)
}

#[cfg(test)]
mod tests {
    use std::sync::Mutex;
    use std::sync::mpsc::sync_channel;
    use std::time::Duration;

    use deckmaste_english::CatalogKind;
    use deckmaste_english::syntax::AbilityHeader;
    use deckmaste_english::syntax::AbilityKind;
    use deckmaste_english::syntax::IndependentClause;
    use deckmaste_english::syntax::NominalModifier;
    use deckmaste_english::syntax::Predicate;
    use deckmaste_english::syntax::PredicateExpression;
    use deckmaste_english::syntax::PredicateObject;
    use deckmaste_english::syntax::SentenceBody;
    use deckmaste_english::word::Noun;
    use deckmaste_english::word::NounInstanceKind;

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
    fn supported_face_map_caps_parser_concurrency() {
        assert_eq!(supported_face_jobs(64, 100), MAX_SUPPORTED_FACE_JOBS);
        assert_eq!(supported_face_jobs(2, 100), 2);
        assert_eq!(supported_face_jobs(64, 3), 3);
    }

    #[test]
    #[cfg_attr(
        not(gen_catalogs),
        ignore = "needs generated data/gen/catalogs-legacy; run `cargo xtask catalogs text`"
    )]
    fn generated_catalogs_classify_current_artifact_types() {
        let catalogs = load_catalogs(&default_catalogs_path()).unwrap();

        for artifact_type in ["Lander", "Mutagen"] {
            let source = format!("Create a {artifact_type} token.");
            let report = deckmaste_english::parse_with_catalogs(&source, &catalogs);
            let [ability] = report.ast().abilities.as_slice() else {
                panic!("expected one ability: {:#?}", report.ast());
            };
            let AbilityKind::Paragraph(paragraph) = ability.kind() else {
                panic!("expected a paragraph ability: {:#?}", ability.kind());
            };
            let [sentence] = paragraph.sentences.as_slice() else {
                panic!("expected one sentence: {paragraph:#?}");
            };
            let SentenceBody::Independent(IndependentClause::Finite(clause)) = sentence.body()
            else {
                panic!(
                    "expected an imperative create clause: {:#?}",
                    sentence.body()
                );
            };
            let None = clause.subject() else {
                panic!("expected an implicit imperative subject: {clause:#?}");
            };
            let PredicateExpression::Simple(Predicate::Transitive(predicate)) = clause.predicate()
            else {
                panic!("expected a transitive create predicate: {clause:#?}");
            };
            let PredicateObject::NounPhrase(noun_phrase) = predicate.object() else {
                panic!("expected a nominal object: {:#?}", predicate.object());
            };
            let deckmaste_english::syntax::NounPhraseKind::Nominal(object) = noun_phrase.kind()
            else {
                panic!("expected a nominal object: {:#?}", predicate.object());
            };
            let [NominalModifier::Noun { noun, .. }] = object.modifiers() else {
                panic!(
                    "expected one catalog noun modifier: {:#?}",
                    object.modifiers()
                );
            };
            let NounInstanceKind::Singular(Noun::Catalog(atom)) = noun.kind() else {
                panic!("expected one singular catalog noun modifier: {noun:#?}");
            };
            assert_eq!(atom.kind, CatalogKind::ArtifactType);
            assert_eq!(atom.canonical(), artifact_type);
            assert_eq!(
                report.into_ast().render("Test Card", false).unwrap(),
                source
            );
        }
    }

    #[test]
    #[cfg_attr(
        not(gen_catalogs),
        ignore = "needs generated data/gen/catalogs-legacy; run `cargo xtask catalogs text`"
    )]
    fn scryfall_flavor_words_license_a_header_peel() {
        // The shared legacy loader reads flavor words from the Scryfall dump.
        // `Polymorphine` (Callidus Assassin) is a real member: loaded from the
        // dump, it licenses the ability-level header peel end to end.
        let catalogs = load_catalogs(&default_catalogs_path()).unwrap();

        let source = "Polymorphine — Draw a card.";
        let report = deckmaste_english::parse_with_catalogs(source, &catalogs);
        let [ability] = report.ast().abilities.as_slice() else {
            panic!("expected one ability: {:#?}", report.ast());
        };
        let Some(AbilityHeader::Flavor(flavor)) = ability.header() else {
            panic!("a Scryfall flavor word should license the header peel: {ability:#?}");
        };
        assert_eq!(flavor.text(), "Polymorphine");
        assert_eq!(
            report
                .into_ast()
                .render("Callidus Assassin", false)
                .unwrap(),
            source
        );
    }
}
