---
needs: []
---
**Engine BUG (panic): `DivideAmong` / `ForEach` crash when an element is a
player.** Found in the 2026-06-29 code review of the idris-port batch.

`run_effect`'s `DivideAmong` arm (`crates/deckmaste_engine/src/resolve.rs:469`)
and `ForEach` (`:420`) capture every group element with `LkiSnapshot::capture`,
which panics on a player proxy (`crates/deckmaste_engine/src/lki.rs:56`,
`o.zone.expect(...)`; player proxies are minted zoneless, `state.rs:326`). Arc
Lightning's group is `Choose(AnyNumber, CreatureOrPlayer)` with body
`DealDamage(ThatObject, Allotment)`, so dividing damage onto a player — its
primary use — panics the engine. The only `DivideAmong` test uses an all-creature
group, so the suite is green.

Fix: branch on `ObjectSource` for the iteration element — bind a player as
`that_player` / `that_patient::Player(p)` (no snapshot), an object as
`that_object` / `that_patient::Object(snapshot)`, mirroring
`replace_registry.rs:695`. Add an engine test dividing damage among a creature
AND a player.

Severity: **critical** (reachable engine panic on a shipped card). Effort: **S**.
