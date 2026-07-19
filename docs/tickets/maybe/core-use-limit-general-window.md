---
needs: []
design: true
---
**Generalize `UseLimit::{OncePerTurn,OncePerGame}` to a times-per-window
form.** 2026-07-18 deep-dive: enforcement is already
`ability_used_count(obj, idx, window) >= 1` — i.e. the variants are sugar for
`Compare(EventCount(Used{…}, Lookback), Less, N)` with N=1, and the whole
vocabulary (`EventCount`, `EventFilter::Used`, `Lookback::ThisTurn/ThisGame`)
exists. A `{times: Count, per: window}` form subsumes both plus the real
unmodeled N=2 cards ("Activate no more than twice each turn" — Roterothopter,
Vampire Bats, both sitting in `wizards.bak` as Unparsed).

## The blocker (why design-gated)

`EventFilter::Used{of: Reference}` is object-scoped, but the restriction is
per-ability: [CR#602.5c] — an acquired restriction "applies only to that
ability as acquired from that object," not to identically worded siblings.
The engine's history fact already keys on (object, ability-index) jointly;
the *grammar* has no way to address a specific ability. Needs an
ability-addressable reference/filter (a `Used{of, which}` or
`Reference::ThisAbility`) — that addressing scheme is the design question.
Also mirror in idris: `UsageLimit` (`Core.idr`) is a flat 3-ctor type; a
Rust-only rewrite desyncs the model.

Keep the sugar spellings (`OncePerTurn` etc.) at the macro layer regardless —
the RON surface should not get noisier. `LoyaltyOncePerTurn` is NOT part of
this: it graduates downward instead (`engine-loyalty-limit-intrinsic`).
Adjacent but distinct: `engine-history-filtered-counts` (filtered Count forms
over history generally).
