use std::collections::BTreeSet;
use std::fmt::Write as FmtWrite;
use std::fs::File;
use std::io::BufReader;
use std::io::Read;
use std::io::Write;
use std::path::Path;
use std::path::PathBuf;

use anyhow::Context;
use anyhow::ensure;
use clap::Args;
use deckmaste_data::scryfall::OracleCard;
use deckmaste_data::scryfall::OracleCardReader;
use deckmaste_data::scryfall::OracleUnitKind;
use serde::Deserialize;
use serde::Serialize;
use sha2::Digest;
use sha2::Sha256;

pub(crate) const SUPPORT_FILTER: &str =
    "Scryfall legalities.vintage is exactly legal or restricted";

/// Reproducible union of explicit identities/names and conjunctive predicates.
#[derive(Debug, Clone, Default, PartialEq, Eq, Deserialize, Serialize)]
pub(crate) struct SelectionRequest {
    #[serde(default)]
    pub oracle_ids: BTreeSet<String>,
    #[serde(default)]
    pub face_identities: BTreeSet<String>,
    #[serde(default)]
    pub card_names: BTreeSet<String>,
    #[serde(default)]
    pub layouts: BTreeSet<String>,
    #[serde(default)]
    pub text_contains: BTreeSet<String>,
    #[serde(default)]
    pub type_line_contains: BTreeSet<String>,
    #[serde(default)]
    pub all: bool,
}

#[derive(Debug, Clone, Default, Args)]
pub(crate) struct CorpusSelectionArgs {
    /// Select every supported source unit. Required when no selector is given.
    #[arg(long)]
    all: bool,
    /// Select every supported unit with this durable root/face Oracle ID.
    #[arg(long = "oracle-id", value_name = "UUID")]
    oracle_ids: Vec<String>,
    /// Select one durable `ORACLE_ID#card` or `ORACLE_ID#face:N` identity.
    #[arg(long = "face-id", value_name = "IDENTITY")]
    face_identities: Vec<String>,
    /// Select a supported card by full card name or displayed face name.
    #[arg(long = "card-name", value_name = "NAME")]
    card_names: Vec<String>,
    /// Select supported cards with this Scryfall layout.
    #[arg(long, value_name = "LAYOUT")]
    layouts: Vec<String>,
    /// Select supported source units whose raw Oracle text contains this text.
    #[arg(long, value_name = "TEXT")]
    text_contains: Vec<String>,
    /// Select supported source units whose type line contains this text.
    #[arg(long, value_name = "TEXT")]
    type_line_contains: Vec<String>,
    /// JSON selection manifest using the command's selector field names.
    #[arg(long, value_name = "PATH")]
    identity_manifest: Option<PathBuf>,
    /// Persist selected source card objects as a reusable JSONL subset.
    #[arg(long, value_name = "PATH")]
    export_subset: Option<PathBuf>,
}

impl CorpusSelectionArgs {
    pub(crate) fn all() -> Self {
        Self {
            all: true,
            ..Self::default()
        }
    }

    pub fn load(&self, data: &Path) -> anyhow::Result<SelectedCorpus> {
        let mut request = if let Some(path) = &self.identity_manifest {
            let bytes = std::fs::read(path)
                .with_context(|| format!("reading identity manifest {}", path.display()))?;
            serde_json::from_slice(&bytes)
                .with_context(|| format!("parsing identity manifest {}", path.display()))?
        } else {
            SelectionRequest::default()
        };
        request.all |= self.all;
        request.oracle_ids.extend(self.oracle_ids.iter().cloned());
        request
            .face_identities
            .extend(self.face_identities.iter().cloned());
        request.card_names.extend(self.card_names.iter().cloned());
        request.layouts.extend(self.layouts.iter().cloned());
        request
            .text_contains
            .extend(self.text_contains.iter().cloned());
        request
            .type_line_contains
            .extend(self.type_line_contains.iter().cloned());
        load_selected(data, &request, self.export_subset.as_deref())
    }
}

impl SelectionRequest {
    fn has_explicit_selectors(&self) -> bool {
        !self.oracle_ids.is_empty()
            || !self.face_identities.is_empty()
            || !self.card_names.is_empty()
    }

    fn has_predicates(&self) -> bool {
        !self.layouts.is_empty()
            || !self.text_contains.is_empty()
            || !self.type_line_contains.is_empty()
    }

    fn validate(&self) -> anyhow::Result<()> {
        let has_selection = self.has_explicit_selectors() || self.has_predicates();
        ensure!(
            self.all || has_selection,
            "corpus analysis requires a selection or explicit --all"
        );
        ensure!(
            !(self.all && has_selection),
            "--all cannot be combined with corpus selectors"
        );
        Ok(())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub(crate) enum SelectionStatus {
    Complete,
    Subset,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct SelectedFace {
    pub identity: String,
    pub oracle_id: String,
    pub face_oracle_id: Option<String>,
    pub printing_id: String,
    pub group_name: String,
    pub face_index: Option<usize>,
    pub name: String,
    pub face_name: Option<String>,
    pub side: Option<String>,
    pub layout: String,
    pub unit_kind: String,
    pub mana_cost: Option<String>,
    pub mana_value: Option<f64>,
    pub type_line: Option<String>,
    pub text: Option<String>,
    pub color_indicator: Option<Vec<String>>,
    pub colors: Option<Vec<String>>,
    pub color_identity: Option<Vec<String>>,
    pub keywords: Vec<String>,
    pub power: Option<String>,
    pub toughness: Option<String>,
    pub loyalty: Option<String>,
    pub defense: Option<String>,
    pub related_cards: Vec<deckmaste_data::scryfall::RelatedCard>,
}

#[derive(Debug, Serialize)]
pub(crate) struct SelectedCorpus {
    pub snapshot_sha256: String,
    pub selection: SelectionRequest,
    pub selection_status: SelectionStatus,
    pub records_scanned: usize,
    pub supported_faces_examined: usize,
    pub faces: Vec<SelectedFace>,
}

/// Stream one pinned snapshot, retain only selected supported faces, and
/// optionally persist the selected source card records as JSONL.
pub(crate) fn load_selected(
    path: &Path,
    request: &SelectionRequest,
    export_subset: Option<&Path>,
) -> anyhow::Result<SelectedCorpus> {
    request.validate()?;
    let file = File::open(path).with_context(|| format!("opening {}", path.display()))?;
    let mut records_scanned = 0;
    let mut supported_faces_examined = 0;
    let mut faces = Vec::new();
    let mut selected_cards = Vec::new();
    let mut found_oracle_ids = BTreeSet::new();
    let mut found_face_identities = BTreeSet::new();
    let mut found_card_names = BTreeSet::new();

    for card in OracleCardReader::new(BufReader::new(file)) {
        let card = card.with_context(|| format!("reading {}", path.display()))?;
        records_scanned += 1;
        if !card.vintage_playable() {
            continue;
        }
        let mut selected_from_card = Vec::new();
        for unit in card.oracle_units() {
            supported_faces_examined += 1;
            let identity = unit.identity();
            let identity_string = identity.to_string();
            let oracle_match = request.oracle_ids.contains(identity.oracle_id);
            let identity_match = request.face_identities.contains(&identity_string);
            let group_name_match = request.card_names.contains(unit.group_name());
            let display_name_match = request.card_names.contains(unit.display_name());
            if oracle_match {
                found_oracle_ids.insert(identity.oracle_id.to_owned());
            }
            if identity_match {
                found_face_identities.insert(identity_string.clone());
            }
            if group_name_match {
                found_card_names.insert(unit.group_name().to_owned());
            }
            if display_name_match {
                found_card_names.insert(unit.display_name().to_owned());
            }
            let explicit_match =
                oracle_match || identity_match || group_name_match || display_name_match;
            let predicate_match = request.has_predicates()
                && (request.layouts.is_empty() || request.layouts.contains(unit.layout()))
                && (request.text_contains.is_empty()
                    || request.text_contains.iter().any(|needle| {
                        unit.oracle_text().is_some_and(|text| text.contains(needle))
                    }))
                && (request.type_line_contains.is_empty()
                    || request.type_line_contains.iter().any(|needle| {
                        unit.type_line()
                            .is_some_and(|type_line| type_line.contains(needle))
                    }));
            if request.all || explicit_match || predicate_match {
                selected_from_card.push(selected_face(unit));
            }
        }
        if !selected_from_card.is_empty() {
            faces.extend(selected_from_card);
            selected_cards.push(card);
        }
    }

    ensure_found("Oracle IDs", &request.oracle_ids, &found_oracle_ids)?;
    ensure_found(
        "face identities",
        &request.face_identities,
        &found_face_identities,
    )?;
    ensure_found("card names", &request.card_names, &found_card_names)?;

    faces.sort_by(|left, right| left.identity.cmp(&right.identity));
    selected_cards.sort_by(|left, right| {
        left.oracle_id
            .as_deref()
            .cmp(&right.oracle_id.as_deref())
            .then_with(|| left.name.cmp(&right.name))
    });
    if let Some(output) = export_subset {
        write_subset(output, &selected_cards)?;
    }

    Ok(SelectedCorpus {
        snapshot_sha256: digest_file(path)?,
        selection: request.clone(),
        selection_status: if request.all {
            SelectionStatus::Complete
        } else {
            SelectionStatus::Subset
        },
        records_scanned,
        supported_faces_examined,
        faces,
    })
}

fn ensure_found(
    label: &str,
    requested: &BTreeSet<String>,
    found: &BTreeSet<String>,
) -> anyhow::Result<()> {
    let missing = requested.difference(found).cloned().collect::<Vec<_>>();
    ensure!(
        missing.is_empty(),
        "missing requested {label}: {}",
        missing.join(", ")
    );
    Ok(())
}

fn selected_face(unit: deckmaste_data::scryfall::OracleUnit<'_>) -> SelectedFace {
    let identity = unit.identity();
    SelectedFace {
        identity: identity.to_string(),
        oracle_id: identity.oracle_id.to_owned(),
        face_oracle_id: unit.face_oracle_id().map(str::to_owned),
        printing_id: unit.printing_id().to_owned(),
        group_name: unit.group_name().to_owned(),
        face_index: unit.face_index(),
        name: unit.display_name().to_owned(),
        face_name: unit.face_index().map(|_| unit.display_name().to_owned()),
        side: unit.face_index().map(face_side),
        layout: unit.layout().to_owned(),
        unit_kind: match unit.kind() {
            OracleUnitKind::CardFace => "card_face",
            OracleUnitKind::AlternativeCharacteristics => "alternative_characteristics",
        }
        .to_owned(),
        mana_cost: unit.mana_cost().map(str::to_owned),
        mana_value: unit.mana_value(),
        type_line: unit.type_line().map(str::to_owned),
        text: unit.oracle_text().map(str::to_owned),
        color_indicator: unit.color_indicator().map(<[String]>::to_vec),
        colors: unit.colors().map(<[String]>::to_vec),
        color_identity: unit.color_identity().map(<[String]>::to_vec),
        keywords: unit.keywords().to_vec(),
        power: unit.power().map(str::to_owned),
        toughness: unit.toughness().map(str::to_owned),
        loyalty: unit.loyalty().map(str::to_owned),
        defense: unit.defense().map(str::to_owned),
        related_cards: unit.related_cards().to_vec(),
    }
}

fn face_side(index: usize) -> String {
    u8::try_from(index)
        .ok()
        .and_then(|index| b'a'.checked_add(index))
        .filter(u8::is_ascii_lowercase)
        .map_or_else(|| index.to_string(), |side| char::from(side).to_string())
}

fn write_subset(path: &Path, cards: &[OracleCard]) -> anyhow::Result<()> {
    let parent = path
        .parent()
        .filter(|parent| !parent.as_os_str().is_empty())
        .unwrap_or(Path::new("."));
    std::fs::create_dir_all(parent)
        .with_context(|| format!("creating subset directory {}", parent.display()))?;
    let mut temporary = tempfile::NamedTempFile::new_in(parent)
        .with_context(|| format!("staging subset in {}", parent.display()))?;
    for card in cards {
        serde_json::to_writer(&mut temporary, card)
            .context("serializing selected Scryfall card")?;
        temporary
            .write_all(b"\n")
            .context("writing selected Scryfall card")?;
    }
    temporary
        .flush()
        .context("flushing selected Scryfall cards")?;
    temporary
        .persist(path)
        .map_err(|error| error.error)
        .with_context(|| format!("publishing subset {}", path.display()))?;
    Ok(())
}

fn digest_file(path: &Path) -> anyhow::Result<String> {
    let mut file =
        File::open(path).with_context(|| format!("opening {} for hashing", path.display()))?;
    let mut hasher = Sha256::new();
    let mut buffer = vec![0_u8; 64 * 1024].into_boxed_slice();
    loop {
        let read = file
            .read(&mut buffer)
            .with_context(|| format!("hashing {}", path.display()))?;
        if read == 0 {
            break;
        }
        hasher.update(&buffer[..read]);
    }
    let mut result = String::with_capacity(64);
    for byte in hasher.finalize() {
        write!(result, "{byte:02x}").expect("writing to a String is infallible");
    }
    Ok(result)
}

pub(crate) fn digest(bytes: &[u8]) -> String {
    let mut result = String::with_capacity(64);
    for byte in Sha256::digest(bytes) {
        write!(result, "{byte:02x}").expect("writing to a String is infallible");
    }
    result
}

#[cfg(test)]
mod selection_tests {
    use std::fs;

    use super::*;

    fn record(oracle_id: &str, name: &str, vintage: &str, faces: serde_json::Value) -> String {
        let mut value = serde_json::json!({
            "object": "card",
            "id": format!("printing-{oracle_id}"),
            "oracle_id": oracle_id,
            "name": name,
            "layout": if faces.is_null() { "normal" } else { "transform" },
            "cmc": 1.0,
            "type_line": "Creature — Human",
            "oracle_text": "top text",
            "colors": [],
            "color_identity": [],
            "legalities": { "vintage": vintage }
        });
        if !faces.is_null() {
            value.as_object_mut().unwrap().remove("oracle_text");
            value
                .as_object_mut()
                .unwrap()
                .insert("card_faces".into(), faces);
        }
        serde_json::to_string(&value).unwrap()
    }

    #[test]
    fn explicit_selection_is_stable_bounded_and_exportable() {
        let a = record("oracle-a", "Alpha", "legal", serde_json::Value::Null);
        let b = record(
            "oracle-b",
            "Beta // Gamma",
            "restricted",
            serde_json::json!([
                {"name":"Beta", "mana_cost":"{1}", "type_line":"Creature — Human", "oracle_text":"front", "colors":[]},
                {"name":"Gamma", "mana_cost":"", "type_line":"Land", "oracle_text":"back", "colors":[]}
            ]),
        );
        let c = record(
            "oracle-c",
            "Unsupported",
            "not_legal",
            serde_json::Value::Null,
        );
        let request = SelectionRequest {
            oracle_ids: BTreeSet::from(["oracle-a".to_owned()]),
            face_identities: BTreeSet::from(["oracle-b#face:1".to_owned()]),
            ..SelectionRequest::default()
        };
        let root = tempfile::tempdir().unwrap();
        let first = root.path().join("first.jsonl");
        let reordered = root.path().join("reordered.jsonl");
        let subset = root.path().join("subset.jsonl");
        fs::write(&first, format!("{a}\n{b}\n{c}\n")).unwrap();
        fs::write(&reordered, format!("{c}\n{b}\n{a}\n")).unwrap();

        let selected = load_selected(&first, &request, Some(&subset)).unwrap();
        let selected_reordered = load_selected(&reordered, &request, None).unwrap();

        assert_eq!(selected.records_scanned, 3);
        assert_eq!(selected.faces.len(), 2);
        assert_eq!(
            selected
                .faces
                .iter()
                .map(|face| face.identity.as_str())
                .collect::<Vec<_>>(),
            ["oracle-a#card", "oracle-b#face:1"]
        );
        assert_eq!(selected.faces, selected_reordered.faces);
        assert_eq!(selected.selection_status, SelectionStatus::Subset);

        let exported = load_selected(&subset, &request, None).unwrap();
        assert_eq!(selected.faces, exported.faces);
        assert_eq!(exported.records_scanned, 2);
    }

    #[test]
    fn missing_requested_identities_fail_visibly() {
        let root = tempfile::tempdir().unwrap();
        let input = root.path().join("cards.jsonl");
        fs::write(
            &input,
            format!(
                "{}\n",
                record("oracle-a", "Alpha", "legal", serde_json::Value::Null)
            ),
        )
        .unwrap();
        let request = SelectionRequest {
            card_names: BTreeSet::from(["Missing Name".to_owned()]),
            ..SelectionRequest::default()
        };

        let error = load_selected(&input, &request, None).unwrap_err();

        assert!(format!("{error:#}").contains("missing requested card names: Missing Name"));
    }

    #[test]
    fn identity_manifest_and_command_line_selectors_form_one_reproducible_request() {
        let root = tempfile::tempdir().unwrap();
        let input = root.path().join("cards.jsonl");
        fs::write(
            &input,
            format!(
                "{}\n{}\n",
                record("oracle-a", "Alpha", "legal", serde_json::Value::Null),
                record("oracle-b", "Beta", "restricted", serde_json::Value::Null)
            ),
        )
        .unwrap();
        let manifest = root.path().join("selection.json");
        fs::write(&manifest, r#"{"card_names":["Alpha"]}"#).unwrap();
        let args = CorpusSelectionArgs {
            oracle_ids: vec!["oracle-b".to_owned()],
            identity_manifest: Some(manifest),
            ..CorpusSelectionArgs::default()
        };

        let selected = args.load(&input).unwrap();

        assert_eq!(
            selected
                .faces
                .iter()
                .map(|face| face.identity.as_str())
                .collect::<Vec<_>>(),
            ["oracle-a#card", "oracle-b#card"]
        );
        assert_eq!(
            selected.selection.card_names,
            BTreeSet::from(["Alpha".to_owned()])
        );
        assert_eq!(
            selected.selection.oracle_ids,
            BTreeSet::from(["oracle-b".to_owned()])
        );
    }

    #[test]
    fn analysis_without_a_selector_or_all_is_rejected_before_opening_data() {
        let error = load_selected(
            Path::new("this-file-must-not-be-opened.jsonl"),
            &SelectionRequest::default(),
            None,
        )
        .unwrap_err();

        assert_eq!(
            error.to_string(),
            "corpus analysis requires a selection or explicit --all"
        );
    }

    #[test]
    fn large_stream_scans_every_record_but_materializes_only_the_selection() {
        let root = tempfile::tempdir().unwrap();
        let input = root.path().join("many-cards.jsonl");
        let mut contents = String::new();
        for index in 0..5_000 {
            let oracle_id = format!("oracle-{index:04}");
            contents.push_str(&record(
                &oracle_id,
                &format!("Card {index}"),
                "legal",
                serde_json::Value::Null,
            ));
            contents.push('\n');
        }
        fs::write(&input, contents).unwrap();
        let request = SelectionRequest {
            oracle_ids: BTreeSet::from(["oracle-0007".to_owned(), "oracle-4999".to_owned()]),
            ..SelectionRequest::default()
        };

        let selected = load_selected(&input, &request, None).unwrap();

        assert_eq!(selected.records_scanned, 5_000);
        assert_eq!(selected.supported_faces_examined, 5_000);
        assert_eq!(selected.faces.len(), 2);
        assert_eq!(selected.faces[0].identity, "oracle-0007#card");
        assert_eq!(selected.faces[1].identity, "oracle-4999#card");
    }
}
