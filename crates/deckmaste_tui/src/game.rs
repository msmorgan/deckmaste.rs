//! Builds the demo `GameState` from committed plugin data + decklist files.
use std::path::Path;
use std::path::PathBuf;
use std::sync::Arc;

use anyhow::Result;
use deckmaste_engine::GameConfig;
use deckmaste_engine::GameState;
use deckmaste_engine::PlayerConfig;
use deckmaste_engine::PlayerId;
use deckmaste_engine::StartingPlayer;
use deckmaste_plugin::Deck;
use deckmaste_plugin::plugin::Plugin;
use deckmaste_plugin::provenance::ProvenanceIndex;

/// The semantic half of every card in the game, indexed by the engine's
/// `CardId`.
///
/// A `Vec`, not a map: `CardId` is a dense index into the game's card table,
/// and the deck cards occupy the whole prefix of it. Tokens and emblems are
/// minted later, past the end, and resolve `None` — their prose comes from the
/// ability index, not from a semantic card.
pub struct CardProvenance {
    by_card: Vec<Arc<deckmaste_semantics::Card>>,
}

impl CardProvenance {
    /// The semantic card behind a `CardId`, or `None` for a token or emblem.
    #[must_use]
    pub fn get(&self, id: deckmaste_engine::CardId) -> Option<&Arc<deckmaste_semantics::Card>> {
        self.by_card.get(usize::try_from(id.0).ok()?)
    }
}

/// The provenance one render pass needs, bundled so it threads as a single
/// parameter: the `CardId` companion table plus the value-keyed index.
#[derive(Clone, Copy)]
pub struct ProvenanceRefs<'a> {
    pub cards: &'a CardProvenance,
    pub index: &'a ProvenanceIndex,
}

/// A built demo game: the engine state, the `CardId` companion table, and the
/// provenance index the detail pane renders through.
pub struct BuiltGame {
    pub state: GameState,
    pub cards: CardProvenance,
    pub provenance: ProvenanceIndex,
}

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
pub fn build_game() -> Result<BuiltGame> {
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
pub fn build_game_with_seed(seed: u64) -> Result<BuiltGame> {
    let canon = Plugin::load_with_sibling_prelude(data("../../plugins/canon"))?;
    let builtin = Plugin::load(data("../../plugins/builtin"))?;
    let wizards = Plugin::load_with_prelude(&builtin, data("../../plugins/wizards"))?;

    let goblins = Deck::load(&data("../../plugins/demo/decks/goblins.txt"))?;
    let elves = Deck::load(&data("../../plugins/demo/decks/elves.txt"))?;

    let loaded0 = goblins.resolve(&[&canon, &builtin, &wizards])?;
    let loaded1 = elves.resolve(&[&canon, &builtin, &wizards])?;

    // Both halves of the pair survive here. The engine gets the core half; the
    // semantic half becomes the provenance the renderer needs, because lowering
    // erases invocation provenance and a core value carries no template
    // (docs/decisions/semantics-spelling-lowering.md §12).
    let p0: Vec<Arc<deckmaste_card::Card>> =
        loaded0.iter().map(|l| Arc::new(l.core.clone())).collect();
    let p1: Vec<Arc<deckmaste_card::Card>> =
        loaded1.iter().map(|l| Arc::new(l.core.clone())).collect();

    // `CardId` is a dense index assigned at setup in deck order, pre-shuffle:
    // each player's deck in player order, then the library is shuffled. So
    // flattening the decks in that same order makes index == CardId. The
    // engine cannot build this itself — it must not learn about
    // `deckmaste_semantics` — which is why the zip lives here and why
    // `pins_card_ids_to_deck_order` guards it.
    let card_provenance = CardProvenance {
        by_card: loaded0
            .iter()
            .chain(loaded1.iter())
            .map(|l| Arc::new(l.semantic.clone()))
            .collect(),
    };

    // Registry conferral first (it is the same for every game), then each
    // card's own abilities and everything nested inside them.
    //
    // Folded in the SAME plugin order as the `subtypes`/`types`/`counters`
    // registries below, and `ProvenanceIndex::extend` resolves its name-keyed
    // tables last-plugin-wins to match them, so a subtype a later plugin
    // redefines raises to the definition the engine is actually using. The
    // index's ability map is first-insert-wins instead, and deliberately so:
    // it is keyed by lowered VALUE, not by name, so it carries no plugin
    // precedence to mirror — every preimage is semantically exact
    // (`docs/decisions/semantics-spelling-lowering.md` §9) and the rule there
    // picks the best SPELLING, not the winning plugin.
    let mut provenance = ProvenanceIndex::default();
    provenance.extend(&canon.provenance);
    provenance.extend(&builtin.provenance);
    provenance.extend(&wizards.provenance);
    for loaded in loaded0.iter().chain(loaded1.iter()) {
        provenance.insert_card(&loaded.semantic);
    }

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
    // Last plugin wins, mirroring `counter_decls` — and mirrored in turn by
    // the provenance index's name-keyed tables above.
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

    let state = GameState::new(GameConfig {
        players: vec![PlayerConfig { deck: p0 }, PlayerConfig { deck: p1 }],
        seed,
        starting_life: 20,
        starting_player: StartingPlayer::Fixed(PlayerId(0)),
        sba_rules,
        conferral_rules,
        damage_result_rules,
        counter_decls,
        subtypes,
        types,
    });
    Ok(BuiltGame {
        state,
        cards: card_provenance,
        provenance,
    })
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
        let state = build_game().expect("build demo game").state;
        assert_eq!(state.players.len(), 2);
        assert_eq!(state.players[0].life, 20);
        assert_eq!(state.players[1].life, 20);
    }

    /// The zip in [`build_game_with_seed`] is an ASSUMPTION about the engine:
    /// `GameState::new` assigns `CardId`s densely, in player order, in deck
    /// order, before the shuffle — so flattening the two resolved decklists in
    /// that same order makes index == `CardId`. Nothing type-checks that, and
    /// getting it wrong would give every object the wrong card's semantic half
    /// (a silently misattributed detail pane, not a crash). Pin it by name:
    /// the semantic card at each `CardId` must be the card the engine holds
    /// there.
    ///
    /// The tail of the invariant matters too — the table covers exactly the
    /// deck cards, so the first `CardId` past it (a token or emblem, minted
    /// later by `Cards::push_token`) must resolve `None` rather than
    /// wrapping onto some unrelated deck card.
    #[test]
    #[cfg_attr(
        not(wizards_corpus),
        ignore = "requires generated plugins/wizards corpus"
    )]
    fn pins_card_ids_to_deck_order() {
        use deckmaste_engine::CardId;

        let game = build_game().expect("build demo game");
        let count = game.state.cards.len();
        assert!(count > 0, "the demo game has cards");
        for i in 0..count {
            let id = CardId(u32::try_from(i).expect("card count fits in a CardId"));
            let engine = &deckmaste_engine::face(&game.state.cards.get(id).def)
                .characteristics
                .name;
            let semantic = game
                .cards
                .get(id)
                .unwrap_or_else(|| panic!("{id:?} has a semantic half"));
            let semantic = match &**semantic {
                deckmaste_semantics::Card::Normal(f) => &f.name,
                deckmaste_semantics::Card::TwoFaced { front, .. } => &front.name,
            };
            assert_eq!(
                &**semantic, &**engine,
                "the semantic card at {id:?} is not the card the engine holds there",
            );
        }
        let past_end = CardId(u32::try_from(count).expect("card count fits in a CardId"));
        assert!(
            game.cards.get(past_end).is_none(),
            "a token's {past_end:?} must resolve to no semantic card",
        );
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
        let state = build_game().expect("build demo game").state;
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
    /// `TypeDef` carries the correct `permanent_type` flag from the plugin
    /// data.
    #[test]
    #[cfg_attr(
        not(wizards_corpus),
        ignore = "requires generated plugins/wizards corpus"
    )]
    fn game_state_carries_the_type_registry() {
        let state = build_game().expect("build demo game").state;
        assert!(
            state.types.contains_key("Land"),
            "Land resolves in the runtime registry"
        );
        assert!(state.types.contains_key("Creature"));
        assert!(!state.types["Instant"].permanent_type);
    }

    /// End-to-end: both seats are auto-developed by `GreedyDemo` (which, unlike
    /// `GreedyCreatures`, also chooses legal targets for the burn / sac-outlet
    /// pings these decks cast), so the tribal lords, Krenko's token scaling,
    /// sac outlets, and burn all reach the battlefield. A lord-filled board
    /// exercises the continuous-effect carrier (engine-static-scope-carrier)
    /// that used to panic every layer rebuild — so this finishing at all is the
    /// demo's real proof of life.
    #[cfg(feature = "slow-tests")]
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

        let game = build_game().expect("build demo game");
        let mut driver = Driver::new(game, Box::new(GreedyDemo));
        match driver
            .run_to_end(HEADLESS_BUDGET)
            .expect("no decision error")
        {
            Stop::GameOver(_) => {}
            other => panic!("demo did not play to completion: {other:?}"),
        }
    }
}
