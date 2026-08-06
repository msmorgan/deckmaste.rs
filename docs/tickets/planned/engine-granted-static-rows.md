---
needs: []
---
**Engine: most `Continuously(...)` granted static-row kinds can't be minted.**

`crates/deckmaste_engine/src/resolve/effect.rs` mints a granted continuous row
only for `Each(SelectAll, Modify(It, _))`. Three arms stay loud:

- `Each(SelectAll, inner)` where `inner` isn't `Modify(It, _)`;
- `Each(sel, _)` for any non-`SelectAll` selection;
- the catch-all — `TriggerMultiplier`, `AsThough`, `Sba`, `OutcomeGate`, and
  the rest of `StaticEffect`.

Documented residue of `engine-durations-grants` (done). The catch-all is the
one to split: each `StaticEffect` variant that a continuous *grant* can carry
needs its own mint path and its own test, and some already have owners
(`engine-asthough-had-flash`, `engine-cda-authorable-statics`). Inventory the
variants first, then split per-kind rather than implementing the catch-all as
one lump.

Granted *prevention* rows are `engine-granted-prevention-rows`.

Effort: **M** (many small sub-kinds).
