//! Reading card data from plugin directories.

/// The plugin-directory path conventions, re-exported. A consumer that only
/// needs the on-disk layout — the legacy renderer's fidelity harness — reaches
/// them through the loader instead of taking a direct `deckmaste_core`
/// dependency it has no other use for.
pub use deckmaste_core::plugin as layout;

pub mod deck;
pub mod energy;
pub mod idris_emit;
pub mod loaded;
pub mod macros;
pub mod plugin;
pub mod provenance;
pub mod validate;

pub use deck::Deck;
pub use deck::DeckEntry;
pub use loaded::CardResolution;
pub use loaded::LoadedCard;
pub use loaded::LoadedToken;
