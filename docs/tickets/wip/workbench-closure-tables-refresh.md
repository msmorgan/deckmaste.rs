# workbench-closure-tables-refresh

`docs/idris-workbench-closure-tables.md` is stale after the choice split
(B–E, done 2026-08-27): §2.1's `Words.idr:1501`/`1866` anchors are dead
(the subtype-labels conversion removed the per-word census), §2.5 predates
the split and still lists `SetsChosenBasicType` (folded by choice-B). Moved
or added rows to reflect, named in choice-D's As-landed section:
`BecomesAlso`'s widening column and distinctness gate (`TokenCanonical`),
the chosen-quality rows' dropped zone demand, `TokenRider`'s `StatusVal`
catalog, and new rows `SetsColor`, `LosesEveryType`, `ColorSpec`. A
doc-refresh pass against the current grammar; verify each row against the
code, not the old table.
