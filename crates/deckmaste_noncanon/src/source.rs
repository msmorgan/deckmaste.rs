//! Card lookup over the loaded plugins: `noncanon` (matchup cards, loaded
//! with the `builtin` sibling prelude) plus `builtin` itself (basic lands).

use std::path::Path;
use std::sync::Arc;

use deckmaste_cards::plugin::Plugin;
use deckmaste_core::Card;

pub struct CardSource {
    noncanon: Plugin,
    builtin: Plugin,
}

impl CardSource {
    /// Loads both plugins from the repo's `plugins/` directory.
    ///
    /// # Panics
    ///
    /// Panics if either plugin fails to load — the harness is unusable then.
    #[must_use]
    pub fn load() -> Self {
        let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../plugins");
        let noncanon = Plugin::load_with_sibling_prelude(root.join("noncanon"))
            .expect("plugins/noncanon loads");
        let builtin = Plugin::load(root.join("builtin")).expect("plugins/builtin loads");
        Self { noncanon, builtin }
    }

    /// Resolves a card by name: basics come from builtin, the rest from
    /// noncanon.
    ///
    /// # Panics
    ///
    /// Panics when the name resolves in neither plugin — for ungraduated
    /// cards this is the expected "not yet" signal.
    #[must_use]
    pub fn card(&self, name: &str) -> Arc<Card> {
        if let Ok(card) = self.builtin.card(name) {
            return Arc::new(card);
        }
        match self.noncanon.card(name) {
            Ok(card) => Arc::new(card),
            Err(e) => panic!("card {name:?} not available: {e}"),
        }
    }
}
