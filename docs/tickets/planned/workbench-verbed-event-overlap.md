---
needs: []
---
# Retire or record: the verbed event's overlap with three dedicated rows

Routed from `workbench-event-zone-1-verbed-event` (close, 2026-08-26).
`VerbedEvent` (the label-carrying act event) overlaps three dedicated
`GameEvent` rows — `IsDestroyed` [CR#701.8a], `StatusEvent`/tapped
[CR#701.26a], and `PutInto` — so one happening has two writable terms.
Refused by no rule; named in the row's docstring. Decide retire-vs-record
per row: retiring a dedicated row must not lose its richer slots (e.g.
`PutInto`'s source/destination) — if the verbed form cannot carry them, the
dedicated row stays and the overlap is recorded as deliberate, with the
spelling boundary told which term each printed form elaborates to.

## Consumption boundary

`idris/src/Experimental/Triggers.idr`, `Events.idr`; `Cards.idr` bench;
`Proofs*.idr`. No Rust crate.

## Acceptance

- One written verdict per overlapping row; no witness lost either way.
- `idris/scripts/build` PASS, no pin silently passing.

Standard constraints apply.
