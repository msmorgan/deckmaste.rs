---
needs: []
---
The spell-target parser graduates `deals N damage to any target` (Lightning
Bolt, Shock) but declines the restricted form `deals N damage to target player
or planeswalker` — Lava Spike graduates as `Unparsed("~ deals 3 damage to
target player or planeswalker.")`. Extend the target parser (in
`crates/deckmaste_migrations/src/parsers/`) to emit the restricted `TargetSpec`
(player-or-planeswalker, i.e. cannot hit creatures) alongside the existing
`AnyTarget`. Verify Lava Spike graduates with `DealDamage(Target(0), 3)` over a
player/planeswalker target filter via a `cargo xtask generate` delta. Small;
part of demo-goblins-elves (bench cards).
