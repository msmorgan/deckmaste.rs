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
pub use action::PlayerAction;

mod count;
pub use count::Count;
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
pub use condition::Window;

mod continuous;
pub use continuous::CostChange;
pub use continuous::Duration;
pub use continuous::Modification;
pub use continuous::Permission;
pub use continuous::Requirement;
pub use continuous::Restriction;
pub use continuous::Scope;
pub use continuous::StaticEffect;

mod cost;
pub use cost::CostComponent;

mod counter;
pub use counter::CounterDecl;

mod designation;
pub use designation::DesignationDecl;
pub use designation::DesignationDef;
pub use designation::DesignationPersistence;
pub use designation::DesignationScope;
pub use designation::DesignationShape;
pub use designation::DesignationUniqueness;

mod deontic;
pub use deontic::Deontic;
pub use deontic::DeonticAction;
pub use deontic::Window as CastWindow;

mod effect;
pub use effect::ContinuouslyEffect;
pub use effect::Effect;
pub use effect::ForEachEffect;
pub use effect::IfEffect;
pub use effect::MayEffect;
pub use effect::ModalEffect;
pub use effect::UnlessEffect;

mod event;
pub use event::BeginningStep;
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
pub use macro_ron::SupportsMacros;

mod keyword;
pub use keyword::KeywordAbility;

mod mana;
pub use mana::ManaCost;
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
pub use status::Status;

mod symbol;
pub use symbol::Symbol;

pub mod ron;

mod selection;
pub use selection::Selection;

mod target_spec;
pub use target_spec::TargetSpec;

mod token;
pub use token::Token;

mod r#type;
pub use r#type::Subtype;
pub use r#type::Supertype;
pub use r#type::Type;

mod zone;
pub use zone::Zone;

/// The unsigned integer type for game quantities that can't be negative
/// (generic mana amounts, counters, deck counts).
pub type Uint = u32;
/// The signed integer type for game state values that can go negative
/// (power/toughness, life totals).
pub type Int = i32;
