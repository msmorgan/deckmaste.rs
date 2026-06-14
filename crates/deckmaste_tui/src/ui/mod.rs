//! Board-view layout and ratatui glue. The only module that touches `Frame`,
//! `Layout`, `List`, `Block`. Pure helpers live in `board`/`zones`/`format`/
//! `detail`.
mod board;
mod detail;
mod format;
mod zones;

use std::fmt::Write as _;

pub use board::BoardState;
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
use crate::ui::board::Selected;
use crate::ui::board::Zone;

/// Draw the whole board for one frame. `view` is the caller's once-per-frame
/// `state.layers()`.
pub fn render(
    frame: &mut Frame,
    state: &GameState,
    view: &LayeredView,
    board: &BoardState,
    stop: &Stop,
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
        Zone::Battlefield(PlayerId(0)),
        "P0 Battlefield",
    );
    render_zone(
        frame,
        p1_area,
        state,
        view,
        board,
        Zone::Battlefield(PlayerId(1)),
        "P1 Battlefield",
    );
    render_zone(frame, stack_area, state, view, board, Zone::Stack, "Stack");
    let hand_title = format!("Hand · P{}", board.perspective.0);
    render_zone(
        frame,
        hand_area,
        state,
        view,
        board,
        Zone::Hand,
        &hand_title,
    );
    render_detail(frame, detail_area, state, view, board);
    render_footer(frame, footer, stop);
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

fn render_zone(
    frame: &mut Frame,
    area: Rect,
    state: &GameState,
    view: &LayeredView,
    board: &BoardState,
    zone: Zone,
    title: &str,
) {
    let items = zones::contents(state, view, board.perspective, zone);
    let rows: Vec<ListItem> = items
        .iter()
        .map(|sel| {
            let s = match *sel {
                Selected::Object(id) => format::object_row(state, view, id),
                Selected::StackEntry(i) => format::stack_label(state, &state.stack[i]),
            };
            ListItem::new(s)
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

fn render_footer(frame: &mut Frame, area: Rect, stop: &Stop) {
    let status = match stop {
        Stop::Decision(p) => format!("P{} to decide", p.decider_player().0),
        Stop::GameOver(o) => format!("GAME OVER: {o:?}"),
        Stop::Budget => "step budget reached".to_string(),
    };
    let hint = "[Tab] zone  [↑↓←→] select  [space] pass  [enter] auto  [q] quit";
    frame.render_widget(Paragraph::new(format!("{status}    {hint}")), area);
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
            .draw(|frame| render(frame, &driver.state, &view, &board, &stop))
            .expect("draw");

        let text = buffer_text(terminal.backend().buffer());
        assert!(text.contains("Turn"), "header present:\n{text}");
        assert!(text.contains("life"), "life shown");
        assert!(text.contains("P0 Battlefield"), "battlefield titled");
        assert!(text.contains("Detail"), "detail pane present");
        assert!(text.contains("[Tab]"), "footer hints rendered");
    }
}
