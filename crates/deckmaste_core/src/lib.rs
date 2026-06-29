//! Core game data types for deckmaste.rs.

mod ability;
pub use ability::Ability;
pub use ability::ActivatedAbility;
pub use ability::ChooseSpec;
pub use ability::Mode;
pub use ability::SpellAbility;
pub use ability::StaticAbility;
pub use ability::TriggeredAbility;
pub use ability::UseLimit;

mod action;
pub use action::Action;
pub use action::Bin;
pub use action::PlayerAction;

mod count;
pub use count::Characteristic;
pub use count::Count;
pub use count::RoundMode;
pub use count::Stat;

mod card;
pub use card::Card;
pub use card::CardFace;
pub use card::StatValue;

mod color;
pub use color::Color;
pub use color::ColorOrColorless;

mod condition;
pub use condition::Cmp;
pub use condition::Condition;

mod continuous;
pub use continuous::CostChange;
pub use continuous::Duration;
pub use continuous::Modification;
pub use continuous::OutcomeGateKind;
pub use continuous::Scope;
pub use continuous::StaticEffect;

mod cost;
pub use cost::Cost;
pub use cost::CostComponent;
pub use cost::TotalCost;

mod counter;
pub use counter::Counter;
pub use counter::CounterRef;

mod decision;
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
pub use deontic::CountBound;
pub use deontic::Deontic;
pub use deontic::DeonticAction;

mod temporal;
pub use temporal::LockPoint;
pub use temporal::TurnMarker;
pub use temporal::Window;

mod effect;
pub use effect::Continuously;
pub use effect::Effect;
pub use effect::ForEach;
pub use effect::If;
pub use effect::May;
pub use effect::MayPay;
pub use effect::Modal;
pub use effect::MustPay;
pub use effect::Noting;
pub use effect::Targeted;
pub use effect::Unless;
pub use effect::With;

mod event;
pub use event::Agency;
pub use event::BeginningStep;
pub use event::Cause;
pub use event::CausePattern;
pub use event::CombatStep;
pub use event::EndingStep;
pub use event::Event;
pub use event::Phase;
pub use event::StateFilterEvent;
pub use event::WhoseTurn;

mod filter;
pub use filter::CharacteristicFilter;
pub use filter::Filter;
pub use filter::ObjectKind;
pub use filter::RelationFilter;
pub use filter::StateFilter;
pub use macro_ron::Expand;
pub use macro_ron::Expansion;
pub use macro_ron::ExpansionArgs;
pub use macro_ron::Ident;
pub use macro_ron::IdentSeed;
pub use macro_ron::Normalize;
pub use macro_ron::SupportsMacros;

mod keyword;
pub use keyword::KeywordAbility;
pub use keyword::KeywordRef;

mod mana;
pub use mana::ManaCost;
pub use mana::ManaProduction;
pub use mana::ManaRider;
pub use mana::ManaSpec;
pub use mana::ManaSymbol;
pub use mana::SimpleManaSymbol;

pub mod plugin;

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
pub use status::Face;
pub use status::FaceDownCharacteristics;
pub use status::FaceDownSpec;
pub use status::Phasing;
pub use status::Status;

mod symbol;
pub use symbol::Symbol;

pub mod ron;

mod selection;
pub use selection::Extremum;
pub use selection::Selection;

// Play-policy types (`strategy::Strategy`, `Preference`, …). Deliberately NOT
// re-exported at the crate root: kept namespaced so the rules-primitive
// namespace stays separate from play policy.
pub mod strategy;

mod target_spec;
pub use target_spec::TargetSpec;

mod token;
pub use token::PredefinedToken;
pub use token::Token;
pub use token::TokenName;
pub use token::TokenSpec;

mod r#type;
pub use r#type::Subtype;
pub use r#type::Supertype;
pub use r#type::Type;

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
