---
needs: []
---
**Engine BUG (panic): triggers don't bind event provenance for non-`ZoneChanged`
events → Exalted & Ward crash.** Found in the 2026-06-29 code review.

`scan_event` (`crates/deckmaste_engine/src/trigger.rs:713`) sets `that_object`
only from `subject`, which is `Some` solely for `GameEvent::ZoneChanged` (`:636`);
`that_player` / `that_patient` are hardcoded `None` and no trigger ever fills
them. Resolution reads these via `.expect(...)` (`resolve.rs:1516` and
neighbours). Two shipped keywords read `ThatObject` on non-zone-change events and
panic at resolution:
- **Exalted** — `Triggered(StateBecomes(Attacking)) → Modify(of: Of(ThatObject))`
  (panics when a creature attacks alone).
- **Ward** — `Triggered(BecomesTarget) → Counter(ThatObject)` (panics on
  decline-to-pay).

Only macro *expansion* is tested, never resolution, so the suite is green. The
event-reference rework (`core-event-reference-provenance`) added the resolution
`.expect()`s and the (currently dead) `that_patient`/`that_player` fields but
wired only `defending_player`, and only for `Attacking`.

Fix: in `scan_event`, derive the event's agent/patient/actor per event kind (the
attacker for `Attacking`, the targeting source for `BecomesTarget`,
source/recipient for `DamageDealt`, …) and populate the matching binding slots —
symmetric with `defending_player`. Add engine tests resolving Exalted (attack
alone) and Ward (decline).

Severity: **critical** (reachable engine panic on evergreen keywords). Effort:
**M**.
