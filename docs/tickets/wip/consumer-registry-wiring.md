---
needs: [migrate-damage-sbas-to-rules]
---
Wire the rules-SBA and counter registries into game-running consumers so
state-based actions actually fire in real play. `GameState.sba_rules` and
`GameState.counter_decls` are injected by the consumer after construction and
default to empty; no non-test path injects them today — so the TUI
(`deckmaste_tui/src/game.rs` `build_game`) and any real game runner currently fire
NONE of the data-driven SBAs: lethal/deathtouch damage destroy, creature
toughness-0, planeswalker loyalty-0, battle defense-0, and the +1/+1 vs −1/−1
counter annihilation. Inject `plugin.sba_rules` and `plugin.counter_decls` (the
loaded plugin already carries both) at every real `GameState` construction site.
Prefer making this un-forgettable: thread the registries through `GameConfig` (or
a builder) so `GameState::new` wires them, rather than relying on each consumer to
inject post-construction. Without this, combat damage does not kill creatures in
the TUI.
