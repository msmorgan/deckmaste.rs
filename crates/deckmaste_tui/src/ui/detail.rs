//! The detail pane's text. v1 is a readable STUB derived from the encoding and
//! the derived characteristics. SEAM: `card-text-render` replaces the body of
//! [`render`] with real rules text — the signature and all callers stay put.
use std::fmt::Write as _;

use deckmaste_engine::Characteristics;
use deckmaste_engine::GameState;
use deckmaste_engine::LayeredView;
use deckmaste_engine::ObjectId;
use deckmaste_engine::StackObject;
use deckmaste_engine::face;
use ratatui::text::Text;

use crate::ui::board::Selected;
use crate::ui::format;

/// Render the detail pane for the current selection.
#[must_use]
pub fn render(state: &GameState, view: &LayeredView, sel: Option<Selected>) -> Text<'static> {
    match sel {
        None => Text::from("(no selection)"),
        Some(Selected::Object(id)) => object_detail(state, view, id),
        Some(Selected::StackEntry(i)) => stack_detail(state, view, i),
    }
}

fn object_detail(state: &GameState, view: &LayeredView, id: ObjectId) -> Text<'static> {
    let mut s = String::new();
    let printed = face(state.def(id));
    let _ = writeln!(s, "{}", printed.name);
    let cost = format::mana_cost(&printed.mana_cost);
    if !cost.is_empty() {
        let _ = writeln!(s, "{cost}");
    }
    let chars = view.get(id);
    let _ = writeln!(s, "{}", type_line(chars));
    if let (Some(p), Some(t)) = (chars.power, chars.toughness) {
        let _ = writeln!(s, "{p}/{t}");
    }
    // STUB: derived abilities, Debug-formatted. card-text-render renders these
    // as real rules text.
    for ability in state.abilities(id).iter() {
        let _ = writeln!(s, "{ability:?}");
    }
    Text::from(s)
}

fn stack_detail(state: &GameState, view: &LayeredView, i: usize) -> Text<'static> {
    let Some(entry) = state.stack.get(i) else {
        return Text::from("(empty)");
    };
    match &entry.object {
        StackObject::Spell(id) => object_detail(state, view, *id),
        StackObject::Triggered { .. } => Text::from(format!(
            "triggered ability\ncontroller: P{}",
            entry.controller.0
        )),
        StackObject::Activated { .. } => Text::from(format!(
            "activated ability\ncontroller: P{}",
            entry.controller.0
        )),
    }
}

fn type_line(chars: &Characteristics) -> String {
    let mut parts: Vec<String> = Vec::new();
    for st in chars.supertypes.iter() {
        parts.push(format!("{st:?}"));
    }
    for t in chars.card_types.iter() {
        parts.push(format!("{t:?}"));
    }
    let mut line = parts.join(" ");
    if !chars.subtypes.is_empty() {
        let subs: Vec<String> = chars.subtypes.iter().map(|s| s.name.to_string()).collect();
        let _ = write!(line, " — {}", subs.join(" "));
    }
    line
}

#[cfg(test)]
mod tests {
    use deckmaste_engine::face;
    use deckmaste_engine::sim::GreedyCreatures;

    use super::*;
    use crate::driver::Driver;
    use crate::game;

    fn text_to_string(t: &Text) -> String {
        t.lines
            .iter()
            .map(|l| {
                l.spans
                    .iter()
                    .map(|s| s.content.as_ref())
                    .collect::<String>()
            })
            .collect::<Vec<_>>()
            .join("\n")
    }

    fn opening() -> GameState {
        let mut d = Driver::new(
            game::build_game().expect("build"),
            Box::new(GreedyCreatures),
        );
        d.run_to_priority().expect("priority");
        d.state
    }

    #[test]
    fn none_is_placeholder() {
        let state = opening();
        let view = state.layers();
        assert_eq!(
            text_to_string(&render(&state, &view, None)),
            "(no selection)"
        );
    }

    #[test]
    fn object_detail_names_the_card_and_type_line() {
        let state = opening();
        let view = state.layers();
        let id = *state.zones.hands[0].first().expect("nonempty hand");
        let s = text_to_string(&render(&state, &view, Some(Selected::Object(id))));
        assert!(
            s.contains(&face(state.def(id)).name),
            "detail names the card: {s}"
        );
    }
}
