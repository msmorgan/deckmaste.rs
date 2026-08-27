use std::collections::BTreeMap;
use std::fmt::Write as _;
use std::fs;
use std::path::Path;

use anyhow::Context;
use anyhow::ensure;
use deckmaste_data::mtgjson::AtomicCards;
use deckmaste_english_v2::context::ParseContext;
use macro_ron::v2::Onset;
use rayon::prelude::*;
use sha2::Digest;
use sha2::Sha256;

const ID_DOMAIN: &[u8] = b"deckmaste:english-v2:corpus-unit:v1";
const MAX_CORPUS_UNIT_JOBS: usize = 4;

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize)]
pub(super) struct CorpusUnit {
    id: String,
    card_name: String,
    face_name: Option<String>,
    side: Option<String>,
    context_name: String,
    is_legendary: bool,
    context_onset: Onset,
    text: String,
}

impl CorpusUnit {
    pub(super) fn id(&self) -> &str {
        &self.id
    }

    pub(super) fn card_name(&self) -> &str {
        &self.card_name
    }

    pub(super) fn face_name(&self) -> Option<&str> {
        self.face_name.as_deref()
    }

    pub(super) fn side(&self) -> Option<&str> {
        self.side.as_deref()
    }

    pub(super) fn context_name(&self) -> &str {
        &self.context_name
    }

    pub(super) const fn is_legendary(&self) -> bool {
        self.is_legendary
    }

    pub(super) const fn context_onset(&self) -> Onset {
        self.context_onset
    }

    pub(super) fn text(&self) -> &str {
        &self.text
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) struct ValidatedCorpusId(String);

impl ValidatedCorpusId {
    pub(super) fn parse(id: &str) -> anyhow::Result<Self> {
        ensure!(
            id.len() == 64
                && id
                    .bytes()
                    .all(|byte| byte.is_ascii_digit() || (b'a'..=b'f').contains(&byte)),
            "invalid corpus ID {}; expected exactly 64 lowercase hexadecimal bytes",
            quoted(id),
        );
        Ok(Self(id.to_owned()))
    }

    pub(super) fn as_str(&self) -> &str {
        &self.0
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) struct Corpus {
    source_fingerprint: String,
    units: Vec<CorpusUnit>,
}

impl Corpus {
    pub(super) fn from_bytes_with_context_onsets(
        bytes: &[u8],
        context_onsets: &BTreeMap<String, Onset>,
    ) -> anyhow::Result<Self> {
        let cards = AtomicCards::parse(bytes).context("parsing MTGJSON atomic-card snapshot")?;
        let mut units = cards
            .data
            .values()
            .flat_map(|cards| cards.iter())
            .filter(|card| card.vintage_playable())
            .map(|card| -> anyhow::Result<_> {
                let card_name = card.name.to_string();
                let face_name = card.face_name.as_deref().map(str::to_owned);
                let side = card.side.as_deref().map(str::to_owned);
                let context_name = face_name.clone().unwrap_or_else(|| card_name.clone());
                let is_legendary = card
                    .supertypes
                    .iter()
                    .any(|supertype| supertype.as_str() == "Legendary");
                ensure!(
                    !context_name.is_empty(),
                    "invalid parser context: context name is empty; card name {card_name:?}, face name {face_name:?}, side {side:?}",
                );
                let context_onset = context_onsets.get(&context_name).copied().with_context(|| {
                    format!(
                        "missing explicit card-name onset metadata for opaque context {context_name:?}"
                    )
                })?;
                let text = normalize_oracle_text(card.text.as_deref().unwrap_or_default());
                Ok(corpus_unit(
                    &card_name,
                    face_name.as_deref(),
                    side.as_deref(),
                    &context_name,
                    is_legendary,
                    context_onset,
                    &text,
                ))
            })
            .collect::<anyhow::Result<Vec<_>>>()?;
        units.sort_by(|left, right| corpus_sort_key(left).cmp(&corpus_sort_key(right)));
        validate_contexts(&units)?;

        Ok(Self {
            source_fingerprint: sha256_hex(&Sha256::digest(bytes)),
            units,
        })
    }

    pub(super) fn load(path: &Path) -> anyhow::Result<Self> {
        let bytes = fs::read(path).with_context(|| format!("reading {}", path.display()))?;
        let catalog_root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../data/gen/catalogs");
        let adapted = super::adapt_card_name_catalog_provider(&catalog_root)?;
        Self::from_bytes_with_context_onsets(&bytes, &adapted.context_onsets)
    }

    pub(super) fn source_fingerprint(&self) -> &str {
        &self.source_fingerprint
    }

    pub(super) fn units(&self) -> &[CorpusUnit] {
        &self.units
    }

    pub(super) fn resolve_exact(&self, id: &ValidatedCorpusId) -> anyhow::Result<&CorpusUnit> {
        let mut matches = self.units.iter().filter(|unit| unit.id() == id.as_str());
        let Some(unit) = matches.next() else {
            anyhow::bail!("no corpus unit has exact ID {}", quoted(id.as_str()));
        };
        ensure!(
            matches.next().is_none(),
            "multiple corpus units have exact ID {}",
            quoted(id.as_str()),
        );
        Ok(unit)
    }
}

pub(super) fn map_corpus_units<T: Send>(
    units: &[CorpusUnit],
    map: impl Fn(usize, &CorpusUnit) -> T + Send + Sync,
) -> Vec<T> {
    let available = std::thread::available_parallelism().map_or(1, usize::from);
    map_corpus_units_with_workers(units, available, map)
}

fn map_corpus_units_with_workers<T: Send>(
    units: &[CorpusUnit],
    requested_workers: usize,
    map: impl Fn(usize, &CorpusUnit) -> T + Send + Sync,
) -> Vec<T> {
    if units.is_empty() {
        return Vec::new();
    }
    let jobs = corpus_unit_jobs(requested_workers, units.len());
    rayon::ThreadPoolBuilder::new()
        .num_threads(jobs)
        .thread_name(|index| format!("english-v2-corpus-{index}"))
        .build()
        .expect("bounded English-v2 corpus pool must build")
        .install(|| {
            units
                .par_iter()
                .enumerate()
                .map(|(index, unit)| map(index, unit))
                .collect()
        })
}

fn corpus_unit_jobs(requested_workers: usize, units: usize) -> usize {
    requested_workers
        .min(MAX_CORPUS_UNIT_JOBS)
        .min(units)
        .max(1)
}

fn quoted(value: &str) -> String {
    serde_json::to_string(value).expect("serializing a string cannot fail")
}

fn validate_contexts(units: &[CorpusUnit]) -> anyhow::Result<()> {
    for unit in units {
        if ParseContext::new(
            unit.context_name(),
            unit.is_legendary(),
            unit.context_onset(),
        )
        .is_some()
        {
            continue;
        }
        anyhow::bail!(
            "invalid parser context: context name is empty; card name {:?}, face name {:?}, side {:?}",
            unit.card_name(),
            unit.face_name(),
            unit.side(),
        );
    }
    Ok(())
}

fn corpus_unit(
    card_name: &str,
    face_name: Option<&str>,
    side: Option<&str>,
    context_name: &str,
    is_legendary: bool,
    context_onset: Onset,
    text: &str,
) -> CorpusUnit {
    let mut hasher = Sha256::new();
    hasher.update(ID_DOMAIN);
    for field in [
        card_name,
        face_name.unwrap_or_default(),
        side.unwrap_or_default(),
        context_name,
        text,
    ] {
        hasher.update((field.len() as u64).to_be_bytes());
        hasher.update(field.as_bytes());
    }

    CorpusUnit {
        id: sha256_hex(&hasher.finalize()),
        card_name: card_name.to_owned(),
        face_name: face_name.map(str::to_owned),
        side: side.map(str::to_owned),
        context_name: context_name.to_owned(),
        is_legendary,
        context_onset,
        text: text.to_owned(),
    }
}

fn sha256_hex(digest: &[u8]) -> String {
    let mut hexadecimal = String::with_capacity(digest.len() * 2);
    for byte in digest {
        write!(&mut hexadecimal, "{byte:02x}").expect("writing to String cannot fail");
    }
    hexadecimal
}

fn corpus_sort_key(unit: &CorpusUnit) -> (&str, Option<&str>, Option<&str>, &str, &str, &str) {
    (
        &unit.card_name,
        unit.face_name.as_deref(),
        unit.side.as_deref(),
        &unit.context_name,
        &unit.text,
        &unit.id,
    )
}

fn normalize_oracle_text(text: &str) -> String {
    normalize_roll_row_dashes(&deckmaste_data::academyruins::normalize_quotes(text))
}

fn normalize_roll_row_dashes(text: &str) -> String {
    text.split('\n')
        .map(normalize_roll_row_line)
        .collect::<Vec<_>>()
        .join("\n")
}

fn normalize_roll_row_line(line: &str) -> String {
    let low_end = line
        .find(|character: char| !character.is_ascii_digit())
        .unwrap_or(line.len());
    if low_end == 0 {
        return line.to_owned();
    }
    let Some(dash) = line[low_end..].chars().next() else {
        return line.to_owned();
    };
    if !matches!(dash, '-' | '—') {
        return line.to_owned();
    }
    let high_start = low_end + dash.len_utf8();
    let high_len = line[high_start..]
        .find(|character: char| !character.is_ascii_digit())
        .unwrap_or(line.len() - high_start);
    if high_len == 0 || !line[high_start + high_len..].starts_with(" |") {
        return line.to_owned();
    }
    format!("{}–{}", &line[..low_end], &line[high_start..])
}

#[cfg(test)]
impl CorpusUnit {
    pub(super) fn for_test(card_name: &str, text: &str) -> Self {
        let context_onset = if card_name.is_empty() {
            // The test-only invalid-context constructor rejects this unit
            // before the realization fact can be observed.
            Onset::Consonant
        } else {
            super::catalog_surface_onset(card_name)
                .expect("test corpus fixture must use a name with known onset")
        };
        corpus_unit(
            card_name,
            None,
            None,
            card_name,
            false,
            context_onset,
            &normalize_oracle_text(text),
        )
    }

    pub(super) fn for_test_with_metadata(
        card_name: &str,
        face_name: Option<&str>,
        side: Option<&str>,
        context_name: &str,
        text: &str,
    ) -> Self {
        corpus_unit(
            card_name,
            face_name,
            side,
            context_name,
            false,
            super::catalog_surface_onset(context_name)
                .expect("test corpus fixture must use a context name with known onset"),
            &normalize_oracle_text(text),
        )
    }
}

#[cfg(test)]
impl Corpus {
    pub(super) fn from_units_for_test(mut units: Vec<CorpusUnit>) -> Self {
        units.sort_by(|left, right| corpus_sort_key(left).cmp(&corpus_sort_key(right)));
        validate_contexts(&units).expect("test corpus must contain valid parser contexts");
        Self {
            source_fingerprint: "0".repeat(64),
            units,
        }
    }
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeMap;
    use std::sync::Arc;
    use std::sync::Barrier;
    use std::sync::Mutex;
    use std::sync::atomic::AtomicUsize;
    use std::sync::atomic::Ordering;
    use std::sync::mpsc::sync_channel;
    use std::time::Duration;

    use super::*;

    fn explicit_onsets(
        rows: impl IntoIterator<Item = (&'static str, Onset)>,
    ) -> BTreeMap<String, Onset> {
        rows.into_iter()
            .map(|(name, onset)| (name.to_owned(), onset))
            .collect()
    }

    #[test]
    fn corpus_unit_map_overlaps_work_and_preserves_source_order() {
        let units = [
            CorpusUnit::for_test("First", ""),
            CorpusUnit::for_test("Second", ""),
        ];
        let (sender, receiver) = sync_channel(1);
        let receiver = Mutex::new(receiver);

        let results =
            map_corpus_units_with_workers(&units, 2, |index, unit| match unit.card_name() {
                "First" => format!(
                    "{index}:{}",
                    receiver
                        .lock()
                        .unwrap()
                        .recv_timeout(Duration::from_secs(1))
                        .unwrap()
                ),
                "Second" => {
                    sender.send("released").unwrap();
                    format!("{index}:second")
                }
                name => panic!("unexpected mapped corpus unit {name}"),
            });

        assert_eq!(results, ["0:released", "1:second"]);
    }

    #[test]
    fn corpus_unit_map_caps_parser_concurrency() {
        let units = (0..12)
            .map(|index| CorpusUnit::for_test(&format!("Unit {index:02}"), ""))
            .collect::<Vec<_>>();
        let active = Arc::new(AtomicUsize::new(0));
        let peak = Arc::new(AtomicUsize::new(0));
        let cohort = Arc::new(Barrier::new(MAX_CORPUS_UNIT_JOBS));

        let results = map_corpus_units_with_workers(&units, 64, {
            let active = Arc::clone(&active);
            let peak = Arc::clone(&peak);
            let cohort = Arc::clone(&cohort);
            move |index, _unit| {
                let now = active.fetch_add(1, Ordering::SeqCst) + 1;
                peak.fetch_max(now, Ordering::SeqCst);
                cohort.wait();
                active.fetch_sub(1, Ordering::SeqCst);
                index
            }
        });

        assert_eq!(results, (0..units.len()).collect::<Vec<_>>());
        assert_eq!(peak.load(Ordering::SeqCst), MAX_CORPUS_UNIT_JOBS);
        assert!(peak.load(Ordering::SeqCst) <= 4);
    }

    fn snapshot_onsets() -> BTreeMap<String, Onset> {
        explicit_onsets([
            ("Alpha", Onset::Vowel),
            ("Empty", Onset::Vowel),
            ("Front", Onset::Consonant),
            ("Restricted", Onset::Consonant),
        ])
    }

    const SNAPSHOT_A: &[u8] = br#"{
        "data": {
            "Restricted": [{
                "name": "Restricted", "layout": "normal", "types": ["Creature"],
                "supertypes": [], "subtypes": [], "legalities": {"vintage": "Restricted"},
                "text": "Restricted text."
            }],
            "Front // Back": [{
                "name": "Front // Back", "faceName": "Front", "side": "a",
                "layout": "modal_dfc", "types": ["Creature"], "supertypes": [],
                "subtypes": [], "legalities": {"vintage": "Legal"},
                "text": "1\u20142 | Choose one."
            }, {
                "name": "Front // Back", "faceName": "Back", "side": "b",
                "layout": "modal_dfc", "types": ["Creature"], "supertypes": [],
                "subtypes": [], "legalities": {"vintage": "Banned"},
                "text": "Banned back."
            }],
            "Alpha": [{
                "name": "Alpha", "layout": "normal", "types": ["Creature"],
                "supertypes": [], "subtypes": [], "legalities": {"vintage": "Legal"},
                "text": "\u2018Alpha\u2019"
            }],
            "Banned": [{
                "name": "Banned", "layout": "normal", "types": ["Creature"],
                "supertypes": [], "subtypes": [], "legalities": {"vintage": "Banned"},
                "text": "Banned text."
            }],
            "Empty": [{
                "name": "Empty", "layout": "normal", "types": ["Creature"],
                "supertypes": [], "subtypes": [], "legalities": {"vintage": "Legal"}
            }]
        }
    }"#;

    const SNAPSHOT_B: &[u8] = br#"{
        "data": {
            "Empty": [{
                "name": "Empty", "layout": "normal", "types": ["Creature"],
                "supertypes": [], "subtypes": [], "legalities": {"vintage": "Legal"}
            }],
            "Banned": [{
                "name": "Banned", "layout": "normal", "types": ["Creature"],
                "supertypes": [], "subtypes": [], "legalities": {"vintage": "Banned"},
                "text": "Banned text."
            }],
            "Alpha": [{
                "name": "Alpha", "layout": "normal", "types": ["Creature"],
                "supertypes": [], "subtypes": [], "legalities": {"vintage": "Legal"},
                "text": "\u2018Alpha\u2019"
            }],
            "Front // Back": [{
                "name": "Front // Back", "faceName": "Back", "side": "b",
                "layout": "modal_dfc", "types": ["Creature"], "supertypes": [],
                "subtypes": [], "legalities": {"vintage": "Banned"},
                "text": "Banned back."
            }, {
                "name": "Front // Back", "faceName": "Front", "side": "a",
                "layout": "modal_dfc", "types": ["Creature"], "supertypes": [],
                "subtypes": [], "legalities": {"vintage": "Legal"},
                "text": "1\u20142 | Choose one."
            }],
            "Restricted": [{
                "name": "Restricted", "layout": "normal", "types": ["Creature"],
                "supertypes": [], "subtypes": [], "legalities": {"vintage": "Restricted"},
                "text": "Restricted text."
            }]
        }
    }"#;

    #[test]
    fn normalization_changes_only_quotes_and_roll_row_range_dashes() {
        let input = "‘Choose’.\n1—9 | Draw a card.\n2-10 | Don’t strip (reminder text.)\nDeal 3-4 damage.\n[-2]: Act.";
        assert_eq!(
            normalize_oracle_text(input),
            "'Choose'.\n1–9 | Draw a card.\n2–10 | Don't strip (reminder text.)\nDeal 3-4 damage.\n[-2]: Act."
        );
    }

    #[test]
    fn normalization_preserves_complete_document_structure() {
        let input = "Choose one —\n• Draw a card.\n• Create a token.\n(Fixed reminder.)";
        assert_eq!(normalize_oracle_text(input), input);
    }

    #[test]
    fn corpus_filters_normalizes_and_sorts_equivalent_snapshots() {
        let onsets = snapshot_onsets();
        let left = Corpus::from_bytes_with_context_onsets(SNAPSHOT_A, &onsets).unwrap();
        let right = Corpus::from_bytes_with_context_onsets(SNAPSHOT_B, &onsets).unwrap();
        assert_eq!(left.units(), right.units());
        assert_eq!(left.units().len(), 4);
        assert_eq!(
            left.units()
                .iter()
                .map(CorpusUnit::context_name)
                .collect::<Vec<_>>(),
            ["Alpha", "Empty", "Front", "Restricted"]
        );
        assert!(left.units().iter().all(|unit| unit.id().len() == 64));
        assert_eq!(left.units()[1].text(), "");

        let front = &left.units()[2];
        assert_eq!(front.card_name(), "Front // Back");
        assert_eq!(front.face_name(), Some("Front"));
        assert_eq!(front.side.as_deref(), Some("a"));
        assert_eq!(front.text(), "1–2 | Choose one.");
    }

    #[test]
    fn corpus_threads_authoritative_legendary_face_metadata() {
        let snapshot = br#"{"data":{
            "Aang, A Lot to Learn":[{
                "name":"Aang, A Lot to Learn", "layout":"normal",
                "types":["Creature"], "supertypes":["Legendary"], "subtypes":[],
                "legalities":{"vintage":"Legal"}, "text":"Aang gains 2 life."
            }],
            "Fear, Fire, Foes!":[{
                "name":"Fear, Fire, Foes!", "layout":"normal",
                "types":["Sorcery"], "supertypes":[], "subtypes":[],
                "legalities":{"vintage":"Legal"}, "text":"Fear, Fire, Foes! deals 1 damage to any target."
            }]
        }}"#;
        let onsets = explicit_onsets([
            ("Aang, A Lot to Learn", Onset::Vowel),
            ("Fear, Fire, Foes!", Onset::Consonant),
        ]);
        let corpus = Corpus::from_bytes_with_context_onsets(snapshot, &onsets)
            .expect("metadata fixture loads");
        let legendary = corpus
            .units()
            .iter()
            .find(|unit| unit.context_name() == "Aang, A Lot to Learn")
            .expect("legendary row exists");
        let ordinary = corpus
            .units()
            .iter()
            .find(|unit| unit.context_name() == "Fear, Fire, Foes!")
            .expect("nonlegendary row exists");
        assert!(legendary.is_legendary());
        assert!(!ordinary.is_legendary());
    }

    #[test]
    fn corpus_retains_an_explicit_empty_text_face() {
        let snapshot = br#"{"data":{"Explicit Empty":[{"name":"Explicit Empty","layout":"normal","types":["Creature"],"supertypes":[],"subtypes":[],"legalities":{"vintage":"Legal"},"text":""}]}}"#;

        let onsets = explicit_onsets([("Explicit Empty", Onset::Vowel)]);
        let corpus = Corpus::from_bytes_with_context_onsets(snapshot, &onsets).unwrap();
        assert_eq!(corpus.units().len(), 1);
        assert_eq!(corpus.units()[0].context_name(), "Explicit Empty");
        assert_eq!(corpus.units()[0].text(), "");
    }

    #[test]
    fn corpus_rejects_only_empty_parse_contexts() {
        let snapshot = br#"{"data":{"Fixture":[{"name":"","layout":"normal","types":["Creature"],"supertypes":[],"subtypes":[],"legalities":{"vintage":"Legal"},"text":"Fixture text."}]}}"#;

        let error = Corpus::from_bytes_with_context_onsets(snapshot, &BTreeMap::new())
            .expect_err("empty parser context must reject the MTGJSON source")
            .to_string();

        assert!(error.contains("invalid parser context"));
        assert!(error.contains("context name is empty"));
        assert!(error.contains("card name \"\""));
    }

    #[test]
    fn corpus_admits_opaque_nonempty_names_with_explicit_onset_metadata() {
        let snapshot = br#"{"data":{
            ", Invalid":[{
                "name":", Invalid", "layout":"normal", "types":["Creature"],
                "supertypes":[], "subtypes":[],
                "legalities":{"vintage":"Legal"}, "text":", Invalid deals 1 damage to any target."
            }],
            "+2 Mace":[{
                "name":"+2 Mace", "layout":"normal", "types":["Artifact"],
                "supertypes":[], "subtypes":["Equipment"],
                "legalities":{"vintage":"Legal"}, "text":"Equipped creature gets +2/+2."
            }]
        }}"#;
        let onsets = explicit_onsets([(", Invalid", Onset::Vowel), ("+2 Mace", Onset::Consonant)]);

        let corpus = Corpus::from_bytes_with_context_onsets(snapshot, &onsets)
            .expect("nonempty opaque names with explicit realization metadata load");

        assert_eq!(corpus.units().len(), 2);
        assert_eq!(corpus.units()[0].context_name(), "+2 Mace");
        assert_eq!(corpus.units()[0].context_onset(), Onset::Consonant);
        assert_eq!(corpus.units()[1].context_name(), ", Invalid");
        assert_eq!(corpus.units()[1].context_onset(), Onset::Vowel);
    }

    #[test]
    fn corpus_reports_missing_realization_metadata_without_parsing_the_name() {
        let snapshot = br#"{"data":{"! Unknown":[{
            "name":"! Unknown", "layout":"normal", "types":["Creature"],
            "supertypes":[], "subtypes":[], "legalities":{"vintage":"Legal"},
            "text":"! Unknown deals 1 damage to any target."
        }]}}"#;

        let error = Corpus::from_bytes_with_context_onsets(snapshot, &BTreeMap::new())
            .expect_err("the separate generated onset fact is required")
            .to_string();

        assert!(error.contains("missing explicit card-name onset metadata"));
        assert!(error.contains("! Unknown"));
        assert!(!error.contains("normalizing"));
    }

    #[test]
    #[should_panic(expected = "invalid parser context")]
    fn test_corpus_constructor_rejects_invalid_contexts() {
        let _ = Corpus::from_units_for_test(vec![CorpusUnit::for_test("", "Fixture text.")]);
    }
}
