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
use deckmaste_engine::GameState;
use deckmaste_engine::LayeredView;
use deckmaste_engine::PlayerId;
use ratatui::Frame;
use ratatui::layout::Constraint;
use ratatui::layout::Layout;
use ratatui::layout::Rect;
use ratatui::style::Modifier;
use ratatui::style::Style;
use ratatui::widgets::Block;
use ratatui::widgets::List;
use ratatui::widgets::ListItem;
use ratatui::widgets::ListState;
use ratatui::widgets::Paragraph;
use ratatui::widgets::Wrap;

use crate::driver::Stop;
use crate::ui::board::Zone;

/// Draw the whole board for one frame. `view` is the caller's once-per-frame
/// `state.layers()`.
pub fn render(
    frame: &mut Frame,
    state: &GameState,
    view: &LayeredView,
    board: &BoardState,
    stop: &Stop,
    interaction: Option<&crate::interact::Interaction>,
    error: Option<&str>,
) {
    let [header, main, footer] = Layout::vertical([
        Constraint::Length(3),
        Constraint::Fill(1),
        Constraint::Length(1),
    ])
    .areas(frame.area());
    let [left, detail_area] =
        Layout::horizontal([Constraint::Fill(1), Constraint::Min(30)]).areas(main);
    let [fields, stack_area, hand_area] = Layout::vertical([
        Constraint::Fill(1),
        Constraint::Length(6),
        Constraint::Length(5),
    ])
    .areas(left);
    let [p0_area, p1_area] =
        Layout::horizontal([Constraint::Fill(1), Constraint::Fill(1)]).areas(fields);

    render_header(frame, header, state, board);
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
    let hand_title = format!("Hand · P{}", board.perspective.0);
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
}

fn render_header(frame: &mut Frame, area: Rect, state: &GameState, board: &BoardState) {
    let turn = &state.turn;
    let mut text = format!(
        "Turn {} · {:?} · active P{}",
        turn.turn_number, turn.current, turn.active_player.0
    );
    if board.perspective != turn.active_player {
        let _ = write!(text, " · viewing P{}", board.perspective.0);
    }
    for p in &state.players {
        let _ = write!(text, "    P{}: {} life", p.id.0, p.life);
    }
    frame.render_widget(Paragraph::new(text).block(Block::bordered()), area);
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
    let rows: Vec<ListItem> = items
        .iter()
        .map(|sel| {
            let body = match *sel {
                Selected::Object(id) => format::object_row(state, view, id),
                Selected::StackEntry(i) => format::stack_label(state, &state.stack[i]),
            };
            let (chosen, candidate) = match (*sel, interaction) {
                (Selected::Object(id), Some(it)) => (it.is_chosen(id), it.is_candidate(id)),
                _ => (false, false),
            };
            let text = if pick_mode {
                format!("{} {body}", if chosen { "✓" } else { " " })
            } else {
                body
            };
            let mut item = ListItem::new(text);
            if pick_mode && !chosen && !candidate {
                item = item.style(Style::new().add_modifier(Modifier::DIM));
            }
            item
        })
        .collect();

    let focused = board.is_focused(zone);
    let border_style =
        if focused { Style::new().add_modifier(Modifier::BOLD) } else { Style::new() };
    let block = Block::bordered().title(title).border_style(border_style);
    let list = List::new(rows)
        .block(block)
        .highlight_symbol("> ")
        .highlight_style(Style::new().add_modifier(Modifier::REVERSED));

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
    let text = match (stop, interaction) {
        (Stop::GameOver(o), _) => format!("GAME OVER: {o:?}    [q] quit"),
        (Stop::Budget, _) => "step budget reached    [q] quit".to_string(),
        (Stop::Decision(_), Some(Interaction::Priority { sub: Some(_) })) => {
            "Choose ability — [↑↓] select  [enter] do  [esc] cancel".to_string()
        }
        (Stop::Decision(p), Some(Interaction::Priority { .. })) => format!(
            "P{} priority — [tab/↑↓←→] select  [enter] act  [space] pass  [q] quit",
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
            "Declare attackers — [space] toggle ({} selected)  [enter] confirm  [esc] clear",
            chosen.len()
        ),
        (
            Stop::Decision(_),
            Some(Interaction::Blockers {
                pending: Some(_), ..
            }),
        ) => "Block which attacker? — [enter] assign  [backspace] cancel  [esc] clear".to_string(),
        (Stop::Decision(_), Some(Interaction::Blockers { pairs, .. })) => format!(
            "Declare blockers — [space] pick blocker ({} paired)  [enter] confirm  [esc] clear",
            pairs.len()
        ),
        (Stop::Decision(_), None) => "deciding…".to_string(),
    };
    let text = match error {
        Some(e) => format!("{text}   ⚠ {e}"),
        None => text,
    };
    frame.render_widget(Paragraph::new(text), area);
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

        let backend = TestBackend::new(120, 40);
        let mut terminal = Terminal::new(backend).expect("terminal");
        terminal
            .draw(|frame| render(frame, &driver.state, &view, &board, &stop, None, None))
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
                );
            })
            .expect("draw");
        let text = buffer_text(terminal.backend().buffer());
        assert!(text.contains("Declare attackers"), "footer prompt:\n{text}");
        assert!(text.contains("nope"), "error surfaced");
    }
}
