---
needs: [english-v4-cutover-v3, semantics-v1-cutover]
---
# Cutover 2: move MTG catalogs into `plugins/` and delete `deckmaste_features`

Execute what remains of Cutover 2 of [`docs/decisions/english-v4.md`](../../decisions/english-v4.md)
(§7) once `semantics-v1-cutover` has retired the legacy v1 consumers and renamed
`plugins_v3/` to `plugins/`. Legacy v1 `deckmaste_english`,
`deckmaste_construction_compiler` and `deckmaste_constructions_macro` were
already deleted by `retire-v1-english-and-spelling`, and `deckmaste_spelling`
is now a stub crate.

## Done (change `drop-v4-suffixes`, 2026-10-26)

The `_v4` suffix drop and the command rename no longer wait on
`semantics-v1-cutover`; the owner moved them forward:

- `crates/deckmaste_english_v4` is `crates/deckmaste_english` and
  `crates/deckmaste_lexical_source_v4` is `crates/deckmaste_lexical_source`
  (both names were free: v1 English and the old `deckmaste_lexical_source` are
  deleted).
- `cargo xtask english-v4` is `cargo xtask english`; its default output is
  `target/english/strata.json`. `strata.json` was byte-identical across the
  rename.
- `deckmaste_syntax` already had its canonical unversioned name.

The slug keeps its historic cutover name, which other tickets and CLAUDE.md cite.

## Remaining

1. **Delete `crates/deckmaste_features`** once `macro_ron` (its only remaining
   dependent) no longer needs it.
2. **Migrate MTG Catalogs to `plugins/` Metadata:**
   - Move the MTG Catalog declarations (`catalogs/*.ron`)
     into the metadata of the corresponding `plugins/` (`v3`) DSL declarations,
     leaving core English vocabulary in `deckmaste_lexical_source`.

## Gate

- `cargo test --workspace` and `cargo clippy --workspace --tests -- -D warnings`
  pass, with no `v1`/`v2`/`v3`/`v4` English crate suffixes remaining.

Standard constraints apply.
