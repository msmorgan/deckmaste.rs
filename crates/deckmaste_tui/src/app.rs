use std::process::ExitCode;

use anyhow::Result;
use clap::Parser;
use deckmaste_engine::Action;
use deckmaste_engine::Decision;
use deckmaste_engine::GameState;
use deckmaste_engine::PendingDecision;
use deckmaste_engine::sim::GreedyDemo;
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
use crate::shortcuts::PassMode;
use crate::shortcuts::PassState;
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

/// The demo client's command-line options.
#[derive(Parser)]
#[command(version, about = "Deckmaste demo — a Goblins-vs-Elves hotseat game")]
struct Cli {
    /// Auto-play the demo to completion instead of opening the interactive UI.
    #[arg(long)]
    headless: bool,
    /// Pin the shuffle to a specific seed. Omit for a fresh entropy seed each
    /// run (printed so a game can be replayed with `--seed`).
    #[arg(long)]
    seed: Option<u64>,
}

fn try_run() -> Result<()> {
    let cli = Cli::parse();
    // Reproducible on demand: `--seed N` pins the shuffle, otherwise draw a
    // fresh entropy seed so each run differs. The chosen value is announced so
    // a game can be replayed with `--seed`.
    let seed = cli.seed.unwrap_or_else(entropy_seed);

    // `GreedyDemo`, not `GreedyCreatures`: headless auto-resolves *every*
    // decision via the strategy (including the demo's targeted burn), so the
    // strategy must answer priority/targeting — `GreedyCreatures` punts those.
    let mut driver = Driver::new(game::build_game_with_seed(seed)?, Box::new(GreedyDemo));

    if cli.headless {
        eprintln!("shuffle seed: {seed} (pass --seed {seed} to replay)");
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
    // The alternate screen swallowed any startup print, so announce the seed on
    // the restored terminal — that's where the player can read it to replay.
    eprintln!("shuffle seed: {seed} (pass --seed {seed} to replay)");
    result
}

/// A best-effort entropy seed from the wall clock — enough to vary the demo's
/// shuffle from run to run (`ChaCha8` diffuses it). The value is printed so the
/// game can be reproduced with `--seed`.
fn entropy_seed() -> u64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .ok()
        .and_then(|d| u64::try_from(d.as_nanos()).ok())
        .unwrap_or(0)
}

#[expect(
    clippy::too_many_lines,
    reason = "event loop + key-dispatch match; extracting key handling \
              tracked in refactor-oversized-fns"
)]
fn interactive_loop(terminal: &mut DefaultTerminal, driver: &mut Driver) -> Result<()> {
    let mut board = BoardState::new();
    let mut pass = PassState::new();
    let mut stop = driver.advance(&mut pass)?;
    let mut current = interaction_for(&stop, &driver.state);
    let mut error: Option<String> = None;
    let mut help = false;
    // When a fresh pick-step opens, land the cursor on a legal candidate so the
    // player can act immediately (esp. blockers, whose legal blockers sit on
    // their own battlefield, away from the just-declared attackers).
    let mut steer_pending = true;

    loop {
        board.sync(&driver.state);
        let view = driver.state.layers();
        if std::mem::take(&mut steer_pending)
            && let Some(it) = current.as_ref()
            && let Some(&first) = it.candidates().first()
        {
            board.steer_to(first, &driver.state, &view);
        }
        terminal.draw(|frame| {
            ui::render(
                frame,
                &driver.state,
                &view,
                &board,
                &stop,
                current.as_ref(),
                error.as_deref(),
                &pass,
                help,
            );
        })?;

        let Event::Key(key) = event::read()? else { continue };
        if key.kind != KeyEventKind::Press {
            continue;
        }
        // The help overlay is modal: the next keypress dismisses it.
        if help {
            help = false;
            continue;
        }
        if key.code == KeyCode::Char('q') {
            break Ok(());
        }
        if key.code == KeyCode::Char('?') {
            help = true;
            continue;
        }
        // The cursor's object, if the focused selection is one.
        let cursor = match board.selected(&driver.state, &view) {
            Some(Selected::Object(id)) => Some(id),
            _ => None,
        };

        // Direct zone hotkeys (b/o/h/s/g/e) jump focus, except under the
        // ability popup where letter keys would be ambiguous (arrows drive it).
        let popup_open = matches!(
            current.as_ref(),
            Some(Interaction::Priority { sub: Some(_) })
        );
        if !popup_open
            && let KeyCode::Char(c) = key.code
            && let Some(zone) = ui::zone_for_key(c, board.perspective)
        {
            board.focus_zone(zone);
            continue;
        }

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
            // Pass is `a`, not Space — Space is the giant easy-to-fat-finger key
            // and still toggles selections in the pick modes below, so binding
            // priority-pass off it stops accidental advances.
            Some(Interaction::Priority { sub: None }) => match key.code {
                KeyCode::Char('a') | KeyCode::F(2) => submit = Some(Decision::Act(Action::Pass)),
                KeyCode::Char('y') | KeyCode::F(4) => {
                    pass.arm(board.perspective, PassMode::Yield, &driver.state);
                    submit = Some(Decision::Act(Action::Pass));
                }
                KeyCode::Char('P') | KeyCode::F(6) => {
                    pass.arm(board.perspective, PassMode::Turn, &driver.state);
                    submit = Some(Decision::Act(Action::Pass));
                }
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
            // ---- Attackers / Discard (toggle a subset of the dimmed board,
            //      then submit; the cursor's object is the one toggled) ----
            Some(it @ (Interaction::Attackers { .. } | Interaction::Discard { .. })) => {
                match key.code {
                    KeyCode::Char(' ') => {
                        if let Some(id) = cursor {
                            it.toggle(id);
                        }
                    }
                    KeyCode::Enter => submit = it.confirm(),
                    KeyCode::Esc => it.cancel(),
                    _ => {}
                }
            }
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
                            // Pairing just started: steer to the live attackers
                            // (which aren't in `candidates()`).
                            if matches!(
                                it,
                                Interaction::Blockers {
                                    pending: Some(_),
                                    ..
                                }
                            ) && let Some(&atk) = driver.state.combat.attackers().first()
                            {
                                board.steer_to(atk, &driver.state, &view);
                            }
                        }
                    }
                    KeyCode::Enter => {
                        if pairing {
                            if let Some(id) = cursor {
                                it.pair_with(id);
                                // Back to the defender's remaining blockers.
                                if let Some(&next) = it.candidates().first() {
                                    board.steer_to(next, &driver.state, &view);
                                }
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
            match driver.submit_and_advance(decision, &mut pass) {
                Ok(next) => {
                    stop = next;
                    current = interaction_for(&stop, &driver.state);
                    error = None;
                    steer_pending = true;
                }
                Err(e) => error = Some(e.to_string()),
            }
        }
    }
}

/// The interaction to drive for a stop (None for game-over / budget). A discard
/// picker's candidate set is the player's hand, read from `state`; every other
/// kind is built straight from the pending decision.
fn interaction_for(stop: &Stop, state: &GameState) -> Option<Interaction> {
    match stop {
        Stop::Decision(
            PendingDecision::DiscardToHandSize { player, count }
            | PendingDecision::DiscardCards { player, count },
        ) => Some(Interaction::for_discard(
            &state.zones.hands[player.index()],
            *count as usize,
        )),
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
