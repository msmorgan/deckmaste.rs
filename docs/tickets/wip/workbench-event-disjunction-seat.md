---
needs: []
---
# Build the n-ary event disjunction seat

Both arms the seat waited on are landed (`BecomesTarget`, `Leaves` with its
source slot — event-zone sub-rounds 3, close 2026-08-26), so the recorded
shape is now implementable. The shape (settled in
`done/workbench-payment-events-and-replacement-disjunction`, restated on the
`GameEvent`/`AltEvent` docstrings — do not re-derive): a SEAT slot, n-ary,
never a `GameEvent` row; `AltEvent`'s binary becomes an arm LIST gated arm by
arm; `headerCtx`'s whole-agreement (`sameBindings`) read becomes a fold over
the list; `Delayed` and `Intercepts` gain the same slot (they have none
today). `Interceptable` distributes arm-by-arm. Carriers: Giggling
Skitterspike, Trouble in Pairs, Syr Konrad (header); Repeated Reverberation
(delayed); Illusionary Mask (replacement `would`) — 5 lines, 3 seats;
re-verify each against current vocabulary before benching.

## Consumption boundary

`idris/src/Experimental/Triggers.idr` (`AltEvent`, `headerCtx`), `Effect.idr`
(`Triggered`, `Delayed`, `Intercepts`), `Cards.idr` bench, `Proofs*.idr`.
No Rust crate.

## Acceptance

- At least one carrier per seat benches its disjunction, or the seat's
  remaining blocker is named per carrier.
- `idris/scripts/build` PASS, no witness lost, no pin silently passing.

Standard constraints apply.
