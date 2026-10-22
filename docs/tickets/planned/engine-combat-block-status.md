---
needs: []
---
**Engine: "unblocked" is a status a creature gets at the declare blockers step,
and block triggers fire per [CR#509.3a..509.3d] instead of being deduplicated per
attacker before any trigger sees them.**

Provenance: `docs/evidence/mtg-rules-review-2026-10-21/departures-engine.md` #20,
#21.

## 1. `Unblocked` matches before blocks (#20)

`crates/deckmaste_engine/src/target.rs:499-502`: `// [CR#509.1h]: attacking and not
(stickily) blocked.` → `state.combat.is_attacking(id) &&
!state.combat.is_blocked(id)`; `CombatState` (`combat.rs:90-104`) has a blocked set
and no unblocked set. The test `combat_filters_read_combat_state`
(`target.rs:1554`, `:1586-1597`) asserts `Unblocked` matches right after
`declare_attacker`; rewrite it. [CR#509.1h]: an attacker with no blockers declared
"becomes an unblocked creature"; [CR#508.4d]: one put onto the battlefield attacking
during declare blockers or later enters unblocked. Ninja of the Deep Hours ruling
(2021-03-19): "Before then, attacking creatures are neither blocked nor
unblocked." Latent (only `macros/identity/Unblocked.ron` uses it), but every
ninjutsu-style consumer would be payable a step early.

**Change:** keep an explicit unblocked set in `CombatState`, filled by the declare
blockers TBA and by [CR#508.4d] entries, updated when an effect makes a creature
blocked, cleared on removal from combat; `Unblocked` reads it.
`maybe/engine-relation-spine.md` derives "unblocked" as attacking ∧ ¬blocked; that
derivation must use this status instead.

## 2. Block triggers are dropped after the first per attacker (#21)

`trigger.rs:620-627` and `:659`: `GameEvent::Blocked(Blocked { attacker, .. }) if
!blocked_attackers.insert(*attacker) => { continue; }` drops every `Blocked` fact
after the first per attacker before any watcher sees it, for every `BlockDeclared`
spelling (`eval.rs:1043-1047`): `by` ("blocks", `macros/event/Blocks.ron`), `of`
("becomes blocked"), `by`+`of` (`Blocking.ron`), and Bushido's union
(`keyword/Bushido.ron`). The comment's "[CR#509.3d] … that pattern shape doesn't
exist yet" is stale. Two bushido creatures double-blocking one attacker: the second
never triggers (recorded as a seam in `done/kw-bushido.md:21-23`). And by-only
"blocks" is never deduplicated per blocker, so a creature blocking two attackers
fires twice.

[CR#509.3a]: "Whenever [a creature] blocks" triggers "only once each combat for that
creature, even if it blocks multiple creatures". [CR#509.3b]: "blocks a creature"
triggers "once for each attacking creature the creature with the ability blocks".
[CR#509.3c]: "becomes blocked" triggers once each combat "even if it's blocked by
multiple creatures". [CR#509.3d]: "becomes blocked by a creature" triggers "once for
each creature that blocks the specified creature". [CR#700.1]: one happening may be
one event for one ability and several for another.

**Change:** stop dropping `Blocked` facts; deduplicate per ability by its key.
`of`-only: (watcher, ability, attacker) per declaration. `by`-only: (watcher,
ability, blocker). `by`+`of`: no dedup (per pair). Bushido's union: (watcher,
ability). Correct the comment. Keep the of-only behaviour canon Deepwood Tantiv
relies on. `maybe/engine-lookback-occurrence-watchers` touches the same loop.

## Tests

- `Unblocked` is false after attackers are declared and true after blockers are
  declared for an attacker with no blocker.
- Two bushido creatures block one attacker: both blockers' bushido triggers.
- One creature with "whenever this blocks" blocking two attackers triggers once;
  one with "whenever this blocks a creature" triggers twice.
