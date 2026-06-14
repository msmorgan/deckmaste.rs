use std::process::ExitCode;

use anyhow::Result;
use deckmaste_engine::PendingDecision;
use deckmaste_engine::sim::GreedyCreatures;
use ratatui::DefaultTerminal;
use ratatui::crossterm::event::Event;
use ratatui::crossterm::event::KeyCode;
use ratatui::crossterm::event::KeyEventKind;
use ratatui::crossterm::event::{self};

use crate::driver::Driver;
use crate::driver::HEADLESS_BUDGET;
use crate::driver::Stop;
use crate::game;
use crate::ui;
use crate::ui::BoardState;

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
        let stop = driver.run_to_end(HEADLESS_BUDGET)?;
        match stop {
            Stop::GameOver(outcome) => println!("game over: {outcome:?}"),
            Stop::Budget => println!("step budget reached without a result"),
            Stop::Decision(_) => unreachable!("headless auto-resolves every decision"),
        }
        return Ok(());
    }

    // `ratatui::init` enters raw mode + the alternate screen and installs a
    // panic hook that restores the terminal; `ratatui::restore` undoes it.
    let mut terminal = ratatui::init();
    let result = interactive_loop(&mut terminal, &mut driver);
    ratatui::restore();
    result
}

fn interactive_loop(terminal: &mut DefaultTerminal, driver: &mut Driver) -> Result<()> {
    let mut board = BoardState::new();
    let mut stop = driver.run_to_decision()?;
    loop {
        board.sync(&driver.state);
        let view = driver.state.layers();
        terminal.draw(|frame| ui::render(frame, &driver.state, &view, &board, &stop))?;

        let Event::Key(key) = event::read()? else {
            continue;
        };
        if key.kind != KeyEventKind::Press {
            continue;
        }
        match key.code {
            KeyCode::Char('q') => break Ok(()),
            KeyCode::Tab => board.cycle_zone(true),
            KeyCode::BackTab => board.cycle_zone(false),
            KeyCode::Up | KeyCode::Left => {
                board.step_selection(false, board.focused_len(&driver.state, &view));
            }
            KeyCode::Down | KeyCode::Right => {
                board.step_selection(true, board.focused_len(&driver.state, &view));
            }
            KeyCode::Char(' ') => {
                if let Stop::Decision(PendingDecision::Priority { .. }) = &stop {
                    driver.pass()?;
                    stop = driver.run_to_decision()?;
                }
            }
            KeyCode::Enter => {
                if let Stop::Decision(pending) = &stop {
                    let pending = pending.clone();
                    driver.auto(&pending)?;
                    stop = driver.run_to_decision()?;
                }
            }
            _ => {}
        }
    }
}
