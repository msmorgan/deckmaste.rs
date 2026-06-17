//! Board-view layout and ratatui glue. The only module that touches `Frame`,
//! `Layout`, `List`, `Block`. Pure helpers live in `board`/`zones`/`format`/
//! `detail`.
mod board;
mod detail;
mod format;
mod zones;

use std::fmt::Write as _;

pub use board::BoardState;
pub use board::Selected;
pub use board::zone_for_key;
use deckmaste_engine::GameState;
use deckmaste_engine::LayeredView;
use deckmaste_engine::PlayerId;
use ratatui::Frame;
use ratatui::layout::Constraint;
use ratatui::layout::Layout;
use ratatui::layout::Rect;
use ratatui::style::Color;
use ratatui::style::Modifier;
use ratatui::style::Style;
use ratatui::text::Line;
use ratatui::text::Span;
use ratatui::widgets::Block;
use ratatui::widgets::BorderType;
use ratatui::widgets::List;
use ratatui::widgets::ListItem;
use ratatui::widgets::ListState;
use ratatui::widgets::Paragraph;
use ratatui::widgets::Wrap;

use crate::driver::Stop;
use crate::shortcuts::PassMode;
use crate::shortcuts::PassState;
use crate::ui::board::Zone;

/// A human-readable name for a turn phase/step (the header otherwise shows the
/// `Debug` form, e.g. `Beginning(Upkeep)`).
fn phase_name(phase: deckmaste_core::Phase) -> &'static str {
    use deckmaste_core::BeginningStep as B;
    use deckmaste_core::CombatStep as C;
    use deckmaste_core::EndingStep as E;
    use deckmaste_core::Phase as P;
    match phase {
        P::Beginning(B::Untap) => "Untap",
        P::Beginning(B::Upkeep) => "Upkeep",
        P::Beginning(B::Draw) => "Draw",
        P::PrecombatMain => "Main 1",
        P::Combat(C::BeginningOfCombat) => "Begin Combat",
        P::Combat(C::DeclareAttackers) => "Declare Attackers",
        P::Combat(C::DeclareBlockers) => "Declare Blockers",
        P::Combat(C::FirstCombatDamage) => "First-Strike Damage",
        P::Combat(C::CombatDamage) => "Combat Damage",
        P::Combat(C::EndOfCombat) => "End Combat",
        P::PostcombatMain => "Main 2",
        P::Ending(E::End) => "End Step",
        P::Ending(E::Cleanup) => "Cleanup",
    }
}

/// Stable per-seat accent color, so "who controls this" reads at a glance and
/// stays put as the perspective flips.
fn player_color(p: PlayerId) -> Color {
    match p.0 {
        0 => Color::Cyan,
        _ => Color::Magenta,
    }
}

/// A zone's accent color and whether it belongs to the controlled (perspective)
/// player. Battlefields are colored by their owner; the hand and graveyard are
/// always the perspective player's; the stack and exile are shared.
fn zone_accent(zone: Zone, perspective: PlayerId) -> (Color, bool) {
    match zone {
        Zone::Battlefield(p) => (player_color(p), p == perspective),
        Zone::Hand | Zone::Graveyard => (player_color(perspective), true),
        Zone::Stack | Zone::Exile => (Color::Gray, false),
    }
}

/// The style for one board row. Pick-mode selection state wins (chosen rows are
/// green, non-candidates dimmed); otherwise combat role colors the row
/// (attackers red, blockers yellow). `None` leaves the row at the default
/// style.
fn row_style(
    sel: Selected,
    state: &GameState,
    pick_mode: bool,
    chosen: bool,
    candidate: bool,
) -> Option<Style> {
    if pick_mode {
        if chosen {
            return Some(Style::new().fg(Color::Green).add_modifier(Modifier::BOLD));
        }
        if !candidate {
            return Some(Style::new().add_modifier(Modifier::DIM));
        }
    }
    if let Selected::Object(id) = sel {
        if state.combat.is_attacking(id) {
            return Some(Style::new().fg(Color::Red).add_modifier(Modifier::BOLD));
        }
        if state.combat.attacker_of(id).is_some() {
            return Some(Style::new().fg(Color::Yellow));
        }
    }
    None
}

/// Draw the whole board for one frame. `view` is the caller's once-per-frame
/// `state.layers()`. `help` overlays the keybinding cheat-sheet.
#[allow(clippy::too_many_arguments)]
pub fn render(
    frame: &mut Frame,
    state: &GameState,
    view: &LayeredView,
    board: &BoardState,
    stop: &Stop,
    interaction: Option<&crate::interact::Interaction>,
    error: Option<&str>,
    pass: &PassState,
    help: bool,
) {
    let [header, main, footer] = Layout::vertical([
        Constraint::Length(3),
        Constraint::Fill(1),
        Constraint::Length(1),
    ])
    .areas(frame.area());
    let [left, detail_area] =
        Layout::horizontal([Constraint::Fill(1), Constraint::Min(30)]).areas(main);
    let [fields, mid_row, hand_area] = Layout::vertical([
        Constraint::Fill(1),
        Constraint::Length(7),
        Constraint::Length(5),
    ])
    .areas(left);
    let [p0_area, p1_area] =
        Layout::horizontal([Constraint::Fill(1), Constraint::Fill(1)]).areas(fields);
    let [stack_area, gy_area, exile_area] = Layout::horizontal([
        Constraint::Fill(1),
        Constraint::Fill(1),
        Constraint::Fill(1),
    ])
    .areas(mid_row);

    let you = board.perspective;

    render_header(frame, header, state, board, pass);
    render_zone(
        frame,
        p0_area,
        state,
        view,
        board,
        interaction,
        Zone::Battlefield(PlayerId(0)),
        "P0 Battlefield",
    );
    render_zone(
        frame,
        p1_area,
        state,
        view,
        board,
        interaction,
        Zone::Battlefield(PlayerId(1)),
        "P1 Battlefield",
    );
    render_zone(
        frame,
        stack_area,
        state,
        view,
        board,
        interaction,
        Zone::Stack,
        "Stack",
    );
    render_zone(
        frame,
        gy_area,
        state,
        view,
        board,
        interaction,
        Zone::Graveyard,
        "Graveyard",
    );
    render_zone(
        frame,
        exile_area,
        state,
        view,
        board,
        interaction,
        Zone::Exile,
        "Exile",
    );
    let hand_title = format!("Hand · P{}", you.0);
    render_zone(
        frame,
        hand_area,
        state,
        view,
        board,
        interaction,
        Zone::Hand,
        &hand_title,
    );
    render_detail(frame, detail_area, state, view, board);
    render_footer(frame, footer, stop, interaction, error);
    if let Some(crate::interact::Interaction::Priority { sub: Some(pick) }) = interaction {
        render_ability_popup(frame, frame.area(), state, pick);
    }
    if help {
        render_help_overlay(frame, frame.area());
    }
}

fn render_header(
    frame: &mut Frame,
    area: Rect,
    state: &GameState,
    board: &BoardState,
    pass: &PassState,
) {
    let turn = &state.turn;
    let you = board.perspective;
    let you_color = player_color(you);

    // The block title makes "which seat you're driving" unmissable, in that
    // seat's color — it swaps sides when the engine hands off to the opponent.
    let title = Line::from(vec![
        Span::raw(" deckmaste.rs — you control "),
        Span::styled(
            format!("Player {}", you.0),
            Style::new().fg(you_color).add_modifier(Modifier::BOLD),
        ),
        Span::raw(" "),
    ]);

    let mut spans = vec![
        Span::raw(format!(
            "Turn {} · {} · active ",
            turn.turn_number,
            phase_name(turn.current)
        )),
        Span::styled(
            format!("P{}", turn.active_player.0),
            Style::new()
                .fg(player_color(turn.active_player))
                .add_modifier(Modifier::BOLD),
        ),
    ];
    for p in &state.players {
        spans.push(Span::raw("    "));
        let mut label = format!("P{}", p.id.0);
        if p.id == you {
            label.push_str(" (you)");
        }
        let _ = write!(label, ": {} life", p.life);
        let life_style = if p.life <= 5 {
            Style::new().fg(Color::Red).add_modifier(Modifier::BOLD)
        } else {
            Style::new().fg(player_color(p.id))
        };
        spans.push(Span::styled(label, life_style));
        if !p.mana_pool.is_empty() {
            spans.push(Span::styled(
                format!(" {}", format::mana_pool(&p.mana_pool)),
                Style::new().fg(Color::Yellow).add_modifier(Modifier::BOLD),
            ));
        }
        match pass.mode(p.id) {
            Some(PassMode::Yield) => {
                spans.push(Span::styled(
                    " · yielding",
                    Style::new().fg(Color::DarkGray),
                ));
            }
            Some(PassMode::Turn) => {
                spans.push(Span::styled(
                    " · passing turn",
                    Style::new().fg(Color::DarkGray),
                ));
            }
            None => {}
        }
    }
    frame.render_widget(
        Paragraph::new(Line::from(spans)).block(
            Block::bordered()
                .title(title)
                .border_style(Style::new().fg(you_color)),
        ),
        area,
    );
}

#[allow(clippy::too_many_arguments)]
fn render_zone(
    frame: &mut Frame,
    area: Rect,
    state: &GameState,
    view: &LayeredView,
    board: &BoardState,
    interaction: Option<&crate::interact::Interaction>,
    zone: Zone,
    title: &str,
) {
    let items = zones::contents(state, view, board.perspective, zone);
    let pick_mode = interaction.is_some_and(crate::interact::Interaction::is_pick_mode);
    // During blocker pairing the pick candidates are the live attackers, which
    // `Interaction::candidates` deliberately leaves out (combat-derived).
    let pairing = matches!(
        interaction,
        Some(crate::interact::Interaction::Blockers {
            pending: Some(_),
            ..
        })
    );
    let rows: Vec<ListItem> = items
        .iter()
        .map(|sel| {
            let body = match *sel {
                Selected::Object(id) => format::object_row(state, view, id),
                Selected::StackEntry(i) => format::stack_label(state, &state.stack[i]),
            };
            let (chosen, mut candidate) = match (*sel, interaction) {
                (Selected::Object(id), Some(it)) => (it.is_chosen(id), it.is_candidate(id)),
                _ => (false, false),
            };
            if let Selected::Object(id) = *sel
                && pairing
                && state.combat.is_attacking(id)
            {
                candidate = true;
            }
            let text = if pick_mode {
                format!("{} {body}", if chosen { "✓" } else { " " })
            } else {
                body
            };
            let mut item = ListItem::new(text);
            if let Some(style) = row_style(*sel, state, pick_mode, chosen, candidate) {
                item = item.style(style);
            }
            item
        })
        .collect();

    let (accent, is_you) = zone_accent(zone, board.perspective);
    let focused = board.is_focused(zone);

    // Title in the zone's accent color; "◀ YOU" tags the seat you control.
    let mut title_spans = vec![Span::styled(
        title.to_string(),
        Style::new().fg(accent).add_modifier(if is_you {
            Modifier::BOLD
        } else {
            Modifier::empty()
        }),
    )];
    if is_you {
        title_spans.push(Span::styled(
            " ◀ YOU",
            Style::new().fg(accent).add_modifier(Modifier::BOLD),
        ));
    }

    let border_style = if focused {
        Style::new().fg(accent).add_modifier(Modifier::BOLD)
    } else {
        Style::new().fg(accent)
    };
    let block = Block::bordered()
        .border_type(if focused { BorderType::Thick } else { BorderType::Plain })
        .title(Line::from(title_spans))
        .border_style(border_style);
    let list = List::new(rows)
        .block(block)
        .highlight_symbol("> ")
        .highlight_style(Style::new().fg(accent).add_modifier(Modifier::REVERSED));

    let mut list_state = ListState::default();
    if focused && !items.is_empty() {
        list_state.select(Some(board.selection_index(zone).min(items.len() - 1)));
    }
    frame.render_stateful_widget(list, area, &mut list_state);
}

fn render_detail(
    frame: &mut Frame,
    area: Rect,
    state: &GameState,
    view: &LayeredView,
    board: &BoardState,
) {
    let text = detail::render(state, view, board.selected(state, view));
    frame.render_widget(
        Paragraph::new(text)
            .block(Block::bordered().title("Detail"))
            .wrap(Wrap { trim: false }),
        area,
    );
}

fn render_footer(
    frame: &mut Frame,
    area: Rect,
    stop: &Stop,
    interaction: Option<&crate::interact::Interaction>,
    error: Option<&str>,
) {
    use crate::interact::Interaction;
    let mut base_style = Style::new();
    let text = match (stop, interaction) {
        (Stop::GameOver(o), _) => {
            base_style = Style::new().fg(Color::Green).add_modifier(Modifier::BOLD);
            format!("GAME OVER: {o:?}    [q] quit")
        }
        (Stop::Budget, _) => "step budget reached    [q] quit".to_string(),
        (Stop::Decision(_), Some(Interaction::Priority { sub: Some(_) })) => {
            "Choose ability — [↑↓] select  [enter] do  [esc] cancel".to_string()
        }
        (Stop::Decision(p), Some(Interaction::Priority { .. })) => format!(
            "P{} priority — [enter] act  [a] pass  [y] yield  [P] pass turn  [?] help  [q] quit",
            p.decider_player().0
        ),
        (Stop::Decision(_), Some(Interaction::Targets { chosen, active, .. })) => {
            let done = chosen.iter().filter(|c| c.is_some()).count();
            format!(
                "Choose target {}/{} — [space] toggle ({done} set)  [enter] confirm/next  [esc] reset",
                active + 1,
                chosen.len()
            )
        }
        (Stop::Decision(_), Some(Interaction::Attackers { chosen, .. })) => format!(
            "Declare attackers — [space] toggle attacker  ·  [enter] submit ({} attacking)  ·  [esc] clear",
            chosen.len()
        ),
        (
            Stop::Decision(_),
            Some(Interaction::Blockers {
                pending: Some(_), ..
            }),
        ) => "Pick the attacker to block — [enter] assign  ·  [backspace] cancel  ·  [esc] clear"
            .to_string(),
        (Stop::Decision(_), Some(Interaction::Blockers { pairs, .. })) => format!(
            "Declare blockers — [space] pick a blocker, then its attacker  ·  [enter] submit ({} blocking)  ·  [esc] clear",
            pairs.len()
        ),
        (Stop::Decision(_), Some(Interaction::Discard { chosen, count, .. })) => format!(
            "Discard {count} — [space] toggle a card in hand  ·  [enter] submit ({}/{count} chosen)  ·  [esc] clear",
            chosen.len()
        ),
        (Stop::Decision(_), None) => "deciding…".to_string(),
    };
    let mut spans = vec![Span::styled(text, base_style)];
    if let Some(e) = error {
        spans.push(Span::styled(
            format!("   ⚠ {e}"),
            Style::new().fg(Color::Red).add_modifier(Modifier::BOLD),
        ));
    }
    frame.render_widget(Paragraph::new(Line::from(spans)), area);
}

fn render_ability_popup(
    frame: &mut Frame,
    full: Rect,
    state: &GameState,
    pick: &crate::interact::AbilityPick,
) {
    use ratatui::widgets::Clear;
    let rows: Vec<ListItem> = pick
        .actions
        .iter()
        .map(|a| ListItem::new(format::action_label(state, a)))
        .collect();
    let area = centered_rect(
        full,
        50,
        u16::try_from(rows.len())
            .unwrap_or(u16::MAX)
            .saturating_add(2),
    );
    let list = List::new(rows)
        .block(Block::bordered().title("Choose ability"))
        .highlight_symbol("> ")
        .highlight_style(Style::new().add_modifier(Modifier::REVERSED));
    let mut st = ListState::default();
    st.select(Some(pick.sel.min(pick.actions.len().saturating_sub(1))));
    frame.render_widget(Clear, area);
    frame.render_stateful_widget(list, area, &mut st);
}

/// The keybinding cheat-sheet, toggled with `?`.
fn render_help_overlay(frame: &mut Frame, full: Rect) {
    use ratatui::widgets::Clear;

    let header = |s: &str| {
        Line::from(Span::styled(
            s.to_string(),
            Style::new().fg(Color::Yellow).add_modifier(Modifier::BOLD),
        ))
    };
    let key = |k: &str, desc: &str| {
        Line::from(vec![
            Span::styled(
                format!("  {k:<12}"),
                Style::new().fg(Color::Cyan).add_modifier(Modifier::BOLD),
            ),
            Span::raw(desc.to_string()),
        ])
    };
    let lines = vec![
        header("Navigate"),
        key("Tab / S-Tab", "cycle zones"),
        key("↑ ↓ ← →", "move selection"),
        key("b / o", "your / opponent battlefield"),
        key("h / g", "hand / graveyard"),
        key("s / e", "stack / exile"),
        Line::raw(""),
        header("Priority"),
        key("Enter", "play or activate the selected card"),
        key("a", "pass priority"),
        key("y", "yield (auto-pass until something happens)"),
        key("P", "pass until your next turn"),
        Line::raw(""),
        header("Choosing targets / attackers / discards"),
        key("Space", "toggle the highlighted object"),
        key("Enter", "submit (targets: confirm, then next)"),
        key("Esc", "reset the choice"),
        Line::raw(""),
        header("Declaring blockers (two steps)"),
        key("Space", "choose one of your blockers"),
        key("Enter", "then pick the attacker it blocks"),
        key("Enter", "with none chosen → submit your blocks"),
        key("Backspace", "undo the last pairing"),
        key("Esc", "clear all blocks"),
        Line::raw(""),
        header("General"),
        key("?", "toggle this help"),
        key("q", "quit"),
    ];
    let height = u16::try_from(lines.len())
        .unwrap_or(u16::MAX)
        .saturating_add(2);
    let area = centered_rect(full, 60, height);
    let para = Paragraph::new(lines).block(
        Block::bordered()
            .border_type(BorderType::Thick)
            .border_style(Style::new().fg(Color::Yellow))
            .title("Help — keybindings"),
    );
    frame.render_widget(Clear, area);
    frame.render_widget(para, area);
}

/// A centered rect `width` columns wide and `height` rows tall, clamped to
/// `full`.
fn centered_rect(full: Rect, width: u16, height: u16) -> Rect {
    let w = width.min(full.width);
    let h = height.min(full.height);
    Rect {
        x: full.x + (full.width - w) / 2,
        y: full.y + (full.height - h) / 2,
        width: w,
        height: h,
    }
}

#[cfg(test)]
mod tests {
    use deckmaste_engine::sim::GreedyCreatures;
    use ratatui::Terminal;
    use ratatui::backend::TestBackend;
    use ratatui::buffer::Buffer;

    use super::*;
    use crate::driver::Driver;
    use crate::game;

    fn buffer_text(buf: &Buffer) -> String {
        let area = buf.area;
        let mut s = String::new();
        for y in 0..area.height {
            for x in 0..area.width {
                s.push_str(buf[(x, y)].symbol());
            }
        }
        s
    }

    #[test]
    fn renders_opening_board_without_panicking() {
        let mut driver = Driver::new(
            game::build_game().expect("build"),
            Box::new(GreedyCreatures),
        );
        let stop = driver.run_to_priority().expect("priority");
        let mut board = BoardState::new();
        board.sync(&driver.state);
        let view = driver.state.layers();

        let pass = PassState::new();
        let backend = TestBackend::new(120, 40);
        let mut terminal = Terminal::new(backend).expect("terminal");
        terminal
            .draw(|frame| {
                render(
                    frame,
                    &driver.state,
                    &view,
                    &board,
                    &stop,
                    None,
                    None,
                    &pass,
                    false,
                );
            })
            .expect("draw");

        let text = buffer_text(terminal.backend().buffer());
        assert!(text.contains("Turn"), "header present:\n{text}");
        assert!(text.contains("life"), "life shown");
        assert!(text.contains("P0 Battlefield"), "battlefield titled");
        assert!(text.contains("Detail"), "detail pane present");
        assert!(text.contains("deciding"), "footer rendered");
    }

    #[test]
    fn renders_attacker_pick_mode_without_panicking() {
        use crate::interact::Interaction;
        let mut driver = Driver::new(
            game::build_game().expect("build"),
            Box::new(GreedyCreatures),
        );
        let stop = driver.run_to_priority().expect("priority");
        let mut board = BoardState::new();
        board.sync(&driver.state);
        let view = driver.state.layers();
        // A synthetic attacker pick over a real battlefield id (or empty legal).
        let legal: Vec<_> = driver.state.zones.battlefield.clone();
        let it = Interaction::Attackers {
            legal,
            chosen: vec![],
        };

        let pass = PassState::new();
        let backend = TestBackend::new(120, 40);
        let mut terminal = Terminal::new(backend).expect("terminal");
        terminal
            .draw(|frame| {
                render(
                    frame,
                    &driver.state,
                    &view,
                    &board,
                    &stop,
                    Some(&it),
                    Some("nope"),
                    &pass,
                    false,
                );
            })
            .expect("draw");
        let text = buffer_text(terminal.backend().buffer());
        assert!(text.contains("Declare attackers"), "footer prompt:\n{text}");
        assert!(text.contains("nope"), "error surfaced");
    }

    #[test]
    fn renders_discard_pick_mode_with_footer_prompt() {
        use crate::interact::Interaction;
        let mut driver = Driver::new(
            game::build_game().expect("build"),
            Box::new(GreedyCreatures),
        );
        let stop = driver.run_to_priority().expect("priority");
        let mut board = BoardState::new();
        board.sync(&driver.state);
        let view = driver.state.layers();
        // A discard picker over the perspective player's hand.
        let hand: Vec<_> = driver.state.zones.hands[board.perspective.index()].clone();
        let it = Interaction::Discard {
            legal: hand,
            chosen: vec![],
            count: 1,
        };

        let pass = PassState::new();
        let backend = TestBackend::new(120, 40);
        let mut terminal = Terminal::new(backend).expect("terminal");
        terminal
            .draw(|frame| {
                render(
                    frame,
                    &driver.state,
                    &view,
                    &board,
                    &stop,
                    Some(&it),
                    None,
                    &pass,
                    false,
                );
            })
            .expect("draw");
        let text = buffer_text(terminal.backend().buffer());
        assert!(text.contains("Discard 1"), "footer prompt:\n{text}");
        assert!(text.contains("0/1 chosen"), "progress shown:\n{text}");
    }

    #[test]
    fn priority_footer_shows_yield_and_pass_turn() {
        let mut driver = Driver::new(
            game::build_game().expect("build"),
            Box::new(GreedyCreatures),
        );
        let stop = driver.run_to_priority().expect("priority");
        let mut board = BoardState::new();
        board.sync(&driver.state);
        let view = driver.state.layers();
        let pass = PassState::new();
        let interaction = crate::interact::Interaction::Priority { sub: None };

        let backend = TestBackend::new(120, 40);
        let mut terminal = Terminal::new(backend).expect("terminal");
        terminal
            .draw(|frame| {
                render(
                    frame,
                    &driver.state,
                    &view,
                    &board,
                    &stop,
                    Some(&interaction),
                    None,
                    &pass,
                    false,
                );
            })
            .expect("draw");
        let text = buffer_text(terminal.backend().buffer());
        assert!(text.contains("yield"), "footer offers yield:\n{text}");
        assert!(
            text.contains("pass turn"),
            "footer offers pass turn:\n{text}"
        );
    }

    #[test]
    fn header_shows_armed_mode() {
        let mut driver = Driver::new(
            game::build_game().expect("build"),
            Box::new(GreedyCreatures),
        );
        let stop = driver.run_to_priority().expect("priority");
        let mut board = BoardState::new();
        board.sync(&driver.state);
        let view = driver.state.layers();
        let mut pass = PassState::new();
        pass.arm(PlayerId(0), PassMode::Yield, &driver.state);

        let backend = TestBackend::new(120, 40);
        let mut terminal = Terminal::new(backend).expect("terminal");
        terminal
            .draw(|frame| {
                render(
                    frame,
                    &driver.state,
                    &view,
                    &board,
                    &stop,
                    None,
                    None,
                    &pass,
                    false,
                );
            })
            .expect("draw");
        let text = buffer_text(terminal.backend().buffer());
        assert!(
            text.contains("yielding"),
            "header shows armed mode:\n{text}"
        );
    }

    #[test]
    fn board_shows_controlled_seat_and_all_zone_panes() {
        let mut driver = Driver::new(
            game::build_game().expect("build"),
            Box::new(GreedyCreatures),
        );
        let stop = driver.run_to_priority().expect("priority");
        let mut board = BoardState::new();
        board.sync(&driver.state);
        let view = driver.state.layers();
        let pass = PassState::new();

        let backend = TestBackend::new(120, 40);
        let mut terminal = Terminal::new(backend).expect("terminal");
        terminal
            .draw(|frame| {
                render(
                    frame,
                    &driver.state,
                    &view,
                    &board,
                    &stop,
                    None,
                    None,
                    &pass,
                    false,
                );
            })
            .expect("draw");
        let text = buffer_text(terminal.backend().buffer());
        assert!(
            text.contains("you control"),
            "controlled seat shown:\n{text}"
        );
        assert!(text.contains("YOU"), "perspective marker shown:\n{text}");
        assert!(
            text.contains("Graveyard"),
            "graveyard pane present:\n{text}"
        );
        assert!(text.contains("Exile"), "exile pane present:\n{text}");
    }

    #[test]
    fn help_overlay_lists_keybindings() {
        let mut driver = Driver::new(
            game::build_game().expect("build"),
            Box::new(GreedyCreatures),
        );
        let stop = driver.run_to_priority().expect("priority");
        let mut board = BoardState::new();
        board.sync(&driver.state);
        let view = driver.state.layers();
        let pass = PassState::new();

        let backend = TestBackend::new(120, 40);
        let mut terminal = Terminal::new(backend).expect("terminal");
        terminal
            .draw(|frame| {
                render(
                    frame,
                    &driver.state,
                    &view,
                    &board,
                    &stop,
                    None,
                    None,
                    &pass,
                    true,
                );
            })
            .expect("draw");
        let text = buffer_text(terminal.backend().buffer());
        assert!(text.contains("Help"), "help overlay titled:\n{text}");
        assert!(
            text.contains("pass priority"),
            "help lists pass key:\n{text}"
        );
        assert!(
            text.contains("opponent battlefield"),
            "help lists zone keys:\n{text}"
        );
    }
}
