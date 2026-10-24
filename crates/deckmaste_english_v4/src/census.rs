//! The census: every Corpus Unit profiled into its Complexity Stratum and
//! parsed, each reported as read, known-wrong-only or unread
//! (`docs/decisions/english-v4.md` §6.2).
//!
//! A unit is read when it has a Reading and both round-trip laws hold:
//! law 1 over the whole Packed Forest
//! ([`deckmaste_syntax::Parse::verify_realization`]) and law 2 on its
//! cheapest Readings ([`LAW_TWO_READINGS`]). No attachment tree is enumerated
//! except for a unit with a judgment, whose Readings are checked against it
//! up to [`judgment::READING_LIMIT`]. A unit is known-wrong-only when it has a
//! judgment none of its Readings satisfies.
//!
//! A line of a cleave face with square brackets is read as its two texts
//! ([`crate::cleave`]), each judged as a unit of its own: it is read when
//! both are, and otherwise has the first verdict of the two that is not read.
//! Its counts are its Uncleaved Text's.
//!
//! A unit the tracked deferral list ([`crate::DEFERRED`]) names, by card and
//! text, is deferred while it is not read: counted apart and never a
//! regression. A law failure is a defect and is never deferred. A deferral
//! whose unit is read is stale, as is one that names no unit at all
//! ([`crate::corpus::Corpus::unmatched_deferrals`]).

use std::collections::BTreeMap;
use std::collections::BTreeSet;

use deckmaste_lexical::AnalyzedText;
use deckmaste_lexical_source_v4::LexicalEnvironment;
use deckmaste_syntax::CompiledGrammar;

use crate::cleave::CleaveText;
use crate::corpus::Corpus;
use crate::corpus::UnitKind;
use crate::corpus::split_lines;
use crate::judgment;
use crate::judgment::Judge;
use crate::judgment::Judgment;
use crate::judgment::Located;
use crate::profile::Family;
use crate::profile::MAX_STRATUM;
use crate::profile::Profile;
use crate::profile::Profiler;
use crate::profile::Stratum;
use crate::segments::SegmentFrame;
use crate::segments::Segmentation;

/// How many of a unit's cheapest Readings round-trip law 2 is checked on.
pub const LAW_TWO_READINGS: usize = 8;

/// What the census found for one unit.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize)]
pub enum Verdict {
    Read,
    KnownWrongOnly,
    Unread(Unread),
}

/// Why a unit is unread.
#[derive(Debug, Clone, PartialEq, Eq, Ord, PartialOrd, serde::Serialize)]
pub enum Unread {
    /// The grammar does not bind to the lexical environment.
    Unbound,
    VocabularyGap,
    NoReading,
    /// Round-trip law 1 or 2 fails: a defect, never coverage.
    Law(String),
}

/// One unit's result.
#[derive(Debug, Clone, serde::Serialize)]
pub struct UnitResult {
    pub face: String,
    pub name: String,
    pub kind: UnitKind,
    pub index: usize,
    pub text: String,
    pub profile: Profile,
    pub verdict: Verdict,
    /// Distinct trees in its Packed Forest: the product of its Segments'
    /// counts.
    pub derivations: u128,
    /// Its Segments and their Derivations, when it has a Derivation.
    pub segments: Option<Segmentation>,
    /// Σ(nᵢ − 1) over its Segments' Derivations; `derivations − 1` for a
    /// unit that is not segmentable.
    pub surplus: u128,
    /// Derivations sharing the least Ranking Key: the cheapest tier before
    /// the final tie-break (0 without a Derivation).
    pub cheapest_tier: u128,
}

/// Counts for one Stratum.
#[derive(Debug, Clone, Default, PartialEq, Eq, serde::Serialize)]
pub struct StratumRow {
    pub stratum: Stratum,
    pub units: usize,
    pub read: usize,
    pub known_wrong_only: usize,
    pub unread: usize,
    /// Units not read that the deferral list names (not counted in `unread`
    /// or `known_wrong_only`).
    pub deferred: usize,
    /// Units not read that the declined list names (not counted in `unread`,
    /// `known_wrong_only` or `deferred`).
    pub declined: usize,
    /// Units with a round-trip law failure (counted in `unread`).
    pub law_failures: usize,
    /// Faces whose highest unit is in this Stratum.
    pub faces: usize,
    /// Of those faces, the ones every unit of which is read.
    pub faces_read: usize,
    /// Of those faces, the ones the baseline census did not read.
    pub baseline_unread_faces: Option<usize>,
}

/// A Syntactic Skeleton's frequency.
#[derive(Debug, Clone, serde::Serialize)]
pub struct SkeletonRow {
    pub stratum: Stratum,
    pub skeleton: String,
    pub units: usize,
    pub example: String,
}

/// The whole census.
#[derive(Debug, Clone, serde::Serialize)]
pub struct Report {
    pub bind_error: Option<String>,
    pub completed_stratum: Option<Stratum>,
    pub faces: usize,
    pub strata: Vec<StratumRow>,
    /// Units per Syntactic Family.
    pub families: Vec<(Family, usize)>,
    /// Units per family among the faces the baseline census did not read.
    pub baseline_unread_families: Option<Vec<(Family, usize)>>,
    /// Baseline-unread faces absent from this corpus (they should be none).
    pub baseline_unread_missing: Option<Vec<String>>,
    pub skeletons: Vec<SkeletonRow>,
    /// Units at or below the completed Stratum that are not read and not
    /// deferred.
    pub regressions: Vec<String>,
    /// Deferrals whose unit is read.
    pub stale_deferrals: Vec<String>,
    pub total_derivations: u128,
    /// The units' Surplus, summed.
    pub total_surplus: u128,
    /// Units with more than one Reading (a Surplus above 0).
    pub ambiguous_units: usize,
    /// Ambiguous units whose cheapest tier is a single Derivation.
    pub singleton_cheapest_units: usize,
    /// The ambiguous units' cheapest tiers, summed.
    pub cheapest_tier_total: u128,
    /// The largest cheapest tier of any unit.
    pub largest_cheapest_tier: u128,
    /// Segments with more than one Derivation.
    pub ambiguous_segments: usize,
    /// Units with a Derivation but no single product segmentation.
    pub not_segmentable: Vec<String>,
    pub units: Vec<UnitResult>,
}

/// Runs the census over `corpus`. `baseline` maps face identities to
/// whether an earlier census read them.
///
/// # Panics
/// If a census worker thread panics.
#[must_use]
pub fn run(
    corpus: &Corpus,
    env: &LexicalEnvironment,
    judgments: &[Judgment],
    baseline: Option<&BTreeMap<String, bool>>,
) -> Report {
    let grammar = crate::bind(&env.lexicon, &env.frames).and_then(|grammar| {
        let frame = crate::segments::segment_frame(&grammar)?;
        Ok((grammar, frame))
    });
    let bind_error = grammar.as_ref().err().map(ToString::to_string);
    let by_text: BTreeMap<&str, Vec<&Judgment>> = judgments
        .iter()
        .filter(|e| e.grammatical && e.category.is_none())
        .fold(BTreeMap::new(), |mut map, e| {
            map.entry(e.text.as_str()).or_insert_with(Vec::new).push(e);
            map
        });
    let bracket_keywords = crate::cleave::BracketKeywords::new(&env.lexicon);
    let cleave_faces: Vec<bool> = corpus
        .faces
        .iter()
        .map(|face| {
            face.oracle_text
                .as_deref()
                .is_some_and(|text| bracket_keywords.mark(&split_lines(text)))
        })
        .collect();
    let threads = std::thread::available_parallelism().map_or(1, std::num::NonZero::get);
    let chunk = corpus.units.len().div_ceil(threads).max(1);
    let units: Vec<UnitResult> = std::thread::scope(|scope| {
        let workers: Vec<_> = corpus
            .units
            .chunks(chunk)
            .map(|units| {
                let grammar = grammar.as_ref().ok();
                let by_text = &by_text;
                let cleave_faces = &cleave_faces;
                scope.spawn(move || {
                    let profiler = Profiler::new(env);
                    units
                        .iter()
                        .map(|unit| {
                            let face = &corpus.faces[unit.face];
                            let analyzed =
                                env.lexicon
                                    .analyze_source_scoped(&unit.text, None, &face.scope);
                            let profile = profiler.profile(unit.kind, &analyzed);
                            let entries = |text: &str| -> Vec<&Judgment> {
                                by_text
                                    .get(text)
                                    .into_iter()
                                    .flatten()
                                    .filter(|e| {
                                        e.card
                                            .as_ref()
                                            .is_none_or(|c| *c == face.card || *c == face.name)
                                    })
                                    .copied()
                                    .collect()
                            };
                            let cleave = cleave_faces[unit.face]
                                .then(|| CleaveText::split(&unit.text))
                                .flatten();
                            let (verdict, derivations, segments, cheapest_tier) = match grammar {
                                None => (Verdict::Unread(Unread::Unbound), 0, None, 0),
                                Some((grammar, frame)) => {
                                    let judge = |text: &str, analyzed: &AnalyzedText| {
                                        let entries = entries(text);
                                        judge_unit(
                                            grammar,
                                            frame,
                                            env,
                                            analyzed,
                                            &face.scope,
                                            &entries,
                                        )
                                    };
                                    match cleave {
                                        None => judge(&unit.text, &analyzed),
                                        Some(cleave) => judge_cleave(&cleave, |text| {
                                            let analyzed = env.lexicon.analyze_source_scoped(
                                                text,
                                                None,
                                                &face.scope,
                                            );
                                            judge(text, &analyzed)
                                        }),
                                    }
                                }
                            };
                            let surplus = segments
                                .as_ref()
                                .map_or(0, |s: &Segmentation| s.surplus(derivations));
                            UnitResult {
                                face: face.identity.clone(),
                                name: face.name.clone(),
                                kind: unit.kind,
                                index: unit.index,
                                text: unit.text.clone(),
                                profile,
                                verdict,
                                derivations,
                                segments,
                                surplus,
                                cheapest_tier,
                            }
                        })
                        .collect::<Vec<_>>()
                })
            })
            .collect();
        workers
            .into_iter()
            .flat_map(|w| w.join().expect("census worker"))
            .collect()
    });
    summarize(corpus.faces.len(), bind_error, baseline, units)
}

/// Judges a bracketed line of a cleave face as its Uncleaved Text, then, if
/// that is read, its Cleaved Text: the line keeps the Uncleaved Text's counts
/// and the first verdict that is not read.
fn judge_cleave(
    cleave: &CleaveText,
    judge: impl Fn(&str) -> (Verdict, u128, Option<Segmentation>, u128),
) -> (Verdict, u128, Option<Segmentation>, u128) {
    let (verdict, derivations, segments, cheapest_tier) = judge(&cleave.uncleaved);
    let verdict = match verdict {
        Verdict::Read => judge(&cleave.cleaved).0,
        unread => unread,
    };
    (verdict, derivations, segments, cheapest_tier)
}

fn judge_unit(
    grammar: &CompiledGrammar,
    frame: &SegmentFrame,
    env: &LexicalEnvironment,
    analyzed: &deckmaste_lexical::AnalyzedText,
    scope: &BTreeSet<deckmaste_lexical::LexemeId>,
    entries: &[&Judgment],
) -> (Verdict, u128, Option<Segmentation>, u128) {
    let parse = grammar.parse_analyzed(analyzed);
    if !parse.vocabulary_gaps.is_empty() {
        return (Verdict::Unread(Unread::VocabularyGap), 0, None, 0);
    }
    let derivations = parse.derivation_count();
    if derivations == 0 {
        return (Verdict::Unread(Unread::NoReading), 0, None, 0);
    }
    let segments = Some(crate::segments::segments(&parse, frame));
    let cheapest_tier = parse.cheapest_tier().map_or(0, |(_, tied)| tied);
    let (verdict, derivations) = judge_parse(grammar, env, &parse, derivations, scope, entries);
    (verdict, derivations, segments, cheapest_tier)
}

fn judge_parse(
    grammar: &CompiledGrammar,
    env: &LexicalEnvironment,
    parse: &deckmaste_syntax::Parse<'_>,
    derivations: u128,
    scope: &BTreeSet<deckmaste_lexical::LexemeId>,
    entries: &[&Judgment],
) -> (Verdict, u128) {
    if let Err(fault) = parse.verify_realization(&env.lexicon) {
        return (
            Verdict::Unread(Unread::Law(format!("law 1: {fault}"))),
            derivations,
        );
    }
    for reading in parse.best_readings(LAW_TWO_READINGS) {
        if let Err(why) = judgment::law_two(grammar, &env.lexicon, parse, &reading, scope) {
            return (Verdict::Unread(Unread::Law(why)), derivations);
        }
    }
    if !entries.is_empty() {
        let judge = Judge {
            grammar,
            lexicon: &env.lexicon,
        };
        let limit = usize::try_from(judgment::READING_LIMIT).unwrap_or(usize::MAX);
        let endorsed = parse.best_readings(limit).iter().any(|reading| {
            grammar
                .constituents(reading, &env.lexicon)
                .is_ok_and(|(text, constituents)| {
                    let located = Located { text, constituents };
                    entries.iter().all(|e| judge.endorses(e, &located))
                })
        });
        if !endorsed {
            return (Verdict::KnownWrongOnly, derivations);
        }
    }
    (Verdict::Read, derivations)
}

fn summarize(
    faces: usize,
    bind_error: Option<String>,
    baseline: Option<&BTreeMap<String, bool>>,
    units: Vec<UnitResult>,
) -> Report {
    let completed = crate::COMPLETED_STRATUM;
    let mut strata: Vec<StratumRow> = (0..=MAX_STRATUM)
        .map(|stratum| StratumRow {
            stratum,
            baseline_unread_faces: baseline.map(|_| 0),
            ..StratumRow::default()
        })
        .collect();
    let mut families: BTreeMap<Family, usize> = BTreeMap::new();
    let mut residual_families: BTreeMap<Family, usize> = BTreeMap::new();
    let mut skeletons: BTreeMap<(Stratum, &str), (usize, &str)> = BTreeMap::new();
    // Per face: highest Stratum, and whether every unit is read.
    let mut by_face: BTreeMap<&str, (Stratum, bool)> = BTreeMap::new();
    let mut regressions = Vec::new();
    let mut total_derivations = 0u128;
    let mut total_surplus = 0u128;
    let mut ambiguous_units = 0;
    let mut singleton_cheapest_units = 0;
    let mut cheapest_tier_total = 0u128;
    let mut largest_cheapest_tier = 0u128;
    let mut ambiguous_segments = 0;
    let mut not_segmentable = Vec::new();
    let mut stale_deferrals = Vec::new();
    let deferrals = deferrals();
    let declined_list = declined();
    let residual = |face: &str| baseline.is_some_and(|b| b.get(face) == Some(&false));
    for unit in &units {
        let stratum = unit.profile.stratum;
        let row = &mut strata[usize::from(stratum)];
        row.units += 1;
        let named = deferrals.contains(&(unit.name.as_str(), unit.text.as_str()));
        let deferred = named
            && !matches!(
                unit.verdict,
                Verdict::Read | Verdict::Unread(Unread::Law(_))
            );
        let declined = declined_list.contains(&(unit.name.as_str(), unit.text.as_str()))
            && !matches!(
                unit.verdict,
                Verdict::Read | Verdict::Unread(Unread::Law(_))
            );
        match &unit.verdict {
            _ if declined => row.declined += 1,
            _ if deferred => row.deferred += 1,
            Verdict::Read => row.read += 1,
            Verdict::KnownWrongOnly => row.known_wrong_only += 1,
            Verdict::Unread(reason) => {
                row.unread += 1;
                if matches!(reason, Unread::Law(_)) {
                    row.law_failures += 1;
                }
            }
        }
        if (named || declined_list.contains(&(unit.name.as_str(), unit.text.as_str())))
            && unit.verdict == Verdict::Read
        {
            stale_deferrals.push(format!("{} | {}", unit.name, unit.text));
        }
        if unit.verdict != Verdict::Read
            && !deferred
            && !declined
            && completed.is_some_and(|done| stratum <= done)
        {
            regressions.push(format!("{} ({}): {}", unit.name, unit.face, unit.text));
        }
        total_derivations = total_derivations.saturating_add(unit.derivations);
        total_surplus = total_surplus.saturating_add(unit.surplus);
        ambiguous_units += usize::from(unit.derivations > 1);
        if unit.derivations > 1 {
            singleton_cheapest_units += usize::from(unit.cheapest_tier == 1);
            cheapest_tier_total = cheapest_tier_total.saturating_add(unit.cheapest_tier);
        }
        largest_cheapest_tier = largest_cheapest_tier.max(unit.cheapest_tier);
        match &unit.segments {
            Some(Segmentation::Segments(segments)) => {
                ambiguous_segments += segments.iter().filter(|s| s.derivations > 1).count();
            }
            Some(Segmentation::NotSegmentable(why)) => {
                not_segmentable.push(format!("{} ({why:?}): {}", unit.name, unit.text));
            }
            None => {}
        }
        for family in &unit.profile.families {
            *families.entry(*family).or_default() += 1;
            if residual(&unit.face) {
                *residual_families.entry(*family).or_default() += 1;
            }
        }
        let skeleton = skeletons
            .entry((stratum, unit.profile.skeleton.as_str()))
            .or_insert((0, unit.text.as_str()));
        skeleton.0 += 1;
        let face = by_face.entry(unit.face.as_str()).or_insert((0, true));
        face.0 = face.0.max(stratum);
        face.1 &= unit.verdict == Verdict::Read;
    }
    for (face, (stratum, read)) in &by_face {
        let row = &mut strata[usize::from(*stratum)];
        row.faces += 1;
        row.faces_read += usize::from(*read);
        if residual(face)
            && let Some(count) = &mut row.baseline_unread_faces
        {
            *count += 1;
        }
    }
    let baseline_unread_missing = baseline.map(|b| {
        b.iter()
            .filter(|(face, read)| !**read && !by_face.contains_key(face.as_str()))
            .map(|(face, _)| face.clone())
            .collect()
    });
    let mut skeletons: Vec<SkeletonRow> = skeletons
        .into_iter()
        .map(|((stratum, skeleton), (units, example))| SkeletonRow {
            stratum,
            skeleton: skeleton.to_owned(),
            units,
            example: example.to_owned(),
        })
        .collect();
    skeletons.sort_by(|a, b| {
        a.stratum
            .cmp(&b.stratum)
            .then(b.units.cmp(&a.units))
            .then(a.skeleton.cmp(&b.skeleton))
    });
    Report {
        bind_error,
        completed_stratum: completed,
        faces,
        strata,
        families: families.into_iter().collect(),
        baseline_unread_families: baseline.map(|_| residual_families.into_iter().collect()),
        baseline_unread_missing,
        skeletons,
        regressions,
        stale_deferrals,
        total_derivations,
        total_surplus,
        ambiguous_units,
        singleton_cheapest_units,
        cheapest_tier_total,
        largest_cheapest_tier,
        ambiguous_segments,
        not_segmentable,
        units,
    }
}

/// The tracked deferral list as (card name, unit text) pairs: one `NAME |
/// TEXT` line each, with `#` comments naming the ticket that owns them.
///
/// # Panics
/// On a malformed line, which `tests/census.rs` rejects first.
#[must_use]
pub fn deferrals() -> BTreeSet<(&'static str, &'static str)> {
    crate::DEFERRED
        .lines()
        .map(str::trim)
        .filter(|line| !line.is_empty() && !line.starts_with('#'))
        .map(|line| {
            let (name, text) = line
                .split_once(" | ")
                .unwrap_or_else(|| panic!("deferral `{line}` is not `NAME | TEXT`"));
            (name.trim(), text.trim())
        })
        .collect()
}

/// The declined units: `NAME | TEXT` lines of `declined.txt`, each preceded by
/// comment lines giving the construction and the reason.
///
/// # Panics
/// On a malformed line, which `tests/census.rs` rejects first.
#[must_use]
pub fn declined() -> BTreeSet<(&'static str, &'static str)> {
    crate::DECLINED
        .lines()
        .map(str::trim)
        .filter(|line| !line.is_empty() && !line.starts_with('#'))
        .map(|line| {
            let (name, text) = line
                .split_once(" | ")
                .unwrap_or_else(|| panic!("declined entry `{line}` is not `NAME | TEXT`"));
            (name.trim(), text.trim())
        })
        .collect()
}

impl Report {
    /// A plain-text summary: per-Stratum counts, families, and the most
    /// frequent `top` skeletons of each Stratum.
    #[must_use]
    pub fn summary(&self, top: usize) -> String {
        use std::fmt::Write;
        let mut out = String::new();
        if let Some(error) = &self.bind_error {
            let _ = writeln!(
                out,
                "grammar does not bind to the lexical environment: {error}"
            );
        }
        let _ = writeln!(
            out,
            "faces {}  units {}  completed stratum {}",
            self.faces,
            self.units.len(),
            self.completed_stratum
                .map_or_else(|| "none".to_owned(), |s| s.to_string()),
        );
        let _ = writeln!(
            out,
            "surplus {}  ambiguous units {}  ambiguous segments {}  not segmentable {}  total derivations (product) {}",
            self.total_surplus,
            self.ambiguous_units,
            self.ambiguous_segments,
            self.not_segmentable.len(),
            self.total_derivations
        );
        let _ = writeln!(
            out,
            "cheapest tier: singleton in {} of {} ambiguous units  summed {}  largest {}",
            self.singleton_cheapest_units,
            self.ambiguous_units,
            self.cheapest_tier_total,
            self.largest_cheapest_tier
        );
        let _ = writeln!(
            out,
            "stratum  units   read  known-wrong-only  unread  deferred  declined  law-failures   faces  faces-read  baseline-unread-faces"
        );
        for row in &self.strata {
            let _ = writeln!(
                out,
                "{:>7}  {:>5}  {:>5}  {:>16}  {:>6}  {:>8}  {:>8}  {:>12}  {:>6}  {:>10}  {:>21}",
                row.stratum,
                row.units,
                row.read,
                row.known_wrong_only,
                row.unread,
                row.deferred,
                row.declined,
                row.law_failures,
                row.faces,
                row.faces_read,
                row.baseline_unread_faces
                    .map_or_else(|| "-".to_owned(), |c| c.to_string())
            );
        }
        if let Some(missing) = &self.baseline_unread_missing {
            let _ = writeln!(
                out,
                "baseline-unread faces missing from this corpus: {}",
                missing.len()
            );
        }
        let residual: BTreeMap<Family, usize> = self
            .baseline_unread_families
            .iter()
            .flatten()
            .copied()
            .collect();
        let _ = writeln!(out, "\nfamily units (units in baseline-unread faces)");
        for (family, units) in &self.families {
            let _ = writeln!(
                out,
                "  S{} {:<26} {:>6} ({})",
                family.stratum,
                family.name,
                units,
                residual.get(family).copied().unwrap_or(0)
            );
        }
        for stratum in 0..=MAX_STRATUM {
            let _ = writeln!(out, "\nStratum {stratum}: top skeletons");
            for row in self
                .skeletons
                .iter()
                .filter(|r| r.stratum == stratum)
                .take(top)
            {
                let _ = writeln!(out, "  {:>5}  {}", row.units, row.skeleton);
            }
        }
        if !self.regressions.is_empty() {
            let _ = writeln!(out, "\nregressions at or below the completed stratum:");
            for line in &self.regressions {
                let _ = writeln!(out, "  {line}");
            }
        }
        if !self.not_segmentable.is_empty() {
            let _ = writeln!(out, "\nunits with no single product segmentation:");
            for line in &self.not_segmentable {
                let _ = writeln!(out, "  {line}");
            }
        }
        if !self.stale_deferrals.is_empty() {
            let _ = writeln!(out, "\nstale deferrals (the unit is read):");
            for line in &self.stale_deferrals {
                let _ = writeln!(out, "  {line}");
            }
        }
        out
    }
}
