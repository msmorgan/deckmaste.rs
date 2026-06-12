//! Card lookup over the loaded plugins: `noncanon` (matchup cards, loaded
//! with the `builtin` sibling prelude) plus `builtin` itself (basic lands).

use std::collections::HashMap;
use std::path::Path;
use std::sync::Arc;

use deckmaste_cards::plugin::Plugin;
use deckmaste_core::Card;
use deckmaste_core::Counter;
use deckmaste_core::Ident;
use deckmaste_core::SbaRule;
use deckmaste_core::Subtype;

pub struct CardSource {
    noncanon: Plugin,
    builtin: Plugin,
}

/// The engine registries a real game needs wired into its [`GameConfig`], the
/// consumer-side wiring no shared helper does yet (the TUI's pending
/// `consumer-registry-wiring`). `sba_rules` carries the core [CR#704.3] rules
/// (toughness-0, lethal damage, …) without which creatures never die;
/// `counter_decls`/`subtypes` are the counter and subtype registries — dormant
/// for the current WC99 waves but wired so a future counter/subtype card
/// behaves the day it graduates.
///
/// [`GameConfig`]: deckmaste_engine::GameConfig
#[derive(Clone)]
pub struct EngineRules {
    pub sba_rules: Vec<SbaRule>,
    pub counter_decls: HashMap<Ident, Counter>,
    pub subtypes: HashMap<Ident, Subtype>,
}

impl CardSource {
    /// Loads both plugins from the repo's `plugins/` directory.
    ///
    /// Loads `builtin` once and passes it as the explicit prelude to
    /// `noncanon`, so the builtin macro/subtype layer is shared rather than
    /// loaded twice.
    ///
    /// # Panics
    ///
    /// Panics if either plugin fails to load — the harness is unusable then.
    #[must_use]
    pub fn load() -> Self {
        let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../plugins");
        let builtin = Plugin::load(root.join("builtin")).expect("plugins/builtin loads");
        let noncanon = Plugin::load_with_prelude(&builtin, root.join("noncanon"))
            .expect("plugins/noncanon loads");
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

    /// The engine registries to wire into a game's `GameConfig`. `sba_rules`
    /// merges both plugins' `rules/sba/` (the core rules live in `builtin`;
    /// `noncanon` adds none today but may); `counter_decls`/`subtypes` come
    /// from `noncanon`, already a superset of `builtin`'s via the load prelude.
    #[must_use]
    pub fn engine_rules(&self) -> EngineRules {
        EngineRules {
            sba_rules: self
                .builtin
                .sba_rules
                .iter()
                .chain(&self.noncanon.sba_rules)
                .cloned()
                .collect(),
            counter_decls: self.noncanon.counters.clone(),
            subtypes: self.noncanon.subtypes.clone(),
        }
    }
}
