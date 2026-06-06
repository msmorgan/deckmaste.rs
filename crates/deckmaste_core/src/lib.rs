//! Core game data types for deckmaste.rs.

mod ability;
pub use ability::{
    Ability, ActivatedAbility, ChooseSpec, KeywordAbility, Mode, SpellAbility, StaticAbility,
    TriggerLimit, TriggeredAbility,
};

mod action;
pub use action::Action;

mod card;
pub use card::{Card, CardFace, StatValue};

mod color;
pub use color::{Color, ColorOrColorless};

mod condition;
pub use condition::{Cmp, Condition, Window};

mod continuous;
pub use continuous::{
    CostChange, Duration, Modification, Permission, Requirement, Restriction, Scope, StaticEffect,
};

mod cost;
pub use cost::CostComponent;

mod counter;
pub use counter::CounterDecl;

mod de_util;

mod designation;
pub use designation::{
    DesignationDecl, DesignationDef, DesignationPersistence, DesignationScope, DesignationShape,
    DesignationUniqueness,
};

mod effect;
pub use effect::{
    ContinuouslyEffect, Effect, ForEachEffect, IfEffect, MayEffect, ModalEffect, UnlessEffect,
};

mod event;
pub use event::{Event, StateFilterEvent, StepOrPhase, WhoseTurn};

mod filter;
pub use filter::{CharacteristicFilter, Filter, ObjectKind, RelationFilter, StateFilter};

pub mod ident;
pub use ident::Ident;

mod mana;
pub use mana::{ManaCost, ManaSpec, ManaSymbol, SimpleManaSymbol};

pub mod plugin;

mod quantity;
pub use quantity::{Quantity, Stat};

mod reference;
pub use reference::Reference;

mod replacement;
pub use replacement::{Prevention, Replacement};

mod status;
pub use status::Status;

mod subtype;
pub use subtype::Subtype;

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
pub use r#type::{Supertype, Type};

mod zone;
pub use zone::Zone;

/// The unsigned integer type for game quantities that can't be negative
/// (generic mana amounts, counters, deck counts).
pub type Uint = u32;
/// The signed integer type for game state values that can go negative
/// (power/toughness, life totals).
pub type Int = i32;
