//! Per-variant payload structs for [`crate::decide::PendingDecision`] — one
//! newtype struct per enum variant, grouped by decision family. The enum
//! definition and its inherent methods live in the parent `decide` module;
//! this dir-module only holds the field shapes.
//!
//! Re-exported `pub` (not `pub(crate)`): `crate::decide::pending` is itself
//! only crate-visible, but `lib.rs` re-exports every struct at the crate
//! root (alongside `PendingDecision`) for external (integration-test)
//! construction/matching — that promotion requires the item itself be `pub`
//! here, not merely `pub(crate)`.

mod cast;
mod choice;
mod combat;
mod mana;
mod priority;

pub use cast::ChooseCostOptions;
pub use cast::ChooseModes;
pub use cast::ChooseNewTargets;
pub use cast::ChooseTargets;
pub use cast::ChooseXValue;
pub use cast::PayMana;
pub use choice::ArrangePile;
pub use choice::CallFlip;
pub use choice::ChooseNoteCardName;
pub use choice::ChooseNoteNumber;
pub use choice::ChooseObjects;
pub use choice::ChooseReplacement;
pub use choice::DiscardCards;
pub use choice::DiscardToHandSize;
pub use choice::Division;
pub use choice::LegendRule;
pub use choice::OrderReplacements;
pub use choice::OrderTriggers;
pub use choice::PreGame;
pub use choice::Vote;
pub use choice::YesNo;
pub use combat::AssignCombatDamage;
pub use combat::DeclareAttackers;
pub use combat::DeclareBlockers;
pub use mana::ChooseManaColor;
pub use mana::ChooseManaMode;
pub use priority::Priority;
