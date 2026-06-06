//! Core game data types for deckmaste.

pub mod color;
pub mod mana;
pub mod symbol;

pub use color::Color;
pub use mana::{ManaCost, ManaSymbol, SimpleManaSymbol};

/// The unsigned integer type for game quantities that can't be negative
/// (generic mana amounts, counters, deck counts).
pub type Uint = u32;
/// The signed integer type for game state values that can go negative
/// (power/toughness, life totals).
pub type Int = i32;
