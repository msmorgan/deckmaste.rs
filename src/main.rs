mod bootstrap;
use std::process::ExitCode;

fn main() -> ExitCode {
    if let Err(e) = bootstrap::ensure_ready() {
        eprintln!("error: {e}");
        return ExitCode::FAILURE;
    }
    deckmaste_tui::run()
}
