use deckmaste_core::Uint;

use crate::agenda::WorkItem;
use crate::decide::Decision;
use crate::decide::DecisionError;
use crate::decide::DecisionHandler;
use crate::event::GameEvent;
use crate::event::Occurrence;
use crate::player::PlayerId;
use crate::state::GameState;

/// [CR#106.1b]: a resolving `AddMana` whose production is a choice ("any
/// color" offers the five colors per [CR#105.4]; "{W} or {U}" offers its
/// printed set) — `player` picks one of `options`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ChooseManaColor {
    pub player: PlayerId,
    pub options: Vec<deckmaste_core::ColorOrColorless>,
    pub amount: Uint,
    pub riders: Vec<deckmaste_core::ManaRider>,
}

impl DecisionHandler for ChooseManaColor {
    fn resolve(self, g: &mut GameState, answer: Decision) -> Result<(), DecisionError> {
        let Decision::ManaColor(mana) = answer else {
            return Err(DecisionError::WrongKind);
        };
        // [CR#106.1b]: the choice is drawn from the offered set.
        if !self.options.contains(&mana) {
            return Err(DecisionError::Illegal {
                reason: format!("{mana:?} is not one of the offered mana options"),
            });
        }
        let (player, amount, riders) = (self.player, self.amount, self.riders);
        g.pending = None;
        g.schedule_front(vec![WorkItem::Emit(Occurrence::single(
            GameEvent::ManaAdded {
                player,
                mana,
                amount,
                riders,
            },
        ))]);
        Ok(())
    }
}

/// [CR#106.1b]: a resolving `AddMana` whose production is a choice among
/// multi-symbol runs (the filterland cycle "{W}{W}, {W}{U}, or {U}{U}") —
/// `player` picks one of `options` (each a run of mana), and the chosen
/// run's whole sequence lands. The answer ([`Decision::ManaMode`]) is the
/// chosen option index.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ChooseManaMode {
    pub player: PlayerId,
    pub options: Vec<Vec<deckmaste_core::ColorOrColorless>>,
    pub amount: Uint,
    pub riders: Vec<deckmaste_core::ManaRider>,
}

impl DecisionHandler for ChooseManaMode {
    fn resolve(self, g: &mut GameState, answer: Decision) -> Result<(), DecisionError> {
        let Decision::ManaMode(choice) = answer else {
            return Err(DecisionError::WrongKind);
        };
        // [CR#106.1b]: the chosen run is drawn from the offered set.
        let run = usize::try_from(choice)
            .ok()
            .and_then(|i| self.options.get(i))
            .ok_or_else(|| DecisionError::Illegal {
                reason: format!("{choice} is not one of the offered mana runs"),
            })?;
        // The whole run's mana lands at once — one `ManaAdded` per
        // symbol in printed order, each carrying the shared riders.
        let (player, amount, riders) = (self.player, self.amount, self.riders);
        let events = run
            .iter()
            .map(|&mana| GameEvent::ManaAdded {
                player,
                mana,
                amount,
                riders: riders.clone(),
            })
            .collect();
        g.pending = None;
        g.schedule_front(vec![WorkItem::Emit(Occurrence::Batch(events))]);
        Ok(())
    }
}
