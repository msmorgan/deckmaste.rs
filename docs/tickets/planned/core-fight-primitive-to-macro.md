---
needs: []
---
**Decompose the `Action::Fight` primitive into a grammar macro over
`DealDamage`.** Fight is not a primitive — it is two creatures each dealing
damage equal to their power to the other ([CR#701.14a]) — but Rust still carries
it as a bespoke verb, a double-representation against the north-star.

## Current state

- Core: `Action::Fight(Reference, Reference)` (`crates/deckmaste_core/src/action.rs`)
  with a dedicated resolve arm (`crates/deckmaste_engine/src/resolve.rs`).
- The Idris emitter already represents fight as its decomposition (two
  `DealDamage(It, StatOf(power))`, emitter-only) — so idris-check passes, but the
  Rust primitive was deliberately left in place.

Per the minimal-primitives philosophy (keyword actions = macros over the ~6
primitives), Fight should be a macro over `DealDamage`, not a verb.

## The catch (why it is parked, not trivial)

Fight damage is dealt **simultaneously** ([CR#701.14a]) — both creatures deal
before either checks lethal — so the macro body wants
`Simultaneous[DealDamage(A, StatOf(B,power)), DealDamage(B, StatOf(A,power))]`,
NOT a `Sequence`. But `Effect::Simultaneous` is currently gated to the
exchange-family macro bodies (see the note on `Effect::Simultaneous` in
`crates/deckmaste_core/src/effect.rs`: *"Fight is NOT Simultaneous sugar — it
stays a primitive verb … until the wiring generalizes"*).

So the real work is two steps:

1. **Generalize the `Simultaneous` wiring** beyond the exchange-family
   allow-list — one batch, one timestamp, per-member replacement, SBAs after the
   whole batch ([CR#603.3b]) — so an arbitrary macro body may use it.
2. **Retire `Action::Fight`**: define a `Fight` macro
   (`Simultaneous[DealDamage×2]` with `StatOf(power)` each way, redirectable
   source per Pounce), delete the core variant + resolve arm, re-point the
   fight cards, and confirm the mutual-lethal / deathtouch / first-damage cases
   still resolve identically.

## Done

- `grep -rn 'Fight' crates/deckmaste_core/src crates/deckmaste_engine/src` shows
  no `Action::Fight` variant or resolve arm (only the macro + any event cause).
- Fight cards resolve with simultaneous mutual damage (both can die); a semantic
  engine test pins the both-lethal and deathtouch cases.
- idris-check unchanged; render fidelity for fight cards holds.
