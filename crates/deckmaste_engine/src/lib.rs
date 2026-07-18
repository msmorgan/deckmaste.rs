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

mod cost_options;
pub use cost_options::ChoosableOptions;
pub use cost_options::ConcretizeError;
pub use cost_options::CostOptionChoices;
pub use cost_options::SymbolChoice;
pub use cost_options::SymbolOptions;
pub use cost_options::choosable;
pub use cost_options::concretize;

mod combat;
pub use combat::CombatState;
pub use combat::has_keyword;
pub use combat::has_keyword_named;

mod decide;
pub use decide::Action;
pub use decide::Decision;
pub use decide::DecisionError;
pub use decide::PendingDecision;
pub use decide::pending::ArrangePile;
pub use decide::pending::AssignCombatDamage;
pub use decide::pending::CallFlip;
pub use decide::pending::ChooseCostOptions;
pub use decide::pending::ChooseManaColor;
pub use decide::pending::ChooseManaMode;
pub use decide::pending::ChooseModes;
pub use decide::pending::ChooseNewTargets;
pub use decide::pending::ChooseNoteCardName;
pub use decide::pending::ChooseNoteNumber;
pub use decide::pending::ChooseObjects;
pub use decide::pending::ChooseReplacement;
pub use decide::pending::ChooseTargets;
pub use decide::pending::ChooseXValue;
pub use decide::pending::DeclareAttackers;
pub use decide::pending::DeclareBlockers;
pub use decide::pending::DiscardCards;
pub use decide::pending::DiscardToHandSize;
pub use decide::pending::Division;
pub use decide::pending::LegendRule;
pub use decide::pending::OrderReplacements;
pub use decide::pending::OrderTriggers;
pub use decide::pending::PayMana;
pub use decide::pending::PreGame;
pub use decide::pending::Priority;
pub use decide::pending::Vote;
pub use decide::pending::YesNo;

mod derive;
pub use derive::face;

mod entail;

mod eval;

mod event;
pub use event::AbilityActivated;
pub use event::AbilityCountered;
pub use event::AbilityUsed;
pub use event::Attached;
pub use event::Attacking;
pub use event::Audience;
pub use event::BecameTarget;
pub use event::Blocked;
pub use event::CoinFlipped;
pub use event::ControlChanged;
pub use event::Copied;
pub use event::CounterPlaced;
pub use event::CounterRemoved;
pub use event::DamageDealt;
pub use event::DamageRemoved;
pub use event::DesignationChanged;
pub use event::DieRolled;
pub use event::EmblemCreated;
pub use event::EnterStatus;
pub use event::GameEvent;
pub use event::GotDesignation;
pub use event::LifeGained;
pub use event::LifeLost;
pub use event::LossReason;
pub use event::ManaAdded;
pub use event::ManaEmptied;
pub use event::Occurrence;
pub use event::PlayerLost;
pub use event::PlayerWon;
pub use event::Revealed;
pub use event::Tapped;
pub use event::TokenCreated;
pub use event::TriggerFired;
pub use event::TurnBegan;
pub use event::Unattached;
pub use event::ZoneChange;

mod history;

mod layer;
pub use layer::Characteristics;
pub use layer::ContinuousEffect;
pub use layer::LayeredView;
pub use layer::ScopeResolved;

mod lki;
pub use lki::LkiSnapshot;

mod legal;
pub use legal::legal_attack_targets;
pub use legal::legal_attackers;
pub use legal::legal_blockers;

mod render;
pub use render::ActionView;
pub use render::ActionViewKind;

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

mod player_statics;

mod replace;

mod replace_registry;
pub use replace_registry::InstanceId;
pub use replace_registry::ReplacementInstance;
pub use replace_registry::ReplacementKey;

mod resolve;

mod runner;
pub use runner::RunStop;
pub use runner::Runner;

mod sba;

mod stack;
pub use stack::Anaphora;
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

#[cfg(test)]
mod test_support;

mod turn;
pub use turn::PriorityRound;
pub use turn::TurnState;

mod zone;
pub use zone::Zones;

mod strategy;
pub use strategy::StrategyEvaluator;

/// Self-play simulation harness for tests and benchmarks — not a stable API.
#[doc(hidden)]
pub mod sim;
