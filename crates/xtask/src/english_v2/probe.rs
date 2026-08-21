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
use super::ProbeRoot;
use super::diagnostic;
use super::diagnostic::DiagnosticReport;

pub(super) fn run(args: &ProbeArgs, output: &mut dyn Write) -> anyhow::Result<()> {
    orchestrate(args, output, &mut ProductionSteps)
}

trait ProbeSteps {
    type Parser;
    type Context<'a>;
    type Trace;

    fn load_parser(&mut self) -> anyhow::Result<Self::Parser>;
    fn context<'a>(&mut self, name: &'a str) -> anyhow::Result<Self::Context<'a>>;
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
        Ok(crate::english_v2::parser_from_builtin_v2())
    }

    fn context<'a>(&mut self, name: &'a str) -> anyhow::Result<Self::Context<'a>> {
        ParseContext::new(name).with_context(|| {
            format!(
                "invalid --context {}; expected a nonempty parser context with a nonempty comma abbreviation",
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
}

fn orchestrate<S: ProbeSteps>(
    args: &ProbeArgs,
    output: &mut dyn Write,
    steps: &mut S,
) -> anyhow::Result<()> {
    ensure!(
        !args.text.is_empty(),
        "invalid --text: value must be nonempty"
    );

    let parser = steps.load_parser()?;
    let context = steps.context(&args.context)?;
    let trace = steps.trace(
        &parser,
        &args.text,
        &context,
        TraceLimits::new(args.limit),
        args.root,
    );
    let report = steps.map(&args.text, &args.context, &trace);
    steps.render(&report, args.json, output)?;
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

    use serde_json::Value;

    use super::*;
    use crate::english_v2::diagnostic::FixtureOutcome;
    use crate::english_v2::diagnostic::fixture_report;

    fn args(text: &str, context: &str) -> ProbeArgs {
        ProbeArgs {
            text: text.to_owned(),
            context: context.to_owned(),
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
    fn validation_errors_are_context_rich_and_nonzero() {
        for (text, context, needle) in [
            ("", "Probe Card", "--text"),
            ("Destroy target creature.", "", "--context"),
            ("Destroy target creature.", ", Invalid", "--context"),
        ] {
            let error = run(&args(text, context), &mut Vec::new()).unwrap_err();
            assert!(error.to_string().contains(needle), "{error:#}");
        }
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

        fn context<'a>(&mut self, _name: &'a str) -> anyhow::Result<Self::Context<'a>> {
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
