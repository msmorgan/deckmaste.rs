//! The rules engine: a manually-steppable [`GameState`] (one event per
//! [`GameState::step`]) and a [`Runner`] that auto-steps to decision points.
//!
//! Control flow is reified: the agenda (a work queue) holds what the engine
//! does next, so a `step()` can return mid-cascade and tests can assert
//! state between any two events.
//!
//! # Failure policy
//!
//! Three distinct kinds of "this input doesn't work" get three distinct
//! behaviors, and conflating them is the bug this policy exists to prevent.
//!
//! **Invalid semantic input fizzles.** A malformed reference or a missing
//! semantic object resolves to no applicable object, emits no game facts, and
//! retains a diagnostic for validation and logging. It does not panic. See
//! `docs/decisions/invalid-semantic-input-fizzles.md` for the boundary; that
//! decision is the only thing that licenses a fizzle.
//!
//! **An unsupported Magic mechanic panics loudly.** When the input is a legal
//! card behavior the engine has not built yet, the site keeps a
//! `todo!()`/`unimplemented!()` whose message names the mechanic, cites the
//! governing rule where one applies, and names the ticket slug that owns the
//! work — `owner: <slug>`. Never turn a legal behavior into a silent no-op:
//! a fizzle here would produce a wrong game result that looks like a correct
//! one. The message is the contract, so it must stay specific enough to
//! identify the mechanic from a panic trace alone.
//!
//! **An internal invariant stays an assertion.** Where the engine's own
//! bookkeeping is violated, the panic asserts a property of the engine, not
//! of the input, and is explicitly outside the fizzle decision. State the
//! invariant the assertion protects.
//!
//! Anonymous owners are not acceptable in any of the three. Every production
//! panic site names either a rule, an invariant, or a ticket.

mod activate;

mod agenda;
pub use agenda::WorkItem;

mod cast;
pub use cast::Payment;

mod condition;

mod control;
pub use control::ControlError;
pub use control::ControlSnapshot;

pub mod copy;

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
pub use decide::pending::ChooseManaReversals;
pub use decide::pending::ChooseModes;
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
pub use decide::pending::Retarget;
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
pub use event::Act;
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
pub use event::ManaAbilityActivated;
pub use event::ManaAdded;
pub use event::ManaEmptied;
pub use event::ManaProduced;
pub use event::Occurrence;
pub use event::PlayerLost;
pub use event::PlayerWon;
pub use event::Revealed;
pub use event::Tapped;
pub use event::TappedForMana;
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
pub use player::FloatingManaId;
pub use player::ManaActionId;
pub use player::ManaPool;
pub use player::ManaPoolError;
pub use player::ManaProvenance;
pub use player::ManaUnit;
pub use player::PlayerId;
pub use player::PlayerState;

mod player_statics;

mod payment;
pub use payment::DecisionTranscript;
pub use payment::FulfillmentWitness;
pub use payment::IouId;
pub use payment::IouKind;
pub use payment::LockedPayment;
pub use payment::LogicalObject;
pub use payment::ManaCoverage;
pub use payment::ManaPayment;
pub use payment::ManaPip;
pub use payment::ObservationBarrier;
pub use payment::PaymentCommand;
pub use payment::PaymentController;
pub use payment::PaymentFrame;
pub use payment::PaymentIou;
pub use payment::PaymentLockError;
pub use payment::PaymentProgress;
pub use payment::PaymentPrompt;
pub use payment::PaymentPurpose;
pub use payment::PaymentRecordId;
pub use payment::PaymentStage;
pub use payment::PaymentSubject;
pub use payment::QualifiedManaId;
pub use payment::RecordedRandomOutcome;
pub use payment::RecordedRngState;
pub use payment::ReplayCommand;
pub use payment::ReplayError;
pub use payment::ReplayMap;
pub use payment::ReversalBarrier;
pub use payment::TransactionRecord;
pub use payment::lock_cost;
pub use payment::reconstruct;

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

mod transform;

mod trigger;
pub use trigger::NotedTrigger;
pub use trigger::PendingTrigger;
pub use trigger::TriggerBindings;

mod state;
pub use state::EngineIncident;
pub use state::GameConfig;
pub use state::GameImage;
pub use state::GameOutcome;
pub use state::GameState;
pub use state::PaymentDeclined;
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

mod strategy_def;
pub use strategy_def::BlockPolicy;
pub use strategy_def::Extremum;
pub use strategy_def::Preference;
pub use strategy_def::Rule;
pub use strategy_def::Selector;
pub use strategy_def::Strategy as StrategyDef;

/// Self-play simulation harness for tests and benchmarks — not a stable API.
#[doc(hidden)]
pub mod sim;
