//! Presentational helpers: an object's one-line board row, a stack entry's
//! label, a mana-cost string. Pure `String`s — no ratatui types.
use std::fmt::Write as _;

use deckmaste_core::ManaCost;
use deckmaste_core::ManaSymbol;
use deckmaste_core::SimpleManaSymbol;
use deckmaste_core::Type;
use deckmaste_engine::Action;
use deckmaste_engine::ActionViewKind;
use deckmaste_engine::GameState;
use deckmaste_engine::LayeredView;
use deckmaste_engine::ObjectId;
use deckmaste_engine::ObjectSource;
use deckmaste_engine::StackEntry;
use deckmaste_engine::StackObject;
use deckmaste_engine::face;

/// One board row for a battlefield/hand object: name, derived P/T (creatures),
/// then status markers (tapped / summoning-sick / attacking / blocked / marked
/// damage) and counters, sorted for stable display.
#[must_use]
pub fn object_row(state: &GameState, view: &LayeredView, id: ObjectId) -> String {
    // A player proxy is an object in no zone (the targetable "face"); it has no
    // backing card, so render its life instead of dereferencing a card def.
    if let ObjectSource::Player(pid) = state.objects.obj(id).source {
        return format!("Player {} — {} life", pid.0, state.player(pid).life);
    }
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
    // Summoning sickness only matters for creatures (attacking / {T} abilities);
    // showing it on lands and other non-creatures is just noise.
    if obj.summoning_sick && chars.card_types.contains(&Type::Creature) {
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

/// A short name for an object: its card name, or `Player N` for a player proxy
/// (which has no backing card).
#[must_use]
pub fn object_name(state: &GameState, id: ObjectId) -> String {
    if let ObjectSource::Player(pid) = state.objects.obj(id).source {
        format!("Player {}", pid.0)
    } else {
        face(state.def(id)).name.clone()
    }
}

/// A short label for a stack entry: the spell/ability, then its targets so you
/// can see what's about to resolve (e.g. `Lightning Bolt → Player 1`).
#[must_use]
pub fn stack_label(state: &GameState, entry: &StackEntry) -> String {
    let base = match &entry.object {
        StackObject::Spell(id) => object_name(state, *id),
        StackObject::Triggered { .. } => "(triggered ability)".to_string(),
        StackObject::Activated { .. } => "(activated ability)".to_string(),
    };
    if entry.targets.is_empty() {
        base
    } else {
        let targets = entry
            .targets
            .iter()
            .map(|&t| object_name(state, t))
            .collect::<Vec<_>>()
            .join(", ");
        format!("{base} → {targets}")
    }
}

/// A player's floating mana as symbols in WUBRG-then-colorless order, e.g.
/// `{R}{R}{G}`. Empty string for an empty pool.
#[must_use]
pub fn mana_pool(pool: &deckmaste_engine::ManaPool) -> String {
    use deckmaste_core::Color;
    use deckmaste_core::ColorOrColorless::Color as Col;
    use deckmaste_core::ColorOrColorless::Colorless;

    let mut s = String::new();
    for (kind, sym) in [
        (Col(Color::White), 'W'),
        (Col(Color::Blue), 'U'),
        (Col(Color::Black), 'B'),
        (Col(Color::Red), 'R'),
        (Col(Color::Green), 'G'),
        (Colorless, 'C'),
    ] {
        for _ in 0..pool.amount(kind) {
            let _ = write!(s, "{{{sym}}}");
        }
    }
    s
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
    fn summoning_sickness_is_hidden_on_non_creatures() {
        use deckmaste_core::Type;
        use deckmaste_engine::Action;
        use deckmaste_engine::Decision;
        use deckmaste_engine::PendingDecision;

        use crate::driver::Driver;
        use crate::driver::Stop;
        use crate::interact::Interaction;

        let mut driver = Driver::new(
            game::build_game().expect("build"),
            Box::new(GreedyCreatures),
        );
        let mut stop = driver.run_to_decision().expect("first stop");
        // Develop the board (play lands / cast) until a summoning-sick land
        // appears; its row must never show "sick".
        for _ in 0..50_000 {
            {
                let view = driver.state.layers();
                for &id in &driver.state.zones.battlefield {
                    if view.get(id).card_types.contains(&Type::Creature) {
                        continue;
                    }
                    let row = object_row(&driver.state, &view, id);
                    assert!(!row.contains("sick"), "non-creature marked sick: {row}");
                    if driver.state.objects.obj(id).summoning_sick {
                        return; // found a sick land whose row correctly hides it
                    }
                }
            }
            let pending = match &stop {
                Stop::GameOver(_) | Stop::Budget => break,
                Stop::Decision(p) => p.clone(),
            };
            let decision = match &pending {
                PendingDecision::Priority { legal, .. } => {
                    let pick = legal
                        .iter()
                        .find(|a| matches!(a, Action::PlayLand { .. }))
                        .or_else(|| legal.iter().find(|a| matches!(a, Action::CastSpell { .. })))
                        .cloned()
                        .unwrap_or(Action::Pass);
                    Decision::Act(pick)
                }
                PendingDecision::ChooseTargets { .. } => {
                    let mut it = Interaction::for_decision(&pending).expect("interactive");
                    loop {
                        if let Some(&first) = it.candidates().first() {
                            it.toggle(first);
                        }
                        match it.confirm() {
                            Some(d) => break d,
                            None => it.advance(),
                        }
                    }
                }
                PendingDecision::DeclareAttackers { .. } => Decision::Attackers(vec![]),
                PendingDecision::DeclareBlockers { .. } => Decision::Blocks(vec![]),
                other => panic!("unexpected surfaced kind: {other:?}"),
            };
            stop = driver.submit(decision).expect("legal decision");
        }
        panic!("no summoning-sick non-creature ever appeared — test is vacuous");
    }

    #[test]
    fn player_proxy_row_shows_life_not_a_card() {
        let state = opening();
        let view = state.layers();
        // The player proxy has no backing card; its row reads as a face/life,
        // and must not panic dereferencing a card def.
        let proxy = state.player(PlayerId(0)).object;
        let row = object_row(&state, &view, proxy);
        assert!(row.contains("Player 0"), "names the player: {row}");
        assert!(row.contains("20 life"), "shows life: {row}");
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

    #[test]
    fn mana_pool_renders_symbols_in_wubrg_order() {
        use deckmaste_core::Color;
        use deckmaste_core::ColorOrColorless;

        let mut pool = deckmaste_engine::ManaPool::default();
        pool.add(ColorOrColorless::Color(Color::Green), 1);
        pool.add(ColorOrColorless::Color(Color::Red), 2);
        pool.add(ColorOrColorless::Colorless, 1);
        // Output is normalized to WUBRG-then-colorless regardless of add order.
        assert_eq!(mana_pool(&pool), "{R}{R}{G}{C}");
        assert_eq!(mana_pool(&deckmaste_engine::ManaPool::default()), "");
    }

    #[test]
    fn stack_label_shows_targets() {
        let state = opening();
        let id = *state.zones.libraries[0].front().expect("nonempty library");
        let target = state.player(PlayerId(1)).object;
        let entry = StackEntry {
            id,
            object: StackObject::Spell(id),
            controller: PlayerId(0),
            targets: vec![target],
            x: None,
        };
        let label = stack_label(&state, &entry);
        assert!(
            label.contains(&face(state.def(id)).name),
            "names the spell: {label}"
        );
        assert!(label.contains("→ Player 1"), "shows the target: {label}");
    }
}
