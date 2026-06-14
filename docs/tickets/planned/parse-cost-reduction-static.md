---
needs: []
---
The static-ability parser declines cost-reduction prose: `<Type> spells you cast
cost {N} less to cast` — Goblin Warchief graduates as `Unparsed("Goblin spells
you cast cost {1} less to cast.")`. Extend the static parser (in
`crates/deckmaste_migrations/src/parsers/`) to emit a `CostModifier` reducer
static (and the symmetric tax form, `cost {N} more`). This is the PARSE half;
engine application of the emitted static is tracked by engine-cost-modification.
Verify Goblin Warchief graduates with a cost-reducing static scoped to "Goblin
spells you cast" via a `cargo xtask generate` delta. Part of demo-goblins-elves
(bench cards).
