use std::collections::BTreeMap;
use std::collections::BTreeSet;
use std::path::Path;
use std::path::PathBuf;

use anyhow::Context;
use anyhow::Result;
use deckmaste_lexical::LexicalReading;
use deckmaste_lexical::Lexicon;
use serde::Deserialize;
use serde::Serialize;

use super::EnglishV3Args;
use super::SourceField;
use super::analyze_cards;
use super::report::Enumeration;
use super::report::FaceReport;
use crate::raw_corpus::SelectedCorpus;
use crate::raw_corpus::digest;

const LEXICON_SOURCES: [&str; 3] = [
    "crates/deckmaste_lexical_source/lexicon/core.ron",
    "crates/deckmaste_lexical_source/lexicon/verbs.ron",
    "crates/deckmaste_lexical_source/lexicon/vocabulary.ron",
];

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub(super) struct LexemeState {
    pub fingerprint: String,
    pub surfaces: BTreeSet<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub(super) struct Snapshot {
    pub sources: BTreeMap<String, String>,
    pub lexemes: BTreeMap<String, LexemeState>,
}

impl Snapshot {
    pub fn capture(root: &Path, lexicon: &Lexicon) -> Result<Self> {
        let mut sources = BTreeMap::new();
        // Include the parser, compiler, lexical adapters, normalization and
        // their dependencies. A change outside the three supported lexical
        // sources disables reuse rather than guessing its impact. Grammar-owned
        // closed words live in declarations.rs, so registry changes also
        // disable reuse; they are not treated as data-only lexical edits.
        for directory in [
            "crates/deckmaste_english_v3/src",
            "crates/deckmaste_construction_v3/src",
            "crates/deckmaste_construction_v3_core/src",
            "crates/deckmaste_macro_def/src",
            "crates/deckmaste_lexical/src",
            "crates/deckmaste_lexical_model/src",
            "crates/deckmaste_lexical_source/src",
            "crates/deckmaste_lexical_source/lexicon",
            "crates/deckmaste_catalogs/src",
            "crates/deckmaste_data/src",
            "crates/macro_ron/src",
            "crates/macro_ron_derive/src",
            "crates/deckmaste_features/src",
            "crates/xtask/src",
            "plugins_v2/builtin",
            "data/gen/catalogs",
        ] {
            hash_directory(root, &root.join(directory), &mut sources)?;
        }
        for path in [
            "Cargo.toml",
            "Cargo.lock",
            ".cargo/config.toml",
            "data/scryfall/oracle-cards.jsonl",
        ] {
            hash_file(root, &root.join(path), &mut sources)?;
        }
        for package in [
            "deckmaste_english_v3",
            "deckmaste_construction_v3",
            "deckmaste_construction_v3_core",
            "deckmaste_macro_def",
            "deckmaste_lexical",
            "deckmaste_lexical_model",
            "deckmaste_lexical_source",
            "deckmaste_catalogs",
            "deckmaste_data",
            "macro_ron",
            "macro_ron_derive",
            "deckmaste_features",
            "xtask",
        ] {
            hash_file(
                root,
                &root.join(format!("crates/{package}/Cargo.toml")),
                &mut sources,
            )?;
        }
        Ok(Self {
            sources,
            lexemes: Self::lexemes(lexicon)?,
        })
    }

    pub fn lexemes(lexicon: &Lexicon) -> Result<BTreeMap<String, LexemeState>> {
        let mut entries = BTreeMap::new();
        for (id, lexeme) in lexicon.lexemes() {
            entries.insert(
                id.to_string(),
                LexemeState {
                    fingerprint: digest(&serde_json::to_vec(lexeme)?),
                    surfaces: BTreeSet::new(),
                },
            );
        }
        for value in lexicon.values() {
            entries
                .get_mut(&value.lexeme.to_string())
                .context("licensed value lacks its declaration")?
                .surfaces
                .insert(lexicon.realize(&LexicalReading::Word(value.clone()))?);
        }
        Ok(entries)
    }
}

fn hash_directory(
    root: &Path,
    directory: &Path,
    sources: &mut BTreeMap<String, String>,
) -> Result<()> {
    for entry in
        std::fs::read_dir(directory).with_context(|| format!("reading {}", directory.display()))?
    {
        let path = entry?.path();
        if path.is_dir() {
            hash_directory(root, &path, sources)?;
        } else {
            hash_file(root, &path, sources)?;
        }
    }
    Ok(())
}

fn hash_file(root: &Path, path: &Path, sources: &mut BTreeMap<String, String>) -> Result<()> {
    let bytes = std::fs::read(path).with_context(|| format!("reading {}", path.display()))?;
    sources.insert(
        path.strip_prefix(root)?.to_string_lossy().into_owned(),
        digest(&bytes),
    );
    Ok(())
}

#[derive(Debug, Default, Serialize)]
pub(super) struct Incremental {
    #[serde(rename = "incremental")]
    pub used: bool,
    pub base_census: Option<PathBuf>,
    pub fallback_reason: Option<String>,
    pub changed_sources: Vec<String>,
    pub affected_lexemes: Vec<String>,
    pub affected_forms: BTreeSet<String>,
    pub rechecked_faces: Vec<String>,
    pub reused_faces: Vec<String>,
}

#[derive(Deserialize)]
pub(super) struct Base {
    pub schema_version: usize,
    pub readings_digest_version: usize,
    pub decomposition_version: usize,
    pub context_version: usize,
    pub normal_form_version: usize,
    pub normal_form_source_sha256: String,
    pub field: SourceField,
    pub input_sha256: String,
    pub support_filter: String,
    pub reading_limit: Option<usize>,
    pub samples_per_face: usize,
    pub incremental_snapshot: Snapshot,
    pub faces: Vec<FaceReport>,
}

impl Base {
    pub fn load(path: &Path) -> Result<Option<Self>> {
        let file = std::fs::File::open(path)
            .with_context(|| format!("opening base census {}", path.display()))?;
        let json: serde_json::Value = serde_json::from_reader(std::io::BufReader::new(file))
            .context("reading base census JSON")?;
        // Old census schemas lack the evidence required for safe reuse.
        if json["schema_version"] != super::report::SCHEMA_VERSION
            || json["readings_digest_version"] != super::report::READINGS_DIGEST_VERSION
            || json["decomposition_version"] != super::report::DECOMPOSITION_VERSION
            || json["context_version"] != super::report::CONTEXT_VERSION
            || json["normal_form_version"] != super::reading_normal_form::VERSION
            || json["normal_form_source_sha256"].as_str()
                != Some(super::reading_normal_form::source_sha256().as_str())
            || json["incremental_snapshot"].is_null()
        {
            return Ok(None);
        }
        Ok(Some(
            serde_json::from_value(json).context("decoding base census")?,
        ))
    }

    fn incompatibility(
        &self,
        current: &Snapshot,
        corpus: &SelectedCorpus,
        args: &EnglishV3Args,
    ) -> Option<String> {
        if self.schema_version != super::report::SCHEMA_VERSION
            || self.readings_digest_version != super::report::READINGS_DIGEST_VERSION
            || self.decomposition_version != super::report::DECOMPOSITION_VERSION
            || self.context_version != super::report::CONTEXT_VERSION
        {
            return Some("census or Readings digest version changed".into());
        }
        if self.normal_form_version != super::reading_normal_form::VERSION
            || self.normal_form_source_sha256 != super::reading_normal_form::source_sha256()
        {
            return Some(
                "normal-form version or classification/normalization source changed".into(),
            );
        }
        if self.field != args.field
            || self.input_sha256 != corpus.snapshot_sha256
            || self.support_filter != crate::raw_corpus::SUPPORT_FILTER
            || self.reading_limit != args.reading_limit.map(std::num::NonZeroUsize::get)
            || self.samples_per_face != args.samples_per_face
        {
            return Some("input, field, support filter or enumeration policy changed".into());
        }
        if args.reading_limit.is_some() {
            return Some("incremental reuse requires complete enumeration".into());
        }
        let changed = changed_keys(&self.incremental_snapshot.sources, &current.sources);
        if let Some(path) = changed
            .iter()
            .find(|path| !LEXICON_SOURCES.contains(&path.as_str()))
        {
            return Some(format!("non-lexicon source changed: {path}"));
        }
        let mut ids = BTreeSet::new();
        for face in &self.faces {
            if !ids.insert(&face.id)
                || face.enumeration != Enumeration::Complete
                || !face.issues.is_empty()
                || face.readings_sha256.is_none()
                || face.exact_readings != Some(face.checked_readings)
            {
                return Some("base faces are duplicate, incomplete or invalid".into());
            }
        }
        if corpus
            .faces
            .iter()
            .any(|face| !ids.contains(&face.identity))
        {
            return Some("base census does not contain every selected face".into());
        }
        None
    }
}

fn changed_keys<T: PartialEq>(
    before: &BTreeMap<String, T>,
    after: &BTreeMap<String, T>,
) -> BTreeSet<String> {
    before
        .keys()
        .chain(after.keys())
        .filter(|key| before.get(*key) != after.get(*key))
        .cloned()
        .collect()
}

pub(super) fn analyze(
    corpus: &SelectedCorpus,
    lexicon: &Lexicon,
    grammar: &deckmaste_english_v3::grammar::Grammar,
    args: &EnglishV3Args,
    current: &Snapshot,
    base: Option<Base>,
) -> Result<(Vec<FaceReport>, Incremental)> {
    let mut mode = Incremental {
        base_census: args.incremental_from.clone(),
        ..Incremental::default()
    };
    if args.incremental_from.is_none() {
        return Ok((analyze_cards(&corpus.faces, lexicon, grammar, args)?, mode));
    }
    let Some(base) = base else {
        mode.fallback_reason = Some("base census lacks compatible incremental metadata".into());
        return Ok((analyze_cards(&corpus.faces, lexicon, grammar, args)?, mode));
    };
    if let Some(reason) = base.incompatibility(current, corpus, args) {
        mode.fallback_reason = Some(reason);
        return Ok((analyze_cards(&corpus.faces, lexicon, grammar, args)?, mode));
    }
    mode.changed_sources = changed_keys(&base.incremental_snapshot.sources, &current.sources)
        .into_iter()
        .collect();
    mode.affected_lexemes = changed_keys(&base.incremental_snapshot.lexemes, &current.lexemes)
        .into_iter()
        .collect();
    for id in &mode.affected_lexemes {
        for state in [
            base.incremental_snapshot.lexemes.get(id),
            current.lexemes.get(id),
        ]
        .into_iter()
        .flatten()
        {
            mode.affected_forms
                .extend(state.surfaces.iter().map(|surface| surface.to_lowercase()));
        }
    }
    let mut previous: BTreeMap<_, _> = base
        .faces
        .into_iter()
        .map(|face| (face.id.clone(), face))
        .collect();
    let mut pending = Vec::new();
    for face in &corpus.faces {
        // Match conservative substrings, including bound forms and internal
        // pieces of multiword forms; extra checks cannot lose a Reading.
        let old = &previous[&face.identity];
        // Eligibility fixes the input and normalization policy. Use the
        // recorded parser input: removing reminders can join the components
        // of a multiword form that is not contiguous in the raw source.
        let source = old.analyzed_source.to_lowercase();
        let source_unchanged =
            old.source_sha256 == digest(args.field.source(face).unwrap_or("").as_bytes());
        if !source_unchanged || mode.affected_forms.iter().any(|form| source.contains(form)) {
            pending.push(face.clone());
            mode.rechecked_faces.push(face.identity.clone());
        } else {
            mode.reused_faces.push(face.identity.clone());
        }
    }
    for face in analyze_cards(&pending, lexicon, grammar, args)? {
        previous.insert(face.id.clone(), face);
    }
    let mut faces = Vec::with_capacity(corpus.faces.len());
    for face in &corpus.faces {
        let report = previous
            .remove(&face.identity)
            .context("incremental face missing")?;
        faces.push(report);
    }
    mode.used = true;
    Ok((faces, mode))
}
