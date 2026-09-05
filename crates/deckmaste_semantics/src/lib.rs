//! The semantics rules grammar: the single source of truth every card-content
//! container is written in — card files, token files, and the `rules/` engine
//! tables.
//!
//! Forked from `deckmaste_core` as an exact mirror; see the fork commit for
//! provenance. Must never depend on `deckmaste_core`
//! (`docs/decisions/semantics-spelling-lowering.md` §1).
//!
//! `plugin` (path conventions) and `strategy` (play policy, outside the
//! semantics program per §3) deliberately stay core-side.

use std::sync::Arc;

/// `skip_serializing_if` helper for slice-backed fields (`Vec<T>` and
/// `Arc<[T]>` alike, via deref coercion at the call site).
pub(crate) fn slice_is_empty<T>(s: &[T]) -> bool {
    s.is_empty()
}

/// An empty `Arc<[T]>`. A path-call default for `#[macro_ron(default = ...)]`
/// (a method-call expression like `[].into()` needs syn's `full` feature,
/// which the derive does not enable).
pub(crate) fn empty_arc<T>() -> Arc<[T]> {
    Arc::from([])
}

pub mod authoring;
pub mod identity_registry;

mod ability;
pub use ability::Ability;
pub use ability::ActivatedAbility;
pub use ability::ChooseSpec;
pub use ability::ModalCostRider;
pub use ability::Mode;
pub use ability::SpellAbility;
pub use ability::TriggeredAbility;
pub use ability::UseLimit;
pub use subterms::AbilitySubterms;

mod action;
pub use action::Action;
pub use action::Anchor;
pub use action::Arrangement;
pub use action::CopyRetarget;
pub use action::Destination;
pub use action::EnterRider;
pub use action::LifeOp;
pub use action::RetargetMode;
pub use action::discard_body_count;
pub use action::discard_body_random;
pub use action::discard_body_what;
pub use action::discard_body_whose;
pub use action::fight_body_fighters;

mod binder;
pub use binder::Binder;

// The semantic container types; `deckmaste_card`'s are the engine's (§1).
mod card;
pub use card::Card;
pub use card::CardFace;
pub use card::FaceLayout;

mod count;
pub use count::AggregateOp;
pub use count::BASIC_LAND_TYPES;
pub use count::Characteristic;
pub use count::Count;
pub use count::Countable;
pub use count::Projection;
pub use count::RoundMode;
pub use count::Stat;

mod stat_value;
pub use stat_value::StatValue;

mod color;
pub use color::Color;
pub use color::ColorOrColorless;

mod condition;
pub use condition::Cmp;
pub use condition::Condition;

mod conferral_rule;
pub use conferral_rule::ConferralRule;

mod damage_result_rule;
pub use damage_result_rule::DamageResultRule;

mod continuous;
pub use continuous::CollectionOp;
pub use continuous::CostChange;
pub use continuous::Duration;
pub use continuous::IgnoreRule;
pub use continuous::Modification;
pub use continuous::NumericOp;
pub use continuous::OutcomeGateKind;
pub use continuous::PayAct;
pub use continuous::PipClass;
pub use continuous::PlayerAttr;
pub use continuous::PlayerMod;
pub use continuous::StaticEffect;

mod cost;
pub use cost::Cost;
pub use cost::CostComponent;
pub use cost::CostTag;
pub use cost::OptionalCost;
pub use cost::TotalCost;

mod counter;
pub use counter::Counter;
pub use counter::CounterRef;
pub use counter::CounterScope;
pub use counter::CounterSpec;

mod copy;
pub use copy::CopiableValues;
pub use copy::CopyException;
pub use copy::CopySource;
pub use copy::CopySpec;

mod decision;
pub use decision::ChosenValueKind;
pub use decision::DeciderSpec;
pub use decision::NotedKind;
pub use decision::Visibility;

mod designation;
pub use designation::CoreDeed;
pub use designation::DesignationConferrer;
pub use designation::DesignationDecl;
pub use designation::DesignationDef;
pub use designation::DesignationPersistence;
pub use designation::DesignationScope;
pub use designation::DesignationShape;
pub use designation::DesignationUniqueness;

mod deontic;
pub use deontic::AlternativeCost;
pub use deontic::AsThough;
pub use deontic::CostPredicate;
pub use deontic::CountBound;
pub use deontic::DeedAgent;
pub use deontic::Deontic;
pub use deontic::DeonticAction;

mod temporal;
pub use temporal::LockPoint;
pub use temporal::Lookback;
pub use temporal::Timing;
pub use temporal::TurnMarker;

mod effect;
pub use effect::AdditionalCost;
pub use effect::ChoosePile;
pub use effect::Continuously;
pub use effect::Distribute;
pub use effect::Each;
pub use effect::If;
pub use effect::Label;
pub use effect::May;
pub use effect::Modal;
pub use effect::Noting;
pub use effect::OneShotEffect;
pub use effect::PileSource;
pub use effect::RevealUntil;
pub use effect::SeparatePiles;
pub use effect::Targeted;
pub use effect::With;

mod event;
pub use event::Agency;
pub use event::BeginningStep;
pub use event::Cause;
pub use event::CausePattern;
pub use event::CombatStep;
pub use event::EndingStep;
pub use event::EventFilter;
pub use event::PhaseKind;
pub use event::PhaseStep;
pub use event::StateChange;
pub use event::VerbName;
pub use event::WhoseTurn;

mod filter;
pub use filter::Adjacency;
pub use filter::CharacteristicPredicate;
pub use filter::ObjectKind;
pub use filter::Predicate;
pub use filter::RelationPredicate;
pub use filter::StatePredicate;
pub use macro_ron::Expand;
pub use macro_ron::Expansion;
pub use macro_ron::ExpansionArgs;
pub use macro_ron::Ident;
pub use macro_ron::IdentSeed;
pub use macro_ron::MacroFields;
pub use macro_ron::Normalize;
pub use macro_ron::SupportsMacros;

mod keyword;
pub use keyword::KeywordAbility;
pub use keyword::KeywordDecl;
pub use keyword::KeywordRef;
pub use keyword::ParamShape;

mod mana;
pub use mana::ManaCost;
pub use mana::ManaProduction;
pub use mana::ManaRider;
pub use mana::ManaSpec;
pub use mana::ManaSymbol;
pub use mana::PlanarFace;
pub use mana::SimpleManaSymbol;
pub use mana::SymbolPred;

pub mod macros;

pub type Vec<T> = Arc<[T]>;

mod property;
pub use property::Property;

mod quantity;
pub use quantity::Quantity;

mod reference;
pub use reference::Reference;

mod replacement;
pub use replacement::Prevention;
pub use replacement::Replacement;

mod status;
mod subterms;
pub use status::Face;
pub use status::FaceDownCharacteristics;
pub use status::FaceDownSpec;
pub use status::Phasing;
pub use status::Status;

mod sort;
pub use sort::Sort;

pub mod ron;

mod selection;
pub use selection::Selection;

mod target_spec;
pub use target_spec::TargetSpec;

mod token;
pub use token::PredefinedToken;
pub use token::Token;
pub use token::TokenName;
pub use token::TokenSpec;

mod r#type;
pub use r#type::Subtype;
pub use r#type::SubtypeRef;
pub use r#type::Supertype;
pub use r#type::Type;
pub use r#type::TypeDef;
pub use r#type::TypeRef;

mod sba_rule;
pub use sba_rule::SbaRule;

mod zone;
pub use zone::Zone;

/// The unsigned integer type for game quantities that can't be negative
/// (generic mana amounts, counters, deck counts).
pub type Uint = u32;
/// The signed integer type for game state values that can go negative
/// (power/toughness, life totals).
pub type Int = i32;
