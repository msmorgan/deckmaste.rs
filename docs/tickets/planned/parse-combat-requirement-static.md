---
needs: []
---
The static-ability parser declines combat-requirement prose: `<Filter> attack
each combat if able` — Goblin Rabblemaster graduates as `Unparsed("Other Goblin
creatures you control attack each combat if able.")`. Extend the static parser
(in `crates/deckmaste_migrations/src/parsers/`) to emit the `Must(Attack)`
deontic static (done: engine-deontic-polarities already models `Must(Attack)`).
This is the PARSE half; engine enforcement is tracked by
engine-combat-requirements. Verify Rabblemaster's requirement clause graduates as
a `Must(Attack)` static over the right filter via a `cargo xtask generate`
delta. Part of demo-goblins-elves (bench cards).
