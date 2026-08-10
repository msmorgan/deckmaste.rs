use crate::decide::Decision;
use crate::decide::DecisionError;
use crate::decide::DecisionHandler;
use crate::payment::PaymentPrompt;
use crate::player::ManaActionId;
use crate::player::PlayerId;
use crate::state::GameState;

/// The legal [CR#733] sets of activated mana actions that may be reversed
/// while abandoning an announcement. Each entry is a complete set; the
/// engine does not repair an illegal partial answer.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ChooseManaReversals {
    pub player: PlayerId,
    pub legal: Vec<Vec<ManaActionId>>,
}

impl DecisionHandler for PaymentPrompt {
    fn resolve(self, game: &mut GameState, answer: Decision) -> Result<(), DecisionError> {
        let Decision::Payment(command) = answer else {
            return Err(DecisionError::WrongKind);
        };
        game.submit_payment_command(command)
    }
}

impl DecisionHandler for ChooseManaReversals {
    fn resolve(self, game: &mut GameState, answer: Decision) -> Result<(), DecisionError> {
        let Decision::ManaReversals(actions) = answer else {
            return Err(DecisionError::WrongKind);
        };
        if !self.legal.contains(&actions) {
            return Err(DecisionError::Illegal {
                reason: "the submitted mana-reversal set is not legal".into(),
            });
        }
        game.submit_mana_reversals(actions)
    }
}
