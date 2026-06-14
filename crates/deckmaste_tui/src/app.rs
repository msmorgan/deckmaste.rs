use std::process::ExitCode;

use anyhow::Result;
use deckmaste_engine::sim::GreedyCreatures;

use crate::driver::Driver;
use crate::driver::HEADLESS_BUDGET;
use crate::driver::Stop;
use crate::game;

/// Binary entry point. Prints a clean error and exits non-zero on failure.
#[must_use]
pub fn run() -> ExitCode {
    match try_run() {
        Ok(()) => ExitCode::SUCCESS,
        Err(err) => {
            eprintln!("error: {err:#}");
            ExitCode::FAILURE
        }
    }
}

fn try_run() -> Result<()> {
    let headless = std::env::args().skip(1).any(|a| a == "--headless");
    let mut driver = Driver::new(game::build_game()?, Box::new(GreedyCreatures));

    if headless {
        // `DecisionError: std::error::Error`, so `?` lifts it into anyhow.
        let stop = driver.run_to_end(HEADLESS_BUDGET)?;
        match stop {
            Stop::GameOver(outcome) => println!("game over: {outcome:?}"),
            Stop::Budget => println!("step budget reached without a result"),
            Stop::Priority(_) => unreachable!("headless auto-resolves priority"),
        }
        return Ok(());
    }

    // Interactive ratatui UI is wired in Task 5.
    println!("interactive UI not yet wired; run with --headless");
    Ok(())
}
