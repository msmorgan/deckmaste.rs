use std::io::Write;
use std::path::Path;

use anyhow::Context;
use deckmaste_english_v2::ast::OracleText;
use deckmaste_english_v2::context::ParseContext;
use deckmaste_english_v2::parser::Parser;
use deckmaste_english_v2::parser::ParserTrace;
use deckmaste_english_v2::parser::TraceLimits;

use super::InspectArgs;
use super::corpus::Corpus;
use super::corpus::CorpusUnit;
use super::corpus::ValidatedCorpusId;
use super::diagnostic;
use super::diagnostic::DiagnosticReport;
use super::packed;

pub(super) fn run(args: &InspectArgs, output: &mut dyn Write) -> anyhow::Result<()> {
    orchestrate(args, output, &mut ProductionSteps)
}

trait InspectSteps {
    type Corpus;
    type Id;
    type Unit;
    type Parser;
    type Context<'a>
    where
        Self: 'a;
    type Trace;

    fn load_corpus(&mut self, path: &Path) -> anyhow::Result<Self::Corpus>;
    fn validate_id(&mut self, id: &str) -> anyhow::Result<Self::Id>;
    fn resolve(&mut self, corpus: &Self::Corpus, id: &Self::Id) -> anyhow::Result<Self::Unit>;
    fn unit_bytes(&self, unit: &Self::Unit) -> usize;
    fn load_parser(&mut self) -> anyhow::Result<Self::Parser>;
    fn context<'a>(&mut self, unit: &'a Self::Unit) -> anyhow::Result<Self::Context<'a>>;
    fn trace(
        &mut self,
        parser: &Self::Parser,
        unit: &Self::Unit,
        context: &Self::Context<'_>,
        limits: TraceLimits,
    ) -> Self::Trace;
    fn map(&mut self, unit: &Self::Unit, trace: &Self::Trace) -> DiagnosticReport;
    fn render(
        &mut self,
        report: &DiagnosticReport,
        json: bool,
        output: &mut dyn Write,
    ) -> anyhow::Result<()>;
    fn flush(&mut self, output: &mut dyn Write) -> anyhow::Result<()>;
    fn render_packed(
        &mut self,
        _trace: &Self::Trace,
        _json: bool,
        _output: &mut dyn Write,
    ) -> anyhow::Result<()> {
        Ok(())
    }
}

struct ProductionSteps;

impl InspectSteps for ProductionSteps {
    type Corpus = Corpus;
    type Id = ValidatedCorpusId;
    type Unit = CorpusUnit;
    type Parser = Parser;
    type Context<'a> = ParseContext<'a>;
    type Trace = ParserTrace<OracleText>;

    fn load_corpus(&mut self, path: &Path) -> anyhow::Result<Self::Corpus> {
        Corpus::load(path, &crate::raw_corpus::CorpusSelectionArgs::all())
    }

    fn validate_id(&mut self, id: &str) -> anyhow::Result<Self::Id> {
        ValidatedCorpusId::parse(id)
    }

    fn resolve(&mut self, corpus: &Self::Corpus, id: &Self::Id) -> anyhow::Result<Self::Unit> {
        corpus.resolve_exact(id).cloned()
    }

    fn unit_bytes(&self, unit: &Self::Unit) -> usize {
        unit.text().len()
    }

    fn load_parser(&mut self) -> anyhow::Result<Self::Parser> {
        crate::english_v2::parser_from_builtin_v2()
    }

    fn context<'a>(&mut self, unit: &'a Self::Unit) -> anyhow::Result<Self::Context<'a>> {
        ParseContext::new(
            unit.context_name(),
            unit.is_legendary(),
            unit.context_onset(),
        )
        .with_context(|| {
            format!(
                "stored corpus context invariant failed for exact ID {} and context {}",
                quoted(unit.id()),
                quoted(unit.context_name()),
            )
        })
    }

    fn trace(
        &mut self,
        parser: &Self::Parser,
        unit: &Self::Unit,
        context: &Self::Context<'_>,
        limits: TraceLimits,
    ) -> Self::Trace {
        parser.trace_oracle_text(unit.text(), context, limits)
    }

    fn map(&mut self, unit: &Self::Unit, trace: &Self::Trace) -> DiagnosticReport {
        DiagnosticReport::from_corpus(unit, trace)
    }

    fn render(
        &mut self,
        report: &DiagnosticReport,
        json: bool,
        output: &mut dyn Write,
    ) -> anyhow::Result<()> {
        diagnostic::render(report, json, output)
    }

    fn flush(&mut self, output: &mut dyn Write) -> anyhow::Result<()> {
        output.flush().context("flush English-v2 inspect output")
    }

    fn render_packed(
        &mut self,
        trace: &Self::Trace,
        json: bool,
        output: &mut dyn Write,
    ) -> anyhow::Result<()> {
        if json {
            return Ok(());
        }
        let packed_sites = trace.selected().map_or_else(Vec::new, packed::oracle_text);
        packed::write_human(&packed_sites, output)
    }
}

fn orchestrate<S: InspectSteps>(
    args: &InspectArgs,
    output: &mut dyn Write,
    steps: &mut S,
) -> anyhow::Result<()> {
    let wall_started = std::time::Instant::now();
    let corpus = steps.load_corpus(&args.data)?;
    let id = steps.validate_id(&args.id)?;
    let unit = steps.resolve(&corpus, &id)?;
    let parser = steps.load_parser()?;
    let context = steps.context(&unit).with_context(|| {
        format!(
            "constructing stored corpus context from {}",
            args.data.display()
        )
    })?;
    let cpu_started = super::corpus::thread_cpu_time();
    let trace = steps.trace(&parser, &unit, &context, TraceLimits::new(args.limit));
    let cpu_elapsed = super::corpus::thread_cpu_time()
        .checked_sub(cpu_started)
        .expect("thread CPU clock is monotonic");
    let report = steps.map(&unit, &trace);
    steps.render(&report, args.json, output)?;
    steps.render_packed(&trace, args.json, output)?;
    steps.flush(output)?;
    super::corpus::write_corpus_performance(
        "inspect",
        wall_started.elapsed(),
        super::corpus::CorpusPerformance::for_unit_bytes(
            steps.unit_bytes(&unit),
            cpu_elapsed,
            report.accepted(),
        ),
    )?;

    if let Some((kind, message)) = report.internal_failure() {
        anyhow::bail!(
            "English-v2 inspect internal failure kind={}: {message}",
            kind.as_str()
        );
    }
    Ok(())
}

fn quoted(value: &str) -> String {
    serde_json::to_string(value).expect("serializing a string cannot fail")
}

#[cfg(test)]
mod tests {
    use std::io;
    use std::io::Write;
    use std::path::Path;
    use std::path::PathBuf;

    use anyhow::Context;
    use anyhow::ensure;
    use deckmaste_english_v2::context::ParseContext;
    use deckmaste_english_v2::parser::ParserEntryPoint;
    use deckmaste_english_v2::parser::TraceLimits;
    use deckmaste_english_v2::parser::reset_parser_entry_calls_for_test;
    use deckmaste_english_v2::parser::take_parser_entry_calls_for_test;
    use serde_json::Value;
    use tempfile::tempdir;

    use super::*;
    use crate::english_v2::InspectArgs;
    use crate::english_v2::ProbeArgs;
    use crate::english_v2::corpus::Corpus;
    use crate::english_v2::corpus::CorpusUnit;
    use crate::english_v2::corpus::ValidatedCorpusId;
    use crate::english_v2::diagnostic;
    use crate::english_v2::diagnostic::DiagnosticReport;
    use crate::english_v2::diagnostic::FixtureOutcome;
    use crate::english_v2::diagnostic::fixture_report;
    use crate::english_v2::probe;

    const VALID_ID: &str = "0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef";
    const SYNTHETIC_COVERAGE_LOCK_MEMBER: &str =
        "eeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeee";

    fn args(id: &str) -> InspectArgs {
        InspectArgs {
            id: id.to_owned(),
            data: PathBuf::from("fixture-oracle-cards.jsonl"),
            limit: 1,
            json: true,
        }
    }

    fn unit(card: &str, face: Option<&str>, side: Option<&str>, text: &str) -> CorpusUnit {
        CorpusUnit::for_test_with_metadata(card, face, side, face.unwrap_or(card), text)
    }

    #[test]
    fn exact_id_validation_rejects_every_substitute_before_lookup() {
        let invalid = [
            "A".repeat(64),
            format!("{}A", "0".repeat(63)),
            "g".repeat(64),
            "0".repeat(63),
            "0".repeat(65),
            format!(" {VALID_ID}"),
            format!("{VALID_ID} "),
            format!("0x{VALID_ID}"),
            VALID_ID[..32].to_owned(),
            "Fixture Card".to_owned(),
            "17".to_owned(),
            "Destroy target creature.".to_owned(),
        ];

        for offending in invalid {
            let error = ValidatedCorpusId::parse(&offending).unwrap_err();
            let message = error.to_string();
            assert!(message.contains("exactly 64 lowercase hexadecimal bytes"));
            assert!(message.contains(&serde_json::to_string(&offending).unwrap()));
        }
    }

    #[test]
    fn exact_id_resolution_distinguishes_missing_and_duplicate_and_preserves_the_row() {
        let first = unit(
            "Card // Other",
            Some("Face\nName"),
            Some("a\rside"),
            "Choose one —\n• Destroy target creature.\n(Reminder text.)",
        );
        let second = unit("Second Card", None, None, "Destroy target creature.");
        let corpus = Corpus::from_units_for_test(vec![second, first.clone()]);
        let id = ValidatedCorpusId::parse(first.id()).unwrap();

        assert_eq!(corpus.resolve_exact(&id).unwrap(), &first);

        let missing = ValidatedCorpusId::parse(SYNTHETIC_COVERAGE_LOCK_MEMBER).unwrap();
        let error = corpus.resolve_exact(&missing).unwrap_err().to_string();
        assert!(error.contains("no corpus unit has exact ID"));
        assert!(error.contains(missing.as_str()));

        let duplicate = Corpus::from_units_for_test(vec![first.clone(), first.clone()]);
        let error = duplicate.resolve_exact(&id).unwrap_err().to_string();
        assert!(error.contains("multiple corpus units have exact ID"));
        assert!(error.contains(first.id()));
    }

    #[test]
    fn probe_and_inspect_share_the_exact_trace_payload_for_complete_documents_and_limits() {
        let parser = crate::english_v2::parser_from_builtin_v2().unwrap();
        for (text, context) in [
            ("Destroy target creature.", "Accepted Card"),
            (
                "Destroy target Forest.\nSecond complete line with (the Fridge).",
                "Failed Card",
            ),
        ] {
            let unit = unit(context, None, None, text);
            let normalized = unit.text();
            let parse_context = ParseContext::new(
                unit.context_name(),
                unit.is_legendary(),
                unit.context_onset(),
            )
            .unwrap();
            for limit in [0, 1, 256] {
                let probe_trace = parser.trace(normalized, &parse_context, TraceLimits::new(limit));
                let inspect_trace =
                    parser.trace(normalized, &parse_context, TraceLimits::new(limit));
                let probe = DiagnosticReport::from_probe(normalized, context, &probe_trace);
                let inspect = DiagnosticReport::from_corpus(&unit, &inspect_trace);

                assert_eq!(probe.trace_payload(), inspect.trace_payload());

                let mut probe_json = Vec::new();
                diagnostic::render(&probe, true, &mut probe_json).unwrap();
                let mut inspect_json = Vec::new();
                diagnostic::render(&inspect, true, &mut inspect_json).unwrap();
                let probe_value: Value = serde_json::from_slice(&probe_json).unwrap();
                let inspect_value: Value = serde_json::from_slice(&inspect_json).unwrap();
                assert_eq!(probe_value["trace"], inspect_value["trace"]);
                assert_eq!(inspect_value["source"]["text"], normalized);
                assert_eq!(inspect_value["source"]["context"], context);
            }
        }
    }

    struct RecordingSteps {
        events: Vec<&'static str>,
        unit: CorpusUnit,
        outcome: FixtureOutcome,
        fail_at: Option<&'static str>,
        traced_text: Vec<String>,
        traced_context: Vec<String>,
        limits: Vec<usize>,
    }

    impl RecordingSteps {
        fn new(outcome: FixtureOutcome) -> Self {
            Self {
                events: Vec::new(),
                unit: unit(
                    "Stored Card",
                    Some("Stored\nContext"),
                    Some("a\rside"),
                    "Complete first line.\nComplete second line (the Fridge).",
                ),
                outcome,
                fail_at: None,
                traced_text: Vec::new(),
                traced_context: Vec::new(),
                limits: Vec::new(),
            }
        }

        fn fail(&self, point: &str) -> anyhow::Result<()> {
            ensure!(self.fail_at != Some(point), "fixture {point} failure");
            Ok(())
        }
    }

    impl InspectSteps for RecordingSteps {
        type Corpus = ();
        type Id = String;
        type Unit = CorpusUnit;
        type Parser = ();
        type Context<'a> = ();
        type Trace = ();

        fn load_corpus(&mut self, _path: &Path) -> anyhow::Result<Self::Corpus> {
            self.events.push("corpus");
            self.fail("corpus")
        }

        fn validate_id(&mut self, id: &str) -> anyhow::Result<Self::Id> {
            self.events.push("validate");
            self.fail("validate")?;
            Ok(id.to_owned())
        }

        fn resolve(
            &mut self,
            _corpus: &Self::Corpus,
            _id: &Self::Id,
        ) -> anyhow::Result<Self::Unit> {
            self.events.push("resolve");
            self.fail("resolve")?;
            Ok(self.unit.clone())
        }

        fn unit_bytes(&self, unit: &Self::Unit) -> usize {
            unit.text().len()
        }

        fn load_parser(&mut self) -> anyhow::Result<Self::Parser> {
            self.events.push("parser");
            self.fail("parser")
        }

        fn context<'a>(&mut self, _unit: &'a Self::Unit) -> anyhow::Result<Self::Context<'a>> {
            self.events.push("context");
            self.fail("context")
        }

        fn trace(
            &mut self,
            _parser: &Self::Parser,
            unit: &Self::Unit,
            _context: &Self::Context<'_>,
            limits: TraceLimits,
        ) -> Self::Trace {
            self.events.push("trace");
            self.traced_text.push(unit.text().to_owned());
            self.traced_context.push(unit.context_name().to_owned());
            self.limits.push(limits.per_collection());
        }

        fn map(&mut self, unit: &Self::Unit, _trace: &Self::Trace) -> DiagnosticReport {
            self.events.push("map");
            fixture_report(self.outcome).with_corpus_source_for_test(unit)
        }

        fn render(
            &mut self,
            report: &DiagnosticReport,
            json: bool,
            output: &mut dyn Write,
        ) -> anyhow::Result<()> {
            self.events.push("render");
            self.fail("render")?;
            diagnostic::render(report, json, output)
        }

        fn flush(&mut self, output: &mut dyn Write) -> anyhow::Result<()> {
            self.events.push("flush");
            self.fail("flush")?;
            output.flush().context("fixture flush")
        }
    }

    #[test]
    fn orchestration_is_exactly_once_in_order_and_traces_stored_input_unchanged() {
        let mut steps = RecordingSteps::new(FixtureOutcome::Selected);
        let expected_text = steps.unit.text().to_owned();
        let expected_context = steps.unit.context_name().to_owned();
        orchestrate(&args(VALID_ID), &mut Vec::new(), &mut steps).unwrap();

        assert_eq!(
            steps.events,
            [
                "corpus", "validate", "resolve", "parser", "context", "trace", "map", "render",
                "flush",
            ]
        );
        assert_eq!(steps.traced_text, [expected_text]);
        assert_eq!(steps.traced_context, [expected_context]);
        assert_eq!(steps.limits, [1]);
    }

    #[test]
    fn every_pretrace_failure_stops_at_its_exact_precedence_point() {
        let ordered = ["corpus", "validate", "resolve", "parser", "context"];
        for (index, point) in ordered.iter().enumerate() {
            let mut steps = RecordingSteps::new(FixtureOutcome::Selected);
            steps.fail_at = Some(point);
            let error = orchestrate(&args(VALID_ID), &mut Vec::new(), &mut steps).unwrap_err();
            assert!(error.to_string().contains(point), "{error:#}");
            assert_eq!(steps.events, ordered[..=index]);
        }
    }

    #[test]
    fn ordinary_outcomes_succeed_but_internal_outcomes_flush_one_json_then_error() {
        for outcome in [
            FixtureOutcome::ParseFailure,
            FixtureOutcome::UnresolvedAmbiguity,
        ] {
            let mut steps = RecordingSteps::new(outcome);
            orchestrate(&args(VALID_ID), &mut Vec::new(), &mut steps).unwrap();
        }

        for outcome in [
            FixtureOutcome::ValidatedRootDidNotMaterialize,
            FixtureOutcome::SelectionConfiguration,
            FixtureOutcome::OwnershipInspection,
        ] {
            let mut steps = RecordingSteps::new(outcome);
            let mut output = Vec::new();
            let error = orchestrate(&args(VALID_ID), &mut output, &mut steps).unwrap_err();
            let value: Value = serde_json::from_slice(&output).unwrap();
            assert_eq!(value["source"]["kind"], "corpus");
            assert_eq!(value["source"]["id"], steps.unit.id());
            assert_eq!(value["trace"]["outcome"]["status"], "internal_failure");
            assert!(
                !String::from_utf8(output)
                    .unwrap()
                    .contains(&error.to_string())
            );
            assert_eq!(steps.events.last(), Some(&"flush"));
        }
    }

    struct FailingWriter {
        fail_flush: bool,
    }

    impl Write for FailingWriter {
        fn write(&mut self, bytes: &[u8]) -> io::Result<usize> {
            if self.fail_flush {
                Ok(bytes.len())
            } else {
                Err(io::Error::new(
                    io::ErrorKind::BrokenPipe,
                    "fixture write failure",
                ))
            }
        }

        fn flush(&mut self) -> io::Result<()> {
            if self.fail_flush {
                Err(io::Error::new(
                    io::ErrorKind::BrokenPipe,
                    "fixture flush failure",
                ))
            } else {
                Ok(())
            }
        }
    }

    #[test]
    fn write_and_flush_failures_precede_internal_exit_status() {
        for fail_flush in [false, true] {
            let mut steps = RecordingSteps::new(FixtureOutcome::SelectionConfiguration);
            let error = orchestrate(
                &args(VALID_ID),
                &mut FailingWriter { fail_flush },
                &mut steps,
            )
            .unwrap_err();
            let expected = if fail_flush { "flush" } else { "write" };
            assert!(error.to_string().contains(expected), "{error:#}");
        }
    }

    const ONE_ROW: &[u8] = br#"{"object":"card","id":"printing-fixture","oracle_id":"oracle-fixture","name":"Card\nName","layout":"modal_dfc","legalities":{"vintage":"legal"},"card_faces":[{"name":"Seven Dwarves","type_line":"Creature","oracle_text":"Destroy target Forest.\nSecond complete line (the Fridge)."}]}
"#;

    #[test]
    fn real_runner_preserves_metadata_is_line_safe_and_matches_probe_trace() {
        let temp = tempdir().unwrap();
        let data = temp.path().join("oracle-cards.jsonl");
        std::fs::write(&data, ONE_ROW).unwrap();
        let corpus = Corpus::load(&data, &crate::raw_corpus::CorpusSelectionArgs::all()).unwrap();
        let unit = corpus.units().first().unwrap();
        let mut inspect_args = args(unit.id());
        inspect_args.data = data;
        inspect_args.limit = 0;

        reset_parser_entry_calls_for_test();
        let mut inspect_json = Vec::new();
        run(&inspect_args, &mut inspect_json).unwrap();
        assert_eq!(
            take_parser_entry_calls_for_test(),
            [(
                ParserEntryPoint::TraceOracleText,
                "Destroy target Forest.\nSecond complete line.".to_owned(),
                "Seven Dwarves".to_owned(),
            )],
            "inspect must enter the real OracleText trace exactly once"
        );
        let inspect_value: Value = serde_json::from_slice(&inspect_json).unwrap();
        assert_eq!(inspect_value["source"]["kind"], "corpus");
        assert_eq!(inspect_value["source"]["id"], unit.id());
        assert_eq!(inspect_value["source"]["card"], unit.card_name());
        assert_eq!(inspect_value["source"]["face"], unit.face_name().unwrap());
        assert_eq!(inspect_value["source"]["side"], unit.side().unwrap());
        assert_eq!(inspect_value["source"]["context"], unit.context_name());
        assert_eq!(inspect_value["source"]["text"], unit.text());
        assert_eq!(inspect_value["root"], "OracleText");

        let probe_args = ProbeArgs {
            text: unit.text().to_owned(),
            context: unit.context_name().to_owned(),
            onset: unit.context_onset().into(),
            legendary: unit.is_legendary(),
            limit: 0,
            root: crate::english_v2::ProbeRoot::Ability,
            json: true,
        };
        let mut probe_json = Vec::new();
        probe::run(&probe_args, &mut probe_json).unwrap();
        let probe_value: Value = serde_json::from_slice(&probe_json).unwrap();
        assert_eq!(probe_value["root"], "Ability");

        inspect_args.json = false;
        let mut human = Vec::new();
        run(&inspect_args, &mut human).unwrap();
        let human = String::from_utf8(human).unwrap();
        let header = human.lines().next().unwrap();
        assert!(header.contains("source_kind=corpus"));
        assert!(header.contains(unit.id()));
        for stored in [
            unit.card_name(),
            unit.face_name().unwrap(),
            unit.side().unwrap(),
            unit.context_name(),
            unit.text(),
        ] {
            assert!(
                header.contains(&serde_json::to_string(stored).unwrap()),
                "header omitted line-safe stored value {stored:?}: {header}"
            );
        }
        assert!(!header.contains('\r'));
        assert_eq!(std::fs::read(&inspect_args.data).unwrap(), ONE_ROW);
    }
}
