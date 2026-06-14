---
needs: []
---
Generator task. Parse a chosen-sacrifice activated cost — "Sacrifice a creature",
"Sacrifice another Goblin" → a `Sacrifice(<filter>, N)` cost (subject via filter.rs)
— alongside the existing self-sacrifice (`Sacrifice ~` → `SacrificeThis`, cost.rs).
Unlocks sac-outlet goblins (Goblin Bombardment: "Sacrifice a creature: ~ deals 1
damage to any target"); self-sac (Mogg Fanatic) already works. Medium priority for
the demo. Feeds canon-goblins-elves; part of tui-client.
