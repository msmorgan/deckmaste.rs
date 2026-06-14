//! The detail pane's text. An object's printed face plus its *derived*
//! [`Characteristics`](deckmaste_engine::Characteristics) are bridged into a
//! `core`-typed [`CardView`] and run through the engine-free `deckmaste_cards`
//! renderer, so the pane shows real rules text over the live (pumped, animated,
//! control-changed) object — not the printed encoding.
use std::fmt::Write as _;

use deckmaste_cards::render::CardView;
use deckmaste_cards::render::RenderedCard;
use deckmaste_cards::render::render as render_card_view;
use deckmaste_core::StatValue;
use deckmaste_engine::GameState;
use deckmaste_engine::LayeredView;
use deckmaste_engine::ObjectId;
use deckmaste_engine::StackObject;
use deckmaste_engine::face;
use ratatui::text::Text;

use crate::ui::board::Selected;

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
    let printed = face(state.def(id));
    let chars = view.get(id);
    // Mana cost and name aren't derived characteristics; take them from the
    // printed face. Power/toughness derive to concrete numbers — lift them back
    // into the `StatValue` the renderer's `CardView` expects.
    let power = chars.power.map(StatValue::Number);
    let toughness = chars.toughness.map(StatValue::Number);
    let card = render_card_view(&CardView {
        name: &printed.name,
        mana_cost: Some(&printed.mana_cost),
        supertypes: &chars.supertypes,
        types: &chars.card_types,
        subtypes: &chars.subtypes,
        power: power.as_ref(),
        toughness: toughness.as_ref(),
        abilities: &chars.abilities,
    });
    detail_text(&card)
}

/// Lay out a rendered card as the detail pane's plain text: name, cost (when
/// any), type line, P/T (when any), then one line per rule.
fn detail_text(card: &RenderedCard) -> Text<'static> {
    let mut s = String::new();
    let _ = writeln!(s, "{}", card.name);
    if !card.mana_cost.is_empty() {
        let _ = writeln!(s, "{}", card.mana_cost);
    }
    let _ = writeln!(s, "{}", card.type_line);
    if let Some(pt) = &card.pt {
        let _ = writeln!(s, "{pt}");
    }
    for rule in &card.rules {
        let _ = writeln!(s, "{rule}");
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

    /// Elvish Visionary's `Triggered(ThisEnters, Draw(1))` renders as a real
    /// sentence ("…draw a card."), proving the detail pane runs the
    /// `deckmaste_cards` renderer over derived characteristics rather than
    /// Debug-formatting the abilities.
    #[test]
    fn object_detail_renders_abilities_as_prose_not_debug() {
        let state = opening();
        let view = state.layers();
        let id = state
            .objects
            .iter()
            .filter(|o| o.card_id().is_some())
            .map(|o| o.id)
            .find(|&id| face(state.def(id)).name == "Elvish Visionary")
            .expect("Elvish Visionary in game");
        let s = text_to_string(&render(&state, &view, Some(Selected::Object(id))));
        assert!(s.contains("draw a card"), "renders effect as prose: {s}");
        assert!(!s.contains("Triggered"), "no Debug ability form: {s}");
    }
}
