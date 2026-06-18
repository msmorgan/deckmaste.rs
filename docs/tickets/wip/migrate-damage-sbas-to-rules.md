---
needs: [engine-sba-breadth]
---
Migrate the lethal-damage [CR#704.5g] and deathtouch [CR#704.5h] destroy SBAs out
of the hardcoded `sweep()` block (sba.rs) into ONE merged `rules/sba/` rule with
`then: Destroy(This)`, matching the toughness/loyalty/defense rules from
`engine-sba-breadth`. New grammar: `Count::Damage(This)` (reads `obj.damage`) and
a bespoke `Condition::DamagedByDeathtouch(This)` (reads the deal-time
`struck_by_deathtouch` flag). One rule, not two — `OneOf` dedups so a creature
that is both lethally-damaged and deathtouched emits a single `WillDestroy` (two
in a batch panic).

Doing this correctly exposes — and this ticket also fixes — two general SBA-driver
corrections:

- **Loop termination**: `check_sbas` re-looped whenever the sweep *emitted* events,
  so an indestructible creature with lethal marked damage (whose destroy is canted)
  looped forever. Now the check re-runs only when the applied batch actually
  changed state ([CR#704.3] "if any SBA *performed*"); a fully-canted batch
  terminates the loop.
- **Cause attribution**: a rules-SBA `Destroy` routed through the effect path would
  be attributed to an effect / the creature itself; rules-SBA events now carry
  `agency = StateBasedAction`, no agent.

The hardcoded block is removed; `Destroy(This)` flows through the shared
`WillDestroy` replacement path, so indestructible/regeneration intercede uniformly.

Follow-ups filed by this ticket: `damage-provenance` (make damage source-aware so
deathtouch becomes `Is(Ref(Source), Has(Deathtouch))` and the bespoke flag goes
away) and `consumer-registry-wiring` (inject the rules-SBA/counter registries into
the TUI and other game runners — without it, the data-driven SBAs, now including
combat-damage death, do not fire in real play).
