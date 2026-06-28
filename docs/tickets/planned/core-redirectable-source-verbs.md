---
needs: []
---
**Core: redirectable `source` on object verbs — express non-self-source damage
and "fight".** From the 2026-06-28 idris↔rust model audit.

Today `Action::DealDamage(Selection, Count)` (`crates/deckmaste_core/src/action.rs`)
has no source slot — the dealer is implicitly the ability's source. So "fight"
(two creatures each deal damage equal to their power to the other) and any
"[X] deals N damage" where X is not the source object are inexpressible.

Idris `Action.DealDamage {default This source : Reference …AnObject} (Reference k)
Count` carries an optional `source` defaulting to `This`; the `fight` macro
(`idris/src/Macros.idr`) uses it directly: `DealDamage {source = x} y (StatOf x
Power)`.

Adoption: add an optional `source: Reference` (default `This`, omitted on write
when default) to the source-agent object verbs with an implicit agent — chiefly
`DealDamage`. The common-case RON is unchanged.

Verdict: **improvement** (fight, and redirected/arbitrary-source damage, are real
gaps). Effort: **M** (action.rs + RON + parser + a few cards).

Note: this is the worthwhile *slice* of the larger Idris "one `Action` with
defaulted agent fields" unification. The full merge of `Action` / `PlayerAction`
/ `By` into a single enum is **XL and not recommended** — Rust's split keeps the
player/source partition (and `PlayerAction::is_cost_eligible`) cheap without
dependent kinds. Related: NONE.
