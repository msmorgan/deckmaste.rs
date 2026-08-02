//! Builds the demo `GameState` from committed plugin data + decklist files.
use std::path::Path;
use std::path::PathBuf;

use anyhow::Result;
use deckmaste_engine::GameConfig;
use deckmaste_engine::GameState;
use deckmaste_engine::PlayerConfig;
use deckmaste_engine::PlayerId;
use deckmaste_engine::StartingPlayer;
use deckmaste_plugin::Deck;
use deckmaste_plugin::plugin::Plugin;

/// The fixed shuffle seed used by the test suite — chosen so the shuffle deals
/// both decks a keepable opening hand (guarded by
/// `opening_hands_are_keepable`); an earlier value dealt the red Goblins deck a
/// landless hand on every run. The binary seeds from `--seed`/entropy instead
/// (see [`build_game_with_seed`]), so live play varies run to run.
#[cfg(test)]
const SEED: u64 = 11;

fn data(rel: &str) -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join(rel)
}

/// Build the demo game with the fixed test seed — a deterministic game for the
/// test suite. The binary builds with a `--seed`/entropy value via
/// [`build_game_with_seed`] instead.
///
/// # Errors
/// If a plugin or decklist fails to load, or a listed card can't be resolved.
#[cfg(test)]
pub fn build_game() -> Result<GameState> {
    build_game_with_seed(SEED)
}

/// Loads canon + builtin + the generated `wizards` corpus and the two demo
/// decklists, and assembles a two-player Goblins-vs-Elves game shuffled with
/// `seed`. The binary passes a `--seed` value or an entropy-derived one (so
/// live play varies); tests use the fixed-seed `build_game`.
///
/// The demo's cards are produced locally (not shipped): `wizards` is the
/// gitignored generated corpus (`cargo xtask generate plugins/wizards`), so the
/// decklists resolve against canon staples (Lightning Bolt, basics) plus the
/// rest of each card materialized from bulk data.
///
/// # Errors
/// If a plugin or decklist fails to load, or a listed card can't be resolved.
pub fn build_game_with_seed(seed: u64) -> Result<GameState> {
    let canon = Plugin::load_with_sibling_prelude(data("../../plugins/canon"))?;
    let builtin = Plugin::load(data("../../plugins/builtin"))?;
    let wizards = Plugin::load_with_prelude(&builtin, data("../../plugins/wizards"))?;

    let goblins = Deck::load(&data("../../plugins/demo/decks/goblins.txt"))?;
    let elves = Deck::load(&data("../../plugins/demo/decks/elves.txt"))?;

    let p0 = goblins.resolve(&[&canon, &builtin, &wizards])?;
    let p1 = elves.resolve(&[&canon, &builtin, &wizards])?;

    let sba_rules = canon
        .sba_rules
        .iter()
        .chain(builtin.sba_rules.iter())
        .chain(wizards.sba_rules.iter())
        .cloned()
        .collect();

    let conferral_rules = canon
        .conferral_rules
        .iter()
        .chain(builtin.conferral_rules.iter())
        .chain(wizards.conferral_rules.iter())
        .cloned()
        .collect();

    let damage_result_rules = canon
        .damage_result_rules
        .iter()
        .chain(builtin.damage_result_rules.iter())
        .chain(wizards.damage_result_rules.iter())
        .cloned()
        .collect();

    let mut counter_decls = std::collections::HashMap::new();
    counter_decls.extend(canon.counters.clone());
    counter_decls.extend(builtin.counters.clone());
    counter_decls.extend(wizards.counters.clone());

    // Subtype registry ([CR#205.3]): the engine resolves a layer-4
    // `Subtypes(...)` modification's `SubtypeRef` names against this map.
    // Last plugin wins, mirroring `counter_decls`.
    let mut subtypes = std::collections::HashMap::new();
    subtypes.extend(canon.subtypes.clone());
    subtypes.extend(builtin.subtypes.clone());
    subtypes.extend(wizards.subtypes.clone());

    // Type registry ([CR#300.1]): the engine resolves a layer-4
    // `CardTypes(...)` modification's bare `Ident` names against this map.
    // Last plugin wins, mirroring `subtypes`.
    let mut types = std::collections::HashMap::new();
    types.extend(canon.types.clone());
    types.extend(builtin.types.clone());
    types.extend(wizards.types.clone());

    Ok(GameState::new(GameConfig {
        players: vec![
            PlayerConfig { deck: p0.to_vec() },
            PlayerConfig { deck: p1.to_vec() },
        ],
        seed,
        starting_life: 20,
        starting_player: StartingPlayer::Fixed(PlayerId(0)),
        sba_rules,
        conferral_rules,
        damage_result_rules,
        counter_decls,
        subtypes,
        types,
    }))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[cfg_attr(
        not(wizards_corpus),
        ignore = "requires generated plugins/wizards corpus"
    )]
    fn builds_two_player_twenty_life_game() {
        let state = build_game().expect("build demo game");
        assert_eq!(state.players.len(), 2);
        assert_eq!(state.players[0].life, 20);
        assert_eq!(state.players[1].life, 20);
    }

    /// The demo seed is fixed for reproducibility, so whatever opening hand it
    /// deals is dealt on every run. Guard that the chosen seed never reverts to
    /// dealing a degenerate hand — in particular the red Goblins deck (P0),
    /// which a prior seed opened with zero lands every time. Both decks run 14
    /// lands in 40, so a keepable 2–5 land hand is the sensible window.
    #[test]
    #[cfg_attr(
        not(wizards_corpus),
        ignore = "requires generated plugins/wizards corpus"
    )]
    fn opening_hands_are_keepable() {
        use deckmaste_core::Type;
        let state = build_game().expect("build demo game");
        let view = state.layers();
        for (i, label) in ["Goblins (red, P0)", "Elves (green, P1)"]
            .iter()
            .enumerate()
        {
            let lands = state.zones.hands[i]
                .iter()
                .filter(|&&id| view.get(id).has_type(Type::Land))
                .count();
            assert!(
                (2..=5).contains(&lands),
                "{label} opens with {lands} lands; expected a keepable 2-5"
            );
        }
    }

    /// The runtime type registry ([CR#300.1]) assembled from canon+builtin+
    /// wizards mirrors the subtype registry: bare names resolve, and each
    /// `TypeDef` carries the correct `permanent` flag from the plugin data.
    #[test]
    #[cfg_attr(
        not(wizards_corpus),
        ignore = "requires generated plugins/wizards corpus"
    )]
    fn game_state_carries_the_type_registry() {
        let state = build_game().expect("build demo game");
        assert!(
            state.types.contains_key("Land"),
            "Land resolves in the runtime registry"
        );
        assert!(state.types.contains_key("Creature"));
        assert!(!state.types["Instant"].permanent);
    }

    /// End-to-end: both seats are auto-developed by `GreedyDemo` (which, unlike
    /// `GreedyCreatures`, also chooses legal targets for the burn / sac-outlet
    /// pings these decks cast), so the tribal lords, Krenko's token scaling,
    /// sac outlets, and burn all reach the battlefield. A lord-filled board
    /// exercises the continuous-effect carrier (engine-static-scope-carrier)
    /// that used to panic every layer rebuild — so this finishing at all is the
    /// demo's real proof of life.
    #[test]
    #[cfg_attr(
        not(wizards_corpus),
        ignore = "requires generated plugins/wizards corpus"
    )]
    fn demo_auto_plays_to_completion() {
        use deckmaste_engine::sim::GreedyDemo;

        use crate::driver::Driver;
        use crate::driver::HEADLESS_BUDGET;
        use crate::driver::Stop;

        let state = build_game().expect("build demo game");
        let mut driver = Driver::new(state, Box::new(GreedyDemo));
        match driver
            .run_to_end(HEADLESS_BUDGET)
            .expect("no decision error")
        {
            Stop::GameOver(_) => {}
            other => panic!("demo did not play to completion: {other:?}"),
        }
    }
}
