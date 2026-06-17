//! Reading card data from plugin directories.

pub mod deck;
pub mod macros;
pub mod plugin;
pub mod render;
pub mod strategy;
pub mod template;
pub mod validate;

pub use deck::Deck;
pub use deck::DeckEntry;
pub use strategy::BlockPolicy;
pub use strategy::Extremum;
pub use strategy::Preference;
pub use strategy::Rule;
pub use strategy::Selector;
pub use strategy::Strategy;
