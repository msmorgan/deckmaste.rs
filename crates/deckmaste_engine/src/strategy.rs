//! Strategy evaluation context: the `Frame` a data-driven strategy's sensing
//! (`Condition`/`Count`/`Reference`) is evaluated against. `Reference::You`
//! binds to the deciding seat; `Reference::This`/`~` binds to the candidate
//! option being scored. The engine's existing
//! `eval_count`/`condition_holds`/`eval_reference` do the rest — there is no
//! second evaluator. `strategy-evaluator-core` builds the `StrategyEvaluator`
//! on top of this.

use crate::object::ObjectId;
use crate::player::PlayerId;
use crate::stack::Frame;
use crate::state::GameState;

/// The evaluation frame for scoring a `candidate` option from `seat`'s
/// perspective: `Reference::You` resolves to `seat`, and `Reference::This`/`~`
/// resolves to `candidate` — or, when there is no candidate (player-only
/// sensing), to `seat`'s own player proxy. Sensing only: no targets, trigger
/// bindings, choice, or X. The engine's `eval_count`/`condition_holds`/
/// `eval_reference` evaluate a strategy's `Count`/`Condition`/`Reference`
/// against this exactly as they do during effect resolution.
// Production consumer is `strategy-evaluator-core` (the next ticket); for now
// only the spike's tests exercise it.
#[allow(dead_code)]
pub(crate) fn eval_frame(state: &GameState, seat: PlayerId, candidate: Option<ObjectId>) -> Frame {
    Frame {
        source: candidate.unwrap_or_else(|| state.player(seat).object),
        controller: seat,
        targets: Vec::new(),
        bindings: None,
        chosen: None,
        x: None,
        subject: None,
    }
}

#[cfg(test)]
mod tests {
    use std::path::Path;
    use std::sync::Arc;

    use deckmaste_cards::plugin::Plugin;
    use deckmaste_core::Cmp;
    use deckmaste_core::Condition;
    use deckmaste_core::Count;
    use deckmaste_core::Reference;
    use deckmaste_core::Stat;
    use deckmaste_core::Zone;

    use super::eval_frame;
    use crate::object::ObjectSource;
    use crate::player::PlayerId;
    use crate::state::GameConfig;
    use crate::state::GameState;
    use crate::state::PlayerConfig;
    use crate::state::StartingPlayer;

    fn canon() -> Plugin {
        Plugin::load_with_sibling_prelude(
            Path::new(env!("CARGO_MANIFEST_DIR")).join("../../plugins/canon"),
        )
        .unwrap()
    }

    /// The R1 spike: a *synthesized* frame (no resolving effect) drives the
    /// engine's existing evaluators. `You` → the seat, `This` → the candidate,
    /// and `Count`/`Condition` read live state. This is the whole premise of
    /// data-driven strategies — sensing reuses the card evaluators verbatim.
    #[test]
    fn synthesized_frame_drives_engine_evaluators() {
        let bears = Arc::new(canon().card("Grizzly Bears").unwrap());
        let mut state = GameState::new(GameConfig {
            players: vec![
                PlayerConfig {
                    deck: vec![Arc::clone(&bears); 10],
                },
                PlayerConfig { deck: vec![] },
            ],
            seed: 1,
            starting_life: 20,
            starting_player: StartingPlayer::Fixed(PlayerId(0)),
        });

        // A 2/2 Grizzly Bears on P0's battlefield — the candidate being scored.
        let bear_card = state.cards.push(Arc::clone(&bears), PlayerId(0));
        let bear = state.objects.mint(
            ObjectSource::Card(bear_card),
            PlayerId(0),
            Some(Zone::Battlefield),
        );
        state.zones.battlefield.push(bear);

        let frame = eval_frame(&state, PlayerId(0), Some(bear));

        // `You` → the seat's player proxy; `This` → the candidate object.
        assert_eq!(
            state.eval_reference(&Reference::You, &frame),
            state.player(PlayerId(0)).object,
        );
        assert_eq!(state.eval_reference(&Reference::This, &frame), bear);

        // A `Count` over the candidate: Grizzly Bears' power is 2.
        assert_eq!(
            state.eval_count(&Count::StatOf(Reference::This, Stat::Power), &frame),
            2,
        );

        // A `Condition` comparing the candidate's power against a literal: 2 >= 2.
        let cond = Condition::Compare(
            Count::StatOf(Reference::This, Stat::Power),
            Cmp::AtLeast,
            Count::Literal(2),
        );
        assert!(state.condition_holds(&cond, &frame));
    }

    /// With no candidate, `This` falls back to the seat's own player proxy —
    /// the shape for player-only sensing (mulligan keep/ship, life totals).
    #[test]
    fn frame_without_candidate_binds_this_to_seat_proxy() {
        let state = GameState::new(GameConfig {
            players: vec![PlayerConfig { deck: vec![] }, PlayerConfig { deck: vec![] }],
            seed: 7,
            starting_life: 20,
            starting_player: StartingPlayer::Fixed(PlayerId(0)),
        });
        let frame = eval_frame(&state, PlayerId(1), None);
        assert_eq!(
            state.eval_reference(&Reference::You, &frame),
            state.player(PlayerId(1)).object,
        );
        assert_eq!(
            state.eval_reference(&Reference::This, &frame),
            state.player(PlayerId(1)).object,
        );
    }
}
