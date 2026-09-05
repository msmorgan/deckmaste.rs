use std::io::Write;

use anyhow::Context;
use anyhow::ensure;
use deckmaste_english_v2::ast::OracleText;
use deckmaste_english_v2::ast::Sentence;
use deckmaste_english_v2::context::ParseContext;
use deckmaste_english_v2::parser::Parser;
use deckmaste_english_v2::parser::ParserTrace;
use deckmaste_english_v2::parser::TraceLimits;

use super::ProbeArgs;
use super::ProbeOnset;
use super::ProbeRoot;
use super::diagnostic;
use super::diagnostic::DiagnosticReport;
use super::packed;

pub(super) fn run(args: &ProbeArgs, output: &mut dyn Write) -> anyhow::Result<()> {
    orchestrate(args, output, &mut ProductionSteps)
}

trait ProbeSteps {
    type Parser;
    type Context<'a>;
    type Trace;

    fn load_parser(&mut self) -> anyhow::Result<Self::Parser>;
    fn context<'a>(
        &mut self,
        name: &'a str,
        is_legendary: bool,
        onset: ProbeOnset,
    ) -> anyhow::Result<Self::Context<'a>>;
    fn trace(
        &mut self,
        parser: &Self::Parser,
        text: &str,
        context: &Self::Context<'_>,
        limits: TraceLimits,
        root: ProbeRoot,
    ) -> Self::Trace;
    fn map(&mut self, text: &str, context: &str, trace: &Self::Trace) -> DiagnosticReport;
    fn render(
        &mut self,
        report: &DiagnosticReport,
        json: bool,
        output: &mut dyn Write,
    ) -> anyhow::Result<()>;
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

enum ProductionTrace {
    Ability(ParserTrace),
    Sentence(ParserTrace<Sentence>),
    OracleText(ParserTrace<OracleText>),
}

impl ProbeSteps for ProductionSteps {
    type Parser = Parser;
    type Context<'a> = ParseContext<'a>;
    type Trace = ProductionTrace;

    fn load_parser(&mut self) -> anyhow::Result<Self::Parser> {
        crate::english_v2::parser_from_builtin_v2()
    }

    fn context<'a>(
        &mut self,
        name: &'a str,
        is_legendary: bool,
        onset: ProbeOnset,
    ) -> anyhow::Result<Self::Context<'a>> {
        anyhow::ensure!(
            !name.is_empty(),
            "invalid --context \"\"; expected a nonempty parser context"
        );
        ParseContext::new(name, is_legendary, onset.into()).ok_or_else(|| {
            anyhow::anyhow!(
                "invalid --context {}; expected a nonempty parser context",
                quoted(name)
            )
        })
    }

    fn trace(
        &mut self,
        parser: &Self::Parser,
        text: &str,
        context: &Self::Context<'_>,
        limits: TraceLimits,
        root: ProbeRoot,
    ) -> Self::Trace {
        match root {
            ProbeRoot::Ability => Self::Trace::Ability(parser.trace(text, context, limits)),
            ProbeRoot::Sentence => {
                Self::Trace::Sentence(parser.trace_sentence(text, context, limits))
            }
            ProbeRoot::OracleText => {
                Self::Trace::OracleText(parser.trace_oracle_text(text, context, limits))
            }
        }
    }

    fn map(&mut self, text: &str, context: &str, trace: &Self::Trace) -> DiagnosticReport {
        match trace {
            ProductionTrace::Ability(trace) => DiagnosticReport::from_probe(text, context, trace),
            ProductionTrace::Sentence(trace) => DiagnosticReport::from_probe(text, context, trace),
            ProductionTrace::OracleText(trace) => {
                DiagnosticReport::from_probe(text, context, trace)
            }
        }
    }

    fn render(
        &mut self,
        report: &DiagnosticReport,
        json: bool,
        output: &mut dyn Write,
    ) -> anyhow::Result<()> {
        diagnostic::render(report, json, output)
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
        let packed_sites = match trace {
            ProductionTrace::Ability(trace) => {
                trace.selected().map_or_else(Vec::new, packed::ability)
            }
            ProductionTrace::Sentence(trace) => {
                trace.selected().map_or_else(Vec::new, packed::sentence)
            }
            ProductionTrace::OracleText(trace) => {
                trace.selected().map_or_else(Vec::new, packed::oracle_text)
            }
        };
        packed::write_human(&packed_sites, output)
    }
}

fn orchestrate<S: ProbeSteps>(
    args: &ProbeArgs,
    output: &mut dyn Write,
    steps: &mut S,
) -> anyhow::Result<()> {
    ensure!(
        !args.text.is_empty() || args.root == ProbeRoot::OracleText,
        "invalid --text: value must be nonempty"
    );

    let parser = steps.load_parser()?;
    let context = steps.context(&args.context, args.legendary, args.onset)?;
    let trace = steps.trace(
        &parser,
        &args.text,
        &context,
        TraceLimits::new(args.limit),
        args.root,
    );
    let report = steps.map(&args.text, &args.context, &trace);
    steps.render(&report, args.json, output)?;
    steps.render_packed(&trace, args.json, output)?;
    output.flush().context("flush English-v2 probe output")?;

    if let Some((kind, message)) = report.internal_failure() {
        anyhow::bail!(
            "English-v2 probe internal failure kind={}: {message}",
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

    use deckmaste_english_v2::parser::ParserEntryPoint;
    use deckmaste_english_v2::parser::reset_parser_entry_calls_for_test;
    use deckmaste_english_v2::parser::take_parser_entry_calls_for_test;
    use serde_json::Value;

    use super::*;
    use crate::english_v2::diagnostic::FixtureOutcome;
    use crate::english_v2::diagnostic::fixture_report;

    fn args(text: &str, context: &str) -> ProbeArgs {
        ProbeArgs {
            text: text.to_owned(),
            context: context.to_owned(),
            onset: ProbeOnset::Consonant,
            legendary: false,
            limit: 1,
            root: ProbeRoot::Ability,
            json: true,
        }
    }

    #[test]
    fn real_runner_emits_one_json_document_for_selected_and_parse_failure() {
        for text in ["Destroy target creature.", "Destroy target Forest"] {
            let mut output = Vec::new();
            run(&args(text, "Probe Card"), &mut output).unwrap();
            let report: Value = serde_json::from_slice(&output).unwrap();
            assert_eq!(report["schema_version"], 2);
            assert_eq!(report["source"]["kind"], "probe");
            assert_eq!(report["source"]["text"], text);
            assert_eq!(report["source"]["context"], "Probe Card");
            assert!(report["trace"].get("tokens").is_none());
            assert!(report["trace"].get("scanner_matches").is_some());
            assert!(report["trace"].get("selected_lexical_claims").is_some());
            if text == "Destroy target creature." {
                assert_eq!(report["trace"]["ownership"]["covered"], true);
                assert_eq!(
                    report["trace"]["ownership"]["failures"],
                    serde_json::json!([]),
                );
            } else {
                assert!(report["trace"]["ownership"].is_null());
            }
        }
    }

    #[test]
    fn real_runner_enters_exactly_one_public_trace_for_each_typed_root() {
        let text = "Destroy target creature.";
        let context = "Probe Card";
        for (root, expected_entry) in [
            (ProbeRoot::Ability, ParserEntryPoint::TraceAbility),
            (ProbeRoot::Sentence, ParserEntryPoint::TraceSentence),
            (ProbeRoot::OracleText, ParserEntryPoint::TraceOracleText),
        ] {
            let mut arguments = args(text, context);
            arguments.root = root;
            reset_parser_entry_calls_for_test();

            run(&arguments, &mut Vec::new()).unwrap();

            assert_eq!(
                take_parser_entry_calls_for_test(),
                [(expected_entry, text.to_owned(), context.to_owned())],
                "probe must enter exactly the one public trace selected by --root"
            );
        }
    }

    #[test]
    fn empty_text_reaches_only_the_oracle_text_root_with_zero_claim_ownership() {
        let mut arguments = args("", "Grizzly Bears");
        arguments.root = ProbeRoot::OracleText;
        reset_parser_entry_calls_for_test();

        let mut output = Vec::new();
        run(&arguments, &mut output).unwrap();

        assert_eq!(
            take_parser_entry_calls_for_test(),
            [(
                ParserEntryPoint::TraceOracleText,
                String::new(),
                "Grizzly Bears".to_owned(),
            )],
        );
        let report: Value = serde_json::from_slice(&output).unwrap();
        assert_eq!(report["root"], "OracleText");
        assert_eq!(report["source"]["text"], "");
        assert_eq!(report["trace"]["outcome"]["status"], "selected");
        assert_eq!(report["trace"]["outcome"]["rendered"], "");
        assert_eq!(report["trace"]["selected_lexical_claims"]["total"], 0);
        let ownership = &report["trace"]["ownership"];
        assert_eq!(ownership["covered"], true);
        assert_eq!(ownership["failures"], serde_json::json!([]));
        for field in [
            "claims",
            "claimed_bytes",
            "form_literal_claims",
            "form_literal_bytes",
            "vocab_claims",
            "vocab_bytes",
            "lexeme_claims",
            "lexeme_bytes",
            "codec_claims",
            "codec_bytes",
            "identity_claims",
            "identity_bytes",
            "gap_spans",
            "gap_bytes",
            "overlap_spans",
            "overlap_bytes",
            "synthetic_claims",
            "provenance_plan_mismatches",
        ] {
            assert_eq!(ownership[field], 0, "{field}");
        }

        for root in [ProbeRoot::Ability, ProbeRoot::Sentence] {
            arguments.root = root;
            let error = run(&arguments, &mut Vec::new()).unwrap_err();
            assert!(error.to_string().contains("--text"), "{error:#}");
        }
    }

    #[test]
    fn validation_errors_are_context_rich_and_nonzero() {
        let error = run(&args("Destroy target creature.", ""), &mut Vec::new()).unwrap_err();
        assert!(error.to_string().contains("--context"), "{error:#}");
    }

    #[test]
    fn explicit_probe_metadata_admits_opaque_names_and_licenses_only_legendary_abbreviation() {
        let mut opaque = args("Destroy target creature.", ", Invalid");
        opaque.onset = ProbeOnset::Vowel;
        run(&opaque, &mut Vec::new()).expect("opaque spelling cannot reject a probe context");

        let text = "Zacama deals 3 damage to target creature.";
        let mut legendary = args(text, "Zacama, Primal Calamity");
        legendary.legendary = true;
        legendary.root = ProbeRoot::Sentence;
        let mut legendary_output = Vec::new();
        run(&legendary, &mut legendary_output).expect("licensed short form probe completes");
        let legendary_report: Value = serde_json::from_slice(&legendary_output).unwrap();
        assert_eq!(legendary_report["trace"]["outcome"]["status"], "selected");

        let mut ordinary = legendary;
        ordinary.legendary = false;
        let mut ordinary_output = Vec::new();
        run(&ordinary, &mut ordinary_output).expect("ordinary parse failure is diagnostic output");
        let ordinary_report: Value = serde_json::from_slice(&ordinary_output).unwrap();
        assert_eq!(
            ordinary_report["trace"]["outcome"]["status"],
            "parse_failure"
        );
    }

    #[derive(Default)]
    struct RecordingSteps {
        events: Vec<&'static str>,
        limits: Vec<usize>,
        outcome: FixtureOutcome,
    }

    impl ProbeSteps for RecordingSteps {
        type Parser = ();
        type Context<'a> = ();
        type Trace = ();

        fn load_parser(&mut self) -> anyhow::Result<Self::Parser> {
            self.events.push("load");
            Ok(())
        }

        fn context<'a>(
            &mut self,
            _name: &'a str,
            _is_legendary: bool,
            _onset: ProbeOnset,
        ) -> anyhow::Result<Self::Context<'a>> {
            self.events.push("context");
            Ok(())
        }

        fn trace(
            &mut self,
            _parser: &Self::Parser,
            _text: &str,
            _context: &Self::Context<'_>,
            limits: deckmaste_english_v2::parser::TraceLimits,
            root: ProbeRoot,
        ) -> Self::Trace {
            self.events.push(match root {
                ProbeRoot::Ability => "trace_ability",
                ProbeRoot::Sentence => "trace_sentence",
                ProbeRoot::OracleText => "trace_oracle_text",
            });
            self.limits.push(limits.per_collection());
        }

        fn map(
            &mut self,
            _text: &str,
            _context: &str,
            _trace: &Self::Trace,
        ) -> crate::english_v2::diagnostic::DiagnosticReport {
            self.events.push("map");
            fixture_report(self.outcome)
        }

        fn render(
            &mut self,
            report: &crate::english_v2::diagnostic::DiagnosticReport,
            json: bool,
            output: &mut dyn Write,
        ) -> anyhow::Result<()> {
            self.events.push("render");
            crate::english_v2::diagnostic::render(report, json, output)
        }
    }

    #[test]
    fn orchestration_runs_load_context_trace_map_render_once_in_order() {
        let mut steps = RecordingSteps {
            outcome: FixtureOutcome::Selected,
            ..RecordingSteps::default()
        };
        orchestrate(&args("text", "context"), &mut Vec::new(), &mut steps).unwrap();
        assert_eq!(
            steps.events,
            ["load", "context", "trace_ability", "map", "render"]
        );
        assert_eq!(steps.limits, [1]);
    }

    #[test]
    fn internal_outcomes_return_after_one_complete_json_without_appended_error_bytes() {
        for outcome in [
            FixtureOutcome::ValidatedRootDidNotMaterialize,
            FixtureOutcome::SelectionConfiguration,
            FixtureOutcome::OwnershipInspection,
        ] {
            let mut steps = RecordingSteps {
                outcome,
                ..RecordingSteps::default()
            };
            let mut output = Vec::new();
            let error = orchestrate(&args("text", "context"), &mut output, &mut steps).unwrap_err();
            let value: Value = serde_json::from_slice(&output).unwrap();
            assert_eq!(value["trace"]["outcome"]["status"], "internal_failure");
            assert!(
                !String::from_utf8(output)
                    .unwrap()
                    .contains(&error.to_string())
            );
            assert_eq!(
                steps.events,
                ["load", "context", "trace_ability", "map", "render"]
            );
        }
    }

    #[test]
    fn orchestration_dispatches_exactly_one_typed_root_for_every_outcome() {
        for root in [
            ProbeRoot::Ability,
            ProbeRoot::Sentence,
            ProbeRoot::OracleText,
        ] {
            for outcome in [
                FixtureOutcome::Selected,
                FixtureOutcome::ParseFailure,
                FixtureOutcome::UnresolvedAmbiguity,
                FixtureOutcome::ValidatedRootDidNotMaterialize,
                FixtureOutcome::SelectionConfiguration,
                FixtureOutcome::OwnershipInspection,
            ] {
                let mut arguments = args("text", "context");
                arguments.root = root;
                let mut steps = RecordingSteps {
                    outcome,
                    ..RecordingSteps::default()
                };
                let _ = orchestrate(&arguments, &mut Vec::new(), &mut steps);
                let trace_events = steps
                    .events
                    .iter()
                    .filter(|event| event.starts_with("trace_"))
                    .copied()
                    .collect::<Vec<_>>();
                assert_eq!(
                    trace_events,
                    [match root {
                        ProbeRoot::Ability => "trace_ability",
                        ProbeRoot::Sentence => "trace_sentence",
                        ProbeRoot::OracleText => "trace_oracle_text",
                    }]
                );
            }
        }
    }

    struct FailingWriter {
        fail_flush: bool,
    }

    impl Write for FailingWriter {
        fn write(&mut self, buffer: &[u8]) -> io::Result<usize> {
            if self.fail_flush {
                Ok(buffer.len())
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
    fn writer_failure_precedes_internal_status_at_the_actual_write_or_flush_point() {
        for fail_flush in [false, true] {
            let mut steps = RecordingSteps {
                outcome: FixtureOutcome::SelectionConfiguration,
                ..RecordingSteps::default()
            };
            let error = orchestrate(
                &args("text", "context"),
                &mut FailingWriter { fail_flush },
                &mut steps,
            )
            .unwrap_err();
            let expected = if fail_flush { "flush" } else { "write" };
            assert!(error.to_string().contains(expected), "{error:#}");
        }
    }
}
