---
needs: []
---
**Replace stringly dispatch for engine-interpreted verbs with one typed
vocabulary.** There are 51 `as_str() == "…"` sites in
`crates/deckmaste_engine/src` as of 2026-08-03, including production paths for
`Destroy`, `Draw`, `Mill`, `Discard`, and `DayNight`/`Day`/`Night`. A typo at a
call site silently disables behavior.

Inventory the sites and separate open-ended semantic-name comparisons from
the closed set of verbs the engine assigns semantics. Introduce one
`EngineVerb`-style enum/table for the closed set, with a single tested mapping
to interned `Ident` spellings, and route production dispatch through typed
variants rather than raw literals. Do not close genuinely extensible authored
names merely to make the grep reach zero.

Acceptance:

- every engine-semantic verb has one canonical spelling and round-trip test;
- production dispatch contains no raw string comparison for those verbs;
- unknown/open-ended identifiers retain their current behavior; and
- focused tests pin each migrated dispatch family, including a negative
  unknown-verb case.
