use crate::decide::Action;
use crate::decide::Decision;
use crate::decide::DecisionError;
use crate::decide::DecisionHandler;
use crate::player::PlayerId;
use crate::state::GameState;

/// [CR#117]: the holder may act or pass. `legal` is advisory UI data —
/// submission re-validates.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Priority {
    pub player: PlayerId,
    pub legal: Vec<Action>,
}

impl DecisionHandler for Priority {
    fn resolve(self, g: &mut GameState, answer: Decision) -> Result<(), DecisionError> {
        let Decision::Act(action) = answer else {
            return Err(DecisionError::WrongKind);
        };
        if !self.legal.contains(&action) {
            return Err(DecisionError::Illegal {
                reason: format!("{action:?} is not a legal action right now"),
            });
        }
        let player = self.player;
        g.pending = None;
        g.take_priority_action(player, &action);
        Ok(())
    }
}
