---
needs: [engine-counters-api, engine-transform, pipeline-layout-extraction, engine-dfc-cast-faces]
---
Protector designation, attacking battles, the defeated trigger, and back-face
casting. (Defense counters and the battle SBA already landed — see
`defense_zero_battle_is_put_into_graveyard` in `crates/deckmaste_engine/src/sba.rs`.)

Battle-adjacent substrate already in place: counter-apply (`engine-counters-api`),
the designation registry (`engine-citys-blessing`,
`idris-open-counter-designation-registries`) that Protector rides on, trigger
events + the combat-damage event, and per-attacker attack-target threading
(`Decision::Attackers` already carries `(attacker, target)` pairs) — attacking a
battle just adds it as a legal target.

DFC deps (the piece this ticket originally missed): Siege cards are two-faced, so
back-face casting needs (1) transform-layout **extraction** to get the cards
(`pipeline-layout-extraction`), (2) the runtime front/back **face-state substrate**
(`engine-transform`), and (3) the **cast-a-face mechanism** itself
(`engine-dfc-cast-faces` — the census `modal_dfc` "back-face casting rules" line;
distinct from transform-in-place). `core-card-shapes` deliberately deferred the
layout work. Soft/related, not a hard need: `planeswalker-attack-target-ui` (the
AI/TUI picker to *choose* to attack a battle) — battles can land core-only without
it, exactly as planeswalkers did.
