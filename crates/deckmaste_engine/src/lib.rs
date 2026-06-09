//! The rules engine: a manually-steppable [`GameState`] (one event per
//! [`GameState::step`]) and a [`Runner`] that auto-steps to decision points.
//!
//! Control flow is reified: the agenda (a work queue) holds what the engine
//! does next, so a `step()` can return mid-cascade and tests can assert
//! state between any two events.

mod agenda;
pub use agenda::WorkItem;

mod cast;
pub use cast::Payment;

mod combat;
pub use combat::{CombatState, has_keyword};

mod decide;
pub use decide::{Action, Decision, DecisionError, PendingDecision};

mod derive;

mod event;
pub use event::{EnterStatus, GameEvent, LossReason, Occurrence};

mod lki;
pub use lki::LkiSnapshot;

mod legal;
pub use legal::{legal_attackers, legal_blockers};

mod object;
pub use object::{CardId, CardInstance, Cards, GameObject, ObjectId, ObjectSource, ObjectStore};

mod player;
pub use player::{ManaPool, PlayerId, PlayerState};

mod tally;
pub use tally::{Tallies, Tally};

mod replace;

mod resolve;

mod runner;
pub use runner::{RunStop, Runner};

mod sba;

mod stack;
pub use stack::{Frame, PendingStackEntry, StackEntry, StackObject};

mod target;
pub use target::{candidates, matches, object_kind};

mod trigger;
pub use trigger::{NotedTrigger, PendingTrigger, TriggerBindings};

mod state;
pub use state::{GameConfig, GameOutcome, GameState, PlayerConfig, StartingPlayer};

mod step;
pub use step::{Progress, StepOutcome};

mod turn;
pub use turn::{PriorityRound, TurnState};

mod zone;
pub use zone::Zones;
