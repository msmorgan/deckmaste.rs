---
needs: []
---
**`eval_reference` returns a bare `ObjectId` with no last-known-information
channel, so two consumers each re-derive which anaphora slot a `Reference` reads
when the object is gone — parallel copies of `eval_reference`'s own dispatch that
have already drifted three ways.** The engine has no English-impedance excuse
(the data is fully structured); this is pure special-casing — a concept (the LKI
snapshot) lacking its own channel, patched by shadow routing tables.

The two shadow tables (`crates/deckmaste_engine/src/`):

- `resolve/count.rs:615-637` (`lki_counters`, read by `Count::CounterCount` at
  `count.rs:155-163`)
- `condition.rs:202-227` (`bound_snapshot`, read by `Condition::Matches` at
  `condition.rs:66-83`)

Each re-implements `This → frame.this`, `EventObject → that_object`, `It →
anaphora.it`, etc. Drift already shipped:

- `lki_counters` has no `Expanded` peel (`count.rs:634` `_ => None`), so
  `CounterCount(Expanded(This))` on a dead object reads 0 while `Matches(Expanded(
  This), …)` reads the snapshot correctly.
- Neither table carries `eval_reference`'s `EventObject → that_patient` fallback
  (`query.rs:499-506`), so a patient-only frame's LKI reads find nothing.
- Neither covers `That(Sort)` / `Target(n)` / `Coalesce`.

Coupled: `Reference::EventObject` itself inspects the **sibling** `that_patient`
binding (`query.rs:493-507`) to special-case the zoneless player-proxy recipient —
its meaning depends on which sibling slots the frame-builder populated, and the
fallback exists only here, not in the two tables, so the reads are inconsistent.

## Fix

Reference resolution returns one product carrying `(live id, optional LKI
snapshot)`. `CounterCount`/`Matches` consume the snapshot from that single channel
and both shadow tables delete. Make `that_object` kind-polymorphic (like
`EventPatient`) or always-populated by the trigger/replacement emitter, so
`EventObject` reads exactly one channel with no sibling fallback. This is the
highest-leverage single fix in the resolve-layer cross-boundary audit — it also
resolves the `EventObject` inconsistency and hardens the paid-object read folded
into [[engine-bound-references]]. Relates to [[engine-lki-robustness]] (the
snapshot struct's field-enumeration), [[atom-independence-anaphora-only]].

Verify: engine tests; a gone-object `CounterCount(Expanded(This))` reads the
snapshot, not 0; a patient-only frame resolves `EventObject` LKI; both former
shadow tables are deleted.
