---
needs: []
---
Finish the two WC99 matchup decks (Mark Le Pine's Sped Red, Matt Linde's
Mono-Green Stompy) in `deckmaste_noncanon`. Today only Shock (red) and Llanowar
Elves (green) are engine-runnable; the other 22 unique cards load as inert
proxies. This bundles the two remaining buckets: cards ready to author now, and
the four mechanic gaps that have no ticket of their own. (Distinct from the
ticket-gated cards — Mogg Fanatic/Wasteland/Elvish Lyrist on `engine-cost-payment`,
River Boa/Albino Troll on `engine-replacements`, Rancor on `engine-attach`,
Arc Lightning/Cursed Scroll on `engine-resolve-effects`, Hammer of Bogardan on
`engine-cast-from-zones` — track those there.)

Per-wave ritual: graduate the frame in `plugins/noncanon`, add the name to the
matching allowlist in `crates/deckmaste_noncanon/src/wc99.rs`, extend the probes,
keep the 50-game subset gate green. Non-creature spells also need the relevant
pilot (SpedRed/Stompy) taught to play them.

## Ready to graduate now (engine already supports — authoring only)
1. Stone Rain — sorcery, destroy target land (`gen-destroy-effect`).
2. Giant Growth — instant, +3/+3 until EOT (`gen-pump-effect`); teach Stompy to cast non-creature spells.
3. Fireslinger — `{T}`: 1 damage to any target and 1 to you (tap cost + DealDamage live).
4. Jackal Pup — when dealt damage, deals that much to you (trigger-events live).
5. Avalanche Riders — ETB destroy target land (ETB trigger + destroy live); haste/echo riders stay proxied (see below).
6. Uktabi Orangutan — ETB destroy target artifact; echo rider stays proxied.
7. Gaea's Cradle — `{T}`: add `{G}` per creature you control (mana ability + `gen-dynamic-count`).
8. Ancient Tomb — `{T}`: add `{C}{C}`, deals 2 to you (mana ability + damage rider).
9. Pillage — sorcery, destroy target artifact or land (needs the "artifact or land" target filter; "can't be regenerated" is moot until regeneration exists).

## Mechanic gaps with no ticket yet (the hard part)
- **Echo** — upkeep echo cost ([CR#702.30]). Gates Pouncing Jaguar, Albino Troll, Uktabi Orangutan, Avalanche Riders (currently inert keyword proxies).
- **Cycling** — activated-from-hand, discard cost ([CR#702.29]). Gates Wild Dogs (which also needs its life-comparison attack restriction).
- **Haste** — summoning-sickness bypass ([CR#702.10]); lives in deontic "May" territory. Gates Avalanche Riders.
- **Manland animation** — `{cost}: becomes an N/N creature [with keywords] until end of turn` ([CR#205.1b]). Layers already support the type/PT set; this needs the activated-self-animate grammar + parser. Gates Ghitu Encampment (first strike), Treetop Village (trample). Their combat keywords already work once animated.

Each mechanic gap may warrant its own engine/grammar ticket when picked up; spin
them out if scope grows.
