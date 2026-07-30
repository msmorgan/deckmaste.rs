---
needs: []
---
# engine-aggregate-fight-finalize — aggregate Fight finalization

Landed a bounded resolution-scoped success ledger: aggregates now consume their contained actions' own verb-appropriate finalization decisions, including zero-damage Fight, without a Fight-specific watcher branch.
