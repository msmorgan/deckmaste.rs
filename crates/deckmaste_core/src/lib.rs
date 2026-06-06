//! Core game data types for deckmaste.rs.

mod ability;
pub use ability::{Ability, Effect, KeywordAbility, Selector, SpellAbility, Target};

mod card;
pub use card::{Card, CardFace, StatValue};

mod color;
pub use color::{Color, ColorOrColorless};

pub mod ident;
pub use ident::Ident;

mod mana;
pub use mana::{ManaCost, ManaSymbol, SimpleManaSymbol};

pub mod plugin;

mod subtype;
pub use subtype::Subtype;

mod symbol;
pub use symbol::Symbol;

pub mod ron;

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
