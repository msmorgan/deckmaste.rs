---
needs: [engine-planeswalkers]
---
Headless `sim::play` constructs its `GameConfig` with `sba_rules: vec![]` and
`conferral_rules: vec![]`, so no rules-as-data fire in the simulator — builtin
state-based-action rules (e.g. 0-loyalty death) and type-conferral rules (e.g.
planeswalker enters-with-loyalty) are both inert there. The TUI game path and the
Plugin-loaded engine tests DO wire them from a loaded `Plugin`; `sim::play` should do the
same (load the builtin plugin's `sba_rules`/`conferral_rules`/`counter_decls`/`subtypes`
into its config) so simulated games match real games. Until then, any sim-driven scenario
touching planeswalkers (or any conferral/SBA rule) silently diverges from engine truth.
