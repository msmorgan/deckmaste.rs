---
needs: [english-v4-cutover-v3, semantics-v1-cutover]
---
# Cutover 2: Drop `_v4` suffixes and move MTG catalogs into `plugins/`

Execute Cutover 2 of [`docs/decisions/english-v4.md`](../../decisions/english-v4.md)
(§7) once `semantics-v1-cutover` has retired the legacy v1 consumers and renamed
`plugins_v3/` to `plugins/`. Legacy v1 `deckmaste_english`,
`deckmaste_construction_compiler` and `deckmaste_constructions_macro` were
already deleted by `retire-v1-english-and-spelling`, and `deckmaste_spelling`
is now a stub crate. Remaining:

1. **Delete `crates/deckmaste_features`** once `macro_ron` (its only remaining
   dependent) no longer needs it.
2. **Drop `_v4` Suffixes:**
   - `deckmaste_english_v4` → `deckmaste_english`
   - `deckmaste_lexical_source_v4` → `deckmaste_lexical_source`
   - (`deckmaste_syntax` already has its canonical unversioned name.)
3. **Migrate MTG Catalogs to `plugins/` Metadata:**
   - Move the MTG Catalog declarations (`catalogs/*.ron`)
     into the metadata of the corresponding `plugins/` (`v3`) DSL declarations,
     leaving core English vocabulary in `deckmaste_lexical_source`.

## Gate

- `cargo test --workspace` and `cargo clippy --workspace --tests -- -D warnings`
  pass with canonical suffixless crate names (`deckmaste_syntax`,
  `deckmaste_english`, `deckmaste_lexical_source`) and zero `v1`/`v2`/`v3`/`v4`
  English crate suffixes remaining.

Standard constraints apply.
