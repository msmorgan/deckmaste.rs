use std::collections::BTreeMap;
use std::fmt::Write as _;
use std::fs;
use std::path::Path;

use anyhow::Context;
use anyhow::ensure;
use deckmaste_construction_core::macro_def::Onset;
use deckmaste_data::mtgjson::AtomicCards;
use deckmaste_english_v2::context::ParseContext;
use rayon::prelude::*;
use sha2::Digest;
use sha2::Sha256;

const ID_DOMAIN: &[u8] = b"deckmaste:english-v2:corpus-unit:v1";
const MAX_CORPUS_UNIT_JOBS: usize = 4;
const RULES_BEARING_PARENTHETICALS: &[&str] = &[
    "(as long as this creature is on the battlefield)",
    "(even if this card isn't on the battlefield)",
    "(front face up)",
    "(if it's still on the battlefield)",
    "(or {1})",
];
const REMINDER_MID_LINE_PARENTHETICALS: &[&str] = &[
    "(For example, you may change \"black creatures can't attack\" to \"blue creatures can't attack.\")",
    "(a ticket counter)",
    "(an energy counter)",
    "(energy counter)",
    "(energy counters)",
    "(four energy counters)",
    "(the Fridge)",
    "(three energy counters)",
    "(two energy counters)",
];

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
                let text = normalize_oracle_text(
                    &card_name,
                    card.text.as_deref().unwrap_or_default(),
                )?;
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
    if jobs == 1 {
        return units
            .iter()
            .enumerate()
            .map(|(index, unit)| map(index, unit))
            .collect();
    }
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

fn normalize_oracle_text(card_name: &str, text: &str) -> anyhow::Result<String> {
    let typography =
        normalize_roll_row_dashes(&deckmaste_data::academyruins::normalize_quotes(text));
    strip_reminder_text(card_name, &typography)
}

/// Removes nonempty, single-line reminder parentheticals while retaining at
/// most one surrounding space. A line containing only reminder text
/// disappears. Every mid-line parenthetical must be explicitly classified;
/// authored rules-bearing parentheticals survive byte-exactly.
fn strip_reminder_text(card_name: &str, text: &str) -> anyhow::Result<String> {
    text.split('\n')
        .map(|line| {
            strip_reminder_text_line(card_name, line).map(|stripped| {
                (!stripped.trim().is_empty() || line.is_empty()).then_some(stripped)
            })
        })
        .collect::<anyhow::Result<Vec<_>>>()
        .map(|lines| lines.into_iter().flatten().collect::<Vec<_>>().join("\n"))
}

fn strip_reminder_text_line(card_name: &str, line: &str) -> anyhow::Result<String> {
    assert_no_nested_parentheses(line);

    let mut stripped = String::with_capacity(line.len());
    let mut remainder = line;
    while let Some(open) = remainder.find('(') {
        stripped.push_str(&remainder[..open]);
        let Some(relative_close) = remainder[open + 1..].find(')') else {
            stripped.push_str(&remainder[open..]);
            return Ok(stripped);
        };
        let close = open + 1 + relative_close;
        let parenthetical = &remainder[open..=close];
        let after = &remainder[close + 1..];

        if RULES_BEARING_PARENTHETICALS.contains(&parenthetical) {
            stripped.push_str(parenthetical);
            remainder = after;
            continue;
        }
        let original_prefix_len = line.len() - remainder.len() + open;
        let is_mid_line =
            !line[..original_prefix_len].trim().is_empty() && !after.trim().is_empty();
        ensure!(
            !is_mid_line || REMINDER_MID_LINE_PARENTHETICALS.contains(&parenthetical),
            "unknown mid-line parenthetical {parenthetical:?} while normalizing card {card_name:?}",
        );

        if relative_close == 0 {
            stripped.push_str(parenthetical);
            remainder = after;
            continue;
        }

        if stripped.ends_with(' ') {
            stripped.pop();
        }
        let (after, had_space_after) = after
            .strip_prefix(' ')
            .map_or((after, false), |after| (after, true));
        if !stripped.is_empty() && !after.is_empty() && had_space_after {
            stripped.push(' ');
        }
        remainder = after;
    }
    stripped.push_str(remainder);
    Ok(stripped)
}

fn assert_no_nested_parentheses(line: &str) {
    let mut open = false;
    for character in line.chars() {
        match character {
            '(' => {
                assert!(!open, "nested parenthetical in Oracle text line {line:?}");
                open = true;
            }
            ')' => open = false,
            _ => {}
        }
    }
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
            &normalize_oracle_text(card_name, text)
                .expect("test corpus fixture must satisfy normalization invariants"),
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
            &normalize_oracle_text(card_name, text)
                .expect("test corpus fixture must satisfy normalization invariants"),
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

    fn previous_strip_reminder_text(text: &str, preserve_rules_bearing: bool) -> String {
        text.split('\n')
            .filter_map(|line| {
                assert_no_nested_parentheses(line);
                let mut stripped = String::with_capacity(line.len());
                let mut remainder = line;
                while let Some(open) = remainder.find('(') {
                    stripped.push_str(&remainder[..open]);
                    let Some(relative_close) = remainder[open + 1..].find(')') else {
                        stripped.push_str(&remainder[open..]);
                        break;
                    };
                    let close = open + 1 + relative_close;
                    let parenthetical = &remainder[open..=close];
                    let after = &remainder[close + 1..];

                    if relative_close == 0
                        || (preserve_rules_bearing
                            && RULES_BEARING_PARENTHETICALS.contains(&parenthetical))
                    {
                        stripped.push_str(parenthetical);
                        remainder = after;
                        continue;
                    }

                    if stripped.ends_with(' ') {
                        stripped.pop();
                    }
                    let (after, had_space_after) = after
                        .strip_prefix(' ')
                        .map_or((after, false), |after| (after, true));
                    if !stripped.is_empty() && !after.is_empty() && had_space_after {
                        stripped.push(' ');
                    }
                    remainder = after;
                }
                stripped.push_str(remainder);
                (!stripped.trim().is_empty() || line.is_empty()).then_some(stripped)
            })
            .collect::<Vec<_>>()
            .join("\n")
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
    fn normalization_straightens_typography_and_strips_reminder_text() {
        let input = "‘Choose’.\n1—9 | Draw a card.\n2-10 | Get {E}{E} (two energy counters), then don’t strip.\nDeal 3-4 damage.\n[-2]: Act.";
        assert_eq!(
            normalize_oracle_text("Fixture", input).unwrap(),
            "'Choose'.\n1–9 | Draw a card.\n2–10 | Get {E}{E}, then don't strip.\nDeal 3-4 damage.\n[-2]: Act."
        );
    }

    #[test]
    fn normalization_preserves_non_reminder_document_structure() {
        let input = "Choose one —\n• Draw a card.\n• Create a token.\n(Fixed reminder.)";
        assert_eq!(
            normalize_oracle_text("Fixture", input).unwrap(),
            "Choose one —\n• Draw a card.\n• Create a token."
        );
    }

    #[test]
    fn reminder_stripping_pins_surrounding_space_and_malformed_input_contracts() {
        assert_eq!(
            strip_reminder_text(
                "Fixture",
                "Flying (This creature can't be blocked except by...)"
            )
            .unwrap(),
            "Flying"
        );
        assert_eq!(
            strip_reminder_text("Fixture", "(Reminder) Foo").unwrap(),
            "Foo"
        );
        assert_eq!(
            strip_reminder_text(
                "Fixture",
                "Get {E} (an energy counter), then {E}{E} (two energy counters)."
            )
            .unwrap(),
            "Get {E}, then {E}{E}."
        );
        assert_eq!(
            strip_reminder_text("Fixture", "Choose (perhaps").unwrap(),
            "Choose (perhaps"
        );
        assert_eq!(
            strip_reminder_text(
                "Fixture",
                "({R/P} can be paid with {R} or 2 life.)\nGain control."
            )
            .unwrap(),
            "Gain control."
        );
        assert_eq!(
            strip_reminder_text("Fixture", "(Reminder only.) ").unwrap(),
            ""
        );
        assert_eq!(
            strip_reminder_text("Fixture", "A (an energy counter) (two energy counters) d")
                .unwrap(),
            "A d"
        );
    }

    #[test]
    fn unknown_mid_line_parenthetical_is_a_card_named_error() {
        let error = strip_reminder_text("Tripwire Card", "Before (unknown spelling) after")
            .unwrap_err()
            .to_string();

        assert!(error.contains("Tripwire Card"), "{error}");
        assert!(error.contains("(unknown spelling)"), "{error}");
    }

    #[test]
    fn unknown_mid_line_parenthetical_after_a_leading_reminder_is_an_error() {
        let error = strip_reminder_text(
            "Tripwire Card",
            "(A leading reminder.) (a wholly novel gloss) trample.",
        )
        .unwrap_err()
        .to_string();

        assert!(error.contains("Tripwire Card"), "{error}");
        assert!(error.contains("(a wholly novel gloss)"), "{error}");
    }

    #[test]
    fn empty_mid_line_parenthetical_is_a_card_named_error() {
        let error = strip_reminder_text("Empty Group", "Before () after")
            .unwrap_err()
            .to_string();

        assert!(error.contains("Empty Group"), "{error}");
        assert!(error.contains("()"), "{error}");
    }

    #[test]
    fn rules_bearing_parentheticals_survive_byte_exactly() {
        for parenthetical in RULES_BEARING_PARENTHETICALS {
            for input in [
                format!("{parenthetical} after"),
                format!("Before {parenthetical} after"),
                format!("Before {parenthetical}"),
            ] {
                assert_eq!(strip_reminder_text("Fixture", &input).unwrap(), input);
            }
        }
    }

    #[test]
    fn mid_line_parenthetical_inventory_matches_the_vintage_snapshot() {
        const EXPECTED_RULES_BEARING_USES: [usize; 5] = [2, 1, 11, 2, 1];
        const EXPECTED_REMINDER_USES: [usize; 9] = [1, 2, 23, 1, 16, 7, 1, 21, 64];

        let bytes =
            deckmaste_data::mtgjson::atomic_cards_bytes().expect("reading AtomicCards snapshot");
        let cards = AtomicCards::parse(&bytes).expect("parsing AtomicCards snapshot");
        let vintage_cards = cards
            .data
            .values()
            .flatten()
            .filter(|card| card.vintage_playable())
            .collect::<Vec<_>>();

        for card in &vintage_cards {
            strip_reminder_text(card.name.as_str(), card.text.as_deref().unwrap_or_default())
                .unwrap_or_else(|error| panic!("{error}"));
        }

        let rules_bearing_counts = RULES_BEARING_PARENTHETICALS
            .iter()
            .map(|parenthetical| {
                vintage_cards
                    .iter()
                    .filter(|card| {
                        card.text
                            .as_deref()
                            .is_some_and(|text| text.contains(*parenthetical))
                    })
                    .count()
            })
            .collect::<Vec<_>>();
        let reminder_counts = REMINDER_MID_LINE_PARENTHETICALS
            .iter()
            .map(|parenthetical| {
                vintage_cards
                    .iter()
                    .filter(|card| {
                        card.text
                            .as_deref()
                            .is_some_and(|text| text.contains(*parenthetical))
                    })
                    .count()
            })
            .collect::<Vec<_>>();

        assert_eq!(rules_bearing_counts, EXPECTED_RULES_BEARING_USES);
        assert_eq!(rules_bearing_counts.iter().sum::<usize>(), 17);
        assert_eq!(reminder_counts, EXPECTED_REMINDER_USES);
        assert_eq!(reminder_counts.iter().sum::<usize>(), 136);
    }

    #[test]
    fn exhaustive_tripwire_preserves_every_current_normalized_identity() {
        let bytes =
            deckmaste_data::mtgjson::atomic_cards_bytes().expect("reading AtomicCards snapshot");
        let cards = AtomicCards::parse(&bytes).expect("parsing AtomicCards snapshot");
        let mut differences_from_raw_strip = 0;

        for card in cards
            .data
            .values()
            .flatten()
            .filter(|card| card.vintage_playable())
        {
            let raw = card.text.as_deref().unwrap_or_default();
            let typography =
                normalize_roll_row_dashes(&deckmaste_data::academyruins::normalize_quotes(raw));
            let normalized = normalize_oracle_text(card.name.as_str(), raw)
                .unwrap_or_else(|error| panic!("{error}"));
            let previous = previous_strip_reminder_text(&typography, true);
            let raw_strip = previous_strip_reminder_text(&typography, false);

            assert_eq!(
                normalized,
                previous,
                "normalization identity changed for {:?}",
                card.name.as_str()
            );
            differences_from_raw_strip += usize::from(normalized != raw_strip);
        }

        assert_eq!(differences_from_raw_strip, 17);
    }

    #[test]
    #[should_panic(expected = "nested parenthetical")]
    fn nested_parentheticals_are_rejected() {
        let _ = strip_reminder_text("Fixture", "Choose (an outer (nested) phrase).");
    }

    #[test]
    fn whole_text_reminder_normalizes_to_an_empty_document() {
        assert_eq!(
            normalize_oracle_text("Fixture", "(Basic land reminder text.)").unwrap(),
            ""
        );
    }

    #[test]
    fn normalized_basic_land_and_french_vanilla_probes_select() {
        let snapshot = br#"{"data":{
            "A.I.M. Bot":[{
                "name":"A.I.M. Bot", "layout":"normal", "types":["Creature"],
                "supertypes":[], "subtypes":[],
                "legalities":{"vintage":"Legal"},
                "text":"Flying (This creature can't be blocked except by creatures with flying or reach.)"
            }],
            "Plains":[{
                "name":"Plains", "layout":"normal", "types":["Land"],
                "supertypes":["Basic"], "subtypes":["Plains"],
                "legalities":{"vintage":"Legal"}, "text":"({T}: Add {W}.)"
            }]
        }}"#;
        let onsets = explicit_onsets([("A.I.M. Bot", Onset::Vowel), ("Plains", Onset::Consonant)]);
        let corpus = Corpus::from_bytes_with_context_onsets(snapshot, &onsets).unwrap();
        let parser =
            crate::english_v2::parser_from_builtin_v2().expect("probe grammar initializes");

        for (name, text) in [("A.I.M. Bot", "Flying"), ("Plains", "")] {
            let unit = corpus
                .units()
                .iter()
                .find(|unit| unit.card_name() == name)
                .expect("probe unit exists");
            assert_eq!(unit.text(), text);
            let context = ParseContext::new(
                unit.context_name(),
                unit.is_legendary(),
                unit.context_onset(),
            )
            .expect("probe context is valid");
            let analysis = parser.analyze_oracle_text(unit.text(), &context);
            assert!(analysis.selected().is_some(), "{name} selects");
            assert!(
                analysis
                    .ownership()
                    .expect("selected probe has ownership")
                    .summary()
                    .covered(),
                "{name} is totally owned",
            );
        }
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
