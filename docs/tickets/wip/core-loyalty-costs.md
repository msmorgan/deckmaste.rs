---
needs: []
---
Loyalty activation costs (+N / −N) for activated abilities; prerequisite for the
~1,512 planeswalker faces in the card pool.

RESOLUTION: loyalty costs need NO new cost verb. A loyalty ability's cost is to
put on or remove that many loyalty counters from its source ([CR#606.4]), so it
rides the existing `Do(PutCounters/RemoveCounters(This, LoyaltyCounter, N))`
cost — exactly as the Idris grammar already models it ("Loyalty/counter COSTS
reuse these via `Do`, so there is no duplicate counter-cost verb" — see the
`Cost` `Do` and `PutCounters`/`RemoveCounters` doc comments in
`idris/src/Core.idr`). A dedicated `CostComponent::Loyalty` variant would be
that rejected duplicate, so it was NOT added.

- `−N`: already worked before this ticket — `RemoveCounters` was already
  cost-eligible; `Do(RemoveCounters(This, LoyaltyCounter, N))` pays in the
  activation window and removes the counters.
- `+N`: this ticket made `PutCounters` cost-eligible
  (`PlayerAction::is_cost_eligible`) and added its always-payable arm in the
  activation payability check, so `Do(PutCounters(This, LoyaltyCounter, N))`
  now pays as an activation cost (adds the counters).
- `−X`: DEFERRED. A variable loyalty cost is `Do(RemoveCounters(This,
  LoyaltyCounter, Count::X))` — representable already, but paying it needs the
  activate-time announce-X decision to fire for a `Count::X` buried in a cost
  verb. The `engine-x-costs` work (now in done/) only wired X ANNOUNCEMENT for a
  `{X}` MANA symbol (`ManaSymbol::Variable`); a loyalty `−X` carries no `{X}`
  mana, so it needs a fresh follow-up extending the announce trigger to non-mana
  X costs. Not minted here — flag for triage.

Out of scope (separate concerns, not built here): loyalty-ability activation
RULES — sorcery-speed timing and once-per-turn-per-permanent ([CR#606.3]) — and
placing a planeswalker's starting loyalty on entry.
