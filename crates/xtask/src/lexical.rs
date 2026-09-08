//! Raw supported-corpus lexical accounting, independent of construction
//! parsing.

use std::collections::BTreeMap;
use std::collections::BTreeSet;
use std::io::BufWriter;
use std::io::Write;
use std::num::NonZeroUsize;
use std::path::Path;
use std::path::PathBuf;
use std::time::Instant;

use anyhow::Context;
use anyhow::Result;
use anyhow::ensure;
use deckmaste_lexical::AnalyzedText;
use deckmaste_lexical::Category;
use deckmaste_lexical::LexicalProperties;
use deckmaste_lexical::LexicalReading;
use deckmaste_lexical::Lexicon;
use deckmaste_lexical::Source;
use deckmaste_lexical::SourceKind;
use rayon::prelude::*;
use serde::Serialize;

use crate::raw_corpus::CorpusSelectionArgs;
use crate::raw_corpus::SelectedCorpus;
use crate::raw_corpus::SelectedFace;
use crate::raw_corpus::SelectionRequest;
use crate::raw_corpus::SelectionStatus;
use crate::raw_corpus::digest;

#[derive(Debug, clap::Args)]
pub struct LexicalArgs {
    /// Pinned Scryfall Oracle Cards JSONL snapshot.
    #[arg(long, default_value = "data/scryfall/oracle-cards.jsonl")]
    data: PathBuf,
    #[command(flatten)]
    selection: CorpusSelectionArgs,
    /// Dedicated lexical-analysis worker threads.
    #[arg(long, default_value = "1")]
    workers: NonZeroUsize,
    /// Destination for raw source occurrences and lexical inventory accounting.
    #[arg(long)]
    output: PathBuf,
    /// Export the complete declared lexical inventory as an unstable RON
    /// inspection artifact; regenerate it after model-shape changes.
    #[arg(long)]
    export: Option<PathBuf>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Ord, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
enum Material {
    Ordinary,
    Catalog,
    Keyword,
    Numeral,
    Symbol,
    Affix,
    Clitic,
}

/// These inventories describe source material; they do not admit grammar.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Ord, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
enum RemainderClass {
    UnresolvedWord,
    StructuralSeparator,
    Notation,
    UnresolvedNonword,
}

fn nonword_class(ch: char) -> RemainderClass {
    if ch.is_whitespace()
        || matches!(
            ch,
            '.' | ','
                | ':'
                | ';'
                | '!'
                | '?'
                | '"'
                | '\''
                | '’'
                | '‘'
                | '“'
                | '”'
                | '('
                | ')'
                | '['
                | ']'
                | '—'
                | '–'
                | '•'
        )
    {
        RemainderClass::StructuralSeparator
    } else if matches!(
        ch,
        '{' | '}' | '+' | '-' | '−' | '/' | '=' | '|' | '¹' | '²' | '³' | '⁰' | '⁴' | '⁵'
    ) {
        RemainderClass::Notation
    } else {
        RemainderClass::UnresolvedNonword
    }
}

#[derive(Debug, Serialize)]
struct Span {
    start_byte: usize,
    end_byte: usize,
    text: String,
}

#[derive(Debug, Serialize)]
struct WordOccurrence {
    #[serde(flatten)]
    span: Span,
    /// All classes touching this word, including partial and overlapping
    /// matches.
    overlapping_classes: BTreeSet<Material>,
    /// Classes with a contiguous lexical path covering the complete word.
    fully_covered_classes: BTreeSet<Material>,
    /// No contiguous lexical path covers this word across all classes combined.
    unknown: bool,
}

#[derive(Debug, Default, Serialize)]
struct WordInventory {
    occurrences: usize,
    unknown_occurrences: usize,
    overlapping_occurrences: BTreeMap<Material, usize>,
    fully_covered_occurrences: BTreeMap<Material, usize>,
}

#[derive(Debug, Serialize)]
struct FaceReport {
    /// Durable Oracle identity plus the within-object face discriminator.
    id: String,
    oracle_id: String,
    printing_id: String,
    group_name: String,
    face_index: Option<usize>,
    name: String,
    face_name: Option<String>,
    side: Option<String>,
    raw_text: String,
    raw_text_sha256: String,
    lexical_matches: usize,
    words: Vec<WordOccurrence>,
    /// Non-whitespace, non-word material not covered by a declared reading.
    unrecognized_nonwords: Vec<Span>,
}

#[derive(Debug, Default, Serialize)]
struct Inventory {
    /// Lowercased spelling keys; exact casing remains in face occurrences.
    words: BTreeMap<String, WordInventory>,
    unknown_words: BTreeMap<String, usize>,
    unrecognized_nonwords: BTreeMap<String, usize>,
    /// Unique occurrence spans per class, independent of inflectional
    /// ambiguity.
    matched_surfaces: BTreeMap<Material, BTreeMap<String, usize>>,
    matched_readings: usize,
    word_occurrences: usize,
    unknown_word_occurrences: usize,
    /// Every unknown word and every nonlexical scalar, including whitespace,
    /// is retained by spelling in one of these classes.
    remainder_classes: BTreeMap<RemainderClass, BTreeMap<String, usize>>,
}

#[derive(Debug, Serialize)]
struct LexemeInventory {
    lemma: String,
    category: Category,
    properties: LexicalProperties,
    independent_values_checked: usize,
}

#[derive(Debug, Default, Serialize)]
struct Timings {
    load: f64,
    index: f64,
    value_law: f64,
    analysis: f64,
}

#[derive(Debug, Serialize)]
struct Report {
    schema: u32,
    input: PathBuf,
    input_sha256: String,
    selection: SelectionRequest,
    selection_status: SelectionStatus,
    analysis_status: &'static str,
    records_scanned: usize,
    supported_faces_examined: usize,
    workers: usize,
    support_filter: &'static str,
    normalization: &'static str,
    nonword_policy: &'static str,
    supported_faces: usize,
    supported_faces_without_text: usize,
    independent_values_checked: usize,
    lexeme_sources: BTreeMap<String, Source>,
    lexeme_inventory: BTreeMap<String, LexemeInventory>,
    /// Declared frame signatures belong to lexical identities; they are not
    /// stand-alone surface readings or constructed grammar values.
    declared_frame_signatures: usize,
    unmapped_sources: Vec<String>,
    #[serde(rename = "timings_seconds")]
    timings: Timings,
    inventory: Inventory,
    faces: Vec<FaceReport>,
}

pub fn run(args: &LexicalArgs, output: &mut dyn Write) -> Result<()> {
    let started = Instant::now();
    let corpus = args.selection.load(&args.data)?;
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
    let sources = deckmaste_lexical_source::load_workspace(&root)?;
    let load = started.elapsed().as_secs_f64();
    let started = Instant::now();
    let lexicon = Lexicon::new(sources.lexemes).context("indexing declared lexical inventory")?;
    let index = started.elapsed().as_secs_f64();
    let started = Instant::now();
    let independent_values_checked = check_independent_values(&lexicon)?;
    let value_law = started.elapsed().as_secs_f64();
    let started = Instant::now();
    let mut report = analyze_cards(&corpus.faces, &lexicon, args.workers.get())?;
    report.input.clone_from(&args.data);
    apply_corpus_metadata(&mut report, &corpus, args.workers.get());
    report.unmapped_sources = sources.unmapped;
    report.independent_values_checked = independent_values_checked;
    report.timings = Timings {
        load,
        index,
        value_law,
        analysis: started.elapsed().as_secs_f64(),
    };
    if let Some(path) = &args.export {
        let lexemes = lexicon.lexemes().values().cloned().collect::<Vec<_>>();
        let exported = ron::ser::to_string_pretty(&lexemes, ron::ser::PrettyConfig::default())?;
        std::fs::write(path, exported)
            .with_context(|| format!("writing lexical declarations to {}", path.display()))?;
    }
    let file = std::fs::File::create(&args.output)
        .with_context(|| format!("creating {}", args.output.display()))?;
    let mut writer = BufWriter::new(file);
    serde_json::to_writer(&mut writer, &report).context("writing lexical inventory JSON")?;
    writer.write_all(b"\n")?;
    writer.flush()?;
    writeln!(
        output,
        "Lexical inventory: {} supported faces, {} words ({} unique), {} unknown occurrences ({} unique); {} matched readings; {} independent values checked.",
        report.supported_faces,
        report.inventory.word_occurrences,
        report.inventory.words.len(),
        report.inventory.unknown_word_occurrences,
        report.inventory.unknown_words.len(),
        report.inventory.matched_readings,
        report.independent_values_checked
    )?;
    writeln!(
        output,
        "Seconds: load {:.3}, index {:.3}, value law {:.3}, analysis/accounting {:.3}. Artifact: {}",
        report.timings.load,
        report.timings.index,
        report.timings.value_law,
        report.timings.analysis,
        args.output.display()
    )?;
    Ok(())
}

fn check_independent_values(lexicon: &Lexicon) -> Result<usize> {
    let mut checked = 0;
    for value in lexicon.values() {
        let reading = LexicalReading::Word(value.clone());
        let surface = lexicon.realize(&reading)?;
        let analyzed = lexicon.analyze_source(&surface, None);
        ensure!(
            analyzed.matches.iter().any(|found| {
                found.start == 0 && found.end == analyzed.tokens.len() && found.reading == reading
            }),
            "lexical value/render/analyze law failed: {reading:?} -> {surface:?}"
        );
        checked += 1;
    }
    Ok(checked)
}

fn analyze_cards(faces: &[SelectedFace], lexicon: &Lexicon, workers: usize) -> Result<Report> {
    let mut report = Report {
        schema: 3,
        input: PathBuf::new(),
        input_sha256: String::new(),
        selection: SelectionRequest::default(),
        selection_status: SelectionStatus::Subset,
        analysis_status: "subset",
        records_scanned: 0,
        supported_faces_examined: 0,
        workers,
        support_filter: crate::raw_corpus::SUPPORT_FILTER,
        normalization: "none; raw Oracle text including reminders",
        nonword_policy: "only lexical matches count as lexical coverage; all remaining scalars and unknown words are classified by spelling as unresolved words, structural separators, notation, or unresolved nonwords; material classification does not license grammar",
        supported_faces: 0,
        supported_faces_without_text: 0,
        independent_values_checked: 0,
        lexeme_sources: lexicon
            .lexemes()
            .iter()
            .map(|(id, lexeme)| (id.clone(), lexeme.source.clone()))
            .collect(),
        lexeme_inventory: lexeme_inventory(lexicon),
        declared_frame_signatures: lexicon
            .lexemes()
            .values()
            .map(|lexeme| lexeme.properties.frames.len())
            .sum(),
        unmapped_sources: Vec::new(),
        timings: Timings::default(),
        inventory: Inventory::default(),
        faces: Vec::new(),
    };
    let pool = rayon::ThreadPoolBuilder::new()
        .num_threads(workers)
        .build()?;
    let analyzed_faces = pool.install(|| {
        faces
            .par_iter()
            .map(|face| analyze_face(face, lexicon))
            .collect::<Result<Vec<_>>>()
    })?;
    for analyzed in analyzed_faces {
        report.supported_faces += 1;
        let Some((face, inventory)) = analyzed else {
            report.supported_faces_without_text += 1;
            continue;
        };
        merge_inventory(&mut report.inventory, inventory);
        report.faces.push(face);
    }
    Ok(report)
}

fn analyze_face(face: &SelectedFace, lexicon: &Lexicon) -> Result<Option<(FaceReport, Inventory)>> {
    let Some(raw) = face.text.as_deref() else {
        return Ok(None);
    };
    let analyzed = lexicon.analyze_source(raw, None);
    let mut inventory = Inventory::default();
    let (words, unrecognized_nonwords) = account_text(&analyzed, lexicon, &mut inventory)
        .with_context(|| format!("lexical laws for {} ({})", face.name, face.identity))?;
    Ok(Some((
        FaceReport {
            id: face.identity.clone(),
            oracle_id: face.oracle_id.clone(),
            printing_id: face.printing_id.clone(),
            group_name: face.group_name.clone(),
            face_index: face.face_index,
            name: face.name.clone(),
            face_name: face.face_name.clone(),
            side: face.side.clone(),
            raw_text: raw.to_owned(),
            raw_text_sha256: digest(raw.as_bytes()),
            lexical_matches: analyzed.matches.len(),
            words,
            unrecognized_nonwords,
        },
        inventory,
    )))
}

fn apply_corpus_metadata(report: &mut Report, corpus: &SelectedCorpus, workers: usize) {
    report.input_sha256.clone_from(&corpus.snapshot_sha256);
    report.selection.clone_from(&corpus.selection);
    report.selection_status = corpus.selection_status;
    report.analysis_status = match corpus.selection_status {
        SelectionStatus::Complete => "complete",
        SelectionStatus::Subset => "subset",
    };
    report.records_scanned = corpus.records_scanned;
    report.supported_faces_examined = corpus.supported_faces_examined;
    report.workers = workers;
}

fn merge_inventory(target: &mut Inventory, source: Inventory) {
    merge_count_map(&mut target.unknown_words, source.unknown_words);
    merge_count_map(
        &mut target.unrecognized_nonwords,
        source.unrecognized_nonwords,
    );
    merge_nested_count_map(&mut target.matched_surfaces, source.matched_surfaces);
    merge_nested_count_map(&mut target.remainder_classes, source.remainder_classes);
    for (word, source_word) in source.words {
        let target_word = target.words.entry(word).or_default();
        target_word.occurrences += source_word.occurrences;
        target_word.unknown_occurrences += source_word.unknown_occurrences;
        merge_count_map(
            &mut target_word.overlapping_occurrences,
            source_word.overlapping_occurrences,
        );
        merge_count_map(
            &mut target_word.fully_covered_occurrences,
            source_word.fully_covered_occurrences,
        );
    }
    target.matched_readings += source.matched_readings;
    target.word_occurrences += source.word_occurrences;
    target.unknown_word_occurrences += source.unknown_word_occurrences;
}

fn merge_count_map<K: Ord>(target: &mut BTreeMap<K, usize>, source: BTreeMap<K, usize>) {
    for (key, count) in source {
        *target.entry(key).or_default() += count;
    }
}

fn merge_nested_count_map<K: Ord, J: Ord>(
    target: &mut BTreeMap<K, BTreeMap<J, usize>>,
    source: BTreeMap<K, BTreeMap<J, usize>>,
) {
    for (key, counts) in source {
        merge_count_map(target.entry(key).or_default(), counts);
    }
}

fn lexeme_inventory(lexicon: &Lexicon) -> BTreeMap<String, LexemeInventory> {
    let mut counts = BTreeMap::<&str, usize>::new();
    for value in lexicon.values() {
        *counts.entry(&value.lexeme).or_default() += 1;
    }
    lexicon
        .lexemes()
        .iter()
        .map(|(id, lexeme)| {
            (
                id.clone(),
                LexemeInventory {
                    lemma: lexeme.lemma.clone(),
                    category: lexeme.category,
                    properties: lexeme.properties.clone(),
                    independent_values_checked: counts
                        .get(id.as_str())
                        .copied()
                        .unwrap_or_default(),
                },
            )
        })
        .collect()
}

fn materials(lexicon: &Lexicon, reading: &LexicalReading) -> Result<BTreeSet<Material>> {
    let LexicalReading::Word(value) = reading else {
        return Ok(BTreeSet::from([Material::Numeral]));
    };
    let lexeme = lexicon
        .lexemes()
        .get(&value.lexeme)
        .with_context(|| format!("match references absent lexeme {}", value.lexeme))?;
    let primary = match lexeme.category {
        Category::Catalog => Material::Catalog,
        Category::Keyword => Material::Keyword,
        Category::Numeral => Material::Numeral,
        Category::Symbol => Material::Symbol,
        Category::Affix => Material::Affix,
        Category::Clitic => Material::Clitic,
        _ => Material::Ordinary,
    };
    let mut result = BTreeSet::from([primary]);
    match lexeme.source.kind {
        SourceKind::Catalog => {
            result.insert(Material::Catalog);
        }
        SourceKind::Keyword => {
            result.insert(Material::Keyword);
        }
        SourceKind::Symbol => {
            result.insert(Material::Symbol);
        }
        SourceKind::Core | SourceKind::Plugin => {}
    }
    Ok(result)
}

fn occurrence(analyzed: &AnalyzedText, start: usize, end: usize) -> Result<Span> {
    let bytes = analyzed
        .byte_range(start, end)
        .context("invalid lexical occurrence range")?;
    Ok(Span {
        start_byte: bytes.start,
        end_byte: bytes.end,
        text: analyzed.text()[bytes].to_owned(),
    })
}

fn cover_matches(
    analyzed: &AnalyzedText,
    lexicon: &Lexicon,
    inventory: &mut Inventory,
) -> Result<BTreeMap<Material, Vec<bool>>> {
    let mut coverage = BTreeMap::<Material, Vec<bool>>::new();
    let mut distinct_spans = BTreeSet::new();
    inventory.matched_readings += analyzed.matches.len();
    for found in &analyzed.matches {
        let span = occurrence(analyzed, found.start, found.end)?;
        let realized = lexicon.realize(&found.reading)?;
        ensure!(
            realized == span.text,
            "lexical analyze/render law failed at {}..{}: {:?} -> {realized:?}, expected {:?}",
            span.start_byte,
            span.end_byte,
            found.reading,
            span.text
        );
        for material in materials(lexicon, &found.reading)? {
            coverage
                .entry(material)
                .or_insert_with(|| vec![false; analyzed.tokens.len()])[found.start..found.end]
                .fill(true);
            if distinct_spans.insert((material, found.start, found.end)) {
                *inventory
                    .matched_surfaces
                    .entry(material)
                    .or_default()
                    .entry(span.text.clone())
                    .or_default() += 1;
            }
        }
    }
    Ok(coverage)
}

fn account_text(
    analyzed: &AnalyzedText,
    lexicon: &Lexicon,
    inventory: &mut Inventory,
) -> Result<(Vec<WordOccurrence>, Vec<Span>)> {
    let coverage = cover_matches(analyzed, lexicon, inventory)?;
    let unknown = analyzed
        .unknown_words()
        .into_iter()
        .map(|range| (range.start, range.end))
        .collect::<BTreeSet<_>>();
    let mut word_characters = vec![false; analyzed.tokens.len()];
    let mut words = Vec::new();
    for range in analyzed.words() {
        word_characters[range.clone()].fill(true);
        let overlapping_classes = coverage
            .iter()
            .filter(|(_, mask)| mask[range.clone()].iter().any(|x| *x))
            .map(|(material, _)| *material)
            .collect::<BTreeSet<_>>();
        let fully_covered_classes = coverage
            .iter()
            .filter(|(material, mask)| {
                mask[range.clone()].iter().all(|x| *x)
                    && analyzed.is_covered(range.clone(), |reading| {
                        materials(lexicon, reading).is_ok_and(|classes| classes.contains(material))
                    })
            })
            .map(|(material, _)| *material)
            .collect::<BTreeSet<_>>();
        let span = occurrence(analyzed, range.start, range.end)?;
        let key = span.text.to_lowercase();
        let is_unknown = unknown.contains(&(range.start, range.end));
        let entry = inventory.words.entry(key.clone()).or_default();
        entry.occurrences += 1;
        inventory.word_occurrences += 1;
        for material in &overlapping_classes {
            *entry.overlapping_occurrences.entry(*material).or_default() += 1;
        }
        for material in &fully_covered_classes {
            *entry
                .fully_covered_occurrences
                .entry(*material)
                .or_default() += 1;
        }
        if is_unknown {
            entry.unknown_occurrences += 1;
            inventory.unknown_word_occurrences += 1;
            *inventory
                .remainder_classes
                .entry(RemainderClass::UnresolvedWord)
                .or_default()
                .entry(key.clone())
                .or_default() += 1;
            *inventory.unknown_words.entry(key).or_default() += 1;
        }
        words.push(WordOccurrence {
            span,
            overlapping_classes,
            fully_covered_classes,
            unknown: is_unknown,
        });
    }
    for (index, ch) in analyzed.tokens.iter().enumerate() {
        if !word_characters[index] && !coverage.values().any(|mask| mask[index]) {
            *inventory
                .remainder_classes
                .entry(nonword_class(*ch))
                .or_default()
                .entry(ch.to_string())
                .or_default() += 1;
        }
    }
    let unrecognized = (0..analyzed.tokens.len())
        .map(|index| {
            !word_characters[index]
                && !analyzed.tokens[index].is_whitespace()
                && !coverage.values().any(|mask| mask[index])
        })
        .collect::<Vec<_>>();
    let mut nonwords = Vec::new();
    let mut start = 0;
    while start < unrecognized.len() {
        if !unrecognized[start] {
            start += 1;
            continue;
        }
        let mut end = start + 1;
        while end < unrecognized.len() && unrecognized[end] {
            end += 1;
        }
        let span = occurrence(analyzed, start, end)?;
        *inventory
            .unrecognized_nonwords
            .entry(span.text.clone())
            .or_default() += 1;
        nonwords.push(span);
        start = end;
    }
    Ok((words, nonwords))
}

#[cfg(test)]
mod tests {
    use deckmaste_lexical::Countability;
    use deckmaste_lexical::Lexeme;

    use super::*;

    fn source(kind: SourceKind, owner: &str) -> Source {
        Source {
            kind,
            owner: owner.to_owned(),
            path: "fixture.ron".to_owned(),
        }
    }

    fn lexicon() -> Lexicon {
        Lexicon::new([
            Lexeme::verb("draw", "draw", source(SourceKind::Core, "Draw")),
            Lexeme::noun(
                "card",
                "card",
                vec![Countability::Count],
                source(SourceKind::Core, "Card"),
            ),
            Lexeme::invariant(
                "name",
                "Draw two",
                Category::Catalog,
                source(SourceKind::Catalog, "Name"),
            ),
            Lexeme::invariant(
                "saga",
                "Urza’s Saga",
                Category::Catalog,
                source(SourceKind::Catalog, "Saga"),
            ),
            Lexeme::invariant(
                "flying",
                "flying",
                Category::Keyword,
                source(SourceKind::Keyword, "Flying"),
            ),
            Lexeme::invariant(
                "flying-adj",
                "flying",
                Category::Adjective,
                source(SourceKind::Core, "FlyingAdjective"),
            ),
            Lexeme::invariant(
                "tap",
                "{T}",
                Category::Symbol,
                source(SourceKind::Symbol, "Tap"),
            ),
        ])
        .unwrap()
    }

    #[test]
    fn accounting_preserves_overlap_reminders_unicode_and_unknown_symbols() {
        let lexicon = lexicon();
        let text = "Draw two cards. (mystery {QZ}) Urza’s Saga flying {T}.";
        let analyzed = lexicon.analyze_source(text, None);
        let mut inventory = Inventory::default();
        let (words, nonwords) = account_text(&analyzed, &lexicon, &mut inventory).unwrap();
        let word = |text: &str| words.iter().find(|word| word.span.text == text).unwrap();
        assert_eq!(
            word("Draw").fully_covered_classes,
            BTreeSet::from([Material::Ordinary, Material::Catalog])
        );
        assert_eq!(
            word("two").fully_covered_classes,
            BTreeSet::from([Material::Catalog, Material::Numeral])
        );
        assert_eq!(
            word("flying").fully_covered_classes,
            BTreeSet::from([Material::Ordinary, Material::Keyword])
        );
        assert!(word("mystery").unknown);
        assert!(word("QZ").unknown);
        assert!(!word("T").unknown);
        assert_eq!(
            inventory.unknown_words,
            BTreeMap::from([("mystery".to_owned(), 1), ("qz".to_owned(), 1)])
        );
        assert!(nonwords.iter().any(|span| span.text.contains('{')));
        assert!(nonwords.iter().any(|span| span.text.contains('.')));
        for word in &words {
            assert_eq!(
                &text[word.span.start_byte..word.span.end_byte],
                word.span.text
            );
        }
        let saga = word("Saga");
        assert_eq!(saga.span.start_byte, text.find("Saga").unwrap());
        assert!(
            saga.span.start_byte
                > analyzed
                    .words()
                    .iter()
                    .find(|range| {
                        analyzed.tokens[(*range).clone()].iter().collect::<String>() == "Saga"
                    })
                    .unwrap()
                    .start
        );
        assert!(check_independent_values(&lexicon).unwrap() > lexicon.lexemes().len());
    }

    #[test]
    fn supported_faces_remain_distinct_and_raw_reminders_are_not_removed() {
        let supported = serde_json::json!({
            "object":"card", "id":"printing-supported", "oracle_id":"oracle-supported",
            "name":"Supported", "layout":"transform", "color_identity":[],
            "legalities":{"vintage":"legal"},
            "card_faces":[
                {"name":"Front", "mana_cost":"", "type_line":"Creature", "oracle_text":"Draw two cards. (mystery)", "colors":[]},
                {"name":"Back", "mana_cost":"", "type_line":"Creature", "oracle_text":"Draw two cards. (mystery)", "colors":[]}
            ]
        });
        let unsupported = serde_json::json!({
            "object":"card", "id":"printing-unsupported", "oracle_id":"oracle-unsupported",
            "name":"Unsupported", "layout":"normal", "oracle_text":"unseen",
            "legalities":{"vintage":"not_legal"}
        });
        let input = tempfile::NamedTempFile::new().unwrap();
        std::fs::write(input.path(), format!("{supported}\n{unsupported}\n")).unwrap();
        let corpus = crate::raw_corpus::load_selected(
            input.path(),
            &SelectionRequest {
                all: true,
                ..SelectionRequest::default()
            },
            None,
        )
        .unwrap();
        let report = analyze_cards(&corpus.faces, &lexicon(), 2).unwrap();
        assert_eq!(report.supported_faces, 2);
        assert_eq!(report.faces.len(), 2);
        assert_ne!(report.faces[0].id, report.faces[1].id);
        assert_eq!(
            report.faces[0].raw_text_sha256,
            report.faces[1].raw_text_sha256
        );
        assert_eq!(report.faces[0].raw_text, "Draw two cards. (mystery)");
        assert_eq!(
            report.inventory.unknown_words,
            BTreeMap::from([("mystery".to_owned(), 2)])
        );
        assert_eq!(report.inventory.words["draw"].occurrences, 2);
        assert_eq!(report.faces[0].face_name.as_deref(), Some("Front"));
        assert_eq!(report.faces[1].face_name.as_deref(), Some("Back"));
    }

    #[test]
    fn worker_count_does_not_change_lexical_results() {
        let supported = serde_json::json!({
            "object":"card", "id":"printing-supported", "oracle_id":"oracle-supported",
            "name":"Supported", "layout":"transform", "color_identity":[],
            "legalities":{"vintage":"legal"},
            "card_faces":[
                {"name":"Front", "type_line":"Creature", "oracle_text":"Draw two cards.", "colors":[]},
                {"name":"Back", "type_line":"Creature", "oracle_text":"Flying (mystery).", "colors":[]}
            ]
        });
        let input = tempfile::NamedTempFile::new().unwrap();
        std::fs::write(input.path(), format!("{supported}\n")).unwrap();
        let corpus = crate::raw_corpus::load_selected(
            input.path(),
            &SelectionRequest {
                all: true,
                ..SelectionRequest::default()
            },
            None,
        )
        .unwrap();
        let lexicon = lexicon();

        let mut one =
            serde_json::to_value(analyze_cards(&corpus.faces, &lexicon, 1).unwrap()).unwrap();
        let mut many =
            serde_json::to_value(analyze_cards(&corpus.faces, &lexicon, 3).unwrap()).unwrap();
        for value in [&mut one, &mut many] {
            value.as_object_mut().unwrap().remove("workers");
            value.as_object_mut().unwrap().remove("timings_seconds");
        }
        assert_eq!(one, many);
    }

    #[test]
    fn accounting_rejects_a_reading_that_does_not_realize_its_source_occurrence() {
        let lexicon = lexicon();
        let mut analyzed = lexicon.analyze_source("Draw", None);
        analyzed.matches[0].reading = lexicon.analyze_source("cards", None).matches[0]
            .reading
            .clone();
        let error = account_text(&analyzed, &lexicon, &mut Inventory::default()).unwrap_err();
        assert!(
            error
                .to_string()
                .contains("lexical analyze/render law failed at 0..4")
        );
    }
    #[test]
    fn remainder_classes_account_for_every_nonlexical_scalar_without_licensing_it() {
        let lexicon = lexicon();
        let analyzed = lexicon.analyze("mystery {QZ}. ☃\n");
        let mut inventory = Inventory::default();
        account_text(&analyzed, &lexicon, &mut inventory).unwrap();
        assert_eq!(
            inventory.remainder_classes[&RemainderClass::UnresolvedWord],
            BTreeMap::from([("mystery".into(), 1), ("qz".into(), 1)])
        );
        assert_eq!(
            inventory.remainder_classes[&RemainderClass::Notation],
            BTreeMap::from([("{".into(), 1), ("}".into(), 1)])
        );
        assert_eq!(
            inventory.remainder_classes[&RemainderClass::StructuralSeparator],
            BTreeMap::from([(" ".into(), 2), (".".into(), 1), ("\n".into(), 1)])
        );
        assert_eq!(
            inventory.remainder_classes[&RemainderClass::UnresolvedNonword],
            BTreeMap::from([("☃".into(), 1)])
        );
        assert_eq!(inventory.unknown_word_occurrences, 2);
    }
}
