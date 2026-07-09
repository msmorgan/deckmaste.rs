---
needs: []
---
Extend the activate-time **announce-X decision** to fire for a `Count::X`
appearing in a **non-mana cost verb**, not only a `{X}` mana symbol.

`engine-x-costs` (done) wired X announcement, but its trigger is gated on
`ManaSymbol::Variable` (`crates/deckmaste_engine/src/cast.rs:1032,1037`) — it only
fires when the cost contains an `{X}` mana symbol. A cost like
`Do(RemoveCounters(This, LoyaltyCounter, Count::X))` (loyalty `−X`, deferred here
from `core-loyalty-costs`) carries a `Count::X` but no `{X}` mana, so the
announce decision never fires and the X is never chosen/paid.

Scope: make the announce-X trigger also scan cost verbs for a `Count::X` operand
(`PutCounters`/`RemoveCounters` counts, and any other cost-eligible verb that can
take a `Count`), announce/choose X once for the activation, and bind it so the
verb pays the chosen amount. Reuse the existing X-announce/choose machinery from
`engine-x-costs` — this is extending its trigger surface, not a new subsystem.

Unblocks loyalty `−X` planeswalker abilities (`Do(RemoveCounters loyaltyCounter
Count::X This)`) and any non-mana `X` cost. Surfaced by `core-loyalty-costs`,
which landed `+N`/`−N` loyalty costs via `Do(PutCounters/RemoveCounters)` and
deferred `−X` to here.
