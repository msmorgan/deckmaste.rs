//! Presentational helpers: an object's one-line board row, a stack entry's
//! label, a mana-cost string. Pure `String`s — no ratatui types.
use std::fmt::Write as _;

use deckmaste_core::ManaCost;
use deckmaste_core::ManaSymbol;
use deckmaste_core::SimpleManaSymbol;
use deckmaste_engine::Action;
use deckmaste_engine::ActionViewKind;
use deckmaste_engine::GameState;
use deckmaste_engine::LayeredView;
use deckmaste_engine::ObjectId;
use deckmaste_engine::StackEntry;
use deckmaste_engine::StackObject;
use deckmaste_engine::face;

/// One board row for a battlefield/hand object: name, derived P/T (creatures),
/// then status markers (tapped / summoning-sick / attacking / blocked / marked
/// damage) and counters, sorted for stable display.
#[must_use]
pub fn object_row(state: &GameState, view: &LayeredView, id: ObjectId) -> String {
    let mut row = face(state.def(id)).name.clone();
    let chars = view.get(id);
    if let (Some(p), Some(t)) = (chars.power, chars.toughness) {
        let _ = write!(row, " {p}/{t}");
    }
    let obj = state.objects.obj(id);
    let mut marks: Vec<String> = Vec::new();
    if obj.tapped {
        marks.push("T".to_string());
    }
    if obj.summoning_sick {
        marks.push("sick".to_string());
    }
    if state.combat.is_attacking(id) {
        marks.push("atk".to_string());
    }
    if state.combat.is_blocked(id) {
        marks.push("blk".to_string());
    }
    if obj.damage > 0 {
        marks.push(format!("dmg{}", obj.damage));
    }
    let mut counters: Vec<(&str, &u32)> =
        obj.counters.iter().map(|(k, n)| (k.as_str(), n)).collect();
    counters.sort_by_key(|(k, _)| *k);
    for (kind, n) in counters {
        marks.push(format!("{kind}×{n}"));
    }
    if !marks.is_empty() {
        let _ = write!(row, " [{}]", marks.join(" "));
    }
    row
}

/// A short label for a stack entry.
#[must_use]
pub fn stack_label(state: &GameState, entry: &StackEntry) -> String {
    match &entry.object {
        StackObject::Spell(id) => face(state.def(*id)).name.clone(),
        StackObject::Triggered { .. } => "(triggered ability)".to_string(),
        StackObject::Activated { .. } => "(activated ability)".to_string(),
    }
}

/// A readable mana-cost string, e.g. `{1}{R}`. Best-effort for the demo set;
/// exotic symbols fall back to a `Debug` form (the detail pane is a stub until
/// `card-text-render`).
#[must_use]
pub fn mana_cost(cost: &ManaCost) -> String { cost.iter().map(symbol).collect() }

/// A short, human label for a legal priority action, built from the engine's
/// `describe_action` view. Used by the footer and the ability-pick popup.
#[must_use]
pub fn action_label(state: &GameState, action: &Action) -> String {
    let view = state.describe_action(action);
    let name = view.name.unwrap_or("?");
    match view.kind {
        ActionViewKind::Pass => "Pass".to_string(),
        ActionViewKind::Concede => "Concede".to_string(),
        ActionViewKind::PlayLand => format!("Play land: {name}"),
        ActionViewKind::Cast { cost } => match cost {
            Some(c) => format!("Cast {name} {}", mana_cost(&c)),
            None => format!("Cast {name}"),
        },
        ActionViewKind::Activate { mana, .. } => {
            if mana {
                format!("{name}: tap for mana")
            } else {
                format!("{name}: activate ability")
            }
        }
    }
}

fn symbol(s: &ManaSymbol) -> String {
    match s {
        ManaSymbol::Variable => "{X}".to_string(),
        ManaSymbol::Snow => "{S}".to_string(),
        ManaSymbol::Simple(SimpleManaSymbol::Generic(n)) => format!("{{{n}}}"),
        ManaSymbol::Simple(SimpleManaSymbol::Specific(c)) => format!("{{{c:?}}}"),
        other => format!("{{{other:?}}}"),
    }
}

#[cfg(test)]
mod tests {
    use deckmaste_engine::PlayerId;
    use deckmaste_engine::StackEntry;
    use deckmaste_engine::StackObject;
    use deckmaste_engine::face;
    use deckmaste_engine::sim::GreedyCreatures;

    use super::*;
    use crate::driver::Driver;
    use crate::game;

    fn opening() -> GameState {
        let mut d = Driver::new(
            game::build_game().expect("build"),
            Box::new(GreedyCreatures),
        );
        d.run_to_priority().expect("priority");
        d.state
    }

    #[test]
    fn object_row_for_a_vanilla_library_creature_is_name_and_pt() {
        let state = opening();
        let view = state.layers();
        // A creature in P0's library is untapped, not in combat, no counters.
        let id = state.zones.libraries[0]
            .iter()
            .copied()
            .find(|&id| view.get(id).power.is_some() && view.get(id).toughness.is_some())
            .expect("a creature in P0's library");
        let chars = view.get(id);
        let expected = format!(
            "{} {}/{}",
            face(state.def(id)).name,
            chars.power.unwrap(),
            chars.toughness.unwrap()
        );
        assert_eq!(object_row(&state, &view, id), expected);
    }

    #[test]
    fn stack_label_names_a_spell() {
        let state = opening();
        let id = *state.zones.libraries[0].front().expect("nonempty library");
        let entry = StackEntry {
            id,
            object: StackObject::Spell(id),
            controller: PlayerId(0),
            targets: vec![],
            x: None,
        };
        assert_eq!(stack_label(&state, &entry), face(state.def(id)).name);
    }
}
