use std::process::ExitCode;

use anyhow::Result;
use deckmaste_engine::Action;
use deckmaste_engine::Decision;
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
use crate::interact;
use crate::interact::AbilityPick;
use crate::interact::Interaction;
use crate::ui;
use crate::ui::BoardState;
use crate::ui::Selected;

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

#[allow(clippy::too_many_lines)]
fn interactive_loop(terminal: &mut DefaultTerminal, driver: &mut Driver) -> Result<()> {
    let mut board = BoardState::new();
    let mut stop = driver.run_to_decision()?;
    let mut current = interaction_for(&stop);
    let mut error: Option<String> = None;

    loop {
        board.sync(&driver.state);
        let view = driver.state.layers();
        terminal.draw(|frame| {
            ui::render(
                frame,
                &driver.state,
                &view,
                &board,
                &stop,
                current.as_ref(),
                error.as_deref(),
            );
        })?;

        let Event::Key(key) = event::read()? else { continue };
        if key.kind != KeyEventKind::Press {
            continue;
        }
        if key.code == KeyCode::Char('q') {
            break Ok(());
        }
        // The cursor's object, if the focused selection is one.
        let cursor = match board.selected(&driver.state, &view) {
            Some(Selected::Object(id)) => Some(id),
            _ => None,
        };

        // Shared navigation (Tab / arrows). When the ability popup is open the
        // arrows move its selection instead of the board. Each arm `continue`s,
        // ending the `current.as_mut()` borrow immediately.
        match key.code {
            KeyCode::Tab => {
                board.cycle_zone(true);
                continue;
            }
            KeyCode::BackTab => {
                board.cycle_zone(false);
                continue;
            }
            KeyCode::Up | KeyCode::Left => {
                if let Some(Interaction::Priority { sub: Some(pick) }) = current.as_mut() {
                    pick.sel = pick.sel.saturating_sub(1);
                } else {
                    board.step_selection(false, board.focused_len(&driver.state, &view));
                }
                continue;
            }
            KeyCode::Down | KeyCode::Right => {
                if let Some(Interaction::Priority { sub: Some(pick) }) = current.as_mut() {
                    if pick.sel + 1 < pick.actions.len() {
                        pick.sel += 1;
                    }
                } else {
                    board.step_selection(true, board.focused_len(&driver.state, &view));
                }
                continue;
            }
            _ => {}
        }

        // Main dispatch. Own `current` for the duration so arms may mutate it
        // through the bound `&mut` without aliasing; `submit`/`replace` are
        // applied AFTER the match (see the borrow-safety contract above).
        let mut cur = current.take();
        let mut submit: Option<Decision> = None;
        let mut replace: Option<Interaction> = None;
        match cur.as_mut() {
            // ---- Priority, ability popup open ----
            Some(Interaction::Priority { sub: Some(pick) }) => match key.code {
                KeyCode::Enter => submit = Some(Decision::Act(pick.actions[pick.sel].clone())),
                KeyCode::Esc => replace = Some(Interaction::Priority { sub: None }),
                _ => {}
            },
            // ---- Priority, object-first ----
            Some(Interaction::Priority { sub: None }) => match key.code {
                KeyCode::Char(' ') => submit = Some(Decision::Act(Action::Pass)),
                KeyCode::Enter => match (cursor, priority_legal(&stop)) {
                    (Some(id), Some(legal)) => {
                        let acts = interact::actions_for(id, legal);
                        match acts.len() {
                            0 => error = Some("no legal action for that card".to_string()),
                            1 => submit = Some(Decision::Act(acts[0].clone())),
                            _ => {
                                replace = Some(Interaction::Priority {
                                    sub: Some(AbilityPick {
                                        object: id,
                                        actions: acts,
                                        sel: 0,
                                    }),
                                });
                            }
                        }
                    }
                    _ => error = Some("select a card or permanent first".to_string()),
                },
                _ => {}
            },
            // ---- Targets ----
            Some(it @ Interaction::Targets { .. }) => match key.code {
                KeyCode::Char(' ') => {
                    if let Some(id) = cursor {
                        it.toggle(id);
                    }
                }
                KeyCode::Enter => {
                    if let Some(d) = it.confirm() {
                        submit = Some(d);
                    } else {
                        it.advance();
                    }
                }
                KeyCode::Esc => it.cancel(),
                _ => {}
            },
            // ---- Attackers ----
            Some(it @ Interaction::Attackers { .. }) => match key.code {
                KeyCode::Char(' ') => {
                    if let Some(id) = cursor {
                        it.toggle(id);
                    }
                }
                KeyCode::Enter => submit = it.confirm(),
                KeyCode::Esc => it.cancel(),
                _ => {}
            },
            // ---- Blockers ----
            Some(it @ Interaction::Blockers { .. }) => {
                let pairing = matches!(
                    it,
                    Interaction::Blockers {
                        pending: Some(_),
                        ..
                    }
                );
                match key.code {
                    KeyCode::Char(' ') if !pairing => {
                        if let Some(id) = cursor {
                            it.toggle(id);
                        }
                    }
                    KeyCode::Enter => {
                        if pairing {
                            if let Some(id) = cursor {
                                it.pair_with(id);
                            }
                        } else if let Some(d) = it.confirm() {
                            submit = Some(d);
                        }
                    }
                    KeyCode::Backspace => it.unpair_last(),
                    KeyCode::Esc => it.cancel(),
                    _ => {}
                }
            }
            None => {}
        }
        // Borrow of `cur` from `as_mut()` has ended here.
        current = replace.or(cur);

        if let Some(decision) = submit {
            match driver.submit(decision) {
                Ok(next) => {
                    stop = next;
                    current = interaction_for(&stop);
                    error = None;
                }
                Err(e) => error = Some(e.to_string()),
            }
        }
    }
}

/// The interaction to drive for a stop (None for game-over / budget).
fn interaction_for(stop: &Stop) -> Option<Interaction> {
    match stop {
        Stop::Decision(pending) => Interaction::for_decision(pending),
        Stop::GameOver(_) | Stop::Budget => None,
    }
}

/// The legal priority action list, if the stop is a priority decision.
fn priority_legal(stop: &Stop) -> Option<&[Action]> {
    match stop {
        Stop::Decision(PendingDecision::Priority { legal, .. }) => Some(legal),
        _ => None,
    }
}
