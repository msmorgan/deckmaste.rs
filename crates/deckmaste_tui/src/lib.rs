//! Interactive ratatui hotseat client — the engine's first external consumer.
mod app;
mod driver;
mod game;
mod interact;
mod shortcuts;
mod ui;

pub use app::run;
