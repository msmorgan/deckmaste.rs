use std::collections::BTreeMap;
use std::collections::BTreeSet;

use anyhow::Context;
use anyhow::Result;
use anyhow::ensure;
use deckmaste_english_v3::Decomposition;
use deckmaste_english_v3::Grammar as _;
use deckmaste_english_v3::GrammarForest;
use deckmaste_english_v3::Symbol;
use deckmaste_english_v3::grammar::Category;
use deckmaste_english_v3::grammar::Grammar;
use deckmaste_english_v3::grammar::RealizationContext;
use deckmaste_english_v3::grammar::Summary;
use deckmaste_english_v3::grammar::Value;
use deckmaste_lexical::AnalyzedText;
use deckmaste_lexical::Lexicon;
use serde::Deserialize;
use serde::Serialize;

use super::EnglishV3Args;
use super::report::Enumeration;
use super::report::FaceReport;
use super::report::LineReport;
use super::sample_line_reading;
use super::validation::Issue;
use super::validation::TracedValue;
use super::validation::Tracing;
use super::validation::validate_in_context;
use crate::raw_corpus::digest;

#[derive(Debug, Deserialize, Serialize)]
pub(super) struct Bridge {
    pub span: (usize, usize),
    pub category: String,
    pub factorized: bool,
    pub factors: Vec<usize>,
    pub old_count: usize,
    pub old_digest: String,
    pub reconstructed_count: usize,
    pub reconstructed_digest: String,
}

struct UnitValues {
    representatives: BTreeMap<Summary, TracedValue>,
    all: Vec<TracedValue>,
}

struct UnitEnumeration<'a, 'ctx> {
    forest: &'a GrammarForest<Grammar>,
    analyzed: &'a AnalyzedText,
    grammar: &'a Grammar,
    lexicon: &'a Lexicon,
    args: &'a EnglishV3Args,
    context: &'a RealizationContext<'ctx>,
}

impl UnitEnumeration<'_, '_> {
    fn plan(
        &self,
        nodes: &[deckmaste_english_v3::CompletedId],
    ) -> Result<(Decomposition<Category>, bool)> {
        let Self {
            forest,
            analyzed,
            grammar,
            ..
        } = *self;
        let mut plan = forest
            .decompose(
                nodes,
                &|c, productions| {
                    if matches!(
                        c,
                        Category::Sentence | Category::Cost | Category::Clause | Category::TypeLine
                    ) {
                        return false;
                    }
                    if *c == Category::Document && analyzed.tokens.is_empty() {
                        return false;
                    }
                    // A keyword line, including supplementation, remains one
                    // Ability.
                    if *c == Category::Ability
                        && productions.iter().any(|&id| {
                            grammar.productions()[id].symbols.iter().any(|symbol| {
                                matches!(symbol, Symbol::Nonterminal(Category::KeywordPhrase))
                            })
                        })
                    {
                        return false;
                    }
                    true
                },
                &|category| can_retain(*category),
            )
            .map_err(|error| anyhow::anyhow!(error))?
            .context("nonempty contextual node set")?;
        // A compiler sequence is not a Reading. If an alternative cannot be
        // factored at that internal position, retain its enclosing Ability.
        let fallback = plan.units().iter().any(|unit| !can_retain(unit.category));
        if fallback || plan.units().is_empty() {
            plan = forest
                .decompose(nodes, &|_, _| false, &|category| can_retain(*category))?
                .context("whole region")?;
        }
        Ok((plan, fallback))
    }

    fn enumerate(
        &self,
        unit: &deckmaste_english_v3::EnumerationUnit<Category>,
        result: &mut FaceReport,
        fallback: bool,
    ) -> Result<(LineReport, UnitValues)> {
        let Self {
            forest,
            analyzed,
            grammar,
            lexicon,
            args,
            context,
        } = *self;
        let bytes = analyzed
            .byte_range(unit.span.start, unit.span.end)
            .context("invalid unit coordinates")?;
        let mut report = LineReport {
            span: (bytes.start, bytes.end),
            category: format!("{:?}", unit.category),
            retained_reason: unit
                .retained_reason
                .map(str::to_owned)
                .or_else(|| fallback.then(|| "factorization-unproved".into())),
            proposed_parts: unit
                .proposed_parts
                .iter()
                .map(|(category, span)| {
                    let bytes = analyzed
                        .byte_range(span.start, span.end)
                        .expect("valid chart span");
                    (format!("{category:?}"), (bytes.start, bytes.end))
                })
                .collect(),
            text_sha256: digest(result.analyzed_source[bytes.clone()].as_bytes()),
            checked_readings: 0,
            readings_sha256: None,
            enumeration: Enumeration::Complete,
            nominal_normal_forms: None,
        };
        let mut readings = forest
            .readings_from(
                &unit.nodes.iter().copied().collect::<Vec<_>>(),
                Tracing::new(grammar),
            )
            .with_digest_dedup(TracedValue::dedup_key);
        let mut checked = BTreeSet::new();
        let mut normal_forms = BTreeSet::new();
        let mut representatives = BTreeMap::new();
        let mut all = Vec::new();
        let mut requested = 0;
        let issues_before = result.issues.len();
        loop {
            if args
                .reading_limit
                .is_some_and(|limit| requested >= limit.get())
            {
                report.enumeration = Enumeration::Limited;
                break;
            }
            let Some(value) = readings.next() else {
                break;
            };
            requested += 1;
            let value = match value {
                Ok(value) => value,
                Err(error) => {
                    result
                        .issues
                        .push(Issue::Materialization(error.to_string()));
                    continue;
                }
            };
            match validate_in_context(
                &value,
                &result.analyzed_source[bytes.clone()],
                unit.category,
                context,
            ) {
                Ok(reading) => {
                    let summary = reading.admit_with(grammar, lexicon)?;
                    ensure!(
                        unit.nodes
                            .iter()
                            .any(|&node| forest.summary(node) == Some(&summary)),
                        "unit admission summary changed"
                    );
                    if !checked.insert(value.dedup_key()) {
                        result.issues.push(Issue::DuplicateReading);
                        continue;
                    }
                    normal_forms.insert(super::nominal_normal_form::fingerprint(reading)?);
                    for node in value.nodes.iter() {
                        if let Value::Reading(node) = &**node {
                            *result
                                .constructions
                                .entry(node.construction().to_owned())
                                .or_default() += 1;
                        }
                    }
                    sample_line_reading(
                        reading,
                        &value,
                        lexicon,
                        &mut result.samples,
                        args.samples_per_face,
                    )?;
                    representatives
                        .entry(summary)
                        .or_insert_with(|| value.clone());
                    if args.migration_from.is_some() {
                        all.push(value);
                    }
                }
                Err(issue) => result.issues.push(issue),
            }
        }
        for (key, count) in super::report::reading_metrics(readings.metrics()) {
            *result.readings.entry(key).or_default() += count;
        }
        let m = readings.materializer().metrics();
        result.materializations.leaf_requests += m.leaf_requests;
        result.materializations.unique_leaves += m.unique_leaves;
        result.materializations.build_requests += m.build_requests;
        result.materializations.unique_builds += m.unique_builds;
        report.checked_readings = checked.len();
        if issues_before != result.issues.len() {
            report.enumeration = Enumeration::Failed;
        } else if report.enumeration == Enumeration::Complete {
            report.readings_sha256 = Some(super::report::readings_digest(&checked));
            report.nominal_normal_forms = Some(normal_forms);
        }
        Ok((
            report,
            UnitValues {
                representatives,
                all,
            },
        ))
    }
}

fn can_retain(category: Category) -> bool {
    // Compiler sequence/optional helpers have no standalone Reading value.
    !format!("{category:?}").starts_with("__")
}

#[allow(
    clippy::too_many_arguments,
    reason = "The proof keeps the original forest, surface, grammar and lexical context explicit"
)]
pub(super) fn analyze(
    forest: &GrammarForest<Grammar>,
    analyzed: &AnalyzedText,
    grammar: &Grammar,
    lexicon: &Lexicon,
    args: &EnglishV3Args,
    result: &mut FaceReport,
    context: &RealizationContext<'_>,
) -> Result<()> {
    let enumeration = UnitEnumeration {
        forest,
        analyzed,
        grammar,
        lexicon,
        args,
        context,
    };
    let category = args.field.category();
    let outer = forest
        .decompose(
            forest.roots(),
            &|c, _| {
                category == Category::Document
                    && !analyzed.tokens.is_empty()
                    && *c != Category::Ability
            },
            &|category| can_retain(*category),
        )
        .map_err(|error| anyhow::anyhow!(error))?;
    let Some(outer) = outer else {
        return Ok(());
    };
    let mut outer_values = Vec::new();
    let mut surroundings = Vec::new();
    for segment in outer.units() {
        let bytes = analyzed
            .byte_range(segment.span.start, segment.span.end)
            .context("invalid Ability span")?;
        result.ability_spans.push((bytes.start, bytes.end));
        let nodes: Vec<_> = segment.nodes.iter().copied().collect();
        let (plan, fallback) = enumeration.plan(&nodes)?;
        result.helper_fallbacks += usize::from(fallback);
        let mut values = Vec::new();
        for unit in plan.units() {
            let (line, unit_values) = enumeration.enumerate(unit, result, fallback)?;
            result.lines.push(line);
            values.push(unit_values);
        }
        if result
            .lines
            .iter()
            .any(|unit| unit.enumeration != Enumeration::Complete)
        {
            continue;
        }
        let bytes = analyzed
            .byte_range(segment.span.start, segment.span.end)
            .context("invalid surrounding span")?;
        // Generated surrounding constructors depend on children through their
        // admission summaries and recursive realization. Every unit value has
        // the same verified surface; check every summary tuple at the boundary.
        let choices = values
            .iter()
            .map(|unit| unit.representatives.values().cloned().collect::<Vec<_>>())
            .collect::<Vec<_>>();
        surroundings.push(encoding(forest, &plan, &example(&choices)?, grammar)?);
        let mut representatives = BTreeMap::new();
        visit_product(&choices, &mut |tuple| {
            let value = reconstruct(forest, &plan, tuple, grammar)?;
            let reading = validate_in_context(
                &value,
                &result.analyzed_source[bytes.clone()],
                segment.category,
                context,
            )
            .map_err(|issue| anyhow::anyhow!("surrounding validation: {issue:?}"))?;
            let summary = reading.admit_with(grammar, lexicon)?;
            ensure!(
                segment
                    .nodes
                    .iter()
                    .any(|&id| forest.summary(id) == Some(&summary)),
                "surrounding admission summary changed"
            );
            representatives.entry(summary).or_insert(value);
            Ok(())
        })?;
        if args.migration_from.is_some() {
            let all = values
                .iter()
                .map(|unit| unit.all.clone())
                .collect::<Vec<_>>();
            result.migration.push(bridge(
                forest,
                &plan,
                &nodes,
                &all,
                grammar,
                context,
                &result.analyzed_source[bytes.clone()],
                segment.category,
                (bytes.start, bytes.end),
            )?);
            // Verify every accepted Paragraph region separately, including
            // nested mode bodies, while stopping at Sentence boundaries.
            paragraph_bridges(forest, &plan, &all, grammar, context, analyzed, result)?;
        }
        outer_values.push(UnitValues {
            representatives,
            all: Vec::new(),
        });
    }
    if result
        .lines
        .iter()
        .all(|unit| unit.enumeration == Enumeration::Complete)
    {
        let choices = outer_values
            .iter()
            .map(|unit| unit.representatives.values().cloned().collect::<Vec<_>>())
            .collect::<Vec<_>>();
        surroundings.push(encoding(forest, &outer, &example(&choices)?, grammar)?);
        visit_product(&choices, &mut |tuple| {
            let value = reconstruct(forest, &outer, tuple, grammar)?;
            let reading =
                validate_in_context(&value, &result.analyzed_source, category, context)
                    .map_err(|issue| anyhow::anyhow!("root surrounding validation: {issue:?}"))?;
            let summary = reading.admit_with(grammar, lexicon)?;
            ensure!(
                grammar.root(&category, &summary),
                "reconstructed root constraint failed"
            );
            Ok(())
        })?;
        result.surroundings_sha256 = Some(surroundings_digest(category, &surroundings)?);
    }
    Ok(())
}

fn surroundings_digest(category: Category, surroundings: &[String]) -> Result<String> {
    // Summaries are derived admission data, not Reading identity. Reconstructed
    // values above still pass admission and the root constraint; hashing their
    // summary storage would additionally expose the compiler's feature slots.
    Ok(digest(&serde_json::to_vec(&(
        super::report::DECOMPOSITION_VERSION,
        format!("{category:?}"),
        surroundings,
        root_constraint_digest(),
    ))?))
}

fn root_constraint_digest() -> &'static str {
    // Context version 1 records the generated root predicate verified in
    // runtime.rs.txt: a public Category and no unresolved shared segments.
    // A root-contract change requires a new context version.
    static DIGEST: std::sync::OnceLock<String> = std::sync::OnceLock::new();
    DIGEST.get_or_init(|| digest(b"context-v1: public Category; Summary.segments is empty"))
}

fn reconstruct(
    forest: &GrammarForest<Grammar>,
    plan: &Decomposition<Category>,
    tuple: &[TracedValue],
    grammar: &Grammar,
) -> Result<TracedValue> {
    let mut tracing = Tracing::new(grammar);
    let values = tuple
        .iter()
        .map(|value| tracing.import(value))
        .collect::<Vec<_>>();
    Ok(forest.rebuild(plan, &values, &mut tracing)?)
}

fn example(choices: &[Vec<TracedValue>]) -> Result<Vec<TracedValue>> {
    choices
        .iter()
        .map(|values| {
            values
                .first()
                .cloned()
                .context("no validated boundary value")
        })
        .collect()
}

fn encoding(
    forest: &GrammarForest<Grammar>,
    plan: &Decomposition<Category>,
    values: &[TracedValue],
    grammar: &Grammar,
) -> Result<String> {
    let values = values
        .iter()
        .map(|value| (*value.value).clone())
        .collect::<Vec<_>>();
    Ok(
        forest.context_encoding(plan, &values, &mut &*grammar, &|value| {
            let Value::Reading(reading) = value else {
                return None;
            };
            let form = match reading {
                deckmaste_english_v3::grammar::Reading::OrdinaryAbility { form, .. }
                | deckmaste_english_v3::grammar::Reading::ActivatedAbility { form, .. }
                | deckmaste_english_v3::grammar::Reading::ModalItem { form, .. }
                | deckmaste_english_v3::grammar::Reading::Mode { form, .. } => Some(*form),
                _ => None,
            };
            Some(format!(
                "{}:{:?}:{form:?}",
                reading.construction(),
                reading.category()
            ))
        })?,
    )
}

fn visit_product<T: Clone>(
    choices: &[Vec<T>],
    visit: &mut impl FnMut(&[T]) -> Result<()>,
) -> Result<()> {
    fn next<T: Clone>(
        choices: &[Vec<T>],
        tuple: &mut Vec<T>,
        visit: &mut impl FnMut(&[T]) -> Result<()>,
    ) -> Result<()> {
        if let Some((first, rest)) = choices.split_first() {
            for value in first {
                tuple.push(value.clone());
                next(rest, tuple, visit)?;
                tuple.pop();
            }
        } else {
            visit(tuple)?;
        }
        Ok(())
    }
    next(choices, &mut Vec::new(), visit)
}

#[allow(
    clippy::too_many_arguments,
    reason = "The proof keeps the original forest, surface, grammar and lexical context explicit"
)]
fn bridge(
    forest: &GrammarForest<Grammar>,
    plan: &Decomposition<Category>,
    nodes: &[deckmaste_english_v3::CompletedId],
    choices: &[Vec<TracedValue>],
    grammar: &Grammar,
    context: &RealizationContext<'_>,
    surface: &str,
    category: Category,
    span: (usize, usize),
) -> Result<Bridge> {
    let factors: Vec<_> = choices.iter().map(Vec::len).collect();
    let product = factors
        .iter()
        .try_fold(1_usize, |product, &factor| product.checked_mul(factor))
        .context("migration product overflow")?;
    let mut old = BTreeMap::new();
    for value in forest
        .readings_from(nodes, Tracing::new(grammar))
        .with_digest_dedup(TracedValue::dedup_key)
    {
        let value = value?;
        validate_in_context(&value, surface, category, context)
            .map_err(|issue| anyhow::anyhow!("old Reading validation: {issue:?}"))?;
        old.insert(value.dedup_key(), value.identity().to_owned());
    }
    let mut reconstructed = BTreeMap::new();
    visit_product(choices, &mut |tuple| {
        let value = reconstruct(forest, plan, tuple, grammar)?;
        validate_in_context(&value, surface, category, context)
            .map_err(|issue| anyhow::anyhow!("migration reconstruction: {issue:?}"))?;
        ensure!(
            reconstructed
                .insert(value.dedup_key(), value.identity().to_owned())
                .is_none(),
            "two tuples rebuild the same Reading"
        );
        Ok(())
    })?;
    ensure!(
        old == reconstructed,
        "migration Reading identities differ at {span:?} ({category:?})"
    );
    ensure!(product == old.len(), "migration count differs at {span:?}");
    Ok(Bridge {
        span,
        category: format!("{category:?}"),
        factorized: plan.units().len() > 1
            || plan
                .units()
                .first()
                .is_some_and(|unit| unit.category != category),
        factors,
        old_count: old.len(),
        old_digest: super::report::readings_digest(&old.keys().copied().collect()),
        reconstructed_count: reconstructed.len(),
        reconstructed_digest: super::report::readings_digest(
            &reconstructed.keys().copied().collect(),
        ),
    })
}

#[allow(
    clippy::too_many_arguments,
    reason = "The proof keeps the original forest, surface, grammar and lexical context explicit"
)]
fn paragraph_bridges(
    forest: &GrammarForest<Grammar>,
    plan: &Decomposition<Category>,
    choices: &[Vec<TracedValue>],
    grammar: &Grammar,
    context: &RealizationContext<'_>,
    analyzed: &AnalyzedText,
    result: &mut FaceReport,
) -> Result<()> {
    for category in [Category::Paragraph, Category::SentenceParagraph] {
        for (nodes, region, range) in forest.regions(plan, &category) {
            let span = forest.span(nodes[0]);
            let bytes = analyzed
                .byte_range(span.start, span.end)
                .context("invalid paragraph span")?;
            result.migration.push(bridge(
                forest,
                &region,
                &nodes,
                &choices[range],
                grammar,
                context,
                &result.analyzed_source[bytes.clone()],
                category,
                (bytes.start, bytes.end),
            )?);
        }
    }
    Ok(())
}

pub(super) fn verify_migration(
    path: &std::path::Path,
    report: &super::report::Report,
    output: &mut dyn std::io::Write,
) -> Result<()> {
    let old: serde_json::Value =
        serde_json::from_reader(std::io::BufReader::new(std::fs::File::open(path)?))?;
    ensure!(
        old["schema_version"] == 8 && old["readings_digest_version"] == 1,
        "migration requires the original schema-8/digest-1 census"
    );
    ensure!(
        old["totals"]["undetermined"] == 0 && old["totals"]["issues"] == 0,
        "old census is incomplete or failed"
    );
    let new = serde_json::to_value(report)?;
    for key in [
        "field",
        "input_sha256",
        "selection",
        "lexical_inventory_sha256",
        "support_filter",
    ] {
        ensure!(old[key] == new[key], "migration context changed: {key}");
    }
    ensure!(
        new["totals"]["undetermined"] == 0 && new["totals"]["issues"] == 0,
        "new census is incomplete or failed"
    );
    let old_faces = old["faces"].as_array().context("old face array")?;
    ensure!(
        old_faces.len() == report.faces.len(),
        "migration face set changed"
    );
    let old_faces: BTreeMap<_, _> = old_faces
        .iter()
        .map(|face| (face["id"].as_str().expect("face identity"), face))
        .collect();
    let mut checked = 0;
    let mut factorized = 0;
    let mut paragraphs = 0;
    let mut retained = BTreeMap::<&str, usize>::new();
    for face in &report.faces {
        let old = old_faces
            .get(face.id.as_str())
            .context("migration face identity changed")?;
        ensure!(
            old["analyzed_source_sha256"] == face.analyzed_source_sha256,
            "migration source changed: {}",
            face.name
        );
        ensure!(
            (old["checked_readings"]
                .as_u64()
                .context("old exact count")?
                > 0)
                == (face.checked_readings > 0),
            "face changed between read and unread: {}",
            face.name
        );
        ensure!(
            old["chart"] == serde_json::to_value(&face.chart)?
                && old["unknown_words"] == serde_json::to_value(&face.unknown_words)?,
            "lexical/chart results changed: {}",
            face.name
        );
        for line in old["lines"].as_array().context("old segments")? {
            let span: (usize, usize) = serde_json::from_value(line["span"].clone())?;
            let bridge = face
                .migration
                .iter()
                .find(|bridge| {
                    bridge.span == span
                        && matches!(
                            bridge.category.as_str(),
                            "Ability" | "Document" | "TypeLine"
                        )
                })
                .context("old census segment has no reconstruction")?;
            ensure!(
                line["checked_readings"] == bridge.old_count
                    && line["readings_sha256"] == bridge.old_digest,
                "old census segment changed: {} {span:?}",
                face.name
            );
            ensure!(
                bridge.old_count == bridge.reconstructed_count
                    && bridge.old_digest == bridge.reconstructed_digest,
                "migration reconstruction mismatch"
            );
            checked += 1;
            factorized += usize::from(bridge.factorized);
        }
        paragraphs += face
            .migration
            .iter()
            .filter(|bridge| matches!(bridge.category.as_str(), "Paragraph" | "SentenceParagraph"))
            .count();
        for unit in &face.lines {
            if let Some(reason) = &unit.retained_reason {
                *retained.entry(reason).or_default() += 1;
            }
        }
    }
    writeln!(
        output,
        "Migration equality: {checked} old segments checked, {factorized} factorised; {paragraphs} Paragraph splits checked; retained-whole units by reason {retained:?}; 0 failures"
    )?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::english_v3::report::Census;

    #[test]
    fn surrounding_digest_records_structure_and_root_contract() {
        let structure = vec!["OrdinaryAbility(unit(Clause,0..12))".to_owned()];
        let expected = digest(
            &serde_json::to_vec(&(2, "Document", &structure, root_constraint_digest())).unwrap(),
        );
        assert_eq!(
            surroundings_digest(Category::Document, &structure).unwrap(),
            expected
        );
        assert_ne!(
            surroundings_digest(Category::Clause, &structure).unwrap(),
            expected
        );
        assert_ne!(
            surroundings_digest(
                Category::Document,
                &["ActivatedAbility(unit(Clause,0..12))".into()]
            )
            .unwrap(),
            expected,
        );
        assert_ne!(
            surroundings_digest(
                Category::Document,
                &["OrdinaryAbility(unit(Clause,1..12))".into()]
            )
            .unwrap(),
            expected,
        );
    }

    fn check(
        text: &str,
        category: Category,
        whole_count: usize,
        counts: &[usize],
        kinds: &[&str],
    ) -> FaceReport {
        let (face, actual_whole_count) = check_current(text, category, kinds);
        assert_eq!(
            face.lines
                .iter()
                .map(|unit| unit.checked_readings)
                .collect::<Vec<_>>(),
            counts
        );
        assert_eq!(actual_whole_count, whole_count);
        face
    }

    fn check_current(text: &str, category: Category, kinds: &[&str]) -> (FaceReport, usize) {
        let lexicon = super::super::tests::lexicon();
        let grammar = Grammar::with_lexicon(lexicon);
        let input = lexicon.analyze(text);
        let forest = deckmaste_english_v3::parse(&grammar, lexicon, &input, &category).unwrap();
        let scope = BTreeSet::new();
        let context = RealizationContext::new_scoped(&grammar, lexicon, text, &scope);
        let mut trees = BTreeSet::new();
        for value in forest.readings(Tracing::new(&grammar)) {
            let value = value.unwrap();
            validate_in_context(&value, text, category, &context).unwrap();
            trees.insert(value.dedup_key());
        }
        let whole_count = trees.len();
        let corpus = super::super::tests::selected_corpus(
            &serde_json::to_vec(&serde_json::json!({"data":{"Fixture":[{
                "name":"Fixture", "layout":"normal", "legalities":{"vintage":"Legal"}, "text":text,
                "type":text
            }]}}))
            .unwrap(),
        );
        let mut args = super::super::tests::args(None);
        args.migration_from = Some("unused-migration.json".into());
        if category == Category::TypeLine {
            args.field = super::super::SourceField::TypeLine;
        }
        let mut face = FaceReport::new(&corpus.faces[0], args.field);
        // Structural probes parse exact text, including parentheses. Corpus
        // normalization continues to remove balanced reminder text.
        face.analyzed_source = text.to_owned();
        analyze(
            &forest, &input, &grammar, lexicon, &args, &mut face, &context,
        )
        .unwrap();
        face.finish_readings();
        assert_eq!(face.issues, []);
        assert_eq!(face.enumeration, Enumeration::Complete);
        let counts: Vec<_> = face
            .lines
            .iter()
            .map(|unit| {
                let unit_category = grammar
                    .productions()
                    .iter()
                    .map(|production| production.category)
                    .find(|category| format!("{category:?}") == unit.category)
                    .unwrap();
                let input = lexicon.analyze(&text[unit.span.0..unit.span.1]);
                let forest =
                    deckmaste_english_v3::parse(&grammar, lexicon, &input, &unit_category).unwrap();
                let values: BTreeSet<_> = forest.readings(&grammar).map(Result::unwrap).collect();
                assert_eq!(unit.checked_readings, values.len());
                values.len()
            })
            .collect();
        assert_eq!(
            face.lines
                .iter()
                .map(|unit| unit.category.as_str())
                .collect::<Vec<_>>(),
            kinds,
            "categories: {text}"
        );
        assert_eq!(face.exact_readings, Some(counts.iter().sum()));
        assert_eq!(
            face.census,
            if whole_count == 0 {
                Census::No
            } else if counts.iter().any(|&n| n > 1) {
                Census::Multiple
            } else {
                Census::One
            }
        );
        for bridge in &face.migration {
            assert_eq!(bridge.old_count, bridge.reconstructed_count);
            assert_eq!(bridge.old_digest, bridge.reconstructed_digest);
            assert_eq!(bridge.old_count, bridge.factors.iter().product::<usize>());
        }
        if let Some(bridge) = face
            .migration
            .iter()
            .find(|bridge| bridge.category == format!("{category:?}"))
        {
            assert_eq!(bridge.old_count, whole_count);
            assert_eq!(bridge.reconstructed_count, trees.len());
        }
        (face, whole_count)
    }

    #[test]
    fn ticket_structural_probes_preserve_admission_and_account_for_every_unit() {
        for (text, trees, counts, kinds) in [
            (
                "Destroy target creature with flying. Destroy target creature with flying.",
                4,
                vec![2, 2],
                vec!["Sentence", "Sentence"],
            ),
            (
                "{T}: Destroy target creature with flying. Draw a card.",
                2,
                vec![1, 2, 1],
                vec!["Cost", "Sentence", "Sentence"],
            ),
            (
                "Sacrifice a creature with flying: Draw a card.",
                2,
                vec![2, 1],
                vec!["Cost", "Sentence"],
            ),
            (
                "Choose one —\n• Draw a card.\n• Destroy target creature with flying.",
                2,
                vec![1, 1, 2],
                vec!["Clause", "Sentence", "Sentence"],
            ),
            (
                "Choose one —\n• Destroy target creature with flying. Draw a card.\n• Target creature is tapped.",
                4,
                vec![1, 2, 1, 2],
                vec!["Clause", "Sentence", "Sentence", "Sentence"],
            ),
            (
                "Choose a creature with flying —\n• Draw a card.\n• Gain 1 life.",
                2,
                vec![2, 1, 1],
                vec!["Clause", "Sentence", "Sentence"],
            ),
            (
                "Choose one.\n• First — Draw a card.\n• Second — Gain 1 life.",
                1,
                vec![1, 1, 1],
                vec!["Sentence", "Sentence", "Sentence"],
            ),
            ("Ward {2}, protection from red", 1, vec![1], vec!["Ability"]),
            (
                "Protection from creatures with flying",
                2,
                vec![2],
                vec!["Ability"],
            ),
            (
                "I — Draw a card.\nII — Destroy target creature with flying.",
                2,
                vec![1, 2],
                vec!["Sentence", "Sentence"],
            ),
            ("Level up {1}", 1, vec![1], vec!["Ability"]),
            ("Level up {1}\nLEVEL 1-2\n2/2\nFlying", 0, vec![], vec![]),
            ("{1}: Level 2\nThis creature has flying.", 0, vec![], vec![]),
            (
                "This creature has \"Destroy target creature with flying. Draw a card.\"",
                2,
                vec![2],
                vec!["Sentence"],
            ),
            (
                "Draw a card. (Destroy target creature with flying.)",
                2,
                vec![1, 2],
                vec!["Sentence", "Sentence"],
            ),
            ("", 1, vec![1], vec!["Document"]),
            ("Draw a card", 0, vec![], vec![]),
            (
                "Draw a card.\nDestroy target creature with flying.",
                2,
                vec![1, 2],
                vec!["Sentence", "Sentence"],
            ),
        ] {
            check(text, Category::Document, trees, &counts, &kinds);
        }
        check(
            "Legendary Creature — Human Wizard",
            Category::TypeLine,
            1,
            &[1],
            &["TypeLine"],
        );
        let lexicon = super::super::tests::lexicon();
        let grammar = Grammar::with_lexicon(lexicon);
        let input = lexicon.analyze("Draw a card");
        let forest =
            deckmaste_english_v3::parse(&grammar, lexicon, &input, &Category::Sentence).unwrap();
        let values = forest
            .readings(Tracing::new(&grammar))
            .collect::<std::result::Result<Vec<_>, _>>()
            .unwrap();
        assert_eq!(values.len(), 1);
        super::super::validation::validate(
            &values[0],
            "Draw a card",
            lexicon,
            Category::Sentence,
            &grammar,
        )
        .unwrap();
    }

    #[test]
    fn keyword_units_are_ordinary_units_without_retained_reasons() {
        for text in ["Flying", "Ward {2}, protection from red"] {
            let face = check(text, Category::Document, 1, &[1], &["Ability"]);
            assert_eq!(face.lines[0].retained_reason, None);
            assert_eq!(face.lines[0].proposed_parts, []);
            assert_eq!(face.helper_fallbacks, 0);
        }
    }

    #[test]
    fn ability_word_label_alternatives_keep_the_current_whole_unit_and_proposed_sentence() {
        let text = "Landfall — Whenever a land you control enters, you get {E}{E}.";
        let face = check(text, Category::Document, 2, &[2], &["Ability"]);
        assert_eq!(
            face.lines[0].retained_reason.as_deref(),
            Some("alternative-wrapper")
        );
        assert_eq!(
            face.lines[0].proposed_parts,
            [("Sentence".into(), (13, text.len()))]
        );
        assert_eq!(face.helper_fallbacks, 0);
    }

    #[test]
    fn bridge_rejects_a_mutated_reconstruction_with_the_original_forest_unchanged() {
        let text = "Draw a card.";
        let lexicon = super::super::tests::lexicon();
        let grammar = Grammar::with_lexicon(lexicon);
        let input = lexicon.analyze(text);
        let forest =
            deckmaste_english_v3::parse(&grammar, lexicon, &input, &Category::Document).unwrap();
        let scope = BTreeSet::new();
        let context = RealizationContext::new_scoped(&grammar, lexicon, text, &scope);
        let args = super::super::tests::args(None);
        let enumeration = UnitEnumeration {
            forest: &forest,
            analyzed: &input,
            grammar: &grammar,
            lexicon,
            args: &args,
            context: &context,
        };
        let (plan, _) = enumeration.plan(forest.roots()).unwrap();
        let values = forest
            .readings_from(
                &plan.units()[0].nodes.iter().copied().collect::<Vec<_>>(),
                Tracing::new(&grammar),
            )
            .collect::<std::result::Result<Vec<_>, _>>()
            .unwrap();
        bridge(
            &forest,
            &plan,
            forest.roots(),
            &[values],
            &grammar,
            &context,
            text,
            Category::Document,
            (0, text.len()),
        )
        .unwrap();
        let replacement = lexicon.analyze("Gain 1 life.");
        let replacement_forest =
            deckmaste_english_v3::parse(&grammar, lexicon, &replacement, &Category::Sentence)
                .unwrap();
        let mutated = replacement_forest
            .readings(Tracing::new(&grammar))
            .next()
            .unwrap()
            .unwrap();
        let error = bridge(
            &forest,
            &plan,
            forest.roots(),
            &[vec![mutated]],
            &grammar,
            &context,
            text,
            Category::Document,
            (0, text.len()),
        )
        .unwrap_err();
        assert!(
            error.to_string().contains("migration reconstruction"),
            "{error}"
        );
    }

    #[test]
    fn necromentia_migration_preserves_trees_and_counts_unit_readings() {
        let text = "Choose a card name other than a basic land card name. Search target opponent's graveyard, hand, and library for any number of cards with that name and exile them. That player shuffles, then creates a 2/2 black Zombie creature token for each card exiled from their hand this way.";
        let (face, whole_count) = check_current(
            text,
            Category::Document,
            &["Sentence", "Sentence", "Sentence"],
        );
        let ability = face
            .migration
            .iter()
            .find(|region| region.category == "Ability")
            .unwrap();
        assert_eq!(ability.old_count, whole_count);
        assert_eq!(ability.reconstructed_count, whole_count);
        assert!(face.lines.iter().all(|unit| unit.retained_reason.is_none()));
    }
}

#[cfg(test)]
mod gate_tests {
    use super::*;

    #[test]
    fn migration_gate_rejects_digest_substitution_and_coverage_change_at_equal_context() {
        let corpus = super::super::tests::selected_corpus(
            &serde_json::to_vec(&serde_json::json!({"data":{"Fixture":[{
                "layout":"normal", "legalities":{"vintage":"Legal"}, "text":"Draw a card."
            }]}}))
            .unwrap(),
        );
        let lexicon = super::super::tests::lexicon();
        let grammar = Grammar::with_lexicon(lexicon);
        let mut args = super::super::tests::args(None);
        args.migration_from = Some("unused".into());
        let faces = super::super::analyze_cards(&corpus.faces, lexicon, &grammar, &args).unwrap();
        let report = super::super::report::Report::new(
            &args,
            &corpus,
            "inventory".into(),
            vec![],
            faces,
            super::super::report::Measurements {
                host_load: None,
                setup_wall_ns: 0,
                corpus_wall_ns: 0,
            },
        );
        let mut old = serde_json::to_value(&report).unwrap();
        old["schema_version"] = 8.into();
        old["readings_digest_version"] = 1.into();
        let bridge = &report.faces[0].migration[0];
        old["faces"][0]["lines"] = serde_json::json!([{
            "span":bridge.span, "checked_readings":bridge.old_count, "readings_sha256":bridge.old_digest
        }]);
        let file = tempfile::NamedTempFile::new().unwrap();
        std::fs::write(file.path(), serde_json::to_vec(&old).unwrap()).unwrap();
        verify_migration(file.path(), &report, &mut Vec::new()).unwrap();
        old["faces"][0]["lines"][0]["readings_sha256"] = "substituted-equal-count-set".into();
        std::fs::write(file.path(), serde_json::to_vec(&old).unwrap()).unwrap();
        assert!(
            verify_migration(file.path(), &report, &mut Vec::new())
                .unwrap_err()
                .to_string()
                .contains("old census segment changed")
        );
        old["faces"][0]["checked_readings"] = 0.into();
        std::fs::write(file.path(), serde_json::to_vec(&old).unwrap()).unwrap();
        assert!(
            verify_migration(file.path(), &report, &mut Vec::new())
                .unwrap_err()
                .to_string()
                .contains("face changed between read and unread")
        );
    }
}
