//! The rules engine: a manually-steppable [`GameState`] (one event per
//! [`GameState::step`]) and a [`Runner`] that auto-steps to decision points.
//!
//! Control flow is reified: the agenda (a work queue) holds what the engine
//! does next, so a `step()` can return mid-cascade and tests can assert
//! state between any two events.

mod activate;

mod agenda;
pub use agenda::WorkItem;

mod cast;
pub use cast::Payment;

mod condition;

mod combat;
pub use combat::CombatState;
pub use combat::has_keyword;
pub use combat::has_keyword_named;

mod decide;
pub use decide::Action;
pub use decide::Decision;
pub use decide::DecisionError;
pub use decide::PendingDecision;

mod derive;

mod event;
pub use event::Audience;
pub use event::EnterStatus;
pub use event::GameEvent;
pub use event::LossReason;
pub use event::Occurrence;

mod history;

mod layer;
pub use layer::Characteristics;
pub use layer::ContinuousEffect;
pub use layer::LayeredView;
pub use layer::ScopeResolved;

mod lki;
pub use lki::LkiSnapshot;

mod legal;
pub use legal::legal_attackers;
pub use legal::legal_blockers;

mod render;

mod object;
pub use object::CardId;
pub use object::CardInstance;
pub use object::Cards;
pub use object::GameObject;
pub use object::ObjectId;
pub use object::ObjectSource;
pub use object::ObjectStore;
pub use object::Timestamp;

mod player;
pub use player::ManaPool;
pub use player::PlayerId;
pub use player::PlayerState;

mod tally;
pub use tally::ActivationLedger;

mod replace;

mod resolve;

mod runner;
pub use runner::RunStop;
pub use runner::Runner;

mod sba;

mod stack;
pub use stack::Frame;
pub use stack::PendingStackEntry;
pub use stack::StackEntry;
pub use stack::StackObject;

mod target;
pub use target::candidates;
pub use target::matches;
pub use target::object_kind;

mod trigger;
pub use trigger::NotedTrigger;
pub use trigger::PendingTrigger;
pub use trigger::TriggerBindings;

mod state;
pub use state::GameConfig;
pub use state::GameOutcome;
pub use state::GameState;
pub use state::PlayerConfig;
pub use state::StartingPlayer;

mod step;
pub use step::Progress;
pub use step::StepOutcome;

mod turn;
pub use turn::PriorityRound;
pub use turn::TurnState;

mod zone;
pub use zone::Zones;

/// Self-play simulation harness for tests and benchmarks — not a stable API.
#[doc(hidden)]
pub mod sim;
