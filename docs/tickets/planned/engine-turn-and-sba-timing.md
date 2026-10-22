---
needs: []
---
**Engine: the cleanup step follows [CR#514.1,514.2,514.3a], combat with no attackers
skips the declare blockers and combat damage steps, and the deathtouch SBA only
counts damage dealt since the last check.** Three timing departures in
`crates/deckmaste_engine/src/step/mod.rs` and the SBA data; all reachable.

Provenance: `docs/evidence/mtg-rules-review-2026-10-21/departures-engine.md` #17,
#39, #18.

## 1. Cleanup order, priority and repeat (#17)

- `step/mod.rs:1523-1530`: Cleanup runs `self.clear_marked_damage();
  self.expire_end_of_turn(); vec![WorkItem::CheckHandSize]`, so damage and "this
  turn" effects end before the discard.
- `step/mod.rs:1546-1561` `step_tail`: Cleanup is `[CheckSbas, PlaceTriggers] +
  end_of_step_items()`, with no `OpenPriority` and no repeated cleanup; the comment
  and the doc at `:1547-1548` say the sweep "can never act in the skeleton". Stale:
  canon Falkenrath Gorger grants madness, so a cleanup discard places a madness
  trigger and the turn moves on with no priority (it resolves in the next upkeep).

[CR#514.1]: first the active player discards to maximum hand size. [CR#514.2]:
"Second, the following actions happen simultaneously: all damage marked … is
removed and all 'until end of turn' and 'this turn' effects end." [CR#514.3a]: if
any state-based action was performed or trigger is waiting, they are performed and
put on the stack, "then the active player gets priority", and another cleanup step
begins.

**Change:** run `CheckHandSize` first, then clear damage and expire effects as one
simultaneous step. After `CheckSbas` and `PlaceTriggers`, if any SBA was performed
or any trigger placed, schedule `OpenPriority` then another `BeginStep(Cleanup)`.
Delete the stale comments.

## 2. No attackers: skip declare blockers and combat damage (#39)

`begin_step` (`step/mod.rs:1362-1394`) elides only `FirstCombatDamage`
(`:1369-1376`); `DeclareBlockers` and `CombatDamage` still emit `StepBegan` and open
priority (`step_tail`, `:1565-1569`). `:1495-1515` says "with no creatures
attacking, the Declare Blockers step is skipped" and then skips only the TBA
(`PhaseStep::Combat(CombatStep::DeclareBlockers) => vec![]`), keyed on
`self.combat.attackers().is_empty()`. `legal.rs:532-533` ("the Declare Blockers
step is reached only when an attacker exists") is false. [CR#508.8]: "If no
creatures are declared as attackers or put onto the battlefield attacking, skip
the declare blockers and combat damage steps."

**Change:** set a per-combat flag when any creature is declared as an attacker or
put onto the battlefield attacking (not `combat.attackers()`, which removal from
combat empties). When it is false, go from declare attackers straight to end of
combat. Fix the `legal.rs` comment. Extend `declare_blockers_skipped_when_no_attackers`
(`tests/combat.rs:2278-2304`), which only checks for no DeclareBlockers decision, to
assert no `StepBegan` and no priority for the skipped steps.

## 3. Deathtouch counts only since the last SBA check (#18)

`check_sbas` (`step/mod.rs:1644-1650`): "deathtouch provenance now rides the damage
mark itself … no separate per-check window to close. The mark persists exactly as
long as the damage does". `DamageMark` (`object.rs:253-271`) has no check epoch;
`plugins/builtin/rules/sba/lethal-damage.ron` reads `DealtDamageBy(This,
Has(Deathtouch))` over every mark. [CR#704.5h]: a creature "dealt damage by a
source with deathtouch since the last time state-based actions were checked … is
destroyed." Reachable with canon cards: Darksteel Myr is dealt damage by Typhoid
Rats, survives the check (indestructible), then Humility resolves that turn; the
engine destroys the Myr at the next check, the CR does not. Same for a creature
that stops being a creature and becomes one again ([CR#120.6] keeps the damage
marked). `done/engine-deathtouch-sba-window.md` specified the per-check window; the
landing replaced it with the mark's lifetime.

**Change:** stamp each `DamageMark` with an SBA-check epoch (or a `checked` flag
`check_sbas` sets after each sweep); the deathtouch clause matches only marks dealt
since the last check, while [CR#704.5g] still counts all marked damage. Correct the
comment.

## Tests

- A madness card discarded in cleanup: its trigger goes on the stack, the active
  player gets priority, and a second cleanup step follows.
- A "this turn" graveyard-to-exile replacement still applies to a cleanup discard.
- A turn with no attackers has no declare blockers or combat damage `StepBegan`.
- Darksteel Myr + Typhoid Rats + Humility later that turn: the Myr survives.
