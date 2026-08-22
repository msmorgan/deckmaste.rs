use std::ffi::OsString;
use std::io::Write;
use std::path::PathBuf;
use std::process::Command;
use std::process::Stdio;
use std::time::Duration;
use std::time::Instant;

use anyhow::anyhow;
use anyhow::bail;
use clap::Args;
use clap::ValueEnum;

const HARD_CEILING: Duration = Duration::from_millis(16_260);

#[derive(Debug, Args)]
pub(super) struct PlanGateArgs {
    #[arg(long, value_enum)]
    profile: PlanProfile,
    #[arg(long, value_enum)]
    gate: PlanGate,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, ValueEnum)]
enum PlanProfile {
    #[value(name = "07")]
    Plan07,
}

impl PlanProfile {
    const fn name(self) -> &'static str {
        match self {
            Self::Plan07 => "07",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, ValueEnum)]
enum PlanGate {
    Expand,
    Report,
    Parse,
    Roundtrip,
    Ambiguity,
    Coverage,
    RequireComplete,
}

impl PlanGate {
    const fn name(self) -> &'static str {
        match self {
            Self::Expand => "expand",
            Self::Report => "report",
            Self::Parse => "parse",
            Self::Roundtrip => "roundtrip",
            Self::Ambiguity => "ambiguity",
            Self::Coverage => "coverage",
            Self::RequireComplete => "require-complete",
        }
    }

    const fn warning_threshold(self) -> Duration {
        match self {
            Self::Expand => Duration::from_millis(300),
            Self::Report => Duration::from_millis(225),
            Self::Parse => Duration::from_millis(13_245),
            Self::Roundtrip => Duration::from_millis(12_240),
            Self::Ambiguity => Duration::from_millis(12_960),
            Self::Coverage => Duration::from_millis(13_290),
            Self::RequireComplete => Duration::from_secs(12),
        }
    }

    fn command(self) -> GateCommand {
        let args = match self {
            Self::Expand => &["xtask", "english_v2", "expand"][..],
            Self::Report => &["xtask", "english_v2", "report", "--json"][..],
            Self::Parse => &["xtask", "english_v2", "parse", "--json"][..],
            Self::Roundtrip => &["xtask", "english_v2", "roundtrip", "--require-clean"][..],
            Self::Ambiguity => &["xtask", "english_v2", "ambiguity", "--require-resolved"][..],
            Self::Coverage => &["xtask", "english_v2", "coverage", "--check"][..],
            Self::RequireComplete => &["xtask", "english_v2", "parse", "--require-complete"][..],
        };
        GateCommand::new("cargo", args.iter().copied())
    }

    const fn expects_incomplete_failure(self) -> bool {
        matches!(self, Self::RequireComplete)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct GateCommand {
    program: PathBuf,
    args: Vec<OsString>,
}

impl GateCommand {
    fn new<P, I, S>(program: P, args: I) -> Self
    where
        P: Into<PathBuf>,
        I: IntoIterator<Item = S>,
        S: Into<OsString>,
    {
        Self {
            program: program.into(),
            args: args.into_iter().map(Into::into).collect(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct ChildOutcome {
    success: bool,
    exit_code: Option<i32>,
}

impl ChildOutcome {
    #[cfg(test)]
    const fn from_exit_code(exit_code: Option<i32>) -> Self {
        Self {
            success: matches!(exit_code, Some(0)),
            exit_code,
        }
    }

    fn exit_description(self) -> String {
        self.exit_code
            .map_or_else(|| "signal".to_owned(), |code| code.to_string())
    }
}

trait Runner {
    fn run(&mut self, command: &GateCommand) -> anyhow::Result<ChildOutcome>;
}

struct ProcessRunner;

impl Runner for ProcessRunner {
    fn run(&mut self, command: &GateCommand) -> anyhow::Result<ChildOutcome> {
        let status = Command::new(&command.program)
            .args(&command.args)
            .stdin(Stdio::inherit())
            .stdout(Stdio::inherit())
            .stderr(Stdio::inherit())
            .status()?;
        Ok(ChildOutcome {
            success: status.success(),
            exit_code: status.code(),
        })
    }
}

trait Clock {
    fn now(&mut self) -> Duration;
}

struct WallClock {
    origin: Instant,
}

impl WallClock {
    fn start() -> Self {
        Self {
            origin: Instant::now(),
        }
    }
}

impl Clock for WallClock {
    fn now(&mut self) -> Duration {
        self.origin.elapsed()
    }
}

pub(super) fn run(args: &PlanGateArgs, output: &mut dyn Write) -> anyhow::Result<()> {
    let mut runner = ProcessRunner;
    let mut clock = WallClock::start();
    run_with(args, output, &mut runner, &mut clock)
}

fn run_with(
    args: &PlanGateArgs,
    output: &mut dyn Write,
    runner: &mut impl Runner,
    clock: &mut impl Clock,
) -> anyhow::Result<()> {
    let command = args.gate.command();
    let started = clock.now();
    let child = runner.run(&command);
    let finished = clock.now();
    let elapsed = finished.checked_sub(started).ok_or_else(|| {
        anyhow!(
            "english-v2-plan07-clock-failure gate={}: elapsed clock moved backwards",
            args.gate.name(),
        )
    })?;

    writeln!(
        output,
        "SAMPLE english-v2-plan07-elapsed gate={} profile={} elapsed_seconds={}",
        args.gate.name(),
        args.profile.name(),
        seconds(elapsed),
    )?;
    let warning = args.gate.warning_threshold();
    if elapsed >= warning {
        writeln!(
            output,
            "WARNING english-v2-plan07-relative-slowdown gate={} elapsed_seconds={} warning_seconds={}",
            args.gate.name(),
            seconds(elapsed),
            seconds(warning),
        )?;
    }
    output.flush()?;

    if elapsed > HARD_CEILING {
        bail!(
            "english-v2-plan07-timing-failure gate={} elapsed_seconds={} hard_ceiling_seconds={}",
            args.gate.name(),
            seconds(elapsed),
            seconds(HARD_CEILING),
        );
    }

    let child = child.map_err(|error| {
        anyhow!(
            "english-v2-plan07-child-launch-failure gate={}: {error:#}",
            args.gate.name(),
        )
    })?;
    if !child.success {
        bail!(
            "english-v2-plan07-child-failure gate={} exit_code={} expected_incomplete={}",
            args.gate.name(),
            child.exit_description(),
            args.gate.expects_incomplete_failure(),
        );
    }
    Ok(())
}

fn seconds(duration: Duration) -> String {
    format!("{}.{:09}", duration.as_secs(), duration.subsec_nanos())
}

#[cfg(test)]
mod tests {
    use std::cell::RefCell;
    use std::collections::VecDeque;
    use std::ffi::OsString;
    use std::io::Write as _;
    use std::process::Command as ProcessCommand;
    use std::rc::Rc;
    use std::time::Duration;

    use super::*;

    const ADAPTER_HELPER_TEST: &str = "english_v2::timing::tests::process_runner_adapter_helper";
    const ADAPTER_OUTPUT_CHILD_TEST: &str =
        "english_v2::timing::tests::process_runner_child_writes_both_streams";
    const ADAPTER_FAILURE_CHILD_TEST: &str =
        "english_v2::timing::tests::process_runner_child_exits_17";

    struct FakeClock {
        samples: VecDeque<Duration>,
    }

    impl Clock for FakeClock {
        fn now(&mut self) -> Duration {
            self.samples.pop_front().expect("a clock sample remains")
        }
    }

    struct FakeRunner {
        result: Option<anyhow::Result<ChildOutcome>>,
        observed: Vec<GateCommand>,
    }

    impl Runner for FakeRunner {
        fn run(&mut self, command: &GateCommand) -> anyhow::Result<ChildOutcome> {
            self.observed.push(command.clone());
            self.result.take().expect("one child result remains")
        }
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    enum TraceEvent {
        ClockBefore,
        Runner,
        ClockAfter,
    }

    struct TraceClock {
        trace: Rc<RefCell<Vec<TraceEvent>>>,
        calls: usize,
    }

    impl Clock for TraceClock {
        fn now(&mut self) -> Duration {
            let event =
                if self.calls == 0 { TraceEvent::ClockBefore } else { TraceEvent::ClockAfter };
            self.calls += 1;
            self.trace.borrow_mut().push(event);
            Duration::from_millis(self.calls as u64)
        }
    }

    struct TraceRunner {
        trace: Rc<RefCell<Vec<TraceEvent>>>,
    }

    impl Runner for TraceRunner {
        fn run(&mut self, _command: &GateCommand) -> anyhow::Result<ChildOutcome> {
            self.trace.borrow_mut().push(TraceEvent::Runner);
            Ok(success())
        }
    }

    fn args(gate: PlanGate) -> PlanGateArgs {
        PlanGateArgs {
            profile: PlanProfile::Plan07,
            gate,
        }
    }

    fn exercise(
        gate: PlanGate,
        elapsed: Duration,
        result: anyhow::Result<ChildOutcome>,
    ) -> (anyhow::Result<()>, String, Vec<GateCommand>) {
        let mut clock = FakeClock {
            samples: VecDeque::from([Duration::ZERO, elapsed]),
        };
        let mut runner = FakeRunner {
            result: Some(result),
            observed: vec![],
        };
        let mut output = Vec::new();
        let result = run_with(&args(gate), &mut output, &mut runner, &mut clock);
        (
            result,
            String::from_utf8(output).expect("gate output is UTF-8"),
            runner.observed,
        )
    }

    fn success() -> ChildOutcome {
        ChildOutcome::from_exit_code(Some(0))
    }

    fn exact_test_invocation(name: &str) -> bool {
        std::env::args_os().skip(1).eq([
            OsString::from("--exact"),
            OsString::from(name),
            OsString::from("--nocapture"),
        ])
    }

    fn test_binary_command(name: &str) -> GateCommand {
        GateCommand::new(
            std::env::current_exe().expect("the current test executable has a path"),
            ["--exact", name, "--nocapture"],
        )
    }

    #[test]
    fn process_runner_child_writes_both_streams() {
        if !exact_test_invocation(ADAPTER_OUTPUT_CHILD_TEST) {
            return;
        }
        let mut stdout = std::io::stdout().lock();
        writeln!(stdout, "PLAN07_PROCESS_RUNNER_CHILD_STDOUT").unwrap();
        stdout.flush().unwrap();
        let mut stderr = std::io::stderr().lock();
        writeln!(stderr, "PLAN07_PROCESS_RUNNER_CHILD_STDERR").unwrap();
        stderr.flush().unwrap();
    }

    #[test]
    fn process_runner_child_exits_17() {
        if exact_test_invocation(ADAPTER_FAILURE_CHILD_TEST) {
            std::process::exit(17);
        }
    }

    #[test]
    fn process_runner_adapter_helper() {
        if !exact_test_invocation(ADAPTER_HELPER_TEST) {
            return;
        }
        let outcome = ProcessRunner
            .run(&test_binary_command(ADAPTER_OUTPUT_CHILD_TEST))
            .expect("the real child launches");
        assert_eq!(outcome, ChildOutcome::from_exit_code(Some(0)));
        println!("PLAN07_PROCESS_RUNNER_AFTER_CHILD");
    }

    #[test]
    fn process_runner_uses_exact_argv_waits_and_inherits_both_output_streams() {
        let output = ProcessCommand::new(
            std::env::current_exe().expect("the current test executable has a path"),
        )
        .args(["--exact", ADAPTER_HELPER_TEST, "--nocapture"])
        .output()
        .expect("the adapter helper launches");
        assert!(
            output.status.success(),
            "adapter helper failed:\nstdout:\n{}\nstderr:\n{}",
            String::from_utf8_lossy(&output.stdout),
            String::from_utf8_lossy(&output.stderr),
        );

        let stdout = String::from_utf8(output.stdout).expect("helper stdout is UTF-8");
        let stderr = String::from_utf8(output.stderr).expect("helper stderr is UTF-8");
        let child = stdout
            .find("PLAN07_PROCESS_RUNNER_CHILD_STDOUT")
            .expect("exact child argv selected the stdout helper");
        let after = stdout
            .find("PLAN07_PROCESS_RUNNER_AFTER_CHILD")
            .expect("adapter returned after its child");
        assert!(child < after, "adapter returned before child completion");
        assert!(stderr.contains("PLAN07_PROCESS_RUNNER_CHILD_STDERR"));
        assert!(!stdout.contains("PLAN07_PROCESS_RUNNER_CHILD_STDERR"));
        assert!(!stderr.contains("PLAN07_PROCESS_RUNNER_CHILD_STDOUT"));
    }

    struct RealFailureRunner {
        process: ProcessRunner,
        command: GateCommand,
    }

    impl Runner for RealFailureRunner {
        fn run(&mut self, requested: &GateCommand) -> anyhow::Result<ChildOutcome> {
            assert_eq!(requested, &PlanGate::RequireComplete.command());
            self.process.run(&self.command)
        }
    }

    #[test]
    fn require_complete_marks_a_real_nonzero_child_as_expected_incomplete() {
        let mut runner = RealFailureRunner {
            process: ProcessRunner,
            command: test_binary_command(ADAPTER_FAILURE_CHILD_TEST),
        };
        let mut clock = FakeClock {
            samples: VecDeque::from([Duration::ZERO, Duration::from_millis(1)]),
        };
        let mut output = Vec::new();

        let error = run_with(
            &args(PlanGate::RequireComplete),
            &mut output,
            &mut runner,
            &mut clock,
        )
        .expect_err("the real nonzero child remains a distinct child failure");

        assert!(error.to_string().contains("exit_code=17"));
        assert!(error.to_string().contains("expected_incomplete=true"));
        assert!(!error.to_string().contains("timing-failure"));
        assert!(
            String::from_utf8(output)
                .unwrap()
                .contains("gate=require-complete")
        );
    }

    #[test]
    fn run_with_measures_clock_before_and_after_the_complete_runner_call() {
        let trace = Rc::new(RefCell::new(Vec::new()));
        let mut clock = TraceClock {
            trace: Rc::clone(&trace),
            calls: 0,
        };
        let mut runner = TraceRunner {
            trace: Rc::clone(&trace),
        };
        let mut output = Vec::new();

        run_with(
            &args(PlanGate::Expand),
            &mut output,
            &mut runner,
            &mut clock,
        )
        .expect("the traced successful child passes");

        assert_eq!(
            *trace.borrow(),
            [
                TraceEvent::ClockBefore,
                TraceEvent::Runner,
                TraceEvent::ClockAfter,
            ],
        );
    }

    #[test]
    fn each_gate_runs_its_exact_real_child_command() {
        let cases = [
            (PlanGate::Expand, &["xtask", "english_v2", "expand"][..]),
            (
                PlanGate::Report,
                &["xtask", "english_v2", "report", "--json"][..],
            ),
            (
                PlanGate::Parse,
                &["xtask", "english_v2", "parse", "--json"][..],
            ),
            (
                PlanGate::Roundtrip,
                &["xtask", "english_v2", "roundtrip", "--require-clean"][..],
            ),
            (
                PlanGate::Ambiguity,
                &["xtask", "english_v2", "ambiguity", "--require-resolved"][..],
            ),
            (
                PlanGate::Coverage,
                &["xtask", "english_v2", "coverage", "--check"][..],
            ),
            (
                PlanGate::RequireComplete,
                &["xtask", "english_v2", "parse", "--require-complete"][..],
            ),
        ];

        for (gate, expected_args) in cases {
            let (result, _, observed) = exercise(gate, Duration::ZERO, Ok(success()));
            result.expect("successful child below the ceiling passes");
            assert_eq!(observed.len(), 1);
            assert_eq!(observed[0].program, PathBuf::from("cargo"));
            assert_eq!(
                observed[0].args,
                expected_args.iter().map(OsString::from).collect::<Vec<_>>(),
            );
        }
    }

    #[test]
    fn warning_boundaries_use_each_exact_plan07_threshold() {
        let cases = [
            (PlanGate::Expand, 300),
            (PlanGate::Report, 225),
            (PlanGate::Parse, 13_245),
            (PlanGate::Roundtrip, 12_240),
            (PlanGate::Ambiguity, 12_960),
            (PlanGate::Coverage, 13_290),
            (PlanGate::RequireComplete, 12_000),
        ];

        for (gate, threshold_ms) in cases {
            let (_, below, _) =
                exercise(gate, Duration::from_millis(threshold_ms - 1), Ok(success()));
            assert!(
                !below.contains("WARNING"),
                "{gate:?} warned below threshold"
            );

            let (_, exact, _) = exercise(gate, Duration::from_millis(threshold_ms), Ok(success()));
            assert!(
                exact.contains("WARNING english-v2-plan07-relative-slowdown"),
                "{gate:?} omitted its exact-threshold warning",
            );

            let (_, above, _) =
                exercise(gate, Duration::from_millis(threshold_ms + 1), Ok(success()));
            assert!(
                above.contains("WARNING english-v2-plan07-relative-slowdown"),
                "{gate:?} omitted its above-threshold warning",
            );
        }
    }

    #[test]
    fn hard_ceiling_allows_exactly_16_26_seconds_and_rejects_above_it() {
        let (exact_result, exact_output, _) = exercise(
            PlanGate::Expand,
            Duration::from_millis(16_260),
            Ok(success()),
        );
        exact_result.expect("the hard ceiling is inclusive");
        assert!(exact_output.contains("elapsed_seconds=16.260"));

        let (above_result, above_output, _) = exercise(
            PlanGate::Expand,
            Duration::from_millis(16_261),
            Ok(success()),
        );
        let error = above_result.expect_err("above the hard ceiling fails");
        assert!(
            error
                .to_string()
                .contains("english-v2-plan07-timing-failure")
        );
        assert!(!error.to_string().contains("child-failure"));
        assert!(above_output.contains("elapsed_seconds=16.261"));
    }

    #[test]
    fn elapsed_render_preserves_the_first_nanosecond_above_the_hard_ceiling() {
        let elapsed = HARD_CEILING + Duration::from_nanos(1);
        assert_eq!(seconds(elapsed), "16.260000001");

        let (result, output, _) = exercise(PlanGate::Expand, elapsed, Ok(success()));
        assert!(
            result
                .expect_err("one nanosecond above the ceiling fails")
                .to_string()
                .contains("elapsed_seconds=16.260000001")
        );
        assert!(output.contains("elapsed_seconds=16.260000001"));
    }

    #[test]
    fn child_failure_is_reported_after_the_elapsed_sample_without_becoming_timing_failure() {
        let (result, output, _) = exercise(
            PlanGate::Parse,
            Duration::from_secs(1),
            Ok(ChildOutcome::from_exit_code(Some(23))),
        );
        let error = result.expect_err("a failed ordinary child fails the wrapper");
        assert!(
            error
                .to_string()
                .contains("english-v2-plan07-child-failure")
        );
        assert!(error.to_string().contains("exit_code=23"));
        assert!(error.to_string().contains("expected_incomplete=false"));
        assert!(!error.to_string().contains("timing-failure"));
        assert!(output.contains("SAMPLE english-v2-plan07-elapsed gate=parse"));
    }

    #[test]
    fn require_complete_failure_is_explicitly_marked_as_the_expected_incomplete_probe() {
        let (result, output, _) = exercise(
            PlanGate::RequireComplete,
            Duration::from_secs(1),
            Ok(ChildOutcome::from_exit_code(Some(1))),
        );
        let error = result.expect_err("the child exit remains observable");
        assert!(
            error
                .to_string()
                .contains("english-v2-plan07-child-failure")
        );
        assert!(error.to_string().contains("expected_incomplete=true"));
        assert!(output.contains("gate=require-complete"));
    }

    #[test]
    fn launch_failure_still_prints_the_named_elapsed_sample() {
        let (result, output, _) = exercise(
            PlanGate::Report,
            Duration::from_millis(10),
            Err(anyhow::anyhow!("sentinel spawn refusal")),
        );
        let error = result.expect_err("launch failure propagates");
        assert!(
            error
                .to_string()
                .contains("english-v2-plan07-child-launch-failure")
        );
        assert!(error.to_string().contains("sentinel spawn refusal"));
        assert!(output.contains("SAMPLE english-v2-plan07-elapsed gate=report"));
    }
}
