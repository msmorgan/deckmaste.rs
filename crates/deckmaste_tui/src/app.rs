use std::process::ExitCode;

/// Entry point. Real driving/rendering arrives in later tasks; for now this
/// proves the binary is wired as the workspace run target.
#[must_use]
pub fn run() -> ExitCode {
    let headless = std::env::args().skip(1).any(|a| a == "--headless");
    if headless {
        println!("deckmaste_tui: scaffold (headless stub)");
    } else {
        println!("deckmaste_tui: scaffold (interactive UI not yet wired)");
    }
    ExitCode::SUCCESS
}
