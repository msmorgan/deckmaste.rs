---
needs: []
---
Parse activation-restriction rider sentences on activated abilities:
`Activate only as a sorcery.`, `Activate only once each turn.`, `Activate
only during your turn[, before attackers are declared].`, `Activate only if
<condition>.`, plus `Any player may activate this ability.` The engine side
exists — `engine-activation-windows` (done) and the `UseLimit` machinery on
abilities (`crates/deckmaste_core/src/ability.rs`).

Today the restriction sentence makes the whole ability `Unparsed`: the
activated-ability parser (`parsers/activated_ability.rs`) parses
`{2}, {T}: Draw a card.` but declines the moment the rider follows. The fix
is a trailing-rider pass: strip and lower each recognized rider onto the
already-parsed ability frame (window, `UseLimit`, condition — condition
phrases via the `Condition`-macro path), leaving unknown riders as declines.

**~540 of 17,022 one-away cards** (2026-07-16 tally; `activate only as a
sorcery` alone is the top single failing sentence at 44).

Verify: `cargo xtask generate plugins/wizards` graduation delta;
`cargo xtask fidelity` on a sorcery-speed and a once-each-turn card.
