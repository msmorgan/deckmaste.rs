---
needs: [engine-trigger-events]
---
Combat-damage EVENT coordinate. `deckmaste_core::Event` has no combat flag: the
engine's damage GameEvent carries no "is combat damage" bit, so a
`Performed(verb: "DealDamage")` trigger matches ALL damage, and "Whenever ~ deals
COMBAT damage to a player/creature" [CR#120.2a] cannot be spelled faithfully
(emitting it would be semantically wrong — it would also fire on noncombat
damage).

Add a combat coordinate to the damage event (a `combat: bool` on the damage
GameEvent + a `BecomesCombatDamage`/filtered `Performed` event variant) so the
"deals combat damage" trigger family parses + fires correctly.

Highest-count trigger cluster still blocked: ~50+ one-away cards
(`Whenever ~ deals combat damage to a player, put a +1/+1 counter on it`, draw /
drain / scry-on-combat-damage, etc.), plus many more multi-blocked. The effect
bodies already parse — only the trigger event is missing. Flagged by the
parse-trigger-shells worker (deferred rather than emit wrong RON).
