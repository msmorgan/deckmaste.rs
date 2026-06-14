//! Targeting ([CR#115]): which objects a `Filter` admits. Stage 2 wires only
//! the arms the corpus's `AnyTarget` reaches; the rest are `todo!`.

use deckmaste_core::CharacteristicFilter;
use deckmaste_core::Filter;
use deckmaste_core::ObjectKind;
use deckmaste_core::StateFilter;
use deckmaste_core::Type;
use deckmaste_core::Zone;

use crate::object::ObjectId;
use crate::object::ObjectSource;
use crate::state::GameState;

/// The object's kind ([CR#109.1]) as the corpus needs it: a player proxy is a
/// `Player`; a card on the stack is a `Spell`; a created token is a `Token`
/// ([CR#111.6] — not a card); otherwise a `Card`. The stack check outranks
/// the token check: a stack entry minted from a token source (its activated
/// ability) is an ability on the stack, not the token itself.
#[must_use]
pub fn object_kind(state: &GameState, id: ObjectId) -> ObjectKind {
    let obj = state.objects.obj(id);
    match obj.source {
        ObjectSource::Player(_) => ObjectKind::Player,
        ObjectSource::Card(_) if obj.zone == Some(Zone::Stack) => ObjectKind::Spell,
        ObjectSource::Card(c) if state.cards.get(c).is_token => ObjectKind::Token,
        ObjectSource::Card(_) => ObjectKind::Card,
    }
}

/// Whether `id` matches `filter`. Only the corpus-reachable arms are wired.
#[must_use]
pub fn matches(state: &GameState, id: ObjectId, filter: &Filter) -> bool {
    match filter {
        Filter::Kind(k) => object_kind(state, id) == *k,
        Filter::Characteristic(CharacteristicFilter::Type(t)) => has_type(state, id, *t),
        // [CR#110.5a]: state, not characteristic — card/token objects only, so a
        // player proxy (zone None) never matches InZone.
        Filter::State(StateFilter::InZone(z)) => state.objects.obj(id).zone == Some(*z),
        Filter::AllOf(fs) => fs.iter().all(|f| matches(state, id, f)),
        Filter::OneOf(fs) => fs.iter().any(|f| matches(state, id, f)),
        Filter::Not(f) => !matches(state, id, f),
        Filter::Any => true,
        // A filter-position macro (`kinds: [Filter]` — `Self`, evasion sets,
        // protection qualities) survives expansion as `Filter::Expanded`;
        // evaluate it transparently through the remembered body. (Target-position
        // macros like `AnyTarget` are looked through earlier, in
        // `resolve::target_spec_filter`, so they never reach here.)
        Filter::Expanded(e) => matches(state, id, &e.value),
        // [CR#702]: keyword presence by NAME against the DERIVED abilities
        // (granted keywords count; the carried Composite name survives
        // expansion). Per-call layers() rebuild — a perf seam if a hot path
        // ever evaluates Has in bulk.
        Filter::Characteristic(CharacteristicFilter::Has(kw)) => {
            let view = state.layers();
            view.get(id)
                .abilities
                .iter()
                .any(|a| crate::layer::ability_is_named(a, &kw.0))
        }
        // Subtype presence by NAME against the DERIVED subtype list
        // ([CR#205.3] — layer-4 type changes count); a player proxy has
        // none. Same per-call layers() perf seam as `Has`.
        Filter::Characteristic(CharacteristicFilter::Subtype(name)) => {
            state.objects.obj(id).card_id().is_some()
                && state
                    .layers()
                    .get(id)
                    .subtypes
                    .iter()
                    .any(|s| s.name == *name)
        }
        // [CR#122.1] counters go on objects AND players — player counters
        // live on the player's proxy object, so one LIVE read serves both.
        Filter::State(deckmaste_core::StateFilter::HasCounter(kind)) => state
            .objects
            .obj(id)
            .counters
            .get(kind)
            .is_some_and(|&n| n > 0),
        // [CR#109.3] designations are non-characteristic state: a LIVE read
        // against the registry (object entry, or the player's for proxies).
        Filter::State(deckmaste_core::StateFilter::Designated(name)) => {
            state
                .designations
                .objects
                .get(&(id, *name))
                .is_some_and(|instances| !instances.is_empty())
                || matches!(
                    state.objects.obj(id).source,
                    crate::object::ObjectSource::Player(p)
                        if state.designations.players.contains_key(&(p, *name)))
        }
        other => todo!("stage 2 does not evaluate filter {other:?}"),
    }
}

/// Whether a card object has card type `ty` in the DERIVED view ([CR#613.1d]
/// layer-4 type changes count — an animated land or crewed Vehicle types as a
/// Creature); a player proxy has none. Same per-call `layers()` perf seam as
/// `Has`/`Subtype`.
fn has_type(state: &GameState, id: ObjectId, ty: Type) -> bool {
    state.objects.obj(id).card_id().is_some() && state.layers().get(id).card_types.contains(&ty)
}

/// Every object (card objects in their zones + player proxies) matching
/// `filter`, in deterministic id order.
#[must_use]
pub fn candidates(state: &GameState, filter: &Filter) -> Vec<ObjectId> {
    state
        .objects
        .iter()
        .map(|o| o.id)
        .filter(|&id| matches(state, id, filter))
        .collect()
}

#[cfg(test)]
mod tests {
    use std::path::Path;
    use std::sync::Arc;

    use deckmaste_cards::plugin::Plugin;
    use deckmaste_core::Filter;
    use deckmaste_core::TargetSpec;
    use deckmaste_core::Zone;

    use super::*;
    use crate::player::PlayerId;
    use crate::state::GameConfig;
    use crate::state::GameState;
    use crate::state::PlayerConfig;
    use crate::state::StartingPlayer;

    fn builtin() -> Plugin {
        Plugin::load(Path::new(env!("CARGO_MANIFEST_DIR")).join("../../plugins/builtin")).unwrap()
    }

    fn canon() -> Plugin {
        Plugin::load_with_sibling_prelude(
            Path::new(env!("CARGO_MANIFEST_DIR")).join("../../plugins/canon"),
        )
        .unwrap()
    }

    /// A two-player game; player 0's deck is Grizzly Bears, player 1's is
    /// Forest. Returns the state plus a creature object moved onto the
    /// battlefield.
    fn game_with_a_bear_on_the_field() -> (GameState, ObjectId) {
        let bears = Arc::new(canon().card("Grizzly Bears").unwrap());
        let forest = Arc::new(builtin().card("Forest").unwrap());
        let mut state = GameState::new(GameConfig {
            players: vec![
                PlayerConfig {
                    deck: vec![Arc::clone(&bears); 10],
                },
                PlayerConfig {
                    deck: vec![Arc::clone(&forest); 10],
                },
            ],
            seed: 1,
            starting_life: 20,
            starting_player: StartingPlayer::Fixed(PlayerId(0)),
        });
        // Force a Grizzly Bears from player 0's hand onto the battlefield.
        let bear = *state.zones.hands[0]
            .iter()
            .find(|&&o| {
                matches(
                    &state,
                    o,
                    &Filter::Characteristic(deckmaste_core::CharacteristicFilter::Type(
                        deckmaste_core::Type::Creature,
                    )),
                )
            })
            .expect("a Grizzly Bears in the opening hand (10-card mono deck)");
        state.remove_from_hand(PlayerId(0), bear);
        state.objects.obj_mut(bear).zone = Some(Zone::Battlefield);
        state.zones.battlefield.push(bear);
        (state, bear)
    }

    /// A two-player game (P0 Grizzly Bears, P1 Forest) with one of P1's
    /// Forests forced onto the battlefield. Returns the state plus the land
    /// object.
    fn game_with_a_forest_on_the_field() -> (GameState, ObjectId) {
        let bears = Arc::new(canon().card("Grizzly Bears").unwrap());
        let forest = Arc::new(builtin().card("Forest").unwrap());
        let mut state = GameState::new(GameConfig {
            players: vec![
                PlayerConfig {
                    deck: vec![Arc::clone(&bears); 10],
                },
                PlayerConfig {
                    deck: vec![Arc::clone(&forest); 10],
                },
            ],
            seed: 1,
            starting_life: 20,
            starting_player: StartingPlayer::Fixed(PlayerId(1)),
        });
        // Force a Forest from player 1's hand onto the battlefield.
        let land = *state.zones.hands[1]
            .iter()
            .find(|&&o| {
                matches(
                    &state,
                    o,
                    &Filter::Characteristic(deckmaste_core::CharacteristicFilter::Type(
                        deckmaste_core::Type::Land,
                    )),
                )
            })
            .expect("a Forest in the opening hand (10-card mono deck)");
        state.remove_from_hand(PlayerId(1), land);
        state.objects.obj_mut(land).zone = Some(Zone::Battlefield);
        state.zones.battlefield.push(land);
        (state, land)
    }

    /// Regression ([CR#613.1d]): `Filter::Type` reads the DERIVED type, not the
    /// printed face. A battlefield land animated into a creature by a layer-4
    /// continuous effect matches `Type(Creature)` — the view says Creature, so
    /// the matcher must agree. (Before the fix `has_type` read the printed face
    /// and reported the land was not a creature, mis-typing every `candidates`
    /// / targeting caller.)
    #[test]
    fn type_filter_reads_derived_type_for_animated_land() {
        use deckmaste_core::Duration;
        use deckmaste_core::Modification;

        use crate::layer::ContinuousEffect;
        use crate::layer::ScopeResolved;
        use crate::object::Timestamp;

        let (mut state, land) = game_with_a_forest_on_the_field();
        let creature = Filter::Characteristic(deckmaste_core::CharacteristicFilter::Type(
            deckmaste_core::Type::Creature,
        ));
        // Sanity: a plain Forest is not a creature.
        assert!(
            !matches(&state, land, &creature),
            "a plain Forest is not a creature"
        );

        // Animate it: a layer-4 effect adds the Creature type ([CR#613.1d]).
        state.continuous.push(ContinuousEffect {
            timestamp: Timestamp(1_000),
            controller: PlayerId(1),
            scope: ScopeResolved::Locked(vec![land]),
            changes: vec![Modification::AddCardTypes(vec![
                deckmaste_core::Type::Creature,
            ])],
            duration: Duration::EndOfGame,
            is_cda: false,
        });

        // The derived view says Creature...
        assert!(
            state
                .layers()
                .get(land)
                .card_types
                .contains(&deckmaste_core::Type::Creature),
            "sanity: the animated land derives as a creature"
        );
        // ...and the Type filter must agree ([CR#613.1d]).
        assert!(
            matches(&state, land, &creature),
            "Filter::Type reads the derived type — the animated land matches Type(Creature)"
        );
    }

    #[test]
    fn any_target_is_creatures_and_players_not_lands() {
        // `read_str` returns the remembered `TargetSpec::Expanded(AnyTarget)`.
        // `resolve::target_spec_filter` is the engine's own TargetSpec→Filter
        // extraction — the path real targeting funnels through — so the test
        // exercises it rather than hand-unwrapping the expansion.
        let any_target: TargetSpec = builtin().macros.read_str("AnyTarget").unwrap();
        let filter = crate::resolve::target_spec_filter(&any_target);
        let (state, bear) = game_with_a_bear_on_the_field();
        let targets = candidates(&state, filter);
        // Both player proxies + the lone battlefield creature; no lands (in
        // hand/library), no spells (stack empty).
        assert!(targets.contains(&bear));
        assert!(targets.contains(&state.players[0].object));
        assert!(targets.contains(&state.players[1].object));
        assert_eq!(targets.len(), 3);
    }

    /// A filter-position macro (`kinds: [Filter]`) survives expansion as
    /// `Filter::Expanded`; `matches` must look through it transparently.
    /// Guards the delegation arm against being mistaken for dead code.
    #[test]
    fn matches_looks_through_a_filter_macro() {
        // `CreatureOrPlayer` reads as `Filter::Expanded(.., value: OneOf([..]))`:
        // the invocation survives, wrapping its expanded body.
        let wrapped: Filter = builtin().macros.read_str("CreatureOrPlayer").unwrap();
        assert!(
            matches!(wrapped, Filter::Expanded(_)),
            "a filter macro should survive as Filter::Expanded, got {wrapped:?}"
        );
        let (state, bear) = game_with_a_bear_on_the_field();
        // Evaluating the wrapped macro reaches the battlefield creature through
        // the remembered body — delegation is transparent.
        assert!(candidates(&state, &wrapped).contains(&bear));
    }
}
