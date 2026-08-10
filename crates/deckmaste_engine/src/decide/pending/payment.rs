use crate::decide::Decision;
use crate::decide::DecisionError;
use crate::decide::DecisionHandler;
use crate::payment::PaymentPrompt;
use crate::state::GameState;

impl DecisionHandler for PaymentPrompt {
    fn resolve(self, game: &mut GameState, answer: Decision) -> Result<(), DecisionError> {
        let Decision::Payment(command) = answer else {
            return Err(DecisionError::WrongKind);
        };
        game.submit_payment_command(command)
    }
}
