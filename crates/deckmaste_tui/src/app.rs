use std::process::ExitCode;
use std::time::Duration;
use std::time::Instant;

use anyhow::Result;
use clap::Parser;
use deckmaste_engine::Action;
use deckmaste_engine::GameState;
use deckmaste_engine::LayeredView;
use deckmaste_engine::PendingDecision;
use deckmaste_engine::PlayerId;
use deckmaste_engine::sim::GreedyDemo;
use ratatui::DefaultTerminal;
use ratatui::crossterm::event::Event;
use ratatui::crossterm::event::KeyCode;
use ratatui::crossterm::event::KeyEvent;
use ratatui::crossterm::event::KeyEventKind;
use ratatui::crossterm::event::{self};

use crate::driver::Driver;
use crate::driver::HEADLESS_BUDGET;
use crate::driver::Stop;
use crate::game;
use crate::interact::Interaction;
use crate::interact::KeyCtx;
use crate::interact::KeyOutcome;
use crate::shortcuts::PassState;
use crate::ui;
use crate::ui::BoardState;
use crate::ui::Selected;

/// How long a pressed key remains highlighted in the footer legend.
const KEY_FLASH_DURATION: Duration = Duration::from_millis(250);

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

fn interactive_loop(terminal: &mut DefaultTerminal, driver: &mut Driver) -> Result<()> {
    let mut board = BoardState::new();
    let mut pass = PassState::new();
    let mut stop = driver.advance(&mut pass)?;
    let mut current = interaction_for(&stop, &driver.state);
    let mut error: Option<String> = None;
    let mut help = false;
    let mut key_flash: Option<(KeyCode, Instant)> = None;
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
            ui::render_with_key(
                frame,
                &driver.state,
                &view,
                &board,
                &stop,
                current.as_ref(),
                error.as_deref(),
                &pass,
                help,
                key_flash.as_ref().map(|(code, _)| *code),
                driver.provenance_refs(),
            );
        })?;

        let next_event = if let Some((_, until)) = key_flash.as_ref() {
            let remaining = until.saturating_duration_since(Instant::now());
            if !event::poll(remaining)? {
                key_flash = None;
                continue;
            }
            event::read()?
        } else {
            event::read()?
        };
        let Event::Key(key) = next_event else { continue };
        if key.kind != KeyEventKind::Press {
            continue;
        }
        key_flash = Some((key.code, Instant::now() + KEY_FLASH_DURATION));
        match handle_global_key(
            key,
            &mut board,
            &mut help,
            &mut current,
            &driver.state,
            &view,
        ) {
            GlobalKey::Quit => break Ok(()),
            GlobalKey::Consumed => continue,
            GlobalKey::Pass => {}
        }
        // The cursor's object, if the focused selection is one.
        let cursor = match board.selected(&driver.state, &view) {
            Some(Selected::Object(id)) => Some(id),
            _ => None,
        };

        // Main dispatch: delegate to the current interaction's own key
        // semantics ([`Interaction::on_key`]). Own `current` for the duration
        // so the call may mutate it through the bound `&mut` without
        // aliasing; `submit`/`replace` are applied AFTER the call (see the
        // borrow-safety contract above).
        let mut cur = current.take();
        let outcome = match cur.as_mut() {
            Some(it) => it.on_key(
                key,
                &mut KeyCtx {
                    board: &mut board,
                    pass: &mut pass,
                    state: &driver.state,
                    view: &view,
                    cursor,
                    legal: priority_legal(&stop),
                },
            ),
            None => KeyOutcome::default(),
        };
        let submit = outcome.submit;
        let replace = outcome.replace;
        // A hand card the player tried to cast with no mana floated: attempt an
        // auto-tap-then-cast after the match (multi-submit, so not `submit`).
        let autotap_cast = outcome.autotap_cast;
        if let Some(e) = outcome.error {
            error = Some(e);
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

        // Auto-tap-then-cast: the player pressed Enter on a hand card with no
        // legal action, which for an otherwise-castable spell means no mana is
        // floated. Tap the minimal set of untapped lands that covers its cost,
        // then cast — else keep the plain "no legal action" message.
        if let (Some(id), Some(player)) = (autotap_cast, priority_player(&stop)) {
            match driver.autotap_and_cast(player, id, &mut pass) {
                Ok(Some(next)) => {
                    stop = next;
                    current = interaction_for(&stop, &driver.state);
                    error = None;
                    steer_pending = true;
                }
                Ok(None) => error = Some("no legal action for that card".to_string()),
                Err(e) => error = Some(e.to_string()),
            }
        }
    }
}

/// How the loop should proceed after [`handle_global_key`].
enum GlobalKey {
    /// `q` was pressed — exit the loop.
    Quit,
    /// The key was fully handled here (e.g. a navigation key, or the help
    /// overlay's dismiss) — the loop should `continue` without reaching
    /// `Interaction::on_key`.
    Consumed,
    /// The key is none of the global ones — fall through to the cursor
    /// computation and the interaction's own dispatch.
    Pass,
}

/// Handle the window-global keys (help, quit, zone focus, navigation) that
/// take precedence over the active interaction. Returns how the loop should
/// proceed.
fn handle_global_key(
    key: KeyEvent,
    board: &mut BoardState,
    help: &mut bool,
    current: &mut Option<Interaction>,
    state: &GameState,
    view: &LayeredView,
) -> GlobalKey {
    // The help overlay is modal: the next keypress dismisses it.
    if *help {
        *help = false;
        return GlobalKey::Consumed;
    }
    if key.code == KeyCode::Char('q') {
        return GlobalKey::Quit;
    }
    if key.code == KeyCode::Char('?') {
        *help = true;
        return GlobalKey::Consumed;
    }

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
        return GlobalKey::Consumed;
    }

    // Shared navigation (Tab / arrows). When the ability popup is open the
    // arrows move its selection instead of the board. Each arm returns
    // `Consumed`, ending the `current.as_mut()` borrow immediately.
    match key.code {
        KeyCode::Tab => {
            board.cycle_zone(true);
            GlobalKey::Consumed
        }
        KeyCode::BackTab => {
            board.cycle_zone(false);
            GlobalKey::Consumed
        }
        KeyCode::Up | KeyCode::Left => {
            if let Some(Interaction::Priority { sub: Some(pick) }) = current.as_mut() {
                pick.sel = pick.sel.saturating_sub(1);
            } else {
                board.step_selection(false, board.focused_len(state, view));
            }
            GlobalKey::Consumed
        }
        KeyCode::Down | KeyCode::Right => {
            if let Some(Interaction::Priority { sub: Some(pick) }) = current.as_mut() {
                if pick.sel + 1 < pick.actions.len() {
                    pick.sel += 1;
                }
            } else {
                board.step_selection(true, board.focused_len(state, view));
            }
            GlobalKey::Consumed
        }
        _ => GlobalKey::Pass,
    }
}

/// The interaction to drive for a stop (None for game-over / budget). A discard
/// picker's candidate set is the player's hand, read from `state`; every other
/// kind is built straight from the pending decision.
fn interaction_for(stop: &Stop, state: &GameState) -> Option<Interaction> {
    match stop {
        Stop::Decision(
            PendingDecision::DiscardToHandSize(deckmaste_engine::DiscardToHandSize {
                player,
                count,
            })
            | PendingDecision::DiscardCards(deckmaste_engine::DiscardCards { player, count }),
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
        Stop::Decision(PendingDecision::Priority(deckmaste_engine::Priority { legal, .. })) => {
            Some(legal)
        }
        _ => None,
    }
}

/// The player holding priority, if the stop is a priority decision.
fn priority_player(stop: &Stop) -> Option<PlayerId> {
    match stop {
        Stop::Decision(PendingDecision::Priority(deckmaste_engine::Priority {
            player, ..
        })) => Some(*player),
        _ => None,
    }
}
