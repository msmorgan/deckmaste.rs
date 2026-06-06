//! Core game data types for deckmaste.rs.

mod ability;
pub use ability::{Ability, ActivatedAbility, KeywordAbility, SpellAbility};

mod action;
pub use action::Action;

mod card;
pub use card::{Card, CardFace, StatValue};

mod color;
pub use color::{Color, ColorOrColorless};

mod cost;
pub use cost::CostComponent;

mod effect;
pub use effect::Effect;

mod filter;
pub use filter::{CharacteristicFilter, Filter, ObjectKind, RelationFilter, StateFilter};

pub mod ident;
pub use ident::Ident;

mod mana;
pub use mana::{ManaCost, ManaSpec, ManaSymbol, SimpleManaSymbol};

pub mod plugin;

mod reference;
pub use reference::Reference;

mod subtype;
pub use subtype::Subtype;

mod symbol;
pub use symbol::Symbol;

pub mod ron;

mod selection;
pub use selection::Selection;

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
