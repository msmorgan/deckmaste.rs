//! Card lookup over the builtin, canon, and generated Wizards plugins.

use std::collections::HashMap;
use std::path::Path;
use std::sync::Arc;

use deckmaste_card::Card;
use deckmaste_core::ConferralRule;
use deckmaste_core::Counter;
use deckmaste_core::DamageResultRule;
use deckmaste_core::Ident;
use deckmaste_core::SbaRule;
use deckmaste_core::Subtype;
use deckmaste_core::TypeDef;
use deckmaste_plugin::plugin::Plugin;

pub struct CardSource {
    wizards: Plugin,
    canon: Plugin,
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
    pub conferral_rules: Vec<ConferralRule>,
    pub damage_result_rules: Vec<DamageResultRule>,
    pub counter_decls: HashMap<Ident, Counter>,
    pub subtypes: HashMap<Ident, Subtype>,
    pub types: HashMap<Ident, TypeDef>,
}

impl CardSource {
    /// Loads the supported card plugins from the repo's `plugins/` directory.
    ///
    /// # Panics
    ///
    /// Panics if either plugin fails to load — the harness is unusable then.
    #[must_use]
    pub fn load() -> Self {
        let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../plugins");
        let builtin = Plugin::load(root.join("builtin")).expect("plugins/builtin loads");
        let canon =
            Plugin::load_with_prelude(&builtin, root.join("canon")).expect("plugins/canon loads");
        let wizards = Plugin::load_with_prelude(&builtin, root.join("wizards"))
            .expect("plugins/wizards loads");
        Self {
            wizards,
            canon,
            builtin,
        }
    }

    /// Resolves a card by name: builtin basics first, then authored canon,
    /// then the generated Wizards corpus.
    ///
    /// # Panics
    ///
    /// Panics when the name resolves in no supported plugin.
    #[must_use]
    pub fn card(&self, name: &str) -> Arc<Card> {
        if let Ok(card) = self.builtin.card(name) {
            return Arc::new(card);
        }
        if let Ok(card) = self.canon.card(name) {
            return Arc::new(card);
        }
        match self.wizards.card(name) {
            Ok(card) => Arc::new(card),
            Err(error) => panic!("card {name:?} not available: {error:?}"),
        }
    }

    /// The engine registries to wire into a game's `GameConfig`. `sba_rules`
    /// uses builtin rules plus the generated Wizards registries.
    #[must_use]
    pub fn engine_rules(&self) -> EngineRules {
        EngineRules {
            sba_rules: self.builtin.sba_rules.clone(),
            conferral_rules: self.builtin.conferral_rules.clone(),
            damage_result_rules: self.builtin.damage_result_rules.clone(),
            counter_decls: self.wizards.counters.clone(),
            subtypes: self.wizards.subtypes.clone(),
            types: self.wizards.types.clone(),
        }
    }
}
