---
needs: []
---
**Engine: gate the event anaphora (`ThatObject`/`ThatPlayer`/`ThatMuch`) on what
the event actually supplies.** From the idris↔rust structure audit (2026-06-26) —
a cheap, high-value adoption of an Idris invariant as a Rust runtime check (not a
compile-time guarantee; Rust stays runtime-validated).

Today `Reference::ThatObject`/`ThatPlayer` (`crates/deckmaste_core/src/reference.rs`)
and `Count::ThatMuch` (`count.rs`) may appear in any effect body, and a missing
binding fails late in the engine (the `that_object`/`ThatMuch` registers in
`engine/src/state.rs`) — there is no description of which event supplies which
anaphor, and no authoring-time guard.

Idris encodes this as a total `eventKindCaps : EventKind -> EventCaps`
(`idris/src/Core.idr`) with `EventCaps { hasObject, hasActor, hasAmount }`, gating
`EventObject`/`EventActor`/`ThatMuch`; a multi-kind trigger intersects caps over
the kind-disjunction so it stays sound. A cast supplies no amount, a zone-change
no actor, etc.

Adopt the table in Rust as `fn event_caps(&Event) -> EventCaps` (a pure match) plus
a validation pass over loaded cards that rejects an anaphor whose enclosing event
doesn't supply it (and, for multi-kind event queries, intersects). Pure data + a
lint; no enum churn. Catches a class of authoring bugs the engine currently can't
detect until runtime. (Examples the gate would catch: `ThatMuch` in a cast trigger
body; `ThatObject` under a step-begin event.)
