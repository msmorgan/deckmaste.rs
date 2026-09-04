//! Core game data types for deckmaste.rs.

use std::sync::Arc;

/// `skip_serializing_if` helper for slice-backed fields (`Vec<T>` and
/// `Arc<[T]>` alike, via deref coercion at the call site).
pub(crate) fn slice_is_empty<T>(s: &[T]) -> bool {
    s.is_empty()
}

/// An empty `Arc<[T]>` for serde defaults on slice-backed fields.
pub(crate) fn empty_arc<T>() -> Arc<[T]> {
    Arc::from([])
}

mod ability;
pub use ability::Ability;
pub use ability::ActivatedAbility;
pub use ability::ActivatedManaProfile;
pub use ability::ChooseSpec;
pub use ability::ManaModeClass;
pub use ability::ModalCostRider;
pub use ability::Mode;
pub use ability::SpellAbility;
pub use ability::TriggeredAbility;
pub use ability::UseLimit;

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
pub use continuous::StaticSpec;

mod cost;
pub use cost::Cost;
pub use cost::CostComponent;
pub use cost::CostTag;
pub use cost::OptionalCost;
pub use cost::RunnableCostAction;
pub use cost::RunnableCostActionError;
pub use cost::Sample;
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
pub use effect::Choose;
pub use effect::ChoosePile;
pub use effect::ChooseValue;
pub use effect::Continuously;
pub use effect::Distribute;
pub use effect::Each;
pub use effect::If;
pub use effect::Instruction;
pub use effect::Let;
pub use effect::May;
pub use effect::Modal;
pub use effect::Remember;
pub use effect::RevealUntil;
pub use effect::Search;
pub use effect::SeparatePiles;

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
pub use dpsi::Ident;
pub(crate) use dpsi::IdentSeed;
pub use filter::Adjacency;
pub use filter::CharacteristicPredicate;
pub use filter::CollectionDomain;
pub use filter::Domain;
pub use filter::EntityClass;
pub use filter::ObjectClass;
pub use filter::Predicate;
pub use filter::RelationPredicate;
pub use filter::StatePredicate;

/// Canonicalize a core value after construction or plain-serde loading.
pub trait Normalize {
    #[must_use]
    fn normalize(self) -> Self;
}

impl<T: Normalize + Clone> Normalize for Arc<T> {
    fn normalize(self) -> Self {
        Arc::new(Arc::unwrap_or_clone(self).normalize())
    }
}

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

pub mod plugin;

pub type Vec<T> = Arc<[T]>;

mod property;
pub use property::Property;

mod quantity;
pub use quantity::Quantity;

mod region;
pub use region::Block;
pub use region::CandidateBody;
pub use region::DefId;
pub use region::Expr;
pub use region::Kind;
pub use region::Param;
pub use region::Provenance;
pub use region::RefId;
pub use region::Region;
pub use region::ValidationError;
pub use region::announced_region_params;
pub use region::event_region_params;
pub use region::source_controller_params;
pub use region::triggered_region_params;
pub use region::validate;
pub use region::validate_announced;
pub use region::validate_sba;
pub use region::validate_static;
pub use region::validate_telescope;

mod reference;
pub use reference::Reference;

mod replacement;
pub use replacement::Prevention;
pub use replacement::Replacement;

mod status;
pub use status::Face;
pub use status::FaceDownCharacteristics;
pub use status::FaceDownSpec;
pub use status::Phasing;
pub use status::Status;

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
pub use sba_rule::SbaBody;
pub use sba_rule::SbaRule;

mod zone;
pub use zone::Zone;

/// The unsigned integer type for game quantities that can't be negative
/// (generic mana amounts, counters, deck counts).
pub type Uint = u32;
/// The signed integer type for game state values that can go negative
/// (power/toughness, life totals).
pub type Int = i32;
