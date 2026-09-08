use std::collections::BTreeMap;
use std::collections::BTreeSet;
use std::fs::File;
use std::io::BufReader;
use std::io::Read;
use std::io::Write;
use std::path::Path;
use std::path::PathBuf;
use std::time::SystemTime;
use std::time::UNIX_EPOCH;

use anyhow::Context;
use anyhow::ensure;
use clap::Args;
use clap::Subcommand;
use flate2::read::GzDecoder;
use serde::Deserialize;
use serde::Serialize;
use sha2::Digest;
use sha2::Sha256;

const SCHEMA_POLICY: &str = "scryfall-oracle-cards-jsonl-v1";

#[derive(Debug, Args)]
pub struct ScryfallSnapshotArgs {
    #[command(subcommand)]
    command: SnapshotCommand,
}

#[derive(Debug, Subcommand)]
enum SnapshotCommand {
    /// Print the current `jsonl_download_uri` from a cached descriptor.
    Uri(UriArgs),
    /// Validate a cached gzip and atomically publish its decompressed JSONL.
    Prepare(PrepareArgs),
    /// Reconcile every formerly supported `AtomicCards` face with Scryfall.
    Reconcile(ReconcileArgs),
}

#[derive(Debug, Args)]
struct UriArgs {
    #[arg(long)]
    descriptor: PathBuf,
}

#[derive(Debug, Args)]
struct PrepareArgs {
    #[arg(long)]
    descriptor: PathBuf,
    #[arg(long)]
    compressed: PathBuf,
    #[arg(long, default_value = "data/scryfall/oracle-cards.jsonl")]
    output: PathBuf,
    #[arg(long, default_value = "data/scryfall/oracle-cards.metadata.json")]
    metadata: PathBuf,
}

#[derive(Debug, Args)]
struct ReconcileArgs {
    #[arg(long, default_value = "data/mtgjson/AtomicCards.json")]
    old_atomic: PathBuf,
    #[arg(long, default_value = "data/scryfall/oracle-cards.jsonl")]
    new_oracle_cards: PathBuf,
    #[arg(long)]
    output: Option<PathBuf>,
}

#[derive(Debug, Deserialize)]
struct Descriptor {
    object: String,
    id: String,
    #[serde(rename = "type")]
    data_type: String,
    updated_at: String,
    jsonl_download_uri: String,
    compressed_size: Option<u64>,
}

#[derive(Debug, Serialize)]
struct SnapshotMetadata<'a> {
    schema_version: u32,
    schema_policy: &'a str,
    descriptor_id: &'a str,
    descriptor_updated_at: &'a str,
    jsonl_download_uri: &'a str,
    prepared_unix_seconds: u64,
    records: usize,
    compressed_bytes: u64,
    compressed_sha256: String,
    jsonl_bytes: u64,
    jsonl_sha256: String,
}

pub fn run(args: &ScryfallSnapshotArgs) -> anyhow::Result<()> {
    match &args.command {
        SnapshotCommand::Uri(args) => {
            println!("{}", read_descriptor(&args.descriptor)?.jsonl_download_uri);
        }
        SnapshotCommand::Prepare(args) => prepare(args)?,
        SnapshotCommand::Reconcile(args) => reconcile(args)?,
    }
    Ok(())
}

#[derive(Deserialize)]
struct OldAtomicCards {
    data: BTreeMap<String, Vec<OldAtomicFace>>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct OldAtomicFace {
    name: String,
    face_name: Option<String>,
    side: Option<String>,
    layout: String,
    text: Option<String>,
    #[serde(default)]
    supertypes: Vec<String>,
    #[serde(default)]
    legalities: BTreeMap<String, Option<String>>,
    identifiers: OldIdentifiers,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct OldIdentifiers {
    scryfall_oracle_id: Option<String>,
    scryfall_id: Option<String>,
}

impl OldAtomicFace {
    fn supported(&self) -> bool {
        matches!(
            self.legalities.get("vintage").and_then(Option::as_deref),
            Some("Legal" | "Restricted")
        )
    }
}

#[derive(Clone)]
struct NewFace {
    identity: String,
    oracle_id: String,
    printing_id: String,
    group_name: String,
    face_name: Option<String>,
    face_index: Option<usize>,
    layout: String,
    text: Option<String>,
    type_line: Option<String>,
    supported: bool,
}

#[derive(Serialize)]
struct ReconciliationReport {
    schema_version: u32,
    old_snapshot_sha256: String,
    new_snapshot_sha256: String,
    old_supported_faces: usize,
    new_supported_faces: usize,
    exact_identity: usize,
    identity_remaps: usize,
    ambiguous_identity_joins: usize,
    upstream_text_changes: usize,
    legality_changes: usize,
    grouping_changes: usize,
    real_losses: usize,
    grammar_results_equal_by_identical_input: usize,
    unchanged_text_with_context_change: usize,
    new_supported_additions: Vec<String>,
    rows: Vec<ReconciliationRow>,
}

#[derive(Serialize)]
struct ReconciliationRow {
    old_identity: String,
    old_printing_id: Option<String>,
    old_name: String,
    new_printing_id: Option<String>,
    candidate_new_identities: Vec<String>,
    identity_outcome: &'static str,
    text_outcome: &'static str,
    legality_outcome: &'static str,
    grouping_outcome: &'static str,
    context_outcome: &'static str,
    loss_outcome: &'static str,
}

fn reconcile(args: &ReconcileArgs) -> anyhow::Result<()> {
    let old_bytes = std::fs::read(&args.old_atomic)
        .with_context(|| format!("reading old snapshot {}", args.old_atomic.display()))?;
    let old: OldAtomicCards = serde_json::from_slice(&old_bytes)
        .with_context(|| format!("parsing old snapshot {}", args.old_atomic.display()))?;
    let new_file = File::open(&args.new_oracle_cards)
        .with_context(|| format!("opening new snapshot {}", args.new_oracle_cards.display()))?;
    let mut new_by_identity = BTreeMap::new();
    let mut new_by_oracle = BTreeMap::<String, Vec<String>>::new();
    for card in deckmaste_data::scryfall::OracleCardReader::new(BufReader::new(new_file)) {
        let card = card.context("reading new Scryfall snapshot for reconciliation")?;
        let supported = card.vintage_playable();
        for unit in card.oracle_units() {
            let identity = unit.identity().to_string();
            let face = NewFace {
                identity: identity.clone(),
                oracle_id: unit.identity().oracle_id.to_owned(),
                printing_id: unit.printing_id().to_owned(),
                group_name: unit.group_name().to_owned(),
                face_name: unit.face_index().map(|_| unit.display_name().to_owned()),
                face_index: unit.face_index(),
                layout: unit.layout().to_owned(),
                text: unit.oracle_text().map(str::to_owned),
                type_line: unit.type_line().map(str::to_owned),
                supported,
            };
            new_by_oracle
                .entry(face.oracle_id.clone())
                .or_default()
                .push(identity.clone());
            new_by_identity.insert(identity, face);
        }
    }
    for identities in new_by_oracle.values_mut() {
        identities.sort();
    }

    let mut rows = Vec::new();
    let mut used_new = BTreeSet::new();
    for faces in old.data.values() {
        for (index, face) in faces
            .iter()
            .enumerate()
            .filter(|(_, face)| face.supported())
        {
            let oracle_id = face
                .identifiers
                .scryfall_oracle_id
                .as_deref()
                .context("supported old face has no Scryfall Oracle ID")?;
            let old_index = face
                .face_name
                .as_ref()
                .map(|_| old_face_index(face.side.as_deref(), index));
            let old_identity = identity(oracle_id, old_index);
            let direct = new_by_identity.get(&old_identity);
            let candidates = if direct.is_some() {
                vec![old_identity.clone()]
            } else {
                new_by_oracle.get(oracle_id).cloned().unwrap_or_default()
            };
            let joined = if direct.is_some() {
                direct
            } else if candidates.len() == 1 {
                new_by_identity.get(&candidates[0])
            } else {
                None
            };
            if let Some(new) = joined {
                used_new.insert(new.identity.clone());
            }
            let identity_outcome = if direct.is_some() {
                "exact"
            } else if candidates.len() == 1 {
                "remapped"
            } else if candidates.len() > 1 {
                "ambiguous"
            } else {
                "missing"
            };
            let text_outcome = joined.map_or("unavailable", |new| {
                if face.text == new.text { "unchanged" } else { "changed" }
            });
            let legality_outcome = joined.map_or("unavailable", |new| {
                if new.supported { "unchanged" } else { "changed" }
            });
            let grouping_outcome = joined.map_or("unavailable", |new| {
                if face.name == new.group_name
                    && face.face_name == new.face_name
                    && old_index == new.face_index
                    && face.layout == new.layout
                {
                    "unchanged"
                } else {
                    "changed"
                }
            });
            let context_outcome = joined.map_or("unavailable", |new| {
                let old_context = face.face_name.as_deref().unwrap_or(&face.name);
                let new_context = new.face_name.as_deref().unwrap_or(&new.group_name);
                let old_legendary = face.supertypes.iter().any(|label| label == "Legendary");
                let new_legendary = new.type_line.as_deref().is_some_and(|line| {
                    line.split([' ', '\u{2014}'])
                        .any(|label| label == "Legendary")
                });
                if old_context == new_context && old_legendary == new_legendary {
                    "unchanged"
                } else {
                    "changed"
                }
            });
            let loss_outcome = if joined.is_none() && candidates.is_empty() {
                "real_loss"
            } else if joined.is_none() {
                "ambiguous_join"
            } else if joined.is_some_and(|new| !new.supported) {
                "legality_loss"
            } else {
                "retained"
            };
            rows.push(ReconciliationRow {
                old_identity,
                old_printing_id: face.identifiers.scryfall_id.clone(),
                old_name: face.face_name.clone().unwrap_or_else(|| face.name.clone()),
                new_printing_id: joined.map(|new| new.printing_id.clone()),
                candidate_new_identities: candidates,
                identity_outcome,
                text_outcome,
                legality_outcome,
                grouping_outcome,
                context_outcome,
                loss_outcome,
            });
        }
    }
    let new_supported = new_by_identity
        .values()
        .filter(|face| face.supported)
        .map(|face| face.identity.clone())
        .collect::<BTreeSet<_>>();
    let report = reconciliation_report(
        &old_bytes,
        &args.new_oracle_cards,
        rows,
        &new_supported,
        &used_new,
    )?;
    write_reconciliation_report(&report, args.output.as_deref())
}

fn write_reconciliation_report(
    report: &ReconciliationReport,
    output: Option<&Path>,
) -> anyhow::Result<()> {
    if let Some(path) = output {
        let parent = parent(path);
        std::fs::create_dir_all(parent)?;
        let mut temporary = tempfile::NamedTempFile::new_in(parent)?;
        serde_json::to_writer_pretty(&mut temporary, report)?;
        temporary.write_all(b"\n")?;
        temporary.persist(path).map_err(|error| error.error)?;
    } else {
        serde_json::to_writer_pretty(std::io::stdout().lock(), report)?;
        println!();
    }
    Ok(())
}

fn reconciliation_report(
    old_bytes: &[u8],
    new_oracle_cards: &Path,
    mut rows: Vec<ReconciliationRow>,
    new_supported: &BTreeSet<String>,
    used_new: &BTreeSet<String>,
) -> anyhow::Result<ReconciliationReport> {
    rows.sort_by(|left, right| left.old_identity.cmp(&right.old_identity));
    Ok(ReconciliationReport {
        schema_version: 1,
        old_snapshot_sha256: hex(&Sha256::digest(old_bytes)),
        new_snapshot_sha256: digest_file(new_oracle_cards)?,
        old_supported_faces: rows.len(),
        new_supported_faces: new_supported.len(),
        exact_identity: rows
            .iter()
            .filter(|row| row.identity_outcome == "exact")
            .count(),
        identity_remaps: rows
            .iter()
            .filter(|row| row.identity_outcome == "remapped")
            .count(),
        ambiguous_identity_joins: rows
            .iter()
            .filter(|row| row.identity_outcome == "ambiguous")
            .count(),
        upstream_text_changes: rows
            .iter()
            .filter(|row| row.text_outcome == "changed")
            .count(),
        legality_changes: rows
            .iter()
            .filter(|row| row.legality_outcome == "changed")
            .count(),
        grouping_changes: rows
            .iter()
            .filter(|row| row.grouping_outcome == "changed")
            .count(),
        real_losses: rows
            .iter()
            .filter(|row| row.loss_outcome == "real_loss")
            .count(),
        grammar_results_equal_by_identical_input: rows
            .iter()
            .filter(|row| {
                row.text_outcome == "unchanged"
                    && row.context_outcome == "unchanged"
                    && row.loss_outcome == "retained"
            })
            .count(),
        unchanged_text_with_context_change: rows
            .iter()
            .filter(|row| row.text_outcome == "unchanged" && row.context_outcome == "changed")
            .count(),
        new_supported_additions: new_supported.difference(used_new).cloned().collect(),
        rows,
    })
}

fn old_face_index(side: Option<&str>, fallback: usize) -> usize {
    side.and_then(|side| side.as_bytes().first().copied())
        .and_then(|side| side.checked_sub(b'a'))
        .map_or(fallback, usize::from)
}

fn identity(oracle_id: &str, face_index: Option<usize>) -> String {
    face_index.map_or_else(
        || format!("{oracle_id}#card"),
        |index| format!("{oracle_id}#face:{index}"),
    )
}

fn read_descriptor(path: &Path) -> anyhow::Result<Descriptor> {
    let bytes = std::fs::read(path)
        .with_context(|| format!("reading Scryfall descriptor {}", path.display()))?;
    let descriptor: Descriptor = serde_json::from_slice(&bytes)
        .with_context(|| format!("parsing Scryfall descriptor {}", path.display()))?;
    ensure!(
        descriptor.object == "bulk_data",
        "descriptor object must be bulk_data"
    );
    ensure!(
        descriptor.data_type == "oracle_cards",
        "descriptor type must be oracle_cards"
    );
    ensure!(
        descriptor
            .jsonl_download_uri
            .starts_with("https://data.scryfall.io/oracle-cards/")
            && descriptor.jsonl_download_uri.ends_with(".jsonl.gz"),
        "descriptor jsonl_download_uri is not a Scryfall Oracle Cards gzip URL"
    );
    Ok(descriptor)
}

struct TeeReader<R, W> {
    input: R,
    output: W,
    hash: Sha256,
    bytes: u64,
}

impl<R, W> TeeReader<R, W> {
    fn new(input: R, output: W) -> Self {
        Self {
            input,
            output,
            hash: Sha256::new(),
            bytes: 0,
        }
    }
}

impl<R: Read, W: Write> Read for TeeReader<R, W> {
    fn read(&mut self, buffer: &mut [u8]) -> std::io::Result<usize> {
        let read = self.input.read(buffer)?;
        self.output.write_all(&buffer[..read])?;
        self.hash.update(&buffer[..read]);
        self.bytes = self.bytes.saturating_add(read as u64);
        Ok(read)
    }
}

fn prepare(args: &PrepareArgs) -> anyhow::Result<()> {
    let descriptor = read_descriptor(&args.descriptor)?;
    let compressed_bytes = std::fs::metadata(&args.compressed)
        .with_context(|| format!("reading metadata for {}", args.compressed.display()))?
        .len();
    if let Some(expected) = descriptor.compressed_size {
        ensure!(
            compressed_bytes == expected,
            "compressed export size mismatch: descriptor {expected}, file {compressed_bytes}"
        );
    }
    let output_parent = parent(&args.output);
    let metadata_parent = parent(&args.metadata);
    std::fs::create_dir_all(output_parent)?;
    std::fs::create_dir_all(metadata_parent)?;
    let compressed = File::open(&args.compressed)
        .with_context(|| format!("opening {}", args.compressed.display()))?;
    let temporary = tempfile::NamedTempFile::new_in(output_parent)
        .with_context(|| format!("staging snapshot beside {}", args.output.display()))?;
    let tee = TeeReader::new(GzDecoder::new(BufReader::new(compressed)), temporary);
    let mut cards = deckmaste_data::scryfall::OracleCardReader::new(BufReader::new(tee));
    let mut records = 0usize;
    for card in cards.by_ref() {
        card.context("validating decompressed Scryfall Oracle Cards record")?;
        records += 1;
    }
    ensure!(records > 0, "Scryfall Oracle Cards export is empty");
    let tee = cards.into_inner().into_inner();
    let jsonl_sha256 = hex(&tee.hash.finalize());
    let jsonl_bytes = tee.bytes;
    let mut temporary = tee.output;
    temporary
        .flush()
        .context("flushing staged Oracle Cards JSONL")?;
    temporary
        .as_file()
        .sync_all()
        .context("syncing staged Oracle Cards JSONL")?;

    let metadata = SnapshotMetadata {
        schema_version: 1,
        schema_policy: SCHEMA_POLICY,
        descriptor_id: &descriptor.id,
        descriptor_updated_at: &descriptor.updated_at,
        jsonl_download_uri: &descriptor.jsonl_download_uri,
        prepared_unix_seconds: SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs(),
        records,
        compressed_bytes,
        compressed_sha256: digest_file(&args.compressed)?,
        jsonl_bytes,
        jsonl_sha256,
    };
    let mut metadata_temporary = tempfile::NamedTempFile::new_in(metadata_parent)?;
    serde_json::to_writer_pretty(&mut metadata_temporary, &metadata)?;
    metadata_temporary.write_all(b"\n")?;
    metadata_temporary.flush()?;
    metadata_temporary.as_file().sync_all()?;

    temporary
        .persist(&args.output)
        .map_err(|error| error.error)
        .with_context(|| format!("publishing {}", args.output.display()))?;
    metadata_temporary
        .persist(&args.metadata)
        .map_err(|error| error.error)
        .with_context(|| format!("publishing {}", args.metadata.display()))?;
    println!("validated and published {records} Oracle card records");
    Ok(())
}

fn parent(path: &Path) -> &Path {
    path.parent()
        .filter(|parent| !parent.as_os_str().is_empty())
        .unwrap_or_else(|| Path::new("."))
}

fn digest_file(path: &Path) -> anyhow::Result<String> {
    let mut file = File::open(path)?;
    let mut hash = Sha256::new();
    let mut buffer = vec![0; 64 * 1024].into_boxed_slice();
    loop {
        let read = file.read(&mut buffer)?;
        if read == 0 {
            break;
        }
        hash.update(&buffer[..read]);
    }
    Ok(hex(&hash.finalize()))
}

fn hex(bytes: &[u8]) -> String {
    use std::fmt::Write as _;
    let mut output = String::with_capacity(bytes.len() * 2);
    for byte in bytes {
        write!(output, "{byte:02x}").expect("writing to String cannot fail");
    }
    output
}

#[cfg(test)]
mod tests {
    use flate2::Compression;
    use flate2::write::GzEncoder;

    use super::*;

    fn descriptor(size: u64) -> serde_json::Value {
        serde_json::json!({
            "object": "bulk_data",
            "id": "oracle_cards",
            "type": "oracle_cards",
            "updated_at": "2026-09-08T09:01:57Z",
            "jsonl_download_uri": "https://data.scryfall.io/oracle-cards/test.jsonl.gz",
            "compressed_size": size,
        })
    }

    #[test]
    fn failed_refresh_preserves_published_snapshot_and_metadata() {
        let root = tempfile::tempdir().unwrap();
        let compressed = root.path().join("cards.gz");
        let descriptor_path = root.path().join("descriptor.json");
        let output = root.path().join("cards.jsonl");
        let metadata = root.path().join("metadata.json");
        std::fs::write(&compressed, b"not gzip").unwrap();
        std::fs::write(
            &descriptor_path,
            serde_json::to_vec(&descriptor(8)).unwrap(),
        )
        .unwrap();
        std::fs::write(&output, b"old cards\n").unwrap();
        std::fs::write(&metadata, b"old metadata\n").unwrap();
        let error = prepare(&PrepareArgs {
            descriptor: descriptor_path,
            compressed,
            output: output.clone(),
            metadata: metadata.clone(),
        })
        .unwrap_err();
        assert!(format!("{error:#}").contains("validating decompressed"));
        assert_eq!(std::fs::read(output).unwrap(), b"old cards\n");
        assert_eq!(std::fs::read(metadata).unwrap(), b"old metadata\n");
    }

    #[test]
    fn valid_refresh_publishes_snapshot_and_hash_metadata() {
        let root = tempfile::tempdir().unwrap();
        let line = concat!(
            r#"{"object":"card","id":"p","oracle_id":"o","name":"One","layout":"normal","legalities":{"vintage":"legal"}}"#,
            "\n"
        );
        let mut encoder = GzEncoder::new(Vec::new(), Compression::default());
        encoder.write_all(line.as_bytes()).unwrap();
        let bytes = encoder.finish().unwrap();
        let compressed = root.path().join("cards.gz");
        let descriptor_path = root.path().join("descriptor.json");
        let output = root.path().join("cards.jsonl");
        let metadata = root.path().join("metadata.json");
        std::fs::write(&compressed, &bytes).unwrap();
        std::fs::write(
            &descriptor_path,
            serde_json::to_vec(&descriptor(bytes.len() as u64)).unwrap(),
        )
        .unwrap();
        prepare(&PrepareArgs {
            descriptor: descriptor_path,
            compressed,
            output: output.clone(),
            metadata: metadata.clone(),
        })
        .unwrap();
        assert_eq!(std::fs::read_to_string(output).unwrap(), line);
        let metadata: serde_json::Value =
            serde_json::from_slice(&std::fs::read(metadata).unwrap()).unwrap();
        assert_eq!(metadata["records"], 1);
        assert_eq!(
            metadata["jsonl_sha256"],
            crate::raw_corpus::digest(line.as_bytes())
        );
    }
}
