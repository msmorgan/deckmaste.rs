//! Minimal plain-text rendering of the game snapshot. Deliberately sparse —
//! `tui-board-view` + `card-text-render` replace it with the real board.
mod board;
use std::fmt::Write as _;

use deckmaste_core::Card;
use deckmaste_engine::GameState;
use deckmaste_engine::ObjectId;
use ratatui::Frame;
use ratatui::layout::Constraint;
use ratatui::layout::Layout;
use ratatui::widgets::Block;
use ratatui::widgets::Paragraph;

use crate::driver::Stop;

pub fn render(frame: &mut Frame, state: &GameState, stop: &Stop) {
    let [header, body, footer] = Layout::vertical([
        Constraint::Length(3),
        Constraint::Fill(1),
        Constraint::Length(1),
    ])
    .areas(frame.area());

    let turn = &state.turn;
    let header_text = format!(
        "Turn {} — {:?} — active P{}",
        turn.turn_number, turn.current, turn.active_player.0,
    );
    frame.render_widget(Paragraph::new(header_text).block(Block::bordered()), header);

    let mut text = String::new();
    for player in &state.players {
        let _ = writeln!(text, "P{}: {} life", player.id.0, player.life);
    }
    text.push_str("\nBattlefield:\n");
    for &id in &state.zones.battlefield {
        let _ = writeln!(text, "  {}", object_name(state, id));
    }
    text.push('\n');
    match stop {
        Stop::Priority(p) => {
            let _ = write!(text, "Pending: {p:?}");
        }
        Stop::GameOver(o) => {
            let _ = write!(text, "GAME OVER: {o:?}");
        }
        Stop::Budget => text.push_str("step budget reached"),
    }
    frame.render_widget(Paragraph::new(text).block(Block::bordered()), body);

    frame.render_widget(
        Paragraph::new("[space] pass   [enter] auto   [q] quit"),
        footer,
    );
}

fn object_name(state: &GameState, id: ObjectId) -> String {
    // Presentational only (no test pins this). Name card-backed objects via the
    // front face; defend against a non-card object (e.g. a player proxy) since
    // `state.def` panics on those. tui-board-view owns real rendering.
    if state.objects.obj(id).card_id().is_none() {
        return "<non-card>".to_string();
    }
    match state.def(id) {
        Card::Normal(face) | Card::ModalDfc(face, _) => face.name.clone(),
    }
}
